{ pkgs, ... }:
{
  DATABASE_URL = "sqlite://linked.db";

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
        usql
        sqlx-cli
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
      usql
      sqlx-cli
    ];
}
