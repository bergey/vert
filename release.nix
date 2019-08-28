let

  nixpkgs =
    let snapshot = builtins.fromJSON (builtins.readFile ./nixpkgs-snapshot.json);
    inherit (snapshot) owner repo rev;
    in builtins.fetchTarball {
        inherit (snapshot) sha256;
        url = "https://github.com/${owner}/${repo}/archive/${rev}.tar.gz";
        };
  pkgs = import nixpkgs {};

in pkgs.callPackage ./. {}
