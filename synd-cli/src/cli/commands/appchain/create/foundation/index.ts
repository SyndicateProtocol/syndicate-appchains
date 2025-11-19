import {
  appchainCreateFoundationOptionsSchema,
  handleSchemaErrors
} from "@/cli/schema"
import { createClients } from "@/utils/createClients"
import type { Command } from "@commander-js/extra-typings"
import type { Hex } from "viem"
import { foundation } from "./foundation"

export function createFoundationCommand(program: Command) {
  program
    .command("foundation")
    .description(
      "Create foundation for a new appchain. Nitro core, Syndicate sequencing, Arb Chain config "
    )
    .requiredOption("--settlement-rpc <url>", "Parent chain RPC URL")
    .requiredOption("--sequencing-rpc <url>", "Sequencing chain RPC URL")
    .requiredOption("--ethereum-rpc <url>", "Ethereum chain RPC URL")
    .requiredOption("--appchain-rpc <url>", "Appchain RPC URL")
    .requiredOption("--appchain-explorer <url>", "Appchain explorer URL")
    .requiredOption("--id <number>", "Chain ID")
    .requiredOption("--name <string>", "Chain name")
    .requiredOption("--deployer-private-key <key>", "Deployer private key")
    .requiredOption("--owner-private-key <key>", "Owner private key")
    .option(
      "--native-token-address <address>",
      "Native token address (optional): defaults to ETH"
    )
    .option(
      "--core-contracts-created-at-hash <hash>",
      "Core contracts created at hash (optional): if provided, will skip deploying the nitro core contracts"
    )
    .action(async (options: Record<string, unknown>) => {
      const {
        data: validatedOptions,
        success,
        error
      } = appchainCreateFoundationOptionsSchema.safeParse(options)

      if (!success) {
        return handleSchemaErrors(error)
      }

      const clients = await createClients(validatedOptions)

      await foundation({
        deployerSettlementWalletClient: clients.deployerSettlementWalletClient,
        deployerSequencingWalletClient: clients.deployerSequencingWalletClient,
        ownerSettlementWalletClient: clients.ownerSettlementWalletClient,
        ownerSequencingWalletClient: clients.ownerSequencingWalletClient,
        settlementPublicClient: clients.settlementPublicClient,
        sequencingPublicClient: clients.sequencingPublicClient,

        chainId: validatedOptions.id,
        chainName: validatedOptions.name.toLowerCase().replace(/\s+/g, "-"),
        nativeToken: validatedOptions.nativeTokenAddress,
        ethereumChainRpcUrl:
          clients.ethereumPublicClient.chain.rpcUrls.default.http[0],
        ownerPrivateKey: validatedOptions.ownerPrivateKey as Hex,
        coreContractsCreatedAtHash:
          validatedOptions.coreContractsCreatedAtHash as Hex,
        appChainRpc: validatedOptions.appchainRpc,
        appChainExplorer: validatedOptions.appchainExplorer
      })

      console.log("validatedOptions", validatedOptions)
    })
}
