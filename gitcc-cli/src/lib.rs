//! CLI commands

use clap::{Parser, Subcommand};

pub mod changelog;
pub mod commit;
pub mod config;
pub mod init;
pub mod log;
pub mod release;
mod util;
pub mod version;

#[derive(Debug, Parser)]
#[clap(
    version,
    about = "Misc. tools for conventional commits, versioning, and changelogs"
)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Inititializes the config
    Init(init::InitArgs),
    /// Shows the current configuration
    Config(config::ConfigArgs),
    /// Creates a conventional commit
    Commit(commit::CommitArgs),
    /// Displays the commit history
    Log(log::LogArgs),
    /// Checks the current version and determines the next version
    Version(version::VersionArgs),
    /// Generates the changelog
    Changelog(changelog::ChangelogArgs),
    /// Creates a release
    Release(release::ReleaseArgs),
}

/// Executes the program
pub fn exec(cli: Cli) -> anyhow::Result<()> {
    match cli.commands {
        Commands::Init(args) => init::run(args),
        Commands::Config(args) => config::run(args),
        Commands::Commit(args) => commit::run(args),
        Commands::Version(args) => version::run(args),
        Commands::Log(args) => log::run(args),
        Commands::Changelog(args) => changelog::run(args),
        Commands::Release(args) => release::run(args),
    }
}
