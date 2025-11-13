import { exitWithError } from "@/src/utils/print";
import type { Command } from "@commander-js/extra-typings";
import {
	handleSchemaErrors,
	setWasmMaxStackDepthOptionsSchema,
} from "../../schema";
import { generateSetWasmMaxStackDepthTx } from "./setWasmMaxStackDepth";

export function configureAppchainCommand(program: Command) {
	const configureAppchain = program
		.command("configureAppchain")
		.description("Configure a syndicate appchain");

	configureAppchain
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
				return handleSchemaErrors(error);
			}

			await generateSetWasmMaxStackDepthTx({
				...validatedOptions,
				wasmMaxStackDepth: depthNum,
			});
		});
}
