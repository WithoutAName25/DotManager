# DotManager

***WARNING: This tool is currently in the experimental stage. Use at your own risk.***

DotManager is a command-line tool designed to simplify the management of dotfiles on your system.
Dotfiles are configuration files that typically start with a dot (.) and are commonly used to personalize your system environment.
This tool helps organize and sync these dotfiles across different machines.

## How does it work?

DotManager works by moving your configuration files to a central folder (by default `~/.dotfiles`)
and creating symlinks to their new locations.
This allows you to maintain a centralized repository of your dotfiles,
making it easy to sync them across different machines using external tools such as Git.

## Installation

To use DotManager, you need to have Rust installed.
If you don't have Rust installed, you can get it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

Once Rust is installed, you can install DotManager using the following command:

```bash
cargo install dot-manager
```

## Setup

Run the following command to interactively generate the config file for dot-manager:
```bash
dot-manager setup
```

## Usage

See `dot-manager --help`.

## Example

Assuming you have a configuration file located at `~/.config/example_config`,
you can sync it with DotManager using the following command:

```bash
dot-manager add ~/.config/example_config
```

This will move the example_config file or folder to ~/.dotfiles and create a symlink at the original location.

## Contributing

If you encounter any issues or have suggestions for improvements,
feel free to open an issue or create a pull request on the [GitHub repository](https://github.com/WithoutAName25/DotManager).

## License

DotManager is licensed under the [MIT License](LICENSE).