import type { Address } from "viem";
import { encodeFunctionData } from "viem";
import { ArbOwnerABI } from "../../../abi/nitro/ArbOwner";
import { generateCallArbOwnerTx } from "./callArbOwner";

interface SetWasmMaxStackDepthParams {
	parentRpc: string;
	childRpc?: string;
	parentUpgradeExecutor: Address;
	parentInbox: Address;
	childUpgradeExecutor: Address;
	refundAddress: Address;
	wasmMaxStackDepth: number;
	gasLimit?: bigint;
	maxFeePerGas?: bigint;
}

export async function generateSetWasmMaxStackDepthTx({
	wasmMaxStackDepth,
	...params
}: SetWasmMaxStackDepthParams) {
	const arbOwnerCalldata = encodeFunctionData({
		abi: ArbOwnerABI,
		functionName: "setWasmMaxStackDepth",
		args: [wasmMaxStackDepth],
	});

	await generateCallArbOwnerTx({
		...params,
		arbOwnerFunctionName: "setWasmMaxStackDepth",
		arbOwnerCalldata,
	});
}
