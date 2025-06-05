{
  description = "Rust environment.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in
  {
    packages.x86_64-linux.default = pkgs.mkShell {
      buildInputs = [
        pkgs.pkg-config
        pkgs.libasound2-dev
        pkgs.libudev-dev
        pkgs.rustup
        pkgs.cargo
        pkgs.rust-analyzer
        pkgs.clippy
        pkgs.rustfmt
      ];

      shellHook = ''
        export RUSTUP_HOME=$HOME/.rustup
        export CARGO_HOME=$HOME/.cargo
        export PATH=$CARGO_HOME/bin:$PATH

        rustup component add clippy rustfmt
      '';
    };
  };
}
