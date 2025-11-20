import { ArbOwnerABI } from "@/abi/nitro/ArbOwner"
import type { HandoffNitro } from "@/types"
import { ARB_OWNER_PRECOMPILE_ADDRESS } from "@/utils/constants"
import { getChainExplorerUrl } from "@/utils/helpers"
import { print } from "@/utils/print"
import {
  upgradeExecutorPrepareAddExecutorTransactionRequest,
  upgradeExecutorPrepareRemoveExecutorTransactionRequest
} from "@arbitrum/orbit-sdk"
import { checkSequencerInboxConfig } from "./checkSequencerInboxConfig"

export default async function handoffNitro({
  newOwner,
  ownerSettlementWalletClient,
  settlementPublicClient,
  synd,
  ownerAppchainWalletClient,
  appchainPublicClient
}: HandoffNitro) {
  await checkSequencerInboxConfig({
    ownerSettlementWalletClient,
    settlementPublicClient,
    synd
  })

  // 1. Add new owner as EXECUTOR_ROLE on settlement chain
  const addSettlementExecutorTx =
    await upgradeExecutorPrepareAddExecutorTransactionRequest({
      account: newOwner,
      upgradeExecutorAddress: synd.bridge.coreContracts.upgradeExecutor,
      executorAccountAddress: ownerSettlementWalletClient.account.address,
      publicClient: settlementPublicClient
    })
  const addSettlementExecutorHash =
    await settlementPublicClient.sendRawTransaction({
      serializedTransaction:
        await ownerSettlementWalletClient.account.signTransaction(
          addSettlementExecutorTx
        )
    })
  await settlementPublicClient.waitForTransactionReceipt({
    hash: addSettlementExecutorHash
  })
  print(
    `🔍  Added new owner ${newOwner} as EXECUTOR on settlement chain in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${addSettlementExecutorHash}`
  )

  // 2. Remove EXECUTOR_ROLE from syndicate internal owner on the settlement chain
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
  print(
    `🔍  Removed EXECUTOR_ROLE from syndicate internal owner ${ownerSettlementWalletClient.account.address} on the settlement chain in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${settlementRemoveHash}`
  )

  // 3. Remove EXECUTOR_ROLE from syndicate internal owner on the appchain
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
  print(
    `🔍  Removed EXECUTOR_ROLE from syndicate internal owner ${ownerAppchainWalletClient.account.address} on the appchain in ${getChainExplorerUrl(appchainPublicClient.chain)}/tx/${appchainRemoveHash}`
  )

  // 4. Add the UpgradeExecutor as the owner of the ArbOwner precompile
  const addArbOwnerTx = await ownerAppchainWalletClient.writeContract({
    address: ARB_OWNER_PRECOMPILE_ADDRESS,
    abi: ArbOwnerABI,
    functionName: "addChainOwner",
    args: [synd.bridge.tokenBridgeContracts.l3Contracts.upgradeExecutor]
  })
  await appchainPublicClient.waitForTransactionReceipt({
    hash: addArbOwnerTx
  })
  print(
    `🔍  Appchain's UpgradeExecutor added as chain owenr in the ArbOwner precompile in ${getChainExplorerUrl(appchainPublicClient.chain)}/tx/${addArbOwnerTx}`
  )

  // 5. Remove the syndicate internal owner as owner from the ArbOwner precompile
  const removeArbOwnerTx = await ownerAppchainWalletClient.writeContract({
    address: ARB_OWNER_PRECOMPILE_ADDRESS,
    abi: ArbOwnerABI,
    functionName: "removeChainOwner",
    args: [ownerAppchainWalletClient.account.address]
  })
  await appchainPublicClient.waitForTransactionReceipt({
    hash: removeArbOwnerTx
  })
  print(
    `🔍  Syndicate internal owner removed as chain owner from ArbOwner precompile in ${getChainExplorerUrl(appchainPublicClient.chain)}/tx/${removeArbOwnerTx}`
  )
}
