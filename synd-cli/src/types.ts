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

export type GetChainsResponse = Array<{
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
}>
export type PublicClientWithChain = PublicClient<Transport, Chain, undefined>

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
  chainOwner: Hex
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
    rollupCreator: Hex
    tokenBridgeCreator: Hex
    arbConfigManager: Hex
    teeKeyManager: Hex
    chain: Chain
  }
>

export type SupportedSequencingChains = Record<
  number,
  {
    bridge: Hex
    requireAndFactory: Hex
    syndicateFactory: Hex
    chain: Chain
  }
>

export type SupportedEthereumChains = Record<
  number,
  {
    chain: Chain
  }
>

export type PrivateKeyWalletAccount = WalletClient<
  Transport,
  Chain,
  PrivateKeyAccount
>

export interface Foundation {
  chainId: number
  chainName: string
  nativeToken: Hex
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
  coreContractsCreatedAtHash?: Hex
  appchainRpc: string
  appchainExplorer: string
}

export interface DeployNitroRollupParams {
  chainId: number
  chainName: string
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
  appchainRpc: string
  appchainExplorer: string
  nativeToken: Hex
  deployerSettlementWalletClient: PrivateKeyWalletAccount
}

export interface CreateRollupParams {
  chainId: number
  nativeToken: Hex
  deployerSettlementWalletClient: PrivateKeyWalletAccount
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
}

export interface CreateArbChainConfig {
  coreContracts: CoreContracts
  settlementStartBlock: bigint | string
  sequencingContract: Hex
  sequencingStartBlock: bigint | string
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
  sequencingPublicClient: PublicClientWithChain
  appchainExplorer: string
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
  requireAndModule: Hex
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
  requireAndModule: Hex
  allowlistSequencingModule: Hex
  deployerSequencingWalletClient: PrivateKeyWalletAccount
  sequencingPublicClient: PublicClientWithChain
}

export interface TransferAllContractsOwnershipParams {
  sequencingContract: Hex
  allowlistSequencingModule: Hex
  requireAndModule: Hex
  deployerSequencingWalletClient: PrivateKeyWalletAccount
  sequencingPublicClient: PublicClientWithChain
  ownerSequencingWalletClient: PrivateKeyWalletAccount
}

export interface DeployTeeModule {
  settlementPublicClient: PublicClientWithChain
  deployerSettlementWalletClient: PrivateKeyWalletAccount
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  sequencingContract: Hex
  sequencingPublicClient: PublicClientWithChain
  appchainPublicClient: PublicClientWithChain
  ethereumPublicClient: PublicClientWithChain
  syndForkSequencingRpc: string
  coreContracts: { rollup: Hex; upgradeExecutor: Hex; bridge: Hex }
}

export interface Features {
  coreContracts: CoreContracts
  chainId: number
  chainName: string
  appchainPublicClient: PublicClientWithChain
  deployerSequencingWalletClient: PrivateKeyWalletAccount
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  deployerSettlementWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
  deployerAppchainWalletClient: PrivateKeyWalletAccount
  sequencingContract: Hex
  sequencingPublicClient: PublicClientWithChain
  ethereumPublicClient: PublicClientWithChain
  syndForkSequencingRpc: string
}

export interface DeployMulticall3 {
  appchainPublicClient: PublicClientWithChain
  deployerAppchainWalletClient: PrivateKeyWalletAccount
}

export interface CanDeployMulticall3 {
  appchainPublicClient: PublicClientWithChain
  deployerAppchainWalletClient: PrivateKeyWalletAccount
}

export interface CreateTeeModule {
  assertionPoster: Hex
  bridge: Hex
  deployerSettlementWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
  sequencingContract: Hex
  sequencingPublicClient: PublicClientWithChain
  appchainPublicClient: PublicClientWithChain
  ethereumPublicClient: PublicClientWithChain
  syndForkSequencingRpc: string
}

interface Synd {
  config: {
    arbConfigManager: Hex
    arbChainConfig: Hex
  }
  bridge: {
    chainInfo: {
      chainName: string
      chainId: number
      chainOwner: Hex
      minL2BaseFee: number
      parentChainId: number
      nativeToken: Hex
      staker?: string
      batchPoster?: string
      networkFeeReceiver?: string
      infrastructureFeeCollector?: string
      explorerUrl: string
      rpcUrl: string
    }
    coreContracts: {
      rollup: Hex
      inbox: Hex
      nativeToken: Hex
      outbox: Hex
      rollupEventInbox: Hex
      challengeManager: Hex
      adminProxy: Hex
      sequencerInbox: Hex
      bridge: Hex
      upgradeExecutor: Hex
      validatorUtils?: Hex
      validatorWalletCreator: Hex
      deployedAtBlockNumber: number
    }
    tokenBridgeContracts: {
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
  }
  sequencing: {
    syndicateSequencingChain: Hex
    allowlistSequencingModule: Hex
    requireAndModule: Hex
    settlementBlockBeforeDeployment: string
    deployedAtBlock: string
  }
  withdrawals: {
    teeKeyManager: Hex
    assertionPoster: Hex
    teeModule: Hex
    attestationDocVerifier: Hex
  }
}

export interface Handoff {
  newOwner: Hex
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  ownerSequencingWalletClient: PrivateKeyWalletAccount
  ownerAppchainWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
  sequencingPublicClient: PublicClientWithChain
  appchainPublicClient: PublicClientWithChain
  synd: Synd
}

export interface SetAppchainConfig {
  appchainPublicClient: PublicClientWithChain
  ownerAppchainWalletClient: PrivateKeyWalletAccount
  newOwner: Hex
}

export interface HandoffNitro {
  newOwner: Hex
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
  synd: Synd
  ownerAppchainWalletClient: PrivateKeyWalletAccount
  appchainPublicClient: PublicClientWithChain
}

export interface HandoffSynd {
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
  synd: Synd
  newOwner: Hex
  ownerSequencingWalletClient: PrivateKeyWalletAccount
  sequencingPublicClient: PublicClientWithChain
}

export interface E2E {
  inbox: Hex
  privateKey: Hex
  appchainPublicClient: PublicClientWithChain
  appchainWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
  settlementWalletClient: PrivateKeyWalletAccount
}

export interface Deposit {
  settlementPublicClient: PublicClientWithChain
  settlementWalletClient: PrivateKeyWalletAccount
  appchainPublicClient: PublicClientWithChain
  inbox: Hex
  account: Account
  value: bigint
}

export interface DeployCounter {
  appchainPublicClient: PublicClientWithChain
  appchainWalletClient: PrivateKeyWalletAccount
}

export interface TransferToSelf {
  appchainWalletClient: PrivateKeyWalletAccount
  appchainPublicClient: PublicClientWithChain
  value: bigint
}

export interface CheckSequencerInbox {
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
  synd: Synd
}

export interface CallArbOwner {
  settlementPublicClient: PublicClientWithChain
  appchainPublicClient?: PublicClientWithChain
  settlementUpgradeExecutor: Hex
  settlementInbox: Hex
  appchainUpgradeExecutor: Hex
  refundAddress: Hex
  gasLimit?: bigint
  maxFeePerGas?: bigint
  functionName: string
  calldata: Hex
}

export interface CheckTokenBridge {
  rollup: Hex
  appchainPublicClient: PublicClientWithChain
  settlementPublicClient: PublicClientWithChain
  createdAtHash: Hex
}
