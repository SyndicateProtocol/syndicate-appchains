import { appchainCreateTeeModuleOptionsSchema } from "@/cli/schema"
import { parseConfigAndOptions } from "@/utils/config"
import { addInitSubcommand } from "@/utils/addInitCommand"
import { createClients } from "@/utils/createClients"
import type { Command } from "@commander-js/extra-typings"
import { deployWithdrawals } from "./features/deployWithdrawals"

export function createWithdrawalsContractsCommand(program: Command) {
  const withdrawalsCmd = program
    .command("withdrawals")
    .description("Deploy withdrawals contracts: AssertionPoster & TeeModule")

  addInitSubcommand(
    withdrawalsCmd,
    "withdrawals",
    appchainCreateTeeModuleOptionsSchema
  )

  withdrawalsCmd
    .option("--config <path>", "Path to JSON config file")
    .option("--settlement-rpc <url>", "RPC URL for the settlement chain")
    .option("--sequencing-rpc <url>", "RPC URL for the sequencing chain")
    .option(
      "--synd-fork-sequencing-rpc <url>",
      "RPC URL for the synd fork sequencing chain"
    )
    .option("--ethereum-rpc <url>", "RPC URL for Ethereum")
    .option("--appchain-rpc <url>", "RPC URL for the appchain")
    .option("--owner-private-key <key>", "Private key of the owner account")
    .option(
      "--deployer-private-key <key>",
      "Private key of the deployer account"
    )
    .option(
      "--sequencing-contract <address>",
      "Address of the sequencing contract"
    )
    .option("--rollup <address>", "Address of the rollup contract")
    .option(
      "--upgrade-executor <address>",
      "Address of the upgrade executor contract"
    )
    .option("--bridge <address>", "Address of the bridge contract")
    .action(async (options: Record<string, unknown>) => {
      const validatedOptions = parseConfigAndOptions(
        options,
        appchainCreateTeeModuleOptionsSchema
      )

      const {
        settlementRpc,
        sequencingRpc,
        ethereumRpc,
        deployerPrivateKey,
        ownerPrivateKey,
        rollup,
        upgradeExecutor,
        bridge,
        appchainRpc,
        sequencingContract,
        syndForkSequencingRpc
      } = validatedOptions

      const {
        settlementPublicClient,
        sequencingPublicClient,
        ethereumPublicClient,
        ownerSettlementWalletClient,
        deployerSettlementWalletClient,
        appchainPublicClient
      } = await createClients({
        settlementRpc,
        sequencingRpc,
        ethereumRpc,
        deployerPrivateKey,
        ownerPrivateKey,
        appchainRpc
      })

      await deployWithdrawals({
        syndForkSequencingRpc,
        settlementPublicClient,
        deployerSettlementWalletClient,
        ownerSettlementWalletClient,
        sequencingContract,
        sequencingPublicClient,
        appchainPublicClient,
        ethereumPublicClient,
        coreContracts: {
          rollup,
          upgradeExecutor,
          bridge
        }
      })
    })
}
