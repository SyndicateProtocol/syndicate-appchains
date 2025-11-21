import { createClients } from "@/utils/createClients"
import type { Command } from "@commander-js/extra-typings"
import { formatEther, parseEther } from "viem"
import {
  appchainHandoffOptionsSchema,
  handleSchemaErrors
} from "../../../schema"
import { kebabToCamelCase, loadConfigFile, mergeConfigWithOptions } from "@/utils/config"
import { existsSync, readFileSync } from "node:fs"
import { resolve } from "node:path"
import { handoff } from "./handoff"

export function handoffCommand(program: Command) {
  program
    .command("handoff")
    .description("Transfer appchain ownership to a new owner")
    .option("--config <path>", "Path to JSON config file")
    .option(
      "--settlement-rpc <url>",
      "RPC URL for the settlement chain"
    )
    .option(
      "--sequencing-rpc <url>",
      "RPC URL for the sequencing chain"
    )
    .option("--appchain-rpc <url>", "RPC URL for the appchain")
    .option(
      "--owner-private-key <key>",
      "Private key of the current owner"
    )
    .option("--new-owner <address>", "Address of the new owner")
    .option(
      "--synd <json>",
      "JSON string or path to JSON file containing synd contract addresses (config, bridge, sequencing, withdrawals)"
    )
    .action(async (options: Record<string, unknown>) => {
      // Load config file if provided
      let configFileOptions: Record<string, unknown> = {}
      if (options.config) {
        const rawConfig = loadConfigFile<Record<string, unknown>>(options.config as string)
        configFileOptions = kebabToCamelCase(rawConfig)
      }

      // Remove config key before merging (it's not part of the schema)
      const { config, ...optionsWithoutConfig } = options

      // Merge config file with CLI options (CLI takes precedence)
      let mergedOptions = mergeConfigWithOptions(configFileOptions, optionsWithoutConfig)

      // Parse synd JSON string or file path
      if (typeof mergedOptions.synd === "string") {
        const syndValue = mergedOptions.synd as string
        const resolvedPath = resolve(syndValue)

        try {
          if (existsSync(resolvedPath)) {
            // It's a file path
            const fileContent = readFileSync(resolvedPath, "utf-8")
            mergedOptions = {
              ...mergedOptions,
              synd: JSON.parse(fileContent)
            }
          } else {
            // It's a JSON string
            mergedOptions = {
              ...mergedOptions,
              synd: JSON.parse(syndValue)
            }
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
      } = appchainHandoffOptionsSchema.safeParse(mergedOptions)

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
