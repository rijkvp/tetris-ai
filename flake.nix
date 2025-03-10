{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs =
    {
      self,
      nixpkgs,
      devenv,
      ...
    }@inputs:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [
          (
            { pkgs, config, ... }:
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

              # TODO: fix
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
            }
          )
        ];
      };
    };
}
