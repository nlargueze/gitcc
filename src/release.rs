//! Release

use std::process::Command;

use anyhow::bail;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::config::Config;

/// Release configuration
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReleaseConfig {
    /// Bump commands
    pub bump_commands: Vec<String>,
}

/// Executes the bump commands
pub fn execute_bump_commands(
    config: &Config,
    version: &Version,
) -> anyhow::Result<Vec<(String, String)>> {
    let mut stdouts = vec![];
    for cmd in &config.release.bump_commands {
        let cmd = cmd.replace("{version}", &version.to_string());
        let cmd_args: Vec<&str> = cmd.split(' ').collect();
        let output = Command::new(cmd_args[0])
            .args(&cmd_args[1..])
            .current_dir(&config.repo_dir)
            .output()?;

        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;

        if !output.status.success() {
            bail!(format!("{stdout}{stderr}"));
        }
        stdouts.push((cmd, stdout));
    }

    Ok(stdouts)
}
