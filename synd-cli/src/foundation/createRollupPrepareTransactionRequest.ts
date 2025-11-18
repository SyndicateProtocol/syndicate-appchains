import {
  type CreateRollupFunctionInputs,
  type CreateRollupGetRetryablesFeesParams,
  type CreateRollupParams,
  fetchDecimals
} from "@arbitrum/orbit-sdk"
import {
  type Address,
  type CallParameters,
  type Chain,
  type EstimateGasParameters,
  type Hex,
  type PublicClient,
  type Transport,
  decodeFunctionResult,
  encodeFunctionData,
  parseEther,
  parseGwei,
  zeroAddress
} from "viem"
import { bridgeCreatorAbi } from "../abi/nitro/BridgeCreator"
import { deployHelperAbi } from "../abi/nitro/DeployHelper"
import { rollupCreatorAbi } from "../abi/nitro/RollupCreator"
import type { PublicClientWithChain } from "../types"
import { isNonZeroAddress, scaleByPercentage } from "../utils/helpers"

interface CreateRollupTxParams {
  params: Omit<
    CreateRollupParams<"v3.1">,
    "batchPosterManager" | "batchPosters" | "validators"
  >
  account: Address
  rollupCreatorAddress: Hex
  publicClient: PublicClientWithChain
  gasOverrides?: {
    gasLimit?: {
      base?: bigint
      percentIncrease: bigint
    }
  }
}

const createRollupDefaultRetryablesFees = parseEther("0.125")
const createRollupDefaults = {
  nativeToken: zeroAddress,
  deployFactoriesToL2: true,
  maxFeePerGasForRetryables: parseGwei(String("0.1")),
  batchPosterManager: zeroAddress,
  feeTokenPricer: zeroAddress
}

export async function createRollupPrepareTransactionRequest({
  params,
  account,
  publicClient,
  rollupCreatorAddress,
  gasOverrides
}: CreateRollupTxParams) {
  if (isNonZeroAddress(params.nativeToken)) {
    if (
      (await fetchDecimals({ address: params.nativeToken, publicClient })) > 36
    ) {
      throw new Error(
        `"params.nativeToken" can only be configured with a token that uses 36 decimals or less.`
      )
    }
  }

  const paramsWithDefaults: CreateRollupFunctionInputs<"v3.1"> = {
    ...createRollupDefaults,
    ...params
  }

  const value = await createRollupGetCallValue(
    publicClient,
    {
      ...paramsWithDefaults,
      account
    },
    rollupCreatorAddress
  )

  const data = encodeFunctionData({
    abi: rollupCreatorAbi,
    functionName: "createRollup",
    args: paramsWithDefaults
  })

  const request = await publicClient.prepareTransactionRequest({
    chain: publicClient.chain,
    to: rollupCreatorAddress,
    data,
    value,
    account,
    // if the base gas limit override was provided, hardcode gas to 0 to skip estimation
    // we'll set the actual value in the code below
    gas:
      typeof gasOverrides?.gasLimit?.base !== "undefined"
        ? BigInt(0)
        : undefined
  })

  // potential gas overrides (gas limit)
  if (gasOverrides?.gasLimit) {
    request.gas = scaleByPercentage(
      gasOverrides.gasLimit.base ?? request.gas,
      gasOverrides.gasLimit.percentIncrease
    )
  }

  return { ...request, chainId: publicClient.chain.id }
}

async function createRollupGetCallValue(
  publicClient: PublicClientWithChain,
  params: {
    account: Address
    nativeToken: Address
    deployFactoriesToL2: boolean
  },
  rollupCreatorAddress: Hex
): Promise<bigint> {
  // when not deploying deterministic factories to L2, no callvalue is necessary, as no retryable tickets will be created
  if (!params.deployFactoriesToL2) {
    return BigInt(0)
  }

  // when using a custom fee token, the retryable tickets will be paid for in the custom fee token, so no callvalue is necessary
  if (isNonZeroAddress(params.nativeToken)) {
    return BigInt(0)
  }

  return createRollupGetRetryablesFeesWithDefaults(
    publicClient,
    params,
    rollupCreatorAddress
  )
}

async function createRollupGetRetryablesFeesWithDefaults(
  publicClient: PublicClientWithChain,
  {
    account,
    nativeToken,
    maxFeePerGasForRetryables
  }: CreateRollupGetRetryablesFeesParams,
  rollupCreatorAddress: Hex
): Promise<bigint> {
  try {
    return await createRollupGetRetryablesFees(
      publicClient,
      {
        account,
        nativeToken,
        maxFeePerGasForRetryables
      },
      rollupCreatorAddress
    )
  } catch (error) {
    console.error(
      `[createRollupGetRetryablesFeesWithDefaults] Failed to fetch retryables fees, falling back to defaults.\n\n${error}`
    )
    return createRollupDefaultRetryablesFees
  }
}

async function createRollupGetRetryablesFees<TChain extends Chain | undefined>(
  publicClient: PublicClient<Transport, TChain>,
  {
    account,
    nativeToken,
    maxFeePerGasForRetryables
  }: CreateRollupGetRetryablesFeesParams,
  rollupCreatorAddress: Hex
): Promise<bigint> {
  const deployHelperAddress = await publicClient.readContract({
    abi: rollupCreatorAbi,
    address: rollupCreatorAddress,
    functionName: "l2FactoriesDeployer"
  })

  const [ethTemplateInbox, erc20TemplateInbox] = await getTemplates(
    publicClient,
    rollupCreatorAddress
  )

  const isCustomGasToken = isNonZeroAddress(nativeToken)
  const inbox = isCustomGasToken ? erc20TemplateInbox : ethTemplateInbox
  const maxFeePerGas =
    maxFeePerGasForRetryables ?? createRollupDefaults.maxFeePerGasForRetryables

  // add 30% buffer in case of a spike
  const baseFeeWithBuffer = scaleByPercentage(
    await publicClient.getGasPrice(),
    30
  )

  const callParams: CallParameters = {
    account,
    data: encodeFunctionData({
      abi: deployHelperAbi,
      functionName: "getDeploymentTotalCost",
      args: [inbox, maxFeePerGas]
    }),
    to: deployHelperAddress,
    maxFeePerGas: baseFeeWithBuffer
  }

  // calculate the gas necessary for the call, otherwise it's inflated and the call will fail
  // https://github.com/wevm/viem/discussions/862#discussioncomment-6398745
  const gasWithBuffer = scaleByPercentage(
    await publicClient.estimateGas(
      callParams as unknown as EstimateGasParameters<TChain>
    ),
    30
  )

  const { data: result } = await publicClient.call({
    ...callParams,
    gas: gasWithBuffer
  })

  if (!result) {
    throw new Error("Failed to get deployment total cost")
  }

  const decodedResult = decodeFunctionResult({
    abi: deployHelperAbi,
    functionName: "getDeploymentTotalCost",
    data: result
  })

  return isCustomGasToken
    ? // for custom gas token chains, retryable fees don't scale with parent base fee and are constant at 124708400000000000
      //
      // we add some buffer (around 100k gwei) due to potential rounding issues for non-18 decimals, because:
      // - in the sdk, we get the total cost, then scale and round up
      // - in the contract, we scale and round up each component, then add them together, which can lead to a very tiny discrepancy
      //
      // https://github.com/OffchainLabs/nitro-contracts/blob/main/src/rollup/RollupCreator.sol#L287-L302
      parseEther("0.1248")
    : // for eth chains, add 3% buffer
      scaleByPercentage(decodedResult, 3)
}

async function getTemplates<TChain extends Chain | undefined>(
  publicClient: PublicClient<Transport, TChain>,
  rollupCreatorAddress: Hex
) {
  const bridgeCreatorAddress = await publicClient.readContract({
    abi: rollupCreatorAbi,
    address: rollupCreatorAddress,
    functionName: "bridgeCreator"
  })

  // v3.1 - bridge, sequencerInbox, delayBufferableSequencerInbox, inbox, rollupEventInbox, outbox
  // inbox at index 3
  // https://github.com/OffchainLabs/nitro-contracts/blob/11a59629c472e16644c9f536cdb8a91b00685f32/src/rollup/BridgeCreator.sol#L32-L33
  const [ethBasedTemplates, erc20BasedTemplates] = await Promise.all([
    publicClient.readContract({
      abi: bridgeCreatorAbi,
      address: bridgeCreatorAddress,
      functionName: "ethBasedTemplates"
    }),
    publicClient.readContract({
      abi: bridgeCreatorAbi,
      address: bridgeCreatorAddress,
      functionName: "erc20BasedTemplates"
    })
  ])

  return [ethBasedTemplates[3], erc20BasedTemplates[3]] as const
}
