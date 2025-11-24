import type { CheckTokenBridge, PublicClientWithChain } from "@/types"
import { getTokenBridgeContracts } from "@/utils/getTokenBridgeContracts"
import { print } from "@/utils/print"
import { createTokenBridgePrepareTransactionReceipt } from "@arbitrum/orbit-sdk"
import {
  type ArbitrumNetwork,
  getArbitrumNetworkInformationFromRollup,
  registerCustomArbitrumNetwork
} from "@arbitrum/sdk"
import type { JsonRpcProvider } from "@ethersproject/providers"
import { providers } from "ethers"
import { stringify } from "viem"

export async function checkTokenBridge({
  rollup,
  appchainPublicClient,
  settlementPublicClient,
  createdAtHash
}: CheckTokenBridge) {
  registerNewNetwork(
    publicClientToProvider(settlementPublicClient),
    publicClientToProvider(appchainPublicClient),
    rollup,
    settlementPublicClient.chain.testnet ?? false
  )

  const transaction = await settlementPublicClient.getTransaction({
    hash: createdAtHash
  })
  print("Bridge created at transaction", stringify(transaction, null, 2))

  const tokenBridgeCreatorAddress = transaction?.to
  if (!tokenBridgeCreatorAddress) {
    throw new Error("Token bridge creator address not found")
  }

  const txReceipt = createTokenBridgePrepareTransactionReceipt(
    await settlementPublicClient.waitForTransactionReceipt({
      hash: createdAtHash
    })
  )

  print("Waiting for retryable tickets to execute on the Orbit chain...")
  const orbitChainRetryableReceipts = await txReceipt.waitForRetryables({
    orbitPublicClient: appchainPublicClient
  })
  print("Retryables executed")
  print(
    "Transaction hash for first retryable",
    orbitChainRetryableReceipts[0].transactionHash
  )
  print(
    "Transaction hash for second retryable",
    orbitChainRetryableReceipts[1].transactionHash
  )

  // fetching the TokenBridge contracts
  const tokenBridgeContracts = await getTokenBridgeContracts({
    bridgeCreationHash: createdAtHash,
    parentChainPublicClient: settlementPublicClient,
    tokenBridgeCreatorAddressOverride: tokenBridgeCreatorAddress
  })
  print("TokenBridge contracts fetched")
  print(stringify(tokenBridgeContracts, null, 2))
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

export function publicClientToProvider(publicClient: PublicClientWithChain) {
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
