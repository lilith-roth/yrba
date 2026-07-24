{
  pkgs ? import <nixpkgs> { },
  lib ? pkgs.lib,
}:
pkgs.rustPlatform.buildRustPackage {
  pname = "yrba";
  version = "main";
  meta = {
    description = "Incremental remote backups made simple!";
    longDescription = ''
            YRBA is a tool to automatically perform periodic incremental backups
      	  of all your important data, and automatically copy them to a specified
      	  location or upload them to a remote server.
      	'';
    homepage = "https://github.com/lilith-roth/yrba";
    licenses = lib.licenses.gpl3Only;
    platforms = lib.platforms.all;
    downloadPage = "https://github.com/lilith-roth/yrba/releases";
    changelog = "https://github.com/lilith-roth/yrba/blob/HEAD/CHANGELOG.md";
    mainProgram = "yrba";
  };
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
    "tar-0.4.46" = "sha256-t46L7e7KU8a5X+91IPTuFdXn6Bz9hVP+hUC0rgrd7Rc=";
  };
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
