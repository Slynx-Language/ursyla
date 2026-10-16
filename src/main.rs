mod cli;
mod config;
mod macros;
mod profile;
mod runner;

use clap::Parser;

use crate::{cli::CliArguments, config::UrsylaConfig, runner::Invocation};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = CliArguments::parse();
    let project_root = std::env::current_dir()?;

    if cli.list_profiles {
        for profile in profile::ProfileConfig::list_profiles()? {
            println!("Profile: {profile}");
        }
        std::process::exit(0);
    }

    let mut config = if cli.exec {
        config::load(&project_root)?
    } else {
        UrsylaConfig::default()
    };
    config.project.merge(&cli.project);

    let profile = profile::ProfileConfig::load(config.profile_name()?)?;
    let invocation = Invocation::build(&project_root, &config, &profile)?;

    std::process::exit(invocation.run()?.code().unwrap_or(1));
}
