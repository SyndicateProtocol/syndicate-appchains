import { applyL1ToL2Alias } from "../../utils/alias"

export async function aliasCommand(args: string[]) {
  const address = args[0]

  if (!address) {
    console.error(`❌ Missing address for alias command`)
    console.error("\nUsage: bun cli alias <ADDRESS>")
    process.exit(1)
  }

  const aliasedAddress = applyL1ToL2Alias(address)
  console.log(`Original:  ${address}`)
  console.log(`Aliased:   ${aliasedAddress}`)
}
