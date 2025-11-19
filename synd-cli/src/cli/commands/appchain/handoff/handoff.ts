import type { Handoff } from "@/types"
import { print } from "@/utils/print"
import handoffNitro from "./handoffNitro"
import handoffSynd from "./handoffSynd"
import setAppchainConfig from "./setAppchainConfig"

export async function handoff(params: Handoff) {
  const {
    newOwnerAddress,
    ownerSettlementWalletClient,
    settlementPublicClient,
    sequencingPublicClient,
    appchainPublicClient,
    ownerAppchainWalletClient,
    synd
  } = params

  print("              HANDOFF TO NEW OWNER              ")
  print("⚠️  Please confirm the following details before proceeding ⚠️")
  print("---------------------------------------------------------")

  print("Current Owner", ownerSettlementWalletClient.account.address)
  print("New Owner", newOwnerAddress)
  print("Settlement Chain", settlementPublicClient.chain.name)
  print("Sequencing Chain", sequencingPublicClient.chain.name)
  print("Appchain Chain", appchainPublicClient.chain.name)
  print("---------------------------------------------------------")

  const promptResponse = prompt("Are you sure you want to proceed? (y/n)")
  const confirmation = ["y", "yes"]
  if (!promptResponse || !confirmation.includes(promptResponse.toLowerCase())) {
    print("🚫 Exiting...")
    return
  }

  await setAppchainConfig({
    appchainPublicClient,
    ownerAppchainWalletClient,
    newOwnerAddress
  })
  await handoffNitro({
    newOwnerAddress,
    ownerSettlementWalletClient,
    settlementPublicClient,
    synd,
    ownerAppchainWalletClient,
    appchainPublicClient
  })
  await handoffSynd(params)
  print(`🏁  Handoff complete: Ownership transferred to ${newOwnerAddress}`)
}
