{
  description = "Nix flake for synd-enclave";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    nixpkgs-2505.url = "github:NixOS/nixpkgs/nixos-25.05";

    # TODO: verify whether aarch64-linux works
    systems.url = "github:nix-systems/x86_64-linux";
    flake-parts.url = "github:hercules-ci/flake-parts";
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
    nixpkgs-2505,
    systems,
    flake-parts,
    nix-filter,
    solc,
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
        pkgs-2505 = import nixpkgs-2505 {inherit system;};
        solc-pkgs = solc.packages.${system};
        filter = nix-filter.lib;
        build-ramdisk = {
          name,
          src ? ./eif,
          init,
          kernel,
          linuxkit,
          enclave,
        }:
          pkgs.stdenv.mkDerivation {
            name = "${name}-ramdisk";
            inherit src;
            buildPhase = ''
              mkdir -p ./bootstrap ./bin
              cp ${init}/* ./bootstrap/
              cp ${kernel}/* ./bootstrap/
              cp ${enclave}/bin/enclave ./bin/enclave

              HOME=$PWD ${linuxkit}/bin/linuxkit build \
              --format kernel+initrd \
              --no-sbom \
              --name ${name}-ramdisk \
              $src/${name}-ramdisk.yaml
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
                foundry
                solc-pkgs.solc_0_8_28
                yarnConfigHook
                yarnBuildHook
                yarnInstallHook
                writableTmpDirAsHomeHook
                nodejs
              ];
              offlineCache = fetchYarnDeps {
                yarnLock = "${final.src}/yarn.lock";
                hash = "sha256-1DqXlJvhWf7OugnTdNfqupHGyPz2AphZjTLuKjOyppY=";
              };
              patchPhase = let
                targetArch =
                  if system == "x86_64-linux"
                  then "amd64"
                  else if system == "aarch64-linux"
                  then "arm64"
                  else abort "Unsupported architecture '${system}'";
                hardhatSolcCache = "$HOME/.cache/hardhat-nodejs/compilers-v2/linux-${targetArch}";
                solcBin = version: let
                  cleanVersion = builtins.head (builtins.split "-" version);
                  pkg = solc-pkgs."solc_${builtins.replaceStrings ["."] ["_"] cleanVersion}";
                in "${pkg}/bin/solc-${cleanVersion}";
                hardhatPatch = ''
                  import { HardhatUserConfig, subtask } from 'hardhat/config';

                  const {
                    TASK_COMPILE_SOLIDITY_GET_SOLC_BUILD,
                  } = require('hardhat/builtin-tasks/task-names');

                  subtask(
                    TASK_COMPILE_SOLIDITY_GET_SOLC_BUILD,
                    async (
                      args: {
                        solcVersion: string;
                      },
                      hre,
                      runSuper
                    ) => {
                      var compilerPath;
                      switch (args.solcVersion) {
                        case '0.8.17':
                          compilerPath = '${solcBin "0.8.17"}';
                          break;
                        case '0.8.24':
                          compilerPath = '${solcBin "0.8.24"}';
                          break;
                        default:
                          return runSuper();
                      }
                      return {
                        compilerPath,
                        isSolcJs: false,
                        version: args.solcVersion,
                        // This is used as extra information in the build-info files,
                        // but other than that is not important
                        longVersion: args.solcVersion,
                      };
                    }
                  );
                '';
              in ''
                awk -v n=113 -v s="${hardhatPatch}" "NR == n {print s} {print}" hardhat.config.ts > tmp && mv tmp hardhat.config.ts
              '';
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
            cargoHash = "sha256-w38dRbXeXM5IsE/cktk9M9D29+8d+dULNzcPcx+Yzv8=";
            doCheck = false;
            preBuild = ''
              mkdir -p ../target
              cp -r ${brotli-lib}/{include,lib} ../target/
              cp -r ${brotli-wasm}/lib-wasm ../target/

              # forward_stub.wasm -> forward.wasm
              sed -i 's#../../../target/machines/latest/forward_stub.wasm#${nitro-arbitrator-forward-wasm}#' prover/src/{machine,test}.rs
            '';
            postInstall = ''
              mkdir -p $out/include
              ${pkgs.rust-cbindgen}/bin/cbindgen --config stylus/cbindgen.toml --crate stylus --output $out/include/arbitrator.h
            '';
          };

          nitro-arbitrator-prover-header = pkgs.runCommandLocal "arbitrator-prover-header" {} ''
            mkdir -p $out
            cp ${nitro-arbitrator-stylus-lib}/include/arbitrator.h $out/arbitrator.h
          '';

          nitro-solgen = pkgs-2505.buildGoModule {
            name = "solgen";
            src = filter {
              root = nitro;
              include = ["go-ethereum" "solgen" "go.mod" "go.sum"];
            };
            subPackages = ["solgen"];
            patchPhase = ''
              mkdir -p ./contracts
              ln -s ${nitro-contracts}/lib/node_modules/@eigenda/nitro-contracts/build ./contracts/build
            '';
            installPhase = ''
              mkdir $out
              go run solgen/gen.go
              cp -r solgen/go/* $out/
            '';
            vendorHash = "sha256-dJUOTd/LSeIMO2m8DmTc1tkphBn3bA2OM4Vl66PgJR8=";
          };

          enclave-src-with-generated = let
            enclave-src = pkgs.lib.fileset.toSource {
              root = ./.;
              fileset = pkgs.lib.fileset.unions [
                ./cmd/enclave
                ./enclave
                ./teemodule
                ./teetypes
                ./go.mod
                ./go.sum
              ];
            };
          in
            pkgs.runCommand "enclave-src-with-generated" {} ''
              mkdir -p $out/nitro/solgen/go $out/nitro/target/include $out/nitro/target/lib
              cp -rv ${enclave-src}/* $out/
              cp -rv ${nitro}/* $out/nitro/
              cp -rv ${nitro-solgen}/* $out/nitro/solgen/go
              cp -v ${nitro-arbitrator-prover-header}/arbitrator.h $out/nitro/target/include
              cp -rv ${nitro-arbitrator-stylus-lib}/lib/* $out/nitro/target/lib
            '';

          synd-enclave-server = pkgs-2505.buildGoModule {
            pname = "synd-enclave";
            version = "0.1.0";
            src = enclave-src-with-generated;
            dontCheck = true;
            preBuild = ''
              if [ -d vendor ]; then
                chmod +w vendor/github.com/offchainlabs/nitro
                mkdir -p vendor/github.com/offchainlabs/nitro/target/{include,lib}
                cp -v ${nitro-arbitrator-prover-header}/arbitrator.h vendor/github.com/offchainlabs/nitro/target/include
                cp -rv ${nitro-arbitrator-stylus-lib}/lib/* vendor/github.com/offchainlabs/nitro/target/lib
              fi
            '';
            vendorHash = "sha256-IWk71nxHQH/Uw3MfMTrJYVHndy89GZjOx1d0wN1TYek=";
            subPackages = ["cmd/enclave"];
            ldFlags = [
              "-linkmode external"
              "-extldflags"
              "-static"
            ];
          };

          init-ramdisk = build-ramdisk {
            name = "init";
            init = enclaves-sdk-init;
            kernel = enclaves-sdk-kernel;
            inherit linuxkit;
            enclave = synd-enclave-server;
          };

          user-ramdisk = build-ramdisk {
            name = "user";
            init = enclaves-sdk-init;
            kernel = enclaves-sdk-kernel;
            inherit linuxkit;
            enclave = synd-enclave-server;
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
                --build-time 1970-01-01T00:00:00Z \
                --output $out
            '';
        };
      };
    });
}
