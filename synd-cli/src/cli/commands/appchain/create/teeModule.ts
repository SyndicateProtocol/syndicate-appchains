import { appchainDeployTeeModuleOptionsSchema } from "@/cli/schema"
import { addInitSubcommand } from "@/utils/addInitCommand"
import {
  getAppchainClients,
  getSupportedChainClients,
  getSupportedChainPublicClient
} from "@/utils/clients"
import { parseConfigAndOptions } from "@/utils/config"
import { print } from "@/utils/print"
import type { Command } from "@commander-js/extra-typings"
import { deployTeeModule } from "./features/deployTeeModule"

export function createTeeModuleCommand(program: Command) {
  const teeCmd = program.command("tee-module").description("Deploys TeeModule")

  addInitSubcommand(teeCmd, "tee-module", appchainDeployTeeModuleOptionsSchema)

  teeCmd
    .option("--config <path>", "Path to JSON config file")
    .option("--settlement-rpc <url>", "RPC URL for the settlement chain")
    .option("--sequencing-rpc <url>", "RPC URL for the sequencing chain")
    .option(
      "--synd-fork-sequencing-rpc <url>",
      "RPC URL for the synd fork sequencing chain"
    )
    .option("--ethereum-rpc <url>", "RPC URL for Ethereum")
    .option("--appchain-rpc <url>", "RPC URL for the appchain")
    .option(
      "--deployer-private-key <key>",
      "Private key of the deployer account"
    )
    .option(
      "--sequencing-contract <address>",
      "Address of the sequencing contract"
    )
    .option(
      "--assertion-poster <address>",
      "Address of the AssertionPoster contract"
    )
    .option("--bridge <address>", "Address of the bridge contract")
    .action(async (options: Record<string, unknown>) => {
      const validatedOptions = parseConfigAndOptions(
        options,
        appchainDeployTeeModuleOptionsSchema
      )

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

      const [settlementPublicClient, [deployerSettlementWalletClient]] =
        await getSupportedChainClients(settlementRpc, [deployerPrivateKey])
      const sequencingPublicClient =
        await getSupportedChainPublicClient(sequencingRpc)
      const ethereumPublicClient =
        await getSupportedChainPublicClient(ethereumRpc)
      const [appchainPublicClient] = await getAppchainClients(appchainRpc)

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

      print("TeeModule deployed at", teeModuleAddress)
      print(
        "Next steps",
        `
        1. Transfer AssertionPoster ownership to TeeModule
        2. Set TeeModule DEFAULT_ADMIN_ROLE to the desired owner
        3. Revoke DEFAULT_ADMIN_ROLE from deployer
        `
      )
    })
}
