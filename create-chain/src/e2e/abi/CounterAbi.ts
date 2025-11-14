export const counterAbi = [
  {
    type: "function",
    name: "increment",
    inputs: [],
    outputs: [],
    stateMutability: "nonpayable"
  },
  {
    type: "function",
    name: "number",
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
    name: "setNumber",
    inputs: [
      {
        name: "newNumber",
        type: "uint256",
        internalType: "uint256"
      }
    ],
    outputs: [],
    stateMutability: "nonpayable"
  }
] as const

export const counterBytecode =
  "0x6080806040523460135760d2908160188239f35b5f80fdfe60808060405260043610156011575f80fd5b5f3560e01c9081633fb5c1cb1460865781638381f58a14606f575063d09de08a146039575f80fd5b34606b575f366003190112606b575f545f1981146057576001015f55005b634e487b7160e01b5f52601160045260245ffd5b5f80fd5b34606b575f366003190112606b576020905f548152f35b34606b576020366003190112606b576004355f5500fea26469706673582212209a991cce46984e5cb42733df338874b3b7c6783fd3a823838615f7094c5eb72a64736f6c63430008190033"
