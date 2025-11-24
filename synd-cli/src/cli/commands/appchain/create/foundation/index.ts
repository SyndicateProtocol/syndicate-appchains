import { appchainCreateFoundationOptionsSchema } from "@/cli/schema"
import { addInitSubcommand } from "@/utils/addInitCommand"
import { getSupportedChainClients } from "@/utils/clients"
import { parseConfigAndOptions } from "@/utils/config"
import type { Command } from "@commander-js/extra-typings"
import { foundation } from "./foundation"

export function createFoundationCommand(program: Command) {
  const foundationCmd = program
    .command("foundation")
    .description(
      "Deploys Arbitrum nitro core contracts, Syndicate sequencing contracts & ArbChainConfig"
    )

  // Add init subcommand - automatically extracts from schema!
  addInitSubcommand(
    foundationCmd,
    "foundation",
    appchainCreateFoundationOptionsSchema
  )

  // Original foundation command
  foundationCmd
    .option("--config <path>", "Path to JSON config file")
    .option("--settlement-rpc <url>", "Parent chain RPC URL")
    .option("--sequencing-rpc <url>", "Sequencing chain RPC URL")
    .option("--ethereum-rpc <url>", "Ethereum chain RPC URL")
    .option("--appchain-rpc <url>", "Appchain RPC URL")
    .option("--appchain-explorer <url>", "Appchain explorer URL")
    .option("--id <number>", "Chain ID")
    .option("--name <string>", "Chain name")
    .option("--deployer-private-key <key>", "Deployer private key")
    .option("--owner-private-key <key>", "Owner private key")
    .option(
      "--native-token <address>",
      "Native token address (optional): defaults to ETH"
    )
    .option(
      "--core-contracts-created-at-hash <hash>",
      "Core contracts created at hash (optional): if provided, will skip deploying the nitro core contracts"
    )
    .action(async (options: Record<string, unknown>) => {
      const validatedOptions = parseConfigAndOptions(
        options,
        appchainCreateFoundationOptionsSchema
      )

      const {
        id,
        name,
        nativeToken,
        ownerPrivateKey,
        deployerPrivateKey,
        coreContractsCreatedAtHash,
        appchainRpc,
        appchainExplorer,
        ethereumRpc,
        settlementRpc,
        sequencingRpc
      } = validatedOptions

      const [
        settlementPublicClient,
        [deployerSettlementWalletClient, ownerSettlementWalletClient]
      ] = await getSupportedChainClients(settlementRpc, [
        deployerPrivateKey,
        ownerPrivateKey
      ])
      const [
        sequencingPublicClient,
        [deployerSequencingWalletClient, ownerSequencingWalletClient]
      ] = await getSupportedChainClients(sequencingRpc, [
        deployerPrivateKey,
        ownerPrivateKey
      ])

      await foundation({
        deployerSettlementWalletClient,
        deployerSequencingWalletClient,
        ownerSettlementWalletClient,
        ownerSequencingWalletClient,
        settlementPublicClient,
        sequencingPublicClient,
        chainId: id,
        chainName: name,
        nativeToken,
        ethereumChainRpcUrl: ethereumRpc,
        ownerPrivateKey,
        coreContractsCreatedAtHash,
        appchainRpc,
        appchainExplorer
      })
    })
}
