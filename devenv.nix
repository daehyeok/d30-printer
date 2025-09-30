{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # https://devenv.sh/packages/
  packages =
    with pkgs;
    [
      nixfmt-rfc-style
    ]
    ++ lib.optionals stdenv.isLinux [
      dbus
    ];

  # https://devenv.sh/languages/
  languages = {
    rust = {
      enable = true;
      channel = "stable";
      components = [
        "rustc"
        "cargo"
        "clippy"
        "rustfmt"
        "rust-analyzer"
      ];
    };
  };

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy = {
      enable = true;
      settings.allFeatures = true;
    };

    unit-tests = {
      enable = true;
      name = "nix linter -c";
      entry = "nixfmt";
      files = "\\.nix$";
    };
  };
}
