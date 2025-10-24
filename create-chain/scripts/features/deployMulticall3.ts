import { multicall3Abi, multicall3Bytecode } from "../abi/Multicall3"
import { getFeaturesConfig } from "../utils/config"
import { getChainExplorerUrl } from "../utils/helpers"
import { print } from "../utils/print"

export async function deployMulticall3() {
  print("🔎 Deploying multicall3")
  const { deployerAppchainWalletClient, appchainPublicClient } =
    await getFeaturesConfig()
  const multicall3Hash = await deployerAppchainWalletClient.deployContract({
    abi: multicall3Abi,
    bytecode: multicall3Bytecode,
    account: deployerAppchainWalletClient.account,
    chain: null
  })
  const receipt = await appchainPublicClient.waitForTransactionReceipt({
    hash: multicall3Hash
  })
  const contractAddress = receipt.contractAddress
  if (!contractAddress) {
    throw new Error("Multicall3 contract address could not be found")
  }
  print(
    `🏁  Multicall3 deployed to ${contractAddress} at ${getChainExplorerUrl(
      appchainPublicClient.chain
    )}/tx/${multicall3Hash}`
  )
  return contractAddress
}

export async function canDeployMulticall3() {
  const { deployerAppchainWalletClient, appchainPublicClient } =
    await getFeaturesConfig()

  const balance = await appchainPublicClient.getBalance({
    address: deployerAppchainWalletClient.account.address
  })

  if (balance === BigInt(0)) {
    print(
      "⏪ Deployer does not have balance on appchain. Skipping multicall3 deployment"
    )
    return false
  }

  const [{ maxFeePerGas }, gas] = await Promise.all([
    appchainPublicClient.estimateFeesPerGas(),
    appchainPublicClient.estimateGas({
      account: deployerAppchainWalletClient.account,
      data: multicall3Bytecode
    })
  ])
  return balance > gas * maxFeePerGas
}
