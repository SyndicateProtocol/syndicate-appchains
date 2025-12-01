import {
  assertionPosterABI,
  assertionPosterBytecode
} from "@/abi/synd/AssertionPoster"
import type { PrivateKeyWalletAccount, PublicClientWithChain } from "@/types"
import { getChainExplorerUrl } from "@/utils/helpers"
import { print } from "@/utils/print"
import { sleep } from "bun"
import { type Hex, parseAbiItem } from "viem"

export async function deployAssertionPoster({
  rollup,
  upgradeExecutor,
  settlementPublicClient,
  deployerSettlementWalletClient,
  ownerSettlementWalletClient
}: {
  rollup: Hex
  upgradeExecutor: Hex
  settlementPublicClient: PublicClientWithChain
  deployerSettlementWalletClient: PrivateKeyWalletAccount
  ownerSettlementWalletClient: PrivateKeyWalletAccount
}) {
  // 1. Get the executor address
  const executorAddress = await settlementPublicClient.readContract({
    address: rollup,
    abi: [parseAbiItem("function owner() view returns (address)")],
    functionName: "owner"
  })
  if (executorAddress !== upgradeExecutor) {
    throw new Error(
      "🚫  Executor address onchain is not the same as the upgrade executor address"
    )
  }

  // 2. Deploy the assertion poster
  print("🔍  Deploying assertion poster...")
  const assertionPosterCreationHash =
    await deployerSettlementWalletClient.deployContract({
      abi: assertionPosterABI,
      bytecode: assertionPosterBytecode,
      account: deployerSettlementWalletClient.account,
      args: [
        rollup,
        "0x0000000000000000000000000000000000000000000000000000000000000000",
        BigInt(0),
        BigInt(1)
      ]
    })
  const receipt = await settlementPublicClient.waitForTransactionReceipt({
    hash: assertionPosterCreationHash
  })
  const assertionPosterAddress = receipt.contractAddress
  if (!assertionPosterAddress) {
    throw new Error("🚫  AssertionPoster address could not be found")
  }
  print(
    `🔍  Assertion poster deployed to ${assertionPosterAddress} in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${
      receipt.transactionHash
    }`
  )

  // sleep to ensure the executor is ready
  await sleep(5000)

  // 3. Initialize the assertion poster through the upgrade executor, this must happen via the rollup owner
  // ABI encoded initialize call: cast calldata "configure()"
  const initializeEncoded = "0x3e0b1a23"
  const initializeData = await ownerSettlementWalletClient.writeContract({
    account: ownerSettlementWalletClient.account,
    address: executorAddress,
    abi: [parseAbiItem("function execute(address, bytes) public")],
    functionName: "execute",
    args: [assertionPosterAddress, initializeEncoded]
  })
  const initializeReceipt =
    await settlementPublicClient.waitForTransactionReceipt({
      hash: initializeData
    })
  if (initializeReceipt.status !== "success") {
    throw new Error("Assertion poster initialization failed")
  }
  print(
    `🔍  Assertion poster initialized in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${
      initializeReceipt.transactionHash
    }`
  )

  return assertionPosterAddress
}
