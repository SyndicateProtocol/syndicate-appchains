import type { ChainNativeCurrency, GetChainsResponse } from "@/types"
import {
  type Address,
  type Chain,
  type Hex,
  type PublicClient,
  erc20Abi,
  getAddress,
  parseAbiItem,
  zeroAddress
} from "viem"

export async function getDoesChainExist(chainId: number) {
  const res = await fetch("https://chainid.network/chains.json")
  const chains = (await res.json()) as GetChainsResponse
  const chain = chains.find((chain) => chain.chainId === chainId)
  if (chain) {
    console.debug(`Chain ${chainId} already exists`, chain)
    return true
  }
  return false
}

export function isNativeTokenEth(nativeTokenAddress?: string) {
  return nativeTokenAddress === undefined || nativeTokenAddress === zeroAddress
}

export async function getNativeCurrency(
  parentChainClient: PublicClient,
  tokenAddress: Hex
) {
  let nativeCurrency: ChainNativeCurrency
  if (isNativeTokenEth(tokenAddress)) {
    nativeCurrency = {
      decimals: 18,
      name: "Ether",
      symbol: "ETH"
    }
  } else {
    const [decimals, name, symbol] = await Promise.all([
      parentChainClient.readContract({
        address: tokenAddress,
        abi: erc20Abi,
        functionName: "decimals"
      }),
      parentChainClient.readContract({
        address: tokenAddress,
        abi: erc20Abi,
        functionName: "name"
      }),
      parentChainClient.readContract({
        address: tokenAddress,
        abi: erc20Abi,
        functionName: "symbol"
      })
    ])
    nativeCurrency = {
      decimals,
      name,
      symbol
    }
  }
  return nativeCurrency
}

export function getChainExplorerUrl(chain: Chain) {
  return chain.blockExplorers?.default.url
}

export function getChainRpcUrl(chain: Chain) {
  return chain.rpcUrls.default.http[0]
}

export function isAddressEq(
  address1: Address | Hex | string,
  address2: Address | Hex | string
) {
  return getAddress(address1) === getAddress(address2)
}

export function scaleByPercentage(
  value: bigint,
  percentIncrease: number | bigint
) {
  return value + (value * BigInt(percentIncrease)) / BigInt(100)
}

export async function detectCustomNativeToken(
  publicClient: PublicClient,
  inboxAddress: Address
) {
  try {
    const bridgeAddress = await publicClient.readContract({
      address: inboxAddress,
      abi: [parseAbiItem("function bridge() public view returns (address)")],
      functionName: "bridge"
    })

    return await getNativeTokenFromBridge(publicClient, bridgeAddress)
  } catch (_) {
    console.warn("⚠️  Could not detect native token, assuming ETH-native chain")
    return null
  }
}
export async function getNativeTokenFromBridge(
  publicClient: PublicClient,
  bridgeAddress: Address
) {
  try {
    const address = await publicClient.readContract({
      address: bridgeAddress,
      abi: [
        parseAbiItem("function nativeToken() public view returns (address)")
      ],
      functionName: "nativeToken"
    })
    const currency = await getNativeCurrency(publicClient, address)
    return {
      ...currency,
      address
    }
  } catch (_) {
    return null
  }
}
