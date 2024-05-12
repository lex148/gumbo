use crate::templates::ensure_directory_exists;
use crate::templates::TemplateError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./src/controllers/auth_controller/providers/fakeoauth.rs");
    ensure_directory_exists(&path)?;
    let mut file = File::create(&path)?;
    file.write_all(CODE.trim().as_bytes())?;
    Ok(())
}

static CODE: &str = r##"
use crate::errors::{oauth_error, Result};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use serde::Deserialize;

/// create a unique identifier for this user
pub(crate) async fn get_unique_id(access_token: &str) -> Result<String> {
    // request user info from fakeoauth using the access_token
    let client = reqwest::Client::new();
    let url = "http://127.0.0.1:5860/userinfo";
    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|_| oauth_error("fakeoauth", "failed to get userinfo"))?;
    // parse the json result into UserInfo
    if !response.status().is_success() {
        println!("Failed to fetch data. Status: {:?}", response.status());
        Err(oauth_error("fakeoauth", "userinfo status != 200"))?;
    }
    // return the user_id
    let user: UserInfo = response
        .json()
        .await
        .map_err(|e| oauth_error("fakeoauth", format!("userinfo parse error: {:?}", e)))?;
    let sub = user.sub;
    Ok(format!("fakeoauth:{sub}"))
}

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    sub: String,
}

/// returns a client that is setup to login a user using fakeoauth's oauth2 service
pub(crate) fn build(siteroot: &str) -> Result<BasicClient> {
    // build all the URLs needed fro the client
    let redirect_url = format!("{siteroot}/auth/return/fakeoauth");
    let auth_url = AuthUrl::new("http://127.0.0.1:5860".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("http://127.0.0.1:5860/token".to_string())
        .expect("Invalid token endpoint URL");

    // build the client
    let client = BasicClient::new(
        ClientId::new("".to_string()),
        Some(ClientSecret::new("".to_string())),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

    Ok(client)
}
"##;
