// v3.1.1
export const bridgeCreatorAbi = [
  {
    type: "constructor",
    inputs: [
      {
        name: "_ethBasedTemplates",
        type: "tuple",
        internalType: "struct BridgeCreator.BridgeTemplates",
        components: [
          {
            name: "bridge",
            type: "address",
            internalType: "contract IBridge"
          },
          {
            name: "sequencerInbox",
            type: "address",
            internalType: "contract ISequencerInbox"
          },
          {
            name: "delayBufferableSequencerInbox",
            type: "address",
            internalType: "contract ISequencerInbox"
          },
          {
            name: "inbox",
            type: "address",
            internalType: "contract IInboxBase"
          },
          {
            name: "rollupEventInbox",
            type: "address",
            internalType: "contract IRollupEventInbox"
          },
          {
            name: "outbox",
            type: "address",
            internalType: "contract IOutbox"
          }
        ]
      },
      {
        name: "_erc20BasedTemplates",
        type: "tuple",
        internalType: "struct BridgeCreator.BridgeTemplates",
        components: [
          {
            name: "bridge",
            type: "address",
            internalType: "contract IBridge"
          },
          {
            name: "sequencerInbox",
            type: "address",
            internalType: "contract ISequencerInbox"
          },
          {
            name: "delayBufferableSequencerInbox",
            type: "address",
            internalType: "contract ISequencerInbox"
          },
          {
            name: "inbox",
            type: "address",
            internalType: "contract IInboxBase"
          },
          {
            name: "rollupEventInbox",
            type: "address",
            internalType: "contract IRollupEventInbox"
          },
          {
            name: "outbox",
            type: "address",
            internalType: "contract IOutbox"
          }
        ]
      }
    ],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "createBridge",
    inputs: [
      {
        name: "adminProxy",
        type: "address",
        internalType: "address"
      },
      {
        name: "rollup",
        type: "address",
        internalType: "address"
      },
      {
        name: "nativeToken",
        type: "address",
        internalType: "address"
      },
      {
        name: "maxTimeVariation",
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
        name: "bufferConfig",
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
        name: "feeTokenPricer",
        type: "address",
        internalType: "contract IFeeTokenPricer"
      }
    ],
    outputs: [
      {
        name: "",
        type: "tuple",
        internalType: "struct BridgeCreator.BridgeContracts",
        components: [
          {
            name: "bridge",
            type: "address",
            internalType: "contract IBridge"
          },
          {
            name: "inbox",
            type: "address",
            internalType: "contract IInboxBase"
          },
          {
            name: "sequencerInbox",
            type: "address",
            internalType: "contract ISequencerInbox"
          },
          {
            name: "rollupEventInbox",
            type: "address",
            internalType: "contract IRollupEventInbox"
          },
          {
            name: "outbox",
            type: "address",
            internalType: "contract IOutbox"
          }
        ]
      }
    ],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "erc20BasedTemplates",
    inputs: [],
    outputs: [
      {
        name: "bridge",
        type: "address",
        internalType: "contract IBridge"
      },
      {
        name: "sequencerInbox",
        type: "address",
        internalType: "contract ISequencerInbox"
      },
      {
        name: "delayBufferableSequencerInbox",
        type: "address",
        internalType: "contract ISequencerInbox"
      },
      {
        name: "inbox",
        type: "address",
        internalType: "contract IInboxBase"
      },
      {
        name: "rollupEventInbox",
        type: "address",
        internalType: "contract IRollupEventInbox"
      },
      {
        name: "outbox",
        type: "address",
        internalType: "contract IOutbox"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "ethBasedTemplates",
    inputs: [],
    outputs: [
      {
        name: "bridge",
        type: "address",
        internalType: "contract IBridge"
      },
      {
        name: "sequencerInbox",
        type: "address",
        internalType: "contract ISequencerInbox"
      },
      {
        name: "delayBufferableSequencerInbox",
        type: "address",
        internalType: "contract ISequencerInbox"
      },
      {
        name: "inbox",
        type: "address",
        internalType: "contract IInboxBase"
      },
      {
        name: "rollupEventInbox",
        type: "address",
        internalType: "contract IRollupEventInbox"
      },
      {
        name: "outbox",
        type: "address",
        internalType: "contract IOutbox"
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
    name: "updateERC20Templates",
    inputs: [
      {
        name: "_newTemplates",
        type: "tuple",
        internalType: "struct BridgeCreator.BridgeTemplates",
        components: [
          {
            name: "bridge",
            type: "address",
            internalType: "contract IBridge"
          },
          {
            name: "sequencerInbox",
            type: "address",
            internalType: "contract ISequencerInbox"
          },
          {
            name: "delayBufferableSequencerInbox",
            type: "address",
            internalType: "contract ISequencerInbox"
          },
          {
            name: "inbox",
            type: "address",
            internalType: "contract IInboxBase"
          },
          {
            name: "rollupEventInbox",
            type: "address",
            internalType: "contract IRollupEventInbox"
          },
          {
            name: "outbox",
            type: "address",
            internalType: "contract IOutbox"
          }
        ]
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "updateTemplates",
    inputs: [
      {
        name: "_newTemplates",
        type: "tuple",
        internalType: "struct BridgeCreator.BridgeTemplates",
        components: [
          {
            name: "bridge",
            type: "address",
            internalType: "contract IBridge"
          },
          {
            name: "sequencerInbox",
            type: "address",
            internalType: "contract ISequencerInbox"
          },
          {
            name: "delayBufferableSequencerInbox",
            type: "address",
            internalType: "contract ISequencerInbox"
          },
          {
            name: "inbox",
            type: "address",
            internalType: "contract IInboxBase"
          },
          {
            name: "rollupEventInbox",
            type: "address",
            internalType: "contract IRollupEventInbox"
          },
          {
            name: "outbox",
            type: "address",
            internalType: "contract IOutbox"
          }
        ]
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "event",
    name: "ERC20TemplatesUpdated",
    inputs: [],
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
    type: "event",
    name: "TemplatesUpdated",
    inputs: [],
    anonymous: false
  }
] as const
