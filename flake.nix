{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem = {pkgs, ...}: {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            nixd
            just
            alejandra
            nodejs_22
            nodePackages.pnpm
            nodePackages.typescript
            nodePackages.typescript-language-server
            # Rust toolchain
            rustc
            cargo
            rust-analyzer
            clippy
            rustfmt
          ];

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          

          shellHook = ''
            echo "Backyardhost Development Environment"
            echo "Run 'just' to see available recipes"
          '';
        };
      };
    };
}
