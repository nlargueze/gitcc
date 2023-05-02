//! `log` command

use std::env;

use clap::Parser;
use colored::Colorize;
use gitcc_core::Config;

use crate::info;

/// `log` command arguments
#[derive(Debug, Parser)]
pub struct LogArgs {}

/// Executes the commnad `init`
pub fn run(_args: LogArgs) -> anyhow::Result<()> {
    let cwd = env::current_dir()?;
    let config = Config::load_from_fs(&cwd)?;
    let config = if let Some(c) = config {
        c
    } else {
        info!("using default config");
        Config::default()
    };

    let history = gitcc_core::commit_history(&cwd, &config)?;
    for c in history.commits {
        println!("{}{}", "commit: ".blue().bold(), c.id.to_string().bold());
        println!("{}{}", "date: ".cyan(), c.date);
        println!(
            "{}{} <{}>",
            "author: ".cyan(),
            c.author_name,
            c.author_email
        );
        // println!(
        //     "{}{} <{}>",
        //     "committer: ".cyan(),
        //     c.committer_name,
        //     c.committer_email
        // );
        println!("{}", c.raw_message);
    }

    Ok(())
}
