//! Release

use std::{path::Path, process::Command};

use gitcc_git::discover_repo;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{Config, Error};

/// Release configuration
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReleaseConfig {
    /// Bump commands
    ///
    /// The version is passed as a tag `{{version}}`
    pub bump_cmds: Vec<String>,
}

/// Bumps the package(s) version to the next version
pub fn exec_bump_commands(cfg: &Config, version: &str) -> Result<Vec<String>, Error> {
    let mut ran_commands = vec![];
    for cmd in cfg.release.bump_cmds.iter() {
        let cmd = cmd.replace("{{version}}", version);
        ran_commands.push(cmd.clone());
        let cmd_split = cmd.split(' ').collect::<Vec<_>>();
        let program = cmd_split[0];
        let args = cmd_split[1..].iter().copied().collect_vec();

        let cmd_res = Command::new(program)
            .args(&args)
            .output()
            .map_err(|err| Error::msg(format!("failed to execute '{cmd}': {err}").as_str()))?;

        if !cmd_res.status.success() {
            let stderr = String::from_utf8_lossy(&cmd_res.stderr);
            return Err(Error::msg(
                format!("failed to execute '{cmd}': {stderr}").as_str(),
            ));
        }
    }
    Ok(ran_commands)
}

/// Add all changes to the index
pub fn add_all_changes(cwd: &Path) -> Result<(), Error> {
    let repo = discover_repo(cwd)?;
    Ok(gitcc_git::add_all(&repo)?)
}

/// Sets an annotated tag to the HEAD
pub fn set_annotated_tag(cwd: &Path, tag: &str, message: &str) -> Result<(), Error> {
    let repo = discover_repo(cwd)?;
    Ok(gitcc_git::set_annotated_tag(&repo, tag, message)?)
}

/// Push with tags
pub fn push_with_tags(cwd: &Path) -> Result<(), Error> {
    let repo = discover_repo(cwd)?;
    Ok(gitcc_git::push_to_remote(&repo, "origin")?)
}
