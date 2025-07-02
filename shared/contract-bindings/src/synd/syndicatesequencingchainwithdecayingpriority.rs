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
    function TRACKING_OVERHEAD() external view returns (uint256);
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
    "name": "TRACKING_OVERHEAD",
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
    ///0x60a060405234610038576100196100146100e9565b61010a565b61002161003d565b6126ca61053f823960805181610d9401526126ca90f35b610043565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b9061006f90610047565b810190811060018060401b0382111761008757604052565b610051565b9061009f61009861003d565b9283610065565b565b5f80fd5b90565b6100b1816100a5565b036100b857565b5f80fd5b905051906100c9826100a8565b565b906020828203126100e4576100e1915f016100bc565b90565b6100a1565b610107612dc8803803806100fc8161008c565b9283398101906100cb565b90565b610113906101c2565b565b90565b90565b61012f61012a61013492610115565b610118565b6100a5565b90565b60209181520190565b5f7f41707020636861696e2049442063616e6e6f7420626520300000000000000000910152565b6101746018602092610137565b61017d81610140565b0190565b6101969060208101905f818303910152610167565b90565b156101a057565b6101a861003d565b62461bcd60e51b8152806101be60048201610181565b0390fd5b6101ca610249565b6101e7816101e06101da5f61011b565b916100a5565b1415610199565b608052565b5f1b90565b906101fd5f19916101ec565b9181191691161790565b90565b61021e61021961022392610207565b610118565b6100a5565b90565b90565b9061023e6102396102459261020a565b610226565b82546101f1565b9055565b610251610353565b610260633b9aca006002610229565b565b60a01b90565b9061027760ff60a01b91610262565b9181191691161790565b151590565b61028f90610281565b90565b90565b906102aa6102a56102b192610286565b610292565b8254610268565b9055565b5f0190565b6102c261003d565b3d5f823e3d90fd5b60018060a01b031690565b6102e96102e46102ee926102ca565b610118565b6102ca565b90565b6102fa906102d5565b90565b610306906102f1565b90565b9061031a60018060a01b03916101ec565b9181191691161790565b61032d906102f1565b90565b90565b9061034861034361034f92610324565b610330565b8254610309565b9055565b61035c336103c0565b6103675f6001610295565b61036f61003d565b6101bf810181811060018060401b038211176103bb5761039782916101bf612c0984396102b5565b03905ff080156103b6576103ad6103b4916102fd565b6001610333565b565b6102ba565b610051565b6103c990610421565b565b6103df6103da6103e492610115565b610118565b6102ca565b90565b6103f0906103cb565b90565b6103fc906102ca565b90565b610408906103f3565b9052565b919061041f905f602085019401906103ff565b565b8061043c6104366104315f6103e7565b6103f3565b916103f3565b1461044c5761044a906104df565b565b61046f6104585f6103e7565b5f918291631e4fbdf760e01b83526004830161040c565b0390fd5b5f1c90565b60018060a01b031690565b61048f61049491610473565b610478565b90565b6104a19054610483565b90565b6104ad906102d5565b90565b6104b9906104a4565b90565b90565b906104d46104cf6104db926104b0565b6104bc565b8254610309565b9055565b6104e85f610497565b6104f2825f6104bf565b906105266105207f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0936104b0565b916104b0565b9161052f61003d565b80610539816102b5565b0390a356fe60806040526004361015610013575b610fbb565b61001d5f356101fc565b8063050ec138146101f7578063086146d2146101f257806311992f8c146101ed57806318d5aafe146101e85780631c0b6367146101e3578063366cbab7146101de5780633b6ab2a9146101d95780633d44ae8b146101d457806346e2cc09146101cf578063485cc955146101ca5780634b2c0706146101c55780635b3cd6e2146101c057806361543801146101bb5780636558954f146101b6578063703cfcbb146101b1578063715018a6146101ac5780637a3979dc146101a7578063804e5123146101a257806382f44ade1461019d57806383d3c115146101985780638d5a239b146101935780638da5cb5b1461018e578063aff74c6d14610189578063c660d3f314610184578063cdafb9781461017f578063d4f0eb4d1461017a578063d878134214610175578063ea4a110414610170578063ede07bd61461016b5763f2fde38b0361000e57610f88565b610f53565b610ee2565b610db6565b610d5f565b610d0d565b610ca2565b610c6d565b610c38565b610be1565b610bab565b610b3c565b610b08565b610acf565b610a4a565b610a15565b6109d1565b610963565b6108f6565b61082b565b6107d9565b61073e565b610709565b610678565b610603565b61052e565b6104f9565b6104a0565b61038e565b6102cf565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f8301121561025a5781359167ffffffffffffffff831161025557602001926001830284011161025057565b61021c565b610218565b610214565b90565b61026b8161025f565b0361027257565b5f80fd5b9050359061028382610262565b565b916040838303126102c5575f83013567ffffffffffffffff81116102c0576102b2836102bd928601610220565b939094602001610276565b90565b610210565b61020c565b5f0190565b346102fe576102e86102e2366004610285565b9161109b565b6102f0610202565b806102fa816102ca565b0390f35b610208565b5f91031261030d57565b61020c565b61031b9061025f565b9052565b151590565b61032d9061031f565b9052565b90606080610377936103495f8201515f860190610312565b61035b60208201516020860190610312565b61036d60408201516040860190610312565b0151910190610324565b565b919061038c905f60808501940190610331565b565b346103be5761039e366004610303565b6103ba6103a9611215565b6103b1610202565b91829182610379565b0390f35b610208565b909182601f830112156103fd5781359167ffffffffffffffff83116103f85760200192602083028401116103f357565b61021c565b610218565b610214565b909182601f8301121561043c5781359167ffffffffffffffff831161043757602001926020830284011161043257565b61021c565b610218565b610214565b909160408284031261049b575f82013567ffffffffffffffff8111610496578361046c9184016103c3565b929093602082013567ffffffffffffffff81116104915761048d9201610402565b9091565b610210565b610210565b61020c565b346104d2576104bc6104b3366004610441565b9291909161141e565b6104c4610202565b806104ce816102ca565b0390f35b610208565b6104e09061031f565b9052565b91906104f7905f602085019401906104d7565b565b3461052957610509366004610303565b610525610514611522565b61051c610202565b918291826104e4565b0390f35b610208565b3461055d57610547610541366004610285565b9161162f565b61054f610202565b80610559816102ca565b0390f35b610208565b90602082820312610593575f82013567ffffffffffffffff811161058e5761058a9201610220565b9091565b610210565b61020c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6105d96105e26020936105e7936105d081610598565b9384809361059c565b958691016105a5565b6105b0565b0190565b6106009160208201915f8184039101526105ba565b90565b346106345761063061061f610619366004610562565b906116bf565b610627610202565b918291826105eb565b0390f35b610208565b1c90565b60ff1690565b6106539060086106589302610639565b61063d565b90565b906106669154610643565b90565b61067560045f9061065b565b90565b346106a857610688366004610303565b6106a4610693610669565b61069b610202565b918291826104e4565b0390f35b610208565b90565b90565b6106c76106c26106cc926106ad565b6106b0565b61025f565b90565b6106d9600a6106b3565b90565b6106e46106cf565b90565b6106f09061025f565b9052565b9190610707905f602085019401906106e7565b565b3461073957610719366004610303565b6107356107246106dc565b61072c610202565b918291826106f4565b0390f35b610208565b3461076d57610757610751366004610562565b90611837565b61075f610202565b80610769816102ca565b0390f35b610208565b60018060a01b031690565b61078690610772565b90565b6107928161077d565b0361079957565b5f80fd5b905035906107aa82610789565b565b91906040838203126107d457806107c86107d1925f860161079d565b9360200161079d565b90565b61020c565b34610808576107f26107ec3660046107ac565b906119e8565b6107fa610202565b80610804816102ca565b0390f35b610208565b9060208282031261082657610823915f01610276565b90565b61020c565b3461085b5761085761084661084136600461080d565b6119f4565b61084e610202565b91829182610379565b0390f35b610208565b60018060a01b031690565b61087b9060086108809302610639565b610860565b90565b9061088e915461086b565b90565b61089d60015f90610883565b90565b6108b46108af6108b992610772565b6106b0565b610772565b90565b6108c5906108a0565b90565b6108d1906108bc565b90565b6108dd906108c8565b9052565b91906108f4905f602085019401906108d4565b565b3461092657610906366004610303565b610922610911610891565b610919610202565b918291826108e1565b0390f35b610208565b90565b61093e9060086109439302610639565b61092b565b90565b90610951915461092e565b90565b61096060035f90610946565b90565b3461099357610973366004610303565b61098f61097e610954565b610986610202565b918291826106f4565b0390f35b610208565b90565b6109af6109aa6109b492610998565b6106b0565b61025f565b90565b6109c362278d0061099b565b90565b6109ce6109b7565b90565b34610a01576109e1366004610303565b6109fd6109ec6109c6565b6109f4610202565b918291826106f4565b0390f35b610208565b610a1260025f90610946565b90565b34610a4557610a25366004610303565b610a41610a30610a06565b610a38610202565b918291826106f4565b0390f35b610208565b34610a7857610a5a366004610303565b610a62611a38565b610a6a610202565b80610a74816102ca565b0390f35b610208565b91606083830312610aca57610a94825f850161079d565b92610aa2836020830161079d565b92604082013567ffffffffffffffff8111610ac557610ac19201610220565b9091565b610210565b61020c565b34610b0357610aff610aee610ae5366004610a7d565b92919091611af0565b610af6610202565b918291826104e4565b0390f35b610208565b34610b3757610b21610b1b366004610562565b90611c50565b610b29610202565b80610b33816102ca565b0390f35b610208565b34610b6c57610b4c366004610303565b610b68610b57611c6d565b610b5f610202565b918291826106f4565b0390f35b610208565b9091606082840312610ba657610ba3610b8c845f8501610276565b93610b9a8160208601610276565b93604001610276565b90565b61020c565b34610bdc57610bd8610bc7610bc1366004610b71565b91611d3a565b610bcf610202565b918291826106f4565b0390f35b610208565b34610c1157610bf1366004610303565b610c0d610bfc611db0565b610c04610202565b918291826106f4565b0390f35b610208565b610c1f9061077d565b9052565b9190610c36905f60208501940190610c16565b565b34610c6857610c48366004610303565b610c64610c53611e42565b610c5b610202565b91829182610c23565b0390f35b610208565b34610c9d57610c7d366004610303565b610c99610c88611e76565b610c90610202565b918291826106f4565b0390f35b610208565b34610cd257610cb2366004610303565b610cce610cbd611ec2565b610cc5610202565b918291826106f4565b0390f35b610208565b90602082820312610d08575f82013567ffffffffffffffff8111610d0357610cff92016103c3565b9091565b610210565b61020c565b34610d3c57610d26610d20366004610cd7565b9061200b565b610d2e610202565b80610d38816102ca565b0390f35b610208565b90602082820312610d5a57610d57915f0161079d565b90565b61020c565b34610d8d57610d77610d72366004610d41565b6120bb565b610d7f610202565b80610d89816102ca565b0390f35b610208565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610de657610dc6366004610303565b610de2610dd1610d92565b610dd9610202565b918291826106f4565b0390f35b610208565b610dff610dfa610e049261025f565b6106b0565b61025f565b90565b90610e1190610deb565b5f5260205260405f2090565b5f1c90565b610e2e610e3391610e1d565b61092b565b90565b610e409054610e22565b90565b610e4f610e5491610e1d565b61063d565b90565b610e619054610e43565b90565b610e6f906005610e07565b90610e7b5f8301610e36565b91610e8860018201610e36565b91610ea16003610e9a60028501610e36565b9301610e57565b90565b610ed9610ee094610ecf606094989795610ec5608086019a5f8701906106e7565b60208501906106e7565b60408301906106e7565b01906104d7565b565b34610f1657610f12610efd610ef836600461080d565b610e64565b90610f09949294610202565b94859485610ea4565b0390f35b610208565b90565b610f32610f2d610f3792610f1b565b6106b0565b61025f565b90565b610f45611388610f1e565b90565b610f50610f3a565b90565b34610f8357610f63366004610303565b610f7f610f6e610f48565b610f76610202565b918291826106f4565b0390f35b610208565b34610fb657610fa0610f9b366004610d41565b61212b565b610fa8610202565b80610fb2816102ca565b0390f35b610208565b5f80fd5b9190610fdc610fd633329086859192909192611af0565b1561031f565b610feb57610fe992611048565b565b5f631b8e828b60e31b815280611003600482016102ca565b0390fd5b611010906108bc565b90565b60409061103f6110346110469597969460608401908482035f8601526105ba565b9660208301906106e7565b01906106e7565b565b906110549033926116bf565b9142926110966110847f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f294611007565b9461108d610202565b93849384611013565b0390a2565b906110a69291610fbf565b565b634e487b7160e01b5f52604160045260245ffd5b906110c6906105b0565b810190811067ffffffffffffffff8211176110e057604052565b6110a8565b906110f86110f1610202565b92836110bc565b565b61110460806110e5565b90565b5f90565b5f90565b6111176110fa565b90602080808085611126611107565b815201611131611107565b81520161113c611107565b81520161114761110b565b81525050565b61115561110f565b90565b61116260806110e5565b90565b90565b61117c61117761118192611165565b6106b0565b61025f565b90565b9061118e9061025f565b9052565b9061119c9061031f565b9052565b906112076111fe60036111b16110fa565b946111c86111c05f8301610e36565b5f8801611184565b6111e06111d760018301610e36565b60208801611184565b6111f86111ef60028301610e36565b60408801611184565b01610e57565b60608401611192565b565b611212906111a0565b90565b61121d61114d565b5061123161122b6004610e57565b1561031f565b6112555761125261124d60056112476003610e36565b90610e07565b611209565b90565b5f6112aa5f6112a16112985f61129361128a5f9561128561127d611277611158565b9a611168565b5f8b01611184565b611168565b60208801611184565b611168565b60408501611184565b60608301611192565b90565b5090565b5090565b60209181520190565b60207f7665207468652073616d65206c656e6774680000000000000000000000000000917f4461746120616e64207072696f7269747920617272617973206d7573742068615f8201520152565b61131860326040926112b5565b611321816112be565b0190565b61133a9060208101905f81830391015261130b565b90565b1561134457565b61134c610202565b62461bcd60e51b81528061136260048201611325565b0390fd5b6001611372910161025f565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b9035906001602003813603038212156113d7570180359067ffffffffffffffff82116113d2576020019160018202360383136113cd57565b611391565b61138d565b611389565b908210156113f75760206113f39202810190611395565b9091565b611375565b919081101561140c576020020190565b611375565b3561141b81610262565b90565b909261142b8285906112ad565b936114528561144c6114466114418887906112b1565b61025f565b9161025f565b1461133d565b61145b5f611168565b5b8061146f6114698861025f565b9161025f565b10156115165761149d9061149333329061148b888786916113dc565b929091611af0565b6114a2575b611366565b61145c565b336114b86114b2878685916113dc565b906116bf565b906114cd6114c8898886916113fc565b611411565b429261150e6114fc7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f294611007565b94611505610202565b93849384611013565b0390a2611498565b505050505050565b5f90565b61152a61151e565b506115356004610e57565b90565b919061155561154f33329086859192909192611af0565b1561031f565b61156457611562926115e3565b565b5f631b8e828b60e31b81528061157c600482016102ca565b0390fd5b90825f939282370152565b91906115a58161159e816115aa9561059c565b8095611580565b6105b0565b0190565b6115da6115cf6040936115e19698979560608501918583035f87015261158b565b9660208301906106e7565b01906106e7565b565b9091339192909261162a426116187f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f295611007565b95611621610202565b948594856115ae565b0390a2565b9061163a9291611538565b565b606090565b60ff60f81b1690565b60f81b90565b61166461165f61166992611165565b61164a565b611641565b90565b90565b61167b61168091611641565b61166c565b9052565b905090565b909182611699816116a093611684565b8093611580565b0190565b806116b56001926116bc969461166f565b0191611689565b90565b6116fd906116cb61163c565b506116ee6116d85f611650565b91936116e2610202565b948593602085016116a4565b602082018103825203826110bc565b90565b9061171c61171633329085859192909192611af0565b1561031f565b61172b57611729916117a5565b565b5f631b8e828b60e31b815280611743600482016102ca565b0390fd5b634e487b7160e01b5f52601160045260245ffd5b61176a6117709193929361025f565b9261025f565b820391821161177b57565b611747565b61178f6117959193929361025f565b9261025f565b82018092116117a057565b611747565b6117c16117cf916117ba6117d4945a926117f0565b5a9061175b565b6117c9610f3a565b90611780565b612192565b565b90916117ed9260208301925f81850391015261158b565b90565b33909161181d7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611007565b92611832611829610202565b928392836117d6565b0390a2565b9061184191611700565b565b9061185591611850612242565b61195b565b565b60a01c90565b61186961186e91611857565b61063d565b90565b61187b905461185d565b90565b61189261188d61189792611165565b6106b0565b610772565b90565b6118a39061187e565b90565b60a01b90565b906118bb60ff60a01b916118a6565b9181191691161790565b6118ce9061031f565b90565b90565b906118e96118e46118f0926118c5565b6118d1565b82546118ac565b9055565b6118fd906108a0565b90565b611909906118f4565b90565b5f1b90565b9061192260018060a01b039161190c565b9181191691161790565b611935906118f4565b90565b90565b9061195061194b6119579261192c565b611938565b8254611911565b9055565b6119656001611871565b6119cd578161198461197e6119795f61189a565b61077d565b9161077d565b146119b1576119aa6119a36119af9361199e6001806118d4565b611900565b600161193b565b61212b565b565b5f632e7f3c7f60e11b8152806119c9600482016102ca565b0390fd5b5f62dc149f60e41b8152806119e4600482016102ca565b0390fd5b906119f291611843565b565b611a0b611a1091611a0361114d565b506005610e07565b611209565b90565b611a1b612242565b611a23611a25565b565b611a36611a315f61189a565b6122b3565b565b611a40611a13565b565b611a4e611a5391610e1d565b610860565b90565b611a609054611a42565b90565b60e01b90565b611a728161031f565b03611a7957565b5f80fd5b90505190611a8a82611a69565b565b90602082820312611aa557611aa2915f01611a7d565b90565b61020c565b611ad0611add9593949294611ac660608401965f850190610c16565b6020830190610c16565b604081850391015261158b565b90565b611ae8610202565b3d5f823e3d90fd5b92611b3360209394611b0061151e565b50611b3e611b16611b116001611a56565b6108c8565b93637a3979dc929597611b27610202565b98899788968796611a63565b865260048601611aaa565b03915afa908115611b82575f91611b54575b5090565b611b75915060203d8111611b7b575b611b6d81836110bc565b810190611a8c565b5f611b50565b503d611b63565b611ae0565b90611ba3611b9d33329085859192909192611af0565b1561031f565b611bb257611bb091611bce565b565b5f631b8e828b60e31b815280611bca600482016102ca565b0390fd5b611bea611bf891611be3611bfd945a92611bff565b5a9061175b565b611bf2610f3a565b90611780565b612192565b565b90611c0b9033926116bf565b90611c4b611c397f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611007565b92611c42610202565b918291826105eb565b0390a2565b90611c5a91611b87565b565b5f90565b611c6a905161025f565b90565b611c75611c5c565b50611c89611c836004610e57565b1561031f565b611cf957611cc5611cb75f611cb1611cac6005611ca66003610e36565b90610e07565b611209565b01611c60565b611cbf6109b7565b90611780565b42611cd8611cd28361025f565b9161025f565b1015611cec57611ce990429061175b565b90565b50611cf65f611168565b90565b611d025f611168565b90565b611d14611d1a9193929361025f565b9261025f565b91611d2683820261025f565b928184041490151715611d3557565b611747565b91611d43611c5c565b5080611d57611d518461025f565b9161025f565b1115611dab57611d7891611d6a9161175b565b611d726106cf565b90611d05565b80611d8b611d858461025f565b9161025f565b1015611d9d57611d9a9161175b565b90565b5050611da85f611168565b90565b505090565b611db8611c5c565b50611dcc611dc66004610e57565b1561031f565b611e0657611e03611df36002611ded6005611de76003610e36565b90610e07565b01610e36565b611dfd6002610e36565b90611d05565b90565b611e0f5f611168565b90565b5f90565b60018060a01b031690565b611e2d611e3291610e1d565b611e16565b90565b611e3f9054611e21565b90565b611e4a611e12565b50611e545f611e35565b90565b90565b611e6e611e69611e7392611e57565b6106b0565b61025f565b90565b611e7e611c5c565b50611e92611e8c6004610e57565b1561031f565b611eb657611eb3611ea36003610e36565b611ead6001611e5a565b90611780565b90565b611ebf5f611168565b90565b611eca611c5c565b50611ede611ed86004610e57565b1561031f565b611f0557611f026002611efc6005611ef66003610e36565b90610e07565b01610e36565b90565b611f0e5f611168565b90565b611f2d611f3b91611f26611f40945a92611f42565b5a9061175b565b611f35610f3a565b90611780565b612192565b565b611f4d8183906112ad565b91611f56611c5c565b50611f605f611168565b5b80611f74611f6e8661025f565b9161025f565b101561200557611fa290611f98333290611f90878786916113dc565b929091611af0565b611fa7575b611366565b611f61565b33611fbd611fb7868685916113dc565b906116bf565b90611ffd611feb7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611007565b92611ff4610202565b918291826105eb565b0390a2611f9d565b50505050565b9061201591611f11565b565b61202890612023612242565b61202a565b565b8061204561203f61203a5f61189a565b61077d565b9161077d565b1461209f5761205d61205682611900565b600161193b565b6120877f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b991611007565b90612090610202565b8061209a816102ca565b0390a2565b5f632e7f3c7f60e11b8152806120b7600482016102ca565b0390fd5b6120c490612017565b565b6120d7906120d2612242565b6120d9565b565b806120f46120ee6120e95f61189a565b61077d565b9161077d565b1461210457612102906122b3565b565b6121276121105f61189a565b5f918291631e4fbdf760e01b835260048301610c23565b0390fd5b612134906120c6565b565b906121425f199161190c565b9181191691161790565b90565b9061216461215f61216b92610deb565b61214c565b8254612136565b9055565b91602061219092949361218960408201965f8301906106e7565b01906106e7565b565b6121a561219f6004610e57565b1561031f565b612235575b6121b26124df565b6121e6816121e060026121d060056121ca6003610e36565b90610e07565b01916121db83610e36565b611780565b9061214f565b6121f06003610e36565b3a61221b7f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610deb565b92612230612227610202565b9283928361216f565b0390a2565b61223d6123dc565b6121aa565b61224a611e42565b61226361225d6122586126bd565b61077d565b9161077d565b0361226a57565b61228c6122756126bd565b5f91829163118cdaa760e01b835260048301610c23565b0390fd5b90565b906122a86122a36122af92611007565b612290565b8254611911565b9055565b6122bc5f611e35565b6122c6825f612293565b906122fa6122f47f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093611007565b91611007565b91612303610202565b8061230d816102ca565b0390a3565b9061231e60ff9161190c565b9181191691161790565b9061233d612338612344926118c5565b6118d1565b8254612312565b9055565b9061235290611168565b5f5260205260405f2090565b612368905161031f565b90565b906123c8606060036123ce9461238e5f82016123885f8801611c60565b9061214f565b6123a7600182016123a160208801611c60565b9061214f565b6123c0600282016123ba60408801611c60565b9061214f565b01920161235e565b90612328565b565b906123da9161236b565b565b6123ef6123e96004610e57565b1561031f565b6123f6575b565b61240260016004612328565b61241561240e5f611168565b600361214f565b612476426124655f61245c6124535f61244e6124455f95612440612437611158565b995f8b01611184565b611168565b60208801611184565b611168565b60408501611184565b60608301611192565b61247160055f90612348565b6123d0565b5f42906124b86124a67f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92611168565b926124af610202565b918291826106f4565b0390a26123f4565b90565b6124cc9061025f565b5f1981146124da5760010190565b611747565b6124fc6124f760056124f16003610e36565b90610e07565b6124c0565b4261252a61252461251f6125115f8601610e36565b6125196109b7565b90611780565b61025f565b9161025f565b1015612534575b50565b61255c6125536125455f8401610e36565b61254d6109b7565b90611780565b6001830161214f565b61256a600160038301612328565b6125746003610e36565b6125a161258360028401610e36565b9261259b5f61259460018401610e36565b9201610e36565b9061175b565b6125cb7f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610deb565b926125e06125d7610202565b9283928361216f565b0390a26125ff6125f86125f36003610e36565b6124c3565b600361214f565b6126694261264f5f61264661263d5f61263861262f5f9561262a612621611158565b995f8b01611184565b611168565b60208801611184565b611168565b60408501611184565b60608301611192565b612664600561265e6003610e36565b90610e07565b6123d0565b6126736003610e36565b42906126b46126a27f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610deb565b926126ab610202565b918291826106f4565b0390a25f612531565b6126c5611e12565b50339056608060405234601c57600e6020565b61019461002b823961019490f35b6026565b60405190565b5f80fdfe60806040526004361015610013575b61017b565b61001d5f3561002c565b637a3979dc0361000e57610142565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b60018060a01b031690565b61005890610044565b90565b6100648161004f565b0361006b57565b5f80fd5b9050359061007c8261005b565b565b5f80fd5b5f80fd5b5f80fd5b909182601f830112156100c45781359167ffffffffffffffff83116100bf5760200192600183028401116100ba57565b610086565b610082565b61007e565b91606083830312610116576100e0825f850161006f565b926100ee836020830161006f565b92604082013567ffffffffffffffff81116101115761010d920161008a565b9091565b610040565b61003c565b151590565b6101299061011b565b9052565b9190610140905f60208501940190610120565b565b34610176576101726101616101583660046100c9565b92919091610183565b610169610032565b9182918261012d565b0390f35b610038565b5f80fd5b5f90565b5050505061018f61017f565b505f9056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4a\08Wa\0\x19a\0\x14a\0\xE9V[a\x01\nV[a\0!a\0=V[a&\xCAa\x05?\x829`\x80Q\x81a\r\x94\x01Ra&\xCA\x90\xF3[a\0CV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0o\x90a\0GV[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x87W`@RV[a\0QV[\x90a\0\x9Fa\0\x98a\0=V[\x92\x83a\0eV[V[_\x80\xFD[\x90V[a\0\xB1\x81a\0\xA5V[\x03a\0\xB8WV[_\x80\xFD[\x90PQ\x90a\0\xC9\x82a\0\xA8V[V[\x90` \x82\x82\x03\x12a\0\xE4Wa\0\xE1\x91_\x01a\0\xBCV[\x90V[a\0\xA1V[a\x01\x07a-\xC8\x808\x03\x80a\0\xFC\x81a\0\x8CV[\x92\x839\x81\x01\x90a\0\xCBV[\x90V[a\x01\x13\x90a\x01\xC2V[V[\x90V[\x90V[a\x01/a\x01*a\x014\x92a\x01\x15V[a\x01\x18V[a\0\xA5V[\x90V[` \x91\x81R\x01\x90V[_\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01t`\x18` \x92a\x017V[a\x01}\x81a\x01@V[\x01\x90V[a\x01\x96\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x01gV[\x90V[\x15a\x01\xA0WV[a\x01\xA8a\0=V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x01\xBE`\x04\x82\x01a\x01\x81V[\x03\x90\xFD[a\x01\xCAa\x02IV[a\x01\xE7\x81a\x01\xE0a\x01\xDA_a\x01\x1BV[\x91a\0\xA5V[\x14\x15a\x01\x99V[`\x80RV[_\x1B\x90V[\x90a\x01\xFD_\x19\x91a\x01\xECV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[a\x02\x1Ea\x02\x19a\x02#\x92a\x02\x07V[a\x01\x18V[a\0\xA5V[\x90V[\x90V[\x90a\x02>a\x029a\x02E\x92a\x02\nV[a\x02&V[\x82Ta\x01\xF1V[\x90UV[a\x02Qa\x03SV[a\x02`c;\x9A\xCA\0`\x02a\x02)V[V[`\xA0\x1B\x90V[\x90a\x02w`\xFF`\xA0\x1B\x91a\x02bV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x15\x15\x90V[a\x02\x8F\x90a\x02\x81V[\x90V[\x90V[\x90a\x02\xAAa\x02\xA5a\x02\xB1\x92a\x02\x86V[a\x02\x92V[\x82Ta\x02hV[\x90UV[_\x01\x90V[a\x02\xC2a\0=V[=_\x82>=\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x02\xE9a\x02\xE4a\x02\xEE\x92a\x02\xCAV[a\x01\x18V[a\x02\xCAV[\x90V[a\x02\xFA\x90a\x02\xD5V[\x90V[a\x03\x06\x90a\x02\xF1V[\x90V[\x90a\x03\x1A`\x01\x80`\xA0\x1B\x03\x91a\x01\xECV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x03-\x90a\x02\xF1V[\x90V[\x90V[\x90a\x03Ha\x03Ca\x03O\x92a\x03$V[a\x030V[\x82Ta\x03\tV[\x90UV[a\x03\\3a\x03\xC0V[a\x03g_`\x01a\x02\x95V[a\x03oa\0=V[a\x01\xBF\x81\x01\x81\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\x03\xBBWa\x03\x97\x82\x91a\x01\xBFa,\t\x849a\x02\xB5V[\x03\x90_\xF0\x80\x15a\x03\xB6Wa\x03\xADa\x03\xB4\x91a\x02\xFDV[`\x01a\x033V[V[a\x02\xBAV[a\0QV[a\x03\xC9\x90a\x04!V[V[a\x03\xDFa\x03\xDAa\x03\xE4\x92a\x01\x15V[a\x01\x18V[a\x02\xCAV[\x90V[a\x03\xF0\x90a\x03\xCBV[\x90V[a\x03\xFC\x90a\x02\xCAV[\x90V[a\x04\x08\x90a\x03\xF3V[\x90RV[\x91\x90a\x04\x1F\x90_` \x85\x01\x94\x01\x90a\x03\xFFV[V[\x80a\x04<a\x046a\x041_a\x03\xE7V[a\x03\xF3V[\x91a\x03\xF3V[\x14a\x04LWa\x04J\x90a\x04\xDFV[V[a\x04oa\x04X_a\x03\xE7V[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x04\x0CV[\x03\x90\xFD[_\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\x8Fa\x04\x94\x91a\x04sV[a\x04xV[\x90V[a\x04\xA1\x90Ta\x04\x83V[\x90V[a\x04\xAD\x90a\x02\xD5V[\x90V[a\x04\xB9\x90a\x04\xA4V[\x90V[\x90V[\x90a\x04\xD4a\x04\xCFa\x04\xDB\x92a\x04\xB0V[a\x04\xBCV[\x82Ta\x03\tV[\x90UV[a\x04\xE8_a\x04\x97V[a\x04\xF2\x82_a\x04\xBFV[\x90a\x05&a\x05 \x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x04\xB0V[\x91a\x04\xB0V[\x91a\x05/a\0=V[\x80a\x059\x81a\x02\xB5V[\x03\x90\xA3V\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x0F\xBBV[a\0\x1D_5a\x01\xFCV[\x80c\x05\x0E\xC18\x14a\x01\xF7W\x80c\x08aF\xD2\x14a\x01\xF2W\x80c\x11\x99/\x8C\x14a\x01\xEDW\x80c\x18\xD5\xAA\xFE\x14a\x01\xE8W\x80c\x1C\x0Bcg\x14a\x01\xE3W\x80c6l\xBA\xB7\x14a\x01\xDEW\x80c;j\xB2\xA9\x14a\x01\xD9W\x80c=D\xAE\x8B\x14a\x01\xD4W\x80cF\xE2\xCC\t\x14a\x01\xCFW\x80cH\\\xC9U\x14a\x01\xCAW\x80cK,\x07\x06\x14a\x01\xC5W\x80c[<\xD6\xE2\x14a\x01\xC0W\x80caT8\x01\x14a\x01\xBBW\x80ceX\x95O\x14a\x01\xB6W\x80cp<\xFC\xBB\x14a\x01\xB1W\x80cqP\x18\xA6\x14a\x01\xACW\x80cz9y\xDC\x14a\x01\xA7W\x80c\x80NQ#\x14a\x01\xA2W\x80c\x82\xF4J\xDE\x14a\x01\x9DW\x80c\x83\xD3\xC1\x15\x14a\x01\x98W\x80c\x8DZ#\x9B\x14a\x01\x93W\x80c\x8D\xA5\xCB[\x14a\x01\x8EW\x80c\xAF\xF7Lm\x14a\x01\x89W\x80c\xC6`\xD3\xF3\x14a\x01\x84W\x80c\xCD\xAF\xB9x\x14a\x01\x7FW\x80c\xD4\xF0\xEBM\x14a\x01zW\x80c\xD8x\x13B\x14a\x01uW\x80c\xEAJ\x11\x04\x14a\x01pW\x80c\xED\xE0{\xD6\x14a\x01kWc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\x0F\x88V[a\x0FSV[a\x0E\xE2V[a\r\xB6V[a\r_V[a\r\rV[a\x0C\xA2V[a\x0CmV[a\x0C8V[a\x0B\xE1V[a\x0B\xABV[a\x0B<V[a\x0B\x08V[a\n\xCFV[a\nJV[a\n\x15V[a\t\xD1V[a\tcV[a\x08\xF6V[a\x08+V[a\x07\xD9V[a\x07>V[a\x07\tV[a\x06xV[a\x06\x03V[a\x05.V[a\x04\xF9V[a\x04\xA0V[a\x03\x8EV[a\x02\xCFV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x02ZW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x02UW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x02PWV[a\x02\x1CV[a\x02\x18V[a\x02\x14V[\x90V[a\x02k\x81a\x02_V[\x03a\x02rWV[_\x80\xFD[\x90P5\x90a\x02\x83\x82a\x02bV[V[\x91`@\x83\x83\x03\x12a\x02\xC5W_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02\xC0Wa\x02\xB2\x83a\x02\xBD\x92\x86\x01a\x02 V[\x93\x90\x94` \x01a\x02vV[\x90V[a\x02\x10V[a\x02\x0CV[_\x01\x90V[4a\x02\xFEWa\x02\xE8a\x02\xE26`\x04a\x02\x85V[\x91a\x10\x9BV[a\x02\xF0a\x02\x02V[\x80a\x02\xFA\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[_\x91\x03\x12a\x03\rWV[a\x02\x0CV[a\x03\x1B\x90a\x02_V[\x90RV[\x15\x15\x90V[a\x03-\x90a\x03\x1FV[\x90RV[\x90``\x80a\x03w\x93a\x03I_\x82\x01Q_\x86\x01\x90a\x03\x12V[a\x03[` \x82\x01Q` \x86\x01\x90a\x03\x12V[a\x03m`@\x82\x01Q`@\x86\x01\x90a\x03\x12V[\x01Q\x91\x01\x90a\x03$V[V[\x91\x90a\x03\x8C\x90_`\x80\x85\x01\x94\x01\x90a\x031V[V[4a\x03\xBEWa\x03\x9E6`\x04a\x03\x03V[a\x03\xBAa\x03\xA9a\x12\x15V[a\x03\xB1a\x02\x02V[\x91\x82\x91\x82a\x03yV[\x03\x90\xF3[a\x02\x08V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03\xFDW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\xF8W` \x01\x92` \x83\x02\x84\x01\x11a\x03\xF3WV[a\x02\x1CV[a\x02\x18V[a\x02\x14V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04<W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x047W` \x01\x92` \x83\x02\x84\x01\x11a\x042WV[a\x02\x1CV[a\x02\x18V[a\x02\x14V[\x90\x91`@\x82\x84\x03\x12a\x04\x9BW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x96W\x83a\x04l\x91\x84\x01a\x03\xC3V[\x92\x90\x93` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x91Wa\x04\x8D\x92\x01a\x04\x02V[\x90\x91V[a\x02\x10V[a\x02\x10V[a\x02\x0CV[4a\x04\xD2Wa\x04\xBCa\x04\xB36`\x04a\x04AV[\x92\x91\x90\x91a\x14\x1EV[a\x04\xC4a\x02\x02V[\x80a\x04\xCE\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[a\x04\xE0\x90a\x03\x1FV[\x90RV[\x91\x90a\x04\xF7\x90_` \x85\x01\x94\x01\x90a\x04\xD7V[V[4a\x05)Wa\x05\t6`\x04a\x03\x03V[a\x05%a\x05\x14a\x15\"V[a\x05\x1Ca\x02\x02V[\x91\x82\x91\x82a\x04\xE4V[\x03\x90\xF3[a\x02\x08V[4a\x05]Wa\x05Ga\x05A6`\x04a\x02\x85V[\x91a\x16/V[a\x05Oa\x02\x02V[\x80a\x05Y\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[\x90` \x82\x82\x03\x12a\x05\x93W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05\x8EWa\x05\x8A\x92\x01a\x02 V[\x90\x91V[a\x02\x10V[a\x02\x0CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x05\xD9a\x05\xE2` \x93a\x05\xE7\x93a\x05\xD0\x81a\x05\x98V[\x93\x84\x80\x93a\x05\x9CV[\x95\x86\x91\x01a\x05\xA5V[a\x05\xB0V[\x01\x90V[a\x06\0\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x05\xBAV[\x90V[4a\x064Wa\x060a\x06\x1Fa\x06\x196`\x04a\x05bV[\x90a\x16\xBFV[a\x06'a\x02\x02V[\x91\x82\x91\x82a\x05\xEBV[\x03\x90\xF3[a\x02\x08V[\x1C\x90V[`\xFF\x16\x90V[a\x06S\x90`\x08a\x06X\x93\x02a\x069V[a\x06=V[\x90V[\x90a\x06f\x91Ta\x06CV[\x90V[a\x06u`\x04_\x90a\x06[V[\x90V[4a\x06\xA8Wa\x06\x886`\x04a\x03\x03V[a\x06\xA4a\x06\x93a\x06iV[a\x06\x9Ba\x02\x02V[\x91\x82\x91\x82a\x04\xE4V[\x03\x90\xF3[a\x02\x08V[\x90V[\x90V[a\x06\xC7a\x06\xC2a\x06\xCC\x92a\x06\xADV[a\x06\xB0V[a\x02_V[\x90V[a\x06\xD9`\na\x06\xB3V[\x90V[a\x06\xE4a\x06\xCFV[\x90V[a\x06\xF0\x90a\x02_V[\x90RV[\x91\x90a\x07\x07\x90_` \x85\x01\x94\x01\x90a\x06\xE7V[V[4a\x079Wa\x07\x196`\x04a\x03\x03V[a\x075a\x07$a\x06\xDCV[a\x07,a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[4a\x07mWa\x07Wa\x07Q6`\x04a\x05bV[\x90a\x187V[a\x07_a\x02\x02V[\x80a\x07i\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x07\x86\x90a\x07rV[\x90V[a\x07\x92\x81a\x07}V[\x03a\x07\x99WV[_\x80\xFD[\x90P5\x90a\x07\xAA\x82a\x07\x89V[V[\x91\x90`@\x83\x82\x03\x12a\x07\xD4W\x80a\x07\xC8a\x07\xD1\x92_\x86\x01a\x07\x9DV[\x93` \x01a\x07\x9DV[\x90V[a\x02\x0CV[4a\x08\x08Wa\x07\xF2a\x07\xEC6`\x04a\x07\xACV[\x90a\x19\xE8V[a\x07\xFAa\x02\x02V[\x80a\x08\x04\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[\x90` \x82\x82\x03\x12a\x08&Wa\x08#\x91_\x01a\x02vV[\x90V[a\x02\x0CV[4a\x08[Wa\x08Wa\x08Fa\x08A6`\x04a\x08\rV[a\x19\xF4V[a\x08Na\x02\x02V[\x91\x82\x91\x82a\x03yV[\x03\x90\xF3[a\x02\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08{\x90`\x08a\x08\x80\x93\x02a\x069V[a\x08`V[\x90V[\x90a\x08\x8E\x91Ta\x08kV[\x90V[a\x08\x9D`\x01_\x90a\x08\x83V[\x90V[a\x08\xB4a\x08\xAFa\x08\xB9\x92a\x07rV[a\x06\xB0V[a\x07rV[\x90V[a\x08\xC5\x90a\x08\xA0V[\x90V[a\x08\xD1\x90a\x08\xBCV[\x90V[a\x08\xDD\x90a\x08\xC8V[\x90RV[\x91\x90a\x08\xF4\x90_` \x85\x01\x94\x01\x90a\x08\xD4V[V[4a\t&Wa\t\x066`\x04a\x03\x03V[a\t\"a\t\x11a\x08\x91V[a\t\x19a\x02\x02V[\x91\x82\x91\x82a\x08\xE1V[\x03\x90\xF3[a\x02\x08V[\x90V[a\t>\x90`\x08a\tC\x93\x02a\x069V[a\t+V[\x90V[\x90a\tQ\x91Ta\t.V[\x90V[a\t``\x03_\x90a\tFV[\x90V[4a\t\x93Wa\ts6`\x04a\x03\x03V[a\t\x8Fa\t~a\tTV[a\t\x86a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[\x90V[a\t\xAFa\t\xAAa\t\xB4\x92a\t\x98V[a\x06\xB0V[a\x02_V[\x90V[a\t\xC3b'\x8D\0a\t\x9BV[\x90V[a\t\xCEa\t\xB7V[\x90V[4a\n\x01Wa\t\xE16`\x04a\x03\x03V[a\t\xFDa\t\xECa\t\xC6V[a\t\xF4a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[a\n\x12`\x02_\x90a\tFV[\x90V[4a\nEWa\n%6`\x04a\x03\x03V[a\nAa\n0a\n\x06V[a\n8a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[4a\nxWa\nZ6`\x04a\x03\x03V[a\nba\x1A8V[a\nja\x02\x02V[\x80a\nt\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[\x91``\x83\x83\x03\x12a\n\xCAWa\n\x94\x82_\x85\x01a\x07\x9DV[\x92a\n\xA2\x83` \x83\x01a\x07\x9DV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xC5Wa\n\xC1\x92\x01a\x02 V[\x90\x91V[a\x02\x10V[a\x02\x0CV[4a\x0B\x03Wa\n\xFFa\n\xEEa\n\xE56`\x04a\n}V[\x92\x91\x90\x91a\x1A\xF0V[a\n\xF6a\x02\x02V[\x91\x82\x91\x82a\x04\xE4V[\x03\x90\xF3[a\x02\x08V[4a\x0B7Wa\x0B!a\x0B\x1B6`\x04a\x05bV[\x90a\x1CPV[a\x0B)a\x02\x02V[\x80a\x0B3\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[4a\x0BlWa\x0BL6`\x04a\x03\x03V[a\x0Bha\x0BWa\x1CmV[a\x0B_a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[\x90\x91``\x82\x84\x03\x12a\x0B\xA6Wa\x0B\xA3a\x0B\x8C\x84_\x85\x01a\x02vV[\x93a\x0B\x9A\x81` \x86\x01a\x02vV[\x93`@\x01a\x02vV[\x90V[a\x02\x0CV[4a\x0B\xDCWa\x0B\xD8a\x0B\xC7a\x0B\xC16`\x04a\x0BqV[\x91a\x1D:V[a\x0B\xCFa\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[4a\x0C\x11Wa\x0B\xF16`\x04a\x03\x03V[a\x0C\ra\x0B\xFCa\x1D\xB0V[a\x0C\x04a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[a\x0C\x1F\x90a\x07}V[\x90RV[\x91\x90a\x0C6\x90_` \x85\x01\x94\x01\x90a\x0C\x16V[V[4a\x0ChWa\x0CH6`\x04a\x03\x03V[a\x0Cda\x0CSa\x1EBV[a\x0C[a\x02\x02V[\x91\x82\x91\x82a\x0C#V[\x03\x90\xF3[a\x02\x08V[4a\x0C\x9DWa\x0C}6`\x04a\x03\x03V[a\x0C\x99a\x0C\x88a\x1EvV[a\x0C\x90a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[4a\x0C\xD2Wa\x0C\xB26`\x04a\x03\x03V[a\x0C\xCEa\x0C\xBDa\x1E\xC2V[a\x0C\xC5a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[\x90` \x82\x82\x03\x12a\r\x08W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r\x03Wa\x0C\xFF\x92\x01a\x03\xC3V[\x90\x91V[a\x02\x10V[a\x02\x0CV[4a\r<Wa\r&a\r 6`\x04a\x0C\xD7V[\x90a \x0BV[a\r.a\x02\x02V[\x80a\r8\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[\x90` \x82\x82\x03\x12a\rZWa\rW\x91_\x01a\x07\x9DV[\x90V[a\x02\x0CV[4a\r\x8DWa\rwa\rr6`\x04a\rAV[a \xBBV[a\r\x7Fa\x02\x02V[\x80a\r\x89\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\r\xE6Wa\r\xC66`\x04a\x03\x03V[a\r\xE2a\r\xD1a\r\x92V[a\r\xD9a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[a\r\xFFa\r\xFAa\x0E\x04\x92a\x02_V[a\x06\xB0V[a\x02_V[\x90V[\x90a\x0E\x11\x90a\r\xEBV[_R` R`@_ \x90V[_\x1C\x90V[a\x0E.a\x0E3\x91a\x0E\x1DV[a\t+V[\x90V[a\x0E@\x90Ta\x0E\"V[\x90V[a\x0EOa\x0ET\x91a\x0E\x1DV[a\x06=V[\x90V[a\x0Ea\x90Ta\x0ECV[\x90V[a\x0Eo\x90`\x05a\x0E\x07V[\x90a\x0E{_\x83\x01a\x0E6V[\x91a\x0E\x88`\x01\x82\x01a\x0E6V[\x91a\x0E\xA1`\x03a\x0E\x9A`\x02\x85\x01a\x0E6V[\x93\x01a\x0EWV[\x90V[a\x0E\xD9a\x0E\xE0\x94a\x0E\xCF``\x94\x98\x97\x95a\x0E\xC5`\x80\x86\x01\x9A_\x87\x01\x90a\x06\xE7V[` \x85\x01\x90a\x06\xE7V[`@\x83\x01\x90a\x06\xE7V[\x01\x90a\x04\xD7V[V[4a\x0F\x16Wa\x0F\x12a\x0E\xFDa\x0E\xF86`\x04a\x08\rV[a\x0EdV[\x90a\x0F\t\x94\x92\x94a\x02\x02V[\x94\x85\x94\x85a\x0E\xA4V[\x03\x90\xF3[a\x02\x08V[\x90V[a\x0F2a\x0F-a\x0F7\x92a\x0F\x1BV[a\x06\xB0V[a\x02_V[\x90V[a\x0FEa\x13\x88a\x0F\x1EV[\x90V[a\x0FPa\x0F:V[\x90V[4a\x0F\x83Wa\x0Fc6`\x04a\x03\x03V[a\x0F\x7Fa\x0Fna\x0FHV[a\x0Fva\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[4a\x0F\xB6Wa\x0F\xA0a\x0F\x9B6`\x04a\rAV[a!+V[a\x0F\xA8a\x02\x02V[\x80a\x0F\xB2\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[_\x80\xFD[\x91\x90a\x0F\xDCa\x0F\xD632\x90\x86\x85\x91\x92\x90\x91\x92a\x1A\xF0V[\x15a\x03\x1FV[a\x0F\xEBWa\x0F\xE9\x92a\x10HV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x10\x03`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[a\x10\x10\x90a\x08\xBCV[\x90V[`@\x90a\x10?a\x104a\x10F\x95\x97\x96\x94``\x84\x01\x90\x84\x82\x03_\x86\x01Ra\x05\xBAV[\x96` \x83\x01\x90a\x06\xE7V[\x01\x90a\x06\xE7V[V[\x90a\x10T\x903\x92a\x16\xBFV[\x91B\x92a\x10\x96a\x10\x84\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x10\x07V[\x94a\x10\x8Da\x02\x02V[\x93\x84\x93\x84a\x10\x13V[\x03\x90\xA2V[\x90a\x10\xA6\x92\x91a\x0F\xBFV[V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x10\xC6\x90a\x05\xB0V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xE0W`@RV[a\x10\xA8V[\x90a\x10\xF8a\x10\xF1a\x02\x02V[\x92\x83a\x10\xBCV[V[a\x11\x04`\x80a\x10\xE5V[\x90V[_\x90V[_\x90V[a\x11\x17a\x10\xFAV[\x90` \x80\x80\x80\x85a\x11&a\x11\x07V[\x81R\x01a\x111a\x11\x07V[\x81R\x01a\x11<a\x11\x07V[\x81R\x01a\x11Ga\x11\x0BV[\x81RPPV[a\x11Ua\x11\x0FV[\x90V[a\x11b`\x80a\x10\xE5V[\x90V[\x90V[a\x11|a\x11wa\x11\x81\x92a\x11eV[a\x06\xB0V[a\x02_V[\x90V[\x90a\x11\x8E\x90a\x02_V[\x90RV[\x90a\x11\x9C\x90a\x03\x1FV[\x90RV[\x90a\x12\x07a\x11\xFE`\x03a\x11\xB1a\x10\xFAV[\x94a\x11\xC8a\x11\xC0_\x83\x01a\x0E6V[_\x88\x01a\x11\x84V[a\x11\xE0a\x11\xD7`\x01\x83\x01a\x0E6V[` \x88\x01a\x11\x84V[a\x11\xF8a\x11\xEF`\x02\x83\x01a\x0E6V[`@\x88\x01a\x11\x84V[\x01a\x0EWV[``\x84\x01a\x11\x92V[V[a\x12\x12\x90a\x11\xA0V[\x90V[a\x12\x1Da\x11MV[Pa\x121a\x12+`\x04a\x0EWV[\x15a\x03\x1FV[a\x12UWa\x12Ra\x12M`\x05a\x12G`\x03a\x0E6V[\x90a\x0E\x07V[a\x12\tV[\x90V[_a\x12\xAA_a\x12\xA1a\x12\x98_a\x12\x93a\x12\x8A_\x95a\x12\x85a\x12}a\x12wa\x11XV[\x9Aa\x11hV[_\x8B\x01a\x11\x84V[a\x11hV[` \x88\x01a\x11\x84V[a\x11hV[`@\x85\x01a\x11\x84V[``\x83\x01a\x11\x92V[\x90V[P\x90V[P\x90V[` \x91\x81R\x01\x90V[` \x7Fve the same length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x7FData and priority arrays must ha_\x82\x01R\x01RV[a\x13\x18`2`@\x92a\x12\xB5V[a\x13!\x81a\x12\xBEV[\x01\x90V[a\x13:\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x13\x0BV[\x90V[\x15a\x13DWV[a\x13La\x02\x02V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x13b`\x04\x82\x01a\x13%V[\x03\x90\xFD[`\x01a\x13r\x91\x01a\x02_V[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x13\xD7W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x13\xD2W` \x01\x91`\x01\x82\x026\x03\x83\x13a\x13\xCDWV[a\x13\x91V[a\x13\x8DV[a\x13\x89V[\x90\x82\x10\x15a\x13\xF7W` a\x13\xF3\x92\x02\x81\x01\x90a\x13\x95V[\x90\x91V[a\x13uV[\x91\x90\x81\x10\x15a\x14\x0CW` \x02\x01\x90V[a\x13uV[5a\x14\x1B\x81a\x02bV[\x90V[\x90\x92a\x14+\x82\x85\x90a\x12\xADV[\x93a\x14R\x85a\x14La\x14Fa\x14A\x88\x87\x90a\x12\xB1V[a\x02_V[\x91a\x02_V[\x14a\x13=V[a\x14[_a\x11hV[[\x80a\x14oa\x14i\x88a\x02_V[\x91a\x02_V[\x10\x15a\x15\x16Wa\x14\x9D\x90a\x14\x9332\x90a\x14\x8B\x88\x87\x86\x91a\x13\xDCV[\x92\x90\x91a\x1A\xF0V[a\x14\xA2W[a\x13fV[a\x14\\V[3a\x14\xB8a\x14\xB2\x87\x86\x85\x91a\x13\xDCV[\x90a\x16\xBFV[\x90a\x14\xCDa\x14\xC8\x89\x88\x86\x91a\x13\xFCV[a\x14\x11V[B\x92a\x15\x0Ea\x14\xFC\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x10\x07V[\x94a\x15\x05a\x02\x02V[\x93\x84\x93\x84a\x10\x13V[\x03\x90\xA2a\x14\x98V[PPPPPPV[_\x90V[a\x15*a\x15\x1EV[Pa\x155`\x04a\x0EWV[\x90V[\x91\x90a\x15Ua\x15O32\x90\x86\x85\x91\x92\x90\x91\x92a\x1A\xF0V[\x15a\x03\x1FV[a\x15dWa\x15b\x92a\x15\xE3V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x15|`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[\x90\x82_\x93\x92\x827\x01RV[\x91\x90a\x15\xA5\x81a\x15\x9E\x81a\x15\xAA\x95a\x05\x9CV[\x80\x95a\x15\x80V[a\x05\xB0V[\x01\x90V[a\x15\xDAa\x15\xCF`@\x93a\x15\xE1\x96\x98\x97\x95``\x85\x01\x91\x85\x83\x03_\x87\x01Ra\x15\x8BV[\x96` \x83\x01\x90a\x06\xE7V[\x01\x90a\x06\xE7V[V[\x90\x913\x91\x92\x90\x92a\x16*Ba\x16\x18\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x95a\x10\x07V[\x95a\x16!a\x02\x02V[\x94\x85\x94\x85a\x15\xAEV[\x03\x90\xA2V[\x90a\x16:\x92\x91a\x158V[V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x16da\x16_a\x16i\x92a\x11eV[a\x16JV[a\x16AV[\x90V[\x90V[a\x16{a\x16\x80\x91a\x16AV[a\x16lV[\x90RV[\x90P\x90V[\x90\x91\x82a\x16\x99\x81a\x16\xA0\x93a\x16\x84V[\x80\x93a\x15\x80V[\x01\x90V[\x80a\x16\xB5`\x01\x92a\x16\xBC\x96\x94a\x16oV[\x01\x91a\x16\x89V[\x90V[a\x16\xFD\x90a\x16\xCBa\x16<V[Pa\x16\xEEa\x16\xD8_a\x16PV[\x91\x93a\x16\xE2a\x02\x02V[\x94\x85\x93` \x85\x01a\x16\xA4V[` \x82\x01\x81\x03\x82R\x03\x82a\x10\xBCV[\x90V[\x90a\x17\x1Ca\x17\x1632\x90\x85\x85\x91\x92\x90\x91\x92a\x1A\xF0V[\x15a\x03\x1FV[a\x17+Wa\x17)\x91a\x17\xA5V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x17C`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x17ja\x17p\x91\x93\x92\x93a\x02_V[\x92a\x02_V[\x82\x03\x91\x82\x11a\x17{WV[a\x17GV[a\x17\x8Fa\x17\x95\x91\x93\x92\x93a\x02_V[\x92a\x02_V[\x82\x01\x80\x92\x11a\x17\xA0WV[a\x17GV[a\x17\xC1a\x17\xCF\x91a\x17\xBAa\x17\xD4\x94Z\x92a\x17\xF0V[Z\x90a\x17[V[a\x17\xC9a\x0F:V[\x90a\x17\x80V[a!\x92V[V[\x90\x91a\x17\xED\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x15\x8BV[\x90V[3\x90\x91a\x18\x1D\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\x07V[\x92a\x182a\x18)a\x02\x02V[\x92\x83\x92\x83a\x17\xD6V[\x03\x90\xA2V[\x90a\x18A\x91a\x17\0V[V[\x90a\x18U\x91a\x18Pa\"BV[a\x19[V[V[`\xA0\x1C\x90V[a\x18ia\x18n\x91a\x18WV[a\x06=V[\x90V[a\x18{\x90Ta\x18]V[\x90V[a\x18\x92a\x18\x8Da\x18\x97\x92a\x11eV[a\x06\xB0V[a\x07rV[\x90V[a\x18\xA3\x90a\x18~V[\x90V[`\xA0\x1B\x90V[\x90a\x18\xBB`\xFF`\xA0\x1B\x91a\x18\xA6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18\xCE\x90a\x03\x1FV[\x90V[\x90V[\x90a\x18\xE9a\x18\xE4a\x18\xF0\x92a\x18\xC5V[a\x18\xD1V[\x82Ta\x18\xACV[\x90UV[a\x18\xFD\x90a\x08\xA0V[\x90V[a\x19\t\x90a\x18\xF4V[\x90V[_\x1B\x90V[\x90a\x19\"`\x01\x80`\xA0\x1B\x03\x91a\x19\x0CV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x195\x90a\x18\xF4V[\x90V[\x90V[\x90a\x19Pa\x19Ka\x19W\x92a\x19,V[a\x198V[\x82Ta\x19\x11V[\x90UV[a\x19e`\x01a\x18qV[a\x19\xCDW\x81a\x19\x84a\x19~a\x19y_a\x18\x9AV[a\x07}V[\x91a\x07}V[\x14a\x19\xB1Wa\x19\xAAa\x19\xA3a\x19\xAF\x93a\x19\x9E`\x01\x80a\x18\xD4V[a\x19\0V[`\x01a\x19;V[a!+V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x19\xC9`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x19\xE4`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[\x90a\x19\xF2\x91a\x18CV[V[a\x1A\x0Ba\x1A\x10\x91a\x1A\x03a\x11MV[P`\x05a\x0E\x07V[a\x12\tV[\x90V[a\x1A\x1Ba\"BV[a\x1A#a\x1A%V[V[a\x1A6a\x1A1_a\x18\x9AV[a\"\xB3V[V[a\x1A@a\x1A\x13V[V[a\x1ANa\x1AS\x91a\x0E\x1DV[a\x08`V[\x90V[a\x1A`\x90Ta\x1ABV[\x90V[`\xE0\x1B\x90V[a\x1Ar\x81a\x03\x1FV[\x03a\x1AyWV[_\x80\xFD[\x90PQ\x90a\x1A\x8A\x82a\x1AiV[V[\x90` \x82\x82\x03\x12a\x1A\xA5Wa\x1A\xA2\x91_\x01a\x1A}V[\x90V[a\x02\x0CV[a\x1A\xD0a\x1A\xDD\x95\x93\x94\x92\x94a\x1A\xC6``\x84\x01\x96_\x85\x01\x90a\x0C\x16V[` \x83\x01\x90a\x0C\x16V[`@\x81\x85\x03\x91\x01Ra\x15\x8BV[\x90V[a\x1A\xE8a\x02\x02V[=_\x82>=\x90\xFD[\x92a\x1B3` \x93\x94a\x1B\0a\x15\x1EV[Pa\x1B>a\x1B\x16a\x1B\x11`\x01a\x1AVV[a\x08\xC8V[\x93cz9y\xDC\x92\x95\x97a\x1B'a\x02\x02V[\x98\x89\x97\x88\x96\x87\x96a\x1AcV[\x86R`\x04\x86\x01a\x1A\xAAV[\x03\x91Z\xFA\x90\x81\x15a\x1B\x82W_\x91a\x1BTW[P\x90V[a\x1Bu\x91P` =\x81\x11a\x1B{W[a\x1Bm\x81\x83a\x10\xBCV[\x81\x01\x90a\x1A\x8CV[_a\x1BPV[P=a\x1BcV[a\x1A\xE0V[\x90a\x1B\xA3a\x1B\x9D32\x90\x85\x85\x91\x92\x90\x91\x92a\x1A\xF0V[\x15a\x03\x1FV[a\x1B\xB2Wa\x1B\xB0\x91a\x1B\xCEV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x1B\xCA`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[a\x1B\xEAa\x1B\xF8\x91a\x1B\xE3a\x1B\xFD\x94Z\x92a\x1B\xFFV[Z\x90a\x17[V[a\x1B\xF2a\x0F:V[\x90a\x17\x80V[a!\x92V[V[\x90a\x1C\x0B\x903\x92a\x16\xBFV[\x90a\x1CKa\x1C9\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\x07V[\x92a\x1CBa\x02\x02V[\x91\x82\x91\x82a\x05\xEBV[\x03\x90\xA2V[\x90a\x1CZ\x91a\x1B\x87V[V[_\x90V[a\x1Cj\x90Qa\x02_V[\x90V[a\x1Cua\x1C\\V[Pa\x1C\x89a\x1C\x83`\x04a\x0EWV[\x15a\x03\x1FV[a\x1C\xF9Wa\x1C\xC5a\x1C\xB7_a\x1C\xB1a\x1C\xAC`\x05a\x1C\xA6`\x03a\x0E6V[\x90a\x0E\x07V[a\x12\tV[\x01a\x1C`V[a\x1C\xBFa\t\xB7V[\x90a\x17\x80V[Ba\x1C\xD8a\x1C\xD2\x83a\x02_V[\x91a\x02_V[\x10\x15a\x1C\xECWa\x1C\xE9\x90B\x90a\x17[V[\x90V[Pa\x1C\xF6_a\x11hV[\x90V[a\x1D\x02_a\x11hV[\x90V[a\x1D\x14a\x1D\x1A\x91\x93\x92\x93a\x02_V[\x92a\x02_V[\x91a\x1D&\x83\x82\x02a\x02_V[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1D5WV[a\x17GV[\x91a\x1DCa\x1C\\V[P\x80a\x1DWa\x1DQ\x84a\x02_V[\x91a\x02_V[\x11\x15a\x1D\xABWa\x1Dx\x91a\x1Dj\x91a\x17[V[a\x1Dra\x06\xCFV[\x90a\x1D\x05V[\x80a\x1D\x8Ba\x1D\x85\x84a\x02_V[\x91a\x02_V[\x10\x15a\x1D\x9DWa\x1D\x9A\x91a\x17[V[\x90V[PPa\x1D\xA8_a\x11hV[\x90V[PP\x90V[a\x1D\xB8a\x1C\\V[Pa\x1D\xCCa\x1D\xC6`\x04a\x0EWV[\x15a\x03\x1FV[a\x1E\x06Wa\x1E\x03a\x1D\xF3`\x02a\x1D\xED`\x05a\x1D\xE7`\x03a\x0E6V[\x90a\x0E\x07V[\x01a\x0E6V[a\x1D\xFD`\x02a\x0E6V[\x90a\x1D\x05V[\x90V[a\x1E\x0F_a\x11hV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1E-a\x1E2\x91a\x0E\x1DV[a\x1E\x16V[\x90V[a\x1E?\x90Ta\x1E!V[\x90V[a\x1EJa\x1E\x12V[Pa\x1ET_a\x1E5V[\x90V[\x90V[a\x1Ena\x1Eia\x1Es\x92a\x1EWV[a\x06\xB0V[a\x02_V[\x90V[a\x1E~a\x1C\\V[Pa\x1E\x92a\x1E\x8C`\x04a\x0EWV[\x15a\x03\x1FV[a\x1E\xB6Wa\x1E\xB3a\x1E\xA3`\x03a\x0E6V[a\x1E\xAD`\x01a\x1EZV[\x90a\x17\x80V[\x90V[a\x1E\xBF_a\x11hV[\x90V[a\x1E\xCAa\x1C\\V[Pa\x1E\xDEa\x1E\xD8`\x04a\x0EWV[\x15a\x03\x1FV[a\x1F\x05Wa\x1F\x02`\x02a\x1E\xFC`\x05a\x1E\xF6`\x03a\x0E6V[\x90a\x0E\x07V[\x01a\x0E6V[\x90V[a\x1F\x0E_a\x11hV[\x90V[a\x1F-a\x1F;\x91a\x1F&a\x1F@\x94Z\x92a\x1FBV[Z\x90a\x17[V[a\x1F5a\x0F:V[\x90a\x17\x80V[a!\x92V[V[a\x1FM\x81\x83\x90a\x12\xADV[\x91a\x1FVa\x1C\\V[Pa\x1F`_a\x11hV[[\x80a\x1Fta\x1Fn\x86a\x02_V[\x91a\x02_V[\x10\x15a \x05Wa\x1F\xA2\x90a\x1F\x9832\x90a\x1F\x90\x87\x87\x86\x91a\x13\xDCV[\x92\x90\x91a\x1A\xF0V[a\x1F\xA7W[a\x13fV[a\x1FaV[3a\x1F\xBDa\x1F\xB7\x86\x86\x85\x91a\x13\xDCV[\x90a\x16\xBFV[\x90a\x1F\xFDa\x1F\xEB\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\x07V[\x92a\x1F\xF4a\x02\x02V[\x91\x82\x91\x82a\x05\xEBV[\x03\x90\xA2a\x1F\x9DV[PPPPV[\x90a \x15\x91a\x1F\x11V[V[a (\x90a #a\"BV[a *V[V[\x80a Ea ?a :_a\x18\x9AV[a\x07}V[\x91a\x07}V[\x14a \x9FWa ]a V\x82a\x19\0V[`\x01a\x19;V[a \x87\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10\x07V[\x90a \x90a\x02\x02V[\x80a \x9A\x81a\x02\xCAV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a \xB7`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[a \xC4\x90a \x17V[V[a \xD7\x90a \xD2a\"BV[a \xD9V[V[\x80a \xF4a \xEEa \xE9_a\x18\x9AV[a\x07}V[\x91a\x07}V[\x14a!\x04Wa!\x02\x90a\"\xB3V[V[a!'a!\x10_a\x18\x9AV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x0C#V[\x03\x90\xFD[a!4\x90a \xC6V[V[\x90a!B_\x19\x91a\x19\x0CV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a!da!_a!k\x92a\r\xEBV[a!LV[\x82Ta!6V[\x90UV[\x91` a!\x90\x92\x94\x93a!\x89`@\x82\x01\x96_\x83\x01\x90a\x06\xE7V[\x01\x90a\x06\xE7V[V[a!\xA5a!\x9F`\x04a\x0EWV[\x15a\x03\x1FV[a\"5W[a!\xB2a$\xDFV[a!\xE6\x81a!\xE0`\x02a!\xD0`\x05a!\xCA`\x03a\x0E6V[\x90a\x0E\x07V[\x01\x91a!\xDB\x83a\x0E6V[a\x17\x80V[\x90a!OV[a!\xF0`\x03a\x0E6V[:a\"\x1B\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\r\xEBV[\x92a\"0a\"'a\x02\x02V[\x92\x83\x92\x83a!oV[\x03\x90\xA2V[a\"=a#\xDCV[a!\xAAV[a\"Ja\x1EBV[a\"ca\"]a\"Xa&\xBDV[a\x07}V[\x91a\x07}V[\x03a\"jWV[a\"\x8Ca\"ua&\xBDV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\x0C#V[\x03\x90\xFD[\x90V[\x90a\"\xA8a\"\xA3a\"\xAF\x92a\x10\x07V[a\"\x90V[\x82Ta\x19\x11V[\x90UV[a\"\xBC_a\x1E5V[a\"\xC6\x82_a\"\x93V[\x90a\"\xFAa\"\xF4\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10\x07V[\x91a\x10\x07V[\x91a#\x03a\x02\x02V[\x80a#\r\x81a\x02\xCAV[\x03\x90\xA3V[\x90a#\x1E`\xFF\x91a\x19\x0CV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a#=a#8a#D\x92a\x18\xC5V[a\x18\xD1V[\x82Ta#\x12V[\x90UV[\x90a#R\x90a\x11hV[_R` R`@_ \x90V[a#h\x90Qa\x03\x1FV[\x90V[\x90a#\xC8```\x03a#\xCE\x94a#\x8E_\x82\x01a#\x88_\x88\x01a\x1C`V[\x90a!OV[a#\xA7`\x01\x82\x01a#\xA1` \x88\x01a\x1C`V[\x90a!OV[a#\xC0`\x02\x82\x01a#\xBA`@\x88\x01a\x1C`V[\x90a!OV[\x01\x92\x01a#^V[\x90a#(V[V[\x90a#\xDA\x91a#kV[V[a#\xEFa#\xE9`\x04a\x0EWV[\x15a\x03\x1FV[a#\xF6W[V[a$\x02`\x01`\x04a#(V[a$\x15a$\x0E_a\x11hV[`\x03a!OV[a$vBa$e_a$\\a$S_a$Na$E_\x95a$@a$7a\x11XV[\x99_\x8B\x01a\x11\x84V[a\x11hV[` \x88\x01a\x11\x84V[a\x11hV[`@\x85\x01a\x11\x84V[``\x83\x01a\x11\x92V[a$q`\x05_\x90a#HV[a#\xD0V[_B\x90a$\xB8a$\xA6\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x11hV[\x92a$\xAFa\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xA2a#\xF4V[\x90V[a$\xCC\x90a\x02_V[_\x19\x81\x14a$\xDAW`\x01\x01\x90V[a\x17GV[a$\xFCa$\xF7`\x05a$\xF1`\x03a\x0E6V[\x90a\x0E\x07V[a$\xC0V[Ba%*a%$a%\x1Fa%\x11_\x86\x01a\x0E6V[a%\x19a\t\xB7V[\x90a\x17\x80V[a\x02_V[\x91a\x02_V[\x10\x15a%4W[PV[a%\\a%Sa%E_\x84\x01a\x0E6V[a%Ma\t\xB7V[\x90a\x17\x80V[`\x01\x83\x01a!OV[a%j`\x01`\x03\x83\x01a#(V[a%t`\x03a\x0E6V[a%\xA1a%\x83`\x02\x84\x01a\x0E6V[\x92a%\x9B_a%\x94`\x01\x84\x01a\x0E6V[\x92\x01a\x0E6V[\x90a\x17[V[a%\xCB\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\r\xEBV[\x92a%\xE0a%\xD7a\x02\x02V[\x92\x83\x92\x83a!oV[\x03\x90\xA2a%\xFFa%\xF8a%\xF3`\x03a\x0E6V[a$\xC3V[`\x03a!OV[a&iBa&O_a&Fa&=_a&8a&/_\x95a&*a&!a\x11XV[\x99_\x8B\x01a\x11\x84V[a\x11hV[` \x88\x01a\x11\x84V[a\x11hV[`@\x85\x01a\x11\x84V[``\x83\x01a\x11\x92V[a&d`\x05a&^`\x03a\x0E6V[\x90a\x0E\x07V[a#\xD0V[a&s`\x03a\x0E6V[B\x90a&\xB4a&\xA2\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\r\xEBV[\x92a&\xABa\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xA2_a%1V[a&\xC5a\x1E\x12V[P3\x90V`\x80`@R4`\x1CW`\x0E` V[a\x01\x94a\0+\x829a\x01\x94\x90\xF3[`&V[`@Q\x90V[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x01{V[a\0\x1D_5a\0,V[cz9y\xDC\x03a\0\x0EWa\x01BV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\0X\x90a\0DV[\x90V[a\0d\x81a\0OV[\x03a\0kWV[_\x80\xFD[\x90P5\x90a\0|\x82a\0[V[V[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\0\xC4W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xBFW` \x01\x92`\x01\x83\x02\x84\x01\x11a\0\xBAWV[a\0\x86V[a\0\x82V[a\0~V[\x91``\x83\x83\x03\x12a\x01\x16Wa\0\xE0\x82_\x85\x01a\0oV[\x92a\0\xEE\x83` \x83\x01a\0oV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x11Wa\x01\r\x92\x01a\0\x8AV[\x90\x91V[a\0@V[a\0<V[\x15\x15\x90V[a\x01)\x90a\x01\x1BV[\x90RV[\x91\x90a\x01@\x90_` \x85\x01\x94\x01\x90a\x01 V[V[4a\x01vWa\x01ra\x01aa\x01X6`\x04a\0\xC9V[\x92\x91\x90\x91a\x01\x83V[a\x01ia\x002V[\x91\x82\x91\x82a\x01-V[\x03\x90\xF3[a\08V[_\x80\xFD[_\x90V[PPPPa\x01\x8Fa\x01\x7FV[P_\x90V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b610fbb565b61001d5f356101fc565b8063050ec138146101f7578063086146d2146101f257806311992f8c146101ed57806318d5aafe146101e85780631c0b6367146101e3578063366cbab7146101de5780633b6ab2a9146101d95780633d44ae8b146101d457806346e2cc09146101cf578063485cc955146101ca5780634b2c0706146101c55780635b3cd6e2146101c057806361543801146101bb5780636558954f146101b6578063703cfcbb146101b1578063715018a6146101ac5780637a3979dc146101a7578063804e5123146101a257806382f44ade1461019d57806383d3c115146101985780638d5a239b146101935780638da5cb5b1461018e578063aff74c6d14610189578063c660d3f314610184578063cdafb9781461017f578063d4f0eb4d1461017a578063d878134214610175578063ea4a110414610170578063ede07bd61461016b5763f2fde38b0361000e57610f88565b610f53565b610ee2565b610db6565b610d5f565b610d0d565b610ca2565b610c6d565b610c38565b610be1565b610bab565b610b3c565b610b08565b610acf565b610a4a565b610a15565b6109d1565b610963565b6108f6565b61082b565b6107d9565b61073e565b610709565b610678565b610603565b61052e565b6104f9565b6104a0565b61038e565b6102cf565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f8301121561025a5781359167ffffffffffffffff831161025557602001926001830284011161025057565b61021c565b610218565b610214565b90565b61026b8161025f565b0361027257565b5f80fd5b9050359061028382610262565b565b916040838303126102c5575f83013567ffffffffffffffff81116102c0576102b2836102bd928601610220565b939094602001610276565b90565b610210565b61020c565b5f0190565b346102fe576102e86102e2366004610285565b9161109b565b6102f0610202565b806102fa816102ca565b0390f35b610208565b5f91031261030d57565b61020c565b61031b9061025f565b9052565b151590565b61032d9061031f565b9052565b90606080610377936103495f8201515f860190610312565b61035b60208201516020860190610312565b61036d60408201516040860190610312565b0151910190610324565b565b919061038c905f60808501940190610331565b565b346103be5761039e366004610303565b6103ba6103a9611215565b6103b1610202565b91829182610379565b0390f35b610208565b909182601f830112156103fd5781359167ffffffffffffffff83116103f85760200192602083028401116103f357565b61021c565b610218565b610214565b909182601f8301121561043c5781359167ffffffffffffffff831161043757602001926020830284011161043257565b61021c565b610218565b610214565b909160408284031261049b575f82013567ffffffffffffffff8111610496578361046c9184016103c3565b929093602082013567ffffffffffffffff81116104915761048d9201610402565b9091565b610210565b610210565b61020c565b346104d2576104bc6104b3366004610441565b9291909161141e565b6104c4610202565b806104ce816102ca565b0390f35b610208565b6104e09061031f565b9052565b91906104f7905f602085019401906104d7565b565b3461052957610509366004610303565b610525610514611522565b61051c610202565b918291826104e4565b0390f35b610208565b3461055d57610547610541366004610285565b9161162f565b61054f610202565b80610559816102ca565b0390f35b610208565b90602082820312610593575f82013567ffffffffffffffff811161058e5761058a9201610220565b9091565b610210565b61020c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6105d96105e26020936105e7936105d081610598565b9384809361059c565b958691016105a5565b6105b0565b0190565b6106009160208201915f8184039101526105ba565b90565b346106345761063061061f610619366004610562565b906116bf565b610627610202565b918291826105eb565b0390f35b610208565b1c90565b60ff1690565b6106539060086106589302610639565b61063d565b90565b906106669154610643565b90565b61067560045f9061065b565b90565b346106a857610688366004610303565b6106a4610693610669565b61069b610202565b918291826104e4565b0390f35b610208565b90565b90565b6106c76106c26106cc926106ad565b6106b0565b61025f565b90565b6106d9600a6106b3565b90565b6106e46106cf565b90565b6106f09061025f565b9052565b9190610707905f602085019401906106e7565b565b3461073957610719366004610303565b6107356107246106dc565b61072c610202565b918291826106f4565b0390f35b610208565b3461076d57610757610751366004610562565b90611837565b61075f610202565b80610769816102ca565b0390f35b610208565b60018060a01b031690565b61078690610772565b90565b6107928161077d565b0361079957565b5f80fd5b905035906107aa82610789565b565b91906040838203126107d457806107c86107d1925f860161079d565b9360200161079d565b90565b61020c565b34610808576107f26107ec3660046107ac565b906119e8565b6107fa610202565b80610804816102ca565b0390f35b610208565b9060208282031261082657610823915f01610276565b90565b61020c565b3461085b5761085761084661084136600461080d565b6119f4565b61084e610202565b91829182610379565b0390f35b610208565b60018060a01b031690565b61087b9060086108809302610639565b610860565b90565b9061088e915461086b565b90565b61089d60015f90610883565b90565b6108b46108af6108b992610772565b6106b0565b610772565b90565b6108c5906108a0565b90565b6108d1906108bc565b90565b6108dd906108c8565b9052565b91906108f4905f602085019401906108d4565b565b3461092657610906366004610303565b610922610911610891565b610919610202565b918291826108e1565b0390f35b610208565b90565b61093e9060086109439302610639565b61092b565b90565b90610951915461092e565b90565b61096060035f90610946565b90565b3461099357610973366004610303565b61098f61097e610954565b610986610202565b918291826106f4565b0390f35b610208565b90565b6109af6109aa6109b492610998565b6106b0565b61025f565b90565b6109c362278d0061099b565b90565b6109ce6109b7565b90565b34610a01576109e1366004610303565b6109fd6109ec6109c6565b6109f4610202565b918291826106f4565b0390f35b610208565b610a1260025f90610946565b90565b34610a4557610a25366004610303565b610a41610a30610a06565b610a38610202565b918291826106f4565b0390f35b610208565b34610a7857610a5a366004610303565b610a62611a38565b610a6a610202565b80610a74816102ca565b0390f35b610208565b91606083830312610aca57610a94825f850161079d565b92610aa2836020830161079d565b92604082013567ffffffffffffffff8111610ac557610ac19201610220565b9091565b610210565b61020c565b34610b0357610aff610aee610ae5366004610a7d565b92919091611af0565b610af6610202565b918291826104e4565b0390f35b610208565b34610b3757610b21610b1b366004610562565b90611c50565b610b29610202565b80610b33816102ca565b0390f35b610208565b34610b6c57610b4c366004610303565b610b68610b57611c6d565b610b5f610202565b918291826106f4565b0390f35b610208565b9091606082840312610ba657610ba3610b8c845f8501610276565b93610b9a8160208601610276565b93604001610276565b90565b61020c565b34610bdc57610bd8610bc7610bc1366004610b71565b91611d3a565b610bcf610202565b918291826106f4565b0390f35b610208565b34610c1157610bf1366004610303565b610c0d610bfc611db0565b610c04610202565b918291826106f4565b0390f35b610208565b610c1f9061077d565b9052565b9190610c36905f60208501940190610c16565b565b34610c6857610c48366004610303565b610c64610c53611e42565b610c5b610202565b91829182610c23565b0390f35b610208565b34610c9d57610c7d366004610303565b610c99610c88611e76565b610c90610202565b918291826106f4565b0390f35b610208565b34610cd257610cb2366004610303565b610cce610cbd611ec2565b610cc5610202565b918291826106f4565b0390f35b610208565b90602082820312610d08575f82013567ffffffffffffffff8111610d0357610cff92016103c3565b9091565b610210565b61020c565b34610d3c57610d26610d20366004610cd7565b9061200b565b610d2e610202565b80610d38816102ca565b0390f35b610208565b90602082820312610d5a57610d57915f0161079d565b90565b61020c565b34610d8d57610d77610d72366004610d41565b6120bb565b610d7f610202565b80610d89816102ca565b0390f35b610208565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610de657610dc6366004610303565b610de2610dd1610d92565b610dd9610202565b918291826106f4565b0390f35b610208565b610dff610dfa610e049261025f565b6106b0565b61025f565b90565b90610e1190610deb565b5f5260205260405f2090565b5f1c90565b610e2e610e3391610e1d565b61092b565b90565b610e409054610e22565b90565b610e4f610e5491610e1d565b61063d565b90565b610e619054610e43565b90565b610e6f906005610e07565b90610e7b5f8301610e36565b91610e8860018201610e36565b91610ea16003610e9a60028501610e36565b9301610e57565b90565b610ed9610ee094610ecf606094989795610ec5608086019a5f8701906106e7565b60208501906106e7565b60408301906106e7565b01906104d7565b565b34610f1657610f12610efd610ef836600461080d565b610e64565b90610f09949294610202565b94859485610ea4565b0390f35b610208565b90565b610f32610f2d610f3792610f1b565b6106b0565b61025f565b90565b610f45611388610f1e565b90565b610f50610f3a565b90565b34610f8357610f63366004610303565b610f7f610f6e610f48565b610f76610202565b918291826106f4565b0390f35b610208565b34610fb657610fa0610f9b366004610d41565b61212b565b610fa8610202565b80610fb2816102ca565b0390f35b610208565b5f80fd5b9190610fdc610fd633329086859192909192611af0565b1561031f565b610feb57610fe992611048565b565b5f631b8e828b60e31b815280611003600482016102ca565b0390fd5b611010906108bc565b90565b60409061103f6110346110469597969460608401908482035f8601526105ba565b9660208301906106e7565b01906106e7565b565b906110549033926116bf565b9142926110966110847f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f294611007565b9461108d610202565b93849384611013565b0390a2565b906110a69291610fbf565b565b634e487b7160e01b5f52604160045260245ffd5b906110c6906105b0565b810190811067ffffffffffffffff8211176110e057604052565b6110a8565b906110f86110f1610202565b92836110bc565b565b61110460806110e5565b90565b5f90565b5f90565b6111176110fa565b90602080808085611126611107565b815201611131611107565b81520161113c611107565b81520161114761110b565b81525050565b61115561110f565b90565b61116260806110e5565b90565b90565b61117c61117761118192611165565b6106b0565b61025f565b90565b9061118e9061025f565b9052565b9061119c9061031f565b9052565b906112076111fe60036111b16110fa565b946111c86111c05f8301610e36565b5f8801611184565b6111e06111d760018301610e36565b60208801611184565b6111f86111ef60028301610e36565b60408801611184565b01610e57565b60608401611192565b565b611212906111a0565b90565b61121d61114d565b5061123161122b6004610e57565b1561031f565b6112555761125261124d60056112476003610e36565b90610e07565b611209565b90565b5f6112aa5f6112a16112985f61129361128a5f9561128561127d611277611158565b9a611168565b5f8b01611184565b611168565b60208801611184565b611168565b60408501611184565b60608301611192565b90565b5090565b5090565b60209181520190565b60207f7665207468652073616d65206c656e6774680000000000000000000000000000917f4461746120616e64207072696f7269747920617272617973206d7573742068615f8201520152565b61131860326040926112b5565b611321816112be565b0190565b61133a9060208101905f81830391015261130b565b90565b1561134457565b61134c610202565b62461bcd60e51b81528061136260048201611325565b0390fd5b6001611372910161025f565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b9035906001602003813603038212156113d7570180359067ffffffffffffffff82116113d2576020019160018202360383136113cd57565b611391565b61138d565b611389565b908210156113f75760206113f39202810190611395565b9091565b611375565b919081101561140c576020020190565b611375565b3561141b81610262565b90565b909261142b8285906112ad565b936114528561144c6114466114418887906112b1565b61025f565b9161025f565b1461133d565b61145b5f611168565b5b8061146f6114698861025f565b9161025f565b10156115165761149d9061149333329061148b888786916113dc565b929091611af0565b6114a2575b611366565b61145c565b336114b86114b2878685916113dc565b906116bf565b906114cd6114c8898886916113fc565b611411565b429261150e6114fc7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f294611007565b94611505610202565b93849384611013565b0390a2611498565b505050505050565b5f90565b61152a61151e565b506115356004610e57565b90565b919061155561154f33329086859192909192611af0565b1561031f565b61156457611562926115e3565b565b5f631b8e828b60e31b81528061157c600482016102ca565b0390fd5b90825f939282370152565b91906115a58161159e816115aa9561059c565b8095611580565b6105b0565b0190565b6115da6115cf6040936115e19698979560608501918583035f87015261158b565b9660208301906106e7565b01906106e7565b565b9091339192909261162a426116187f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f295611007565b95611621610202565b948594856115ae565b0390a2565b9061163a9291611538565b565b606090565b60ff60f81b1690565b60f81b90565b61166461165f61166992611165565b61164a565b611641565b90565b90565b61167b61168091611641565b61166c565b9052565b905090565b909182611699816116a093611684565b8093611580565b0190565b806116b56001926116bc969461166f565b0191611689565b90565b6116fd906116cb61163c565b506116ee6116d85f611650565b91936116e2610202565b948593602085016116a4565b602082018103825203826110bc565b90565b9061171c61171633329085859192909192611af0565b1561031f565b61172b57611729916117a5565b565b5f631b8e828b60e31b815280611743600482016102ca565b0390fd5b634e487b7160e01b5f52601160045260245ffd5b61176a6117709193929361025f565b9261025f565b820391821161177b57565b611747565b61178f6117959193929361025f565b9261025f565b82018092116117a057565b611747565b6117c16117cf916117ba6117d4945a926117f0565b5a9061175b565b6117c9610f3a565b90611780565b612192565b565b90916117ed9260208301925f81850391015261158b565b90565b33909161181d7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611007565b92611832611829610202565b928392836117d6565b0390a2565b9061184191611700565b565b9061185591611850612242565b61195b565b565b60a01c90565b61186961186e91611857565b61063d565b90565b61187b905461185d565b90565b61189261188d61189792611165565b6106b0565b610772565b90565b6118a39061187e565b90565b60a01b90565b906118bb60ff60a01b916118a6565b9181191691161790565b6118ce9061031f565b90565b90565b906118e96118e46118f0926118c5565b6118d1565b82546118ac565b9055565b6118fd906108a0565b90565b611909906118f4565b90565b5f1b90565b9061192260018060a01b039161190c565b9181191691161790565b611935906118f4565b90565b90565b9061195061194b6119579261192c565b611938565b8254611911565b9055565b6119656001611871565b6119cd578161198461197e6119795f61189a565b61077d565b9161077d565b146119b1576119aa6119a36119af9361199e6001806118d4565b611900565b600161193b565b61212b565b565b5f632e7f3c7f60e11b8152806119c9600482016102ca565b0390fd5b5f62dc149f60e41b8152806119e4600482016102ca565b0390fd5b906119f291611843565b565b611a0b611a1091611a0361114d565b506005610e07565b611209565b90565b611a1b612242565b611a23611a25565b565b611a36611a315f61189a565b6122b3565b565b611a40611a13565b565b611a4e611a5391610e1d565b610860565b90565b611a609054611a42565b90565b60e01b90565b611a728161031f565b03611a7957565b5f80fd5b90505190611a8a82611a69565b565b90602082820312611aa557611aa2915f01611a7d565b90565b61020c565b611ad0611add9593949294611ac660608401965f850190610c16565b6020830190610c16565b604081850391015261158b565b90565b611ae8610202565b3d5f823e3d90fd5b92611b3360209394611b0061151e565b50611b3e611b16611b116001611a56565b6108c8565b93637a3979dc929597611b27610202565b98899788968796611a63565b865260048601611aaa565b03915afa908115611b82575f91611b54575b5090565b611b75915060203d8111611b7b575b611b6d81836110bc565b810190611a8c565b5f611b50565b503d611b63565b611ae0565b90611ba3611b9d33329085859192909192611af0565b1561031f565b611bb257611bb091611bce565b565b5f631b8e828b60e31b815280611bca600482016102ca565b0390fd5b611bea611bf891611be3611bfd945a92611bff565b5a9061175b565b611bf2610f3a565b90611780565b612192565b565b90611c0b9033926116bf565b90611c4b611c397f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611007565b92611c42610202565b918291826105eb565b0390a2565b90611c5a91611b87565b565b5f90565b611c6a905161025f565b90565b611c75611c5c565b50611c89611c836004610e57565b1561031f565b611cf957611cc5611cb75f611cb1611cac6005611ca66003610e36565b90610e07565b611209565b01611c60565b611cbf6109b7565b90611780565b42611cd8611cd28361025f565b9161025f565b1015611cec57611ce990429061175b565b90565b50611cf65f611168565b90565b611d025f611168565b90565b611d14611d1a9193929361025f565b9261025f565b91611d2683820261025f565b928184041490151715611d3557565b611747565b91611d43611c5c565b5080611d57611d518461025f565b9161025f565b1115611dab57611d7891611d6a9161175b565b611d726106cf565b90611d05565b80611d8b611d858461025f565b9161025f565b1015611d9d57611d9a9161175b565b90565b5050611da85f611168565b90565b505090565b611db8611c5c565b50611dcc611dc66004610e57565b1561031f565b611e0657611e03611df36002611ded6005611de76003610e36565b90610e07565b01610e36565b611dfd6002610e36565b90611d05565b90565b611e0f5f611168565b90565b5f90565b60018060a01b031690565b611e2d611e3291610e1d565b611e16565b90565b611e3f9054611e21565b90565b611e4a611e12565b50611e545f611e35565b90565b90565b611e6e611e69611e7392611e57565b6106b0565b61025f565b90565b611e7e611c5c565b50611e92611e8c6004610e57565b1561031f565b611eb657611eb3611ea36003610e36565b611ead6001611e5a565b90611780565b90565b611ebf5f611168565b90565b611eca611c5c565b50611ede611ed86004610e57565b1561031f565b611f0557611f026002611efc6005611ef66003610e36565b90610e07565b01610e36565b90565b611f0e5f611168565b90565b611f2d611f3b91611f26611f40945a92611f42565b5a9061175b565b611f35610f3a565b90611780565b612192565b565b611f4d8183906112ad565b91611f56611c5c565b50611f605f611168565b5b80611f74611f6e8661025f565b9161025f565b101561200557611fa290611f98333290611f90878786916113dc565b929091611af0565b611fa7575b611366565b611f61565b33611fbd611fb7868685916113dc565b906116bf565b90611ffd611feb7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611007565b92611ff4610202565b918291826105eb565b0390a2611f9d565b50505050565b9061201591611f11565b565b61202890612023612242565b61202a565b565b8061204561203f61203a5f61189a565b61077d565b9161077d565b1461209f5761205d61205682611900565b600161193b565b6120877f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b991611007565b90612090610202565b8061209a816102ca565b0390a2565b5f632e7f3c7f60e11b8152806120b7600482016102ca565b0390fd5b6120c490612017565b565b6120d7906120d2612242565b6120d9565b565b806120f46120ee6120e95f61189a565b61077d565b9161077d565b1461210457612102906122b3565b565b6121276121105f61189a565b5f918291631e4fbdf760e01b835260048301610c23565b0390fd5b612134906120c6565b565b906121425f199161190c565b9181191691161790565b90565b9061216461215f61216b92610deb565b61214c565b8254612136565b9055565b91602061219092949361218960408201965f8301906106e7565b01906106e7565b565b6121a561219f6004610e57565b1561031f565b612235575b6121b26124df565b6121e6816121e060026121d060056121ca6003610e36565b90610e07565b01916121db83610e36565b611780565b9061214f565b6121f06003610e36565b3a61221b7f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610deb565b92612230612227610202565b9283928361216f565b0390a2565b61223d6123dc565b6121aa565b61224a611e42565b61226361225d6122586126bd565b61077d565b9161077d565b0361226a57565b61228c6122756126bd565b5f91829163118cdaa760e01b835260048301610c23565b0390fd5b90565b906122a86122a36122af92611007565b612290565b8254611911565b9055565b6122bc5f611e35565b6122c6825f612293565b906122fa6122f47f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093611007565b91611007565b91612303610202565b8061230d816102ca565b0390a3565b9061231e60ff9161190c565b9181191691161790565b9061233d612338612344926118c5565b6118d1565b8254612312565b9055565b9061235290611168565b5f5260205260405f2090565b612368905161031f565b90565b906123c8606060036123ce9461238e5f82016123885f8801611c60565b9061214f565b6123a7600182016123a160208801611c60565b9061214f565b6123c0600282016123ba60408801611c60565b9061214f565b01920161235e565b90612328565b565b906123da9161236b565b565b6123ef6123e96004610e57565b1561031f565b6123f6575b565b61240260016004612328565b61241561240e5f611168565b600361214f565b612476426124655f61245c6124535f61244e6124455f95612440612437611158565b995f8b01611184565b611168565b60208801611184565b611168565b60408501611184565b60608301611192565b61247160055f90612348565b6123d0565b5f42906124b86124a67f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92611168565b926124af610202565b918291826106f4565b0390a26123f4565b90565b6124cc9061025f565b5f1981146124da5760010190565b611747565b6124fc6124f760056124f16003610e36565b90610e07565b6124c0565b4261252a61252461251f6125115f8601610e36565b6125196109b7565b90611780565b61025f565b9161025f565b1015612534575b50565b61255c6125536125455f8401610e36565b61254d6109b7565b90611780565b6001830161214f565b61256a600160038301612328565b6125746003610e36565b6125a161258360028401610e36565b9261259b5f61259460018401610e36565b9201610e36565b9061175b565b6125cb7f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610deb565b926125e06125d7610202565b9283928361216f565b0390a26125ff6125f86125f36003610e36565b6124c3565b600361214f565b6126694261264f5f61264661263d5f61263861262f5f9561262a612621611158565b995f8b01611184565b611168565b60208801611184565b611168565b60408501611184565b60608301611192565b612664600561265e6003610e36565b90610e07565b6123d0565b6126736003610e36565b42906126b46126a27f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610deb565b926126ab610202565b918291826106f4565b0390a25f612531565b6126c5611e12565b50339056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x0F\xBBV[a\0\x1D_5a\x01\xFCV[\x80c\x05\x0E\xC18\x14a\x01\xF7W\x80c\x08aF\xD2\x14a\x01\xF2W\x80c\x11\x99/\x8C\x14a\x01\xEDW\x80c\x18\xD5\xAA\xFE\x14a\x01\xE8W\x80c\x1C\x0Bcg\x14a\x01\xE3W\x80c6l\xBA\xB7\x14a\x01\xDEW\x80c;j\xB2\xA9\x14a\x01\xD9W\x80c=D\xAE\x8B\x14a\x01\xD4W\x80cF\xE2\xCC\t\x14a\x01\xCFW\x80cH\\\xC9U\x14a\x01\xCAW\x80cK,\x07\x06\x14a\x01\xC5W\x80c[<\xD6\xE2\x14a\x01\xC0W\x80caT8\x01\x14a\x01\xBBW\x80ceX\x95O\x14a\x01\xB6W\x80cp<\xFC\xBB\x14a\x01\xB1W\x80cqP\x18\xA6\x14a\x01\xACW\x80cz9y\xDC\x14a\x01\xA7W\x80c\x80NQ#\x14a\x01\xA2W\x80c\x82\xF4J\xDE\x14a\x01\x9DW\x80c\x83\xD3\xC1\x15\x14a\x01\x98W\x80c\x8DZ#\x9B\x14a\x01\x93W\x80c\x8D\xA5\xCB[\x14a\x01\x8EW\x80c\xAF\xF7Lm\x14a\x01\x89W\x80c\xC6`\xD3\xF3\x14a\x01\x84W\x80c\xCD\xAF\xB9x\x14a\x01\x7FW\x80c\xD4\xF0\xEBM\x14a\x01zW\x80c\xD8x\x13B\x14a\x01uW\x80c\xEAJ\x11\x04\x14a\x01pW\x80c\xED\xE0{\xD6\x14a\x01kWc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\x0F\x88V[a\x0FSV[a\x0E\xE2V[a\r\xB6V[a\r_V[a\r\rV[a\x0C\xA2V[a\x0CmV[a\x0C8V[a\x0B\xE1V[a\x0B\xABV[a\x0B<V[a\x0B\x08V[a\n\xCFV[a\nJV[a\n\x15V[a\t\xD1V[a\tcV[a\x08\xF6V[a\x08+V[a\x07\xD9V[a\x07>V[a\x07\tV[a\x06xV[a\x06\x03V[a\x05.V[a\x04\xF9V[a\x04\xA0V[a\x03\x8EV[a\x02\xCFV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x02ZW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x02UW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x02PWV[a\x02\x1CV[a\x02\x18V[a\x02\x14V[\x90V[a\x02k\x81a\x02_V[\x03a\x02rWV[_\x80\xFD[\x90P5\x90a\x02\x83\x82a\x02bV[V[\x91`@\x83\x83\x03\x12a\x02\xC5W_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02\xC0Wa\x02\xB2\x83a\x02\xBD\x92\x86\x01a\x02 V[\x93\x90\x94` \x01a\x02vV[\x90V[a\x02\x10V[a\x02\x0CV[_\x01\x90V[4a\x02\xFEWa\x02\xE8a\x02\xE26`\x04a\x02\x85V[\x91a\x10\x9BV[a\x02\xF0a\x02\x02V[\x80a\x02\xFA\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[_\x91\x03\x12a\x03\rWV[a\x02\x0CV[a\x03\x1B\x90a\x02_V[\x90RV[\x15\x15\x90V[a\x03-\x90a\x03\x1FV[\x90RV[\x90``\x80a\x03w\x93a\x03I_\x82\x01Q_\x86\x01\x90a\x03\x12V[a\x03[` \x82\x01Q` \x86\x01\x90a\x03\x12V[a\x03m`@\x82\x01Q`@\x86\x01\x90a\x03\x12V[\x01Q\x91\x01\x90a\x03$V[V[\x91\x90a\x03\x8C\x90_`\x80\x85\x01\x94\x01\x90a\x031V[V[4a\x03\xBEWa\x03\x9E6`\x04a\x03\x03V[a\x03\xBAa\x03\xA9a\x12\x15V[a\x03\xB1a\x02\x02V[\x91\x82\x91\x82a\x03yV[\x03\x90\xF3[a\x02\x08V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03\xFDW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\xF8W` \x01\x92` \x83\x02\x84\x01\x11a\x03\xF3WV[a\x02\x1CV[a\x02\x18V[a\x02\x14V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04<W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x047W` \x01\x92` \x83\x02\x84\x01\x11a\x042WV[a\x02\x1CV[a\x02\x18V[a\x02\x14V[\x90\x91`@\x82\x84\x03\x12a\x04\x9BW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x96W\x83a\x04l\x91\x84\x01a\x03\xC3V[\x92\x90\x93` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x91Wa\x04\x8D\x92\x01a\x04\x02V[\x90\x91V[a\x02\x10V[a\x02\x10V[a\x02\x0CV[4a\x04\xD2Wa\x04\xBCa\x04\xB36`\x04a\x04AV[\x92\x91\x90\x91a\x14\x1EV[a\x04\xC4a\x02\x02V[\x80a\x04\xCE\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[a\x04\xE0\x90a\x03\x1FV[\x90RV[\x91\x90a\x04\xF7\x90_` \x85\x01\x94\x01\x90a\x04\xD7V[V[4a\x05)Wa\x05\t6`\x04a\x03\x03V[a\x05%a\x05\x14a\x15\"V[a\x05\x1Ca\x02\x02V[\x91\x82\x91\x82a\x04\xE4V[\x03\x90\xF3[a\x02\x08V[4a\x05]Wa\x05Ga\x05A6`\x04a\x02\x85V[\x91a\x16/V[a\x05Oa\x02\x02V[\x80a\x05Y\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[\x90` \x82\x82\x03\x12a\x05\x93W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05\x8EWa\x05\x8A\x92\x01a\x02 V[\x90\x91V[a\x02\x10V[a\x02\x0CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x05\xD9a\x05\xE2` \x93a\x05\xE7\x93a\x05\xD0\x81a\x05\x98V[\x93\x84\x80\x93a\x05\x9CV[\x95\x86\x91\x01a\x05\xA5V[a\x05\xB0V[\x01\x90V[a\x06\0\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x05\xBAV[\x90V[4a\x064Wa\x060a\x06\x1Fa\x06\x196`\x04a\x05bV[\x90a\x16\xBFV[a\x06'a\x02\x02V[\x91\x82\x91\x82a\x05\xEBV[\x03\x90\xF3[a\x02\x08V[\x1C\x90V[`\xFF\x16\x90V[a\x06S\x90`\x08a\x06X\x93\x02a\x069V[a\x06=V[\x90V[\x90a\x06f\x91Ta\x06CV[\x90V[a\x06u`\x04_\x90a\x06[V[\x90V[4a\x06\xA8Wa\x06\x886`\x04a\x03\x03V[a\x06\xA4a\x06\x93a\x06iV[a\x06\x9Ba\x02\x02V[\x91\x82\x91\x82a\x04\xE4V[\x03\x90\xF3[a\x02\x08V[\x90V[\x90V[a\x06\xC7a\x06\xC2a\x06\xCC\x92a\x06\xADV[a\x06\xB0V[a\x02_V[\x90V[a\x06\xD9`\na\x06\xB3V[\x90V[a\x06\xE4a\x06\xCFV[\x90V[a\x06\xF0\x90a\x02_V[\x90RV[\x91\x90a\x07\x07\x90_` \x85\x01\x94\x01\x90a\x06\xE7V[V[4a\x079Wa\x07\x196`\x04a\x03\x03V[a\x075a\x07$a\x06\xDCV[a\x07,a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[4a\x07mWa\x07Wa\x07Q6`\x04a\x05bV[\x90a\x187V[a\x07_a\x02\x02V[\x80a\x07i\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x07\x86\x90a\x07rV[\x90V[a\x07\x92\x81a\x07}V[\x03a\x07\x99WV[_\x80\xFD[\x90P5\x90a\x07\xAA\x82a\x07\x89V[V[\x91\x90`@\x83\x82\x03\x12a\x07\xD4W\x80a\x07\xC8a\x07\xD1\x92_\x86\x01a\x07\x9DV[\x93` \x01a\x07\x9DV[\x90V[a\x02\x0CV[4a\x08\x08Wa\x07\xF2a\x07\xEC6`\x04a\x07\xACV[\x90a\x19\xE8V[a\x07\xFAa\x02\x02V[\x80a\x08\x04\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[\x90` \x82\x82\x03\x12a\x08&Wa\x08#\x91_\x01a\x02vV[\x90V[a\x02\x0CV[4a\x08[Wa\x08Wa\x08Fa\x08A6`\x04a\x08\rV[a\x19\xF4V[a\x08Na\x02\x02V[\x91\x82\x91\x82a\x03yV[\x03\x90\xF3[a\x02\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08{\x90`\x08a\x08\x80\x93\x02a\x069V[a\x08`V[\x90V[\x90a\x08\x8E\x91Ta\x08kV[\x90V[a\x08\x9D`\x01_\x90a\x08\x83V[\x90V[a\x08\xB4a\x08\xAFa\x08\xB9\x92a\x07rV[a\x06\xB0V[a\x07rV[\x90V[a\x08\xC5\x90a\x08\xA0V[\x90V[a\x08\xD1\x90a\x08\xBCV[\x90V[a\x08\xDD\x90a\x08\xC8V[\x90RV[\x91\x90a\x08\xF4\x90_` \x85\x01\x94\x01\x90a\x08\xD4V[V[4a\t&Wa\t\x066`\x04a\x03\x03V[a\t\"a\t\x11a\x08\x91V[a\t\x19a\x02\x02V[\x91\x82\x91\x82a\x08\xE1V[\x03\x90\xF3[a\x02\x08V[\x90V[a\t>\x90`\x08a\tC\x93\x02a\x069V[a\t+V[\x90V[\x90a\tQ\x91Ta\t.V[\x90V[a\t``\x03_\x90a\tFV[\x90V[4a\t\x93Wa\ts6`\x04a\x03\x03V[a\t\x8Fa\t~a\tTV[a\t\x86a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[\x90V[a\t\xAFa\t\xAAa\t\xB4\x92a\t\x98V[a\x06\xB0V[a\x02_V[\x90V[a\t\xC3b'\x8D\0a\t\x9BV[\x90V[a\t\xCEa\t\xB7V[\x90V[4a\n\x01Wa\t\xE16`\x04a\x03\x03V[a\t\xFDa\t\xECa\t\xC6V[a\t\xF4a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[a\n\x12`\x02_\x90a\tFV[\x90V[4a\nEWa\n%6`\x04a\x03\x03V[a\nAa\n0a\n\x06V[a\n8a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[4a\nxWa\nZ6`\x04a\x03\x03V[a\nba\x1A8V[a\nja\x02\x02V[\x80a\nt\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[\x91``\x83\x83\x03\x12a\n\xCAWa\n\x94\x82_\x85\x01a\x07\x9DV[\x92a\n\xA2\x83` \x83\x01a\x07\x9DV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xC5Wa\n\xC1\x92\x01a\x02 V[\x90\x91V[a\x02\x10V[a\x02\x0CV[4a\x0B\x03Wa\n\xFFa\n\xEEa\n\xE56`\x04a\n}V[\x92\x91\x90\x91a\x1A\xF0V[a\n\xF6a\x02\x02V[\x91\x82\x91\x82a\x04\xE4V[\x03\x90\xF3[a\x02\x08V[4a\x0B7Wa\x0B!a\x0B\x1B6`\x04a\x05bV[\x90a\x1CPV[a\x0B)a\x02\x02V[\x80a\x0B3\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[4a\x0BlWa\x0BL6`\x04a\x03\x03V[a\x0Bha\x0BWa\x1CmV[a\x0B_a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[\x90\x91``\x82\x84\x03\x12a\x0B\xA6Wa\x0B\xA3a\x0B\x8C\x84_\x85\x01a\x02vV[\x93a\x0B\x9A\x81` \x86\x01a\x02vV[\x93`@\x01a\x02vV[\x90V[a\x02\x0CV[4a\x0B\xDCWa\x0B\xD8a\x0B\xC7a\x0B\xC16`\x04a\x0BqV[\x91a\x1D:V[a\x0B\xCFa\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[4a\x0C\x11Wa\x0B\xF16`\x04a\x03\x03V[a\x0C\ra\x0B\xFCa\x1D\xB0V[a\x0C\x04a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[a\x0C\x1F\x90a\x07}V[\x90RV[\x91\x90a\x0C6\x90_` \x85\x01\x94\x01\x90a\x0C\x16V[V[4a\x0ChWa\x0CH6`\x04a\x03\x03V[a\x0Cda\x0CSa\x1EBV[a\x0C[a\x02\x02V[\x91\x82\x91\x82a\x0C#V[\x03\x90\xF3[a\x02\x08V[4a\x0C\x9DWa\x0C}6`\x04a\x03\x03V[a\x0C\x99a\x0C\x88a\x1EvV[a\x0C\x90a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[4a\x0C\xD2Wa\x0C\xB26`\x04a\x03\x03V[a\x0C\xCEa\x0C\xBDa\x1E\xC2V[a\x0C\xC5a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[\x90` \x82\x82\x03\x12a\r\x08W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r\x03Wa\x0C\xFF\x92\x01a\x03\xC3V[\x90\x91V[a\x02\x10V[a\x02\x0CV[4a\r<Wa\r&a\r 6`\x04a\x0C\xD7V[\x90a \x0BV[a\r.a\x02\x02V[\x80a\r8\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[\x90` \x82\x82\x03\x12a\rZWa\rW\x91_\x01a\x07\x9DV[\x90V[a\x02\x0CV[4a\r\x8DWa\rwa\rr6`\x04a\rAV[a \xBBV[a\r\x7Fa\x02\x02V[\x80a\r\x89\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\r\xE6Wa\r\xC66`\x04a\x03\x03V[a\r\xE2a\r\xD1a\r\x92V[a\r\xD9a\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[a\r\xFFa\r\xFAa\x0E\x04\x92a\x02_V[a\x06\xB0V[a\x02_V[\x90V[\x90a\x0E\x11\x90a\r\xEBV[_R` R`@_ \x90V[_\x1C\x90V[a\x0E.a\x0E3\x91a\x0E\x1DV[a\t+V[\x90V[a\x0E@\x90Ta\x0E\"V[\x90V[a\x0EOa\x0ET\x91a\x0E\x1DV[a\x06=V[\x90V[a\x0Ea\x90Ta\x0ECV[\x90V[a\x0Eo\x90`\x05a\x0E\x07V[\x90a\x0E{_\x83\x01a\x0E6V[\x91a\x0E\x88`\x01\x82\x01a\x0E6V[\x91a\x0E\xA1`\x03a\x0E\x9A`\x02\x85\x01a\x0E6V[\x93\x01a\x0EWV[\x90V[a\x0E\xD9a\x0E\xE0\x94a\x0E\xCF``\x94\x98\x97\x95a\x0E\xC5`\x80\x86\x01\x9A_\x87\x01\x90a\x06\xE7V[` \x85\x01\x90a\x06\xE7V[`@\x83\x01\x90a\x06\xE7V[\x01\x90a\x04\xD7V[V[4a\x0F\x16Wa\x0F\x12a\x0E\xFDa\x0E\xF86`\x04a\x08\rV[a\x0EdV[\x90a\x0F\t\x94\x92\x94a\x02\x02V[\x94\x85\x94\x85a\x0E\xA4V[\x03\x90\xF3[a\x02\x08V[\x90V[a\x0F2a\x0F-a\x0F7\x92a\x0F\x1BV[a\x06\xB0V[a\x02_V[\x90V[a\x0FEa\x13\x88a\x0F\x1EV[\x90V[a\x0FPa\x0F:V[\x90V[4a\x0F\x83Wa\x0Fc6`\x04a\x03\x03V[a\x0F\x7Fa\x0Fna\x0FHV[a\x0Fva\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xF3[a\x02\x08V[4a\x0F\xB6Wa\x0F\xA0a\x0F\x9B6`\x04a\rAV[a!+V[a\x0F\xA8a\x02\x02V[\x80a\x0F\xB2\x81a\x02\xCAV[\x03\x90\xF3[a\x02\x08V[_\x80\xFD[\x91\x90a\x0F\xDCa\x0F\xD632\x90\x86\x85\x91\x92\x90\x91\x92a\x1A\xF0V[\x15a\x03\x1FV[a\x0F\xEBWa\x0F\xE9\x92a\x10HV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x10\x03`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[a\x10\x10\x90a\x08\xBCV[\x90V[`@\x90a\x10?a\x104a\x10F\x95\x97\x96\x94``\x84\x01\x90\x84\x82\x03_\x86\x01Ra\x05\xBAV[\x96` \x83\x01\x90a\x06\xE7V[\x01\x90a\x06\xE7V[V[\x90a\x10T\x903\x92a\x16\xBFV[\x91B\x92a\x10\x96a\x10\x84\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x10\x07V[\x94a\x10\x8Da\x02\x02V[\x93\x84\x93\x84a\x10\x13V[\x03\x90\xA2V[\x90a\x10\xA6\x92\x91a\x0F\xBFV[V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x10\xC6\x90a\x05\xB0V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xE0W`@RV[a\x10\xA8V[\x90a\x10\xF8a\x10\xF1a\x02\x02V[\x92\x83a\x10\xBCV[V[a\x11\x04`\x80a\x10\xE5V[\x90V[_\x90V[_\x90V[a\x11\x17a\x10\xFAV[\x90` \x80\x80\x80\x85a\x11&a\x11\x07V[\x81R\x01a\x111a\x11\x07V[\x81R\x01a\x11<a\x11\x07V[\x81R\x01a\x11Ga\x11\x0BV[\x81RPPV[a\x11Ua\x11\x0FV[\x90V[a\x11b`\x80a\x10\xE5V[\x90V[\x90V[a\x11|a\x11wa\x11\x81\x92a\x11eV[a\x06\xB0V[a\x02_V[\x90V[\x90a\x11\x8E\x90a\x02_V[\x90RV[\x90a\x11\x9C\x90a\x03\x1FV[\x90RV[\x90a\x12\x07a\x11\xFE`\x03a\x11\xB1a\x10\xFAV[\x94a\x11\xC8a\x11\xC0_\x83\x01a\x0E6V[_\x88\x01a\x11\x84V[a\x11\xE0a\x11\xD7`\x01\x83\x01a\x0E6V[` \x88\x01a\x11\x84V[a\x11\xF8a\x11\xEF`\x02\x83\x01a\x0E6V[`@\x88\x01a\x11\x84V[\x01a\x0EWV[``\x84\x01a\x11\x92V[V[a\x12\x12\x90a\x11\xA0V[\x90V[a\x12\x1Da\x11MV[Pa\x121a\x12+`\x04a\x0EWV[\x15a\x03\x1FV[a\x12UWa\x12Ra\x12M`\x05a\x12G`\x03a\x0E6V[\x90a\x0E\x07V[a\x12\tV[\x90V[_a\x12\xAA_a\x12\xA1a\x12\x98_a\x12\x93a\x12\x8A_\x95a\x12\x85a\x12}a\x12wa\x11XV[\x9Aa\x11hV[_\x8B\x01a\x11\x84V[a\x11hV[` \x88\x01a\x11\x84V[a\x11hV[`@\x85\x01a\x11\x84V[``\x83\x01a\x11\x92V[\x90V[P\x90V[P\x90V[` \x91\x81R\x01\x90V[` \x7Fve the same length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x7FData and priority arrays must ha_\x82\x01R\x01RV[a\x13\x18`2`@\x92a\x12\xB5V[a\x13!\x81a\x12\xBEV[\x01\x90V[a\x13:\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x13\x0BV[\x90V[\x15a\x13DWV[a\x13La\x02\x02V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x13b`\x04\x82\x01a\x13%V[\x03\x90\xFD[`\x01a\x13r\x91\x01a\x02_V[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x13\xD7W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x13\xD2W` \x01\x91`\x01\x82\x026\x03\x83\x13a\x13\xCDWV[a\x13\x91V[a\x13\x8DV[a\x13\x89V[\x90\x82\x10\x15a\x13\xF7W` a\x13\xF3\x92\x02\x81\x01\x90a\x13\x95V[\x90\x91V[a\x13uV[\x91\x90\x81\x10\x15a\x14\x0CW` \x02\x01\x90V[a\x13uV[5a\x14\x1B\x81a\x02bV[\x90V[\x90\x92a\x14+\x82\x85\x90a\x12\xADV[\x93a\x14R\x85a\x14La\x14Fa\x14A\x88\x87\x90a\x12\xB1V[a\x02_V[\x91a\x02_V[\x14a\x13=V[a\x14[_a\x11hV[[\x80a\x14oa\x14i\x88a\x02_V[\x91a\x02_V[\x10\x15a\x15\x16Wa\x14\x9D\x90a\x14\x9332\x90a\x14\x8B\x88\x87\x86\x91a\x13\xDCV[\x92\x90\x91a\x1A\xF0V[a\x14\xA2W[a\x13fV[a\x14\\V[3a\x14\xB8a\x14\xB2\x87\x86\x85\x91a\x13\xDCV[\x90a\x16\xBFV[\x90a\x14\xCDa\x14\xC8\x89\x88\x86\x91a\x13\xFCV[a\x14\x11V[B\x92a\x15\x0Ea\x14\xFC\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x10\x07V[\x94a\x15\x05a\x02\x02V[\x93\x84\x93\x84a\x10\x13V[\x03\x90\xA2a\x14\x98V[PPPPPPV[_\x90V[a\x15*a\x15\x1EV[Pa\x155`\x04a\x0EWV[\x90V[\x91\x90a\x15Ua\x15O32\x90\x86\x85\x91\x92\x90\x91\x92a\x1A\xF0V[\x15a\x03\x1FV[a\x15dWa\x15b\x92a\x15\xE3V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x15|`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[\x90\x82_\x93\x92\x827\x01RV[\x91\x90a\x15\xA5\x81a\x15\x9E\x81a\x15\xAA\x95a\x05\x9CV[\x80\x95a\x15\x80V[a\x05\xB0V[\x01\x90V[a\x15\xDAa\x15\xCF`@\x93a\x15\xE1\x96\x98\x97\x95``\x85\x01\x91\x85\x83\x03_\x87\x01Ra\x15\x8BV[\x96` \x83\x01\x90a\x06\xE7V[\x01\x90a\x06\xE7V[V[\x90\x913\x91\x92\x90\x92a\x16*Ba\x16\x18\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x95a\x10\x07V[\x95a\x16!a\x02\x02V[\x94\x85\x94\x85a\x15\xAEV[\x03\x90\xA2V[\x90a\x16:\x92\x91a\x158V[V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x16da\x16_a\x16i\x92a\x11eV[a\x16JV[a\x16AV[\x90V[\x90V[a\x16{a\x16\x80\x91a\x16AV[a\x16lV[\x90RV[\x90P\x90V[\x90\x91\x82a\x16\x99\x81a\x16\xA0\x93a\x16\x84V[\x80\x93a\x15\x80V[\x01\x90V[\x80a\x16\xB5`\x01\x92a\x16\xBC\x96\x94a\x16oV[\x01\x91a\x16\x89V[\x90V[a\x16\xFD\x90a\x16\xCBa\x16<V[Pa\x16\xEEa\x16\xD8_a\x16PV[\x91\x93a\x16\xE2a\x02\x02V[\x94\x85\x93` \x85\x01a\x16\xA4V[` \x82\x01\x81\x03\x82R\x03\x82a\x10\xBCV[\x90V[\x90a\x17\x1Ca\x17\x1632\x90\x85\x85\x91\x92\x90\x91\x92a\x1A\xF0V[\x15a\x03\x1FV[a\x17+Wa\x17)\x91a\x17\xA5V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x17C`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x17ja\x17p\x91\x93\x92\x93a\x02_V[\x92a\x02_V[\x82\x03\x91\x82\x11a\x17{WV[a\x17GV[a\x17\x8Fa\x17\x95\x91\x93\x92\x93a\x02_V[\x92a\x02_V[\x82\x01\x80\x92\x11a\x17\xA0WV[a\x17GV[a\x17\xC1a\x17\xCF\x91a\x17\xBAa\x17\xD4\x94Z\x92a\x17\xF0V[Z\x90a\x17[V[a\x17\xC9a\x0F:V[\x90a\x17\x80V[a!\x92V[V[\x90\x91a\x17\xED\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x15\x8BV[\x90V[3\x90\x91a\x18\x1D\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\x07V[\x92a\x182a\x18)a\x02\x02V[\x92\x83\x92\x83a\x17\xD6V[\x03\x90\xA2V[\x90a\x18A\x91a\x17\0V[V[\x90a\x18U\x91a\x18Pa\"BV[a\x19[V[V[`\xA0\x1C\x90V[a\x18ia\x18n\x91a\x18WV[a\x06=V[\x90V[a\x18{\x90Ta\x18]V[\x90V[a\x18\x92a\x18\x8Da\x18\x97\x92a\x11eV[a\x06\xB0V[a\x07rV[\x90V[a\x18\xA3\x90a\x18~V[\x90V[`\xA0\x1B\x90V[\x90a\x18\xBB`\xFF`\xA0\x1B\x91a\x18\xA6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18\xCE\x90a\x03\x1FV[\x90V[\x90V[\x90a\x18\xE9a\x18\xE4a\x18\xF0\x92a\x18\xC5V[a\x18\xD1V[\x82Ta\x18\xACV[\x90UV[a\x18\xFD\x90a\x08\xA0V[\x90V[a\x19\t\x90a\x18\xF4V[\x90V[_\x1B\x90V[\x90a\x19\"`\x01\x80`\xA0\x1B\x03\x91a\x19\x0CV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x195\x90a\x18\xF4V[\x90V[\x90V[\x90a\x19Pa\x19Ka\x19W\x92a\x19,V[a\x198V[\x82Ta\x19\x11V[\x90UV[a\x19e`\x01a\x18qV[a\x19\xCDW\x81a\x19\x84a\x19~a\x19y_a\x18\x9AV[a\x07}V[\x91a\x07}V[\x14a\x19\xB1Wa\x19\xAAa\x19\xA3a\x19\xAF\x93a\x19\x9E`\x01\x80a\x18\xD4V[a\x19\0V[`\x01a\x19;V[a!+V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x19\xC9`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x19\xE4`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[\x90a\x19\xF2\x91a\x18CV[V[a\x1A\x0Ba\x1A\x10\x91a\x1A\x03a\x11MV[P`\x05a\x0E\x07V[a\x12\tV[\x90V[a\x1A\x1Ba\"BV[a\x1A#a\x1A%V[V[a\x1A6a\x1A1_a\x18\x9AV[a\"\xB3V[V[a\x1A@a\x1A\x13V[V[a\x1ANa\x1AS\x91a\x0E\x1DV[a\x08`V[\x90V[a\x1A`\x90Ta\x1ABV[\x90V[`\xE0\x1B\x90V[a\x1Ar\x81a\x03\x1FV[\x03a\x1AyWV[_\x80\xFD[\x90PQ\x90a\x1A\x8A\x82a\x1AiV[V[\x90` \x82\x82\x03\x12a\x1A\xA5Wa\x1A\xA2\x91_\x01a\x1A}V[\x90V[a\x02\x0CV[a\x1A\xD0a\x1A\xDD\x95\x93\x94\x92\x94a\x1A\xC6``\x84\x01\x96_\x85\x01\x90a\x0C\x16V[` \x83\x01\x90a\x0C\x16V[`@\x81\x85\x03\x91\x01Ra\x15\x8BV[\x90V[a\x1A\xE8a\x02\x02V[=_\x82>=\x90\xFD[\x92a\x1B3` \x93\x94a\x1B\0a\x15\x1EV[Pa\x1B>a\x1B\x16a\x1B\x11`\x01a\x1AVV[a\x08\xC8V[\x93cz9y\xDC\x92\x95\x97a\x1B'a\x02\x02V[\x98\x89\x97\x88\x96\x87\x96a\x1AcV[\x86R`\x04\x86\x01a\x1A\xAAV[\x03\x91Z\xFA\x90\x81\x15a\x1B\x82W_\x91a\x1BTW[P\x90V[a\x1Bu\x91P` =\x81\x11a\x1B{W[a\x1Bm\x81\x83a\x10\xBCV[\x81\x01\x90a\x1A\x8CV[_a\x1BPV[P=a\x1BcV[a\x1A\xE0V[\x90a\x1B\xA3a\x1B\x9D32\x90\x85\x85\x91\x92\x90\x91\x92a\x1A\xF0V[\x15a\x03\x1FV[a\x1B\xB2Wa\x1B\xB0\x91a\x1B\xCEV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x1B\xCA`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[a\x1B\xEAa\x1B\xF8\x91a\x1B\xE3a\x1B\xFD\x94Z\x92a\x1B\xFFV[Z\x90a\x17[V[a\x1B\xF2a\x0F:V[\x90a\x17\x80V[a!\x92V[V[\x90a\x1C\x0B\x903\x92a\x16\xBFV[\x90a\x1CKa\x1C9\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\x07V[\x92a\x1CBa\x02\x02V[\x91\x82\x91\x82a\x05\xEBV[\x03\x90\xA2V[\x90a\x1CZ\x91a\x1B\x87V[V[_\x90V[a\x1Cj\x90Qa\x02_V[\x90V[a\x1Cua\x1C\\V[Pa\x1C\x89a\x1C\x83`\x04a\x0EWV[\x15a\x03\x1FV[a\x1C\xF9Wa\x1C\xC5a\x1C\xB7_a\x1C\xB1a\x1C\xAC`\x05a\x1C\xA6`\x03a\x0E6V[\x90a\x0E\x07V[a\x12\tV[\x01a\x1C`V[a\x1C\xBFa\t\xB7V[\x90a\x17\x80V[Ba\x1C\xD8a\x1C\xD2\x83a\x02_V[\x91a\x02_V[\x10\x15a\x1C\xECWa\x1C\xE9\x90B\x90a\x17[V[\x90V[Pa\x1C\xF6_a\x11hV[\x90V[a\x1D\x02_a\x11hV[\x90V[a\x1D\x14a\x1D\x1A\x91\x93\x92\x93a\x02_V[\x92a\x02_V[\x91a\x1D&\x83\x82\x02a\x02_V[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1D5WV[a\x17GV[\x91a\x1DCa\x1C\\V[P\x80a\x1DWa\x1DQ\x84a\x02_V[\x91a\x02_V[\x11\x15a\x1D\xABWa\x1Dx\x91a\x1Dj\x91a\x17[V[a\x1Dra\x06\xCFV[\x90a\x1D\x05V[\x80a\x1D\x8Ba\x1D\x85\x84a\x02_V[\x91a\x02_V[\x10\x15a\x1D\x9DWa\x1D\x9A\x91a\x17[V[\x90V[PPa\x1D\xA8_a\x11hV[\x90V[PP\x90V[a\x1D\xB8a\x1C\\V[Pa\x1D\xCCa\x1D\xC6`\x04a\x0EWV[\x15a\x03\x1FV[a\x1E\x06Wa\x1E\x03a\x1D\xF3`\x02a\x1D\xED`\x05a\x1D\xE7`\x03a\x0E6V[\x90a\x0E\x07V[\x01a\x0E6V[a\x1D\xFD`\x02a\x0E6V[\x90a\x1D\x05V[\x90V[a\x1E\x0F_a\x11hV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1E-a\x1E2\x91a\x0E\x1DV[a\x1E\x16V[\x90V[a\x1E?\x90Ta\x1E!V[\x90V[a\x1EJa\x1E\x12V[Pa\x1ET_a\x1E5V[\x90V[\x90V[a\x1Ena\x1Eia\x1Es\x92a\x1EWV[a\x06\xB0V[a\x02_V[\x90V[a\x1E~a\x1C\\V[Pa\x1E\x92a\x1E\x8C`\x04a\x0EWV[\x15a\x03\x1FV[a\x1E\xB6Wa\x1E\xB3a\x1E\xA3`\x03a\x0E6V[a\x1E\xAD`\x01a\x1EZV[\x90a\x17\x80V[\x90V[a\x1E\xBF_a\x11hV[\x90V[a\x1E\xCAa\x1C\\V[Pa\x1E\xDEa\x1E\xD8`\x04a\x0EWV[\x15a\x03\x1FV[a\x1F\x05Wa\x1F\x02`\x02a\x1E\xFC`\x05a\x1E\xF6`\x03a\x0E6V[\x90a\x0E\x07V[\x01a\x0E6V[\x90V[a\x1F\x0E_a\x11hV[\x90V[a\x1F-a\x1F;\x91a\x1F&a\x1F@\x94Z\x92a\x1FBV[Z\x90a\x17[V[a\x1F5a\x0F:V[\x90a\x17\x80V[a!\x92V[V[a\x1FM\x81\x83\x90a\x12\xADV[\x91a\x1FVa\x1C\\V[Pa\x1F`_a\x11hV[[\x80a\x1Fta\x1Fn\x86a\x02_V[\x91a\x02_V[\x10\x15a \x05Wa\x1F\xA2\x90a\x1F\x9832\x90a\x1F\x90\x87\x87\x86\x91a\x13\xDCV[\x92\x90\x91a\x1A\xF0V[a\x1F\xA7W[a\x13fV[a\x1FaV[3a\x1F\xBDa\x1F\xB7\x86\x86\x85\x91a\x13\xDCV[\x90a\x16\xBFV[\x90a\x1F\xFDa\x1F\xEB\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\x07V[\x92a\x1F\xF4a\x02\x02V[\x91\x82\x91\x82a\x05\xEBV[\x03\x90\xA2a\x1F\x9DV[PPPPV[\x90a \x15\x91a\x1F\x11V[V[a (\x90a #a\"BV[a *V[V[\x80a Ea ?a :_a\x18\x9AV[a\x07}V[\x91a\x07}V[\x14a \x9FWa ]a V\x82a\x19\0V[`\x01a\x19;V[a \x87\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10\x07V[\x90a \x90a\x02\x02V[\x80a \x9A\x81a\x02\xCAV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a \xB7`\x04\x82\x01a\x02\xCAV[\x03\x90\xFD[a \xC4\x90a \x17V[V[a \xD7\x90a \xD2a\"BV[a \xD9V[V[\x80a \xF4a \xEEa \xE9_a\x18\x9AV[a\x07}V[\x91a\x07}V[\x14a!\x04Wa!\x02\x90a\"\xB3V[V[a!'a!\x10_a\x18\x9AV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x0C#V[\x03\x90\xFD[a!4\x90a \xC6V[V[\x90a!B_\x19\x91a\x19\x0CV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a!da!_a!k\x92a\r\xEBV[a!LV[\x82Ta!6V[\x90UV[\x91` a!\x90\x92\x94\x93a!\x89`@\x82\x01\x96_\x83\x01\x90a\x06\xE7V[\x01\x90a\x06\xE7V[V[a!\xA5a!\x9F`\x04a\x0EWV[\x15a\x03\x1FV[a\"5W[a!\xB2a$\xDFV[a!\xE6\x81a!\xE0`\x02a!\xD0`\x05a!\xCA`\x03a\x0E6V[\x90a\x0E\x07V[\x01\x91a!\xDB\x83a\x0E6V[a\x17\x80V[\x90a!OV[a!\xF0`\x03a\x0E6V[:a\"\x1B\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\r\xEBV[\x92a\"0a\"'a\x02\x02V[\x92\x83\x92\x83a!oV[\x03\x90\xA2V[a\"=a#\xDCV[a!\xAAV[a\"Ja\x1EBV[a\"ca\"]a\"Xa&\xBDV[a\x07}V[\x91a\x07}V[\x03a\"jWV[a\"\x8Ca\"ua&\xBDV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\x0C#V[\x03\x90\xFD[\x90V[\x90a\"\xA8a\"\xA3a\"\xAF\x92a\x10\x07V[a\"\x90V[\x82Ta\x19\x11V[\x90UV[a\"\xBC_a\x1E5V[a\"\xC6\x82_a\"\x93V[\x90a\"\xFAa\"\xF4\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10\x07V[\x91a\x10\x07V[\x91a#\x03a\x02\x02V[\x80a#\r\x81a\x02\xCAV[\x03\x90\xA3V[\x90a#\x1E`\xFF\x91a\x19\x0CV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a#=a#8a#D\x92a\x18\xC5V[a\x18\xD1V[\x82Ta#\x12V[\x90UV[\x90a#R\x90a\x11hV[_R` R`@_ \x90V[a#h\x90Qa\x03\x1FV[\x90V[\x90a#\xC8```\x03a#\xCE\x94a#\x8E_\x82\x01a#\x88_\x88\x01a\x1C`V[\x90a!OV[a#\xA7`\x01\x82\x01a#\xA1` \x88\x01a\x1C`V[\x90a!OV[a#\xC0`\x02\x82\x01a#\xBA`@\x88\x01a\x1C`V[\x90a!OV[\x01\x92\x01a#^V[\x90a#(V[V[\x90a#\xDA\x91a#kV[V[a#\xEFa#\xE9`\x04a\x0EWV[\x15a\x03\x1FV[a#\xF6W[V[a$\x02`\x01`\x04a#(V[a$\x15a$\x0E_a\x11hV[`\x03a!OV[a$vBa$e_a$\\a$S_a$Na$E_\x95a$@a$7a\x11XV[\x99_\x8B\x01a\x11\x84V[a\x11hV[` \x88\x01a\x11\x84V[a\x11hV[`@\x85\x01a\x11\x84V[``\x83\x01a\x11\x92V[a$q`\x05_\x90a#HV[a#\xD0V[_B\x90a$\xB8a$\xA6\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x11hV[\x92a$\xAFa\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xA2a#\xF4V[\x90V[a$\xCC\x90a\x02_V[_\x19\x81\x14a$\xDAW`\x01\x01\x90V[a\x17GV[a$\xFCa$\xF7`\x05a$\xF1`\x03a\x0E6V[\x90a\x0E\x07V[a$\xC0V[Ba%*a%$a%\x1Fa%\x11_\x86\x01a\x0E6V[a%\x19a\t\xB7V[\x90a\x17\x80V[a\x02_V[\x91a\x02_V[\x10\x15a%4W[PV[a%\\a%Sa%E_\x84\x01a\x0E6V[a%Ma\t\xB7V[\x90a\x17\x80V[`\x01\x83\x01a!OV[a%j`\x01`\x03\x83\x01a#(V[a%t`\x03a\x0E6V[a%\xA1a%\x83`\x02\x84\x01a\x0E6V[\x92a%\x9B_a%\x94`\x01\x84\x01a\x0E6V[\x92\x01a\x0E6V[\x90a\x17[V[a%\xCB\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\r\xEBV[\x92a%\xE0a%\xD7a\x02\x02V[\x92\x83\x92\x83a!oV[\x03\x90\xA2a%\xFFa%\xF8a%\xF3`\x03a\x0E6V[a$\xC3V[`\x03a!OV[a&iBa&O_a&Fa&=_a&8a&/_\x95a&*a&!a\x11XV[\x99_\x8B\x01a\x11\x84V[a\x11hV[` \x88\x01a\x11\x84V[a\x11hV[`@\x85\x01a\x11\x84V[``\x83\x01a\x11\x92V[a&d`\x05a&^`\x03a\x0E6V[\x90a\x0E\x07V[a#\xD0V[a&s`\x03a\x0E6V[B\x90a&\xB4a&\xA2\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\r\xEBV[\x92a&\xABa\x02\x02V[\x91\x82\x91\x82a\x06\xF4V[\x03\x90\xA2_a%1V[a&\xC5a\x1E\x12V[P3\x90V",
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
    /**Function with signature `TRACKING_OVERHEAD()` and selector `0xede07bd6`.
```solidity
function TRACKING_OVERHEAD() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TRACKING_OVERHEADCall {}
    ///Container type for the return parameters of the [`TRACKING_OVERHEAD()`](TRACKING_OVERHEADCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TRACKING_OVERHEADReturn {
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
            impl ::core::convert::From<TRACKING_OVERHEADCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: TRACKING_OVERHEADCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TRACKING_OVERHEADCall {
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
            impl ::core::convert::From<TRACKING_OVERHEADReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: TRACKING_OVERHEADReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TRACKING_OVERHEADReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TRACKING_OVERHEADCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = TRACKING_OVERHEADReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TRACKING_OVERHEAD()";
            const SELECTOR: [u8; 4] = [237u8, 224u8, 123u8, 214u8];
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
        TRACKING_OVERHEAD(TRACKING_OVERHEADCall),
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
            [237u8, 224u8, 123u8, 214u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface
    for SyndicateSequencingChainWithDecayingPriorityCalls {
        const NAME: &'static str = "SyndicateSequencingChainWithDecayingPriorityCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 30usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::PERIOD_DURATION(_) => {
                    <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PRIORITY_DECAY_RATE(_) => {
                    <PRIORITY_DECAY_RATECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::TRACKING_OVERHEAD(_) => {
                    <TRACKING_OVERHEADCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn TRACKING_OVERHEAD(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <TRACKING_OVERHEADCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::TRACKING_OVERHEAD,
                            )
                    }
                    TRACKING_OVERHEAD
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
                Self::TRACKING_OVERHEAD(inner) => {
                    <TRACKING_OVERHEADCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::TRACKING_OVERHEAD(inner) => {
                    <TRACKING_OVERHEADCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        ///Creates a new call builder for the [`TRACKING_OVERHEAD`] function.
        pub fn TRACKING_OVERHEAD(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, TRACKING_OVERHEADCall, N> {
            self.call_builder(&TRACKING_OVERHEADCall {})
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
