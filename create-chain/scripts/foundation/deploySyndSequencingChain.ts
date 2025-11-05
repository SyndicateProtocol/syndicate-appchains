import { type Account, type Hex, parseEventLogs, toBytes, toHex } from "viem"

import {
  allowlistSequencingModuleABI,
  allowlistSequencingModuleBytecode
} from "@/scripts/abi/synd/AllowlistSequencingModule"
import { requireAndModuleABI } from "@/scripts/abi/synd/RequireAndModule"
import { requireAndModuleFactoryABI } from "@/scripts/abi/synd/RequireAndModuleFactory"
import { syndicateFactoryABI } from "@/scripts/abi/synd/SyndicateFactory"
import { syndicateSequencingChainABI } from "@/scripts/abi/synd/SyndicateSequencingChain"
import { supportedSequencingChains } from "@/scripts/utils/constants"
import { getFoundationConfig } from "../utils/config"
import { getChainExplorerUrl } from "../utils/helpers"
import { print } from "../utils/print"

export async function deploySyndSequencingChain(sequencerAccount: Account) {
  // 1. Create RequireAndModule
  const requireAndModuleAddress = await createRequireAndModule()

  // 2. Create SyndicateSequencingChain
  const { syndicateSequencingChainAddress, deployedAtBlock } =
    await createSyndicateSequencingChain(requireAndModuleAddress)

  // 3. Deploy and setup AllowlistSequencingModule
  const allowlistSequencingModuleAddress =
    await deployAndSetupAllowlistSequencingModule(sequencerAccount)

  // 4. Register AllowlistSequencingModule on RequireAllModule
  await registerAllowlistSequencingModuleOnRequireAllModule(
    requireAndModuleAddress,
    allowlistSequencingModuleAddress
  )

  // 5. Transfer ownership of all contracts to owner account
  await transferAllContractsOwnership({
    syndicateSequencingChainAddress,
    allowlistSequencingModuleAddress,
    requireAndModuleAddress
  })

  return {
    syndicateSequencingChainAddress,
    allowlistSequencingModuleAddress,
    requireAndModuleAddress,
    deployedAtBlock
  }
}

async function createRequireAndModule() {
  const { chainId, sequencingPublicClient, deployerSequencingWalletClient } =
    await getFoundationConfig()

  const requireAndFactoryAddress =
    supportedSequencingChains[sequencingPublicClient.chain.id]
      .requireAndFactoryAddress

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
  const requireAndModuleAddress = requireAndFactoryLogs.find(
    (l) => l.eventName === "RequireAndModuleCreated"
  )?.args.module
  if (!requireAndModuleAddress) {
    throw new Error("RequireAndModule deployment failed")
  }
  print(
    `🔍  RequireAndModule deployed to ${requireAndModuleAddress}\n${getChainExplorerUrl(
      sequencingPublicClient.chain
    )}/tx/${requireAndModuleHash}`
  )
  return requireAndModuleAddress
}

async function createSyndicateSequencingChain(requireAndModuleAddress: Hex) {
  const { chainId, sequencingPublicClient, deployerSequencingWalletClient } =
    await getFoundationConfig()

  const syndicateFactoryAddress =
    supportedSequencingChains[sequencingPublicClient.chain.id]
      .syndicateFactoryAddress
  const { request: syndicateSequencingChainRequest } =
    await sequencingPublicClient.simulateContract({
      account: deployerSequencingWalletClient.account,
      address: syndicateFactoryAddress,
      abi: syndicateFactoryABI,
      functionName: "createSyndicateSequencingChainWithCustomId",
      args: [
        BigInt(chainId),
        deployerSequencingWalletClient.account.address,
        requireAndModuleAddress
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
  const syndicateSequencingChainAddress = syndicateFactoryLogs.find(
    (l) => l.eventName === "SyndicateSequencingChainCreated"
  )?.args.sequencingChainAddress
  if (!syndicateSequencingChainAddress) {
    throw new Error("SyndicateSequencingChain deployment failed")
  }
  print(
    `🔍  SyndicateSequencingChain deployed to ${syndicateSequencingChainAddress}\n${getChainExplorerUrl(
      sequencingPublicClient.chain
    )}/tx/${syndicateSequencingChainHash}`
  )
  return {
    syndicateSequencingChainAddress,
    deployedAtBlock: syndicateSequencingChainTx.blockNumber
  }
}

async function deployAndSetupAllowlistSequencingModule(
  sequencerAccount: Account
) {
  const { sequencingPublicClient, deployerSequencingWalletClient } =
    await getFoundationConfig()
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
  const allowlistSequencingModuleAddress = sequencingModuleTx.contractAddress
  if (!allowlistSequencingModuleAddress) {
    throw new Error("AllowlistSequencingModule deployment failed")
  }
  print(
    `🔍  AllowlistSequencingModule deployed to ${allowlistSequencingModuleAddress}\n${getChainExplorerUrl(
      sequencingPublicClient.chain
    )}/tx/${allowlistSequencingModuleHash}`
  )

  // 2: Allowlist wallet pool by calling 'addToAllowlist' on AllowlistSequencingModule
  const addAllowlistTxHash = await deployerSequencingWalletClient.writeContract(
    {
      address: allowlistSequencingModuleAddress,
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
    `🔍  Sequencer ${sequencerAccount.address} added to sequencer allowlist\n${getChainExplorerUrl(
      sequencingPublicClient.chain
    )}/tx/${allowlistSequencerTx.transactionHash}`
  )
  return allowlistSequencingModuleAddress
}

async function registerAllowlistSequencingModuleOnRequireAllModule(
  requireAndModuleAddress: Hex,
  allowlistSequencingModuleAddress: Hex
) {
  const { sequencingPublicClient, deployerSequencingWalletClient } =
    await getFoundationConfig()
  const registerSequencerAllowlistTxHash =
    await deployerSequencingWalletClient.writeContract({
      address: requireAndModuleAddress,
      abi: requireAndModuleABI,
      functionName: "addPermissionCheck",
      args: [allowlistSequencingModuleAddress, true],
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
  syndicateSequencingChainAddress,
  allowlistSequencingModuleAddress,
  requireAndModuleAddress
}: {
  syndicateSequencingChainAddress: Hex
  allowlistSequencingModuleAddress: Hex
  requireAndModuleAddress: Hex
}) {
  const {
    sequencingPublicClient,
    deployerSequencingWalletClient,
    ownerSequencingWalletClient
  } = await getFoundationConfig()
  const transferOwnershipTxHash =
    await deployerSequencingWalletClient.writeContract({
      address: syndicateSequencingChainAddress,
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
      address: allowlistSequencingModuleAddress,
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
      address: requireAndModuleAddress,
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
