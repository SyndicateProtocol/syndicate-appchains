import { createTokenBridgeFetchTokenBridgeContracts } from "@arbitrum/orbit-sdk"
import { getFeaturesConfig } from "../utils/config"
import { supportedSettlementChains } from "../utils/constants"
import { generateBridgeConfig } from "../utils/generateBridgeConfig"
import { getChainRpcUrl, upsertToSyndObject } from "../utils/helpers"

async function main() {
  const {
    coreContracts,
    settlementPublicClient,
    appchainPublicClient,
    chainName,
    chainId,
    ownerSettlementWalletClient
  } = await getFeaturesConfig()
  const tokenBridgeContracts = await createTokenBridgeFetchTokenBridgeContracts(
    {
      inbox: coreContracts.inbox,
      parentChainPublicClient: settlementPublicClient,
      tokenBridgeCreatorAddressOverride:
        supportedSettlementChains[settlementPublicClient.chain.id]
          .tokenBridgeCreatorAddress
    }
  )
  console.log(tokenBridgeContracts)

  const environment = settlementPublicClient.chain.testnet
    ? "testnet"
    : "mainnet"

  await upsertToSyndObject(
    chainName,
    environment,
    "bridge",
    generateBridgeConfig({
      explorerUrl: appchainPublicClient.chain.blockExplorers?.default.url,
      parentChainId: settlementPublicClient.chain.id,
      rpcUrl: getChainRpcUrl(appchainPublicClient.chain),
      tokenContracts: {
        l2Contracts: {
          ...tokenBridgeContracts.parentChainContracts,
          proxyAdmin: coreContracts.adminProxy
        },
        l3Contracts: tokenBridgeContracts.orbitChainContracts
      },
      coreContracts,
      chainName,
      chainId,
      rollupOwnerAddress: ownerSettlementWalletClient.account.address
    })
  )
}

main()
