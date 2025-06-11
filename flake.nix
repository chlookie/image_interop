{
  inputs = {
    # Import the stable version of nixpkgs
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    # Import flake-utils for utility functions
    flake-utils.url = "github:numtide/flake-utils";

    # Used to format all kinds of files
    treefmt-nix.url = "github:numtide/treefmt-nix";

    # Used to build the project by parsing the cargo dependencies
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Used to generate/get a specific rust toolchain to use with naersk
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      treefmt-nix,
      naersk,
      fenix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        # Import nixpkgs for the current system
        pkgs = nixpkgs.legacyPackages.${system};

        # Define the Rust toolchain by reading rust-toolchain.toml
        toolchain = fenix.packages.${system}.fromToolchainFile {
          dir = ./.;
          sha256 = "sha256-KUm16pHj+cRedf8vxs/Hd2YWxpOrWZ7UOrwhILdSJBU=";
        };

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

        # Setup treefmt with the formatters we need
        treefmtEval = treefmt-nix.lib.evalModule pkgs (
          { ... }:
          {
            projectRootFile = "flake.nix";
            programs = {
              nixfmt.enable = true; # For nix files
              rustfmt.enable = true; # For rust files
            };
          }
        );
      in
      {
        # For `nix build` and `nix run`
        # Define the package using naersk with the specified Rust toolchain
        packages.default = naersk'.buildPackage {
          src = ./.;
        };

        # For `nix develop`
        # Define the development shell with necessary tools
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [ toolchain ];
          buildInputs = with pkgs; [
            rustfmt
            pre-commit
            rust-analyzer
            rustPackages.clippy
            cargo-limit
          ];

          # Needed for rust-analyser to work
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };

        # For `nix fmt`
        formatter = treefmtEval.config.build.wrapper;

        # For `nix flake check`
        checks.formatting = treefmtEval.config.build.check self;
      }
    );
}
