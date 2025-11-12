import {
  type Address,
  type Chain,
  type Hex,
  type PublicClient,
  type WalletClient,
  createPublicClient,
  createWalletClient,
  encodeFunctionData,
  http,
  parseEther
} from "viem"
import { privateKeyToAccount } from "viem/accounts"
import { ArbOwnerABI } from "../abi/nitro/ArbOwner"
import { InboxABI } from "../abi/nitro/Inbox"
import { UpgradeExecutorABI } from "../abi/nitro/UpgradeExecutor"
import { ARB_OWNER_PRECOMPILE_ADDRESS } from "../utils/constants"
import { print } from "../utils/print"

/**
 * Configuration for cross-chain L3 setup via parent chain's UpgradeExecutor
 */
interface ConfigureL3FromParentParams {
  // Parent chain (L2) configuration
  parentChainRpcUrl: string
  parentUpgradeExecutorAddress: Address
  parentInboxAddress: Address

  // L3 configuration
  l3UpgradeExecutorAddress: Address

  // Retryable ticket parameters
  gasLimit?: bigint
  maxFeePerGas?: bigint
  maxSubmissionCost?: bigint
  refundAddress: Address

  // ArbOwner configuration to set
  arbOwnerConfig: ArbOwnerConfig
}

interface ArbOwnerConfig {
  minimumL2BaseFee?: bigint
  networkFeeAccount?: Address
  infraFeeAccount?: Address
  wasmMaxStackDepth?: number
  speedLimit?: bigint
  maxTxGasLimit?: bigint
  // Add more ArbOwner settings as needed
}

/**
 * Helper to calculate the aliased address for L1->L2 messages
 */
function applyL1ToL2Alias(address: Address): Address {
  const offset = BigInt("0x1111000000000000000000000000000000001111")
  const aliased = (BigInt(address) + offset) & ((1n << 160n) - 1n)
  return `0x${aliased.toString(16).padStart(40, "0")}` as Address
}

/**
 * Configures an L3 chain by calling through the parent chain's UpgradeExecutor
 * to the L3's UpgradeExecutor, which then calls ArbOwner.
 *
 * Flow:
 * 1. Parent UpgradeExecutor.executeCall() -> Inbox
 * 2. Inbox.createRetryableTicket() -> L3 (message gets aliased)
 * 3. Aliased parent UpgradeExecutor calls L3 UpgradeExecutor.executeCall()
 * 4. L3 UpgradeExecutor calls ArbOwner precompile
 */
export async function configureL3FromParent(
  params: ConfigureL3FromParentParams
) {
  const {
    parentChainRpcUrl,
    parentUpgradeExecutorAddress,
    parentInboxAddress,
    l3UpgradeExecutorAddress,
    gasLimit = 1_000_000n,
    maxFeePerGas = 100000000n, // 0.1 gwei default
    refundAddress,
    arbOwnerConfig
  } = params

  // Setup parent chain client (read-only)
  const publicClient = createPublicClient({
    transport: http(parentChainRpcUrl)
  })

  print("🚀 Generating L3 configuration transaction data...")
  print(`Parent UpgradeExecutor: ${parentUpgradeExecutorAddress}`)
  print(`L3 UpgradeExecutor: ${l3UpgradeExecutorAddress}`)
  print(
    `Aliased Parent UpgradeExecutor: ${applyL1ToL2Alias(parentUpgradeExecutorAddress)}`
  )

  // Build the configuration calls
  const configCalls: { name: string; calldata: Hex }[] = []

  if (arbOwnerConfig.wasmMaxStackDepth !== undefined) {
    configCalls.push({
      name: "setWasmMaxStackDepth",
      calldata: encodeFunctionData({
        abi: ArbOwnerABI,
        functionName: "setWasmMaxStackDepth",
        args: [arbOwnerConfig.wasmMaxStackDepth]
      })
    })
  }

  print(`\n📋 Generating calldata for ${configCalls.length} ArbOwner setting(s)...\n`)

  // Generate transaction data for each configuration call
  for (const call of configCalls) {
    print(`⚙️  ${call.name}\n`)

    // Step 1: Encode call to ArbOwner
    const arbOwnerCalldata = call.calldata

    // Step 2: Encode call to L3 UpgradeExecutor.executeCall()
    const l3UpgradeExecutorCalldata = encodeFunctionData({
      abi: UpgradeExecutorABI,
      functionName: "executeCall",
      args: [ARB_OWNER_PRECOMPILE_ADDRESS, arbOwnerCalldata]
    })

    // Calculate submission cost
    const dataLength = BigInt(
      (l3UpgradeExecutorCalldata.length - 2) / 2
    ) // Remove '0x' and divide by 2

    let submissionCost: bigint
    try {
      submissionCost = await publicClient.readContract({
        address: parentInboxAddress,
        abi: InboxABI,
        functionName: "calculateRetryableSubmissionFee",
        args: [dataLength, 0n] // 0 means use current basefee
      })

      // If the result is 0, the function might not be working correctly
      if (submissionCost === 0n) {
        print("⚠️  Calculated submission cost is 0, using formula-based estimate\n")
        // Use Arbitrum's formula: (1400 + 6 * dataLength) * baseFee
        // Assuming a reasonable base fee of 0.1 gwei = 100000000 wei
        const estimatedBaseFee = 100000000n
        submissionCost = (1400n + 6n * dataLength) * estimatedBaseFee
      }
    } catch (error) {
      print("⚠️  Could not calculate submission cost, using formula-based estimate\n")
      // Use Arbitrum's formula: (1400 + 6 * dataLength) * baseFee
      const estimatedBaseFee = 100000000n
      submissionCost = (1400n + 6n * dataLength) * estimatedBaseFee
    }

    const maxSubmissionCost = (submissionCost * 150n) / 100n // Add 50% buffer for safety

    // Step 3: Encode call to Inbox.createRetryableTicket()
    const inboxCalldata = encodeFunctionData({
      abi: InboxABI,
      functionName: "createRetryableTicket",
      args: [
        l3UpgradeExecutorAddress, // to
        0n, // l2CallValue
        maxSubmissionCost, // maxSubmissionCost
        refundAddress, // excessFeeRefundAddress
        refundAddress, // callValueRefundAddress
        gasLimit, // gasLimit
        maxFeePerGas, // maxFeePerGas
        l3UpgradeExecutorCalldata // data
      ]
    })

    // Step 4: Encode call to parent UpgradeExecutor.executeCall()
    const upgradeExecutorCalldata = encodeFunctionData({
      abi: UpgradeExecutorABI,
      functionName: "executeCall",
      args: [parentInboxAddress, inboxCalldata]
    })

    // Calculate total value needed
    const totalValue = maxSubmissionCost + gasLimit * maxFeePerGas

    print("=" .repeat(80))
    print("\n📝 TRANSACTION DATA\n")
    print("=" .repeat(80))
    print(`\nTo:        ${parentUpgradeExecutorAddress}`)
    print(`Value:     ${totalValue} wei`)
    print(`           ${Number(totalValue) / 1e18} ETH`)
    print(`           ${Number(totalValue) / 1e9} gwei`)
    print(`Calldata:  ${upgradeExecutorCalldata}\n`)
    print("=" .repeat(80))
    print("\n📊 BREAKDOWN\n")
    print("=" .repeat(80))
    print(`Submission Cost:     ${maxSubmissionCost} wei (${Number(maxSubmissionCost) / 1e18} ETH)`)
    print(`Gas Cost:            ${gasLimit * maxFeePerGas} wei (${Number(gasLimit * maxFeePerGas) / 1e18} ETH)`)
    print(`  Gas Limit:         ${gasLimit}`)
    print(`  Max Fee Per Gas:   ${maxFeePerGas} wei (${Number(maxFeePerGas) / 1e9} gwei)`)
    print(`Refund Address:      ${refundAddress}`)
    print("\n" + "=" .repeat(80))
    print("\n💡 INSTRUCTIONS\n")
    print("=" .repeat(80))
    print("Your smart contract should call the parent UpgradeExecutor with:")
    print(`  - Target: ${parentUpgradeExecutorAddress}`)
    print(`  - Value:  ${totalValue} wei`)
    print(`  - Data:   ${upgradeExecutorCalldata}`)
    print("\n⚠️  Note: The retryable ticket will need to be redeemed on L3.")
    print("    This usually happens automatically.\n")
    print("=" .repeat(80) + "\n")
  }
}

// Print help message
function printHelp() {
  console.log(`
L3 Configuration CLI

USAGE:
  bun run cli <COMMAND> <VALUE> [OPTIONS]

COMMANDS:
  setWasmMaxStackDepth <DEPTH>
                              Set the WASM max stack depth on L3 via parent chain's UpgradeExecutor

OPTIONS:
  --help                      Show this help message

  --parent-rpc <URL>          Parent chain RPC URL (required)
  --parent-upgrade-executor <ADDRESS>
                              Parent chain UpgradeExecutor address (required)
  --parent-inbox <ADDRESS>    Parent chain Inbox address (required)
  --l3-upgrade-executor <ADDRESS>
                              L3 UpgradeExecutor address (required)
  --refund-address <ADDRESS>  Address on L3 to receive excess fees (required)

  --gas-limit <LIMIT>         Gas limit for retryable ticket (optional, default: 1000000)
  --max-fee-per-gas <GWEI>    Max fee per gas in gwei (optional, default: 0.1)

EXAMPLES:
  # Generate transaction data to set WASM max stack depth to 22000
  bun run cli setWasmMaxStackDepth 22000 \\
    --parent-rpc https://sepolia.base.org \\
    --parent-upgrade-executor 0x1234... \\
    --parent-inbox 0x5678... \\
    --l3-upgrade-executor 0x9abc... \\
    --refund-address 0xdef0...

DESCRIPTION:
  This CLI tool generates the transaction data needed to configure an L3 chain
  by calling through the parent chain's UpgradeExecutor to the L3's UpgradeExecutor,
  which then calls the ArbOwner precompile.

  The script outputs the target address, value, and calldata that your smart
  contract should use to call the parent UpgradeExecutor.

  Flow:
  1. Your contract -> Parent UpgradeExecutor.executeCall() -> Inbox
  2. Inbox.createRetryableTicket() -> L3 (message gets aliased)
  3. Aliased parent UpgradeExecutor calls L3 UpgradeExecutor.executeCall()
  4. L3 UpgradeExecutor calls ArbOwner precompile
`)
}

// Parse CLI arguments
function parseArgs() {
  const args = process.argv.slice(2)

  // Check for help flag
  if (args.includes("--help") || args.includes("-h") || args.length === 0) {
    printHelp()
    process.exit(0)
  }

  const getArg = (flag: string): string | undefined => {
    const index = args.indexOf(flag)
    return index !== -1 && args[index + 1] ? args[index + 1] : undefined
  }

  const command = args[0]
  const commandValue = args[1]

  if (command !== "setWasmMaxStackDepth") {
    console.error(`❌ Unknown command: ${command}`)
    console.error("\nAvailable commands: setWasmMaxStackDepth <DEPTH>")
    console.error("\nRun 'bun run cli --help' for more information")
    process.exit(1)
  }

  if (!commandValue || commandValue.startsWith("--")) {
    console.error(`❌ Missing value for setWasmMaxStackDepth`)
    console.error("\nUsage: bun run cli setWasmMaxStackDepth <DEPTH> [OPTIONS]")
    process.exit(1)
  }

  const wasmMaxStackDepth = commandValue
  const parentRpc = getArg("--parent-rpc")
  const parentUpgradeExecutor = getArg("--parent-upgrade-executor")
  const parentInbox = getArg("--parent-inbox")
  const l3UpgradeExecutor = getArg("--l3-upgrade-executor")
  const refundAddress = getArg("--refund-address")
  const gasLimit = getArg("--gas-limit")
  const maxFeePerGas = getArg("--max-fee-per-gas")

  // Validate required arguments
  const required = {
    "--parent-rpc": parentRpc,
    "--parent-upgrade-executor": parentUpgradeExecutor,
    "--parent-inbox": parentInbox,
    "--l3-upgrade-executor": l3UpgradeExecutor,
    "--refund-address": refundAddress
  }

  const missing = Object.entries(required)
    .filter(([_, value]) => !value)
    .map(([key]) => key)

  if (missing.length > 0) {
    console.error(`Missing required arguments: ${missing.join(", ")}`)
    console.error(`
Usage: bun run cli setWasmMaxStackDepth <DEPTH> \\
  --parent-rpc <RPC_URL> \\
  --parent-upgrade-executor <ADDRESS> \\
  --parent-inbox <ADDRESS> \\
  --l3-upgrade-executor <ADDRESS> \\
  --refund-address <ADDRESS> \\
  [--gas-limit <GAS_LIMIT>] \\
  [--max-fee-per-gas <GWEI>]
`)
    process.exit(1)
  }

  return {
    parentRpc: parentRpc!,
    parentUpgradeExecutor: parentUpgradeExecutor! as Address,
    parentInbox: parentInbox! as Address,
    l3UpgradeExecutor: l3UpgradeExecutor! as Address,
    refundAddress: refundAddress! as Address,
    wasmMaxStackDepth: Number(wasmMaxStackDepth!),
    gasLimit: gasLimit ? BigInt(gasLimit) : undefined,
    maxFeePerGas: maxFeePerGas ? BigInt(maxFeePerGas) * 1_000_000_000n : undefined // Convert gwei to wei
  }
}

async function main() {
  const args = parseArgs()

  await configureL3FromParent({
    parentChainRpcUrl: args.parentRpc,
    parentUpgradeExecutorAddress: args.parentUpgradeExecutor,
    parentInboxAddress: args.parentInbox,
    l3UpgradeExecutorAddress: args.l3UpgradeExecutor,
    refundAddress: args.refundAddress,
    gasLimit: args.gasLimit,
    maxFeePerGas: args.maxFeePerGas,
    arbOwnerConfig: {
      wasmMaxStackDepth: args.wasmMaxStackDepth
    }
  })
}

// Run if executed directly
if (import.meta.main) {
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error("🚫", error)
      process.exit(1)
    })
}
