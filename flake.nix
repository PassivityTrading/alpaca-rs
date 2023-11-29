{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    devenv.url = "github:cachix/devenv";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fxpl.url = "github:Implodent/flakexplode";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  nixConfig = {
    extra-trusted-public-keys =
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
        inputs.treefmt-nix.flakeModule
        inputs.fxpl.flakeModules.fenix
      ];
      systems = inputs.nixpkgs.lib.systems.flakeExposed;

      perSystem = { config, self', inputs', pkgs, system, ... }: rec {
        _module.args.pkgs = (import inputs.nixpkgs) {
          inherit system;
          config.allowUnfree = true;
        };

        treefmt = {
          package = pkgs.treefmt;
          # programs.leptosfmt.enable = true;
          # programs.prettier.enable = true;
          programs.nixfmt.enable = true;
          programs.rustfmt.enable = true;
          projectRootFile = ./flake.nix;
        };

        fenix = {
          enable = false;
          profile = "default";
        };

        devenv.shells.default = rec {
          packages = with pkgs; [
            openssl
            pkg-config
            clang
            cmake
            gcc
            # config.fenix.toolchain.toolchain
          ];
          env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

          scripts = {
            fmt.exec = "${config.treefmt.build.wrapper}/bin/treefmt .";
          };
        };
      };
    };
}
