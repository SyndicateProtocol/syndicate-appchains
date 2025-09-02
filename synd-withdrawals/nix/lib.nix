{
  pkgs,
  system,
  inputs,
  ...
}: rec {
  inherit pkgs;
  pkgs-2505 = import inputs.nixpkgs-2505 {inherit system;};
  solc-pkgs = inputs.solc.packages.${system};
  filter = inputs.nix-filter.lib;

  targetArch =
    if system == "x86_64-linux"
    then "amd64"
    else if system == "aarch64-linux"
    then "arm64"
    else abort "Unsupported architecture '${system}'";
  eifBuildCmdline = builtins.readFile ../synd-enclave/eif/cmdline-${targetArch};

  build-ramdisk = name: {
    src ? ../synd-enclave/eif,
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
}
