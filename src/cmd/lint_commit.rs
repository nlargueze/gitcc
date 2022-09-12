//! Lint command

use std::{env, io, path::PathBuf};

use anyhow::bail;
use clap::Parser;
use colored::Colorize;

use crate::{commit::ConvCommitMessage, config::Config};

/// Commit command arguments
#[derive(Debug, Parser)]
pub struct LintArgs {
    /// Path to the repo directory
    #[clap(long)]
    cwd: Option<PathBuf>,
    /// Commit message
    ///
    /// If omitted, the message is read from stdin
    #[clap(short, long)]
    message: Option<String>,
}

/// Lints a commit message
pub fn run(args: LintArgs) -> anyhow::Result<()> {
    // Set the working directory
    if let Some(cwd) = args.cwd {
        env::set_current_dir(cwd)?;
    }

    // Load the config
    let config = match Config::load()? {
        Some(c) => c,
        None => bail!("No configuration found"),
    };

    // get the commig message
    let msg = match &args.message {
        Some(msg) => msg.clone(),
        None => {
            let mut stdin = String::new();
            io::stdin()
                .read_line(&mut stdin)
                .expect("Cannot read stdin");
            stdin
        }
    };

    // validate the commit message
    ConvCommitMessage::parse(&msg, &config.commits)?;
    println!("{} Conventional commit OK", "✔".bright_green());

    Ok(())
}
