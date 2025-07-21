{
  description = "Nix flake for synd-enclave";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    systems.url = "github:nix-systems/default-linux";
    flake-parts.url = "github:hercules-ci/flake-parts";

    nitro = {
      type = "git";
      url = "https://github.com/Layr-Labs/nitro.git";
      rev = "5b41a2dcf81c2b6f4798ecb472c27002a6af9bab";
      flake = false;
      submodules = true;
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
    systems,
    flake-parts,
    nitro,
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
      }: let
      build-ramdisk = {name, init, kernel, linuxkit}: pkgs.stdenv.mkDerivation {
        name = "${name}-ramdisk";
        buildPhase = ''
          mkdir -p ./bootstrap
          cp ${init}/* ./bootstrap/
          cp ${kernel}/* ./bootstrap/

          HOME=$PWD ${linuxkit}/bin/linuxkit build \
          --format kernel+initrd \
          --no-sbom \
          --name ${name}-ramdisk \
          ${./eif/${name}-ramdisk.yaml}
        '';
        installPhase = ''
          mkdir -p $out
          cp ${name}-ramdisk-initrd.img $out
        '';
        nativeBuildInputs = [ linuxkit ];
      };

      in {
        packages = rec {
          enclaves-sdk-init = (import "${enclaves-sdk-bootstrap}/init/init.nix") {inherit pkgs;};
          enclaves-sdk-kernel = (import "${enclaves-sdk-bootstrap}/kernel/kernel.nix") {inherit pkgs;};
          linuxkit = (import "${enclaves-sdk-bootstrap}/linuxkit/linuxkit.nix") {inherit pkgs;};

          brotli-wasm = pkgs.stdenv.mkDerivation {
            pname = "brotli-wasm";
            version = "1.0.9";
            src = "${nitro}/brotli";
            nativeBuildInputs = with pkgs; [cmake coreutils emscripten];
            preConfigure = ''
              export HOME=$(mktemp -d)
            '';
            cmakeFlags = with pkgs; [
                (lib.cmakeFeature "CMAKE_POLICY_VERSION_MINIMUM" "3.5")
                (lib.cmakeFeature "CMAKE_BUILD_TYPE" "Release")
                (lib.cmakeFeature "CMAKE_C_COMPILER" "${emscripten}/bin/emcc")
                (lib.cmakeFeature "CMAKE_C_FLAGS" "-fPIC")
                (lib.cmakeFeature "CMAKE_INSTALL_PREFIX" "$out")
                (lib.cmakeFeature "CMAKE_AR" "${emscripten}/bin/emar")
                (lib.cmakeFeature "CMAKE_RANLIB" "${coreutils}/bin/touch")
            ];
            postInstall = ''
              mv $out/lib $out/lib-wasm
            '';
          };

          init-ramdisk = build-ramdisk {
            name = "init";
            init = enclaves-sdk-init;
            kernel = enclaves-sdk-kernel;
            inherit linuxkit;
          };

          user-ramdisk = build-ramdisk {
            name = "user";
            init = enclaves-sdk-init;
            kernel = enclaves-sdk-kernel;
            inherit linuxkit;
          };

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
                --ramdisk "${init-ramdisk}/init-ramdisk-initrd.img" \
                --ramdisk "${user-ramdisk}/user-ramdisk-initrd.img" \
                --output $out
            '';
        };
      };
    });
}
