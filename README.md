# vx
A small convenience wrapper for XBPS.

## Install

Download the latest Linux release from the GitHub Releases page.

### Binary

```sh
curl -L -o vx.tar.gz \
  https://github.com/sirtony/vx/releases/latest/download/vx-x86_64-unknown-linux-musl.tar.gz

tar -xzf vx.tar.gz
chmod +x vx
sudo install -Dm755 vx /usr/local/bin/vx
```

### From source

```
cargo install --git https://github.com/sirtony/vx.git
```

## Usage

```
Usage: vx <COMMAND>

Commands:
  sync     Sync the XBPS repositories
  add      Install packages
  upgrade  Upgrade all packages to their latest versions using existing repo data
  update   Perform a sync and full system update
  remove   Remove package(s)
  clean    Cleans orphaned packages and outdated packages in the cache
  find     Find a package using a query string
  pin      Marks package(s) as manually installed so the clean command doesn't try to remove it
  unpin    Unpin manually pinned package(s)
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
