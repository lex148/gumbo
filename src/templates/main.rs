use super::TemplateError;
use super::{ensure_directory_exists, try_format};
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("src/main.rs");
    ensure_directory_exists(&path)?;
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;

    let buf: String = write()?.to_string();
    file.write_all(buf.as_bytes())?;
    let _ = try_format(&path);
    Ok(())
}

/// Adds a route to the list of actix services
pub(crate) fn append_service(
    root_path: &Path,
    service: impl Into<String>,
) -> Result<(), TemplateError> {
    let service: String = service.into();
    let mut path = root_path.to_path_buf();
    path.push("src/main.rs");

    let mut file = File::options().write(true).read(true).open(&path)?;
    file.rewind()?;
    let mut content = String::default();
    file.read_to_string(&mut content)?;
    file.rewind()?;
    let last_service = match find_last_service_call(&content) {
        Some(x) => x.to_owned(),
        None => return Ok(()),
    };
    if last_service.contains(&service) {
        return Ok(());
    }
    let new_service = format!(".service({})", service);
    let code = format!("{}\n            {}", last_service, new_service);
    let modified_content = content.replace(&last_service, &code);
    file.write_all(modified_content.as_bytes())?;
    Ok(())
}

fn find_last_service_call(input: &str) -> Option<&str> {
    // Start by searching for the last occurrence of ".service("
    let mut pos = input.rfind(".service(");
    while let Some(start) = pos {
        // Find the closing parenthesis starting from the position just after ".service("
        let sub_str = &input[start..];
        if let Some(end) = sub_str.find(')') {
            // Return the substring from ".service(" to the matching ")"
            return Some(&sub_str[..end + 1]);
        }
        // Update the position to search before the current found position
        if start == 0 {
            break;
        }
        pos = input[..start - 1].rfind(".service(");
    }
    None
}

fn write() -> Result<&'static str, TemplateError> {
    Ok(r#"
use actix_web::{App, HttpServer};
use std::env;
use std::net::SocketAddr;
use crate::controllers::*;

mod controllers;
mod errors;
mod helpers;
mod migrations;
mod models;
mod views;

use welds::connections::sqlite::SqliteClient;
pub(crate) type DbClient = actix_web::web::Data<SqliteClient>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    // read the environment variables
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_owned());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let bind_interface: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    // Connect to the database and run the migrations
    let connection_string =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_owned());
    let client = welds::connections::sqlite::connect(&connection_string)
        .await
        .expect("Unable to connect to Database");
    migrations::up(&client).await.unwrap();
    let client = actix_web::web::Data::new(client);

    // boot up the server
    log::info!("Server Running: http://{}", bind_interface);
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(assets_controller::styles)
            .service(greetings_controller::index)
    })
    .bind(bind_interface)?
    .run()
    .await
}
"#)
}
