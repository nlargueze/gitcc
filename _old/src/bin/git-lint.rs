//! Lints a commit message

use std::{io, process::exit};

use clap::Parser;

use console::{style, Term};
use gitext::{
    commands::shared::{load_config, set_current_dir_from_arg},
    conventional::ConventionalCommitMessage,
};

/// Lint command
#[derive(Debug, Parser)]
#[clap(author, version, about = "Lints a commit message")]
pub struct Cli {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// Commit message (if ommitted, the message will be read from stdin)
    #[clap(short, long)]
    pub msg: Option<String>,
}

fn main() {
    env_logger::init();

    let term = Term::stderr();

    let args = Cli::parse();

    // set CWD
    let cwd = set_current_dir_from_arg(&args.cwd);

    // load the config
    let config = load_config(&cwd, true);

    // get the commig message
    let commit = match &args.msg {
        Some(c) => c.to_string(),
        None => {
            // input is piped
            if atty::is(atty::Stream::Stdin) {
                term.write_line(
                    style("✗ pass a commit message as an option or pipe input")
                        .red()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
                exit(1);
            }
            let mut stdin = String::new();
            io::stdin()
                .read_line(&mut stdin)
                .expect("Cannot read stdin");
            stdin
        }
    };

    // validate the commit message
    let _commit = match ConventionalCommitMessage::parse(&commit, &config.valid_commit_types()) {
        Ok(c) => {
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style("Conventional commit OK").bold()
                )
                .as_str(),
            )
            .unwrap();
            c
        }
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };
}
