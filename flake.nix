{
  description = "go development flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rustAnalyzer = pkgs.stdenv.mkDerivation {
          pname = "rust-analyzer";
          version = "2024-09-30"; # Replace with your desired version tag
          src = pkgs.fetchurl {
            url = "https://github.com/rust-lang/rust-analyzer/releases/download/2024-09-30/rust-analyzer-x86_64-unknown-linux-gnu.gz";
            sha256 = ""; # Replace with the correct hash
          };
          installPhase = ''
            mkdir -p $out/bin
            gunzip -c $src > $out/bin/rust-analyzer
            chmod +x $out/bin/rust-analyzer
          '';
        };
      in
      {
        devShells = {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              rustc
              rustfmt
              rustAnalyzer
              taplo
            ];
          };
        };
      }
    );
}
