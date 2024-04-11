use super::TemplateError;
use super::{ensure_directory_exists, try_format};
use std::fs::File;
use std::io::Write;
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

fn write() -> Result<&'static str, TemplateError> {
    Ok(r#"
use actix_web::{App, HttpServer};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use crate::controllers::*;

mod controllers;
mod errors;
mod helpers;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_owned());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let bind_interface: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
    let addr = ("127.0.0.1", 3000);


    let connection_string = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_owned());
    let client = Arc::new(
        welds::connections::sqlite::connect(&connection_string)
            .await
            .expect("Unable to connect to Database"),
    );

    log::info!("Server Running: {}", bind_interface);
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(assets_controller::styles)
            .service(greetings_controller::index)
    })
    .bind(addr)?
    .run()
    .await
}
"#)
}
