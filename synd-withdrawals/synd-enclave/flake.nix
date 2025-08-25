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
    nixpkgs-foundry100,
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
        pkgs-foundry100 = import nixpkgs-foundry100 {inherit system;};
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

          solcCompilerList = pkgs.fetchurl {
            url = "https://solc-bin.ethereum.org/bin/list.json";
            hash = "sha256-cwxIF5o76FqObMW0romEUKg9xksLDOzghgHBwoBawE0=";
          };

          nitro-safe-smart-account = pkgs-2505.stdenv.mkDerivation (final: {
            name = "safe-smart-account";
            src = "${nitro}/${final.name}";
            nativeBuildInputs =
              (with solc-pkgs; [solc_0_7_6 solc_0_6_12])
              ++ (with pkgs-2505; [
                nodejs
                yarnConfigHook
                writableTmpDirAsHomeHook
                yarnInstallHook
              ]);
            buildPhase = ''
              runHook preBuild
              yarn --offline build
              runHook postBuild
            '';
            offlineCache = pkgs.fetchYarnDeps {
              yarnLock = final.src + "/yarn.lock";
              hash = "sha256-J836BIf/OJmKiruXt6HhtQzhkn0KL6hbs2Tf8se1kAY=";
            };
          });

          nitro-contracts = with pkgs-2505;
            stdenv.mkDerivation (final: {
              name = "nitro-contracts";
              src = "${nitro}/contracts";
              nativeBuildInputs = [
                pkgs-foundry100.foundry
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

          nitro-src-with-generated = pkgs.runCommand "nitro-with-generated" {} ''
            mkdir -p $out/solgen/go $out/target/include $out/target/lib
            cp -rv ${nitro}/* $out
            cp -rv ${nitro-solgen}/* $out/solgen/go
            cp ${nitro-arbitrator-prover-header}/arbitrator.h $out/target/include/arbitrator.h
            cp ${nitro-arbitrator-stylus-lib}/lib/libstylus.a $out/target/lib
            substituteInPlace \
              $out/arbcompress/native.go \
              --replace-fail "''${SRCDIR}/../target" "$out/target"
            substituteInPlace \
              $out/validator/server_arb/prover_interface.go \
              $out/execution/gethexec/executionengine.go \
              $out/arbos/programs/native_api.go \
              $out/arbos/programs/native.go \
              --replace-fail "''${SRCDIR}/../../target" "$out/target"
          '';

          synd-enclave-server = pkgs-2505.buildGoModule {
            pname = "synd-enclave";
            version = "0.1.0";
            src = pkgs.lib.fileset.toSource {
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
            preBuild = ''
              substituteInPlace \
                vendor/github.com/offchainlabs/nitro/arbcompress/native.go \
                --replace-warn '-I''${SRCDIR}/../target/include/' '-I${nitro-arbitrator-prover-header}/' \
                --replace-warn 'LDFLAGS: ''${SRCDIR}/../target/lib' 'LDFLAGS: ${nitro-arbitrator-stylus-lib}/lib'
              substituteInPlace \
                vendor/github.com/offchainlabs/nitro/execution/gethexec/executionengine.go \
                vendor/github.com/offchainlabs/nitro/arbos/programs/native_api.go \
                vendor/github.com/offchainlabs/nitro/arbos/programs/native.go \
                --replace-warn '-I../../target/include/' '-I${nitro-arbitrator-prover-header}/' \
                --replace-warn 'LDFLAGS: ''${SRCDIR}/../../target/lib' 'LDFLAGS: ${nitro-arbitrator-stylus-lib}/lib'
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
