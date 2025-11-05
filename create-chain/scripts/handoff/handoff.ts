import { formatEther, parseEther } from "viem"
import { getHandoffConfig } from "../utils/config"
import { print } from "../utils/print"
import handoffNitro from "./handoffNitro"
import handoffSynd from "./handoffSynd"
import setAppchainConfig from "./setAppchainConfig"

export default async function handoff() {
  const {
    newOwnerAddress,
    ownerSettlementWalletClient,
    settlementPublicClient,
    sequencingPublicClient,
    appchainPublicClient
  } = await getHandoffConfig()

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

  await checkOwnerBalance()
  await setAppchainConfig()
  await handoffNitro()
  await handoffSynd()
  print(`🏁  Handoff complete: Ownership transferred to ${newOwnerAddress}`)
}

async function checkOwnerBalance() {
  const {
    ownerSettlementWalletClient,
    settlementPublicClient,
    sequencingPublicClient,
    ownerSequencingWalletClient,
    ownerAppchainWalletClient,
    appchainPublicClient
  } = await getHandoffConfig()
  const balanceThreshold = parseEther("0.001")
  const [appchainBalance, sequencingBalance, settlementBalance] =
    await Promise.all([
      appchainPublicClient.getBalance({
        address: ownerAppchainWalletClient.account.address
      }),
      sequencingPublicClient.getBalance({
        address: ownerSequencingWalletClient.account.address
      }),
      settlementPublicClient.getBalance({
        address: ownerSettlementWalletClient.account.address
      })
    ])

  if (appchainBalance < balanceThreshold) {
    throw new Error(
      `Owner balance on the appchain: ${formatEther(appchainBalance)} is less than the threshold: ${formatEther(balanceThreshold)}`
    )
  }
  if (sequencingBalance < balanceThreshold) {
    throw new Error(
      `Owner balance on the sequencing chain: ${formatEther(sequencingBalance)} is less than the threshold: ${formatEther(balanceThreshold)}`
    )
  }
  if (settlementBalance < balanceThreshold) {
    throw new Error(
      `Owner balance on the settlement chain: ${formatEther(settlementBalance)} is less than the threshold: ${formatEther(balanceThreshold)}`
    )
  }
}

handoff()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error("🚫", err)
    process.exit(1)
  })
