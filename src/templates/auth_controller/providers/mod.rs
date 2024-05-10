use crate::templates::ensure_directory_exists;
use crate::templates::TemplateError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) mod google;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./src/controllers/auth_controller/providers/mod.rs");
    ensure_directory_exists(&path)?;
    let mut file = File::create(&path)?;
    file.write_all(CODE.trim().as_bytes())?;
    Ok(())
}

static CODE: &str = r##"
use crate::errors::{oauth_error, Result};
use oauth2::basic::BasicClient;

mod google;

pub(crate) fn build(provider: &str, siteroot: &str) -> Result<BasicClient> {
    let config = match provider {
        "google" => google::build(siteroot)?,
        _ => Err(oauth_error(provider, "PROVIDER NOT CONFIGURED"))?,
    };
    Ok(config)
}

pub(crate) async fn get_unique_id(provider: &str, access_token: &str) -> Result<String> {
    let config = match provider {
        "google" => google::get_unique_id(access_token).await?,
        _ => Err(oauth_error(provider, "PROVIDER NOT CONFIGURED"))?,
    };
    Ok(config)
}

pub(crate) fn get_scopes(provider: &str) -> Result<&'static [&'static str]> {
    let scopes = match provider {
        "google" => &["email"],
        _ => Err(oauth_error(provider, "PROVIDER NOT CONFIGURED"))?,
    };
    Ok(scopes)
}

"##;
