import {
  appchainCreateTeeModuleOptionsSchema,
  handleSchemaErrors
} from "@/cli/schema"
import { createClients } from "@/utils/createClients"
import type { Command } from "@commander-js/extra-typings"
import { deployWithdrawals } from "./features/deployWithdrawals"

export function createWithdrawalsContractsCommand(program: Command) {
  program
    .command("withdrawals")
    .description("Deploy withdrawals contracts: AssertionPoster & TeeModule")
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
      "--owner-private-key <key>",
      "Private key of the owner account"
    )
    .requiredOption(
      "--deployer-private-key <key>",
      "Private key of the deployer account"
    )
    .requiredOption(
      "--sequencing-contract <address>",
      "Address of the sequencing contract"
    )
    .requiredOption("--rollup <address>", "Address of the rollup contract")
    .requiredOption(
      "--upgrade-executor <address>",
      "Address of the upgrade executor contract"
    )
    .requiredOption("--bridge <address>", "Address of the bridge contract")
    .action(async (options: Record<string, unknown>) => {
      const {
        data: validatedOptions,
        success,
        error
      } = appchainCreateTeeModuleOptionsSchema.safeParse(options)

      if (!success) {
        return handleSchemaErrors(error)
      }

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
