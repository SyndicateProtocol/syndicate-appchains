import { deploySequencingChain } from "@/cli/commands/appchain/create/foundation/deploySequencingChain"
import {
  appchainCreateSequencingChainOptionsSchema,
  handleSchemaErrors
} from "@/cli/schema"
import { createClients } from "@/utils/createClients"
import type { Command } from "@commander-js/extra-typings"
import { generatePrivateKey, privateKeyToAccount } from "viem/accounts"

export function createSequencingCommand(program: Command) {
  program
    .command("sequencing")
    .description(
      "Deploy sequencing contracts: SyndicateSequencingChain, AllowlistSequencingModule, RequireAndModule"
    )
    .requiredOption(
      "--sequencing-rpc <url>",
      "RPC URL for the sequencing chain"
    )
    .requiredOption(
      "--owner-private-key <key>",
      "Private key of the owner account"
    )
    .requiredOption(
      "--deployer-private-key <key>",
      "Private key of the deployer account"
    )
    .requiredOption("--id <number>", "Chain ID for the appchain")
    .action(async (options: Record<string, unknown>) => {
      const {
        data: validatedOptions,
        success,
        error
      } = appchainCreateSequencingChainOptionsSchema.safeParse(options)

      if (!success) {
        return handleSchemaErrors(error)
      }

      const { id: chainId } = validatedOptions
      const {
        sequencingPublicClient,
        deployerSequencingWalletClient,
        ownerSequencingWalletClient
      } = await createClients(validatedOptions)

      const sequencerPrivateKey = generatePrivateKey()
      const sequencerAccount = privateKeyToAccount(sequencerPrivateKey)

      console.log("Sequencer Address:", sequencerAccount.address)
      console.log("Sequencer Private Key:", sequencerPrivateKey)
      console.log("\n=Deploying Syndicate sequencing chain...\n")

      const {
        sequencingContract,
        allowlistSequencingModule,
        requireAndModule,
        deployedAtBlock
      } = await deploySequencingChain({
        sequencerAccount,
        chainId,
        sequencingPublicClient,
        deployerSequencingWalletClient,
        ownerSequencingWalletClient
      })

      console.log("\nDeployed contracts:")
      console.log(`  Sequencing Contract: ${sequencingContract}`)
      console.log(`  Allowlist Sequencing Module: ${allowlistSequencingModule}`)
      console.log(`  Require And Module: ${requireAndModule}`)
      console.log(`  Deployed at Block: ${deployedAtBlock}`)
    })
}
