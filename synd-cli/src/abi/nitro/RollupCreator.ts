// v3.1.1
export const rollupCreatorAbi = [
  {
    type: "constructor",
    inputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "receive",
    stateMutability: "payable"
  },
  {
    type: "function",
    name: "bridgeCreator",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "contract BridgeCreator"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "challengeManagerTemplate",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "contract IEdgeChallengeManager"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "createRollup",
    inputs: [
      {
        name: "deployParams",
        type: "tuple",
        internalType: "struct RollupCreator.RollupDeploymentParams",
        components: [
          {
            name: "config",
            type: "tuple",
            internalType: "struct Config",
            components: [
              {
                name: "confirmPeriodBlocks",
                type: "uint64",
                internalType: "uint64"
              },
              {
                name: "stakeToken",
                type: "address",
                internalType: "address"
              },
              {
                name: "baseStake",
                type: "uint256",
                internalType: "uint256"
              },
              {
                name: "wasmModuleRoot",
                type: "bytes32",
                internalType: "bytes32"
              },
              {
                name: "owner",
                type: "address",
                internalType: "address"
              },
              {
                name: "loserStakeEscrow",
                type: "address",
                internalType: "address"
              },
              {
                name: "chainId",
                type: "uint256",
                internalType: "uint256"
              },
              {
                name: "chainConfig",
                type: "string",
                internalType: "string"
              },
              {
                name: "minimumAssertionPeriod",
                type: "uint256",
                internalType: "uint256"
              },
              {
                name: "validatorAfkBlocks",
                type: "uint64",
                internalType: "uint64"
              },
              {
                name: "miniStakeValues",
                type: "uint256[]",
                internalType: "uint256[]"
              },
              {
                name: "sequencerInboxMaxTimeVariation",
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
                name: "layerZeroBlockEdgeHeight",
                type: "uint256",
                internalType: "uint256"
              },
              {
                name: "layerZeroBigStepEdgeHeight",
                type: "uint256",
                internalType: "uint256"
              },
              {
                name: "layerZeroSmallStepEdgeHeight",
                type: "uint256",
                internalType: "uint256"
              },
              {
                name: "genesisAssertionState",
                type: "tuple",
                internalType: "struct AssertionState",
                components: [
                  {
                    name: "globalState",
                    type: "tuple",
                    internalType: "struct GlobalState",
                    components: [
                      {
                        name: "bytes32Vals",
                        type: "bytes32[2]",
                        internalType: "bytes32[2]"
                      },
                      {
                        name: "u64Vals",
                        type: "uint64[2]",
                        internalType: "uint64[2]"
                      }
                    ]
                  },
                  {
                    name: "machineStatus",
                    type: "uint8",
                    internalType: "enum MachineStatus"
                  },
                  {
                    name: "endHistoryRoot",
                    type: "bytes32",
                    internalType: "bytes32"
                  }
                ]
              },
              {
                name: "genesisInboxCount",
                type: "uint256",
                internalType: "uint256"
              },
              {
                name: "anyTrustFastConfirmer",
                type: "address",
                internalType: "address"
              },
              {
                name: "numBigStepLevel",
                type: "uint8",
                internalType: "uint8"
              },
              {
                name: "challengeGracePeriodBlocks",
                type: "uint64",
                internalType: "uint64"
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
              }
            ]
          },
          {
            name: "validators",
            type: "address[]",
            internalType: "address[]"
          },
          {
            name: "maxDataSize",
            type: "uint256",
            internalType: "uint256"
          },
          {
            name: "nativeToken",
            type: "address",
            internalType: "address"
          },
          {
            name: "deployFactoriesToL2",
            type: "bool",
            internalType: "bool"
          },
          {
            name: "maxFeePerGasForRetryables",
            type: "uint256",
            internalType: "uint256"
          },
          {
            name: "batchPosters",
            type: "address[]",
            internalType: "address[]"
          },
          {
            name: "batchPosterManager",
            type: "address",
            internalType: "address"
          },
          {
            name: "feeTokenPricer",
            type: "address",
            internalType: "contract IFeeTokenPricer"
          }
        ]
      }
    ],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "address"
      }
    ],
    stateMutability: "payable"
  },
  {
    type: "function",
    name: "l2FactoriesDeployer",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "contract DeployHelper"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "osp",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "contract IOneStepProofEntry"
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
    name: "rollupAdminLogic",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "contract IRollupAdmin"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "rollupUserLogic",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "contract IRollupUser"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "setTemplates",
    inputs: [
      {
        name: "_bridgeCreator",
        type: "address",
        internalType: "contract BridgeCreator"
      },
      {
        name: "_osp",
        type: "address",
        internalType: "contract IOneStepProofEntry"
      },
      {
        name: "_challengeManagerLogic",
        type: "address",
        internalType: "contract IEdgeChallengeManager"
      },
      {
        name: "_rollupAdminLogic",
        type: "address",
        internalType: "contract IRollupAdmin"
      },
      {
        name: "_rollupUserLogic",
        type: "address",
        internalType: "contract IRollupUser"
      },
      {
        name: "_upgradeExecutorLogic",
        type: "address",
        internalType: "contract IUpgradeExecutor"
      },
      {
        name: "_validatorWalletCreator",
        type: "address",
        internalType: "address"
      },
      {
        name: "_l2FactoriesDeployer",
        type: "address",
        internalType: "contract DeployHelper"
      }
    ],
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
    name: "upgradeExecutorLogic",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "contract IUpgradeExecutor"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "validatorWalletCreator",
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
    name: "RollupCreated",
    inputs: [
      {
        name: "rollupAddress",
        type: "address",
        indexed: true,
        internalType: "address"
      },
      {
        name: "nativeToken",
        type: "address",
        indexed: true,
        internalType: "address"
      },
      {
        name: "inboxAddress",
        type: "address",
        indexed: false,
        internalType: "address"
      },
      {
        name: "outbox",
        type: "address",
        indexed: false,
        internalType: "address"
      },
      {
        name: "rollupEventInbox",
        type: "address",
        indexed: false,
        internalType: "address"
      },
      {
        name: "challengeManager",
        type: "address",
        indexed: false,
        internalType: "address"
      },
      {
        name: "adminProxy",
        type: "address",
        indexed: false,
        internalType: "address"
      },
      {
        name: "sequencerInbox",
        type: "address",
        indexed: false,
        internalType: "address"
      },
      {
        name: "bridge",
        type: "address",
        indexed: false,
        internalType: "address"
      },
      {
        name: "upgradeExecutor",
        type: "address",
        indexed: false,
        internalType: "address"
      },
      {
        name: "validatorWalletCreator",
        type: "address",
        indexed: false,
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
