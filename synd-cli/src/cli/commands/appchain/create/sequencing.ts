import { deploySequencingChain } from "@/cli/commands/appchain/create/foundation/deploySequencingChain"
import { appchainCreateSequencingChainOptionsSchema } from "@/cli/schema"
import { addInitSubcommand } from "@/utils/addInitCommand"
import { getSupportedChainClients } from "@/utils/clients"
import { parseConfigAndOptions } from "@/utils/config"
import { print } from "@/utils/print"
import type { Command } from "@commander-js/extra-typings"
import { generatePrivateKey, privateKeyToAccount } from "viem/accounts"

export function createSequencingCommand(program: Command) {
  const sequencingCmd = program
    .command("sequencing")
    .description(
      "Deploy sequencing contracts: SyndicateSequencingChain, AllowlistSequencingModule, RequireAndModule"
    )

  addInitSubcommand(
    sequencingCmd,
    "sequencing",
    appchainCreateSequencingChainOptionsSchema
  )

  sequencingCmd
    .option("--config <path>", "Path to JSON config file")
    .option("--sequencing-rpc <url>", "RPC URL for the sequencing chain")
    .option("--ethereum-rpc <url>", "RPC URL for Ethereum")
    .option("--owner-private-key <key>", "Private key of the owner account")
    .option(
      "--deployer-private-key <key>",
      "Private key of the deployer account"
    )
    .option("--id <number>", "Chain ID for the appchain")
    .action(async (options: Record<string, unknown>) => {
      const validatedOptions = parseConfigAndOptions(
        options,
        appchainCreateSequencingChainOptionsSchema
      )

      const {
        id: chainId,
        sequencingRpc,
        ownerPrivateKey,
        deployerPrivateKey,
        ethereumRpc
      } = validatedOptions
      const [
        sequencingPublicClient,
        [deployerSequencingWalletClient, ownerSequencingWalletClient]
      ] = await getSupportedChainClients(sequencingRpc, [
        deployerPrivateKey,
        ownerPrivateKey
      ])

      const [ethereumPublicClient, [deployerEthereumWalletClient]] =
        await getSupportedChainClients(ethereumRpc, [deployerPrivateKey])

      const sequencerPrivateKey = generatePrivateKey()
      const sequencerAccount = privateKeyToAccount(sequencerPrivateKey)

      print("Sequencer Address", sequencerAccount.address)
      print("Sequencer Private Key", sequencerPrivateKey)
      print("Deploying Syndicate sequencing chain...")

      const {
        sequencingContract,
        allowlistSequencingModule,
        requireAndModule
      } = await deploySequencingChain({
        sequencerAccount,
        chainId,
        sequencingPublicClient,
        deployerSequencingWalletClient,
        ownerSequencingWalletClient,
        deployerEthereumWalletClient,
        ethereumPublicClient
      })

      print("Deployed contracts:")
      print("Sequencing Contract", sequencingContract)
      print("Allowlist Sequencing Module", allowlistSequencingModule)
      print("Require And Module", requireAndModule)
    })
}
