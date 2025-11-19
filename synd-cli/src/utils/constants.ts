import { defineChain } from "viem"
import { base, baseSepolia, mainnet, sepolia } from "viem/chains"
import type {
  SupportedEthereumChains,
  SupportedSequencingChains,
  SupportedSettlementChains
} from "../types"

export const risa = defineChain({
  id: 51014,
  name: "Risa Testnet",
  network: "risa-testnet",
  nativeCurrency: {
    name: "Testnet Syndicate",
    symbol: "TestnetSYND",
    decimals: 18
  },
  rpcUrls: {
    default: {
      http: ["https://risa-testnet.g.alchemy.com/public"]
    },
    public: {
      http: ["https://risa-testnet.g.alchemy.com/public"]
    }
  },
  blockExplorers: {
    default: {
      name: "Risa Testnet Explorer",
      url: "https://risa-testnet.explorer.alchemy.com"
    }
  },
  testnet: true
})

export const syndicate = defineChain({
  id: 510,
  name: "Syndicate Network",
  network: "syndicate-network",
  nativeCurrency: {
    name: "Syndicate",
    symbol: "SYND",
    decimals: 18
  },
  rpcUrls: {
    default: {
      http: ["https://synd-mainnet.g.alchemy.com/public"]
    },
    public: {
      http: ["https://synd-mainnet.g.alchemy.com/public"]
    }
  },
  blockExplorers: {
    default: {
      name: "Syndicate Network Explorer",
      url: "https://synd-mainnet.explorer.alchemy.com/"
    }
  },
  isTestnet: false
})

export const supportedSequencingChains: SupportedSequencingChains = {
  [risa.id]: {
    bridgeAddress: "0x1043E08195914c32ec3a4a075d9Eb2B0DC2fB1aA",
    requireAndFactoryAddress: "0x60e6Ac9FF8ff09175329EfB3daDa27abDA812aA4",
    syndicateFactoryAddress: "0x2e44cd104A6b67037b5e6DB662C0E917d1828D9E",
    chain: risa
  },
  [syndicate.id]: {
    bridgeAddress: "0x3C8cF0ae6E89AC0796f29B3a58e7dEa1cD072277",
    requireAndFactoryAddress: "0x3eEb8b1500cbaCbc4A3718D39414C8D191AC906B",
    syndicateFactoryAddress: "0x0620625c3662CbD6a8ca8Eef196ee3b10A8Bd157",
    chain: syndicate
  }
}

// rollupCreator deploys: v3.1.1 of nitro-core
// tokenBridgeCreator deploys: v1.2.5 of token bridge contracts
export const supportedSettlementChains: SupportedSettlementChains = {
  [baseSepolia.id]: {
    chain: baseSepolia,
    rollupCreatorAddress: "0x234ea0E2aB220f1c7e15B5e62Ec53e01e526e241",
    tokenBridgeCreatorAddress: "0x0369039c392E82AA8e4dB19B1149C486aB0c4698",
    arbConfigManagerAddress: "0xbb53E8736Cc018bb46D0F67A9d2Dbe3C3b306E92",
    teeKeyManagerAddress: "0x0831F5F32E424554E5742713B1AB3d5b9740eE19"
  },
  [sepolia.id]: {
    chain: sepolia,
    rollupCreatorAddress: "0x06d499101874a6260990AfcB5b41866Cc8BE6e08",
    tokenBridgeCreatorAddress: "0x8A4484fdE0D2f8675617897Ff1984fe5419DDD5E",
    arbConfigManagerAddress: "0xc18feFb2E79Ec35Ca9f3c3e7F1920EC1cad06e8F",
    teeKeyManagerAddress: "0xf02F6BC7b04930D3A4f71F105bBf84988568f187"
  },
  [base.id]: {
    chain: base,
    // @note TODO: update
    rollupCreatorAddress: "0x",
    tokenBridgeCreatorAddress: "0x882bCF9413885EFDAD307FD9fa369CC670304740",
    arbConfigManagerAddress: "0x65e6D336E311C92D1F19C66CfE68Ec6bE5b4f50B",
    teeKeyManagerAddress: "0x9CF9FF139C09Df70BD94b31ff935DFD648e0fa54"
  }
  // [mainnet.id]: {
  // chain: mainnet,
  // rollupCreatorAddress: "0x1AEdB6C4F5332a08251545066c686AD251c90bA2",
  // tokenBridgeCreatorAddress: "0x5f8aeF00194334B3033E2Ed97c48eCAeDcEFDCaE",
  // arbConfigManagerAddress: "0xec2ba05a9cFFcb86e3225F7A046bA9124419397C",
  // teeKeyManagerAddress: "0x0000000000000000000000000000000000000000", // @note TODO: to support mainnet as the settlement chain we need to deploy the TeeKeyManager to mainnet
  // },
}

export const supportedEthereumChains: SupportedEthereumChains = {
  [sepolia.id]: {
    chain: sepolia
  },
  [mainnet.id]: {
    chain: mainnet
  }
}

export const ARB_OWNER_PRECOMPILE_ADDRESS =
  "0x0000000000000000000000000000000000000070" as const
export const ARB_OWNER_PUBLIC_PRECOMPILE_ADDRESS =
  "0x000000000000000000000000000000000000006b" as const
export const ARB_GAS_INFO_ADDRESS =
  "0x000000000000000000000000000000000000006c" as const
export const NODE_INTERFACE_ADDRESS =
  "0x00000000000000000000000000000000000000C8" as const

export const DEFAULT_APPCHAIN_MIN_BASE_FEE = 1 // wei
