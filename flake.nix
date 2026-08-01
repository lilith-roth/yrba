{
  description = "Incremental remote backups made simple!";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };
  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      treefmt-nix,
    }:
    let
      eachDefaultEnvironment =
        f:
        flake-utils.lib.eachDefaultSystem (
          system:
          f {
            inherit system;
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                self.overlays.default
                (import rust-overlay)
              ];
            };
          }
        );
      pkgsFor = nixpkgs.legacyPackages;
    in
    eachDefaultEnvironment (
      { system, pkgs }: {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.gcc
            pkgs.just
            pkgs.rust-analyzer
            pkgs.systemd-language-server
            pkgs.docker-language-server
            pkgs.bash-language-server
            pkgs.nil
            pkgs.efm-langserver
            pkgs.rustfmt
            pkgs.prettierd
            pkgs.marksman
            pkgs.openssl
            pkgs.pkg-config
            pkgs.vscode-extensions.vadimcn.vscode-lldb.adapter
            pkgs.mdbook
            pkgs.statix
          ];
        };

        nixosModules.default = import ./nixos-module.nix;

        packages.default = pkgsFor.${pkgs.system}.callPackage ./. { };

        formatter = (treefmt-nix.lib.evalModule pkgs ./treefmt.nix).config.build.wrapper;

        checks.formatting = (treefmt-nix.lib.evalModule pkgs ./treefmt.nix).config.build.check self;
      }
    )
    // {
      overlays.default = import ./overlay.nix;
    };
}
