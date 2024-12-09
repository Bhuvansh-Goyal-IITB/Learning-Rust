{
  description = "go development flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          system = system;
          overlays = [
            (self: super: {
              rust-analyzer = super.rust-analyzer.overrideAttrs (old: {
                src = super.fetchFromGitHub {
                  owner = "rust-lang";
                  repo = "rust-analyzer";
                  rev = "822644d97d7f64e1bdff25b1d636e366a29facc4";
                  sha256 = "sha256-iJTCG7vtECll27sxDElL4SuIY/kRhamJf0DDYCW6fb4";
                };
              });
            })
          ];
        };
      in
      {
        devShells = {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              rustc
              rustfmt
              rust-analyzer
              taplo
            ];
          };
        };
      }
    );
}
