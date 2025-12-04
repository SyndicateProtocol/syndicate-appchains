import type { EnsureOwnerBalance } from "@/types"
import { fundAccount } from "@/utils/fundAccount"
import { isNativeTokenEth } from "@/utils/helpers"
import { print } from "@/utils/print"
import { erc20Abi, formatEther, parseEther, parseUnits } from "viem"

// Ensures the owner has sufficient balance to pay for retryables.
// For ETH native tokens, checks ETH balance directly.
// For ERC20 tokens, checks token balance and prompts to transfer from deployer if needed.
export async function ensureOwnerBalance({
  ownerWalletClient,
  deployerWalletClient,
  publicClient,
  nativeTokenAddress,
  nativeTokenSymbol,
  estimatedCostOfRetryables
}: EnsureOwnerBalance) {
  if (isNativeTokenEth(nativeTokenAddress)) {
    await ensureEthBalance({
      ownerWalletClient,
      deployerWalletClient,
      publicClient,
      estimatedCostOfRetryables
    })
  } else {
    await ensureERC20Balance({
      ownerWalletClient,
      deployerWalletClient,
      publicClient,
      nativeTokenAddress,
      nativeTokenSymbol,
      estimatedCostOfRetryables
    })
  }
}

async function ensureEthBalance({
  ownerWalletClient,
  deployerWalletClient,
  publicClient,
  estimatedCostOfRetryables
}: Omit<EnsureOwnerBalance, "nativeTokenAddress" | "nativeTokenSymbol">) {
  const balance = await publicClient.getBalance({
    address: ownerWalletClient.account.address
  })

  if (balance < estimatedCostOfRetryables) {
    print("---------------------------------------------------------")
    print("                    Insufficient Balance                    ")
    print("⚠️  Owner does not have enough balance for retryables ⚠️")
    print("---------------------------------------------------------")
    print("Owner", ownerWalletClient.account.address)
    print("Owner Balance", `${formatEther(balance)} ETH`)
    print("Required Balance", `${formatEther(estimatedCostOfRetryables)} ETH`)
    print("Deployer", deployerWalletClient.account.address)
    print("---------------------------------------------------------")

    const fundResponse = prompt(
      "Do you want to fund the owner from the deployer? (y/n)"
    )
    const fundConfirmation = ["y", "yes"]
    if (
      !fundResponse ||
      !fundConfirmation.includes(fundResponse.toLowerCase())
    ) {
      throw new Error(
        `🚫 Owner does not have balance of native token on settlement chain. Owner balance: ${formatEther(balance)} ETH, estimated cost of retryables: ${formatEther(estimatedCostOfRetryables)} ETH`
      )
    }

    print("💰  Funding owner from deployer...")
    await fundAccount(
      deployerWalletClient,
      ownerWalletClient.account.address,
      estimatedCostOfRetryables
    )
    print("✅  Owner funded successfully")
  }
}

async function ensureERC20Balance({
  ownerWalletClient,
  deployerWalletClient,
  publicClient,
  nativeTokenAddress,
  nativeTokenSymbol,
  estimatedCostOfRetryables
}: EnsureOwnerBalance) {
  const nativeTokenBalance = await publicClient.readContract({
    address: nativeTokenAddress,
    abi: erc20Abi,
    functionName: "balanceOf",
    args: [ownerWalletClient.account.address]
  })

  const tokenDecimals = publicClient.chain.nativeCurrency.decimals
  const convertedBalance =
    parseUnits(nativeTokenBalance.toString(), tokenDecimals) *
    BigInt(10 ** (18 - tokenDecimals))

  if (convertedBalance <= estimatedCostOfRetryables) {
    print("---------------------------------------------------------")
    print("                    Insufficient Balance                    ")
    print("⚠️  Owner does not have enough balance for retryables ⚠️")
    print("---------------------------------------------------------")
    print("Owner", ownerWalletClient.account.address)
    print(
      "Owner Balance",
      `${formatEther(convertedBalance)} ${nativeTokenSymbol}`
    )
    print(
      "Required Balance",
      `${formatEther(estimatedCostOfRetryables)} ${nativeTokenSymbol}`
    )
    print("Deployer", deployerWalletClient.account.address)
    print("---------------------------------------------------------")

    // Check if deployer has enough balance
    const deployerTokenBalance = await publicClient.readContract({
      address: nativeTokenAddress,
      abi: erc20Abi,
      functionName: "balanceOf",
      args: [deployerWalletClient.account.address]
    })
    const convertedDeployerBalance =
      parseUnits(deployerTokenBalance.toString(), tokenDecimals) *
      BigInt(10 ** (18 - tokenDecimals))

    // Add 10% buffer to the required amount
    const amountWithBuffer =
      (estimatedCostOfRetryables * BigInt(110)) / BigInt(100)

    if (convertedDeployerBalance < amountWithBuffer) {
      throw new Error(
        `🚫 Owner does not have sufficient ${nativeTokenSymbol} balance and deployer cannot fund them. Owner balance: ${formatEther(convertedBalance)} ${nativeTokenSymbol}, deployer balance: ${formatEther(convertedDeployerBalance)} ${nativeTokenSymbol}, required (with 10% buffer): ${formatEther(amountWithBuffer)} ${nativeTokenSymbol}. Please fund the owner with at least ${formatEther(amountWithBuffer)} ${nativeTokenSymbol}`
      )
    }

    const fundResponse = prompt(
      "Do you want to fund the owner from the deployer? (y/n)"
    )
    const fundConfirmation = ["y", "yes"]
    if (
      !fundResponse ||
      !fundConfirmation.includes(fundResponse.toLowerCase())
    ) {
      throw new Error(
        `🚫 Owner does not have balance of ${nativeTokenSymbol} (${nativeTokenAddress}) on the settlement chain. Owner balance: ${formatEther(convertedBalance)} ${nativeTokenSymbol}, estimated cost of retryables: ${formatEther(estimatedCostOfRetryables)} ${nativeTokenSymbol}`
      )
    }

    print(
      `💰  Transferring ${formatEther(amountWithBuffer)} ${nativeTokenSymbol} from deployer to owner...`
    )

    // Convert back to token's native decimals for transfer
    const transferAmount = amountWithBuffer / BigInt(10 ** (18 - tokenDecimals))

    const hash = await deployerWalletClient.writeContract({
      address: nativeTokenAddress,
      abi: erc20Abi,
      functionName: "transfer",
      args: [ownerWalletClient.account.address, transferAmount]
    })

    await publicClient.waitForTransactionReceipt({ hash })
    print("✅  Owner funded successfully")
  }

  // Fund owner for gas fees
  const ethBalance = await publicClient.getBalance({
    address: ownerWalletClient.account.address
  })
  if (ethBalance < parseEther("0.001")) {
    await fundAccount(
      deployerWalletClient,
      ownerWalletClient.account.address,
      parseEther("0.001")
    )
  }
}
