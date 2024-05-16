# cargo-open

A cargo subcommand to open an installed crate in your `$EDITOR`,
modelled on [`bundle open`](https://bundler.io/v2.5/man/bundle-open.1.html).

Note that the intended use is to open a crate's source for reading.
Making changes to installed crates is not reccommended, and may produce unexpected results.
Instead, clone the crate locally and [specify the path in your Cargo.toml](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-path-dependencies).

## Installation

This is a binary crate, so it's [installed manually](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html) rather than as a dependency:

```sh
cargo install cargo-open
```

## Usage

```sh
cargo open clap
```

## Configuration

The editor command is picked from the `CARGO_EDITOR`, `VISUAL`, or `EDITOR` environment variables,
checked in that order.

Specify a different manifest file with the `--manifest-path` option.
By default, `Cargo.toml` in the current directory is used.

## Todo/Contributing

There aren't any tests, as this is just glueing together the [cargo-metadata](https://crates.io/crates/cargo_metadata)
crate and some bits from the standard library. As such it should be robust, and what little logic there is should be
adequately constrained by the type system.

Regardless, if you have any problems, suggestions or improvements, feel free to create an issue or PR.

This project uses cargo-readme to export this README from the doc comments.

## Attribution

The original cargo-open was authored by Carol Nichols ([@carols10cents](https://github.com/carols10cents)), and crate ownership was transferred
in may 2024. Many thanks to Carol for all her work in the rust community.

License: MIT
