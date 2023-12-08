{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix/monthly";
  };

  outputs = { self, nixpkgs, utils, naersk, fenix }:
    utils.lib.eachDefaultSystem (system: {
      packages.default = let
        pkgs = nixpkgs.legacyPackages.${system};
        target = "x86_64-unknown-linux-gnu";
        toolchain = with fenix.packages.${system};
          combine [
            minimal.cargo
            minimal.rustc
            targets.${target}.latest.rust-std
          ];

      in (naersk.lib.${system}.override {
        cargo = toolchain;
        rustc = toolchain;
      }).buildPackage {
        doCheck = true;
        src = ./.;
        CARGO_BUILD_TARGET = target;
      };
    });

}
