import {
	type Account,
	type PublicClient,
	type Transport,
	type WalletClient,
	formatEther,
} from "viem"

import { print } from "../../../../utils/print"

interface TransferConfig {
	l3Client: PublicClient
	l3WalletClient: WalletClient<Transport, undefined, Account>
	value: bigint
}

export async function transferToSelf({
	l3Client,
	l3WalletClient,
	value,
}: TransferConfig) {
	const to = l3WalletClient.account.address
	const balanceBefore = await l3Client.getBalance({ address: to })
	print(
		`Transferring ${formatEther(value)} from ${l3WalletClient.account.address} to self, balance before: ${formatEther(balanceBefore)}`,
	)
	const gasPrice = await l3Client.getGasPrice()
	const valueToTransfer = value - gasPrice * BigInt(21000)
	const hash = await l3WalletClient.sendTransaction({
		to,
		value: valueToTransfer,
		chain: null,
	})
	print(`Transaction hash: ${hash}`)
	await l3Client.waitForTransactionReceipt({
		hash,
		retryCount: 10,
		retryDelay: 5_000,
	})
	const balanceAfter = await l3Client.getBalance({ address: to })
	print(
		`Transferred ${formatEther(value)} from ${l3WalletClient.account.address} to self, final balance: ${formatEther(balanceAfter)}`,
	)
}
