# cargo-crate

## Intro

`cargo-crate` is an additional cargo command for your crate.

## Features

In a nutshell `cargo-crate` allows:

-   fetching information about one or several crates, including its owners

-   opening a crate in crates.io

-   opening a crates' documentation directly

-   opening a crates' repository directly

-   opening a crates' homepage directly

## Install

    cargo instal cargo-crate

### Aliases

You will typically use cargo-crate as `cargo crate <command> [args]` which you may find a bit verbose.
To make it easier, `cargo` allows defining aliases in you `$CARGO_HOME/config` file under the `[alias]` section.

You may for instance add:

    [alias]
    ...
    open = "crate open"
    info = "crate info"
    show = "crate info"
    repo = "crate open --repo"
    home = "crate open --home"
    docu = "crate open --doc"

Those will allow you the following calls:

-   `cargo repo clap` opens the clap documentation

-   `cargo home clap` opens the clap homepage

-   etc…​

## Usage

`cargo-crate` can be started using both `cargo crate` or `cargo-crate`.

### Help

    cargo-crate 0.0.1
    Wilfried Kopp <chevdor@gmail.com>
    You can find all available commands below

    USAGE:
        cargo-crate [OPTIONS] <SUBCOMMAND>

    OPTIONS:
        -h, --help       Print help information
        -j, --json       Output as json
        -V, --version    Print version information

    SUBCOMMANDS:
        help    Print this message or the help of the given subcommand(s)
        info    The `info` command returns summarized information
        open    Opens the crate in a browser

### Info command

    cargo-crate-info 0.0.1
    Wilfried Kopp <chevdor@gmail.com>
    The `info` command returns summarized information

    USAGE:
        cargo-crate info [OPTIONS] [CRATE_NAME]...

    ARGS:
        <CRATE_NAME>...    One or more crate names

    OPTIONS:
        -h, --help           Print help information
        -j, --json           Output as json
        -n, --no-versions    Do not show the details about all the older versions
        -V, --version        Print version information

### Open command

    cargo-crate-open 0.0.1
    Wilfried Kopp <chevdor@gmail.com>
    Opens the crate in a browser

    USAGE:
        cargo-crate open [OPTIONS] <CRATE_NAME>

    ARGS:
        <CRATE_NAME>    The name of the crate to open in your browser

    OPTIONS:
            --documentation    We open crates.io by default, use this flag to open the documentation
                               instead
        -h, --help             Print help information
            --homepage         We open crates.io by default, use this flag to open the homepage instead
        -j, --json             Output as json
            --repository       We open crates.io by default, use this flag to open the repo instead
        -V, --version          Print version information
