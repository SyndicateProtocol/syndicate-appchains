import { ArbOwnerABI } from "../abi/nitro/ArbOwner"
import { getHandoffConfig } from "../utils/config"
import {
  ARB_OWNER_PRECOMPILE_ADDRESS,
  DEFAULT_APPCHAIN_MIN_BASE_FEE
} from "../utils/constants"
import { getChainExplorerUrl } from "../utils/helpers"
import { print } from "../utils/print"

export default async function setAppchainConfig() {
  const { appchainPublicClient, ownerAppchainWalletClient, newOwnerAddress } =
    await getHandoffConfig()

  // The chain owner is the only one who can set the config
  const isSignerChainOwner = await appchainPublicClient.readContract({
    address: ARB_OWNER_PRECOMPILE_ADDRESS,
    abi: ArbOwnerABI,
    functionName: "isChainOwner",
    args: [ownerAppchainWalletClient.account.address],
    account: ownerAppchainWalletClient.account
  })

  if (!isSignerChainOwner) {
    throw new Error("The address you have provided is not the chain owner")
  }

  // 1. Set the min base fee
  const tx1Hash = await ownerAppchainWalletClient.writeContract({
    address: ARB_OWNER_PRECOMPILE_ADDRESS,
    abi: ArbOwnerABI,
    functionName: "setMinimumL2BaseFee",
    args: [BigInt(DEFAULT_APPCHAIN_MIN_BASE_FEE)]
  })

  const receipt1 = await appchainPublicClient.waitForTransactionReceipt({
    hash: tx1Hash
  })
  if (receipt1.status === "reverted") {
    throw new Error("Transaction failed, could not set the Minimum base fee")
  }
  print(
    `🔍  Minimum base fee set to ${DEFAULT_APPCHAIN_MIN_BASE_FEE} wei on appchain in ${getChainExplorerUrl(appchainPublicClient.chain)}/tx/${tx1Hash}`
  )

  // 2. Set the network fee receiver - this should be a customer provided address
  // Collects the L2 surplus fees
  const tx2Hash = await ownerAppchainWalletClient.writeContract({
    address: ARB_OWNER_PRECOMPILE_ADDRESS,
    abi: ArbOwnerABI,
    functionName: "setNetworkFeeAccount",
    args: [newOwnerAddress]
  })

  const receipt2 = await appchainPublicClient.waitForTransactionReceipt({
    hash: tx2Hash
  })
  if (receipt2.status === "reverted") {
    throw new Error("Network fee receiver setting transaction failed")
  }
  print(
    `🔍  Network fee receiver set to ${newOwnerAddress} on appchain in ${getChainExplorerUrl(appchainPublicClient.chain)}/tx/${tx2Hash}`
  )

  // 3. Set the infrastructure fee collector
  // Collects the L2 base fees
  const tx3Hash = await ownerAppchainWalletClient.writeContract({
    address: ARB_OWNER_PRECOMPILE_ADDRESS,
    abi: ArbOwnerABI,
    functionName: "setInfraFeeAccount",
    args: [newOwnerAddress]
  })

  const receipt3 = await appchainPublicClient.waitForTransactionReceipt({
    hash: tx3Hash
  })
  if (receipt3.status === "reverted") {
    throw new Error("Setting infrastructure fee collector transaction failed")
  }
  print(
    `🔍  Infrastructure fee collector set to ${newOwnerAddress} on appchain in ${getChainExplorerUrl(appchainPublicClient.chain)}/tx/${tx3Hash}`
  )
}
