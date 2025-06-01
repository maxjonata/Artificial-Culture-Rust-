{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "rust-dev-shell";
  buildInputs = [
    pkgs.rustup
    pkgs.pkg-config
    pkgs.alsaLib
    pkgs.libudev
    pkgs.gh
  ];

  shellHook = ''
    echo "Ambiente Rust pronto!"
  '';
}