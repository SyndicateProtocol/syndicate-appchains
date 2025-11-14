import type { Abi, AbiFunction } from "viem";
import { exitWithError } from "../../../utils/print";

/**
 * Formats a function signature for display
 */
function formatFunctionSignature(functionAbi: AbiFunction): string {
	const params = functionAbi.inputs
		.map((input) => `${input.type} ${input.name || ""}`.trim())
		.join(", ");
	return `${functionAbi.name}(${params})`;
}

/**
 * Parses command line arguments into the correct types for a function's ABI inputs
 */
export function parseFunctionArgs(
	abi: Abi,
	functionName: string,
	args: string[],
) {
	// Find the function in the ABI
	const functionAbi = abi.find(
		(item) => item.type === "function" && item.name === functionName,
	) as AbiFunction | undefined;

	if (!functionAbi) {
		return exitWithError(`Function ${functionName} not found in ABI`);
	}

	const inputs = functionAbi.inputs;
	const signature = formatFunctionSignature(functionAbi);

	if (args.length !== inputs.length) {
		return exitWithError(
			`Function ${functionName} expects ${inputs.length} argument(s) but got ${args.length}.\n${signature}`,
		);
	}

	// Parse each argument based on its type
	return args.map((arg, index) => {
		const input = inputs[index];
		const type = input.type;

		try {
			// Handle different Solidity types
			if (type === "address") {
				// Address validation will be handled by viem's encodeFunctionData
				return arg as `0x${string}`;
			}
			if (type === "bool") {
				if (arg === "true") return true;
				if (arg === "false") return false;
				throw new Error("Must be 'true' or 'false'");
			}
			if (type === "string") {
				return arg;
			}
			// Handle uint/int types
			if (type.startsWith("uint") || type.startsWith("int")) {
				const isSigned = type.startsWith("int");
				const num = BigInt(arg);

				// Basic validation for unsigned types
				if (!isSigned && num < 0) {
					throw new Error("Cannot be negative for unsigned type");
				}

				// Check bit size if specified (e.g., uint8, uint64)
				const bitMatch = type.match(/\d+$/);
				if (bitMatch) {
					const bits = Number.parseInt(bitMatch[0], 10);
					const max = BigInt(2) ** BigInt(bits) - BigInt(1);
					const min = isSigned ? -(BigInt(2) ** BigInt(bits - 1)) : BigInt(0);

					if (num > max || num < min) {
						throw new Error(
							`Value out of range for ${type} (min: ${min}, max: ${max})`,
						);
					}
				}

				return num;
			}
			if (type === "bytes" || type.startsWith("bytes")) {
				// Hex string for bytes
				if (!arg.startsWith("0x")) {
					throw new Error("Bytes value must start with 0x");
				}
				return arg as `0x${string}`;
			}
			// Add support for arrays
			if (type.endsWith("[]")) {
				try {
					return JSON.parse(arg);
				} catch {
					throw new Error("Array values must be valid JSON");
				}
			}

			// Default: return as-is and let viem handle it
			return arg;
		} catch (error) {
			return exitWithError(
				`Invalid argument at position ${index} (${input.name || "unnamed"}): ${error instanceof Error ? error.message : "Unknown error"}.\n${signature}`,
			);
		}
	});
}
