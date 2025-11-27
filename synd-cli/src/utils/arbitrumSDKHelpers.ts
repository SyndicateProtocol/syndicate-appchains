import type { PublicClientWithChain } from "@/types"
import {
  type ArbitrumNetwork,
  getArbitrumNetworkInformationFromRollup,
  registerCustomArbitrumNetwork
} from "@arbitrum/sdk"
import type { JsonRpcProvider } from "@ethersproject/providers"
import { StaticJsonRpcProvider } from "@ethersproject/providers"

export const registerNetworkInArbSDK = async (
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

  return new StaticJsonRpcProvider(url, network)
}
