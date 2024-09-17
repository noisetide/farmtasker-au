# TODO not yet implemented!
{
  stdenv,
  pkgs ? import <nixpkgs> {},
}:
stdenv.mkDerivation rec {
  pname = "farmtasker-au";
  version = "0.1";
  src = pkgs.lib.cleanSource ./.;

  buildinputs = with pkgs; [
    cargo-leptos

    openssl
    libclang
    hidapi
    pkg-config
    alsa-lib
    udev
    clang
    lld
  ];

  buildPhase = ''
    echo "Building farmtasker-au..."
    # Insert your build commands here, for example:
    cargo leptos build --release
  '';
}
