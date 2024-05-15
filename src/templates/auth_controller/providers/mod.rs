use crate::change::Change;
use crate::errors::Result;

pub(crate) mod fakeoauth;
pub(crate) mod google;

pub(crate) fn write_template() -> Result<Change> {
    Change::new("./src/controllers/auth_controller/providers/mod.rs", CODE)
}

static CODE: &str = r##"
use crate::errors::{oauth_error, Result};
use oauth2::basic::BasicClient;

mod fakeoauth;
mod google;

pub(crate) fn build(provider: &str, siteroot: &str) -> Result<BasicClient> {
    let config = match provider {
        "google" => google::build(siteroot)?,
        "fakeoauth" => fakeoauth::build(siteroot)?,
        _ => Err(oauth_error(provider, "PROVIDER NOT CONFIGURED"))?,
    };
    Ok(config)
}

pub(crate) async fn get_unique_id(provider: &str, access_token: &str) -> Result<String> {
    let config = match provider {
        "google" => google::get_unique_id(access_token).await?,
        "fakeoauth" => fakeoauth::get_unique_id(access_token).await?,
        _ => Err(oauth_error(provider, "PROVIDER NOT CONFIGURED"))?,
    };
    Ok(config)
}

pub(crate) fn get_scopes(provider: &str) -> Result<&'static [&'static str]> {
    let scopes = match provider {
        "google" => &["email"],
        "fakeoauth" => &["fake"],
        _ => Err(oauth_error(provider, "PROVIDER NOT CONFIGURED"))?,
    };
    Ok(scopes)
}

"##;
