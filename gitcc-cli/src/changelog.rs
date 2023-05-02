//! `changelog` command

use std::env;

use clap::Parser;
use colored::Colorize;
use gitcc_core::{build_changelog, commit_history, Config, TEMPLATE_CHANGELOG_STD};

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
        eprintln!("{} using default config", "i".blue().bold());
        Config::default()
    };

    // Checks that the repo is clean
    let dirty_files = gitcc_core::dirty_files(&cwd)?;
    if !dirty_files.is_empty() {
        eprintln!("{} repo is dirty", "!".yellow().bold());
    }

    // Generate the changelog
    let history = commit_history(&cwd, &cfg)?;
    let changelog = build_changelog(&cwd, &cfg, &history, None)?;
    let changelog_str = changelog.generate(TEMPLATE_CHANGELOG_STD)?;
    println!("{changelog_str}");

    Ok(())
}
