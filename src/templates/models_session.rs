use crate::templates::ensure_directory_exists;
use crate::templates::modrs::append_module;
use crate::templates::TemplateError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./src/models/session.rs");
    ensure_directory_exists(&path)?;
    append_module(root_path, "./src/models/mod.rs", "session")?;
    let mut file = File::create(&path)?;
    file.write_all(CODE.trim().as_bytes())?;
    Ok(())
}

static CODE: &str = r##"
use actix_web::FromRequest;
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::prelude::*;
use rand::RngCore;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// An Active Users Session
/// Try to keep this model as lite weight as you can.
/// If you want to store info about a user You should go make a user table/model
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct Session {
    // A unique identifier for the given user
    sub: String,
    // unix timestamp (sec) when this session will expire
    exp: i64,
    // The expected csrf_token for this given session
    csrf_token: String,
}

impl Session {
    pub(crate) fn sub(&self) -> &str {
        &self.sub
    }

    /// This is called when a user is logged in
    pub(crate) fn build(sub: impl Into<String>) -> Session {
        let csrf_token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        Session {
            sub: sub.into(),
            csrf_token,
            exp: next_exp_time(),
        }
    }

    pub(crate) fn as_encrypted(&self) -> String {
        let key_bytes = auth_key();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        // Generate a random nonce
        let mut noncebytes = [0u8; 12];
        OsRng.fill_bytes(&mut noncebytes);
        let nonce = Nonce::from_slice(&noncebytes);
        // generate an encrypt string of this struct
        let serialized = bincode::serialize(self).expect("Serialization failed");
        let cipherbytes: Vec<u8> = cipher
            .encrypt(nonce, serialized.as_ref())
            .expect("Serialization failed");
        let allbytes: Vec<u8> = noncebytes
            .as_slice()
            .iter()
            .chain(cipherbytes.iter())
            .cloned()
            .collect();
        let allbase64 = BASE64_STANDARD.encode(&allbytes);
        allbase64
    }

    fn from_encrypted(encrypted_bytes: &[u8]) -> Result<Session, actix_web::Error> {
        use actix_web::error::ErrorForbidden;
        if encrypted_bytes.len() <= 12 {
            return Err(ErrorForbidden(""));
        }
        let (noncebytes, contents) = encrypted_bytes.split_at(12);
        let nonce = Nonce::from_slice(&noncebytes);
        let key_bytes = auth_key();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        let bytes = cipher
            .decrypt(nonce, contents)
            .or(Err(ErrorForbidden("")))?;
        let session: Self = bincode::deserialize(&bytes).or(Err(ErrorForbidden("")))?;
        Ok(session)
    }
}

/// Panics if the AUTH_SECRET is not set or is invalid.
/// used at boot to make sure the app is setup
pub(crate) fn verify_auth_key() {
    let _ = auth_key();
}

/// Panics if the AUTH_SECRET is not set or is invalid.
/// used at boot to make sure the app is setup
fn auth_key() -> Vec<u8> {
    use base64::prelude::*;
    let key_base64 =
        std::env::var("AUTH_SECRET").expect("\n\nAUTH_SECRET env not set. expected a AES_256_KEY\nYou can generate an AUTH_SECRET for your gumbo project to use by running the command:\ngumbo generate env\n\n");
    let key_bytes = BASE64_STANDARD
        .decode(key_base64)
        .expect("\nFailed to read env AUTH_SECRET. expected a AES_256_KEY\nYou can generate an AUTH_SECRET for your gumbo project to use by running the command:\ngumbo generate env\n\n");
    assert_eq!(key_bytes.len(), 32, "Key must be 256 bits (32 bytes)");
    key_bytes
}

/// returns the time now
fn now_sec() -> i64 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

/// return the time one hour from now
fn next_exp_time() -> i64 {
    let now = SystemTime::now();
    now.checked_add(Duration::new(60 * 60 * 24, 0)) //24 hours from now
        .expect("time overflowed")
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

use actix_web::dev::Payload;
use actix_web::HttpRequest;
use futures::future::LocalBoxFuture;

/// Allows you to request a Session from an actix resource
impl FromRequest for Session {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, std::result::Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req_clone = req.clone();
        Box::pin(async move { load_session(&req_clone).await })
    }
}

/// loads a session from the database using the Authentication cookie
async fn load_session(req: &HttpRequest) -> std::result::Result<Session, actix_web::Error> {
    use actix_web::error::ErrorForbidden;
    let auth_cookie = req.cookie("_session").ok_or(ErrorForbidden(""))?;
    let encrypted_base64 = auth_cookie.value().to_string();
    let encrypted_bytes = BASE64_STANDARD
        .decode(&encrypted_base64)
        .or(Err(ErrorForbidden("")))?;
    let session = Session::from_encrypted(&encrypted_bytes).or(Err(ErrorForbidden("")))?;
    if session.exp < now_sec() {
        return Err(ErrorForbidden(""));
    }
    Ok(session)
}
"##;
