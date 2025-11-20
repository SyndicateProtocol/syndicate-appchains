import { createClients } from "@/utils/createClients"
import type { Command } from "@commander-js/extra-typings"
import { formatEther, parseEther } from "viem"
import {
  appchainHandoffOptionsSchema,
  handleSchemaErrors
} from "../../../schema"
import { handoff } from "./handoff"

export function handoffCommand(program: Command) {
  program
    .command("handoff")
    .description("Transfer appchain ownership to a new owner")
    .requiredOption(
      "--settlement-rpc <url>",
      "RPC URL for the settlement chain"
    )
    .requiredOption(
      "--sequencing-rpc <url>",
      "RPC URL for the sequencing chain"
    )
    .requiredOption("--appchain-rpc <url>", "RPC URL for the appchain")
    .requiredOption(
      "--owner-private-key <key>",
      "Private key of the current owner"
    )
    .requiredOption("--new-owner <address>", "Address of the new owner")
    .requiredOption(
      "--synd <json>",
      "JSON string containing synd contract addresses (config, bridge, sequencing, withdrawals)"
    )
    .action(async (options: Record<string, unknown>) => {
      // Parse synd JSON string
      let parsedOptions = options
      if (typeof options.synd === "string") {
        try {
          parsedOptions = {
            ...options,
            synd: JSON.parse(options.synd)
          }
        } catch (error) {
          console.error("🚫 Invalid JSON for --synd option")
          process.exit(1)
        }
      }

      const {
        data: validatedOptions,
        success,
        error
      } = appchainHandoffOptionsSchema.safeParse(parsedOptions)

      if (!success) {
        return handleSchemaErrors(error)
      }

      const { newOwner, synd } = validatedOptions

      const {
        ownerAppchainWalletClient,
        ownerSequencingWalletClient,
        ownerSettlementWalletClient,
        settlementPublicClient,
        sequencingPublicClient,
        appchainPublicClient
      } = await createClients(validatedOptions)

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
