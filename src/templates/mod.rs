use std::path::Path;

pub(crate) mod asset_controller;
pub(crate) mod auth_controller;
pub(crate) mod build;
pub(crate) mod controller;
pub(crate) mod docker;
pub(crate) mod errors;
pub(crate) mod greetings_controller;
pub(crate) mod helpers_mod;
pub(crate) mod inputcss;
pub(crate) mod main;
pub(crate) mod migrations;
pub(crate) mod models;
pub(crate) mod models_session;
pub(crate) mod modrs;
pub(crate) mod view;
pub(crate) mod views_mod;

pub(crate) fn ensure_directory_exists(path: &Path) -> Result<(), std::io::Error> {
    if let Some(dir_path) = path.parent() {
        std::fs::create_dir_all(dir_path)?;
    }
    Ok(())
}
