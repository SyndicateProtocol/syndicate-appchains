{
  description = "Nix flake for synd-enclave";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    nixpkgs-2505.url = "github:NixOS/nixpkgs/nixos-25.05";
    nixpkgs-foundry100 = {
      type = "github";
      owner = "NixOS";
      repo = "nixpkgs";
      rev = "6a0960ad4b3d13bff34bd78e9fcefc4310507707";
      flake = false;
    };

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
    nixpkgs-foundry100,
    nixpkgs-2505,
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
        pkgs-foundry100 = import nixpkgs-foundry100 {inherit system;};
        pkgs-2505 = import nixpkgs-2505 {inherit system;};
        build-ramdisk = {
          name,
          init,
          kernel,
          linuxkit,
        }:
          pkgs.stdenv.mkDerivation {
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
            nativeBuildInputs = [linuxkit];
          };
      in {
        packages = rec {
          enclaves-sdk-init = (import "${enclaves-sdk-bootstrap}/init/init.nix") {inherit pkgs;};
          enclaves-sdk-kernel = (import "${enclaves-sdk-bootstrap}/kernel/kernel.nix") {inherit pkgs;};
          linuxkit = (import "${enclaves-sdk-bootstrap}/linuxkit/linuxkit.nix") {inherit pkgs;};

          brotli-lib = pkgs.stdenv.mkDerivation {
            pname = "brotli";
            version = "1.0.9";
            src = "${nitro}/brotli";
            nativeBuildInputs = with pkgs; [cmake];
            preConfigure = ''
              export HOME=$(mktemp -d)
            '';
            cmakeFlags = with pkgs; [
              (lib.cmakeFeature "CMAKE_POLICY_VERSION_MINIMUM" "3.5")
              (lib.cmakeFeature "CMAKE_BUILD_TYPE" "Release")
              (lib.cmakeFeature "CMAKE_INSTALL_PREFIX" "$out")
            ];
          };

          brotli-wasm = brotli-lib.overrideAttrs (prev: {
            pname = prev.pname + "-wasm";
            nativeBuildInputs = with pkgs;
              prev.nativeBuildInputs
              ++ [coreutils emscripten];
            cmakeFlags = with pkgs;
              prev.cmakeFlags
              ++ [
                (lib.cmakeFeature "CMAKE_C_COMPILER" "${emscripten}/bin/emcc")
                (lib.cmakeFeature "CMAKE_C_FLAGS" "-fPIC")
                (lib.cmakeFeature "CMAKE_AR" "${emscripten}/bin/emar")
                (lib.cmakeFeature "CMAKE_RANLIB" "${coreutils}/bin/touch")
              ];
            postInstall = ''
              mv $out/lib $out/lib-wasm
            '';
          });

          nitro-contracts = with pkgs-2505;
            stdenv.mkDerivation (final: {
              name = "nitro-contracts";
              src = "${nitro}/contracts";
              nativeBuildInputs = [
                pkgs-foundry100.foundry
                pkgs-foundry100.solc
                yarnConfigHook
                writableTmpDirAsHomeHook
                yarnInstallHook
                nodejs
              ];
              buildPhase = ''
                runHook preBuild
                yarn --offline build
                yarn --offline build:forge:yul
                runHook postBuild
              '';
              offlineCache = fetchYarnDeps {
                yarnLock = final.src + "/yarn.lock";
                hash = "sha256-tg8Mk0c+x3gLOSNY+y1fBdQ0I95PAgG07qFQ/d64wIU=";
              };
            });

          nitro-arbitrator-wasm-forward-bin = pkgs-2505.rustPlatform.buildRustPackage {
            pname = "forward";
            version = "0.1.0";
            src = "${nitro}/arbitrator/wasm-libraries/forward";
            cargoHash = "sha256-KVpxO0/VuNGM1LT5ReFyf6+Qd7JYXGDjzpLWab46P4o=";
            cargoPatches = [./patches/wasm-libraries-forward-cargo-lock.patch];
          };

          nitro-arbitrator-forward-wasm = pkgs.runCommand "forward.wasm" {} ''
            ${nitro-arbitrator-wasm-forward-bin}/bin/forward --path forward.wat
            ${pkgs.wabt}/bin/wat2wasm forward.wat -o $out
          '';

          nitro-arbitrator-stylus-lib = pkgs.rustPlatform.buildRustPackage {
            pname = "stylus";
            version = "0.1.0";
            src = "${nitro}/arbitrator";
            buildAndTestSubdir = "stylus";
            cargoHash = "sha256-/57DFSr9nxNVpIyNBdFai6zKCrA/RHCoX69Cj29p1pI=";
            doCheck = false;
            preBuild = ''
              mkdir -p ../target
              cp -r ${brotli-lib}/{include,lib} ../target/
              cp -r ${brotli-wasm}/lib-wasm ../target/

              # forward_stub.wasm -> forward.wasm
              sed -i 's#../../../target/machines/latest/forward_stub.wasm#${nitro-arbitrator-forward-wasm}#' prover/src/{machine,test}.rs
            '';
          };

          # TODO: stylus.overrideAttrs or DRY function
          nitro-arbitrator-prover = pkgs.rustPlatform.buildRustPackage {
            pname = "prover";
            version = "0.1.0";
            src = "${nitro}/arbitrator";
            buildAndTestSubdir = "prover";
            cargoHash = "sha256-ah/bZj4X40Q1l2O9vLpGE0E0AHi2CMpKTBFj8HyE66k=";
            preBuild = ''
              mkdir -p ../target
              cp -r ${brotli-lib}/{include,lib} ../target/
              cp -r ${brotli-wasm}/lib-wasm ../target/

              # forward_stub.wasm -> forward.wasm
              sed -i 's#../../../target/machines/latest/forward_stub.wasm#${nitro-arbitrator-forward-wasm}#' prover/src/{machine,test}.rs
            '';
          };

          nitro-arbitrator-jit = pkgs.rustPlatform.buildRustPackage {
            pname = "jit";
            version = "0.1.0";
            src = "${nitro}/arbitrator";
            buildAndTestSubdir = "jit";
            cargoHash = "sha256-CsbZhKxa6lf+VbeSw7CBFYOKcHkzvvtOTK/F+lsbty4=";
            preBuild = ''
              mkdir -p ../target
              cp -r ${brotli-lib}/{include,lib} ../target/
              cp -r ${brotli-wasm}/lib-wasm ../target/

              # forward_stub.wasm -> forward.wasm
              sed -i 's#../../../target/machines/latest/forward_stub.wasm#${nitro-arbitrator-forward-wasm}#' prover/src/{machine,test}.rs
            '';
          };

          nitro-arbitrator-prover-header = pkgs.runCommand "arbitrator.h" {} ''
            ${pkgs-2505.rust-cbindgen}/bin/cbindgen \
              --config ${nitro}/arbitrator/stylus/cbindgen.toml \
              --metadata ${./patches/stylus-cargo-metadata.json} \
              --output $out \
              ${nitro}/arbitrator/stylus
          '';

          synd-enclave = pkgs-2505.buildGoModule {
            pname = "synd-enclave";
            version = "0.1.0";
            src = ./.;
            vendorHash = pkgs.lib.fakeHash;
            subPackages = [ "cmd/enclave" "enclave" ];
            ldFlags = [
              "-linkmode external"
              "-extldflags"
              "-static"
            ];
            preBuild = ''
              ln -s ${nitro} ./nitro
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
            cargoPatches = [./patches/eif-build-cargo-lock.patch];
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
