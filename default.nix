let
  lock = builtins.fromJSON (builtins.readFile ./flake.lock);
  flake-compat = fetchTarball
    {
      url = "https://github.com/edolstra/flake-compat/archive/${lock.nodes.flakeCompat.locked.rev}.tar.gz";
      sha256 = lock.nodes.flakeCompat.locked.narHash;
    };
in
(import (flake-compat) { src = ./.; }).defaultNix
