//! Configuration

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

/// Config directory
pub const CONFIG_DIR: &str = ".repo";

/// Config file name
pub const CONFIG_FILE: &str = "config.toml";

/// Configuration object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Root directory
    #[serde(skip)]
    pub root_dir: PathBuf,
    /// Commits config
    pub commit: CommitsConfig,
    /// Custom hooks
    pub hooks: BTreeMap<String, Vec<String>>,
    /// Changelog config
    pub changelog: ChangeLogConfig,
    /// Release config
    pub release: ReleaseConfig,
}

/// Commits configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitsConfig {
    /// Commit types causing a minor version increment
    pub types_inc_minor: Vec<String>,
    /// Commit types (key + description)
    pub types: BTreeMap<String, String>,
}

impl Default for CommitsConfig {
    fn default() -> Self {
        let mut types = BTreeMap::new();
        types.insert("feat".to_string(), "New features".to_string());
        types.insert("fix".to_string(), "Bug fixes".to_string());
        types.insert("docs".to_string(), "Documentation".to_string());
        types.insert("style".to_string(), "Code styling".to_string());
        types.insert("refactor".to_string(), "Code refactoring".to_string());
        types.insert("perf".to_string(), "Performance Improvements".to_string());
        types.insert("test".to_string(), "Testing".to_string());
        types.insert("build".to_string(), "Build system".to_string());
        types.insert("ci".to_string(), "Continuous Integration".to_string());
        types.insert("cd".to_string(), "Continuous Delivery".to_string());
        types.insert("chore".to_string(), "Other changes".to_string());

        let types_inc_minor = vec!["feat".to_string()];

        Self {
            types,
            types_inc_minor,
        }
    }
}

/// Changelog configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeLogConfig {
    /// Types to include in the changelog
    pub types: Vec<String>,
}

impl Default for ChangeLogConfig {
    fn default() -> Self {
        let types = vec![
            "feat".to_string(),
            "fix".to_string(),
            "docs".to_string(),
            "perf".to_string(),
            "test".to_string(),
            "build".to_string(),
            "ci".to_string(),
            "cd".to_string(),
            "chore".to_string(),
        ];

        Self { types }
    }
}

/// Release configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReleaseConfig {
    /// Commands to execute when the version is bumped
    pub bump_commands: Vec<String>,
}

impl Config {
    /// Loads the configuration file for the current working directory
    pub fn load(repo_path: &Path) -> Result<Option<Self>> {
        let cfg_file = repo_path.join(CONFIG_DIR).join(CONFIG_FILE);
        if cfg_file.exists() {
            let cfg_str = fs::read_to_string(&cfg_file)?;
            let mut cfg = toml::from_str::<Config>(&cfg_str)?;
            cfg.root_dir = repo_path.to_path_buf();
            Ok(Some(cfg))
        } else {
            Ok(None)
        }
    }

    /// Saves a [Configuration] to the repo
    pub fn save(&self, repo_path: &Path) -> Result<()> {
        let cfg_str = toml::to_string(self)?;
        if !repo_path.join(CONFIG_DIR).exists() {
            fs::create_dir(repo_path.join(CONFIG_DIR))?;
        }
        fs::write(repo_path.join(CONFIG_DIR).join(CONFIG_FILE), cfg_str)?;
        Ok(())
    }

    /// Returns a list of valid commit types
    pub fn valid_commit_types(&self) -> Vec<String> {
        self.commit.types.keys().cloned().collect()
    }

    /// Checks if a commit type causes a minor increment
    pub fn type_is_minor_inc(&self, commit_type: &str) -> bool {
        self.commit
            .types_inc_minor
            .contains(&commit_type.to_string())
    }

    /// Returns the folder for hook
    pub fn hooks_folder(&self) -> PathBuf {
        self.root_dir.join(CONFIG_DIR).join("hooks")
    }
}
