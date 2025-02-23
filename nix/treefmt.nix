{ pkgs, ... }:
{
  projectRootFile = "flake.nix";
  programs.nixfmt.enable = true;
  programs.mdformat.enable = true;
  programs.stylua.enable = true;
  programs.rustfmt.enable = true;
  programs.toml-sort.enable = true;
  programs.prettier = {
    enable = true;
    settings = {
      useTabs = true;
      plugins = [
        "${
          pkgs.callPackage ./prettier-plugin-jinja-template.nix { }
        }/lib/node_modules/prettier-plugin-jinja-template/lib/index.js"
      ];
      overrides = [
        {
          files = [ "*.html" ];
          options.parser = "jinja-template";
        }
      ];
    };
  };
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
