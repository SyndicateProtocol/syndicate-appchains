import type { CreateRollupParams } from "@/types"
import { supportedSettlementChains } from "@/utils/constants"
import {
  getChainExplorerUrl,
  getNativeCurrency,
  isNativeTokenEth
} from "@/utils/helpers"
import { print } from "@/utils/print"
import {
  type CreateRollupParams as NitroCreateRollupParams,
  createRollupEnoughCustomFeeTokenAllowance,
  createRollupGetRetryablesFeesWithDefaults,
  createRollupPrepareCustomFeeTokenApprovalTransactionRequest,
  createRollupPrepareDeploymentParamsConfig,
  createRollupPrepareTransactionReceipt,
  prepareChainConfig
} from "@arbitrum/orbit-sdk"
import { sleep } from "bun"
import {
  type TransactionSerializable,
  erc20Abi,
  formatEther,
  parseUnits,
  zeroAddress
} from "viem"
import { createRollupPrepareTransactionRequest } from "./createRollupPrepareTransactionRequest"

export async function deployRollup({
  chainId,
  nativeToken,
  deployerSettlementWalletClient,
  ownerSettlementWalletClient,
  settlementPublicClient
}: CreateRollupParams) {
  const deployerAccount = deployerSettlementWalletClient.account
  const rollupOwnerAccount = ownerSettlementWalletClient.account
  const owner = rollupOwnerAccount.address
  const rollupCreatorAddress =
    supportedSettlementChains[settlementPublicClient.chain.id].rollupCreator

  const maxDataSize = 117964
  const chainConfig = prepareChainConfig({
    chainId,
    arbitrum: {
      InitialArbOSVersion: 40,
      DataAvailabilityCommittee: false,
      InitialChainOwner: owner,
      MaxCodeSize: 0,
      MaxInitCodeSize: 0
    }
  })

  // syndicate appchains do not utilize batch posters or validators so we exclude them here
  const params: Omit<
    NitroCreateRollupParams,
    "batchPosters" | "validators" | "batchPosterManager"
  > = {
    config: createRollupPrepareDeploymentParamsConfig(settlementPublicClient, {
      chainId: BigInt(chainId),
      owner,
      chainConfig
    }),
    maxDataSize,
    batchPosterManager: zeroAddress,
    batchPosters: [],
    validators: []
  }

  const costOfRetryables = await createRollupGetRetryablesFeesWithDefaults(
    settlementPublicClient,
    {
      account: deployerAccount.address,
      nativeToken
    }
  )

  if (isNativeTokenEth(nativeToken)) {
    // Owner must have a balance of native token to pay for retryables
    const balance = await settlementPublicClient.getBalance({
      address: deployerAccount.address
    })
    if (balance < costOfRetryables) {
      throw new Error("Insufficient balance for deployer to deploy rollup")
    }
  } else {
    // Deployer must have a balance of the native token to deploy the rollup
    const deployerBalance = await settlementPublicClient.readContract({
      address: nativeToken,
      abi: erc20Abi,
      functionName: "balanceOf",
      args: [deployerAccount.address]
    })

    const tokenDecimals = settlementPublicClient.chain.nativeCurrency.decimals
    const convertedBalance =
      parseUnits(deployerBalance.toString(), tokenDecimals) *
      BigInt(10 ** (18 - tokenDecimals))
    if (convertedBalance < costOfRetryables) {
      const nativeCurrency = nativeToken
        ? await getNativeCurrency(settlementPublicClient, nativeToken)
        : undefined
      throw new Error(
        `Insufficient balance for deployer account ${
          deployerAccount.address
        } to deploy rollup. Deployer balance: ${formatEther(
          convertedBalance
        )}. Please fund the deployer with at least ${formatEther(
          costOfRetryables
        )} ${nativeCurrency?.symbol}`
      )
    }

    const allowanceParams = {
      nativeToken: nativeToken,
      account: deployerAccount.address,
      publicClient: settlementPublicClient,
      rollupCreatorAddressOverride: rollupCreatorAddress
    }

    print("🔍  Deploying Arbtirum Nitro rollup...")
    if (!(await createRollupEnoughCustomFeeTokenAllowance(allowanceParams))) {
      print("🔍  Approving native gas token...")
      const approvalTxRequest =
        await createRollupPrepareCustomFeeTokenApprovalTransactionRequest(
          allowanceParams
        )

      // sign and send the transaction
      const approvalTxHash = await settlementPublicClient.sendRawTransaction({
        serializedTransaction:
          await deployerAccount.signTransaction(approvalTxRequest)
      })

      // get the transaction receipt after waiting for the transaction to complete
      const approvalTxReceipt = createRollupPrepareTransactionReceipt(
        await settlementPublicClient.waitForTransactionReceipt({
          hash: approvalTxHash
        })
      )

      print(
        `🔍  Native gas token approved in ${getChainExplorerUrl(
          settlementPublicClient.chain
        )}/tx/${approvalTxReceipt.transactionHash}`
      )
    }

    params.nativeToken = nativeToken
    print("🫷  Waiting for 5 seconds before calling createRollup...")
    await sleep(5_000)
  }

  const request = await createRollupPrepareTransactionRequest({
    params,
    account: deployerAccount.address,
    publicClient: settlementPublicClient,
    rollupCreatorAddress
  })

  const txHash = await settlementPublicClient.sendRawTransaction({
    serializedTransaction: await deployerAccount.signTransaction(
      request as TransactionSerializable
    )
  })

  const { transactionHash } = createRollupPrepareTransactionReceipt(
    await settlementPublicClient.waitForTransactionReceipt({ hash: txHash })
  )
  print(
    `🔍  Rollup deployed in ${getChainExplorerUrl(
      settlementPublicClient.chain
    )}/tx/${transactionHash}`
  )
  return transactionHash
}
