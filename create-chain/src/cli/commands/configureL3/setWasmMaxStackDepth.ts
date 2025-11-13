import { ERC20Abi } from "@/src/abi/ERC20";
import type { Address, Hex } from "viem";
import { createPublicClient, encodeFunctionData, http } from "viem";
import { ArbOwnerABI } from "../../../abi/nitro/ArbOwner";
import { ERC20InboxABI } from "../../../abi/nitro/ERC20Inbox";
import { InboxABI } from "../../../abi/nitro/Inbox";
import { UpgradeExecutorABI } from "../../../abi/nitro/UpgradeExecutor";
import { ARB_OWNER_PRECOMPILE_ADDRESS } from "../../../utils/constants";
import {
	formatWeiValue,
	print,
	printIndented,
	printSection,
	printSeparator,
} from "../../../utils/print";

interface SetWasmMaxStackDepthParams {
	parentChainRpcUrl: string;
	parentUpgradeExecutorAddress: Address;
	parentInboxAddress: Address;
	childUpgradeExecutorAddress: Address;
	refundAddress: Address;
	wasmMaxStackDepth: number;
	gasLimit?: bigint;
	maxFeePerGas?: bigint;
	customGasTokenAddress?: Address;
}

export async function generateSetWasmMaxStackDepthTx({
	parentChainRpcUrl,
	parentUpgradeExecutorAddress,
	parentInboxAddress,
	childUpgradeExecutorAddress,
	gasLimit = BigInt(1_000_000),
	maxFeePerGas = BigInt(100000000), // 0.1 gwei default
	refundAddress,
	customGasTokenAddress,
	wasmMaxStackDepth,
}: SetWasmMaxStackDepthParams) {
	const useCustomGasToken = !!customGasTokenAddress;
	const publicClient = createPublicClient({
		transport: http(parentChainRpcUrl),
	});

	// Get calldata for calling setWasmMaxStackDepth through the UpgradeExecutro
	const l3UpgradeExecutorCalldata = encodeFunctionData({
		abi: UpgradeExecutorABI,
		functionName: "executeCall",
		args: [
			ARB_OWNER_PRECOMPILE_ADDRESS,
			encodeFunctionData({
				abi: ArbOwnerABI,
				functionName: "setWasmMaxStackDepth",
				args: [wasmMaxStackDepth],
			}),
		],
	});

	// Calculate submission cost for the retryable ticket
	const dataLength = BigInt((l3UpgradeExecutorCalldata.length - 2) / 2); // Remove '0x' and divide by 2
	let submissionCost: bigint;
	try {
		submissionCost = await publicClient.readContract({
			address: parentInboxAddress,
			abi: InboxABI,
			functionName: "calculateRetryableSubmissionFee",
			args: [dataLength, BigInt(0)], // 0 means use current basefee
		});
		if (submissionCost === BigInt(0)) {
			throw new Error("Submission cost is 0");
		}
	} catch (error) {
		console.warn(
			"Could not calculate submission cost, using formula-based estimate",
			error,
		);

		// fallback to hardcoded estimate from Inbox's calculateRetryableSubmissionFee()
		// https://github.com/OffchainLabs/nitro-contracts/blob/c32af127fe6a9124316abebbf756609649ede1f5/src/bridge/Inbox.sol#L309-L310
		// Assuming a reasonable base fee of 0.1 gwei = 100_000_000 wei
		const estimatedBaseFee = BigInt(100_000_000);
		submissionCost = (BigInt(1400) + BigInt(6) * dataLength) * estimatedBaseFee;
	}

	const maxSubmissionCost = (submissionCost * BigInt(150)) / BigInt(100); // Add 50% buffer for safety

	// Calculate total value needed
	const totalValue = maxSubmissionCost + gasLimit * maxFeePerGas;

	// Encode call to Inbox (ERC20Inbox for custom gas token, regular Inbox for ETH)
	let inboxCalldata: Hex;
	if (useCustomGasToken) {
		// https://github.com/OffchainLabs/nitro-contracts/blob/c32af127fe6a9124316abebbf756609649ede1f5/src/bridge/ERC20Inbox.sol#L64-L65
		inboxCalldata = encodeFunctionData({
			abi: ERC20InboxABI,
			functionName: "createRetryableTicket",
			args: [
				childUpgradeExecutorAddress, // to
				BigInt(0), // l2CallValue
				maxSubmissionCost, // maxSubmissionCost
				refundAddress, // excessFeeRefundAddress
				refundAddress, // callValueRefundAddress
				gasLimit, // gasLimit
				maxFeePerGas, // maxFeePerGas
				totalValue, // tokenTotalFeeAmount - amount to pull from sender
				l3UpgradeExecutorCalldata, // data
			],
		});
	} else {
		// https://github.com/OffchainLabs/nitro-contracts/blob/c32af127fe6a9124316abebbf756609649ede1f5/src/bridge/Inbox.sol#L261
		inboxCalldata = encodeFunctionData({
			abi: InboxABI,
			functionName: "createRetryableTicket",
			args: [
				childUpgradeExecutorAddress, // to
				BigInt(0), // l2CallValue
				maxSubmissionCost, // maxSubmissionCost
				refundAddress, // excessFeeRefundAddress
				refundAddress, // callValueRefundAddress
				gasLimit, // gasLimit
				maxFeePerGas, // maxFeePerGas
				l3UpgradeExecutorCalldata, // data
			],
		});
	}

	// Encode call to parent UpgradeExecutor.executeCall()
	const upgradeExecutorCalldata = encodeFunctionData({
		abi: UpgradeExecutorABI,
		functionName: "executeCall",
		args: [parentInboxAddress, inboxCalldata],
	});

	printSection("📝 TRANSACTION DATA");
	print("");
	print("To", parentUpgradeExecutorAddress);
	if (useCustomGasToken) {
		print("Value", "0 wei (custom gas token will be used)");
		formatWeiValue("Token Amount", totalValue, { unit: "tokens" });
	} else {
		formatWeiValue("Value", totalValue);
	}
	print("Calldata", upgradeExecutorCalldata);
	print("");

	printSection("📊 BREAKDOWN");
	print("");
	formatWeiValue("Submission Cost", maxSubmissionCost, {
		unit: useCustomGasToken ? "tokens" : "ETH",
	});
	formatWeiValue("Gas Cost", gasLimit * maxFeePerGas, {
		unit: useCustomGasToken ? "tokens" : "ETH",
	});
	printIndented("Gas Limit", gasLimit.toString());
	printIndented(
		"Max Fee Per Gas",
		`${maxFeePerGas} wei (${Number(maxFeePerGas) / 1e9} gwei)`,
	);
	print("Refund Address", refundAddress);
	print("Custom Gas Token", useCustomGasToken ? "Yes" : "No");
	print("");

	printSection("💡 INSTRUCTIONS");
	if (useCustomGasToken) {
		// EOA needs to transfer tokens to the parent UpgradeExecutor
		const transferCalldata = encodeFunctionData({
			abi: ERC20Abi,
			functionName: "transfer",
			args: [parentUpgradeExecutorAddress, totalValue],
		});

		// UpgradeExecutor needs to approve Inbox (via executeCall on the UpgradeExecutor)
		const inboxApprovalCalldata = encodeFunctionData({
			abi: ERC20Abi,
			functionName: "approve",
			args: [parentInboxAddress, totalValue],
		});

		const upgradeExecutorApprovalCalldata = encodeFunctionData({
			abi: UpgradeExecutorABI,
			functionName: "executeCall",
			args: [customGasTokenAddress, inboxApprovalCalldata],
		});

		print("");
		print("For custom gas token chains, you need to:");
		print("");
		print("1. [EOA → Token] Transfer tokens to the parent UpgradeExecutor:");
		printIndented("Target", customGasTokenAddress);
		printIndented("Value", "0 wei");
		printIndented("Calldata", transferCalldata);
		print("");
		print(
			"2. [UpgradeExecutor → Token] Have the UpgradeExecutor approve Inbox to spend tokens:",
		);
		printIndented("Target", parentUpgradeExecutorAddress);
		printIndented("Value", "0 wei");
		printIndented("Calldata", upgradeExecutorApprovalCalldata);
		print("");
		print("3. [UpgradeExecutor → Inbox] Call the parent UpgradeExecutor:");
		printIndented("Target", parentUpgradeExecutorAddress);
		printIndented("Value", "0 wei (no ETH, uses approved tokens)");
		printIndented("Calldata", upgradeExecutorCalldata);
	} else {
		print("");
		print("Your smart contract should call the parent UpgradeExecutor with:");
		printIndented("Target", parentUpgradeExecutorAddress);
		printIndented(
			"Value",
			`${totalValue} wei (${Number(totalValue) / 1e18} ETH)`,
		);
		printIndented("Calldata", upgradeExecutorCalldata);
	}
	print("");
	print("⚠️  Note: The retryable ticket will need to be redeemed on L3.");
	print("    This usually happens automatically.");
	print("");
	printSeparator();
}
