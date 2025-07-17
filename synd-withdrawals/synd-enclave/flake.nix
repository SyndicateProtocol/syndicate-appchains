{
  description = "Nix flake for synd-enclave";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

    aws-nitro-enclaves-sdk-bootstrap = {
      type = "github";
      owner = "aws";
      repo = "aws-nitro-enclaves-sdk-bootstrap";
      # commit from Dockerfile doesn't have some Nix-related fixes
      # rev = "7614f19963e4e956493b3260fda4d62834bb281c";
      rev = "f718dea60a9d9bb8b8682fd852ad793912f3c5db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, aws-nitro-enclaves-sdk-bootstrap, ... }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
    };
    enclaves-sdk = (import aws-nitro-enclaves-sdk-bootstrap) { inherit pkgs; };
  in
  {
      packages.${system} = {
        enclaves-sdk-init = enclaves-sdk.init;
        enclaves-sdk-kernel = enclaves-sdk.kernel;
        linuxkit = enclaves-sdk.linuxkit;
      };
    };
}
