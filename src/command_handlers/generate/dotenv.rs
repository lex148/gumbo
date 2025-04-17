use super::get_root_path;
use crate::change::{Change, write_to_disk};
use crate::errors::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use rand::{RngCore, rngs::OsRng};

pub(crate) fn generate() -> Result<()> {
    let rootpath = get_root_path().unwrap();

    let changes: Vec<Change> = write_template().expect("unable to write .env file");

    for change in &changes {
        write_to_disk(&rootpath, change)?;
    }

    println!("A .env was create.");

    Ok(())
}

pub(crate) fn write_template() -> Result<Vec<Change>> {
    Ok(vec![Change::new("./.env", build_envfile())?])
}

fn rand_auth_secret() -> String {
    let mut rng = OsRng;
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    STANDARD.encode(bytes)
}

fn build_envfile() -> String {
    let auth_secret = rand_auth_secret();
    format!(
        r##"
# If you want to login with google. Add your oauth2 id/secret here.
OAUTH_GOOGLE_CLIENT_ID=""
OAUTH_GOOGLE_CLIENT_SECRET=""

RUST_LOG=info

#openssl rand -base64 32
AUTH_SECRET={auth_secret}
   "##,
    )
}
