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
    function gasPriceInSynd() external view returns (uint256);
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
    function periods(uint256) external view returns (uint256 startTimestamp, uint256 endTimestamp, uint256 totalGasUsed, bool finalized);
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
    ///0x60a060405234610038576100196100146100e9565b6101b7565b61002161003d565b61224561057d823960805181610b8e015261224590f35b610043565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b9061006f90610047565b810190811060018060401b0382111761008757604052565b610051565b9061009f61009861003d565b9283610065565b565b5f80fd5b90565b6100b1816100a5565b036100b857565b5f80fd5b905051906100c9826100a8565b565b906020828203126100e4576100e1915f016100bc565b90565b6100a1565b610107612981803803806100fc8161008c565b9283398101906100cb565b90565b90565b90565b61012461011f6101299261010a565b61010d565b6100a5565b90565b60209181520190565b5f7f41707020636861696e2049442063616e6e6f7420626520300000000000000000910152565b610169601860209261012c565b61017281610135565b0190565b61018b9060208101905f81830391015261015c565b90565b1561019557565b61019d61003d565b62461bcd60e51b8152806101b360048201610176565b0390fd5b6101bf61028f565b6101dc816101d56101cf5f610110565b916100a5565b141561018e565b608052565b5f1b90565b906101f25f19916101e1565b9181191691161790565b90565b61021361020e610218926101fc565b61010d565b6100a5565b90565b90565b9061023361022e61023a926101ff565b61021b565b82546101e6565b9055565b60081b90565b9061025161ff009161023e565b9181191691161790565b151590565b6102699061025b565b90565b90565b9061028461027f61028b92610260565b61026c565b8254610244565b9055565b610297610391565b6102a6633b9aca00600261021e565b6102b26001600461026f565b565b60a01b90565b906102c960ff60a01b916102b4565b9181191691161790565b906102e86102e36102ef92610260565b61026c565b82546102ba565b9055565b5f0190565b61030061003d565b3d5f823e3d90fd5b60018060a01b031690565b61032761032261032c92610308565b61010d565b610308565b90565b61033890610313565b90565b6103449061032f565b90565b9061035860018060a01b03916101e1565b9181191691161790565b61036b9061032f565b90565b90565b9061038661038161038d92610362565b61036e565b8254610347565b9055565b61039a336103fe565b6103a55f60016102d3565b6103ad61003d565b6101bf810181811060018060401b038211176103f9576103d582916101bf6127c284396102f3565b03905ff080156103f4576103eb6103f29161033b565b6001610371565b565b6102f8565b610051565b6104079061045f565b565b61041d6104186104229261010a565b61010d565b610308565b90565b61042e90610409565b90565b61043a90610308565b90565b61044690610431565b9052565b919061045d905f6020850194019061043d565b565b8061047a61047461046f5f610425565b610431565b91610431565b1461048a576104889061051d565b565b6104ad6104965f610425565b5f918291631e4fbdf760e01b83526004830161044a565b0390fd5b5f1c90565b60018060a01b031690565b6104cd6104d2916104b1565b6104b6565b90565b6104df90546104c1565b90565b6104eb90610313565b90565b6104f7906104e2565b90565b90565b9061051261050d610519926104ee565b6104fa565b8254610347565b9055565b6105265f6104d5565b610530825f6104fd565b9061056461055e7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0936104ee565b916104ee565b9161056d61003d565b80610577816102f3565b0390a356fe60806040526004361015610013575b610de8565b61001d5f356101dc565b8063086146d2146101d757806318d5aafe146101d2578063366cbab7146101cd5780633b6ab2a9146101c857806346e2cc09146101c3578063485cc955146101be5780634b2c0706146101b95780635467cb48146101b45780635b3cd6e2146101af57806361543801146101aa5780636558954f146101a5578063703cfcbb146101a0578063715018a61461019b5780637a3979dc14610196578063804e51231461019157806382f44ade1461018c57806384fab62b146101875780638d5a239b146101825780638da5cb5b1461017d578063aff74c6d14610178578063c660d3f314610173578063cdafb9781461016e578063d4f0eb4d14610169578063d878134214610164578063de1f453e1461015f578063ea4a11041461015a578063ede07bd6146101555763f2fde38b0361000e57610db5565b610d80565b610d0f565b610be5565b610bb0565b610b59565b610b07565b610a5d565b610a28565b6109f3565b61099c565b610967565b610922565b6108ee565b6108b5565b610830565b6107fb565b6107b7565b610749565b6106ba565b6105ee565b6105b9565b610544565b6104a9565b61046f565b6103fa565b6102d5565b61027e565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f9103126101fa57565b6101ec565b90565b61020b906101ff565b9052565b151590565b61021d9061020f565b9052565b90606080610267936102395f8201515f860190610202565b61024b60208201516020860190610202565b61025d60408201516040860190610202565b0151910190610214565b565b919061027c905f60808501940190610221565b565b346102ae5761028e3660046101f0565b6102aa610299610f59565b6102a16101e2565b91829182610269565b0390f35b6101e8565b6102bc9061020f565b9052565b91906102d3905f602085019401906102b3565b565b34610305576102e53660046101f0565b6103016102f0610ff5565b6102f86101e2565b918291826102c0565b0390f35b6101e8565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156103545781359167ffffffffffffffff831161034f57602001926001830284011161034a57565b610316565b610312565b61030e565b9060208282031261038a575f82013567ffffffffffffffff811161038557610381920161031a565b9091565b61030a565b6101ec565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6103d06103d96020936103de936103c78161038f565b93848093610393565b9586910161039c565b6103a7565b0190565b6103f79160208201915f8184039101526103b1565b90565b3461042b57610427610416610410366004610359565b90611099565b61041e6101e2565b918291826103e2565b0390f35b6101e8565b1c90565b60ff1690565b61044a90600861044f9302610430565b610434565b90565b9061045d915461043a565b90565b61046c60045f90610452565b90565b3461049f5761047f3660046101f0565b61049b61048a610460565b6104926101e2565b918291826102c0565b0390f35b6101e8565b5f0190565b346104d8576104c26104bc366004610359565b9061128a565b6104ca6101e2565b806104d4816104a4565b0390f35b6101e8565b60018060a01b031690565b6104f1906104dd565b90565b6104fd816104e8565b0361050457565b5f80fd5b90503590610515826104f4565b565b919060408382031261053f578061053361053c925f8601610508565b93602001610508565b90565b6101ec565b346105735761055d610557366004610517565b9061143b565b6105656101e2565b8061056f816104a4565b0390f35b6101e8565b610581816101ff565b0361058857565b5f80fd5b9050359061059982610578565b565b906020828203126105b4576105b1915f0161058c565b90565b6101ec565b346105e9576105e56105d46105cf36600461059b565b611447565b6105dc6101e2565b91829182610269565b0390f35b6101e8565b3461061c576105fe3660046101f0565b610606611482565b61060e6101e2565b80610618816104a4565b0390f35b6101e8565b60018060a01b031690565b61063c9060086106419302610430565b610621565b90565b9061064f915461062c565b90565b61065e60015f90610644565b90565b90565b61067861067361067d926104dd565b610661565b6104dd565b90565b61068990610664565b90565b61069590610680565b90565b6106a19061068c565b9052565b91906106b8905f60208501940190610698565b565b346106ea576106ca3660046101f0565b6106e66106d5610652565b6106dd6101e2565b918291826106a5565b0390f35b6101e8565b90565b6107029060086107079302610430565b6106ef565b90565b9061071591546106f2565b90565b61072460035f9061070a565b90565b610730906101ff565b9052565b9190610747905f60208501940190610727565b565b34610779576107593660046101f0565b610775610764610718565b61076c6101e2565b91829182610734565b0390f35b6101e8565b90565b61079561079061079a9261077e565b610661565b6101ff565b90565b6107a962278d00610781565b90565b6107b461079d565b90565b346107e7576107c73660046101f0565b6107e36107d26107ac565b6107da6101e2565b91829182610734565b0390f35b6101e8565b6107f860025f9061070a565b90565b3461082b5761080b3660046101f0565b6108276108166107ec565b61081e6101e2565b91829182610734565b0390f35b6101e8565b3461085e576108403660046101f0565b6108486114b1565b6108506101e2565b8061085a816104a4565b0390f35b6101e8565b916060838303126108b05761087a825f8501610508565b926108888360208301610508565b92604082013567ffffffffffffffff81116108ab576108a7920161031a565b9091565b61030a565b6101ec565b346108e9576108e56108d46108cb366004610863565b92919091611569565b6108dc6101e2565b918291826102c0565b0390f35b6101e8565b3461091d57610907610901366004610359565b906116ec565b61090f6101e2565b80610919816104a4565b0390f35b6101e8565b34610952576109323660046101f0565b61094e61093d611709565b6109456101e2565b91829182610734565b0390f35b6101e8565b6109646004600190610452565b90565b34610997576109773660046101f0565b610993610982610957565b61098a6101e2565b918291826102c0565b0390f35b6101e8565b346109cc576109ac3660046101f0565b6109c86109b76117d6565b6109bf6101e2565b91829182610734565b0390f35b6101e8565b6109da906104e8565b9052565b91906109f1905f602085019401906109d1565b565b34610a2357610a033660046101f0565b610a1f610a0e611868565b610a166101e2565b918291826109de565b0390f35b6101e8565b34610a5857610a383660046101f0565b610a54610a4361189c565b610a4b6101e2565b91829182610734565b0390f35b6101e8565b34610a8d57610a6d3660046101f0565b610a89610a786118e8565b610a806101e2565b91829182610734565b0390f35b6101e8565b909182601f83011215610acc5781359167ffffffffffffffff8311610ac7576020019260208302840111610ac257565b610316565b610312565b61030e565b90602082820312610b02575f82013567ffffffffffffffff8111610afd57610af99201610a92565b9091565b61030a565b6101ec565b34610b3657610b20610b1a366004610ad1565b90611aee565b610b286101e2565b80610b32816104a4565b0390f35b6101e8565b90602082820312610b5457610b51915f01610508565b90565b6101ec565b34610b8757610b71610b6c366004610b3b565b611b9e565b610b796101e2565b80610b83816104a4565b0390f35b6101e8565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610be057610bc03660046101f0565b610bdc610bcb610b8c565b610bd36101e2565b91829182610734565b0390f35b6101e8565b34610c1357610bf53660046101f0565b610bfd611bc5565b610c056101e2565b80610c0f816104a4565b0390f35b6101e8565b610c2c610c27610c31926101ff565b610661565b6101ff565b90565b90610c3e90610c18565b5f5260205260405f2090565b5f1c90565b610c5b610c6091610c4a565b6106ef565b90565b610c6d9054610c4f565b90565b610c7c610c8191610c4a565b610434565b90565b610c8e9054610c70565b90565b610c9c906005610c34565b90610ca85f8301610c63565b91610cb560018201610c63565b91610cce6003610cc760028501610c63565b9301610c84565b90565b610d06610d0d94610cfc606094989795610cf2608086019a5f870190610727565b6020850190610727565b6040830190610727565b01906102b3565b565b34610d4357610d3f610d2a610d2536600461059b565b610c91565b90610d369492946101e2565b94859485610cd1565b0390f35b6101e8565b90565b610d5f610d5a610d6492610d48565b610661565b6101ff565b90565b610d72611388610d4b565b90565b610d7d610d67565b90565b34610db057610d903660046101f0565b610dac610d9b610d75565b610da36101e2565b91829182610734565b0390f35b6101e8565b34610de357610dcd610dc8366004610b3b565b611c34565b610dd56101e2565b80610ddf816104a4565b0390f35b6101e8565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b90610e0a906103a7565b810190811067ffffffffffffffff821117610e2457604052565b610dec565b90610e3c610e356101e2565b9283610e00565b565b610e486080610e29565b90565b5f90565b5f90565b610e5b610e3e565b90602080808085610e6a610e4b565b815201610e75610e4b565b815201610e80610e4b565b815201610e8b610e4f565b81525050565b610e99610e53565b90565b610ea66080610e29565b90565b90565b610ec0610ebb610ec592610ea9565b610661565b6101ff565b90565b90610ed2906101ff565b9052565b90610ee09061020f565b9052565b90610f4b610f426003610ef5610e3e565b94610f0c610f045f8301610c63565b5f8801610ec8565b610f24610f1b60018301610c63565b60208801610ec8565b610f3c610f3360028301610c63565b60408801610ec8565b01610c84565b60608401610ed6565b565b610f5690610ee4565b90565b610f61610e91565b50610f75610f6f6004610c84565b1561020f565b610f9957610f96610f916005610f8b6003610c63565b90610c34565b610f4d565b90565b5f610fee5f610fe5610fdc5f610fd7610fce5f95610fc9610fc1610fbb610e9c565b9a610eac565b5f8b01610ec8565b610eac565b60208801610ec8565b610eac565b60408501610ec8565b60608301610ed6565b90565b5f90565b610ffd610ff1565b506110086004610c84565b90565b606090565b60ff60f81b1690565b60f81b90565b61103361102e61103892610ea9565b611019565b611010565b90565b90565b61104a61104f91611010565b61103b565b9052565b905090565b90825f939282370152565b9091826110738161107a93611053565b8093611058565b0190565b8061108f600192611096969461103e565b0191611063565b90565b6110d7906110a561100b565b506110c86110b25f61101f565b91936110bc6101e2565b9485936020850161107e565b60208201810382520382610e00565b90565b906110f66110f033329085859192909192611569565b1561020f565b61110557611103916111a6565b565b5f631b8e828b60e31b81528061111d600482016104a4565b0390fd5b60081c90565b61113361113891611121565b610434565b90565b6111459054611127565b90565b634e487b7160e01b5f52601160045260245ffd5b61116b611171919392936101ff565b926101ff565b820391821161117c57565b611148565b611190611196919392936101ff565b926101ff565b82018092116111a157565b611148565b906111ba6111b4600461113b565b1561020f565b6111ef576111da6111ed926111d36111e8935a92611243565b5a9061115c565b6111e2610d67565b90611181565b611c9b565b565b6111f891611243565b565b61120390610680565b90565b9190611220816112198161122595610393565b8095611058565b6103a7565b0190565b90916112409260208301925f818503910152611206565b90565b3390916112707f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926111fa565b9261128561127c6101e2565b92839283611229565b0390a2565b90611294916110da565b565b906112a8916112a3611d65565b6113ae565b565b60a01c90565b6112bc6112c1916112aa565b610434565b90565b6112ce90546112b0565b90565b6112e56112e06112ea92610ea9565b610661565b6104dd565b90565b6112f6906112d1565b90565b60a01b90565b9061130e60ff60a01b916112f9565b9181191691161790565b6113219061020f565b90565b90565b9061133c61133761134392611318565b611324565b82546112ff565b9055565b61135090610664565b90565b61135c90611347565b90565b5f1b90565b9061137560018060a01b039161135f565b9181191691161790565b61138890611347565b90565b90565b906113a361139e6113aa9261137f565b61138b565b8254611364565b9055565b6113b860016112c4565b61142057816113d76113d16113cc5f6112ed565b6104e8565b916104e8565b14611404576113fd6113f6611402936113f1600180611327565b611353565b600161138e565b611c34565b565b5f632e7f3c7f60e11b81528061141c600482016104a4565b0390fd5b5f62dc149f60e41b815280611437600482016104a4565b0390fd5b9061144591611296565b565b61145e61146391611456610e91565b506005610c34565b610f4d565b90565b61146e611d65565b611476611478565b565b611480611df0565b565b61148a611466565b565b611494611d65565b61149c61149e565b565b6114af6114aa5f6112ed565b611e20565b565b6114b961148c565b565b6114c76114cc91610c4a565b610621565b90565b6114d990546114bb565b90565b60e01b90565b6114eb8161020f565b036114f257565b5f80fd5b90505190611503826114e2565b565b9060208282031261151e5761151b915f016114f6565b90565b6101ec565b611549611556959394929461153f60608401965f8501906109d1565b60208301906109d1565b6040818503910152611206565b90565b6115616101e2565b3d5f823e3d90fd5b926115ac60209394611579610ff1565b506115b761158f61158a60016114cf565b61068c565b93637a3979dc9295976115a06101e2565b988997889687966114dc565b865260048601611523565b03915afa9081156115fb575f916115cd575b5090565b6115ee915060203d81116115f4575b6115e68183610e00565b810190611505565b5f6115c9565b503d6115dc565b611559565b9061161c61161633329085859192909192611569565b1561020f565b61162b5761162991611647565b565b5f631b8e828b60e31b815280611643600482016104a4565b0390fd5b9061165b611655600461113b565b1561020f565b6116905761167b61168e92611674611689935a9261169b565b5a9061115c565b611683610d67565b90611181565b611c9b565b565b6116999161169b565b565b906116a7903392611099565b906116e76116d57f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926111fa565b926116de6101e2565b918291826103e2565b0390a2565b906116f691611600565b565b5f90565b61170690516101ff565b90565b6117116116f8565b5061172561171f6004610c84565b1561020f565b611795576117616117535f61174d61174860056117426003610c63565b90610c34565b610f4d565b016116fc565b61175b61079d565b90611181565b4261177461176e836101ff565b916101ff565b10156117885761178590429061115c565b90565b506117925f610eac565b90565b61179e5f610eac565b90565b6117b06117b6919392936101ff565b926101ff565b916117c28382026101ff565b9281840414901517156117d157565b611148565b6117de6116f8565b506117f26117ec6004610c84565b1561020f565b61182c576118296118196002611813600561180d6003610c63565b90610c34565b01610c63565b6118236002610c63565b906117a1565b90565b6118355f610eac565b90565b5f90565b60018060a01b031690565b61185361185891610c4a565b61183c565b90565b6118659054611847565b90565b611870611838565b5061187a5f61185b565b90565b90565b61189461188f6118999261187d565b610661565b6101ff565b90565b6118a46116f8565b506118b86118b26004610c84565b1561020f565b6118dc576118d96118c96003610c63565b6118d36001611880565b90611181565b90565b6118e55f610eac565b90565b6118f06116f8565b506119046118fe6004610c84565b1561020f565b61192b576119286002611922600561191c6003610c63565b90610c34565b01610c63565b90565b6119345f610eac565b90565b9061194b611945600461113b565b1561020f565b6119805761196b61197e92611964611979935a92611a25565b5a9061115c565b611973610d67565b90611181565b611c9b565b565b61198991611a25565b565b5090565b600161199b91016101ff565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b903590600160200381360303821215611a00570180359067ffffffffffffffff82116119fb576020019160018202360383136119f657565b6119ba565b6119b6565b6119b2565b90821015611a20576020611a1c92028101906119be565b9091565b61199e565b611a3081839061198b565b91611a396116f8565b50611a435f610eac565b5b80611a57611a51866101ff565b916101ff565b1015611ae857611a8590611a7b333290611a7387878691611a05565b929091611569565b611a8a575b61198f565b611a44565b33611aa0611a9a86868591611a05565b90611099565b90611ae0611ace7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926111fa565b92611ad76101e2565b918291826103e2565b0390a2611a80565b50505050565b90611af891611937565b565b611b0b90611b06611d65565b611b0d565b565b80611b28611b22611b1d5f6112ed565b6104e8565b916104e8565b14611b8257611b40611b3982611353565b600161138e565b611b6a7f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b9916111fa565b90611b736101e2565b80611b7d816104a4565b0390a2565b5f632e7f3c7f60e11b815280611b9a600482016104a4565b0390fd5b611ba790611afa565b565b611bb1611d65565b611bb9611bbb565b565b611bc3611e7f565b565b611bcd611ba9565b565b611be090611bdb611d65565b611be2565b565b80611bfd611bf7611bf25f6112ed565b6104e8565b916104e8565b14611c0d57611c0b90611e20565b565b611c30611c195f6112ed565b5f918291631e4fbdf760e01b8352600483016109de565b0390fd5b611c3d90611bcf565b565b90611c4b5f199161135f565b9181191691161790565b90565b90611c6d611c68611c7492610c18565b611c55565b8254611c3f565b9055565b916020611c99929493611c9260408201965f830190610727565b0190610727565b565b611cae611ca8600461113b565b1561020f565b611d6257611cc5611cbf6004610c84565b1561020f565b611d55575b611cd261205a565b611d0681611d006002611cf06005611cea6003610c63565b90610c34565b0191611cfb83610c63565b611181565b90611c58565b611d106003610c63565b3a611d3b7f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610c18565b92611d50611d476101e2565b92839283611c78565b0390a2565b611d5d611f57565b611cca565b50565b611d6d611868565b611d86611d80611d7b612238565b6104e8565b916104e8565b03611d8d57565b611daf611d98612238565b5f91829163118cdaa760e01b8352600483016109de565b0390fd5b60081b90565b90611dc661ff0091611db3565b9181191691161790565b90611de5611de0611dec92611318565b611324565b8254611db9565b9055565b611dfb5f6004611dd0565b565b90565b90611e15611e10611e1c926111fa565b611dfd565b8254611364565b9055565b611e295f61185b565b611e33825f611e00565b90611e67611e617f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0936111fa565b916111fa565b91611e706101e2565b80611e7a816104a4565b0390a3565b611e8b60016004611dd0565b565b90611e9960ff9161135f565b9181191691161790565b90611eb8611eb3611ebf92611318565b611324565b8254611e8d565b9055565b90611ecd90610eac565b5f5260205260405f2090565b611ee3905161020f565b90565b90611f4360606003611f4994611f095f8201611f035f88016116fc565b90611c58565b611f2260018201611f1c602088016116fc565b90611c58565b611f3b60028201611f35604088016116fc565b90611c58565b019201611ed9565b90611ea3565b565b90611f5591611ee6565b565b611f6a611f646004610c84565b1561020f565b611f71575b565b611f7d60016004611ea3565b611f90611f895f610eac565b6003611c58565b611ff142611fe05f611fd7611fce5f611fc9611fc05f95611fbb611fb2610e9c565b995f8b01610ec8565b610eac565b60208801610ec8565b610eac565b60408501610ec8565b60608301610ed6565b611fec60055f90611ec3565b611f4b565b5f42906120336120217f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610eac565b9261202a6101e2565b91829182610734565b0390a2611f6f565b90565b612047906101ff565b5f1981146120555760010190565b611148565b612077612072600561206c6003610c63565b90610c34565b61203b565b426120a561209f61209a61208c5f8601610c63565b61209461079d565b90611181565b6101ff565b916101ff565b10156120af575b50565b6120d76120ce6120c05f8401610c63565b6120c861079d565b90611181565b60018301611c58565b6120e5600160038301611ea3565b6120ef6003610c63565b61211c6120fe60028401610c63565b926121165f61210f60018401610c63565b9201610c63565b9061115c565b6121467f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610c18565b9261215b6121526101e2565b92839283611c78565b0390a261217a61217361216e6003610c63565b61203e565b6003611c58565b6121e4426121ca5f6121c16121b85f6121b36121aa5f956121a561219c610e9c565b995f8b01610ec8565b610eac565b60208801610ec8565b610eac565b60408501610ec8565b60608301610ed6565b6121df60056121d96003610c63565b90610c34565b611f4b565b6121ee6003610c63565b429061222f61221d7f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610c18565b926122266101e2565b91829182610734565b0390a25f6120ac565b612240611838565b50339056608060405234601c57600e6020565b61019461002b823961019490f35b6026565b60405190565b5f80fdfe60806040526004361015610013575b61017b565b61001d5f3561002c565b637a3979dc0361000e57610142565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b60018060a01b031690565b61005890610044565b90565b6100648161004f565b0361006b57565b5f80fd5b9050359061007c8261005b565b565b5f80fd5b5f80fd5b5f80fd5b909182601f830112156100c45781359167ffffffffffffffff83116100bf5760200192600183028401116100ba57565b610086565b610082565b61007e565b91606083830312610116576100e0825f850161006f565b926100ee836020830161006f565b92604082013567ffffffffffffffff81116101115761010d920161008a565b9091565b610040565b61003c565b151590565b6101299061011b565b9052565b9190610140905f60208501940190610120565b565b34610176576101726101616101583660046100c9565b92919091610183565b610169610032565b9182918261012d565b0390f35b610038565b5f80fd5b5f90565b5050505061018f61017f565b505f9056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4a\08Wa\0\x19a\0\x14a\0\xE9V[a\x01\xB7V[a\0!a\0=V[a\"Ea\x05}\x829`\x80Q\x81a\x0B\x8E\x01Ra\"E\x90\xF3[a\0CV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0o\x90a\0GV[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x87W`@RV[a\0QV[\x90a\0\x9Fa\0\x98a\0=V[\x92\x83a\0eV[V[_\x80\xFD[\x90V[a\0\xB1\x81a\0\xA5V[\x03a\0\xB8WV[_\x80\xFD[\x90PQ\x90a\0\xC9\x82a\0\xA8V[V[\x90` \x82\x82\x03\x12a\0\xE4Wa\0\xE1\x91_\x01a\0\xBCV[\x90V[a\0\xA1V[a\x01\x07a)\x81\x808\x03\x80a\0\xFC\x81a\0\x8CV[\x92\x839\x81\x01\x90a\0\xCBV[\x90V[\x90V[\x90V[a\x01$a\x01\x1Fa\x01)\x92a\x01\nV[a\x01\rV[a\0\xA5V[\x90V[` \x91\x81R\x01\x90V[_\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01i`\x18` \x92a\x01,V[a\x01r\x81a\x015V[\x01\x90V[a\x01\x8B\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x01\\V[\x90V[\x15a\x01\x95WV[a\x01\x9Da\0=V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x01\xB3`\x04\x82\x01a\x01vV[\x03\x90\xFD[a\x01\xBFa\x02\x8FV[a\x01\xDC\x81a\x01\xD5a\x01\xCF_a\x01\x10V[\x91a\0\xA5V[\x14\x15a\x01\x8EV[`\x80RV[_\x1B\x90V[\x90a\x01\xF2_\x19\x91a\x01\xE1V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[a\x02\x13a\x02\x0Ea\x02\x18\x92a\x01\xFCV[a\x01\rV[a\0\xA5V[\x90V[\x90V[\x90a\x023a\x02.a\x02:\x92a\x01\xFFV[a\x02\x1BV[\x82Ta\x01\xE6V[\x90UV[`\x08\x1B\x90V[\x90a\x02Qa\xFF\0\x91a\x02>V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x15\x15\x90V[a\x02i\x90a\x02[V[\x90V[\x90V[\x90a\x02\x84a\x02\x7Fa\x02\x8B\x92a\x02`V[a\x02lV[\x82Ta\x02DV[\x90UV[a\x02\x97a\x03\x91V[a\x02\xA6c;\x9A\xCA\0`\x02a\x02\x1EV[a\x02\xB2`\x01`\x04a\x02oV[V[`\xA0\x1B\x90V[\x90a\x02\xC9`\xFF`\xA0\x1B\x91a\x02\xB4V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x02\xE8a\x02\xE3a\x02\xEF\x92a\x02`V[a\x02lV[\x82Ta\x02\xBAV[\x90UV[_\x01\x90V[a\x03\0a\0=V[=_\x82>=\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x03'a\x03\"a\x03,\x92a\x03\x08V[a\x01\rV[a\x03\x08V[\x90V[a\x038\x90a\x03\x13V[\x90V[a\x03D\x90a\x03/V[\x90V[\x90a\x03X`\x01\x80`\xA0\x1B\x03\x91a\x01\xE1V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x03k\x90a\x03/V[\x90V[\x90V[\x90a\x03\x86a\x03\x81a\x03\x8D\x92a\x03bV[a\x03nV[\x82Ta\x03GV[\x90UV[a\x03\x9A3a\x03\xFEV[a\x03\xA5_`\x01a\x02\xD3V[a\x03\xADa\0=V[a\x01\xBF\x81\x01\x81\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\x03\xF9Wa\x03\xD5\x82\x91a\x01\xBFa'\xC2\x849a\x02\xF3V[\x03\x90_\xF0\x80\x15a\x03\xF4Wa\x03\xEBa\x03\xF2\x91a\x03;V[`\x01a\x03qV[V[a\x02\xF8V[a\0QV[a\x04\x07\x90a\x04_V[V[a\x04\x1Da\x04\x18a\x04\"\x92a\x01\nV[a\x01\rV[a\x03\x08V[\x90V[a\x04.\x90a\x04\tV[\x90V[a\x04:\x90a\x03\x08V[\x90V[a\x04F\x90a\x041V[\x90RV[\x91\x90a\x04]\x90_` \x85\x01\x94\x01\x90a\x04=V[V[\x80a\x04za\x04ta\x04o_a\x04%V[a\x041V[\x91a\x041V[\x14a\x04\x8AWa\x04\x88\x90a\x05\x1DV[V[a\x04\xADa\x04\x96_a\x04%V[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x04JV[\x03\x90\xFD[_\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\xCDa\x04\xD2\x91a\x04\xB1V[a\x04\xB6V[\x90V[a\x04\xDF\x90Ta\x04\xC1V[\x90V[a\x04\xEB\x90a\x03\x13V[\x90V[a\x04\xF7\x90a\x04\xE2V[\x90V[\x90V[\x90a\x05\x12a\x05\ra\x05\x19\x92a\x04\xEEV[a\x04\xFAV[\x82Ta\x03GV[\x90UV[a\x05&_a\x04\xD5V[a\x050\x82_a\x04\xFDV[\x90a\x05da\x05^\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x04\xEEV[\x91a\x04\xEEV[\x91a\x05ma\0=V[\x80a\x05w\x81a\x02\xF3V[\x03\x90\xA3V\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\r\xE8V[a\0\x1D_5a\x01\xDCV[\x80c\x08aF\xD2\x14a\x01\xD7W\x80c\x18\xD5\xAA\xFE\x14a\x01\xD2W\x80c6l\xBA\xB7\x14a\x01\xCDW\x80c;j\xB2\xA9\x14a\x01\xC8W\x80cF\xE2\xCC\t\x14a\x01\xC3W\x80cH\\\xC9U\x14a\x01\xBEW\x80cK,\x07\x06\x14a\x01\xB9W\x80cTg\xCBH\x14a\x01\xB4W\x80c[<\xD6\xE2\x14a\x01\xAFW\x80caT8\x01\x14a\x01\xAAW\x80ceX\x95O\x14a\x01\xA5W\x80cp<\xFC\xBB\x14a\x01\xA0W\x80cqP\x18\xA6\x14a\x01\x9BW\x80cz9y\xDC\x14a\x01\x96W\x80c\x80NQ#\x14a\x01\x91W\x80c\x82\xF4J\xDE\x14a\x01\x8CW\x80c\x84\xFA\xB6+\x14a\x01\x87W\x80c\x8DZ#\x9B\x14a\x01\x82W\x80c\x8D\xA5\xCB[\x14a\x01}W\x80c\xAF\xF7Lm\x14a\x01xW\x80c\xC6`\xD3\xF3\x14a\x01sW\x80c\xCD\xAF\xB9x\x14a\x01nW\x80c\xD4\xF0\xEBM\x14a\x01iW\x80c\xD8x\x13B\x14a\x01dW\x80c\xDE\x1FE>\x14a\x01_W\x80c\xEAJ\x11\x04\x14a\x01ZW\x80c\xED\xE0{\xD6\x14a\x01UWc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\r\xB5V[a\r\x80V[a\r\x0FV[a\x0B\xE5V[a\x0B\xB0V[a\x0BYV[a\x0B\x07V[a\n]V[a\n(V[a\t\xF3V[a\t\x9CV[a\tgV[a\t\"V[a\x08\xEEV[a\x08\xB5V[a\x080V[a\x07\xFBV[a\x07\xB7V[a\x07IV[a\x06\xBAV[a\x05\xEEV[a\x05\xB9V[a\x05DV[a\x04\xA9V[a\x04oV[a\x03\xFAV[a\x02\xD5V[a\x02~V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01\xFAWV[a\x01\xECV[\x90V[a\x02\x0B\x90a\x01\xFFV[\x90RV[\x15\x15\x90V[a\x02\x1D\x90a\x02\x0FV[\x90RV[\x90``\x80a\x02g\x93a\x029_\x82\x01Q_\x86\x01\x90a\x02\x02V[a\x02K` \x82\x01Q` \x86\x01\x90a\x02\x02V[a\x02]`@\x82\x01Q`@\x86\x01\x90a\x02\x02V[\x01Q\x91\x01\x90a\x02\x14V[V[\x91\x90a\x02|\x90_`\x80\x85\x01\x94\x01\x90a\x02!V[V[4a\x02\xAEWa\x02\x8E6`\x04a\x01\xF0V[a\x02\xAAa\x02\x99a\x0FYV[a\x02\xA1a\x01\xE2V[\x91\x82\x91\x82a\x02iV[\x03\x90\xF3[a\x01\xE8V[a\x02\xBC\x90a\x02\x0FV[\x90RV[\x91\x90a\x02\xD3\x90_` \x85\x01\x94\x01\x90a\x02\xB3V[V[4a\x03\x05Wa\x02\xE56`\x04a\x01\xF0V[a\x03\x01a\x02\xF0a\x0F\xF5V[a\x02\xF8a\x01\xE2V[\x91\x82\x91\x82a\x02\xC0V[\x03\x90\xF3[a\x01\xE8V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03TW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03OW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x03JWV[a\x03\x16V[a\x03\x12V[a\x03\x0EV[\x90` \x82\x82\x03\x12a\x03\x8AW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\x85Wa\x03\x81\x92\x01a\x03\x1AV[\x90\x91V[a\x03\nV[a\x01\xECV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\xD0a\x03\xD9` \x93a\x03\xDE\x93a\x03\xC7\x81a\x03\x8FV[\x93\x84\x80\x93a\x03\x93V[\x95\x86\x91\x01a\x03\x9CV[a\x03\xA7V[\x01\x90V[a\x03\xF7\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xB1V[\x90V[4a\x04+Wa\x04'a\x04\x16a\x04\x106`\x04a\x03YV[\x90a\x10\x99V[a\x04\x1Ea\x01\xE2V[\x91\x82\x91\x82a\x03\xE2V[\x03\x90\xF3[a\x01\xE8V[\x1C\x90V[`\xFF\x16\x90V[a\x04J\x90`\x08a\x04O\x93\x02a\x040V[a\x044V[\x90V[\x90a\x04]\x91Ta\x04:V[\x90V[a\x04l`\x04_\x90a\x04RV[\x90V[4a\x04\x9FWa\x04\x7F6`\x04a\x01\xF0V[a\x04\x9Ba\x04\x8Aa\x04`V[a\x04\x92a\x01\xE2V[\x91\x82\x91\x82a\x02\xC0V[\x03\x90\xF3[a\x01\xE8V[_\x01\x90V[4a\x04\xD8Wa\x04\xC2a\x04\xBC6`\x04a\x03YV[\x90a\x12\x8AV[a\x04\xCAa\x01\xE2V[\x80a\x04\xD4\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\xF1\x90a\x04\xDDV[\x90V[a\x04\xFD\x81a\x04\xE8V[\x03a\x05\x04WV[_\x80\xFD[\x90P5\x90a\x05\x15\x82a\x04\xF4V[V[\x91\x90`@\x83\x82\x03\x12a\x05?W\x80a\x053a\x05<\x92_\x86\x01a\x05\x08V[\x93` \x01a\x05\x08V[\x90V[a\x01\xECV[4a\x05sWa\x05]a\x05W6`\x04a\x05\x17V[\x90a\x14;V[a\x05ea\x01\xE2V[\x80a\x05o\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[a\x05\x81\x81a\x01\xFFV[\x03a\x05\x88WV[_\x80\xFD[\x90P5\x90a\x05\x99\x82a\x05xV[V[\x90` \x82\x82\x03\x12a\x05\xB4Wa\x05\xB1\x91_\x01a\x05\x8CV[\x90V[a\x01\xECV[4a\x05\xE9Wa\x05\xE5a\x05\xD4a\x05\xCF6`\x04a\x05\x9BV[a\x14GV[a\x05\xDCa\x01\xE2V[\x91\x82\x91\x82a\x02iV[\x03\x90\xF3[a\x01\xE8V[4a\x06\x1CWa\x05\xFE6`\x04a\x01\xF0V[a\x06\x06a\x14\x82V[a\x06\x0Ea\x01\xE2V[\x80a\x06\x18\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x06<\x90`\x08a\x06A\x93\x02a\x040V[a\x06!V[\x90V[\x90a\x06O\x91Ta\x06,V[\x90V[a\x06^`\x01_\x90a\x06DV[\x90V[\x90V[a\x06xa\x06sa\x06}\x92a\x04\xDDV[a\x06aV[a\x04\xDDV[\x90V[a\x06\x89\x90a\x06dV[\x90V[a\x06\x95\x90a\x06\x80V[\x90V[a\x06\xA1\x90a\x06\x8CV[\x90RV[\x91\x90a\x06\xB8\x90_` \x85\x01\x94\x01\x90a\x06\x98V[V[4a\x06\xEAWa\x06\xCA6`\x04a\x01\xF0V[a\x06\xE6a\x06\xD5a\x06RV[a\x06\xDDa\x01\xE2V[\x91\x82\x91\x82a\x06\xA5V[\x03\x90\xF3[a\x01\xE8V[\x90V[a\x07\x02\x90`\x08a\x07\x07\x93\x02a\x040V[a\x06\xEFV[\x90V[\x90a\x07\x15\x91Ta\x06\xF2V[\x90V[a\x07$`\x03_\x90a\x07\nV[\x90V[a\x070\x90a\x01\xFFV[\x90RV[\x91\x90a\x07G\x90_` \x85\x01\x94\x01\x90a\x07'V[V[4a\x07yWa\x07Y6`\x04a\x01\xF0V[a\x07ua\x07da\x07\x18V[a\x07la\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[\x90V[a\x07\x95a\x07\x90a\x07\x9A\x92a\x07~V[a\x06aV[a\x01\xFFV[\x90V[a\x07\xA9b'\x8D\0a\x07\x81V[\x90V[a\x07\xB4a\x07\x9DV[\x90V[4a\x07\xE7Wa\x07\xC76`\x04a\x01\xF0V[a\x07\xE3a\x07\xD2a\x07\xACV[a\x07\xDAa\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[a\x07\xF8`\x02_\x90a\x07\nV[\x90V[4a\x08+Wa\x08\x0B6`\x04a\x01\xF0V[a\x08'a\x08\x16a\x07\xECV[a\x08\x1Ea\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[4a\x08^Wa\x08@6`\x04a\x01\xF0V[a\x08Ha\x14\xB1V[a\x08Pa\x01\xE2V[\x80a\x08Z\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[\x91``\x83\x83\x03\x12a\x08\xB0Wa\x08z\x82_\x85\x01a\x05\x08V[\x92a\x08\x88\x83` \x83\x01a\x05\x08V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08\xABWa\x08\xA7\x92\x01a\x03\x1AV[\x90\x91V[a\x03\nV[a\x01\xECV[4a\x08\xE9Wa\x08\xE5a\x08\xD4a\x08\xCB6`\x04a\x08cV[\x92\x91\x90\x91a\x15iV[a\x08\xDCa\x01\xE2V[\x91\x82\x91\x82a\x02\xC0V[\x03\x90\xF3[a\x01\xE8V[4a\t\x1DWa\t\x07a\t\x016`\x04a\x03YV[\x90a\x16\xECV[a\t\x0Fa\x01\xE2V[\x80a\t\x19\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[4a\tRWa\t26`\x04a\x01\xF0V[a\tNa\t=a\x17\tV[a\tEa\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[a\td`\x04`\x01\x90a\x04RV[\x90V[4a\t\x97Wa\tw6`\x04a\x01\xF0V[a\t\x93a\t\x82a\tWV[a\t\x8Aa\x01\xE2V[\x91\x82\x91\x82a\x02\xC0V[\x03\x90\xF3[a\x01\xE8V[4a\t\xCCWa\t\xAC6`\x04a\x01\xF0V[a\t\xC8a\t\xB7a\x17\xD6V[a\t\xBFa\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[a\t\xDA\x90a\x04\xE8V[\x90RV[\x91\x90a\t\xF1\x90_` \x85\x01\x94\x01\x90a\t\xD1V[V[4a\n#Wa\n\x036`\x04a\x01\xF0V[a\n\x1Fa\n\x0Ea\x18hV[a\n\x16a\x01\xE2V[\x91\x82\x91\x82a\t\xDEV[\x03\x90\xF3[a\x01\xE8V[4a\nXWa\n86`\x04a\x01\xF0V[a\nTa\nCa\x18\x9CV[a\nKa\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[4a\n\x8DWa\nm6`\x04a\x01\xF0V[a\n\x89a\nxa\x18\xE8V[a\n\x80a\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\n\xCCW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\xC7W` \x01\x92` \x83\x02\x84\x01\x11a\n\xC2WV[a\x03\x16V[a\x03\x12V[a\x03\x0EV[\x90` \x82\x82\x03\x12a\x0B\x02W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xFDWa\n\xF9\x92\x01a\n\x92V[\x90\x91V[a\x03\nV[a\x01\xECV[4a\x0B6Wa\x0B a\x0B\x1A6`\x04a\n\xD1V[\x90a\x1A\xEEV[a\x0B(a\x01\xE2V[\x80a\x0B2\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[\x90` \x82\x82\x03\x12a\x0BTWa\x0BQ\x91_\x01a\x05\x08V[\x90V[a\x01\xECV[4a\x0B\x87Wa\x0Bqa\x0Bl6`\x04a\x0B;V[a\x1B\x9EV[a\x0Bya\x01\xE2V[\x80a\x0B\x83\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B\xE0Wa\x0B\xC06`\x04a\x01\xF0V[a\x0B\xDCa\x0B\xCBa\x0B\x8CV[a\x0B\xD3a\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[4a\x0C\x13Wa\x0B\xF56`\x04a\x01\xF0V[a\x0B\xFDa\x1B\xC5V[a\x0C\x05a\x01\xE2V[\x80a\x0C\x0F\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[a\x0C,a\x0C'a\x0C1\x92a\x01\xFFV[a\x06aV[a\x01\xFFV[\x90V[\x90a\x0C>\x90a\x0C\x18V[_R` R`@_ \x90V[_\x1C\x90V[a\x0C[a\x0C`\x91a\x0CJV[a\x06\xEFV[\x90V[a\x0Cm\x90Ta\x0COV[\x90V[a\x0C|a\x0C\x81\x91a\x0CJV[a\x044V[\x90V[a\x0C\x8E\x90Ta\x0CpV[\x90V[a\x0C\x9C\x90`\x05a\x0C4V[\x90a\x0C\xA8_\x83\x01a\x0CcV[\x91a\x0C\xB5`\x01\x82\x01a\x0CcV[\x91a\x0C\xCE`\x03a\x0C\xC7`\x02\x85\x01a\x0CcV[\x93\x01a\x0C\x84V[\x90V[a\r\x06a\r\r\x94a\x0C\xFC``\x94\x98\x97\x95a\x0C\xF2`\x80\x86\x01\x9A_\x87\x01\x90a\x07'V[` \x85\x01\x90a\x07'V[`@\x83\x01\x90a\x07'V[\x01\x90a\x02\xB3V[V[4a\rCWa\r?a\r*a\r%6`\x04a\x05\x9BV[a\x0C\x91V[\x90a\r6\x94\x92\x94a\x01\xE2V[\x94\x85\x94\x85a\x0C\xD1V[\x03\x90\xF3[a\x01\xE8V[\x90V[a\r_a\rZa\rd\x92a\rHV[a\x06aV[a\x01\xFFV[\x90V[a\rra\x13\x88a\rKV[\x90V[a\r}a\rgV[\x90V[4a\r\xB0Wa\r\x906`\x04a\x01\xF0V[a\r\xACa\r\x9Ba\ruV[a\r\xA3a\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[4a\r\xE3Wa\r\xCDa\r\xC86`\x04a\x0B;V[a\x1C4V[a\r\xD5a\x01\xE2V[\x80a\r\xDF\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x0E\n\x90a\x03\xA7V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E$W`@RV[a\r\xECV[\x90a\x0E<a\x0E5a\x01\xE2V[\x92\x83a\x0E\0V[V[a\x0EH`\x80a\x0E)V[\x90V[_\x90V[_\x90V[a\x0E[a\x0E>V[\x90` \x80\x80\x80\x85a\x0Eja\x0EKV[\x81R\x01a\x0Eua\x0EKV[\x81R\x01a\x0E\x80a\x0EKV[\x81R\x01a\x0E\x8Ba\x0EOV[\x81RPPV[a\x0E\x99a\x0ESV[\x90V[a\x0E\xA6`\x80a\x0E)V[\x90V[\x90V[a\x0E\xC0a\x0E\xBBa\x0E\xC5\x92a\x0E\xA9V[a\x06aV[a\x01\xFFV[\x90V[\x90a\x0E\xD2\x90a\x01\xFFV[\x90RV[\x90a\x0E\xE0\x90a\x02\x0FV[\x90RV[\x90a\x0FKa\x0FB`\x03a\x0E\xF5a\x0E>V[\x94a\x0F\x0Ca\x0F\x04_\x83\x01a\x0CcV[_\x88\x01a\x0E\xC8V[a\x0F$a\x0F\x1B`\x01\x83\x01a\x0CcV[` \x88\x01a\x0E\xC8V[a\x0F<a\x0F3`\x02\x83\x01a\x0CcV[`@\x88\x01a\x0E\xC8V[\x01a\x0C\x84V[``\x84\x01a\x0E\xD6V[V[a\x0FV\x90a\x0E\xE4V[\x90V[a\x0Faa\x0E\x91V[Pa\x0Fua\x0Fo`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x0F\x99Wa\x0F\x96a\x0F\x91`\x05a\x0F\x8B`\x03a\x0CcV[\x90a\x0C4V[a\x0FMV[\x90V[_a\x0F\xEE_a\x0F\xE5a\x0F\xDC_a\x0F\xD7a\x0F\xCE_\x95a\x0F\xC9a\x0F\xC1a\x0F\xBBa\x0E\x9CV[\x9Aa\x0E\xACV[_\x8B\x01a\x0E\xC8V[a\x0E\xACV[` \x88\x01a\x0E\xC8V[a\x0E\xACV[`@\x85\x01a\x0E\xC8V[``\x83\x01a\x0E\xD6V[\x90V[_\x90V[a\x0F\xFDa\x0F\xF1V[Pa\x10\x08`\x04a\x0C\x84V[\x90V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x103a\x10.a\x108\x92a\x0E\xA9V[a\x10\x19V[a\x10\x10V[\x90V[\x90V[a\x10Ja\x10O\x91a\x10\x10V[a\x10;V[\x90RV[\x90P\x90V[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x10s\x81a\x10z\x93a\x10SV[\x80\x93a\x10XV[\x01\x90V[\x80a\x10\x8F`\x01\x92a\x10\x96\x96\x94a\x10>V[\x01\x91a\x10cV[\x90V[a\x10\xD7\x90a\x10\xA5a\x10\x0BV[Pa\x10\xC8a\x10\xB2_a\x10\x1FV[\x91\x93a\x10\xBCa\x01\xE2V[\x94\x85\x93` \x85\x01a\x10~V[` \x82\x01\x81\x03\x82R\x03\x82a\x0E\0V[\x90V[\x90a\x10\xF6a\x10\xF032\x90\x85\x85\x91\x92\x90\x91\x92a\x15iV[\x15a\x02\x0FV[a\x11\x05Wa\x11\x03\x91a\x11\xA6V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x11\x1D`\x04\x82\x01a\x04\xA4V[\x03\x90\xFD[`\x08\x1C\x90V[a\x113a\x118\x91a\x11!V[a\x044V[\x90V[a\x11E\x90Ta\x11'V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x11ka\x11q\x91\x93\x92\x93a\x01\xFFV[\x92a\x01\xFFV[\x82\x03\x91\x82\x11a\x11|WV[a\x11HV[a\x11\x90a\x11\x96\x91\x93\x92\x93a\x01\xFFV[\x92a\x01\xFFV[\x82\x01\x80\x92\x11a\x11\xA1WV[a\x11HV[\x90a\x11\xBAa\x11\xB4`\x04a\x11;V[\x15a\x02\x0FV[a\x11\xEFWa\x11\xDAa\x11\xED\x92a\x11\xD3a\x11\xE8\x93Z\x92a\x12CV[Z\x90a\x11\\V[a\x11\xE2a\rgV[\x90a\x11\x81V[a\x1C\x9BV[V[a\x11\xF8\x91a\x12CV[V[a\x12\x03\x90a\x06\x80V[\x90V[\x91\x90a\x12 \x81a\x12\x19\x81a\x12%\x95a\x03\x93V[\x80\x95a\x10XV[a\x03\xA7V[\x01\x90V[\x90\x91a\x12@\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x12\x06V[\x90V[3\x90\x91a\x12p\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11\xFAV[\x92a\x12\x85a\x12|a\x01\xE2V[\x92\x83\x92\x83a\x12)V[\x03\x90\xA2V[\x90a\x12\x94\x91a\x10\xDAV[V[\x90a\x12\xA8\x91a\x12\xA3a\x1DeV[a\x13\xAEV[V[`\xA0\x1C\x90V[a\x12\xBCa\x12\xC1\x91a\x12\xAAV[a\x044V[\x90V[a\x12\xCE\x90Ta\x12\xB0V[\x90V[a\x12\xE5a\x12\xE0a\x12\xEA\x92a\x0E\xA9V[a\x06aV[a\x04\xDDV[\x90V[a\x12\xF6\x90a\x12\xD1V[\x90V[`\xA0\x1B\x90V[\x90a\x13\x0E`\xFF`\xA0\x1B\x91a\x12\xF9V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x13!\x90a\x02\x0FV[\x90V[\x90V[\x90a\x13<a\x137a\x13C\x92a\x13\x18V[a\x13$V[\x82Ta\x12\xFFV[\x90UV[a\x13P\x90a\x06dV[\x90V[a\x13\\\x90a\x13GV[\x90V[_\x1B\x90V[\x90a\x13u`\x01\x80`\xA0\x1B\x03\x91a\x13_V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x13\x88\x90a\x13GV[\x90V[\x90V[\x90a\x13\xA3a\x13\x9Ea\x13\xAA\x92a\x13\x7FV[a\x13\x8BV[\x82Ta\x13dV[\x90UV[a\x13\xB8`\x01a\x12\xC4V[a\x14 W\x81a\x13\xD7a\x13\xD1a\x13\xCC_a\x12\xEDV[a\x04\xE8V[\x91a\x04\xE8V[\x14a\x14\x04Wa\x13\xFDa\x13\xF6a\x14\x02\x93a\x13\xF1`\x01\x80a\x13'V[a\x13SV[`\x01a\x13\x8EV[a\x1C4V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x14\x1C`\x04\x82\x01a\x04\xA4V[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x147`\x04\x82\x01a\x04\xA4V[\x03\x90\xFD[\x90a\x14E\x91a\x12\x96V[V[a\x14^a\x14c\x91a\x14Va\x0E\x91V[P`\x05a\x0C4V[a\x0FMV[\x90V[a\x14na\x1DeV[a\x14va\x14xV[V[a\x14\x80a\x1D\xF0V[V[a\x14\x8Aa\x14fV[V[a\x14\x94a\x1DeV[a\x14\x9Ca\x14\x9EV[V[a\x14\xAFa\x14\xAA_a\x12\xEDV[a\x1E V[V[a\x14\xB9a\x14\x8CV[V[a\x14\xC7a\x14\xCC\x91a\x0CJV[a\x06!V[\x90V[a\x14\xD9\x90Ta\x14\xBBV[\x90V[`\xE0\x1B\x90V[a\x14\xEB\x81a\x02\x0FV[\x03a\x14\xF2WV[_\x80\xFD[\x90PQ\x90a\x15\x03\x82a\x14\xE2V[V[\x90` \x82\x82\x03\x12a\x15\x1EWa\x15\x1B\x91_\x01a\x14\xF6V[\x90V[a\x01\xECV[a\x15Ia\x15V\x95\x93\x94\x92\x94a\x15?``\x84\x01\x96_\x85\x01\x90a\t\xD1V[` \x83\x01\x90a\t\xD1V[`@\x81\x85\x03\x91\x01Ra\x12\x06V[\x90V[a\x15aa\x01\xE2V[=_\x82>=\x90\xFD[\x92a\x15\xAC` \x93\x94a\x15ya\x0F\xF1V[Pa\x15\xB7a\x15\x8Fa\x15\x8A`\x01a\x14\xCFV[a\x06\x8CV[\x93cz9y\xDC\x92\x95\x97a\x15\xA0a\x01\xE2V[\x98\x89\x97\x88\x96\x87\x96a\x14\xDCV[\x86R`\x04\x86\x01a\x15#V[\x03\x91Z\xFA\x90\x81\x15a\x15\xFBW_\x91a\x15\xCDW[P\x90V[a\x15\xEE\x91P` =\x81\x11a\x15\xF4W[a\x15\xE6\x81\x83a\x0E\0V[\x81\x01\x90a\x15\x05V[_a\x15\xC9V[P=a\x15\xDCV[a\x15YV[\x90a\x16\x1Ca\x16\x1632\x90\x85\x85\x91\x92\x90\x91\x92a\x15iV[\x15a\x02\x0FV[a\x16+Wa\x16)\x91a\x16GV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x16C`\x04\x82\x01a\x04\xA4V[\x03\x90\xFD[\x90a\x16[a\x16U`\x04a\x11;V[\x15a\x02\x0FV[a\x16\x90Wa\x16{a\x16\x8E\x92a\x16ta\x16\x89\x93Z\x92a\x16\x9BV[Z\x90a\x11\\V[a\x16\x83a\rgV[\x90a\x11\x81V[a\x1C\x9BV[V[a\x16\x99\x91a\x16\x9BV[V[\x90a\x16\xA7\x903\x92a\x10\x99V[\x90a\x16\xE7a\x16\xD5\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11\xFAV[\x92a\x16\xDEa\x01\xE2V[\x91\x82\x91\x82a\x03\xE2V[\x03\x90\xA2V[\x90a\x16\xF6\x91a\x16\0V[V[_\x90V[a\x17\x06\x90Qa\x01\xFFV[\x90V[a\x17\x11a\x16\xF8V[Pa\x17%a\x17\x1F`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x17\x95Wa\x17aa\x17S_a\x17Ma\x17H`\x05a\x17B`\x03a\x0CcV[\x90a\x0C4V[a\x0FMV[\x01a\x16\xFCV[a\x17[a\x07\x9DV[\x90a\x11\x81V[Ba\x17ta\x17n\x83a\x01\xFFV[\x91a\x01\xFFV[\x10\x15a\x17\x88Wa\x17\x85\x90B\x90a\x11\\V[\x90V[Pa\x17\x92_a\x0E\xACV[\x90V[a\x17\x9E_a\x0E\xACV[\x90V[a\x17\xB0a\x17\xB6\x91\x93\x92\x93a\x01\xFFV[\x92a\x01\xFFV[\x91a\x17\xC2\x83\x82\x02a\x01\xFFV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x17\xD1WV[a\x11HV[a\x17\xDEa\x16\xF8V[Pa\x17\xF2a\x17\xEC`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x18,Wa\x18)a\x18\x19`\x02a\x18\x13`\x05a\x18\r`\x03a\x0CcV[\x90a\x0C4V[\x01a\x0CcV[a\x18#`\x02a\x0CcV[\x90a\x17\xA1V[\x90V[a\x185_a\x0E\xACV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x18Sa\x18X\x91a\x0CJV[a\x18<V[\x90V[a\x18e\x90Ta\x18GV[\x90V[a\x18pa\x188V[Pa\x18z_a\x18[V[\x90V[\x90V[a\x18\x94a\x18\x8Fa\x18\x99\x92a\x18}V[a\x06aV[a\x01\xFFV[\x90V[a\x18\xA4a\x16\xF8V[Pa\x18\xB8a\x18\xB2`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x18\xDCWa\x18\xD9a\x18\xC9`\x03a\x0CcV[a\x18\xD3`\x01a\x18\x80V[\x90a\x11\x81V[\x90V[a\x18\xE5_a\x0E\xACV[\x90V[a\x18\xF0a\x16\xF8V[Pa\x19\x04a\x18\xFE`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x19+Wa\x19(`\x02a\x19\"`\x05a\x19\x1C`\x03a\x0CcV[\x90a\x0C4V[\x01a\x0CcV[\x90V[a\x194_a\x0E\xACV[\x90V[\x90a\x19Ka\x19E`\x04a\x11;V[\x15a\x02\x0FV[a\x19\x80Wa\x19ka\x19~\x92a\x19da\x19y\x93Z\x92a\x1A%V[Z\x90a\x11\\V[a\x19sa\rgV[\x90a\x11\x81V[a\x1C\x9BV[V[a\x19\x89\x91a\x1A%V[V[P\x90V[`\x01a\x19\x9B\x91\x01a\x01\xFFV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x1A\0W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x19\xFBW` \x01\x91`\x01\x82\x026\x03\x83\x13a\x19\xF6WV[a\x19\xBAV[a\x19\xB6V[a\x19\xB2V[\x90\x82\x10\x15a\x1A W` a\x1A\x1C\x92\x02\x81\x01\x90a\x19\xBEV[\x90\x91V[a\x19\x9EV[a\x1A0\x81\x83\x90a\x19\x8BV[\x91a\x1A9a\x16\xF8V[Pa\x1AC_a\x0E\xACV[[\x80a\x1AWa\x1AQ\x86a\x01\xFFV[\x91a\x01\xFFV[\x10\x15a\x1A\xE8Wa\x1A\x85\x90a\x1A{32\x90a\x1As\x87\x87\x86\x91a\x1A\x05V[\x92\x90\x91a\x15iV[a\x1A\x8AW[a\x19\x8FV[a\x1ADV[3a\x1A\xA0a\x1A\x9A\x86\x86\x85\x91a\x1A\x05V[\x90a\x10\x99V[\x90a\x1A\xE0a\x1A\xCE\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11\xFAV[\x92a\x1A\xD7a\x01\xE2V[\x91\x82\x91\x82a\x03\xE2V[\x03\x90\xA2a\x1A\x80V[PPPPV[\x90a\x1A\xF8\x91a\x197V[V[a\x1B\x0B\x90a\x1B\x06a\x1DeV[a\x1B\rV[V[\x80a\x1B(a\x1B\"a\x1B\x1D_a\x12\xEDV[a\x04\xE8V[\x91a\x04\xE8V[\x14a\x1B\x82Wa\x1B@a\x1B9\x82a\x13SV[`\x01a\x13\x8EV[a\x1Bj\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x11\xFAV[\x90a\x1Bsa\x01\xE2V[\x80a\x1B}\x81a\x04\xA4V[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x1B\x9A`\x04\x82\x01a\x04\xA4V[\x03\x90\xFD[a\x1B\xA7\x90a\x1A\xFAV[V[a\x1B\xB1a\x1DeV[a\x1B\xB9a\x1B\xBBV[V[a\x1B\xC3a\x1E\x7FV[V[a\x1B\xCDa\x1B\xA9V[V[a\x1B\xE0\x90a\x1B\xDBa\x1DeV[a\x1B\xE2V[V[\x80a\x1B\xFDa\x1B\xF7a\x1B\xF2_a\x12\xEDV[a\x04\xE8V[\x91a\x04\xE8V[\x14a\x1C\rWa\x1C\x0B\x90a\x1E V[V[a\x1C0a\x1C\x19_a\x12\xEDV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\t\xDEV[\x03\x90\xFD[a\x1C=\x90a\x1B\xCFV[V[\x90a\x1CK_\x19\x91a\x13_V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x1Cma\x1Cha\x1Ct\x92a\x0C\x18V[a\x1CUV[\x82Ta\x1C?V[\x90UV[\x91` a\x1C\x99\x92\x94\x93a\x1C\x92`@\x82\x01\x96_\x83\x01\x90a\x07'V[\x01\x90a\x07'V[V[a\x1C\xAEa\x1C\xA8`\x04a\x11;V[\x15a\x02\x0FV[a\x1DbWa\x1C\xC5a\x1C\xBF`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x1DUW[a\x1C\xD2a ZV[a\x1D\x06\x81a\x1D\0`\x02a\x1C\xF0`\x05a\x1C\xEA`\x03a\x0CcV[\x90a\x0C4V[\x01\x91a\x1C\xFB\x83a\x0CcV[a\x11\x81V[\x90a\x1CXV[a\x1D\x10`\x03a\x0CcV[:a\x1D;\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0C\x18V[\x92a\x1DPa\x1DGa\x01\xE2V[\x92\x83\x92\x83a\x1CxV[\x03\x90\xA2V[a\x1D]a\x1FWV[a\x1C\xCAV[PV[a\x1Dma\x18hV[a\x1D\x86a\x1D\x80a\x1D{a\"8V[a\x04\xE8V[\x91a\x04\xE8V[\x03a\x1D\x8DWV[a\x1D\xAFa\x1D\x98a\"8V[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\t\xDEV[\x03\x90\xFD[`\x08\x1B\x90V[\x90a\x1D\xC6a\xFF\0\x91a\x1D\xB3V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1D\xE5a\x1D\xE0a\x1D\xEC\x92a\x13\x18V[a\x13$V[\x82Ta\x1D\xB9V[\x90UV[a\x1D\xFB_`\x04a\x1D\xD0V[V[\x90V[\x90a\x1E\x15a\x1E\x10a\x1E\x1C\x92a\x11\xFAV[a\x1D\xFDV[\x82Ta\x13dV[\x90UV[a\x1E)_a\x18[V[a\x1E3\x82_a\x1E\0V[\x90a\x1Ega\x1Ea\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x11\xFAV[\x91a\x11\xFAV[\x91a\x1Epa\x01\xE2V[\x80a\x1Ez\x81a\x04\xA4V[\x03\x90\xA3V[a\x1E\x8B`\x01`\x04a\x1D\xD0V[V[\x90a\x1E\x99`\xFF\x91a\x13_V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1E\xB8a\x1E\xB3a\x1E\xBF\x92a\x13\x18V[a\x13$V[\x82Ta\x1E\x8DV[\x90UV[\x90a\x1E\xCD\x90a\x0E\xACV[_R` R`@_ \x90V[a\x1E\xE3\x90Qa\x02\x0FV[\x90V[\x90a\x1FC```\x03a\x1FI\x94a\x1F\t_\x82\x01a\x1F\x03_\x88\x01a\x16\xFCV[\x90a\x1CXV[a\x1F\"`\x01\x82\x01a\x1F\x1C` \x88\x01a\x16\xFCV[\x90a\x1CXV[a\x1F;`\x02\x82\x01a\x1F5`@\x88\x01a\x16\xFCV[\x90a\x1CXV[\x01\x92\x01a\x1E\xD9V[\x90a\x1E\xA3V[V[\x90a\x1FU\x91a\x1E\xE6V[V[a\x1Fja\x1Fd`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x1FqW[V[a\x1F}`\x01`\x04a\x1E\xA3V[a\x1F\x90a\x1F\x89_a\x0E\xACV[`\x03a\x1CXV[a\x1F\xF1Ba\x1F\xE0_a\x1F\xD7a\x1F\xCE_a\x1F\xC9a\x1F\xC0_\x95a\x1F\xBBa\x1F\xB2a\x0E\x9CV[\x99_\x8B\x01a\x0E\xC8V[a\x0E\xACV[` \x88\x01a\x0E\xC8V[a\x0E\xACV[`@\x85\x01a\x0E\xC8V[``\x83\x01a\x0E\xD6V[a\x1F\xEC`\x05_\x90a\x1E\xC3V[a\x1FKV[_B\x90a 3a !\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0E\xACV[\x92a *a\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xA2a\x1FoV[\x90V[a G\x90a\x01\xFFV[_\x19\x81\x14a UW`\x01\x01\x90V[a\x11HV[a wa r`\x05a l`\x03a\x0CcV[\x90a\x0C4V[a ;V[Ba \xA5a \x9Fa \x9Aa \x8C_\x86\x01a\x0CcV[a \x94a\x07\x9DV[\x90a\x11\x81V[a\x01\xFFV[\x91a\x01\xFFV[\x10\x15a \xAFW[PV[a \xD7a \xCEa \xC0_\x84\x01a\x0CcV[a \xC8a\x07\x9DV[\x90a\x11\x81V[`\x01\x83\x01a\x1CXV[a \xE5`\x01`\x03\x83\x01a\x1E\xA3V[a \xEF`\x03a\x0CcV[a!\x1Ca \xFE`\x02\x84\x01a\x0CcV[\x92a!\x16_a!\x0F`\x01\x84\x01a\x0CcV[\x92\x01a\x0CcV[\x90a\x11\\V[a!F\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0C\x18V[\x92a![a!Ra\x01\xE2V[\x92\x83\x92\x83a\x1CxV[\x03\x90\xA2a!za!sa!n`\x03a\x0CcV[a >V[`\x03a\x1CXV[a!\xE4Ba!\xCA_a!\xC1a!\xB8_a!\xB3a!\xAA_\x95a!\xA5a!\x9Ca\x0E\x9CV[\x99_\x8B\x01a\x0E\xC8V[a\x0E\xACV[` \x88\x01a\x0E\xC8V[a\x0E\xACV[`@\x85\x01a\x0E\xC8V[``\x83\x01a\x0E\xD6V[a!\xDF`\x05a!\xD9`\x03a\x0CcV[\x90a\x0C4V[a\x1FKV[a!\xEE`\x03a\x0CcV[B\x90a\"/a\"\x1D\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0C\x18V[\x92a\"&a\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xA2_a \xACV[a\"@a\x188V[P3\x90V`\x80`@R4`\x1CW`\x0E` V[a\x01\x94a\0+\x829a\x01\x94\x90\xF3[`&V[`@Q\x90V[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x01{V[a\0\x1D_5a\0,V[cz9y\xDC\x03a\0\x0EWa\x01BV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\0X\x90a\0DV[\x90V[a\0d\x81a\0OV[\x03a\0kWV[_\x80\xFD[\x90P5\x90a\0|\x82a\0[V[V[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\0\xC4W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xBFW` \x01\x92`\x01\x83\x02\x84\x01\x11a\0\xBAWV[a\0\x86V[a\0\x82V[a\0~V[\x91``\x83\x83\x03\x12a\x01\x16Wa\0\xE0\x82_\x85\x01a\0oV[\x92a\0\xEE\x83` \x83\x01a\0oV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x11Wa\x01\r\x92\x01a\0\x8AV[\x90\x91V[a\0@V[a\0<V[\x15\x15\x90V[a\x01)\x90a\x01\x1BV[\x90RV[\x91\x90a\x01@\x90_` \x85\x01\x94\x01\x90a\x01 V[V[4a\x01vWa\x01ra\x01aa\x01X6`\x04a\0\xC9V[\x92\x91\x90\x91a\x01\x83V[a\x01ia\x002V[\x91\x82\x91\x82a\x01-V[\x03\x90\xF3[a\08V[_\x80\xFD[_\x90V[PPPPa\x01\x8Fa\x01\x7FV[P_\x90V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b610de8565b61001d5f356101dc565b8063086146d2146101d757806318d5aafe146101d2578063366cbab7146101cd5780633b6ab2a9146101c857806346e2cc09146101c3578063485cc955146101be5780634b2c0706146101b95780635467cb48146101b45780635b3cd6e2146101af57806361543801146101aa5780636558954f146101a5578063703cfcbb146101a0578063715018a61461019b5780637a3979dc14610196578063804e51231461019157806382f44ade1461018c57806384fab62b146101875780638d5a239b146101825780638da5cb5b1461017d578063aff74c6d14610178578063c660d3f314610173578063cdafb9781461016e578063d4f0eb4d14610169578063d878134214610164578063de1f453e1461015f578063ea4a11041461015a578063ede07bd6146101555763f2fde38b0361000e57610db5565b610d80565b610d0f565b610be5565b610bb0565b610b59565b610b07565b610a5d565b610a28565b6109f3565b61099c565b610967565b610922565b6108ee565b6108b5565b610830565b6107fb565b6107b7565b610749565b6106ba565b6105ee565b6105b9565b610544565b6104a9565b61046f565b6103fa565b6102d5565b61027e565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f9103126101fa57565b6101ec565b90565b61020b906101ff565b9052565b151590565b61021d9061020f565b9052565b90606080610267936102395f8201515f860190610202565b61024b60208201516020860190610202565b61025d60408201516040860190610202565b0151910190610214565b565b919061027c905f60808501940190610221565b565b346102ae5761028e3660046101f0565b6102aa610299610f59565b6102a16101e2565b91829182610269565b0390f35b6101e8565b6102bc9061020f565b9052565b91906102d3905f602085019401906102b3565b565b34610305576102e53660046101f0565b6103016102f0610ff5565b6102f86101e2565b918291826102c0565b0390f35b6101e8565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156103545781359167ffffffffffffffff831161034f57602001926001830284011161034a57565b610316565b610312565b61030e565b9060208282031261038a575f82013567ffffffffffffffff811161038557610381920161031a565b9091565b61030a565b6101ec565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6103d06103d96020936103de936103c78161038f565b93848093610393565b9586910161039c565b6103a7565b0190565b6103f79160208201915f8184039101526103b1565b90565b3461042b57610427610416610410366004610359565b90611099565b61041e6101e2565b918291826103e2565b0390f35b6101e8565b1c90565b60ff1690565b61044a90600861044f9302610430565b610434565b90565b9061045d915461043a565b90565b61046c60045f90610452565b90565b3461049f5761047f3660046101f0565b61049b61048a610460565b6104926101e2565b918291826102c0565b0390f35b6101e8565b5f0190565b346104d8576104c26104bc366004610359565b9061128a565b6104ca6101e2565b806104d4816104a4565b0390f35b6101e8565b60018060a01b031690565b6104f1906104dd565b90565b6104fd816104e8565b0361050457565b5f80fd5b90503590610515826104f4565b565b919060408382031261053f578061053361053c925f8601610508565b93602001610508565b90565b6101ec565b346105735761055d610557366004610517565b9061143b565b6105656101e2565b8061056f816104a4565b0390f35b6101e8565b610581816101ff565b0361058857565b5f80fd5b9050359061059982610578565b565b906020828203126105b4576105b1915f0161058c565b90565b6101ec565b346105e9576105e56105d46105cf36600461059b565b611447565b6105dc6101e2565b91829182610269565b0390f35b6101e8565b3461061c576105fe3660046101f0565b610606611482565b61060e6101e2565b80610618816104a4565b0390f35b6101e8565b60018060a01b031690565b61063c9060086106419302610430565b610621565b90565b9061064f915461062c565b90565b61065e60015f90610644565b90565b90565b61067861067361067d926104dd565b610661565b6104dd565b90565b61068990610664565b90565b61069590610680565b90565b6106a19061068c565b9052565b91906106b8905f60208501940190610698565b565b346106ea576106ca3660046101f0565b6106e66106d5610652565b6106dd6101e2565b918291826106a5565b0390f35b6101e8565b90565b6107029060086107079302610430565b6106ef565b90565b9061071591546106f2565b90565b61072460035f9061070a565b90565b610730906101ff565b9052565b9190610747905f60208501940190610727565b565b34610779576107593660046101f0565b610775610764610718565b61076c6101e2565b91829182610734565b0390f35b6101e8565b90565b61079561079061079a9261077e565b610661565b6101ff565b90565b6107a962278d00610781565b90565b6107b461079d565b90565b346107e7576107c73660046101f0565b6107e36107d26107ac565b6107da6101e2565b91829182610734565b0390f35b6101e8565b6107f860025f9061070a565b90565b3461082b5761080b3660046101f0565b6108276108166107ec565b61081e6101e2565b91829182610734565b0390f35b6101e8565b3461085e576108403660046101f0565b6108486114b1565b6108506101e2565b8061085a816104a4565b0390f35b6101e8565b916060838303126108b05761087a825f8501610508565b926108888360208301610508565b92604082013567ffffffffffffffff81116108ab576108a7920161031a565b9091565b61030a565b6101ec565b346108e9576108e56108d46108cb366004610863565b92919091611569565b6108dc6101e2565b918291826102c0565b0390f35b6101e8565b3461091d57610907610901366004610359565b906116ec565b61090f6101e2565b80610919816104a4565b0390f35b6101e8565b34610952576109323660046101f0565b61094e61093d611709565b6109456101e2565b91829182610734565b0390f35b6101e8565b6109646004600190610452565b90565b34610997576109773660046101f0565b610993610982610957565b61098a6101e2565b918291826102c0565b0390f35b6101e8565b346109cc576109ac3660046101f0565b6109c86109b76117d6565b6109bf6101e2565b91829182610734565b0390f35b6101e8565b6109da906104e8565b9052565b91906109f1905f602085019401906109d1565b565b34610a2357610a033660046101f0565b610a1f610a0e611868565b610a166101e2565b918291826109de565b0390f35b6101e8565b34610a5857610a383660046101f0565b610a54610a4361189c565b610a4b6101e2565b91829182610734565b0390f35b6101e8565b34610a8d57610a6d3660046101f0565b610a89610a786118e8565b610a806101e2565b91829182610734565b0390f35b6101e8565b909182601f83011215610acc5781359167ffffffffffffffff8311610ac7576020019260208302840111610ac257565b610316565b610312565b61030e565b90602082820312610b02575f82013567ffffffffffffffff8111610afd57610af99201610a92565b9091565b61030a565b6101ec565b34610b3657610b20610b1a366004610ad1565b90611aee565b610b286101e2565b80610b32816104a4565b0390f35b6101e8565b90602082820312610b5457610b51915f01610508565b90565b6101ec565b34610b8757610b71610b6c366004610b3b565b611b9e565b610b796101e2565b80610b83816104a4565b0390f35b6101e8565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610be057610bc03660046101f0565b610bdc610bcb610b8c565b610bd36101e2565b91829182610734565b0390f35b6101e8565b34610c1357610bf53660046101f0565b610bfd611bc5565b610c056101e2565b80610c0f816104a4565b0390f35b6101e8565b610c2c610c27610c31926101ff565b610661565b6101ff565b90565b90610c3e90610c18565b5f5260205260405f2090565b5f1c90565b610c5b610c6091610c4a565b6106ef565b90565b610c6d9054610c4f565b90565b610c7c610c8191610c4a565b610434565b90565b610c8e9054610c70565b90565b610c9c906005610c34565b90610ca85f8301610c63565b91610cb560018201610c63565b91610cce6003610cc760028501610c63565b9301610c84565b90565b610d06610d0d94610cfc606094989795610cf2608086019a5f870190610727565b6020850190610727565b6040830190610727565b01906102b3565b565b34610d4357610d3f610d2a610d2536600461059b565b610c91565b90610d369492946101e2565b94859485610cd1565b0390f35b6101e8565b90565b610d5f610d5a610d6492610d48565b610661565b6101ff565b90565b610d72611388610d4b565b90565b610d7d610d67565b90565b34610db057610d903660046101f0565b610dac610d9b610d75565b610da36101e2565b91829182610734565b0390f35b6101e8565b34610de357610dcd610dc8366004610b3b565b611c34565b610dd56101e2565b80610ddf816104a4565b0390f35b6101e8565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b90610e0a906103a7565b810190811067ffffffffffffffff821117610e2457604052565b610dec565b90610e3c610e356101e2565b9283610e00565b565b610e486080610e29565b90565b5f90565b5f90565b610e5b610e3e565b90602080808085610e6a610e4b565b815201610e75610e4b565b815201610e80610e4b565b815201610e8b610e4f565b81525050565b610e99610e53565b90565b610ea66080610e29565b90565b90565b610ec0610ebb610ec592610ea9565b610661565b6101ff565b90565b90610ed2906101ff565b9052565b90610ee09061020f565b9052565b90610f4b610f426003610ef5610e3e565b94610f0c610f045f8301610c63565b5f8801610ec8565b610f24610f1b60018301610c63565b60208801610ec8565b610f3c610f3360028301610c63565b60408801610ec8565b01610c84565b60608401610ed6565b565b610f5690610ee4565b90565b610f61610e91565b50610f75610f6f6004610c84565b1561020f565b610f9957610f96610f916005610f8b6003610c63565b90610c34565b610f4d565b90565b5f610fee5f610fe5610fdc5f610fd7610fce5f95610fc9610fc1610fbb610e9c565b9a610eac565b5f8b01610ec8565b610eac565b60208801610ec8565b610eac565b60408501610ec8565b60608301610ed6565b90565b5f90565b610ffd610ff1565b506110086004610c84565b90565b606090565b60ff60f81b1690565b60f81b90565b61103361102e61103892610ea9565b611019565b611010565b90565b90565b61104a61104f91611010565b61103b565b9052565b905090565b90825f939282370152565b9091826110738161107a93611053565b8093611058565b0190565b8061108f600192611096969461103e565b0191611063565b90565b6110d7906110a561100b565b506110c86110b25f61101f565b91936110bc6101e2565b9485936020850161107e565b60208201810382520382610e00565b90565b906110f66110f033329085859192909192611569565b1561020f565b61110557611103916111a6565b565b5f631b8e828b60e31b81528061111d600482016104a4565b0390fd5b60081c90565b61113361113891611121565b610434565b90565b6111459054611127565b90565b634e487b7160e01b5f52601160045260245ffd5b61116b611171919392936101ff565b926101ff565b820391821161117c57565b611148565b611190611196919392936101ff565b926101ff565b82018092116111a157565b611148565b906111ba6111b4600461113b565b1561020f565b6111ef576111da6111ed926111d36111e8935a92611243565b5a9061115c565b6111e2610d67565b90611181565b611c9b565b565b6111f891611243565b565b61120390610680565b90565b9190611220816112198161122595610393565b8095611058565b6103a7565b0190565b90916112409260208301925f818503910152611206565b90565b3390916112707f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926111fa565b9261128561127c6101e2565b92839283611229565b0390a2565b90611294916110da565b565b906112a8916112a3611d65565b6113ae565b565b60a01c90565b6112bc6112c1916112aa565b610434565b90565b6112ce90546112b0565b90565b6112e56112e06112ea92610ea9565b610661565b6104dd565b90565b6112f6906112d1565b90565b60a01b90565b9061130e60ff60a01b916112f9565b9181191691161790565b6113219061020f565b90565b90565b9061133c61133761134392611318565b611324565b82546112ff565b9055565b61135090610664565b90565b61135c90611347565b90565b5f1b90565b9061137560018060a01b039161135f565b9181191691161790565b61138890611347565b90565b90565b906113a361139e6113aa9261137f565b61138b565b8254611364565b9055565b6113b860016112c4565b61142057816113d76113d16113cc5f6112ed565b6104e8565b916104e8565b14611404576113fd6113f6611402936113f1600180611327565b611353565b600161138e565b611c34565b565b5f632e7f3c7f60e11b81528061141c600482016104a4565b0390fd5b5f62dc149f60e41b815280611437600482016104a4565b0390fd5b9061144591611296565b565b61145e61146391611456610e91565b506005610c34565b610f4d565b90565b61146e611d65565b611476611478565b565b611480611df0565b565b61148a611466565b565b611494611d65565b61149c61149e565b565b6114af6114aa5f6112ed565b611e20565b565b6114b961148c565b565b6114c76114cc91610c4a565b610621565b90565b6114d990546114bb565b90565b60e01b90565b6114eb8161020f565b036114f257565b5f80fd5b90505190611503826114e2565b565b9060208282031261151e5761151b915f016114f6565b90565b6101ec565b611549611556959394929461153f60608401965f8501906109d1565b60208301906109d1565b6040818503910152611206565b90565b6115616101e2565b3d5f823e3d90fd5b926115ac60209394611579610ff1565b506115b761158f61158a60016114cf565b61068c565b93637a3979dc9295976115a06101e2565b988997889687966114dc565b865260048601611523565b03915afa9081156115fb575f916115cd575b5090565b6115ee915060203d81116115f4575b6115e68183610e00565b810190611505565b5f6115c9565b503d6115dc565b611559565b9061161c61161633329085859192909192611569565b1561020f565b61162b5761162991611647565b565b5f631b8e828b60e31b815280611643600482016104a4565b0390fd5b9061165b611655600461113b565b1561020f565b6116905761167b61168e92611674611689935a9261169b565b5a9061115c565b611683610d67565b90611181565b611c9b565b565b6116999161169b565b565b906116a7903392611099565b906116e76116d57f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926111fa565b926116de6101e2565b918291826103e2565b0390a2565b906116f691611600565b565b5f90565b61170690516101ff565b90565b6117116116f8565b5061172561171f6004610c84565b1561020f565b611795576117616117535f61174d61174860056117426003610c63565b90610c34565b610f4d565b016116fc565b61175b61079d565b90611181565b4261177461176e836101ff565b916101ff565b10156117885761178590429061115c565b90565b506117925f610eac565b90565b61179e5f610eac565b90565b6117b06117b6919392936101ff565b926101ff565b916117c28382026101ff565b9281840414901517156117d157565b611148565b6117de6116f8565b506117f26117ec6004610c84565b1561020f565b61182c576118296118196002611813600561180d6003610c63565b90610c34565b01610c63565b6118236002610c63565b906117a1565b90565b6118355f610eac565b90565b5f90565b60018060a01b031690565b61185361185891610c4a565b61183c565b90565b6118659054611847565b90565b611870611838565b5061187a5f61185b565b90565b90565b61189461188f6118999261187d565b610661565b6101ff565b90565b6118a46116f8565b506118b86118b26004610c84565b1561020f565b6118dc576118d96118c96003610c63565b6118d36001611880565b90611181565b90565b6118e55f610eac565b90565b6118f06116f8565b506119046118fe6004610c84565b1561020f565b61192b576119286002611922600561191c6003610c63565b90610c34565b01610c63565b90565b6119345f610eac565b90565b9061194b611945600461113b565b1561020f565b6119805761196b61197e92611964611979935a92611a25565b5a9061115c565b611973610d67565b90611181565b611c9b565b565b61198991611a25565b565b5090565b600161199b91016101ff565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b903590600160200381360303821215611a00570180359067ffffffffffffffff82116119fb576020019160018202360383136119f657565b6119ba565b6119b6565b6119b2565b90821015611a20576020611a1c92028101906119be565b9091565b61199e565b611a3081839061198b565b91611a396116f8565b50611a435f610eac565b5b80611a57611a51866101ff565b916101ff565b1015611ae857611a8590611a7b333290611a7387878691611a05565b929091611569565b611a8a575b61198f565b611a44565b33611aa0611a9a86868591611a05565b90611099565b90611ae0611ace7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926111fa565b92611ad76101e2565b918291826103e2565b0390a2611a80565b50505050565b90611af891611937565b565b611b0b90611b06611d65565b611b0d565b565b80611b28611b22611b1d5f6112ed565b6104e8565b916104e8565b14611b8257611b40611b3982611353565b600161138e565b611b6a7f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b9916111fa565b90611b736101e2565b80611b7d816104a4565b0390a2565b5f632e7f3c7f60e11b815280611b9a600482016104a4565b0390fd5b611ba790611afa565b565b611bb1611d65565b611bb9611bbb565b565b611bc3611e7f565b565b611bcd611ba9565b565b611be090611bdb611d65565b611be2565b565b80611bfd611bf7611bf25f6112ed565b6104e8565b916104e8565b14611c0d57611c0b90611e20565b565b611c30611c195f6112ed565b5f918291631e4fbdf760e01b8352600483016109de565b0390fd5b611c3d90611bcf565b565b90611c4b5f199161135f565b9181191691161790565b90565b90611c6d611c68611c7492610c18565b611c55565b8254611c3f565b9055565b916020611c99929493611c9260408201965f830190610727565b0190610727565b565b611cae611ca8600461113b565b1561020f565b611d6257611cc5611cbf6004610c84565b1561020f565b611d55575b611cd261205a565b611d0681611d006002611cf06005611cea6003610c63565b90610c34565b0191611cfb83610c63565b611181565b90611c58565b611d106003610c63565b3a611d3b7f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610c18565b92611d50611d476101e2565b92839283611c78565b0390a2565b611d5d611f57565b611cca565b50565b611d6d611868565b611d86611d80611d7b612238565b6104e8565b916104e8565b03611d8d57565b611daf611d98612238565b5f91829163118cdaa760e01b8352600483016109de565b0390fd5b60081b90565b90611dc661ff0091611db3565b9181191691161790565b90611de5611de0611dec92611318565b611324565b8254611db9565b9055565b611dfb5f6004611dd0565b565b90565b90611e15611e10611e1c926111fa565b611dfd565b8254611364565b9055565b611e295f61185b565b611e33825f611e00565b90611e67611e617f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0936111fa565b916111fa565b91611e706101e2565b80611e7a816104a4565b0390a3565b611e8b60016004611dd0565b565b90611e9960ff9161135f565b9181191691161790565b90611eb8611eb3611ebf92611318565b611324565b8254611e8d565b9055565b90611ecd90610eac565b5f5260205260405f2090565b611ee3905161020f565b90565b90611f4360606003611f4994611f095f8201611f035f88016116fc565b90611c58565b611f2260018201611f1c602088016116fc565b90611c58565b611f3b60028201611f35604088016116fc565b90611c58565b019201611ed9565b90611ea3565b565b90611f5591611ee6565b565b611f6a611f646004610c84565b1561020f565b611f71575b565b611f7d60016004611ea3565b611f90611f895f610eac565b6003611c58565b611ff142611fe05f611fd7611fce5f611fc9611fc05f95611fbb611fb2610e9c565b995f8b01610ec8565b610eac565b60208801610ec8565b610eac565b60408501610ec8565b60608301610ed6565b611fec60055f90611ec3565b611f4b565b5f42906120336120217f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610eac565b9261202a6101e2565b91829182610734565b0390a2611f6f565b90565b612047906101ff565b5f1981146120555760010190565b611148565b612077612072600561206c6003610c63565b90610c34565b61203b565b426120a561209f61209a61208c5f8601610c63565b61209461079d565b90611181565b6101ff565b916101ff565b10156120af575b50565b6120d76120ce6120c05f8401610c63565b6120c861079d565b90611181565b60018301611c58565b6120e5600160038301611ea3565b6120ef6003610c63565b61211c6120fe60028401610c63565b926121165f61210f60018401610c63565b9201610c63565b9061115c565b6121467f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610c18565b9261215b6121526101e2565b92839283611c78565b0390a261217a61217361216e6003610c63565b61203e565b6003611c58565b6121e4426121ca5f6121c16121b85f6121b36121aa5f956121a561219c610e9c565b995f8b01610ec8565b610eac565b60208801610ec8565b610eac565b60408501610ec8565b60608301610ed6565b6121df60056121d96003610c63565b90610c34565b611f4b565b6121ee6003610c63565b429061222f61221d7f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610c18565b926122266101e2565b91829182610734565b0390a25f6120ac565b612240611838565b50339056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\r\xE8V[a\0\x1D_5a\x01\xDCV[\x80c\x08aF\xD2\x14a\x01\xD7W\x80c\x18\xD5\xAA\xFE\x14a\x01\xD2W\x80c6l\xBA\xB7\x14a\x01\xCDW\x80c;j\xB2\xA9\x14a\x01\xC8W\x80cF\xE2\xCC\t\x14a\x01\xC3W\x80cH\\\xC9U\x14a\x01\xBEW\x80cK,\x07\x06\x14a\x01\xB9W\x80cTg\xCBH\x14a\x01\xB4W\x80c[<\xD6\xE2\x14a\x01\xAFW\x80caT8\x01\x14a\x01\xAAW\x80ceX\x95O\x14a\x01\xA5W\x80cp<\xFC\xBB\x14a\x01\xA0W\x80cqP\x18\xA6\x14a\x01\x9BW\x80cz9y\xDC\x14a\x01\x96W\x80c\x80NQ#\x14a\x01\x91W\x80c\x82\xF4J\xDE\x14a\x01\x8CW\x80c\x84\xFA\xB6+\x14a\x01\x87W\x80c\x8DZ#\x9B\x14a\x01\x82W\x80c\x8D\xA5\xCB[\x14a\x01}W\x80c\xAF\xF7Lm\x14a\x01xW\x80c\xC6`\xD3\xF3\x14a\x01sW\x80c\xCD\xAF\xB9x\x14a\x01nW\x80c\xD4\xF0\xEBM\x14a\x01iW\x80c\xD8x\x13B\x14a\x01dW\x80c\xDE\x1FE>\x14a\x01_W\x80c\xEAJ\x11\x04\x14a\x01ZW\x80c\xED\xE0{\xD6\x14a\x01UWc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\r\xB5V[a\r\x80V[a\r\x0FV[a\x0B\xE5V[a\x0B\xB0V[a\x0BYV[a\x0B\x07V[a\n]V[a\n(V[a\t\xF3V[a\t\x9CV[a\tgV[a\t\"V[a\x08\xEEV[a\x08\xB5V[a\x080V[a\x07\xFBV[a\x07\xB7V[a\x07IV[a\x06\xBAV[a\x05\xEEV[a\x05\xB9V[a\x05DV[a\x04\xA9V[a\x04oV[a\x03\xFAV[a\x02\xD5V[a\x02~V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01\xFAWV[a\x01\xECV[\x90V[a\x02\x0B\x90a\x01\xFFV[\x90RV[\x15\x15\x90V[a\x02\x1D\x90a\x02\x0FV[\x90RV[\x90``\x80a\x02g\x93a\x029_\x82\x01Q_\x86\x01\x90a\x02\x02V[a\x02K` \x82\x01Q` \x86\x01\x90a\x02\x02V[a\x02]`@\x82\x01Q`@\x86\x01\x90a\x02\x02V[\x01Q\x91\x01\x90a\x02\x14V[V[\x91\x90a\x02|\x90_`\x80\x85\x01\x94\x01\x90a\x02!V[V[4a\x02\xAEWa\x02\x8E6`\x04a\x01\xF0V[a\x02\xAAa\x02\x99a\x0FYV[a\x02\xA1a\x01\xE2V[\x91\x82\x91\x82a\x02iV[\x03\x90\xF3[a\x01\xE8V[a\x02\xBC\x90a\x02\x0FV[\x90RV[\x91\x90a\x02\xD3\x90_` \x85\x01\x94\x01\x90a\x02\xB3V[V[4a\x03\x05Wa\x02\xE56`\x04a\x01\xF0V[a\x03\x01a\x02\xF0a\x0F\xF5V[a\x02\xF8a\x01\xE2V[\x91\x82\x91\x82a\x02\xC0V[\x03\x90\xF3[a\x01\xE8V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03TW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03OW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x03JWV[a\x03\x16V[a\x03\x12V[a\x03\x0EV[\x90` \x82\x82\x03\x12a\x03\x8AW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\x85Wa\x03\x81\x92\x01a\x03\x1AV[\x90\x91V[a\x03\nV[a\x01\xECV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\xD0a\x03\xD9` \x93a\x03\xDE\x93a\x03\xC7\x81a\x03\x8FV[\x93\x84\x80\x93a\x03\x93V[\x95\x86\x91\x01a\x03\x9CV[a\x03\xA7V[\x01\x90V[a\x03\xF7\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xB1V[\x90V[4a\x04+Wa\x04'a\x04\x16a\x04\x106`\x04a\x03YV[\x90a\x10\x99V[a\x04\x1Ea\x01\xE2V[\x91\x82\x91\x82a\x03\xE2V[\x03\x90\xF3[a\x01\xE8V[\x1C\x90V[`\xFF\x16\x90V[a\x04J\x90`\x08a\x04O\x93\x02a\x040V[a\x044V[\x90V[\x90a\x04]\x91Ta\x04:V[\x90V[a\x04l`\x04_\x90a\x04RV[\x90V[4a\x04\x9FWa\x04\x7F6`\x04a\x01\xF0V[a\x04\x9Ba\x04\x8Aa\x04`V[a\x04\x92a\x01\xE2V[\x91\x82\x91\x82a\x02\xC0V[\x03\x90\xF3[a\x01\xE8V[_\x01\x90V[4a\x04\xD8Wa\x04\xC2a\x04\xBC6`\x04a\x03YV[\x90a\x12\x8AV[a\x04\xCAa\x01\xE2V[\x80a\x04\xD4\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\xF1\x90a\x04\xDDV[\x90V[a\x04\xFD\x81a\x04\xE8V[\x03a\x05\x04WV[_\x80\xFD[\x90P5\x90a\x05\x15\x82a\x04\xF4V[V[\x91\x90`@\x83\x82\x03\x12a\x05?W\x80a\x053a\x05<\x92_\x86\x01a\x05\x08V[\x93` \x01a\x05\x08V[\x90V[a\x01\xECV[4a\x05sWa\x05]a\x05W6`\x04a\x05\x17V[\x90a\x14;V[a\x05ea\x01\xE2V[\x80a\x05o\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[a\x05\x81\x81a\x01\xFFV[\x03a\x05\x88WV[_\x80\xFD[\x90P5\x90a\x05\x99\x82a\x05xV[V[\x90` \x82\x82\x03\x12a\x05\xB4Wa\x05\xB1\x91_\x01a\x05\x8CV[\x90V[a\x01\xECV[4a\x05\xE9Wa\x05\xE5a\x05\xD4a\x05\xCF6`\x04a\x05\x9BV[a\x14GV[a\x05\xDCa\x01\xE2V[\x91\x82\x91\x82a\x02iV[\x03\x90\xF3[a\x01\xE8V[4a\x06\x1CWa\x05\xFE6`\x04a\x01\xF0V[a\x06\x06a\x14\x82V[a\x06\x0Ea\x01\xE2V[\x80a\x06\x18\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x06<\x90`\x08a\x06A\x93\x02a\x040V[a\x06!V[\x90V[\x90a\x06O\x91Ta\x06,V[\x90V[a\x06^`\x01_\x90a\x06DV[\x90V[\x90V[a\x06xa\x06sa\x06}\x92a\x04\xDDV[a\x06aV[a\x04\xDDV[\x90V[a\x06\x89\x90a\x06dV[\x90V[a\x06\x95\x90a\x06\x80V[\x90V[a\x06\xA1\x90a\x06\x8CV[\x90RV[\x91\x90a\x06\xB8\x90_` \x85\x01\x94\x01\x90a\x06\x98V[V[4a\x06\xEAWa\x06\xCA6`\x04a\x01\xF0V[a\x06\xE6a\x06\xD5a\x06RV[a\x06\xDDa\x01\xE2V[\x91\x82\x91\x82a\x06\xA5V[\x03\x90\xF3[a\x01\xE8V[\x90V[a\x07\x02\x90`\x08a\x07\x07\x93\x02a\x040V[a\x06\xEFV[\x90V[\x90a\x07\x15\x91Ta\x06\xF2V[\x90V[a\x07$`\x03_\x90a\x07\nV[\x90V[a\x070\x90a\x01\xFFV[\x90RV[\x91\x90a\x07G\x90_` \x85\x01\x94\x01\x90a\x07'V[V[4a\x07yWa\x07Y6`\x04a\x01\xF0V[a\x07ua\x07da\x07\x18V[a\x07la\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[\x90V[a\x07\x95a\x07\x90a\x07\x9A\x92a\x07~V[a\x06aV[a\x01\xFFV[\x90V[a\x07\xA9b'\x8D\0a\x07\x81V[\x90V[a\x07\xB4a\x07\x9DV[\x90V[4a\x07\xE7Wa\x07\xC76`\x04a\x01\xF0V[a\x07\xE3a\x07\xD2a\x07\xACV[a\x07\xDAa\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[a\x07\xF8`\x02_\x90a\x07\nV[\x90V[4a\x08+Wa\x08\x0B6`\x04a\x01\xF0V[a\x08'a\x08\x16a\x07\xECV[a\x08\x1Ea\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[4a\x08^Wa\x08@6`\x04a\x01\xF0V[a\x08Ha\x14\xB1V[a\x08Pa\x01\xE2V[\x80a\x08Z\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[\x91``\x83\x83\x03\x12a\x08\xB0Wa\x08z\x82_\x85\x01a\x05\x08V[\x92a\x08\x88\x83` \x83\x01a\x05\x08V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08\xABWa\x08\xA7\x92\x01a\x03\x1AV[\x90\x91V[a\x03\nV[a\x01\xECV[4a\x08\xE9Wa\x08\xE5a\x08\xD4a\x08\xCB6`\x04a\x08cV[\x92\x91\x90\x91a\x15iV[a\x08\xDCa\x01\xE2V[\x91\x82\x91\x82a\x02\xC0V[\x03\x90\xF3[a\x01\xE8V[4a\t\x1DWa\t\x07a\t\x016`\x04a\x03YV[\x90a\x16\xECV[a\t\x0Fa\x01\xE2V[\x80a\t\x19\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[4a\tRWa\t26`\x04a\x01\xF0V[a\tNa\t=a\x17\tV[a\tEa\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[a\td`\x04`\x01\x90a\x04RV[\x90V[4a\t\x97Wa\tw6`\x04a\x01\xF0V[a\t\x93a\t\x82a\tWV[a\t\x8Aa\x01\xE2V[\x91\x82\x91\x82a\x02\xC0V[\x03\x90\xF3[a\x01\xE8V[4a\t\xCCWa\t\xAC6`\x04a\x01\xF0V[a\t\xC8a\t\xB7a\x17\xD6V[a\t\xBFa\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[a\t\xDA\x90a\x04\xE8V[\x90RV[\x91\x90a\t\xF1\x90_` \x85\x01\x94\x01\x90a\t\xD1V[V[4a\n#Wa\n\x036`\x04a\x01\xF0V[a\n\x1Fa\n\x0Ea\x18hV[a\n\x16a\x01\xE2V[\x91\x82\x91\x82a\t\xDEV[\x03\x90\xF3[a\x01\xE8V[4a\nXWa\n86`\x04a\x01\xF0V[a\nTa\nCa\x18\x9CV[a\nKa\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[4a\n\x8DWa\nm6`\x04a\x01\xF0V[a\n\x89a\nxa\x18\xE8V[a\n\x80a\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\n\xCCW\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\xC7W` \x01\x92` \x83\x02\x84\x01\x11a\n\xC2WV[a\x03\x16V[a\x03\x12V[a\x03\x0EV[\x90` \x82\x82\x03\x12a\x0B\x02W_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xFDWa\n\xF9\x92\x01a\n\x92V[\x90\x91V[a\x03\nV[a\x01\xECV[4a\x0B6Wa\x0B a\x0B\x1A6`\x04a\n\xD1V[\x90a\x1A\xEEV[a\x0B(a\x01\xE2V[\x80a\x0B2\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[\x90` \x82\x82\x03\x12a\x0BTWa\x0BQ\x91_\x01a\x05\x08V[\x90V[a\x01\xECV[4a\x0B\x87Wa\x0Bqa\x0Bl6`\x04a\x0B;V[a\x1B\x9EV[a\x0Bya\x01\xE2V[\x80a\x0B\x83\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B\xE0Wa\x0B\xC06`\x04a\x01\xF0V[a\x0B\xDCa\x0B\xCBa\x0B\x8CV[a\x0B\xD3a\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[4a\x0C\x13Wa\x0B\xF56`\x04a\x01\xF0V[a\x0B\xFDa\x1B\xC5V[a\x0C\x05a\x01\xE2V[\x80a\x0C\x0F\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[a\x0C,a\x0C'a\x0C1\x92a\x01\xFFV[a\x06aV[a\x01\xFFV[\x90V[\x90a\x0C>\x90a\x0C\x18V[_R` R`@_ \x90V[_\x1C\x90V[a\x0C[a\x0C`\x91a\x0CJV[a\x06\xEFV[\x90V[a\x0Cm\x90Ta\x0COV[\x90V[a\x0C|a\x0C\x81\x91a\x0CJV[a\x044V[\x90V[a\x0C\x8E\x90Ta\x0CpV[\x90V[a\x0C\x9C\x90`\x05a\x0C4V[\x90a\x0C\xA8_\x83\x01a\x0CcV[\x91a\x0C\xB5`\x01\x82\x01a\x0CcV[\x91a\x0C\xCE`\x03a\x0C\xC7`\x02\x85\x01a\x0CcV[\x93\x01a\x0C\x84V[\x90V[a\r\x06a\r\r\x94a\x0C\xFC``\x94\x98\x97\x95a\x0C\xF2`\x80\x86\x01\x9A_\x87\x01\x90a\x07'V[` \x85\x01\x90a\x07'V[`@\x83\x01\x90a\x07'V[\x01\x90a\x02\xB3V[V[4a\rCWa\r?a\r*a\r%6`\x04a\x05\x9BV[a\x0C\x91V[\x90a\r6\x94\x92\x94a\x01\xE2V[\x94\x85\x94\x85a\x0C\xD1V[\x03\x90\xF3[a\x01\xE8V[\x90V[a\r_a\rZa\rd\x92a\rHV[a\x06aV[a\x01\xFFV[\x90V[a\rra\x13\x88a\rKV[\x90V[a\r}a\rgV[\x90V[4a\r\xB0Wa\r\x906`\x04a\x01\xF0V[a\r\xACa\r\x9Ba\ruV[a\r\xA3a\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xF3[a\x01\xE8V[4a\r\xE3Wa\r\xCDa\r\xC86`\x04a\x0B;V[a\x1C4V[a\r\xD5a\x01\xE2V[\x80a\r\xDF\x81a\x04\xA4V[\x03\x90\xF3[a\x01\xE8V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x0E\n\x90a\x03\xA7V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E$W`@RV[a\r\xECV[\x90a\x0E<a\x0E5a\x01\xE2V[\x92\x83a\x0E\0V[V[a\x0EH`\x80a\x0E)V[\x90V[_\x90V[_\x90V[a\x0E[a\x0E>V[\x90` \x80\x80\x80\x85a\x0Eja\x0EKV[\x81R\x01a\x0Eua\x0EKV[\x81R\x01a\x0E\x80a\x0EKV[\x81R\x01a\x0E\x8Ba\x0EOV[\x81RPPV[a\x0E\x99a\x0ESV[\x90V[a\x0E\xA6`\x80a\x0E)V[\x90V[\x90V[a\x0E\xC0a\x0E\xBBa\x0E\xC5\x92a\x0E\xA9V[a\x06aV[a\x01\xFFV[\x90V[\x90a\x0E\xD2\x90a\x01\xFFV[\x90RV[\x90a\x0E\xE0\x90a\x02\x0FV[\x90RV[\x90a\x0FKa\x0FB`\x03a\x0E\xF5a\x0E>V[\x94a\x0F\x0Ca\x0F\x04_\x83\x01a\x0CcV[_\x88\x01a\x0E\xC8V[a\x0F$a\x0F\x1B`\x01\x83\x01a\x0CcV[` \x88\x01a\x0E\xC8V[a\x0F<a\x0F3`\x02\x83\x01a\x0CcV[`@\x88\x01a\x0E\xC8V[\x01a\x0C\x84V[``\x84\x01a\x0E\xD6V[V[a\x0FV\x90a\x0E\xE4V[\x90V[a\x0Faa\x0E\x91V[Pa\x0Fua\x0Fo`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x0F\x99Wa\x0F\x96a\x0F\x91`\x05a\x0F\x8B`\x03a\x0CcV[\x90a\x0C4V[a\x0FMV[\x90V[_a\x0F\xEE_a\x0F\xE5a\x0F\xDC_a\x0F\xD7a\x0F\xCE_\x95a\x0F\xC9a\x0F\xC1a\x0F\xBBa\x0E\x9CV[\x9Aa\x0E\xACV[_\x8B\x01a\x0E\xC8V[a\x0E\xACV[` \x88\x01a\x0E\xC8V[a\x0E\xACV[`@\x85\x01a\x0E\xC8V[``\x83\x01a\x0E\xD6V[\x90V[_\x90V[a\x0F\xFDa\x0F\xF1V[Pa\x10\x08`\x04a\x0C\x84V[\x90V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x103a\x10.a\x108\x92a\x0E\xA9V[a\x10\x19V[a\x10\x10V[\x90V[\x90V[a\x10Ja\x10O\x91a\x10\x10V[a\x10;V[\x90RV[\x90P\x90V[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x10s\x81a\x10z\x93a\x10SV[\x80\x93a\x10XV[\x01\x90V[\x80a\x10\x8F`\x01\x92a\x10\x96\x96\x94a\x10>V[\x01\x91a\x10cV[\x90V[a\x10\xD7\x90a\x10\xA5a\x10\x0BV[Pa\x10\xC8a\x10\xB2_a\x10\x1FV[\x91\x93a\x10\xBCa\x01\xE2V[\x94\x85\x93` \x85\x01a\x10~V[` \x82\x01\x81\x03\x82R\x03\x82a\x0E\0V[\x90V[\x90a\x10\xF6a\x10\xF032\x90\x85\x85\x91\x92\x90\x91\x92a\x15iV[\x15a\x02\x0FV[a\x11\x05Wa\x11\x03\x91a\x11\xA6V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x11\x1D`\x04\x82\x01a\x04\xA4V[\x03\x90\xFD[`\x08\x1C\x90V[a\x113a\x118\x91a\x11!V[a\x044V[\x90V[a\x11E\x90Ta\x11'V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x11ka\x11q\x91\x93\x92\x93a\x01\xFFV[\x92a\x01\xFFV[\x82\x03\x91\x82\x11a\x11|WV[a\x11HV[a\x11\x90a\x11\x96\x91\x93\x92\x93a\x01\xFFV[\x92a\x01\xFFV[\x82\x01\x80\x92\x11a\x11\xA1WV[a\x11HV[\x90a\x11\xBAa\x11\xB4`\x04a\x11;V[\x15a\x02\x0FV[a\x11\xEFWa\x11\xDAa\x11\xED\x92a\x11\xD3a\x11\xE8\x93Z\x92a\x12CV[Z\x90a\x11\\V[a\x11\xE2a\rgV[\x90a\x11\x81V[a\x1C\x9BV[V[a\x11\xF8\x91a\x12CV[V[a\x12\x03\x90a\x06\x80V[\x90V[\x91\x90a\x12 \x81a\x12\x19\x81a\x12%\x95a\x03\x93V[\x80\x95a\x10XV[a\x03\xA7V[\x01\x90V[\x90\x91a\x12@\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x12\x06V[\x90V[3\x90\x91a\x12p\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11\xFAV[\x92a\x12\x85a\x12|a\x01\xE2V[\x92\x83\x92\x83a\x12)V[\x03\x90\xA2V[\x90a\x12\x94\x91a\x10\xDAV[V[\x90a\x12\xA8\x91a\x12\xA3a\x1DeV[a\x13\xAEV[V[`\xA0\x1C\x90V[a\x12\xBCa\x12\xC1\x91a\x12\xAAV[a\x044V[\x90V[a\x12\xCE\x90Ta\x12\xB0V[\x90V[a\x12\xE5a\x12\xE0a\x12\xEA\x92a\x0E\xA9V[a\x06aV[a\x04\xDDV[\x90V[a\x12\xF6\x90a\x12\xD1V[\x90V[`\xA0\x1B\x90V[\x90a\x13\x0E`\xFF`\xA0\x1B\x91a\x12\xF9V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x13!\x90a\x02\x0FV[\x90V[\x90V[\x90a\x13<a\x137a\x13C\x92a\x13\x18V[a\x13$V[\x82Ta\x12\xFFV[\x90UV[a\x13P\x90a\x06dV[\x90V[a\x13\\\x90a\x13GV[\x90V[_\x1B\x90V[\x90a\x13u`\x01\x80`\xA0\x1B\x03\x91a\x13_V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x13\x88\x90a\x13GV[\x90V[\x90V[\x90a\x13\xA3a\x13\x9Ea\x13\xAA\x92a\x13\x7FV[a\x13\x8BV[\x82Ta\x13dV[\x90UV[a\x13\xB8`\x01a\x12\xC4V[a\x14 W\x81a\x13\xD7a\x13\xD1a\x13\xCC_a\x12\xEDV[a\x04\xE8V[\x91a\x04\xE8V[\x14a\x14\x04Wa\x13\xFDa\x13\xF6a\x14\x02\x93a\x13\xF1`\x01\x80a\x13'V[a\x13SV[`\x01a\x13\x8EV[a\x1C4V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x14\x1C`\x04\x82\x01a\x04\xA4V[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x147`\x04\x82\x01a\x04\xA4V[\x03\x90\xFD[\x90a\x14E\x91a\x12\x96V[V[a\x14^a\x14c\x91a\x14Va\x0E\x91V[P`\x05a\x0C4V[a\x0FMV[\x90V[a\x14na\x1DeV[a\x14va\x14xV[V[a\x14\x80a\x1D\xF0V[V[a\x14\x8Aa\x14fV[V[a\x14\x94a\x1DeV[a\x14\x9Ca\x14\x9EV[V[a\x14\xAFa\x14\xAA_a\x12\xEDV[a\x1E V[V[a\x14\xB9a\x14\x8CV[V[a\x14\xC7a\x14\xCC\x91a\x0CJV[a\x06!V[\x90V[a\x14\xD9\x90Ta\x14\xBBV[\x90V[`\xE0\x1B\x90V[a\x14\xEB\x81a\x02\x0FV[\x03a\x14\xF2WV[_\x80\xFD[\x90PQ\x90a\x15\x03\x82a\x14\xE2V[V[\x90` \x82\x82\x03\x12a\x15\x1EWa\x15\x1B\x91_\x01a\x14\xF6V[\x90V[a\x01\xECV[a\x15Ia\x15V\x95\x93\x94\x92\x94a\x15?``\x84\x01\x96_\x85\x01\x90a\t\xD1V[` \x83\x01\x90a\t\xD1V[`@\x81\x85\x03\x91\x01Ra\x12\x06V[\x90V[a\x15aa\x01\xE2V[=_\x82>=\x90\xFD[\x92a\x15\xAC` \x93\x94a\x15ya\x0F\xF1V[Pa\x15\xB7a\x15\x8Fa\x15\x8A`\x01a\x14\xCFV[a\x06\x8CV[\x93cz9y\xDC\x92\x95\x97a\x15\xA0a\x01\xE2V[\x98\x89\x97\x88\x96\x87\x96a\x14\xDCV[\x86R`\x04\x86\x01a\x15#V[\x03\x91Z\xFA\x90\x81\x15a\x15\xFBW_\x91a\x15\xCDW[P\x90V[a\x15\xEE\x91P` =\x81\x11a\x15\xF4W[a\x15\xE6\x81\x83a\x0E\0V[\x81\x01\x90a\x15\x05V[_a\x15\xC9V[P=a\x15\xDCV[a\x15YV[\x90a\x16\x1Ca\x16\x1632\x90\x85\x85\x91\x92\x90\x91\x92a\x15iV[\x15a\x02\x0FV[a\x16+Wa\x16)\x91a\x16GV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x16C`\x04\x82\x01a\x04\xA4V[\x03\x90\xFD[\x90a\x16[a\x16U`\x04a\x11;V[\x15a\x02\x0FV[a\x16\x90Wa\x16{a\x16\x8E\x92a\x16ta\x16\x89\x93Z\x92a\x16\x9BV[Z\x90a\x11\\V[a\x16\x83a\rgV[\x90a\x11\x81V[a\x1C\x9BV[V[a\x16\x99\x91a\x16\x9BV[V[\x90a\x16\xA7\x903\x92a\x10\x99V[\x90a\x16\xE7a\x16\xD5\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11\xFAV[\x92a\x16\xDEa\x01\xE2V[\x91\x82\x91\x82a\x03\xE2V[\x03\x90\xA2V[\x90a\x16\xF6\x91a\x16\0V[V[_\x90V[a\x17\x06\x90Qa\x01\xFFV[\x90V[a\x17\x11a\x16\xF8V[Pa\x17%a\x17\x1F`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x17\x95Wa\x17aa\x17S_a\x17Ma\x17H`\x05a\x17B`\x03a\x0CcV[\x90a\x0C4V[a\x0FMV[\x01a\x16\xFCV[a\x17[a\x07\x9DV[\x90a\x11\x81V[Ba\x17ta\x17n\x83a\x01\xFFV[\x91a\x01\xFFV[\x10\x15a\x17\x88Wa\x17\x85\x90B\x90a\x11\\V[\x90V[Pa\x17\x92_a\x0E\xACV[\x90V[a\x17\x9E_a\x0E\xACV[\x90V[a\x17\xB0a\x17\xB6\x91\x93\x92\x93a\x01\xFFV[\x92a\x01\xFFV[\x91a\x17\xC2\x83\x82\x02a\x01\xFFV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x17\xD1WV[a\x11HV[a\x17\xDEa\x16\xF8V[Pa\x17\xF2a\x17\xEC`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x18,Wa\x18)a\x18\x19`\x02a\x18\x13`\x05a\x18\r`\x03a\x0CcV[\x90a\x0C4V[\x01a\x0CcV[a\x18#`\x02a\x0CcV[\x90a\x17\xA1V[\x90V[a\x185_a\x0E\xACV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x18Sa\x18X\x91a\x0CJV[a\x18<V[\x90V[a\x18e\x90Ta\x18GV[\x90V[a\x18pa\x188V[Pa\x18z_a\x18[V[\x90V[\x90V[a\x18\x94a\x18\x8Fa\x18\x99\x92a\x18}V[a\x06aV[a\x01\xFFV[\x90V[a\x18\xA4a\x16\xF8V[Pa\x18\xB8a\x18\xB2`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x18\xDCWa\x18\xD9a\x18\xC9`\x03a\x0CcV[a\x18\xD3`\x01a\x18\x80V[\x90a\x11\x81V[\x90V[a\x18\xE5_a\x0E\xACV[\x90V[a\x18\xF0a\x16\xF8V[Pa\x19\x04a\x18\xFE`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x19+Wa\x19(`\x02a\x19\"`\x05a\x19\x1C`\x03a\x0CcV[\x90a\x0C4V[\x01a\x0CcV[\x90V[a\x194_a\x0E\xACV[\x90V[\x90a\x19Ka\x19E`\x04a\x11;V[\x15a\x02\x0FV[a\x19\x80Wa\x19ka\x19~\x92a\x19da\x19y\x93Z\x92a\x1A%V[Z\x90a\x11\\V[a\x19sa\rgV[\x90a\x11\x81V[a\x1C\x9BV[V[a\x19\x89\x91a\x1A%V[V[P\x90V[`\x01a\x19\x9B\x91\x01a\x01\xFFV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x1A\0W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x19\xFBW` \x01\x91`\x01\x82\x026\x03\x83\x13a\x19\xF6WV[a\x19\xBAV[a\x19\xB6V[a\x19\xB2V[\x90\x82\x10\x15a\x1A W` a\x1A\x1C\x92\x02\x81\x01\x90a\x19\xBEV[\x90\x91V[a\x19\x9EV[a\x1A0\x81\x83\x90a\x19\x8BV[\x91a\x1A9a\x16\xF8V[Pa\x1AC_a\x0E\xACV[[\x80a\x1AWa\x1AQ\x86a\x01\xFFV[\x91a\x01\xFFV[\x10\x15a\x1A\xE8Wa\x1A\x85\x90a\x1A{32\x90a\x1As\x87\x87\x86\x91a\x1A\x05V[\x92\x90\x91a\x15iV[a\x1A\x8AW[a\x19\x8FV[a\x1ADV[3a\x1A\xA0a\x1A\x9A\x86\x86\x85\x91a\x1A\x05V[\x90a\x10\x99V[\x90a\x1A\xE0a\x1A\xCE\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x11\xFAV[\x92a\x1A\xD7a\x01\xE2V[\x91\x82\x91\x82a\x03\xE2V[\x03\x90\xA2a\x1A\x80V[PPPPV[\x90a\x1A\xF8\x91a\x197V[V[a\x1B\x0B\x90a\x1B\x06a\x1DeV[a\x1B\rV[V[\x80a\x1B(a\x1B\"a\x1B\x1D_a\x12\xEDV[a\x04\xE8V[\x91a\x04\xE8V[\x14a\x1B\x82Wa\x1B@a\x1B9\x82a\x13SV[`\x01a\x13\x8EV[a\x1Bj\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x11\xFAV[\x90a\x1Bsa\x01\xE2V[\x80a\x1B}\x81a\x04\xA4V[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x1B\x9A`\x04\x82\x01a\x04\xA4V[\x03\x90\xFD[a\x1B\xA7\x90a\x1A\xFAV[V[a\x1B\xB1a\x1DeV[a\x1B\xB9a\x1B\xBBV[V[a\x1B\xC3a\x1E\x7FV[V[a\x1B\xCDa\x1B\xA9V[V[a\x1B\xE0\x90a\x1B\xDBa\x1DeV[a\x1B\xE2V[V[\x80a\x1B\xFDa\x1B\xF7a\x1B\xF2_a\x12\xEDV[a\x04\xE8V[\x91a\x04\xE8V[\x14a\x1C\rWa\x1C\x0B\x90a\x1E V[V[a\x1C0a\x1C\x19_a\x12\xEDV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\t\xDEV[\x03\x90\xFD[a\x1C=\x90a\x1B\xCFV[V[\x90a\x1CK_\x19\x91a\x13_V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x1Cma\x1Cha\x1Ct\x92a\x0C\x18V[a\x1CUV[\x82Ta\x1C?V[\x90UV[\x91` a\x1C\x99\x92\x94\x93a\x1C\x92`@\x82\x01\x96_\x83\x01\x90a\x07'V[\x01\x90a\x07'V[V[a\x1C\xAEa\x1C\xA8`\x04a\x11;V[\x15a\x02\x0FV[a\x1DbWa\x1C\xC5a\x1C\xBF`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x1DUW[a\x1C\xD2a ZV[a\x1D\x06\x81a\x1D\0`\x02a\x1C\xF0`\x05a\x1C\xEA`\x03a\x0CcV[\x90a\x0C4V[\x01\x91a\x1C\xFB\x83a\x0CcV[a\x11\x81V[\x90a\x1CXV[a\x1D\x10`\x03a\x0CcV[:a\x1D;\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0C\x18V[\x92a\x1DPa\x1DGa\x01\xE2V[\x92\x83\x92\x83a\x1CxV[\x03\x90\xA2V[a\x1D]a\x1FWV[a\x1C\xCAV[PV[a\x1Dma\x18hV[a\x1D\x86a\x1D\x80a\x1D{a\"8V[a\x04\xE8V[\x91a\x04\xE8V[\x03a\x1D\x8DWV[a\x1D\xAFa\x1D\x98a\"8V[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\t\xDEV[\x03\x90\xFD[`\x08\x1B\x90V[\x90a\x1D\xC6a\xFF\0\x91a\x1D\xB3V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1D\xE5a\x1D\xE0a\x1D\xEC\x92a\x13\x18V[a\x13$V[\x82Ta\x1D\xB9V[\x90UV[a\x1D\xFB_`\x04a\x1D\xD0V[V[\x90V[\x90a\x1E\x15a\x1E\x10a\x1E\x1C\x92a\x11\xFAV[a\x1D\xFDV[\x82Ta\x13dV[\x90UV[a\x1E)_a\x18[V[a\x1E3\x82_a\x1E\0V[\x90a\x1Ega\x1Ea\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x11\xFAV[\x91a\x11\xFAV[\x91a\x1Epa\x01\xE2V[\x80a\x1Ez\x81a\x04\xA4V[\x03\x90\xA3V[a\x1E\x8B`\x01`\x04a\x1D\xD0V[V[\x90a\x1E\x99`\xFF\x91a\x13_V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1E\xB8a\x1E\xB3a\x1E\xBF\x92a\x13\x18V[a\x13$V[\x82Ta\x1E\x8DV[\x90UV[\x90a\x1E\xCD\x90a\x0E\xACV[_R` R`@_ \x90V[a\x1E\xE3\x90Qa\x02\x0FV[\x90V[\x90a\x1FC```\x03a\x1FI\x94a\x1F\t_\x82\x01a\x1F\x03_\x88\x01a\x16\xFCV[\x90a\x1CXV[a\x1F\"`\x01\x82\x01a\x1F\x1C` \x88\x01a\x16\xFCV[\x90a\x1CXV[a\x1F;`\x02\x82\x01a\x1F5`@\x88\x01a\x16\xFCV[\x90a\x1CXV[\x01\x92\x01a\x1E\xD9V[\x90a\x1E\xA3V[V[\x90a\x1FU\x91a\x1E\xE6V[V[a\x1Fja\x1Fd`\x04a\x0C\x84V[\x15a\x02\x0FV[a\x1FqW[V[a\x1F}`\x01`\x04a\x1E\xA3V[a\x1F\x90a\x1F\x89_a\x0E\xACV[`\x03a\x1CXV[a\x1F\xF1Ba\x1F\xE0_a\x1F\xD7a\x1F\xCE_a\x1F\xC9a\x1F\xC0_\x95a\x1F\xBBa\x1F\xB2a\x0E\x9CV[\x99_\x8B\x01a\x0E\xC8V[a\x0E\xACV[` \x88\x01a\x0E\xC8V[a\x0E\xACV[`@\x85\x01a\x0E\xC8V[``\x83\x01a\x0E\xD6V[a\x1F\xEC`\x05_\x90a\x1E\xC3V[a\x1FKV[_B\x90a 3a !\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0E\xACV[\x92a *a\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xA2a\x1FoV[\x90V[a G\x90a\x01\xFFV[_\x19\x81\x14a UW`\x01\x01\x90V[a\x11HV[a wa r`\x05a l`\x03a\x0CcV[\x90a\x0C4V[a ;V[Ba \xA5a \x9Fa \x9Aa \x8C_\x86\x01a\x0CcV[a \x94a\x07\x9DV[\x90a\x11\x81V[a\x01\xFFV[\x91a\x01\xFFV[\x10\x15a \xAFW[PV[a \xD7a \xCEa \xC0_\x84\x01a\x0CcV[a \xC8a\x07\x9DV[\x90a\x11\x81V[`\x01\x83\x01a\x1CXV[a \xE5`\x01`\x03\x83\x01a\x1E\xA3V[a \xEF`\x03a\x0CcV[a!\x1Ca \xFE`\x02\x84\x01a\x0CcV[\x92a!\x16_a!\x0F`\x01\x84\x01a\x0CcV[\x92\x01a\x0CcV[\x90a\x11\\V[a!F\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0C\x18V[\x92a![a!Ra\x01\xE2V[\x92\x83\x92\x83a\x1CxV[\x03\x90\xA2a!za!sa!n`\x03a\x0CcV[a >V[`\x03a\x1CXV[a!\xE4Ba!\xCA_a!\xC1a!\xB8_a!\xB3a!\xAA_\x95a!\xA5a!\x9Ca\x0E\x9CV[\x99_\x8B\x01a\x0E\xC8V[a\x0E\xACV[` \x88\x01a\x0E\xC8V[a\x0E\xACV[`@\x85\x01a\x0E\xC8V[``\x83\x01a\x0E\xD6V[a!\xDF`\x05a!\xD9`\x03a\x0CcV[\x90a\x0C4V[a\x1FKV[a!\xEE`\x03a\x0CcV[B\x90a\"/a\"\x1D\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0C\x18V[\x92a\"&a\x01\xE2V[\x91\x82\x91\x82a\x074V[\x03\x90\xA2_a \xACV[a\"@a\x188V[P3\x90V",
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
        gasPriceInSynd(gasPriceInSyndCall),
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
            [112u8, 60u8, 252u8, 187u8],
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
        const COUNT: usize = 28usize;
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
                Self::gasPriceInSynd(_) => {
                    <gasPriceInSyndCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn gasPriceInSynd(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateSequencingChainCalls> {
                        <gasPriceInSyndCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateSequencingChainCalls::gasPriceInSynd)
                    }
                    gasPriceInSynd
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
                Self::gasPriceInSynd(inner) => {
                    <gasPriceInSyndCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::gasPriceInSynd(inner) => {
                    <gasPriceInSyndCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        ///Creates a new call builder for the [`gasPriceInSynd`] function.
        pub fn gasPriceInSynd(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, gasPriceInSyndCall, N> {
            self.call_builder(&gasPriceInSyndCall {})
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
