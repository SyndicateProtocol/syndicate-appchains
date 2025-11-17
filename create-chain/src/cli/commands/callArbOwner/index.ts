import { exitWithError, print } from "@/src/utils/print";
import type { Command } from "@commander-js/extra-typings";
import type { ExtractAbiFunctionNames } from "abitype";
import type { AbiFunction } from "viem";
import { encodeFunctionData } from "viem";
import { ArbOwnerABI } from "../../../abi/nitro/ArbOwner";
import { callArbOwnerOptionsSchema, handleSchemaErrors } from "../../schema";
import { generateCallArbOwnerTx } from "./callArbOwner";
import { formatFunctionSignatureForDisplay, preprocessArgs } from "./helpers";

export function callArbOwnerCommand(program: Command) {
	const callArbOwner = program
		.command("callArbOwner")
		.description("Call ArbOwner functions through the UpgradeExecutor");

	const getWriteFunctions = () => {
		return ArbOwnerABI.filter(
			(item) => item.type === "function" && item.stateMutability !== "view",
		) as AbiFunction[];
	};

	callArbOwner
		.command("list")
		.description("List all available ArbOwner write functions")
		.action(() => {
			const functions = getWriteFunctions();
			print("\nAvailable ArbOwner write functions:\n");
			for (const fn of functions) {
				print(`  ${formatFunctionSignatureForDisplay(fn)}`);
			}
		});

	callArbOwner
		.command("call")
		.description("Call a specific ArbOwner function")
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
				const writeFunctions = getWriteFunctions();
				const functionAbi = writeFunctions.find(
					(item) => item.name === functionName,
				) as AbiFunction | undefined;

				if (!functionAbi) {
					return exitWithError(
						`Function '${functionName}' not found in ArbOwner ABI.\n\nAvailable write functions:\n${writeFunctions
							.map((fn) => `  ${formatFunctionSignatureForDisplay(fn)}`)
							.join(
								"\n",
							)}\n\nTip: Run 'synd-cli callArbOwner list' to see all available functions.`,
					);
				}

				const {
					data: validatedOptions,
					success,
					error,
				} = callArbOwnerOptionsSchema.safeParse(options);

				if (!success) {
					return handleSchemaErrors(error);
				}

				if (args.length !== functionAbi.inputs.length) {
					return exitWithError(
						`Function '${functionName}' expects ${functionAbi.inputs.length} argument(s) but got ${args.length}.\n${formatFunctionSignatureForDisplay(functionAbi)}`,
					);
				}

				let preprocessedArgs: unknown[] = [];
				try {
					preprocessedArgs = preprocessArgs(functionAbi, args);
				} catch (error) {
					return exitWithError(
						`Invalid arguments: ${error instanceof Error ? error.message : "Unknown error"}\n${formatFunctionSignatureForDisplay(functionAbi)}`,
					);
				}

				const arbOwnerCalldata = encodeFunctionData({
					abi: ArbOwnerABI,
					functionName: functionName as ExtractAbiFunctionNames<
						typeof ArbOwnerABI
					>,
					// biome-ignore lint/suspicious/noExplicitAny: args could be of any type here, we rely on viem to validate
					args: preprocessedArgs as any,
				});

				await generateCallArbOwnerTx({
					...validatedOptions,
					arbOwnerFunctionName: functionName,
					arbOwnerCalldata,
				});
			},
		);
}
