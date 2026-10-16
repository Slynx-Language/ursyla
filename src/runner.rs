use std::{path::Path, process::ExitStatus};

use color_eyre::eyre::eyre;

use crate::{
    config::{CONFIG_FILE_NAME, UrsylaConfig},
    profile::ProfileConfig,
};

pub struct Invocation {
    program: String,
    arguments: Vec<String>,
}

impl Invocation {
    pub fn build(
        project_root: &Path,
        config: &UrsylaConfig,
        profile: &ProfileConfig,
    ) -> color_eyre::Result<Self> {
        let entry = project_root.join(config.project.entry());
        let output = config.project.output.as_deref().ok_or_else(|| {
            eyre!(
                "no output defined, give `--output` or set `project.output` on '{}'",
                CONFIG_FILE_NAME
            )
        })?;

        Ok(Self {
            program: config
                .project
                .runner
                .clone()
                .unwrap_or_else(|| profile.binary.target.clone()),
            arguments: vec![
                profile.argument_for("target", &entry.display().to_string()),
                profile.argument_for("output", output),
            ],
        })
    }

    pub fn run(&self) -> color_eyre::Result<ExitStatus> {
        println!("Oh o programa: {:?}", self.program);
        println!("{:?}", self.arguments);
        Ok(std::process::Command::new(&self.program)
            .args(&self.arguments)
            .spawn()?
            .wait()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProjectFlags;

    fn profile() -> ProfileConfig {
        toml::from_str(
            "[binary]\ntarget = \"slynx-to-js\"\narguments = { \"output\" = \"dest\" }\nflags = []",
        )
        .unwrap()
    }

    fn config(output: Option<&str>) -> UrsylaConfig {
        let mut config = UrsylaConfig::default();
        config.project.merge(&ProjectFlags {
            profile: None,
            output: output.map(str::to_string),
            runner: None,
            input: None,
        });
        config
    }

    #[test]
    fn the_profile_owns_the_binary_and_maps_the_arguments() {
        let invocation =
            Invocation::build(Path::new("/project"), &config(Some("out.js")), &profile())
                .expect("should build with an output");

        assert_eq!(invocation.program, "slynx-to-js");
        assert_eq!(
            invocation.arguments,
            vec![
                "--target=/project/src/main.slx".to_string(),
                "--dest=out.js".to_string(),
            ]
        );
    }

    #[test]
    fn the_config_overwrites_the_runner_and_the_entry_of_the_profile() {
        let mut config = config(Some("dist/out.js"));
        config.project.merge(&ProjectFlags {
            profile: None,
            output: None,
            runner: Some("other-backend".to_string()),
            input: Some("src/lib.slx".to_string()),
        });
        let invocation = Invocation::build(Path::new("/project"), &config, &profile()).unwrap();

        assert_eq!(invocation.program, "other-backend");
        assert_eq!(
            invocation.arguments,
            vec![
                "--target=/project/src/lib.slx".to_string(),
                "--dest=dist/out.js".to_string(),
            ]
        );
    }

    #[test]
    fn an_output_is_needed() {
        assert!(Invocation::build(Path::new("/project"), &config(None), &profile()).is_err());
    }
}
