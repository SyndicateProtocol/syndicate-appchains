import {
  allowlistSequencingModuleABI,
  allowlistSequencingModuleBytecode
} from "@/abi/synd/AllowlistSequencingModule"
import {
  requireAndModuleABI,
  requireAndModuleBytecode
} from "@/abi/synd/RequireAndModule"
import type {
  CreateRequireAndModule,
  CreateSyndicateSequencingChain,
  DeployAndSetupAllowlistSequencingModule,
  DeploySequencingChain,
  RegisterAllowlistSequencingModuleOnRequireAllModule,
  TransferPermissionModuleOwnership
} from "@/types"
import { supportedSequencingChains } from "@/utils/constants"
import { getChainExplorerUrl } from "@/utils/helpers"
import { print } from "@/utils/print"
import { zeroAddress } from "viem"

export async function deploySequencingChain({
  sequencerAccount,
  chainId,
  sequencingPublicClient,
  deployerSequencingWalletClient,
  ownerSequencingWalletClient,
  deployerEthereumWalletClient,
  ethereumPublicClient
}: DeploySequencingChain) {
  // 1. Create RequireAndModule
  const requireAndModule = await createRequireAndModule({
    sequencingPublicClient,
    deployerSequencingWalletClient
  })

  // 2. Deploy and setup AllowlistSequencingModule
  const allowlistSequencingModule =
    await deployAndSetupAllowlistSequencingModule({
      sequencerAccount,
      sequencingPublicClient,
      deployerSequencingWalletClient
    })

  // 3. Register AllowlistSequencingModule on RequireAllModule
  await registerAllowlistSequencingModuleOnRequireAllModule({
    requireAndModule,
    allowlistSequencingModule,
    deployerSequencingWalletClient,
    sequencingPublicClient
  })

  // 4. Transfer ownership permission modules
  await transferPermissonModuleOwnership({
    allowlistSequencingModule,
    requireAndModule,
    deployerSequencingWalletClient,
    sequencingPublicClient,
    ownerSequencingWalletClient
  })

  // 5. Create SyndicateSequencingChain
  const sequencingContract = await createSyndicateSequencingChain({
    requireAndModule,
    ethereumPublicClient,
    deployerEthereumWalletClient,
    chainId,
    sequencingChainId: sequencingPublicClient.chain.id
  })

  return {
    sequencingContract,
    allowlistSequencingModule,
    requireAndModule
  }
}

async function createRequireAndModule({
  sequencingPublicClient,
  deployerSequencingWalletClient
}: CreateRequireAndModule) {
  const hash = await deployerSequencingWalletClient.deployContract({
    abi: requireAndModuleABI,
    bytecode: requireAndModuleBytecode,
    args: [deployerSequencingWalletClient.account.address]
  })
  const requireAndModuleReceipt =
    await sequencingPublicClient.waitForTransactionReceipt({
      hash
    })
  const requireAndModuleAddress = requireAndModuleReceipt.contractAddress
  if (!requireAndModuleAddress) {
    throw new Error("RequireAndModule deployment failed")
  }
  print(
    `🔍  RequireAndModule deployed to ${requireAndModuleAddress}\n${getChainExplorerUrl(
      sequencingPublicClient.chain
    )}/tx/${hash}`
  )
  return requireAndModuleAddress
}

async function createSyndicateSequencingChain({
  chainId,
  requireAndModule,
  ethereumPublicClient,
  deployerEthereumWalletClient,
  sequencingChainId
}: CreateSyndicateSequencingChain) {
  // @note TODO: Deploy SyndicateSequencingChain
  const _implementationAddress =
    supportedSequencingChains[sequencingChainId].sequencingChainImplementation
  return zeroAddress
}

async function deployAndSetupAllowlistSequencingModule({
  sequencerAccount,
  sequencingPublicClient,
  deployerSequencingWalletClient
}: DeployAndSetupAllowlistSequencingModule) {
  const allowlistSequencingModuleHash =
    await deployerSequencingWalletClient.deployContract({
      abi: allowlistSequencingModuleABI,
      bytecode: allowlistSequencingModuleBytecode,
      account: deployerSequencingWalletClient.account,
      args: [deployerSequencingWalletClient.account.address]
    })
  const sequencingModuleTx =
    await sequencingPublicClient.waitForTransactionReceipt({
      hash: allowlistSequencingModuleHash
    })
  const allowlistSequencingModule = sequencingModuleTx.contractAddress
  if (!allowlistSequencingModule) {
    throw new Error("AllowlistSequencingModule deployment failed")
  }
  print(
    `🔍  AllowlistSequencingModule deployed to ${allowlistSequencingModule}\n${getChainExplorerUrl(
      sequencingPublicClient.chain
    )}/tx/${allowlistSequencingModuleHash}`
  )

  // 2: Allowlist wallet pool by calling 'addToAllowlist' on AllowlistSequencingModule
  const addAllowlistTxHash = await deployerSequencingWalletClient.writeContract(
    {
      address: allowlistSequencingModule,
      abi: allowlistSequencingModuleABI,
      functionName: "addToAllowlist",
      args: [sequencerAccount.address],
      account: deployerSequencingWalletClient.account
    }
  )
  const allowlistSequencerTx =
    await sequencingPublicClient.waitForTransactionReceipt({
      hash: addAllowlistTxHash
    })
  print(
    `🔍  Sequencer added to sequencer allowlist\n${getChainExplorerUrl(
      sequencingPublicClient.chain
    )}/tx/${allowlistSequencerTx.transactionHash}`
  )
  return allowlistSequencingModule
}

async function registerAllowlistSequencingModuleOnRequireAllModule({
  requireAndModule,
  allowlistSequencingModule,
  deployerSequencingWalletClient,
  sequencingPublicClient
}: RegisterAllowlistSequencingModuleOnRequireAllModule) {
  const registerSequencerAllowlistTxHash =
    await deployerSequencingWalletClient.writeContract({
      address: requireAndModule,
      abi: requireAndModuleABI,
      functionName: "addPermissionCheck",
      args: [allowlistSequencingModule, true],
      account: deployerSequencingWalletClient.account
    })
  const registerSequencerAllowlistTx =
    await sequencingPublicClient.waitForTransactionReceipt({
      hash: registerSequencerAllowlistTxHash
    })
  print(
    `🔍  AllowlistSequencingModule added to RequireAllModule\n${getChainExplorerUrl(
      sequencingPublicClient.chain
    )}/tx/${registerSequencerAllowlistTx.transactionHash}`
  )
}

async function transferPermissonModuleOwnership({
  allowlistSequencingModule,
  requireAndModule,
  deployerSequencingWalletClient,
  sequencingPublicClient,
  ownerSequencingWalletClient
}: TransferPermissionModuleOwnership) {
  // AllowlistSequencingModule
  const transferAllowlistSequencingModuleOwnershipTxHash =
    await deployerSequencingWalletClient.writeContract({
      address: allowlistSequencingModule,
      abi: allowlistSequencingModuleABI,
      functionName: "transferAdmin",
      args: [ownerSequencingWalletClient.account.address],
      account: deployerSequencingWalletClient.account
    })
  const transferAllowlistSequencingModuleOwnershipTx =
    await sequencingPublicClient.waitForTransactionReceipt({
      hash: transferAllowlistSequencingModuleOwnershipTxHash
    })
  print(
    `🔍  AllowlistSequencingModule ownership transferred to owner ${ownerSequencingWalletClient.account.address}\n${getChainExplorerUrl(
      sequencingPublicClient.chain
    )}/tx/${transferAllowlistSequencingModuleOwnershipTx.transactionHash}`
  )

  // RequireAllModule
  const transferRequireAllModuleOwnershipTxHash =
    await deployerSequencingWalletClient.writeContract({
      address: requireAndModule,
      abi: requireAndModuleABI,
      functionName: "transferOwnership",
      args: [ownerSequencingWalletClient.account.address],
      account: deployerSequencingWalletClient.account
    })
  const transferRequireAllModuleOwnershipTx =
    await sequencingPublicClient.waitForTransactionReceipt({
      hash: transferRequireAllModuleOwnershipTxHash
    })
  print(
    `🔍  RequireAllModule ownership transferred to owner ${ownerSequencingWalletClient.account.address}\n${getChainExplorerUrl(
      sequencingPublicClient.chain
    )}/tx/${transferRequireAllModuleOwnershipTx.transactionHash}`
  )
}
