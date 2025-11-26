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
  parentTxHash,
  parentPublicClient,
  childPublicClient,
  rollup
}: CheckRetryable) {
  const parentProvider: JsonRpcProvider =
    publicClientToProvider(parentPublicClient)
  const childProvider: JsonRpcProvider =
    publicClientToProvider(childPublicClient)

  await registerNetworkInArbSDK(
    parentProvider,
    childProvider,
    rollup,
    childPublicClient.chain.testnet ?? false
  )

  print(`Checking retryable tickets for parent tx: ${parentTxHash}`)

  // Get the parent transaction receipt
  const parentReceipt = await parentProvider.getTransactionReceipt(parentTxHash)
  if (!parentReceipt) {
    throw new Error(`Transaction receipt not found for hash: ${parentTxHash}`)
  }

  print(`Parent tx found in block ${parentReceipt.blockNumber}`)

  // Wrap it in ParentTransactionReceipt
  const parentTxReceipt = new ParentTransactionReceipt(parentReceipt)

  // Get ParentToChildMessages from the receipt
  const parentToChildMessages =
    await parentTxReceipt.getParentToChildMessages(childProvider)

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
        print(`Child chain tx hash: ${result.childTxReceipt.transactionHash}`)
        print(`Child chain block: ${result.childTxReceipt.blockNumber}`)
      }
    }
  }
}
