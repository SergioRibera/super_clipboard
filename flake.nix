{
  description = "Standar cross compile flake for Rust Lang Projects";
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix/monthly";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
  };
  outputs = inputs @ {
    flake-parts,
    fenix,
    nixpkgs,
    flake-utils,
    crane,
    self,
    ...
  }:
    inputs.flake-parts.lib.mkFlake
    {
      inherit inputs;
    }
    {
      systems = ["x86_64-linux"];
      perSystem = {
        config,
        pkgs,
        system,
        ...
      }: let
        inherit (pkgs) lib;
        # Toolchain
        toolchain = with fenix.packages.${system};
          fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-U2yfueFohJHjif7anmJB5vZbpP7G6bICH4ZsjtufRoU=";
          };
        craneLib = crane.lib.${system}.overrideToolchain toolchain;

        src = ./.;
        buildInputs = with pkgs; [
          pkg-config
          freetype
          freetype.dev

          libxkbcommon
          vulkan-loader
          wayland
          wayland-protocols
          xorg.libX11
        ];
        commonArgs = {
          inherit src;
          inherit buildInputs;
        };
        # Compile all artifacts for x86_64-unknown-linux-gnu
        linuxArtifacts = craneLib.buildDepsOnly (commonArgs
          // {
            CARGO_BUILD_TARGET = "x86_64-unknown-linux-gnu";
            doCheck = false;
          });

        # Compile app for x86_64-unknown-linux-gnu
        linuxApp = craneLib.buildPackage (
          commonArgs
          // {
            doCheck = false;
            cargoArtifacts = linuxArtifacts;
            nativeBuildInputs = with pkgs; [
              pkg-config
              gtk-layer-shell
              gtk3
            ];
          }
        );
      in {
        # nix build
        packages = {
          default = linuxApp;
        };

        # nix run
        apps = {
          default = flake-utils.lib.mkApp {
            drv = linuxApp;
          };
        };

        # nix develop
        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            toolchain
            fontconfig
            noto-fonts-color-emoji
          ] ++ commonArgs.buildInputs;
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      };
    };
}
