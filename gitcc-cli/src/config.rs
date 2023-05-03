//! `config` command

use std::env;

use clap::Parser;
use gitcc_core::Config;

use crate::{info, new_line};

/// `config` command arguments
#[derive(Debug, Parser)]
pub struct ConfigArgs {
    /// Displays the config as YAML
    #[clap(long)]
    pub yaml: bool,
}

/// Executes the commnad `config`
pub fn run(args: ConfigArgs) -> anyhow::Result<()> {
    new_line!();

    let cwd = env::current_dir()?;
    let config = Config::load_from_fs(&cwd)?;
    let config = if let Some(c) = config {
        c
    } else {
        info!("using default config");
        Config::default()
    };

    new_line!();
    let config_str = if args.yaml {
        config.to_yaml()?
    } else {
        config.to_toml()?
    };
    println!("{}", config_str);

    Ok(())
}
