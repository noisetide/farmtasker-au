{
  description = "A devshell flake for the website";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    devenv,
    systems,
    ...
  } @ inputs: let
    forEachSystem = nixpkgs.lib.genAttrs (import systems);
  in {
    packages = # TODO not yet implemented. Use Dockerfile instead
      forEachSystem
      (system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        default = pkgs.callPackage ./. {};
      });
    devShells =
      forEachSystem
      (system: let
        pkgs = nixpkgs.legacyPackages.${system};
        leptos = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-leptos";
          version = "0.2.21";

          src = pkgs.fetchFromGitHub {
            owner = "leptos-rs";
            repo = pname;
            rev = "v${version}";
            hash = "sha256-Oe65m9io7ihymUjylaWHQM/x7r0y/xXqD313H3oyjN8=";
          };

          cargoHash = "sha256-wZNtEr6IAy+OABpTm93rOhKAP1NEEYUvokjaVdoaSG4=";

          buildFeatures = [ "no_downloads" ]; # cargo-leptos will try to install missing dependencies on its own otherwise
          doCheck = false; # Check phase tries to query crates.io
        };
        buildInputs = with pkgs; [
          # Cli
          bacon
          cargo-binutils
          leptos
          cargo-watch
          cargo-shuttle
          cargo-generate
          dart-sass
          leptosfmt
          nodePackages.svelte-language-server
          leptosfmt
          trunk
          binaryen

          sqlx-cli

          # Lib
          openssl
          libclang
          hidapi
          pkg-config
          alsa-lib
          udev
          clang
          lld
        ];
      in {
        default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              # https://devenv.sh/reference/options/
              dotenv.disableHint = true;

              packages = buildInputs;

              languages.javascript = {
                enable = true;
                corepack.enable = true;
                npm = {
                  enable = true;
                  install.enable = true;
                };
              };

              services.nginx.enable = true;

              languages.typescript.enable = true;

              languages.rust = {
                enable = true;
                channel = "nightly";
                toolchain = {
                  rustc = pkgs.rustc-wasm32;
                };
                targets = ["wasm32-unknown-unknown"];
              };

              env = {
                RUST_BACKTRACE = 1;
                LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}:$LD_LIBRARY_PATH";
                XDG_DATA_DIRS = "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS";
              };
            }
          ];
        };
      });
  };
}
