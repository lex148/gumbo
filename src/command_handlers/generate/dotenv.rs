use super::get_root_path;
use crate::change::{write_to_disk, Change};
use crate::errors::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::{rngs::OsRng, RngCore};

pub(crate) fn generate() -> Result<()> {
    let rootpath = get_root_path().unwrap();
    println!("A .env was create.");
    let changes: Vec<Change> = write_template().expect("unable to write .env file");

    for change in &changes {
        write_to_disk(&rootpath, change)?;
    }

    Ok(())
}

pub(crate) fn write_template() -> Result<Vec<Change>> {
    Ok(vec![Change::new("./.env", build_envfile())?])
}

fn rand_auth_secret() -> String {
    let mut rng = OsRng::default();
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

#openssl rand -base64 32
AUTH_SECRET={auth_secret}
   "##,
    )
}
