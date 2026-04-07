{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";
    rust-flake.url = "github:juspay/rust-flake";
    rust-flake.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      debug = true;
      imports = [
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];

      perSystem = {
        self',
        pkgs,
        lib,
        config,
        ...
      }: {
        # Rust Build
        rust-project.src = lib.cleanSourceWith {
          src = inputs.self; # The original, unfiltered source
          filter = path: type:
            (lib.hasSuffix "tailwind.css" path)
            || (lib.hasInfix "/assets/" path)
            || (config.rust-project.crane-lib.filterCargoSources path type);
        };
        rust-project.crates."backyardhost" = {
          crane.args = {
            buildInputs =
              lib.optionals pkgs.stdenv.isLinux
              (with pkgs; [
                webkitgtk_4_1
                xdotool
                pkg-config
                openssl
              ]);

            nativeBuildInputs = with pkgs; [
              pkg-config
              makeWrapper
              tailwindcss
              dioxus-cli
            ];
          };
        };
        packages.default = self'.packages.backyardhost;

        # Dev Tools
        devShells.default = pkgs.mkShell {
          name = "backyardhost-shell";
          inputsFrom = [
            self'.devShells.rust
          ];
          packages = with pkgs; [
            nixd
            alejandra
            nats-top
            natscli
            just
          ];
        };

        # Temporary until 0.7.5 is in unstable
        nixpkgs.overlays = [
          (final: prev: {
            dioxus-cli = prev.rustPlatform.buildRustPackage (
              let
                old = prev.dioxus-cli;
              in {
                pname = "dioxus-cli";
                version = "0.7.5";
                src = prev.fetchCrate {
                  pname = "dioxus-cli";
                  version = "0.7.5";
                  hash = "sha256-iAwR43SwmOBvuHa9qZBJLCjyhQSj/XgDx0jkWR+lgrE=";
                };
                cargoHash = "sha256-JS5/7hQhgN2gbMmLY2zD2GE/Ony8AAHAzj7Ituj6l90=";
                buildFeatures = [
                  "no-downloads"
                  "disable-telemetry"
                ];
                env = {
                  OPENSSL_NO_VENDOR = 1;
                };
                nativeBuildInputs = with prev; [
                  pkg-config
                  cacert
                  makeWrapper
                ];
                buildInputs = with prev; [
                  openssl
                ];
                nativeCheckInputs = with prev; [
                  rustfmt
                ];
                checkFlags = [
                  "--skip=serve::proxy::test"
                  "--skip=test_harnesses::run_harness"
                ];
                postInstall = ''
                  wrapProgram $out/bin/dx \
                    --suffix PATH : ${
                    prev.lib.makeBinPath [
                      prev.esbuild
                      prev.wasm-bindgen-cli_0_2_114
                    ]
                  }
                '';
                inherit (old) meta;
              }
            );
          })
        ];
      };
    };
}
