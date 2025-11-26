import type { CheckRetryable } from "@/types"
import {
  publicClientToProvider,
  registerNetworkInArbSDK
} from "@/utils/arbitrumSDKHelpers"
import { print } from "@/utils/print"
import {
  ParentToChildMessageStatus,
  ParentTransactionReceipt
} from "@arbitrum/sdk"
import type { JsonRpcProvider } from "@ethersproject/providers"

export async function checkRetryable({
  hash,
  settlementPublicClient,
  appchainPublicClient,
  rollup
}: CheckRetryable) {
  const settlementChainProvider: JsonRpcProvider = publicClientToProvider(
    settlementPublicClient
  )
  const appchainProvider: JsonRpcProvider =
    publicClientToProvider(appchainPublicClient)

  await registerNetworkInArbSDK(
    settlementChainProvider,
    appchainProvider,
    rollup,
    appchainPublicClient.chain.testnet ?? false
  )

  print(`Checking retryable tickets for settlement chain tx: ${hash}`)

  const parentReceipt =
    await settlementChainProvider.getTransactionReceipt(hash)
  if (!parentReceipt) {
    throw new Error(`Transaction receipt not found for hash: ${hash}`)
  }

  print(`Settlement chain tx found in block ${parentReceipt.blockNumber}`)

  const parentTxReceipt = new ParentTransactionReceipt(parentReceipt)

  const parentToChildMessages =
    await parentTxReceipt.getParentToChildMessages(appchainProvider)

  print(`Found ${parentToChildMessages.length} retryable ticket(s)`)

  if (parentToChildMessages.length === 0) {
    print("No retryable tickets found in this transaction")
    return
  }

  for (let i = 0; i < parentToChildMessages.length; i++) {
    const message = parentToChildMessages[i]
    print(`\n=== Retryable Ticket ${i + 1}/${parentToChildMessages.length} ===`)
    print(`Retryable creation ID: ${message.retryableCreationId}`)

    const status = await message.status()
    print(`Status: ${ParentToChildMessageStatus[status]}`)

    if (status === ParentToChildMessageStatus.REDEEMED) {
      const result = await message.waitForStatus()
      if ("childTxReceipt" in result) {
        print(`Appchain tx hash: ${result.childTxReceipt.transactionHash}`)
        print(`Appchain block: ${result.childTxReceipt.blockNumber}`)
      }
    }
  }
}
