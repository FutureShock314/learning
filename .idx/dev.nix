# To learn more about how to use Nix to configure your environment
# see: https://developers.google.com/idx/guides/customize-idx-env
{ pkgs, ... }: {
  # Which nixpkgs channel to use.
  channel = "stable-24.05"; # or "unstable"
  # Use https://search.nixos.org/packages to find packages
  packages = with pkgs; [
    ### ENV
    direnv
    gh

    ### PYTHON
    python312
    python312Packages.pip
    (python312.withPackages (python-pkgs: with python-pkgs; [
      # dreamberd
      rich
    ]))
    
    ### RUST
    # rustc
    rustup
    cargo
    gcc
    ## Webscraper
    openssl
    pkg-config
    ## Tauri
    bun
  ];
  # Sets environment variables in the workspace
  env = {
    PATH = [ "/home/user/learningnix/cli/rust/" "/home/user/learningnix/cli/" ];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };
  idx = {
    # Search for the extensions you want on https://open-vsx.org/ and use "publisher.id"
    extensions = [
      ### ENV
      "vscodevim.vim"
      "mkhl.direnv"

      ### VSC
      "catppuccin.catppuccin-vsc"
      "catppuccin.catppuccin-vsc-icons"
      "catppuccin.catppuccin-vsc-pack"

      ### NIX
      "arrterian.nix-env-selector"
      "bbenoist.Nix"
      "jeff-hykin.better-nix-syntax"
      "jnoortheen.nix-ide"
      "pinage404.nix-extension-pack"

      ### PYTHON
      "ms-python.debugpy"
      "ms-python.python"

      ### RUST
      "rust-lang.rust-analyzer"
    ];
    # Enable previews
    previews = {
      enable = true;
      previews = {
        # web = {
        #   # Example: run "npm run dev" with PORT set to IDX's defined port for previews,
        #   # and show it in IDX's web preview panel
        #   command = ["npm" "run" "dev"];
        #   manager = "web";
        #   env = {
        #     # Environment variables to set for your server
        #     PORT = "$PORT";
        #   };
        # };
      };
    };
    # Workspace lifecycle hooks
    workspace = {
      # Runs when a workspace is first created
      onCreate = {
        # Example: install JS dependencies from NPM
        # npm-install = "npm install";
        # Open editors for the following files by default, if they exist:
        default.openFiles = [ ".idx/dev.nix" "README.md" ];
      };
      # Runs when the workspace is (re)started
      onStart = {
        # Example: start a background task to watch and re-build backend code
        # watch-backend = "npm run watch-backend";
      };
    };
  };
}
