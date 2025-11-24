import { createTokenBridgeFetchTokenBridgeContracts } from "@arbitrum/orbit-sdk"
import { tokenBridgeCreatorABI } from "@arbitrum/orbit-sdk/contracts/TokenBridgeCreator/v1.2.js"
import {
  type Chain,
  type Hex,
  type Log,
  type PublicClient,
  type TransactionReceipt,
  type Transport,
  decodeEventLog,
  getAbiItem,
  getEventSelector
} from "viem"

export async function getTokenBridgeContracts({
  bridgeCreationHash,
  parentChainPublicClient,
  tokenBridgeCreatorAddressOverride
}: {
  bridgeCreationHash: Hex
  parentChainPublicClient: PublicClient<Transport, Chain>
  tokenBridgeCreatorAddressOverride: Hex
}) {
  const txReceipt = await parentChainPublicClient.waitForTransactionReceipt({
    hash: bridgeCreationHash
  })
  const eventLog = findOrbitTokenBridgeCreatedEventLog(txReceipt)
  const decodedEventLog = decodeOrbitTokenBridgeCreatedEventLog(eventLog)
  const { inbox } = decodedEventLog.args
  const contracts = await createTokenBridgeFetchTokenBridgeContracts({
    inbox,
    parentChainPublicClient,
    tokenBridgeCreatorAddressOverride
  })

  return contracts
}

function findOrbitTokenBridgeCreatedEventLog(txReceipt: TransactionReceipt) {
  const abiItem = getAbiItem({
    abi: tokenBridgeCreatorABI,
    name: "OrbitTokenBridgeCreated"
  })
  const eventSelector = getEventSelector(abiItem)
  const log = txReceipt.logs.find((log) => log.topics[0] === eventSelector)

  if (typeof log === "undefined") {
    throw new Error(
      `No "OrbitTokenBridgeCreated" logs found in logs for transaction: ${txReceipt.transactionHash}`
    )
  }

  return log
}

function decodeOrbitTokenBridgeCreatedEventLog(log: Log<bigint, number>) {
  const decodedEventLog = decodeEventLog({
    ...log,
    abi: tokenBridgeCreatorABI
  })

  if (decodedEventLog.eventName !== "OrbitTokenBridgeCreated") {
    throw new Error(
      `Expected "OrbitTokenBridgeCreated" event but found: ${decodedEventLog.eventName}`
    )
  }

  return decodedEventLog
}
