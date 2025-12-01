import { getTokenBridgeContracts } from "@/utils/getTokenBridgeContracts"
import { getChainExplorerUrl } from "@/utils/helpers"
import { print } from "@/utils/print"
import {
  type CreateTokenBridgeEnoughCustomFeeTokenAllowanceParams,
  type CreateTokenBridgeParams,
  createTokenBridgeEnoughCustomFeeTokenAllowance,
  createTokenBridgePrepareCustomFeeTokenApprovalTransactionRequest,
  createTokenBridgePrepareSetWethGatewayTransactionReceipt,
  createTokenBridgePrepareSetWethGatewayTransactionRequest,
  createTokenBridgePrepareTransactionReceipt,
  createTokenBridgePrepareTransactionRequest,
  isTokenBridgeDeployed
} from "@arbitrum/orbit-sdk"
import {
  type Chain,
  type Hex,
  type PublicClient,
  type Transport,
  zeroAddress
} from "viem"

// Source: https://github.com/OffchainLabs/arbitrum-orbit-sdk/blob/7143a874a94dc0d59d076a0407319f4927f5f49d/src/createTokenBridge.ts#L171-L172
export async function createTokenBridge<
  TParentChain extends Chain | undefined,
  TOrbitChain extends Chain | undefined
>({
  rollupOwner,
  rollupAddress,
  rollupDeploymentBlockNumber,
  account,
  nativeTokenAddress,
  parentChainPublicClient,
  orbitChainPublicClient,
  tokenBridgeCreatorAddressOverride,
  gasOverrides,
  retryableGasOverrides,
  setWethGatewayGasOverrides
}: CreateTokenBridgeParams<TParentChain, TOrbitChain> & {
  tokenBridgeCreatorAddressOverride: Hex
  parentChainPublicClient: PublicClient<Transport, Chain>
}) {
  const isTokenBridgeAlreadyDeployed = await isTokenBridgeDeployed({
    parentChainPublicClient,
    orbitChainPublicClient,
    rollup: rollupAddress,
    tokenBridgeCreatorAddressOverride
  })

  if (isTokenBridgeAlreadyDeployed) {
    throw new Error(
      `Token bridge contracts for Rollup ${rollupAddress} are already deployed`
    )
  }

  const isCustomFeeTokenBridge =
    nativeTokenAddress && nativeTokenAddress !== zeroAddress
  if (isCustomFeeTokenBridge) {
    // set the custom fee token
    // prepare transaction to approve custom fee token spend
    const allowanceParams: CreateTokenBridgeEnoughCustomFeeTokenAllowanceParams<TParentChain> =
      {
        nativeToken: nativeTokenAddress,
        owner: account.address,
        publicClient: parentChainPublicClient,
        tokenBridgeCreatorAddressOverride
      }

    // Check allowance and approve if necessary
    if (
      !(await createTokenBridgeEnoughCustomFeeTokenAllowance(allowanceParams))
    ) {
      const approvalTxRequest =
        await createTokenBridgePrepareCustomFeeTokenApprovalTransactionRequest(
          allowanceParams
        )

      // sign and send the transaction
      const approvalTxHash = await parentChainPublicClient.sendRawTransaction({
        serializedTransaction: await account.signTransaction(approvalTxRequest)
      })

      // get the transaction receipt after waiting for the transaction to complete
      const approvalTxReceipt =
        await parentChainPublicClient.waitForTransactionReceipt({
          hash: approvalTxHash
        })

      print(
        `Tokens approved in ${getChainExplorerUrl(parentChainPublicClient.chain)}/tx/${
          approvalTxReceipt.transactionHash
        }`
      )
    }
  }

  // prepare the transaction for deploying the core contracts
  const txRequest = await createTokenBridgePrepareTransactionRequest({
    params: {
      rollup: rollupAddress,
      rollupOwner
    },
    parentChainPublicClient,
    orbitChainPublicClient,
    account: account.address,
    tokenBridgeCreatorAddressOverride,
    gasOverrides,
    retryableGasOverrides
  })

  // sign and send the transaction
  print("🔎  Deploying the non-native token bridge...")
  const txHash = await parentChainPublicClient.sendRawTransaction({
    serializedTransaction: await account.signTransaction(txRequest)
  })

  // wait for the transaction to be mined
  const receipt = await parentChainPublicClient.waitForTransactionReceipt({
    hash: txHash
  })

  const transaction = await parentChainPublicClient.getTransaction({
    hash: receipt.transactionHash
  })

  // get the transaction receipt after waiting for the transaction to complete
  const txReceipt = createTokenBridgePrepareTransactionReceipt(receipt)
  print(
    `🔎  Token bridge deployed in ${getChainExplorerUrl(parentChainPublicClient.chain)}/tx/${
      txReceipt.transactionHash
    }`
  )

  // wait for retryables to execute
  print("🔎  Waiting for retryable tickets to execute on the appchain...")
  const orbitChainRetryableReceipts = await txReceipt.waitForRetryables({
    // @ts-expect-error (todo: fix viem type issue)
    orbitPublicClient: orbitChainPublicClient
  })
  print(
    `🔎  Transaction hash for first retryable is ${orbitChainRetryableReceipts[0].transactionHash}`
  )
  print(
    `🔎  Transaction hash for second retryable is ${orbitChainRetryableReceipts[1].transactionHash}`
  )

  const tokenBridgeContracts = await getTokenBridgeContracts({
    bridgeCreationHash: txReceipt.transactionHash,
    parentChainPublicClient,
    tokenBridgeCreatorAddressOverride
  })

  // Non custom fee token
  if (!isCustomFeeTokenBridge) {
    // set weth gateway
    const setWethGatewayTxRequest =
      await createTokenBridgePrepareSetWethGatewayTransactionRequest({
        rollup: rollupAddress,
        rollupDeploymentBlockNumber,
        parentChainPublicClient,
        orbitChainPublicClient,
        account: account.address,
        tokenBridgeCreatorAddressOverride,
        retryableGasOverrides: setWethGatewayGasOverrides
      })

    // sign and send the transaction
    const setWethGatewayTxHash =
      await parentChainPublicClient.sendRawTransaction({
        serializedTransaction: await account.signTransaction(
          setWethGatewayTxRequest
        )
      })

    const setWethGatewayTransaction =
      await parentChainPublicClient.getTransaction({
        hash: setWethGatewayTxHash
      })

    // get the transaction receipt after waiting for the transaction to complete
    const setWethGatewayTxReceipt =
      createTokenBridgePrepareSetWethGatewayTransactionReceipt(
        await parentChainPublicClient.waitForTransactionReceipt({
          hash: setWethGatewayTxHash
        })
      )

    print(
      `🔎  Weth gateway set in ${getChainExplorerUrl(parentChainPublicClient.chain)}/tx/${
        setWethGatewayTxReceipt.transactionHash
      }`
    )

    // Wait for retryables to execute
    const orbitChainSetWethGatewayRetryableReceipt =
      await setWethGatewayTxReceipt.waitForRetryables({
        // @ts-expect-error (todo: fix viem type issue)
        orbitPublicClient: orbitChainPublicClient
      })
    print(
      `🔎  Transaction hash for retryable is ${orbitChainSetWethGatewayRetryableReceipt[0].transactionHash}`
    )

    if (orbitChainSetWethGatewayRetryableReceipt[0].status !== "success") {
      throw new Error(
        `🚫  Retryable status is not success: ${orbitChainSetWethGatewayRetryableReceipt[0].status}. Aborting...`
      )
    }

    return {
      transaction,
      transactionReceipt: txReceipt,
      retryables: orbitChainRetryableReceipts,
      tokenBridgeContracts,
      setWethGateway: {
        transaction: setWethGatewayTransaction,
        transactionReceipt: setWethGatewayTxReceipt,
        retryables: [orbitChainSetWethGatewayRetryableReceipt[0]]
      }
    }
  }

  return {
    transaction,
    transactionReceipt: txReceipt,
    retryables: orbitChainRetryableReceipts,
    tokenBridgeContracts
  }
}
