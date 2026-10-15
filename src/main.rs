mod profile;
mod ursyla;

use std::{path::Path, process::ExitStatus};

use clap::Parser;

use crate::{
    profile::ProfileConfig,
    ursyla::{ProjectConfig, UrsylaConfig},
};

fn get_config(current_dir: &Path) -> color_eyre::Result<UrsylaConfig> {
    let file = current_dir.join("ursyla.toml");
    let content = std::fs::read_to_string(&file)?;
    Ok(toml::from_str(&content)?)
}

#[derive(clap::Parser)]
pub struct CliArguments {
    #[arg(long)]
    exec: bool,
    #[arg(long, required_unless_present = "exec")]
    profile: Option<String>,
    #[arg(long, required_unless_present = "exec")]
    target: Option<String>,
    #[arg(long, required_unless_present = "exec")]
    output: Option<String>,
    #[arg(long)]
    input: Option<String>,
}

fn run(
    entry: &Path,
    profile: ProfileConfig,
    project_config: UrsylaConfig,
) -> color_eyre::Result<ExitStatus> {
    let entry_source = entry.join(&project_config.project.input);
    let mut child = std::process::Command::new(&profile.binary.target);
    let target_parameter = profile.argument_replacement_for("target");
    let output_parameter = profile.argument_replacement_for("output");
    child
        .arg(format!(
            "--{target_parameter}={}",
            entry_source.display().to_string()
        ))
        .arg(format!(
            "--{output_parameter}={}",
            project_config.project.output
        ));
    let mut child = child.spawn()?;
    let out = child.wait()?;
    Ok(out)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let arguments = CliArguments::parse();
    let current_dir = std::env::current_dir()?;
    let (profile, project_config) = if arguments.exec {
        let config = get_config(&current_dir)?;
        (config.get_profile()?, config)
    } else {
        let profile = arguments
            .profile
            .expect("Should provide profile since not running with 'run'");
        let output = arguments
            .output
            .expect("Should provide output since not runnign with 'run'");
        let empty = UrsylaConfig::empty()
            .with_profile(profile)
            .with_output(output);

        (empty.get_profile()?, empty)
    };

    let status = run(&current_dir, profile, project_config)?;
    std::process::exit(status.code().unwrap_or(1));
}
