{
  description = "Nix flake for synd-withdrawals";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    nixpkgs-2505.url = "github:NixOS/nixpkgs/nixos-25.05";

    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default-linux";
    nix-filter.url = "github:numtide/nix-filter";

    nitro = {
      type = "git";
      url = "https://github.com/SyndicateProtocol/nitro.git";
      rev = "2625cbb20485326af9f8c05bce6b025b42ba59b6";
      submodules = true;
      flake = false;
    };

    solc-0-8-17 = {
      type = "file";
      url = "https://binaries.soliditylang.org/emscripten-wasm32/solc-emscripten-wasm32-v0.8.17+commit.8df45f5f.js";
      flake = false;
    };

    solc-0-8-24 = {
      type = "file";
      url = "https://binaries.soliditylang.org/emscripten-wasm32/solc-emscripten-wasm32-v0.8.24+commit.e11b9ed9.js";
      flake = false;
    };

    enclaves-sdk-bootstrap = {
      type = "github";
      owner = "aws";
      repo = "aws-nitro-enclaves-sdk-bootstrap";
      rev = "f718dea60a9d9bb8b8682fd852ad793912f3c5db";
      flake = false;
    };

    enclaves-image-format = {
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
    flake-parts,
    systems,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} ({...}: {
      systems = import systems;
      perSystem = {
        pkgs,
        system,
        ...
      }: let
        lib = import ./nix/lib.nix {inherit pkgs system inputs;};
        myPackages = import ./nix/packages.nix {inherit lib system inputs;};
      in {
        packages = {
          inherit
            (myPackages)
            synd-withdrawals-server
            synd-enclave-server
            eif-bin
            ;
        };
      };
    });
}
