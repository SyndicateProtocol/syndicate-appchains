import {
  type CoreContracts,
  getBatchPosters as getBatchPostersFromOrbitSdk,
  rollupAdminLogicPublicActions
} from "@arbitrum/orbit-sdk"
import { getFoundationConfig } from "../utils/config"

export async function getBatchPosters(coreContracts: CoreContracts) {
  const { settlementPublicClient: parentChainClient } =
    await getFoundationConfig()
  const extendedClient = parentChainClient.extend(
    rollupAdminLogicPublicActions({
      rollup: coreContracts.rollup
    })
  )

  return getBatchPostersFromOrbitSdk(extendedClient, {
    rollup: coreContracts.rollup,
    sequencerInbox: coreContracts.sequencerInbox
  })
}
