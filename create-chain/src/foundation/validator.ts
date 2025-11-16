import type { CoreContracts } from "@arbitrum/orbit-sdk"
import {
  getValidators as getValidatorsFromOrbitSdk,
  rollupAdminLogicPublicActions
} from "@arbitrum/orbit-sdk"
import { getFoundationConfig } from "../utils/config"

export async function getValidators(coreContracts: CoreContracts) {
  const { settlementPublicClient: parentChainClient } =
    await getFoundationConfig()
  const extendedClient = parentChainClient.extend(
    rollupAdminLogicPublicActions({
      rollup: coreContracts.rollup
    })
  )

  return getValidatorsFromOrbitSdk(extendedClient, {
    rollup: coreContracts.rollup
  })
}
