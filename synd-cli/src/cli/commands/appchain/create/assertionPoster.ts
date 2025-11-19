import {
  appchainDeployAssertionPosterOptionsSchema,
  handleSchemaErrors
} from "@/cli/schema"
import { getPublicClient, getWalletClient } from "@/utils/clients"
import { supportedSettlementChains } from "@/utils/constants"
import type { Command } from "@commander-js/extra-typings"
import { deployAssertionPoster } from "./features/deployAssertionPoster"

export function createAssertionPosterCommand(program: Command) {
  program
    .command("assertion-poster")
    .description("Deploy a new AssertionPoster contract")
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

      const {
        settlementRpc,
        deployerPrivateKey,
        ownerPrivateKey,
        rollup,
        upgradeExecutor
      } = validatedOptions

      const [
        settlementPublicClient,
        ownerSettlementWalletClient,
        deployerSettlementWalletClient
      ] = await Promise.all([
        getPublicClient(settlementRpc, supportedSettlementChains),
        getWalletClient(
          settlementRpc,
          supportedSettlementChains,
          ownerPrivateKey as `0x${string}`
        ),
        getWalletClient(
          settlementRpc,
          supportedSettlementChains,
          deployerPrivateKey as `0x${string}`
        )
      ])

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
