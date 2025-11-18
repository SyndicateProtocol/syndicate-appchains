import { getAddress } from "viem"
import { z } from "zod"

const chainInfo = z.object({
  chainName: z.string(),
  chainId: z.number(),
  chainOwner: z.string().transform((val) => getAddress(val)),
  minL2BaseFee: z.number(),
  parentChainId: z.number(),
  nativeToken: z.string().transform((val) => getAddress(val)),
  staker: z.string().optional(),
  batchPoster: z.string().optional(),
  networkFeeReceiver: z.string().optional(),
  infrastructureFeeCollector: z.string().optional(),
  explorerUrl: z.string().url(),
  rpcUrl: z.string().url()
})

const l2Contracts = z.object({
  router: z.string().transform((val) => getAddress(val)),
  standardGateway: z.string().transform((val) => getAddress(val)),
  customGateway: z.string().transform((val) => getAddress(val)),
  wethGateway: z.string().transform((val) => getAddress(val)),
  weth: z.string().transform((val) => getAddress(val)),
  multicall: z.string().transform((val) => getAddress(val)),
  proxyAdmin: z.string().transform((val) => getAddress(val))
})

const l3Contracts = z.object({
  router: z.string().transform((val) => getAddress(val)),
  standardGateway: z.string().transform((val) => getAddress(val)),
  customGateway: z.string().transform((val) => getAddress(val)),
  wethGateway: z.string().transform((val) => getAddress(val)),
  weth: z.string().transform((val) => getAddress(val)),
  proxyAdmin: z.string().transform((val) => getAddress(val)),
  beaconProxyFactory: z.string().transform((val) => getAddress(val)),
  upgradeExecutor: z.string().transform((val) => getAddress(val)),
  multicall: z.string().transform((val) => getAddress(val))
})

const coreContracts = z.object({
  rollup: z.string().transform((val) => getAddress(val)),
  inbox: z.string().transform((val) => getAddress(val)),
  nativeToken: z.string().transform((val) => getAddress(val)),
  outbox: z.string().transform((val) => getAddress(val)),
  rollupEventInbox: z.string().transform((val) => getAddress(val)),
  challengeManager: z.string().transform((val) => getAddress(val)),
  adminProxy: z.string().transform((val) => getAddress(val)),
  sequencerInbox: z.string().transform((val) => getAddress(val)),
  bridge: z.string().transform((val) => getAddress(val)),
  upgradeExecutor: z.string().transform((val) => getAddress(val)),
  validatorUtils: z
    .string()
    .optional()
    .transform((val) => (val ? getAddress(val) : undefined)),
  validatorWalletCreator: z.string().transform((val) => getAddress(val)),
  deployedAtBlockNumber: z.number()
})

const sequencing = z.object({
  syndicateSequencingChain: z.string().transform((val) => getAddress(val)),
  allowlistSequencingModule: z.string().transform((val) => getAddress(val)),
  requireAndModule: z.string().transform((val) => getAddress(val)),
  settlementBlockBeforeDeployment: z.string(),
  deployedAtBlock: z.string()
})

const withdrawals = z.object({
  teeKeyManager: z.string().transform((val) => getAddress(val)),
  assertionPoster: z.string().transform((val) => getAddress(val)),
  teeModule: z.string().transform((val) => getAddress(val)),
  attestationDocVerifier: z.string().transform((val) => getAddress(val))
})

const bridge = z.object({
  chainInfo,
  coreContracts,
  tokenBridgeContracts: z.object({
    l2Contracts,
    l3Contracts
  })
})

export const synd = z.object({
  config: z.object({
    arbConfigManager: z.string().transform((val) => getAddress(val)),
    arbChainConfig: z.string().transform((val) => getAddress(val))
  }),
  bridge,
  sequencing,
  withdrawals
})

export const handoffConfig = z.object({
  settlementChainRpcUrl: z.string().url(),
  sequencingChainRpcUrl: z.string().url(),
  appChainRpcUrl: z.string().url(),
  appChainExplorerUrl: z.string().url(),
  ownerPrivateKey: z.string(),
  newOwnerAddress: z.string().transform((val) => getAddress(val)),
  synd
})

export const foundationConfig = z.object({
  settlementChainRpcUrl: z.string().url(),
  sequencingChainRpcUrl: z.string().url(),
  ethereumChainRpcUrl: z.string().url(),
  chainId: z.string().transform((val) => Number.parseInt(val)),
  chainName: z.string(),
  appChainRpcUrl: z.string().url(),
  appChainExplorerUrl: z.string().url(),
  nativeTokenAddress: z
    .string()
    .optional()
    .transform((val) => (val ? getAddress(val) : undefined)),
  deployerPrivateKey: z.string(),
  ownerPrivateKey: z.string(),
  // hash of the transaction that created the core contracts
  coreContractsCreatedAtHash: z.string().optional()
})

export const featuresConfig = z.object({
  coreContracts,
  ownerPrivateKey: z.string(),
  deployerPrivateKey: z.string(),
  chainId: z.string().transform((val) => Number.parseInt(val)),
  chainName: z.string(),
  settlementChainRpcUrl: z.string().url(),
  sequencingChainRpcUrl: z.string().url(),
  syndForkSequencingChainRpcUrl: z.string().url(),
  ethereumChainRpcUrl: z.string().url(),
  appChainRpcUrl: z.string().url(),
  appChainExplorerUrl: z.string().url(),
  sequencingContractAddress: z.string().transform((val) => getAddress(val))
})

export const e2eConfig = z.object({
  settlementRpcUrl: z.string().url(),
  l3RpcUrl: z.string().url(),
  inboxAddress: z.string().transform((val) => getAddress(val)),
  privateKey: z.string()
})

// Type utilities for generating valid synd object paths
type PathImpl<T, Key extends keyof T> = Key extends string
  ? T[Key] extends Record<string, unknown>
    ?
        | `${Key}.${PathImpl<T[Key], Exclude<keyof T[Key], keyof unknown[]>> & string}`
        | `${Key}.${Exclude<keyof T[Key], keyof unknown[]> & string}`
    : never
  : never

type PathImpl2<T> = PathImpl<T, keyof T> | keyof T

export type SyndPath<T> = PathImpl2<T> extends string | keyof T
  ? PathImpl2<T>
  : keyof T

export type SyndObjectPaths = SyndPath<z.infer<typeof synd>>
