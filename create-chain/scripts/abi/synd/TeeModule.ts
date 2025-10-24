export const teeModuleABI = [
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "poster_",
        "type": "address",
        "internalType": "contract IAssertionPoster"
      },
      {
        "name": "bridge_",
        "type": "address",
        "internalType": "contract IBridge"
      },
      {
        "name": "configHash_",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "appStartBlockHash_",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "seqStartBlockHash_",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "l1StartBatchAcc_",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "l1BlockOrBridge_",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "isL1Chain_",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "challengeWindowDuration_",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "slowDuration_",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "teeKeyManager_",
        "type": "address",
        "internalType": "contract ITeeKeyManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "DEFAULT_ADMIN_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "SLOW_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "bridge",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBridge"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "challengeWindowDuration",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "challengeWindowEnd",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "challengeWindowStart",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "closeChallengeWindow",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "enterSlowMode",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getRoleAdmin",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleMember",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleMemberCount",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleMembers",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "grantRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "hasRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isL1Chain",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "l1BlockOrBridge",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pendingAssertions",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "appBlockHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "appSendRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "seqBlockHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "l1BatchAcc",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pendingAssertionsCount",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "poster",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAssertionPoster"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "callerConfirmation",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "resolveChallenge",
    "inputs": [
      {
        "name": "assertion",
        "type": "tuple",
        "internalType": "struct PendingAssertion",
        "components": [
          {
            "name": "appBlockHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "appSendRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "seqBlockHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "l1BatchAcc",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "revokeRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setSlowDuration",
    "inputs": [
      {
        "name": "slowDuration_",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "slowDuration",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "submitAssertion",
    "inputs": [
      {
        "name": "assertion",
        "type": "tuple",
        "internalType": "struct PendingAssertion",
        "components": [
          {
            "name": "appBlockHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "appSendRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "seqBlockHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "l1BatchAcc",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "rewardAddr",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "teeHackCount",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "teeKeyManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ITeeKeyManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "teeTrustedInput",
    "inputs": [],
    "outputs": [
      {
        "name": "configHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "appStartBlockHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "seqStartBlockHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "setDelayedMessageAcc",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "l1StartBatchAcc",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "l1EndHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "transferAssertionPosterOwner",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferFunds",
    "inputs": [
      {
        "name": "dest",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateChallengeWindowDuration",
    "inputs": [
      {
        "name": "challengeWindowDuration_",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateKeyManager",
    "inputs": [
      {
        "name": "newTeeKeyManager",
        "type": "address",
        "internalType": "contract ITeeKeyManager"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "AssertionPosterTransferred",
    "inputs": [
      {
        "name": "dest",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChallengeResolved",
    "inputs": [
      {
        "name": "",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct PendingAssertion",
        "components": [
          {
            "name": "appBlockHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "appSendRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "seqBlockHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "l1BatchAcc",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChallengeWindowDurationUpdate",
    "inputs": [
      {
        "name": "newChallengeWindowDuration",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "oldChallengeWindowDuration",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "FundsTransferred",
    "inputs": [
      {
        "name": "dest",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "KeyManagerUpdate",
    "inputs": [
      {
        "name": "newTeeKeyManager",
        "type": "address",
        "indexed": false,
        "internalType": "contract ITeeKeyManager"
      },
      {
        "name": "oldTeeKeyManager",
        "type": "address",
        "indexed": false,
        "internalType": "contract ITeeKeyManager"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleAdminChanged",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "previousAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "newAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleGranted",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleRevoked",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TeeHacked",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TeeInput",
    "inputs": [
      {
        "name": "input",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct TeeTrustedInput",
        "components": [
          {
            "name": "configHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "appStartBlockHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "seqStartBlockHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "setDelayedMessageAcc",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "l1StartBatchAcc",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "l1EndHash",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AccessControlBadConfirmation",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AccessControlUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "neededRole",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureLength",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureS",
    "inputs": [
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  }
] as const

export const teeModuleBytecode =
  "0x61010080604052346103c45761016081613189803803809161002182856108d3565b8339810103126103c45780516001600160a01b038116908181036103c45760208301516001600160a01b03811693908481036103c45760408201519460608301519360808401519560a08501519760c086015160018060a01b03811681036103c45760e08701519081151582036103c45761009f610100890161090a565b6101406100af6101208b0161090a565b9901516001600160a01b03811699908a90036103c4576001600160401b03828116908216111561086857600b8054600160401b600160c01b03191660409390931b6fffffffffffffffff0000000000000000169290921760809190911b600160801b600160c01b031617905560c05260e05260035561012d3361091e565b610830575b60e051156106f95760c0516001600160a01b0316734200000000000000000000000000000000000015146106b45760c0516040516221048360e21b815290602090829060049082906001600160a01b03165afa9081156103d0575f91610682575b5015610627575b3b156105d45760805260405163eca067ad60e01b815290602090829060049082905afa9081156103d0575f916105a2575b501561054d5760a052803b156104f35760018060a01b0319600254161760025560045560055560018060a01b0360a0511660405163eca067ad60e01b8152602081600481855afa9081156103d0575f916104c1575b505f1981019081116103db57602090602460405180948193636ab8cee160e11b835260048301525afa9081156103d0575f9161048f575b5060065560075560e051156104215760c0516040516221048360e21b81526001600160a01b0390911690602081600481855afa9081156103d0575f916103ef575b505f1981019081116103db576020906024604051809481936316bf557960e01b835260048301525afa9081156103d0575f9161039a575b506008555b7f55232299d83faf4dc2c32e228af37632bca7fa6dbc03b41224c100c6c9dca34960c06040516003548152600454602082015260055460408201526006546060820152600754608082015260085460a0820152a16040516127519081610a18823960805181818161098401528181610a840152611da3015260a0518181816101ff0152611cb9015260c051818181610e4301528181611be101528181611fa701526120bf015260e051818181610b2401528181611419015281816119190152611b5d0152f35b90506020813d6020116103c8575b816103b5602093836108d3565b810103126103c457515f6102cf565b5f80fd5b3d91506103a8565b6040513d5f823e3d90fd5b634e487b7160e01b5f52601160045260245ffd5b90506020813d602011610419575b8161040a602093836108d3565b810103126103c457515f610298565b3d91506103fd565b60c051604051624dead360e51b815290602090829060049082906001600160a01b03165afa9081156103d0575f9161045d575b506008556102d4565b90506020813d602011610487575b81610478602093836108d3565b810103126103c457515f610454565b3d915061046b565b90506020813d6020116104b9575b816104aa602093836108d3565b810103126103c457515f610257565b3d915061049d565b90506020813d6020116104eb575b816104dc602093836108d3565b810103126103c457515f610220565b3d91506104cf565b60405162461bcd60e51b815260206004820152602c60248201527f7465654b65794d616e61676572206164647265737320646f6573206e6f74206860448201526b61766520616e7920636f646560a01b6064820152608490fd5b60405162461bcd60e51b815260206004820152602760248201527f696e73756666696369656e742064656c61796564206d6573736167657320696e6044820152662062726964676560c81b6064820152608490fd5b90506020813d6020116105cc575b816105bd602093836108d3565b810103126103c457515f6101cb565b3d91506105b0565b60405162461bcd60e51b815260206004820152602560248201527f706f73746572206164647265737320646f6573206e6f74206861766520616e7960448201526420636f646560d81b6064820152608490fd5b60405162461bcd60e51b815260206004820152602d60248201527f73657175656e63696e6720636861696e206d7573742068617665206174206c6560448201526c0c2e6e840dedcca40c4c2e8c6d609b1b6064820152608490fd5b90506020813d6020116106ac575b8161069d602093836108d3565b810103126103c457515f610193565b3d9150610690565b60405162461bcd60e51b815260206004820152601d60248201527f756e6578706563746564207365712062726964676520616464726573730000006044820152606490fd5b60c051604051635c03bbf560e11b815290602090829060049082906001600160a01b03165afa9081156103d0575f916107f6575b506001600160401b0316151580610788575b61019a5760405162461bcd60e51b815260206004820152601960248201527f6c3120626c6f636b20636f6e747261637420696e76616c6964000000000000006044820152606490fd5b5060c051604051624dead360e51b815290602090829060049082906001600160a01b03165afa9081156103d0575f916107c4575b50151561073f565b90506020813d6020116107ee575b816107df602093836108d3565b810103126103c457515f6107bc565b3d91506107d2565b90506020813d602011610828575b81610811602093836108d3565b810103126103c4576108229061090a565b5f61072d565b3d9150610804565b5f80526001602052610862337fa6eef7e35abe7026729641147f7915573c7e97b47efa546f5f6e3230263bcb496109a7565b50610132565b60405162461bcd60e51b815260206004820152603c60248201527f736c6f77206475726174696f6e206d757374206265206772656174657220746860448201527f616e206368616c6c656e67652077696e646f77206475726174696f6e000000006064820152608490fd5b601f909101601f19168101906001600160401b038211908210176108f657604052565b634e487b7160e01b5f52604160045260245ffd5b51906001600160401b03821682036103c457565b6001600160a01b0381165f9081525f5160206131695f395f51905f52602052604090205460ff166109a2576001600160a01b03165f8181525f5160206131695f395f51905f5260205260408120805460ff191660011790553391907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d8180a4600190565b505f90565b6001810190825f528160205260405f2054155f14610a10578054680100000000000000008110156108f657600181018083558110156109fc578390825f5260205f20015554915f5260205260405f2055600190565b634e487b7160e01b5f52603260045260245ffd5b5050505f9056fe608080604052600436101561001c575b50361561001a575f80fd5b005b5f905f3560e01c90816301ffc9a714610f925750806307369de514610f6857806316275f8714610eb6578063248a9ca314610e845780632521c53514610e6757806327d4029914610e175780632f2ff15d14610da75780633183baac14610d44578063350bd6a314610c2d57806336568abe14610bc35780633a009a0614610b905780633ceaae7d14610b49578063470b9b1a14610b0d578063478bf55614610a0a5780634bd167c9146109df578063697b5e62146109c15780636c4c2060146109a857806380959721146109575780639010d07c1461090557806391d14854146108ae5780639b79e0c21461078d578063a217fddf14610771578063a3246ad3146106b4578063a56ec6cd1461065d578063bb787cc91461055f578063ca15c87314610535578063d547741f146104ee578063d6ad5ec71461034f578063e39ff19f14610286578063e4ee70e51461025e578063e6b4f81614610223578063e78cea92146101d25763ee1c28b80361000f57346101cf57806003193601126101cf5760206101bd600b5467ffffffffffffffff808260401c169116611b2f565b67ffffffffffffffff60405191168152f35b80fd5b50346101cf57806003193601126101cf57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b50346101cf57806003193601126101cf5760206040517fcdb20e26573324aceeff65baefea690e77bb8b116924d166a9fd1c2471ce17108152f35b50346101cf57806003193601126101cf57602067ffffffffffffffff600b5416604051908152f35b50346101cf5760206003193601126101cf5773ffffffffffffffffffffffffffffffffffffffff6102b5611086565b6102bd6121a2565b16801561030b5781808080610308947f17f29f58ff29e58f40fe3fa963a7469e393593784592e72c3b2355f9199776e06020604051838152a147905af16103026111fa565b50611229565b80f35b606460405162461bcd60e51b815260206004820152601b60248201527f64657374696e6174696f6e2061646472657373206973207a65726f00000000006044820152fd5b50346101cf57806003193601126101cf577fcdb20e26573324aceeff65baefea690e77bb8b116924d166a9fd1c2471ce17108152806020526040812073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f2054161561049e57600b5467ffffffffffffffff8160801c168160401c67ffffffffffffffff8116908183111561045a576040805167ffffffffffffffff94851681529290931660208301527fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff926fffffffffffffffff0000000000000000927f75689a8adaf52fab3f618b2698a3868150b33d8ba13b2f1a3ee2bcc3107336419190a116911617600b5580f35b606460405162461bcd60e51b815260206004820152601460248201527f616c726561647920696e20736c6f77206d6f64650000000000000000000000006044820152fd5b807fe2517d3f0000000000000000000000000000000000000000000000000000000060449252336004527fcdb20e26573324aceeff65baefea690e77bb8b116924d166a9fd1c2471ce1710602452fd5b50346101cf5760406003193601126101cf5761053160043561050e611063565b9061052c610527825f525f602052600160405f20015490565b61220a565b612270565b5080f35b50346101cf5760206003193601126101cf5760406020916004358152600183522054604051908152f35b50346101cf5760206003193601126101cf5760043567ffffffffffffffff8116908181036106595761058f6121a2565b600b549167ffffffffffffffff8360401c1610156105ef5777ffffffffffffffff000000000000000000000000000000007fffffffffffffffff0000000000000000ffffffffffffffffffffffffffffffff9160801b16911617600b5580f35b608460405162461bcd60e51b815260206004820152603c60248201527f736c6f77206475726174696f6e206d757374206265206772656174657220746860448201527f616e206368616c6c656e67652077696e646f77206475726174696f6e000000006064820152fd5b8280fd5b50346101cf5760206003193601126101cf57600435906009548210156101cf576080610688836110d7565b508054906001810154906003600282015491015491604051938452602084015260408301526060820152f35b50346101cf5760206003193601126101cf576004358152600160205260408120604051908160208254918281520190819285526020852090855b81811061075b5750505082610704910383611152565b604051928392602084019060208552518091526040840192915b81811061072c575050500390f35b825173ffffffffffffffffffffffffffffffffffffffff1684528594506020938401939092019160010161071e565b82548452602090930192600192830192016106ee565b50346101cf57806003193601126101cf57602090604051908152f35b50346101cf5760206003193601126101cf5760043573ffffffffffffffffffffffffffffffffffffffff81168091036108aa576107c86121a2565b803b15610840577fffffffffffffffffffffffff00000000000000000000000000000000000000006002547ff0993f232dc1fec9928385ddc3794d109479cdee2d14bf929a000bb3a448d70c6040805185815273ffffffffffffffffffffffffffffffffffffffff84166020820152a1161760025580f35b608460405162461bcd60e51b815260206004820152602c60248201527f7465654b65794d616e61676572206164647265737320646f6573206e6f74206860448201527f61766520616e7920636f646500000000000000000000000000000000000000006064820152fd5b5080fd5b50346101cf5760406003193601126101cf5773ffffffffffffffffffffffffffffffffffffffff60406108df611063565b926004358152806020522091165f52602052602060ff60405f2054166040519015158152f35b50346101cf5760406003193601126101cf5773ffffffffffffffffffffffffffffffffffffffff61094760209260043581526001845260406024359120612561565b90549060031b1c16604051908152f35b50346101cf57806003193601126101cf57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b50346101cf57806003193601126101cf57610308611b51565b50346101cf57806003193601126101cf576020600a54604051908152f35b50346101cf57806003193601126101cf57602067ffffffffffffffff600b5460401c16604051908152f35b5034610b09576020600319360112610b0957610a24611086565b610a2c6121a2565b7e2ae90e22e60b8948054f7d1ac3af1d32155f74a4911928decf0c3a6f6351b1602073ffffffffffffffffffffffffffffffffffffffff604051931692838152a173ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690813b15610b09575f916024839260405194859384927ff2fde38b00000000000000000000000000000000000000000000000000000000845260048401525af18015610afe57610af2575080f35b61001a91505f90611152565b6040513d5f823e3d90fd5b5f80fd5b34610b09575f600319360112610b095760206040517f000000000000000000000000000000000000000000000000000000000000000015158152f35b34610b09575f600319360112610b095760c06003546004546005546006546007549160085493604051958652602086015260408501526060840152608083015260a0820152f35b34610b09575f600319360112610b0957602073ffffffffffffffffffffffffffffffffffffffff60025416604051908152f35b34610b09576040600319360112610b0957610bdc611063565b3373ffffffffffffffffffffffffffffffffffffffff821603610c055761001a90600435612270565b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610b0957600319360160a08112610b0957608013610b095760843567ffffffffffffffff8111610b0957610c669036906004016110a9565b610c6e6121a2565b60016009541115610d0057610c8a91610c85611aa1565b6117a9565b7fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000600b5416600b55610cba611b51565b7f2020542b6e6b951d4c0736eed2a4d762d20bb1ba579f99feffae9b1dea24088360806040516004358152602435602082015260443560408201526064356060820152a1005b606460405162461bcd60e51b815260206004820152601860248201527f6368616c6c656e676520646f6573206e6f7420657869737400000000000000006044820152fd5b34610b0957600319360160c08112610b0957608013610b095760843567ffffffffffffffff8111610b0957610d7d9036906004016110a9565b60a4359073ffffffffffffffffffffffffffffffffffffffff82168203610b095761001a92611274565b34610b09576040600319360112610b0957600435610dc3611063565b610ddb610527835f525f602052600160405f20015490565b610de581836122b5565b610deb57005b61001a915f52600160205273ffffffffffffffffffffffffffffffffffffffff60405f20911690612576565b34610b09575f600319360112610b0957602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610b09575f600319360112610b09576020600954604051908152f35b34610b09576020600319360112610b09576020610eae6004355f525f602052600160405f20015490565b604051908152f35b34610b09576020600319360112610b095760043567ffffffffffffffff81168103610b0957610ee36121a2565b600b546040805167ffffffffffffffff848116825283831c16602082015291927fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff926fffffffffffffffff0000000000000000927f75689a8adaf52fab3f618b2698a3868150b33d8ba13b2f1a3ee2bcc31073364191a160401b16911617600b555f80f35b34610b09575f600319360112610b0957602067ffffffffffffffff600b5460801c16604051908152f35b34610b09576020600319360112610b0957600435907fffffffff000000000000000000000000000000000000000000000000000000008216809203610b0957817f5a05180f0000000000000000000000000000000000000000000000000000000060209314908115611006575b5015158152f35b7f7965db0b00000000000000000000000000000000000000000000000000000000811491508115611039575b5083610fff565b7f01ffc9a70000000000000000000000000000000000000000000000000000000091501483611032565b6024359073ffffffffffffffffffffffffffffffffffffffff82168203610b0957565b6004359073ffffffffffffffffffffffffffffffffffffffff82168203610b0957565b9181601f84011215610b095782359167ffffffffffffffff8311610b095760208381860195010111610b0957565b6009548110156110f35760095f5260205f209060021b01905f90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b600954156110f35760095f9081527f6e1540171b6c0c960b71a7020d9f60077f6af931a8bbf590da0223dacf75c7af91565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761119357604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b67ffffffffffffffff811161119357601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b3d15611224573d9061120b826111c0565b916112196040519384611152565b82523d5f602084013e565b606090565b1561123057565b606460405162461bcd60e51b815260206004820152600e60248201527f7061796d656e74206661696c65640000000000000000000000000000000000006044820152fd5b9060418103611765576004356024356044356064359360405160208101906112e6816112ba8987898b889290916080949284526020840152604083015260608201520190565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282611152565b519020956003546004546005546006546007549060085492604051946020860196875260408601526060850152608084015260a083015260c082015260c0815261133160e082611152565b5190206040516020810191825288604082015260408152611353606082611152565b5190209173ffffffffffffffffffffffffffffffffffffffff600254169261137a826111c0565b916113886040519384611152565b8083523681850111610b09576113c7836024935f6020856113d096829a8373ffffffffffffffffffffffffffffffffffffffff9b013784010152612387565b909291926123c1565b60405194859384927f7217efcd0000000000000000000000000000000000000000000000000000000084521660048301525afa908115610afe575f9161172a575b50156116e6577f00000000000000000000000000000000000000000000000000000000000000001580156116db575b1561169757600954680100000000000000008110156111935780600161146992016009556110d7565b92909261166b57600393835560018301556002820155015560095460018114611632576002036115c85761149b611120565b508054906114dc60018201546112ba600360028501549401546040519485936020850197889290916080949284526020840152604083015260608201520190565b5190201461158457600a549060018201809211611557577f37e8add694c5926d564e971160f5974103cbbbc7c90747c4c6f802031d3567a760208373ffffffffffffffffffffffffffffffffffffffff94600a55604051908152a1168015611554575f8080806115529447905af16103026111fa565b565b50565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b606460405162461bcd60e51b815260206004820152601860248201527f617373657274696f6e20616c72656164792065786973747300000000000000006044820152fd5b608460405162461bcd60e51b815260206004820152602660248201527f5465654d6f64756c653a20546f6f206d616e792070656e64696e67206173736560448201527f7274696f6e7300000000000000000000000000000000000000000000000000006064820152fd5b50505067ffffffffffffffff42167fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000600b541617600b55565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b606460405162461bcd60e51b815260206004820152601b60248201527f756e6578706563746564206c3120656e642062617463682061636300000000006044820152fd5b506008548414611440565b606460405162461bcd60e51b815260206004820152601560248201527f696e76616c696420746565207369676e617475726500000000000000000000006044820152fd5b90506020813d60201161175d575b8161174560209383611152565b81010312610b0957518015158103610b09575f611411565b3d9150611738565b606460405162461bcd60e51b815260206004820152601860248201527f696e76616c6964207369676e6174757265206c656e67746800000000000000006044820152fd5b9060418103611765576004356024356044356064359360405160208101906117ef816112ba8987898b889290916080949284526020840152604083015260608201520190565b519020956003546004546005546006546007549060085492604051946020860196875260408601526060850152608084015260a083015260c082015260c0815261183a60e082611152565b519020604051602081019182528860408201526040815261185c606082611152565b5190209173ffffffffffffffffffffffffffffffffffffffff6002541692611883826111c0565b916118916040519384611152565b8083523681850111610b09576113c7836024935f6020856118d096829a8373ffffffffffffffffffffffffffffffffffffffff9b013784010152612387565b60405194859384927f7217efcd0000000000000000000000000000000000000000000000000000000084521660048301525afa908115610afe575f91611a66575b50156116e6577f0000000000000000000000000000000000000000000000000000000000000000158015611a5b575b1561169757600954680100000000000000008110156111935780600161196992016009556110d7565b92909261166b57600393835560018301556002820155015560095460018114611a23576002036115c85761199b611120565b508054906119dc60018201546112ba600360028501549401546040519485936020850197889290916080949284526020840152604083015260608201520190565b5190201461158457600a5460018101809111611557576020817f37e8add694c5926d564e971160f5974103cbbbc7c90747c4c6f802031d3567a792600a55604051908152a1565b505067ffffffffffffffff42167fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000600b541617600b55565b506008548414611940565b90506020813d602011611a99575b81611a8160209383611152565b81010312610b0957518015158103610b09575f611911565b3d9150611a74565b6009545f60095580611ab05750565b7f3fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff811681036115575760095f5260021b7f6e1540171b6c0c960b71a7020d9f60077f6af931a8bbf590da0223dacf75c7af908101905b818110611b11575050565b6004905f81555f60018201555f60028201555f600382015501611b06565b9067ffffffffffffffff8091169116019067ffffffffffffffff821161155757565b600160095403612138577f0000000000000000000000000000000000000000000000000000000000000000801561207c5767ffffffffffffffff42165b67ffffffffffffffff80611bac600b5482808260401c169116611b2f565b1691161115612012576003611bbf611120565b50015460075515611f645773ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517e84120c000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610afe575f91611f32575b505f198101908111611557576020906024604051809481937f16bf557900000000000000000000000000000000000000000000000000000000835260048301525afa908115610afe575f91611f00575b506008555b6002611c9c611120565b50015460055573ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517feca067ad000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610afe575f91611ece575b505f198101908111611557576020906024604051809481937fd5719dc200000000000000000000000000000000000000000000000000000000835260048301525afa908115610afe575f91611e9c575b50600655600454611d75611120565b505414611e8f57611d84611120565b50546004556001611d93611120565b500154611d9e611aa1565b6004547f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1691823b15610b095760445f928360405195869485937fdaeab412000000000000000000000000000000000000000000000000000000008552600485015260248401525af18015610afe57611e7f575b505b7f55232299d83faf4dc2c32e228af37632bca7fa6dbc03b41224c100c6c9dca34960c06040516003548152600454602082015260055460408201526006546060820152600754608082015260085460a0820152a1565b5f611e8991611152565b5f611e27565b611e97611aa1565b611e29565b90506020813d602011611ec6575b81611eb760209383611152565b81010312610b0957515f611d66565b3d9150611eaa565b90506020813d602011611ef8575b81611ee960209383611152565b81010312610b0957515f611d16565b3d9150611edc565b90506020813d602011611f2a575b81611f1b60209383611152565b81010312610b0957515f611c8d565b3d9150611f0e565b90506020813d602011611f5c575b81611f4d60209383611152565b81010312610b0957515f611c3d565b3d9150611f40565b6040517f09bd5a6000000000000000000000000000000000000000000000000000000000815260208160048173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115610afe575f91611fe0575b50600855611c92565b90506020813d60201161200a575b81611ffb60209383611152565b81010312610b0957515f611fd7565b3d9150611fee565b608460405162461bcd60e51b815260206004820152603c60248201527f63616e6e6f7420636c6f7365206368616c6c656e67652077696e646f77202d2060448201527f696e73756666696369656e742074696d652068617320706173736564000000006064820152fd5b6040517fb80777ea00000000000000000000000000000000000000000000000000000000815260208160048173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115610afe575f916120f5575b50611b8e565b90506020813d602011612130575b8161211060209383611152565b81010312610b09575167ffffffffffffffff81168103610b09575f6120ef565b3d9150612103565b608460405162461bcd60e51b815260206004820152603a60248201527f63616e6e6f7420636c6f7365206368616c6c656e67652077696e646f77202d2060448201527f77726f6e67206e756d626572206f6620617373657274696f6e730000000000006064820152fd5b335f9081527fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5602052604090205460ff16156121da57565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f205416156122415750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b61227a8282612499565b918261228557505090565b6122b1915f52600160205273ffffffffffffffffffffffffffffffffffffffff60405f20911690612674565b5090565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f1461238157805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b81519190604183036123b7576123b09250602082015190606060408401519301515f1a906125e5565b9192909190565b50505f9160029190565b600481101561246c57806123d3575050565b60018103612403577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b6002810361243757507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b6003146124415750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f1461238157805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190565b80548210156110f3575f5260205f2001905f90565b6001810190825f528160205260405f2054155f146125de57805468010000000000000000811015611193576125cb6125b5826001879401855584612561565b81939154905f199060031b92831b921b19161790565b905554915f5260205260405f2055600190565b5050505f90565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411612669579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15610afe575f5173ffffffffffffffffffffffffffffffffffffffff81161561265f57905f905f90565b505f906001905f90565b5050505f9160039190565b906001820191815f528260205260405f20548015155f14612749575f198101818111611557578254905f19820191821161155757818103612714575b505050805480156126e7575f1901906126c98282612561565b5f1982549160031b1b19169055555f526020525f6040812055600190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b6127346127246125b59386612561565b90549060031b1c92839286612561565b90555f528360205260405f20555f80806126b0565b505050505f9056ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5"
