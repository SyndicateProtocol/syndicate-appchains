import { allowlistSequencingModuleABI } from "@/src/abi/synd/AllowlistSequencingModule"
import { arbChainConfigABI } from "@/src/abi/synd/ArbChainConfig"
import { requireAndModuleABI } from "@/src/abi/synd/RequireAndModule"
import { syndicateSequencingChainABI } from "@/src/abi/synd/SyndicateSequencingChain"
import { teeModuleABI } from "@/src/abi/synd/TeeModule"
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
      abi: arbChainConfigABI,
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
  // Grant the DEFAULT_ADMIN_ROLE to the new owner
  const teeModuleDefaultAdminRole = await settlementPublicClient.readContract({
    address: synd.withdrawals.teeModule,
    abi: teeModuleABI,
    functionName: "DEFAULT_ADMIN_ROLE"
  })
  const teeModuleDefaultAdminRoleTransferHash = await ownerSettlementWalletClient.writeContract({
    address: synd.withdrawals.teeModule,
    abi: teeModuleABI,
    functionName: "grantRole",
    args: [teeModuleDefaultAdminRole, newOwnerAddress]
  })
  await settlementPublicClient.waitForTransactionReceipt({
    hash: teeModuleDefaultAdminRoleTransferHash
  })
  print(
    `🔍  Syndicate internal owner set the DEFAULT_ADMIN_ROLE of the TeeModule to ${newOwnerAddress} in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${teeModuleDefaultAdminRoleTransferHash}`
  )

  // Revoke the DEFAULT_ADMIN_ROLE from the owner
  const teeModuleDefaultAdminRoleRevokeHash = await ownerSettlementWalletClient.writeContract({
    address: synd.withdrawals.teeModule,
    abi: teeModuleABI,
    functionName: "revokeRole",
    args: [teeModuleDefaultAdminRole, ownerSettlementWalletClient.account.address]
  })
  await settlementPublicClient.waitForTransactionReceipt({
    hash: teeModuleDefaultAdminRoleRevokeHash
  })
  print(
    `🔍  Syndicate internal owner revoked the DEFAULT_ADMIN_ROLE of the TeeModule from the owner in ${getChainExplorerUrl(settlementPublicClient.chain)}/tx/${teeModuleDefaultAdminRoleRevokeHash}`
  )

  // 1. Transfer ownership of the SyndicateSequencingChain
  const transferSyndicateSequencingChainTx =
    await ownerSequencingWalletClient.writeContract({
      address: synd.sequencing.syndicateSequencingChain,
      abi: syndicateSequencingChainABI,
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
      abi: allowlistSequencingModuleABI,
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
      abi: requireAndModuleABI,
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
