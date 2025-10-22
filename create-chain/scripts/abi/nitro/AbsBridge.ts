export const AbsBridgeAbi = [
  {
    type: "function",
    name: "acceptFundsFromOldBridge",
    inputs: [],
    outputs: [],
    stateMutability: "payable"
  },
  {
    type: "function",
    name: "activeOutbox",
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
    name: "allowedDelayedInboxList",
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
    name: "allowedDelayedInboxes",
    inputs: [
      {
        name: "inbox",
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
    name: "allowedOutboxList",
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
    name: "allowedOutboxes",
    inputs: [
      {
        name: "outbox",
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
    name: "delayedInboxAccs",
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
        type: "bytes32",
        internalType: "bytes32"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "delayedMessageCount",
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
    name: "enqueueSequencerMessage",
    inputs: [
      {
        name: "dataHash",
        type: "bytes32",
        internalType: "bytes32"
      },
      {
        name: "afterDelayedMessagesRead",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "prevMessageCount",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "newMessageCount",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    outputs: [
      {
        name: "seqMessageIndex",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "beforeAcc",
        type: "bytes32",
        internalType: "bytes32"
      },
      {
        name: "delayedAcc",
        type: "bytes32",
        internalType: "bytes32"
      },
      {
        name: "acc",
        type: "bytes32",
        internalType: "bytes32"
      }
    ],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "executeCall",
    inputs: [
      {
        name: "to",
        type: "address",
        internalType: "address"
      },
      {
        name: "value",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "data",
        type: "bytes",
        internalType: "bytes"
      }
    ],
    outputs: [
      {
        name: "success",
        type: "bool",
        internalType: "bool"
      },
      {
        name: "returnData",
        type: "bytes",
        internalType: "bytes"
      }
    ],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "rollup",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "contract IOwnable"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "sequencerInbox",
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
    name: "sequencerInboxAccs",
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
        type: "bytes32",
        internalType: "bytes32"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "sequencerMessageCount",
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
    name: "sequencerReportedSubMessageCount",
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
    name: "setDelayedInbox",
    inputs: [
      {
        name: "inbox",
        type: "address",
        internalType: "address"
      },
      {
        name: "enabled",
        type: "bool",
        internalType: "bool"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "setOutbox",
    inputs: [
      {
        name: "outbox",
        type: "address",
        internalType: "address"
      },
      {
        name: "enabled",
        type: "bool",
        internalType: "bool"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "setSequencerInbox",
    inputs: [
      {
        name: "_sequencerInbox",
        type: "address",
        internalType: "address"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "setSequencerReportedSubMessageCount",
    inputs: [
      {
        name: "newMsgCount",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "submitBatchSpendingReport",
    inputs: [
      {
        name: "sender",
        type: "address",
        internalType: "address"
      },
      {
        name: "messageDataHash",
        type: "bytes32",
        internalType: "bytes32"
      }
    ],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "updateRollupAddress",
    inputs: [
      {
        name: "_rollup",
        type: "address",
        internalType: "contract IOwnable"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "event",
    name: "BridgeCallTriggered",
    inputs: [
      {
        name: "outbox",
        type: "address",
        indexed: true,
        internalType: "address"
      },
      {
        name: "to",
        type: "address",
        indexed: true,
        internalType: "address"
      },
      {
        name: "value",
        type: "uint256",
        indexed: false,
        internalType: "uint256"
      },
      {
        name: "data",
        type: "bytes",
        indexed: false,
        internalType: "bytes"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "InboxToggle",
    inputs: [
      {
        name: "inbox",
        type: "address",
        indexed: true,
        internalType: "address"
      },
      {
        name: "enabled",
        type: "bool",
        indexed: false,
        internalType: "bool"
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
        type: "uint8",
        indexed: false,
        internalType: "uint8"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "MessageDelivered",
    inputs: [
      {
        name: "messageIndex",
        type: "uint256",
        indexed: true,
        internalType: "uint256"
      },
      {
        name: "beforeInboxAcc",
        type: "bytes32",
        indexed: true,
        internalType: "bytes32"
      },
      {
        name: "inbox",
        type: "address",
        indexed: false,
        internalType: "address"
      },
      {
        name: "kind",
        type: "uint8",
        indexed: false,
        internalType: "uint8"
      },
      {
        name: "sender",
        type: "address",
        indexed: false,
        internalType: "address"
      },
      {
        name: "messageDataHash",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32"
      },
      {
        name: "baseFeeL1",
        type: "uint256",
        indexed: false,
        internalType: "uint256"
      },
      {
        name: "timestamp",
        type: "uint64",
        indexed: false,
        internalType: "uint64"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "OutboxToggle",
    inputs: [
      {
        name: "outbox",
        type: "address",
        indexed: true,
        internalType: "address"
      },
      {
        name: "enabled",
        type: "bool",
        indexed: false,
        internalType: "bool"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "RollupUpdated",
    inputs: [
      {
        name: "rollup",
        type: "address",
        indexed: false,
        internalType: "address"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "SequencerInboxUpdated",
    inputs: [
      {
        name: "newSequencerInbox",
        type: "address",
        indexed: false,
        internalType: "address"
      }
    ],
    anonymous: false
  },
  {
    type: "error",
    name: "BadSequencerMessageNumber",
    inputs: [
      {
        name: "stored",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "received",
        type: "uint256",
        internalType: "uint256"
      }
    ]
  },
  {
    type: "error",
    name: "InvalidOutboxSet",
    inputs: [
      {
        name: "outbox",
        type: "address",
        internalType: "address"
      }
    ]
  },
  {
    type: "error",
    name: "NotContract",
    inputs: [
      {
        name: "addr",
        type: "address",
        internalType: "address"
      }
    ]
  },
  {
    type: "error",
    name: "NotOutbox",
    inputs: [
      {
        name: "sender",
        type: "address",
        internalType: "address"
      }
    ]
  },
  {
    type: "error",
    name: "NotRollupOrOwner",
    inputs: [
      {
        name: "sender",
        type: "address",
        internalType: "address"
      },
      {
        name: "rollup",
        type: "address",
        internalType: "address"
      },
      {
        name: "owner",
        type: "address",
        internalType: "address"
      }
    ]
  },
  {
    type: "error",
    name: "NotSequencerInbox",
    inputs: [
      {
        name: "sender",
        type: "address",
        internalType: "address"
      }
    ]
  }
] as const
