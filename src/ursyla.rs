use color_eyre::eyre::eyre;
use serde::Deserialize;

use crate::profile::ProfileConfig;

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub profile: String,
}

#[derive(Debug, Deserialize)]
pub struct UrsylaConfig {
    pub project: ProjectConfig,
}
impl UrsylaConfig {
    pub fn empty() -> Self {
        Self {
            project: ProjectConfig {
                profile: String::new(),
            },
        }
    }

    pub fn with_profile(self, profile: String) -> Self {
        Self {
            project: ProjectConfig { profile },
        }
    }

    pub fn get_profile(&self) -> color_eyre::Result<ProfileConfig> {
        let home = std::env::home_dir().ok_or(eyre!("Should contain home folder"))?;
        let file = home.join(format!(".config/ursyla/{}.toml", self.project.profile));
        let toml = toml::from_str(&std::fs::read_to_string(file)?)?;
        Ok(toml)
    }
}
