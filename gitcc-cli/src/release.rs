//! `release` command

use std::env;

use clap::Parser;
use colored::Colorize;
use gitcc_core::Config;

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
        eprintln!("{} using default config", "i".blue().bold());
        Config::default()
    };

    // Checks that the repo is clean
    let dirty_files = gitcc_core::dirty_files(&cwd)?;
    if !dirty_files.is_empty() {
        eprintln!("{} repo is dirty", "i".blue().bold());
        // for f in dirty_files {
        //     eprintln!("  {f}");
        // }
        // eprintln!("{} aborted", "!".red().bold());
        // return Ok(());
    }

    eprintln!("1: make sure there is no untracked/uncommitted changes");
    eprintln!("2: create the changelog file");
    eprintln!("3: commit the changelog");
    eprintln!("4: tag the repo with the next version (annotated tag, leading 'v')");
    eprintln!("5: push with --follow-tags");
    eprintln!("6: create a Github release");
    eprintln!("7: publish the release to creates.io");

    Ok(())
}
