// https://github.com/OffchainLabs/nitro-contracts/blob/c32af127fe6a9124316abebbf756609649ede1f5/test/foundry/fee-token-pricers/OwnerAdjustableExchangeRatePricer.sol
export const OwnerAdjustableExchangeRatePricerABI = [
  {
    type: "constructor",
    inputs: [
      {
        name: "initialExchangeRate",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "exchangeRate",
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
    name: "getExchangeRate",
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
    name: "setExchangeRate",
    inputs: [
      {
        name: "_exchangeRate",
        type: "uint256",
        internalType: "uint256"
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
    type: "event",
    name: "ExchangeRateSet",
    inputs: [
      {
        name: "newExchangeRate",
        type: "uint256",
        indexed: false,
        internalType: "uint256"
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
  }
] as const

export const OwnerAdjustableExchangeRatePricerBytecode =
  "0x6080604052348015600e575f5ffd5b5060405161045c38038061045c833981016040819052602b9160be565b603233606f565b60018190556040518181527f972aba470577c14606bbf4bbdec1fed4925f242fcef40b4a8d242983365d02919060200160405180910390a15060d4565b5f80546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b5f6020828403121560cd575f5ffd5b5051919050565b61037b806100e15f395ff3fe608060405234801561000f575f5ffd5b506004361061006f575f3560e01c8063db068e0e1161004d578063db068e0e146100c0578063e6aa216c146100d3578063f2fde38b146100db575f5ffd5b80633ba0b9a914610073578063715018a61461008f5780638da5cb5b14610099575b5f5ffd5b61007c60015481565b6040519081526020015b60405180910390f35b6100976100ee565b005b5f5460405173ffffffffffffffffffffffffffffffffffffffff9091168152602001610086565b6100976100ce3660046102f4565b610101565b60015461007c565b6100976100e936600461030b565b610144565b6100f6610200565b6100ff5f610280565b565b610109610200565b60018190556040518181527f972aba470577c14606bbf4bbdec1fed4925f242fcef40b4a8d242983365d02919060200160405180910390a150565b61014c610200565b73ffffffffffffffffffffffffffffffffffffffff81166101f4576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f646472657373000000000000000000000000000000000000000000000000000060648201526084015b60405180910390fd5b6101fd81610280565b50565b5f5473ffffffffffffffffffffffffffffffffffffffff1633146100ff576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101eb565b5f805473ffffffffffffffffffffffffffffffffffffffff8381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b5f60208284031215610304575f5ffd5b5035919050565b5f6020828403121561031b575f5ffd5b813573ffffffffffffffffffffffffffffffffffffffff8116811461033e575f5ffd5b939250505056fea26469706673582212202d8c33098cf478e4ed1372682e2b176e55d962af8f0071fb5cac39c71fd1cd2c64736f6c634300081d0033"
