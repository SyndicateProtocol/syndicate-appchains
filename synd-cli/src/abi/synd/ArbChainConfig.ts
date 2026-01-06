export const arbChainConfigABI = [
  {
    type: "constructor",
    inputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "APPCHAIN_BLOCK_EXPLORER_URL",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "string",
        internalType: "string"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "ARBITRUM_BRIDGE_ADDRESS",
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
    name: "ARBITRUM_INBOX_ADDRESS",
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
    name: "CHAIN_ID",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "DEFAULT_SEQUENCING_CHAIN_WS_RPC_URL",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "string",
        internalType: "string"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "INITIAL_APPCHAIN_OWNER",
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
    name: "MIGRATED_APPCHAIN_BLOCK_HASH",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "MIGRATED_BATCH_ACC",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "MIGRATED_BATCH_COUNT",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "MIGRATED_DELAYED_MSGS_ACC",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "MIGRATED_DELAYED_MSGS_COUNT",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "MIGRATED_GENESIS_CONFIG",
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
    name: "SEQUENCING_CHAIN_ID",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "SEQUENCING_CONTRACT_ADDRESS",
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
    name: "SEQUENCING_START_BLOCK",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "SETTLEMENT_DELAY",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "SETTLEMENT_START_BLOCK",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "getArbChainConfigAddress",
    inputs: [
      {
        name: "",
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
    name: "initialize",
    inputs: [
      {
        name: "_owner",
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
        name: "sequencingChainWsRpcUrl",
        type: "string",
        internalType: "string"
      },
      {
        name: "appchainBlockExplorerUrl",
        type: "string",
        internalType: "string"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "migration",
    inputs: [
      {
        name: "_setStartBlock",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "_seqStartBlock",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "_batchAcc",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "_batchCount",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "_delayedMsgsAcc",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "_delayedMsgsCount",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "_appchainBlockHash",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "_genesisConfig",
        type: "bytes",
        internalType: "bytes"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
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
    name: "updateAppchainBlockExplorerUrl",
    inputs: [
      {
        name: "newUrl",
        type: "string",
        internalType: "string"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "updateDefaultSequencingChainWsRpcUrl",
    inputs: [
      {
        name: "newWsRpcUrl",
        type: "string",
        internalType: "string"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "event",
    name: "AppchainBlockExplorerUrlUpdated",
    inputs: [
      {
        name: "newUrl",
        type: "string",
        indexed: false,
        internalType: "string"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "DefaultSequencingChainWsRpcUrlUpdated",
    inputs: [
      {
        name: "newWsRpcUrl",
        type: "string",
        indexed: false,
        internalType: "string"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "Initialized",
    inputs: [
      {
        name: "version",
        type: "uint64",
        indexed: false,
        internalType: "uint64"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "Migration",
    inputs: [
      {
        name: "setStartBlock",
        type: "uint256",
        indexed: false,
        internalType: "uint256"
      },
      {
        name: "seqStartBlock",
        type: "uint256",
        indexed: false,
        internalType: "uint256"
      },
      {
        name: "batchAcc",
        type: "uint256",
        indexed: true,
        internalType: "uint256"
      },
      {
        name: "batchCount",
        type: "uint256",
        indexed: false,
        internalType: "uint256"
      },
      {
        name: "delayedMsgsAcc",
        type: "uint256",
        indexed: true,
        internalType: "uint256"
      },
      {
        name: "delayedMsgsCount",
        type: "uint256",
        indexed: false,
        internalType: "uint256"
      },
      {
        name: "appchainBlockHash",
        type: "uint256",
        indexed: true,
        internalType: "uint256"
      },
      {
        name: "genesisConfig",
        type: "bytes",
        indexed: false,
        internalType: "bytes"
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
    name: "InvalidInitialization",
    inputs: []
  },
  {
    type: "error",
    name: "NotInitializing",
    inputs: []
  }
] as const

export const arbChainConfigBytecode =
  "0x6080806040523460aa575f5160206118575f395f51905f525460ff8160401c16609b576002600160401b03196001600160401b038216016049575b6040516117a890816100af8239f35b6001600160401b0319166001600160401b039081175f5160206118575f395f51905f525581527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602090a15f80603a565b63f92ee8a960e01b5f5260045ffd5b5f80fdfe60806040526004361015610011575f80fd5b5f3560e01c80630743bf6e146113075780630b04ebfd146112ec57806318b5ce81146112b9578063290803561461114a5780634b8be3f714610fc957806357d1ba2514610fac5780636c31cc4014610f8f5780636edd6c0914610f5c57806385e1f4d014610f3f5780638703b40a14610f225780638da5cb5b14610ef0578063a3c6e1e714610ed3578063a52145f214610cb6578063aa6a43d814610c83578063b51646a614610c66578063bc8ff19c14610c49578063bf6db6f814610c16578063bf79fd1c1461030c578063c7a7609514610224578063d1f4737c14610207578063f0ecc3c5146101c5578063f2fde38b1461015f578063f8a144be146101425763f8acb81114610121575f80fd5b3461013e575f60031936011261013e576020600e54604051908152f35b5f80fd5b3461013e575f60031936011261013e576020600654604051908152f35b3461013e57602060031936011261013e576101c361017b611473565b61019d73ffffffffffffffffffffffffffffffffffffffff5f541633146115a9565b6101be73ffffffffffffffffffffffffffffffffffffffff821615156116bd565b611722565b005b3461013e575f60031936011261013e576102036040516101ef816101e8816114ec565b03826113fd565b604051918291602083526020830190611420565b0390f35b3461013e575f60031936011261013e576020600854604051908152f35b3461013e575f60031936011261013e576040515f600a54610244816113ac565b80845290600181169081156102ca575060011461026c575b610203836101ef818503826113fd565b600a5f9081527fc65a7bb8d6351c1cf70c95a316cc6a92839c986682d98bc35f958f4883f9d2a8939250905b8082106102b0575090915081016020016101ef61025c565b919260018160209254838588010152019101909291610298565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208086019190915291151560051b840190910191506101ef905061025c565b3461013e5761018060031936011261013e57610326611473565b602435604435916064359273ffffffffffffffffffffffffffffffffffffffff841680940361013e5760843573ffffffffffffffffffffffffffffffffffffffff811680910361013e5760e43573ffffffffffffffffffffffffffffffffffffffff811680910361013e57610124359173ffffffffffffffffffffffffffffffffffffffff831680930361013e576101443567ffffffffffffffff811161013e576103d5903690600401611496565b966101643567ffffffffffffffff811161013e576103f7903690600401611496565b947ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00549760ff8960401c16159867ffffffffffffffff811680159081610c0e575b6001149081610c04575b159081610bfb575b50610bd3578960017fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000008316177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0055610b7e575b5073ffffffffffffffffffffffffffffffffffffffff881615610b20578015610ac2578115610a3e5782156109ba5783156109365784156108b257851561082e576005556006557fffffffffffffffffffffffff000000000000000000000000000000000000000060025416176002557fffffffffffffffffffffffff0000000000000000000000000000000000000000600354161760035560a43560075560c4356008557fffffffffffffffffffffffff00000000000000000000000000000000000000006004541617600455610104356009557fffffffffffffffffffffffff00000000000000000000000000000000000000006001541617600155835167ffffffffffffffff8111610780576105c0816105bb600a546113ac565b611624565b602094601f82116001146107ad576105f19293949582915f926106ee575b50505f198260011b9260031b1c19161790565b600a555b80519067ffffffffffffffff82116107805761061b82610616600b546113ac565b61165e565b602090601f83116001146106f957918061064d9261065595945f926106ee5750505f198260011b9260031b1c19161790565b600b55611722565b61065b57005b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160018152a1005b0151905086806105de565b90601f19831691600b5f527f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9925f5b818110610768575091600193918561065597969410610750575b505050811b01600b55611722565b01515f1960f88460031b161c19169055858080610742565b92936020600181928786015181550195019301610728565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b601f19821695600a5f527fc65a7bb8d6351c1cf70c95a316cc6a92839c986682d98bc35f958f4883f9d2a8915f5b888110610816575083600195969798106107fe575b505050811b01600a556105f5565b01515f1960f88460031b161c191690558580806107f0565b919260206001819286850151815501940192016107db565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602d60248201527f496e697469616c20617070636861696e206f776e65722063616e6e6f7420626560448201527f207a65726f2061646472657373000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602a60248201527f53657175656e63696e6720636f6e747261637420616464726573732063616e6e60448201527f6f74206265207a65726f000000000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602560248201527f417262697472756d20696e626f7820616464726573732063616e6e6f7420626560448201527f207a65726f0000000000000000000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f417262697472756d2062726964676520616464726573732063616e6e6f74206260448201527f65207a65726f00000000000000000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f53657175656e63696e6720636861696e2049442063616e6e6f74206265207a6560448201527f726f0000000000000000000000000000000000000000000000000000000000006064820152fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f436861696e2049442063616e6e6f74206265207a65726f0000000000000000006044820152fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601c60248201527f4f776e65722063616e6e6f74206265207a65726f2061646472657373000000006044820152fd5b7fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000001668010000000000000001177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00558a61049d565b7ff92ee8a9000000000000000000000000000000000000000000000000000000005f5260045ffd5b9050158c61044a565b303b159150610442565b8b9150610438565b3461013e575f60031936011261013e57602073ffffffffffffffffffffffffffffffffffffffff60045416604051908152f35b3461013e575f60031936011261013e576020601054604051908152f35b3461013e575f60031936011261013e576020600d54604051908152f35b3461013e575f60031936011261013e57602073ffffffffffffffffffffffffffffffffffffffff60035416604051908152f35b3461013e5761010060031936011261013e5760e43567ffffffffffffffff811161013e57610ce8903690600401611445565b610d0a73ffffffffffffffffffffffffffffffffffffffff5f541633146115a9565b600435600855602435600955604435600c55606435600d55608435600e5560a435600f5560c43560105567ffffffffffffffff811161078057610d4e6011546113ac565b601f8111610e7a575b505f601f8211600114610dfb578190610d83935f92610df05750505f198260011b9260031b1c19161790565b6011555b60085460095490600c547f807e8088430ce7c504e4dfbb498f90ecd3e8b8657d633be82ac2bd92bb497932600d5492600e5493600f54906010549660405193845260208401526040830152606082015260a0608082015280610deb60a082016114ec565b0390a4005b0135905083806105de565b601f1982169260115f527f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c68915f5b858110610e6257508360019510610e49575b505050811b01601155610d87565b5f1960f88560031b161c19910135169055828080610e3b565b90926020600181928686013581550194019101610e29565b60115f52610ec3907f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c68601f840160051c81019160208510610ec9575b601f0160051c019061160e565b82610d57565b9091508190610eb6565b3461013e575f60031936011261013e576020600954604051908152f35b3461013e575f60031936011261013e57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b3461013e575f60031936011261013e576020600f54604051908152f35b3461013e575f60031936011261013e576020600554604051908152f35b3461013e575f60031936011261013e57602073ffffffffffffffffffffffffffffffffffffffff60025416604051908152f35b3461013e575f60031936011261013e576020600c54604051908152f35b3461013e575f60031936011261013e576020600754604051908152f35b3461013e57602060031936011261013e5760043567ffffffffffffffff811161013e57610ffa903690600401611445565b61101c73ffffffffffffffffffffffffffffffffffffffff5f541633146115a9565b67ffffffffffffffff81116107805761103a81610616600b546113ac565b5f91601f82116001146110aa5761108782807f02585ebce918f656604dabb23332a6df1dcde119741f7c1f8fb37b191392a2a7955f9161109f575b505f198260011b9260031b1c19161790565b600b555b61109a60405192839283611696565b0390a1005b905083013586611075565b601f198216600b5f527f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9905f5b818110611132575093837f02585ebce918f656604dabb23332a6df1dcde119741f7c1f8fb37b191392a2a79510611119575b5050600182811b01600b5561108b565b5f1960f88560031b161c19908301351690558380611109565b838601358355602095860195600190930192016110d7565b3461013e57602060031936011261013e5760043567ffffffffffffffff811161013e5761117b903690600401611445565b61119d73ffffffffffffffffffffffffffffffffffffffff5f541633146115a9565b67ffffffffffffffff8111610780576111bb816105bb600a546113ac565b5f91601f82116001146112195761120782807f67c57ae6ad924cd093fb2f06bc0b28fd5879481051a9c203a44d27c8904d437c955f9161109f57505f198260011b9260031b1c19161790565b600a5561109a60405192839283611696565b601f198216600a5f527fc65a7bb8d6351c1cf70c95a316cc6a92839c986682d98bc35f958f4883f9d2a8905f5b8181106112a1575093837f67c57ae6ad924cd093fb2f06bc0b28fd5879481051a9c203a44d27c8904d437c9510611288575b5050600182811b01600a5561108b565b5f1960f88560031b161c19908301351690558380611278565b83860135835560209586019560019093019201611246565b3461013e575f60031936011261013e57602073ffffffffffffffffffffffffffffffffffffffff60015416604051908152f35b3461013e57602060031936011261013e576020604051308152f35b3461013e575f60031936011261013e576040515f600b54611327816113ac565b80845290600181169081156102ca575060011461134e57610203836101ef818503826113fd565b600b5f9081527f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9939250905b808210611392575090915081016020016101ef61025c565b91926001816020925483858801015201910190929161137a565b90600182811c921680156113f3575b60208310146113c657565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f16916113bb565b90601f601f19910116810190811067ffffffffffffffff82111761078057604052565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b9181601f8401121561013e5782359167ffffffffffffffff831161013e576020838186019501011161013e57565b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361013e57565b81601f8201121561013e5780359067ffffffffffffffff821161078057604051926114cb6020601f19601f86011601856113fd565b8284526020838301011161013e57815f926020809301838601378301015290565b6011545f92916114fb826113ac565b808252916001811690811561156f5750600114611516575050565b60115f9081529293509091907f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c685b838310611555575060209250010190565b600181602092949394548385870101520191019190611544565b60209495507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091509291921683830152151560051b010190565b156115b057565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f43616c6c6572206973206e6f7420746865206f776e65720000000000000000006044820152fd5b818110611619575050565b5f815560010161160e565b90601f8211611631575050565b61165c91600a5f5260205f20906020601f840160051c83019310610ec957601f0160051c019061160e565b565b90601f821161166b575050565b61165c91600b5f5260205f20906020601f840160051c83019310610ec957601f0160051c019061160e565b90601f83604094601f199360208652816020870152868601375f8582860101520116010190565b156116c457565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4e6577206f776e65722063616e6e6f74206265207a65726f20616464726573736044820152fd5b73ffffffffffffffffffffffffffffffffffffffff166117438115156116bd565b73ffffffffffffffffffffffffffffffffffffffff5f54827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a356f0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00"
