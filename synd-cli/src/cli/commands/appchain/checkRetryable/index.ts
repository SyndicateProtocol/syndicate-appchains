import { appchainCheckRetryableOptionsSchema } from "@/cli/schema"
import { addInitSubcommand } from "@/utils/addInitCommand"
import {
  getAppchainClients,
  getSupportedChainPublicClient
} from "@/utils/clients"
import { parseConfigAndOptions } from "@/utils/config"
import type { Command } from "@commander-js/extra-typings"
import { checkRetryable } from "./checkRetryable"

export function checkRetryableCommand(program: Command) {
  const checkCmd = program
    .command("check-retryable")
    .description(
      "Check retryable ticket status from a settlement chain transaction"
    )

  addInitSubcommand(
    checkCmd,
    "check-retryable",
    appchainCheckRetryableOptionsSchema
  )

  checkCmd
    .option("--config <path>", "Path to JSON config file")
    .option("--settlement-rpc <url>", "RPC URL for the settlement chain")
    .option("--appchain-rpc <url>", "RPC URL for the appchain")
    .option("--rollup <address>", "Address of the rollup contract")
    .option("--hash <hash>", "Settlement chain transaction hash to check")
    .action(async (options: Record<string, unknown>) => {
      const validatedOptions = parseConfigAndOptions(
        options,
        appchainCheckRetryableOptionsSchema
      )

      const { settlementRpc, appchainRpc, rollup, hash } = validatedOptions

      const settlementPublicClient =
        await getSupportedChainPublicClient(settlementRpc)
      const [appchainPublicClient] = await getAppchainClients(appchainRpc)

      await checkRetryable({
        parentTxHash: hash,
        parentPublicClient: settlementPublicClient,
        childPublicClient: appchainPublicClient,
        rollup
      })
    })
}
