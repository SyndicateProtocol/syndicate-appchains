{
  description = "Nix flake for synd-enclave";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    systems.url = "github:nix-systems/default-linux";
    flake-parts.url = "github:hercules-ci/flake-parts";

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
    systems,
    flake-parts,
    enclaves-sdk-bootstrap,
    enclaves-image-format,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} ({...}: {
      systems = import systems;
      perSystem = {
        pkgs,
        system,
        ...
      }: {
        packages = rec {
          enclaves-sdk-init = (import "${enclaves-sdk-bootstrap}/init/init.nix") {inherit pkgs;};
          enclaves-sdk-kernel = (import "${enclaves-sdk-bootstrap}/kernel/kernel.nix") {inherit pkgs;};
          linuxkit = (import "${enclaves-sdk-bootstrap}/linuxkit/linuxkit.nix") {inherit pkgs;};

          eif-build = pkgs.rustPlatform.buildRustPackage {
            pname = "eif_build";
            version = "0.2.0";
            src = enclaves-image-format;
            buildAndTestSubdir = "eif_build";
            cargoHash = "sha256-mQGxBZFWQ3xW4R4j13LCt4NtAYQyO09uigLwXgOWDVE=";
            cargoPatches = [./0001-eif-build-add-cargo-lock.patch];
            nativeBuildInputs = [pkgs.pkg-config];
            buildInputs = [pkgs.openssl];
          };

          build-ramdisk = name:
            pkgs.runCommand "build-ramdisk-${name}" {} ''
                mkdir -p ./bootstrap
                cp ${enclaves-sdk-init}/* ./bootstrap/
                cp ${enclaves-sdk-kernel}/* ./bootstrap/

                HOME=$PWD ${linuxkit}/bin/linuxkit build \
                --format kernel+initrd \
                --no-sbom \
                --name ${name}-ramdisk \
                ${./eif/${name}-ramdisk.yaml}
              cp ${name}-ramdisk-initrd.img $out
            '';

          eif-bin = let
            targetArch =
              if system == "x86_64-linux"
              then "amd64"
              else if system == "aarch64-linux"
              then "arm64"
              else abort "Unsupported architecture '${system}'";
            cmdline = builtins.readFile ./eif/cmdline-${targetArch};
          in
            pkgs.runCommand "eif.bin" {} ''
              ${eif-build}/bin/eif_build \
                --kernel ${enclaves-sdk-kernel}/*Image \
                --kernel_config ${enclaves-sdk-kernel}/*Image.config \
                --cmdline "${cmdline}" \
                --ramdisk ${build-ramdisk "init"} \
                --ramdisk ${build-ramdisk "user"} \
                --output $out
            '';
        };
      };
    });
}
