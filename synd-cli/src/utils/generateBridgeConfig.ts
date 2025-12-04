import { zeroAddress } from "viem"

import type { GenerateBridgeConfig } from "@/types"
import { DEFAULT_APPCHAIN_MIN_BASE_FEE } from "./constants"

export function generateBridgeConfig({
  coreContracts,
  tokenContracts,
  rpcUrl,
  explorerUrl,
  parentChainId,
  chainName,
  chainId,
  chainOwner
}: GenerateBridgeConfig) {
  const bridgeConfig = {
    chainInfo: {
      chainName,
      chainId,
      chainOwner,
      minL2BaseFee: DEFAULT_APPCHAIN_MIN_BASE_FEE,
      parentChainId: Number(parentChainId),
      nativeToken: coreContracts.nativeToken ?? zeroAddress,
      staker: "",
      batchPoster: "",
      networkFeeReceiver: "",
      infrastructureFeeCollector: "",
      rpcUrl,
      explorerUrl
    },
    coreContracts,
    tokenBridgeContracts: tokenContracts || {
      l2Contracts: {
        weth: zeroAddress,
        router: zeroAddress,
        multicall: zeroAddress,
        proxyAdmin: zeroAddress,
        wethGateway: zeroAddress,
        customGateway: zeroAddress,
        standardGateway: zeroAddress
      },
      l3Contracts: {
        weth: zeroAddress,
        router: zeroAddress,
        multicall: zeroAddress,
        proxyAdmin: zeroAddress,
        wethGateway: zeroAddress,
        customGateway: zeroAddress,
        standardGateway: zeroAddress,
        upgradeExecutor: zeroAddress,
        beaconProxyFactory: zeroAddress
      }
    }
  }
  return bridgeConfig
}
