{
  pkgs,
  lib,
  ...
}:
{
  packages = [
    pkgs.cargo-edit
    pkgs.python3
    pkgs.python3Packages.numpy
    pkgs.cargo-flamegraph
    pkgs.wasm-pack
    pkgs.nodejs
    pkgs.yarn
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [ "wasm32-unknown-unknown" ];
  };

  git-hooks.hooks = {
    rustfmt = {
      enable = true;
      settings.manifest-path = "./tetris-ai/Cargo.toml";
    };
    clippy = {
      enable = true;
      settings = {
        denyWarnings = true;
        extraArgs = "--manifest-path ./tetris-ai/Cargo.toml";
      };
    };
  };

  env.LD_LIBRARY_PATH =
    with pkgs;
    lib.makeLibraryPath [
      python3
      python3Packages.numpy
    ];

  scripts.deploy.exec = ''
    #!/bin/sh
    if [ -z "$1" ]; then
      echo "usage: $0 <deploy_dir>"
      exit 1
    fi
    npm run build
    cd "$1"
    find . -mindepth 1 -maxdepth 1 -not -name '.git' -exec rm -rf {} +
    cp -a "$DEVENV_ROOT/build/." .
    touch .nojekyll
  '';

  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';
}
