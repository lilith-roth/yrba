{
  pkgs ? import <nixpkgs> { },
}:
pkgs.rustPlatform.buildRustPackage {
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
    "tar-0.4.45" = "sha256-r2W5clo4LBD4pgXedW1dR28fsX4dRCahOTKiqMdjAF0=";
  };
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
