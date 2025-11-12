import type { Address, Hex } from "viem";
import { createPublicClient, encodeFunctionData, http } from "viem";
import { ArbOwnerABI } from "../../../abi/nitro/ArbOwner";
import { ERC20InboxABI } from "../../../abi/nitro/ERC20Inbox";
import { InboxABI } from "../../../abi/nitro/Inbox";
import { UpgradeExecutorABI } from "../../../abi/nitro/UpgradeExecutor";
import { applyL1ToL2Alias } from "../../../utils/alias";
import { ARB_OWNER_PRECOMPILE_ADDRESS } from "../../../utils/constants";
import { print } from "../../../utils/print";
import type { CommandSchema, SubcommandDefinition } from "../../types";

interface SetWasmMaxStackDepthParams {
	parentChainRpcUrl: string;
	parentUpgradeExecutorAddress: Address;
	parentInboxAddress: Address;
	l3UpgradeExecutorAddress: Address;
	refundAddress: Address;
	wasmMaxStackDepth: number;
	gasLimit?: bigint;
	maxFeePerGas?: bigint;
	customGasTokenAddress?: Address;
}

async function generateSetWasmMaxStackDepthTx({
	parentChainRpcUrl,
	parentUpgradeExecutorAddress,
	parentInboxAddress,
	l3UpgradeExecutorAddress,
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

	print("🚀 Generating setWasmMaxStackDepth transaction data...");
	print(`Parent UpgradeExecutor: ${parentUpgradeExecutorAddress}`);
	print(`L3 UpgradeExecutor: ${l3UpgradeExecutorAddress}`);
	print(
		`Aliased Parent UpgradeExecutor: ${applyL1ToL2Alias(parentUpgradeExecutorAddress)}`,
	);
	print(`WASM Max Stack Depth: ${wasmMaxStackDepth}\n`);

	// Step 1: Encode call to ArbOwner
	const arbOwnerCalldata = encodeFunctionData({
		abi: ArbOwnerABI,
		functionName: "setWasmMaxStackDepth",
		args: [wasmMaxStackDepth],
	});

	// Step 2: Encode call to L3 UpgradeExecutor.executeCall()
	const l3UpgradeExecutorCalldata = encodeFunctionData({
		abi: UpgradeExecutorABI,
		functionName: "executeCall",
		args: [ARB_OWNER_PRECOMPILE_ADDRESS, arbOwnerCalldata],
	});

	// Calculate submission cost
	const dataLength = BigInt((l3UpgradeExecutorCalldata.length - 2) / 2); // Remove '0x' and divide by 2

	let submissionCost: bigint;
	try {
		submissionCost = await publicClient.readContract({
			address: parentInboxAddress,
			abi: InboxABI,
			functionName: "calculateRetryableSubmissionFee",
			args: [dataLength, BigInt(0)], // 0 means use current basefee
		});

		// If the result is 0, the function might not be working correctly
		if (submissionCost === BigInt(0)) {
			print(
				"⚠️  Calculated submission cost is 0, using formula-based estimate\n",
			);
			// Use Arbitrum's formula: (1400 + 6 * dataLength) * baseFee
			// Assuming a reasonable base fee of 0.1 gwei = 100000000 wei
			const estimatedBaseFee = BigInt(100000000);
			submissionCost =
				(BigInt(1400) + BigInt(6) * dataLength) * estimatedBaseFee;
		}
	} catch (_error) {
		print(
			"⚠️  Could not calculate submission cost, using formula-based estimate\n",
		);
		// Use Arbitrum's formula: (1400 + 6 * dataLength) * baseFee
		const estimatedBaseFee = BigInt(100000000);
		submissionCost = (BigInt(1400) + BigInt(6) * dataLength) * estimatedBaseFee;
	}

	const maxSubmissionCost = (submissionCost * BigInt(150)) / BigInt(100); // Add 50% buffer for safety

	// Calculate total value needed
	const totalValue = maxSubmissionCost + gasLimit * maxFeePerGas;

	// Step 3: Encode call to Inbox (ERC20Inbox for custom gas token, regular Inbox for ETH)
	let inboxCalldata: Hex;
	if (useCustomGasToken) {
		// ERC20Inbox has an extra tokenTotalFeeAmount parameter
		inboxCalldata = encodeFunctionData({
			abi: ERC20InboxABI,
			functionName: "createRetryableTicket",
			args: [
				l3UpgradeExecutorAddress, // to
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
		// Regular Inbox uses payable function
		inboxCalldata = encodeFunctionData({
			abi: InboxABI,
			functionName: "createRetryableTicket",
			args: [
				l3UpgradeExecutorAddress, // to
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

	// Step 4: Encode call to parent UpgradeExecutor.executeCall()
	const upgradeExecutorCalldata = encodeFunctionData({
		abi: UpgradeExecutorABI,
		functionName: "executeCall",
		args: [parentInboxAddress, inboxCalldata],
	});

	print("=".repeat(80));
	print("\n📝 TRANSACTION DATA\n");
	print("=".repeat(80));
	print(`\nTo:        ${parentUpgradeExecutorAddress}`);
	if (useCustomGasToken) {
		print(`Value:     0 wei (custom gas token will be used)`);
		print(
			`Token Amount: ${totalValue} wei (${Number(totalValue) / 1e18} tokens)`,
		);
	} else {
		print(`Value:     ${totalValue} wei`);
		print(`           ${Number(totalValue) / 1e18} ETH`);
		print(`           ${Number(totalValue) / 1e9} gwei`);
	}
	print(`Calldata:  ${upgradeExecutorCalldata}\n`);
	print("=".repeat(80));
	print("\n📊 BREAKDOWN\n");
	print("=".repeat(80));
	print(
		`Submission Cost:     ${maxSubmissionCost} wei (${Number(maxSubmissionCost) / 1e18} tokens)`,
	);
	print(
		`Gas Cost:            ${gasLimit * maxFeePerGas} wei (${Number(gasLimit * maxFeePerGas) / 1e18} tokens)`,
	);
	print(`  Gas Limit:         ${gasLimit}`);
	print(
		`  Max Fee Per Gas:   ${maxFeePerGas} wei (${Number(maxFeePerGas) / 1e9} gwei)`,
	);
	print(`Refund Address:      ${refundAddress}`);
	print(`Custom Gas Token:    ${useCustomGasToken ? "Yes" : "No"}`);
	print(`\n${"=".repeat(80)}`);
	print("\n💡 INSTRUCTIONS\n");
	print("=".repeat(80));
	if (useCustomGasToken) {
		// Generate approval calldatas
		const erc20ApprovalAbi = [
			{
				type: "function",
				name: "approve",
				inputs: [
					{ name: "spender", type: "address" },
					{ name: "amount", type: "uint256" },
				],
				outputs: [{ name: "", type: "bool" }],
				stateMutability: "nonpayable",
			},
		];

		// Step 1: EOA transfers tokens to UpgradeExecutor
		const transferCalldata = encodeFunctionData({
			abi: [
				{
					type: "function",
					name: "transfer",
					inputs: [
						{ name: "to", type: "address" },
						{ name: "amount", type: "uint256" },
					],
					outputs: [{ name: "", type: "bool" }],
					stateMutability: "nonpayable",
				},
			],
			functionName: "transfer",
			args: [parentUpgradeExecutorAddress, totalValue],
		});

		// Step 2: UpgradeExecutor approves Inbox (via executeCall)
		const inboxApprovalCalldata = encodeFunctionData({
			abi: erc20ApprovalAbi,
			functionName: "approve",
			args: [parentInboxAddress, totalValue],
		});

		const upgradeExecutorApprovalCalldata = encodeFunctionData({
			abi: UpgradeExecutorABI,
			functionName: "executeCall",
			args: [customGasTokenAddress, inboxApprovalCalldata],
		});

		print("For custom gas token chains, you need to:");
		print("\n1. [EOA → Token] Transfer tokens to the parent UpgradeExecutor:");
		print(`   Target:   ${customGasTokenAddress}`);
		print(`   Value:    0 wei`);
		print(`   Calldata: ${transferCalldata}`);
		print("");
		print(
			"2. [UpgradeExecutor → Token] Have the UpgradeExecutor approve Inbox to spend tokens:",
		);
		print(`   Target:   ${parentUpgradeExecutorAddress}`);
		print(`   Value:    0 wei`);
		print(`   Calldata: ${upgradeExecutorApprovalCalldata}`);
		print("");
		print("3. [UpgradeExecutor → Inbox] Call the parent UpgradeExecutor:");
		print(`   Target:   ${parentUpgradeExecutorAddress}`);
		print(`   Value:    0 wei (no ETH, uses approved tokens)`);
		print(`   Calldata: ${upgradeExecutorCalldata}`);
	} else {
		print("Your smart contract should call the parent UpgradeExecutor with:");
		print(`  Target:   ${parentUpgradeExecutorAddress}`);
		print(`  Value:    ${totalValue} wei`);
		print(`  Calldata: ${upgradeExecutorCalldata}`);
	}
	print("\n⚠️  Note: The retryable ticket will need to be redeemed on L3.");
	print("    This usually happens automatically.\n");
	print(`${"=".repeat(80)}\n`);
}

/**
 * Schema for setWasmMaxStackDepth subcommand
 */
const setWasmMaxStackDepthSchema = {
	positional: [
		{
			position: 0,
			name: "depth",
			description: "The maximum WASM stack depth",
			type: "number",
			required: true,
		},
	],
	flags: {
		parentRpc: {
			flag: "--parent-rpc",
			description: "Parent chain RPC URL",
			type: "string",
			required: true,
		},
		parentUpgradeExecutor: {
			flag: "--parent-upgrade-executor",
			description: "Parent chain UpgradeExecutor address",
			type: "address",
			required: true,
		},
		parentInbox: {
			flag: "--parent-inbox",
			description: "Parent chain Inbox address",
			type: "address",
			required: true,
		},
		l3UpgradeExecutor: {
			flag: "--l3-upgrade-executor",
			description: "L3 UpgradeExecutor address",
			type: "address",
			required: true,
		},
		refundAddress: {
			flag: "--refund-address",
			description: "Address on L3 to receive excess fees",
			type: "address",
			required: true,
		},
		gasLimit: {
			flag: "--gas-limit",
			description: "Gas limit for retryable ticket",
			type: "bigint",
			default: BigInt(1_000_000),
		},
		maxFeePerGas: {
			flag: "--max-fee-per-gas",
			description: "Max fee per gas in gwei",
			type: "bigint",
			default: BigInt(100000000),
			transform: (value: string) => BigInt(value) * BigInt(1_000_000_000),
		},
		customGasToken: {
			flag: "--custom-gas-token",
			description:
				"Custom gas token contract address for chains using ERC20 gas tokens",
			type: "address",
		},
	},
} as const satisfies CommandSchema;

/**
 * Subcommand definition for setWasmMaxStackDepth
 */
export const setWasmMaxStackDepthSubcommand: SubcommandDefinition<
	typeof setWasmMaxStackDepthSchema
> = {
	name: "setWasmMaxStackDepth",
	description: "Set the WASM max stack depth on L3",
	schema: setWasmMaxStackDepthSchema,
	handler: async (args) => {
		await generateSetWasmMaxStackDepthTx({
			parentChainRpcUrl: args.parentRpc,
			parentUpgradeExecutorAddress: args.parentUpgradeExecutor,
			parentInboxAddress: args.parentInbox,
			l3UpgradeExecutorAddress: args.l3UpgradeExecutor,
			refundAddress: args.refundAddress,
			wasmMaxStackDepth: args.depth,
			gasLimit: args.gasLimit,
			maxFeePerGas: args.maxFeePerGas,
			customGasTokenAddress: args.customGasToken,
		});
	},
	examples: [
		`bun cli configureL3 setWasmMaxStackDepth 22000 \\
  --parent-rpc <RPC_URL> \\
  --parent-upgrade-executor <ADDRESS> \\
  --parent-inbox <ADDRESS> \\
  --l3-upgrade-executor <ADDRESS> \\
  --refund-address <ADDRESS>`,
	],
};
