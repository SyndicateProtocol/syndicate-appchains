import type { CoreContracts } from "@arbitrum/orbit-sdk"
import type {
  Account,
  Chain,
  Hex,
  PrivateKeyAccount,
  PublicClient,
  Transport,
  WalletClient
} from "viem"

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
  chainId: number
  chainName: string
  ownerSettlementWalletClient: WalletClient<Transport, Chain>
  settlementPublicClient: PublicClientWithChain
  appChainRpcUrl: string
  appChainExplorerUrl: string
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
    chain: Chain
  }
>

export type SupportedEthereumChains = Record<
  number,
  {
    chain: Chain
  }
>

export type PrivateKeyWalletAccount = WalletClient<Transport, Chain, PrivateKeyAccount>

export interface Foundation {
  chainId: number
  chainName: string
  nativeTokenAddress: Hex
  deployerSettlementWalletClient: PrivateKeyWalletAccount
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  ownerSequencingWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
  sequencingPublicClient: PublicClientWithChain
  ethereumChainRpcUrl: string
  deployerSequencingWalletClient: WalletClient<
    Transport,
    Chain,
    PrivateKeyAccount
  >
  ownerPrivateKey: Hex
  coreContractsCreatedAtHash: Hex
  appChainRpcUrl: string
  appChainExplorerUrl: string
}

export interface DeployNitroRollupParams {
  chainId: number
  chainName: string
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
  appChainRpcUrl: string
  appChainExplorerUrl: string
  nativeTokenAddress: Hex
  deployerSettlementWalletClient: PrivateKeyWalletAccount
}

export interface CreateRollupParams {
  chainId: number
  nativeTokenAddress: Hex
  deployerSettlementWalletClient: PrivateKeyWalletAccount
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
}

export interface CreateArbChainConfig {
  coreContracts: CoreContracts
  settlementStartBlock: bigint | string
  sequencingContractAddress: Hex
  sequencingStartBlock: bigint | string
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
  sequencingPublicClient: PublicClientWithChain
  appChainExplorerUrl: string
  chainId: number
  deployerSettlementWalletClient: PrivateKeyWalletAccount
}

export interface DeploySequencingChainParams {
  sequencerAccount: Account
  chainId: number
  sequencingPublicClient: PublicClientWithChain
  deployerSequencingWalletClient: PrivateKeyWalletAccount
  ownerSequencingWalletClient: PrivateKeyWalletAccount
}

export interface CreateRequireAndModuleParams {
  chainId: number
  sequencingPublicClient: PublicClientWithChain
  deployerSequencingWalletClient: PrivateKeyWalletAccount
}

export interface CreateSyndicateSequencingChainParams {
  requireAndModuleAddress: Hex
  sequencingPublicClient: PublicClientWithChain
  deployerSequencingWalletClient: PrivateKeyWalletAccount
  chainId: number
}

export interface DeployAndSetupAllowlistSequencingModuleParams {
  sequencerAccount: Account
  sequencingPublicClient: PublicClientWithChain
  deployerSequencingWalletClient: PrivateKeyWalletAccount
}

export interface RegisterAllowlistSequencingModuleOnRequireAllModuleParams {
  requireAndModuleAddress: Hex
  allowlistSequencingModuleAddress: Hex
  deployerSequencingWalletClient: PrivateKeyWalletAccount
  sequencingPublicClient: PublicClientWithChain
}

export interface TransferAllContractsOwnershipParams {
  syndicateSequencingChainAddress: Hex
  allowlistSequencingModuleAddress: Hex
  requireAndModuleAddress: Hex
  deployerSequencingWalletClient: PrivateKeyWalletAccount
  sequencingPublicClient: PublicClientWithChain
  ownerSequencingWalletClient: PrivateKeyWalletAccount
}

export interface DeployTeeModuleParams {
  settlementPublicClient: PublicClientWithChain
  deployerSettlementWalletClient: PrivateKeyWalletAccount
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  sequencingContractAddress: Hex
  sequencingPublicClient: PublicClientWithChain
  appchainPublicClient: PublicClientWithChain
  ethereumPublicClient: PublicClientWithChain
  syndForkSequencingChainRpcUrl: string
  coreContracts: { rollup: Hex; upgradeExecutor: Hex; bridge: Hex }
}
