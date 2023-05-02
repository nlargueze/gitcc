//! `version` command

use std::env;

use clap::Parser;
use gitcc_core::{commit_history, Config, StatusShow};

use crate::{info, new_line, warn};

/// Bump command arguments
#[derive(Debug, Parser)]
pub struct VersionArgs {}

/// Gets the current version and determines the next version
pub fn run(_args: VersionArgs) -> anyhow::Result<()> {
    new_line!();

    // load the config
    let cwd = env::current_dir().unwrap();
    let config = Config::load_from_fs(&cwd)?;
    let config = if let Some(cfg) = config {
        cfg
    } else {
        info!("using default config");
        Config::default()
    };

    // Checks that the repo is clean
    let status = gitcc_core::git_status(&cwd, StatusShow::IndexAndWorkdir)?;
    if !status.is_empty() {
        warn!("repo is dirty");
    }

    let history = commit_history(&cwd, &config)?;
    println!(
        "{} --> {}",
        history
            .curr_version
            .map(|v| v.to_string())
            .unwrap_or_else(|| "none".to_string()),
        history.next_version
    );

    Ok(())
}
