# buffercraft.kak

Buffercraft is a plugin for the [kakoune](https://github.com/mawww/kakoune)
text editor which tries to emulate some behavior of
[`tpope/vim-projectionist`](https://github.com/tpope/vim-projectionist).

It's a work-in-progress. Check back later to see docs about usage, installation,
and configuration.

## Development

This repo uses [nix](https://nixos.org/) for development. You may also
ignore nix and use `cargo` if you have it installed.

Spawn a development shell with all necessary dependencies with `nix-shell`
in this directory. Use `check` to build the `kak-buffercraft` binary. Follow
the output of `menu` for other options.
