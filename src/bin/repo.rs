//! `repo` command

use std::process::exit;

use clap::{Parser, Subcommand};

use colored::Colorize;
use repo_tools::cmd::{
    self, changelog::ChangelogArgs, commit::CommitArgs, init::InitArgs,
    install_hooks::InstallHooksArgs, lint_commit::LintArgs, release::ReleaseArgs,
    version::BumpArgs,
};

#[derive(Debug, Parser)]
#[clap(version, about = "Repo management utilities")]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Inititializes the repo (config)
    Init(InitArgs),
    /// Creates a conventional commit
    Commit(CommitArgs),
    /// Alias for `commit`
    C(CommitArgs),
    /// Lints a commit message
    LintCommit(LintArgs),
    /// Installs git hooks
    InstallHooks(InstallHooksArgs),
    /// Checks the current version and determines the next version
    Version(BumpArgs),
    /// Generates the changelog
    Changelog(ChangelogArgs),
    /// Creates a release
    Release(ReleaseArgs),
}

fn main() {
    println!();

    let cli = Cli::parse();

    let res = match cli.commands {
        Commands::Init(args) => cmd::init::run(args),
        Commands::Commit(args) => cmd::commit::run(args),
        Commands::C(args) => cmd::commit::run(args),
        Commands::LintCommit(args) => cmd::lint_commit::run(args),
        Commands::InstallHooks(args) => cmd::install_hooks::run(args),
        Commands::Version(args) => cmd::version::run(args),
        Commands::Changelog(args) => cmd::changelog::run(args),
        Commands::Release(args) => cmd::release::run(args),
    };

    if let Err(err) = res {
        eprintln!();
        eprintln!("{}", format!("✗ {err}").bright_red());
        exit(1);
    }
}
