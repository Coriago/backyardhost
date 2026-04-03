{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
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
        # nixpkgs.overlays = [
        #   # Configure tailwind to enable all relevant plugins
        #   (self: super: {
        #     tailwindcss =
        #       super.tailwindcss.overrideAttrs
        #       (oa: {
        #         plugins = [
        #           pkgs.nodePackages."@tailwindcss/aspect-ratio"
        #           pkgs.nodePackages."@tailwindcss/forms"
        #           pkgs.nodePackages."@tailwindcss/language-server"
        #           pkgs.nodePackages."@tailwindcss/line-clamp"
        #           pkgs.nodePackages."@tailwindcss/typography"
        #         ];
        #       });
        #   })
        # ];

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

        # Dev Tools
        devShells.default = pkgs.mkShell {
          name = "backyardhost-shell";
          inputsFrom = [
            self'.devShells.rust
          ];
          packages = with pkgs; [
            # Nix
            nixd
            alejandra
            # Nats
            nats-top
            natscli
            # Just
            just
          ];
        };
        packages.default = self'.packages.backyardhost;
      };
    };
}
