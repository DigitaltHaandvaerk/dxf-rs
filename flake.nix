{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    systems.url = "github:nix-systems/default";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { flake-parts, ... } @ inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devshell.flakeModule inputs.treefmt-nix.flakeModule ];
      systems = import inputs.systems;
      perSystem =
        { system
        , ...
        }:
        let
          # set up `pkgs` with rust-overlay
          overlays = [ (import inputs.rust-overlay) ];
          pkgs = (import inputs.nixpkgs) {
            inherit system overlays;
            config = {
              allowUnfree = true;
            };
          };

          #########
          # Rust  #
          #########
          rust_toolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
            targets = [ "x86_64-unknown-linux-gnu" ];
          };

          # Common build inputs based on features
          commonBuildInputs = with pkgs; [
            # Basic tools
            pkg-config
            gcc
            cmake
            gnumake
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs
            pkgs.libiconv
          ];

        in {
          # Development shell configuration
          devshells.default = {
            packages = commonBuildInputs
              ++ [
              rust_toolchain

              # Development tools
              pkgs.cargo-edit
              pkgs.cargo-audit
              pkgs.cargo-nextest
              pkgs.cargo-watch
            ];

            commands = [
              {
                help = "Run tests";
                name = "ci";
                command = "./build-and-test.sh";
              }
            ];

            env = [
              {
                name = "RUST_SRC_PATH";
                value = "${rust_toolchain}/lib/rustlib/src/rust/library";
              }
              {
                name = "LIBRARY_PATH"; # Ensures static linking finds the correct paths
                value = "${pkgs.libiconv}/lib";
              }
            ];
          };

          # Code formatting configuration
          treefmt = {
            projectRootFile = "flake.nix";
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
            };
          };

        };
    };
}
