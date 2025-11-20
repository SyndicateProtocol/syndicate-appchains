import type { DeployNitroRollupParams } from "@/types"
import { generateBridgeConfig } from "@/utils/generateBridgeConfig"
import { print } from "@/utils/print"
import { sleep } from "bun"
import { stringify } from "viem"
import { deployRollup } from "./deployRollup"
import { getConfigAndCoreContracts } from "./getConfigAndCoreContracts"

export async function deployNitroRollup({
  chainId,
  chainName,
  ownerSettlementWalletClient,
  settlementPublicClient,
  appchainRpc,
  appchainExplorer,
  nativeToken,
  deployerSettlementWalletClient
}: DeployNitroRollupParams) {
  const hash = await deployRollup({
    chainId,
    nativeToken,
    deployerSettlementWalletClient,
    ownerSettlementWalletClient,
    settlementPublicClient
  })
  print("🫷  Waiting for 10 seconds before fetching chain config...")
  await sleep(10000)

  const { chainConfig, coreContracts } = await getConfigAndCoreContracts({
    hash,
    settlementPublicClient
  })

  const bridgeConfig = generateBridgeConfig({
    coreContracts,
    chainName,
    chainId,
    parentChainId: settlementPublicClient.chain.id,
    chainOwner: ownerSettlementWalletClient.account.address,
    rpcUrl: appchainRpc,
    explorerUrl: appchainExplorer
  })
  print("🔍  Bridge Config")
  print(stringify(bridgeConfig, null, 2))

  return {
    chainConfig,
    bridgeConfig,
    coreContracts
  }
}
