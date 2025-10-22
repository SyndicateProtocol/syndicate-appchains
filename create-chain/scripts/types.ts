import type { CoreContracts } from "@arbitrum/orbit-sdk"
import type { Chain, Hex, PublicClient, Transport } from "viem"

export type GetChainsResponse = Array<ChainIdNetworkChain>
export type PublicClientWithChain = PublicClient<Transport, Chain>

export interface ChainIdNetworkChain {
  name: string
  chain: string
  icon?: string
  rpc: string[]
  features: Array<{ name: string }>
  faucets: string[]
  nativeCurrency: {
    name: string
    symbol: string
    decimals: number
  }
  infoURL: string
  shortName: string
  chainId: number
  networkId: number
  slip44: number
  ens?: {
    registry: string
  }
  explorers: Array<{
    name: string
    url: string
    icon?: string
    standard: string
  }>
}

export interface CreateSettlementRollupParams {
  validators: Hex[]
  batchPosters: Hex[]
  batchPosterManager: Hex
}

export interface TokenContracts {
  l2Contracts: {
    router: Hex
    standardGateway: Hex
    customGateway: Hex
    wethGateway: Hex
    weth: Hex
    multicall: Hex
    proxyAdmin: Hex
  }
  l3Contracts: {
    router: Hex
    standardGateway: Hex
    customGateway: Hex
    wethGateway: Hex
    weth: Hex
    proxyAdmin: Hex
    beaconProxyFactory: Hex
    upgradeExecutor: Hex
    multicall: Hex
  }
}

export interface GenerateBridgeConfigParams {
  coreContracts: CoreContracts
  rpcUrl: string
  explorerUrl: string
  parentChainId: number
  tokenContracts?: TokenContracts
  chainName: string
  chainId: number
  rollupOwnerAddress: Hex
}

export interface CreateNodeConfigParams {
  chainName: string
  deploymentTxHash: Hex
  parentChainClient: PublicClientWithChain
  batchPosterPrivateKey?: Hex
  validatorPrivateKey?: Hex
}

export type ChainNativeCurrency = {
  name: string
  /** 2-6 characters long */
  symbol: string
  decimals: number
}

export type SupportedSettlementChains = Record<
  number,
  {
    rollupCreatorAddress: Hex
    tokenBridgeCreatorAddress: Hex
    arbConfigManagerAddress: Hex
    teeKeyManagerAddress: Hex
    chain: Chain
  }
>

export type SupportedSequencingChains = Record<
  number,
  {
    bridgeAddress: Hex
    requireAndFactoryAddress: Hex
    syndicateFactoryAddress: Hex
    syndNitroForkRpcUrl: string
    chain: Chain
  }
>

export type SupportedEthereumChains = Record<
  number,
  {
    chain: Chain
  }
>
