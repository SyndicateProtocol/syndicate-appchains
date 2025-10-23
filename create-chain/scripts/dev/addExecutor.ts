import { upgradeExecutorPrepareAddExecutorTransactionRequest } from "@arbitrum/orbit-sdk"
import { getHandoffConfig } from "../utils/config"

export default async function main() {
  const {
    newOwnerAddress,
    ownerSettlementWalletClient,
    ownerAppchainWalletClient,
    settlementPublicClient,
    sequencingPublicClient,
    appchainPublicClient,
    synd
  } = await getHandoffConfig()

  console.log("New Owner Address:", newOwnerAddress)
  console.log(
    "Owner Settlement Wallet Address:",
    ownerSettlementWalletClient.account.address
  )
  console.log(
    "Owner Appchain Wallet Address:",
    ownerAppchainWalletClient.account.address
  )
  console.log("Settlement Public Client:", settlementPublicClient.chain.name)
  console.log("Sequencing Public Client:", sequencingPublicClient.chain.name)
  console.log("Appchain Public Client:", appchainPublicClient.chain.name)

  // prompt
  const promptResponse = prompt("Are you sure you want to proceed? (y/n)")
  const confirmation = ["y", "yes"]
  if (!promptResponse || !confirmation.includes(promptResponse.toLowerCase())) {
    console.log("🚫 Exiting...")
    return
  }

  // 1. Add EXECUTOR_ROLE to the new owner on the settlement chain
  const addSettlementExecutorTx =
    await upgradeExecutorPrepareAddExecutorTransactionRequest({
      account: newOwnerAddress,
      upgradeExecutorAddress: synd.bridge.coreContracts.upgradeExecutor,
      executorAccountAddress: ownerSettlementWalletClient.account.address,
      publicClient: settlementPublicClient
    })
  const settlementHash = await settlementPublicClient.sendRawTransaction({
    serializedTransaction:
      await ownerSettlementWalletClient.account.signTransaction(
        addSettlementExecutorTx
      )
  })
  await settlementPublicClient.waitForTransactionReceipt({
    hash: settlementHash
  })
  console.log("Settlement hash:", settlementHash)

  // 2. Add EXECUTOR_ROLE to the new owner on the appchain
  const addAppchainExecutorTx =
    await upgradeExecutorPrepareAddExecutorTransactionRequest({
      account: newOwnerAddress,
      upgradeExecutorAddress:
        synd.bridge.tokenBridgeContracts.l3Contracts.upgradeExecutor,
      executorAccountAddress: ownerAppchainWalletClient.account.address,
      publicClient: appchainPublicClient
    })
  const appchainHash = await appchainPublicClient.sendRawTransaction({
    serializedTransaction:
      await ownerAppchainWalletClient.account.signTransaction(
        addAppchainExecutorTx
      )
  })
  await appchainPublicClient.waitForTransactionReceipt({
    hash: appchainHash
  })
  console.log("Appchain hash:", appchainHash)
}

main().catch(console.error)
