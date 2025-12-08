import type { Command } from "@commander-js/extra-typings"
import { arbOwnerCommand } from "./arbOwner"
import { checkRetryableCommand } from "./checkRetryable"
import { checkTokenBridgeCommand } from "./checkTokenBridge/"
import { createAppchainCommand } from "./create"
import { e2eCommand } from "./e2e"
import { executeOutboxCommand } from "./executeOutbox"
import { handoffCommand } from "./handoff"
import { determineSequencingChainAddressCommand } from "./determineSequencingChainAddress"

export function appchainCommand(program: Command) {
  const appchainProgram = program
    .command("appchain")
    .description("Manage appchains")

  createAppchainCommand(appchainProgram)
  handoffCommand(appchainProgram)
  arbOwnerCommand(appchainProgram)
  checkTokenBridgeCommand(appchainProgram)
  checkRetryableCommand(appchainProgram)
  executeOutboxCommand(appchainProgram)
  e2eCommand(appchainProgram)
  determineSequencingChainAddressCommand(appchainProgram)
}