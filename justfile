VERSION := `toml get cli/Cargo.toml package.version | jq -r`
export TAG:=`toml get cli/Cargo.toml package.version | jq -r .`
default_bump := 'patch'

# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

# Test / watch
test:
	cargo watch -x "test -- --no-capture"

# Test including ignored tests
test_all:
	cargo test -- --include-ignored

# Generate usage samples
_usage:
	cargo run -q --bin cargo-crate -- --help > doc/help.adoc
	cargo run -q --bin cargo-crate -- info --help > doc/usage_info.adoc
	cargo run -q --bin cargo-crate -- open --help > doc/usage_open.adoc
	cargo run -q --bin cargo-crate -- search --help > doc/usage_search.adoc

# Generate documentation
doc: _usage
	cargo doc -p cargo-crate -p lib-cargo-crate --all-features --no-deps

# Run rustfmt
_fmt:
	cargo +nightly fmt --all

# Run clippy
_clippy:
	cargo +nightly clippy --all-features --all-targets

deny:
	cargo deny check

# Run checks such as clippy, rustfmt, etc...
check: _clippy _fmt

# Minor bump, can be used once the release is ready
bump bump=default_bump:
	cargo workspaces version {{bump}} --no-git-commit

clean:
	rm -f cli/*.wasm
	rm -f *.wasm

changelog:
	#!/usr/bin/env bash
	latest=$(git rev-list -n 1 latest)
	cog changelog -f $latest

# Generate the readme as .md
md:
	#!/usr/bin/env bash
	asciidoctor -b docbook -a leveloffset=+1 -o - README_src.adoc | pandoc   --markdown-headings=atx --wrap=preserve -t markdown_strict -f docbook - > README.md

publish:
	#!/bin/sh
	echo Releasing v$TAG
	git checkout v$TAG
	cargo workspaces publish --skip-published --amend --exact --all

tag:
    #!/bin/sh
    echo Tagging version v$TAG
    git tag "v$TAG" -f
    git tag | sort -Vr | head

tag_push:
    #!/bin/sh
    echo Pushing v$TAG
    git push origin v$TAG

release : check test_all bump doc md tag tag_push publish
