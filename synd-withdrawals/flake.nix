{
  description = "Nix flake for synd-withdrawals";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    nixpkgs-2505.url = "github:NixOS/nixpkgs/nixos-25.05";

    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default-linux";
    nix-filter.url = "github:numtide/nix-filter";

    solc = {
      url = "github:hellwolf/solc.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nitro = {
      type = "git";
      url = "https://github.com/SyndicateProtocol/nitro.git";
      submodules = true;
      flake = false;
    };

    enclaves-sdk-bootstrap = {
      type = "github";
      owner = "aws";
      repo = "aws-nitro-enclaves-sdk-bootstrap";
      rev = "7614f19963e4e956493b3260fda4d62834bb281c";
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
