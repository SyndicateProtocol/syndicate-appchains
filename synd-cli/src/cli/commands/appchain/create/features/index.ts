import {
  appchainCreateFeaturesOptionsSchema,
  handleSchemaErrors
} from "@/cli/schema"
import {
  getAppchainClient,
  getChainIdFromRpc,
  getPublicClient,
  getWalletClient
} from "@/utils/clients"
import {
  supportedEthereumChains,
  supportedSequencingChains,
  supportedSettlementChains
} from "@/utils/constants"
import type { Command } from "@commander-js/extra-typings"
import { createWalletClient, http } from "viem"
import { privateKeyToAccount } from "viem/accounts"
import { features } from "./features"

export function createFeaturesCommand(program: Command) {
  program
    .command("features")
    .description(
      "Deploy features for a new appchain. The foundation must be deployed first. Nitro token bridge, Syndicate withdrawals, multicall3"
    )
    .requiredOption(
      "--owner-private-key <key>",
      "Private key of the owner account"
    )
    .requiredOption(
      "--deployer-private-key <key>",
      "Private key of the deployer account"
    )
    .requiredOption("--chain-name <name>", "Name of the appchain")
    .requiredOption(
      "--settlement-rpc <url>",
      "RPC URL for the settlement chain"
    )
    .requiredOption(
      "--sequencing-rpc <url>",
      "RPC URL for the sequencing chain"
    )
    .requiredOption(
      "--synd-fork-sequencing-rpc <url>",
      "RPC URL for the synd fork sequencing chain"
    )
    .requiredOption("--ethereum-rpc <url>", "RPC URL for Ethereum")
    .requiredOption("--appchain-rpc <url>", "RPC URL for the appchain")
    .requiredOption(
      "--appchain-explorer <url>",
      "Explorer URL for the appchain"
    )
    .requiredOption(
      "--sequencing-contract-address <address>",
      "Address of the sequencing contract"
    )
    .requiredOption(
      "--core-contracts <contracts>",
      "Core contracts for the appchain"
    )
    .action(async (options: Record<string, unknown>) => {
      const {
        data: validatedOptions,
        success,
        error
      } = appchainCreateFeaturesOptionsSchema.safeParse(options)

      if (!success) {
        return handleSchemaErrors(error)
      }

      const {
        settlementRpc,
        sequencingRpc,
        ethereumRpc,
        deployerPrivateKey,
        ownerPrivateKey,
        coreContracts,
        appchainRpc,
        appchainExplorer,
        chainName
      } = validatedOptions

      const [
        settlementPublicClient,
        sequencingPublicClient,
        ethereumPublicClient,
        ownerSettlementWalletClient,
        deployerSettlementWalletClient,
        deployerSequencingWalletClient
      ] = await Promise.all([
        getPublicClient(settlementRpc, supportedSettlementChains),
        getPublicClient(sequencingRpc, supportedSequencingChains),
        getPublicClient(ethereumRpc, supportedEthereumChains),
        getWalletClient(
          settlementRpc,
          supportedSettlementChains,
          ownerPrivateKey
        ),
        getWalletClient(
          settlementRpc,
          supportedSettlementChains,
          deployerPrivateKey
        ),
        getWalletClient(
          sequencingRpc,
          supportedSequencingChains,
          deployerPrivateKey
        )
      ])

      const appchainPublicClient = await getAppchainClient({
        chainName,
        nativeToken: coreContracts.nativeToken,
        settlementPublicClient: settlementPublicClient,
        rpcUrl: appchainRpc,
        explorerUrl: appchainExplorer
      })

      const deployerAppchainWalletClient = createWalletClient({
        chain: appchainPublicClient.chain,
        account: privateKeyToAccount(deployerPrivateKey),
        transport: http(appchainRpc)
      })
      const chainId = await getChainIdFromRpc(appchainRpc)

      await features({
        ...validatedOptions,
        chainId,
        appchainPublicClient,
        deployerSequencingWalletClient,
        ownerSettlementWalletClient,
        deployerSettlementWalletClient,
        settlementPublicClient,
        deployerAppchainWalletClient,
        sequencingPublicClient,
        ethereumPublicClient
      })
    })
}
