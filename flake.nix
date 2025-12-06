{
  description = ''
    Examples of NixOS systems' configuration for Raspberry Pi boards
    using nixos-raspberrypi
  '';

  nixConfig = {
    bash-prompt = "\[nixos-raspberrypi-demo\] ➜ ";
    extra-substituters = [
      "https://nixos-raspberrypi.cachix.org"
    ];
    extra-trusted-public-keys = [
      "nixos-raspberrypi.cachix.org-1:4iMO9LXa8BqhU+Rpg6LQKiGa2lsNh/j2oiYLNOQ5sPI="
    ];
    connect-timeout = 5;
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    nixos-raspberrypi = {
      url = "github:nvmd/nixos-raspberrypi/main";
    };

    disko = {
      url = "github:nix-community/disko";
      inputs.nixpkgs.follows = "nixos-raspberrypi/nixpkgs";
    };

    nixos-anywhere = {
      url = "github:nix-community/nixos-anywhere";
    };
  };

  outputs = {nixos-raspberrypi, ...}: {
    installerImages = {
      rpi5 = nixos-raspberrypi.installerImages.rpi5.extendModules {
        modules = [
          {
            users.users.nixos.openssh.authorizedKeys.keys = [
              # YOUR SSH PUB KEY HERE #
            ];
            users.users.root.openssh.authorizedKeys.keys = [
              # YOUR SSH PUB KEY HERE #
            ];
          }
        ];
      };
    };

    # nixosConfigurations = {
    #   rpi5 = nixos-raspberrypi.lib.nixosSystemFull {
    #     specialArgs = inputs;
    #     modules = [
    #       {
    #         imports = with nixos-raspberrypi.nixosModules; [
    #           # Hardware configuration
    #           raspberry-pi-5.base
    #           raspberry-pi-5.page-size-16k
    #           raspberry-pi-5.display-vc4

    #         ];
    #       }
    #       # Disk configuration
    #       disko.nixosModules.disko
    #       # WARNING: formatting disk with disko is DESTRUCTIVE, check if
    #       # `disko.devices.disk.nvme0.device` is set correctly!
    #       ./disko-nvme-zfs.nix
    #       {networking.hostId = "8821e309";} # NOTE: for zfs, must be unique
    #       # Further user configuration
    #       common-user-config
    #       {
    #         boot.tmp.useTmpfs = true;
    #       }
    #     ];
    #   };
    # };
  };
}
