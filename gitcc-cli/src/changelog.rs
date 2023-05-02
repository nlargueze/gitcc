//! `changelog` command

use std::env;

use clap::Parser;
use gitcc_core::{
    build_changelog, commit_history, ChangelogBuildOptions, Config, StatusShow,
    TEMPLATE_CHANGELOG_STD,
};

use crate::{info, warn};

/// Changelog command arguments
#[derive(Debug, Parser)]
pub struct ChangelogArgs {
    /// Includes all commits
    #[arg(long)]
    pub all: bool,
}

/// Generates the change log
pub fn run(args: ChangelogArgs) -> anyhow::Result<()> {
    let cwd = env::current_dir()?;
    let cfg = Config::load_from_fs(&cwd)?;
    let cfg = if let Some(c) = cfg {
        c
    } else {
        info!("using default config");
        Config::default()
    };

    // Checks that the repo is clean
    let status = gitcc_core::git_status(&cwd, StatusShow::IndexAndWorkdir)?;
    if !status.is_empty() {
        warn!("repo is dirty");
    }

    // Generate the changelog
    let history = commit_history(&cwd, &cfg)?;
    let changelog_opts = ChangelogBuildOptions {
        origin_name: None,
        all: args.all,
    };
    let changelog = build_changelog(&cwd, &cfg, &history, Some(changelog_opts))?;
    let changelog_str = changelog.render(TEMPLATE_CHANGELOG_STD)?;
    println!("{changelog_str}");

    Ok(())
}
