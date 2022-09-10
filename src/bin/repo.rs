//! `repo` command

use std::process::exit;

use clap::{Parser, Subcommand};

use colored::Colorize;
use repo_tools::cmd::{self, commit::CommitArgs, init::InitArgs};

#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Inititializes repo
    Init(InitArgs),
    /// Commits a message
    Commit(CommitArgs),
    /// Alias for `commit`
    C(CommitArgs),
}

fn main() {
    let cli = Cli::parse();

    let res = match cli.commands {
        Commands::Init(args) => cmd::init::run(args),
        Commands::Commit(args) => cmd::commit::run(args),
        Commands::C(args) => cmd::commit::run(args),
    };

    if let Err(err) = res {
        eprintln!();
        eprintln!("{}", format!("✗ {err}").bright_red());
        exit(1);
    }
}
