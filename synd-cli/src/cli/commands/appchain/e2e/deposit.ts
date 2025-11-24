import type { Deposit } from "@/types"
import { print } from "@/utils/print"
import { sleep } from "bun"
import { formatEther, type Hex, parseAbiItem } from "viem"

export async function deposit({
  settlementPublicClient,
  settlementWalletClient,
  appchainPublicClient,
  inbox,
  account,
  value
}: Deposit) {
  // 1: Check if the native token is ETH or and ERC20
  const bridgeAddress = await settlementPublicClient.readContract({
    address: inbox,
    abi: [parseAbiItem("function bridge() public view returns (address)")],
    functionName: "bridge"
  })

  const nativeTokenAddress = await settlementPublicClient
    .readContract({
      address: bridgeAddress,
      abi: [
        parseAbiItem("function nativeToken() public view returns (address)")
      ],
      functionName: "nativeToken"
    })
    .catch(() => {
      return null
    })

  // 2: Deposit ETH or ERC20
  let bridgeHash: Hex
  const l3BalanceBefore = await appchainPublicClient.getBalance({
    address: account.address
  })
  if (nativeTokenAddress) {
    // 2.1: Approve the inbox to spend your ERC20
    const { request: approveRequest } =
      await settlementPublicClient.simulateContract({
        account,
        address: nativeTokenAddress,
        abi: [
          parseAbiItem(
            "function approve(address,uint256) public returns (bool)"
          )
        ],
        functionName: "approve",
        args: [inbox, value]
      })
    const approvalHash =
      await settlementWalletClient.writeContract(approveRequest)
    await settlementPublicClient.waitForTransactionReceipt({
      hash: approvalHash
    })

    print(`Approved ${formatEther(value)} ${nativeTokenAddress}`)
    // 2.2: Deposit ERC20
    const { request } = await settlementPublicClient.simulateContract({
      account,
      address: inbox,
      abi: [
        parseAbiItem("function depositERC20(uint256) public returns (uint256)")
      ],
      functionName: "depositERC20",
      args: [value]
    })
    bridgeHash = await settlementWalletClient.writeContract(request)
  } else {
    // 2.3: Deposit ETH
    const { request } = await settlementPublicClient.simulateContract({
      account,
      address: inbox,
      abi: [
        parseAbiItem("function depositEth() public payable returns (uint256)")
      ],
      functionName: "depositEth",
      value
    })
    bridgeHash = await settlementWalletClient.writeContract(request)
  }

  await settlementPublicClient.waitForTransactionReceipt({ hash: bridgeHash })
  print(
    `Deposited ${formatEther(value)} ${nativeTokenAddress || "ETH"} at hash: ${bridgeHash}`
  )

  print(
    `Appchain balance before deposit: ${formatEther(l3BalanceBefore)} ${nativeTokenAddress || "ETH"}`
  )

  // Poll L3 balance until it increases by deposited amount
  let l3BalanceAfter = l3BalanceBefore
  while (l3BalanceAfter <= l3BalanceBefore) {
    await sleep(2_500)
    l3BalanceAfter = await appchainPublicClient.getBalance({
      address: account.address
    })
    print(
      "Waiting for deposit to arrive on appchain, you may need to send a tx on the sequencing chain..."
    )
  }
  print(
    `Deposit confirmed, balance before: ${formatEther(l3BalanceBefore)}, balance now: ${formatEther(l3BalanceAfter)}`
  )
}
