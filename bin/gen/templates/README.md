# Intermodal: A 40' shipping container for the Internet

[![Crate](https://img.shields.io/crates/v/imdl.svg?logo=rust)](https://crates.io/crates/imdl)
[![Build](https://github.com/casey/intermodal/workflows/Build/badge.svg)](https://github.com/casey/intermodal/actions)
[![Book](https://img.shields.io/static/v1?logo=read-the-docs&label=book&message=imdl.io&color=informational)](https://imdl.io/book/)
[![Chat](https://img.shields.io/discord/679283456261226516.svg?logo=discord&color=7289da)](https://discord.gg/HaaT5Qz)

Intermodal is a user-friendly and featureful command-line BitTorrent metainfo
utility. The binary is called `imdl` and runs on Linux, Windows, and macOS.

At the moment, creation, viewing, and verification of `.torrent` files is
supported. See [the book](https://imdl.io/book/) for examples and usage
information.

For more about the project and its goals, check out
[this post](https://rodarmor.com/blog/intermodal).

![demonstration animation](https://raw.githubusercontent.com/casey/intermodal/master/www/demo.gif)

## Table of Contents

{{table_of_contents}}

## Installation

### Supported Operating Systems

`imdl` supports Linux, MacOS, and Windows, and should work on other unix OSes.
If it does not, please open an issue!

### Packages

{{packages}}

### Pre-built binaries

Pre-built binaries for Linux, macOS, and Windows can be found on
[the releases page](https://github.com/casey/intermodal/releases).

### Linux and MacOS Install Script

You can use the following command on Linux and MacOS to download the latest
binary, just replace `DEST` with the directory where you'd like to install the
`imdl` binary:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://imdl.io/install.sh | bash -s -- --to DEST
```

A good place to install personal binaries is `~/bin`, which `install.sh` uses
when `--to` is not supplied. To create the `~/bin` directory and install `imdl`
there, do:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://imdl.io/install.sh | bash
```

Additionally, you'll have to add `~/bin` to the `PATH` environment variable,
which the system uses to find executables. How to do this depends on the shell.

For `sh`, `bash`, and `zsh`, it should be done in `~/.profile`:

```sh
echo 'export PATH=$HOME/bin:$PATH' >> ~/.profile
```

For `fish`, it should be done in `~/.config/fish/config.fish`:

```fish
echo 'set -gx PATH ~/bin $PATH' >> ~/.config/fish/config.fish
```

### Cargo

`imdl` is written in [Rust](https://www.rust-lang.org/) and can be built from
source and installed with `cargo install imdl`. To get Rust, use the
[rustup installer](https://rustup.rs/).

### Shell Completion Scripts

Shell completion scripts for Bash, Zsh, Fish, PowerShell, and Elvish are
included in all [binary releases](https://github.com/casey/imdl/releases).

For Bash, move `imdl.bash` to `$XDG_CONFIG_HOME/bash_completion` or
`/etc/bash_completion.d/`.

For Fish, move `imdl.fish` to `$HOME/.config/fish/completions/`.

For the Z shell, move `_imdl` to one of your `$fpath` directories.

For PowerShell, add `. _imdl.ps1` to your PowerShell
[profile](https://technet.microsoft.com/en-us/library/bb613488(v=vs.85).aspx)
(note the leading period). If the `_imdl.ps1` file is not on your `PATH`, do
`. /path/to/_imdl.ps1` instead.

The `imdl` binary can also generate the same completion scripts at runtime,
using the `completions` command:

```sh
$ imdl completions --shell bash > imdl.bash
```

The `--dir` argument can be used to write a completion script into a directory
with a filename that's appropriate for the shell. For example, the following
command will write the Z shell completion script to `$fpath[0]/_imdl`:

```sh
$ imdl completions --shell zsh --dir $fpath[0]
```


## Usage

Online documentation is available in the book, hosted
[here](https://imdl.io/book/).

### Commands

Adding `--help` to any command will print help text about how to use that
command, including detailed information about any command-line arguments it
accepts.

So, to get information about `imdl torrent create`, run `imdl torrent create
--help`.

Additionally, the same help text is available online in
[the book](https://imdl.io/book/).

### Examples

The intro to [the book](https://imdl.io/book/) has a few simple examples. Check
[the FAQ](https://imdl.io/book/faq.html) for more complex usage examples.

### FAQ

The [FAQ](https://imdl.io/book/faq.html) covers a variety of specific
use-cases. If there's a use case you think should be covered, feel free to open
[an issue](https://github.com/casey/intermodal/issues/new).


## Notes for Packagers

First off, thank you very much! If I can do anything to make packaging
Intermodal easier, please don't hesistate to open
[an issue](https://github.com/casey/intermodal/issues/new).

The Intermodal binary is called `imdl`, and the suggested name for the package
is `intermodal`.

Intermodal is written in Rust, and can be built with `cargo build --release`.

Intermodal is distributed under the
[Creative Commons Zero](https://creativecommons.org/share-your-work/public-domain/cc0/),
a public domain dedication with a fallback all-permissive license. The SPDX
identifier of the CC0 is [CC0-1.0](https://spdx.org/licenses/CC0-1.0.html).

### Package Artifacts

There are three primary build artifacts: the binary, the man pages, and the
shell completion scripts.

#### Binary

The binary is called `imdl`, and can be built with:

```
cargo build --release
```

After building, the binary will be present at `target/release/imdl`.

#### Man Pages

Intermodal has a number of subcommands, each of which has a man page. The man
pages are generated from the `--help` text using
[`help2man`](https://www.gnu.org/software/help2man/).

To generate the man pages, ensure `help2man` is available, and run:

```
mkdir -p man
cargo run --package gen man
```

After building, the man pages will be available in `man`.

#### Completion Scripts

Completion scripts are available for a number of shells. To generate them, run:

```
mkdir -p completions
cargo run --release completions --dir completions
```

After running, the completion scripts will be available in `completions`.

### Release Updates

If you'd like to receive an update whenever a new version is released, you can
watch the intermodal repository in "Releases only" mode.

## Chat

The primary chat is on [Discord](https://discord.gg/HaaT5Qz), but I try to also
check [Keybase](https://keybase.io/team/intermodal) and
[IRC](ircs://chat.freenode.net:6697/#intermodal) as time permits.

## Contributing

Your bug reports, feature requests, pull requests, and design help are much
appreciated!

Check out issues with the
["good first issue" label](https://github.com/casey/intermodal/labels/good%20first%20issue)
for some ideas.

Quite a few files are generated by the program in `bin/gen`. Some files are
generated from templates, so those templates should be edited to make changes
to those files:

- `bin/gen/templates/SUMMARY.md` -> `book/src/SUMMARY.md`
- `bin/gen/templates/README.md` -> `README.md`
- `bin/gen/templates/introduction.md` -> `book/src/introduction.md`

Some files are completely generated, and so shouldn't be manually edited at
all:

- `CHANGELOG.md`
- `book/src/commands/*`
- `completions/*`
- `man/*`

All files can be regenerated by running `cargo run --package gen all`, or
`just gen`, if you have [just](https://github.com/casey/just) installed.

The changelog is generated from YAML metadata in commit messages. Here is an
example commit message, with metadata:

```
Upgrade foo

Upgrade foo to v7.5, which is much better.

type: changed
pr:
- https://github.com/casey/intermodal/pull/1
fixes:
- https://github.com/intermodal/issues/2
- https://github.com/intermodal/issues/3
```

The only required field is `type`. To see the possible values for `type`, run
`cargo run --package gen commit-types`.

## Semantic Versioning

Intermodal follows [semantic versioning](https://semver.org/).

In particular:

- v0.0.X: Breaking changes may be introduced at any time.
- v0.X.Y: Breaking changes may only be introduced with a minor version number
  bump.
- vX.Y.Z: Breaking changes may only be introduced with a major version number
  bump

## Unstable Features

To avoid premature stabilization and excessive version churn, unstable features
are unavailable unless the `--unstable` / `-u` flag is passed, for example
`imdl --unstable torrent create .`. Unstable features may be changed or removed
at any time.

## Source Signatures

All commits to the intermodal master branch signed with Casey Rodarmor's PGP
key with fingerprint `3259DAEDB29636B0E2025A70556186B153EC6FE0`, which can be
found
[on keybase](https://keybase.io/rodarmor/pgp_keys.asc?fingerprint=3259daedb29636b0e2025a70556186b153ec6fe0) and on
[his homepage](https://rodarmor.com/static/rodarmor.asc).

## Acknowledgments

The formatting of `imdl torrent show` is entirely copied from
[torf](https://github.com/rndusr/torf-cli), an excellent command-line torrent
creator, editor, and viewer.
