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
    ///0x60a060405234610038576100196100146100e9565b6101b7565b61002161003d565b611fc7610534823960805181610ad60152611fc790f35b610043565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b9061006f90610047565b810190811060018060401b0382111761008757604052565b610051565b9061009f61009861003d565b9283610065565b565b5f80fd5b90565b6100b1816100a5565b036100b857565b5f80fd5b905051906100c9826100a8565b565b906020828203126100e4576100e1915f016100bc565b90565b6100a1565b6101076126ba803803806100fc8161008c565b9283398101906100cb565b90565b90565b90565b61012461011f6101299261010a565b61010d565b6100a5565b90565b60209181520190565b5f7f41707020636861696e2049442063616e6e6f7420626520300000000000000000910152565b610169601860209261012c565b61017281610135565b0190565b61018b9060208101905f81830391015261015c565b90565b1561019557565b61019d61003d565b62461bcd60e51b8152806101b360048201610176565b0390fd5b6101bf61023e565b6101dc816101d56101cf5f610110565b916100a5565b141561018e565b608052565b5f1b90565b906101f25f19916101e1565b9181191691161790565b90565b61021361020e610218926101fc565b61010d565b6100a5565b90565b90565b9061023361022e61023a926101ff565b61021b565b82546101e6565b9055565b610246610348565b610255633b9aca00600261021e565b565b60a01b90565b9061026c60ff60a01b91610257565b9181191691161790565b151590565b61028490610276565b90565b90565b9061029f61029a6102a69261027b565b610287565b825461025d565b9055565b5f0190565b6102b761003d565b3d5f823e3d90fd5b60018060a01b031690565b6102de6102d96102e3926102bf565b61010d565b6102bf565b90565b6102ef906102ca565b90565b6102fb906102e6565b90565b9061030f60018060a01b03916101e1565b9181191691161790565b610322906102e6565b90565b90565b9061033d61033861034492610319565b610325565b82546102fe565b9055565b610351336103b5565b61035c5f600161028a565b61036461003d565b6101bf810181811060018060401b038211176103b05761038c82916101bf6124fb84396102aa565b03905ff080156103ab576103a26103a9916102f2565b6001610328565b565b6102af565b610051565b6103be90610416565b565b6103d46103cf6103d99261010a565b61010d565b6102bf565b90565b6103e5906103c0565b90565b6103f1906102bf565b90565b6103fd906103e8565b9052565b9190610414905f602085019401906103f4565b565b8061043161042b6104265f6103dc565b6103e8565b916103e8565b146104415761043f906104d4565b565b61046461044d5f6103dc565b5f918291631e4fbdf760e01b835260048301610401565b0390fd5b5f1c90565b60018060a01b031690565b61048461048991610468565b61046d565b90565b6104969054610478565b90565b6104a2906102ca565b90565b6104ae90610499565b90565b90565b906104c96104c46104d0926104a5565b6104b1565b82546102fe565b9055565b6104dd5f61048c565b6104e7825f6104b4565b9061051b6105157f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0936104a5565b916104a5565b9161052461003d565b8061052e816102aa565b0390a356fe60806040526004361015610013575b610c90565b61001d5f3561019c565b8063086146d21461019757806318d5aafe14610192578063366cbab71461018d5780633b6ab2a91461018857806346e2cc0914610183578063485cc9551461017e5780634b2c0706146101795780635b3cd6e214610174578063615438011461016f5780636558954f1461016a578063703cfcbb14610165578063715018a6146101605780637a3979dc1461015b578063804e51231461015657806382f44ade146101515780638d5a239b1461014c5780638da5cb5b14610147578063aff74c6d14610142578063c660d3f31461013d578063cdafb97814610138578063d4f0eb4d14610133578063d87813421461012e578063ea4a1104146101295763f2fde38b0361000e57610c5d565b610c24565b610af8565b610aa1565b610a4f565b6109a5565b610970565b61093b565b6108e4565b6108af565b61087b565b610842565b6107bd565b610788565b610744565b6106d6565b610647565b610579565b610504565b610469565b61042f565b6103ba565b610295565b61023e565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f9103126101ba57565b6101ac565b90565b6101cb906101bf565b9052565b151590565b6101dd906101cf565b9052565b90606080610227936101f95f8201515f8601906101c2565b61020b602082015160208601906101c2565b61021d604082015160408601906101c2565b01519101906101d4565b565b919061023c905f608085019401906101e1565b565b3461026e5761024e3660046101b0565b61026a610259610e01565b6102616101a2565b91829182610229565b0390f35b6101a8565b61027c906101cf565b9052565b9190610293905f60208501940190610273565b565b346102c5576102a53660046101b0565b6102c16102b0610e9d565b6102b86101a2565b91829182610280565b0390f35b6101a8565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156103145781359167ffffffffffffffff831161030f57602001926001830284011161030a57565b6102d6565b6102d2565b6102ce565b9060208282031261034a575f82013567ffffffffffffffff81116103455761034192016102da565b9091565b6102ca565b6101ac565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61039061039960209361039e936103878161034f565b93848093610353565b9586910161035c565b610367565b0190565b6103b79160208201915f818403910152610371565b90565b346103eb576103e76103d66103d0366004610319565b90610f41565b6103de6101a2565b918291826103a2565b0390f35b6101a8565b1c90565b60ff1690565b61040a90600861040f93026103f0565b6103f4565b90565b9061041d91546103fa565b90565b61042c60045f90610412565b90565b3461045f5761043f3660046101b0565b61045b61044a610420565b6104526101a2565b91829182610280565b0390f35b6101a8565b5f0190565b346104985761048261047c366004610319565b9061110a565b61048a6101a2565b8061049481610464565b0390f35b6101a8565b60018060a01b031690565b6104b19061049d565b90565b6104bd816104a8565b036104c457565b5f80fd5b905035906104d5826104b4565b565b91906040838203126104ff57806104f36104fc925f86016104c8565b936020016104c8565b90565b6101ac565b346105335761051d6105173660046104d7565b906112bb565b6105256101a2565b8061052f81610464565b0390f35b6101a8565b610541816101bf565b0361054857565b5f80fd5b9050359061055982610538565b565b9060208282031261057457610571915f0161054c565b90565b6101ac565b346105a9576105a561059461058f36600461055b565b6112c7565b61059c6101a2565b91829182610229565b0390f35b6101a8565b60018060a01b031690565b6105c99060086105ce93026103f0565b6105ae565b90565b906105dc91546105b9565b90565b6105eb60015f906105d1565b90565b90565b61060561060061060a9261049d565b6105ee565b61049d565b90565b610616906105f1565b90565b6106229061060d565b90565b61062e90610619565b9052565b9190610645905f60208501940190610625565b565b34610677576106573660046101b0565b6106736106626105df565b61066a6101a2565b91829182610632565b0390f35b6101a8565b90565b61068f90600861069493026103f0565b61067c565b90565b906106a2915461067f565b90565b6106b160035f90610697565b90565b6106bd906101bf565b9052565b91906106d4905f602085019401906106b4565b565b34610706576106e63660046101b0565b6107026106f16106a5565b6106f96101a2565b918291826106c1565b0390f35b6101a8565b90565b61072261071d6107279261070b565b6105ee565b6101bf565b90565b61073662278d0061070e565b90565b61074161072a565b90565b34610774576107543660046101b0565b61077061075f610739565b6107676101a2565b918291826106c1565b0390f35b6101a8565b61078560025f90610697565b90565b346107b8576107983660046101b0565b6107b46107a3610779565b6107ab6101a2565b918291826106c1565b0390f35b6101a8565b346107eb576107cd3660046101b0565b6107d561130b565b6107dd6101a2565b806107e781610464565b0390f35b6101a8565b9160608383031261083d57610807825f85016104c8565b9261081583602083016104c8565b92604082013567ffffffffffffffff81116108385761083492016102da565b9091565b6102ca565b6101ac565b34610876576108726108616108583660046107f0565b929190916113c3565b6108696101a2565b91829182610280565b0390f35b6101a8565b346108aa5761089461088e366004610319565b90611526565b61089c6101a2565b806108a681610464565b0390f35b6101a8565b346108df576108bf3660046101b0565b6108db6108ca611543565b6108d26101a2565b918291826106c1565b0390f35b6101a8565b34610914576108f43660046101b0565b6109106108ff611610565b6109076101a2565b918291826106c1565b0390f35b6101a8565b610922906104a8565b9052565b9190610939905f60208501940190610919565b565b3461096b5761094b3660046101b0565b6109676109566116a2565b61095e6101a2565b91829182610926565b0390f35b6101a8565b346109a0576109803660046101b0565b61099c61098b6116d6565b6109936101a2565b918291826106c1565b0390f35b6101a8565b346109d5576109b53660046101b0565b6109d16109c0611722565b6109c86101a2565b918291826106c1565b0390f35b6101a8565b909182601f83011215610a145781359167ffffffffffffffff8311610a0f576020019260208302840111610a0a57565b6102d6565b6102d2565b6102ce565b90602082820312610a4a575f82013567ffffffffffffffff8111610a4557610a4192016109da565b9091565b6102ca565b6101ac565b34610a7e57610a68610a62366004610a19565b90611908565b610a706101a2565b80610a7a81610464565b0390f35b6101a8565b90602082820312610a9c57610a99915f016104c8565b90565b6101ac565b34610acf57610ab9610ab4366004610a83565b6119b8565b610ac16101a2565b80610acb81610464565b0390f35b6101a8565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610b2857610b083660046101b0565b610b24610b13610ad4565b610b1b6101a2565b918291826106c1565b0390f35b6101a8565b610b41610b3c610b46926101bf565b6105ee565b6101bf565b90565b90610b5390610b2d565b5f5260205260405f2090565b5f1c90565b610b70610b7591610b5f565b61067c565b90565b610b829054610b64565b90565b610b91610b9691610b5f565b6103f4565b90565b610ba39054610b85565b90565b610bb1906005610b49565b90610bbd5f8301610b78565b91610bca60018201610b78565b91610be36003610bdc60028501610b78565b9301610b99565b90565b610c1b610c2294610c11606094989795610c07608086019a5f8701906106b4565b60208501906106b4565b60408301906106b4565b0190610273565b565b34610c5857610c54610c3f610c3a36600461055b565b610ba6565b90610c4b9492946101a2565b94859485610be6565b0390f35b6101a8565b34610c8b57610c75610c70366004610a83565b611a28565b610c7d6101a2565b80610c8781610464565b0390f35b6101a8565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b90610cb290610367565b810190811067ffffffffffffffff821117610ccc57604052565b610c94565b90610ce4610cdd6101a2565b9283610ca8565b565b610cf06080610cd1565b90565b5f90565b5f90565b610d03610ce6565b90602080808085610d12610cf3565b815201610d1d610cf3565b815201610d28610cf3565b815201610d33610cf7565b81525050565b610d41610cfb565b90565b610d4e6080610cd1565b90565b90565b610d68610d63610d6d92610d51565b6105ee565b6101bf565b90565b90610d7a906101bf565b9052565b90610d88906101cf565b9052565b90610df3610dea6003610d9d610ce6565b94610db4610dac5f8301610b78565b5f8801610d70565b610dcc610dc360018301610b78565b60208801610d70565b610de4610ddb60028301610b78565b60408801610d70565b01610b99565b60608401610d7e565b565b610dfe90610d8c565b90565b610e09610d39565b50610e1d610e176004610b99565b156101cf565b610e4157610e3e610e396005610e336003610b78565b90610b49565b610df5565b90565b5f610e965f610e8d610e845f610e7f610e765f95610e71610e69610e63610d44565b9a610d54565b5f8b01610d70565b610d54565b60208801610d70565b610d54565b60408501610d70565b60608301610d7e565b90565b5f90565b610ea5610e99565b50610eb06004610b99565b90565b606090565b60ff60f81b1690565b60f81b90565b610edb610ed6610ee092610d51565b610ec1565b610eb8565b90565b90565b610ef2610ef791610eb8565b610ee3565b9052565b905090565b90825f939282370152565b909182610f1b81610f2293610efb565b8093610f00565b0190565b80610f37600192610f3e9694610ee6565b0191610f0b565b90565b610f7f90610f4d610eb3565b50610f70610f5a5f610ec7565b9193610f646101a2565b94859360208501610f26565b60208201810382520382610ca8565b90565b90610f9e610f98333290858591929091926113c3565b156101cf565b610fad57610fab91611046565b565b5f631b8e828b60e31b815280610fc560048201610464565b0390fd5b634e487b7160e01b5f52601160045260245ffd5b610fec610ff2919392936101bf565b926101bf565b8203918211610ffd57565b610fc9565b90565b61101961101461101e92611002565b6105ee565b6101bf565b90565b611030611036919392936101bf565b926101bf565b820180921161104157565b610fc9565b6110626110739161105b611078945a926110c3565b5a90610fdd565b61106d611388611005565b90611021565b611a8f565b565b6110839061060d565b90565b91906110a081611099816110a595610353565b8095610f00565b610367565b0190565b90916110c09260208301925f818503910152611086565b90565b3390916110f07f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f9261107a565b926111056110fc6101a2565b928392836110a9565b0390a2565b9061111491610f82565b565b9061112891611123611b3f565b61122e565b565b60a01c90565b61113c6111419161112a565b6103f4565b90565b61114e9054611130565b90565b61116561116061116a92610d51565b6105ee565b61049d565b90565b61117690611151565b90565b60a01b90565b9061118e60ff60a01b91611179565b9181191691161790565b6111a1906101cf565b90565b90565b906111bc6111b76111c392611198565b6111a4565b825461117f565b9055565b6111d0906105f1565b90565b6111dc906111c7565b90565b5f1b90565b906111f560018060a01b03916111df565b9181191691161790565b611208906111c7565b90565b90565b9061122361121e61122a926111ff565b61120b565b82546111e4565b9055565b6112386001611144565b6112a0578161125761125161124c5f61116d565b6104a8565b916104a8565b146112845761127d611276611282936112716001806111a7565b6111d3565b600161120e565b611a28565b565b5f632e7f3c7f60e11b81528061129c60048201610464565b0390fd5b5f62dc149f60e41b8152806112b760048201610464565b0390fd5b906112c591611116565b565b6112de6112e3916112d6610d39565b506005610b49565b610df5565b90565b6112ee611b3f565b6112f66112f8565b565b6113096113045f61116d565b611bb0565b565b6113136112e6565b565b61132161132691610b5f565b6105ae565b90565b6113339054611315565b90565b60e01b90565b611345816101cf565b0361134c57565b5f80fd5b9050519061135d8261133c565b565b9060208282031261137857611375915f01611350565b90565b6101ac565b6113a36113b0959394929461139960608401965f850190610919565b6020830190610919565b6040818503910152611086565b90565b6113bb6101a2565b3d5f823e3d90fd5b92611406602093946113d3610e99565b506114116113e96113e46001611329565b610619565b93637a3979dc9295976113fa6101a2565b98899788968796611336565b86526004860161137d565b03915afa908115611455575f91611427575b5090565b611448915060203d811161144e575b6114408183610ca8565b81019061135f565b5f611423565b503d611436565b6113b3565b90611476611470333290858591929091926113c3565b156101cf565b61148557611483916114a1565b565b5f631b8e828b60e31b81528061149d60048201610464565b0390fd5b6114bd6114ce916114b66114d3945a926114d5565b5a90610fdd565b6114c8611388611005565b90611021565b611a8f565b565b906114e1903392610f41565b9061152161150f7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f9261107a565b926115186101a2565b918291826103a2565b0390a2565b906115309161145a565b565b5f90565b61154090516101bf565b90565b61154b611532565b5061155f6115596004610b99565b156101cf565b6115cf5761159b61158d5f611587611582600561157c6003610b78565b90610b49565b610df5565b01611536565b61159561072a565b90611021565b426115ae6115a8836101bf565b916101bf565b10156115c2576115bf904290610fdd565b90565b506115cc5f610d54565b90565b6115d85f610d54565b90565b6115ea6115f0919392936101bf565b926101bf565b916115fc8382026101bf565b92818404149015171561160b57565b610fc9565b611618611532565b5061162c6116266004610b99565b156101cf565b61166657611663611653600261164d60056116476003610b78565b90610b49565b01610b78565b61165d6002610b78565b906115db565b90565b61166f5f610d54565b90565b5f90565b60018060a01b031690565b61168d61169291610b5f565b611676565b90565b61169f9054611681565b90565b6116aa611672565b506116b45f611695565b90565b90565b6116ce6116c96116d3926116b7565b6105ee565b6101bf565b90565b6116de611532565b506116f26116ec6004610b99565b156101cf565b611716576117136117036003610b78565b61170d60016116ba565b90611021565b90565b61171f5f610d54565b90565b61172a611532565b5061173e6117386004610b99565b156101cf565b61176557611762600261175c60056117566003610b78565b90610b49565b01610b78565b90565b61176e5f610d54565b90565b61178d61179e916117866117a3945a9261183f565b5a90610fdd565b611798611388611005565b90611021565b611a8f565b565b5090565b60016117b591016101bf565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561181a570180359067ffffffffffffffff82116118155760200191600182023603831361181057565b6117d4565b6117d0565b6117cc565b9082101561183a57602061183692028101906117d8565b9091565b6117b8565b61184a8183906117a5565b91611853611532565b5061185d5f610d54565b5b8061187161186b866101bf565b916101bf565b10156119025761189f9061189533329061188d8787869161181f565b9290916113c3565b6118a4575b6117a9565b61185e565b336118ba6118b48686859161181f565b90610f41565b906118fa6118e87f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f9261107a565b926118f16101a2565b918291826103a2565b0390a261189a565b50505050565b9061191291611771565b565b61192590611920611b3f565b611927565b565b8061194261193c6119375f61116d565b6104a8565b916104a8565b1461199c5761195a611953826111d3565b600161120e565b6119847f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b99161107a565b9061198d6101a2565b8061199781610464565b0390a2565b5f632e7f3c7f60e11b8152806119b460048201610464565b0390fd5b6119c190611914565b565b6119d4906119cf611b3f565b6119d6565b565b806119f16119eb6119e65f61116d565b6104a8565b916104a8565b14611a01576119ff90611bb0565b565b611a24611a0d5f61116d565b5f918291631e4fbdf760e01b835260048301610926565b0390fd5b611a31906119c3565b565b90611a3f5f19916111df565b9181191691161790565b90565b90611a61611a5c611a6892610b2d565b611a49565b8254611a33565b9055565b916020611a8d929493611a8660408201965f8301906106b4565b01906106b4565b565b611aa2611a9c6004610b99565b156101cf565b611b32575b611aaf611ddc565b611ae381611add6002611acd6005611ac76003610b78565b90610b49565b0191611ad883610b78565b611021565b90611a4c565b611aed6003610b78565b3a611b187f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610b2d565b92611b2d611b246101a2565b92839283611a6c565b0390a2565b611b3a611cd9565b611aa7565b611b476116a2565b611b60611b5a611b55611fba565b6104a8565b916104a8565b03611b6757565b611b89611b72611fba565b5f91829163118cdaa760e01b835260048301610926565b0390fd5b90565b90611ba5611ba0611bac9261107a565b611b8d565b82546111e4565b9055565b611bb95f611695565b611bc3825f611b90565b90611bf7611bf17f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09361107a565b9161107a565b91611c006101a2565b80611c0a81610464565b0390a3565b90611c1b60ff916111df565b9181191691161790565b90611c3a611c35611c4192611198565b6111a4565b8254611c0f565b9055565b90611c4f90610d54565b5f5260205260405f2090565b611c6590516101cf565b90565b90611cc560606003611ccb94611c8b5f8201611c855f8801611536565b90611a4c565b611ca460018201611c9e60208801611536565b90611a4c565b611cbd60028201611cb760408801611536565b90611a4c565b019201611c5b565b90611c25565b565b90611cd791611c68565b565b611cec611ce66004610b99565b156101cf565b611cf3575b565b611cff60016004611c25565b611d12611d0b5f610d54565b6003611a4c565b611d7342611d625f611d59611d505f611d4b611d425f95611d3d611d34610d44565b995f8b01610d70565b610d54565b60208801610d70565b610d54565b60408501610d70565b60608301610d7e565b611d6e60055f90611c45565b611ccd565b5f4290611db5611da37f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610d54565b92611dac6101a2565b918291826106c1565b0390a2611cf1565b90565b611dc9906101bf565b5f198114611dd75760010190565b610fc9565b611df9611df46005611dee6003610b78565b90610b49565b611dbd565b42611e27611e21611e1c611e0e5f8601610b78565b611e1661072a565b90611021565b6101bf565b916101bf565b1015611e31575b50565b611e59611e50611e425f8401610b78565b611e4a61072a565b90611021565b60018301611a4c565b611e67600160038301611c25565b611e716003610b78565b611e9e611e8060028401610b78565b92611e985f611e9160018401610b78565b9201610b78565b90610fdd565b611ec87f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610b2d565b92611edd611ed46101a2565b92839283611a6c565b0390a2611efc611ef5611ef06003610b78565b611dc0565b6003611a4c565b611f6642611f4c5f611f43611f3a5f611f35611f2c5f95611f27611f1e610d44565b995f8b01610d70565b610d54565b60208801610d70565b610d54565b60408501610d70565b60608301610d7e565b611f616005611f5b6003610b78565b90610b49565b611ccd565b611f706003610b78565b4290611fb1611f9f7f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610b2d565b92611fa86101a2565b918291826106c1565b0390a25f611e2e565b611fc2611672565b50339056608060405234601c57600e6020565b61019461002b823961019490f35b6026565b60405190565b5f80fdfe60806040526004361015610013575b61017b565b61001d5f3561002c565b637a3979dc0361000e57610142565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f80fd5b60018060a01b031690565b61005890610044565b90565b6100648161004f565b0361006b57565b5f80fd5b9050359061007c8261005b565b565b5f80fd5b5f80fd5b5f80fd5b909182601f830112156100c45781359167ffffffffffffffff83116100bf5760200192600183028401116100ba57565b610086565b610082565b61007e565b91606083830312610116576100e0825f850161006f565b926100ee836020830161006f565b92604082013567ffffffffffffffff81116101115761010d920161008a565b9091565b610040565b61003c565b151590565b6101299061011b565b9052565b9190610140905f60208501940190610120565b565b34610176576101726101616101583660046100c9565b92919091610183565b610169610032565b9182918261012d565b0390f35b610038565b5f80fd5b5f90565b5050505061018f61017f565b505f9056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4a\08Wa\0\x19a\0\x14a\0\xE9V[a\x01\xB7V[a\0!a\0=V[a\x1F\xC7a\x054\x829`\x80Q\x81a\n\xD6\x01Ra\x1F\xC7\x90\xF3[a\0CV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0o\x90a\0GV[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x87W`@RV[a\0QV[\x90a\0\x9Fa\0\x98a\0=V[\x92\x83a\0eV[V[_\x80\xFD[\x90V[a\0\xB1\x81a\0\xA5V[\x03a\0\xB8WV[_\x80\xFD[\x90PQ\x90a\0\xC9\x82a\0\xA8V[V[\x90` \x82\x82\x03\x12a\0\xE4Wa\0\xE1\x91_\x01a\0\xBCV[\x90V[a\0\xA1V[a\x01\x07a&\xBA\x808\x03\x80a\0\xFC\x81a\0\x8CV[\x92\x839\x81\x01\x90a\0\xCBV[\x90V[\x90V[\x90V[a\x01$a\x01\x1Fa\x01)\x92a\x01\nV[a\x01\rV[a\0\xA5V[\x90V[` \x91\x81R\x01\x90V[_\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01i`\x18` \x92a\x01,V[a\x01r\x81a\x015V[\x01\x90V[a\x01\x8B\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x01\\V[\x90V[\x15a\x01\x95WV[a\x01\x9Da\0=V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x01\xB3`\x04\x82\x01a\x01vV[\x03\x90\xFD[a\x01\xBFa\x02>V[a\x01\xDC\x81a\x01\xD5a\x01\xCF_a\x01\x10V[\x91a\0\xA5V[\x14\x15a\x01\x8EV[`\x80RV[_\x1B\x90V[\x90a\x01\xF2_\x19\x91a\x01\xE1V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[a\x02\x13a\x02\x0Ea\x02\x18\x92a\x01\xFCV[a\x01\rV[a\0\xA5V[\x90V[\x90V[\x90a\x023a\x02.a\x02:\x92a\x01\xFFV[a\x02\x1BV[\x82Ta\x01\xE6V[\x90UV[a\x02Fa\x03HV[a\x02Uc;\x9A\xCA\0`\x02a\x02\x1EV[V[`\xA0\x1B\x90V[\x90a\x02l`\xFF`\xA0\x1B\x91a\x02WV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x15\x15\x90V[a\x02\x84\x90a\x02vV[\x90V[\x90V[\x90a\x02\x9Fa\x02\x9Aa\x02\xA6\x92a\x02{V[a\x02\x87V[\x82Ta\x02]V[\x90UV[_\x01\x90V[a\x02\xB7a\0=V[=_\x82>=\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x02\xDEa\x02\xD9a\x02\xE3\x92a\x02\xBFV[a\x01\rV[a\x02\xBFV[\x90V[a\x02\xEF\x90a\x02\xCAV[\x90V[a\x02\xFB\x90a\x02\xE6V[\x90V[\x90a\x03\x0F`\x01\x80`\xA0\x1B\x03\x91a\x01\xE1V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x03\"\x90a\x02\xE6V[\x90V[\x90V[\x90a\x03=a\x038a\x03D\x92a\x03\x19V[a\x03%V[\x82Ta\x02\xFEV[\x90UV[a\x03Q3a\x03\xB5V[a\x03\\_`\x01a\x02\x8AV[a\x03da\0=V[a\x01\xBF\x81\x01\x81\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\x03\xB0Wa\x03\x8C\x82\x91a\x01\xBFa$\xFB\x849a\x02\xAAV[\x03\x90_\xF0\x80\x15a\x03\xABWa\x03\xA2a\x03\xA9\x91a\x02\xF2V[`\x01a\x03(V[V[a\x02\xAFV[a\0QV[a\x03\xBE\x90a\x04\x16V[V[a\x03\xD4a\x03\xCFa\x03\xD9\x92a\x01\nV[a\x01\rV[a\x02\xBFV[\x90V[a\x03\xE5\x90a\x03\xC0V[\x90V[a\x03\xF1\x90a\x02\xBFV[\x90V[a\x03\xFD\x90a\x03\xE8V[\x90RV[\x91\x90a\x04\x14\x90_` \x85\x01\x94\x01\x90a\x03\xF4V[V[\x80a\x041a\x04+a\x04&_a\x03\xDCV[a\x03\xE8V[\x91a\x03\xE8V[\x14a\x04AWa\x04?\x90a\x04\xD4V[V[a\x04da\x04M_a\x03\xDCV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x04\x01V[\x03\x90\xFD[_\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\x84a\x04\x89\x91a\x04hV[a\x04mV[\x90V[a\x04\x96\x90Ta\x04xV[\x90V[a\x04\xA2\x90a\x02\xCAV[\x90V[a\x04\xAE\x90a\x04\x99V[\x90V[\x90V[\x90a\x04\xC9a\x04\xC4a\x04\xD0\x92a\x04\xA5V[a\x04\xB1V[\x82Ta\x02\xFEV[\x90UV[a\x04\xDD_a\x04\x8CV[a\x04\xE7\x82_a\x04\xB4V[\x90a\x05\x1Ba\x05\x15\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x04\xA5V[\x91a\x04\xA5V[\x91a\x05$a\0=V[\x80a\x05.\x81a\x02\xAAV[\x03\x90\xA3V\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x0C\x90V[a\0\x1D_5a\x01\x9CV[\x80c\x08aF\xD2\x14a\x01\x97W\x80c\x18\xD5\xAA\xFE\x14a\x01\x92W\x80c6l\xBA\xB7\x14a\x01\x8DW\x80c;j\xB2\xA9\x14a\x01\x88W\x80cF\xE2\xCC\t\x14a\x01\x83W\x80cH\\\xC9U\x14a\x01~W\x80cK,\x07\x06\x14a\x01yW\x80c[<\xD6\xE2\x14a\x01tW\x80caT8\x01\x14a\x01oW\x80ceX\x95O\x14a\x01jW\x80cp<\xFC\xBB\x14a\x01eW\x80cqP\x18\xA6\x14a\x01`W\x80cz9y\xDC\x14a\x01[W\x80c\x80NQ#\x14a\x01VW\x80c\x82\xF4J\xDE\x14a\x01QW\x80c\x8DZ#\x9B\x14a\x01LW\x80c\x8D\xA5\xCB[\x14a\x01GW\x80c\xAF\xF7Lm\x14a\x01BW\x80c\xC6`\xD3\xF3\x14a\x01=W\x80c\xCD\xAF\xB9x\x14a\x018W\x80c\xD4\xF0\xEBM\x14a\x013W\x80c\xD8x\x13B\x14a\x01.W\x80c\xEAJ\x11\x04\x14a\x01)Wc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\x0C]V[a\x0C$V[a\n\xF8V[a\n\xA1V[a\nOV[a\t\xA5V[a\tpV[a\t;V[a\x08\xE4V[a\x08\xAFV[a\x08{V[a\x08BV[a\x07\xBDV[a\x07\x88V[a\x07DV[a\x06\xD6V[a\x06GV[a\x05yV[a\x05\x04V[a\x04iV[a\x04/V[a\x03\xBAV[a\x02\x95V[a\x02>V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01\xBAWV[a\x01\xACV[\x90V[a\x01\xCB\x90a\x01\xBFV[\x90RV[\x15\x15\x90V[a\x01\xDD\x90a\x01\xCFV[\x90RV[\x90``\x80a\x02'\x93a\x01\xF9_\x82\x01Q_\x86\x01\x90a\x01\xC2V[a\x02\x0B` \x82\x01Q` \x86\x01\x90a\x01\xC2V[a\x02\x1D`@\x82\x01Q`@\x86\x01\x90a\x01\xC2V[\x01Q\x91\x01\x90a\x01\xD4V[V[\x91\x90a\x02<\x90_`\x80\x85\x01\x94\x01\x90a\x01\xE1V[V[4a\x02nWa\x02N6`\x04a\x01\xB0V[a\x02ja\x02Ya\x0E\x01V[a\x02aa\x01\xA2V[\x91\x82\x91\x82a\x02)V[\x03\x90\xF3[a\x01\xA8V[a\x02|\x90a\x01\xCFV[\x90RV[\x91\x90a\x02\x93\x90_` \x85\x01\x94\x01\x90a\x02sV[V[4a\x02\xC5Wa\x02\xA56`\x04a\x01\xB0V[a\x02\xC1a\x02\xB0a\x0E\x9DV[a\x02\xB8a\x01\xA2V[\x91\x82\x91\x82a\x02\x80V[\x03\x90\xF3[a\x01\xA8V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03\x14W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\x0FW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x03\nWV[a\x02\xD6V[a\x02\xD2V[a\x02\xCEV[\x90` \x82\x82\x03\x12a\x03JW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03EWa\x03A\x92\x01a\x02\xDAV[\x90\x91V[a\x02\xCAV[a\x01\xACV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\x90a\x03\x99` \x93a\x03\x9E\x93a\x03\x87\x81a\x03OV[\x93\x84\x80\x93a\x03SV[\x95\x86\x91\x01a\x03\\V[a\x03gV[\x01\x90V[a\x03\xB7\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03qV[\x90V[4a\x03\xEBWa\x03\xE7a\x03\xD6a\x03\xD06`\x04a\x03\x19V[\x90a\x0FAV[a\x03\xDEa\x01\xA2V[\x91\x82\x91\x82a\x03\xA2V[\x03\x90\xF3[a\x01\xA8V[\x1C\x90V[`\xFF\x16\x90V[a\x04\n\x90`\x08a\x04\x0F\x93\x02a\x03\xF0V[a\x03\xF4V[\x90V[\x90a\x04\x1D\x91Ta\x03\xFAV[\x90V[a\x04,`\x04_\x90a\x04\x12V[\x90V[4a\x04_Wa\x04?6`\x04a\x01\xB0V[a\x04[a\x04Ja\x04 V[a\x04Ra\x01\xA2V[\x91\x82\x91\x82a\x02\x80V[\x03\x90\xF3[a\x01\xA8V[_\x01\x90V[4a\x04\x98Wa\x04\x82a\x04|6`\x04a\x03\x19V[\x90a\x11\nV[a\x04\x8Aa\x01\xA2V[\x80a\x04\x94\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\xB1\x90a\x04\x9DV[\x90V[a\x04\xBD\x81a\x04\xA8V[\x03a\x04\xC4WV[_\x80\xFD[\x90P5\x90a\x04\xD5\x82a\x04\xB4V[V[\x91\x90`@\x83\x82\x03\x12a\x04\xFFW\x80a\x04\xF3a\x04\xFC\x92_\x86\x01a\x04\xC8V[\x93` \x01a\x04\xC8V[\x90V[a\x01\xACV[4a\x053Wa\x05\x1Da\x05\x176`\x04a\x04\xD7V[\x90a\x12\xBBV[a\x05%a\x01\xA2V[\x80a\x05/\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[a\x05A\x81a\x01\xBFV[\x03a\x05HWV[_\x80\xFD[\x90P5\x90a\x05Y\x82a\x058V[V[\x90` \x82\x82\x03\x12a\x05tWa\x05q\x91_\x01a\x05LV[\x90V[a\x01\xACV[4a\x05\xA9Wa\x05\xA5a\x05\x94a\x05\x8F6`\x04a\x05[V[a\x12\xC7V[a\x05\x9Ca\x01\xA2V[\x91\x82\x91\x82a\x02)V[\x03\x90\xF3[a\x01\xA8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x05\xC9\x90`\x08a\x05\xCE\x93\x02a\x03\xF0V[a\x05\xAEV[\x90V[\x90a\x05\xDC\x91Ta\x05\xB9V[\x90V[a\x05\xEB`\x01_\x90a\x05\xD1V[\x90V[\x90V[a\x06\x05a\x06\0a\x06\n\x92a\x04\x9DV[a\x05\xEEV[a\x04\x9DV[\x90V[a\x06\x16\x90a\x05\xF1V[\x90V[a\x06\"\x90a\x06\rV[\x90V[a\x06.\x90a\x06\x19V[\x90RV[\x91\x90a\x06E\x90_` \x85\x01\x94\x01\x90a\x06%V[V[4a\x06wWa\x06W6`\x04a\x01\xB0V[a\x06sa\x06ba\x05\xDFV[a\x06ja\x01\xA2V[\x91\x82\x91\x82a\x062V[\x03\x90\xF3[a\x01\xA8V[\x90V[a\x06\x8F\x90`\x08a\x06\x94\x93\x02a\x03\xF0V[a\x06|V[\x90V[\x90a\x06\xA2\x91Ta\x06\x7FV[\x90V[a\x06\xB1`\x03_\x90a\x06\x97V[\x90V[a\x06\xBD\x90a\x01\xBFV[\x90RV[\x91\x90a\x06\xD4\x90_` \x85\x01\x94\x01\x90a\x06\xB4V[V[4a\x07\x06Wa\x06\xE66`\x04a\x01\xB0V[a\x07\x02a\x06\xF1a\x06\xA5V[a\x06\xF9a\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[\x90V[a\x07\"a\x07\x1Da\x07'\x92a\x07\x0BV[a\x05\xEEV[a\x01\xBFV[\x90V[a\x076b'\x8D\0a\x07\x0EV[\x90V[a\x07Aa\x07*V[\x90V[4a\x07tWa\x07T6`\x04a\x01\xB0V[a\x07pa\x07_a\x079V[a\x07ga\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[a\x07\x85`\x02_\x90a\x06\x97V[\x90V[4a\x07\xB8Wa\x07\x986`\x04a\x01\xB0V[a\x07\xB4a\x07\xA3a\x07yV[a\x07\xABa\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[4a\x07\xEBWa\x07\xCD6`\x04a\x01\xB0V[a\x07\xD5a\x13\x0BV[a\x07\xDDa\x01\xA2V[\x80a\x07\xE7\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[\x91``\x83\x83\x03\x12a\x08=Wa\x08\x07\x82_\x85\x01a\x04\xC8V[\x92a\x08\x15\x83` \x83\x01a\x04\xC8V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x088Wa\x084\x92\x01a\x02\xDAV[\x90\x91V[a\x02\xCAV[a\x01\xACV[4a\x08vWa\x08ra\x08aa\x08X6`\x04a\x07\xF0V[\x92\x91\x90\x91a\x13\xC3V[a\x08ia\x01\xA2V[\x91\x82\x91\x82a\x02\x80V[\x03\x90\xF3[a\x01\xA8V[4a\x08\xAAWa\x08\x94a\x08\x8E6`\x04a\x03\x19V[\x90a\x15&V[a\x08\x9Ca\x01\xA2V[\x80a\x08\xA6\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[4a\x08\xDFWa\x08\xBF6`\x04a\x01\xB0V[a\x08\xDBa\x08\xCAa\x15CV[a\x08\xD2a\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[4a\t\x14Wa\x08\xF46`\x04a\x01\xB0V[a\t\x10a\x08\xFFa\x16\x10V[a\t\x07a\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[a\t\"\x90a\x04\xA8V[\x90RV[\x91\x90a\t9\x90_` \x85\x01\x94\x01\x90a\t\x19V[V[4a\tkWa\tK6`\x04a\x01\xB0V[a\tga\tVa\x16\xA2V[a\t^a\x01\xA2V[\x91\x82\x91\x82a\t&V[\x03\x90\xF3[a\x01\xA8V[4a\t\xA0Wa\t\x806`\x04a\x01\xB0V[a\t\x9Ca\t\x8Ba\x16\xD6V[a\t\x93a\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[4a\t\xD5Wa\t\xB56`\x04a\x01\xB0V[a\t\xD1a\t\xC0a\x17\"V[a\t\xC8a\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\n\x14W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\x0FW` \x01\x92` \x83\x02\x84\x01\x11a\n\nWV[a\x02\xD6V[a\x02\xD2V[a\x02\xCEV[\x90` \x82\x82\x03\x12a\nJW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\nEWa\nA\x92\x01a\t\xDAV[\x90\x91V[a\x02\xCAV[a\x01\xACV[4a\n~Wa\nha\nb6`\x04a\n\x19V[\x90a\x19\x08V[a\npa\x01\xA2V[\x80a\nz\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[\x90` \x82\x82\x03\x12a\n\x9CWa\n\x99\x91_\x01a\x04\xC8V[\x90V[a\x01\xACV[4a\n\xCFWa\n\xB9a\n\xB46`\x04a\n\x83V[a\x19\xB8V[a\n\xC1a\x01\xA2V[\x80a\n\xCB\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B(Wa\x0B\x086`\x04a\x01\xB0V[a\x0B$a\x0B\x13a\n\xD4V[a\x0B\x1Ba\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[a\x0BAa\x0B<a\x0BF\x92a\x01\xBFV[a\x05\xEEV[a\x01\xBFV[\x90V[\x90a\x0BS\x90a\x0B-V[_R` R`@_ \x90V[_\x1C\x90V[a\x0Bpa\x0Bu\x91a\x0B_V[a\x06|V[\x90V[a\x0B\x82\x90Ta\x0BdV[\x90V[a\x0B\x91a\x0B\x96\x91a\x0B_V[a\x03\xF4V[\x90V[a\x0B\xA3\x90Ta\x0B\x85V[\x90V[a\x0B\xB1\x90`\x05a\x0BIV[\x90a\x0B\xBD_\x83\x01a\x0BxV[\x91a\x0B\xCA`\x01\x82\x01a\x0BxV[\x91a\x0B\xE3`\x03a\x0B\xDC`\x02\x85\x01a\x0BxV[\x93\x01a\x0B\x99V[\x90V[a\x0C\x1Ba\x0C\"\x94a\x0C\x11``\x94\x98\x97\x95a\x0C\x07`\x80\x86\x01\x9A_\x87\x01\x90a\x06\xB4V[` \x85\x01\x90a\x06\xB4V[`@\x83\x01\x90a\x06\xB4V[\x01\x90a\x02sV[V[4a\x0CXWa\x0CTa\x0C?a\x0C:6`\x04a\x05[V[a\x0B\xA6V[\x90a\x0CK\x94\x92\x94a\x01\xA2V[\x94\x85\x94\x85a\x0B\xE6V[\x03\x90\xF3[a\x01\xA8V[4a\x0C\x8BWa\x0Cua\x0Cp6`\x04a\n\x83V[a\x1A(V[a\x0C}a\x01\xA2V[\x80a\x0C\x87\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x0C\xB2\x90a\x03gV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\xCCW`@RV[a\x0C\x94V[\x90a\x0C\xE4a\x0C\xDDa\x01\xA2V[\x92\x83a\x0C\xA8V[V[a\x0C\xF0`\x80a\x0C\xD1V[\x90V[_\x90V[_\x90V[a\r\x03a\x0C\xE6V[\x90` \x80\x80\x80\x85a\r\x12a\x0C\xF3V[\x81R\x01a\r\x1Da\x0C\xF3V[\x81R\x01a\r(a\x0C\xF3V[\x81R\x01a\r3a\x0C\xF7V[\x81RPPV[a\rAa\x0C\xFBV[\x90V[a\rN`\x80a\x0C\xD1V[\x90V[\x90V[a\rha\rca\rm\x92a\rQV[a\x05\xEEV[a\x01\xBFV[\x90V[\x90a\rz\x90a\x01\xBFV[\x90RV[\x90a\r\x88\x90a\x01\xCFV[\x90RV[\x90a\r\xF3a\r\xEA`\x03a\r\x9Da\x0C\xE6V[\x94a\r\xB4a\r\xAC_\x83\x01a\x0BxV[_\x88\x01a\rpV[a\r\xCCa\r\xC3`\x01\x83\x01a\x0BxV[` \x88\x01a\rpV[a\r\xE4a\r\xDB`\x02\x83\x01a\x0BxV[`@\x88\x01a\rpV[\x01a\x0B\x99V[``\x84\x01a\r~V[V[a\r\xFE\x90a\r\x8CV[\x90V[a\x0E\ta\r9V[Pa\x0E\x1Da\x0E\x17`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x0EAWa\x0E>a\x0E9`\x05a\x0E3`\x03a\x0BxV[\x90a\x0BIV[a\r\xF5V[\x90V[_a\x0E\x96_a\x0E\x8Da\x0E\x84_a\x0E\x7Fa\x0Ev_\x95a\x0Eqa\x0Eia\x0Eca\rDV[\x9Aa\rTV[_\x8B\x01a\rpV[a\rTV[` \x88\x01a\rpV[a\rTV[`@\x85\x01a\rpV[``\x83\x01a\r~V[\x90V[_\x90V[a\x0E\xA5a\x0E\x99V[Pa\x0E\xB0`\x04a\x0B\x99V[\x90V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x0E\xDBa\x0E\xD6a\x0E\xE0\x92a\rQV[a\x0E\xC1V[a\x0E\xB8V[\x90V[\x90V[a\x0E\xF2a\x0E\xF7\x91a\x0E\xB8V[a\x0E\xE3V[\x90RV[\x90P\x90V[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x0F\x1B\x81a\x0F\"\x93a\x0E\xFBV[\x80\x93a\x0F\0V[\x01\x90V[\x80a\x0F7`\x01\x92a\x0F>\x96\x94a\x0E\xE6V[\x01\x91a\x0F\x0BV[\x90V[a\x0F\x7F\x90a\x0FMa\x0E\xB3V[Pa\x0Fpa\x0FZ_a\x0E\xC7V[\x91\x93a\x0Fda\x01\xA2V[\x94\x85\x93` \x85\x01a\x0F&V[` \x82\x01\x81\x03\x82R\x03\x82a\x0C\xA8V[\x90V[\x90a\x0F\x9Ea\x0F\x9832\x90\x85\x85\x91\x92\x90\x91\x92a\x13\xC3V[\x15a\x01\xCFV[a\x0F\xADWa\x0F\xAB\x91a\x10FV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x0F\xC5`\x04\x82\x01a\x04dV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x0F\xECa\x0F\xF2\x91\x93\x92\x93a\x01\xBFV[\x92a\x01\xBFV[\x82\x03\x91\x82\x11a\x0F\xFDWV[a\x0F\xC9V[\x90V[a\x10\x19a\x10\x14a\x10\x1E\x92a\x10\x02V[a\x05\xEEV[a\x01\xBFV[\x90V[a\x100a\x106\x91\x93\x92\x93a\x01\xBFV[\x92a\x01\xBFV[\x82\x01\x80\x92\x11a\x10AWV[a\x0F\xC9V[a\x10ba\x10s\x91a\x10[a\x10x\x94Z\x92a\x10\xC3V[Z\x90a\x0F\xDDV[a\x10ma\x13\x88a\x10\x05V[\x90a\x10!V[a\x1A\x8FV[V[a\x10\x83\x90a\x06\rV[\x90V[\x91\x90a\x10\xA0\x81a\x10\x99\x81a\x10\xA5\x95a\x03SV[\x80\x95a\x0F\0V[a\x03gV[\x01\x90V[\x90\x91a\x10\xC0\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x10\x86V[\x90V[3\x90\x91a\x10\xF0\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10zV[\x92a\x11\x05a\x10\xFCa\x01\xA2V[\x92\x83\x92\x83a\x10\xA9V[\x03\x90\xA2V[\x90a\x11\x14\x91a\x0F\x82V[V[\x90a\x11(\x91a\x11#a\x1B?V[a\x12.V[V[`\xA0\x1C\x90V[a\x11<a\x11A\x91a\x11*V[a\x03\xF4V[\x90V[a\x11N\x90Ta\x110V[\x90V[a\x11ea\x11`a\x11j\x92a\rQV[a\x05\xEEV[a\x04\x9DV[\x90V[a\x11v\x90a\x11QV[\x90V[`\xA0\x1B\x90V[\x90a\x11\x8E`\xFF`\xA0\x1B\x91a\x11yV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x11\xA1\x90a\x01\xCFV[\x90V[\x90V[\x90a\x11\xBCa\x11\xB7a\x11\xC3\x92a\x11\x98V[a\x11\xA4V[\x82Ta\x11\x7FV[\x90UV[a\x11\xD0\x90a\x05\xF1V[\x90V[a\x11\xDC\x90a\x11\xC7V[\x90V[_\x1B\x90V[\x90a\x11\xF5`\x01\x80`\xA0\x1B\x03\x91a\x11\xDFV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x12\x08\x90a\x11\xC7V[\x90V[\x90V[\x90a\x12#a\x12\x1Ea\x12*\x92a\x11\xFFV[a\x12\x0BV[\x82Ta\x11\xE4V[\x90UV[a\x128`\x01a\x11DV[a\x12\xA0W\x81a\x12Wa\x12Qa\x12L_a\x11mV[a\x04\xA8V[\x91a\x04\xA8V[\x14a\x12\x84Wa\x12}a\x12va\x12\x82\x93a\x12q`\x01\x80a\x11\xA7V[a\x11\xD3V[`\x01a\x12\x0EV[a\x1A(V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x12\x9C`\x04\x82\x01a\x04dV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x12\xB7`\x04\x82\x01a\x04dV[\x03\x90\xFD[\x90a\x12\xC5\x91a\x11\x16V[V[a\x12\xDEa\x12\xE3\x91a\x12\xD6a\r9V[P`\x05a\x0BIV[a\r\xF5V[\x90V[a\x12\xEEa\x1B?V[a\x12\xF6a\x12\xF8V[V[a\x13\ta\x13\x04_a\x11mV[a\x1B\xB0V[V[a\x13\x13a\x12\xE6V[V[a\x13!a\x13&\x91a\x0B_V[a\x05\xAEV[\x90V[a\x133\x90Ta\x13\x15V[\x90V[`\xE0\x1B\x90V[a\x13E\x81a\x01\xCFV[\x03a\x13LWV[_\x80\xFD[\x90PQ\x90a\x13]\x82a\x13<V[V[\x90` \x82\x82\x03\x12a\x13xWa\x13u\x91_\x01a\x13PV[\x90V[a\x01\xACV[a\x13\xA3a\x13\xB0\x95\x93\x94\x92\x94a\x13\x99``\x84\x01\x96_\x85\x01\x90a\t\x19V[` \x83\x01\x90a\t\x19V[`@\x81\x85\x03\x91\x01Ra\x10\x86V[\x90V[a\x13\xBBa\x01\xA2V[=_\x82>=\x90\xFD[\x92a\x14\x06` \x93\x94a\x13\xD3a\x0E\x99V[Pa\x14\x11a\x13\xE9a\x13\xE4`\x01a\x13)V[a\x06\x19V[\x93cz9y\xDC\x92\x95\x97a\x13\xFAa\x01\xA2V[\x98\x89\x97\x88\x96\x87\x96a\x136V[\x86R`\x04\x86\x01a\x13}V[\x03\x91Z\xFA\x90\x81\x15a\x14UW_\x91a\x14'W[P\x90V[a\x14H\x91P` =\x81\x11a\x14NW[a\x14@\x81\x83a\x0C\xA8V[\x81\x01\x90a\x13_V[_a\x14#V[P=a\x146V[a\x13\xB3V[\x90a\x14va\x14p32\x90\x85\x85\x91\x92\x90\x91\x92a\x13\xC3V[\x15a\x01\xCFV[a\x14\x85Wa\x14\x83\x91a\x14\xA1V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x14\x9D`\x04\x82\x01a\x04dV[\x03\x90\xFD[a\x14\xBDa\x14\xCE\x91a\x14\xB6a\x14\xD3\x94Z\x92a\x14\xD5V[Z\x90a\x0F\xDDV[a\x14\xC8a\x13\x88a\x10\x05V[\x90a\x10!V[a\x1A\x8FV[V[\x90a\x14\xE1\x903\x92a\x0FAV[\x90a\x15!a\x15\x0F\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10zV[\x92a\x15\x18a\x01\xA2V[\x91\x82\x91\x82a\x03\xA2V[\x03\x90\xA2V[\x90a\x150\x91a\x14ZV[V[_\x90V[a\x15@\x90Qa\x01\xBFV[\x90V[a\x15Ka\x152V[Pa\x15_a\x15Y`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x15\xCFWa\x15\x9Ba\x15\x8D_a\x15\x87a\x15\x82`\x05a\x15|`\x03a\x0BxV[\x90a\x0BIV[a\r\xF5V[\x01a\x156V[a\x15\x95a\x07*V[\x90a\x10!V[Ba\x15\xAEa\x15\xA8\x83a\x01\xBFV[\x91a\x01\xBFV[\x10\x15a\x15\xC2Wa\x15\xBF\x90B\x90a\x0F\xDDV[\x90V[Pa\x15\xCC_a\rTV[\x90V[a\x15\xD8_a\rTV[\x90V[a\x15\xEAa\x15\xF0\x91\x93\x92\x93a\x01\xBFV[\x92a\x01\xBFV[\x91a\x15\xFC\x83\x82\x02a\x01\xBFV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x16\x0BWV[a\x0F\xC9V[a\x16\x18a\x152V[Pa\x16,a\x16&`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x16fWa\x16ca\x16S`\x02a\x16M`\x05a\x16G`\x03a\x0BxV[\x90a\x0BIV[\x01a\x0BxV[a\x16]`\x02a\x0BxV[\x90a\x15\xDBV[\x90V[a\x16o_a\rTV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x16\x8Da\x16\x92\x91a\x0B_V[a\x16vV[\x90V[a\x16\x9F\x90Ta\x16\x81V[\x90V[a\x16\xAAa\x16rV[Pa\x16\xB4_a\x16\x95V[\x90V[\x90V[a\x16\xCEa\x16\xC9a\x16\xD3\x92a\x16\xB7V[a\x05\xEEV[a\x01\xBFV[\x90V[a\x16\xDEa\x152V[Pa\x16\xF2a\x16\xEC`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x17\x16Wa\x17\x13a\x17\x03`\x03a\x0BxV[a\x17\r`\x01a\x16\xBAV[\x90a\x10!V[\x90V[a\x17\x1F_a\rTV[\x90V[a\x17*a\x152V[Pa\x17>a\x178`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x17eWa\x17b`\x02a\x17\\`\x05a\x17V`\x03a\x0BxV[\x90a\x0BIV[\x01a\x0BxV[\x90V[a\x17n_a\rTV[\x90V[a\x17\x8Da\x17\x9E\x91a\x17\x86a\x17\xA3\x94Z\x92a\x18?V[Z\x90a\x0F\xDDV[a\x17\x98a\x13\x88a\x10\x05V[\x90a\x10!V[a\x1A\x8FV[V[P\x90V[`\x01a\x17\xB5\x91\x01a\x01\xBFV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x18\x1AW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x18\x15W` \x01\x91`\x01\x82\x026\x03\x83\x13a\x18\x10WV[a\x17\xD4V[a\x17\xD0V[a\x17\xCCV[\x90\x82\x10\x15a\x18:W` a\x186\x92\x02\x81\x01\x90a\x17\xD8V[\x90\x91V[a\x17\xB8V[a\x18J\x81\x83\x90a\x17\xA5V[\x91a\x18Sa\x152V[Pa\x18]_a\rTV[[\x80a\x18qa\x18k\x86a\x01\xBFV[\x91a\x01\xBFV[\x10\x15a\x19\x02Wa\x18\x9F\x90a\x18\x9532\x90a\x18\x8D\x87\x87\x86\x91a\x18\x1FV[\x92\x90\x91a\x13\xC3V[a\x18\xA4W[a\x17\xA9V[a\x18^V[3a\x18\xBAa\x18\xB4\x86\x86\x85\x91a\x18\x1FV[\x90a\x0FAV[\x90a\x18\xFAa\x18\xE8\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10zV[\x92a\x18\xF1a\x01\xA2V[\x91\x82\x91\x82a\x03\xA2V[\x03\x90\xA2a\x18\x9AV[PPPPV[\x90a\x19\x12\x91a\x17qV[V[a\x19%\x90a\x19 a\x1B?V[a\x19'V[V[\x80a\x19Ba\x19<a\x197_a\x11mV[a\x04\xA8V[\x91a\x04\xA8V[\x14a\x19\x9CWa\x19Za\x19S\x82a\x11\xD3V[`\x01a\x12\x0EV[a\x19\x84\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10zV[\x90a\x19\x8Da\x01\xA2V[\x80a\x19\x97\x81a\x04dV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x19\xB4`\x04\x82\x01a\x04dV[\x03\x90\xFD[a\x19\xC1\x90a\x19\x14V[V[a\x19\xD4\x90a\x19\xCFa\x1B?V[a\x19\xD6V[V[\x80a\x19\xF1a\x19\xEBa\x19\xE6_a\x11mV[a\x04\xA8V[\x91a\x04\xA8V[\x14a\x1A\x01Wa\x19\xFF\x90a\x1B\xB0V[V[a\x1A$a\x1A\r_a\x11mV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\t&V[\x03\x90\xFD[a\x1A1\x90a\x19\xC3V[V[\x90a\x1A?_\x19\x91a\x11\xDFV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x1Aaa\x1A\\a\x1Ah\x92a\x0B-V[a\x1AIV[\x82Ta\x1A3V[\x90UV[\x91` a\x1A\x8D\x92\x94\x93a\x1A\x86`@\x82\x01\x96_\x83\x01\x90a\x06\xB4V[\x01\x90a\x06\xB4V[V[a\x1A\xA2a\x1A\x9C`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x1B2W[a\x1A\xAFa\x1D\xDCV[a\x1A\xE3\x81a\x1A\xDD`\x02a\x1A\xCD`\x05a\x1A\xC7`\x03a\x0BxV[\x90a\x0BIV[\x01\x91a\x1A\xD8\x83a\x0BxV[a\x10!V[\x90a\x1ALV[a\x1A\xED`\x03a\x0BxV[:a\x1B\x18\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0B-V[\x92a\x1B-a\x1B$a\x01\xA2V[\x92\x83\x92\x83a\x1AlV[\x03\x90\xA2V[a\x1B:a\x1C\xD9V[a\x1A\xA7V[a\x1BGa\x16\xA2V[a\x1B`a\x1BZa\x1BUa\x1F\xBAV[a\x04\xA8V[\x91a\x04\xA8V[\x03a\x1BgWV[a\x1B\x89a\x1Bra\x1F\xBAV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\t&V[\x03\x90\xFD[\x90V[\x90a\x1B\xA5a\x1B\xA0a\x1B\xAC\x92a\x10zV[a\x1B\x8DV[\x82Ta\x11\xE4V[\x90UV[a\x1B\xB9_a\x16\x95V[a\x1B\xC3\x82_a\x1B\x90V[\x90a\x1B\xF7a\x1B\xF1\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10zV[\x91a\x10zV[\x91a\x1C\0a\x01\xA2V[\x80a\x1C\n\x81a\x04dV[\x03\x90\xA3V[\x90a\x1C\x1B`\xFF\x91a\x11\xDFV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1C:a\x1C5a\x1CA\x92a\x11\x98V[a\x11\xA4V[\x82Ta\x1C\x0FV[\x90UV[\x90a\x1CO\x90a\rTV[_R` R`@_ \x90V[a\x1Ce\x90Qa\x01\xCFV[\x90V[\x90a\x1C\xC5```\x03a\x1C\xCB\x94a\x1C\x8B_\x82\x01a\x1C\x85_\x88\x01a\x156V[\x90a\x1ALV[a\x1C\xA4`\x01\x82\x01a\x1C\x9E` \x88\x01a\x156V[\x90a\x1ALV[a\x1C\xBD`\x02\x82\x01a\x1C\xB7`@\x88\x01a\x156V[\x90a\x1ALV[\x01\x92\x01a\x1C[V[\x90a\x1C%V[V[\x90a\x1C\xD7\x91a\x1ChV[V[a\x1C\xECa\x1C\xE6`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x1C\xF3W[V[a\x1C\xFF`\x01`\x04a\x1C%V[a\x1D\x12a\x1D\x0B_a\rTV[`\x03a\x1ALV[a\x1DsBa\x1Db_a\x1DYa\x1DP_a\x1DKa\x1DB_\x95a\x1D=a\x1D4a\rDV[\x99_\x8B\x01a\rpV[a\rTV[` \x88\x01a\rpV[a\rTV[`@\x85\x01a\rpV[``\x83\x01a\r~V[a\x1Dn`\x05_\x90a\x1CEV[a\x1C\xCDV[_B\x90a\x1D\xB5a\x1D\xA3\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\rTV[\x92a\x1D\xACa\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xA2a\x1C\xF1V[\x90V[a\x1D\xC9\x90a\x01\xBFV[_\x19\x81\x14a\x1D\xD7W`\x01\x01\x90V[a\x0F\xC9V[a\x1D\xF9a\x1D\xF4`\x05a\x1D\xEE`\x03a\x0BxV[\x90a\x0BIV[a\x1D\xBDV[Ba\x1E'a\x1E!a\x1E\x1Ca\x1E\x0E_\x86\x01a\x0BxV[a\x1E\x16a\x07*V[\x90a\x10!V[a\x01\xBFV[\x91a\x01\xBFV[\x10\x15a\x1E1W[PV[a\x1EYa\x1EPa\x1EB_\x84\x01a\x0BxV[a\x1EJa\x07*V[\x90a\x10!V[`\x01\x83\x01a\x1ALV[a\x1Eg`\x01`\x03\x83\x01a\x1C%V[a\x1Eq`\x03a\x0BxV[a\x1E\x9Ea\x1E\x80`\x02\x84\x01a\x0BxV[\x92a\x1E\x98_a\x1E\x91`\x01\x84\x01a\x0BxV[\x92\x01a\x0BxV[\x90a\x0F\xDDV[a\x1E\xC8\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0B-V[\x92a\x1E\xDDa\x1E\xD4a\x01\xA2V[\x92\x83\x92\x83a\x1AlV[\x03\x90\xA2a\x1E\xFCa\x1E\xF5a\x1E\xF0`\x03a\x0BxV[a\x1D\xC0V[`\x03a\x1ALV[a\x1FfBa\x1FL_a\x1FCa\x1F:_a\x1F5a\x1F,_\x95a\x1F'a\x1F\x1Ea\rDV[\x99_\x8B\x01a\rpV[a\rTV[` \x88\x01a\rpV[a\rTV[`@\x85\x01a\rpV[``\x83\x01a\r~V[a\x1Fa`\x05a\x1F[`\x03a\x0BxV[\x90a\x0BIV[a\x1C\xCDV[a\x1Fp`\x03a\x0BxV[B\x90a\x1F\xB1a\x1F\x9F\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0B-V[\x92a\x1F\xA8a\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xA2_a\x1E.V[a\x1F\xC2a\x16rV[P3\x90V`\x80`@R4`\x1CW`\x0E` V[a\x01\x94a\0+\x829a\x01\x94\x90\xF3[`&V[`@Q\x90V[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x01{V[a\0\x1D_5a\0,V[cz9y\xDC\x03a\0\x0EWa\x01BV[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\0X\x90a\0DV[\x90V[a\0d\x81a\0OV[\x03a\0kWV[_\x80\xFD[\x90P5\x90a\0|\x82a\0[V[V[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\0\xC4W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xBFW` \x01\x92`\x01\x83\x02\x84\x01\x11a\0\xBAWV[a\0\x86V[a\0\x82V[a\0~V[\x91``\x83\x83\x03\x12a\x01\x16Wa\0\xE0\x82_\x85\x01a\0oV[\x92a\0\xEE\x83` \x83\x01a\0oV[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x11Wa\x01\r\x92\x01a\0\x8AV[\x90\x91V[a\0@V[a\0<V[\x15\x15\x90V[a\x01)\x90a\x01\x1BV[\x90RV[\x91\x90a\x01@\x90_` \x85\x01\x94\x01\x90a\x01 V[V[4a\x01vWa\x01ra\x01aa\x01X6`\x04a\0\xC9V[\x92\x91\x90\x91a\x01\x83V[a\x01ia\x002V[\x91\x82\x91\x82a\x01-V[\x03\x90\xF3[a\08V[_\x80\xFD[_\x90V[PPPPa\x01\x8Fa\x01\x7FV[P_\x90V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b610c90565b61001d5f3561019c565b8063086146d21461019757806318d5aafe14610192578063366cbab71461018d5780633b6ab2a91461018857806346e2cc0914610183578063485cc9551461017e5780634b2c0706146101795780635b3cd6e214610174578063615438011461016f5780636558954f1461016a578063703cfcbb14610165578063715018a6146101605780637a3979dc1461015b578063804e51231461015657806382f44ade146101515780638d5a239b1461014c5780638da5cb5b14610147578063aff74c6d14610142578063c660d3f31461013d578063cdafb97814610138578063d4f0eb4d14610133578063d87813421461012e578063ea4a1104146101295763f2fde38b0361000e57610c5d565b610c24565b610af8565b610aa1565b610a4f565b6109a5565b610970565b61093b565b6108e4565b6108af565b61087b565b610842565b6107bd565b610788565b610744565b6106d6565b610647565b610579565b610504565b610469565b61042f565b6103ba565b610295565b61023e565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f9103126101ba57565b6101ac565b90565b6101cb906101bf565b9052565b151590565b6101dd906101cf565b9052565b90606080610227936101f95f8201515f8601906101c2565b61020b602082015160208601906101c2565b61021d604082015160408601906101c2565b01519101906101d4565b565b919061023c905f608085019401906101e1565b565b3461026e5761024e3660046101b0565b61026a610259610e01565b6102616101a2565b91829182610229565b0390f35b6101a8565b61027c906101cf565b9052565b9190610293905f60208501940190610273565b565b346102c5576102a53660046101b0565b6102c16102b0610e9d565b6102b86101a2565b91829182610280565b0390f35b6101a8565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b909182601f830112156103145781359167ffffffffffffffff831161030f57602001926001830284011161030a57565b6102d6565b6102d2565b6102ce565b9060208282031261034a575f82013567ffffffffffffffff81116103455761034192016102da565b9091565b6102ca565b6101ac565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61039061039960209361039e936103878161034f565b93848093610353565b9586910161035c565b610367565b0190565b6103b79160208201915f818403910152610371565b90565b346103eb576103e76103d66103d0366004610319565b90610f41565b6103de6101a2565b918291826103a2565b0390f35b6101a8565b1c90565b60ff1690565b61040a90600861040f93026103f0565b6103f4565b90565b9061041d91546103fa565b90565b61042c60045f90610412565b90565b3461045f5761043f3660046101b0565b61045b61044a610420565b6104526101a2565b91829182610280565b0390f35b6101a8565b5f0190565b346104985761048261047c366004610319565b9061110a565b61048a6101a2565b8061049481610464565b0390f35b6101a8565b60018060a01b031690565b6104b19061049d565b90565b6104bd816104a8565b036104c457565b5f80fd5b905035906104d5826104b4565b565b91906040838203126104ff57806104f36104fc925f86016104c8565b936020016104c8565b90565b6101ac565b346105335761051d6105173660046104d7565b906112bb565b6105256101a2565b8061052f81610464565b0390f35b6101a8565b610541816101bf565b0361054857565b5f80fd5b9050359061055982610538565b565b9060208282031261057457610571915f0161054c565b90565b6101ac565b346105a9576105a561059461058f36600461055b565b6112c7565b61059c6101a2565b91829182610229565b0390f35b6101a8565b60018060a01b031690565b6105c99060086105ce93026103f0565b6105ae565b90565b906105dc91546105b9565b90565b6105eb60015f906105d1565b90565b90565b61060561060061060a9261049d565b6105ee565b61049d565b90565b610616906105f1565b90565b6106229061060d565b90565b61062e90610619565b9052565b9190610645905f60208501940190610625565b565b34610677576106573660046101b0565b6106736106626105df565b61066a6101a2565b91829182610632565b0390f35b6101a8565b90565b61068f90600861069493026103f0565b61067c565b90565b906106a2915461067f565b90565b6106b160035f90610697565b90565b6106bd906101bf565b9052565b91906106d4905f602085019401906106b4565b565b34610706576106e63660046101b0565b6107026106f16106a5565b6106f96101a2565b918291826106c1565b0390f35b6101a8565b90565b61072261071d6107279261070b565b6105ee565b6101bf565b90565b61073662278d0061070e565b90565b61074161072a565b90565b34610774576107543660046101b0565b61077061075f610739565b6107676101a2565b918291826106c1565b0390f35b6101a8565b61078560025f90610697565b90565b346107b8576107983660046101b0565b6107b46107a3610779565b6107ab6101a2565b918291826106c1565b0390f35b6101a8565b346107eb576107cd3660046101b0565b6107d561130b565b6107dd6101a2565b806107e781610464565b0390f35b6101a8565b9160608383031261083d57610807825f85016104c8565b9261081583602083016104c8565b92604082013567ffffffffffffffff81116108385761083492016102da565b9091565b6102ca565b6101ac565b34610876576108726108616108583660046107f0565b929190916113c3565b6108696101a2565b91829182610280565b0390f35b6101a8565b346108aa5761089461088e366004610319565b90611526565b61089c6101a2565b806108a681610464565b0390f35b6101a8565b346108df576108bf3660046101b0565b6108db6108ca611543565b6108d26101a2565b918291826106c1565b0390f35b6101a8565b34610914576108f43660046101b0565b6109106108ff611610565b6109076101a2565b918291826106c1565b0390f35b6101a8565b610922906104a8565b9052565b9190610939905f60208501940190610919565b565b3461096b5761094b3660046101b0565b6109676109566116a2565b61095e6101a2565b91829182610926565b0390f35b6101a8565b346109a0576109803660046101b0565b61099c61098b6116d6565b6109936101a2565b918291826106c1565b0390f35b6101a8565b346109d5576109b53660046101b0565b6109d16109c0611722565b6109c86101a2565b918291826106c1565b0390f35b6101a8565b909182601f83011215610a145781359167ffffffffffffffff8311610a0f576020019260208302840111610a0a57565b6102d6565b6102d2565b6102ce565b90602082820312610a4a575f82013567ffffffffffffffff8111610a4557610a4192016109da565b9091565b6102ca565b6101ac565b34610a7e57610a68610a62366004610a19565b90611908565b610a706101a2565b80610a7a81610464565b0390f35b6101a8565b90602082820312610a9c57610a99915f016104c8565b90565b6101ac565b34610acf57610ab9610ab4366004610a83565b6119b8565b610ac16101a2565b80610acb81610464565b0390f35b6101a8565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610b2857610b083660046101b0565b610b24610b13610ad4565b610b1b6101a2565b918291826106c1565b0390f35b6101a8565b610b41610b3c610b46926101bf565b6105ee565b6101bf565b90565b90610b5390610b2d565b5f5260205260405f2090565b5f1c90565b610b70610b7591610b5f565b61067c565b90565b610b829054610b64565b90565b610b91610b9691610b5f565b6103f4565b90565b610ba39054610b85565b90565b610bb1906005610b49565b90610bbd5f8301610b78565b91610bca60018201610b78565b91610be36003610bdc60028501610b78565b9301610b99565b90565b610c1b610c2294610c11606094989795610c07608086019a5f8701906106b4565b60208501906106b4565b60408301906106b4565b0190610273565b565b34610c5857610c54610c3f610c3a36600461055b565b610ba6565b90610c4b9492946101a2565b94859485610be6565b0390f35b6101a8565b34610c8b57610c75610c70366004610a83565b611a28565b610c7d6101a2565b80610c8781610464565b0390f35b6101a8565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b90610cb290610367565b810190811067ffffffffffffffff821117610ccc57604052565b610c94565b90610ce4610cdd6101a2565b9283610ca8565b565b610cf06080610cd1565b90565b5f90565b5f90565b610d03610ce6565b90602080808085610d12610cf3565b815201610d1d610cf3565b815201610d28610cf3565b815201610d33610cf7565b81525050565b610d41610cfb565b90565b610d4e6080610cd1565b90565b90565b610d68610d63610d6d92610d51565b6105ee565b6101bf565b90565b90610d7a906101bf565b9052565b90610d88906101cf565b9052565b90610df3610dea6003610d9d610ce6565b94610db4610dac5f8301610b78565b5f8801610d70565b610dcc610dc360018301610b78565b60208801610d70565b610de4610ddb60028301610b78565b60408801610d70565b01610b99565b60608401610d7e565b565b610dfe90610d8c565b90565b610e09610d39565b50610e1d610e176004610b99565b156101cf565b610e4157610e3e610e396005610e336003610b78565b90610b49565b610df5565b90565b5f610e965f610e8d610e845f610e7f610e765f95610e71610e69610e63610d44565b9a610d54565b5f8b01610d70565b610d54565b60208801610d70565b610d54565b60408501610d70565b60608301610d7e565b90565b5f90565b610ea5610e99565b50610eb06004610b99565b90565b606090565b60ff60f81b1690565b60f81b90565b610edb610ed6610ee092610d51565b610ec1565b610eb8565b90565b90565b610ef2610ef791610eb8565b610ee3565b9052565b905090565b90825f939282370152565b909182610f1b81610f2293610efb565b8093610f00565b0190565b80610f37600192610f3e9694610ee6565b0191610f0b565b90565b610f7f90610f4d610eb3565b50610f70610f5a5f610ec7565b9193610f646101a2565b94859360208501610f26565b60208201810382520382610ca8565b90565b90610f9e610f98333290858591929091926113c3565b156101cf565b610fad57610fab91611046565b565b5f631b8e828b60e31b815280610fc560048201610464565b0390fd5b634e487b7160e01b5f52601160045260245ffd5b610fec610ff2919392936101bf565b926101bf565b8203918211610ffd57565b610fc9565b90565b61101961101461101e92611002565b6105ee565b6101bf565b90565b611030611036919392936101bf565b926101bf565b820180921161104157565b610fc9565b6110626110739161105b611078945a926110c3565b5a90610fdd565b61106d611388611005565b90611021565b611a8f565b565b6110839061060d565b90565b91906110a081611099816110a595610353565b8095610f00565b610367565b0190565b90916110c09260208301925f818503910152611086565b90565b3390916110f07f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f9261107a565b926111056110fc6101a2565b928392836110a9565b0390a2565b9061111491610f82565b565b9061112891611123611b3f565b61122e565b565b60a01c90565b61113c6111419161112a565b6103f4565b90565b61114e9054611130565b90565b61116561116061116a92610d51565b6105ee565b61049d565b90565b61117690611151565b90565b60a01b90565b9061118e60ff60a01b91611179565b9181191691161790565b6111a1906101cf565b90565b90565b906111bc6111b76111c392611198565b6111a4565b825461117f565b9055565b6111d0906105f1565b90565b6111dc906111c7565b90565b5f1b90565b906111f560018060a01b03916111df565b9181191691161790565b611208906111c7565b90565b90565b9061122361121e61122a926111ff565b61120b565b82546111e4565b9055565b6112386001611144565b6112a0578161125761125161124c5f61116d565b6104a8565b916104a8565b146112845761127d611276611282936112716001806111a7565b6111d3565b600161120e565b611a28565b565b5f632e7f3c7f60e11b81528061129c60048201610464565b0390fd5b5f62dc149f60e41b8152806112b760048201610464565b0390fd5b906112c591611116565b565b6112de6112e3916112d6610d39565b506005610b49565b610df5565b90565b6112ee611b3f565b6112f66112f8565b565b6113096113045f61116d565b611bb0565b565b6113136112e6565b565b61132161132691610b5f565b6105ae565b90565b6113339054611315565b90565b60e01b90565b611345816101cf565b0361134c57565b5f80fd5b9050519061135d8261133c565b565b9060208282031261137857611375915f01611350565b90565b6101ac565b6113a36113b0959394929461139960608401965f850190610919565b6020830190610919565b6040818503910152611086565b90565b6113bb6101a2565b3d5f823e3d90fd5b92611406602093946113d3610e99565b506114116113e96113e46001611329565b610619565b93637a3979dc9295976113fa6101a2565b98899788968796611336565b86526004860161137d565b03915afa908115611455575f91611427575b5090565b611448915060203d811161144e575b6114408183610ca8565b81019061135f565b5f611423565b503d611436565b6113b3565b90611476611470333290858591929091926113c3565b156101cf565b61148557611483916114a1565b565b5f631b8e828b60e31b81528061149d60048201610464565b0390fd5b6114bd6114ce916114b66114d3945a926114d5565b5a90610fdd565b6114c8611388611005565b90611021565b611a8f565b565b906114e1903392610f41565b9061152161150f7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f9261107a565b926115186101a2565b918291826103a2565b0390a2565b906115309161145a565b565b5f90565b61154090516101bf565b90565b61154b611532565b5061155f6115596004610b99565b156101cf565b6115cf5761159b61158d5f611587611582600561157c6003610b78565b90610b49565b610df5565b01611536565b61159561072a565b90611021565b426115ae6115a8836101bf565b916101bf565b10156115c2576115bf904290610fdd565b90565b506115cc5f610d54565b90565b6115d85f610d54565b90565b6115ea6115f0919392936101bf565b926101bf565b916115fc8382026101bf565b92818404149015171561160b57565b610fc9565b611618611532565b5061162c6116266004610b99565b156101cf565b61166657611663611653600261164d60056116476003610b78565b90610b49565b01610b78565b61165d6002610b78565b906115db565b90565b61166f5f610d54565b90565b5f90565b60018060a01b031690565b61168d61169291610b5f565b611676565b90565b61169f9054611681565b90565b6116aa611672565b506116b45f611695565b90565b90565b6116ce6116c96116d3926116b7565b6105ee565b6101bf565b90565b6116de611532565b506116f26116ec6004610b99565b156101cf565b611716576117136117036003610b78565b61170d60016116ba565b90611021565b90565b61171f5f610d54565b90565b61172a611532565b5061173e6117386004610b99565b156101cf565b61176557611762600261175c60056117566003610b78565b90610b49565b01610b78565b90565b61176e5f610d54565b90565b61178d61179e916117866117a3945a9261183f565b5a90610fdd565b611798611388611005565b90611021565b611a8f565b565b5090565b60016117b591016101bf565b90565b634e487b7160e01b5f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b90359060016020038136030382121561181a570180359067ffffffffffffffff82116118155760200191600182023603831361181057565b6117d4565b6117d0565b6117cc565b9082101561183a57602061183692028101906117d8565b9091565b6117b8565b61184a8183906117a5565b91611853611532565b5061185d5f610d54565b5b8061187161186b866101bf565b916101bf565b10156119025761189f9061189533329061188d8787869161181f565b9290916113c3565b6118a4575b6117a9565b61185e565b336118ba6118b48686859161181f565b90610f41565b906118fa6118e87f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f9261107a565b926118f16101a2565b918291826103a2565b0390a261189a565b50505050565b9061191291611771565b565b61192590611920611b3f565b611927565b565b8061194261193c6119375f61116d565b6104a8565b916104a8565b1461199c5761195a611953826111d3565b600161120e565b6119847f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b99161107a565b9061198d6101a2565b8061199781610464565b0390a2565b5f632e7f3c7f60e11b8152806119b460048201610464565b0390fd5b6119c190611914565b565b6119d4906119cf611b3f565b6119d6565b565b806119f16119eb6119e65f61116d565b6104a8565b916104a8565b14611a01576119ff90611bb0565b565b611a24611a0d5f61116d565b5f918291631e4fbdf760e01b835260048301610926565b0390fd5b611a31906119c3565b565b90611a3f5f19916111df565b9181191691161790565b90565b90611a61611a5c611a6892610b2d565b611a49565b8254611a33565b9055565b916020611a8d929493611a8660408201965f8301906106b4565b01906106b4565b565b611aa2611a9c6004610b99565b156101cf565b611b32575b611aaf611ddc565b611ae381611add6002611acd6005611ac76003610b78565b90610b49565b0191611ad883610b78565b611021565b90611a4c565b611aed6003610b78565b3a611b187f2d9c47ad553b63bbbad1819d4fd97da088505c96a58182691b8abb5f2bcc29ee92610b2d565b92611b2d611b246101a2565b92839283611a6c565b0390a2565b611b3a611cd9565b611aa7565b611b476116a2565b611b60611b5a611b55611fba565b6104a8565b916104a8565b03611b6757565b611b89611b72611fba565b5f91829163118cdaa760e01b835260048301610926565b0390fd5b90565b90611ba5611ba0611bac9261107a565b611b8d565b82546111e4565b9055565b611bb95f611695565b611bc3825f611b90565b90611bf7611bf17f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09361107a565b9161107a565b91611c006101a2565b80611c0a81610464565b0390a3565b90611c1b60ff916111df565b9181191691161790565b90611c3a611c35611c4192611198565b6111a4565b8254611c0f565b9055565b90611c4f90610d54565b5f5260205260405f2090565b611c6590516101cf565b90565b90611cc560606003611ccb94611c8b5f8201611c855f8801611536565b90611a4c565b611ca460018201611c9e60208801611536565b90611a4c565b611cbd60028201611cb760408801611536565b90611a4c565b019201611c5b565b90611c25565b565b90611cd791611c68565b565b611cec611ce66004610b99565b156101cf565b611cf3575b565b611cff60016004611c25565b611d12611d0b5f610d54565b6003611a4c565b611d7342611d625f611d59611d505f611d4b611d425f95611d3d611d34610d44565b995f8b01610d70565b610d54565b60208801610d70565b610d54565b60408501610d70565b60608301610d7e565b611d6e60055f90611c45565b611ccd565b5f4290611db5611da37f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610d54565b92611dac6101a2565b918291826106c1565b0390a2611cf1565b90565b611dc9906101bf565b5f198114611dd75760010190565b610fc9565b611df9611df46005611dee6003610b78565b90610b49565b611dbd565b42611e27611e21611e1c611e0e5f8601610b78565b611e1661072a565b90611021565b6101bf565b916101bf565b1015611e31575b50565b611e59611e50611e425f8401610b78565b611e4a61072a565b90611021565b60018301611a4c565b611e67600160038301611c25565b611e716003610b78565b611e9e611e8060028401610b78565b92611e985f611e9160018401610b78565b9201610b78565b90610fdd565b611ec87f48a298f9d376b82a7174a798e90cf1209495fdd68b0c11eb11beabacc2d29cf592610b2d565b92611edd611ed46101a2565b92839283611a6c565b0390a2611efc611ef5611ef06003610b78565b611dc0565b6003611a4c565b611f6642611f4c5f611f43611f3a5f611f35611f2c5f95611f27611f1e610d44565b995f8b01610d70565b610d54565b60208801610d70565b610d54565b60408501610d70565b60608301610d7e565b611f616005611f5b6003610b78565b90610b49565b611ccd565b611f706003610b78565b4290611fb1611f9f7f41f1e08f21cc818cf0cffb3a62609fb6a3cbc9b3671b011e285e17a1ebb4688e92610b2d565b92611fa86101a2565b918291826106c1565b0390a25f611e2e565b611fc2611672565b50339056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x0C\x90V[a\0\x1D_5a\x01\x9CV[\x80c\x08aF\xD2\x14a\x01\x97W\x80c\x18\xD5\xAA\xFE\x14a\x01\x92W\x80c6l\xBA\xB7\x14a\x01\x8DW\x80c;j\xB2\xA9\x14a\x01\x88W\x80cF\xE2\xCC\t\x14a\x01\x83W\x80cH\\\xC9U\x14a\x01~W\x80cK,\x07\x06\x14a\x01yW\x80c[<\xD6\xE2\x14a\x01tW\x80caT8\x01\x14a\x01oW\x80ceX\x95O\x14a\x01jW\x80cp<\xFC\xBB\x14a\x01eW\x80cqP\x18\xA6\x14a\x01`W\x80cz9y\xDC\x14a\x01[W\x80c\x80NQ#\x14a\x01VW\x80c\x82\xF4J\xDE\x14a\x01QW\x80c\x8DZ#\x9B\x14a\x01LW\x80c\x8D\xA5\xCB[\x14a\x01GW\x80c\xAF\xF7Lm\x14a\x01BW\x80c\xC6`\xD3\xF3\x14a\x01=W\x80c\xCD\xAF\xB9x\x14a\x018W\x80c\xD4\xF0\xEBM\x14a\x013W\x80c\xD8x\x13B\x14a\x01.W\x80c\xEAJ\x11\x04\x14a\x01)Wc\xF2\xFD\xE3\x8B\x03a\0\x0EWa\x0C]V[a\x0C$V[a\n\xF8V[a\n\xA1V[a\nOV[a\t\xA5V[a\tpV[a\t;V[a\x08\xE4V[a\x08\xAFV[a\x08{V[a\x08BV[a\x07\xBDV[a\x07\x88V[a\x07DV[a\x06\xD6V[a\x06GV[a\x05yV[a\x05\x04V[a\x04iV[a\x04/V[a\x03\xBAV[a\x02\x95V[a\x02>V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01\xBAWV[a\x01\xACV[\x90V[a\x01\xCB\x90a\x01\xBFV[\x90RV[\x15\x15\x90V[a\x01\xDD\x90a\x01\xCFV[\x90RV[\x90``\x80a\x02'\x93a\x01\xF9_\x82\x01Q_\x86\x01\x90a\x01\xC2V[a\x02\x0B` \x82\x01Q` \x86\x01\x90a\x01\xC2V[a\x02\x1D`@\x82\x01Q`@\x86\x01\x90a\x01\xC2V[\x01Q\x91\x01\x90a\x01\xD4V[V[\x91\x90a\x02<\x90_`\x80\x85\x01\x94\x01\x90a\x01\xE1V[V[4a\x02nWa\x02N6`\x04a\x01\xB0V[a\x02ja\x02Ya\x0E\x01V[a\x02aa\x01\xA2V[\x91\x82\x91\x82a\x02)V[\x03\x90\xF3[a\x01\xA8V[a\x02|\x90a\x01\xCFV[\x90RV[\x91\x90a\x02\x93\x90_` \x85\x01\x94\x01\x90a\x02sV[V[4a\x02\xC5Wa\x02\xA56`\x04a\x01\xB0V[a\x02\xC1a\x02\xB0a\x0E\x9DV[a\x02\xB8a\x01\xA2V[\x91\x82\x91\x82a\x02\x80V[\x03\x90\xF3[a\x01\xA8V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x03\x14W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\x0FW` \x01\x92`\x01\x83\x02\x84\x01\x11a\x03\nWV[a\x02\xD6V[a\x02\xD2V[a\x02\xCEV[\x90` \x82\x82\x03\x12a\x03JW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03EWa\x03A\x92\x01a\x02\xDAV[\x90\x91V[a\x02\xCAV[a\x01\xACV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\x90a\x03\x99` \x93a\x03\x9E\x93a\x03\x87\x81a\x03OV[\x93\x84\x80\x93a\x03SV[\x95\x86\x91\x01a\x03\\V[a\x03gV[\x01\x90V[a\x03\xB7\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03qV[\x90V[4a\x03\xEBWa\x03\xE7a\x03\xD6a\x03\xD06`\x04a\x03\x19V[\x90a\x0FAV[a\x03\xDEa\x01\xA2V[\x91\x82\x91\x82a\x03\xA2V[\x03\x90\xF3[a\x01\xA8V[\x1C\x90V[`\xFF\x16\x90V[a\x04\n\x90`\x08a\x04\x0F\x93\x02a\x03\xF0V[a\x03\xF4V[\x90V[\x90a\x04\x1D\x91Ta\x03\xFAV[\x90V[a\x04,`\x04_\x90a\x04\x12V[\x90V[4a\x04_Wa\x04?6`\x04a\x01\xB0V[a\x04[a\x04Ja\x04 V[a\x04Ra\x01\xA2V[\x91\x82\x91\x82a\x02\x80V[\x03\x90\xF3[a\x01\xA8V[_\x01\x90V[4a\x04\x98Wa\x04\x82a\x04|6`\x04a\x03\x19V[\x90a\x11\nV[a\x04\x8Aa\x01\xA2V[\x80a\x04\x94\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\xB1\x90a\x04\x9DV[\x90V[a\x04\xBD\x81a\x04\xA8V[\x03a\x04\xC4WV[_\x80\xFD[\x90P5\x90a\x04\xD5\x82a\x04\xB4V[V[\x91\x90`@\x83\x82\x03\x12a\x04\xFFW\x80a\x04\xF3a\x04\xFC\x92_\x86\x01a\x04\xC8V[\x93` \x01a\x04\xC8V[\x90V[a\x01\xACV[4a\x053Wa\x05\x1Da\x05\x176`\x04a\x04\xD7V[\x90a\x12\xBBV[a\x05%a\x01\xA2V[\x80a\x05/\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[a\x05A\x81a\x01\xBFV[\x03a\x05HWV[_\x80\xFD[\x90P5\x90a\x05Y\x82a\x058V[V[\x90` \x82\x82\x03\x12a\x05tWa\x05q\x91_\x01a\x05LV[\x90V[a\x01\xACV[4a\x05\xA9Wa\x05\xA5a\x05\x94a\x05\x8F6`\x04a\x05[V[a\x12\xC7V[a\x05\x9Ca\x01\xA2V[\x91\x82\x91\x82a\x02)V[\x03\x90\xF3[a\x01\xA8V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x05\xC9\x90`\x08a\x05\xCE\x93\x02a\x03\xF0V[a\x05\xAEV[\x90V[\x90a\x05\xDC\x91Ta\x05\xB9V[\x90V[a\x05\xEB`\x01_\x90a\x05\xD1V[\x90V[\x90V[a\x06\x05a\x06\0a\x06\n\x92a\x04\x9DV[a\x05\xEEV[a\x04\x9DV[\x90V[a\x06\x16\x90a\x05\xF1V[\x90V[a\x06\"\x90a\x06\rV[\x90V[a\x06.\x90a\x06\x19V[\x90RV[\x91\x90a\x06E\x90_` \x85\x01\x94\x01\x90a\x06%V[V[4a\x06wWa\x06W6`\x04a\x01\xB0V[a\x06sa\x06ba\x05\xDFV[a\x06ja\x01\xA2V[\x91\x82\x91\x82a\x062V[\x03\x90\xF3[a\x01\xA8V[\x90V[a\x06\x8F\x90`\x08a\x06\x94\x93\x02a\x03\xF0V[a\x06|V[\x90V[\x90a\x06\xA2\x91Ta\x06\x7FV[\x90V[a\x06\xB1`\x03_\x90a\x06\x97V[\x90V[a\x06\xBD\x90a\x01\xBFV[\x90RV[\x91\x90a\x06\xD4\x90_` \x85\x01\x94\x01\x90a\x06\xB4V[V[4a\x07\x06Wa\x06\xE66`\x04a\x01\xB0V[a\x07\x02a\x06\xF1a\x06\xA5V[a\x06\xF9a\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[\x90V[a\x07\"a\x07\x1Da\x07'\x92a\x07\x0BV[a\x05\xEEV[a\x01\xBFV[\x90V[a\x076b'\x8D\0a\x07\x0EV[\x90V[a\x07Aa\x07*V[\x90V[4a\x07tWa\x07T6`\x04a\x01\xB0V[a\x07pa\x07_a\x079V[a\x07ga\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[a\x07\x85`\x02_\x90a\x06\x97V[\x90V[4a\x07\xB8Wa\x07\x986`\x04a\x01\xB0V[a\x07\xB4a\x07\xA3a\x07yV[a\x07\xABa\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[4a\x07\xEBWa\x07\xCD6`\x04a\x01\xB0V[a\x07\xD5a\x13\x0BV[a\x07\xDDa\x01\xA2V[\x80a\x07\xE7\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[\x91``\x83\x83\x03\x12a\x08=Wa\x08\x07\x82_\x85\x01a\x04\xC8V[\x92a\x08\x15\x83` \x83\x01a\x04\xC8V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x088Wa\x084\x92\x01a\x02\xDAV[\x90\x91V[a\x02\xCAV[a\x01\xACV[4a\x08vWa\x08ra\x08aa\x08X6`\x04a\x07\xF0V[\x92\x91\x90\x91a\x13\xC3V[a\x08ia\x01\xA2V[\x91\x82\x91\x82a\x02\x80V[\x03\x90\xF3[a\x01\xA8V[4a\x08\xAAWa\x08\x94a\x08\x8E6`\x04a\x03\x19V[\x90a\x15&V[a\x08\x9Ca\x01\xA2V[\x80a\x08\xA6\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[4a\x08\xDFWa\x08\xBF6`\x04a\x01\xB0V[a\x08\xDBa\x08\xCAa\x15CV[a\x08\xD2a\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[4a\t\x14Wa\x08\xF46`\x04a\x01\xB0V[a\t\x10a\x08\xFFa\x16\x10V[a\t\x07a\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[a\t\"\x90a\x04\xA8V[\x90RV[\x91\x90a\t9\x90_` \x85\x01\x94\x01\x90a\t\x19V[V[4a\tkWa\tK6`\x04a\x01\xB0V[a\tga\tVa\x16\xA2V[a\t^a\x01\xA2V[\x91\x82\x91\x82a\t&V[\x03\x90\xF3[a\x01\xA8V[4a\t\xA0Wa\t\x806`\x04a\x01\xB0V[a\t\x9Ca\t\x8Ba\x16\xD6V[a\t\x93a\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[4a\t\xD5Wa\t\xB56`\x04a\x01\xB0V[a\t\xD1a\t\xC0a\x17\"V[a\t\xC8a\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[\x90\x91\x82`\x1F\x83\x01\x12\x15a\n\x14W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\n\x0FW` \x01\x92` \x83\x02\x84\x01\x11a\n\nWV[a\x02\xD6V[a\x02\xD2V[a\x02\xCEV[\x90` \x82\x82\x03\x12a\nJW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\nEWa\nA\x92\x01a\t\xDAV[\x90\x91V[a\x02\xCAV[a\x01\xACV[4a\n~Wa\nha\nb6`\x04a\n\x19V[\x90a\x19\x08V[a\npa\x01\xA2V[\x80a\nz\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[\x90` \x82\x82\x03\x12a\n\x9CWa\n\x99\x91_\x01a\x04\xC8V[\x90V[a\x01\xACV[4a\n\xCFWa\n\xB9a\n\xB46`\x04a\n\x83V[a\x19\xB8V[a\n\xC1a\x01\xA2V[\x80a\n\xCB\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B(Wa\x0B\x086`\x04a\x01\xB0V[a\x0B$a\x0B\x13a\n\xD4V[a\x0B\x1Ba\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xF3[a\x01\xA8V[a\x0BAa\x0B<a\x0BF\x92a\x01\xBFV[a\x05\xEEV[a\x01\xBFV[\x90V[\x90a\x0BS\x90a\x0B-V[_R` R`@_ \x90V[_\x1C\x90V[a\x0Bpa\x0Bu\x91a\x0B_V[a\x06|V[\x90V[a\x0B\x82\x90Ta\x0BdV[\x90V[a\x0B\x91a\x0B\x96\x91a\x0B_V[a\x03\xF4V[\x90V[a\x0B\xA3\x90Ta\x0B\x85V[\x90V[a\x0B\xB1\x90`\x05a\x0BIV[\x90a\x0B\xBD_\x83\x01a\x0BxV[\x91a\x0B\xCA`\x01\x82\x01a\x0BxV[\x91a\x0B\xE3`\x03a\x0B\xDC`\x02\x85\x01a\x0BxV[\x93\x01a\x0B\x99V[\x90V[a\x0C\x1Ba\x0C\"\x94a\x0C\x11``\x94\x98\x97\x95a\x0C\x07`\x80\x86\x01\x9A_\x87\x01\x90a\x06\xB4V[` \x85\x01\x90a\x06\xB4V[`@\x83\x01\x90a\x06\xB4V[\x01\x90a\x02sV[V[4a\x0CXWa\x0CTa\x0C?a\x0C:6`\x04a\x05[V[a\x0B\xA6V[\x90a\x0CK\x94\x92\x94a\x01\xA2V[\x94\x85\x94\x85a\x0B\xE6V[\x03\x90\xF3[a\x01\xA8V[4a\x0C\x8BWa\x0Cua\x0Cp6`\x04a\n\x83V[a\x1A(V[a\x0C}a\x01\xA2V[\x80a\x0C\x87\x81a\x04dV[\x03\x90\xF3[a\x01\xA8V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x0C\xB2\x90a\x03gV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\xCCW`@RV[a\x0C\x94V[\x90a\x0C\xE4a\x0C\xDDa\x01\xA2V[\x92\x83a\x0C\xA8V[V[a\x0C\xF0`\x80a\x0C\xD1V[\x90V[_\x90V[_\x90V[a\r\x03a\x0C\xE6V[\x90` \x80\x80\x80\x85a\r\x12a\x0C\xF3V[\x81R\x01a\r\x1Da\x0C\xF3V[\x81R\x01a\r(a\x0C\xF3V[\x81R\x01a\r3a\x0C\xF7V[\x81RPPV[a\rAa\x0C\xFBV[\x90V[a\rN`\x80a\x0C\xD1V[\x90V[\x90V[a\rha\rca\rm\x92a\rQV[a\x05\xEEV[a\x01\xBFV[\x90V[\x90a\rz\x90a\x01\xBFV[\x90RV[\x90a\r\x88\x90a\x01\xCFV[\x90RV[\x90a\r\xF3a\r\xEA`\x03a\r\x9Da\x0C\xE6V[\x94a\r\xB4a\r\xAC_\x83\x01a\x0BxV[_\x88\x01a\rpV[a\r\xCCa\r\xC3`\x01\x83\x01a\x0BxV[` \x88\x01a\rpV[a\r\xE4a\r\xDB`\x02\x83\x01a\x0BxV[`@\x88\x01a\rpV[\x01a\x0B\x99V[``\x84\x01a\r~V[V[a\r\xFE\x90a\r\x8CV[\x90V[a\x0E\ta\r9V[Pa\x0E\x1Da\x0E\x17`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x0EAWa\x0E>a\x0E9`\x05a\x0E3`\x03a\x0BxV[\x90a\x0BIV[a\r\xF5V[\x90V[_a\x0E\x96_a\x0E\x8Da\x0E\x84_a\x0E\x7Fa\x0Ev_\x95a\x0Eqa\x0Eia\x0Eca\rDV[\x9Aa\rTV[_\x8B\x01a\rpV[a\rTV[` \x88\x01a\rpV[a\rTV[`@\x85\x01a\rpV[``\x83\x01a\r~V[\x90V[_\x90V[a\x0E\xA5a\x0E\x99V[Pa\x0E\xB0`\x04a\x0B\x99V[\x90V[``\x90V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1B\x90V[a\x0E\xDBa\x0E\xD6a\x0E\xE0\x92a\rQV[a\x0E\xC1V[a\x0E\xB8V[\x90V[\x90V[a\x0E\xF2a\x0E\xF7\x91a\x0E\xB8V[a\x0E\xE3V[\x90RV[\x90P\x90V[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x0F\x1B\x81a\x0F\"\x93a\x0E\xFBV[\x80\x93a\x0F\0V[\x01\x90V[\x80a\x0F7`\x01\x92a\x0F>\x96\x94a\x0E\xE6V[\x01\x91a\x0F\x0BV[\x90V[a\x0F\x7F\x90a\x0FMa\x0E\xB3V[Pa\x0Fpa\x0FZ_a\x0E\xC7V[\x91\x93a\x0Fda\x01\xA2V[\x94\x85\x93` \x85\x01a\x0F&V[` \x82\x01\x81\x03\x82R\x03\x82a\x0C\xA8V[\x90V[\x90a\x0F\x9Ea\x0F\x9832\x90\x85\x85\x91\x92\x90\x91\x92a\x13\xC3V[\x15a\x01\xCFV[a\x0F\xADWa\x0F\xAB\x91a\x10FV[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x0F\xC5`\x04\x82\x01a\x04dV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x0F\xECa\x0F\xF2\x91\x93\x92\x93a\x01\xBFV[\x92a\x01\xBFV[\x82\x03\x91\x82\x11a\x0F\xFDWV[a\x0F\xC9V[\x90V[a\x10\x19a\x10\x14a\x10\x1E\x92a\x10\x02V[a\x05\xEEV[a\x01\xBFV[\x90V[a\x100a\x106\x91\x93\x92\x93a\x01\xBFV[\x92a\x01\xBFV[\x82\x01\x80\x92\x11a\x10AWV[a\x0F\xC9V[a\x10ba\x10s\x91a\x10[a\x10x\x94Z\x92a\x10\xC3V[Z\x90a\x0F\xDDV[a\x10ma\x13\x88a\x10\x05V[\x90a\x10!V[a\x1A\x8FV[V[a\x10\x83\x90a\x06\rV[\x90V[\x91\x90a\x10\xA0\x81a\x10\x99\x81a\x10\xA5\x95a\x03SV[\x80\x95a\x0F\0V[a\x03gV[\x01\x90V[\x90\x91a\x10\xC0\x92` \x83\x01\x92_\x81\x85\x03\x91\x01Ra\x10\x86V[\x90V[3\x90\x91a\x10\xF0\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10zV[\x92a\x11\x05a\x10\xFCa\x01\xA2V[\x92\x83\x92\x83a\x10\xA9V[\x03\x90\xA2V[\x90a\x11\x14\x91a\x0F\x82V[V[\x90a\x11(\x91a\x11#a\x1B?V[a\x12.V[V[`\xA0\x1C\x90V[a\x11<a\x11A\x91a\x11*V[a\x03\xF4V[\x90V[a\x11N\x90Ta\x110V[\x90V[a\x11ea\x11`a\x11j\x92a\rQV[a\x05\xEEV[a\x04\x9DV[\x90V[a\x11v\x90a\x11QV[\x90V[`\xA0\x1B\x90V[\x90a\x11\x8E`\xFF`\xA0\x1B\x91a\x11yV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x11\xA1\x90a\x01\xCFV[\x90V[\x90V[\x90a\x11\xBCa\x11\xB7a\x11\xC3\x92a\x11\x98V[a\x11\xA4V[\x82Ta\x11\x7FV[\x90UV[a\x11\xD0\x90a\x05\xF1V[\x90V[a\x11\xDC\x90a\x11\xC7V[\x90V[_\x1B\x90V[\x90a\x11\xF5`\x01\x80`\xA0\x1B\x03\x91a\x11\xDFV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x12\x08\x90a\x11\xC7V[\x90V[\x90V[\x90a\x12#a\x12\x1Ea\x12*\x92a\x11\xFFV[a\x12\x0BV[\x82Ta\x11\xE4V[\x90UV[a\x128`\x01a\x11DV[a\x12\xA0W\x81a\x12Wa\x12Qa\x12L_a\x11mV[a\x04\xA8V[\x91a\x04\xA8V[\x14a\x12\x84Wa\x12}a\x12va\x12\x82\x93a\x12q`\x01\x80a\x11\xA7V[a\x11\xD3V[`\x01a\x12\x0EV[a\x1A(V[V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x12\x9C`\x04\x82\x01a\x04dV[\x03\x90\xFD[_b\xDC\x14\x9F`\xE4\x1B\x81R\x80a\x12\xB7`\x04\x82\x01a\x04dV[\x03\x90\xFD[\x90a\x12\xC5\x91a\x11\x16V[V[a\x12\xDEa\x12\xE3\x91a\x12\xD6a\r9V[P`\x05a\x0BIV[a\r\xF5V[\x90V[a\x12\xEEa\x1B?V[a\x12\xF6a\x12\xF8V[V[a\x13\ta\x13\x04_a\x11mV[a\x1B\xB0V[V[a\x13\x13a\x12\xE6V[V[a\x13!a\x13&\x91a\x0B_V[a\x05\xAEV[\x90V[a\x133\x90Ta\x13\x15V[\x90V[`\xE0\x1B\x90V[a\x13E\x81a\x01\xCFV[\x03a\x13LWV[_\x80\xFD[\x90PQ\x90a\x13]\x82a\x13<V[V[\x90` \x82\x82\x03\x12a\x13xWa\x13u\x91_\x01a\x13PV[\x90V[a\x01\xACV[a\x13\xA3a\x13\xB0\x95\x93\x94\x92\x94a\x13\x99``\x84\x01\x96_\x85\x01\x90a\t\x19V[` \x83\x01\x90a\t\x19V[`@\x81\x85\x03\x91\x01Ra\x10\x86V[\x90V[a\x13\xBBa\x01\xA2V[=_\x82>=\x90\xFD[\x92a\x14\x06` \x93\x94a\x13\xD3a\x0E\x99V[Pa\x14\x11a\x13\xE9a\x13\xE4`\x01a\x13)V[a\x06\x19V[\x93cz9y\xDC\x92\x95\x97a\x13\xFAa\x01\xA2V[\x98\x89\x97\x88\x96\x87\x96a\x136V[\x86R`\x04\x86\x01a\x13}V[\x03\x91Z\xFA\x90\x81\x15a\x14UW_\x91a\x14'W[P\x90V[a\x14H\x91P` =\x81\x11a\x14NW[a\x14@\x81\x83a\x0C\xA8V[\x81\x01\x90a\x13_V[_a\x14#V[P=a\x146V[a\x13\xB3V[\x90a\x14va\x14p32\x90\x85\x85\x91\x92\x90\x91\x92a\x13\xC3V[\x15a\x01\xCFV[a\x14\x85Wa\x14\x83\x91a\x14\xA1V[V[_c\x1B\x8E\x82\x8B`\xE3\x1B\x81R\x80a\x14\x9D`\x04\x82\x01a\x04dV[\x03\x90\xFD[a\x14\xBDa\x14\xCE\x91a\x14\xB6a\x14\xD3\x94Z\x92a\x14\xD5V[Z\x90a\x0F\xDDV[a\x14\xC8a\x13\x88a\x10\x05V[\x90a\x10!V[a\x1A\x8FV[V[\x90a\x14\xE1\x903\x92a\x0FAV[\x90a\x15!a\x15\x0F\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10zV[\x92a\x15\x18a\x01\xA2V[\x91\x82\x91\x82a\x03\xA2V[\x03\x90\xA2V[\x90a\x150\x91a\x14ZV[V[_\x90V[a\x15@\x90Qa\x01\xBFV[\x90V[a\x15Ka\x152V[Pa\x15_a\x15Y`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x15\xCFWa\x15\x9Ba\x15\x8D_a\x15\x87a\x15\x82`\x05a\x15|`\x03a\x0BxV[\x90a\x0BIV[a\r\xF5V[\x01a\x156V[a\x15\x95a\x07*V[\x90a\x10!V[Ba\x15\xAEa\x15\xA8\x83a\x01\xBFV[\x91a\x01\xBFV[\x10\x15a\x15\xC2Wa\x15\xBF\x90B\x90a\x0F\xDDV[\x90V[Pa\x15\xCC_a\rTV[\x90V[a\x15\xD8_a\rTV[\x90V[a\x15\xEAa\x15\xF0\x91\x93\x92\x93a\x01\xBFV[\x92a\x01\xBFV[\x91a\x15\xFC\x83\x82\x02a\x01\xBFV[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x16\x0BWV[a\x0F\xC9V[a\x16\x18a\x152V[Pa\x16,a\x16&`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x16fWa\x16ca\x16S`\x02a\x16M`\x05a\x16G`\x03a\x0BxV[\x90a\x0BIV[\x01a\x0BxV[a\x16]`\x02a\x0BxV[\x90a\x15\xDBV[\x90V[a\x16o_a\rTV[\x90V[_\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x16\x8Da\x16\x92\x91a\x0B_V[a\x16vV[\x90V[a\x16\x9F\x90Ta\x16\x81V[\x90V[a\x16\xAAa\x16rV[Pa\x16\xB4_a\x16\x95V[\x90V[\x90V[a\x16\xCEa\x16\xC9a\x16\xD3\x92a\x16\xB7V[a\x05\xEEV[a\x01\xBFV[\x90V[a\x16\xDEa\x152V[Pa\x16\xF2a\x16\xEC`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x17\x16Wa\x17\x13a\x17\x03`\x03a\x0BxV[a\x17\r`\x01a\x16\xBAV[\x90a\x10!V[\x90V[a\x17\x1F_a\rTV[\x90V[a\x17*a\x152V[Pa\x17>a\x178`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x17eWa\x17b`\x02a\x17\\`\x05a\x17V`\x03a\x0BxV[\x90a\x0BIV[\x01a\x0BxV[\x90V[a\x17n_a\rTV[\x90V[a\x17\x8Da\x17\x9E\x91a\x17\x86a\x17\xA3\x94Z\x92a\x18?V[Z\x90a\x0F\xDDV[a\x17\x98a\x13\x88a\x10\x05V[\x90a\x10!V[a\x1A\x8FV[V[P\x90V[`\x01a\x17\xB5\x91\x01a\x01\xBFV[\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[\x905\x90`\x01` \x03\x816\x03\x03\x82\x12\x15a\x18\x1AW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x18\x15W` \x01\x91`\x01\x82\x026\x03\x83\x13a\x18\x10WV[a\x17\xD4V[a\x17\xD0V[a\x17\xCCV[\x90\x82\x10\x15a\x18:W` a\x186\x92\x02\x81\x01\x90a\x17\xD8V[\x90\x91V[a\x17\xB8V[a\x18J\x81\x83\x90a\x17\xA5V[\x91a\x18Sa\x152V[Pa\x18]_a\rTV[[\x80a\x18qa\x18k\x86a\x01\xBFV[\x91a\x01\xBFV[\x10\x15a\x19\x02Wa\x18\x9F\x90a\x18\x9532\x90a\x18\x8D\x87\x87\x86\x91a\x18\x1FV[\x92\x90\x91a\x13\xC3V[a\x18\xA4W[a\x17\xA9V[a\x18^V[3a\x18\xBAa\x18\xB4\x86\x86\x85\x91a\x18\x1FV[\x90a\x0FAV[\x90a\x18\xFAa\x18\xE8\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F\x92a\x10zV[\x92a\x18\xF1a\x01\xA2V[\x91\x82\x91\x82a\x03\xA2V[\x03\x90\xA2a\x18\x9AV[PPPPV[\x90a\x19\x12\x91a\x17qV[V[a\x19%\x90a\x19 a\x1B?V[a\x19'V[V[\x80a\x19Ba\x19<a\x197_a\x11mV[a\x04\xA8V[\x91a\x04\xA8V[\x14a\x19\x9CWa\x19Za\x19S\x82a\x11\xD3V[`\x01a\x12\x0EV[a\x19\x84\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x91a\x10zV[\x90a\x19\x8Da\x01\xA2V[\x80a\x19\x97\x81a\x04dV[\x03\x90\xA2V[_c.\x7F<\x7F`\xE1\x1B\x81R\x80a\x19\xB4`\x04\x82\x01a\x04dV[\x03\x90\xFD[a\x19\xC1\x90a\x19\x14V[V[a\x19\xD4\x90a\x19\xCFa\x1B?V[a\x19\xD6V[V[\x80a\x19\xF1a\x19\xEBa\x19\xE6_a\x11mV[a\x04\xA8V[\x91a\x04\xA8V[\x14a\x1A\x01Wa\x19\xFF\x90a\x1B\xB0V[V[a\x1A$a\x1A\r_a\x11mV[_\x91\x82\x91c\x1EO\xBD\xF7`\xE0\x1B\x83R`\x04\x83\x01a\t&V[\x03\x90\xFD[a\x1A1\x90a\x19\xC3V[V[\x90a\x1A?_\x19\x91a\x11\xDFV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x1Aaa\x1A\\a\x1Ah\x92a\x0B-V[a\x1AIV[\x82Ta\x1A3V[\x90UV[\x91` a\x1A\x8D\x92\x94\x93a\x1A\x86`@\x82\x01\x96_\x83\x01\x90a\x06\xB4V[\x01\x90a\x06\xB4V[V[a\x1A\xA2a\x1A\x9C`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x1B2W[a\x1A\xAFa\x1D\xDCV[a\x1A\xE3\x81a\x1A\xDD`\x02a\x1A\xCD`\x05a\x1A\xC7`\x03a\x0BxV[\x90a\x0BIV[\x01\x91a\x1A\xD8\x83a\x0BxV[a\x10!V[\x90a\x1ALV[a\x1A\xED`\x03a\x0BxV[:a\x1B\x18\x7F-\x9CG\xADU;c\xBB\xBA\xD1\x81\x9DO\xD9}\xA0\x88P\\\x96\xA5\x81\x82i\x1B\x8A\xBB_+\xCC)\xEE\x92a\x0B-V[\x92a\x1B-a\x1B$a\x01\xA2V[\x92\x83\x92\x83a\x1AlV[\x03\x90\xA2V[a\x1B:a\x1C\xD9V[a\x1A\xA7V[a\x1BGa\x16\xA2V[a\x1B`a\x1BZa\x1BUa\x1F\xBAV[a\x04\xA8V[\x91a\x04\xA8V[\x03a\x1BgWV[a\x1B\x89a\x1Bra\x1F\xBAV[_\x91\x82\x91c\x11\x8C\xDA\xA7`\xE0\x1B\x83R`\x04\x83\x01a\t&V[\x03\x90\xFD[\x90V[\x90a\x1B\xA5a\x1B\xA0a\x1B\xAC\x92a\x10zV[a\x1B\x8DV[\x82Ta\x11\xE4V[\x90UV[a\x1B\xB9_a\x16\x95V[a\x1B\xC3\x82_a\x1B\x90V[\x90a\x1B\xF7a\x1B\xF1\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x93a\x10zV[\x91a\x10zV[\x91a\x1C\0a\x01\xA2V[\x80a\x1C\n\x81a\x04dV[\x03\x90\xA3V[\x90a\x1C\x1B`\xFF\x91a\x11\xDFV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x1C:a\x1C5a\x1CA\x92a\x11\x98V[a\x11\xA4V[\x82Ta\x1C\x0FV[\x90UV[\x90a\x1CO\x90a\rTV[_R` R`@_ \x90V[a\x1Ce\x90Qa\x01\xCFV[\x90V[\x90a\x1C\xC5```\x03a\x1C\xCB\x94a\x1C\x8B_\x82\x01a\x1C\x85_\x88\x01a\x156V[\x90a\x1ALV[a\x1C\xA4`\x01\x82\x01a\x1C\x9E` \x88\x01a\x156V[\x90a\x1ALV[a\x1C\xBD`\x02\x82\x01a\x1C\xB7`@\x88\x01a\x156V[\x90a\x1ALV[\x01\x92\x01a\x1C[V[\x90a\x1C%V[V[\x90a\x1C\xD7\x91a\x1ChV[V[a\x1C\xECa\x1C\xE6`\x04a\x0B\x99V[\x15a\x01\xCFV[a\x1C\xF3W[V[a\x1C\xFF`\x01`\x04a\x1C%V[a\x1D\x12a\x1D\x0B_a\rTV[`\x03a\x1ALV[a\x1DsBa\x1Db_a\x1DYa\x1DP_a\x1DKa\x1DB_\x95a\x1D=a\x1D4a\rDV[\x99_\x8B\x01a\rpV[a\rTV[` \x88\x01a\rpV[a\rTV[`@\x85\x01a\rpV[``\x83\x01a\r~V[a\x1Dn`\x05_\x90a\x1CEV[a\x1C\xCDV[_B\x90a\x1D\xB5a\x1D\xA3\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\rTV[\x92a\x1D\xACa\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xA2a\x1C\xF1V[\x90V[a\x1D\xC9\x90a\x01\xBFV[_\x19\x81\x14a\x1D\xD7W`\x01\x01\x90V[a\x0F\xC9V[a\x1D\xF9a\x1D\xF4`\x05a\x1D\xEE`\x03a\x0BxV[\x90a\x0BIV[a\x1D\xBDV[Ba\x1E'a\x1E!a\x1E\x1Ca\x1E\x0E_\x86\x01a\x0BxV[a\x1E\x16a\x07*V[\x90a\x10!V[a\x01\xBFV[\x91a\x01\xBFV[\x10\x15a\x1E1W[PV[a\x1EYa\x1EPa\x1EB_\x84\x01a\x0BxV[a\x1EJa\x07*V[\x90a\x10!V[`\x01\x83\x01a\x1ALV[a\x1Eg`\x01`\x03\x83\x01a\x1C%V[a\x1Eq`\x03a\x0BxV[a\x1E\x9Ea\x1E\x80`\x02\x84\x01a\x0BxV[\x92a\x1E\x98_a\x1E\x91`\x01\x84\x01a\x0BxV[\x92\x01a\x0BxV[\x90a\x0F\xDDV[a\x1E\xC8\x7FH\xA2\x98\xF9\xD3v\xB8*qt\xA7\x98\xE9\x0C\xF1 \x94\x95\xFD\xD6\x8B\x0C\x11\xEB\x11\xBE\xAB\xAC\xC2\xD2\x9C\xF5\x92a\x0B-V[\x92a\x1E\xDDa\x1E\xD4a\x01\xA2V[\x92\x83\x92\x83a\x1AlV[\x03\x90\xA2a\x1E\xFCa\x1E\xF5a\x1E\xF0`\x03a\x0BxV[a\x1D\xC0V[`\x03a\x1ALV[a\x1FfBa\x1FL_a\x1FCa\x1F:_a\x1F5a\x1F,_\x95a\x1F'a\x1F\x1Ea\rDV[\x99_\x8B\x01a\rpV[a\rTV[` \x88\x01a\rpV[a\rTV[`@\x85\x01a\rpV[``\x83\x01a\r~V[a\x1Fa`\x05a\x1F[`\x03a\x0BxV[\x90a\x0BIV[a\x1C\xCDV[a\x1Fp`\x03a\x0BxV[B\x90a\x1F\xB1a\x1F\x9F\x7FA\xF1\xE0\x8F!\xCC\x81\x8C\xF0\xCF\xFB:b`\x9F\xB6\xA3\xCB\xC9\xB3g\x1B\x01\x1E(^\x17\xA1\xEB\xB4h\x8E\x92a\x0B-V[\x92a\x1F\xA8a\x01\xA2V[\x91\x82\x91\x82a\x06\xC1V[\x03\x90\xA2_a\x1E.V[a\x1F\xC2a\x16rV[P3\x90V",
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
            [242u8, 253u8, 227u8, 139u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SyndicateSequencingChainCalls {
        const NAME: &'static str = "SyndicateSequencingChainCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 24usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::PERIOD_DURATION(_) => {
                    <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::SELECTOR
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
