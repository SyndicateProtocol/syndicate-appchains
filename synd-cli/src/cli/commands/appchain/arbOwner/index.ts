import type { Command } from "@commander-js/extra-typings"
import { callArbOwnerCommand } from "./call"
import { listArbOwnerCommand } from "./list"

export function arbOwnerCommand(program: Command) {
  const arbOwner = program
    .command("arb-owner")
    .description("Call ArbOwner functions through the parent UpgradeExecutor")

  listArbOwnerCommand(arbOwner)
  callArbOwnerCommand(arbOwner)
}
