import { appchainE2EOptionsSchema } from "@/cli/schema"
import { parseConfigAndOptions } from "@/utils/config"
import { addInitSubcommand } from "@/utils/addInitCommand"
import {
  getAppchainClient,
  getPublicClient,
  getWalletClient
} from "@/utils/clients"
import { supportedSettlementChains } from "@/utils/constants"
import type { Command } from "@commander-js/extra-typings"
import { createWalletClient, http } from "viem"
import { privateKeyToAccount } from "viem/accounts"
import { e2e } from "./e2e"

export function e2eCommand(program: Command) {
  const e2eCmd = program
    .command("e2e")
    .description("Run end-to-end tests on an appchain")

  addInitSubcommand(e2eCmd, "e2e", appchainE2EOptionsSchema)

  e2eCmd
    .option("--config <path>", "Path to JSON config file")
    .option("--settlement-rpc <url>", "Settlement chain RPC URL")
    .option("--appchain-rpc <url>", "Appchain RPC URL")
    .option("--inbox <address>", "Inbox contract address")
    .option("--private-key <key>", "Private key for transactions")
    .action(async (options: Record<string, unknown>) => {
      const validatedOptions = parseConfigAndOptions(
        options,
        appchainE2EOptionsSchema
      )

      const { settlementRpc, appchainRpc, privateKey } = validatedOptions

      const [settlementPublicClient, settlementWalletClient] =
        await Promise.all([
          getPublicClient(settlementRpc, supportedSettlementChains),
          getWalletClient(settlementRpc, supportedSettlementChains, privateKey)
        ])

      const appchainPublicClient = await getAppchainClient({
        settlementPublicClient,
        rpcUrl: appchainRpc
      })

      const appchainWalletClient = createWalletClient({
        chain: appchainPublicClient.chain,
        account: privateKeyToAccount(privateKey),
        transport: http(appchainRpc)
      })

      await e2e({
        ...validatedOptions,
        appchainPublicClient,
        appchainWalletClient,
        settlementPublicClient,
        settlementWalletClient
      })
    })
}
