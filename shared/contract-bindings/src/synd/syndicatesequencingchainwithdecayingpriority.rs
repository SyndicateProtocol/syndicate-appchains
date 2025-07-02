///Module containing a contract's types and functions.
/**

```solidity
library GasCounter {
    struct GasPeriod { uint256 startTimestamp; uint256 endTimestamp; uint256 totalGasUsed; bool finalized; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod GasCounter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct GasPeriod { uint256 startTimestamp; uint256 endTimestamp; uint256 totalGasUsed; bool finalized; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GasPeriod {
        #[allow(missing_docs)]
        pub startTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub endTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalGasUsed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub finalized: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            bool,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GasPeriod> for UnderlyingRustTuple<'_> {
            fn from(value: GasPeriod) -> Self {
                (
                    value.startTimestamp,
                    value.endTimestamp,
                    value.totalGasUsed,
                    value.finalized,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GasPeriod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    startTimestamp: tuple.0,
                    endTimestamp: tuple.1,
                    totalGasUsed: tuple.2,
                    finalized: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for GasPeriod {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for GasPeriod {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.endTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalGasUsed),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.finalized,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for GasPeriod {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for GasPeriod {
            const NAME: &'static str = "GasPeriod";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "GasPeriod(uint256 startTimestamp,uint256 endTimestamp,uint256 totalGasUsed,bool finalized)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.endTimestamp)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.totalGasUsed)
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.finalized,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GasPeriod {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.endTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalGasUsed,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.finalized,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.endTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalGasUsed,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.finalized,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`GasCounter`](self) contract instance.

See the [wrapper's documentation](`GasCounterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GasCounterInstance<T, P, N> {
        GasCounterInstance::<T, P, N>::new(address, provider)
    }
    /**A [`GasCounter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GasCounter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GasCounterInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for GasCounterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GasCounterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GasCounterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`GasCounter`](self) contract instance.

See the [wrapper's documentation](`GasCounterInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> GasCounterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> GasCounterInstance<T, P, N> {
            GasCounterInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GasCounterInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GasCounterInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library GasCounter {
    struct GasPeriod {
        uint256 startTimestamp;
        uint256 endTimestamp;
        uint256 totalGasUsed;
        bool finalized;
    }
}

interface SyndicateSequencingChainWithDecayingPriority {
    error AlreadyInitialized();
    error InvalidModuleAddress();
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error TransactionOrSenderNotAllowed();

    event GasTracked(uint256 indexed periodIndex, uint256 gasUsed, uint256 gasPrice);
    event NewPeriodStarted(uint256 indexed periodIndex, uint256 startTimestamp);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event PeriodFinalized(uint256 indexed periodIndex, uint256 totalGasUsed, uint256 duration);
    event RequirementModuleUpdated(address indexed newModule);
    event TransactionProcessed(address indexed sender, bytes data);
    event TransactionProcessed(address indexed sender, bytes data, uint256 originalPriority, uint256 timestamp);

    constructor(uint256 _appchainId);

    function PERIOD_DURATION() external view returns (uint256);
    function PRIORITY_DECAY_RATE() external view returns (uint256);
    function appchainId() external view returns (uint256);
    function calculateEffectivePriority(uint256 originalPriority, uint256 submittedTimestamp, uint256 currentTimestamp) external pure returns (uint256);
    function currentPeriodIndex() external view returns (uint256);
    function gasPriceInSynd() external view returns (uint256);
    function gasTrackingInitialized() external view returns (bool);
    function getCurrentPeriod() external view returns (GasCounter.GasPeriod memory period);
    function getCurrentPeriodGasUsed() external view returns (uint256 totalGas);
    function getCurrentPeriodTimeRemaining() external view returns (uint256 timeRemaining);
    function getPeriod(uint256 periodIndex) external view returns (GasCounter.GasPeriod memory period);
    function getTotalGasFees() external view returns (uint256 totalCost);
    function getTotalPeriods() external view returns (uint256 totalPeriods);
    function initialize(address admin, address _permissionRequirementModule) external;
    function isAllowed(address proposer, address originator, bytes memory data) external view returns (bool);
    function isGasTrackingInitialized() external view returns (bool initialized);
    function owner() external view returns (address);
    function periods(uint256) external view returns (uint256 startTimestamp, uint256 endTimestamp, uint256 totalGasUsed, bool finalized);
    function permissionRequirementModule() external view returns (address);
    function prependZeroByte(bytes memory _data) external pure returns (bytes memory);
    function processTransaction(bytes memory data, uint256 priority) external;
    function processTransaction(bytes memory data) external;
    function processTransactionUncompressed(bytes memory data, uint256 priority) external;
    function processTransactionUncompressed(bytes memory data) external;
    function processTransactionsBulk(bytes[] memory data, uint256[] memory priorities) external;
    function processTransactionsBulk(bytes[] memory data) external;
    function renounceOwnership() external;
    function transferOwnership(address newOwner) external;
    function updateRequirementModule(address _newModule) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_appchainId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "PERIOD_DURATION",
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
    "name": "PRIORITY_DECAY_RATE",
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
    "name": "appchainId",
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
    "name": "calculateEffectivePriority",
    "inputs": [
      {
        "name": "originalPriority",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "submittedTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "currentTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "currentPeriodIndex",
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
    "name": "gasPriceInSynd",
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
    "name": "gasTrackingInitialized",
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
    "name": "getCurrentPeriod",
    "inputs": [],
    "outputs": [
      {
        "name": "period",
        "type": "tuple",
        "internalType": "struct GasCounter.GasPeriod",
        "components": [
          {
            "name": "startTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "endTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "totalGasUsed",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "finalized",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCurrentPeriodGasUsed",
    "inputs": [],
    "outputs": [
      {
        "name": "totalGas",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCurrentPeriodTimeRemaining",
    "inputs": [],
    "outputs": [
      {
        "name": "timeRemaining",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPeriod",
    "inputs": [
      {
        "name": "periodIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "period",
        "type": "tuple",
        "internalType": "struct GasCounter.GasPeriod",
        "components": [
          {
            "name": "startTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "endTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "totalGasUsed",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "finalized",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalGasFees",
    "inputs": [],
    "outputs": [
      {
        "name": "totalCost",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalPeriods",
    "inputs": [],
    "outputs": [
      {
        "name": "totalPeriods",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "admin",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_permissionRequirementModule",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isAllowed",
    "inputs": [
      {
        "name": "proposer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "originator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
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
    "name": "isGasTrackingInitialized",
    "inputs": [],
    "outputs": [
      {
        "name": "initialized",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
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
    "name": "periods",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "startTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "endTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "totalGasUsed",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "finalized",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "permissionRequirementModule",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPermissionModule"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "prependZeroByte",
    "inputs": [
      {
        "name": "_data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "processTransaction",
    "inputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "priority",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "processTransaction",
    "inputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "processTransactionUncompressed",
    "inputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "priority",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "processTransactionUncompressed",
    "inputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "processTransactionsBulk",
    "inputs": [
      {
        "name": "data",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "priorities",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "processTransactionsBulk",
    "inputs": [
      {
        "name": "data",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
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
    "name": "updateRequirementModule",
    "inputs": [
      {
        "name": "_newModule",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "GasTracked",
    "inputs": [
      {
        "name": "periodIndex",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "gasUsed",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "gasPrice",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NewPeriodStarted",
    "inputs": [
      {
        "name": "periodIndex",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "startTimestamp",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PeriodFinalized",
    "inputs": [
      {
        "name": "periodIndex",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "totalGasUsed",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "duration",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RequirementModuleUpdated",
    "inputs": [
      {
        "name": "newModule",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TransactionProcessed",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TransactionProcessed",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "originalPriority",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "timestamp",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AlreadyInitialized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidModuleAddress",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OwnableInvalidOwner",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "OwnableUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "TransactionOrSenderNotAllowed",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod SyndicateSequencingChainWithDecayingPriority {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a060405234610038576100196100146100e9565b61010a565b61002161003d565b61267561053f823960805181610d84015261267590f35b610043565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b9061006f90610047565b810190811060018060401b0382111761008757604052565b610051565b9061009f61009861003d565b9283610065565b565b5f80fd5b90565b6100b1816100a5565b036100b857565b5f80fd5b905051906100c9826100a8565b565b906020828203126100e4576100e1915f016100bc565b90565b6100a1565b610107612d73803803806100fc8161008c565b9283398101906100cb565b90565b610113906101c2565b565b90565b90565b61012f61012a61013492610115565b610118565b6100a5565b90565b60209181520190565b5f7f41707020636861696e2049442063616e6e6f7420626520300000000000000000910152565b6101746018602092610137565b61017d81610140565b0190565b6101969060208101905f818303910152610167565b90565b156101a057565b6101a861003d565b62461bcd60e51b8152806101be60048201610181565b0390fd5b6101ca610249565b6101e7816101e06101da5f61011b565b916100a5565b1415610199565b608052565b5f1b90565b906101fd5f19916101ec565b9181191691161790565b90565b61021e61021961022392610207565b610118565b6100a5565b90565b90565b9061023e6102396102459261020a565b610226565b82546101f1565b9055565b610251610353565b610260633b9aca006002610229565b565b60a01b90565b9061027760ff60a01b91610262565b9181191691161790565b151590565b61028f90610281565b90565b90565b906102aa6102a56102b192610286565b610292565b8254610268565b9055565b5f0190565b6102c261003d565b3d5f823e3d90fd5b60018060a01b031690565b6102e96102e46102ee926102ca565b610118565b6102ca565b90565b6102fa906102d5565b90565b610306906102f1565b90565b9061031a60018060a01b03916101ec565b9181191691161790565b61032d906102f1565b90565b90565b9061034861034361034f92610324565b610330565b8254610309565b9055565b61035c336103c0565b6103675f6001610295565b61036f61003d565b6101bf810181811060018060401b038211176103bb5761039782916101bf612bb484396102b5565b03905ff080156103b6576103ad6103b4916102fd565b6001610333565b565b6102ba565b610051565b6103c990610421565b565b6103df6103da6103e492610115565b610118565b6102ca565b90565b6103f0906103cb565b90565b6103fc906102ca565b90565b610408906103f3565b9052565b919061041f905f602085019401906103ff565b565b8061043c6104366104315f6103e7565b6103f3565b916103f3565b1461044c5761044a906104df565b565b61046f6104585f6103e7565b5f918291631e4fbdf760e01b83526004830161040c565b0390fd5b5f1c90565b60018060a01b031690565b61048f61049491610473565b610478565b90565b6104a19054610483565b90565b6104ad906102d5565b90565b6104b9906104a4565b90565b90565b906104d46104cf6104db926104b0565b6104bc565b8254610309565b9055565b6104e85f610497565b6104f2825f6104bf565b906105266105207f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0936104b0565b916104b0565b9161052f61003d565b80610539816102b5565b0390a356fe60806040526004361015610013575b610f3e565b61001d5f356101ec565b8063050ec138146101e7578063086146d2146101e257806311992f8c146101dd57806318d5aafe146101d85780631c0b6367146101d3578063366cbab7146101ce5780633b6ab2a9146101c95780633d44ae8b146101c457806346e2cc09146101bf578063485cc955146101ba5780634b2c0706146101b55780635b3cd6e2146101b057806361543801146101ab5780636558954f146101a6578063703cfcbb146101a1578063715018a61461019c5780637a3979dc14610197578063804e51231461019257806382f44ade1461018d57806383d3c115146101885780638d5a239b146101835780638da5cb5b1461017e578063aff74c6d14610179578063c660d3f314610174578063cdafb9781461016f578063d4f0eb4d1461016a578063d878134214610165578063ea4a1104146101605763f2fde38b0361000e57610f0b565b610ed2565b610da6565b610d4f565b610cfd565b610c92565b610c5d565b610c28565b610bd1565b610b9b565b610b2c565b610af8565b610abf565b610a3a565b610a05565b6109c1565b610953565b6108e6565b61081b565b6107c9565b61072e565b6106f9565b610668565b6105f3565b61051e565b6104e9565b610490565b61037e565b6102bf565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f8301121561024a5781359167ffffffffffffffff831161024557602001926001830284011161024057565b61020c565b610208565b610204565b90565b61025b8161024f565b0361026257565b5f80fd5b9050359061027382610252565b565b916040838303126102b5575f83013567ffffffffffffffff81116102b0576102a2836102ad928601610210565b939094602001610266565b90565b610200565b6101fc565b5f0190565b346102ee576102d86102d2366004610275565b9161101e565b6102e06101f2565b806102ea816102ba565b0390f35b6101f8565b5f9103126102fd57565b6101fc565b61030b9061024f565b9052565b151590565b61031d9061030f565b9052565b90606080610367936103395f8201515f860190610302565b61034b60208201516020860190610302565b61035d60408201516040860190610302565b0151910190610314565b565b919061037c905f60808501940190610321565b565b346103ae5761038e3660046102f3565b6103aa610399611198565b6103a16101f2565b91829182610369565b0390f35b6101f8565b909182601f830112156103ed5781359167ffffffffffffffff83116103e85760200192602083028401116103e357565b61020c565b610208565b610204565b909182601f8301121561042c5781359167ffffffffffffffff831161042757602001926020830284011161042257565b61020c565b610208565b610204565b909160408284031261048b575f82013567ffffffffffffffff8111610486578361045c9184016103b3565b929093602082013567ffffffffffffffff81116104815761047d92016103f2565b9091565b610200565b610200565b6101fc565b346104c2576104ac6104a3366004610431565b929190916113a1565b6104b46101f2565b806104be816102ba565b0390f35b6101f8565b6104d09061030f565b9052565b91906104e7905f602085019401906104c7565b565b34610519576104f93660046102f3565b6105156105046114a5565b61050c6101f2565b918291826104d4565b0390f35b6101f8565b3461054d57610537610531366004610275565b916115b2565b61053f6101f2565b80610549816102ba565b0390f35b6101f8565b90602082820312610583575f82013567ffffffffffffffff811161057e5761057a9201610210565b9091565b610200565b6101fc565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6105c96105d26020936105d7936105c081610588565b9384809361058c565b95869101610595565b6105a0565b0190565b6105f09160208201915f8184039101526105aa565b90565b346106245761062061060f610609366004610552565b90611642565b6106176101f2565b918291826105db565b0390f35b6101f8565b1c90565b60ff1690565b6106439060086106489302610629565b61062d565b90565b906106569154610633565b90565b61066560045f9061064b565b90565b34610698576106783660046102f3565b610694610683610659565b61068b6101f2565b918291826104d4565b0390f35b6101f8565b90565b90565b6106b76106b26106bc9261069d565b6106a0565b61024f565b90565b6106c9600a6106a3565b90565b6106d46106bf565b90565b6106e09061024f565b9052565b91906106f7905f602085019401906106d7565b565b34610729576107093660046102f3565b6107256107146106cc565b61071c6101f2565b918291826106e4565b0390f35b6101f8565b3461075d57610747610741366004610552565b906117dc565b61074f6101f2565b80610759816102ba565b0390f35b6101f8565b60018060a01b031690565b61077690610762565b90565b6107828161076d565b0361078957565b5f80fd5b9050359061079a82610779565b565b91906040838203126107c457806107b86107c1925f860161078d565b9360200161078d565b90565b6101fc565b346107f8576107e26107dc36600461079c565b9061198d565b6107ea6101f2565b806107f4816102ba565b0390f35b6101f8565b9060208282031261081657610813915f01610266565b90565b6101fc565b3461084b576108476108366108313660046107fd565b611999565b61083e6101f2565b91829182610369565b0390f35b6101f8565b60018060a01b031690565b61086b9060086108709302610629565b610850565b90565b9061087e915461085b565b90565b61088d60015f90610873565b90565b6108a461089f6108a992610762565b6106a0565b610762565b90565b6108b590610890565b90565b6108c1906108ac565b90565b6108cd906108b8565b9052565b91906108e4905f602085019401906108c4565b565b34610916576108f63660046102f3565b610912610901610881565b6109096101f2565b918291826108d1565b0390f35b6101f8565b90565b61092e9060086109339302610629565b61091b565b90565b90610941915461091e565b90565b61095060035f90610936565b90565b34610983576109633660046102f3565b61097f61096e610944565b6109766101f2565b918291826106e4565b0390f35b6101f8565b90565b61099f61099a6109a492610988565b6106a0565b61024f565b90565b6109b362278d0061098b565b90565b6109be6109a7565b90565b346109f1576109d13660046102f3565b6109ed6109dc6109b6565b6109e46101f2565b918291826106e4565b0390f35b6101f8565b610a0260025f90610936565b90565b34610a3557610a153660046102f3565b610a31610a206109f6565b610a286101f2565b918291826106e4565b0390f35b6101f8565b34610a6857610a4a3660046102f3565b610a526119dd565b610a5a6101f2565b80610a64816102ba565b0390f35b6101f8565b91606083830312610aba57610a84825f850161078d565b92610a92836020830161078d565b92604082013567ffffffffffffffff8111610ab557610ab19201610210565b9091565b610200565b6101fc565b34610af357610aef610ade610ad5366004610a6d565b92919091611a95565b610ae66101f2565b918291826104d4565b0390f35b6101f8565b34610b2757610b11610b0b366004610552565b90611bf8565b610b196101f2565b80610b23816102ba565b0390f35b6101f8565b34610b5c57610b3c3660046102f3565b610b58610b47611c15565b610b4f6101f2565b918291826106e4565b0390f35b6101f8565b9091606082840312610b9657610b93610b7c845f8501610266565b93610b8a8160208601610266565b93604001610266565b90565b6101fc565b34610bcc57610bc8610bb7610bb1366004610b61565b91611ce2565b610bbf6101f2565b918291826106e4565b0390f35b6101f8565b34610c0157610be13660046102f3565b610bfd610bec611d58565b610bf46101f2565b918291826106e4565b0390f35b6101f8565b610c0f9061076d565b9052565b9190610c26905f60208501940190610c06565b565b34610c5857610c383660046102f3565b610c54610c43611dea565b610c4b6101f2565b91829182610c13565b0390f35b6101f8565b34610c8d57610c6d3660046102f3565b610c89610c78611e1e565b610c806101f2565b918291826106e4565b0390f35b6101f8565b34610cc257610ca23660046102f3565b610cbe610cad611e6a565b610cb56101f2565b918291826106e4565b0390f35b6101f8565b90602082820312610cf8575f82013567ffffffffffffffff8111610cf357610cef92016103b3565b9091565b610200565b6101fc565b34610d2c57610d16610d10366004610cc7565b90611fb6565b610d1e6101f2565b80610d28816102ba565b0390f35b6101f8565b90602082820312610d4a57610d47915f0161078d565b90565b6101fc565b34610d7d57610d67610d62366004610d31565b612066565b610d6f6101f2565b80610d79816102ba565b0390f35b6101f8565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610dd657610db63660046102f3565b610dd2610dc1610d82565b610dc96101f2565b918291826106e4565b0390f35b6101f8565b610def610dea610df49261024f565b6106a0565b61024f565b90565b90610e0190610ddb565b5f5260205260405f2090565b5f1c90565b610e1e610e2391610e0d565b61091b565b90565b610e309054610e12565b90565b610e3f610e4491610e0d565b61062d565b90565b610e519054610e33565b90565b610e5f906005610df7565b90610e6b5f8301610e26565b91610e7860018201610e26565b91610e916003610e8a60028501610e26565b9301610e47565b90565b610ec9610ed094610ebf606094989795610eb5608086019a5f8701906106d7565b60208501906106d7565b60408301906106d7565b01906104c7565b565b34610f0657610f02610eed610ee83660046107fd565b610e54565b90610ef99492946101f2565b94859485610e94565b0390f35b6101f8565b34610f3957610f23610f1e366004610d31565b6120d6565b610f2b6101f2565b80610f35816102ba565b0390f35b6101f8565b5f80fd5b9190610f5f610f5933329086859192909192611a95565b1561030f565b610f6e57610f6c92610fcb565b565b5f631b8e828b60e31b815280610f86600482016102ba565b0390fd5b610f93906108ac565b90565b604090610fc2610fb7610fc99597969460608401908482035f8601526105aa565b9660208301906106d7565b01906106d7565b565b90610fd7903392611642565b9142926110196110077f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f294610f8a565b946110106101f2565b93849384610f96565b0390a2565b906110299291610f42565b565b634e487b7160e01b5f52604160045260245ffd5b90611049906105a0565b810190811067ffffffffffffffff82111761106357604052565b61102b565b9061107b6110746101f2565b928361103f565b565b6110876080611068565b90565b5f90565b5f90565b61109a61107d565b906020808080856110a961108a565b8152016110b461108a565b8152016110bf61108a565b8152016110ca61108e565b81525050565b6110d8611092565b90565b6110e56080611068565b90565b90565b6110ff6110fa611104926110e8565b6106a0565b61024f565b90565b906111119061024f565b9052565b9061111f9061030f565b9052565b9061118a611181600361113461107d565b9461114b6111435f8301610e26565b5f8801611107565b61116361115a60018301610e26565b60208801611107565b61117b61117260028301610e26565b60408801611107565b01610e47565b60608401611115565b565b61119590611123565b90565b6111a06110d0565b506111b46111ae6004610e47565b1561030f565b6111d8576111d56111d060056111ca6003610e26565b90610df7565b61118c565b90565b5f61122d5f61122461121b5f61121661120d5f956112086112006111fa6110db565b9a6110eb565b5f8b01611107565b6110eb565b60208801611107565b6110eb565b60408501611107565b60608301611115565b90565b5090565b5090565b60209181520190565b60207f7665207468652073616d65206c656e6774680000000000000000000000000000917f4461746120616e64207072696f7269747920617272617973206d7573742068615f8201520152565b61129b6032604092611238565b6112a481611241565b0190565b6112bd9060208101905f81830391015261128e565b90565b156112c757565b6112cf6101f2565b62461bcd60e51b8152806112e5600482016112a8565b0390fd5b60016112f5910161024f565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561135a570180359067ffffffffffffffff82116113555760200191600182023603831361135057565b611314565b611310565b61130c565b9082101561137a5760206113769202810190611318565b9091565b6112f8565b919081101561138f576020020190565b6112f8565b3561139e81610252565b90565b90926113ae828590611230565b936113d5856113cf6113c96113c4888790611234565b61024f565b9161024f565b146112c0565b6113de5f6110eb565b5b806113f26113ec8861024f565b9161024f565b1015611499576114209061141633329061140e8887869161135f565b929091611a95565b611425575b6112e9565b6113df565b3361143b6114358786859161135f565b90611642565b9061145061144b8988869161137f565b611394565b429261149161147f7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f294610f8a565b946114886101f2565b93849384610f96565b0390a261141b565b505050505050565b5f90565b6114ad6114a1565b506114b86004610e47565b90565b91906114d86114d233329086859192909192611a95565b1561030f565b6114e7576114e592611566565b565b5f631b8e828b60e31b8152806114ff600482016102ba565b0390fd5b90825f939282370152565b9190611528816115218161152d9561058c565b8095611503565b6105a0565b0190565b61155d6115526040936115649698979560608501918583035f87015261150e565b9660208301906106d7565b01906106d7565b565b909133919290926115ad4261159b7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f295610f8a565b956115a46101f2565b94859485611531565b0390a2565b906115bd92916114bb565b565b606090565b60ff60f81b1690565b60f81b90565b6115e76115e26115ec926110e8565b6115cd565b6115c4565b90565b90565b6115fe611603916115c4565b6115ef565b9052565b905090565b90918261161c8161162393611607565b8093611503565b0190565b8061163860019261163f96946115f2565b019161160c565b90565b6116809061164e6115bf565b5061167161165b5f6115d3565b91936116656101f2565b94859360208501611627565b6020820181038252038261103f565b90565b9061169f61169933329085859192909192611a95565b1561030f565b6116ae576116ac91611747565b565b5f631b8e828b60e31b8152806116c6600482016102ba565b0390fd5b634e487b7160e01b5f52601160045260245ffd5b6116ed6116f39193929361024f565b9261024f565b82039182116116fe57565b6116ca565b90565b61171a61171561171f92611703565b6106a0565b61024f565b90565b6117316117379193929361024f565b9261024f565b820180921161174257565b6116ca565b6117636117749161175c611779945a92611795565b5a906116de565b61176e611388611706565b90611722565b61213d565b565b90916117929260208301925f81850391015261150e565b90565b3390916117c27f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92610f8a565b926117d76117ce6101f2565b9283928361177b565b0390a2565b906117e691611683565b565b906117fa916117f56121ed565b611900565b565b60a01c90565b61180e611813916117fc565b61062d565b90565b6118209054611802565b90565b61183761183261183c926110e8565b6106a0565b610762565b90565b61184890611823565b90565b60a01b90565b9061186060ff60a01b9161184b565b9181191691161790565b6118739061030f565b90565b90565b9061188e6118896118959261186a565b611876565b8254611851565b9055565b6118a290610890565b90565b6118ae90611899565b90565b5f1b90565b906118c760018060a01b03916118b1565b9181191691161790565b6118da90611899565b90565b90565b906118f56118f06118fc926118d1565b6118dd565b82546118b6565b9055565b61190a6001611816565b611972578161192961192361191e5f61183f565b61076d565b9161076d565b146119565761194f61194861195493611943600180611879565b6118a5565b60016118e0565b6120d6565b565b5f632e7f3c7f60e11b81528061196e600482016102ba565b0390fd5b5f62dc149f60e41b815280611989600482016102ba565b0390fd5b90611997916117e8565b565b6119b06119b5916119a86110d0565b506005610df7565b61118c565b90565b6119c06121ed565b6119c86119ca565b565b6119db6119d65f61183f565b61225e565b565b6119e56119b8565b565b6119f36119f891610e0d565b610850565b90565b611a0590546119e7565b90565b60e01b90565b611a178161030f565b03611a1e57565b5f80fd5b90505190611a2f82611a0e565b565b90602082820312611a4a57611a47915f01611a22565b90565b6101fc565b611a75611a829593949294611a6b60608401965f850190610c06565b6020830190610c06565b604081850391015261150e565b90565b611a8d6101f2565b3d5f823e3d90fd5b92611ad860209394611aa56114a1565b50611ae3611abb611ab660016119fb565b6108b8565b93637a3979dc929597611acc6101f2565b98899788968796611a08565b865260048601611a4f565b03915afa908115611b27575f91611af9575b5090565b611b1a915060203d8111611b20575b611b12818361103f565b810190611a31565b5f611af5565b503d611b08565b611a85565b90611b48611b4233329085859192909192611a95565b1561030f565b611b5757611b5591611b73565b565b5f631b8e828b60e31b815280611b6f600482016102ba565b0390fd5b611b8f611ba091611b88611ba5945a92611ba7565b5a906116de565b611b9a611388611706565b90611722565b61213d565b565b90611bb3903392611642565b90611bf3611be17f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92610f8a565b92611bea6101f2565b918291826105db565b0390a2565b90611c0291611b2c565b565b5f90565b611c12905161024f565b90565b611c1d611c04565b50611c31611c2b6004610e47565b1561030f565b611ca157611c6d611c5f5f611c59611c546005611c4e6003610e26565b90610df7565b61118c565b01611c08565b611c676109a7565b90611722565b42611c80611c7a8361024f565b9161024f565b1015611c9457611c919042906116de565b90565b50611c9e5f6110eb565b90565b611caa5f6110eb565b90565b611cbc611cc29193929361024f565b9261024f565b91611cce83820261024f565b928184041490151715611cdd57565b6116ca565b91611ceb611c04565b5080611cff611cf98461024f565b9161024f565b1115611d5357611d2091611d12916116de565b611d1a6106bf565b90611cad565b80611d33611d2d8461024f565b9161024f565b1015611d4557611d42916116de565b90565b5050611d505f6110eb565b90565b505090565b611d60611c04565b50611d74611d6e6004610e47565b1561030f565b611dae57611dab611d9b6002611d956005611d8f6003610e26565b90610df7565b01610e26565b611da56002610e26565b90611cad565b90565b611db75f6110eb565b90565b5f90565b60018060a01b031690565b611dd5611dda91610e0d565b611dbe565b90565b611de79054611dc9565b90565b611df2611dba565b50611dfc5f611ddd565b90565b90565b611e16611e11611e1b92611dff565b6106a0565b61024f565b90565b611e26611c04565b50611e3a611e346004610e47565b1561030f565b611e5e57611e5b611e4b6003610e26565b611e556001611e02565b90611722565b90565b611e675f6110eb565b90565b611e72611c04565b50611e86611e806004610e47565b1561030f565b611ead57611eaa6002611ea46005611e9e6003610e26565b90610df7565b01610e26565b90565b611eb65f6110eb565b90565b611ed5611ee691611ece611eeb945a92611eed565b5a906116de565b611ee0611388611706565b90611722565b61213d565b565b611ef8818390611230565b91611f01611c04565b50611f0b5f6110eb565b5b80611f1f611f198661024f565b9161024f565b1015611fb057611f4d90611f43333290611f3b8787869161135f565b929091611a95565b611f52575b6112e9565b611f0c565b33611f68611f628686859161135f565b90611642565b90611fa8611f967f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92610f8a565b92611f9f6101f2565b918291826105db565b0390a2611f48565b50505050565b90611fc091611eb9565b565b611fd390611fce6121ed565b611fd5565b565b80611ff0611fea611fe55f61183f565b61076d565b9161076d565b1461204a57612008612001826118a5565b60016118e0565b6120327f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b991610f8a565b9061203b6101f2565b80612045816102ba565b0390a2565b5f632e7f3c7f60e11b815280612062600482016102ba565b0390fd5b61206f90611fc2565b565b6120829061207d6121ed565b612084565b565b8061209f6120996120945f61183f565b61076d565b9161076d565b146120af576120ad9061225e565b565b6120d26120bb5f61183f565b5f918291631e4fbdf760e01b835260048301610c13565b0390fd5b6120df90612071565b565b906120ed5f19916118b1565b9181191691161790565b90565b9061210f61210a61211692610ddb565b6120f7565b82546120e1565b9055565b91602061213b92949361213460408201965f8301906106d7565b01906106d7565b565b61215061214a6004610e47565b1561030f565b6121e0575b61215d61248a565b6121918161218b600261217b60056121756003610e26565b90610df7565b019161218683610e26565b611722565b906120fa565b61219b6003610e26565b3a6121c67f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610ddb565b926121db6121d26101f2565b9283928361211a565b0390a2565b6121e8612387565b612155565b6121f5611dea565b61220e612208612203612668565b61076d565b9161076d565b0361221557565b612237612220612668565b5f91829163118cdaa760e01b835260048301610c13565b0390fd5b90565b9061225361224e61225a92610f8a565b61223b565b82546118b6565b9055565b6122675f611ddd565b612271825f61223e565b906122a561229f7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093610f8a565b91610f8a565b916122ae6101f2565b806122b8816102ba565b0390a3565b906122c960ff916118b1565b9181191691161790565b906122e86122e36122ef9261186a565b611876565b82546122bd565b9055565b906122fd906110eb565b5f5260205260405f2090565b612313905161030f565b90565b9061237360606003612379946123395f82016123335f8801611c08565b906120fa565b6123526001820161234c60208801611c08565b906120fa565b61236b6002820161236560408801611c08565b906120fa565b019201612309565b906122d3565b565b9061238591612316565b565b61239a6123946004610e47565b1561030f565b6123a1575b565b6123ad600160046122d3565b6123c06123b95f6110eb565b60036120fa565b612421426124105f6124076123fe5f6123f96123f05f956123eb6123e26110db565b995f8b01611107565b6110eb565b60208801611107565b6110eb565b60408501611107565b60608301611115565b61241c60055f906122f3565b61237b565b5f42906124636124517f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e926110eb565b9261245a6101f2565b918291826106e4565b0390a261239f565b90565b6124779061024f565b5f1981146124855760010190565b6116ca565b6124a76124a2600561249c6003610e26565b90610df7565b61246b565b426124d56124cf6124ca6124bc5f8601610e26565b6124c46109a7565b90611722565b61024f565b9161024f565b10156124df575b50565b6125076124fe6124f05f8401610e26565b6124f86109a7565b90611722565b600183016120fa565b6125156001600383016122d3565b61251f6003610e26565b61254c61252e60028401610e26565b926125465f61253f60018401610e26565b9201610e26565b906116de565b6125767f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610ddb565b9261258b6125826101f2565b9283928361211a565b0390a26125aa6125a361259e6003610e26565b61246e565b60036120fa565b612614426125fa5f6125f16125e85f6125e36125da5f956125d56125cc6110db565b995f8b01611107565b6110eb565b60208801611107565b6110eb565b60408501611107565b60608301611115565b61260f60056126096003610e26565b90610df7565b61237b565b61261e6003610e26565b429061265f61264d7f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610ddb565b926126566101f2565b918291826106e4565b0390a25f6124dc565b612670611dba565b50339056608060405234601c57600e6020565b61019461002b823961019490f35b6026565b60405190565b5f80fdfe60806040526004361015610013575b61017b565b61001d5f3561002c565b637a3979dc0361000e57610142565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b60018060a01b031690565b61005890610044565b90565b6100648161004f565b0361006b57565b5f80fd5b9050359061007c8261005b565b565b5f80fd5b5f80fd5b5f80fd5b909182601f830112156100c45781359167ffffffffffffffff83116100bf5760200192600183028401116100ba57565b610086565b610082565b61007e565b91606083830312610116576100e0825f850161006f565b926100ee836020830161006f565b92604082013567ffffffffffffffff81116101115761010d920161008a565b9091565b610040565b61003c565b151590565b6101299061011b565b9052565b9190610140905f60208501940190610120565b565b34610176576101726101616101583660046100c9565b92919091610183565b610169610032565b9182918261012d565b0390f35b610038565b5f80fd5b5f90565b5050505061018f61017f565b505f9056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4a\08Wa\0\x19a\0\x14a\0\xE9V[a\x01\nV[a\0!a\0=V[a&ua\x05?\x829`\x80Q\x81a\r\x84\x01Ra&u\x90\xF3[a\0CV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0o\x90a\0GV[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x87W`@RV[a\0QV[\x90a\0\x9Fa\0\x98a\0=V[\x92\x83a\0eV[V[_\x80\xFD[\x90V[a\0\xB1\x81a\0\xA5V[\x03a\0\xB8WV[_\x80\xFD[\x90PQ\x90a\0\xC9\x82a\0\xA8V[V[\x90` \x82\x82\x03\x12a\0\xE4Wa\0\xE1\x91_\x01a\0\xBCV[\x90V[a\0\xA1V[a\x01\x07a-s\x808\x03\x80a\0\xFC\x81a\0\x8CV[\x92\x839\x81\x01\x90a\0\xCBV[\x90V[a\x01\x13\x90a\x01\xC2V[V[\x90V[\x90V[a\x01/a\x01*a\x014\x92a\x01\x15V[a\x01\x18V[a\0\xA5V[\x90V[` \x91\x81R\x01\x90V[_\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01t`\x18` \x92a\x017V[a\x01}\x81a\x01@V[\x01\x90V[a\x01\x96\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x01gV[\x90V[\x15a\x01\xA0WV[a\x01\xA8a\0=V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x01\xBE`\x04\x82\x01a\x01\x81V[\x03\x90\xFD[a\x01\xCAa\x02IV[a\x01\xE7\x81a\x01\xE0a\x01\xDA_a\x01\x1BV[\x91a\0\xA5V[\x14\x15a\x01\x99V[`\x80RV[_\x1B\x90V[\x90a\x01\xFD_\x19\x91a\x01\xECV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[a\x02\x1Ea\x02\x19a\x02#\x92a\x02\x07V[a\x01\x18V[a\0\xA5V[\x90V[\x90V[\x90a\x02>a\x029a\x02E\x92a\x02\nV[a\x02&V[\x82Ta\x01\xF1V[\x90UV[a\x02Qa\x03SV[a\x02`c;\x9A\xCA\0`\x02a\x02)V[V[`\xA0\x1B\x90V[\x90a\x02w`\xFF`\xA0\x1B\x91a\x02bV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x15\x15\x90V[a\x02\x8F\x90a\x02\x81V[\x90V[\x90V[\x90a\x02\xAAa\x02\xA5a\x02\xB1\x92a\x02\x86V[a\x02\x92V[\x82Ta\x02hV[\x90UV[_\x01\x90V[a\x02\xC2a\0=V[=_\x82>=\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x02\xE9a\x02\xE4a\x02\xEE\x92a\x02\xCAV[a\x01\x18V[a\x02\xCAV[\x90V[a\x02\xFA\x90a\x02\xD5V[\x90V[a\x03\x06\x90a\x02\xF1V[\x90V[\x90a\x03\x1A`\x01\x80`\xA0\x1B\x03\x91a\x01\xECV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x03-\x90a\x02\xF1V[\x90V[\x90V[\x90a\x03Ha\x03Ca\x03O\x92a\x03$V[a\x030V[\x82Ta\x03\tV[\x90UV[a\x03\\3a\x03\xC0V[a\x03g_`\x01a\x02\x95V[a\x03oa\0=V[a\x01\xBF\x81\x01\x81\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\x03\xBBWa\x03\x97\x82\x91a\x01\xBFa+\xB4\x849a\x02\xB5V[\x03\x90_\xF0\x80\x15a\x03\xB6Wa\x03\xADa\x03\xB4\x91a\x02\xFDV[`\x01a\x033V[V[a\x02\xBAV[a\0QV[a\x03\xC9\x90a\x04!V[V[a\x03\xDFa\x03\xDAa\x03\xE4\x92a\x01\x15V[a\x01\x18V[a\x02\xCAV[\x90V[a\x03\xF0\x90a\x03\xCBV[\x90V[a\x03\xFC\x90a\x02\xCAV[\x90V[a\x04\x08\x90a\x03\xF3V[\x90RV[\x91\x90a\x04\x1F\x90_` \x85\x01\x94\x01\x90a\x03\xFFV[V[\x80a\x04<a\x046a\x041_a\x03\xE7V[a\x03\xF3V[\x91a\x03\xF3V[\x14a\x04LWa\x04J\x90a\x04\xDFV[V[a\x04oa\x04X_a\x03\xE7V[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x04\x0CV[\x03\x90\xFD[_\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\x8Fa\x04\x94\x91a\x04sV[a\x04xV[\x90V[a\x04\xA1\x90Ta\x04\x83V[\x90V[a\x04\xAD\x90a\x02\xD5V[\x90V[a\x04\xB9\x90a\x04\xA4V[\x90V[\x90V[\x90a\x04\xD4a\x04\xCFa\x04\xDB\x92a\x04\xB0V[a\x04\xBCV[\x82Ta\x03\tV[\x90UV[a\x04\xE8_a\x04\x97V[a\x04\xF2\x82_a\x04\xBFV[\x90a\x05&a\x05 \x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x04\xB0V[\x91a\x04\xB0V[\x91a\x05/a\0=V[\x80a\x059\x81a\x02\xB5V[\x03\x90\xA3V\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x0F>V[a\0\x1D_5a\x01\xECV[\x80c\x05\x0E\xC18\x14a\x01\xE7W\x80c\x08aF\xD2\x14a\x01\xE2W\x80c\x11\x99/\x8C\x14a\x01\xDDW\x80c\x18\xD5\xAA\xFE\x14a\x01\xD8W\x80c\x1C\x0Bcg\x14a\x01\xD3W\x80c6l\xBA\xB7\x14a\x01\xCEW\x80c;j\xB2\xA9\x14a\x01\xC9W\x80c=D\xAE\x8B\x14a\x01\xC4W\x80cF\xE2\xCC\t\x14a\x01\xBFW\x80cH\\\xC9U\x14a\x01\xBAW\x80cK,\x07\x06\x14a\x01\xB5W\x80c[<\xD6\xE2\x14a\x01\xB0W\x80caT8\x01\x14a\x01\xABW\x80ceX\x95O\x14a\x01\xA6W\x80cp<\xFC\xBB\x14a\x01\xA1W\x80cqP\x18\xA6\x14a\x01\x9CW\x80cz9y\xDC\x14a\x01\x97W\x80c\x80NQ#\x14a\x01\x92W\x80c\x82\xF4J\xDE\x14a\x01\x8DW\x80c\x83\xD3\xC1\x15\x14a\x01\x88W\x80c\x8DZ#\x9B\x14a\x01\x83W\x80c\x8D\xA5\xCB[\x14a\x01~W\x80c\xAF\xF7Lm\x14a\x01yW\x80c\xC6`\xD3\xF3\x14a\x01tW\x80c\xCD\xAF\xB9x\x14a\x01oW\x80c\xD4\xF0\xEBM\x14a\x01jW\x80c\xD8x\x13B\x14a\x01eW\x80c\xEAJ\x11\x04\x14a\x01`Wc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\x0F\x0BV[a\x0E\xD2V[a\r\xA6V[a\rOV[a\x0C\xFDV[a\x0C\x92V[a\x0C]V[a\x0C(V[a\x0B\xD1V[a\x0B\x9BV[a\x0B,V[a\n\xF8V[a\n\xBFV[a\n:V[a\n\x05V[a\t\xC1V[a\tSV[a\x08\xE6V[a\x08\x1BV[a\x07\xC9V[a\x07.V[a\x06\xF9V[a\x06hV[a\x05\xF3V[a\x05\x1EV[a\x04\xE9V[a\x04\x90V[a\x03~V[a\x02\xBFV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x02JW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x02EW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x02@WV[a\x02\x0CV[a\x02\x08V[a\x02\x04V[\x90V[a\x02[\x81a\x02OV[\x03a\x02bWV[_\x80\xFD[\x90P5\x90a\x02s\x82a\x02RV[V[\x91`@\x83\x83\x03\x12a\x02\xB5W_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02\xB0Wa\x02\xA2\x83a\x02\xAD\x92\x86\x01a\x02\x10V[\x93\x90\x94` \x01a\x02fV[\x90V[a\x02\0V[a\x01\xFCV[_\x01\x90V[4a\x02\xEEWa\x02\xD8a\x02\xD26`\x04a\x02uV[\x91a\x10\x1EV[a\x02\xE0a\x01\xF2V[\x80a\x02\xEA\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[_\x91\x03\x12a\x02\xFDWV[a\x01\xFCV[a\x03\x0B\x90a\x02OV[\x90RV[\x15\x15\x90V[a\x03\x1D\x90a\x03\x0FV[\x90RV[\x90``\x80a\x03g\x93a\x039_\x82\x01Q_\x86\x01\x90a\x03\x02V[a\x03K` \x82\x01Q` \x86\x01\x90a\x03\x02V[a\x03]`@\x82\x01Q`@\x86\x01\x90a\x03\x02V[\x01Q\x91\x01\x90a\x03\x14V[V[\x91\x90a\x03|\x90_`\x80\x85\x01\x94\x01\x90a\x03!V[V[4a\x03\xAEWa\x03\x8E6`\x04a\x02\xF3V[a\x03\xAAa\x03\x99a\x11\x98V[a\x03\xA1a\x01\xF2V[\x91\x82\x91\x82a\x03iV[\x03\x90\xF3[a\x01\xF8V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03\xEDW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\xE8W` \x01\x92` \x83\x02\x84\x01\x11a\x03\xE3WV[a\x02\x0CV[a\x02\x08V[a\x02\x04V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04,W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04'W` \x01\x92` \x83\x02\x84\x01\x11a\x04\"WV[a\x02\x0CV[a\x02\x08V[a\x02\x04V[\x90\x91`@\x82\x84\x03\x12a\x04\x8BW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x86W\x83a\x04\\\x91\x84\x01a\x03\xB3V[\x92\x90\x93` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x81Wa\x04}\x92\x01a\x03\xF2V[\x90\x91V[a\x02\0V[a\x02\0V[a\x01\xFCV[4a\x04\xC2Wa\x04\xACa\x04\xA36`\x04a\x041V[\x92\x91\x90\x91a\x13\xA1V[a\x04\xB4a\x01\xF2V[\x80a\x04\xBE\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[a\x04\xD0\x90a\x03\x0FV[\x90RV[\x91\x90a\x04\xE7\x90_` \x85\x01\x94\x01\x90a\x04\xC7V[V[4a\x05\x19Wa\x04\xF96`\x04a\x02\xF3V[a\x05\x15a\x05\x04a\x14\xA5V[a\x05\x0Ca\x01\xF2V[\x91\x82\x91\x82a\x04\xD4V[\x03\x90\xF3[a\x01\xF8V[4a\x05MWa\x057a\x0516`\x04a\x02uV[\x91a\x15\xB2V[a\x05?a\x01\xF2V[\x80a\x05I\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[\x90` \x82\x82\x03\x12a\x05\x83W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05~Wa\x05z\x92\x01a\x02\x10V[\x90\x91V[a\x02\0V[a\x01\xFCV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x05\xC9a\x05\xD2` \x93a\x05\xD7\x93a\x05\xC0\x81a\x05\x88V[\x93\x84\x80\x93a\x05\x8CV[\x95\x86\x91\x01a\x05\x95V[a\x05\xA0V[\x01\x90V[a\x05\xF0\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x05\xAAV[\x90V[4a\x06$Wa\x06 a\x06\x0Fa\x06\t6`\x04a\x05RV[\x90a\x16BV[a\x06\x17a\x01\xF2V[\x91\x82\x91\x82a\x05\xDBV[\x03\x90\xF3[a\x01\xF8V[\x1C\x90V[`\xFF\x16\x90V[a\x06C\x90`\x08a\x06H\x93\x02a\x06)V[a\x06-V[\x90V[\x90a\x06V\x91Ta\x063V[\x90V[a\x06e`\x04_\x90a\x06KV[\x90V[4a\x06\x98Wa\x06x6`\x04a\x02\xF3V[a\x06\x94a\x06\x83a\x06YV[a\x06\x8Ba\x01\xF2V[\x91\x82\x91\x82a\x04\xD4V[\x03\x90\xF3[a\x01\xF8V[\x90V[\x90V[a\x06\xB7a\x06\xB2a\x06\xBC\x92a\x06\x9DV[a\x06\xA0V[a\x02OV[\x90V[a\x06\xC9`\na\x06\xA3V[\x90V[a\x06\xD4a\x06\xBFV[\x90V[a\x06\xE0\x90a\x02OV[\x90RV[\x91\x90a\x06\xF7\x90_` \x85\x01\x94\x01\x90a\x06\xD7V[V[4a\x07)Wa\x07\t6`\x04a\x02\xF3V[a\x07%a\x07\x14a\x06\xCCV[a\x07\x1Ca\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[4a\x07]Wa\x07Ga\x07A6`\x04a\x05RV[\x90a\x17\xDCV[a\x07Oa\x01\xF2V[\x80a\x07Y\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x07v\x90a\x07bV[\x90V[a\x07\x82\x81a\x07mV[\x03a\x07\x89WV[_\x80\xFD[\x90P5\x90a\x07\x9A\x82a\x07yV[V[\x91\x90`@\x83\x82\x03\x12a\x07\xC4W\x80a\x07\xB8a\x07\xC1\x92_\x86\x01a\x07\x8DV[\x93` \x01a\x07\x8DV[\x90V[a\x01\xFCV[4a\x07\xF8Wa\x07\xE2a\x07\xDC6`\x04a\x07\x9CV[\x90a\x19\x8DV[a\x07\xEAa\x01\xF2V[\x80a\x07\xF4\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[\x90` \x82\x82\x03\x12a\x08\x16Wa\x08\x13\x91_\x01a\x02fV[\x90V[a\x01\xFCV[4a\x08KWa\x08Ga\x086a\x0816`\x04a\x07\xFDV[a\x19\x99V[a\x08>a\x01\xF2V[\x91\x82\x91\x82a\x03iV[\x03\x90\xF3[a\x01\xF8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08k\x90`\x08a\x08p\x93\x02a\x06)V[a\x08PV[\x90V[\x90a\x08~\x91Ta\x08[V[\x90V[a\x08\x8D`\x01_\x90a\x08sV[\x90V[a\x08\xA4a\x08\x9Fa\x08\xA9\x92a\x07bV[a\x06\xA0V[a\x07bV[\x90V[a\x08\xB5\x90a\x08\x90V[\x90V[a\x08\xC1\x90a\x08\xACV[\x90V[a\x08\xCD\x90a\x08\xB8V[\x90RV[\x91\x90a\x08\xE4\x90_` \x85\x01\x94\x01\x90a\x08\xC4V[V[4a\t\x16Wa\x08\xF66`\x04a\x02\xF3V[a\t\x12a\t\x01a\x08\x81V[a\t\ta\x01\xF2V[\x91\x82\x91\x82a\x08\xD1V[\x03\x90\xF3[a\x01\xF8V[\x90V[a\t.\x90`\x08a\t3\x93\x02a\x06)V[a\t\x1BV[\x90V[\x90a\tA\x91Ta\t\x1EV[\x90V[a\tP`\x03_\x90a\t6V[\x90V[4a\t\x83Wa\tc6`\x04a\x02\xF3V[a\t\x7Fa\tna\tDV[a\tva\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[\x90V[a\t\x9Fa\t\x9Aa\t\xA4\x92a\t\x88V[a\x06\xA0V[a\x02OV[\x90V[a\t\xB3b'\x8D\0a\t\x8BV[\x90V[a\t\xBEa\t\xA7V[\x90V[4a\t\xF1Wa\t\xD16`\x04a\x02\xF3V[a\t\xEDa\t\xDCa\t\xB6V[a\t\xE4a\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[a\n\x02`\x02_\x90a\t6V[\x90V[4a\n5Wa\n\x156`\x04a\x02\xF3V[a\n1a\n a\t\xF6V[a\n(a\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[4a\nhWa\nJ6`\x04a\x02\xF3V[a\nRa\x19\xDDV[a\nZa\x01\xF2V[\x80a\nd\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[\x91``\x83\x83\x03\x12a\n\xBAWa\n\x84\x82_\x85\x01a\x07\x8DV[\x92a\n\x92\x83` \x83\x01a\x07\x8DV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xB5Wa\n\xB1\x92\x01a\x02\x10V[\x90\x91V[a\x02\0V[a\x01\xFCV[4a\n\xF3Wa\n\xEFa\n\xDEa\n\xD56`\x04a\nmV[\x92\x91\x90\x91a\x1A\x95V[a\n\xE6a\x01\xF2V[\x91\x82\x91\x82a\x04\xD4V[\x03\x90\xF3[a\x01\xF8V[4a\x0B'Wa\x0B\x11a\x0B\x0B6`\x04a\x05RV[\x90a\x1B\xF8V[a\x0B\x19a\x01\xF2V[\x80a\x0B#\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[4a\x0B\\Wa\x0B<6`\x04a\x02\xF3V[a\x0BXa\x0BGa\x1C\x15V[a\x0BOa\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[\x90\x91``\x82\x84\x03\x12a\x0B\x96Wa\x0B\x93a\x0B|\x84_\x85\x01a\x02fV[\x93a\x0B\x8A\x81` \x86\x01a\x02fV[\x93`@\x01a\x02fV[\x90V[a\x01\xFCV[4a\x0B\xCCWa\x0B\xC8a\x0B\xB7a\x0B\xB16`\x04a\x0BaV[\x91a\x1C\xE2V[a\x0B\xBFa\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[4a\x0C\x01Wa\x0B\xE16`\x04a\x02\xF3V[a\x0B\xFDa\x0B\xECa\x1DXV[a\x0B\xF4a\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[a\x0C\x0F\x90a\x07mV[\x90RV[\x91\x90a\x0C&\x90_` \x85\x01\x94\x01\x90a\x0C\x06V[V[4a\x0CXWa\x0C86`\x04a\x02\xF3V[a\x0CTa\x0CCa\x1D\xEAV[a\x0CKa\x01\xF2V[\x91\x82\x91\x82a\x0C\x13V[\x03\x90\xF3[a\x01\xF8V[4a\x0C\x8DWa\x0Cm6`\x04a\x02\xF3V[a\x0C\x89a\x0Cxa\x1E\x1EV[a\x0C\x80a\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[4a\x0C\xC2Wa\x0C\xA26`\x04a\x02\xF3V[a\x0C\xBEa\x0C\xADa\x1EjV[a\x0C\xB5a\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[\x90` \x82\x82\x03\x12a\x0C\xF8W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0C\xF3Wa\x0C\xEF\x92\x01a\x03\xB3V[\x90\x91V[a\x02\0V[a\x01\xFCV[4a\r,Wa\r\x16a\r\x106`\x04a\x0C\xC7V[\x90a\x1F\xB6V[a\r\x1Ea\x01\xF2V[\x80a\r(\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[\x90` \x82\x82\x03\x12a\rJWa\rG\x91_\x01a\x07\x8DV[\x90V[a\x01\xFCV[4a\r}Wa\rga\rb6`\x04a\r1V[a fV[a\roa\x01\xF2V[\x80a\ry\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\r\xD6Wa\r\xB66`\x04a\x02\xF3V[a\r\xD2a\r\xC1a\r\x82V[a\r\xC9a\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[a\r\xEFa\r\xEAa\r\xF4\x92a\x02OV[a\x06\xA0V[a\x02OV[\x90V[\x90a\x0E\x01\x90a\r\xDBV[_R` R`@_ \x90V[_\x1C\x90V[a\x0E\x1Ea\x0E#\x91a\x0E\rV[a\t\x1BV[\x90V[a\x0E0\x90Ta\x0E\x12V[\x90V[a\x0E?a\x0ED\x91a\x0E\rV[a\x06-V[\x90V[a\x0EQ\x90Ta\x0E3V[\x90V[a\x0E_\x90`\x05a\r\xF7V[\x90a\x0Ek_\x83\x01a\x0E&V[\x91a\x0Ex`\x01\x82\x01a\x0E&V[\x91a\x0E\x91`\x03a\x0E\x8A`\x02\x85\x01a\x0E&V[\x93\x01a\x0EGV[\x90V[a\x0E\xC9a\x0E\xD0\x94a\x0E\xBF``\x94\x98\x97\x95a\x0E\xB5`\x80\x86\x01\x9A_\x87\x01\x90a\x06\xD7V[` \x85\x01\x90a\x06\xD7V[`@\x83\x01\x90a\x06\xD7V[\x01\x90a\x04\xC7V[V[4a\x0F\x06Wa\x0F\x02a\x0E\xEDa\x0E\xE86`\x04a\x07\xFDV[a\x0ETV[\x90a\x0E\xF9\x94\x92\x94a\x01\xF2V[\x94\x85\x94\x85a\x0E\x94V[\x03\x90\xF3[a\x01\xF8V[4a\x0F9Wa\x0F#a\x0F\x1E6`\x04a\r1V[a \xD6V[a\x0F+a\x01\xF2V[\x80a\x0F5\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[_\x80\xFD[\x91\x90a\x0F_a\x0FY32\x90\x86\x85\x91\x92\x90\x91\x92a\x1A\x95V[\x15a\x03\x0FV[a\x0FnWa\x0Fl\x92a\x0F\xCBV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x0F\x86`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[a\x0F\x93\x90a\x08\xACV[\x90V[`@\x90a\x0F\xC2a\x0F\xB7a\x0F\xC9\x95\x97\x96\x94``\x84\x01\x90\x84\x82\x03_\x86\x01Ra\x05\xAAV[\x96` \x83\x01\x90a\x06\xD7V[\x01\x90a\x06\xD7V[V[\x90a\x0F\xD7\x903\x92a\x16BV[\x91B\x92a\x10\x19a\x10\x07\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x0F\x8AV[\x94a\x10\x10a\x01\xF2V[\x93\x84\x93\x84a\x0F\x96V[\x03\x90\xA2V[\x90a\x10)\x92\x91a\x0FBV[V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x10I\x90a\x05\xA0V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10cW`@RV[a\x10+V[\x90a\x10{a\x10ta\x01\xF2V[\x92\x83a\x10?V[V[a\x10\x87`\x80a\x10hV[\x90V[_\x90V[_\x90V[a\x10\x9Aa\x10}V[\x90` \x80\x80\x80\x85a\x10\xA9a\x10\x8AV[\x81R\x01a\x10\xB4a\x10\x8AV[\x81R\x01a\x10\xBFa\x10\x8AV[\x81R\x01a\x10\xCAa\x10\x8EV[\x81RPPV[a\x10\xD8a\x10\x92V[\x90V[a\x10\xE5`\x80a\x10hV[\x90V[\x90V[a\x10\xFFa\x10\xFAa\x11\x04\x92a\x10\xE8V[a\x06\xA0V[a\x02OV[\x90V[\x90a\x11\x11\x90a\x02OV[\x90RV[\x90a\x11\x1F\x90a\x03\x0FV[\x90RV[\x90a\x11\x8Aa\x11\x81`\x03a\x114a\x10}V[\x94a\x11Ka\x11C_\x83\x01a\x0E&V[_\x88\x01a\x11\x07V[a\x11ca\x11Z`\x01\x83\x01a\x0E&V[` \x88\x01a\x11\x07V[a\x11{a\x11r`\x02\x83\x01a\x0E&V[`@\x88\x01a\x11\x07V[\x01a\x0EGV[``\x84\x01a\x11\x15V[V[a\x11\x95\x90a\x11#V[\x90V[a\x11\xA0a\x10\xD0V[Pa\x11\xB4a\x11\xAE`\x04a\x0EGV[\x15a\x03\x0FV[a\x11\xD8Wa\x11\xD5a\x11\xD0`\x05a\x11\xCA`\x03a\x0E&V[\x90a\r\xF7V[a\x11\x8CV[\x90V[_a\x12-_a\x12$a\x12\x1B_a\x12\x16a\x12\r_\x95a\x12\x08a\x12\0a\x11\xFAa\x10\xDBV[\x9Aa\x10\xEBV[_\x8B\x01a\x11\x07V[a\x10\xEBV[` \x88\x01a\x11\x07V[a\x10\xEBV[`@\x85\x01a\x11\x07V[``\x83\x01a\x11\x15V[\x90V[P\x90V[P\x90V[` \x91\x81R\x01\x90V[` \x7Fve the same length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x7FData and priority arrays must ha_\x82\x01R\x01RV[a\x12\x9B`2`@\x92a\x128V[a\x12\xA4\x81a\x12AV[\x01\x90V[a\x12\xBD\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x12\x8EV[\x90V[\x15a\x12\xC7WV[a\x12\xCFa\x01\xF2V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x12\xE5`\x04\x82\x01a\x12\xA8V[\x03\x90\xFD[`\x01a\x12\xF5\x91\x01a\x02OV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x13ZW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x13UW` \x01\x91`\x01\x82\x026\x03\x83\x13a\x13PWV[a\x13\x14V[a\x13\x10V[a\x13\x0CV[\x90\x82\x10\x15a\x13zW` a\x13v\x92\x02\x81\x01\x90a\x13\x18V[\x90\x91V[a\x12\xF8V[\x91\x90\x81\x10\x15a\x13\x8FW` \x02\x01\x90V[a\x12\xF8V[5a\x13\x9E\x81a\x02RV[\x90V[\x90\x92a\x13\xAE\x82\x85\x90a\x120V[\x93a\x13\xD5\x85a\x13\xCFa\x13\xC9a\x13\xC4\x88\x87\x90a\x124V[a\x02OV[\x91a\x02OV[\x14a\x12\xC0V[a\x13\xDE_a\x10\xEBV[[\x80a\x13\xF2a\x13\xEC\x88a\x02OV[\x91a\x02OV[\x10\x15a\x14\x99Wa\x14 \x90a\x14\x1632\x90a\x14\x0E\x88\x87\x86\x91a\x13_V[\x92\x90\x91a\x1A\x95V[a\x14%W[a\x12\xE9V[a\x13\xDFV[3a\x14;a\x145\x87\x86\x85\x91a\x13_V[\x90a\x16BV[\x90a\x14Pa\x14K\x89\x88\x86\x91a\x13\x7FV[a\x13\x94V[B\x92a\x14\x91a\x14\x7F\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x0F\x8AV[\x94a\x14\x88a\x01\xF2V[\x93\x84\x93\x84a\x0F\x96V[\x03\x90\xA2a\x14\x1BV[PPPPPPV[_\x90V[a\x14\xADa\x14\xA1V[Pa\x14\xB8`\x04a\x0EGV[\x90V[\x91\x90a\x14\xD8a\x14\xD232\x90\x86\x85\x91\x92\x90\x91\x92a\x1A\x95V[\x15a\x03\x0FV[a\x14\xE7Wa\x14\xE5\x92a\x15fV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x14\xFF`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[\x90\x82_\x93\x92\x827\x01RV[\x91\x90a\x15(\x81a\x15!\x81a\x15-\x95a\x05\x8CV[\x80\x95a\x15\x03V[a\x05\xA0V[\x01\x90V[a\x15]a\x15R`@\x93a\x15d\x96\x98\x97\x95``\x85\x01\x91\x85\x83\x03_\x87\x01Ra\x15\x0EV[\x96` \x83\x01\x90a\x06\xD7V[\x01\x90a\x06\xD7V[V[\x90\x913\x91\x92\x90\x92a\x15\xADBa\x15\x9B\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x95a\x0F\x8AV[\x95a\x15\xA4a\x01\xF2V[\x94\x85\x94\x85a\x151V[\x03\x90\xA2V[\x90a\x15\xBD\x92\x91a\x14\xBBV[V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x15\xE7a\x15\xE2a\x15\xEC\x92a\x10\xE8V[a\x15\xCDV[a\x15\xC4V[\x90V[\x90V[a\x15\xFEa\x16\x03\x91a\x15\xC4V[a\x15\xEFV[\x90RV[\x90P\x90V[\x90\x91\x82a\x16\x1C\x81a\x16#\x93a\x16\x07V[\x80\x93a\x15\x03V[\x01\x90V[\x80a\x168`\x01\x92a\x16?\x96\x94a\x15\xF2V[\x01\x91a\x16\x0CV[\x90V[a\x16\x80\x90a\x16Na\x15\xBFV[Pa\x16qa\x16[_a\x15\xD3V[\x91\x93a\x16ea\x01\xF2V[\x94\x85\x93` \x85\x01a\x16'V[` \x82\x01\x81\x03\x82R\x03\x82a\x10?V[\x90V[\x90a\x16\x9Fa\x16\x9932\x90\x85\x85\x91\x92\x90\x91\x92a\x1A\x95V[\x15a\x03\x0FV[a\x16\xAEWa\x16\xAC\x91a\x17GV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x16\xC6`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x16\xEDa\x16\xF3\x91\x93\x92\x93a\x02OV[\x92a\x02OV[\x82\x03\x91\x82\x11a\x16\xFEWV[a\x16\xCAV[\x90V[a\x17\x1Aa\x17\x15a\x17\x1F\x92a\x17\x03V[a\x06\xA0V[a\x02OV[\x90V[a\x171a\x177\x91\x93\x92\x93a\x02OV[\x92a\x02OV[\x82\x01\x80\x92\x11a\x17BWV[a\x16\xCAV[a\x17ca\x17t\x91a\x17\\a\x17y\x94Z\x92a\x17\x95V[Z\x90a\x16\xDEV[a\x17na\x13\x88a\x17\x06V[\x90a\x17\"V[a!=V[V[\x90\x91a\x17\x92\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x15\x0EV[\x90V[3\x90\x91a\x17\xC2\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x0F\x8AV[\x92a\x17\xD7a\x17\xCEa\x01\xF2V[\x92\x83\x92\x83a\x17{V[\x03\x90\xA2V[\x90a\x17\xE6\x91a\x16\x83V[V[\x90a\x17\xFA\x91a\x17\xF5a!\xEDV[a\x19\0V[V[`\xA0\x1C\x90V[a\x18\x0Ea\x18\x13\x91a\x17\xFCV[a\x06-V[\x90V[a\x18 \x90Ta\x18\x02V[\x90V[a\x187a\x182a\x18<\x92a\x10\xE8V[a\x06\xA0V[a\x07bV[\x90V[a\x18H\x90a\x18#V[\x90V[`\xA0\x1B\x90V[\x90a\x18``\xFF`\xA0\x1B\x91a\x18KV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18s\x90a\x03\x0FV[\x90V[\x90V[\x90a\x18\x8Ea\x18\x89a\x18\x95\x92a\x18jV[a\x18vV[\x82Ta\x18QV[\x90UV[a\x18\xA2\x90a\x08\x90V[\x90V[a\x18\xAE\x90a\x18\x99V[\x90V[_\x1B\x90V[\x90a\x18\xC7`\x01\x80`\xA0\x1B\x03\x91a\x18\xB1V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18\xDA\x90a\x18\x99V[\x90V[\x90V[\x90a\x18\xF5a\x18\xF0a\x18\xFC\x92a\x18\xD1V[a\x18\xDDV[\x82Ta\x18\xB6V[\x90UV[a\x19\n`\x01a\x18\x16V[a\x19rW\x81a\x19)a\x19#a\x19\x1E_a\x18?V[a\x07mV[\x91a\x07mV[\x14a\x19VWa\x19Oa\x19Ha\x19T\x93a\x19C`\x01\x80a\x18yV[a\x18\xA5V[`\x01a\x18\xE0V[a \xD6V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x19n`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x19\x89`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[\x90a\x19\x97\x91a\x17\xE8V[V[a\x19\xB0a\x19\xB5\x91a\x19\xA8a\x10\xD0V[P`\x05a\r\xF7V[a\x11\x8CV[\x90V[a\x19\xC0a!\xEDV[a\x19\xC8a\x19\xCAV[V[a\x19\xDBa\x19\xD6_a\x18?V[a\"^V[V[a\x19\xE5a\x19\xB8V[V[a\x19\xF3a\x19\xF8\x91a\x0E\rV[a\x08PV[\x90V[a\x1A\x05\x90Ta\x19\xE7V[\x90V[`\xE0\x1B\x90V[a\x1A\x17\x81a\x03\x0FV[\x03a\x1A\x1EWV[_\x80\xFD[\x90PQ\x90a\x1A/\x82a\x1A\x0EV[V[\x90` \x82\x82\x03\x12a\x1AJWa\x1AG\x91_\x01a\x1A\"V[\x90V[a\x01\xFCV[a\x1Aua\x1A\x82\x95\x93\x94\x92\x94a\x1Ak``\x84\x01\x96_\x85\x01\x90a\x0C\x06V[` \x83\x01\x90a\x0C\x06V[`@\x81\x85\x03\x91\x01Ra\x15\x0EV[\x90V[a\x1A\x8Da\x01\xF2V[=_\x82>=\x90\xFD[\x92a\x1A\xD8` \x93\x94a\x1A\xA5a\x14\xA1V[Pa\x1A\xE3a\x1A\xBBa\x1A\xB6`\x01a\x19\xFBV[a\x08\xB8V[\x93cz9y\xDC\x92\x95\x97a\x1A\xCCa\x01\xF2V[\x98\x89\x97\x88\x96\x87\x96a\x1A\x08V[\x86R`\x04\x86\x01a\x1AOV[\x03\x91Z\xFA\x90\x81\x15a\x1B'W_\x91a\x1A\xF9W[P\x90V[a\x1B\x1A\x91P` =\x81\x11a\x1B W[a\x1B\x12\x81\x83a\x10?V[\x81\x01\x90a\x1A1V[_a\x1A\xF5V[P=a\x1B\x08V[a\x1A\x85V[\x90a\x1BHa\x1BB32\x90\x85\x85\x91\x92\x90\x91\x92a\x1A\x95V[\x15a\x03\x0FV[a\x1BWWa\x1BU\x91a\x1BsV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x1Bo`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[a\x1B\x8Fa\x1B\xA0\x91a\x1B\x88a\x1B\xA5\x94Z\x92a\x1B\xA7V[Z\x90a\x16\xDEV[a\x1B\x9Aa\x13\x88a\x17\x06V[\x90a\x17\"V[a!=V[V[\x90a\x1B\xB3\x903\x92a\x16BV[\x90a\x1B\xF3a\x1B\xE1\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x0F\x8AV[\x92a\x1B\xEAa\x01\xF2V[\x91\x82\x91\x82a\x05\xDBV[\x03\x90\xA2V[\x90a\x1C\x02\x91a\x1B,V[V[_\x90V[a\x1C\x12\x90Qa\x02OV[\x90V[a\x1C\x1Da\x1C\x04V[Pa\x1C1a\x1C+`\x04a\x0EGV[\x15a\x03\x0FV[a\x1C\xA1Wa\x1Cma\x1C__a\x1CYa\x1CT`\x05a\x1CN`\x03a\x0E&V[\x90a\r\xF7V[a\x11\x8CV[\x01a\x1C\x08V[a\x1Cga\t\xA7V[\x90a\x17\"V[Ba\x1C\x80a\x1Cz\x83a\x02OV[\x91a\x02OV[\x10\x15a\x1C\x94Wa\x1C\x91\x90B\x90a\x16\xDEV[\x90V[Pa\x1C\x9E_a\x10\xEBV[\x90V[a\x1C\xAA_a\x10\xEBV[\x90V[a\x1C\xBCa\x1C\xC2\x91\x93\x92\x93a\x02OV[\x92a\x02OV[\x91a\x1C\xCE\x83\x82\x02a\x02OV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1C\xDDWV[a\x16\xCAV[\x91a\x1C\xEBa\x1C\x04V[P\x80a\x1C\xFFa\x1C\xF9\x84a\x02OV[\x91a\x02OV[\x11\x15a\x1DSWa\x1D \x91a\x1D\x12\x91a\x16\xDEV[a\x1D\x1Aa\x06\xBFV[\x90a\x1C\xADV[\x80a\x1D3a\x1D-\x84a\x02OV[\x91a\x02OV[\x10\x15a\x1DEWa\x1DB\x91a\x16\xDEV[\x90V[PPa\x1DP_a\x10\xEBV[\x90V[PP\x90V[a\x1D`a\x1C\x04V[Pa\x1Dta\x1Dn`\x04a\x0EGV[\x15a\x03\x0FV[a\x1D\xAEWa\x1D\xABa\x1D\x9B`\x02a\x1D\x95`\x05a\x1D\x8F`\x03a\x0E&V[\x90a\r\xF7V[\x01a\x0E&V[a\x1D\xA5`\x02a\x0E&V[\x90a\x1C\xADV[\x90V[a\x1D\xB7_a\x10\xEBV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1D\xD5a\x1D\xDA\x91a\x0E\rV[a\x1D\xBEV[\x90V[a\x1D\xE7\x90Ta\x1D\xC9V[\x90V[a\x1D\xF2a\x1D\xBAV[Pa\x1D\xFC_a\x1D\xDDV[\x90V[\x90V[a\x1E\x16a\x1E\x11a\x1E\x1B\x92a\x1D\xFFV[a\x06\xA0V[a\x02OV[\x90V[a\x1E&a\x1C\x04V[Pa\x1E:a\x1E4`\x04a\x0EGV[\x15a\x03\x0FV[a\x1E^Wa\x1E[a\x1EK`\x03a\x0E&V[a\x1EU`\x01a\x1E\x02V[\x90a\x17\"V[\x90V[a\x1Eg_a\x10\xEBV[\x90V[a\x1Era\x1C\x04V[Pa\x1E\x86a\x1E\x80`\x04a\x0EGV[\x15a\x03\x0FV[a\x1E\xADWa\x1E\xAA`\x02a\x1E\xA4`\x05a\x1E\x9E`\x03a\x0E&V[\x90a\r\xF7V[\x01a\x0E&V[\x90V[a\x1E\xB6_a\x10\xEBV[\x90V[a\x1E\xD5a\x1E\xE6\x91a\x1E\xCEa\x1E\xEB\x94Z\x92a\x1E\xEDV[Z\x90a\x16\xDEV[a\x1E\xE0a\x13\x88a\x17\x06V[\x90a\x17\"V[a!=V[V[a\x1E\xF8\x81\x83\x90a\x120V[\x91a\x1F\x01a\x1C\x04V[Pa\x1F\x0B_a\x10\xEBV[[\x80a\x1F\x1Fa\x1F\x19\x86a\x02OV[\x91a\x02OV[\x10\x15a\x1F\xB0Wa\x1FM\x90a\x1FC32\x90a\x1F;\x87\x87\x86\x91a\x13_V[\x92\x90\x91a\x1A\x95V[a\x1FRW[a\x12\xE9V[a\x1F\x0CV[3a\x1Fha\x1Fb\x86\x86\x85\x91a\x13_V[\x90a\x16BV[\x90a\x1F\xA8a\x1F\x96\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x0F\x8AV[\x92a\x1F\x9Fa\x01\xF2V[\x91\x82\x91\x82a\x05\xDBV[\x03\x90\xA2a\x1FHV[PPPPV[\x90a\x1F\xC0\x91a\x1E\xB9V[V[a\x1F\xD3\x90a\x1F\xCEa!\xEDV[a\x1F\xD5V[V[\x80a\x1F\xF0a\x1F\xEAa\x1F\xE5_a\x18?V[a\x07mV[\x91a\x07mV[\x14a JWa \x08a \x01\x82a\x18\xA5V[`\x01a\x18\xE0V[a 2\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x0F\x8AV[\x90a ;a\x01\xF2V[\x80a E\x81a\x02\xBAV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a b`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[a o\x90a\x1F\xC2V[V[a \x82\x90a }a!\xEDV[a \x84V[V[\x80a \x9Fa \x99a \x94_a\x18?V[a\x07mV[\x91a\x07mV[\x14a \xAFWa \xAD\x90a\"^V[V[a \xD2a \xBB_a\x18?V[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x0C\x13V[\x03\x90\xFD[a \xDF\x90a qV[V[\x90a \xED_\x19\x91a\x18\xB1V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a!\x0Fa!\na!\x16\x92a\r\xDBV[a \xF7V[\x82Ta \xE1V[\x90UV[\x91` a!;\x92\x94\x93a!4`@\x82\x01\x96_\x83\x01\x90a\x06\xD7V[\x01\x90a\x06\xD7V[V[a!Pa!J`\x04a\x0EGV[\x15a\x03\x0FV[a!\xE0W[a!]a$\x8AV[a!\x91\x81a!\x8B`\x02a!{`\x05a!u`\x03a\x0E&V[\x90a\r\xF7V[\x01\x91a!\x86\x83a\x0E&V[a\x17\"V[\x90a \xFAV[a!\x9B`\x03a\x0E&V[:a!\xC6\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\r\xDBV[\x92a!\xDBa!\xD2a\x01\xF2V[\x92\x83\x92\x83a!\x1AV[\x03\x90\xA2V[a!\xE8a#\x87V[a!UV[a!\xF5a\x1D\xEAV[a\"\x0Ea\"\x08a\"\x03a&hV[a\x07mV[\x91a\x07mV[\x03a\"\x15WV[a\"7a\" a&hV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\x0C\x13V[\x03\x90\xFD[\x90V[\x90a\"Sa\"Na\"Z\x92a\x0F\x8AV[a\";V[\x82Ta\x18\xB6V[\x90UV[a\"g_a\x1D\xDDV[a\"q\x82_a\">V[\x90a\"\xA5a\"\x9F\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x0F\x8AV[\x91a\x0F\x8AV[\x91a\"\xAEa\x01\xF2V[\x80a\"\xB8\x81a\x02\xBAV[\x03\x90\xA3V[\x90a\"\xC9`\xFF\x91a\x18\xB1V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\"\xE8a\"\xE3a\"\xEF\x92a\x18jV[a\x18vV[\x82Ta\"\xBDV[\x90UV[\x90a\"\xFD\x90a\x10\xEBV[_R` R`@_ \x90V[a#\x13\x90Qa\x03\x0FV[\x90V[\x90a#s```\x03a#y\x94a#9_\x82\x01a#3_\x88\x01a\x1C\x08V[\x90a \xFAV[a#R`\x01\x82\x01a#L` \x88\x01a\x1C\x08V[\x90a \xFAV[a#k`\x02\x82\x01a#e`@\x88\x01a\x1C\x08V[\x90a \xFAV[\x01\x92\x01a#\tV[\x90a\"\xD3V[V[\x90a#\x85\x91a#\x16V[V[a#\x9Aa#\x94`\x04a\x0EGV[\x15a\x03\x0FV[a#\xA1W[V[a#\xAD`\x01`\x04a\"\xD3V[a#\xC0a#\xB9_a\x10\xEBV[`\x03a \xFAV[a$!Ba$\x10_a$\x07a#\xFE_a#\xF9a#\xF0_\x95a#\xEBa#\xE2a\x10\xDBV[\x99_\x8B\x01a\x11\x07V[a\x10\xEBV[` \x88\x01a\x11\x07V[a\x10\xEBV[`@\x85\x01a\x11\x07V[``\x83\x01a\x11\x15V[a$\x1C`\x05_\x90a\"\xF3V[a#{V[_B\x90a$ca$Q\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x10\xEBV[\x92a$Za\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xA2a#\x9FV[\x90V[a$w\x90a\x02OV[_\x19\x81\x14a$\x85W`\x01\x01\x90V[a\x16\xCAV[a$\xA7a$\xA2`\x05a$\x9C`\x03a\x0E&V[\x90a\r\xF7V[a$kV[Ba$\xD5a$\xCFa$\xCAa$\xBC_\x86\x01a\x0E&V[a$\xC4a\t\xA7V[\x90a\x17\"V[a\x02OV[\x91a\x02OV[\x10\x15a$\xDFW[PV[a%\x07a$\xFEa$\xF0_\x84\x01a\x0E&V[a$\xF8a\t\xA7V[\x90a\x17\"V[`\x01\x83\x01a \xFAV[a%\x15`\x01`\x03\x83\x01a\"\xD3V[a%\x1F`\x03a\x0E&V[a%La%.`\x02\x84\x01a\x0E&V[\x92a%F_a%?`\x01\x84\x01a\x0E&V[\x92\x01a\x0E&V[\x90a\x16\xDEV[a%v\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\r\xDBV[\x92a%\x8Ba%\x82a\x01\xF2V[\x92\x83\x92\x83a!\x1AV[\x03\x90\xA2a%\xAAa%\xA3a%\x9E`\x03a\x0E&V[a$nV[`\x03a \xFAV[a&\x14Ba%\xFA_a%\xF1a%\xE8_a%\xE3a%\xDA_\x95a%\xD5a%\xCCa\x10\xDBV[\x99_\x8B\x01a\x11\x07V[a\x10\xEBV[` \x88\x01a\x11\x07V[a\x10\xEBV[`@\x85\x01a\x11\x07V[``\x83\x01a\x11\x15V[a&\x0F`\x05a&\t`\x03a\x0E&V[\x90a\r\xF7V[a#{V[a&\x1E`\x03a\x0E&V[B\x90a&_a&M\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\r\xDBV[\x92a&Va\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xA2_a$\xDCV[a&pa\x1D\xBAV[P3\x90V`\x80`@R4`\x1CW`\x0E` V[a\x01\x94a\0+\x829a\x01\x94\x90\xF3[`&V[`@Q\x90V[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x01{V[a\0\x1D_5a\0,V[cz9y\xDC\x03a\0\x0EWa\x01BV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\0X\x90a\0DV[\x90V[a\0d\x81a\0OV[\x03a\0kWV[_\x80\xFD[\x90P5\x90a\0|\x82a\0[V[V[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\0\xC4W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xBFW` \x01\x92`\x01\x83\x02\x84\x01\x11a\0\xBAWV[a\0\x86V[a\0\x82V[a\0~V[\x91``\x83\x83\x03\x12a\x01\x16Wa\0\xE0\x82_\x85\x01a\0oV[\x92a\0\xEE\x83` \x83\x01a\0oV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x11Wa\x01\r\x92\x01a\0\x8AV[\x90\x91V[a\0@V[a\0<V[\x15\x15\x90V[a\x01)\x90a\x01\x1BV[\x90RV[\x91\x90a\x01@\x90_` \x85\x01\x94\x01\x90a\x01 V[V[4a\x01vWa\x01ra\x01aa\x01X6`\x04a\0\xC9V[\x92\x91\x90\x91a\x01\x83V[a\x01ia\x002V[\x91\x82\x91\x82a\x01-V[\x03\x90\xF3[a\08V[_\x80\xFD[_\x90V[PPPPa\x01\x8Fa\x01\x7FV[P_\x90V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b610f3e565b61001d5f356101ec565b8063050ec138146101e7578063086146d2146101e257806311992f8c146101dd57806318d5aafe146101d85780631c0b6367146101d3578063366cbab7146101ce5780633b6ab2a9146101c95780633d44ae8b146101c457806346e2cc09146101bf578063485cc955146101ba5780634b2c0706146101b55780635b3cd6e2146101b057806361543801146101ab5780636558954f146101a6578063703cfcbb146101a1578063715018a61461019c5780637a3979dc14610197578063804e51231461019257806382f44ade1461018d57806383d3c115146101885780638d5a239b146101835780638da5cb5b1461017e578063aff74c6d14610179578063c660d3f314610174578063cdafb9781461016f578063d4f0eb4d1461016a578063d878134214610165578063ea4a1104146101605763f2fde38b0361000e57610f0b565b610ed2565b610da6565b610d4f565b610cfd565b610c92565b610c5d565b610c28565b610bd1565b610b9b565b610b2c565b610af8565b610abf565b610a3a565b610a05565b6109c1565b610953565b6108e6565b61081b565b6107c9565b61072e565b6106f9565b610668565b6105f3565b61051e565b6104e9565b610490565b61037e565b6102bf565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f8301121561024a5781359167ffffffffffffffff831161024557602001926001830284011161024057565b61020c565b610208565b610204565b90565b61025b8161024f565b0361026257565b5f80fd5b9050359061027382610252565b565b916040838303126102b5575f83013567ffffffffffffffff81116102b0576102a2836102ad928601610210565b939094602001610266565b90565b610200565b6101fc565b5f0190565b346102ee576102d86102d2366004610275565b9161101e565b6102e06101f2565b806102ea816102ba565b0390f35b6101f8565b5f9103126102fd57565b6101fc565b61030b9061024f565b9052565b151590565b61031d9061030f565b9052565b90606080610367936103395f8201515f860190610302565b61034b60208201516020860190610302565b61035d60408201516040860190610302565b0151910190610314565b565b919061037c905f60808501940190610321565b565b346103ae5761038e3660046102f3565b6103aa610399611198565b6103a16101f2565b91829182610369565b0390f35b6101f8565b909182601f830112156103ed5781359167ffffffffffffffff83116103e85760200192602083028401116103e357565b61020c565b610208565b610204565b909182601f8301121561042c5781359167ffffffffffffffff831161042757602001926020830284011161042257565b61020c565b610208565b610204565b909160408284031261048b575f82013567ffffffffffffffff8111610486578361045c9184016103b3565b929093602082013567ffffffffffffffff81116104815761047d92016103f2565b9091565b610200565b610200565b6101fc565b346104c2576104ac6104a3366004610431565b929190916113a1565b6104b46101f2565b806104be816102ba565b0390f35b6101f8565b6104d09061030f565b9052565b91906104e7905f602085019401906104c7565b565b34610519576104f93660046102f3565b6105156105046114a5565b61050c6101f2565b918291826104d4565b0390f35b6101f8565b3461054d57610537610531366004610275565b916115b2565b61053f6101f2565b80610549816102ba565b0390f35b6101f8565b90602082820312610583575f82013567ffffffffffffffff811161057e5761057a9201610210565b9091565b610200565b6101fc565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6105c96105d26020936105d7936105c081610588565b9384809361058c565b95869101610595565b6105a0565b0190565b6105f09160208201915f8184039101526105aa565b90565b346106245761062061060f610609366004610552565b90611642565b6106176101f2565b918291826105db565b0390f35b6101f8565b1c90565b60ff1690565b6106439060086106489302610629565b61062d565b90565b906106569154610633565b90565b61066560045f9061064b565b90565b34610698576106783660046102f3565b610694610683610659565b61068b6101f2565b918291826104d4565b0390f35b6101f8565b90565b90565b6106b76106b26106bc9261069d565b6106a0565b61024f565b90565b6106c9600a6106a3565b90565b6106d46106bf565b90565b6106e09061024f565b9052565b91906106f7905f602085019401906106d7565b565b34610729576107093660046102f3565b6107256107146106cc565b61071c6101f2565b918291826106e4565b0390f35b6101f8565b3461075d57610747610741366004610552565b906117dc565b61074f6101f2565b80610759816102ba565b0390f35b6101f8565b60018060a01b031690565b61077690610762565b90565b6107828161076d565b0361078957565b5f80fd5b9050359061079a82610779565b565b91906040838203126107c457806107b86107c1925f860161078d565b9360200161078d565b90565b6101fc565b346107f8576107e26107dc36600461079c565b9061198d565b6107ea6101f2565b806107f4816102ba565b0390f35b6101f8565b9060208282031261081657610813915f01610266565b90565b6101fc565b3461084b576108476108366108313660046107fd565b611999565b61083e6101f2565b91829182610369565b0390f35b6101f8565b60018060a01b031690565b61086b9060086108709302610629565b610850565b90565b9061087e915461085b565b90565b61088d60015f90610873565b90565b6108a461089f6108a992610762565b6106a0565b610762565b90565b6108b590610890565b90565b6108c1906108ac565b90565b6108cd906108b8565b9052565b91906108e4905f602085019401906108c4565b565b34610916576108f63660046102f3565b610912610901610881565b6109096101f2565b918291826108d1565b0390f35b6101f8565b90565b61092e9060086109339302610629565b61091b565b90565b90610941915461091e565b90565b61095060035f90610936565b90565b34610983576109633660046102f3565b61097f61096e610944565b6109766101f2565b918291826106e4565b0390f35b6101f8565b90565b61099f61099a6109a492610988565b6106a0565b61024f565b90565b6109b362278d0061098b565b90565b6109be6109a7565b90565b346109f1576109d13660046102f3565b6109ed6109dc6109b6565b6109e46101f2565b918291826106e4565b0390f35b6101f8565b610a0260025f90610936565b90565b34610a3557610a153660046102f3565b610a31610a206109f6565b610a286101f2565b918291826106e4565b0390f35b6101f8565b34610a6857610a4a3660046102f3565b610a526119dd565b610a5a6101f2565b80610a64816102ba565b0390f35b6101f8565b91606083830312610aba57610a84825f850161078d565b92610a92836020830161078d565b92604082013567ffffffffffffffff8111610ab557610ab19201610210565b9091565b610200565b6101fc565b34610af357610aef610ade610ad5366004610a6d565b92919091611a95565b610ae66101f2565b918291826104d4565b0390f35b6101f8565b34610b2757610b11610b0b366004610552565b90611bf8565b610b196101f2565b80610b23816102ba565b0390f35b6101f8565b34610b5c57610b3c3660046102f3565b610b58610b47611c15565b610b4f6101f2565b918291826106e4565b0390f35b6101f8565b9091606082840312610b9657610b93610b7c845f8501610266565b93610b8a8160208601610266565b93604001610266565b90565b6101fc565b34610bcc57610bc8610bb7610bb1366004610b61565b91611ce2565b610bbf6101f2565b918291826106e4565b0390f35b6101f8565b34610c0157610be13660046102f3565b610bfd610bec611d58565b610bf46101f2565b918291826106e4565b0390f35b6101f8565b610c0f9061076d565b9052565b9190610c26905f60208501940190610c06565b565b34610c5857610c383660046102f3565b610c54610c43611dea565b610c4b6101f2565b91829182610c13565b0390f35b6101f8565b34610c8d57610c6d3660046102f3565b610c89610c78611e1e565b610c806101f2565b918291826106e4565b0390f35b6101f8565b34610cc257610ca23660046102f3565b610cbe610cad611e6a565b610cb56101f2565b918291826106e4565b0390f35b6101f8565b90602082820312610cf8575f82013567ffffffffffffffff8111610cf357610cef92016103b3565b9091565b610200565b6101fc565b34610d2c57610d16610d10366004610cc7565b90611fb6565b610d1e6101f2565b80610d28816102ba565b0390f35b6101f8565b90602082820312610d4a57610d47915f0161078d565b90565b6101fc565b34610d7d57610d67610d62366004610d31565b612066565b610d6f6101f2565b80610d79816102ba565b0390f35b6101f8565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610dd657610db63660046102f3565b610dd2610dc1610d82565b610dc96101f2565b918291826106e4565b0390f35b6101f8565b610def610dea610df49261024f565b6106a0565b61024f565b90565b90610e0190610ddb565b5f5260205260405f2090565b5f1c90565b610e1e610e2391610e0d565b61091b565b90565b610e309054610e12565b90565b610e3f610e4491610e0d565b61062d565b90565b610e519054610e33565b90565b610e5f906005610df7565b90610e6b5f8301610e26565b91610e7860018201610e26565b91610e916003610e8a60028501610e26565b9301610e47565b90565b610ec9610ed094610ebf606094989795610eb5608086019a5f8701906106d7565b60208501906106d7565b60408301906106d7565b01906104c7565b565b34610f0657610f02610eed610ee83660046107fd565b610e54565b90610ef99492946101f2565b94859485610e94565b0390f35b6101f8565b34610f3957610f23610f1e366004610d31565b6120d6565b610f2b6101f2565b80610f35816102ba565b0390f35b6101f8565b5f80fd5b9190610f5f610f5933329086859192909192611a95565b1561030f565b610f6e57610f6c92610fcb565b565b5f631b8e828b60e31b815280610f86600482016102ba565b0390fd5b610f93906108ac565b90565b604090610fc2610fb7610fc99597969460608401908482035f8601526105aa565b9660208301906106d7565b01906106d7565b565b90610fd7903392611642565b9142926110196110077f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f294610f8a565b946110106101f2565b93849384610f96565b0390a2565b906110299291610f42565b565b634e487b7160e01b5f52604160045260245ffd5b90611049906105a0565b810190811067ffffffffffffffff82111761106357604052565b61102b565b9061107b6110746101f2565b928361103f565b565b6110876080611068565b90565b5f90565b5f90565b61109a61107d565b906020808080856110a961108a565b8152016110b461108a565b8152016110bf61108a565b8152016110ca61108e565b81525050565b6110d8611092565b90565b6110e56080611068565b90565b90565b6110ff6110fa611104926110e8565b6106a0565b61024f565b90565b906111119061024f565b9052565b9061111f9061030f565b9052565b9061118a611181600361113461107d565b9461114b6111435f8301610e26565b5f8801611107565b61116361115a60018301610e26565b60208801611107565b61117b61117260028301610e26565b60408801611107565b01610e47565b60608401611115565b565b61119590611123565b90565b6111a06110d0565b506111b46111ae6004610e47565b1561030f565b6111d8576111d56111d060056111ca6003610e26565b90610df7565b61118c565b90565b5f61122d5f61122461121b5f61121661120d5f956112086112006111fa6110db565b9a6110eb565b5f8b01611107565b6110eb565b60208801611107565b6110eb565b60408501611107565b60608301611115565b90565b5090565b5090565b60209181520190565b60207f7665207468652073616d65206c656e6774680000000000000000000000000000917f4461746120616e64207072696f7269747920617272617973206d7573742068615f8201520152565b61129b6032604092611238565b6112a481611241565b0190565b6112bd9060208101905f81830391015261128e565b90565b156112c757565b6112cf6101f2565b62461bcd60e51b8152806112e5600482016112a8565b0390fd5b60016112f5910161024f565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561135a570180359067ffffffffffffffff82116113555760200191600182023603831361135057565b611314565b611310565b61130c565b9082101561137a5760206113769202810190611318565b9091565b6112f8565b919081101561138f576020020190565b6112f8565b3561139e81610252565b90565b90926113ae828590611230565b936113d5856113cf6113c96113c4888790611234565b61024f565b9161024f565b146112c0565b6113de5f6110eb565b5b806113f26113ec8861024f565b9161024f565b1015611499576114209061141633329061140e8887869161135f565b929091611a95565b611425575b6112e9565b6113df565b3361143b6114358786859161135f565b90611642565b9061145061144b8988869161137f565b611394565b429261149161147f7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f294610f8a565b946114886101f2565b93849384610f96565b0390a261141b565b505050505050565b5f90565b6114ad6114a1565b506114b86004610e47565b90565b91906114d86114d233329086859192909192611a95565b1561030f565b6114e7576114e592611566565b565b5f631b8e828b60e31b8152806114ff600482016102ba565b0390fd5b90825f939282370152565b9190611528816115218161152d9561058c565b8095611503565b6105a0565b0190565b61155d6115526040936115649698979560608501918583035f87015261150e565b9660208301906106d7565b01906106d7565b565b909133919290926115ad4261159b7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f295610f8a565b956115a46101f2565b94859485611531565b0390a2565b906115bd92916114bb565b565b606090565b60ff60f81b1690565b60f81b90565b6115e76115e26115ec926110e8565b6115cd565b6115c4565b90565b90565b6115fe611603916115c4565b6115ef565b9052565b905090565b90918261161c8161162393611607565b8093611503565b0190565b8061163860019261163f96946115f2565b019161160c565b90565b6116809061164e6115bf565b5061167161165b5f6115d3565b91936116656101f2565b94859360208501611627565b6020820181038252038261103f565b90565b9061169f61169933329085859192909192611a95565b1561030f565b6116ae576116ac91611747565b565b5f631b8e828b60e31b8152806116c6600482016102ba565b0390fd5b634e487b7160e01b5f52601160045260245ffd5b6116ed6116f39193929361024f565b9261024f565b82039182116116fe57565b6116ca565b90565b61171a61171561171f92611703565b6106a0565b61024f565b90565b6117316117379193929361024f565b9261024f565b820180921161174257565b6116ca565b6117636117749161175c611779945a92611795565b5a906116de565b61176e611388611706565b90611722565b61213d565b565b90916117929260208301925f81850391015261150e565b90565b3390916117c27f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92610f8a565b926117d76117ce6101f2565b9283928361177b565b0390a2565b906117e691611683565b565b906117fa916117f56121ed565b611900565b565b60a01c90565b61180e611813916117fc565b61062d565b90565b6118209054611802565b90565b61183761183261183c926110e8565b6106a0565b610762565b90565b61184890611823565b90565b60a01b90565b9061186060ff60a01b9161184b565b9181191691161790565b6118739061030f565b90565b90565b9061188e6118896118959261186a565b611876565b8254611851565b9055565b6118a290610890565b90565b6118ae90611899565b90565b5f1b90565b906118c760018060a01b03916118b1565b9181191691161790565b6118da90611899565b90565b90565b906118f56118f06118fc926118d1565b6118dd565b82546118b6565b9055565b61190a6001611816565b611972578161192961192361191e5f61183f565b61076d565b9161076d565b146119565761194f61194861195493611943600180611879565b6118a5565b60016118e0565b6120d6565b565b5f632e7f3c7f60e11b81528061196e600482016102ba565b0390fd5b5f62dc149f60e41b815280611989600482016102ba565b0390fd5b90611997916117e8565b565b6119b06119b5916119a86110d0565b506005610df7565b61118c565b90565b6119c06121ed565b6119c86119ca565b565b6119db6119d65f61183f565b61225e565b565b6119e56119b8565b565b6119f36119f891610e0d565b610850565b90565b611a0590546119e7565b90565b60e01b90565b611a178161030f565b03611a1e57565b5f80fd5b90505190611a2f82611a0e565b565b90602082820312611a4a57611a47915f01611a22565b90565b6101fc565b611a75611a829593949294611a6b60608401965f850190610c06565b6020830190610c06565b604081850391015261150e565b90565b611a8d6101f2565b3d5f823e3d90fd5b92611ad860209394611aa56114a1565b50611ae3611abb611ab660016119fb565b6108b8565b93637a3979dc929597611acc6101f2565b98899788968796611a08565b865260048601611a4f565b03915afa908115611b27575f91611af9575b5090565b611b1a915060203d8111611b20575b611b12818361103f565b810190611a31565b5f611af5565b503d611b08565b611a85565b90611b48611b4233329085859192909192611a95565b1561030f565b611b5757611b5591611b73565b565b5f631b8e828b60e31b815280611b6f600482016102ba565b0390fd5b611b8f611ba091611b88611ba5945a92611ba7565b5a906116de565b611b9a611388611706565b90611722565b61213d565b565b90611bb3903392611642565b90611bf3611be17f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92610f8a565b92611bea6101f2565b918291826105db565b0390a2565b90611c0291611b2c565b565b5f90565b611c12905161024f565b90565b611c1d611c04565b50611c31611c2b6004610e47565b1561030f565b611ca157611c6d611c5f5f611c59611c546005611c4e6003610e26565b90610df7565b61118c565b01611c08565b611c676109a7565b90611722565b42611c80611c7a8361024f565b9161024f565b1015611c9457611c919042906116de565b90565b50611c9e5f6110eb565b90565b611caa5f6110eb565b90565b611cbc611cc29193929361024f565b9261024f565b91611cce83820261024f565b928184041490151715611cdd57565b6116ca565b91611ceb611c04565b5080611cff611cf98461024f565b9161024f565b1115611d5357611d2091611d12916116de565b611d1a6106bf565b90611cad565b80611d33611d2d8461024f565b9161024f565b1015611d4557611d42916116de565b90565b5050611d505f6110eb565b90565b505090565b611d60611c04565b50611d74611d6e6004610e47565b1561030f565b611dae57611dab611d9b6002611d956005611d8f6003610e26565b90610df7565b01610e26565b611da56002610e26565b90611cad565b90565b611db75f6110eb565b90565b5f90565b60018060a01b031690565b611dd5611dda91610e0d565b611dbe565b90565b611de79054611dc9565b90565b611df2611dba565b50611dfc5f611ddd565b90565b90565b611e16611e11611e1b92611dff565b6106a0565b61024f565b90565b611e26611c04565b50611e3a611e346004610e47565b1561030f565b611e5e57611e5b611e4b6003610e26565b611e556001611e02565b90611722565b90565b611e675f6110eb565b90565b611e72611c04565b50611e86611e806004610e47565b1561030f565b611ead57611eaa6002611ea46005611e9e6003610e26565b90610df7565b01610e26565b90565b611eb65f6110eb565b90565b611ed5611ee691611ece611eeb945a92611eed565b5a906116de565b611ee0611388611706565b90611722565b61213d565b565b611ef8818390611230565b91611f01611c04565b50611f0b5f6110eb565b5b80611f1f611f198661024f565b9161024f565b1015611fb057611f4d90611f43333290611f3b8787869161135f565b929091611a95565b611f52575b6112e9565b611f0c565b33611f68611f628686859161135f565b90611642565b90611fa8611f967f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92610f8a565b92611f9f6101f2565b918291826105db565b0390a2611f48565b50505050565b90611fc091611eb9565b565b611fd390611fce6121ed565b611fd5565b565b80611ff0611fea611fe55f61183f565b61076d565b9161076d565b1461204a57612008612001826118a5565b60016118e0565b6120327f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b991610f8a565b9061203b6101f2565b80612045816102ba565b0390a2565b5f632e7f3c7f60e11b815280612062600482016102ba565b0390fd5b61206f90611fc2565b565b6120829061207d6121ed565b612084565b565b8061209f6120996120945f61183f565b61076d565b9161076d565b146120af576120ad9061225e565b565b6120d26120bb5f61183f565b5f918291631e4fbdf760e01b835260048301610c13565b0390fd5b6120df90612071565b565b906120ed5f19916118b1565b9181191691161790565b90565b9061210f61210a61211692610ddb565b6120f7565b82546120e1565b9055565b91602061213b92949361213460408201965f8301906106d7565b01906106d7565b565b61215061214a6004610e47565b1561030f565b6121e0575b61215d61248a565b6121918161218b600261217b60056121756003610e26565b90610df7565b019161218683610e26565b611722565b906120fa565b61219b6003610e26565b3a6121c67f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610ddb565b926121db6121d26101f2565b9283928361211a565b0390a2565b6121e8612387565b612155565b6121f5611dea565b61220e612208612203612668565b61076d565b9161076d565b0361221557565b612237612220612668565b5f91829163118cdaa760e01b835260048301610c13565b0390fd5b90565b9061225361224e61225a92610f8a565b61223b565b82546118b6565b9055565b6122675f611ddd565b612271825f61223e565b906122a561229f7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093610f8a565b91610f8a565b916122ae6101f2565b806122b8816102ba565b0390a3565b906122c960ff916118b1565b9181191691161790565b906122e86122e36122ef9261186a565b611876565b82546122bd565b9055565b906122fd906110eb565b5f5260205260405f2090565b612313905161030f565b90565b9061237360606003612379946123395f82016123335f8801611c08565b906120fa565b6123526001820161234c60208801611c08565b906120fa565b61236b6002820161236560408801611c08565b906120fa565b019201612309565b906122d3565b565b9061238591612316565b565b61239a6123946004610e47565b1561030f565b6123a1575b565b6123ad600160046122d3565b6123c06123b95f6110eb565b60036120fa565b612421426124105f6124076123fe5f6123f96123f05f956123eb6123e26110db565b995f8b01611107565b6110eb565b60208801611107565b6110eb565b60408501611107565b60608301611115565b61241c60055f906122f3565b61237b565b5f42906124636124517f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e926110eb565b9261245a6101f2565b918291826106e4565b0390a261239f565b90565b6124779061024f565b5f1981146124855760010190565b6116ca565b6124a76124a2600561249c6003610e26565b90610df7565b61246b565b426124d56124cf6124ca6124bc5f8601610e26565b6124c46109a7565b90611722565b61024f565b9161024f565b10156124df575b50565b6125076124fe6124f05f8401610e26565b6124f86109a7565b90611722565b600183016120fa565b6125156001600383016122d3565b61251f6003610e26565b61254c61252e60028401610e26565b926125465f61253f60018401610e26565b9201610e26565b906116de565b6125767f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610ddb565b9261258b6125826101f2565b9283928361211a565b0390a26125aa6125a361259e6003610e26565b61246e565b60036120fa565b612614426125fa5f6125f16125e85f6125e36125da5f956125d56125cc6110db565b995f8b01611107565b6110eb565b60208801611107565b6110eb565b60408501611107565b60608301611115565b61260f60056126096003610e26565b90610df7565b61237b565b61261e6003610e26565b429061265f61264d7f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610ddb565b926126566101f2565b918291826106e4565b0390a25f6124dc565b612670611dba565b50339056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x0F>V[a\0\x1D_5a\x01\xECV[\x80c\x05\x0E\xC18\x14a\x01\xE7W\x80c\x08aF\xD2\x14a\x01\xE2W\x80c\x11\x99/\x8C\x14a\x01\xDDW\x80c\x18\xD5\xAA\xFE\x14a\x01\xD8W\x80c\x1C\x0Bcg\x14a\x01\xD3W\x80c6l\xBA\xB7\x14a\x01\xCEW\x80c;j\xB2\xA9\x14a\x01\xC9W\x80c=D\xAE\x8B\x14a\x01\xC4W\x80cF\xE2\xCC\t\x14a\x01\xBFW\x80cH\\\xC9U\x14a\x01\xBAW\x80cK,\x07\x06\x14a\x01\xB5W\x80c[<\xD6\xE2\x14a\x01\xB0W\x80caT8\x01\x14a\x01\xABW\x80ceX\x95O\x14a\x01\xA6W\x80cp<\xFC\xBB\x14a\x01\xA1W\x80cqP\x18\xA6\x14a\x01\x9CW\x80cz9y\xDC\x14a\x01\x97W\x80c\x80NQ#\x14a\x01\x92W\x80c\x82\xF4J\xDE\x14a\x01\x8DW\x80c\x83\xD3\xC1\x15\x14a\x01\x88W\x80c\x8DZ#\x9B\x14a\x01\x83W\x80c\x8D\xA5\xCB[\x14a\x01~W\x80c\xAF\xF7Lm\x14a\x01yW\x80c\xC6`\xD3\xF3\x14a\x01tW\x80c\xCD\xAF\xB9x\x14a\x01oW\x80c\xD4\xF0\xEBM\x14a\x01jW\x80c\xD8x\x13B\x14a\x01eW\x80c\xEAJ\x11\x04\x14a\x01`Wc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\x0F\x0BV[a\x0E\xD2V[a\r\xA6V[a\rOV[a\x0C\xFDV[a\x0C\x92V[a\x0C]V[a\x0C(V[a\x0B\xD1V[a\x0B\x9BV[a\x0B,V[a\n\xF8V[a\n\xBFV[a\n:V[a\n\x05V[a\t\xC1V[a\tSV[a\x08\xE6V[a\x08\x1BV[a\x07\xC9V[a\x07.V[a\x06\xF9V[a\x06hV[a\x05\xF3V[a\x05\x1EV[a\x04\xE9V[a\x04\x90V[a\x03~V[a\x02\xBFV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x02JW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x02EW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x02@WV[a\x02\x0CV[a\x02\x08V[a\x02\x04V[\x90V[a\x02[\x81a\x02OV[\x03a\x02bWV[_\x80\xFD[\x90P5\x90a\x02s\x82a\x02RV[V[\x91`@\x83\x83\x03\x12a\x02\xB5W_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02\xB0Wa\x02\xA2\x83a\x02\xAD\x92\x86\x01a\x02\x10V[\x93\x90\x94` \x01a\x02fV[\x90V[a\x02\0V[a\x01\xFCV[_\x01\x90V[4a\x02\xEEWa\x02\xD8a\x02\xD26`\x04a\x02uV[\x91a\x10\x1EV[a\x02\xE0a\x01\xF2V[\x80a\x02\xEA\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[_\x91\x03\x12a\x02\xFDWV[a\x01\xFCV[a\x03\x0B\x90a\x02OV[\x90RV[\x15\x15\x90V[a\x03\x1D\x90a\x03\x0FV[\x90RV[\x90``\x80a\x03g\x93a\x039_\x82\x01Q_\x86\x01\x90a\x03\x02V[a\x03K` \x82\x01Q` \x86\x01\x90a\x03\x02V[a\x03]`@\x82\x01Q`@\x86\x01\x90a\x03\x02V[\x01Q\x91\x01\x90a\x03\x14V[V[\x91\x90a\x03|\x90_`\x80\x85\x01\x94\x01\x90a\x03!V[V[4a\x03\xAEWa\x03\x8E6`\x04a\x02\xF3V[a\x03\xAAa\x03\x99a\x11\x98V[a\x03\xA1a\x01\xF2V[\x91\x82\x91\x82a\x03iV[\x03\x90\xF3[a\x01\xF8V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03\xEDW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\xE8W` \x01\x92` \x83\x02\x84\x01\x11a\x03\xE3WV[a\x02\x0CV[a\x02\x08V[a\x02\x04V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04,W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04'W` \x01\x92` \x83\x02\x84\x01\x11a\x04\"WV[a\x02\x0CV[a\x02\x08V[a\x02\x04V[\x90\x91`@\x82\x84\x03\x12a\x04\x8BW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x86W\x83a\x04\\\x91\x84\x01a\x03\xB3V[\x92\x90\x93` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x81Wa\x04}\x92\x01a\x03\xF2V[\x90\x91V[a\x02\0V[a\x02\0V[a\x01\xFCV[4a\x04\xC2Wa\x04\xACa\x04\xA36`\x04a\x041V[\x92\x91\x90\x91a\x13\xA1V[a\x04\xB4a\x01\xF2V[\x80a\x04\xBE\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[a\x04\xD0\x90a\x03\x0FV[\x90RV[\x91\x90a\x04\xE7\x90_` \x85\x01\x94\x01\x90a\x04\xC7V[V[4a\x05\x19Wa\x04\xF96`\x04a\x02\xF3V[a\x05\x15a\x05\x04a\x14\xA5V[a\x05\x0Ca\x01\xF2V[\x91\x82\x91\x82a\x04\xD4V[\x03\x90\xF3[a\x01\xF8V[4a\x05MWa\x057a\x0516`\x04a\x02uV[\x91a\x15\xB2V[a\x05?a\x01\xF2V[\x80a\x05I\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[\x90` \x82\x82\x03\x12a\x05\x83W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05~Wa\x05z\x92\x01a\x02\x10V[\x90\x91V[a\x02\0V[a\x01\xFCV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x05\xC9a\x05\xD2` \x93a\x05\xD7\x93a\x05\xC0\x81a\x05\x88V[\x93\x84\x80\x93a\x05\x8CV[\x95\x86\x91\x01a\x05\x95V[a\x05\xA0V[\x01\x90V[a\x05\xF0\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x05\xAAV[\x90V[4a\x06$Wa\x06 a\x06\x0Fa\x06\t6`\x04a\x05RV[\x90a\x16BV[a\x06\x17a\x01\xF2V[\x91\x82\x91\x82a\x05\xDBV[\x03\x90\xF3[a\x01\xF8V[\x1C\x90V[`\xFF\x16\x90V[a\x06C\x90`\x08a\x06H\x93\x02a\x06)V[a\x06-V[\x90V[\x90a\x06V\x91Ta\x063V[\x90V[a\x06e`\x04_\x90a\x06KV[\x90V[4a\x06\x98Wa\x06x6`\x04a\x02\xF3V[a\x06\x94a\x06\x83a\x06YV[a\x06\x8Ba\x01\xF2V[\x91\x82\x91\x82a\x04\xD4V[\x03\x90\xF3[a\x01\xF8V[\x90V[\x90V[a\x06\xB7a\x06\xB2a\x06\xBC\x92a\x06\x9DV[a\x06\xA0V[a\x02OV[\x90V[a\x06\xC9`\na\x06\xA3V[\x90V[a\x06\xD4a\x06\xBFV[\x90V[a\x06\xE0\x90a\x02OV[\x90RV[\x91\x90a\x06\xF7\x90_` \x85\x01\x94\x01\x90a\x06\xD7V[V[4a\x07)Wa\x07\t6`\x04a\x02\xF3V[a\x07%a\x07\x14a\x06\xCCV[a\x07\x1Ca\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[4a\x07]Wa\x07Ga\x07A6`\x04a\x05RV[\x90a\x17\xDCV[a\x07Oa\x01\xF2V[\x80a\x07Y\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x07v\x90a\x07bV[\x90V[a\x07\x82\x81a\x07mV[\x03a\x07\x89WV[_\x80\xFD[\x90P5\x90a\x07\x9A\x82a\x07yV[V[\x91\x90`@\x83\x82\x03\x12a\x07\xC4W\x80a\x07\xB8a\x07\xC1\x92_\x86\x01a\x07\x8DV[\x93` \x01a\x07\x8DV[\x90V[a\x01\xFCV[4a\x07\xF8Wa\x07\xE2a\x07\xDC6`\x04a\x07\x9CV[\x90a\x19\x8DV[a\x07\xEAa\x01\xF2V[\x80a\x07\xF4\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[\x90` \x82\x82\x03\x12a\x08\x16Wa\x08\x13\x91_\x01a\x02fV[\x90V[a\x01\xFCV[4a\x08KWa\x08Ga\x086a\x0816`\x04a\x07\xFDV[a\x19\x99V[a\x08>a\x01\xF2V[\x91\x82\x91\x82a\x03iV[\x03\x90\xF3[a\x01\xF8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08k\x90`\x08a\x08p\x93\x02a\x06)V[a\x08PV[\x90V[\x90a\x08~\x91Ta\x08[V[\x90V[a\x08\x8D`\x01_\x90a\x08sV[\x90V[a\x08\xA4a\x08\x9Fa\x08\xA9\x92a\x07bV[a\x06\xA0V[a\x07bV[\x90V[a\x08\xB5\x90a\x08\x90V[\x90V[a\x08\xC1\x90a\x08\xACV[\x90V[a\x08\xCD\x90a\x08\xB8V[\x90RV[\x91\x90a\x08\xE4\x90_` \x85\x01\x94\x01\x90a\x08\xC4V[V[4a\t\x16Wa\x08\xF66`\x04a\x02\xF3V[a\t\x12a\t\x01a\x08\x81V[a\t\ta\x01\xF2V[\x91\x82\x91\x82a\x08\xD1V[\x03\x90\xF3[a\x01\xF8V[\x90V[a\t.\x90`\x08a\t3\x93\x02a\x06)V[a\t\x1BV[\x90V[\x90a\tA\x91Ta\t\x1EV[\x90V[a\tP`\x03_\x90a\t6V[\x90V[4a\t\x83Wa\tc6`\x04a\x02\xF3V[a\t\x7Fa\tna\tDV[a\tva\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[\x90V[a\t\x9Fa\t\x9Aa\t\xA4\x92a\t\x88V[a\x06\xA0V[a\x02OV[\x90V[a\t\xB3b'\x8D\0a\t\x8BV[\x90V[a\t\xBEa\t\xA7V[\x90V[4a\t\xF1Wa\t\xD16`\x04a\x02\xF3V[a\t\xEDa\t\xDCa\t\xB6V[a\t\xE4a\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[a\n\x02`\x02_\x90a\t6V[\x90V[4a\n5Wa\n\x156`\x04a\x02\xF3V[a\n1a\n a\t\xF6V[a\n(a\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[4a\nhWa\nJ6`\x04a\x02\xF3V[a\nRa\x19\xDDV[a\nZa\x01\xF2V[\x80a\nd\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[\x91``\x83\x83\x03\x12a\n\xBAWa\n\x84\x82_\x85\x01a\x07\x8DV[\x92a\n\x92\x83` \x83\x01a\x07\x8DV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xB5Wa\n\xB1\x92\x01a\x02\x10V[\x90\x91V[a\x02\0V[a\x01\xFCV[4a\n\xF3Wa\n\xEFa\n\xDEa\n\xD56`\x04a\nmV[\x92\x91\x90\x91a\x1A\x95V[a\n\xE6a\x01\xF2V[\x91\x82\x91\x82a\x04\xD4V[\x03\x90\xF3[a\x01\xF8V[4a\x0B'Wa\x0B\x11a\x0B\x0B6`\x04a\x05RV[\x90a\x1B\xF8V[a\x0B\x19a\x01\xF2V[\x80a\x0B#\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[4a\x0B\\Wa\x0B<6`\x04a\x02\xF3V[a\x0BXa\x0BGa\x1C\x15V[a\x0BOa\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[\x90\x91``\x82\x84\x03\x12a\x0B\x96Wa\x0B\x93a\x0B|\x84_\x85\x01a\x02fV[\x93a\x0B\x8A\x81` \x86\x01a\x02fV[\x93`@\x01a\x02fV[\x90V[a\x01\xFCV[4a\x0B\xCCWa\x0B\xC8a\x0B\xB7a\x0B\xB16`\x04a\x0BaV[\x91a\x1C\xE2V[a\x0B\xBFa\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[4a\x0C\x01Wa\x0B\xE16`\x04a\x02\xF3V[a\x0B\xFDa\x0B\xECa\x1DXV[a\x0B\xF4a\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[a\x0C\x0F\x90a\x07mV[\x90RV[\x91\x90a\x0C&\x90_` \x85\x01\x94\x01\x90a\x0C\x06V[V[4a\x0CXWa\x0C86`\x04a\x02\xF3V[a\x0CTa\x0CCa\x1D\xEAV[a\x0CKa\x01\xF2V[\x91\x82\x91\x82a\x0C\x13V[\x03\x90\xF3[a\x01\xF8V[4a\x0C\x8DWa\x0Cm6`\x04a\x02\xF3V[a\x0C\x89a\x0Cxa\x1E\x1EV[a\x0C\x80a\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[4a\x0C\xC2Wa\x0C\xA26`\x04a\x02\xF3V[a\x0C\xBEa\x0C\xADa\x1EjV[a\x0C\xB5a\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[\x90` \x82\x82\x03\x12a\x0C\xF8W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0C\xF3Wa\x0C\xEF\x92\x01a\x03\xB3V[\x90\x91V[a\x02\0V[a\x01\xFCV[4a\r,Wa\r\x16a\r\x106`\x04a\x0C\xC7V[\x90a\x1F\xB6V[a\r\x1Ea\x01\xF2V[\x80a\r(\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[\x90` \x82\x82\x03\x12a\rJWa\rG\x91_\x01a\x07\x8DV[\x90V[a\x01\xFCV[4a\r}Wa\rga\rb6`\x04a\r1V[a fV[a\roa\x01\xF2V[\x80a\ry\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\r\xD6Wa\r\xB66`\x04a\x02\xF3V[a\r\xD2a\r\xC1a\r\x82V[a\r\xC9a\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xF3[a\x01\xF8V[a\r\xEFa\r\xEAa\r\xF4\x92a\x02OV[a\x06\xA0V[a\x02OV[\x90V[\x90a\x0E\x01\x90a\r\xDBV[_R` R`@_ \x90V[_\x1C\x90V[a\x0E\x1Ea\x0E#\x91a\x0E\rV[a\t\x1BV[\x90V[a\x0E0\x90Ta\x0E\x12V[\x90V[a\x0E?a\x0ED\x91a\x0E\rV[a\x06-V[\x90V[a\x0EQ\x90Ta\x0E3V[\x90V[a\x0E_\x90`\x05a\r\xF7V[\x90a\x0Ek_\x83\x01a\x0E&V[\x91a\x0Ex`\x01\x82\x01a\x0E&V[\x91a\x0E\x91`\x03a\x0E\x8A`\x02\x85\x01a\x0E&V[\x93\x01a\x0EGV[\x90V[a\x0E\xC9a\x0E\xD0\x94a\x0E\xBF``\x94\x98\x97\x95a\x0E\xB5`\x80\x86\x01\x9A_\x87\x01\x90a\x06\xD7V[` \x85\x01\x90a\x06\xD7V[`@\x83\x01\x90a\x06\xD7V[\x01\x90a\x04\xC7V[V[4a\x0F\x06Wa\x0F\x02a\x0E\xEDa\x0E\xE86`\x04a\x07\xFDV[a\x0ETV[\x90a\x0E\xF9\x94\x92\x94a\x01\xF2V[\x94\x85\x94\x85a\x0E\x94V[\x03\x90\xF3[a\x01\xF8V[4a\x0F9Wa\x0F#a\x0F\x1E6`\x04a\r1V[a \xD6V[a\x0F+a\x01\xF2V[\x80a\x0F5\x81a\x02\xBAV[\x03\x90\xF3[a\x01\xF8V[_\x80\xFD[\x91\x90a\x0F_a\x0FY32\x90\x86\x85\x91\x92\x90\x91\x92a\x1A\x95V[\x15a\x03\x0FV[a\x0FnWa\x0Fl\x92a\x0F\xCBV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x0F\x86`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[a\x0F\x93\x90a\x08\xACV[\x90V[`@\x90a\x0F\xC2a\x0F\xB7a\x0F\xC9\x95\x97\x96\x94``\x84\x01\x90\x84\x82\x03_\x86\x01Ra\x05\xAAV[\x96` \x83\x01\x90a\x06\xD7V[\x01\x90a\x06\xD7V[V[\x90a\x0F\xD7\x903\x92a\x16BV[\x91B\x92a\x10\x19a\x10\x07\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x0F\x8AV[\x94a\x10\x10a\x01\xF2V[\x93\x84\x93\x84a\x0F\x96V[\x03\x90\xA2V[\x90a\x10)\x92\x91a\x0FBV[V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x10I\x90a\x05\xA0V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10cW`@RV[a\x10+V[\x90a\x10{a\x10ta\x01\xF2V[\x92\x83a\x10?V[V[a\x10\x87`\x80a\x10hV[\x90V[_\x90V[_\x90V[a\x10\x9Aa\x10}V[\x90` \x80\x80\x80\x85a\x10\xA9a\x10\x8AV[\x81R\x01a\x10\xB4a\x10\x8AV[\x81R\x01a\x10\xBFa\x10\x8AV[\x81R\x01a\x10\xCAa\x10\x8EV[\x81RPPV[a\x10\xD8a\x10\x92V[\x90V[a\x10\xE5`\x80a\x10hV[\x90V[\x90V[a\x10\xFFa\x10\xFAa\x11\x04\x92a\x10\xE8V[a\x06\xA0V[a\x02OV[\x90V[\x90a\x11\x11\x90a\x02OV[\x90RV[\x90a\x11\x1F\x90a\x03\x0FV[\x90RV[\x90a\x11\x8Aa\x11\x81`\x03a\x114a\x10}V[\x94a\x11Ka\x11C_\x83\x01a\x0E&V[_\x88\x01a\x11\x07V[a\x11ca\x11Z`\x01\x83\x01a\x0E&V[` \x88\x01a\x11\x07V[a\x11{a\x11r`\x02\x83\x01a\x0E&V[`@\x88\x01a\x11\x07V[\x01a\x0EGV[``\x84\x01a\x11\x15V[V[a\x11\x95\x90a\x11#V[\x90V[a\x11\xA0a\x10\xD0V[Pa\x11\xB4a\x11\xAE`\x04a\x0EGV[\x15a\x03\x0FV[a\x11\xD8Wa\x11\xD5a\x11\xD0`\x05a\x11\xCA`\x03a\x0E&V[\x90a\r\xF7V[a\x11\x8CV[\x90V[_a\x12-_a\x12$a\x12\x1B_a\x12\x16a\x12\r_\x95a\x12\x08a\x12\0a\x11\xFAa\x10\xDBV[\x9Aa\x10\xEBV[_\x8B\x01a\x11\x07V[a\x10\xEBV[` \x88\x01a\x11\x07V[a\x10\xEBV[`@\x85\x01a\x11\x07V[``\x83\x01a\x11\x15V[\x90V[P\x90V[P\x90V[` \x91\x81R\x01\x90V[` \x7Fve the same length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x7FData and priority arrays must ha_\x82\x01R\x01RV[a\x12\x9B`2`@\x92a\x128V[a\x12\xA4\x81a\x12AV[\x01\x90V[a\x12\xBD\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x12\x8EV[\x90V[\x15a\x12\xC7WV[a\x12\xCFa\x01\xF2V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x12\xE5`\x04\x82\x01a\x12\xA8V[\x03\x90\xFD[`\x01a\x12\xF5\x91\x01a\x02OV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x13ZW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x13UW` \x01\x91`\x01\x82\x026\x03\x83\x13a\x13PWV[a\x13\x14V[a\x13\x10V[a\x13\x0CV[\x90\x82\x10\x15a\x13zW` a\x13v\x92\x02\x81\x01\x90a\x13\x18V[\x90\x91V[a\x12\xF8V[\x91\x90\x81\x10\x15a\x13\x8FW` \x02\x01\x90V[a\x12\xF8V[5a\x13\x9E\x81a\x02RV[\x90V[\x90\x92a\x13\xAE\x82\x85\x90a\x120V[\x93a\x13\xD5\x85a\x13\xCFa\x13\xC9a\x13\xC4\x88\x87\x90a\x124V[a\x02OV[\x91a\x02OV[\x14a\x12\xC0V[a\x13\xDE_a\x10\xEBV[[\x80a\x13\xF2a\x13\xEC\x88a\x02OV[\x91a\x02OV[\x10\x15a\x14\x99Wa\x14 \x90a\x14\x1632\x90a\x14\x0E\x88\x87\x86\x91a\x13_V[\x92\x90\x91a\x1A\x95V[a\x14%W[a\x12\xE9V[a\x13\xDFV[3a\x14;a\x145\x87\x86\x85\x91a\x13_V[\x90a\x16BV[\x90a\x14Pa\x14K\x89\x88\x86\x91a\x13\x7FV[a\x13\x94V[B\x92a\x14\x91a\x14\x7F\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x0F\x8AV[\x94a\x14\x88a\x01\xF2V[\x93\x84\x93\x84a\x0F\x96V[\x03\x90\xA2a\x14\x1BV[PPPPPPV[_\x90V[a\x14\xADa\x14\xA1V[Pa\x14\xB8`\x04a\x0EGV[\x90V[\x91\x90a\x14\xD8a\x14\xD232\x90\x86\x85\x91\x92\x90\x91\x92a\x1A\x95V[\x15a\x03\x0FV[a\x14\xE7Wa\x14\xE5\x92a\x15fV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x14\xFF`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[\x90\x82_\x93\x92\x827\x01RV[\x91\x90a\x15(\x81a\x15!\x81a\x15-\x95a\x05\x8CV[\x80\x95a\x15\x03V[a\x05\xA0V[\x01\x90V[a\x15]a\x15R`@\x93a\x15d\x96\x98\x97\x95``\x85\x01\x91\x85\x83\x03_\x87\x01Ra\x15\x0EV[\x96` \x83\x01\x90a\x06\xD7V[\x01\x90a\x06\xD7V[V[\x90\x913\x91\x92\x90\x92a\x15\xADBa\x15\x9B\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x95a\x0F\x8AV[\x95a\x15\xA4a\x01\xF2V[\x94\x85\x94\x85a\x151V[\x03\x90\xA2V[\x90a\x15\xBD\x92\x91a\x14\xBBV[V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x15\xE7a\x15\xE2a\x15\xEC\x92a\x10\xE8V[a\x15\xCDV[a\x15\xC4V[\x90V[\x90V[a\x15\xFEa\x16\x03\x91a\x15\xC4V[a\x15\xEFV[\x90RV[\x90P\x90V[\x90\x91\x82a\x16\x1C\x81a\x16#\x93a\x16\x07V[\x80\x93a\x15\x03V[\x01\x90V[\x80a\x168`\x01\x92a\x16?\x96\x94a\x15\xF2V[\x01\x91a\x16\x0CV[\x90V[a\x16\x80\x90a\x16Na\x15\xBFV[Pa\x16qa\x16[_a\x15\xD3V[\x91\x93a\x16ea\x01\xF2V[\x94\x85\x93` \x85\x01a\x16'V[` \x82\x01\x81\x03\x82R\x03\x82a\x10?V[\x90V[\x90a\x16\x9Fa\x16\x9932\x90\x85\x85\x91\x92\x90\x91\x92a\x1A\x95V[\x15a\x03\x0FV[a\x16\xAEWa\x16\xAC\x91a\x17GV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x16\xC6`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x16\xEDa\x16\xF3\x91\x93\x92\x93a\x02OV[\x92a\x02OV[\x82\x03\x91\x82\x11a\x16\xFEWV[a\x16\xCAV[\x90V[a\x17\x1Aa\x17\x15a\x17\x1F\x92a\x17\x03V[a\x06\xA0V[a\x02OV[\x90V[a\x171a\x177\x91\x93\x92\x93a\x02OV[\x92a\x02OV[\x82\x01\x80\x92\x11a\x17BWV[a\x16\xCAV[a\x17ca\x17t\x91a\x17\\a\x17y\x94Z\x92a\x17\x95V[Z\x90a\x16\xDEV[a\x17na\x13\x88a\x17\x06V[\x90a\x17\"V[a!=V[V[\x90\x91a\x17\x92\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x15\x0EV[\x90V[3\x90\x91a\x17\xC2\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x0F\x8AV[\x92a\x17\xD7a\x17\xCEa\x01\xF2V[\x92\x83\x92\x83a\x17{V[\x03\x90\xA2V[\x90a\x17\xE6\x91a\x16\x83V[V[\x90a\x17\xFA\x91a\x17\xF5a!\xEDV[a\x19\0V[V[`\xA0\x1C\x90V[a\x18\x0Ea\x18\x13\x91a\x17\xFCV[a\x06-V[\x90V[a\x18 \x90Ta\x18\x02V[\x90V[a\x187a\x182a\x18<\x92a\x10\xE8V[a\x06\xA0V[a\x07bV[\x90V[a\x18H\x90a\x18#V[\x90V[`\xA0\x1B\x90V[\x90a\x18``\xFF`\xA0\x1B\x91a\x18KV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18s\x90a\x03\x0FV[\x90V[\x90V[\x90a\x18\x8Ea\x18\x89a\x18\x95\x92a\x18jV[a\x18vV[\x82Ta\x18QV[\x90UV[a\x18\xA2\x90a\x08\x90V[\x90V[a\x18\xAE\x90a\x18\x99V[\x90V[_\x1B\x90V[\x90a\x18\xC7`\x01\x80`\xA0\x1B\x03\x91a\x18\xB1V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18\xDA\x90a\x18\x99V[\x90V[\x90V[\x90a\x18\xF5a\x18\xF0a\x18\xFC\x92a\x18\xD1V[a\x18\xDDV[\x82Ta\x18\xB6V[\x90UV[a\x19\n`\x01a\x18\x16V[a\x19rW\x81a\x19)a\x19#a\x19\x1E_a\x18?V[a\x07mV[\x91a\x07mV[\x14a\x19VWa\x19Oa\x19Ha\x19T\x93a\x19C`\x01\x80a\x18yV[a\x18\xA5V[`\x01a\x18\xE0V[a \xD6V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x19n`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x19\x89`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[\x90a\x19\x97\x91a\x17\xE8V[V[a\x19\xB0a\x19\xB5\x91a\x19\xA8a\x10\xD0V[P`\x05a\r\xF7V[a\x11\x8CV[\x90V[a\x19\xC0a!\xEDV[a\x19\xC8a\x19\xCAV[V[a\x19\xDBa\x19\xD6_a\x18?V[a\"^V[V[a\x19\xE5a\x19\xB8V[V[a\x19\xF3a\x19\xF8\x91a\x0E\rV[a\x08PV[\x90V[a\x1A\x05\x90Ta\x19\xE7V[\x90V[`\xE0\x1B\x90V[a\x1A\x17\x81a\x03\x0FV[\x03a\x1A\x1EWV[_\x80\xFD[\x90PQ\x90a\x1A/\x82a\x1A\x0EV[V[\x90` \x82\x82\x03\x12a\x1AJWa\x1AG\x91_\x01a\x1A\"V[\x90V[a\x01\xFCV[a\x1Aua\x1A\x82\x95\x93\x94\x92\x94a\x1Ak``\x84\x01\x96_\x85\x01\x90a\x0C\x06V[` \x83\x01\x90a\x0C\x06V[`@\x81\x85\x03\x91\x01Ra\x15\x0EV[\x90V[a\x1A\x8Da\x01\xF2V[=_\x82>=\x90\xFD[\x92a\x1A\xD8` \x93\x94a\x1A\xA5a\x14\xA1V[Pa\x1A\xE3a\x1A\xBBa\x1A\xB6`\x01a\x19\xFBV[a\x08\xB8V[\x93cz9y\xDC\x92\x95\x97a\x1A\xCCa\x01\xF2V[\x98\x89\x97\x88\x96\x87\x96a\x1A\x08V[\x86R`\x04\x86\x01a\x1AOV[\x03\x91Z\xFA\x90\x81\x15a\x1B'W_\x91a\x1A\xF9W[P\x90V[a\x1B\x1A\x91P` =\x81\x11a\x1B W[a\x1B\x12\x81\x83a\x10?V[\x81\x01\x90a\x1A1V[_a\x1A\xF5V[P=a\x1B\x08V[a\x1A\x85V[\x90a\x1BHa\x1BB32\x90\x85\x85\x91\x92\x90\x91\x92a\x1A\x95V[\x15a\x03\x0FV[a\x1BWWa\x1BU\x91a\x1BsV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x1Bo`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[a\x1B\x8Fa\x1B\xA0\x91a\x1B\x88a\x1B\xA5\x94Z\x92a\x1B\xA7V[Z\x90a\x16\xDEV[a\x1B\x9Aa\x13\x88a\x17\x06V[\x90a\x17\"V[a!=V[V[\x90a\x1B\xB3\x903\x92a\x16BV[\x90a\x1B\xF3a\x1B\xE1\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x0F\x8AV[\x92a\x1B\xEAa\x01\xF2V[\x91\x82\x91\x82a\x05\xDBV[\x03\x90\xA2V[\x90a\x1C\x02\x91a\x1B,V[V[_\x90V[a\x1C\x12\x90Qa\x02OV[\x90V[a\x1C\x1Da\x1C\x04V[Pa\x1C1a\x1C+`\x04a\x0EGV[\x15a\x03\x0FV[a\x1C\xA1Wa\x1Cma\x1C__a\x1CYa\x1CT`\x05a\x1CN`\x03a\x0E&V[\x90a\r\xF7V[a\x11\x8CV[\x01a\x1C\x08V[a\x1Cga\t\xA7V[\x90a\x17\"V[Ba\x1C\x80a\x1Cz\x83a\x02OV[\x91a\x02OV[\x10\x15a\x1C\x94Wa\x1C\x91\x90B\x90a\x16\xDEV[\x90V[Pa\x1C\x9E_a\x10\xEBV[\x90V[a\x1C\xAA_a\x10\xEBV[\x90V[a\x1C\xBCa\x1C\xC2\x91\x93\x92\x93a\x02OV[\x92a\x02OV[\x91a\x1C\xCE\x83\x82\x02a\x02OV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1C\xDDWV[a\x16\xCAV[\x91a\x1C\xEBa\x1C\x04V[P\x80a\x1C\xFFa\x1C\xF9\x84a\x02OV[\x91a\x02OV[\x11\x15a\x1DSWa\x1D \x91a\x1D\x12\x91a\x16\xDEV[a\x1D\x1Aa\x06\xBFV[\x90a\x1C\xADV[\x80a\x1D3a\x1D-\x84a\x02OV[\x91a\x02OV[\x10\x15a\x1DEWa\x1DB\x91a\x16\xDEV[\x90V[PPa\x1DP_a\x10\xEBV[\x90V[PP\x90V[a\x1D`a\x1C\x04V[Pa\x1Dta\x1Dn`\x04a\x0EGV[\x15a\x03\x0FV[a\x1D\xAEWa\x1D\xABa\x1D\x9B`\x02a\x1D\x95`\x05a\x1D\x8F`\x03a\x0E&V[\x90a\r\xF7V[\x01a\x0E&V[a\x1D\xA5`\x02a\x0E&V[\x90a\x1C\xADV[\x90V[a\x1D\xB7_a\x10\xEBV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1D\xD5a\x1D\xDA\x91a\x0E\rV[a\x1D\xBEV[\x90V[a\x1D\xE7\x90Ta\x1D\xC9V[\x90V[a\x1D\xF2a\x1D\xBAV[Pa\x1D\xFC_a\x1D\xDDV[\x90V[\x90V[a\x1E\x16a\x1E\x11a\x1E\x1B\x92a\x1D\xFFV[a\x06\xA0V[a\x02OV[\x90V[a\x1E&a\x1C\x04V[Pa\x1E:a\x1E4`\x04a\x0EGV[\x15a\x03\x0FV[a\x1E^Wa\x1E[a\x1EK`\x03a\x0E&V[a\x1EU`\x01a\x1E\x02V[\x90a\x17\"V[\x90V[a\x1Eg_a\x10\xEBV[\x90V[a\x1Era\x1C\x04V[Pa\x1E\x86a\x1E\x80`\x04a\x0EGV[\x15a\x03\x0FV[a\x1E\xADWa\x1E\xAA`\x02a\x1E\xA4`\x05a\x1E\x9E`\x03a\x0E&V[\x90a\r\xF7V[\x01a\x0E&V[\x90V[a\x1E\xB6_a\x10\xEBV[\x90V[a\x1E\xD5a\x1E\xE6\x91a\x1E\xCEa\x1E\xEB\x94Z\x92a\x1E\xEDV[Z\x90a\x16\xDEV[a\x1E\xE0a\x13\x88a\x17\x06V[\x90a\x17\"V[a!=V[V[a\x1E\xF8\x81\x83\x90a\x120V[\x91a\x1F\x01a\x1C\x04V[Pa\x1F\x0B_a\x10\xEBV[[\x80a\x1F\x1Fa\x1F\x19\x86a\x02OV[\x91a\x02OV[\x10\x15a\x1F\xB0Wa\x1FM\x90a\x1FC32\x90a\x1F;\x87\x87\x86\x91a\x13_V[\x92\x90\x91a\x1A\x95V[a\x1FRW[a\x12\xE9V[a\x1F\x0CV[3a\x1Fha\x1Fb\x86\x86\x85\x91a\x13_V[\x90a\x16BV[\x90a\x1F\xA8a\x1F\x96\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x0F\x8AV[\x92a\x1F\x9Fa\x01\xF2V[\x91\x82\x91\x82a\x05\xDBV[\x03\x90\xA2a\x1FHV[PPPPV[\x90a\x1F\xC0\x91a\x1E\xB9V[V[a\x1F\xD3\x90a\x1F\xCEa!\xEDV[a\x1F\xD5V[V[\x80a\x1F\xF0a\x1F\xEAa\x1F\xE5_a\x18?V[a\x07mV[\x91a\x07mV[\x14a JWa \x08a \x01\x82a\x18\xA5V[`\x01a\x18\xE0V[a 2\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x0F\x8AV[\x90a ;a\x01\xF2V[\x80a E\x81a\x02\xBAV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a b`\x04\x82\x01a\x02\xBAV[\x03\x90\xFD[a o\x90a\x1F\xC2V[V[a \x82\x90a }a!\xEDV[a \x84V[V[\x80a \x9Fa \x99a \x94_a\x18?V[a\x07mV[\x91a\x07mV[\x14a \xAFWa \xAD\x90a\"^V[V[a \xD2a \xBB_a\x18?V[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x0C\x13V[\x03\x90\xFD[a \xDF\x90a qV[V[\x90a \xED_\x19\x91a\x18\xB1V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a!\x0Fa!\na!\x16\x92a\r\xDBV[a \xF7V[\x82Ta \xE1V[\x90UV[\x91` a!;\x92\x94\x93a!4`@\x82\x01\x96_\x83\x01\x90a\x06\xD7V[\x01\x90a\x06\xD7V[V[a!Pa!J`\x04a\x0EGV[\x15a\x03\x0FV[a!\xE0W[a!]a$\x8AV[a!\x91\x81a!\x8B`\x02a!{`\x05a!u`\x03a\x0E&V[\x90a\r\xF7V[\x01\x91a!\x86\x83a\x0E&V[a\x17\"V[\x90a \xFAV[a!\x9B`\x03a\x0E&V[:a!\xC6\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\r\xDBV[\x92a!\xDBa!\xD2a\x01\xF2V[\x92\x83\x92\x83a!\x1AV[\x03\x90\xA2V[a!\xE8a#\x87V[a!UV[a!\xF5a\x1D\xEAV[a\"\x0Ea\"\x08a\"\x03a&hV[a\x07mV[\x91a\x07mV[\x03a\"\x15WV[a\"7a\" a&hV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\x0C\x13V[\x03\x90\xFD[\x90V[\x90a\"Sa\"Na\"Z\x92a\x0F\x8AV[a\";V[\x82Ta\x18\xB6V[\x90UV[a\"g_a\x1D\xDDV[a\"q\x82_a\">V[\x90a\"\xA5a\"\x9F\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x0F\x8AV[\x91a\x0F\x8AV[\x91a\"\xAEa\x01\xF2V[\x80a\"\xB8\x81a\x02\xBAV[\x03\x90\xA3V[\x90a\"\xC9`\xFF\x91a\x18\xB1V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\"\xE8a\"\xE3a\"\xEF\x92a\x18jV[a\x18vV[\x82Ta\"\xBDV[\x90UV[\x90a\"\xFD\x90a\x10\xEBV[_R` R`@_ \x90V[a#\x13\x90Qa\x03\x0FV[\x90V[\x90a#s```\x03a#y\x94a#9_\x82\x01a#3_\x88\x01a\x1C\x08V[\x90a \xFAV[a#R`\x01\x82\x01a#L` \x88\x01a\x1C\x08V[\x90a \xFAV[a#k`\x02\x82\x01a#e`@\x88\x01a\x1C\x08V[\x90a \xFAV[\x01\x92\x01a#\tV[\x90a\"\xD3V[V[\x90a#\x85\x91a#\x16V[V[a#\x9Aa#\x94`\x04a\x0EGV[\x15a\x03\x0FV[a#\xA1W[V[a#\xAD`\x01`\x04a\"\xD3V[a#\xC0a#\xB9_a\x10\xEBV[`\x03a \xFAV[a$!Ba$\x10_a$\x07a#\xFE_a#\xF9a#\xF0_\x95a#\xEBa#\xE2a\x10\xDBV[\x99_\x8B\x01a\x11\x07V[a\x10\xEBV[` \x88\x01a\x11\x07V[a\x10\xEBV[`@\x85\x01a\x11\x07V[``\x83\x01a\x11\x15V[a$\x1C`\x05_\x90a\"\xF3V[a#{V[_B\x90a$ca$Q\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x10\xEBV[\x92a$Za\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xA2a#\x9FV[\x90V[a$w\x90a\x02OV[_\x19\x81\x14a$\x85W`\x01\x01\x90V[a\x16\xCAV[a$\xA7a$\xA2`\x05a$\x9C`\x03a\x0E&V[\x90a\r\xF7V[a$kV[Ba$\xD5a$\xCFa$\xCAa$\xBC_\x86\x01a\x0E&V[a$\xC4a\t\xA7V[\x90a\x17\"V[a\x02OV[\x91a\x02OV[\x10\x15a$\xDFW[PV[a%\x07a$\xFEa$\xF0_\x84\x01a\x0E&V[a$\xF8a\t\xA7V[\x90a\x17\"V[`\x01\x83\x01a \xFAV[a%\x15`\x01`\x03\x83\x01a\"\xD3V[a%\x1F`\x03a\x0E&V[a%La%.`\x02\x84\x01a\x0E&V[\x92a%F_a%?`\x01\x84\x01a\x0E&V[\x92\x01a\x0E&V[\x90a\x16\xDEV[a%v\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\r\xDBV[\x92a%\x8Ba%\x82a\x01\xF2V[\x92\x83\x92\x83a!\x1AV[\x03\x90\xA2a%\xAAa%\xA3a%\x9E`\x03a\x0E&V[a$nV[`\x03a \xFAV[a&\x14Ba%\xFA_a%\xF1a%\xE8_a%\xE3a%\xDA_\x95a%\xD5a%\xCCa\x10\xDBV[\x99_\x8B\x01a\x11\x07V[a\x10\xEBV[` \x88\x01a\x11\x07V[a\x10\xEBV[`@\x85\x01a\x11\x07V[``\x83\x01a\x11\x15V[a&\x0F`\x05a&\t`\x03a\x0E&V[\x90a\r\xF7V[a#{V[a&\x1E`\x03a\x0E&V[B\x90a&_a&M\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\r\xDBV[\x92a&Va\x01\xF2V[\x91\x82\x91\x82a\x06\xE4V[\x03\x90\xA2_a$\xDCV[a&pa\x1D\xBAV[P3\x90V",
    );
    /**Custom error with signature `AlreadyInitialized()` and selector `0x0dc149f0`.
```solidity
error AlreadyInitialized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyInitialized {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AlreadyInitialized> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyInitialized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyInitialized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyInitialized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyInitialized()";
            const SELECTOR: [u8; 4] = [13u8, 193u8, 73u8, 240u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidModuleAddress()` and selector `0x5cfe78fe`.
```solidity
error InvalidModuleAddress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidModuleAddress {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidModuleAddress> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidModuleAddress) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidModuleAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidModuleAddress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidModuleAddress()";
            const SELECTOR: [u8; 4] = [92u8, 254u8, 120u8, 254u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`.
```solidity
error OwnableInvalidOwner(address owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableInvalidOwner {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OwnableInvalidOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OwnableInvalidOwner) -> Self {
                (value.owner,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OwnableInvalidOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { owner: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableInvalidOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableInvalidOwner(address)";
            const SELECTOR: [u8; 4] = [30u8, 79u8, 189u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`.
```solidity
error OwnableUnauthorizedAccount(address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableUnauthorizedAccount {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OwnableUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: OwnableUnauthorizedAccount) -> Self {
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OwnableUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableUnauthorizedAccount(address)";
            const SELECTOR: [u8; 4] = [17u8, 140u8, 218u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `TransactionOrSenderNotAllowed()` and selector `0xdc741458`.
```solidity
error TransactionOrSenderNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TransactionOrSenderNotAllowed {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TransactionOrSenderNotAllowed>
        for UnderlyingRustTuple<'_> {
            fn from(value: TransactionOrSenderNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TransactionOrSenderNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TransactionOrSenderNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TransactionOrSenderNotAllowed()";
            const SELECTOR: [u8; 4] = [220u8, 116u8, 20u8, 88u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Event with signature `GasTracked(uint256,uint256,uint256)` and selector `0x2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee`.
```solidity
event GasTracked(uint256 indexed periodIndex, uint256 gasUsed, uint256 gasPrice);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct GasTracked {
        #[allow(missing_docs)]
        pub periodIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasUsed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasPrice: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for GasTracked {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "GasTracked(uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                45u8,
                156u8,
                71u8,
                173u8,
                85u8,
                59u8,
                99u8,
                187u8,
                186u8,
                209u8,
                129u8,
                157u8,
                79u8,
                217u8,
                125u8,
                160u8,
                136u8,
                80u8,
                92u8,
                150u8,
                165u8,
                129u8,
                130u8,
                105u8,
                27u8,
                138u8,
                187u8,
                95u8,
                43u8,
                204u8,
                41u8,
                238u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    periodIndex: topics.1,
                    gasUsed: data.0,
                    gasPrice: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasUsed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasPrice),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.periodIndex.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.periodIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for GasTracked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&GasTracked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &GasTracked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `NewPeriodStarted(uint256,uint256)` and selector `0x41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e`.
```solidity
event NewPeriodStarted(uint256 indexed periodIndex, uint256 startTimestamp);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewPeriodStarted {
        #[allow(missing_docs)]
        pub periodIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub startTimestamp: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NewPeriodStarted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "NewPeriodStarted(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                65u8,
                241u8,
                224u8,
                143u8,
                33u8,
                204u8,
                129u8,
                140u8,
                240u8,
                207u8,
                251u8,
                58u8,
                98u8,
                96u8,
                159u8,
                182u8,
                163u8,
                203u8,
                201u8,
                179u8,
                103u8,
                27u8,
                1u8,
                30u8,
                40u8,
                94u8,
                23u8,
                161u8,
                235u8,
                180u8,
                104u8,
                142u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    periodIndex: topics.1,
                    startTimestamp: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.periodIndex.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.periodIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewPeriodStarted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewPeriodStarted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewPeriodStarted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `PeriodFinalized(uint256,uint256,uint256)` and selector `0x48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf5`.
```solidity
event PeriodFinalized(uint256 indexed periodIndex, uint256 totalGasUsed, uint256 duration);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PeriodFinalized {
        #[allow(missing_docs)]
        pub periodIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalGasUsed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub duration: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PeriodFinalized {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "PeriodFinalized(uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                72u8,
                162u8,
                152u8,
                249u8,
                211u8,
                118u8,
                184u8,
                42u8,
                113u8,
                116u8,
                167u8,
                152u8,
                233u8,
                12u8,
                241u8,
                32u8,
                148u8,
                149u8,
                253u8,
                214u8,
                139u8,
                12u8,
                17u8,
                235u8,
                17u8,
                190u8,
                171u8,
                172u8,
                194u8,
                210u8,
                156u8,
                245u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    periodIndex: topics.1,
                    totalGasUsed: data.0,
                    duration: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalGasUsed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.duration),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.periodIndex.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.periodIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PeriodFinalized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PeriodFinalized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PeriodFinalized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RequirementModuleUpdated(address)` and selector `0x253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b9`.
```solidity
event RequirementModuleUpdated(address indexed newModule);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RequirementModuleUpdated {
        #[allow(missing_docs)]
        pub newModule: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RequirementModuleUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RequirementModuleUpdated(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                37u8,
                53u8,
                128u8,
                248u8,
                6u8,
                116u8,
                28u8,
                17u8,
                179u8,
                212u8,
                170u8,
                96u8,
                217u8,
                202u8,
                204u8,
                91u8,
                239u8,
                12u8,
                235u8,
                179u8,
                87u8,
                72u8,
                118u8,
                127u8,
                226u8,
                63u8,
                17u8,
                145u8,
                110u8,
                47u8,
                4u8,
                185u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { newModule: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.newModule.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newModule,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RequirementModuleUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RequirementModuleUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RequirementModuleUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TransactionProcessed(address,bytes)` and selector `0x83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f`.
```solidity
event TransactionProcessed(address indexed sender, bytes data);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransactionProcessed_0 {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TransactionProcessed_0 {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TransactionProcessed(address,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                131u8,
                54u8,
                59u8,
                120u8,
                189u8,
                251u8,
                178u8,
                62u8,
                42u8,
                97u8,
                219u8,
                122u8,
                204u8,
                195u8,
                192u8,
                31u8,
                218u8,
                41u8,
                197u8,
                197u8,
                236u8,
                129u8,
                136u8,
                128u8,
                3u8,
                203u8,
                150u8,
                41u8,
                18u8,
                97u8,
                138u8,
                127u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    sender: topics.1,
                    data: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.sender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TransactionProcessed_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransactionProcessed_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransactionProcessed_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TransactionProcessed(address,bytes,uint256,uint256)` and selector `0x4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f2`.
```solidity
event TransactionProcessed(address indexed sender, bytes data, uint256 originalPriority, uint256 timestamp);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransactionProcessed_1 {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub originalPriority: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub timestamp: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TransactionProcessed_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TransactionProcessed(address,bytes,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                75u8,
                90u8,
                168u8,
                208u8,
                130u8,
                230u8,
                145u8,
                203u8,
                153u8,
                114u8,
                167u8,
                149u8,
                143u8,
                164u8,
                21u8,
                63u8,
                102u8,
                63u8,
                33u8,
                95u8,
                230u8,
                151u8,
                163u8,
                224u8,
                139u8,
                210u8,
                114u8,
                158u8,
                215u8,
                143u8,
                2u8,
                242u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    sender: topics.1,
                    data: data.0,
                    originalPriority: data.1,
                    timestamp: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.originalPriority),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timestamp),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.sender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TransactionProcessed_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransactionProcessed_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransactionProcessed_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(uint256 _appchainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _appchainId: alloy::sol_types::private::primitives::aliases::U256,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._appchainId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _appchainId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._appchainId),
                )
            }
        }
    };
    /**Function with signature `PERIOD_DURATION()` and selector `0x6558954f`.
```solidity
function PERIOD_DURATION() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PERIOD_DURATIONCall {}
    ///Container type for the return parameters of the [`PERIOD_DURATION()`](PERIOD_DURATIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PERIOD_DURATIONReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PERIOD_DURATIONCall> for UnderlyingRustTuple<'_> {
                fn from(value: PERIOD_DURATIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PERIOD_DURATIONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PERIOD_DURATIONReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: PERIOD_DURATIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PERIOD_DURATIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PERIOD_DURATIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = PERIOD_DURATIONReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PERIOD_DURATION()";
            const SELECTOR: [u8; 4] = [101u8, 88u8, 149u8, 79u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `PRIORITY_DECAY_RATE()` and selector `0x3d44ae8b`.
```solidity
function PRIORITY_DECAY_RATE() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PRIORITY_DECAY_RATECall {}
    ///Container type for the return parameters of the [`PRIORITY_DECAY_RATE()`](PRIORITY_DECAY_RATECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PRIORITY_DECAY_RATEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PRIORITY_DECAY_RATECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: PRIORITY_DECAY_RATECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PRIORITY_DECAY_RATECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PRIORITY_DECAY_RATEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: PRIORITY_DECAY_RATEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PRIORITY_DECAY_RATEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PRIORITY_DECAY_RATECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = PRIORITY_DECAY_RATEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PRIORITY_DECAY_RATE()";
            const SELECTOR: [u8; 4] = [61u8, 68u8, 174u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `appchainId()` and selector `0xd8781342`.
```solidity
function appchainId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct appchainIdCall {}
    ///Container type for the return parameters of the [`appchainId()`](appchainIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct appchainIdReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<appchainIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: appchainIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for appchainIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<appchainIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: appchainIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for appchainIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for appchainIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = appchainIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "appchainId()";
            const SELECTOR: [u8; 4] = [216u8, 120u8, 19u8, 66u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `calculateEffectivePriority(uint256,uint256,uint256)` and selector `0x83d3c115`.
```solidity
function calculateEffectivePriority(uint256 originalPriority, uint256 submittedTimestamp, uint256 currentTimestamp) external pure returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateEffectivePriorityCall {
        #[allow(missing_docs)]
        pub originalPriority: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub submittedTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub currentTimestamp: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateEffectivePriority(uint256,uint256,uint256)`](calculateEffectivePriorityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateEffectivePriorityReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateEffectivePriorityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateEffectivePriorityCall) -> Self {
                    (
                        value.originalPriority,
                        value.submittedTimestamp,
                        value.currentTimestamp,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateEffectivePriorityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        originalPriority: tuple.0,
                        submittedTimestamp: tuple.1,
                        currentTimestamp: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateEffectivePriorityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateEffectivePriorityReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateEffectivePriorityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateEffectivePriorityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateEffectivePriorityReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateEffectivePriority(uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [131u8, 211u8, 193u8, 21u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.originalPriority),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.submittedTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentTimestamp),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `currentPeriodIndex()` and selector `0x61543801`.
```solidity
function currentPeriodIndex() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentPeriodIndexCall {}
    ///Container type for the return parameters of the [`currentPeriodIndex()`](currentPeriodIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentPeriodIndexReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<currentPeriodIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentPeriodIndexCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentPeriodIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<currentPeriodIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentPeriodIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentPeriodIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentPeriodIndexCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentPeriodIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentPeriodIndex()";
            const SELECTOR: [u8; 4] = [97u8, 84u8, 56u8, 1u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `gasPriceInSynd()` and selector `0x703cfcbb`.
```solidity
function gasPriceInSynd() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasPriceInSyndCall {}
    ///Container type for the return parameters of the [`gasPriceInSynd()`](gasPriceInSyndCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasPriceInSyndReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<gasPriceInSyndCall> for UnderlyingRustTuple<'_> {
                fn from(value: gasPriceInSyndCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gasPriceInSyndCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<gasPriceInSyndReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: gasPriceInSyndReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for gasPriceInSyndReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for gasPriceInSyndCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = gasPriceInSyndReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "gasPriceInSynd()";
            const SELECTOR: [u8; 4] = [112u8, 60u8, 252u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `gasTrackingInitialized()` and selector `0x3b6ab2a9`.
```solidity
function gasTrackingInitialized() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasTrackingInitializedCall {}
    ///Container type for the return parameters of the [`gasTrackingInitialized()`](gasTrackingInitializedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasTrackingInitializedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<gasTrackingInitializedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: gasTrackingInitializedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for gasTrackingInitializedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<gasTrackingInitializedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: gasTrackingInitializedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for gasTrackingInitializedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for gasTrackingInitializedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = gasTrackingInitializedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "gasTrackingInitialized()";
            const SELECTOR: [u8; 4] = [59u8, 106u8, 178u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getCurrentPeriod()` and selector `0x086146d2`.
```solidity
function getCurrentPeriod() external view returns (GasCounter.GasPeriod memory period);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentPeriodCall {}
    ///Container type for the return parameters of the [`getCurrentPeriod()`](getCurrentPeriodCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentPeriodReturn {
        #[allow(missing_docs)]
        pub period: <GasCounter::GasPeriod as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentPeriodCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentPeriodCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentPeriodCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (GasCounter::GasPeriod,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GasCounter::GasPeriod as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentPeriodReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentPeriodReturn) -> Self {
                    (value.period,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentPeriodReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { period: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentPeriodCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentPeriodReturn;
            type ReturnTuple<'a> = (GasCounter::GasPeriod,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentPeriod()";
            const SELECTOR: [u8; 4] = [8u8, 97u8, 70u8, 210u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getCurrentPeriodGasUsed()` and selector `0xc660d3f3`.
```solidity
function getCurrentPeriodGasUsed() external view returns (uint256 totalGas);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentPeriodGasUsedCall {}
    ///Container type for the return parameters of the [`getCurrentPeriodGasUsed()`](getCurrentPeriodGasUsedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentPeriodGasUsedReturn {
        #[allow(missing_docs)]
        pub totalGas: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentPeriodGasUsedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentPeriodGasUsedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentPeriodGasUsedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentPeriodGasUsedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentPeriodGasUsedReturn) -> Self {
                    (value.totalGas,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentPeriodGasUsedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { totalGas: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentPeriodGasUsedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentPeriodGasUsedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentPeriodGasUsed()";
            const SELECTOR: [u8; 4] = [198u8, 96u8, 211u8, 243u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getCurrentPeriodTimeRemaining()` and selector `0x82f44ade`.
```solidity
function getCurrentPeriodTimeRemaining() external view returns (uint256 timeRemaining);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentPeriodTimeRemainingCall {}
    ///Container type for the return parameters of the [`getCurrentPeriodTimeRemaining()`](getCurrentPeriodTimeRemainingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentPeriodTimeRemainingReturn {
        #[allow(missing_docs)]
        pub timeRemaining: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentPeriodTimeRemainingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentPeriodTimeRemainingCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentPeriodTimeRemainingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentPeriodTimeRemainingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentPeriodTimeRemainingReturn) -> Self {
                    (value.timeRemaining,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentPeriodTimeRemainingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { timeRemaining: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentPeriodTimeRemainingCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentPeriodTimeRemainingReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentPeriodTimeRemaining()";
            const SELECTOR: [u8; 4] = [130u8, 244u8, 74u8, 222u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getPeriod(uint256)` and selector `0x4b2c0706`.
```solidity
function getPeriod(uint256 periodIndex) external view returns (GasCounter.GasPeriod memory period);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPeriodCall {
        #[allow(missing_docs)]
        pub periodIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getPeriod(uint256)`](getPeriodCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPeriodReturn {
        #[allow(missing_docs)]
        pub period: <GasCounter::GasPeriod as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPeriodCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPeriodCall) -> Self {
                    (value.periodIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPeriodCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { periodIndex: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (GasCounter::GasPeriod,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GasCounter::GasPeriod as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPeriodReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPeriodReturn) -> Self {
                    (value.period,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPeriodReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { period: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPeriodCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPeriodReturn;
            type ReturnTuple<'a> = (GasCounter::GasPeriod,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPeriod(uint256)";
            const SELECTOR: [u8; 4] = [75u8, 44u8, 7u8, 6u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.periodIndex),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getTotalGasFees()` and selector `0x8d5a239b`.
```solidity
function getTotalGasFees() external view returns (uint256 totalCost);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalGasFeesCall {}
    ///Container type for the return parameters of the [`getTotalGasFees()`](getTotalGasFeesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalGasFeesReturn {
        #[allow(missing_docs)]
        pub totalCost: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalGasFeesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalGasFeesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalGasFeesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalGasFeesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalGasFeesReturn) -> Self {
                    (value.totalCost,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalGasFeesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { totalCost: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalGasFeesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalGasFeesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalGasFees()";
            const SELECTOR: [u8; 4] = [141u8, 90u8, 35u8, 155u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getTotalPeriods()` and selector `0xaff74c6d`.
```solidity
function getTotalPeriods() external view returns (uint256 totalPeriods);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalPeriodsCall {}
    ///Container type for the return parameters of the [`getTotalPeriods()`](getTotalPeriodsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalPeriodsReturn {
        #[allow(missing_docs)]
        pub totalPeriods: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalPeriodsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalPeriodsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalPeriodsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalPeriodsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalPeriodsReturn) -> Self {
                    (value.totalPeriods,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalPeriodsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { totalPeriods: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalPeriodsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalPeriodsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalPeriods()";
            const SELECTOR: [u8; 4] = [175u8, 247u8, 76u8, 109u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address,address)` and selector `0x485cc955`.
```solidity
function initialize(address admin, address _permissionRequirementModule) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub admin: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _permissionRequirementModule: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.admin, value._permissionRequirementModule)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        admin: tuple.0,
                        _permissionRequirementModule: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address)";
            const SELECTOR: [u8; 4] = [72u8, 92u8, 201u8, 85u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.admin,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._permissionRequirementModule,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `isAllowed(address,address,bytes)` and selector `0x7a3979dc`.
```solidity
function isAllowed(address proposer, address originator, bytes memory data) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isAllowedCall {
        #[allow(missing_docs)]
        pub proposer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub originator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`isAllowed(address,address,bytes)`](isAllowedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isAllowedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isAllowedCall> for UnderlyingRustTuple<'_> {
                fn from(value: isAllowedCall) -> Self {
                    (value.proposer, value.originator, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isAllowedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        proposer: tuple.0,
                        originator: tuple.1,
                        data: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isAllowedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isAllowedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isAllowedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isAllowedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isAllowedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isAllowed(address,address,bytes)";
            const SELECTOR: [u8; 4] = [122u8, 57u8, 121u8, 220u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.proposer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.originator,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `isGasTrackingInitialized()` and selector `0x18d5aafe`.
```solidity
function isGasTrackingInitialized() external view returns (bool initialized);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isGasTrackingInitializedCall {}
    ///Container type for the return parameters of the [`isGasTrackingInitialized()`](isGasTrackingInitializedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isGasTrackingInitializedReturn {
        #[allow(missing_docs)]
        pub initialized: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isGasTrackingInitializedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isGasTrackingInitializedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isGasTrackingInitializedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isGasTrackingInitializedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isGasTrackingInitializedReturn) -> Self {
                    (value.initialized,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isGasTrackingInitializedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { initialized: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isGasTrackingInitializedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isGasTrackingInitializedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isGasTrackingInitialized()";
            const SELECTOR: [u8; 4] = [24u8, 213u8, 170u8, 254u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `periods(uint256)` and selector `0xea4a1104`.
```solidity
function periods(uint256) external view returns (uint256 startTimestamp, uint256 endTimestamp, uint256 totalGasUsed, bool finalized);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct periodsCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`periods(uint256)`](periodsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct periodsReturn {
        #[allow(missing_docs)]
        pub startTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub endTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalGasUsed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub finalized: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<periodsCall> for UnderlyingRustTuple<'_> {
                fn from(value: periodsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for periodsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                bool,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<periodsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: periodsReturn) -> Self {
                    (
                        value.startTimestamp,
                        value.endTimestamp,
                        value.totalGasUsed,
                        value.finalized,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for periodsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        startTimestamp: tuple.0,
                        endTimestamp: tuple.1,
                        totalGasUsed: tuple.2,
                        finalized: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for periodsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = periodsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "periods(uint256)";
            const SELECTOR: [u8; 4] = [234u8, 74u8, 17u8, 4u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `permissionRequirementModule()` and selector `0x5b3cd6e2`.
```solidity
function permissionRequirementModule() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permissionRequirementModuleCall {}
    ///Container type for the return parameters of the [`permissionRequirementModule()`](permissionRequirementModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permissionRequirementModuleReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<permissionRequirementModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: permissionRequirementModuleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for permissionRequirementModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<permissionRequirementModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: permissionRequirementModuleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for permissionRequirementModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for permissionRequirementModuleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = permissionRequirementModuleReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "permissionRequirementModule()";
            const SELECTOR: [u8; 4] = [91u8, 60u8, 214u8, 226u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `prependZeroByte(bytes)` and selector `0x366cbab7`.
```solidity
function prependZeroByte(bytes memory _data) external pure returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct prependZeroByteCall {
        #[allow(missing_docs)]
        pub _data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`prependZeroByte(bytes)`](prependZeroByteCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct prependZeroByteReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<prependZeroByteCall> for UnderlyingRustTuple<'_> {
                fn from(value: prependZeroByteCall) -> Self {
                    (value._data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for prependZeroByteCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _data: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<prependZeroByteReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: prependZeroByteReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for prependZeroByteReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for prependZeroByteCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = prependZeroByteReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "prependZeroByte(bytes)";
            const SELECTOR: [u8; 4] = [54u8, 108u8, 186u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._data,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `processTransaction(bytes,uint256)` and selector `0x1c0b6367`.
```solidity
function processTransaction(bytes memory data, uint256 priority) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransaction_0Call {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub priority: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`processTransaction(bytes,uint256)`](processTransaction_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransaction_0Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processTransaction_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransaction_0Call) -> Self {
                    (value.data, value.priority)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransaction_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        data: tuple.0,
                        priority: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processTransaction_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransaction_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransaction_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processTransaction_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processTransaction_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "processTransaction(bytes,uint256)";
            const SELECTOR: [u8; 4] = [28u8, 11u8, 99u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.priority),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `processTransaction(bytes)` and selector `0x46e2cc09`.
```solidity
function processTransaction(bytes memory data) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransaction_1Call {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`processTransaction(bytes)`](processTransaction_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransaction_1Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processTransaction_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransaction_1Call) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransaction_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processTransaction_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransaction_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransaction_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processTransaction_1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processTransaction_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "processTransaction(bytes)";
            const SELECTOR: [u8; 4] = [70u8, 226u8, 204u8, 9u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `processTransactionUncompressed(bytes,uint256)` and selector `0x050ec138`.
```solidity
function processTransactionUncompressed(bytes memory data, uint256 priority) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionUncompressed_0Call {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub priority: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`processTransactionUncompressed(bytes,uint256)`](processTransactionUncompressed_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionUncompressed_0Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processTransactionUncompressed_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionUncompressed_0Call) -> Self {
                    (value.data, value.priority)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionUncompressed_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        data: tuple.0,
                        priority: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processTransactionUncompressed_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionUncompressed_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionUncompressed_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processTransactionUncompressed_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processTransactionUncompressed_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "processTransactionUncompressed(bytes,uint256)";
            const SELECTOR: [u8; 4] = [5u8, 14u8, 193u8, 56u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.priority),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `processTransactionUncompressed(bytes)` and selector `0x804e5123`.
```solidity
function processTransactionUncompressed(bytes memory data) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionUncompressed_1Call {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`processTransactionUncompressed(bytes)`](processTransactionUncompressed_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionUncompressed_1Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processTransactionUncompressed_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionUncompressed_1Call) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionUncompressed_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processTransactionUncompressed_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionUncompressed_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionUncompressed_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processTransactionUncompressed_1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processTransactionUncompressed_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "processTransactionUncompressed(bytes)";
            const SELECTOR: [u8; 4] = [128u8, 78u8, 81u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `processTransactionsBulk(bytes[],uint256[])` and selector `0x11992f8c`.
```solidity
function processTransactionsBulk(bytes[] memory data, uint256[] memory priorities) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionsBulk_0Call {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub priorities: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`processTransactionsBulk(bytes[],uint256[])`](processTransactionsBulk_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionsBulk_0Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processTransactionsBulk_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionsBulk_0Call) -> Self {
                    (value.data, value.priorities)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionsBulk_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        data: tuple.0,
                        priorities: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processTransactionsBulk_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionsBulk_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionsBulk_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processTransactionsBulk_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processTransactionsBulk_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "processTransactionsBulk(bytes[],uint256[])";
            const SELECTOR: [u8; 4] = [17u8, 153u8, 47u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.data),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.priorities),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `processTransactionsBulk(bytes[])` and selector `0xcdafb978`.
```solidity
function processTransactionsBulk(bytes[] memory data) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionsBulk_1Call {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
    }
    ///Container type for the return parameters of the [`processTransactionsBulk(bytes[])`](processTransactionsBulk_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionsBulk_1Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processTransactionsBulk_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionsBulk_1Call) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionsBulk_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processTransactionsBulk_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionsBulk_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionsBulk_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processTransactionsBulk_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processTransactionsBulk_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "processTransactionsBulk(bytes[])";
            const SELECTOR: [u8; 4] = [205u8, 175u8, 185u8, 120u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.data),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateRequirementModule(address)` and selector `0xd4f0eb4d`.
```solidity
function updateRequirementModule(address _newModule) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateRequirementModuleCall {
        #[allow(missing_docs)]
        pub _newModule: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`updateRequirementModule(address)`](updateRequirementModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateRequirementModuleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateRequirementModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateRequirementModuleCall) -> Self {
                    (value._newModule,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateRequirementModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _newModule: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateRequirementModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateRequirementModuleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateRequirementModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateRequirementModuleCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateRequirementModuleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateRequirementModule(address)";
            const SELECTOR: [u8; 4] = [212u8, 240u8, 235u8, 77u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._newModule,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`SyndicateSequencingChainWithDecayingPriority`](self) function calls.
    pub enum SyndicateSequencingChainWithDecayingPriorityCalls {
        #[allow(missing_docs)]
        PERIOD_DURATION(PERIOD_DURATIONCall),
        #[allow(missing_docs)]
        PRIORITY_DECAY_RATE(PRIORITY_DECAY_RATECall),
        #[allow(missing_docs)]
        appchainId(appchainIdCall),
        #[allow(missing_docs)]
        calculateEffectivePriority(calculateEffectivePriorityCall),
        #[allow(missing_docs)]
        currentPeriodIndex(currentPeriodIndexCall),
        #[allow(missing_docs)]
        gasPriceInSynd(gasPriceInSyndCall),
        #[allow(missing_docs)]
        gasTrackingInitialized(gasTrackingInitializedCall),
        #[allow(missing_docs)]
        getCurrentPeriod(getCurrentPeriodCall),
        #[allow(missing_docs)]
        getCurrentPeriodGasUsed(getCurrentPeriodGasUsedCall),
        #[allow(missing_docs)]
        getCurrentPeriodTimeRemaining(getCurrentPeriodTimeRemainingCall),
        #[allow(missing_docs)]
        getPeriod(getPeriodCall),
        #[allow(missing_docs)]
        getTotalGasFees(getTotalGasFeesCall),
        #[allow(missing_docs)]
        getTotalPeriods(getTotalPeriodsCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isAllowed(isAllowedCall),
        #[allow(missing_docs)]
        isGasTrackingInitialized(isGasTrackingInitializedCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        periods(periodsCall),
        #[allow(missing_docs)]
        permissionRequirementModule(permissionRequirementModuleCall),
        #[allow(missing_docs)]
        prependZeroByte(prependZeroByteCall),
        #[allow(missing_docs)]
        processTransaction_0(processTransaction_0Call),
        #[allow(missing_docs)]
        processTransaction_1(processTransaction_1Call),
        #[allow(missing_docs)]
        processTransactionUncompressed_0(processTransactionUncompressed_0Call),
        #[allow(missing_docs)]
        processTransactionUncompressed_1(processTransactionUncompressed_1Call),
        #[allow(missing_docs)]
        processTransactionsBulk_0(processTransactionsBulk_0Call),
        #[allow(missing_docs)]
        processTransactionsBulk_1(processTransactionsBulk_1Call),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        updateRequirementModule(updateRequirementModuleCall),
    }
    #[automatically_derived]
    impl SyndicateSequencingChainWithDecayingPriorityCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [5u8, 14u8, 193u8, 56u8],
            [8u8, 97u8, 70u8, 210u8],
            [17u8, 153u8, 47u8, 140u8],
            [24u8, 213u8, 170u8, 254u8],
            [28u8, 11u8, 99u8, 103u8],
            [54u8, 108u8, 186u8, 183u8],
            [59u8, 106u8, 178u8, 169u8],
            [61u8, 68u8, 174u8, 139u8],
            [70u8, 226u8, 204u8, 9u8],
            [72u8, 92u8, 201u8, 85u8],
            [75u8, 44u8, 7u8, 6u8],
            [91u8, 60u8, 214u8, 226u8],
            [97u8, 84u8, 56u8, 1u8],
            [101u8, 88u8, 149u8, 79u8],
            [112u8, 60u8, 252u8, 187u8],
            [113u8, 80u8, 24u8, 166u8],
            [122u8, 57u8, 121u8, 220u8],
            [128u8, 78u8, 81u8, 35u8],
            [130u8, 244u8, 74u8, 222u8],
            [131u8, 211u8, 193u8, 21u8],
            [141u8, 90u8, 35u8, 155u8],
            [141u8, 165u8, 203u8, 91u8],
            [175u8, 247u8, 76u8, 109u8],
            [198u8, 96u8, 211u8, 243u8],
            [205u8, 175u8, 185u8, 120u8],
            [212u8, 240u8, 235u8, 77u8],
            [216u8, 120u8, 19u8, 66u8],
            [234u8, 74u8, 17u8, 4u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface
    for SyndicateSequencingChainWithDecayingPriorityCalls {
        const NAME: &'static str = "SyndicateSequencingChainWithDecayingPriorityCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::PERIOD_DURATION(_) => {
                    <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PRIORITY_DECAY_RATE(_) => {
                    <PRIORITY_DECAY_RATECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::appchainId(_) => {
                    <appchainIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateEffectivePriority(_) => {
                    <calculateEffectivePriorityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentPeriodIndex(_) => {
                    <currentPeriodIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::gasPriceInSynd(_) => {
                    <gasPriceInSyndCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::gasTrackingInitialized(_) => {
                    <gasTrackingInitializedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentPeriod(_) => {
                    <getCurrentPeriodCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentPeriodGasUsed(_) => {
                    <getCurrentPeriodGasUsedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentPeriodTimeRemaining(_) => {
                    <getCurrentPeriodTimeRemainingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPeriod(_) => {
                    <getPeriodCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalGasFees(_) => {
                    <getTotalGasFeesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalPeriods(_) => {
                    <getTotalPeriodsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isAllowed(_) => {
                    <isAllowedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isGasTrackingInitialized(_) => {
                    <isGasTrackingInitializedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::periods(_) => <periodsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::permissionRequirementModule(_) => {
                    <permissionRequirementModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::prependZeroByte(_) => {
                    <prependZeroByteCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::processTransaction_0(_) => {
                    <processTransaction_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::processTransaction_1(_) => {
                    <processTransaction_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::processTransactionUncompressed_0(_) => {
                    <processTransactionUncompressed_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::processTransactionUncompressed_1(_) => {
                    <processTransactionUncompressed_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::processTransactionsBulk_0(_) => {
                    <processTransactionsBulk_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::processTransactionsBulk_1(_) => {
                    <processTransactionsBulk_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateRequirementModule(_) => {
                    <updateRequirementModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<
                SyndicateSequencingChainWithDecayingPriorityCalls,
            >] = &[
                {
                    fn processTransactionUncompressed_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransactionUncompressed_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::processTransactionUncompressed_0,
                            )
                    }
                    processTransactionUncompressed_0
                },
                {
                    fn getCurrentPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getCurrentPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::getCurrentPeriod,
                            )
                    }
                    getCurrentPeriod
                },
                {
                    fn processTransactionsBulk_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransactionsBulk_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::processTransactionsBulk_0,
                            )
                    }
                    processTransactionsBulk_0
                },
                {
                    fn isGasTrackingInitialized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <isGasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::isGasTrackingInitialized,
                            )
                    }
                    isGasTrackingInitialized
                },
                {
                    fn processTransaction_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransaction_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::processTransaction_0,
                            )
                    }
                    processTransaction_0
                },
                {
                    fn prependZeroByte(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <prependZeroByteCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::prependZeroByte,
                            )
                    }
                    prependZeroByte
                },
                {
                    fn gasTrackingInitialized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <gasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::gasTrackingInitialized,
                            )
                    }
                    gasTrackingInitialized
                },
                {
                    fn PRIORITY_DECAY_RATE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <PRIORITY_DECAY_RATECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::PRIORITY_DECAY_RATE,
                            )
                    }
                    PRIORITY_DECAY_RATE
                },
                {
                    fn processTransaction_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransaction_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::processTransaction_1,
                            )
                    }
                    processTransaction_1
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::initialize,
                            )
                    }
                    initialize
                },
                {
                    fn getPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::getPeriod,
                            )
                    }
                    getPeriod
                },
                {
                    fn permissionRequirementModule(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <permissionRequirementModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::permissionRequirementModule,
                            )
                    }
                    permissionRequirementModule
                },
                {
                    fn currentPeriodIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <currentPeriodIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::currentPeriodIndex,
                            )
                    }
                    currentPeriodIndex
                },
                {
                    fn PERIOD_DURATION(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::PERIOD_DURATION,
                            )
                    }
                    PERIOD_DURATION
                },
                {
                    fn gasPriceInSynd(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <gasPriceInSyndCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::gasPriceInSynd,
                            )
                    }
                    gasPriceInSynd
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::renounceOwnership,
                            )
                    }
                    renounceOwnership
                },
                {
                    fn isAllowed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <isAllowedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::isAllowed,
                            )
                    }
                    isAllowed
                },
                {
                    fn processTransactionUncompressed_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransactionUncompressed_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::processTransactionUncompressed_1,
                            )
                    }
                    processTransactionUncompressed_1
                },
                {
                    fn getCurrentPeriodTimeRemaining(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getCurrentPeriodTimeRemainingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::getCurrentPeriodTimeRemaining,
                            )
                    }
                    getCurrentPeriodTimeRemaining
                },
                {
                    fn calculateEffectivePriority(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <calculateEffectivePriorityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::calculateEffectivePriority,
                            )
                    }
                    calculateEffectivePriority
                },
                {
                    fn getTotalGasFees(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getTotalGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::getTotalGasFees,
                            )
                    }
                    getTotalGasFees
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::owner,
                            )
                    }
                    owner
                },
                {
                    fn getTotalPeriods(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getTotalPeriodsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::getTotalPeriods,
                            )
                    }
                    getTotalPeriods
                },
                {
                    fn getCurrentPeriodGasUsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getCurrentPeriodGasUsedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::getCurrentPeriodGasUsed,
                            )
                    }
                    getCurrentPeriodGasUsed
                },
                {
                    fn processTransactionsBulk_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransactionsBulk_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::processTransactionsBulk_1,
                            )
                    }
                    processTransactionsBulk_1
                },
                {
                    fn updateRequirementModule(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <updateRequirementModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::updateRequirementModule,
                            )
                    }
                    updateRequirementModule
                },
                {
                    fn appchainId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <appchainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::appchainId,
                            )
                    }
                    appchainId
                },
                {
                    fn periods(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <periodsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::periods,
                            )
                    }
                    periods
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::transferOwnership,
                            )
                    }
                    transferOwnership
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::PERIOD_DURATION(inner) => {
                    <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PRIORITY_DECAY_RATE(inner) => {
                    <PRIORITY_DECAY_RATECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::appchainId(inner) => {
                    <appchainIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::calculateEffectivePriority(inner) => {
                    <calculateEffectivePriorityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentPeriodIndex(inner) => {
                    <currentPeriodIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::gasPriceInSynd(inner) => {
                    <gasPriceInSyndCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::gasTrackingInitialized(inner) => {
                    <gasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentPeriod(inner) => {
                    <getCurrentPeriodCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentPeriodGasUsed(inner) => {
                    <getCurrentPeriodGasUsedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentPeriodTimeRemaining(inner) => {
                    <getCurrentPeriodTimeRemainingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPeriod(inner) => {
                    <getPeriodCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getTotalGasFees(inner) => {
                    <getTotalGasFeesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalPeriods(inner) => {
                    <getTotalPeriodsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isAllowed(inner) => {
                    <isAllowedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isGasTrackingInitialized(inner) => {
                    <isGasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::periods(inner) => {
                    <periodsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::permissionRequirementModule(inner) => {
                    <permissionRequirementModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::prependZeroByte(inner) => {
                    <prependZeroByteCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::processTransaction_0(inner) => {
                    <processTransaction_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::processTransaction_1(inner) => {
                    <processTransaction_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::processTransactionUncompressed_0(inner) => {
                    <processTransactionUncompressed_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::processTransactionUncompressed_1(inner) => {
                    <processTransactionUncompressed_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::processTransactionsBulk_0(inner) => {
                    <processTransactionsBulk_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::processTransactionsBulk_1(inner) => {
                    <processTransactionsBulk_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateRequirementModule(inner) => {
                    <updateRequirementModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::PERIOD_DURATION(inner) => {
                    <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PRIORITY_DECAY_RATE(inner) => {
                    <PRIORITY_DECAY_RATECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::appchainId(inner) => {
                    <appchainIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateEffectivePriority(inner) => {
                    <calculateEffectivePriorityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currentPeriodIndex(inner) => {
                    <currentPeriodIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::gasPriceInSynd(inner) => {
                    <gasPriceInSyndCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::gasTrackingInitialized(inner) => {
                    <gasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentPeriod(inner) => {
                    <getCurrentPeriodCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentPeriodGasUsed(inner) => {
                    <getCurrentPeriodGasUsedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentPeriodTimeRemaining(inner) => {
                    <getCurrentPeriodTimeRemainingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPeriod(inner) => {
                    <getPeriodCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalGasFees(inner) => {
                    <getTotalGasFeesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalPeriods(inner) => {
                    <getTotalPeriodsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isAllowed(inner) => {
                    <isAllowedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isGasTrackingInitialized(inner) => {
                    <isGasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::periods(inner) => {
                    <periodsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::permissionRequirementModule(inner) => {
                    <permissionRequirementModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::prependZeroByte(inner) => {
                    <prependZeroByteCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::processTransaction_0(inner) => {
                    <processTransaction_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::processTransaction_1(inner) => {
                    <processTransaction_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::processTransactionUncompressed_0(inner) => {
                    <processTransactionUncompressed_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::processTransactionUncompressed_1(inner) => {
                    <processTransactionUncompressed_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::processTransactionsBulk_0(inner) => {
                    <processTransactionsBulk_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::processTransactionsBulk_1(inner) => {
                    <processTransactionsBulk_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateRequirementModule(inner) => {
                    <updateRequirementModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SyndicateSequencingChainWithDecayingPriority`](self) custom errors.
    pub enum SyndicateSequencingChainWithDecayingPriorityErrors {
        #[allow(missing_docs)]
        AlreadyInitialized(AlreadyInitialized),
        #[allow(missing_docs)]
        InvalidModuleAddress(InvalidModuleAddress),
        #[allow(missing_docs)]
        OwnableInvalidOwner(OwnableInvalidOwner),
        #[allow(missing_docs)]
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        #[allow(missing_docs)]
        TransactionOrSenderNotAllowed(TransactionOrSenderNotAllowed),
    }
    #[automatically_derived]
    impl SyndicateSequencingChainWithDecayingPriorityErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [13u8, 193u8, 73u8, 240u8],
            [17u8, 140u8, 218u8, 167u8],
            [30u8, 79u8, 189u8, 247u8],
            [92u8, 254u8, 120u8, 254u8],
            [220u8, 116u8, 20u8, 88u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface
    for SyndicateSequencingChainWithDecayingPriorityErrors {
        const NAME: &'static str = "SyndicateSequencingChainWithDecayingPriorityErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 5usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyInitialized(_) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidModuleAddress(_) => {
                    <InvalidModuleAddress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableInvalidOwner(_) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableUnauthorizedAccount(_) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TransactionOrSenderNotAllowed(_) => {
                    <TransactionOrSenderNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<
                SyndicateSequencingChainWithDecayingPriorityErrors,
            >] = &[
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityErrors::AlreadyInitialized,
                            )
                    }
                    AlreadyInitialized
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityErrors::OwnableUnauthorizedAccount,
                            )
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityErrors::OwnableInvalidOwner,
                            )
                    }
                    OwnableInvalidOwner
                },
                {
                    fn InvalidModuleAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <InvalidModuleAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityErrors::InvalidModuleAddress,
                            )
                    }
                    InvalidModuleAddress
                },
                {
                    fn TransactionOrSenderNotAllowed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <TransactionOrSenderNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityErrors::TransactionOrSenderNotAllowed,
                            )
                    }
                    TransactionOrSenderNotAllowed
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidModuleAddress(inner) => {
                    <InvalidModuleAddress as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TransactionOrSenderNotAllowed(inner) => {
                    <TransactionOrSenderNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidModuleAddress(inner) => {
                    <InvalidModuleAddress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TransactionOrSenderNotAllowed(inner) => {
                    <TransactionOrSenderNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SyndicateSequencingChainWithDecayingPriority`](self) events.
    pub enum SyndicateSequencingChainWithDecayingPriorityEvents {
        #[allow(missing_docs)]
        GasTracked(GasTracked),
        #[allow(missing_docs)]
        NewPeriodStarted(NewPeriodStarted),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        PeriodFinalized(PeriodFinalized),
        #[allow(missing_docs)]
        RequirementModuleUpdated(RequirementModuleUpdated),
        #[allow(missing_docs)]
        TransactionProcessed_0(TransactionProcessed_0),
        #[allow(missing_docs)]
        TransactionProcessed_1(TransactionProcessed_1),
    }
    #[automatically_derived]
    impl SyndicateSequencingChainWithDecayingPriorityEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                37u8,
                53u8,
                128u8,
                248u8,
                6u8,
                116u8,
                28u8,
                17u8,
                179u8,
                212u8,
                170u8,
                96u8,
                217u8,
                202u8,
                204u8,
                91u8,
                239u8,
                12u8,
                235u8,
                179u8,
                87u8,
                72u8,
                118u8,
                127u8,
                226u8,
                63u8,
                17u8,
                145u8,
                110u8,
                47u8,
                4u8,
                185u8,
            ],
            [
                45u8,
                156u8,
                71u8,
                173u8,
                85u8,
                59u8,
                99u8,
                187u8,
                186u8,
                209u8,
                129u8,
                157u8,
                79u8,
                217u8,
                125u8,
                160u8,
                136u8,
                80u8,
                92u8,
                150u8,
                165u8,
                129u8,
                130u8,
                105u8,
                27u8,
                138u8,
                187u8,
                95u8,
                43u8,
                204u8,
                41u8,
                238u8,
            ],
            [
                65u8,
                241u8,
                224u8,
                143u8,
                33u8,
                204u8,
                129u8,
                140u8,
                240u8,
                207u8,
                251u8,
                58u8,
                98u8,
                96u8,
                159u8,
                182u8,
                163u8,
                203u8,
                201u8,
                179u8,
                103u8,
                27u8,
                1u8,
                30u8,
                40u8,
                94u8,
                23u8,
                161u8,
                235u8,
                180u8,
                104u8,
                142u8,
            ],
            [
                72u8,
                162u8,
                152u8,
                249u8,
                211u8,
                118u8,
                184u8,
                42u8,
                113u8,
                116u8,
                167u8,
                152u8,
                233u8,
                12u8,
                241u8,
                32u8,
                148u8,
                149u8,
                253u8,
                214u8,
                139u8,
                12u8,
                17u8,
                235u8,
                17u8,
                190u8,
                171u8,
                172u8,
                194u8,
                210u8,
                156u8,
                245u8,
            ],
            [
                75u8,
                90u8,
                168u8,
                208u8,
                130u8,
                230u8,
                145u8,
                203u8,
                153u8,
                114u8,
                167u8,
                149u8,
                143u8,
                164u8,
                21u8,
                63u8,
                102u8,
                63u8,
                33u8,
                95u8,
                230u8,
                151u8,
                163u8,
                224u8,
                139u8,
                210u8,
                114u8,
                158u8,
                215u8,
                143u8,
                2u8,
                242u8,
            ],
            [
                131u8,
                54u8,
                59u8,
                120u8,
                189u8,
                251u8,
                178u8,
                62u8,
                42u8,
                97u8,
                219u8,
                122u8,
                204u8,
                195u8,
                192u8,
                31u8,
                218u8,
                41u8,
                197u8,
                197u8,
                236u8,
                129u8,
                136u8,
                128u8,
                3u8,
                203u8,
                150u8,
                41u8,
                18u8,
                97u8,
                138u8,
                127u8,
            ],
            [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface
    for SyndicateSequencingChainWithDecayingPriorityEvents {
        const NAME: &'static str = "SyndicateSequencingChainWithDecayingPriorityEvents";
        const COUNT: usize = 7usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<GasTracked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <GasTracked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::GasTracked)
                }
                Some(<NewPeriodStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewPeriodStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::NewPeriodStarted)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<PeriodFinalized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PeriodFinalized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::PeriodFinalized)
                }
                Some(
                    <RequirementModuleUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RequirementModuleUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RequirementModuleUpdated)
                }
                Some(
                    <TransactionProcessed_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionProcessed_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TransactionProcessed_0)
                }
                Some(
                    <TransactionProcessed_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionProcessed_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TransactionProcessed_1)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData
    for SyndicateSequencingChainWithDecayingPriorityEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::GasTracked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewPeriodStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PeriodFinalized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RequirementModuleUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransactionProcessed_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransactionProcessed_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::GasTracked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewPeriodStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PeriodFinalized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RequirementModuleUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransactionProcessed_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransactionProcessed_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SyndicateSequencingChainWithDecayingPriority`](self) contract instance.

See the [wrapper's documentation](`SyndicateSequencingChainWithDecayingPriorityInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SyndicateSequencingChainWithDecayingPriorityInstance<T, P, N> {
        SyndicateSequencingChainWithDecayingPriorityInstance::<
            T,
            P,
            N,
        >::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _appchainId: alloy::sol_types::private::primitives::aliases::U256,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<
            SyndicateSequencingChainWithDecayingPriorityInstance<T, P, N>,
        >,
    > {
        SyndicateSequencingChainWithDecayingPriorityInstance::<
            T,
            P,
            N,
        >::deploy(provider, _appchainId)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _appchainId: alloy::sol_types::private::primitives::aliases::U256,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        SyndicateSequencingChainWithDecayingPriorityInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _appchainId)
    }
    /**A [`SyndicateSequencingChainWithDecayingPriority`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SyndicateSequencingChainWithDecayingPriority`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SyndicateSequencingChainWithDecayingPriorityInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug
    for SyndicateSequencingChainWithDecayingPriorityInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SyndicateSequencingChainWithDecayingPriorityInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SyndicateSequencingChainWithDecayingPriorityInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`SyndicateSequencingChainWithDecayingPriority`](self) contract instance.

See the [wrapper's documentation](`SyndicateSequencingChainWithDecayingPriorityInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _appchainId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::Result<
            SyndicateSequencingChainWithDecayingPriorityInstance<T, P, N>,
        > {
            let call_builder = Self::deploy_builder(provider, _appchainId);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _appchainId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { _appchainId },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<
        T,
        P: ::core::clone::Clone,
        N,
    > SyndicateSequencingChainWithDecayingPriorityInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> SyndicateSequencingChainWithDecayingPriorityInstance<T, P, N> {
            SyndicateSequencingChainWithDecayingPriorityInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SyndicateSequencingChainWithDecayingPriorityInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`PERIOD_DURATION`] function.
        pub fn PERIOD_DURATION(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, PERIOD_DURATIONCall, N> {
            self.call_builder(&PERIOD_DURATIONCall {})
        }
        ///Creates a new call builder for the [`PRIORITY_DECAY_RATE`] function.
        pub fn PRIORITY_DECAY_RATE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, PRIORITY_DECAY_RATECall, N> {
            self.call_builder(&PRIORITY_DECAY_RATECall {})
        }
        ///Creates a new call builder for the [`appchainId`] function.
        pub fn appchainId(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, appchainIdCall, N> {
            self.call_builder(&appchainIdCall {})
        }
        ///Creates a new call builder for the [`calculateEffectivePriority`] function.
        pub fn calculateEffectivePriority(
            &self,
            originalPriority: alloy::sol_types::private::primitives::aliases::U256,
            submittedTimestamp: alloy::sol_types::private::primitives::aliases::U256,
            currentTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateEffectivePriorityCall, N> {
            self.call_builder(
                &calculateEffectivePriorityCall {
                    originalPriority,
                    submittedTimestamp,
                    currentTimestamp,
                },
            )
        }
        ///Creates a new call builder for the [`currentPeriodIndex`] function.
        pub fn currentPeriodIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, currentPeriodIndexCall, N> {
            self.call_builder(&currentPeriodIndexCall {})
        }
        ///Creates a new call builder for the [`gasPriceInSynd`] function.
        pub fn gasPriceInSynd(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, gasPriceInSyndCall, N> {
            self.call_builder(&gasPriceInSyndCall {})
        }
        ///Creates a new call builder for the [`gasTrackingInitialized`] function.
        pub fn gasTrackingInitialized(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, gasTrackingInitializedCall, N> {
            self.call_builder(&gasTrackingInitializedCall {})
        }
        ///Creates a new call builder for the [`getCurrentPeriod`] function.
        pub fn getCurrentPeriod(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentPeriodCall, N> {
            self.call_builder(&getCurrentPeriodCall {})
        }
        ///Creates a new call builder for the [`getCurrentPeriodGasUsed`] function.
        pub fn getCurrentPeriodGasUsed(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentPeriodGasUsedCall, N> {
            self.call_builder(&getCurrentPeriodGasUsedCall {})
        }
        ///Creates a new call builder for the [`getCurrentPeriodTimeRemaining`] function.
        pub fn getCurrentPeriodTimeRemaining(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getCurrentPeriodTimeRemainingCall,
            N,
        > {
            self.call_builder(
                &getCurrentPeriodTimeRemainingCall {
                },
            )
        }
        ///Creates a new call builder for the [`getPeriod`] function.
        pub fn getPeriod(
            &self,
            periodIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPeriodCall, N> {
            self.call_builder(&getPeriodCall { periodIndex })
        }
        ///Creates a new call builder for the [`getTotalGasFees`] function.
        pub fn getTotalGasFees(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTotalGasFeesCall, N> {
            self.call_builder(&getTotalGasFeesCall {})
        }
        ///Creates a new call builder for the [`getTotalPeriods`] function.
        pub fn getTotalPeriods(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTotalPeriodsCall, N> {
            self.call_builder(&getTotalPeriodsCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            admin: alloy::sol_types::private::Address,
            _permissionRequirementModule: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    admin,
                    _permissionRequirementModule,
                },
            )
        }
        ///Creates a new call builder for the [`isAllowed`] function.
        pub fn isAllowed(
            &self,
            proposer: alloy::sol_types::private::Address,
            originator: alloy::sol_types::private::Address,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, isAllowedCall, N> {
            self.call_builder(
                &isAllowedCall {
                    proposer,
                    originator,
                    data,
                },
            )
        }
        ///Creates a new call builder for the [`isGasTrackingInitialized`] function.
        pub fn isGasTrackingInitialized(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, isGasTrackingInitializedCall, N> {
            self.call_builder(&isGasTrackingInitializedCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`periods`] function.
        pub fn periods(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, periodsCall, N> {
            self.call_builder(&periodsCall { _0 })
        }
        ///Creates a new call builder for the [`permissionRequirementModule`] function.
        pub fn permissionRequirementModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, permissionRequirementModuleCall, N> {
            self.call_builder(&permissionRequirementModuleCall {})
        }
        ///Creates a new call builder for the [`prependZeroByte`] function.
        pub fn prependZeroByte(
            &self,
            _data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, prependZeroByteCall, N> {
            self.call_builder(&prependZeroByteCall { _data })
        }
        ///Creates a new call builder for the [`processTransaction_0`] function.
        pub fn processTransaction_0(
            &self,
            data: alloy::sol_types::private::Bytes,
            priority: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, processTransaction_0Call, N> {
            self.call_builder(
                &processTransaction_0Call {
                    data,
                    priority,
                },
            )
        }
        ///Creates a new call builder for the [`processTransaction_1`] function.
        pub fn processTransaction_1(
            &self,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, processTransaction_1Call, N> {
            self.call_builder(&processTransaction_1Call { data })
        }
        ///Creates a new call builder for the [`processTransactionUncompressed_0`] function.
        pub fn processTransactionUncompressed_0(
            &self,
            data: alloy::sol_types::private::Bytes,
            priority: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            processTransactionUncompressed_0Call,
            N,
        > {
            self.call_builder(
                &processTransactionUncompressed_0Call {
                    data,
                    priority,
                },
            )
        }
        ///Creates a new call builder for the [`processTransactionUncompressed_1`] function.
        pub fn processTransactionUncompressed_1(
            &self,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            processTransactionUncompressed_1Call,
            N,
        > {
            self.call_builder(
                &processTransactionUncompressed_1Call {
                    data,
                },
            )
        }
        ///Creates a new call builder for the [`processTransactionsBulk_0`] function.
        pub fn processTransactionsBulk_0(
            &self,
            data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            priorities: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, processTransactionsBulk_0Call, N> {
            self.call_builder(
                &processTransactionsBulk_0Call {
                    data,
                    priorities,
                },
            )
        }
        ///Creates a new call builder for the [`processTransactionsBulk_1`] function.
        pub fn processTransactionsBulk_1(
            &self,
            data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        ) -> alloy_contract::SolCallBuilder<T, &P, processTransactionsBulk_1Call, N> {
            self.call_builder(
                &processTransactionsBulk_1Call {
                    data,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`updateRequirementModule`] function.
        pub fn updateRequirementModule(
            &self,
            _newModule: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateRequirementModuleCall, N> {
            self.call_builder(
                &updateRequirementModuleCall {
                    _newModule,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SyndicateSequencingChainWithDecayingPriorityInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`GasTracked`] event.
        pub fn GasTracked_filter(&self) -> alloy_contract::Event<T, &P, GasTracked, N> {
            self.event_filter::<GasTracked>()
        }
        ///Creates a new event filter for the [`NewPeriodStarted`] event.
        pub fn NewPeriodStarted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, NewPeriodStarted, N> {
            self.event_filter::<NewPeriodStarted>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`PeriodFinalized`] event.
        pub fn PeriodFinalized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, PeriodFinalized, N> {
            self.event_filter::<PeriodFinalized>()
        }
        ///Creates a new event filter for the [`RequirementModuleUpdated`] event.
        pub fn RequirementModuleUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RequirementModuleUpdated, N> {
            self.event_filter::<RequirementModuleUpdated>()
        }
        ///Creates a new event filter for the [`TransactionProcessed_0`] event.
        pub fn TransactionProcessed_0_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TransactionProcessed_0, N> {
            self.event_filter::<TransactionProcessed_0>()
        }
        ///Creates a new event filter for the [`TransactionProcessed_1`] event.
        pub fn TransactionProcessed_1_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TransactionProcessed_1, N> {
            self.event_filter::<TransactionProcessed_1>()
        }
    }
}
