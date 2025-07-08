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

    function PRIORITY_DECAY_RATE() external view returns (uint256);
    function appchainId() external view returns (uint256);
    function calculateEffectivePriority(uint256 originalPriority, uint256 submittedTimestamp, uint256 currentTimestamp) external pure returns (uint256);
    function cumulativeGasFees() external view returns (uint256);
    function currentPeriodIndex() external view returns (uint256);
    function disableGasTracking() external;
    function enableGasTracking() external;
    function gasTrackingEnabled() external view returns (bool);
    function gasTrackingInitialized() external view returns (bool);
    function getCumulativeGasFees() external view returns (uint256 totalCost);
    function getCurrentPeriod() external view returns (GasCounter.GasPeriod memory period);
    function getCurrentPeriodGasUsed() external view returns (uint256 totalGas);
    function getCurrentPeriodTimeRemaining() external view returns (uint256 timeRemaining);
    function getGasFeesInRange(uint256 startCumulative, uint256 endCumulative) external pure returns (uint256 feesDuring);
    function getPeriod(uint256 periodIndex) external view returns (GasCounter.GasPeriod memory period);
    function getTotalGasFees() external view returns (uint256 totalCost);
    function getTotalPeriods() external view returns (uint256 totalPeriods);
    function initialize(address admin, address _permissionRequirementModule) external;
    function isAllowed(address proposer, address originator, bytes memory data) external view returns (bool);
    function isGasTrackingInitialized() external view returns (bool initialized);
    function owner() external view returns (address);
    function periodDuration() external view returns (uint256);
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
    "name": "cumulativeGasFees",
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
    "name": "getCumulativeGasFees",
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
    "name": "getGasFeesInRange",
    "inputs": [
      {
        "name": "startCumulative",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "endCumulative",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "feesDuring",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
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
    "name": "periodDuration",
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
    ///0x60a060405234610038576100196100146100e9565b61010a565b61002161003d565b612aad610587823960805181610e060152612aad90f35b610043565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b9061006f90610047565b810190811060018060401b0382111761008757604052565b610051565b9061009f61009861003d565b9283610065565b565b5f80fd5b90565b6100b1816100a5565b036100b857565b5f80fd5b905051906100c9826100a8565b565b906020828203126100e4576100e1915f016100bc565b90565b6100a1565b6101076131f3803803806100fc8161008c565b9283398101906100cb565b90565b610113906101c2565b565b90565b90565b61012f61012a61013492610115565b610118565b6100a5565b90565b60209181520190565b5f7f41707020636861696e2049442063616e6e6f7420626520300000000000000000910152565b6101746018602092610137565b61017d81610140565b0190565b6101969060208101905f818303910152610167565b90565b156101a057565b6101a861003d565b62461bcd60e51b8152806101be60048201610181565b0390fd5b6101ca61029a565b6101e7816101e06101da5f61011b565b916100a5565b1415610199565b608052565b5f1b90565b906101fd5f19916101ec565b9181191691161790565b90565b61021e61021961022392610207565b610118565b6100a5565b90565b90565b9061023e6102396102459261020a565b610226565b82546101f1565b9055565b60081b90565b9061025c61ff0091610249565b9181191691161790565b151590565b61027490610266565b90565b90565b9061028f61028a6102969261026b565b610277565b825461024f565b9055565b6102a261039b565b6102b062278d006002610229565b6102bc6001600461027a565b565b60a01b90565b906102d360ff60a01b916102be565b9181191691161790565b906102f26102ed6102f99261026b565b610277565b82546102c4565b9055565b5f0190565b61030a61003d565b3d5f823e3d90fd5b60018060a01b031690565b61033161032c61033692610312565b610118565b610312565b90565b6103429061031d565b90565b61034e90610339565b90565b9061036260018060a01b03916101ec565b9181191691161790565b61037590610339565b90565b90565b9061039061038b6103979261036c565b610378565b8254610351565b9055565b6103a433610408565b6103af5f60016102dd565b6103b761003d565b6101bf810181811060018060401b03821117610403576103df82916101bf61303484396102fd565b03905ff080156103fe576103f56103fc91610345565b600161037b565b565b610302565b610051565b61041190610469565b565b61042761042261042c92610115565b610118565b610312565b90565b61043890610413565b90565b61044490610312565b90565b6104509061043b565b9052565b9190610467905f60208501940190610447565b565b8061048461047e6104795f61042f565b61043b565b9161043b565b146104945761049290610527565b565b6104b76104a05f61042f565b5f918291631e4fbdf760e01b835260048301610454565b0390fd5b5f1c90565b60018060a01b031690565b6104d76104dc916104bb565b6104c0565b90565b6104e990546104cb565b90565b6104f59061031d565b90565b610501906104ec565b90565b90565b9061051c610517610523926104f8565b610504565b8254610351565b9055565b6105305f6104df565b61053a825f610507565b9061056e6105687f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0936104f8565b916104f8565b9161057761003d565b80610581816102fd565b0390a356fe60806040526004361015610013575b611079565b61001d5f3561023c565b8063050ec13814610237578063086146d21461023257806311992f8c1461022d57806318d5aafe146102285780631c0b636714610223578063366cbab71461021e5780633b6ab2a9146102195780633d44ae8b1461021457806346e2cc091461020f578063485cc9551461020a5780634b2c0706146102055780635467cb48146102005780635b3cd6e2146101fb57806361543801146101f6578063715018a6146101f15780637a3979dc146101ec5780637fbd295e146101e7578063804e5123146101e257806382f44ade146101dd57806383d3c115146101d857806384fab62b146101d35780638d5a239b146101ce5780638da5cb5b146101c9578063aff74c6d146101c4578063b470aade146101bf578063c660d3f3146101ba578063cdafb978146101b5578063d4f0eb4d146101b0578063d8781342146101ab578063de1f453e146101a6578063ea4a1104146101a1578063f2fde38b1461019c578063f7b8935e146101975763ff7b30840361000e57611044565b610fff565b610f9f565b610f66565b610e5d565b610e28565b610dd1565b610d7f565b610d14565b610cdf565b610c9b565b610c66565b610c0f565b610bda565b610b94565b610b25565b610af1565b610abc565b610a83565b6109fe565b6109c9565b61095c565b610893565b61085e565b61080c565b610771565b61073c565b6106ab565b610636565b610561565b61052c565b6104ce565b6103bc565b61030f565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f8301121561029a5781359167ffffffffffffffff831161029557602001926001830284011161029057565b61025c565b610258565b610254565b90565b6102ab8161029f565b036102b257565b5f80fd5b905035906102c3826102a2565b565b91604083830312610305575f83013567ffffffffffffffff8111610300576102f2836102fd928601610260565b9390946020016102b6565b90565b610250565b61024c565b5f0190565b3461033e576103286103223660046102c5565b91611159565b610330610242565b8061033a8161030a565b0390f35b610248565b5f91031261034d57565b61024c565b61035b9061029f565b9052565b906060806103a5936103775f8201515f860190610352565b61038960208201516020860190610352565b61039b60408201516040860190610352565b0151910190610352565b565b91906103ba905f6080850194019061035f565b565b346103ec576103cc366004610343565b6103e86103d7611212565b6103df610242565b918291826103a7565b0390f35b610248565b909182601f8301121561042b5781359167ffffffffffffffff831161042657602001926020830284011161042157565b61025c565b610258565b610254565b909182601f8301121561046a5781359167ffffffffffffffff831161046557602001926020830284011161046057565b61025c565b610258565b610254565b90916040828403126104c9575f82013567ffffffffffffffff81116104c4578361049a9184016103f1565b929093602082013567ffffffffffffffff81116104bf576104bb9201610430565b9091565b610250565b610250565b61024c565b34610500576104ea6104e136600461046f565b929190916113b6565b6104f2610242565b806104fc8161030a565b0390f35b610248565b151590565b61051390610505565b9052565b919061052a905f6020850194019061050a565b565b3461055c5761053c366004610343565b6105586105476114db565b61054f610242565b91829182610517565b0390f35b610248565b346105905761057a6105743660046102c5565b916115e8565b610582610242565b8061058c8161030a565b0390f35b610248565b906020828203126105c6575f82013567ffffffffffffffff81116105c1576105bd9201610260565b9091565b610250565b61024c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61060c61061560209361061a93610603816105cb565b938480936105cf565b958691016105d8565b6105e3565b0190565b6106339160208201915f8184039101526105ed565b90565b346106675761066361065261064c366004610595565b90611678565b61065a610242565b9182918261061e565b0390f35b610248565b1c90565b60ff1690565b61068690600861068b930261066c565b610670565b90565b906106999154610676565b90565b6106a860045f9061068e565b90565b346106db576106bb366004610343565b6106d76106c661069c565b6106ce610242565b91829182610517565b0390f35b610248565b90565b90565b6106fa6106f56106ff926106e0565b6106e3565b61029f565b90565b61070c600a6106e6565b90565b610717610702565b90565b6107239061029f565b9052565b919061073a905f6020850194019061071a565b565b3461076c5761074c366004610343565b61076861075761070f565b61075f610242565b91829182610727565b0390f35b610248565b346107a05761078a610784366004610595565b90611804565b610792610242565b8061079c8161030a565b0390f35b610248565b60018060a01b031690565b6107b9906107a5565b90565b6107c5816107b0565b036107cc57565b5f80fd5b905035906107dd826107bc565b565b919060408382031261080757806107fb610804925f86016107d0565b936020016107d0565b90565b61024c565b3461083b5761082561081f3660046107df565b906119b5565b61082d610242565b806108378161030a565b0390f35b610248565b9060208282031261085957610856915f016102b6565b90565b61024c565b3461088e5761088a610879610874366004610840565b611a44565b610881610242565b918291826103a7565b0390f35b610248565b346108c1576108a3366004610343565b6108ab611a7f565b6108b3610242565b806108bd8161030a565b0390f35b610248565b60018060a01b031690565b6108e19060086108e6930261066c565b6108c6565b90565b906108f491546108d1565b90565b61090360015f906108e9565b90565b61091a61091561091f926107a5565b6106e3565b6107a5565b90565b61092b90610906565b90565b61093790610922565b90565b6109439061092e565b9052565b919061095a905f6020850194019061093a565b565b3461098c5761096c366004610343565b6109886109776108f7565b61097f610242565b91829182610947565b0390f35b610248565b90565b6109a49060086109a9930261066c565b610991565b90565b906109b79154610994565b90565b6109c660035f906109ac565b90565b346109f9576109d9366004610343565b6109f56109e46109ba565b6109ec610242565b91829182610727565b0390f35b610248565b34610a2c57610a0e366004610343565b610a16611aae565b610a1e610242565b80610a288161030a565b0390f35b610248565b91606083830312610a7e57610a48825f85016107d0565b92610a5683602083016107d0565b92604082013567ffffffffffffffff8111610a7957610a759201610260565b9091565b610250565b61024c565b34610ab757610ab3610aa2610a99366004610a31565b92919091611b66565b610aaa610242565b91829182610517565b0390f35b610248565b34610aec57610acc366004610343565b610ae8610ad7611c33565b610adf610242565b91829182610727565b0390f35b610248565b34610b2057610b0a610b04366004610595565b90611d40565b610b12610242565b80610b1c8161030a565b0390f35b610248565b34610b5557610b35366004610343565b610b51610b40611d4c565b610b48610242565b91829182610727565b0390f35b610248565b9091606082840312610b8f57610b8c610b75845f85016102b6565b93610b8381602086016102b6565b936040016102b6565b90565b61024c565b34610bc557610bc1610bb0610baa366004610b5a565b91611e13565b610bb8610242565b91829182610727565b0390f35b610248565b610bd7600460019061068e565b90565b34610c0a57610bea366004610343565b610c06610bf5610bca565b610bfd610242565b91829182610517565b0390f35b610248565b34610c3f57610c1f366004610343565b610c3b610c2a611e89565b610c32610242565b91829182610727565b0390f35b610248565b610c4d906107b0565b9052565b9190610c64905f60208501940190610c44565b565b34610c9657610c76366004610343565b610c92610c81611ed8565b610c89610242565b91829182610c51565b0390f35b610248565b34610ccb57610cab366004610343565b610cc7610cb6611f0c565b610cbe610242565b91829182610727565b0390f35b610248565b610cdc60025f906109ac565b90565b34610d0f57610cef366004610343565b610d0b610cfa610cd0565b610d02610242565b91829182610727565b0390f35b610248565b34610d4457610d24366004610343565b610d40610d2f611f58565b610d37610242565b91829182610727565b0390f35b610248565b90602082820312610d7a575f82013567ffffffffffffffff8111610d7557610d7192016103f1565b9091565b610250565b61024c565b34610dae57610d98610d92366004610d49565b90612083565b610da0610242565b80610daa8161030a565b0390f35b610248565b90602082820312610dcc57610dc9915f016107d0565b90565b61024c565b34610dff57610de9610de4366004610db3565b612133565b610df1610242565b80610dfb8161030a565b0390f35b610248565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610e5857610e38366004610343565b610e54610e43610e04565b610e4b610242565b91829182610727565b0390f35b610248565b34610e8b57610e6d366004610343565b610e7561215a565b610e7d610242565b80610e878161030a565b0390f35b610248565b610ea4610e9f610ea99261029f565b6106e3565b61029f565b90565b90610eb690610e90565b5f5260205260405f2090565b5f1c90565b610ed3610ed891610ec2565b610991565b90565b610ee59054610ec7565b90565b610ef3906005610eac565b90610eff5f8301610edb565b91610f0c60018201610edb565b91610f256003610f1e60028501610edb565b9301610edb565b90565b610f5d610f6494610f53606094989795610f49608086019a5f87019061071a565b602085019061071a565b604083019061071a565b019061071a565b565b34610f9a57610f96610f81610f7c366004610840565b610ee8565b90610f8d949294610242565b94859485610f28565b0390f35b610248565b34610fcd57610fb7610fb2366004610db3565b6121c9565b610fbf610242565b80610fc98161030a565b0390f35b610248565b9190604083820312610ffa5780610fee610ff7925f86016102b6565b936020016102b6565b90565b61024c565b346110305761102c61101b611015366004610fd2565b90612256565b611023610242565b91829182610727565b0390f35b610248565b61104160065f906109ac565b90565b3461107457611054366004610343565b61107061105f611035565b611067610242565b91829182610727565b0390f35b610248565b5f80fd5b919061109a61109433329086859192909192611b66565b15610505565b6110a9576110a792611106565b565b5f631b8e828b60e31b8152806110c16004820161030a565b0390fd5b6110ce90610922565b90565b6040906110fd6110f26111049597969460608401908482035f8601526105ed565b96602083019061071a565b019061071a565b565b90611112903392611678565b9142926111546111427f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f2946110c5565b9461114b610242565b938493846110d1565b0390a2565b90611164929161107d565b565b634e487b7160e01b5f52604160045260245ffd5b90611184906105e3565b810190811067ffffffffffffffff82111761119e57604052565b611166565b906111b66111af610242565b928361117a565b565b6111c260806111a3565b90565b5f90565b6111d16111b8565b906020808080856111e06111c5565b8152016111eb6111c5565b8152016111f66111c5565b8152016112016111c5565b81525050565b61120f6111c9565b90565b61121a611207565b506112236122cb565b90565b5090565b5090565b60209181520190565b60207f7665207468652073616d65206c656e6774680000000000000000000000000000917f4461746120616e64207072696f7269747920617272617973206d7573742068615f8201520152565b611291603260409261122e565b61129a81611237565b0190565b6112b39060208101905f818303910152611284565b90565b156112bd57565b6112c5610242565b62461bcd60e51b8152806112db6004820161129e565b0390fd5b90565b6112f66112f16112fb926112df565b6106e3565b61029f565b90565b600161130a910161029f565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561136f570180359067ffffffffffffffff821161136a5760200191600182023603831361136557565b611329565b611325565b611321565b9082101561138f57602061138b920281019061132d565b9091565b61130d565b91908110156113a4576020020190565b61130d565b356113b3816102a2565b90565b90926113c3828590611226565b936113ea856113e46113de6113d988879061122a565b61029f565b9161029f565b146112b6565b6113f35f6112e2565b5b806114076114018861029f565b9161029f565b10156114ae576114359061142b33329061142388878691611374565b929091611b66565b61143a575b6112fe565b6113f4565b3361145061144a87868591611374565b90611678565b9061146561146089888691611394565b6113a9565b42926114a66114947f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f2946110c5565b9461149d610242565b938493846110d1565b0390a2611430565b505050505050565b5f90565b6114c66114cb91610ec2565b610670565b90565b6114d890546114ba565b90565b6114e36114b6565b506114ee60046114ce565b90565b919061150e61150833329086859192909192611b66565b15610505565b61151d5761151b9261159c565b565b5f631b8e828b60e31b8152806115356004820161030a565b0390fd5b90825f939282370152565b919061155e8161155781611563956105cf565b8095611539565b6105e3565b0190565b61159361158860409361159a9698979560608501918583035f870152611544565b96602083019061071a565b019061071a565b565b909133919290926115e3426115d17f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f2956110c5565b956115da610242565b94859485611567565b0390a2565b906115f392916114f1565b565b606090565b60ff60f81b1690565b60f81b90565b61161d611618611622926112df565b611603565b6115fa565b90565b90565b611634611639916115fa565b611625565b9052565b905090565b909182611652816116599361163d565b8093611539565b0190565b8061166e6001926116759694611628565b0191611642565b90565b6116b6906116846115f5565b506116a76116915f611609565b919361169b610242565b9485936020850161165d565b6020820181038252038261117a565b90565b906116d56116cf33329085859192909192611b66565b15610505565b6116e4576116e291611760565b565b5f631b8e828b60e31b8152806116fc6004820161030a565b0390fd5b60081c90565b61171261171791611700565b610670565b90565b6117249054611706565b90565b634e487b7160e01b5f52601160045260245ffd5b61174a6117509193929361029f565b9261029f565b820391821161175b57565b611727565b9061177461176e600461171a565b15610505565b611798576117969161178a611791925a926117bd565b5a9061173b565b6124a5565b565b6117a1916117bd565b565b90916117ba9260208301925f818503910152611544565b90565b3390916117ea7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926110c5565b926117ff6117f6610242565b928392836117a3565b0390a2565b9061180e916116b9565b565b906118229161181d6125ac565b611928565b565b60a01c90565b61183661183b91611824565b610670565b90565b611848905461182a565b90565b61185f61185a611864926112df565b6106e3565b6107a5565b90565b6118709061184b565b90565b60a01b90565b9061188860ff60a01b91611873565b9181191691161790565b61189b90610505565b90565b90565b906118b66118b16118bd92611892565b61189e565b8254611879565b9055565b6118ca90610906565b90565b6118d6906118c1565b90565b5f1b90565b906118ef60018060a01b03916118d9565b9181191691161790565b611902906118c1565b90565b90565b9061191d611918611924926118f9565b611905565b82546118de565b9055565b611932600161183e565b61199a578161195161194b6119465f611867565b6107b0565b916107b0565b1461197e5761197761197061197c9361196b6001806118a1565b6118cd565b6001611908565b6121c9565b565b5f632e7f3c7f60e11b8152806119966004820161030a565b0390fd5b5f62dc149f60e41b8152806119b16004820161030a565b0390fd5b906119bf91611810565b565b906119cb9061029f565b9052565b90611a36611a2d60036119e06111b8565b946119f76119ef5f8301610edb565b5f88016119c1565b611a0f611a0660018301610edb565b602088016119c1565b611a27611a1e60028301610edb565b604088016119c1565b01610edb565b606084016119c1565b565b611a41906119cf565b90565b611a5b611a6091611a53611207565b506005610eac565b611a38565b90565b611a6b6125ac565b611a73611a75565b565b611a7d612637565b565b611a87611a63565b565b611a916125ac565b611a99611a9b565b565b611aac611aa75f611867565b612667565b565b611ab6611a89565b565b611ac4611ac991610ec2565b6108c6565b90565b611ad69054611ab8565b90565b60e01b90565b611ae881610505565b03611aef57565b5f80fd5b90505190611b0082611adf565b565b90602082820312611b1b57611b18915f01611af3565b90565b61024c565b611b46611b539593949294611b3c60608401965f850190610c44565b6020830190610c44565b6040818503910152611544565b90565b611b5e610242565b3d5f823e3d90fd5b92611ba960209394611b766114b6565b50611bb4611b8c611b876001611acc565b61092e565b93637a3979dc929597611b9d610242565b98899788968796611ad9565b865260048601611b20565b03915afa908115611bf8575f91611bca575b5090565b611beb915060203d8111611bf1575b611be3818361117a565b810190611b02565b5f611bc6565b503d611bd9565b611b56565b5f90565b611c0b905161029f565b90565b611c1d611c239193929361029f565b9261029f565b8201809211611c2e57565b611727565b611c3b611bfd565b50611c62611c496006610edb565b611c5c6060611c566122cb565b01611c01565b90611c0e565b90565b90611c81611c7b33329085859192909192611b66565b15610505565b611c9057611c8e91611cac565b565b5f631b8e828b60e31b815280611ca86004820161030a565b0390fd5b90611cc0611cba600461171a565b15610505565b611ce457611ce291611cd6611cdd925a92611cef565b5a9061173b565b6124a5565b565b611ced91611cef565b565b90611cfb903392611678565b90611d3b611d297f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926110c5565b92611d32610242565b9182918261061e565b0390a2565b90611d4a91611c65565b565b611d54611bfd565b50611d5d6122cb565b611d685f8201611c01565b611d7a611d745f6112e2565b9161029f565b14611dd157611d8d5f611d9d9201611c01565b611d976002610edb565b90611c0e565b42611db0611daa8361029f565b9161029f565b1015611dc457611dc190429061173b565b90565b50611dce5f6112e2565b90565b50611ddb5f6112e2565b90565b611ded611df39193929361029f565b9261029f565b91611dff83820261029f565b928184041490151715611e0e57565b611727565b91611e1c611bfd565b5080611e30611e2a8461029f565b9161029f565b1115611e8457611e5191611e439161173b565b611e4b610702565b90611dde565b80611e64611e5e8461029f565b9161029f565b1015611e7657611e739161173b565b90565b5050611e815f6112e2565b90565b505090565b611e91611bfd565b50611ea56060611e9f6122cb565b01611c01565b90565b5f90565b60018060a01b031690565b611ec3611ec891610ec2565b611eac565b90565b611ed59054611eb7565b90565b611ee0611ea8565b50611eea5f611ecb565b90565b90565b611f04611eff611f0992611eed565b6106e3565b61029f565b90565b611f14611bfd565b50611f28611f2260046114ce565b15610505565b611f4c57611f49611f396003610edb565b611f436001611ef0565b90611c0e565b90565b611f555f6112e2565b90565b611f60611bfd565b50611f746040611f6e6122cb565b01611c01565b90565b90611f8b611f85600461171a565b15610505565b611faf57611fad91611fa1611fa8925a92611fba565b5a9061173b565b6124a5565b565b611fb891611fba565b565b611fc5818390611226565b91611fce611bfd565b50611fd85f6112e2565b5b80611fec611fe68661029f565b9161029f565b101561207d5761201a9061201033329061200887878691611374565b929091611b66565b61201f575b6112fe565b611fd9565b3361203561202f86868591611374565b90611678565b906120756120637f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926110c5565b9261206c610242565b9182918261061e565b0390a2612015565b50505050565b9061208d91611f77565b565b6120a09061209b6125ac565b6120a2565b565b806120bd6120b76120b25f611867565b6107b0565b916107b0565b14612117576120d56120ce826118cd565b6001611908565b6120ff7f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b9916110c5565b90612108610242565b806121128161030a565b0390a2565b5f632e7f3c7f60e11b81528061212f6004820161030a565b0390fd5b61213c9061208f565b565b6121466125ac565b61214e612150565b565b6121586126c6565b565b61216261213e565b565b612175906121706125ac565b612177565b565b8061219261218c6121875f611867565b6107b0565b916107b0565b146121a2576121a090612667565b565b6121c56121ae5f611867565b5f918291631e4fbdf760e01b835260048301610c51565b0390fd5b6121d290612164565b565b5f7f476173436f756e7465723a20696e76616c69642072616e676500000000000000910152565b612208601960209261122e565b612211816121d4565b0190565b61222a9060208101905f8183039101526121fb565b90565b1561223457565b61223c610242565b62461bcd60e51b81528061225260048201612215565b0390fd5b61228591612262611bfd565b50612280816122796122738561029f565b9161029f565b101561222d565b61173b565b90565b61229260806111a3565b90565b634e487b7160e01b5f52601260045260245ffd5b6122b56122bb9161029f565b9161029f565b9081156122c6570490565b612295565b6122d3611207565b506122e76122e160046114ce565b15610505565b6123e95761230861230360056122fd6003610edb565b90610eac565b611a38565b4261233861233261232d61231d5f8601611c01565b6123276002610edb565b90611c0e565b61029f565b9161029f565b10156123415790565b6123929061238c61237b5f6123746123644261235e848801611c01565b9061173b565b61236e6002610edb565b906122a9565b9301611c01565b916123866002610edb565b90611dde565b90611c0e565b6123e66123dd5f6123d86123cf5f6123ca6123c15f956123bc6123b3612288565b9a5f8c016119c1565b6112e2565b602089016119c1565b6112e2565b604086016119c1565b6112e2565b606083016119c1565b90565b5f61244661243d5f61243861242f5f61242a6124215f9561241c61241461240e612288565b9b6112e2565b5f8c016119c1565b6112e2565b602089016119c1565b6112e2565b604086016119c1565b6112e2565b606083016119c1565b90565b906124555f19916118d9565b9181191691161790565b90565b9061247761247261247e92610e90565b61245f565b8254612449565b9055565b9160206124a392949361249c60408201965f83019061071a565b019061071a565b565b6124b86124b2600461171a565b15610505565b6125a9576124cf6124c960046114ce565b15610505565b61259c575b6124dc61289c565b61254d6124ea823a90611dde565b61251e83612518600261250860056125026003610edb565b90610eac565b019161251383610edb565b611c0e565b90612462565b6125476003612537600561253183610edb565b90610eac565b019161254283610edb565b611c0e565b90612462565b6125576003610edb565b3a6125827f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610e90565b9261259761258e610242565b92839283612482565b0390a2565b6125a4612791565b6124d4565b50565b6125b4611ed8565b6125cd6125c76125c2612aa0565b6107b0565b916107b0565b036125d457565b6125f66125df612aa0565b5f91829163118cdaa760e01b835260048301610c51565b0390fd5b60081b90565b9061260d61ff00916125fa565b9181191691161790565b9061262c61262761263392611892565b61189e565b8254612600565b9055565b6126425f6004612617565b565b90565b9061265c612657612663926110c5565b612644565b82546118de565b9055565b6126705f611ecb565b61267a825f612647565b906126ae6126a87f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0936110c5565b916110c5565b916126b7610242565b806126c18161030a565b0390a3565b6126d260016004612617565b565b906126e060ff916118d9565b9181191691161790565b906126ff6126fa61270692611892565b61189e565b82546126d4565b9055565b90612714906112e2565b5f5260205260405f2090565b9061277d60606003612783946127435f820161273d5f8801611c01565b90612462565b61275c6001820161275660208801611c01565b90612462565b6127756002820161276f60408801611c01565b90612462565b019201611c01565b90612462565b565b9061278f91612720565b565b6127a461279e60046114ce565b15610505565b6127ab575b565b6127b7600160046126ea565b6127ca6127c35f6112e2565b6003612462565b612833426128226128195f61281461280b5f6128066127fd5f956127f86127ef612288565b9a5f8c016119c1565b6112e2565b602089016119c1565b6112e2565b604086016119c1565b6112e2565b606083016119c1565b61282e60055f9061270a565b612785565b5f42906128756128637f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e926112e2565b9261286c610242565b91829182610727565b0390a26127a9565b90565b6128899061029f565b5f1981146128975760010190565b611727565b6128b96128b460056128ae6003610edb565b90610eac565b61287d565b426128e96128e36128de6128ce5f8601610edb565b6128d86002610edb565b90611c0e565b61029f565b9161029f565b10156128f3575b50565b61291d6129146129045f8401610edb565b61290e6002610edb565b90611c0e565b60018301612462565b61294561293e61292f60038401610edb565b6129396006610edb565b611c0e565b6006612462565b61294f6003610edb565b61297c61295e60028401610edb565b926129765f61296f60018401610edb565b9201610edb565b9061173b565b6129a67f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610e90565b926129bb6129b2610242565b92839283612482565b0390a26129da6129d36129ce6003610edb565b612880565b6003612462565b612a4c42612a32612a295f612a24612a1b5f612a16612a0d5f95612a086129ff612288565b9a5f8c016119c1565b6112e2565b602089016119c1565b6112e2565b604086016119c1565b6112e2565b606083016119c1565b612a476005612a416003610edb565b90610eac565b612785565b612a566003610edb565b4290612a97612a857f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610e90565b92612a8e610242565b91829182610727565b0390a25f6128f0565b612aa8611ea8565b50339056608060405234601c57600e6020565b61019461002b823961019490f35b6026565b60405190565b5f80fdfe60806040526004361015610013575b61017b565b61001d5f3561002c565b637a3979dc0361000e57610142565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b60018060a01b031690565b61005890610044565b90565b6100648161004f565b0361006b57565b5f80fd5b9050359061007c8261005b565b565b5f80fd5b5f80fd5b5f80fd5b909182601f830112156100c45781359167ffffffffffffffff83116100bf5760200192600183028401116100ba57565b610086565b610082565b61007e565b91606083830312610116576100e0825f850161006f565b926100ee836020830161006f565b92604082013567ffffffffffffffff81116101115761010d920161008a565b9091565b610040565b61003c565b151590565b6101299061011b565b9052565b9190610140905f60208501940190610120565b565b34610176576101726101616101583660046100c9565b92919091610183565b610169610032565b9182918261012d565b0390f35b610038565b5f80fd5b5f90565b5050505061018f61017f565b505f9056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4a\08Wa\0\x19a\0\x14a\0\xE9V[a\x01\nV[a\0!a\0=V[a*\xADa\x05\x87\x829`\x80Q\x81a\x0E\x06\x01Ra*\xAD\x90\xF3[a\0CV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0o\x90a\0GV[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x87W`@RV[a\0QV[\x90a\0\x9Fa\0\x98a\0=V[\x92\x83a\0eV[V[_\x80\xFD[\x90V[a\0\xB1\x81a\0\xA5V[\x03a\0\xB8WV[_\x80\xFD[\x90PQ\x90a\0\xC9\x82a\0\xA8V[V[\x90` \x82\x82\x03\x12a\0\xE4Wa\0\xE1\x91_\x01a\0\xBCV[\x90V[a\0\xA1V[a\x01\x07a1\xF3\x808\x03\x80a\0\xFC\x81a\0\x8CV[\x92\x839\x81\x01\x90a\0\xCBV[\x90V[a\x01\x13\x90a\x01\xC2V[V[\x90V[\x90V[a\x01/a\x01*a\x014\x92a\x01\x15V[a\x01\x18V[a\0\xA5V[\x90V[` \x91\x81R\x01\x90V[_\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01t`\x18` \x92a\x017V[a\x01}\x81a\x01@V[\x01\x90V[a\x01\x96\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x01gV[\x90V[\x15a\x01\xA0WV[a\x01\xA8a\0=V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x01\xBE`\x04\x82\x01a\x01\x81V[\x03\x90\xFD[a\x01\xCAa\x02\x9AV[a\x01\xE7\x81a\x01\xE0a\x01\xDA_a\x01\x1BV[\x91a\0\xA5V[\x14\x15a\x01\x99V[`\x80RV[_\x1B\x90V[\x90a\x01\xFD_\x19\x91a\x01\xECV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[a\x02\x1Ea\x02\x19a\x02#\x92a\x02\x07V[a\x01\x18V[a\0\xA5V[\x90V[\x90V[\x90a\x02>a\x029a\x02E\x92a\x02\nV[a\x02&V[\x82Ta\x01\xF1V[\x90UV[`\x08\x1B\x90V[\x90a\x02\\a\xFF\0\x91a\x02IV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x15\x15\x90V[a\x02t\x90a\x02fV[\x90V[\x90V[\x90a\x02\x8Fa\x02\x8Aa\x02\x96\x92a\x02kV[a\x02wV[\x82Ta\x02OV[\x90UV[a\x02\xA2a\x03\x9BV[a\x02\xB0b'\x8D\0`\x02a\x02)V[a\x02\xBC`\x01`\x04a\x02zV[V[`\xA0\x1B\x90V[\x90a\x02\xD3`\xFF`\xA0\x1B\x91a\x02\xBEV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x02\xF2a\x02\xEDa\x02\xF9\x92a\x02kV[a\x02wV[\x82Ta\x02\xC4V[\x90UV[_\x01\x90V[a\x03\na\0=V[=_\x82>=\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x031a\x03,a\x036\x92a\x03\x12V[a\x01\x18V[a\x03\x12V[\x90V[a\x03B\x90a\x03\x1DV[\x90V[a\x03N\x90a\x039V[\x90V[\x90a\x03b`\x01\x80`\xA0\x1B\x03\x91a\x01\xECV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x03u\x90a\x039V[\x90V[\x90V[\x90a\x03\x90a\x03\x8Ba\x03\x97\x92a\x03lV[a\x03xV[\x82Ta\x03QV[\x90UV[a\x03\xA43a\x04\x08V[a\x03\xAF_`\x01a\x02\xDDV[a\x03\xB7a\0=V[a\x01\xBF\x81\x01\x81\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\x04\x03Wa\x03\xDF\x82\x91a\x01\xBFa04\x849a\x02\xFDV[\x03\x90_\xF0\x80\x15a\x03\xFEWa\x03\xF5a\x03\xFC\x91a\x03EV[`\x01a\x03{V[V[a\x03\x02V[a\0QV[a\x04\x11\x90a\x04iV[V[a\x04'a\x04\"a\x04,\x92a\x01\x15V[a\x01\x18V[a\x03\x12V[\x90V[a\x048\x90a\x04\x13V[\x90V[a\x04D\x90a\x03\x12V[\x90V[a\x04P\x90a\x04;V[\x90RV[\x91\x90a\x04g\x90_` \x85\x01\x94\x01\x90a\x04GV[V[\x80a\x04\x84a\x04~a\x04y_a\x04/V[a\x04;V[\x91a\x04;V[\x14a\x04\x94Wa\x04\x92\x90a\x05'V[V[a\x04\xB7a\x04\xA0_a\x04/V[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x04TV[\x03\x90\xFD[_\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\xD7a\x04\xDC\x91a\x04\xBBV[a\x04\xC0V[\x90V[a\x04\xE9\x90Ta\x04\xCBV[\x90V[a\x04\xF5\x90a\x03\x1DV[\x90V[a\x05\x01\x90a\x04\xECV[\x90V[\x90V[\x90a\x05\x1Ca\x05\x17a\x05#\x92a\x04\xF8V[a\x05\x04V[\x82Ta\x03QV[\x90UV[a\x050_a\x04\xDFV[a\x05:\x82_a\x05\x07V[\x90a\x05na\x05h\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x04\xF8V[\x91a\x04\xF8V[\x91a\x05wa\0=V[\x80a\x05\x81\x81a\x02\xFDV[\x03\x90\xA3V\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x10yV[a\0\x1D_5a\x02<V[\x80c\x05\x0E\xC18\x14a\x027W\x80c\x08aF\xD2\x14a\x022W\x80c\x11\x99/\x8C\x14a\x02-W\x80c\x18\xD5\xAA\xFE\x14a\x02(W\x80c\x1C\x0Bcg\x14a\x02#W\x80c6l\xBA\xB7\x14a\x02\x1EW\x80c;j\xB2\xA9\x14a\x02\x19W\x80c=D\xAE\x8B\x14a\x02\x14W\x80cF\xE2\xCC\t\x14a\x02\x0FW\x80cH\\\xC9U\x14a\x02\nW\x80cK,\x07\x06\x14a\x02\x05W\x80cTg\xCBH\x14a\x02\0W\x80c[<\xD6\xE2\x14a\x01\xFBW\x80caT8\x01\x14a\x01\xF6W\x80cqP\x18\xA6\x14a\x01\xF1W\x80cz9y\xDC\x14a\x01\xECW\x80c\x7F\xBD)^\x14a\x01\xE7W\x80c\x80NQ#\x14a\x01\xE2W\x80c\x82\xF4J\xDE\x14a\x01\xDDW\x80c\x83\xD3\xC1\x15\x14a\x01\xD8W\x80c\x84\xFA\xB6+\x14a\x01\xD3W\x80c\x8DZ#\x9B\x14a\x01\xCEW\x80c\x8D\xA5\xCB[\x14a\x01\xC9W\x80c\xAF\xF7Lm\x14a\x01\xC4W\x80c\xB4p\xAA\xDE\x14a\x01\xBFW\x80c\xC6`\xD3\xF3\x14a\x01\xBAW\x80c\xCD\xAF\xB9x\x14a\x01\xB5W\x80c\xD4\xF0\xEBM\x14a\x01\xB0W\x80c\xD8x\x13B\x14a\x01\xABW\x80c\xDE\x1FE>\x14a\x01\xA6W\x80c\xEAJ\x11\x04\x14a\x01\xA1W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x9CW\x80c\xF7\xB8\x93^\x14a\x01\x97Wc\xFF{0\x84\x03a\0\x0EWa\x10DV[a\x0F\xFFV[a\x0F\x9FV[a\x0FfV[a\x0E]V[a\x0E(V[a\r\xD1V[a\r\x7FV[a\r\x14V[a\x0C\xDFV[a\x0C\x9BV[a\x0CfV[a\x0C\x0FV[a\x0B\xDAV[a\x0B\x94V[a\x0B%V[a\n\xF1V[a\n\xBCV[a\n\x83V[a\t\xFEV[a\t\xC9V[a\t\\V[a\x08\x93V[a\x08^V[a\x08\x0CV[a\x07qV[a\x07<V[a\x06\xABV[a\x066V[a\x05aV[a\x05,V[a\x04\xCEV[a\x03\xBCV[a\x03\x0FV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x02\x9AW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x02\x95W` \x01\x92`\x01\x83\x02\x84\x01\x11a\x02\x90WV[a\x02\\V[a\x02XV[a\x02TV[\x90V[a\x02\xAB\x81a\x02\x9FV[\x03a\x02\xB2WV[_\x80\xFD[\x90P5\x90a\x02\xC3\x82a\x02\xA2V[V[\x91`@\x83\x83\x03\x12a\x03\x05W_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\0Wa\x02\xF2\x83a\x02\xFD\x92\x86\x01a\x02`V[\x93\x90\x94` \x01a\x02\xB6V[\x90V[a\x02PV[a\x02LV[_\x01\x90V[4a\x03>Wa\x03(a\x03\"6`\x04a\x02\xC5V[\x91a\x11YV[a\x030a\x02BV[\x80a\x03:\x81a\x03\nV[\x03\x90\xF3[a\x02HV[_\x91\x03\x12a\x03MWV[a\x02LV[a\x03[\x90a\x02\x9FV[\x90RV[\x90``\x80a\x03\xA5\x93a\x03w_\x82\x01Q_\x86\x01\x90a\x03RV[a\x03\x89` \x82\x01Q` \x86\x01\x90a\x03RV[a\x03\x9B`@\x82\x01Q`@\x86\x01\x90a\x03RV[\x01Q\x91\x01\x90a\x03RV[V[\x91\x90a\x03\xBA\x90_`\x80\x85\x01\x94\x01\x90a\x03_V[V[4a\x03\xECWa\x03\xCC6`\x04a\x03CV[a\x03\xE8a\x03\xD7a\x12\x12V[a\x03\xDFa\x02BV[\x91\x82\x91\x82a\x03\xA7V[\x03\x90\xF3[a\x02HV[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04+W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04&W` \x01\x92` \x83\x02\x84\x01\x11a\x04!WV[a\x02\\V[a\x02XV[a\x02TV[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04jW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04eW` \x01\x92` \x83\x02\x84\x01\x11a\x04`WV[a\x02\\V[a\x02XV[a\x02TV[\x90\x91`@\x82\x84\x03\x12a\x04\xC9W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xC4W\x83a\x04\x9A\x91\x84\x01a\x03\xF1V[\x92\x90\x93` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xBFWa\x04\xBB\x92\x01a\x040V[\x90\x91V[a\x02PV[a\x02PV[a\x02LV[4a\x05\0Wa\x04\xEAa\x04\xE16`\x04a\x04oV[\x92\x91\x90\x91a\x13\xB6V[a\x04\xF2a\x02BV[\x80a\x04\xFC\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x15\x15\x90V[a\x05\x13\x90a\x05\x05V[\x90RV[\x91\x90a\x05*\x90_` \x85\x01\x94\x01\x90a\x05\nV[V[4a\x05\\Wa\x05<6`\x04a\x03CV[a\x05Xa\x05Ga\x14\xDBV[a\x05Oa\x02BV[\x91\x82\x91\x82a\x05\x17V[\x03\x90\xF3[a\x02HV[4a\x05\x90Wa\x05za\x05t6`\x04a\x02\xC5V[\x91a\x15\xE8V[a\x05\x82a\x02BV[\x80a\x05\x8C\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x90` \x82\x82\x03\x12a\x05\xC6W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05\xC1Wa\x05\xBD\x92\x01a\x02`V[\x90\x91V[a\x02PV[a\x02LV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x06\x0Ca\x06\x15` \x93a\x06\x1A\x93a\x06\x03\x81a\x05\xCBV[\x93\x84\x80\x93a\x05\xCFV[\x95\x86\x91\x01a\x05\xD8V[a\x05\xE3V[\x01\x90V[a\x063\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x05\xEDV[\x90V[4a\x06gWa\x06ca\x06Ra\x06L6`\x04a\x05\x95V[\x90a\x16xV[a\x06Za\x02BV[\x91\x82\x91\x82a\x06\x1EV[\x03\x90\xF3[a\x02HV[\x1C\x90V[`\xFF\x16\x90V[a\x06\x86\x90`\x08a\x06\x8B\x93\x02a\x06lV[a\x06pV[\x90V[\x90a\x06\x99\x91Ta\x06vV[\x90V[a\x06\xA8`\x04_\x90a\x06\x8EV[\x90V[4a\x06\xDBWa\x06\xBB6`\x04a\x03CV[a\x06\xD7a\x06\xC6a\x06\x9CV[a\x06\xCEa\x02BV[\x91\x82\x91\x82a\x05\x17V[\x03\x90\xF3[a\x02HV[\x90V[\x90V[a\x06\xFAa\x06\xF5a\x06\xFF\x92a\x06\xE0V[a\x06\xE3V[a\x02\x9FV[\x90V[a\x07\x0C`\na\x06\xE6V[\x90V[a\x07\x17a\x07\x02V[\x90V[a\x07#\x90a\x02\x9FV[\x90RV[\x91\x90a\x07:\x90_` \x85\x01\x94\x01\x90a\x07\x1AV[V[4a\x07lWa\x07L6`\x04a\x03CV[a\x07ha\x07Wa\x07\x0FV[a\x07_a\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[4a\x07\xA0Wa\x07\x8Aa\x07\x846`\x04a\x05\x95V[\x90a\x18\x04V[a\x07\x92a\x02BV[\x80a\x07\x9C\x81a\x03\nV[\x03\x90\xF3[a\x02HV[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x07\xB9\x90a\x07\xA5V[\x90V[a\x07\xC5\x81a\x07\xB0V[\x03a\x07\xCCWV[_\x80\xFD[\x90P5\x90a\x07\xDD\x82a\x07\xBCV[V[\x91\x90`@\x83\x82\x03\x12a\x08\x07W\x80a\x07\xFBa\x08\x04\x92_\x86\x01a\x07\xD0V[\x93` \x01a\x07\xD0V[\x90V[a\x02LV[4a\x08;Wa\x08%a\x08\x1F6`\x04a\x07\xDFV[\x90a\x19\xB5V[a\x08-a\x02BV[\x80a\x087\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x90` \x82\x82\x03\x12a\x08YWa\x08V\x91_\x01a\x02\xB6V[\x90V[a\x02LV[4a\x08\x8EWa\x08\x8Aa\x08ya\x08t6`\x04a\x08@V[a\x1ADV[a\x08\x81a\x02BV[\x91\x82\x91\x82a\x03\xA7V[\x03\x90\xF3[a\x02HV[4a\x08\xC1Wa\x08\xA36`\x04a\x03CV[a\x08\xABa\x1A\x7FV[a\x08\xB3a\x02BV[\x80a\x08\xBD\x81a\x03\nV[\x03\x90\xF3[a\x02HV[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08\xE1\x90`\x08a\x08\xE6\x93\x02a\x06lV[a\x08\xC6V[\x90V[\x90a\x08\xF4\x91Ta\x08\xD1V[\x90V[a\t\x03`\x01_\x90a\x08\xE9V[\x90V[a\t\x1Aa\t\x15a\t\x1F\x92a\x07\xA5V[a\x06\xE3V[a\x07\xA5V[\x90V[a\t+\x90a\t\x06V[\x90V[a\t7\x90a\t\"V[\x90V[a\tC\x90a\t.V[\x90RV[\x91\x90a\tZ\x90_` \x85\x01\x94\x01\x90a\t:V[V[4a\t\x8CWa\tl6`\x04a\x03CV[a\t\x88a\twa\x08\xF7V[a\t\x7Fa\x02BV[\x91\x82\x91\x82a\tGV[\x03\x90\xF3[a\x02HV[\x90V[a\t\xA4\x90`\x08a\t\xA9\x93\x02a\x06lV[a\t\x91V[\x90V[\x90a\t\xB7\x91Ta\t\x94V[\x90V[a\t\xC6`\x03_\x90a\t\xACV[\x90V[4a\t\xF9Wa\t\xD96`\x04a\x03CV[a\t\xF5a\t\xE4a\t\xBAV[a\t\xECa\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[4a\n,Wa\n\x0E6`\x04a\x03CV[a\n\x16a\x1A\xAEV[a\n\x1Ea\x02BV[\x80a\n(\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x91``\x83\x83\x03\x12a\n~Wa\nH\x82_\x85\x01a\x07\xD0V[\x92a\nV\x83` \x83\x01a\x07\xD0V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\nyWa\nu\x92\x01a\x02`V[\x90\x91V[a\x02PV[a\x02LV[4a\n\xB7Wa\n\xB3a\n\xA2a\n\x996`\x04a\n1V[\x92\x91\x90\x91a\x1BfV[a\n\xAAa\x02BV[\x91\x82\x91\x82a\x05\x17V[\x03\x90\xF3[a\x02HV[4a\n\xECWa\n\xCC6`\x04a\x03CV[a\n\xE8a\n\xD7a\x1C3V[a\n\xDFa\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[4a\x0B Wa\x0B\na\x0B\x046`\x04a\x05\x95V[\x90a\x1D@V[a\x0B\x12a\x02BV[\x80a\x0B\x1C\x81a\x03\nV[\x03\x90\xF3[a\x02HV[4a\x0BUWa\x0B56`\x04a\x03CV[a\x0BQa\x0B@a\x1DLV[a\x0BHa\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[\x90\x91``\x82\x84\x03\x12a\x0B\x8FWa\x0B\x8Ca\x0Bu\x84_\x85\x01a\x02\xB6V[\x93a\x0B\x83\x81` \x86\x01a\x02\xB6V[\x93`@\x01a\x02\xB6V[\x90V[a\x02LV[4a\x0B\xC5Wa\x0B\xC1a\x0B\xB0a\x0B\xAA6`\x04a\x0BZV[\x91a\x1E\x13V[a\x0B\xB8a\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[a\x0B\xD7`\x04`\x01\x90a\x06\x8EV[\x90V[4a\x0C\nWa\x0B\xEA6`\x04a\x03CV[a\x0C\x06a\x0B\xF5a\x0B\xCAV[a\x0B\xFDa\x02BV[\x91\x82\x91\x82a\x05\x17V[\x03\x90\xF3[a\x02HV[4a\x0C?Wa\x0C\x1F6`\x04a\x03CV[a\x0C;a\x0C*a\x1E\x89V[a\x0C2a\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[a\x0CM\x90a\x07\xB0V[\x90RV[\x91\x90a\x0Cd\x90_` \x85\x01\x94\x01\x90a\x0CDV[V[4a\x0C\x96Wa\x0Cv6`\x04a\x03CV[a\x0C\x92a\x0C\x81a\x1E\xD8V[a\x0C\x89a\x02BV[\x91\x82\x91\x82a\x0CQV[\x03\x90\xF3[a\x02HV[4a\x0C\xCBWa\x0C\xAB6`\x04a\x03CV[a\x0C\xC7a\x0C\xB6a\x1F\x0CV[a\x0C\xBEa\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[a\x0C\xDC`\x02_\x90a\t\xACV[\x90V[4a\r\x0FWa\x0C\xEF6`\x04a\x03CV[a\r\x0Ba\x0C\xFAa\x0C\xD0V[a\r\x02a\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[4a\rDWa\r$6`\x04a\x03CV[a\r@a\r/a\x1FXV[a\r7a\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[\x90` \x82\x82\x03\x12a\rzW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\ruWa\rq\x92\x01a\x03\xF1V[\x90\x91V[a\x02PV[a\x02LV[4a\r\xAEWa\r\x98a\r\x926`\x04a\rIV[\x90a \x83V[a\r\xA0a\x02BV[\x80a\r\xAA\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x90` \x82\x82\x03\x12a\r\xCCWa\r\xC9\x91_\x01a\x07\xD0V[\x90V[a\x02LV[4a\r\xFFWa\r\xE9a\r\xE46`\x04a\r\xB3V[a!3V[a\r\xF1a\x02BV[\x80a\r\xFB\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0EXWa\x0E86`\x04a\x03CV[a\x0ETa\x0ECa\x0E\x04V[a\x0EKa\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[4a\x0E\x8BWa\x0Em6`\x04a\x03CV[a\x0Eua!ZV[a\x0E}a\x02BV[\x80a\x0E\x87\x81a\x03\nV[\x03\x90\xF3[a\x02HV[a\x0E\xA4a\x0E\x9Fa\x0E\xA9\x92a\x02\x9FV[a\x06\xE3V[a\x02\x9FV[\x90V[\x90a\x0E\xB6\x90a\x0E\x90V[_R` R`@_ \x90V[_\x1C\x90V[a\x0E\xD3a\x0E\xD8\x91a\x0E\xC2V[a\t\x91V[\x90V[a\x0E\xE5\x90Ta\x0E\xC7V[\x90V[a\x0E\xF3\x90`\x05a\x0E\xACV[\x90a\x0E\xFF_\x83\x01a\x0E\xDBV[\x91a\x0F\x0C`\x01\x82\x01a\x0E\xDBV[\x91a\x0F%`\x03a\x0F\x1E`\x02\x85\x01a\x0E\xDBV[\x93\x01a\x0E\xDBV[\x90V[a\x0F]a\x0Fd\x94a\x0FS``\x94\x98\x97\x95a\x0FI`\x80\x86\x01\x9A_\x87\x01\x90a\x07\x1AV[` \x85\x01\x90a\x07\x1AV[`@\x83\x01\x90a\x07\x1AV[\x01\x90a\x07\x1AV[V[4a\x0F\x9AWa\x0F\x96a\x0F\x81a\x0F|6`\x04a\x08@V[a\x0E\xE8V[\x90a\x0F\x8D\x94\x92\x94a\x02BV[\x94\x85\x94\x85a\x0F(V[\x03\x90\xF3[a\x02HV[4a\x0F\xCDWa\x0F\xB7a\x0F\xB26`\x04a\r\xB3V[a!\xC9V[a\x0F\xBFa\x02BV[\x80a\x0F\xC9\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x91\x90`@\x83\x82\x03\x12a\x0F\xFAW\x80a\x0F\xEEa\x0F\xF7\x92_\x86\x01a\x02\xB6V[\x93` \x01a\x02\xB6V[\x90V[a\x02LV[4a\x100Wa\x10,a\x10\x1Ba\x10\x156`\x04a\x0F\xD2V[\x90a\"VV[a\x10#a\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[a\x10A`\x06_\x90a\t\xACV[\x90V[4a\x10tWa\x10T6`\x04a\x03CV[a\x10pa\x10_a\x105V[a\x10ga\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[_\x80\xFD[\x91\x90a\x10\x9Aa\x10\x9432\x90\x86\x85\x91\x92\x90\x91\x92a\x1BfV[\x15a\x05\x05V[a\x10\xA9Wa\x10\xA7\x92a\x11\x06V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x10\xC1`\x04\x82\x01a\x03\nV[\x03\x90\xFD[a\x10\xCE\x90a\t\"V[\x90V[`@\x90a\x10\xFDa\x10\xF2a\x11\x04\x95\x97\x96\x94``\x84\x01\x90\x84\x82\x03_\x86\x01Ra\x05\xEDV[\x96` \x83\x01\x90a\x07\x1AV[\x01\x90a\x07\x1AV[V[\x90a\x11\x12\x903\x92a\x16xV[\x91B\x92a\x11Ta\x11B\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x10\xC5V[\x94a\x11Ka\x02BV[\x93\x84\x93\x84a\x10\xD1V[\x03\x90\xA2V[\x90a\x11d\x92\x91a\x10}V[V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x11\x84\x90a\x05\xE3V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x11\x9EW`@RV[a\x11fV[\x90a\x11\xB6a\x11\xAFa\x02BV[\x92\x83a\x11zV[V[a\x11\xC2`\x80a\x11\xA3V[\x90V[_\x90V[a\x11\xD1a\x11\xB8V[\x90` \x80\x80\x80\x85a\x11\xE0a\x11\xC5V[\x81R\x01a\x11\xEBa\x11\xC5V[\x81R\x01a\x11\xF6a\x11\xC5V[\x81R\x01a\x12\x01a\x11\xC5V[\x81RPPV[a\x12\x0Fa\x11\xC9V[\x90V[a\x12\x1Aa\x12\x07V[Pa\x12#a\"\xCBV[\x90V[P\x90V[P\x90V[` \x91\x81R\x01\x90V[` \x7Fve the same length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x7FData and priority arrays must ha_\x82\x01R\x01RV[a\x12\x91`2`@\x92a\x12.V[a\x12\x9A\x81a\x127V[\x01\x90V[a\x12\xB3\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x12\x84V[\x90V[\x15a\x12\xBDWV[a\x12\xC5a\x02BV[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x12\xDB`\x04\x82\x01a\x12\x9EV[\x03\x90\xFD[\x90V[a\x12\xF6a\x12\xF1a\x12\xFB\x92a\x12\xDFV[a\x06\xE3V[a\x02\x9FV[\x90V[`\x01a\x13\n\x91\x01a\x02\x9FV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x13oW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x13jW` \x01\x91`\x01\x82\x026\x03\x83\x13a\x13eWV[a\x13)V[a\x13%V[a\x13!V[\x90\x82\x10\x15a\x13\x8FW` a\x13\x8B\x92\x02\x81\x01\x90a\x13-V[\x90\x91V[a\x13\rV[\x91\x90\x81\x10\x15a\x13\xA4W` \x02\x01\x90V[a\x13\rV[5a\x13\xB3\x81a\x02\xA2V[\x90V[\x90\x92a\x13\xC3\x82\x85\x90a\x12&V[\x93a\x13\xEA\x85a\x13\xE4a\x13\xDEa\x13\xD9\x88\x87\x90a\x12*V[a\x02\x9FV[\x91a\x02\x9FV[\x14a\x12\xB6V[a\x13\xF3_a\x12\xE2V[[\x80a\x14\x07a\x14\x01\x88a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a\x14\xAEWa\x145\x90a\x14+32\x90a\x14#\x88\x87\x86\x91a\x13tV[\x92\x90\x91a\x1BfV[a\x14:W[a\x12\xFEV[a\x13\xF4V[3a\x14Pa\x14J\x87\x86\x85\x91a\x13tV[\x90a\x16xV[\x90a\x14ea\x14`\x89\x88\x86\x91a\x13\x94V[a\x13\xA9V[B\x92a\x14\xA6a\x14\x94\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x10\xC5V[\x94a\x14\x9Da\x02BV[\x93\x84\x93\x84a\x10\xD1V[\x03\x90\xA2a\x140V[PPPPPPV[_\x90V[a\x14\xC6a\x14\xCB\x91a\x0E\xC2V[a\x06pV[\x90V[a\x14\xD8\x90Ta\x14\xBAV[\x90V[a\x14\xE3a\x14\xB6V[Pa\x14\xEE`\x04a\x14\xCEV[\x90V[\x91\x90a\x15\x0Ea\x15\x0832\x90\x86\x85\x91\x92\x90\x91\x92a\x1BfV[\x15a\x05\x05V[a\x15\x1DWa\x15\x1B\x92a\x15\x9CV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x155`\x04\x82\x01a\x03\nV[\x03\x90\xFD[\x90\x82_\x93\x92\x827\x01RV[\x91\x90a\x15^\x81a\x15W\x81a\x15c\x95a\x05\xCFV[\x80\x95a\x159V[a\x05\xE3V[\x01\x90V[a\x15\x93a\x15\x88`@\x93a\x15\x9A\x96\x98\x97\x95``\x85\x01\x91\x85\x83\x03_\x87\x01Ra\x15DV[\x96` \x83\x01\x90a\x07\x1AV[\x01\x90a\x07\x1AV[V[\x90\x913\x91\x92\x90\x92a\x15\xE3Ba\x15\xD1\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x95a\x10\xC5V[\x95a\x15\xDAa\x02BV[\x94\x85\x94\x85a\x15gV[\x03\x90\xA2V[\x90a\x15\xF3\x92\x91a\x14\xF1V[V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x16\x1Da\x16\x18a\x16\"\x92a\x12\xDFV[a\x16\x03V[a\x15\xFAV[\x90V[\x90V[a\x164a\x169\x91a\x15\xFAV[a\x16%V[\x90RV[\x90P\x90V[\x90\x91\x82a\x16R\x81a\x16Y\x93a\x16=V[\x80\x93a\x159V[\x01\x90V[\x80a\x16n`\x01\x92a\x16u\x96\x94a\x16(V[\x01\x91a\x16BV[\x90V[a\x16\xB6\x90a\x16\x84a\x15\xF5V[Pa\x16\xA7a\x16\x91_a\x16\tV[\x91\x93a\x16\x9Ba\x02BV[\x94\x85\x93` \x85\x01a\x16]V[` \x82\x01\x81\x03\x82R\x03\x82a\x11zV[\x90V[\x90a\x16\xD5a\x16\xCF32\x90\x85\x85\x91\x92\x90\x91\x92a\x1BfV[\x15a\x05\x05V[a\x16\xE4Wa\x16\xE2\x91a\x17`V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x16\xFC`\x04\x82\x01a\x03\nV[\x03\x90\xFD[`\x08\x1C\x90V[a\x17\x12a\x17\x17\x91a\x17\0V[a\x06pV[\x90V[a\x17$\x90Ta\x17\x06V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x17Ja\x17P\x91\x93\x92\x93a\x02\x9FV[\x92a\x02\x9FV[\x82\x03\x91\x82\x11a\x17[WV[a\x17'V[\x90a\x17ta\x17n`\x04a\x17\x1AV[\x15a\x05\x05V[a\x17\x98Wa\x17\x96\x91a\x17\x8Aa\x17\x91\x92Z\x92a\x17\xBDV[Z\x90a\x17;V[a$\xA5V[V[a\x17\xA1\x91a\x17\xBDV[V[\x90\x91a\x17\xBA\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x15DV[\x90V[3\x90\x91a\x17\xEA\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\xC5V[\x92a\x17\xFFa\x17\xF6a\x02BV[\x92\x83\x92\x83a\x17\xA3V[\x03\x90\xA2V[\x90a\x18\x0E\x91a\x16\xB9V[V[\x90a\x18\"\x91a\x18\x1Da%\xACV[a\x19(V[V[`\xA0\x1C\x90V[a\x186a\x18;\x91a\x18$V[a\x06pV[\x90V[a\x18H\x90Ta\x18*V[\x90V[a\x18_a\x18Za\x18d\x92a\x12\xDFV[a\x06\xE3V[a\x07\xA5V[\x90V[a\x18p\x90a\x18KV[\x90V[`\xA0\x1B\x90V[\x90a\x18\x88`\xFF`\xA0\x1B\x91a\x18sV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18\x9B\x90a\x05\x05V[\x90V[\x90V[\x90a\x18\xB6a\x18\xB1a\x18\xBD\x92a\x18\x92V[a\x18\x9EV[\x82Ta\x18yV[\x90UV[a\x18\xCA\x90a\t\x06V[\x90V[a\x18\xD6\x90a\x18\xC1V[\x90V[_\x1B\x90V[\x90a\x18\xEF`\x01\x80`\xA0\x1B\x03\x91a\x18\xD9V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x19\x02\x90a\x18\xC1V[\x90V[\x90V[\x90a\x19\x1Da\x19\x18a\x19$\x92a\x18\xF9V[a\x19\x05V[\x82Ta\x18\xDEV[\x90UV[a\x192`\x01a\x18>V[a\x19\x9AW\x81a\x19Qa\x19Ka\x19F_a\x18gV[a\x07\xB0V[\x91a\x07\xB0V[\x14a\x19~Wa\x19wa\x19pa\x19|\x93a\x19k`\x01\x80a\x18\xA1V[a\x18\xCDV[`\x01a\x19\x08V[a!\xC9V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x19\x96`\x04\x82\x01a\x03\nV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x19\xB1`\x04\x82\x01a\x03\nV[\x03\x90\xFD[\x90a\x19\xBF\x91a\x18\x10V[V[\x90a\x19\xCB\x90a\x02\x9FV[\x90RV[\x90a\x1A6a\x1A-`\x03a\x19\xE0a\x11\xB8V[\x94a\x19\xF7a\x19\xEF_\x83\x01a\x0E\xDBV[_\x88\x01a\x19\xC1V[a\x1A\x0Fa\x1A\x06`\x01\x83\x01a\x0E\xDBV[` \x88\x01a\x19\xC1V[a\x1A'a\x1A\x1E`\x02\x83\x01a\x0E\xDBV[`@\x88\x01a\x19\xC1V[\x01a\x0E\xDBV[``\x84\x01a\x19\xC1V[V[a\x1AA\x90a\x19\xCFV[\x90V[a\x1A[a\x1A`\x91a\x1ASa\x12\x07V[P`\x05a\x0E\xACV[a\x1A8V[\x90V[a\x1Aka%\xACV[a\x1Asa\x1AuV[V[a\x1A}a&7V[V[a\x1A\x87a\x1AcV[V[a\x1A\x91a%\xACV[a\x1A\x99a\x1A\x9BV[V[a\x1A\xACa\x1A\xA7_a\x18gV[a&gV[V[a\x1A\xB6a\x1A\x89V[V[a\x1A\xC4a\x1A\xC9\x91a\x0E\xC2V[a\x08\xC6V[\x90V[a\x1A\xD6\x90Ta\x1A\xB8V[\x90V[`\xE0\x1B\x90V[a\x1A\xE8\x81a\x05\x05V[\x03a\x1A\xEFWV[_\x80\xFD[\x90PQ\x90a\x1B\0\x82a\x1A\xDFV[V[\x90` \x82\x82\x03\x12a\x1B\x1BWa\x1B\x18\x91_\x01a\x1A\xF3V[\x90V[a\x02LV[a\x1BFa\x1BS\x95\x93\x94\x92\x94a\x1B<``\x84\x01\x96_\x85\x01\x90a\x0CDV[` \x83\x01\x90a\x0CDV[`@\x81\x85\x03\x91\x01Ra\x15DV[\x90V[a\x1B^a\x02BV[=_\x82>=\x90\xFD[\x92a\x1B\xA9` \x93\x94a\x1Bva\x14\xB6V[Pa\x1B\xB4a\x1B\x8Ca\x1B\x87`\x01a\x1A\xCCV[a\t.V[\x93cz9y\xDC\x92\x95\x97a\x1B\x9Da\x02BV[\x98\x89\x97\x88\x96\x87\x96a\x1A\xD9V[\x86R`\x04\x86\x01a\x1B V[\x03\x91Z\xFA\x90\x81\x15a\x1B\xF8W_\x91a\x1B\xCAW[P\x90V[a\x1B\xEB\x91P` =\x81\x11a\x1B\xF1W[a\x1B\xE3\x81\x83a\x11zV[\x81\x01\x90a\x1B\x02V[_a\x1B\xC6V[P=a\x1B\xD9V[a\x1BVV[_\x90V[a\x1C\x0B\x90Qa\x02\x9FV[\x90V[a\x1C\x1Da\x1C#\x91\x93\x92\x93a\x02\x9FV[\x92a\x02\x9FV[\x82\x01\x80\x92\x11a\x1C.WV[a\x17'V[a\x1C;a\x1B\xFDV[Pa\x1Cba\x1CI`\x06a\x0E\xDBV[a\x1C\\``a\x1CVa\"\xCBV[\x01a\x1C\x01V[\x90a\x1C\x0EV[\x90V[\x90a\x1C\x81a\x1C{32\x90\x85\x85\x91\x92\x90\x91\x92a\x1BfV[\x15a\x05\x05V[a\x1C\x90Wa\x1C\x8E\x91a\x1C\xACV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x1C\xA8`\x04\x82\x01a\x03\nV[\x03\x90\xFD[\x90a\x1C\xC0a\x1C\xBA`\x04a\x17\x1AV[\x15a\x05\x05V[a\x1C\xE4Wa\x1C\xE2\x91a\x1C\xD6a\x1C\xDD\x92Z\x92a\x1C\xEFV[Z\x90a\x17;V[a$\xA5V[V[a\x1C\xED\x91a\x1C\xEFV[V[\x90a\x1C\xFB\x903\x92a\x16xV[\x90a\x1D;a\x1D)\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\xC5V[\x92a\x1D2a\x02BV[\x91\x82\x91\x82a\x06\x1EV[\x03\x90\xA2V[\x90a\x1DJ\x91a\x1CeV[V[a\x1DTa\x1B\xFDV[Pa\x1D]a\"\xCBV[a\x1Dh_\x82\x01a\x1C\x01V[a\x1Dza\x1Dt_a\x12\xE2V[\x91a\x02\x9FV[\x14a\x1D\xD1Wa\x1D\x8D_a\x1D\x9D\x92\x01a\x1C\x01V[a\x1D\x97`\x02a\x0E\xDBV[\x90a\x1C\x0EV[Ba\x1D\xB0a\x1D\xAA\x83a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a\x1D\xC4Wa\x1D\xC1\x90B\x90a\x17;V[\x90V[Pa\x1D\xCE_a\x12\xE2V[\x90V[Pa\x1D\xDB_a\x12\xE2V[\x90V[a\x1D\xEDa\x1D\xF3\x91\x93\x92\x93a\x02\x9FV[\x92a\x02\x9FV[\x91a\x1D\xFF\x83\x82\x02a\x02\x9FV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1E\x0EWV[a\x17'V[\x91a\x1E\x1Ca\x1B\xFDV[P\x80a\x1E0a\x1E*\x84a\x02\x9FV[\x91a\x02\x9FV[\x11\x15a\x1E\x84Wa\x1EQ\x91a\x1EC\x91a\x17;V[a\x1EKa\x07\x02V[\x90a\x1D\xDEV[\x80a\x1Eda\x1E^\x84a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a\x1EvWa\x1Es\x91a\x17;V[\x90V[PPa\x1E\x81_a\x12\xE2V[\x90V[PP\x90V[a\x1E\x91a\x1B\xFDV[Pa\x1E\xA5``a\x1E\x9Fa\"\xCBV[\x01a\x1C\x01V[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1E\xC3a\x1E\xC8\x91a\x0E\xC2V[a\x1E\xACV[\x90V[a\x1E\xD5\x90Ta\x1E\xB7V[\x90V[a\x1E\xE0a\x1E\xA8V[Pa\x1E\xEA_a\x1E\xCBV[\x90V[\x90V[a\x1F\x04a\x1E\xFFa\x1F\t\x92a\x1E\xEDV[a\x06\xE3V[a\x02\x9FV[\x90V[a\x1F\x14a\x1B\xFDV[Pa\x1F(a\x1F\"`\x04a\x14\xCEV[\x15a\x05\x05V[a\x1FLWa\x1FIa\x1F9`\x03a\x0E\xDBV[a\x1FC`\x01a\x1E\xF0V[\x90a\x1C\x0EV[\x90V[a\x1FU_a\x12\xE2V[\x90V[a\x1F`a\x1B\xFDV[Pa\x1Ft`@a\x1Fna\"\xCBV[\x01a\x1C\x01V[\x90V[\x90a\x1F\x8Ba\x1F\x85`\x04a\x17\x1AV[\x15a\x05\x05V[a\x1F\xAFWa\x1F\xAD\x91a\x1F\xA1a\x1F\xA8\x92Z\x92a\x1F\xBAV[Z\x90a\x17;V[a$\xA5V[V[a\x1F\xB8\x91a\x1F\xBAV[V[a\x1F\xC5\x81\x83\x90a\x12&V[\x91a\x1F\xCEa\x1B\xFDV[Pa\x1F\xD8_a\x12\xE2V[[\x80a\x1F\xECa\x1F\xE6\x86a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a }Wa \x1A\x90a \x1032\x90a \x08\x87\x87\x86\x91a\x13tV[\x92\x90\x91a\x1BfV[a \x1FW[a\x12\xFEV[a\x1F\xD9V[3a 5a /\x86\x86\x85\x91a\x13tV[\x90a\x16xV[\x90a ua c\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\xC5V[\x92a la\x02BV[\x91\x82\x91\x82a\x06\x1EV[\x03\x90\xA2a \x15V[PPPPV[\x90a \x8D\x91a\x1FwV[V[a \xA0\x90a \x9Ba%\xACV[a \xA2V[V[\x80a \xBDa \xB7a \xB2_a\x18gV[a\x07\xB0V[\x91a\x07\xB0V[\x14a!\x17Wa \xD5a \xCE\x82a\x18\xCDV[`\x01a\x19\x08V[a \xFF\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10\xC5V[\x90a!\x08a\x02BV[\x80a!\x12\x81a\x03\nV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a!/`\x04\x82\x01a\x03\nV[\x03\x90\xFD[a!<\x90a \x8FV[V[a!Fa%\xACV[a!Na!PV[V[a!Xa&\xC6V[V[a!ba!>V[V[a!u\x90a!pa%\xACV[a!wV[V[\x80a!\x92a!\x8Ca!\x87_a\x18gV[a\x07\xB0V[\x91a\x07\xB0V[\x14a!\xA2Wa!\xA0\x90a&gV[V[a!\xC5a!\xAE_a\x18gV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x0CQV[\x03\x90\xFD[a!\xD2\x90a!dV[V[_\x7FGasCounter: invalid range\0\0\0\0\0\0\0\x91\x01RV[a\"\x08`\x19` \x92a\x12.V[a\"\x11\x81a!\xD4V[\x01\x90V[a\"*\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra!\xFBV[\x90V[\x15a\"4WV[a\"<a\x02BV[bF\x1B\xCD`\xE5\x1B\x81R\x80a\"R`\x04\x82\x01a\"\x15V[\x03\x90\xFD[a\"\x85\x91a\"ba\x1B\xFDV[Pa\"\x80\x81a\"ya\"s\x85a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a\"-V[a\x17;V[\x90V[a\"\x92`\x80a\x11\xA3V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\"\xB5a\"\xBB\x91a\x02\x9FV[\x91a\x02\x9FV[\x90\x81\x15a\"\xC6W\x04\x90V[a\"\x95V[a\"\xD3a\x12\x07V[Pa\"\xE7a\"\xE1`\x04a\x14\xCEV[\x15a\x05\x05V[a#\xE9Wa#\x08a#\x03`\x05a\"\xFD`\x03a\x0E\xDBV[\x90a\x0E\xACV[a\x1A8V[Ba#8a#2a#-a#\x1D_\x86\x01a\x1C\x01V[a#'`\x02a\x0E\xDBV[\x90a\x1C\x0EV[a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a#AW\x90V[a#\x92\x90a#\x8Ca#{_a#ta#dBa#^\x84\x88\x01a\x1C\x01V[\x90a\x17;V[a#n`\x02a\x0E\xDBV[\x90a\"\xA9V[\x93\x01a\x1C\x01V[\x91a#\x86`\x02a\x0E\xDBV[\x90a\x1D\xDEV[\x90a\x1C\x0EV[a#\xE6a#\xDD_a#\xD8a#\xCF_a#\xCAa#\xC1_\x95a#\xBCa#\xB3a\"\x88V[\x9A_\x8C\x01a\x19\xC1V[a\x12\xE2V[` \x89\x01a\x19\xC1V[a\x12\xE2V[`@\x86\x01a\x19\xC1V[a\x12\xE2V[``\x83\x01a\x19\xC1V[\x90V[_a$Fa$=_a$8a$/_a$*a$!_\x95a$\x1Ca$\x14a$\x0Ea\"\x88V[\x9Ba\x12\xE2V[_\x8C\x01a\x19\xC1V[a\x12\xE2V[` \x89\x01a\x19\xC1V[a\x12\xE2V[`@\x86\x01a\x19\xC1V[a\x12\xE2V[``\x83\x01a\x19\xC1V[\x90V[\x90a$U_\x19\x91a\x18\xD9V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a$wa$ra$~\x92a\x0E\x90V[a$_V[\x82Ta$IV[\x90UV[\x91` a$\xA3\x92\x94\x93a$\x9C`@\x82\x01\x96_\x83\x01\x90a\x07\x1AV[\x01\x90a\x07\x1AV[V[a$\xB8a$\xB2`\x04a\x17\x1AV[\x15a\x05\x05V[a%\xA9Wa$\xCFa$\xC9`\x04a\x14\xCEV[\x15a\x05\x05V[a%\x9CW[a$\xDCa(\x9CV[a%Ma$\xEA\x82:\x90a\x1D\xDEV[a%\x1E\x83a%\x18`\x02a%\x08`\x05a%\x02`\x03a\x0E\xDBV[\x90a\x0E\xACV[\x01\x91a%\x13\x83a\x0E\xDBV[a\x1C\x0EV[\x90a$bV[a%G`\x03a%7`\x05a%1\x83a\x0E\xDBV[\x90a\x0E\xACV[\x01\x91a%B\x83a\x0E\xDBV[a\x1C\x0EV[\x90a$bV[a%W`\x03a\x0E\xDBV[:a%\x82\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0E\x90V[\x92a%\x97a%\x8Ea\x02BV[\x92\x83\x92\x83a$\x82V[\x03\x90\xA2V[a%\xA4a'\x91V[a$\xD4V[PV[a%\xB4a\x1E\xD8V[a%\xCDa%\xC7a%\xC2a*\xA0V[a\x07\xB0V[\x91a\x07\xB0V[\x03a%\xD4WV[a%\xF6a%\xDFa*\xA0V[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\x0CQV[\x03\x90\xFD[`\x08\x1B\x90V[\x90a&\ra\xFF\0\x91a%\xFAV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a&,a&'a&3\x92a\x18\x92V[a\x18\x9EV[\x82Ta&\0V[\x90UV[a&B_`\x04a&\x17V[V[\x90V[\x90a&\\a&Wa&c\x92a\x10\xC5V[a&DV[\x82Ta\x18\xDEV[\x90UV[a&p_a\x1E\xCBV[a&z\x82_a&GV[\x90a&\xAEa&\xA8\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10\xC5V[\x91a\x10\xC5V[\x91a&\xB7a\x02BV[\x80a&\xC1\x81a\x03\nV[\x03\x90\xA3V[a&\xD2`\x01`\x04a&\x17V[V[\x90a&\xE0`\xFF\x91a\x18\xD9V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a&\xFFa&\xFAa'\x06\x92a\x18\x92V[a\x18\x9EV[\x82Ta&\xD4V[\x90UV[\x90a'\x14\x90a\x12\xE2V[_R` R`@_ \x90V[\x90a'}```\x03a'\x83\x94a'C_\x82\x01a'=_\x88\x01a\x1C\x01V[\x90a$bV[a'\\`\x01\x82\x01a'V` \x88\x01a\x1C\x01V[\x90a$bV[a'u`\x02\x82\x01a'o`@\x88\x01a\x1C\x01V[\x90a$bV[\x01\x92\x01a\x1C\x01V[\x90a$bV[V[\x90a'\x8F\x91a' V[V[a'\xA4a'\x9E`\x04a\x14\xCEV[\x15a\x05\x05V[a'\xABW[V[a'\xB7`\x01`\x04a&\xEAV[a'\xCAa'\xC3_a\x12\xE2V[`\x03a$bV[a(3Ba(\"a(\x19_a(\x14a(\x0B_a(\x06a'\xFD_\x95a'\xF8a'\xEFa\"\x88V[\x9A_\x8C\x01a\x19\xC1V[a\x12\xE2V[` \x89\x01a\x19\xC1V[a\x12\xE2V[`@\x86\x01a\x19\xC1V[a\x12\xE2V[``\x83\x01a\x19\xC1V[a(.`\x05_\x90a'\nV[a'\x85V[_B\x90a(ua(c\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x12\xE2V[\x92a(la\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xA2a'\xA9V[\x90V[a(\x89\x90a\x02\x9FV[_\x19\x81\x14a(\x97W`\x01\x01\x90V[a\x17'V[a(\xB9a(\xB4`\x05a(\xAE`\x03a\x0E\xDBV[\x90a\x0E\xACV[a(}V[Ba(\xE9a(\xE3a(\xDEa(\xCE_\x86\x01a\x0E\xDBV[a(\xD8`\x02a\x0E\xDBV[\x90a\x1C\x0EV[a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a(\xF3W[PV[a)\x1Da)\x14a)\x04_\x84\x01a\x0E\xDBV[a)\x0E`\x02a\x0E\xDBV[\x90a\x1C\x0EV[`\x01\x83\x01a$bV[a)Ea)>a)/`\x03\x84\x01a\x0E\xDBV[a)9`\x06a\x0E\xDBV[a\x1C\x0EV[`\x06a$bV[a)O`\x03a\x0E\xDBV[a)|a)^`\x02\x84\x01a\x0E\xDBV[\x92a)v_a)o`\x01\x84\x01a\x0E\xDBV[\x92\x01a\x0E\xDBV[\x90a\x17;V[a)\xA6\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0E\x90V[\x92a)\xBBa)\xB2a\x02BV[\x92\x83\x92\x83a$\x82V[\x03\x90\xA2a)\xDAa)\xD3a)\xCE`\x03a\x0E\xDBV[a(\x80V[`\x03a$bV[a*LBa*2a*)_a*$a*\x1B_a*\x16a*\r_\x95a*\x08a)\xFFa\"\x88V[\x9A_\x8C\x01a\x19\xC1V[a\x12\xE2V[` \x89\x01a\x19\xC1V[a\x12\xE2V[`@\x86\x01a\x19\xC1V[a\x12\xE2V[``\x83\x01a\x19\xC1V[a*G`\x05a*A`\x03a\x0E\xDBV[\x90a\x0E\xACV[a'\x85V[a*V`\x03a\x0E\xDBV[B\x90a*\x97a*\x85\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0E\x90V[\x92a*\x8Ea\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xA2_a(\xF0V[a*\xA8a\x1E\xA8V[P3\x90V`\x80`@R4`\x1CW`\x0E` V[a\x01\x94a\0+\x829a\x01\x94\x90\xF3[`&V[`@Q\x90V[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x01{V[a\0\x1D_5a\0,V[cz9y\xDC\x03a\0\x0EWa\x01BV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\0X\x90a\0DV[\x90V[a\0d\x81a\0OV[\x03a\0kWV[_\x80\xFD[\x90P5\x90a\0|\x82a\0[V[V[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\0\xC4W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xBFW` \x01\x92`\x01\x83\x02\x84\x01\x11a\0\xBAWV[a\0\x86V[a\0\x82V[a\0~V[\x91``\x83\x83\x03\x12a\x01\x16Wa\0\xE0\x82_\x85\x01a\0oV[\x92a\0\xEE\x83` \x83\x01a\0oV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x11Wa\x01\r\x92\x01a\0\x8AV[\x90\x91V[a\0@V[a\0<V[\x15\x15\x90V[a\x01)\x90a\x01\x1BV[\x90RV[\x91\x90a\x01@\x90_` \x85\x01\x94\x01\x90a\x01 V[V[4a\x01vWa\x01ra\x01aa\x01X6`\x04a\0\xC9V[\x92\x91\x90\x91a\x01\x83V[a\x01ia\x002V[\x91\x82\x91\x82a\x01-V[\x03\x90\xF3[a\08V[_\x80\xFD[_\x90V[PPPPa\x01\x8Fa\x01\x7FV[P_\x90V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b611079565b61001d5f3561023c565b8063050ec13814610237578063086146d21461023257806311992f8c1461022d57806318d5aafe146102285780631c0b636714610223578063366cbab71461021e5780633b6ab2a9146102195780633d44ae8b1461021457806346e2cc091461020f578063485cc9551461020a5780634b2c0706146102055780635467cb48146102005780635b3cd6e2146101fb57806361543801146101f6578063715018a6146101f15780637a3979dc146101ec5780637fbd295e146101e7578063804e5123146101e257806382f44ade146101dd57806383d3c115146101d857806384fab62b146101d35780638d5a239b146101ce5780638da5cb5b146101c9578063aff74c6d146101c4578063b470aade146101bf578063c660d3f3146101ba578063cdafb978146101b5578063d4f0eb4d146101b0578063d8781342146101ab578063de1f453e146101a6578063ea4a1104146101a1578063f2fde38b1461019c578063f7b8935e146101975763ff7b30840361000e57611044565b610fff565b610f9f565b610f66565b610e5d565b610e28565b610dd1565b610d7f565b610d14565b610cdf565b610c9b565b610c66565b610c0f565b610bda565b610b94565b610b25565b610af1565b610abc565b610a83565b6109fe565b6109c9565b61095c565b610893565b61085e565b61080c565b610771565b61073c565b6106ab565b610636565b610561565b61052c565b6104ce565b6103bc565b61030f565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f8301121561029a5781359167ffffffffffffffff831161029557602001926001830284011161029057565b61025c565b610258565b610254565b90565b6102ab8161029f565b036102b257565b5f80fd5b905035906102c3826102a2565b565b91604083830312610305575f83013567ffffffffffffffff8111610300576102f2836102fd928601610260565b9390946020016102b6565b90565b610250565b61024c565b5f0190565b3461033e576103286103223660046102c5565b91611159565b610330610242565b8061033a8161030a565b0390f35b610248565b5f91031261034d57565b61024c565b61035b9061029f565b9052565b906060806103a5936103775f8201515f860190610352565b61038960208201516020860190610352565b61039b60408201516040860190610352565b0151910190610352565b565b91906103ba905f6080850194019061035f565b565b346103ec576103cc366004610343565b6103e86103d7611212565b6103df610242565b918291826103a7565b0390f35b610248565b909182601f8301121561042b5781359167ffffffffffffffff831161042657602001926020830284011161042157565b61025c565b610258565b610254565b909182601f8301121561046a5781359167ffffffffffffffff831161046557602001926020830284011161046057565b61025c565b610258565b610254565b90916040828403126104c9575f82013567ffffffffffffffff81116104c4578361049a9184016103f1565b929093602082013567ffffffffffffffff81116104bf576104bb9201610430565b9091565b610250565b610250565b61024c565b34610500576104ea6104e136600461046f565b929190916113b6565b6104f2610242565b806104fc8161030a565b0390f35b610248565b151590565b61051390610505565b9052565b919061052a905f6020850194019061050a565b565b3461055c5761053c366004610343565b6105586105476114db565b61054f610242565b91829182610517565b0390f35b610248565b346105905761057a6105743660046102c5565b916115e8565b610582610242565b8061058c8161030a565b0390f35b610248565b906020828203126105c6575f82013567ffffffffffffffff81116105c1576105bd9201610260565b9091565b610250565b61024c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61060c61061560209361061a93610603816105cb565b938480936105cf565b958691016105d8565b6105e3565b0190565b6106339160208201915f8184039101526105ed565b90565b346106675761066361065261064c366004610595565b90611678565b61065a610242565b9182918261061e565b0390f35b610248565b1c90565b60ff1690565b61068690600861068b930261066c565b610670565b90565b906106999154610676565b90565b6106a860045f9061068e565b90565b346106db576106bb366004610343565b6106d76106c661069c565b6106ce610242565b91829182610517565b0390f35b610248565b90565b90565b6106fa6106f56106ff926106e0565b6106e3565b61029f565b90565b61070c600a6106e6565b90565b610717610702565b90565b6107239061029f565b9052565b919061073a905f6020850194019061071a565b565b3461076c5761074c366004610343565b61076861075761070f565b61075f610242565b91829182610727565b0390f35b610248565b346107a05761078a610784366004610595565b90611804565b610792610242565b8061079c8161030a565b0390f35b610248565b60018060a01b031690565b6107b9906107a5565b90565b6107c5816107b0565b036107cc57565b5f80fd5b905035906107dd826107bc565b565b919060408382031261080757806107fb610804925f86016107d0565b936020016107d0565b90565b61024c565b3461083b5761082561081f3660046107df565b906119b5565b61082d610242565b806108378161030a565b0390f35b610248565b9060208282031261085957610856915f016102b6565b90565b61024c565b3461088e5761088a610879610874366004610840565b611a44565b610881610242565b918291826103a7565b0390f35b610248565b346108c1576108a3366004610343565b6108ab611a7f565b6108b3610242565b806108bd8161030a565b0390f35b610248565b60018060a01b031690565b6108e19060086108e6930261066c565b6108c6565b90565b906108f491546108d1565b90565b61090360015f906108e9565b90565b61091a61091561091f926107a5565b6106e3565b6107a5565b90565b61092b90610906565b90565b61093790610922565b90565b6109439061092e565b9052565b919061095a905f6020850194019061093a565b565b3461098c5761096c366004610343565b6109886109776108f7565b61097f610242565b91829182610947565b0390f35b610248565b90565b6109a49060086109a9930261066c565b610991565b90565b906109b79154610994565b90565b6109c660035f906109ac565b90565b346109f9576109d9366004610343565b6109f56109e46109ba565b6109ec610242565b91829182610727565b0390f35b610248565b34610a2c57610a0e366004610343565b610a16611aae565b610a1e610242565b80610a288161030a565b0390f35b610248565b91606083830312610a7e57610a48825f85016107d0565b92610a5683602083016107d0565b92604082013567ffffffffffffffff8111610a7957610a759201610260565b9091565b610250565b61024c565b34610ab757610ab3610aa2610a99366004610a31565b92919091611b66565b610aaa610242565b91829182610517565b0390f35b610248565b34610aec57610acc366004610343565b610ae8610ad7611c33565b610adf610242565b91829182610727565b0390f35b610248565b34610b2057610b0a610b04366004610595565b90611d40565b610b12610242565b80610b1c8161030a565b0390f35b610248565b34610b5557610b35366004610343565b610b51610b40611d4c565b610b48610242565b91829182610727565b0390f35b610248565b9091606082840312610b8f57610b8c610b75845f85016102b6565b93610b8381602086016102b6565b936040016102b6565b90565b61024c565b34610bc557610bc1610bb0610baa366004610b5a565b91611e13565b610bb8610242565b91829182610727565b0390f35b610248565b610bd7600460019061068e565b90565b34610c0a57610bea366004610343565b610c06610bf5610bca565b610bfd610242565b91829182610517565b0390f35b610248565b34610c3f57610c1f366004610343565b610c3b610c2a611e89565b610c32610242565b91829182610727565b0390f35b610248565b610c4d906107b0565b9052565b9190610c64905f60208501940190610c44565b565b34610c9657610c76366004610343565b610c92610c81611ed8565b610c89610242565b91829182610c51565b0390f35b610248565b34610ccb57610cab366004610343565b610cc7610cb6611f0c565b610cbe610242565b91829182610727565b0390f35b610248565b610cdc60025f906109ac565b90565b34610d0f57610cef366004610343565b610d0b610cfa610cd0565b610d02610242565b91829182610727565b0390f35b610248565b34610d4457610d24366004610343565b610d40610d2f611f58565b610d37610242565b91829182610727565b0390f35b610248565b90602082820312610d7a575f82013567ffffffffffffffff8111610d7557610d7192016103f1565b9091565b610250565b61024c565b34610dae57610d98610d92366004610d49565b90612083565b610da0610242565b80610daa8161030a565b0390f35b610248565b90602082820312610dcc57610dc9915f016107d0565b90565b61024c565b34610dff57610de9610de4366004610db3565b612133565b610df1610242565b80610dfb8161030a565b0390f35b610248565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610e5857610e38366004610343565b610e54610e43610e04565b610e4b610242565b91829182610727565b0390f35b610248565b34610e8b57610e6d366004610343565b610e7561215a565b610e7d610242565b80610e878161030a565b0390f35b610248565b610ea4610e9f610ea99261029f565b6106e3565b61029f565b90565b90610eb690610e90565b5f5260205260405f2090565b5f1c90565b610ed3610ed891610ec2565b610991565b90565b610ee59054610ec7565b90565b610ef3906005610eac565b90610eff5f8301610edb565b91610f0c60018201610edb565b91610f256003610f1e60028501610edb565b9301610edb565b90565b610f5d610f6494610f53606094989795610f49608086019a5f87019061071a565b602085019061071a565b604083019061071a565b019061071a565b565b34610f9a57610f96610f81610f7c366004610840565b610ee8565b90610f8d949294610242565b94859485610f28565b0390f35b610248565b34610fcd57610fb7610fb2366004610db3565b6121c9565b610fbf610242565b80610fc98161030a565b0390f35b610248565b9190604083820312610ffa5780610fee610ff7925f86016102b6565b936020016102b6565b90565b61024c565b346110305761102c61101b611015366004610fd2565b90612256565b611023610242565b91829182610727565b0390f35b610248565b61104160065f906109ac565b90565b3461107457611054366004610343565b61107061105f611035565b611067610242565b91829182610727565b0390f35b610248565b5f80fd5b919061109a61109433329086859192909192611b66565b15610505565b6110a9576110a792611106565b565b5f631b8e828b60e31b8152806110c16004820161030a565b0390fd5b6110ce90610922565b90565b6040906110fd6110f26111049597969460608401908482035f8601526105ed565b96602083019061071a565b019061071a565b565b90611112903392611678565b9142926111546111427f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f2946110c5565b9461114b610242565b938493846110d1565b0390a2565b90611164929161107d565b565b634e487b7160e01b5f52604160045260245ffd5b90611184906105e3565b810190811067ffffffffffffffff82111761119e57604052565b611166565b906111b66111af610242565b928361117a565b565b6111c260806111a3565b90565b5f90565b6111d16111b8565b906020808080856111e06111c5565b8152016111eb6111c5565b8152016111f66111c5565b8152016112016111c5565b81525050565b61120f6111c9565b90565b61121a611207565b506112236122cb565b90565b5090565b5090565b60209181520190565b60207f7665207468652073616d65206c656e6774680000000000000000000000000000917f4461746120616e64207072696f7269747920617272617973206d7573742068615f8201520152565b611291603260409261122e565b61129a81611237565b0190565b6112b39060208101905f818303910152611284565b90565b156112bd57565b6112c5610242565b62461bcd60e51b8152806112db6004820161129e565b0390fd5b90565b6112f66112f16112fb926112df565b6106e3565b61029f565b90565b600161130a910161029f565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561136f570180359067ffffffffffffffff821161136a5760200191600182023603831361136557565b611329565b611325565b611321565b9082101561138f57602061138b920281019061132d565b9091565b61130d565b91908110156113a4576020020190565b61130d565b356113b3816102a2565b90565b90926113c3828590611226565b936113ea856113e46113de6113d988879061122a565b61029f565b9161029f565b146112b6565b6113f35f6112e2565b5b806114076114018861029f565b9161029f565b10156114ae576114359061142b33329061142388878691611374565b929091611b66565b61143a575b6112fe565b6113f4565b3361145061144a87868591611374565b90611678565b9061146561146089888691611394565b6113a9565b42926114a66114947f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f2946110c5565b9461149d610242565b938493846110d1565b0390a2611430565b505050505050565b5f90565b6114c66114cb91610ec2565b610670565b90565b6114d890546114ba565b90565b6114e36114b6565b506114ee60046114ce565b90565b919061150e61150833329086859192909192611b66565b15610505565b61151d5761151b9261159c565b565b5f631b8e828b60e31b8152806115356004820161030a565b0390fd5b90825f939282370152565b919061155e8161155781611563956105cf565b8095611539565b6105e3565b0190565b61159361158860409361159a9698979560608501918583035f870152611544565b96602083019061071a565b019061071a565b565b909133919290926115e3426115d17f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f2956110c5565b956115da610242565b94859485611567565b0390a2565b906115f392916114f1565b565b606090565b60ff60f81b1690565b60f81b90565b61161d611618611622926112df565b611603565b6115fa565b90565b90565b611634611639916115fa565b611625565b9052565b905090565b909182611652816116599361163d565b8093611539565b0190565b8061166e6001926116759694611628565b0191611642565b90565b6116b6906116846115f5565b506116a76116915f611609565b919361169b610242565b9485936020850161165d565b6020820181038252038261117a565b90565b906116d56116cf33329085859192909192611b66565b15610505565b6116e4576116e291611760565b565b5f631b8e828b60e31b8152806116fc6004820161030a565b0390fd5b60081c90565b61171261171791611700565b610670565b90565b6117249054611706565b90565b634e487b7160e01b5f52601160045260245ffd5b61174a6117509193929361029f565b9261029f565b820391821161175b57565b611727565b9061177461176e600461171a565b15610505565b611798576117969161178a611791925a926117bd565b5a9061173b565b6124a5565b565b6117a1916117bd565b565b90916117ba9260208301925f818503910152611544565b90565b3390916117ea7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926110c5565b926117ff6117f6610242565b928392836117a3565b0390a2565b9061180e916116b9565b565b906118229161181d6125ac565b611928565b565b60a01c90565b61183661183b91611824565b610670565b90565b611848905461182a565b90565b61185f61185a611864926112df565b6106e3565b6107a5565b90565b6118709061184b565b90565b60a01b90565b9061188860ff60a01b91611873565b9181191691161790565b61189b90610505565b90565b90565b906118b66118b16118bd92611892565b61189e565b8254611879565b9055565b6118ca90610906565b90565b6118d6906118c1565b90565b5f1b90565b906118ef60018060a01b03916118d9565b9181191691161790565b611902906118c1565b90565b90565b9061191d611918611924926118f9565b611905565b82546118de565b9055565b611932600161183e565b61199a578161195161194b6119465f611867565b6107b0565b916107b0565b1461197e5761197761197061197c9361196b6001806118a1565b6118cd565b6001611908565b6121c9565b565b5f632e7f3c7f60e11b8152806119966004820161030a565b0390fd5b5f62dc149f60e41b8152806119b16004820161030a565b0390fd5b906119bf91611810565b565b906119cb9061029f565b9052565b90611a36611a2d60036119e06111b8565b946119f76119ef5f8301610edb565b5f88016119c1565b611a0f611a0660018301610edb565b602088016119c1565b611a27611a1e60028301610edb565b604088016119c1565b01610edb565b606084016119c1565b565b611a41906119cf565b90565b611a5b611a6091611a53611207565b506005610eac565b611a38565b90565b611a6b6125ac565b611a73611a75565b565b611a7d612637565b565b611a87611a63565b565b611a916125ac565b611a99611a9b565b565b611aac611aa75f611867565b612667565b565b611ab6611a89565b565b611ac4611ac991610ec2565b6108c6565b90565b611ad69054611ab8565b90565b60e01b90565b611ae881610505565b03611aef57565b5f80fd5b90505190611b0082611adf565b565b90602082820312611b1b57611b18915f01611af3565b90565b61024c565b611b46611b539593949294611b3c60608401965f850190610c44565b6020830190610c44565b6040818503910152611544565b90565b611b5e610242565b3d5f823e3d90fd5b92611ba960209394611b766114b6565b50611bb4611b8c611b876001611acc565b61092e565b93637a3979dc929597611b9d610242565b98899788968796611ad9565b865260048601611b20565b03915afa908115611bf8575f91611bca575b5090565b611beb915060203d8111611bf1575b611be3818361117a565b810190611b02565b5f611bc6565b503d611bd9565b611b56565b5f90565b611c0b905161029f565b90565b611c1d611c239193929361029f565b9261029f565b8201809211611c2e57565b611727565b611c3b611bfd565b50611c62611c496006610edb565b611c5c6060611c566122cb565b01611c01565b90611c0e565b90565b90611c81611c7b33329085859192909192611b66565b15610505565b611c9057611c8e91611cac565b565b5f631b8e828b60e31b815280611ca86004820161030a565b0390fd5b90611cc0611cba600461171a565b15610505565b611ce457611ce291611cd6611cdd925a92611cef565b5a9061173b565b6124a5565b565b611ced91611cef565b565b90611cfb903392611678565b90611d3b611d297f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926110c5565b92611d32610242565b9182918261061e565b0390a2565b90611d4a91611c65565b565b611d54611bfd565b50611d5d6122cb565b611d685f8201611c01565b611d7a611d745f6112e2565b9161029f565b14611dd157611d8d5f611d9d9201611c01565b611d976002610edb565b90611c0e565b42611db0611daa8361029f565b9161029f565b1015611dc457611dc190429061173b565b90565b50611dce5f6112e2565b90565b50611ddb5f6112e2565b90565b611ded611df39193929361029f565b9261029f565b91611dff83820261029f565b928184041490151715611e0e57565b611727565b91611e1c611bfd565b5080611e30611e2a8461029f565b9161029f565b1115611e8457611e5191611e439161173b565b611e4b610702565b90611dde565b80611e64611e5e8461029f565b9161029f565b1015611e7657611e739161173b565b90565b5050611e815f6112e2565b90565b505090565b611e91611bfd565b50611ea56060611e9f6122cb565b01611c01565b90565b5f90565b60018060a01b031690565b611ec3611ec891610ec2565b611eac565b90565b611ed59054611eb7565b90565b611ee0611ea8565b50611eea5f611ecb565b90565b90565b611f04611eff611f0992611eed565b6106e3565b61029f565b90565b611f14611bfd565b50611f28611f2260046114ce565b15610505565b611f4c57611f49611f396003610edb565b611f436001611ef0565b90611c0e565b90565b611f555f6112e2565b90565b611f60611bfd565b50611f746040611f6e6122cb565b01611c01565b90565b90611f8b611f85600461171a565b15610505565b611faf57611fad91611fa1611fa8925a92611fba565b5a9061173b565b6124a5565b565b611fb891611fba565b565b611fc5818390611226565b91611fce611bfd565b50611fd85f6112e2565b5b80611fec611fe68661029f565b9161029f565b101561207d5761201a9061201033329061200887878691611374565b929091611b66565b61201f575b6112fe565b611fd9565b3361203561202f86868591611374565b90611678565b906120756120637f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926110c5565b9261206c610242565b9182918261061e565b0390a2612015565b50505050565b9061208d91611f77565b565b6120a09061209b6125ac565b6120a2565b565b806120bd6120b76120b25f611867565b6107b0565b916107b0565b14612117576120d56120ce826118cd565b6001611908565b6120ff7f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b9916110c5565b90612108610242565b806121128161030a565b0390a2565b5f632e7f3c7f60e11b81528061212f6004820161030a565b0390fd5b61213c9061208f565b565b6121466125ac565b61214e612150565b565b6121586126c6565b565b61216261213e565b565b612175906121706125ac565b612177565b565b8061219261218c6121875f611867565b6107b0565b916107b0565b146121a2576121a090612667565b565b6121c56121ae5f611867565b5f918291631e4fbdf760e01b835260048301610c51565b0390fd5b6121d290612164565b565b5f7f476173436f756e7465723a20696e76616c69642072616e676500000000000000910152565b612208601960209261122e565b612211816121d4565b0190565b61222a9060208101905f8183039101526121fb565b90565b1561223457565b61223c610242565b62461bcd60e51b81528061225260048201612215565b0390fd5b61228591612262611bfd565b50612280816122796122738561029f565b9161029f565b101561222d565b61173b565b90565b61229260806111a3565b90565b634e487b7160e01b5f52601260045260245ffd5b6122b56122bb9161029f565b9161029f565b9081156122c6570490565b612295565b6122d3611207565b506122e76122e160046114ce565b15610505565b6123e95761230861230360056122fd6003610edb565b90610eac565b611a38565b4261233861233261232d61231d5f8601611c01565b6123276002610edb565b90611c0e565b61029f565b9161029f565b10156123415790565b6123929061238c61237b5f6123746123644261235e848801611c01565b9061173b565b61236e6002610edb565b906122a9565b9301611c01565b916123866002610edb565b90611dde565b90611c0e565b6123e66123dd5f6123d86123cf5f6123ca6123c15f956123bc6123b3612288565b9a5f8c016119c1565b6112e2565b602089016119c1565b6112e2565b604086016119c1565b6112e2565b606083016119c1565b90565b5f61244661243d5f61243861242f5f61242a6124215f9561241c61241461240e612288565b9b6112e2565b5f8c016119c1565b6112e2565b602089016119c1565b6112e2565b604086016119c1565b6112e2565b606083016119c1565b90565b906124555f19916118d9565b9181191691161790565b90565b9061247761247261247e92610e90565b61245f565b8254612449565b9055565b9160206124a392949361249c60408201965f83019061071a565b019061071a565b565b6124b86124b2600461171a565b15610505565b6125a9576124cf6124c960046114ce565b15610505565b61259c575b6124dc61289c565b61254d6124ea823a90611dde565b61251e83612518600261250860056125026003610edb565b90610eac565b019161251383610edb565b611c0e565b90612462565b6125476003612537600561253183610edb565b90610eac565b019161254283610edb565b611c0e565b90612462565b6125576003610edb565b3a6125827f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610e90565b9261259761258e610242565b92839283612482565b0390a2565b6125a4612791565b6124d4565b50565b6125b4611ed8565b6125cd6125c76125c2612aa0565b6107b0565b916107b0565b036125d457565b6125f66125df612aa0565b5f91829163118cdaa760e01b835260048301610c51565b0390fd5b60081b90565b9061260d61ff00916125fa565b9181191691161790565b9061262c61262761263392611892565b61189e565b8254612600565b9055565b6126425f6004612617565b565b90565b9061265c612657612663926110c5565b612644565b82546118de565b9055565b6126705f611ecb565b61267a825f612647565b906126ae6126a87f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0936110c5565b916110c5565b916126b7610242565b806126c18161030a565b0390a3565b6126d260016004612617565b565b906126e060ff916118d9565b9181191691161790565b906126ff6126fa61270692611892565b61189e565b82546126d4565b9055565b90612714906112e2565b5f5260205260405f2090565b9061277d60606003612783946127435f820161273d5f8801611c01565b90612462565b61275c6001820161275660208801611c01565b90612462565b6127756002820161276f60408801611c01565b90612462565b019201611c01565b90612462565b565b9061278f91612720565b565b6127a461279e60046114ce565b15610505565b6127ab575b565b6127b7600160046126ea565b6127ca6127c35f6112e2565b6003612462565b612833426128226128195f61281461280b5f6128066127fd5f956127f86127ef612288565b9a5f8c016119c1565b6112e2565b602089016119c1565b6112e2565b604086016119c1565b6112e2565b606083016119c1565b61282e60055f9061270a565b612785565b5f42906128756128637f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e926112e2565b9261286c610242565b91829182610727565b0390a26127a9565b90565b6128899061029f565b5f1981146128975760010190565b611727565b6128b96128b460056128ae6003610edb565b90610eac565b61287d565b426128e96128e36128de6128ce5f8601610edb565b6128d86002610edb565b90611c0e565b61029f565b9161029f565b10156128f3575b50565b61291d6129146129045f8401610edb565b61290e6002610edb565b90611c0e565b60018301612462565b61294561293e61292f60038401610edb565b6129396006610edb565b611c0e565b6006612462565b61294f6003610edb565b61297c61295e60028401610edb565b926129765f61296f60018401610edb565b9201610edb565b9061173b565b6129a67f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610e90565b926129bb6129b2610242565b92839283612482565b0390a26129da6129d36129ce6003610edb565b612880565b6003612462565b612a4c42612a32612a295f612a24612a1b5f612a16612a0d5f95612a086129ff612288565b9a5f8c016119c1565b6112e2565b602089016119c1565b6112e2565b604086016119c1565b6112e2565b606083016119c1565b612a476005612a416003610edb565b90610eac565b612785565b612a566003610edb565b4290612a97612a857f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610e90565b92612a8e610242565b91829182610727565b0390a25f6128f0565b612aa8611ea8565b50339056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x10yV[a\0\x1D_5a\x02<V[\x80c\x05\x0E\xC18\x14a\x027W\x80c\x08aF\xD2\x14a\x022W\x80c\x11\x99/\x8C\x14a\x02-W\x80c\x18\xD5\xAA\xFE\x14a\x02(W\x80c\x1C\x0Bcg\x14a\x02#W\x80c6l\xBA\xB7\x14a\x02\x1EW\x80c;j\xB2\xA9\x14a\x02\x19W\x80c=D\xAE\x8B\x14a\x02\x14W\x80cF\xE2\xCC\t\x14a\x02\x0FW\x80cH\\\xC9U\x14a\x02\nW\x80cK,\x07\x06\x14a\x02\x05W\x80cTg\xCBH\x14a\x02\0W\x80c[<\xD6\xE2\x14a\x01\xFBW\x80caT8\x01\x14a\x01\xF6W\x80cqP\x18\xA6\x14a\x01\xF1W\x80cz9y\xDC\x14a\x01\xECW\x80c\x7F\xBD)^\x14a\x01\xE7W\x80c\x80NQ#\x14a\x01\xE2W\x80c\x82\xF4J\xDE\x14a\x01\xDDW\x80c\x83\xD3\xC1\x15\x14a\x01\xD8W\x80c\x84\xFA\xB6+\x14a\x01\xD3W\x80c\x8DZ#\x9B\x14a\x01\xCEW\x80c\x8D\xA5\xCB[\x14a\x01\xC9W\x80c\xAF\xF7Lm\x14a\x01\xC4W\x80c\xB4p\xAA\xDE\x14a\x01\xBFW\x80c\xC6`\xD3\xF3\x14a\x01\xBAW\x80c\xCD\xAF\xB9x\x14a\x01\xB5W\x80c\xD4\xF0\xEBM\x14a\x01\xB0W\x80c\xD8x\x13B\x14a\x01\xABW\x80c\xDE\x1FE>\x14a\x01\xA6W\x80c\xEAJ\x11\x04\x14a\x01\xA1W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x9CW\x80c\xF7\xB8\x93^\x14a\x01\x97Wc\xFF{0\x84\x03a\0\x0EWa\x10DV[a\x0F\xFFV[a\x0F\x9FV[a\x0FfV[a\x0E]V[a\x0E(V[a\r\xD1V[a\r\x7FV[a\r\x14V[a\x0C\xDFV[a\x0C\x9BV[a\x0CfV[a\x0C\x0FV[a\x0B\xDAV[a\x0B\x94V[a\x0B%V[a\n\xF1V[a\n\xBCV[a\n\x83V[a\t\xFEV[a\t\xC9V[a\t\\V[a\x08\x93V[a\x08^V[a\x08\x0CV[a\x07qV[a\x07<V[a\x06\xABV[a\x066V[a\x05aV[a\x05,V[a\x04\xCEV[a\x03\xBCV[a\x03\x0FV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x02\x9AW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x02\x95W` \x01\x92`\x01\x83\x02\x84\x01\x11a\x02\x90WV[a\x02\\V[a\x02XV[a\x02TV[\x90V[a\x02\xAB\x81a\x02\x9FV[\x03a\x02\xB2WV[_\x80\xFD[\x90P5\x90a\x02\xC3\x82a\x02\xA2V[V[\x91`@\x83\x83\x03\x12a\x03\x05W_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\0Wa\x02\xF2\x83a\x02\xFD\x92\x86\x01a\x02`V[\x93\x90\x94` \x01a\x02\xB6V[\x90V[a\x02PV[a\x02LV[_\x01\x90V[4a\x03>Wa\x03(a\x03\"6`\x04a\x02\xC5V[\x91a\x11YV[a\x030a\x02BV[\x80a\x03:\x81a\x03\nV[\x03\x90\xF3[a\x02HV[_\x91\x03\x12a\x03MWV[a\x02LV[a\x03[\x90a\x02\x9FV[\x90RV[\x90``\x80a\x03\xA5\x93a\x03w_\x82\x01Q_\x86\x01\x90a\x03RV[a\x03\x89` \x82\x01Q` \x86\x01\x90a\x03RV[a\x03\x9B`@\x82\x01Q`@\x86\x01\x90a\x03RV[\x01Q\x91\x01\x90a\x03RV[V[\x91\x90a\x03\xBA\x90_`\x80\x85\x01\x94\x01\x90a\x03_V[V[4a\x03\xECWa\x03\xCC6`\x04a\x03CV[a\x03\xE8a\x03\xD7a\x12\x12V[a\x03\xDFa\x02BV[\x91\x82\x91\x82a\x03\xA7V[\x03\x90\xF3[a\x02HV[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04+W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04&W` \x01\x92` \x83\x02\x84\x01\x11a\x04!WV[a\x02\\V[a\x02XV[a\x02TV[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04jW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04eW` \x01\x92` \x83\x02\x84\x01\x11a\x04`WV[a\x02\\V[a\x02XV[a\x02TV[\x90\x91`@\x82\x84\x03\x12a\x04\xC9W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xC4W\x83a\x04\x9A\x91\x84\x01a\x03\xF1V[\x92\x90\x93` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xBFWa\x04\xBB\x92\x01a\x040V[\x90\x91V[a\x02PV[a\x02PV[a\x02LV[4a\x05\0Wa\x04\xEAa\x04\xE16`\x04a\x04oV[\x92\x91\x90\x91a\x13\xB6V[a\x04\xF2a\x02BV[\x80a\x04\xFC\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x15\x15\x90V[a\x05\x13\x90a\x05\x05V[\x90RV[\x91\x90a\x05*\x90_` \x85\x01\x94\x01\x90a\x05\nV[V[4a\x05\\Wa\x05<6`\x04a\x03CV[a\x05Xa\x05Ga\x14\xDBV[a\x05Oa\x02BV[\x91\x82\x91\x82a\x05\x17V[\x03\x90\xF3[a\x02HV[4a\x05\x90Wa\x05za\x05t6`\x04a\x02\xC5V[\x91a\x15\xE8V[a\x05\x82a\x02BV[\x80a\x05\x8C\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x90` \x82\x82\x03\x12a\x05\xC6W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05\xC1Wa\x05\xBD\x92\x01a\x02`V[\x90\x91V[a\x02PV[a\x02LV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x06\x0Ca\x06\x15` \x93a\x06\x1A\x93a\x06\x03\x81a\x05\xCBV[\x93\x84\x80\x93a\x05\xCFV[\x95\x86\x91\x01a\x05\xD8V[a\x05\xE3V[\x01\x90V[a\x063\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x05\xEDV[\x90V[4a\x06gWa\x06ca\x06Ra\x06L6`\x04a\x05\x95V[\x90a\x16xV[a\x06Za\x02BV[\x91\x82\x91\x82a\x06\x1EV[\x03\x90\xF3[a\x02HV[\x1C\x90V[`\xFF\x16\x90V[a\x06\x86\x90`\x08a\x06\x8B\x93\x02a\x06lV[a\x06pV[\x90V[\x90a\x06\x99\x91Ta\x06vV[\x90V[a\x06\xA8`\x04_\x90a\x06\x8EV[\x90V[4a\x06\xDBWa\x06\xBB6`\x04a\x03CV[a\x06\xD7a\x06\xC6a\x06\x9CV[a\x06\xCEa\x02BV[\x91\x82\x91\x82a\x05\x17V[\x03\x90\xF3[a\x02HV[\x90V[\x90V[a\x06\xFAa\x06\xF5a\x06\xFF\x92a\x06\xE0V[a\x06\xE3V[a\x02\x9FV[\x90V[a\x07\x0C`\na\x06\xE6V[\x90V[a\x07\x17a\x07\x02V[\x90V[a\x07#\x90a\x02\x9FV[\x90RV[\x91\x90a\x07:\x90_` \x85\x01\x94\x01\x90a\x07\x1AV[V[4a\x07lWa\x07L6`\x04a\x03CV[a\x07ha\x07Wa\x07\x0FV[a\x07_a\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[4a\x07\xA0Wa\x07\x8Aa\x07\x846`\x04a\x05\x95V[\x90a\x18\x04V[a\x07\x92a\x02BV[\x80a\x07\x9C\x81a\x03\nV[\x03\x90\xF3[a\x02HV[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x07\xB9\x90a\x07\xA5V[\x90V[a\x07\xC5\x81a\x07\xB0V[\x03a\x07\xCCWV[_\x80\xFD[\x90P5\x90a\x07\xDD\x82a\x07\xBCV[V[\x91\x90`@\x83\x82\x03\x12a\x08\x07W\x80a\x07\xFBa\x08\x04\x92_\x86\x01a\x07\xD0V[\x93` \x01a\x07\xD0V[\x90V[a\x02LV[4a\x08;Wa\x08%a\x08\x1F6`\x04a\x07\xDFV[\x90a\x19\xB5V[a\x08-a\x02BV[\x80a\x087\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x90` \x82\x82\x03\x12a\x08YWa\x08V\x91_\x01a\x02\xB6V[\x90V[a\x02LV[4a\x08\x8EWa\x08\x8Aa\x08ya\x08t6`\x04a\x08@V[a\x1ADV[a\x08\x81a\x02BV[\x91\x82\x91\x82a\x03\xA7V[\x03\x90\xF3[a\x02HV[4a\x08\xC1Wa\x08\xA36`\x04a\x03CV[a\x08\xABa\x1A\x7FV[a\x08\xB3a\x02BV[\x80a\x08\xBD\x81a\x03\nV[\x03\x90\xF3[a\x02HV[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08\xE1\x90`\x08a\x08\xE6\x93\x02a\x06lV[a\x08\xC6V[\x90V[\x90a\x08\xF4\x91Ta\x08\xD1V[\x90V[a\t\x03`\x01_\x90a\x08\xE9V[\x90V[a\t\x1Aa\t\x15a\t\x1F\x92a\x07\xA5V[a\x06\xE3V[a\x07\xA5V[\x90V[a\t+\x90a\t\x06V[\x90V[a\t7\x90a\t\"V[\x90V[a\tC\x90a\t.V[\x90RV[\x91\x90a\tZ\x90_` \x85\x01\x94\x01\x90a\t:V[V[4a\t\x8CWa\tl6`\x04a\x03CV[a\t\x88a\twa\x08\xF7V[a\t\x7Fa\x02BV[\x91\x82\x91\x82a\tGV[\x03\x90\xF3[a\x02HV[\x90V[a\t\xA4\x90`\x08a\t\xA9\x93\x02a\x06lV[a\t\x91V[\x90V[\x90a\t\xB7\x91Ta\t\x94V[\x90V[a\t\xC6`\x03_\x90a\t\xACV[\x90V[4a\t\xF9Wa\t\xD96`\x04a\x03CV[a\t\xF5a\t\xE4a\t\xBAV[a\t\xECa\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[4a\n,Wa\n\x0E6`\x04a\x03CV[a\n\x16a\x1A\xAEV[a\n\x1Ea\x02BV[\x80a\n(\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x91``\x83\x83\x03\x12a\n~Wa\nH\x82_\x85\x01a\x07\xD0V[\x92a\nV\x83` \x83\x01a\x07\xD0V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\nyWa\nu\x92\x01a\x02`V[\x90\x91V[a\x02PV[a\x02LV[4a\n\xB7Wa\n\xB3a\n\xA2a\n\x996`\x04a\n1V[\x92\x91\x90\x91a\x1BfV[a\n\xAAa\x02BV[\x91\x82\x91\x82a\x05\x17V[\x03\x90\xF3[a\x02HV[4a\n\xECWa\n\xCC6`\x04a\x03CV[a\n\xE8a\n\xD7a\x1C3V[a\n\xDFa\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[4a\x0B Wa\x0B\na\x0B\x046`\x04a\x05\x95V[\x90a\x1D@V[a\x0B\x12a\x02BV[\x80a\x0B\x1C\x81a\x03\nV[\x03\x90\xF3[a\x02HV[4a\x0BUWa\x0B56`\x04a\x03CV[a\x0BQa\x0B@a\x1DLV[a\x0BHa\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[\x90\x91``\x82\x84\x03\x12a\x0B\x8FWa\x0B\x8Ca\x0Bu\x84_\x85\x01a\x02\xB6V[\x93a\x0B\x83\x81` \x86\x01a\x02\xB6V[\x93`@\x01a\x02\xB6V[\x90V[a\x02LV[4a\x0B\xC5Wa\x0B\xC1a\x0B\xB0a\x0B\xAA6`\x04a\x0BZV[\x91a\x1E\x13V[a\x0B\xB8a\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[a\x0B\xD7`\x04`\x01\x90a\x06\x8EV[\x90V[4a\x0C\nWa\x0B\xEA6`\x04a\x03CV[a\x0C\x06a\x0B\xF5a\x0B\xCAV[a\x0B\xFDa\x02BV[\x91\x82\x91\x82a\x05\x17V[\x03\x90\xF3[a\x02HV[4a\x0C?Wa\x0C\x1F6`\x04a\x03CV[a\x0C;a\x0C*a\x1E\x89V[a\x0C2a\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[a\x0CM\x90a\x07\xB0V[\x90RV[\x91\x90a\x0Cd\x90_` \x85\x01\x94\x01\x90a\x0CDV[V[4a\x0C\x96Wa\x0Cv6`\x04a\x03CV[a\x0C\x92a\x0C\x81a\x1E\xD8V[a\x0C\x89a\x02BV[\x91\x82\x91\x82a\x0CQV[\x03\x90\xF3[a\x02HV[4a\x0C\xCBWa\x0C\xAB6`\x04a\x03CV[a\x0C\xC7a\x0C\xB6a\x1F\x0CV[a\x0C\xBEa\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[a\x0C\xDC`\x02_\x90a\t\xACV[\x90V[4a\r\x0FWa\x0C\xEF6`\x04a\x03CV[a\r\x0Ba\x0C\xFAa\x0C\xD0V[a\r\x02a\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[4a\rDWa\r$6`\x04a\x03CV[a\r@a\r/a\x1FXV[a\r7a\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[\x90` \x82\x82\x03\x12a\rzW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\ruWa\rq\x92\x01a\x03\xF1V[\x90\x91V[a\x02PV[a\x02LV[4a\r\xAEWa\r\x98a\r\x926`\x04a\rIV[\x90a \x83V[a\r\xA0a\x02BV[\x80a\r\xAA\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x90` \x82\x82\x03\x12a\r\xCCWa\r\xC9\x91_\x01a\x07\xD0V[\x90V[a\x02LV[4a\r\xFFWa\r\xE9a\r\xE46`\x04a\r\xB3V[a!3V[a\r\xF1a\x02BV[\x80a\r\xFB\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0EXWa\x0E86`\x04a\x03CV[a\x0ETa\x0ECa\x0E\x04V[a\x0EKa\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[4a\x0E\x8BWa\x0Em6`\x04a\x03CV[a\x0Eua!ZV[a\x0E}a\x02BV[\x80a\x0E\x87\x81a\x03\nV[\x03\x90\xF3[a\x02HV[a\x0E\xA4a\x0E\x9Fa\x0E\xA9\x92a\x02\x9FV[a\x06\xE3V[a\x02\x9FV[\x90V[\x90a\x0E\xB6\x90a\x0E\x90V[_R` R`@_ \x90V[_\x1C\x90V[a\x0E\xD3a\x0E\xD8\x91a\x0E\xC2V[a\t\x91V[\x90V[a\x0E\xE5\x90Ta\x0E\xC7V[\x90V[a\x0E\xF3\x90`\x05a\x0E\xACV[\x90a\x0E\xFF_\x83\x01a\x0E\xDBV[\x91a\x0F\x0C`\x01\x82\x01a\x0E\xDBV[\x91a\x0F%`\x03a\x0F\x1E`\x02\x85\x01a\x0E\xDBV[\x93\x01a\x0E\xDBV[\x90V[a\x0F]a\x0Fd\x94a\x0FS``\x94\x98\x97\x95a\x0FI`\x80\x86\x01\x9A_\x87\x01\x90a\x07\x1AV[` \x85\x01\x90a\x07\x1AV[`@\x83\x01\x90a\x07\x1AV[\x01\x90a\x07\x1AV[V[4a\x0F\x9AWa\x0F\x96a\x0F\x81a\x0F|6`\x04a\x08@V[a\x0E\xE8V[\x90a\x0F\x8D\x94\x92\x94a\x02BV[\x94\x85\x94\x85a\x0F(V[\x03\x90\xF3[a\x02HV[4a\x0F\xCDWa\x0F\xB7a\x0F\xB26`\x04a\r\xB3V[a!\xC9V[a\x0F\xBFa\x02BV[\x80a\x0F\xC9\x81a\x03\nV[\x03\x90\xF3[a\x02HV[\x91\x90`@\x83\x82\x03\x12a\x0F\xFAW\x80a\x0F\xEEa\x0F\xF7\x92_\x86\x01a\x02\xB6V[\x93` \x01a\x02\xB6V[\x90V[a\x02LV[4a\x100Wa\x10,a\x10\x1Ba\x10\x156`\x04a\x0F\xD2V[\x90a\"VV[a\x10#a\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[a\x10A`\x06_\x90a\t\xACV[\x90V[4a\x10tWa\x10T6`\x04a\x03CV[a\x10pa\x10_a\x105V[a\x10ga\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xF3[a\x02HV[_\x80\xFD[\x91\x90a\x10\x9Aa\x10\x9432\x90\x86\x85\x91\x92\x90\x91\x92a\x1BfV[\x15a\x05\x05V[a\x10\xA9Wa\x10\xA7\x92a\x11\x06V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x10\xC1`\x04\x82\x01a\x03\nV[\x03\x90\xFD[a\x10\xCE\x90a\t\"V[\x90V[`@\x90a\x10\xFDa\x10\xF2a\x11\x04\x95\x97\x96\x94``\x84\x01\x90\x84\x82\x03_\x86\x01Ra\x05\xEDV[\x96` \x83\x01\x90a\x07\x1AV[\x01\x90a\x07\x1AV[V[\x90a\x11\x12\x903\x92a\x16xV[\x91B\x92a\x11Ta\x11B\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x10\xC5V[\x94a\x11Ka\x02BV[\x93\x84\x93\x84a\x10\xD1V[\x03\x90\xA2V[\x90a\x11d\x92\x91a\x10}V[V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x11\x84\x90a\x05\xE3V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x11\x9EW`@RV[a\x11fV[\x90a\x11\xB6a\x11\xAFa\x02BV[\x92\x83a\x11zV[V[a\x11\xC2`\x80a\x11\xA3V[\x90V[_\x90V[a\x11\xD1a\x11\xB8V[\x90` \x80\x80\x80\x85a\x11\xE0a\x11\xC5V[\x81R\x01a\x11\xEBa\x11\xC5V[\x81R\x01a\x11\xF6a\x11\xC5V[\x81R\x01a\x12\x01a\x11\xC5V[\x81RPPV[a\x12\x0Fa\x11\xC9V[\x90V[a\x12\x1Aa\x12\x07V[Pa\x12#a\"\xCBV[\x90V[P\x90V[P\x90V[` \x91\x81R\x01\x90V[` \x7Fve the same length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x7FData and priority arrays must ha_\x82\x01R\x01RV[a\x12\x91`2`@\x92a\x12.V[a\x12\x9A\x81a\x127V[\x01\x90V[a\x12\xB3\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x12\x84V[\x90V[\x15a\x12\xBDWV[a\x12\xC5a\x02BV[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x12\xDB`\x04\x82\x01a\x12\x9EV[\x03\x90\xFD[\x90V[a\x12\xF6a\x12\xF1a\x12\xFB\x92a\x12\xDFV[a\x06\xE3V[a\x02\x9FV[\x90V[`\x01a\x13\n\x91\x01a\x02\x9FV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x13oW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x13jW` \x01\x91`\x01\x82\x026\x03\x83\x13a\x13eWV[a\x13)V[a\x13%V[a\x13!V[\x90\x82\x10\x15a\x13\x8FW` a\x13\x8B\x92\x02\x81\x01\x90a\x13-V[\x90\x91V[a\x13\rV[\x91\x90\x81\x10\x15a\x13\xA4W` \x02\x01\x90V[a\x13\rV[5a\x13\xB3\x81a\x02\xA2V[\x90V[\x90\x92a\x13\xC3\x82\x85\x90a\x12&V[\x93a\x13\xEA\x85a\x13\xE4a\x13\xDEa\x13\xD9\x88\x87\x90a\x12*V[a\x02\x9FV[\x91a\x02\x9FV[\x14a\x12\xB6V[a\x13\xF3_a\x12\xE2V[[\x80a\x14\x07a\x14\x01\x88a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a\x14\xAEWa\x145\x90a\x14+32\x90a\x14#\x88\x87\x86\x91a\x13tV[\x92\x90\x91a\x1BfV[a\x14:W[a\x12\xFEV[a\x13\xF4V[3a\x14Pa\x14J\x87\x86\x85\x91a\x13tV[\x90a\x16xV[\x90a\x14ea\x14`\x89\x88\x86\x91a\x13\x94V[a\x13\xA9V[B\x92a\x14\xA6a\x14\x94\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x10\xC5V[\x94a\x14\x9Da\x02BV[\x93\x84\x93\x84a\x10\xD1V[\x03\x90\xA2a\x140V[PPPPPPV[_\x90V[a\x14\xC6a\x14\xCB\x91a\x0E\xC2V[a\x06pV[\x90V[a\x14\xD8\x90Ta\x14\xBAV[\x90V[a\x14\xE3a\x14\xB6V[Pa\x14\xEE`\x04a\x14\xCEV[\x90V[\x91\x90a\x15\x0Ea\x15\x0832\x90\x86\x85\x91\x92\x90\x91\x92a\x1BfV[\x15a\x05\x05V[a\x15\x1DWa\x15\x1B\x92a\x15\x9CV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x155`\x04\x82\x01a\x03\nV[\x03\x90\xFD[\x90\x82_\x93\x92\x827\x01RV[\x91\x90a\x15^\x81a\x15W\x81a\x15c\x95a\x05\xCFV[\x80\x95a\x159V[a\x05\xE3V[\x01\x90V[a\x15\x93a\x15\x88`@\x93a\x15\x9A\x96\x98\x97\x95``\x85\x01\x91\x85\x83\x03_\x87\x01Ra\x15DV[\x96` \x83\x01\x90a\x07\x1AV[\x01\x90a\x07\x1AV[V[\x90\x913\x91\x92\x90\x92a\x15\xE3Ba\x15\xD1\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x95a\x10\xC5V[\x95a\x15\xDAa\x02BV[\x94\x85\x94\x85a\x15gV[\x03\x90\xA2V[\x90a\x15\xF3\x92\x91a\x14\xF1V[V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x16\x1Da\x16\x18a\x16\"\x92a\x12\xDFV[a\x16\x03V[a\x15\xFAV[\x90V[\x90V[a\x164a\x169\x91a\x15\xFAV[a\x16%V[\x90RV[\x90P\x90V[\x90\x91\x82a\x16R\x81a\x16Y\x93a\x16=V[\x80\x93a\x159V[\x01\x90V[\x80a\x16n`\x01\x92a\x16u\x96\x94a\x16(V[\x01\x91a\x16BV[\x90V[a\x16\xB6\x90a\x16\x84a\x15\xF5V[Pa\x16\xA7a\x16\x91_a\x16\tV[\x91\x93a\x16\x9Ba\x02BV[\x94\x85\x93` \x85\x01a\x16]V[` \x82\x01\x81\x03\x82R\x03\x82a\x11zV[\x90V[\x90a\x16\xD5a\x16\xCF32\x90\x85\x85\x91\x92\x90\x91\x92a\x1BfV[\x15a\x05\x05V[a\x16\xE4Wa\x16\xE2\x91a\x17`V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x16\xFC`\x04\x82\x01a\x03\nV[\x03\x90\xFD[`\x08\x1C\x90V[a\x17\x12a\x17\x17\x91a\x17\0V[a\x06pV[\x90V[a\x17$\x90Ta\x17\x06V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x17Ja\x17P\x91\x93\x92\x93a\x02\x9FV[\x92a\x02\x9FV[\x82\x03\x91\x82\x11a\x17[WV[a\x17'V[\x90a\x17ta\x17n`\x04a\x17\x1AV[\x15a\x05\x05V[a\x17\x98Wa\x17\x96\x91a\x17\x8Aa\x17\x91\x92Z\x92a\x17\xBDV[Z\x90a\x17;V[a$\xA5V[V[a\x17\xA1\x91a\x17\xBDV[V[\x90\x91a\x17\xBA\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x15DV[\x90V[3\x90\x91a\x17\xEA\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\xC5V[\x92a\x17\xFFa\x17\xF6a\x02BV[\x92\x83\x92\x83a\x17\xA3V[\x03\x90\xA2V[\x90a\x18\x0E\x91a\x16\xB9V[V[\x90a\x18\"\x91a\x18\x1Da%\xACV[a\x19(V[V[`\xA0\x1C\x90V[a\x186a\x18;\x91a\x18$V[a\x06pV[\x90V[a\x18H\x90Ta\x18*V[\x90V[a\x18_a\x18Za\x18d\x92a\x12\xDFV[a\x06\xE3V[a\x07\xA5V[\x90V[a\x18p\x90a\x18KV[\x90V[`\xA0\x1B\x90V[\x90a\x18\x88`\xFF`\xA0\x1B\x91a\x18sV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18\x9B\x90a\x05\x05V[\x90V[\x90V[\x90a\x18\xB6a\x18\xB1a\x18\xBD\x92a\x18\x92V[a\x18\x9EV[\x82Ta\x18yV[\x90UV[a\x18\xCA\x90a\t\x06V[\x90V[a\x18\xD6\x90a\x18\xC1V[\x90V[_\x1B\x90V[\x90a\x18\xEF`\x01\x80`\xA0\x1B\x03\x91a\x18\xD9V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x19\x02\x90a\x18\xC1V[\x90V[\x90V[\x90a\x19\x1Da\x19\x18a\x19$\x92a\x18\xF9V[a\x19\x05V[\x82Ta\x18\xDEV[\x90UV[a\x192`\x01a\x18>V[a\x19\x9AW\x81a\x19Qa\x19Ka\x19F_a\x18gV[a\x07\xB0V[\x91a\x07\xB0V[\x14a\x19~Wa\x19wa\x19pa\x19|\x93a\x19k`\x01\x80a\x18\xA1V[a\x18\xCDV[`\x01a\x19\x08V[a!\xC9V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x19\x96`\x04\x82\x01a\x03\nV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x19\xB1`\x04\x82\x01a\x03\nV[\x03\x90\xFD[\x90a\x19\xBF\x91a\x18\x10V[V[\x90a\x19\xCB\x90a\x02\x9FV[\x90RV[\x90a\x1A6a\x1A-`\x03a\x19\xE0a\x11\xB8V[\x94a\x19\xF7a\x19\xEF_\x83\x01a\x0E\xDBV[_\x88\x01a\x19\xC1V[a\x1A\x0Fa\x1A\x06`\x01\x83\x01a\x0E\xDBV[` \x88\x01a\x19\xC1V[a\x1A'a\x1A\x1E`\x02\x83\x01a\x0E\xDBV[`@\x88\x01a\x19\xC1V[\x01a\x0E\xDBV[``\x84\x01a\x19\xC1V[V[a\x1AA\x90a\x19\xCFV[\x90V[a\x1A[a\x1A`\x91a\x1ASa\x12\x07V[P`\x05a\x0E\xACV[a\x1A8V[\x90V[a\x1Aka%\xACV[a\x1Asa\x1AuV[V[a\x1A}a&7V[V[a\x1A\x87a\x1AcV[V[a\x1A\x91a%\xACV[a\x1A\x99a\x1A\x9BV[V[a\x1A\xACa\x1A\xA7_a\x18gV[a&gV[V[a\x1A\xB6a\x1A\x89V[V[a\x1A\xC4a\x1A\xC9\x91a\x0E\xC2V[a\x08\xC6V[\x90V[a\x1A\xD6\x90Ta\x1A\xB8V[\x90V[`\xE0\x1B\x90V[a\x1A\xE8\x81a\x05\x05V[\x03a\x1A\xEFWV[_\x80\xFD[\x90PQ\x90a\x1B\0\x82a\x1A\xDFV[V[\x90` \x82\x82\x03\x12a\x1B\x1BWa\x1B\x18\x91_\x01a\x1A\xF3V[\x90V[a\x02LV[a\x1BFa\x1BS\x95\x93\x94\x92\x94a\x1B<``\x84\x01\x96_\x85\x01\x90a\x0CDV[` \x83\x01\x90a\x0CDV[`@\x81\x85\x03\x91\x01Ra\x15DV[\x90V[a\x1B^a\x02BV[=_\x82>=\x90\xFD[\x92a\x1B\xA9` \x93\x94a\x1Bva\x14\xB6V[Pa\x1B\xB4a\x1B\x8Ca\x1B\x87`\x01a\x1A\xCCV[a\t.V[\x93cz9y\xDC\x92\x95\x97a\x1B\x9Da\x02BV[\x98\x89\x97\x88\x96\x87\x96a\x1A\xD9V[\x86R`\x04\x86\x01a\x1B V[\x03\x91Z\xFA\x90\x81\x15a\x1B\xF8W_\x91a\x1B\xCAW[P\x90V[a\x1B\xEB\x91P` =\x81\x11a\x1B\xF1W[a\x1B\xE3\x81\x83a\x11zV[\x81\x01\x90a\x1B\x02V[_a\x1B\xC6V[P=a\x1B\xD9V[a\x1BVV[_\x90V[a\x1C\x0B\x90Qa\x02\x9FV[\x90V[a\x1C\x1Da\x1C#\x91\x93\x92\x93a\x02\x9FV[\x92a\x02\x9FV[\x82\x01\x80\x92\x11a\x1C.WV[a\x17'V[a\x1C;a\x1B\xFDV[Pa\x1Cba\x1CI`\x06a\x0E\xDBV[a\x1C\\``a\x1CVa\"\xCBV[\x01a\x1C\x01V[\x90a\x1C\x0EV[\x90V[\x90a\x1C\x81a\x1C{32\x90\x85\x85\x91\x92\x90\x91\x92a\x1BfV[\x15a\x05\x05V[a\x1C\x90Wa\x1C\x8E\x91a\x1C\xACV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x1C\xA8`\x04\x82\x01a\x03\nV[\x03\x90\xFD[\x90a\x1C\xC0a\x1C\xBA`\x04a\x17\x1AV[\x15a\x05\x05V[a\x1C\xE4Wa\x1C\xE2\x91a\x1C\xD6a\x1C\xDD\x92Z\x92a\x1C\xEFV[Z\x90a\x17;V[a$\xA5V[V[a\x1C\xED\x91a\x1C\xEFV[V[\x90a\x1C\xFB\x903\x92a\x16xV[\x90a\x1D;a\x1D)\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\xC5V[\x92a\x1D2a\x02BV[\x91\x82\x91\x82a\x06\x1EV[\x03\x90\xA2V[\x90a\x1DJ\x91a\x1CeV[V[a\x1DTa\x1B\xFDV[Pa\x1D]a\"\xCBV[a\x1Dh_\x82\x01a\x1C\x01V[a\x1Dza\x1Dt_a\x12\xE2V[\x91a\x02\x9FV[\x14a\x1D\xD1Wa\x1D\x8D_a\x1D\x9D\x92\x01a\x1C\x01V[a\x1D\x97`\x02a\x0E\xDBV[\x90a\x1C\x0EV[Ba\x1D\xB0a\x1D\xAA\x83a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a\x1D\xC4Wa\x1D\xC1\x90B\x90a\x17;V[\x90V[Pa\x1D\xCE_a\x12\xE2V[\x90V[Pa\x1D\xDB_a\x12\xE2V[\x90V[a\x1D\xEDa\x1D\xF3\x91\x93\x92\x93a\x02\x9FV[\x92a\x02\x9FV[\x91a\x1D\xFF\x83\x82\x02a\x02\x9FV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1E\x0EWV[a\x17'V[\x91a\x1E\x1Ca\x1B\xFDV[P\x80a\x1E0a\x1E*\x84a\x02\x9FV[\x91a\x02\x9FV[\x11\x15a\x1E\x84Wa\x1EQ\x91a\x1EC\x91a\x17;V[a\x1EKa\x07\x02V[\x90a\x1D\xDEV[\x80a\x1Eda\x1E^\x84a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a\x1EvWa\x1Es\x91a\x17;V[\x90V[PPa\x1E\x81_a\x12\xE2V[\x90V[PP\x90V[a\x1E\x91a\x1B\xFDV[Pa\x1E\xA5``a\x1E\x9Fa\"\xCBV[\x01a\x1C\x01V[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1E\xC3a\x1E\xC8\x91a\x0E\xC2V[a\x1E\xACV[\x90V[a\x1E\xD5\x90Ta\x1E\xB7V[\x90V[a\x1E\xE0a\x1E\xA8V[Pa\x1E\xEA_a\x1E\xCBV[\x90V[\x90V[a\x1F\x04a\x1E\xFFa\x1F\t\x92a\x1E\xEDV[a\x06\xE3V[a\x02\x9FV[\x90V[a\x1F\x14a\x1B\xFDV[Pa\x1F(a\x1F\"`\x04a\x14\xCEV[\x15a\x05\x05V[a\x1FLWa\x1FIa\x1F9`\x03a\x0E\xDBV[a\x1FC`\x01a\x1E\xF0V[\x90a\x1C\x0EV[\x90V[a\x1FU_a\x12\xE2V[\x90V[a\x1F`a\x1B\xFDV[Pa\x1Ft`@a\x1Fna\"\xCBV[\x01a\x1C\x01V[\x90V[\x90a\x1F\x8Ba\x1F\x85`\x04a\x17\x1AV[\x15a\x05\x05V[a\x1F\xAFWa\x1F\xAD\x91a\x1F\xA1a\x1F\xA8\x92Z\x92a\x1F\xBAV[Z\x90a\x17;V[a$\xA5V[V[a\x1F\xB8\x91a\x1F\xBAV[V[a\x1F\xC5\x81\x83\x90a\x12&V[\x91a\x1F\xCEa\x1B\xFDV[Pa\x1F\xD8_a\x12\xE2V[[\x80a\x1F\xECa\x1F\xE6\x86a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a }Wa \x1A\x90a \x1032\x90a \x08\x87\x87\x86\x91a\x13tV[\x92\x90\x91a\x1BfV[a \x1FW[a\x12\xFEV[a\x1F\xD9V[3a 5a /\x86\x86\x85\x91a\x13tV[\x90a\x16xV[\x90a ua c\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\xC5V[\x92a la\x02BV[\x91\x82\x91\x82a\x06\x1EV[\x03\x90\xA2a \x15V[PPPPV[\x90a \x8D\x91a\x1FwV[V[a \xA0\x90a \x9Ba%\xACV[a \xA2V[V[\x80a \xBDa \xB7a \xB2_a\x18gV[a\x07\xB0V[\x91a\x07\xB0V[\x14a!\x17Wa \xD5a \xCE\x82a\x18\xCDV[`\x01a\x19\x08V[a \xFF\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10\xC5V[\x90a!\x08a\x02BV[\x80a!\x12\x81a\x03\nV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a!/`\x04\x82\x01a\x03\nV[\x03\x90\xFD[a!<\x90a \x8FV[V[a!Fa%\xACV[a!Na!PV[V[a!Xa&\xC6V[V[a!ba!>V[V[a!u\x90a!pa%\xACV[a!wV[V[\x80a!\x92a!\x8Ca!\x87_a\x18gV[a\x07\xB0V[\x91a\x07\xB0V[\x14a!\xA2Wa!\xA0\x90a&gV[V[a!\xC5a!\xAE_a\x18gV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x0CQV[\x03\x90\xFD[a!\xD2\x90a!dV[V[_\x7FGasCounter: invalid range\0\0\0\0\0\0\0\x91\x01RV[a\"\x08`\x19` \x92a\x12.V[a\"\x11\x81a!\xD4V[\x01\x90V[a\"*\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra!\xFBV[\x90V[\x15a\"4WV[a\"<a\x02BV[bF\x1B\xCD`\xE5\x1B\x81R\x80a\"R`\x04\x82\x01a\"\x15V[\x03\x90\xFD[a\"\x85\x91a\"ba\x1B\xFDV[Pa\"\x80\x81a\"ya\"s\x85a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a\"-V[a\x17;V[\x90V[a\"\x92`\x80a\x11\xA3V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\"\xB5a\"\xBB\x91a\x02\x9FV[\x91a\x02\x9FV[\x90\x81\x15a\"\xC6W\x04\x90V[a\"\x95V[a\"\xD3a\x12\x07V[Pa\"\xE7a\"\xE1`\x04a\x14\xCEV[\x15a\x05\x05V[a#\xE9Wa#\x08a#\x03`\x05a\"\xFD`\x03a\x0E\xDBV[\x90a\x0E\xACV[a\x1A8V[Ba#8a#2a#-a#\x1D_\x86\x01a\x1C\x01V[a#'`\x02a\x0E\xDBV[\x90a\x1C\x0EV[a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a#AW\x90V[a#\x92\x90a#\x8Ca#{_a#ta#dBa#^\x84\x88\x01a\x1C\x01V[\x90a\x17;V[a#n`\x02a\x0E\xDBV[\x90a\"\xA9V[\x93\x01a\x1C\x01V[\x91a#\x86`\x02a\x0E\xDBV[\x90a\x1D\xDEV[\x90a\x1C\x0EV[a#\xE6a#\xDD_a#\xD8a#\xCF_a#\xCAa#\xC1_\x95a#\xBCa#\xB3a\"\x88V[\x9A_\x8C\x01a\x19\xC1V[a\x12\xE2V[` \x89\x01a\x19\xC1V[a\x12\xE2V[`@\x86\x01a\x19\xC1V[a\x12\xE2V[``\x83\x01a\x19\xC1V[\x90V[_a$Fa$=_a$8a$/_a$*a$!_\x95a$\x1Ca$\x14a$\x0Ea\"\x88V[\x9Ba\x12\xE2V[_\x8C\x01a\x19\xC1V[a\x12\xE2V[` \x89\x01a\x19\xC1V[a\x12\xE2V[`@\x86\x01a\x19\xC1V[a\x12\xE2V[``\x83\x01a\x19\xC1V[\x90V[\x90a$U_\x19\x91a\x18\xD9V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a$wa$ra$~\x92a\x0E\x90V[a$_V[\x82Ta$IV[\x90UV[\x91` a$\xA3\x92\x94\x93a$\x9C`@\x82\x01\x96_\x83\x01\x90a\x07\x1AV[\x01\x90a\x07\x1AV[V[a$\xB8a$\xB2`\x04a\x17\x1AV[\x15a\x05\x05V[a%\xA9Wa$\xCFa$\xC9`\x04a\x14\xCEV[\x15a\x05\x05V[a%\x9CW[a$\xDCa(\x9CV[a%Ma$\xEA\x82:\x90a\x1D\xDEV[a%\x1E\x83a%\x18`\x02a%\x08`\x05a%\x02`\x03a\x0E\xDBV[\x90a\x0E\xACV[\x01\x91a%\x13\x83a\x0E\xDBV[a\x1C\x0EV[\x90a$bV[a%G`\x03a%7`\x05a%1\x83a\x0E\xDBV[\x90a\x0E\xACV[\x01\x91a%B\x83a\x0E\xDBV[a\x1C\x0EV[\x90a$bV[a%W`\x03a\x0E\xDBV[:a%\x82\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0E\x90V[\x92a%\x97a%\x8Ea\x02BV[\x92\x83\x92\x83a$\x82V[\x03\x90\xA2V[a%\xA4a'\x91V[a$\xD4V[PV[a%\xB4a\x1E\xD8V[a%\xCDa%\xC7a%\xC2a*\xA0V[a\x07\xB0V[\x91a\x07\xB0V[\x03a%\xD4WV[a%\xF6a%\xDFa*\xA0V[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\x0CQV[\x03\x90\xFD[`\x08\x1B\x90V[\x90a&\ra\xFF\0\x91a%\xFAV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a&,a&'a&3\x92a\x18\x92V[a\x18\x9EV[\x82Ta&\0V[\x90UV[a&B_`\x04a&\x17V[V[\x90V[\x90a&\\a&Wa&c\x92a\x10\xC5V[a&DV[\x82Ta\x18\xDEV[\x90UV[a&p_a\x1E\xCBV[a&z\x82_a&GV[\x90a&\xAEa&\xA8\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10\xC5V[\x91a\x10\xC5V[\x91a&\xB7a\x02BV[\x80a&\xC1\x81a\x03\nV[\x03\x90\xA3V[a&\xD2`\x01`\x04a&\x17V[V[\x90a&\xE0`\xFF\x91a\x18\xD9V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a&\xFFa&\xFAa'\x06\x92a\x18\x92V[a\x18\x9EV[\x82Ta&\xD4V[\x90UV[\x90a'\x14\x90a\x12\xE2V[_R` R`@_ \x90V[\x90a'}```\x03a'\x83\x94a'C_\x82\x01a'=_\x88\x01a\x1C\x01V[\x90a$bV[a'\\`\x01\x82\x01a'V` \x88\x01a\x1C\x01V[\x90a$bV[a'u`\x02\x82\x01a'o`@\x88\x01a\x1C\x01V[\x90a$bV[\x01\x92\x01a\x1C\x01V[\x90a$bV[V[\x90a'\x8F\x91a' V[V[a'\xA4a'\x9E`\x04a\x14\xCEV[\x15a\x05\x05V[a'\xABW[V[a'\xB7`\x01`\x04a&\xEAV[a'\xCAa'\xC3_a\x12\xE2V[`\x03a$bV[a(3Ba(\"a(\x19_a(\x14a(\x0B_a(\x06a'\xFD_\x95a'\xF8a'\xEFa\"\x88V[\x9A_\x8C\x01a\x19\xC1V[a\x12\xE2V[` \x89\x01a\x19\xC1V[a\x12\xE2V[`@\x86\x01a\x19\xC1V[a\x12\xE2V[``\x83\x01a\x19\xC1V[a(.`\x05_\x90a'\nV[a'\x85V[_B\x90a(ua(c\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x12\xE2V[\x92a(la\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xA2a'\xA9V[\x90V[a(\x89\x90a\x02\x9FV[_\x19\x81\x14a(\x97W`\x01\x01\x90V[a\x17'V[a(\xB9a(\xB4`\x05a(\xAE`\x03a\x0E\xDBV[\x90a\x0E\xACV[a(}V[Ba(\xE9a(\xE3a(\xDEa(\xCE_\x86\x01a\x0E\xDBV[a(\xD8`\x02a\x0E\xDBV[\x90a\x1C\x0EV[a\x02\x9FV[\x91a\x02\x9FV[\x10\x15a(\xF3W[PV[a)\x1Da)\x14a)\x04_\x84\x01a\x0E\xDBV[a)\x0E`\x02a\x0E\xDBV[\x90a\x1C\x0EV[`\x01\x83\x01a$bV[a)Ea)>a)/`\x03\x84\x01a\x0E\xDBV[a)9`\x06a\x0E\xDBV[a\x1C\x0EV[`\x06a$bV[a)O`\x03a\x0E\xDBV[a)|a)^`\x02\x84\x01a\x0E\xDBV[\x92a)v_a)o`\x01\x84\x01a\x0E\xDBV[\x92\x01a\x0E\xDBV[\x90a\x17;V[a)\xA6\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0E\x90V[\x92a)\xBBa)\xB2a\x02BV[\x92\x83\x92\x83a$\x82V[\x03\x90\xA2a)\xDAa)\xD3a)\xCE`\x03a\x0E\xDBV[a(\x80V[`\x03a$bV[a*LBa*2a*)_a*$a*\x1B_a*\x16a*\r_\x95a*\x08a)\xFFa\"\x88V[\x9A_\x8C\x01a\x19\xC1V[a\x12\xE2V[` \x89\x01a\x19\xC1V[a\x12\xE2V[`@\x86\x01a\x19\xC1V[a\x12\xE2V[``\x83\x01a\x19\xC1V[a*G`\x05a*A`\x03a\x0E\xDBV[\x90a\x0E\xACV[a'\x85V[a*V`\x03a\x0E\xDBV[B\x90a*\x97a*\x85\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0E\x90V[\x92a*\x8Ea\x02BV[\x91\x82\x91\x82a\x07'V[\x03\x90\xA2_a(\xF0V[a*\xA8a\x1E\xA8V[P3\x90V",
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
    /**Function with signature `cumulativeGasFees()` and selector `0xff7b3084`.
```solidity
function cumulativeGasFees() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeGasFeesCall {}
    ///Container type for the return parameters of the [`cumulativeGasFees()`](cumulativeGasFeesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeGasFeesReturn {
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
            impl ::core::convert::From<cumulativeGasFeesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeGasFeesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeGasFeesCall {
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
            impl ::core::convert::From<cumulativeGasFeesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeGasFeesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeGasFeesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cumulativeGasFeesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cumulativeGasFeesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cumulativeGasFees()";
            const SELECTOR: [u8; 4] = [255u8, 123u8, 48u8, 132u8];
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
    /**Function with signature `getCumulativeGasFees()` and selector `0x7fbd295e`.
```solidity
function getCumulativeGasFees() external view returns (uint256 totalCost);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCumulativeGasFeesCall {}
    ///Container type for the return parameters of the [`getCumulativeGasFees()`](getCumulativeGasFeesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCumulativeGasFeesReturn {
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
            impl ::core::convert::From<getCumulativeGasFeesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCumulativeGasFeesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCumulativeGasFeesCall {
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
            impl ::core::convert::From<getCumulativeGasFeesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCumulativeGasFeesReturn) -> Self {
                    (value.totalCost,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCumulativeGasFeesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { totalCost: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCumulativeGasFeesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCumulativeGasFeesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCumulativeGasFees()";
            const SELECTOR: [u8; 4] = [127u8, 189u8, 41u8, 94u8];
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
    /**Function with signature `getGasFeesInRange(uint256,uint256)` and selector `0xf7b8935e`.
```solidity
function getGasFeesInRange(uint256 startCumulative, uint256 endCumulative) external pure returns (uint256 feesDuring);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGasFeesInRangeCall {
        #[allow(missing_docs)]
        pub startCumulative: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub endCumulative: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getGasFeesInRange(uint256,uint256)`](getGasFeesInRangeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGasFeesInRangeReturn {
        #[allow(missing_docs)]
        pub feesDuring: alloy::sol_types::private::primitives::aliases::U256,
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getGasFeesInRangeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getGasFeesInRangeCall) -> Self {
                    (value.startCumulative, value.endCumulative)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getGasFeesInRangeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        startCumulative: tuple.0,
                        endCumulative: tuple.1,
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
            impl ::core::convert::From<getGasFeesInRangeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getGasFeesInRangeReturn) -> Self {
                    (value.feesDuring,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getGasFeesInRangeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { feesDuring: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGasFeesInRangeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getGasFeesInRangeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getGasFeesInRange(uint256,uint256)";
            const SELECTOR: [u8; 4] = [247u8, 184u8, 147u8, 94u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.startCumulative),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.endCumulative),
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
    /**Function with signature `periodDuration()` and selector `0xb470aade`.
```solidity
function periodDuration() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct periodDurationCall {}
    ///Container type for the return parameters of the [`periodDuration()`](periodDurationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct periodDurationReturn {
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
            impl ::core::convert::From<periodDurationCall> for UnderlyingRustTuple<'_> {
                fn from(value: periodDurationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for periodDurationCall {
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
            impl ::core::convert::From<periodDurationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: periodDurationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for periodDurationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for periodDurationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = periodDurationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "periodDuration()";
            const SELECTOR: [u8; 4] = [180u8, 112u8, 170u8, 222u8];
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
        PRIORITY_DECAY_RATE(PRIORITY_DECAY_RATECall),
        #[allow(missing_docs)]
        appchainId(appchainIdCall),
        #[allow(missing_docs)]
        calculateEffectivePriority(calculateEffectivePriorityCall),
        #[allow(missing_docs)]
        cumulativeGasFees(cumulativeGasFeesCall),
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
        getCumulativeGasFees(getCumulativeGasFeesCall),
        #[allow(missing_docs)]
        getCurrentPeriod(getCurrentPeriodCall),
        #[allow(missing_docs)]
        getCurrentPeriodGasUsed(getCurrentPeriodGasUsedCall),
        #[allow(missing_docs)]
        getCurrentPeriodTimeRemaining(getCurrentPeriodTimeRemainingCall),
        #[allow(missing_docs)]
        getGasFeesInRange(getGasFeesInRangeCall),
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
        periodDuration(periodDurationCall),
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
            [113u8, 80u8, 24u8, 166u8],
            [122u8, 57u8, 121u8, 220u8],
            [127u8, 189u8, 41u8, 94u8],
            [128u8, 78u8, 81u8, 35u8],
            [130u8, 244u8, 74u8, 222u8],
            [131u8, 211u8, 193u8, 21u8],
            [132u8, 250u8, 182u8, 43u8],
            [141u8, 90u8, 35u8, 155u8],
            [141u8, 165u8, 203u8, 91u8],
            [175u8, 247u8, 76u8, 109u8],
            [180u8, 112u8, 170u8, 222u8],
            [198u8, 96u8, 211u8, 243u8],
            [205u8, 175u8, 185u8, 120u8],
            [212u8, 240u8, 235u8, 77u8],
            [216u8, 120u8, 19u8, 66u8],
            [222u8, 31u8, 69u8, 62u8],
            [234u8, 74u8, 17u8, 4u8],
            [242u8, 253u8, 227u8, 139u8],
            [247u8, 184u8, 147u8, 94u8],
            [255u8, 123u8, 48u8, 132u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface
    for SyndicateSequencingChainWithDecayingPriorityCalls {
        const NAME: &'static str = "SyndicateSequencingChainWithDecayingPriorityCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 34usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::PRIORITY_DECAY_RATE(_) => {
                    <PRIORITY_DECAY_RATECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::appchainId(_) => {
                    <appchainIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateEffectivePriority(_) => {
                    <calculateEffectivePriorityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cumulativeGasFees(_) => {
                    <cumulativeGasFeesCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::getCumulativeGasFees(_) => {
                    <getCumulativeGasFeesCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::getGasFeesInRange(_) => {
                    <getGasFeesInRangeCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::periodDuration(_) => {
                    <periodDurationCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                    fn getCumulativeGasFees(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getCumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::getCumulativeGasFees,
                            )
                    }
                    getCumulativeGasFees
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
                    fn periodDuration(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <periodDurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::periodDuration,
                            )
                    }
                    periodDuration
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
                {
                    fn getGasFeesInRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getGasFeesInRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::getGasFeesInRange,
                            )
                    }
                    getGasFeesInRange
                },
                {
                    fn cumulativeGasFees(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <cumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::cumulativeGasFees,
                            )
                    }
                    cumulativeGasFees
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
                Self::cumulativeGasFees(inner) => {
                    <cumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getCumulativeGasFees(inner) => {
                    <getCumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getGasFeesInRange(inner) => {
                    <getGasFeesInRangeCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::periodDuration(inner) => {
                    <periodDurationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::cumulativeGasFees(inner) => {
                    <cumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getCumulativeGasFees(inner) => {
                    <getCumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getGasFeesInRange(inner) => {
                    <getGasFeesInRangeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::periodDuration(inner) => {
                    <periodDurationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
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
        ///Creates a new call builder for the [`cumulativeGasFees`] function.
        pub fn cumulativeGasFees(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, cumulativeGasFeesCall, N> {
            self.call_builder(&cumulativeGasFeesCall {})
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
        ///Creates a new call builder for the [`getCumulativeGasFees`] function.
        pub fn getCumulativeGasFees(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCumulativeGasFeesCall, N> {
            self.call_builder(&getCumulativeGasFeesCall {})
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
        ///Creates a new call builder for the [`getGasFeesInRange`] function.
        pub fn getGasFeesInRange(
            &self,
            startCumulative: alloy::sol_types::private::primitives::aliases::U256,
            endCumulative: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getGasFeesInRangeCall, N> {
            self.call_builder(
                &getGasFeesInRangeCall {
                    startCumulative,
                    endCumulative,
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
        ///Creates a new call builder for the [`periodDuration`] function.
        pub fn periodDuration(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, periodDurationCall, N> {
            self.call_builder(&periodDurationCall {})
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
