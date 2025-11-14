{
    fetchFromGitHub,
}:

let
    # https://github.com/oxalica/rust-overlay/issues/209
    rust-overlay-src = fetchFromGitHub {
        owner = "oxalica";
        repo = "rust-overlay";
        rev = "971d18658c83f3a6a434ac647798141fddce3175";
        hash = "sha256-LTgfljA/X8aGEXk/EUVoLL+0wfJjQAhF/rUQOFsx+/U=";
    };
    # re-evaluate pkgs
    pkgsWithOverlay = import <nixpkgs> {
        overlays = [ (import rust-overlay-src) ];
    };
in

with pkgsWithOverlay;

let
    rustSpecific = rust-bin.stable.latest.default;
    rustPlatform = makeRustPlatform {
        cargo = rustSpecific;
        rustc = rustSpecific;
    };
in
rustPlatform.buildRustPackage {
    pname = "yrba";
    version = "main";
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    nativeBuildInputs = with pkgs; [
        pkg-config
    ];
    buildInputs = with pkgs; [
        openssl
    ];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
