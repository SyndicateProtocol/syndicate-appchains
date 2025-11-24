import { appchainCreateFeaturesOptionsSchema } from "@/cli/schema"
import { addInitSubcommand } from "@/utils/addInitCommand"
import {
  getAppchainClients,
  getSupportedChainClients,
  getSupportedChainPublicClient
} from "@/utils/clients"
import { parseConfigAndOptions } from "@/utils/config"
import type { Command } from "@commander-js/extra-typings"
import { features } from "./features"

export function createFeaturesCommand(program: Command) {
  const featuresCmd = program
    .command("features")
    .description(
      "Deploys Arbitrum nitro token bridge, Syndicate withdrawals contracts, Multicall3"
    )

  addInitSubcommand(
    featuresCmd,
    "features",
    appchainCreateFeaturesOptionsSchema
  )

  featuresCmd
    .option("--config <path>", "Path to JSON config file")
    .option("--settlement-rpc <url>", "RPC URL for the settlement chain")
    .option("--sequencing-rpc <url>", "RPC URL for the sequencing chain")
    .option(
      "--synd-fork-sequencing-rpc <url>",
      "RPC URL for the synd fork sequencing chain"
    )
    .option("--ethereum-rpc <url>", "RPC URL for Ethereum")
    .option("--appchain-rpc <url>", "RPC URL for the appchain")
    .option("--appchain-explorer <url>", "Explorer URL for the appchain")
    .option("--owner-private-key <key>", "Private key of the owner account")
    .option(
      "--deployer-private-key <key>",
      "Private key of the deployer account"
    )
    .option("--chain-name <name>", "Name of the appchain")
    .option(
      "--sequencing-contract <address>",
      "Address of the sequencing contract"
    )
    .option(
      "--core-contracts <contracts>",
      "Core contracts for the appchain (JSON object, JSON string, or path to JSON file)"
    )
    .action(async (options: Record<string, unknown>) => {
      const validatedOptions = parseConfigAndOptions(
        options,
        appchainCreateFeaturesOptionsSchema
      )

      const {
        sequencingRpc,
        settlementRpc,
        deployerPrivateKey,
        ownerPrivateKey,
        coreContracts,
        chainName,
        sequencingContract,
        syndForkSequencingRpc,
        ethereumRpc,
        appchainRpc
      } = validatedOptions

      const [sequencingPublicClient, [deployerSequencingWalletClient]] =
        await getSupportedChainClients(sequencingRpc, [deployerPrivateKey])

      const [
        settlementPublicClient,
        [deployerSettlementWalletClient, ownerSettlementWalletClient]
      ] = await getSupportedChainClients(settlementRpc, [
        deployerPrivateKey,
        ownerPrivateKey
      ])

      const ethereumPublicClient =
        await getSupportedChainPublicClient(ethereumRpc)
      const [appchainPublicClient, [deployerAppchainWalletClient]] =
        await getAppchainClients(appchainRpc, [deployerPrivateKey])

      await features({
        sequencingPublicClient,
        appchainPublicClient,
        settlementPublicClient,
        deployerSequencingWalletClient,
        deployerSettlementWalletClient,
        deployerAppchainWalletClient,
        ownerSettlementWalletClient,
        ethereumPublicClient,
        coreContracts,
        chainId: appchainPublicClient.chain.id,
        chainName,
        sequencingContract,
        syndForkSequencingRpc
      })
    })
}
