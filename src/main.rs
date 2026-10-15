mod profile;
mod ursyla;

use std::{path::Path, process::ExitStatus};

use clap::Parser;

use crate::{profile::ProfileConfig, ursyla::UrsylaConfig};

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
}

fn run(entry: &Path, profile: ProfileConfig) -> color_eyre::Result<ExitStatus> {
    let entry_source = entry.join("src/main.slx");
    let mut child = std::process::Command::new(profile.binary.target);
    let target_parameter = profile
        .binary
        .arguments
        .get("target")
        .map(|s| s.as_str())
        .unwrap_or("target");
    child.arg(format!(
        "--{target_parameter}={}",
        entry_source.display().to_string()
    ));
    let mut child = child.spawn()?;
    let out = child.wait()?;
    Ok(out)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let arguments = CliArguments::parse();
    let current_dir = std::env::current_dir()?;
    let profile = if arguments.exec {
        let config = get_config(&current_dir)?;
        config.get_profile()?
    } else {
        let profile = arguments
            .profile
            .expect("Should provide profile since not running with 'run'");
        UrsylaConfig::empty().with_profile(profile).get_profile()?
    };
    let status = run(&current_dir, profile)?;
    std::process::exit(status.code().unwrap_or(1));
}
