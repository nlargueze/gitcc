//! Wrappers for the `git config` command.

use std::{path::Path, process::Command};

use crate::error::{Error, Result};

/// Returns the git origin URL.
pub fn get_config_origin_url() -> Result<String> {
    let output = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        return Err(Error::InternalError(
            "Failed to execute git config".to_string(),
        ));
    }

    let stdout = match String::from_utf8(output.stdout) {
        Ok(s) => s.trim().to_string(),
        Err(err) => {
            return Err(Error::InternalError(format!("Invalid stdout ({err})")));
        }
    };

    let stdout = stdout.strip_suffix(".git").unwrap_or(&stdout);

    Ok(stdout.to_string())
}

/// Sets the git hooks directory.
///
/// `git config core.hookspath ${dir}`
pub fn set_config_install_hooks(dir: &Path) -> Result<()> {
    let dir_str_lossy = dir.to_string_lossy();
    let dir_str = dir_str_lossy.as_ref();
    let output = Command::new("git")
        .args(["config", "core.hookspath", dir_str])
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        return Err(Error::InternalError(
            "Failed to execute git config core.hooksPath".to_string(),
        ));
    }

    Ok(())
}
