import { SequencerInboxABI } from "@/scripts/abi/nitro/SequencerInbox"
import { UpgradeExecutorABI } from "@/scripts/abi/nitro/UpgradeExecutor"
import { encodeFunctionData, zeroAddress } from "viem"
import { getHandoffConfig } from "../utils/config"
import { getChainExplorerUrl, isAddressEq } from "../utils/helpers"
import { print } from "../utils/print"

export async function checkSequencerInboxConfig() {
  const { ownerSettlementWalletClient, settlementPublicClient, synd } =
    await getHandoffConfig()

  const batchPosterManager = await settlementPublicClient.readContract({
    address: synd.bridge.coreContracts.sequencerInbox,
    abi: SequencerInboxABI,
    functionName: "batchPosterManager"
  })

  const feeTokenPricer = await settlementPublicClient.readContract({
    address: synd.bridge.coreContracts.sequencerInbox,
    abi: SequencerInboxABI,
    functionName: "feeTokenPricer"
  })

  const isOwnerBatchPoster = await settlementPublicClient.readContract({
    address: synd.bridge.coreContracts.sequencerInbox,
    abi: SequencerInboxABI,
    functionName: "isBatchPoster",
    args: [ownerSettlementWalletClient.account.address]
  })

  const isBatchPosterManagerZero = isAddressEq(batchPosterManager, zeroAddress)

  const isFeeTokenPricerZeroAddress = isAddressEq(feeTokenPricer, zeroAddress)

  // If old owner is a batch poster, remove it
  if (isOwnerBatchPoster) {
    // Owner should be batch poster manager so we can call directly
    const removeBatchPosterTx = await ownerSettlementWalletClient.writeContract(
      {
        address: synd.bridge.coreContracts.sequencerInbox,
        abi: SequencerInboxABI,
        functionName: "setIsBatchPoster",
        args: [ownerSettlementWalletClient.account.address, false]
      }
    )
    await settlementPublicClient.waitForTransactionReceipt({
      hash: removeBatchPosterTx
    })
    print(
      `🔍  Removed old owner as batch poster in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${removeBatchPosterTx}`
    )
  }

  if (!isBatchPosterManagerZero) {
    // Set batch poster manager address to the zero address
    const setBatchPosterManagerCalldata = encodeFunctionData({
      abi: SequencerInboxABI,
      functionName: "setBatchPosterManager",
      args: [zeroAddress]
    })
    const removeBatchPosterManagerTx =
      await ownerSettlementWalletClient.writeContract({
        address: synd.bridge.coreContracts.upgradeExecutor,
        abi: UpgradeExecutorABI,
        functionName: "executeCall",
        args: [
          synd.bridge.coreContracts.sequencerInbox, // target
          setBatchPosterManagerCalldata // targetCallData
        ]
      })
    await settlementPublicClient.waitForTransactionReceipt({
      hash: removeBatchPosterManagerTx
    })
    print(
      `🔍  Batch poster manager set to zero address in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${removeBatchPosterManagerTx}`
    )
  }

  if (!isFeeTokenPricerZeroAddress) {
    // Set fee token pricer address to the zero address
    const setFeeTokenPricerCalldata = encodeFunctionData({
      abi: SequencerInboxABI,
      functionName: "setFeeTokenPricer",
      args: [zeroAddress]
    })
    const removeFeeTokenPricerTx =
      await ownerSettlementWalletClient.writeContract({
        address: synd.bridge.coreContracts.upgradeExecutor,
        abi: UpgradeExecutorABI,
        functionName: "executeCall",
        args: [
          synd.bridge.coreContracts.sequencerInbox, // target
          setFeeTokenPricerCalldata // targetCallData
        ]
      })
    await settlementPublicClient.waitForTransactionReceipt({
      hash: removeFeeTokenPricerTx
    })
    print(
      `🔍  Fee token pricer set to zero address in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${removeFeeTokenPricerTx}`
    )
  }
}
