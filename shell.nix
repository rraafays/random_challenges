{ pkgs ? import <nixpkgs> { } }:
let
  unstable = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz") { };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
  ];
  buildInputs = with pkgs; [
    unstable.rustc
    unstable.rust-analyzer
    unstable.rustfmt
  ] ++ lib.optional stdenv.isDarwin libiconv;
}
