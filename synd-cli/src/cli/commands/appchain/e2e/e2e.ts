import type { E2E } from "@/types"
import { createPublicClient, createWalletClient, http, parseEther } from "viem"
import { privateKeyToAccount } from "viem/accounts"
import { print } from "../../../../utils/print"
import { counterAbi } from "./abi/CounterAbi"
import { deployCounterContract } from "./deployCounterContract"
import { deposit } from "./deposit"
import { transferToSelf } from "./transferToSelf"

export async function e2e({
  settlementRpc,
  appchainRpc,
  inbox,
  privateKey
}: E2E) {
  const account = privateKeyToAccount(privateKey)
  const l3Client = createPublicClient({
    transport: http(appchainRpc, { timeout: 60_000 })
  })
  const l3WalletClient = createWalletClient({
    account,
    transport: http(appchainRpc, { timeout: 60_000 })
  })
  const settlementClient = createPublicClient({
    transport: http(settlementRpc)
  })
  const settlementWalletClient = createWalletClient({
    account,
    transport: http(settlementRpc)
  })

  const value = parseEther("0.001")
  print("Depositing...")
  await deposit({
    settlementClient,
    settlementWalletClient,
    l3Client,
    inbox,
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
