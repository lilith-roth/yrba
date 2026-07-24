# NixOS Setup

Using the NixOS module you can declaritively configure your YRBA setup together
with your NixOS configuration, and keep it automatically updated when you run
`nixos-rebuild switch`.

## Classic NixOS configuration

With a few simple lines in your `configuration.nix` you can have YRBA set up on
your system.

```nix
{
  config,
  pkgs,
  lib,
  ...
}:
let
  yrba = builtins.fetchGit {
    url = "https://github.com/lilith-roth/yrba";
    ref = "main";
  }
in
{
  # nixpkgs overlay to make the YRBA package available to your system
  nixpkgs.overlays = [
    (import ("${yrba}/overlay.nix"))
  ];

  imports = [
    ./hardware-configuration.nix
    # Imports the NixOS module
    "${yrba}/nixos-module.nix"
  ];

  # Configures the YRBA backup service and enables it
  services.yrba = {
    enable = true;
    schedule = {
      enable = true;
      dates = "weekly";
    };
    extraConfig = {
      remote = "file:///directory/where/backups/are/stored";
      amount_of_backups_to_keep = 3;
      folders_to_backup = [
        "/super/important/directory/that/needs/to/be/backed/up"
      ];
    };
  };
};
```

## NixOS Flake Configuration

If you use flakes to configure your NixOS system, define YRBA as an import in
your `flake.nix`, and pass the overlay to nixpkgs.

`flake.nix`

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    yrba = {
      url = "github:lilith-roth/yrba";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    yrba,
    ...
  }:
  let
    pkgs = import nixpkgs {
      system = "x86_64-linux";
      overlays = [
        yrba.overlays.default
      ];
    };
    pkgsFor = nixpkgs.legacyPackages;
  in
  {
    nixosConfigurations = {
      # Desktop configuration
      my-super-system = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        specialArgs = {
          inherit yrba pkgs;
        };
        modules = [
          ./hosts/lilypads-nixos-main
        ];
      };
    };
  };
}
```

`configuration.nix`

```nix
{
  pkgs,
  yrba,
  ...
}: {
  imports = [
    "{yrba}/nixos-module.nix"
  ];

  # Configures the YRBA backup service and enables it
  services.yrba = {
    enable = true;
    schedule = {
      enable = true;
      dates = "weekly";
    };
    extraConfig = {
      remote = "file:///directory/where/backups/are/stored";
      amount_of_backups_to_keep = 3;
      folders_to_backup = [
        "/super/important/directory/that/needs/to/be/backed/up"
      ];
    };
  };
}
```

## Explanation

### `enable`

bool: If enables will install the YRBA package on your system.

### `schedule`

systemd configuration: Automatically configures a timer on your system to
periodically run YRBA.

### `extraConfig`

The options for `extraConfig` relate 1 to 1 to the settings in the yrba.toml
file.

For the configuration details check out:
[Configuration](../../configuration/configuration.md).
