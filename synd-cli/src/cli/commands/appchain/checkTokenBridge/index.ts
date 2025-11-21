import { getAppchainClient, getPublicClient } from "@/utils/clients"
import { supportedSettlementChains } from "@/utils/constants"
import type { Command } from "@commander-js/extra-typings"
import { appchainCheckTokenBridgeOptionsSchema } from "../../../schema"
import { parseConfigAndOptions } from "@/utils/config"
import { addInitSubcommand } from "@/utils/addInitCommand"
import { checkTokenBridge } from "./checkTokenBridge"

export function checkTokenBridgeCommand(program: Command) {
  const checkCmd = program
    .command("check-token-bridge")
    .description("Check token bridge deployment and retryable ticket execution")

  addInitSubcommand(checkCmd, "check-token-bridge", appchainCheckTokenBridgeOptionsSchema)

  checkCmd
    .option("--config <path>", "Path to JSON config file")
    .option("--settlement-rpc <url>", "RPC URL for the settlement chain")
    .option("--appchain-rpc <url>", "RPC URL for the appchain")
    .option("--rollup <address>", "Address of the rollup contract")
    .option(
      "--created-at-hash <hash>",
      "Transaction hash where token bridge was created"
    )
    .action(async (options: Record<string, unknown>) => {
      const validatedOptions = parseConfigAndOptions(
        options,
        appchainCheckTokenBridgeOptionsSchema
      )

      const { settlementRpc, appchainRpc, rollup, createdAtHash } =
        validatedOptions

      const settlementPublicClient = await getPublicClient(
        settlementRpc,
        supportedSettlementChains
      )

      const appchainPublicClient = await getAppchainClient({
        settlementPublicClient: settlementPublicClient,
        rpcUrl: appchainRpc
      })

      await checkTokenBridge({
        rollup,
        appchainPublicClient,
        settlementPublicClient,
        createdAtHash
      })
    })
}
