import { appchainE2EOptionsSchema, handleSchemaErrors } from "@/cli/schema"
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
  program
    .command("e2e")
    .description("Run end-to-end tests on an appchain")
    .requiredOption("--settlement-rpc <url>", "Settlement chain RPC URL")
    .requiredOption("--appchain-rpc <url>", "Appchain RPC URL")
    .requiredOption("--inbox <address>", "Inbox contract address")
    .requiredOption("--private-key <key>", "Private key for transactions")
    .action(async (options: Record<string, unknown>) => {
      const {
        data: validatedOptions,
        success,
        error
      } = appchainE2EOptionsSchema.safeParse(options)

      if (!success) {
        return handleSchemaErrors(error)
      }

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
