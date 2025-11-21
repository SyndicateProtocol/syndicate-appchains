import {
  appchainCreateFoundationOptionsSchema,
  handleSchemaErrors
} from "@/cli/schema"
import {
  kebabToCamelCase,
  loadConfigFile,
  mergeConfigWithOptions
} from "@/utils/config"
import { createClients } from "@/utils/createClients"
import type { Command } from "@commander-js/extra-typings"
import { foundation } from "./foundation"

export function createFoundationCommand(program: Command) {
  program
    .command("foundation")
    .description(
      "Deploys Arbitrum nitro core contracts, Syndicate sequencing contracts & ArbChainConfig"
    )
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
      // Load config file if provided
      let configFileOptions: Record<string, unknown> = {}
      if (options.config) {
        const rawConfig = loadConfigFile<Record<string, unknown>>(
          options.config as string
        )
        configFileOptions = kebabToCamelCase(rawConfig)
      }

      // Remove config key before merging (it's not part of the schema)
      const { config, ...optionsWithoutConfig } = options

      // Merge config file with CLI options (CLI takes precedence)
      const mergedOptions = mergeConfigWithOptions(
        configFileOptions,
        optionsWithoutConfig
      )
      console.log("mergedOptions", mergedOptions)
      const {
        data: validatedOptions,
        success,
        error
      } = appchainCreateFoundationOptionsSchema.safeParse(mergedOptions)

      if (!success) {
        return handleSchemaErrors(error)
      }
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
      const {
        deployerSettlementWalletClient,
        deployerSequencingWalletClient,
        ownerSettlementWalletClient,
        ownerSequencingWalletClient,
        settlementPublicClient,
        sequencingPublicClient
      } = await createClients({
        settlementRpc,
        sequencingRpc,
        ownerPrivateKey,
        deployerPrivateKey
      })

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
