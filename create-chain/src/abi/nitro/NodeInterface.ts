export const NodeInterfaceABI = [
	{
		inputs: [
			{
				internalType: "address",
				name: "sender",
				type: "address",
			},
			{
				internalType: "uint256",
				name: "deposit",
				type: "uint256",
			},
			{
				internalType: "address",
				name: "to",
				type: "address",
			},
			{
				internalType: "uint256",
				name: "l2CallValue",
				type: "uint256",
			},
			{
				internalType: "address",
				name: "excessFeeRefundAddress",
				type: "address",
			},
			{
				internalType: "address",
				name: "callValueRefundAddress",
				type: "address",
			},
			{
				internalType: "bytes",
				name: "data",
				type: "bytes",
			},
		],
		name: "estimateRetryableTicket",
		outputs: [],
		stateMutability: "nonpayable",
		type: "function",
	},
] as const;
