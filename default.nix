{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "rust-dev-shell";
  buildInputs = [
    pkgs.pkg-config
    pkgs.libasound2-dev
    pkgs.libudev-dev
    pkgs.rustup
    pkgs.pkg-config
    pkgs.alsaLib
    pkgs.libudev
    pkgs.gh
  ];

  shellHook = ''
    echo "Rust env ready."
  '';
}