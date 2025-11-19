import {
  appchainDeployTeeModuleOptionsSchema,
  handleSchemaErrors
} from "@/cli/schema"
import {
  getAppchainClient,
  getPublicClient,
  getWalletClient
} from "@/utils/clients"
import {
  supportedEthereumChains,
  supportedSequencingChains,
  supportedSettlementChains
} from "@/utils/constants"
import type { Command } from "@commander-js/extra-typings"
import { type Hex, zeroAddress } from "viem"
import { getNativeTokenFromBridge } from "../arbOwner/helpers"
import { deployTeeModule } from "./features/deployTeeModule"

export function createTeeModuleCommand(program: Command) {
  program
    .command("tee-module")
    .description("Deploy a new TeeModule contract")
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
      "--deployer-private-key <key>",
      "Private key of the deployer account"
    )
    .requiredOption(
      "--sequencing-contract <address>",
      "Address of the sequencing contract"
    )
    .requiredOption(
      "--assertion-poster <address>",
      "Address of the AssertionPoster contract"
    )
    .requiredOption("--bridge <address>", "Address of the bridge contract")
    .action(async (options: Record<string, unknown>) => {
      const {
        data: validatedOptions,
        success,
        error
      } = appchainDeployTeeModuleOptionsSchema.safeParse(options)

      if (!success) {
        return handleSchemaErrors(error)
      }

      const {
        settlementRpc,
        sequencingRpc,
        ethereumRpc,
        deployerPrivateKey,
        bridge,
        appchainRpc,
        assertionPoster,
        sequencingContract,
        syndForkSequencingRpc
      } = validatedOptions

      const [
        settlementPublicClient,
        sequencingPublicClient,
        ethereumPublicClient,
        deployerSettlementWalletClient
      ] = await Promise.all([
        getPublicClient(settlementRpc, supportedSettlementChains),
        getPublicClient(sequencingRpc, supportedSequencingChains),
        getPublicClient(ethereumRpc, supportedEthereumChains),
        getWalletClient(
          settlementRpc,
          supportedSettlementChains,
          deployerPrivateKey as `0x${string}`
        )
      ])

      const customNativeToken = await getNativeTokenFromBridge(
        settlementPublicClient,
        bridge
      )

      const appchainPublicClient = await getAppchainClient({
        nativeToken: customNativeToken?.address ?? zeroAddress,
        settlementPublicClient: settlementPublicClient,
        rpcUrl: appchainRpc
      })

      const teeModuleAddress = await deployTeeModule({
        assertionPosterAddress: assertionPoster,
        bridge,
        deployerSettlementWalletClient,
        settlementPublicClient,
        sequencingContractAddress: sequencingContract as Hex,
        sequencingPublicClient,
        appchainPublicClient,
        ethereumPublicClient,
        syndForkSequencingRpc
      })

      console.log(`\nTeeModule deployed at: ${teeModuleAddress}`)
      console.log(
        "\nNext steps:",
        "\n1. Transfer AssertionPoster ownership to TeeModule",
        "\n2. Set TeeModule DEFAULT_ADMIN_ROLE to the desired owner",
        "\n3. Revoke DEFAULT_ADMIN_ROLE from deployer"
      )
    })
}
