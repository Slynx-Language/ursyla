use clap::Parser;

use crate::config::ProjectFlags;

#[derive(Debug, Parser)]
pub struct CliArguments {
    /// Take ursyla.toml as the base configuration, the other flags overwrite it
    #[arg(long)]
    pub exec: bool,
    ///Lists all the available profiles when running ursyla
    #[arg(long)]
    pub list_profiles: bool,

    #[command(flatten)]
    pub project: ProjectFlags,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn essentials_are_required_without_exec() {
        assert!(CliArguments::try_parse_from(["ursyla"]).is_err());
        assert!(CliArguments::try_parse_from(["ursyla", "--profile", "js"]).is_err());
        assert!(
            CliArguments::try_parse_from(["ursyla", "--profile", "js", "--output", "out.js"])
                .is_err()
        );

        let cli = CliArguments::try_parse_from([
            "ursyla",
            "--profile",
            "js",
            "--target",
            "slynx-to-js",
            "--output",
            "out.js",
        ])
        .unwrap();

        assert!(!cli.exec);
        assert_eq!(cli.project.profile.as_deref(), Some("js"));
        assert_eq!(cli.project.target.as_deref(), Some("slynx-to-js"));
        assert_eq!(cli.project.output.as_deref(), Some("out.js"));
        assert_eq!(cli.project.input, None);
    }

    #[test]
    fn exec_only_takes_what_it_needs_from_the_file() {
        let cli = CliArguments::try_parse_from(["ursyla", "--exec"]).unwrap();

        assert!(cli.exec);
        assert_eq!(cli.project.profile, None);
        assert_eq!(cli.project.target, None);
        assert_eq!(cli.project.output, None);
        assert_eq!(cli.project.input, None);
    }

    #[test]
    fn flags_are_optional_next_to_exec() {
        let cli = CliArguments::try_parse_from(["ursyla", "--exec", "--output", "out.js"]).unwrap();

        assert_eq!(cli.project.output.as_deref(), Some("out.js"));
        assert_eq!(cli.project.profile, None);
    }
}
