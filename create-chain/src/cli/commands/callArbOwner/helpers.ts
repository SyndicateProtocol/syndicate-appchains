import { getNativeCurrency } from "@/src/utils/helpers";
import { exitWithError } from "@/src/utils/print";
import {
	type Abi,
	type AbiFunction,
	type Address,
	type PublicClient,
	parseAbiItem,
} from "viem";

export async function detectCustomNativeToken(
	publicClient: PublicClient,
	inboxAddress: Address,
) {
	try {
		const bridgeAddress = await publicClient.readContract({
			address: inboxAddress,
			abi: [parseAbiItem("function bridge() public view returns (address)")],
			functionName: "bridge",
		});

		try {
			const address = await publicClient.readContract({
				address: bridgeAddress,
				abi: [
					parseAbiItem("function nativeToken() public view returns (address)"),
				],
				functionName: "nativeToken",
			});
			const currency = await getNativeCurrency(publicClient, address);
			return {
				...currency,
				address,
			};
		} catch (_) {
			return null;
		}
	} catch (_) {
		console.warn("⚠️  Could not detect native token, assuming ETH-native chain");
		return null;
	}
}

export function formatFunctionSignatureForDisplay(
	functionAbi: AbiFunction,
): string {
	const params = functionAbi.inputs
		.map((input) => `${input.type} ${input.name || ""}`.trim())
		.join(", ");
	return `${functionAbi.name}(${params})`;
}

/**
 * Preprocesses command line arguments before passing to viem.
 * Does MINIMAL parsing - just enough to convert string inputs to the right JavaScript types.
 * Viem's encodeFunctionData handles all validation.
 */
export function preprocessArgs(
	functionAbi: AbiFunction,
	args: string[],
): unknown[] {
	return args.map((arg, index) => {
		const input = functionAbi.inputs[index];
		const type = input.type;

		// Handle booleans
		if (type === "bool") {
			if (arg === "true") return true;
			if (arg === "false") return false;
			throw new Error(`Invalid boolean value "${arg}". Use "true" or "false"`);
		}

		// Handle numeric types - convert to BigInt
		if (type.startsWith("uint") || type.startsWith("int")) {
			try {
				return BigInt(arg);
			} catch {
				throw new Error(`Invalid number "${arg}"`);
			}
		}

		// Handle arrays and tuples - parse as JSON
		if (type.endsWith("[]") || type.startsWith("tuple")) {
			try {
				return JSON.parse(arg);
			} catch {
				throw new Error(`Invalid JSON "${arg}"`);
			}
		}

		// For everything else (address, bytes, string), return as-is
		// viem will validate and handle these
		return arg;
	});
}
