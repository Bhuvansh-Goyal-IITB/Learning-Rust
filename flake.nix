{
  description = "go development flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    old-rust-analyzer-nixpkgs.url = "github:nixos/nixpkgs/34a626458d686f1b58139620a8b2793e9e123bba";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        old-rust-analyzer-pkgs = inputs.old-rust-analyzer-nixpkgs.legacyPackages.${system};
      in
      {
        devShells = {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              rustc
              rustfmt
              old-rust-analyzer-pkgs.rust-analyzer
              taplo
            ];
          };
        };
      }
    );
}
