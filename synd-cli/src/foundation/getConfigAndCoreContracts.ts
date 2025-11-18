import {
  createRollupPrepareTransaction,
  createRollupPrepareTransactionReceipt
} from "@arbitrum/orbit-sdk"

import type { ChainConfig } from "@arbitrum/orbit-sdk"

import type { Chain, Hex, PublicClient, Transaction, Transport } from "viem"

export async function getConfigAndCoreContracts({
  hash,
  settlementPublicClient
}: { hash: Hex; settlementPublicClient: PublicClient<Transport, Chain> }) {
  // get the transaction
  const tx = createRollupPrepareTransaction(
    (await settlementPublicClient.getTransaction({
      hash
    })) as Transaction
  )

  // get the transaction receipt
  const txReceipt = createRollupPrepareTransactionReceipt(
    await settlementPublicClient.getTransactionReceipt({ hash })
  )

  // get the chain config from the transaction inputs
  const chainConfig: ChainConfig = JSON.parse(
    tx.getInputs()[0].config.chainConfig
  )
  // get the core contracts from the transaction receipt
  const coreContracts = txReceipt.getCoreContracts()
  return { chainConfig, coreContracts }
}
