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
    ///0x60a060405234610038576100196100146100e9565b6101b7565b61002161003d565b6122b3610516823960805181610b2d01526122b390f35b610043565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b9061006f90610047565b810190811060018060401b0382111761008757604052565b610051565b9061009f61009861003d565b9283610065565b565b5f80fd5b90565b6100b1816100a5565b036100b857565b5f80fd5b905051906100c9826100a8565b565b906020828203126100e4576100e1915f016100bc565b90565b6100a1565b610107612988803803806100fc8161008c565b9283398101906100cb565b90565b90565b90565b61012461011f6101299261010a565b61010d565b6100a5565b90565b60209181520190565b5f7f41707020636861696e2049442063616e6e6f7420626520300000000000000000910152565b610169601860209261012c565b61017281610135565b0190565b61018b9060208101905f81830391015261015c565b90565b1561019557565b61019d61003d565b62461bcd60e51b8152806101b360048201610176565b0390fd5b6101bf610232565b6101dc816101d56101cf5f610110565b916100a5565b141561018e565b608052565b60081b90565b906101f461ff00916101e1565b9181191691161790565b151590565b61020c906101fe565b90565b90565b9061022761022261022e92610203565b61020f565b82546101e7565b9055565b61023a61032a565b61024660016003610212565b565b60a01b90565b9061025d60ff60a01b91610248565b9181191691161790565b9061027c61027761028392610203565b61020f565b825461024e565b9055565b5f0190565b61029461003d565b3d5f823e3d90fd5b60018060a01b031690565b6102bb6102b66102c09261029c565b61010d565b61029c565b90565b6102cc906102a7565b90565b6102d8906102c3565b90565b5f1b90565b906102f160018060a01b03916102db565b9181191691161790565b610304906102c3565b90565b90565b9061031f61031a610326926102fb565b610307565b82546102e0565b9055565b61033333610397565b61033e5f6001610267565b61034661003d565b6101bf810181811060018060401b038211176103925761036e82916101bf6127c98439610287565b03905ff0801561038d5761038461038b916102cf565b600161030a565b565b61028c565b610051565b6103a0906103f8565b565b6103b66103b16103bb9261010a565b61010d565b61029c565b90565b6103c7906103a2565b90565b6103d39061029c565b90565b6103df906103ca565b9052565b91906103f6905f602085019401906103d6565b565b8061041361040d6104085f6103be565b6103ca565b916103ca565b1461042357610421906104b6565b565b61044661042f5f6103be565b5f918291631e4fbdf760e01b8352600483016103e3565b0390fd5b5f1c90565b60018060a01b031690565b61046661046b9161044a565b61044f565b90565b610478905461045a565b90565b610484906102a7565b90565b6104909061047b565b90565b90565b906104ab6104a66104b292610487565b610493565b82546102e0565b9055565b6104bf5f61046e565b6104c9825f610496565b906104fd6104f77f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093610487565b91610487565b9161050661003d565b8061051081610287565b0390a356fe60806040526004361015610013575b610d66565b61001d5f356101cc565b8063086146d2146101c757806318d5aafe146101c2578063366cbab7146101bd5780633b6ab2a9146101b857806346e2cc09146101b3578063485cc955146101ae5780634b2c0706146101a95780635467cb48146101a45780635b3cd6e21461019f578063615438011461019a5780636558954f14610195578063715018a6146101905780637a3979dc1461018b578063804e51231461018657806382f44ade1461018157806384fab62b1461017c5780638d5a239b146101775780638da5cb5b14610172578063aff74c6d1461016d578063c660d3f314610168578063cdafb97814610163578063d4f0eb4d1461015e578063d878134214610159578063de1f453e14610154578063ea4a11041461014f578063ede07bd61461014a5763f2fde38b0361000e57610d33565b610cfe565b610c8d565b610b84565b610b4f565b610af8565b610aa6565b6109fc565b6109c7565b610992565b61093b565b610906565b6108c1565b61088d565b610854565b6107cf565b61079a565b61072c565b61069d565b6105d1565b61059c565b610527565b61048c565b610452565b6103dd565b6102b8565b61025c565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f9103126101ea57565b6101dc565b90565b6101fb906101ef565b9052565b90606080610245936102175f8201515f8601906101f2565b610229602082015160208601906101f2565b61023b604082015160408601906101f2565b01519101906101f2565b565b919061025a905f608085019401906101ff565b565b3461028c5761026c3660046101e0565b610288610277610e16565b61027f6101d2565b91829182610247565b0390f35b6101d8565b151590565b61029f90610291565b9052565b91906102b6905f60208501940190610296565b565b346102e8576102c83660046101e0565b6102e46102d3610e4f565b6102db6101d2565b918291826102a3565b0390f35b6101d8565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156103375781359167ffffffffffffffff831161033257602001926001830284011161032d57565b6102f9565b6102f5565b6102f1565b9060208282031261036d575f82013567ffffffffffffffff81116103685761036492016102fd565b9091565b6102ed565b6101dc565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6103b36103bc6020936103c1936103aa81610372565b93848093610376565b9586910161037f565b61038a565b0190565b6103da9160208201915f818403910152610394565b90565b3461040e5761040a6103f96103f336600461033c565b90610ef6565b6104016101d2565b918291826103c5565b0390f35b6101d8565b1c90565b60ff1690565b61042d9060086104329302610413565b610417565b90565b90610440915461041d565b90565b61044f60035f90610435565b90565b34610482576104623660046101e0565b61047e61046d610443565b6104756101d2565b918291826102a3565b0390f35b6101d8565b5f0190565b346104bb576104a561049f36600461033c565b906110e7565b6104ad6101d2565b806104b781610487565b0390f35b6101d8565b60018060a01b031690565b6104d4906104c0565b90565b6104e0816104cb565b036104e757565b5f80fd5b905035906104f8826104d7565b565b9190604083820312610522578061051661051f925f86016104eb565b936020016104eb565b90565b6101dc565b346105565761054061053a3660046104fa565b90611298565b6105486101d2565b8061055281610487565b0390f35b6101d8565b610564816101ef565b0361056b57565b5f80fd5b9050359061057c8261055b565b565b9060208282031261059757610594915f0161056f565b90565b6101dc565b346105cc576105c86105b76105b236600461057e565b611327565b6105bf6101d2565b91829182610247565b0390f35b6101d8565b346105ff576105e13660046101e0565b6105e9611362565b6105f16101d2565b806105fb81610487565b0390f35b6101d8565b60018060a01b031690565b61061f9060086106249302610413565b610604565b90565b90610632915461060f565b90565b61064160015f90610627565b90565b90565b61065b610656610660926104c0565b610644565b6104c0565b90565b61066c90610647565b90565b61067890610663565b90565b6106849061066f565b9052565b919061069b905f6020850194019061067b565b565b346106cd576106ad3660046101e0565b6106c96106b8610635565b6106c06101d2565b91829182610688565b0390f35b6101d8565b90565b6106e59060086106ea9302610413565b6106d2565b90565b906106f891546106d5565b90565b61070760025f906106ed565b90565b610713906101ef565b9052565b919061072a905f6020850194019061070a565b565b3461075c5761073c3660046101e0565b6107586107476106fb565b61074f6101d2565b91829182610717565b0390f35b6101d8565b90565b61077861077361077d92610761565b610644565b6101ef565b90565b61078c62278d00610764565b90565b610797610780565b90565b346107ca576107aa3660046101e0565b6107c66107b561078f565b6107bd6101d2565b91829182610717565b0390f35b6101d8565b346107fd576107df3660046101e0565b6107e7611391565b6107ef6101d2565b806107f981610487565b0390f35b6101d8565b9160608383031261084f57610819825f85016104eb565b9261082783602083016104eb565b92604082013567ffffffffffffffff811161084a5761084692016102fd565b9091565b6102ed565b6101dc565b346108885761088461087361086a366004610802565b92919091611449565b61087b6101d2565b918291826102a3565b0390f35b6101d8565b346108bc576108a66108a036600461033c565b906115cc565b6108ae6101d2565b806108b881610487565b0390f35b6101d8565b346108f1576108d13660046101e0565b6108ed6108dc611605565b6108e46101d2565b91829182610717565b0390f35b6101d8565b6109036003600190610435565b90565b34610936576109163660046101e0565b6109326109216108f6565b6109296101d2565b918291826102a3565b0390f35b6101d8565b3461096b5761094b3660046101e0565b610967610956611695565b61095e6101d2565b91829182610717565b0390f35b6101d8565b610979906104cb565b9052565b9190610990905f60208501940190610970565b565b346109c2576109a23660046101e0565b6109be6109ad6116e4565b6109b56101d2565b9182918261097d565b0390f35b6101d8565b346109f7576109d73660046101e0565b6109f36109e2611718565b6109ea6101d2565b91829182610717565b0390f35b6101d8565b34610a2c57610a0c3660046101e0565b610a28610a17611764565b610a1f6101d2565b91829182610717565b0390f35b6101d8565b909182601f83011215610a6b5781359167ffffffffffffffff8311610a66576020019260208302840111610a6157565b6102f9565b6102f5565b6102f1565b90602082820312610aa1575f82013567ffffffffffffffff8111610a9c57610a989201610a31565b9091565b6102ed565b6101dc565b34610ad557610abf610ab9366004610a70565b9061193a565b610ac76101d2565b80610ad181610487565b0390f35b6101d8565b90602082820312610af357610af0915f016104eb565b90565b6101dc565b34610b2657610b10610b0b366004610ada565b6119ea565b610b186101d2565b80610b2281610487565b0390f35b6101d8565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610b7f57610b5f3660046101e0565b610b7b610b6a610b2b565b610b726101d2565b91829182610717565b0390f35b6101d8565b34610bb257610b943660046101e0565b610b9c611a11565b610ba46101d2565b80610bae81610487565b0390f35b6101d8565b610bcb610bc6610bd0926101ef565b610644565b6101ef565b90565b90610bdd90610bb7565b5f5260205260405f2090565b5f1c90565b610bfa610bff91610be9565b6106d2565b90565b610c0c9054610bee565b90565b610c1a906004610bd3565b90610c265f8301610c02565b91610c3360018201610c02565b91610c4c6003610c4560028501610c02565b9301610c02565b90565b610c84610c8b94610c7a606094989795610c70608086019a5f87019061070a565b602085019061070a565b604083019061070a565b019061070a565b565b34610cc157610cbd610ca8610ca336600461057e565b610c0f565b90610cb49492946101d2565b94859485610c4f565b0390f35b6101d8565b90565b610cdd610cd8610ce292610cc6565b610644565b6101ef565b90565b610cf0611388610cc9565b90565b610cfb610ce5565b90565b34610d2e57610d0e3660046101e0565b610d2a610d19610cf3565b610d216101d2565b91829182610717565b0390f35b6101d8565b34610d6157610d4b610d46366004610ada565b611a80565b610d536101d2565b80610d5d81610487565b0390f35b6101d8565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b90610d889061038a565b810190811067ffffffffffffffff821117610da257604052565b610d6a565b90610dba610db36101d2565b9283610d7e565b565b610dc66080610da7565b90565b5f90565b610dd5610dbc565b90602080808085610de4610dc9565b815201610def610dc9565b815201610dfa610dc9565b815201610e05610dc9565b81525050565b610e13610dcd565b90565b610e1e610e0b565b50610e27611b03565b90565b5f90565b610e3a610e3f91610be9565b610417565b90565b610e4c9054610e2e565b90565b610e57610e2a565b50610e626003610e42565b90565b606090565b90565b60ff60f81b1690565b60f81b90565b610e90610e8b610e9592610e6a565b610e76565b610e6d565b90565b90565b610ea7610eac91610e6d565b610e98565b9052565b905090565b90825f939282370152565b909182610ed081610ed793610eb0565b8093610eb5565b0190565b80610eec600192610ef39694610e9b565b0191610ec0565b90565b610f3490610f02610e65565b50610f25610f0f5f610e7c565b9193610f196101d2565b94859360208501610edb565b60208201810382520382610d7e565b90565b90610f53610f4d33329085859192909192611449565b15610291565b610f6257610f6091611003565b565b5f631b8e828b60e31b815280610f7a60048201610487565b0390fd5b60081c90565b610f90610f9591610f7e565b610417565b90565b610fa29054610f84565b90565b634e487b7160e01b5f52601160045260245ffd5b610fc8610fce919392936101ef565b926101ef565b8203918211610fd957565b610fa5565b610fed610ff3919392936101ef565b926101ef565b8201809211610ffe57565b610fa5565b906110176110116003610f98565b15610291565b61104c5761103761104a92611030611045935a926110a0565b5a90610fb9565b61103f610ce5565b90610fde565b611cd7565b565b611055916110a0565b565b61106090610663565b90565b919061107d816110768161108295610376565b8095610eb5565b61038a565b0190565b909161109d9260208301925f818503910152611063565b90565b3390916110cd7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611057565b926110e26110d96101d2565b92839283611086565b0390a2565b906110f191610f37565b565b9061110591611100611dde565b61120b565b565b60a01c90565b61111961111e91611107565b610417565b90565b61112b905461110d565b90565b61114261113d61114792610e6a565b610644565b6104c0565b90565b6111539061112e565b90565b60a01b90565b9061116b60ff60a01b91611156565b9181191691161790565b61117e90610291565b90565b90565b906111996111946111a092611175565b611181565b825461115c565b9055565b6111ad90610647565b90565b6111b9906111a4565b90565b5f1b90565b906111d260018060a01b03916111bc565b9181191691161790565b6111e5906111a4565b90565b90565b906112006111fb611207926111dc565b6111e8565b82546111c1565b9055565b6112156001611121565b61127d578161123461122e6112295f61114a565b6104cb565b916104cb565b146112615761125a61125361125f9361124e600180611184565b6111b0565b60016111eb565b611a80565b565b5f632e7f3c7f60e11b81528061127960048201610487565b0390fd5b5f62dc149f60e41b81528061129460048201610487565b0390fd5b906112a2916110f3565b565b906112ae906101ef565b9052565b9061131961131060036112c3610dbc565b946112da6112d25f8301610c02565b5f88016112a4565b6112f26112e960018301610c02565b602088016112a4565b61130a61130160028301610c02565b604088016112a4565b01610c02565b606084016112a4565b565b611324906112b2565b90565b61133e61134391611336610e0b565b506004610bd3565b61131b565b90565b61134e611dde565b611356611358565b565b611360611e69565b565b61136a611346565b565b611374611dde565b61137c61137e565b565b61138f61138a5f61114a565b611e99565b565b61139961136c565b565b6113a76113ac91610be9565b610604565b90565b6113b9905461139b565b90565b60e01b90565b6113cb81610291565b036113d257565b5f80fd5b905051906113e3826113c2565b565b906020828203126113fe576113fb915f016113d6565b90565b6101dc565b611429611436959394929461141f60608401965f850190610970565b6020830190610970565b6040818503910152611063565b90565b6114416101d2565b3d5f823e3d90fd5b9261148c60209394611459610e2a565b5061149761146f61146a60016113af565b61066f565b93637a3979dc9295976114806101d2565b988997889687966113bc565b865260048601611403565b03915afa9081156114db575f916114ad575b5090565b6114ce915060203d81116114d4575b6114c68183610d7e565b8101906113e5565b5f6114a9565b503d6114bc565b611439565b906114fc6114f633329085859192909192611449565b15610291565b61150b5761150991611527565b565b5f631b8e828b60e31b81528061152360048201610487565b0390fd5b9061153b6115356003610f98565b15610291565b6115705761155b61156e92611554611569935a9261157b565b5a90610fb9565b611563610ce5565b90610fde565b611cd7565b565b6115799161157b565b565b90611587903392610ef6565b906115c76115b57f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611057565b926115be6101d2565b918291826103c5565b0390a2565b906115d6916114e0565b565b5f90565b6115e690516101ef565b90565b6115fd6115f861160292610e6a565b610644565b6101ef565b90565b61160d6115d8565b50611616611b03565b6116215f82016115dc565b61163361162d5f6115e9565b916101ef565b14611688576116465f61165492016115dc565b61164e610780565b90610fde565b42611667611661836101ef565b916101ef565b101561167b57611678904290610fb9565b90565b506116855f6115e9565b90565b506116925f6115e9565b90565b61169d6115d8565b506116b160606116ab611b03565b016115dc565b90565b5f90565b60018060a01b031690565b6116cf6116d491610be9565b6116b8565b90565b6116e190546116c3565b90565b6116ec6116b4565b506116f65f6116d7565b90565b90565b61171061170b611715926116f9565b610644565b6101ef565b90565b6117206115d8565b5061173461172e6003610e42565b15610291565b611758576117556117456002610c02565b61174f60016116fc565b90610fde565b90565b6117615f6115e9565b90565b61176c6115d8565b50611780604061177a611b03565b016115dc565b90565b906117976117916003610f98565b15610291565b6117cc576117b76117ca926117b06117c5935a92611871565b5a90610fb9565b6117bf610ce5565b90610fde565b611cd7565b565b6117d591611871565b565b5090565b60016117e791016101ef565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561184c570180359067ffffffffffffffff82116118475760200191600182023603831361184257565b611806565b611802565b6117fe565b9082101561186c576020611868920281019061180a565b9091565b6117ea565b61187c8183906117d7565b916118856115d8565b5061188f5f6115e9565b5b806118a361189d866101ef565b916101ef565b1015611934576118d1906118c73332906118bf87878691611851565b929091611449565b6118d6575b6117db565b611890565b336118ec6118e686868591611851565b90610ef6565b9061192c61191a7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611057565b926119236101d2565b918291826103c5565b0390a26118cc565b50505050565b9061194491611783565b565b61195790611952611dde565b611959565b565b8061197461196e6119695f61114a565b6104cb565b916104cb565b146119ce5761198c611985826111b0565b60016111eb565b6119b67f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b991611057565b906119bf6101d2565b806119c981610487565b0390a2565b5f632e7f3c7f60e11b8152806119e660048201610487565b0390fd5b6119f390611946565b565b6119fd611dde565b611a05611a07565b565b611a0f611ef8565b565b611a196119f5565b565b611a2c90611a27611dde565b611a2e565b565b80611a49611a43611a3e5f61114a565b6104cb565b916104cb565b14611a5957611a5790611e99565b565b611a7c611a655f61114a565b5f918291631e4fbdf760e01b83526004830161097d565b0390fd5b611a8990611a1b565b565b611a956080610da7565b90565b634e487b7160e01b5f52601260045260245ffd5b611ab8611abe916101ef565b916101ef565b908115611ac9570490565b611a98565b611add611ae3919392936101ef565b926101ef565b91611aef8382026101ef565b928184041490151715611afe57565b610fa5565b611b0b610e0b565b50611b1f611b196003610e42565b15610291565b611c1b57611b40611b3b6004611b356002610c02565b90610bd3565b61131b565b42611b6e611b68611b63611b555f86016115dc565b611b5d610780565b90610fde565b6101ef565b916101ef565b1015611b775790565b611bc490611bbe611baf5f611ba8611b9a42611b948488016115dc565b90610fb9565b611ba2610780565b90611aac565b93016115dc565b91611bb8610780565b90611ace565b90610fde565b611c18611c0f5f611c0a611c015f611bfc611bf35f95611bee611be5611a8b565b9a5f8c016112a4565b6115e9565b602089016112a4565b6115e9565b604086016112a4565b6115e9565b606083016112a4565b90565b5f611c78611c6f5f611c6a611c615f611c5c611c535f95611c4e611c46611c40611a8b565b9b6115e9565b5f8c016112a4565b6115e9565b602089016112a4565b6115e9565b604086016112a4565b6115e9565b606083016112a4565b90565b90611c875f19916111bc565b9181191691161790565b90565b90611ca9611ca4611cb092610bb7565b611c91565b8254611c7b565b9055565b916020611cd5929493611cce60408201965f83019061070a565b019061070a565b565b611cea611ce46003610f98565b15610291565b611ddb57611d01611cfb6003610e42565b15610291565b611dce575b611d0e6120ce565b611d7f611d1c823a90611ace565b611d4f83611d496002611d396004611d3383610c02565b90610bd3565b0191611d4483610c02565b610fde565b90611c94565b611d796003611d696004611d636002610c02565b90610bd3565b0191611d7483610c02565b610fde565b90611c94565b611d896002610c02565b3a611db47f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610bb7565b92611dc9611dc06101d2565b92839283611cb4565b0390a2565b611dd6611fc3565b611d06565b50565b611de66116e4565b611dff611df9611df46122a6565b6104cb565b916104cb565b03611e0657565b611e28611e116122a6565b5f91829163118cdaa760e01b83526004830161097d565b0390fd5b60081b90565b90611e3f61ff0091611e2c565b9181191691161790565b90611e5e611e59611e6592611175565b611181565b8254611e32565b9055565b611e745f6003611e49565b565b90565b90611e8e611e89611e9592611057565b611e76565b82546111c1565b9055565b611ea25f6116d7565b611eac825f611e79565b90611ee0611eda7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093611057565b91611057565b91611ee96101d2565b80611ef381610487565b0390a3565b611f0460016003611e49565b565b90611f1260ff916111bc565b9181191691161790565b90611f31611f2c611f3892611175565b611181565b8254611f06565b9055565b90611f46906115e9565b5f5260205260405f2090565b90611faf60606003611fb594611f755f8201611f6f5f88016115dc565b90611c94565b611f8e60018201611f88602088016115dc565b90611c94565b611fa760028201611fa1604088016115dc565b90611c94565b0192016115dc565b90611c94565b565b90611fc191611f52565b565b611fd6611fd06003610e42565b15610291565b611fdd575b565b611fe960016003611f1c565b611ffc611ff55f6115e9565b6002611c94565b6120654261205461204b5f61204661203d5f61203861202f5f9561202a612021611a8b565b9a5f8c016112a4565b6115e9565b602089016112a4565b6115e9565b604086016112a4565b6115e9565b606083016112a4565b61206060045f90611f3c565b611fb7565b5f42906120a76120957f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e926115e9565b9261209e6101d2565b91829182610717565b0390a2611fdb565b90565b6120bb906101ef565b5f1981146120c95760010190565b610fa5565b6120eb6120e660046120e06002610c02565b90610bd3565b6120af565b4261211961211361210e6121005f8601610c02565b612108610780565b90610fde565b6101ef565b916101ef565b1015612123575b50565b61214b6121426121345f8401610c02565b61213c610780565b90610fde565b60018301611c94565b6121556002610c02565b61218261216460028401610c02565b9261217c5f61217560018401610c02565b9201610c02565b90610fb9565b6121ac7f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610bb7565b926121c16121b86101d2565b92839283611cb4565b0390a26121e06121d96121d46002610c02565b6120b2565b6002611c94565b6122524261223861222f5f61222a6122215f61221c6122135f9561220e612205611a8b565b9a5f8c016112a4565b6115e9565b602089016112a4565b6115e9565b604086016112a4565b6115e9565b606083016112a4565b61224d60046122476002610c02565b90610bd3565b611fb7565b61225c6002610c02565b429061229d61228b7f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610bb7565b926122946101d2565b91829182610717565b0390a25f612120565b6122ae6116b4565b50339056608060405234601c57600e6020565b61019461002b823961019490f35b6026565b60405190565b5f80fdfe60806040526004361015610013575b61017b565b61001d5f3561002c565b637a3979dc0361000e57610142565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b60018060a01b031690565b61005890610044565b90565b6100648161004f565b0361006b57565b5f80fd5b9050359061007c8261005b565b565b5f80fd5b5f80fd5b5f80fd5b909182601f830112156100c45781359167ffffffffffffffff83116100bf5760200192600183028401116100ba57565b610086565b610082565b61007e565b91606083830312610116576100e0825f850161006f565b926100ee836020830161006f565b92604082013567ffffffffffffffff81116101115761010d920161008a565b9091565b610040565b61003c565b151590565b6101299061011b565b9052565b9190610140905f60208501940190610120565b565b34610176576101726101616101583660046100c9565b92919091610183565b610169610032565b9182918261012d565b0390f35b610038565b5f80fd5b5f90565b5050505061018f61017f565b505f9056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4a\08Wa\0\x19a\0\x14a\0\xE9V[a\x01\xB7V[a\0!a\0=V[a\"\xB3a\x05\x16\x829`\x80Q\x81a\x0B-\x01Ra\"\xB3\x90\xF3[a\0CV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0o\x90a\0GV[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x87W`@RV[a\0QV[\x90a\0\x9Fa\0\x98a\0=V[\x92\x83a\0eV[V[_\x80\xFD[\x90V[a\0\xB1\x81a\0\xA5V[\x03a\0\xB8WV[_\x80\xFD[\x90PQ\x90a\0\xC9\x82a\0\xA8V[V[\x90` \x82\x82\x03\x12a\0\xE4Wa\0\xE1\x91_\x01a\0\xBCV[\x90V[a\0\xA1V[a\x01\x07a)\x88\x808\x03\x80a\0\xFC\x81a\0\x8CV[\x92\x839\x81\x01\x90a\0\xCBV[\x90V[\x90V[\x90V[a\x01$a\x01\x1Fa\x01)\x92a\x01\nV[a\x01\rV[a\0\xA5V[\x90V[` \x91\x81R\x01\x90V[_\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01i`\x18` \x92a\x01,V[a\x01r\x81a\x015V[\x01\x90V[a\x01\x8B\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x01\\V[\x90V[\x15a\x01\x95WV[a\x01\x9Da\0=V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x01\xB3`\x04\x82\x01a\x01vV[\x03\x90\xFD[a\x01\xBFa\x022V[a\x01\xDC\x81a\x01\xD5a\x01\xCF_a\x01\x10V[\x91a\0\xA5V[\x14\x15a\x01\x8EV[`\x80RV[`\x08\x1B\x90V[\x90a\x01\xF4a\xFF\0\x91a\x01\xE1V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x15\x15\x90V[a\x02\x0C\x90a\x01\xFEV[\x90V[\x90V[\x90a\x02'a\x02\"a\x02.\x92a\x02\x03V[a\x02\x0FV[\x82Ta\x01\xE7V[\x90UV[a\x02:a\x03*V[a\x02F`\x01`\x03a\x02\x12V[V[`\xA0\x1B\x90V[\x90a\x02]`\xFF`\xA0\x1B\x91a\x02HV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x02|a\x02wa\x02\x83\x92a\x02\x03V[a\x02\x0FV[\x82Ta\x02NV[\x90UV[_\x01\x90V[a\x02\x94a\0=V[=_\x82>=\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x02\xBBa\x02\xB6a\x02\xC0\x92a\x02\x9CV[a\x01\rV[a\x02\x9CV[\x90V[a\x02\xCC\x90a\x02\xA7V[\x90V[a\x02\xD8\x90a\x02\xC3V[\x90V[_\x1B\x90V[\x90a\x02\xF1`\x01\x80`\xA0\x1B\x03\x91a\x02\xDBV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x03\x04\x90a\x02\xC3V[\x90V[\x90V[\x90a\x03\x1Fa\x03\x1Aa\x03&\x92a\x02\xFBV[a\x03\x07V[\x82Ta\x02\xE0V[\x90UV[a\x0333a\x03\x97V[a\x03>_`\x01a\x02gV[a\x03Fa\0=V[a\x01\xBF\x81\x01\x81\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\x03\x92Wa\x03n\x82\x91a\x01\xBFa'\xC9\x849a\x02\x87V[\x03\x90_\xF0\x80\x15a\x03\x8DWa\x03\x84a\x03\x8B\x91a\x02\xCFV[`\x01a\x03\nV[V[a\x02\x8CV[a\0QV[a\x03\xA0\x90a\x03\xF8V[V[a\x03\xB6a\x03\xB1a\x03\xBB\x92a\x01\nV[a\x01\rV[a\x02\x9CV[\x90V[a\x03\xC7\x90a\x03\xA2V[\x90V[a\x03\xD3\x90a\x02\x9CV[\x90V[a\x03\xDF\x90a\x03\xCAV[\x90RV[\x91\x90a\x03\xF6\x90_` \x85\x01\x94\x01\x90a\x03\xD6V[V[\x80a\x04\x13a\x04\ra\x04\x08_a\x03\xBEV[a\x03\xCAV[\x91a\x03\xCAV[\x14a\x04#Wa\x04!\x90a\x04\xB6V[V[a\x04Fa\x04/_a\x03\xBEV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x03\xE3V[\x03\x90\xFD[_\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04fa\x04k\x91a\x04JV[a\x04OV[\x90V[a\x04x\x90Ta\x04ZV[\x90V[a\x04\x84\x90a\x02\xA7V[\x90V[a\x04\x90\x90a\x04{V[\x90V[\x90V[\x90a\x04\xABa\x04\xA6a\x04\xB2\x92a\x04\x87V[a\x04\x93V[\x82Ta\x02\xE0V[\x90UV[a\x04\xBF_a\x04nV[a\x04\xC9\x82_a\x04\x96V[\x90a\x04\xFDa\x04\xF7\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x04\x87V[\x91a\x04\x87V[\x91a\x05\x06a\0=V[\x80a\x05\x10\x81a\x02\x87V[\x03\x90\xA3V\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\rfV[a\0\x1D_5a\x01\xCCV[\x80c\x08aF\xD2\x14a\x01\xC7W\x80c\x18\xD5\xAA\xFE\x14a\x01\xC2W\x80c6l\xBA\xB7\x14a\x01\xBDW\x80c;j\xB2\xA9\x14a\x01\xB8W\x80cF\xE2\xCC\t\x14a\x01\xB3W\x80cH\\\xC9U\x14a\x01\xAEW\x80cK,\x07\x06\x14a\x01\xA9W\x80cTg\xCBH\x14a\x01\xA4W\x80c[<\xD6\xE2\x14a\x01\x9FW\x80caT8\x01\x14a\x01\x9AW\x80ceX\x95O\x14a\x01\x95W\x80cqP\x18\xA6\x14a\x01\x90W\x80cz9y\xDC\x14a\x01\x8BW\x80c\x80NQ#\x14a\x01\x86W\x80c\x82\xF4J\xDE\x14a\x01\x81W\x80c\x84\xFA\xB6+\x14a\x01|W\x80c\x8DZ#\x9B\x14a\x01wW\x80c\x8D\xA5\xCB[\x14a\x01rW\x80c\xAF\xF7Lm\x14a\x01mW\x80c\xC6`\xD3\xF3\x14a\x01hW\x80c\xCD\xAF\xB9x\x14a\x01cW\x80c\xD4\xF0\xEBM\x14a\x01^W\x80c\xD8x\x13B\x14a\x01YW\x80c\xDE\x1FE>\x14a\x01TW\x80c\xEAJ\x11\x04\x14a\x01OW\x80c\xED\xE0{\xD6\x14a\x01JWc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\r3V[a\x0C\xFEV[a\x0C\x8DV[a\x0B\x84V[a\x0BOV[a\n\xF8V[a\n\xA6V[a\t\xFCV[a\t\xC7V[a\t\x92V[a\t;V[a\t\x06V[a\x08\xC1V[a\x08\x8DV[a\x08TV[a\x07\xCFV[a\x07\x9AV[a\x07,V[a\x06\x9DV[a\x05\xD1V[a\x05\x9CV[a\x05'V[a\x04\x8CV[a\x04RV[a\x03\xDDV[a\x02\xB8V[a\x02\\V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01\xEAWV[a\x01\xDCV[\x90V[a\x01\xFB\x90a\x01\xEFV[\x90RV[\x90``\x80a\x02E\x93a\x02\x17_\x82\x01Q_\x86\x01\x90a\x01\xF2V[a\x02)` \x82\x01Q` \x86\x01\x90a\x01\xF2V[a\x02;`@\x82\x01Q`@\x86\x01\x90a\x01\xF2V[\x01Q\x91\x01\x90a\x01\xF2V[V[\x91\x90a\x02Z\x90_`\x80\x85\x01\x94\x01\x90a\x01\xFFV[V[4a\x02\x8CWa\x02l6`\x04a\x01\xE0V[a\x02\x88a\x02wa\x0E\x16V[a\x02\x7Fa\x01\xD2V[\x91\x82\x91\x82a\x02GV[\x03\x90\xF3[a\x01\xD8V[\x15\x15\x90V[a\x02\x9F\x90a\x02\x91V[\x90RV[\x91\x90a\x02\xB6\x90_` \x85\x01\x94\x01\x90a\x02\x96V[V[4a\x02\xE8Wa\x02\xC86`\x04a\x01\xE0V[a\x02\xE4a\x02\xD3a\x0EOV[a\x02\xDBa\x01\xD2V[\x91\x82\x91\x82a\x02\xA3V[\x03\x90\xF3[a\x01\xD8V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x037W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x032W` \x01\x92`\x01\x83\x02\x84\x01\x11a\x03-WV[a\x02\xF9V[a\x02\xF5V[a\x02\xF1V[\x90` \x82\x82\x03\x12a\x03mW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03hWa\x03d\x92\x01a\x02\xFDV[\x90\x91V[a\x02\xEDV[a\x01\xDCV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\xB3a\x03\xBC` \x93a\x03\xC1\x93a\x03\xAA\x81a\x03rV[\x93\x84\x80\x93a\x03vV[\x95\x86\x91\x01a\x03\x7FV[a\x03\x8AV[\x01\x90V[a\x03\xDA\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\x94V[\x90V[4a\x04\x0EWa\x04\na\x03\xF9a\x03\xF36`\x04a\x03<V[\x90a\x0E\xF6V[a\x04\x01a\x01\xD2V[\x91\x82\x91\x82a\x03\xC5V[\x03\x90\xF3[a\x01\xD8V[\x1C\x90V[`\xFF\x16\x90V[a\x04-\x90`\x08a\x042\x93\x02a\x04\x13V[a\x04\x17V[\x90V[\x90a\x04@\x91Ta\x04\x1DV[\x90V[a\x04O`\x03_\x90a\x045V[\x90V[4a\x04\x82Wa\x04b6`\x04a\x01\xE0V[a\x04~a\x04ma\x04CV[a\x04ua\x01\xD2V[\x91\x82\x91\x82a\x02\xA3V[\x03\x90\xF3[a\x01\xD8V[_\x01\x90V[4a\x04\xBBWa\x04\xA5a\x04\x9F6`\x04a\x03<V[\x90a\x10\xE7V[a\x04\xADa\x01\xD2V[\x80a\x04\xB7\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\xD4\x90a\x04\xC0V[\x90V[a\x04\xE0\x81a\x04\xCBV[\x03a\x04\xE7WV[_\x80\xFD[\x90P5\x90a\x04\xF8\x82a\x04\xD7V[V[\x91\x90`@\x83\x82\x03\x12a\x05\"W\x80a\x05\x16a\x05\x1F\x92_\x86\x01a\x04\xEBV[\x93` \x01a\x04\xEBV[\x90V[a\x01\xDCV[4a\x05VWa\x05@a\x05:6`\x04a\x04\xFAV[\x90a\x12\x98V[a\x05Ha\x01\xD2V[\x80a\x05R\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[a\x05d\x81a\x01\xEFV[\x03a\x05kWV[_\x80\xFD[\x90P5\x90a\x05|\x82a\x05[V[V[\x90` \x82\x82\x03\x12a\x05\x97Wa\x05\x94\x91_\x01a\x05oV[\x90V[a\x01\xDCV[4a\x05\xCCWa\x05\xC8a\x05\xB7a\x05\xB26`\x04a\x05~V[a\x13'V[a\x05\xBFa\x01\xD2V[\x91\x82\x91\x82a\x02GV[\x03\x90\xF3[a\x01\xD8V[4a\x05\xFFWa\x05\xE16`\x04a\x01\xE0V[a\x05\xE9a\x13bV[a\x05\xF1a\x01\xD2V[\x80a\x05\xFB\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x06\x1F\x90`\x08a\x06$\x93\x02a\x04\x13V[a\x06\x04V[\x90V[\x90a\x062\x91Ta\x06\x0FV[\x90V[a\x06A`\x01_\x90a\x06'V[\x90V[\x90V[a\x06[a\x06Va\x06`\x92a\x04\xC0V[a\x06DV[a\x04\xC0V[\x90V[a\x06l\x90a\x06GV[\x90V[a\x06x\x90a\x06cV[\x90V[a\x06\x84\x90a\x06oV[\x90RV[\x91\x90a\x06\x9B\x90_` \x85\x01\x94\x01\x90a\x06{V[V[4a\x06\xCDWa\x06\xAD6`\x04a\x01\xE0V[a\x06\xC9a\x06\xB8a\x065V[a\x06\xC0a\x01\xD2V[\x91\x82\x91\x82a\x06\x88V[\x03\x90\xF3[a\x01\xD8V[\x90V[a\x06\xE5\x90`\x08a\x06\xEA\x93\x02a\x04\x13V[a\x06\xD2V[\x90V[\x90a\x06\xF8\x91Ta\x06\xD5V[\x90V[a\x07\x07`\x02_\x90a\x06\xEDV[\x90V[a\x07\x13\x90a\x01\xEFV[\x90RV[\x91\x90a\x07*\x90_` \x85\x01\x94\x01\x90a\x07\nV[V[4a\x07\\Wa\x07<6`\x04a\x01\xE0V[a\x07Xa\x07Ga\x06\xFBV[a\x07Oa\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[\x90V[a\x07xa\x07sa\x07}\x92a\x07aV[a\x06DV[a\x01\xEFV[\x90V[a\x07\x8Cb'\x8D\0a\x07dV[\x90V[a\x07\x97a\x07\x80V[\x90V[4a\x07\xCAWa\x07\xAA6`\x04a\x01\xE0V[a\x07\xC6a\x07\xB5a\x07\x8FV[a\x07\xBDa\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[4a\x07\xFDWa\x07\xDF6`\x04a\x01\xE0V[a\x07\xE7a\x13\x91V[a\x07\xEFa\x01\xD2V[\x80a\x07\xF9\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[\x91``\x83\x83\x03\x12a\x08OWa\x08\x19\x82_\x85\x01a\x04\xEBV[\x92a\x08'\x83` \x83\x01a\x04\xEBV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08JWa\x08F\x92\x01a\x02\xFDV[\x90\x91V[a\x02\xEDV[a\x01\xDCV[4a\x08\x88Wa\x08\x84a\x08sa\x08j6`\x04a\x08\x02V[\x92\x91\x90\x91a\x14IV[a\x08{a\x01\xD2V[\x91\x82\x91\x82a\x02\xA3V[\x03\x90\xF3[a\x01\xD8V[4a\x08\xBCWa\x08\xA6a\x08\xA06`\x04a\x03<V[\x90a\x15\xCCV[a\x08\xAEa\x01\xD2V[\x80a\x08\xB8\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[4a\x08\xF1Wa\x08\xD16`\x04a\x01\xE0V[a\x08\xEDa\x08\xDCa\x16\x05V[a\x08\xE4a\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[a\t\x03`\x03`\x01\x90a\x045V[\x90V[4a\t6Wa\t\x166`\x04a\x01\xE0V[a\t2a\t!a\x08\xF6V[a\t)a\x01\xD2V[\x91\x82\x91\x82a\x02\xA3V[\x03\x90\xF3[a\x01\xD8V[4a\tkWa\tK6`\x04a\x01\xE0V[a\tga\tVa\x16\x95V[a\t^a\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[a\ty\x90a\x04\xCBV[\x90RV[\x91\x90a\t\x90\x90_` \x85\x01\x94\x01\x90a\tpV[V[4a\t\xC2Wa\t\xA26`\x04a\x01\xE0V[a\t\xBEa\t\xADa\x16\xE4V[a\t\xB5a\x01\xD2V[\x91\x82\x91\x82a\t}V[\x03\x90\xF3[a\x01\xD8V[4a\t\xF7Wa\t\xD76`\x04a\x01\xE0V[a\t\xF3a\t\xE2a\x17\x18V[a\t\xEAa\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[4a\n,Wa\n\x0C6`\x04a\x01\xE0V[a\n(a\n\x17a\x17dV[a\n\x1Fa\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\nkW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\nfW` \x01\x92` \x83\x02\x84\x01\x11a\naWV[a\x02\xF9V[a\x02\xF5V[a\x02\xF1V[\x90` \x82\x82\x03\x12a\n\xA1W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\x9CWa\n\x98\x92\x01a\n1V[\x90\x91V[a\x02\xEDV[a\x01\xDCV[4a\n\xD5Wa\n\xBFa\n\xB96`\x04a\npV[\x90a\x19:V[a\n\xC7a\x01\xD2V[\x80a\n\xD1\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[\x90` \x82\x82\x03\x12a\n\xF3Wa\n\xF0\x91_\x01a\x04\xEBV[\x90V[a\x01\xDCV[4a\x0B&Wa\x0B\x10a\x0B\x0B6`\x04a\n\xDAV[a\x19\xEAV[a\x0B\x18a\x01\xD2V[\x80a\x0B\"\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B\x7FWa\x0B_6`\x04a\x01\xE0V[a\x0B{a\x0Bja\x0B+V[a\x0Bra\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[4a\x0B\xB2Wa\x0B\x946`\x04a\x01\xE0V[a\x0B\x9Ca\x1A\x11V[a\x0B\xA4a\x01\xD2V[\x80a\x0B\xAE\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[a\x0B\xCBa\x0B\xC6a\x0B\xD0\x92a\x01\xEFV[a\x06DV[a\x01\xEFV[\x90V[\x90a\x0B\xDD\x90a\x0B\xB7V[_R` R`@_ \x90V[_\x1C\x90V[a\x0B\xFAa\x0B\xFF\x91a\x0B\xE9V[a\x06\xD2V[\x90V[a\x0C\x0C\x90Ta\x0B\xEEV[\x90V[a\x0C\x1A\x90`\x04a\x0B\xD3V[\x90a\x0C&_\x83\x01a\x0C\x02V[\x91a\x0C3`\x01\x82\x01a\x0C\x02V[\x91a\x0CL`\x03a\x0CE`\x02\x85\x01a\x0C\x02V[\x93\x01a\x0C\x02V[\x90V[a\x0C\x84a\x0C\x8B\x94a\x0Cz``\x94\x98\x97\x95a\x0Cp`\x80\x86\x01\x9A_\x87\x01\x90a\x07\nV[` \x85\x01\x90a\x07\nV[`@\x83\x01\x90a\x07\nV[\x01\x90a\x07\nV[V[4a\x0C\xC1Wa\x0C\xBDa\x0C\xA8a\x0C\xA36`\x04a\x05~V[a\x0C\x0FV[\x90a\x0C\xB4\x94\x92\x94a\x01\xD2V[\x94\x85\x94\x85a\x0COV[\x03\x90\xF3[a\x01\xD8V[\x90V[a\x0C\xDDa\x0C\xD8a\x0C\xE2\x92a\x0C\xC6V[a\x06DV[a\x01\xEFV[\x90V[a\x0C\xF0a\x13\x88a\x0C\xC9V[\x90V[a\x0C\xFBa\x0C\xE5V[\x90V[4a\r.Wa\r\x0E6`\x04a\x01\xE0V[a\r*a\r\x19a\x0C\xF3V[a\r!a\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[4a\raWa\rKa\rF6`\x04a\n\xDAV[a\x1A\x80V[a\rSa\x01\xD2V[\x80a\r]\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\r\x88\x90a\x03\x8AV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r\xA2W`@RV[a\rjV[\x90a\r\xBAa\r\xB3a\x01\xD2V[\x92\x83a\r~V[V[a\r\xC6`\x80a\r\xA7V[\x90V[_\x90V[a\r\xD5a\r\xBCV[\x90` \x80\x80\x80\x85a\r\xE4a\r\xC9V[\x81R\x01a\r\xEFa\r\xC9V[\x81R\x01a\r\xFAa\r\xC9V[\x81R\x01a\x0E\x05a\r\xC9V[\x81RPPV[a\x0E\x13a\r\xCDV[\x90V[a\x0E\x1Ea\x0E\x0BV[Pa\x0E'a\x1B\x03V[\x90V[_\x90V[a\x0E:a\x0E?\x91a\x0B\xE9V[a\x04\x17V[\x90V[a\x0EL\x90Ta\x0E.V[\x90V[a\x0EWa\x0E*V[Pa\x0Eb`\x03a\x0EBV[\x90V[``\x90V[\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x0E\x90a\x0E\x8Ba\x0E\x95\x92a\x0EjV[a\x0EvV[a\x0EmV[\x90V[\x90V[a\x0E\xA7a\x0E\xAC\x91a\x0EmV[a\x0E\x98V[\x90RV[\x90P\x90V[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x0E\xD0\x81a\x0E\xD7\x93a\x0E\xB0V[\x80\x93a\x0E\xB5V[\x01\x90V[\x80a\x0E\xEC`\x01\x92a\x0E\xF3\x96\x94a\x0E\x9BV[\x01\x91a\x0E\xC0V[\x90V[a\x0F4\x90a\x0F\x02a\x0EeV[Pa\x0F%a\x0F\x0F_a\x0E|V[\x91\x93a\x0F\x19a\x01\xD2V[\x94\x85\x93` \x85\x01a\x0E\xDBV[` \x82\x01\x81\x03\x82R\x03\x82a\r~V[\x90V[\x90a\x0FSa\x0FM32\x90\x85\x85\x91\x92\x90\x91\x92a\x14IV[\x15a\x02\x91V[a\x0FbWa\x0F`\x91a\x10\x03V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x0Fz`\x04\x82\x01a\x04\x87V[\x03\x90\xFD[`\x08\x1C\x90V[a\x0F\x90a\x0F\x95\x91a\x0F~V[a\x04\x17V[\x90V[a\x0F\xA2\x90Ta\x0F\x84V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x0F\xC8a\x0F\xCE\x91\x93\x92\x93a\x01\xEFV[\x92a\x01\xEFV[\x82\x03\x91\x82\x11a\x0F\xD9WV[a\x0F\xA5V[a\x0F\xEDa\x0F\xF3\x91\x93\x92\x93a\x01\xEFV[\x92a\x01\xEFV[\x82\x01\x80\x92\x11a\x0F\xFEWV[a\x0F\xA5V[\x90a\x10\x17a\x10\x11`\x03a\x0F\x98V[\x15a\x02\x91V[a\x10LWa\x107a\x10J\x92a\x100a\x10E\x93Z\x92a\x10\xA0V[Z\x90a\x0F\xB9V[a\x10?a\x0C\xE5V[\x90a\x0F\xDEV[a\x1C\xD7V[V[a\x10U\x91a\x10\xA0V[V[a\x10`\x90a\x06cV[\x90V[\x91\x90a\x10}\x81a\x10v\x81a\x10\x82\x95a\x03vV[\x80\x95a\x0E\xB5V[a\x03\x8AV[\x01\x90V[\x90\x91a\x10\x9D\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x10cV[\x90V[3\x90\x91a\x10\xCD\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10WV[\x92a\x10\xE2a\x10\xD9a\x01\xD2V[\x92\x83\x92\x83a\x10\x86V[\x03\x90\xA2V[\x90a\x10\xF1\x91a\x0F7V[V[\x90a\x11\x05\x91a\x11\0a\x1D\xDEV[a\x12\x0BV[V[`\xA0\x1C\x90V[a\x11\x19a\x11\x1E\x91a\x11\x07V[a\x04\x17V[\x90V[a\x11+\x90Ta\x11\rV[\x90V[a\x11Ba\x11=a\x11G\x92a\x0EjV[a\x06DV[a\x04\xC0V[\x90V[a\x11S\x90a\x11.V[\x90V[`\xA0\x1B\x90V[\x90a\x11k`\xFF`\xA0\x1B\x91a\x11VV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x11~\x90a\x02\x91V[\x90V[\x90V[\x90a\x11\x99a\x11\x94a\x11\xA0\x92a\x11uV[a\x11\x81V[\x82Ta\x11\\V[\x90UV[a\x11\xAD\x90a\x06GV[\x90V[a\x11\xB9\x90a\x11\xA4V[\x90V[_\x1B\x90V[\x90a\x11\xD2`\x01\x80`\xA0\x1B\x03\x91a\x11\xBCV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x11\xE5\x90a\x11\xA4V[\x90V[\x90V[\x90a\x12\0a\x11\xFBa\x12\x07\x92a\x11\xDCV[a\x11\xE8V[\x82Ta\x11\xC1V[\x90UV[a\x12\x15`\x01a\x11!V[a\x12}W\x81a\x124a\x12.a\x12)_a\x11JV[a\x04\xCBV[\x91a\x04\xCBV[\x14a\x12aWa\x12Za\x12Sa\x12_\x93a\x12N`\x01\x80a\x11\x84V[a\x11\xB0V[`\x01a\x11\xEBV[a\x1A\x80V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x12y`\x04\x82\x01a\x04\x87V[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x12\x94`\x04\x82\x01a\x04\x87V[\x03\x90\xFD[\x90a\x12\xA2\x91a\x10\xF3V[V[\x90a\x12\xAE\x90a\x01\xEFV[\x90RV[\x90a\x13\x19a\x13\x10`\x03a\x12\xC3a\r\xBCV[\x94a\x12\xDAa\x12\xD2_\x83\x01a\x0C\x02V[_\x88\x01a\x12\xA4V[a\x12\xF2a\x12\xE9`\x01\x83\x01a\x0C\x02V[` \x88\x01a\x12\xA4V[a\x13\na\x13\x01`\x02\x83\x01a\x0C\x02V[`@\x88\x01a\x12\xA4V[\x01a\x0C\x02V[``\x84\x01a\x12\xA4V[V[a\x13$\x90a\x12\xB2V[\x90V[a\x13>a\x13C\x91a\x136a\x0E\x0BV[P`\x04a\x0B\xD3V[a\x13\x1BV[\x90V[a\x13Na\x1D\xDEV[a\x13Va\x13XV[V[a\x13`a\x1EiV[V[a\x13ja\x13FV[V[a\x13ta\x1D\xDEV[a\x13|a\x13~V[V[a\x13\x8Fa\x13\x8A_a\x11JV[a\x1E\x99V[V[a\x13\x99a\x13lV[V[a\x13\xA7a\x13\xAC\x91a\x0B\xE9V[a\x06\x04V[\x90V[a\x13\xB9\x90Ta\x13\x9BV[\x90V[`\xE0\x1B\x90V[a\x13\xCB\x81a\x02\x91V[\x03a\x13\xD2WV[_\x80\xFD[\x90PQ\x90a\x13\xE3\x82a\x13\xC2V[V[\x90` \x82\x82\x03\x12a\x13\xFEWa\x13\xFB\x91_\x01a\x13\xD6V[\x90V[a\x01\xDCV[a\x14)a\x146\x95\x93\x94\x92\x94a\x14\x1F``\x84\x01\x96_\x85\x01\x90a\tpV[` \x83\x01\x90a\tpV[`@\x81\x85\x03\x91\x01Ra\x10cV[\x90V[a\x14Aa\x01\xD2V[=_\x82>=\x90\xFD[\x92a\x14\x8C` \x93\x94a\x14Ya\x0E*V[Pa\x14\x97a\x14oa\x14j`\x01a\x13\xAFV[a\x06oV[\x93cz9y\xDC\x92\x95\x97a\x14\x80a\x01\xD2V[\x98\x89\x97\x88\x96\x87\x96a\x13\xBCV[\x86R`\x04\x86\x01a\x14\x03V[\x03\x91Z\xFA\x90\x81\x15a\x14\xDBW_\x91a\x14\xADW[P\x90V[a\x14\xCE\x91P` =\x81\x11a\x14\xD4W[a\x14\xC6\x81\x83a\r~V[\x81\x01\x90a\x13\xE5V[_a\x14\xA9V[P=a\x14\xBCV[a\x149V[\x90a\x14\xFCa\x14\xF632\x90\x85\x85\x91\x92\x90\x91\x92a\x14IV[\x15a\x02\x91V[a\x15\x0BWa\x15\t\x91a\x15'V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x15#`\x04\x82\x01a\x04\x87V[\x03\x90\xFD[\x90a\x15;a\x155`\x03a\x0F\x98V[\x15a\x02\x91V[a\x15pWa\x15[a\x15n\x92a\x15Ta\x15i\x93Z\x92a\x15{V[Z\x90a\x0F\xB9V[a\x15ca\x0C\xE5V[\x90a\x0F\xDEV[a\x1C\xD7V[V[a\x15y\x91a\x15{V[V[\x90a\x15\x87\x903\x92a\x0E\xF6V[\x90a\x15\xC7a\x15\xB5\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10WV[\x92a\x15\xBEa\x01\xD2V[\x91\x82\x91\x82a\x03\xC5V[\x03\x90\xA2V[\x90a\x15\xD6\x91a\x14\xE0V[V[_\x90V[a\x15\xE6\x90Qa\x01\xEFV[\x90V[a\x15\xFDa\x15\xF8a\x16\x02\x92a\x0EjV[a\x06DV[a\x01\xEFV[\x90V[a\x16\ra\x15\xD8V[Pa\x16\x16a\x1B\x03V[a\x16!_\x82\x01a\x15\xDCV[a\x163a\x16-_a\x15\xE9V[\x91a\x01\xEFV[\x14a\x16\x88Wa\x16F_a\x16T\x92\x01a\x15\xDCV[a\x16Na\x07\x80V[\x90a\x0F\xDEV[Ba\x16ga\x16a\x83a\x01\xEFV[\x91a\x01\xEFV[\x10\x15a\x16{Wa\x16x\x90B\x90a\x0F\xB9V[\x90V[Pa\x16\x85_a\x15\xE9V[\x90V[Pa\x16\x92_a\x15\xE9V[\x90V[a\x16\x9Da\x15\xD8V[Pa\x16\xB1``a\x16\xABa\x1B\x03V[\x01a\x15\xDCV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x16\xCFa\x16\xD4\x91a\x0B\xE9V[a\x16\xB8V[\x90V[a\x16\xE1\x90Ta\x16\xC3V[\x90V[a\x16\xECa\x16\xB4V[Pa\x16\xF6_a\x16\xD7V[\x90V[\x90V[a\x17\x10a\x17\x0Ba\x17\x15\x92a\x16\xF9V[a\x06DV[a\x01\xEFV[\x90V[a\x17 a\x15\xD8V[Pa\x174a\x17.`\x03a\x0EBV[\x15a\x02\x91V[a\x17XWa\x17Ua\x17E`\x02a\x0C\x02V[a\x17O`\x01a\x16\xFCV[\x90a\x0F\xDEV[\x90V[a\x17a_a\x15\xE9V[\x90V[a\x17la\x15\xD8V[Pa\x17\x80`@a\x17za\x1B\x03V[\x01a\x15\xDCV[\x90V[\x90a\x17\x97a\x17\x91`\x03a\x0F\x98V[\x15a\x02\x91V[a\x17\xCCWa\x17\xB7a\x17\xCA\x92a\x17\xB0a\x17\xC5\x93Z\x92a\x18qV[Z\x90a\x0F\xB9V[a\x17\xBFa\x0C\xE5V[\x90a\x0F\xDEV[a\x1C\xD7V[V[a\x17\xD5\x91a\x18qV[V[P\x90V[`\x01a\x17\xE7\x91\x01a\x01\xEFV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x18LW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x18GW` \x01\x91`\x01\x82\x026\x03\x83\x13a\x18BWV[a\x18\x06V[a\x18\x02V[a\x17\xFEV[\x90\x82\x10\x15a\x18lW` a\x18h\x92\x02\x81\x01\x90a\x18\nV[\x90\x91V[a\x17\xEAV[a\x18|\x81\x83\x90a\x17\xD7V[\x91a\x18\x85a\x15\xD8V[Pa\x18\x8F_a\x15\xE9V[[\x80a\x18\xA3a\x18\x9D\x86a\x01\xEFV[\x91a\x01\xEFV[\x10\x15a\x194Wa\x18\xD1\x90a\x18\xC732\x90a\x18\xBF\x87\x87\x86\x91a\x18QV[\x92\x90\x91a\x14IV[a\x18\xD6W[a\x17\xDBV[a\x18\x90V[3a\x18\xECa\x18\xE6\x86\x86\x85\x91a\x18QV[\x90a\x0E\xF6V[\x90a\x19,a\x19\x1A\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10WV[\x92a\x19#a\x01\xD2V[\x91\x82\x91\x82a\x03\xC5V[\x03\x90\xA2a\x18\xCCV[PPPPV[\x90a\x19D\x91a\x17\x83V[V[a\x19W\x90a\x19Ra\x1D\xDEV[a\x19YV[V[\x80a\x19ta\x19na\x19i_a\x11JV[a\x04\xCBV[\x91a\x04\xCBV[\x14a\x19\xCEWa\x19\x8Ca\x19\x85\x82a\x11\xB0V[`\x01a\x11\xEBV[a\x19\xB6\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10WV[\x90a\x19\xBFa\x01\xD2V[\x80a\x19\xC9\x81a\x04\x87V[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x19\xE6`\x04\x82\x01a\x04\x87V[\x03\x90\xFD[a\x19\xF3\x90a\x19FV[V[a\x19\xFDa\x1D\xDEV[a\x1A\x05a\x1A\x07V[V[a\x1A\x0Fa\x1E\xF8V[V[a\x1A\x19a\x19\xF5V[V[a\x1A,\x90a\x1A'a\x1D\xDEV[a\x1A.V[V[\x80a\x1AIa\x1ACa\x1A>_a\x11JV[a\x04\xCBV[\x91a\x04\xCBV[\x14a\x1AYWa\x1AW\x90a\x1E\x99V[V[a\x1A|a\x1Ae_a\x11JV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\t}V[\x03\x90\xFD[a\x1A\x89\x90a\x1A\x1BV[V[a\x1A\x95`\x80a\r\xA7V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\x1A\xB8a\x1A\xBE\x91a\x01\xEFV[\x91a\x01\xEFV[\x90\x81\x15a\x1A\xC9W\x04\x90V[a\x1A\x98V[a\x1A\xDDa\x1A\xE3\x91\x93\x92\x93a\x01\xEFV[\x92a\x01\xEFV[\x91a\x1A\xEF\x83\x82\x02a\x01\xEFV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1A\xFEWV[a\x0F\xA5V[a\x1B\x0Ba\x0E\x0BV[Pa\x1B\x1Fa\x1B\x19`\x03a\x0EBV[\x15a\x02\x91V[a\x1C\x1BWa\x1B@a\x1B;`\x04a\x1B5`\x02a\x0C\x02V[\x90a\x0B\xD3V[a\x13\x1BV[Ba\x1Bna\x1Bha\x1Bca\x1BU_\x86\x01a\x15\xDCV[a\x1B]a\x07\x80V[\x90a\x0F\xDEV[a\x01\xEFV[\x91a\x01\xEFV[\x10\x15a\x1BwW\x90V[a\x1B\xC4\x90a\x1B\xBEa\x1B\xAF_a\x1B\xA8a\x1B\x9ABa\x1B\x94\x84\x88\x01a\x15\xDCV[\x90a\x0F\xB9V[a\x1B\xA2a\x07\x80V[\x90a\x1A\xACV[\x93\x01a\x15\xDCV[\x91a\x1B\xB8a\x07\x80V[\x90a\x1A\xCEV[\x90a\x0F\xDEV[a\x1C\x18a\x1C\x0F_a\x1C\na\x1C\x01_a\x1B\xFCa\x1B\xF3_\x95a\x1B\xEEa\x1B\xE5a\x1A\x8BV[\x9A_\x8C\x01a\x12\xA4V[a\x15\xE9V[` \x89\x01a\x12\xA4V[a\x15\xE9V[`@\x86\x01a\x12\xA4V[a\x15\xE9V[``\x83\x01a\x12\xA4V[\x90V[_a\x1Cxa\x1Co_a\x1Cja\x1Ca_a\x1C\\a\x1CS_\x95a\x1CNa\x1CFa\x1C@a\x1A\x8BV[\x9Ba\x15\xE9V[_\x8C\x01a\x12\xA4V[a\x15\xE9V[` \x89\x01a\x12\xA4V[a\x15\xE9V[`@\x86\x01a\x12\xA4V[a\x15\xE9V[``\x83\x01a\x12\xA4V[\x90V[\x90a\x1C\x87_\x19\x91a\x11\xBCV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x1C\xA9a\x1C\xA4a\x1C\xB0\x92a\x0B\xB7V[a\x1C\x91V[\x82Ta\x1C{V[\x90UV[\x91` a\x1C\xD5\x92\x94\x93a\x1C\xCE`@\x82\x01\x96_\x83\x01\x90a\x07\nV[\x01\x90a\x07\nV[V[a\x1C\xEAa\x1C\xE4`\x03a\x0F\x98V[\x15a\x02\x91V[a\x1D\xDBWa\x1D\x01a\x1C\xFB`\x03a\x0EBV[\x15a\x02\x91V[a\x1D\xCEW[a\x1D\x0Ea \xCEV[a\x1D\x7Fa\x1D\x1C\x82:\x90a\x1A\xCEV[a\x1DO\x83a\x1DI`\x02a\x1D9`\x04a\x1D3\x83a\x0C\x02V[\x90a\x0B\xD3V[\x01\x91a\x1DD\x83a\x0C\x02V[a\x0F\xDEV[\x90a\x1C\x94V[a\x1Dy`\x03a\x1Di`\x04a\x1Dc`\x02a\x0C\x02V[\x90a\x0B\xD3V[\x01\x91a\x1Dt\x83a\x0C\x02V[a\x0F\xDEV[\x90a\x1C\x94V[a\x1D\x89`\x02a\x0C\x02V[:a\x1D\xB4\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0B\xB7V[\x92a\x1D\xC9a\x1D\xC0a\x01\xD2V[\x92\x83\x92\x83a\x1C\xB4V[\x03\x90\xA2V[a\x1D\xD6a\x1F\xC3V[a\x1D\x06V[PV[a\x1D\xE6a\x16\xE4V[a\x1D\xFFa\x1D\xF9a\x1D\xF4a\"\xA6V[a\x04\xCBV[\x91a\x04\xCBV[\x03a\x1E\x06WV[a\x1E(a\x1E\x11a\"\xA6V[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\t}V[\x03\x90\xFD[`\x08\x1B\x90V[\x90a\x1E?a\xFF\0\x91a\x1E,V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1E^a\x1EYa\x1Ee\x92a\x11uV[a\x11\x81V[\x82Ta\x1E2V[\x90UV[a\x1Et_`\x03a\x1EIV[V[\x90V[\x90a\x1E\x8Ea\x1E\x89a\x1E\x95\x92a\x10WV[a\x1EvV[\x82Ta\x11\xC1V[\x90UV[a\x1E\xA2_a\x16\xD7V[a\x1E\xAC\x82_a\x1EyV[\x90a\x1E\xE0a\x1E\xDA\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10WV[\x91a\x10WV[\x91a\x1E\xE9a\x01\xD2V[\x80a\x1E\xF3\x81a\x04\x87V[\x03\x90\xA3V[a\x1F\x04`\x01`\x03a\x1EIV[V[\x90a\x1F\x12`\xFF\x91a\x11\xBCV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1F1a\x1F,a\x1F8\x92a\x11uV[a\x11\x81V[\x82Ta\x1F\x06V[\x90UV[\x90a\x1FF\x90a\x15\xE9V[_R` R`@_ \x90V[\x90a\x1F\xAF```\x03a\x1F\xB5\x94a\x1Fu_\x82\x01a\x1Fo_\x88\x01a\x15\xDCV[\x90a\x1C\x94V[a\x1F\x8E`\x01\x82\x01a\x1F\x88` \x88\x01a\x15\xDCV[\x90a\x1C\x94V[a\x1F\xA7`\x02\x82\x01a\x1F\xA1`@\x88\x01a\x15\xDCV[\x90a\x1C\x94V[\x01\x92\x01a\x15\xDCV[\x90a\x1C\x94V[V[\x90a\x1F\xC1\x91a\x1FRV[V[a\x1F\xD6a\x1F\xD0`\x03a\x0EBV[\x15a\x02\x91V[a\x1F\xDDW[V[a\x1F\xE9`\x01`\x03a\x1F\x1CV[a\x1F\xFCa\x1F\xF5_a\x15\xE9V[`\x02a\x1C\x94V[a eBa Ta K_a Fa =_a 8a /_\x95a *a !a\x1A\x8BV[\x9A_\x8C\x01a\x12\xA4V[a\x15\xE9V[` \x89\x01a\x12\xA4V[a\x15\xE9V[`@\x86\x01a\x12\xA4V[a\x15\xE9V[``\x83\x01a\x12\xA4V[a ``\x04_\x90a\x1F<V[a\x1F\xB7V[_B\x90a \xA7a \x95\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x15\xE9V[\x92a \x9Ea\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xA2a\x1F\xDBV[\x90V[a \xBB\x90a\x01\xEFV[_\x19\x81\x14a \xC9W`\x01\x01\x90V[a\x0F\xA5V[a \xEBa \xE6`\x04a \xE0`\x02a\x0C\x02V[\x90a\x0B\xD3V[a \xAFV[Ba!\x19a!\x13a!\x0Ea!\0_\x86\x01a\x0C\x02V[a!\x08a\x07\x80V[\x90a\x0F\xDEV[a\x01\xEFV[\x91a\x01\xEFV[\x10\x15a!#W[PV[a!Ka!Ba!4_\x84\x01a\x0C\x02V[a!<a\x07\x80V[\x90a\x0F\xDEV[`\x01\x83\x01a\x1C\x94V[a!U`\x02a\x0C\x02V[a!\x82a!d`\x02\x84\x01a\x0C\x02V[\x92a!|_a!u`\x01\x84\x01a\x0C\x02V[\x92\x01a\x0C\x02V[\x90a\x0F\xB9V[a!\xAC\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0B\xB7V[\x92a!\xC1a!\xB8a\x01\xD2V[\x92\x83\x92\x83a\x1C\xB4V[\x03\x90\xA2a!\xE0a!\xD9a!\xD4`\x02a\x0C\x02V[a \xB2V[`\x02a\x1C\x94V[a\"RBa\"8a\"/_a\"*a\"!_a\"\x1Ca\"\x13_\x95a\"\x0Ea\"\x05a\x1A\x8BV[\x9A_\x8C\x01a\x12\xA4V[a\x15\xE9V[` \x89\x01a\x12\xA4V[a\x15\xE9V[`@\x86\x01a\x12\xA4V[a\x15\xE9V[``\x83\x01a\x12\xA4V[a\"M`\x04a\"G`\x02a\x0C\x02V[\x90a\x0B\xD3V[a\x1F\xB7V[a\"\\`\x02a\x0C\x02V[B\x90a\"\x9Da\"\x8B\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0B\xB7V[\x92a\"\x94a\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xA2_a! V[a\"\xAEa\x16\xB4V[P3\x90V`\x80`@R4`\x1CW`\x0E` V[a\x01\x94a\0+\x829a\x01\x94\x90\xF3[`&V[`@Q\x90V[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x01{V[a\0\x1D_5a\0,V[cz9y\xDC\x03a\0\x0EWa\x01BV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\0X\x90a\0DV[\x90V[a\0d\x81a\0OV[\x03a\0kWV[_\x80\xFD[\x90P5\x90a\0|\x82a\0[V[V[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\0\xC4W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xBFW` \x01\x92`\x01\x83\x02\x84\x01\x11a\0\xBAWV[a\0\x86V[a\0\x82V[a\0~V[\x91``\x83\x83\x03\x12a\x01\x16Wa\0\xE0\x82_\x85\x01a\0oV[\x92a\0\xEE\x83` \x83\x01a\0oV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x11Wa\x01\r\x92\x01a\0\x8AV[\x90\x91V[a\0@V[a\0<V[\x15\x15\x90V[a\x01)\x90a\x01\x1BV[\x90RV[\x91\x90a\x01@\x90_` \x85\x01\x94\x01\x90a\x01 V[V[4a\x01vWa\x01ra\x01aa\x01X6`\x04a\0\xC9V[\x92\x91\x90\x91a\x01\x83V[a\x01ia\x002V[\x91\x82\x91\x82a\x01-V[\x03\x90\xF3[a\08V[_\x80\xFD[_\x90V[PPPPa\x01\x8Fa\x01\x7FV[P_\x90V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b610d66565b61001d5f356101cc565b8063086146d2146101c757806318d5aafe146101c2578063366cbab7146101bd5780633b6ab2a9146101b857806346e2cc09146101b3578063485cc955146101ae5780634b2c0706146101a95780635467cb48146101a45780635b3cd6e21461019f578063615438011461019a5780636558954f14610195578063715018a6146101905780637a3979dc1461018b578063804e51231461018657806382f44ade1461018157806384fab62b1461017c5780638d5a239b146101775780638da5cb5b14610172578063aff74c6d1461016d578063c660d3f314610168578063cdafb97814610163578063d4f0eb4d1461015e578063d878134214610159578063de1f453e14610154578063ea4a11041461014f578063ede07bd61461014a5763f2fde38b0361000e57610d33565b610cfe565b610c8d565b610b84565b610b4f565b610af8565b610aa6565b6109fc565b6109c7565b610992565b61093b565b610906565b6108c1565b61088d565b610854565b6107cf565b61079a565b61072c565b61069d565b6105d1565b61059c565b610527565b61048c565b610452565b6103dd565b6102b8565b61025c565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f9103126101ea57565b6101dc565b90565b6101fb906101ef565b9052565b90606080610245936102175f8201515f8601906101f2565b610229602082015160208601906101f2565b61023b604082015160408601906101f2565b01519101906101f2565b565b919061025a905f608085019401906101ff565b565b3461028c5761026c3660046101e0565b610288610277610e16565b61027f6101d2565b91829182610247565b0390f35b6101d8565b151590565b61029f90610291565b9052565b91906102b6905f60208501940190610296565b565b346102e8576102c83660046101e0565b6102e46102d3610e4f565b6102db6101d2565b918291826102a3565b0390f35b6101d8565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156103375781359167ffffffffffffffff831161033257602001926001830284011161032d57565b6102f9565b6102f5565b6102f1565b9060208282031261036d575f82013567ffffffffffffffff81116103685761036492016102fd565b9091565b6102ed565b6101dc565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6103b36103bc6020936103c1936103aa81610372565b93848093610376565b9586910161037f565b61038a565b0190565b6103da9160208201915f818403910152610394565b90565b3461040e5761040a6103f96103f336600461033c565b90610ef6565b6104016101d2565b918291826103c5565b0390f35b6101d8565b1c90565b60ff1690565b61042d9060086104329302610413565b610417565b90565b90610440915461041d565b90565b61044f60035f90610435565b90565b34610482576104623660046101e0565b61047e61046d610443565b6104756101d2565b918291826102a3565b0390f35b6101d8565b5f0190565b346104bb576104a561049f36600461033c565b906110e7565b6104ad6101d2565b806104b781610487565b0390f35b6101d8565b60018060a01b031690565b6104d4906104c0565b90565b6104e0816104cb565b036104e757565b5f80fd5b905035906104f8826104d7565b565b9190604083820312610522578061051661051f925f86016104eb565b936020016104eb565b90565b6101dc565b346105565761054061053a3660046104fa565b90611298565b6105486101d2565b8061055281610487565b0390f35b6101d8565b610564816101ef565b0361056b57565b5f80fd5b9050359061057c8261055b565b565b9060208282031261059757610594915f0161056f565b90565b6101dc565b346105cc576105c86105b76105b236600461057e565b611327565b6105bf6101d2565b91829182610247565b0390f35b6101d8565b346105ff576105e13660046101e0565b6105e9611362565b6105f16101d2565b806105fb81610487565b0390f35b6101d8565b60018060a01b031690565b61061f9060086106249302610413565b610604565b90565b90610632915461060f565b90565b61064160015f90610627565b90565b90565b61065b610656610660926104c0565b610644565b6104c0565b90565b61066c90610647565b90565b61067890610663565b90565b6106849061066f565b9052565b919061069b905f6020850194019061067b565b565b346106cd576106ad3660046101e0565b6106c96106b8610635565b6106c06101d2565b91829182610688565b0390f35b6101d8565b90565b6106e59060086106ea9302610413565b6106d2565b90565b906106f891546106d5565b90565b61070760025f906106ed565b90565b610713906101ef565b9052565b919061072a905f6020850194019061070a565b565b3461075c5761073c3660046101e0565b6107586107476106fb565b61074f6101d2565b91829182610717565b0390f35b6101d8565b90565b61077861077361077d92610761565b610644565b6101ef565b90565b61078c62278d00610764565b90565b610797610780565b90565b346107ca576107aa3660046101e0565b6107c66107b561078f565b6107bd6101d2565b91829182610717565b0390f35b6101d8565b346107fd576107df3660046101e0565b6107e7611391565b6107ef6101d2565b806107f981610487565b0390f35b6101d8565b9160608383031261084f57610819825f85016104eb565b9261082783602083016104eb565b92604082013567ffffffffffffffff811161084a5761084692016102fd565b9091565b6102ed565b6101dc565b346108885761088461087361086a366004610802565b92919091611449565b61087b6101d2565b918291826102a3565b0390f35b6101d8565b346108bc576108a66108a036600461033c565b906115cc565b6108ae6101d2565b806108b881610487565b0390f35b6101d8565b346108f1576108d13660046101e0565b6108ed6108dc611605565b6108e46101d2565b91829182610717565b0390f35b6101d8565b6109036003600190610435565b90565b34610936576109163660046101e0565b6109326109216108f6565b6109296101d2565b918291826102a3565b0390f35b6101d8565b3461096b5761094b3660046101e0565b610967610956611695565b61095e6101d2565b91829182610717565b0390f35b6101d8565b610979906104cb565b9052565b9190610990905f60208501940190610970565b565b346109c2576109a23660046101e0565b6109be6109ad6116e4565b6109b56101d2565b9182918261097d565b0390f35b6101d8565b346109f7576109d73660046101e0565b6109f36109e2611718565b6109ea6101d2565b91829182610717565b0390f35b6101d8565b34610a2c57610a0c3660046101e0565b610a28610a17611764565b610a1f6101d2565b91829182610717565b0390f35b6101d8565b909182601f83011215610a6b5781359167ffffffffffffffff8311610a66576020019260208302840111610a6157565b6102f9565b6102f5565b6102f1565b90602082820312610aa1575f82013567ffffffffffffffff8111610a9c57610a989201610a31565b9091565b6102ed565b6101dc565b34610ad557610abf610ab9366004610a70565b9061193a565b610ac76101d2565b80610ad181610487565b0390f35b6101d8565b90602082820312610af357610af0915f016104eb565b90565b6101dc565b34610b2657610b10610b0b366004610ada565b6119ea565b610b186101d2565b80610b2281610487565b0390f35b6101d8565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610b7f57610b5f3660046101e0565b610b7b610b6a610b2b565b610b726101d2565b91829182610717565b0390f35b6101d8565b34610bb257610b943660046101e0565b610b9c611a11565b610ba46101d2565b80610bae81610487565b0390f35b6101d8565b610bcb610bc6610bd0926101ef565b610644565b6101ef565b90565b90610bdd90610bb7565b5f5260205260405f2090565b5f1c90565b610bfa610bff91610be9565b6106d2565b90565b610c0c9054610bee565b90565b610c1a906004610bd3565b90610c265f8301610c02565b91610c3360018201610c02565b91610c4c6003610c4560028501610c02565b9301610c02565b90565b610c84610c8b94610c7a606094989795610c70608086019a5f87019061070a565b602085019061070a565b604083019061070a565b019061070a565b565b34610cc157610cbd610ca8610ca336600461057e565b610c0f565b90610cb49492946101d2565b94859485610c4f565b0390f35b6101d8565b90565b610cdd610cd8610ce292610cc6565b610644565b6101ef565b90565b610cf0611388610cc9565b90565b610cfb610ce5565b90565b34610d2e57610d0e3660046101e0565b610d2a610d19610cf3565b610d216101d2565b91829182610717565b0390f35b6101d8565b34610d6157610d4b610d46366004610ada565b611a80565b610d536101d2565b80610d5d81610487565b0390f35b6101d8565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b90610d889061038a565b810190811067ffffffffffffffff821117610da257604052565b610d6a565b90610dba610db36101d2565b9283610d7e565b565b610dc66080610da7565b90565b5f90565b610dd5610dbc565b90602080808085610de4610dc9565b815201610def610dc9565b815201610dfa610dc9565b815201610e05610dc9565b81525050565b610e13610dcd565b90565b610e1e610e0b565b50610e27611b03565b90565b5f90565b610e3a610e3f91610be9565b610417565b90565b610e4c9054610e2e565b90565b610e57610e2a565b50610e626003610e42565b90565b606090565b90565b60ff60f81b1690565b60f81b90565b610e90610e8b610e9592610e6a565b610e76565b610e6d565b90565b90565b610ea7610eac91610e6d565b610e98565b9052565b905090565b90825f939282370152565b909182610ed081610ed793610eb0565b8093610eb5565b0190565b80610eec600192610ef39694610e9b565b0191610ec0565b90565b610f3490610f02610e65565b50610f25610f0f5f610e7c565b9193610f196101d2565b94859360208501610edb565b60208201810382520382610d7e565b90565b90610f53610f4d33329085859192909192611449565b15610291565b610f6257610f6091611003565b565b5f631b8e828b60e31b815280610f7a60048201610487565b0390fd5b60081c90565b610f90610f9591610f7e565b610417565b90565b610fa29054610f84565b90565b634e487b7160e01b5f52601160045260245ffd5b610fc8610fce919392936101ef565b926101ef565b8203918211610fd957565b610fa5565b610fed610ff3919392936101ef565b926101ef565b8201809211610ffe57565b610fa5565b906110176110116003610f98565b15610291565b61104c5761103761104a92611030611045935a926110a0565b5a90610fb9565b61103f610ce5565b90610fde565b611cd7565b565b611055916110a0565b565b61106090610663565b90565b919061107d816110768161108295610376565b8095610eb5565b61038a565b0190565b909161109d9260208301925f818503910152611063565b90565b3390916110cd7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611057565b926110e26110d96101d2565b92839283611086565b0390a2565b906110f191610f37565b565b9061110591611100611dde565b61120b565b565b60a01c90565b61111961111e91611107565b610417565b90565b61112b905461110d565b90565b61114261113d61114792610e6a565b610644565b6104c0565b90565b6111539061112e565b90565b60a01b90565b9061116b60ff60a01b91611156565b9181191691161790565b61117e90610291565b90565b90565b906111996111946111a092611175565b611181565b825461115c565b9055565b6111ad90610647565b90565b6111b9906111a4565b90565b5f1b90565b906111d260018060a01b03916111bc565b9181191691161790565b6111e5906111a4565b90565b90565b906112006111fb611207926111dc565b6111e8565b82546111c1565b9055565b6112156001611121565b61127d578161123461122e6112295f61114a565b6104cb565b916104cb565b146112615761125a61125361125f9361124e600180611184565b6111b0565b60016111eb565b611a80565b565b5f632e7f3c7f60e11b81528061127960048201610487565b0390fd5b5f62dc149f60e41b81528061129460048201610487565b0390fd5b906112a2916110f3565b565b906112ae906101ef565b9052565b9061131961131060036112c3610dbc565b946112da6112d25f8301610c02565b5f88016112a4565b6112f26112e960018301610c02565b602088016112a4565b61130a61130160028301610c02565b604088016112a4565b01610c02565b606084016112a4565b565b611324906112b2565b90565b61133e61134391611336610e0b565b506004610bd3565b61131b565b90565b61134e611dde565b611356611358565b565b611360611e69565b565b61136a611346565b565b611374611dde565b61137c61137e565b565b61138f61138a5f61114a565b611e99565b565b61139961136c565b565b6113a76113ac91610be9565b610604565b90565b6113b9905461139b565b90565b60e01b90565b6113cb81610291565b036113d257565b5f80fd5b905051906113e3826113c2565b565b906020828203126113fe576113fb915f016113d6565b90565b6101dc565b611429611436959394929461141f60608401965f850190610970565b6020830190610970565b6040818503910152611063565b90565b6114416101d2565b3d5f823e3d90fd5b9261148c60209394611459610e2a565b5061149761146f61146a60016113af565b61066f565b93637a3979dc9295976114806101d2565b988997889687966113bc565b865260048601611403565b03915afa9081156114db575f916114ad575b5090565b6114ce915060203d81116114d4575b6114c68183610d7e565b8101906113e5565b5f6114a9565b503d6114bc565b611439565b906114fc6114f633329085859192909192611449565b15610291565b61150b5761150991611527565b565b5f631b8e828b60e31b81528061152360048201610487565b0390fd5b9061153b6115356003610f98565b15610291565b6115705761155b61156e92611554611569935a9261157b565b5a90610fb9565b611563610ce5565b90610fde565b611cd7565b565b6115799161157b565b565b90611587903392610ef6565b906115c76115b57f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611057565b926115be6101d2565b918291826103c5565b0390a2565b906115d6916114e0565b565b5f90565b6115e690516101ef565b90565b6115fd6115f861160292610e6a565b610644565b6101ef565b90565b61160d6115d8565b50611616611b03565b6116215f82016115dc565b61163361162d5f6115e9565b916101ef565b14611688576116465f61165492016115dc565b61164e610780565b90610fde565b42611667611661836101ef565b916101ef565b101561167b57611678904290610fb9565b90565b506116855f6115e9565b90565b506116925f6115e9565b90565b61169d6115d8565b506116b160606116ab611b03565b016115dc565b90565b5f90565b60018060a01b031690565b6116cf6116d491610be9565b6116b8565b90565b6116e190546116c3565b90565b6116ec6116b4565b506116f65f6116d7565b90565b90565b61171061170b611715926116f9565b610644565b6101ef565b90565b6117206115d8565b5061173461172e6003610e42565b15610291565b611758576117556117456002610c02565b61174f60016116fc565b90610fde565b90565b6117615f6115e9565b90565b61176c6115d8565b50611780604061177a611b03565b016115dc565b90565b906117976117916003610f98565b15610291565b6117cc576117b76117ca926117b06117c5935a92611871565b5a90610fb9565b6117bf610ce5565b90610fde565b611cd7565b565b6117d591611871565b565b5090565b60016117e791016101ef565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561184c570180359067ffffffffffffffff82116118475760200191600182023603831361184257565b611806565b611802565b6117fe565b9082101561186c576020611868920281019061180a565b9091565b6117ea565b61187c8183906117d7565b916118856115d8565b5061188f5f6115e9565b5b806118a361189d866101ef565b916101ef565b1015611934576118d1906118c73332906118bf87878691611851565b929091611449565b6118d6575b6117db565b611890565b336118ec6118e686868591611851565b90610ef6565b9061192c61191a7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f92611057565b926119236101d2565b918291826103c5565b0390a26118cc565b50505050565b9061194491611783565b565b61195790611952611dde565b611959565b565b8061197461196e6119695f61114a565b6104cb565b916104cb565b146119ce5761198c611985826111b0565b60016111eb565b6119b67f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b991611057565b906119bf6101d2565b806119c981610487565b0390a2565b5f632e7f3c7f60e11b8152806119e660048201610487565b0390fd5b6119f390611946565b565b6119fd611dde565b611a05611a07565b565b611a0f611ef8565b565b611a196119f5565b565b611a2c90611a27611dde565b611a2e565b565b80611a49611a43611a3e5f61114a565b6104cb565b916104cb565b14611a5957611a5790611e99565b565b611a7c611a655f61114a565b5f918291631e4fbdf760e01b83526004830161097d565b0390fd5b611a8990611a1b565b565b611a956080610da7565b90565b634e487b7160e01b5f52601260045260245ffd5b611ab8611abe916101ef565b916101ef565b908115611ac9570490565b611a98565b611add611ae3919392936101ef565b926101ef565b91611aef8382026101ef565b928184041490151715611afe57565b610fa5565b611b0b610e0b565b50611b1f611b196003610e42565b15610291565b611c1b57611b40611b3b6004611b356002610c02565b90610bd3565b61131b565b42611b6e611b68611b63611b555f86016115dc565b611b5d610780565b90610fde565b6101ef565b916101ef565b1015611b775790565b611bc490611bbe611baf5f611ba8611b9a42611b948488016115dc565b90610fb9565b611ba2610780565b90611aac565b93016115dc565b91611bb8610780565b90611ace565b90610fde565b611c18611c0f5f611c0a611c015f611bfc611bf35f95611bee611be5611a8b565b9a5f8c016112a4565b6115e9565b602089016112a4565b6115e9565b604086016112a4565b6115e9565b606083016112a4565b90565b5f611c78611c6f5f611c6a611c615f611c5c611c535f95611c4e611c46611c40611a8b565b9b6115e9565b5f8c016112a4565b6115e9565b602089016112a4565b6115e9565b604086016112a4565b6115e9565b606083016112a4565b90565b90611c875f19916111bc565b9181191691161790565b90565b90611ca9611ca4611cb092610bb7565b611c91565b8254611c7b565b9055565b916020611cd5929493611cce60408201965f83019061070a565b019061070a565b565b611cea611ce46003610f98565b15610291565b611ddb57611d01611cfb6003610e42565b15610291565b611dce575b611d0e6120ce565b611d7f611d1c823a90611ace565b611d4f83611d496002611d396004611d3383610c02565b90610bd3565b0191611d4483610c02565b610fde565b90611c94565b611d796003611d696004611d636002610c02565b90610bd3565b0191611d7483610c02565b610fde565b90611c94565b611d896002610c02565b3a611db47f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610bb7565b92611dc9611dc06101d2565b92839283611cb4565b0390a2565b611dd6611fc3565b611d06565b50565b611de66116e4565b611dff611df9611df46122a6565b6104cb565b916104cb565b03611e0657565b611e28611e116122a6565b5f91829163118cdaa760e01b83526004830161097d565b0390fd5b60081b90565b90611e3f61ff0091611e2c565b9181191691161790565b90611e5e611e59611e6592611175565b611181565b8254611e32565b9055565b611e745f6003611e49565b565b90565b90611e8e611e89611e9592611057565b611e76565b82546111c1565b9055565b611ea25f6116d7565b611eac825f611e79565b90611ee0611eda7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e093611057565b91611057565b91611ee96101d2565b80611ef381610487565b0390a3565b611f0460016003611e49565b565b90611f1260ff916111bc565b9181191691161790565b90611f31611f2c611f3892611175565b611181565b8254611f06565b9055565b90611f46906115e9565b5f5260205260405f2090565b90611faf60606003611fb594611f755f8201611f6f5f88016115dc565b90611c94565b611f8e60018201611f88602088016115dc565b90611c94565b611fa760028201611fa1604088016115dc565b90611c94565b0192016115dc565b90611c94565b565b90611fc191611f52565b565b611fd6611fd06003610e42565b15610291565b611fdd575b565b611fe960016003611f1c565b611ffc611ff55f6115e9565b6002611c94565b6120654261205461204b5f61204661203d5f61203861202f5f9561202a612021611a8b565b9a5f8c016112a4565b6115e9565b602089016112a4565b6115e9565b604086016112a4565b6115e9565b606083016112a4565b61206060045f90611f3c565b611fb7565b5f42906120a76120957f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e926115e9565b9261209e6101d2565b91829182610717565b0390a2611fdb565b90565b6120bb906101ef565b5f1981146120c95760010190565b610fa5565b6120eb6120e660046120e06002610c02565b90610bd3565b6120af565b4261211961211361210e6121005f8601610c02565b612108610780565b90610fde565b6101ef565b916101ef565b1015612123575b50565b61214b6121426121345f8401610c02565b61213c610780565b90610fde565b60018301611c94565b6121556002610c02565b61218261216460028401610c02565b9261217c5f61217560018401610c02565b9201610c02565b90610fb9565b6121ac7f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610bb7565b926121c16121b86101d2565b92839283611cb4565b0390a26121e06121d96121d46002610c02565b6120b2565b6002611c94565b6122524261223861222f5f61222a6122215f61221c6122135f9561220e612205611a8b565b9a5f8c016112a4565b6115e9565b602089016112a4565b6115e9565b604086016112a4565b6115e9565b606083016112a4565b61224d60046122476002610c02565b90610bd3565b611fb7565b61225c6002610c02565b429061229d61228b7f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610bb7565b926122946101d2565b91829182610717565b0390a25f612120565b6122ae6116b4565b50339056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\rfV[a\0\x1D_5a\x01\xCCV[\x80c\x08aF\xD2\x14a\x01\xC7W\x80c\x18\xD5\xAA\xFE\x14a\x01\xC2W\x80c6l\xBA\xB7\x14a\x01\xBDW\x80c;j\xB2\xA9\x14a\x01\xB8W\x80cF\xE2\xCC\t\x14a\x01\xB3W\x80cH\\\xC9U\x14a\x01\xAEW\x80cK,\x07\x06\x14a\x01\xA9W\x80cTg\xCBH\x14a\x01\xA4W\x80c[<\xD6\xE2\x14a\x01\x9FW\x80caT8\x01\x14a\x01\x9AW\x80ceX\x95O\x14a\x01\x95W\x80cqP\x18\xA6\x14a\x01\x90W\x80cz9y\xDC\x14a\x01\x8BW\x80c\x80NQ#\x14a\x01\x86W\x80c\x82\xF4J\xDE\x14a\x01\x81W\x80c\x84\xFA\xB6+\x14a\x01|W\x80c\x8DZ#\x9B\x14a\x01wW\x80c\x8D\xA5\xCB[\x14a\x01rW\x80c\xAF\xF7Lm\x14a\x01mW\x80c\xC6`\xD3\xF3\x14a\x01hW\x80c\xCD\xAF\xB9x\x14a\x01cW\x80c\xD4\xF0\xEBM\x14a\x01^W\x80c\xD8x\x13B\x14a\x01YW\x80c\xDE\x1FE>\x14a\x01TW\x80c\xEAJ\x11\x04\x14a\x01OW\x80c\xED\xE0{\xD6\x14a\x01JWc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\r3V[a\x0C\xFEV[a\x0C\x8DV[a\x0B\x84V[a\x0BOV[a\n\xF8V[a\n\xA6V[a\t\xFCV[a\t\xC7V[a\t\x92V[a\t;V[a\t\x06V[a\x08\xC1V[a\x08\x8DV[a\x08TV[a\x07\xCFV[a\x07\x9AV[a\x07,V[a\x06\x9DV[a\x05\xD1V[a\x05\x9CV[a\x05'V[a\x04\x8CV[a\x04RV[a\x03\xDDV[a\x02\xB8V[a\x02\\V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01\xEAWV[a\x01\xDCV[\x90V[a\x01\xFB\x90a\x01\xEFV[\x90RV[\x90``\x80a\x02E\x93a\x02\x17_\x82\x01Q_\x86\x01\x90a\x01\xF2V[a\x02)` \x82\x01Q` \x86\x01\x90a\x01\xF2V[a\x02;`@\x82\x01Q`@\x86\x01\x90a\x01\xF2V[\x01Q\x91\x01\x90a\x01\xF2V[V[\x91\x90a\x02Z\x90_`\x80\x85\x01\x94\x01\x90a\x01\xFFV[V[4a\x02\x8CWa\x02l6`\x04a\x01\xE0V[a\x02\x88a\x02wa\x0E\x16V[a\x02\x7Fa\x01\xD2V[\x91\x82\x91\x82a\x02GV[\x03\x90\xF3[a\x01\xD8V[\x15\x15\x90V[a\x02\x9F\x90a\x02\x91V[\x90RV[\x91\x90a\x02\xB6\x90_` \x85\x01\x94\x01\x90a\x02\x96V[V[4a\x02\xE8Wa\x02\xC86`\x04a\x01\xE0V[a\x02\xE4a\x02\xD3a\x0EOV[a\x02\xDBa\x01\xD2V[\x91\x82\x91\x82a\x02\xA3V[\x03\x90\xF3[a\x01\xD8V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x037W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x032W` \x01\x92`\x01\x83\x02\x84\x01\x11a\x03-WV[a\x02\xF9V[a\x02\xF5V[a\x02\xF1V[\x90` \x82\x82\x03\x12a\x03mW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03hWa\x03d\x92\x01a\x02\xFDV[\x90\x91V[a\x02\xEDV[a\x01\xDCV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\xB3a\x03\xBC` \x93a\x03\xC1\x93a\x03\xAA\x81a\x03rV[\x93\x84\x80\x93a\x03vV[\x95\x86\x91\x01a\x03\x7FV[a\x03\x8AV[\x01\x90V[a\x03\xDA\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\x94V[\x90V[4a\x04\x0EWa\x04\na\x03\xF9a\x03\xF36`\x04a\x03<V[\x90a\x0E\xF6V[a\x04\x01a\x01\xD2V[\x91\x82\x91\x82a\x03\xC5V[\x03\x90\xF3[a\x01\xD8V[\x1C\x90V[`\xFF\x16\x90V[a\x04-\x90`\x08a\x042\x93\x02a\x04\x13V[a\x04\x17V[\x90V[\x90a\x04@\x91Ta\x04\x1DV[\x90V[a\x04O`\x03_\x90a\x045V[\x90V[4a\x04\x82Wa\x04b6`\x04a\x01\xE0V[a\x04~a\x04ma\x04CV[a\x04ua\x01\xD2V[\x91\x82\x91\x82a\x02\xA3V[\x03\x90\xF3[a\x01\xD8V[_\x01\x90V[4a\x04\xBBWa\x04\xA5a\x04\x9F6`\x04a\x03<V[\x90a\x10\xE7V[a\x04\xADa\x01\xD2V[\x80a\x04\xB7\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\xD4\x90a\x04\xC0V[\x90V[a\x04\xE0\x81a\x04\xCBV[\x03a\x04\xE7WV[_\x80\xFD[\x90P5\x90a\x04\xF8\x82a\x04\xD7V[V[\x91\x90`@\x83\x82\x03\x12a\x05\"W\x80a\x05\x16a\x05\x1F\x92_\x86\x01a\x04\xEBV[\x93` \x01a\x04\xEBV[\x90V[a\x01\xDCV[4a\x05VWa\x05@a\x05:6`\x04a\x04\xFAV[\x90a\x12\x98V[a\x05Ha\x01\xD2V[\x80a\x05R\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[a\x05d\x81a\x01\xEFV[\x03a\x05kWV[_\x80\xFD[\x90P5\x90a\x05|\x82a\x05[V[V[\x90` \x82\x82\x03\x12a\x05\x97Wa\x05\x94\x91_\x01a\x05oV[\x90V[a\x01\xDCV[4a\x05\xCCWa\x05\xC8a\x05\xB7a\x05\xB26`\x04a\x05~V[a\x13'V[a\x05\xBFa\x01\xD2V[\x91\x82\x91\x82a\x02GV[\x03\x90\xF3[a\x01\xD8V[4a\x05\xFFWa\x05\xE16`\x04a\x01\xE0V[a\x05\xE9a\x13bV[a\x05\xF1a\x01\xD2V[\x80a\x05\xFB\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x06\x1F\x90`\x08a\x06$\x93\x02a\x04\x13V[a\x06\x04V[\x90V[\x90a\x062\x91Ta\x06\x0FV[\x90V[a\x06A`\x01_\x90a\x06'V[\x90V[\x90V[a\x06[a\x06Va\x06`\x92a\x04\xC0V[a\x06DV[a\x04\xC0V[\x90V[a\x06l\x90a\x06GV[\x90V[a\x06x\x90a\x06cV[\x90V[a\x06\x84\x90a\x06oV[\x90RV[\x91\x90a\x06\x9B\x90_` \x85\x01\x94\x01\x90a\x06{V[V[4a\x06\xCDWa\x06\xAD6`\x04a\x01\xE0V[a\x06\xC9a\x06\xB8a\x065V[a\x06\xC0a\x01\xD2V[\x91\x82\x91\x82a\x06\x88V[\x03\x90\xF3[a\x01\xD8V[\x90V[a\x06\xE5\x90`\x08a\x06\xEA\x93\x02a\x04\x13V[a\x06\xD2V[\x90V[\x90a\x06\xF8\x91Ta\x06\xD5V[\x90V[a\x07\x07`\x02_\x90a\x06\xEDV[\x90V[a\x07\x13\x90a\x01\xEFV[\x90RV[\x91\x90a\x07*\x90_` \x85\x01\x94\x01\x90a\x07\nV[V[4a\x07\\Wa\x07<6`\x04a\x01\xE0V[a\x07Xa\x07Ga\x06\xFBV[a\x07Oa\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[\x90V[a\x07xa\x07sa\x07}\x92a\x07aV[a\x06DV[a\x01\xEFV[\x90V[a\x07\x8Cb'\x8D\0a\x07dV[\x90V[a\x07\x97a\x07\x80V[\x90V[4a\x07\xCAWa\x07\xAA6`\x04a\x01\xE0V[a\x07\xC6a\x07\xB5a\x07\x8FV[a\x07\xBDa\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[4a\x07\xFDWa\x07\xDF6`\x04a\x01\xE0V[a\x07\xE7a\x13\x91V[a\x07\xEFa\x01\xD2V[\x80a\x07\xF9\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[\x91``\x83\x83\x03\x12a\x08OWa\x08\x19\x82_\x85\x01a\x04\xEBV[\x92a\x08'\x83` \x83\x01a\x04\xEBV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08JWa\x08F\x92\x01a\x02\xFDV[\x90\x91V[a\x02\xEDV[a\x01\xDCV[4a\x08\x88Wa\x08\x84a\x08sa\x08j6`\x04a\x08\x02V[\x92\x91\x90\x91a\x14IV[a\x08{a\x01\xD2V[\x91\x82\x91\x82a\x02\xA3V[\x03\x90\xF3[a\x01\xD8V[4a\x08\xBCWa\x08\xA6a\x08\xA06`\x04a\x03<V[\x90a\x15\xCCV[a\x08\xAEa\x01\xD2V[\x80a\x08\xB8\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[4a\x08\xF1Wa\x08\xD16`\x04a\x01\xE0V[a\x08\xEDa\x08\xDCa\x16\x05V[a\x08\xE4a\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[a\t\x03`\x03`\x01\x90a\x045V[\x90V[4a\t6Wa\t\x166`\x04a\x01\xE0V[a\t2a\t!a\x08\xF6V[a\t)a\x01\xD2V[\x91\x82\x91\x82a\x02\xA3V[\x03\x90\xF3[a\x01\xD8V[4a\tkWa\tK6`\x04a\x01\xE0V[a\tga\tVa\x16\x95V[a\t^a\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[a\ty\x90a\x04\xCBV[\x90RV[\x91\x90a\t\x90\x90_` \x85\x01\x94\x01\x90a\tpV[V[4a\t\xC2Wa\t\xA26`\x04a\x01\xE0V[a\t\xBEa\t\xADa\x16\xE4V[a\t\xB5a\x01\xD2V[\x91\x82\x91\x82a\t}V[\x03\x90\xF3[a\x01\xD8V[4a\t\xF7Wa\t\xD76`\x04a\x01\xE0V[a\t\xF3a\t\xE2a\x17\x18V[a\t\xEAa\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[4a\n,Wa\n\x0C6`\x04a\x01\xE0V[a\n(a\n\x17a\x17dV[a\n\x1Fa\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\nkW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\nfW` \x01\x92` \x83\x02\x84\x01\x11a\naWV[a\x02\xF9V[a\x02\xF5V[a\x02\xF1V[\x90` \x82\x82\x03\x12a\n\xA1W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\x9CWa\n\x98\x92\x01a\n1V[\x90\x91V[a\x02\xEDV[a\x01\xDCV[4a\n\xD5Wa\n\xBFa\n\xB96`\x04a\npV[\x90a\x19:V[a\n\xC7a\x01\xD2V[\x80a\n\xD1\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[\x90` \x82\x82\x03\x12a\n\xF3Wa\n\xF0\x91_\x01a\x04\xEBV[\x90V[a\x01\xDCV[4a\x0B&Wa\x0B\x10a\x0B\x0B6`\x04a\n\xDAV[a\x19\xEAV[a\x0B\x18a\x01\xD2V[\x80a\x0B\"\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B\x7FWa\x0B_6`\x04a\x01\xE0V[a\x0B{a\x0Bja\x0B+V[a\x0Bra\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[4a\x0B\xB2Wa\x0B\x946`\x04a\x01\xE0V[a\x0B\x9Ca\x1A\x11V[a\x0B\xA4a\x01\xD2V[\x80a\x0B\xAE\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[a\x0B\xCBa\x0B\xC6a\x0B\xD0\x92a\x01\xEFV[a\x06DV[a\x01\xEFV[\x90V[\x90a\x0B\xDD\x90a\x0B\xB7V[_R` R`@_ \x90V[_\x1C\x90V[a\x0B\xFAa\x0B\xFF\x91a\x0B\xE9V[a\x06\xD2V[\x90V[a\x0C\x0C\x90Ta\x0B\xEEV[\x90V[a\x0C\x1A\x90`\x04a\x0B\xD3V[\x90a\x0C&_\x83\x01a\x0C\x02V[\x91a\x0C3`\x01\x82\x01a\x0C\x02V[\x91a\x0CL`\x03a\x0CE`\x02\x85\x01a\x0C\x02V[\x93\x01a\x0C\x02V[\x90V[a\x0C\x84a\x0C\x8B\x94a\x0Cz``\x94\x98\x97\x95a\x0Cp`\x80\x86\x01\x9A_\x87\x01\x90a\x07\nV[` \x85\x01\x90a\x07\nV[`@\x83\x01\x90a\x07\nV[\x01\x90a\x07\nV[V[4a\x0C\xC1Wa\x0C\xBDa\x0C\xA8a\x0C\xA36`\x04a\x05~V[a\x0C\x0FV[\x90a\x0C\xB4\x94\x92\x94a\x01\xD2V[\x94\x85\x94\x85a\x0COV[\x03\x90\xF3[a\x01\xD8V[\x90V[a\x0C\xDDa\x0C\xD8a\x0C\xE2\x92a\x0C\xC6V[a\x06DV[a\x01\xEFV[\x90V[a\x0C\xF0a\x13\x88a\x0C\xC9V[\x90V[a\x0C\xFBa\x0C\xE5V[\x90V[4a\r.Wa\r\x0E6`\x04a\x01\xE0V[a\r*a\r\x19a\x0C\xF3V[a\r!a\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xF3[a\x01\xD8V[4a\raWa\rKa\rF6`\x04a\n\xDAV[a\x1A\x80V[a\rSa\x01\xD2V[\x80a\r]\x81a\x04\x87V[\x03\x90\xF3[a\x01\xD8V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\r\x88\x90a\x03\x8AV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r\xA2W`@RV[a\rjV[\x90a\r\xBAa\r\xB3a\x01\xD2V[\x92\x83a\r~V[V[a\r\xC6`\x80a\r\xA7V[\x90V[_\x90V[a\r\xD5a\r\xBCV[\x90` \x80\x80\x80\x85a\r\xE4a\r\xC9V[\x81R\x01a\r\xEFa\r\xC9V[\x81R\x01a\r\xFAa\r\xC9V[\x81R\x01a\x0E\x05a\r\xC9V[\x81RPPV[a\x0E\x13a\r\xCDV[\x90V[a\x0E\x1Ea\x0E\x0BV[Pa\x0E'a\x1B\x03V[\x90V[_\x90V[a\x0E:a\x0E?\x91a\x0B\xE9V[a\x04\x17V[\x90V[a\x0EL\x90Ta\x0E.V[\x90V[a\x0EWa\x0E*V[Pa\x0Eb`\x03a\x0EBV[\x90V[``\x90V[\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x0E\x90a\x0E\x8Ba\x0E\x95\x92a\x0EjV[a\x0EvV[a\x0EmV[\x90V[\x90V[a\x0E\xA7a\x0E\xAC\x91a\x0EmV[a\x0E\x98V[\x90RV[\x90P\x90V[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x0E\xD0\x81a\x0E\xD7\x93a\x0E\xB0V[\x80\x93a\x0E\xB5V[\x01\x90V[\x80a\x0E\xEC`\x01\x92a\x0E\xF3\x96\x94a\x0E\x9BV[\x01\x91a\x0E\xC0V[\x90V[a\x0F4\x90a\x0F\x02a\x0EeV[Pa\x0F%a\x0F\x0F_a\x0E|V[\x91\x93a\x0F\x19a\x01\xD2V[\x94\x85\x93` \x85\x01a\x0E\xDBV[` \x82\x01\x81\x03\x82R\x03\x82a\r~V[\x90V[\x90a\x0FSa\x0FM32\x90\x85\x85\x91\x92\x90\x91\x92a\x14IV[\x15a\x02\x91V[a\x0FbWa\x0F`\x91a\x10\x03V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x0Fz`\x04\x82\x01a\x04\x87V[\x03\x90\xFD[`\x08\x1C\x90V[a\x0F\x90a\x0F\x95\x91a\x0F~V[a\x04\x17V[\x90V[a\x0F\xA2\x90Ta\x0F\x84V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x0F\xC8a\x0F\xCE\x91\x93\x92\x93a\x01\xEFV[\x92a\x01\xEFV[\x82\x03\x91\x82\x11a\x0F\xD9WV[a\x0F\xA5V[a\x0F\xEDa\x0F\xF3\x91\x93\x92\x93a\x01\xEFV[\x92a\x01\xEFV[\x82\x01\x80\x92\x11a\x0F\xFEWV[a\x0F\xA5V[\x90a\x10\x17a\x10\x11`\x03a\x0F\x98V[\x15a\x02\x91V[a\x10LWa\x107a\x10J\x92a\x100a\x10E\x93Z\x92a\x10\xA0V[Z\x90a\x0F\xB9V[a\x10?a\x0C\xE5V[\x90a\x0F\xDEV[a\x1C\xD7V[V[a\x10U\x91a\x10\xA0V[V[a\x10`\x90a\x06cV[\x90V[\x91\x90a\x10}\x81a\x10v\x81a\x10\x82\x95a\x03vV[\x80\x95a\x0E\xB5V[a\x03\x8AV[\x01\x90V[\x90\x91a\x10\x9D\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x10cV[\x90V[3\x90\x91a\x10\xCD\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10WV[\x92a\x10\xE2a\x10\xD9a\x01\xD2V[\x92\x83\x92\x83a\x10\x86V[\x03\x90\xA2V[\x90a\x10\xF1\x91a\x0F7V[V[\x90a\x11\x05\x91a\x11\0a\x1D\xDEV[a\x12\x0BV[V[`\xA0\x1C\x90V[a\x11\x19a\x11\x1E\x91a\x11\x07V[a\x04\x17V[\x90V[a\x11+\x90Ta\x11\rV[\x90V[a\x11Ba\x11=a\x11G\x92a\x0EjV[a\x06DV[a\x04\xC0V[\x90V[a\x11S\x90a\x11.V[\x90V[`\xA0\x1B\x90V[\x90a\x11k`\xFF`\xA0\x1B\x91a\x11VV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x11~\x90a\x02\x91V[\x90V[\x90V[\x90a\x11\x99a\x11\x94a\x11\xA0\x92a\x11uV[a\x11\x81V[\x82Ta\x11\\V[\x90UV[a\x11\xAD\x90a\x06GV[\x90V[a\x11\xB9\x90a\x11\xA4V[\x90V[_\x1B\x90V[\x90a\x11\xD2`\x01\x80`\xA0\x1B\x03\x91a\x11\xBCV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x11\xE5\x90a\x11\xA4V[\x90V[\x90V[\x90a\x12\0a\x11\xFBa\x12\x07\x92a\x11\xDCV[a\x11\xE8V[\x82Ta\x11\xC1V[\x90UV[a\x12\x15`\x01a\x11!V[a\x12}W\x81a\x124a\x12.a\x12)_a\x11JV[a\x04\xCBV[\x91a\x04\xCBV[\x14a\x12aWa\x12Za\x12Sa\x12_\x93a\x12N`\x01\x80a\x11\x84V[a\x11\xB0V[`\x01a\x11\xEBV[a\x1A\x80V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x12y`\x04\x82\x01a\x04\x87V[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x12\x94`\x04\x82\x01a\x04\x87V[\x03\x90\xFD[\x90a\x12\xA2\x91a\x10\xF3V[V[\x90a\x12\xAE\x90a\x01\xEFV[\x90RV[\x90a\x13\x19a\x13\x10`\x03a\x12\xC3a\r\xBCV[\x94a\x12\xDAa\x12\xD2_\x83\x01a\x0C\x02V[_\x88\x01a\x12\xA4V[a\x12\xF2a\x12\xE9`\x01\x83\x01a\x0C\x02V[` \x88\x01a\x12\xA4V[a\x13\na\x13\x01`\x02\x83\x01a\x0C\x02V[`@\x88\x01a\x12\xA4V[\x01a\x0C\x02V[``\x84\x01a\x12\xA4V[V[a\x13$\x90a\x12\xB2V[\x90V[a\x13>a\x13C\x91a\x136a\x0E\x0BV[P`\x04a\x0B\xD3V[a\x13\x1BV[\x90V[a\x13Na\x1D\xDEV[a\x13Va\x13XV[V[a\x13`a\x1EiV[V[a\x13ja\x13FV[V[a\x13ta\x1D\xDEV[a\x13|a\x13~V[V[a\x13\x8Fa\x13\x8A_a\x11JV[a\x1E\x99V[V[a\x13\x99a\x13lV[V[a\x13\xA7a\x13\xAC\x91a\x0B\xE9V[a\x06\x04V[\x90V[a\x13\xB9\x90Ta\x13\x9BV[\x90V[`\xE0\x1B\x90V[a\x13\xCB\x81a\x02\x91V[\x03a\x13\xD2WV[_\x80\xFD[\x90PQ\x90a\x13\xE3\x82a\x13\xC2V[V[\x90` \x82\x82\x03\x12a\x13\xFEWa\x13\xFB\x91_\x01a\x13\xD6V[\x90V[a\x01\xDCV[a\x14)a\x146\x95\x93\x94\x92\x94a\x14\x1F``\x84\x01\x96_\x85\x01\x90a\tpV[` \x83\x01\x90a\tpV[`@\x81\x85\x03\x91\x01Ra\x10cV[\x90V[a\x14Aa\x01\xD2V[=_\x82>=\x90\xFD[\x92a\x14\x8C` \x93\x94a\x14Ya\x0E*V[Pa\x14\x97a\x14oa\x14j`\x01a\x13\xAFV[a\x06oV[\x93cz9y\xDC\x92\x95\x97a\x14\x80a\x01\xD2V[\x98\x89\x97\x88\x96\x87\x96a\x13\xBCV[\x86R`\x04\x86\x01a\x14\x03V[\x03\x91Z\xFA\x90\x81\x15a\x14\xDBW_\x91a\x14\xADW[P\x90V[a\x14\xCE\x91P` =\x81\x11a\x14\xD4W[a\x14\xC6\x81\x83a\r~V[\x81\x01\x90a\x13\xE5V[_a\x14\xA9V[P=a\x14\xBCV[a\x149V[\x90a\x14\xFCa\x14\xF632\x90\x85\x85\x91\x92\x90\x91\x92a\x14IV[\x15a\x02\x91V[a\x15\x0BWa\x15\t\x91a\x15'V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x15#`\x04\x82\x01a\x04\x87V[\x03\x90\xFD[\x90a\x15;a\x155`\x03a\x0F\x98V[\x15a\x02\x91V[a\x15pWa\x15[a\x15n\x92a\x15Ta\x15i\x93Z\x92a\x15{V[Z\x90a\x0F\xB9V[a\x15ca\x0C\xE5V[\x90a\x0F\xDEV[a\x1C\xD7V[V[a\x15y\x91a\x15{V[V[\x90a\x15\x87\x903\x92a\x0E\xF6V[\x90a\x15\xC7a\x15\xB5\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10WV[\x92a\x15\xBEa\x01\xD2V[\x91\x82\x91\x82a\x03\xC5V[\x03\x90\xA2V[\x90a\x15\xD6\x91a\x14\xE0V[V[_\x90V[a\x15\xE6\x90Qa\x01\xEFV[\x90V[a\x15\xFDa\x15\xF8a\x16\x02\x92a\x0EjV[a\x06DV[a\x01\xEFV[\x90V[a\x16\ra\x15\xD8V[Pa\x16\x16a\x1B\x03V[a\x16!_\x82\x01a\x15\xDCV[a\x163a\x16-_a\x15\xE9V[\x91a\x01\xEFV[\x14a\x16\x88Wa\x16F_a\x16T\x92\x01a\x15\xDCV[a\x16Na\x07\x80V[\x90a\x0F\xDEV[Ba\x16ga\x16a\x83a\x01\xEFV[\x91a\x01\xEFV[\x10\x15a\x16{Wa\x16x\x90B\x90a\x0F\xB9V[\x90V[Pa\x16\x85_a\x15\xE9V[\x90V[Pa\x16\x92_a\x15\xE9V[\x90V[a\x16\x9Da\x15\xD8V[Pa\x16\xB1``a\x16\xABa\x1B\x03V[\x01a\x15\xDCV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x16\xCFa\x16\xD4\x91a\x0B\xE9V[a\x16\xB8V[\x90V[a\x16\xE1\x90Ta\x16\xC3V[\x90V[a\x16\xECa\x16\xB4V[Pa\x16\xF6_a\x16\xD7V[\x90V[\x90V[a\x17\x10a\x17\x0Ba\x17\x15\x92a\x16\xF9V[a\x06DV[a\x01\xEFV[\x90V[a\x17 a\x15\xD8V[Pa\x174a\x17.`\x03a\x0EBV[\x15a\x02\x91V[a\x17XWa\x17Ua\x17E`\x02a\x0C\x02V[a\x17O`\x01a\x16\xFCV[\x90a\x0F\xDEV[\x90V[a\x17a_a\x15\xE9V[\x90V[a\x17la\x15\xD8V[Pa\x17\x80`@a\x17za\x1B\x03V[\x01a\x15\xDCV[\x90V[\x90a\x17\x97a\x17\x91`\x03a\x0F\x98V[\x15a\x02\x91V[a\x17\xCCWa\x17\xB7a\x17\xCA\x92a\x17\xB0a\x17\xC5\x93Z\x92a\x18qV[Z\x90a\x0F\xB9V[a\x17\xBFa\x0C\xE5V[\x90a\x0F\xDEV[a\x1C\xD7V[V[a\x17\xD5\x91a\x18qV[V[P\x90V[`\x01a\x17\xE7\x91\x01a\x01\xEFV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x18LW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x18GW` \x01\x91`\x01\x82\x026\x03\x83\x13a\x18BWV[a\x18\x06V[a\x18\x02V[a\x17\xFEV[\x90\x82\x10\x15a\x18lW` a\x18h\x92\x02\x81\x01\x90a\x18\nV[\x90\x91V[a\x17\xEAV[a\x18|\x81\x83\x90a\x17\xD7V[\x91a\x18\x85a\x15\xD8V[Pa\x18\x8F_a\x15\xE9V[[\x80a\x18\xA3a\x18\x9D\x86a\x01\xEFV[\x91a\x01\xEFV[\x10\x15a\x194Wa\x18\xD1\x90a\x18\xC732\x90a\x18\xBF\x87\x87\x86\x91a\x18QV[\x92\x90\x91a\x14IV[a\x18\xD6W[a\x17\xDBV[a\x18\x90V[3a\x18\xECa\x18\xE6\x86\x86\x85\x91a\x18QV[\x90a\x0E\xF6V[\x90a\x19,a\x19\x1A\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10WV[\x92a\x19#a\x01\xD2V[\x91\x82\x91\x82a\x03\xC5V[\x03\x90\xA2a\x18\xCCV[PPPPV[\x90a\x19D\x91a\x17\x83V[V[a\x19W\x90a\x19Ra\x1D\xDEV[a\x19YV[V[\x80a\x19ta\x19na\x19i_a\x11JV[a\x04\xCBV[\x91a\x04\xCBV[\x14a\x19\xCEWa\x19\x8Ca\x19\x85\x82a\x11\xB0V[`\x01a\x11\xEBV[a\x19\xB6\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10WV[\x90a\x19\xBFa\x01\xD2V[\x80a\x19\xC9\x81a\x04\x87V[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x19\xE6`\x04\x82\x01a\x04\x87V[\x03\x90\xFD[a\x19\xF3\x90a\x19FV[V[a\x19\xFDa\x1D\xDEV[a\x1A\x05a\x1A\x07V[V[a\x1A\x0Fa\x1E\xF8V[V[a\x1A\x19a\x19\xF5V[V[a\x1A,\x90a\x1A'a\x1D\xDEV[a\x1A.V[V[\x80a\x1AIa\x1ACa\x1A>_a\x11JV[a\x04\xCBV[\x91a\x04\xCBV[\x14a\x1AYWa\x1AW\x90a\x1E\x99V[V[a\x1A|a\x1Ae_a\x11JV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\t}V[\x03\x90\xFD[a\x1A\x89\x90a\x1A\x1BV[V[a\x1A\x95`\x80a\r\xA7V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\x1A\xB8a\x1A\xBE\x91a\x01\xEFV[\x91a\x01\xEFV[\x90\x81\x15a\x1A\xC9W\x04\x90V[a\x1A\x98V[a\x1A\xDDa\x1A\xE3\x91\x93\x92\x93a\x01\xEFV[\x92a\x01\xEFV[\x91a\x1A\xEF\x83\x82\x02a\x01\xEFV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x1A\xFEWV[a\x0F\xA5V[a\x1B\x0Ba\x0E\x0BV[Pa\x1B\x1Fa\x1B\x19`\x03a\x0EBV[\x15a\x02\x91V[a\x1C\x1BWa\x1B@a\x1B;`\x04a\x1B5`\x02a\x0C\x02V[\x90a\x0B\xD3V[a\x13\x1BV[Ba\x1Bna\x1Bha\x1Bca\x1BU_\x86\x01a\x15\xDCV[a\x1B]a\x07\x80V[\x90a\x0F\xDEV[a\x01\xEFV[\x91a\x01\xEFV[\x10\x15a\x1BwW\x90V[a\x1B\xC4\x90a\x1B\xBEa\x1B\xAF_a\x1B\xA8a\x1B\x9ABa\x1B\x94\x84\x88\x01a\x15\xDCV[\x90a\x0F\xB9V[a\x1B\xA2a\x07\x80V[\x90a\x1A\xACV[\x93\x01a\x15\xDCV[\x91a\x1B\xB8a\x07\x80V[\x90a\x1A\xCEV[\x90a\x0F\xDEV[a\x1C\x18a\x1C\x0F_a\x1C\na\x1C\x01_a\x1B\xFCa\x1B\xF3_\x95a\x1B\xEEa\x1B\xE5a\x1A\x8BV[\x9A_\x8C\x01a\x12\xA4V[a\x15\xE9V[` \x89\x01a\x12\xA4V[a\x15\xE9V[`@\x86\x01a\x12\xA4V[a\x15\xE9V[``\x83\x01a\x12\xA4V[\x90V[_a\x1Cxa\x1Co_a\x1Cja\x1Ca_a\x1C\\a\x1CS_\x95a\x1CNa\x1CFa\x1C@a\x1A\x8BV[\x9Ba\x15\xE9V[_\x8C\x01a\x12\xA4V[a\x15\xE9V[` \x89\x01a\x12\xA4V[a\x15\xE9V[`@\x86\x01a\x12\xA4V[a\x15\xE9V[``\x83\x01a\x12\xA4V[\x90V[\x90a\x1C\x87_\x19\x91a\x11\xBCV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x1C\xA9a\x1C\xA4a\x1C\xB0\x92a\x0B\xB7V[a\x1C\x91V[\x82Ta\x1C{V[\x90UV[\x91` a\x1C\xD5\x92\x94\x93a\x1C\xCE`@\x82\x01\x96_\x83\x01\x90a\x07\nV[\x01\x90a\x07\nV[V[a\x1C\xEAa\x1C\xE4`\x03a\x0F\x98V[\x15a\x02\x91V[a\x1D\xDBWa\x1D\x01a\x1C\xFB`\x03a\x0EBV[\x15a\x02\x91V[a\x1D\xCEW[a\x1D\x0Ea \xCEV[a\x1D\x7Fa\x1D\x1C\x82:\x90a\x1A\xCEV[a\x1DO\x83a\x1DI`\x02a\x1D9`\x04a\x1D3\x83a\x0C\x02V[\x90a\x0B\xD3V[\x01\x91a\x1DD\x83a\x0C\x02V[a\x0F\xDEV[\x90a\x1C\x94V[a\x1Dy`\x03a\x1Di`\x04a\x1Dc`\x02a\x0C\x02V[\x90a\x0B\xD3V[\x01\x91a\x1Dt\x83a\x0C\x02V[a\x0F\xDEV[\x90a\x1C\x94V[a\x1D\x89`\x02a\x0C\x02V[:a\x1D\xB4\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0B\xB7V[\x92a\x1D\xC9a\x1D\xC0a\x01\xD2V[\x92\x83\x92\x83a\x1C\xB4V[\x03\x90\xA2V[a\x1D\xD6a\x1F\xC3V[a\x1D\x06V[PV[a\x1D\xE6a\x16\xE4V[a\x1D\xFFa\x1D\xF9a\x1D\xF4a\"\xA6V[a\x04\xCBV[\x91a\x04\xCBV[\x03a\x1E\x06WV[a\x1E(a\x1E\x11a\"\xA6V[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\t}V[\x03\x90\xFD[`\x08\x1B\x90V[\x90a\x1E?a\xFF\0\x91a\x1E,V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1E^a\x1EYa\x1Ee\x92a\x11uV[a\x11\x81V[\x82Ta\x1E2V[\x90UV[a\x1Et_`\x03a\x1EIV[V[\x90V[\x90a\x1E\x8Ea\x1E\x89a\x1E\x95\x92a\x10WV[a\x1EvV[\x82Ta\x11\xC1V[\x90UV[a\x1E\xA2_a\x16\xD7V[a\x1E\xAC\x82_a\x1EyV[\x90a\x1E\xE0a\x1E\xDA\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10WV[\x91a\x10WV[\x91a\x1E\xE9a\x01\xD2V[\x80a\x1E\xF3\x81a\x04\x87V[\x03\x90\xA3V[a\x1F\x04`\x01`\x03a\x1EIV[V[\x90a\x1F\x12`\xFF\x91a\x11\xBCV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1F1a\x1F,a\x1F8\x92a\x11uV[a\x11\x81V[\x82Ta\x1F\x06V[\x90UV[\x90a\x1FF\x90a\x15\xE9V[_R` R`@_ \x90V[\x90a\x1F\xAF```\x03a\x1F\xB5\x94a\x1Fu_\x82\x01a\x1Fo_\x88\x01a\x15\xDCV[\x90a\x1C\x94V[a\x1F\x8E`\x01\x82\x01a\x1F\x88` \x88\x01a\x15\xDCV[\x90a\x1C\x94V[a\x1F\xA7`\x02\x82\x01a\x1F\xA1`@\x88\x01a\x15\xDCV[\x90a\x1C\x94V[\x01\x92\x01a\x15\xDCV[\x90a\x1C\x94V[V[\x90a\x1F\xC1\x91a\x1FRV[V[a\x1F\xD6a\x1F\xD0`\x03a\x0EBV[\x15a\x02\x91V[a\x1F\xDDW[V[a\x1F\xE9`\x01`\x03a\x1F\x1CV[a\x1F\xFCa\x1F\xF5_a\x15\xE9V[`\x02a\x1C\x94V[a eBa Ta K_a Fa =_a 8a /_\x95a *a !a\x1A\x8BV[\x9A_\x8C\x01a\x12\xA4V[a\x15\xE9V[` \x89\x01a\x12\xA4V[a\x15\xE9V[`@\x86\x01a\x12\xA4V[a\x15\xE9V[``\x83\x01a\x12\xA4V[a ``\x04_\x90a\x1F<V[a\x1F\xB7V[_B\x90a \xA7a \x95\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x15\xE9V[\x92a \x9Ea\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xA2a\x1F\xDBV[\x90V[a \xBB\x90a\x01\xEFV[_\x19\x81\x14a \xC9W`\x01\x01\x90V[a\x0F\xA5V[a \xEBa \xE6`\x04a \xE0`\x02a\x0C\x02V[\x90a\x0B\xD3V[a \xAFV[Ba!\x19a!\x13a!\x0Ea!\0_\x86\x01a\x0C\x02V[a!\x08a\x07\x80V[\x90a\x0F\xDEV[a\x01\xEFV[\x91a\x01\xEFV[\x10\x15a!#W[PV[a!Ka!Ba!4_\x84\x01a\x0C\x02V[a!<a\x07\x80V[\x90a\x0F\xDEV[`\x01\x83\x01a\x1C\x94V[a!U`\x02a\x0C\x02V[a!\x82a!d`\x02\x84\x01a\x0C\x02V[\x92a!|_a!u`\x01\x84\x01a\x0C\x02V[\x92\x01a\x0C\x02V[\x90a\x0F\xB9V[a!\xAC\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0B\xB7V[\x92a!\xC1a!\xB8a\x01\xD2V[\x92\x83\x92\x83a\x1C\xB4V[\x03\x90\xA2a!\xE0a!\xD9a!\xD4`\x02a\x0C\x02V[a \xB2V[`\x02a\x1C\x94V[a\"RBa\"8a\"/_a\"*a\"!_a\"\x1Ca\"\x13_\x95a\"\x0Ea\"\x05a\x1A\x8BV[\x9A_\x8C\x01a\x12\xA4V[a\x15\xE9V[` \x89\x01a\x12\xA4V[a\x15\xE9V[`@\x86\x01a\x12\xA4V[a\x15\xE9V[``\x83\x01a\x12\xA4V[a\"M`\x04a\"G`\x02a\x0C\x02V[\x90a\x0B\xD3V[a\x1F\xB7V[a\"\\`\x02a\x0C\x02V[B\x90a\"\x9Da\"\x8B\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0B\xB7V[\x92a\"\x94a\x01\xD2V[\x91\x82\x91\x82a\x07\x17V[\x03\x90\xA2_a! V[a\"\xAEa\x16\xB4V[P3\x90V",
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
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SyndicateSequencingChainCalls {
        const NAME: &'static str = "SyndicateSequencingChainCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 27usize;
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
