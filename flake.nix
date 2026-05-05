{
  description = "Incremental remote backups made simple!";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system;
          inherit overlays;
        };
        supportedSystems = [
          "x86_64-linux"
          "aarch64-darwin"
        ];
        forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
        pkgsFor = nixpkgs.legacyPackages;
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rustup
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
            pkgs.prettierd
            pkgs.marksman
            pkgs.openssl
            pkgs.pkg-config
          ];
        };

        packages = {
          nixpkgs.overlays = [ rust-overlay.overlays.default ];
          environment.systemPackages = [ pkgs.rust-bin.stable.latest.default ];
          default = pkgsFor.${system}.callPackage ./. { };
        };
      }
    );
}
