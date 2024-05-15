use crate::change::Change;
use crate::errors::Result;

pub(crate) fn write_template() -> Result<Vec<Change>> {
    Ok(vec![
        Change::new("./src/input.css", CODE)?,
        Change::new("./tailwind.config.js", TWCFG)?,
    ])
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
