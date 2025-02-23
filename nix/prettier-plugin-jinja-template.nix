{
  lib,
  buildNpmPackage,
  fetchFromGitHub,
}:

buildNpmPackage rec {
  pname = "prettier-plugin-jinja-template";
  version = "0.0.15-unstable-2023-07-26";

  src = fetchFromGitHub {
    owner = "davidodenwald";
    repo = pname;
    rev = "eca651381349336de7ec6a879d99468522ee8240";
    hash = "sha256-fy+MqtgPNPH2djXgqU5ZzvDgwwZEayv1qO1pNKJxBNM=";
  };

  npmDepsHash = "sha256-dlQkvji36Za86lAt5ds8nphDnu2uA28tNZqZKzt2o5A=";

  dontNpmPrune = true;

  # Fixes error: Cannot find module 'prettier'
  postInstall = ''
    pushd "$nodeModulesPath"
    find -mindepth 1 -maxdepth 1 -type d -print0 | grep --null-data -Exv "\./(ulid|prettier)" | xargs -0 rm -rfv
    popd
  '';
}
