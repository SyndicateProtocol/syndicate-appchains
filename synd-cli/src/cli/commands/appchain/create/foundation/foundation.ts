import type { Foundation } from "@/types"
import { supportedSequencingChains } from "@/utils/constants"
import { upsertToEoaSecrets, upsertToSyndObject, writeToFile } from "@/utils/fs"
import { fundAccount } from "@/utils/fundAccount"
import { generateBridgeConfig } from "@/utils/generateBridgeConfig"
import {
  getDoesChainExist,
  getNativeCurrency,
  isNativeTokenEth
} from "@/utils/helpers"
import { print } from "@/utils/print"
import type { ChainConfig, CoreContracts } from "@arbitrum/orbit-sdk"
import { type Hex, parseEther, stringify } from "viem"
import { generatePrivateKey, privateKeyToAccount } from "viem/accounts"
import { getBlockNumber } from "viem/actions"
import { createArbChainConfig } from "./createArbChainConfig"
import { deployNitroRollup } from "./deployNitroRollup"
import { deploySequencingChain } from "./deploySequencingChain"
import { getConfigAndCoreContracts } from "./getConfigAndCoreContracts"

export async function foundation({
  chainId,
  chainName,
  nativeToken,
  deployerSettlementWalletClient,
  ownerSettlementWalletClient,
  ownerSequencingWalletClient,
  settlementPublicClient,
  sequencingPublicClient,
  ethereumChainRpcUrl,
  deployerSequencingWalletClient,
  ownerPrivateKey,
  coreContractsCreatedAtHash,
  appchainRpc,
  appchainExplorer
}: Foundation) {
  const sequencerPrivateKey = generatePrivateKey()
  const sequencerAccount = privateKeyToAccount(sequencerPrivateKey)
  const nativeCurrency = nativeToken
    ? await getNativeCurrency(settlementPublicClient, nativeToken)
    : undefined
  const isTestnet = ownerSettlementWalletClient.chain.testnet
  const environment = isTestnet ? "testnet" : "mainnet"

  // Fetch core contracts and chain config if hash is provided
  let chainConfig: ChainConfig | undefined
  let coreContracts: CoreContracts | undefined
  if (coreContractsCreatedAtHash) {
    try {
      const result = await getConfigAndCoreContracts({
        hash: coreContractsCreatedAtHash as `0x${string}`,
        settlementPublicClient
      })
      chainConfig = result.chainConfig
      coreContracts = result.coreContracts
    } catch (_error) {
      throw new Error(
        "Failed to fetch existing core contracts and chain config from coreContractsCreatedAtHash. Please make sure coreContractsCreatedAtHash is valid and the transaction has been confirmed on the settlement chain."
      )
    }
  }

  print("              CREATE INITIAL DEPENDENCIES FOR APPCHAIN              ")
  print("⚠️  Please confirm the following details before proceeding ⚠️")
  print("---------------------------------------------------------")

  print("Chain ID", chainId)
  print("Chain Name", chainName)
  coreContractsCreatedAtHash &&
    print("Core Contracts Created At Hash", coreContractsCreatedAtHash)
  print("Deployer", deployerSettlementWalletClient.account.address)
  print("Sequencer Address", sequencerAccount.address)
  print("Sequencer PK", sequencerPrivateKey)
  print("Owner Address", ownerSettlementWalletClient.account.address)
  print("Owner PK", ownerPrivateKey)
  print(
    "Native Token",
    isNativeTokenEth(nativeToken)
      ? "ETH"
      : `${nativeToken} (${nativeCurrency?.symbol})`
  )
  print("Settlement Chain", settlementPublicClient.chain.name)
  print("Sequencing Chain", sequencingPublicClient.chain.name)
  print("Environment", environment)

  if (coreContractsCreatedAtHash && coreContracts) {
    print("---------------------------------------------------------")
    print("              EXISTING CORE CONTRACTS                    ")
    print("---------------------------------------------------------")
    print("Rollup", coreContracts.rollup)
    print("Inbox", coreContracts.inbox)
    print("Outbox", coreContracts.outbox)
    print("Bridge", coreContracts.bridge)
    print("Sequencer Inbox", coreContracts.sequencerInbox)
    print("Rollup Event Inbox", coreContracts.rollupEventInbox)
    print("Challenge Manager", coreContracts.challengeManager)
    print("Admin Proxy", coreContracts.adminProxy)
    print("Upgrade Executor", coreContracts.upgradeExecutor)
    print("Validator Wallet Creator", coreContracts.validatorWalletCreator)
    if (coreContracts.validatorUtils) {
      print("Validator Utils", coreContracts.validatorUtils)
    }
    print("Deployed At Block", coreContracts.deployedAtBlockNumber)

    print("---------------------------------------------------------")
    print("              EXISTING CHAIN CONFIG                      ")
    print("---------------------------------------------------------")
    print("Chain Config", stringify(chainConfig, null, 2))
    print("---------------------------------------------------------")
  }
  print("---------------------------------------------------------")

  const promptResponse = prompt("Are you sure you want to proceed? (y/n)")
  const confirmation = ["y", "yes"]
  if (!promptResponse || !confirmation.includes(promptResponse.toLowerCase())) {
    print("🚫 Exiting...")
    return
  }

  if (!coreContractsCreatedAtHash && (await getDoesChainExist(chainId))) {
    print(`🚫 Chain ${chainId} already exists`)
    process.exit(1)
  }

  print(`⚙️   Setting up ${chainName}...`)

  const settlementBlockBeforeDeployment = (
    await getBlockNumber(settlementPublicClient)
  ).toString()
  print(
    `🔍  Settlement block before sequencing deployment: ${settlementBlockBeforeDeployment}`
  )

  let bridgeConfig: ReturnType<typeof generateBridgeConfig>

  if (coreContractsCreatedAtHash && chainConfig && coreContracts) {
    print("🔍  Using existing Nitro core contracts")
    bridgeConfig = generateBridgeConfig({
      coreContracts,
      chainName,
      chainId,
      parentChainId: settlementPublicClient.chain.id,
      chainOwner: ownerSettlementWalletClient.account.address,
      rpcUrl: appchainRpc,
      explorerUrl: appchainExplorer
    })
  } else {
    print("🔍  Deploying Nitro core contracts")
    const nitroDeploymentResult = await deployNitroRollup({
      chainId,
      chainName,
      ownerSettlementWalletClient,
      settlementPublicClient,
      appchainRpc,
      appchainExplorer,
      nativeToken,
      deployerSettlementWalletClient
    })
    chainConfig = nitroDeploymentResult.chainConfig
    coreContracts = nitroDeploymentResult.coreContracts
    bridgeConfig = nitroDeploymentResult.bridgeConfig
    print("✅  Nitro Rollup deployed")
  }

  await upsertToSyndObject(chainName, environment, "bridge", bridgeConfig)

  print("---------------------------------------------------------")

  print("🔍  Deploying Syndicate sequencing chain...")
  // Sequencing Chain
  const {
    sequencingContract,
    allowlistSequencingModule,
    requireAndModule,
    deployedAtBlock
  } = await deploySequencingChain({
    sequencerAccount,
    chainId,
    sequencingPublicClient,
    deployerSequencingWalletClient,
    ownerSequencingWalletClient
  })
  await upsertToSyndObject(chainName, environment, "sequencing", {
    sequencingContract,
    allowlistSequencingModule,
    requireAndModule,
    settlementBlockBeforeDeployment,
    deployedAtBlock,
    sequencer: sequencerAccount.address
  })

  print("✅  Syndicate Sequencing Chain deployed")
  print("---------------------------------------------------------")

  const arbChainConfigAddress = await createArbChainConfig({
    coreContracts,
    settlementStartBlock: settlementBlockBeforeDeployment,
    sequencingContract,
    sequencingStartBlock: deployedAtBlock,
    ownerSettlementWalletClient,
    settlementPublicClient,
    sequencingPublicClient,
    appchainExplorer,
    chainId,
    deployerSettlementWalletClient
  })

  await upsertToSyndObject(chainName, environment, "config", {
    arbChainConfig: arbChainConfigAddress
  })
  print("✅  Arb Chain Config deployed")
  print("---------------------------------------------------------")

  // Write all EOA secrets to a single file
  await upsertToEoaSecrets(chainName, environment, "owner", {
    address: privateKeyToAccount(ownerPrivateKey as Hex).address,
    privateKey: ownerPrivateKey
  })
  await upsertToEoaSecrets(chainName, environment, "sequencer", {
    address: sequencerAccount.address,
    privateKey: sequencerPrivateKey
  })

  const internalChainConfig = {
    ...chainConfig,
    arbitrum: {
      ...chainConfig.arbitrum,
      EnableArbOS: true,
      AllowDebugPrecompiles: false,
      GenesisBlockNum: 0,
      Syndicate: true
    }
  }

  // Generate appchain secret file
  await writeToFile(
    chainName,
    `${environment}-${chainName}-${chainId}-secret.json`,
    stringify(
      {
        "settlement-rpc-url": settlementPublicClient.transport.url,
        "sequencing-chain-rpc-url": sequencingPublicClient.transport.url,
        "ethereum-rpc-url": ethereumChainRpcUrl,
        "sequencing-chain-contract-address": sequencingContract,
        "sequencer-private-key": sequencerPrivateKey,
        "assertion-poster-contract-address": "0xREPLACEMELATER",
        "tee-module-address": "0xREPLACEMELATER",
        "proposer-private-key":
          "REPLACEMELATER - private key with no 0x prefix",
        "config-manager-address": arbChainConfigAddress,
        "appchain-bridge-address": bridgeConfig.coreContracts.bridge,
        "sequencing-bridge-address":
          supportedSequencingChains[sequencingPublicClient.chain.id].bridge,
        "memorystore-read-endpoint": isTestnet
          ? "redis://10.40.0.17:6379"
          : "redis://10.60.0.2:6379",
        "maestro-rpc-urls": {
          [chainId]: [`http://${chainName}-${chainId}-nitro:8449/rpc`]
        },
        "chain-info-json": [
          {
            "chain-id": chainId,
            "parent-chain-id": 511000,
            "parent-chain-is-arbitrum": false,
            "sequencer-url": "null",
            "has-genesis-state": false,
            "chain-name": chainName,
            "chain-config": internalChainConfig,
            rollup: {
              bridge: "0x0000000000000000000000000000000000511000",
              inbox: "0x0000000000000000000000000000000000000000",
              "sequencer-inbox": "0x0000000000000000000000000000000000511000",
              "deployed-at": 1,
              rollup: "0x0000000000000000000000000000000000000000",
              "native-token": nativeToken,
              "upgrade-executor": "0x0000000000000000000000000000000000000000",
              "validator-wallet-creator":
                "0x0000000000000000000000000000000000000000"
            }
          }
        ]
      },
      null,
      2
    )
  )

  print(
    `🏁  Appchain setup complete. Artifacts saved at:
    - ./appchains/${chainName}/${environment}.synd.${chainName}.json
    - ./appchains/${chainName}/${environment}-eoaSecrets.${chainName}.json
    - ./appchains/${chainName}/${environment}-${chainName}-${chainId}-secret.json`
  )

  print("---------------------------------------------------------")
  print("                      Fund Accounts                      ")
  print("⚠️  Please confirm the following details before proceeding ⚠️")
  print("---------------------------------------------------------")
  print("Sequencer", sequencerAccount.address)
  print("---------------------------------------------------------")
  const fundResponse = prompt(
    "Do you want to fund the following account? (y/n)"
  )
  const fundConfirmation = ["y", "yes"]
  if (!fundResponse || !fundConfirmation.includes(fundResponse.toLowerCase())) {
    print("🚫 Skipping funding...")
  } else {
    await fundAccount(
      deployerSequencingWalletClient,
      sequencerAccount.address,
      parseEther("0.1")
    )
  }

  print("🏁  Foundation setup complete")
}
