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

interface BridgeCreator {
    struct BridgeContracts {
        address bridge;
        address inbox;
        address sequencerInbox;
        address rollupEventInbox;
        address outbox;
    }
    struct BridgeTemplates {
        address bridge;
        address sequencerInbox;
        address delayBufferableSequencerInbox;
        address inbox;
        address rollupEventInbox;
        address outbox;
    }
    struct BufferConfig {
        uint64 threshold;
        uint64 max;
        uint64 replenishRateInBasis;
    }

    event ERC20TemplatesUpdated();
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event TemplatesUpdated();

    constructor(BridgeTemplates _ethBasedTemplates, BridgeTemplates _erc20BasedTemplates);

    function createBridge(address adminProxy, address rollup, address nativeToken, ISequencerInbox.MaxTimeVariation memory maxTimeVariation, BufferConfig memory bufferConfig) external returns (BridgeContracts memory);
    function erc20BasedTemplates() external view returns (address bridge, address sequencerInbox, address delayBufferableSequencerInbox, address inbox, address rollupEventInbox, address outbox);
    function ethBasedTemplates() external view returns (address bridge, address sequencerInbox, address delayBufferableSequencerInbox, address inbox, address rollupEventInbox, address outbox);
    function owner() external view returns (address);
    function renounceOwnership() external;
    function transferOwnership(address newOwner) external;
    function updateERC20Templates(BridgeTemplates memory _newTemplates) external;
    function updateTemplates(BridgeTemplates memory _newTemplates) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_ethBasedTemplates",
        "type": "tuple",
        "internalType": "struct BridgeCreator.BridgeTemplates",
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
            "name": "delayBufferableSequencerInbox",
            "type": "address",
            "internalType": "contract ISequencerInbox"
          },
          {
            "name": "inbox",
            "type": "address",
            "internalType": "contract IInboxBase"
          },
          {
            "name": "rollupEventInbox",
            "type": "address",
            "internalType": "contract IRollupEventInbox"
          },
          {
            "name": "outbox",
            "type": "address",
            "internalType": "contract IOutbox"
          }
        ]
      },
      {
        "name": "_erc20BasedTemplates",
        "type": "tuple",
        "internalType": "struct BridgeCreator.BridgeTemplates",
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
            "name": "delayBufferableSequencerInbox",
            "type": "address",
            "internalType": "contract ISequencerInbox"
          },
          {
            "name": "inbox",
            "type": "address",
            "internalType": "contract IInboxBase"
          },
          {
            "name": "rollupEventInbox",
            "type": "address",
            "internalType": "contract IRollupEventInbox"
          },
          {
            "name": "outbox",
            "type": "address",
            "internalType": "contract IOutbox"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createBridge",
    "inputs": [
      {
        "name": "adminProxy",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "rollup",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "nativeToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "maxTimeVariation",
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
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct BridgeCreator.BridgeContracts",
        "components": [
          {
            "name": "bridge",
            "type": "address",
            "internalType": "contract IBridge"
          },
          {
            "name": "inbox",
            "type": "address",
            "internalType": "contract IInboxBase"
          },
          {
            "name": "sequencerInbox",
            "type": "address",
            "internalType": "contract ISequencerInbox"
          },
          {
            "name": "rollupEventInbox",
            "type": "address",
            "internalType": "contract IRollupEventInbox"
          },
          {
            "name": "outbox",
            "type": "address",
            "internalType": "contract IOutbox"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "erc20BasedTemplates",
    "inputs": [],
    "outputs": [
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
        "name": "delayBufferableSequencerInbox",
        "type": "address",
        "internalType": "contract ISequencerInbox"
      },
      {
        "name": "inbox",
        "type": "address",
        "internalType": "contract IInboxBase"
      },
      {
        "name": "rollupEventInbox",
        "type": "address",
        "internalType": "contract IRollupEventInbox"
      },
      {
        "name": "outbox",
        "type": "address",
        "internalType": "contract IOutbox"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ethBasedTemplates",
    "inputs": [],
    "outputs": [
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
        "name": "delayBufferableSequencerInbox",
        "type": "address",
        "internalType": "contract ISequencerInbox"
      },
      {
        "name": "inbox",
        "type": "address",
        "internalType": "contract IInboxBase"
      },
      {
        "name": "rollupEventInbox",
        "type": "address",
        "internalType": "contract IRollupEventInbox"
      },
      {
        "name": "outbox",
        "type": "address",
        "internalType": "contract IOutbox"
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
    "name": "updateERC20Templates",
    "inputs": [
      {
        "name": "_newTemplates",
        "type": "tuple",
        "internalType": "struct BridgeCreator.BridgeTemplates",
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
            "name": "delayBufferableSequencerInbox",
            "type": "address",
            "internalType": "contract ISequencerInbox"
          },
          {
            "name": "inbox",
            "type": "address",
            "internalType": "contract IInboxBase"
          },
          {
            "name": "rollupEventInbox",
            "type": "address",
            "internalType": "contract IRollupEventInbox"
          },
          {
            "name": "outbox",
            "type": "address",
            "internalType": "contract IOutbox"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateTemplates",
    "inputs": [
      {
        "name": "_newTemplates",
        "type": "tuple",
        "internalType": "struct BridgeCreator.BridgeTemplates",
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
            "name": "delayBufferableSequencerInbox",
            "type": "address",
            "internalType": "contract ISequencerInbox"
          },
          {
            "name": "inbox",
            "type": "address",
            "internalType": "contract IInboxBase"
          },
          {
            "name": "rollupEventInbox",
            "type": "address",
            "internalType": "contract IRollupEventInbox"
          },
          {
            "name": "outbox",
            "type": "address",
            "internalType": "contract IOutbox"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "ERC20TemplatesUpdated",
    "inputs": [],
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
    "name": "TemplatesUpdated",
    "inputs": [],
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
pub mod BridgeCreator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50604051611db7380380611db783398101604081905261002e9161024a565b61003733610135565b8151600180546001600160a01b03199081166001600160a01b039384161790915560208085015160028054841691851691909117905560408086015160038054851691861691909117905560608087015160048054861691871691909117905560808088015160058054871691881691909117905560a09788015160068054871691881691909117905586516007805487169188169190911790559286015160088054861691871691909117905590850151600980548516918616919091179055840151600a80548416918516919091179055830151600b805483169184169190911790559190920151600c8054909216921691909117905561027e565b5f80546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b80516001600160a01b038116811461019a575f80fd5b919050565b5f60c082840312156101af575f80fd5b60405160c081016001600160401b03811182821017156101dd57634e487b7160e01b5f52604160045260245ffd5b6040529050806101ec83610184565b81526101fa60208401610184565b602082015261020b60408401610184565b604082015261021c60608401610184565b606082015261022d60808401610184565b608082015261023e60a08401610184565b60a08201525092915050565b5f80610180838503121561025c575f80fd5b610266848461019f565b91506102758460c0850161019f565b90509250929050565b611b2c8061028b5f395ff3fe608060405234801561000f575f80fd5b5060043610610085575f3560e01c80638da5cb5b116100585780638da5cb5b146101a0578063ceab9410146101ba578063e83b0e16146101cd578063f2fde38b146101e0575f80fd5b806311f022271461008957806357d3a20014610102578063715018a61461016457806376768ab91461016e575b5f80fd5b6001546002546003546004546005546006546100bb956001600160a01b03908116958116948116938116928116911686565b604080516001600160a01b03978816815295871660208701529386169385019390935290841660608401528316608083015290911660a082015260c0015b60405180910390f35b610115610110366004610a34565b6101f3565b6040516100f9919081516001600160a01b039081168252602080840151821690830152604080840151821690830152606080840151821690830152608092830151169181019190915260a00190565b61016c610595565b005b600754600854600954600a54600b54600c546100bb956001600160a01b03908116958116948116938116928116911686565b5f546040516001600160a01b0390911681526020016100f9565b61016c6101c8366004610aec565b6105a8565b61016c6101db366004610aec565b6105ea565b61016c6101ee366004610b02565b61062c565b6040805160a0810182525f808252602082018190529181018290526060810182905260808101919091525f80363360405160200161023393929190610b24565b60408051601f19818403018152919052805160209182012091505f9061025b90850185610b80565b67ffffffffffffffff16151590505f6102e5838a6001600160a01b038a1615610285576007610288565b60015b6040805160c08101825282546001600160a01b03908116825260018401548116602083015260028401548116928201929092526003830154821660608201526004830154821660808201526005909201541660a0820152856106db565b90506001600160a01b03871661035357805160405163189acdbd60e31b81526001600160a01b038a811660048301529091169063c4d66de8906024015f604051808303815f87803b158015610338575f80fd5b505af115801561034a573d5f803e3d5ffd5b505050506103ce565b80516040517f485cc9550000000000000000000000000000000000000000000000000000000081526001600160a01b038a8116600483015289811660248301529091169063485cc955906044015f604051808303815f87803b1580156103b7575f80fd5b505af11580156103c9573d5f803e3d5ffd5b505050505b604080820151825191517f1ad87e450000000000000000000000000000000000000000000000000000000081526001600160a01b0390911691631ad87e459161041e91908a908a90600401610b99565b5f604051808303815f87803b158015610435575f80fd5b505af1158015610447573d5f803e3d5ffd5b505050506020810151815160408084015190517f485cc9550000000000000000000000000000000000000000000000000000000081526001600160a01b039283166004820152908216602482015291169063485cc955906044015f604051808303815f87803b1580156104b8575f80fd5b505af11580156104ca573d5f803e3d5ffd5b505050506060810151815160405163189acdbd60e31b81526001600160a01b03918216600482015291169063c4d66de8906024015f604051808303815f87803b158015610515575f80fd5b505af1158015610527573d5f803e3d5ffd5b505050506080810151815160405163189acdbd60e31b81526001600160a01b03918216600482015291169063c4d66de8906024015f604051808303815f87803b158015610572575f80fd5b505af1158015610584573d5f803e3d5ffd5b50929b9a5050505050505050505050565b61059d610944565b6105a65f6109b7565b565b6105b0610944565b8060076105bd8282610c1a565b50506040517fa47434bb6d1ddd5521e8980ded6a783513e159f80437d78715b10e6e8b6bba5e905f90a150565b6105f2610944565b8060016105ff8282610c1a565b50506040517fc9d3947d22fa124aaec4c7e8c919f79016e2d7b48eee10568375d98b86460d1b905f90a150565b610634610944565b6001600160a01b0381166106cf576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f646472657373000000000000000000000000000000000000000000000000000060648201526084015b60405180910390fd5b6106d8816109b7565b50565b6040805160a0810182525f808252602082018190529181018290526060810182905260808101919091526040805160a0810182525f8082526020820181905291810182905260608101829052608081019190915285845f01518660405161074190610a13565b6001600160a01b039283168152911660208201526060604082018190525f908201526080018190604051809103905ff5905080158015610783573d5f803e3d5ffd5b506001600160a01b03168152858361079f5784602001516107a5565b84604001515b866040516107b290610a13565b6001600160a01b039283168152911660208201526060604082018190525f908201526080018190604051809103905ff59050801580156107f4573d5f803e3d5ffd5b506001600160a01b031660408083019190915260608501519051879190879061081c90610a13565b6001600160a01b039283168152911660208201526060604082018190525f908201526080018190604051809103905ff590508015801561085e573d5f803e3d5ffd5b506001600160a01b031660208201526080840151604051879190879061088390610a13565b6001600160a01b039283168152911660208201526060604082018190525f908201526080018190604051809103905ff59050801580156108c5573d5f803e3d5ffd5b506001600160a01b0316606082015260a084015160405187919087906108ea90610a13565b6001600160a01b039283168152911660208201526060604082018190525f908201526080018190604051809103905ff590508015801561092c573d5f803e3d5ffd5b506001600160a01b0316608082015295945050505050565b5f546001600160a01b031633146105a6576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016106c6565b5f80546001600160a01b0383811673ffffffffffffffffffffffffffffffffffffffff19831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b610d8980610d6e83390190565b6001600160a01b03811681146106d8575f80fd5b5f805f805f858703610140811215610a4a575f80fd5b8635610a5581610a20565b95506020870135610a6581610a20565b94506040870135610a7581610a20565b935060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa082011215610aa6575f80fd5b60608701925060607fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2082011215610adb575f80fd5b5060e0860190509295509295909350565b5f60c08284031215610afc575f80fd5b50919050565b5f60208284031215610b12575f80fd5b8135610b1d81610a20565b9392505050565b60408152826040820152828460608301375f606084830101525f6060601f19601f86011683010190506001600160a01b0383166020830152949350505050565b803567ffffffffffffffff81168114610b7b575f80fd5b919050565b5f60208284031215610b90575f80fd5b610b1d82610b64565b5f610100820190506001600160a01b03851682528335602083015260208401356040830152604084013560608301526060840135608083015267ffffffffffffffff80610be585610b64565b1660a084015280610bf860208601610b64565b1660c084015280610c0b60408601610b64565b1660e084015250949350505050565b8135610c2581610a20565b815473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b038216178255506020820135610c5a81610a20565b60018201805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b038316179055506040820135610c9381610a20565b60028201805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b038316179055506060820135610ccc81610a20565b60038201805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b038316179055506080820135610d0581610a20565b60048201805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b0383161790555060a0820135610d3e81610a20565b60058201805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b03831617905550505056fe6080604052604051610d89380380610d89833981016040819052610022916103b7565b828161002f82825f610043565b5061003b90508261006e565b5050506104cd565b61004c836100db565b5f825111806100585750805b1561006957610067838361011a565b505b505050565b7f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f6100ad5f80516020610d42833981519152546001600160a01b031690565b604080516001600160a01b03928316815291841660208301520160405180910390a16100d881610146565b50565b6100e4816101e1565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a250565b606061013f8383604051806060016040528060278152602001610d6260279139610275565b9392505050565b6001600160a01b0381166101b05760405162461bcd60e51b815260206004820152602660248201527f455243313936373a206e65772061646d696e20697320746865207a65726f206160448201526564647265737360d01b60648201526084015b60405180910390fd5b805f80516020610d428339815191525b80546001600160a01b0319166001600160a01b039290921691909117905550565b6001600160a01b0381163b61024e5760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201526c1bdd08184818dbdb9d1c9858dd609a1b60648201526084016101a7565b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc6101c0565b60606001600160a01b0384163b6102dd5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f6044820152651b9d1c9858dd60d21b60648201526084016101a7565b5f80856001600160a01b0316856040516102f79190610482565b5f60405180830381855af49150503d805f811461032f576040519150601f19603f3d011682016040523d82523d5f602084013e610334565b606091505b50909250905061034582828661034f565b9695505050505050565b6060831561035e57508161013f565b82511561036e5782518084602001fd5b8160405162461bcd60e51b81526004016101a79190610498565b80516001600160a01b038116811461039e575f80fd5b919050565b634e487b7160e01b5f52604160045260245ffd5b5f805f606084860312156103c9575f80fd5b6103d284610388565b92506103e060208501610388565b60408501519092506001600160401b03808211156103fc575f80fd5b818601915086601f83011261040f575f80fd5b815181811115610421576104216103a3565b604051601f8201601f19908116603f01168101908382118183101715610449576104496103a3565b81604052828152896020848701011115610461575f80fd5b8260208601602083015e5f6020848301015280955050505050509250925092565b5f82518060208501845e5f920191825250919050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b610868806104da5f395ff3fe60806040526004361061005d575f3560e01c80635c60da1b116100425780635c60da1b146100a65780638f283970146100d6578063f851a440146100f55761006c565b80633659cfe6146100745780634f1ef286146100935761006c565b3661006c5761006a610109565b005b61006a610109565b34801561007f575f80fd5b5061006a61008e36600461070d565b610123565b61006a6100a1366004610726565b61015e565b3480156100b1575f80fd5b506100ba6101c4565b6040516001600160a01b03909116815260200160405180910390f35b3480156100e1575f80fd5b5061006a6100f036600461070d565b6101f4565b348015610100575f80fd5b506100ba610214565b610111610234565b61012161011c6102e4565b6102ed565b565b61012b61030b565b6001600160a01b03163303610156576101538160405180602001604052805f8152505f61033d565b50565b610153610109565b61016661030b565b6001600160a01b031633036101bc576101b78383838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152506001925061033d915050565b505050565b6101b7610109565b5f6101cd61030b565b6001600160a01b031633036101e9576101e46102e4565b905090565b6101f1610109565b90565b6101fc61030b565b6001600160a01b031633036101565761015381610367565b5f61021d61030b565b6001600160a01b031633036101e9576101e461030b565b61023c61030b565b6001600160a01b031633036101215760405162461bcd60e51b815260206004820152604260248201527f5472616e73706172656e745570677261646561626c6550726f78793a2061646d60448201527f696e2063616e6e6f742066616c6c6261636b20746f2070726f7879207461726760648201527f6574000000000000000000000000000000000000000000000000000000000000608482015260a4015b60405180910390fd5b5f6101e46103bb565b365f80375f80365f845af43d5f803e808015610307573d5ff35b3d5ffd5b5f7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035b546001600160a01b0316919050565b610346836103e2565b5f825111806103525750805b156101b7576103618383610421565b50505050565b7f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f61039061030b565b604080516001600160a01b03928316815291841660208301520160405180910390a16101538161044d565b5f7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc61032e565b6103eb81610525565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a250565b6060610446838360405180606001604052806027815260200161080c602791396105c9565b9392505050565b6001600160a01b0381166104c95760405162461bcd60e51b815260206004820152602660248201527f455243313936373a206e65772061646d696e20697320746865207a65726f206160448201527f646472657373000000000000000000000000000000000000000000000000000060648201526084016102db565b807fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035b80547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b039290921691909117905550565b6001600160a01b0381163b6105a25760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201527f6f74206120636f6e74726163740000000000000000000000000000000000000060648201526084016102db565b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc6104ec565b60606001600160a01b0384163b6106485760405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f60448201527f6e7472616374000000000000000000000000000000000000000000000000000060648201526084016102db565b5f80856001600160a01b03168560405161066291906107a2565b5f60405180830381855af49150503d805f811461069a576040519150601f19603f3d011682016040523d82523d5f602084013e61069f565b606091505b50915091506106af8282866106b9565b9695505050505050565b606083156106c8575081610446565b8251156106d85782518084602001fd5b8160405162461bcd60e51b81526004016102db91906107b8565b80356001600160a01b0381168114610708575f80fd5b919050565b5f6020828403121561071d575f80fd5b610446826106f2565b5f805f60408486031215610738575f80fd5b610741846106f2565b9250602084013567ffffffffffffffff8082111561075d575f80fd5b818601915086601f830112610770575f80fd5b81358181111561077e575f80fd5b87602082850101111561078f575f80fd5b6020830194508093505050509250925092565b5f82518060208501845e5f920191825250919050565b602081525f82518060208401528060208501604085015e5f6040828501015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8301168401019150509291505056fe416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a26469706673582212205147d38957e9df6e1da1b98751987c55e32bdc4cb009ea7a145ab634aee1897364736f6c63430008190033b53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a2646970667358221220d3d30607b43fda6857967dff289b103cc808d9a1bac2ab84fcf674ef4154d47264736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x1D\xB78\x03\x80a\x1D\xB7\x839\x81\x01`@\x81\x90Ra\0.\x91a\x02JV[a\x0073a\x015V[\x81Q`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U` \x80\x85\x01Q`\x02\x80T\x84\x16\x91\x85\x16\x91\x90\x91\x17\x90U`@\x80\x86\x01Q`\x03\x80T\x85\x16\x91\x86\x16\x91\x90\x91\x17\x90U``\x80\x87\x01Q`\x04\x80T\x86\x16\x91\x87\x16\x91\x90\x91\x17\x90U`\x80\x80\x88\x01Q`\x05\x80T\x87\x16\x91\x88\x16\x91\x90\x91\x17\x90U`\xA0\x97\x88\x01Q`\x06\x80T\x87\x16\x91\x88\x16\x91\x90\x91\x17\x90U\x86Q`\x07\x80T\x87\x16\x91\x88\x16\x91\x90\x91\x17\x90U\x92\x86\x01Q`\x08\x80T\x86\x16\x91\x87\x16\x91\x90\x91\x17\x90U\x90\x85\x01Q`\t\x80T\x85\x16\x91\x86\x16\x91\x90\x91\x17\x90U\x84\x01Q`\n\x80T\x84\x16\x91\x85\x16\x91\x90\x91\x17\x90U\x83\x01Q`\x0B\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x91\x90\x92\x01Q`\x0C\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90Ua\x02~V[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x9AW_\x80\xFD[\x91\x90PV[_`\xC0\x82\x84\x03\x12\x15a\x01\xAFW_\x80\xFD[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01\xDDWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x90P\x80a\x01\xEC\x83a\x01\x84V[\x81Ra\x01\xFA` \x84\x01a\x01\x84V[` \x82\x01Ra\x02\x0B`@\x84\x01a\x01\x84V[`@\x82\x01Ra\x02\x1C``\x84\x01a\x01\x84V[``\x82\x01Ra\x02-`\x80\x84\x01a\x01\x84V[`\x80\x82\x01Ra\x02>`\xA0\x84\x01a\x01\x84V[`\xA0\x82\x01RP\x92\x91PPV[_\x80a\x01\x80\x83\x85\x03\x12\x15a\x02\\W_\x80\xFD[a\x02f\x84\x84a\x01\x9FV[\x91Pa\x02u\x84`\xC0\x85\x01a\x01\x9FV[\x90P\x92P\x92\x90PV[a\x1B,\x80a\x02\x8B_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\x85W_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0XW\x80c\x8D\xA5\xCB[\x14a\x01\xA0W\x80c\xCE\xAB\x94\x10\x14a\x01\xBAW\x80c\xE8;\x0E\x16\x14a\x01\xCDW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xE0W_\x80\xFD[\x80c\x11\xF0\"'\x14a\0\x89W\x80cW\xD3\xA2\0\x14a\x01\x02W\x80cqP\x18\xA6\x14a\x01dW\x80cvv\x8A\xB9\x14a\x01nW[_\x80\xFD[`\x01T`\x02T`\x03T`\x04T`\x05T`\x06Ta\0\xBB\x95`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95\x81\x16\x94\x81\x16\x93\x81\x16\x92\x81\x16\x91\x16\x86V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x81R\x95\x87\x16` \x87\x01R\x93\x86\x16\x93\x85\x01\x93\x90\x93R\x90\x84\x16``\x84\x01R\x83\x16`\x80\x83\x01R\x90\x91\x16`\xA0\x82\x01R`\xC0\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x15a\x01\x106`\x04a\n4V[a\x01\xF3V[`@Qa\0\xF9\x91\x90\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x84\x01Q\x82\x16\x90\x83\x01R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x82\x16\x90\x83\x01R`\x80\x92\x83\x01Q\x16\x91\x81\x01\x91\x90\x91R`\xA0\x01\x90V[a\x01la\x05\x95V[\0[`\x07T`\x08T`\tT`\nT`\x0BT`\x0CTa\0\xBB\x95`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95\x81\x16\x94\x81\x16\x93\x81\x16\x92\x81\x16\x91\x16\x86V[_T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF9V[a\x01la\x01\xC86`\x04a\n\xECV[a\x05\xA8V[a\x01la\x01\xDB6`\x04a\n\xECV[a\x05\xEAV[a\x01la\x01\xEE6`\x04a\x0B\x02V[a\x06,V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R_\x8063`@Q` \x01a\x023\x93\x92\x91\x90a\x0B$V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x91P_\x90a\x02[\x90\x85\x01\x85a\x0B\x80V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x90P_a\x02\xE5\x83\x8A`\x01`\x01`\xA0\x1B\x03\x8A\x16\x15a\x02\x85W`\x07a\x02\x88V[`\x01[`@\x80Q`\xC0\x81\x01\x82R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x84\x01T\x81\x16` \x83\x01R`\x02\x84\x01T\x81\x16\x92\x82\x01\x92\x90\x92R`\x03\x83\x01T\x82\x16``\x82\x01R`\x04\x83\x01T\x82\x16`\x80\x82\x01R`\x05\x90\x92\x01T\x16`\xA0\x82\x01R\x85a\x06\xDBV[\x90P`\x01`\x01`\xA0\x1B\x03\x87\x16a\x03SW\x80Q`@Qc\x18\x9A\xCD\xBD`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xC4\xD6m\xE8\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x038W_\x80\xFD[PZ\xF1\x15\x80\x15a\x03JW=_\x80>=_\xFD[PPPPa\x03\xCEV[\x80Q`@Q\x7FH\\\xC9U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x90\x91\x16\x90cH\\\xC9U\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03\xB7W_\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xC9W=_\x80>=_\xFD[PPPP[`@\x80\x82\x01Q\x82Q\x91Q\x7F\x1A\xD8~E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x1A\xD8~E\x91a\x04\x1E\x91\x90\x8A\x90\x8A\x90`\x04\x01a\x0B\x99V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x045W_\x80\xFD[PZ\xF1\x15\x80\x15a\x04GW=_\x80>=_\xFD[PPPP` \x81\x01Q\x81Q`@\x80\x84\x01Q\x90Q\x7FH\\\xC9U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R\x91\x16\x90cH\\\xC9U\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04\xB8W_\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xCAW=_\x80>=_\xFD[PPPP``\x81\x01Q\x81Q`@Qc\x18\x9A\xCD\xBD`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90c\xC4\xD6m\xE8\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\x15W_\x80\xFD[PZ\xF1\x15\x80\x15a\x05'W=_\x80>=_\xFD[PPPP`\x80\x81\x01Q\x81Q`@Qc\x18\x9A\xCD\xBD`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90c\xC4\xD6m\xE8\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05rW_\x80\xFD[PZ\xF1\x15\x80\x15a\x05\x84W=_\x80>=_\xFD[P\x92\x9B\x9APPPPPPPPPPPV[a\x05\x9Da\tDV[a\x05\xA6_a\t\xB7V[V[a\x05\xB0a\tDV[\x80`\x07a\x05\xBD\x82\x82a\x0C\x1AV[PP`@Q\x7F\xA4t4\xBBm\x1D\xDDU!\xE8\x98\r\xEDjx5\x13\xE1Y\xF8\x047\xD7\x87\x15\xB1\x0En\x8Bk\xBA^\x90_\x90\xA1PV[a\x05\xF2a\tDV[\x80`\x01a\x05\xFF\x82\x82a\x0C\x1AV[PP`@Q\x7F\xC9\xD3\x94}\"\xFA\x12J\xAE\xC4\xC7\xE8\xC9\x19\xF7\x90\x16\xE2\xD7\xB4\x8E\xEE\x10V\x83u\xD9\x8B\x86F\r\x1B\x90_\x90\xA1PV[a\x064a\tDV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06\xD8\x81a\t\xB7V[PV[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x85\x84_\x01Q\x86`@Qa\x07A\x90a\n\x13V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\x07\x83W=_\x80>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x16\x81R\x85\x83a\x07\x9FW\x84` \x01Qa\x07\xA5V[\x84`@\x01Q[\x86`@Qa\x07\xB2\x90a\n\x13V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\x07\xF4W=_\x80>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`@\x80\x83\x01\x91\x90\x91R``\x85\x01Q\x90Q\x87\x91\x90\x87\x90a\x08\x1C\x90a\n\x13V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\x08^W=_\x80>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`\x80\x84\x01Q`@Q\x87\x91\x90\x87\x90a\x08\x83\x90a\n\x13V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\x08\xC5W=_\x80>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\xA0\x84\x01Q`@Q\x87\x91\x90\x87\x90a\x08\xEA\x90a\n\x13V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\t,W=_\x80>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\x80\x82\x01R\x95\x94PPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\xA6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\xC6V[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\r\x89\x80a\rn\x839\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xD8W_\x80\xFD[_\x80_\x80_\x85\x87\x03a\x01@\x81\x12\x15a\nJW_\x80\xFD[\x865a\nU\x81a\n V[\x95P` \x87\x015a\ne\x81a\n V[\x94P`@\x87\x015a\nu\x81a\n V[\x93P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x82\x01\x12\x15a\n\xA6W_\x80\xFD[``\x87\x01\x92P``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF \x82\x01\x12\x15a\n\xDBW_\x80\xFD[P`\xE0\x86\x01\x90P\x92\x95P\x92\x95\x90\x93PV[_`\xC0\x82\x84\x03\x12\x15a\n\xFCW_\x80\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15a\x0B\x12W_\x80\xFD[\x815a\x0B\x1D\x81a\n V[\x93\x92PPPV[`@\x81R\x82`@\x82\x01R\x82\x84``\x83\x017_``\x84\x83\x01\x01R_```\x1F\x19`\x1F\x86\x01\x16\x83\x01\x01\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x94\x93PPPPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0B{W_\x80\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x0B\x90W_\x80\xFD[a\x0B\x1D\x82a\x0BdV[_a\x01\0\x82\x01\x90P`\x01`\x01`\xA0\x1B\x03\x85\x16\x82R\x835` \x83\x01R` \x84\x015`@\x83\x01R`@\x84\x015``\x83\x01R``\x84\x015`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a\x0B\xE5\x85a\x0BdV[\x16`\xA0\x84\x01R\x80a\x0B\xF8` \x86\x01a\x0BdV[\x16`\xC0\x84\x01R\x80a\x0C\x0B`@\x86\x01a\x0BdV[\x16`\xE0\x84\x01RP\x94\x93PPPPV[\x815a\x0C%\x81a\n V[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x82\x16\x17\x82UP` \x82\x015a\x0CZ\x81a\n V[`\x01\x82\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UP`@\x82\x015a\x0C\x93\x81a\n V[`\x02\x82\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UP``\x82\x015a\x0C\xCC\x81a\n V[`\x03\x82\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UP`\x80\x82\x015a\r\x05\x81a\n V[`\x04\x82\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UP`\xA0\x82\x015a\r>\x81a\n V[`\x05\x82\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UPPPV\xFE`\x80`@R`@Qa\r\x898\x03\x80a\r\x89\x839\x81\x01`@\x81\x90Ra\0\"\x91a\x03\xB7V[\x82\x81a\0/\x82\x82_a\0CV[Pa\0;\x90P\x82a\0nV[PPPa\x04\xCDV[a\0L\x83a\0\xDBV[_\x82Q\x11\x80a\0XWP\x80[\x15a\0iWa\0g\x83\x83a\x01\x1AV[P[PPPV[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fa\0\xAD_\x80Q` a\rB\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x84\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1a\0\xD8\x81a\x01FV[PV[a\0\xE4\x81a\x01\xE1V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2PV[``a\x01?\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\rb`'\x919a\x02uV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC1967: new admin is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x80_\x80Q` a\rB\x839\x81Q\x91R[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x02NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01a\x01\xA7V[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x01\xC0V[```\x01`\x01`\xA0\x1B\x03\x84\x16;a\x02\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: delegate call to non-co`D\x82\x01Re\x1B\x9D\x1C\x98X\xDD`\xD2\x1B`d\x82\x01R`\x84\x01a\x01\xA7V[_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x02\xF7\x91\x90a\x04\x82V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x03/W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x034V[``\x91P[P\x90\x92P\x90Pa\x03E\x82\x82\x86a\x03OV[\x96\x95PPPPPPV[``\x83\x15a\x03^WP\x81a\x01?V[\x82Q\x15a\x03nW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA7\x91\x90a\x04\x98V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x9EW_\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80_``\x84\x86\x03\x12\x15a\x03\xC9W_\x80\xFD[a\x03\xD2\x84a\x03\x88V[\x92Pa\x03\xE0` \x85\x01a\x03\x88V[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x03\xFCW_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x04\x0FW_\x80\xFD[\x81Q\x81\x81\x11\x15a\x04!Wa\x04!a\x03\xA3V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x04IWa\x04Ia\x03\xA3V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x04aW_\x80\xFD[\x82` \x86\x01` \x83\x01^_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[a\x08h\x80a\x04\xDA_9_\xF3\xFE`\x80`@R`\x046\x10a\0]W_5`\xE0\x1C\x80c\\`\xDA\x1B\x11a\0BW\x80c\\`\xDA\x1B\x14a\0\xA6W\x80c\x8F(9p\x14a\0\xD6W\x80c\xF8Q\xA4@\x14a\0\xF5Wa\0lV[\x80c6Y\xCF\xE6\x14a\0tW\x80cO\x1E\xF2\x86\x14a\0\x93Wa\0lV[6a\0lWa\0ja\x01\tV[\0[a\0ja\x01\tV[4\x80\x15a\0\x7FW_\x80\xFD[Pa\0ja\0\x8E6`\x04a\x07\rV[a\x01#V[a\0ja\0\xA16`\x04a\x07&V[a\x01^V[4\x80\x15a\0\xB1W_\x80\xFD[Pa\0\xBAa\x01\xC4V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xE1W_\x80\xFD[Pa\0ja\0\xF06`\x04a\x07\rV[a\x01\xF4V[4\x80\x15a\x01\0W_\x80\xFD[Pa\0\xBAa\x02\x14V[a\x01\x11a\x024V[a\x01!a\x01\x1Ca\x02\xE4V[a\x02\xEDV[V[a\x01+a\x03\x0BV[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01VWa\x01S\x81`@Q\x80` \x01`@R\x80_\x81RP_a\x03=V[PV[a\x01Sa\x01\tV[a\x01fa\x03\x0BV[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01\xBCWa\x01\xB7\x83\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP`\x01\x92Pa\x03=\x91PPV[PPPV[a\x01\xB7a\x01\tV[_a\x01\xCDa\x03\x0BV[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01\xE9Wa\x01\xE4a\x02\xE4V[\x90P\x90V[a\x01\xF1a\x01\tV[\x90V[a\x01\xFCa\x03\x0BV[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01VWa\x01S\x81a\x03gV[_a\x02\x1Da\x03\x0BV[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01\xE9Wa\x01\xE4a\x03\x0BV[a\x02<a\x03\x0BV[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FTransparentUpgradeableProxy: adm`D\x82\x01R\x7Fin cannot fallback to proxy targ`d\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[_a\x01\xE4a\x03\xBBV[6_\x807_\x806_\x84Z\xF4=_\x80>\x80\x80\x15a\x03\x07W=_\xF3[=_\xFD[_\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90PV[a\x03F\x83a\x03\xE2V[_\x82Q\x11\x80a\x03RWP\x80[\x15a\x01\xB7Wa\x03a\x83\x83a\x04!V[PPPPV[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fa\x03\x90a\x03\x0BV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x84\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1a\x01S\x81a\x04MV[_\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x03.V[a\x03\xEB\x81a\x05%V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2PV[``a\x04F\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x08\x0C`'\x919a\x05\xC9V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC1967: new admin is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xDBV[\x80\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x05\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01R\x7Fot a contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xDBV[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x04\xECV[```\x01`\x01`\xA0\x1B\x03\x84\x16;a\x06HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: delegate call to non-co`D\x82\x01R\x7Fntract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xDBV[_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x06b\x91\x90a\x07\xA2V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x06\x9AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06\x9FV[``\x91P[P\x91P\x91Pa\x06\xAF\x82\x82\x86a\x06\xB9V[\x96\x95PPPPPPV[``\x83\x15a\x06\xC8WP\x81a\x04FV[\x82Q\x15a\x06\xD8W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xDB\x91\x90a\x07\xB8V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x08W_\x80\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x07\x1DW_\x80\xFD[a\x04F\x82a\x06\xF2V[_\x80_`@\x84\x86\x03\x12\x15a\x078W_\x80\xFD[a\x07A\x84a\x06\xF2V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07]W_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x07pW_\x80\xFD[\x815\x81\x81\x11\x15a\x07~W_\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x07\x8FW_\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 QG\xD3\x89W\xE9\xDFn\x1D\xA1\xB9\x87Q\x98|U\xE3+\xDCL\xB0\t\xEAz\x14Z\xB64\xAE\xE1\x89sdsolcC\0\x08\x19\x003\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03Address: low-level delegate call failed\xA2dipfsX\"\x12 \xD3\xD3\x06\x07\xB4?\xDAhW\x96}\xFF(\x9B\x10<\xC8\x08\xD9\xA1\xBA\xC2\xAB\x84\xFC\xF6t\xEFAT\xD4rdsolcC\0\x08\x19\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610085575f3560e01c80638da5cb5b116100585780638da5cb5b146101a0578063ceab9410146101ba578063e83b0e16146101cd578063f2fde38b146101e0575f80fd5b806311f022271461008957806357d3a20014610102578063715018a61461016457806376768ab91461016e575b5f80fd5b6001546002546003546004546005546006546100bb956001600160a01b03908116958116948116938116928116911686565b604080516001600160a01b03978816815295871660208701529386169385019390935290841660608401528316608083015290911660a082015260c0015b60405180910390f35b610115610110366004610a34565b6101f3565b6040516100f9919081516001600160a01b039081168252602080840151821690830152604080840151821690830152606080840151821690830152608092830151169181019190915260a00190565b61016c610595565b005b600754600854600954600a54600b54600c546100bb956001600160a01b03908116958116948116938116928116911686565b5f546040516001600160a01b0390911681526020016100f9565b61016c6101c8366004610aec565b6105a8565b61016c6101db366004610aec565b6105ea565b61016c6101ee366004610b02565b61062c565b6040805160a0810182525f808252602082018190529181018290526060810182905260808101919091525f80363360405160200161023393929190610b24565b60408051601f19818403018152919052805160209182012091505f9061025b90850185610b80565b67ffffffffffffffff16151590505f6102e5838a6001600160a01b038a1615610285576007610288565b60015b6040805160c08101825282546001600160a01b03908116825260018401548116602083015260028401548116928201929092526003830154821660608201526004830154821660808201526005909201541660a0820152856106db565b90506001600160a01b03871661035357805160405163189acdbd60e31b81526001600160a01b038a811660048301529091169063c4d66de8906024015f604051808303815f87803b158015610338575f80fd5b505af115801561034a573d5f803e3d5ffd5b505050506103ce565b80516040517f485cc9550000000000000000000000000000000000000000000000000000000081526001600160a01b038a8116600483015289811660248301529091169063485cc955906044015f604051808303815f87803b1580156103b7575f80fd5b505af11580156103c9573d5f803e3d5ffd5b505050505b604080820151825191517f1ad87e450000000000000000000000000000000000000000000000000000000081526001600160a01b0390911691631ad87e459161041e91908a908a90600401610b99565b5f604051808303815f87803b158015610435575f80fd5b505af1158015610447573d5f803e3d5ffd5b505050506020810151815160408084015190517f485cc9550000000000000000000000000000000000000000000000000000000081526001600160a01b039283166004820152908216602482015291169063485cc955906044015f604051808303815f87803b1580156104b8575f80fd5b505af11580156104ca573d5f803e3d5ffd5b505050506060810151815160405163189acdbd60e31b81526001600160a01b03918216600482015291169063c4d66de8906024015f604051808303815f87803b158015610515575f80fd5b505af1158015610527573d5f803e3d5ffd5b505050506080810151815160405163189acdbd60e31b81526001600160a01b03918216600482015291169063c4d66de8906024015f604051808303815f87803b158015610572575f80fd5b505af1158015610584573d5f803e3d5ffd5b50929b9a5050505050505050505050565b61059d610944565b6105a65f6109b7565b565b6105b0610944565b8060076105bd8282610c1a565b50506040517fa47434bb6d1ddd5521e8980ded6a783513e159f80437d78715b10e6e8b6bba5e905f90a150565b6105f2610944565b8060016105ff8282610c1a565b50506040517fc9d3947d22fa124aaec4c7e8c919f79016e2d7b48eee10568375d98b86460d1b905f90a150565b610634610944565b6001600160a01b0381166106cf576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f646472657373000000000000000000000000000000000000000000000000000060648201526084015b60405180910390fd5b6106d8816109b7565b50565b6040805160a0810182525f808252602082018190529181018290526060810182905260808101919091526040805160a0810182525f8082526020820181905291810182905260608101829052608081019190915285845f01518660405161074190610a13565b6001600160a01b039283168152911660208201526060604082018190525f908201526080018190604051809103905ff5905080158015610783573d5f803e3d5ffd5b506001600160a01b03168152858361079f5784602001516107a5565b84604001515b866040516107b290610a13565b6001600160a01b039283168152911660208201526060604082018190525f908201526080018190604051809103905ff59050801580156107f4573d5f803e3d5ffd5b506001600160a01b031660408083019190915260608501519051879190879061081c90610a13565b6001600160a01b039283168152911660208201526060604082018190525f908201526080018190604051809103905ff590508015801561085e573d5f803e3d5ffd5b506001600160a01b031660208201526080840151604051879190879061088390610a13565b6001600160a01b039283168152911660208201526060604082018190525f908201526080018190604051809103905ff59050801580156108c5573d5f803e3d5ffd5b506001600160a01b0316606082015260a084015160405187919087906108ea90610a13565b6001600160a01b039283168152911660208201526060604082018190525f908201526080018190604051809103905ff590508015801561092c573d5f803e3d5ffd5b506001600160a01b0316608082015295945050505050565b5f546001600160a01b031633146105a6576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016106c6565b5f80546001600160a01b0383811673ffffffffffffffffffffffffffffffffffffffff19831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b610d8980610d6e83390190565b6001600160a01b03811681146106d8575f80fd5b5f805f805f858703610140811215610a4a575f80fd5b8635610a5581610a20565b95506020870135610a6581610a20565b94506040870135610a7581610a20565b935060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa082011215610aa6575f80fd5b60608701925060607fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2082011215610adb575f80fd5b5060e0860190509295509295909350565b5f60c08284031215610afc575f80fd5b50919050565b5f60208284031215610b12575f80fd5b8135610b1d81610a20565b9392505050565b60408152826040820152828460608301375f606084830101525f6060601f19601f86011683010190506001600160a01b0383166020830152949350505050565b803567ffffffffffffffff81168114610b7b575f80fd5b919050565b5f60208284031215610b90575f80fd5b610b1d82610b64565b5f610100820190506001600160a01b03851682528335602083015260208401356040830152604084013560608301526060840135608083015267ffffffffffffffff80610be585610b64565b1660a084015280610bf860208601610b64565b1660c084015280610c0b60408601610b64565b1660e084015250949350505050565b8135610c2581610a20565b815473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b038216178255506020820135610c5a81610a20565b60018201805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b038316179055506040820135610c9381610a20565b60028201805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b038316179055506060820135610ccc81610a20565b60038201805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b038316179055506080820135610d0581610a20565b60048201805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b0383161790555060a0820135610d3e81610a20565b60058201805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b03831617905550505056fe6080604052604051610d89380380610d89833981016040819052610022916103b7565b828161002f82825f610043565b5061003b90508261006e565b5050506104cd565b61004c836100db565b5f825111806100585750805b1561006957610067838361011a565b505b505050565b7f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f6100ad5f80516020610d42833981519152546001600160a01b031690565b604080516001600160a01b03928316815291841660208301520160405180910390a16100d881610146565b50565b6100e4816101e1565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a250565b606061013f8383604051806060016040528060278152602001610d6260279139610275565b9392505050565b6001600160a01b0381166101b05760405162461bcd60e51b815260206004820152602660248201527f455243313936373a206e65772061646d696e20697320746865207a65726f206160448201526564647265737360d01b60648201526084015b60405180910390fd5b805f80516020610d428339815191525b80546001600160a01b0319166001600160a01b039290921691909117905550565b6001600160a01b0381163b61024e5760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201526c1bdd08184818dbdb9d1c9858dd609a1b60648201526084016101a7565b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc6101c0565b60606001600160a01b0384163b6102dd5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f6044820152651b9d1c9858dd60d21b60648201526084016101a7565b5f80856001600160a01b0316856040516102f79190610482565b5f60405180830381855af49150503d805f811461032f576040519150601f19603f3d011682016040523d82523d5f602084013e610334565b606091505b50909250905061034582828661034f565b9695505050505050565b6060831561035e57508161013f565b82511561036e5782518084602001fd5b8160405162461bcd60e51b81526004016101a79190610498565b80516001600160a01b038116811461039e575f80fd5b919050565b634e487b7160e01b5f52604160045260245ffd5b5f805f606084860312156103c9575f80fd5b6103d284610388565b92506103e060208501610388565b60408501519092506001600160401b03808211156103fc575f80fd5b818601915086601f83011261040f575f80fd5b815181811115610421576104216103a3565b604051601f8201601f19908116603f01168101908382118183101715610449576104496103a3565b81604052828152896020848701011115610461575f80fd5b8260208601602083015e5f6020848301015280955050505050509250925092565b5f82518060208501845e5f920191825250919050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b610868806104da5f395ff3fe60806040526004361061005d575f3560e01c80635c60da1b116100425780635c60da1b146100a65780638f283970146100d6578063f851a440146100f55761006c565b80633659cfe6146100745780634f1ef286146100935761006c565b3661006c5761006a610109565b005b61006a610109565b34801561007f575f80fd5b5061006a61008e36600461070d565b610123565b61006a6100a1366004610726565b61015e565b3480156100b1575f80fd5b506100ba6101c4565b6040516001600160a01b03909116815260200160405180910390f35b3480156100e1575f80fd5b5061006a6100f036600461070d565b6101f4565b348015610100575f80fd5b506100ba610214565b610111610234565b61012161011c6102e4565b6102ed565b565b61012b61030b565b6001600160a01b03163303610156576101538160405180602001604052805f8152505f61033d565b50565b610153610109565b61016661030b565b6001600160a01b031633036101bc576101b78383838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152506001925061033d915050565b505050565b6101b7610109565b5f6101cd61030b565b6001600160a01b031633036101e9576101e46102e4565b905090565b6101f1610109565b90565b6101fc61030b565b6001600160a01b031633036101565761015381610367565b5f61021d61030b565b6001600160a01b031633036101e9576101e461030b565b61023c61030b565b6001600160a01b031633036101215760405162461bcd60e51b815260206004820152604260248201527f5472616e73706172656e745570677261646561626c6550726f78793a2061646d60448201527f696e2063616e6e6f742066616c6c6261636b20746f2070726f7879207461726760648201527f6574000000000000000000000000000000000000000000000000000000000000608482015260a4015b60405180910390fd5b5f6101e46103bb565b365f80375f80365f845af43d5f803e808015610307573d5ff35b3d5ffd5b5f7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035b546001600160a01b0316919050565b610346836103e2565b5f825111806103525750805b156101b7576103618383610421565b50505050565b7f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f61039061030b565b604080516001600160a01b03928316815291841660208301520160405180910390a16101538161044d565b5f7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc61032e565b6103eb81610525565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a250565b6060610446838360405180606001604052806027815260200161080c602791396105c9565b9392505050565b6001600160a01b0381166104c95760405162461bcd60e51b815260206004820152602660248201527f455243313936373a206e65772061646d696e20697320746865207a65726f206160448201527f646472657373000000000000000000000000000000000000000000000000000060648201526084016102db565b807fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035b80547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b039290921691909117905550565b6001600160a01b0381163b6105a25760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201527f6f74206120636f6e74726163740000000000000000000000000000000000000060648201526084016102db565b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc6104ec565b60606001600160a01b0384163b6106485760405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f60448201527f6e7472616374000000000000000000000000000000000000000000000000000060648201526084016102db565b5f80856001600160a01b03168560405161066291906107a2565b5f60405180830381855af49150503d805f811461069a576040519150601f19603f3d011682016040523d82523d5f602084013e61069f565b606091505b50915091506106af8282866106b9565b9695505050505050565b606083156106c8575081610446565b8251156106d85782518084602001fd5b8160405162461bcd60e51b81526004016102db91906107b8565b80356001600160a01b0381168114610708575f80fd5b919050565b5f6020828403121561071d575f80fd5b610446826106f2565b5f805f60408486031215610738575f80fd5b610741846106f2565b9250602084013567ffffffffffffffff8082111561075d575f80fd5b818601915086601f830112610770575f80fd5b81358181111561077e575f80fd5b87602082850101111561078f575f80fd5b6020830194508093505050509250925092565b5f82518060208501845e5f920191825250919050565b602081525f82518060208401528060208501604085015e5f6040828501015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8301168401019150509291505056fe416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a26469706673582212205147d38957e9df6e1da1b98751987c55e32bdc4cb009ea7a145ab634aee1897364736f6c63430008190033b53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a2646970667358221220d3d30607b43fda6857967dff289b103cc808d9a1bac2ab84fcf674ef4154d47264736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\x85W_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0XW\x80c\x8D\xA5\xCB[\x14a\x01\xA0W\x80c\xCE\xAB\x94\x10\x14a\x01\xBAW\x80c\xE8;\x0E\x16\x14a\x01\xCDW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xE0W_\x80\xFD[\x80c\x11\xF0\"'\x14a\0\x89W\x80cW\xD3\xA2\0\x14a\x01\x02W\x80cqP\x18\xA6\x14a\x01dW\x80cvv\x8A\xB9\x14a\x01nW[_\x80\xFD[`\x01T`\x02T`\x03T`\x04T`\x05T`\x06Ta\0\xBB\x95`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95\x81\x16\x94\x81\x16\x93\x81\x16\x92\x81\x16\x91\x16\x86V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x81R\x95\x87\x16` \x87\x01R\x93\x86\x16\x93\x85\x01\x93\x90\x93R\x90\x84\x16``\x84\x01R\x83\x16`\x80\x83\x01R\x90\x91\x16`\xA0\x82\x01R`\xC0\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x15a\x01\x106`\x04a\n4V[a\x01\xF3V[`@Qa\0\xF9\x91\x90\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x84\x01Q\x82\x16\x90\x83\x01R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x82\x16\x90\x83\x01R`\x80\x92\x83\x01Q\x16\x91\x81\x01\x91\x90\x91R`\xA0\x01\x90V[a\x01la\x05\x95V[\0[`\x07T`\x08T`\tT`\nT`\x0BT`\x0CTa\0\xBB\x95`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95\x81\x16\x94\x81\x16\x93\x81\x16\x92\x81\x16\x91\x16\x86V[_T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF9V[a\x01la\x01\xC86`\x04a\n\xECV[a\x05\xA8V[a\x01la\x01\xDB6`\x04a\n\xECV[a\x05\xEAV[a\x01la\x01\xEE6`\x04a\x0B\x02V[a\x06,V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R_\x8063`@Q` \x01a\x023\x93\x92\x91\x90a\x0B$V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x91P_\x90a\x02[\x90\x85\x01\x85a\x0B\x80V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x90P_a\x02\xE5\x83\x8A`\x01`\x01`\xA0\x1B\x03\x8A\x16\x15a\x02\x85W`\x07a\x02\x88V[`\x01[`@\x80Q`\xC0\x81\x01\x82R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x84\x01T\x81\x16` \x83\x01R`\x02\x84\x01T\x81\x16\x92\x82\x01\x92\x90\x92R`\x03\x83\x01T\x82\x16``\x82\x01R`\x04\x83\x01T\x82\x16`\x80\x82\x01R`\x05\x90\x92\x01T\x16`\xA0\x82\x01R\x85a\x06\xDBV[\x90P`\x01`\x01`\xA0\x1B\x03\x87\x16a\x03SW\x80Q`@Qc\x18\x9A\xCD\xBD`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xC4\xD6m\xE8\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x038W_\x80\xFD[PZ\xF1\x15\x80\x15a\x03JW=_\x80>=_\xFD[PPPPa\x03\xCEV[\x80Q`@Q\x7FH\\\xC9U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x90\x91\x16\x90cH\\\xC9U\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03\xB7W_\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xC9W=_\x80>=_\xFD[PPPP[`@\x80\x82\x01Q\x82Q\x91Q\x7F\x1A\xD8~E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x1A\xD8~E\x91a\x04\x1E\x91\x90\x8A\x90\x8A\x90`\x04\x01a\x0B\x99V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x045W_\x80\xFD[PZ\xF1\x15\x80\x15a\x04GW=_\x80>=_\xFD[PPPP` \x81\x01Q\x81Q`@\x80\x84\x01Q\x90Q\x7FH\\\xC9U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R\x91\x16\x90cH\\\xC9U\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04\xB8W_\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xCAW=_\x80>=_\xFD[PPPP``\x81\x01Q\x81Q`@Qc\x18\x9A\xCD\xBD`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90c\xC4\xD6m\xE8\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\x15W_\x80\xFD[PZ\xF1\x15\x80\x15a\x05'W=_\x80>=_\xFD[PPPP`\x80\x81\x01Q\x81Q`@Qc\x18\x9A\xCD\xBD`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90c\xC4\xD6m\xE8\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05rW_\x80\xFD[PZ\xF1\x15\x80\x15a\x05\x84W=_\x80>=_\xFD[P\x92\x9B\x9APPPPPPPPPPPV[a\x05\x9Da\tDV[a\x05\xA6_a\t\xB7V[V[a\x05\xB0a\tDV[\x80`\x07a\x05\xBD\x82\x82a\x0C\x1AV[PP`@Q\x7F\xA4t4\xBBm\x1D\xDDU!\xE8\x98\r\xEDjx5\x13\xE1Y\xF8\x047\xD7\x87\x15\xB1\x0En\x8Bk\xBA^\x90_\x90\xA1PV[a\x05\xF2a\tDV[\x80`\x01a\x05\xFF\x82\x82a\x0C\x1AV[PP`@Q\x7F\xC9\xD3\x94}\"\xFA\x12J\xAE\xC4\xC7\xE8\xC9\x19\xF7\x90\x16\xE2\xD7\xB4\x8E\xEE\x10V\x83u\xD9\x8B\x86F\r\x1B\x90_\x90\xA1PV[a\x064a\tDV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06\xD8\x81a\t\xB7V[PV[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x85\x84_\x01Q\x86`@Qa\x07A\x90a\n\x13V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\x07\x83W=_\x80>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x16\x81R\x85\x83a\x07\x9FW\x84` \x01Qa\x07\xA5V[\x84`@\x01Q[\x86`@Qa\x07\xB2\x90a\n\x13V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\x07\xF4W=_\x80>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`@\x80\x83\x01\x91\x90\x91R``\x85\x01Q\x90Q\x87\x91\x90\x87\x90a\x08\x1C\x90a\n\x13V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\x08^W=_\x80>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`\x80\x84\x01Q`@Q\x87\x91\x90\x87\x90a\x08\x83\x90a\n\x13V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\x08\xC5W=_\x80>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\xA0\x84\x01Q`@Q\x87\x91\x90\x87\x90a\x08\xEA\x90a\n\x13V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\t,W=_\x80>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\x80\x82\x01R\x95\x94PPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\xA6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\xC6V[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\r\x89\x80a\rn\x839\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xD8W_\x80\xFD[_\x80_\x80_\x85\x87\x03a\x01@\x81\x12\x15a\nJW_\x80\xFD[\x865a\nU\x81a\n V[\x95P` \x87\x015a\ne\x81a\n V[\x94P`@\x87\x015a\nu\x81a\n V[\x93P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x82\x01\x12\x15a\n\xA6W_\x80\xFD[``\x87\x01\x92P``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF \x82\x01\x12\x15a\n\xDBW_\x80\xFD[P`\xE0\x86\x01\x90P\x92\x95P\x92\x95\x90\x93PV[_`\xC0\x82\x84\x03\x12\x15a\n\xFCW_\x80\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15a\x0B\x12W_\x80\xFD[\x815a\x0B\x1D\x81a\n V[\x93\x92PPPV[`@\x81R\x82`@\x82\x01R\x82\x84``\x83\x017_``\x84\x83\x01\x01R_```\x1F\x19`\x1F\x86\x01\x16\x83\x01\x01\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x94\x93PPPPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0B{W_\x80\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x0B\x90W_\x80\xFD[a\x0B\x1D\x82a\x0BdV[_a\x01\0\x82\x01\x90P`\x01`\x01`\xA0\x1B\x03\x85\x16\x82R\x835` \x83\x01R` \x84\x015`@\x83\x01R`@\x84\x015``\x83\x01R``\x84\x015`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a\x0B\xE5\x85a\x0BdV[\x16`\xA0\x84\x01R\x80a\x0B\xF8` \x86\x01a\x0BdV[\x16`\xC0\x84\x01R\x80a\x0C\x0B`@\x86\x01a\x0BdV[\x16`\xE0\x84\x01RP\x94\x93PPPPV[\x815a\x0C%\x81a\n V[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x82\x16\x17\x82UP` \x82\x015a\x0CZ\x81a\n V[`\x01\x82\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UP`@\x82\x015a\x0C\x93\x81a\n V[`\x02\x82\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UP``\x82\x015a\x0C\xCC\x81a\n V[`\x03\x82\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UP`\x80\x82\x015a\r\x05\x81a\n V[`\x04\x82\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UP`\xA0\x82\x015a\r>\x81a\n V[`\x05\x82\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UPPPV\xFE`\x80`@R`@Qa\r\x898\x03\x80a\r\x89\x839\x81\x01`@\x81\x90Ra\0\"\x91a\x03\xB7V[\x82\x81a\0/\x82\x82_a\0CV[Pa\0;\x90P\x82a\0nV[PPPa\x04\xCDV[a\0L\x83a\0\xDBV[_\x82Q\x11\x80a\0XWP\x80[\x15a\0iWa\0g\x83\x83a\x01\x1AV[P[PPPV[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fa\0\xAD_\x80Q` a\rB\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x84\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1a\0\xD8\x81a\x01FV[PV[a\0\xE4\x81a\x01\xE1V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2PV[``a\x01?\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\rb`'\x919a\x02uV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC1967: new admin is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x80_\x80Q` a\rB\x839\x81Q\x91R[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x02NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01a\x01\xA7V[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x01\xC0V[```\x01`\x01`\xA0\x1B\x03\x84\x16;a\x02\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: delegate call to non-co`D\x82\x01Re\x1B\x9D\x1C\x98X\xDD`\xD2\x1B`d\x82\x01R`\x84\x01a\x01\xA7V[_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x02\xF7\x91\x90a\x04\x82V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x03/W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x034V[``\x91P[P\x90\x92P\x90Pa\x03E\x82\x82\x86a\x03OV[\x96\x95PPPPPPV[``\x83\x15a\x03^WP\x81a\x01?V[\x82Q\x15a\x03nW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA7\x91\x90a\x04\x98V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x9EW_\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80_``\x84\x86\x03\x12\x15a\x03\xC9W_\x80\xFD[a\x03\xD2\x84a\x03\x88V[\x92Pa\x03\xE0` \x85\x01a\x03\x88V[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x03\xFCW_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x04\x0FW_\x80\xFD[\x81Q\x81\x81\x11\x15a\x04!Wa\x04!a\x03\xA3V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x04IWa\x04Ia\x03\xA3V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x04aW_\x80\xFD[\x82` \x86\x01` \x83\x01^_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[a\x08h\x80a\x04\xDA_9_\xF3\xFE`\x80`@R`\x046\x10a\0]W_5`\xE0\x1C\x80c\\`\xDA\x1B\x11a\0BW\x80c\\`\xDA\x1B\x14a\0\xA6W\x80c\x8F(9p\x14a\0\xD6W\x80c\xF8Q\xA4@\x14a\0\xF5Wa\0lV[\x80c6Y\xCF\xE6\x14a\0tW\x80cO\x1E\xF2\x86\x14a\0\x93Wa\0lV[6a\0lWa\0ja\x01\tV[\0[a\0ja\x01\tV[4\x80\x15a\0\x7FW_\x80\xFD[Pa\0ja\0\x8E6`\x04a\x07\rV[a\x01#V[a\0ja\0\xA16`\x04a\x07&V[a\x01^V[4\x80\x15a\0\xB1W_\x80\xFD[Pa\0\xBAa\x01\xC4V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xE1W_\x80\xFD[Pa\0ja\0\xF06`\x04a\x07\rV[a\x01\xF4V[4\x80\x15a\x01\0W_\x80\xFD[Pa\0\xBAa\x02\x14V[a\x01\x11a\x024V[a\x01!a\x01\x1Ca\x02\xE4V[a\x02\xEDV[V[a\x01+a\x03\x0BV[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01VWa\x01S\x81`@Q\x80` \x01`@R\x80_\x81RP_a\x03=V[PV[a\x01Sa\x01\tV[a\x01fa\x03\x0BV[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01\xBCWa\x01\xB7\x83\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP`\x01\x92Pa\x03=\x91PPV[PPPV[a\x01\xB7a\x01\tV[_a\x01\xCDa\x03\x0BV[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01\xE9Wa\x01\xE4a\x02\xE4V[\x90P\x90V[a\x01\xF1a\x01\tV[\x90V[a\x01\xFCa\x03\x0BV[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01VWa\x01S\x81a\x03gV[_a\x02\x1Da\x03\x0BV[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01\xE9Wa\x01\xE4a\x03\x0BV[a\x02<a\x03\x0BV[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FTransparentUpgradeableProxy: adm`D\x82\x01R\x7Fin cannot fallback to proxy targ`d\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[_a\x01\xE4a\x03\xBBV[6_\x807_\x806_\x84Z\xF4=_\x80>\x80\x80\x15a\x03\x07W=_\xF3[=_\xFD[_\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90PV[a\x03F\x83a\x03\xE2V[_\x82Q\x11\x80a\x03RWP\x80[\x15a\x01\xB7Wa\x03a\x83\x83a\x04!V[PPPPV[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fa\x03\x90a\x03\x0BV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x84\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1a\x01S\x81a\x04MV[_\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x03.V[a\x03\xEB\x81a\x05%V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2PV[``a\x04F\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x08\x0C`'\x919a\x05\xC9V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC1967: new admin is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xDBV[\x80\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x05\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01R\x7Fot a contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xDBV[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x04\xECV[```\x01`\x01`\xA0\x1B\x03\x84\x16;a\x06HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: delegate call to non-co`D\x82\x01R\x7Fntract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xDBV[_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x06b\x91\x90a\x07\xA2V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x06\x9AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06\x9FV[``\x91P[P\x91P\x91Pa\x06\xAF\x82\x82\x86a\x06\xB9V[\x96\x95PPPPPPV[``\x83\x15a\x06\xC8WP\x81a\x04FV[\x82Q\x15a\x06\xD8W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xDB\x91\x90a\x07\xB8V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x08W_\x80\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x07\x1DW_\x80\xFD[a\x04F\x82a\x06\xF2V[_\x80_`@\x84\x86\x03\x12\x15a\x078W_\x80\xFD[a\x07A\x84a\x06\xF2V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07]W_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x07pW_\x80\xFD[\x815\x81\x81\x11\x15a\x07~W_\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x07\x8FW_\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 QG\xD3\x89W\xE9\xDFn\x1D\xA1\xB9\x87Q\x98|U\xE3+\xDCL\xB0\t\xEAz\x14Z\xB64\xAE\xE1\x89sdsolcC\0\x08\x19\x003\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03Address: low-level delegate call failed\xA2dipfsX\"\x12 \xD3\xD3\x06\x07\xB4?\xDAhW\x96}\xFF(\x9B\x10<\xC8\x08\xD9\xA1\xBA\xC2\xAB\x84\xFC\xF6t\xEFAT\xD4rdsolcC\0\x08\x19\x003",
    );
    /**```solidity
struct BridgeContracts { address bridge; address inbox; address sequencerInbox; address rollupEventInbox; address outbox; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BridgeContracts {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub inbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sequencerInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub rollupEventInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub outbox: alloy::sol_types::private::Address,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<BridgeContracts> for UnderlyingRustTuple<'_> {
            fn from(value: BridgeContracts) -> Self {
                (
                    value.bridge,
                    value.inbox,
                    value.sequencerInbox,
                    value.rollupEventInbox,
                    value.outbox,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BridgeContracts {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bridge: tuple.0,
                    inbox: tuple.1,
                    sequencerInbox: tuple.2,
                    rollupEventInbox: tuple.3,
                    outbox: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BridgeContracts {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BridgeContracts {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.bridge,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.inbox,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sequencerInbox,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.rollupEventInbox,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.outbox,
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
        impl alloy_sol_types::SolType for BridgeContracts {
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
        impl alloy_sol_types::SolStruct for BridgeContracts {
            const NAME: &'static str = "BridgeContracts";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BridgeContracts(address bridge,address inbox,address sequencerInbox,address rollupEventInbox,address outbox)",
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
                            &self.inbox,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sequencerInbox,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rollupEventInbox,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.outbox,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BridgeContracts {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.bridge,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.inbox,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sequencerInbox,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rollupEventInbox,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.outbox,
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
                    &rust.inbox,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sequencerInbox,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rollupEventInbox,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.outbox,
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
struct BridgeTemplates { address bridge; address sequencerInbox; address delayBufferableSequencerInbox; address inbox; address rollupEventInbox; address outbox; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BridgeTemplates {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sequencerInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delayBufferableSequencerInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub inbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub rollupEventInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub outbox: alloy::sol_types::private::Address,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<BridgeTemplates> for UnderlyingRustTuple<'_> {
            fn from(value: BridgeTemplates) -> Self {
                (
                    value.bridge,
                    value.sequencerInbox,
                    value.delayBufferableSequencerInbox,
                    value.inbox,
                    value.rollupEventInbox,
                    value.outbox,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BridgeTemplates {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bridge: tuple.0,
                    sequencerInbox: tuple.1,
                    delayBufferableSequencerInbox: tuple.2,
                    inbox: tuple.3,
                    rollupEventInbox: tuple.4,
                    outbox: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BridgeTemplates {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BridgeTemplates {
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
                        &self.delayBufferableSequencerInbox,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.inbox,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.rollupEventInbox,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.outbox,
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
        impl alloy_sol_types::SolType for BridgeTemplates {
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
        impl alloy_sol_types::SolStruct for BridgeTemplates {
            const NAME: &'static str = "BridgeTemplates";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BridgeTemplates(address bridge,address sequencerInbox,address delayBufferableSequencerInbox,address inbox,address rollupEventInbox,address outbox)",
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
                            &self.delayBufferableSequencerInbox,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.inbox,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rollupEventInbox,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.outbox,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BridgeTemplates {
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
                        &rust.delayBufferableSequencerInbox,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.inbox,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rollupEventInbox,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.outbox,
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
                    &rust.delayBufferableSequencerInbox,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.inbox,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rollupEventInbox,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.outbox,
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
    /**Event with signature `ERC20TemplatesUpdated()` and selector `0xa47434bb6d1ddd5521e8980ded6a783513e159f80437d78715b10e6e8b6bba5e`.
```solidity
event ERC20TemplatesUpdated();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ERC20TemplatesUpdated {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ERC20TemplatesUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ERC20TemplatesUpdated()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8,
                116u8,
                52u8,
                187u8,
                109u8,
                29u8,
                221u8,
                85u8,
                33u8,
                232u8,
                152u8,
                13u8,
                237u8,
                106u8,
                120u8,
                53u8,
                19u8,
                225u8,
                89u8,
                248u8,
                4u8,
                55u8,
                215u8,
                135u8,
                21u8,
                177u8,
                14u8,
                110u8,
                139u8,
                107u8,
                186u8,
                94u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
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
        impl alloy_sol_types::private::IntoLogData for ERC20TemplatesUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ERC20TemplatesUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ERC20TemplatesUpdated) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `TemplatesUpdated()` and selector `0xc9d3947d22fa124aaec4c7e8c919f79016e2d7b48eee10568375d98b86460d1b`.
```solidity
event TemplatesUpdated();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TemplatesUpdated {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TemplatesUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TemplatesUpdated()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                201u8,
                211u8,
                148u8,
                125u8,
                34u8,
                250u8,
                18u8,
                74u8,
                174u8,
                196u8,
                199u8,
                232u8,
                201u8,
                25u8,
                247u8,
                144u8,
                22u8,
                226u8,
                215u8,
                180u8,
                142u8,
                238u8,
                16u8,
                86u8,
                131u8,
                117u8,
                217u8,
                139u8,
                134u8,
                70u8,
                13u8,
                27u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
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
        impl alloy_sol_types::private::IntoLogData for TemplatesUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TemplatesUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TemplatesUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(BridgeTemplates _ethBasedTemplates, BridgeTemplates _erc20BasedTemplates);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _ethBasedTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _erc20BasedTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BridgeTemplates, BridgeTemplates);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BridgeTemplates as alloy::sol_types::SolType>::RustType,
                <BridgeTemplates as alloy::sol_types::SolType>::RustType,
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
                    (value._ethBasedTemplates, value._erc20BasedTemplates)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _ethBasedTemplates: tuple.0,
                        _erc20BasedTemplates: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (BridgeTemplates, BridgeTemplates);
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
                    <BridgeTemplates as alloy_sol_types::SolType>::tokenize(
                        &self._ethBasedTemplates,
                    ),
                    <BridgeTemplates as alloy_sol_types::SolType>::tokenize(
                        &self._erc20BasedTemplates,
                    ),
                )
            }
        }
    };
    /**Function with signature `createBridge(address,address,address,(uint256,uint256,uint256,uint256),(uint64,uint64,uint64))` and selector `0x57d3a200`.
```solidity
function createBridge(address adminProxy, address rollup, address nativeToken, ISequencerInbox.MaxTimeVariation memory maxTimeVariation, BufferConfig memory bufferConfig) external returns (BridgeContracts memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createBridgeCall {
        #[allow(missing_docs)]
        pub adminProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub rollup: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub nativeToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub maxTimeVariation: <ISequencerInbox::MaxTimeVariation as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub bufferConfig: <BufferConfig as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`createBridge(address,address,address,(uint256,uint256,uint256,uint256),(uint64,uint64,uint64))`](createBridgeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createBridgeReturn {
        #[allow(missing_docs)]
        pub _0: <BridgeContracts as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Address,
                ISequencerInbox::MaxTimeVariation,
                BufferConfig,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                <ISequencerInbox::MaxTimeVariation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createBridgeCall> for UnderlyingRustTuple<'_> {
                fn from(value: createBridgeCall) -> Self {
                    (
                        value.adminProxy,
                        value.rollup,
                        value.nativeToken,
                        value.maxTimeVariation,
                        value.bufferConfig,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createBridgeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        adminProxy: tuple.0,
                        rollup: tuple.1,
                        nativeToken: tuple.2,
                        maxTimeVariation: tuple.3,
                        bufferConfig: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BridgeContracts,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BridgeContracts as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createBridgeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createBridgeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createBridgeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createBridgeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                ISequencerInbox::MaxTimeVariation,
                BufferConfig,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createBridgeReturn;
            type ReturnTuple<'a> = (BridgeContracts,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createBridge(address,address,address,(uint256,uint256,uint256,uint256),(uint64,uint64,uint64))";
            const SELECTOR: [u8; 4] = [87u8, 211u8, 162u8, 0u8];
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
                        &self.adminProxy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.rollup,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.nativeToken,
                    ),
                    <ISequencerInbox::MaxTimeVariation as alloy_sol_types::SolType>::tokenize(
                        &self.maxTimeVariation,
                    ),
                    <BufferConfig as alloy_sol_types::SolType>::tokenize(
                        &self.bufferConfig,
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
    /**Function with signature `erc20BasedTemplates()` and selector `0x76768ab9`.
```solidity
function erc20BasedTemplates() external view returns (address bridge, address sequencerInbox, address delayBufferableSequencerInbox, address inbox, address rollupEventInbox, address outbox);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct erc20BasedTemplatesCall {}
    ///Container type for the return parameters of the [`erc20BasedTemplates()`](erc20BasedTemplatesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct erc20BasedTemplatesReturn {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sequencerInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delayBufferableSequencerInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub inbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub rollupEventInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub outbox: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<erc20BasedTemplatesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: erc20BasedTemplatesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for erc20BasedTemplatesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
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
            impl ::core::convert::From<erc20BasedTemplatesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: erc20BasedTemplatesReturn) -> Self {
                    (
                        value.bridge,
                        value.sequencerInbox,
                        value.delayBufferableSequencerInbox,
                        value.inbox,
                        value.rollupEventInbox,
                        value.outbox,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for erc20BasedTemplatesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        bridge: tuple.0,
                        sequencerInbox: tuple.1,
                        delayBufferableSequencerInbox: tuple.2,
                        inbox: tuple.3,
                        rollupEventInbox: tuple.4,
                        outbox: tuple.5,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for erc20BasedTemplatesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = erc20BasedTemplatesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "erc20BasedTemplates()";
            const SELECTOR: [u8; 4] = [118u8, 118u8, 138u8, 185u8];
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
    /**Function with signature `ethBasedTemplates()` and selector `0x11f02227`.
```solidity
function ethBasedTemplates() external view returns (address bridge, address sequencerInbox, address delayBufferableSequencerInbox, address inbox, address rollupEventInbox, address outbox);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ethBasedTemplatesCall {}
    ///Container type for the return parameters of the [`ethBasedTemplates()`](ethBasedTemplatesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ethBasedTemplatesReturn {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sequencerInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delayBufferableSequencerInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub inbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub rollupEventInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub outbox: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<ethBasedTemplatesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ethBasedTemplatesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ethBasedTemplatesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
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
            impl ::core::convert::From<ethBasedTemplatesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ethBasedTemplatesReturn) -> Self {
                    (
                        value.bridge,
                        value.sequencerInbox,
                        value.delayBufferableSequencerInbox,
                        value.inbox,
                        value.rollupEventInbox,
                        value.outbox,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ethBasedTemplatesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        bridge: tuple.0,
                        sequencerInbox: tuple.1,
                        delayBufferableSequencerInbox: tuple.2,
                        inbox: tuple.3,
                        rollupEventInbox: tuple.4,
                        outbox: tuple.5,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ethBasedTemplatesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ethBasedTemplatesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ethBasedTemplates()";
            const SELECTOR: [u8; 4] = [17u8, 240u8, 34u8, 39u8];
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
    /**Function with signature `updateERC20Templates((address,address,address,address,address,address))` and selector `0xceab9410`.
```solidity
function updateERC20Templates(BridgeTemplates memory _newTemplates) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateERC20TemplatesCall {
        #[allow(missing_docs)]
        pub _newTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`updateERC20Templates((address,address,address,address,address,address))`](updateERC20TemplatesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateERC20TemplatesReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BridgeTemplates,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BridgeTemplates as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<updateERC20TemplatesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateERC20TemplatesCall) -> Self {
                    (value._newTemplates,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateERC20TemplatesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _newTemplates: tuple.0 }
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
            impl ::core::convert::From<updateERC20TemplatesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateERC20TemplatesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateERC20TemplatesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateERC20TemplatesCall {
            type Parameters<'a> = (BridgeTemplates,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateERC20TemplatesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateERC20Templates((address,address,address,address,address,address))";
            const SELECTOR: [u8; 4] = [206u8, 171u8, 148u8, 16u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <BridgeTemplates as alloy_sol_types::SolType>::tokenize(
                        &self._newTemplates,
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
    /**Function with signature `updateTemplates((address,address,address,address,address,address))` and selector `0xe83b0e16`.
```solidity
function updateTemplates(BridgeTemplates memory _newTemplates) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateTemplatesCall {
        #[allow(missing_docs)]
        pub _newTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`updateTemplates((address,address,address,address,address,address))`](updateTemplatesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateTemplatesReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BridgeTemplates,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BridgeTemplates as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<updateTemplatesCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateTemplatesCall) -> Self {
                    (value._newTemplates,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateTemplatesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _newTemplates: tuple.0 }
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
            impl ::core::convert::From<updateTemplatesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateTemplatesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateTemplatesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateTemplatesCall {
            type Parameters<'a> = (BridgeTemplates,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateTemplatesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateTemplates((address,address,address,address,address,address))";
            const SELECTOR: [u8; 4] = [232u8, 59u8, 14u8, 22u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <BridgeTemplates as alloy_sol_types::SolType>::tokenize(
                        &self._newTemplates,
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
    ///Container for all the [`BridgeCreator`](self) function calls.
    pub enum BridgeCreatorCalls {
        #[allow(missing_docs)]
        createBridge(createBridgeCall),
        #[allow(missing_docs)]
        erc20BasedTemplates(erc20BasedTemplatesCall),
        #[allow(missing_docs)]
        ethBasedTemplates(ethBasedTemplatesCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        updateERC20Templates(updateERC20TemplatesCall),
        #[allow(missing_docs)]
        updateTemplates(updateTemplatesCall),
    }
    #[automatically_derived]
    impl BridgeCreatorCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [17u8, 240u8, 34u8, 39u8],
            [87u8, 211u8, 162u8, 0u8],
            [113u8, 80u8, 24u8, 166u8],
            [118u8, 118u8, 138u8, 185u8],
            [141u8, 165u8, 203u8, 91u8],
            [206u8, 171u8, 148u8, 16u8],
            [232u8, 59u8, 14u8, 22u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for BridgeCreatorCalls {
        const NAME: &'static str = "BridgeCreatorCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 8usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::createBridge(_) => {
                    <createBridgeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::erc20BasedTemplates(_) => {
                    <erc20BasedTemplatesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ethBasedTemplates(_) => {
                    <ethBasedTemplatesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateERC20Templates(_) => {
                    <updateERC20TemplatesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateTemplates(_) => {
                    <updateTemplatesCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<BridgeCreatorCalls>] = &[
                {
                    fn ethBasedTemplates(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BridgeCreatorCalls> {
                        <ethBasedTemplatesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BridgeCreatorCalls::ethBasedTemplates)
                    }
                    ethBasedTemplates
                },
                {
                    fn createBridge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BridgeCreatorCalls> {
                        <createBridgeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BridgeCreatorCalls::createBridge)
                    }
                    createBridge
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BridgeCreatorCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BridgeCreatorCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn erc20BasedTemplates(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BridgeCreatorCalls> {
                        <erc20BasedTemplatesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BridgeCreatorCalls::erc20BasedTemplates)
                    }
                    erc20BasedTemplates
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BridgeCreatorCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BridgeCreatorCalls::owner)
                    }
                    owner
                },
                {
                    fn updateERC20Templates(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BridgeCreatorCalls> {
                        <updateERC20TemplatesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BridgeCreatorCalls::updateERC20Templates)
                    }
                    updateERC20Templates
                },
                {
                    fn updateTemplates(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BridgeCreatorCalls> {
                        <updateTemplatesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BridgeCreatorCalls::updateTemplates)
                    }
                    updateTemplates
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BridgeCreatorCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BridgeCreatorCalls::transferOwnership)
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
                Self::createBridge(inner) => {
                    <createBridgeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::erc20BasedTemplates(inner) => {
                    <erc20BasedTemplatesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ethBasedTemplates(inner) => {
                    <ethBasedTemplatesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::updateERC20Templates(inner) => {
                    <updateERC20TemplatesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateTemplates(inner) => {
                    <updateTemplatesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::createBridge(inner) => {
                    <createBridgeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::erc20BasedTemplates(inner) => {
                    <erc20BasedTemplatesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ethBasedTemplates(inner) => {
                    <ethBasedTemplatesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::updateERC20Templates(inner) => {
                    <updateERC20TemplatesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateTemplates(inner) => {
                    <updateTemplatesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`BridgeCreator`](self) events.
    pub enum BridgeCreatorEvents {
        #[allow(missing_docs)]
        ERC20TemplatesUpdated(ERC20TemplatesUpdated),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        TemplatesUpdated(TemplatesUpdated),
    }
    #[automatically_derived]
    impl BridgeCreatorEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
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
            [
                164u8,
                116u8,
                52u8,
                187u8,
                109u8,
                29u8,
                221u8,
                85u8,
                33u8,
                232u8,
                152u8,
                13u8,
                237u8,
                106u8,
                120u8,
                53u8,
                19u8,
                225u8,
                89u8,
                248u8,
                4u8,
                55u8,
                215u8,
                135u8,
                21u8,
                177u8,
                14u8,
                110u8,
                139u8,
                107u8,
                186u8,
                94u8,
            ],
            [
                201u8,
                211u8,
                148u8,
                125u8,
                34u8,
                250u8,
                18u8,
                74u8,
                174u8,
                196u8,
                199u8,
                232u8,
                201u8,
                25u8,
                247u8,
                144u8,
                22u8,
                226u8,
                215u8,
                180u8,
                142u8,
                238u8,
                16u8,
                86u8,
                131u8,
                117u8,
                217u8,
                139u8,
                134u8,
                70u8,
                13u8,
                27u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for BridgeCreatorEvents {
        const NAME: &'static str = "BridgeCreatorEvents";
        const COUNT: usize = 3usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <ERC20TemplatesUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ERC20TemplatesUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ERC20TemplatesUpdated)
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
                Some(<TemplatesUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TemplatesUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TemplatesUpdated)
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
    impl alloy_sol_types::private::IntoLogData for BridgeCreatorEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ERC20TemplatesUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TemplatesUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ERC20TemplatesUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TemplatesUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BridgeCreator`](self) contract instance.

See the [wrapper's documentation](`BridgeCreatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BridgeCreatorInstance<T, P, N> {
        BridgeCreatorInstance::<T, P, N>::new(address, provider)
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
        _ethBasedTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
        _erc20BasedTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<BridgeCreatorInstance<T, P, N>>,
    > {
        BridgeCreatorInstance::<
            T,
            P,
            N,
        >::deploy(provider, _ethBasedTemplates, _erc20BasedTemplates)
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
        _ethBasedTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
        _erc20BasedTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        BridgeCreatorInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _ethBasedTemplates, _erc20BasedTemplates)
    }
    /**A [`BridgeCreator`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BridgeCreator`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BridgeCreatorInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BridgeCreatorInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BridgeCreatorInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BridgeCreatorInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BridgeCreator`](self) contract instance.

See the [wrapper's documentation](`BridgeCreatorInstance`) for more details.*/
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
            _ethBasedTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
            _erc20BasedTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::Result<BridgeCreatorInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _ethBasedTemplates,
                _erc20BasedTemplates,
            );
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
            _ethBasedTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
            _erc20BasedTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _ethBasedTemplates,
                            _erc20BasedTemplates,
                        },
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
    impl<T, P: ::core::clone::Clone, N> BridgeCreatorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BridgeCreatorInstance<T, P, N> {
            BridgeCreatorInstance {
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
    > BridgeCreatorInstance<T, P, N> {
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
        ///Creates a new call builder for the [`createBridge`] function.
        pub fn createBridge(
            &self,
            adminProxy: alloy::sol_types::private::Address,
            rollup: alloy::sol_types::private::Address,
            nativeToken: alloy::sol_types::private::Address,
            maxTimeVariation: <ISequencerInbox::MaxTimeVariation as alloy::sol_types::SolType>::RustType,
            bufferConfig: <BufferConfig as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, createBridgeCall, N> {
            self.call_builder(
                &createBridgeCall {
                    adminProxy,
                    rollup,
                    nativeToken,
                    maxTimeVariation,
                    bufferConfig,
                },
            )
        }
        ///Creates a new call builder for the [`erc20BasedTemplates`] function.
        pub fn erc20BasedTemplates(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, erc20BasedTemplatesCall, N> {
            self.call_builder(&erc20BasedTemplatesCall {})
        }
        ///Creates a new call builder for the [`ethBasedTemplates`] function.
        pub fn ethBasedTemplates(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, ethBasedTemplatesCall, N> {
            self.call_builder(&ethBasedTemplatesCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
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
        ///Creates a new call builder for the [`updateERC20Templates`] function.
        pub fn updateERC20Templates(
            &self,
            _newTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateERC20TemplatesCall, N> {
            self.call_builder(
                &updateERC20TemplatesCall {
                    _newTemplates,
                },
            )
        }
        ///Creates a new call builder for the [`updateTemplates`] function.
        pub fn updateTemplates(
            &self,
            _newTemplates: <BridgeTemplates as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateTemplatesCall, N> {
            self.call_builder(
                &updateTemplatesCall {
                    _newTemplates,
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
    > BridgeCreatorInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ERC20TemplatesUpdated`] event.
        pub fn ERC20TemplatesUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ERC20TemplatesUpdated, N> {
            self.event_filter::<ERC20TemplatesUpdated>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`TemplatesUpdated`] event.
        pub fn TemplatesUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TemplatesUpdated, N> {
            self.event_filter::<TemplatesUpdated>()
        }
    }
}
