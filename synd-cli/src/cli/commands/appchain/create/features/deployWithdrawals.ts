import { teeKeyManagerABI } from "@/abi/synd/TeeKeyManager"
import { teeModuleABI } from "@/abi/synd/TeeModule"
import type {
  DeployTeeModule,
  PrivateKeyWalletAccount,
  PublicClientWithChain
} from "@/types"
import { supportedSettlementChains } from "@/utils/constants"
import { getChainExplorerUrl } from "@/utils/helpers"
import { print } from "@/utils/print"
import { sleep } from "bun"
import { type Hex, parseAbi } from "viem"
import { deployAssertionPoster } from "./deployAssertionPoster"
import { deployTeeModule } from "./deployTeeModule"

export async function deployWithdrawals({
  settlementPublicClient,
  coreContracts: { rollup, upgradeExecutor, bridge },
  deployerSettlementWalletClient,
  ownerSettlementWalletClient,
  sequencingContractAddress,
  sequencingPublicClient,
  appchainPublicClient,
  ethereumPublicClient,
  syndForkSequencingRpc
}: DeployTeeModule) {
  const assertionPosterAddress = await deployAssertionPoster({
    rollup,
    upgradeExecutor,
    settlementPublicClient,
    deployerSettlementWalletClient,
    ownerSettlementWalletClient
  })
  const teeModuleAddress = await deployTeeModule({
    assertionPosterAddress,
    bridge,
    deployerSettlementWalletClient,
    settlementPublicClient,
    sequencingContractAddress,
    sequencingPublicClient,
    appchainPublicClient,
    ethereumPublicClient,
    syndForkSequencingRpc
  })

  // Sleep a bit to ensure contracts are ready to have ownership transferred
  await sleep(2_000)

  // AssertionPoster should be owned by the TeeModule
  const assertionPosterOwnershipTransferHash =
    await transferOwnershipFromDeployer(
      assertionPosterAddress,
      teeModuleAddress,
      settlementPublicClient,
      deployerSettlementWalletClient
    )
  print(
    `🔍  AssertionPoster ownership transferred to TeeModule (${teeModuleAddress})\n${getChainExplorerUrl(
      settlementPublicClient.chain
    )}/tx/${assertionPosterOwnershipTransferHash}`
  )

  // TeeModule's DEFAULT_ADMIN_ROLE should be set to the owner
  await transferTeeModuleDefaultAdminRole({
    teeModuleAddress,
    ownerSettlementWalletClient,
    settlementPublicClient,
    deployerSettlementWalletClient
  })

  // Read attestation doc verifier address from TeeKeyManager
  const teeKeyManagerAddress =
    supportedSettlementChains[settlementPublicClient.chain.id]
      .teeKeyManagerAddress

  const attestationDocVerifierAddress =
    await settlementPublicClient.readContract({
      address: teeKeyManagerAddress,
      abi: teeKeyManagerABI,
      functionName: "attestationDocVerifier"
    })

  return {
    assertionPosterAddress,
    teeModuleAddress,
    attestationDocVerifierAddress,
    teeKeyManagerAddress
  }
}

export async function transferOwnershipFromDeployer(
  address: Hex,
  newOwner: Hex,
  settlementPublicClient: PublicClientWithChain,
  deployerSettlementWalletClient: PrivateKeyWalletAccount
) {
  print(`🔍  Transferring ownership of ${address} to ${newOwner}`)

  const owner = await settlementPublicClient.readContract({
    address: address,
    abi: parseAbi(["function owner() public view returns (address)"]),
    functionName: "owner"
  })

  if (owner !== deployerSettlementWalletClient.account.address) {
    throw new Error(
      `Current owner of ${address} is not the deployer. Current owner: ${owner}. Deployer: ${deployerSettlementWalletClient.account.address}`
    )
  }

  const transferOwnershipHash =
    await deployerSettlementWalletClient.writeContract({
      address: address,
      abi: parseAbi(["function transferOwnership(address newOwner) public"]),
      functionName: "transferOwnership",
      args: [newOwner]
    })

  const transferReceipt =
    await settlementPublicClient.waitForTransactionReceipt({
      hash: transferOwnershipHash
    })

  if (transferReceipt.status !== "success") {
    throw new Error("AssertionPoster ownership transfer to TeeModule failed")
  }
  return transferOwnershipHash
}

async function transferTeeModuleDefaultAdminRole({
  teeModuleAddress,
  ownerSettlementWalletClient,
  settlementPublicClient,
  deployerSettlementWalletClient
}: {
  teeModuleAddress: Hex
  ownerSettlementWalletClient: PrivateKeyWalletAccount
  settlementPublicClient: PublicClientWithChain
  deployerSettlementWalletClient: PrivateKeyWalletAccount
}) {
  const defaultAdminRole = await settlementPublicClient.readContract({
    address: teeModuleAddress,
    abi: teeModuleABI,
    functionName: "DEFAULT_ADMIN_ROLE"
  })
  const teeModuleDefaultAdminRoleTransferHash =
    await deployerSettlementWalletClient.writeContract({
      address: teeModuleAddress,
      abi: teeModuleABI,
      functionName: "grantRole",
      args: [defaultAdminRole, ownerSettlementWalletClient.account.address]
    })
  await settlementPublicClient.waitForTransactionReceipt({
    hash: teeModuleDefaultAdminRoleTransferHash
  })
  print(
    `🔍  TeeModule's DEFAULT_ADMIN_ROLE set to owner (${
      ownerSettlementWalletClient.account.address
    })\n${getChainExplorerUrl(
      settlementPublicClient.chain
    )}/tx/${teeModuleDefaultAdminRoleTransferHash}`
  )

  // Revoke the DEFAULT_ADMIN_ROLE from the deployer
  const teeModuleDefaultAdminRoleRevokeHash =
    await deployerSettlementWalletClient.writeContract({
      address: teeModuleAddress,
      abi: teeModuleABI,
      functionName: "revokeRole",
      args: [defaultAdminRole, deployerSettlementWalletClient.account.address]
    })
  await settlementPublicClient.waitForTransactionReceipt({
    hash: teeModuleDefaultAdminRoleRevokeHash
  })
  print(
    `🔍  TeeModule's DEFAULT_ADMIN_ROLE revoked from deployer (${
      deployerSettlementWalletClient.account.address
    })\n${getChainExplorerUrl(
      settlementPublicClient.chain
    )}/tx/${teeModuleDefaultAdminRoleRevokeHash}`
  )
}
