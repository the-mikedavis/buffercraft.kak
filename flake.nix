{
  description = "A kakoune plugin for crafting and jumping through structured projects.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nixCargoIntegration = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flakeCompat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = inputs@{ self, nixCargoIntegration, ... }:
    nixCargoIntegration.lib.makeOutputs {
      root = ./.;
      buildPlatform = "crate2nix";
      defaultOutputs = { app = "kak-buffercraft"; package = "kak-buffercraft"; };
      overrides = {
        shell = common: prev: {
          # packages = prev.packages ++ (with common.pkgs; [ lld_10 lldb cargo-tarpaulin ]);
          env = prev.env ++ [
            { name = "RUST_BACKTRACE"; value = "1"; }
            { name = "RUSTFLAGS"; value = "-C link-arg=-fuse-ld=lld -C target-cpu=native"; }
          ];
        };
      };
    };
}
