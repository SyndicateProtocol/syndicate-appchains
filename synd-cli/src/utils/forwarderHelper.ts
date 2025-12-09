import { ERC20InboxABI } from "@/abi/nitro/ERC20Inbox"
import { syndForwarderABI } from "@/abi/synd/SyndForwarder"
import {
  type Address,
  type Hex,
  encodeFunctionData,
  getContractAddress,
  pad,
  toBytes
} from "viem"
import { supportedSequencingChains } from "./constants"

const OPTIMISM_PORTAL_ABI = [
  {
    name: "depositTransaction",
    type: "function",
    stateMutability: "nonpayable",
    inputs: [
      { name: "_to", type: "address" },
      { name: "_value", type: "uint256" },
      { name: "_gasLimit", type: "uint64" },
      { name: "_isCreation", type: "bool" },
      { name: "_data", type: "bytes" }
    ],
    outputs: []
  }
] as const

/**
 * Encodes a call to SyndForwarder.deploy
 * @param salt - The deployment salt
 * @param impl - The implementation address
 * @param init - The initialization data
 * @returns Encoded function data
 */
export function wrapDeploy(salt: Hex, impl: Address, init: Hex): Hex {
  return encodeFunctionData({
    abi: syndForwarderABI,
    functionName: "deploy",
    args: [salt, impl, init]
  })
}

/**
 * Encodes a call to DeployerParent.call
 * @param to - The target address
 * @param data - The call data
 * @returns Encoded function data
 */
export function wrapCall(to: Address, data: Hex): Hex {
  return encodeFunctionData({
    abi: syndForwarderABI,
    functionName: "call",
    args: [to, data]
  })
}

/**
 * Encodes a call to IOptimismPortal.depositTransaction
 * @param forwarder - The forwarder address
 * @param gasLimit - The gas limit for the L2 transaction
 * @param data - The transaction data
 * @returns Encoded function data
 */
export function wrapOP(forwarder: Address, gasLimit: bigint, data: Hex): Hex {
  return encodeFunctionData({
    abi: OPTIMISM_PORTAL_ABI,
    functionName: "depositTransaction",
    args: [
      forwarder, // _to
      BigInt(0), // _value
      gasLimit, // _gasLimit
      false, // _isCreation
      data // _data
    ]
  })
}

/**
 * Encodes a call to IArbBridge.unsafeCreateRetryableTicket
 * @param forwarder - The forwarder address
 * @param gasLimitArb - The gas limit for the Arbitrum retryable ticket
 * @param maxFeePerGas - The max fee per gas
 * @param data - The transaction data
 * @returns Encoded function data
 */
export function wrapArb(
  forwarder: Address,
  data: Hex,
  gasLimitArb: bigint = BigInt(210000),
  maxFeePerGas: bigint = BigInt(1000000000) // 1 gwei
): Hex {
  const amount = gasLimitArb * maxFeePerGas
  return encodeFunctionData({
    abi: ERC20InboxABI,
    functionName: "unsafeCreateRetryableTicket",
    args: [
      forwarder, // to
      BigInt(0), // l2CallValue
      BigInt(0), // maxSubmissionCost
      forwarder, // excessFeeRefundAddress
      forwarder, // callValueRefundAddress
      gasLimitArb, // gasLimit
      maxFeePerGas, // maxFeePerGas
      amount, // tokenTotalFeeAmount
      data // data
    ]
  })
}

export function getSequencingChainAddress(
  chainId: number,
  seqChainId: number
): Address {
  return getContractAddress({
    bytecodeHash:
      "0xe64a956779ab4f25594d056c498bb94989fa8edbf4b4124362dda18e5c29746e", // << keccak of MinimalUUPSStub bytecode
    from: supportedSequencingChains[seqChainId].forwarderAddress,
    opcode: "CREATE2",
    salt: pad(toBytes(chainId))
  })
}
