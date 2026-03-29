{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      imports = [
        inputs.treefmt-nix.flakeModule
        ./nix/flake-module.nix
      ];
      debug = true;

      perSystem = {config, self', pkgs, system, ...}: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.rust-overlay.overlays.default
          ];
        };

        treefmt.config = {
          projectRootFile = "flake.nix";
          programs = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
            leptosfmt.enable = true;
          };
        };

        packages.default = self'.packages.leptos-fullstack;

        devShells.default = pkgs.mkShell {
          inputsFrom = [
            config.treefmt.build.devShell
            self'.devShells.leptos-fullstack
          ];
          nativeBuildInputs = with pkgs; [
            # Nix
            nixd
            alejandra

            # Just
            just
            just-lsp

            just
            cargo-watch

            # NATS
            nats-top
            natscli
          ];
        };
      };
    };
}
