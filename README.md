# Ursyla
Ursyla is a runner for slynx. Since the slynx language is focused on being a lib, ursyla makes it easier to run code from slynx to test the frontend.

Ursyla also aims to be a program to make it easier to find and use other compilers to compile slynx code. For example using ursyla to compile slynx to js, or slynx to binary(if some backend).

## How Ursyla Works

Ursyla works with two kinds of files.

A **profile** lives at `~/.config/ursyla/<profile>.toml` and holds everything that
describes a backend: which binary to run, how ursyla names translate into the names
that binary expects, and the extra flags to hand it.

The **project file** is `ursyla.toml`, at the root of the project being compiled, and
holds what is specific to that project: which profile to use, where to write the
result, which file to compile.

Both of them are optional in the sense that every key may also be given on the
command line. The command line always wins.

```
ursyla --exec                      # ursyla.toml as it is
ursyla --exec --output dist/a.js   # ursyla.toml, but a different output
ursyla --profile js --output a.js  # nothing read from disk
```

Without `--exec` nothing is read from `ursyla.toml` and the two essential flags
(`--profile` and `--output`) have to be given. With `--exec` they come from the
file instead.

Where a value can come from more than one place, the order is:

| value  | first                        | then                        | then                            |
| ------ | ---------------------------- | --------------------------- | ------------------------------- |
| output | `--output`                   | `project.output`            | required, ursyla stops          |
| input  | `--input`                    | `project.input`             | `src/main.slx`                  |
| runner | `--runner`                   | `project.runner`            | `binary.target` of the profile  |
| profile| `--profile`                  | `project.profile`           | required, ursyla stops          |

`--runner` is only needed to compile with a binary other than the one the profile
names, since the profile already carries it.

## ursyla.toml

```toml
[project]
profile = "js"           # ~/.config/ursyla/js.toml
output = "dist/main.js"  # where the result is written
input = "src/main.slx"   # entry file, relative to the project root
runner = "slynx-to-js"   # optional, the profile already has a binary
```

Every key is optional, and every key has a flag with the same name.

## Profiles

`~/.config/ursyla/js.toml`:

```toml
[binary]
target = "slynx-to-js"          # binary ursyla runs
arguments = { "output" = "out" } # ursyla argument -> argument of that binary
flags = []                       # read, but not passed to the binary yet
```

`target` and `output` are the two arguments ursyla knows how to fill in, mapped
through `arguments` and handed to the binary as `--target=<entry>` and
`--output=<output>`. A name that is not in `arguments` is passed through as is.
The `target` here is the input file of the backend, which is why the binary ursyla
runs is called `runner` on the ursyla side.

## Adding an option

Everything that ursyla can be configured with lives in one list, `ursyla_fields!`
in `src/config.rs`:

```rust
crate::ursyla_fields! {
    ProjectConfig, ProjectFlags {
        /// Binary used to compile, defaults to the binary of the profile
        runner: Option<String> = None,
    }
}
```

That single line is the option: it adds the key to `[project]`, the `--runner` flag
with its help, and the rule that the flag overwrites the file. Adding an option
never means touching another struct, and there is no way to add a key without also
adding the flag.

The default is what serde uses when the key is missing from the file, so `input`
falls back to `src/main.slx` without a lookup at runtime.

To make an option mandatory when `--exec` is not passed, the flag carries the clap
attribute `#[arg(required_unless_present = "exec")]`.

Every option is an `Option<T>`, `None` meaning "not given" on both sides, which is
what keeps the merge to a single rule.

A new table (say `[dependencies]`) is the same recipe: one `ursyla_fields!` block,
a `#[command(flatten)] pub dependencies: DependencyFlags` in `src/cli.rs`, and one
more `config.dependencies.merge(&cli.dependencies)` in `src/main.rs`.

## Layout

| file               | what it holds                                                     |
| ------------------ | ----------------------------------------------------------------- |
| `src/main.rs`      | the order of things: parse, load, merge, run                       |
| `src/cli.rs`       | the flags, nothing else                                            |
| `src/config.rs`    | `ursyla.toml`: the option list, `UrsylaConfig` and the merge       |
| `src/profile.rs`   | `~/.config/ursyla/<profile>.toml`: backend and argument names      |
| `src/runner.rs`    | turning a config and a profile into a running process              |
| `src/macros.rs`    | `ursyla_fields!`, which generates an option list                   |

`cargo test` covers the merge, the flags and the arguments handed to the backend.