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
                pkgs.python3Packages.numpy
              ];

              languages.rust = {
                enable = true;
                channel = "stable";
                targets = [ "wasm32-unknown-unknown" ];
              };

              languages.python = {
                enable = true;
              };

              pre-commit.hooks = {
                rustfmt.enable = true;
                clippy.enable = true;
              };
            }
          )
        ];
      };
    };
}
