use super::ensure_directory_exists;
use super::modrs::append_module;
use super::TemplateError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod providers;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./src/controllers/auth_controller/mod.rs");
    ensure_directory_exists(&path)?;
    append_module(root_path, "./src/controllers/mod.rs", "auth_controller")?;
    let mut file = File::create(&path)?;
    file.write_all(CONTROLLER.trim().as_bytes())?;
    //
    providers::write_template(root_path)?;
    providers::google::write_template(root_path)?;
    providers::fakeoauth::write_template(root_path)?;
    Ok(())
}

static CONTROLLER: &str = r##"
use crate::errors::{oauth_error, Result};
use crate::models::session::Session;
use actix_web::cookie::time::OffsetDateTime;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{delete, get, web::Path, web::Query, HttpRequest, HttpResponse};
use oauth2::{reqwest::async_http_client, AuthorizationCode};
use oauth2::{CsrfToken, Scope, TokenResponse};
use serde::Deserialize;

pub(crate) mod providers;

#[get("/auth/login/{provider}")]
async fn auth_login(req: HttpRequest, provider: Path<String>) -> Result<HttpResponse> {
    // get the oauth provider
    let root = siteroot(&req);
    let client = providers::build(provider.as_str(), &root)?;

    // add the scope and prepare to make the login request.
    let mut client = client.authorize_url(CsrfToken::new_random);
    for scope in providers::get_scopes(&provider)? {
        client = client.add_scope(Scope::new(scope.to_string()));
    }

    // redirect the user to the oauth providers login screen
    let (url, _) = client.url();
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", url.to_string()))
        .finish())
}

#[derive(Debug, Deserialize)]
struct ReturnParams {
    code: String,
}

#[get("/auth/return/{provider}")]
async fn auth_return(
    req: HttpRequest,
    provider: Path<String>,
    query: Query<ReturnParams>,
) -> Result<HttpResponse> {
    // get the oauth provider
    let root = siteroot(&req);
    let client = providers::build(provider.as_str(), &root)?;

    // exchange the one time code for a security token
    let code = query.code.to_owned();
    let token = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await
        .map_err(|err| {
            log::error!("{:?}", err);
            oauth_error(provider.as_ref(), "TOKEN ERROR")
        })?;

    // get a unique way to identity this specific user
    let sub = providers::get_unique_id(provider.as_str(), token.access_token().secret()).await?;

    // Build a session for this user.
    let session = Session::build(sub);
    let encrypted_session = session.as_encrypted();

    // create an Authorization cookie for this validated user.
    let cookie = Cookie::build("_session", encrypted_session)
        .path("/")
        .secure(true) // Marks the cookie to be used with HTTPS only
        .http_only(true) // Not accessible via JavaScript
        .same_site(SameSite::Strict) // Strict CSRF protection
        .finish();

    // Firefox will not set the cookie from a SeeOther
    // use meta redirect instead
    let return_url = root;
    let html =
        format!(r#"<head><meta http-equiv="Refresh" content="0; URL={return_url}" /></head>"#);

    // The user is logged in! Send them were you want them to go.
    Ok(HttpResponse::Ok().cookie(cookie).body(html))
}

#[delete("/auth/logout")]
async fn logout(req: HttpRequest) -> Result<HttpResponse> {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    // Calculate the past date for expiration
    let expire_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs() as i64
        - 86400; // 86400 seconds make 24 hours

    // Expire the _session cookie
    let cookie = Cookie::build("_session", "".to_owned())
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::ZERO)
        .expires(OffsetDateTime::from_unix_timestamp(expire_time).unwrap())
        .finish();

    // Where you want to go when you logout
    let url = siteroot(&req);
    Ok(HttpResponse::SeeOther()
        .cookie(cookie)
        .append_header(("Location", url.to_string()))
        .finish())
}

/// returns the root of the site based on the incoming request
fn siteroot(req: &HttpRequest) -> String {
    // Get the original requested host from the Host header
    let original_host = req
        .headers()
        .get("Host")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    // Reconstruct the original requested URL using the scheme and original host
    let root = format!("{}://{}", req.connection_info().scheme(), original_host);
    root
}

"##;
