//! Installs custom hooks

use std::{fs, os::unix::prelude::PermissionsExt, process::exit};

use clap::Parser;

use console::{style, Term};
use gitext::{
    commands::shared::{load_config, set_current_dir_from_arg},
    git::set_config_install_hooks,
    hooks::create_git_hooks_scripts,
};
use log::debug;

/// install-hooks command arguments
#[derive(Debug, Parser)]
#[clap(author, version, about = "Installs custom hooks")]
pub struct Cli {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
}

fn main() {
    env_logger::init();

    let term = Term::stderr();

    let args = Cli::parse();

    // set CWD
    let cwd = set_current_dir_from_arg(&args.cwd);

    // load the config
    let config = load_config(&cwd, true);

    // create the hooks dir
    let hooks_dir = config.hooks_folder();
    if hooks_dir.exists() {
        match fs::remove_dir_all(&hooks_dir) {
            Ok(_) => {
                debug!("Removed existing hooks dir: {}", hooks_dir.display());
            }
            Err(err) => {
                term.write_line(
                    style(format!("✗ Failed to delete hooks dir : {err}"))
                        .red()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
                exit(1);
            }
        }
    }
    match fs::create_dir(&hooks_dir) {
        Ok(_) => {
            debug!("Created hooks dir: {}", hooks_dir.display());
        }
        Err(err) => {
            term.write_line(
                style(format!("✗ Failed to create hooks dir : {err}"))
                    .red()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            exit(1);
        }
    }

    // create the hooks scripts
    let scripts = match create_git_hooks_scripts(&config) {
        Ok(s) => s,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    // generate the files
    for (key, script) in scripts {
        let script_file = hooks_dir.join(&key);
        match fs::write(&script_file, script) {
            Ok(_) => {
                fs::set_permissions(&script_file, fs::Permissions::from_mode(0o755)).unwrap();
                term.write_line(
                    format!(
                        "{} {}",
                        style("✔").green(),
                        style(format!("Generated hook script for {key}")).bold()
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

    // add to git config
    let hooks_dir_short = hooks_dir.strip_prefix(&config.root_dir).unwrap();
    match set_config_install_hooks(hooks_dir_short) {
        Ok(_) => {
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style("Added .gitx/hooks folder to git config core.hooksPath").bold()
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
