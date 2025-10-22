import { AllowlistSequencingModuleABI } from "@/scripts/abi/synd/AllowlistSequencingModule"
import { ArbChainConfigABI } from "@/scripts/abi/synd/ArbChainConfig"
import { RequireAndModuleABI } from "@/scripts/abi/synd/RequireAndModule"
import { SyndicateSequencingChainABI } from "@/scripts/abi/synd/SyndicateSequencingChain"
import { TeeModuleABI } from "@/scripts/abi/synd/TeeModule"
import { getHandoffConfig } from "../utils/config"
import { getChainExplorerUrl } from "../utils/helpers"
import { print } from "../utils/print"

export default async function handoffSynd() {
  const {
    ownerSettlementWalletClient,
    settlementPublicClient,
    synd,
    newOwnerAddress,
    ownerSequencingWalletClient,
    sequencingPublicClient
  } = await getHandoffConfig()

  // 1. Transfer ownership of the ArbChainConfig
  const transferArbChainConfigTx =
    await ownerSettlementWalletClient.writeContract({
      address: synd.config.arbChainConfig,
      abi: ArbChainConfigABI,
      functionName: "transferOwnership",
      args: [newOwnerAddress]
    })
  await settlementPublicClient.waitForTransactionReceipt({
    hash: transferArbChainConfigTx
  })
  print(
    `🔍  Syndicate internal owner transferred ownership of the ArbChainConfig to ${newOwnerAddress} in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${transferArbChainConfigTx}`
  )

  // 2. Transfer ownership of the TeeModule
  const transferTeeModuleTx = await ownerSettlementWalletClient.writeContract({
    address: synd.withdrawals.teeModule,
    abi: TeeModuleABI,
    functionName: "transferOwnership",
    args: [newOwnerAddress]
  })
  await settlementPublicClient.waitForTransactionReceipt({
    hash: transferTeeModuleTx
  })
  print(
    `🔍  Syndicate internal owner transferred ownership of the TeeModule to ${newOwnerAddress} in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${transferTeeModuleTx}`
  )

  // 1. Transfer ownership of the SyndicateSequencingChain
  const transferSyndicateSequencingChainTx =
    await ownerSequencingWalletClient.writeContract({
      address: synd.sequencing.syndicateSequencingChain,
      abi: SyndicateSequencingChainABI,
      functionName: "transferOwnership",
      args: [newOwnerAddress]
    })
  await sequencingPublicClient.waitForTransactionReceipt({
    hash: transferSyndicateSequencingChainTx
  })
  print(
    `🔍  Syndicate internal owner transferred ownership of the SyndicateSequencingChain to ${newOwnerAddress} in ${getChainExplorerUrl(sequencingPublicClient.chain)}/tx/${transferSyndicateSequencingChainTx}`
  )

  // 2. Transfer ownership of the AllowlistSequencingModule
  const transferAllowlistSequencingModuleTx =
    await ownerSequencingWalletClient.writeContract({
      address: synd.sequencing.allowlistSequencingModule,
      abi: AllowlistSequencingModuleABI,
      functionName: "transferAdmin",
      args: [newOwnerAddress]
    })
  await sequencingPublicClient.waitForTransactionReceipt({
    hash: transferAllowlistSequencingModuleTx
  })
  print(
    `🔍  Syndicate internal owner transferred ownership of the AllowlistSequencingModule to ${newOwnerAddress} in ${getChainExplorerUrl(sequencingPublicClient.chain)}/tx/${transferAllowlistSequencingModuleTx}`
  )

  // 3. Transfer ownership of the RequireAndModule
  const transferRequireAndModuleTx =
    await ownerSequencingWalletClient.writeContract({
      address: synd.sequencing.requireAndModule,
      abi: RequireAndModuleABI,
      functionName: "transferOwnership",
      args: [newOwnerAddress]
    })
  await sequencingPublicClient.waitForTransactionReceipt({
    hash: transferRequireAndModuleTx
  })
  print(
    `🔍  Syndicate internal owner transferred ownership of the RequireAndModule to ${newOwnerAddress} in ${getChainExplorerUrl(sequencingPublicClient.chain)}/tx/${transferRequireAndModuleTx}`
  )
}
