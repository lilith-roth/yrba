{
    description = "Incremental remote backups made simple!";
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };
    outputs = { self, nixpkgs, rust-overlay }:
    let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
            inherit overlays;
        };
        supportedSystems = [ "x86_64-linux" "aarch64-darwin" ];
        forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
        pkgsFor = nixpkgs.legacyPackages;
    in {
        packages = forAllSystems (system: {
            nixpkgs.overlays = [ rust-overlay.overlays.default ];
            environment.systemPackages = [ pkgs.rust-bin.stable.latest.default ];
            default = pkgsFor.${system}.callPackage ./. { };
        });
    };
}
