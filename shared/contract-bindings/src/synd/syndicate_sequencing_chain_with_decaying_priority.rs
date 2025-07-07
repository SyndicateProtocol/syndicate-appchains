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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GasCounterInstance<P, N> {
        GasCounterInstance::<P, N>::new(address, provider)
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
    pub struct GasCounterInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for GasCounterInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GasCounterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > GasCounterInstance<P, N> {
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
                _network: ::core::marker::PhantomData,
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
    impl<P: ::core::clone::Clone, N> GasCounterInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> GasCounterInstance<P, N> {
            GasCounterInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > GasCounterInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > GasCounterInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
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
    ///0x60a060405234610038576100196100146100e9565b61010a565b61002161003d565b612b7b610521823960805181610e400152612b7b90f35b610043565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b9061006f90610047565b810190811060018060401b0382111761008757604052565b610051565b9061009f61009861003d565b9283610065565b565b5f80fd5b90565b6100b1816100a5565b036100b857565b5f80fd5b905051906100c9826100a8565b565b906020828203126100e4576100e1915f016100bc565b90565b6100a1565b61010761325b803803806100fc8161008c565b9283398101906100cb565b90565b610113906101c2565b565b90565b90565b61012f61012a61013492610115565b610118565b6100a5565b90565b60209181520190565b5f7f41707020636861696e2049442063616e6e6f7420626520300000000000000000910152565b6101746018602092610137565b61017d81610140565b0190565b6101969060208101905f818303910152610167565b90565b156101a057565b6101a861003d565b62461bcd60e51b8152806101be60048201610181565b0390fd5b6101ca61023d565b6101e7816101e06101da5f61011b565b916100a5565b1415610199565b608052565b60081b90565b906101ff61ff00916101ec565b9181191691161790565b151590565b61021790610209565b90565b90565b9061023261022d6102399261020e565b61021a565b82546101f2565b9055565b610245610335565b6102516001600361021d565b565b60a01b90565b9061026860ff60a01b91610253565b9181191691161790565b9061028761028261028e9261020e565b61021a565b8254610259565b9055565b5f0190565b61029f61003d565b3d5f823e3d90fd5b60018060a01b031690565b6102c66102c16102cb926102a7565b610118565b6102a7565b90565b6102d7906102b2565b90565b6102e3906102ce565b90565b5f1b90565b906102fc60018060a01b03916102e6565b9181191691161790565b61030f906102ce565b90565b90565b9061032a61032561033192610306565b610312565b82546102eb565b9055565b61033e336103a2565b6103495f6001610272565b61035161003d565b6101bf810181811060018060401b0382111761039d5761037982916101bf61309c8439610292565b03905ff080156103985761038f610396916102da565b6001610315565b565b610297565b610051565b6103ab90610403565b565b6103c16103bc6103c692610115565b610118565b6102a7565b90565b6103d2906103ad565b90565b6103de906102a7565b90565b6103ea906103d5565b9052565b9190610401905f602085019401906103e1565b565b8061041e6104186104135f6103c9565b6103d5565b916103d5565b1461042e5761042c906104c1565b565b61045161043a5f6103c9565b5f918291631e4fbdf760e01b8352600483016103ee565b0390fd5b5f1c90565b60018060a01b031690565b61047161047691610455565b61045a565b90565b6104839054610465565b90565b61048f906102b2565b90565b61049b90610486565b90565b90565b906104b66104b16104bd92610492565b61049e565b82546102eb565b9055565b6104ca5f610479565b6104d4825f6104a1565b906105086105027f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093610492565b91610492565b9161051161003d565b8061051b81610292565b0390a356fe60806040526004361015610013575b611120565b61001d5f3561024c565b8063050ec13814610247578063086146d21461024257806311992f8c1461023d57806318d5aafe146102385780631c0b636714610233578063366cbab71461022e5780633b6ab2a9146102295780633d44ae8b1461022457806346e2cc091461021f578063485cc9551461021a5780634b2c0706146102155780635467cb48146102105780635b3cd6e21461020b57806361543801146102065780636558954f14610201578063715018a6146101fc5780637a3979dc146101f75780637fbd295e146101f2578063804e5123146101ed57806382f44ade146101e857806383d3c115146101e357806384fab62b146101de5780638d5a239b146101d95780638da5cb5b146101d4578063aff74c6d146101cf578063c660d3f3146101ca578063cdafb978146101c5578063d4f0eb4d146101c0578063d8781342146101bb578063de1f453e146101b6578063ea4a1104146101b1578063ede07bd6146101ac578063f2fde38b146101a7578063f7b8935e146101a25763ff7b30840361000e576110eb565b6110a6565b611046565b611011565b610fa0565b610e97565b610e62565b610e0b565b610db9565b610d4e565b610d19565b610ce4565b610c8d565b610c58565b610c12565b610ba3565b610b6f565b610b3a565b610b01565b610a7c565b610a47565b6109d9565b61096c565b6108a3565b61086e565b61081c565b610781565b61074c565b6106bb565b610646565b610571565b61053c565b6104de565b6103cc565b61031f565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156102aa5781359167ffffffffffffffff83116102a55760200192600183028401116102a057565b61026c565b610268565b610264565b90565b6102bb816102af565b036102c257565b5f80fd5b905035906102d3826102b2565b565b91604083830312610315575f83013567ffffffffffffffff8111610310576103028361030d928601610270565b9390946020016102c6565b90565b610260565b61025c565b5f0190565b3461034e576103386103323660046102d5565b91611200565b610340610252565b8061034a8161031a565b0390f35b610258565b5f91031261035d57565b61025c565b61036b906102af565b9052565b906060806103b5936103875f8201515f860190610362565b61039960208201516020860190610362565b6103ab60408201516040860190610362565b0151910190610362565b565b91906103ca905f6080850194019061036f565b565b346103fc576103dc366004610353565b6103f86103e76112b9565b6103ef610252565b918291826103b7565b0390f35b610258565b909182601f8301121561043b5781359167ffffffffffffffff831161043657602001926020830284011161043157565b61026c565b610268565b610264565b909182601f8301121561047a5781359167ffffffffffffffff831161047557602001926020830284011161047057565b61026c565b610268565b610264565b90916040828403126104d9575f82013567ffffffffffffffff81116104d457836104aa918401610401565b929093602082013567ffffffffffffffff81116104cf576104cb9201610440565b9091565b610260565b610260565b61025c565b34610510576104fa6104f136600461047f565b9291909161145d565b610502610252565b8061050c8161031a565b0390f35b610258565b151590565b61052390610515565b9052565b919061053a905f6020850194019061051a565b565b3461056c5761054c366004610353565b610568610557611582565b61055f610252565b91829182610527565b0390f35b610258565b346105a05761058a6105843660046102d5565b9161168f565b610592610252565b8061059c8161031a565b0390f35b610258565b906020828203126105d6575f82013567ffffffffffffffff81116105d1576105cd9201610270565b9091565b610260565b61025c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61061c61062560209361062a93610613816105db565b938480936105df565b958691016105e8565b6105f3565b0190565b6106439160208201915f8184039101526105fd565b90565b346106775761067361066261065c3660046105a5565b9061171f565b61066a610252565b9182918261062e565b0390f35b610258565b1c90565b60ff1690565b61069690600861069b930261067c565b610680565b90565b906106a99154610686565b90565b6106b860035f9061069e565b90565b346106eb576106cb366004610353565b6106e76106d66106ac565b6106de610252565b91829182610527565b0390f35b610258565b90565b90565b61070a61070561070f926106f0565b6106f3565b6102af565b90565b61071c600a6106f6565b90565b610727610712565b90565b610733906102af565b9052565b919061074a905f6020850194019061072a565b565b3461077c5761075c366004610353565b61077861076761071f565b61076f610252565b91829182610737565b0390f35b610258565b346107b05761079a6107943660046105a5565b906118e1565b6107a2610252565b806107ac8161031a565b0390f35b610258565b60018060a01b031690565b6107c9906107b5565b90565b6107d5816107c0565b036107dc57565b5f80fd5b905035906107ed826107cc565b565b9190604083820312610817578061080b610814925f86016107e0565b936020016107e0565b90565b61025c565b3461084b5761083561082f3660046107ef565b90611a92565b61083d610252565b806108478161031a565b0390f35b610258565b9060208282031261086957610866915f016102c6565b90565b61025c565b3461089e5761089a610889610884366004610850565b611b21565b610891610252565b918291826103b7565b0390f35b610258565b346108d1576108b3366004610353565b6108bb611b5c565b6108c3610252565b806108cd8161031a565b0390f35b610258565b60018060a01b031690565b6108f19060086108f6930261067c565b6108d6565b90565b9061090491546108e1565b90565b61091360015f906108f9565b90565b61092a61092561092f926107b5565b6106f3565b6107b5565b90565b61093b90610916565b90565b61094790610932565b90565b6109539061093e565b9052565b919061096a905f6020850194019061094a565b565b3461099c5761097c366004610353565b610998610987610907565b61098f610252565b91829182610957565b0390f35b610258565b90565b6109b49060086109b9930261067c565b6109a1565b90565b906109c791546109a4565b90565b6109d660025f906109bc565b90565b34610a09576109e9366004610353565b610a056109f46109ca565b6109fc610252565b91829182610737565b0390f35b610258565b90565b610a25610a20610a2a92610a0e565b6106f3565b6102af565b90565b610a3962278d00610a11565b90565b610a44610a2d565b90565b34610a7757610a57366004610353565b610a73610a62610a3c565b610a6a610252565b91829182610737565b0390f35b610258565b34610aaa57610a8c366004610353565b610a94611b8b565b610a9c610252565b80610aa68161031a565b0390f35b610258565b91606083830312610afc57610ac6825f85016107e0565b92610ad483602083016107e0565b92604082013567ffffffffffffffff8111610af757610af39201610270565b9091565b610260565b61025c565b34610b3557610b31610b20610b17366004610aaf565b92919091611c43565b610b28610252565b91829182610527565b0390f35b610258565b34610b6a57610b4a366004610353565b610b66610b55611ceb565b610b5d610252565b91829182610737565b0390f35b610258565b34610b9e57610b88610b823660046105a5565b90611e09565b610b90610252565b80610b9a8161031a565b0390f35b610258565b34610bd357610bb3366004610353565b610bcf610bbe611e15565b610bc6610252565b91829182610737565b0390f35b610258565b9091606082840312610c0d57610c0a610bf3845f85016102c6565b93610c0181602086016102c6565b936040016102c6565b90565b61025c565b34610c4357610c3f610c2e610c28366004610bd8565b91611eda565b610c36610252565b91829182610737565b0390f35b610258565b610c55600360019061069e565b90565b34610c8857610c68366004610353565b610c84610c73610c48565b610c7b610252565b91829182610527565b0390f35b610258565b34610cbd57610c9d366004610353565b610cb9610ca8611f50565b610cb0610252565b91829182610737565b0390f35b610258565b610ccb906107c0565b9052565b9190610ce2905f60208501940190610cc2565b565b34610d1457610cf4366004610353565b610d10610cff611f9f565b610d07610252565b91829182610ccf565b0390f35b610258565b34610d4957610d29366004610353565b610d45610d34611fd3565b610d3c610252565b91829182610737565b0390f35b610258565b34610d7e57610d5e366004610353565b610d7a610d6961201f565b610d71610252565b91829182610737565b0390f35b610258565b90602082820312610db4575f82013567ffffffffffffffff8111610daf57610dab9201610401565b9091565b610260565b61025c565b34610de857610dd2610dcc366004610d83565b9061215b565b610dda610252565b80610de48161031a565b0390f35b610258565b90602082820312610e0657610e03915f016107e0565b90565b61025c565b34610e3957610e23610e1e366004610ded565b61220b565b610e2b610252565b80610e358161031a565b0390f35b610258565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610e9257610e72366004610353565b610e8e610e7d610e3e565b610e85610252565b91829182610737565b0390f35b610258565b34610ec557610ea7366004610353565b610eaf612232565b610eb7610252565b80610ec18161031a565b0390f35b610258565b610ede610ed9610ee3926102af565b6106f3565b6102af565b90565b90610ef090610eca565b5f5260205260405f2090565b5f1c90565b610f0d610f1291610efc565b6109a1565b90565b610f1f9054610f01565b90565b610f2d906004610ee6565b90610f395f8301610f15565b91610f4660018201610f15565b91610f5f6003610f5860028501610f15565b9301610f15565b90565b610f97610f9e94610f8d606094989795610f83608086019a5f87019061072a565b602085019061072a565b604083019061072a565b019061072a565b565b34610fd457610fd0610fbb610fb6366004610850565b610f22565b90610fc7949294610252565b94859485610f62565b0390f35b610258565b90565b610ff0610feb610ff592610fd9565b6106f3565b6102af565b90565b611003611388610fdc565b90565b61100e610ff8565b90565b3461104157611021366004610353565b61103d61102c611006565b611034610252565b91829182610737565b0390f35b610258565b346110745761105e611059366004610ded565b6122a1565b611066610252565b806110708161031a565b0390f35b610258565b91906040838203126110a1578061109561109e925f86016102c6565b936020016102c6565b90565b61025c565b346110d7576110d36110c26110bc366004611079565b9061232e565b6110ca610252565b91829182610737565b0390f35b610258565b6110e860055f906109bc565b90565b3461111b576110fb366004610353565b6111176111066110dc565b61110e610252565b91829182610737565b0390f35b610258565b5f80fd5b919061114161113b33329086859192909192611c43565b15610515565b6111505761114e926111ad565b565b5f631b8e828b60e31b8152806111686004820161031a565b0390fd5b61117590610932565b90565b6040906111a46111996111ab9597969460608401908482035f8601526105fd565b96602083019061072a565b019061072a565b565b906111b990339261171f565b9142926111fb6111e97f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f29461116c565b946111f2610252565b93849384611178565b0390a2565b9061120b9291611124565b565b634e487b7160e01b5f52604160045260245ffd5b9061122b906105f3565b810190811067ffffffffffffffff82111761124557604052565b61120d565b9061125d611256610252565b9283611221565b565b611269608061124a565b90565b5f90565b61127861125f565b9060208080808561128761126c565b81520161129261126c565b81520161129d61126c565b8152016112a861126c565b81525050565b6112b6611270565b90565b6112c16112ae565b506112ca6123a3565b90565b5090565b5090565b60209181520190565b60207f7665207468652073616d65206c656e6774680000000000000000000000000000917f4461746120616e64207072696f7269747920617272617973206d7573742068615f8201520152565b61133860326040926112d5565b611341816112de565b0190565b61135a9060208101905f81830391015261132b565b90565b1561136457565b61136c610252565b62461bcd60e51b81528061138260048201611345565b0390fd5b90565b61139d6113986113a292611386565b6106f3565b6102af565b90565b60016113b191016102af565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b903590600160200381360303821215611416570180359067ffffffffffffffff82116114115760200191600182023603831361140c57565b6113d0565b6113cc565b6113c8565b9082101561143657602061143292028101906113d4565b9091565b6113b4565b919081101561144b576020020190565b6113b4565b3561145a816102b2565b90565b909261146a8285906112cd565b936114918561148b6114856114808887906112d1565b6102af565b916102af565b1461135d565b61149a5f611389565b5b806114ae6114a8886102af565b916102af565b1015611555576114dc906114d23332906114ca8887869161141b565b929091611c43565b6114e1575b6113a5565b61149b565b336114f76114f18786859161141b565b9061171f565b9061150c6115078988869161143b565b611450565b429261154d61153b7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f29461116c565b94611544610252565b93849384611178565b0390a26114d7565b505050505050565b5f90565b61156d61157291610efc565b610680565b90565b61157f9054611561565b90565b61158a61155d565b506115956003611575565b90565b91906115b56115af33329086859192909192611c43565b15610515565b6115c4576115c292611643565b565b5f631b8e828b60e31b8152806115dc6004820161031a565b0390fd5b90825f939282370152565b9190611605816115fe8161160a956105df565b80956115e0565b6105f3565b0190565b61163a61162f6040936116419698979560608501918583035f8701526115eb565b96602083019061072a565b019061072a565b565b9091339192909261168a426116787f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f29561116c565b95611681610252565b9485948561160e565b0390a2565b9061169a9291611598565b565b606090565b60ff60f81b1690565b60f81b90565b6116c46116bf6116c992611386565b6116aa565b6116a1565b90565b90565b6116db6116e0916116a1565b6116cc565b9052565b905090565b9091826116f981611700936116e4565b80936115e0565b0190565b8061171560019261171c96946116cf565b01916116e9565b90565b61175d9061172b61169c565b5061174e6117385f6116b0565b9193611742610252565b94859360208501611704565b60208201810382520382611221565b90565b9061177c61177633329085859192909192611c43565b15610515565b61178b576117899161182c565b565b5f631b8e828b60e31b8152806117a36004820161031a565b0390fd5b60081c90565b6117b96117be916117a7565b610680565b90565b6117cb90546117ad565b90565b634e487b7160e01b5f52601160045260245ffd5b6117f16117f7919392936102af565b926102af565b820391821161180257565b6117ce565b61181661181c919392936102af565b926102af565b820180921161182757565b6117ce565b9061184061183a60036117c1565b15610515565b611875576118606118739261185961186e935a9261189a565b5a906117e2565b611868610ff8565b90611807565b612577565b565b61187e9161189a565b565b90916118979260208301925f8185039101526115eb565b90565b3390916118c77f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f9261116c565b926118dc6118d3610252565b92839283611880565b0390a2565b906118eb91611760565b565b906118ff916118fa61267e565b611a05565b565b60a01c90565b61191361191891611901565b610680565b90565b6119259054611907565b90565b61193c61193761194192611386565b6106f3565b6107b5565b90565b61194d90611928565b90565b60a01b90565b9061196560ff60a01b91611950565b9181191691161790565b61197890610515565b90565b90565b9061199361198e61199a9261196f565b61197b565b8254611956565b9055565b6119a790610916565b90565b6119b39061199e565b90565b5f1b90565b906119cc60018060a01b03916119b6565b9181191691161790565b6119df9061199e565b90565b90565b906119fa6119f5611a01926119d6565b6119e2565b82546119bb565b9055565b611a0f600161191b565b611a775781611a2e611a28611a235f611944565b6107c0565b916107c0565b14611a5b57611a54611a4d611a5993611a4860018061197e565b6119aa565b60016119e5565b6122a1565b565b5f632e7f3c7f60e11b815280611a736004820161031a565b0390fd5b5f62dc149f60e41b815280611a8e6004820161031a565b0390fd5b90611a9c916118ed565b565b90611aa8906102af565b9052565b90611b13611b0a6003611abd61125f565b94611ad4611acc5f8301610f15565b5f8801611a9e565b611aec611ae360018301610f15565b60208801611a9e565b611b04611afb60028301610f15565b60408801611a9e565b01610f15565b60608401611a9e565b565b611b1e90611aac565b90565b611b38611b3d91611b306112ae565b506004610ee6565b611b15565b90565b611b4861267e565b611b50611b52565b565b611b5a612709565b565b611b64611b40565b565b611b6e61267e565b611b76611b78565b565b611b89611b845f611944565b612739565b565b611b93611b66565b565b611ba1611ba691610efc565b6108d6565b90565b611bb39054611b95565b90565b60e01b90565b611bc581610515565b03611bcc57565b5f80fd5b90505190611bdd82611bbc565b565b90602082820312611bf857611bf5915f01611bd0565b90565b61025c565b611c23611c309593949294611c1960608401965f850190610cc2565b6020830190610cc2565b60408185039101526115eb565b90565b611c3b610252565b3d5f823e3d90fd5b92611c8660209394611c5361155d565b50611c91611c69611c646001611ba9565b61093e565b93637a3979dc929597611c7a610252565b98899788968796611bb6565b865260048601611bfd565b03915afa908115611cd5575f91611ca7575b5090565b611cc8915060203d8111611cce575b611cc08183611221565b810190611bdf565b5f611ca3565b503d611cb6565b611c33565b5f90565b611ce890516102af565b90565b611cf3611cda565b50611d1a611d016005610f15565b611d146060611d0e6123a3565b01611cde565b90611807565b90565b90611d39611d3333329085859192909192611c43565b15610515565b611d4857611d4691611d64565b565b5f631b8e828b60e31b815280611d606004820161031a565b0390fd5b90611d78611d7260036117c1565b15610515565b611dad57611d98611dab92611d91611da6935a92611db8565b5a906117e2565b611da0610ff8565b90611807565b612577565b565b611db691611db8565b565b90611dc490339261171f565b90611e04611df27f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f9261116c565b92611dfb610252565b9182918261062e565b0390a2565b90611e1391611d1d565b565b611e1d611cda565b50611e266123a3565b611e315f8201611cde565b611e43611e3d5f611389565b916102af565b14611e9857611e565f611e649201611cde565b611e5e610a2d565b90611807565b42611e77611e71836102af565b916102af565b1015611e8b57611e889042906117e2565b90565b50611e955f611389565b90565b50611ea25f611389565b90565b611eb4611eba919392936102af565b926102af565b91611ec68382026102af565b928184041490151715611ed557565b6117ce565b91611ee3611cda565b5080611ef7611ef1846102af565b916102af565b1115611f4b57611f1891611f0a916117e2565b611f12610712565b90611ea5565b80611f2b611f25846102af565b916102af565b1015611f3d57611f3a916117e2565b90565b5050611f485f611389565b90565b505090565b611f58611cda565b50611f6c6060611f666123a3565b01611cde565b90565b5f90565b60018060a01b031690565b611f8a611f8f91610efc565b611f73565b90565b611f9c9054611f7e565b90565b611fa7611f6f565b50611fb15f611f92565b90565b90565b611fcb611fc6611fd092611fb4565b6106f3565b6102af565b90565b611fdb611cda565b50611fef611fe96003611575565b15610515565b612013576120106120006002610f15565b61200a6001611fb7565b90611807565b90565b61201c5f611389565b90565b612027611cda565b5061203b60406120356123a3565b01611cde565b90565b9061205261204c60036117c1565b15610515565b612087576120726120859261206b612080935a92612092565b5a906117e2565b61207a610ff8565b90611807565b612577565b565b61209091612092565b565b61209d8183906112cd565b916120a6611cda565b506120b05f611389565b5b806120c46120be866102af565b916102af565b1015612155576120f2906120e83332906120e08787869161141b565b929091611c43565b6120f7575b6113a5565b6120b1565b3361210d6121078686859161141b565b9061171f565b9061214d61213b7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f9261116c565b92612144610252565b9182918261062e565b0390a26120ed565b50505050565b906121659161203e565b565b6121789061217361267e565b61217a565b565b8061219561218f61218a5f611944565b6107c0565b916107c0565b146121ef576121ad6121a6826119aa565b60016119e5565b6121d77f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b99161116c565b906121e0610252565b806121ea8161031a565b0390a2565b5f632e7f3c7f60e11b8152806122076004820161031a565b0390fd5b61221490612167565b565b61221e61267e565b612226612228565b565b612230612798565b565b61223a612216565b565b61224d9061224861267e565b61224f565b565b8061226a61226461225f5f611944565b6107c0565b916107c0565b1461227a5761227890612739565b565b61229d6122865f611944565b5f918291631e4fbdf760e01b835260048301610ccf565b0390fd5b6122aa9061223c565b565b5f7f476173436f756e7465723a20696e76616c69642072616e676500000000000000910152565b6122e060196020926112d5565b6122e9816122ac565b0190565b6123029060208101905f8183039101526122d3565b90565b1561230c57565b612314610252565b62461bcd60e51b81528061232a600482016122ed565b0390fd5b61235d9161233a611cda565b506123588161235161234b856102af565b916102af565b1015612305565b6117e2565b90565b61236a608061124a565b90565b634e487b7160e01b5f52601260045260245ffd5b61238d612393916102af565b916102af565b90811561239e570490565b61236d565b6123ab6112ae565b506123bf6123b96003611575565b15610515565b6124bb576123e06123db60046123d56002610f15565b90610ee6565b611b15565b4261240e6124086124036123f55f8601611cde565b6123fd610a2d565b90611807565b6102af565b916102af565b10156124175790565b6124649061245e61244f5f61244861243a42612434848801611cde565b906117e2565b612442610a2d565b90612381565b9301611cde565b91612458610a2d565b90611ea5565b90611807565b6124b86124af5f6124aa6124a15f61249c6124935f9561248e612485612360565b9a5f8c01611a9e565b611389565b60208901611a9e565b611389565b60408601611a9e565b611389565b60608301611a9e565b90565b5f61251861250f5f61250a6125015f6124fc6124f35f956124ee6124e66124e0612360565b9b611389565b5f8c01611a9e565b611389565b60208901611a9e565b611389565b60408601611a9e565b611389565b60608301611a9e565b90565b906125275f19916119b6565b9181191691161790565b90565b9061254961254461255092610eca565b612531565b825461251b565b9055565b91602061257592949361256e60408201965f83019061072a565b019061072a565b565b61258a61258460036117c1565b15610515565b61267b576125a161259b6003611575565b15610515565b61266e575b6125ae61296e565b61261f6125bc823a90611ea5565b6125ef836125e960026125d960046125d383610f15565b90610ee6565b01916125e483610f15565b611807565b90612534565b612619600361260960046126036002610f15565b90610ee6565b019161261483610f15565b611807565b90612534565b6126296002610f15565b3a6126547f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610eca565b92612669612660610252565b92839283612554565b0390a2565b612676612863565b6125a6565b50565b612686611f9f565b61269f612699612694612b6e565b6107c0565b916107c0565b036126a657565b6126c86126b1612b6e565b5f91829163118cdaa760e01b835260048301610ccf565b0390fd5b60081b90565b906126df61ff00916126cc565b9181191691161790565b906126fe6126f96127059261196f565b61197b565b82546126d2565b9055565b6127145f60036126e9565b565b90565b9061272e6127296127359261116c565b612716565b82546119bb565b9055565b6127425f611f92565b61274c825f612719565b9061278061277a7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09361116c565b9161116c565b91612789610252565b806127938161031a565b0390a3565b6127a4600160036126e9565b565b906127b260ff916119b6565b9181191691161790565b906127d16127cc6127d89261196f565b61197b565b82546127a6565b9055565b906127e690611389565b5f5260205260405f2090565b9061284f60606003612855946128155f820161280f5f8801611cde565b90612534565b61282e6001820161282860208801611cde565b90612534565b6128476002820161284160408801611cde565b90612534565b019201611cde565b90612534565b565b90612861916127f2565b565b6128766128706003611575565b15610515565b61287d575b565b612889600160036127bc565b61289c6128955f611389565b6002612534565b612905426128f46128eb5f6128e66128dd5f6128d86128cf5f956128ca6128c1612360565b9a5f8c01611a9e565b611389565b60208901611a9e565b611389565b60408601611a9e565b611389565b60608301611a9e565b61290060045f906127dc565b612857565b5f42906129476129357f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92611389565b9261293e610252565b91829182610737565b0390a261287b565b90565b61295b906102af565b5f1981146129695760010190565b6117ce565b61298b61298660046129806002610f15565b90610ee6565b61294f565b426129b96129b36129ae6129a05f8601610f15565b6129a8610a2d565b90611807565b6102af565b916102af565b10156129c3575b50565b6129eb6129e26129d45f8401610f15565b6129dc610a2d565b90611807565b60018301612534565b612a13612a0c6129fd60038401610f15565b612a076005610f15565b611807565b6005612534565b612a1d6002610f15565b612a4a612a2c60028401610f15565b92612a445f612a3d60018401610f15565b9201610f15565b906117e2565b612a747f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610eca565b92612a89612a80610252565b92839283612554565b0390a2612aa8612aa1612a9c6002610f15565b612952565b6002612534565b612b1a42612b00612af75f612af2612ae95f612ae4612adb5f95612ad6612acd612360565b9a5f8c01611a9e565b611389565b60208901611a9e565b611389565b60408601611a9e565b611389565b60608301611a9e565b612b156004612b0f6002610f15565b90610ee6565b612857565b612b246002610f15565b4290612b65612b537f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610eca565b92612b5c610252565b91829182610737565b0390a25f6129c0565b612b76611f6f565b50339056608060405234601c57600e6020565b61019461002b823961019490f35b6026565b60405190565b5f80fdfe60806040526004361015610013575b61017b565b61001d5f3561002c565b637a3979dc0361000e57610142565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b60018060a01b031690565b61005890610044565b90565b6100648161004f565b0361006b57565b5f80fd5b9050359061007c8261005b565b565b5f80fd5b5f80fd5b5f80fd5b909182601f830112156100c45781359167ffffffffffffffff83116100bf5760200192600183028401116100ba57565b610086565b610082565b61007e565b91606083830312610116576100e0825f850161006f565b926100ee836020830161006f565b92604082013567ffffffffffffffff81116101115761010d920161008a565b9091565b610040565b61003c565b151590565b6101299061011b565b9052565b9190610140905f60208501940190610120565b565b34610176576101726101616101583660046100c9565b92919091610183565b610169610032565b9182918261012d565b0390f35b610038565b5f80fd5b5f90565b5050505061018f61017f565b505f9056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4a\08Wa\0\x19a\0\x14a\0\xE9V[a\x01\nV[a\0!a\0=V[a+{a\x05!\x829`\x80Q\x81a\x0E@\x01Ra+{\x90\xF3[a\0CV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0o\x90a\0GV[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x87W`@RV[a\0QV[\x90a\0\x9Fa\0\x98a\0=V[\x92\x83a\0eV[V[_\x80\xFD[\x90V[a\0\xB1\x81a\0\xA5V[\x03a\0\xB8WV[_\x80\xFD[\x90PQ\x90a\0\xC9\x82a\0\xA8V[V[\x90` \x82\x82\x03\x12a\0\xE4Wa\0\xE1\x91_\x01a\0\xBCV[\x90V[a\0\xA1V[a\x01\x07a2[\x808\x03\x80a\0\xFC\x81a\0\x8CV[\x92\x839\x81\x01\x90a\0\xCBV[\x90V[a\x01\x13\x90a\x01\xC2V[V[\x90V[\x90V[a\x01/a\x01*a\x014\x92a\x01\x15V[a\x01\x18V[a\0\xA5V[\x90V[` \x91\x81R\x01\x90V[_\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01t`\x18` \x92a\x017V[a\x01}\x81a\x01@V[\x01\x90V[a\x01\x96\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x01gV[\x90V[\x15a\x01\xA0WV[a\x01\xA8a\0=V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x01\xBE`\x04\x82\x01a\x01\x81V[\x03\x90\xFD[a\x01\xCAa\x02=V[a\x01\xE7\x81a\x01\xE0a\x01\xDA_a\x01\x1BV[\x91a\0\xA5V[\x14\x15a\x01\x99V[`\x80RV[`\x08\x1B\x90V[\x90a\x01\xFFa\xFF\0\x91a\x01\xECV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x15\x15\x90V[a\x02\x17\x90a\x02\tV[\x90V[\x90V[\x90a\x022a\x02-a\x029\x92a\x02\x0EV[a\x02\x1AV[\x82Ta\x01\xF2V[\x90UV[a\x02Ea\x035V[a\x02Q`\x01`\x03a\x02\x1DV[V[`\xA0\x1B\x90V[\x90a\x02h`\xFF`\xA0\x1B\x91a\x02SV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x02\x87a\x02\x82a\x02\x8E\x92a\x02\x0EV[a\x02\x1AV[\x82Ta\x02YV[\x90UV[_\x01\x90V[a\x02\x9Fa\0=V[=_\x82>=\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x02\xC6a\x02\xC1a\x02\xCB\x92a\x02\xA7V[a\x01\x18V[a\x02\xA7V[\x90V[a\x02\xD7\x90a\x02\xB2V[\x90V[a\x02\xE3\x90a\x02\xCEV[\x90V[_\x1B\x90V[\x90a\x02\xFC`\x01\x80`\xA0\x1B\x03\x91a\x02\xE6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x03\x0F\x90a\x02\xCEV[\x90V[\x90V[\x90a\x03*a\x03%a\x031\x92a\x03\x06V[a\x03\x12V[\x82Ta\x02\xEBV[\x90UV[a\x03>3a\x03\xA2V[a\x03I_`\x01a\x02rV[a\x03Qa\0=V[a\x01\xBF\x81\x01\x81\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\x03\x9DWa\x03y\x82\x91a\x01\xBFa0\x9C\x849a\x02\x92V[\x03\x90_\xF0\x80\x15a\x03\x98Wa\x03\x8Fa\x03\x96\x91a\x02\xDAV[`\x01a\x03\x15V[V[a\x02\x97V[a\0QV[a\x03\xAB\x90a\x04\x03V[V[a\x03\xC1a\x03\xBCa\x03\xC6\x92a\x01\x15V[a\x01\x18V[a\x02\xA7V[\x90V[a\x03\xD2\x90a\x03\xADV[\x90V[a\x03\xDE\x90a\x02\xA7V[\x90V[a\x03\xEA\x90a\x03\xD5V[\x90RV[\x91\x90a\x04\x01\x90_` \x85\x01\x94\x01\x90a\x03\xE1V[V[\x80a\x04\x1Ea\x04\x18a\x04\x13_a\x03\xC9V[a\x03\xD5V[\x91a\x03\xD5V[\x14a\x04.Wa\x04,\x90a\x04\xC1V[V[a\x04Qa\x04:_a\x03\xC9V[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x03\xEEV[\x03\x90\xFD[_\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04qa\x04v\x91a\x04UV[a\x04ZV[\x90V[a\x04\x83\x90Ta\x04eV[\x90V[a\x04\x8F\x90a\x02\xB2V[\x90V[a\x04\x9B\x90a\x04\x86V[\x90V[\x90V[\x90a\x04\xB6a\x04\xB1a\x04\xBD\x92a\x04\x92V[a\x04\x9EV[\x82Ta\x02\xEBV[\x90UV[a\x04\xCA_a\x04yV[a\x04\xD4\x82_a\x04\xA1V[\x90a\x05\x08a\x05\x02\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x04\x92V[\x91a\x04\x92V[\x91a\x05\x11a\0=V[\x80a\x05\x1B\x81a\x02\x92V[\x03\x90\xA3V\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x11 V[a\0\x1D_5a\x02LV[\x80c\x05\x0E\xC18\x14a\x02GW\x80c\x08aF\xD2\x14a\x02BW\x80c\x11\x99/\x8C\x14a\x02=W\x80c\x18\xD5\xAA\xFE\x14a\x028W\x80c\x1C\x0Bcg\x14a\x023W\x80c6l\xBA\xB7\x14a\x02.W\x80c;j\xB2\xA9\x14a\x02)W\x80c=D\xAE\x8B\x14a\x02$W\x80cF\xE2\xCC\t\x14a\x02\x1FW\x80cH\\\xC9U\x14a\x02\x1AW\x80cK,\x07\x06\x14a\x02\x15W\x80cTg\xCBH\x14a\x02\x10W\x80c[<\xD6\xE2\x14a\x02\x0BW\x80caT8\x01\x14a\x02\x06W\x80ceX\x95O\x14a\x02\x01W\x80cqP\x18\xA6\x14a\x01\xFCW\x80cz9y\xDC\x14a\x01\xF7W\x80c\x7F\xBD)^\x14a\x01\xF2W\x80c\x80NQ#\x14a\x01\xEDW\x80c\x82\xF4J\xDE\x14a\x01\xE8W\x80c\x83\xD3\xC1\x15\x14a\x01\xE3W\x80c\x84\xFA\xB6+\x14a\x01\xDEW\x80c\x8DZ#\x9B\x14a\x01\xD9W\x80c\x8D\xA5\xCB[\x14a\x01\xD4W\x80c\xAF\xF7Lm\x14a\x01\xCFW\x80c\xC6`\xD3\xF3\x14a\x01\xCAW\x80c\xCD\xAF\xB9x\x14a\x01\xC5W\x80c\xD4\xF0\xEBM\x14a\x01\xC0W\x80c\xD8x\x13B\x14a\x01\xBBW\x80c\xDE\x1FE>\x14a\x01\xB6W\x80c\xEAJ\x11\x04\x14a\x01\xB1W\x80c\xED\xE0{\xD6\x14a\x01\xACW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xA7W\x80c\xF7\xB8\x93^\x14a\x01\xA2Wc\xFF{0\x84\x03a\0\x0EWa\x10\xEBV[a\x10\xA6V[a\x10FV[a\x10\x11V[a\x0F\xA0V[a\x0E\x97V[a\x0EbV[a\x0E\x0BV[a\r\xB9V[a\rNV[a\r\x19V[a\x0C\xE4V[a\x0C\x8DV[a\x0CXV[a\x0C\x12V[a\x0B\xA3V[a\x0BoV[a\x0B:V[a\x0B\x01V[a\n|V[a\nGV[a\t\xD9V[a\tlV[a\x08\xA3V[a\x08nV[a\x08\x1CV[a\x07\x81V[a\x07LV[a\x06\xBBV[a\x06FV[a\x05qV[a\x05<V[a\x04\xDEV[a\x03\xCCV[a\x03\x1FV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x02\xAAW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x02\xA5W` \x01\x92`\x01\x83\x02\x84\x01\x11a\x02\xA0WV[a\x02lV[a\x02hV[a\x02dV[\x90V[a\x02\xBB\x81a\x02\xAFV[\x03a\x02\xC2WV[_\x80\xFD[\x90P5\x90a\x02\xD3\x82a\x02\xB2V[V[\x91`@\x83\x83\x03\x12a\x03\x15W_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\x10Wa\x03\x02\x83a\x03\r\x92\x86\x01a\x02pV[\x93\x90\x94` \x01a\x02\xC6V[\x90V[a\x02`V[a\x02\\V[_\x01\x90V[4a\x03NWa\x038a\x0326`\x04a\x02\xD5V[\x91a\x12\0V[a\x03@a\x02RV[\x80a\x03J\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[_\x91\x03\x12a\x03]WV[a\x02\\V[a\x03k\x90a\x02\xAFV[\x90RV[\x90``\x80a\x03\xB5\x93a\x03\x87_\x82\x01Q_\x86\x01\x90a\x03bV[a\x03\x99` \x82\x01Q` \x86\x01\x90a\x03bV[a\x03\xAB`@\x82\x01Q`@\x86\x01\x90a\x03bV[\x01Q\x91\x01\x90a\x03bV[V[\x91\x90a\x03\xCA\x90_`\x80\x85\x01\x94\x01\x90a\x03oV[V[4a\x03\xFCWa\x03\xDC6`\x04a\x03SV[a\x03\xF8a\x03\xE7a\x12\xB9V[a\x03\xEFa\x02RV[\x91\x82\x91\x82a\x03\xB7V[\x03\x90\xF3[a\x02XV[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04;W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x046W` \x01\x92` \x83\x02\x84\x01\x11a\x041WV[a\x02lV[a\x02hV[a\x02dV[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04zW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04uW` \x01\x92` \x83\x02\x84\x01\x11a\x04pWV[a\x02lV[a\x02hV[a\x02dV[\x90\x91`@\x82\x84\x03\x12a\x04\xD9W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xD4W\x83a\x04\xAA\x91\x84\x01a\x04\x01V[\x92\x90\x93` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xCFWa\x04\xCB\x92\x01a\x04@V[\x90\x91V[a\x02`V[a\x02`V[a\x02\\V[4a\x05\x10Wa\x04\xFAa\x04\xF16`\x04a\x04\x7FV[\x92\x91\x90\x91a\x14]V[a\x05\x02a\x02RV[\x80a\x05\x0C\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x15\x15\x90V[a\x05#\x90a\x05\x15V[\x90RV[\x91\x90a\x05:\x90_` \x85\x01\x94\x01\x90a\x05\x1AV[V[4a\x05lWa\x05L6`\x04a\x03SV[a\x05ha\x05Wa\x15\x82V[a\x05_a\x02RV[\x91\x82\x91\x82a\x05'V[\x03\x90\xF3[a\x02XV[4a\x05\xA0Wa\x05\x8Aa\x05\x846`\x04a\x02\xD5V[\x91a\x16\x8FV[a\x05\x92a\x02RV[\x80a\x05\x9C\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x90` \x82\x82\x03\x12a\x05\xD6W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05\xD1Wa\x05\xCD\x92\x01a\x02pV[\x90\x91V[a\x02`V[a\x02\\V[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x06\x1Ca\x06%` \x93a\x06*\x93a\x06\x13\x81a\x05\xDBV[\x93\x84\x80\x93a\x05\xDFV[\x95\x86\x91\x01a\x05\xE8V[a\x05\xF3V[\x01\x90V[a\x06C\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x05\xFDV[\x90V[4a\x06wWa\x06sa\x06ba\x06\\6`\x04a\x05\xA5V[\x90a\x17\x1FV[a\x06ja\x02RV[\x91\x82\x91\x82a\x06.V[\x03\x90\xF3[a\x02XV[\x1C\x90V[`\xFF\x16\x90V[a\x06\x96\x90`\x08a\x06\x9B\x93\x02a\x06|V[a\x06\x80V[\x90V[\x90a\x06\xA9\x91Ta\x06\x86V[\x90V[a\x06\xB8`\x03_\x90a\x06\x9EV[\x90V[4a\x06\xEBWa\x06\xCB6`\x04a\x03SV[a\x06\xE7a\x06\xD6a\x06\xACV[a\x06\xDEa\x02RV[\x91\x82\x91\x82a\x05'V[\x03\x90\xF3[a\x02XV[\x90V[\x90V[a\x07\na\x07\x05a\x07\x0F\x92a\x06\xF0V[a\x06\xF3V[a\x02\xAFV[\x90V[a\x07\x1C`\na\x06\xF6V[\x90V[a\x07'a\x07\x12V[\x90V[a\x073\x90a\x02\xAFV[\x90RV[\x91\x90a\x07J\x90_` \x85\x01\x94\x01\x90a\x07*V[V[4a\x07|Wa\x07\\6`\x04a\x03SV[a\x07xa\x07ga\x07\x1FV[a\x07oa\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[4a\x07\xB0Wa\x07\x9Aa\x07\x946`\x04a\x05\xA5V[\x90a\x18\xE1V[a\x07\xA2a\x02RV[\x80a\x07\xAC\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x07\xC9\x90a\x07\xB5V[\x90V[a\x07\xD5\x81a\x07\xC0V[\x03a\x07\xDCWV[_\x80\xFD[\x90P5\x90a\x07\xED\x82a\x07\xCCV[V[\x91\x90`@\x83\x82\x03\x12a\x08\x17W\x80a\x08\x0Ba\x08\x14\x92_\x86\x01a\x07\xE0V[\x93` \x01a\x07\xE0V[\x90V[a\x02\\V[4a\x08KWa\x085a\x08/6`\x04a\x07\xEFV[\x90a\x1A\x92V[a\x08=a\x02RV[\x80a\x08G\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x90` \x82\x82\x03\x12a\x08iWa\x08f\x91_\x01a\x02\xC6V[\x90V[a\x02\\V[4a\x08\x9EWa\x08\x9Aa\x08\x89a\x08\x846`\x04a\x08PV[a\x1B!V[a\x08\x91a\x02RV[\x91\x82\x91\x82a\x03\xB7V[\x03\x90\xF3[a\x02XV[4a\x08\xD1Wa\x08\xB36`\x04a\x03SV[a\x08\xBBa\x1B\\V[a\x08\xC3a\x02RV[\x80a\x08\xCD\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08\xF1\x90`\x08a\x08\xF6\x93\x02a\x06|V[a\x08\xD6V[\x90V[\x90a\t\x04\x91Ta\x08\xE1V[\x90V[a\t\x13`\x01_\x90a\x08\xF9V[\x90V[a\t*a\t%a\t/\x92a\x07\xB5V[a\x06\xF3V[a\x07\xB5V[\x90V[a\t;\x90a\t\x16V[\x90V[a\tG\x90a\t2V[\x90V[a\tS\x90a\t>V[\x90RV[\x91\x90a\tj\x90_` \x85\x01\x94\x01\x90a\tJV[V[4a\t\x9CWa\t|6`\x04a\x03SV[a\t\x98a\t\x87a\t\x07V[a\t\x8Fa\x02RV[\x91\x82\x91\x82a\tWV[\x03\x90\xF3[a\x02XV[\x90V[a\t\xB4\x90`\x08a\t\xB9\x93\x02a\x06|V[a\t\xA1V[\x90V[\x90a\t\xC7\x91Ta\t\xA4V[\x90V[a\t\xD6`\x02_\x90a\t\xBCV[\x90V[4a\n\tWa\t\xE96`\x04a\x03SV[a\n\x05a\t\xF4a\t\xCAV[a\t\xFCa\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[\x90V[a\n%a\n a\n*\x92a\n\x0EV[a\x06\xF3V[a\x02\xAFV[\x90V[a\n9b'\x8D\0a\n\x11V[\x90V[a\nDa\n-V[\x90V[4a\nwWa\nW6`\x04a\x03SV[a\nsa\nba\n<V[a\nja\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[4a\n\xAAWa\n\x8C6`\x04a\x03SV[a\n\x94a\x1B\x8BV[a\n\x9Ca\x02RV[\x80a\n\xA6\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x91``\x83\x83\x03\x12a\n\xFCWa\n\xC6\x82_\x85\x01a\x07\xE0V[\x92a\n\xD4\x83` \x83\x01a\x07\xE0V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xF7Wa\n\xF3\x92\x01a\x02pV[\x90\x91V[a\x02`V[a\x02\\V[4a\x0B5Wa\x0B1a\x0B a\x0B\x176`\x04a\n\xAFV[\x92\x91\x90\x91a\x1CCV[a\x0B(a\x02RV[\x91\x82\x91\x82a\x05'V[\x03\x90\xF3[a\x02XV[4a\x0BjWa\x0BJ6`\x04a\x03SV[a\x0Bfa\x0BUa\x1C\xEBV[a\x0B]a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[4a\x0B\x9EWa\x0B\x88a\x0B\x826`\x04a\x05\xA5V[\x90a\x1E\tV[a\x0B\x90a\x02RV[\x80a\x0B\x9A\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[4a\x0B\xD3Wa\x0B\xB36`\x04a\x03SV[a\x0B\xCFa\x0B\xBEa\x1E\x15V[a\x0B\xC6a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[\x90\x91``\x82\x84\x03\x12a\x0C\rWa\x0C\na\x0B\xF3\x84_\x85\x01a\x02\xC6V[\x93a\x0C\x01\x81` \x86\x01a\x02\xC6V[\x93`@\x01a\x02\xC6V[\x90V[a\x02\\V[4a\x0CCWa\x0C?a\x0C.a\x0C(6`\x04a\x0B\xD8V[\x91a\x1E\xDAV[a\x0C6a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[a\x0CU`\x03`\x01\x90a\x06\x9EV[\x90V[4a\x0C\x88Wa\x0Ch6`\x04a\x03SV[a\x0C\x84a\x0Csa\x0CHV[a\x0C{a\x02RV[\x91\x82\x91\x82a\x05'V[\x03\x90\xF3[a\x02XV[4a\x0C\xBDWa\x0C\x9D6`\x04a\x03SV[a\x0C\xB9a\x0C\xA8a\x1FPV[a\x0C\xB0a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[a\x0C\xCB\x90a\x07\xC0V[\x90RV[\x91\x90a\x0C\xE2\x90_` \x85\x01\x94\x01\x90a\x0C\xC2V[V[4a\r\x14Wa\x0C\xF46`\x04a\x03SV[a\r\x10a\x0C\xFFa\x1F\x9FV[a\r\x07a\x02RV[\x91\x82\x91\x82a\x0C\xCFV[\x03\x90\xF3[a\x02XV[4a\rIWa\r)6`\x04a\x03SV[a\rEa\r4a\x1F\xD3V[a\r<a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[4a\r~Wa\r^6`\x04a\x03SV[a\rza\ria \x1FV[a\rqa\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[\x90` \x82\x82\x03\x12a\r\xB4W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r\xAFWa\r\xAB\x92\x01a\x04\x01V[\x90\x91V[a\x02`V[a\x02\\V[4a\r\xE8Wa\r\xD2a\r\xCC6`\x04a\r\x83V[\x90a![V[a\r\xDAa\x02RV[\x80a\r\xE4\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x90` \x82\x82\x03\x12a\x0E\x06Wa\x0E\x03\x91_\x01a\x07\xE0V[\x90V[a\x02\\V[4a\x0E9Wa\x0E#a\x0E\x1E6`\x04a\r\xEDV[a\"\x0BV[a\x0E+a\x02RV[\x80a\x0E5\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0E\x92Wa\x0Er6`\x04a\x03SV[a\x0E\x8Ea\x0E}a\x0E>V[a\x0E\x85a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[4a\x0E\xC5Wa\x0E\xA76`\x04a\x03SV[a\x0E\xAFa\"2V[a\x0E\xB7a\x02RV[\x80a\x0E\xC1\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[a\x0E\xDEa\x0E\xD9a\x0E\xE3\x92a\x02\xAFV[a\x06\xF3V[a\x02\xAFV[\x90V[\x90a\x0E\xF0\x90a\x0E\xCAV[_R` R`@_ \x90V[_\x1C\x90V[a\x0F\ra\x0F\x12\x91a\x0E\xFCV[a\t\xA1V[\x90V[a\x0F\x1F\x90Ta\x0F\x01V[\x90V[a\x0F-\x90`\x04a\x0E\xE6V[\x90a\x0F9_\x83\x01a\x0F\x15V[\x91a\x0FF`\x01\x82\x01a\x0F\x15V[\x91a\x0F_`\x03a\x0FX`\x02\x85\x01a\x0F\x15V[\x93\x01a\x0F\x15V[\x90V[a\x0F\x97a\x0F\x9E\x94a\x0F\x8D``\x94\x98\x97\x95a\x0F\x83`\x80\x86\x01\x9A_\x87\x01\x90a\x07*V[` \x85\x01\x90a\x07*V[`@\x83\x01\x90a\x07*V[\x01\x90a\x07*V[V[4a\x0F\xD4Wa\x0F\xD0a\x0F\xBBa\x0F\xB66`\x04a\x08PV[a\x0F\"V[\x90a\x0F\xC7\x94\x92\x94a\x02RV[\x94\x85\x94\x85a\x0FbV[\x03\x90\xF3[a\x02XV[\x90V[a\x0F\xF0a\x0F\xEBa\x0F\xF5\x92a\x0F\xD9V[a\x06\xF3V[a\x02\xAFV[\x90V[a\x10\x03a\x13\x88a\x0F\xDCV[\x90V[a\x10\x0Ea\x0F\xF8V[\x90V[4a\x10AWa\x10!6`\x04a\x03SV[a\x10=a\x10,a\x10\x06V[a\x104a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[4a\x10tWa\x10^a\x10Y6`\x04a\r\xEDV[a\"\xA1V[a\x10fa\x02RV[\x80a\x10p\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x91\x90`@\x83\x82\x03\x12a\x10\xA1W\x80a\x10\x95a\x10\x9E\x92_\x86\x01a\x02\xC6V[\x93` \x01a\x02\xC6V[\x90V[a\x02\\V[4a\x10\xD7Wa\x10\xD3a\x10\xC2a\x10\xBC6`\x04a\x10yV[\x90a#.V[a\x10\xCAa\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[a\x10\xE8`\x05_\x90a\t\xBCV[\x90V[4a\x11\x1BWa\x10\xFB6`\x04a\x03SV[a\x11\x17a\x11\x06a\x10\xDCV[a\x11\x0Ea\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[_\x80\xFD[\x91\x90a\x11Aa\x11;32\x90\x86\x85\x91\x92\x90\x91\x92a\x1CCV[\x15a\x05\x15V[a\x11PWa\x11N\x92a\x11\xADV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x11h`\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[a\x11u\x90a\t2V[\x90V[`@\x90a\x11\xA4a\x11\x99a\x11\xAB\x95\x97\x96\x94``\x84\x01\x90\x84\x82\x03_\x86\x01Ra\x05\xFDV[\x96` \x83\x01\x90a\x07*V[\x01\x90a\x07*V[V[\x90a\x11\xB9\x903\x92a\x17\x1FV[\x91B\x92a\x11\xFBa\x11\xE9\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x11lV[\x94a\x11\xF2a\x02RV[\x93\x84\x93\x84a\x11xV[\x03\x90\xA2V[\x90a\x12\x0B\x92\x91a\x11$V[V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x12+\x90a\x05\xF3V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x12EW`@RV[a\x12\rV[\x90a\x12]a\x12Va\x02RV[\x92\x83a\x12!V[V[a\x12i`\x80a\x12JV[\x90V[_\x90V[a\x12xa\x12_V[\x90` \x80\x80\x80\x85a\x12\x87a\x12lV[\x81R\x01a\x12\x92a\x12lV[\x81R\x01a\x12\x9Da\x12lV[\x81R\x01a\x12\xA8a\x12lV[\x81RPPV[a\x12\xB6a\x12pV[\x90V[a\x12\xC1a\x12\xAEV[Pa\x12\xCAa#\xA3V[\x90V[P\x90V[P\x90V[` \x91\x81R\x01\x90V[` \x7Fve the same length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x7FData and priority arrays must ha_\x82\x01R\x01RV[a\x138`2`@\x92a\x12\xD5V[a\x13A\x81a\x12\xDEV[\x01\x90V[a\x13Z\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x13+V[\x90V[\x15a\x13dWV[a\x13la\x02RV[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x13\x82`\x04\x82\x01a\x13EV[\x03\x90\xFD[\x90V[a\x13\x9Da\x13\x98a\x13\xA2\x92a\x13\x86V[a\x06\xF3V[a\x02\xAFV[\x90V[`\x01a\x13\xB1\x91\x01a\x02\xAFV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x14\x16W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x14\x11W` \x01\x91`\x01\x82\x026\x03\x83\x13a\x14\x0CWV[a\x13\xD0V[a\x13\xCCV[a\x13\xC8V[\x90\x82\x10\x15a\x146W` a\x142\x92\x02\x81\x01\x90a\x13\xD4V[\x90\x91V[a\x13\xB4V[\x91\x90\x81\x10\x15a\x14KW` \x02\x01\x90V[a\x13\xB4V[5a\x14Z\x81a\x02\xB2V[\x90V[\x90\x92a\x14j\x82\x85\x90a\x12\xCDV[\x93a\x14\x91\x85a\x14\x8Ba\x14\x85a\x14\x80\x88\x87\x90a\x12\xD1V[a\x02\xAFV[\x91a\x02\xAFV[\x14a\x13]V[a\x14\x9A_a\x13\x89V[[\x80a\x14\xAEa\x14\xA8\x88a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a\x15UWa\x14\xDC\x90a\x14\xD232\x90a\x14\xCA\x88\x87\x86\x91a\x14\x1BV[\x92\x90\x91a\x1CCV[a\x14\xE1W[a\x13\xA5V[a\x14\x9BV[3a\x14\xF7a\x14\xF1\x87\x86\x85\x91a\x14\x1BV[\x90a\x17\x1FV[\x90a\x15\x0Ca\x15\x07\x89\x88\x86\x91a\x14;V[a\x14PV[B\x92a\x15Ma\x15;\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x11lV[\x94a\x15Da\x02RV[\x93\x84\x93\x84a\x11xV[\x03\x90\xA2a\x14\xD7V[PPPPPPV[_\x90V[a\x15ma\x15r\x91a\x0E\xFCV[a\x06\x80V[\x90V[a\x15\x7F\x90Ta\x15aV[\x90V[a\x15\x8Aa\x15]V[Pa\x15\x95`\x03a\x15uV[\x90V[\x91\x90a\x15\xB5a\x15\xAF32\x90\x86\x85\x91\x92\x90\x91\x92a\x1CCV[\x15a\x05\x15V[a\x15\xC4Wa\x15\xC2\x92a\x16CV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x15\xDC`\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[\x90\x82_\x93\x92\x827\x01RV[\x91\x90a\x16\x05\x81a\x15\xFE\x81a\x16\n\x95a\x05\xDFV[\x80\x95a\x15\xE0V[a\x05\xF3V[\x01\x90V[a\x16:a\x16/`@\x93a\x16A\x96\x98\x97\x95``\x85\x01\x91\x85\x83\x03_\x87\x01Ra\x15\xEBV[\x96` \x83\x01\x90a\x07*V[\x01\x90a\x07*V[V[\x90\x913\x91\x92\x90\x92a\x16\x8ABa\x16x\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x95a\x11lV[\x95a\x16\x81a\x02RV[\x94\x85\x94\x85a\x16\x0EV[\x03\x90\xA2V[\x90a\x16\x9A\x92\x91a\x15\x98V[V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x16\xC4a\x16\xBFa\x16\xC9\x92a\x13\x86V[a\x16\xAAV[a\x16\xA1V[\x90V[\x90V[a\x16\xDBa\x16\xE0\x91a\x16\xA1V[a\x16\xCCV[\x90RV[\x90P\x90V[\x90\x91\x82a\x16\xF9\x81a\x17\0\x93a\x16\xE4V[\x80\x93a\x15\xE0V[\x01\x90V[\x80a\x17\x15`\x01\x92a\x17\x1C\x96\x94a\x16\xCFV[\x01\x91a\x16\xE9V[\x90V[a\x17]\x90a\x17+a\x16\x9CV[Pa\x17Na\x178_a\x16\xB0V[\x91\x93a\x17Ba\x02RV[\x94\x85\x93` \x85\x01a\x17\x04V[` \x82\x01\x81\x03\x82R\x03\x82a\x12!V[\x90V[\x90a\x17|a\x17v32\x90\x85\x85\x91\x92\x90\x91\x92a\x1CCV[\x15a\x05\x15V[a\x17\x8BWa\x17\x89\x91a\x18,V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x17\xA3`\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[`\x08\x1C\x90V[a\x17\xB9a\x17\xBE\x91a\x17\xA7V[a\x06\x80V[\x90V[a\x17\xCB\x90Ta\x17\xADV[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x17\xF1a\x17\xF7\x91\x93\x92\x93a\x02\xAFV[\x92a\x02\xAFV[\x82\x03\x91\x82\x11a\x18\x02WV[a\x17\xCEV[a\x18\x16a\x18\x1C\x91\x93\x92\x93a\x02\xAFV[\x92a\x02\xAFV[\x82\x01\x80\x92\x11a\x18'WV[a\x17\xCEV[\x90a\x18@a\x18:`\x03a\x17\xC1V[\x15a\x05\x15V[a\x18uWa\x18`a\x18s\x92a\x18Ya\x18n\x93Z\x92a\x18\x9AV[Z\x90a\x17\xE2V[a\x18ha\x0F\xF8V[\x90a\x18\x07V[a%wV[V[a\x18~\x91a\x18\x9AV[V[\x90\x91a\x18\x97\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x15\xEBV[\x90V[3\x90\x91a\x18\xC7\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11lV[\x92a\x18\xDCa\x18\xD3a\x02RV[\x92\x83\x92\x83a\x18\x80V[\x03\x90\xA2V[\x90a\x18\xEB\x91a\x17`V[V[\x90a\x18\xFF\x91a\x18\xFAa&~V[a\x1A\x05V[V[`\xA0\x1C\x90V[a\x19\x13a\x19\x18\x91a\x19\x01V[a\x06\x80V[\x90V[a\x19%\x90Ta\x19\x07V[\x90V[a\x19<a\x197a\x19A\x92a\x13\x86V[a\x06\xF3V[a\x07\xB5V[\x90V[a\x19M\x90a\x19(V[\x90V[`\xA0\x1B\x90V[\x90a\x19e`\xFF`\xA0\x1B\x91a\x19PV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x19x\x90a\x05\x15V[\x90V[\x90V[\x90a\x19\x93a\x19\x8Ea\x19\x9A\x92a\x19oV[a\x19{V[\x82Ta\x19VV[\x90UV[a\x19\xA7\x90a\t\x16V[\x90V[a\x19\xB3\x90a\x19\x9EV[\x90V[_\x1B\x90V[\x90a\x19\xCC`\x01\x80`\xA0\x1B\x03\x91a\x19\xB6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x19\xDF\x90a\x19\x9EV[\x90V[\x90V[\x90a\x19\xFAa\x19\xF5a\x1A\x01\x92a\x19\xD6V[a\x19\xE2V[\x82Ta\x19\xBBV[\x90UV[a\x1A\x0F`\x01a\x19\x1BV[a\x1AwW\x81a\x1A.a\x1A(a\x1A#_a\x19DV[a\x07\xC0V[\x91a\x07\xC0V[\x14a\x1A[Wa\x1ATa\x1AMa\x1AY\x93a\x1AH`\x01\x80a\x19~V[a\x19\xAAV[`\x01a\x19\xE5V[a\"\xA1V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x1As`\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x1A\x8E`\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[\x90a\x1A\x9C\x91a\x18\xEDV[V[\x90a\x1A\xA8\x90a\x02\xAFV[\x90RV[\x90a\x1B\x13a\x1B\n`\x03a\x1A\xBDa\x12_V[\x94a\x1A\xD4a\x1A\xCC_\x83\x01a\x0F\x15V[_\x88\x01a\x1A\x9EV[a\x1A\xECa\x1A\xE3`\x01\x83\x01a\x0F\x15V[` \x88\x01a\x1A\x9EV[a\x1B\x04a\x1A\xFB`\x02\x83\x01a\x0F\x15V[`@\x88\x01a\x1A\x9EV[\x01a\x0F\x15V[``\x84\x01a\x1A\x9EV[V[a\x1B\x1E\x90a\x1A\xACV[\x90V[a\x1B8a\x1B=\x91a\x1B0a\x12\xAEV[P`\x04a\x0E\xE6V[a\x1B\x15V[\x90V[a\x1BHa&~V[a\x1BPa\x1BRV[V[a\x1BZa'\tV[V[a\x1Bda\x1B@V[V[a\x1Bna&~V[a\x1Bva\x1BxV[V[a\x1B\x89a\x1B\x84_a\x19DV[a'9V[V[a\x1B\x93a\x1BfV[V[a\x1B\xA1a\x1B\xA6\x91a\x0E\xFCV[a\x08\xD6V[\x90V[a\x1B\xB3\x90Ta\x1B\x95V[\x90V[`\xE0\x1B\x90V[a\x1B\xC5\x81a\x05\x15V[\x03a\x1B\xCCWV[_\x80\xFD[\x90PQ\x90a\x1B\xDD\x82a\x1B\xBCV[V[\x90` \x82\x82\x03\x12a\x1B\xF8Wa\x1B\xF5\x91_\x01a\x1B\xD0V[\x90V[a\x02\\V[a\x1C#a\x1C0\x95\x93\x94\x92\x94a\x1C\x19``\x84\x01\x96_\x85\x01\x90a\x0C\xC2V[` \x83\x01\x90a\x0C\xC2V[`@\x81\x85\x03\x91\x01Ra\x15\xEBV[\x90V[a\x1C;a\x02RV[=_\x82>=\x90\xFD[\x92a\x1C\x86` \x93\x94a\x1CSa\x15]V[Pa\x1C\x91a\x1Cia\x1Cd`\x01a\x1B\xA9V[a\t>V[\x93cz9y\xDC\x92\x95\x97a\x1Cza\x02RV[\x98\x89\x97\x88\x96\x87\x96a\x1B\xB6V[\x86R`\x04\x86\x01a\x1B\xFDV[\x03\x91Z\xFA\x90\x81\x15a\x1C\xD5W_\x91a\x1C\xA7W[P\x90V[a\x1C\xC8\x91P` =\x81\x11a\x1C\xCEW[a\x1C\xC0\x81\x83a\x12!V[\x81\x01\x90a\x1B\xDFV[_a\x1C\xA3V[P=a\x1C\xB6V[a\x1C3V[_\x90V[a\x1C\xE8\x90Qa\x02\xAFV[\x90V[a\x1C\xF3a\x1C\xDAV[Pa\x1D\x1Aa\x1D\x01`\x05a\x0F\x15V[a\x1D\x14``a\x1D\x0Ea#\xA3V[\x01a\x1C\xDEV[\x90a\x18\x07V[\x90V[\x90a\x1D9a\x1D332\x90\x85\x85\x91\x92\x90\x91\x92a\x1CCV[\x15a\x05\x15V[a\x1DHWa\x1DF\x91a\x1DdV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x1D``\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[\x90a\x1Dxa\x1Dr`\x03a\x17\xC1V[\x15a\x05\x15V[a\x1D\xADWa\x1D\x98a\x1D\xAB\x92a\x1D\x91a\x1D\xA6\x93Z\x92a\x1D\xB8V[Z\x90a\x17\xE2V[a\x1D\xA0a\x0F\xF8V[\x90a\x18\x07V[a%wV[V[a\x1D\xB6\x91a\x1D\xB8V[V[\x90a\x1D\xC4\x903\x92a\x17\x1FV[\x90a\x1E\x04a\x1D\xF2\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11lV[\x92a\x1D\xFBa\x02RV[\x91\x82\x91\x82a\x06.V[\x03\x90\xA2V[\x90a\x1E\x13\x91a\x1D\x1DV[V[a\x1E\x1Da\x1C\xDAV[Pa\x1E&a#\xA3V[a\x1E1_\x82\x01a\x1C\xDEV[a\x1ECa\x1E=_a\x13\x89V[\x91a\x02\xAFV[\x14a\x1E\x98Wa\x1EV_a\x1Ed\x92\x01a\x1C\xDEV[a\x1E^a\n-V[\x90a\x18\x07V[Ba\x1Ewa\x1Eq\x83a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a\x1E\x8BWa\x1E\x88\x90B\x90a\x17\xE2V[\x90V[Pa\x1E\x95_a\x13\x89V[\x90V[Pa\x1E\xA2_a\x13\x89V[\x90V[a\x1E\xB4a\x1E\xBA\x91\x93\x92\x93a\x02\xAFV[\x92a\x02\xAFV[\x91a\x1E\xC6\x83\x82\x02a\x02\xAFV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1E\xD5WV[a\x17\xCEV[\x91a\x1E\xE3a\x1C\xDAV[P\x80a\x1E\xF7a\x1E\xF1\x84a\x02\xAFV[\x91a\x02\xAFV[\x11\x15a\x1FKWa\x1F\x18\x91a\x1F\n\x91a\x17\xE2V[a\x1F\x12a\x07\x12V[\x90a\x1E\xA5V[\x80a\x1F+a\x1F%\x84a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a\x1F=Wa\x1F:\x91a\x17\xE2V[\x90V[PPa\x1FH_a\x13\x89V[\x90V[PP\x90V[a\x1FXa\x1C\xDAV[Pa\x1Fl``a\x1Ffa#\xA3V[\x01a\x1C\xDEV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1F\x8Aa\x1F\x8F\x91a\x0E\xFCV[a\x1FsV[\x90V[a\x1F\x9C\x90Ta\x1F~V[\x90V[a\x1F\xA7a\x1FoV[Pa\x1F\xB1_a\x1F\x92V[\x90V[\x90V[a\x1F\xCBa\x1F\xC6a\x1F\xD0\x92a\x1F\xB4V[a\x06\xF3V[a\x02\xAFV[\x90V[a\x1F\xDBa\x1C\xDAV[Pa\x1F\xEFa\x1F\xE9`\x03a\x15uV[\x15a\x05\x15V[a \x13Wa \x10a \0`\x02a\x0F\x15V[a \n`\x01a\x1F\xB7V[\x90a\x18\x07V[\x90V[a \x1C_a\x13\x89V[\x90V[a 'a\x1C\xDAV[Pa ;`@a 5a#\xA3V[\x01a\x1C\xDEV[\x90V[\x90a Ra L`\x03a\x17\xC1V[\x15a\x05\x15V[a \x87Wa ra \x85\x92a ka \x80\x93Z\x92a \x92V[Z\x90a\x17\xE2V[a za\x0F\xF8V[\x90a\x18\x07V[a%wV[V[a \x90\x91a \x92V[V[a \x9D\x81\x83\x90a\x12\xCDV[\x91a \xA6a\x1C\xDAV[Pa \xB0_a\x13\x89V[[\x80a \xC4a \xBE\x86a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a!UWa \xF2\x90a \xE832\x90a \xE0\x87\x87\x86\x91a\x14\x1BV[\x92\x90\x91a\x1CCV[a \xF7W[a\x13\xA5V[a \xB1V[3a!\ra!\x07\x86\x86\x85\x91a\x14\x1BV[\x90a\x17\x1FV[\x90a!Ma!;\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11lV[\x92a!Da\x02RV[\x91\x82\x91\x82a\x06.V[\x03\x90\xA2a \xEDV[PPPPV[\x90a!e\x91a >V[V[a!x\x90a!sa&~V[a!zV[V[\x80a!\x95a!\x8Fa!\x8A_a\x19DV[a\x07\xC0V[\x91a\x07\xC0V[\x14a!\xEFWa!\xADa!\xA6\x82a\x19\xAAV[`\x01a\x19\xE5V[a!\xD7\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x11lV[\x90a!\xE0a\x02RV[\x80a!\xEA\x81a\x03\x1AV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\"\x07`\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[a\"\x14\x90a!gV[V[a\"\x1Ea&~V[a\"&a\"(V[V[a\"0a'\x98V[V[a\":a\"\x16V[V[a\"M\x90a\"Ha&~V[a\"OV[V[\x80a\"ja\"da\"__a\x19DV[a\x07\xC0V[\x91a\x07\xC0V[\x14a\"zWa\"x\x90a'9V[V[a\"\x9Da\"\x86_a\x19DV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x0C\xCFV[\x03\x90\xFD[a\"\xAA\x90a\"<V[V[_\x7FGasCounter: invalid range\0\0\0\0\0\0\0\x91\x01RV[a\"\xE0`\x19` \x92a\x12\xD5V[a\"\xE9\x81a\"\xACV[\x01\x90V[a#\x02\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\"\xD3V[\x90V[\x15a#\x0CWV[a#\x14a\x02RV[bF\x1B\xCD`\xE5\x1B\x81R\x80a#*`\x04\x82\x01a\"\xEDV[\x03\x90\xFD[a#]\x91a#:a\x1C\xDAV[Pa#X\x81a#Qa#K\x85a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a#\x05V[a\x17\xE2V[\x90V[a#j`\x80a\x12JV[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a#\x8Da#\x93\x91a\x02\xAFV[\x91a\x02\xAFV[\x90\x81\x15a#\x9EW\x04\x90V[a#mV[a#\xABa\x12\xAEV[Pa#\xBFa#\xB9`\x03a\x15uV[\x15a\x05\x15V[a$\xBBWa#\xE0a#\xDB`\x04a#\xD5`\x02a\x0F\x15V[\x90a\x0E\xE6V[a\x1B\x15V[Ba$\x0Ea$\x08a$\x03a#\xF5_\x86\x01a\x1C\xDEV[a#\xFDa\n-V[\x90a\x18\x07V[a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a$\x17W\x90V[a$d\x90a$^a$O_a$Ha$:Ba$4\x84\x88\x01a\x1C\xDEV[\x90a\x17\xE2V[a$Ba\n-V[\x90a#\x81V[\x93\x01a\x1C\xDEV[\x91a$Xa\n-V[\x90a\x1E\xA5V[\x90a\x18\x07V[a$\xB8a$\xAF_a$\xAAa$\xA1_a$\x9Ca$\x93_\x95a$\x8Ea$\x85a#`V[\x9A_\x8C\x01a\x1A\x9EV[a\x13\x89V[` \x89\x01a\x1A\x9EV[a\x13\x89V[`@\x86\x01a\x1A\x9EV[a\x13\x89V[``\x83\x01a\x1A\x9EV[\x90V[_a%\x18a%\x0F_a%\na%\x01_a$\xFCa$\xF3_\x95a$\xEEa$\xE6a$\xE0a#`V[\x9Ba\x13\x89V[_\x8C\x01a\x1A\x9EV[a\x13\x89V[` \x89\x01a\x1A\x9EV[a\x13\x89V[`@\x86\x01a\x1A\x9EV[a\x13\x89V[``\x83\x01a\x1A\x9EV[\x90V[\x90a%'_\x19\x91a\x19\xB6V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a%Ia%Da%P\x92a\x0E\xCAV[a%1V[\x82Ta%\x1BV[\x90UV[\x91` a%u\x92\x94\x93a%n`@\x82\x01\x96_\x83\x01\x90a\x07*V[\x01\x90a\x07*V[V[a%\x8Aa%\x84`\x03a\x17\xC1V[\x15a\x05\x15V[a&{Wa%\xA1a%\x9B`\x03a\x15uV[\x15a\x05\x15V[a&nW[a%\xAEa)nV[a&\x1Fa%\xBC\x82:\x90a\x1E\xA5V[a%\xEF\x83a%\xE9`\x02a%\xD9`\x04a%\xD3\x83a\x0F\x15V[\x90a\x0E\xE6V[\x01\x91a%\xE4\x83a\x0F\x15V[a\x18\x07V[\x90a%4V[a&\x19`\x03a&\t`\x04a&\x03`\x02a\x0F\x15V[\x90a\x0E\xE6V[\x01\x91a&\x14\x83a\x0F\x15V[a\x18\x07V[\x90a%4V[a&)`\x02a\x0F\x15V[:a&T\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0E\xCAV[\x92a&ia&`a\x02RV[\x92\x83\x92\x83a%TV[\x03\x90\xA2V[a&va(cV[a%\xA6V[PV[a&\x86a\x1F\x9FV[a&\x9Fa&\x99a&\x94a+nV[a\x07\xC0V[\x91a\x07\xC0V[\x03a&\xA6WV[a&\xC8a&\xB1a+nV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\x0C\xCFV[\x03\x90\xFD[`\x08\x1B\x90V[\x90a&\xDFa\xFF\0\x91a&\xCCV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a&\xFEa&\xF9a'\x05\x92a\x19oV[a\x19{V[\x82Ta&\xD2V[\x90UV[a'\x14_`\x03a&\xE9V[V[\x90V[\x90a'.a')a'5\x92a\x11lV[a'\x16V[\x82Ta\x19\xBBV[\x90UV[a'B_a\x1F\x92V[a'L\x82_a'\x19V[\x90a'\x80a'z\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x11lV[\x91a\x11lV[\x91a'\x89a\x02RV[\x80a'\x93\x81a\x03\x1AV[\x03\x90\xA3V[a'\xA4`\x01`\x03a&\xE9V[V[\x90a'\xB2`\xFF\x91a\x19\xB6V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a'\xD1a'\xCCa'\xD8\x92a\x19oV[a\x19{V[\x82Ta'\xA6V[\x90UV[\x90a'\xE6\x90a\x13\x89V[_R` R`@_ \x90V[\x90a(O```\x03a(U\x94a(\x15_\x82\x01a(\x0F_\x88\x01a\x1C\xDEV[\x90a%4V[a(.`\x01\x82\x01a((` \x88\x01a\x1C\xDEV[\x90a%4V[a(G`\x02\x82\x01a(A`@\x88\x01a\x1C\xDEV[\x90a%4V[\x01\x92\x01a\x1C\xDEV[\x90a%4V[V[\x90a(a\x91a'\xF2V[V[a(va(p`\x03a\x15uV[\x15a\x05\x15V[a(}W[V[a(\x89`\x01`\x03a'\xBCV[a(\x9Ca(\x95_a\x13\x89V[`\x02a%4V[a)\x05Ba(\xF4a(\xEB_a(\xE6a(\xDD_a(\xD8a(\xCF_\x95a(\xCAa(\xC1a#`V[\x9A_\x8C\x01a\x1A\x9EV[a\x13\x89V[` \x89\x01a\x1A\x9EV[a\x13\x89V[`@\x86\x01a\x1A\x9EV[a\x13\x89V[``\x83\x01a\x1A\x9EV[a)\0`\x04_\x90a'\xDCV[a(WV[_B\x90a)Ga)5\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x13\x89V[\x92a)>a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xA2a({V[\x90V[a)[\x90a\x02\xAFV[_\x19\x81\x14a)iW`\x01\x01\x90V[a\x17\xCEV[a)\x8Ba)\x86`\x04a)\x80`\x02a\x0F\x15V[\x90a\x0E\xE6V[a)OV[Ba)\xB9a)\xB3a)\xAEa)\xA0_\x86\x01a\x0F\x15V[a)\xA8a\n-V[\x90a\x18\x07V[a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a)\xC3W[PV[a)\xEBa)\xE2a)\xD4_\x84\x01a\x0F\x15V[a)\xDCa\n-V[\x90a\x18\x07V[`\x01\x83\x01a%4V[a*\x13a*\x0Ca)\xFD`\x03\x84\x01a\x0F\x15V[a*\x07`\x05a\x0F\x15V[a\x18\x07V[`\x05a%4V[a*\x1D`\x02a\x0F\x15V[a*Ja*,`\x02\x84\x01a\x0F\x15V[\x92a*D_a*=`\x01\x84\x01a\x0F\x15V[\x92\x01a\x0F\x15V[\x90a\x17\xE2V[a*t\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0E\xCAV[\x92a*\x89a*\x80a\x02RV[\x92\x83\x92\x83a%TV[\x03\x90\xA2a*\xA8a*\xA1a*\x9C`\x02a\x0F\x15V[a)RV[`\x02a%4V[a+\x1ABa+\0a*\xF7_a*\xF2a*\xE9_a*\xE4a*\xDB_\x95a*\xD6a*\xCDa#`V[\x9A_\x8C\x01a\x1A\x9EV[a\x13\x89V[` \x89\x01a\x1A\x9EV[a\x13\x89V[`@\x86\x01a\x1A\x9EV[a\x13\x89V[``\x83\x01a\x1A\x9EV[a+\x15`\x04a+\x0F`\x02a\x0F\x15V[\x90a\x0E\xE6V[a(WV[a+$`\x02a\x0F\x15V[B\x90a+ea+S\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0E\xCAV[\x92a+\\a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xA2_a)\xC0V[a+va\x1FoV[P3\x90V`\x80`@R4`\x1CW`\x0E` V[a\x01\x94a\0+\x829a\x01\x94\x90\xF3[`&V[`@Q\x90V[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x01{V[a\0\x1D_5a\0,V[cz9y\xDC\x03a\0\x0EWa\x01BV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\0X\x90a\0DV[\x90V[a\0d\x81a\0OV[\x03a\0kWV[_\x80\xFD[\x90P5\x90a\0|\x82a\0[V[V[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\0\xC4W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xBFW` \x01\x92`\x01\x83\x02\x84\x01\x11a\0\xBAWV[a\0\x86V[a\0\x82V[a\0~V[\x91``\x83\x83\x03\x12a\x01\x16Wa\0\xE0\x82_\x85\x01a\0oV[\x92a\0\xEE\x83` \x83\x01a\0oV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x11Wa\x01\r\x92\x01a\0\x8AV[\x90\x91V[a\0@V[a\0<V[\x15\x15\x90V[a\x01)\x90a\x01\x1BV[\x90RV[\x91\x90a\x01@\x90_` \x85\x01\x94\x01\x90a\x01 V[V[4a\x01vWa\x01ra\x01aa\x01X6`\x04a\0\xC9V[\x92\x91\x90\x91a\x01\x83V[a\x01ia\x002V[\x91\x82\x91\x82a\x01-V[\x03\x90\xF3[a\08V[_\x80\xFD[_\x90V[PPPPa\x01\x8Fa\x01\x7FV[P_\x90V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b611120565b61001d5f3561024c565b8063050ec13814610247578063086146d21461024257806311992f8c1461023d57806318d5aafe146102385780631c0b636714610233578063366cbab71461022e5780633b6ab2a9146102295780633d44ae8b1461022457806346e2cc091461021f578063485cc9551461021a5780634b2c0706146102155780635467cb48146102105780635b3cd6e21461020b57806361543801146102065780636558954f14610201578063715018a6146101fc5780637a3979dc146101f75780637fbd295e146101f2578063804e5123146101ed57806382f44ade146101e857806383d3c115146101e357806384fab62b146101de5780638d5a239b146101d95780638da5cb5b146101d4578063aff74c6d146101cf578063c660d3f3146101ca578063cdafb978146101c5578063d4f0eb4d146101c0578063d8781342146101bb578063de1f453e146101b6578063ea4a1104146101b1578063ede07bd6146101ac578063f2fde38b146101a7578063f7b8935e146101a25763ff7b30840361000e576110eb565b6110a6565b611046565b611011565b610fa0565b610e97565b610e62565b610e0b565b610db9565b610d4e565b610d19565b610ce4565b610c8d565b610c58565b610c12565b610ba3565b610b6f565b610b3a565b610b01565b610a7c565b610a47565b6109d9565b61096c565b6108a3565b61086e565b61081c565b610781565b61074c565b6106bb565b610646565b610571565b61053c565b6104de565b6103cc565b61031f565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156102aa5781359167ffffffffffffffff83116102a55760200192600183028401116102a057565b61026c565b610268565b610264565b90565b6102bb816102af565b036102c257565b5f80fd5b905035906102d3826102b2565b565b91604083830312610315575f83013567ffffffffffffffff8111610310576103028361030d928601610270565b9390946020016102c6565b90565b610260565b61025c565b5f0190565b3461034e576103386103323660046102d5565b91611200565b610340610252565b8061034a8161031a565b0390f35b610258565b5f91031261035d57565b61025c565b61036b906102af565b9052565b906060806103b5936103875f8201515f860190610362565b61039960208201516020860190610362565b6103ab60408201516040860190610362565b0151910190610362565b565b91906103ca905f6080850194019061036f565b565b346103fc576103dc366004610353565b6103f86103e76112b9565b6103ef610252565b918291826103b7565b0390f35b610258565b909182601f8301121561043b5781359167ffffffffffffffff831161043657602001926020830284011161043157565b61026c565b610268565b610264565b909182601f8301121561047a5781359167ffffffffffffffff831161047557602001926020830284011161047057565b61026c565b610268565b610264565b90916040828403126104d9575f82013567ffffffffffffffff81116104d457836104aa918401610401565b929093602082013567ffffffffffffffff81116104cf576104cb9201610440565b9091565b610260565b610260565b61025c565b34610510576104fa6104f136600461047f565b9291909161145d565b610502610252565b8061050c8161031a565b0390f35b610258565b151590565b61052390610515565b9052565b919061053a905f6020850194019061051a565b565b3461056c5761054c366004610353565b610568610557611582565b61055f610252565b91829182610527565b0390f35b610258565b346105a05761058a6105843660046102d5565b9161168f565b610592610252565b8061059c8161031a565b0390f35b610258565b906020828203126105d6575f82013567ffffffffffffffff81116105d1576105cd9201610270565b9091565b610260565b61025c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61061c61062560209361062a93610613816105db565b938480936105df565b958691016105e8565b6105f3565b0190565b6106439160208201915f8184039101526105fd565b90565b346106775761067361066261065c3660046105a5565b9061171f565b61066a610252565b9182918261062e565b0390f35b610258565b1c90565b60ff1690565b61069690600861069b930261067c565b610680565b90565b906106a99154610686565b90565b6106b860035f9061069e565b90565b346106eb576106cb366004610353565b6106e76106d66106ac565b6106de610252565b91829182610527565b0390f35b610258565b90565b90565b61070a61070561070f926106f0565b6106f3565b6102af565b90565b61071c600a6106f6565b90565b610727610712565b90565b610733906102af565b9052565b919061074a905f6020850194019061072a565b565b3461077c5761075c366004610353565b61077861076761071f565b61076f610252565b91829182610737565b0390f35b610258565b346107b05761079a6107943660046105a5565b906118e1565b6107a2610252565b806107ac8161031a565b0390f35b610258565b60018060a01b031690565b6107c9906107b5565b90565b6107d5816107c0565b036107dc57565b5f80fd5b905035906107ed826107cc565b565b9190604083820312610817578061080b610814925f86016107e0565b936020016107e0565b90565b61025c565b3461084b5761083561082f3660046107ef565b90611a92565b61083d610252565b806108478161031a565b0390f35b610258565b9060208282031261086957610866915f016102c6565b90565b61025c565b3461089e5761089a610889610884366004610850565b611b21565b610891610252565b918291826103b7565b0390f35b610258565b346108d1576108b3366004610353565b6108bb611b5c565b6108c3610252565b806108cd8161031a565b0390f35b610258565b60018060a01b031690565b6108f19060086108f6930261067c565b6108d6565b90565b9061090491546108e1565b90565b61091360015f906108f9565b90565b61092a61092561092f926107b5565b6106f3565b6107b5565b90565b61093b90610916565b90565b61094790610932565b90565b6109539061093e565b9052565b919061096a905f6020850194019061094a565b565b3461099c5761097c366004610353565b610998610987610907565b61098f610252565b91829182610957565b0390f35b610258565b90565b6109b49060086109b9930261067c565b6109a1565b90565b906109c791546109a4565b90565b6109d660025f906109bc565b90565b34610a09576109e9366004610353565b610a056109f46109ca565b6109fc610252565b91829182610737565b0390f35b610258565b90565b610a25610a20610a2a92610a0e565b6106f3565b6102af565b90565b610a3962278d00610a11565b90565b610a44610a2d565b90565b34610a7757610a57366004610353565b610a73610a62610a3c565b610a6a610252565b91829182610737565b0390f35b610258565b34610aaa57610a8c366004610353565b610a94611b8b565b610a9c610252565b80610aa68161031a565b0390f35b610258565b91606083830312610afc57610ac6825f85016107e0565b92610ad483602083016107e0565b92604082013567ffffffffffffffff8111610af757610af39201610270565b9091565b610260565b61025c565b34610b3557610b31610b20610b17366004610aaf565b92919091611c43565b610b28610252565b91829182610527565b0390f35b610258565b34610b6a57610b4a366004610353565b610b66610b55611ceb565b610b5d610252565b91829182610737565b0390f35b610258565b34610b9e57610b88610b823660046105a5565b90611e09565b610b90610252565b80610b9a8161031a565b0390f35b610258565b34610bd357610bb3366004610353565b610bcf610bbe611e15565b610bc6610252565b91829182610737565b0390f35b610258565b9091606082840312610c0d57610c0a610bf3845f85016102c6565b93610c0181602086016102c6565b936040016102c6565b90565b61025c565b34610c4357610c3f610c2e610c28366004610bd8565b91611eda565b610c36610252565b91829182610737565b0390f35b610258565b610c55600360019061069e565b90565b34610c8857610c68366004610353565b610c84610c73610c48565b610c7b610252565b91829182610527565b0390f35b610258565b34610cbd57610c9d366004610353565b610cb9610ca8611f50565b610cb0610252565b91829182610737565b0390f35b610258565b610ccb906107c0565b9052565b9190610ce2905f60208501940190610cc2565b565b34610d1457610cf4366004610353565b610d10610cff611f9f565b610d07610252565b91829182610ccf565b0390f35b610258565b34610d4957610d29366004610353565b610d45610d34611fd3565b610d3c610252565b91829182610737565b0390f35b610258565b34610d7e57610d5e366004610353565b610d7a610d6961201f565b610d71610252565b91829182610737565b0390f35b610258565b90602082820312610db4575f82013567ffffffffffffffff8111610daf57610dab9201610401565b9091565b610260565b61025c565b34610de857610dd2610dcc366004610d83565b9061215b565b610dda610252565b80610de48161031a565b0390f35b610258565b90602082820312610e0657610e03915f016107e0565b90565b61025c565b34610e3957610e23610e1e366004610ded565b61220b565b610e2b610252565b80610e358161031a565b0390f35b610258565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610e9257610e72366004610353565b610e8e610e7d610e3e565b610e85610252565b91829182610737565b0390f35b610258565b34610ec557610ea7366004610353565b610eaf612232565b610eb7610252565b80610ec18161031a565b0390f35b610258565b610ede610ed9610ee3926102af565b6106f3565b6102af565b90565b90610ef090610eca565b5f5260205260405f2090565b5f1c90565b610f0d610f1291610efc565b6109a1565b90565b610f1f9054610f01565b90565b610f2d906004610ee6565b90610f395f8301610f15565b91610f4660018201610f15565b91610f5f6003610f5860028501610f15565b9301610f15565b90565b610f97610f9e94610f8d606094989795610f83608086019a5f87019061072a565b602085019061072a565b604083019061072a565b019061072a565b565b34610fd457610fd0610fbb610fb6366004610850565b610f22565b90610fc7949294610252565b94859485610f62565b0390f35b610258565b90565b610ff0610feb610ff592610fd9565b6106f3565b6102af565b90565b611003611388610fdc565b90565b61100e610ff8565b90565b3461104157611021366004610353565b61103d61102c611006565b611034610252565b91829182610737565b0390f35b610258565b346110745761105e611059366004610ded565b6122a1565b611066610252565b806110708161031a565b0390f35b610258565b91906040838203126110a1578061109561109e925f86016102c6565b936020016102c6565b90565b61025c565b346110d7576110d36110c26110bc366004611079565b9061232e565b6110ca610252565b91829182610737565b0390f35b610258565b6110e860055f906109bc565b90565b3461111b576110fb366004610353565b6111176111066110dc565b61110e610252565b91829182610737565b0390f35b610258565b5f80fd5b919061114161113b33329086859192909192611c43565b15610515565b6111505761114e926111ad565b565b5f631b8e828b60e31b8152806111686004820161031a565b0390fd5b61117590610932565b90565b6040906111a46111996111ab9597969460608401908482035f8601526105fd565b96602083019061072a565b019061072a565b565b906111b990339261171f565b9142926111fb6111e97f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f29461116c565b946111f2610252565b93849384611178565b0390a2565b9061120b9291611124565b565b634e487b7160e01b5f52604160045260245ffd5b9061122b906105f3565b810190811067ffffffffffffffff82111761124557604052565b61120d565b9061125d611256610252565b9283611221565b565b611269608061124a565b90565b5f90565b61127861125f565b9060208080808561128761126c565b81520161129261126c565b81520161129d61126c565b8152016112a861126c565b81525050565b6112b6611270565b90565b6112c16112ae565b506112ca6123a3565b90565b5090565b5090565b60209181520190565b60207f7665207468652073616d65206c656e6774680000000000000000000000000000917f4461746120616e64207072696f7269747920617272617973206d7573742068615f8201520152565b61133860326040926112d5565b611341816112de565b0190565b61135a9060208101905f81830391015261132b565b90565b1561136457565b61136c610252565b62461bcd60e51b81528061138260048201611345565b0390fd5b90565b61139d6113986113a292611386565b6106f3565b6102af565b90565b60016113b191016102af565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b903590600160200381360303821215611416570180359067ffffffffffffffff82116114115760200191600182023603831361140c57565b6113d0565b6113cc565b6113c8565b9082101561143657602061143292028101906113d4565b9091565b6113b4565b919081101561144b576020020190565b6113b4565b3561145a816102b2565b90565b909261146a8285906112cd565b936114918561148b6114856114808887906112d1565b6102af565b916102af565b1461135d565b61149a5f611389565b5b806114ae6114a8886102af565b916102af565b1015611555576114dc906114d23332906114ca8887869161141b565b929091611c43565b6114e1575b6113a5565b61149b565b336114f76114f18786859161141b565b9061171f565b9061150c6115078988869161143b565b611450565b429261154d61153b7f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f29461116c565b94611544610252565b93849384611178565b0390a26114d7565b505050505050565b5f90565b61156d61157291610efc565b610680565b90565b61157f9054611561565b90565b61158a61155d565b506115956003611575565b90565b91906115b56115af33329086859192909192611c43565b15610515565b6115c4576115c292611643565b565b5f631b8e828b60e31b8152806115dc6004820161031a565b0390fd5b90825f939282370152565b9190611605816115fe8161160a956105df565b80956115e0565b6105f3565b0190565b61163a61162f6040936116419698979560608501918583035f8701526115eb565b96602083019061072a565b019061072a565b565b9091339192909261168a426116787f4b5aa8d082e691cb9972a7958fa4153f663f215fe697a3e08bd2729ed78f02f29561116c565b95611681610252565b9485948561160e565b0390a2565b9061169a9291611598565b565b606090565b60ff60f81b1690565b60f81b90565b6116c46116bf6116c992611386565b6116aa565b6116a1565b90565b90565b6116db6116e0916116a1565b6116cc565b9052565b905090565b9091826116f981611700936116e4565b80936115e0565b0190565b8061171560019261171c96946116cf565b01916116e9565b90565b61175d9061172b61169c565b5061174e6117385f6116b0565b9193611742610252565b94859360208501611704565b60208201810382520382611221565b90565b9061177c61177633329085859192909192611c43565b15610515565b61178b576117899161182c565b565b5f631b8e828b60e31b8152806117a36004820161031a565b0390fd5b60081c90565b6117b96117be916117a7565b610680565b90565b6117cb90546117ad565b90565b634e487b7160e01b5f52601160045260245ffd5b6117f16117f7919392936102af565b926102af565b820391821161180257565b6117ce565b61181661181c919392936102af565b926102af565b820180921161182757565b6117ce565b9061184061183a60036117c1565b15610515565b611875576118606118739261185961186e935a9261189a565b5a906117e2565b611868610ff8565b90611807565b612577565b565b61187e9161189a565b565b90916118979260208301925f8185039101526115eb565b90565b3390916118c77f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f9261116c565b926118dc6118d3610252565b92839283611880565b0390a2565b906118eb91611760565b565b906118ff916118fa61267e565b611a05565b565b60a01c90565b61191361191891611901565b610680565b90565b6119259054611907565b90565b61193c61193761194192611386565b6106f3565b6107b5565b90565b61194d90611928565b90565b60a01b90565b9061196560ff60a01b91611950565b9181191691161790565b61197890610515565b90565b90565b9061199361198e61199a9261196f565b61197b565b8254611956565b9055565b6119a790610916565b90565b6119b39061199e565b90565b5f1b90565b906119cc60018060a01b03916119b6565b9181191691161790565b6119df9061199e565b90565b90565b906119fa6119f5611a01926119d6565b6119e2565b82546119bb565b9055565b611a0f600161191b565b611a775781611a2e611a28611a235f611944565b6107c0565b916107c0565b14611a5b57611a54611a4d611a5993611a4860018061197e565b6119aa565b60016119e5565b6122a1565b565b5f632e7f3c7f60e11b815280611a736004820161031a565b0390fd5b5f62dc149f60e41b815280611a8e6004820161031a565b0390fd5b90611a9c916118ed565b565b90611aa8906102af565b9052565b90611b13611b0a6003611abd61125f565b94611ad4611acc5f8301610f15565b5f8801611a9e565b611aec611ae360018301610f15565b60208801611a9e565b611b04611afb60028301610f15565b60408801611a9e565b01610f15565b60608401611a9e565b565b611b1e90611aac565b90565b611b38611b3d91611b306112ae565b506004610ee6565b611b15565b90565b611b4861267e565b611b50611b52565b565b611b5a612709565b565b611b64611b40565b565b611b6e61267e565b611b76611b78565b565b611b89611b845f611944565b612739565b565b611b93611b66565b565b611ba1611ba691610efc565b6108d6565b90565b611bb39054611b95565b90565b60e01b90565b611bc581610515565b03611bcc57565b5f80fd5b90505190611bdd82611bbc565b565b90602082820312611bf857611bf5915f01611bd0565b90565b61025c565b611c23611c309593949294611c1960608401965f850190610cc2565b6020830190610cc2565b60408185039101526115eb565b90565b611c3b610252565b3d5f823e3d90fd5b92611c8660209394611c5361155d565b50611c91611c69611c646001611ba9565b61093e565b93637a3979dc929597611c7a610252565b98899788968796611bb6565b865260048601611bfd565b03915afa908115611cd5575f91611ca7575b5090565b611cc8915060203d8111611cce575b611cc08183611221565b810190611bdf565b5f611ca3565b503d611cb6565b611c33565b5f90565b611ce890516102af565b90565b611cf3611cda565b50611d1a611d016005610f15565b611d146060611d0e6123a3565b01611cde565b90611807565b90565b90611d39611d3333329085859192909192611c43565b15610515565b611d4857611d4691611d64565b565b5f631b8e828b60e31b815280611d606004820161031a565b0390fd5b90611d78611d7260036117c1565b15610515565b611dad57611d98611dab92611d91611da6935a92611db8565b5a906117e2565b611da0610ff8565b90611807565b612577565b565b611db691611db8565b565b90611dc490339261171f565b90611e04611df27f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f9261116c565b92611dfb610252565b9182918261062e565b0390a2565b90611e1391611d1d565b565b611e1d611cda565b50611e266123a3565b611e315f8201611cde565b611e43611e3d5f611389565b916102af565b14611e9857611e565f611e649201611cde565b611e5e610a2d565b90611807565b42611e77611e71836102af565b916102af565b1015611e8b57611e889042906117e2565b90565b50611e955f611389565b90565b50611ea25f611389565b90565b611eb4611eba919392936102af565b926102af565b91611ec68382026102af565b928184041490151715611ed557565b6117ce565b91611ee3611cda565b5080611ef7611ef1846102af565b916102af565b1115611f4b57611f1891611f0a916117e2565b611f12610712565b90611ea5565b80611f2b611f25846102af565b916102af565b1015611f3d57611f3a916117e2565b90565b5050611f485f611389565b90565b505090565b611f58611cda565b50611f6c6060611f666123a3565b01611cde565b90565b5f90565b60018060a01b031690565b611f8a611f8f91610efc565b611f73565b90565b611f9c9054611f7e565b90565b611fa7611f6f565b50611fb15f611f92565b90565b90565b611fcb611fc6611fd092611fb4565b6106f3565b6102af565b90565b611fdb611cda565b50611fef611fe96003611575565b15610515565b612013576120106120006002610f15565b61200a6001611fb7565b90611807565b90565b61201c5f611389565b90565b612027611cda565b5061203b60406120356123a3565b01611cde565b90565b9061205261204c60036117c1565b15610515565b612087576120726120859261206b612080935a92612092565b5a906117e2565b61207a610ff8565b90611807565b612577565b565b61209091612092565b565b61209d8183906112cd565b916120a6611cda565b506120b05f611389565b5b806120c46120be866102af565b916102af565b1015612155576120f2906120e83332906120e08787869161141b565b929091611c43565b6120f7575b6113a5565b6120b1565b3361210d6121078686859161141b565b9061171f565b9061214d61213b7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f9261116c565b92612144610252565b9182918261062e565b0390a26120ed565b50505050565b906121659161203e565b565b6121789061217361267e565b61217a565b565b8061219561218f61218a5f611944565b6107c0565b916107c0565b146121ef576121ad6121a6826119aa565b60016119e5565b6121d77f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b99161116c565b906121e0610252565b806121ea8161031a565b0390a2565b5f632e7f3c7f60e11b8152806122076004820161031a565b0390fd5b61221490612167565b565b61221e61267e565b612226612228565b565b612230612798565b565b61223a612216565b565b61224d9061224861267e565b61224f565b565b8061226a61226461225f5f611944565b6107c0565b916107c0565b1461227a5761227890612739565b565b61229d6122865f611944565b5f918291631e4fbdf760e01b835260048301610ccf565b0390fd5b6122aa9061223c565b565b5f7f476173436f756e7465723a20696e76616c69642072616e676500000000000000910152565b6122e060196020926112d5565b6122e9816122ac565b0190565b6123029060208101905f8183039101526122d3565b90565b1561230c57565b612314610252565b62461bcd60e51b81528061232a600482016122ed565b0390fd5b61235d9161233a611cda565b506123588161235161234b856102af565b916102af565b1015612305565b6117e2565b90565b61236a608061124a565b90565b634e487b7160e01b5f52601260045260245ffd5b61238d612393916102af565b916102af565b90811561239e570490565b61236d565b6123ab6112ae565b506123bf6123b96003611575565b15610515565b6124bb576123e06123db60046123d56002610f15565b90610ee6565b611b15565b4261240e6124086124036123f55f8601611cde565b6123fd610a2d565b90611807565b6102af565b916102af565b10156124175790565b6124649061245e61244f5f61244861243a42612434848801611cde565b906117e2565b612442610a2d565b90612381565b9301611cde565b91612458610a2d565b90611ea5565b90611807565b6124b86124af5f6124aa6124a15f61249c6124935f9561248e612485612360565b9a5f8c01611a9e565b611389565b60208901611a9e565b611389565b60408601611a9e565b611389565b60608301611a9e565b90565b5f61251861250f5f61250a6125015f6124fc6124f35f956124ee6124e66124e0612360565b9b611389565b5f8c01611a9e565b611389565b60208901611a9e565b611389565b60408601611a9e565b611389565b60608301611a9e565b90565b906125275f19916119b6565b9181191691161790565b90565b9061254961254461255092610eca565b612531565b825461251b565b9055565b91602061257592949361256e60408201965f83019061072a565b019061072a565b565b61258a61258460036117c1565b15610515565b61267b576125a161259b6003611575565b15610515565b61266e575b6125ae61296e565b61261f6125bc823a90611ea5565b6125ef836125e960026125d960046125d383610f15565b90610ee6565b01916125e483610f15565b611807565b90612534565b612619600361260960046126036002610f15565b90610ee6565b019161261483610f15565b611807565b90612534565b6126296002610f15565b3a6126547f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610eca565b92612669612660610252565b92839283612554565b0390a2565b612676612863565b6125a6565b50565b612686611f9f565b61269f612699612694612b6e565b6107c0565b916107c0565b036126a657565b6126c86126b1612b6e565b5f91829163118cdaa760e01b835260048301610ccf565b0390fd5b60081b90565b906126df61ff00916126cc565b9181191691161790565b906126fe6126f96127059261196f565b61197b565b82546126d2565b9055565b6127145f60036126e9565b565b90565b9061272e6127296127359261116c565b612716565b82546119bb565b9055565b6127425f611f92565b61274c825f612719565b9061278061277a7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09361116c565b9161116c565b91612789610252565b806127938161031a565b0390a3565b6127a4600160036126e9565b565b906127b260ff916119b6565b9181191691161790565b906127d16127cc6127d89261196f565b61197b565b82546127a6565b9055565b906127e690611389565b5f5260205260405f2090565b9061284f60606003612855946128155f820161280f5f8801611cde565b90612534565b61282e6001820161282860208801611cde565b90612534565b6128476002820161284160408801611cde565b90612534565b019201611cde565b90612534565b565b90612861916127f2565b565b6128766128706003611575565b15610515565b61287d575b565b612889600160036127bc565b61289c6128955f611389565b6002612534565b612905426128f46128eb5f6128e66128dd5f6128d86128cf5f956128ca6128c1612360565b9a5f8c01611a9e565b611389565b60208901611a9e565b611389565b60408601611a9e565b611389565b60608301611a9e565b61290060045f906127dc565b612857565b5f42906129476129357f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92611389565b9261293e610252565b91829182610737565b0390a261287b565b90565b61295b906102af565b5f1981146129695760010190565b6117ce565b61298b61298660046129806002610f15565b90610ee6565b61294f565b426129b96129b36129ae6129a05f8601610f15565b6129a8610a2d565b90611807565b6102af565b916102af565b10156129c3575b50565b6129eb6129e26129d45f8401610f15565b6129dc610a2d565b90611807565b60018301612534565b612a13612a0c6129fd60038401610f15565b612a076005610f15565b611807565b6005612534565b612a1d6002610f15565b612a4a612a2c60028401610f15565b92612a445f612a3d60018401610f15565b9201610f15565b906117e2565b612a747f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610eca565b92612a89612a80610252565b92839283612554565b0390a2612aa8612aa1612a9c6002610f15565b612952565b6002612534565b612b1a42612b00612af75f612af2612ae95f612ae4612adb5f95612ad6612acd612360565b9a5f8c01611a9e565b611389565b60208901611a9e565b611389565b60408601611a9e565b611389565b60608301611a9e565b612b156004612b0f6002610f15565b90610ee6565b612857565b612b246002610f15565b4290612b65612b537f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610eca565b92612b5c610252565b91829182610737565b0390a25f6129c0565b612b76611f6f565b50339056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x11 V[a\0\x1D_5a\x02LV[\x80c\x05\x0E\xC18\x14a\x02GW\x80c\x08aF\xD2\x14a\x02BW\x80c\x11\x99/\x8C\x14a\x02=W\x80c\x18\xD5\xAA\xFE\x14a\x028W\x80c\x1C\x0Bcg\x14a\x023W\x80c6l\xBA\xB7\x14a\x02.W\x80c;j\xB2\xA9\x14a\x02)W\x80c=D\xAE\x8B\x14a\x02$W\x80cF\xE2\xCC\t\x14a\x02\x1FW\x80cH\\\xC9U\x14a\x02\x1AW\x80cK,\x07\x06\x14a\x02\x15W\x80cTg\xCBH\x14a\x02\x10W\x80c[<\xD6\xE2\x14a\x02\x0BW\x80caT8\x01\x14a\x02\x06W\x80ceX\x95O\x14a\x02\x01W\x80cqP\x18\xA6\x14a\x01\xFCW\x80cz9y\xDC\x14a\x01\xF7W\x80c\x7F\xBD)^\x14a\x01\xF2W\x80c\x80NQ#\x14a\x01\xEDW\x80c\x82\xF4J\xDE\x14a\x01\xE8W\x80c\x83\xD3\xC1\x15\x14a\x01\xE3W\x80c\x84\xFA\xB6+\x14a\x01\xDEW\x80c\x8DZ#\x9B\x14a\x01\xD9W\x80c\x8D\xA5\xCB[\x14a\x01\xD4W\x80c\xAF\xF7Lm\x14a\x01\xCFW\x80c\xC6`\xD3\xF3\x14a\x01\xCAW\x80c\xCD\xAF\xB9x\x14a\x01\xC5W\x80c\xD4\xF0\xEBM\x14a\x01\xC0W\x80c\xD8x\x13B\x14a\x01\xBBW\x80c\xDE\x1FE>\x14a\x01\xB6W\x80c\xEAJ\x11\x04\x14a\x01\xB1W\x80c\xED\xE0{\xD6\x14a\x01\xACW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xA7W\x80c\xF7\xB8\x93^\x14a\x01\xA2Wc\xFF{0\x84\x03a\0\x0EWa\x10\xEBV[a\x10\xA6V[a\x10FV[a\x10\x11V[a\x0F\xA0V[a\x0E\x97V[a\x0EbV[a\x0E\x0BV[a\r\xB9V[a\rNV[a\r\x19V[a\x0C\xE4V[a\x0C\x8DV[a\x0CXV[a\x0C\x12V[a\x0B\xA3V[a\x0BoV[a\x0B:V[a\x0B\x01V[a\n|V[a\nGV[a\t\xD9V[a\tlV[a\x08\xA3V[a\x08nV[a\x08\x1CV[a\x07\x81V[a\x07LV[a\x06\xBBV[a\x06FV[a\x05qV[a\x05<V[a\x04\xDEV[a\x03\xCCV[a\x03\x1FV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x02\xAAW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x02\xA5W` \x01\x92`\x01\x83\x02\x84\x01\x11a\x02\xA0WV[a\x02lV[a\x02hV[a\x02dV[\x90V[a\x02\xBB\x81a\x02\xAFV[\x03a\x02\xC2WV[_\x80\xFD[\x90P5\x90a\x02\xD3\x82a\x02\xB2V[V[\x91`@\x83\x83\x03\x12a\x03\x15W_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\x10Wa\x03\x02\x83a\x03\r\x92\x86\x01a\x02pV[\x93\x90\x94` \x01a\x02\xC6V[\x90V[a\x02`V[a\x02\\V[_\x01\x90V[4a\x03NWa\x038a\x0326`\x04a\x02\xD5V[\x91a\x12\0V[a\x03@a\x02RV[\x80a\x03J\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[_\x91\x03\x12a\x03]WV[a\x02\\V[a\x03k\x90a\x02\xAFV[\x90RV[\x90``\x80a\x03\xB5\x93a\x03\x87_\x82\x01Q_\x86\x01\x90a\x03bV[a\x03\x99` \x82\x01Q` \x86\x01\x90a\x03bV[a\x03\xAB`@\x82\x01Q`@\x86\x01\x90a\x03bV[\x01Q\x91\x01\x90a\x03bV[V[\x91\x90a\x03\xCA\x90_`\x80\x85\x01\x94\x01\x90a\x03oV[V[4a\x03\xFCWa\x03\xDC6`\x04a\x03SV[a\x03\xF8a\x03\xE7a\x12\xB9V[a\x03\xEFa\x02RV[\x91\x82\x91\x82a\x03\xB7V[\x03\x90\xF3[a\x02XV[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04;W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x046W` \x01\x92` \x83\x02\x84\x01\x11a\x041WV[a\x02lV[a\x02hV[a\x02dV[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x04zW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04uW` \x01\x92` \x83\x02\x84\x01\x11a\x04pWV[a\x02lV[a\x02hV[a\x02dV[\x90\x91`@\x82\x84\x03\x12a\x04\xD9W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xD4W\x83a\x04\xAA\x91\x84\x01a\x04\x01V[\x92\x90\x93` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xCFWa\x04\xCB\x92\x01a\x04@V[\x90\x91V[a\x02`V[a\x02`V[a\x02\\V[4a\x05\x10Wa\x04\xFAa\x04\xF16`\x04a\x04\x7FV[\x92\x91\x90\x91a\x14]V[a\x05\x02a\x02RV[\x80a\x05\x0C\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x15\x15\x90V[a\x05#\x90a\x05\x15V[\x90RV[\x91\x90a\x05:\x90_` \x85\x01\x94\x01\x90a\x05\x1AV[V[4a\x05lWa\x05L6`\x04a\x03SV[a\x05ha\x05Wa\x15\x82V[a\x05_a\x02RV[\x91\x82\x91\x82a\x05'V[\x03\x90\xF3[a\x02XV[4a\x05\xA0Wa\x05\x8Aa\x05\x846`\x04a\x02\xD5V[\x91a\x16\x8FV[a\x05\x92a\x02RV[\x80a\x05\x9C\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x90` \x82\x82\x03\x12a\x05\xD6W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05\xD1Wa\x05\xCD\x92\x01a\x02pV[\x90\x91V[a\x02`V[a\x02\\V[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x06\x1Ca\x06%` \x93a\x06*\x93a\x06\x13\x81a\x05\xDBV[\x93\x84\x80\x93a\x05\xDFV[\x95\x86\x91\x01a\x05\xE8V[a\x05\xF3V[\x01\x90V[a\x06C\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x05\xFDV[\x90V[4a\x06wWa\x06sa\x06ba\x06\\6`\x04a\x05\xA5V[\x90a\x17\x1FV[a\x06ja\x02RV[\x91\x82\x91\x82a\x06.V[\x03\x90\xF3[a\x02XV[\x1C\x90V[`\xFF\x16\x90V[a\x06\x96\x90`\x08a\x06\x9B\x93\x02a\x06|V[a\x06\x80V[\x90V[\x90a\x06\xA9\x91Ta\x06\x86V[\x90V[a\x06\xB8`\x03_\x90a\x06\x9EV[\x90V[4a\x06\xEBWa\x06\xCB6`\x04a\x03SV[a\x06\xE7a\x06\xD6a\x06\xACV[a\x06\xDEa\x02RV[\x91\x82\x91\x82a\x05'V[\x03\x90\xF3[a\x02XV[\x90V[\x90V[a\x07\na\x07\x05a\x07\x0F\x92a\x06\xF0V[a\x06\xF3V[a\x02\xAFV[\x90V[a\x07\x1C`\na\x06\xF6V[\x90V[a\x07'a\x07\x12V[\x90V[a\x073\x90a\x02\xAFV[\x90RV[\x91\x90a\x07J\x90_` \x85\x01\x94\x01\x90a\x07*V[V[4a\x07|Wa\x07\\6`\x04a\x03SV[a\x07xa\x07ga\x07\x1FV[a\x07oa\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[4a\x07\xB0Wa\x07\x9Aa\x07\x946`\x04a\x05\xA5V[\x90a\x18\xE1V[a\x07\xA2a\x02RV[\x80a\x07\xAC\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x07\xC9\x90a\x07\xB5V[\x90V[a\x07\xD5\x81a\x07\xC0V[\x03a\x07\xDCWV[_\x80\xFD[\x90P5\x90a\x07\xED\x82a\x07\xCCV[V[\x91\x90`@\x83\x82\x03\x12a\x08\x17W\x80a\x08\x0Ba\x08\x14\x92_\x86\x01a\x07\xE0V[\x93` \x01a\x07\xE0V[\x90V[a\x02\\V[4a\x08KWa\x085a\x08/6`\x04a\x07\xEFV[\x90a\x1A\x92V[a\x08=a\x02RV[\x80a\x08G\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x90` \x82\x82\x03\x12a\x08iWa\x08f\x91_\x01a\x02\xC6V[\x90V[a\x02\\V[4a\x08\x9EWa\x08\x9Aa\x08\x89a\x08\x846`\x04a\x08PV[a\x1B!V[a\x08\x91a\x02RV[\x91\x82\x91\x82a\x03\xB7V[\x03\x90\xF3[a\x02XV[4a\x08\xD1Wa\x08\xB36`\x04a\x03SV[a\x08\xBBa\x1B\\V[a\x08\xC3a\x02RV[\x80a\x08\xCD\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08\xF1\x90`\x08a\x08\xF6\x93\x02a\x06|V[a\x08\xD6V[\x90V[\x90a\t\x04\x91Ta\x08\xE1V[\x90V[a\t\x13`\x01_\x90a\x08\xF9V[\x90V[a\t*a\t%a\t/\x92a\x07\xB5V[a\x06\xF3V[a\x07\xB5V[\x90V[a\t;\x90a\t\x16V[\x90V[a\tG\x90a\t2V[\x90V[a\tS\x90a\t>V[\x90RV[\x91\x90a\tj\x90_` \x85\x01\x94\x01\x90a\tJV[V[4a\t\x9CWa\t|6`\x04a\x03SV[a\t\x98a\t\x87a\t\x07V[a\t\x8Fa\x02RV[\x91\x82\x91\x82a\tWV[\x03\x90\xF3[a\x02XV[\x90V[a\t\xB4\x90`\x08a\t\xB9\x93\x02a\x06|V[a\t\xA1V[\x90V[\x90a\t\xC7\x91Ta\t\xA4V[\x90V[a\t\xD6`\x02_\x90a\t\xBCV[\x90V[4a\n\tWa\t\xE96`\x04a\x03SV[a\n\x05a\t\xF4a\t\xCAV[a\t\xFCa\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[\x90V[a\n%a\n a\n*\x92a\n\x0EV[a\x06\xF3V[a\x02\xAFV[\x90V[a\n9b'\x8D\0a\n\x11V[\x90V[a\nDa\n-V[\x90V[4a\nwWa\nW6`\x04a\x03SV[a\nsa\nba\n<V[a\nja\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[4a\n\xAAWa\n\x8C6`\x04a\x03SV[a\n\x94a\x1B\x8BV[a\n\x9Ca\x02RV[\x80a\n\xA6\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x91``\x83\x83\x03\x12a\n\xFCWa\n\xC6\x82_\x85\x01a\x07\xE0V[\x92a\n\xD4\x83` \x83\x01a\x07\xE0V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xF7Wa\n\xF3\x92\x01a\x02pV[\x90\x91V[a\x02`V[a\x02\\V[4a\x0B5Wa\x0B1a\x0B a\x0B\x176`\x04a\n\xAFV[\x92\x91\x90\x91a\x1CCV[a\x0B(a\x02RV[\x91\x82\x91\x82a\x05'V[\x03\x90\xF3[a\x02XV[4a\x0BjWa\x0BJ6`\x04a\x03SV[a\x0Bfa\x0BUa\x1C\xEBV[a\x0B]a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[4a\x0B\x9EWa\x0B\x88a\x0B\x826`\x04a\x05\xA5V[\x90a\x1E\tV[a\x0B\x90a\x02RV[\x80a\x0B\x9A\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[4a\x0B\xD3Wa\x0B\xB36`\x04a\x03SV[a\x0B\xCFa\x0B\xBEa\x1E\x15V[a\x0B\xC6a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[\x90\x91``\x82\x84\x03\x12a\x0C\rWa\x0C\na\x0B\xF3\x84_\x85\x01a\x02\xC6V[\x93a\x0C\x01\x81` \x86\x01a\x02\xC6V[\x93`@\x01a\x02\xC6V[\x90V[a\x02\\V[4a\x0CCWa\x0C?a\x0C.a\x0C(6`\x04a\x0B\xD8V[\x91a\x1E\xDAV[a\x0C6a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[a\x0CU`\x03`\x01\x90a\x06\x9EV[\x90V[4a\x0C\x88Wa\x0Ch6`\x04a\x03SV[a\x0C\x84a\x0Csa\x0CHV[a\x0C{a\x02RV[\x91\x82\x91\x82a\x05'V[\x03\x90\xF3[a\x02XV[4a\x0C\xBDWa\x0C\x9D6`\x04a\x03SV[a\x0C\xB9a\x0C\xA8a\x1FPV[a\x0C\xB0a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[a\x0C\xCB\x90a\x07\xC0V[\x90RV[\x91\x90a\x0C\xE2\x90_` \x85\x01\x94\x01\x90a\x0C\xC2V[V[4a\r\x14Wa\x0C\xF46`\x04a\x03SV[a\r\x10a\x0C\xFFa\x1F\x9FV[a\r\x07a\x02RV[\x91\x82\x91\x82a\x0C\xCFV[\x03\x90\xF3[a\x02XV[4a\rIWa\r)6`\x04a\x03SV[a\rEa\r4a\x1F\xD3V[a\r<a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[4a\r~Wa\r^6`\x04a\x03SV[a\rza\ria \x1FV[a\rqa\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[\x90` \x82\x82\x03\x12a\r\xB4W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r\xAFWa\r\xAB\x92\x01a\x04\x01V[\x90\x91V[a\x02`V[a\x02\\V[4a\r\xE8Wa\r\xD2a\r\xCC6`\x04a\r\x83V[\x90a![V[a\r\xDAa\x02RV[\x80a\r\xE4\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x90` \x82\x82\x03\x12a\x0E\x06Wa\x0E\x03\x91_\x01a\x07\xE0V[\x90V[a\x02\\V[4a\x0E9Wa\x0E#a\x0E\x1E6`\x04a\r\xEDV[a\"\x0BV[a\x0E+a\x02RV[\x80a\x0E5\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0E\x92Wa\x0Er6`\x04a\x03SV[a\x0E\x8Ea\x0E}a\x0E>V[a\x0E\x85a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[4a\x0E\xC5Wa\x0E\xA76`\x04a\x03SV[a\x0E\xAFa\"2V[a\x0E\xB7a\x02RV[\x80a\x0E\xC1\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[a\x0E\xDEa\x0E\xD9a\x0E\xE3\x92a\x02\xAFV[a\x06\xF3V[a\x02\xAFV[\x90V[\x90a\x0E\xF0\x90a\x0E\xCAV[_R` R`@_ \x90V[_\x1C\x90V[a\x0F\ra\x0F\x12\x91a\x0E\xFCV[a\t\xA1V[\x90V[a\x0F\x1F\x90Ta\x0F\x01V[\x90V[a\x0F-\x90`\x04a\x0E\xE6V[\x90a\x0F9_\x83\x01a\x0F\x15V[\x91a\x0FF`\x01\x82\x01a\x0F\x15V[\x91a\x0F_`\x03a\x0FX`\x02\x85\x01a\x0F\x15V[\x93\x01a\x0F\x15V[\x90V[a\x0F\x97a\x0F\x9E\x94a\x0F\x8D``\x94\x98\x97\x95a\x0F\x83`\x80\x86\x01\x9A_\x87\x01\x90a\x07*V[` \x85\x01\x90a\x07*V[`@\x83\x01\x90a\x07*V[\x01\x90a\x07*V[V[4a\x0F\xD4Wa\x0F\xD0a\x0F\xBBa\x0F\xB66`\x04a\x08PV[a\x0F\"V[\x90a\x0F\xC7\x94\x92\x94a\x02RV[\x94\x85\x94\x85a\x0FbV[\x03\x90\xF3[a\x02XV[\x90V[a\x0F\xF0a\x0F\xEBa\x0F\xF5\x92a\x0F\xD9V[a\x06\xF3V[a\x02\xAFV[\x90V[a\x10\x03a\x13\x88a\x0F\xDCV[\x90V[a\x10\x0Ea\x0F\xF8V[\x90V[4a\x10AWa\x10!6`\x04a\x03SV[a\x10=a\x10,a\x10\x06V[a\x104a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[4a\x10tWa\x10^a\x10Y6`\x04a\r\xEDV[a\"\xA1V[a\x10fa\x02RV[\x80a\x10p\x81a\x03\x1AV[\x03\x90\xF3[a\x02XV[\x91\x90`@\x83\x82\x03\x12a\x10\xA1W\x80a\x10\x95a\x10\x9E\x92_\x86\x01a\x02\xC6V[\x93` \x01a\x02\xC6V[\x90V[a\x02\\V[4a\x10\xD7Wa\x10\xD3a\x10\xC2a\x10\xBC6`\x04a\x10yV[\x90a#.V[a\x10\xCAa\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[a\x10\xE8`\x05_\x90a\t\xBCV[\x90V[4a\x11\x1BWa\x10\xFB6`\x04a\x03SV[a\x11\x17a\x11\x06a\x10\xDCV[a\x11\x0Ea\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x02XV[_\x80\xFD[\x91\x90a\x11Aa\x11;32\x90\x86\x85\x91\x92\x90\x91\x92a\x1CCV[\x15a\x05\x15V[a\x11PWa\x11N\x92a\x11\xADV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x11h`\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[a\x11u\x90a\t2V[\x90V[`@\x90a\x11\xA4a\x11\x99a\x11\xAB\x95\x97\x96\x94``\x84\x01\x90\x84\x82\x03_\x86\x01Ra\x05\xFDV[\x96` \x83\x01\x90a\x07*V[\x01\x90a\x07*V[V[\x90a\x11\xB9\x903\x92a\x17\x1FV[\x91B\x92a\x11\xFBa\x11\xE9\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x11lV[\x94a\x11\xF2a\x02RV[\x93\x84\x93\x84a\x11xV[\x03\x90\xA2V[\x90a\x12\x0B\x92\x91a\x11$V[V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x12+\x90a\x05\xF3V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x12EW`@RV[a\x12\rV[\x90a\x12]a\x12Va\x02RV[\x92\x83a\x12!V[V[a\x12i`\x80a\x12JV[\x90V[_\x90V[a\x12xa\x12_V[\x90` \x80\x80\x80\x85a\x12\x87a\x12lV[\x81R\x01a\x12\x92a\x12lV[\x81R\x01a\x12\x9Da\x12lV[\x81R\x01a\x12\xA8a\x12lV[\x81RPPV[a\x12\xB6a\x12pV[\x90V[a\x12\xC1a\x12\xAEV[Pa\x12\xCAa#\xA3V[\x90V[P\x90V[P\x90V[` \x91\x81R\x01\x90V[` \x7Fve the same length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x7FData and priority arrays must ha_\x82\x01R\x01RV[a\x138`2`@\x92a\x12\xD5V[a\x13A\x81a\x12\xDEV[\x01\x90V[a\x13Z\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x13+V[\x90V[\x15a\x13dWV[a\x13la\x02RV[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x13\x82`\x04\x82\x01a\x13EV[\x03\x90\xFD[\x90V[a\x13\x9Da\x13\x98a\x13\xA2\x92a\x13\x86V[a\x06\xF3V[a\x02\xAFV[\x90V[`\x01a\x13\xB1\x91\x01a\x02\xAFV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x14\x16W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x14\x11W` \x01\x91`\x01\x82\x026\x03\x83\x13a\x14\x0CWV[a\x13\xD0V[a\x13\xCCV[a\x13\xC8V[\x90\x82\x10\x15a\x146W` a\x142\x92\x02\x81\x01\x90a\x13\xD4V[\x90\x91V[a\x13\xB4V[\x91\x90\x81\x10\x15a\x14KW` \x02\x01\x90V[a\x13\xB4V[5a\x14Z\x81a\x02\xB2V[\x90V[\x90\x92a\x14j\x82\x85\x90a\x12\xCDV[\x93a\x14\x91\x85a\x14\x8Ba\x14\x85a\x14\x80\x88\x87\x90a\x12\xD1V[a\x02\xAFV[\x91a\x02\xAFV[\x14a\x13]V[a\x14\x9A_a\x13\x89V[[\x80a\x14\xAEa\x14\xA8\x88a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a\x15UWa\x14\xDC\x90a\x14\xD232\x90a\x14\xCA\x88\x87\x86\x91a\x14\x1BV[\x92\x90\x91a\x1CCV[a\x14\xE1W[a\x13\xA5V[a\x14\x9BV[3a\x14\xF7a\x14\xF1\x87\x86\x85\x91a\x14\x1BV[\x90a\x17\x1FV[\x90a\x15\x0Ca\x15\x07\x89\x88\x86\x91a\x14;V[a\x14PV[B\x92a\x15Ma\x15;\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x94a\x11lV[\x94a\x15Da\x02RV[\x93\x84\x93\x84a\x11xV[\x03\x90\xA2a\x14\xD7V[PPPPPPV[_\x90V[a\x15ma\x15r\x91a\x0E\xFCV[a\x06\x80V[\x90V[a\x15\x7F\x90Ta\x15aV[\x90V[a\x15\x8Aa\x15]V[Pa\x15\x95`\x03a\x15uV[\x90V[\x91\x90a\x15\xB5a\x15\xAF32\x90\x86\x85\x91\x92\x90\x91\x92a\x1CCV[\x15a\x05\x15V[a\x15\xC4Wa\x15\xC2\x92a\x16CV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x15\xDC`\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[\x90\x82_\x93\x92\x827\x01RV[\x91\x90a\x16\x05\x81a\x15\xFE\x81a\x16\n\x95a\x05\xDFV[\x80\x95a\x15\xE0V[a\x05\xF3V[\x01\x90V[a\x16:a\x16/`@\x93a\x16A\x96\x98\x97\x95``\x85\x01\x91\x85\x83\x03_\x87\x01Ra\x15\xEBV[\x96` \x83\x01\x90a\x07*V[\x01\x90a\x07*V[V[\x90\x913\x91\x92\x90\x92a\x16\x8ABa\x16x\x7FKZ\xA8\xD0\x82\xE6\x91\xCB\x99r\xA7\x95\x8F\xA4\x15?f?!_\xE6\x97\xA3\xE0\x8B\xD2r\x9E\xD7\x8F\x02\xF2\x95a\x11lV[\x95a\x16\x81a\x02RV[\x94\x85\x94\x85a\x16\x0EV[\x03\x90\xA2V[\x90a\x16\x9A\x92\x91a\x15\x98V[V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x16\xC4a\x16\xBFa\x16\xC9\x92a\x13\x86V[a\x16\xAAV[a\x16\xA1V[\x90V[\x90V[a\x16\xDBa\x16\xE0\x91a\x16\xA1V[a\x16\xCCV[\x90RV[\x90P\x90V[\x90\x91\x82a\x16\xF9\x81a\x17\0\x93a\x16\xE4V[\x80\x93a\x15\xE0V[\x01\x90V[\x80a\x17\x15`\x01\x92a\x17\x1C\x96\x94a\x16\xCFV[\x01\x91a\x16\xE9V[\x90V[a\x17]\x90a\x17+a\x16\x9CV[Pa\x17Na\x178_a\x16\xB0V[\x91\x93a\x17Ba\x02RV[\x94\x85\x93` \x85\x01a\x17\x04V[` \x82\x01\x81\x03\x82R\x03\x82a\x12!V[\x90V[\x90a\x17|a\x17v32\x90\x85\x85\x91\x92\x90\x91\x92a\x1CCV[\x15a\x05\x15V[a\x17\x8BWa\x17\x89\x91a\x18,V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x17\xA3`\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[`\x08\x1C\x90V[a\x17\xB9a\x17\xBE\x91a\x17\xA7V[a\x06\x80V[\x90V[a\x17\xCB\x90Ta\x17\xADV[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x17\xF1a\x17\xF7\x91\x93\x92\x93a\x02\xAFV[\x92a\x02\xAFV[\x82\x03\x91\x82\x11a\x18\x02WV[a\x17\xCEV[a\x18\x16a\x18\x1C\x91\x93\x92\x93a\x02\xAFV[\x92a\x02\xAFV[\x82\x01\x80\x92\x11a\x18'WV[a\x17\xCEV[\x90a\x18@a\x18:`\x03a\x17\xC1V[\x15a\x05\x15V[a\x18uWa\x18`a\x18s\x92a\x18Ya\x18n\x93Z\x92a\x18\x9AV[Z\x90a\x17\xE2V[a\x18ha\x0F\xF8V[\x90a\x18\x07V[a%wV[V[a\x18~\x91a\x18\x9AV[V[\x90\x91a\x18\x97\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x15\xEBV[\x90V[3\x90\x91a\x18\xC7\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11lV[\x92a\x18\xDCa\x18\xD3a\x02RV[\x92\x83\x92\x83a\x18\x80V[\x03\x90\xA2V[\x90a\x18\xEB\x91a\x17`V[V[\x90a\x18\xFF\x91a\x18\xFAa&~V[a\x1A\x05V[V[`\xA0\x1C\x90V[a\x19\x13a\x19\x18\x91a\x19\x01V[a\x06\x80V[\x90V[a\x19%\x90Ta\x19\x07V[\x90V[a\x19<a\x197a\x19A\x92a\x13\x86V[a\x06\xF3V[a\x07\xB5V[\x90V[a\x19M\x90a\x19(V[\x90V[`\xA0\x1B\x90V[\x90a\x19e`\xFF`\xA0\x1B\x91a\x19PV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x19x\x90a\x05\x15V[\x90V[\x90V[\x90a\x19\x93a\x19\x8Ea\x19\x9A\x92a\x19oV[a\x19{V[\x82Ta\x19VV[\x90UV[a\x19\xA7\x90a\t\x16V[\x90V[a\x19\xB3\x90a\x19\x9EV[\x90V[_\x1B\x90V[\x90a\x19\xCC`\x01\x80`\xA0\x1B\x03\x91a\x19\xB6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x19\xDF\x90a\x19\x9EV[\x90V[\x90V[\x90a\x19\xFAa\x19\xF5a\x1A\x01\x92a\x19\xD6V[a\x19\xE2V[\x82Ta\x19\xBBV[\x90UV[a\x1A\x0F`\x01a\x19\x1BV[a\x1AwW\x81a\x1A.a\x1A(a\x1A#_a\x19DV[a\x07\xC0V[\x91a\x07\xC0V[\x14a\x1A[Wa\x1ATa\x1AMa\x1AY\x93a\x1AH`\x01\x80a\x19~V[a\x19\xAAV[`\x01a\x19\xE5V[a\"\xA1V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x1As`\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x1A\x8E`\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[\x90a\x1A\x9C\x91a\x18\xEDV[V[\x90a\x1A\xA8\x90a\x02\xAFV[\x90RV[\x90a\x1B\x13a\x1B\n`\x03a\x1A\xBDa\x12_V[\x94a\x1A\xD4a\x1A\xCC_\x83\x01a\x0F\x15V[_\x88\x01a\x1A\x9EV[a\x1A\xECa\x1A\xE3`\x01\x83\x01a\x0F\x15V[` \x88\x01a\x1A\x9EV[a\x1B\x04a\x1A\xFB`\x02\x83\x01a\x0F\x15V[`@\x88\x01a\x1A\x9EV[\x01a\x0F\x15V[``\x84\x01a\x1A\x9EV[V[a\x1B\x1E\x90a\x1A\xACV[\x90V[a\x1B8a\x1B=\x91a\x1B0a\x12\xAEV[P`\x04a\x0E\xE6V[a\x1B\x15V[\x90V[a\x1BHa&~V[a\x1BPa\x1BRV[V[a\x1BZa'\tV[V[a\x1Bda\x1B@V[V[a\x1Bna&~V[a\x1Bva\x1BxV[V[a\x1B\x89a\x1B\x84_a\x19DV[a'9V[V[a\x1B\x93a\x1BfV[V[a\x1B\xA1a\x1B\xA6\x91a\x0E\xFCV[a\x08\xD6V[\x90V[a\x1B\xB3\x90Ta\x1B\x95V[\x90V[`\xE0\x1B\x90V[a\x1B\xC5\x81a\x05\x15V[\x03a\x1B\xCCWV[_\x80\xFD[\x90PQ\x90a\x1B\xDD\x82a\x1B\xBCV[V[\x90` \x82\x82\x03\x12a\x1B\xF8Wa\x1B\xF5\x91_\x01a\x1B\xD0V[\x90V[a\x02\\V[a\x1C#a\x1C0\x95\x93\x94\x92\x94a\x1C\x19``\x84\x01\x96_\x85\x01\x90a\x0C\xC2V[` \x83\x01\x90a\x0C\xC2V[`@\x81\x85\x03\x91\x01Ra\x15\xEBV[\x90V[a\x1C;a\x02RV[=_\x82>=\x90\xFD[\x92a\x1C\x86` \x93\x94a\x1CSa\x15]V[Pa\x1C\x91a\x1Cia\x1Cd`\x01a\x1B\xA9V[a\t>V[\x93cz9y\xDC\x92\x95\x97a\x1Cza\x02RV[\x98\x89\x97\x88\x96\x87\x96a\x1B\xB6V[\x86R`\x04\x86\x01a\x1B\xFDV[\x03\x91Z\xFA\x90\x81\x15a\x1C\xD5W_\x91a\x1C\xA7W[P\x90V[a\x1C\xC8\x91P` =\x81\x11a\x1C\xCEW[a\x1C\xC0\x81\x83a\x12!V[\x81\x01\x90a\x1B\xDFV[_a\x1C\xA3V[P=a\x1C\xB6V[a\x1C3V[_\x90V[a\x1C\xE8\x90Qa\x02\xAFV[\x90V[a\x1C\xF3a\x1C\xDAV[Pa\x1D\x1Aa\x1D\x01`\x05a\x0F\x15V[a\x1D\x14``a\x1D\x0Ea#\xA3V[\x01a\x1C\xDEV[\x90a\x18\x07V[\x90V[\x90a\x1D9a\x1D332\x90\x85\x85\x91\x92\x90\x91\x92a\x1CCV[\x15a\x05\x15V[a\x1DHWa\x1DF\x91a\x1DdV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x1D``\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[\x90a\x1Dxa\x1Dr`\x03a\x17\xC1V[\x15a\x05\x15V[a\x1D\xADWa\x1D\x98a\x1D\xAB\x92a\x1D\x91a\x1D\xA6\x93Z\x92a\x1D\xB8V[Z\x90a\x17\xE2V[a\x1D\xA0a\x0F\xF8V[\x90a\x18\x07V[a%wV[V[a\x1D\xB6\x91a\x1D\xB8V[V[\x90a\x1D\xC4\x903\x92a\x17\x1FV[\x90a\x1E\x04a\x1D\xF2\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11lV[\x92a\x1D\xFBa\x02RV[\x91\x82\x91\x82a\x06.V[\x03\x90\xA2V[\x90a\x1E\x13\x91a\x1D\x1DV[V[a\x1E\x1Da\x1C\xDAV[Pa\x1E&a#\xA3V[a\x1E1_\x82\x01a\x1C\xDEV[a\x1ECa\x1E=_a\x13\x89V[\x91a\x02\xAFV[\x14a\x1E\x98Wa\x1EV_a\x1Ed\x92\x01a\x1C\xDEV[a\x1E^a\n-V[\x90a\x18\x07V[Ba\x1Ewa\x1Eq\x83a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a\x1E\x8BWa\x1E\x88\x90B\x90a\x17\xE2V[\x90V[Pa\x1E\x95_a\x13\x89V[\x90V[Pa\x1E\xA2_a\x13\x89V[\x90V[a\x1E\xB4a\x1E\xBA\x91\x93\x92\x93a\x02\xAFV[\x92a\x02\xAFV[\x91a\x1E\xC6\x83\x82\x02a\x02\xAFV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1E\xD5WV[a\x17\xCEV[\x91a\x1E\xE3a\x1C\xDAV[P\x80a\x1E\xF7a\x1E\xF1\x84a\x02\xAFV[\x91a\x02\xAFV[\x11\x15a\x1FKWa\x1F\x18\x91a\x1F\n\x91a\x17\xE2V[a\x1F\x12a\x07\x12V[\x90a\x1E\xA5V[\x80a\x1F+a\x1F%\x84a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a\x1F=Wa\x1F:\x91a\x17\xE2V[\x90V[PPa\x1FH_a\x13\x89V[\x90V[PP\x90V[a\x1FXa\x1C\xDAV[Pa\x1Fl``a\x1Ffa#\xA3V[\x01a\x1C\xDEV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1F\x8Aa\x1F\x8F\x91a\x0E\xFCV[a\x1FsV[\x90V[a\x1F\x9C\x90Ta\x1F~V[\x90V[a\x1F\xA7a\x1FoV[Pa\x1F\xB1_a\x1F\x92V[\x90V[\x90V[a\x1F\xCBa\x1F\xC6a\x1F\xD0\x92a\x1F\xB4V[a\x06\xF3V[a\x02\xAFV[\x90V[a\x1F\xDBa\x1C\xDAV[Pa\x1F\xEFa\x1F\xE9`\x03a\x15uV[\x15a\x05\x15V[a \x13Wa \x10a \0`\x02a\x0F\x15V[a \n`\x01a\x1F\xB7V[\x90a\x18\x07V[\x90V[a \x1C_a\x13\x89V[\x90V[a 'a\x1C\xDAV[Pa ;`@a 5a#\xA3V[\x01a\x1C\xDEV[\x90V[\x90a Ra L`\x03a\x17\xC1V[\x15a\x05\x15V[a \x87Wa ra \x85\x92a ka \x80\x93Z\x92a \x92V[Z\x90a\x17\xE2V[a za\x0F\xF8V[\x90a\x18\x07V[a%wV[V[a \x90\x91a \x92V[V[a \x9D\x81\x83\x90a\x12\xCDV[\x91a \xA6a\x1C\xDAV[Pa \xB0_a\x13\x89V[[\x80a \xC4a \xBE\x86a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a!UWa \xF2\x90a \xE832\x90a \xE0\x87\x87\x86\x91a\x14\x1BV[\x92\x90\x91a\x1CCV[a \xF7W[a\x13\xA5V[a \xB1V[3a!\ra!\x07\x86\x86\x85\x91a\x14\x1BV[\x90a\x17\x1FV[\x90a!Ma!;\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11lV[\x92a!Da\x02RV[\x91\x82\x91\x82a\x06.V[\x03\x90\xA2a \xEDV[PPPPV[\x90a!e\x91a >V[V[a!x\x90a!sa&~V[a!zV[V[\x80a!\x95a!\x8Fa!\x8A_a\x19DV[a\x07\xC0V[\x91a\x07\xC0V[\x14a!\xEFWa!\xADa!\xA6\x82a\x19\xAAV[`\x01a\x19\xE5V[a!\xD7\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x11lV[\x90a!\xE0a\x02RV[\x80a!\xEA\x81a\x03\x1AV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\"\x07`\x04\x82\x01a\x03\x1AV[\x03\x90\xFD[a\"\x14\x90a!gV[V[a\"\x1Ea&~V[a\"&a\"(V[V[a\"0a'\x98V[V[a\":a\"\x16V[V[a\"M\x90a\"Ha&~V[a\"OV[V[\x80a\"ja\"da\"__a\x19DV[a\x07\xC0V[\x91a\x07\xC0V[\x14a\"zWa\"x\x90a'9V[V[a\"\x9Da\"\x86_a\x19DV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x0C\xCFV[\x03\x90\xFD[a\"\xAA\x90a\"<V[V[_\x7FGasCounter: invalid range\0\0\0\0\0\0\0\x91\x01RV[a\"\xE0`\x19` \x92a\x12\xD5V[a\"\xE9\x81a\"\xACV[\x01\x90V[a#\x02\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\"\xD3V[\x90V[\x15a#\x0CWV[a#\x14a\x02RV[bF\x1B\xCD`\xE5\x1B\x81R\x80a#*`\x04\x82\x01a\"\xEDV[\x03\x90\xFD[a#]\x91a#:a\x1C\xDAV[Pa#X\x81a#Qa#K\x85a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a#\x05V[a\x17\xE2V[\x90V[a#j`\x80a\x12JV[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a#\x8Da#\x93\x91a\x02\xAFV[\x91a\x02\xAFV[\x90\x81\x15a#\x9EW\x04\x90V[a#mV[a#\xABa\x12\xAEV[Pa#\xBFa#\xB9`\x03a\x15uV[\x15a\x05\x15V[a$\xBBWa#\xE0a#\xDB`\x04a#\xD5`\x02a\x0F\x15V[\x90a\x0E\xE6V[a\x1B\x15V[Ba$\x0Ea$\x08a$\x03a#\xF5_\x86\x01a\x1C\xDEV[a#\xFDa\n-V[\x90a\x18\x07V[a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a$\x17W\x90V[a$d\x90a$^a$O_a$Ha$:Ba$4\x84\x88\x01a\x1C\xDEV[\x90a\x17\xE2V[a$Ba\n-V[\x90a#\x81V[\x93\x01a\x1C\xDEV[\x91a$Xa\n-V[\x90a\x1E\xA5V[\x90a\x18\x07V[a$\xB8a$\xAF_a$\xAAa$\xA1_a$\x9Ca$\x93_\x95a$\x8Ea$\x85a#`V[\x9A_\x8C\x01a\x1A\x9EV[a\x13\x89V[` \x89\x01a\x1A\x9EV[a\x13\x89V[`@\x86\x01a\x1A\x9EV[a\x13\x89V[``\x83\x01a\x1A\x9EV[\x90V[_a%\x18a%\x0F_a%\na%\x01_a$\xFCa$\xF3_\x95a$\xEEa$\xE6a$\xE0a#`V[\x9Ba\x13\x89V[_\x8C\x01a\x1A\x9EV[a\x13\x89V[` \x89\x01a\x1A\x9EV[a\x13\x89V[`@\x86\x01a\x1A\x9EV[a\x13\x89V[``\x83\x01a\x1A\x9EV[\x90V[\x90a%'_\x19\x91a\x19\xB6V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a%Ia%Da%P\x92a\x0E\xCAV[a%1V[\x82Ta%\x1BV[\x90UV[\x91` a%u\x92\x94\x93a%n`@\x82\x01\x96_\x83\x01\x90a\x07*V[\x01\x90a\x07*V[V[a%\x8Aa%\x84`\x03a\x17\xC1V[\x15a\x05\x15V[a&{Wa%\xA1a%\x9B`\x03a\x15uV[\x15a\x05\x15V[a&nW[a%\xAEa)nV[a&\x1Fa%\xBC\x82:\x90a\x1E\xA5V[a%\xEF\x83a%\xE9`\x02a%\xD9`\x04a%\xD3\x83a\x0F\x15V[\x90a\x0E\xE6V[\x01\x91a%\xE4\x83a\x0F\x15V[a\x18\x07V[\x90a%4V[a&\x19`\x03a&\t`\x04a&\x03`\x02a\x0F\x15V[\x90a\x0E\xE6V[\x01\x91a&\x14\x83a\x0F\x15V[a\x18\x07V[\x90a%4V[a&)`\x02a\x0F\x15V[:a&T\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0E\xCAV[\x92a&ia&`a\x02RV[\x92\x83\x92\x83a%TV[\x03\x90\xA2V[a&va(cV[a%\xA6V[PV[a&\x86a\x1F\x9FV[a&\x9Fa&\x99a&\x94a+nV[a\x07\xC0V[\x91a\x07\xC0V[\x03a&\xA6WV[a&\xC8a&\xB1a+nV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\x0C\xCFV[\x03\x90\xFD[`\x08\x1B\x90V[\x90a&\xDFa\xFF\0\x91a&\xCCV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a&\xFEa&\xF9a'\x05\x92a\x19oV[a\x19{V[\x82Ta&\xD2V[\x90UV[a'\x14_`\x03a&\xE9V[V[\x90V[\x90a'.a')a'5\x92a\x11lV[a'\x16V[\x82Ta\x19\xBBV[\x90UV[a'B_a\x1F\x92V[a'L\x82_a'\x19V[\x90a'\x80a'z\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x11lV[\x91a\x11lV[\x91a'\x89a\x02RV[\x80a'\x93\x81a\x03\x1AV[\x03\x90\xA3V[a'\xA4`\x01`\x03a&\xE9V[V[\x90a'\xB2`\xFF\x91a\x19\xB6V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a'\xD1a'\xCCa'\xD8\x92a\x19oV[a\x19{V[\x82Ta'\xA6V[\x90UV[\x90a'\xE6\x90a\x13\x89V[_R` R`@_ \x90V[\x90a(O```\x03a(U\x94a(\x15_\x82\x01a(\x0F_\x88\x01a\x1C\xDEV[\x90a%4V[a(.`\x01\x82\x01a((` \x88\x01a\x1C\xDEV[\x90a%4V[a(G`\x02\x82\x01a(A`@\x88\x01a\x1C\xDEV[\x90a%4V[\x01\x92\x01a\x1C\xDEV[\x90a%4V[V[\x90a(a\x91a'\xF2V[V[a(va(p`\x03a\x15uV[\x15a\x05\x15V[a(}W[V[a(\x89`\x01`\x03a'\xBCV[a(\x9Ca(\x95_a\x13\x89V[`\x02a%4V[a)\x05Ba(\xF4a(\xEB_a(\xE6a(\xDD_a(\xD8a(\xCF_\x95a(\xCAa(\xC1a#`V[\x9A_\x8C\x01a\x1A\x9EV[a\x13\x89V[` \x89\x01a\x1A\x9EV[a\x13\x89V[`@\x86\x01a\x1A\x9EV[a\x13\x89V[``\x83\x01a\x1A\x9EV[a)\0`\x04_\x90a'\xDCV[a(WV[_B\x90a)Ga)5\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x13\x89V[\x92a)>a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xA2a({V[\x90V[a)[\x90a\x02\xAFV[_\x19\x81\x14a)iW`\x01\x01\x90V[a\x17\xCEV[a)\x8Ba)\x86`\x04a)\x80`\x02a\x0F\x15V[\x90a\x0E\xE6V[a)OV[Ba)\xB9a)\xB3a)\xAEa)\xA0_\x86\x01a\x0F\x15V[a)\xA8a\n-V[\x90a\x18\x07V[a\x02\xAFV[\x91a\x02\xAFV[\x10\x15a)\xC3W[PV[a)\xEBa)\xE2a)\xD4_\x84\x01a\x0F\x15V[a)\xDCa\n-V[\x90a\x18\x07V[`\x01\x83\x01a%4V[a*\x13a*\x0Ca)\xFD`\x03\x84\x01a\x0F\x15V[a*\x07`\x05a\x0F\x15V[a\x18\x07V[`\x05a%4V[a*\x1D`\x02a\x0F\x15V[a*Ja*,`\x02\x84\x01a\x0F\x15V[\x92a*D_a*=`\x01\x84\x01a\x0F\x15V[\x92\x01a\x0F\x15V[\x90a\x17\xE2V[a*t\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0E\xCAV[\x92a*\x89a*\x80a\x02RV[\x92\x83\x92\x83a%TV[\x03\x90\xA2a*\xA8a*\xA1a*\x9C`\x02a\x0F\x15V[a)RV[`\x02a%4V[a+\x1ABa+\0a*\xF7_a*\xF2a*\xE9_a*\xE4a*\xDB_\x95a*\xD6a*\xCDa#`V[\x9A_\x8C\x01a\x1A\x9EV[a\x13\x89V[` \x89\x01a\x1A\x9EV[a\x13\x89V[`@\x86\x01a\x1A\x9EV[a\x13\x89V[``\x83\x01a\x1A\x9EV[a+\x15`\x04a+\x0F`\x02a\x0F\x15V[\x90a\x0E\xE6V[a(WV[a+$`\x02a\x0F\x15V[B\x90a+ea+S\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0E\xCAV[\x92a+\\a\x02RV[\x91\x82\x91\x82a\x077V[\x03\x90\xA2_a)\xC0V[a+va\x1FoV[P3\x90V",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AlreadyInitialized()` and selector `0x0dc149f0`.
```solidity
error AlreadyInitialized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyInitialized;
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
                Self
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidModuleAddress()` and selector `0x5cfe78fe`.
```solidity
error InvalidModuleAddress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidModuleAddress;
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
                Self
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `TransactionOrSenderNotAllowed()` and selector `0xdc741458`.
```solidity
error TransactionOrSenderNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TransactionOrSenderNotAllowed;
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
                Self
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                45u8, 156u8, 71u8, 173u8, 85u8, 59u8, 99u8, 187u8, 186u8, 209u8, 129u8,
                157u8, 79u8, 217u8, 125u8, 160u8, 136u8, 80u8, 92u8, 150u8, 165u8, 129u8,
                130u8, 105u8, 27u8, 138u8, 187u8, 95u8, 43u8, 204u8, 41u8, 238u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                65u8, 241u8, 224u8, 143u8, 33u8, 204u8, 129u8, 140u8, 240u8, 207u8,
                251u8, 58u8, 98u8, 96u8, 159u8, 182u8, 163u8, 203u8, 201u8, 179u8, 103u8,
                27u8, 1u8, 30u8, 40u8, 94u8, 23u8, 161u8, 235u8, 180u8, 104u8, 142u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                72u8, 162u8, 152u8, 249u8, 211u8, 118u8, 184u8, 42u8, 113u8, 116u8,
                167u8, 152u8, 233u8, 12u8, 241u8, 32u8, 148u8, 149u8, 253u8, 214u8,
                139u8, 12u8, 17u8, 235u8, 17u8, 190u8, 171u8, 172u8, 194u8, 210u8, 156u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                37u8, 53u8, 128u8, 248u8, 6u8, 116u8, 28u8, 17u8, 179u8, 212u8, 170u8,
                96u8, 217u8, 202u8, 204u8, 91u8, 239u8, 12u8, 235u8, 179u8, 87u8, 72u8,
                118u8, 127u8, 226u8, 63u8, 17u8, 145u8, 110u8, 47u8, 4u8, 185u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                131u8, 54u8, 59u8, 120u8, 189u8, 251u8, 178u8, 62u8, 42u8, 97u8, 219u8,
                122u8, 204u8, 195u8, 192u8, 31u8, 218u8, 41u8, 197u8, 197u8, 236u8,
                129u8, 136u8, 128u8, 3u8, 203u8, 150u8, 41u8, 18u8, 97u8, 138u8, 127u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                75u8, 90u8, 168u8, 208u8, 130u8, 230u8, 145u8, 203u8, 153u8, 114u8,
                167u8, 149u8, 143u8, 164u8, 21u8, 63u8, 102u8, 63u8, 33u8, 95u8, 230u8,
                151u8, 163u8, 224u8, 139u8, 210u8, 114u8, 158u8, 215u8, 143u8, 2u8, 242u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `PERIOD_DURATION()` and selector `0x6558954f`.
```solidity
function PERIOD_DURATION() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PERIOD_DURATIONCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: PERIOD_DURATIONReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: PERIOD_DURATIONReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `PRIORITY_DECAY_RATE()` and selector `0x3d44ae8b`.
```solidity
function PRIORITY_DECAY_RATE() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PRIORITY_DECAY_RATECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: PRIORITY_DECAY_RATEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: PRIORITY_DECAY_RATEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `TRACKING_OVERHEAD()` and selector `0xede07bd6`.
```solidity
function TRACKING_OVERHEAD() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TRACKING_OVERHEADCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: TRACKING_OVERHEADReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: TRACKING_OVERHEADReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `appchainId()` and selector `0xd8781342`.
```solidity
function appchainId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct appchainIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: appchainIdReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: appchainIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: calculateEffectivePriorityReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: calculateEffectivePriorityReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `cumulativeGasFees()` and selector `0xff7b3084`.
```solidity
function cumulativeGasFees() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeGasFeesCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: cumulativeGasFeesReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: cumulativeGasFeesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `currentPeriodIndex()` and selector `0x61543801`.
```solidity
function currentPeriodIndex() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentPeriodIndexCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: currentPeriodIndexReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: currentPeriodIndexReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `disableGasTracking()` and selector `0x5467cb48`.
```solidity
function disableGasTracking() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableGasTrackingCall;
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
                    Self
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
        impl disableGasTrackingReturn {
            fn _tokenize(
                &self,
            ) -> <disableGasTrackingCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                disableGasTrackingReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `enableGasTracking()` and selector `0xde1f453e`.
```solidity
function enableGasTracking() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableGasTrackingCall;
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
                    Self
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
        impl enableGasTrackingReturn {
            fn _tokenize(
                &self,
            ) -> <enableGasTrackingCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                enableGasTrackingReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `gasTrackingEnabled()` and selector `0x84fab62b`.
```solidity
function gasTrackingEnabled() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasTrackingEnabledCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = bool;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: gasTrackingEnabledReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: gasTrackingEnabledReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `gasTrackingInitialized()` and selector `0x3b6ab2a9`.
```solidity
function gasTrackingInitialized() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasTrackingInitializedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = bool;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: gasTrackingInitializedReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: gasTrackingInitializedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCumulativeGasFees()` and selector `0x7fbd295e`.
```solidity
function getCumulativeGasFees() external view returns (uint256 totalCost);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCumulativeGasFeesCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getCumulativeGasFeesReturn = r.into();
                        r.totalCost
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getCumulativeGasFeesReturn = r.into();
                        r.totalCost
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCurrentPeriod()` and selector `0x086146d2`.
```solidity
function getCurrentPeriod() external view returns (GasCounter.GasPeriod memory period);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentPeriodCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = <GasCounter::GasPeriod as alloy::sol_types::SolType>::RustType;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<GasCounter::GasPeriod as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getCurrentPeriodReturn = r.into();
                        r.period
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getCurrentPeriodReturn = r.into();
                        r.period
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCurrentPeriodGasUsed()` and selector `0xc660d3f3`.
```solidity
function getCurrentPeriodGasUsed() external view returns (uint256 totalGas);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentPeriodGasUsedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getCurrentPeriodGasUsedReturn = r.into();
                        r.totalGas
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getCurrentPeriodGasUsedReturn = r.into();
                        r.totalGas
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCurrentPeriodTimeRemaining()` and selector `0x82f44ade`.
```solidity
function getCurrentPeriodTimeRemaining() external view returns (uint256 timeRemaining);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentPeriodTimeRemainingCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getCurrentPeriodTimeRemainingReturn = r.into();
                        r.timeRemaining
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getCurrentPeriodTimeRemainingReturn = r.into();
                        r.timeRemaining
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getGasFeesInRangeReturn = r.into();
                        r.feesDuring
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getGasFeesInRangeReturn = r.into();
                        r.feesDuring
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = <GasCounter::GasPeriod as alloy::sol_types::SolType>::RustType;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<GasCounter::GasPeriod as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getPeriodReturn = r.into();
                        r.period
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getPeriodReturn = r.into();
                        r.period
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getTotalGasFees()` and selector `0x8d5a239b`.
```solidity
function getTotalGasFees() external view returns (uint256 totalCost);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalGasFeesCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getTotalGasFeesReturn = r.into();
                        r.totalCost
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getTotalGasFeesReturn = r.into();
                        r.totalCost
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getTotalPeriods()` and selector `0xaff74c6d`.
```solidity
function getTotalPeriods() external view returns (uint256 totalPeriods);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalPeriodsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getTotalPeriodsReturn = r.into();
                        r.totalPeriods
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getTotalPeriodsReturn = r.into();
                        r.totalPeriods
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        impl initializeReturn {
            fn _tokenize(
                &self,
            ) -> <initializeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = bool;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isAllowedReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: isAllowedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isGasTrackingInitialized()` and selector `0x18d5aafe`.
```solidity
function isGasTrackingInitialized() external view returns (bool initialized);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isGasTrackingInitializedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = bool;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isGasTrackingInitializedReturn = r.into();
                        r.initialized
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: isGasTrackingInitializedReturn = r.into();
                        r.initialized
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::Address;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `periods(uint256)` and selector `0xea4a1104`.
```solidity
function periods(uint256) external view returns (uint256 startTimestamp, uint256 endTimestamp, uint256 totalGasUsed, uint256 totalGasCost);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct periodsCall(pub alloy::sol_types::private::primitives::aliases::U256);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for periodsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
        impl periodsReturn {
            fn _tokenize(
                &self,
            ) -> <periodsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                periodsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `permissionRequirementModule()` and selector `0x5b3cd6e2`.
```solidity
function permissionRequirementModule() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permissionRequirementModuleCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::Address;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: permissionRequirementModuleReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: permissionRequirementModuleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = alloy::sol_types::private::Bytes;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: prependZeroByteReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: prependZeroByteReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        impl processTransaction_0Return {
            fn _tokenize(
                &self,
            ) -> <processTransaction_0Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                processTransaction_0Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        impl processTransaction_1Return {
            fn _tokenize(
                &self,
            ) -> <processTransaction_1Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                processTransaction_1Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        impl processTransactionUncompressed_0Return {
            fn _tokenize(
                &self,
            ) -> <processTransactionUncompressed_0Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                processTransactionUncompressed_0Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        impl processTransactionUncompressed_1Return {
            fn _tokenize(
                &self,
            ) -> <processTransactionUncompressed_1Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                processTransactionUncompressed_1Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        impl processTransactionsBulk_0Return {
            fn _tokenize(
                &self,
            ) -> <processTransactionsBulk_0Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                processTransactionsBulk_0Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        impl processTransactionsBulk_1Return {
            fn _tokenize(
                &self,
            ) -> <processTransactionsBulk_1Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                processTransactionsBulk_1Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall;
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
                    Self
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
        impl renounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <renounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        impl transferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <transferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        impl updateRequirementModuleReturn {
            fn _tokenize(
                &self,
            ) -> <updateRequirementModuleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateRequirementModuleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`SyndicateSequencingChainWithDecayingPriority`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
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
            [127u8, 189u8, 41u8, 94u8],
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
            [247u8, 184u8, 147u8, 94u8],
            [255u8, 123u8, 48u8, 132u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface
    for SyndicateSequencingChainWithDecayingPriorityCalls {
        const NAME: &'static str = "SyndicateSequencingChainWithDecayingPriorityCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 35usize;
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
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<
                SyndicateSequencingChainWithDecayingPriorityCalls,
            >] = &[
                {
                    fn processTransactionUncompressed_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransactionUncompressed_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getCurrentPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransactionsBulk_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <isGasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransaction_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <prependZeroByteCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <gasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <PRIORITY_DECAY_RATECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransaction_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::getPeriod,
                            )
                    }
                    getPeriod
                },
                {
                    fn disableGasTracking(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <disableGasTrackingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <permissionRequirementModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <currentPeriodIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <isAllowedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::isAllowed,
                            )
                    }
                    isAllowed
                },
                {
                    fn getCumulativeGasFees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getCumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransactionUncompressed_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getCurrentPeriodTimeRemainingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <calculateEffectivePriorityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <gasTrackingEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getTotalGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::owner,
                            )
                    }
                    owner
                },
                {
                    fn getTotalPeriods(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getTotalPeriodsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getCurrentPeriodGasUsedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransactionsBulk_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <updateRequirementModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <appchainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <enableGasTrackingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <periodsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(
                                SyndicateSequencingChainWithDecayingPriorityCalls::periods,
                            )
                    }
                    periods
                },
                {
                    fn TRACKING_OVERHEAD(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <TRACKING_OVERHEADCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getGasFeesInRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <cumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<
                SyndicateSequencingChainWithDecayingPriorityCalls,
            >] = &[
                {
                    fn processTransactionUncompressed_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransactionUncompressed_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getCurrentPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransactionsBulk_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <isGasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransaction_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <prependZeroByteCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <gasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <PRIORITY_DECAY_RATECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransaction_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <disableGasTrackingCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <permissionRequirementModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <currentPeriodIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <isAllowedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getCumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransactionUncompressed_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getCurrentPeriodTimeRemainingCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <calculateEffectivePriorityCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <gasTrackingEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getTotalGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getTotalPeriodsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getCurrentPeriodGasUsedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <processTransactionsBulk_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <updateRequirementModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <appchainIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <enableGasTrackingCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <periodsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <TRACKING_OVERHEADCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <getGasFeesInRangeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityCalls,
                    > {
                        <cumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
            DECODE_VALIDATE_SHIMS[idx](data)
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
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
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<
                SyndicateSequencingChainWithDecayingPriorityErrors,
            >] = &[
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <InvalidModuleAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <TransactionOrSenderNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
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
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<
                SyndicateSequencingChainWithDecayingPriorityErrors,
            >] = &[
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <InvalidModuleAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<
                        SyndicateSequencingChainWithDecayingPriorityErrors,
                    > {
                        <TransactionOrSenderNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
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
            DECODE_VALIDATE_SHIMS[idx](data)
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
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
                37u8, 53u8, 128u8, 248u8, 6u8, 116u8, 28u8, 17u8, 179u8, 212u8, 170u8,
                96u8, 217u8, 202u8, 204u8, 91u8, 239u8, 12u8, 235u8, 179u8, 87u8, 72u8,
                118u8, 127u8, 226u8, 63u8, 17u8, 145u8, 110u8, 47u8, 4u8, 185u8,
            ],
            [
                45u8, 156u8, 71u8, 173u8, 85u8, 59u8, 99u8, 187u8, 186u8, 209u8, 129u8,
                157u8, 79u8, 217u8, 125u8, 160u8, 136u8, 80u8, 92u8, 150u8, 165u8, 129u8,
                130u8, 105u8, 27u8, 138u8, 187u8, 95u8, 43u8, 204u8, 41u8, 238u8,
            ],
            [
                65u8, 241u8, 224u8, 143u8, 33u8, 204u8, 129u8, 140u8, 240u8, 207u8,
                251u8, 58u8, 98u8, 96u8, 159u8, 182u8, 163u8, 203u8, 201u8, 179u8, 103u8,
                27u8, 1u8, 30u8, 40u8, 94u8, 23u8, 161u8, 235u8, 180u8, 104u8, 142u8,
            ],
            [
                72u8, 162u8, 152u8, 249u8, 211u8, 118u8, 184u8, 42u8, 113u8, 116u8,
                167u8, 152u8, 233u8, 12u8, 241u8, 32u8, 148u8, 149u8, 253u8, 214u8,
                139u8, 12u8, 17u8, 235u8, 17u8, 190u8, 171u8, 172u8, 194u8, 210u8, 156u8,
                245u8,
            ],
            [
                75u8, 90u8, 168u8, 208u8, 130u8, 230u8, 145u8, 203u8, 153u8, 114u8,
                167u8, 149u8, 143u8, 164u8, 21u8, 63u8, 102u8, 63u8, 33u8, 95u8, 230u8,
                151u8, 163u8, 224u8, 139u8, 210u8, 114u8, 158u8, 215u8, 143u8, 2u8, 242u8,
            ],
            [
                131u8, 54u8, 59u8, 120u8, 189u8, 251u8, 178u8, 62u8, 42u8, 97u8, 219u8,
                122u8, 204u8, 195u8, 192u8, 31u8, 218u8, 41u8, 197u8, 197u8, 236u8,
                129u8, 136u8, 128u8, 3u8, 203u8, 150u8, 41u8, 18u8, 97u8, 138u8, 127u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
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
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<GasTracked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <GasTracked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::GasTracked)
                }
                Some(<NewPeriodStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewPeriodStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::NewPeriodStarted)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<PeriodFinalized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PeriodFinalized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PeriodFinalized)
                }
                Some(
                    <RequirementModuleUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RequirementModuleUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RequirementModuleUpdated)
                }
                Some(
                    <TransactionProcessed_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionProcessed_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::TransactionProcessed_0)
                }
                Some(
                    <TransactionProcessed_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionProcessed_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
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
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SyndicateSequencingChainWithDecayingPriorityInstance<P, N> {
        SyndicateSequencingChainWithDecayingPriorityInstance::<
            P,
            N,
        >::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _appchainId: alloy::sol_types::private::primitives::aliases::U256,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<
            SyndicateSequencingChainWithDecayingPriorityInstance<P, N>,
        >,
    > {
        SyndicateSequencingChainWithDecayingPriorityInstance::<
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
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _appchainId: alloy::sol_types::private::primitives::aliases::U256,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        SyndicateSequencingChainWithDecayingPriorityInstance::<
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
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug
    for SyndicateSequencingChainWithDecayingPriorityInstance<P, N> {
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
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SyndicateSequencingChainWithDecayingPriorityInstance<P, N> {
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
                _network: ::core::marker::PhantomData,
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
            SyndicateSequencingChainWithDecayingPriorityInstance<P, N>,
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
        ) -> alloy_contract::RawCallBuilder<P, N> {
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
        P: ::core::clone::Clone,
        N,
    > SyndicateSequencingChainWithDecayingPriorityInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> SyndicateSequencingChainWithDecayingPriorityInstance<P, N> {
            SyndicateSequencingChainWithDecayingPriorityInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SyndicateSequencingChainWithDecayingPriorityInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`PERIOD_DURATION`] function.
        pub fn PERIOD_DURATION(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, PERIOD_DURATIONCall, N> {
            self.call_builder(&PERIOD_DURATIONCall)
        }
        ///Creates a new call builder for the [`PRIORITY_DECAY_RATE`] function.
        pub fn PRIORITY_DECAY_RATE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, PRIORITY_DECAY_RATECall, N> {
            self.call_builder(&PRIORITY_DECAY_RATECall)
        }
        ///Creates a new call builder for the [`TRACKING_OVERHEAD`] function.
        pub fn TRACKING_OVERHEAD(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, TRACKING_OVERHEADCall, N> {
            self.call_builder(&TRACKING_OVERHEADCall)
        }
        ///Creates a new call builder for the [`appchainId`] function.
        pub fn appchainId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, appchainIdCall, N> {
            self.call_builder(&appchainIdCall)
        }
        ///Creates a new call builder for the [`calculateEffectivePriority`] function.
        pub fn calculateEffectivePriority(
            &self,
            originalPriority: alloy::sol_types::private::primitives::aliases::U256,
            submittedTimestamp: alloy::sol_types::private::primitives::aliases::U256,
            currentTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, calculateEffectivePriorityCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<&P, cumulativeGasFeesCall, N> {
            self.call_builder(&cumulativeGasFeesCall)
        }
        ///Creates a new call builder for the [`currentPeriodIndex`] function.
        pub fn currentPeriodIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, currentPeriodIndexCall, N> {
            self.call_builder(&currentPeriodIndexCall)
        }
        ///Creates a new call builder for the [`disableGasTracking`] function.
        pub fn disableGasTracking(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, disableGasTrackingCall, N> {
            self.call_builder(&disableGasTrackingCall)
        }
        ///Creates a new call builder for the [`enableGasTracking`] function.
        pub fn enableGasTracking(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, enableGasTrackingCall, N> {
            self.call_builder(&enableGasTrackingCall)
        }
        ///Creates a new call builder for the [`gasTrackingEnabled`] function.
        pub fn gasTrackingEnabled(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, gasTrackingEnabledCall, N> {
            self.call_builder(&gasTrackingEnabledCall)
        }
        ///Creates a new call builder for the [`gasTrackingInitialized`] function.
        pub fn gasTrackingInitialized(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, gasTrackingInitializedCall, N> {
            self.call_builder(&gasTrackingInitializedCall)
        }
        ///Creates a new call builder for the [`getCumulativeGasFees`] function.
        pub fn getCumulativeGasFees(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCumulativeGasFeesCall, N> {
            self.call_builder(&getCumulativeGasFeesCall)
        }
        ///Creates a new call builder for the [`getCurrentPeriod`] function.
        pub fn getCurrentPeriod(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentPeriodCall, N> {
            self.call_builder(&getCurrentPeriodCall)
        }
        ///Creates a new call builder for the [`getCurrentPeriodGasUsed`] function.
        pub fn getCurrentPeriodGasUsed(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentPeriodGasUsedCall, N> {
            self.call_builder(&getCurrentPeriodGasUsedCall)
        }
        ///Creates a new call builder for the [`getCurrentPeriodTimeRemaining`] function.
        pub fn getCurrentPeriodTimeRemaining(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentPeriodTimeRemainingCall, N> {
            self.call_builder(&getCurrentPeriodTimeRemainingCall)
        }
        ///Creates a new call builder for the [`getGasFeesInRange`] function.
        pub fn getGasFeesInRange(
            &self,
            startCumulative: alloy::sol_types::private::primitives::aliases::U256,
            endCumulative: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getGasFeesInRangeCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<&P, getPeriodCall, N> {
            self.call_builder(&getPeriodCall { periodIndex })
        }
        ///Creates a new call builder for the [`getTotalGasFees`] function.
        pub fn getTotalGasFees(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getTotalGasFeesCall, N> {
            self.call_builder(&getTotalGasFeesCall)
        }
        ///Creates a new call builder for the [`getTotalPeriods`] function.
        pub fn getTotalPeriods(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getTotalPeriodsCall, N> {
            self.call_builder(&getTotalPeriodsCall)
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            admin: alloy::sol_types::private::Address,
            _permissionRequirementModule: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<&P, isAllowedCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<&P, isGasTrackingInitializedCall, N> {
            self.call_builder(&isGasTrackingInitializedCall)
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`periods`] function.
        pub fn periods(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, periodsCall, N> {
            self.call_builder(&periodsCall(_0))
        }
        ///Creates a new call builder for the [`permissionRequirementModule`] function.
        pub fn permissionRequirementModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, permissionRequirementModuleCall, N> {
            self.call_builder(&permissionRequirementModuleCall)
        }
        ///Creates a new call builder for the [`prependZeroByte`] function.
        pub fn prependZeroByte(
            &self,
            _data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, prependZeroByteCall, N> {
            self.call_builder(&prependZeroByteCall { _data })
        }
        ///Creates a new call builder for the [`processTransaction_0`] function.
        pub fn processTransaction_0(
            &self,
            data: alloy::sol_types::private::Bytes,
            priority: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, processTransaction_0Call, N> {
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
        ) -> alloy_contract::SolCallBuilder<&P, processTransaction_1Call, N> {
            self.call_builder(&processTransaction_1Call { data })
        }
        ///Creates a new call builder for the [`processTransactionUncompressed_0`] function.
        pub fn processTransactionUncompressed_0(
            &self,
            data: alloy::sol_types::private::Bytes,
            priority: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
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
        ) -> alloy_contract::SolCallBuilder<&P, processTransactionsBulk_0Call, N> {
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
        ) -> alloy_contract::SolCallBuilder<&P, processTransactionsBulk_1Call, N> {
            self.call_builder(
                &processTransactionsBulk_1Call {
                    data,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`updateRequirementModule`] function.
        pub fn updateRequirementModule(
            &self,
            _newModule: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, updateRequirementModuleCall, N> {
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
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SyndicateSequencingChainWithDecayingPriorityInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`GasTracked`] event.
        pub fn GasTracked_filter(&self) -> alloy_contract::Event<&P, GasTracked, N> {
            self.event_filter::<GasTracked>()
        }
        ///Creates a new event filter for the [`NewPeriodStarted`] event.
        pub fn NewPeriodStarted_filter(
            &self,
        ) -> alloy_contract::Event<&P, NewPeriodStarted, N> {
            self.event_filter::<NewPeriodStarted>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`PeriodFinalized`] event.
        pub fn PeriodFinalized_filter(
            &self,
        ) -> alloy_contract::Event<&P, PeriodFinalized, N> {
            self.event_filter::<PeriodFinalized>()
        }
        ///Creates a new event filter for the [`RequirementModuleUpdated`] event.
        pub fn RequirementModuleUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, RequirementModuleUpdated, N> {
            self.event_filter::<RequirementModuleUpdated>()
        }
        ///Creates a new event filter for the [`TransactionProcessed_0`] event.
        pub fn TransactionProcessed_0_filter(
            &self,
        ) -> alloy_contract::Event<&P, TransactionProcessed_0, N> {
            self.event_filter::<TransactionProcessed_0>()
        }
        ///Creates a new event filter for the [`TransactionProcessed_1`] event.
        pub fn TransactionProcessed_1_filter(
            &self,
        ) -> alloy_contract::Event<&P, TransactionProcessed_1, N> {
            self.event_filter::<TransactionProcessed_1>()
        }
    }
}
