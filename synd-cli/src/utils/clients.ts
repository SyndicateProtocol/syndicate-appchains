import type { PublicClientWithChain } from "@/types"
import {
  type Chain,
  createPublicClient,
  createWalletClient,
  defineChain,
  type Hex,
  hexToNumber,
  http
} from "viem"
import { privateKeyToAccount } from "viem/accounts"
import {
  supportedEthereumChains,
  supportedSequencingChains,
  supportedSettlementChains
} from "./constants"

export async function getAppchainClients(rpcUrl: string, pks: Array<Hex> = []) {
  const chainId = await getChainIdFromRpc(rpcUrl)
  const publicClient = createPublicClient({
    chain: defineChain({
      id: chainId,
      name: `appchain: ${chainId}`,
      nativeCurrency: {
        name: "Ether",
        symbol: "ETH",
        decimals: 18
      },
      rpcUrls: {
        default: { http: [rpcUrl] },
        public: { http: [rpcUrl] }
      }
    }),
    transport: http(rpcUrl)
  })

  const walletClients = pks.map((pk) =>
    createWalletClient({
      chain: publicClient.chain,
      account: privateKeyToAccount(pk),
      transport: http(publicClient.transport.url)
    })
  )
  return [publicClient, walletClients] as const
}

export async function getSupportedChainClients(
  rpc: string,
  pks: Array<Hex> = []
) {
  const publicClient = await getSupportedChainPublicClient(rpc)
  const walletClients = await Promise.all(
    pks.map((pk) => getSupportedChainWalletClient(rpc, pk))
  )
  return [publicClient, walletClients] as const
}

export async function getSupportedChainPublicClient(
  rpcUrl: string
): Promise<PublicClientWithChain> {
  const chain = await getSupportedChainFromRpcUrl(rpcUrl)
  return createPublicClient({
    chain,
    transport: http(rpcUrl)
  })
}

async function getSupportedChainWalletClient(rpcUrl: string, privateKey: Hex) {
  const chain = await getSupportedChainFromRpcUrl(rpcUrl)
  return createWalletClient({
    chain,
    account: privateKeyToAccount(privateKey),
    transport: http(rpcUrl)
  })
}

async function getSupportedChainFromRpcUrl<
  _T extends Record<string, { chain: Chain }>
>(rpcUrl: string): Promise<Chain> {
  const chains = {
    ...supportedEthereumChains,
    ...supportedSequencingChains,
    ...supportedSettlementChains
  }
  const supportedChainIds = Object.keys(chains)
  const chainId = await getChainIdFromRpc(rpcUrl)
  if (supportedChainIds.includes(chainId.toString())) {
    return chains[chainId].chain
  }

  throw new Error(
    `Could not resolve chain for RPC URL: ${rpcUrl} (chainId: ${chainId})`
  )
}

async function getChainIdFromRpc(rpcUrl: string) {
  const res = await fetch(rpcUrl, {
    method: "POST",
    body: JSON.stringify({
      jsonrpc: "2.0",
      method: "eth_chainId",
      params: [],
      id: 1
    }),
    headers: {
      "Content-Type": "application/json"
    }
  })
  if (!res.ok) {
    throw new Error(`Failed to get chainId for ${rpcUrl}`)
  }
  const chainId = await res.json()
  return hexToNumber(chainId.result)
}
