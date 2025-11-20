import {
  appchainDeployAssertionPosterOptionsSchema,
  handleSchemaErrors
} from "@/cli/schema"
import { createClients } from "@/utils/createClients"
import type { Command } from "@commander-js/extra-typings"
import { deployAssertionPoster } from "./features/deployAssertionPoster"

export function createAssertionPosterCommand(program: Command) {
  program
    .command("assertion-poster")
    .description("Deploys AssertionPoster")
    .requiredOption(
      "--settlement-rpc <url>",
      "RPC URL for the settlement chain"
    )
    .requiredOption(
      "--owner-private-key <key>",
      "Private key of the owner account"
    )
    .requiredOption(
      "--deployer-private-key <key>",
      "Private key of the deployer account"
    )
    .requiredOption("--rollup <address>", "Address of the rollup contract")
    .requiredOption(
      "--upgrade-executor <address>",
      "Address of the upgrade executor contract"
    )
    .action(async (options: Record<string, unknown>) => {
      const {
        data: validatedOptions,
        success,
        error
      } = appchainDeployAssertionPosterOptionsSchema.safeParse(options)

      if (!success) {
        return handleSchemaErrors(error)
      }

      const { rollup, upgradeExecutor } = validatedOptions
      const {
        settlementPublicClient,
        deployerSettlementWalletClient,
        ownerSettlementWalletClient
      } = await createClients(validatedOptions)
      const assertionPosterAddress = await deployAssertionPoster({
        rollup,
        upgradeExecutor,
        settlementPublicClient,
        deployerSettlementWalletClient,
        ownerSettlementWalletClient
      })

      console.log(`\nAssertionPoster deployed at: ${assertionPosterAddress}`)
      console.log(
        "\nNext steps:",
        "\n1. Deploy TeeModule contract",
        "\n2. Transfer AssertionPoster ownership to TeeModule"
      )
    })
}
