use super::get_root_path;
use crate::command_handlers::generate::GenerateError;
use crate::templates::TemplateError;
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::{rngs::OsRng, RngCore};
use std::path::Path;
use std::{fs::File, io::Write};

pub(crate) fn generate() -> Result<(), GenerateError> {
    let root_path = get_root_path()?;

    println!("A .env was create.");

    write_template(&root_path).expect("unable to write .env file");

    Ok(())
}

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut envpath = root_path.to_path_buf();
    envpath.push(".env");

    // Don't override the users .env file if it exists
    if envpath.exists() {
        eprintln!("A .env already exists file. unable to generate the file:");
        eprintln!("{}", envpath.to_str().unwrap_or_default());
        std::process::exit(1);
    }

    let content = build_envfile();
    let mut file = File::create(envpath)?;
    file.write_all(&content.as_bytes())?;

    Ok(())
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
