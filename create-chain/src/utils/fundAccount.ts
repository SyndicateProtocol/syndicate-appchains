import {
  createPublicClient,
  formatEther,
  http,
  type Account,
  type Address,
  type Chain,
  type Transport,
  type WalletClient
} from "viem"
import { print } from "./print"

export async function fundAccount(
  senderWalletClient: WalletClient<Transport, Chain, Account>,
  receipientAddress: Address,
  desiredBalance: bigint
) {
  const publicClient = createPublicClient({
    chain: senderWalletClient.chain,
    transport: http(senderWalletClient.transport.url)
  })
  const receipientBalance = await publicClient.getBalance({
    address: receipientAddress
  })

  const amountToFund = desiredBalance - receipientBalance
  if (amountToFund > 0) {
    const senderAddress = senderWalletClient.account.address
    const senderBalance = await publicClient.getBalance({
      address: senderAddress
    })
    if (senderBalance < amountToFund) {
      print(
        `Sender account ${senderAddress} has insufficient balance (${formatEther(senderBalance)}) to fund ${formatEther(amountToFund)}, skipping funding...`
      )
      return
    }

    const hash = await senderWalletClient.sendTransaction({
      to: receipientAddress,
      value: amountToFund
    })
    const receipt = await publicClient.waitForTransactionReceipt({
      hash
    })
    print(
      `💰  Funded ${receipientAddress} with ${formatEther(amountToFund)} for total balance of ${formatEther(desiredBalance)}: ${receipt.transactionHash}`
    )
  } else {
    print(
      `💰  ${receipientAddress} already has ${formatEther(receipientBalance)}, skipping funding...`
    )
  }
}
