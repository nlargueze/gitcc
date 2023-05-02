//! `version` command

use std::env;

use clap::Parser;
use gitcc_core::{commit_history, Config};

use crate::{info, warn};

/// Bump command arguments
#[derive(Debug, Parser)]
pub struct VersionArgs {}

/// Gets the current version and determines the next version
pub fn run(_args: VersionArgs) -> anyhow::Result<()> {
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
    let dirty_files = gitcc_core::dirty_files(&cwd)?;
    if !dirty_files.is_empty() {
        warn!("repo is dirty");
        // for f in dirty_files {
        //     eprintln!("  {f}");
        // }
    }

    let history = commit_history(&cwd, &config)?;
    println!(
        "current_version:{}",
        history
            .curr_version
            .map(|v| v.to_string())
            .unwrap_or_else(|| "none".to_string())
    );
    println!("next_version:{}", history.next_version);

    Ok(())
}
