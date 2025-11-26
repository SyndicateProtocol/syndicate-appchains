import type { ExecuteOutbox } from "@/types"
import { registerNetworkInArbSDK } from "@/utils/arbitrumSDKHelpers"
import { getAppchainClients } from "@/utils/clients"
import { print } from "@/utils/print"
import {
  ChildToParentMessageStatus,
  ChildTransactionReceipt
} from "@arbitrum/sdk"
import { JsonRpcProvider } from "@ethersproject/providers"
import { Wallet } from "ethers"

// child chain tx           : 0x5e9125f319ff7fb0d78249fc55e7d8aa952c3b8f53abb15eb5750476c6d4ec7e
// parent chain redemption  : 0x48171248f411868ef6e631c45c7b17819ae6901ddb4a5fd0658e9e106c710243

export async function executeOutbox({
  hash,
  appchainRpc,
  settlementRpc,
  privateKey,
  rollup
}: ExecuteOutbox) {
  const settlementChainProvider = new JsonRpcProvider(settlementRpc)
  const appchainProvider = new JsonRpcProvider(appchainRpc)
  const settlementWallet = new Wallet(privateKey, settlementChainProvider)
  const [appchainPublicClient] = await getAppchainClients(appchainRpc)

  await registerNetworkInArbSDK(
    settlementChainProvider,
    appchainProvider,
    rollup,
    appchainPublicClient.chain.testnet ?? false
  )

  print(`Executing outbox message from appchain tx: ${hash}`)

  const receipt = await appchainProvider.getTransactionReceipt(hash)
  if (!receipt) {
    throw new Error(`Transaction receipt not found for hash: ${hash}`)
  }

  print(`Appchain tx found in block ${receipt.blockNumber}`)
  const transactionReceipt = new ChildTransactionReceipt(receipt)

  /**
   * Note that in principle, a single transaction could trigger any number of outgoing messages; the common case will be there's only one.
   * For the sake of this script, we assume there's only one, so we just grab the first one.
   */
  const messages =
    await transactionReceipt.getChildToParentMessages(settlementWallet)
  if (messages.length === 0) {
    throw new Error("No child-to-parent messages found in this transaction")
  }

  print(`Found ${messages.length} child-to-parent message(s)`)
  if (messages.length > 1) {
    console.warn("Multiple child-to-parent messages found in this transaction")
    console.warn("Using the first one")
  }

  const childToParentMessage = messages[0]
  // @ts-expect-error - nitroReader is private and only accessible within class 'ChildToParentMessageReader'
  const position = childToParentMessage.nitroReader.event.position
  print(`Child to parent message position: ${position}`)

  const status = await childToParentMessage.status(appchainProvider)
  print(`Current status: ${ChildToParentMessageStatus[status]}`)

  if (status === ChildToParentMessageStatus.EXECUTED) {
    print("Message already executed! Exiting...")
    return
  }

  print(
    "Waiting for the outbox entry to be created. This only happens when the appchain's block is confirmed on the settlement chain."
  )
  print(
    "This typically takes around 1 week after block creation (by default).\n"
  )

  const timeToWaitMs = 1000 * 60
  await childToParentMessage.waitUntilReadyToExecute(
    appchainProvider,
    timeToWaitMs
  )
  print("Outbox entry exists! Executing now...\n")

  const executeTransaction =
    await childToParentMessage.execute(appchainProvider)
  const executeTransactionReceipt = await executeTransaction.wait()

  print("✓ Transaction executed successfully!")
  print(`Transaction hash: ${executeTransactionReceipt.transactionHash}`)
  print(`Block number: ${executeTransactionReceipt.blockNumber}`)
}
