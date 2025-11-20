import type { PrivateKeyWalletAccount, PublicClientWithChain } from "@/types"
import type { Hex } from "viem"
import { createWalletClient, http } from "viem"
import { privateKeyToAccount } from "viem/accounts"
import { getAppchainClient, getPublicClient, getWalletClient } from "./clients"
import {
  supportedEthereumChains,
  supportedSequencingChains,
  supportedSettlementChains
} from "./constants"
import { getChainRpcUrl } from "./helpers"

interface BaseClientOptions {
  settlementRpc?: string
  sequencingRpc?: string
  ethereumRpc?: string
  appchainRpc?: string
  ownerPrivateKey?: Hex
  deployerPrivateKey?: Hex
}

interface AppchainOptions {
  chainName?: string
  nativeToken?: Hex
  appchainExplorer?: string
}

type ClientOptions = BaseClientOptions & AppchainOptions

// Type-safe return type based on what's provided in options
type CreatedClients<T extends ClientOptions> = {
  settlementPublicClient: T["settlementRpc"] extends string
    ? PublicClientWithChain
    : undefined
  sequencingPublicClient: T["sequencingRpc"] extends string
    ? PublicClientWithChain
    : undefined
  ethereumPublicClient: T["ethereumRpc"] extends string
    ? PublicClientWithChain
    : undefined
  appchainPublicClient: undefined extends T["appchainRpc"]
    ? PublicClientWithChain | undefined
    : T["appchainRpc"] extends string
      ? PublicClientWithChain
      : undefined
  ownerSettlementWalletClient: T["settlementRpc"] extends string
    ? T["ownerPrivateKey"] extends Hex
      ? PrivateKeyWalletAccount
      : undefined
    : undefined
  deployerSettlementWalletClient: T["settlementRpc"] extends string
    ? T["deployerPrivateKey"] extends Hex
      ? PrivateKeyWalletAccount
      : undefined
    : undefined
  ownerSequencingWalletClient: T["sequencingRpc"] extends string
    ? T["ownerPrivateKey"] extends Hex
      ? PrivateKeyWalletAccount
      : undefined
    : undefined
  deployerSequencingWalletClient: T["sequencingRpc"] extends string
    ? T["deployerPrivateKey"] extends Hex
      ? PrivateKeyWalletAccount
      : undefined
    : undefined
  ownerAppchainWalletClient: undefined extends T["appchainRpc"]
    ? T["ownerPrivateKey"] extends Hex
      ? PrivateKeyWalletAccount | undefined
      : undefined
    : T["appchainRpc"] extends string
      ? T["ownerPrivateKey"] extends Hex
        ? PrivateKeyWalletAccount
        : undefined
      : undefined
  deployerAppchainWalletClient: undefined extends T["appchainRpc"]
    ? T["deployerPrivateKey"] extends Hex
      ? PrivateKeyWalletAccount | undefined
      : undefined
    : T["appchainRpc"] extends string
      ? T["deployerPrivateKey"] extends Hex
        ? PrivateKeyWalletAccount
        : undefined
      : undefined
}

export async function createClients<T extends ClientOptions>(
  options: T
): Promise<CreatedClients<T>> {
  // biome-ignore lint/suspicious/noExplicitAny: conditional logic here makes typing difficult
  const clients = {} as Record<string, any>
  const promises: Promise<void>[] = []

  // Settlement chain clients
  if (options.settlementRpc) {
    promises.push(
      getPublicClient(options.settlementRpc, supportedSettlementChains).then(
        (client) => {
          clients.settlementPublicClient = client
        }
      )
    )

    if (options.ownerPrivateKey) {
      promises.push(
        getWalletClient(
          options.settlementRpc,
          supportedSettlementChains,
          options.ownerPrivateKey
        ).then((client) => {
          clients.ownerSettlementWalletClient = client
        })
      )
    }

    if (options.deployerPrivateKey) {
      promises.push(
        getWalletClient(
          options.settlementRpc,
          supportedSettlementChains,
          options.deployerPrivateKey
        ).then((client) => {
          clients.deployerSettlementWalletClient = client
        })
      )
    }
  }

  // Sequencing chain clients
  if (options.sequencingRpc) {
    promises.push(
      getPublicClient(options.sequencingRpc, supportedSequencingChains).then(
        (client) => {
          clients.sequencingPublicClient = client
        }
      )
    )

    if (options.ownerPrivateKey) {
      promises.push(
        getWalletClient(
          options.sequencingRpc,
          supportedSequencingChains,
          options.ownerPrivateKey
        ).then((client) => {
          clients.ownerSequencingWalletClient = client
        })
      )
    }

    if (options.deployerPrivateKey) {
      promises.push(
        getWalletClient(
          options.sequencingRpc,
          supportedSequencingChains,
          options.deployerPrivateKey
        ).then((client) => {
          clients.deployerSequencingWalletClient = client
        })
      )
    }
  }

  // Ethereum chain clients
  if (options.ethereumRpc) {
    promises.push(
      getPublicClient(options.ethereumRpc, supportedEthereumChains).then(
        (client) => {
          clients.ethereumPublicClient = client
        }
      )
    )
  }

  // Wait for all basic clients to be created first
  await Promise.all(promises)

  // Appchain clients (requires settlement client for some cases)
  if (options.appchainRpc) {
    if (clients.settlementPublicClient) {
      clients.appchainPublicClient = await getAppchainClient({
        chainName: options.chainName,
        nativeToken: options.nativeToken,
        rpcUrl: options.appchainRpc,
        explorerUrl: options.appchainExplorer,
        settlementPublicClient: clients.settlementPublicClient
      })

      // Create appchain wallet clients if appchain client exists
      if (options.ownerPrivateKey && clients.appchainPublicClient) {
        clients.ownerAppchainWalletClient = createWalletClient({
          chain: clients.appchainPublicClient.chain,
          account: privateKeyToAccount(options.ownerPrivateKey),
          transport: http(getChainRpcUrl(clients.appchainPublicClient.chain))
        })
      }

      if (options.deployerPrivateKey && clients.appchainPublicClient) {
        clients.deployerAppchainWalletClient = createWalletClient({
          chain: clients.appchainPublicClient.chain,
          account: privateKeyToAccount(options.deployerPrivateKey),
          transport: http(getChainRpcUrl(clients.appchainPublicClient.chain))
        })
      }
    }
  }

  return clients as CreatedClients<T>
}
