//! Configuration

use std::{env, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    changelog::ChangelogConfig, commit::CommitsConfig, hook::HooksConfig, release::ReleaseConfig,
};

/// Config directory
pub const CONFIG_DIR: &str = ".repo";

/// Config file name
pub const CONFIG_FILE: &str = "config.toml";

/// Configuration object
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    /// Repo directory
    #[serde(skip)]
    #[serde(default = "set_repo_dir")]
    pub repo_dir: PathBuf,
    /// Commits config
    pub commits: CommitsConfig,
    /// Git hooks config
    pub hooks: HooksConfig,
    /// Changelog config
    pub changelog: ChangelogConfig,
    /// Release config
    pub release: ReleaseConfig,
}

/// Sets the default repo dir
fn set_repo_dir() -> PathBuf {
    env::current_dir().unwrap()
}

impl Config {
    /// Loads the configuration file
    ///
    /// It looks for the configuration file recursively in the parent
    /// directories.
    ///
    /// Returns [None] if the file is not found
    pub fn load() -> anyhow::Result<Option<Self>> {
        let mut cwd = env::current_dir()?;
        loop {
            let cfg_file = cwd.join(CONFIG_DIR).join(CONFIG_FILE);
            if cfg_file.exists() && cfg_file.is_file() {
                let data = fs::read_to_string(&cfg_file)?;
                let config = toml::from_str::<Self>(&data)?;
                return Ok(Some(config));
            }

            // Config file is not found
            if !cwd.pop() {
                return Ok(None);
            }
        }
    }

    /// Saves the [Config] inside the current working directory
    pub fn save(&self) -> anyhow::Result<()> {
        let cwd = env::current_dir()?;

        let cfg_dir = cwd.join(CONFIG_DIR);
        if !cfg_dir.exists() {
            fs::create_dir(&cfg_dir)?;
        }

        let cfg_str = toml::to_string(self)?;
        fs::write(cfg_dir.join(CONFIG_FILE), cfg_str)?;

        Ok(())
    }

    /// Returns the folder for hooks scripts
    pub fn hooks_dir(&self) -> PathBuf {
        self.repo_dir.join(CONFIG_DIR).join("hooks")
    }
}
