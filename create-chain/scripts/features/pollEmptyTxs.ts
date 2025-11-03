import { getChainExplorerUrl } from "../utils/helpers"
import { print } from "../utils/print"

import type { Account, Chain, Transport, WalletClient } from "viem"

export function pollEmptyTxs(
  walletClient: WalletClient<Transport, Chain, Account>
) {
  const sendEmptyTx = async () => {
    const hash = await walletClient.sendTransaction({
      account: walletClient.account,
      to: walletClient.account.address,
      value: BigInt(0),
      data: "0x"
    })
    print(
      `🔄  Sent empty transaction to sequencing chain at: ${getChainExplorerUrl(
        walletClient.chain
      )}/tx/${hash}`
    )
  }

  sendEmptyTx()

  // Set up interval for subsequent transactions
  const interval = setInterval(sendEmptyTx, 20_000)
  return interval
}
