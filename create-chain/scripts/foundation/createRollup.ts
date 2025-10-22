import {
  type CreateRollupParams,
  createRollupEnoughCustomFeeTokenAllowance,
  createRollupGetRetryablesFeesWithDefaults,
  createRollupPrepareCustomFeeTokenApprovalTransactionRequest,
  createRollupPrepareDeploymentParamsConfig,
  createRollupPrepareTransactionReceipt,
  createRollupPrepareTransactionRequest,
  prepareChainConfig
} from "@arbitrum/orbit-sdk"

import {
  getChainExplorerUrl,
  getNativeCurrency,
  isNativeTokenEth
} from "@/scripts/utils/helpers"
import { getFoundationConfig } from "../utils/config"

import {
  ownerAdjustableExchangeRatePricerAbi,
  ownerAdjustableExchangeRatePricerBytecode
} from "@/scripts/abi/nitro/OwnerAdjustableExchangeRatePricer"
import type { CreateSettlementRollupParams } from "@/scripts/types"
import { supportedSettlementChains } from "@/scripts/utils/constants"
import { sleep } from "bun"
import { erc20Abi, formatEther, parseEther, parseUnits } from "viem"
import { print } from "../utils/print"

export async function createRollup({
  validators,
  batchPosters,
  batchPosterManager
}: CreateSettlementRollupParams) {
  const {
    chainId,
    nativeTokenAddress,
    deployerSettlementWalletClient,
    ownerSettlementWalletClient,
    settlementPublicClient
  } = await getFoundationConfig()
  const deployerAccount = deployerSettlementWalletClient.account
  const rollupOwnerAccount = ownerSettlementWalletClient.account
  const owner = rollupOwnerAccount.address
  const rollupCreatorAddress =
    supportedSettlementChains[settlementPublicClient.chain.id]
      .rollupCreatorAddress

  const maxDataSize = 117964
  const chainConfig = prepareChainConfig({
    chainId,
    arbitrum: {
      InitialArbOSVersion: 32,
      DataAvailabilityCommittee: false,
      InitialChainOwner: owner,
      MaxCodeSize: 0,
      MaxInitCodeSize: 0
    }
  })

  const params: CreateRollupParams = {
    config: createRollupPrepareDeploymentParamsConfig(settlementPublicClient, {
      chainId: BigInt(chainId),
      owner,
      chainConfig
    }),
    batchPosterManager,
    batchPosters,
    validators,
    maxDataSize
  }

  const costOfRetryables = await createRollupGetRetryablesFeesWithDefaults(
    settlementPublicClient,
    {
      account: deployerAccount.address,
      nativeToken: nativeTokenAddress
    }
  )

  if (isNativeTokenEth(nativeTokenAddress)) {
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
      address: nativeTokenAddress,
      abi: erc20Abi,
      functionName: "balanceOf",
      args: [deployerAccount.address]
    })

    const tokenDecimals = settlementPublicClient.chain.nativeCurrency.decimals
    const convertedBalance =
      parseUnits(deployerBalance.toString(), tokenDecimals) *
      BigInt(10 ** (18 - tokenDecimals))
    if (convertedBalance < costOfRetryables) {
      const nativeCurrency = nativeTokenAddress
        ? await getNativeCurrency(settlementPublicClient, nativeTokenAddress)
        : undefined
      throw new Error(
        `Insufficient balance for deployer account ${deployerAccount.address} to deploy rollup. Deployer balance: ${formatEther(convertedBalance)}. Please fund the deployer with at least ${formatEther(costOfRetryables)} ${nativeCurrency?.symbol}`
      )
    }

    const allowanceParams = {
      nativeToken: nativeTokenAddress,
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
        `🔍  Native gas token approved in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${
          approvalTxReceipt.transactionHash
        }`
      )
    }

    print("🔍  Deploying exchange rate pricer...")
    const createExchangeRatePricerHash =
      await deployerSettlementWalletClient.deployContract({
        abi: ownerAdjustableExchangeRatePricerAbi,
        bytecode: ownerAdjustableExchangeRatePricerBytecode,
        account: deployerAccount,
        // Exchange rate set to 1:1
        args: [parseEther("1")]
      })
    const exchangeRatePricerTx =
      await settlementPublicClient.waitForTransactionReceipt({
        hash: createExchangeRatePricerHash
      })
    const exchangeRatePricerAddress = exchangeRatePricerTx.contractAddress
    if (!exchangeRatePricerAddress) {
      throw new Error("❌ Exchange rate pricer deployment failed")
    }
    print(
      `🔍  Exchange rate pricer deployed to ${exchangeRatePricerAddress} in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${
        createExchangeRatePricerHash
      }`
    )

    // Sleep before calling the contract
    await sleep(2_000)

    // Transfer ownership of the exchange rate pricer to the owner
    const transferOwnershipHash =
      await deployerSettlementWalletClient.writeContract({
        address: exchangeRatePricerAddress,
        abi: ownerAdjustableExchangeRatePricerAbi,
        functionName: "transferOwnership",
        args: [owner]
      })

    const transferOwnershipTxReceipt =
      await settlementPublicClient.waitForTransactionReceipt({
        hash: transferOwnershipHash
      })
    print(
      `🔍  Exchange rate pricer ownership transferred to ${owner} in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${
        transferOwnershipTxReceipt.transactionHash
      }`
    )

    params.feeTokenPricer = exchangeRatePricerAddress
    params.nativeToken = nativeTokenAddress
    print("🫷  Waiting for 5 seconds before calling createRollup...")
    await sleep(5_000)
  }

  const request = await createRollupPrepareTransactionRequest({
    params,
    account: deployerAccount.address,
    publicClient: settlementPublicClient,
    rollupCreatorAddressOverride: rollupCreatorAddress
  })

  const txHash = await settlementPublicClient.sendRawTransaction({
    serializedTransaction: await deployerAccount.signTransaction(request)
  })

  const { transactionHash } = createRollupPrepareTransactionReceipt(
    await settlementPublicClient.waitForTransactionReceipt({ hash: txHash })
  )
  print(
    `🔍  Rollup deployed in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${transactionHash}`
  )
  return transactionHash
}
