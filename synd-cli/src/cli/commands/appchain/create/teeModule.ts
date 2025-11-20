import {
  appchainDeployTeeModuleOptionsSchema,
  handleSchemaErrors
} from "@/cli/schema"
import { createClients } from "@/utils/createClients"
import type { Command } from "@commander-js/extra-typings"
import { deployTeeModule } from "./features/deployTeeModule"

export function createTeeModuleCommand(program: Command) {
  program
    .command("tee-module")
    .description("Deploys TeeModule")
    .requiredOption(
      "--settlement-rpc <url>",
      "RPC URL for the settlement chain"
    )
    .requiredOption(
      "--sequencing-rpc <url>",
      "RPC URL for the sequencing chain"
    )
    .requiredOption(
      "--synd-fork-sequencing-rpc <url>",
      "RPC URL for the synd fork sequencing chain"
    )
    .requiredOption("--ethereum-rpc <url>", "RPC URL for Ethereum")
    .requiredOption("--appchain-rpc <url>", "RPC URL for the appchain")
    .requiredOption(
      "--deployer-private-key <key>",
      "Private key of the deployer account"
    )
    .requiredOption(
      "--sequencing-contract <address>",
      "Address of the sequencing contract"
    )
    .requiredOption(
      "--assertion-poster <address>",
      "Address of the AssertionPoster contract"
    )
    .requiredOption("--bridge <address>", "Address of the bridge contract")
    .action(async (options: Record<string, unknown>) => {
      const {
        data: validatedOptions,
        success,
        error
      } = appchainDeployTeeModuleOptionsSchema.safeParse(options)

      if (!success) {
        return handleSchemaErrors(error)
      }

      const {
        bridge,
        assertionPoster,
        sequencingContract,
        syndForkSequencingRpc,
        settlementRpc,
        sequencingRpc,
        ethereumRpc,
        deployerPrivateKey,
        appchainRpc
      } = validatedOptions

      const {
        settlementPublicClient,
        deployerSettlementWalletClient,
        sequencingPublicClient,
        ethereumPublicClient,
        appchainPublicClient
      } = await createClients({
        settlementRpc,
        sequencingRpc,
        ethereumRpc,
        deployerPrivateKey,
        appchainRpc
      })

      const teeModuleAddress = await deployTeeModule({
        assertionPoster,
        bridge,
        deployerSettlementWalletClient,
        settlementPublicClient,
        sequencingContract,
        sequencingPublicClient,
        appchainPublicClient,
        ethereumPublicClient,
        syndForkSequencingRpc
      })

      console.log(`\nTeeModule deployed at: ${teeModuleAddress}`)
      console.log(
        "\nNext steps:",
        "\n1. Transfer AssertionPoster ownership to TeeModule",
        "\n2. Set TeeModule DEFAULT_ADMIN_ROLE to the desired owner",
        "\n3. Revoke DEFAULT_ADMIN_ROLE from deployer"
      )
    })
}
