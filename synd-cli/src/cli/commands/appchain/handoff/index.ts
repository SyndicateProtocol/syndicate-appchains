import { addInitSubcommand } from "@/utils/addInitCommand"
import { getAppchainClients, getSupportedChainClients } from "@/utils/clients"
import { parseConfigAndOptions } from "@/utils/config"
import type { Command } from "@commander-js/extra-typings"
import { formatEther, parseEther } from "viem"
import { appchainHandoffOptionsSchema } from "../../../schema"
import { handoff } from "./handoff"

export function handoffCommand(program: Command) {
  const handoffCmd = program
    .command("handoff")
    .description("Transfer appchain ownership to a new owner")

  addInitSubcommand(handoffCmd, "handoff", appchainHandoffOptionsSchema)

  handoffCmd
    .option("--config <path>", "Path to JSON config file")
    .option("--settlement-rpc <url>", "RPC URL for the settlement chain")
    .option("--sequencing-rpc <url>", "RPC URL for the sequencing chain")
    .option("--appchain-rpc <url>", "RPC URL for the appchain")
    .option("--owner-private-key <key>", "Private key of the current owner")
    .option("--new-owner <address>", "Address of the new owner")
    .option(
      "--synd <json>",
      "Synd contract addresses (JSON object, JSON string, or path to JSON file)"
    )
    .action(async (options: Record<string, unknown>) => {
      const validatedOptions = parseConfigAndOptions(
        options,
        appchainHandoffOptionsSchema
      )

      const {
        newOwner,
        synd,
        appchainRpc,
        sequencingRpc,
        settlementRpc,
        ownerPrivateKey
      } = validatedOptions

      const [sequencingPublicClient, [ownerSequencingWalletClient]] =
        await getSupportedChainClients(sequencingRpc, [ownerPrivateKey])
      const [settlementPublicClient, [ownerSettlementWalletClient]] =
        await getSupportedChainClients(settlementRpc, [ownerPrivateKey])

      const [appchainPublicClient, [ownerAppchainWalletClient]] =
        await getAppchainClients(appchainRpc, [ownerPrivateKey])

      const balanceThreshold = parseEther("0.001")
      const [appchainBalance, sequencingBalance, settlementBalance] =
        await Promise.all([
          appchainPublicClient.getBalance({
            address: ownerAppchainWalletClient.account.address
          }),
          sequencingPublicClient.getBalance({
            address: ownerSequencingWalletClient.account.address
          }),
          settlementPublicClient.getBalance({
            address: ownerSettlementWalletClient.account.address
          })
        ])

      if (appchainBalance < balanceThreshold) {
        console.error(
          `🚫 Owner balance on the appchain: ${formatEther(appchainBalance)} is less than the threshold: ${formatEther(balanceThreshold)}`
        )
        process.exit(1)
      }
      if (sequencingBalance < balanceThreshold) {
        console.error(
          `🚫 Owner balance on the sequencing chain: ${formatEther(sequencingBalance)} is less than the threshold: ${formatEther(balanceThreshold)}`
        )
        process.exit(1)
      }
      if (settlementBalance < balanceThreshold) {
        console.error(
          `🚫 Owner balance on the settlement chain: ${formatEther(settlementBalance)} is less than the threshold: ${formatEther(balanceThreshold)}`
        )
        process.exit(1)
      }

      await handoff({
        newOwner,
        ownerSettlementWalletClient,
        ownerSequencingWalletClient,
        ownerAppchainWalletClient,
        settlementPublicClient,
        sequencingPublicClient,
        appchainPublicClient,
        synd
      })
    })
}
