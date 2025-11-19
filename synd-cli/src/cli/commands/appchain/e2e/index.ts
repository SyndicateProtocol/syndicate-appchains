import { appchainE2EOptionsSchema, handleSchemaErrors } from "@/cli/schema"
import type { Command } from "@commander-js/extra-typings"
import { e2e } from "./e2e"

export function e2eCommand(program: Command) {
  program
    .command("e2e")
    .description("Run end-to-end tests on an appchain")
    .requiredOption("--settlement-rpc <url>", "Settlement chain RPC URL")
    .requiredOption("--appchain-rpc <url>", "Appchain RPC URL")
    .requiredOption("--inbox <address>", "Inbox contract address")
    .requiredOption("--private-key <key>", "Private key for transactions")
    .action(async (options: Record<string, unknown>) => {
      const {
        data: validatedOptions,
        success,
        error
      } = appchainE2EOptionsSchema.safeParse(options)

      if (!success) {
        return handleSchemaErrors(error)
      }

      await e2e(validatedOptions)
    })
}
