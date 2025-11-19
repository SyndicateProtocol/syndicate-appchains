import type { E2E } from "@/types"
import { parseEther } from "viem"
import { privateKeyToAccount } from "viem/accounts"
import { counterAbi } from "../../../../abi/Counter"
import { print } from "../../../../utils/print"
import { deployCounter } from "./deployCounter"
import { deposit } from "./deposit"
import { transferToSelf } from "./transferToSelf"

export async function e2e({
  inbox,
  privateKey,
  appchainPublicClient,
  appchainWalletClient,
  settlementPublicClient,
  settlementWalletClient
}: E2E) {
  const account = privateKeyToAccount(privateKey)

  const value = parseEther("0.001")
  print("Depositing...")
  await deposit({
    settlementPublicClient,
    settlementWalletClient,
    appchainPublicClient,
    inbox,
    account,
    value
  })
  print("\n\nTransferring to self...")
  const transferValue = value / BigInt(3)
  await transferToSelf({
    appchainWalletClient,
    appchainPublicClient,
    value: transferValue
  })
  print("\n\nDeploying Counter.sol...")
  const contractAddress = await deployCounter({
    appchainPublicClient,
    appchainWalletClient
  })
  print(`Counter.sol deployed at: ${contractAddress}`)

  print("\n\nReading Counter.sol...")
  const readResponse = await appchainPublicClient.readContract({
    address: contractAddress,
    abi: counterAbi,
    functionName: "number"
  })
  print(`Counter.sol number: ${readResponse}`)

  print("\n\nIncrementing Counter.sol...")
  const incrementHash = await appchainWalletClient.writeContract({
    address: contractAddress,
    abi: counterAbi,
    functionName: "increment",
    chain: null
  })
  await appchainPublicClient.waitForTransactionReceipt({ hash: incrementHash })
  print(`Counter.sol incremented: ${incrementHash}`)

  print("\n\nReading Counter.sol...")
  const readAfterResponse = await appchainPublicClient.readContract({
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
