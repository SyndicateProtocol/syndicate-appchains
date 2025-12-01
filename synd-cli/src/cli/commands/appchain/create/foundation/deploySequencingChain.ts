import {type Hex, zeroAddress } from "viem"

import {
  allowlistSequencingModuleABI,
  allowlistSequencingModuleBytecode
} from "@/abi/synd/AllowlistSequencingModule"
import { requireAndModuleABI } from "@/abi/synd/RequireAndModule"
import { syndicateSequencingChainABI } from "@/abi/synd/SyndicateSequencingChain"
import type {
  DeployAndSetupAllowlistSequencingModule,
  DeploySequencingChain,
  RegisterAllowlistSequencingModuleOnRequireAllModule,
  TransferAllContractsOwnershipParams
} from "@/types"
import { getChainExplorerUrl } from "@/utils/helpers"
import { print } from "@/utils/print"

export async function deploySequencingChain({
  sequencerAccount,
  chainId,
  sequencingPublicClient,
  deployerSequencingWalletClient,
  ownerSequencingWalletClient
}: DeploySequencingChain) {
  // 1. Create RequireAndModule
  const requireAndModule = await createRequireAndModule({
    chainId,
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

  // 3. Create SyndicateSequencingChain
  const { sequencingContract, deployedAtBlock } =
    await createSyndicateSequencingChain({
      requireAndModule,
      sequencingPublicClient,
      deployerSequencingWalletClient,
      chainId
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

async function createRequireAndModule(input: any) {
  // const { chainId, sequencingPublicClient, deployerSequencingWalletClient } =
  //   await getFoundationConfig()

  // const requireAndFactoryAddress =
  //   supportedSequencingChains[sequencingPublicClient.chain.id]
  //     .requireAndFactoryAddress

  // const { request: requireAndModuleRequest } =
  //   await sequencingPublicClient.simulateContract({
  //     account: deployerSequencingWalletClient.account,
  //     address: requireAndFactoryAddress,
  //     abi: requireAndModuleFactoryABI,
  //     functionName: "createRequireAndModule",
  //     args: [
  //       deployerSequencingWalletClient.account.address,
  //       toHex(toBytes(chainId, { size: 32 }))
  //     ]
  //   })
  // const requireAndModuleHash =
  //   await deployerSequencingWalletClient.writeContract(requireAndModuleRequest)
  // const requireAndModuleTx =
  //   await sequencingPublicClient.waitForTransactionReceipt({
  //     hash: requireAndModuleHash
  //   })
  // const requireAndFactoryLogs = parseEventLogs({
  //   abi: requireAndModuleFactoryABI,
  //   logs: requireAndModuleTx.logs
  // })
  // const requireAndModuleAddress = requireAndFactoryLogs.find(
  //   (l) => l.eventName === "RequireAndModuleCreated"
  // )?.args.module
  // if (!requireAndModuleAddress) {
  //   throw new Error("RequireAndModule deployment failed")
  // }
  // print(
  //   `🔍  RequireAndModule deployed to ${requireAndModuleAddress}\n${getChainExplorerUrl(
  //     sequencingPublicClient.chain
  //   )}/tx/${requireAndModuleHash}`
  // )
  // return requireAndModuleAddress

  // TODO (ENG-2215)
  return zeroAddress
}

async function createSyndicateSequencingChain(requireAndModuleAddress: Hex) {
  // const { chainId, sequencingPublicClient, deployerSequencingWalletClient } =
  //   await getFoundationConfig()

  // const syndicateFactoryAddress =
  //   supportedSequencingChains[sequencingPublicClient.chain.id]
  //     .syndicateFactoryAddress
  // const { request: syndicateSequencingChainRequest } =
  //   await sequencingPublicClient.simulateContract({
  //     account: deployerSequencingWalletClient.account,
  //     address: syndicateFactoryAddress,
  //     abi: syndicateFactoryABI,
  //     functionName: "createSyndicateSequencingChainWithCustomId",
  //     args: [
  //       BigInt(chainId),
  //       deployerSequencingWalletClient.account.address,
  //       requireAndModuleAddress
  //     ]
  //   })
  // const syndicateSequencingChainHash =
  //   await deployerSequencingWalletClient.writeContract(
  //     syndicateSequencingChainRequest
  //   )
  // const syndicateSequencingChainTx =
  //   await sequencingPublicClient.waitForTransactionReceipt({
  //     hash: syndicateSequencingChainHash
  //   })
  // const syndicateFactoryLogs = parseEventLogs({
  //   abi: syndicateFactoryABI,
  //   logs: syndicateSequencingChainTx.logs
  // })
  // const syndicateSequencingChainAddress = syndicateFactoryLogs.find(
  //   (l) => l.eventName === "SyndicateSequencingChainCreated"
  // )?.args.sequencingChainAddress
  // if (!syndicateSequencingChainAddress) {
  //   throw new Error("SyndicateSequencingChain deployment failed")
  // }
  // print(
  //   `🔍  SyndicateSequencingChain deployed to ${syndicateSequencingChainAddress}\n${getChainExplorerUrl(
  //     sequencingPublicClient.chain
  //   )}/tx/${syndicateSequencingChainHash}`
  // )
  // return {
  //   syndicateSequencingChainAddress,
  //   deployedAtBlock: syndicateSequencingChainTx.blockNumber
  // }

  // TODO (ENG-2215)
  return {
    sequencingContract: zeroAddress,
    deployedAtBlock: BigInt(0)
  }
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
