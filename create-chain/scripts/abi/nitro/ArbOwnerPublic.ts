export const ArbOwnerPublicABI = [
  {
    type: "function",
    name: "getAllChainOwners",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address[]",
        internalType: "address[]"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "getBrotliCompressionLevel",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "uint64",
        internalType: "uint64"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "getInfraFeeAccount",
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
    name: "getNetworkFeeAccount",
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
    name: "getScheduledUpgrade",
    inputs: [],
    outputs: [
      {
        name: "arbosVersion",
        type: "uint64",
        internalType: "uint64"
      },
      {
        name: "scheduledForTimestamp",
        type: "uint64",
        internalType: "uint64"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "isCalldataPriceIncreaseEnabled",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "bool",
        internalType: "bool"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "isChainOwner",
    inputs: [
      {
        name: "addr",
        type: "address",
        internalType: "address"
      }
    ],
    outputs: [
      {
        name: "",
        type: "bool",
        internalType: "bool"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "rectifyChainOwner",
    inputs: [
      {
        name: "ownerToRectify",
        type: "address",
        internalType: "address"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "event",
    name: "ChainOwnerRectified",
    inputs: [
      {
        name: "rectifiedOwner",
        type: "address",
        indexed: false,
        internalType: "address"
      }
    ],
    anonymous: false
  }
] as const
