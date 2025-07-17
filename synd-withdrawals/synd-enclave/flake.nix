{
  description = "Nix flake for synd-enclave";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    systems.url = "github:nix-systems/default-linux";
    flake-parts.url = "github:hercules-ci/flake-parts";

    aws-nitro-enclaves-sdk-bootstrap = {
      type = "github";
      owner = "aws";
      repo = "aws-nitro-enclaves-sdk-bootstrap";
      # commit from Dockerfile doesn't have some Nix-related fixes
      # rev = "7614f19963e4e956493b3260fda4d62834bb281c";
      rev = "f718dea60a9d9bb8b8682fd852ad793912f3c5db";
      flake = false;
    };

    aws-nitro-enclaves-image-format = {
      type = "github";
      owner = "aws";
      repo = "aws-nitro-enclaves-image-format";
      rev = "483114f1da3bad913ad1fb7d5c00dadacc6cbae6";
      flake = false;
    };
  };

  nixConfig = {
    filter-syscalls = false;
  };

  outputs = inputs @ {
    systems,
    flake-parts,
    aws-nitro-enclaves-sdk-bootstrap,
    aws-nitro-enclaves-image-format,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} ({...}: {
      systems = import systems;
      perSystem = {pkgs, ...}: let
        enclaves-sdk = (import aws-nitro-enclaves-sdk-bootstrap) {inherit pkgs;};
      in {
        packages = {
          enclaves-sdk-init = enclaves-sdk.init;
          enclaves-sdk-kernel = enclaves-sdk.kernel;
          linuxkit = enclaves-sdk.linuxkit;
          eif-build = pkgs.rustPlatform.buildRustPackage {
            pname = "eif_build";
            version = "0.2.0";
            src = aws-nitro-enclaves-image-format;
            buildAndTestSubdir = "eif_build";
            cargoHash = "sha256-mQGxBZFWQ3xW4R4j13LCt4NtAYQyO09uigLwXgOWDVE=";
            cargoPatches = [./0001-eif-build-add-cargo-lock.patch];
            nativeBuildInputs = [pkgs.pkg-config];
            buildInputs = [pkgs.openssl];
          };
        };
      };
    });
}
