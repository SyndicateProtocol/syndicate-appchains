import { teeKeyManagerABI } from "@/scripts/abi/synd/TeeKeyManager"
import { teeModuleABI, teeModuleBytecode } from "@/scripts/abi/synd/TeeModule"
import { sleep } from "bun"
import {
  type Hex,
  type PublicClient,
  encodePacked,
  keccak256,
  parseAbi,
  stringify
} from "viem"
import { holesky, mainnet, sepolia } from "viem/chains"
import { AbsBridgeAbi } from "../abi/nitro/AbsBridge"
import { getFeaturesConfig } from "../utils/config"
import {
  supportedSequencingChains,
  supportedSettlementChains
} from "../utils/constants"
import { getChainExplorerUrl } from "../utils/helpers"
import { print } from "../utils/print"
import { createAssertionPoster } from "./createAssertionPoster"

export async function deployTeeModule() {
  const { ownerSettlementWalletClient, settlementPublicClient, deployerSettlementWalletClient } =
    await getFeaturesConfig()
  const assertionPosterAddress = await createAssertionPoster()
  const teeModuleAddress = await createTeeModule(assertionPosterAddress)

  // Sleep a bit to ensure contracts are ready to have ownership transferred
  await sleep(2_000)

  // AssertionPoster should be owned by the TeeModule
  const assertionPosterOwnershipTransferHash =
    await transferOwnershipFromDeployer(
      assertionPosterAddress,
      teeModuleAddress
    )
  print(
    `🔍  AssertionPoster ownership transferred to TeeModule (${teeModuleAddress})\n${getChainExplorerUrl(
      settlementPublicClient.chain
    )}/tx/${assertionPosterOwnershipTransferHash}`
  )

  // TeeModule's DEFAULT_ADMIN_ROLE should be set to the owner
  await transferTeeModuleDefaultAdminRole(teeModuleAddress)

  // Read attestation doc verifier address from TeeKeyManager
  const teeKeyManagerAddress =
    supportedSettlementChains[settlementPublicClient.chain.id]
      .teeKeyManagerAddress

  const attestationDocVerifierAddress =
    await settlementPublicClient.readContract({
      address: teeKeyManagerAddress,
      abi: teeKeyManagerABI,
      functionName: "attestationDocVerifier"
    })

  return {
    assertionPosterAddress,
    teeModuleAddress,
    attestationDocVerifierAddress,
    teeKeyManagerAddress
  }
}

export async function createTeeModule(assertionPosterAddress: Hex) {
  const {
    coreContracts,
    deployerSettlementWalletClient,
    settlementPublicClient,
    sequencingContractAddress,
    sequencingPublicClient,
    appchainPublicClient,
    ethereumPublicClient,
    syndForkSequencingChainRpcUrl
  } = await getFeaturesConfig()

  const isL1Chain = (
    [mainnet.id, sepolia.id, holesky.id] as Array<number>
  ).includes(settlementPublicClient.chain.id)

  const bridgeAddress =
    supportedSequencingChains[sequencingPublicClient.chain.id].bridgeAddress
  const teeKeyManagerAddress =
    supportedSettlementChains[settlementPublicClient.chain.id]
      .teeKeyManagerAddress

  const l1BlockOrBridge = isL1Chain
    ? bridgeAddress
    : "0x4200000000000000000000000000000000000015"

  const settlementDelay = BigInt(60) // seconds
  const challengeWindowDuration = BigInt(10 * 60) // 10 minutes
  const slowDuration = BigInt(7 * 24 * 60 * 60) // 7 days

  const sequencerMessageCount = await ethereumPublicClient.readContract({
    address: bridgeAddress,
    abi: AbsBridgeAbi,
    functionName: "sequencerMessageCount"
  })

  // -2 because the l1Block from the TeeModule is 5 minutes behind the L1 chain
  // starting from a previous batch in case the last batch was posted in the last 5 minutes
  const batchNumber = sequencerMessageCount - BigInt(2)

  const l1StartBatchAcc = await ethereumPublicClient.readContract({
    address: bridgeAddress,
    abi: AbsBridgeAbi,
    functionName: "sequencerInboxAccs",
    args: [batchNumber]
  })

  const res = await fetch(
    syndForkSequencingChainRpcUrl,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        jsonrpc: "2.0",
        method: "synd_batchMetadata",
        params: [Number(batchNumber)],
        id: 1
      })
    }
  )
  if (!res.ok) {
    throw new Error("Failed to get sequencing batch metadata")
  }
  const data = await res.json()
  const sequencingStartBlockNumber = data.result.MessageCount - 1
  const seqStartBlockHash = (
    await sequencingPublicClient.getBlock({
      blockNumber: BigInt(sequencingStartBlockNumber)
    })
  ).hash

  const appStartBlockHash = await findAppStartBlockHash(
    appchainPublicClient,
    BigInt(sequencingStartBlockNumber)
  )

  const args = [
    // 1. poster_: AssertionPoster contract address
    assertionPosterAddress,
    // 2. bridge_: Nitro Bridge contract address
    coreContracts.bridge,
    // 3. configHash_: Hash of the configuration data passed to the TEE
    // keccak256(abi.encodePacked(seqContract, seqBridge, setDelay))
    keccak256(
      encodePacked(
        ["address", "address", "uint64"],
        [
          sequencingContractAddress, // SyndicateSequencingChain contract address
          bridgeAddress, // Arb Bridge contract address for sequencing chain
          settlementDelay
        ]
      )
    ),
    // 4. appStartBlockHash_: Starting block hash of the appchain
    appStartBlockHash,
    // 5. seqStartBlockHash_: Starting block hash of the sequencing chain
    seqStartBlockHash,
    // 6. l1StartBatchAcc_: Sequencing chain start batch accumulator
    l1StartBatchAcc,
    // 7. l1BlockOrBridge_: Address of the l1 block contract - 0x4200000000000000000000000000000000000015 for bedrock rollups - or the l1 <-> sequencing chain bridge if the settlement chain is the same as the l1 chain.
    l1BlockOrBridge,
    // 8. isL1Chain_: True if l1BlockOrBridge is the bridge address and false if it is the l1 block contract address instead
    isL1Chain,
    // 9. challengeWindowDuration_: The duration of the challenge window in seconds
    challengeWindowDuration,
    // 10. slowDuration_: The duration of the challenge window in slow mode in seconds.
    slowDuration,
    // 10. teeKeyManager_: The address of the TEE key manager contract
    // Note that the AssertionPoster must be owned by the TeeModule for closing the challenge window to work properly
    teeKeyManagerAddress
  ] as const

  print(`🔍  Deploying TeeModule with args: ${stringify(args)}`)

  const teeModuleHash = await deployerSettlementWalletClient.deployContract({
    abi: teeModuleABI,
    bytecode: teeModuleBytecode,
    args
  })
  const teeModuleReceipt =
    await settlementPublicClient.waitForTransactionReceipt({
      hash: teeModuleHash
    })
  const teeModuleAddress = teeModuleReceipt.contractAddress
  if (!teeModuleAddress) {
    throw new Error("TeeModule deployment failed")
  }
  print(
    `🔍  Deployed TeeModule to ${teeModuleAddress} in ${getChainExplorerUrl(
      settlementPublicClient.chain
    )}/tx/${teeModuleHash}`
  )
  return teeModuleAddress
}

export async function transferOwnershipFromDeployer(
  address: Hex,
  newOwner: Hex
) {
  const { settlementPublicClient, deployerSettlementWalletClient } =
    await getFeaturesConfig()

  print(`🔍  Transferring ownership of ${address} to ${newOwner}`)

  const owner = await settlementPublicClient.readContract({
    address: address,
    abi: parseAbi(["function owner() public view returns (address)"]),
    functionName: "owner"
  })

  if (owner !== deployerSettlementWalletClient.account.address) {
    throw new Error(
      `Current owner of ${address} is not the deployer. Current owner: ${owner}. Deployer: ${deployerSettlementWalletClient.account.address}`
    )
  }

  const transferOwnershipHash =
    await deployerSettlementWalletClient.writeContract({
      address: address,
      abi: parseAbi(["function transferOwnership(address newOwner) public"]),
      functionName: "transferOwnership",
      args: [newOwner]
    })

  const transferReceipt =
    await settlementPublicClient.waitForTransactionReceipt({
      hash: transferOwnershipHash
    })

  if (transferReceipt.status !== "success") {
    throw new Error("AssertionPoster ownership transfer to TeeModule failed")
  }
  return transferOwnershipHash
}

async function transferTeeModuleDefaultAdminRole(teeModuleAddress: Hex) {
  const { ownerSettlementWalletClient, settlementPublicClient, deployerSettlementWalletClient } =
    await getFeaturesConfig()

  const defaultAdminRole = await settlementPublicClient.readContract({
    address: teeModuleAddress,
    abi: teeModuleABI,
    functionName: "DEFAULT_ADMIN_ROLE"
  })
  const teeModuleDefaultAdminRoleTransferHash = await deployerSettlementWalletClient.writeContract({
    address: teeModuleAddress,
    abi: teeModuleABI,
    functionName: "grantRole",
    args: [defaultAdminRole, ownerSettlementWalletClient.account.address]
  })
  await settlementPublicClient.waitForTransactionReceipt({
    hash: teeModuleDefaultAdminRoleTransferHash
  })
  print(
    `🔍  TeeModule's DEFAULT_ADMIN_ROLE set to owner (${
      ownerSettlementWalletClient.account.address
    })\n${getChainExplorerUrl(
      settlementPublicClient.chain
    )}/tx/${teeModuleDefaultAdminRoleTransferHash}`
  )

  // Revoke the DEFAULT_ADMIN_ROLE from the deployer
  const teeModuleDefaultAdminRoleRevokeHash = await deployerSettlementWalletClient.writeContract({
    address: teeModuleAddress,
    abi: teeModuleABI,
    functionName: "revokeRole",
    args: [defaultAdminRole, deployerSettlementWalletClient.account.address]
  })
  await settlementPublicClient.waitForTransactionReceipt({
    hash: teeModuleDefaultAdminRoleRevokeHash
  })
  print(
    `🔍  TeeModule's DEFAULT_ADMIN_ROLE revoked from deployer (${
      deployerSettlementWalletClient.account.address
    })\n${getChainExplorerUrl(
      settlementPublicClient.chain
    )}/tx/${teeModuleDefaultAdminRoleRevokeHash}`
  )
}

async function findAppStartBlockHash(
  appchainPublicClient: PublicClient,
  l1Number: bigint
) {
  let start = BigInt(0)
  let end = await appchainPublicClient.getBlockNumber()
  end = end + BigInt(1)

  while (end - start > BigInt(1)) {
    const mid = (start + end) / BigInt(2)
    const header = await appchainPublicClient.getBlock({ blockNumber: mid })
    // @ts-ignore
    const l1NumberFromHeader = header.l1BlockNumber

    if (l1NumberFromHeader <= l1Number) {
      start = mid
    } else {
      end = mid
    }
  }

  const finalHeader = await appchainPublicClient.getBlock({
    blockNumber: start
  })
  // @ts-ignore
  const finalL1Number = finalHeader.l1BlockNumber
  if (finalL1Number > l1Number) {
    throw new Error("Block not found")
  }

  return finalHeader.hash
}
