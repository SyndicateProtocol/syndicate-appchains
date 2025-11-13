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
	parentRpc: string;
	childRpc?: string;
	parentUpgradeExecutor: Address;
	parentInbox: Address;
	childUpgradeExecutor: Address;
	refundAddress: Address;
	wasmMaxStackDepth: number;
	gasLimit?: bigint;
	maxFeePerGas?: bigint;
	customGasTokenAddress?: Address;
}

export async function generateSetWasmMaxStackDepthTx({
	parentRpc,
	childRpc,
	parentUpgradeExecutor,
	parentInbox,
	childUpgradeExecutor,
	gasLimit,
	maxFeePerGas,
	refundAddress,
	customGasTokenAddress,
	wasmMaxStackDepth,
}: SetWasmMaxStackDepthParams) {
	const useCustomGasToken = !!customGasTokenAddress;
	const publicClient = createPublicClient({
		transport: http(parentRpc),
	});
	const nativeCurrency = useCustomGasToken
		? await getNativeCurrency(publicClient, customGasTokenAddress)
		: undefined;

	let estimatedGasLimit: bigint;
	let estimatedMaxFeePerGas: bigint;

	if (childRpc) {
		const childPublicClient = createPublicClient({
			transport: http(childRpc),
		});

		const arbOwnerCalldata = encodeFunctionData({
			abi: ArbOwnerABI,
			functionName: "setWasmMaxStackDepth",
			args: [wasmMaxStackDepth],
		});

		try {
			estimatedGasLimit = await childPublicClient.estimateGas({
				account: childUpgradeExecutor,
				to: ARB_OWNER_PRECOMPILE_ADDRESS,
				data: arbOwnerCalldata,
			});
			estimatedMaxFeePerGas = await childPublicClient.getGasPrice();

			// Add 20% buffer to gas limit for safety
			estimatedGasLimit = (estimatedGasLimit * BigInt(120)) / BigInt(100);
		} catch (error) {
			console.warn(
				"⚠️  Could not estimate gas from child chain, using defaults",
			);
			console.warn("Error:", error);
			estimatedGasLimit = gasLimit ?? BigInt(50_000);
			estimatedMaxFeePerGas = maxFeePerGas ?? BigInt(100_000_000); // 0.1 gwei
		}
	} else {
		estimatedGasLimit = gasLimit ?? BigInt(50_000);
		estimatedMaxFeePerGas = maxFeePerGas ?? BigInt(100_000_000); // 0.1 gwei
		if (!gasLimit || !maxFeePerGas) {
			print("");
			print(
				"ℹ️  Using default gas parameters. Pass --child-rpc to estimate from chain.",
			);
		}
	}

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
		const estimatedBaseFee = BigInt(100_000_000); // 0.1 gwei

		// For ETH chains, calculate the retryable ticket submission cost
		try {
			const block = await publicClient.getBlock();
			const baseFeePerGas = block.baseFeePerGas ?? estimatedBaseFee;
			submissionCost = await publicClient.readContract({
				address: parentInbox,
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
			submissionCost =
				(BigInt(1400) + BigInt(6) * dataLength) * estimatedBaseFee;
		}
	}

	// Add 50% buffer to total submission cost for safety
	const maxSubmissionCost = (submissionCost * BigInt(150)) / BigInt(100);
	const totalValue =
		maxSubmissionCost + estimatedGasLimit * estimatedMaxFeePerGas;

	let inboxCalldata: Hex;
	if (useCustomGasToken) {
		// https://github.com/OffchainLabs/nitro-contracts/blob/c32af127fe6a9124316abebbf756609649ede1f5/src/bridge/ERC20Inbox.sol#L64-L65
		inboxCalldata = encodeFunctionData({
			abi: ERC20InboxABI,
			functionName: "createRetryableTicket",
			args: [
				childUpgradeExecutor, // to
				BigInt(0), // l2CallValue
				maxSubmissionCost, // maxSubmissionCost
				refundAddress, // excessFeeRefundAddress
				refundAddress, // callValueRefundAddress
				estimatedGasLimit, // gasLimit
				estimatedMaxFeePerGas, // maxFeePerGas
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
				childUpgradeExecutor, // to
				BigInt(0), // l2CallValue
				maxSubmissionCost, // maxSubmissionCost
				refundAddress, // excessFeeRefundAddress
				refundAddress, // callValueRefundAddress
				estimatedGasLimit, // gasLimit
				estimatedMaxFeePerGas, // maxFeePerGas
				l3UpgradeExecutorCalldata, // data
			],
		});
	}

	const upgradeExecutorCalldata = encodeFunctionData({
		abi: UpgradeExecutorABI,
		functionName: "executeCall",
		args: [parentInbox, inboxCalldata],
	});

	const tokenAmount = formatUnits(totalValue, nativeCurrency?.decimals || 18);
	const tokenSymbol = nativeCurrency?.symbol || "tokens";

	printSection("📊 BREAKDOWN");
	print("");

	!useCustomGasToken &&
		print("Ticket Submission Cost", `${formatEther(maxSubmissionCost)} ETH`);
	print(
		"Appchain Tx Transaction Cost",
		`${formatEther(estimatedGasLimit * estimatedMaxFeePerGas)} ${useCustomGasToken ? nativeCurrency?.symbol || "tokens" : "ETH"}`,
	);
	printIndented("Max Fee Per Gas", `${formatGwei(estimatedMaxFeePerGas)} gwei`);
	printIndented("Gas Limit", estimatedGasLimit.toString());
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
			args: [parentUpgradeExecutor, totalValue],
		});

		// UpgradeExecutor needs to approve Inbox (via executeCall on the UpgradeExecutor)
		const inboxApprovalCalldata = encodeFunctionData({
			abi: ERC20Abi,
			functionName: "approve",
			args: [parentInbox, totalValue],
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
		printIndented("Target", parentUpgradeExecutor);
		!useCustomGasToken && printIndented("Value", "0");
		printIndented("Calldata", upgradeExecutorApprovalCalldata);
		print("");
		print("3. [UpgradeExecutor → Inbox] Call the parent UpgradeExecutor:");
		printIndented("Target", parentUpgradeExecutor);
		!useCustomGasToken &&
			printIndented("Value", `0 (no ETH, uses approved ${tokenSymbol})`);
		printIndented("Calldata", upgradeExecutorCalldata);
	} else {
		print("");
		printIndented("Target", parentUpgradeExecutor);
		printIndented("Value", totalValue.toString());
		printIndented("Calldata", upgradeExecutorCalldata);
	}
	print("");
	print("⚠️  Note: The retryable ticket will need to be redeemed on L3.");
	print("    This usually happens automatically.");
	print("");
	printSeparator();
}
