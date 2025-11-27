import { appchainExecuteOutboxOptionsSchema } from "@/cli/schema"
import { addInitSubcommand } from "@/utils/addInitCommand"
import { parseConfigAndOptions } from "@/utils/config"
import type { Command } from "@commander-js/extra-typings"
import { executeOutbox } from "./executeOutbox"

export function executeOutboxCommand(program: Command) {
  const executeCmd = program
    .command("execute-outbox")
    .description(
      "Execute an outbox message from an appchain transaction that triggered a child-to-parent message"
    )

  addInitSubcommand(
    executeCmd,
    "execute-outbox",
    appchainExecuteOutboxOptionsSchema
  )

  executeCmd
    .option("--config <path>", "Path to JSON config file")
    .option("--settlement-rpc <url>", "RPC URL for the settlement chain")
    .option("--appchain-rpc <url>", "RPC URL for the appchain")
    .option("--rollup <address>", "Address of the rollup contract")
    .option(
      "--private-key <key>",
      "Private key for settlement chain transactions"
    )
    .option(
      "--hash <hash>",
      "Appchain transaction hash that sent a child-to-parent message"
    )
    .action(async (options: Record<string, unknown>) => {
      const validatedOptions = parseConfigAndOptions(
        options,
        appchainExecuteOutboxOptionsSchema
      )
      await executeOutbox(validatedOptions)
    })
}
