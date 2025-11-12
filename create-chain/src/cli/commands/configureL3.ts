import type { Address, Hex } from "viem"
import {
  createPublicClient,
  encodeFunctionData,
  http,
  parseEther
} from "viem"
import { ArbOwnerABI } from "../../abi/nitro/ArbOwner"
import { InboxABI } from "../../abi/nitro/Inbox"
import { UpgradeExecutorABI } from "../../abi/nitro/UpgradeExecutor"
import { ARB_OWNER_PRECOMPILE_ADDRESS } from "../../utils/constants"
import { print } from "../../utils/print"
import { applyL1ToL2Alias } from "../../utils/alias"

interface ConfigureL3Params {
  parentChainRpcUrl: string
  parentUpgradeExecutorAddress: Address
  parentInboxAddress: Address
  l3UpgradeExecutorAddress: Address
  refundAddress: Address
  gasLimit?: bigint
  maxFeePerGas?: bigint
  arbOwnerConfig: {
    wasmMaxStackDepth?: number
  }
}

async function generateConfigureTx(params: ConfigureL3Params) {
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

export async function configureL3Command(args: string[]) {
  const getArg = (flag: string): string | undefined => {
    const index = args.indexOf(flag)
    return index !== -1 && args[index + 1] ? args[index + 1] : undefined
  }

  const subCommand = args[0]
  const commandValue = args[1]

  if (subCommand !== "setWasmMaxStackDepth") {
    console.error(`❌ Unknown configureL3 subcommand: ${subCommand}`)
    console.error("\nAvailable subcommands: setWasmMaxStackDepth <DEPTH>")
    process.exit(1)
  }

  if (!commandValue || commandValue.startsWith("--")) {
    console.error(`❌ Missing value for setWasmMaxStackDepth`)
    console.error("\nUsage: bun cli configureL3 setWasmMaxStackDepth <DEPTH> [OPTIONS]")
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
Usage: bun cli configureL3 setWasmMaxStackDepth <DEPTH> \\
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

  await generateConfigureTx({
    parentChainRpcUrl: parentRpc!,
    parentUpgradeExecutorAddress: parentUpgradeExecutor! as Address,
    parentInboxAddress: parentInbox! as Address,
    l3UpgradeExecutorAddress: l3UpgradeExecutor! as Address,
    refundAddress: refundAddress! as Address,
    gasLimit: gasLimit ? BigInt(gasLimit) : undefined,
    maxFeePerGas: maxFeePerGas ? BigInt(maxFeePerGas) * 1_000_000_000n : undefined,
    arbOwnerConfig: {
      wasmMaxStackDepth: Number(wasmMaxStackDepth!)
    }
  })
}
