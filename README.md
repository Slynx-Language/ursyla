# Ursyla
Ursyla is a runner for slynx. Since the slynx language is focused on being a lib, ursyla makes it easier to run code from slynx to test the frontend.

Ursyla also aims to be a program to make it easier to find and use other compilers to compile slynx code. For example using ursyla to compile slynx to js, or slynx to binary(if some backend).

## How Ursyla Works

Ursyla works by defining profiles on `~/.config/ursyla/profilename.toml`
A profile contains all the informations related on how to compile the slynx code. For example, on compiling the slynx project with a 'js' profile, ursyla will read at `~/.config/ursyla/js.toml` and check things like:
  * What is the binary to use
  * How to map ursyla argument to the binary arguments
  * Additional flags to give the frontend
And others. 

When creating a project with ursyla(on the future) there will be an 'ursyla.toml' file which will be able to handle things like:
  * Project dependencies
  * What is the default profile to use
  * Custom scripts
