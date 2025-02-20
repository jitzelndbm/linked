_: {
  projectRootFile = "flake.nix";
  programs.nixfmt.enable = true;
  programs.mdformat.enable = true;
  programs.stylua.enable = true;
  programs.rustfmt.enable = true;
  programs.toml-sort.enable = true;
  programs.sqlfluff = {
    enable = true;
    dialect = "sqlite";
  };
  settings.global = {
    hidden = true;
    excludes = [
      ".editorconfig"
      ".envrc"
      "LICENSE"
    ];
  };
}
