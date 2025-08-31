{
  lib,
  pkgs ? lib.pkgs,
  system,
  inputs,
  ...
}: rec {
  synd-withdrawals-server = lib.pkgs-2505.buildGoModule {
    pname = "synd-withdrawals-server";
    version = "0.1.0";
    src = pkgs.lib.fileset.toSource {
      root = ../server;
      fileset = pkgs.lib.fileset.unions [
        ../server/main.go
        ../server/go.mod
        ../server/go.sum
      ];
    };
    vendorHash = "sha256-2Qz6GSZJaFclof3R+Qgn5sB7dQS7lF9sXR27Z0TEm4w=";
  };

  synd-enclave-server = lib.pkgs-2505.buildGoModule {
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

  eif-bin = let
    inherit (enclaves-sdk) init kernel linuxkit;
    enclave = synd-enclave-server;
    init-ramdisk = lib.build-ramdisk "init" {
      inherit init kernel linuxkit enclave;
    };
    user-ramdisk = lib.build-ramdisk "user" {
      inherit init kernel linuxkit enclave;
    };
  in
    pkgs.runCommand "eif.bin" {} ''
      ${eif-build}/bin/eif_build \
        --kernel ${kernel}/*Image \
        --kernel_config ${kernel}/*Image.config \
        --cmdline "${lib.eifBuildCmdline}" \
        --ramdisk "${init-ramdisk}/init-ramdisk-initrd.img" \
        --ramdisk "${user-ramdisk}/user-ramdisk-initrd.img" \
        --build-time 1970-01-01T00:00:00Z \
        --output $out
    '';

  # dependencies start here

  enclaves-sdk.init = (import "${inputs.enclaves-sdk-bootstrap}/init/init.nix") {inherit pkgs;};
  enclaves-sdk.kernel = (import "${inputs.enclaves-sdk-bootstrap}/kernel/kernel.nix") {inherit pkgs;};
  enclaves-sdk.linuxkit = (import "${inputs.enclaves-sdk-bootstrap}/linuxkit/linuxkit.nix") {inherit pkgs;};

  brotli-lib = pkgs.stdenv.mkDerivation {
    pname = "brotli";
    version = "1.0.9";
    src = "${inputs.nitro}/brotli";
    nativeBuildInputs = [pkgs.cmake];
    preConfigure = ''
      export HOME=$(mktemp -d)
    '';
    cmakeFlags = [
      (pkgs.lib.cmakeFeature "CMAKE_POLICY_VERSION_MINIMUM" "3.5")
      (pkgs.lib.cmakeFeature "CMAKE_BUILD_TYPE" "Release")
      (pkgs.lib.cmakeFeature "CMAKE_INSTALL_PREFIX" "$out")
    ];
  };

  brotli-wasm = brotli-lib.overrideAttrs (prev: {
    pname = prev.pname + "-wasm";
    nativeBuildInputs =
      prev.nativeBuildInputs
      ++ (with pkgs; [coreutils emscripten]);
    cmakeFlags =
      prev.cmakeFlags
      ++ [
        (pkgs.lib.cmakeFeature "CMAKE_C_COMPILER" "${pkgs.emscripten}/bin/emcc")
        (pkgs.lib.cmakeFeature "CMAKE_C_FLAGS" "-fPIC")
        (pkgs.lib.cmakeFeature "CMAKE_AR" "${pkgs.emscripten}/bin/emar")
        (pkgs.lib.cmakeFeature "CMAKE_RANLIB" "${pkgs.coreutils}/bin/touch")
      ];
    postInstall = ''
      mv $out/lib $out/lib-wasm
    '';
  });

  nitro-contracts = lib.pkgs-2505.stdenv.mkDerivation (final: {
    name = "nitro-contracts";
    src = "${inputs.nitro}/contracts";
    nativeBuildInputs = with lib.pkgs-2505;
      [
        foundry
        yarnConfigHook
        yarnBuildHook
        yarnInstallHook
        writableTmpDirAsHomeHook
        nodejs
      ]
      ++ [lib.solc-pkgs.solc_0_8_28];
    offlineCache = lib.pkgs-2505.fetchYarnDeps {
      yarnLock = "${final.src}/yarn.lock";
      hash = "sha256-1DqXlJvhWf7OugnTdNfqupHGyPz2AphZjTLuKjOyppY=";
    };
    patchPhase = let
      hardhatSolcCache = "$HOME/.cache/hardhat-nodejs/compilers-v2/linux-${lib.targetArch}";
      solcBin = version: let
        cleanVersion = builtins.head (builtins.split "-" version);
        pkg = lib.solc-pkgs."solc_${builtins.replaceStrings ["."] ["_"] cleanVersion}";
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

  nitro-arbitrator-wasm-forward-bin = lib.pkgs-2505.rustPlatform.buildRustPackage {
    pname = "forward";
    version = "0.1.0";
    src = "${inputs.nitro}/arbitrator/wasm-libraries/forward";
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
    src = "${inputs.nitro}/arbitrator";
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

  nitro-solgen = lib.pkgs-2505.buildGoModule {
    name = "solgen";
    src = lib.filter {
      root = inputs.nitro;
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
      root = ../synd-enclave;
      fileset = pkgs.lib.fileset.unions [
        ../synd-enclave/cmd/enclave
        ../synd-enclave/enclave
        ../synd-enclave/teemodule
        ../synd-enclave/teetypes
        ../synd-enclave/go.mod
        ../synd-enclave/go.sum
      ];
    };
  in
    pkgs.runCommand "enclave-src-with-generated" {} ''
      mkdir -p $out/nitro/solgen/go $out/nitro/target/include $out/nitro/target/lib
      cp -rv ${enclave-src}/* $out/
      cp -rv ${inputs.nitro}/* $out/nitro/
      cp -rv ${nitro-solgen}/* $out/nitro/solgen/go
      cp -v ${nitro-arbitrator-prover-header}/arbitrator.h $out/nitro/target/include
      cp -rv ${nitro-arbitrator-stylus-lib}/lib/* $out/nitro/target/lib
    '';

  eif-build = pkgs.rustPlatform.buildRustPackage rec {
    pname = "eif_build";
    version = "0.2.0";
    src = inputs.enclaves-image-format;
    buildAndTestSubdir = pname;
    cargoHash = "sha256-mQGxBZFWQ3xW4R4j13LCt4NtAYQyO09uigLwXgOWDVE=";
    cargoPatches = [./patches/eif-build-cargo-lock.patch];
    nativeBuildInputs = [pkgs.pkg-config];
    buildInputs = [pkgs.openssl];
  };
}
