use crate::change::Change;
use crate::errors::Result;

pub(crate) fn write_template() -> Result<Change> {
    Change::new(
        "./src/controllers/auth_controller/providers/google.rs",
        CODE,
    )
}

static CODE: &str = r##"
use crate::errors::{oauth_error, Result};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use serde::Deserialize;
use std::env::var;

/// create a unique identifier for this user
pub(crate) async fn get_unique_id(access_token: &str) -> Result<String> {
    // request user info from google using the access_token
    let client = reqwest::Client::new();
    let url = "https://www.googleapis.com/oauth2/v3/userinfo";
    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|_| oauth_error("google", "failed to get userinfo"))?;
    // parse the json result into UserInfo
    if !response.status().is_success() {
        println!("Failed to fetch data. Status: {:?}", response.status());
        Err(oauth_error("google", "userinfo status != 200"))?;
    }
    // return the user_id
    let user: UserInfo = response
        .json()
        .await
        .map_err(|e| oauth_error("google", format!("userinfo parse error: {:?}", e)))?;
    let sub = user.sub;
    Ok(format!("google:{sub}"))
}


/// All the info returned by userinfo. Us the other stuff as needed.
#[derive(Deserialize, Debug)]
pub struct UserInfo {
    sub: String,
    //picture: Option<String>,
    //email: Option<String>,
    //email_verified: Option<bool>,
    //hd: Option<String>,
}

/// returns a client that is setup to login a user using google's oauth2 service
pub(crate) fn build(siteroot: &str) -> Result<BasicClient> {
    // Load the configs from ENV
    let client_id = readenv("OAUTH_GOOGLE_CLIENT_ID")?;
    let client_secret = readenv("OAUTH_GOOGLE_CLIENT_SECRET")?;

    // build all the URLs needed fro the client
    let redirect_url = format!("{siteroot}/auth/return/google");
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    // build the client
    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

    Ok(client)
}

fn readenv(v: &str) -> Result<String> {
    let value = var(v).map_err(|_err| oauth_error("google", format!("missing ENV: {v}")))?;
    Ok(value)
}

"##;
