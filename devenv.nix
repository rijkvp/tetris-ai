{
  pkgs,
  ...
}:
{
  packages = with pkgs; [
    cargo-edit
    python3
    python3Packages.numpy
    wasm-pack
    nodejs
    pnpm
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [ "wasm32-unknown-unknown" ];
  };

  git-hooks.hooks = {
    typescript = {
      enable = true;
      name = "TypeScript typecheck";
      entry = "pnpm run check";
      types = [
        "svelte"
        "ts"
      ];
      pass_filenames = false;
    };
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
}
