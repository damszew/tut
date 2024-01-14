{
  description = "A dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override
          {
            extensions = [ "rust-analyzer" "rust-src" ];
          };

        # Only compile time deps
        nativeBuildInputs = with pkgs; [
          rustToolchain
          commitizen
        ];

        # Compile & runtime deps
        buildInputs = with pkgs; [
        ];

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          pname = "tut-workspace";

          inherit src buildInputs nativeBuildInputs;
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          pname = "tut-web";
        });

        fmt = craneLib.cargoFmt commonArgs;

        clippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        });

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        bin = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

      in
      with pkgs;
      {
        # `nix flake check`
        checks = {
          inherit fmt clippy bin;
        };

        # `nix develop`
        devShells.default = mkShell {
          inherit buildInputs nativeBuildInputs;
        };
      }
    );
}
