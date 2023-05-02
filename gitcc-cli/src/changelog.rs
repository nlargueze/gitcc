//! `changelog` command

use std::env;

use clap::Parser;
use gitcc_core::{build_changelog, commit_history, Config, TEMPLATE_CHANGELOG_STD};

use crate::{info, warn};

/// Changelog command arguments
#[derive(Debug, Parser)]
pub struct ChangelogArgs {}

/// Generates the change log
pub fn run(_args: ChangelogArgs) -> anyhow::Result<()> {
    let cwd = env::current_dir()?;
    let cfg = Config::load_from_fs(&cwd)?;
    let cfg = if let Some(c) = cfg {
        c
    } else {
        info!("using default config");
        Config::default()
    };

    // Checks that the repo is clean
    let dirty_files = gitcc_core::dirty_files(&cwd)?;
    if !dirty_files.is_empty() {
        warn!("repo is dirty");
    }

    // Generate the changelog
    let history = commit_history(&cwd, &cfg)?;
    let changelog = build_changelog(&cwd, &cfg, &history, None)?;
    let changelog_str = changelog.render(TEMPLATE_CHANGELOG_STD)?;
    println!("{changelog_str}");

    Ok(())
}
