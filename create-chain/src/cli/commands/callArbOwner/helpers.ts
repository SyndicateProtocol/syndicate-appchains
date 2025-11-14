import { getNativeCurrency } from "@/src/utils/helpers";
import {
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
