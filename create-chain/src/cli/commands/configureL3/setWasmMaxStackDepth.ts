import { ERC20Abi } from "@/src/abi/ERC20";
import { getNativeCurrency } from "@/src/utils/helpers";
import type { Address, Hex } from "viem";
import {
	createPublicClient,
	encodeFunctionData,
	formatEther,
	formatGwei,
	formatUnits,
	http,
} from "viem";
import { ArbOwnerABI } from "../../../abi/nitro/ArbOwner";
import { ERC20InboxABI } from "../../../abi/nitro/ERC20Inbox";
import { InboxABI } from "../../../abi/nitro/Inbox";
import { UpgradeExecutorABI } from "../../../abi/nitro/UpgradeExecutor";
import { ARB_OWNER_PRECOMPILE_ADDRESS } from "../../../utils/constants";
import {
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
	gasLimit = BigInt(50_000),
	maxFeePerGas = BigInt(100_000_000), // 0.1 gwei
	refundAddress,
	customGasTokenAddress,
	wasmMaxStackDepth,
}: SetWasmMaxStackDepthParams) {
	const useCustomGasToken = !!customGasTokenAddress;
	const publicClient = createPublicClient({
		transport: http(parentChainRpcUrl),
	});
	const nativeCurrency = useCustomGasToken
		? await getNativeCurrency(publicClient, customGasTokenAddress)
		: undefined;

	// Get calldata for calling setWasmMaxStackDepth through the UpgradeExecutor
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
	// For ERC20 chains, submission cost is always 0 (ERC20Inbox.calculateRetryableSubmissionFee returns 0)
	// https://github.com/OffchainLabs/nitro-contracts/blob/c32af127fe6a9124316abebbf756609649ede1f5/src/bridge/ERC20Inbox.sol#L114-L120
	const dataLength = BigInt((l3UpgradeExecutorCalldata.length - 2) / 2); // Remove '0x' and divide by 2
	let submissionCost: bigint;

	if (useCustomGasToken) {
		// For ERC20 chains, submission cost is always 0
		// https://github.com/OffchainLabs/nitro-contracts/blob/c32af127fe6a9124316abebbf756609649ede1f5/src/bridge/ERC20Inbox.sol#L118-L119
		submissionCost = BigInt(0);
	} else {
		// For ETH chains, calculate the submission cost
		try {
			const block = await publicClient.getBlock();
			// Default to 0.1 gwei if baseFeePerGas is not set
			const baseFeePerGas = block.baseFeePerGas ?? BigInt(100_000_000);
			submissionCost = await publicClient.readContract({
				address: parentInboxAddress,
				abi: InboxABI,
				functionName: "calculateRetryableSubmissionFee",
				args: [dataLength, baseFeePerGas],
			});

			if (submissionCost === BigInt(0)) {
				throw new Error("Inbox returned 0 for submission cost");
			}
		} catch (_) {
			console.warn(
				"Could not calculate submission cost, using formula-based estimate",
			);
			// Fallback to hardcoded estimate from Inbox's calculateRetryableSubmissionFee()
			// https://github.com/OffchainLabs/nitro-contracts/blob/c32af127fe6a9124316abebbf756609649ede1f5/src/bridge/Inbox.sol#L309-L310
			// Assuming a reasonable base fee of 0.1 gwei = 100_000_000 wei
			const estimatedBaseFee = BigInt(100_000_000);
			submissionCost =
				(BigInt(1400) + BigInt(6) * dataLength) * estimatedBaseFee;
		}
	}

	// Add 50% buffer to total submission cost for safety
	const maxSubmissionCost = (submissionCost * BigInt(150)) / BigInt(100);
	const totalValue = maxSubmissionCost + gasLimit * maxFeePerGas;

	// Encode call to Inbox
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

	const tokenAmount = formatUnits(totalValue, nativeCurrency?.decimals || 18);
	const tokenSymbol = nativeCurrency?.symbol || "tokens";

	printSection("📊 BREAKDOWN");
	print("");

	!useCustomGasToken &&
		print("Ticket Submission Cost", `${formatEther(maxSubmissionCost)} ETH`);
	// print("Appchain TX Gas Limit", gasLimit.toString());
	// print("Appchain TX Max Fee Per Gas", `${formatGwei(maxFeePerGas)} gwei`);
	print(
		"Appchain Tx Transaction Cost",
		`${formatEther(gasLimit * maxFeePerGas)} ${useCustomGasToken ? nativeCurrency?.symbol || "tokens" : "ETH"}`,
	);
	printIndented("Gas Limit", gasLimit.toString());
	printIndented("Max Fee Per Gas", `${formatGwei(maxFeePerGas)} gwei`);
	!useCustomGasToken &&
		print("Total Cost To Execute", `${formatEther(totalValue)} ETH`);
	print("Refund Address", refundAddress);
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
		print(
			`1. [EOA → Token] Transfer ${tokenAmount} ${tokenSymbol} to the parent UpgradeExecutor:`,
		);
		printIndented("Target", customGasTokenAddress);
		!useCustomGasToken && printIndented("Value", "0");
		printIndented("Calldata", transferCalldata);
		print("");
		print(
			`2. [UpgradeExecutor → Token] Have the UpgradeExecutor approve Inbox to spend ${tokenAmount} ${tokenSymbol}:`,
		);
		printIndented("Target", parentUpgradeExecutorAddress);
		!useCustomGasToken && printIndented("Value", "0");
		printIndented("Calldata", upgradeExecutorApprovalCalldata);
		print("");
		print("3. [UpgradeExecutor → Inbox] Call the parent UpgradeExecutor:");
		printIndented("Target", parentUpgradeExecutorAddress);
		!useCustomGasToken &&
			printIndented("Value", `0 (no ETH, uses approved ${tokenSymbol})`);
		printIndented("Calldata", upgradeExecutorCalldata);
	} else {
		print("");
		printIndented("Target", parentUpgradeExecutorAddress);
		printIndented("Value", totalValue.toString());
		printIndented("Calldata", upgradeExecutorCalldata);
	}
	print("");
	print("⚠️  Note: The retryable ticket will need to be redeemed on L3.");
	print("    This usually happens automatically.");
	print("");
	printSeparator();
}
