import { upgradeExecutorPrepareRemoveExecutorTransactionRequest } from "@arbitrum/orbit-sdk"
import { getHandoffConfig } from "../utils/config"

export default async function main() {
  const {
    newOwnerAddress,
    ownerSettlementWalletClient,
    ownerSequencingWalletClient,
    ownerAppchainWalletClient,
    settlementPublicClient,
    sequencingPublicClient,
    appchainPublicClient,
    synd
  } = await getHandoffConfig()

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

  // 3. Remove EXECUTOR_ROLE from the old owner on the settlement chain
  const removeSettlementExecutorTx =
    await upgradeExecutorPrepareRemoveExecutorTransactionRequest({
      account: ownerSettlementWalletClient.account.address,
      upgradeExecutorAddress: synd.bridge.coreContracts.upgradeExecutor,
      executorAccountAddress: ownerSettlementWalletClient.account.address,
      publicClient: settlementPublicClient
    })
  const settlementRemoveHash = await settlementPublicClient.sendRawTransaction({
    serializedTransaction:
      await ownerSettlementWalletClient.account.signTransaction(
        removeSettlementExecutorTx
      )
  })
  await settlementPublicClient.waitForTransactionReceipt({
    hash: settlementRemoveHash
  })
  console.log("Settlement remove hash:", settlementRemoveHash)

  // 4. Remove EXECUTOR_ROLE from the old owner on the appchain
  const removeAppchainExecutorTx =
    await upgradeExecutorPrepareRemoveExecutorTransactionRequest({
      account: ownerAppchainWalletClient.account.address,
      upgradeExecutorAddress:
        synd.bridge.tokenBridgeContracts.l3Contracts.upgradeExecutor,
      executorAccountAddress: ownerAppchainWalletClient.account.address,
      publicClient: appchainPublicClient
    })
  const appchainRemoveHash = await appchainPublicClient.sendRawTransaction({
    serializedTransaction:
      await ownerAppchainWalletClient.account.signTransaction(
        removeAppchainExecutorTx
      )
  })
  await appchainPublicClient.waitForTransactionReceipt({
    hash: appchainRemoveHash
  })
  console.log("Appchain remove hash:", appchainRemoveHash)
}

main().catch(console.error)
