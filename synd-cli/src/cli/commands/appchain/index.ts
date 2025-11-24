import { print } from "@/utils/print"
import type { Command } from "@commander-js/extra-typings"
import { arbOwnerCommand } from "./arbOwner"
import { checkTokenBridgeCommand } from "./checkTokenBridge/"
import { createAppchainCommand } from "./create"
import { e2eCommand } from "./e2e"
import { handoffCommand } from "./handoff"

export function appchainCommand(program: Command) {
  const appchainProgram = program
    .command("appchain")
    .description("Manage appchains")
    .action(async () => {
      print("Managing appchains...")
    })

  createAppchainCommand(appchainProgram)
  handoffCommand(appchainProgram)
  arbOwnerCommand(appchainProgram)
  checkTokenBridgeCommand(appchainProgram)
  e2eCommand(appchainProgram)
}
