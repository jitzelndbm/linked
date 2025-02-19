{ pkgs, ... }:
{
  packages =
    let
      inherit (pkgs)
        rustPlatform
        rust-analyzer
        clippy
        rustfmt
        pkg-config
        bacon
        cargo
        rustc
        ;
      inherit (rustPlatform) rustLibSrc;
    in
    [
      rust-analyzer
      clippy
      rustfmt
      pkg-config
      bacon
      cargo
      rustc
      rustLibSrc
    ];
}
