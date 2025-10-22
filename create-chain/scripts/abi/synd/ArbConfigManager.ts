export const ArbConfigManagerABI = [
  {
    type: "constructor",
    inputs: [
      {
        name: "owner_",
        type: "address",
        internalType: "address"
      }
    ],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "beacon",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "contract UpgradeableBeacon"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "createArbChainConfig",
    inputs: [
      {
        name: "owner",
        type: "address",
        internalType: "address"
      },
      {
        name: "chainId",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "sequencingChainId",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "arbitrumBridgeAddress",
        type: "address",
        internalType: "address"
      },
      {
        name: "arbitrumInboxAddress",
        type: "address",
        internalType: "address"
      },
      {
        name: "settlementDelay",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "settlementStartBlock",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "sequencingContractAddress",
        type: "address",
        internalType: "address"
      },
      {
        name: "sequencingStartBlock",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "initialAppchainOwner",
        type: "address",
        internalType: "address"
      },
      {
        name: "sequencingChainRpcUrl",
        type: "string",
        internalType: "string"
      },
      {
        name: "appchainBlockExplorerUrl",
        type: "string",
        internalType: "string"
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
    name: "deployedConfigs",
    inputs: [
      {
        name: "chainId",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    outputs: [
      {
        name: "deployedProxyAddress",
        type: "address",
        internalType: "address"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "getArbChainConfigAddress",
    inputs: [
      {
        name: "chainId",
        type: "uint256",
        internalType: "uint256"
      }
    ],
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
    name: "owner",
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
    name: "renounceOwnership",
    inputs: [],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "transferOwnership",
    inputs: [
      {
        name: "newOwner",
        type: "address",
        internalType: "address"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "upgradeImplementation",
    inputs: [
      {
        name: "newImplementation",
        type: "address",
        internalType: "address"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "event",
    name: "ArbChainConfigCreated",
    inputs: [
      {
        name: "chainId",
        type: "uint256",
        indexed: true,
        internalType: "uint256"
      },
      {
        name: "configAddress",
        type: "address",
        indexed: false,
        internalType: "address"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "ImplementationUpgraded",
    inputs: [
      {
        name: "newImplementation",
        type: "address",
        indexed: false,
        internalType: "address"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "OwnershipTransferred",
    inputs: [
      {
        name: "previousOwner",
        type: "address",
        indexed: true,
        internalType: "address"
      },
      {
        name: "newOwner",
        type: "address",
        indexed: true,
        internalType: "address"
      }
    ],
    anonymous: false
  },
  {
    type: "error",
    name: "OwnableInvalidOwner",
    inputs: [
      {
        name: "owner",
        type: "address",
        internalType: "address"
      }
    ]
  },
  {
    type: "error",
    name: "OwnableUnauthorizedAccount",
    inputs: [
      {
        name: "account",
        type: "address",
        internalType: "address"
      }
    ]
  }
] as const
