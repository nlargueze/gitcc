//! Performs a release.

use std::{fs, process::exit};

use clap::Parser;

use console::{style, Term};
use gitext::{
    changelog::ChangeLog,
    commands::shared::{load_config, set_current_dir_from_arg},
    git::{git_add, git_commit, git_push_follow_tags, git_set_tag, git_status_porcelain},
    version::{exec_bump_commands, get_repo_next_version},
};

/// Release command
#[derive(Debug, Parser)]
#[clap(author, version, about = "Performs a release")]
pub struct Cli {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// Allows uncommitted changes when creating the release
    #[clap(long)]
    pub allow_dirty: bool,
    /// If set, the changelog and tag are committed
    #[clap(long, short)]
    pub commit: bool,
    /// If set, commits and tags are pushed
    #[clap(long, short)]
    pub push: bool,
    /// If set, the RELEASENOTES.md file is not generated
    #[clap(long)]
    pub no_release_notes: bool,
}

fn main() {
    env_logger::init();

    let term = Term::stderr();

    let args = Cli::parse();

    // set CWD
    let cwd = set_current_dir_from_arg(&args.cwd);

    // load the config
    let config = load_config(&cwd, true);

    // 1. check for uncommitted changes
    match git_status_porcelain() {
        Ok(status) => {
            if let Some(files_list) = status {
                if args.allow_dirty {
                    term.write_line(
                        style("i Repo has uncommited changes:")
                            .yellow()
                            .bold()
                            .to_string()
                            .as_str(),
                    )
                    .unwrap();
                    term.write_line(style(&files_list).yellow().to_string().as_str())
                        .unwrap();
                } else {
                    term.write_line(
                        style("✗ Repo has uncommited changes:")
                            .red()
                            .bold()
                            .to_string()
                            .as_str(),
                    )
                    .unwrap();
                    term.write_line(style(&files_list).red().to_string().as_str())
                        .unwrap();
                    exit(1);
                }
            }
        }
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    // 2. generate the changelog
    let changelog = match ChangeLog::init() {
        Ok(cl) => cl,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    let (next_version, prev_version) = match get_repo_next_version(&config) {
        Ok((next_version, prev_version)) => (next_version, prev_version),
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    let (changelog_str, releasenotes_str) =
        match changelog.generate(&config, &next_version.to_string()) {
            Ok(s) => s,
            Err(err) => {
                term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                    .unwrap();
                exit(1);
            }
        };

    if !args.commit {
        term.write_line(
            format!(
                "{} {}",
                style("i").green(),
                style(format!(
                    "{} --> {}",
                    prev_version
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "<none>".to_string()),
                    next_version
                ))
                .bold()
            )
            .as_str(),
        )
        .unwrap();
        term.write_line(
            format!(
                "{} {}",
                style("i").green(),
                style("CHANGELOG ↴".to_string()).bold()
            )
            .as_str(),
        )
        .unwrap();
        println!("{changelog_str}");
        exit(0);
    }

    match fs::write(cwd.join("CHANGELOG.md"), changelog_str) {
        Ok(_) => {
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style("Generated changelog").bold()
                )
                .as_str(),
            )
            .unwrap();
        }
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    }

    if !args.no_release_notes {
        match fs::write(cwd.join("CHANGELOG_RELEASE.md"), releasenotes_str) {
            Ok(_) => {
                term.write_line(
                    format!(
                        "{} {}",
                        style("✔").green(),
                        style("Generated release notes").bold()
                    )
                    .as_str(),
                )
                .unwrap();
            }
            Err(err) => {
                term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                    .unwrap();
                exit(1);
            }
        }
    }

    // 3. Bump the package versions
    match exec_bump_commands(&config, &next_version.to_string()) {
        Ok(exec_commands) => {
            for cmd in exec_commands {
                term.write_line(
                    format!(
                        "{} {}",
                        style("✔").green(),
                        style(format!("Executed bump command: {cmd} ")).bold()
                    )
                    .as_str(),
                )
                .unwrap();
            }
        }
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    }

    // 4. Commit the changes
    match git_add() {
        Ok(_) => {
            term.write_line(
                format!("{} {}", style("✔").green(), style("Staged changes").bold()).as_str(),
            )
            .unwrap();
        }
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    }

    let commit_msg = format!("chore(release): created release {}", next_version);
    match git_commit(&commit_msg) {
        Ok(_) => {
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style(format!("Created commit for release {next_version} ")).bold()
                )
                .as_str(),
            )
            .unwrap();
        }
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    }

    // 5. Tag the commit
    let new_git_tag = format!("v{}", next_version);
    match git_set_tag(
        new_git_tag.as_str(),
        format!("Release {next_version}").as_str(),
    ) {
        Ok(_) => {
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style(format!("Tagged commit as {new_git_tag} ")).bold()
                )
                .as_str(),
            )
            .unwrap();
        }
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    }

    // 6.push the commit and tag
    if args.push {
        match git_push_follow_tags() {
            Ok(_) => {
                term.write_line(
                    format!(
                        "{} {}",
                        style("✔").green(),
                        style("Pushed commit and tag").bold()
                    )
                    .as_str(),
                )
                .unwrap();
            }
            Err(err) => {
                term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                    .unwrap();
                exit(1);
            }
        }
    }
}
