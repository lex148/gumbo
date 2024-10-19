use crate::change::Change;
use crate::errors::Result;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;

pub(crate) fn write_template() -> Result<Vec<Change>> {
    Ok(vec![Change::new("./src/main.rs", CODE)?])
}

/// Adds a route to the list of actix services
pub(crate) fn append_service(path: &Path, service: impl Into<String>) -> crate::errors::Result<()> {
    let service: String = service.into();

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

const CODE: &str = r#"
use actix_web::{web::Data, App, HttpServer};
use std::env;
use std::net::SocketAddr;
use crate::controllers::*;

mod controllers;
mod errors;
mod migrations;
mod models;
mod views;

use welds::connections::sqlite::SqliteClient;
pub(crate) type DbClient = actix_web::web::Data<SqliteClient>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // default log level to info
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    if let Err(err) = dotenvy::dotenv() {
        match err {
            dotenvy::Error::Io(_) => {}
            _ => log::warn!("DOTENV: {:?}", err),
        }
    }

    // read the environment variables to find what Interface to bind to
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_owned());
    let port = port.parse::<u16>().unwrap();
    let host = env::var("HOST").unwrap_or_else(|_| "::1".to_owned());
    let ip: std::net::IpAddr = host.parse().unwrap();
    let bind_interface: SocketAddr = SocketAddr::new(ip, port);

    // verify auth keys are setup
    gumbo_lib::session::verify_auth_key();

    // Connect to the database and run the migrations
    let connection_string =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://./dev.sqlite".to_owned());
    let client = welds::connections::sqlite::connect(&connection_string)
        .await
        .expect("Unable to connect to Database");
    migrations::up(&client).await.unwrap();

    let client = Data::new(client);

    // boot up the server
    log::info!("Server Running: http://{}", bind_interface);
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(health_controller::index)
            .service(assets_controller::styles)
            .service(assets_controller::javascript)
            // serve the contents of the assets directory to the public
            .service(actix_files::Files::new("/assets", "./src/assets"))
            .service(greetings_controller::index)
            .service(greetings_controller::index_restricted)
            .service(auth_controller::auth_login)
            .service(auth_controller::auth_return)
            .service(auth_controller::logout)
    })
    .bind(bind_interface)?
    .run()
    .await
}
"#;
