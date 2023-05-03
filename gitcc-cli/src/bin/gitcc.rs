//! Main CLI

use std::process::exit;

use clap::Parser;

use colored::Colorize;
use gitcc_cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let res = match cli.commands {
        Commands::Init(args) => gitcc_cli::init::run(args),
        Commands::Config(args) => gitcc_cli::config::run(args),
        Commands::Commit(args) => gitcc_cli::commit::run(args),
        Commands::Log(args) => gitcc_cli::log::run(args),
        Commands::Version(args) => gitcc_cli::version::run(args),
        Commands::Changelog(args) => gitcc_cli::changelog::run(args),
        Commands::Release(args) => gitcc_cli::release::run(args),
    };

    if let Err(err) = res {
        eprintln!();
        eprintln!("{}", format!("✗ {err}").bright_red());
        exit(1);
    }
}
