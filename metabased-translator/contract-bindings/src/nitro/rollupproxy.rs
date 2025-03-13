///Module containing a contract's types and functions.
/**

```solidity
library ISequencerInbox {
    struct MaxTimeVariation { uint256 delayBlocks; uint256 futureBlocks; uint256 delaySeconds; uint256 futureSeconds; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISequencerInbox {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct MaxTimeVariation { uint256 delayBlocks; uint256 futureBlocks; uint256 delaySeconds; uint256 futureSeconds; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxTimeVariation {
        #[allow(missing_docs)]
        pub delayBlocks: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub futureBlocks: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub delaySeconds: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub futureSeconds: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<MaxTimeVariation> for UnderlyingRustTuple<'_> {
            fn from(value: MaxTimeVariation) -> Self {
                (
                    value.delayBlocks,
                    value.futureBlocks,
                    value.delaySeconds,
                    value.futureSeconds,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MaxTimeVariation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    delayBlocks: tuple.0,
                    futureBlocks: tuple.1,
                    delaySeconds: tuple.2,
                    futureSeconds: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for MaxTimeVariation {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for MaxTimeVariation {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.delayBlocks),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.futureBlocks),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.delaySeconds),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.futureSeconds),
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
        impl alloy_sol_types::SolType for MaxTimeVariation {
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
        impl alloy_sol_types::SolStruct for MaxTimeVariation {
            const NAME: &'static str = "MaxTimeVariation";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "MaxTimeVariation(uint256 delayBlocks,uint256 futureBlocks,uint256 delaySeconds,uint256 futureSeconds)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.delayBlocks)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.futureBlocks)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.delaySeconds)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.futureSeconds)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for MaxTimeVariation {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delayBlocks,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.futureBlocks,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delaySeconds,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.futureSeconds,
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
                    &rust.delayBlocks,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.futureBlocks,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delaySeconds,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.futureSeconds,
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
    /**Creates a new wrapper around an on-chain [`ISequencerInbox`](self) contract instance.

See the [wrapper's documentation](`ISequencerInboxInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISequencerInboxInstance<T, P, N> {
        ISequencerInboxInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISequencerInbox`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISequencerInbox`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISequencerInboxInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISequencerInboxInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISequencerInboxInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISequencerInboxInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ISequencerInbox`](self) contract instance.

See the [wrapper's documentation](`ISequencerInboxInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ISequencerInboxInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISequencerInboxInstance<T, P, N> {
            ISequencerInboxInstance {
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
    > ISequencerInboxInstance<T, P, N> {
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
    > ISequencerInboxInstance<T, P, N> {
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
library ISequencerInbox {
    struct MaxTimeVariation {
        uint256 delayBlocks;
        uint256 futureBlocks;
        uint256 delaySeconds;
        uint256 futureSeconds;
    }
}

interface RollupProxy {
    type MachineStatus is uint8;
    struct AssertionState {
        GlobalState globalState;
        MachineStatus machineStatus;
        bytes32 endHistoryRoot;
    }
    struct BufferConfig {
        uint64 threshold;
        uint64 max;
        uint64 replenishRateInBasis;
    }
    struct Config {
        uint64 confirmPeriodBlocks;
        address stakeToken;
        uint256 baseStake;
        bytes32 wasmModuleRoot;
        address owner;
        address loserStakeEscrow;
        uint256 chainId;
        string chainConfig;
        uint256 minimumAssertionPeriod;
        uint64 validatorAfkBlocks;
        uint256[] miniStakeValues;
        ISequencerInbox.MaxTimeVariation sequencerInboxMaxTimeVariation;
        uint256 layerZeroBlockEdgeHeight;
        uint256 layerZeroBigStepEdgeHeight;
        uint256 layerZeroSmallStepEdgeHeight;
        AssertionState genesisAssertionState;
        uint256 genesisInboxCount;
        address anyTrustFastConfirmer;
        uint8 numBigStepLevel;
        uint64 challengeGracePeriodBlocks;
        BufferConfig bufferConfig;
    }
    struct ContractDependencies {
        address bridge;
        address sequencerInbox;
        address inbox;
        address outbox;
        address rollupEventInbox;
        address challengeManager;
        address rollupAdminLogic;
        address rollupUserLogic;
        address validatorWalletCreator;
    }
    struct GlobalState {
        bytes32[2] bytes32Vals;
        uint64[2] u64Vals;
    }

    event AdminChanged(address previousAdmin, address newAdmin);
    event BeaconUpgraded(address indexed beacon);
    event Upgraded(address indexed implementation);
    event UpgradedSecondary(address indexed implementation);

    fallback() external payable;

    receive() external payable;

    function initializeProxy(Config memory config, ContractDependencies memory connectedContracts) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "fallback",
    "stateMutability": "payable"
  },
  {
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "initializeProxy",
    "inputs": [
      {
        "name": "config",
        "type": "tuple",
        "internalType": "struct Config",
        "components": [
          {
            "name": "confirmPeriodBlocks",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "stakeToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "baseStake",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "wasmModuleRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "owner",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "loserStakeEscrow",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "chainConfig",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "minimumAssertionPeriod",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "validatorAfkBlocks",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "miniStakeValues",
            "type": "uint256[]",
            "internalType": "uint256[]"
          },
          {
            "name": "sequencerInboxMaxTimeVariation",
            "type": "tuple",
            "internalType": "struct ISequencerInbox.MaxTimeVariation",
            "components": [
              {
                "name": "delayBlocks",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "futureBlocks",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "delaySeconds",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "futureSeconds",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "layerZeroBlockEdgeHeight",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "layerZeroBigStepEdgeHeight",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "layerZeroSmallStepEdgeHeight",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "genesisAssertionState",
            "type": "tuple",
            "internalType": "struct AssertionState",
            "components": [
              {
                "name": "globalState",
                "type": "tuple",
                "internalType": "struct GlobalState",
                "components": [
                  {
                    "name": "bytes32Vals",
                    "type": "bytes32[2]",
                    "internalType": "bytes32[2]"
                  },
                  {
                    "name": "u64Vals",
                    "type": "uint64[2]",
                    "internalType": "uint64[2]"
                  }
                ]
              },
              {
                "name": "machineStatus",
                "type": "uint8",
                "internalType": "enum MachineStatus"
              },
              {
                "name": "endHistoryRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "genesisInboxCount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "anyTrustFastConfirmer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "numBigStepLevel",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "challengeGracePeriodBlocks",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "bufferConfig",
            "type": "tuple",
            "internalType": "struct BufferConfig",
            "components": [
              {
                "name": "threshold",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "max",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "replenishRateInBasis",
                "type": "uint64",
                "internalType": "uint64"
              }
            ]
          }
        ]
      },
      {
        "name": "connectedContracts",
        "type": "tuple",
        "internalType": "struct ContractDependencies",
        "components": [
          {
            "name": "bridge",
            "type": "address",
            "internalType": "contract IBridge"
          },
          {
            "name": "sequencerInbox",
            "type": "address",
            "internalType": "contract ISequencerInbox"
          },
          {
            "name": "inbox",
            "type": "address",
            "internalType": "contract IInboxBase"
          },
          {
            "name": "outbox",
            "type": "address",
            "internalType": "contract IOutbox"
          },
          {
            "name": "rollupEventInbox",
            "type": "address",
            "internalType": "contract IRollupEventInbox"
          },
          {
            "name": "challengeManager",
            "type": "address",
            "internalType": "contract IEdgeChallengeManager"
          },
          {
            "name": "rollupAdminLogic",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "rollupUserLogic",
            "type": "address",
            "internalType": "contract IRollupUser"
          },
          {
            "name": "validatorWalletCreator",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "AdminChanged",
    "inputs": [
      {
        "name": "previousAdmin",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newAdmin",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BeaconUpgraded",
    "inputs": [
      {
        "name": "beacon",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Upgraded",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpgradedSecondary",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
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
pub mod RollupProxy {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b506113cf8061001c5f395ff3fe608060405260043610610021575f3560e01c8063adfef6ac1461003857610030565b366100305761002e610057565b005b61002e610057565b348015610043575f5ffd5b5061002e610052366004610d67565b610069565b6100676100626101b8565b61029a565b565b5f6100726102bd565b6001600160a01b031614801561009757505f61008c6102ef565b6001600160a01b0316145b80156100b257505f6100a7610316565b6001600160a01b0316145b156101b0576101ac8160c0015183836040516024016100d2929190611127565b60408051601f19818403018152918152602080830180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f0ee5ef0c0000000000000000000000000000000000000000000000000000000017905260e08601519087015191516001600160a01b0390921660248301529060440160408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fc4d66de800000000000000000000000000000000000000000000000000000000179052608087015161033d565b5050565b6101ac610057565b5f600436101561020f5760405162461bcd60e51b815260206004820152600b60248201527f4e4f5f46554e435f53494700000000000000000000000000000000000000000060448201526064015b60405180910390fd5b5f336102196102bd565b6001600160a01b0316036102345761022f6102ef565b61023c565b61023c610316565b90506001600160a01b0381163b6102955760405162461bcd60e51b815260206004820152601360248201527f5441524745545f4e4f545f434f4e5452414354000000000000000000000000006044820152606401610206565b919050565b365f5f375f5f365f845af43d5f5f3e8080156102b4573d5ff35b3d5ffd5b505050565b5f7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035b546001600160a01b0316919050565b5f7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc6102e0565b5f7f2b1dbce74324248c222f0ec2d5ed7bd323cfc425b336f0253c5ccfda7265546d6102e0565b61036860017fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6104611317565b7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61031461039657610396611336565b6103c160017f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbd611317565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc146103ef576103ef611336565b61041a60017f2b1dbce74324248c222f0ec2d5ed7bd323cfc425b336f0253c5ccfda7265546e611317565b7f2b1dbce74324248c222f0ec2d5ed7bd323cfc425b336f0253c5ccfda7265546d1461044857610448611336565b6104518161046e565b61045c85855f6104c5565b61046783835f6104ef565b5050505050565b7f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f6104976102bd565b604080516001600160a01b03928316815291841660208301520160405180910390a16104c2816104f8565b50565b6104ce836105d0565b5f825111806104da5750805b156102b8576104e9838361060f565b50505050565b6104ce8361063d565b6001600160a01b0381166105745760405162461bcd60e51b815260206004820152602660248201527f455243313936373a206e65772061646d696e20697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152608401610206565b807fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035b80547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b039290921691909117905550565b6105d98161067c565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a250565b6060610634838360405180606001604052806027815260200161137360279139610720565b90505b92915050565b61064681610812565b6040516001600160a01b038216907ff7eed2a7fabbf1bec8d55ed5e785cc76622376dde5df4ff15470551e030b8134905f90a250565b6001600160a01b0381163b6106f95760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201527f6f74206120636f6e7472616374000000000000000000000000000000000000006064820152608401610206565b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc610597565b60606001600160a01b0384163b61079f5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f60448201527f6e747261637400000000000000000000000000000000000000000000000000006064820152608401610206565b5f5f856001600160a01b0316856040516107b9919061134a565b5f60405180830381855af49150503d805f81146107f1576040519150601f19603f3d011682016040523d82523d5f602084013e6107f6565b606091505b50915091506108068282866108b6565b925050505b9392505050565b6001600160a01b0381163b61088f5760405162461bcd60e51b815260206004820152603760248201527f455243313936373a206e6577207365636f6e6461727920696d706c656d656e7460448201527f6174696f6e206973206e6f74206120636f6e74726163740000000000000000006064820152608401610206565b807f2b1dbce74324248c222f0ec2d5ed7bd323cfc425b336f0253c5ccfda7265546d610597565b606083156108c557508161080b565b8251156108d55782518084602001fd5b8160405162461bcd60e51b81526004016102069190611360565b634e487b7160e01b5f52604160045260245ffd5b6040516060810167ffffffffffffffff81118282101715610926576109266108ef565b60405290565b6040805190810167ffffffffffffffff81118282101715610926576109266108ef565b604051610120810167ffffffffffffffff81118282101715610926576109266108ef565b6040516102a0810167ffffffffffffffff81118282101715610926576109266108ef565b604051601f8201601f1916810167ffffffffffffffff811182821017156109c0576109c06108ef565b604052919050565b803567ffffffffffffffff81168114610295575f5ffd5b80356001600160a01b0381168114610295575f5ffd5b5f82601f830112610a04575f5ffd5b813567ffffffffffffffff811115610a1e57610a1e6108ef565b610a316020601f19601f84011601610997565b818152846020838601011115610a45575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f82601f830112610a70575f5ffd5b813567ffffffffffffffff811115610a8a57610a8a6108ef565b8060051b610a9a60208201610997565b91825260208185018101929081019086841115610ab5575f5ffd5b6020860192505b83831015610ad7578235825260209283019290910190610abc565b9695505050505050565b5f60808284031215610af1575f5ffd5b6040516080810167ffffffffffffffff81118282101715610b1457610b146108ef565b6040908152833582526020808501359083015283810135908201526060928301359281019290925250919050565b5f82601f830112610b51575f5ffd5b610b5b6040610997565b806040840185811115610b6c575f5ffd5b845b81811015610b8d57610b7f816109c8565b845260209384019301610b6e565b509095945050505050565b803560038110610295575f5ffd5b5f81830360c0811215610bb7575f5ffd5b610bbf610903565b91506080811215610bce575f5ffd5b50610bd761092c565b83601f840112610be5575f5ffd5b610bef6040610997565b806040850186811115610c00575f5ffd5b855b81811015610c1a578035845260209384019301610c02565b50818452610c288782610b42565b60208501525050508152610c3e60808301610b98565b602082015260a091909101356040820152919050565b803560ff81168114610295575f5ffd5b5f60608284031215610c74575f5ffd5b610c7c610903565b9050610c87826109c8565b8152610c95602083016109c8565b6020820152610ca6604083016109c8565b604082015292915050565b5f6101208284031215610cc2575f5ffd5b610cca61094f565b9050610cd5826109df565b8152610ce3602083016109df565b6020820152610cf4604083016109df565b6040820152610d05606083016109df565b6060820152610d16608083016109df565b6080820152610d2760a083016109df565b60a0820152610d3860c083016109df565b60c0820152610d4960e083016109df565b60e0820152610d5b61010083016109df565b61010082015292915050565b5f5f6101408385031215610d79575f5ffd5b823567ffffffffffffffff811115610d8f575f5ffd5b83016103e08186031215610da1575f5ffd5b610da9610973565b610db2826109c8565b8152610dc0602083016109df565b60208201526040828101359082015260608083013590820152610de5608083016109df565b6080820152610df660a083016109df565b60a082015260c0828101359082015260e082013567ffffffffffffffff811115610e1e575f5ffd5b610e2a878285016109f5565b60e0830152506101008281013590820152610e4861012083016109c8565b61012082015261014082013567ffffffffffffffff811115610e68575f5ffd5b610e7487828501610a61565b61014083015250610e89866101608401610ae1565b6101608201526101e08201356101808201526102008201356101a08201526102208201356101c0820152610ec1866102408401610ba6565b6101e0820152610300820135610200820152610ee061032083016109df565b610220820152610ef36103408301610c54565b610240820152610f0661036083016109c8565b610260820152610f1a866103808401610c64565b6102808201529250610f3190508460208501610cb1565b90509250929050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f8151808452602084019350602083015f5b82811015610f98578151865260209586019590910190600101610f7a565b5093949350505050565b60038110610fbe57634e487b7160e01b5f52602160045260245ffd5b9052565b80518051835f5b6002811015610fe8578251825260209283019290910190600101610fc9565b50505060200151604083015f5b600281101561101e57825167ffffffffffffffff16825260209283019290910190600101610ff5565b50505060208101516110336080840182610fa2565b506040015160a09190910152565b6001600160a01b038151168252602081015161106860208401826001600160a01b03169052565b50604081015161108360408401826001600160a01b03169052565b50606081015161109e60608401826001600160a01b03169052565b5060808101516110b960808401826001600160a01b03169052565b5060a08101516110d460a08401826001600160a01b03169052565b5060c08101516110ef60c08401826001600160a01b03169052565b5060e081015161110a60e08401826001600160a01b03169052565b506101008101516102b86101008401826001600160a01b03169052565b61014081526111446101408201845167ffffffffffffffff169052565b5f60208401516111606101608401826001600160a01b03169052565b50604084015161018083015260608401516101a083015260808401516001600160a01b039081166101c084015260a0850151166101e083015260c084015161020083015260e08401516103e06102208401526111c0610520840182610f3a565b90506101008501516102408401526101208501516111eb61026085018267ffffffffffffffff169052565b506101408501517ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffec0848303016102808501526112278282610f68565b61016087015180516102a087015260208101516102c087015260408101516102e0870152606001516103008601526101808701516103208601526101a08701516103408601526101c08701516103608601526101e08701519092509050611292610380850182610fc2565b506102008501516104408401526102208501516001600160a01b031661046084015261024085015160ff1661048084015261026085015167ffffffffffffffff9081166104a0850152610280860151805182166104c086015260208082015183166104e087015260409091015190911661050085015290915061080b90830184611041565b8181038181111561063757634e487b7160e01b5f52601160045260245ffd5b634e487b7160e01b5f52600160045260245ffd5b5f82518060208501845e5f920191825250919050565b602081525f6106346020830184610f3a56fe416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a26469706673582212209bf2c0238c2505716be0e1d04df7889d4edb5024e1ba28c8c320a9287abfd25c64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\x13\xCF\x80a\0\x1C_9_\xF3\xFE`\x80`@R`\x046\x10a\0!W_5`\xE0\x1C\x80c\xAD\xFE\xF6\xAC\x14a\08Wa\x000V[6a\x000Wa\0.a\0WV[\0[a\0.a\0WV[4\x80\x15a\0CW__\xFD[Pa\0.a\0R6`\x04a\rgV[a\0iV[a\0ga\0ba\x01\xB8V[a\x02\x9AV[V[_a\0ra\x02\xBDV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a\0\x97WP_a\0\x8Ca\x02\xEFV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80\x15a\0\xB2WP_a\0\xA7a\x03\x16V[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x01\xB0Wa\x01\xAC\x81`\xC0\x01Q\x83\x83`@Q`$\x01a\0\xD2\x92\x91\x90a\x11'V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x80\x83\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x0E\xE5\xEF\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\xE0\x86\x01Q\x90\x87\x01Q\x91Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R\x90`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xC4\xD6m\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x80\x87\x01Qa\x03=V[PPV[a\x01\xACa\0WV[_`\x046\x10\x15a\x02\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7FNO_FUNC_SIG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_3a\x02\x19a\x02\xBDV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x024Wa\x02/a\x02\xEFV[a\x02<V[a\x02<a\x03\x16V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x02\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FTARGET_NOT_CONTRACT\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x06V[\x91\x90PV[6__7__6_\x84Z\xF4=__>\x80\x80\x15a\x02\xB4W=_\xF3[=_\xFD[PPPV[_\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90PV[_\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x02\xE0V[_\x7F+\x1D\xBC\xE7C$$\x8C\"/\x0E\xC2\xD5\xED{\xD3#\xCF\xC4%\xB36\xF0%<\\\xCF\xDAreTma\x02\xE0V[a\x03h`\x01\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x04a\x13\x17V[\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03\x14a\x03\x96Wa\x03\x96a\x136V[a\x03\xC1`\x01\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBDa\x13\x17V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x14a\x03\xEFWa\x03\xEFa\x136V[a\x04\x1A`\x01\x7F+\x1D\xBC\xE7C$$\x8C\"/\x0E\xC2\xD5\xED{\xD3#\xCF\xC4%\xB36\xF0%<\\\xCF\xDAreTna\x13\x17V[\x7F+\x1D\xBC\xE7C$$\x8C\"/\x0E\xC2\xD5\xED{\xD3#\xCF\xC4%\xB36\xF0%<\\\xCF\xDAreTm\x14a\x04HWa\x04Ha\x136V[a\x04Q\x81a\x04nV[a\x04\\\x85\x85_a\x04\xC5V[a\x04g\x83\x83_a\x04\xEFV[PPPPPV[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fa\x04\x97a\x02\xBDV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x84\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1a\x04\xC2\x81a\x04\xF8V[PV[a\x04\xCE\x83a\x05\xD0V[_\x82Q\x11\x80a\x04\xDAWP\x80[\x15a\x02\xB8Wa\x04\xE9\x83\x83a\x06\x0FV[PPPPV[a\x04\xCE\x83a\x06=V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC1967: new admin is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\x06V[\x80\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a\x05\xD9\x81a\x06|V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2PV[``a\x064\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x13s`'\x919a\x07 V[\x90P[\x92\x91PPV[a\x06F\x81a\x08\x12V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xF7\xEE\xD2\xA7\xFA\xBB\xF1\xBE\xC8\xD5^\xD5\xE7\x85\xCCvb#v\xDD\xE5\xDFO\xF1TpU\x1E\x03\x0B\x814\x90_\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x06\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01R\x7Fot a contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\x06V[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x05\x97V[```\x01`\x01`\xA0\x1B\x03\x84\x16;a\x07\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: delegate call to non-co`D\x82\x01R\x7Fntract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\x06V[__\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x07\xB9\x91\x90a\x13JV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x07\xF1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\xF6V[``\x91P[P\x91P\x91Pa\x08\x06\x82\x82\x86a\x08\xB6V[\x92PPP[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x08\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FERC1967: new secondary implement`D\x82\x01R\x7Fation is not a contract\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\x06V[\x80\x7F+\x1D\xBC\xE7C$$\x8C\"/\x0E\xC2\xD5\xED{\xD3#\xCF\xC4%\xB36\xF0%<\\\xCF\xDAreTma\x05\x97V[``\x83\x15a\x08\xC5WP\x81a\x08\x0BV[\x82Q\x15a\x08\xD5W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x06\x91\x90a\x13`V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t&Wa\t&a\x08\xEFV[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t&Wa\t&a\x08\xEFV[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t&Wa\t&a\x08\xEFV[`@Qa\x02\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t&Wa\t&a\x08\xEFV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t\xC0Wa\t\xC0a\x08\xEFV[`@R\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\x95W__\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x95W__\xFD[_\x82`\x1F\x83\x01\x12a\n\x04W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x1EWa\n\x1Ea\x08\xEFV[a\n1` `\x1F\x19`\x1F\x84\x01\x16\x01a\t\x97V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\nEW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\npW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x8AWa\n\x8Aa\x08\xEFV[\x80`\x05\x1Ba\n\x9A` \x82\x01a\t\x97V[\x91\x82R` \x81\x85\x01\x81\x01\x92\x90\x81\x01\x90\x86\x84\x11\x15a\n\xB5W__\xFD[` \x86\x01\x92P[\x83\x83\x10\x15a\n\xD7W\x825\x82R` \x92\x83\x01\x92\x90\x91\x01\x90a\n\xBCV[\x96\x95PPPPPPV[_`\x80\x82\x84\x03\x12\x15a\n\xF1W__\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x14Wa\x0B\x14a\x08\xEFV[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x83\x81\x015\x90\x82\x01R``\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x0BQW__\xFD[a\x0B[`@a\t\x97V[\x80`@\x84\x01\x85\x81\x11\x15a\x0BlW__\xFD[\x84[\x81\x81\x10\x15a\x0B\x8DWa\x0B\x7F\x81a\t\xC8V[\x84R` \x93\x84\x01\x93\x01a\x0BnV[P\x90\x95\x94PPPPPV[\x805`\x03\x81\x10a\x02\x95W__\xFD[_\x81\x83\x03`\xC0\x81\x12\x15a\x0B\xB7W__\xFD[a\x0B\xBFa\t\x03V[\x91P`\x80\x81\x12\x15a\x0B\xCEW__\xFD[Pa\x0B\xD7a\t,V[\x83`\x1F\x84\x01\x12a\x0B\xE5W__\xFD[a\x0B\xEF`@a\t\x97V[\x80`@\x85\x01\x86\x81\x11\x15a\x0C\0W__\xFD[\x85[\x81\x81\x10\x15a\x0C\x1AW\x805\x84R` \x93\x84\x01\x93\x01a\x0C\x02V[P\x81\x84Ra\x0C(\x87\x82a\x0BBV[` \x85\x01RPPP\x81Ra\x0C>`\x80\x83\x01a\x0B\x98V[` \x82\x01R`\xA0\x91\x90\x91\x015`@\x82\x01R\x91\x90PV[\x805`\xFF\x81\x16\x81\x14a\x02\x95W__\xFD[_``\x82\x84\x03\x12\x15a\x0CtW__\xFD[a\x0C|a\t\x03V[\x90Pa\x0C\x87\x82a\t\xC8V[\x81Ra\x0C\x95` \x83\x01a\t\xC8V[` \x82\x01Ra\x0C\xA6`@\x83\x01a\t\xC8V[`@\x82\x01R\x92\x91PPV[_a\x01 \x82\x84\x03\x12\x15a\x0C\xC2W__\xFD[a\x0C\xCAa\tOV[\x90Pa\x0C\xD5\x82a\t\xDFV[\x81Ra\x0C\xE3` \x83\x01a\t\xDFV[` \x82\x01Ra\x0C\xF4`@\x83\x01a\t\xDFV[`@\x82\x01Ra\r\x05``\x83\x01a\t\xDFV[``\x82\x01Ra\r\x16`\x80\x83\x01a\t\xDFV[`\x80\x82\x01Ra\r'`\xA0\x83\x01a\t\xDFV[`\xA0\x82\x01Ra\r8`\xC0\x83\x01a\t\xDFV[`\xC0\x82\x01Ra\rI`\xE0\x83\x01a\t\xDFV[`\xE0\x82\x01Ra\r[a\x01\0\x83\x01a\t\xDFV[a\x01\0\x82\x01R\x92\x91PPV[__a\x01@\x83\x85\x03\x12\x15a\ryW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x8FW__\xFD[\x83\x01a\x03\xE0\x81\x86\x03\x12\x15a\r\xA1W__\xFD[a\r\xA9a\tsV[a\r\xB2\x82a\t\xC8V[\x81Ra\r\xC0` \x83\x01a\t\xDFV[` \x82\x01R`@\x82\x81\x015\x90\x82\x01R``\x80\x83\x015\x90\x82\x01Ra\r\xE5`\x80\x83\x01a\t\xDFV[`\x80\x82\x01Ra\r\xF6`\xA0\x83\x01a\t\xDFV[`\xA0\x82\x01R`\xC0\x82\x81\x015\x90\x82\x01R`\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x1EW__\xFD[a\x0E*\x87\x82\x85\x01a\t\xF5V[`\xE0\x83\x01RPa\x01\0\x82\x81\x015\x90\x82\x01Ra\x0EHa\x01 \x83\x01a\t\xC8V[a\x01 \x82\x01Ra\x01@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EhW__\xFD[a\x0Et\x87\x82\x85\x01a\naV[a\x01@\x83\x01RPa\x0E\x89\x86a\x01`\x84\x01a\n\xE1V[a\x01`\x82\x01Ra\x01\xE0\x82\x015a\x01\x80\x82\x01Ra\x02\0\x82\x015a\x01\xA0\x82\x01Ra\x02 \x82\x015a\x01\xC0\x82\x01Ra\x0E\xC1\x86a\x02@\x84\x01a\x0B\xA6V[a\x01\xE0\x82\x01Ra\x03\0\x82\x015a\x02\0\x82\x01Ra\x0E\xE0a\x03 \x83\x01a\t\xDFV[a\x02 \x82\x01Ra\x0E\xF3a\x03@\x83\x01a\x0CTV[a\x02@\x82\x01Ra\x0F\x06a\x03`\x83\x01a\t\xC8V[a\x02`\x82\x01Ra\x0F\x1A\x86a\x03\x80\x84\x01a\x0CdV[a\x02\x80\x82\x01R\x92Pa\x0F1\x90P\x84` \x85\x01a\x0C\xB1V[\x90P\x92P\x92\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x0F\x98W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x0FzV[P\x93\x94\x93PPPPV[`\x03\x81\x10a\x0F\xBEWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[\x80Q\x80Q\x83_[`\x02\x81\x10\x15a\x0F\xE8W\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x0F\xC9V[PPP` \x01Q`@\x83\x01_[`\x02\x81\x10\x15a\x10\x1EW\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x0F\xF5V[PPP` \x81\x01Qa\x103`\x80\x84\x01\x82a\x0F\xA2V[P`@\x01Q`\xA0\x91\x90\x91\x01RV[`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Qa\x10h` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x81\x01Qa\x10\x83`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x81\x01Qa\x10\x9E``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x81\x01Qa\x10\xB9`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x81\x01Qa\x10\xD4`\xA0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xC0\x81\x01Qa\x10\xEF`\xC0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x81\x01Qa\x11\n`\xE0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x81\x01Qa\x02\xB8a\x01\0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[a\x01@\x81Ra\x11Da\x01@\x82\x01\x84Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[_` \x84\x01Qa\x11`a\x01`\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x84\x01Qa\x01\x80\x83\x01R``\x84\x01Qa\x01\xA0\x83\x01R`\x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xC0\x84\x01R`\xA0\x85\x01Q\x16a\x01\xE0\x83\x01R`\xC0\x84\x01Qa\x02\0\x83\x01R`\xE0\x84\x01Qa\x03\xE0a\x02 \x84\x01Ra\x11\xC0a\x05 \x84\x01\x82a\x0F:V[\x90Pa\x01\0\x85\x01Qa\x02@\x84\x01Ra\x01 \x85\x01Qa\x11\xEBa\x02`\x85\x01\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[Pa\x01@\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xC0\x84\x83\x03\x01a\x02\x80\x85\x01Ra\x12'\x82\x82a\x0FhV[a\x01`\x87\x01Q\x80Qa\x02\xA0\x87\x01R` \x81\x01Qa\x02\xC0\x87\x01R`@\x81\x01Qa\x02\xE0\x87\x01R``\x01Qa\x03\0\x86\x01Ra\x01\x80\x87\x01Qa\x03 \x86\x01Ra\x01\xA0\x87\x01Qa\x03@\x86\x01Ra\x01\xC0\x87\x01Qa\x03`\x86\x01Ra\x01\xE0\x87\x01Q\x90\x92P\x90Pa\x12\x92a\x03\x80\x85\x01\x82a\x0F\xC2V[Pa\x02\0\x85\x01Qa\x04@\x84\x01Ra\x02 \x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x04`\x84\x01Ra\x02@\x85\x01Q`\xFF\x16a\x04\x80\x84\x01Ra\x02`\x85\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16a\x04\xA0\x85\x01Ra\x02\x80\x86\x01Q\x80Q\x82\x16a\x04\xC0\x86\x01R` \x80\x82\x01Q\x83\x16a\x04\xE0\x87\x01R`@\x90\x91\x01Q\x90\x91\x16a\x05\0\x85\x01R\x90\x91Pa\x08\x0B\x90\x83\x01\x84a\x10AV[\x81\x81\x03\x81\x81\x11\x15a\x067WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[` \x81R_a\x064` \x83\x01\x84a\x0F:V\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 \x9B\xF2\xC0#\x8C%\x05qk\xE0\xE1\xD0M\xF7\x88\x9DN\xDBP$\xE1\xBA(\xC8\xC3 \xA9(z\xBF\xD2\\dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610021575f3560e01c8063adfef6ac1461003857610030565b366100305761002e610057565b005b61002e610057565b348015610043575f5ffd5b5061002e610052366004610d67565b610069565b6100676100626101b8565b61029a565b565b5f6100726102bd565b6001600160a01b031614801561009757505f61008c6102ef565b6001600160a01b0316145b80156100b257505f6100a7610316565b6001600160a01b0316145b156101b0576101ac8160c0015183836040516024016100d2929190611127565b60408051601f19818403018152918152602080830180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f0ee5ef0c0000000000000000000000000000000000000000000000000000000017905260e08601519087015191516001600160a01b0390921660248301529060440160408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fc4d66de800000000000000000000000000000000000000000000000000000000179052608087015161033d565b5050565b6101ac610057565b5f600436101561020f5760405162461bcd60e51b815260206004820152600b60248201527f4e4f5f46554e435f53494700000000000000000000000000000000000000000060448201526064015b60405180910390fd5b5f336102196102bd565b6001600160a01b0316036102345761022f6102ef565b61023c565b61023c610316565b90506001600160a01b0381163b6102955760405162461bcd60e51b815260206004820152601360248201527f5441524745545f4e4f545f434f4e5452414354000000000000000000000000006044820152606401610206565b919050565b365f5f375f5f365f845af43d5f5f3e8080156102b4573d5ff35b3d5ffd5b505050565b5f7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035b546001600160a01b0316919050565b5f7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc6102e0565b5f7f2b1dbce74324248c222f0ec2d5ed7bd323cfc425b336f0253c5ccfda7265546d6102e0565b61036860017fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6104611317565b7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61031461039657610396611336565b6103c160017f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbd611317565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc146103ef576103ef611336565b61041a60017f2b1dbce74324248c222f0ec2d5ed7bd323cfc425b336f0253c5ccfda7265546e611317565b7f2b1dbce74324248c222f0ec2d5ed7bd323cfc425b336f0253c5ccfda7265546d1461044857610448611336565b6104518161046e565b61045c85855f6104c5565b61046783835f6104ef565b5050505050565b7f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f6104976102bd565b604080516001600160a01b03928316815291841660208301520160405180910390a16104c2816104f8565b50565b6104ce836105d0565b5f825111806104da5750805b156102b8576104e9838361060f565b50505050565b6104ce8361063d565b6001600160a01b0381166105745760405162461bcd60e51b815260206004820152602660248201527f455243313936373a206e65772061646d696e20697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152608401610206565b807fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035b80547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b039290921691909117905550565b6105d98161067c565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a250565b6060610634838360405180606001604052806027815260200161137360279139610720565b90505b92915050565b61064681610812565b6040516001600160a01b038216907ff7eed2a7fabbf1bec8d55ed5e785cc76622376dde5df4ff15470551e030b8134905f90a250565b6001600160a01b0381163b6106f95760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201527f6f74206120636f6e7472616374000000000000000000000000000000000000006064820152608401610206565b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc610597565b60606001600160a01b0384163b61079f5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f60448201527f6e747261637400000000000000000000000000000000000000000000000000006064820152608401610206565b5f5f856001600160a01b0316856040516107b9919061134a565b5f60405180830381855af49150503d805f81146107f1576040519150601f19603f3d011682016040523d82523d5f602084013e6107f6565b606091505b50915091506108068282866108b6565b925050505b9392505050565b6001600160a01b0381163b61088f5760405162461bcd60e51b815260206004820152603760248201527f455243313936373a206e6577207365636f6e6461727920696d706c656d656e7460448201527f6174696f6e206973206e6f74206120636f6e74726163740000000000000000006064820152608401610206565b807f2b1dbce74324248c222f0ec2d5ed7bd323cfc425b336f0253c5ccfda7265546d610597565b606083156108c557508161080b565b8251156108d55782518084602001fd5b8160405162461bcd60e51b81526004016102069190611360565b634e487b7160e01b5f52604160045260245ffd5b6040516060810167ffffffffffffffff81118282101715610926576109266108ef565b60405290565b6040805190810167ffffffffffffffff81118282101715610926576109266108ef565b604051610120810167ffffffffffffffff81118282101715610926576109266108ef565b6040516102a0810167ffffffffffffffff81118282101715610926576109266108ef565b604051601f8201601f1916810167ffffffffffffffff811182821017156109c0576109c06108ef565b604052919050565b803567ffffffffffffffff81168114610295575f5ffd5b80356001600160a01b0381168114610295575f5ffd5b5f82601f830112610a04575f5ffd5b813567ffffffffffffffff811115610a1e57610a1e6108ef565b610a316020601f19601f84011601610997565b818152846020838601011115610a45575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f82601f830112610a70575f5ffd5b813567ffffffffffffffff811115610a8a57610a8a6108ef565b8060051b610a9a60208201610997565b91825260208185018101929081019086841115610ab5575f5ffd5b6020860192505b83831015610ad7578235825260209283019290910190610abc565b9695505050505050565b5f60808284031215610af1575f5ffd5b6040516080810167ffffffffffffffff81118282101715610b1457610b146108ef565b6040908152833582526020808501359083015283810135908201526060928301359281019290925250919050565b5f82601f830112610b51575f5ffd5b610b5b6040610997565b806040840185811115610b6c575f5ffd5b845b81811015610b8d57610b7f816109c8565b845260209384019301610b6e565b509095945050505050565b803560038110610295575f5ffd5b5f81830360c0811215610bb7575f5ffd5b610bbf610903565b91506080811215610bce575f5ffd5b50610bd761092c565b83601f840112610be5575f5ffd5b610bef6040610997565b806040850186811115610c00575f5ffd5b855b81811015610c1a578035845260209384019301610c02565b50818452610c288782610b42565b60208501525050508152610c3e60808301610b98565b602082015260a091909101356040820152919050565b803560ff81168114610295575f5ffd5b5f60608284031215610c74575f5ffd5b610c7c610903565b9050610c87826109c8565b8152610c95602083016109c8565b6020820152610ca6604083016109c8565b604082015292915050565b5f6101208284031215610cc2575f5ffd5b610cca61094f565b9050610cd5826109df565b8152610ce3602083016109df565b6020820152610cf4604083016109df565b6040820152610d05606083016109df565b6060820152610d16608083016109df565b6080820152610d2760a083016109df565b60a0820152610d3860c083016109df565b60c0820152610d4960e083016109df565b60e0820152610d5b61010083016109df565b61010082015292915050565b5f5f6101408385031215610d79575f5ffd5b823567ffffffffffffffff811115610d8f575f5ffd5b83016103e08186031215610da1575f5ffd5b610da9610973565b610db2826109c8565b8152610dc0602083016109df565b60208201526040828101359082015260608083013590820152610de5608083016109df565b6080820152610df660a083016109df565b60a082015260c0828101359082015260e082013567ffffffffffffffff811115610e1e575f5ffd5b610e2a878285016109f5565b60e0830152506101008281013590820152610e4861012083016109c8565b61012082015261014082013567ffffffffffffffff811115610e68575f5ffd5b610e7487828501610a61565b61014083015250610e89866101608401610ae1565b6101608201526101e08201356101808201526102008201356101a08201526102208201356101c0820152610ec1866102408401610ba6565b6101e0820152610300820135610200820152610ee061032083016109df565b610220820152610ef36103408301610c54565b610240820152610f0661036083016109c8565b610260820152610f1a866103808401610c64565b6102808201529250610f3190508460208501610cb1565b90509250929050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f8151808452602084019350602083015f5b82811015610f98578151865260209586019590910190600101610f7a565b5093949350505050565b60038110610fbe57634e487b7160e01b5f52602160045260245ffd5b9052565b80518051835f5b6002811015610fe8578251825260209283019290910190600101610fc9565b50505060200151604083015f5b600281101561101e57825167ffffffffffffffff16825260209283019290910190600101610ff5565b50505060208101516110336080840182610fa2565b506040015160a09190910152565b6001600160a01b038151168252602081015161106860208401826001600160a01b03169052565b50604081015161108360408401826001600160a01b03169052565b50606081015161109e60608401826001600160a01b03169052565b5060808101516110b960808401826001600160a01b03169052565b5060a08101516110d460a08401826001600160a01b03169052565b5060c08101516110ef60c08401826001600160a01b03169052565b5060e081015161110a60e08401826001600160a01b03169052565b506101008101516102b86101008401826001600160a01b03169052565b61014081526111446101408201845167ffffffffffffffff169052565b5f60208401516111606101608401826001600160a01b03169052565b50604084015161018083015260608401516101a083015260808401516001600160a01b039081166101c084015260a0850151166101e083015260c084015161020083015260e08401516103e06102208401526111c0610520840182610f3a565b90506101008501516102408401526101208501516111eb61026085018267ffffffffffffffff169052565b506101408501517ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffec0848303016102808501526112278282610f68565b61016087015180516102a087015260208101516102c087015260408101516102e0870152606001516103008601526101808701516103208601526101a08701516103408601526101c08701516103608601526101e08701519092509050611292610380850182610fc2565b506102008501516104408401526102208501516001600160a01b031661046084015261024085015160ff1661048084015261026085015167ffffffffffffffff9081166104a0850152610280860151805182166104c086015260208082015183166104e087015260409091015190911661050085015290915061080b90830184611041565b8181038181111561063757634e487b7160e01b5f52601160045260245ffd5b634e487b7160e01b5f52600160045260245ffd5b5f82518060208501845e5f920191825250919050565b602081525f6106346020830184610f3a56fe416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a26469706673582212209bf2c0238c2505716be0e1d04df7889d4edb5024e1ba28c8c320a9287abfd25c64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0!W_5`\xE0\x1C\x80c\xAD\xFE\xF6\xAC\x14a\08Wa\x000V[6a\x000Wa\0.a\0WV[\0[a\0.a\0WV[4\x80\x15a\0CW__\xFD[Pa\0.a\0R6`\x04a\rgV[a\0iV[a\0ga\0ba\x01\xB8V[a\x02\x9AV[V[_a\0ra\x02\xBDV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a\0\x97WP_a\0\x8Ca\x02\xEFV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80\x15a\0\xB2WP_a\0\xA7a\x03\x16V[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x01\xB0Wa\x01\xAC\x81`\xC0\x01Q\x83\x83`@Q`$\x01a\0\xD2\x92\x91\x90a\x11'V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x80\x83\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x0E\xE5\xEF\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\xE0\x86\x01Q\x90\x87\x01Q\x91Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R\x90`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xC4\xD6m\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x80\x87\x01Qa\x03=V[PPV[a\x01\xACa\0WV[_`\x046\x10\x15a\x02\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7FNO_FUNC_SIG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_3a\x02\x19a\x02\xBDV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x024Wa\x02/a\x02\xEFV[a\x02<V[a\x02<a\x03\x16V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x02\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FTARGET_NOT_CONTRACT\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x06V[\x91\x90PV[6__7__6_\x84Z\xF4=__>\x80\x80\x15a\x02\xB4W=_\xF3[=_\xFD[PPPV[_\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90PV[_\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x02\xE0V[_\x7F+\x1D\xBC\xE7C$$\x8C\"/\x0E\xC2\xD5\xED{\xD3#\xCF\xC4%\xB36\xF0%<\\\xCF\xDAreTma\x02\xE0V[a\x03h`\x01\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x04a\x13\x17V[\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03\x14a\x03\x96Wa\x03\x96a\x136V[a\x03\xC1`\x01\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBDa\x13\x17V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x14a\x03\xEFWa\x03\xEFa\x136V[a\x04\x1A`\x01\x7F+\x1D\xBC\xE7C$$\x8C\"/\x0E\xC2\xD5\xED{\xD3#\xCF\xC4%\xB36\xF0%<\\\xCF\xDAreTna\x13\x17V[\x7F+\x1D\xBC\xE7C$$\x8C\"/\x0E\xC2\xD5\xED{\xD3#\xCF\xC4%\xB36\xF0%<\\\xCF\xDAreTm\x14a\x04HWa\x04Ha\x136V[a\x04Q\x81a\x04nV[a\x04\\\x85\x85_a\x04\xC5V[a\x04g\x83\x83_a\x04\xEFV[PPPPPV[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fa\x04\x97a\x02\xBDV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x84\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1a\x04\xC2\x81a\x04\xF8V[PV[a\x04\xCE\x83a\x05\xD0V[_\x82Q\x11\x80a\x04\xDAWP\x80[\x15a\x02\xB8Wa\x04\xE9\x83\x83a\x06\x0FV[PPPPV[a\x04\xCE\x83a\x06=V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC1967: new admin is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\x06V[\x80\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a\x05\xD9\x81a\x06|V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2PV[``a\x064\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x13s`'\x919a\x07 V[\x90P[\x92\x91PPV[a\x06F\x81a\x08\x12V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xF7\xEE\xD2\xA7\xFA\xBB\xF1\xBE\xC8\xD5^\xD5\xE7\x85\xCCvb#v\xDD\xE5\xDFO\xF1TpU\x1E\x03\x0B\x814\x90_\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x06\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01R\x7Fot a contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\x06V[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x05\x97V[```\x01`\x01`\xA0\x1B\x03\x84\x16;a\x07\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: delegate call to non-co`D\x82\x01R\x7Fntract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\x06V[__\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x07\xB9\x91\x90a\x13JV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x07\xF1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\xF6V[``\x91P[P\x91P\x91Pa\x08\x06\x82\x82\x86a\x08\xB6V[\x92PPP[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x08\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FERC1967: new secondary implement`D\x82\x01R\x7Fation is not a contract\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\x06V[\x80\x7F+\x1D\xBC\xE7C$$\x8C\"/\x0E\xC2\xD5\xED{\xD3#\xCF\xC4%\xB36\xF0%<\\\xCF\xDAreTma\x05\x97V[``\x83\x15a\x08\xC5WP\x81a\x08\x0BV[\x82Q\x15a\x08\xD5W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x06\x91\x90a\x13`V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t&Wa\t&a\x08\xEFV[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t&Wa\t&a\x08\xEFV[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t&Wa\t&a\x08\xEFV[`@Qa\x02\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t&Wa\t&a\x08\xEFV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t\xC0Wa\t\xC0a\x08\xEFV[`@R\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\x95W__\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x95W__\xFD[_\x82`\x1F\x83\x01\x12a\n\x04W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x1EWa\n\x1Ea\x08\xEFV[a\n1` `\x1F\x19`\x1F\x84\x01\x16\x01a\t\x97V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\nEW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\npW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x8AWa\n\x8Aa\x08\xEFV[\x80`\x05\x1Ba\n\x9A` \x82\x01a\t\x97V[\x91\x82R` \x81\x85\x01\x81\x01\x92\x90\x81\x01\x90\x86\x84\x11\x15a\n\xB5W__\xFD[` \x86\x01\x92P[\x83\x83\x10\x15a\n\xD7W\x825\x82R` \x92\x83\x01\x92\x90\x91\x01\x90a\n\xBCV[\x96\x95PPPPPPV[_`\x80\x82\x84\x03\x12\x15a\n\xF1W__\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x14Wa\x0B\x14a\x08\xEFV[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x83\x81\x015\x90\x82\x01R``\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x0BQW__\xFD[a\x0B[`@a\t\x97V[\x80`@\x84\x01\x85\x81\x11\x15a\x0BlW__\xFD[\x84[\x81\x81\x10\x15a\x0B\x8DWa\x0B\x7F\x81a\t\xC8V[\x84R` \x93\x84\x01\x93\x01a\x0BnV[P\x90\x95\x94PPPPPV[\x805`\x03\x81\x10a\x02\x95W__\xFD[_\x81\x83\x03`\xC0\x81\x12\x15a\x0B\xB7W__\xFD[a\x0B\xBFa\t\x03V[\x91P`\x80\x81\x12\x15a\x0B\xCEW__\xFD[Pa\x0B\xD7a\t,V[\x83`\x1F\x84\x01\x12a\x0B\xE5W__\xFD[a\x0B\xEF`@a\t\x97V[\x80`@\x85\x01\x86\x81\x11\x15a\x0C\0W__\xFD[\x85[\x81\x81\x10\x15a\x0C\x1AW\x805\x84R` \x93\x84\x01\x93\x01a\x0C\x02V[P\x81\x84Ra\x0C(\x87\x82a\x0BBV[` \x85\x01RPPP\x81Ra\x0C>`\x80\x83\x01a\x0B\x98V[` \x82\x01R`\xA0\x91\x90\x91\x015`@\x82\x01R\x91\x90PV[\x805`\xFF\x81\x16\x81\x14a\x02\x95W__\xFD[_``\x82\x84\x03\x12\x15a\x0CtW__\xFD[a\x0C|a\t\x03V[\x90Pa\x0C\x87\x82a\t\xC8V[\x81Ra\x0C\x95` \x83\x01a\t\xC8V[` \x82\x01Ra\x0C\xA6`@\x83\x01a\t\xC8V[`@\x82\x01R\x92\x91PPV[_a\x01 \x82\x84\x03\x12\x15a\x0C\xC2W__\xFD[a\x0C\xCAa\tOV[\x90Pa\x0C\xD5\x82a\t\xDFV[\x81Ra\x0C\xE3` \x83\x01a\t\xDFV[` \x82\x01Ra\x0C\xF4`@\x83\x01a\t\xDFV[`@\x82\x01Ra\r\x05``\x83\x01a\t\xDFV[``\x82\x01Ra\r\x16`\x80\x83\x01a\t\xDFV[`\x80\x82\x01Ra\r'`\xA0\x83\x01a\t\xDFV[`\xA0\x82\x01Ra\r8`\xC0\x83\x01a\t\xDFV[`\xC0\x82\x01Ra\rI`\xE0\x83\x01a\t\xDFV[`\xE0\x82\x01Ra\r[a\x01\0\x83\x01a\t\xDFV[a\x01\0\x82\x01R\x92\x91PPV[__a\x01@\x83\x85\x03\x12\x15a\ryW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x8FW__\xFD[\x83\x01a\x03\xE0\x81\x86\x03\x12\x15a\r\xA1W__\xFD[a\r\xA9a\tsV[a\r\xB2\x82a\t\xC8V[\x81Ra\r\xC0` \x83\x01a\t\xDFV[` \x82\x01R`@\x82\x81\x015\x90\x82\x01R``\x80\x83\x015\x90\x82\x01Ra\r\xE5`\x80\x83\x01a\t\xDFV[`\x80\x82\x01Ra\r\xF6`\xA0\x83\x01a\t\xDFV[`\xA0\x82\x01R`\xC0\x82\x81\x015\x90\x82\x01R`\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x1EW__\xFD[a\x0E*\x87\x82\x85\x01a\t\xF5V[`\xE0\x83\x01RPa\x01\0\x82\x81\x015\x90\x82\x01Ra\x0EHa\x01 \x83\x01a\t\xC8V[a\x01 \x82\x01Ra\x01@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EhW__\xFD[a\x0Et\x87\x82\x85\x01a\naV[a\x01@\x83\x01RPa\x0E\x89\x86a\x01`\x84\x01a\n\xE1V[a\x01`\x82\x01Ra\x01\xE0\x82\x015a\x01\x80\x82\x01Ra\x02\0\x82\x015a\x01\xA0\x82\x01Ra\x02 \x82\x015a\x01\xC0\x82\x01Ra\x0E\xC1\x86a\x02@\x84\x01a\x0B\xA6V[a\x01\xE0\x82\x01Ra\x03\0\x82\x015a\x02\0\x82\x01Ra\x0E\xE0a\x03 \x83\x01a\t\xDFV[a\x02 \x82\x01Ra\x0E\xF3a\x03@\x83\x01a\x0CTV[a\x02@\x82\x01Ra\x0F\x06a\x03`\x83\x01a\t\xC8V[a\x02`\x82\x01Ra\x0F\x1A\x86a\x03\x80\x84\x01a\x0CdV[a\x02\x80\x82\x01R\x92Pa\x0F1\x90P\x84` \x85\x01a\x0C\xB1V[\x90P\x92P\x92\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x0F\x98W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x0FzV[P\x93\x94\x93PPPPV[`\x03\x81\x10a\x0F\xBEWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[\x80Q\x80Q\x83_[`\x02\x81\x10\x15a\x0F\xE8W\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x0F\xC9V[PPP` \x01Q`@\x83\x01_[`\x02\x81\x10\x15a\x10\x1EW\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x0F\xF5V[PPP` \x81\x01Qa\x103`\x80\x84\x01\x82a\x0F\xA2V[P`@\x01Q`\xA0\x91\x90\x91\x01RV[`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Qa\x10h` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x81\x01Qa\x10\x83`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x81\x01Qa\x10\x9E``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x81\x01Qa\x10\xB9`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x81\x01Qa\x10\xD4`\xA0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xC0\x81\x01Qa\x10\xEF`\xC0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x81\x01Qa\x11\n`\xE0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x81\x01Qa\x02\xB8a\x01\0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[a\x01@\x81Ra\x11Da\x01@\x82\x01\x84Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[_` \x84\x01Qa\x11`a\x01`\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x84\x01Qa\x01\x80\x83\x01R``\x84\x01Qa\x01\xA0\x83\x01R`\x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xC0\x84\x01R`\xA0\x85\x01Q\x16a\x01\xE0\x83\x01R`\xC0\x84\x01Qa\x02\0\x83\x01R`\xE0\x84\x01Qa\x03\xE0a\x02 \x84\x01Ra\x11\xC0a\x05 \x84\x01\x82a\x0F:V[\x90Pa\x01\0\x85\x01Qa\x02@\x84\x01Ra\x01 \x85\x01Qa\x11\xEBa\x02`\x85\x01\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[Pa\x01@\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xC0\x84\x83\x03\x01a\x02\x80\x85\x01Ra\x12'\x82\x82a\x0FhV[a\x01`\x87\x01Q\x80Qa\x02\xA0\x87\x01R` \x81\x01Qa\x02\xC0\x87\x01R`@\x81\x01Qa\x02\xE0\x87\x01R``\x01Qa\x03\0\x86\x01Ra\x01\x80\x87\x01Qa\x03 \x86\x01Ra\x01\xA0\x87\x01Qa\x03@\x86\x01Ra\x01\xC0\x87\x01Qa\x03`\x86\x01Ra\x01\xE0\x87\x01Q\x90\x92P\x90Pa\x12\x92a\x03\x80\x85\x01\x82a\x0F\xC2V[Pa\x02\0\x85\x01Qa\x04@\x84\x01Ra\x02 \x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x04`\x84\x01Ra\x02@\x85\x01Q`\xFF\x16a\x04\x80\x84\x01Ra\x02`\x85\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16a\x04\xA0\x85\x01Ra\x02\x80\x86\x01Q\x80Q\x82\x16a\x04\xC0\x86\x01R` \x80\x82\x01Q\x83\x16a\x04\xE0\x87\x01R`@\x90\x91\x01Q\x90\x91\x16a\x05\0\x85\x01R\x90\x91Pa\x08\x0B\x90\x83\x01\x84a\x10AV[\x81\x81\x03\x81\x81\x11\x15a\x067WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[` \x81R_a\x064` \x83\x01\x84a\x0F:V\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 \x9B\xF2\xC0#\x8C%\x05qk\xE0\xE1\xD0M\xF7\x88\x9DN\xDBP$\xE1\xBA(\xC8\xC3 \xA9(z\xBF\xD2\\dsolcC\0\x08\x1C\x003",
    );
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MachineStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<MachineStatus> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl MachineStatus {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for MachineStatus {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for MachineStatus {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    /**```solidity
struct AssertionState { GlobalState globalState; MachineStatus machineStatus; bytes32 endHistoryRoot; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AssertionState {
        #[allow(missing_docs)]
        pub globalState: <GlobalState as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub machineStatus: <MachineStatus as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub endHistoryRoot: alloy::sol_types::private::FixedBytes<32>,
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
            GlobalState,
            MachineStatus,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <GlobalState as alloy::sol_types::SolType>::RustType,
            <MachineStatus as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<AssertionState> for UnderlyingRustTuple<'_> {
            fn from(value: AssertionState) -> Self {
                (value.globalState, value.machineStatus, value.endHistoryRoot)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AssertionState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    globalState: tuple.0,
                    machineStatus: tuple.1,
                    endHistoryRoot: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AssertionState {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AssertionState {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <GlobalState as alloy_sol_types::SolType>::tokenize(
                        &self.globalState,
                    ),
                    <MachineStatus as alloy_sol_types::SolType>::tokenize(
                        &self.machineStatus,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.endHistoryRoot),
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
        impl alloy_sol_types::SolType for AssertionState {
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
        impl alloy_sol_types::SolStruct for AssertionState {
            const NAME: &'static str = "AssertionState";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AssertionState(GlobalState globalState,uint8 machineStatus,bytes32 endHistoryRoot)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <GlobalState as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <GlobalState as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <GlobalState as alloy_sol_types::SolType>::eip712_data_word(
                            &self.globalState,
                        )
                        .0,
                    <MachineStatus as alloy_sol_types::SolType>::eip712_data_word(
                            &self.machineStatus,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.endHistoryRoot,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AssertionState {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <GlobalState as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.globalState,
                    )
                    + <MachineStatus as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.machineStatus,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.endHistoryRoot,
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
                <GlobalState as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.globalState,
                    out,
                );
                <MachineStatus as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.machineStatus,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.endHistoryRoot,
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
    /**```solidity
struct BufferConfig { uint64 threshold; uint64 max; uint64 replenishRateInBasis; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BufferConfig {
        #[allow(missing_docs)]
        pub threshold: u64,
        #[allow(missing_docs)]
        pub max: u64,
        #[allow(missing_docs)]
        pub replenishRateInBasis: u64,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u64, u64, u64);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BufferConfig> for UnderlyingRustTuple<'_> {
            fn from(value: BufferConfig) -> Self {
                (value.threshold, value.max, value.replenishRateInBasis)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BufferConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    threshold: tuple.0,
                    max: tuple.1,
                    replenishRateInBasis: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BufferConfig {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BufferConfig {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.max),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.replenishRateInBasis),
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
        impl alloy_sol_types::SolType for BufferConfig {
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
        impl alloy_sol_types::SolStruct for BufferConfig {
            const NAME: &'static str = "BufferConfig";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BufferConfig(uint64 threshold,uint64 max,uint64 replenishRateInBasis)",
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
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.threshold)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.max)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.replenishRateInBasis,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BufferConfig {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.threshold,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.max)
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.replenishRateInBasis,
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.threshold,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.max, out);
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.replenishRateInBasis,
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
    /**```solidity
struct Config { uint64 confirmPeriodBlocks; address stakeToken; uint256 baseStake; bytes32 wasmModuleRoot; address owner; address loserStakeEscrow; uint256 chainId; string chainConfig; uint256 minimumAssertionPeriod; uint64 validatorAfkBlocks; uint256[] miniStakeValues; ISequencerInbox.MaxTimeVariation sequencerInboxMaxTimeVariation; uint256 layerZeroBlockEdgeHeight; uint256 layerZeroBigStepEdgeHeight; uint256 layerZeroSmallStepEdgeHeight; AssertionState genesisAssertionState; uint256 genesisInboxCount; address anyTrustFastConfirmer; uint8 numBigStepLevel; uint64 challengeGracePeriodBlocks; BufferConfig bufferConfig; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Config {
        #[allow(missing_docs)]
        pub confirmPeriodBlocks: u64,
        #[allow(missing_docs)]
        pub stakeToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub baseStake: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub wasmModuleRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub loserStakeEscrow: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub chainConfig: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub minimumAssertionPeriod: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub validatorAfkBlocks: u64,
        #[allow(missing_docs)]
        pub miniStakeValues: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub sequencerInboxMaxTimeVariation: <ISequencerInbox::MaxTimeVariation as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub layerZeroBlockEdgeHeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub layerZeroBigStepEdgeHeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub layerZeroSmallStepEdgeHeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub genesisAssertionState: <AssertionState as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub genesisInboxCount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub anyTrustFastConfirmer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub numBigStepLevel: u8,
        #[allow(missing_docs)]
        pub challengeGracePeriodBlocks: u64,
        #[allow(missing_docs)]
        pub bufferConfig: <BufferConfig as alloy::sol_types::SolType>::RustType,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            ISequencerInbox::MaxTimeVariation,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            AssertionState,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<64>,
            BufferConfig,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::String,
            alloy::sol_types::private::primitives::aliases::U256,
            u64,
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            <ISequencerInbox::MaxTimeVariation as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            <AssertionState as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            u8,
            u64,
            <BufferConfig as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<Config> for UnderlyingRustTuple<'_> {
            fn from(value: Config) -> Self {
                (
                    value.confirmPeriodBlocks,
                    value.stakeToken,
                    value.baseStake,
                    value.wasmModuleRoot,
                    value.owner,
                    value.loserStakeEscrow,
                    value.chainId,
                    value.chainConfig,
                    value.minimumAssertionPeriod,
                    value.validatorAfkBlocks,
                    value.miniStakeValues,
                    value.sequencerInboxMaxTimeVariation,
                    value.layerZeroBlockEdgeHeight,
                    value.layerZeroBigStepEdgeHeight,
                    value.layerZeroSmallStepEdgeHeight,
                    value.genesisAssertionState,
                    value.genesisInboxCount,
                    value.anyTrustFastConfirmer,
                    value.numBigStepLevel,
                    value.challengeGracePeriodBlocks,
                    value.bufferConfig,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Config {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    confirmPeriodBlocks: tuple.0,
                    stakeToken: tuple.1,
                    baseStake: tuple.2,
                    wasmModuleRoot: tuple.3,
                    owner: tuple.4,
                    loserStakeEscrow: tuple.5,
                    chainId: tuple.6,
                    chainConfig: tuple.7,
                    minimumAssertionPeriod: tuple.8,
                    validatorAfkBlocks: tuple.9,
                    miniStakeValues: tuple.10,
                    sequencerInboxMaxTimeVariation: tuple.11,
                    layerZeroBlockEdgeHeight: tuple.12,
                    layerZeroBigStepEdgeHeight: tuple.13,
                    layerZeroSmallStepEdgeHeight: tuple.14,
                    genesisAssertionState: tuple.15,
                    genesisInboxCount: tuple.16,
                    anyTrustFastConfirmer: tuple.17,
                    numBigStepLevel: tuple.18,
                    challengeGracePeriodBlocks: tuple.19,
                    bufferConfig: tuple.20,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Config {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Config {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.confirmPeriodBlocks),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.stakeToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseStake),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.wasmModuleRoot),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.loserStakeEscrow,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.chainConfig,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.minimumAssertionPeriod,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorAfkBlocks),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.miniStakeValues),
                    <ISequencerInbox::MaxTimeVariation as alloy_sol_types::SolType>::tokenize(
                        &self.sequencerInboxMaxTimeVariation,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.layerZeroBlockEdgeHeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.layerZeroBigStepEdgeHeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.layerZeroSmallStepEdgeHeight,
                    ),
                    <AssertionState as alloy_sol_types::SolType>::tokenize(
                        &self.genesisAssertionState,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.genesisInboxCount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.anyTrustFastConfirmer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.numBigStepLevel),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.challengeGracePeriodBlocks,
                    ),
                    <BufferConfig as alloy_sol_types::SolType>::tokenize(
                        &self.bufferConfig,
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
        impl alloy_sol_types::SolType for Config {
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
        impl alloy_sol_types::SolStruct for Config {
            const NAME: &'static str = "Config";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Config(uint64 confirmPeriodBlocks,address stakeToken,uint256 baseStake,bytes32 wasmModuleRoot,address owner,address loserStakeEscrow,uint256 chainId,string chainConfig,uint256 minimumAssertionPeriod,uint64 validatorAfkBlocks,uint256[] miniStakeValues,ISequencerInbox.MaxTimeVariation sequencerInboxMaxTimeVariation,uint256 layerZeroBlockEdgeHeight,uint256 layerZeroBigStepEdgeHeight,uint256 layerZeroSmallStepEdgeHeight,AssertionState genesisAssertionState,uint256 genesisInboxCount,address anyTrustFastConfirmer,uint8 numBigStepLevel,uint64 challengeGracePeriodBlocks,BufferConfig bufferConfig)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(3);
                components
                    .push(
                        <ISequencerInbox::MaxTimeVariation as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ISequencerInbox::MaxTimeVariation as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <AssertionState as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <AssertionState as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <BufferConfig as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BufferConfig as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.confirmPeriodBlocks,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.stakeToken,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.baseStake)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.wasmModuleRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.owner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.loserStakeEscrow,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.chainId)
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.chainConfig,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.minimumAssertionPeriod,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.validatorAfkBlocks,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.miniStakeValues,
                        )
                        .0,
                    <ISequencerInbox::MaxTimeVariation as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sequencerInboxMaxTimeVariation,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.layerZeroBlockEdgeHeight,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.layerZeroBigStepEdgeHeight,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.layerZeroSmallStepEdgeHeight,
                        )
                        .0,
                    <AssertionState as alloy_sol_types::SolType>::eip712_data_word(
                            &self.genesisAssertionState,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.genesisInboxCount,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.anyTrustFastConfirmer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.numBigStepLevel,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.challengeGracePeriodBlocks,
                        )
                        .0,
                    <BufferConfig as alloy_sol_types::SolType>::eip712_data_word(
                            &self.bufferConfig,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Config {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.confirmPeriodBlocks,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.stakeToken,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.baseStake,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wasmModuleRoot,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.owner,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.loserStakeEscrow,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.chainId,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.chainConfig,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.minimumAssertionPeriod,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.validatorAfkBlocks,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.miniStakeValues,
                    )
                    + <ISequencerInbox::MaxTimeVariation as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sequencerInboxMaxTimeVariation,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.layerZeroBlockEdgeHeight,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.layerZeroBigStepEdgeHeight,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.layerZeroSmallStepEdgeHeight,
                    )
                    + <AssertionState as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.genesisAssertionState,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.genesisInboxCount,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.anyTrustFastConfirmer,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.numBigStepLevel,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.challengeGracePeriodBlocks,
                    )
                    + <BufferConfig as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.bufferConfig,
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.confirmPeriodBlocks,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stakeToken,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.baseStake,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wasmModuleRoot,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.owner,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.loserStakeEscrow,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.chainId,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.chainConfig,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.minimumAssertionPeriod,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.validatorAfkBlocks,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.miniStakeValues,
                    out,
                );
                <ISequencerInbox::MaxTimeVariation as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sequencerInboxMaxTimeVariation,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.layerZeroBlockEdgeHeight,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.layerZeroBigStepEdgeHeight,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.layerZeroSmallStepEdgeHeight,
                    out,
                );
                <AssertionState as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.genesisAssertionState,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.genesisInboxCount,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.anyTrustFastConfirmer,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.numBigStepLevel,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.challengeGracePeriodBlocks,
                    out,
                );
                <BufferConfig as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.bufferConfig,
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
    /**```solidity
struct ContractDependencies { address bridge; address sequencerInbox; address inbox; address outbox; address rollupEventInbox; address challengeManager; address rollupAdminLogic; address rollupUserLogic; address validatorWalletCreator; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ContractDependencies {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sequencerInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub inbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub outbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub rollupEventInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub challengeManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub rollupAdminLogic: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub rollupUserLogic: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub validatorWalletCreator: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<ContractDependencies> for UnderlyingRustTuple<'_> {
            fn from(value: ContractDependencies) -> Self {
                (
                    value.bridge,
                    value.sequencerInbox,
                    value.inbox,
                    value.outbox,
                    value.rollupEventInbox,
                    value.challengeManager,
                    value.rollupAdminLogic,
                    value.rollupUserLogic,
                    value.validatorWalletCreator,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ContractDependencies {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bridge: tuple.0,
                    sequencerInbox: tuple.1,
                    inbox: tuple.2,
                    outbox: tuple.3,
                    rollupEventInbox: tuple.4,
                    challengeManager: tuple.5,
                    rollupAdminLogic: tuple.6,
                    rollupUserLogic: tuple.7,
                    validatorWalletCreator: tuple.8,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ContractDependencies {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ContractDependencies {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.bridge,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sequencerInbox,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.inbox,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.outbox,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.rollupEventInbox,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.challengeManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.rollupAdminLogic,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.rollupUserLogic,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.validatorWalletCreator,
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
        impl alloy_sol_types::SolType for ContractDependencies {
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
        impl alloy_sol_types::SolStruct for ContractDependencies {
            const NAME: &'static str = "ContractDependencies";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ContractDependencies(address bridge,address sequencerInbox,address inbox,address outbox,address rollupEventInbox,address challengeManager,address rollupAdminLogic,address rollupUserLogic,address validatorWalletCreator)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.bridge,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sequencerInbox,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.inbox,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.outbox,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rollupEventInbox,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.challengeManager,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rollupAdminLogic,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rollupUserLogic,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.validatorWalletCreator,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ContractDependencies {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.bridge,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sequencerInbox,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.inbox,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.outbox,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rollupEventInbox,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.challengeManager,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rollupAdminLogic,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rollupUserLogic,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.validatorWalletCreator,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.bridge,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sequencerInbox,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.inbox,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.outbox,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rollupEventInbox,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.challengeManager,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rollupAdminLogic,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rollupUserLogic,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.validatorWalletCreator,
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
    /**```solidity
struct GlobalState { bytes32[2] bytes32Vals; uint64[2] u64Vals; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GlobalState {
        #[allow(missing_docs)]
        pub bytes32Vals: [alloy::sol_types::private::FixedBytes<32>; 2usize],
        #[allow(missing_docs)]
        pub u64Vals: [u64; 2usize],
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
            alloy::sol_types::sol_data::FixedArray<
                alloy::sol_types::sol_data::FixedBytes<32>,
                2usize,
            >,
            alloy::sol_types::sol_data::FixedArray<
                alloy::sol_types::sol_data::Uint<64>,
                2usize,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [alloy::sol_types::private::FixedBytes<32>; 2usize],
            [u64; 2usize],
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
        impl ::core::convert::From<GlobalState> for UnderlyingRustTuple<'_> {
            fn from(value: GlobalState) -> Self {
                (value.bytes32Vals, value.u64Vals)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GlobalState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bytes32Vals: tuple.0,
                    u64Vals: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for GlobalState {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for GlobalState {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.bytes32Vals),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<64>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.u64Vals),
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
        impl alloy_sol_types::SolType for GlobalState {
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
        impl alloy_sol_types::SolStruct for GlobalState {
            const NAME: &'static str = "GlobalState";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "GlobalState(bytes32[2] bytes32Vals,uint64[2] u64Vals)",
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
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.bytes32Vals)
                        .0,
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<64>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.u64Vals)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GlobalState {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.bytes32Vals,
                    )
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<64>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.u64Vals,
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
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.bytes32Vals,
                    out,
                );
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<64>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.u64Vals,
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
    /**Event with signature `AdminChanged(address,address)` and selector `0x7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f`.
```solidity
event AdminChanged(address previousAdmin, address newAdmin);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AdminChanged {
        #[allow(missing_docs)]
        pub previousAdmin: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newAdmin: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for AdminChanged {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AdminChanged(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                126u8,
                100u8,
                77u8,
                121u8,
                66u8,
                47u8,
                23u8,
                192u8,
                30u8,
                72u8,
                148u8,
                181u8,
                244u8,
                245u8,
                136u8,
                211u8,
                49u8,
                235u8,
                250u8,
                40u8,
                101u8,
                61u8,
                66u8,
                174u8,
                131u8,
                45u8,
                197u8,
                158u8,
                56u8,
                201u8,
                121u8,
                143u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousAdmin: data.0,
                    newAdmin: data.1,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.previousAdmin,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newAdmin,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
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
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for AdminChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AdminChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AdminChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `BeaconUpgraded(address)` and selector `0x1cf3b03a6cf19fa2baba4df148e9dcabedea7f8a5c07840e207e5c089be95d3e`.
```solidity
event BeaconUpgraded(address indexed beacon);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BeaconUpgraded {
        #[allow(missing_docs)]
        pub beacon: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for BeaconUpgraded {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "BeaconUpgraded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                28u8,
                243u8,
                176u8,
                58u8,
                108u8,
                241u8,
                159u8,
                162u8,
                186u8,
                186u8,
                77u8,
                241u8,
                72u8,
                233u8,
                220u8,
                171u8,
                237u8,
                234u8,
                127u8,
                138u8,
                92u8,
                7u8,
                132u8,
                14u8,
                32u8,
                126u8,
                92u8,
                8u8,
                155u8,
                233u8,
                93u8,
                62u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { beacon: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.beacon.clone())
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
                    &self.beacon,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BeaconUpgraded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BeaconUpgraded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BeaconUpgraded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Upgraded(address)` and selector `0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b`.
```solidity
event Upgraded(address indexed implementation);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Upgraded {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for Upgraded {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Upgraded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                188u8,
                124u8,
                215u8,
                90u8,
                32u8,
                238u8,
                39u8,
                253u8,
                154u8,
                222u8,
                186u8,
                179u8,
                32u8,
                65u8,
                247u8,
                85u8,
                33u8,
                77u8,
                188u8,
                107u8,
                255u8,
                169u8,
                12u8,
                192u8,
                34u8,
                91u8,
                57u8,
                218u8,
                46u8,
                92u8,
                45u8,
                59u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { implementation: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.implementation.clone())
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
                    &self.implementation,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Upgraded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Upgraded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Upgraded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UpgradedSecondary(address)` and selector `0xf7eed2a7fabbf1bec8d55ed5e785cc76622376dde5df4ff15470551e030b8134`.
```solidity
event UpgradedSecondary(address indexed implementation);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpgradedSecondary {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for UpgradedSecondary {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "UpgradedSecondary(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                247u8,
                238u8,
                210u8,
                167u8,
                250u8,
                187u8,
                241u8,
                190u8,
                200u8,
                213u8,
                94u8,
                213u8,
                231u8,
                133u8,
                204u8,
                118u8,
                98u8,
                35u8,
                118u8,
                221u8,
                229u8,
                223u8,
                79u8,
                241u8,
                84u8,
                112u8,
                85u8,
                30u8,
                3u8,
                11u8,
                129u8,
                52u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { implementation: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.implementation.clone())
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
                    &self.implementation,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UpgradedSecondary {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpgradedSecondary> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UpgradedSecondary) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `initializeProxy((uint64,address,uint256,bytes32,address,address,uint256,string,uint256,uint64,uint256[],(uint256,uint256,uint256,uint256),uint256,uint256,uint256,((bytes32[2],uint64[2]),uint8,bytes32),uint256,address,uint8,uint64,(uint64,uint64,uint64)),(address,address,address,address,address,address,address,address,address))` and selector `0xadfef6ac`.
```solidity
function initializeProxy(Config memory config, ContractDependencies memory connectedContracts) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeProxyCall {
        #[allow(missing_docs)]
        pub config: <Config as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub connectedContracts: <ContractDependencies as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initializeProxy((uint64,address,uint256,bytes32,address,address,uint256,string,uint256,uint64,uint256[],(uint256,uint256,uint256,uint256),uint256,uint256,uint256,((bytes32[2],uint64[2]),uint8,bytes32),uint256,address,uint8,uint64,(uint64,uint64,uint64)),(address,address,address,address,address,address,address,address,address))`](initializeProxyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeProxyReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Config, ContractDependencies);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Config as alloy::sol_types::SolType>::RustType,
                <ContractDependencies as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<initializeProxyCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeProxyCall) -> Self {
                    (value.config, value.connectedContracts)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeProxyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        config: tuple.0,
                        connectedContracts: tuple.1,
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
            impl ::core::convert::From<initializeProxyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeProxyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeProxyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeProxyCall {
            type Parameters<'a> = (Config, ContractDependencies);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeProxyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeProxy((uint64,address,uint256,bytes32,address,address,uint256,string,uint256,uint64,uint256[],(uint256,uint256,uint256,uint256),uint256,uint256,uint256,((bytes32[2],uint64[2]),uint8,bytes32),uint256,address,uint8,uint64,(uint64,uint64,uint64)),(address,address,address,address,address,address,address,address,address))";
            const SELECTOR: [u8; 4] = [173u8, 254u8, 246u8, 172u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Config as alloy_sol_types::SolType>::tokenize(&self.config),
                    <ContractDependencies as alloy_sol_types::SolType>::tokenize(
                        &self.connectedContracts,
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
    ///Container for all the [`RollupProxy`](self) function calls.
    pub enum RollupProxyCalls {
        #[allow(missing_docs)]
        initializeProxy(initializeProxyCall),
    }
    #[automatically_derived]
    impl RollupProxyCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[173u8, 254u8, 246u8, 172u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RollupProxyCalls {
        const NAME: &'static str = "RollupProxyCalls";
        const MIN_DATA_LENGTH: usize = 1344usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::initializeProxy(_) => {
                    <initializeProxyCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<RollupProxyCalls>] = &[
                {
                    fn initializeProxy(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupProxyCalls> {
                        <initializeProxyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupProxyCalls::initializeProxy)
                    }
                    initializeProxy
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
                Self::initializeProxy(inner) => {
                    <initializeProxyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::initializeProxy(inner) => {
                    <initializeProxyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`RollupProxy`](self) events.
    pub enum RollupProxyEvents {
        #[allow(missing_docs)]
        AdminChanged(AdminChanged),
        #[allow(missing_docs)]
        BeaconUpgraded(BeaconUpgraded),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
        #[allow(missing_docs)]
        UpgradedSecondary(UpgradedSecondary),
    }
    #[automatically_derived]
    impl RollupProxyEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                28u8,
                243u8,
                176u8,
                58u8,
                108u8,
                241u8,
                159u8,
                162u8,
                186u8,
                186u8,
                77u8,
                241u8,
                72u8,
                233u8,
                220u8,
                171u8,
                237u8,
                234u8,
                127u8,
                138u8,
                92u8,
                7u8,
                132u8,
                14u8,
                32u8,
                126u8,
                92u8,
                8u8,
                155u8,
                233u8,
                93u8,
                62u8,
            ],
            [
                126u8,
                100u8,
                77u8,
                121u8,
                66u8,
                47u8,
                23u8,
                192u8,
                30u8,
                72u8,
                148u8,
                181u8,
                244u8,
                245u8,
                136u8,
                211u8,
                49u8,
                235u8,
                250u8,
                40u8,
                101u8,
                61u8,
                66u8,
                174u8,
                131u8,
                45u8,
                197u8,
                158u8,
                56u8,
                201u8,
                121u8,
                143u8,
            ],
            [
                188u8,
                124u8,
                215u8,
                90u8,
                32u8,
                238u8,
                39u8,
                253u8,
                154u8,
                222u8,
                186u8,
                179u8,
                32u8,
                65u8,
                247u8,
                85u8,
                33u8,
                77u8,
                188u8,
                107u8,
                255u8,
                169u8,
                12u8,
                192u8,
                34u8,
                91u8,
                57u8,
                218u8,
                46u8,
                92u8,
                45u8,
                59u8,
            ],
            [
                247u8,
                238u8,
                210u8,
                167u8,
                250u8,
                187u8,
                241u8,
                190u8,
                200u8,
                213u8,
                94u8,
                213u8,
                231u8,
                133u8,
                204u8,
                118u8,
                98u8,
                35u8,
                118u8,
                221u8,
                229u8,
                223u8,
                79u8,
                241u8,
                84u8,
                112u8,
                85u8,
                30u8,
                3u8,
                11u8,
                129u8,
                52u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for RollupProxyEvents {
        const NAME: &'static str = "RollupProxyEvents";
        const COUNT: usize = 4usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<AdminChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AdminChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AdminChanged)
                }
                Some(<BeaconUpgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BeaconUpgraded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BeaconUpgraded)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Upgraded)
                }
                Some(
                    <UpgradedSecondary as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UpgradedSecondary as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UpgradedSecondary)
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
    impl alloy_sol_types::private::IntoLogData for RollupProxyEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BeaconUpgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpgradedSecondary(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BeaconUpgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpgradedSecondary(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`RollupProxy`](self) contract instance.

See the [wrapper's documentation](`RollupProxyInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> RollupProxyInstance<T, P, N> {
        RollupProxyInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<RollupProxyInstance<T, P, N>>,
    > {
        RollupProxyInstance::<T, P, N>::deploy(provider)
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
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        RollupProxyInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`RollupProxy`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`RollupProxy`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct RollupProxyInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for RollupProxyInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("RollupProxyInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > RollupProxyInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`RollupProxy`](self) contract instance.

See the [wrapper's documentation](`RollupProxyInstance`) for more details.*/
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
        ) -> alloy_contract::Result<RollupProxyInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
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
    impl<T, P: ::core::clone::Clone, N> RollupProxyInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> RollupProxyInstance<T, P, N> {
            RollupProxyInstance {
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
    > RollupProxyInstance<T, P, N> {
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
        ///Creates a new call builder for the [`initializeProxy`] function.
        pub fn initializeProxy(
            &self,
            config: <Config as alloy::sol_types::SolType>::RustType,
            connectedContracts: <ContractDependencies as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeProxyCall, N> {
            self.call_builder(
                &initializeProxyCall {
                    config,
                    connectedContracts,
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
    > RollupProxyInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AdminChanged`] event.
        pub fn AdminChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AdminChanged, N> {
            self.event_filter::<AdminChanged>()
        }
        ///Creates a new event filter for the [`BeaconUpgraded`] event.
        pub fn BeaconUpgraded_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BeaconUpgraded, N> {
            self.event_filter::<BeaconUpgraded>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<T, &P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
        ///Creates a new event filter for the [`UpgradedSecondary`] event.
        pub fn UpgradedSecondary_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UpgradedSecondary, N> {
            self.event_filter::<UpgradedSecondary>()
        }
    }
}
