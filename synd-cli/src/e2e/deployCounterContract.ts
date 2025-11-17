import type { Account, PublicClient, Transport, WalletClient } from "viem"
import { counterAbi, counterBytecode } from "./abi/CounterAbi"

interface DeployCounterContract {
  l3Client: PublicClient
  l3WalletClient: WalletClient<Transport, undefined, Account>
}

export async function deployCounterContract({
  l3Client,
  l3WalletClient
}: DeployCounterContract) {
  // Generally Multicall3 is deployed at the same address via a presigned transaction, but due to gas estimation differences
  // on arbitrum, we deploy it explicitly. Read more here: https://github.com/mds1/multicall3?tab=readme-ov-file#new-deployments
  const hash = await l3WalletClient.deployContract({
    abi: counterAbi,
    bytecode: counterBytecode,
    chain: null
  })
  const receipt = await l3Client.waitForTransactionReceipt({ hash })
  const contractAddress = receipt.contractAddress
  if (!contractAddress) {
    throw new Error("Contract address could not be found")
  }
  return contractAddress
}
