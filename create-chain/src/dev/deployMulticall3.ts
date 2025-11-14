import { deployMulticall3 } from "../features/deployMulticall3"

async function main() {
  const multicall3Address = await deployMulticall3()
  console.log("Multicall3 deployed to:", multicall3Address)
}

main()
