export const syndForwarderABI = [
  {
    type: "constructor",
    inputs: [
      {
        name: "_sourceSender",
        type: "address",
        internalType: "address"
      },
      {
        name: "_sourceChainId",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "allowedSender",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "address"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "call",
    inputs: [
      {
        name: "dest",
        type: "address",
        internalType: "address"
      },
      {
        name: "data",
        type: "bytes",
        internalType: "bytes"
      }
    ],
    outputs: [],
    stateMutability: "payable"
  },
  {
    type: "function",
    name: "deploy",
    inputs: [
      {
        name: "salt",
        type: "bytes32",
        internalType: "bytes32"
      },
      {
        name: "impl",
        type: "address",
        internalType: "address"
      },
      {
        name: "init",
        type: "bytes",
        internalType: "bytes"
      }
    ],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "address"
      }
    ],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "getProxyBytecode",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "bytes",
        internalType: "bytes"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "stubImplementation",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "address"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "error",
    name: "Create2EmptyBytecode",
    inputs: []
  },
  {
    type: "error",
    name: "FailedDeployment",
    inputs: []
  },
  {
    type: "error",
    name: "InsufficientBalance",
    inputs: [
      {
        name: "balance",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "needed",
        type: "uint256",
        internalType: "uint256"
      }
    ]
  },
  {
    type: "error",
    name: "NotAllowedSender",
    inputs: []
  }
] as const
