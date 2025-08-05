# ZK proofs for TEE attestations

This directory contains a few components;

- `aws-nitro` - contains generic and reusable code to validate AWS nitro TEE certificates. The code in this crate can be used on different zk proof engines
- `sp1` - sp1 implementation for zk proving of TEE attestion documents. Initially only supports AWS nitro.
- `proof-submitter` - CLI utility used to generate and submit zk proofs
- `setup-box.sh` - script used to automate the setup of a fresh GPU instance.

# ⚙️ GPU Box Provisioning Script

This repository includes a provisioning script, `setup-box.sh`, used to **automate the setup of a fresh GPU instance** for submitting SP1 proofs to the smart contract verifier.

The script performs the following actions:

- Installs system dependencies (Rust, Docker, Git, etc.)
- Fixes `/etc/hosts` hostname resolution
- Installs SP1
- Clones the `syndicate-appchains` repo
- Runs the `proof-submitter` binary using your compiled ELF and secrets

## 🚀 How to Use

### 1. Prepare environment variables

Update the env vars in the `setup-box.sh` script:

```
KEY_MANAGER_ADDRESS=0xYourKeyManagerAddress
PRIVATE_KEY=0xYourPrivateKey
```

PRIVATE_KEY just needs to be any wallet with settlement funds.

### 2. Generate the file to copy

On your machine, run the following from inside of `syndicate-appchains/synd-withdrawals/synd-tee-attestation-zk-proofs`

```
curl -L https://sp1.succinct.xyz | bash
sp1up
just sp1-generate-vkey
```

> This should have generated the ../../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/synd-tee-attestation-zk-proofs-sp1-program file. Also the "vkey" which you can probably ignore.

### 3. Upload files to the box

On your machine, run the following from inside of ``syndicate-appchains/synd-withdrawals/`:

```
scp setup-box.sh ubuntu@<YOUR_BOX_IP>:~/
scp ../../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/synd-tee-attestation-zk-proofs-sp1-program ubuntu@<YOUR_BOX_IP>:~/
```

# 3. SSH and run the setup

```
ssh ubuntu@<YOUR_BOX_IP>
bash ~/setup-box.sh
```
