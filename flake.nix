{
  inputs = {
    nixpkgs.url = "https://flakehub.com/f/DeterminateSystems/nixpkgs-weekly/*";

    fenix.url = "https://flakehub.com/f/nix-community/fenix/*";
    fenix.inputs = {
      nixpkgs.follows = "nixpkgs";
    };

    crane.url = "https://flakehub.com/f/ipetkov/crane/*";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      fenix,
    }:
    let
      inherit (nixpkgs) lib;

      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];

      forEachSystem =
        f:
        lib.genAttrs systems (
          system:
          let
            pkgs = import nixpkgs {
              inherit system;
            };

            toolchain = with fenix.packages.${system}; stable.toolchain;

            craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
          in
          f { inherit pkgs craneLib toolchain; }
        );
    in
    {
      checks = forEachSystem (
        { pkgs, craneLib, ... }:
        let
          src = craneLib.cleanCargoSource ./.;

          deps = craneLib.buildDepsOnly {
            inherit src;
          };
        in
        {
          cargo-build = craneLib.cargoBuild {
            inherit src;
            cargoArtifacts = deps;
          };
        }
      );

      devShells = forEachSystem (
        { pkgs, toolchain, ... }:
        let
          # Helper to run inspect on something (usually a flake in the tests dir)
          inspect = pkgs.writeShellScriptBin "inspect" ''
            set -euo pipefail

            nix eval \
              --json \
              --no-write-lock-file \
              --override-input flake "$1" \
              "https://flakehub.com/f/DeterminateSystems/inspect/*#contents.includingOutputPaths"
          '';

          serve-docs =
            let
              http-server = lib.getExe pkgs.http-server;
            in
            pkgs.writeShellScriptBin "serve-docs" ''
              set -e
              cargo doc
              ${http-server} target/doc
            '';
        in
        {
          default = pkgs.mkShell {
            packages = [
              inspect
              serve-docs
              toolchain

              pkgs.nixfmt
            ];
          };
        }
      );
    };
}
