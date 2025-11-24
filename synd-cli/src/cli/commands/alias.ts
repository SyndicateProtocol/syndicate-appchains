import { applyL1ToL2Alias } from "@/utils/alias"
import { print } from "@/utils/print"
import type { Command } from "@commander-js/extra-typings"
import { ethAddressSchema, handleSchemaErrors } from "../schema"

export function aliasCommand(program: Command) {
  program
    .command("alias")
    .description("Calculate the aliased address for L1->L2 messages")
    .argument("<address>", "The address to alias")
    .action((address: string) => {
      const {
        success,
        data: parsedAddress,
        error
      } = ethAddressSchema.safeParse(address)
      if (!success) {
        return handleSchemaErrors(error)
      }
      print(applyL1ToL2Alias(parsedAddress))
    })
}
