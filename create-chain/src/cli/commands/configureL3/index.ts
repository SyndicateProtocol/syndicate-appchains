import { isValidAddress } from "@/src/utils/helpers";
import { exitWithError } from "@/src/utils/print";
import type { Command } from "@commander-js/extra-typings";
import type { Address } from "viem";
import { generateSetWasmMaxStackDepthTx } from "./setWasmMaxStackDepth";

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
		.option("--max-fee-per-gas <gwei>", "Max fee per gas in gwei")
		.option(
			"--custom-gas-token <address>",
			"Custom gas token contract address for chains using ERC20 gas tokens",
		)
		.action(
			async (
				depth: string,
				options: {
					parentRpc: string;
					childRpc?: string;
					parentUpgradeExecutor: string;
					parentInbox: string;
					childUpgradeExecutor: string;
					refundAddress: string;
					gasLimit?: string;
					maxFeePerGas?: string;
					customGasToken?: string;
				},
			) => {
				// Manual validation and type conversion
				const depthNum = Number(depth);
				if (Number.isNaN(depthNum) || depthNum <= 0) {
					exitWithError(`Invalid depth: ${depth}`);
				}

				// Validate addresses
				if (!isValidAddress(options.parentUpgradeExecutor)) {
					exitWithError(
						`Invalid parent upgrade executor address: ${options.parentUpgradeExecutor}`,
					);
				}
				if (!isValidAddress(options.parentInbox)) {
					exitWithError(`Invalid parent inbox address: ${options.parentInbox}`);
				}
				if (!isValidAddress(options.childUpgradeExecutor)) {
					exitWithError(
						`Invalid child upgrade executor address: ${options.childUpgradeExecutor}`,
					);
				}
				if (!isValidAddress(options.refundAddress)) {
					exitWithError(`Invalid refund address: ${options.refundAddress}`);
				}
				if (options.customGasToken && !isValidAddress(options.customGasToken)) {
					exitWithError(
						`Invalid custom gas token address: ${options.customGasToken}`,
					);
				}

				// Convert gas limit (optional)
				let gasLimit: bigint | undefined;
				if (options.gasLimit) {
					gasLimit = BigInt(options.gasLimit);
					if (gasLimit <= BigInt(0)) {
						exitWithError(`Invalid gas limit: ${options.gasLimit}`);
					}
				}

				// Convert max fee per gas (gwei to wei) (optional)
				let maxFeePerGas: bigint | undefined;
				if (options.maxFeePerGas) {
					const maxFeePerGasNum = Number(options.maxFeePerGas);
					if (Number.isNaN(maxFeePerGasNum) || maxFeePerGasNum <= 0) {
						exitWithError(`Invalid max fee per gas: ${options.maxFeePerGas}`);
					}
					// Convert gwei (decimal) to wei (integer) by multiplying by 1e9
					maxFeePerGas = BigInt(Math.round(maxFeePerGasNum * 1_000_000_000));
				}

				await generateSetWasmMaxStackDepthTx({
					parentRpc: options.parentRpc,
					childRpc: options.childRpc,
					parentUpgradeExecutorAddress:
						options.parentUpgradeExecutor as Address,
					parentInboxAddress: options.parentInbox as Address,
					childUpgradeExecutorAddress: options.childUpgradeExecutor as Address,
					refundAddress: options.refundAddress as Address,
					wasmMaxStackDepth: depthNum,
					gasLimit,
					maxFeePerGas,
					customGasTokenAddress: options.customGasToken as Address | undefined,
				});
			},
		);
}
