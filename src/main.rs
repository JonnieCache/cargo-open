//! A cargo subcommand to open an installed crate in your `$EDITOR`,
//! modelled on [`bundle open`](https://bundler.io/v2.5/man/bundle-open.1.html).
//!
//! Note that the intended use is to open a crate's source for reading.
//! Making changes to installed crates is not reccommended, and may produce unexpected results.
//! Instead, clone the crate locally and [specify the path in your Cargo.toml](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-path-dependencies).
//!
//! # Installation
//!
//! This is a binary crate, so it's [installed manually](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html) rather than as a dependency:
//!
//! ```sh
//! cargo install cargo-open
//! ```
//!
//! # Usage
//!
//! ```sh
//! cargo open clap
//! ```
//!
//! # Configuration
//!
//! The editor command is picked from the `CARGO_EDITOR`, `VISUAL`, or `EDITOR` environment variables,
//! checked in that order.
//!
//! Specify a different manifest file with the `--manifest-path` option.
//! By default, `Cargo.toml` in the current directory is used.
//!
//! # Todo/Contributing
//!
//! There aren't any tests, as this is just glueing together the [cargo-metadata](https://crates.io/crates/cargo_metadata)
//! crate and some bits from the standard library. As such it should be robust, and what little logic there is should be
//! adequately constrained by the type system.
//!
//! Regardless, if you have any problems, suggestions or improvements, feel free to create an issue or PR.
//!
//! This project uses cargo-readme to export this README from the doc comments.
//!
//! # Attribution
//!
//! The original cargo-open was authored by Carol Nichols ([@carols10cents](https://github.com/carols10cents)), and crate ownership was transferred
//! in may 2024. Many thanks to Carol for all her work in the rust community.  

use cargo_metadata::{Metadata, MetadataCommand, Package};
use clap::{error::ErrorKind, CommandFactory, Error, Parser};
use std::{
    path::PathBuf,
    process::{Command, ExitStatus},
};

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Cli {
    Open(Args),
}

/// Open an installed crate in your editor
#[derive(clap::Args)]
struct Args {
    /// The name of the crate to open
    #[arg(value_name = "CRATE")]
    package_name: String,

    /// Use a specific manifest file
    #[arg(long, value_name = "PATH")]
    manifest_path: Option<PathBuf>,
}

fn main() {
    if let Err(err) = try_main() {
        let mut command = Cli::command();
        err.format(&mut command).exit();
    }
}

fn try_main() -> Result<(), Error> {
    let Cli::Open(args) = Cli::parse();

    let metadata = get_metadata(args.manifest_path)?;
    let package = get_package(&args.package_name, &metadata)?;
    let package_path = get_package_path(package)?;
    let editor_path = get_editor_path()?;

    run_editor(editor_path, package_path)?;

    Ok(())
}

fn get_metadata(manifest_path: Option<PathBuf>) -> Result<Metadata, Error> {
    let mut cmd = MetadataCommand::new();
    if let Some(manifest_path) = manifest_path {
        cmd.manifest_path(manifest_path);
    }

    cmd.exec()
        .map_err(|e| Error::raw(ErrorKind::Io, format!("Metadata error: {}", e)))
}

fn get_package<'a>(package_name: &'a str, metadata: &'a Metadata) -> Result<&'a Package, Error> {
    metadata
        .packages
        .iter()
        .find(|package| package.name == package_name)
        .ok_or_else(|| {
            Error::raw(
                ErrorKind::InvalidValue,
                format!("Package not found: {}", package_name),
            )
        })
}

fn get_package_path(package: &Package) -> Result<PathBuf, Error> {
    package
        .manifest_path
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| Error::raw(ErrorKind::Io, "Path error"))
}

fn get_editor_path() -> Result<PathBuf, Error> {
    std::env::var("CARGO_EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .or_else(|_| std::env::var("EDITOR"))
        .map_err(|_| Error::raw(ErrorKind::Io, "Cannot resolve editor"))
        .map(PathBuf::from)
}

fn run_editor(editor_path: PathBuf, package_path: PathBuf) -> Result<ExitStatus, Error> {
    Command::new(editor_path.clone())
        .arg(package_path.clone())
        .status()
        .map_err(|e| Error::raw(ErrorKind::Io, e))
}
