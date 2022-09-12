//! Wrappers for the `git config` command.

use std::{path::Path, process::Command};

use crate::error::{Error, Result};

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
