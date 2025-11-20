import { type Hex, getAddress, zeroAddress } from "viem"
import { z } from "zod"
import { exitWithError } from "../utils/print"

export const ethAddressSchema = z
  .string()
  .refine(
    (val) => {
      try {
        getAddress(val)
        return true
      } catch {
        return false
      }
    },
    {
      message: "Must be a valid Ethereum address"
    }
  )
  .transform((val) => getAddress(val))

const positiveBigIntSchema = z
  .string()
  .refine(
    (val) => {
      if (!val) return true
      try {
        const bigIntVal = BigInt(val)
        return bigIntVal > BigInt(0)
      } catch {
        return false
      }
    },
    { message: "Must be a positive integer" }
  )
  .transform((val) => (val ? BigInt(val) : undefined))

const privateKeySchema = (privateKeyName: string) =>
  z
    .string()
    .regex(/^0x[a-fA-F0-9]{64}$/, `Invalid ${privateKeyName} private key`)
    .transform((val) => val as Hex)

const chainIdSchema = z.coerce
  .number()
  .int()
  .positive("Chain ID must be a positive integer")

const chainNameSchema = z
  .string()
  .min(1, "Chain name is required")
  .transform((val) => val.toLowerCase().replace(/\s+/g, "-"))

export const callArbOwnerOptionsSchema = z
  .object({
    settlementRpc: z.url("Invalid settlement chain RPC URL"),
    appchainRpc: z.url("Invalid appchain RPC URL").optional(),
    settlementUpgradeExecutor: ethAddressSchema,
    settlementInbox: ethAddressSchema,
    appchainUpgradeExecutor: ethAddressSchema,
    refundAddress: ethAddressSchema,
    gasLimit: positiveBigIntSchema.optional(),
    maxFeePerGas: positiveBigIntSchema.optional()
  })
  .strict()

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
  newOwner: z.string().transform((val) => getAddress(val)),
  synd
})

export const appchainCreateFoundationOptionsSchema = z
  .object({
    settlementRpc: z.url("Invalid settlement chain RPC URL"),
    sequencingRpc: z.url("Invalid sequencing chain RPC URL"),
    ethereumRpc: z.url("Invalid ethereum chain RPC URL"),
    appchainRpc: z.url("Invalid appchain RPC URL"),
    appchainExplorer: z.url("Invalid appchain explorer URL"),
    id: chainIdSchema,
    name: chainNameSchema,
    deployerPrivateKey: privateKeySchema("deployer"),
    ownerPrivateKey: privateKeySchema("owner"),
    nativeToken: ethAddressSchema.default(zeroAddress),
    coreContractsCreatedAtHash: z
      .string()
      .regex(/^0x[a-fA-F0-9]{64}$/, "Invalid transaction hash")
      .transform((val) => (val ? (val as Hex) : undefined))
      .optional()
  })
  .strict()

export const appchainCreateFeaturesOptionsSchema = z
  .object({
    ownerPrivateKey: privateKeySchema("owner"),
    deployerPrivateKey: privateKeySchema("deployer"),
    chainName: chainNameSchema,
    settlementRpc: z.url("Invalid settlement chain RPC URL"),
    sequencingRpc: z.url("Invalid sequencing chain RPC URL"),
    syndForkSequencingRpc: z.url("Invalid synd fork sequencing chain RPC URL"),
    ethereumRpc: z.url("Invalid ethereum chain RPC URL"),
    appchainRpc: z.url("Invalid appchain RPC URL"),
    appchainExplorer: z.url("Invalid appchain explorer URL"),
    sequencingContract: ethAddressSchema,
    coreContracts
  })
  .strict()

export const appchainCreateTeeModuleOptionsSchema = z
  .object({
    ownerPrivateKey: privateKeySchema("owner"),
    deployerPrivateKey: privateKeySchema("deployer"),
    settlementRpc: z.url("Invalid settlement chain RPC URL"),
    sequencingRpc: z.url("Invalid sequencing chain RPC URL"),
    syndForkSequencingRpc: z.url("Invalid synd fork sequencing chain RPC URL"),
    ethereumRpc: z.url("Invalid ethereum chain RPC URL"),
    appchainRpc: z.url("Invalid appchain RPC URL"),
    sequencingContract: ethAddressSchema,
    rollup: ethAddressSchema,
    upgradeExecutor: ethAddressSchema,
    bridge: ethAddressSchema
  })
  .strict()

export const appchainCheckTokenBridgeOptionsSchema = z
  .object({
    settlementRpc: z.url("Invalid settlement chain RPC URL"),
    appchainRpc: z.url("Invalid appchain RPC URL"),
    rollup: ethAddressSchema,
    createdAtHash: z
      .string()
      .regex(/^0x[a-fA-F0-9]{64}$/, "Invalid transaction hash")
  })
  .strict()

export const appchainDeployTeeModuleOptionsSchema = z
  .object({
    deployerPrivateKey: privateKeySchema("deployer"),
    settlementRpc: z.url("Invalid settlement chain RPC URL"),
    sequencingRpc: z.url("Invalid sequencing chain RPC URL"),
    syndForkSequencingRpc: z.url("Invalid synd fork sequencing chain RPC URL"),
    ethereumRpc: z.url("Invalid ethereum chain RPC URL"),
    appchainRpc: z.url("Invalid appchain RPC URL"),
    sequencingContract: ethAddressSchema,
    assertionPoster: ethAddressSchema,
    bridge: ethAddressSchema
  })
  .strict()

export const appchainDeployAssertionPosterOptionsSchema = z
  .object({
    ownerPrivateKey: privateKeySchema("owner"),
    deployerPrivateKey: privateKeySchema("deployer"),
    settlementRpc: z.url("Invalid settlement chain RPC URL"),
    rollup: ethAddressSchema,
    upgradeExecutor: ethAddressSchema
  })
  .strict()

export const appchainCreateSequencingChainOptionsSchema = z
  .object({
    sequencingRpc: z.url("Invalid sequencing chain RPC URL"),
    ownerPrivateKey: privateKeySchema("owner"),
    deployerPrivateKey: privateKeySchema("deployer"),
    id: chainIdSchema
  })
  .strict()

export const appchainHandoffOptionsSchema = z
  .object({
    settlementRpc: z.url("Invalid settlement chain RPC URL"),
    sequencingRpc: z.url("Invalid sequencing chain RPC URL"),
    appchainRpc: z.url("Invalid appchain RPC URL"),
    ownerPrivateKey: privateKeySchema("owner"),
    newOwner: ethAddressSchema,
    synd
  })
  .strict()

export const appchainE2EOptionsSchema = z
  .object({
    settlementRpc: z.url("Invalid settlement chain RPC URL"),
    appchainRpc: z.url("Invalid appchain RPC URL"),
    inbox: ethAddressSchema,
    privateKey: privateKeySchema("transaction")
  })
  .strict()

export function handleSchemaErrors(errors: z.ZodError) {
  const err = errors.issues
    .map((err) => `  - ${err.path.join(".")}: ${err.message}`)
    .join("\n")
  return exitWithError(`Invalid options:\n${err}`)
}

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
