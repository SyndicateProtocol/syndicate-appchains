import { formatEther } from "viem"

import type { TransferToSelf } from "@/types"
import { print } from "../../../../utils/print"

export async function transferToSelf({
  appchainPublicClient,
  appchainWalletClient,
  value
}: TransferToSelf) {
  const to = appchainWalletClient.account.address
  const balanceBefore = await appchainPublicClient.getBalance({ address: to })
  print(
    `Transferring ${formatEther(value)} from ${appchainWalletClient.account.address} to self, balance before: ${formatEther(balanceBefore)}`
  )
  const gasPrice = await appchainPublicClient.getGasPrice()
  const valueToTransfer = value - gasPrice * BigInt(21000)
  const hash = await appchainWalletClient.sendTransaction({
    to,
    value: valueToTransfer,
    chain: null
  })
  print(`Transaction hash: ${hash}`)
  await appchainPublicClient.waitForTransactionReceipt({
    hash,
    retryCount: 10,
    retryDelay: 5_000
  })
  const balanceAfter = await appchainPublicClient.getBalance({ address: to })
  print(
    `Transferred ${formatEther(value)} from ${appchainWalletClient.account.address} to self, final balance: ${formatEther(balanceAfter)}`
  )
}
