export const SequencerInboxABI = [
  {
    type: "constructor",
    inputs: [
      {
        name: "_maxDataSize",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "reader4844_",
        type: "address",
        internalType: "contract IReader4844"
      },
      {
        name: "_isUsingFeeToken",
        type: "bool",
        internalType: "bool"
      },
      {
        name: "_isDelayBufferable",
        type: "bool",
        internalType: "bool"
      }
    ],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "BROTLI_MESSAGE_HEADER_FLAG",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "bytes1",
        internalType: "bytes1"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "DAS_MESSAGE_HEADER_FLAG",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "bytes1",
        internalType: "bytes1"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "DATA_AUTHENTICATED_FLAG",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "bytes1",
        internalType: "bytes1"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "DATA_BLOB_HEADER_FLAG",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "bytes1",
        internalType: "bytes1"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "HEADER_LENGTH",
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
    name: "TREE_DAS_MESSAGE_HEADER_FLAG",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "bytes1",
        internalType: "bytes1"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "ZERO_HEAVY_MESSAGE_HEADER_FLAG",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "bytes1",
        internalType: "bytes1"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "addSequencerL2Batch",
    inputs: [
      {
        name: "sequenceNumber",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "data",
        type: "bytes",
        internalType: "bytes"
      },
      {
        name: "afterDelayedMessagesRead",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "gasRefunder",
        type: "address",
        internalType: "contract IGasRefunder"
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
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "addSequencerL2BatchDelayProof",
    inputs: [
      {
        name: "sequenceNumber",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "data",
        type: "bytes",
        internalType: "bytes"
      },
      {
        name: "afterDelayedMessagesRead",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "gasRefunder",
        type: "address",
        internalType: "contract IGasRefunder"
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
      },
      {
        name: "delayProof",
        type: "tuple",
        internalType: "struct DelayProof",
        components: [
          {
            name: "beforeDelayedAcc",
            type: "bytes32",
            internalType: "bytes32"
          },
          {
            name: "delayedMessage",
            type: "tuple",
            internalType: "struct Messages.Message",
            components: [
              {
                name: "kind",
                type: "uint8",
                internalType: "uint8"
              },
              {
                name: "sender",
                type: "address",
                internalType: "address"
              },
              {
                name: "blockNumber",
                type: "uint64",
                internalType: "uint64"
              },
              {
                name: "timestamp",
                type: "uint64",
                internalType: "uint64"
              },
              {
                name: "inboxSeqNum",
                type: "uint256",
                internalType: "uint256"
              },
              {
                name: "baseFeeL1",
                type: "uint256",
                internalType: "uint256"
              },
              {
                name: "messageDataHash",
                type: "bytes32",
                internalType: "bytes32"
              }
            ]
          }
        ]
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "addSequencerL2BatchFromBlobs",
    inputs: [
      {
        name: "sequenceNumber",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "afterDelayedMessagesRead",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "gasRefunder",
        type: "address",
        internalType: "contract IGasRefunder"
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
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "addSequencerL2BatchFromBlobsDelayProof",
    inputs: [
      {
        name: "sequenceNumber",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "afterDelayedMessagesRead",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "gasRefunder",
        type: "address",
        internalType: "contract IGasRefunder"
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
      },
      {
        name: "delayProof",
        type: "tuple",
        internalType: "struct DelayProof",
        components: [
          {
            name: "beforeDelayedAcc",
            type: "bytes32",
            internalType: "bytes32"
          },
          {
            name: "delayedMessage",
            type: "tuple",
            internalType: "struct Messages.Message",
            components: [
              {
                name: "kind",
                type: "uint8",
                internalType: "uint8"
              },
              {
                name: "sender",
                type: "address",
                internalType: "address"
              },
              {
                name: "blockNumber",
                type: "uint64",
                internalType: "uint64"
              },
              {
                name: "timestamp",
                type: "uint64",
                internalType: "uint64"
              },
              {
                name: "inboxSeqNum",
                type: "uint256",
                internalType: "uint256"
              },
              {
                name: "baseFeeL1",
                type: "uint256",
                internalType: "uint256"
              },
              {
                name: "messageDataHash",
                type: "bytes32",
                internalType: "bytes32"
              }
            ]
          }
        ]
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "addSequencerL2BatchFromOrigin",
    inputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "",
        type: "bytes",
        internalType: "bytes"
      },
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "",
        type: "address",
        internalType: "contract IGasRefunder"
      }
    ],
    outputs: [],
    stateMutability: "pure"
  },
  {
    type: "function",
    name: "addSequencerL2BatchFromOrigin",
    inputs: [
      {
        name: "sequenceNumber",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "data",
        type: "bytes",
        internalType: "bytes"
      },
      {
        name: "afterDelayedMessagesRead",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "gasRefunder",
        type: "address",
        internalType: "contract IGasRefunder"
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
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "addSequencerL2BatchFromOriginDelayProof",
    inputs: [
      {
        name: "sequenceNumber",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "data",
        type: "bytes",
        internalType: "bytes"
      },
      {
        name: "afterDelayedMessagesRead",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "gasRefunder",
        type: "address",
        internalType: "contract IGasRefunder"
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
      },
      {
        name: "delayProof",
        type: "tuple",
        internalType: "struct DelayProof",
        components: [
          {
            name: "beforeDelayedAcc",
            type: "bytes32",
            internalType: "bytes32"
          },
          {
            name: "delayedMessage",
            type: "tuple",
            internalType: "struct Messages.Message",
            components: [
              {
                name: "kind",
                type: "uint8",
                internalType: "uint8"
              },
              {
                name: "sender",
                type: "address",
                internalType: "address"
              },
              {
                name: "blockNumber",
                type: "uint64",
                internalType: "uint64"
              },
              {
                name: "timestamp",
                type: "uint64",
                internalType: "uint64"
              },
              {
                name: "inboxSeqNum",
                type: "uint256",
                internalType: "uint256"
              },
              {
                name: "baseFeeL1",
                type: "uint256",
                internalType: "uint256"
              },
              {
                name: "messageDataHash",
                type: "bytes32",
                internalType: "bytes32"
              }
            ]
          }
        ]
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "batchCount",
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
    name: "batchPosterManager",
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
    name: "bridge",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "contract IBridge"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "buffer",
    inputs: [],
    outputs: [
      {
        name: "bufferBlocks",
        type: "uint64",
        internalType: "uint64"
      },
      {
        name: "max",
        type: "uint64",
        internalType: "uint64"
      },
      {
        name: "threshold",
        type: "uint64",
        internalType: "uint64"
      },
      {
        name: "prevBlockNumber",
        type: "uint64",
        internalType: "uint64"
      },
      {
        name: "replenishRateInBasis",
        type: "uint64",
        internalType: "uint64"
      },
      {
        name: "prevSequencedBlockNumber",
        type: "uint64",
        internalType: "uint64"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "dasKeySetInfo",
    inputs: [
      {
        name: "",
        type: "bytes32",
        internalType: "bytes32"
      }
    ],
    outputs: [
      {
        name: "isValidKeyset",
        type: "bool",
        internalType: "bool"
      },
      {
        name: "creationBlock",
        type: "uint64",
        internalType: "uint64"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "feeTokenPricer",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "contract IFeeTokenPricer"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "forceInclusion",
    inputs: [
      {
        name: "_totalDelayedMessagesRead",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "kind",
        type: "uint8",
        internalType: "uint8"
      },
      {
        name: "l1BlockAndTime",
        type: "uint64[2]",
        internalType: "uint64[2]"
      },
      {
        name: "baseFeeL1",
        type: "uint256",
        internalType: "uint256"
      },
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
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "forceInclusionDeadline",
    inputs: [
      {
        name: "blockNumber",
        type: "uint64",
        internalType: "uint64"
      }
    ],
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
    name: "getKeysetCreationBlock",
    inputs: [
      {
        name: "ksHash",
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
    stateMutability: "view"
  },
  {
    type: "function",
    name: "inboxAccs",
    inputs: [
      {
        name: "index",
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
    name: "initialize",
    inputs: [
      {
        name: "bridge_",
        type: "address",
        internalType: "contract IBridge"
      },
      {
        name: "maxTimeVariation_",
        type: "tuple",
        internalType: "struct ISequencerInbox.MaxTimeVariation",
        components: [
          {
            name: "delayBlocks",
            type: "uint256",
            internalType: "uint256"
          },
          {
            name: "futureBlocks",
            type: "uint256",
            internalType: "uint256"
          },
          {
            name: "delaySeconds",
            type: "uint256",
            internalType: "uint256"
          },
          {
            name: "futureSeconds",
            type: "uint256",
            internalType: "uint256"
          }
        ]
      },
      {
        name: "bufferConfig_",
        type: "tuple",
        internalType: "struct BufferConfig",
        components: [
          {
            name: "threshold",
            type: "uint64",
            internalType: "uint64"
          },
          {
            name: "max",
            type: "uint64",
            internalType: "uint64"
          },
          {
            name: "replenishRateInBasis",
            type: "uint64",
            internalType: "uint64"
          }
        ]
      },
      {
        name: "feeTokenPricer_",
        type: "address",
        internalType: "contract IFeeTokenPricer"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "invalidateKeysetHash",
    inputs: [
      {
        name: "ksHash",
        type: "bytes32",
        internalType: "bytes32"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "isBatchPoster",
    inputs: [
      {
        name: "",
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
    name: "isDelayBufferable",
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
    name: "isSequencer",
    inputs: [
      {
        name: "",
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
    name: "isUsingFeeToken",
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
    name: "isValidKeysetHash",
    inputs: [
      {
        name: "ksHash",
        type: "bytes32",
        internalType: "bytes32"
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
    name: "maxDataSize",
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
    name: "maxTimeVariation",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "",
        type: "uint256",
        internalType: "uint256"
      },
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
    name: "postUpgradeInit",
    inputs: [
      {
        name: "bufferConfig_",
        type: "tuple",
        internalType: "struct BufferConfig",
        components: [
          {
            name: "threshold",
            type: "uint64",
            internalType: "uint64"
          },
          {
            name: "max",
            type: "uint64",
            internalType: "uint64"
          },
          {
            name: "replenishRateInBasis",
            type: "uint64",
            internalType: "uint64"
          }
        ]
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "reader4844",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "contract IReader4844"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "removeDelayAfterFork",
    inputs: [],
    outputs: [],
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
    name: "setBatchPosterManager",
    inputs: [
      {
        name: "newBatchPosterManager",
        type: "address",
        internalType: "address"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "setBufferConfig",
    inputs: [
      {
        name: "bufferConfig_",
        type: "tuple",
        internalType: "struct BufferConfig",
        components: [
          {
            name: "threshold",
            type: "uint64",
            internalType: "uint64"
          },
          {
            name: "max",
            type: "uint64",
            internalType: "uint64"
          },
          {
            name: "replenishRateInBasis",
            type: "uint64",
            internalType: "uint64"
          }
        ]
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "setFeeTokenPricer",
    inputs: [
      {
        name: "feeTokenPricer_",
        type: "address",
        internalType: "contract IFeeTokenPricer"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "setIsBatchPoster",
    inputs: [
      {
        name: "addr",
        type: "address",
        internalType: "address"
      },
      {
        name: "isBatchPoster_",
        type: "bool",
        internalType: "bool"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "setIsSequencer",
    inputs: [
      {
        name: "addr",
        type: "address",
        internalType: "address"
      },
      {
        name: "isSequencer_",
        type: "bool",
        internalType: "bool"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "setMaxTimeVariation",
    inputs: [
      {
        name: "maxTimeVariation_",
        type: "tuple",
        internalType: "struct ISequencerInbox.MaxTimeVariation",
        components: [
          {
            name: "delayBlocks",
            type: "uint256",
            internalType: "uint256"
          },
          {
            name: "futureBlocks",
            type: "uint256",
            internalType: "uint256"
          },
          {
            name: "delaySeconds",
            type: "uint256",
            internalType: "uint256"
          },
          {
            name: "futureSeconds",
            type: "uint256",
            internalType: "uint256"
          }
        ]
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "setValidKeyset",
    inputs: [
      {
        name: "keysetBytes",
        type: "bytes",
        internalType: "bytes"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "totalDelayedMessagesRead",
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
    name: "updateRollupAddress",
    inputs: [],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "event",
    name: "BatchPosterManagerSet",
    inputs: [
      {
        name: "newBatchPosterManager",
        type: "address",
        indexed: false,
        internalType: "address"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "BatchPosterSet",
    inputs: [
      {
        name: "batchPoster",
        type: "address",
        indexed: false,
        internalType: "address"
      },
      {
        name: "isBatchPoster",
        type: "bool",
        indexed: false,
        internalType: "bool"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "BufferConfigSet",
    inputs: [
      {
        name: "bufferConfig",
        type: "tuple",
        indexed: false,
        internalType: "struct BufferConfig",
        components: [
          {
            name: "threshold",
            type: "uint64",
            internalType: "uint64"
          },
          {
            name: "max",
            type: "uint64",
            internalType: "uint64"
          },
          {
            name: "replenishRateInBasis",
            type: "uint64",
            internalType: "uint64"
          }
        ]
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "FeeTokenPricerSet",
    inputs: [
      {
        name: "feeTokenPricer",
        type: "address",
        indexed: false,
        internalType: "address"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "InboxMessageDelivered",
    inputs: [
      {
        name: "messageNum",
        type: "uint256",
        indexed: true,
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
    name: "InboxMessageDeliveredFromOrigin",
    inputs: [
      {
        name: "messageNum",
        type: "uint256",
        indexed: true,
        internalType: "uint256"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "InvalidateKeyset",
    inputs: [
      {
        name: "keysetHash",
        type: "bytes32",
        indexed: true,
        internalType: "bytes32"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "MaxTimeVariationSet",
    inputs: [
      {
        name: "maxTimeVariation",
        type: "tuple",
        indexed: false,
        internalType: "struct ISequencerInbox.MaxTimeVariation",
        components: [
          {
            name: "delayBlocks",
            type: "uint256",
            internalType: "uint256"
          },
          {
            name: "futureBlocks",
            type: "uint256",
            internalType: "uint256"
          },
          {
            name: "delaySeconds",
            type: "uint256",
            internalType: "uint256"
          },
          {
            name: "futureSeconds",
            type: "uint256",
            internalType: "uint256"
          }
        ]
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "OwnerFunctionCalled",
    inputs: [
      {
        name: "id",
        type: "uint256",
        indexed: true,
        internalType: "uint256"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "SequencerBatchData",
    inputs: [
      {
        name: "batchSequenceNumber",
        type: "uint256",
        indexed: true,
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
    name: "SequencerBatchDelivered",
    inputs: [
      {
        name: "batchSequenceNumber",
        type: "uint256",
        indexed: true,
        internalType: "uint256"
      },
      {
        name: "beforeAcc",
        type: "bytes32",
        indexed: true,
        internalType: "bytes32"
      },
      {
        name: "afterAcc",
        type: "bytes32",
        indexed: true,
        internalType: "bytes32"
      },
      {
        name: "delayedAcc",
        type: "bytes32",
        indexed: false,
        internalType: "bytes32"
      },
      {
        name: "afterDelayedMessagesRead",
        type: "uint256",
        indexed: false,
        internalType: "uint256"
      },
      {
        name: "timeBounds",
        type: "tuple",
        indexed: false,
        internalType: "struct IBridge.TimeBounds",
        components: [
          {
            name: "minTimestamp",
            type: "uint64",
            internalType: "uint64"
          },
          {
            name: "maxTimestamp",
            type: "uint64",
            internalType: "uint64"
          },
          {
            name: "minBlockNumber",
            type: "uint64",
            internalType: "uint64"
          },
          {
            name: "maxBlockNumber",
            type: "uint64",
            internalType: "uint64"
          }
        ]
      },
      {
        name: "dataLocation",
        type: "uint8",
        indexed: false,
        internalType: "enum IBridge.BatchDataLocation"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "SequencerSet",
    inputs: [
      {
        name: "addr",
        type: "address",
        indexed: false,
        internalType: "address"
      },
      {
        name: "isSequencer",
        type: "bool",
        indexed: false,
        internalType: "bool"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "SetValidKeyset",
    inputs: [
      {
        name: "keysetHash",
        type: "bytes32",
        indexed: true,
        internalType: "bytes32"
      },
      {
        name: "keysetBytes",
        type: "bytes",
        indexed: false,
        internalType: "bytes"
      }
    ],
    anonymous: false
  },
  {
    type: "error",
    name: "AlreadyInit",
    inputs: []
  },
  {
    type: "error",
    name: "AlreadyValidDASKeyset",
    inputs: [
      {
        name: "",
        type: "bytes32",
        internalType: "bytes32"
      }
    ]
  },
  {
    type: "error",
    name: "BadBufferConfig",
    inputs: []
  },
  {
    type: "error",
    name: "BadMaxTimeVariation",
    inputs: []
  },
  {
    type: "error",
    name: "BadSequencerNumber",
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
    name: "CannotSetFeeTokenPricer",
    inputs: []
  },
  {
    type: "error",
    name: "DataBlobsNotSupported",
    inputs: []
  },
  {
    type: "error",
    name: "DataTooLarge",
    inputs: [
      {
        name: "dataLength",
        type: "uint256",
        internalType: "uint256"
      },
      {
        name: "maxDataLength",
        type: "uint256",
        internalType: "uint256"
      }
    ]
  },
  {
    type: "error",
    name: "DelayProofRequired",
    inputs: []
  },
  {
    type: "error",
    name: "DelayedBackwards",
    inputs: []
  },
  {
    type: "error",
    name: "DelayedTooFar",
    inputs: []
  },
  {
    type: "error",
    name: "Deprecated",
    inputs: []
  },
  {
    type: "error",
    name: "ExtraGasNotUint64",
    inputs: []
  },
  {
    type: "error",
    name: "ForceIncludeBlockTooSoon",
    inputs: []
  },
  {
    type: "error",
    name: "HadZeroInit",
    inputs: []
  },
  {
    type: "error",
    name: "IncorrectMessagePreimage",
    inputs: []
  },
  {
    type: "error",
    name: "InitParamZero",
    inputs: [
      {
        name: "name",
        type: "string",
        internalType: "string"
      }
    ]
  },
  {
    type: "error",
    name: "InvalidDelayedAccPreimage",
    inputs: []
  },
  {
    type: "error",
    name: "InvalidHeaderFlag",
    inputs: [
      {
        name: "",
        type: "bytes1",
        internalType: "bytes1"
      }
    ]
  },
  {
    type: "error",
    name: "KeysetTooLarge",
    inputs: []
  },
  {
    type: "error",
    name: "MissingDataHashes",
    inputs: []
  },
  {
    type: "error",
    name: "NativeTokenMismatch",
    inputs: []
  },
  {
    type: "error",
    name: "NoSuchKeyset",
    inputs: [
      {
        name: "",
        type: "bytes32",
        internalType: "bytes32"
      }
    ]
  },
  {
    type: "error",
    name: "NotBatchPoster",
    inputs: []
  },
  {
    type: "error",
    name: "NotBatchPosterManager",
    inputs: [
      {
        name: "",
        type: "address",
        internalType: "address"
      }
    ]
  },
  {
    type: "error",
    name: "NotCodelessOrigin",
    inputs: []
  },
  {
    type: "error",
    name: "NotDelayBufferable",
    inputs: []
  },
  {
    type: "error",
    name: "NotForked",
    inputs: []
  },
  {
    type: "error",
    name: "NotOwner",
    inputs: [
      {
        name: "sender",
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
    name: "RollupNotChanged",
    inputs: []
  }
] as const
