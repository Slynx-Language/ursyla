use color_eyre::eyre::eyre;
use serde::Deserialize;

use crate::profile::ProfileConfig;

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub profile: String,
    pub output: String,
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
                output: String::new(),
            },
        }
    }

    pub fn with_profile(self, profile: String) -> Self {
        Self {
            project: ProjectConfig {
                profile,
                output: self.project.output,
            },
        }
    }

    pub fn with_output(self, output: String) -> Self {
        Self {
            project: ProjectConfig {
                profile: self.project.profile,
                output,
            },
        }
    }
    pub fn get_profile(&self) -> color_eyre::Result<ProfileConfig> {
        let home = std::env::home_dir().ok_or(eyre!("Should contain home folder"))?;
        let file = home.join(format!(".config/ursyla/{}.toml", self.project.profile));
        let content = std::fs::read_to_string(&file).map_err(|_| {
            eyre!(
                "Could not find profile '{}'. Please define some at '{}'",
                self.project.profile,
                file.display().to_string()
            )
        })?;
        let toml = toml::from_str(&content)?;
        Ok(toml)
    }
}
