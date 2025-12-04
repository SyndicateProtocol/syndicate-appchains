import type { CheckTokenBridge } from "@/types"
import {
  publicClientToProvider,
  registerNetworkInArbSDK
} from "@/utils/arbitrumSDKHelpers"
import { getTokenBridgeContracts } from "@/utils/getTokenBridgeContracts"
import { print } from "@/utils/print"
import { createTokenBridgePrepareTransactionReceipt } from "@arbitrum/orbit-sdk"
import { stringify } from "viem"

export async function checkTokenBridge({
  rollup,
  appchainPublicClient,
  settlementPublicClient,
  createdAtHash
}: CheckTokenBridge) {
  registerNetworkInArbSDK(
    publicClientToProvider(settlementPublicClient),
    publicClientToProvider(appchainPublicClient),
    rollup,
    settlementPublicClient.chain.testnet ?? false
  )

  const transaction = await settlementPublicClient.getTransaction({
    hash: createdAtHash
  })
  print("Bridge created at transaction", stringify(transaction, null, 2))

  const tokenBridgeCreatorAddress = transaction?.to
  if (!tokenBridgeCreatorAddress) {
    throw new Error("Token bridge creator address not found")
  }

  const txReceipt = createTokenBridgePrepareTransactionReceipt(
    await settlementPublicClient.waitForTransactionReceipt({
      hash: createdAtHash
    })
  )

  print("Waiting for retryable tickets to execute on the Orbit chain...")
  const orbitChainRetryableReceipts = await txReceipt.waitForRetryables({
    orbitPublicClient: appchainPublicClient
  })
  print("Retryables executed")
  print(
    "Transaction hash for first retryable",
    orbitChainRetryableReceipts[0].transactionHash
  )
  print(
    "Transaction hash for second retryable",
    orbitChainRetryableReceipts[1].transactionHash
  )

  // fetching the TokenBridge contracts
  const tokenBridgeContracts = await getTokenBridgeContracts({
    bridgeCreationHash: createdAtHash,
    parentChainPublicClient: settlementPublicClient,
    tokenBridgeCreatorAddressOverride: tokenBridgeCreatorAddress
  })
  print("TokenBridge contracts fetched")
  print(stringify(tokenBridgeContracts, null, 2))
}
