with import <nixpkgs> { };

mkShell {
  name = "yrba-env";
  packages = [
    rustup
    rustc
    cargo
    gcc
    just
  ];
  nativeBuildInputs = with pkgs; [
      pkg-config
  ];
  buildInputs = with pkgs; [
      openssl
  ];
  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
  shellHook = ''
      export NIX_ENFORCE_PURITY=0
    '';
}
