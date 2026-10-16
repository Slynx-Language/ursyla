/// Declares, in one place, every option of a configuration table.
///
/// A single entry generates the three things that must never drift apart:
///   * `$config`, the struct deserialized from the toml file;
///   * `$flags`, the cli arguments of the same options;
///   * `merge`, where a flag given on the cli overwrites the toml value.
///
/// Syntax of an entry: `[<clap attrs>] <name>: <type> = <default>,`
///
///   * `default` is what the option falls back to when the key is missing from
///     the toml file, and what feeds `Default`, which serde relies on for the
///     keys that were not written;
///   * the attributes describe the cli flag, doc comments included, which is
///     why the flags are the ones carrying the documentation;
///   * every option is an `Option<T>`, `None` being what "not given" looks like
///     both on the toml file and on the cli, which is what lets the merge be
///     always `flag overwrites config`.
///
/// Marking an option as mandatory is a cli concern, so it is written as one:
/// `#[arg(required_unless_present = "exec")]`.
///
/// Usage:
///
/// ```ignore
/// crate::ursyla_fields! {
///     ProjectConfig, ProjectFlags {
///         /// Profile to run.
///         #[arg(required_unless_present = "exec")]
///         profile: Option<String> = None,
///         /// Entry file.
///         input: Option<String> = Some(DEFAULT_INPUT.to_string()),
///     }
/// }
/// ```
#[macro_export]
macro_rules! ursyla_fields {
    (
        $config:ident, $flags:ident {
            $(
                $(#[$meta:meta])*
                $name:ident : $ty:ty = $default:expr ,
            )*
        }
    ) => {
        #[derive(Debug, Clone, ::serde::Deserialize)]
        #[serde(default)]
        pub struct $config {
            $( pub $name: $ty, )*
        }

        impl Default for $config {
            fn default() -> Self {
                Self { $( $name: $default, )* }
            }
        }

        impl $config {
            pub fn merge(&mut self, flags: &$flags) -> &mut Self {
                $(
                    if flags.$name.is_some() {
                        self.$name = flags.$name.clone();
                    }
                )*
                self
            }
        }

        #[derive(Debug, Clone, ::clap::Args)]
        pub struct $flags {
            $(
                $(#[$meta])*
                #[arg(long)]
                pub $name: $ty,
            )*
        }
    };
}
