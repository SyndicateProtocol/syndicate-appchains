import { counterAbi, counterBytecode } from "@/abi/Counter"
import type { DeployCounter } from "@/types"

export async function deployCounter({
  appchainPublicClient,
  appchainWalletClient
}: DeployCounter) {
  const hash = await appchainWalletClient.deployContract({
    abi: counterAbi,
    bytecode: counterBytecode
  })
  const receipt = await appchainPublicClient.waitForTransactionReceipt({ hash })
  const contractAddress = receipt.contractAddress
  if (!contractAddress) {
    throw new Error("Contract address could not be found")
  }
  return contractAddress
}
