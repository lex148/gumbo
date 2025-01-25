use crate::change::Change;
use crate::errors::Result;

pub(crate) fn write_template() -> Result<Vec<Change>> {
    Ok(vec![Change::new("./src/input.css", CODE)?])
}

static CODE: &str = r#"@import "tailwindcss";

@layer base {
  a {
		color: var(--color-pink-600);
  }
  a:hover {
		color: var(--color-pink-800);
  }
}
"#;
