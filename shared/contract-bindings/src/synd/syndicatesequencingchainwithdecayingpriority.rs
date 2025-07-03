///Module containing a contract's types and functions.
/**

```solidity
library GasCounter {
    struct GasPeriod { uint256 startTimestamp; uint256 endTimestamp; uint256 totalGasUsed; uint256 totalGasCost; }
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
struct GasPeriod { uint256 startTimestamp; uint256 endTimestamp; uint256 totalGasUsed; uint256 totalGasCost; }
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
        pub totalGasCost: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<GasPeriod> for UnderlyingRustTuple<'_> {
            fn from(value: GasPeriod) -> Self {
                (
                    value.startTimestamp,
                    value.endTimestamp,
                    value.totalGasUsed,
                    value.totalGasCost,
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
                    totalGasCost: tuple.3,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalGasCost),
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
                    "GasPeriod(uint256 startTimestamp,uint256 endTimestamp,uint256 totalGasUsed,uint256 totalGasCost)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.totalGasCost)
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
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalGasCost,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalGasCost,
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
        uint256 totalGasCost;
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
    function disableGasTracking() external;
    function enableGasTracking() external;
    function gasTrackingEnabled() external view returns (bool);
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
    function periods(uint256) external view returns (uint256 startTimestamp, uint256 endTimestamp, uint256 totalGasUsed, uint256 totalGasCost);
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
    "name": "disableGasTracking",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "enableGasTracking",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "gasTrackingEnabled",
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
            "name": "totalGasCost",
            "type": "uint256",
            "internalType": "uint256"
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
            "name": "totalGasCost",
            "type": "uint256",
            "internalType": "uint256"
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
        "name": "totalGasCost",
        "type": "uint256",
        "internalType": "uint256"
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
    ///0x60a060405234610038576100196100146100e9565b61010a565b61002161003d565b612961610521823960805181610ddb015261296190f35b610043565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b9061006f90610047565b810190811060018060401b0382111761008757604052565b610051565b9061009f61009861003d565b9283610065565b565b5f80fd5b90565b6100b1816100a5565b036100b857565b5f80fd5b905051906100c9826100a8565b565b906020828203126100e4576100e1915f016100bc565b90565b6100a1565b610107613041803803806100fc8161008c565b9283398101906100cb565b90565b610113906101c2565b565b90565b90565b61012f61012a61013492610115565b610118565b6100a5565b90565b60209181520190565b5f7f41707020636861696e2049442063616e6e6f7420626520300000000000000000910152565b6101746018602092610137565b61017d81610140565b0190565b6101969060208101905f818303910152610167565b90565b156101a057565b6101a861003d565b62461bcd60e51b8152806101be60048201610181565b0390fd5b6101ca61023d565b6101e7816101e06101da5f61011b565b916100a5565b1415610199565b608052565b60081b90565b906101ff61ff00916101ec565b9181191691161790565b151590565b61021790610209565b90565b90565b9061023261022d6102399261020e565b61021a565b82546101f2565b9055565b610245610335565b6102516001600361021d565b565b60a01b90565b9061026860ff60a01b91610253565b9181191691161790565b9061028761028261028e9261020e565b61021a565b8254610259565b9055565b5f0190565b61029f61003d565b3d5f823e3d90fd5b60018060a01b031690565b6102c66102c16102cb926102a7565b610118565b6102a7565b90565b6102d7906102b2565b90565b6102e3906102ce565b90565b5f1b90565b906102fc60018060a01b03916102e6565b9181191691161790565b61030f906102ce565b90565b90565b9061032a61032561033192610306565b610312565b82546102eb565b9055565b61033e336103a2565b6103495f6001610272565b61035161003d565b6101bf810181811060018060401b0382111761039d5761037982916101bf612e828439610292565b03905ff080156103985761038f610396916102da565b6001610315565b565b610297565b610051565b6103ab90610403565b565b6103c16103bc6103c692610115565b610118565b6102a7565b90565b6103d2906103ad565b90565b6103de906102a7565b90565b6103ea906103d5565b9052565b9190610401905f602085019401906103e1565b565b8061041e6104186104135f6103c9565b6103d5565b916103d5565b1461042e5761042c906104c1565b565b61045161043a5f6103c9565b5f918291631e4fbdf760e01b8352600483016103ee565b0390fd5b5f1c90565b60018060a01b031690565b61047161047691610455565b61045a565b90565b6104839054610465565b90565b61048f906102b2565b90565b61049b90610486565b90565b90565b906104b66104b16104bd92610492565b61049e565b82546102eb565b9055565b6104ca5f610479565b6104d4825f6104a1565b906105086105027f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093610492565b91610492565b9161051161003d565b8061051b81610292565b0390a356fe60806040526004361015610013575b611014565b61001d5f3561021c565b8063050ec13814610217578063086146d21461021257806311992f8c1461020d57806318d5aafe146102085780631c0b636714610203578063366cbab7146101fe5780633b6ab2a9146101f95780633d44ae8b146101f457806346e2cc09146101ef578063485cc955146101ea5780634b2c0706146101e55780635467cb48146101e05780635b3cd6e2146101db57806361543801146101d65780636558954f146101d1578063715018a6146101cc5780637a3979dc146101c7578063804e5123146101c257806382f44ade146101bd57806383d3c115146101b857806384fab62b146101b35780638d5a239b146101ae5780638da5cb5b146101a9578063aff74c6d146101a4578063c660d3f31461019f578063cdafb9781461019a578063d4f0eb4d14610195578063d878134214610190578063de1f453e1461018b578063ea4a110414610186578063ede07bd6146101815763f2fde38b0361000e57610fe1565b610fac565b610f3b565b610e32565b610dfd565b610da6565b610d54565b610ce9565b610cb4565b610c7f565b610c28565b610bf3565b610bad565b610b3e565b610b0a565b610ad1565b610a4c565b610a17565b6109a9565b61093c565b610873565b61083e565b6107ec565b610751565b61071c565b61068b565b610616565b610541565b61050c565b6104ae565b61039c565b6102ef565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f8301121561027a5781359167ffffffffffffffff831161027557602001926001830284011161027057565b61023c565b610238565b610234565b90565b61028b8161027f565b0361029257565b5f80fd5b905035906102a382610282565b565b916040838303126102e5575f83013567ffffffffffffffff81116102e0576102d2836102dd928601610240565b939094602001610296565b90565b610230565b61022c565b5f0190565b3461031e576103086103023660046102a5565b916110f4565b610310610222565b8061031a816102ea565b0390f35b610228565b5f91031261032d57565b61022c565b61033b9061027f565b9052565b90606080610385936103575f8201515f860190610332565b61036960208201516020860190610332565b61037b60408201516040860190610332565b0151910190610332565b565b919061039a905f6080850194019061033f565b565b346103cc576103ac366004610323565b6103c86103b76111ad565b6103bf610222565b91829182610387565b0390f35b610228565b909182601f8301121561040b5781359167ffffffffffffffff831161040657602001926020830284011161040157565b61023c565b610238565b610234565b909182601f8301121561044a5781359167ffffffffffffffff831161044557602001926020830284011161044057565b61023c565b610238565b610234565b90916040828403126104a9575f82013567ffffffffffffffff81116104a4578361047a9184016103d1565b929093602082013567ffffffffffffffff811161049f5761049b9201610410565b9091565b610230565b610230565b61022c565b346104e0576104ca6104c136600461044f565b92919091611351565b6104d2610222565b806104dc816102ea565b0390f35b610228565b151590565b6104f3906104e5565b9052565b919061050a905f602085019401906104ea565b565b3461053c5761051c366004610323565b610538610527611476565b61052f610222565b918291826104f7565b0390f35b610228565b346105705761055a6105543660046102a5565b91611583565b610562610222565b8061056c816102ea565b0390f35b610228565b906020828203126105a6575f82013567ffffffffffffffff81116105a15761059d9201610240565b9091565b610230565b61022c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6105ec6105f56020936105fa936105e3816105ab565b938480936105af565b958691016105b8565b6105c3565b0190565b6106139160208201915f8184039101526105cd565b90565b346106475761064361063261062c366004610575565b90611613565b61063a610222565b918291826105fe565b0390f35b610228565b1c90565b60ff1690565b61066690600861066b930261064c565b610650565b90565b906106799154610656565b90565b61068860035f9061066e565b90565b346106bb5761069b366004610323565b6106b76106a661067c565b6106ae610222565b918291826104f7565b0390f35b610228565b90565b90565b6106da6106d56106df926106c0565b6106c3565b61027f565b90565b6106ec600a6106c6565b90565b6106f76106e2565b90565b6107039061027f565b9052565b919061071a905f602085019401906106fa565b565b3461074c5761072c366004610323565b6107486107376106ef565b61073f610222565b91829182610707565b0390f35b610228565b346107805761076a610764366004610575565b906117d5565b610772610222565b8061077c816102ea565b0390f35b610228565b60018060a01b031690565b61079990610785565b90565b6107a581610790565b036107ac57565b5f80fd5b905035906107bd8261079c565b565b91906040838203126107e757806107db6107e4925f86016107b0565b936020016107b0565b90565b61022c565b3461081b576108056107ff3660046107bf565b90611986565b61080d610222565b80610817816102ea565b0390f35b610228565b9060208282031261083957610836915f01610296565b90565b61022c565b3461086e5761086a610859610854366004610820565b611a15565b610861610222565b91829182610387565b0390f35b610228565b346108a157610883366004610323565b61088b611a50565b610893610222565b8061089d816102ea565b0390f35b610228565b60018060a01b031690565b6108c19060086108c6930261064c565b6108a6565b90565b906108d491546108b1565b90565b6108e360015f906108c9565b90565b6108fa6108f56108ff92610785565b6106c3565b610785565b90565b61090b906108e6565b90565b61091790610902565b90565b6109239061090e565b9052565b919061093a905f6020850194019061091a565b565b3461096c5761094c366004610323565b6109686109576108d7565b61095f610222565b91829182610927565b0390f35b610228565b90565b610984906008610989930261064c565b610971565b90565b906109979154610974565b90565b6109a660025f9061098c565b90565b346109d9576109b9366004610323565b6109d56109c461099a565b6109cc610222565b91829182610707565b0390f35b610228565b90565b6109f56109f06109fa926109de565b6106c3565b61027f565b90565b610a0962278d006109e1565b90565b610a146109fd565b90565b34610a4757610a27366004610323565b610a43610a32610a0c565b610a3a610222565b91829182610707565b0390f35b610228565b34610a7a57610a5c366004610323565b610a64611a7f565b610a6c610222565b80610a76816102ea565b0390f35b610228565b91606083830312610acc57610a96825f85016107b0565b92610aa483602083016107b0565b92604082013567ffffffffffffffff8111610ac757610ac39201610240565b9091565b610230565b61022c565b34610b0557610b01610af0610ae7366004610a7f565b92919091611b37565b610af8610222565b918291826104f7565b0390f35b610228565b34610b3957610b23610b1d366004610575565b90611cba565b610b2b610222565b80610b35816102ea565b0390f35b610228565b34610b6e57610b4e366004610323565b610b6a610b59611cd7565b610b61610222565b91829182610707565b0390f35b610228565b9091606082840312610ba857610ba5610b8e845f8501610296565b93610b9c8160208601610296565b93604001610296565b90565b61022c565b34610bde57610bda610bc9610bc3366004610b73565b91611d9c565b610bd1610222565b91829182610707565b0390f35b610228565b610bf0600360019061066e565b90565b34610c2357610c03366004610323565b610c1f610c0e610be3565b610c16610222565b918291826104f7565b0390f35b610228565b34610c5857610c38366004610323565b610c54610c43611e12565b610c4b610222565b91829182610707565b0390f35b610228565b610c6690610790565b9052565b9190610c7d905f60208501940190610c5d565b565b34610caf57610c8f366004610323565b610cab610c9a611e61565b610ca2610222565b91829182610c6a565b0390f35b610228565b34610ce457610cc4366004610323565b610ce0610ccf611e95565b610cd7610222565b91829182610707565b0390f35b610228565b34610d1957610cf9366004610323565b610d15610d04611ee1565b610d0c610222565b91829182610707565b0390f35b610228565b90602082820312610d4f575f82013567ffffffffffffffff8111610d4a57610d4692016103d1565b9091565b610230565b61022c565b34610d8357610d6d610d67366004610d1e565b9061201d565b610d75610222565b80610d7f816102ea565b0390f35b610228565b90602082820312610da157610d9e915f016107b0565b90565b61022c565b34610dd457610dbe610db9366004610d88565b6120cd565b610dc6610222565b80610dd0816102ea565b0390f35b610228565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610e2d57610e0d366004610323565b610e29610e18610dd9565b610e20610222565b91829182610707565b0390f35b610228565b34610e6057610e42366004610323565b610e4a6120f4565b610e52610222565b80610e5c816102ea565b0390f35b610228565b610e79610e74610e7e9261027f565b6106c3565b61027f565b90565b90610e8b90610e65565b5f5260205260405f2090565b5f1c90565b610ea8610ead91610e97565b610971565b90565b610eba9054610e9c565b90565b610ec8906004610e81565b90610ed45f8301610eb0565b91610ee160018201610eb0565b91610efa6003610ef360028501610eb0565b9301610eb0565b90565b610f32610f3994610f28606094989795610f1e608086019a5f8701906106fa565b60208501906106fa565b60408301906106fa565b01906106fa565b565b34610f6f57610f6b610f56610f51366004610820565b610ebd565b90610f62949294610222565b94859485610efd565b0390f35b610228565b90565b610f8b610f86610f9092610f74565b6106c3565b61027f565b90565b610f9e611388610f77565b90565b610fa9610f93565b90565b34610fdc57610fbc366004610323565b610fd8610fc7610fa1565b610fcf610222565b91829182610707565b0390f35b610228565b3461100f57610ff9610ff4366004610d88565b612163565b611001610222565b8061100b816102ea565b0390f35b610228565b5f80fd5b919061103561102f33329086859192909192611b37565b156104e5565b61104457611042926110a1565b565b5f631b8e828b60e31b81528061105c600482016102ea565b0390fd5b61106990610902565b90565b60409061109861108d61109f9597969460608401908482035f8601526105cd565b9660208301906106fa565b01906106fa565b565b906110ad903392611613565b9142926110ef6110dd7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f294611060565b946110e6610222565b9384938461106c565b0390a2565b906110ff9291611018565b565b634e487b7160e01b5f52604160045260245ffd5b9061111f906105c3565b810190811067ffffffffffffffff82111761113957604052565b611101565b9061115161114a610222565b9283611115565b565b61115d608061113e565b90565b5f90565b61116c611153565b9060208080808561117b611160565b815201611186611160565b815201611191611160565b81520161119c611160565b81525050565b6111aa611164565b90565b6111b56111a2565b506111be6121b1565b90565b5090565b5090565b60209181520190565b60207f7665207468652073616d65206c656e6774680000000000000000000000000000917f4461746120616e64207072696f7269747920617272617973206d7573742068615f8201520152565b61122c60326040926111c9565b611235816111d2565b0190565b61124e9060208101905f81830391015261121f565b90565b1561125857565b611260610222565b62461bcd60e51b81528061127660048201611239565b0390fd5b90565b61129161128c6112969261127a565b6106c3565b61027f565b90565b60016112a5910161027f565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561130a570180359067ffffffffffffffff82116113055760200191600182023603831361130057565b6112c4565b6112c0565b6112bc565b9082101561132a57602061132692028101906112c8565b9091565b6112a8565b919081101561133f576020020190565b6112a8565b3561134e81610282565b90565b909261135e8285906111c1565b936113858561137f6113796113748887906111c5565b61027f565b9161027f565b14611251565b61138e5f61127d565b5b806113a261139c8861027f565b9161027f565b1015611449576113d0906113c63332906113be8887869161130f565b929091611b37565b6113d5575b611299565b61138f565b336113eb6113e58786859161130f565b90611613565b906114006113fb8988869161132f565b611344565b429261144161142f7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f294611060565b94611438610222565b9384938461106c565b0390a26113cb565b505050505050565b5f90565b61146161146691610e97565b610650565b90565b6114739054611455565b90565b61147e611451565b506114896003611469565b90565b91906114a96114a333329086859192909192611b37565b156104e5565b6114b8576114b692611537565b565b5f631b8e828b60e31b8152806114d0600482016102ea565b0390fd5b90825f939282370152565b91906114f9816114f2816114fe956105af565b80956114d4565b6105c3565b0190565b61152e6115236040936115359698979560608501918583035f8701526114df565b9660208301906106fa565b01906106fa565b565b9091339192909261157e4261156c7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f295611060565b95611575610222565b94859485611502565b0390a2565b9061158e929161148c565b565b606090565b60ff60f81b1690565b60f81b90565b6115b86115b36115bd9261127a565b61159e565b611595565b90565b90565b6115cf6115d491611595565b6115c0565b9052565b905090565b9091826115ed816115f4936115d8565b80936114d4565b0190565b8061160960019261161096946115c3565b01916115dd565b90565b6116519061161f611590565b5061164261162c5f6115a4565b9193611636610222565b948593602085016115f8565b60208201810382520382611115565b90565b9061167061166a33329085859192909192611b37565b156104e5565b61167f5761167d91611720565b565b5f631b8e828b60e31b815280611697600482016102ea565b0390fd5b60081c90565b6116ad6116b29161169b565b610650565b90565b6116bf90546116a1565b90565b634e487b7160e01b5f52601160045260245ffd5b6116e56116eb9193929361027f565b9261027f565b82039182116116f657565b6116c2565b61170a6117109193929361027f565b9261027f565b820180921161171b57565b6116c2565b9061173461172e60036116b5565b156104e5565b611769576117546117679261174d611762935a9261178e565b5a906116d6565b61175c610f93565b906116fb565b612385565b565b6117729161178e565b565b909161178b9260208301925f8185039101526114df565b90565b3390916117bb7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611060565b926117d06117c7610222565b92839283611774565b0390a2565b906117df91611654565b565b906117f3916117ee61248c565b6118f9565b565b60a01c90565b61180761180c916117f5565b610650565b90565b61181990546117fb565b90565b61183061182b6118359261127a565b6106c3565b610785565b90565b6118419061181c565b90565b60a01b90565b9061185960ff60a01b91611844565b9181191691161790565b61186c906104e5565b90565b90565b9061188761188261188e92611863565b61186f565b825461184a565b9055565b61189b906108e6565b90565b6118a790611892565b90565b5f1b90565b906118c060018060a01b03916118aa565b9181191691161790565b6118d390611892565b90565b90565b906118ee6118e96118f5926118ca565b6118d6565b82546118af565b9055565b611903600161180f565b61196b578161192261191c6119175f611838565b610790565b91610790565b1461194f5761194861194161194d9361193c600180611872565b61189e565b60016118d9565b612163565b565b5f632e7f3c7f60e11b815280611967600482016102ea565b0390fd5b5f62dc149f60e41b815280611982600482016102ea565b0390fd5b90611990916117e1565b565b9061199c9061027f565b9052565b90611a076119fe60036119b1611153565b946119c86119c05f8301610eb0565b5f8801611992565b6119e06119d760018301610eb0565b60208801611992565b6119f86119ef60028301610eb0565b60408801611992565b01610eb0565b60608401611992565b565b611a12906119a0565b90565b611a2c611a3191611a246111a2565b506004610e81565b611a09565b90565b611a3c61248c565b611a44611a46565b565b611a4e612517565b565b611a58611a34565b565b611a6261248c565b611a6a611a6c565b565b611a7d611a785f611838565b612547565b565b611a87611a5a565b565b611a95611a9a91610e97565b6108a6565b90565b611aa79054611a89565b90565b60e01b90565b611ab9816104e5565b03611ac057565b5f80fd5b90505190611ad182611ab0565b565b90602082820312611aec57611ae9915f01611ac4565b90565b61022c565b611b17611b249593949294611b0d60608401965f850190610c5d565b6020830190610c5d565b60408185039101526114df565b90565b611b2f610222565b3d5f823e3d90fd5b92611b7a60209394611b47611451565b50611b85611b5d611b586001611a9d565b61090e565b93637a3979dc929597611b6e610222565b98899788968796611aaa565b865260048601611af1565b03915afa908115611bc9575f91611b9b575b5090565b611bbc915060203d8111611bc2575b611bb48183611115565b810190611ad3565b5f611b97565b503d611baa565b611b27565b90611bea611be433329085859192909192611b37565b156104e5565b611bf957611bf791611c15565b565b5f631b8e828b60e31b815280611c11600482016102ea565b0390fd5b90611c29611c2360036116b5565b156104e5565b611c5e57611c49611c5c92611c42611c57935a92611c69565b5a906116d6565b611c51610f93565b906116fb565b612385565b565b611c6791611c69565b565b90611c75903392611613565b90611cb5611ca37f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611060565b92611cac610222565b918291826105fe565b0390a2565b90611cc491611bce565b565b5f90565b611cd4905161027f565b90565b611cdf611cc6565b50611ce86121b1565b611cf35f8201611cca565b611d05611cff5f61127d565b9161027f565b14611d5a57611d185f611d269201611cca565b611d206109fd565b906116fb565b42611d39611d338361027f565b9161027f565b1015611d4d57611d4a9042906116d6565b90565b50611d575f61127d565b90565b50611d645f61127d565b90565b611d76611d7c9193929361027f565b9261027f565b91611d8883820261027f565b928184041490151715611d9757565b6116c2565b91611da5611cc6565b5080611db9611db38461027f565b9161027f565b1115611e0d57611dda91611dcc916116d6565b611dd46106e2565b90611d67565b80611ded611de78461027f565b9161027f565b1015611dff57611dfc916116d6565b90565b5050611e0a5f61127d565b90565b505090565b611e1a611cc6565b50611e2e6060611e286121b1565b01611cca565b90565b5f90565b60018060a01b031690565b611e4c611e5191610e97565b611e35565b90565b611e5e9054611e40565b90565b611e69611e31565b50611e735f611e54565b90565b90565b611e8d611e88611e9292611e76565b6106c3565b61027f565b90565b611e9d611cc6565b50611eb1611eab6003611469565b156104e5565b611ed557611ed2611ec26002610eb0565b611ecc6001611e79565b906116fb565b90565b611ede5f61127d565b90565b611ee9611cc6565b50611efd6040611ef76121b1565b01611cca565b90565b90611f14611f0e60036116b5565b156104e5565b611f4957611f34611f4792611f2d611f42935a92611f54565b5a906116d6565b611f3c610f93565b906116fb565b612385565b565b611f5291611f54565b565b611f5f8183906111c1565b91611f68611cc6565b50611f725f61127d565b5b80611f86611f808661027f565b9161027f565b101561201757611fb490611faa333290611fa28787869161130f565b929091611b37565b611fb9575b611299565b611f73565b33611fcf611fc98686859161130f565b90611613565b9061200f611ffd7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611060565b92612006610222565b918291826105fe565b0390a2611faf565b50505050565b9061202791611f00565b565b61203a9061203561248c565b61203c565b565b8061205761205161204c5f611838565b610790565b91610790565b146120b15761206f6120688261189e565b60016118d9565b6120997f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b991611060565b906120a2610222565b806120ac816102ea565b0390a2565b5f632e7f3c7f60e11b8152806120c9600482016102ea565b0390fd5b6120d690612029565b565b6120e061248c565b6120e86120ea565b565b6120f26125a6565b565b6120fc6120d8565b565b61210f9061210a61248c565b612111565b565b8061212c6121266121215f611838565b610790565b91610790565b1461213c5761213a90612547565b565b61215f6121485f611838565b5f918291631e4fbdf760e01b835260048301610c6a565b0390fd5b61216c906120fe565b565b612178608061113e565b90565b634e487b7160e01b5f52601260045260245ffd5b61219b6121a19161027f565b9161027f565b9081156121ac570490565b61217b565b6121b96111a2565b506121cd6121c76003611469565b156104e5565b6122c9576121ee6121e960046121e36002610eb0565b90610e81565b611a09565b4261221c6122166122116122035f8601611cca565b61220b6109fd565b906116fb565b61027f565b9161027f565b10156122255790565b6122729061226c61225d5f61225661224842612242848801611cca565b906116d6565b6122506109fd565b9061218f565b9301611cca565b916122666109fd565b90611d67565b906116fb565b6122c66122bd5f6122b86122af5f6122aa6122a15f9561229c61229361216e565b9a5f8c01611992565b61127d565b60208901611992565b61127d565b60408601611992565b61127d565b60608301611992565b90565b5f61232661231d5f61231861230f5f61230a6123015f956122fc6122f46122ee61216e565b9b61127d565b5f8c01611992565b61127d565b60208901611992565b61127d565b60408601611992565b61127d565b60608301611992565b90565b906123355f19916118aa565b9181191691161790565b90565b9061235761235261235e92610e65565b61233f565b8254612329565b9055565b91602061238392949361237c60408201965f8301906106fa565b01906106fa565b565b61239861239260036116b5565b156104e5565b612489576123af6123a96003611469565b156104e5565b61247c575b6123bc61277c565b61242d6123ca823a90611d67565b6123fd836123f760026123e760046123e183610eb0565b90610e81565b01916123f283610eb0565b6116fb565b90612342565b612427600361241760046124116002610eb0565b90610e81565b019161242283610eb0565b6116fb565b90612342565b6124376002610eb0565b3a6124627f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610e65565b9261247761246e610222565b92839283612362565b0390a2565b612484612671565b6123b4565b50565b612494611e61565b6124ad6124a76124a2612954565b610790565b91610790565b036124b457565b6124d66124bf612954565b5f91829163118cdaa760e01b835260048301610c6a565b0390fd5b60081b90565b906124ed61ff00916124da565b9181191691161790565b9061250c61250761251392611863565b61186f565b82546124e0565b9055565b6125225f60036124f7565b565b90565b9061253c61253761254392611060565b612524565b82546118af565b9055565b6125505f611e54565b61255a825f612527565b9061258e6125887f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093611060565b91611060565b91612597610222565b806125a1816102ea565b0390a3565b6125b2600160036124f7565b565b906125c060ff916118aa565b9181191691161790565b906125df6125da6125e692611863565b61186f565b82546125b4565b9055565b906125f49061127d565b5f5260205260405f2090565b9061265d60606003612663946126235f820161261d5f8801611cca565b90612342565b61263c6001820161263660208801611cca565b90612342565b6126556002820161264f60408801611cca565b90612342565b019201611cca565b90612342565b565b9061266f91612600565b565b61268461267e6003611469565b156104e5565b61268b575b565b612697600160036125ca565b6126aa6126a35f61127d565b6002612342565b612713426127026126f95f6126f46126eb5f6126e66126dd5f956126d86126cf61216e565b9a5f8c01611992565b61127d565b60208901611992565b61127d565b60408601611992565b61127d565b60608301611992565b61270e60045f906125ea565b612665565b5f42906127556127437f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e9261127d565b9261274c610222565b91829182610707565b0390a2612689565b90565b6127699061027f565b5f1981146127775760010190565b6116c2565b612799612794600461278e6002610eb0565b90610e81565b61275d565b426127c76127c16127bc6127ae5f8601610eb0565b6127b66109fd565b906116fb565b61027f565b9161027f565b10156127d1575b50565b6127f96127f06127e25f8401610eb0565b6127ea6109fd565b906116fb565b60018301612342565b6128036002610eb0565b61283061281260028401610eb0565b9261282a5f61282360018401610eb0565b9201610eb0565b906116d6565b61285a7f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610e65565b9261286f612866610222565b92839283612362565b0390a261288e6128876128826002610eb0565b612760565b6002612342565b612900426128e66128dd5f6128d86128cf5f6128ca6128c15f956128bc6128b361216e565b9a5f8c01611992565b61127d565b60208901611992565b61127d565b60408601611992565b61127d565b60608301611992565b6128fb60046128f56002610eb0565b90610e81565b612665565b61290a6002610eb0565b429061294b6129397f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610e65565b92612942610222565b91829182610707565b0390a25f6127ce565b61295c611e31565b50339056608060405234601c57600e6020565b61019461002b823961019490f35b6026565b60405190565b5f80fdfe60806040526004361015610013575b61017b565b61001d5f3561002c565b637a3979dc0361000e57610142565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b60018060a01b031690565b61005890610044565b90565b6100648161004f565b0361006b57565b5f80fd5b9050359061007c8261005b565b565b5f80fd5b5f80fd5b5f80fd5b909182601f830112156100c45781359167ffffffffffffffff83116100bf5760200192600183028401116100ba57565b610086565b610082565b61007e565b91606083830312610116576100e0825f850161006f565b926100ee836020830161006f565b92604082013567ffffffffffffffff81116101115761010d920161008a565b9091565b610040565b61003c565b151590565b6101299061011b565b9052565b9190610140905f60208501940190610120565b565b34610176576101726101616101583660046100c9565b92919091610183565b610169610032565b9182918261012d565b0390f35b610038565b5f80fd5b5f90565b5050505061018f61017f565b505f9056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4a\08Wa\0\x19a\0\x14a\0\xE9V[a\x01\nV[a\0!a\0=V[a)aa\x05!\x829`\x80Q\x81a\r\xDB\x01Ra)a\x90\xF3[a\0CV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0o\x90a\0GV[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x87W`@RV[a\0QV[\x90a\0\x9Fa\0\x98a\0=V[\x92\x83a\0eV[V[_\x80\xFD[\x90V[a\0\xB1\x81a\0\xA5V[\x03a\0\xB8WV[_\x80\xFD[\x90PQ\x90a\0\xC9\x82a\0\xA8V[V[\x90` \x82\x82\x03\x12a\0\xE4Wa\0\xE1\x91_\x01a\0\xBCV[\x90V[a\0\xA1V[a\x01\x07a0A\x808\x03\x80a\0\xFC\x81a\0\x8CV[\x92\x839\x81\x01\x90a\0\xCBV[\x90V[a\x01\x13\x90a\x01\xC2V[V[\x90V[\x90V[a\x01/a\x01*a\x014\x92a\x01\x15V[a\x01\x18V[a\0\xA5V[\x90V[` \x91\x81R\x01\x90V[_\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01t`\x18` \x92a\x017V[a\x01}\x81a\x01@V[\x01\x90V[a\x01\x96\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x01gV[\x90V[\x15a\x01\xA0WV[a\x01\xA8a\0=V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x01\xBE`\x04\x82\x01a\x01\x81V[\x03\x90\xFD[a\x01\xCAa\x02=V[a\x01\xE7\x81a\x01\xE0a\x01\xDA_a\x01\x1BV[\x91a\0\xA5V[\x14\x15a\x01\x99V[`\x80RV[`\x08\x1B\x90V[\x90a\x01\xFFa\xFF\0\x91a\x01\xECV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x15\x15\x90V[a\x02\x17\x90a\x02\tV[\x90V[\x90V[\x90a\x022a\x02-a\x029\x92a\x02\x0EV[a\x02\x1AV[\x82Ta\x01\xF2V[\x90UV[a\x02Ea\x035V[a\x02Q`\x01`\x03a\x02\x1DV[V[`\xA0\x1B\x90V[\x90a\x02h`\xFF`\xA0\x1B\x91a\x02SV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x02\x87a\x02\x82a\x02\x8E\x92a\x02\x0EV[a\x02\x1AV[\x82Ta\x02YV[\x90UV[_\x01\x90V[a\x02\x9Fa\0=V[=_\x82>=\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x02\xC6a\x02\xC1a\x02\xCB\x92a\x02\xA7V[a\x01\x18V[a\x02\xA7V[\x90V[a\x02\xD7\x90a\x02\xB2V[\x90V[a\x02\xE3\x90a\x02\xCEV[\x90V[_\x1B\x90V[\x90a\x02\xFC`\x01\x80`\xA0\x1B\x03\x91a\x02\xE6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x03\x0F\x90a\x02\xCEV[\x90V[\x90V[\x90a\x03*a\x03%a\x031\x92a\x03\x06V[a\x03\x12V[\x82Ta\x02\xEBV[\x90UV[a\x03>3a\x03\xA2V[a\x03I_`\x01a\x02rV[a\x03Qa\0=V[a\x01\xBF\x81\x01\x81\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\x03\x9DWa\x03y\x82\x91a\x01\xBFa.\x82\x849a\x02\x92V[\x03\x90_\xF0\x80\x15a\x03\x98Wa\x03\x8Fa\x03\x96\x91a\x02\xDAV[`\x01a\x03\x15V[V[a\x02\x97V[a\0QV[a\x03\xAB\x90a\x04\x03V[V[a\x03\xC1a\x03\xBCa\x03\xC6\x92a\x01\x15V[a\x01\x18V[a\x02\xA7V[\x90V[a\x03\xD2\x90a\x03\xADV[\x90V[a\x03\xDE\x90a\x02\xA7V[\x90V[a\x03\xEA\x90a\x03\xD5V[\x90RV[\x91\x90a\x04\x01\x90_` \x85\x01\x94\x01\x90a\x03\xE1V[V[\x80a\x04\x1Ea\x04\x18a\x04\x13_a\x03\xC9V[a\x03\xD5V[\x91a\x03\xD5V[\x14a\x04.Wa\x04,\x90a\x04\xC1V[V[a\x04Qa\x04:_a\x03\xC9V[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x03\xEEV[\x03\x90\xFD[_\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04qa\x04v\x91a\x04UV[a\x04ZV[\x90V[a\x04\x83\x90Ta\x04eV[\x90V[a\x04\x8F\x90a\x02\xB2V[\x90V[a\x04\x9B\x90a\x04\x86V[\x90V[\x90V[\x90a\x04\xB6a\x04\xB1a\x04\xBD\x92a\x04\x92V[a\x04\x9EV[\x82Ta\x02\xEBV[\x90UV[a\x04\xCA_a\x04yV[a\x04\xD4\x82_a\x04\xA1V[\x90a\x05\x08a\x05\x02\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x04\x92V[\x91a\x04\x92V[\x91a\x05\x11a\0=V[\x80a\x05\x1B\x81a\x02\x92V[\x03\x90\xA3V\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x10\x14V[a\0\x1D_5a\x02\x1CV[\x80c\x05\x0E\xC18\x14a\x02\x17W\x80c\x08aF\xD2\x14a\x02\x12W\x80c\x11\x99/\x8C\x14a\x02\rW\x80c\x18\xD5\xAA\xFE\x14a\x02\x08W\x80c\x1C\x0Bcg\x14a\x02\x03W\x80c6l\xBA\xB7\x14a\x01\xFEW\x80c;j\xB2\xA9\x14a\x01\xF9W\x80c=D\xAE\x8B\x14a\x01\xF4W\x80cF\xE2\xCC\t\x14a\x01\xEFW\x80cH\\\xC9U\x14a\x01\xEAW\x80cK,\x07\x06\x14a\x01\xE5W\x80cTg\xCBH\x14a\x01\xE0W\x80c[<\xD6\xE2\x14a\x01\xDBW\x80caT8\x01\x14a\x01\xD6W\x80ceX\x95O\x14a\x01\xD1W\x80cqP\x18\xA6\x14a\x01\xCCW\x80cz9y\xDC\x14a\x01\xC7W\x80c\x80NQ#\x14a\x01\xC2W\x80c\x82\xF4J\xDE\x14a\x01\xBDW\x80c\x83\xD3\xC1\x15\x14a\x01\xB8W\x80c\x84\xFA\xB6+\x14a\x01\xB3W\x80c\x8DZ#\x9B\x14a\x01\xAEW\x80c\x8D\xA5\xCB[\x14a\x01\xA9W\x80c\xAF\xF7Lm\x14a\x01\xA4W\x80c\xC6`\xD3\xF3\x14a\x01\x9FW\x80c\xCD\xAF\xB9x\x14a\x01\x9AW\x80c\xD4\xF0\xEBM\x14a\x01\x95W\x80c\xD8x\x13B\x14a\x01\x90W\x80c\xDE\x1FE>\x14a\x01\x8BW\x80c\xEAJ\x11\x04\x14a\x01\x86W\x80c\xED\xE0{\xD6\x14a\x01\x81Wc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\x0F\xE1V[a\x0F\xACV[a\x0F;V[a\x0E2V[a\r\xFDV[a\r\xA6V[a\rTV[a\x0C\xE9V[a\x0C\xB4V[a\x0C\x7FV[a\x0C(V[a\x0B\xF3V[a\x0B\xADV[a\x0B>V[a\x0B\nV[a\n\xD1V[a\nLV[a\n\x17V[a\t\xA9V[a\t<V[a\x08sV[a\x08>V[a\x07\xECV[a\x07QV[a\x07\x1CV[a\x06\x8BV[a\x06\x16V[a\x05AV[a\x05\x0CV[a\x04\xAEV[a\x03\x9CV[a\x02\xEFV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x02zW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x02uW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x02pWV[a\x02<V[a\x028V[a\x024V[\x90V[a\x02\x8B\x81a\x02\x7FV[\x03a\x02\x92WV[_\x80\xFD[\x90P5\x90a\x02\xA3\x82a\x02\x82V[V[\x91`@\x83\x83\x03\x12a\x02\xE5W_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02\xE0Wa\x02\xD2\x83a\x02\xDD\x92\x86\x01a\x02@V[\x93\x90\x94` \x01a\x02\x96V[\x90V[a\x020V[a\x02,V[_\x01\x90V[4a\x03\x1EWa\x03\x08a\x03\x026`\x04a\x02\xA5V[\x91a\x10\xF4V[a\x03\x10a\x02\"V[\x80a\x03\x1A\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[_\x91\x03\x12a\x03-WV[a\x02,V[a\x03;\x90a\x02\x7FV[\x90RV[\x90``\x80a\x03\x85\x93a\x03W_\x82\x01Q_\x86\x01\x90a\x032V[a\x03i` \x82\x01Q` \x86\x01\x90a\x032V[a\x03{`@\x82\x01Q`@\x86\x01\x90a\x032V[\x01Q\x91\x01\x90a\x032V[V[\x91\x90a\x03\x9A\x90_`\x80\x85\x01\x94\x01\x90a\x03?V[V[4a\x03\xCCWa\x03\xAC6`\x04a\x03#V[a\x03\xC8a\x03\xB7a\x11\xADV[a\x03\xBFa\x02\"V[\x91\x82\x91\x82a\x03\x87V[\x03\x90\xF3[a\x02(V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04\x0BW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04\x06W` \x01\x92` \x83\x02\x84\x01\x11a\x04\x01WV[a\x02<V[a\x028V[a\x024V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04JW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04EW` \x01\x92` \x83\x02\x84\x01\x11a\x04@WV[a\x02<V[a\x028V[a\x024V[\x90\x91`@\x82\x84\x03\x12a\x04\xA9W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xA4W\x83a\x04z\x91\x84\x01a\x03\xD1V[\x92\x90\x93` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x9FWa\x04\x9B\x92\x01a\x04\x10V[\x90\x91V[a\x020V[a\x020V[a\x02,V[4a\x04\xE0Wa\x04\xCAa\x04\xC16`\x04a\x04OV[\x92\x91\x90\x91a\x13QV[a\x04\xD2a\x02\"V[\x80a\x04\xDC\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[\x15\x15\x90V[a\x04\xF3\x90a\x04\xE5V[\x90RV[\x91\x90a\x05\n\x90_` \x85\x01\x94\x01\x90a\x04\xEAV[V[4a\x05<Wa\x05\x1C6`\x04a\x03#V[a\x058a\x05'a\x14vV[a\x05/a\x02\"V[\x91\x82\x91\x82a\x04\xF7V[\x03\x90\xF3[a\x02(V[4a\x05pWa\x05Za\x05T6`\x04a\x02\xA5V[\x91a\x15\x83V[a\x05ba\x02\"V[\x80a\x05l\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[\x90` \x82\x82\x03\x12a\x05\xA6W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05\xA1Wa\x05\x9D\x92\x01a\x02@V[\x90\x91V[a\x020V[a\x02,V[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x05\xECa\x05\xF5` \x93a\x05\xFA\x93a\x05\xE3\x81a\x05\xABV[\x93\x84\x80\x93a\x05\xAFV[\x95\x86\x91\x01a\x05\xB8V[a\x05\xC3V[\x01\x90V[a\x06\x13\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x05\xCDV[\x90V[4a\x06GWa\x06Ca\x062a\x06,6`\x04a\x05uV[\x90a\x16\x13V[a\x06:a\x02\"V[\x91\x82\x91\x82a\x05\xFEV[\x03\x90\xF3[a\x02(V[\x1C\x90V[`\xFF\x16\x90V[a\x06f\x90`\x08a\x06k\x93\x02a\x06LV[a\x06PV[\x90V[\x90a\x06y\x91Ta\x06VV[\x90V[a\x06\x88`\x03_\x90a\x06nV[\x90V[4a\x06\xBBWa\x06\x9B6`\x04a\x03#V[a\x06\xB7a\x06\xA6a\x06|V[a\x06\xAEa\x02\"V[\x91\x82\x91\x82a\x04\xF7V[\x03\x90\xF3[a\x02(V[\x90V[\x90V[a\x06\xDAa\x06\xD5a\x06\xDF\x92a\x06\xC0V[a\x06\xC3V[a\x02\x7FV[\x90V[a\x06\xEC`\na\x06\xC6V[\x90V[a\x06\xF7a\x06\xE2V[\x90V[a\x07\x03\x90a\x02\x7FV[\x90RV[\x91\x90a\x07\x1A\x90_` \x85\x01\x94\x01\x90a\x06\xFAV[V[4a\x07LWa\x07,6`\x04a\x03#V[a\x07Ha\x077a\x06\xEFV[a\x07?a\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[4a\x07\x80Wa\x07ja\x07d6`\x04a\x05uV[\x90a\x17\xD5V[a\x07ra\x02\"V[\x80a\x07|\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x07\x99\x90a\x07\x85V[\x90V[a\x07\xA5\x81a\x07\x90V[\x03a\x07\xACWV[_\x80\xFD[\x90P5\x90a\x07\xBD\x82a\x07\x9CV[V[\x91\x90`@\x83\x82\x03\x12a\x07\xE7W\x80a\x07\xDBa\x07\xE4\x92_\x86\x01a\x07\xB0V[\x93` \x01a\x07\xB0V[\x90V[a\x02,V[4a\x08\x1BWa\x08\x05a\x07\xFF6`\x04a\x07\xBFV[\x90a\x19\x86V[a\x08\ra\x02\"V[\x80a\x08\x17\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[\x90` \x82\x82\x03\x12a\x089Wa\x086\x91_\x01a\x02\x96V[\x90V[a\x02,V[4a\x08nWa\x08ja\x08Ya\x08T6`\x04a\x08 V[a\x1A\x15V[a\x08aa\x02\"V[\x91\x82\x91\x82a\x03\x87V[\x03\x90\xF3[a\x02(V[4a\x08\xA1Wa\x08\x836`\x04a\x03#V[a\x08\x8Ba\x1APV[a\x08\x93a\x02\"V[\x80a\x08\x9D\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08\xC1\x90`\x08a\x08\xC6\x93\x02a\x06LV[a\x08\xA6V[\x90V[\x90a\x08\xD4\x91Ta\x08\xB1V[\x90V[a\x08\xE3`\x01_\x90a\x08\xC9V[\x90V[a\x08\xFAa\x08\xF5a\x08\xFF\x92a\x07\x85V[a\x06\xC3V[a\x07\x85V[\x90V[a\t\x0B\x90a\x08\xE6V[\x90V[a\t\x17\x90a\t\x02V[\x90V[a\t#\x90a\t\x0EV[\x90RV[\x91\x90a\t:\x90_` \x85\x01\x94\x01\x90a\t\x1AV[V[4a\tlWa\tL6`\x04a\x03#V[a\tha\tWa\x08\xD7V[a\t_a\x02\"V[\x91\x82\x91\x82a\t'V[\x03\x90\xF3[a\x02(V[\x90V[a\t\x84\x90`\x08a\t\x89\x93\x02a\x06LV[a\tqV[\x90V[\x90a\t\x97\x91Ta\ttV[\x90V[a\t\xA6`\x02_\x90a\t\x8CV[\x90V[4a\t\xD9Wa\t\xB96`\x04a\x03#V[a\t\xD5a\t\xC4a\t\x9AV[a\t\xCCa\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[\x90V[a\t\xF5a\t\xF0a\t\xFA\x92a\t\xDEV[a\x06\xC3V[a\x02\x7FV[\x90V[a\n\tb'\x8D\0a\t\xE1V[\x90V[a\n\x14a\t\xFDV[\x90V[4a\nGWa\n'6`\x04a\x03#V[a\nCa\n2a\n\x0CV[a\n:a\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[4a\nzWa\n\\6`\x04a\x03#V[a\nda\x1A\x7FV[a\nla\x02\"V[\x80a\nv\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[\x91``\x83\x83\x03\x12a\n\xCCWa\n\x96\x82_\x85\x01a\x07\xB0V[\x92a\n\xA4\x83` \x83\x01a\x07\xB0V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xC7Wa\n\xC3\x92\x01a\x02@V[\x90\x91V[a\x020V[a\x02,V[4a\x0B\x05Wa\x0B\x01a\n\xF0a\n\xE76`\x04a\n\x7FV[\x92\x91\x90\x91a\x1B7V[a\n\xF8a\x02\"V[\x91\x82\x91\x82a\x04\xF7V[\x03\x90\xF3[a\x02(V[4a\x0B9Wa\x0B#a\x0B\x1D6`\x04a\x05uV[\x90a\x1C\xBAV[a\x0B+a\x02\"V[\x80a\x0B5\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[4a\x0BnWa\x0BN6`\x04a\x03#V[a\x0Bja\x0BYa\x1C\xD7V[a\x0Baa\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[\x90\x91``\x82\x84\x03\x12a\x0B\xA8Wa\x0B\xA5a\x0B\x8E\x84_\x85\x01a\x02\x96V[\x93a\x0B\x9C\x81` \x86\x01a\x02\x96V[\x93`@\x01a\x02\x96V[\x90V[a\x02,V[4a\x0B\xDEWa\x0B\xDAa\x0B\xC9a\x0B\xC36`\x04a\x0BsV[\x91a\x1D\x9CV[a\x0B\xD1a\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[a\x0B\xF0`\x03`\x01\x90a\x06nV[\x90V[4a\x0C#Wa\x0C\x036`\x04a\x03#V[a\x0C\x1Fa\x0C\x0Ea\x0B\xE3V[a\x0C\x16a\x02\"V[\x91\x82\x91\x82a\x04\xF7V[\x03\x90\xF3[a\x02(V[4a\x0CXWa\x0C86`\x04a\x03#V[a\x0CTa\x0CCa\x1E\x12V[a\x0CKa\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[a\x0Cf\x90a\x07\x90V[\x90RV[\x91\x90a\x0C}\x90_` \x85\x01\x94\x01\x90a\x0C]V[V[4a\x0C\xAFWa\x0C\x8F6`\x04a\x03#V[a\x0C\xABa\x0C\x9Aa\x1EaV[a\x0C\xA2a\x02\"V[\x91\x82\x91\x82a\x0CjV[\x03\x90\xF3[a\x02(V[4a\x0C\xE4Wa\x0C\xC46`\x04a\x03#V[a\x0C\xE0a\x0C\xCFa\x1E\x95V[a\x0C\xD7a\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[4a\r\x19Wa\x0C\xF96`\x04a\x03#V[a\r\x15a\r\x04a\x1E\xE1V[a\r\x0Ca\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[\x90` \x82\x82\x03\x12a\rOW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\rJWa\rF\x92\x01a\x03\xD1V[\x90\x91V[a\x020V[a\x02,V[4a\r\x83Wa\rma\rg6`\x04a\r\x1EV[\x90a \x1DV[a\rua\x02\"V[\x80a\r\x7F\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[\x90` \x82\x82\x03\x12a\r\xA1Wa\r\x9E\x91_\x01a\x07\xB0V[\x90V[a\x02,V[4a\r\xD4Wa\r\xBEa\r\xB96`\x04a\r\x88V[a \xCDV[a\r\xC6a\x02\"V[\x80a\r\xD0\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0E-Wa\x0E\r6`\x04a\x03#V[a\x0E)a\x0E\x18a\r\xD9V[a\x0E a\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[4a\x0E`Wa\x0EB6`\x04a\x03#V[a\x0EJa \xF4V[a\x0ERa\x02\"V[\x80a\x0E\\\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[a\x0Eya\x0Eta\x0E~\x92a\x02\x7FV[a\x06\xC3V[a\x02\x7FV[\x90V[\x90a\x0E\x8B\x90a\x0EeV[_R` R`@_ \x90V[_\x1C\x90V[a\x0E\xA8a\x0E\xAD\x91a\x0E\x97V[a\tqV[\x90V[a\x0E\xBA\x90Ta\x0E\x9CV[\x90V[a\x0E\xC8\x90`\x04a\x0E\x81V[\x90a\x0E\xD4_\x83\x01a\x0E\xB0V[\x91a\x0E\xE1`\x01\x82\x01a\x0E\xB0V[\x91a\x0E\xFA`\x03a\x0E\xF3`\x02\x85\x01a\x0E\xB0V[\x93\x01a\x0E\xB0V[\x90V[a\x0F2a\x0F9\x94a\x0F(``\x94\x98\x97\x95a\x0F\x1E`\x80\x86\x01\x9A_\x87\x01\x90a\x06\xFAV[` \x85\x01\x90a\x06\xFAV[`@\x83\x01\x90a\x06\xFAV[\x01\x90a\x06\xFAV[V[4a\x0FoWa\x0Fka\x0FVa\x0FQ6`\x04a\x08 V[a\x0E\xBDV[\x90a\x0Fb\x94\x92\x94a\x02\"V[\x94\x85\x94\x85a\x0E\xFDV[\x03\x90\xF3[a\x02(V[\x90V[a\x0F\x8Ba\x0F\x86a\x0F\x90\x92a\x0FtV[a\x06\xC3V[a\x02\x7FV[\x90V[a\x0F\x9Ea\x13\x88a\x0FwV[\x90V[a\x0F\xA9a\x0F\x93V[\x90V[4a\x0F\xDCWa\x0F\xBC6`\x04a\x03#V[a\x0F\xD8a\x0F\xC7a\x0F\xA1V[a\x0F\xCFa\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[4a\x10\x0FWa\x0F\xF9a\x0F\xF46`\x04a\r\x88V[a!cV[a\x10\x01a\x02\"V[\x80a\x10\x0B\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[_\x80\xFD[\x91\x90a\x105a\x10/32\x90\x86\x85\x91\x92\x90\x91\x92a\x1B7V[\x15a\x04\xE5V[a\x10DWa\x10B\x92a\x10\xA1V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x10\\`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[a\x10i\x90a\t\x02V[\x90V[`@\x90a\x10\x98a\x10\x8Da\x10\x9F\x95\x97\x96\x94``\x84\x01\x90\x84\x82\x03_\x86\x01Ra\x05\xCDV[\x96` \x83\x01\x90a\x06\xFAV[\x01\x90a\x06\xFAV[V[\x90a\x10\xAD\x903\x92a\x16\x13V[\x91B\x92a\x10\xEFa\x10\xDD\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x10`V[\x94a\x10\xE6a\x02\"V[\x93\x84\x93\x84a\x10lV[\x03\x90\xA2V[\x90a\x10\xFF\x92\x91a\x10\x18V[V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x11\x1F\x90a\x05\xC3V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x119W`@RV[a\x11\x01V[\x90a\x11Qa\x11Ja\x02\"V[\x92\x83a\x11\x15V[V[a\x11]`\x80a\x11>V[\x90V[_\x90V[a\x11la\x11SV[\x90` \x80\x80\x80\x85a\x11{a\x11`V[\x81R\x01a\x11\x86a\x11`V[\x81R\x01a\x11\x91a\x11`V[\x81R\x01a\x11\x9Ca\x11`V[\x81RPPV[a\x11\xAAa\x11dV[\x90V[a\x11\xB5a\x11\xA2V[Pa\x11\xBEa!\xB1V[\x90V[P\x90V[P\x90V[` \x91\x81R\x01\x90V[` \x7Fve the same length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x7FData and priority arrays must ha_\x82\x01R\x01RV[a\x12,`2`@\x92a\x11\xC9V[a\x125\x81a\x11\xD2V[\x01\x90V[a\x12N\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x12\x1FV[\x90V[\x15a\x12XWV[a\x12`a\x02\"V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x12v`\x04\x82\x01a\x129V[\x03\x90\xFD[\x90V[a\x12\x91a\x12\x8Ca\x12\x96\x92a\x12zV[a\x06\xC3V[a\x02\x7FV[\x90V[`\x01a\x12\xA5\x91\x01a\x02\x7FV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x13\nW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x13\x05W` \x01\x91`\x01\x82\x026\x03\x83\x13a\x13\0WV[a\x12\xC4V[a\x12\xC0V[a\x12\xBCV[\x90\x82\x10\x15a\x13*W` a\x13&\x92\x02\x81\x01\x90a\x12\xC8V[\x90\x91V[a\x12\xA8V[\x91\x90\x81\x10\x15a\x13?W` \x02\x01\x90V[a\x12\xA8V[5a\x13N\x81a\x02\x82V[\x90V[\x90\x92a\x13^\x82\x85\x90a\x11\xC1V[\x93a\x13\x85\x85a\x13\x7Fa\x13ya\x13t\x88\x87\x90a\x11\xC5V[a\x02\x7FV[\x91a\x02\x7FV[\x14a\x12QV[a\x13\x8E_a\x12}V[[\x80a\x13\xA2a\x13\x9C\x88a\x02\x7FV[\x91a\x02\x7FV[\x10\x15a\x14IWa\x13\xD0\x90a\x13\xC632\x90a\x13\xBE\x88\x87\x86\x91a\x13\x0FV[\x92\x90\x91a\x1B7V[a\x13\xD5W[a\x12\x99V[a\x13\x8FV[3a\x13\xEBa\x13\xE5\x87\x86\x85\x91a\x13\x0FV[\x90a\x16\x13V[\x90a\x14\0a\x13\xFB\x89\x88\x86\x91a\x13/V[a\x13DV[B\x92a\x14Aa\x14/\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x10`V[\x94a\x148a\x02\"V[\x93\x84\x93\x84a\x10lV[\x03\x90\xA2a\x13\xCBV[PPPPPPV[_\x90V[a\x14aa\x14f\x91a\x0E\x97V[a\x06PV[\x90V[a\x14s\x90Ta\x14UV[\x90V[a\x14~a\x14QV[Pa\x14\x89`\x03a\x14iV[\x90V[\x91\x90a\x14\xA9a\x14\xA332\x90\x86\x85\x91\x92\x90\x91\x92a\x1B7V[\x15a\x04\xE5V[a\x14\xB8Wa\x14\xB6\x92a\x157V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x14\xD0`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[\x90\x82_\x93\x92\x827\x01RV[\x91\x90a\x14\xF9\x81a\x14\xF2\x81a\x14\xFE\x95a\x05\xAFV[\x80\x95a\x14\xD4V[a\x05\xC3V[\x01\x90V[a\x15.a\x15#`@\x93a\x155\x96\x98\x97\x95``\x85\x01\x91\x85\x83\x03_\x87\x01Ra\x14\xDFV[\x96` \x83\x01\x90a\x06\xFAV[\x01\x90a\x06\xFAV[V[\x90\x913\x91\x92\x90\x92a\x15~Ba\x15l\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x95a\x10`V[\x95a\x15ua\x02\"V[\x94\x85\x94\x85a\x15\x02V[\x03\x90\xA2V[\x90a\x15\x8E\x92\x91a\x14\x8CV[V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x15\xB8a\x15\xB3a\x15\xBD\x92a\x12zV[a\x15\x9EV[a\x15\x95V[\x90V[\x90V[a\x15\xCFa\x15\xD4\x91a\x15\x95V[a\x15\xC0V[\x90RV[\x90P\x90V[\x90\x91\x82a\x15\xED\x81a\x15\xF4\x93a\x15\xD8V[\x80\x93a\x14\xD4V[\x01\x90V[\x80a\x16\t`\x01\x92a\x16\x10\x96\x94a\x15\xC3V[\x01\x91a\x15\xDDV[\x90V[a\x16Q\x90a\x16\x1Fa\x15\x90V[Pa\x16Ba\x16,_a\x15\xA4V[\x91\x93a\x166a\x02\"V[\x94\x85\x93` \x85\x01a\x15\xF8V[` \x82\x01\x81\x03\x82R\x03\x82a\x11\x15V[\x90V[\x90a\x16pa\x16j32\x90\x85\x85\x91\x92\x90\x91\x92a\x1B7V[\x15a\x04\xE5V[a\x16\x7FWa\x16}\x91a\x17 V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x16\x97`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[`\x08\x1C\x90V[a\x16\xADa\x16\xB2\x91a\x16\x9BV[a\x06PV[\x90V[a\x16\xBF\x90Ta\x16\xA1V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x16\xE5a\x16\xEB\x91\x93\x92\x93a\x02\x7FV[\x92a\x02\x7FV[\x82\x03\x91\x82\x11a\x16\xF6WV[a\x16\xC2V[a\x17\na\x17\x10\x91\x93\x92\x93a\x02\x7FV[\x92a\x02\x7FV[\x82\x01\x80\x92\x11a\x17\x1BWV[a\x16\xC2V[\x90a\x174a\x17.`\x03a\x16\xB5V[\x15a\x04\xE5V[a\x17iWa\x17Ta\x17g\x92a\x17Ma\x17b\x93Z\x92a\x17\x8EV[Z\x90a\x16\xD6V[a\x17\\a\x0F\x93V[\x90a\x16\xFBV[a#\x85V[V[a\x17r\x91a\x17\x8EV[V[\x90\x91a\x17\x8B\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x14\xDFV[\x90V[3\x90\x91a\x17\xBB\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10`V[\x92a\x17\xD0a\x17\xC7a\x02\"V[\x92\x83\x92\x83a\x17tV[\x03\x90\xA2V[\x90a\x17\xDF\x91a\x16TV[V[\x90a\x17\xF3\x91a\x17\xEEa$\x8CV[a\x18\xF9V[V[`\xA0\x1C\x90V[a\x18\x07a\x18\x0C\x91a\x17\xF5V[a\x06PV[\x90V[a\x18\x19\x90Ta\x17\xFBV[\x90V[a\x180a\x18+a\x185\x92a\x12zV[a\x06\xC3V[a\x07\x85V[\x90V[a\x18A\x90a\x18\x1CV[\x90V[`\xA0\x1B\x90V[\x90a\x18Y`\xFF`\xA0\x1B\x91a\x18DV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18l\x90a\x04\xE5V[\x90V[\x90V[\x90a\x18\x87a\x18\x82a\x18\x8E\x92a\x18cV[a\x18oV[\x82Ta\x18JV[\x90UV[a\x18\x9B\x90a\x08\xE6V[\x90V[a\x18\xA7\x90a\x18\x92V[\x90V[_\x1B\x90V[\x90a\x18\xC0`\x01\x80`\xA0\x1B\x03\x91a\x18\xAAV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18\xD3\x90a\x18\x92V[\x90V[\x90V[\x90a\x18\xEEa\x18\xE9a\x18\xF5\x92a\x18\xCAV[a\x18\xD6V[\x82Ta\x18\xAFV[\x90UV[a\x19\x03`\x01a\x18\x0FV[a\x19kW\x81a\x19\"a\x19\x1Ca\x19\x17_a\x188V[a\x07\x90V[\x91a\x07\x90V[\x14a\x19OWa\x19Ha\x19Aa\x19M\x93a\x19<`\x01\x80a\x18rV[a\x18\x9EV[`\x01a\x18\xD9V[a!cV[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x19g`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x19\x82`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[\x90a\x19\x90\x91a\x17\xE1V[V[\x90a\x19\x9C\x90a\x02\x7FV[\x90RV[\x90a\x1A\x07a\x19\xFE`\x03a\x19\xB1a\x11SV[\x94a\x19\xC8a\x19\xC0_\x83\x01a\x0E\xB0V[_\x88\x01a\x19\x92V[a\x19\xE0a\x19\xD7`\x01\x83\x01a\x0E\xB0V[` \x88\x01a\x19\x92V[a\x19\xF8a\x19\xEF`\x02\x83\x01a\x0E\xB0V[`@\x88\x01a\x19\x92V[\x01a\x0E\xB0V[``\x84\x01a\x19\x92V[V[a\x1A\x12\x90a\x19\xA0V[\x90V[a\x1A,a\x1A1\x91a\x1A$a\x11\xA2V[P`\x04a\x0E\x81V[a\x1A\tV[\x90V[a\x1A<a$\x8CV[a\x1ADa\x1AFV[V[a\x1ANa%\x17V[V[a\x1AXa\x1A4V[V[a\x1Aba$\x8CV[a\x1Aja\x1AlV[V[a\x1A}a\x1Ax_a\x188V[a%GV[V[a\x1A\x87a\x1AZV[V[a\x1A\x95a\x1A\x9A\x91a\x0E\x97V[a\x08\xA6V[\x90V[a\x1A\xA7\x90Ta\x1A\x89V[\x90V[`\xE0\x1B\x90V[a\x1A\xB9\x81a\x04\xE5V[\x03a\x1A\xC0WV[_\x80\xFD[\x90PQ\x90a\x1A\xD1\x82a\x1A\xB0V[V[\x90` \x82\x82\x03\x12a\x1A\xECWa\x1A\xE9\x91_\x01a\x1A\xC4V[\x90V[a\x02,V[a\x1B\x17a\x1B$\x95\x93\x94\x92\x94a\x1B\r``\x84\x01\x96_\x85\x01\x90a\x0C]V[` \x83\x01\x90a\x0C]V[`@\x81\x85\x03\x91\x01Ra\x14\xDFV[\x90V[a\x1B/a\x02\"V[=_\x82>=\x90\xFD[\x92a\x1Bz` \x93\x94a\x1BGa\x14QV[Pa\x1B\x85a\x1B]a\x1BX`\x01a\x1A\x9DV[a\t\x0EV[\x93cz9y\xDC\x92\x95\x97a\x1Bna\x02\"V[\x98\x89\x97\x88\x96\x87\x96a\x1A\xAAV[\x86R`\x04\x86\x01a\x1A\xF1V[\x03\x91Z\xFA\x90\x81\x15a\x1B\xC9W_\x91a\x1B\x9BW[P\x90V[a\x1B\xBC\x91P` =\x81\x11a\x1B\xC2W[a\x1B\xB4\x81\x83a\x11\x15V[\x81\x01\x90a\x1A\xD3V[_a\x1B\x97V[P=a\x1B\xAAV[a\x1B'V[\x90a\x1B\xEAa\x1B\xE432\x90\x85\x85\x91\x92\x90\x91\x92a\x1B7V[\x15a\x04\xE5V[a\x1B\xF9Wa\x1B\xF7\x91a\x1C\x15V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x1C\x11`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[\x90a\x1C)a\x1C#`\x03a\x16\xB5V[\x15a\x04\xE5V[a\x1C^Wa\x1CIa\x1C\\\x92a\x1CBa\x1CW\x93Z\x92a\x1CiV[Z\x90a\x16\xD6V[a\x1CQa\x0F\x93V[\x90a\x16\xFBV[a#\x85V[V[a\x1Cg\x91a\x1CiV[V[\x90a\x1Cu\x903\x92a\x16\x13V[\x90a\x1C\xB5a\x1C\xA3\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10`V[\x92a\x1C\xACa\x02\"V[\x91\x82\x91\x82a\x05\xFEV[\x03\x90\xA2V[\x90a\x1C\xC4\x91a\x1B\xCEV[V[_\x90V[a\x1C\xD4\x90Qa\x02\x7FV[\x90V[a\x1C\xDFa\x1C\xC6V[Pa\x1C\xE8a!\xB1V[a\x1C\xF3_\x82\x01a\x1C\xCAV[a\x1D\x05a\x1C\xFF_a\x12}V[\x91a\x02\x7FV[\x14a\x1DZWa\x1D\x18_a\x1D&\x92\x01a\x1C\xCAV[a\x1D a\t\xFDV[\x90a\x16\xFBV[Ba\x1D9a\x1D3\x83a\x02\x7FV[\x91a\x02\x7FV[\x10\x15a\x1DMWa\x1DJ\x90B\x90a\x16\xD6V[\x90V[Pa\x1DW_a\x12}V[\x90V[Pa\x1Dd_a\x12}V[\x90V[a\x1Dva\x1D|\x91\x93\x92\x93a\x02\x7FV[\x92a\x02\x7FV[\x91a\x1D\x88\x83\x82\x02a\x02\x7FV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1D\x97WV[a\x16\xC2V[\x91a\x1D\xA5a\x1C\xC6V[P\x80a\x1D\xB9a\x1D\xB3\x84a\x02\x7FV[\x91a\x02\x7FV[\x11\x15a\x1E\rWa\x1D\xDA\x91a\x1D\xCC\x91a\x16\xD6V[a\x1D\xD4a\x06\xE2V[\x90a\x1DgV[\x80a\x1D\xEDa\x1D\xE7\x84a\x02\x7FV[\x91a\x02\x7FV[\x10\x15a\x1D\xFFWa\x1D\xFC\x91a\x16\xD6V[\x90V[PPa\x1E\n_a\x12}V[\x90V[PP\x90V[a\x1E\x1Aa\x1C\xC6V[Pa\x1E.``a\x1E(a!\xB1V[\x01a\x1C\xCAV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1ELa\x1EQ\x91a\x0E\x97V[a\x1E5V[\x90V[a\x1E^\x90Ta\x1E@V[\x90V[a\x1Eia\x1E1V[Pa\x1Es_a\x1ETV[\x90V[\x90V[a\x1E\x8Da\x1E\x88a\x1E\x92\x92a\x1EvV[a\x06\xC3V[a\x02\x7FV[\x90V[a\x1E\x9Da\x1C\xC6V[Pa\x1E\xB1a\x1E\xAB`\x03a\x14iV[\x15a\x04\xE5V[a\x1E\xD5Wa\x1E\xD2a\x1E\xC2`\x02a\x0E\xB0V[a\x1E\xCC`\x01a\x1EyV[\x90a\x16\xFBV[\x90V[a\x1E\xDE_a\x12}V[\x90V[a\x1E\xE9a\x1C\xC6V[Pa\x1E\xFD`@a\x1E\xF7a!\xB1V[\x01a\x1C\xCAV[\x90V[\x90a\x1F\x14a\x1F\x0E`\x03a\x16\xB5V[\x15a\x04\xE5V[a\x1FIWa\x1F4a\x1FG\x92a\x1F-a\x1FB\x93Z\x92a\x1FTV[Z\x90a\x16\xD6V[a\x1F<a\x0F\x93V[\x90a\x16\xFBV[a#\x85V[V[a\x1FR\x91a\x1FTV[V[a\x1F_\x81\x83\x90a\x11\xC1V[\x91a\x1Fha\x1C\xC6V[Pa\x1Fr_a\x12}V[[\x80a\x1F\x86a\x1F\x80\x86a\x02\x7FV[\x91a\x02\x7FV[\x10\x15a \x17Wa\x1F\xB4\x90a\x1F\xAA32\x90a\x1F\xA2\x87\x87\x86\x91a\x13\x0FV[\x92\x90\x91a\x1B7V[a\x1F\xB9W[a\x12\x99V[a\x1FsV[3a\x1F\xCFa\x1F\xC9\x86\x86\x85\x91a\x13\x0FV[\x90a\x16\x13V[\x90a \x0Fa\x1F\xFD\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10`V[\x92a \x06a\x02\"V[\x91\x82\x91\x82a\x05\xFEV[\x03\x90\xA2a\x1F\xAFV[PPPPV[\x90a '\x91a\x1F\0V[V[a :\x90a 5a$\x8CV[a <V[V[\x80a Wa Qa L_a\x188V[a\x07\x90V[\x91a\x07\x90V[\x14a \xB1Wa oa h\x82a\x18\x9EV[`\x01a\x18\xD9V[a \x99\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10`V[\x90a \xA2a\x02\"V[\x80a \xAC\x81a\x02\xEAV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a \xC9`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[a \xD6\x90a )V[V[a \xE0a$\x8CV[a \xE8a \xEAV[V[a \xF2a%\xA6V[V[a \xFCa \xD8V[V[a!\x0F\x90a!\na$\x8CV[a!\x11V[V[\x80a!,a!&a!!_a\x188V[a\x07\x90V[\x91a\x07\x90V[\x14a!<Wa!:\x90a%GV[V[a!_a!H_a\x188V[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x0CjV[\x03\x90\xFD[a!l\x90a \xFEV[V[a!x`\x80a\x11>V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a!\x9Ba!\xA1\x91a\x02\x7FV[\x91a\x02\x7FV[\x90\x81\x15a!\xACW\x04\x90V[a!{V[a!\xB9a\x11\xA2V[Pa!\xCDa!\xC7`\x03a\x14iV[\x15a\x04\xE5V[a\"\xC9Wa!\xEEa!\xE9`\x04a!\xE3`\x02a\x0E\xB0V[\x90a\x0E\x81V[a\x1A\tV[Ba\"\x1Ca\"\x16a\"\x11a\"\x03_\x86\x01a\x1C\xCAV[a\"\x0Ba\t\xFDV[\x90a\x16\xFBV[a\x02\x7FV[\x91a\x02\x7FV[\x10\x15a\"%W\x90V[a\"r\x90a\"la\"]_a\"Va\"HBa\"B\x84\x88\x01a\x1C\xCAV[\x90a\x16\xD6V[a\"Pa\t\xFDV[\x90a!\x8FV[\x93\x01a\x1C\xCAV[\x91a\"fa\t\xFDV[\x90a\x1DgV[\x90a\x16\xFBV[a\"\xC6a\"\xBD_a\"\xB8a\"\xAF_a\"\xAAa\"\xA1_\x95a\"\x9Ca\"\x93a!nV[\x9A_\x8C\x01a\x19\x92V[a\x12}V[` \x89\x01a\x19\x92V[a\x12}V[`@\x86\x01a\x19\x92V[a\x12}V[``\x83\x01a\x19\x92V[\x90V[_a#&a#\x1D_a#\x18a#\x0F_a#\na#\x01_\x95a\"\xFCa\"\xF4a\"\xEEa!nV[\x9Ba\x12}V[_\x8C\x01a\x19\x92V[a\x12}V[` \x89\x01a\x19\x92V[a\x12}V[`@\x86\x01a\x19\x92V[a\x12}V[``\x83\x01a\x19\x92V[\x90V[\x90a#5_\x19\x91a\x18\xAAV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a#Wa#Ra#^\x92a\x0EeV[a#?V[\x82Ta#)V[\x90UV[\x91` a#\x83\x92\x94\x93a#|`@\x82\x01\x96_\x83\x01\x90a\x06\xFAV[\x01\x90a\x06\xFAV[V[a#\x98a#\x92`\x03a\x16\xB5V[\x15a\x04\xE5V[a$\x89Wa#\xAFa#\xA9`\x03a\x14iV[\x15a\x04\xE5V[a$|W[a#\xBCa'|V[a$-a#\xCA\x82:\x90a\x1DgV[a#\xFD\x83a#\xF7`\x02a#\xE7`\x04a#\xE1\x83a\x0E\xB0V[\x90a\x0E\x81V[\x01\x91a#\xF2\x83a\x0E\xB0V[a\x16\xFBV[\x90a#BV[a$'`\x03a$\x17`\x04a$\x11`\x02a\x0E\xB0V[\x90a\x0E\x81V[\x01\x91a$\"\x83a\x0E\xB0V[a\x16\xFBV[\x90a#BV[a$7`\x02a\x0E\xB0V[:a$b\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0EeV[\x92a$wa$na\x02\"V[\x92\x83\x92\x83a#bV[\x03\x90\xA2V[a$\x84a&qV[a#\xB4V[PV[a$\x94a\x1EaV[a$\xADa$\xA7a$\xA2a)TV[a\x07\x90V[\x91a\x07\x90V[\x03a$\xB4WV[a$\xD6a$\xBFa)TV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\x0CjV[\x03\x90\xFD[`\x08\x1B\x90V[\x90a$\xEDa\xFF\0\x91a$\xDAV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a%\x0Ca%\x07a%\x13\x92a\x18cV[a\x18oV[\x82Ta$\xE0V[\x90UV[a%\"_`\x03a$\xF7V[V[\x90V[\x90a%<a%7a%C\x92a\x10`V[a%$V[\x82Ta\x18\xAFV[\x90UV[a%P_a\x1ETV[a%Z\x82_a%'V[\x90a%\x8Ea%\x88\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10`V[\x91a\x10`V[\x91a%\x97a\x02\"V[\x80a%\xA1\x81a\x02\xEAV[\x03\x90\xA3V[a%\xB2`\x01`\x03a$\xF7V[V[\x90a%\xC0`\xFF\x91a\x18\xAAV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a%\xDFa%\xDAa%\xE6\x92a\x18cV[a\x18oV[\x82Ta%\xB4V[\x90UV[\x90a%\xF4\x90a\x12}V[_R` R`@_ \x90V[\x90a&]```\x03a&c\x94a&#_\x82\x01a&\x1D_\x88\x01a\x1C\xCAV[\x90a#BV[a&<`\x01\x82\x01a&6` \x88\x01a\x1C\xCAV[\x90a#BV[a&U`\x02\x82\x01a&O`@\x88\x01a\x1C\xCAV[\x90a#BV[\x01\x92\x01a\x1C\xCAV[\x90a#BV[V[\x90a&o\x91a&\0V[V[a&\x84a&~`\x03a\x14iV[\x15a\x04\xE5V[a&\x8BW[V[a&\x97`\x01`\x03a%\xCAV[a&\xAAa&\xA3_a\x12}V[`\x02a#BV[a'\x13Ba'\x02a&\xF9_a&\xF4a&\xEB_a&\xE6a&\xDD_\x95a&\xD8a&\xCFa!nV[\x9A_\x8C\x01a\x19\x92V[a\x12}V[` \x89\x01a\x19\x92V[a\x12}V[`@\x86\x01a\x19\x92V[a\x12}V[``\x83\x01a\x19\x92V[a'\x0E`\x04_\x90a%\xEAV[a&eV[_B\x90a'Ua'C\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x12}V[\x92a'La\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xA2a&\x89V[\x90V[a'i\x90a\x02\x7FV[_\x19\x81\x14a'wW`\x01\x01\x90V[a\x16\xC2V[a'\x99a'\x94`\x04a'\x8E`\x02a\x0E\xB0V[\x90a\x0E\x81V[a']V[Ba'\xC7a'\xC1a'\xBCa'\xAE_\x86\x01a\x0E\xB0V[a'\xB6a\t\xFDV[\x90a\x16\xFBV[a\x02\x7FV[\x91a\x02\x7FV[\x10\x15a'\xD1W[PV[a'\xF9a'\xF0a'\xE2_\x84\x01a\x0E\xB0V[a'\xEAa\t\xFDV[\x90a\x16\xFBV[`\x01\x83\x01a#BV[a(\x03`\x02a\x0E\xB0V[a(0a(\x12`\x02\x84\x01a\x0E\xB0V[\x92a(*_a(#`\x01\x84\x01a\x0E\xB0V[\x92\x01a\x0E\xB0V[\x90a\x16\xD6V[a(Z\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0EeV[\x92a(oa(fa\x02\"V[\x92\x83\x92\x83a#bV[\x03\x90\xA2a(\x8Ea(\x87a(\x82`\x02a\x0E\xB0V[a'`V[`\x02a#BV[a)\0Ba(\xE6a(\xDD_a(\xD8a(\xCF_a(\xCAa(\xC1_\x95a(\xBCa(\xB3a!nV[\x9A_\x8C\x01a\x19\x92V[a\x12}V[` \x89\x01a\x19\x92V[a\x12}V[`@\x86\x01a\x19\x92V[a\x12}V[``\x83\x01a\x19\x92V[a(\xFB`\x04a(\xF5`\x02a\x0E\xB0V[\x90a\x0E\x81V[a&eV[a)\n`\x02a\x0E\xB0V[B\x90a)Ka)9\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0EeV[\x92a)Ba\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xA2_a'\xCEV[a)\\a\x1E1V[P3\x90V`\x80`@R4`\x1CW`\x0E` V[a\x01\x94a\0+\x829a\x01\x94\x90\xF3[`&V[`@Q\x90V[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x01{V[a\0\x1D_5a\0,V[cz9y\xDC\x03a\0\x0EWa\x01BV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\0X\x90a\0DV[\x90V[a\0d\x81a\0OV[\x03a\0kWV[_\x80\xFD[\x90P5\x90a\0|\x82a\0[V[V[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\0\xC4W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xBFW` \x01\x92`\x01\x83\x02\x84\x01\x11a\0\xBAWV[a\0\x86V[a\0\x82V[a\0~V[\x91``\x83\x83\x03\x12a\x01\x16Wa\0\xE0\x82_\x85\x01a\0oV[\x92a\0\xEE\x83` \x83\x01a\0oV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x11Wa\x01\r\x92\x01a\0\x8AV[\x90\x91V[a\0@V[a\0<V[\x15\x15\x90V[a\x01)\x90a\x01\x1BV[\x90RV[\x91\x90a\x01@\x90_` \x85\x01\x94\x01\x90a\x01 V[V[4a\x01vWa\x01ra\x01aa\x01X6`\x04a\0\xC9V[\x92\x91\x90\x91a\x01\x83V[a\x01ia\x002V[\x91\x82\x91\x82a\x01-V[\x03\x90\xF3[a\08V[_\x80\xFD[_\x90V[PPPPa\x01\x8Fa\x01\x7FV[P_\x90V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b611014565b61001d5f3561021c565b8063050ec13814610217578063086146d21461021257806311992f8c1461020d57806318d5aafe146102085780631c0b636714610203578063366cbab7146101fe5780633b6ab2a9146101f95780633d44ae8b146101f457806346e2cc09146101ef578063485cc955146101ea5780634b2c0706146101e55780635467cb48146101e05780635b3cd6e2146101db57806361543801146101d65780636558954f146101d1578063715018a6146101cc5780637a3979dc146101c7578063804e5123146101c257806382f44ade146101bd57806383d3c115146101b857806384fab62b146101b35780638d5a239b146101ae5780638da5cb5b146101a9578063aff74c6d146101a4578063c660d3f31461019f578063cdafb9781461019a578063d4f0eb4d14610195578063d878134214610190578063de1f453e1461018b578063ea4a110414610186578063ede07bd6146101815763f2fde38b0361000e57610fe1565b610fac565b610f3b565b610e32565b610dfd565b610da6565b610d54565b610ce9565b610cb4565b610c7f565b610c28565b610bf3565b610bad565b610b3e565b610b0a565b610ad1565b610a4c565b610a17565b6109a9565b61093c565b610873565b61083e565b6107ec565b610751565b61071c565b61068b565b610616565b610541565b61050c565b6104ae565b61039c565b6102ef565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f8301121561027a5781359167ffffffffffffffff831161027557602001926001830284011161027057565b61023c565b610238565b610234565b90565b61028b8161027f565b0361029257565b5f80fd5b905035906102a382610282565b565b916040838303126102e5575f83013567ffffffffffffffff81116102e0576102d2836102dd928601610240565b939094602001610296565b90565b610230565b61022c565b5f0190565b3461031e576103086103023660046102a5565b916110f4565b610310610222565b8061031a816102ea565b0390f35b610228565b5f91031261032d57565b61022c565b61033b9061027f565b9052565b90606080610385936103575f8201515f860190610332565b61036960208201516020860190610332565b61037b60408201516040860190610332565b0151910190610332565b565b919061039a905f6080850194019061033f565b565b346103cc576103ac366004610323565b6103c86103b76111ad565b6103bf610222565b91829182610387565b0390f35b610228565b909182601f8301121561040b5781359167ffffffffffffffff831161040657602001926020830284011161040157565b61023c565b610238565b610234565b909182601f8301121561044a5781359167ffffffffffffffff831161044557602001926020830284011161044057565b61023c565b610238565b610234565b90916040828403126104a9575f82013567ffffffffffffffff81116104a4578361047a9184016103d1565b929093602082013567ffffffffffffffff811161049f5761049b9201610410565b9091565b610230565b610230565b61022c565b346104e0576104ca6104c136600461044f565b92919091611351565b6104d2610222565b806104dc816102ea565b0390f35b610228565b151590565b6104f3906104e5565b9052565b919061050a905f602085019401906104ea565b565b3461053c5761051c366004610323565b610538610527611476565b61052f610222565b918291826104f7565b0390f35b610228565b346105705761055a6105543660046102a5565b91611583565b610562610222565b8061056c816102ea565b0390f35b610228565b906020828203126105a6575f82013567ffffffffffffffff81116105a15761059d9201610240565b9091565b610230565b61022c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6105ec6105f56020936105fa936105e3816105ab565b938480936105af565b958691016105b8565b6105c3565b0190565b6106139160208201915f8184039101526105cd565b90565b346106475761064361063261062c366004610575565b90611613565b61063a610222565b918291826105fe565b0390f35b610228565b1c90565b60ff1690565b61066690600861066b930261064c565b610650565b90565b906106799154610656565b90565b61068860035f9061066e565b90565b346106bb5761069b366004610323565b6106b76106a661067c565b6106ae610222565b918291826104f7565b0390f35b610228565b90565b90565b6106da6106d56106df926106c0565b6106c3565b61027f565b90565b6106ec600a6106c6565b90565b6106f76106e2565b90565b6107039061027f565b9052565b919061071a905f602085019401906106fa565b565b3461074c5761072c366004610323565b6107486107376106ef565b61073f610222565b91829182610707565b0390f35b610228565b346107805761076a610764366004610575565b906117d5565b610772610222565b8061077c816102ea565b0390f35b610228565b60018060a01b031690565b61079990610785565b90565b6107a581610790565b036107ac57565b5f80fd5b905035906107bd8261079c565b565b91906040838203126107e757806107db6107e4925f86016107b0565b936020016107b0565b90565b61022c565b3461081b576108056107ff3660046107bf565b90611986565b61080d610222565b80610817816102ea565b0390f35b610228565b9060208282031261083957610836915f01610296565b90565b61022c565b3461086e5761086a610859610854366004610820565b611a15565b610861610222565b91829182610387565b0390f35b610228565b346108a157610883366004610323565b61088b611a50565b610893610222565b8061089d816102ea565b0390f35b610228565b60018060a01b031690565b6108c19060086108c6930261064c565b6108a6565b90565b906108d491546108b1565b90565b6108e360015f906108c9565b90565b6108fa6108f56108ff92610785565b6106c3565b610785565b90565b61090b906108e6565b90565b61091790610902565b90565b6109239061090e565b9052565b919061093a905f6020850194019061091a565b565b3461096c5761094c366004610323565b6109686109576108d7565b61095f610222565b91829182610927565b0390f35b610228565b90565b610984906008610989930261064c565b610971565b90565b906109979154610974565b90565b6109a660025f9061098c565b90565b346109d9576109b9366004610323565b6109d56109c461099a565b6109cc610222565b91829182610707565b0390f35b610228565b90565b6109f56109f06109fa926109de565b6106c3565b61027f565b90565b610a0962278d006109e1565b90565b610a146109fd565b90565b34610a4757610a27366004610323565b610a43610a32610a0c565b610a3a610222565b91829182610707565b0390f35b610228565b34610a7a57610a5c366004610323565b610a64611a7f565b610a6c610222565b80610a76816102ea565b0390f35b610228565b91606083830312610acc57610a96825f85016107b0565b92610aa483602083016107b0565b92604082013567ffffffffffffffff8111610ac757610ac39201610240565b9091565b610230565b61022c565b34610b0557610b01610af0610ae7366004610a7f565b92919091611b37565b610af8610222565b918291826104f7565b0390f35b610228565b34610b3957610b23610b1d366004610575565b90611cba565b610b2b610222565b80610b35816102ea565b0390f35b610228565b34610b6e57610b4e366004610323565b610b6a610b59611cd7565b610b61610222565b91829182610707565b0390f35b610228565b9091606082840312610ba857610ba5610b8e845f8501610296565b93610b9c8160208601610296565b93604001610296565b90565b61022c565b34610bde57610bda610bc9610bc3366004610b73565b91611d9c565b610bd1610222565b91829182610707565b0390f35b610228565b610bf0600360019061066e565b90565b34610c2357610c03366004610323565b610c1f610c0e610be3565b610c16610222565b918291826104f7565b0390f35b610228565b34610c5857610c38366004610323565b610c54610c43611e12565b610c4b610222565b91829182610707565b0390f35b610228565b610c6690610790565b9052565b9190610c7d905f60208501940190610c5d565b565b34610caf57610c8f366004610323565b610cab610c9a611e61565b610ca2610222565b91829182610c6a565b0390f35b610228565b34610ce457610cc4366004610323565b610ce0610ccf611e95565b610cd7610222565b91829182610707565b0390f35b610228565b34610d1957610cf9366004610323565b610d15610d04611ee1565b610d0c610222565b91829182610707565b0390f35b610228565b90602082820312610d4f575f82013567ffffffffffffffff8111610d4a57610d4692016103d1565b9091565b610230565b61022c565b34610d8357610d6d610d67366004610d1e565b9061201d565b610d75610222565b80610d7f816102ea565b0390f35b610228565b90602082820312610da157610d9e915f016107b0565b90565b61022c565b34610dd457610dbe610db9366004610d88565b6120cd565b610dc6610222565b80610dd0816102ea565b0390f35b610228565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610e2d57610e0d366004610323565b610e29610e18610dd9565b610e20610222565b91829182610707565b0390f35b610228565b34610e6057610e42366004610323565b610e4a6120f4565b610e52610222565b80610e5c816102ea565b0390f35b610228565b610e79610e74610e7e9261027f565b6106c3565b61027f565b90565b90610e8b90610e65565b5f5260205260405f2090565b5f1c90565b610ea8610ead91610e97565b610971565b90565b610eba9054610e9c565b90565b610ec8906004610e81565b90610ed45f8301610eb0565b91610ee160018201610eb0565b91610efa6003610ef360028501610eb0565b9301610eb0565b90565b610f32610f3994610f28606094989795610f1e608086019a5f8701906106fa565b60208501906106fa565b60408301906106fa565b01906106fa565b565b34610f6f57610f6b610f56610f51366004610820565b610ebd565b90610f62949294610222565b94859485610efd565b0390f35b610228565b90565b610f8b610f86610f9092610f74565b6106c3565b61027f565b90565b610f9e611388610f77565b90565b610fa9610f93565b90565b34610fdc57610fbc366004610323565b610fd8610fc7610fa1565b610fcf610222565b91829182610707565b0390f35b610228565b3461100f57610ff9610ff4366004610d88565b612163565b611001610222565b8061100b816102ea565b0390f35b610228565b5f80fd5b919061103561102f33329086859192909192611b37565b156104e5565b61104457611042926110a1565b565b5f631b8e828b60e31b81528061105c600482016102ea565b0390fd5b61106990610902565b90565b60409061109861108d61109f9597969460608401908482035f8601526105cd565b9660208301906106fa565b01906106fa565b565b906110ad903392611613565b9142926110ef6110dd7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f294611060565b946110e6610222565b9384938461106c565b0390a2565b906110ff9291611018565b565b634e487b7160e01b5f52604160045260245ffd5b9061111f906105c3565b810190811067ffffffffffffffff82111761113957604052565b611101565b9061115161114a610222565b9283611115565b565b61115d608061113e565b90565b5f90565b61116c611153565b9060208080808561117b611160565b815201611186611160565b815201611191611160565b81520161119c611160565b81525050565b6111aa611164565b90565b6111b56111a2565b506111be6121b1565b90565b5090565b5090565b60209181520190565b60207f7665207468652073616d65206c656e6774680000000000000000000000000000917f4461746120616e64207072696f7269747920617272617973206d7573742068615f8201520152565b61122c60326040926111c9565b611235816111d2565b0190565b61124e9060208101905f81830391015261121f565b90565b1561125857565b611260610222565b62461bcd60e51b81528061127660048201611239565b0390fd5b90565b61129161128c6112969261127a565b6106c3565b61027f565b90565b60016112a5910161027f565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561130a570180359067ffffffffffffffff82116113055760200191600182023603831361130057565b6112c4565b6112c0565b6112bc565b9082101561132a57602061132692028101906112c8565b9091565b6112a8565b919081101561133f576020020190565b6112a8565b3561134e81610282565b90565b909261135e8285906111c1565b936113858561137f6113796113748887906111c5565b61027f565b9161027f565b14611251565b61138e5f61127d565b5b806113a261139c8861027f565b9161027f565b1015611449576113d0906113c63332906113be8887869161130f565b929091611b37565b6113d5575b611299565b61138f565b336113eb6113e58786859161130f565b90611613565b906114006113fb8988869161132f565b611344565b429261144161142f7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f294611060565b94611438610222565b9384938461106c565b0390a26113cb565b505050505050565b5f90565b61146161146691610e97565b610650565b90565b6114739054611455565b90565b61147e611451565b506114896003611469565b90565b91906114a96114a333329086859192909192611b37565b156104e5565b6114b8576114b692611537565b565b5f631b8e828b60e31b8152806114d0600482016102ea565b0390fd5b90825f939282370152565b91906114f9816114f2816114fe956105af565b80956114d4565b6105c3565b0190565b61152e6115236040936115359698979560608501918583035f8701526114df565b9660208301906106fa565b01906106fa565b565b9091339192909261157e4261156c7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f295611060565b95611575610222565b94859485611502565b0390a2565b9061158e929161148c565b565b606090565b60ff60f81b1690565b60f81b90565b6115b86115b36115bd9261127a565b61159e565b611595565b90565b90565b6115cf6115d491611595565b6115c0565b9052565b905090565b9091826115ed816115f4936115d8565b80936114d4565b0190565b8061160960019261161096946115c3565b01916115dd565b90565b6116519061161f611590565b5061164261162c5f6115a4565b9193611636610222565b948593602085016115f8565b60208201810382520382611115565b90565b9061167061166a33329085859192909192611b37565b156104e5565b61167f5761167d91611720565b565b5f631b8e828b60e31b815280611697600482016102ea565b0390fd5b60081c90565b6116ad6116b29161169b565b610650565b90565b6116bf90546116a1565b90565b634e487b7160e01b5f52601160045260245ffd5b6116e56116eb9193929361027f565b9261027f565b82039182116116f657565b6116c2565b61170a6117109193929361027f565b9261027f565b820180921161171b57565b6116c2565b9061173461172e60036116b5565b156104e5565b611769576117546117679261174d611762935a9261178e565b5a906116d6565b61175c610f93565b906116fb565b612385565b565b6117729161178e565b565b909161178b9260208301925f8185039101526114df565b90565b3390916117bb7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611060565b926117d06117c7610222565b92839283611774565b0390a2565b906117df91611654565b565b906117f3916117ee61248c565b6118f9565b565b60a01c90565b61180761180c916117f5565b610650565b90565b61181990546117fb565b90565b61183061182b6118359261127a565b6106c3565b610785565b90565b6118419061181c565b90565b60a01b90565b9061185960ff60a01b91611844565b9181191691161790565b61186c906104e5565b90565b90565b9061188761188261188e92611863565b61186f565b825461184a565b9055565b61189b906108e6565b90565b6118a790611892565b90565b5f1b90565b906118c060018060a01b03916118aa565b9181191691161790565b6118d390611892565b90565b90565b906118ee6118e96118f5926118ca565b6118d6565b82546118af565b9055565b611903600161180f565b61196b578161192261191c6119175f611838565b610790565b91610790565b1461194f5761194861194161194d9361193c600180611872565b61189e565b60016118d9565b612163565b565b5f632e7f3c7f60e11b815280611967600482016102ea565b0390fd5b5f62dc149f60e41b815280611982600482016102ea565b0390fd5b90611990916117e1565b565b9061199c9061027f565b9052565b90611a076119fe60036119b1611153565b946119c86119c05f8301610eb0565b5f8801611992565b6119e06119d760018301610eb0565b60208801611992565b6119f86119ef60028301610eb0565b60408801611992565b01610eb0565b60608401611992565b565b611a12906119a0565b90565b611a2c611a3191611a246111a2565b506004610e81565b611a09565b90565b611a3c61248c565b611a44611a46565b565b611a4e612517565b565b611a58611a34565b565b611a6261248c565b611a6a611a6c565b565b611a7d611a785f611838565b612547565b565b611a87611a5a565b565b611a95611a9a91610e97565b6108a6565b90565b611aa79054611a89565b90565b60e01b90565b611ab9816104e5565b03611ac057565b5f80fd5b90505190611ad182611ab0565b565b90602082820312611aec57611ae9915f01611ac4565b90565b61022c565b611b17611b249593949294611b0d60608401965f850190610c5d565b6020830190610c5d565b60408185039101526114df565b90565b611b2f610222565b3d5f823e3d90fd5b92611b7a60209394611b47611451565b50611b85611b5d611b586001611a9d565b61090e565b93637a3979dc929597611b6e610222565b98899788968796611aaa565b865260048601611af1565b03915afa908115611bc9575f91611b9b575b5090565b611bbc915060203d8111611bc2575b611bb48183611115565b810190611ad3565b5f611b97565b503d611baa565b611b27565b90611bea611be433329085859192909192611b37565b156104e5565b611bf957611bf791611c15565b565b5f631b8e828b60e31b815280611c11600482016102ea565b0390fd5b90611c29611c2360036116b5565b156104e5565b611c5e57611c49611c5c92611c42611c57935a92611c69565b5a906116d6565b611c51610f93565b906116fb565b612385565b565b611c6791611c69565b565b90611c75903392611613565b90611cb5611ca37f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611060565b92611cac610222565b918291826105fe565b0390a2565b90611cc491611bce565b565b5f90565b611cd4905161027f565b90565b611cdf611cc6565b50611ce86121b1565b611cf35f8201611cca565b611d05611cff5f61127d565b9161027f565b14611d5a57611d185f611d269201611cca565b611d206109fd565b906116fb565b42611d39611d338361027f565b9161027f565b1015611d4d57611d4a9042906116d6565b90565b50611d575f61127d565b90565b50611d645f61127d565b90565b611d76611d7c9193929361027f565b9261027f565b91611d8883820261027f565b928184041490151715611d9757565b6116c2565b91611da5611cc6565b5080611db9611db38461027f565b9161027f565b1115611e0d57611dda91611dcc916116d6565b611dd46106e2565b90611d67565b80611ded611de78461027f565b9161027f565b1015611dff57611dfc916116d6565b90565b5050611e0a5f61127d565b90565b505090565b611e1a611cc6565b50611e2e6060611e286121b1565b01611cca565b90565b5f90565b60018060a01b031690565b611e4c611e5191610e97565b611e35565b90565b611e5e9054611e40565b90565b611e69611e31565b50611e735f611e54565b90565b90565b611e8d611e88611e9292611e76565b6106c3565b61027f565b90565b611e9d611cc6565b50611eb1611eab6003611469565b156104e5565b611ed557611ed2611ec26002610eb0565b611ecc6001611e79565b906116fb565b90565b611ede5f61127d565b90565b611ee9611cc6565b50611efd6040611ef76121b1565b01611cca565b90565b90611f14611f0e60036116b5565b156104e5565b611f4957611f34611f4792611f2d611f42935a92611f54565b5a906116d6565b611f3c610f93565b906116fb565b612385565b565b611f5291611f54565b565b611f5f8183906111c1565b91611f68611cc6565b50611f725f61127d565b5b80611f86611f808661027f565b9161027f565b101561201757611fb490611faa333290611fa28787869161130f565b929091611b37565b611fb9575b611299565b611f73565b33611fcf611fc98686859161130f565b90611613565b9061200f611ffd7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611060565b92612006610222565b918291826105fe565b0390a2611faf565b50505050565b9061202791611f00565b565b61203a9061203561248c565b61203c565b565b8061205761205161204c5f611838565b610790565b91610790565b146120b15761206f6120688261189e565b60016118d9565b6120997f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b991611060565b906120a2610222565b806120ac816102ea565b0390a2565b5f632e7f3c7f60e11b8152806120c9600482016102ea565b0390fd5b6120d690612029565b565b6120e061248c565b6120e86120ea565b565b6120f26125a6565b565b6120fc6120d8565b565b61210f9061210a61248c565b612111565b565b8061212c6121266121215f611838565b610790565b91610790565b1461213c5761213a90612547565b565b61215f6121485f611838565b5f918291631e4fbdf760e01b835260048301610c6a565b0390fd5b61216c906120fe565b565b612178608061113e565b90565b634e487b7160e01b5f52601260045260245ffd5b61219b6121a19161027f565b9161027f565b9081156121ac570490565b61217b565b6121b96111a2565b506121cd6121c76003611469565b156104e5565b6122c9576121ee6121e960046121e36002610eb0565b90610e81565b611a09565b4261221c6122166122116122035f8601611cca565b61220b6109fd565b906116fb565b61027f565b9161027f565b10156122255790565b6122729061226c61225d5f61225661224842612242848801611cca565b906116d6565b6122506109fd565b9061218f565b9301611cca565b916122666109fd565b90611d67565b906116fb565b6122c66122bd5f6122b86122af5f6122aa6122a15f9561229c61229361216e565b9a5f8c01611992565b61127d565b60208901611992565b61127d565b60408601611992565b61127d565b60608301611992565b90565b5f61232661231d5f61231861230f5f61230a6123015f956122fc6122f46122ee61216e565b9b61127d565b5f8c01611992565b61127d565b60208901611992565b61127d565b60408601611992565b61127d565b60608301611992565b90565b906123355f19916118aa565b9181191691161790565b90565b9061235761235261235e92610e65565b61233f565b8254612329565b9055565b91602061238392949361237c60408201965f8301906106fa565b01906106fa565b565b61239861239260036116b5565b156104e5565b612489576123af6123a96003611469565b156104e5565b61247c575b6123bc61277c565b61242d6123ca823a90611d67565b6123fd836123f760026123e760046123e183610eb0565b90610e81565b01916123f283610eb0565b6116fb565b90612342565b612427600361241760046124116002610eb0565b90610e81565b019161242283610eb0565b6116fb565b90612342565b6124376002610eb0565b3a6124627f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610e65565b9261247761246e610222565b92839283612362565b0390a2565b612484612671565b6123b4565b50565b612494611e61565b6124ad6124a76124a2612954565b610790565b91610790565b036124b457565b6124d66124bf612954565b5f91829163118cdaa760e01b835260048301610c6a565b0390fd5b60081b90565b906124ed61ff00916124da565b9181191691161790565b9061250c61250761251392611863565b61186f565b82546124e0565b9055565b6125225f60036124f7565b565b90565b9061253c61253761254392611060565b612524565b82546118af565b9055565b6125505f611e54565b61255a825f612527565b9061258e6125887f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093611060565b91611060565b91612597610222565b806125a1816102ea565b0390a3565b6125b2600160036124f7565b565b906125c060ff916118aa565b9181191691161790565b906125df6125da6125e692611863565b61186f565b82546125b4565b9055565b906125f49061127d565b5f5260205260405f2090565b9061265d60606003612663946126235f820161261d5f8801611cca565b90612342565b61263c6001820161263660208801611cca565b90612342565b6126556002820161264f60408801611cca565b90612342565b019201611cca565b90612342565b565b9061266f91612600565b565b61268461267e6003611469565b156104e5565b61268b575b565b612697600160036125ca565b6126aa6126a35f61127d565b6002612342565b612713426127026126f95f6126f46126eb5f6126e66126dd5f956126d86126cf61216e565b9a5f8c01611992565b61127d565b60208901611992565b61127d565b60408601611992565b61127d565b60608301611992565b61270e60045f906125ea565b612665565b5f42906127556127437f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e9261127d565b9261274c610222565b91829182610707565b0390a2612689565b90565b6127699061027f565b5f1981146127775760010190565b6116c2565b612799612794600461278e6002610eb0565b90610e81565b61275d565b426127c76127c16127bc6127ae5f8601610eb0565b6127b66109fd565b906116fb565b61027f565b9161027f565b10156127d1575b50565b6127f96127f06127e25f8401610eb0565b6127ea6109fd565b906116fb565b60018301612342565b6128036002610eb0565b61283061281260028401610eb0565b9261282a5f61282360018401610eb0565b9201610eb0565b906116d6565b61285a7f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610e65565b9261286f612866610222565b92839283612362565b0390a261288e6128876128826002610eb0565b612760565b6002612342565b612900426128e66128dd5f6128d86128cf5f6128ca6128c15f956128bc6128b361216e565b9a5f8c01611992565b61127d565b60208901611992565b61127d565b60408601611992565b61127d565b60608301611992565b6128fb60046128f56002610eb0565b90610e81565b612665565b61290a6002610eb0565b429061294b6129397f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610e65565b92612942610222565b91829182610707565b0390a25f6127ce565b61295c611e31565b50339056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x10\x14V[a\0\x1D_5a\x02\x1CV[\x80c\x05\x0E\xC18\x14a\x02\x17W\x80c\x08aF\xD2\x14a\x02\x12W\x80c\x11\x99/\x8C\x14a\x02\rW\x80c\x18\xD5\xAA\xFE\x14a\x02\x08W\x80c\x1C\x0Bcg\x14a\x02\x03W\x80c6l\xBA\xB7\x14a\x01\xFEW\x80c;j\xB2\xA9\x14a\x01\xF9W\x80c=D\xAE\x8B\x14a\x01\xF4W\x80cF\xE2\xCC\t\x14a\x01\xEFW\x80cH\\\xC9U\x14a\x01\xEAW\x80cK,\x07\x06\x14a\x01\xE5W\x80cTg\xCBH\x14a\x01\xE0W\x80c[<\xD6\xE2\x14a\x01\xDBW\x80caT8\x01\x14a\x01\xD6W\x80ceX\x95O\x14a\x01\xD1W\x80cqP\x18\xA6\x14a\x01\xCCW\x80cz9y\xDC\x14a\x01\xC7W\x80c\x80NQ#\x14a\x01\xC2W\x80c\x82\xF4J\xDE\x14a\x01\xBDW\x80c\x83\xD3\xC1\x15\x14a\x01\xB8W\x80c\x84\xFA\xB6+\x14a\x01\xB3W\x80c\x8DZ#\x9B\x14a\x01\xAEW\x80c\x8D\xA5\xCB[\x14a\x01\xA9W\x80c\xAF\xF7Lm\x14a\x01\xA4W\x80c\xC6`\xD3\xF3\x14a\x01\x9FW\x80c\xCD\xAF\xB9x\x14a\x01\x9AW\x80c\xD4\xF0\xEBM\x14a\x01\x95W\x80c\xD8x\x13B\x14a\x01\x90W\x80c\xDE\x1FE>\x14a\x01\x8BW\x80c\xEAJ\x11\x04\x14a\x01\x86W\x80c\xED\xE0{\xD6\x14a\x01\x81Wc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\x0F\xE1V[a\x0F\xACV[a\x0F;V[a\x0E2V[a\r\xFDV[a\r\xA6V[a\rTV[a\x0C\xE9V[a\x0C\xB4V[a\x0C\x7FV[a\x0C(V[a\x0B\xF3V[a\x0B\xADV[a\x0B>V[a\x0B\nV[a\n\xD1V[a\nLV[a\n\x17V[a\t\xA9V[a\t<V[a\x08sV[a\x08>V[a\x07\xECV[a\x07QV[a\x07\x1CV[a\x06\x8BV[a\x06\x16V[a\x05AV[a\x05\x0CV[a\x04\xAEV[a\x03\x9CV[a\x02\xEFV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x02zW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x02uW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x02pWV[a\x02<V[a\x028V[a\x024V[\x90V[a\x02\x8B\x81a\x02\x7FV[\x03a\x02\x92WV[_\x80\xFD[\x90P5\x90a\x02\xA3\x82a\x02\x82V[V[\x91`@\x83\x83\x03\x12a\x02\xE5W_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02\xE0Wa\x02\xD2\x83a\x02\xDD\x92\x86\x01a\x02@V[\x93\x90\x94` \x01a\x02\x96V[\x90V[a\x020V[a\x02,V[_\x01\x90V[4a\x03\x1EWa\x03\x08a\x03\x026`\x04a\x02\xA5V[\x91a\x10\xF4V[a\x03\x10a\x02\"V[\x80a\x03\x1A\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[_\x91\x03\x12a\x03-WV[a\x02,V[a\x03;\x90a\x02\x7FV[\x90RV[\x90``\x80a\x03\x85\x93a\x03W_\x82\x01Q_\x86\x01\x90a\x032V[a\x03i` \x82\x01Q` \x86\x01\x90a\x032V[a\x03{`@\x82\x01Q`@\x86\x01\x90a\x032V[\x01Q\x91\x01\x90a\x032V[V[\x91\x90a\x03\x9A\x90_`\x80\x85\x01\x94\x01\x90a\x03?V[V[4a\x03\xCCWa\x03\xAC6`\x04a\x03#V[a\x03\xC8a\x03\xB7a\x11\xADV[a\x03\xBFa\x02\"V[\x91\x82\x91\x82a\x03\x87V[\x03\x90\xF3[a\x02(V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04\x0BW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04\x06W` \x01\x92` \x83\x02\x84\x01\x11a\x04\x01WV[a\x02<V[a\x028V[a\x024V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04JW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04EW` \x01\x92` \x83\x02\x84\x01\x11a\x04@WV[a\x02<V[a\x028V[a\x024V[\x90\x91`@\x82\x84\x03\x12a\x04\xA9W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xA4W\x83a\x04z\x91\x84\x01a\x03\xD1V[\x92\x90\x93` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x9FWa\x04\x9B\x92\x01a\x04\x10V[\x90\x91V[a\x020V[a\x020V[a\x02,V[4a\x04\xE0Wa\x04\xCAa\x04\xC16`\x04a\x04OV[\x92\x91\x90\x91a\x13QV[a\x04\xD2a\x02\"V[\x80a\x04\xDC\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[\x15\x15\x90V[a\x04\xF3\x90a\x04\xE5V[\x90RV[\x91\x90a\x05\n\x90_` \x85\x01\x94\x01\x90a\x04\xEAV[V[4a\x05<Wa\x05\x1C6`\x04a\x03#V[a\x058a\x05'a\x14vV[a\x05/a\x02\"V[\x91\x82\x91\x82a\x04\xF7V[\x03\x90\xF3[a\x02(V[4a\x05pWa\x05Za\x05T6`\x04a\x02\xA5V[\x91a\x15\x83V[a\x05ba\x02\"V[\x80a\x05l\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[\x90` \x82\x82\x03\x12a\x05\xA6W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05\xA1Wa\x05\x9D\x92\x01a\x02@V[\x90\x91V[a\x020V[a\x02,V[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x05\xECa\x05\xF5` \x93a\x05\xFA\x93a\x05\xE3\x81a\x05\xABV[\x93\x84\x80\x93a\x05\xAFV[\x95\x86\x91\x01a\x05\xB8V[a\x05\xC3V[\x01\x90V[a\x06\x13\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x05\xCDV[\x90V[4a\x06GWa\x06Ca\x062a\x06,6`\x04a\x05uV[\x90a\x16\x13V[a\x06:a\x02\"V[\x91\x82\x91\x82a\x05\xFEV[\x03\x90\xF3[a\x02(V[\x1C\x90V[`\xFF\x16\x90V[a\x06f\x90`\x08a\x06k\x93\x02a\x06LV[a\x06PV[\x90V[\x90a\x06y\x91Ta\x06VV[\x90V[a\x06\x88`\x03_\x90a\x06nV[\x90V[4a\x06\xBBWa\x06\x9B6`\x04a\x03#V[a\x06\xB7a\x06\xA6a\x06|V[a\x06\xAEa\x02\"V[\x91\x82\x91\x82a\x04\xF7V[\x03\x90\xF3[a\x02(V[\x90V[\x90V[a\x06\xDAa\x06\xD5a\x06\xDF\x92a\x06\xC0V[a\x06\xC3V[a\x02\x7FV[\x90V[a\x06\xEC`\na\x06\xC6V[\x90V[a\x06\xF7a\x06\xE2V[\x90V[a\x07\x03\x90a\x02\x7FV[\x90RV[\x91\x90a\x07\x1A\x90_` \x85\x01\x94\x01\x90a\x06\xFAV[V[4a\x07LWa\x07,6`\x04a\x03#V[a\x07Ha\x077a\x06\xEFV[a\x07?a\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[4a\x07\x80Wa\x07ja\x07d6`\x04a\x05uV[\x90a\x17\xD5V[a\x07ra\x02\"V[\x80a\x07|\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x07\x99\x90a\x07\x85V[\x90V[a\x07\xA5\x81a\x07\x90V[\x03a\x07\xACWV[_\x80\xFD[\x90P5\x90a\x07\xBD\x82a\x07\x9CV[V[\x91\x90`@\x83\x82\x03\x12a\x07\xE7W\x80a\x07\xDBa\x07\xE4\x92_\x86\x01a\x07\xB0V[\x93` \x01a\x07\xB0V[\x90V[a\x02,V[4a\x08\x1BWa\x08\x05a\x07\xFF6`\x04a\x07\xBFV[\x90a\x19\x86V[a\x08\ra\x02\"V[\x80a\x08\x17\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[\x90` \x82\x82\x03\x12a\x089Wa\x086\x91_\x01a\x02\x96V[\x90V[a\x02,V[4a\x08nWa\x08ja\x08Ya\x08T6`\x04a\x08 V[a\x1A\x15V[a\x08aa\x02\"V[\x91\x82\x91\x82a\x03\x87V[\x03\x90\xF3[a\x02(V[4a\x08\xA1Wa\x08\x836`\x04a\x03#V[a\x08\x8Ba\x1APV[a\x08\x93a\x02\"V[\x80a\x08\x9D\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08\xC1\x90`\x08a\x08\xC6\x93\x02a\x06LV[a\x08\xA6V[\x90V[\x90a\x08\xD4\x91Ta\x08\xB1V[\x90V[a\x08\xE3`\x01_\x90a\x08\xC9V[\x90V[a\x08\xFAa\x08\xF5a\x08\xFF\x92a\x07\x85V[a\x06\xC3V[a\x07\x85V[\x90V[a\t\x0B\x90a\x08\xE6V[\x90V[a\t\x17\x90a\t\x02V[\x90V[a\t#\x90a\t\x0EV[\x90RV[\x91\x90a\t:\x90_` \x85\x01\x94\x01\x90a\t\x1AV[V[4a\tlWa\tL6`\x04a\x03#V[a\tha\tWa\x08\xD7V[a\t_a\x02\"V[\x91\x82\x91\x82a\t'V[\x03\x90\xF3[a\x02(V[\x90V[a\t\x84\x90`\x08a\t\x89\x93\x02a\x06LV[a\tqV[\x90V[\x90a\t\x97\x91Ta\ttV[\x90V[a\t\xA6`\x02_\x90a\t\x8CV[\x90V[4a\t\xD9Wa\t\xB96`\x04a\x03#V[a\t\xD5a\t\xC4a\t\x9AV[a\t\xCCa\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[\x90V[a\t\xF5a\t\xF0a\t\xFA\x92a\t\xDEV[a\x06\xC3V[a\x02\x7FV[\x90V[a\n\tb'\x8D\0a\t\xE1V[\x90V[a\n\x14a\t\xFDV[\x90V[4a\nGWa\n'6`\x04a\x03#V[a\nCa\n2a\n\x0CV[a\n:a\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[4a\nzWa\n\\6`\x04a\x03#V[a\nda\x1A\x7FV[a\nla\x02\"V[\x80a\nv\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[\x91``\x83\x83\x03\x12a\n\xCCWa\n\x96\x82_\x85\x01a\x07\xB0V[\x92a\n\xA4\x83` \x83\x01a\x07\xB0V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xC7Wa\n\xC3\x92\x01a\x02@V[\x90\x91V[a\x020V[a\x02,V[4a\x0B\x05Wa\x0B\x01a\n\xF0a\n\xE76`\x04a\n\x7FV[\x92\x91\x90\x91a\x1B7V[a\n\xF8a\x02\"V[\x91\x82\x91\x82a\x04\xF7V[\x03\x90\xF3[a\x02(V[4a\x0B9Wa\x0B#a\x0B\x1D6`\x04a\x05uV[\x90a\x1C\xBAV[a\x0B+a\x02\"V[\x80a\x0B5\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[4a\x0BnWa\x0BN6`\x04a\x03#V[a\x0Bja\x0BYa\x1C\xD7V[a\x0Baa\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[\x90\x91``\x82\x84\x03\x12a\x0B\xA8Wa\x0B\xA5a\x0B\x8E\x84_\x85\x01a\x02\x96V[\x93a\x0B\x9C\x81` \x86\x01a\x02\x96V[\x93`@\x01a\x02\x96V[\x90V[a\x02,V[4a\x0B\xDEWa\x0B\xDAa\x0B\xC9a\x0B\xC36`\x04a\x0BsV[\x91a\x1D\x9CV[a\x0B\xD1a\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[a\x0B\xF0`\x03`\x01\x90a\x06nV[\x90V[4a\x0C#Wa\x0C\x036`\x04a\x03#V[a\x0C\x1Fa\x0C\x0Ea\x0B\xE3V[a\x0C\x16a\x02\"V[\x91\x82\x91\x82a\x04\xF7V[\x03\x90\xF3[a\x02(V[4a\x0CXWa\x0C86`\x04a\x03#V[a\x0CTa\x0CCa\x1E\x12V[a\x0CKa\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[a\x0Cf\x90a\x07\x90V[\x90RV[\x91\x90a\x0C}\x90_` \x85\x01\x94\x01\x90a\x0C]V[V[4a\x0C\xAFWa\x0C\x8F6`\x04a\x03#V[a\x0C\xABa\x0C\x9Aa\x1EaV[a\x0C\xA2a\x02\"V[\x91\x82\x91\x82a\x0CjV[\x03\x90\xF3[a\x02(V[4a\x0C\xE4Wa\x0C\xC46`\x04a\x03#V[a\x0C\xE0a\x0C\xCFa\x1E\x95V[a\x0C\xD7a\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[4a\r\x19Wa\x0C\xF96`\x04a\x03#V[a\r\x15a\r\x04a\x1E\xE1V[a\r\x0Ca\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[\x90` \x82\x82\x03\x12a\rOW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\rJWa\rF\x92\x01a\x03\xD1V[\x90\x91V[a\x020V[a\x02,V[4a\r\x83Wa\rma\rg6`\x04a\r\x1EV[\x90a \x1DV[a\rua\x02\"V[\x80a\r\x7F\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[\x90` \x82\x82\x03\x12a\r\xA1Wa\r\x9E\x91_\x01a\x07\xB0V[\x90V[a\x02,V[4a\r\xD4Wa\r\xBEa\r\xB96`\x04a\r\x88V[a \xCDV[a\r\xC6a\x02\"V[\x80a\r\xD0\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0E-Wa\x0E\r6`\x04a\x03#V[a\x0E)a\x0E\x18a\r\xD9V[a\x0E a\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[4a\x0E`Wa\x0EB6`\x04a\x03#V[a\x0EJa \xF4V[a\x0ERa\x02\"V[\x80a\x0E\\\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[a\x0Eya\x0Eta\x0E~\x92a\x02\x7FV[a\x06\xC3V[a\x02\x7FV[\x90V[\x90a\x0E\x8B\x90a\x0EeV[_R` R`@_ \x90V[_\x1C\x90V[a\x0E\xA8a\x0E\xAD\x91a\x0E\x97V[a\tqV[\x90V[a\x0E\xBA\x90Ta\x0E\x9CV[\x90V[a\x0E\xC8\x90`\x04a\x0E\x81V[\x90a\x0E\xD4_\x83\x01a\x0E\xB0V[\x91a\x0E\xE1`\x01\x82\x01a\x0E\xB0V[\x91a\x0E\xFA`\x03a\x0E\xF3`\x02\x85\x01a\x0E\xB0V[\x93\x01a\x0E\xB0V[\x90V[a\x0F2a\x0F9\x94a\x0F(``\x94\x98\x97\x95a\x0F\x1E`\x80\x86\x01\x9A_\x87\x01\x90a\x06\xFAV[` \x85\x01\x90a\x06\xFAV[`@\x83\x01\x90a\x06\xFAV[\x01\x90a\x06\xFAV[V[4a\x0FoWa\x0Fka\x0FVa\x0FQ6`\x04a\x08 V[a\x0E\xBDV[\x90a\x0Fb\x94\x92\x94a\x02\"V[\x94\x85\x94\x85a\x0E\xFDV[\x03\x90\xF3[a\x02(V[\x90V[a\x0F\x8Ba\x0F\x86a\x0F\x90\x92a\x0FtV[a\x06\xC3V[a\x02\x7FV[\x90V[a\x0F\x9Ea\x13\x88a\x0FwV[\x90V[a\x0F\xA9a\x0F\x93V[\x90V[4a\x0F\xDCWa\x0F\xBC6`\x04a\x03#V[a\x0F\xD8a\x0F\xC7a\x0F\xA1V[a\x0F\xCFa\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xF3[a\x02(V[4a\x10\x0FWa\x0F\xF9a\x0F\xF46`\x04a\r\x88V[a!cV[a\x10\x01a\x02\"V[\x80a\x10\x0B\x81a\x02\xEAV[\x03\x90\xF3[a\x02(V[_\x80\xFD[\x91\x90a\x105a\x10/32\x90\x86\x85\x91\x92\x90\x91\x92a\x1B7V[\x15a\x04\xE5V[a\x10DWa\x10B\x92a\x10\xA1V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x10\\`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[a\x10i\x90a\t\x02V[\x90V[`@\x90a\x10\x98a\x10\x8Da\x10\x9F\x95\x97\x96\x94``\x84\x01\x90\x84\x82\x03_\x86\x01Ra\x05\xCDV[\x96` \x83\x01\x90a\x06\xFAV[\x01\x90a\x06\xFAV[V[\x90a\x10\xAD\x903\x92a\x16\x13V[\x91B\x92a\x10\xEFa\x10\xDD\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x10`V[\x94a\x10\xE6a\x02\"V[\x93\x84\x93\x84a\x10lV[\x03\x90\xA2V[\x90a\x10\xFF\x92\x91a\x10\x18V[V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x11\x1F\x90a\x05\xC3V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x119W`@RV[a\x11\x01V[\x90a\x11Qa\x11Ja\x02\"V[\x92\x83a\x11\x15V[V[a\x11]`\x80a\x11>V[\x90V[_\x90V[a\x11la\x11SV[\x90` \x80\x80\x80\x85a\x11{a\x11`V[\x81R\x01a\x11\x86a\x11`V[\x81R\x01a\x11\x91a\x11`V[\x81R\x01a\x11\x9Ca\x11`V[\x81RPPV[a\x11\xAAa\x11dV[\x90V[a\x11\xB5a\x11\xA2V[Pa\x11\xBEa!\xB1V[\x90V[P\x90V[P\x90V[` \x91\x81R\x01\x90V[` \x7Fve the same length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x7FData and priority arrays must ha_\x82\x01R\x01RV[a\x12,`2`@\x92a\x11\xC9V[a\x125\x81a\x11\xD2V[\x01\x90V[a\x12N\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x12\x1FV[\x90V[\x15a\x12XWV[a\x12`a\x02\"V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x12v`\x04\x82\x01a\x129V[\x03\x90\xFD[\x90V[a\x12\x91a\x12\x8Ca\x12\x96\x92a\x12zV[a\x06\xC3V[a\x02\x7FV[\x90V[`\x01a\x12\xA5\x91\x01a\x02\x7FV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x13\nW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x13\x05W` \x01\x91`\x01\x82\x026\x03\x83\x13a\x13\0WV[a\x12\xC4V[a\x12\xC0V[a\x12\xBCV[\x90\x82\x10\x15a\x13*W` a\x13&\x92\x02\x81\x01\x90a\x12\xC8V[\x90\x91V[a\x12\xA8V[\x91\x90\x81\x10\x15a\x13?W` \x02\x01\x90V[a\x12\xA8V[5a\x13N\x81a\x02\x82V[\x90V[\x90\x92a\x13^\x82\x85\x90a\x11\xC1V[\x93a\x13\x85\x85a\x13\x7Fa\x13ya\x13t\x88\x87\x90a\x11\xC5V[a\x02\x7FV[\x91a\x02\x7FV[\x14a\x12QV[a\x13\x8E_a\x12}V[[\x80a\x13\xA2a\x13\x9C\x88a\x02\x7FV[\x91a\x02\x7FV[\x10\x15a\x14IWa\x13\xD0\x90a\x13\xC632\x90a\x13\xBE\x88\x87\x86\x91a\x13\x0FV[\x92\x90\x91a\x1B7V[a\x13\xD5W[a\x12\x99V[a\x13\x8FV[3a\x13\xEBa\x13\xE5\x87\x86\x85\x91a\x13\x0FV[\x90a\x16\x13V[\x90a\x14\0a\x13\xFB\x89\x88\x86\x91a\x13/V[a\x13DV[B\x92a\x14Aa\x14/\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x10`V[\x94a\x148a\x02\"V[\x93\x84\x93\x84a\x10lV[\x03\x90\xA2a\x13\xCBV[PPPPPPV[_\x90V[a\x14aa\x14f\x91a\x0E\x97V[a\x06PV[\x90V[a\x14s\x90Ta\x14UV[\x90V[a\x14~a\x14QV[Pa\x14\x89`\x03a\x14iV[\x90V[\x91\x90a\x14\xA9a\x14\xA332\x90\x86\x85\x91\x92\x90\x91\x92a\x1B7V[\x15a\x04\xE5V[a\x14\xB8Wa\x14\xB6\x92a\x157V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x14\xD0`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[\x90\x82_\x93\x92\x827\x01RV[\x91\x90a\x14\xF9\x81a\x14\xF2\x81a\x14\xFE\x95a\x05\xAFV[\x80\x95a\x14\xD4V[a\x05\xC3V[\x01\x90V[a\x15.a\x15#`@\x93a\x155\x96\x98\x97\x95``\x85\x01\x91\x85\x83\x03_\x87\x01Ra\x14\xDFV[\x96` \x83\x01\x90a\x06\xFAV[\x01\x90a\x06\xFAV[V[\x90\x913\x91\x92\x90\x92a\x15~Ba\x15l\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x95a\x10`V[\x95a\x15ua\x02\"V[\x94\x85\x94\x85a\x15\x02V[\x03\x90\xA2V[\x90a\x15\x8E\x92\x91a\x14\x8CV[V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x15\xB8a\x15\xB3a\x15\xBD\x92a\x12zV[a\x15\x9EV[a\x15\x95V[\x90V[\x90V[a\x15\xCFa\x15\xD4\x91a\x15\x95V[a\x15\xC0V[\x90RV[\x90P\x90V[\x90\x91\x82a\x15\xED\x81a\x15\xF4\x93a\x15\xD8V[\x80\x93a\x14\xD4V[\x01\x90V[\x80a\x16\t`\x01\x92a\x16\x10\x96\x94a\x15\xC3V[\x01\x91a\x15\xDDV[\x90V[a\x16Q\x90a\x16\x1Fa\x15\x90V[Pa\x16Ba\x16,_a\x15\xA4V[\x91\x93a\x166a\x02\"V[\x94\x85\x93` \x85\x01a\x15\xF8V[` \x82\x01\x81\x03\x82R\x03\x82a\x11\x15V[\x90V[\x90a\x16pa\x16j32\x90\x85\x85\x91\x92\x90\x91\x92a\x1B7V[\x15a\x04\xE5V[a\x16\x7FWa\x16}\x91a\x17 V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x16\x97`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[`\x08\x1C\x90V[a\x16\xADa\x16\xB2\x91a\x16\x9BV[a\x06PV[\x90V[a\x16\xBF\x90Ta\x16\xA1V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x16\xE5a\x16\xEB\x91\x93\x92\x93a\x02\x7FV[\x92a\x02\x7FV[\x82\x03\x91\x82\x11a\x16\xF6WV[a\x16\xC2V[a\x17\na\x17\x10\x91\x93\x92\x93a\x02\x7FV[\x92a\x02\x7FV[\x82\x01\x80\x92\x11a\x17\x1BWV[a\x16\xC2V[\x90a\x174a\x17.`\x03a\x16\xB5V[\x15a\x04\xE5V[a\x17iWa\x17Ta\x17g\x92a\x17Ma\x17b\x93Z\x92a\x17\x8EV[Z\x90a\x16\xD6V[a\x17\\a\x0F\x93V[\x90a\x16\xFBV[a#\x85V[V[a\x17r\x91a\x17\x8EV[V[\x90\x91a\x17\x8B\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x14\xDFV[\x90V[3\x90\x91a\x17\xBB\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10`V[\x92a\x17\xD0a\x17\xC7a\x02\"V[\x92\x83\x92\x83a\x17tV[\x03\x90\xA2V[\x90a\x17\xDF\x91a\x16TV[V[\x90a\x17\xF3\x91a\x17\xEEa$\x8CV[a\x18\xF9V[V[`\xA0\x1C\x90V[a\x18\x07a\x18\x0C\x91a\x17\xF5V[a\x06PV[\x90V[a\x18\x19\x90Ta\x17\xFBV[\x90V[a\x180a\x18+a\x185\x92a\x12zV[a\x06\xC3V[a\x07\x85V[\x90V[a\x18A\x90a\x18\x1CV[\x90V[`\xA0\x1B\x90V[\x90a\x18Y`\xFF`\xA0\x1B\x91a\x18DV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18l\x90a\x04\xE5V[\x90V[\x90V[\x90a\x18\x87a\x18\x82a\x18\x8E\x92a\x18cV[a\x18oV[\x82Ta\x18JV[\x90UV[a\x18\x9B\x90a\x08\xE6V[\x90V[a\x18\xA7\x90a\x18\x92V[\x90V[_\x1B\x90V[\x90a\x18\xC0`\x01\x80`\xA0\x1B\x03\x91a\x18\xAAV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18\xD3\x90a\x18\x92V[\x90V[\x90V[\x90a\x18\xEEa\x18\xE9a\x18\xF5\x92a\x18\xCAV[a\x18\xD6V[\x82Ta\x18\xAFV[\x90UV[a\x19\x03`\x01a\x18\x0FV[a\x19kW\x81a\x19\"a\x19\x1Ca\x19\x17_a\x188V[a\x07\x90V[\x91a\x07\x90V[\x14a\x19OWa\x19Ha\x19Aa\x19M\x93a\x19<`\x01\x80a\x18rV[a\x18\x9EV[`\x01a\x18\xD9V[a!cV[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x19g`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x19\x82`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[\x90a\x19\x90\x91a\x17\xE1V[V[\x90a\x19\x9C\x90a\x02\x7FV[\x90RV[\x90a\x1A\x07a\x19\xFE`\x03a\x19\xB1a\x11SV[\x94a\x19\xC8a\x19\xC0_\x83\x01a\x0E\xB0V[_\x88\x01a\x19\x92V[a\x19\xE0a\x19\xD7`\x01\x83\x01a\x0E\xB0V[` \x88\x01a\x19\x92V[a\x19\xF8a\x19\xEF`\x02\x83\x01a\x0E\xB0V[`@\x88\x01a\x19\x92V[\x01a\x0E\xB0V[``\x84\x01a\x19\x92V[V[a\x1A\x12\x90a\x19\xA0V[\x90V[a\x1A,a\x1A1\x91a\x1A$a\x11\xA2V[P`\x04a\x0E\x81V[a\x1A\tV[\x90V[a\x1A<a$\x8CV[a\x1ADa\x1AFV[V[a\x1ANa%\x17V[V[a\x1AXa\x1A4V[V[a\x1Aba$\x8CV[a\x1Aja\x1AlV[V[a\x1A}a\x1Ax_a\x188V[a%GV[V[a\x1A\x87a\x1AZV[V[a\x1A\x95a\x1A\x9A\x91a\x0E\x97V[a\x08\xA6V[\x90V[a\x1A\xA7\x90Ta\x1A\x89V[\x90V[`\xE0\x1B\x90V[a\x1A\xB9\x81a\x04\xE5V[\x03a\x1A\xC0WV[_\x80\xFD[\x90PQ\x90a\x1A\xD1\x82a\x1A\xB0V[V[\x90` \x82\x82\x03\x12a\x1A\xECWa\x1A\xE9\x91_\x01a\x1A\xC4V[\x90V[a\x02,V[a\x1B\x17a\x1B$\x95\x93\x94\x92\x94a\x1B\r``\x84\x01\x96_\x85\x01\x90a\x0C]V[` \x83\x01\x90a\x0C]V[`@\x81\x85\x03\x91\x01Ra\x14\xDFV[\x90V[a\x1B/a\x02\"V[=_\x82>=\x90\xFD[\x92a\x1Bz` \x93\x94a\x1BGa\x14QV[Pa\x1B\x85a\x1B]a\x1BX`\x01a\x1A\x9DV[a\t\x0EV[\x93cz9y\xDC\x92\x95\x97a\x1Bna\x02\"V[\x98\x89\x97\x88\x96\x87\x96a\x1A\xAAV[\x86R`\x04\x86\x01a\x1A\xF1V[\x03\x91Z\xFA\x90\x81\x15a\x1B\xC9W_\x91a\x1B\x9BW[P\x90V[a\x1B\xBC\x91P` =\x81\x11a\x1B\xC2W[a\x1B\xB4\x81\x83a\x11\x15V[\x81\x01\x90a\x1A\xD3V[_a\x1B\x97V[P=a\x1B\xAAV[a\x1B'V[\x90a\x1B\xEAa\x1B\xE432\x90\x85\x85\x91\x92\x90\x91\x92a\x1B7V[\x15a\x04\xE5V[a\x1B\xF9Wa\x1B\xF7\x91a\x1C\x15V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x1C\x11`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[\x90a\x1C)a\x1C#`\x03a\x16\xB5V[\x15a\x04\xE5V[a\x1C^Wa\x1CIa\x1C\\\x92a\x1CBa\x1CW\x93Z\x92a\x1CiV[Z\x90a\x16\xD6V[a\x1CQa\x0F\x93V[\x90a\x16\xFBV[a#\x85V[V[a\x1Cg\x91a\x1CiV[V[\x90a\x1Cu\x903\x92a\x16\x13V[\x90a\x1C\xB5a\x1C\xA3\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10`V[\x92a\x1C\xACa\x02\"V[\x91\x82\x91\x82a\x05\xFEV[\x03\x90\xA2V[\x90a\x1C\xC4\x91a\x1B\xCEV[V[_\x90V[a\x1C\xD4\x90Qa\x02\x7FV[\x90V[a\x1C\xDFa\x1C\xC6V[Pa\x1C\xE8a!\xB1V[a\x1C\xF3_\x82\x01a\x1C\xCAV[a\x1D\x05a\x1C\xFF_a\x12}V[\x91a\x02\x7FV[\x14a\x1DZWa\x1D\x18_a\x1D&\x92\x01a\x1C\xCAV[a\x1D a\t\xFDV[\x90a\x16\xFBV[Ba\x1D9a\x1D3\x83a\x02\x7FV[\x91a\x02\x7FV[\x10\x15a\x1DMWa\x1DJ\x90B\x90a\x16\xD6V[\x90V[Pa\x1DW_a\x12}V[\x90V[Pa\x1Dd_a\x12}V[\x90V[a\x1Dva\x1D|\x91\x93\x92\x93a\x02\x7FV[\x92a\x02\x7FV[\x91a\x1D\x88\x83\x82\x02a\x02\x7FV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1D\x97WV[a\x16\xC2V[\x91a\x1D\xA5a\x1C\xC6V[P\x80a\x1D\xB9a\x1D\xB3\x84a\x02\x7FV[\x91a\x02\x7FV[\x11\x15a\x1E\rWa\x1D\xDA\x91a\x1D\xCC\x91a\x16\xD6V[a\x1D\xD4a\x06\xE2V[\x90a\x1DgV[\x80a\x1D\xEDa\x1D\xE7\x84a\x02\x7FV[\x91a\x02\x7FV[\x10\x15a\x1D\xFFWa\x1D\xFC\x91a\x16\xD6V[\x90V[PPa\x1E\n_a\x12}V[\x90V[PP\x90V[a\x1E\x1Aa\x1C\xC6V[Pa\x1E.``a\x1E(a!\xB1V[\x01a\x1C\xCAV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1ELa\x1EQ\x91a\x0E\x97V[a\x1E5V[\x90V[a\x1E^\x90Ta\x1E@V[\x90V[a\x1Eia\x1E1V[Pa\x1Es_a\x1ETV[\x90V[\x90V[a\x1E\x8Da\x1E\x88a\x1E\x92\x92a\x1EvV[a\x06\xC3V[a\x02\x7FV[\x90V[a\x1E\x9Da\x1C\xC6V[Pa\x1E\xB1a\x1E\xAB`\x03a\x14iV[\x15a\x04\xE5V[a\x1E\xD5Wa\x1E\xD2a\x1E\xC2`\x02a\x0E\xB0V[a\x1E\xCC`\x01a\x1EyV[\x90a\x16\xFBV[\x90V[a\x1E\xDE_a\x12}V[\x90V[a\x1E\xE9a\x1C\xC6V[Pa\x1E\xFD`@a\x1E\xF7a!\xB1V[\x01a\x1C\xCAV[\x90V[\x90a\x1F\x14a\x1F\x0E`\x03a\x16\xB5V[\x15a\x04\xE5V[a\x1FIWa\x1F4a\x1FG\x92a\x1F-a\x1FB\x93Z\x92a\x1FTV[Z\x90a\x16\xD6V[a\x1F<a\x0F\x93V[\x90a\x16\xFBV[a#\x85V[V[a\x1FR\x91a\x1FTV[V[a\x1F_\x81\x83\x90a\x11\xC1V[\x91a\x1Fha\x1C\xC6V[Pa\x1Fr_a\x12}V[[\x80a\x1F\x86a\x1F\x80\x86a\x02\x7FV[\x91a\x02\x7FV[\x10\x15a \x17Wa\x1F\xB4\x90a\x1F\xAA32\x90a\x1F\xA2\x87\x87\x86\x91a\x13\x0FV[\x92\x90\x91a\x1B7V[a\x1F\xB9W[a\x12\x99V[a\x1FsV[3a\x1F\xCFa\x1F\xC9\x86\x86\x85\x91a\x13\x0FV[\x90a\x16\x13V[\x90a \x0Fa\x1F\xFD\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10`V[\x92a \x06a\x02\"V[\x91\x82\x91\x82a\x05\xFEV[\x03\x90\xA2a\x1F\xAFV[PPPPV[\x90a '\x91a\x1F\0V[V[a :\x90a 5a$\x8CV[a <V[V[\x80a Wa Qa L_a\x188V[a\x07\x90V[\x91a\x07\x90V[\x14a \xB1Wa oa h\x82a\x18\x9EV[`\x01a\x18\xD9V[a \x99\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10`V[\x90a \xA2a\x02\"V[\x80a \xAC\x81a\x02\xEAV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a \xC9`\x04\x82\x01a\x02\xEAV[\x03\x90\xFD[a \xD6\x90a )V[V[a \xE0a$\x8CV[a \xE8a \xEAV[V[a \xF2a%\xA6V[V[a \xFCa \xD8V[V[a!\x0F\x90a!\na$\x8CV[a!\x11V[V[\x80a!,a!&a!!_a\x188V[a\x07\x90V[\x91a\x07\x90V[\x14a!<Wa!:\x90a%GV[V[a!_a!H_a\x188V[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x0CjV[\x03\x90\xFD[a!l\x90a \xFEV[V[a!x`\x80a\x11>V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a!\x9Ba!\xA1\x91a\x02\x7FV[\x91a\x02\x7FV[\x90\x81\x15a!\xACW\x04\x90V[a!{V[a!\xB9a\x11\xA2V[Pa!\xCDa!\xC7`\x03a\x14iV[\x15a\x04\xE5V[a\"\xC9Wa!\xEEa!\xE9`\x04a!\xE3`\x02a\x0E\xB0V[\x90a\x0E\x81V[a\x1A\tV[Ba\"\x1Ca\"\x16a\"\x11a\"\x03_\x86\x01a\x1C\xCAV[a\"\x0Ba\t\xFDV[\x90a\x16\xFBV[a\x02\x7FV[\x91a\x02\x7FV[\x10\x15a\"%W\x90V[a\"r\x90a\"la\"]_a\"Va\"HBa\"B\x84\x88\x01a\x1C\xCAV[\x90a\x16\xD6V[a\"Pa\t\xFDV[\x90a!\x8FV[\x93\x01a\x1C\xCAV[\x91a\"fa\t\xFDV[\x90a\x1DgV[\x90a\x16\xFBV[a\"\xC6a\"\xBD_a\"\xB8a\"\xAF_a\"\xAAa\"\xA1_\x95a\"\x9Ca\"\x93a!nV[\x9A_\x8C\x01a\x19\x92V[a\x12}V[` \x89\x01a\x19\x92V[a\x12}V[`@\x86\x01a\x19\x92V[a\x12}V[``\x83\x01a\x19\x92V[\x90V[_a#&a#\x1D_a#\x18a#\x0F_a#\na#\x01_\x95a\"\xFCa\"\xF4a\"\xEEa!nV[\x9Ba\x12}V[_\x8C\x01a\x19\x92V[a\x12}V[` \x89\x01a\x19\x92V[a\x12}V[`@\x86\x01a\x19\x92V[a\x12}V[``\x83\x01a\x19\x92V[\x90V[\x90a#5_\x19\x91a\x18\xAAV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a#Wa#Ra#^\x92a\x0EeV[a#?V[\x82Ta#)V[\x90UV[\x91` a#\x83\x92\x94\x93a#|`@\x82\x01\x96_\x83\x01\x90a\x06\xFAV[\x01\x90a\x06\xFAV[V[a#\x98a#\x92`\x03a\x16\xB5V[\x15a\x04\xE5V[a$\x89Wa#\xAFa#\xA9`\x03a\x14iV[\x15a\x04\xE5V[a$|W[a#\xBCa'|V[a$-a#\xCA\x82:\x90a\x1DgV[a#\xFD\x83a#\xF7`\x02a#\xE7`\x04a#\xE1\x83a\x0E\xB0V[\x90a\x0E\x81V[\x01\x91a#\xF2\x83a\x0E\xB0V[a\x16\xFBV[\x90a#BV[a$'`\x03a$\x17`\x04a$\x11`\x02a\x0E\xB0V[\x90a\x0E\x81V[\x01\x91a$\"\x83a\x0E\xB0V[a\x16\xFBV[\x90a#BV[a$7`\x02a\x0E\xB0V[:a$b\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0EeV[\x92a$wa$na\x02\"V[\x92\x83\x92\x83a#bV[\x03\x90\xA2V[a$\x84a&qV[a#\xB4V[PV[a$\x94a\x1EaV[a$\xADa$\xA7a$\xA2a)TV[a\x07\x90V[\x91a\x07\x90V[\x03a$\xB4WV[a$\xD6a$\xBFa)TV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\x0CjV[\x03\x90\xFD[`\x08\x1B\x90V[\x90a$\xEDa\xFF\0\x91a$\xDAV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a%\x0Ca%\x07a%\x13\x92a\x18cV[a\x18oV[\x82Ta$\xE0V[\x90UV[a%\"_`\x03a$\xF7V[V[\x90V[\x90a%<a%7a%C\x92a\x10`V[a%$V[\x82Ta\x18\xAFV[\x90UV[a%P_a\x1ETV[a%Z\x82_a%'V[\x90a%\x8Ea%\x88\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10`V[\x91a\x10`V[\x91a%\x97a\x02\"V[\x80a%\xA1\x81a\x02\xEAV[\x03\x90\xA3V[a%\xB2`\x01`\x03a$\xF7V[V[\x90a%\xC0`\xFF\x91a\x18\xAAV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a%\xDFa%\xDAa%\xE6\x92a\x18cV[a\x18oV[\x82Ta%\xB4V[\x90UV[\x90a%\xF4\x90a\x12}V[_R` R`@_ \x90V[\x90a&]```\x03a&c\x94a&#_\x82\x01a&\x1D_\x88\x01a\x1C\xCAV[\x90a#BV[a&<`\x01\x82\x01a&6` \x88\x01a\x1C\xCAV[\x90a#BV[a&U`\x02\x82\x01a&O`@\x88\x01a\x1C\xCAV[\x90a#BV[\x01\x92\x01a\x1C\xCAV[\x90a#BV[V[\x90a&o\x91a&\0V[V[a&\x84a&~`\x03a\x14iV[\x15a\x04\xE5V[a&\x8BW[V[a&\x97`\x01`\x03a%\xCAV[a&\xAAa&\xA3_a\x12}V[`\x02a#BV[a'\x13Ba'\x02a&\xF9_a&\xF4a&\xEB_a&\xE6a&\xDD_\x95a&\xD8a&\xCFa!nV[\x9A_\x8C\x01a\x19\x92V[a\x12}V[` \x89\x01a\x19\x92V[a\x12}V[`@\x86\x01a\x19\x92V[a\x12}V[``\x83\x01a\x19\x92V[a'\x0E`\x04_\x90a%\xEAV[a&eV[_B\x90a'Ua'C\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x12}V[\x92a'La\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xA2a&\x89V[\x90V[a'i\x90a\x02\x7FV[_\x19\x81\x14a'wW`\x01\x01\x90V[a\x16\xC2V[a'\x99a'\x94`\x04a'\x8E`\x02a\x0E\xB0V[\x90a\x0E\x81V[a']V[Ba'\xC7a'\xC1a'\xBCa'\xAE_\x86\x01a\x0E\xB0V[a'\xB6a\t\xFDV[\x90a\x16\xFBV[a\x02\x7FV[\x91a\x02\x7FV[\x10\x15a'\xD1W[PV[a'\xF9a'\xF0a'\xE2_\x84\x01a\x0E\xB0V[a'\xEAa\t\xFDV[\x90a\x16\xFBV[`\x01\x83\x01a#BV[a(\x03`\x02a\x0E\xB0V[a(0a(\x12`\x02\x84\x01a\x0E\xB0V[\x92a(*_a(#`\x01\x84\x01a\x0E\xB0V[\x92\x01a\x0E\xB0V[\x90a\x16\xD6V[a(Z\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0EeV[\x92a(oa(fa\x02\"V[\x92\x83\x92\x83a#bV[\x03\x90\xA2a(\x8Ea(\x87a(\x82`\x02a\x0E\xB0V[a'`V[`\x02a#BV[a)\0Ba(\xE6a(\xDD_a(\xD8a(\xCF_a(\xCAa(\xC1_\x95a(\xBCa(\xB3a!nV[\x9A_\x8C\x01a\x19\x92V[a\x12}V[` \x89\x01a\x19\x92V[a\x12}V[`@\x86\x01a\x19\x92V[a\x12}V[``\x83\x01a\x19\x92V[a(\xFB`\x04a(\xF5`\x02a\x0E\xB0V[\x90a\x0E\x81V[a&eV[a)\n`\x02a\x0E\xB0V[B\x90a)Ka)9\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0EeV[\x92a)Ba\x02\"V[\x91\x82\x91\x82a\x07\x07V[\x03\x90\xA2_a'\xCEV[a)\\a\x1E1V[P3\x90V",
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
    /**Function with signature `disableGasTracking()` and selector `0x5467cb48`.
```solidity
function disableGasTracking() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableGasTrackingCall {}
    ///Container type for the return parameters of the [`disableGasTracking()`](disableGasTrackingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableGasTrackingReturn {}
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
            impl ::core::convert::From<disableGasTrackingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: disableGasTrackingCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disableGasTrackingCall {
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
            impl ::core::convert::From<disableGasTrackingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: disableGasTrackingReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disableGasTrackingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disableGasTrackingCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = disableGasTrackingReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disableGasTracking()";
            const SELECTOR: [u8; 4] = [84u8, 103u8, 203u8, 72u8];
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
    /**Function with signature `enableGasTracking()` and selector `0xde1f453e`.
```solidity
function enableGasTracking() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableGasTrackingCall {}
    ///Container type for the return parameters of the [`enableGasTracking()`](enableGasTrackingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableGasTrackingReturn {}
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
            impl ::core::convert::From<enableGasTrackingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: enableGasTrackingCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for enableGasTrackingCall {
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
            impl ::core::convert::From<enableGasTrackingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: enableGasTrackingReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for enableGasTrackingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for enableGasTrackingCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = enableGasTrackingReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "enableGasTracking()";
            const SELECTOR: [u8; 4] = [222u8, 31u8, 69u8, 62u8];
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
    /**Function with signature `gasTrackingEnabled()` and selector `0x84fab62b`.
```solidity
function gasTrackingEnabled() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasTrackingEnabledCall {}
    ///Container type for the return parameters of the [`gasTrackingEnabled()`](gasTrackingEnabledCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasTrackingEnabledReturn {
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
            impl ::core::convert::From<gasTrackingEnabledCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: gasTrackingEnabledCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for gasTrackingEnabledCall {
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
            impl ::core::convert::From<gasTrackingEnabledReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: gasTrackingEnabledReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for gasTrackingEnabledReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for gasTrackingEnabledCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = gasTrackingEnabledReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "gasTrackingEnabled()";
            const SELECTOR: [u8; 4] = [132u8, 250u8, 182u8, 43u8];
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
function periods(uint256) external view returns (uint256 startTimestamp, uint256 endTimestamp, uint256 totalGasUsed, uint256 totalGasCost);
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
        pub totalGasCost: alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<periodsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: periodsReturn) -> Self {
                    (
                        value.startTimestamp,
                        value.endTimestamp,
                        value.totalGasUsed,
                        value.totalGasCost,
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
                        totalGasCost: tuple.3,
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
                alloy::sol_types::sol_data::Uint<256>,
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
        disableGasTracking(disableGasTrackingCall),
        #[allow(missing_docs)]
        enableGasTracking(enableGasTrackingCall),
        #[allow(missing_docs)]
        gasTrackingEnabled(gasTrackingEnabledCall),
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
            [84u8, 103u8, 203u8, 72u8],
            [91u8, 60u8, 214u8, 226u8],
            [97u8, 84u8, 56u8, 1u8],
            [101u8, 88u8, 149u8, 79u8],
            [113u8, 80u8, 24u8, 166u8],
            [122u8, 57u8, 121u8, 220u8],
            [128u8, 78u8, 81u8, 35u8],
            [130u8, 244u8, 74u8, 222u8],
            [131u8, 211u8, 193u8, 21u8],
            [132u8, 250u8, 182u8, 43u8],
            [141u8, 90u8, 35u8, 155u8],
            [141u8, 165u8, 203u8, 91u8],
            [175u8, 247u8, 76u8, 109u8],
            [198u8, 96u8, 211u8, 243u8],
            [205u8, 175u8, 185u8, 120u8],
            [212u8, 240u8, 235u8, 77u8],
            [216u8, 120u8, 19u8, 66u8],
            [222u8, 31u8, 69u8, 62u8],
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
        const COUNT: usize = 32usize;
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
                Self::disableGasTracking(_) => {
                    <disableGasTrackingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::enableGasTracking(_) => {
                    <enableGasTrackingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::gasTrackingEnabled(_) => {
                    <gasTrackingEnabledCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn disableGasTracking(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <disableGasTrackingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::disableGasTracking,
                            )
                    }
                    disableGasTracking
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
                    fn gasTrackingEnabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <gasTrackingEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::gasTrackingEnabled,
                            )
                    }
                    gasTrackingEnabled
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
                    fn enableGasTracking(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <enableGasTrackingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::enableGasTracking,
                            )
                    }
                    enableGasTracking
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
                Self::disableGasTracking(inner) => {
                    <disableGasTrackingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::enableGasTracking(inner) => {
                    <enableGasTrackingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::gasTrackingEnabled(inner) => {
                    <gasTrackingEnabledCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::disableGasTracking(inner) => {
                    <disableGasTrackingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::enableGasTracking(inner) => {
                    <enableGasTrackingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::gasTrackingEnabled(inner) => {
                    <gasTrackingEnabledCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        ///Creates a new call builder for the [`disableGasTracking`] function.
        pub fn disableGasTracking(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, disableGasTrackingCall, N> {
            self.call_builder(&disableGasTrackingCall {})
        }
        ///Creates a new call builder for the [`enableGasTracking`] function.
        pub fn enableGasTracking(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, enableGasTrackingCall, N> {
            self.call_builder(&enableGasTrackingCall {})
        }
        ///Creates a new call builder for the [`gasTrackingEnabled`] function.
        pub fn gasTrackingEnabled(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, gasTrackingEnabledCall, N> {
            self.call_builder(&gasTrackingEnabledCall {})
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
