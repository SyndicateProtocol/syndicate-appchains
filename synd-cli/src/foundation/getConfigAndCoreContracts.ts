import {
  createRollupPrepareTransaction,
  createRollupPrepareTransactionReceipt
} from "@arbitrum/orbit-sdk"

import type { ChainConfig } from "@arbitrum/orbit-sdk"

import type { Hex, Transaction } from "viem"
import { getFoundationConfig } from "../utils/config"

export async function getConfigAndCoreContracts({ hash }: { hash: Hex }) {
  const { settlementPublicClient } = await getFoundationConfig()
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
