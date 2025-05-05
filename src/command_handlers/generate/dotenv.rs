use super::get_root_path;
use crate::change::{Change, write_to_disk};
use crate::errors::Result;
use base64::{Engine, engine::general_purpose::STANDARD};

pub(crate) fn generate() -> Result<()> {
    let rootpath = get_root_path().unwrap();

    let changes: Vec<Change> = write_template().expect("unable to write .env file");

    for change in &changes {
        write_to_disk(&rootpath, change)?;
    }

    println!("A .env was create.");

    Ok(())
}

fn rand_auth_secret() -> String {
    use rand::rand_core::{OsRng, TryRngCore};
    let mut rng = OsRng;
    let mut bytes = [0u8; 32];
    rng.try_fill_bytes(&mut bytes)
        .expect("OS: rand not available");
    STANDARD.encode(bytes)
}

pub(crate) fn write_template() -> Result<Vec<Change>> {
    let auth_secret = rand_auth_secret();
    let mut lines = vec![
        "# If you want to login with google. Add your oauth2 id/secret here.".to_owned(),
        "OAUTH_GOOGLE_CLIENT_ID=\"\"".to_owned(),
        "OAUTH_GOOGLE_CLIENT_SECRET=\"\"".to_owned(),
        "RUST_LOG=info".to_owned(),
        "#openssl rand -base64 32".to_owned(),
        format!("AUTH_SECRET={auth_secret}"),
    ];
    if let Ok(db_env) = std::env::var("DATABASE_URL") {
        lines.push(format!("DATABASE_URL={db_env}"));
    }
    let text = lines.join("\n");
    Ok(vec![Change::new("./.env", text)?])
}

pub(crate) fn write_template_lite() -> Result<Vec<Change>> {
    let mut lines = vec!["RUST_LOG=info".to_owned()];
    if let Ok(db_env) = std::env::var("DATABASE_URL") {
        lines.push(format!("DATABASE_URL={db_env}"));
    }
    let text = lines.join("\n");
    Ok(vec![Change::new("./.env", text)?])
}
