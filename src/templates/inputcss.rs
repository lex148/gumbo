use super::{ensure_directory_exists, TemplateError};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./src/input.css");
    ensure_directory_exists(&path)?;
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;

    file.write_all(CODE.as_bytes())?;
    tailwindcfg(root_path)?;

    Ok(())
}

static CODE: &str = r#"@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  a {
    @apply text-pink-600;
  }
  a:hover {
    @apply text-pink-800;
  }
}
"#;

pub(crate) fn tailwindcfg(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./tailwind.config.js");
    ensure_directory_exists(&path)?;
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;
    file.write_all(TWCFG.as_bytes())?;
    Ok(())
}

static TWCFG: &str = r#"/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "index.html",
    "./src/views/**/*.rs"
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
"#;
