import { getAddress, zeroAddress } from "viem"
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

export const appchainCreateFoundationOptionsSchema = z
  .object({
    settlementRpc: z.url("Invalid settlement chain RPC URL"),
    sequencingRpc: z.url("Invalid sequencing chain RPC URL"),
    ethereumRpc: z.url("Invalid ethereum chain RPC URL"),
    appchainRpc: z.url("Invalid appchain RPC URL"),
    appchainExplorerUrl: z.url("Invalid appchain explorer URL"),
    id: z.coerce.number().int().positive("Chain ID must be a positive integer"),
    name: z
      .string()
      .min(1, "Chain name is required")
      .transform((val) => val.toLowerCase().replace(/\s+/g, "-")),
    deployerPrivateKey: z
      .string()
      .regex(/^0x[a-fA-F0-9]{64}$/, "Invalid deployer private key"),
    ownerPrivateKey: z
      .string()
      .regex(/^0x[a-fA-F0-9]{64}$/, "Invalid owner private key"),
    nativeTokenAddress: ethAddressSchema.default(zeroAddress),
    coreContractsCreatedAtHash: z
      .string()
      .regex(/^0x[a-fA-F0-9]{64}$/, "Invalid transaction hash")
      .optional()
  })
  .strict()

export function handleSchemaErrors(errors: z.ZodError) {
  const err = errors.issues
    .map((err) => `  - ${err.path.join(".")}: ${err.message}`)
    .join("\n")
  return exitWithError(`Invalid options:\n${err}`)
}
