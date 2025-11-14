import {
  createPublicClient,
  createWalletClient,
  http,
  parseEther,
  type Hex
} from "viem"
import { privateKeyToAccount } from "viem/accounts"
import { print } from "../utils/print"
import { e2eConfig } from "../utils/schema"
import { counterAbi } from "./abi/CounterAbi"
import { deployCounterContract } from "./deployCounterContract"
import { deposit } from "./deposit"
import { transferToSelf } from "./transferToSelf"

const configFile = Bun.file("./e2e.config.json")
if (!(await configFile.exists())) {
  print("🚫 e2e.config.json not found")
  process.exit(1)
}

const contents = await configFile.json()
const { success, data: config, error } = e2eConfig.safeParse(contents)
if (!success) {
  print("🚫 Invalid e2e.config.json")
  console.error(error)
  process.exit(1)
}
const { settlementRpcUrl, l3RpcUrl, inboxAddress, privateKey } = config
if (!success) {
  print("🚫 Invalid e2e.config.json")
  console.error(error)
}

const account = privateKeyToAccount(privateKey as Hex)
const l3Client = createPublicClient({
  transport: http(l3RpcUrl, { timeout: 60_000 })
})
const l3WalletClient = createWalletClient({
  account,
  transport: http(l3RpcUrl, { timeout: 60_000 })
})
const settlementClient = createPublicClient({
  transport: http(settlementRpcUrl)
})
const settlementWalletClient = createWalletClient({
  account,
  transport: http(settlementRpcUrl)
})

async function e2e() {
  const value = parseEther("0.001")
  print("Depositing...")
  await deposit({
    settlementClient,
    settlementWalletClient,
    l3Client,
    inboxAddress,
    account,
    value
  })
  print("\n\nTransferring to self...")
  const transferValue = value / BigInt(3)
  await transferToSelf({
    l3WalletClient,
    l3Client,
    value: transferValue
  })
  print("\n\nDeploying Counter.sol...")
  const contractAddress = await deployCounterContract({
    l3Client,
    l3WalletClient
  })
  print(`Counter.sol deployed at: ${contractAddress}`)

  print("\n\nReading Counter.sol...")
  const readResponse = await l3Client.readContract({
    address: contractAddress,
    abi: counterAbi,
    functionName: "number"
  })
  print(`Counter.sol number: ${readResponse}`)

  print("\n\nIncrementing Counter.sol...")
  const incrementHash = await l3WalletClient.writeContract({
    address: contractAddress,
    abi: counterAbi,
    functionName: "increment",
    chain: null
  })
  await l3Client.waitForTransactionReceipt({ hash: incrementHash })
  print(`Counter.sol incremented: ${incrementHash}`)

  print("\n\nReading Counter.sol...")
  const readAfterResponse = await l3Client.readContract({
    address: contractAddress,
    abi: counterAbi,
    functionName: "number"
  })
  if (readAfterResponse !== readResponse + BigInt(1)) {
    print("🚫 Counter.sol number is not incremented")
    process.exit(1)
  }
  print(`Counter.sol number: ${readAfterResponse}`)

  print("\n\nDone!")
}

await e2e()
