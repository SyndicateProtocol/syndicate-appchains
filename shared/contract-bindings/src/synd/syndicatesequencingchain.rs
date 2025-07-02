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
    ///0x60a060405234610038576100196100146100e9565b6101b7565b61002161003d565b61201c610534823960805181610ae6015261201c90f35b610043565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b9061006f90610047565b810190811060018060401b0382111761008757604052565b610051565b9061009f61009861003d565b9283610065565b565b5f80fd5b90565b6100b1816100a5565b036100b857565b5f80fd5b905051906100c9826100a8565b565b906020828203126100e4576100e1915f016100bc565b90565b6100a1565b61010761270f803803806100fc8161008c565b9283398101906100cb565b90565b90565b90565b61012461011f6101299261010a565b61010d565b6100a5565b90565b60209181520190565b5f7f41707020636861696e2049442063616e6e6f7420626520300000000000000000910152565b610169601860209261012c565b61017281610135565b0190565b61018b9060208101905f81830391015261015c565b90565b1561019557565b61019d61003d565b62461bcd60e51b8152806101b360048201610176565b0390fd5b6101bf61023e565b6101dc816101d56101cf5f610110565b916100a5565b141561018e565b608052565b5f1b90565b906101f25f19916101e1565b9181191691161790565b90565b61021361020e610218926101fc565b61010d565b6100a5565b90565b90565b9061023361022e61023a926101ff565b61021b565b82546101e6565b9055565b610246610348565b610255633b9aca00600261021e565b565b60a01b90565b9061026c60ff60a01b91610257565b9181191691161790565b151590565b61028490610276565b90565b90565b9061029f61029a6102a69261027b565b610287565b825461025d565b9055565b5f0190565b6102b761003d565b3d5f823e3d90fd5b60018060a01b031690565b6102de6102d96102e3926102bf565b61010d565b6102bf565b90565b6102ef906102ca565b90565b6102fb906102e6565b90565b9061030f60018060a01b03916101e1565b9181191691161790565b610322906102e6565b90565b90565b9061033d61033861034492610319565b610325565b82546102fe565b9055565b610351336103b5565b61035c5f600161028a565b61036461003d565b6101bf810181811060018060401b038211176103b05761038c82916101bf61255084396102aa565b03905ff080156103ab576103a26103a9916102f2565b6001610328565b565b6102af565b610051565b6103be90610416565b565b6103d46103cf6103d99261010a565b61010d565b6102bf565b90565b6103e5906103c0565b90565b6103f1906102bf565b90565b6103fd906103e8565b9052565b9190610414905f602085019401906103f4565b565b8061043161042b6104265f6103dc565b6103e8565b916103e8565b146104415761043f906104d4565b565b61046461044d5f6103dc565b5f918291631e4fbdf760e01b835260048301610401565b0390fd5b5f1c90565b60018060a01b031690565b61048461048991610468565b61046d565b90565b6104969054610478565b90565b6104a2906102ca565b90565b6104ae90610499565b90565b90565b906104c96104c46104d0926104a5565b6104b1565b82546102fe565b9055565b6104dd5f61048c565b6104e7825f6104b4565b9061051b6105157f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0936104a5565b916104a5565b9161052461003d565b8061052e816102aa565b0390a356fe60806040526004361015610013575b610d0d565b61001d5f356101ac565b8063086146d2146101a757806318d5aafe146101a2578063366cbab71461019d5780633b6ab2a91461019857806346e2cc0914610193578063485cc9551461018e5780634b2c0706146101895780635b3cd6e214610184578063615438011461017f5780636558954f1461017a578063703cfcbb14610175578063715018a6146101705780637a3979dc1461016b578063804e51231461016657806382f44ade146101615780638d5a239b1461015c5780638da5cb5b14610157578063aff74c6d14610152578063c660d3f31461014d578063cdafb97814610148578063d4f0eb4d14610143578063d87813421461013e578063ea4a110414610139578063ede07bd6146101345763f2fde38b0361000e57610cda565b610ca5565b610c34565b610b08565b610ab1565b610a5f565b6109b5565b610980565b61094b565b6108f4565b6108bf565b61088b565b610852565b6107cd565b610798565b610754565b6106e6565b610657565b610589565b610514565b610479565b61043f565b6103ca565b6102a5565b61024e565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f9103126101ca57565b6101bc565b90565b6101db906101cf565b9052565b151590565b6101ed906101df565b9052565b90606080610237936102095f8201515f8601906101d2565b61021b602082015160208601906101d2565b61022d604082015160408601906101d2565b01519101906101e4565b565b919061024c905f608085019401906101f1565b565b3461027e5761025e3660046101c0565b61027a610269610e7e565b6102716101b2565b91829182610239565b0390f35b6101b8565b61028c906101df565b9052565b91906102a3905f60208501940190610283565b565b346102d5576102b53660046101c0565b6102d16102c0610f1a565b6102c86101b2565b91829182610290565b0390f35b6101b8565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156103245781359167ffffffffffffffff831161031f57602001926001830284011161031a57565b6102e6565b6102e2565b6102de565b9060208282031261035a575f82013567ffffffffffffffff81116103555761035192016102ea565b9091565b6102da565b6101bc565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6103a06103a96020936103ae936103978161035f565b93848093610363565b9586910161036c565b610377565b0190565b6103c79160208201915f818403910152610381565b90565b346103fb576103f76103e66103e0366004610329565b90610fbe565b6103ee6101b2565b918291826103b2565b0390f35b6101b8565b1c90565b60ff1690565b61041a90600861041f9302610400565b610404565b90565b9061042d915461040a565b90565b61043c60045f90610422565b90565b3461046f5761044f3660046101c0565b61046b61045a610430565b6104626101b2565b91829182610290565b0390f35b6101b8565b5f0190565b346104a85761049261048c366004610329565b90611165565b61049a6101b2565b806104a481610474565b0390f35b6101b8565b60018060a01b031690565b6104c1906104ad565b90565b6104cd816104b8565b036104d457565b5f80fd5b905035906104e5826104c4565b565b919060408382031261050f578061050361050c925f86016104d8565b936020016104d8565b90565b6101bc565b346105435761052d6105273660046104e7565b90611316565b6105356101b2565b8061053f81610474565b0390f35b6101b8565b610551816101cf565b0361055857565b5f80fd5b9050359061056982610548565b565b9060208282031261058457610581915f0161055c565b90565b6101bc565b346105b9576105b56105a461059f36600461056b565b611322565b6105ac6101b2565b91829182610239565b0390f35b6101b8565b60018060a01b031690565b6105d99060086105de9302610400565b6105be565b90565b906105ec91546105c9565b90565b6105fb60015f906105e1565b90565b90565b61061561061061061a926104ad565b6105fe565b6104ad565b90565b61062690610601565b90565b6106329061061d565b90565b61063e90610629565b9052565b9190610655905f60208501940190610635565b565b34610687576106673660046101c0565b6106836106726105ef565b61067a6101b2565b91829182610642565b0390f35b6101b8565b90565b61069f9060086106a49302610400565b61068c565b90565b906106b2915461068f565b90565b6106c160035f906106a7565b90565b6106cd906101cf565b9052565b91906106e4905f602085019401906106c4565b565b34610716576106f63660046101c0565b6107126107016106b5565b6107096101b2565b918291826106d1565b0390f35b6101b8565b90565b61073261072d6107379261071b565b6105fe565b6101cf565b90565b61074662278d0061071e565b90565b61075161073a565b90565b34610784576107643660046101c0565b61078061076f610749565b6107776101b2565b918291826106d1565b0390f35b6101b8565b61079560025f906106a7565b90565b346107c8576107a83660046101c0565b6107c46107b3610789565b6107bb6101b2565b918291826106d1565b0390f35b6101b8565b346107fb576107dd3660046101c0565b6107e5611366565b6107ed6101b2565b806107f781610474565b0390f35b6101b8565b9160608383031261084d57610817825f85016104d8565b9261082583602083016104d8565b92604082013567ffffffffffffffff81116108485761084492016102ea565b9091565b6102da565b6101bc565b3461088657610882610871610868366004610800565b9291909161141e565b6108796101b2565b91829182610290565b0390f35b6101b8565b346108ba576108a461089e366004610329565b9061157e565b6108ac6101b2565b806108b681610474565b0390f35b6101b8565b346108ef576108cf3660046101c0565b6108eb6108da61159b565b6108e26101b2565b918291826106d1565b0390f35b6101b8565b34610924576109043660046101c0565b61092061090f611668565b6109176101b2565b918291826106d1565b0390f35b6101b8565b610932906104b8565b9052565b9190610949905f60208501940190610929565b565b3461097b5761095b3660046101c0565b6109776109666116fa565b61096e6101b2565b91829182610936565b0390f35b6101b8565b346109b0576109903660046101c0565b6109ac61099b61172e565b6109a36101b2565b918291826106d1565b0390f35b6101b8565b346109e5576109c53660046101c0565b6109e16109d061177a565b6109d86101b2565b918291826106d1565b0390f35b6101b8565b909182601f83011215610a245781359167ffffffffffffffff8311610a1f576020019260208302840111610a1a57565b6102e6565b6102e2565b6102de565b90602082820312610a5a575f82013567ffffffffffffffff8111610a5557610a5192016109ea565b9091565b6102da565b6101bc565b34610a8e57610a78610a72366004610a29565b9061195d565b610a806101b2565b80610a8a81610474565b0390f35b6101b8565b90602082820312610aac57610aa9915f016104d8565b90565b6101bc565b34610adf57610ac9610ac4366004610a93565b611a0d565b610ad16101b2565b80610adb81610474565b0390f35b6101b8565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610b3857610b183660046101c0565b610b34610b23610ae4565b610b2b6101b2565b918291826106d1565b0390f35b6101b8565b610b51610b4c610b56926101cf565b6105fe565b6101cf565b90565b90610b6390610b3d565b5f5260205260405f2090565b5f1c90565b610b80610b8591610b6f565b61068c565b90565b610b929054610b74565b90565b610ba1610ba691610b6f565b610404565b90565b610bb39054610b95565b90565b610bc1906005610b59565b90610bcd5f8301610b88565b91610bda60018201610b88565b91610bf36003610bec60028501610b88565b9301610ba9565b90565b610c2b610c3294610c21606094989795610c17608086019a5f8701906106c4565b60208501906106c4565b60408301906106c4565b0190610283565b565b34610c6857610c64610c4f610c4a36600461056b565b610bb6565b90610c5b9492946101b2565b94859485610bf6565b0390f35b6101b8565b90565b610c84610c7f610c8992610c6d565b6105fe565b6101cf565b90565b610c97611388610c70565b90565b610ca2610c8c565b90565b34610cd557610cb53660046101c0565b610cd1610cc0610c9a565b610cc86101b2565b918291826106d1565b0390f35b6101b8565b34610d0857610cf2610ced366004610a93565b611a7d565b610cfa6101b2565b80610d0481610474565b0390f35b6101b8565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b90610d2f90610377565b810190811067ffffffffffffffff821117610d4957604052565b610d11565b90610d61610d5a6101b2565b9283610d25565b565b610d6d6080610d4e565b90565b5f90565b5f90565b610d80610d63565b90602080808085610d8f610d70565b815201610d9a610d70565b815201610da5610d70565b815201610db0610d74565b81525050565b610dbe610d78565b90565b610dcb6080610d4e565b90565b90565b610de5610de0610dea92610dce565b6105fe565b6101cf565b90565b90610df7906101cf565b9052565b90610e05906101df565b9052565b90610e70610e676003610e1a610d63565b94610e31610e295f8301610b88565b5f8801610ded565b610e49610e4060018301610b88565b60208801610ded565b610e61610e5860028301610b88565b60408801610ded565b01610ba9565b60608401610dfb565b565b610e7b90610e09565b90565b610e86610db6565b50610e9a610e946004610ba9565b156101df565b610ebe57610ebb610eb66005610eb06003610b88565b90610b59565b610e72565b90565b5f610f135f610f0a610f015f610efc610ef35f95610eee610ee6610ee0610dc1565b9a610dd1565b5f8b01610ded565b610dd1565b60208801610ded565b610dd1565b60408501610ded565b60608301610dfb565b90565b5f90565b610f22610f16565b50610f2d6004610ba9565b90565b606090565b60ff60f81b1690565b60f81b90565b610f58610f53610f5d92610dce565b610f3e565b610f35565b90565b90565b610f6f610f7491610f35565b610f60565b9052565b905090565b90825f939282370152565b909182610f9881610f9f93610f78565b8093610f7d565b0190565b80610fb4600192610fbb9694610f63565b0191610f88565b90565b610ffc90610fca610f30565b50610fed610fd75f610f44565b9193610fe16101b2565b94859360208501610fa3565b60208201810382520382610d25565b90565b9061101b6110153332908585919290919261141e565b156101df565b61102a57611028916110a4565b565b5f631b8e828b60e31b81528061104260048201610474565b0390fd5b634e487b7160e01b5f52601160045260245ffd5b61106961106f919392936101cf565b926101cf565b820391821161107a57565b611046565b61108e611094919392936101cf565b926101cf565b820180921161109f57565b611046565b6110c06110ce916110b96110d3945a9261111e565b5a9061105a565b6110c8610c8c565b9061107f565b611ae4565b565b6110de9061061d565b90565b91906110fb816110f48161110095610363565b8095610f7d565b610377565b0190565b909161111b9260208301925f8185039101526110e1565b90565b33909161114b7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926110d5565b926111606111576101b2565b92839283611104565b0390a2565b9061116f91610fff565b565b906111839161117e611b94565b611289565b565b60a01c90565b61119761119c91611185565b610404565b90565b6111a9905461118b565b90565b6111c06111bb6111c592610dce565b6105fe565b6104ad565b90565b6111d1906111ac565b90565b60a01b90565b906111e960ff60a01b916111d4565b9181191691161790565b6111fc906101df565b90565b90565b9061121761121261121e926111f3565b6111ff565b82546111da565b9055565b61122b90610601565b90565b61123790611222565b90565b5f1b90565b9061125060018060a01b039161123a565b9181191691161790565b61126390611222565b90565b90565b9061127e6112796112859261125a565b611266565b825461123f565b9055565b611293600161119f565b6112fb57816112b26112ac6112a75f6111c8565b6104b8565b916104b8565b146112df576112d86112d16112dd936112cc600180611202565b61122e565b6001611269565b611a7d565b565b5f632e7f3c7f60e11b8152806112f760048201610474565b0390fd5b5f62dc149f60e41b81528061131260048201610474565b0390fd5b9061132091611171565b565b61133961133e91611331610db6565b506005610b59565b610e72565b90565b611349611b94565b611351611353565b565b61136461135f5f6111c8565b611c05565b565b61136e611341565b565b61137c61138191610b6f565b6105be565b90565b61138e9054611370565b90565b60e01b90565b6113a0816101df565b036113a757565b5f80fd5b905051906113b882611397565b565b906020828203126113d3576113d0915f016113ab565b90565b6101bc565b6113fe61140b95939492946113f460608401965f850190610929565b6020830190610929565b60408185039101526110e1565b90565b6114166101b2565b3d5f823e3d90fd5b926114616020939461142e610f16565b5061146c61144461143f6001611384565b610629565b93637a3979dc9295976114556101b2565b98899788968796611391565b8652600486016113d8565b03915afa9081156114b0575f91611482575b5090565b6114a3915060203d81116114a9575b61149b8183610d25565b8101906113ba565b5f61147e565b503d611491565b61140e565b906114d16114cb3332908585919290919261141e565b156101df565b6114e0576114de916114fc565b565b5f631b8e828b60e31b8152806114f860048201610474565b0390fd5b6115186115269161151161152b945a9261152d565b5a9061105a565b611520610c8c565b9061107f565b611ae4565b565b90611539903392610fbe565b906115796115677f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926110d5565b926115706101b2565b918291826103b2565b0390a2565b90611588916114b5565b565b5f90565b61159890516101cf565b90565b6115a361158a565b506115b76115b16004610ba9565b156101df565b611627576115f36115e55f6115df6115da60056115d46003610b88565b90610b59565b610e72565b0161158e565b6115ed61073a565b9061107f565b42611606611600836101cf565b916101cf565b101561161a5761161790429061105a565b90565b506116245f610dd1565b90565b6116305f610dd1565b90565b611642611648919392936101cf565b926101cf565b916116548382026101cf565b92818404149015171561166357565b611046565b61167061158a565b5061168461167e6004610ba9565b156101df565b6116be576116bb6116ab60026116a5600561169f6003610b88565b90610b59565b01610b88565b6116b56002610b88565b90611633565b90565b6116c75f610dd1565b90565b5f90565b60018060a01b031690565b6116e56116ea91610b6f565b6116ce565b90565b6116f790546116d9565b90565b6117026116ca565b5061170c5f6116ed565b90565b90565b61172661172161172b9261170f565b6105fe565b6101cf565b90565b61173661158a565b5061174a6117446004610ba9565b156101df565b61176e5761176b61175b6003610b88565b6117656001611712565b9061107f565b90565b6117775f610dd1565b90565b61178261158a565b506117966117906004610ba9565b156101df565b6117bd576117ba60026117b460056117ae6003610b88565b90610b59565b01610b88565b90565b6117c65f610dd1565b90565b6117e56117f3916117de6117f8945a92611894565b5a9061105a565b6117ed610c8c565b9061107f565b611ae4565b565b5090565b600161180a91016101cf565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561186f570180359067ffffffffffffffff821161186a5760200191600182023603831361186557565b611829565b611825565b611821565b9082101561188f57602061188b920281019061182d565b9091565b61180d565b61189f8183906117fa565b916118a861158a565b506118b25f610dd1565b5b806118c66118c0866101cf565b916101cf565b1015611957576118f4906118ea3332906118e287878691611874565b92909161141e565b6118f9575b6117fe565b6118b3565b3361190f61190986868591611874565b90610fbe565b9061194f61193d7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926110d5565b926119466101b2565b918291826103b2565b0390a26118ef565b50505050565b90611967916117c9565b565b61197a90611975611b94565b61197c565b565b8061199761199161198c5f6111c8565b6104b8565b916104b8565b146119f1576119af6119a88261122e565b6001611269565b6119d97f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b9916110d5565b906119e26101b2565b806119ec81610474565b0390a2565b5f632e7f3c7f60e11b815280611a0960048201610474565b0390fd5b611a1690611969565b565b611a2990611a24611b94565b611a2b565b565b80611a46611a40611a3b5f6111c8565b6104b8565b916104b8565b14611a5657611a5490611c05565b565b611a79611a625f6111c8565b5f918291631e4fbdf760e01b835260048301610936565b0390fd5b611a8690611a18565b565b90611a945f199161123a565b9181191691161790565b90565b90611ab6611ab1611abd92610b3d565b611a9e565b8254611a88565b9055565b916020611ae2929493611adb60408201965f8301906106c4565b01906106c4565b565b611af7611af16004610ba9565b156101df565b611b87575b611b04611e31565b611b3881611b326002611b226005611b1c6003610b88565b90610b59565b0191611b2d83610b88565b61107f565b90611aa1565b611b426003610b88565b3a611b6d7f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610b3d565b92611b82611b796101b2565b92839283611ac1565b0390a2565b611b8f611d2e565b611afc565b611b9c6116fa565b611bb5611baf611baa61200f565b6104b8565b916104b8565b03611bbc57565b611bde611bc761200f565b5f91829163118cdaa760e01b835260048301610936565b0390fd5b90565b90611bfa611bf5611c01926110d5565b611be2565b825461123f565b9055565b611c0e5f6116ed565b611c18825f611be5565b90611c4c611c467f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0936110d5565b916110d5565b91611c556101b2565b80611c5f81610474565b0390a3565b90611c7060ff9161123a565b9181191691161790565b90611c8f611c8a611c96926111f3565b6111ff565b8254611c64565b9055565b90611ca490610dd1565b5f5260205260405f2090565b611cba90516101df565b90565b90611d1a60606003611d2094611ce05f8201611cda5f880161158e565b90611aa1565b611cf960018201611cf36020880161158e565b90611aa1565b611d1260028201611d0c6040880161158e565b90611aa1565b019201611cb0565b90611c7a565b565b90611d2c91611cbd565b565b611d41611d3b6004610ba9565b156101df565b611d48575b565b611d5460016004611c7a565b611d67611d605f610dd1565b6003611aa1565b611dc842611db75f611dae611da55f611da0611d975f95611d92611d89610dc1565b995f8b01610ded565b610dd1565b60208801610ded565b610dd1565b60408501610ded565b60608301610dfb565b611dc360055f90611c9a565b611d22565b5f4290611e0a611df87f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610dd1565b92611e016101b2565b918291826106d1565b0390a2611d46565b90565b611e1e906101cf565b5f198114611e2c5760010190565b611046565b611e4e611e496005611e436003610b88565b90610b59565b611e12565b42611e7c611e76611e71611e635f8601610b88565b611e6b61073a565b9061107f565b6101cf565b916101cf565b1015611e86575b50565b611eae611ea5611e975f8401610b88565b611e9f61073a565b9061107f565b60018301611aa1565b611ebc600160038301611c7a565b611ec66003610b88565b611ef3611ed560028401610b88565b92611eed5f611ee660018401610b88565b9201610b88565b9061105a565b611f1d7f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610b3d565b92611f32611f296101b2565b92839283611ac1565b0390a2611f51611f4a611f456003610b88565b611e15565b6003611aa1565b611fbb42611fa15f611f98611f8f5f611f8a611f815f95611f7c611f73610dc1565b995f8b01610ded565b610dd1565b60208801610ded565b610dd1565b60408501610ded565b60608301610dfb565b611fb66005611fb06003610b88565b90610b59565b611d22565b611fc56003610b88565b4290612006611ff47f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610b3d565b92611ffd6101b2565b918291826106d1565b0390a25f611e83565b6120176116ca565b50339056608060405234601c57600e6020565b61019461002b823961019490f35b6026565b60405190565b5f80fdfe60806040526004361015610013575b61017b565b61001d5f3561002c565b637a3979dc0361000e57610142565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b60018060a01b031690565b61005890610044565b90565b6100648161004f565b0361006b57565b5f80fd5b9050359061007c8261005b565b565b5f80fd5b5f80fd5b5f80fd5b909182601f830112156100c45781359167ffffffffffffffff83116100bf5760200192600183028401116100ba57565b610086565b610082565b61007e565b91606083830312610116576100e0825f850161006f565b926100ee836020830161006f565b92604082013567ffffffffffffffff81116101115761010d920161008a565b9091565b610040565b61003c565b151590565b6101299061011b565b9052565b9190610140905f60208501940190610120565b565b34610176576101726101616101583660046100c9565b92919091610183565b610169610032565b9182918261012d565b0390f35b610038565b5f80fd5b5f90565b5050505061018f61017f565b505f9056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4a\08Wa\0\x19a\0\x14a\0\xE9V[a\x01\xB7V[a\0!a\0=V[a \x1Ca\x054\x829`\x80Q\x81a\n\xE6\x01Ra \x1C\x90\xF3[a\0CV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0o\x90a\0GV[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x87W`@RV[a\0QV[\x90a\0\x9Fa\0\x98a\0=V[\x92\x83a\0eV[V[_\x80\xFD[\x90V[a\0\xB1\x81a\0\xA5V[\x03a\0\xB8WV[_\x80\xFD[\x90PQ\x90a\0\xC9\x82a\0\xA8V[V[\x90` \x82\x82\x03\x12a\0\xE4Wa\0\xE1\x91_\x01a\0\xBCV[\x90V[a\0\xA1V[a\x01\x07a'\x0F\x808\x03\x80a\0\xFC\x81a\0\x8CV[\x92\x839\x81\x01\x90a\0\xCBV[\x90V[\x90V[\x90V[a\x01$a\x01\x1Fa\x01)\x92a\x01\nV[a\x01\rV[a\0\xA5V[\x90V[` \x91\x81R\x01\x90V[_\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01i`\x18` \x92a\x01,V[a\x01r\x81a\x015V[\x01\x90V[a\x01\x8B\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x01\\V[\x90V[\x15a\x01\x95WV[a\x01\x9Da\0=V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x01\xB3`\x04\x82\x01a\x01vV[\x03\x90\xFD[a\x01\xBFa\x02>V[a\x01\xDC\x81a\x01\xD5a\x01\xCF_a\x01\x10V[\x91a\0\xA5V[\x14\x15a\x01\x8EV[`\x80RV[_\x1B\x90V[\x90a\x01\xF2_\x19\x91a\x01\xE1V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[a\x02\x13a\x02\x0Ea\x02\x18\x92a\x01\xFCV[a\x01\rV[a\0\xA5V[\x90V[\x90V[\x90a\x023a\x02.a\x02:\x92a\x01\xFFV[a\x02\x1BV[\x82Ta\x01\xE6V[\x90UV[a\x02Fa\x03HV[a\x02Uc;\x9A\xCA\0`\x02a\x02\x1EV[V[`\xA0\x1B\x90V[\x90a\x02l`\xFF`\xA0\x1B\x91a\x02WV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x15\x15\x90V[a\x02\x84\x90a\x02vV[\x90V[\x90V[\x90a\x02\x9Fa\x02\x9Aa\x02\xA6\x92a\x02{V[a\x02\x87V[\x82Ta\x02]V[\x90UV[_\x01\x90V[a\x02\xB7a\0=V[=_\x82>=\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x02\xDEa\x02\xD9a\x02\xE3\x92a\x02\xBFV[a\x01\rV[a\x02\xBFV[\x90V[a\x02\xEF\x90a\x02\xCAV[\x90V[a\x02\xFB\x90a\x02\xE6V[\x90V[\x90a\x03\x0F`\x01\x80`\xA0\x1B\x03\x91a\x01\xE1V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x03\"\x90a\x02\xE6V[\x90V[\x90V[\x90a\x03=a\x038a\x03D\x92a\x03\x19V[a\x03%V[\x82Ta\x02\xFEV[\x90UV[a\x03Q3a\x03\xB5V[a\x03\\_`\x01a\x02\x8AV[a\x03da\0=V[a\x01\xBF\x81\x01\x81\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\x03\xB0Wa\x03\x8C\x82\x91a\x01\xBFa%P\x849a\x02\xAAV[\x03\x90_\xF0\x80\x15a\x03\xABWa\x03\xA2a\x03\xA9\x91a\x02\xF2V[`\x01a\x03(V[V[a\x02\xAFV[a\0QV[a\x03\xBE\x90a\x04\x16V[V[a\x03\xD4a\x03\xCFa\x03\xD9\x92a\x01\nV[a\x01\rV[a\x02\xBFV[\x90V[a\x03\xE5\x90a\x03\xC0V[\x90V[a\x03\xF1\x90a\x02\xBFV[\x90V[a\x03\xFD\x90a\x03\xE8V[\x90RV[\x91\x90a\x04\x14\x90_` \x85\x01\x94\x01\x90a\x03\xF4V[V[\x80a\x041a\x04+a\x04&_a\x03\xDCV[a\x03\xE8V[\x91a\x03\xE8V[\x14a\x04AWa\x04?\x90a\x04\xD4V[V[a\x04da\x04M_a\x03\xDCV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x04\x01V[\x03\x90\xFD[_\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\x84a\x04\x89\x91a\x04hV[a\x04mV[\x90V[a\x04\x96\x90Ta\x04xV[\x90V[a\x04\xA2\x90a\x02\xCAV[\x90V[a\x04\xAE\x90a\x04\x99V[\x90V[\x90V[\x90a\x04\xC9a\x04\xC4a\x04\xD0\x92a\x04\xA5V[a\x04\xB1V[\x82Ta\x02\xFEV[\x90UV[a\x04\xDD_a\x04\x8CV[a\x04\xE7\x82_a\x04\xB4V[\x90a\x05\x1Ba\x05\x15\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x04\xA5V[\x91a\x04\xA5V[\x91a\x05$a\0=V[\x80a\x05.\x81a\x02\xAAV[\x03\x90\xA3V\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\r\rV[a\0\x1D_5a\x01\xACV[\x80c\x08aF\xD2\x14a\x01\xA7W\x80c\x18\xD5\xAA\xFE\x14a\x01\xA2W\x80c6l\xBA\xB7\x14a\x01\x9DW\x80c;j\xB2\xA9\x14a\x01\x98W\x80cF\xE2\xCC\t\x14a\x01\x93W\x80cH\\\xC9U\x14a\x01\x8EW\x80cK,\x07\x06\x14a\x01\x89W\x80c[<\xD6\xE2\x14a\x01\x84W\x80caT8\x01\x14a\x01\x7FW\x80ceX\x95O\x14a\x01zW\x80cp<\xFC\xBB\x14a\x01uW\x80cqP\x18\xA6\x14a\x01pW\x80cz9y\xDC\x14a\x01kW\x80c\x80NQ#\x14a\x01fW\x80c\x82\xF4J\xDE\x14a\x01aW\x80c\x8DZ#\x9B\x14a\x01\\W\x80c\x8D\xA5\xCB[\x14a\x01WW\x80c\xAF\xF7Lm\x14a\x01RW\x80c\xC6`\xD3\xF3\x14a\x01MW\x80c\xCD\xAF\xB9x\x14a\x01HW\x80c\xD4\xF0\xEBM\x14a\x01CW\x80c\xD8x\x13B\x14a\x01>W\x80c\xEAJ\x11\x04\x14a\x019W\x80c\xED\xE0{\xD6\x14a\x014Wc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\x0C\xDAV[a\x0C\xA5V[a\x0C4V[a\x0B\x08V[a\n\xB1V[a\n_V[a\t\xB5V[a\t\x80V[a\tKV[a\x08\xF4V[a\x08\xBFV[a\x08\x8BV[a\x08RV[a\x07\xCDV[a\x07\x98V[a\x07TV[a\x06\xE6V[a\x06WV[a\x05\x89V[a\x05\x14V[a\x04yV[a\x04?V[a\x03\xCAV[a\x02\xA5V[a\x02NV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01\xCAWV[a\x01\xBCV[\x90V[a\x01\xDB\x90a\x01\xCFV[\x90RV[\x15\x15\x90V[a\x01\xED\x90a\x01\xDFV[\x90RV[\x90``\x80a\x027\x93a\x02\t_\x82\x01Q_\x86\x01\x90a\x01\xD2V[a\x02\x1B` \x82\x01Q` \x86\x01\x90a\x01\xD2V[a\x02-`@\x82\x01Q`@\x86\x01\x90a\x01\xD2V[\x01Q\x91\x01\x90a\x01\xE4V[V[\x91\x90a\x02L\x90_`\x80\x85\x01\x94\x01\x90a\x01\xF1V[V[4a\x02~Wa\x02^6`\x04a\x01\xC0V[a\x02za\x02ia\x0E~V[a\x02qa\x01\xB2V[\x91\x82\x91\x82a\x029V[\x03\x90\xF3[a\x01\xB8V[a\x02\x8C\x90a\x01\xDFV[\x90RV[\x91\x90a\x02\xA3\x90_` \x85\x01\x94\x01\x90a\x02\x83V[V[4a\x02\xD5Wa\x02\xB56`\x04a\x01\xC0V[a\x02\xD1a\x02\xC0a\x0F\x1AV[a\x02\xC8a\x01\xB2V[\x91\x82\x91\x82a\x02\x90V[\x03\x90\xF3[a\x01\xB8V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03$W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\x1FW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x03\x1AWV[a\x02\xE6V[a\x02\xE2V[a\x02\xDEV[\x90` \x82\x82\x03\x12a\x03ZW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03UWa\x03Q\x92\x01a\x02\xEAV[\x90\x91V[a\x02\xDAV[a\x01\xBCV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\xA0a\x03\xA9` \x93a\x03\xAE\x93a\x03\x97\x81a\x03_V[\x93\x84\x80\x93a\x03cV[\x95\x86\x91\x01a\x03lV[a\x03wV[\x01\x90V[a\x03\xC7\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\x81V[\x90V[4a\x03\xFBWa\x03\xF7a\x03\xE6a\x03\xE06`\x04a\x03)V[\x90a\x0F\xBEV[a\x03\xEEa\x01\xB2V[\x91\x82\x91\x82a\x03\xB2V[\x03\x90\xF3[a\x01\xB8V[\x1C\x90V[`\xFF\x16\x90V[a\x04\x1A\x90`\x08a\x04\x1F\x93\x02a\x04\0V[a\x04\x04V[\x90V[\x90a\x04-\x91Ta\x04\nV[\x90V[a\x04<`\x04_\x90a\x04\"V[\x90V[4a\x04oWa\x04O6`\x04a\x01\xC0V[a\x04ka\x04Za\x040V[a\x04ba\x01\xB2V[\x91\x82\x91\x82a\x02\x90V[\x03\x90\xF3[a\x01\xB8V[_\x01\x90V[4a\x04\xA8Wa\x04\x92a\x04\x8C6`\x04a\x03)V[\x90a\x11eV[a\x04\x9Aa\x01\xB2V[\x80a\x04\xA4\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\xC1\x90a\x04\xADV[\x90V[a\x04\xCD\x81a\x04\xB8V[\x03a\x04\xD4WV[_\x80\xFD[\x90P5\x90a\x04\xE5\x82a\x04\xC4V[V[\x91\x90`@\x83\x82\x03\x12a\x05\x0FW\x80a\x05\x03a\x05\x0C\x92_\x86\x01a\x04\xD8V[\x93` \x01a\x04\xD8V[\x90V[a\x01\xBCV[4a\x05CWa\x05-a\x05'6`\x04a\x04\xE7V[\x90a\x13\x16V[a\x055a\x01\xB2V[\x80a\x05?\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[a\x05Q\x81a\x01\xCFV[\x03a\x05XWV[_\x80\xFD[\x90P5\x90a\x05i\x82a\x05HV[V[\x90` \x82\x82\x03\x12a\x05\x84Wa\x05\x81\x91_\x01a\x05\\V[\x90V[a\x01\xBCV[4a\x05\xB9Wa\x05\xB5a\x05\xA4a\x05\x9F6`\x04a\x05kV[a\x13\"V[a\x05\xACa\x01\xB2V[\x91\x82\x91\x82a\x029V[\x03\x90\xF3[a\x01\xB8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x05\xD9\x90`\x08a\x05\xDE\x93\x02a\x04\0V[a\x05\xBEV[\x90V[\x90a\x05\xEC\x91Ta\x05\xC9V[\x90V[a\x05\xFB`\x01_\x90a\x05\xE1V[\x90V[\x90V[a\x06\x15a\x06\x10a\x06\x1A\x92a\x04\xADV[a\x05\xFEV[a\x04\xADV[\x90V[a\x06&\x90a\x06\x01V[\x90V[a\x062\x90a\x06\x1DV[\x90V[a\x06>\x90a\x06)V[\x90RV[\x91\x90a\x06U\x90_` \x85\x01\x94\x01\x90a\x065V[V[4a\x06\x87Wa\x06g6`\x04a\x01\xC0V[a\x06\x83a\x06ra\x05\xEFV[a\x06za\x01\xB2V[\x91\x82\x91\x82a\x06BV[\x03\x90\xF3[a\x01\xB8V[\x90V[a\x06\x9F\x90`\x08a\x06\xA4\x93\x02a\x04\0V[a\x06\x8CV[\x90V[\x90a\x06\xB2\x91Ta\x06\x8FV[\x90V[a\x06\xC1`\x03_\x90a\x06\xA7V[\x90V[a\x06\xCD\x90a\x01\xCFV[\x90RV[\x91\x90a\x06\xE4\x90_` \x85\x01\x94\x01\x90a\x06\xC4V[V[4a\x07\x16Wa\x06\xF66`\x04a\x01\xC0V[a\x07\x12a\x07\x01a\x06\xB5V[a\x07\ta\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[\x90V[a\x072a\x07-a\x077\x92a\x07\x1BV[a\x05\xFEV[a\x01\xCFV[\x90V[a\x07Fb'\x8D\0a\x07\x1EV[\x90V[a\x07Qa\x07:V[\x90V[4a\x07\x84Wa\x07d6`\x04a\x01\xC0V[a\x07\x80a\x07oa\x07IV[a\x07wa\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[a\x07\x95`\x02_\x90a\x06\xA7V[\x90V[4a\x07\xC8Wa\x07\xA86`\x04a\x01\xC0V[a\x07\xC4a\x07\xB3a\x07\x89V[a\x07\xBBa\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[4a\x07\xFBWa\x07\xDD6`\x04a\x01\xC0V[a\x07\xE5a\x13fV[a\x07\xEDa\x01\xB2V[\x80a\x07\xF7\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[\x91``\x83\x83\x03\x12a\x08MWa\x08\x17\x82_\x85\x01a\x04\xD8V[\x92a\x08%\x83` \x83\x01a\x04\xD8V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08HWa\x08D\x92\x01a\x02\xEAV[\x90\x91V[a\x02\xDAV[a\x01\xBCV[4a\x08\x86Wa\x08\x82a\x08qa\x08h6`\x04a\x08\0V[\x92\x91\x90\x91a\x14\x1EV[a\x08ya\x01\xB2V[\x91\x82\x91\x82a\x02\x90V[\x03\x90\xF3[a\x01\xB8V[4a\x08\xBAWa\x08\xA4a\x08\x9E6`\x04a\x03)V[\x90a\x15~V[a\x08\xACa\x01\xB2V[\x80a\x08\xB6\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[4a\x08\xEFWa\x08\xCF6`\x04a\x01\xC0V[a\x08\xEBa\x08\xDAa\x15\x9BV[a\x08\xE2a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[4a\t$Wa\t\x046`\x04a\x01\xC0V[a\t a\t\x0Fa\x16hV[a\t\x17a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[a\t2\x90a\x04\xB8V[\x90RV[\x91\x90a\tI\x90_` \x85\x01\x94\x01\x90a\t)V[V[4a\t{Wa\t[6`\x04a\x01\xC0V[a\twa\tfa\x16\xFAV[a\tna\x01\xB2V[\x91\x82\x91\x82a\t6V[\x03\x90\xF3[a\x01\xB8V[4a\t\xB0Wa\t\x906`\x04a\x01\xC0V[a\t\xACa\t\x9Ba\x17.V[a\t\xA3a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[4a\t\xE5Wa\t\xC56`\x04a\x01\xC0V[a\t\xE1a\t\xD0a\x17zV[a\t\xD8a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\n$W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\x1FW` \x01\x92` \x83\x02\x84\x01\x11a\n\x1AWV[a\x02\xE6V[a\x02\xE2V[a\x02\xDEV[\x90` \x82\x82\x03\x12a\nZW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\nUWa\nQ\x92\x01a\t\xEAV[\x90\x91V[a\x02\xDAV[a\x01\xBCV[4a\n\x8EWa\nxa\nr6`\x04a\n)V[\x90a\x19]V[a\n\x80a\x01\xB2V[\x80a\n\x8A\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[\x90` \x82\x82\x03\x12a\n\xACWa\n\xA9\x91_\x01a\x04\xD8V[\x90V[a\x01\xBCV[4a\n\xDFWa\n\xC9a\n\xC46`\x04a\n\x93V[a\x1A\rV[a\n\xD1a\x01\xB2V[\x80a\n\xDB\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B8Wa\x0B\x186`\x04a\x01\xC0V[a\x0B4a\x0B#a\n\xE4V[a\x0B+a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[a\x0BQa\x0BLa\x0BV\x92a\x01\xCFV[a\x05\xFEV[a\x01\xCFV[\x90V[\x90a\x0Bc\x90a\x0B=V[_R` R`@_ \x90V[_\x1C\x90V[a\x0B\x80a\x0B\x85\x91a\x0BoV[a\x06\x8CV[\x90V[a\x0B\x92\x90Ta\x0BtV[\x90V[a\x0B\xA1a\x0B\xA6\x91a\x0BoV[a\x04\x04V[\x90V[a\x0B\xB3\x90Ta\x0B\x95V[\x90V[a\x0B\xC1\x90`\x05a\x0BYV[\x90a\x0B\xCD_\x83\x01a\x0B\x88V[\x91a\x0B\xDA`\x01\x82\x01a\x0B\x88V[\x91a\x0B\xF3`\x03a\x0B\xEC`\x02\x85\x01a\x0B\x88V[\x93\x01a\x0B\xA9V[\x90V[a\x0C+a\x0C2\x94a\x0C!``\x94\x98\x97\x95a\x0C\x17`\x80\x86\x01\x9A_\x87\x01\x90a\x06\xC4V[` \x85\x01\x90a\x06\xC4V[`@\x83\x01\x90a\x06\xC4V[\x01\x90a\x02\x83V[V[4a\x0ChWa\x0Cda\x0COa\x0CJ6`\x04a\x05kV[a\x0B\xB6V[\x90a\x0C[\x94\x92\x94a\x01\xB2V[\x94\x85\x94\x85a\x0B\xF6V[\x03\x90\xF3[a\x01\xB8V[\x90V[a\x0C\x84a\x0C\x7Fa\x0C\x89\x92a\x0CmV[a\x05\xFEV[a\x01\xCFV[\x90V[a\x0C\x97a\x13\x88a\x0CpV[\x90V[a\x0C\xA2a\x0C\x8CV[\x90V[4a\x0C\xD5Wa\x0C\xB56`\x04a\x01\xC0V[a\x0C\xD1a\x0C\xC0a\x0C\x9AV[a\x0C\xC8a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[4a\r\x08Wa\x0C\xF2a\x0C\xED6`\x04a\n\x93V[a\x1A}V[a\x0C\xFAa\x01\xB2V[\x80a\r\x04\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\r/\x90a\x03wV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rIW`@RV[a\r\x11V[\x90a\raa\rZa\x01\xB2V[\x92\x83a\r%V[V[a\rm`\x80a\rNV[\x90V[_\x90V[_\x90V[a\r\x80a\rcV[\x90` \x80\x80\x80\x85a\r\x8Fa\rpV[\x81R\x01a\r\x9Aa\rpV[\x81R\x01a\r\xA5a\rpV[\x81R\x01a\r\xB0a\rtV[\x81RPPV[a\r\xBEa\rxV[\x90V[a\r\xCB`\x80a\rNV[\x90V[\x90V[a\r\xE5a\r\xE0a\r\xEA\x92a\r\xCEV[a\x05\xFEV[a\x01\xCFV[\x90V[\x90a\r\xF7\x90a\x01\xCFV[\x90RV[\x90a\x0E\x05\x90a\x01\xDFV[\x90RV[\x90a\x0Epa\x0Eg`\x03a\x0E\x1Aa\rcV[\x94a\x0E1a\x0E)_\x83\x01a\x0B\x88V[_\x88\x01a\r\xEDV[a\x0EIa\x0E@`\x01\x83\x01a\x0B\x88V[` \x88\x01a\r\xEDV[a\x0Eaa\x0EX`\x02\x83\x01a\x0B\x88V[`@\x88\x01a\r\xEDV[\x01a\x0B\xA9V[``\x84\x01a\r\xFBV[V[a\x0E{\x90a\x0E\tV[\x90V[a\x0E\x86a\r\xB6V[Pa\x0E\x9Aa\x0E\x94`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x0E\xBEWa\x0E\xBBa\x0E\xB6`\x05a\x0E\xB0`\x03a\x0B\x88V[\x90a\x0BYV[a\x0ErV[\x90V[_a\x0F\x13_a\x0F\na\x0F\x01_a\x0E\xFCa\x0E\xF3_\x95a\x0E\xEEa\x0E\xE6a\x0E\xE0a\r\xC1V[\x9Aa\r\xD1V[_\x8B\x01a\r\xEDV[a\r\xD1V[` \x88\x01a\r\xEDV[a\r\xD1V[`@\x85\x01a\r\xEDV[``\x83\x01a\r\xFBV[\x90V[_\x90V[a\x0F\"a\x0F\x16V[Pa\x0F-`\x04a\x0B\xA9V[\x90V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x0FXa\x0FSa\x0F]\x92a\r\xCEV[a\x0F>V[a\x0F5V[\x90V[\x90V[a\x0Foa\x0Ft\x91a\x0F5V[a\x0F`V[\x90RV[\x90P\x90V[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x0F\x98\x81a\x0F\x9F\x93a\x0FxV[\x80\x93a\x0F}V[\x01\x90V[\x80a\x0F\xB4`\x01\x92a\x0F\xBB\x96\x94a\x0FcV[\x01\x91a\x0F\x88V[\x90V[a\x0F\xFC\x90a\x0F\xCAa\x0F0V[Pa\x0F\xEDa\x0F\xD7_a\x0FDV[\x91\x93a\x0F\xE1a\x01\xB2V[\x94\x85\x93` \x85\x01a\x0F\xA3V[` \x82\x01\x81\x03\x82R\x03\x82a\r%V[\x90V[\x90a\x10\x1Ba\x10\x1532\x90\x85\x85\x91\x92\x90\x91\x92a\x14\x1EV[\x15a\x01\xDFV[a\x10*Wa\x10(\x91a\x10\xA4V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x10B`\x04\x82\x01a\x04tV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x10ia\x10o\x91\x93\x92\x93a\x01\xCFV[\x92a\x01\xCFV[\x82\x03\x91\x82\x11a\x10zWV[a\x10FV[a\x10\x8Ea\x10\x94\x91\x93\x92\x93a\x01\xCFV[\x92a\x01\xCFV[\x82\x01\x80\x92\x11a\x10\x9FWV[a\x10FV[a\x10\xC0a\x10\xCE\x91a\x10\xB9a\x10\xD3\x94Z\x92a\x11\x1EV[Z\x90a\x10ZV[a\x10\xC8a\x0C\x8CV[\x90a\x10\x7FV[a\x1A\xE4V[V[a\x10\xDE\x90a\x06\x1DV[\x90V[\x91\x90a\x10\xFB\x81a\x10\xF4\x81a\x11\0\x95a\x03cV[\x80\x95a\x0F}V[a\x03wV[\x01\x90V[\x90\x91a\x11\x1B\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x10\xE1V[\x90V[3\x90\x91a\x11K\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\xD5V[\x92a\x11`a\x11Wa\x01\xB2V[\x92\x83\x92\x83a\x11\x04V[\x03\x90\xA2V[\x90a\x11o\x91a\x0F\xFFV[V[\x90a\x11\x83\x91a\x11~a\x1B\x94V[a\x12\x89V[V[`\xA0\x1C\x90V[a\x11\x97a\x11\x9C\x91a\x11\x85V[a\x04\x04V[\x90V[a\x11\xA9\x90Ta\x11\x8BV[\x90V[a\x11\xC0a\x11\xBBa\x11\xC5\x92a\r\xCEV[a\x05\xFEV[a\x04\xADV[\x90V[a\x11\xD1\x90a\x11\xACV[\x90V[`\xA0\x1B\x90V[\x90a\x11\xE9`\xFF`\xA0\x1B\x91a\x11\xD4V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x11\xFC\x90a\x01\xDFV[\x90V[\x90V[\x90a\x12\x17a\x12\x12a\x12\x1E\x92a\x11\xF3V[a\x11\xFFV[\x82Ta\x11\xDAV[\x90UV[a\x12+\x90a\x06\x01V[\x90V[a\x127\x90a\x12\"V[\x90V[_\x1B\x90V[\x90a\x12P`\x01\x80`\xA0\x1B\x03\x91a\x12:V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x12c\x90a\x12\"V[\x90V[\x90V[\x90a\x12~a\x12ya\x12\x85\x92a\x12ZV[a\x12fV[\x82Ta\x12?V[\x90UV[a\x12\x93`\x01a\x11\x9FV[a\x12\xFBW\x81a\x12\xB2a\x12\xACa\x12\xA7_a\x11\xC8V[a\x04\xB8V[\x91a\x04\xB8V[\x14a\x12\xDFWa\x12\xD8a\x12\xD1a\x12\xDD\x93a\x12\xCC`\x01\x80a\x12\x02V[a\x12.V[`\x01a\x12iV[a\x1A}V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x12\xF7`\x04\x82\x01a\x04tV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x13\x12`\x04\x82\x01a\x04tV[\x03\x90\xFD[\x90a\x13 \x91a\x11qV[V[a\x139a\x13>\x91a\x131a\r\xB6V[P`\x05a\x0BYV[a\x0ErV[\x90V[a\x13Ia\x1B\x94V[a\x13Qa\x13SV[V[a\x13da\x13__a\x11\xC8V[a\x1C\x05V[V[a\x13na\x13AV[V[a\x13|a\x13\x81\x91a\x0BoV[a\x05\xBEV[\x90V[a\x13\x8E\x90Ta\x13pV[\x90V[`\xE0\x1B\x90V[a\x13\xA0\x81a\x01\xDFV[\x03a\x13\xA7WV[_\x80\xFD[\x90PQ\x90a\x13\xB8\x82a\x13\x97V[V[\x90` \x82\x82\x03\x12a\x13\xD3Wa\x13\xD0\x91_\x01a\x13\xABV[\x90V[a\x01\xBCV[a\x13\xFEa\x14\x0B\x95\x93\x94\x92\x94a\x13\xF4``\x84\x01\x96_\x85\x01\x90a\t)V[` \x83\x01\x90a\t)V[`@\x81\x85\x03\x91\x01Ra\x10\xE1V[\x90V[a\x14\x16a\x01\xB2V[=_\x82>=\x90\xFD[\x92a\x14a` \x93\x94a\x14.a\x0F\x16V[Pa\x14la\x14Da\x14?`\x01a\x13\x84V[a\x06)V[\x93cz9y\xDC\x92\x95\x97a\x14Ua\x01\xB2V[\x98\x89\x97\x88\x96\x87\x96a\x13\x91V[\x86R`\x04\x86\x01a\x13\xD8V[\x03\x91Z\xFA\x90\x81\x15a\x14\xB0W_\x91a\x14\x82W[P\x90V[a\x14\xA3\x91P` =\x81\x11a\x14\xA9W[a\x14\x9B\x81\x83a\r%V[\x81\x01\x90a\x13\xBAV[_a\x14~V[P=a\x14\x91V[a\x14\x0EV[\x90a\x14\xD1a\x14\xCB32\x90\x85\x85\x91\x92\x90\x91\x92a\x14\x1EV[\x15a\x01\xDFV[a\x14\xE0Wa\x14\xDE\x91a\x14\xFCV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x14\xF8`\x04\x82\x01a\x04tV[\x03\x90\xFD[a\x15\x18a\x15&\x91a\x15\x11a\x15+\x94Z\x92a\x15-V[Z\x90a\x10ZV[a\x15 a\x0C\x8CV[\x90a\x10\x7FV[a\x1A\xE4V[V[\x90a\x159\x903\x92a\x0F\xBEV[\x90a\x15ya\x15g\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\xD5V[\x92a\x15pa\x01\xB2V[\x91\x82\x91\x82a\x03\xB2V[\x03\x90\xA2V[\x90a\x15\x88\x91a\x14\xB5V[V[_\x90V[a\x15\x98\x90Qa\x01\xCFV[\x90V[a\x15\xA3a\x15\x8AV[Pa\x15\xB7a\x15\xB1`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x16'Wa\x15\xF3a\x15\xE5_a\x15\xDFa\x15\xDA`\x05a\x15\xD4`\x03a\x0B\x88V[\x90a\x0BYV[a\x0ErV[\x01a\x15\x8EV[a\x15\xEDa\x07:V[\x90a\x10\x7FV[Ba\x16\x06a\x16\0\x83a\x01\xCFV[\x91a\x01\xCFV[\x10\x15a\x16\x1AWa\x16\x17\x90B\x90a\x10ZV[\x90V[Pa\x16$_a\r\xD1V[\x90V[a\x160_a\r\xD1V[\x90V[a\x16Ba\x16H\x91\x93\x92\x93a\x01\xCFV[\x92a\x01\xCFV[\x91a\x16T\x83\x82\x02a\x01\xCFV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x16cWV[a\x10FV[a\x16pa\x15\x8AV[Pa\x16\x84a\x16~`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x16\xBEWa\x16\xBBa\x16\xAB`\x02a\x16\xA5`\x05a\x16\x9F`\x03a\x0B\x88V[\x90a\x0BYV[\x01a\x0B\x88V[a\x16\xB5`\x02a\x0B\x88V[\x90a\x163V[\x90V[a\x16\xC7_a\r\xD1V[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x16\xE5a\x16\xEA\x91a\x0BoV[a\x16\xCEV[\x90V[a\x16\xF7\x90Ta\x16\xD9V[\x90V[a\x17\x02a\x16\xCAV[Pa\x17\x0C_a\x16\xEDV[\x90V[\x90V[a\x17&a\x17!a\x17+\x92a\x17\x0FV[a\x05\xFEV[a\x01\xCFV[\x90V[a\x176a\x15\x8AV[Pa\x17Ja\x17D`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x17nWa\x17ka\x17[`\x03a\x0B\x88V[a\x17e`\x01a\x17\x12V[\x90a\x10\x7FV[\x90V[a\x17w_a\r\xD1V[\x90V[a\x17\x82a\x15\x8AV[Pa\x17\x96a\x17\x90`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x17\xBDWa\x17\xBA`\x02a\x17\xB4`\x05a\x17\xAE`\x03a\x0B\x88V[\x90a\x0BYV[\x01a\x0B\x88V[\x90V[a\x17\xC6_a\r\xD1V[\x90V[a\x17\xE5a\x17\xF3\x91a\x17\xDEa\x17\xF8\x94Z\x92a\x18\x94V[Z\x90a\x10ZV[a\x17\xEDa\x0C\x8CV[\x90a\x10\x7FV[a\x1A\xE4V[V[P\x90V[`\x01a\x18\n\x91\x01a\x01\xCFV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x18oW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x18jW` \x01\x91`\x01\x82\x026\x03\x83\x13a\x18eWV[a\x18)V[a\x18%V[a\x18!V[\x90\x82\x10\x15a\x18\x8FW` a\x18\x8B\x92\x02\x81\x01\x90a\x18-V[\x90\x91V[a\x18\rV[a\x18\x9F\x81\x83\x90a\x17\xFAV[\x91a\x18\xA8a\x15\x8AV[Pa\x18\xB2_a\r\xD1V[[\x80a\x18\xC6a\x18\xC0\x86a\x01\xCFV[\x91a\x01\xCFV[\x10\x15a\x19WWa\x18\xF4\x90a\x18\xEA32\x90a\x18\xE2\x87\x87\x86\x91a\x18tV[\x92\x90\x91a\x14\x1EV[a\x18\xF9W[a\x17\xFEV[a\x18\xB3V[3a\x19\x0Fa\x19\t\x86\x86\x85\x91a\x18tV[\x90a\x0F\xBEV[\x90a\x19Oa\x19=\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\xD5V[\x92a\x19Fa\x01\xB2V[\x91\x82\x91\x82a\x03\xB2V[\x03\x90\xA2a\x18\xEFV[PPPPV[\x90a\x19g\x91a\x17\xC9V[V[a\x19z\x90a\x19ua\x1B\x94V[a\x19|V[V[\x80a\x19\x97a\x19\x91a\x19\x8C_a\x11\xC8V[a\x04\xB8V[\x91a\x04\xB8V[\x14a\x19\xF1Wa\x19\xAFa\x19\xA8\x82a\x12.V[`\x01a\x12iV[a\x19\xD9\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10\xD5V[\x90a\x19\xE2a\x01\xB2V[\x80a\x19\xEC\x81a\x04tV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x1A\t`\x04\x82\x01a\x04tV[\x03\x90\xFD[a\x1A\x16\x90a\x19iV[V[a\x1A)\x90a\x1A$a\x1B\x94V[a\x1A+V[V[\x80a\x1AFa\x1A@a\x1A;_a\x11\xC8V[a\x04\xB8V[\x91a\x04\xB8V[\x14a\x1AVWa\x1AT\x90a\x1C\x05V[V[a\x1Aya\x1Ab_a\x11\xC8V[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\t6V[\x03\x90\xFD[a\x1A\x86\x90a\x1A\x18V[V[\x90a\x1A\x94_\x19\x91a\x12:V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x1A\xB6a\x1A\xB1a\x1A\xBD\x92a\x0B=V[a\x1A\x9EV[\x82Ta\x1A\x88V[\x90UV[\x91` a\x1A\xE2\x92\x94\x93a\x1A\xDB`@\x82\x01\x96_\x83\x01\x90a\x06\xC4V[\x01\x90a\x06\xC4V[V[a\x1A\xF7a\x1A\xF1`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x1B\x87W[a\x1B\x04a\x1E1V[a\x1B8\x81a\x1B2`\x02a\x1B\"`\x05a\x1B\x1C`\x03a\x0B\x88V[\x90a\x0BYV[\x01\x91a\x1B-\x83a\x0B\x88V[a\x10\x7FV[\x90a\x1A\xA1V[a\x1BB`\x03a\x0B\x88V[:a\x1Bm\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0B=V[\x92a\x1B\x82a\x1Bya\x01\xB2V[\x92\x83\x92\x83a\x1A\xC1V[\x03\x90\xA2V[a\x1B\x8Fa\x1D.V[a\x1A\xFCV[a\x1B\x9Ca\x16\xFAV[a\x1B\xB5a\x1B\xAFa\x1B\xAAa \x0FV[a\x04\xB8V[\x91a\x04\xB8V[\x03a\x1B\xBCWV[a\x1B\xDEa\x1B\xC7a \x0FV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\t6V[\x03\x90\xFD[\x90V[\x90a\x1B\xFAa\x1B\xF5a\x1C\x01\x92a\x10\xD5V[a\x1B\xE2V[\x82Ta\x12?V[\x90UV[a\x1C\x0E_a\x16\xEDV[a\x1C\x18\x82_a\x1B\xE5V[\x90a\x1CLa\x1CF\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10\xD5V[\x91a\x10\xD5V[\x91a\x1CUa\x01\xB2V[\x80a\x1C_\x81a\x04tV[\x03\x90\xA3V[\x90a\x1Cp`\xFF\x91a\x12:V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1C\x8Fa\x1C\x8Aa\x1C\x96\x92a\x11\xF3V[a\x11\xFFV[\x82Ta\x1CdV[\x90UV[\x90a\x1C\xA4\x90a\r\xD1V[_R` R`@_ \x90V[a\x1C\xBA\x90Qa\x01\xDFV[\x90V[\x90a\x1D\x1A```\x03a\x1D \x94a\x1C\xE0_\x82\x01a\x1C\xDA_\x88\x01a\x15\x8EV[\x90a\x1A\xA1V[a\x1C\xF9`\x01\x82\x01a\x1C\xF3` \x88\x01a\x15\x8EV[\x90a\x1A\xA1V[a\x1D\x12`\x02\x82\x01a\x1D\x0C`@\x88\x01a\x15\x8EV[\x90a\x1A\xA1V[\x01\x92\x01a\x1C\xB0V[\x90a\x1CzV[V[\x90a\x1D,\x91a\x1C\xBDV[V[a\x1DAa\x1D;`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x1DHW[V[a\x1DT`\x01`\x04a\x1CzV[a\x1Dga\x1D`_a\r\xD1V[`\x03a\x1A\xA1V[a\x1D\xC8Ba\x1D\xB7_a\x1D\xAEa\x1D\xA5_a\x1D\xA0a\x1D\x97_\x95a\x1D\x92a\x1D\x89a\r\xC1V[\x99_\x8B\x01a\r\xEDV[a\r\xD1V[` \x88\x01a\r\xEDV[a\r\xD1V[`@\x85\x01a\r\xEDV[``\x83\x01a\r\xFBV[a\x1D\xC3`\x05_\x90a\x1C\x9AV[a\x1D\"V[_B\x90a\x1E\na\x1D\xF8\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\r\xD1V[\x92a\x1E\x01a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xA2a\x1DFV[\x90V[a\x1E\x1E\x90a\x01\xCFV[_\x19\x81\x14a\x1E,W`\x01\x01\x90V[a\x10FV[a\x1ENa\x1EI`\x05a\x1EC`\x03a\x0B\x88V[\x90a\x0BYV[a\x1E\x12V[Ba\x1E|a\x1Eva\x1Eqa\x1Ec_\x86\x01a\x0B\x88V[a\x1Eka\x07:V[\x90a\x10\x7FV[a\x01\xCFV[\x91a\x01\xCFV[\x10\x15a\x1E\x86W[PV[a\x1E\xAEa\x1E\xA5a\x1E\x97_\x84\x01a\x0B\x88V[a\x1E\x9Fa\x07:V[\x90a\x10\x7FV[`\x01\x83\x01a\x1A\xA1V[a\x1E\xBC`\x01`\x03\x83\x01a\x1CzV[a\x1E\xC6`\x03a\x0B\x88V[a\x1E\xF3a\x1E\xD5`\x02\x84\x01a\x0B\x88V[\x92a\x1E\xED_a\x1E\xE6`\x01\x84\x01a\x0B\x88V[\x92\x01a\x0B\x88V[\x90a\x10ZV[a\x1F\x1D\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0B=V[\x92a\x1F2a\x1F)a\x01\xB2V[\x92\x83\x92\x83a\x1A\xC1V[\x03\x90\xA2a\x1FQa\x1FJa\x1FE`\x03a\x0B\x88V[a\x1E\x15V[`\x03a\x1A\xA1V[a\x1F\xBBBa\x1F\xA1_a\x1F\x98a\x1F\x8F_a\x1F\x8Aa\x1F\x81_\x95a\x1F|a\x1Fsa\r\xC1V[\x99_\x8B\x01a\r\xEDV[a\r\xD1V[` \x88\x01a\r\xEDV[a\r\xD1V[`@\x85\x01a\r\xEDV[``\x83\x01a\r\xFBV[a\x1F\xB6`\x05a\x1F\xB0`\x03a\x0B\x88V[\x90a\x0BYV[a\x1D\"V[a\x1F\xC5`\x03a\x0B\x88V[B\x90a \x06a\x1F\xF4\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0B=V[\x92a\x1F\xFDa\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xA2_a\x1E\x83V[a \x17a\x16\xCAV[P3\x90V`\x80`@R4`\x1CW`\x0E` V[a\x01\x94a\0+\x829a\x01\x94\x90\xF3[`&V[`@Q\x90V[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x01{V[a\0\x1D_5a\0,V[cz9y\xDC\x03a\0\x0EWa\x01BV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\0X\x90a\0DV[\x90V[a\0d\x81a\0OV[\x03a\0kWV[_\x80\xFD[\x90P5\x90a\0|\x82a\0[V[V[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\0\xC4W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xBFW` \x01\x92`\x01\x83\x02\x84\x01\x11a\0\xBAWV[a\0\x86V[a\0\x82V[a\0~V[\x91``\x83\x83\x03\x12a\x01\x16Wa\0\xE0\x82_\x85\x01a\0oV[\x92a\0\xEE\x83` \x83\x01a\0oV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x11Wa\x01\r\x92\x01a\0\x8AV[\x90\x91V[a\0@V[a\0<V[\x15\x15\x90V[a\x01)\x90a\x01\x1BV[\x90RV[\x91\x90a\x01@\x90_` \x85\x01\x94\x01\x90a\x01 V[V[4a\x01vWa\x01ra\x01aa\x01X6`\x04a\0\xC9V[\x92\x91\x90\x91a\x01\x83V[a\x01ia\x002V[\x91\x82\x91\x82a\x01-V[\x03\x90\xF3[a\08V[_\x80\xFD[_\x90V[PPPPa\x01\x8Fa\x01\x7FV[P_\x90V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b610d0d565b61001d5f356101ac565b8063086146d2146101a757806318d5aafe146101a2578063366cbab71461019d5780633b6ab2a91461019857806346e2cc0914610193578063485cc9551461018e5780634b2c0706146101895780635b3cd6e214610184578063615438011461017f5780636558954f1461017a578063703cfcbb14610175578063715018a6146101705780637a3979dc1461016b578063804e51231461016657806382f44ade146101615780638d5a239b1461015c5780638da5cb5b14610157578063aff74c6d14610152578063c660d3f31461014d578063cdafb97814610148578063d4f0eb4d14610143578063d87813421461013e578063ea4a110414610139578063ede07bd6146101345763f2fde38b0361000e57610cda565b610ca5565b610c34565b610b08565b610ab1565b610a5f565b6109b5565b610980565b61094b565b6108f4565b6108bf565b61088b565b610852565b6107cd565b610798565b610754565b6106e6565b610657565b610589565b610514565b610479565b61043f565b6103ca565b6102a5565b61024e565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f9103126101ca57565b6101bc565b90565b6101db906101cf565b9052565b151590565b6101ed906101df565b9052565b90606080610237936102095f8201515f8601906101d2565b61021b602082015160208601906101d2565b61022d604082015160408601906101d2565b01519101906101e4565b565b919061024c905f608085019401906101f1565b565b3461027e5761025e3660046101c0565b61027a610269610e7e565b6102716101b2565b91829182610239565b0390f35b6101b8565b61028c906101df565b9052565b91906102a3905f60208501940190610283565b565b346102d5576102b53660046101c0565b6102d16102c0610f1a565b6102c86101b2565b91829182610290565b0390f35b6101b8565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156103245781359167ffffffffffffffff831161031f57602001926001830284011161031a57565b6102e6565b6102e2565b6102de565b9060208282031261035a575f82013567ffffffffffffffff81116103555761035192016102ea565b9091565b6102da565b6101bc565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6103a06103a96020936103ae936103978161035f565b93848093610363565b9586910161036c565b610377565b0190565b6103c79160208201915f818403910152610381565b90565b346103fb576103f76103e66103e0366004610329565b90610fbe565b6103ee6101b2565b918291826103b2565b0390f35b6101b8565b1c90565b60ff1690565b61041a90600861041f9302610400565b610404565b90565b9061042d915461040a565b90565b61043c60045f90610422565b90565b3461046f5761044f3660046101c0565b61046b61045a610430565b6104626101b2565b91829182610290565b0390f35b6101b8565b5f0190565b346104a85761049261048c366004610329565b90611165565b61049a6101b2565b806104a481610474565b0390f35b6101b8565b60018060a01b031690565b6104c1906104ad565b90565b6104cd816104b8565b036104d457565b5f80fd5b905035906104e5826104c4565b565b919060408382031261050f578061050361050c925f86016104d8565b936020016104d8565b90565b6101bc565b346105435761052d6105273660046104e7565b90611316565b6105356101b2565b8061053f81610474565b0390f35b6101b8565b610551816101cf565b0361055857565b5f80fd5b9050359061056982610548565b565b9060208282031261058457610581915f0161055c565b90565b6101bc565b346105b9576105b56105a461059f36600461056b565b611322565b6105ac6101b2565b91829182610239565b0390f35b6101b8565b60018060a01b031690565b6105d99060086105de9302610400565b6105be565b90565b906105ec91546105c9565b90565b6105fb60015f906105e1565b90565b90565b61061561061061061a926104ad565b6105fe565b6104ad565b90565b61062690610601565b90565b6106329061061d565b90565b61063e90610629565b9052565b9190610655905f60208501940190610635565b565b34610687576106673660046101c0565b6106836106726105ef565b61067a6101b2565b91829182610642565b0390f35b6101b8565b90565b61069f9060086106a49302610400565b61068c565b90565b906106b2915461068f565b90565b6106c160035f906106a7565b90565b6106cd906101cf565b9052565b91906106e4905f602085019401906106c4565b565b34610716576106f63660046101c0565b6107126107016106b5565b6107096101b2565b918291826106d1565b0390f35b6101b8565b90565b61073261072d6107379261071b565b6105fe565b6101cf565b90565b61074662278d0061071e565b90565b61075161073a565b90565b34610784576107643660046101c0565b61078061076f610749565b6107776101b2565b918291826106d1565b0390f35b6101b8565b61079560025f906106a7565b90565b346107c8576107a83660046101c0565b6107c46107b3610789565b6107bb6101b2565b918291826106d1565b0390f35b6101b8565b346107fb576107dd3660046101c0565b6107e5611366565b6107ed6101b2565b806107f781610474565b0390f35b6101b8565b9160608383031261084d57610817825f85016104d8565b9261082583602083016104d8565b92604082013567ffffffffffffffff81116108485761084492016102ea565b9091565b6102da565b6101bc565b3461088657610882610871610868366004610800565b9291909161141e565b6108796101b2565b91829182610290565b0390f35b6101b8565b346108ba576108a461089e366004610329565b9061157e565b6108ac6101b2565b806108b681610474565b0390f35b6101b8565b346108ef576108cf3660046101c0565b6108eb6108da61159b565b6108e26101b2565b918291826106d1565b0390f35b6101b8565b34610924576109043660046101c0565b61092061090f611668565b6109176101b2565b918291826106d1565b0390f35b6101b8565b610932906104b8565b9052565b9190610949905f60208501940190610929565b565b3461097b5761095b3660046101c0565b6109776109666116fa565b61096e6101b2565b91829182610936565b0390f35b6101b8565b346109b0576109903660046101c0565b6109ac61099b61172e565b6109a36101b2565b918291826106d1565b0390f35b6101b8565b346109e5576109c53660046101c0565b6109e16109d061177a565b6109d86101b2565b918291826106d1565b0390f35b6101b8565b909182601f83011215610a245781359167ffffffffffffffff8311610a1f576020019260208302840111610a1a57565b6102e6565b6102e2565b6102de565b90602082820312610a5a575f82013567ffffffffffffffff8111610a5557610a5192016109ea565b9091565b6102da565b6101bc565b34610a8e57610a78610a72366004610a29565b9061195d565b610a806101b2565b80610a8a81610474565b0390f35b6101b8565b90602082820312610aac57610aa9915f016104d8565b90565b6101bc565b34610adf57610ac9610ac4366004610a93565b611a0d565b610ad16101b2565b80610adb81610474565b0390f35b6101b8565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610b3857610b183660046101c0565b610b34610b23610ae4565b610b2b6101b2565b918291826106d1565b0390f35b6101b8565b610b51610b4c610b56926101cf565b6105fe565b6101cf565b90565b90610b6390610b3d565b5f5260205260405f2090565b5f1c90565b610b80610b8591610b6f565b61068c565b90565b610b929054610b74565b90565b610ba1610ba691610b6f565b610404565b90565b610bb39054610b95565b90565b610bc1906005610b59565b90610bcd5f8301610b88565b91610bda60018201610b88565b91610bf36003610bec60028501610b88565b9301610ba9565b90565b610c2b610c3294610c21606094989795610c17608086019a5f8701906106c4565b60208501906106c4565b60408301906106c4565b0190610283565b565b34610c6857610c64610c4f610c4a36600461056b565b610bb6565b90610c5b9492946101b2565b94859485610bf6565b0390f35b6101b8565b90565b610c84610c7f610c8992610c6d565b6105fe565b6101cf565b90565b610c97611388610c70565b90565b610ca2610c8c565b90565b34610cd557610cb53660046101c0565b610cd1610cc0610c9a565b610cc86101b2565b918291826106d1565b0390f35b6101b8565b34610d0857610cf2610ced366004610a93565b611a7d565b610cfa6101b2565b80610d0481610474565b0390f35b6101b8565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b90610d2f90610377565b810190811067ffffffffffffffff821117610d4957604052565b610d11565b90610d61610d5a6101b2565b9283610d25565b565b610d6d6080610d4e565b90565b5f90565b5f90565b610d80610d63565b90602080808085610d8f610d70565b815201610d9a610d70565b815201610da5610d70565b815201610db0610d74565b81525050565b610dbe610d78565b90565b610dcb6080610d4e565b90565b90565b610de5610de0610dea92610dce565b6105fe565b6101cf565b90565b90610df7906101cf565b9052565b90610e05906101df565b9052565b90610e70610e676003610e1a610d63565b94610e31610e295f8301610b88565b5f8801610ded565b610e49610e4060018301610b88565b60208801610ded565b610e61610e5860028301610b88565b60408801610ded565b01610ba9565b60608401610dfb565b565b610e7b90610e09565b90565b610e86610db6565b50610e9a610e946004610ba9565b156101df565b610ebe57610ebb610eb66005610eb06003610b88565b90610b59565b610e72565b90565b5f610f135f610f0a610f015f610efc610ef35f95610eee610ee6610ee0610dc1565b9a610dd1565b5f8b01610ded565b610dd1565b60208801610ded565b610dd1565b60408501610ded565b60608301610dfb565b90565b5f90565b610f22610f16565b50610f2d6004610ba9565b90565b606090565b60ff60f81b1690565b60f81b90565b610f58610f53610f5d92610dce565b610f3e565b610f35565b90565b90565b610f6f610f7491610f35565b610f60565b9052565b905090565b90825f939282370152565b909182610f9881610f9f93610f78565b8093610f7d565b0190565b80610fb4600192610fbb9694610f63565b0191610f88565b90565b610ffc90610fca610f30565b50610fed610fd75f610f44565b9193610fe16101b2565b94859360208501610fa3565b60208201810382520382610d25565b90565b9061101b6110153332908585919290919261141e565b156101df565b61102a57611028916110a4565b565b5f631b8e828b60e31b81528061104260048201610474565b0390fd5b634e487b7160e01b5f52601160045260245ffd5b61106961106f919392936101cf565b926101cf565b820391821161107a57565b611046565b61108e611094919392936101cf565b926101cf565b820180921161109f57565b611046565b6110c06110ce916110b96110d3945a9261111e565b5a9061105a565b6110c8610c8c565b9061107f565b611ae4565b565b6110de9061061d565b90565b91906110fb816110f48161110095610363565b8095610f7d565b610377565b0190565b909161111b9260208301925f8185039101526110e1565b90565b33909161114b7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926110d5565b926111606111576101b2565b92839283611104565b0390a2565b9061116f91610fff565b565b906111839161117e611b94565b611289565b565b60a01c90565b61119761119c91611185565b610404565b90565b6111a9905461118b565b90565b6111c06111bb6111c592610dce565b6105fe565b6104ad565b90565b6111d1906111ac565b90565b60a01b90565b906111e960ff60a01b916111d4565b9181191691161790565b6111fc906101df565b90565b90565b9061121761121261121e926111f3565b6111ff565b82546111da565b9055565b61122b90610601565b90565b61123790611222565b90565b5f1b90565b9061125060018060a01b039161123a565b9181191691161790565b61126390611222565b90565b90565b9061127e6112796112859261125a565b611266565b825461123f565b9055565b611293600161119f565b6112fb57816112b26112ac6112a75f6111c8565b6104b8565b916104b8565b146112df576112d86112d16112dd936112cc600180611202565b61122e565b6001611269565b611a7d565b565b5f632e7f3c7f60e11b8152806112f760048201610474565b0390fd5b5f62dc149f60e41b81528061131260048201610474565b0390fd5b9061132091611171565b565b61133961133e91611331610db6565b506005610b59565b610e72565b90565b611349611b94565b611351611353565b565b61136461135f5f6111c8565b611c05565b565b61136e611341565b565b61137c61138191610b6f565b6105be565b90565b61138e9054611370565b90565b60e01b90565b6113a0816101df565b036113a757565b5f80fd5b905051906113b882611397565b565b906020828203126113d3576113d0915f016113ab565b90565b6101bc565b6113fe61140b95939492946113f460608401965f850190610929565b6020830190610929565b60408185039101526110e1565b90565b6114166101b2565b3d5f823e3d90fd5b926114616020939461142e610f16565b5061146c61144461143f6001611384565b610629565b93637a3979dc9295976114556101b2565b98899788968796611391565b8652600486016113d8565b03915afa9081156114b0575f91611482575b5090565b6114a3915060203d81116114a9575b61149b8183610d25565b8101906113ba565b5f61147e565b503d611491565b61140e565b906114d16114cb3332908585919290919261141e565b156101df565b6114e0576114de916114fc565b565b5f631b8e828b60e31b8152806114f860048201610474565b0390fd5b6115186115269161151161152b945a9261152d565b5a9061105a565b611520610c8c565b9061107f565b611ae4565b565b90611539903392610fbe565b906115796115677f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926110d5565b926115706101b2565b918291826103b2565b0390a2565b90611588916114b5565b565b5f90565b61159890516101cf565b90565b6115a361158a565b506115b76115b16004610ba9565b156101df565b611627576115f36115e55f6115df6115da60056115d46003610b88565b90610b59565b610e72565b0161158e565b6115ed61073a565b9061107f565b42611606611600836101cf565b916101cf565b101561161a5761161790429061105a565b90565b506116245f610dd1565b90565b6116305f610dd1565b90565b611642611648919392936101cf565b926101cf565b916116548382026101cf565b92818404149015171561166357565b611046565b61167061158a565b5061168461167e6004610ba9565b156101df565b6116be576116bb6116ab60026116a5600561169f6003610b88565b90610b59565b01610b88565b6116b56002610b88565b90611633565b90565b6116c75f610dd1565b90565b5f90565b60018060a01b031690565b6116e56116ea91610b6f565b6116ce565b90565b6116f790546116d9565b90565b6117026116ca565b5061170c5f6116ed565b90565b90565b61172661172161172b9261170f565b6105fe565b6101cf565b90565b61173661158a565b5061174a6117446004610ba9565b156101df565b61176e5761176b61175b6003610b88565b6117656001611712565b9061107f565b90565b6117775f610dd1565b90565b61178261158a565b506117966117906004610ba9565b156101df565b6117bd576117ba60026117b460056117ae6003610b88565b90610b59565b01610b88565b90565b6117c65f610dd1565b90565b6117e56117f3916117de6117f8945a92611894565b5a9061105a565b6117ed610c8c565b9061107f565b611ae4565b565b5090565b600161180a91016101cf565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561186f570180359067ffffffffffffffff821161186a5760200191600182023603831361186557565b611829565b611825565b611821565b9082101561188f57602061188b920281019061182d565b9091565b61180d565b61189f8183906117fa565b916118a861158a565b506118b25f610dd1565b5b806118c66118c0866101cf565b916101cf565b1015611957576118f4906118ea3332906118e287878691611874565b92909161141e565b6118f9575b6117fe565b6118b3565b3361190f61190986868591611874565b90610fbe565b9061194f61193d7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f926110d5565b926119466101b2565b918291826103b2565b0390a26118ef565b50505050565b90611967916117c9565b565b61197a90611975611b94565b61197c565b565b8061199761199161198c5f6111c8565b6104b8565b916104b8565b146119f1576119af6119a88261122e565b6001611269565b6119d97f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b9916110d5565b906119e26101b2565b806119ec81610474565b0390a2565b5f632e7f3c7f60e11b815280611a0960048201610474565b0390fd5b611a1690611969565b565b611a2990611a24611b94565b611a2b565b565b80611a46611a40611a3b5f6111c8565b6104b8565b916104b8565b14611a5657611a5490611c05565b565b611a79611a625f6111c8565b5f918291631e4fbdf760e01b835260048301610936565b0390fd5b611a8690611a18565b565b90611a945f199161123a565b9181191691161790565b90565b90611ab6611ab1611abd92610b3d565b611a9e565b8254611a88565b9055565b916020611ae2929493611adb60408201965f8301906106c4565b01906106c4565b565b611af7611af16004610ba9565b156101df565b611b87575b611b04611e31565b611b3881611b326002611b226005611b1c6003610b88565b90610b59565b0191611b2d83610b88565b61107f565b90611aa1565b611b426003610b88565b3a611b6d7f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610b3d565b92611b82611b796101b2565b92839283611ac1565b0390a2565b611b8f611d2e565b611afc565b611b9c6116fa565b611bb5611baf611baa61200f565b6104b8565b916104b8565b03611bbc57565b611bde611bc761200f565b5f91829163118cdaa760e01b835260048301610936565b0390fd5b90565b90611bfa611bf5611c01926110d5565b611be2565b825461123f565b9055565b611c0e5f6116ed565b611c18825f611be5565b90611c4c611c467f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0936110d5565b916110d5565b91611c556101b2565b80611c5f81610474565b0390a3565b90611c7060ff9161123a565b9181191691161790565b90611c8f611c8a611c96926111f3565b6111ff565b8254611c64565b9055565b90611ca490610dd1565b5f5260205260405f2090565b611cba90516101df565b90565b90611d1a60606003611d2094611ce05f8201611cda5f880161158e565b90611aa1565b611cf960018201611cf36020880161158e565b90611aa1565b611d1260028201611d0c6040880161158e565b90611aa1565b019201611cb0565b90611c7a565b565b90611d2c91611cbd565b565b611d41611d3b6004610ba9565b156101df565b611d48575b565b611d5460016004611c7a565b611d67611d605f610dd1565b6003611aa1565b611dc842611db75f611dae611da55f611da0611d975f95611d92611d89610dc1565b995f8b01610ded565b610dd1565b60208801610ded565b610dd1565b60408501610ded565b60608301610dfb565b611dc360055f90611c9a565b611d22565b5f4290611e0a611df87f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610dd1565b92611e016101b2565b918291826106d1565b0390a2611d46565b90565b611e1e906101cf565b5f198114611e2c5760010190565b611046565b611e4e611e496005611e436003610b88565b90610b59565b611e12565b42611e7c611e76611e71611e635f8601610b88565b611e6b61073a565b9061107f565b6101cf565b916101cf565b1015611e86575b50565b611eae611ea5611e975f8401610b88565b611e9f61073a565b9061107f565b60018301611aa1565b611ebc600160038301611c7a565b611ec66003610b88565b611ef3611ed560028401610b88565b92611eed5f611ee660018401610b88565b9201610b88565b9061105a565b611f1d7f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610b3d565b92611f32611f296101b2565b92839283611ac1565b0390a2611f51611f4a611f456003610b88565b611e15565b6003611aa1565b611fbb42611fa15f611f98611f8f5f611f8a611f815f95611f7c611f73610dc1565b995f8b01610ded565b610dd1565b60208801610ded565b610dd1565b60408501610ded565b60608301610dfb565b611fb66005611fb06003610b88565b90610b59565b611d22565b611fc56003610b88565b4290612006611ff47f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610b3d565b92611ffd6101b2565b918291826106d1565b0390a25f611e83565b6120176116ca565b50339056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\r\rV[a\0\x1D_5a\x01\xACV[\x80c\x08aF\xD2\x14a\x01\xA7W\x80c\x18\xD5\xAA\xFE\x14a\x01\xA2W\x80c6l\xBA\xB7\x14a\x01\x9DW\x80c;j\xB2\xA9\x14a\x01\x98W\x80cF\xE2\xCC\t\x14a\x01\x93W\x80cH\\\xC9U\x14a\x01\x8EW\x80cK,\x07\x06\x14a\x01\x89W\x80c[<\xD6\xE2\x14a\x01\x84W\x80caT8\x01\x14a\x01\x7FW\x80ceX\x95O\x14a\x01zW\x80cp<\xFC\xBB\x14a\x01uW\x80cqP\x18\xA6\x14a\x01pW\x80cz9y\xDC\x14a\x01kW\x80c\x80NQ#\x14a\x01fW\x80c\x82\xF4J\xDE\x14a\x01aW\x80c\x8DZ#\x9B\x14a\x01\\W\x80c\x8D\xA5\xCB[\x14a\x01WW\x80c\xAF\xF7Lm\x14a\x01RW\x80c\xC6`\xD3\xF3\x14a\x01MW\x80c\xCD\xAF\xB9x\x14a\x01HW\x80c\xD4\xF0\xEBM\x14a\x01CW\x80c\xD8x\x13B\x14a\x01>W\x80c\xEAJ\x11\x04\x14a\x019W\x80c\xED\xE0{\xD6\x14a\x014Wc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\x0C\xDAV[a\x0C\xA5V[a\x0C4V[a\x0B\x08V[a\n\xB1V[a\n_V[a\t\xB5V[a\t\x80V[a\tKV[a\x08\xF4V[a\x08\xBFV[a\x08\x8BV[a\x08RV[a\x07\xCDV[a\x07\x98V[a\x07TV[a\x06\xE6V[a\x06WV[a\x05\x89V[a\x05\x14V[a\x04yV[a\x04?V[a\x03\xCAV[a\x02\xA5V[a\x02NV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01\xCAWV[a\x01\xBCV[\x90V[a\x01\xDB\x90a\x01\xCFV[\x90RV[\x15\x15\x90V[a\x01\xED\x90a\x01\xDFV[\x90RV[\x90``\x80a\x027\x93a\x02\t_\x82\x01Q_\x86\x01\x90a\x01\xD2V[a\x02\x1B` \x82\x01Q` \x86\x01\x90a\x01\xD2V[a\x02-`@\x82\x01Q`@\x86\x01\x90a\x01\xD2V[\x01Q\x91\x01\x90a\x01\xE4V[V[\x91\x90a\x02L\x90_`\x80\x85\x01\x94\x01\x90a\x01\xF1V[V[4a\x02~Wa\x02^6`\x04a\x01\xC0V[a\x02za\x02ia\x0E~V[a\x02qa\x01\xB2V[\x91\x82\x91\x82a\x029V[\x03\x90\xF3[a\x01\xB8V[a\x02\x8C\x90a\x01\xDFV[\x90RV[\x91\x90a\x02\xA3\x90_` \x85\x01\x94\x01\x90a\x02\x83V[V[4a\x02\xD5Wa\x02\xB56`\x04a\x01\xC0V[a\x02\xD1a\x02\xC0a\x0F\x1AV[a\x02\xC8a\x01\xB2V[\x91\x82\x91\x82a\x02\x90V[\x03\x90\xF3[a\x01\xB8V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03$W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\x1FW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x03\x1AWV[a\x02\xE6V[a\x02\xE2V[a\x02\xDEV[\x90` \x82\x82\x03\x12a\x03ZW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03UWa\x03Q\x92\x01a\x02\xEAV[\x90\x91V[a\x02\xDAV[a\x01\xBCV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\xA0a\x03\xA9` \x93a\x03\xAE\x93a\x03\x97\x81a\x03_V[\x93\x84\x80\x93a\x03cV[\x95\x86\x91\x01a\x03lV[a\x03wV[\x01\x90V[a\x03\xC7\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\x81V[\x90V[4a\x03\xFBWa\x03\xF7a\x03\xE6a\x03\xE06`\x04a\x03)V[\x90a\x0F\xBEV[a\x03\xEEa\x01\xB2V[\x91\x82\x91\x82a\x03\xB2V[\x03\x90\xF3[a\x01\xB8V[\x1C\x90V[`\xFF\x16\x90V[a\x04\x1A\x90`\x08a\x04\x1F\x93\x02a\x04\0V[a\x04\x04V[\x90V[\x90a\x04-\x91Ta\x04\nV[\x90V[a\x04<`\x04_\x90a\x04\"V[\x90V[4a\x04oWa\x04O6`\x04a\x01\xC0V[a\x04ka\x04Za\x040V[a\x04ba\x01\xB2V[\x91\x82\x91\x82a\x02\x90V[\x03\x90\xF3[a\x01\xB8V[_\x01\x90V[4a\x04\xA8Wa\x04\x92a\x04\x8C6`\x04a\x03)V[\x90a\x11eV[a\x04\x9Aa\x01\xB2V[\x80a\x04\xA4\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\xC1\x90a\x04\xADV[\x90V[a\x04\xCD\x81a\x04\xB8V[\x03a\x04\xD4WV[_\x80\xFD[\x90P5\x90a\x04\xE5\x82a\x04\xC4V[V[\x91\x90`@\x83\x82\x03\x12a\x05\x0FW\x80a\x05\x03a\x05\x0C\x92_\x86\x01a\x04\xD8V[\x93` \x01a\x04\xD8V[\x90V[a\x01\xBCV[4a\x05CWa\x05-a\x05'6`\x04a\x04\xE7V[\x90a\x13\x16V[a\x055a\x01\xB2V[\x80a\x05?\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[a\x05Q\x81a\x01\xCFV[\x03a\x05XWV[_\x80\xFD[\x90P5\x90a\x05i\x82a\x05HV[V[\x90` \x82\x82\x03\x12a\x05\x84Wa\x05\x81\x91_\x01a\x05\\V[\x90V[a\x01\xBCV[4a\x05\xB9Wa\x05\xB5a\x05\xA4a\x05\x9F6`\x04a\x05kV[a\x13\"V[a\x05\xACa\x01\xB2V[\x91\x82\x91\x82a\x029V[\x03\x90\xF3[a\x01\xB8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x05\xD9\x90`\x08a\x05\xDE\x93\x02a\x04\0V[a\x05\xBEV[\x90V[\x90a\x05\xEC\x91Ta\x05\xC9V[\x90V[a\x05\xFB`\x01_\x90a\x05\xE1V[\x90V[\x90V[a\x06\x15a\x06\x10a\x06\x1A\x92a\x04\xADV[a\x05\xFEV[a\x04\xADV[\x90V[a\x06&\x90a\x06\x01V[\x90V[a\x062\x90a\x06\x1DV[\x90V[a\x06>\x90a\x06)V[\x90RV[\x91\x90a\x06U\x90_` \x85\x01\x94\x01\x90a\x065V[V[4a\x06\x87Wa\x06g6`\x04a\x01\xC0V[a\x06\x83a\x06ra\x05\xEFV[a\x06za\x01\xB2V[\x91\x82\x91\x82a\x06BV[\x03\x90\xF3[a\x01\xB8V[\x90V[a\x06\x9F\x90`\x08a\x06\xA4\x93\x02a\x04\0V[a\x06\x8CV[\x90V[\x90a\x06\xB2\x91Ta\x06\x8FV[\x90V[a\x06\xC1`\x03_\x90a\x06\xA7V[\x90V[a\x06\xCD\x90a\x01\xCFV[\x90RV[\x91\x90a\x06\xE4\x90_` \x85\x01\x94\x01\x90a\x06\xC4V[V[4a\x07\x16Wa\x06\xF66`\x04a\x01\xC0V[a\x07\x12a\x07\x01a\x06\xB5V[a\x07\ta\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[\x90V[a\x072a\x07-a\x077\x92a\x07\x1BV[a\x05\xFEV[a\x01\xCFV[\x90V[a\x07Fb'\x8D\0a\x07\x1EV[\x90V[a\x07Qa\x07:V[\x90V[4a\x07\x84Wa\x07d6`\x04a\x01\xC0V[a\x07\x80a\x07oa\x07IV[a\x07wa\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[a\x07\x95`\x02_\x90a\x06\xA7V[\x90V[4a\x07\xC8Wa\x07\xA86`\x04a\x01\xC0V[a\x07\xC4a\x07\xB3a\x07\x89V[a\x07\xBBa\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[4a\x07\xFBWa\x07\xDD6`\x04a\x01\xC0V[a\x07\xE5a\x13fV[a\x07\xEDa\x01\xB2V[\x80a\x07\xF7\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[\x91``\x83\x83\x03\x12a\x08MWa\x08\x17\x82_\x85\x01a\x04\xD8V[\x92a\x08%\x83` \x83\x01a\x04\xD8V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08HWa\x08D\x92\x01a\x02\xEAV[\x90\x91V[a\x02\xDAV[a\x01\xBCV[4a\x08\x86Wa\x08\x82a\x08qa\x08h6`\x04a\x08\0V[\x92\x91\x90\x91a\x14\x1EV[a\x08ya\x01\xB2V[\x91\x82\x91\x82a\x02\x90V[\x03\x90\xF3[a\x01\xB8V[4a\x08\xBAWa\x08\xA4a\x08\x9E6`\x04a\x03)V[\x90a\x15~V[a\x08\xACa\x01\xB2V[\x80a\x08\xB6\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[4a\x08\xEFWa\x08\xCF6`\x04a\x01\xC0V[a\x08\xEBa\x08\xDAa\x15\x9BV[a\x08\xE2a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[4a\t$Wa\t\x046`\x04a\x01\xC0V[a\t a\t\x0Fa\x16hV[a\t\x17a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[a\t2\x90a\x04\xB8V[\x90RV[\x91\x90a\tI\x90_` \x85\x01\x94\x01\x90a\t)V[V[4a\t{Wa\t[6`\x04a\x01\xC0V[a\twa\tfa\x16\xFAV[a\tna\x01\xB2V[\x91\x82\x91\x82a\t6V[\x03\x90\xF3[a\x01\xB8V[4a\t\xB0Wa\t\x906`\x04a\x01\xC0V[a\t\xACa\t\x9Ba\x17.V[a\t\xA3a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[4a\t\xE5Wa\t\xC56`\x04a\x01\xC0V[a\t\xE1a\t\xD0a\x17zV[a\t\xD8a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\n$W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\x1FW` \x01\x92` \x83\x02\x84\x01\x11a\n\x1AWV[a\x02\xE6V[a\x02\xE2V[a\x02\xDEV[\x90` \x82\x82\x03\x12a\nZW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\nUWa\nQ\x92\x01a\t\xEAV[\x90\x91V[a\x02\xDAV[a\x01\xBCV[4a\n\x8EWa\nxa\nr6`\x04a\n)V[\x90a\x19]V[a\n\x80a\x01\xB2V[\x80a\n\x8A\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[\x90` \x82\x82\x03\x12a\n\xACWa\n\xA9\x91_\x01a\x04\xD8V[\x90V[a\x01\xBCV[4a\n\xDFWa\n\xC9a\n\xC46`\x04a\n\x93V[a\x1A\rV[a\n\xD1a\x01\xB2V[\x80a\n\xDB\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B8Wa\x0B\x186`\x04a\x01\xC0V[a\x0B4a\x0B#a\n\xE4V[a\x0B+a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[a\x0BQa\x0BLa\x0BV\x92a\x01\xCFV[a\x05\xFEV[a\x01\xCFV[\x90V[\x90a\x0Bc\x90a\x0B=V[_R` R`@_ \x90V[_\x1C\x90V[a\x0B\x80a\x0B\x85\x91a\x0BoV[a\x06\x8CV[\x90V[a\x0B\x92\x90Ta\x0BtV[\x90V[a\x0B\xA1a\x0B\xA6\x91a\x0BoV[a\x04\x04V[\x90V[a\x0B\xB3\x90Ta\x0B\x95V[\x90V[a\x0B\xC1\x90`\x05a\x0BYV[\x90a\x0B\xCD_\x83\x01a\x0B\x88V[\x91a\x0B\xDA`\x01\x82\x01a\x0B\x88V[\x91a\x0B\xF3`\x03a\x0B\xEC`\x02\x85\x01a\x0B\x88V[\x93\x01a\x0B\xA9V[\x90V[a\x0C+a\x0C2\x94a\x0C!``\x94\x98\x97\x95a\x0C\x17`\x80\x86\x01\x9A_\x87\x01\x90a\x06\xC4V[` \x85\x01\x90a\x06\xC4V[`@\x83\x01\x90a\x06\xC4V[\x01\x90a\x02\x83V[V[4a\x0ChWa\x0Cda\x0COa\x0CJ6`\x04a\x05kV[a\x0B\xB6V[\x90a\x0C[\x94\x92\x94a\x01\xB2V[\x94\x85\x94\x85a\x0B\xF6V[\x03\x90\xF3[a\x01\xB8V[\x90V[a\x0C\x84a\x0C\x7Fa\x0C\x89\x92a\x0CmV[a\x05\xFEV[a\x01\xCFV[\x90V[a\x0C\x97a\x13\x88a\x0CpV[\x90V[a\x0C\xA2a\x0C\x8CV[\x90V[4a\x0C\xD5Wa\x0C\xB56`\x04a\x01\xC0V[a\x0C\xD1a\x0C\xC0a\x0C\x9AV[a\x0C\xC8a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xF3[a\x01\xB8V[4a\r\x08Wa\x0C\xF2a\x0C\xED6`\x04a\n\x93V[a\x1A}V[a\x0C\xFAa\x01\xB2V[\x80a\r\x04\x81a\x04tV[\x03\x90\xF3[a\x01\xB8V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\r/\x90a\x03wV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rIW`@RV[a\r\x11V[\x90a\raa\rZa\x01\xB2V[\x92\x83a\r%V[V[a\rm`\x80a\rNV[\x90V[_\x90V[_\x90V[a\r\x80a\rcV[\x90` \x80\x80\x80\x85a\r\x8Fa\rpV[\x81R\x01a\r\x9Aa\rpV[\x81R\x01a\r\xA5a\rpV[\x81R\x01a\r\xB0a\rtV[\x81RPPV[a\r\xBEa\rxV[\x90V[a\r\xCB`\x80a\rNV[\x90V[\x90V[a\r\xE5a\r\xE0a\r\xEA\x92a\r\xCEV[a\x05\xFEV[a\x01\xCFV[\x90V[\x90a\r\xF7\x90a\x01\xCFV[\x90RV[\x90a\x0E\x05\x90a\x01\xDFV[\x90RV[\x90a\x0Epa\x0Eg`\x03a\x0E\x1Aa\rcV[\x94a\x0E1a\x0E)_\x83\x01a\x0B\x88V[_\x88\x01a\r\xEDV[a\x0EIa\x0E@`\x01\x83\x01a\x0B\x88V[` \x88\x01a\r\xEDV[a\x0Eaa\x0EX`\x02\x83\x01a\x0B\x88V[`@\x88\x01a\r\xEDV[\x01a\x0B\xA9V[``\x84\x01a\r\xFBV[V[a\x0E{\x90a\x0E\tV[\x90V[a\x0E\x86a\r\xB6V[Pa\x0E\x9Aa\x0E\x94`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x0E\xBEWa\x0E\xBBa\x0E\xB6`\x05a\x0E\xB0`\x03a\x0B\x88V[\x90a\x0BYV[a\x0ErV[\x90V[_a\x0F\x13_a\x0F\na\x0F\x01_a\x0E\xFCa\x0E\xF3_\x95a\x0E\xEEa\x0E\xE6a\x0E\xE0a\r\xC1V[\x9Aa\r\xD1V[_\x8B\x01a\r\xEDV[a\r\xD1V[` \x88\x01a\r\xEDV[a\r\xD1V[`@\x85\x01a\r\xEDV[``\x83\x01a\r\xFBV[\x90V[_\x90V[a\x0F\"a\x0F\x16V[Pa\x0F-`\x04a\x0B\xA9V[\x90V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x0FXa\x0FSa\x0F]\x92a\r\xCEV[a\x0F>V[a\x0F5V[\x90V[\x90V[a\x0Foa\x0Ft\x91a\x0F5V[a\x0F`V[\x90RV[\x90P\x90V[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x0F\x98\x81a\x0F\x9F\x93a\x0FxV[\x80\x93a\x0F}V[\x01\x90V[\x80a\x0F\xB4`\x01\x92a\x0F\xBB\x96\x94a\x0FcV[\x01\x91a\x0F\x88V[\x90V[a\x0F\xFC\x90a\x0F\xCAa\x0F0V[Pa\x0F\xEDa\x0F\xD7_a\x0FDV[\x91\x93a\x0F\xE1a\x01\xB2V[\x94\x85\x93` \x85\x01a\x0F\xA3V[` \x82\x01\x81\x03\x82R\x03\x82a\r%V[\x90V[\x90a\x10\x1Ba\x10\x1532\x90\x85\x85\x91\x92\x90\x91\x92a\x14\x1EV[\x15a\x01\xDFV[a\x10*Wa\x10(\x91a\x10\xA4V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x10B`\x04\x82\x01a\x04tV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x10ia\x10o\x91\x93\x92\x93a\x01\xCFV[\x92a\x01\xCFV[\x82\x03\x91\x82\x11a\x10zWV[a\x10FV[a\x10\x8Ea\x10\x94\x91\x93\x92\x93a\x01\xCFV[\x92a\x01\xCFV[\x82\x01\x80\x92\x11a\x10\x9FWV[a\x10FV[a\x10\xC0a\x10\xCE\x91a\x10\xB9a\x10\xD3\x94Z\x92a\x11\x1EV[Z\x90a\x10ZV[a\x10\xC8a\x0C\x8CV[\x90a\x10\x7FV[a\x1A\xE4V[V[a\x10\xDE\x90a\x06\x1DV[\x90V[\x91\x90a\x10\xFB\x81a\x10\xF4\x81a\x11\0\x95a\x03cV[\x80\x95a\x0F}V[a\x03wV[\x01\x90V[\x90\x91a\x11\x1B\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x10\xE1V[\x90V[3\x90\x91a\x11K\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\xD5V[\x92a\x11`a\x11Wa\x01\xB2V[\x92\x83\x92\x83a\x11\x04V[\x03\x90\xA2V[\x90a\x11o\x91a\x0F\xFFV[V[\x90a\x11\x83\x91a\x11~a\x1B\x94V[a\x12\x89V[V[`\xA0\x1C\x90V[a\x11\x97a\x11\x9C\x91a\x11\x85V[a\x04\x04V[\x90V[a\x11\xA9\x90Ta\x11\x8BV[\x90V[a\x11\xC0a\x11\xBBa\x11\xC5\x92a\r\xCEV[a\x05\xFEV[a\x04\xADV[\x90V[a\x11\xD1\x90a\x11\xACV[\x90V[`\xA0\x1B\x90V[\x90a\x11\xE9`\xFF`\xA0\x1B\x91a\x11\xD4V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x11\xFC\x90a\x01\xDFV[\x90V[\x90V[\x90a\x12\x17a\x12\x12a\x12\x1E\x92a\x11\xF3V[a\x11\xFFV[\x82Ta\x11\xDAV[\x90UV[a\x12+\x90a\x06\x01V[\x90V[a\x127\x90a\x12\"V[\x90V[_\x1B\x90V[\x90a\x12P`\x01\x80`\xA0\x1B\x03\x91a\x12:V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x12c\x90a\x12\"V[\x90V[\x90V[\x90a\x12~a\x12ya\x12\x85\x92a\x12ZV[a\x12fV[\x82Ta\x12?V[\x90UV[a\x12\x93`\x01a\x11\x9FV[a\x12\xFBW\x81a\x12\xB2a\x12\xACa\x12\xA7_a\x11\xC8V[a\x04\xB8V[\x91a\x04\xB8V[\x14a\x12\xDFWa\x12\xD8a\x12\xD1a\x12\xDD\x93a\x12\xCC`\x01\x80a\x12\x02V[a\x12.V[`\x01a\x12iV[a\x1A}V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x12\xF7`\x04\x82\x01a\x04tV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x13\x12`\x04\x82\x01a\x04tV[\x03\x90\xFD[\x90a\x13 \x91a\x11qV[V[a\x139a\x13>\x91a\x131a\r\xB6V[P`\x05a\x0BYV[a\x0ErV[\x90V[a\x13Ia\x1B\x94V[a\x13Qa\x13SV[V[a\x13da\x13__a\x11\xC8V[a\x1C\x05V[V[a\x13na\x13AV[V[a\x13|a\x13\x81\x91a\x0BoV[a\x05\xBEV[\x90V[a\x13\x8E\x90Ta\x13pV[\x90V[`\xE0\x1B\x90V[a\x13\xA0\x81a\x01\xDFV[\x03a\x13\xA7WV[_\x80\xFD[\x90PQ\x90a\x13\xB8\x82a\x13\x97V[V[\x90` \x82\x82\x03\x12a\x13\xD3Wa\x13\xD0\x91_\x01a\x13\xABV[\x90V[a\x01\xBCV[a\x13\xFEa\x14\x0B\x95\x93\x94\x92\x94a\x13\xF4``\x84\x01\x96_\x85\x01\x90a\t)V[` \x83\x01\x90a\t)V[`@\x81\x85\x03\x91\x01Ra\x10\xE1V[\x90V[a\x14\x16a\x01\xB2V[=_\x82>=\x90\xFD[\x92a\x14a` \x93\x94a\x14.a\x0F\x16V[Pa\x14la\x14Da\x14?`\x01a\x13\x84V[a\x06)V[\x93cz9y\xDC\x92\x95\x97a\x14Ua\x01\xB2V[\x98\x89\x97\x88\x96\x87\x96a\x13\x91V[\x86R`\x04\x86\x01a\x13\xD8V[\x03\x91Z\xFA\x90\x81\x15a\x14\xB0W_\x91a\x14\x82W[P\x90V[a\x14\xA3\x91P` =\x81\x11a\x14\xA9W[a\x14\x9B\x81\x83a\r%V[\x81\x01\x90a\x13\xBAV[_a\x14~V[P=a\x14\x91V[a\x14\x0EV[\x90a\x14\xD1a\x14\xCB32\x90\x85\x85\x91\x92\x90\x91\x92a\x14\x1EV[\x15a\x01\xDFV[a\x14\xE0Wa\x14\xDE\x91a\x14\xFCV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x14\xF8`\x04\x82\x01a\x04tV[\x03\x90\xFD[a\x15\x18a\x15&\x91a\x15\x11a\x15+\x94Z\x92a\x15-V[Z\x90a\x10ZV[a\x15 a\x0C\x8CV[\x90a\x10\x7FV[a\x1A\xE4V[V[\x90a\x159\x903\x92a\x0F\xBEV[\x90a\x15ya\x15g\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\xD5V[\x92a\x15pa\x01\xB2V[\x91\x82\x91\x82a\x03\xB2V[\x03\x90\xA2V[\x90a\x15\x88\x91a\x14\xB5V[V[_\x90V[a\x15\x98\x90Qa\x01\xCFV[\x90V[a\x15\xA3a\x15\x8AV[Pa\x15\xB7a\x15\xB1`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x16'Wa\x15\xF3a\x15\xE5_a\x15\xDFa\x15\xDA`\x05a\x15\xD4`\x03a\x0B\x88V[\x90a\x0BYV[a\x0ErV[\x01a\x15\x8EV[a\x15\xEDa\x07:V[\x90a\x10\x7FV[Ba\x16\x06a\x16\0\x83a\x01\xCFV[\x91a\x01\xCFV[\x10\x15a\x16\x1AWa\x16\x17\x90B\x90a\x10ZV[\x90V[Pa\x16$_a\r\xD1V[\x90V[a\x160_a\r\xD1V[\x90V[a\x16Ba\x16H\x91\x93\x92\x93a\x01\xCFV[\x92a\x01\xCFV[\x91a\x16T\x83\x82\x02a\x01\xCFV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x16cWV[a\x10FV[a\x16pa\x15\x8AV[Pa\x16\x84a\x16~`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x16\xBEWa\x16\xBBa\x16\xAB`\x02a\x16\xA5`\x05a\x16\x9F`\x03a\x0B\x88V[\x90a\x0BYV[\x01a\x0B\x88V[a\x16\xB5`\x02a\x0B\x88V[\x90a\x163V[\x90V[a\x16\xC7_a\r\xD1V[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x16\xE5a\x16\xEA\x91a\x0BoV[a\x16\xCEV[\x90V[a\x16\xF7\x90Ta\x16\xD9V[\x90V[a\x17\x02a\x16\xCAV[Pa\x17\x0C_a\x16\xEDV[\x90V[\x90V[a\x17&a\x17!a\x17+\x92a\x17\x0FV[a\x05\xFEV[a\x01\xCFV[\x90V[a\x176a\x15\x8AV[Pa\x17Ja\x17D`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x17nWa\x17ka\x17[`\x03a\x0B\x88V[a\x17e`\x01a\x17\x12V[\x90a\x10\x7FV[\x90V[a\x17w_a\r\xD1V[\x90V[a\x17\x82a\x15\x8AV[Pa\x17\x96a\x17\x90`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x17\xBDWa\x17\xBA`\x02a\x17\xB4`\x05a\x17\xAE`\x03a\x0B\x88V[\x90a\x0BYV[\x01a\x0B\x88V[\x90V[a\x17\xC6_a\r\xD1V[\x90V[a\x17\xE5a\x17\xF3\x91a\x17\xDEa\x17\xF8\x94Z\x92a\x18\x94V[Z\x90a\x10ZV[a\x17\xEDa\x0C\x8CV[\x90a\x10\x7FV[a\x1A\xE4V[V[P\x90V[`\x01a\x18\n\x91\x01a\x01\xCFV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x18oW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x18jW` \x01\x91`\x01\x82\x026\x03\x83\x13a\x18eWV[a\x18)V[a\x18%V[a\x18!V[\x90\x82\x10\x15a\x18\x8FW` a\x18\x8B\x92\x02\x81\x01\x90a\x18-V[\x90\x91V[a\x18\rV[a\x18\x9F\x81\x83\x90a\x17\xFAV[\x91a\x18\xA8a\x15\x8AV[Pa\x18\xB2_a\r\xD1V[[\x80a\x18\xC6a\x18\xC0\x86a\x01\xCFV[\x91a\x01\xCFV[\x10\x15a\x19WWa\x18\xF4\x90a\x18\xEA32\x90a\x18\xE2\x87\x87\x86\x91a\x18tV[\x92\x90\x91a\x14\x1EV[a\x18\xF9W[a\x17\xFEV[a\x18\xB3V[3a\x19\x0Fa\x19\t\x86\x86\x85\x91a\x18tV[\x90a\x0F\xBEV[\x90a\x19Oa\x19=\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10\xD5V[\x92a\x19Fa\x01\xB2V[\x91\x82\x91\x82a\x03\xB2V[\x03\x90\xA2a\x18\xEFV[PPPPV[\x90a\x19g\x91a\x17\xC9V[V[a\x19z\x90a\x19ua\x1B\x94V[a\x19|V[V[\x80a\x19\x97a\x19\x91a\x19\x8C_a\x11\xC8V[a\x04\xB8V[\x91a\x04\xB8V[\x14a\x19\xF1Wa\x19\xAFa\x19\xA8\x82a\x12.V[`\x01a\x12iV[a\x19\xD9\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10\xD5V[\x90a\x19\xE2a\x01\xB2V[\x80a\x19\xEC\x81a\x04tV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x1A\t`\x04\x82\x01a\x04tV[\x03\x90\xFD[a\x1A\x16\x90a\x19iV[V[a\x1A)\x90a\x1A$a\x1B\x94V[a\x1A+V[V[\x80a\x1AFa\x1A@a\x1A;_a\x11\xC8V[a\x04\xB8V[\x91a\x04\xB8V[\x14a\x1AVWa\x1AT\x90a\x1C\x05V[V[a\x1Aya\x1Ab_a\x11\xC8V[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\t6V[\x03\x90\xFD[a\x1A\x86\x90a\x1A\x18V[V[\x90a\x1A\x94_\x19\x91a\x12:V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x1A\xB6a\x1A\xB1a\x1A\xBD\x92a\x0B=V[a\x1A\x9EV[\x82Ta\x1A\x88V[\x90UV[\x91` a\x1A\xE2\x92\x94\x93a\x1A\xDB`@\x82\x01\x96_\x83\x01\x90a\x06\xC4V[\x01\x90a\x06\xC4V[V[a\x1A\xF7a\x1A\xF1`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x1B\x87W[a\x1B\x04a\x1E1V[a\x1B8\x81a\x1B2`\x02a\x1B\"`\x05a\x1B\x1C`\x03a\x0B\x88V[\x90a\x0BYV[\x01\x91a\x1B-\x83a\x0B\x88V[a\x10\x7FV[\x90a\x1A\xA1V[a\x1BB`\x03a\x0B\x88V[:a\x1Bm\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0B=V[\x92a\x1B\x82a\x1Bya\x01\xB2V[\x92\x83\x92\x83a\x1A\xC1V[\x03\x90\xA2V[a\x1B\x8Fa\x1D.V[a\x1A\xFCV[a\x1B\x9Ca\x16\xFAV[a\x1B\xB5a\x1B\xAFa\x1B\xAAa \x0FV[a\x04\xB8V[\x91a\x04\xB8V[\x03a\x1B\xBCWV[a\x1B\xDEa\x1B\xC7a \x0FV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\t6V[\x03\x90\xFD[\x90V[\x90a\x1B\xFAa\x1B\xF5a\x1C\x01\x92a\x10\xD5V[a\x1B\xE2V[\x82Ta\x12?V[\x90UV[a\x1C\x0E_a\x16\xEDV[a\x1C\x18\x82_a\x1B\xE5V[\x90a\x1CLa\x1CF\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10\xD5V[\x91a\x10\xD5V[\x91a\x1CUa\x01\xB2V[\x80a\x1C_\x81a\x04tV[\x03\x90\xA3V[\x90a\x1Cp`\xFF\x91a\x12:V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1C\x8Fa\x1C\x8Aa\x1C\x96\x92a\x11\xF3V[a\x11\xFFV[\x82Ta\x1CdV[\x90UV[\x90a\x1C\xA4\x90a\r\xD1V[_R` R`@_ \x90V[a\x1C\xBA\x90Qa\x01\xDFV[\x90V[\x90a\x1D\x1A```\x03a\x1D \x94a\x1C\xE0_\x82\x01a\x1C\xDA_\x88\x01a\x15\x8EV[\x90a\x1A\xA1V[a\x1C\xF9`\x01\x82\x01a\x1C\xF3` \x88\x01a\x15\x8EV[\x90a\x1A\xA1V[a\x1D\x12`\x02\x82\x01a\x1D\x0C`@\x88\x01a\x15\x8EV[\x90a\x1A\xA1V[\x01\x92\x01a\x1C\xB0V[\x90a\x1CzV[V[\x90a\x1D,\x91a\x1C\xBDV[V[a\x1DAa\x1D;`\x04a\x0B\xA9V[\x15a\x01\xDFV[a\x1DHW[V[a\x1DT`\x01`\x04a\x1CzV[a\x1Dga\x1D`_a\r\xD1V[`\x03a\x1A\xA1V[a\x1D\xC8Ba\x1D\xB7_a\x1D\xAEa\x1D\xA5_a\x1D\xA0a\x1D\x97_\x95a\x1D\x92a\x1D\x89a\r\xC1V[\x99_\x8B\x01a\r\xEDV[a\r\xD1V[` \x88\x01a\r\xEDV[a\r\xD1V[`@\x85\x01a\r\xEDV[``\x83\x01a\r\xFBV[a\x1D\xC3`\x05_\x90a\x1C\x9AV[a\x1D\"V[_B\x90a\x1E\na\x1D\xF8\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\r\xD1V[\x92a\x1E\x01a\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xA2a\x1DFV[\x90V[a\x1E\x1E\x90a\x01\xCFV[_\x19\x81\x14a\x1E,W`\x01\x01\x90V[a\x10FV[a\x1ENa\x1EI`\x05a\x1EC`\x03a\x0B\x88V[\x90a\x0BYV[a\x1E\x12V[Ba\x1E|a\x1Eva\x1Eqa\x1Ec_\x86\x01a\x0B\x88V[a\x1Eka\x07:V[\x90a\x10\x7FV[a\x01\xCFV[\x91a\x01\xCFV[\x10\x15a\x1E\x86W[PV[a\x1E\xAEa\x1E\xA5a\x1E\x97_\x84\x01a\x0B\x88V[a\x1E\x9Fa\x07:V[\x90a\x10\x7FV[`\x01\x83\x01a\x1A\xA1V[a\x1E\xBC`\x01`\x03\x83\x01a\x1CzV[a\x1E\xC6`\x03a\x0B\x88V[a\x1E\xF3a\x1E\xD5`\x02\x84\x01a\x0B\x88V[\x92a\x1E\xED_a\x1E\xE6`\x01\x84\x01a\x0B\x88V[\x92\x01a\x0B\x88V[\x90a\x10ZV[a\x1F\x1D\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0B=V[\x92a\x1F2a\x1F)a\x01\xB2V[\x92\x83\x92\x83a\x1A\xC1V[\x03\x90\xA2a\x1FQa\x1FJa\x1FE`\x03a\x0B\x88V[a\x1E\x15V[`\x03a\x1A\xA1V[a\x1F\xBBBa\x1F\xA1_a\x1F\x98a\x1F\x8F_a\x1F\x8Aa\x1F\x81_\x95a\x1F|a\x1Fsa\r\xC1V[\x99_\x8B\x01a\r\xEDV[a\r\xD1V[` \x88\x01a\r\xEDV[a\r\xD1V[`@\x85\x01a\r\xEDV[``\x83\x01a\r\xFBV[a\x1F\xB6`\x05a\x1F\xB0`\x03a\x0B\x88V[\x90a\x0BYV[a\x1D\"V[a\x1F\xC5`\x03a\x0B\x88V[B\x90a \x06a\x1F\xF4\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0B=V[\x92a\x1F\xFDa\x01\xB2V[\x91\x82\x91\x82a\x06\xD1V[\x03\x90\xA2_a\x1E\x83V[a \x17a\x16\xCAV[P3\x90V",
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
            [91u8, 60u8, 214u8, 226u8],
            [97u8, 84u8, 56u8, 1u8],
            [101u8, 88u8, 149u8, 79u8],
            [112u8, 60u8, 252u8, 187u8],
            [113u8, 80u8, 24u8, 166u8],
            [122u8, 57u8, 121u8, 220u8],
            [128u8, 78u8, 81u8, 35u8],
            [130u8, 244u8, 74u8, 222u8],
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
    impl alloy_sol_types::SolInterface for SyndicateSequencingChainCalls {
        const NAME: &'static str = "SyndicateSequencingChainCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
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
