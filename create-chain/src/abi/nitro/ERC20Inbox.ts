import { parseAbi } from "viem"

export const ERC20InboxABI = parseAbi([
  "function createRetryableTicket(address to, uint256 l2CallValue, uint256 maxSubmissionCost, address excessFeeRefundAddress, address callValueRefundAddress, uint256 gasLimit, uint256 maxFeePerGas, uint256 tokenTotalFeeAmount, bytes calldata data) external returns (uint256)",
  "function unsafeCreateRetryableTicket(address to, uint256 l2CallValue, uint256 maxSubmissionCost, address excessFeeRefundAddress, address callValueRefundAddress, uint256 gasLimit, uint256 maxFeePerGas, uint256 tokenTotalFeeAmount, bytes calldata data) external returns (uint256)",
  "function calculateRetryableSubmissionFee(uint256 dataLength, uint256 baseFee) public pure returns (uint256)",
  "function bridge() external view returns (address)"
])
