import { aliasCommand } from "./commands/alias"
import { configureL3Command } from "./commands/configureL3"

function printHelp() {
  console.log(`
L3 Configuration CLI

USAGE:
  bun cli <COMMAND> [SUBCOMMAND] [OPTIONS]

COMMANDS:
  alias <ADDRESS>             Calculate the aliased address for L1->L2 messages

  configureL3 <SUBCOMMAND>    Configure L3 chain via parent chain's UpgradeExecutor
    setWasmMaxStackDepth <DEPTH> [OPTIONS]
                              Set the WASM max stack depth on L3

OPTIONS:
  --help, -h                  Show this help message

EXAMPLES:
  # Calculate aliased address
  bun cli alias <ADDRESS>

  # Configure WASM max stack depth
  bun cli configureL3 setWasmMaxStackDepth 22000 \\
    --parent-rpc <RPC_URL> \\
    --parent-upgrade-executor <ADDRESS> \\
    --parent-inbox <ADDRESS> \\
    --l3-upgrade-executor <ADDRESS> \\
    --refund-address <ADDRESS> \\
    --gas-limit <GAS_LIMIT (optional, default: 1000000)> \\
    --max-fee-per-gas <GWEI (optional, default: 0.1)> \\
    --custom-gas-token <ADDRESS (optional, default: undefined)>

CONFIGURE L3 OPTIONS:
  --parent-rpc <URL>          Parent chain RPC URL (required)
  --parent-upgrade-executor <ADDRESS>
                              Parent chain UpgradeExecutor address (required)
  --parent-inbox <ADDRESS>    Parent chain Inbox address (required)
  --l3-upgrade-executor <ADDRESS>
                              L3 UpgradeExecutor address (required)
  --refund-address <ADDRESS>  Address on L3 to receive excess fees (required)
  --gas-limit <LIMIT>         Gas limit for retryable ticket (optional, default: 1000000)
  --max-fee-per-gas <GWEI>    Max fee per gas in gwei (optional, default: 0.1)
  --custom-gas-token <ADDRESS>
                              Custom gas token contract address for chains using ERC20 gas tokens
                              (optional, generates approval calldata when provided)
`)
}

async function main() {
  const args = process.argv.slice(2)

  // Check for help flag
  if (args.includes("--help") || args.includes("-h") || args.length === 0) {
    printHelp()
    process.exit(0)
  }

  const command = args[0]
  const commandArgs = args.slice(1)

  switch (command) {
    case "alias":
      await aliasCommand(commandArgs)
      break
    case "configureL3":
      await configureL3Command(commandArgs)
      break
    default:
      console.error(`❌ Unknown command: ${command}`)
      console.error("\nAvailable commands: alias, configureL3")
      console.error("\nRun 'bun cli --help' for more information")
      process.exit(1)
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("🚫", error)
    process.exit(1)
  })
