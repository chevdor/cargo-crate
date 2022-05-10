# cargo-crate

## Intro

An additional cargo command for your crate.

## Install

    cargo instal cargo-crate

## Usage

### Help

    cargo-crate 0.0.1

    You can find all available commands below

    USAGE:
        crate [OPTIONS] <SUBCOMMAND>

    OPTIONS:
        -h, --help       Print help information
        -j, --json       Output as json
        -V, --version    Print version information

    SUBCOMMANDS:
        help    Print this message or the help of the given subcommand(s)
        info    The `info` command returns summarized information
        open    Opens the crate in a browser

### Info command

    crate-info 0.0.1

    The `info` command returns summarized information

    USAGE:
        crate info [OPTIONS] [CRATE_NAME]...

    ARGS:
        <CRATE_NAME>...    One or more crate names

    OPTIONS:
        -h, --help       Print help information
        -j, --json       Output as json
        -V, --version    Print version information

### Open command

    crate-open 0.0.1

    Opens the crate in a browser

    USAGE:
        crate open [OPTIONS] <CRATE_NAME>

    ARGS:
        <CRATE_NAME>    The name of the crate to open in your browser

    OPTIONS:
        -h, --help       Print help information
        -j, --json       Output as json
        -V, --version    Print version information
