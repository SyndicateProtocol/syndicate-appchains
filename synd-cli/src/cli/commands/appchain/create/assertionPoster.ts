import { appchainDeployAssertionPosterOptionsSchema } from "@/cli/schema"
import { addInitSubcommand } from "@/utils/addInitCommand"
import { getSupportedChainClients } from "@/utils/clients"
import { parseConfigAndOptions } from "@/utils/config"
import type { Command } from "@commander-js/extra-typings"
import { deployAssertionPoster } from "./features/deployAssertionPoster"

export function createAssertionPosterCommand(program: Command) {
  const assertionCmd = program
    .command("assertion-poster")
    .description("Deploys AssertionPoster")

  addInitSubcommand(
    assertionCmd,
    "assertion-poster",
    appchainDeployAssertionPosterOptionsSchema
  )

  assertionCmd
    .option("--config <path>", "Path to JSON config file")
    .option("--settlement-rpc <url>", "RPC URL for the settlement chain")
    .option("--owner-private-key <key>", "Private key of the owner account")
    .option(
      "--deployer-private-key <key>",
      "Private key of the deployer account"
    )
    .option("--rollup <address>", "Address of the rollup contract")
    .option(
      "--upgrade-executor <address>",
      "Address of the upgrade executor contract"
    )
    .action(async (options: Record<string, unknown>) => {
      const validatedOptions = parseConfigAndOptions(
        options,
        appchainDeployAssertionPosterOptionsSchema
      )

      const {
        rollup,
        upgradeExecutor,
        settlementRpc,
        deployerPrivateKey,
        ownerPrivateKey
      } = validatedOptions
      const [
        settlementPublicClient,
        [deployerSettlementWalletClient, ownerSettlementWalletClient]
      ] = await getSupportedChainClients(settlementRpc, [
        deployerPrivateKey,
        ownerPrivateKey
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
