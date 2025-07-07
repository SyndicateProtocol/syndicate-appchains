# Syndicate Withdrawals

This system manages the validation and submission of withdrawal proofs through a Trusted Execution Environment (TEE). It ingests data from multiple chains and nodes, performs secure verification inside an enclave, and submits validated proofs to on-chain contracts.

---

## 🧩 Overview of Components

### 🔹 `Synd-Proposer`

**Role:** Orchestrates the full withdrawal flow.  
**Main responsibilities:**

- **Data aggregation** from multiple sources:

  - **L1 node:** `eth_getProof`
  - **Settlement node:** `eth_getLogs`
  - **Sequencing node:** `eth_getBlockByNumber`, `eth_getLogs`, `eth_getProof`, `arbdebug_validationInputsAt`
  - **Appchain node:** `arbdebug_validationInputsAt`

- **Workflow:**
  1. Constructs a `PendingAssertion` using collected data.
  2. Invokes the secure TEE module to verify the assertion.
  3. Submits the verified assertion on-chain via:
     - `submitPendingAssertion` → `TEEModule.sol`

### 🔹 `Synd-Enclave`

**Role:** Secure enclave runtime that performs the core logic for withdrawal proof validation.  
**Codebase:** Forked from [base/op-enclave](https://github.com/base/op-enclave)

#### 🚀 Running the Enclave

\*\*There are two supported execution modes:

##### ➤ Local Execution (Development Mode)

1. Build the enclave binary:
   ```bash
   go build -C cmd/enclave
   ```
2. Run the enclave:
   ```bash
   ./cmd/enclave/enclave [--local]
   ```

### 🔹 `Synd-Tee-Attestation-ZK-Proofs`

**Role:** Handles generation and verification of zk-SNARKs for attestation documents produced by `synd-enclave`.

---

## 🐳 Docker

To build a Docker image for the `synd-enclave` in a compatible environment:

```bash
docker build -f synd-withdrawals/synd-enclave/Dockerfile . --platform linux/amd64
```

---
