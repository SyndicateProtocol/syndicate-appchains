export const allowlistSequencingModuleABI = [
  {
    type: "constructor",
    inputs: [
      {
        name: "_admin",
        type: "address",
        internalType: "address"
      }
    ],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "addToAllowlist",
    inputs: [
      {
        name: "user",
        type: "address",
        internalType: "address"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "admin",
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
    name: "allowlist",
    inputs: [
      {
        name: "user",
        type: "address",
        internalType: "address"
      }
    ],
    outputs: [
      {
        name: "isAllowed",
        type: "bool",
        internalType: "bool"
      }
    ],
    stateMutability: "view"
  },
  {
    type: "function",
    name: "isAllowed",
    inputs: [
      {
        name: "proposer",
        type: "address",
        internalType: "address"
      },
      {
        name: "",
        type: "address",
        internalType: "address"
      },
      {
        name: "",
        type: "bytes",
        internalType: "bytes"
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
    name: "removeFromAllowlist",
    inputs: [
      {
        name: "user",
        type: "address",
        internalType: "address"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "transferAdmin",
    inputs: [
      {
        name: "newAdmin",
        type: "address",
        internalType: "address"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "event",
    name: "AdminTransferred",
    inputs: [
      {
        name: "previousAdmin",
        type: "address",
        indexed: true,
        internalType: "address"
      },
      {
        name: "newAdmin",
        type: "address",
        indexed: true,
        internalType: "address"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "UserAdded",
    inputs: [
      {
        name: "user",
        type: "address",
        indexed: true,
        internalType: "address"
      }
    ],
    anonymous: false
  },
  {
    type: "event",
    name: "UserRemoved",
    inputs: [
      {
        name: "user",
        type: "address",
        indexed: true,
        internalType: "address"
      }
    ],
    anonymous: false
  },
  {
    type: "error",
    name: "AddressNotAllowed",
    inputs: []
  },
  {
    type: "error",
    name: "NotAdmin",
    inputs: []
  }
] as const

export const allowlistSequencingModuleBytecode =
  "0x608034608357601f61048d38819003918201601f19168301916001600160401b03831184841017608757808492602094604052833981010312608357516001600160a01b0381169081900360835780156074575f80546001600160a01b0319169190911790556040516103f1908161009c8239f35b6315a9bc2760e11b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c80635da93d7e1461030657806375829def1461023a5780637a3979dc146101ab578063a7cd52cb14610161578063f851a4401461012f5763f8e86ece1461005b575f80fd5b3461012b57602060031936011261012b576100746103ab565b73ffffffffffffffffffffffffffffffffffffffff5f541633036101035773ffffffffffffffffffffffffffffffffffffffff16805f52600160205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008254161790557f19ef9a4877199f89440a26acb26895ec02ed86f2df1aeaa90dc18041b892f71f5f80a2005b7f7bfa4b9f000000000000000000000000000000000000000000000000000000005f5260045ffd5b5f80fd5b3461012b575f60031936011261012b57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b3461012b57602060031936011261012b5773ffffffffffffffffffffffffffffffffffffffff61018f6103ab565b165f526001602052602060ff60405f2054166040519015158152f35b3461012b57606060031936011261012b576101c46103ab565b6101cc6103ce565b5060443567ffffffffffffffff811161012b573660238201121561012b57806004013567ffffffffffffffff811161012b573691016024011161012b5773ffffffffffffffffffffffffffffffffffffffff165f526001602052602060ff60405f2054166040519015158152f35b3461012b57602060031936011261012b576102536103ab565b5f549073ffffffffffffffffffffffffffffffffffffffff821633036101035773ffffffffffffffffffffffffffffffffffffffff169081156102de577fffffffffffffffffffffffff0000000000000000000000000000000000000000829116175f55337ff8ccb027dfcd135e000e9d45e6cc2d662578a8825d4c45b5e32e0adf67e79ec65f80a3005b7f2b53784e000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461012b57602060031936011261012b5761031f6103ab565b73ffffffffffffffffffffffffffffffffffffffff5f541633036101035773ffffffffffffffffffffffffffffffffffffffff16805f52600160205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0081541690557fe9dce8c992623ce791725b21e857e33248d1f190a25b5168313420eebdaae99d5f80a2005b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361012b57565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361012b5756"
