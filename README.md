# Sabita-TUI

![Lint/Security/Dependencies/Tests](https://github.com/MikyStar/Sabita-TUI/actions/workflows/test-lint-audit.yml/badge.svg)
![crates.io version](https://img.shields.io/crates/v/sabita_tui)

A Terminal User Interface for the Sabita sudoku package.

## CLI

### Install

#### From a binary release

**You will need to [install jq](https://jqlang.org/download/)**

```sh
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/MikyStar/Sabita-TUI/refs/heads/main/install.sh | sh
```

#### From Cargo

**You will need to [install Rust](https://www.rust-lang.org/tools/install)**

```sh
cargo install sabita_tui
```

### Use

```sh
# Generator
sabita_tui # Runs the interface

# Help
sabita_tui -h
sabita_tui --help

# Version
sabita_tui -v
sabita_tui --version
```

## Dev

### Commands

> Many aliases and sequences are handled through [cargo-make](https://crates.io/crates/cargo-make) *you will need to install it*

```sh
cargo run # Builds and run the project

cargo fmt # Format code
cargo fmt -- --check # Throw error if unformated code

cargo clippy # Advanced linter
cargo clippy --fix # Fix auto fixable

cargo build # Only build it

cargo test # Run all unit tests
cargo test <file without extension> # Run specific test file inside the 'tests' folder (don't write it in path)
cargo test <specific function name> # Run specific test function

cargo add <package> [--dev] # Install a project dependency (or a dev dependency)
cargo install <package> # Install a system wide dependency

cargo doc # Generates HTML documentation

cargo clean # Remove 'targer' directory (build artifacts, doc ...)

cargo publish # Publish project to crates.io registry

cargo tree # Recursize list of lib dependencies
```

### Git hooks

Git hooks are handled with [rusty-hook](https://github.com/swellaby/rusty-hook), to enable them after a fresh install, run `cargo test`

### Tasks

Using [CLI-Manager](https://github.com/MikyStar/CLI-Manager) for task handling.
