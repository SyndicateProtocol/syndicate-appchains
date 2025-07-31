# Syndicate Withdrawals

This system manages the validation and submission of withdrawal proofs through a Trusted Execution Environment (TEE). It ingests data from multiple chains and nodes, performs secure verification inside an enclave, and submits validated proofs to on-chain contracts.

## System Architecture

The Syndicate withdrawal system uses a TEE-based architecture to securely validate and submit withdrawal proofs across multiple chains.

### Architecture Overview

```mermaid
graph TB
    %% Source Chains
    subgraph "Source Chains"
        L1[L1 Chain<br/>- Block headers<br/>- Batch accumulators<br/>- Bridge state]
        SEQ[Sequencing Chain<br/>- Block headers<br/>- Batch data<br/>- Validation inputs]
        SET[Settlement Chain<br/>- Delayed messages<br/>- Bridge logs<br/>- State proofs]
        APP[Appchain<br/>- Block headers<br/>- Send roots<br/>- Validation inputs]
    end

    %% Synd-Proposer
    subgraph "Synd-Proposer"
        PROPOSER[Proposer Service<br/>- Data aggregation<br/>- Trusted input collection<br/>- Assertion construction]
        POLLING[Polling Loop<br/>- Monitors chain state<br/>- Triggers verification<br/>- Submits assertions]
    end

    %% Synd-Enclave
    subgraph "Synd-Enclave"
        ENCLAVE[TEE Enclave<br/>- Secure verification<br/>- Cryptographic signing<br/>- State validation]
        VERIFY_SEQ[Verify Sequencing Chain<br/>- Batch verification<br/>- Accumulator validation<br/>- Signature generation]
        VERIFY_APP[Verify Appchain<br/>- Block verification<br/>- Send root validation<br/>- Assertion creation]
    end

    %% On-Chain Contracts
    subgraph "On-Chain Contracts"
        TEE_MODULE[TeeModule Contract<br/>- Assertion management<br/>- Challenge system<br/>- State transitions]
        KEY_MGR[TeeKeyManager<br/>- TEE key validation<br/>- Public key management<br/>- Attestation verification]
        ASSERTION_POSTER[AssertionPoster<br/>- Assertion posting<br/>- Rollup integration<br/>- State updates]
    end

    %% Data Flow
    L1 --> PROPOSER
    SEQ --> PROPOSER
    SET --> PROPOSER
    APP --> PROPOSER

    PROPOSER --> ENCLAVE
    ENCLAVE --> VERIFY_SEQ
    ENCLAVE --> VERIFY_APP

    VERIFY_SEQ --> TEE_MODULE
    VERIFY_APP --> TEE_MODULE
    TEE_MODULE --> KEY_MGR
    TEE_MODULE --> ASSERTION_POSTER

    %% Styling - High contrast for dark mode
    classDef sourceChain fill:#1e3a8a,stroke:#1e40af,color:#ffffff
    classDef proposer fill:#7c3aed,stroke:#8b5cf6,color:#ffffff
    classDef enclave fill:#059669,stroke:#10b981,color:#ffffff
    classDef contracts fill:#dc2626,stroke:#ef4444,color:#ffffff

    class L1,SEQ,SET,APP sourceChain
    class PROPOSER,POLLING proposer
    class ENCLAVE,VERIFY_SEQ,VERIFY_APP enclave
    class TEE_MODULE,KEY_MGR,ASSERTION_POSTER contracts
```

### Detailed Withdrawal Flow

```mermaid
sequenceDiagram
    participant L1 as L1 Chain
    participant SEQ as Sequencing Chain
    participant SET as Settlement Chain
    participant APP as Appchain
    participant PROPOSER as Synd-Proposer
    participant ENCLAVE as Synd-Enclave
    participant TEE as TeeModule
    participant KEY as TeeKeyManager
    participant POSTER as AssertionPoster

    Note over L1,APP: Data Collection Phase
    PROPOSER->>L1: eth_getProof<br/>(batch accumulator)
    PROPOSER->>SEQ: eth_getBlockByNumber<br/>eth_getLogs<br/>arbdebug_validationInputsAt
    PROPOSER->>SET: eth_getLogs<br/>(delayed messages)
    PROPOSER->>APP: arbdebug_validationInputsAt<br/>(block headers, send roots)

    Note over PROPOSER: Trusted Input Construction
    PROPOSER->>PROPOSER: Collect data from all chains
    PROPOSER->>PROPOSER: Build TrustedInput<br/>(config hash, block hashes, accumulators)
    PROPOSER->>TEE: Get current teeTrustedInput

    Note over ENCLAVE: Secure Verification Phase
    PROPOSER->>ENCLAVE: VerifySequencingChain<br/>(TrustedInput, batches, messages)
    ENCLAVE->>ENCLAVE: Verify batch data<br/>Validate accumulators<br/>Generate signature
    ENCLAVE->>PROPOSER: VerifySequencingChainOutput<br/>(L1BatchAcc, SeqBlockHash, signature)

    PROPOSER->>ENCLAVE: VerifyAppchain<br/>(TrustedInput, sequencing output)
    ENCLAVE->>ENCLAVE: Verify appchain blocks<br/>Validate send roots<br/>Create assertion
    ENCLAVE->>PROPOSER: VerifyAppchainOutput<br/>(PendingAssertion, signature)

    Note over TEE: On-Chain Submission
    PROPOSER->>TEE: submitAssertion<br/>(PendingAssertion, signature, rewardAddr)
    TEE->>KEY: Validate TEE signature<br/>(payload_hash.recover(signature))
    TEE->>TEE: Store pending assertion<br/>Start challenge window

    alt Challenge Window
        Note over TEE: Challenge Period
        TEE->>TEE: Wait for challenge window<br/>(configurable duration)
        alt Multiple Assertions
            TEE->>TEE: Detect conflicting assertions<br/>Increment teeHackCount<br/>Reward honest participant
        else Single Assertion
            TEE->>TEE: closeChallengeWindow<br/>Update trusted input<br/>Post assertion
            TEE->>POSTER: postAssertion<br/>(appBlockHash, appSendRoot)
        end
    end

    Note over POSTER: Final State Update
    POSTER->>POSTER: Update rollup state<br/>Enable withdrawals
```

This architecture ensures secure, verifiable withdrawals while maintaining decentralization and trustlessness through cryptographic proofs and TEE-based verification.

---

## 🔒 System Guarantees & Assumptions

### **TEE Requirements**

- **Hardware Security**: All critical verification must be performed within a Trusted Execution Environment (TEE)
- **Attestation**: TEE instances provide cryptographic attestation documents
- **Key Management**: TEE instances securely manage private keys for signing assertions

### **Supported Platforms**

- **AWS Nitro Enclaves**: Currently the only supported TEE platform
- **Future Support**: Additional TEE platforms (Azure, GCP, etc.) are planned

### **Challenge System**

- **Challenge Window**: Configurable time period during which disputes can be raised
- **Multiple Assertions**: If conflicting assertions are submitted during the challenge window:
  - The system detects the conflict and increments `teeHackCount`
  - Honest participants are rewarded
  - Malicious behavior is penalized
- **Single Assertion**: If only one assertion is submitted, it's finalized after the challenge window

### **Security Assumptions**

- **TEE Integrity**: The TEE hardware and firmware are trusted and secure
- **Network Security**: Communication between components is secure (TLS for enclave communication)
- **Key Compromise**: If TEE keys are compromised, the system can detect and respond
- **Chain Reorgs**: The system handles chain reorganizations through reorg detection

### **Performance Characteristics**

- **Verification Time**: TEE verification adds latency but ensures security
- **Challenge Duration**: Configurable challenge windows (typically 10 mins)
- **Gas Costs**: On-chain operations incur gas costs for assertion submission
- **Throughput**: Limited by TEE verification speed and on-chain gas limits

### **User Considerations**

- **Withdrawal Timing**: Withdrawals require waiting for challenge window completion
- **TEE Dependency**: System availability depends on TEE infrastructure
- **Cross-Chain**: Withdrawals involve multiple chains (L1, sequencing, settlement, appchain)

---

## 🧩 Overview of Components

### 🔹 Source Chains

**Role:** Provide the raw data needed for withdrawal verification.

- **L1 Chain**: Provides batch accumulators and bridge state via `eth_getProof`
- **Sequencing Chain**: Contains batch data and validation inputs via `eth_getBlockByNumber`, `eth_getLogs`, `eth_getProof`, `arbdebug_validationInputsAt`
- **Settlement Chain**: Contains delayed messages and bridge logs via `eth_getLogs`
- **Appchain**: Provides block headers and send roots via `arbdebug_validationInputsAt`

### 🔹 `Synd-Proposer`

**Role:** Orchestrates the full withdrawal flow.  
**Main responsibilities:**

- **Data Aggregation**: Collects data from all source chains
- **Trusted Input Construction**: Builds secure input for TEE verification
- **Assertion Management**: Orchestrates the complete withdrawal flow
- **Polling Loop**: Monitors chain state and triggers verification

**Data aggregation** from multiple sources:

- **L1 node:** `eth_getProof`
- **Settlement node:** `eth_getLogs`
- **Sequencing node:** `eth_getBlockByNumber`, `eth_getLogs`, `eth_getProof`, `arbdebug_validationInputsAt`
- **Appchain node:** `arbdebug_validationInputsAt`

**Workflow:**

1. Constructs a `PendingAssertion` using collected data.
2. Invokes the secure TEE module to verify the assertion.
3. Submits the verified assertion on-chain via:
   - `submitPendingAssertion` → `TEEModule.sol`

### 🔹 `Synd-Enclave`

**Role:** Secure enclave runtime that performs the core logic for withdrawal proof validation.  
**Codebase:** Forked from [base/op-enclave](https://github.com/base/op-enclave)

**Main responsibilities:**

- **Secure Verification**: Performs cryptographic verification in TEE
- **Sequencing Chain Verification**: Validates batch data and accumulators
- **Appchain Verification**: Validates block headers and send roots
- **Signature Generation**: Creates cryptographically signed assertions

#### 🚀 Running the Enclave

There are two supported execution modes:

##### ➤ Local Execution (Development Mode)

1. Build the enclave binary:
   ```bash
   go build -C cmd/enclave
   ```
2. Run the enclave:
   ```bash
   ./cmd/enclave/enclave [--local]
   ```

### 🔹 On-Chain Contracts

**Role:** Manage the assertion lifecycle and provide security guarantees.

#### `TeeModule`

- **Assertion Management**: Manages assertion lifecycle and challenge system
- **Challenge System**: Handles disputes during challenge windows
- **State Transitions**: Manages trusted input updates and state transitions

#### `TeeKeyManager`

- **TEE Key Validation**: Validates TEE signatures and public keys
- **Public Key Management**: Manages authorized TEE public keys
- **Attestation Verification**: Verifies TEE attestation documents

#### `AssertionPoster`

- **Assertion Posting**: Posts verified assertions to rollups
- **Rollup Integration**: Integrates with Arbitrum rollup contracts
- **State Updates**: Updates rollup state to enable withdrawals

### 🔹 `Synd-Tee-Attestation-ZK-Proofs`

**Role:** Handles generation and verification of zk-SNARKs for attestation documents produced by `synd-enclave`.

### Security Features

1. **TEE-Based Verification**: All critical verification happens in secure enclave
2. **Cryptographic Signatures**: All assertions are signed by verified TEE instances
3. **Challenge System**: Disputes can be raised during challenge windows
4. **Hack Detection**: Multiple conflicting assertions trigger security measures
5. **Attestation Verification**: TEE attestation documents are cryptographically verified

### Data Flow

1. **Data Collection** → **Trusted Input** → **TEE Verification**
2. **Sequencing Verification** → **Appchain Verification** → **Assertion Creation**
3. **Signature Validation** → **Challenge Window** → **State Update**

---

## 🐳 Docker

To build a Docker image for the `synd-enclave` in a compatible environment:

```bash
docker build -f synd-withdrawals/synd-enclave/Dockerfile . --platform linux/amd64
```

---
