{
  description = "Substack CLI — create, draft, and publish posts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustToolchain = fenix.packages.${system}.latest.toolchain;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = craneLib.cleanCargoSource ./.;
        commonArgs = { inherit src; pname = "substack-cli"; };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        substack-cli-unwrapped = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
        substack-cli = pkgs.writeShellScriptBin "substack" ''
          set -euo pipefail

          export SUBSTACK_API_KEY="''${SUBSTACK_API_KEY:-$(${pkgs.gopass}/bin/gopass show -o substack.com/api-key)}"

          if [ -z "''${SUBSTACK_HOSTNAME:-}" ]; then
            pub_url="$(${pkgs.gopass}/bin/gopass show -o substack.com/api-key publication-url)"
            pub_url="''${pub_url#https://}"
            pub_url="''${pub_url#http://}"
            export SUBSTACK_HOSTNAME="''${pub_url%/}"
          fi

          exec ${substack-cli-unwrapped}/bin/substack "$@"
        '';
      in
      {
        packages = {
          default = substack-cli;
          unwrapped = substack-cli-unwrapped;
        };

        apps.default = {
          type = "app";
          program = "${substack-cli}/bin/substack";
        };

        devShells.default = craneLib.devShell {
          packages = [ pkgs.rust-analyzer pkgs.jujutsu pkgs.gopass ];
        };
      }
    );
}
