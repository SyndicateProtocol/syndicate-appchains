import { getBatchPosters } from "./batchPoster"
import { getConfigAndCoreContracts } from "./getConfigAndCoreContracts"

import type { CreateSettlementRollupParams } from "@/src/types"
import { sleep } from "bun"
import { stringify } from "viem"
import { getFoundationConfig } from "../utils/config"
import { generateBridgeConfig } from "../utils/generateBridgeConfig"
import { print } from "../utils/print"
import { createRollup } from "./createRollup"
import { getValidators } from "./validator"

export async function deployNitroRollup({
  validators,
  batchPosters,
  batchPosterManager
}: CreateSettlementRollupParams) {
  const {
    chainId,
    chainName,
    ownerSettlementWalletClient,
    settlementPublicClient,
    appChainRpcUrl,
    appChainExplorerUrl
  } = await getFoundationConfig()
  const hash = await createRollup({
    validators,
    batchPosters,
    batchPosterManager
  })
  print("🫷  Waiting for 10 seconds before fetching chain config...")
  await sleep(10000)

  const { chainConfig, coreContracts } = await getConfigAndCoreContracts({
    hash
  })

  const bridgeConfig = generateBridgeConfig({
    coreContracts,
    chainName,
    chainId,
    parentChainId: settlementPublicClient.chain.id,
    rollupOwnerAddress: ownerSettlementWalletClient.account.address,
    rpcUrl: appChainRpcUrl,
    explorerUrl: appChainExplorerUrl
  })
  print("🔍  Bridge Config")
  print(stringify(bridgeConfig, null, 2))

  const onChainBatchPosters = await getBatchPosters(coreContracts)
  print(
    `🔍  Batch posters from deployment: ${onChainBatchPosters.batchPosters}`
  )
  if (!onChainBatchPosters.isAccurate) {
    print("🚫 Batch posters are not accurate")
  }

  if (batchPosters.length !== onChainBatchPosters.batchPosters.length) {
    throw new Error("Batch posters are not accurate")
  }

  for (const batchPoster of batchPosters) {
    if (!onChainBatchPosters.batchPosters.includes(batchPoster)) {
      throw new Error(`Batch poster ${batchPoster} not found on chain`)
    }
  }

  const onChainValidators = await getValidators(coreContracts)
  print(`🔍  Validators from deployment: ${onChainValidators.validators}`)
  if (!onChainValidators.isAccurate) {
    print("🚫 Validators are not accurate")
  }

  if (validators.length !== onChainValidators.validators.length) {
    throw new Error("Validators are not accurate")
  }

  for (const validator of validators) {
    if (!onChainValidators.validators.includes(validator)) {
      throw new Error(`Validator ${validator} not found on chain`)
    }
  }

  return {
    chainConfig,
    bridgeConfig,
    coreContracts
  }
}
