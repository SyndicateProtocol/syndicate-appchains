import { getAddress } from "viem";
import { z } from "zod";
import { exitWithError } from "../utils/print";

// Ethereum address validation schema
export const ethAddressSchema = z
	.string()
	.refine(
		(val) => {
			try {
				getAddress(val);
				return true;
			} catch {
				return false;
			}
		},
		{
			message: "Must be a valid Ethereum address",
		},
	)
	.transform((val) => getAddress(val));

const positiveBigIntSchema = z
	.string()
	.refine(
		(val) => {
			if (!val) return true;
			try {
				const bigIntVal = BigInt(val);
				return bigIntVal > BigInt(0);
			} catch {
				return false;
			}
		},
		{ message: "Must be a positive integer" },
	)
	.transform((val) => (val ? BigInt(val) : undefined));

// Schema for setWasmMaxStackDepth options
export const setWasmMaxStackDepthOptionsSchema = z
	.object({
		parentRpc: z.string().url("Invalid parent RPC URL"),
		childRpc: z.string().url("Invalid child RPC URL").optional(),
		parentUpgradeExecutor: ethAddressSchema,
		parentInbox: ethAddressSchema,
		childUpgradeExecutor: ethAddressSchema,
		refundAddress: ethAddressSchema,
		gasLimit: positiveBigIntSchema.optional(),
		maxFeePerGas: positiveBigIntSchema.optional(),
		customGasToken: ethAddressSchema.optional(),
	})
	.strict();

export function handleSchemaErrors(errors: z.ZodError) {
	const err = errors.issues
		.map((err) => `  - ${err.path.join(".")}: ${err.message}`)
		.join("\n");
	return exitWithError(`Invalid options:\n${err}`);
}
