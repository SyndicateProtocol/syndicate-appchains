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
    function periodDuration() external view returns (uint256);
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
    ///0x60a060405234610038576100196100146100e9565b6101b7565b61002161003d565b61240861057c823960805181610b58015261240890f35b610043565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b9061006f90610047565b810190811060018060401b0382111761008757604052565b610051565b9061009f61009861003d565b9283610065565b565b5f80fd5b90565b6100b1816100a5565b036100b857565b5f80fd5b905051906100c9826100a8565b565b906020828203126100e4576100e1915f016100bc565b90565b6100a1565b610107612b43803803806100fc8161008c565b9283398101906100cb565b90565b90565b90565b61012461011f6101299261010a565b61010d565b6100a5565b90565b60209181520190565b5f7f41707020636861696e2049442063616e6e6f7420626520300000000000000000910152565b610169601860209261012c565b61017281610135565b0190565b61018b9060208101905f81830391015261015c565b90565b1561019557565b61019d61003d565b62461bcd60e51b8152806101b360048201610176565b0390fd5b6101bf61028f565b6101dc816101d56101cf5f610110565b916100a5565b141561018e565b608052565b5f1b90565b906101f25f19916101e1565b9181191691161790565b90565b61021361020e610218926101fc565b61010d565b6100a5565b90565b90565b9061023361022e61023a926101ff565b61021b565b82546101e6565b9055565b60081b90565b9061025161ff009161023e565b9181191691161790565b151590565b6102699061025b565b90565b90565b9061028461027f61028b92610260565b61026c565b8254610244565b9055565b610297610390565b6102a562278d00600261021e565b6102b16001600461026f565b565b60a01b90565b906102c860ff60a01b916102b3565b9181191691161790565b906102e76102e26102ee92610260565b61026c565b82546102b9565b9055565b5f0190565b6102ff61003d565b3d5f823e3d90fd5b60018060a01b031690565b61032661032161032b92610307565b61010d565b610307565b90565b61033790610312565b90565b6103439061032e565b90565b9061035760018060a01b03916101e1565b9181191691161790565b61036a9061032e565b90565b90565b9061038561038061038c92610361565b61036d565b8254610346565b9055565b610399336103fd565b6103a45f60016102d2565b6103ac61003d565b6101bf810181811060018060401b038211176103f8576103d482916101bf61298484396102f2565b03905ff080156103f3576103ea6103f19161033a565b6001610370565b565b6102f7565b610051565b6104069061045e565b565b61041c6104176104219261010a565b61010d565b610307565b90565b61042d90610408565b90565b61043990610307565b90565b61044590610430565b9052565b919061045c905f6020850194019061043c565b565b8061047961047361046e5f610424565b610430565b91610430565b14610489576104879061051c565b565b6104ac6104955f610424565b5f918291631e4fbdf760e01b835260048301610449565b0390fd5b5f1c90565b60018060a01b031690565b6104cc6104d1916104b0565b6104b5565b90565b6104de90546104c0565b90565b6104ea90610312565b90565b6104f6906104e1565b90565b90565b9061051161050c610518926104ed565b6104f9565b8254610346565b9055565b6105255f6104d4565b61052f825f6104fc565b9061056361055d7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0936104ed565b916104ed565b9161056c61003d565b80610576816102f2565b0390a356fe60806040526004361015610013575b610dcb565b61001d5f356101ec565b8063086146d2146101e757806318d5aafe146101e2578063366cbab7146101dd5780633b6ab2a9146101d857806346e2cc09146101d3578063485cc955146101ce5780634b2c0706146101c95780635467cb48146101c45780635b3cd6e2146101bf57806361543801146101ba578063715018a6146101b55780637a3979dc146101b05780637fbd295e146101ab578063804e5123146101a657806382f44ade146101a157806384fab62b1461019c5780638d5a239b146101975780638da5cb5b14610192578063aff74c6d1461018d578063b470aade14610188578063c660d3f314610183578063cdafb9781461017e578063d4f0eb4d14610179578063d878134214610174578063de1f453e1461016f578063ea4a11041461016a578063f2fde38b14610165578063f7b8935e146101605763ff7b30840361000e57610d96565b610d51565b610cf1565b610cb8565b610baf565b610b7a565b610b23565b610ad1565b610a27565b6109f2565b6109ae565b610979565b610922565b6108ed565b6108a8565b610874565b61083f565b610806565b610781565b61074c565b6106bd565b6105f1565b6105bc565b610547565b6104ac565b610472565b6103fd565b6102d8565b61027c565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f91031261020a57565b6101fc565b90565b61021b9061020f565b9052565b90606080610265936102375f8201515f860190610212565b61024960208201516020860190610212565b61025b60408201516040860190610212565b0151910190610212565b565b919061027a905f6080850194019061021f565b565b346102ac5761028c366004610200565b6102a8610297610e7b565b61029f6101f2565b91829182610267565b0390f35b6101f8565b151590565b6102bf906102b1565b9052565b91906102d6905f602085019401906102b6565b565b34610308576102e8366004610200565b6103046102f3610eb4565b6102fb6101f2565b918291826102c3565b0390f35b6101f8565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156103575781359167ffffffffffffffff831161035257602001926001830284011161034d57565b610319565b610315565b610311565b9060208282031261038d575f82013567ffffffffffffffff811161038857610384920161031d565b9091565b61030d565b6101fc565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6103d36103dc6020936103e1936103ca81610392565b93848093610396565b9586910161039f565b6103aa565b0190565b6103fa9160208201915f8184039101526103b4565b90565b3461042e5761042a61041961041336600461035c565b90610f5b565b6104216101f2565b918291826103e5565b0390f35b6101f8565b1c90565b60ff1690565b61044d9060086104529302610433565b610437565b90565b90610460915461043d565b90565b61046f60045f90610455565b90565b346104a257610482366004610200565b61049e61048d610463565b6104956101f2565b918291826102c3565b0390f35b6101f8565b5f0190565b346104db576104c56104bf36600461035c565b90611116565b6104cd6101f2565b806104d7816104a7565b0390f35b6101f8565b60018060a01b031690565b6104f4906104e0565b90565b610500816104eb565b0361050757565b5f80fd5b90503590610518826104f7565b565b9190604083820312610542578061053661053f925f860161050b565b9360200161050b565b90565b6101fc565b346105765761056061055a36600461051a565b906112c7565b6105686101f2565b80610572816104a7565b0390f35b6101f8565b6105848161020f565b0361058b57565b5f80fd5b9050359061059c8261057b565b565b906020828203126105b7576105b4915f0161058f565b90565b6101fc565b346105ec576105e86105d76105d236600461059e565b611356565b6105df6101f2565b91829182610267565b0390f35b6101f8565b3461061f57610601366004610200565b610609611391565b6106116101f2565b8061061b816104a7565b0390f35b6101f8565b60018060a01b031690565b61063f9060086106449302610433565b610624565b90565b90610652915461062f565b90565b61066160015f90610647565b90565b90565b61067b610676610680926104e0565b610664565b6104e0565b90565b61068c90610667565b90565b61069890610683565b90565b6106a49061068f565b9052565b91906106bb905f6020850194019061069b565b565b346106ed576106cd366004610200565b6106e96106d8610655565b6106e06101f2565b918291826106a8565b0390f35b6101f8565b90565b61070590600861070a9302610433565b6106f2565b90565b9061071891546106f5565b90565b61072760035f9061070d565b90565b6107339061020f565b9052565b919061074a905f6020850194019061072a565b565b3461077c5761075c366004610200565b61077861076761071b565b61076f6101f2565b91829182610737565b0390f35b6101f8565b346107af57610791366004610200565b6107996113c0565b6107a16101f2565b806107ab816104a7565b0390f35b6101f8565b91606083830312610801576107cb825f850161050b565b926107d9836020830161050b565b92604082013567ffffffffffffffff81116107fc576107f8920161031d565b9091565b61030d565b6101fc565b3461083a5761083661082561081c3660046107b4565b92919091611478565b61082d6101f2565b918291826102c3565b0390f35b6101f8565b3461086f5761084f366004610200565b61086b61085a611545565b6108626101f2565b91829182610737565b0390f35b6101f8565b346108a35761088d61088736600461035c565b90611652565b6108956101f2565b8061089f816104a7565b0390f35b6101f8565b346108d8576108b8366004610200565b6108d46108c361167a565b6108cb6101f2565b91829182610737565b0390f35b6101f8565b6108ea6004600190610455565b90565b3461091d576108fd366004610200565b6109196109086108dd565b6109106101f2565b918291826102c3565b0390f35b6101f8565b3461095257610932366004610200565b61094e61093d61170c565b6109456101f2565b91829182610737565b0390f35b6101f8565b610960906104eb565b9052565b9190610977905f60208501940190610957565b565b346109a957610989366004610200565b6109a561099461175b565b61099c6101f2565b91829182610964565b0390f35b6101f8565b346109de576109be366004610200565b6109da6109c961178f565b6109d16101f2565b91829182610737565b0390f35b6101f8565b6109ef60025f9061070d565b90565b34610a2257610a02366004610200565b610a1e610a0d6109e3565b610a156101f2565b91829182610737565b0390f35b6101f8565b34610a5757610a37366004610200565b610a53610a426117db565b610a4a6101f2565b91829182610737565b0390f35b6101f8565b909182601f83011215610a965781359167ffffffffffffffff8311610a91576020019260208302840111610a8c57565b610319565b610315565b610311565b90602082820312610acc575f82013567ffffffffffffffff8111610ac757610ac39201610a5c565b9091565b61030d565b6101fc565b34610b0057610aea610ae4366004610a9b565b906119a0565b610af26101f2565b80610afc816104a7565b0390f35b6101f8565b90602082820312610b1e57610b1b915f0161050b565b90565b6101fc565b34610b5157610b3b610b36366004610b05565b611a50565b610b436101f2565b80610b4d816104a7565b0390f35b6101f8565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610baa57610b8a366004610200565b610ba6610b95610b56565b610b9d6101f2565b91829182610737565b0390f35b6101f8565b34610bdd57610bbf366004610200565b610bc7611a77565b610bcf6101f2565b80610bd9816104a7565b0390f35b6101f8565b610bf6610bf1610bfb9261020f565b610664565b61020f565b90565b90610c0890610be2565b5f5260205260405f2090565b5f1c90565b610c25610c2a91610c14565b6106f2565b90565b610c379054610c19565b90565b610c45906005610bfe565b90610c515f8301610c2d565b91610c5e60018201610c2d565b91610c776003610c7060028501610c2d565b9301610c2d565b90565b610caf610cb694610ca5606094989795610c9b608086019a5f87019061072a565b602085019061072a565b604083019061072a565b019061072a565b565b34610cec57610ce8610cd3610cce36600461059e565b610c3a565b90610cdf9492946101f2565b94859485610c7a565b0390f35b6101f8565b34610d1f57610d09610d04366004610b05565b611ae6565b610d116101f2565b80610d1b816104a7565b0390f35b6101f8565b9190604083820312610d4c5780610d40610d49925f860161058f565b9360200161058f565b90565b6101fc565b34610d8257610d7e610d6d610d67366004610d24565b90611b7c565b610d756101f2565b91829182610737565b0390f35b6101f8565b610d9360065f9061070d565b90565b34610dc657610da6366004610200565b610dc2610db1610d87565b610db96101f2565b91829182610737565b0390f35b6101f8565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b90610ded906103aa565b810190811067ffffffffffffffff821117610e0757604052565b610dcf565b90610e1f610e186101f2565b9283610de3565b565b610e2b6080610e0c565b90565b5f90565b610e3a610e21565b90602080808085610e49610e2e565b815201610e54610e2e565b815201610e5f610e2e565b815201610e6a610e2e565b81525050565b610e78610e32565b90565b610e83610e70565b50610e8c611c26565b90565b5f90565b610e9f610ea491610c14565b610437565b90565b610eb19054610e93565b90565b610ebc610e8f565b50610ec76004610ea7565b90565b606090565b90565b60ff60f81b1690565b60f81b90565b610ef5610ef0610efa92610ecf565b610edb565b610ed2565b90565b90565b610f0c610f1191610ed2565b610efd565b9052565b905090565b90825f939282370152565b909182610f3581610f3c93610f15565b8093610f1a565b0190565b80610f51600192610f589694610f00565b0191610f25565b90565b610f9990610f67610eca565b50610f8a610f745f610ee1565b9193610f7e6101f2565b94859360208501610f40565b60208201810382520382610de3565b90565b90610fb8610fb233329085859192909192611478565b156102b1565b610fc757610fc591611043565b565b5f631b8e828b60e31b815280610fdf600482016104a7565b0390fd5b60081c90565b610ff5610ffa91610fe3565b610437565b90565b6110079054610fe9565b90565b634e487b7160e01b5f52601160045260245ffd5b61102d6110339193929361020f565b9261020f565b820391821161103e57565b61100a565b906110576110516004610ffd565b156102b1565b61107b576110799161106d611074925a926110cf565b5a9061101e565b611e00565b565b611084916110cf565b565b61108f90610683565b90565b91906110ac816110a5816110b195610396565b8095610f1a565b6103aa565b0190565b90916110cc9260208301925f818503910152611092565b90565b3390916110fc7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611086565b926111116111086101f2565b928392836110b5565b0390a2565b9061112091610f9c565b565b906111349161112f611f07565b61123a565b565b60a01c90565b61114861114d91611136565b610437565b90565b61115a905461113c565b90565b61117161116c61117692610ecf565b610664565b6104e0565b90565b6111829061115d565b90565b60a01b90565b9061119a60ff60a01b91611185565b9181191691161790565b6111ad906102b1565b90565b90565b906111c86111c36111cf926111a4565b6111b0565b825461118b565b9055565b6111dc90610667565b90565b6111e8906111d3565b90565b5f1b90565b9061120160018060a01b03916111eb565b9181191691161790565b611214906111d3565b90565b90565b9061122f61122a6112369261120b565b611217565b82546111f0565b9055565b6112446001611150565b6112ac578161126361125d6112585f611179565b6104eb565b916104eb565b146112905761128961128261128e9361127d6001806111b3565b6111df565b600161121a565b611ae6565b565b5f632e7f3c7f60e11b8152806112a8600482016104a7565b0390fd5b5f62dc149f60e41b8152806112c3600482016104a7565b0390fd5b906112d191611122565b565b906112dd9061020f565b9052565b9061134861133f60036112f2610e21565b946113096113015f8301610c2d565b5f88016112d3565b61132161131860018301610c2d565b602088016112d3565b61133961133060028301610c2d565b604088016112d3565b01610c2d565b606084016112d3565b565b611353906112e1565b90565b61136d61137291611365610e70565b506005610bfe565b61134a565b90565b61137d611f07565b611385611387565b565b61138f611f92565b565b611399611375565b565b6113a3611f07565b6113ab6113ad565b565b6113be6113b95f611179565b611fc2565b565b6113c861139b565b565b6113d66113db91610c14565b610624565b90565b6113e890546113ca565b90565b60e01b90565b6113fa816102b1565b0361140157565b5f80fd5b90505190611412826113f1565b565b9060208282031261142d5761142a915f01611405565b90565b6101fc565b611458611465959394929461144e60608401965f850190610957565b6020830190610957565b6040818503910152611092565b90565b6114706101f2565b3d5f823e3d90fd5b926114bb60209394611488610e8f565b506114c661149e61149960016113de565b61068f565b93637a3979dc9295976114af6101f2565b988997889687966113eb565b865260048601611432565b03915afa90811561150a575f916114dc575b5090565b6114fd915060203d8111611503575b6114f58183610de3565b810190611414565b5f6114d8565b503d6114eb565b611468565b5f90565b61151d905161020f565b90565b61152f6115359193929361020f565b9261020f565b820180921161154057565b61100a565b61154d61150f565b5061157461155b6006610c2d565b61156e6060611568611c26565b01611513565b90611520565b90565b9061159361158d33329085859192909192611478565b156102b1565b6115a2576115a0916115be565b565b5f631b8e828b60e31b8152806115ba600482016104a7565b0390fd5b906115d26115cc6004610ffd565b156102b1565b6115f6576115f4916115e86115ef925a92611601565b5a9061101e565b611e00565b565b6115ff91611601565b565b9061160d903392610f5b565b9061164d61163b7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611086565b926116446101f2565b918291826103e5565b0390a2565b9061165c91611577565b565b61167261166d61167792610ecf565b610664565b61020f565b90565b61168261150f565b5061168b611c26565b6116965f8201611513565b6116a86116a25f61165e565b9161020f565b146116ff576116bb5f6116cb9201611513565b6116c56002610c2d565b90611520565b426116de6116d88361020f565b9161020f565b10156116f2576116ef90429061101e565b90565b506116fc5f61165e565b90565b506117095f61165e565b90565b61171461150f565b506117286060611722611c26565b01611513565b90565b5f90565b60018060a01b031690565b61174661174b91610c14565b61172f565b90565b611758905461173a565b90565b61176361172b565b5061176d5f61174e565b90565b90565b61178761178261178c92611770565b610664565b61020f565b90565b61179761150f565b506117ab6117a56004610ea7565b156102b1565b6117cf576117cc6117bc6003610c2d565b6117c66001611773565b90611520565b90565b6117d85f61165e565b90565b6117e361150f565b506117f760406117f1611c26565b01611513565b90565b9061180e6118086004610ffd565b156102b1565b611832576118309161182461182b925a926118d7565b5a9061101e565b611e00565b565b61183b916118d7565b565b5090565b600161184d910161020f565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b9035906001602003813603038212156118b2570180359067ffffffffffffffff82116118ad576020019160018202360383136118a857565b61186c565b611868565b611864565b908210156118d25760206118ce9202810190611870565b9091565b611850565b6118e281839061183d565b916118eb61150f565b506118f55f61165e565b5b806119096119038661020f565b9161020f565b101561199a576119379061192d333290611925878786916118b7565b929091611478565b61193c575b611841565b6118f6565b3361195261194c868685916118b7565b90610f5b565b906119926119807f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611086565b926119896101f2565b918291826103e5565b0390a2611932565b50505050565b906119aa916117fa565b565b6119bd906119b8611f07565b6119bf565b565b806119da6119d46119cf5f611179565b6104eb565b916104eb565b14611a34576119f26119eb826111df565b600161121a565b611a1c7f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b991611086565b90611a256101f2565b80611a2f816104a7565b0390a2565b5f632e7f3c7f60e11b815280611a4c600482016104a7565b0390fd5b611a59906119ac565b565b611a63611f07565b611a6b611a6d565b565b611a75612021565b565b611a7f611a5b565b565b611a9290611a8d611f07565b611a94565b565b80611aaf611aa9611aa45f611179565b6104eb565b916104eb565b14611abf57611abd90611fc2565b565b611ae2611acb5f611179565b5f918291631e4fbdf760e01b835260048301610964565b0390fd5b611aef90611a81565b565b60209181520190565b5f7f476173436f756e7465723a20696e76616c69642072616e676500000000000000910152565b611b2e6019602092611af1565b611b3781611afa565b0190565b611b509060208101905f818303910152611b21565b90565b15611b5a57565b611b626101f2565b62461bcd60e51b815280611b7860048201611b3b565b0390fd5b611bab91611b8861150f565b50611ba681611b9f611b998561020f565b9161020f565b1015611b53565b61101e565b90565b611bb86080610e0c565b90565b634e487b7160e01b5f52601260045260245ffd5b611bdb611be19161020f565b9161020f565b908115611bec570490565b611bbb565b611c00611c069193929361020f565b9261020f565b91611c1283820261020f565b928184041490151715611c2157565b61100a565b611c2e610e70565b50611c42611c3c6004610ea7565b156102b1565b611d4457611c63611c5e6005611c586003610c2d565b90610bfe565b61134a565b42611c93611c8d611c88611c785f8601611513565b611c826002610c2d565b90611520565b61020f565b9161020f565b1015611c9c5790565b611ced90611ce7611cd65f611ccf611cbf42611cb9848801611513565b9061101e565b611cc96002610c2d565b90611bcf565b9301611513565b91611ce16002610c2d565b90611bf1565b90611520565b611d41611d385f611d33611d2a5f611d25611d1c5f95611d17611d0e611bae565b9a5f8c016112d3565b61165e565b602089016112d3565b61165e565b604086016112d3565b61165e565b606083016112d3565b90565b5f611da1611d985f611d93611d8a5f611d85611d7c5f95611d77611d6f611d69611bae565b9b61165e565b5f8c016112d3565b61165e565b602089016112d3565b61165e565b604086016112d3565b61165e565b606083016112d3565b90565b90611db05f19916111eb565b9181191691161790565b90565b90611dd2611dcd611dd992610be2565b611dba565b8254611da4565b9055565b916020611dfe929493611df760408201965f83019061072a565b019061072a565b565b611e13611e0d6004610ffd565b156102b1565b611f0457611e2a611e246004610ea7565b156102b1565b611ef7575b611e376121f7565b611ea8611e45823a90611bf1565b611e7983611e736002611e636005611e5d6003610c2d565b90610bfe565b0191611e6e83610c2d565b611520565b90611dbd565b611ea26003611e926005611e8c83610c2d565b90610bfe565b0191611e9d83610c2d565b611520565b90611dbd565b611eb26003610c2d565b3a611edd7f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610be2565b92611ef2611ee96101f2565b92839283611ddd565b0390a2565b611eff6120ec565b611e2f565b50565b611f0f61175b565b611f28611f22611f1d6123fb565b6104eb565b916104eb565b03611f2f57565b611f51611f3a6123fb565b5f91829163118cdaa760e01b835260048301610964565b0390fd5b60081b90565b90611f6861ff0091611f55565b9181191691161790565b90611f87611f82611f8e926111a4565b6111b0565b8254611f5b565b9055565b611f9d5f6004611f72565b565b90565b90611fb7611fb2611fbe92611086565b611f9f565b82546111f0565b9055565b611fcb5f61174e565b611fd5825f611fa2565b906120096120037f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093611086565b91611086565b916120126101f2565b8061201c816104a7565b0390a3565b61202d60016004611f72565b565b9061203b60ff916111eb565b9181191691161790565b9061205a612055612061926111a4565b6111b0565b825461202f565b9055565b9061206f9061165e565b5f5260205260405f2090565b906120d8606060036120de9461209e5f82016120985f8801611513565b90611dbd565b6120b7600182016120b160208801611513565b90611dbd565b6120d0600282016120ca60408801611513565b90611dbd565b019201611513565b90611dbd565b565b906120ea9161207b565b565b6120ff6120f96004610ea7565b156102b1565b612106575b565b61211260016004612045565b61212561211e5f61165e565b6003611dbd565b61218e4261217d6121745f61216f6121665f6121616121585f9561215361214a611bae565b9a5f8c016112d3565b61165e565b602089016112d3565b61165e565b604086016112d3565b61165e565b606083016112d3565b61218960055f90612065565b6120e0565b5f42906121d06121be7f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e9261165e565b926121c76101f2565b91829182610737565b0390a2612104565b90565b6121e49061020f565b5f1981146121f25760010190565b61100a565b61221461220f60056122096003610c2d565b90610bfe565b6121d8565b4261224461223e6122396122295f8601610c2d565b6122336002610c2d565b90611520565b61020f565b9161020f565b101561224e575b50565b61227861226f61225f5f8401610c2d565b6122696002610c2d565b90611520565b60018301611dbd565b6122a061229961228a60038401610c2d565b6122946006610c2d565b611520565b6006611dbd565b6122aa6003610c2d565b6122d76122b960028401610c2d565b926122d15f6122ca60018401610c2d565b9201610c2d565b9061101e565b6123017f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610be2565b9261231661230d6101f2565b92839283611ddd565b0390a261233561232e6123296003610c2d565b6121db565b6003611dbd565b6123a74261238d6123845f61237f6123765f6123716123685f9561236361235a611bae565b9a5f8c016112d3565b61165e565b602089016112d3565b61165e565b604086016112d3565b61165e565b606083016112d3565b6123a2600561239c6003610c2d565b90610bfe565b6120e0565b6123b16003610c2d565b42906123f26123e07f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610be2565b926123e96101f2565b91829182610737565b0390a25f61224b565b61240361172b565b50339056608060405234601c57600e6020565b61019461002b823961019490f35b6026565b60405190565b5f80fdfe60806040526004361015610013575b61017b565b61001d5f3561002c565b637a3979dc0361000e57610142565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b60018060a01b031690565b61005890610044565b90565b6100648161004f565b0361006b57565b5f80fd5b9050359061007c8261005b565b565b5f80fd5b5f80fd5b5f80fd5b909182601f830112156100c45781359167ffffffffffffffff83116100bf5760200192600183028401116100ba57565b610086565b610082565b61007e565b91606083830312610116576100e0825f850161006f565b926100ee836020830161006f565b92604082013567ffffffffffffffff81116101115761010d920161008a565b9091565b610040565b61003c565b151590565b6101299061011b565b9052565b9190610140905f60208501940190610120565b565b34610176576101726101616101583660046100c9565b92919091610183565b610169610032565b9182918261012d565b0390f35b610038565b5f80fd5b5f90565b5050505061018f61017f565b505f9056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4a\08Wa\0\x19a\0\x14a\0\xE9V[a\x01\xB7V[a\0!a\0=V[a$\x08a\x05|\x829`\x80Q\x81a\x0BX\x01Ra$\x08\x90\xF3[a\0CV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0o\x90a\0GV[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x87W`@RV[a\0QV[\x90a\0\x9Fa\0\x98a\0=V[\x92\x83a\0eV[V[_\x80\xFD[\x90V[a\0\xB1\x81a\0\xA5V[\x03a\0\xB8WV[_\x80\xFD[\x90PQ\x90a\0\xC9\x82a\0\xA8V[V[\x90` \x82\x82\x03\x12a\0\xE4Wa\0\xE1\x91_\x01a\0\xBCV[\x90V[a\0\xA1V[a\x01\x07a+C\x808\x03\x80a\0\xFC\x81a\0\x8CV[\x92\x839\x81\x01\x90a\0\xCBV[\x90V[\x90V[\x90V[a\x01$a\x01\x1Fa\x01)\x92a\x01\nV[a\x01\rV[a\0\xA5V[\x90V[` \x91\x81R\x01\x90V[_\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01i`\x18` \x92a\x01,V[a\x01r\x81a\x015V[\x01\x90V[a\x01\x8B\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x01\\V[\x90V[\x15a\x01\x95WV[a\x01\x9Da\0=V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x01\xB3`\x04\x82\x01a\x01vV[\x03\x90\xFD[a\x01\xBFa\x02\x8FV[a\x01\xDC\x81a\x01\xD5a\x01\xCF_a\x01\x10V[\x91a\0\xA5V[\x14\x15a\x01\x8EV[`\x80RV[_\x1B\x90V[\x90a\x01\xF2_\x19\x91a\x01\xE1V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[a\x02\x13a\x02\x0Ea\x02\x18\x92a\x01\xFCV[a\x01\rV[a\0\xA5V[\x90V[\x90V[\x90a\x023a\x02.a\x02:\x92a\x01\xFFV[a\x02\x1BV[\x82Ta\x01\xE6V[\x90UV[`\x08\x1B\x90V[\x90a\x02Qa\xFF\0\x91a\x02>V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x15\x15\x90V[a\x02i\x90a\x02[V[\x90V[\x90V[\x90a\x02\x84a\x02\x7Fa\x02\x8B\x92a\x02`V[a\x02lV[\x82Ta\x02DV[\x90UV[a\x02\x97a\x03\x90V[a\x02\xA5b'\x8D\0`\x02a\x02\x1EV[a\x02\xB1`\x01`\x04a\x02oV[V[`\xA0\x1B\x90V[\x90a\x02\xC8`\xFF`\xA0\x1B\x91a\x02\xB3V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x02\xE7a\x02\xE2a\x02\xEE\x92a\x02`V[a\x02lV[\x82Ta\x02\xB9V[\x90UV[_\x01\x90V[a\x02\xFFa\0=V[=_\x82>=\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x03&a\x03!a\x03+\x92a\x03\x07V[a\x01\rV[a\x03\x07V[\x90V[a\x037\x90a\x03\x12V[\x90V[a\x03C\x90a\x03.V[\x90V[\x90a\x03W`\x01\x80`\xA0\x1B\x03\x91a\x01\xE1V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x03j\x90a\x03.V[\x90V[\x90V[\x90a\x03\x85a\x03\x80a\x03\x8C\x92a\x03aV[a\x03mV[\x82Ta\x03FV[\x90UV[a\x03\x993a\x03\xFDV[a\x03\xA4_`\x01a\x02\xD2V[a\x03\xACa\0=V[a\x01\xBF\x81\x01\x81\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\x03\xF8Wa\x03\xD4\x82\x91a\x01\xBFa)\x84\x849a\x02\xF2V[\x03\x90_\xF0\x80\x15a\x03\xF3Wa\x03\xEAa\x03\xF1\x91a\x03:V[`\x01a\x03pV[V[a\x02\xF7V[a\0QV[a\x04\x06\x90a\x04^V[V[a\x04\x1Ca\x04\x17a\x04!\x92a\x01\nV[a\x01\rV[a\x03\x07V[\x90V[a\x04-\x90a\x04\x08V[\x90V[a\x049\x90a\x03\x07V[\x90V[a\x04E\x90a\x040V[\x90RV[\x91\x90a\x04\\\x90_` \x85\x01\x94\x01\x90a\x04<V[V[\x80a\x04ya\x04sa\x04n_a\x04$V[a\x040V[\x91a\x040V[\x14a\x04\x89Wa\x04\x87\x90a\x05\x1CV[V[a\x04\xACa\x04\x95_a\x04$V[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x04IV[\x03\x90\xFD[_\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\xCCa\x04\xD1\x91a\x04\xB0V[a\x04\xB5V[\x90V[a\x04\xDE\x90Ta\x04\xC0V[\x90V[a\x04\xEA\x90a\x03\x12V[\x90V[a\x04\xF6\x90a\x04\xE1V[\x90V[\x90V[\x90a\x05\x11a\x05\x0Ca\x05\x18\x92a\x04\xEDV[a\x04\xF9V[\x82Ta\x03FV[\x90UV[a\x05%_a\x04\xD4V[a\x05/\x82_a\x04\xFCV[\x90a\x05ca\x05]\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x04\xEDV[\x91a\x04\xEDV[\x91a\x05la\0=V[\x80a\x05v\x81a\x02\xF2V[\x03\x90\xA3V\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\r\xCBV[a\0\x1D_5a\x01\xECV[\x80c\x08aF\xD2\x14a\x01\xE7W\x80c\x18\xD5\xAA\xFE\x14a\x01\xE2W\x80c6l\xBA\xB7\x14a\x01\xDDW\x80c;j\xB2\xA9\x14a\x01\xD8W\x80cF\xE2\xCC\t\x14a\x01\xD3W\x80cH\\\xC9U\x14a\x01\xCEW\x80cK,\x07\x06\x14a\x01\xC9W\x80cTg\xCBH\x14a\x01\xC4W\x80c[<\xD6\xE2\x14a\x01\xBFW\x80caT8\x01\x14a\x01\xBAW\x80cqP\x18\xA6\x14a\x01\xB5W\x80cz9y\xDC\x14a\x01\xB0W\x80c\x7F\xBD)^\x14a\x01\xABW\x80c\x80NQ#\x14a\x01\xA6W\x80c\x82\xF4J\xDE\x14a\x01\xA1W\x80c\x84\xFA\xB6+\x14a\x01\x9CW\x80c\x8DZ#\x9B\x14a\x01\x97W\x80c\x8D\xA5\xCB[\x14a\x01\x92W\x80c\xAF\xF7Lm\x14a\x01\x8DW\x80c\xB4p\xAA\xDE\x14a\x01\x88W\x80c\xC6`\xD3\xF3\x14a\x01\x83W\x80c\xCD\xAF\xB9x\x14a\x01~W\x80c\xD4\xF0\xEBM\x14a\x01yW\x80c\xD8x\x13B\x14a\x01tW\x80c\xDE\x1FE>\x14a\x01oW\x80c\xEAJ\x11\x04\x14a\x01jW\x80c\xF2\xFD\xE3\x8B\x14a\x01eW\x80c\xF7\xB8\x93^\x14a\x01`Wc\xFF{0\x84\x03a\0\x0EWa\r\x96V[a\rQV[a\x0C\xF1V[a\x0C\xB8V[a\x0B\xAFV[a\x0BzV[a\x0B#V[a\n\xD1V[a\n'V[a\t\xF2V[a\t\xAEV[a\tyV[a\t\"V[a\x08\xEDV[a\x08\xA8V[a\x08tV[a\x08?V[a\x08\x06V[a\x07\x81V[a\x07LV[a\x06\xBDV[a\x05\xF1V[a\x05\xBCV[a\x05GV[a\x04\xACV[a\x04rV[a\x03\xFDV[a\x02\xD8V[a\x02|V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x02\nWV[a\x01\xFCV[\x90V[a\x02\x1B\x90a\x02\x0FV[\x90RV[\x90``\x80a\x02e\x93a\x027_\x82\x01Q_\x86\x01\x90a\x02\x12V[a\x02I` \x82\x01Q` \x86\x01\x90a\x02\x12V[a\x02[`@\x82\x01Q`@\x86\x01\x90a\x02\x12V[\x01Q\x91\x01\x90a\x02\x12V[V[\x91\x90a\x02z\x90_`\x80\x85\x01\x94\x01\x90a\x02\x1FV[V[4a\x02\xACWa\x02\x8C6`\x04a\x02\0V[a\x02\xA8a\x02\x97a\x0E{V[a\x02\x9Fa\x01\xF2V[\x91\x82\x91\x82a\x02gV[\x03\x90\xF3[a\x01\xF8V[\x15\x15\x90V[a\x02\xBF\x90a\x02\xB1V[\x90RV[\x91\x90a\x02\xD6\x90_` \x85\x01\x94\x01\x90a\x02\xB6V[V[4a\x03\x08Wa\x02\xE86`\x04a\x02\0V[a\x03\x04a\x02\xF3a\x0E\xB4V[a\x02\xFBa\x01\xF2V[\x91\x82\x91\x82a\x02\xC3V[\x03\x90\xF3[a\x01\xF8V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03WW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03RW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x03MWV[a\x03\x19V[a\x03\x15V[a\x03\x11V[\x90` \x82\x82\x03\x12a\x03\x8DW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\x88Wa\x03\x84\x92\x01a\x03\x1DV[\x90\x91V[a\x03\rV[a\x01\xFCV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\xD3a\x03\xDC` \x93a\x03\xE1\x93a\x03\xCA\x81a\x03\x92V[\x93\x84\x80\x93a\x03\x96V[\x95\x86\x91\x01a\x03\x9FV[a\x03\xAAV[\x01\x90V[a\x03\xFA\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xB4V[\x90V[4a\x04.Wa\x04*a\x04\x19a\x04\x136`\x04a\x03\\V[\x90a\x0F[V[a\x04!a\x01\xF2V[\x91\x82\x91\x82a\x03\xE5V[\x03\x90\xF3[a\x01\xF8V[\x1C\x90V[`\xFF\x16\x90V[a\x04M\x90`\x08a\x04R\x93\x02a\x043V[a\x047V[\x90V[\x90a\x04`\x91Ta\x04=V[\x90V[a\x04o`\x04_\x90a\x04UV[\x90V[4a\x04\xA2Wa\x04\x826`\x04a\x02\0V[a\x04\x9Ea\x04\x8Da\x04cV[a\x04\x95a\x01\xF2V[\x91\x82\x91\x82a\x02\xC3V[\x03\x90\xF3[a\x01\xF8V[_\x01\x90V[4a\x04\xDBWa\x04\xC5a\x04\xBF6`\x04a\x03\\V[\x90a\x11\x16V[a\x04\xCDa\x01\xF2V[\x80a\x04\xD7\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\xF4\x90a\x04\xE0V[\x90V[a\x05\0\x81a\x04\xEBV[\x03a\x05\x07WV[_\x80\xFD[\x90P5\x90a\x05\x18\x82a\x04\xF7V[V[\x91\x90`@\x83\x82\x03\x12a\x05BW\x80a\x056a\x05?\x92_\x86\x01a\x05\x0BV[\x93` \x01a\x05\x0BV[\x90V[a\x01\xFCV[4a\x05vWa\x05`a\x05Z6`\x04a\x05\x1AV[\x90a\x12\xC7V[a\x05ha\x01\xF2V[\x80a\x05r\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[a\x05\x84\x81a\x02\x0FV[\x03a\x05\x8BWV[_\x80\xFD[\x90P5\x90a\x05\x9C\x82a\x05{V[V[\x90` \x82\x82\x03\x12a\x05\xB7Wa\x05\xB4\x91_\x01a\x05\x8FV[\x90V[a\x01\xFCV[4a\x05\xECWa\x05\xE8a\x05\xD7a\x05\xD26`\x04a\x05\x9EV[a\x13VV[a\x05\xDFa\x01\xF2V[\x91\x82\x91\x82a\x02gV[\x03\x90\xF3[a\x01\xF8V[4a\x06\x1FWa\x06\x016`\x04a\x02\0V[a\x06\ta\x13\x91V[a\x06\x11a\x01\xF2V[\x80a\x06\x1B\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x06?\x90`\x08a\x06D\x93\x02a\x043V[a\x06$V[\x90V[\x90a\x06R\x91Ta\x06/V[\x90V[a\x06a`\x01_\x90a\x06GV[\x90V[\x90V[a\x06{a\x06va\x06\x80\x92a\x04\xE0V[a\x06dV[a\x04\xE0V[\x90V[a\x06\x8C\x90a\x06gV[\x90V[a\x06\x98\x90a\x06\x83V[\x90V[a\x06\xA4\x90a\x06\x8FV[\x90RV[\x91\x90a\x06\xBB\x90_` \x85\x01\x94\x01\x90a\x06\x9BV[V[4a\x06\xEDWa\x06\xCD6`\x04a\x02\0V[a\x06\xE9a\x06\xD8a\x06UV[a\x06\xE0a\x01\xF2V[\x91\x82\x91\x82a\x06\xA8V[\x03\x90\xF3[a\x01\xF8V[\x90V[a\x07\x05\x90`\x08a\x07\n\x93\x02a\x043V[a\x06\xF2V[\x90V[\x90a\x07\x18\x91Ta\x06\xF5V[\x90V[a\x07'`\x03_\x90a\x07\rV[\x90V[a\x073\x90a\x02\x0FV[\x90RV[\x91\x90a\x07J\x90_` \x85\x01\x94\x01\x90a\x07*V[V[4a\x07|Wa\x07\\6`\x04a\x02\0V[a\x07xa\x07ga\x07\x1BV[a\x07oa\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[4a\x07\xAFWa\x07\x916`\x04a\x02\0V[a\x07\x99a\x13\xC0V[a\x07\xA1a\x01\xF2V[\x80a\x07\xAB\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[\x91``\x83\x83\x03\x12a\x08\x01Wa\x07\xCB\x82_\x85\x01a\x05\x0BV[\x92a\x07\xD9\x83` \x83\x01a\x05\x0BV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\xFCWa\x07\xF8\x92\x01a\x03\x1DV[\x90\x91V[a\x03\rV[a\x01\xFCV[4a\x08:Wa\x086a\x08%a\x08\x1C6`\x04a\x07\xB4V[\x92\x91\x90\x91a\x14xV[a\x08-a\x01\xF2V[\x91\x82\x91\x82a\x02\xC3V[\x03\x90\xF3[a\x01\xF8V[4a\x08oWa\x08O6`\x04a\x02\0V[a\x08ka\x08Za\x15EV[a\x08ba\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[4a\x08\xA3Wa\x08\x8Da\x08\x876`\x04a\x03\\V[\x90a\x16RV[a\x08\x95a\x01\xF2V[\x80a\x08\x9F\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[4a\x08\xD8Wa\x08\xB86`\x04a\x02\0V[a\x08\xD4a\x08\xC3a\x16zV[a\x08\xCBa\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[a\x08\xEA`\x04`\x01\x90a\x04UV[\x90V[4a\t\x1DWa\x08\xFD6`\x04a\x02\0V[a\t\x19a\t\x08a\x08\xDDV[a\t\x10a\x01\xF2V[\x91\x82\x91\x82a\x02\xC3V[\x03\x90\xF3[a\x01\xF8V[4a\tRWa\t26`\x04a\x02\0V[a\tNa\t=a\x17\x0CV[a\tEa\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[a\t`\x90a\x04\xEBV[\x90RV[\x91\x90a\tw\x90_` \x85\x01\x94\x01\x90a\tWV[V[4a\t\xA9Wa\t\x896`\x04a\x02\0V[a\t\xA5a\t\x94a\x17[V[a\t\x9Ca\x01\xF2V[\x91\x82\x91\x82a\tdV[\x03\x90\xF3[a\x01\xF8V[4a\t\xDEWa\t\xBE6`\x04a\x02\0V[a\t\xDAa\t\xC9a\x17\x8FV[a\t\xD1a\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[a\t\xEF`\x02_\x90a\x07\rV[\x90V[4a\n\"Wa\n\x026`\x04a\x02\0V[a\n\x1Ea\n\ra\t\xE3V[a\n\x15a\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[4a\nWWa\n76`\x04a\x02\0V[a\nSa\nBa\x17\xDBV[a\nJa\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\n\x96W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\x91W` \x01\x92` \x83\x02\x84\x01\x11a\n\x8CWV[a\x03\x19V[a\x03\x15V[a\x03\x11V[\x90` \x82\x82\x03\x12a\n\xCCW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xC7Wa\n\xC3\x92\x01a\n\\V[\x90\x91V[a\x03\rV[a\x01\xFCV[4a\x0B\0Wa\n\xEAa\n\xE46`\x04a\n\x9BV[\x90a\x19\xA0V[a\n\xF2a\x01\xF2V[\x80a\n\xFC\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[\x90` \x82\x82\x03\x12a\x0B\x1EWa\x0B\x1B\x91_\x01a\x05\x0BV[\x90V[a\x01\xFCV[4a\x0BQWa\x0B;a\x0B66`\x04a\x0B\x05V[a\x1APV[a\x0BCa\x01\xF2V[\x80a\x0BM\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B\xAAWa\x0B\x8A6`\x04a\x02\0V[a\x0B\xA6a\x0B\x95a\x0BVV[a\x0B\x9Da\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[4a\x0B\xDDWa\x0B\xBF6`\x04a\x02\0V[a\x0B\xC7a\x1AwV[a\x0B\xCFa\x01\xF2V[\x80a\x0B\xD9\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[a\x0B\xF6a\x0B\xF1a\x0B\xFB\x92a\x02\x0FV[a\x06dV[a\x02\x0FV[\x90V[\x90a\x0C\x08\x90a\x0B\xE2V[_R` R`@_ \x90V[_\x1C\x90V[a\x0C%a\x0C*\x91a\x0C\x14V[a\x06\xF2V[\x90V[a\x0C7\x90Ta\x0C\x19V[\x90V[a\x0CE\x90`\x05a\x0B\xFEV[\x90a\x0CQ_\x83\x01a\x0C-V[\x91a\x0C^`\x01\x82\x01a\x0C-V[\x91a\x0Cw`\x03a\x0Cp`\x02\x85\x01a\x0C-V[\x93\x01a\x0C-V[\x90V[a\x0C\xAFa\x0C\xB6\x94a\x0C\xA5``\x94\x98\x97\x95a\x0C\x9B`\x80\x86\x01\x9A_\x87\x01\x90a\x07*V[` \x85\x01\x90a\x07*V[`@\x83\x01\x90a\x07*V[\x01\x90a\x07*V[V[4a\x0C\xECWa\x0C\xE8a\x0C\xD3a\x0C\xCE6`\x04a\x05\x9EV[a\x0C:V[\x90a\x0C\xDF\x94\x92\x94a\x01\xF2V[\x94\x85\x94\x85a\x0CzV[\x03\x90\xF3[a\x01\xF8V[4a\r\x1FWa\r\ta\r\x046`\x04a\x0B\x05V[a\x1A\xE6V[a\r\x11a\x01\xF2V[\x80a\r\x1B\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[\x91\x90`@\x83\x82\x03\x12a\rLW\x80a\r@a\rI\x92_\x86\x01a\x05\x8FV[\x93` \x01a\x05\x8FV[\x90V[a\x01\xFCV[4a\r\x82Wa\r~a\rma\rg6`\x04a\r$V[\x90a\x1B|V[a\rua\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[a\r\x93`\x06_\x90a\x07\rV[\x90V[4a\r\xC6Wa\r\xA66`\x04a\x02\0V[a\r\xC2a\r\xB1a\r\x87V[a\r\xB9a\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\r\xED\x90a\x03\xAAV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\x07W`@RV[a\r\xCFV[\x90a\x0E\x1Fa\x0E\x18a\x01\xF2V[\x92\x83a\r\xE3V[V[a\x0E+`\x80a\x0E\x0CV[\x90V[_\x90V[a\x0E:a\x0E!V[\x90` \x80\x80\x80\x85a\x0EIa\x0E.V[\x81R\x01a\x0ETa\x0E.V[\x81R\x01a\x0E_a\x0E.V[\x81R\x01a\x0Eja\x0E.V[\x81RPPV[a\x0Exa\x0E2V[\x90V[a\x0E\x83a\x0EpV[Pa\x0E\x8Ca\x1C&V[\x90V[_\x90V[a\x0E\x9Fa\x0E\xA4\x91a\x0C\x14V[a\x047V[\x90V[a\x0E\xB1\x90Ta\x0E\x93V[\x90V[a\x0E\xBCa\x0E\x8FV[Pa\x0E\xC7`\x04a\x0E\xA7V[\x90V[``\x90V[\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x0E\xF5a\x0E\xF0a\x0E\xFA\x92a\x0E\xCFV[a\x0E\xDBV[a\x0E\xD2V[\x90V[\x90V[a\x0F\x0Ca\x0F\x11\x91a\x0E\xD2V[a\x0E\xFDV[\x90RV[\x90P\x90V[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x0F5\x81a\x0F<\x93a\x0F\x15V[\x80\x93a\x0F\x1AV[\x01\x90V[\x80a\x0FQ`\x01\x92a\x0FX\x96\x94a\x0F\0V[\x01\x91a\x0F%V[\x90V[a\x0F\x99\x90a\x0Fga\x0E\xCAV[Pa\x0F\x8Aa\x0Ft_a\x0E\xE1V[\x91\x93a\x0F~a\x01\xF2V[\x94\x85\x93` \x85\x01a\x0F@V[` \x82\x01\x81\x03\x82R\x03\x82a\r\xE3V[\x90V[\x90a\x0F\xB8a\x0F\xB232\x90\x85\x85\x91\x92\x90\x91\x92a\x14xV[\x15a\x02\xB1V[a\x0F\xC7Wa\x0F\xC5\x91a\x10CV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x0F\xDF`\x04\x82\x01a\x04\xA7V[\x03\x90\xFD[`\x08\x1C\x90V[a\x0F\xF5a\x0F\xFA\x91a\x0F\xE3V[a\x047V[\x90V[a\x10\x07\x90Ta\x0F\xE9V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x10-a\x103\x91\x93\x92\x93a\x02\x0FV[\x92a\x02\x0FV[\x82\x03\x91\x82\x11a\x10>WV[a\x10\nV[\x90a\x10Wa\x10Q`\x04a\x0F\xFDV[\x15a\x02\xB1V[a\x10{Wa\x10y\x91a\x10ma\x10t\x92Z\x92a\x10\xCFV[Z\x90a\x10\x1EV[a\x1E\0V[V[a\x10\x84\x91a\x10\xCFV[V[a\x10\x8F\x90a\x06\x83V[\x90V[\x91\x90a\x10\xAC\x81a\x10\xA5\x81a\x10\xB1\x95a\x03\x96V[\x80\x95a\x0F\x1AV[a\x03\xAAV[\x01\x90V[\x90\x91a\x10\xCC\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x10\x92V[\x90V[3\x90\x91a\x10\xFC\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\x86V[\x92a\x11\x11a\x11\x08a\x01\xF2V[\x92\x83\x92\x83a\x10\xB5V[\x03\x90\xA2V[\x90a\x11 \x91a\x0F\x9CV[V[\x90a\x114\x91a\x11/a\x1F\x07V[a\x12:V[V[`\xA0\x1C\x90V[a\x11Ha\x11M\x91a\x116V[a\x047V[\x90V[a\x11Z\x90Ta\x11<V[\x90V[a\x11qa\x11la\x11v\x92a\x0E\xCFV[a\x06dV[a\x04\xE0V[\x90V[a\x11\x82\x90a\x11]V[\x90V[`\xA0\x1B\x90V[\x90a\x11\x9A`\xFF`\xA0\x1B\x91a\x11\x85V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x11\xAD\x90a\x02\xB1V[\x90V[\x90V[\x90a\x11\xC8a\x11\xC3a\x11\xCF\x92a\x11\xA4V[a\x11\xB0V[\x82Ta\x11\x8BV[\x90UV[a\x11\xDC\x90a\x06gV[\x90V[a\x11\xE8\x90a\x11\xD3V[\x90V[_\x1B\x90V[\x90a\x12\x01`\x01\x80`\xA0\x1B\x03\x91a\x11\xEBV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x12\x14\x90a\x11\xD3V[\x90V[\x90V[\x90a\x12/a\x12*a\x126\x92a\x12\x0BV[a\x12\x17V[\x82Ta\x11\xF0V[\x90UV[a\x12D`\x01a\x11PV[a\x12\xACW\x81a\x12ca\x12]a\x12X_a\x11yV[a\x04\xEBV[\x91a\x04\xEBV[\x14a\x12\x90Wa\x12\x89a\x12\x82a\x12\x8E\x93a\x12}`\x01\x80a\x11\xB3V[a\x11\xDFV[`\x01a\x12\x1AV[a\x1A\xE6V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x12\xA8`\x04\x82\x01a\x04\xA7V[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x12\xC3`\x04\x82\x01a\x04\xA7V[\x03\x90\xFD[\x90a\x12\xD1\x91a\x11\"V[V[\x90a\x12\xDD\x90a\x02\x0FV[\x90RV[\x90a\x13Ha\x13?`\x03a\x12\xF2a\x0E!V[\x94a\x13\ta\x13\x01_\x83\x01a\x0C-V[_\x88\x01a\x12\xD3V[a\x13!a\x13\x18`\x01\x83\x01a\x0C-V[` \x88\x01a\x12\xD3V[a\x139a\x130`\x02\x83\x01a\x0C-V[`@\x88\x01a\x12\xD3V[\x01a\x0C-V[``\x84\x01a\x12\xD3V[V[a\x13S\x90a\x12\xE1V[\x90V[a\x13ma\x13r\x91a\x13ea\x0EpV[P`\x05a\x0B\xFEV[a\x13JV[\x90V[a\x13}a\x1F\x07V[a\x13\x85a\x13\x87V[V[a\x13\x8Fa\x1F\x92V[V[a\x13\x99a\x13uV[V[a\x13\xA3a\x1F\x07V[a\x13\xABa\x13\xADV[V[a\x13\xBEa\x13\xB9_a\x11yV[a\x1F\xC2V[V[a\x13\xC8a\x13\x9BV[V[a\x13\xD6a\x13\xDB\x91a\x0C\x14V[a\x06$V[\x90V[a\x13\xE8\x90Ta\x13\xCAV[\x90V[`\xE0\x1B\x90V[a\x13\xFA\x81a\x02\xB1V[\x03a\x14\x01WV[_\x80\xFD[\x90PQ\x90a\x14\x12\x82a\x13\xF1V[V[\x90` \x82\x82\x03\x12a\x14-Wa\x14*\x91_\x01a\x14\x05V[\x90V[a\x01\xFCV[a\x14Xa\x14e\x95\x93\x94\x92\x94a\x14N``\x84\x01\x96_\x85\x01\x90a\tWV[` \x83\x01\x90a\tWV[`@\x81\x85\x03\x91\x01Ra\x10\x92V[\x90V[a\x14pa\x01\xF2V[=_\x82>=\x90\xFD[\x92a\x14\xBB` \x93\x94a\x14\x88a\x0E\x8FV[Pa\x14\xC6a\x14\x9Ea\x14\x99`\x01a\x13\xDEV[a\x06\x8FV[\x93cz9y\xDC\x92\x95\x97a\x14\xAFa\x01\xF2V[\x98\x89\x97\x88\x96\x87\x96a\x13\xEBV[\x86R`\x04\x86\x01a\x142V[\x03\x91Z\xFA\x90\x81\x15a\x15\nW_\x91a\x14\xDCW[P\x90V[a\x14\xFD\x91P` =\x81\x11a\x15\x03W[a\x14\xF5\x81\x83a\r\xE3V[\x81\x01\x90a\x14\x14V[_a\x14\xD8V[P=a\x14\xEBV[a\x14hV[_\x90V[a\x15\x1D\x90Qa\x02\x0FV[\x90V[a\x15/a\x155\x91\x93\x92\x93a\x02\x0FV[\x92a\x02\x0FV[\x82\x01\x80\x92\x11a\x15@WV[a\x10\nV[a\x15Ma\x15\x0FV[Pa\x15ta\x15[`\x06a\x0C-V[a\x15n``a\x15ha\x1C&V[\x01a\x15\x13V[\x90a\x15 V[\x90V[\x90a\x15\x93a\x15\x8D32\x90\x85\x85\x91\x92\x90\x91\x92a\x14xV[\x15a\x02\xB1V[a\x15\xA2Wa\x15\xA0\x91a\x15\xBEV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x15\xBA`\x04\x82\x01a\x04\xA7V[\x03\x90\xFD[\x90a\x15\xD2a\x15\xCC`\x04a\x0F\xFDV[\x15a\x02\xB1V[a\x15\xF6Wa\x15\xF4\x91a\x15\xE8a\x15\xEF\x92Z\x92a\x16\x01V[Z\x90a\x10\x1EV[a\x1E\0V[V[a\x15\xFF\x91a\x16\x01V[V[\x90a\x16\r\x903\x92a\x0F[V[\x90a\x16Ma\x16;\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\x86V[\x92a\x16Da\x01\xF2V[\x91\x82\x91\x82a\x03\xE5V[\x03\x90\xA2V[\x90a\x16\\\x91a\x15wV[V[a\x16ra\x16ma\x16w\x92a\x0E\xCFV[a\x06dV[a\x02\x0FV[\x90V[a\x16\x82a\x15\x0FV[Pa\x16\x8Ba\x1C&V[a\x16\x96_\x82\x01a\x15\x13V[a\x16\xA8a\x16\xA2_a\x16^V[\x91a\x02\x0FV[\x14a\x16\xFFWa\x16\xBB_a\x16\xCB\x92\x01a\x15\x13V[a\x16\xC5`\x02a\x0C-V[\x90a\x15 V[Ba\x16\xDEa\x16\xD8\x83a\x02\x0FV[\x91a\x02\x0FV[\x10\x15a\x16\xF2Wa\x16\xEF\x90B\x90a\x10\x1EV[\x90V[Pa\x16\xFC_a\x16^V[\x90V[Pa\x17\t_a\x16^V[\x90V[a\x17\x14a\x15\x0FV[Pa\x17(``a\x17\"a\x1C&V[\x01a\x15\x13V[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x17Fa\x17K\x91a\x0C\x14V[a\x17/V[\x90V[a\x17X\x90Ta\x17:V[\x90V[a\x17ca\x17+V[Pa\x17m_a\x17NV[\x90V[\x90V[a\x17\x87a\x17\x82a\x17\x8C\x92a\x17pV[a\x06dV[a\x02\x0FV[\x90V[a\x17\x97a\x15\x0FV[Pa\x17\xABa\x17\xA5`\x04a\x0E\xA7V[\x15a\x02\xB1V[a\x17\xCFWa\x17\xCCa\x17\xBC`\x03a\x0C-V[a\x17\xC6`\x01a\x17sV[\x90a\x15 V[\x90V[a\x17\xD8_a\x16^V[\x90V[a\x17\xE3a\x15\x0FV[Pa\x17\xF7`@a\x17\xF1a\x1C&V[\x01a\x15\x13V[\x90V[\x90a\x18\x0Ea\x18\x08`\x04a\x0F\xFDV[\x15a\x02\xB1V[a\x182Wa\x180\x91a\x18$a\x18+\x92Z\x92a\x18\xD7V[Z\x90a\x10\x1EV[a\x1E\0V[V[a\x18;\x91a\x18\xD7V[V[P\x90V[`\x01a\x18M\x91\x01a\x02\x0FV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x18\xB2W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x18\xADW` \x01\x91`\x01\x82\x026\x03\x83\x13a\x18\xA8WV[a\x18lV[a\x18hV[a\x18dV[\x90\x82\x10\x15a\x18\xD2W` a\x18\xCE\x92\x02\x81\x01\x90a\x18pV[\x90\x91V[a\x18PV[a\x18\xE2\x81\x83\x90a\x18=V[\x91a\x18\xEBa\x15\x0FV[Pa\x18\xF5_a\x16^V[[\x80a\x19\ta\x19\x03\x86a\x02\x0FV[\x91a\x02\x0FV[\x10\x15a\x19\x9AWa\x197\x90a\x19-32\x90a\x19%\x87\x87\x86\x91a\x18\xB7V[\x92\x90\x91a\x14xV[a\x19<W[a\x18AV[a\x18\xF6V[3a\x19Ra\x19L\x86\x86\x85\x91a\x18\xB7V[\x90a\x0F[V[\x90a\x19\x92a\x19\x80\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\x86V[\x92a\x19\x89a\x01\xF2V[\x91\x82\x91\x82a\x03\xE5V[\x03\x90\xA2a\x192V[PPPPV[\x90a\x19\xAA\x91a\x17\xFAV[V[a\x19\xBD\x90a\x19\xB8a\x1F\x07V[a\x19\xBFV[V[\x80a\x19\xDAa\x19\xD4a\x19\xCF_a\x11yV[a\x04\xEBV[\x91a\x04\xEBV[\x14a\x1A4Wa\x19\xF2a\x19\xEB\x82a\x11\xDFV[`\x01a\x12\x1AV[a\x1A\x1C\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10\x86V[\x90a\x1A%a\x01\xF2V[\x80a\x1A/\x81a\x04\xA7V[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x1AL`\x04\x82\x01a\x04\xA7V[\x03\x90\xFD[a\x1AY\x90a\x19\xACV[V[a\x1Aca\x1F\x07V[a\x1Aka\x1AmV[V[a\x1Aua !V[V[a\x1A\x7Fa\x1A[V[V[a\x1A\x92\x90a\x1A\x8Da\x1F\x07V[a\x1A\x94V[V[\x80a\x1A\xAFa\x1A\xA9a\x1A\xA4_a\x11yV[a\x04\xEBV[\x91a\x04\xEBV[\x14a\x1A\xBFWa\x1A\xBD\x90a\x1F\xC2V[V[a\x1A\xE2a\x1A\xCB_a\x11yV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\tdV[\x03\x90\xFD[a\x1A\xEF\x90a\x1A\x81V[V[` \x91\x81R\x01\x90V[_\x7FGasCounter: invalid range\0\0\0\0\0\0\0\x91\x01RV[a\x1B.`\x19` \x92a\x1A\xF1V[a\x1B7\x81a\x1A\xFAV[\x01\x90V[a\x1BP\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1B!V[\x90V[\x15a\x1BZWV[a\x1Bba\x01\xF2V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1Bx`\x04\x82\x01a\x1B;V[\x03\x90\xFD[a\x1B\xAB\x91a\x1B\x88a\x15\x0FV[Pa\x1B\xA6\x81a\x1B\x9Fa\x1B\x99\x85a\x02\x0FV[\x91a\x02\x0FV[\x10\x15a\x1BSV[a\x10\x1EV[\x90V[a\x1B\xB8`\x80a\x0E\x0CV[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\x1B\xDBa\x1B\xE1\x91a\x02\x0FV[\x91a\x02\x0FV[\x90\x81\x15a\x1B\xECW\x04\x90V[a\x1B\xBBV[a\x1C\0a\x1C\x06\x91\x93\x92\x93a\x02\x0FV[\x92a\x02\x0FV[\x91a\x1C\x12\x83\x82\x02a\x02\x0FV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1C!WV[a\x10\nV[a\x1C.a\x0EpV[Pa\x1CBa\x1C<`\x04a\x0E\xA7V[\x15a\x02\xB1V[a\x1DDWa\x1Cca\x1C^`\x05a\x1CX`\x03a\x0C-V[\x90a\x0B\xFEV[a\x13JV[Ba\x1C\x93a\x1C\x8Da\x1C\x88a\x1Cx_\x86\x01a\x15\x13V[a\x1C\x82`\x02a\x0C-V[\x90a\x15 V[a\x02\x0FV[\x91a\x02\x0FV[\x10\x15a\x1C\x9CW\x90V[a\x1C\xED\x90a\x1C\xE7a\x1C\xD6_a\x1C\xCFa\x1C\xBFBa\x1C\xB9\x84\x88\x01a\x15\x13V[\x90a\x10\x1EV[a\x1C\xC9`\x02a\x0C-V[\x90a\x1B\xCFV[\x93\x01a\x15\x13V[\x91a\x1C\xE1`\x02a\x0C-V[\x90a\x1B\xF1V[\x90a\x15 V[a\x1DAa\x1D8_a\x1D3a\x1D*_a\x1D%a\x1D\x1C_\x95a\x1D\x17a\x1D\x0Ea\x1B\xAEV[\x9A_\x8C\x01a\x12\xD3V[a\x16^V[` \x89\x01a\x12\xD3V[a\x16^V[`@\x86\x01a\x12\xD3V[a\x16^V[``\x83\x01a\x12\xD3V[\x90V[_a\x1D\xA1a\x1D\x98_a\x1D\x93a\x1D\x8A_a\x1D\x85a\x1D|_\x95a\x1Dwa\x1Doa\x1Dia\x1B\xAEV[\x9Ba\x16^V[_\x8C\x01a\x12\xD3V[a\x16^V[` \x89\x01a\x12\xD3V[a\x16^V[`@\x86\x01a\x12\xD3V[a\x16^V[``\x83\x01a\x12\xD3V[\x90V[\x90a\x1D\xB0_\x19\x91a\x11\xEBV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x1D\xD2a\x1D\xCDa\x1D\xD9\x92a\x0B\xE2V[a\x1D\xBAV[\x82Ta\x1D\xA4V[\x90UV[\x91` a\x1D\xFE\x92\x94\x93a\x1D\xF7`@\x82\x01\x96_\x83\x01\x90a\x07*V[\x01\x90a\x07*V[V[a\x1E\x13a\x1E\r`\x04a\x0F\xFDV[\x15a\x02\xB1V[a\x1F\x04Wa\x1E*a\x1E$`\x04a\x0E\xA7V[\x15a\x02\xB1V[a\x1E\xF7W[a\x1E7a!\xF7V[a\x1E\xA8a\x1EE\x82:\x90a\x1B\xF1V[a\x1Ey\x83a\x1Es`\x02a\x1Ec`\x05a\x1E]`\x03a\x0C-V[\x90a\x0B\xFEV[\x01\x91a\x1En\x83a\x0C-V[a\x15 V[\x90a\x1D\xBDV[a\x1E\xA2`\x03a\x1E\x92`\x05a\x1E\x8C\x83a\x0C-V[\x90a\x0B\xFEV[\x01\x91a\x1E\x9D\x83a\x0C-V[a\x15 V[\x90a\x1D\xBDV[a\x1E\xB2`\x03a\x0C-V[:a\x1E\xDD\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0B\xE2V[\x92a\x1E\xF2a\x1E\xE9a\x01\xF2V[\x92\x83\x92\x83a\x1D\xDDV[\x03\x90\xA2V[a\x1E\xFFa \xECV[a\x1E/V[PV[a\x1F\x0Fa\x17[V[a\x1F(a\x1F\"a\x1F\x1Da#\xFBV[a\x04\xEBV[\x91a\x04\xEBV[\x03a\x1F/WV[a\x1FQa\x1F:a#\xFBV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\tdV[\x03\x90\xFD[`\x08\x1B\x90V[\x90a\x1Fha\xFF\0\x91a\x1FUV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1F\x87a\x1F\x82a\x1F\x8E\x92a\x11\xA4V[a\x11\xB0V[\x82Ta\x1F[V[\x90UV[a\x1F\x9D_`\x04a\x1FrV[V[\x90V[\x90a\x1F\xB7a\x1F\xB2a\x1F\xBE\x92a\x10\x86V[a\x1F\x9FV[\x82Ta\x11\xF0V[\x90UV[a\x1F\xCB_a\x17NV[a\x1F\xD5\x82_a\x1F\xA2V[\x90a \ta \x03\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10\x86V[\x91a\x10\x86V[\x91a \x12a\x01\xF2V[\x80a \x1C\x81a\x04\xA7V[\x03\x90\xA3V[a -`\x01`\x04a\x1FrV[V[\x90a ;`\xFF\x91a\x11\xEBV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a Za Ua a\x92a\x11\xA4V[a\x11\xB0V[\x82Ta /V[\x90UV[\x90a o\x90a\x16^V[_R` R`@_ \x90V[\x90a \xD8```\x03a \xDE\x94a \x9E_\x82\x01a \x98_\x88\x01a\x15\x13V[\x90a\x1D\xBDV[a \xB7`\x01\x82\x01a \xB1` \x88\x01a\x15\x13V[\x90a\x1D\xBDV[a \xD0`\x02\x82\x01a \xCA`@\x88\x01a\x15\x13V[\x90a\x1D\xBDV[\x01\x92\x01a\x15\x13V[\x90a\x1D\xBDV[V[\x90a \xEA\x91a {V[V[a \xFFa \xF9`\x04a\x0E\xA7V[\x15a\x02\xB1V[a!\x06W[V[a!\x12`\x01`\x04a EV[a!%a!\x1E_a\x16^V[`\x03a\x1D\xBDV[a!\x8EBa!}a!t_a!oa!f_a!aa!X_\x95a!Sa!Ja\x1B\xAEV[\x9A_\x8C\x01a\x12\xD3V[a\x16^V[` \x89\x01a\x12\xD3V[a\x16^V[`@\x86\x01a\x12\xD3V[a\x16^V[``\x83\x01a\x12\xD3V[a!\x89`\x05_\x90a eV[a \xE0V[_B\x90a!\xD0a!\xBE\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x16^V[\x92a!\xC7a\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xA2a!\x04V[\x90V[a!\xE4\x90a\x02\x0FV[_\x19\x81\x14a!\xF2W`\x01\x01\x90V[a\x10\nV[a\"\x14a\"\x0F`\x05a\"\t`\x03a\x0C-V[\x90a\x0B\xFEV[a!\xD8V[Ba\"Da\">a\"9a\")_\x86\x01a\x0C-V[a\"3`\x02a\x0C-V[\x90a\x15 V[a\x02\x0FV[\x91a\x02\x0FV[\x10\x15a\"NW[PV[a\"xa\"oa\"__\x84\x01a\x0C-V[a\"i`\x02a\x0C-V[\x90a\x15 V[`\x01\x83\x01a\x1D\xBDV[a\"\xA0a\"\x99a\"\x8A`\x03\x84\x01a\x0C-V[a\"\x94`\x06a\x0C-V[a\x15 V[`\x06a\x1D\xBDV[a\"\xAA`\x03a\x0C-V[a\"\xD7a\"\xB9`\x02\x84\x01a\x0C-V[\x92a\"\xD1_a\"\xCA`\x01\x84\x01a\x0C-V[\x92\x01a\x0C-V[\x90a\x10\x1EV[a#\x01\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0B\xE2V[\x92a#\x16a#\ra\x01\xF2V[\x92\x83\x92\x83a\x1D\xDDV[\x03\x90\xA2a#5a#.a#)`\x03a\x0C-V[a!\xDBV[`\x03a\x1D\xBDV[a#\xA7Ba#\x8Da#\x84_a#\x7Fa#v_a#qa#h_\x95a#ca#Za\x1B\xAEV[\x9A_\x8C\x01a\x12\xD3V[a\x16^V[` \x89\x01a\x12\xD3V[a\x16^V[`@\x86\x01a\x12\xD3V[a\x16^V[``\x83\x01a\x12\xD3V[a#\xA2`\x05a#\x9C`\x03a\x0C-V[\x90a\x0B\xFEV[a \xE0V[a#\xB1`\x03a\x0C-V[B\x90a#\xF2a#\xE0\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0B\xE2V[\x92a#\xE9a\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xA2_a\"KV[a$\x03a\x17+V[P3\x90V`\x80`@R4`\x1CW`\x0E` V[a\x01\x94a\0+\x829a\x01\x94\x90\xF3[`&V[`@Q\x90V[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x01{V[a\0\x1D_5a\0,V[cz9y\xDC\x03a\0\x0EWa\x01BV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\0X\x90a\0DV[\x90V[a\0d\x81a\0OV[\x03a\0kWV[_\x80\xFD[\x90P5\x90a\0|\x82a\0[V[V[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\0\xC4W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xBFW` \x01\x92`\x01\x83\x02\x84\x01\x11a\0\xBAWV[a\0\x86V[a\0\x82V[a\0~V[\x91``\x83\x83\x03\x12a\x01\x16Wa\0\xE0\x82_\x85\x01a\0oV[\x92a\0\xEE\x83` \x83\x01a\0oV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x11Wa\x01\r\x92\x01a\0\x8AV[\x90\x91V[a\0@V[a\0<V[\x15\x15\x90V[a\x01)\x90a\x01\x1BV[\x90RV[\x91\x90a\x01@\x90_` \x85\x01\x94\x01\x90a\x01 V[V[4a\x01vWa\x01ra\x01aa\x01X6`\x04a\0\xC9V[\x92\x91\x90\x91a\x01\x83V[a\x01ia\x002V[\x91\x82\x91\x82a\x01-V[\x03\x90\xF3[a\08V[_\x80\xFD[_\x90V[PPPPa\x01\x8Fa\x01\x7FV[P_\x90V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b610dcb565b61001d5f356101ec565b8063086146d2146101e757806318d5aafe146101e2578063366cbab7146101dd5780633b6ab2a9146101d857806346e2cc09146101d3578063485cc955146101ce5780634b2c0706146101c95780635467cb48146101c45780635b3cd6e2146101bf57806361543801146101ba578063715018a6146101b55780637a3979dc146101b05780637fbd295e146101ab578063804e5123146101a657806382f44ade146101a157806384fab62b1461019c5780638d5a239b146101975780638da5cb5b14610192578063aff74c6d1461018d578063b470aade14610188578063c660d3f314610183578063cdafb9781461017e578063d4f0eb4d14610179578063d878134214610174578063de1f453e1461016f578063ea4a11041461016a578063f2fde38b14610165578063f7b8935e146101605763ff7b30840361000e57610d96565b610d51565b610cf1565b610cb8565b610baf565b610b7a565b610b23565b610ad1565b610a27565b6109f2565b6109ae565b610979565b610922565b6108ed565b6108a8565b610874565b61083f565b610806565b610781565b61074c565b6106bd565b6105f1565b6105bc565b610547565b6104ac565b610472565b6103fd565b6102d8565b61027c565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f91031261020a57565b6101fc565b90565b61021b9061020f565b9052565b90606080610265936102375f8201515f860190610212565b61024960208201516020860190610212565b61025b60408201516040860190610212565b0151910190610212565b565b919061027a905f6080850194019061021f565b565b346102ac5761028c366004610200565b6102a8610297610e7b565b61029f6101f2565b91829182610267565b0390f35b6101f8565b151590565b6102bf906102b1565b9052565b91906102d6905f602085019401906102b6565b565b34610308576102e8366004610200565b6103046102f3610eb4565b6102fb6101f2565b918291826102c3565b0390f35b6101f8565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156103575781359167ffffffffffffffff831161035257602001926001830284011161034d57565b610319565b610315565b610311565b9060208282031261038d575f82013567ffffffffffffffff811161038857610384920161031d565b9091565b61030d565b6101fc565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6103d36103dc6020936103e1936103ca81610392565b93848093610396565b9586910161039f565b6103aa565b0190565b6103fa9160208201915f8184039101526103b4565b90565b3461042e5761042a61041961041336600461035c565b90610f5b565b6104216101f2565b918291826103e5565b0390f35b6101f8565b1c90565b60ff1690565b61044d9060086104529302610433565b610437565b90565b90610460915461043d565b90565b61046f60045f90610455565b90565b346104a257610482366004610200565b61049e61048d610463565b6104956101f2565b918291826102c3565b0390f35b6101f8565b5f0190565b346104db576104c56104bf36600461035c565b90611116565b6104cd6101f2565b806104d7816104a7565b0390f35b6101f8565b60018060a01b031690565b6104f4906104e0565b90565b610500816104eb565b0361050757565b5f80fd5b90503590610518826104f7565b565b9190604083820312610542578061053661053f925f860161050b565b9360200161050b565b90565b6101fc565b346105765761056061055a36600461051a565b906112c7565b6105686101f2565b80610572816104a7565b0390f35b6101f8565b6105848161020f565b0361058b57565b5f80fd5b9050359061059c8261057b565b565b906020828203126105b7576105b4915f0161058f565b90565b6101fc565b346105ec576105e86105d76105d236600461059e565b611356565b6105df6101f2565b91829182610267565b0390f35b6101f8565b3461061f57610601366004610200565b610609611391565b6106116101f2565b8061061b816104a7565b0390f35b6101f8565b60018060a01b031690565b61063f9060086106449302610433565b610624565b90565b90610652915461062f565b90565b61066160015f90610647565b90565b90565b61067b610676610680926104e0565b610664565b6104e0565b90565b61068c90610667565b90565b61069890610683565b90565b6106a49061068f565b9052565b91906106bb905f6020850194019061069b565b565b346106ed576106cd366004610200565b6106e96106d8610655565b6106e06101f2565b918291826106a8565b0390f35b6101f8565b90565b61070590600861070a9302610433565b6106f2565b90565b9061071891546106f5565b90565b61072760035f9061070d565b90565b6107339061020f565b9052565b919061074a905f6020850194019061072a565b565b3461077c5761075c366004610200565b61077861076761071b565b61076f6101f2565b91829182610737565b0390f35b6101f8565b346107af57610791366004610200565b6107996113c0565b6107a16101f2565b806107ab816104a7565b0390f35b6101f8565b91606083830312610801576107cb825f850161050b565b926107d9836020830161050b565b92604082013567ffffffffffffffff81116107fc576107f8920161031d565b9091565b61030d565b6101fc565b3461083a5761083661082561081c3660046107b4565b92919091611478565b61082d6101f2565b918291826102c3565b0390f35b6101f8565b3461086f5761084f366004610200565b61086b61085a611545565b6108626101f2565b91829182610737565b0390f35b6101f8565b346108a35761088d61088736600461035c565b90611652565b6108956101f2565b8061089f816104a7565b0390f35b6101f8565b346108d8576108b8366004610200565b6108d46108c361167a565b6108cb6101f2565b91829182610737565b0390f35b6101f8565b6108ea6004600190610455565b90565b3461091d576108fd366004610200565b6109196109086108dd565b6109106101f2565b918291826102c3565b0390f35b6101f8565b3461095257610932366004610200565b61094e61093d61170c565b6109456101f2565b91829182610737565b0390f35b6101f8565b610960906104eb565b9052565b9190610977905f60208501940190610957565b565b346109a957610989366004610200565b6109a561099461175b565b61099c6101f2565b91829182610964565b0390f35b6101f8565b346109de576109be366004610200565b6109da6109c961178f565b6109d16101f2565b91829182610737565b0390f35b6101f8565b6109ef60025f9061070d565b90565b34610a2257610a02366004610200565b610a1e610a0d6109e3565b610a156101f2565b91829182610737565b0390f35b6101f8565b34610a5757610a37366004610200565b610a53610a426117db565b610a4a6101f2565b91829182610737565b0390f35b6101f8565b909182601f83011215610a965781359167ffffffffffffffff8311610a91576020019260208302840111610a8c57565b610319565b610315565b610311565b90602082820312610acc575f82013567ffffffffffffffff8111610ac757610ac39201610a5c565b9091565b61030d565b6101fc565b34610b0057610aea610ae4366004610a9b565b906119a0565b610af26101f2565b80610afc816104a7565b0390f35b6101f8565b90602082820312610b1e57610b1b915f0161050b565b90565b6101fc565b34610b5157610b3b610b36366004610b05565b611a50565b610b436101f2565b80610b4d816104a7565b0390f35b6101f8565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610baa57610b8a366004610200565b610ba6610b95610b56565b610b9d6101f2565b91829182610737565b0390f35b6101f8565b34610bdd57610bbf366004610200565b610bc7611a77565b610bcf6101f2565b80610bd9816104a7565b0390f35b6101f8565b610bf6610bf1610bfb9261020f565b610664565b61020f565b90565b90610c0890610be2565b5f5260205260405f2090565b5f1c90565b610c25610c2a91610c14565b6106f2565b90565b610c379054610c19565b90565b610c45906005610bfe565b90610c515f8301610c2d565b91610c5e60018201610c2d565b91610c776003610c7060028501610c2d565b9301610c2d565b90565b610caf610cb694610ca5606094989795610c9b608086019a5f87019061072a565b602085019061072a565b604083019061072a565b019061072a565b565b34610cec57610ce8610cd3610cce36600461059e565b610c3a565b90610cdf9492946101f2565b94859485610c7a565b0390f35b6101f8565b34610d1f57610d09610d04366004610b05565b611ae6565b610d116101f2565b80610d1b816104a7565b0390f35b6101f8565b9190604083820312610d4c5780610d40610d49925f860161058f565b9360200161058f565b90565b6101fc565b34610d8257610d7e610d6d610d67366004610d24565b90611b7c565b610d756101f2565b91829182610737565b0390f35b6101f8565b610d9360065f9061070d565b90565b34610dc657610da6366004610200565b610dc2610db1610d87565b610db96101f2565b91829182610737565b0390f35b6101f8565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b90610ded906103aa565b810190811067ffffffffffffffff821117610e0757604052565b610dcf565b90610e1f610e186101f2565b9283610de3565b565b610e2b6080610e0c565b90565b5f90565b610e3a610e21565b90602080808085610e49610e2e565b815201610e54610e2e565b815201610e5f610e2e565b815201610e6a610e2e565b81525050565b610e78610e32565b90565b610e83610e70565b50610e8c611c26565b90565b5f90565b610e9f610ea491610c14565b610437565b90565b610eb19054610e93565b90565b610ebc610e8f565b50610ec76004610ea7565b90565b606090565b90565b60ff60f81b1690565b60f81b90565b610ef5610ef0610efa92610ecf565b610edb565b610ed2565b90565b90565b610f0c610f1191610ed2565b610efd565b9052565b905090565b90825f939282370152565b909182610f3581610f3c93610f15565b8093610f1a565b0190565b80610f51600192610f589694610f00565b0191610f25565b90565b610f9990610f67610eca565b50610f8a610f745f610ee1565b9193610f7e6101f2565b94859360208501610f40565b60208201810382520382610de3565b90565b90610fb8610fb233329085859192909192611478565b156102b1565b610fc757610fc591611043565b565b5f631b8e828b60e31b815280610fdf600482016104a7565b0390fd5b60081c90565b610ff5610ffa91610fe3565b610437565b90565b6110079054610fe9565b90565b634e487b7160e01b5f52601160045260245ffd5b61102d6110339193929361020f565b9261020f565b820391821161103e57565b61100a565b906110576110516004610ffd565b156102b1565b61107b576110799161106d611074925a926110cf565b5a9061101e565b611e00565b565b611084916110cf565b565b61108f90610683565b90565b91906110ac816110a5816110b195610396565b8095610f1a565b6103aa565b0190565b90916110cc9260208301925f818503910152611092565b90565b3390916110fc7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611086565b926111116111086101f2565b928392836110b5565b0390a2565b9061112091610f9c565b565b906111349161112f611f07565b61123a565b565b60a01c90565b61114861114d91611136565b610437565b90565b61115a905461113c565b90565b61117161116c61117692610ecf565b610664565b6104e0565b90565b6111829061115d565b90565b60a01b90565b9061119a60ff60a01b91611185565b9181191691161790565b6111ad906102b1565b90565b90565b906111c86111c36111cf926111a4565b6111b0565b825461118b565b9055565b6111dc90610667565b90565b6111e8906111d3565b90565b5f1b90565b9061120160018060a01b03916111eb565b9181191691161790565b611214906111d3565b90565b90565b9061122f61122a6112369261120b565b611217565b82546111f0565b9055565b6112446001611150565b6112ac578161126361125d6112585f611179565b6104eb565b916104eb565b146112905761128961128261128e9361127d6001806111b3565b6111df565b600161121a565b611ae6565b565b5f632e7f3c7f60e11b8152806112a8600482016104a7565b0390fd5b5f62dc149f60e41b8152806112c3600482016104a7565b0390fd5b906112d191611122565b565b906112dd9061020f565b9052565b9061134861133f60036112f2610e21565b946113096113015f8301610c2d565b5f88016112d3565b61132161131860018301610c2d565b602088016112d3565b61133961133060028301610c2d565b604088016112d3565b01610c2d565b606084016112d3565b565b611353906112e1565b90565b61136d61137291611365610e70565b506005610bfe565b61134a565b90565b61137d611f07565b611385611387565b565b61138f611f92565b565b611399611375565b565b6113a3611f07565b6113ab6113ad565b565b6113be6113b95f611179565b611fc2565b565b6113c861139b565b565b6113d66113db91610c14565b610624565b90565b6113e890546113ca565b90565b60e01b90565b6113fa816102b1565b0361140157565b5f80fd5b90505190611412826113f1565b565b9060208282031261142d5761142a915f01611405565b90565b6101fc565b611458611465959394929461144e60608401965f850190610957565b6020830190610957565b6040818503910152611092565b90565b6114706101f2565b3d5f823e3d90fd5b926114bb60209394611488610e8f565b506114c661149e61149960016113de565b61068f565b93637a3979dc9295976114af6101f2565b988997889687966113eb565b865260048601611432565b03915afa90811561150a575f916114dc575b5090565b6114fd915060203d8111611503575b6114f58183610de3565b810190611414565b5f6114d8565b503d6114eb565b611468565b5f90565b61151d905161020f565b90565b61152f6115359193929361020f565b9261020f565b820180921161154057565b61100a565b61154d61150f565b5061157461155b6006610c2d565b61156e6060611568611c26565b01611513565b90611520565b90565b9061159361158d33329085859192909192611478565b156102b1565b6115a2576115a0916115be565b565b5f631b8e828b60e31b8152806115ba600482016104a7565b0390fd5b906115d26115cc6004610ffd565b156102b1565b6115f6576115f4916115e86115ef925a92611601565b5a9061101e565b611e00565b565b6115ff91611601565b565b9061160d903392610f5b565b9061164d61163b7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611086565b926116446101f2565b918291826103e5565b0390a2565b9061165c91611577565b565b61167261166d61167792610ecf565b610664565b61020f565b90565b61168261150f565b5061168b611c26565b6116965f8201611513565b6116a86116a25f61165e565b9161020f565b146116ff576116bb5f6116cb9201611513565b6116c56002610c2d565b90611520565b426116de6116d88361020f565b9161020f565b10156116f2576116ef90429061101e565b90565b506116fc5f61165e565b90565b506117095f61165e565b90565b61171461150f565b506117286060611722611c26565b01611513565b90565b5f90565b60018060a01b031690565b61174661174b91610c14565b61172f565b90565b611758905461173a565b90565b61176361172b565b5061176d5f61174e565b90565b90565b61178761178261178c92611770565b610664565b61020f565b90565b61179761150f565b506117ab6117a56004610ea7565b156102b1565b6117cf576117cc6117bc6003610c2d565b6117c66001611773565b90611520565b90565b6117d85f61165e565b90565b6117e361150f565b506117f760406117f1611c26565b01611513565b90565b9061180e6118086004610ffd565b156102b1565b611832576118309161182461182b925a926118d7565b5a9061101e565b611e00565b565b61183b916118d7565b565b5090565b600161184d910161020f565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b9035906001602003813603038212156118b2570180359067ffffffffffffffff82116118ad576020019160018202360383136118a857565b61186c565b611868565b611864565b908210156118d25760206118ce9202810190611870565b9091565b611850565b6118e281839061183d565b916118eb61150f565b506118f55f61165e565b5b806119096119038661020f565b9161020f565b101561199a576119379061192d333290611925878786916118b7565b929091611478565b61193c575b611841565b6118f6565b3361195261194c868685916118b7565b90610f5b565b906119926119807f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611086565b926119896101f2565b918291826103e5565b0390a2611932565b50505050565b906119aa916117fa565b565b6119bd906119b8611f07565b6119bf565b565b806119da6119d46119cf5f611179565b6104eb565b916104eb565b14611a34576119f26119eb826111df565b600161121a565b611a1c7f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b991611086565b90611a256101f2565b80611a2f816104a7565b0390a2565b5f632e7f3c7f60e11b815280611a4c600482016104a7565b0390fd5b611a59906119ac565b565b611a63611f07565b611a6b611a6d565b565b611a75612021565b565b611a7f611a5b565b565b611a9290611a8d611f07565b611a94565b565b80611aaf611aa9611aa45f611179565b6104eb565b916104eb565b14611abf57611abd90611fc2565b565b611ae2611acb5f611179565b5f918291631e4fbdf760e01b835260048301610964565b0390fd5b611aef90611a81565b565b60209181520190565b5f7f476173436f756e7465723a20696e76616c69642072616e676500000000000000910152565b611b2e6019602092611af1565b611b3781611afa565b0190565b611b509060208101905f818303910152611b21565b90565b15611b5a57565b611b626101f2565b62461bcd60e51b815280611b7860048201611b3b565b0390fd5b611bab91611b8861150f565b50611ba681611b9f611b998561020f565b9161020f565b1015611b53565b61101e565b90565b611bb86080610e0c565b90565b634e487b7160e01b5f52601260045260245ffd5b611bdb611be19161020f565b9161020f565b908115611bec570490565b611bbb565b611c00611c069193929361020f565b9261020f565b91611c1283820261020f565b928184041490151715611c2157565b61100a565b611c2e610e70565b50611c42611c3c6004610ea7565b156102b1565b611d4457611c63611c5e6005611c586003610c2d565b90610bfe565b61134a565b42611c93611c8d611c88611c785f8601611513565b611c826002610c2d565b90611520565b61020f565b9161020f565b1015611c9c5790565b611ced90611ce7611cd65f611ccf611cbf42611cb9848801611513565b9061101e565b611cc96002610c2d565b90611bcf565b9301611513565b91611ce16002610c2d565b90611bf1565b90611520565b611d41611d385f611d33611d2a5f611d25611d1c5f95611d17611d0e611bae565b9a5f8c016112d3565b61165e565b602089016112d3565b61165e565b604086016112d3565b61165e565b606083016112d3565b90565b5f611da1611d985f611d93611d8a5f611d85611d7c5f95611d77611d6f611d69611bae565b9b61165e565b5f8c016112d3565b61165e565b602089016112d3565b61165e565b604086016112d3565b61165e565b606083016112d3565b90565b90611db05f19916111eb565b9181191691161790565b90565b90611dd2611dcd611dd992610be2565b611dba565b8254611da4565b9055565b916020611dfe929493611df760408201965f83019061072a565b019061072a565b565b611e13611e0d6004610ffd565b156102b1565b611f0457611e2a611e246004610ea7565b156102b1565b611ef7575b611e376121f7565b611ea8611e45823a90611bf1565b611e7983611e736002611e636005611e5d6003610c2d565b90610bfe565b0191611e6e83610c2d565b611520565b90611dbd565b611ea26003611e926005611e8c83610c2d565b90610bfe565b0191611e9d83610c2d565b611520565b90611dbd565b611eb26003610c2d565b3a611edd7f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610be2565b92611ef2611ee96101f2565b92839283611ddd565b0390a2565b611eff6120ec565b611e2f565b50565b611f0f61175b565b611f28611f22611f1d6123fb565b6104eb565b916104eb565b03611f2f57565b611f51611f3a6123fb565b5f91829163118cdaa760e01b835260048301610964565b0390fd5b60081b90565b90611f6861ff0091611f55565b9181191691161790565b90611f87611f82611f8e926111a4565b6111b0565b8254611f5b565b9055565b611f9d5f6004611f72565b565b90565b90611fb7611fb2611fbe92611086565b611f9f565b82546111f0565b9055565b611fcb5f61174e565b611fd5825f611fa2565b906120096120037f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093611086565b91611086565b916120126101f2565b8061201c816104a7565b0390a3565b61202d60016004611f72565b565b9061203b60ff916111eb565b9181191691161790565b9061205a612055612061926111a4565b6111b0565b825461202f565b9055565b9061206f9061165e565b5f5260205260405f2090565b906120d8606060036120de9461209e5f82016120985f8801611513565b90611dbd565b6120b7600182016120b160208801611513565b90611dbd565b6120d0600282016120ca60408801611513565b90611dbd565b019201611513565b90611dbd565b565b906120ea9161207b565b565b6120ff6120f96004610ea7565b156102b1565b612106575b565b61211260016004612045565b61212561211e5f61165e565b6003611dbd565b61218e4261217d6121745f61216f6121665f6121616121585f9561215361214a611bae565b9a5f8c016112d3565b61165e565b602089016112d3565b61165e565b604086016112d3565b61165e565b606083016112d3565b61218960055f90612065565b6120e0565b5f42906121d06121be7f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e9261165e565b926121c76101f2565b91829182610737565b0390a2612104565b90565b6121e49061020f565b5f1981146121f25760010190565b61100a565b61221461220f60056122096003610c2d565b90610bfe565b6121d8565b4261224461223e6122396122295f8601610c2d565b6122336002610c2d565b90611520565b61020f565b9161020f565b101561224e575b50565b61227861226f61225f5f8401610c2d565b6122696002610c2d565b90611520565b60018301611dbd565b6122a061229961228a60038401610c2d565b6122946006610c2d565b611520565b6006611dbd565b6122aa6003610c2d565b6122d76122b960028401610c2d565b926122d15f6122ca60018401610c2d565b9201610c2d565b9061101e565b6123017f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610be2565b9261231661230d6101f2565b92839283611ddd565b0390a261233561232e6123296003610c2d565b6121db565b6003611dbd565b6123a74261238d6123845f61237f6123765f6123716123685f9561236361235a611bae565b9a5f8c016112d3565b61165e565b602089016112d3565b61165e565b604086016112d3565b61165e565b606083016112d3565b6123a2600561239c6003610c2d565b90610bfe565b6120e0565b6123b16003610c2d565b42906123f26123e07f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610be2565b926123e96101f2565b91829182610737565b0390a25f61224b565b61240361172b565b50339056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\r\xCBV[a\0\x1D_5a\x01\xECV[\x80c\x08aF\xD2\x14a\x01\xE7W\x80c\x18\xD5\xAA\xFE\x14a\x01\xE2W\x80c6l\xBA\xB7\x14a\x01\xDDW\x80c;j\xB2\xA9\x14a\x01\xD8W\x80cF\xE2\xCC\t\x14a\x01\xD3W\x80cH\\\xC9U\x14a\x01\xCEW\x80cK,\x07\x06\x14a\x01\xC9W\x80cTg\xCBH\x14a\x01\xC4W\x80c[<\xD6\xE2\x14a\x01\xBFW\x80caT8\x01\x14a\x01\xBAW\x80cqP\x18\xA6\x14a\x01\xB5W\x80cz9y\xDC\x14a\x01\xB0W\x80c\x7F\xBD)^\x14a\x01\xABW\x80c\x80NQ#\x14a\x01\xA6W\x80c\x82\xF4J\xDE\x14a\x01\xA1W\x80c\x84\xFA\xB6+\x14a\x01\x9CW\x80c\x8DZ#\x9B\x14a\x01\x97W\x80c\x8D\xA5\xCB[\x14a\x01\x92W\x80c\xAF\xF7Lm\x14a\x01\x8DW\x80c\xB4p\xAA\xDE\x14a\x01\x88W\x80c\xC6`\xD3\xF3\x14a\x01\x83W\x80c\xCD\xAF\xB9x\x14a\x01~W\x80c\xD4\xF0\xEBM\x14a\x01yW\x80c\xD8x\x13B\x14a\x01tW\x80c\xDE\x1FE>\x14a\x01oW\x80c\xEAJ\x11\x04\x14a\x01jW\x80c\xF2\xFD\xE3\x8B\x14a\x01eW\x80c\xF7\xB8\x93^\x14a\x01`Wc\xFF{0\x84\x03a\0\x0EWa\r\x96V[a\rQV[a\x0C\xF1V[a\x0C\xB8V[a\x0B\xAFV[a\x0BzV[a\x0B#V[a\n\xD1V[a\n'V[a\t\xF2V[a\t\xAEV[a\tyV[a\t\"V[a\x08\xEDV[a\x08\xA8V[a\x08tV[a\x08?V[a\x08\x06V[a\x07\x81V[a\x07LV[a\x06\xBDV[a\x05\xF1V[a\x05\xBCV[a\x05GV[a\x04\xACV[a\x04rV[a\x03\xFDV[a\x02\xD8V[a\x02|V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x02\nWV[a\x01\xFCV[\x90V[a\x02\x1B\x90a\x02\x0FV[\x90RV[\x90``\x80a\x02e\x93a\x027_\x82\x01Q_\x86\x01\x90a\x02\x12V[a\x02I` \x82\x01Q` \x86\x01\x90a\x02\x12V[a\x02[`@\x82\x01Q`@\x86\x01\x90a\x02\x12V[\x01Q\x91\x01\x90a\x02\x12V[V[\x91\x90a\x02z\x90_`\x80\x85\x01\x94\x01\x90a\x02\x1FV[V[4a\x02\xACWa\x02\x8C6`\x04a\x02\0V[a\x02\xA8a\x02\x97a\x0E{V[a\x02\x9Fa\x01\xF2V[\x91\x82\x91\x82a\x02gV[\x03\x90\xF3[a\x01\xF8V[\x15\x15\x90V[a\x02\xBF\x90a\x02\xB1V[\x90RV[\x91\x90a\x02\xD6\x90_` \x85\x01\x94\x01\x90a\x02\xB6V[V[4a\x03\x08Wa\x02\xE86`\x04a\x02\0V[a\x03\x04a\x02\xF3a\x0E\xB4V[a\x02\xFBa\x01\xF2V[\x91\x82\x91\x82a\x02\xC3V[\x03\x90\xF3[a\x01\xF8V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03WW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03RW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x03MWV[a\x03\x19V[a\x03\x15V[a\x03\x11V[\x90` \x82\x82\x03\x12a\x03\x8DW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\x88Wa\x03\x84\x92\x01a\x03\x1DV[\x90\x91V[a\x03\rV[a\x01\xFCV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\xD3a\x03\xDC` \x93a\x03\xE1\x93a\x03\xCA\x81a\x03\x92V[\x93\x84\x80\x93a\x03\x96V[\x95\x86\x91\x01a\x03\x9FV[a\x03\xAAV[\x01\x90V[a\x03\xFA\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xB4V[\x90V[4a\x04.Wa\x04*a\x04\x19a\x04\x136`\x04a\x03\\V[\x90a\x0F[V[a\x04!a\x01\xF2V[\x91\x82\x91\x82a\x03\xE5V[\x03\x90\xF3[a\x01\xF8V[\x1C\x90V[`\xFF\x16\x90V[a\x04M\x90`\x08a\x04R\x93\x02a\x043V[a\x047V[\x90V[\x90a\x04`\x91Ta\x04=V[\x90V[a\x04o`\x04_\x90a\x04UV[\x90V[4a\x04\xA2Wa\x04\x826`\x04a\x02\0V[a\x04\x9Ea\x04\x8Da\x04cV[a\x04\x95a\x01\xF2V[\x91\x82\x91\x82a\x02\xC3V[\x03\x90\xF3[a\x01\xF8V[_\x01\x90V[4a\x04\xDBWa\x04\xC5a\x04\xBF6`\x04a\x03\\V[\x90a\x11\x16V[a\x04\xCDa\x01\xF2V[\x80a\x04\xD7\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\xF4\x90a\x04\xE0V[\x90V[a\x05\0\x81a\x04\xEBV[\x03a\x05\x07WV[_\x80\xFD[\x90P5\x90a\x05\x18\x82a\x04\xF7V[V[\x91\x90`@\x83\x82\x03\x12a\x05BW\x80a\x056a\x05?\x92_\x86\x01a\x05\x0BV[\x93` \x01a\x05\x0BV[\x90V[a\x01\xFCV[4a\x05vWa\x05`a\x05Z6`\x04a\x05\x1AV[\x90a\x12\xC7V[a\x05ha\x01\xF2V[\x80a\x05r\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[a\x05\x84\x81a\x02\x0FV[\x03a\x05\x8BWV[_\x80\xFD[\x90P5\x90a\x05\x9C\x82a\x05{V[V[\x90` \x82\x82\x03\x12a\x05\xB7Wa\x05\xB4\x91_\x01a\x05\x8FV[\x90V[a\x01\xFCV[4a\x05\xECWa\x05\xE8a\x05\xD7a\x05\xD26`\x04a\x05\x9EV[a\x13VV[a\x05\xDFa\x01\xF2V[\x91\x82\x91\x82a\x02gV[\x03\x90\xF3[a\x01\xF8V[4a\x06\x1FWa\x06\x016`\x04a\x02\0V[a\x06\ta\x13\x91V[a\x06\x11a\x01\xF2V[\x80a\x06\x1B\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x06?\x90`\x08a\x06D\x93\x02a\x043V[a\x06$V[\x90V[\x90a\x06R\x91Ta\x06/V[\x90V[a\x06a`\x01_\x90a\x06GV[\x90V[\x90V[a\x06{a\x06va\x06\x80\x92a\x04\xE0V[a\x06dV[a\x04\xE0V[\x90V[a\x06\x8C\x90a\x06gV[\x90V[a\x06\x98\x90a\x06\x83V[\x90V[a\x06\xA4\x90a\x06\x8FV[\x90RV[\x91\x90a\x06\xBB\x90_` \x85\x01\x94\x01\x90a\x06\x9BV[V[4a\x06\xEDWa\x06\xCD6`\x04a\x02\0V[a\x06\xE9a\x06\xD8a\x06UV[a\x06\xE0a\x01\xF2V[\x91\x82\x91\x82a\x06\xA8V[\x03\x90\xF3[a\x01\xF8V[\x90V[a\x07\x05\x90`\x08a\x07\n\x93\x02a\x043V[a\x06\xF2V[\x90V[\x90a\x07\x18\x91Ta\x06\xF5V[\x90V[a\x07'`\x03_\x90a\x07\rV[\x90V[a\x073\x90a\x02\x0FV[\x90RV[\x91\x90a\x07J\x90_` \x85\x01\x94\x01\x90a\x07*V[V[4a\x07|Wa\x07\\6`\x04a\x02\0V[a\x07xa\x07ga\x07\x1BV[a\x07oa\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[4a\x07\xAFWa\x07\x916`\x04a\x02\0V[a\x07\x99a\x13\xC0V[a\x07\xA1a\x01\xF2V[\x80a\x07\xAB\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[\x91``\x83\x83\x03\x12a\x08\x01Wa\x07\xCB\x82_\x85\x01a\x05\x0BV[\x92a\x07\xD9\x83` \x83\x01a\x05\x0BV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\xFCWa\x07\xF8\x92\x01a\x03\x1DV[\x90\x91V[a\x03\rV[a\x01\xFCV[4a\x08:Wa\x086a\x08%a\x08\x1C6`\x04a\x07\xB4V[\x92\x91\x90\x91a\x14xV[a\x08-a\x01\xF2V[\x91\x82\x91\x82a\x02\xC3V[\x03\x90\xF3[a\x01\xF8V[4a\x08oWa\x08O6`\x04a\x02\0V[a\x08ka\x08Za\x15EV[a\x08ba\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[4a\x08\xA3Wa\x08\x8Da\x08\x876`\x04a\x03\\V[\x90a\x16RV[a\x08\x95a\x01\xF2V[\x80a\x08\x9F\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[4a\x08\xD8Wa\x08\xB86`\x04a\x02\0V[a\x08\xD4a\x08\xC3a\x16zV[a\x08\xCBa\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[a\x08\xEA`\x04`\x01\x90a\x04UV[\x90V[4a\t\x1DWa\x08\xFD6`\x04a\x02\0V[a\t\x19a\t\x08a\x08\xDDV[a\t\x10a\x01\xF2V[\x91\x82\x91\x82a\x02\xC3V[\x03\x90\xF3[a\x01\xF8V[4a\tRWa\t26`\x04a\x02\0V[a\tNa\t=a\x17\x0CV[a\tEa\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[a\t`\x90a\x04\xEBV[\x90RV[\x91\x90a\tw\x90_` \x85\x01\x94\x01\x90a\tWV[V[4a\t\xA9Wa\t\x896`\x04a\x02\0V[a\t\xA5a\t\x94a\x17[V[a\t\x9Ca\x01\xF2V[\x91\x82\x91\x82a\tdV[\x03\x90\xF3[a\x01\xF8V[4a\t\xDEWa\t\xBE6`\x04a\x02\0V[a\t\xDAa\t\xC9a\x17\x8FV[a\t\xD1a\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[a\t\xEF`\x02_\x90a\x07\rV[\x90V[4a\n\"Wa\n\x026`\x04a\x02\0V[a\n\x1Ea\n\ra\t\xE3V[a\n\x15a\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[4a\nWWa\n76`\x04a\x02\0V[a\nSa\nBa\x17\xDBV[a\nJa\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\n\x96W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\x91W` \x01\x92` \x83\x02\x84\x01\x11a\n\x8CWV[a\x03\x19V[a\x03\x15V[a\x03\x11V[\x90` \x82\x82\x03\x12a\n\xCCW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xC7Wa\n\xC3\x92\x01a\n\\V[\x90\x91V[a\x03\rV[a\x01\xFCV[4a\x0B\0Wa\n\xEAa\n\xE46`\x04a\n\x9BV[\x90a\x19\xA0V[a\n\xF2a\x01\xF2V[\x80a\n\xFC\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[\x90` \x82\x82\x03\x12a\x0B\x1EWa\x0B\x1B\x91_\x01a\x05\x0BV[\x90V[a\x01\xFCV[4a\x0BQWa\x0B;a\x0B66`\x04a\x0B\x05V[a\x1APV[a\x0BCa\x01\xF2V[\x80a\x0BM\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B\xAAWa\x0B\x8A6`\x04a\x02\0V[a\x0B\xA6a\x0B\x95a\x0BVV[a\x0B\x9Da\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[4a\x0B\xDDWa\x0B\xBF6`\x04a\x02\0V[a\x0B\xC7a\x1AwV[a\x0B\xCFa\x01\xF2V[\x80a\x0B\xD9\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[a\x0B\xF6a\x0B\xF1a\x0B\xFB\x92a\x02\x0FV[a\x06dV[a\x02\x0FV[\x90V[\x90a\x0C\x08\x90a\x0B\xE2V[_R` R`@_ \x90V[_\x1C\x90V[a\x0C%a\x0C*\x91a\x0C\x14V[a\x06\xF2V[\x90V[a\x0C7\x90Ta\x0C\x19V[\x90V[a\x0CE\x90`\x05a\x0B\xFEV[\x90a\x0CQ_\x83\x01a\x0C-V[\x91a\x0C^`\x01\x82\x01a\x0C-V[\x91a\x0Cw`\x03a\x0Cp`\x02\x85\x01a\x0C-V[\x93\x01a\x0C-V[\x90V[a\x0C\xAFa\x0C\xB6\x94a\x0C\xA5``\x94\x98\x97\x95a\x0C\x9B`\x80\x86\x01\x9A_\x87\x01\x90a\x07*V[` \x85\x01\x90a\x07*V[`@\x83\x01\x90a\x07*V[\x01\x90a\x07*V[V[4a\x0C\xECWa\x0C\xE8a\x0C\xD3a\x0C\xCE6`\x04a\x05\x9EV[a\x0C:V[\x90a\x0C\xDF\x94\x92\x94a\x01\xF2V[\x94\x85\x94\x85a\x0CzV[\x03\x90\xF3[a\x01\xF8V[4a\r\x1FWa\r\ta\r\x046`\x04a\x0B\x05V[a\x1A\xE6V[a\r\x11a\x01\xF2V[\x80a\r\x1B\x81a\x04\xA7V[\x03\x90\xF3[a\x01\xF8V[\x91\x90`@\x83\x82\x03\x12a\rLW\x80a\r@a\rI\x92_\x86\x01a\x05\x8FV[\x93` \x01a\x05\x8FV[\x90V[a\x01\xFCV[4a\r\x82Wa\r~a\rma\rg6`\x04a\r$V[\x90a\x1B|V[a\rua\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[a\r\x93`\x06_\x90a\x07\rV[\x90V[4a\r\xC6Wa\r\xA66`\x04a\x02\0V[a\r\xC2a\r\xB1a\r\x87V[a\r\xB9a\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xF3[a\x01\xF8V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\r\xED\x90a\x03\xAAV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\x07W`@RV[a\r\xCFV[\x90a\x0E\x1Fa\x0E\x18a\x01\xF2V[\x92\x83a\r\xE3V[V[a\x0E+`\x80a\x0E\x0CV[\x90V[_\x90V[a\x0E:a\x0E!V[\x90` \x80\x80\x80\x85a\x0EIa\x0E.V[\x81R\x01a\x0ETa\x0E.V[\x81R\x01a\x0E_a\x0E.V[\x81R\x01a\x0Eja\x0E.V[\x81RPPV[a\x0Exa\x0E2V[\x90V[a\x0E\x83a\x0EpV[Pa\x0E\x8Ca\x1C&V[\x90V[_\x90V[a\x0E\x9Fa\x0E\xA4\x91a\x0C\x14V[a\x047V[\x90V[a\x0E\xB1\x90Ta\x0E\x93V[\x90V[a\x0E\xBCa\x0E\x8FV[Pa\x0E\xC7`\x04a\x0E\xA7V[\x90V[``\x90V[\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x0E\xF5a\x0E\xF0a\x0E\xFA\x92a\x0E\xCFV[a\x0E\xDBV[a\x0E\xD2V[\x90V[\x90V[a\x0F\x0Ca\x0F\x11\x91a\x0E\xD2V[a\x0E\xFDV[\x90RV[\x90P\x90V[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x0F5\x81a\x0F<\x93a\x0F\x15V[\x80\x93a\x0F\x1AV[\x01\x90V[\x80a\x0FQ`\x01\x92a\x0FX\x96\x94a\x0F\0V[\x01\x91a\x0F%V[\x90V[a\x0F\x99\x90a\x0Fga\x0E\xCAV[Pa\x0F\x8Aa\x0Ft_a\x0E\xE1V[\x91\x93a\x0F~a\x01\xF2V[\x94\x85\x93` \x85\x01a\x0F@V[` \x82\x01\x81\x03\x82R\x03\x82a\r\xE3V[\x90V[\x90a\x0F\xB8a\x0F\xB232\x90\x85\x85\x91\x92\x90\x91\x92a\x14xV[\x15a\x02\xB1V[a\x0F\xC7Wa\x0F\xC5\x91a\x10CV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x0F\xDF`\x04\x82\x01a\x04\xA7V[\x03\x90\xFD[`\x08\x1C\x90V[a\x0F\xF5a\x0F\xFA\x91a\x0F\xE3V[a\x047V[\x90V[a\x10\x07\x90Ta\x0F\xE9V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x10-a\x103\x91\x93\x92\x93a\x02\x0FV[\x92a\x02\x0FV[\x82\x03\x91\x82\x11a\x10>WV[a\x10\nV[\x90a\x10Wa\x10Q`\x04a\x0F\xFDV[\x15a\x02\xB1V[a\x10{Wa\x10y\x91a\x10ma\x10t\x92Z\x92a\x10\xCFV[Z\x90a\x10\x1EV[a\x1E\0V[V[a\x10\x84\x91a\x10\xCFV[V[a\x10\x8F\x90a\x06\x83V[\x90V[\x91\x90a\x10\xAC\x81a\x10\xA5\x81a\x10\xB1\x95a\x03\x96V[\x80\x95a\x0F\x1AV[a\x03\xAAV[\x01\x90V[\x90\x91a\x10\xCC\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x10\x92V[\x90V[3\x90\x91a\x10\xFC\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\x86V[\x92a\x11\x11a\x11\x08a\x01\xF2V[\x92\x83\x92\x83a\x10\xB5V[\x03\x90\xA2V[\x90a\x11 \x91a\x0F\x9CV[V[\x90a\x114\x91a\x11/a\x1F\x07V[a\x12:V[V[`\xA0\x1C\x90V[a\x11Ha\x11M\x91a\x116V[a\x047V[\x90V[a\x11Z\x90Ta\x11<V[\x90V[a\x11qa\x11la\x11v\x92a\x0E\xCFV[a\x06dV[a\x04\xE0V[\x90V[a\x11\x82\x90a\x11]V[\x90V[`\xA0\x1B\x90V[\x90a\x11\x9A`\xFF`\xA0\x1B\x91a\x11\x85V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x11\xAD\x90a\x02\xB1V[\x90V[\x90V[\x90a\x11\xC8a\x11\xC3a\x11\xCF\x92a\x11\xA4V[a\x11\xB0V[\x82Ta\x11\x8BV[\x90UV[a\x11\xDC\x90a\x06gV[\x90V[a\x11\xE8\x90a\x11\xD3V[\x90V[_\x1B\x90V[\x90a\x12\x01`\x01\x80`\xA0\x1B\x03\x91a\x11\xEBV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x12\x14\x90a\x11\xD3V[\x90V[\x90V[\x90a\x12/a\x12*a\x126\x92a\x12\x0BV[a\x12\x17V[\x82Ta\x11\xF0V[\x90UV[a\x12D`\x01a\x11PV[a\x12\xACW\x81a\x12ca\x12]a\x12X_a\x11yV[a\x04\xEBV[\x91a\x04\xEBV[\x14a\x12\x90Wa\x12\x89a\x12\x82a\x12\x8E\x93a\x12}`\x01\x80a\x11\xB3V[a\x11\xDFV[`\x01a\x12\x1AV[a\x1A\xE6V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x12\xA8`\x04\x82\x01a\x04\xA7V[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x12\xC3`\x04\x82\x01a\x04\xA7V[\x03\x90\xFD[\x90a\x12\xD1\x91a\x11\"V[V[\x90a\x12\xDD\x90a\x02\x0FV[\x90RV[\x90a\x13Ha\x13?`\x03a\x12\xF2a\x0E!V[\x94a\x13\ta\x13\x01_\x83\x01a\x0C-V[_\x88\x01a\x12\xD3V[a\x13!a\x13\x18`\x01\x83\x01a\x0C-V[` \x88\x01a\x12\xD3V[a\x139a\x130`\x02\x83\x01a\x0C-V[`@\x88\x01a\x12\xD3V[\x01a\x0C-V[``\x84\x01a\x12\xD3V[V[a\x13S\x90a\x12\xE1V[\x90V[a\x13ma\x13r\x91a\x13ea\x0EpV[P`\x05a\x0B\xFEV[a\x13JV[\x90V[a\x13}a\x1F\x07V[a\x13\x85a\x13\x87V[V[a\x13\x8Fa\x1F\x92V[V[a\x13\x99a\x13uV[V[a\x13\xA3a\x1F\x07V[a\x13\xABa\x13\xADV[V[a\x13\xBEa\x13\xB9_a\x11yV[a\x1F\xC2V[V[a\x13\xC8a\x13\x9BV[V[a\x13\xD6a\x13\xDB\x91a\x0C\x14V[a\x06$V[\x90V[a\x13\xE8\x90Ta\x13\xCAV[\x90V[`\xE0\x1B\x90V[a\x13\xFA\x81a\x02\xB1V[\x03a\x14\x01WV[_\x80\xFD[\x90PQ\x90a\x14\x12\x82a\x13\xF1V[V[\x90` \x82\x82\x03\x12a\x14-Wa\x14*\x91_\x01a\x14\x05V[\x90V[a\x01\xFCV[a\x14Xa\x14e\x95\x93\x94\x92\x94a\x14N``\x84\x01\x96_\x85\x01\x90a\tWV[` \x83\x01\x90a\tWV[`@\x81\x85\x03\x91\x01Ra\x10\x92V[\x90V[a\x14pa\x01\xF2V[=_\x82>=\x90\xFD[\x92a\x14\xBB` \x93\x94a\x14\x88a\x0E\x8FV[Pa\x14\xC6a\x14\x9Ea\x14\x99`\x01a\x13\xDEV[a\x06\x8FV[\x93cz9y\xDC\x92\x95\x97a\x14\xAFa\x01\xF2V[\x98\x89\x97\x88\x96\x87\x96a\x13\xEBV[\x86R`\x04\x86\x01a\x142V[\x03\x91Z\xFA\x90\x81\x15a\x15\nW_\x91a\x14\xDCW[P\x90V[a\x14\xFD\x91P` =\x81\x11a\x15\x03W[a\x14\xF5\x81\x83a\r\xE3V[\x81\x01\x90a\x14\x14V[_a\x14\xD8V[P=a\x14\xEBV[a\x14hV[_\x90V[a\x15\x1D\x90Qa\x02\x0FV[\x90V[a\x15/a\x155\x91\x93\x92\x93a\x02\x0FV[\x92a\x02\x0FV[\x82\x01\x80\x92\x11a\x15@WV[a\x10\nV[a\x15Ma\x15\x0FV[Pa\x15ta\x15[`\x06a\x0C-V[a\x15n``a\x15ha\x1C&V[\x01a\x15\x13V[\x90a\x15 V[\x90V[\x90a\x15\x93a\x15\x8D32\x90\x85\x85\x91\x92\x90\x91\x92a\x14xV[\x15a\x02\xB1V[a\x15\xA2Wa\x15\xA0\x91a\x15\xBEV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x15\xBA`\x04\x82\x01a\x04\xA7V[\x03\x90\xFD[\x90a\x15\xD2a\x15\xCC`\x04a\x0F\xFDV[\x15a\x02\xB1V[a\x15\xF6Wa\x15\xF4\x91a\x15\xE8a\x15\xEF\x92Z\x92a\x16\x01V[Z\x90a\x10\x1EV[a\x1E\0V[V[a\x15\xFF\x91a\x16\x01V[V[\x90a\x16\r\x903\x92a\x0F[V[\x90a\x16Ma\x16;\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\x86V[\x92a\x16Da\x01\xF2V[\x91\x82\x91\x82a\x03\xE5V[\x03\x90\xA2V[\x90a\x16\\\x91a\x15wV[V[a\x16ra\x16ma\x16w\x92a\x0E\xCFV[a\x06dV[a\x02\x0FV[\x90V[a\x16\x82a\x15\x0FV[Pa\x16\x8Ba\x1C&V[a\x16\x96_\x82\x01a\x15\x13V[a\x16\xA8a\x16\xA2_a\x16^V[\x91a\x02\x0FV[\x14a\x16\xFFWa\x16\xBB_a\x16\xCB\x92\x01a\x15\x13V[a\x16\xC5`\x02a\x0C-V[\x90a\x15 V[Ba\x16\xDEa\x16\xD8\x83a\x02\x0FV[\x91a\x02\x0FV[\x10\x15a\x16\xF2Wa\x16\xEF\x90B\x90a\x10\x1EV[\x90V[Pa\x16\xFC_a\x16^V[\x90V[Pa\x17\t_a\x16^V[\x90V[a\x17\x14a\x15\x0FV[Pa\x17(``a\x17\"a\x1C&V[\x01a\x15\x13V[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x17Fa\x17K\x91a\x0C\x14V[a\x17/V[\x90V[a\x17X\x90Ta\x17:V[\x90V[a\x17ca\x17+V[Pa\x17m_a\x17NV[\x90V[\x90V[a\x17\x87a\x17\x82a\x17\x8C\x92a\x17pV[a\x06dV[a\x02\x0FV[\x90V[a\x17\x97a\x15\x0FV[Pa\x17\xABa\x17\xA5`\x04a\x0E\xA7V[\x15a\x02\xB1V[a\x17\xCFWa\x17\xCCa\x17\xBC`\x03a\x0C-V[a\x17\xC6`\x01a\x17sV[\x90a\x15 V[\x90V[a\x17\xD8_a\x16^V[\x90V[a\x17\xE3a\x15\x0FV[Pa\x17\xF7`@a\x17\xF1a\x1C&V[\x01a\x15\x13V[\x90V[\x90a\x18\x0Ea\x18\x08`\x04a\x0F\xFDV[\x15a\x02\xB1V[a\x182Wa\x180\x91a\x18$a\x18+\x92Z\x92a\x18\xD7V[Z\x90a\x10\x1EV[a\x1E\0V[V[a\x18;\x91a\x18\xD7V[V[P\x90V[`\x01a\x18M\x91\x01a\x02\x0FV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x18\xB2W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x18\xADW` \x01\x91`\x01\x82\x026\x03\x83\x13a\x18\xA8WV[a\x18lV[a\x18hV[a\x18dV[\x90\x82\x10\x15a\x18\xD2W` a\x18\xCE\x92\x02\x81\x01\x90a\x18pV[\x90\x91V[a\x18PV[a\x18\xE2\x81\x83\x90a\x18=V[\x91a\x18\xEBa\x15\x0FV[Pa\x18\xF5_a\x16^V[[\x80a\x19\ta\x19\x03\x86a\x02\x0FV[\x91a\x02\x0FV[\x10\x15a\x19\x9AWa\x197\x90a\x19-32\x90a\x19%\x87\x87\x86\x91a\x18\xB7V[\x92\x90\x91a\x14xV[a\x19<W[a\x18AV[a\x18\xF6V[3a\x19Ra\x19L\x86\x86\x85\x91a\x18\xB7V[\x90a\x0F[V[\x90a\x19\x92a\x19\x80\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\x86V[\x92a\x19\x89a\x01\xF2V[\x91\x82\x91\x82a\x03\xE5V[\x03\x90\xA2a\x192V[PPPPV[\x90a\x19\xAA\x91a\x17\xFAV[V[a\x19\xBD\x90a\x19\xB8a\x1F\x07V[a\x19\xBFV[V[\x80a\x19\xDAa\x19\xD4a\x19\xCF_a\x11yV[a\x04\xEBV[\x91a\x04\xEBV[\x14a\x1A4Wa\x19\xF2a\x19\xEB\x82a\x11\xDFV[`\x01a\x12\x1AV[a\x1A\x1C\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10\x86V[\x90a\x1A%a\x01\xF2V[\x80a\x1A/\x81a\x04\xA7V[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x1AL`\x04\x82\x01a\x04\xA7V[\x03\x90\xFD[a\x1AY\x90a\x19\xACV[V[a\x1Aca\x1F\x07V[a\x1Aka\x1AmV[V[a\x1Aua !V[V[a\x1A\x7Fa\x1A[V[V[a\x1A\x92\x90a\x1A\x8Da\x1F\x07V[a\x1A\x94V[V[\x80a\x1A\xAFa\x1A\xA9a\x1A\xA4_a\x11yV[a\x04\xEBV[\x91a\x04\xEBV[\x14a\x1A\xBFWa\x1A\xBD\x90a\x1F\xC2V[V[a\x1A\xE2a\x1A\xCB_a\x11yV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\tdV[\x03\x90\xFD[a\x1A\xEF\x90a\x1A\x81V[V[` \x91\x81R\x01\x90V[_\x7FGasCounter: invalid range\0\0\0\0\0\0\0\x91\x01RV[a\x1B.`\x19` \x92a\x1A\xF1V[a\x1B7\x81a\x1A\xFAV[\x01\x90V[a\x1BP\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1B!V[\x90V[\x15a\x1BZWV[a\x1Bba\x01\xF2V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1Bx`\x04\x82\x01a\x1B;V[\x03\x90\xFD[a\x1B\xAB\x91a\x1B\x88a\x15\x0FV[Pa\x1B\xA6\x81a\x1B\x9Fa\x1B\x99\x85a\x02\x0FV[\x91a\x02\x0FV[\x10\x15a\x1BSV[a\x10\x1EV[\x90V[a\x1B\xB8`\x80a\x0E\x0CV[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\x1B\xDBa\x1B\xE1\x91a\x02\x0FV[\x91a\x02\x0FV[\x90\x81\x15a\x1B\xECW\x04\x90V[a\x1B\xBBV[a\x1C\0a\x1C\x06\x91\x93\x92\x93a\x02\x0FV[\x92a\x02\x0FV[\x91a\x1C\x12\x83\x82\x02a\x02\x0FV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1C!WV[a\x10\nV[a\x1C.a\x0EpV[Pa\x1CBa\x1C<`\x04a\x0E\xA7V[\x15a\x02\xB1V[a\x1DDWa\x1Cca\x1C^`\x05a\x1CX`\x03a\x0C-V[\x90a\x0B\xFEV[a\x13JV[Ba\x1C\x93a\x1C\x8Da\x1C\x88a\x1Cx_\x86\x01a\x15\x13V[a\x1C\x82`\x02a\x0C-V[\x90a\x15 V[a\x02\x0FV[\x91a\x02\x0FV[\x10\x15a\x1C\x9CW\x90V[a\x1C\xED\x90a\x1C\xE7a\x1C\xD6_a\x1C\xCFa\x1C\xBFBa\x1C\xB9\x84\x88\x01a\x15\x13V[\x90a\x10\x1EV[a\x1C\xC9`\x02a\x0C-V[\x90a\x1B\xCFV[\x93\x01a\x15\x13V[\x91a\x1C\xE1`\x02a\x0C-V[\x90a\x1B\xF1V[\x90a\x15 V[a\x1DAa\x1D8_a\x1D3a\x1D*_a\x1D%a\x1D\x1C_\x95a\x1D\x17a\x1D\x0Ea\x1B\xAEV[\x9A_\x8C\x01a\x12\xD3V[a\x16^V[` \x89\x01a\x12\xD3V[a\x16^V[`@\x86\x01a\x12\xD3V[a\x16^V[``\x83\x01a\x12\xD3V[\x90V[_a\x1D\xA1a\x1D\x98_a\x1D\x93a\x1D\x8A_a\x1D\x85a\x1D|_\x95a\x1Dwa\x1Doa\x1Dia\x1B\xAEV[\x9Ba\x16^V[_\x8C\x01a\x12\xD3V[a\x16^V[` \x89\x01a\x12\xD3V[a\x16^V[`@\x86\x01a\x12\xD3V[a\x16^V[``\x83\x01a\x12\xD3V[\x90V[\x90a\x1D\xB0_\x19\x91a\x11\xEBV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x1D\xD2a\x1D\xCDa\x1D\xD9\x92a\x0B\xE2V[a\x1D\xBAV[\x82Ta\x1D\xA4V[\x90UV[\x91` a\x1D\xFE\x92\x94\x93a\x1D\xF7`@\x82\x01\x96_\x83\x01\x90a\x07*V[\x01\x90a\x07*V[V[a\x1E\x13a\x1E\r`\x04a\x0F\xFDV[\x15a\x02\xB1V[a\x1F\x04Wa\x1E*a\x1E$`\x04a\x0E\xA7V[\x15a\x02\xB1V[a\x1E\xF7W[a\x1E7a!\xF7V[a\x1E\xA8a\x1EE\x82:\x90a\x1B\xF1V[a\x1Ey\x83a\x1Es`\x02a\x1Ec`\x05a\x1E]`\x03a\x0C-V[\x90a\x0B\xFEV[\x01\x91a\x1En\x83a\x0C-V[a\x15 V[\x90a\x1D\xBDV[a\x1E\xA2`\x03a\x1E\x92`\x05a\x1E\x8C\x83a\x0C-V[\x90a\x0B\xFEV[\x01\x91a\x1E\x9D\x83a\x0C-V[a\x15 V[\x90a\x1D\xBDV[a\x1E\xB2`\x03a\x0C-V[:a\x1E\xDD\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0B\xE2V[\x92a\x1E\xF2a\x1E\xE9a\x01\xF2V[\x92\x83\x92\x83a\x1D\xDDV[\x03\x90\xA2V[a\x1E\xFFa \xECV[a\x1E/V[PV[a\x1F\x0Fa\x17[V[a\x1F(a\x1F\"a\x1F\x1Da#\xFBV[a\x04\xEBV[\x91a\x04\xEBV[\x03a\x1F/WV[a\x1FQa\x1F:a#\xFBV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\tdV[\x03\x90\xFD[`\x08\x1B\x90V[\x90a\x1Fha\xFF\0\x91a\x1FUV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1F\x87a\x1F\x82a\x1F\x8E\x92a\x11\xA4V[a\x11\xB0V[\x82Ta\x1F[V[\x90UV[a\x1F\x9D_`\x04a\x1FrV[V[\x90V[\x90a\x1F\xB7a\x1F\xB2a\x1F\xBE\x92a\x10\x86V[a\x1F\x9FV[\x82Ta\x11\xF0V[\x90UV[a\x1F\xCB_a\x17NV[a\x1F\xD5\x82_a\x1F\xA2V[\x90a \ta \x03\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10\x86V[\x91a\x10\x86V[\x91a \x12a\x01\xF2V[\x80a \x1C\x81a\x04\xA7V[\x03\x90\xA3V[a -`\x01`\x04a\x1FrV[V[\x90a ;`\xFF\x91a\x11\xEBV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a Za Ua a\x92a\x11\xA4V[a\x11\xB0V[\x82Ta /V[\x90UV[\x90a o\x90a\x16^V[_R` R`@_ \x90V[\x90a \xD8```\x03a \xDE\x94a \x9E_\x82\x01a \x98_\x88\x01a\x15\x13V[\x90a\x1D\xBDV[a \xB7`\x01\x82\x01a \xB1` \x88\x01a\x15\x13V[\x90a\x1D\xBDV[a \xD0`\x02\x82\x01a \xCA`@\x88\x01a\x15\x13V[\x90a\x1D\xBDV[\x01\x92\x01a\x15\x13V[\x90a\x1D\xBDV[V[\x90a \xEA\x91a {V[V[a \xFFa \xF9`\x04a\x0E\xA7V[\x15a\x02\xB1V[a!\x06W[V[a!\x12`\x01`\x04a EV[a!%a!\x1E_a\x16^V[`\x03a\x1D\xBDV[a!\x8EBa!}a!t_a!oa!f_a!aa!X_\x95a!Sa!Ja\x1B\xAEV[\x9A_\x8C\x01a\x12\xD3V[a\x16^V[` \x89\x01a\x12\xD3V[a\x16^V[`@\x86\x01a\x12\xD3V[a\x16^V[``\x83\x01a\x12\xD3V[a!\x89`\x05_\x90a eV[a \xE0V[_B\x90a!\xD0a!\xBE\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x16^V[\x92a!\xC7a\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xA2a!\x04V[\x90V[a!\xE4\x90a\x02\x0FV[_\x19\x81\x14a!\xF2W`\x01\x01\x90V[a\x10\nV[a\"\x14a\"\x0F`\x05a\"\t`\x03a\x0C-V[\x90a\x0B\xFEV[a!\xD8V[Ba\"Da\">a\"9a\")_\x86\x01a\x0C-V[a\"3`\x02a\x0C-V[\x90a\x15 V[a\x02\x0FV[\x91a\x02\x0FV[\x10\x15a\"NW[PV[a\"xa\"oa\"__\x84\x01a\x0C-V[a\"i`\x02a\x0C-V[\x90a\x15 V[`\x01\x83\x01a\x1D\xBDV[a\"\xA0a\"\x99a\"\x8A`\x03\x84\x01a\x0C-V[a\"\x94`\x06a\x0C-V[a\x15 V[`\x06a\x1D\xBDV[a\"\xAA`\x03a\x0C-V[a\"\xD7a\"\xB9`\x02\x84\x01a\x0C-V[\x92a\"\xD1_a\"\xCA`\x01\x84\x01a\x0C-V[\x92\x01a\x0C-V[\x90a\x10\x1EV[a#\x01\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0B\xE2V[\x92a#\x16a#\ra\x01\xF2V[\x92\x83\x92\x83a\x1D\xDDV[\x03\x90\xA2a#5a#.a#)`\x03a\x0C-V[a!\xDBV[`\x03a\x1D\xBDV[a#\xA7Ba#\x8Da#\x84_a#\x7Fa#v_a#qa#h_\x95a#ca#Za\x1B\xAEV[\x9A_\x8C\x01a\x12\xD3V[a\x16^V[` \x89\x01a\x12\xD3V[a\x16^V[`@\x86\x01a\x12\xD3V[a\x16^V[``\x83\x01a\x12\xD3V[a#\xA2`\x05a#\x9C`\x03a\x0C-V[\x90a\x0B\xFEV[a \xE0V[a#\xB1`\x03a\x0C-V[B\x90a#\xF2a#\xE0\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0B\xE2V[\x92a#\xE9a\x01\xF2V[\x91\x82\x91\x82a\x077V[\x03\x90\xA2_a\"KV[a$\x03a\x17+V[P3\x90V",
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
        periodDuration(periodDurationCall),
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
            [113u8, 80u8, 24u8, 166u8],
            [122u8, 57u8, 121u8, 220u8],
            [127u8, 189u8, 41u8, 94u8],
            [128u8, 78u8, 81u8, 35u8],
            [130u8, 244u8, 74u8, 222u8],
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
    impl alloy_sol_types::SolInterface for SyndicateSequencingChainCalls {
        const NAME: &'static str = "SyndicateSequencingChainCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
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
                    fn periodDuration(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <periodDurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::periodDuration)
                    }
                    periodDuration
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
