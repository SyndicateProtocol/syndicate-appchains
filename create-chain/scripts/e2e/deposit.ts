import { sleep } from "bun"
import {
  type Account,
  type Address,
  type Hex,
  type PublicClient,
  type WalletClient,
  formatEther,
  parseAbiItem
} from "viem"
import { print } from "../utils/print"

interface DepositConfig {
  settlementClient: PublicClient
  settlementWalletClient: WalletClient
  l3Client: PublicClient
  inboxAddress: Address
  account: Account
  value: bigint
}

export async function deposit({
  settlementClient,
  settlementWalletClient,
  l3Client,
  inboxAddress,
  account,
  value
}: DepositConfig) {
  // 1: Check if the native token is ETH or and ERC20
  const bridgeAddress = await settlementClient.readContract({
    address: inboxAddress,
    abi: [parseAbiItem("function bridge() public view returns (address)")],
    functionName: "bridge"
  })

  const nativeTokenAddress = await settlementClient
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
  const l3BalanceBefore = await l3Client.getBalance({
    address: account.address
  })
  if (nativeTokenAddress) {
    // 2.1: Approve the inbox to spend your ERC20
    const { request: approveRequest } = await settlementClient.simulateContract(
      {
        account,
        address: nativeTokenAddress,
        abi: [
          parseAbiItem(
            "function approve(address,uint256) public returns (bool)"
          )
        ],
        functionName: "approve",
        args: [inboxAddress, value]
      }
    )
    const approvalHash =
      await settlementWalletClient.writeContract(approveRequest)
    await settlementClient.waitForTransactionReceipt({ hash: approvalHash })

    print(`Approved ${formatEther(value)} ${nativeTokenAddress}`)
    // 2.2: Deposit ERC20
    const { request } = await settlementClient.simulateContract({
      account,
      address: inboxAddress,
      abi: [
        parseAbiItem("function depositERC20(uint256) public returns (uint256)")
      ],
      functionName: "depositERC20",
      args: [value]
    })
    bridgeHash = await settlementWalletClient.writeContract(request)
  } else {
    // 2.3: Deposit ETH
    const { request } = await settlementClient.simulateContract({
      account,
      address: inboxAddress,
      abi: [
        parseAbiItem("function depositEth() public payable returns (uint256)")
      ],
      functionName: "depositEth",
      value
    })
    bridgeHash = await settlementWalletClient.writeContract(request)
  }

  await settlementClient.waitForTransactionReceipt({ hash: bridgeHash })
  print(
    `Deposited ${formatEther(value)} ${nativeTokenAddress || "ETH"} at hash: ${bridgeHash}`
  )

  print(
    `L3 balance before deposit: ${formatEther(l3BalanceBefore)} ${nativeTokenAddress || "ETH"}`
  )

  // Poll L3 balance until it increases by deposited amount
  let l3BalanceAfter = l3BalanceBefore
  while (l3BalanceAfter <= l3BalanceBefore) {
    await sleep(2_500)
    l3BalanceAfter = await l3Client.getBalance({
      address: account.address
    })
    print("Waiting for deposit to arrive on L3...")
  }
  print(
    `Deposit confirmed, balance before: ${formatEther(l3BalanceBefore)}, balance now: ${formatEther(l3BalanceAfter)}`
  )
}
