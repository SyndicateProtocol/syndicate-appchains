import { ArbOwnerABI } from "@/abi/nitro/ArbOwner"
import { callArbOwnerOptionsSchema } from "@/cli/schema"
import { parseConfigAndOptions } from "@/utils/config"
import { createClients } from "@/utils/createClients"
import { exitWithError } from "@/utils/print"
import type { Command } from "@commander-js/extra-typings"
import type { AbiFunction, ExtractAbiFunctionNames } from "abitype"
import { encodeFunctionData } from "viem"
import {
  formatFunctionSignatureForDisplay,
  getWriteFunctions,
  preprocessArgs
} from "../helpers"
import { generateCallArbOwnerTx } from "./generateCallArbOwnerTx"

export function callArbOwnerCommand(program: Command) {
  program
    .command("call")
    .description("Call a specific ArbOwner function")
    .argument("<functionName>", "Name of the ArbOwner function to call")
    .argument("[args...]", "Arguments for the function")
    .option("--config <path>", "Path to JSON config file")
    .option("--settlement-rpc <url>", "Parent chain RPC URL")
    .option("--appchain-rpc <url>", "Appchain RPC URL")
    .option(
      "--settlement-upgrade-executor <address>",
      "Parent chain UpgradeExecutor address"
    )
    .option("--settlement-inbox <address>", "Parent chain Inbox address")
    .option(
      "--appchain-upgrade-executor <address>",
      "Appchain UpgradeExecutor address"
    )
    .option(
      "--refund-address <address>",
      "Address on appchain to receive excess fees"
    )
    .option("--gas-limit <limit>", "(optional) Gas limit for retryable ticket")
    .option("--max-fee-per-gas <wei>", "(optional) Max fee per gas in wei")
    .action(
      async (
        functionName: string,
        args: string[],
        options: Record<string, unknown>
      ) => {
        const writeFunctions = getWriteFunctions()
        const functionAbi = writeFunctions.find(
          (item) => item.name === functionName
        ) as AbiFunction | undefined

        if (!functionAbi) {
          return exitWithError(
            `Function '${functionName}' not found in ArbOwner ABI.\n\nAvailable write functions:\n${writeFunctions
              .map((fn) => `  ${formatFunctionSignatureForDisplay(fn)}`)
              .join(
                "\n"
              )}\n\nTip: Run 'synd-cli callArbOwner list' to see all available functions.`
          )
        }

        const validatedOptions = parseConfigAndOptions(
          options,
          callArbOwnerOptionsSchema
        )

        if (args.length !== functionAbi.inputs.length) {
          return exitWithError(
            `Function '${functionName}' expects ${functionAbi.inputs.length} argument(s) but got ${args.length}.\n${formatFunctionSignatureForDisplay(functionAbi)}`
          )
        }

        let preprocessedArgs: unknown[] = []
        try {
          preprocessedArgs = preprocessArgs(functionAbi, args)
        } catch (error) {
          return exitWithError(
            `Invalid arguments: ${error instanceof Error ? error.message : "Unknown error"}\n${formatFunctionSignatureForDisplay(functionAbi)}`
          )
        }

        const calldata = encodeFunctionData({
          abi: ArbOwnerABI,
          functionName: functionName as ExtractAbiFunctionNames<
            typeof ArbOwnerABI
          >,
          // biome-ignore lint/suspicious/noExplicitAny: args could be of any type here, we rely on viem to validate
          args: preprocessedArgs as any
        })

        const {
          settlementRpc,
          appchainRpc,
          settlementUpgradeExecutor,
          settlementInbox,
          appchainUpgradeExecutor,
          refundAddress,
          gasLimit,
          maxFeePerGas
        } = validatedOptions

        const { settlementPublicClient, appchainPublicClient } =
          await createClients({
            settlementRpc,
            appchainRpc
          })

        await generateCallArbOwnerTx({
          settlementPublicClient,
          appchainPublicClient,
          settlementUpgradeExecutor,
          settlementInbox,
          appchainUpgradeExecutor,
          refundAddress,
          gasLimit,
          maxFeePerGas,
          functionName,
          calldata
        })
      }
    )
}
