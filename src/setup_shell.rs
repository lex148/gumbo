use dirs::home_dir;
use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

/// Ensure that a given line is present in the file at `path`. If not, append it.
fn ensure_line(path: &PathBuf, line: &str) -> io::Result<()> {
    // Read existing content if file exists
    let existing = fs::read_to_string(path).unwrap_or_default();
    if existing.lines().any(|l| l.trim() == line.trim()) {
        return Ok(());
    }
    // Append the line
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    writeln!(file, "{}", line)?;
    Ok(())
}

/// Setup completion for Bash
fn setup_bash() -> io::Result<()> {
    let mut rc = home_dir().expect("Could not find home directory");
    rc.push(".bashrc");
    let line = "source <(COMPLETE=bash gumbo)";
    ensure_line(&rc, line)
}

/// Setup completion for Zsh
fn setup_zsh() -> io::Result<()> {
    let mut rc = home_dir().expect("Could not find home directory");
    rc.push(".zshrc");
    let line = "source <(COMPLETE=zsh gumbo)";
    ensure_line(&rc, line)
}

/// Setup completion for Fish
fn setup_fish() -> io::Result<()> {
    let mut rc = home_dir().expect("Could not find home directory");
    rc.push(".config/fish/completions/gumbo.fish");
    let line = "source (COMPLETE=fish gumbo | psub)";
    ensure_line(&rc, line)
}

/// Setup completion for Elvish
fn setup_elvish() -> io::Result<()> {
    let mut rc = home_dir().expect("Could not find home directory");
    rc.push(".elvish/rc.elv");
    let line = "eval (E:COMPLETE=elvish gumbo | slurp)";
    ensure_line(&rc, line)
}

/// Setup completion for PowerShell
#[cfg(target_os = "windows")]
fn setup_powershell() -> io::Result<()> {
    let profile = env::var("USERPROFILE")?;
    let mut rc = PathBuf::from(profile);
    rc.push(r"Documents/PowerShell/Microsoft.PowerShell_profile.ps1");
    let lines = vec![
        "$env:COMPLETE = \"powershell\"",
        "gumbo | Out-String | Invoke-Expression",
        "Remove-Item Env:\\COMPLETE",
    ];
    for line in lines {
        ensure_line(&rc, line)?;
    }
    Ok(())
}

/// Main entry: generate completion scripts and set up shell integration
pub(crate) fn setup_completions() -> io::Result<()> {
    // Detect shell and configure
    if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        setup_powershell()?;
    } else if let Ok(shell_path) = env::var("SHELL") {
        if let Some(shell_name) = shell_path.rsplit('/').next() {
            match shell_name {
                "bash" => setup_bash()?,
                "zsh" => setup_zsh()?,
                "fish" => setup_fish()?,
                "elvish" => setup_elvish()?,
                _ => {} // unsupported shell
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ensure_line_idempotent() {
        let tmp = PathBuf::from("test_rc");
        // clean
        let _ = fs::remove_file(&tmp);
        // first add
        ensure_line(&tmp, "hello").unwrap();
        assert!(fs::read_to_string(&tmp).unwrap().contains("hello"));
        // second add should not duplicate
        ensure_line(&tmp, "hello").unwrap();
        let content = fs::read_to_string(&tmp).unwrap();
        assert_eq!(content.lines().filter(|l| l == &"hello").count(), 1);
        let _ = fs::remove_file(&tmp);
    }
}
