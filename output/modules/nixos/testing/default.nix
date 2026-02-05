{...}: {
  system.stateVersion = "25.11";
  fileSystems = {
    "/".device = "/dev/hda1";
  };
  boot.loader.systemd-boot.enable = true;
  boot.loader.grub.enable = false;
}