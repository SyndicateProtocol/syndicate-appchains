import { appchainCreateFeaturesOptionsSchema } from "@/cli/schema"
import { parseConfigAndOptions } from "@/utils/config"
import { getChainIdFromRpc } from "@/utils/clients"
import { createClients } from "@/utils/createClients"
import type { Command } from "@commander-js/extra-typings"
import { features } from "./features"

export function createFeaturesCommand(program: Command) {
  program
    .command("features")
    .description(
      "Deploys Arbitrum nitro token bridge, Syndicate withdrawals contracts, Multicall3"
    )
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
        coreContracts,
        appchainExplorer,
        chainName,
        sequencingContract,
        syndForkSequencingRpc
      } = validatedOptions

      const {
        appchainPublicClient,
        deployerSequencingWalletClient,
        ownerSettlementWalletClient,
        deployerSettlementWalletClient,
        settlementPublicClient,
        deployerAppchainWalletClient,
        sequencingPublicClient,
        ethereumPublicClient
      } = await createClients({
        ...validatedOptions,
        chainName,
        nativeToken: coreContracts.nativeToken,
        appchainExplorer
      })

      const chainId = await getChainIdFromRpc(validatedOptions.appchainRpc)

      await features({
        appchainPublicClient,
        deployerSequencingWalletClient,
        ownerSettlementWalletClient,
        deployerSettlementWalletClient,
        settlementPublicClient,
        deployerAppchainWalletClient,
        sequencingPublicClient,
        ethereumPublicClient,
        coreContracts,
        chainId,
        chainName,
        sequencingContract,
        syndForkSequencingRpc
      })
    })
}
