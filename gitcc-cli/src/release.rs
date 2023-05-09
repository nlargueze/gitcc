//! `release` command

use std::env;

use clap::Parser;
use gitcc_core::{Config, StatusShow};

use crate::{info, warn};

/// Commit command arguments
#[derive(Debug, Parser)]
pub struct ReleaseArgs {}

/// Executes the command `release`
pub fn run(_args: ReleaseArgs) -> anyhow::Result<()> {
    // load the config
    let cwd = env::current_dir()?;
    let config = Config::load_from_fs(&cwd)?;
    let _config = if let Some(cfg) = config {
        cfg
    } else {
        info!("using default config");
        Config::default()
    };

    // Checks that the repo is clean
    let dirty_files = gitcc_core::git_status(&cwd, StatusShow::IndexAndWorkdir)?;
    if !dirty_files.is_empty() {
        warn!("repo is dirty");
    }

    eprintln!("1: make sure there is no untracked/uncommitted changes");
    eprintln!("2: create/update the changelog file");
    eprintln!("3: bump the package versions");
    eprintln!("4: commit the updated packages and changelog");
    eprintln!("5: tag that commit with the next version (annotated tag, leading 'v')");
    eprintln!("6: push with --follow-tags");
    eprintln!("7: create a Github release");
    eprintln!("8: publish the updated packages (crates.io, npm, brew, etc...");

    Ok(())
}
