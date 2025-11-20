import { arbConfigManagerABI } from "@/abi/synd/ArbConfigManager"
import type { CreateArbChainConfig } from "@/types"
import { supportedSettlementChains } from "@/utils/constants"
import { getChainExplorerUrl } from "@/utils/helpers"
import { print } from "@/utils/print"
import { parseEventLogs } from "viem"

export async function createArbChainConfig({
  coreContracts,
  settlementStartBlock,
  sequencingContract,
  sequencingStartBlock,
  ownerSettlementWalletClient,
  settlementPublicClient,
  sequencingPublicClient,
  appchainExplorer,
  chainId,
  deployerSettlementWalletClient
}: CreateArbChainConfig) {
  print("🔍 Creating ArbChainConfig...")

  const arbConfigManagerAddress =
    supportedSettlementChains[settlementPublicClient.chain.id].arbConfigManager
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
      sequencingContract,
      // sequencingStartBlock
      BigInt(sequencingStartBlock),
      // initialAppchainOwner
      ownerSettlementWalletClient.account.address,
      // sequencingChainUrl
      // @note we are leaving blank for now as we require node operators to get their own private RPC URL
      "",
      // appchainBlockExplorerUrl
      appchainExplorer
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
