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

interface SyndicateSequencingChain {
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

    constructor(uint256 _appchainId);

    function PERIOD_DURATION() external view returns (uint256);
    function TRACKING_OVERHEAD() external view returns (uint256);
    function appchainId() external view returns (uint256);
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
    function processTransaction(bytes memory data) external;
    function processTransactionUncompressed(bytes memory data) external;
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
pub mod SyndicateSequencingChain {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a060405234610038576100196100146100e9565b6101b7565b61002161003d565b6124d6610516823960805181610b9201526124d690f35b610043565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b9061006f90610047565b810190811060018060401b0382111761008757604052565b610051565b9061009f61009861003d565b9283610065565b565b5f80fd5b90565b6100b1816100a5565b036100b857565b5f80fd5b905051906100c9826100a8565b565b906020828203126100e4576100e1915f016100bc565b90565b6100a1565b610107612bab803803806100fc8161008c565b9283398101906100cb565b90565b90565b90565b61012461011f6101299261010a565b61010d565b6100a5565b90565b60209181520190565b5f7f41707020636861696e2049442063616e6e6f7420626520300000000000000000910152565b610169601860209261012c565b61017281610135565b0190565b61018b9060208101905f81830391015261015c565b90565b1561019557565b61019d61003d565b62461bcd60e51b8152806101b360048201610176565b0390fd5b6101bf610232565b6101dc816101d56101cf5f610110565b916100a5565b141561018e565b608052565b60081b90565b906101f461ff00916101e1565b9181191691161790565b151590565b61020c906101fe565b90565b90565b9061022761022261022e92610203565b61020f565b82546101e7565b9055565b61023a61032a565b61024660016003610212565b565b60a01b90565b9061025d60ff60a01b91610248565b9181191691161790565b9061027c61027761028392610203565b61020f565b825461024e565b9055565b5f0190565b61029461003d565b3d5f823e3d90fd5b60018060a01b031690565b6102bb6102b66102c09261029c565b61010d565b61029c565b90565b6102cc906102a7565b90565b6102d8906102c3565b90565b5f1b90565b906102f160018060a01b03916102db565b9181191691161790565b610304906102c3565b90565b90565b9061031f61031a610326926102fb565b610307565b82546102e0565b9055565b61033333610397565b61033e5f6001610267565b61034661003d565b6101bf810181811060018060401b038211176103925761036e82916101bf6129ec8439610287565b03905ff0801561038d5761038461038b916102cf565b600161030a565b565b61028c565b610051565b6103a0906103f8565b565b6103b66103b16103bb9261010a565b61010d565b61029c565b90565b6103c7906103a2565b90565b6103d39061029c565b90565b6103df906103ca565b9052565b91906103f6905f602085019401906103d6565b565b8061041361040d6104085f6103be565b6103ca565b916103ca565b1461042357610421906104b6565b565b61044661042f5f6103be565b5f918291631e4fbdf760e01b8352600483016103e3565b0390fd5b5f1c90565b60018060a01b031690565b61046661046b9161044a565b61044f565b90565b610478905461045a565b90565b610484906102a7565b90565b6104909061047b565b90565b90565b906104ab6104a66104b292610487565b610493565b82546102e0565b9055565b6104bf5f61046e565b6104c9825f610496565b906104fd6104f77f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093610487565b91610487565b9161050661003d565b8061051081610287565b0390a356fe60806040526004361015610013575b610e72565b61001d5f356101fc565b8063086146d2146101f757806318d5aafe146101f2578063366cbab7146101ed5780633b6ab2a9146101e857806346e2cc09146101e3578063485cc955146101de5780634b2c0706146101d95780635467cb48146101d45780635b3cd6e2146101cf57806361543801146101ca5780636558954f146101c5578063715018a6146101c05780637a3979dc146101bb5780637fbd295e146101b6578063804e5123146101b157806382f44ade146101ac57806384fab62b146101a75780638d5a239b146101a25780638da5cb5b1461019d578063aff74c6d14610198578063c660d3f314610193578063cdafb9781461018e578063d4f0eb4d14610189578063d878134214610184578063de1f453e1461017f578063ea4a11041461017a578063ede07bd614610175578063f2fde38b14610170578063f7b8935e1461016b5763ff7b30840361000e57610e3d565b610df8565b610d98565b610d63565b610cf2565b610be9565b610bb4565b610b5d565b610b0b565b610a61565b610a2c565b6109f7565b6109a0565b61096b565b610926565b6108f2565b6108bd565b610884565b6107ff565b6107ca565b61075c565b6106cd565b610601565b6105cc565b610557565b6104bc565b610482565b61040d565b6102e8565b61028c565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f91031261021a57565b61020c565b90565b61022b9061021f565b9052565b90606080610275936102475f8201515f860190610222565b61025960208201516020860190610222565b61026b60408201516040860190610222565b0151910190610222565b565b919061028a905f6080850194019061022f565b565b346102bc5761029c366004610210565b6102b86102a7610f22565b6102af610202565b91829182610277565b0390f35b610208565b151590565b6102cf906102c1565b9052565b91906102e6905f602085019401906102c6565b565b34610318576102f8366004610210565b610314610303610f5b565b61030b610202565b918291826102d3565b0390f35b610208565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156103675781359167ffffffffffffffff831161036257602001926001830284011161035d57565b610329565b610325565b610321565b9060208282031261039d575f82013567ffffffffffffffff811161039857610394920161032d565b9091565b61031d565b61020c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6103e36103ec6020936103f1936103da816103a2565b938480936103a6565b958691016103af565b6103ba565b0190565b61040a9160208201915f8184039101526103c4565b90565b3461043e5761043a61042961042336600461036c565b90611002565b610431610202565b918291826103f5565b0390f35b610208565b1c90565b60ff1690565b61045d9060086104629302610443565b610447565b90565b90610470915461044d565b90565b61047f60035f90610465565b90565b346104b257610492366004610210565b6104ae61049d610473565b6104a5610202565b918291826102d3565b0390f35b610208565b5f0190565b346104eb576104d56104cf36600461036c565b906111f3565b6104dd610202565b806104e7816104b7565b0390f35b610208565b60018060a01b031690565b610504906104f0565b90565b610510816104fb565b0361051757565b5f80fd5b9050359061052882610507565b565b9190604083820312610552578061054661054f925f860161051b565b9360200161051b565b90565b61020c565b346105865761057061056a36600461052a565b906113a4565b610578610202565b80610582816104b7565b0390f35b610208565b6105948161021f565b0361059b57565b5f80fd5b905035906105ac8261058b565b565b906020828203126105c7576105c4915f0161059f565b90565b61020c565b346105fc576105f86105e76105e23660046105ae565b611433565b6105ef610202565b91829182610277565b0390f35b610208565b3461062f57610611366004610210565b61061961146e565b610621610202565b8061062b816104b7565b0390f35b610208565b60018060a01b031690565b61064f9060086106549302610443565b610634565b90565b90610662915461063f565b90565b61067160015f90610657565b90565b90565b61068b610686610690926104f0565b610674565b6104f0565b90565b61069c90610677565b90565b6106a890610693565b90565b6106b49061069f565b9052565b91906106cb905f602085019401906106ab565b565b346106fd576106dd366004610210565b6106f96106e8610665565b6106f0610202565b918291826106b8565b0390f35b610208565b90565b61071590600861071a9302610443565b610702565b90565b906107289154610705565b90565b61073760025f9061071d565b90565b6107439061021f565b9052565b919061075a905f6020850194019061073a565b565b3461078c5761076c366004610210565b61078861077761072b565b61077f610202565b91829182610747565b0390f35b610208565b90565b6107a86107a36107ad92610791565b610674565b61021f565b90565b6107bc62278d00610794565b90565b6107c76107b0565b90565b346107fa576107da366004610210565b6107f66107e56107bf565b6107ed610202565b91829182610747565b0390f35b610208565b3461082d5761080f366004610210565b61081761149d565b61081f610202565b80610829816104b7565b0390f35b610208565b9160608383031261087f57610849825f850161051b565b92610857836020830161051b565b92604082013567ffffffffffffffff811161087a57610876920161032d565b9091565b61031d565b61020c565b346108b8576108b46108a361089a366004610832565b92919091611555565b6108ab610202565b918291826102d3565b0390f35b610208565b346108ed576108cd366004610210565b6108e96108d86115fd565b6108e0610202565b91829182610747565b0390f35b610208565b346109215761090b61090536600461036c565b9061171b565b610913610202565b8061091d816104b7565b0390f35b610208565b3461095657610936366004610210565b610952610941611743565b610949610202565b91829182610747565b0390f35b610208565b6109686003600190610465565b90565b3461099b5761097b366004610210565b61099761098661095b565b61098e610202565b918291826102d3565b0390f35b610208565b346109d0576109b0366004610210565b6109cc6109bb6117d3565b6109c3610202565b91829182610747565b0390f35b610208565b6109de906104fb565b9052565b91906109f5905f602085019401906109d5565b565b34610a2757610a07366004610210565b610a23610a12611822565b610a1a610202565b918291826109e2565b0390f35b610208565b34610a5c57610a3c366004610210565b610a58610a47611856565b610a4f610202565b91829182610747565b0390f35b610208565b34610a9157610a71366004610210565b610a8d610a7c6118a2565b610a84610202565b91829182610747565b0390f35b610208565b909182601f83011215610ad05781359167ffffffffffffffff8311610acb576020019260208302840111610ac657565b610329565b610325565b610321565b90602082820312610b06575f82013567ffffffffffffffff8111610b0157610afd9201610a96565b9091565b61031d565b61020c565b34610b3a57610b24610b1e366004610ad5565b90611a78565b610b2c610202565b80610b36816104b7565b0390f35b610208565b90602082820312610b5857610b55915f0161051b565b90565b61020c565b34610b8b57610b75610b70366004610b3f565b611b28565b610b7d610202565b80610b87816104b7565b0390f35b610208565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610be457610bc4366004610210565b610be0610bcf610b90565b610bd7610202565b91829182610747565b0390f35b610208565b34610c1757610bf9366004610210565b610c01611b4f565b610c09610202565b80610c13816104b7565b0390f35b610208565b610c30610c2b610c359261021f565b610674565b61021f565b90565b90610c4290610c1c565b5f5260205260405f2090565b5f1c90565b610c5f610c6491610c4e565b610702565b90565b610c719054610c53565b90565b610c7f906004610c38565b90610c8b5f8301610c67565b91610c9860018201610c67565b91610cb16003610caa60028501610c67565b9301610c67565b90565b610ce9610cf094610cdf606094989795610cd5608086019a5f87019061073a565b602085019061073a565b604083019061073a565b019061073a565b565b34610d2657610d22610d0d610d083660046105ae565b610c74565b90610d19949294610202565b94859485610cb4565b0390f35b610208565b90565b610d42610d3d610d4792610d2b565b610674565b61021f565b90565b610d55611388610d2e565b90565b610d60610d4a565b90565b34610d9357610d73366004610210565b610d8f610d7e610d58565b610d86610202565b91829182610747565b0390f35b610208565b34610dc657610db0610dab366004610b3f565b611bbe565b610db8610202565b80610dc2816104b7565b0390f35b610208565b9190604083820312610df35780610de7610df0925f860161059f565b9360200161059f565b90565b61020c565b34610e2957610e25610e14610e0e366004610dcb565b90611c54565b610e1c610202565b91829182610747565b0390f35b610208565b610e3a60055f9061071d565b90565b34610e6d57610e4d366004610210565b610e69610e58610e2e565b610e60610202565b91829182610747565b0390f35b610208565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b90610e94906103ba565b810190811067ffffffffffffffff821117610eae57604052565b610e76565b90610ec6610ebf610202565b9283610e8a565b565b610ed26080610eb3565b90565b5f90565b610ee1610ec8565b90602080808085610ef0610ed5565b815201610efb610ed5565b815201610f06610ed5565b815201610f11610ed5565b81525050565b610f1f610ed9565b90565b610f2a610f17565b50610f33611cfe565b90565b5f90565b610f46610f4b91610c4e565b610447565b90565b610f589054610f3a565b90565b610f63610f36565b50610f6e6003610f4e565b90565b606090565b90565b60ff60f81b1690565b60f81b90565b610f9c610f97610fa192610f76565b610f82565b610f79565b90565b90565b610fb3610fb891610f79565b610fa4565b9052565b905090565b90825f939282370152565b909182610fdc81610fe393610fbc565b8093610fc1565b0190565b80610ff8600192610fff9694610fa7565b0191610fcc565b90565b6110409061100e610f71565b5061103161101b5f610f88565b9193611025610202565b94859360208501610fe7565b60208201810382520382610e8a565b90565b9061105f61105933329085859192909192611555565b156102c1565b61106e5761106c9161110f565b565b5f631b8e828b60e31b815280611086600482016104b7565b0390fd5b60081c90565b61109c6110a19161108a565b610447565b90565b6110ae9054611090565b90565b634e487b7160e01b5f52601160045260245ffd5b6110d46110da9193929361021f565b9261021f565b82039182116110e557565b6110b1565b6110f96110ff9193929361021f565b9261021f565b820180921161110a57565b6110b1565b9061112361111d60036110a4565b156102c1565b611158576111436111569261113c611151935a926111ac565b5a906110c5565b61114b610d4a565b906110ea565b611ed2565b565b611161916111ac565b565b61116c90610693565b90565b9190611189816111828161118e956103a6565b8095610fc1565b6103ba565b0190565b90916111a99260208301925f81850391015261116f565b90565b3390916111d97f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611163565b926111ee6111e5610202565b92839283611192565b0390a2565b906111fd91611043565b565b906112119161120c611fd9565b611317565b565b60a01c90565b61122561122a91611213565b610447565b90565b6112379054611219565b90565b61124e61124961125392610f76565b610674565b6104f0565b90565b61125f9061123a565b90565b60a01b90565b9061127760ff60a01b91611262565b9181191691161790565b61128a906102c1565b90565b90565b906112a56112a06112ac92611281565b61128d565b8254611268565b9055565b6112b990610677565b90565b6112c5906112b0565b90565b5f1b90565b906112de60018060a01b03916112c8565b9181191691161790565b6112f1906112b0565b90565b90565b9061130c611307611313926112e8565b6112f4565b82546112cd565b9055565b611321600161122d565b611389578161134061133a6113355f611256565b6104fb565b916104fb565b1461136d5761136661135f61136b9361135a600180611290565b6112bc565b60016112f7565b611bbe565b565b5f632e7f3c7f60e11b815280611385600482016104b7565b0390fd5b5f62dc149f60e41b8152806113a0600482016104b7565b0390fd5b906113ae916111ff565b565b906113ba9061021f565b9052565b9061142561141c60036113cf610ec8565b946113e66113de5f8301610c67565b5f88016113b0565b6113fe6113f560018301610c67565b602088016113b0565b61141661140d60028301610c67565b604088016113b0565b01610c67565b606084016113b0565b565b611430906113be565b90565b61144a61144f91611442610f17565b506004610c38565b611427565b90565b61145a611fd9565b611462611464565b565b61146c612064565b565b611476611452565b565b611480611fd9565b61148861148a565b565b61149b6114965f611256565b612094565b565b6114a5611478565b565b6114b36114b891610c4e565b610634565b90565b6114c590546114a7565b90565b60e01b90565b6114d7816102c1565b036114de57565b5f80fd5b905051906114ef826114ce565b565b9060208282031261150a57611507915f016114e2565b90565b61020c565b611535611542959394929461152b60608401965f8501906109d5565b60208301906109d5565b604081850391015261116f565b90565b61154d610202565b3d5f823e3d90fd5b9261159860209394611565610f36565b506115a361157b61157660016114bb565b61069f565b93637a3979dc92959761158c610202565b988997889687966114c8565b86526004860161150f565b03915afa9081156115e7575f916115b9575b5090565b6115da915060203d81116115e0575b6115d28183610e8a565b8101906114f1565b5f6115b5565b503d6115c8565b611545565b5f90565b6115fa905161021f565b90565b6116056115ec565b5061162c6116136005610c67565b6116266060611620611cfe565b016115f0565b906110ea565b90565b9061164b61164533329085859192909192611555565b156102c1565b61165a5761165891611676565b565b5f631b8e828b60e31b815280611672600482016104b7565b0390fd5b9061168a61168460036110a4565b156102c1565b6116bf576116aa6116bd926116a36116b8935a926116ca565b5a906110c5565b6116b2610d4a565b906110ea565b611ed2565b565b6116c8916116ca565b565b906116d6903392611002565b906117166117047f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611163565b9261170d610202565b918291826103f5565b0390a2565b906117259161162f565b565b61173b61173661174092610f76565b610674565b61021f565b90565b61174b6115ec565b50611754611cfe565b61175f5f82016115f0565b61177161176b5f611727565b9161021f565b146117c6576117845f61179292016115f0565b61178c6107b0565b906110ea565b426117a561179f8361021f565b9161021f565b10156117b9576117b69042906110c5565b90565b506117c35f611727565b90565b506117d05f611727565b90565b6117db6115ec565b506117ef60606117e9611cfe565b016115f0565b90565b5f90565b60018060a01b031690565b61180d61181291610c4e565b6117f6565b90565b61181f9054611801565b90565b61182a6117f2565b506118345f611815565b90565b90565b61184e61184961185392611837565b610674565b61021f565b90565b61185e6115ec565b5061187261186c6003610f4e565b156102c1565b611896576118936118836002610c67565b61188d600161183a565b906110ea565b90565b61189f5f611727565b90565b6118aa6115ec565b506118be60406118b8611cfe565b016115f0565b90565b906118d56118cf60036110a4565b156102c1565b61190a576118f5611908926118ee611903935a926119af565b5a906110c5565b6118fd610d4a565b906110ea565b611ed2565b565b611913916119af565b565b5090565b6001611925910161021f565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561198a570180359067ffffffffffffffff82116119855760200191600182023603831361198057565b611944565b611940565b61193c565b908210156119aa5760206119a69202810190611948565b9091565b611928565b6119ba818390611915565b916119c36115ec565b506119cd5f611727565b5b806119e16119db8661021f565b9161021f565b1015611a7257611a0f90611a053332906119fd8787869161198f565b929091611555565b611a14575b611919565b6119ce565b33611a2a611a248686859161198f565b90611002565b90611a6a611a587f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611163565b92611a61610202565b918291826103f5565b0390a2611a0a565b50505050565b90611a82916118c1565b565b611a9590611a90611fd9565b611a97565b565b80611ab2611aac611aa75f611256565b6104fb565b916104fb565b14611b0c57611aca611ac3826112bc565b60016112f7565b611af47f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b991611163565b90611afd610202565b80611b07816104b7565b0390a2565b5f632e7f3c7f60e11b815280611b24600482016104b7565b0390fd5b611b3190611a84565b565b611b3b611fd9565b611b43611b45565b565b611b4d6120f3565b565b611b57611b33565b565b611b6a90611b65611fd9565b611b6c565b565b80611b87611b81611b7c5f611256565b6104fb565b916104fb565b14611b9757611b9590612094565b565b611bba611ba35f611256565b5f918291631e4fbdf760e01b8352600483016109e2565b0390fd5b611bc790611b59565b565b60209181520190565b5f7f476173436f756e7465723a20696e76616c69642072616e676500000000000000910152565b611c066019602092611bc9565b611c0f81611bd2565b0190565b611c289060208101905f818303910152611bf9565b90565b15611c3257565b611c3a610202565b62461bcd60e51b815280611c5060048201611c13565b0390fd5b611c8391611c606115ec565b50611c7e81611c77611c718561021f565b9161021f565b1015611c2b565b6110c5565b90565b611c906080610eb3565b90565b634e487b7160e01b5f52601260045260245ffd5b611cb3611cb99161021f565b9161021f565b908115611cc4570490565b611c93565b611cd8611cde9193929361021f565b9261021f565b91611cea83820261021f565b928184041490151715611cf957565b6110b1565b611d06610f17565b50611d1a611d146003610f4e565b156102c1565b611e1657611d3b611d366004611d306002610c67565b90610c38565b611427565b42611d69611d63611d5e611d505f86016115f0565b611d586107b0565b906110ea565b61021f565b9161021f565b1015611d725790565b611dbf90611db9611daa5f611da3611d9542611d8f8488016115f0565b906110c5565b611d9d6107b0565b90611ca7565b93016115f0565b91611db36107b0565b90611cc9565b906110ea565b611e13611e0a5f611e05611dfc5f611df7611dee5f95611de9611de0611c86565b9a5f8c016113b0565b611727565b602089016113b0565b611727565b604086016113b0565b611727565b606083016113b0565b90565b5f611e73611e6a5f611e65611e5c5f611e57611e4e5f95611e49611e41611e3b611c86565b9b611727565b5f8c016113b0565b611727565b602089016113b0565b611727565b604086016113b0565b611727565b606083016113b0565b90565b90611e825f19916112c8565b9181191691161790565b90565b90611ea4611e9f611eab92610c1c565b611e8c565b8254611e76565b9055565b916020611ed0929493611ec960408201965f83019061073a565b019061073a565b565b611ee5611edf60036110a4565b156102c1565b611fd657611efc611ef66003610f4e565b156102c1565b611fc9575b611f096122c9565b611f7a611f17823a90611cc9565b611f4a83611f446002611f346004611f2e83610c67565b90610c38565b0191611f3f83610c67565b6110ea565b90611e8f565b611f746003611f646004611f5e6002610c67565b90610c38565b0191611f6f83610c67565b6110ea565b90611e8f565b611f846002610c67565b3a611faf7f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610c1c565b92611fc4611fbb610202565b92839283611eaf565b0390a2565b611fd16121be565b611f01565b50565b611fe1611822565b611ffa611ff4611fef6124c9565b6104fb565b916104fb565b0361200157565b61202361200c6124c9565b5f91829163118cdaa760e01b8352600483016109e2565b0390fd5b60081b90565b9061203a61ff0091612027565b9181191691161790565b9061205961205461206092611281565b61128d565b825461202d565b9055565b61206f5f6003612044565b565b90565b9061208961208461209092611163565b612071565b82546112cd565b9055565b61209d5f611815565b6120a7825f612074565b906120db6120d57f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093611163565b91611163565b916120e4610202565b806120ee816104b7565b0390a3565b6120ff60016003612044565b565b9061210d60ff916112c8565b9181191691161790565b9061212c61212761213392611281565b61128d565b8254612101565b9055565b9061214190611727565b5f5260205260405f2090565b906121aa606060036121b0946121705f820161216a5f88016115f0565b90611e8f565b61218960018201612183602088016115f0565b90611e8f565b6121a26002820161219c604088016115f0565b90611e8f565b0192016115f0565b90611e8f565b565b906121bc9161214d565b565b6121d16121cb6003610f4e565b156102c1565b6121d8575b565b6121e460016003612117565b6121f76121f05f611727565b6002611e8f565b6122604261224f6122465f6122416122385f61223361222a5f9561222561221c611c86565b9a5f8c016113b0565b611727565b602089016113b0565b611727565b604086016113b0565b611727565b606083016113b0565b61225b60045f90612137565b6121b2565b5f42906122a26122907f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92611727565b92612299610202565b91829182610747565b0390a26121d6565b90565b6122b69061021f565b5f1981146122c45760010190565b6110b1565b6122e66122e160046122db6002610c67565b90610c38565b6122aa565b4261231461230e6123096122fb5f8601610c67565b6123036107b0565b906110ea565b61021f565b9161021f565b101561231e575b50565b61234661233d61232f5f8401610c67565b6123376107b0565b906110ea565b60018301611e8f565b61236e61236761235860038401610c67565b6123626005610c67565b6110ea565b6005611e8f565b6123786002610c67565b6123a561238760028401610c67565b9261239f5f61239860018401610c67565b9201610c67565b906110c5565b6123cf7f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610c1c565b926123e46123db610202565b92839283611eaf565b0390a26124036123fc6123f76002610c67565b6122ad565b6002611e8f565b6124754261245b6124525f61244d6124445f61243f6124365f95612431612428611c86565b9a5f8c016113b0565b611727565b602089016113b0565b611727565b604086016113b0565b611727565b606083016113b0565b612470600461246a6002610c67565b90610c38565b6121b2565b61247f6002610c67565b42906124c06124ae7f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610c1c565b926124b7610202565b91829182610747565b0390a25f61231b565b6124d16117f2565b50339056608060405234601c57600e6020565b61019461002b823961019490f35b6026565b60405190565b5f80fdfe60806040526004361015610013575b61017b565b61001d5f3561002c565b637a3979dc0361000e57610142565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b60018060a01b031690565b61005890610044565b90565b6100648161004f565b0361006b57565b5f80fd5b9050359061007c8261005b565b565b5f80fd5b5f80fd5b5f80fd5b909182601f830112156100c45781359167ffffffffffffffff83116100bf5760200192600183028401116100ba57565b610086565b610082565b61007e565b91606083830312610116576100e0825f850161006f565b926100ee836020830161006f565b92604082013567ffffffffffffffff81116101115761010d920161008a565b9091565b610040565b61003c565b151590565b6101299061011b565b9052565b9190610140905f60208501940190610120565b565b34610176576101726101616101583660046100c9565b92919091610183565b610169610032565b9182918261012d565b0390f35b610038565b5f80fd5b5f90565b5050505061018f61017f565b505f9056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4a\08Wa\0\x19a\0\x14a\0\xE9V[a\x01\xB7V[a\0!a\0=V[a$\xD6a\x05\x16\x829`\x80Q\x81a\x0B\x92\x01Ra$\xD6\x90\xF3[a\0CV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0o\x90a\0GV[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x87W`@RV[a\0QV[\x90a\0\x9Fa\0\x98a\0=V[\x92\x83a\0eV[V[_\x80\xFD[\x90V[a\0\xB1\x81a\0\xA5V[\x03a\0\xB8WV[_\x80\xFD[\x90PQ\x90a\0\xC9\x82a\0\xA8V[V[\x90` \x82\x82\x03\x12a\0\xE4Wa\0\xE1\x91_\x01a\0\xBCV[\x90V[a\0\xA1V[a\x01\x07a+\xAB\x808\x03\x80a\0\xFC\x81a\0\x8CV[\x92\x839\x81\x01\x90a\0\xCBV[\x90V[\x90V[\x90V[a\x01$a\x01\x1Fa\x01)\x92a\x01\nV[a\x01\rV[a\0\xA5V[\x90V[` \x91\x81R\x01\x90V[_\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01i`\x18` \x92a\x01,V[a\x01r\x81a\x015V[\x01\x90V[a\x01\x8B\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x01\\V[\x90V[\x15a\x01\x95WV[a\x01\x9Da\0=V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x01\xB3`\x04\x82\x01a\x01vV[\x03\x90\xFD[a\x01\xBFa\x022V[a\x01\xDC\x81a\x01\xD5a\x01\xCF_a\x01\x10V[\x91a\0\xA5V[\x14\x15a\x01\x8EV[`\x80RV[`\x08\x1B\x90V[\x90a\x01\xF4a\xFF\0\x91a\x01\xE1V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x15\x15\x90V[a\x02\x0C\x90a\x01\xFEV[\x90V[\x90V[\x90a\x02'a\x02\"a\x02.\x92a\x02\x03V[a\x02\x0FV[\x82Ta\x01\xE7V[\x90UV[a\x02:a\x03*V[a\x02F`\x01`\x03a\x02\x12V[V[`\xA0\x1B\x90V[\x90a\x02]`\xFF`\xA0\x1B\x91a\x02HV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x02|a\x02wa\x02\x83\x92a\x02\x03V[a\x02\x0FV[\x82Ta\x02NV[\x90UV[_\x01\x90V[a\x02\x94a\0=V[=_\x82>=\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x02\xBBa\x02\xB6a\x02\xC0\x92a\x02\x9CV[a\x01\rV[a\x02\x9CV[\x90V[a\x02\xCC\x90a\x02\xA7V[\x90V[a\x02\xD8\x90a\x02\xC3V[\x90V[_\x1B\x90V[\x90a\x02\xF1`\x01\x80`\xA0\x1B\x03\x91a\x02\xDBV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x03\x04\x90a\x02\xC3V[\x90V[\x90V[\x90a\x03\x1Fa\x03\x1Aa\x03&\x92a\x02\xFBV[a\x03\x07V[\x82Ta\x02\xE0V[\x90UV[a\x0333a\x03\x97V[a\x03>_`\x01a\x02gV[a\x03Fa\0=V[a\x01\xBF\x81\x01\x81\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\x03\x92Wa\x03n\x82\x91a\x01\xBFa)\xEC\x849a\x02\x87V[\x03\x90_\xF0\x80\x15a\x03\x8DWa\x03\x84a\x03\x8B\x91a\x02\xCFV[`\x01a\x03\nV[V[a\x02\x8CV[a\0QV[a\x03\xA0\x90a\x03\xF8V[V[a\x03\xB6a\x03\xB1a\x03\xBB\x92a\x01\nV[a\x01\rV[a\x02\x9CV[\x90V[a\x03\xC7\x90a\x03\xA2V[\x90V[a\x03\xD3\x90a\x02\x9CV[\x90V[a\x03\xDF\x90a\x03\xCAV[\x90RV[\x91\x90a\x03\xF6\x90_` \x85\x01\x94\x01\x90a\x03\xD6V[V[\x80a\x04\x13a\x04\ra\x04\x08_a\x03\xBEV[a\x03\xCAV[\x91a\x03\xCAV[\x14a\x04#Wa\x04!\x90a\x04\xB6V[V[a\x04Fa\x04/_a\x03\xBEV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x03\xE3V[\x03\x90\xFD[_\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04fa\x04k\x91a\x04JV[a\x04OV[\x90V[a\x04x\x90Ta\x04ZV[\x90V[a\x04\x84\x90a\x02\xA7V[\x90V[a\x04\x90\x90a\x04{V[\x90V[\x90V[\x90a\x04\xABa\x04\xA6a\x04\xB2\x92a\x04\x87V[a\x04\x93V[\x82Ta\x02\xE0V[\x90UV[a\x04\xBF_a\x04nV[a\x04\xC9\x82_a\x04\x96V[\x90a\x04\xFDa\x04\xF7\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x04\x87V[\x91a\x04\x87V[\x91a\x05\x06a\0=V[\x80a\x05\x10\x81a\x02\x87V[\x03\x90\xA3V\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x0ErV[a\0\x1D_5a\x01\xFCV[\x80c\x08aF\xD2\x14a\x01\xF7W\x80c\x18\xD5\xAA\xFE\x14a\x01\xF2W\x80c6l\xBA\xB7\x14a\x01\xEDW\x80c;j\xB2\xA9\x14a\x01\xE8W\x80cF\xE2\xCC\t\x14a\x01\xE3W\x80cH\\\xC9U\x14a\x01\xDEW\x80cK,\x07\x06\x14a\x01\xD9W\x80cTg\xCBH\x14a\x01\xD4W\x80c[<\xD6\xE2\x14a\x01\xCFW\x80caT8\x01\x14a\x01\xCAW\x80ceX\x95O\x14a\x01\xC5W\x80cqP\x18\xA6\x14a\x01\xC0W\x80cz9y\xDC\x14a\x01\xBBW\x80c\x7F\xBD)^\x14a\x01\xB6W\x80c\x80NQ#\x14a\x01\xB1W\x80c\x82\xF4J\xDE\x14a\x01\xACW\x80c\x84\xFA\xB6+\x14a\x01\xA7W\x80c\x8DZ#\x9B\x14a\x01\xA2W\x80c\x8D\xA5\xCB[\x14a\x01\x9DW\x80c\xAF\xF7Lm\x14a\x01\x98W\x80c\xC6`\xD3\xF3\x14a\x01\x93W\x80c\xCD\xAF\xB9x\x14a\x01\x8EW\x80c\xD4\xF0\xEBM\x14a\x01\x89W\x80c\xD8x\x13B\x14a\x01\x84W\x80c\xDE\x1FE>\x14a\x01\x7FW\x80c\xEAJ\x11\x04\x14a\x01zW\x80c\xED\xE0{\xD6\x14a\x01uW\x80c\xF2\xFD\xE3\x8B\x14a\x01pW\x80c\xF7\xB8\x93^\x14a\x01kWc\xFF{0\x84\x03a\0\x0EWa\x0E=V[a\r\xF8V[a\r\x98V[a\rcV[a\x0C\xF2V[a\x0B\xE9V[a\x0B\xB4V[a\x0B]V[a\x0B\x0BV[a\naV[a\n,V[a\t\xF7V[a\t\xA0V[a\tkV[a\t&V[a\x08\xF2V[a\x08\xBDV[a\x08\x84V[a\x07\xFFV[a\x07\xCAV[a\x07\\V[a\x06\xCDV[a\x06\x01V[a\x05\xCCV[a\x05WV[a\x04\xBCV[a\x04\x82V[a\x04\rV[a\x02\xE8V[a\x02\x8CV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x02\x1AWV[a\x02\x0CV[\x90V[a\x02+\x90a\x02\x1FV[\x90RV[\x90``\x80a\x02u\x93a\x02G_\x82\x01Q_\x86\x01\x90a\x02\"V[a\x02Y` \x82\x01Q` \x86\x01\x90a\x02\"V[a\x02k`@\x82\x01Q`@\x86\x01\x90a\x02\"V[\x01Q\x91\x01\x90a\x02\"V[V[\x91\x90a\x02\x8A\x90_`\x80\x85\x01\x94\x01\x90a\x02/V[V[4a\x02\xBCWa\x02\x9C6`\x04a\x02\x10V[a\x02\xB8a\x02\xA7a\x0F\"V[a\x02\xAFa\x02\x02V[\x91\x82\x91\x82a\x02wV[\x03\x90\xF3[a\x02\x08V[\x15\x15\x90V[a\x02\xCF\x90a\x02\xC1V[\x90RV[\x91\x90a\x02\xE6\x90_` \x85\x01\x94\x01\x90a\x02\xC6V[V[4a\x03\x18Wa\x02\xF86`\x04a\x02\x10V[a\x03\x14a\x03\x03a\x0F[V[a\x03\x0Ba\x02\x02V[\x91\x82\x91\x82a\x02\xD3V[\x03\x90\xF3[a\x02\x08V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03gW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03bW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x03]WV[a\x03)V[a\x03%V[a\x03!V[\x90` \x82\x82\x03\x12a\x03\x9DW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\x98Wa\x03\x94\x92\x01a\x03-V[\x90\x91V[a\x03\x1DV[a\x02\x0CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\xE3a\x03\xEC` \x93a\x03\xF1\x93a\x03\xDA\x81a\x03\xA2V[\x93\x84\x80\x93a\x03\xA6V[\x95\x86\x91\x01a\x03\xAFV[a\x03\xBAV[\x01\x90V[a\x04\n\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xC4V[\x90V[4a\x04>Wa\x04:a\x04)a\x04#6`\x04a\x03lV[\x90a\x10\x02V[a\x041a\x02\x02V[\x91\x82\x91\x82a\x03\xF5V[\x03\x90\xF3[a\x02\x08V[\x1C\x90V[`\xFF\x16\x90V[a\x04]\x90`\x08a\x04b\x93\x02a\x04CV[a\x04GV[\x90V[\x90a\x04p\x91Ta\x04MV[\x90V[a\x04\x7F`\x03_\x90a\x04eV[\x90V[4a\x04\xB2Wa\x04\x926`\x04a\x02\x10V[a\x04\xAEa\x04\x9Da\x04sV[a\x04\xA5a\x02\x02V[\x91\x82\x91\x82a\x02\xD3V[\x03\x90\xF3[a\x02\x08V[_\x01\x90V[4a\x04\xEBWa\x04\xD5a\x04\xCF6`\x04a\x03lV[\x90a\x11\xF3V[a\x04\xDDa\x02\x02V[\x80a\x04\xE7\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x05\x04\x90a\x04\xF0V[\x90V[a\x05\x10\x81a\x04\xFBV[\x03a\x05\x17WV[_\x80\xFD[\x90P5\x90a\x05(\x82a\x05\x07V[V[\x91\x90`@\x83\x82\x03\x12a\x05RW\x80a\x05Fa\x05O\x92_\x86\x01a\x05\x1BV[\x93` \x01a\x05\x1BV[\x90V[a\x02\x0CV[4a\x05\x86Wa\x05pa\x05j6`\x04a\x05*V[\x90a\x13\xA4V[a\x05xa\x02\x02V[\x80a\x05\x82\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[a\x05\x94\x81a\x02\x1FV[\x03a\x05\x9BWV[_\x80\xFD[\x90P5\x90a\x05\xAC\x82a\x05\x8BV[V[\x90` \x82\x82\x03\x12a\x05\xC7Wa\x05\xC4\x91_\x01a\x05\x9FV[\x90V[a\x02\x0CV[4a\x05\xFCWa\x05\xF8a\x05\xE7a\x05\xE26`\x04a\x05\xAEV[a\x143V[a\x05\xEFa\x02\x02V[\x91\x82\x91\x82a\x02wV[\x03\x90\xF3[a\x02\x08V[4a\x06/Wa\x06\x116`\x04a\x02\x10V[a\x06\x19a\x14nV[a\x06!a\x02\x02V[\x80a\x06+\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x06O\x90`\x08a\x06T\x93\x02a\x04CV[a\x064V[\x90V[\x90a\x06b\x91Ta\x06?V[\x90V[a\x06q`\x01_\x90a\x06WV[\x90V[\x90V[a\x06\x8Ba\x06\x86a\x06\x90\x92a\x04\xF0V[a\x06tV[a\x04\xF0V[\x90V[a\x06\x9C\x90a\x06wV[\x90V[a\x06\xA8\x90a\x06\x93V[\x90V[a\x06\xB4\x90a\x06\x9FV[\x90RV[\x91\x90a\x06\xCB\x90_` \x85\x01\x94\x01\x90a\x06\xABV[V[4a\x06\xFDWa\x06\xDD6`\x04a\x02\x10V[a\x06\xF9a\x06\xE8a\x06eV[a\x06\xF0a\x02\x02V[\x91\x82\x91\x82a\x06\xB8V[\x03\x90\xF3[a\x02\x08V[\x90V[a\x07\x15\x90`\x08a\x07\x1A\x93\x02a\x04CV[a\x07\x02V[\x90V[\x90a\x07(\x91Ta\x07\x05V[\x90V[a\x077`\x02_\x90a\x07\x1DV[\x90V[a\x07C\x90a\x02\x1FV[\x90RV[\x91\x90a\x07Z\x90_` \x85\x01\x94\x01\x90a\x07:V[V[4a\x07\x8CWa\x07l6`\x04a\x02\x10V[a\x07\x88a\x07wa\x07+V[a\x07\x7Fa\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[\x90V[a\x07\xA8a\x07\xA3a\x07\xAD\x92a\x07\x91V[a\x06tV[a\x02\x1FV[\x90V[a\x07\xBCb'\x8D\0a\x07\x94V[\x90V[a\x07\xC7a\x07\xB0V[\x90V[4a\x07\xFAWa\x07\xDA6`\x04a\x02\x10V[a\x07\xF6a\x07\xE5a\x07\xBFV[a\x07\xEDa\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[4a\x08-Wa\x08\x0F6`\x04a\x02\x10V[a\x08\x17a\x14\x9DV[a\x08\x1Fa\x02\x02V[\x80a\x08)\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[\x91``\x83\x83\x03\x12a\x08\x7FWa\x08I\x82_\x85\x01a\x05\x1BV[\x92a\x08W\x83` \x83\x01a\x05\x1BV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08zWa\x08v\x92\x01a\x03-V[\x90\x91V[a\x03\x1DV[a\x02\x0CV[4a\x08\xB8Wa\x08\xB4a\x08\xA3a\x08\x9A6`\x04a\x082V[\x92\x91\x90\x91a\x15UV[a\x08\xABa\x02\x02V[\x91\x82\x91\x82a\x02\xD3V[\x03\x90\xF3[a\x02\x08V[4a\x08\xEDWa\x08\xCD6`\x04a\x02\x10V[a\x08\xE9a\x08\xD8a\x15\xFDV[a\x08\xE0a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[4a\t!Wa\t\x0Ba\t\x056`\x04a\x03lV[\x90a\x17\x1BV[a\t\x13a\x02\x02V[\x80a\t\x1D\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[4a\tVWa\t66`\x04a\x02\x10V[a\tRa\tAa\x17CV[a\tIa\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[a\th`\x03`\x01\x90a\x04eV[\x90V[4a\t\x9BWa\t{6`\x04a\x02\x10V[a\t\x97a\t\x86a\t[V[a\t\x8Ea\x02\x02V[\x91\x82\x91\x82a\x02\xD3V[\x03\x90\xF3[a\x02\x08V[4a\t\xD0Wa\t\xB06`\x04a\x02\x10V[a\t\xCCa\t\xBBa\x17\xD3V[a\t\xC3a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[a\t\xDE\x90a\x04\xFBV[\x90RV[\x91\x90a\t\xF5\x90_` \x85\x01\x94\x01\x90a\t\xD5V[V[4a\n'Wa\n\x076`\x04a\x02\x10V[a\n#a\n\x12a\x18\"V[a\n\x1Aa\x02\x02V[\x91\x82\x91\x82a\t\xE2V[\x03\x90\xF3[a\x02\x08V[4a\n\\Wa\n<6`\x04a\x02\x10V[a\nXa\nGa\x18VV[a\nOa\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[4a\n\x91Wa\nq6`\x04a\x02\x10V[a\n\x8Da\n|a\x18\xA2V[a\n\x84a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\n\xD0W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\xCBW` \x01\x92` \x83\x02\x84\x01\x11a\n\xC6WV[a\x03)V[a\x03%V[a\x03!V[\x90` \x82\x82\x03\x12a\x0B\x06W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0B\x01Wa\n\xFD\x92\x01a\n\x96V[\x90\x91V[a\x03\x1DV[a\x02\x0CV[4a\x0B:Wa\x0B$a\x0B\x1E6`\x04a\n\xD5V[\x90a\x1AxV[a\x0B,a\x02\x02V[\x80a\x0B6\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[\x90` \x82\x82\x03\x12a\x0BXWa\x0BU\x91_\x01a\x05\x1BV[\x90V[a\x02\x0CV[4a\x0B\x8BWa\x0Bua\x0Bp6`\x04a\x0B?V[a\x1B(V[a\x0B}a\x02\x02V[\x80a\x0B\x87\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B\xE4Wa\x0B\xC46`\x04a\x02\x10V[a\x0B\xE0a\x0B\xCFa\x0B\x90V[a\x0B\xD7a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[4a\x0C\x17Wa\x0B\xF96`\x04a\x02\x10V[a\x0C\x01a\x1BOV[a\x0C\ta\x02\x02V[\x80a\x0C\x13\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[a\x0C0a\x0C+a\x0C5\x92a\x02\x1FV[a\x06tV[a\x02\x1FV[\x90V[\x90a\x0CB\x90a\x0C\x1CV[_R` R`@_ \x90V[_\x1C\x90V[a\x0C_a\x0Cd\x91a\x0CNV[a\x07\x02V[\x90V[a\x0Cq\x90Ta\x0CSV[\x90V[a\x0C\x7F\x90`\x04a\x0C8V[\x90a\x0C\x8B_\x83\x01a\x0CgV[\x91a\x0C\x98`\x01\x82\x01a\x0CgV[\x91a\x0C\xB1`\x03a\x0C\xAA`\x02\x85\x01a\x0CgV[\x93\x01a\x0CgV[\x90V[a\x0C\xE9a\x0C\xF0\x94a\x0C\xDF``\x94\x98\x97\x95a\x0C\xD5`\x80\x86\x01\x9A_\x87\x01\x90a\x07:V[` \x85\x01\x90a\x07:V[`@\x83\x01\x90a\x07:V[\x01\x90a\x07:V[V[4a\r&Wa\r\"a\r\ra\r\x086`\x04a\x05\xAEV[a\x0CtV[\x90a\r\x19\x94\x92\x94a\x02\x02V[\x94\x85\x94\x85a\x0C\xB4V[\x03\x90\xF3[a\x02\x08V[\x90V[a\rBa\r=a\rG\x92a\r+V[a\x06tV[a\x02\x1FV[\x90V[a\rUa\x13\x88a\r.V[\x90V[a\r`a\rJV[\x90V[4a\r\x93Wa\rs6`\x04a\x02\x10V[a\r\x8Fa\r~a\rXV[a\r\x86a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[4a\r\xC6Wa\r\xB0a\r\xAB6`\x04a\x0B?V[a\x1B\xBEV[a\r\xB8a\x02\x02V[\x80a\r\xC2\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[\x91\x90`@\x83\x82\x03\x12a\r\xF3W\x80a\r\xE7a\r\xF0\x92_\x86\x01a\x05\x9FV[\x93` \x01a\x05\x9FV[\x90V[a\x02\x0CV[4a\x0E)Wa\x0E%a\x0E\x14a\x0E\x0E6`\x04a\r\xCBV[\x90a\x1CTV[a\x0E\x1Ca\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[a\x0E:`\x05_\x90a\x07\x1DV[\x90V[4a\x0EmWa\x0EM6`\x04a\x02\x10V[a\x0Eia\x0EXa\x0E.V[a\x0E`a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x0E\x94\x90a\x03\xBAV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xAEW`@RV[a\x0EvV[\x90a\x0E\xC6a\x0E\xBFa\x02\x02V[\x92\x83a\x0E\x8AV[V[a\x0E\xD2`\x80a\x0E\xB3V[\x90V[_\x90V[a\x0E\xE1a\x0E\xC8V[\x90` \x80\x80\x80\x85a\x0E\xF0a\x0E\xD5V[\x81R\x01a\x0E\xFBa\x0E\xD5V[\x81R\x01a\x0F\x06a\x0E\xD5V[\x81R\x01a\x0F\x11a\x0E\xD5V[\x81RPPV[a\x0F\x1Fa\x0E\xD9V[\x90V[a\x0F*a\x0F\x17V[Pa\x0F3a\x1C\xFEV[\x90V[_\x90V[a\x0FFa\x0FK\x91a\x0CNV[a\x04GV[\x90V[a\x0FX\x90Ta\x0F:V[\x90V[a\x0Fca\x0F6V[Pa\x0Fn`\x03a\x0FNV[\x90V[``\x90V[\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x0F\x9Ca\x0F\x97a\x0F\xA1\x92a\x0FvV[a\x0F\x82V[a\x0FyV[\x90V[\x90V[a\x0F\xB3a\x0F\xB8\x91a\x0FyV[a\x0F\xA4V[\x90RV[\x90P\x90V[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x0F\xDC\x81a\x0F\xE3\x93a\x0F\xBCV[\x80\x93a\x0F\xC1V[\x01\x90V[\x80a\x0F\xF8`\x01\x92a\x0F\xFF\x96\x94a\x0F\xA7V[\x01\x91a\x0F\xCCV[\x90V[a\x10@\x90a\x10\x0Ea\x0FqV[Pa\x101a\x10\x1B_a\x0F\x88V[\x91\x93a\x10%a\x02\x02V[\x94\x85\x93` \x85\x01a\x0F\xE7V[` \x82\x01\x81\x03\x82R\x03\x82a\x0E\x8AV[\x90V[\x90a\x10_a\x10Y32\x90\x85\x85\x91\x92\x90\x91\x92a\x15UV[\x15a\x02\xC1V[a\x10nWa\x10l\x91a\x11\x0FV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x10\x86`\x04\x82\x01a\x04\xB7V[\x03\x90\xFD[`\x08\x1C\x90V[a\x10\x9Ca\x10\xA1\x91a\x10\x8AV[a\x04GV[\x90V[a\x10\xAE\x90Ta\x10\x90V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x10\xD4a\x10\xDA\x91\x93\x92\x93a\x02\x1FV[\x92a\x02\x1FV[\x82\x03\x91\x82\x11a\x10\xE5WV[a\x10\xB1V[a\x10\xF9a\x10\xFF\x91\x93\x92\x93a\x02\x1FV[\x92a\x02\x1FV[\x82\x01\x80\x92\x11a\x11\nWV[a\x10\xB1V[\x90a\x11#a\x11\x1D`\x03a\x10\xA4V[\x15a\x02\xC1V[a\x11XWa\x11Ca\x11V\x92a\x11<a\x11Q\x93Z\x92a\x11\xACV[Z\x90a\x10\xC5V[a\x11Ka\rJV[\x90a\x10\xEAV[a\x1E\xD2V[V[a\x11a\x91a\x11\xACV[V[a\x11l\x90a\x06\x93V[\x90V[\x91\x90a\x11\x89\x81a\x11\x82\x81a\x11\x8E\x95a\x03\xA6V[\x80\x95a\x0F\xC1V[a\x03\xBAV[\x01\x90V[\x90\x91a\x11\xA9\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x11oV[\x90V[3\x90\x91a\x11\xD9\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11cV[\x92a\x11\xEEa\x11\xE5a\x02\x02V[\x92\x83\x92\x83a\x11\x92V[\x03\x90\xA2V[\x90a\x11\xFD\x91a\x10CV[V[\x90a\x12\x11\x91a\x12\x0Ca\x1F\xD9V[a\x13\x17V[V[`\xA0\x1C\x90V[a\x12%a\x12*\x91a\x12\x13V[a\x04GV[\x90V[a\x127\x90Ta\x12\x19V[\x90V[a\x12Na\x12Ia\x12S\x92a\x0FvV[a\x06tV[a\x04\xF0V[\x90V[a\x12_\x90a\x12:V[\x90V[`\xA0\x1B\x90V[\x90a\x12w`\xFF`\xA0\x1B\x91a\x12bV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x12\x8A\x90a\x02\xC1V[\x90V[\x90V[\x90a\x12\xA5a\x12\xA0a\x12\xAC\x92a\x12\x81V[a\x12\x8DV[\x82Ta\x12hV[\x90UV[a\x12\xB9\x90a\x06wV[\x90V[a\x12\xC5\x90a\x12\xB0V[\x90V[_\x1B\x90V[\x90a\x12\xDE`\x01\x80`\xA0\x1B\x03\x91a\x12\xC8V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x12\xF1\x90a\x12\xB0V[\x90V[\x90V[\x90a\x13\x0Ca\x13\x07a\x13\x13\x92a\x12\xE8V[a\x12\xF4V[\x82Ta\x12\xCDV[\x90UV[a\x13!`\x01a\x12-V[a\x13\x89W\x81a\x13@a\x13:a\x135_a\x12VV[a\x04\xFBV[\x91a\x04\xFBV[\x14a\x13mWa\x13fa\x13_a\x13k\x93a\x13Z`\x01\x80a\x12\x90V[a\x12\xBCV[`\x01a\x12\xF7V[a\x1B\xBEV[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x13\x85`\x04\x82\x01a\x04\xB7V[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x13\xA0`\x04\x82\x01a\x04\xB7V[\x03\x90\xFD[\x90a\x13\xAE\x91a\x11\xFFV[V[\x90a\x13\xBA\x90a\x02\x1FV[\x90RV[\x90a\x14%a\x14\x1C`\x03a\x13\xCFa\x0E\xC8V[\x94a\x13\xE6a\x13\xDE_\x83\x01a\x0CgV[_\x88\x01a\x13\xB0V[a\x13\xFEa\x13\xF5`\x01\x83\x01a\x0CgV[` \x88\x01a\x13\xB0V[a\x14\x16a\x14\r`\x02\x83\x01a\x0CgV[`@\x88\x01a\x13\xB0V[\x01a\x0CgV[``\x84\x01a\x13\xB0V[V[a\x140\x90a\x13\xBEV[\x90V[a\x14Ja\x14O\x91a\x14Ba\x0F\x17V[P`\x04a\x0C8V[a\x14'V[\x90V[a\x14Za\x1F\xD9V[a\x14ba\x14dV[V[a\x14la dV[V[a\x14va\x14RV[V[a\x14\x80a\x1F\xD9V[a\x14\x88a\x14\x8AV[V[a\x14\x9Ba\x14\x96_a\x12VV[a \x94V[V[a\x14\xA5a\x14xV[V[a\x14\xB3a\x14\xB8\x91a\x0CNV[a\x064V[\x90V[a\x14\xC5\x90Ta\x14\xA7V[\x90V[`\xE0\x1B\x90V[a\x14\xD7\x81a\x02\xC1V[\x03a\x14\xDEWV[_\x80\xFD[\x90PQ\x90a\x14\xEF\x82a\x14\xCEV[V[\x90` \x82\x82\x03\x12a\x15\nWa\x15\x07\x91_\x01a\x14\xE2V[\x90V[a\x02\x0CV[a\x155a\x15B\x95\x93\x94\x92\x94a\x15+``\x84\x01\x96_\x85\x01\x90a\t\xD5V[` \x83\x01\x90a\t\xD5V[`@\x81\x85\x03\x91\x01Ra\x11oV[\x90V[a\x15Ma\x02\x02V[=_\x82>=\x90\xFD[\x92a\x15\x98` \x93\x94a\x15ea\x0F6V[Pa\x15\xA3a\x15{a\x15v`\x01a\x14\xBBV[a\x06\x9FV[\x93cz9y\xDC\x92\x95\x97a\x15\x8Ca\x02\x02V[\x98\x89\x97\x88\x96\x87\x96a\x14\xC8V[\x86R`\x04\x86\x01a\x15\x0FV[\x03\x91Z\xFA\x90\x81\x15a\x15\xE7W_\x91a\x15\xB9W[P\x90V[a\x15\xDA\x91P` =\x81\x11a\x15\xE0W[a\x15\xD2\x81\x83a\x0E\x8AV[\x81\x01\x90a\x14\xF1V[_a\x15\xB5V[P=a\x15\xC8V[a\x15EV[_\x90V[a\x15\xFA\x90Qa\x02\x1FV[\x90V[a\x16\x05a\x15\xECV[Pa\x16,a\x16\x13`\x05a\x0CgV[a\x16&``a\x16 a\x1C\xFEV[\x01a\x15\xF0V[\x90a\x10\xEAV[\x90V[\x90a\x16Ka\x16E32\x90\x85\x85\x91\x92\x90\x91\x92a\x15UV[\x15a\x02\xC1V[a\x16ZWa\x16X\x91a\x16vV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x16r`\x04\x82\x01a\x04\xB7V[\x03\x90\xFD[\x90a\x16\x8Aa\x16\x84`\x03a\x10\xA4V[\x15a\x02\xC1V[a\x16\xBFWa\x16\xAAa\x16\xBD\x92a\x16\xA3a\x16\xB8\x93Z\x92a\x16\xCAV[Z\x90a\x10\xC5V[a\x16\xB2a\rJV[\x90a\x10\xEAV[a\x1E\xD2V[V[a\x16\xC8\x91a\x16\xCAV[V[\x90a\x16\xD6\x903\x92a\x10\x02V[\x90a\x17\x16a\x17\x04\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11cV[\x92a\x17\ra\x02\x02V[\x91\x82\x91\x82a\x03\xF5V[\x03\x90\xA2V[\x90a\x17%\x91a\x16/V[V[a\x17;a\x176a\x17@\x92a\x0FvV[a\x06tV[a\x02\x1FV[\x90V[a\x17Ka\x15\xECV[Pa\x17Ta\x1C\xFEV[a\x17__\x82\x01a\x15\xF0V[a\x17qa\x17k_a\x17'V[\x91a\x02\x1FV[\x14a\x17\xC6Wa\x17\x84_a\x17\x92\x92\x01a\x15\xF0V[a\x17\x8Ca\x07\xB0V[\x90a\x10\xEAV[Ba\x17\xA5a\x17\x9F\x83a\x02\x1FV[\x91a\x02\x1FV[\x10\x15a\x17\xB9Wa\x17\xB6\x90B\x90a\x10\xC5V[\x90V[Pa\x17\xC3_a\x17'V[\x90V[Pa\x17\xD0_a\x17'V[\x90V[a\x17\xDBa\x15\xECV[Pa\x17\xEF``a\x17\xE9a\x1C\xFEV[\x01a\x15\xF0V[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x18\ra\x18\x12\x91a\x0CNV[a\x17\xF6V[\x90V[a\x18\x1F\x90Ta\x18\x01V[\x90V[a\x18*a\x17\xF2V[Pa\x184_a\x18\x15V[\x90V[\x90V[a\x18Na\x18Ia\x18S\x92a\x187V[a\x06tV[a\x02\x1FV[\x90V[a\x18^a\x15\xECV[Pa\x18ra\x18l`\x03a\x0FNV[\x15a\x02\xC1V[a\x18\x96Wa\x18\x93a\x18\x83`\x02a\x0CgV[a\x18\x8D`\x01a\x18:V[\x90a\x10\xEAV[\x90V[a\x18\x9F_a\x17'V[\x90V[a\x18\xAAa\x15\xECV[Pa\x18\xBE`@a\x18\xB8a\x1C\xFEV[\x01a\x15\xF0V[\x90V[\x90a\x18\xD5a\x18\xCF`\x03a\x10\xA4V[\x15a\x02\xC1V[a\x19\nWa\x18\xF5a\x19\x08\x92a\x18\xEEa\x19\x03\x93Z\x92a\x19\xAFV[Z\x90a\x10\xC5V[a\x18\xFDa\rJV[\x90a\x10\xEAV[a\x1E\xD2V[V[a\x19\x13\x91a\x19\xAFV[V[P\x90V[`\x01a\x19%\x91\x01a\x02\x1FV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x19\x8AW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x19\x85W` \x01\x91`\x01\x82\x026\x03\x83\x13a\x19\x80WV[a\x19DV[a\x19@V[a\x19<V[\x90\x82\x10\x15a\x19\xAAW` a\x19\xA6\x92\x02\x81\x01\x90a\x19HV[\x90\x91V[a\x19(V[a\x19\xBA\x81\x83\x90a\x19\x15V[\x91a\x19\xC3a\x15\xECV[Pa\x19\xCD_a\x17'V[[\x80a\x19\xE1a\x19\xDB\x86a\x02\x1FV[\x91a\x02\x1FV[\x10\x15a\x1ArWa\x1A\x0F\x90a\x1A\x0532\x90a\x19\xFD\x87\x87\x86\x91a\x19\x8FV[\x92\x90\x91a\x15UV[a\x1A\x14W[a\x19\x19V[a\x19\xCEV[3a\x1A*a\x1A$\x86\x86\x85\x91a\x19\x8FV[\x90a\x10\x02V[\x90a\x1Aja\x1AX\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11cV[\x92a\x1Aaa\x02\x02V[\x91\x82\x91\x82a\x03\xF5V[\x03\x90\xA2a\x1A\nV[PPPPV[\x90a\x1A\x82\x91a\x18\xC1V[V[a\x1A\x95\x90a\x1A\x90a\x1F\xD9V[a\x1A\x97V[V[\x80a\x1A\xB2a\x1A\xACa\x1A\xA7_a\x12VV[a\x04\xFBV[\x91a\x04\xFBV[\x14a\x1B\x0CWa\x1A\xCAa\x1A\xC3\x82a\x12\xBCV[`\x01a\x12\xF7V[a\x1A\xF4\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x11cV[\x90a\x1A\xFDa\x02\x02V[\x80a\x1B\x07\x81a\x04\xB7V[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x1B$`\x04\x82\x01a\x04\xB7V[\x03\x90\xFD[a\x1B1\x90a\x1A\x84V[V[a\x1B;a\x1F\xD9V[a\x1BCa\x1BEV[V[a\x1BMa \xF3V[V[a\x1BWa\x1B3V[V[a\x1Bj\x90a\x1Bea\x1F\xD9V[a\x1BlV[V[\x80a\x1B\x87a\x1B\x81a\x1B|_a\x12VV[a\x04\xFBV[\x91a\x04\xFBV[\x14a\x1B\x97Wa\x1B\x95\x90a \x94V[V[a\x1B\xBAa\x1B\xA3_a\x12VV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\t\xE2V[\x03\x90\xFD[a\x1B\xC7\x90a\x1BYV[V[` \x91\x81R\x01\x90V[_\x7FGasCounter: invalid range\0\0\0\0\0\0\0\x91\x01RV[a\x1C\x06`\x19` \x92a\x1B\xC9V[a\x1C\x0F\x81a\x1B\xD2V[\x01\x90V[a\x1C(\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1B\xF9V[\x90V[\x15a\x1C2WV[a\x1C:a\x02\x02V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1CP`\x04\x82\x01a\x1C\x13V[\x03\x90\xFD[a\x1C\x83\x91a\x1C`a\x15\xECV[Pa\x1C~\x81a\x1Cwa\x1Cq\x85a\x02\x1FV[\x91a\x02\x1FV[\x10\x15a\x1C+V[a\x10\xC5V[\x90V[a\x1C\x90`\x80a\x0E\xB3V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\x1C\xB3a\x1C\xB9\x91a\x02\x1FV[\x91a\x02\x1FV[\x90\x81\x15a\x1C\xC4W\x04\x90V[a\x1C\x93V[a\x1C\xD8a\x1C\xDE\x91\x93\x92\x93a\x02\x1FV[\x92a\x02\x1FV[\x91a\x1C\xEA\x83\x82\x02a\x02\x1FV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1C\xF9WV[a\x10\xB1V[a\x1D\x06a\x0F\x17V[Pa\x1D\x1Aa\x1D\x14`\x03a\x0FNV[\x15a\x02\xC1V[a\x1E\x16Wa\x1D;a\x1D6`\x04a\x1D0`\x02a\x0CgV[\x90a\x0C8V[a\x14'V[Ba\x1Dia\x1Dca\x1D^a\x1DP_\x86\x01a\x15\xF0V[a\x1DXa\x07\xB0V[\x90a\x10\xEAV[a\x02\x1FV[\x91a\x02\x1FV[\x10\x15a\x1DrW\x90V[a\x1D\xBF\x90a\x1D\xB9a\x1D\xAA_a\x1D\xA3a\x1D\x95Ba\x1D\x8F\x84\x88\x01a\x15\xF0V[\x90a\x10\xC5V[a\x1D\x9Da\x07\xB0V[\x90a\x1C\xA7V[\x93\x01a\x15\xF0V[\x91a\x1D\xB3a\x07\xB0V[\x90a\x1C\xC9V[\x90a\x10\xEAV[a\x1E\x13a\x1E\n_a\x1E\x05a\x1D\xFC_a\x1D\xF7a\x1D\xEE_\x95a\x1D\xE9a\x1D\xE0a\x1C\x86V[\x9A_\x8C\x01a\x13\xB0V[a\x17'V[` \x89\x01a\x13\xB0V[a\x17'V[`@\x86\x01a\x13\xB0V[a\x17'V[``\x83\x01a\x13\xB0V[\x90V[_a\x1Esa\x1Ej_a\x1Eea\x1E\\_a\x1EWa\x1EN_\x95a\x1EIa\x1EAa\x1E;a\x1C\x86V[\x9Ba\x17'V[_\x8C\x01a\x13\xB0V[a\x17'V[` \x89\x01a\x13\xB0V[a\x17'V[`@\x86\x01a\x13\xB0V[a\x17'V[``\x83\x01a\x13\xB0V[\x90V[\x90a\x1E\x82_\x19\x91a\x12\xC8V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x1E\xA4a\x1E\x9Fa\x1E\xAB\x92a\x0C\x1CV[a\x1E\x8CV[\x82Ta\x1EvV[\x90UV[\x91` a\x1E\xD0\x92\x94\x93a\x1E\xC9`@\x82\x01\x96_\x83\x01\x90a\x07:V[\x01\x90a\x07:V[V[a\x1E\xE5a\x1E\xDF`\x03a\x10\xA4V[\x15a\x02\xC1V[a\x1F\xD6Wa\x1E\xFCa\x1E\xF6`\x03a\x0FNV[\x15a\x02\xC1V[a\x1F\xC9W[a\x1F\ta\"\xC9V[a\x1Fza\x1F\x17\x82:\x90a\x1C\xC9V[a\x1FJ\x83a\x1FD`\x02a\x1F4`\x04a\x1F.\x83a\x0CgV[\x90a\x0C8V[\x01\x91a\x1F?\x83a\x0CgV[a\x10\xEAV[\x90a\x1E\x8FV[a\x1Ft`\x03a\x1Fd`\x04a\x1F^`\x02a\x0CgV[\x90a\x0C8V[\x01\x91a\x1Fo\x83a\x0CgV[a\x10\xEAV[\x90a\x1E\x8FV[a\x1F\x84`\x02a\x0CgV[:a\x1F\xAF\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0C\x1CV[\x92a\x1F\xC4a\x1F\xBBa\x02\x02V[\x92\x83\x92\x83a\x1E\xAFV[\x03\x90\xA2V[a\x1F\xD1a!\xBEV[a\x1F\x01V[PV[a\x1F\xE1a\x18\"V[a\x1F\xFAa\x1F\xF4a\x1F\xEFa$\xC9V[a\x04\xFBV[\x91a\x04\xFBV[\x03a \x01WV[a #a \x0Ca$\xC9V[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\t\xE2V[\x03\x90\xFD[`\x08\x1B\x90V[\x90a :a\xFF\0\x91a 'V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a Ya Ta `\x92a\x12\x81V[a\x12\x8DV[\x82Ta -V[\x90UV[a o_`\x03a DV[V[\x90V[\x90a \x89a \x84a \x90\x92a\x11cV[a qV[\x82Ta\x12\xCDV[\x90UV[a \x9D_a\x18\x15V[a \xA7\x82_a tV[\x90a \xDBa \xD5\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x11cV[\x91a\x11cV[\x91a \xE4a\x02\x02V[\x80a \xEE\x81a\x04\xB7V[\x03\x90\xA3V[a \xFF`\x01`\x03a DV[V[\x90a!\r`\xFF\x91a\x12\xC8V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a!,a!'a!3\x92a\x12\x81V[a\x12\x8DV[\x82Ta!\x01V[\x90UV[\x90a!A\x90a\x17'V[_R` R`@_ \x90V[\x90a!\xAA```\x03a!\xB0\x94a!p_\x82\x01a!j_\x88\x01a\x15\xF0V[\x90a\x1E\x8FV[a!\x89`\x01\x82\x01a!\x83` \x88\x01a\x15\xF0V[\x90a\x1E\x8FV[a!\xA2`\x02\x82\x01a!\x9C`@\x88\x01a\x15\xF0V[\x90a\x1E\x8FV[\x01\x92\x01a\x15\xF0V[\x90a\x1E\x8FV[V[\x90a!\xBC\x91a!MV[V[a!\xD1a!\xCB`\x03a\x0FNV[\x15a\x02\xC1V[a!\xD8W[V[a!\xE4`\x01`\x03a!\x17V[a!\xF7a!\xF0_a\x17'V[`\x02a\x1E\x8FV[a\"`Ba\"Oa\"F_a\"Aa\"8_a\"3a\"*_\x95a\"%a\"\x1Ca\x1C\x86V[\x9A_\x8C\x01a\x13\xB0V[a\x17'V[` \x89\x01a\x13\xB0V[a\x17'V[`@\x86\x01a\x13\xB0V[a\x17'V[``\x83\x01a\x13\xB0V[a\"[`\x04_\x90a!7V[a!\xB2V[_B\x90a\"\xA2a\"\x90\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x17'V[\x92a\"\x99a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xA2a!\xD6V[\x90V[a\"\xB6\x90a\x02\x1FV[_\x19\x81\x14a\"\xC4W`\x01\x01\x90V[a\x10\xB1V[a\"\xE6a\"\xE1`\x04a\"\xDB`\x02a\x0CgV[\x90a\x0C8V[a\"\xAAV[Ba#\x14a#\x0Ea#\ta\"\xFB_\x86\x01a\x0CgV[a#\x03a\x07\xB0V[\x90a\x10\xEAV[a\x02\x1FV[\x91a\x02\x1FV[\x10\x15a#\x1EW[PV[a#Fa#=a#/_\x84\x01a\x0CgV[a#7a\x07\xB0V[\x90a\x10\xEAV[`\x01\x83\x01a\x1E\x8FV[a#na#ga#X`\x03\x84\x01a\x0CgV[a#b`\x05a\x0CgV[a\x10\xEAV[`\x05a\x1E\x8FV[a#x`\x02a\x0CgV[a#\xA5a#\x87`\x02\x84\x01a\x0CgV[\x92a#\x9F_a#\x98`\x01\x84\x01a\x0CgV[\x92\x01a\x0CgV[\x90a\x10\xC5V[a#\xCF\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0C\x1CV[\x92a#\xE4a#\xDBa\x02\x02V[\x92\x83\x92\x83a\x1E\xAFV[\x03\x90\xA2a$\x03a#\xFCa#\xF7`\x02a\x0CgV[a\"\xADV[`\x02a\x1E\x8FV[a$uBa$[a$R_a$Ma$D_a$?a$6_\x95a$1a$(a\x1C\x86V[\x9A_\x8C\x01a\x13\xB0V[a\x17'V[` \x89\x01a\x13\xB0V[a\x17'V[`@\x86\x01a\x13\xB0V[a\x17'V[``\x83\x01a\x13\xB0V[a$p`\x04a$j`\x02a\x0CgV[\x90a\x0C8V[a!\xB2V[a$\x7F`\x02a\x0CgV[B\x90a$\xC0a$\xAE\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0C\x1CV[\x92a$\xB7a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xA2_a#\x1BV[a$\xD1a\x17\xF2V[P3\x90V`\x80`@R4`\x1CW`\x0E` V[a\x01\x94a\0+\x829a\x01\x94\x90\xF3[`&V[`@Q\x90V[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x01{V[a\0\x1D_5a\0,V[cz9y\xDC\x03a\0\x0EWa\x01BV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\0X\x90a\0DV[\x90V[a\0d\x81a\0OV[\x03a\0kWV[_\x80\xFD[\x90P5\x90a\0|\x82a\0[V[V[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\0\xC4W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xBFW` \x01\x92`\x01\x83\x02\x84\x01\x11a\0\xBAWV[a\0\x86V[a\0\x82V[a\0~V[\x91``\x83\x83\x03\x12a\x01\x16Wa\0\xE0\x82_\x85\x01a\0oV[\x92a\0\xEE\x83` \x83\x01a\0oV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x11Wa\x01\r\x92\x01a\0\x8AV[\x90\x91V[a\0@V[a\0<V[\x15\x15\x90V[a\x01)\x90a\x01\x1BV[\x90RV[\x91\x90a\x01@\x90_` \x85\x01\x94\x01\x90a\x01 V[V[4a\x01vWa\x01ra\x01aa\x01X6`\x04a\0\xC9V[\x92\x91\x90\x91a\x01\x83V[a\x01ia\x002V[\x91\x82\x91\x82a\x01-V[\x03\x90\xF3[a\08V[_\x80\xFD[_\x90V[PPPPa\x01\x8Fa\x01\x7FV[P_\x90V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b610e72565b61001d5f356101fc565b8063086146d2146101f757806318d5aafe146101f2578063366cbab7146101ed5780633b6ab2a9146101e857806346e2cc09146101e3578063485cc955146101de5780634b2c0706146101d95780635467cb48146101d45780635b3cd6e2146101cf57806361543801146101ca5780636558954f146101c5578063715018a6146101c05780637a3979dc146101bb5780637fbd295e146101b6578063804e5123146101b157806382f44ade146101ac57806384fab62b146101a75780638d5a239b146101a25780638da5cb5b1461019d578063aff74c6d14610198578063c660d3f314610193578063cdafb9781461018e578063d4f0eb4d14610189578063d878134214610184578063de1f453e1461017f578063ea4a11041461017a578063ede07bd614610175578063f2fde38b14610170578063f7b8935e1461016b5763ff7b30840361000e57610e3d565b610df8565b610d98565b610d63565b610cf2565b610be9565b610bb4565b610b5d565b610b0b565b610a61565b610a2c565b6109f7565b6109a0565b61096b565b610926565b6108f2565b6108bd565b610884565b6107ff565b6107ca565b61075c565b6106cd565b610601565b6105cc565b610557565b6104bc565b610482565b61040d565b6102e8565b61028c565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f91031261021a57565b61020c565b90565b61022b9061021f565b9052565b90606080610275936102475f8201515f860190610222565b61025960208201516020860190610222565b61026b60408201516040860190610222565b0151910190610222565b565b919061028a905f6080850194019061022f565b565b346102bc5761029c366004610210565b6102b86102a7610f22565b6102af610202565b91829182610277565b0390f35b610208565b151590565b6102cf906102c1565b9052565b91906102e6905f602085019401906102c6565b565b34610318576102f8366004610210565b610314610303610f5b565b61030b610202565b918291826102d3565b0390f35b610208565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156103675781359167ffffffffffffffff831161036257602001926001830284011161035d57565b610329565b610325565b610321565b9060208282031261039d575f82013567ffffffffffffffff811161039857610394920161032d565b9091565b61031d565b61020c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6103e36103ec6020936103f1936103da816103a2565b938480936103a6565b958691016103af565b6103ba565b0190565b61040a9160208201915f8184039101526103c4565b90565b3461043e5761043a61042961042336600461036c565b90611002565b610431610202565b918291826103f5565b0390f35b610208565b1c90565b60ff1690565b61045d9060086104629302610443565b610447565b90565b90610470915461044d565b90565b61047f60035f90610465565b90565b346104b257610492366004610210565b6104ae61049d610473565b6104a5610202565b918291826102d3565b0390f35b610208565b5f0190565b346104eb576104d56104cf36600461036c565b906111f3565b6104dd610202565b806104e7816104b7565b0390f35b610208565b60018060a01b031690565b610504906104f0565b90565b610510816104fb565b0361051757565b5f80fd5b9050359061052882610507565b565b9190604083820312610552578061054661054f925f860161051b565b9360200161051b565b90565b61020c565b346105865761057061056a36600461052a565b906113a4565b610578610202565b80610582816104b7565b0390f35b610208565b6105948161021f565b0361059b57565b5f80fd5b905035906105ac8261058b565b565b906020828203126105c7576105c4915f0161059f565b90565b61020c565b346105fc576105f86105e76105e23660046105ae565b611433565b6105ef610202565b91829182610277565b0390f35b610208565b3461062f57610611366004610210565b61061961146e565b610621610202565b8061062b816104b7565b0390f35b610208565b60018060a01b031690565b61064f9060086106549302610443565b610634565b90565b90610662915461063f565b90565b61067160015f90610657565b90565b90565b61068b610686610690926104f0565b610674565b6104f0565b90565b61069c90610677565b90565b6106a890610693565b90565b6106b49061069f565b9052565b91906106cb905f602085019401906106ab565b565b346106fd576106dd366004610210565b6106f96106e8610665565b6106f0610202565b918291826106b8565b0390f35b610208565b90565b61071590600861071a9302610443565b610702565b90565b906107289154610705565b90565b61073760025f9061071d565b90565b6107439061021f565b9052565b919061075a905f6020850194019061073a565b565b3461078c5761076c366004610210565b61078861077761072b565b61077f610202565b91829182610747565b0390f35b610208565b90565b6107a86107a36107ad92610791565b610674565b61021f565b90565b6107bc62278d00610794565b90565b6107c76107b0565b90565b346107fa576107da366004610210565b6107f66107e56107bf565b6107ed610202565b91829182610747565b0390f35b610208565b3461082d5761080f366004610210565b61081761149d565b61081f610202565b80610829816104b7565b0390f35b610208565b9160608383031261087f57610849825f850161051b565b92610857836020830161051b565b92604082013567ffffffffffffffff811161087a57610876920161032d565b9091565b61031d565b61020c565b346108b8576108b46108a361089a366004610832565b92919091611555565b6108ab610202565b918291826102d3565b0390f35b610208565b346108ed576108cd366004610210565b6108e96108d86115fd565b6108e0610202565b91829182610747565b0390f35b610208565b346109215761090b61090536600461036c565b9061171b565b610913610202565b8061091d816104b7565b0390f35b610208565b3461095657610936366004610210565b610952610941611743565b610949610202565b91829182610747565b0390f35b610208565b6109686003600190610465565b90565b3461099b5761097b366004610210565b61099761098661095b565b61098e610202565b918291826102d3565b0390f35b610208565b346109d0576109b0366004610210565b6109cc6109bb6117d3565b6109c3610202565b91829182610747565b0390f35b610208565b6109de906104fb565b9052565b91906109f5905f602085019401906109d5565b565b34610a2757610a07366004610210565b610a23610a12611822565b610a1a610202565b918291826109e2565b0390f35b610208565b34610a5c57610a3c366004610210565b610a58610a47611856565b610a4f610202565b91829182610747565b0390f35b610208565b34610a9157610a71366004610210565b610a8d610a7c6118a2565b610a84610202565b91829182610747565b0390f35b610208565b909182601f83011215610ad05781359167ffffffffffffffff8311610acb576020019260208302840111610ac657565b610329565b610325565b610321565b90602082820312610b06575f82013567ffffffffffffffff8111610b0157610afd9201610a96565b9091565b61031d565b61020c565b34610b3a57610b24610b1e366004610ad5565b90611a78565b610b2c610202565b80610b36816104b7565b0390f35b610208565b90602082820312610b5857610b55915f0161051b565b90565b61020c565b34610b8b57610b75610b70366004610b3f565b611b28565b610b7d610202565b80610b87816104b7565b0390f35b610208565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610be457610bc4366004610210565b610be0610bcf610b90565b610bd7610202565b91829182610747565b0390f35b610208565b34610c1757610bf9366004610210565b610c01611b4f565b610c09610202565b80610c13816104b7565b0390f35b610208565b610c30610c2b610c359261021f565b610674565b61021f565b90565b90610c4290610c1c565b5f5260205260405f2090565b5f1c90565b610c5f610c6491610c4e565b610702565b90565b610c719054610c53565b90565b610c7f906004610c38565b90610c8b5f8301610c67565b91610c9860018201610c67565b91610cb16003610caa60028501610c67565b9301610c67565b90565b610ce9610cf094610cdf606094989795610cd5608086019a5f87019061073a565b602085019061073a565b604083019061073a565b019061073a565b565b34610d2657610d22610d0d610d083660046105ae565b610c74565b90610d19949294610202565b94859485610cb4565b0390f35b610208565b90565b610d42610d3d610d4792610d2b565b610674565b61021f565b90565b610d55611388610d2e565b90565b610d60610d4a565b90565b34610d9357610d73366004610210565b610d8f610d7e610d58565b610d86610202565b91829182610747565b0390f35b610208565b34610dc657610db0610dab366004610b3f565b611bbe565b610db8610202565b80610dc2816104b7565b0390f35b610208565b9190604083820312610df35780610de7610df0925f860161059f565b9360200161059f565b90565b61020c565b34610e2957610e25610e14610e0e366004610dcb565b90611c54565b610e1c610202565b91829182610747565b0390f35b610208565b610e3a60055f9061071d565b90565b34610e6d57610e4d366004610210565b610e69610e58610e2e565b610e60610202565b91829182610747565b0390f35b610208565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b90610e94906103ba565b810190811067ffffffffffffffff821117610eae57604052565b610e76565b90610ec6610ebf610202565b9283610e8a565b565b610ed26080610eb3565b90565b5f90565b610ee1610ec8565b90602080808085610ef0610ed5565b815201610efb610ed5565b815201610f06610ed5565b815201610f11610ed5565b81525050565b610f1f610ed9565b90565b610f2a610f17565b50610f33611cfe565b90565b5f90565b610f46610f4b91610c4e565b610447565b90565b610f589054610f3a565b90565b610f63610f36565b50610f6e6003610f4e565b90565b606090565b90565b60ff60f81b1690565b60f81b90565b610f9c610f97610fa192610f76565b610f82565b610f79565b90565b90565b610fb3610fb891610f79565b610fa4565b9052565b905090565b90825f939282370152565b909182610fdc81610fe393610fbc565b8093610fc1565b0190565b80610ff8600192610fff9694610fa7565b0191610fcc565b90565b6110409061100e610f71565b5061103161101b5f610f88565b9193611025610202565b94859360208501610fe7565b60208201810382520382610e8a565b90565b9061105f61105933329085859192909192611555565b156102c1565b61106e5761106c9161110f565b565b5f631b8e828b60e31b815280611086600482016104b7565b0390fd5b60081c90565b61109c6110a19161108a565b610447565b90565b6110ae9054611090565b90565b634e487b7160e01b5f52601160045260245ffd5b6110d46110da9193929361021f565b9261021f565b82039182116110e557565b6110b1565b6110f96110ff9193929361021f565b9261021f565b820180921161110a57565b6110b1565b9061112361111d60036110a4565b156102c1565b611158576111436111569261113c611151935a926111ac565b5a906110c5565b61114b610d4a565b906110ea565b611ed2565b565b611161916111ac565b565b61116c90610693565b90565b9190611189816111828161118e956103a6565b8095610fc1565b6103ba565b0190565b90916111a99260208301925f81850391015261116f565b90565b3390916111d97f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611163565b926111ee6111e5610202565b92839283611192565b0390a2565b906111fd91611043565b565b906112119161120c611fd9565b611317565b565b60a01c90565b61122561122a91611213565b610447565b90565b6112379054611219565b90565b61124e61124961125392610f76565b610674565b6104f0565b90565b61125f9061123a565b90565b60a01b90565b9061127760ff60a01b91611262565b9181191691161790565b61128a906102c1565b90565b90565b906112a56112a06112ac92611281565b61128d565b8254611268565b9055565b6112b990610677565b90565b6112c5906112b0565b90565b5f1b90565b906112de60018060a01b03916112c8565b9181191691161790565b6112f1906112b0565b90565b90565b9061130c611307611313926112e8565b6112f4565b82546112cd565b9055565b611321600161122d565b611389578161134061133a6113355f611256565b6104fb565b916104fb565b1461136d5761136661135f61136b9361135a600180611290565b6112bc565b60016112f7565b611bbe565b565b5f632e7f3c7f60e11b815280611385600482016104b7565b0390fd5b5f62dc149f60e41b8152806113a0600482016104b7565b0390fd5b906113ae916111ff565b565b906113ba9061021f565b9052565b9061142561141c60036113cf610ec8565b946113e66113de5f8301610c67565b5f88016113b0565b6113fe6113f560018301610c67565b602088016113b0565b61141661140d60028301610c67565b604088016113b0565b01610c67565b606084016113b0565b565b611430906113be565b90565b61144a61144f91611442610f17565b506004610c38565b611427565b90565b61145a611fd9565b611462611464565b565b61146c612064565b565b611476611452565b565b611480611fd9565b61148861148a565b565b61149b6114965f611256565b612094565b565b6114a5611478565b565b6114b36114b891610c4e565b610634565b90565b6114c590546114a7565b90565b60e01b90565b6114d7816102c1565b036114de57565b5f80fd5b905051906114ef826114ce565b565b9060208282031261150a57611507915f016114e2565b90565b61020c565b611535611542959394929461152b60608401965f8501906109d5565b60208301906109d5565b604081850391015261116f565b90565b61154d610202565b3d5f823e3d90fd5b9261159860209394611565610f36565b506115a361157b61157660016114bb565b61069f565b93637a3979dc92959761158c610202565b988997889687966114c8565b86526004860161150f565b03915afa9081156115e7575f916115b9575b5090565b6115da915060203d81116115e0575b6115d28183610e8a565b8101906114f1565b5f6115b5565b503d6115c8565b611545565b5f90565b6115fa905161021f565b90565b6116056115ec565b5061162c6116136005610c67565b6116266060611620611cfe565b016115f0565b906110ea565b90565b9061164b61164533329085859192909192611555565b156102c1565b61165a5761165891611676565b565b5f631b8e828b60e31b815280611672600482016104b7565b0390fd5b9061168a61168460036110a4565b156102c1565b6116bf576116aa6116bd926116a36116b8935a926116ca565b5a906110c5565b6116b2610d4a565b906110ea565b611ed2565b565b6116c8916116ca565b565b906116d6903392611002565b906117166117047f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611163565b9261170d610202565b918291826103f5565b0390a2565b906117259161162f565b565b61173b61173661174092610f76565b610674565b61021f565b90565b61174b6115ec565b50611754611cfe565b61175f5f82016115f0565b61177161176b5f611727565b9161021f565b146117c6576117845f61179292016115f0565b61178c6107b0565b906110ea565b426117a561179f8361021f565b9161021f565b10156117b9576117b69042906110c5565b90565b506117c35f611727565b90565b506117d05f611727565b90565b6117db6115ec565b506117ef60606117e9611cfe565b016115f0565b90565b5f90565b60018060a01b031690565b61180d61181291610c4e565b6117f6565b90565b61181f9054611801565b90565b61182a6117f2565b506118345f611815565b90565b90565b61184e61184961185392611837565b610674565b61021f565b90565b61185e6115ec565b5061187261186c6003610f4e565b156102c1565b611896576118936118836002610c67565b61188d600161183a565b906110ea565b90565b61189f5f611727565b90565b6118aa6115ec565b506118be60406118b8611cfe565b016115f0565b90565b906118d56118cf60036110a4565b156102c1565b61190a576118f5611908926118ee611903935a926119af565b5a906110c5565b6118fd610d4a565b906110ea565b611ed2565b565b611913916119af565b565b5090565b6001611925910161021f565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561198a570180359067ffffffffffffffff82116119855760200191600182023603831361198057565b611944565b611940565b61193c565b908210156119aa5760206119a69202810190611948565b9091565b611928565b6119ba818390611915565b916119c36115ec565b506119cd5f611727565b5b806119e16119db8661021f565b9161021f565b1015611a7257611a0f90611a053332906119fd8787869161198f565b929091611555565b611a14575b611919565b6119ce565b33611a2a611a248686859161198f565b90611002565b90611a6a611a587f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611163565b92611a61610202565b918291826103f5565b0390a2611a0a565b50505050565b90611a82916118c1565b565b611a9590611a90611fd9565b611a97565b565b80611ab2611aac611aa75f611256565b6104fb565b916104fb565b14611b0c57611aca611ac3826112bc565b60016112f7565b611af47f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b991611163565b90611afd610202565b80611b07816104b7565b0390a2565b5f632e7f3c7f60e11b815280611b24600482016104b7565b0390fd5b611b3190611a84565b565b611b3b611fd9565b611b43611b45565b565b611b4d6120f3565b565b611b57611b33565b565b611b6a90611b65611fd9565b611b6c565b565b80611b87611b81611b7c5f611256565b6104fb565b916104fb565b14611b9757611b9590612094565b565b611bba611ba35f611256565b5f918291631e4fbdf760e01b8352600483016109e2565b0390fd5b611bc790611b59565b565b60209181520190565b5f7f476173436f756e7465723a20696e76616c69642072616e676500000000000000910152565b611c066019602092611bc9565b611c0f81611bd2565b0190565b611c289060208101905f818303910152611bf9565b90565b15611c3257565b611c3a610202565b62461bcd60e51b815280611c5060048201611c13565b0390fd5b611c8391611c606115ec565b50611c7e81611c77611c718561021f565b9161021f565b1015611c2b565b6110c5565b90565b611c906080610eb3565b90565b634e487b7160e01b5f52601260045260245ffd5b611cb3611cb99161021f565b9161021f565b908115611cc4570490565b611c93565b611cd8611cde9193929361021f565b9261021f565b91611cea83820261021f565b928184041490151715611cf957565b6110b1565b611d06610f17565b50611d1a611d146003610f4e565b156102c1565b611e1657611d3b611d366004611d306002610c67565b90610c38565b611427565b42611d69611d63611d5e611d505f86016115f0565b611d586107b0565b906110ea565b61021f565b9161021f565b1015611d725790565b611dbf90611db9611daa5f611da3611d9542611d8f8488016115f0565b906110c5565b611d9d6107b0565b90611ca7565b93016115f0565b91611db36107b0565b90611cc9565b906110ea565b611e13611e0a5f611e05611dfc5f611df7611dee5f95611de9611de0611c86565b9a5f8c016113b0565b611727565b602089016113b0565b611727565b604086016113b0565b611727565b606083016113b0565b90565b5f611e73611e6a5f611e65611e5c5f611e57611e4e5f95611e49611e41611e3b611c86565b9b611727565b5f8c016113b0565b611727565b602089016113b0565b611727565b604086016113b0565b611727565b606083016113b0565b90565b90611e825f19916112c8565b9181191691161790565b90565b90611ea4611e9f611eab92610c1c565b611e8c565b8254611e76565b9055565b916020611ed0929493611ec960408201965f83019061073a565b019061073a565b565b611ee5611edf60036110a4565b156102c1565b611fd657611efc611ef66003610f4e565b156102c1565b611fc9575b611f096122c9565b611f7a611f17823a90611cc9565b611f4a83611f446002611f346004611f2e83610c67565b90610c38565b0191611f3f83610c67565b6110ea565b90611e8f565b611f746003611f646004611f5e6002610c67565b90610c38565b0191611f6f83610c67565b6110ea565b90611e8f565b611f846002610c67565b3a611faf7f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610c1c565b92611fc4611fbb610202565b92839283611eaf565b0390a2565b611fd16121be565b611f01565b50565b611fe1611822565b611ffa611ff4611fef6124c9565b6104fb565b916104fb565b0361200157565b61202361200c6124c9565b5f91829163118cdaa760e01b8352600483016109e2565b0390fd5b60081b90565b9061203a61ff0091612027565b9181191691161790565b9061205961205461206092611281565b61128d565b825461202d565b9055565b61206f5f6003612044565b565b90565b9061208961208461209092611163565b612071565b82546112cd565b9055565b61209d5f611815565b6120a7825f612074565b906120db6120d57f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093611163565b91611163565b916120e4610202565b806120ee816104b7565b0390a3565b6120ff60016003612044565b565b9061210d60ff916112c8565b9181191691161790565b9061212c61212761213392611281565b61128d565b8254612101565b9055565b9061214190611727565b5f5260205260405f2090565b906121aa606060036121b0946121705f820161216a5f88016115f0565b90611e8f565b61218960018201612183602088016115f0565b90611e8f565b6121a26002820161219c604088016115f0565b90611e8f565b0192016115f0565b90611e8f565b565b906121bc9161214d565b565b6121d16121cb6003610f4e565b156102c1565b6121d8575b565b6121e460016003612117565b6121f76121f05f611727565b6002611e8f565b6122604261224f6122465f6122416122385f61223361222a5f9561222561221c611c86565b9a5f8c016113b0565b611727565b602089016113b0565b611727565b604086016113b0565b611727565b606083016113b0565b61225b60045f90612137565b6121b2565b5f42906122a26122907f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92611727565b92612299610202565b91829182610747565b0390a26121d6565b90565b6122b69061021f565b5f1981146122c45760010190565b6110b1565b6122e66122e160046122db6002610c67565b90610c38565b6122aa565b4261231461230e6123096122fb5f8601610c67565b6123036107b0565b906110ea565b61021f565b9161021f565b101561231e575b50565b61234661233d61232f5f8401610c67565b6123376107b0565b906110ea565b60018301611e8f565b61236e61236761235860038401610c67565b6123626005610c67565b6110ea565b6005611e8f565b6123786002610c67565b6123a561238760028401610c67565b9261239f5f61239860018401610c67565b9201610c67565b906110c5565b6123cf7f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610c1c565b926123e46123db610202565b92839283611eaf565b0390a26124036123fc6123f76002610c67565b6122ad565b6002611e8f565b6124754261245b6124525f61244d6124445f61243f6124365f95612431612428611c86565b9a5f8c016113b0565b611727565b602089016113b0565b611727565b604086016113b0565b611727565b606083016113b0565b612470600461246a6002610c67565b90610c38565b6121b2565b61247f6002610c67565b42906124c06124ae7f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610c1c565b926124b7610202565b91829182610747565b0390a25f61231b565b6124d16117f2565b50339056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x0ErV[a\0\x1D_5a\x01\xFCV[\x80c\x08aF\xD2\x14a\x01\xF7W\x80c\x18\xD5\xAA\xFE\x14a\x01\xF2W\x80c6l\xBA\xB7\x14a\x01\xEDW\x80c;j\xB2\xA9\x14a\x01\xE8W\x80cF\xE2\xCC\t\x14a\x01\xE3W\x80cH\\\xC9U\x14a\x01\xDEW\x80cK,\x07\x06\x14a\x01\xD9W\x80cTg\xCBH\x14a\x01\xD4W\x80c[<\xD6\xE2\x14a\x01\xCFW\x80caT8\x01\x14a\x01\xCAW\x80ceX\x95O\x14a\x01\xC5W\x80cqP\x18\xA6\x14a\x01\xC0W\x80cz9y\xDC\x14a\x01\xBBW\x80c\x7F\xBD)^\x14a\x01\xB6W\x80c\x80NQ#\x14a\x01\xB1W\x80c\x82\xF4J\xDE\x14a\x01\xACW\x80c\x84\xFA\xB6+\x14a\x01\xA7W\x80c\x8DZ#\x9B\x14a\x01\xA2W\x80c\x8D\xA5\xCB[\x14a\x01\x9DW\x80c\xAF\xF7Lm\x14a\x01\x98W\x80c\xC6`\xD3\xF3\x14a\x01\x93W\x80c\xCD\xAF\xB9x\x14a\x01\x8EW\x80c\xD4\xF0\xEBM\x14a\x01\x89W\x80c\xD8x\x13B\x14a\x01\x84W\x80c\xDE\x1FE>\x14a\x01\x7FW\x80c\xEAJ\x11\x04\x14a\x01zW\x80c\xED\xE0{\xD6\x14a\x01uW\x80c\xF2\xFD\xE3\x8B\x14a\x01pW\x80c\xF7\xB8\x93^\x14a\x01kWc\xFF{0\x84\x03a\0\x0EWa\x0E=V[a\r\xF8V[a\r\x98V[a\rcV[a\x0C\xF2V[a\x0B\xE9V[a\x0B\xB4V[a\x0B]V[a\x0B\x0BV[a\naV[a\n,V[a\t\xF7V[a\t\xA0V[a\tkV[a\t&V[a\x08\xF2V[a\x08\xBDV[a\x08\x84V[a\x07\xFFV[a\x07\xCAV[a\x07\\V[a\x06\xCDV[a\x06\x01V[a\x05\xCCV[a\x05WV[a\x04\xBCV[a\x04\x82V[a\x04\rV[a\x02\xE8V[a\x02\x8CV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x02\x1AWV[a\x02\x0CV[\x90V[a\x02+\x90a\x02\x1FV[\x90RV[\x90``\x80a\x02u\x93a\x02G_\x82\x01Q_\x86\x01\x90a\x02\"V[a\x02Y` \x82\x01Q` \x86\x01\x90a\x02\"V[a\x02k`@\x82\x01Q`@\x86\x01\x90a\x02\"V[\x01Q\x91\x01\x90a\x02\"V[V[\x91\x90a\x02\x8A\x90_`\x80\x85\x01\x94\x01\x90a\x02/V[V[4a\x02\xBCWa\x02\x9C6`\x04a\x02\x10V[a\x02\xB8a\x02\xA7a\x0F\"V[a\x02\xAFa\x02\x02V[\x91\x82\x91\x82a\x02wV[\x03\x90\xF3[a\x02\x08V[\x15\x15\x90V[a\x02\xCF\x90a\x02\xC1V[\x90RV[\x91\x90a\x02\xE6\x90_` \x85\x01\x94\x01\x90a\x02\xC6V[V[4a\x03\x18Wa\x02\xF86`\x04a\x02\x10V[a\x03\x14a\x03\x03a\x0F[V[a\x03\x0Ba\x02\x02V[\x91\x82\x91\x82a\x02\xD3V[\x03\x90\xF3[a\x02\x08V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03gW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03bW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x03]WV[a\x03)V[a\x03%V[a\x03!V[\x90` \x82\x82\x03\x12a\x03\x9DW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\x98Wa\x03\x94\x92\x01a\x03-V[\x90\x91V[a\x03\x1DV[a\x02\x0CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\xE3a\x03\xEC` \x93a\x03\xF1\x93a\x03\xDA\x81a\x03\xA2V[\x93\x84\x80\x93a\x03\xA6V[\x95\x86\x91\x01a\x03\xAFV[a\x03\xBAV[\x01\x90V[a\x04\n\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xC4V[\x90V[4a\x04>Wa\x04:a\x04)a\x04#6`\x04a\x03lV[\x90a\x10\x02V[a\x041a\x02\x02V[\x91\x82\x91\x82a\x03\xF5V[\x03\x90\xF3[a\x02\x08V[\x1C\x90V[`\xFF\x16\x90V[a\x04]\x90`\x08a\x04b\x93\x02a\x04CV[a\x04GV[\x90V[\x90a\x04p\x91Ta\x04MV[\x90V[a\x04\x7F`\x03_\x90a\x04eV[\x90V[4a\x04\xB2Wa\x04\x926`\x04a\x02\x10V[a\x04\xAEa\x04\x9Da\x04sV[a\x04\xA5a\x02\x02V[\x91\x82\x91\x82a\x02\xD3V[\x03\x90\xF3[a\x02\x08V[_\x01\x90V[4a\x04\xEBWa\x04\xD5a\x04\xCF6`\x04a\x03lV[\x90a\x11\xF3V[a\x04\xDDa\x02\x02V[\x80a\x04\xE7\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x05\x04\x90a\x04\xF0V[\x90V[a\x05\x10\x81a\x04\xFBV[\x03a\x05\x17WV[_\x80\xFD[\x90P5\x90a\x05(\x82a\x05\x07V[V[\x91\x90`@\x83\x82\x03\x12a\x05RW\x80a\x05Fa\x05O\x92_\x86\x01a\x05\x1BV[\x93` \x01a\x05\x1BV[\x90V[a\x02\x0CV[4a\x05\x86Wa\x05pa\x05j6`\x04a\x05*V[\x90a\x13\xA4V[a\x05xa\x02\x02V[\x80a\x05\x82\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[a\x05\x94\x81a\x02\x1FV[\x03a\x05\x9BWV[_\x80\xFD[\x90P5\x90a\x05\xAC\x82a\x05\x8BV[V[\x90` \x82\x82\x03\x12a\x05\xC7Wa\x05\xC4\x91_\x01a\x05\x9FV[\x90V[a\x02\x0CV[4a\x05\xFCWa\x05\xF8a\x05\xE7a\x05\xE26`\x04a\x05\xAEV[a\x143V[a\x05\xEFa\x02\x02V[\x91\x82\x91\x82a\x02wV[\x03\x90\xF3[a\x02\x08V[4a\x06/Wa\x06\x116`\x04a\x02\x10V[a\x06\x19a\x14nV[a\x06!a\x02\x02V[\x80a\x06+\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x06O\x90`\x08a\x06T\x93\x02a\x04CV[a\x064V[\x90V[\x90a\x06b\x91Ta\x06?V[\x90V[a\x06q`\x01_\x90a\x06WV[\x90V[\x90V[a\x06\x8Ba\x06\x86a\x06\x90\x92a\x04\xF0V[a\x06tV[a\x04\xF0V[\x90V[a\x06\x9C\x90a\x06wV[\x90V[a\x06\xA8\x90a\x06\x93V[\x90V[a\x06\xB4\x90a\x06\x9FV[\x90RV[\x91\x90a\x06\xCB\x90_` \x85\x01\x94\x01\x90a\x06\xABV[V[4a\x06\xFDWa\x06\xDD6`\x04a\x02\x10V[a\x06\xF9a\x06\xE8a\x06eV[a\x06\xF0a\x02\x02V[\x91\x82\x91\x82a\x06\xB8V[\x03\x90\xF3[a\x02\x08V[\x90V[a\x07\x15\x90`\x08a\x07\x1A\x93\x02a\x04CV[a\x07\x02V[\x90V[\x90a\x07(\x91Ta\x07\x05V[\x90V[a\x077`\x02_\x90a\x07\x1DV[\x90V[a\x07C\x90a\x02\x1FV[\x90RV[\x91\x90a\x07Z\x90_` \x85\x01\x94\x01\x90a\x07:V[V[4a\x07\x8CWa\x07l6`\x04a\x02\x10V[a\x07\x88a\x07wa\x07+V[a\x07\x7Fa\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[\x90V[a\x07\xA8a\x07\xA3a\x07\xAD\x92a\x07\x91V[a\x06tV[a\x02\x1FV[\x90V[a\x07\xBCb'\x8D\0a\x07\x94V[\x90V[a\x07\xC7a\x07\xB0V[\x90V[4a\x07\xFAWa\x07\xDA6`\x04a\x02\x10V[a\x07\xF6a\x07\xE5a\x07\xBFV[a\x07\xEDa\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[4a\x08-Wa\x08\x0F6`\x04a\x02\x10V[a\x08\x17a\x14\x9DV[a\x08\x1Fa\x02\x02V[\x80a\x08)\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[\x91``\x83\x83\x03\x12a\x08\x7FWa\x08I\x82_\x85\x01a\x05\x1BV[\x92a\x08W\x83` \x83\x01a\x05\x1BV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08zWa\x08v\x92\x01a\x03-V[\x90\x91V[a\x03\x1DV[a\x02\x0CV[4a\x08\xB8Wa\x08\xB4a\x08\xA3a\x08\x9A6`\x04a\x082V[\x92\x91\x90\x91a\x15UV[a\x08\xABa\x02\x02V[\x91\x82\x91\x82a\x02\xD3V[\x03\x90\xF3[a\x02\x08V[4a\x08\xEDWa\x08\xCD6`\x04a\x02\x10V[a\x08\xE9a\x08\xD8a\x15\xFDV[a\x08\xE0a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[4a\t!Wa\t\x0Ba\t\x056`\x04a\x03lV[\x90a\x17\x1BV[a\t\x13a\x02\x02V[\x80a\t\x1D\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[4a\tVWa\t66`\x04a\x02\x10V[a\tRa\tAa\x17CV[a\tIa\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[a\th`\x03`\x01\x90a\x04eV[\x90V[4a\t\x9BWa\t{6`\x04a\x02\x10V[a\t\x97a\t\x86a\t[V[a\t\x8Ea\x02\x02V[\x91\x82\x91\x82a\x02\xD3V[\x03\x90\xF3[a\x02\x08V[4a\t\xD0Wa\t\xB06`\x04a\x02\x10V[a\t\xCCa\t\xBBa\x17\xD3V[a\t\xC3a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[a\t\xDE\x90a\x04\xFBV[\x90RV[\x91\x90a\t\xF5\x90_` \x85\x01\x94\x01\x90a\t\xD5V[V[4a\n'Wa\n\x076`\x04a\x02\x10V[a\n#a\n\x12a\x18\"V[a\n\x1Aa\x02\x02V[\x91\x82\x91\x82a\t\xE2V[\x03\x90\xF3[a\x02\x08V[4a\n\\Wa\n<6`\x04a\x02\x10V[a\nXa\nGa\x18VV[a\nOa\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[4a\n\x91Wa\nq6`\x04a\x02\x10V[a\n\x8Da\n|a\x18\xA2V[a\n\x84a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\n\xD0W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\xCBW` \x01\x92` \x83\x02\x84\x01\x11a\n\xC6WV[a\x03)V[a\x03%V[a\x03!V[\x90` \x82\x82\x03\x12a\x0B\x06W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0B\x01Wa\n\xFD\x92\x01a\n\x96V[\x90\x91V[a\x03\x1DV[a\x02\x0CV[4a\x0B:Wa\x0B$a\x0B\x1E6`\x04a\n\xD5V[\x90a\x1AxV[a\x0B,a\x02\x02V[\x80a\x0B6\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[\x90` \x82\x82\x03\x12a\x0BXWa\x0BU\x91_\x01a\x05\x1BV[\x90V[a\x02\x0CV[4a\x0B\x8BWa\x0Bua\x0Bp6`\x04a\x0B?V[a\x1B(V[a\x0B}a\x02\x02V[\x80a\x0B\x87\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B\xE4Wa\x0B\xC46`\x04a\x02\x10V[a\x0B\xE0a\x0B\xCFa\x0B\x90V[a\x0B\xD7a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[4a\x0C\x17Wa\x0B\xF96`\x04a\x02\x10V[a\x0C\x01a\x1BOV[a\x0C\ta\x02\x02V[\x80a\x0C\x13\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[a\x0C0a\x0C+a\x0C5\x92a\x02\x1FV[a\x06tV[a\x02\x1FV[\x90V[\x90a\x0CB\x90a\x0C\x1CV[_R` R`@_ \x90V[_\x1C\x90V[a\x0C_a\x0Cd\x91a\x0CNV[a\x07\x02V[\x90V[a\x0Cq\x90Ta\x0CSV[\x90V[a\x0C\x7F\x90`\x04a\x0C8V[\x90a\x0C\x8B_\x83\x01a\x0CgV[\x91a\x0C\x98`\x01\x82\x01a\x0CgV[\x91a\x0C\xB1`\x03a\x0C\xAA`\x02\x85\x01a\x0CgV[\x93\x01a\x0CgV[\x90V[a\x0C\xE9a\x0C\xF0\x94a\x0C\xDF``\x94\x98\x97\x95a\x0C\xD5`\x80\x86\x01\x9A_\x87\x01\x90a\x07:V[` \x85\x01\x90a\x07:V[`@\x83\x01\x90a\x07:V[\x01\x90a\x07:V[V[4a\r&Wa\r\"a\r\ra\r\x086`\x04a\x05\xAEV[a\x0CtV[\x90a\r\x19\x94\x92\x94a\x02\x02V[\x94\x85\x94\x85a\x0C\xB4V[\x03\x90\xF3[a\x02\x08V[\x90V[a\rBa\r=a\rG\x92a\r+V[a\x06tV[a\x02\x1FV[\x90V[a\rUa\x13\x88a\r.V[\x90V[a\r`a\rJV[\x90V[4a\r\x93Wa\rs6`\x04a\x02\x10V[a\r\x8Fa\r~a\rXV[a\r\x86a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[4a\r\xC6Wa\r\xB0a\r\xAB6`\x04a\x0B?V[a\x1B\xBEV[a\r\xB8a\x02\x02V[\x80a\r\xC2\x81a\x04\xB7V[\x03\x90\xF3[a\x02\x08V[\x91\x90`@\x83\x82\x03\x12a\r\xF3W\x80a\r\xE7a\r\xF0\x92_\x86\x01a\x05\x9FV[\x93` \x01a\x05\x9FV[\x90V[a\x02\x0CV[4a\x0E)Wa\x0E%a\x0E\x14a\x0E\x0E6`\x04a\r\xCBV[\x90a\x1CTV[a\x0E\x1Ca\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[a\x0E:`\x05_\x90a\x07\x1DV[\x90V[4a\x0EmWa\x0EM6`\x04a\x02\x10V[a\x0Eia\x0EXa\x0E.V[a\x0E`a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xF3[a\x02\x08V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x0E\x94\x90a\x03\xBAV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xAEW`@RV[a\x0EvV[\x90a\x0E\xC6a\x0E\xBFa\x02\x02V[\x92\x83a\x0E\x8AV[V[a\x0E\xD2`\x80a\x0E\xB3V[\x90V[_\x90V[a\x0E\xE1a\x0E\xC8V[\x90` \x80\x80\x80\x85a\x0E\xF0a\x0E\xD5V[\x81R\x01a\x0E\xFBa\x0E\xD5V[\x81R\x01a\x0F\x06a\x0E\xD5V[\x81R\x01a\x0F\x11a\x0E\xD5V[\x81RPPV[a\x0F\x1Fa\x0E\xD9V[\x90V[a\x0F*a\x0F\x17V[Pa\x0F3a\x1C\xFEV[\x90V[_\x90V[a\x0FFa\x0FK\x91a\x0CNV[a\x04GV[\x90V[a\x0FX\x90Ta\x0F:V[\x90V[a\x0Fca\x0F6V[Pa\x0Fn`\x03a\x0FNV[\x90V[``\x90V[\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x0F\x9Ca\x0F\x97a\x0F\xA1\x92a\x0FvV[a\x0F\x82V[a\x0FyV[\x90V[\x90V[a\x0F\xB3a\x0F\xB8\x91a\x0FyV[a\x0F\xA4V[\x90RV[\x90P\x90V[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x0F\xDC\x81a\x0F\xE3\x93a\x0F\xBCV[\x80\x93a\x0F\xC1V[\x01\x90V[\x80a\x0F\xF8`\x01\x92a\x0F\xFF\x96\x94a\x0F\xA7V[\x01\x91a\x0F\xCCV[\x90V[a\x10@\x90a\x10\x0Ea\x0FqV[Pa\x101a\x10\x1B_a\x0F\x88V[\x91\x93a\x10%a\x02\x02V[\x94\x85\x93` \x85\x01a\x0F\xE7V[` \x82\x01\x81\x03\x82R\x03\x82a\x0E\x8AV[\x90V[\x90a\x10_a\x10Y32\x90\x85\x85\x91\x92\x90\x91\x92a\x15UV[\x15a\x02\xC1V[a\x10nWa\x10l\x91a\x11\x0FV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x10\x86`\x04\x82\x01a\x04\xB7V[\x03\x90\xFD[`\x08\x1C\x90V[a\x10\x9Ca\x10\xA1\x91a\x10\x8AV[a\x04GV[\x90V[a\x10\xAE\x90Ta\x10\x90V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x10\xD4a\x10\xDA\x91\x93\x92\x93a\x02\x1FV[\x92a\x02\x1FV[\x82\x03\x91\x82\x11a\x10\xE5WV[a\x10\xB1V[a\x10\xF9a\x10\xFF\x91\x93\x92\x93a\x02\x1FV[\x92a\x02\x1FV[\x82\x01\x80\x92\x11a\x11\nWV[a\x10\xB1V[\x90a\x11#a\x11\x1D`\x03a\x10\xA4V[\x15a\x02\xC1V[a\x11XWa\x11Ca\x11V\x92a\x11<a\x11Q\x93Z\x92a\x11\xACV[Z\x90a\x10\xC5V[a\x11Ka\rJV[\x90a\x10\xEAV[a\x1E\xD2V[V[a\x11a\x91a\x11\xACV[V[a\x11l\x90a\x06\x93V[\x90V[\x91\x90a\x11\x89\x81a\x11\x82\x81a\x11\x8E\x95a\x03\xA6V[\x80\x95a\x0F\xC1V[a\x03\xBAV[\x01\x90V[\x90\x91a\x11\xA9\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x11oV[\x90V[3\x90\x91a\x11\xD9\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11cV[\x92a\x11\xEEa\x11\xE5a\x02\x02V[\x92\x83\x92\x83a\x11\x92V[\x03\x90\xA2V[\x90a\x11\xFD\x91a\x10CV[V[\x90a\x12\x11\x91a\x12\x0Ca\x1F\xD9V[a\x13\x17V[V[`\xA0\x1C\x90V[a\x12%a\x12*\x91a\x12\x13V[a\x04GV[\x90V[a\x127\x90Ta\x12\x19V[\x90V[a\x12Na\x12Ia\x12S\x92a\x0FvV[a\x06tV[a\x04\xF0V[\x90V[a\x12_\x90a\x12:V[\x90V[`\xA0\x1B\x90V[\x90a\x12w`\xFF`\xA0\x1B\x91a\x12bV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x12\x8A\x90a\x02\xC1V[\x90V[\x90V[\x90a\x12\xA5a\x12\xA0a\x12\xAC\x92a\x12\x81V[a\x12\x8DV[\x82Ta\x12hV[\x90UV[a\x12\xB9\x90a\x06wV[\x90V[a\x12\xC5\x90a\x12\xB0V[\x90V[_\x1B\x90V[\x90a\x12\xDE`\x01\x80`\xA0\x1B\x03\x91a\x12\xC8V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x12\xF1\x90a\x12\xB0V[\x90V[\x90V[\x90a\x13\x0Ca\x13\x07a\x13\x13\x92a\x12\xE8V[a\x12\xF4V[\x82Ta\x12\xCDV[\x90UV[a\x13!`\x01a\x12-V[a\x13\x89W\x81a\x13@a\x13:a\x135_a\x12VV[a\x04\xFBV[\x91a\x04\xFBV[\x14a\x13mWa\x13fa\x13_a\x13k\x93a\x13Z`\x01\x80a\x12\x90V[a\x12\xBCV[`\x01a\x12\xF7V[a\x1B\xBEV[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x13\x85`\x04\x82\x01a\x04\xB7V[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x13\xA0`\x04\x82\x01a\x04\xB7V[\x03\x90\xFD[\x90a\x13\xAE\x91a\x11\xFFV[V[\x90a\x13\xBA\x90a\x02\x1FV[\x90RV[\x90a\x14%a\x14\x1C`\x03a\x13\xCFa\x0E\xC8V[\x94a\x13\xE6a\x13\xDE_\x83\x01a\x0CgV[_\x88\x01a\x13\xB0V[a\x13\xFEa\x13\xF5`\x01\x83\x01a\x0CgV[` \x88\x01a\x13\xB0V[a\x14\x16a\x14\r`\x02\x83\x01a\x0CgV[`@\x88\x01a\x13\xB0V[\x01a\x0CgV[``\x84\x01a\x13\xB0V[V[a\x140\x90a\x13\xBEV[\x90V[a\x14Ja\x14O\x91a\x14Ba\x0F\x17V[P`\x04a\x0C8V[a\x14'V[\x90V[a\x14Za\x1F\xD9V[a\x14ba\x14dV[V[a\x14la dV[V[a\x14va\x14RV[V[a\x14\x80a\x1F\xD9V[a\x14\x88a\x14\x8AV[V[a\x14\x9Ba\x14\x96_a\x12VV[a \x94V[V[a\x14\xA5a\x14xV[V[a\x14\xB3a\x14\xB8\x91a\x0CNV[a\x064V[\x90V[a\x14\xC5\x90Ta\x14\xA7V[\x90V[`\xE0\x1B\x90V[a\x14\xD7\x81a\x02\xC1V[\x03a\x14\xDEWV[_\x80\xFD[\x90PQ\x90a\x14\xEF\x82a\x14\xCEV[V[\x90` \x82\x82\x03\x12a\x15\nWa\x15\x07\x91_\x01a\x14\xE2V[\x90V[a\x02\x0CV[a\x155a\x15B\x95\x93\x94\x92\x94a\x15+``\x84\x01\x96_\x85\x01\x90a\t\xD5V[` \x83\x01\x90a\t\xD5V[`@\x81\x85\x03\x91\x01Ra\x11oV[\x90V[a\x15Ma\x02\x02V[=_\x82>=\x90\xFD[\x92a\x15\x98` \x93\x94a\x15ea\x0F6V[Pa\x15\xA3a\x15{a\x15v`\x01a\x14\xBBV[a\x06\x9FV[\x93cz9y\xDC\x92\x95\x97a\x15\x8Ca\x02\x02V[\x98\x89\x97\x88\x96\x87\x96a\x14\xC8V[\x86R`\x04\x86\x01a\x15\x0FV[\x03\x91Z\xFA\x90\x81\x15a\x15\xE7W_\x91a\x15\xB9W[P\x90V[a\x15\xDA\x91P` =\x81\x11a\x15\xE0W[a\x15\xD2\x81\x83a\x0E\x8AV[\x81\x01\x90a\x14\xF1V[_a\x15\xB5V[P=a\x15\xC8V[a\x15EV[_\x90V[a\x15\xFA\x90Qa\x02\x1FV[\x90V[a\x16\x05a\x15\xECV[Pa\x16,a\x16\x13`\x05a\x0CgV[a\x16&``a\x16 a\x1C\xFEV[\x01a\x15\xF0V[\x90a\x10\xEAV[\x90V[\x90a\x16Ka\x16E32\x90\x85\x85\x91\x92\x90\x91\x92a\x15UV[\x15a\x02\xC1V[a\x16ZWa\x16X\x91a\x16vV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x16r`\x04\x82\x01a\x04\xB7V[\x03\x90\xFD[\x90a\x16\x8Aa\x16\x84`\x03a\x10\xA4V[\x15a\x02\xC1V[a\x16\xBFWa\x16\xAAa\x16\xBD\x92a\x16\xA3a\x16\xB8\x93Z\x92a\x16\xCAV[Z\x90a\x10\xC5V[a\x16\xB2a\rJV[\x90a\x10\xEAV[a\x1E\xD2V[V[a\x16\xC8\x91a\x16\xCAV[V[\x90a\x16\xD6\x903\x92a\x10\x02V[\x90a\x17\x16a\x17\x04\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11cV[\x92a\x17\ra\x02\x02V[\x91\x82\x91\x82a\x03\xF5V[\x03\x90\xA2V[\x90a\x17%\x91a\x16/V[V[a\x17;a\x176a\x17@\x92a\x0FvV[a\x06tV[a\x02\x1FV[\x90V[a\x17Ka\x15\xECV[Pa\x17Ta\x1C\xFEV[a\x17__\x82\x01a\x15\xF0V[a\x17qa\x17k_a\x17'V[\x91a\x02\x1FV[\x14a\x17\xC6Wa\x17\x84_a\x17\x92\x92\x01a\x15\xF0V[a\x17\x8Ca\x07\xB0V[\x90a\x10\xEAV[Ba\x17\xA5a\x17\x9F\x83a\x02\x1FV[\x91a\x02\x1FV[\x10\x15a\x17\xB9Wa\x17\xB6\x90B\x90a\x10\xC5V[\x90V[Pa\x17\xC3_a\x17'V[\x90V[Pa\x17\xD0_a\x17'V[\x90V[a\x17\xDBa\x15\xECV[Pa\x17\xEF``a\x17\xE9a\x1C\xFEV[\x01a\x15\xF0V[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x18\ra\x18\x12\x91a\x0CNV[a\x17\xF6V[\x90V[a\x18\x1F\x90Ta\x18\x01V[\x90V[a\x18*a\x17\xF2V[Pa\x184_a\x18\x15V[\x90V[\x90V[a\x18Na\x18Ia\x18S\x92a\x187V[a\x06tV[a\x02\x1FV[\x90V[a\x18^a\x15\xECV[Pa\x18ra\x18l`\x03a\x0FNV[\x15a\x02\xC1V[a\x18\x96Wa\x18\x93a\x18\x83`\x02a\x0CgV[a\x18\x8D`\x01a\x18:V[\x90a\x10\xEAV[\x90V[a\x18\x9F_a\x17'V[\x90V[a\x18\xAAa\x15\xECV[Pa\x18\xBE`@a\x18\xB8a\x1C\xFEV[\x01a\x15\xF0V[\x90V[\x90a\x18\xD5a\x18\xCF`\x03a\x10\xA4V[\x15a\x02\xC1V[a\x19\nWa\x18\xF5a\x19\x08\x92a\x18\xEEa\x19\x03\x93Z\x92a\x19\xAFV[Z\x90a\x10\xC5V[a\x18\xFDa\rJV[\x90a\x10\xEAV[a\x1E\xD2V[V[a\x19\x13\x91a\x19\xAFV[V[P\x90V[`\x01a\x19%\x91\x01a\x02\x1FV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x19\x8AW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x19\x85W` \x01\x91`\x01\x82\x026\x03\x83\x13a\x19\x80WV[a\x19DV[a\x19@V[a\x19<V[\x90\x82\x10\x15a\x19\xAAW` a\x19\xA6\x92\x02\x81\x01\x90a\x19HV[\x90\x91V[a\x19(V[a\x19\xBA\x81\x83\x90a\x19\x15V[\x91a\x19\xC3a\x15\xECV[Pa\x19\xCD_a\x17'V[[\x80a\x19\xE1a\x19\xDB\x86a\x02\x1FV[\x91a\x02\x1FV[\x10\x15a\x1ArWa\x1A\x0F\x90a\x1A\x0532\x90a\x19\xFD\x87\x87\x86\x91a\x19\x8FV[\x92\x90\x91a\x15UV[a\x1A\x14W[a\x19\x19V[a\x19\xCEV[3a\x1A*a\x1A$\x86\x86\x85\x91a\x19\x8FV[\x90a\x10\x02V[\x90a\x1Aja\x1AX\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11cV[\x92a\x1Aaa\x02\x02V[\x91\x82\x91\x82a\x03\xF5V[\x03\x90\xA2a\x1A\nV[PPPPV[\x90a\x1A\x82\x91a\x18\xC1V[V[a\x1A\x95\x90a\x1A\x90a\x1F\xD9V[a\x1A\x97V[V[\x80a\x1A\xB2a\x1A\xACa\x1A\xA7_a\x12VV[a\x04\xFBV[\x91a\x04\xFBV[\x14a\x1B\x0CWa\x1A\xCAa\x1A\xC3\x82a\x12\xBCV[`\x01a\x12\xF7V[a\x1A\xF4\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x11cV[\x90a\x1A\xFDa\x02\x02V[\x80a\x1B\x07\x81a\x04\xB7V[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x1B$`\x04\x82\x01a\x04\xB7V[\x03\x90\xFD[a\x1B1\x90a\x1A\x84V[V[a\x1B;a\x1F\xD9V[a\x1BCa\x1BEV[V[a\x1BMa \xF3V[V[a\x1BWa\x1B3V[V[a\x1Bj\x90a\x1Bea\x1F\xD9V[a\x1BlV[V[\x80a\x1B\x87a\x1B\x81a\x1B|_a\x12VV[a\x04\xFBV[\x91a\x04\xFBV[\x14a\x1B\x97Wa\x1B\x95\x90a \x94V[V[a\x1B\xBAa\x1B\xA3_a\x12VV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\t\xE2V[\x03\x90\xFD[a\x1B\xC7\x90a\x1BYV[V[` \x91\x81R\x01\x90V[_\x7FGasCounter: invalid range\0\0\0\0\0\0\0\x91\x01RV[a\x1C\x06`\x19` \x92a\x1B\xC9V[a\x1C\x0F\x81a\x1B\xD2V[\x01\x90V[a\x1C(\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1B\xF9V[\x90V[\x15a\x1C2WV[a\x1C:a\x02\x02V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1CP`\x04\x82\x01a\x1C\x13V[\x03\x90\xFD[a\x1C\x83\x91a\x1C`a\x15\xECV[Pa\x1C~\x81a\x1Cwa\x1Cq\x85a\x02\x1FV[\x91a\x02\x1FV[\x10\x15a\x1C+V[a\x10\xC5V[\x90V[a\x1C\x90`\x80a\x0E\xB3V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\x1C\xB3a\x1C\xB9\x91a\x02\x1FV[\x91a\x02\x1FV[\x90\x81\x15a\x1C\xC4W\x04\x90V[a\x1C\x93V[a\x1C\xD8a\x1C\xDE\x91\x93\x92\x93a\x02\x1FV[\x92a\x02\x1FV[\x91a\x1C\xEA\x83\x82\x02a\x02\x1FV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1C\xF9WV[a\x10\xB1V[a\x1D\x06a\x0F\x17V[Pa\x1D\x1Aa\x1D\x14`\x03a\x0FNV[\x15a\x02\xC1V[a\x1E\x16Wa\x1D;a\x1D6`\x04a\x1D0`\x02a\x0CgV[\x90a\x0C8V[a\x14'V[Ba\x1Dia\x1Dca\x1D^a\x1DP_\x86\x01a\x15\xF0V[a\x1DXa\x07\xB0V[\x90a\x10\xEAV[a\x02\x1FV[\x91a\x02\x1FV[\x10\x15a\x1DrW\x90V[a\x1D\xBF\x90a\x1D\xB9a\x1D\xAA_a\x1D\xA3a\x1D\x95Ba\x1D\x8F\x84\x88\x01a\x15\xF0V[\x90a\x10\xC5V[a\x1D\x9Da\x07\xB0V[\x90a\x1C\xA7V[\x93\x01a\x15\xF0V[\x91a\x1D\xB3a\x07\xB0V[\x90a\x1C\xC9V[\x90a\x10\xEAV[a\x1E\x13a\x1E\n_a\x1E\x05a\x1D\xFC_a\x1D\xF7a\x1D\xEE_\x95a\x1D\xE9a\x1D\xE0a\x1C\x86V[\x9A_\x8C\x01a\x13\xB0V[a\x17'V[` \x89\x01a\x13\xB0V[a\x17'V[`@\x86\x01a\x13\xB0V[a\x17'V[``\x83\x01a\x13\xB0V[\x90V[_a\x1Esa\x1Ej_a\x1Eea\x1E\\_a\x1EWa\x1EN_\x95a\x1EIa\x1EAa\x1E;a\x1C\x86V[\x9Ba\x17'V[_\x8C\x01a\x13\xB0V[a\x17'V[` \x89\x01a\x13\xB0V[a\x17'V[`@\x86\x01a\x13\xB0V[a\x17'V[``\x83\x01a\x13\xB0V[\x90V[\x90a\x1E\x82_\x19\x91a\x12\xC8V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x1E\xA4a\x1E\x9Fa\x1E\xAB\x92a\x0C\x1CV[a\x1E\x8CV[\x82Ta\x1EvV[\x90UV[\x91` a\x1E\xD0\x92\x94\x93a\x1E\xC9`@\x82\x01\x96_\x83\x01\x90a\x07:V[\x01\x90a\x07:V[V[a\x1E\xE5a\x1E\xDF`\x03a\x10\xA4V[\x15a\x02\xC1V[a\x1F\xD6Wa\x1E\xFCa\x1E\xF6`\x03a\x0FNV[\x15a\x02\xC1V[a\x1F\xC9W[a\x1F\ta\"\xC9V[a\x1Fza\x1F\x17\x82:\x90a\x1C\xC9V[a\x1FJ\x83a\x1FD`\x02a\x1F4`\x04a\x1F.\x83a\x0CgV[\x90a\x0C8V[\x01\x91a\x1F?\x83a\x0CgV[a\x10\xEAV[\x90a\x1E\x8FV[a\x1Ft`\x03a\x1Fd`\x04a\x1F^`\x02a\x0CgV[\x90a\x0C8V[\x01\x91a\x1Fo\x83a\x0CgV[a\x10\xEAV[\x90a\x1E\x8FV[a\x1F\x84`\x02a\x0CgV[:a\x1F\xAF\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0C\x1CV[\x92a\x1F\xC4a\x1F\xBBa\x02\x02V[\x92\x83\x92\x83a\x1E\xAFV[\x03\x90\xA2V[a\x1F\xD1a!\xBEV[a\x1F\x01V[PV[a\x1F\xE1a\x18\"V[a\x1F\xFAa\x1F\xF4a\x1F\xEFa$\xC9V[a\x04\xFBV[\x91a\x04\xFBV[\x03a \x01WV[a #a \x0Ca$\xC9V[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\t\xE2V[\x03\x90\xFD[`\x08\x1B\x90V[\x90a :a\xFF\0\x91a 'V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a Ya Ta `\x92a\x12\x81V[a\x12\x8DV[\x82Ta -V[\x90UV[a o_`\x03a DV[V[\x90V[\x90a \x89a \x84a \x90\x92a\x11cV[a qV[\x82Ta\x12\xCDV[\x90UV[a \x9D_a\x18\x15V[a \xA7\x82_a tV[\x90a \xDBa \xD5\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x11cV[\x91a\x11cV[\x91a \xE4a\x02\x02V[\x80a \xEE\x81a\x04\xB7V[\x03\x90\xA3V[a \xFF`\x01`\x03a DV[V[\x90a!\r`\xFF\x91a\x12\xC8V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a!,a!'a!3\x92a\x12\x81V[a\x12\x8DV[\x82Ta!\x01V[\x90UV[\x90a!A\x90a\x17'V[_R` R`@_ \x90V[\x90a!\xAA```\x03a!\xB0\x94a!p_\x82\x01a!j_\x88\x01a\x15\xF0V[\x90a\x1E\x8FV[a!\x89`\x01\x82\x01a!\x83` \x88\x01a\x15\xF0V[\x90a\x1E\x8FV[a!\xA2`\x02\x82\x01a!\x9C`@\x88\x01a\x15\xF0V[\x90a\x1E\x8FV[\x01\x92\x01a\x15\xF0V[\x90a\x1E\x8FV[V[\x90a!\xBC\x91a!MV[V[a!\xD1a!\xCB`\x03a\x0FNV[\x15a\x02\xC1V[a!\xD8W[V[a!\xE4`\x01`\x03a!\x17V[a!\xF7a!\xF0_a\x17'V[`\x02a\x1E\x8FV[a\"`Ba\"Oa\"F_a\"Aa\"8_a\"3a\"*_\x95a\"%a\"\x1Ca\x1C\x86V[\x9A_\x8C\x01a\x13\xB0V[a\x17'V[` \x89\x01a\x13\xB0V[a\x17'V[`@\x86\x01a\x13\xB0V[a\x17'V[``\x83\x01a\x13\xB0V[a\"[`\x04_\x90a!7V[a!\xB2V[_B\x90a\"\xA2a\"\x90\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x17'V[\x92a\"\x99a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xA2a!\xD6V[\x90V[a\"\xB6\x90a\x02\x1FV[_\x19\x81\x14a\"\xC4W`\x01\x01\x90V[a\x10\xB1V[a\"\xE6a\"\xE1`\x04a\"\xDB`\x02a\x0CgV[\x90a\x0C8V[a\"\xAAV[Ba#\x14a#\x0Ea#\ta\"\xFB_\x86\x01a\x0CgV[a#\x03a\x07\xB0V[\x90a\x10\xEAV[a\x02\x1FV[\x91a\x02\x1FV[\x10\x15a#\x1EW[PV[a#Fa#=a#/_\x84\x01a\x0CgV[a#7a\x07\xB0V[\x90a\x10\xEAV[`\x01\x83\x01a\x1E\x8FV[a#na#ga#X`\x03\x84\x01a\x0CgV[a#b`\x05a\x0CgV[a\x10\xEAV[`\x05a\x1E\x8FV[a#x`\x02a\x0CgV[a#\xA5a#\x87`\x02\x84\x01a\x0CgV[\x92a#\x9F_a#\x98`\x01\x84\x01a\x0CgV[\x92\x01a\x0CgV[\x90a\x10\xC5V[a#\xCF\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0C\x1CV[\x92a#\xE4a#\xDBa\x02\x02V[\x92\x83\x92\x83a\x1E\xAFV[\x03\x90\xA2a$\x03a#\xFCa#\xF7`\x02a\x0CgV[a\"\xADV[`\x02a\x1E\x8FV[a$uBa$[a$R_a$Ma$D_a$?a$6_\x95a$1a$(a\x1C\x86V[\x9A_\x8C\x01a\x13\xB0V[a\x17'V[` \x89\x01a\x13\xB0V[a\x17'V[`@\x86\x01a\x13\xB0V[a\x17'V[``\x83\x01a\x13\xB0V[a$p`\x04a$j`\x02a\x0CgV[\x90a\x0C8V[a!\xB2V[a$\x7F`\x02a\x0CgV[B\x90a$\xC0a$\xAE\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0C\x1CV[\x92a$\xB7a\x02\x02V[\x91\x82\x91\x82a\x07GV[\x03\x90\xA2_a#\x1BV[a$\xD1a\x17\xF2V[P3\x90V",
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
    pub struct TransactionProcessed {
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
        impl alloy_sol_types::SolEvent for TransactionProcessed {
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
        impl alloy_sol_types::private::IntoLogData for TransactionProcessed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransactionProcessed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransactionProcessed) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `processTransaction(bytes)` and selector `0x46e2cc09`.
```solidity
function processTransaction(bytes memory data) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionCall {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`processTransaction(bytes)`](processTransactionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionReturn {}
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
            impl ::core::convert::From<processTransactionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionCall) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionCall {
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
            impl ::core::convert::From<processTransactionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processTransactionCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processTransactionReturn;
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
    /**Function with signature `processTransactionUncompressed(bytes)` and selector `0x804e5123`.
```solidity
function processTransactionUncompressed(bytes memory data) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionUncompressedCall {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`processTransactionUncompressed(bytes)`](processTransactionUncompressedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionUncompressedReturn {}
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
            impl ::core::convert::From<processTransactionUncompressedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionUncompressedCall) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionUncompressedCall {
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
            impl ::core::convert::From<processTransactionUncompressedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionUncompressedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionUncompressedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processTransactionUncompressedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processTransactionUncompressedReturn;
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
    /**Function with signature `processTransactionsBulk(bytes[])` and selector `0xcdafb978`.
```solidity
function processTransactionsBulk(bytes[] memory data) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionsBulkCall {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
    }
    ///Container type for the return parameters of the [`processTransactionsBulk(bytes[])`](processTransactionsBulkCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processTransactionsBulkReturn {}
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
            impl ::core::convert::From<processTransactionsBulkCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionsBulkCall) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionsBulkCall {
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
            impl ::core::convert::From<processTransactionsBulkReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: processTransactionsBulkReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processTransactionsBulkReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processTransactionsBulkCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processTransactionsBulkReturn;
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
    ///Container for all the [`SyndicateSequencingChain`](self) function calls.
    pub enum SyndicateSequencingChainCalls {
        #[allow(missing_docs)]
        PERIOD_DURATION(PERIOD_DURATIONCall),
        #[allow(missing_docs)]
        TRACKING_OVERHEAD(TRACKING_OVERHEADCall),
        #[allow(missing_docs)]
        appchainId(appchainIdCall),
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
        processTransaction(processTransactionCall),
        #[allow(missing_docs)]
        processTransactionUncompressed(processTransactionUncompressedCall),
        #[allow(missing_docs)]
        processTransactionsBulk(processTransactionsBulkCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        updateRequirementModule(updateRequirementModuleCall),
    }
    #[automatically_derived]
    impl SyndicateSequencingChainCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 97u8, 70u8, 210u8],
            [24u8, 213u8, 170u8, 254u8],
            [54u8, 108u8, 186u8, 183u8],
            [59u8, 106u8, 178u8, 169u8],
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
    impl alloy_sol_types::SolInterface for SyndicateSequencingChainCalls {
        const NAME: &'static str = "SyndicateSequencingChainCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 30usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::PERIOD_DURATION(_) => {
                    <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::TRACKING_OVERHEAD(_) => {
                    <TRACKING_OVERHEADCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::appchainId(_) => {
                    <appchainIdCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::processTransaction(_) => {
                    <processTransactionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::processTransactionUncompressed(_) => {
                    <processTransactionUncompressedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::processTransactionsBulk(_) => {
                    <processTransactionsBulkCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls>] = &[
                {
                    fn getCurrentPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <getCurrentPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::getCurrentPeriod)
                    }
                    getCurrentPeriod
                },
                {
                    fn isGasTrackingInitialized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <isGasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::isGasTrackingInitialized)
                    }
                    isGasTrackingInitialized
                },
                {
                    fn prependZeroByte(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <prependZeroByteCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::prependZeroByte)
                    }
                    prependZeroByte
                },
                {
                    fn gasTrackingInitialized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <gasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::gasTrackingInitialized)
                    }
                    gasTrackingInitialized
                },
                {
                    fn processTransaction(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <processTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::processTransaction)
                    }
                    processTransaction
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <getPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::getPeriod)
                    }
                    getPeriod
                },
                {
                    fn disableGasTracking(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <disableGasTrackingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::disableGasTracking)
                    }
                    disableGasTracking
                },
                {
                    fn permissionRequirementModule(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <permissionRequirementModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainCalls::permissionRequirementModule,
                            )
                    }
                    permissionRequirementModule
                },
                {
                    fn currentPeriodIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <currentPeriodIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::currentPeriodIndex)
                    }
                    currentPeriodIndex
                },
                {
                    fn PERIOD_DURATION(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::PERIOD_DURATION)
                    }
                    PERIOD_DURATION
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn isAllowed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <isAllowedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::isAllowed)
                    }
                    isAllowed
                },
                {
                    fn getCumulativeGasFees(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <getCumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::getCumulativeGasFees)
                    }
                    getCumulativeGasFees
                },
                {
                    fn processTransactionUncompressed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <processTransactionUncompressedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainCalls::processTransactionUncompressed,
                            )
                    }
                    processTransactionUncompressed
                },
                {
                    fn getCurrentPeriodTimeRemaining(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <getCurrentPeriodTimeRemainingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainCalls::getCurrentPeriodTimeRemaining,
                            )
                    }
                    getCurrentPeriodTimeRemaining
                },
                {
                    fn gasTrackingEnabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <gasTrackingEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::gasTrackingEnabled)
                    }
                    gasTrackingEnabled
                },
                {
                    fn getTotalGasFees(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <getTotalGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::getTotalGasFees)
                    }
                    getTotalGasFees
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::owner)
                    }
                    owner
                },
                {
                    fn getTotalPeriods(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <getTotalPeriodsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::getTotalPeriods)
                    }
                    getTotalPeriods
                },
                {
                    fn getCurrentPeriodGasUsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <getCurrentPeriodGasUsedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::getCurrentPeriodGasUsed)
                    }
                    getCurrentPeriodGasUsed
                },
                {
                    fn processTransactionsBulk(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <processTransactionsBulkCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::processTransactionsBulk)
                    }
                    processTransactionsBulk
                },
                {
                    fn updateRequirementModule(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <updateRequirementModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::updateRequirementModule)
                    }
                    updateRequirementModule
                },
                {
                    fn appchainId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <appchainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::appchainId)
                    }
                    appchainId
                },
                {
                    fn enableGasTracking(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <enableGasTrackingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::enableGasTracking)
                    }
                    enableGasTracking
                },
                {
                    fn periods(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <periodsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::periods)
                    }
                    periods
                },
                {
                    fn TRACKING_OVERHEAD(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <TRACKING_OVERHEADCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::TRACKING_OVERHEAD)
                    }
                    TRACKING_OVERHEAD
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn getGasFeesInRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <getGasFeesInRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::getGasFeesInRange)
                    }
                    getGasFeesInRange
                },
                {
                    fn cumulativeGasFees(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <cumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::cumulativeGasFees)
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
                Self::PERIOD_DURATION(inner) => {
                    <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::processTransaction(inner) => {
                    <processTransactionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::processTransactionUncompressed(inner) => {
                    <processTransactionUncompressedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::processTransactionsBulk(inner) => {
                    <processTransactionsBulkCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::processTransaction(inner) => {
                    <processTransactionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::processTransactionUncompressed(inner) => {
                    <processTransactionUncompressedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::processTransactionsBulk(inner) => {
                    <processTransactionsBulkCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`SyndicateSequencingChain`](self) custom errors.
    pub enum SyndicateSequencingChainErrors {
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
    impl SyndicateSequencingChainErrors {
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
    impl alloy_sol_types::SolInterface for SyndicateSequencingChainErrors {
        const NAME: &'static str = "SyndicateSequencingChainErrors";
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
            ) -> alloy_sol_types::Result<SyndicateSequencingChainErrors>] = &[
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainErrors::OwnableUnauthorizedAccount,
                            )
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn InvalidModuleAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainErrors> {
                        <InvalidModuleAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainErrors::InvalidModuleAddress)
                    }
                    InvalidModuleAddress
                },
                {
                    fn TransactionOrSenderNotAllowed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainErrors> {
                        <TransactionOrSenderNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateSequencingChainErrors::TransactionOrSenderNotAllowed,
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
    ///Container for all the [`SyndicateSequencingChain`](self) events.
    pub enum SyndicateSequencingChainEvents {
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
        TransactionProcessed(TransactionProcessed),
    }
    #[automatically_derived]
    impl SyndicateSequencingChainEvents {
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
    impl alloy_sol_types::SolEventInterface for SyndicateSequencingChainEvents {
        const NAME: &'static str = "SyndicateSequencingChainEvents";
        const COUNT: usize = 6usize;
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
                    <TransactionProcessed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionProcessed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TransactionProcessed)
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
    impl alloy_sol_types::private::IntoLogData for SyndicateSequencingChainEvents {
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
                Self::TransactionProcessed(inner) => {
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
                Self::TransactionProcessed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SyndicateSequencingChain`](self) contract instance.

See the [wrapper's documentation](`SyndicateSequencingChainInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SyndicateSequencingChainInstance<T, P, N> {
        SyndicateSequencingChainInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<SyndicateSequencingChainInstance<T, P, N>>,
    > {
        SyndicateSequencingChainInstance::<T, P, N>::deploy(provider, _appchainId)
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
        SyndicateSequencingChainInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _appchainId)
    }
    /**A [`SyndicateSequencingChain`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SyndicateSequencingChain`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SyndicateSequencingChainInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SyndicateSequencingChainInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SyndicateSequencingChainInstance")
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
    > SyndicateSequencingChainInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`SyndicateSequencingChain`](self) contract instance.

See the [wrapper's documentation](`SyndicateSequencingChainInstance`) for more details.*/
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
        ) -> alloy_contract::Result<SyndicateSequencingChainInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> SyndicateSequencingChainInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SyndicateSequencingChainInstance<T, P, N> {
            SyndicateSequencingChainInstance {
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
    > SyndicateSequencingChainInstance<T, P, N> {
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
        ///Creates a new call builder for the [`processTransaction`] function.
        pub fn processTransaction(
            &self,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, processTransactionCall, N> {
            self.call_builder(&processTransactionCall { data })
        }
        ///Creates a new call builder for the [`processTransactionUncompressed`] function.
        pub fn processTransactionUncompressed(
            &self,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            processTransactionUncompressedCall,
            N,
        > {
            self.call_builder(
                &processTransactionUncompressedCall {
                    data,
                },
            )
        }
        ///Creates a new call builder for the [`processTransactionsBulk`] function.
        pub fn processTransactionsBulk(
            &self,
            data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        ) -> alloy_contract::SolCallBuilder<T, &P, processTransactionsBulkCall, N> {
            self.call_builder(
                &processTransactionsBulkCall {
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
    > SyndicateSequencingChainInstance<T, P, N> {
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
        ///Creates a new event filter for the [`TransactionProcessed`] event.
        pub fn TransactionProcessed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TransactionProcessed, N> {
            self.event_filter::<TransactionProcessed>()
        }
    }
}
