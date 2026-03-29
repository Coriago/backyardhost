{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-flake.url = "github:juspay/rust-flake";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      imports = [
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];
      debug = true;

      perSystem = { self', ... }: {
        devShells.default = self'.devShells.rust;
        packages.default = self'.packages.single-crate;
      };
      # perSystem = {pkgs, ...}: {
      #   devShells.default = pkgs.mkShell {
      #     nativeBuildInputs = with pkgs; [
      #       # Nix
      #       nixd
      #       alejandra

      #       # Just
      #       just
      #       just-lsp

      #       # Rust toolchain
      #       rustc
      #       cargo
      #       rust-analyzer
      #       clippy
      #       rustfmt
      #       cargo-leptos

      #       # NATS
      #       nats-top
      #       natscli
      #     ];

      #     RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          

      #     shellHook = ''
      #       echo "Backyardhost Development Environment"
      #       echo "Run 'just' to see available recipes"
      #     '';
      #   };
      # };
    };
}
