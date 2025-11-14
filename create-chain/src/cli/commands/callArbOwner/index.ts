import { exitWithError } from "@/src/utils/print";
import type { Command } from "@commander-js/extra-typings";
import { encodeFunctionData } from "viem";
import { ArbOwnerABI } from "../../../abi/nitro/ArbOwner";
import { callArbOwnerOptionsSchema, handleSchemaErrors } from "../../schema";
import { generateCallArbOwnerTx } from "./callArbOwner";
import { parseFunctionArgs } from "./parseFunctionArgs";

export function configureAppchainCommand(program: Command) {
	program
		.command("callArbOwner")
		.description("Call any ArbOwner function through the UpgradeExecutor")
		.argument("<functionName>", "Name of the ArbOwner function to call")
		.argument("[args...]", "Arguments for the function")
		.requiredOption("--parent-rpc <url>", "Parent chain RPC URL")
		.requiredOption(
			"--parent-upgrade-executor <address>",
			"Parent chain UpgradeExecutor address",
		)
		.requiredOption("--parent-inbox <address>", "Parent chain Inbox address")
		.requiredOption(
			"--child-upgrade-executor <address>",
			"Appchain UpgradeExecutor address",
		)
		.requiredOption(
			"--refund-address <address>",
			"Address on appchain to receive excess fees",
		)
		.option("--child-rpc <url>", "Appchain RPC URL")
		.option("--gas-limit <limit>", "Gas limit for retryable ticket")
		.option("--max-fee-per-gas <wei>", "Max fee per gas in wei")
		.action(
			async (
				functionName: string,
				args: string[],
				options: Record<string, unknown>,
			) => {
				// Validate the function exists in ArbOwner ABI
				const functionAbi = ArbOwnerABI.find(
					(item) => item.type === "function" && item.name === functionName,
				);

				if (!functionAbi) {
					exitWithError(
						`Function '${functionName}' not found in ArbOwner ABI. Available functions:\n${ArbOwnerABI.filter(
							(item) => item.type === "function",
						)
							.map((item) => `  - ${item.name}`)
							.join("\n")}`,
					);
				}

				// Parse and validate options
				const {
					data: validatedOptions,
					success,
					error,
				} = callArbOwnerOptionsSchema.safeParse(options);

				if (!success) {
					return handleSchemaErrors(error);
				}

				// Parse function arguments
				const parsedArgs = parseFunctionArgs(ArbOwnerABI, functionName, args);
				if (!parsedArgs) {
					return;
				}
				// Generate the calldata
				const arbOwnerCalldata = encodeFunctionData({
					abi: ArbOwnerABI,
					functionName: functionName as never,
					args: parsedArgs as never,
				});

				await generateCallArbOwnerTx({
					...validatedOptions,
					arbOwnerFunctionName: functionName,
					arbOwnerCalldata,
				});
			},
		);

	// // Legacy command - kept for backward compatibility
	// callArbOwner
	// 	.command("setWasmMaxStackDepth")
	// 	.description("Set the WASM max stack depth on an appchain")
	// 	.argument("<depth>", "The maximum WASM stack depth")
	// 	.requiredOption("--parent-rpc <url>", "Parent chain RPC URL")
	// 	.requiredOption(
	// 		"--parent-upgrade-executor <address>",
	// 		"Parent chain UpgradeExecutor address",
	// 	)
	// 	.requiredOption("--parent-inbox <address>", "Parent chain Inbox address")
	// 	.requiredOption(
	// 		"--child-upgrade-executor <address>",
	// 		"Appchain UpgradeExecutor address",
	// 	)
	// 	.requiredOption(
	// 		"--refund-address <address>",
	// 		"Address on appchain to receive excess fees",
	// 	)
	// 	.option("--child-rpc <url>", "Appchain RPC URL")
	// 	.option("--gas-limit <limit>", "Gas limit for retryable ticket")
	// 	.option("--max-fee-per-gas <wei>", "Max fee per gas in wei")
	// 	.action(async (depth: string, options: Record<string, unknown>) => {
	// 		const depthNum = Number(depth);
	// 		if (Number.isNaN(depthNum) || depthNum <= 0) {
	// 			exitWithError(`Invalid depth: ${depth}`);
	// 		}

	// 		const {
	// 			data: validatedOptions,
	// 			success,
	// 			error,
	// 		} = setWasmMaxStackDepthOptionsSchema.safeParse(options);

	// 		if (!success) {
	// 			return handleSchemaErrors(error);
	// 		}

	// 		await generateSetWasmMaxStackDepthTx({
	// 			...validatedOptions,
	// 			wasmMaxStackDepth: depthNum,
	// 		});
	// 	});
}
