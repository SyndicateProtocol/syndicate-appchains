import { isValidAddress } from "@/src/utils/helpers";
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
		.description("Set the WASM max stack depth on L3")
		.argument("<depth>", "The maximum WASM stack depth")
		.requiredOption("--parent-rpc <url>", "Parent chain RPC URL")
		.requiredOption(
			"--parent-upgrade-executor <address>",
			"Parent chain UpgradeExecutor address",
		)
		.requiredOption("--parent-inbox <address>", "Parent chain Inbox address")
		.requiredOption(
			"--l3-upgrade-executor <address>",
			"L3 UpgradeExecutor address",
		)
		.requiredOption(
			"--refund-address <address>",
			"Address on L3 to receive excess fees",
		)
		.option("--gas-limit <limit>", "Gas limit for retryable ticket", "1000000")
		.option("--max-fee-per-gas <gwei>", "Max fee per gas in gwei", "0.1")
		.option(
			"--custom-gas-token <address>",
			"Custom gas token contract address for chains using ERC20 gas tokens",
		)
		.action(
			async (
				depth: string,
				options: {
					parentRpc: string;
					parentUpgradeExecutor: string;
					parentInbox: string;
					l3UpgradeExecutor: string;
					refundAddress: string;
					gasLimit: string;
					maxFeePerGas: string;
					customGasToken?: string;
				},
			) => {
				// Manual validation and type conversion
				const depthNum = Number(depth);
				if (Number.isNaN(depthNum) || depthNum <= 0) {
					console.error(`❌ Invalid depth: ${depth}`);
					process.exit(1);
				}

				// Validate addresses
				const _addressRegex = /^0x[a-fA-F0-9]{40}$/;
				if (!isValidAddress(options.parentUpgradeExecutor)) {
					console.error(
						`❌ Invalid parent upgrade executor address: ${options.parentUpgradeExecutor}`,
					);
					process.exit(1);
				}
				if (!isValidAddress(options.parentInbox)) {
					console.error(
						`❌ Invalid parent inbox address: ${options.parentInbox}`,
					);
					process.exit(1);
				}
				if (!isValidAddress(options.l3UpgradeExecutor)) {
					console.error(
						`❌ Invalid L3 upgrade executor address: ${options.l3UpgradeExecutor}`,
					);
					process.exit(1);
				}
				if (!isValidAddress(options.refundAddress)) {
					console.error(`❌ Invalid refund address: ${options.refundAddress}`);
					process.exit(1);
				}
				if (options.customGasToken && !isValidAddress(options.customGasToken)) {
					console.error(
						`❌ Invalid custom gas token address: ${options.customGasToken}`,
					);
					process.exit(1);
				}

				// Convert gas limit
				const gasLimit = BigInt(options.gasLimit);
				if (gasLimit <= BigInt(0)) {
					console.error(`❌ Invalid gas limit: ${options.gasLimit}`);
					process.exit(1);
				}

				// Convert max fee per gas (gwei to wei)
				const maxFeePerGasNum = Number(options.maxFeePerGas);
				if (Number.isNaN(maxFeePerGasNum) || maxFeePerGasNum <= 0) {
					console.error(`❌ Invalid max fee per gas: ${options.maxFeePerGas}`);
					process.exit(1);
				}
				// Convert gwei (decimal) to wei (integer) by multiplying by 1e9
				const maxFeePerGas = BigInt(Math.round(maxFeePerGasNum * 1_000_000_000));

				await generateSetWasmMaxStackDepthTx({
					parentChainRpcUrl: options.parentRpc,
					parentUpgradeExecutorAddress:
						options.parentUpgradeExecutor as Address,
					parentInboxAddress: options.parentInbox as Address,
					l3UpgradeExecutorAddress: options.l3UpgradeExecutor as Address,
					refundAddress: options.refundAddress as Address,
					wasmMaxStackDepth: depthNum,
					gasLimit,
					maxFeePerGas,
					customGasTokenAddress: options.customGasToken as Address | undefined,
				});
			},
		);
}
