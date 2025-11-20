import { parseEventLogs, toBytes, toHex } from "viem"

import {
  allowlistSequencingModuleABI,
  allowlistSequencingModuleBytecode
} from "@/abi/synd/AllowlistSequencingModule"
import { requireAndModuleABI } from "@/abi/synd/RequireAndModule"
import { requireAndModuleFactoryABI } from "@/abi/synd/RequireAndModuleFactory"
import { syndicateFactoryABI } from "@/abi/synd/SyndicateFactory"
import { syndicateSequencingChainABI } from "@/abi/synd/SyndicateSequencingChain"
import type {
  CreateRequireAndModuleParams,
  CreateSyndicateSequencingChainParams,
  DeployAndSetupAllowlistSequencingModuleParams,
  DeploySequencingChainParams,
  RegisterAllowlistSequencingModuleOnRequireAllModuleParams,
  TransferAllContractsOwnershipParams
} from "@/types"
import { supportedSequencingChains } from "@/utils/constants"
import { getChainExplorerUrl } from "@/utils/helpers"
import { print } from "@/utils/print"

export async function deploySequencingChain({
  sequencerAccount,
  chainId,
  sequencingPublicClient,
  deployerSequencingWalletClient,
  ownerSequencingWalletClient
}: DeploySequencingChainParams) {
  // 1. Create RequireAndModule
  const requireAndModule = await createRequireAndModule({
    chainId,
    sequencingPublicClient,
    deployerSequencingWalletClient
  })

  // 2. Create SyndicateSequencingChain
  const { sequencingContract, deployedAtBlock } =
    await createSyndicateSequencingChain({
      requireAndModule,
      sequencingPublicClient,
      deployerSequencingWalletClient,
      chainId
    })

  // 3. Deploy and setup AllowlistSequencingModule
  const allowlistSequencingModule =
    await deployAndSetupAllowlistSequencingModule({
      sequencerAccount,
      sequencingPublicClient,
      deployerSequencingWalletClient
    })

  // 4. Register AllowlistSequencingModule on RequireAllModule
  await registerAllowlistSequencingModuleOnRequireAllModule({
    requireAndModule,
    allowlistSequencingModule,
    deployerSequencingWalletClient,
    sequencingPublicClient
  })

  // 5. Transfer ownership of all contracts to owner account
  await transferAllContractsOwnership({
    sequencingContract,
    allowlistSequencingModule,
    requireAndModule,
    deployerSequencingWalletClient,
    sequencingPublicClient,
    ownerSequencingWalletClient
  })

  return {
    sequencingContract,
    allowlistSequencingModule,
    requireAndModule,
    deployedAtBlock
  }
}

async function createRequireAndModule({
  chainId,
  sequencingPublicClient,
  deployerSequencingWalletClient
}: CreateRequireAndModuleParams) {
  const requireAndFactoryAddress =
    supportedSequencingChains[sequencingPublicClient.chain.id].requireAndFactory

  const { request: requireAndModuleRequest } =
    await sequencingPublicClient.simulateContract({
      account: deployerSequencingWalletClient.account,
      address: requireAndFactoryAddress,
      abi: requireAndModuleFactoryABI,
      functionName: "createRequireAndModule",
      args: [
        deployerSequencingWalletClient.account.address,
        toHex(toBytes(chainId, { size: 32 }))
      ]
    })
  const requireAndModuleHash =
    await deployerSequencingWalletClient.writeContract(requireAndModuleRequest)
  const requireAndModuleTx =
    await sequencingPublicClient.waitForTransactionReceipt({
      hash: requireAndModuleHash
    })
  const requireAndFactoryLogs = parseEventLogs({
    abi: requireAndModuleFactoryABI,
    logs: requireAndModuleTx.logs
  })
  const requireAndModule = requireAndFactoryLogs.find(
    (l) => l.eventName === "RequireAndModuleCreated"
  )?.args.module
  if (!requireAndModule) {
    throw new Error("RequireAndModule deployment failed")
  }
  print(
    `🔍  RequireAndModule deployed to ${requireAndModule}\n${getChainExplorerUrl(
      sequencingPublicClient.chain
    )}/tx/${requireAndModuleHash}`
  )
  return requireAndModule
}

async function createSyndicateSequencingChain({
  requireAndModule,
  sequencingPublicClient,
  deployerSequencingWalletClient,
  chainId
}: CreateSyndicateSequencingChainParams) {
  const syndicateFactoryAddress =
    supportedSequencingChains[sequencingPublicClient.chain.id].syndicateFactory
  const { request: syndicateSequencingChainRequest } =
    await sequencingPublicClient.simulateContract({
      account: deployerSequencingWalletClient.account,
      address: syndicateFactoryAddress,
      abi: syndicateFactoryABI,
      functionName: "createSyndicateSequencingChain",
      args: [
        BigInt(chainId),
        deployerSequencingWalletClient.account.address,
        requireAndModule,
        toHex(toBytes(chainId, { size: 32 }))
      ]
    })
  const syndicateSequencingChainHash =
    await deployerSequencingWalletClient.writeContract(
      syndicateSequencingChainRequest
    )
  const syndicateSequencingChainTx =
    await sequencingPublicClient.waitForTransactionReceipt({
      hash: syndicateSequencingChainHash
    })
  const syndicateFactoryLogs = parseEventLogs({
    abi: syndicateFactoryABI,
    logs: syndicateSequencingChainTx.logs
  })
  const sequencingContract = syndicateFactoryLogs.find(
    (l) => l.eventName === "SyndicateSequencingChainCreated"
  )?.args.sequencingChainAddress
  if (!sequencingContract) {
    throw new Error("SyndicateSequencingChain deployment failed")
  }
  print(
    `🔍  SyndicateSequencingChain deployed to ${sequencingContract}\n${getChainExplorerUrl(
      sequencingPublicClient.chain
    )}/tx/${syndicateSequencingChainHash}`
  )
  return {
    sequencingContract,
    deployedAtBlock: syndicateSequencingChainTx.blockNumber
  }
}

async function deployAndSetupAllowlistSequencingModule({
  sequencerAccount,
  sequencingPublicClient,
  deployerSequencingWalletClient
}: DeployAndSetupAllowlistSequencingModuleParams) {
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
}: RegisterAllowlistSequencingModuleOnRequireAllModuleParams) {
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

async function transferAllContractsOwnership({
  sequencingContract,
  allowlistSequencingModule,
  requireAndModule,
  deployerSequencingWalletClient,
  sequencingPublicClient,
  ownerSequencingWalletClient
}: TransferAllContractsOwnershipParams) {
  const transferOwnershipTxHash =
    await deployerSequencingWalletClient.writeContract({
      address: sequencingContract,
      abi: syndicateSequencingChainABI,
      functionName: "transferOwnership",
      args: [ownerSequencingWalletClient.account.address],
      account: deployerSequencingWalletClient.account
    })
  const transferOwnershipTx =
    await sequencingPublicClient.waitForTransactionReceipt({
      hash: transferOwnershipTxHash
    })
  print(
    `🔍  SyndicateSequencingChain ownership transferred to owner ${ownerSequencingWalletClient.account.address}\n${getChainExplorerUrl(
      sequencingPublicClient.chain
    )}/tx/${transferOwnershipTx.transactionHash}`
  )

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
