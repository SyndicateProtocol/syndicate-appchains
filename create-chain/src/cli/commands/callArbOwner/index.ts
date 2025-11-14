import { exitWithError, print } from "@/src/utils/print";
import type { Command } from "@commander-js/extra-typings";
import type { AbiFunction } from "viem";
import { encodeFunctionData } from "viem";
import { ArbOwnerABI } from "../../../abi/nitro/ArbOwner";
import { callArbOwnerOptionsSchema, handleSchemaErrors } from "../../schema";
import { generateCallArbOwnerTx } from "./callArbOwner";
import { parseFunctionArgs } from "./parseFunctionArgs";

/**
 * Formats a function signature for display
 */
function formatFunctionSignature(functionAbi: AbiFunction): string {
	const params = functionAbi.inputs
		.map((input) => `${input.type} ${input.name || ""}`.trim())
		.join(", ");
	return `${functionAbi.name}(${params})`;
}

export function callArbOwnerCommand(program: Command) {
	const callArbOwner = program
		.command("callArbOwner")
		.description("Call ArbOwner functions through the UpgradeExecutor");

	// Helper to get only non-view/pure functions (state-changing functions)
	const getWriteFunctions = () => {
		return ArbOwnerABI.filter(
			(item) =>
				item.type === "function" &&
				item.stateMutability !== "view" &&
				item.stateMutability !== "pure",
		) as AbiFunction[];
	};

	// List available functions
	callArbOwner
		.command("list")
		.description("List all available ArbOwner write functions")
		.action(() => {
			const functions = getWriteFunctions();

			print("\nAvailable ArbOwner write functions:\n");
			for (const fn of functions) {
				const signature = formatFunctionSignature(fn);
				print(`  ${signature}`);
			}
			print("");
		});

	// Call a specific function
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
				// Validate the function exists in ArbOwner ABI and is a write function
				const functionAbi = ArbOwnerABI.find(
					(item) => item.type === "function" && item.name === functionName,
				) as AbiFunction | undefined;

				if (!functionAbi) {
					const availableFunctions = getWriteFunctions();
					exitWithError(
						`Function '${functionName}' not found in ArbOwner ABI.\n\nAvailable write functions:\n${availableFunctions
							.map((fn) => `  ${formatFunctionSignature(fn)}`)
							.join("\n")}\n\nTip: Run 'synd-cli callArbOwner list' to see all available functions.`,
					);
				}

				// Check if the function is a view/pure function (read-only)
				if (
					functionAbi.stateMutability === "view" ||
					functionAbi.stateMutability === "pure"
				) {
					exitWithError(
						`Function '${functionName}' is a read-only function (${functionAbi.stateMutability}).\nThis command only supports state-changing functions.\n\nTip: Run 'synd-cli callArbOwner list' to see all available write functions.`,
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
}
