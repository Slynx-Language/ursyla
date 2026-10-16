use std::{collections::HashMap, path::PathBuf};

use color_eyre::eyre::eyre;
use serde::Deserialize;

pub const PROFILES_DIR: &str = ".config/ursyla";

#[derive(Debug, Clone, Deserialize)]
pub struct BinaryConfig {
    pub target: String,
    pub arguments: HashMap<String, String>,
    #[allow(dead_code)]
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProfileConfig {
    pub binary: BinaryConfig,
}

impl ProfileConfig {
    pub fn argument_replacement_for(&self, name: &str) -> String {
        self.binary
            .arguments
            .get(name)
            .cloned()
            .unwrap_or(name.to_string())
    }

    pub fn argument_for(&self, name: &str, value: &str) -> String {
        format!("--{}={}", self.argument_replacement_for(name), value)
    }
    pub fn load(name: &str) -> color_eyre::Result<Self> {
        let file = path(name)?;
        let content = std::fs::read_to_string(&file).map_err(|_| {
            eyre!(
                "could not find the profile '{}', it was expected at '{}'",
                name,
                file.display()
            )
        })?;

        toml::from_str(&content)
            .map_err(|err| eyre!("could not parse '{}': {}", file.display(), err))
    }

    pub fn profiles_folder() -> color_eyre::Result<PathBuf> {
        let home = std::env::home_dir().ok_or_else(|| eyre!("could not find the home folder"))?;
        Ok(home.join(PROFILES_DIR))
    }

    ///Lists all the profiles available
    pub fn list_profiles() -> color_eyre::Result<Vec<String>> {
        let profiles_folder = Self::profiles_folder()?;
        let mut out = Vec::new();
        for entry in std::fs::read_dir(profiles_folder)? {
            let Ok(entry) = entry else { continue };
            out.push(entry.path().with_extension("").display().to_string());
        }
        Ok(out)
    }
}

pub fn path(name: &str) -> color_eyre::Result<PathBuf> {
    let home = std::env::home_dir().ok_or_else(|| eyre!("could not find the home folder"))?;

    Ok(home.join(PROFILES_DIR).join(format!("{name}.toml")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn js() -> ProfileConfig {
        toml::from_str(
            "[binary]\ntarget = \"slynx-to-js\"\narguments = { \"output\" = \"dest\" }\nflags = []",
        )
        .unwrap()
    }

    #[test]
    fn a_mapped_argument_is_renamed_for_the_binary() {
        assert_eq!(js().argument_for("output", "out.js"), "--dest=out.js");
    }

    #[test]
    fn an_unmapped_argument_keeps_its_name() {
        assert_eq!(js().argument_for("target", "main.slx"), "--target=main.slx");
    }

    #[test]
    fn a_profile_lives_inside_the_home_config_folder() {
        assert!(
            path("js")
                .unwrap()
                .ends_with(format!("{PROFILES_DIR}/js.toml"))
        );
    }
}
