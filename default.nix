{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage rec {
    pname = "yrba";
    version = "main";
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;
    cargoTestFlags = [
      "--bins" # Don't run integration tests, as they require other services to run which are configured in docker
    ];

    nativeBuildInputs = with pkgs; [
        pkg-config
    ];
    buildInputs = with pkgs; [
        openssl
    ];
    cargoLock.outputHashes = {
        "tar-0.4.44" = "sha256-0sCBUzZqaV7OD6kEkNN4wylILwY2n7ltahN2xC7iJmU=";
    };
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
