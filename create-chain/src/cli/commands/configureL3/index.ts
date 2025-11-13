import { exitWithError } from "@/src/utils/print";
import type { Command } from "@commander-js/extra-typings";
import { getAddress } from "viem";
import { z } from "zod";
import { generateSetWasmMaxStackDepthTx } from "./setWasmMaxStackDepth";

// Ethereum address validation schema
const ethAddressSchema = z
	.string()
	.transform((val) => getAddress(val))
	.refine((val) => !!val, { message: "Must be a valid Ethereum address" });

// Schema for setWasmMaxStackDepth options
const setWasmMaxStackDepthOptionsSchema = z.object({
	parentRpc: z.string().url("Invalid parent RPC URL"),
	childRpc: z.string().url("Invalid child RPC URL").optional(),
	parentUpgradeExecutor: ethAddressSchema,
	parentInbox: ethAddressSchema,
	childUpgradeExecutor: ethAddressSchema,
	refundAddress: ethAddressSchema,
	gasLimit: z
		.string()
		.transform((val) => BigInt(val))
		.optional()
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
			{ message: "Gas limit must be a positive integer" },
		),
	maxFeePerGas: z
		.string()
		.transform((val) => BigInt(val))
		.optional()
		.refine(
			(val) => {
				if (!val) return true;
				const num = Number(val);
				return !Number.isNaN(num) && num > 0;
			},
			{ message: "Max fee per gas must be a positive number" },
		),
	customGasToken: ethAddressSchema.optional(),
});

/**
 * Register the configureL3 command with its subcommands
 */
export function configureL3Command(program: Command) {
	const configureL3 = program
		.command("configureL3")
		.description(
			"Generate targets & calldata needed to configure syndicate appchains via its UpgradeExecutor on the parent chain",
		);

	// setWasmMaxStackDepth subcommand
	configureL3
		.command("setWasmMaxStackDepth")
		.description("Set the WASM max stack depth on an appchain")
		.argument("<depth>", "The maximum WASM stack depth")
		.requiredOption("--parent-rpc <url>", "Parent chain RPC URL")
		.requiredOption(
			"--parent-upgrade-executor <address>",
			"Parent chain UpgradeExecutor address",
		)
		.requiredOption("--parent-inbox <address>", "Parent chain Inbox address")
		.requiredOption(
			"--child-upgrade-executor <address>",
			"L3 UpgradeExecutor address",
		)
		.requiredOption(
			"--refund-address <address>",
			"Address on appchain to receive excess fees",
		)
		.option(
			"--child-rpc <url>",
			"Child chain RPC URL (enables gas estimation from chain)",
		)
		.option("--gas-limit <limit>", "Gas limit for retryable ticket")
		.option("--max-fee-per-gas <wei>", "Max fee per gas in wei")
		.option(
			"--custom-gas-token <address>",
			"Custom gas token contract address for chains using ERC20 gas tokens",
		)
		.action(async (depth: string, options: Record<string, unknown>) => {
			const depthNum = Number(depth);
			if (Number.isNaN(depthNum) || depthNum <= 0) {
				exitWithError(`Invalid depth: ${depth}`);
			}

			const {
				data: validatedOptions,
				success,
				error,
			} = setWasmMaxStackDepthOptionsSchema.safeParse(options);

			if (!success) {
				const errors = error.issues
					.map((err) => `  - ${err.path.join(".")}: ${err.message}`)
					.join("\n");
				exitWithError(`Invalid options:\n${errors}`);
				return; // exitWithError calls process.exit but TypeScript doesn't know that
			}

			await generateSetWasmMaxStackDepthTx({
				...validatedOptions,
				wasmMaxStackDepth: depthNum,
			});
		});
}
