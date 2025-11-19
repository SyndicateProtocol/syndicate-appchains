export const teeKeyManagerABI = [
  {
    type: "constructor",
    inputs: [
      {
        name: "_attestationDocVerifier",
        type: "address",
        internalType: "contract IAttestationDocVerifier"
      }
    ],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "addKey",
    inputs: [
      {
        name: "_publicValues",
        type: "bytes",
        internalType: "bytes"
      },
      {
        name: "_proofBytes",
        type: "bytes",
        internalType: "bytes"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "attestationDocVerifier",
    inputs: [],
    outputs: [
      {
        name: "",
        type: "address",
        internalType: "contract IAttestationDocVerifier"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "isKeyValid",
    inputs: [
      {
        name: "publicKey",
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
    name: "revokeAllKeys",
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
    name: "updateAttestationDocVerifier",
    inputs: [
      {
        name: "_attestationDocVerifier",
        type: "address",
        internalType: "contract IAttestationDocVerifier"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "event",
    name: "KeyAdded",
    inputs: [
      {
        name: "key",
        type: "address",
        indexed: true,
        internalType: "address"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "KeysRevoked",
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
    type: "error",
    name: "InvalidPublicKey",
    inputs: [
      {
        name: "publicKey",
        type: "address",
        internalType: "address"
      }
    ]
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
