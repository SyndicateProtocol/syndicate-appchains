import { sleep } from "bun"
import { erc20Abi, formatEther, parseEther, parseUnits } from "viem"
import { generatePrivateKey, privateKeyToAccount } from "viem/accounts"
import { getFeaturesConfig } from "../utils/config"
import { supportedSettlementChains } from "../utils/constants"
import { fundAccount } from "../utils/fundAccount"
import { generateBridgeConfig } from "../utils/generateBridgeConfig"
import {
  getChainRpcUrl,
  getNativeCurrency,
  isNativeTokenEth,
  upsertToEoaSecrets,
  upsertToSyndObject,
  writeToFile
} from "../utils/helpers"
import { print } from "../utils/print"
import { createTokenBridge } from "./createTokenBridge"
import { canDeployMulticall3, deployMulticall3 } from "./deployMulticall3"
import { deployTeeModule } from "./deployTeeModule"
import { pollEmptyTxs } from "./pollEmptyTxs"

async function main() {
  const {
    coreContracts,
    chainId,
    chainName,
    appchainPublicClient,
    deployerSequencingWalletClient,
    ownerSettlementWalletClient,
    deployerSettlementWalletClient,
    settlementPublicClient
  } = await getFeaturesConfig()

  const nativeCurrency = await getNativeCurrency(
    settlementPublicClient,
    coreContracts.nativeToken
  )
  const environment = ownerSettlementWalletClient.chain.testnet
    ? "testnet"
    : "mainnet"

  print("                  CREATE TOKEN BRIDGE                  ")
  print("⚠️  Please confirm the following details before proceeding ⚠️")
  print("---------------------------------------------------------")
  print("Chain ID", appchainPublicClient.chain.id)
  print("Chain Name", appchainPublicClient.chain.name)
  print("RPC URL", getChainRpcUrl(appchainPublicClient.chain))
  print("Deployer", deployerSettlementWalletClient.account.address)
  print("Owner", ownerSettlementWalletClient.account.address)
  print("Rollup", coreContracts.rollup)
  print(
    "Native Token",
    isNativeTokenEth(coreContracts.nativeToken)
      ? "ETH"
      : `${coreContracts.nativeToken} (${nativeCurrency?.symbol})`
  )
  print("Settlement Chain", settlementPublicClient.chain.name.toLowerCase())
  print("Appchain Chain", appchainPublicClient.chain.name.toLowerCase())
  print("Environment", environment)
  print("---------------------------------------------------------")

  const promptResponse = prompt("Are you sure you want to proceed? (y/n)")
  const confirmation = ["y", "yes"]
  if (!promptResponse || !confirmation.includes(promptResponse.toLowerCase())) {
    print("🚫 Exiting...")
    return process.exit(1)
  }

  print("🔎 Creating token bridge")

  // Owner must have a balance of native token to pay for retryables
  const estimatedCostOfRetryables = parseEther("0.015")
  if (isNativeTokenEth(coreContracts.nativeToken)) {
    const balance = await settlementPublicClient.getBalance({
      address: ownerSettlementWalletClient.account.address
    })
    if (balance < estimatedCostOfRetryables) {
      print("---------------------------------------------------------")
      print("                    Insufficient Balance                    ")
      print("⚠️  Owner does not have enough balance for retryables ⚠️")
      print("---------------------------------------------------------")
      print("Owner", ownerSettlementWalletClient.account.address)
      print("Owner Balance", `${formatEther(balance)} ETH`)
      print("Required Balance", `${formatEther(estimatedCostOfRetryables)} ETH`)
      print("Deployer", deployerSettlementWalletClient.account.address)
      print("---------------------------------------------------------")

      const fundResponse = prompt(
        "Do you want to fund the owner from the deployer? (y/n)"
      )
      const fundConfirmation = ["y", "yes"]
      if (
        !fundResponse ||
        !fundConfirmation.includes(fundResponse.toLowerCase())
      ) {
        throw new Error(
          `🚫 Owner does not have balance of native token on settlement chain. Owner balance: ${formatEther(balance)} ETH, estimated cost of retryables: ${formatEther(estimatedCostOfRetryables)} ETH`
        )
      }

      print("💰 Funding owner from deployer...")
      await fundAccount(
        deployerSettlementWalletClient,
        ownerSettlementWalletClient.account.address,
        estimatedCostOfRetryables
      )
      await sleep(1000)
      print("✅ Owner funded successfully")
    }
  } else {
    const nativeTokenBalance = await settlementPublicClient.readContract({
      address: coreContracts.nativeToken,
      abi: erc20Abi,
      functionName: "balanceOf",
      args: [ownerSettlementWalletClient.account.address]
    })
    const tokenDecimals = settlementPublicClient.chain.nativeCurrency.decimals
    const convertedBalance =
      parseUnits(nativeTokenBalance.toString(), tokenDecimals) *
      BigInt(10 ** (18 - tokenDecimals))
    if (convertedBalance <= estimatedCostOfRetryables) {
      throw new Error(
        `🚫 Owner does not have balance of ${nativeCurrency.symbol} (${coreContracts.nativeToken}) on the settlement chain. Owner balance: ${formatEther(convertedBalance)} ${nativeCurrency.symbol}, estimated cost of retryables: ${formatEther(estimatedCostOfRetryables)} ${nativeCurrency.symbol}`
      )
    }

    // Fund owner for gas fees
    const ethBalance = await settlementPublicClient.getBalance({
      address: ownerSettlementWalletClient.account.address
    })
    if (ethBalance < parseEther("0.001")) {
      await fundAccount(
        deployerSettlementWalletClient,
        ownerSettlementWalletClient.account.address,
        parseEther("0.001")
      )
      await sleep(1000)
    }
  }

  const interval = pollEmptyTxs(deployerSequencingWalletClient)
  const { tokenBridgeContracts } = await createTokenBridge({
    tokenBridgeCreatorAddressOverride:
      supportedSettlementChains[settlementPublicClient.chain.id]
        .tokenBridgeCreatorAddress,
    rollupOwner: ownerSettlementWalletClient.account.address,
    rollupAddress: coreContracts.rollup,
    rollupDeploymentBlockNumber: BigInt(coreContracts.deployedAtBlockNumber),
    account: ownerSettlementWalletClient.account,
    nativeTokenAddress: coreContracts.nativeToken,
    parentChainPublicClient: settlementPublicClient,
    orbitChainPublicClient: appchainPublicClient,
    retryableGasOverrides: {
      maxSubmissionCostForFactory: { percentIncrease: BigInt(100) },
      maxGasForFactory: { percentIncrease: BigInt(100) },
      maxSubmissionCostForContracts: { percentIncrease: BigInt(100) },
      maxGasForContracts: { percentIncrease: BigInt(100) }
    },
    setWethGatewayGasOverrides: {
      gasLimit: { base: BigInt(100000), percentIncrease: BigInt(100) },
      maxFeePerGas: { percentIncrease: BigInt(100) },
      maxSubmissionCost: { percentIncrease: BigInt(100) }
    }
  })

  clearInterval(interval)
  print("✅ Token bridge successfully created")

  await upsertToSyndObject(
    chainName,
    environment,
    "bridge",
    generateBridgeConfig({
      explorerUrl: appchainPublicClient.chain.blockExplorers?.default.url,
      parentChainId: settlementPublicClient.chain.id,
      rpcUrl: getChainRpcUrl(appchainPublicClient.chain),
      tokenContracts: {
        l2Contracts: {
          ...tokenBridgeContracts.parentChainContracts,
          proxyAdmin: coreContracts.adminProxy
        },
        l3Contracts: tokenBridgeContracts.orbitChainContracts
      },
      coreContracts,
      chainName,
      chainId,
      rollupOwnerAddress: ownerSettlementWalletClient.account.address
    })
  )

  if (await canDeployMulticall3()) {
    const multicall3Address = await deployMulticall3()
    print(`✅ Multicall3 successfully deployed to: ${multicall3Address}`)
    await upsertToSyndObject(
      chainName,
      environment,
      "bridge.tokenBridgeContracts.l3Contracts.multicall",
      multicall3Address
    )
  } else {
    print("🔎  Skipping multicall3 deployment")
  }

  const {
    assertionPosterAddress,
    teeModuleAddress,
    attestationDocVerifierAddress,
    teeKeyManagerAddress
  } = await deployTeeModule()

  await upsertToSyndObject(chainName, environment, "withdrawals", {
    teeKeyManager: teeKeyManagerAddress,
    assertionPoster: assertionPosterAddress,
    teeModule: teeModuleAddress,
    attestationDocVerifier: attestationDocVerifierAddress
  })

  const proposerPrivateKey = generatePrivateKey()
  const proposerAccount = privateKeyToAccount(proposerPrivateKey)

  print("---------------------------------------------------------")
  print("                      Fund Accounts                      ")
  print("⚠️  Please confirm the following details before proceeding ⚠️")
  print("---------------------------------------------------------")
  print("Proposer", proposerAccount.address)
  print("Proposer PK", proposerPrivateKey)
  print("---------------------------------------------------------")
  const fundResponse = prompt(
    "Do you want to fund the following account? (y/n)"
  )
  const fundConfirmation = ["y", "yes"]
  if (!fundResponse || !fundConfirmation.includes(fundResponse.toLowerCase())) {
    print("🚫 Skipping funding...")
  } else {
    await fundAccount(
      deployerSettlementWalletClient,
      proposerAccount.address,
      parseEther("0.1")
    )
    await upsertToEoaSecrets(chainName, environment, "proposer", {
      address: proposerAccount.address,
      privateKey: proposerPrivateKey
    })
  }

  // Generate proposer config file
  await writeToFile(
    chainName,
    `${environment}-proposer.${chainName}.json`,
    JSON.stringify(
      {
        "assertion-poster-contract-address": assertionPosterAddress,
        "tee-module-address": teeModuleAddress,
        "proposer-private-key": proposerPrivateKey.replace("0x", "")
      },
      null,
      2
    )
  )

  print(
    `🏁  Features setup complete. Artifacts saved at:
    - ./out/${chainName}/${environment}-proposer.${chainName}.json`
  )
}

await main()
  .then(() => {
    process.exit(0)
  })
  .catch((error) => {
    console.error(error)
    process.exit(1)
  })
