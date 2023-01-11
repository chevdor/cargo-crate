# cargo-crate

## Intro

`cargo-crate` is an additional cargo command for your crate. It basiscally does the same (I’d argue better) than the following:

-   `cargo-show`

-   `cargo-info`

-   `cargo-open`

-   `cargo-search`

and does not require installing several crates/commands.

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

    You can find all available commands below

    Usage: cargo-crate [OPTIONS] <COMMAND>

    Commands:
      info    The `info` command returns summarized information
      open    Opens the crate in a browser
      search  The `search` command returns a list of crates matching your search pattern
      help    Print this message or the help of the given subcommand(s)

    Options:
      -j, --json     Output as json
      -h, --help     Print help information
      -V, --version  Print version information

### Info command

    The `info` command returns summarized information

    Usage: cargo-crate info [OPTIONS] [CRATE_NAME]...

    Arguments:
      [CRATE_NAME]...  One or more crate names

    Options:
      -j, --json                         Output as json
      -m, --max-versions <MAX_VERSIONS>  Limit the number of versions that are displayed. You can push the limit using this flag [default: 10]
      -h, --help                         Print help information
      -V, --version                      Print version information

### Open command

    Opens the crate in a browser

    Usage: cargo-crate open [OPTIONS] <CRATE_NAME>

    Arguments:
      <CRATE_NAME>  The name of the crate to open in your browser

    Options:
      -j, --json           Output as json
          --repository     We open crates.io by default, use this flag to open the repo instead
          --homepage       We open crates.io by default, use this flag to open the homepage instead
          --documentation  We open crates.io by default, use this flag to open the documentation instead
      -h, --help           Print help information
      -V, --version        Print version information

### Search command

    The `search` command returns a list of crates matching your search pattern

    Usage: cargo-crate search [OPTIONS] <PATTERN>

    Arguments:
      <PATTERN>  You search pattern

    Options:
      -j, --json           Output as json
      -l, --limit <LIMIT>  Number of expected results: 0..100 [default: 12]
      -r, --raw            Show only the list of crates, without extra information
      -h, --help           Print help information
      -V, --version        Print version information

## Advanced Usage

`cargo-crate` can be used with `fzf` for powerful interactive searches.

    function crate_search() {
        PATTERN=${@};
        if [ $PATTERN ]; then
            cargo-crate search --raw -l 100 $PATTERN | fzf -m -i --preview 'cargo-crate info {}' --query "$PATTERN"
        else
            echo You must provide a starting search pattern
            exit 1
        fi
    }

allows doing cool things like [this](https://asciinema.org/a/493910).
