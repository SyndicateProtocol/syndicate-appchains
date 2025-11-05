import { createTokenBridgePrepareTransactionReceipt } from "@arbitrum/orbit-sdk"

import {
  type ArbitrumNetwork,
  getArbitrumNetworkInformationFromRollup,
  registerCustomArbitrumNetwork
} from "@arbitrum/sdk"
import type { JsonRpcProvider } from "@ethersproject/providers"
import { providers } from "ethers"
import type { Chain, PublicClient, Transport } from "viem"
import { getTokenBridgeContracts } from "../features/getTokenBridgeContracts"
import { getFeaturesConfig } from "../utils/config"
import { supportedSettlementChains } from "../utils/constants"
import { generateBridgeConfig } from "../utils/generateBridgeConfig"
import { getChainRpcUrl, upsertToSyndObject } from "../utils/helpers"

async function main() {
  const {
    coreContracts,
    appchainPublicClient,
    ownerSettlementWalletClient,
    settlementPublicClient
  } = await getFeaturesConfig()

  registerNewNetwork(
    publicClientToProvider(settlementPublicClient),
    publicClientToProvider(appchainPublicClient),
    coreContracts.rollup,
    ownerSettlementWalletClient.chain.testnet ?? false
  )

  const tokenBridgeCreatedAtHash = "0x"

  const transaction = await settlementPublicClient.getTransaction({
    hash: tokenBridgeCreatedAtHash
  })
  console.log("Bridge created at transaction:", transaction)

  const txReceipt = createTokenBridgePrepareTransactionReceipt(
    await settlementPublicClient.waitForTransactionReceipt({ hash: tokenBridgeCreatedAtHash })
  )

  console.log("Waiting for retryable tickets to execute on the Orbit chain...")
  const orbitChainRetryableReceipts = await txReceipt.waitForRetryables({
    // @ts-ignore (todo: fix viem type issue)
    orbitPublicClient: appchainPublicClient
  })
  console.log("Retryables executed")
  console.log(
    `Transaction hash for first retryable is ${orbitChainRetryableReceipts[0].transactionHash}`
  )
  console.log(
    `Transaction hash for second retryable is ${orbitChainRetryableReceipts[1].transactionHash}`
  )

  // fetching the TokenBridge contracts
  const tokenBridgeContracts = await getTokenBridgeContracts({
    bridgeCreationHash: tokenBridgeCreatedAtHash,
    parentChainPublicClient: settlementPublicClient,
    tokenBridgeCreatorAddressOverride: supportedSettlementChains[settlementPublicClient.chain.id].tokenBridgeCreatorAddress
  })
  console.log("TokenBridge contracts fetched")
  console.log(tokenBridgeContracts)
}

export const registerNewNetwork = async (
  parentProvider: JsonRpcProvider,
  childProvider: JsonRpcProvider,
  rollupAddress: string,
  isTestnet: boolean
): Promise<ArbitrumNetwork> => {
  const chainId = (await childProvider.getNetwork()).chainId
  const { parentChainId, ethBridge, confirmPeriodBlocks } =
    await getArbitrumNetworkInformationFromRollup(rollupAddress, parentProvider)

  const arbitrumNetwork: ArbitrumNetwork = {
    name: String(`${chainId}-arbitrum-network`),
    chainId,
    parentChainId,
    confirmPeriodBlocks,
    ethBridge,
    isCustom: true,
    isTestnet
  }

  return registerCustomArbitrumNetwork(arbitrumNetwork)
}

export function publicClientToProvider<TChain extends Chain | undefined>(
  publicClient: PublicClient<Transport, TChain>
) {
  const { chain } = publicClient

  if (typeof chain === "undefined") {
    throw new Error(`[publicClientToProvider] "chain" is undefined`)
  }

  const network = {
    chainId: chain.id,
    name: chain.name,
    ensAddress: chain.contracts?.ensRegistry?.address
  }

  const transportUrl = publicClient.transport.url as string | undefined
  const url = transportUrl ?? chain.rpcUrls.default.http[0]

  return new providers.StaticJsonRpcProvider(url, network)
}

await main()

export { main }
