import {
  appchainCreateFeaturesOptionsSchema,
  handleSchemaErrors
} from "@/cli/schema"
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
      "--appchain-explorer <url>",
      "Explorer URL for the appchain"
    )
    .requiredOption(
      "--owner-private-key <key>",
      "Private key of the owner account"
    )
    .requiredOption(
      "--deployer-private-key <key>",
      "Private key of the deployer account"
    )
    .requiredOption("--chain-name <name>", "Name of the appchain")
    .requiredOption(
      "--sequencing-contract <address>",
      "Address of the sequencing contract"
    )
    .requiredOption(
      "--core-contracts <contracts>",
      "Core contracts for the appchain"
    )
    .action(async (options: Record<string, unknown>) => {
      const {
        data: validatedOptions,
        success,
        error
      } = appchainCreateFeaturesOptionsSchema.safeParse(options)

      if (!success) {
        return handleSchemaErrors(error)
      }

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
