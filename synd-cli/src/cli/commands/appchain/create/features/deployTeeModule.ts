import { AbsBridgeAbi } from "@/abi/nitro/AbsBridge"
import { teeModuleABI, teeModuleBytecode } from "@/abi/synd/TeeModule"
import type { CreateTeeModule } from "@/types"
import {
  supportedSequencingChains,
  supportedSettlementChains
} from "@/utils/constants"
import { getChainExplorerUrl } from "@/utils/helpers"
import { print } from "@/utils/print"
import { encodePacked, keccak256, type PublicClient, stringify } from "viem"
import { holesky, mainnet, sepolia } from "viem/chains"

export async function deployTeeModule({
  assertionPosterAddress,
  bridge,
  deployerSettlementWalletClient,
  settlementPublicClient,
  sequencingContractAddress,
  sequencingPublicClient,
  appchainPublicClient,
  ethereumPublicClient,
  syndForkSequencingRpc
}: CreateTeeModule) {
  const isL1Chain = (
    [mainnet.id, sepolia.id, holesky.id] as Array<number>
  ).includes(settlementPublicClient.chain.id)

  const sequencingChainBridge =
    supportedSequencingChains[sequencingPublicClient.chain.id].bridgeAddress
  const teeKeyManagerAddress =
    supportedSettlementChains[settlementPublicClient.chain.id]
      .teeKeyManagerAddress

  const l1BlockOrBridge = isL1Chain
    ? sequencingChainBridge
    : "0x4200000000000000000000000000000000000015"

  const settlementDelay = BigInt(60) // seconds
  const challengeWindowDuration = BigInt(10 * 60) // 10 minutes
  const slowDuration = BigInt(7 * 24 * 60 * 60) // 7 days

  const sequencerMessageCount = await ethereumPublicClient.readContract({
    address: sequencingChainBridge,
    abi: AbsBridgeAbi,
    functionName: "sequencerMessageCount"
  })

  // -2 because the l1Block from the TeeModule is 5 minutes behind the L1 chain
  // starting from a previous batch in case the last batch was posted in the last 5 minutes
  const batchNumber = sequencerMessageCount - BigInt(2)

  const l1StartBatchAcc = await ethereumPublicClient.readContract({
    address: sequencingChainBridge,
    abi: AbsBridgeAbi,
    functionName: "sequencerInboxAccs",
    args: [batchNumber]
  })

  const res = await fetch(syndForkSequencingRpc, {
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
  })
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
    bridge,
    // 3. configHash_: Hash of the configuration data passed to the TEE
    // keccak256(abi.encodePacked(seqContract, seqBridge, setDelay))
    keccak256(
      encodePacked(
        ["address", "address", "uint64"],
        [
          sequencingContractAddress, // SyndicateSequencingChain contract address
          sequencingChainBridge, // Arb Bridge contract address for sequencing chain
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
    // @ts-expect-error
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
  // @ts-expect-error
  const finalL1Number = finalHeader.l1BlockNumber
  if (finalL1Number > l1Number) {
    throw new Error("Block not found")
  }

  return finalHeader.hash
}
