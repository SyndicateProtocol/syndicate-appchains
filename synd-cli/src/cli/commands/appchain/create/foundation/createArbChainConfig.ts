import {
  BeaconProxyABI,
  BeaconProxyBytecode
} from "@/abi/openzeppelin/BeaconProxy"
import { arbChainConfigABI } from "@/abi/synd/ArbChainConfig"
import type { CreateArbChainConfig } from "@/types"
import { supportedSettlementChains } from "@/utils/constants"
import { getChainExplorerUrl } from "@/utils/helpers"
import { print } from "@/utils/print"
import { encodeFunctionData } from "viem"

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
  print("🔍 Deploying and initializing ArbChainConfig BeaconProxy...")

  const beaconAddress =
    supportedSettlementChains[settlementPublicClient.chain.id]
      .arbChainConfigBeacon
  if (
    beaconAddress === "0x0000000000000000000000000000000000000000" ||
    !beaconAddress
  ) {
    throw new Error(
      `ArbChainConfig beacon not deployed on ${settlementPublicClient.chain.name}. Could not get beacon address, it needs to be deployed for this chain first.`
    )
  }

  // Encode the initialize call to pass to BeaconProxy constructor
  // This ensures deploy + initialize happens atomically (no front-running)
  const initializeData = encodeFunctionData({
    abi: arbChainConfigABI,
    functionName: "initialize",
    args: [
      // _owner
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
      // sequencingChainWsRpcUrl
      // @note we are leaving blank for now as we require node operators to get their own private RPC URL
      "",
      // appchainBlockExplorerUrl
      appchainExplorer
    ]
  })

  // Deploy the BeaconProxy with initialization data
  // The proxy constructor will delegatecall initialize() to the implementation
  const deployHash = await deployerSettlementWalletClient.deployContract({
    abi: BeaconProxyABI,
    bytecode: BeaconProxyBytecode,
    args: [beaconAddress, initializeData]
  })
  const deployReceipt = await settlementPublicClient.waitForTransactionReceipt({
    hash: deployHash
  })
  const arbChainConfigAddress = deployReceipt.contractAddress
  if (!arbChainConfigAddress) {
    throw new Error("ArbChainConfig BeaconProxy deployment failed")
  }
  print(
    `🔍 ArbChainConfig deployed and initialized at ${arbChainConfigAddress}\n${getChainExplorerUrl(
      settlementPublicClient.chain
    )}/tx/${deployHash}`
  )

  return arbChainConfigAddress
}
