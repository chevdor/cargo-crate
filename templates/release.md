## Downloads

Download the binary for your OS from below:
- **Linux**: [Debian package]({{ DEBIAN_URL }})
- **MacOS**: [.tgz Archive]({{ MACOS_TGZ_URL }})
## Install

### From source

```
cargo install --locked --git https://github.com/chevdor/cargo-crate
```

### Linux
```
wget {{ DEBIAN_URL }} -O cargo-crate.deb
sudo dpkg -i cargo-crate.deb
cargo-crate --help
```

### MacOS

```
brew tap chevdor/cargo-crate https://github.com/chevdor/cargo-crate
brew update
brew install chevdor/cargo-crate/cargo-crate
```

{{ CHANGELOG }}
