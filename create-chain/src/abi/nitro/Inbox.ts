import { parseAbi } from "viem"

export const InboxABI = parseAbi([
  "function createRetryableTicket(address to, uint256 l2CallValue, uint256 maxSubmissionCost, address excessFeeRefundAddress, address callValueRefundAddress, uint256 gasLimit, uint256 maxFeePerGas, bytes calldata data) external payable returns (uint256)",
  "function sendL1FundedContractTransaction(uint256 gasLimit, uint256 maxFeePerGas, address to, bytes calldata data) external payable returns (uint256)",
  "function calculateRetryableSubmissionFee(uint256 dataLength, uint256 baseFee) public view returns (uint256)"
])
