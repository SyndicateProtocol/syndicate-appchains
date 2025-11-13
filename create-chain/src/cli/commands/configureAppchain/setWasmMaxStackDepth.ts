import { ERC20Abi } from "@/src/abi/ERC20";
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
import { NodeInterfaceABI } from "../../../abi/nitro/NodeInterface";
import { UpgradeExecutorABI } from "../../../abi/nitro/UpgradeExecutor";
import {
	ARB_OWNER_PRECOMPILE_ADDRESS,
	NODE_INTERFACE_ADDRESS,
} from "../../../utils/constants";
import { scaleByPercentage } from "../../../utils/helpers";
import {
	print,
	printIndented,
	printSection,
	printSeparator,
} from "../../../utils/print";
import { detectCustomNativeToken } from "./detectCustomNativeToken";

/*
Flow:
	Settlement Chain (Parent): EXECUTOR_ROLE (EOA or Smart Contract) → ParentUpgradeExecutor → ParentInbox
                          ↓
                  (creates retryable ticket)
                          ↓
	Appchain (Child):  Retryable auto-redeems → ChildUpgradeExecutor → ArbOwner.setWasmMaxStackDepth()
*/

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

const DEFAULT_GAS_LIMIT = BigInt(100_000);
const DEFAULT_MAX_FEE_PER_GAS = BigInt(100_000_000); // 0.1 gwei

export async function generateSetWasmMaxStackDepthTx({
	parentRpc,
	childRpc,
	parentUpgradeExecutor,
	parentInbox,
	childUpgradeExecutor,
	gasLimit,
	maxFeePerGas,
	refundAddress,
	wasmMaxStackDepth,
}: SetWasmMaxStackDepthParams) {
	const publicClient = createPublicClient({
		transport: http(parentRpc),
	});

	const customNativeToken = await detectCustomNativeToken(
		publicClient,
		parentInbox,
	);
	const useCustomGasToken = !!customNativeToken;

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

	let estimatedGasLimit: bigint;
	let estimatedMaxFeePerGas: bigint;

	if (childRpc) {
		const childPublicClient = createPublicClient({
			transport: http(childRpc),
		});

		try {
			// Use NodeInterface to estimate the full retryable ticket execution
			estimatedGasLimit = await childPublicClient.estimateGas({
				to: NODE_INTERFACE_ADDRESS,
				data: encodeFunctionData({
					abi: NodeInterfaceABI,
					functionName: "estimateRetryableTicket",
					args: [
						parentUpgradeExecutor, // sender (will be aliased from parent)
						BigInt(0), // deposit (not needed for estimation)
						childUpgradeExecutor, // to
						BigInt(0), // l2CallValue
						refundAddress, // excessFeeRefundAddress
						refundAddress, // callValueRefundAddress
						l3UpgradeExecutorCalldata, // data - the actual executeCall data
					],
				}),
			});

			// maxFeePerGas of 1 is a magic value in arbitrum but is a valid setting for a syndicate appchain. We add 1 here to avoid reverting.
			// https://github.com/OffchainLabs/nitro-contracts/blob/c32af127fe6a9124316abebbf756609649ede1f5/src/bridge/AbsInbox.sol#L276-L277
			estimatedMaxFeePerGas = await childPublicClient.getGasPrice();
			if (estimatedMaxFeePerGas === BigInt(1)) {
				estimatedMaxFeePerGas += BigInt(1);
			}

			// Add 50% buffer to total submission cost to increase the chances of auto-redemption of the retryable ticket
			estimatedMaxFeePerGas = scaleByPercentage(estimatedMaxFeePerGas, 150);

			// Add 20% buffer to gas limit for safety
			estimatedGasLimit = scaleByPercentage(estimatedGasLimit, 120);
		} catch (error) {
			console.warn(
				"⚠️  Could not estimate gas from child chain, using defaults",
			);
			console.warn("Error:", error);
			estimatedGasLimit = gasLimit ?? DEFAULT_GAS_LIMIT;
			estimatedMaxFeePerGas = maxFeePerGas ?? DEFAULT_MAX_FEE_PER_GAS;
		}
	} else {
		estimatedGasLimit = gasLimit ?? DEFAULT_GAS_LIMIT;
		estimatedMaxFeePerGas = maxFeePerGas ?? DEFAULT_MAX_FEE_PER_GAS;
		if (!gasLimit || !maxFeePerGas) {
			print("");
			print(
				"ℹ️  Using default gas parameters. Pass --child-rpc to estimate from chain.",
			);
		}
	}

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
		// For ETH chains, calculate the retryable ticket submission cost
		try {
			const block = await publicClient.getBlock();
			const baseFeePerGas = block.baseFeePerGas ?? DEFAULT_MAX_FEE_PER_GAS;
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
			submissionCost =
				(BigInt(1400) + BigInt(6) * dataLength) * DEFAULT_MAX_FEE_PER_GAS;
		}
	}

	// Add 50% buffer to total submission cost for safety
	const maxSubmissionCost = scaleByPercentage(submissionCost, 150);
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

	const tokenAmount = formatUnits(
		totalValue,
		customNativeToken?.decimals || 18,
	);
	const tokenSymbol = customNativeToken?.symbol || "tokens";

	printSection("📊 BREAKDOWN");
	print("");

	!useCustomGasToken &&
		print("Ticket Submission Cost", `${formatEther(maxSubmissionCost)} ETH`);
	print(
		"Appchain Tx Transaction Cost",
		`${formatEther(estimatedGasLimit * estimatedMaxFeePerGas)} ${useCustomGasToken ? customNativeToken?.symbol || "tokens" : "ETH"}`,
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
			args: [customNativeToken.address, inboxApprovalCalldata],
		});

		print("");
		print(
			`1. [EOA → Token] Transfer ${tokenAmount} ${tokenSymbol} to the parent UpgradeExecutor:`,
		);
		printIndented("Target", customNativeToken.address);
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
