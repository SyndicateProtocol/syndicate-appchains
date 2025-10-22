import { createTokenBridgePrepareTransactionReceipt } from "@arbitrum/orbit-sdk"

import {
  type ArbitrumNetwork,
  getArbitrumNetworkInformationFromRollup,
  registerCustomArbitrumNetwork
} from "@arbitrum/sdk"
import type { JsonRpcProvider } from "@ethersproject/providers"
import { providers } from "ethers"
import type { Chain, PublicClient, Transport } from "viem"
import { getFeaturesConfig } from "../utils/config"

async function main() {
  const {
    coreContracts,
    chainId,
    chainName,
    appchainPublicClient,
    deployerSequencingWalletClient,
    ownerSettlementWalletClient,
    deployerSettlementWalletClient,
    settlementPublicClient
  } = await getFeaturesConfig()

  registerNewNetwork(
    publicClientToProvider(settlementPublicClient),
    publicClientToProvider(appchainPublicClient),
    coreContracts.rollup,
    ownerSettlementWalletClient.chain.testnet ?? false
  )

  const hash =
    "0x1dbb20c1b59fb372a472b04b90dbf25bfec95b46b34f378a6eedf14603bb55df"

  const transaction = await settlementPublicClient.getTransaction({
    hash
  })

  // get the transaction receipt after waiting for the transaction to complete
  const txReceipt = createTokenBridgePrepareTransactionReceipt(
    await settlementPublicClient.waitForTransactionReceipt({ hash })
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
  const tokenBridgeContracts = await txReceipt.getTokenBridgeContracts({
    // @ts-ignore (todo: fix viem type issue)
    parentChainPublicClient: settlementPublicClient
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
