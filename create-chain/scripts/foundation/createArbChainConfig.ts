import { arbConfigManagerABI } from "@/scripts/abi/synd/ArbConfigManager"
import type { CoreContracts } from "@arbitrum/orbit-sdk"
import { type Hex, parseEventLogs } from "viem"
import { getFoundationConfig } from "../utils/config"
import { supportedSettlementChains } from "../utils/constants"
import { getChainExplorerUrl } from "../utils/helpers"
import { print } from "../utils/print"

export async function createArbChainConfig(
  coreContracts: CoreContracts,
  settlementStartBlock: bigint | string,
  sequencingContractAddress: Hex,
  sequencingStartBlock: bigint | string
) {
  const {
    chainId,
    chainName,
    deployerSettlementWalletClient,
    settlementPublicClient,
    sequencingPublicClient,
    ownerSettlementWalletClient,
    appChainExplorerUrl
  } = await getFoundationConfig()

  print("🔍 Creating ArbChainConfig...")

  const arbConfigManagerAddress =
    supportedSettlementChains[settlementPublicClient.chain.id]
      .arbConfigManagerAddress
  const { request } = await settlementPublicClient.simulateContract({
    account: deployerSettlementWalletClient.account,
    address: arbConfigManagerAddress,
    abi: arbConfigManagerABI,
    functionName: "createArbChainConfig",
    args: [
      // owner
      ownerSettlementWalletClient.account.address,
      // chainId
      BigInt(chainId),
      // sequencingChainId
      BigInt(sequencingPublicClient.chain.id),
      // arbitrumBridgeAddress
      coreContracts.bridge,
      // arbitrumInboxAddress
      coreContracts.inbox,
      // settlementDelay
      BigInt(60),
      // settlementStartBlock
      BigInt(settlementStartBlock),
      // sequencingContractAddress
      sequencingContractAddress,
      // sequencingStartBlock
      BigInt(sequencingStartBlock),
      // initialAppchainOwner
      ownerSettlementWalletClient.account.address,
      // sequencingChainUrl
      // @note we are leaving blank for now as we require node operators to get their own private RPC URL
      "",
      // appChainBlockExplorerUrl
      appChainExplorerUrl
    ]
  })
  const txHash = await deployerSettlementWalletClient.writeContract(request)
  const tx = await settlementPublicClient.waitForTransactionReceipt({
    hash: txHash
  })
  const creationLogs = parseEventLogs({
    abi: arbConfigManagerABI,
    logs: tx.logs
  })
  const arbChainConfigAddress = creationLogs.find(
    (l) => l.eventName === "ArbChainConfigCreated"
  )?.args.configAddress
  if (!arbChainConfigAddress) {
    throw new Error("ArbChainConfig deployment failed")
  }
  print(
    `🔍 ArbChainConfig deployed to ${arbChainConfigAddress}\n${getChainExplorerUrl(
      settlementPublicClient.chain
    )}/tx/${tx.transactionHash}`
  )
  return arbChainConfigAddress
}
