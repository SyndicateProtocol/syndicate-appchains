///Module containing a contract's types and functions.
/**

```solidity
library IBridge {
    type BatchDataLocation is uint8;
    struct TimeBounds { uint64 minTimestamp; uint64 maxTimestamp; uint64 minBlockNumber; uint64 maxBlockNumber; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IBridge {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BatchDataLocation(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<BatchDataLocation> for u8 {
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
        impl BatchDataLocation {
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
        impl alloy_sol_types::SolType for BatchDataLocation {
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
        impl alloy_sol_types::EventTopic for BatchDataLocation {
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
struct TimeBounds { uint64 minTimestamp; uint64 maxTimestamp; uint64 minBlockNumber; uint64 maxBlockNumber; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimeBounds {
        #[allow(missing_docs)]
        pub minTimestamp: u64,
        #[allow(missing_docs)]
        pub maxTimestamp: u64,
        #[allow(missing_docs)]
        pub minBlockNumber: u64,
        #[allow(missing_docs)]
        pub maxBlockNumber: u64,
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
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u64, u64, u64, u64);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TimeBounds> for UnderlyingRustTuple<'_> {
            fn from(value: TimeBounds) -> Self {
                (
                    value.minTimestamp,
                    value.maxTimestamp,
                    value.minBlockNumber,
                    value.maxBlockNumber,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TimeBounds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    minTimestamp: tuple.0,
                    maxTimestamp: tuple.1,
                    minBlockNumber: tuple.2,
                    maxBlockNumber: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TimeBounds {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TimeBounds {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.minTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.minBlockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxBlockNumber),
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
        impl alloy_sol_types::SolType for TimeBounds {
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
        impl alloy_sol_types::SolStruct for TimeBounds {
            const NAME: &'static str = "TimeBounds";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TimeBounds(uint64 minTimestamp,uint64 maxTimestamp,uint64 minBlockNumber,uint64 maxBlockNumber)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.minTimestamp)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxTimestamp)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.minBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.maxBlockNumber,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TimeBounds {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.minTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.minBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxBlockNumber,
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
                    &rust.minTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.minBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxBlockNumber,
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
    /**Creates a new wrapper around an on-chain [`IBridge`](self) contract instance.

See the [wrapper's documentation](`IBridgeInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IBridgeInstance<T, P, N> {
        IBridgeInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IBridge`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IBridge`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IBridgeInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IBridgeInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IBridgeInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IBridgeInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IBridge`](self) contract instance.

See the [wrapper's documentation](`IBridgeInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IBridgeInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IBridgeInstance<T, P, N> {
            IBridgeInstance {
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
    > IBridgeInstance<T, P, N> {
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
    > IBridgeInstance<T, P, N> {
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
library IBridge {
    type BatchDataLocation is uint8;
    struct TimeBounds {
        uint64 minTimestamp;
        uint64 maxTimestamp;
        uint64 minBlockNumber;
        uint64 maxBlockNumber;
    }
}

interface Rollup {
    error DataTooLarge(uint256 dataLength, uint256 maxDataLength);

    event InboxMessageDelivered(uint256 indexed messageNum, bytes data);
    event MessageDelivered(uint256 indexed messageIndex, bytes32 indexed beforeInboxAcc, address inbox, uint8 kind, address sender, bytes32 messageDataHash, uint256 baseFeeL1, uint64 timestamp);
    event SequencerBatchData(uint256 indexed batchSequenceNumber, bytes data);
    event SequencerBatchDelivered(uint256 indexed batchSequenceNumber, bytes32 indexed beforeAcc, bytes32 indexed afterAcc, bytes32 delayedAcc, uint256 afterDelayedMessagesRead, IBridge.TimeBounds timeBounds, IBridge.BatchDataLocation dataLocation);

    constructor(uint256 chainId, string chainConfig);

    function batchCount() external view returns (uint256);
    function delayBlocks() external view returns (uint64);
    function delaySeconds() external view returns (uint64);
    function delayedInboxAccs(uint256) external view returns (bytes32);
    function delayedMessageCount() external view returns (uint256);
    function deliverMessage(uint8 kind, address sender, bytes memory messageData) external;
    function depositEth(address src, address dest, uint256 value) external;
    function futureBlocks() external view returns (uint64);
    function futureSeconds() external view returns (uint64);
    function inboxAccs(uint256 index) external view returns (bytes32);
    function lastProcessedSequencingBlock() external view returns (uint256);
    function lastProcessedSettlementBlock() external view returns (uint256);
    function maxDataSize() external view returns (uint64);
    function postBatch(bytes memory data, uint256 _lastProcessedSequencingBlock, uint256 _lastProcessedSettlementBlock) external;
    function sequencerInboxAccs(uint256) external view returns (bytes32);
    function sequencerMessageCount() external view returns (uint256);
    function totalDelayedMessagesRead() external view returns (uint256);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "chainConfig",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "batchCount",
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
    "name": "delayBlocks",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delaySeconds",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delayedInboxAccs",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delayedMessageCount",
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
    "name": "deliverMessage",
    "inputs": [
      {
        "name": "kind",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "messageData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "depositEth",
    "inputs": [
      {
        "name": "src",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "dest",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "futureBlocks",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "futureSeconds",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "inboxAccs",
    "inputs": [
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lastProcessedSequencingBlock",
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
    "name": "lastProcessedSettlementBlock",
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
    "name": "maxDataSize",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "postBatch",
    "inputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "_lastProcessedSequencingBlock",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_lastProcessedSettlementBlock",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "sequencerInboxAccs",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "sequencerMessageCount",
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
    "name": "totalDelayedMessagesRead",
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
    "type": "event",
    "name": "InboxMessageDelivered",
    "inputs": [
      {
        "name": "messageNum",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
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
    "name": "MessageDelivered",
    "inputs": [
      {
        "name": "messageIndex",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "beforeInboxAcc",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "inbox",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "kind",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "messageDataHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "baseFeeL1",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "timestamp",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SequencerBatchData",
    "inputs": [
      {
        "name": "batchSequenceNumber",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
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
    "name": "SequencerBatchDelivered",
    "inputs": [
      {
        "name": "batchSequenceNumber",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "beforeAcc",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "afterAcc",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "delayedAcc",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "afterDelayedMessagesRead",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "timeBounds",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IBridge.TimeBounds",
        "components": [
          {
            "name": "minTimestamp",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "maxTimestamp",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "minBlockNumber",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "maxBlockNumber",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      },
      {
        "name": "dataLocation",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IBridge.BatchDataLocation"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "DataTooLarge",
    "inputs": [
      {
        "name": "dataLength",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "maxDataLength",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
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
pub mod Rollup {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60806040525f80555f600155348015610016575f80fd5b506040516123c63803806123c683398181016040528101906100389190610796565b5f81511161007b576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016100729061084a565b60405180910390fd5b5f600190505f4890505f8483838660405160200161009c949392919061090c565b60405160208183030381529060405290506100bf600b5f8361010e60201b60201c565b6101046040518060400160405280600681526020017e0b0080020300000000000000000000000000000000000000000000000000008152505f8061026460201b60201c565b5050505050610fd3565b5f60028054905090505f828051906020012090505f858543428648876040516020016101409796959493929190610a3a565b6040516020818303038152906040528051906020012090505f805f1b90505f8411156101925760026001856101759190610ae7565b8154811061018657610185610b1a565b5b905f5260205f20015490505b600281836040516020016101a7929190610b47565b60405160208183030381529060405280519060200120908060018154018082558091505060019003905f5260205f20015f909190919091505580847f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1308a8a88484260405161021b96959493929190610bbd565b60405180910390a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b866040516102539190610c6e565b60405180910390a250505050505050565b815f81905550806001819055505f60028054905090505f8061028c86846103ef60201b60201c565b915091505f60038054905090505f805f1b90505f8211156102d35760036001836102b69190610ae7565b815481106102c7576102c6610b1a565b5b905f5260205f20015490505b5f805f1b90505f86111561030d5760026001876102f09190610ae7565b8154811061030157610300610b1a565b5b905f5260205f20015490505b5f82868360405160200161032393929190610c8e565b604051602081830303815290604052805190602001209050600381908060018154018082558091505060019003905f5260205f20015f9091909190915055866004819055508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7856004548a60016040516103a39493929190610d9f565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208b6040516103db9190610c6e565b60405180910390a250505050505050505050565b5f6103f86105ca565b5f845160286104079190610de2565b90506201cccc67ffffffffffffffff1681111561046057806201cccc6040517f4634691b000000000000000000000000000000000000000000000000000000008152600401610457929190610e4e565b60405180910390fd5b6104686105ca565b6201518067ffffffffffffffff164211156104ab57620151804261048c9190610e75565b815f019067ffffffffffffffff16908167ffffffffffffffff16815250505b610e10426104b99190610eb0565b816020019067ffffffffffffffff16908167ffffffffffffffff1681525050611c2067ffffffffffffffff1643111561051a57611c20436104fa9190610e75565b816040019067ffffffffffffffff16908167ffffffffffffffff16815250505b600c436105279190610eb0565b816060019067ffffffffffffffff16908167ffffffffffffffff16815250505f815f01518260200151836040015184606001518960405160200161056f959493929190610eeb565b6040516020818303038152906040529050602881511461059257610591610f49565b5b80876040516020016105a5929190610fb0565b6040516020818303038152906040528051906020012082945094505050509250929050565b60405180608001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681525090565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b61063981610627565b8114610643575f80fd5b50565b5f8151905061065481610630565b92915050565b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6106a882610662565b810181811067ffffffffffffffff821117156106c7576106c6610672565b5b80604052505050565b5f6106d9610616565b90506106e5828261069f565b919050565b5f67ffffffffffffffff82111561070457610703610672565b5b61070d82610662565b9050602081019050919050565b8281835e5f83830152505050565b5f61073a610735846106ea565b6106d0565b9050828152602081018484840111156107565761075561065e565b5b61076184828561071a565b509392505050565b5f82601f83011261077d5761077c61065a565b5b815161078d848260208601610728565b91505092915050565b5f80604083850312156107ac576107ab61061f565b5b5f6107b985828601610646565b925050602083015167ffffffffffffffff8111156107da576107d9610623565b5b6107e685828601610769565b9150509250929050565b5f82825260208201905092915050565b7f454d5054595f434841494e5f434f4e46494700000000000000000000000000005f82015250565b5f6108346012836107f0565b915061083f82610800565b602082019050919050565b5f6020820190508181035f83015261086181610828565b9050919050565b5f819050919050565b61088261087d82610627565b610868565b82525050565b5f60ff82169050919050565b5f8160f81b9050919050565b5f6108aa82610894565b9050919050565b6108c26108bd82610888565b6108a0565b82525050565b5f81519050919050565b5f81905092915050565b5f6108e6826108c8565b6108f081856108d2565b935061090081856020860161071a565b80840191505092915050565b5f6109178287610871565b60208201915061092782866108b1565b6001820191506109378285610871565b60208201915061094782846108dc565b915081905095945050505050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61097e82610955565b9050919050565b5f8160601b9050919050565b5f61099b82610985565b9050919050565b5f6109ac82610991565b9050919050565b6109c46109bf82610974565b6109a2565b82525050565b5f67ffffffffffffffff82169050919050565b5f8160c01b9050919050565b5f6109f3826109dd565b9050919050565b610a0b610a06826109ca565b6109e9565b82525050565b5f819050919050565b5f819050919050565b610a34610a2f82610a11565b610a1a565b82525050565b5f610a45828a6108b1565b600182019150610a5582896109b3565b601482019150610a6582886109fa565b600882019150610a7582876109fa565b600882019150610a858286610871565b602082019150610a958285610871565b602082019150610aa58284610a23565b60208201915081905098975050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610af182610627565b9150610afc83610627565b9250828203905081811115610b1457610b13610aba565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f610b528285610a23565b602082019150610b628284610a23565b6020820191508190509392505050565b610b7b81610974565b82525050565b610b8a81610888565b82525050565b610b9981610a11565b82525050565b610ba881610627565b82525050565b610bb7816109ca565b82525050565b5f60c082019050610bd05f830189610b72565b610bdd6020830188610b81565b610bea6040830187610b72565b610bf76060830186610b90565b610c046080830185610b9f565b610c1160a0830184610bae565b979650505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f610c4082610c1c565b610c4a8185610c26565b9350610c5a81856020860161071a565b610c6381610662565b840191505092915050565b5f6020820190508181035f830152610c868184610c36565b905092915050565b5f610c998286610a23565b602082019150610ca98285610a23565b602082019150610cb98284610a23565b602082019150819050949350505050565b610cd3816109ca565b82525050565b608082015f820151610ced5f850182610cca565b506020820151610d006020850182610cca565b506040820151610d136040850182610cca565b506060820151610d266060850182610cca565b50505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60048110610d6a57610d69610d2c565b5b50565b5f819050610d7a82610d59565b919050565b5f610d8982610d6d565b9050919050565b610d9981610d7f565b82525050565b5f60e082019050610db25f830187610b90565b610dbf6020830186610b9f565b610dcc6040830185610cd9565b610dd960c0830184610d90565b95945050505050565b5f610dec82610627565b9150610df783610627565b9250828201905080821115610e0f57610e0e610aba565b5b92915050565b5f819050919050565b5f610e38610e33610e2e846109ca565b610e15565b610627565b9050919050565b610e4881610e1e565b82525050565b5f604082019050610e615f830185610b9f565b610e6e6020830184610e3f565b9392505050565b5f610e7f826109ca565b9150610e8a836109ca565b9250828203905067ffffffffffffffff811115610eaa57610ea9610aba565b5b92915050565b5f610eba826109ca565b9150610ec5836109ca565b9250828201905067ffffffffffffffff811115610ee557610ee4610aba565b5b92915050565b5f610ef682886109fa565b600882019150610f0682876109fa565b600882019150610f1682866109fa565b600882019150610f2682856109fa565b600882019150610f3682846109fa565b6008820191508190509695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b5f81905092915050565b5f610f8a82610c1c565b610f948185610f76565b9350610fa481856020860161071a565b80840191505092915050565b5f610fbb8285610f80565b9150610fc78284610f80565b91508190509392505050565b6113e680610fe05f395ff3fe608060405234801561000f575f80fd5b5060043610610108575f3560e01c80637fa3a40e116100a0578063d5719dc21161006f578063d5719dc214610282578063d9dd67ab146102b2578063e1d66afe146102e2578063e8eb1dc3146102fe578063eca067ad1461031c57610108565b80637fa3a40e1461020c578063a7b51d191461022a578063ad9c0c2e14610246578063b752a7d11461026457610108565b80632aa6dae4116100dc5780632aa6dae4146101965780632f1ec5e9146101b45780634f359a37146101d25780635c88e873146101f057610108565b806284120c1461010c57806304dd7b8b1461012a57806306f130561461014857806316bf557914610166575b5f80fd5b61011461033a565b604051610121919061093f565b60405180910390f35b610132610346565b60405161013f919061093f565b60405180910390f35b61015061034b565b60405161015d919061093f565b60405180910390f35b610180600480360381019061017b9190610993565b610357565b60405161018d91906109d6565b60405180910390f35b61019e610377565b6040516101ab919061093f565b60405180910390f35b6101bc61037d565b6040516101c99190610a11565b60405180910390f35b6101da610383565b6040516101e79190610a11565b60405180910390f35b61020a60048036038101906102059190610b66565b610388565b005b61021461050d565b604051610221919061093f565b60405180910390f35b610244600480360381019061023f9190610c62565b610513565b005b61024e610669565b60405161025b9190610a11565b60405180910390f35b61026c61066f565b6040516102799190610a11565b60405180910390f35b61029c60048036038101906102979190610993565b610676565b6040516102a991906109d6565b60405180910390f35b6102cc60048036038101906102c79190610993565b610696565b6040516102d991906109d6565b60405180910390f35b6102fc60048036038101906102f79190610cce565b6106bb565b005b6103066106ed565b6040516103139190610a11565b60405180910390f35b6103246106f4565b604051610331919061093f565b60405180910390f35b5f600380549050905090565b5f5481565b5f600380549050905090565b60038181548110610366575f80fd5b905f5260205f20015f915090505481565b60015481565b610e1081565b600c81565b815f81905550806001819055505f60028054905090505f806103aa8684610700565b915091505f60038054905090505f805f1b90505f8211156103f15760036001836103d49190610d4b565b815481106103e5576103e4610d7e565b5b905f5260205f20015490505b5f805f1b90505f86111561042b57600260018761040e9190610d4b565b8154811061041f5761041e610d7e565b5b905f5260205f20015490505b5f82868360405160200161044193929190610dcb565b604051602081830303815290604052805190602001209050600381908060018154018082558091505060019003905f5260205f20015f9091909190915055866004819055508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7856004548a60016040516104c19493929190610edc565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208b6040516104f99190610f7f565b60405180910390a250505050505050505050565b60045481565b5f60028054905090505f828051906020012090505f85854342864887604051602001610545979695949392919061106c565b6040516020818303038152906040528051906020012090505f805f1b90505f84111561059757600260018561057a9190610d4b565b8154811061058b5761058a610d7e565b5b905f5260205f20015490505b600281836040516020016105ac9291906110ec565b60405160208183030381529060405280519060200120908060018154018082558091505060019003905f5260205f20015f909190919091505580847f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1308a8a88484260405161062096959493929190611135565b60405180910390a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b866040516106589190610f7f565b60405180910390a250505050505050565b611c2081565b6201518081565b60028181548110610685575f80fd5b905f5260205f20015f915090505481565b5f600382815481106106ab576106aa610d7e565b5b905f5260205f2001549050919050565b6106e8600c8484846040516020016106d4929190611194565b604051602081830303815290604052610513565b505050565b6201cccc81565b5f600280549050905090565b5f6107096108db565b5f8451602861071891906111bf565b90506201cccc67ffffffffffffffff1681111561077157806201cccc6040517f4634691b00000000000000000000000000000000000000000000000000000000815260040161076892919061122b565b60405180910390fd5b6107796108db565b6201518067ffffffffffffffff164211156107bc57620151804261079d9190611252565b815f019067ffffffffffffffff16908167ffffffffffffffff16815250505b610e10426107ca919061128d565b816020019067ffffffffffffffff16908167ffffffffffffffff1681525050611c2067ffffffffffffffff1643111561082b57611c204361080b9190611252565b816040019067ffffffffffffffff16908167ffffffffffffffff16815250505b600c43610838919061128d565b816060019067ffffffffffffffff16908167ffffffffffffffff16815250505f815f0151826020015183604001518460600151896040516020016108809594939291906112c8565b604051602081830303815290604052905060288151146108a3576108a2611326565b5b80876040516020016108b692919061138d565b6040516020818303038152906040528051906020012082945094505050509250929050565b60405180608001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681525090565b5f819050919050565b61093981610927565b82525050565b5f6020820190506109525f830184610930565b92915050565b5f604051905090565b5f80fd5b5f80fd5b61097281610927565b811461097c575f80fd5b50565b5f8135905061098d81610969565b92915050565b5f602082840312156109a8576109a7610961565b5b5f6109b58482850161097f565b91505092915050565b5f819050919050565b6109d0816109be565b82525050565b5f6020820190506109e95f8301846109c7565b92915050565b5f67ffffffffffffffff82169050919050565b610a0b816109ef565b82525050565b5f602082019050610a245f830184610a02565b92915050565b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610a7882610a32565b810181811067ffffffffffffffff82111715610a9757610a96610a42565b5b80604052505050565b5f610aa9610958565b9050610ab58282610a6f565b919050565b5f67ffffffffffffffff821115610ad457610ad3610a42565b5b610add82610a32565b9050602081019050919050565b828183375f83830152505050565b5f610b0a610b0584610aba565b610aa0565b905082815260208101848484011115610b2657610b25610a2e565b5b610b31848285610aea565b509392505050565b5f82601f830112610b4d57610b4c610a2a565b5b8135610b5d848260208601610af8565b91505092915050565b5f805f60608486031215610b7d57610b7c610961565b5b5f84013567ffffffffffffffff811115610b9a57610b99610965565b5b610ba686828701610b39565b9350506020610bb78682870161097f565b9250506040610bc88682870161097f565b9150509250925092565b5f60ff82169050919050565b610be781610bd2565b8114610bf1575f80fd5b50565b5f81359050610c0281610bde565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610c3182610c08565b9050919050565b610c4181610c27565b8114610c4b575f80fd5b50565b5f81359050610c5c81610c38565b92915050565b5f805f60608486031215610c7957610c78610961565b5b5f610c8686828701610bf4565b9350506020610c9786828701610c4e565b925050604084013567ffffffffffffffff811115610cb857610cb7610965565b5b610cc486828701610b39565b9150509250925092565b5f805f60608486031215610ce557610ce4610961565b5b5f610cf286828701610c4e565b9350506020610d0386828701610c4e565b9250506040610d148682870161097f565b9150509250925092565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610d5582610927565b9150610d6083610927565b9250828203905081811115610d7857610d77610d1e565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b610dc5610dc0826109be565b610dab565b82525050565b5f610dd68286610db4565b602082019150610de68285610db4565b602082019150610df68284610db4565b602082019150819050949350505050565b610e10816109ef565b82525050565b608082015f820151610e2a5f850182610e07565b506020820151610e3d6020850182610e07565b506040820151610e506040850182610e07565b506060820151610e636060850182610e07565b50505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60048110610ea757610ea6610e69565b5b50565b5f819050610eb782610e96565b919050565b5f610ec682610eaa565b9050919050565b610ed681610ebc565b82525050565b5f60e082019050610eef5f8301876109c7565b610efc6020830186610930565b610f096040830185610e16565b610f1660c0830184610ecd565b95945050505050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f610f5182610f1f565b610f5b8185610f29565b9350610f6b818560208601610f39565b610f7481610a32565b840191505092915050565b5f6020820190508181035f830152610f978184610f47565b905092915050565b5f8160f81b9050919050565b5f610fb582610f9f565b9050919050565b610fcd610fc882610bd2565b610fab565b82525050565b5f8160601b9050919050565b5f610fe982610fd3565b9050919050565b5f610ffa82610fdf565b9050919050565b61101261100d82610c27565b610ff0565b82525050565b5f8160c01b9050919050565b5f61102e82611018565b9050919050565b611046611041826109ef565b611024565b82525050565b5f819050919050565b61106661106182610927565b61104c565b82525050565b5f611077828a610fbc565b6001820191506110878289611001565b6014820191506110978288611035565b6008820191506110a78287611035565b6008820191506110b78286611055565b6020820191506110c78285611055565b6020820191506110d78284610db4565b60208201915081905098975050505050505050565b5f6110f78285610db4565b6020820191506111078284610db4565b6020820191508190509392505050565b61112081610c27565b82525050565b61112f81610bd2565b82525050565b5f60c0820190506111485f830189611117565b6111556020830188611126565b6111626040830187611117565b61116f60608301866109c7565b61117c6080830185610930565b61118960a0830184610a02565b979650505050505050565b5f61119f8285611001565b6014820191506111af8284611055565b6020820191508190509392505050565b5f6111c982610927565b91506111d483610927565b92508282019050808211156111ec576111eb610d1e565b5b92915050565b5f819050919050565b5f61121561121061120b846109ef565b6111f2565b610927565b9050919050565b611225816111fb565b82525050565b5f60408201905061123e5f830185610930565b61124b602083018461121c565b9392505050565b5f61125c826109ef565b9150611267836109ef565b9250828203905067ffffffffffffffff81111561128757611286610d1e565b5b92915050565b5f611297826109ef565b91506112a2836109ef565b9250828201905067ffffffffffffffff8111156112c2576112c1610d1e565b5b92915050565b5f6112d38288611035565b6008820191506112e38287611035565b6008820191506112f38286611035565b6008820191506113038285611035565b6008820191506113138284611035565b6008820191508190509695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b5f81905092915050565b5f61136782610f1f565b6113718185611353565b9350611381818560208601610f39565b80840191505092915050565b5f611398828561135d565b91506113a4828461135d565b9150819050939250505056fea26469706673582212202c505ecc93a7241b2f2bd0313c9b4557b0c122cb4975714e7c879d28af5bd45664736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R_\x80U_`\x01U4\x80\x15a\0\x16W_\x80\xFD[P`@Qa#\xC68\x03\x80a#\xC6\x839\x81\x81\x01`@R\x81\x01\x90a\08\x91\x90a\x07\x96V[_\x81Q\x11a\0{W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0r\x90a\x08JV[`@Q\x80\x91\x03\x90\xFD[_`\x01\x90P_H\x90P_\x84\x83\x83\x86`@Q` \x01a\0\x9C\x94\x93\x92\x91\x90a\t\x0CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\0\xBF`\x0B_\x83a\x01\x0E` \x1B` \x1CV[a\x01\x04`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01~\x0B\0\x80\x02\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP_\x80a\x02d` \x1B` \x1CV[PPPPPa\x0F\xD3V[_`\x02\x80T\x90P\x90P_\x82\x80Q\x90` \x01 \x90P_\x85\x85CB\x86H\x87`@Q` \x01a\x01@\x97\x96\x95\x94\x93\x92\x91\x90a\n:V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_\x80_\x1B\x90P_\x84\x11\x15a\x01\x92W`\x02`\x01\x85a\x01u\x91\x90a\n\xE7V[\x81T\x81\x10a\x01\x86Wa\x01\x85a\x0B\x1AV[[\x90_R` _ \x01T\x90P[`\x02\x81\x83`@Q` \x01a\x01\xA7\x92\x91\x90a\x0BGV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x80\x84\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE10\x8A\x8A\x88HB`@Qa\x02\x1B\x96\x95\x94\x93\x92\x91\x90a\x0B\xBDV[`@Q\x80\x91\x03\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x02S\x91\x90a\x0CnV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[\x81_\x81\x90UP\x80`\x01\x81\x90UP_`\x02\x80T\x90P\x90P_\x80a\x02\x8C\x86\x84a\x03\xEF` \x1B` \x1CV[\x91P\x91P_`\x03\x80T\x90P\x90P_\x80_\x1B\x90P_\x82\x11\x15a\x02\xD3W`\x03`\x01\x83a\x02\xB6\x91\x90a\n\xE7V[\x81T\x81\x10a\x02\xC7Wa\x02\xC6a\x0B\x1AV[[\x90_R` _ \x01T\x90P[_\x80_\x1B\x90P_\x86\x11\x15a\x03\rW`\x02`\x01\x87a\x02\xF0\x91\x90a\n\xE7V[\x81T\x81\x10a\x03\x01Wa\x03\0a\x0B\x1AV[[\x90_R` _ \x01T\x90P[_\x82\x86\x83`@Q` \x01a\x03#\x93\x92\x91\x90a\x0C\x8EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x03\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x86`\x04\x81\x90UP\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85`\x04T\x8A`\x01`@Qa\x03\xA3\x94\x93\x92\x91\x90a\r\x9FV[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8B`@Qa\x03\xDB\x91\x90a\x0CnV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPV[_a\x03\xF8a\x05\xCAV[_\x84Q`(a\x04\x07\x91\x90a\r\xE2V[\x90Pb\x01\xCC\xCCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\x04`W\x80b\x01\xCC\xCC`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04W\x92\x91\x90a\x0ENV[`@Q\x80\x91\x03\x90\xFD[a\x04ha\x05\xCAV[b\x01Q\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15a\x04\xABWb\x01Q\x80Ba\x04\x8C\x91\x90a\x0EuV[\x81_\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[a\x0E\x10Ba\x04\xB9\x91\x90a\x0E\xB0V[\x81` \x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x1C g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x11\x15a\x05\x1AWa\x1C Ca\x04\xFA\x91\x90a\x0EuV[\x81`@\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[`\x0CCa\x05'\x91\x90a\x0E\xB0V[\x81``\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x89`@Q` \x01a\x05o\x95\x94\x93\x92\x91\x90a\x0E\xEBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\x05\x92Wa\x05\x91a\x0FIV[[\x80\x87`@Q` \x01a\x05\xA5\x92\x91\x90a\x0F\xB0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[`@Q\x80`\x80\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[a\x069\x81a\x06'V[\x81\x14a\x06CW_\x80\xFD[PV[_\x81Q\x90Pa\x06T\x81a\x060V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x06\xA8\x82a\x06bV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06\xC7Wa\x06\xC6a\x06rV[[\x80`@RPPPV[_a\x06\xD9a\x06\x16V[\x90Pa\x06\xE5\x82\x82a\x06\x9FV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\x04Wa\x07\x03a\x06rV[[a\x07\r\x82a\x06bV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x07:a\x075\x84a\x06\xEAV[a\x06\xD0V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x07VWa\x07Ua\x06^V[[a\x07a\x84\x82\x85a\x07\x1AV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x07}Wa\x07|a\x06ZV[[\x81Qa\x07\x8D\x84\x82` \x86\x01a\x07(V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x07\xACWa\x07\xABa\x06\x1FV[[_a\x07\xB9\x85\x82\x86\x01a\x06FV[\x92PP` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xDAWa\x07\xD9a\x06#V[[a\x07\xE6\x85\x82\x86\x01a\x07iV[\x91PP\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FEMPTY_CHAIN_CONFIG\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x084`\x12\x83a\x07\xF0V[\x91Pa\x08?\x82a\x08\0V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x08a\x81a\x08(V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x08\x82a\x08}\x82a\x06'V[a\x08hV[\x82RPPV[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\x08\xAA\x82a\x08\x94V[\x90P\x91\x90PV[a\x08\xC2a\x08\xBD\x82a\x08\x88V[a\x08\xA0V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_a\x08\xE6\x82a\x08\xC8V[a\x08\xF0\x81\x85a\x08\xD2V[\x93Pa\t\0\x81\x85` \x86\x01a\x07\x1AV[\x80\x84\x01\x91PP\x92\x91PPV[_a\t\x17\x82\x87a\x08qV[` \x82\x01\x91Pa\t'\x82\x86a\x08\xB1V[`\x01\x82\x01\x91Pa\t7\x82\x85a\x08qV[` \x82\x01\x91Pa\tG\x82\x84a\x08\xDCV[\x91P\x81\x90P\x95\x94PPPPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\t~\x82a\tUV[\x90P\x91\x90PV[_\x81``\x1B\x90P\x91\x90PV[_a\t\x9B\x82a\t\x85V[\x90P\x91\x90PV[_a\t\xAC\x82a\t\x91V[\x90P\x91\x90PV[a\t\xC4a\t\xBF\x82a\ttV[a\t\xA2V[\x82RPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xC0\x1B\x90P\x91\x90PV[_a\t\xF3\x82a\t\xDDV[\x90P\x91\x90PV[a\n\x0Ba\n\x06\x82a\t\xCAV[a\t\xE9V[\x82RPPV[_\x81\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\n4a\n/\x82a\n\x11V[a\n\x1AV[\x82RPPV[_a\nE\x82\x8Aa\x08\xB1V[`\x01\x82\x01\x91Pa\nU\x82\x89a\t\xB3V[`\x14\x82\x01\x91Pa\ne\x82\x88a\t\xFAV[`\x08\x82\x01\x91Pa\nu\x82\x87a\t\xFAV[`\x08\x82\x01\x91Pa\n\x85\x82\x86a\x08qV[` \x82\x01\x91Pa\n\x95\x82\x85a\x08qV[` \x82\x01\x91Pa\n\xA5\x82\x84a\n#V[` \x82\x01\x91P\x81\x90P\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\n\xF1\x82a\x06'V[\x91Pa\n\xFC\x83a\x06'V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x0B\x14Wa\x0B\x13a\n\xBAV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_a\x0BR\x82\x85a\n#V[` \x82\x01\x91Pa\x0Bb\x82\x84a\n#V[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[a\x0B{\x81a\ttV[\x82RPPV[a\x0B\x8A\x81a\x08\x88V[\x82RPPV[a\x0B\x99\x81a\n\x11V[\x82RPPV[a\x0B\xA8\x81a\x06'V[\x82RPPV[a\x0B\xB7\x81a\t\xCAV[\x82RPPV[_`\xC0\x82\x01\x90Pa\x0B\xD0_\x83\x01\x89a\x0BrV[a\x0B\xDD` \x83\x01\x88a\x0B\x81V[a\x0B\xEA`@\x83\x01\x87a\x0BrV[a\x0B\xF7``\x83\x01\x86a\x0B\x90V[a\x0C\x04`\x80\x83\x01\x85a\x0B\x9FV[a\x0C\x11`\xA0\x83\x01\x84a\x0B\xAEV[\x97\x96PPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\x0C@\x82a\x0C\x1CV[a\x0CJ\x81\x85a\x0C&V[\x93Pa\x0CZ\x81\x85` \x86\x01a\x07\x1AV[a\x0Cc\x81a\x06bV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0C\x86\x81\x84a\x0C6V[\x90P\x92\x91PPV[_a\x0C\x99\x82\x86a\n#V[` \x82\x01\x91Pa\x0C\xA9\x82\x85a\n#V[` \x82\x01\x91Pa\x0C\xB9\x82\x84a\n#V[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[a\x0C\xD3\x81a\t\xCAV[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\x0C\xED_\x85\x01\x82a\x0C\xCAV[P` \x82\x01Qa\r\0` \x85\x01\x82a\x0C\xCAV[P`@\x82\x01Qa\r\x13`@\x85\x01\x82a\x0C\xCAV[P``\x82\x01Qa\r&``\x85\x01\x82a\x0C\xCAV[PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\rjWa\ria\r,V[[PV[_\x81\x90Pa\rz\x82a\rYV[\x91\x90PV[_a\r\x89\x82a\rmV[\x90P\x91\x90PV[a\r\x99\x81a\r\x7FV[\x82RPPV[_`\xE0\x82\x01\x90Pa\r\xB2_\x83\x01\x87a\x0B\x90V[a\r\xBF` \x83\x01\x86a\x0B\x9FV[a\r\xCC`@\x83\x01\x85a\x0C\xD9V[a\r\xD9`\xC0\x83\x01\x84a\r\x90V[\x95\x94PPPPPV[_a\r\xEC\x82a\x06'V[\x91Pa\r\xF7\x83a\x06'V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x0E\x0FWa\x0E\x0Ea\n\xBAV[[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x0E8a\x0E3a\x0E.\x84a\t\xCAV[a\x0E\x15V[a\x06'V[\x90P\x91\x90PV[a\x0EH\x81a\x0E\x1EV[\x82RPPV[_`@\x82\x01\x90Pa\x0Ea_\x83\x01\x85a\x0B\x9FV[a\x0En` \x83\x01\x84a\x0E?V[\x93\x92PPPV[_a\x0E\x7F\x82a\t\xCAV[\x91Pa\x0E\x8A\x83a\t\xCAV[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xAAWa\x0E\xA9a\n\xBAV[[\x92\x91PPV[_a\x0E\xBA\x82a\t\xCAV[\x91Pa\x0E\xC5\x83a\t\xCAV[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xE5Wa\x0E\xE4a\n\xBAV[[\x92\x91PPV[_a\x0E\xF6\x82\x88a\t\xFAV[`\x08\x82\x01\x91Pa\x0F\x06\x82\x87a\t\xFAV[`\x08\x82\x01\x91Pa\x0F\x16\x82\x86a\t\xFAV[`\x08\x82\x01\x91Pa\x0F&\x82\x85a\t\xFAV[`\x08\x82\x01\x91Pa\x0F6\x82\x84a\t\xFAV[`\x08\x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[_\x81\x90P\x92\x91PPV[_a\x0F\x8A\x82a\x0C\x1CV[a\x0F\x94\x81\x85a\x0FvV[\x93Pa\x0F\xA4\x81\x85` \x86\x01a\x07\x1AV[\x80\x84\x01\x91PP\x92\x91PPV[_a\x0F\xBB\x82\x85a\x0F\x80V[\x91Pa\x0F\xC7\x82\x84a\x0F\x80V[\x91P\x81\x90P\x93\x92PPPV[a\x13\xE6\x80a\x0F\xE0_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01\x08W_5`\xE0\x1C\x80c\x7F\xA3\xA4\x0E\x11a\0\xA0W\x80c\xD5q\x9D\xC2\x11a\0oW\x80c\xD5q\x9D\xC2\x14a\x02\x82W\x80c\xD9\xDDg\xAB\x14a\x02\xB2W\x80c\xE1\xD6j\xFE\x14a\x02\xE2W\x80c\xE8\xEB\x1D\xC3\x14a\x02\xFEW\x80c\xEC\xA0g\xAD\x14a\x03\x1CWa\x01\x08V[\x80c\x7F\xA3\xA4\x0E\x14a\x02\x0CW\x80c\xA7\xB5\x1D\x19\x14a\x02*W\x80c\xAD\x9C\x0C.\x14a\x02FW\x80c\xB7R\xA7\xD1\x14a\x02dWa\x01\x08V[\x80c*\xA6\xDA\xE4\x11a\0\xDCW\x80c*\xA6\xDA\xE4\x14a\x01\x96W\x80c/\x1E\xC5\xE9\x14a\x01\xB4W\x80cO5\x9A7\x14a\x01\xD2W\x80c\\\x88\xE8s\x14a\x01\xF0Wa\x01\x08V[\x80b\x84\x12\x0C\x14a\x01\x0CW\x80c\x04\xDD{\x8B\x14a\x01*W\x80c\x06\xF10V\x14a\x01HW\x80c\x16\xBFUy\x14a\x01fW[_\x80\xFD[a\x01\x14a\x03:V[`@Qa\x01!\x91\x90a\t?V[`@Q\x80\x91\x03\x90\xF3[a\x012a\x03FV[`@Qa\x01?\x91\x90a\t?V[`@Q\x80\x91\x03\x90\xF3[a\x01Pa\x03KV[`@Qa\x01]\x91\x90a\t?V[`@Q\x80\x91\x03\x90\xF3[a\x01\x80`\x04\x806\x03\x81\x01\x90a\x01{\x91\x90a\t\x93V[a\x03WV[`@Qa\x01\x8D\x91\x90a\t\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x01\x9Ea\x03wV[`@Qa\x01\xAB\x91\x90a\t?V[`@Q\x80\x91\x03\x90\xF3[a\x01\xBCa\x03}V[`@Qa\x01\xC9\x91\x90a\n\x11V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDAa\x03\x83V[`@Qa\x01\xE7\x91\x90a\n\x11V[`@Q\x80\x91\x03\x90\xF3[a\x02\n`\x04\x806\x03\x81\x01\x90a\x02\x05\x91\x90a\x0BfV[a\x03\x88V[\0[a\x02\x14a\x05\rV[`@Qa\x02!\x91\x90a\t?V[`@Q\x80\x91\x03\x90\xF3[a\x02D`\x04\x806\x03\x81\x01\x90a\x02?\x91\x90a\x0CbV[a\x05\x13V[\0[a\x02Na\x06iV[`@Qa\x02[\x91\x90a\n\x11V[`@Q\x80\x91\x03\x90\xF3[a\x02la\x06oV[`@Qa\x02y\x91\x90a\n\x11V[`@Q\x80\x91\x03\x90\xF3[a\x02\x9C`\x04\x806\x03\x81\x01\x90a\x02\x97\x91\x90a\t\x93V[a\x06vV[`@Qa\x02\xA9\x91\x90a\t\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x02\xCC`\x04\x806\x03\x81\x01\x90a\x02\xC7\x91\x90a\t\x93V[a\x06\x96V[`@Qa\x02\xD9\x91\x90a\t\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x02\xFC`\x04\x806\x03\x81\x01\x90a\x02\xF7\x91\x90a\x0C\xCEV[a\x06\xBBV[\0[a\x03\x06a\x06\xEDV[`@Qa\x03\x13\x91\x90a\n\x11V[`@Q\x80\x91\x03\x90\xF3[a\x03$a\x06\xF4V[`@Qa\x031\x91\x90a\t?V[`@Q\x80\x91\x03\x90\xF3[_`\x03\x80T\x90P\x90P\x90V[_T\x81V[_`\x03\x80T\x90P\x90P\x90V[`\x03\x81\x81T\x81\x10a\x03fW_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[`\x01T\x81V[a\x0E\x10\x81V[`\x0C\x81V[\x81_\x81\x90UP\x80`\x01\x81\x90UP_`\x02\x80T\x90P\x90P_\x80a\x03\xAA\x86\x84a\x07\0V[\x91P\x91P_`\x03\x80T\x90P\x90P_\x80_\x1B\x90P_\x82\x11\x15a\x03\xF1W`\x03`\x01\x83a\x03\xD4\x91\x90a\rKV[\x81T\x81\x10a\x03\xE5Wa\x03\xE4a\r~V[[\x90_R` _ \x01T\x90P[_\x80_\x1B\x90P_\x86\x11\x15a\x04+W`\x02`\x01\x87a\x04\x0E\x91\x90a\rKV[\x81T\x81\x10a\x04\x1FWa\x04\x1Ea\r~V[[\x90_R` _ \x01T\x90P[_\x82\x86\x83`@Q` \x01a\x04A\x93\x92\x91\x90a\r\xCBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x03\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x86`\x04\x81\x90UP\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85`\x04T\x8A`\x01`@Qa\x04\xC1\x94\x93\x92\x91\x90a\x0E\xDCV[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8B`@Qa\x04\xF9\x91\x90a\x0F\x7FV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPV[`\x04T\x81V[_`\x02\x80T\x90P\x90P_\x82\x80Q\x90` \x01 \x90P_\x85\x85CB\x86H\x87`@Q` \x01a\x05E\x97\x96\x95\x94\x93\x92\x91\x90a\x10lV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_\x80_\x1B\x90P_\x84\x11\x15a\x05\x97W`\x02`\x01\x85a\x05z\x91\x90a\rKV[\x81T\x81\x10a\x05\x8BWa\x05\x8Aa\r~V[[\x90_R` _ \x01T\x90P[`\x02\x81\x83`@Q` \x01a\x05\xAC\x92\x91\x90a\x10\xECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x80\x84\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE10\x8A\x8A\x88HB`@Qa\x06 \x96\x95\x94\x93\x92\x91\x90a\x115V[`@Q\x80\x91\x03\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x06X\x91\x90a\x0F\x7FV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[a\x1C \x81V[b\x01Q\x80\x81V[`\x02\x81\x81T\x81\x10a\x06\x85W_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[_`\x03\x82\x81T\x81\x10a\x06\xABWa\x06\xAAa\r~V[[\x90_R` _ \x01T\x90P\x91\x90PV[a\x06\xE8`\x0C\x84\x84\x84`@Q` \x01a\x06\xD4\x92\x91\x90a\x11\x94V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x05\x13V[PPPV[b\x01\xCC\xCC\x81V[_`\x02\x80T\x90P\x90P\x90V[_a\x07\ta\x08\xDBV[_\x84Q`(a\x07\x18\x91\x90a\x11\xBFV[\x90Pb\x01\xCC\xCCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\x07qW\x80b\x01\xCC\xCC`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07h\x92\x91\x90a\x12+V[`@Q\x80\x91\x03\x90\xFD[a\x07ya\x08\xDBV[b\x01Q\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15a\x07\xBCWb\x01Q\x80Ba\x07\x9D\x91\x90a\x12RV[\x81_\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[a\x0E\x10Ba\x07\xCA\x91\x90a\x12\x8DV[\x81` \x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x1C g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x11\x15a\x08+Wa\x1C Ca\x08\x0B\x91\x90a\x12RV[\x81`@\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[`\x0CCa\x088\x91\x90a\x12\x8DV[\x81``\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x89`@Q` \x01a\x08\x80\x95\x94\x93\x92\x91\x90a\x12\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\x08\xA3Wa\x08\xA2a\x13&V[[\x80\x87`@Q` \x01a\x08\xB6\x92\x91\x90a\x13\x8DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[`@Q\x80`\x80\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_\x81\x90P\x91\x90PV[a\t9\x81a\t'V[\x82RPPV[_` \x82\x01\x90Pa\tR_\x83\x01\x84a\t0V[\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[a\tr\x81a\t'V[\x81\x14a\t|W_\x80\xFD[PV[_\x815\x90Pa\t\x8D\x81a\tiV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\t\xA8Wa\t\xA7a\taV[[_a\t\xB5\x84\x82\x85\x01a\t\x7FV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\t\xD0\x81a\t\xBEV[\x82RPPV[_` \x82\x01\x90Pa\t\xE9_\x83\x01\x84a\t\xC7V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\n\x0B\x81a\t\xEFV[\x82RPPV[_` \x82\x01\x90Pa\n$_\x83\x01\x84a\n\x02V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\nx\x82a\n2V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\n\x97Wa\n\x96a\nBV[[\x80`@RPPPV[_a\n\xA9a\tXV[\x90Pa\n\xB5\x82\x82a\noV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\n\xD4Wa\n\xD3a\nBV[[a\n\xDD\x82a\n2V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0B\na\x0B\x05\x84a\n\xBAV[a\n\xA0V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0B&Wa\x0B%a\n.V[[a\x0B1\x84\x82\x85a\n\xEAV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0BMWa\x0BLa\n*V[[\x815a\x0B]\x84\x82` \x86\x01a\n\xF8V[\x91PP\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x0B}Wa\x0B|a\taV[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x9AWa\x0B\x99a\teV[[a\x0B\xA6\x86\x82\x87\x01a\x0B9V[\x93PP` a\x0B\xB7\x86\x82\x87\x01a\t\x7FV[\x92PP`@a\x0B\xC8\x86\x82\x87\x01a\t\x7FV[\x91PP\x92P\x92P\x92V[_`\xFF\x82\x16\x90P\x91\x90PV[a\x0B\xE7\x81a\x0B\xD2V[\x81\x14a\x0B\xF1W_\x80\xFD[PV[_\x815\x90Pa\x0C\x02\x81a\x0B\xDEV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0C1\x82a\x0C\x08V[\x90P\x91\x90PV[a\x0CA\x81a\x0C'V[\x81\x14a\x0CKW_\x80\xFD[PV[_\x815\x90Pa\x0C\\\x81a\x0C8V[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x0CyWa\x0Cxa\taV[[_a\x0C\x86\x86\x82\x87\x01a\x0B\xF4V[\x93PP` a\x0C\x97\x86\x82\x87\x01a\x0CNV[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xB8Wa\x0C\xB7a\teV[[a\x0C\xC4\x86\x82\x87\x01a\x0B9V[\x91PP\x92P\x92P\x92V[_\x80_``\x84\x86\x03\x12\x15a\x0C\xE5Wa\x0C\xE4a\taV[[_a\x0C\xF2\x86\x82\x87\x01a\x0CNV[\x93PP` a\r\x03\x86\x82\x87\x01a\x0CNV[\x92PP`@a\r\x14\x86\x82\x87\x01a\t\x7FV[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\rU\x82a\t'V[\x91Pa\r`\x83a\t'V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\rxWa\rwa\r\x1EV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[a\r\xC5a\r\xC0\x82a\t\xBEV[a\r\xABV[\x82RPPV[_a\r\xD6\x82\x86a\r\xB4V[` \x82\x01\x91Pa\r\xE6\x82\x85a\r\xB4V[` \x82\x01\x91Pa\r\xF6\x82\x84a\r\xB4V[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[a\x0E\x10\x81a\t\xEFV[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\x0E*_\x85\x01\x82a\x0E\x07V[P` \x82\x01Qa\x0E=` \x85\x01\x82a\x0E\x07V[P`@\x82\x01Qa\x0EP`@\x85\x01\x82a\x0E\x07V[P``\x82\x01Qa\x0Ec``\x85\x01\x82a\x0E\x07V[PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x0E\xA7Wa\x0E\xA6a\x0EiV[[PV[_\x81\x90Pa\x0E\xB7\x82a\x0E\x96V[\x91\x90PV[_a\x0E\xC6\x82a\x0E\xAAV[\x90P\x91\x90PV[a\x0E\xD6\x81a\x0E\xBCV[\x82RPPV[_`\xE0\x82\x01\x90Pa\x0E\xEF_\x83\x01\x87a\t\xC7V[a\x0E\xFC` \x83\x01\x86a\t0V[a\x0F\t`@\x83\x01\x85a\x0E\x16V[a\x0F\x16`\xC0\x83\x01\x84a\x0E\xCDV[\x95\x94PPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x0FQ\x82a\x0F\x1FV[a\x0F[\x81\x85a\x0F)V[\x93Pa\x0Fk\x81\x85` \x86\x01a\x0F9V[a\x0Ft\x81a\n2V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0F\x97\x81\x84a\x0FGV[\x90P\x92\x91PPV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\x0F\xB5\x82a\x0F\x9FV[\x90P\x91\x90PV[a\x0F\xCDa\x0F\xC8\x82a\x0B\xD2V[a\x0F\xABV[\x82RPPV[_\x81``\x1B\x90P\x91\x90PV[_a\x0F\xE9\x82a\x0F\xD3V[\x90P\x91\x90PV[_a\x0F\xFA\x82a\x0F\xDFV[\x90P\x91\x90PV[a\x10\x12a\x10\r\x82a\x0C'V[a\x0F\xF0V[\x82RPPV[_\x81`\xC0\x1B\x90P\x91\x90PV[_a\x10.\x82a\x10\x18V[\x90P\x91\x90PV[a\x10Fa\x10A\x82a\t\xEFV[a\x10$V[\x82RPPV[_\x81\x90P\x91\x90PV[a\x10fa\x10a\x82a\t'V[a\x10LV[\x82RPPV[_a\x10w\x82\x8Aa\x0F\xBCV[`\x01\x82\x01\x91Pa\x10\x87\x82\x89a\x10\x01V[`\x14\x82\x01\x91Pa\x10\x97\x82\x88a\x105V[`\x08\x82\x01\x91Pa\x10\xA7\x82\x87a\x105V[`\x08\x82\x01\x91Pa\x10\xB7\x82\x86a\x10UV[` \x82\x01\x91Pa\x10\xC7\x82\x85a\x10UV[` \x82\x01\x91Pa\x10\xD7\x82\x84a\r\xB4V[` \x82\x01\x91P\x81\x90P\x98\x97PPPPPPPPV[_a\x10\xF7\x82\x85a\r\xB4V[` \x82\x01\x91Pa\x11\x07\x82\x84a\r\xB4V[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[a\x11 \x81a\x0C'V[\x82RPPV[a\x11/\x81a\x0B\xD2V[\x82RPPV[_`\xC0\x82\x01\x90Pa\x11H_\x83\x01\x89a\x11\x17V[a\x11U` \x83\x01\x88a\x11&V[a\x11b`@\x83\x01\x87a\x11\x17V[a\x11o``\x83\x01\x86a\t\xC7V[a\x11|`\x80\x83\x01\x85a\t0V[a\x11\x89`\xA0\x83\x01\x84a\n\x02V[\x97\x96PPPPPPPV[_a\x11\x9F\x82\x85a\x10\x01V[`\x14\x82\x01\x91Pa\x11\xAF\x82\x84a\x10UV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[_a\x11\xC9\x82a\t'V[\x91Pa\x11\xD4\x83a\t'V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x11\xECWa\x11\xEBa\r\x1EV[[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x12\x15a\x12\x10a\x12\x0B\x84a\t\xEFV[a\x11\xF2V[a\t'V[\x90P\x91\x90PV[a\x12%\x81a\x11\xFBV[\x82RPPV[_`@\x82\x01\x90Pa\x12>_\x83\x01\x85a\t0V[a\x12K` \x83\x01\x84a\x12\x1CV[\x93\x92PPPV[_a\x12\\\x82a\t\xEFV[\x91Pa\x12g\x83a\t\xEFV[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x87Wa\x12\x86a\r\x1EV[[\x92\x91PPV[_a\x12\x97\x82a\t\xEFV[\x91Pa\x12\xA2\x83a\t\xEFV[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xC2Wa\x12\xC1a\r\x1EV[[\x92\x91PPV[_a\x12\xD3\x82\x88a\x105V[`\x08\x82\x01\x91Pa\x12\xE3\x82\x87a\x105V[`\x08\x82\x01\x91Pa\x12\xF3\x82\x86a\x105V[`\x08\x82\x01\x91Pa\x13\x03\x82\x85a\x105V[`\x08\x82\x01\x91Pa\x13\x13\x82\x84a\x105V[`\x08\x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[_\x81\x90P\x92\x91PPV[_a\x13g\x82a\x0F\x1FV[a\x13q\x81\x85a\x13SV[\x93Pa\x13\x81\x81\x85` \x86\x01a\x0F9V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x13\x98\x82\x85a\x13]V[\x91Pa\x13\xA4\x82\x84a\x13]V[\x91P\x81\x90P\x93\x92PPPV\xFE\xA2dipfsX\"\x12 ,P^\xCC\x93\xA7$\x1B/+\xD01<\x9BEW\xB0\xC1\"\xCBIuqN|\x87\x9D(\xAF[\xD4VdsolcC\0\x08\x19\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610108575f3560e01c80637fa3a40e116100a0578063d5719dc21161006f578063d5719dc214610282578063d9dd67ab146102b2578063e1d66afe146102e2578063e8eb1dc3146102fe578063eca067ad1461031c57610108565b80637fa3a40e1461020c578063a7b51d191461022a578063ad9c0c2e14610246578063b752a7d11461026457610108565b80632aa6dae4116100dc5780632aa6dae4146101965780632f1ec5e9146101b45780634f359a37146101d25780635c88e873146101f057610108565b806284120c1461010c57806304dd7b8b1461012a57806306f130561461014857806316bf557914610166575b5f80fd5b61011461033a565b604051610121919061093f565b60405180910390f35b610132610346565b60405161013f919061093f565b60405180910390f35b61015061034b565b60405161015d919061093f565b60405180910390f35b610180600480360381019061017b9190610993565b610357565b60405161018d91906109d6565b60405180910390f35b61019e610377565b6040516101ab919061093f565b60405180910390f35b6101bc61037d565b6040516101c99190610a11565b60405180910390f35b6101da610383565b6040516101e79190610a11565b60405180910390f35b61020a60048036038101906102059190610b66565b610388565b005b61021461050d565b604051610221919061093f565b60405180910390f35b610244600480360381019061023f9190610c62565b610513565b005b61024e610669565b60405161025b9190610a11565b60405180910390f35b61026c61066f565b6040516102799190610a11565b60405180910390f35b61029c60048036038101906102979190610993565b610676565b6040516102a991906109d6565b60405180910390f35b6102cc60048036038101906102c79190610993565b610696565b6040516102d991906109d6565b60405180910390f35b6102fc60048036038101906102f79190610cce565b6106bb565b005b6103066106ed565b6040516103139190610a11565b60405180910390f35b6103246106f4565b604051610331919061093f565b60405180910390f35b5f600380549050905090565b5f5481565b5f600380549050905090565b60038181548110610366575f80fd5b905f5260205f20015f915090505481565b60015481565b610e1081565b600c81565b815f81905550806001819055505f60028054905090505f806103aa8684610700565b915091505f60038054905090505f805f1b90505f8211156103f15760036001836103d49190610d4b565b815481106103e5576103e4610d7e565b5b905f5260205f20015490505b5f805f1b90505f86111561042b57600260018761040e9190610d4b565b8154811061041f5761041e610d7e565b5b905f5260205f20015490505b5f82868360405160200161044193929190610dcb565b604051602081830303815290604052805190602001209050600381908060018154018082558091505060019003905f5260205f20015f9091909190915055866004819055508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7856004548a60016040516104c19493929190610edc565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208b6040516104f99190610f7f565b60405180910390a250505050505050505050565b60045481565b5f60028054905090505f828051906020012090505f85854342864887604051602001610545979695949392919061106c565b6040516020818303038152906040528051906020012090505f805f1b90505f84111561059757600260018561057a9190610d4b565b8154811061058b5761058a610d7e565b5b905f5260205f20015490505b600281836040516020016105ac9291906110ec565b60405160208183030381529060405280519060200120908060018154018082558091505060019003905f5260205f20015f909190919091505580847f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1308a8a88484260405161062096959493929190611135565b60405180910390a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b866040516106589190610f7f565b60405180910390a250505050505050565b611c2081565b6201518081565b60028181548110610685575f80fd5b905f5260205f20015f915090505481565b5f600382815481106106ab576106aa610d7e565b5b905f5260205f2001549050919050565b6106e8600c8484846040516020016106d4929190611194565b604051602081830303815290604052610513565b505050565b6201cccc81565b5f600280549050905090565b5f6107096108db565b5f8451602861071891906111bf565b90506201cccc67ffffffffffffffff1681111561077157806201cccc6040517f4634691b00000000000000000000000000000000000000000000000000000000815260040161076892919061122b565b60405180910390fd5b6107796108db565b6201518067ffffffffffffffff164211156107bc57620151804261079d9190611252565b815f019067ffffffffffffffff16908167ffffffffffffffff16815250505b610e10426107ca919061128d565b816020019067ffffffffffffffff16908167ffffffffffffffff1681525050611c2067ffffffffffffffff1643111561082b57611c204361080b9190611252565b816040019067ffffffffffffffff16908167ffffffffffffffff16815250505b600c43610838919061128d565b816060019067ffffffffffffffff16908167ffffffffffffffff16815250505f815f0151826020015183604001518460600151896040516020016108809594939291906112c8565b604051602081830303815290604052905060288151146108a3576108a2611326565b5b80876040516020016108b692919061138d565b6040516020818303038152906040528051906020012082945094505050509250929050565b60405180608001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681525090565b5f819050919050565b61093981610927565b82525050565b5f6020820190506109525f830184610930565b92915050565b5f604051905090565b5f80fd5b5f80fd5b61097281610927565b811461097c575f80fd5b50565b5f8135905061098d81610969565b92915050565b5f602082840312156109a8576109a7610961565b5b5f6109b58482850161097f565b91505092915050565b5f819050919050565b6109d0816109be565b82525050565b5f6020820190506109e95f8301846109c7565b92915050565b5f67ffffffffffffffff82169050919050565b610a0b816109ef565b82525050565b5f602082019050610a245f830184610a02565b92915050565b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610a7882610a32565b810181811067ffffffffffffffff82111715610a9757610a96610a42565b5b80604052505050565b5f610aa9610958565b9050610ab58282610a6f565b919050565b5f67ffffffffffffffff821115610ad457610ad3610a42565b5b610add82610a32565b9050602081019050919050565b828183375f83830152505050565b5f610b0a610b0584610aba565b610aa0565b905082815260208101848484011115610b2657610b25610a2e565b5b610b31848285610aea565b509392505050565b5f82601f830112610b4d57610b4c610a2a565b5b8135610b5d848260208601610af8565b91505092915050565b5f805f60608486031215610b7d57610b7c610961565b5b5f84013567ffffffffffffffff811115610b9a57610b99610965565b5b610ba686828701610b39565b9350506020610bb78682870161097f565b9250506040610bc88682870161097f565b9150509250925092565b5f60ff82169050919050565b610be781610bd2565b8114610bf1575f80fd5b50565b5f81359050610c0281610bde565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610c3182610c08565b9050919050565b610c4181610c27565b8114610c4b575f80fd5b50565b5f81359050610c5c81610c38565b92915050565b5f805f60608486031215610c7957610c78610961565b5b5f610c8686828701610bf4565b9350506020610c9786828701610c4e565b925050604084013567ffffffffffffffff811115610cb857610cb7610965565b5b610cc486828701610b39565b9150509250925092565b5f805f60608486031215610ce557610ce4610961565b5b5f610cf286828701610c4e565b9350506020610d0386828701610c4e565b9250506040610d148682870161097f565b9150509250925092565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610d5582610927565b9150610d6083610927565b9250828203905081811115610d7857610d77610d1e565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b610dc5610dc0826109be565b610dab565b82525050565b5f610dd68286610db4565b602082019150610de68285610db4565b602082019150610df68284610db4565b602082019150819050949350505050565b610e10816109ef565b82525050565b608082015f820151610e2a5f850182610e07565b506020820151610e3d6020850182610e07565b506040820151610e506040850182610e07565b506060820151610e636060850182610e07565b50505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60048110610ea757610ea6610e69565b5b50565b5f819050610eb782610e96565b919050565b5f610ec682610eaa565b9050919050565b610ed681610ebc565b82525050565b5f60e082019050610eef5f8301876109c7565b610efc6020830186610930565b610f096040830185610e16565b610f1660c0830184610ecd565b95945050505050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f610f5182610f1f565b610f5b8185610f29565b9350610f6b818560208601610f39565b610f7481610a32565b840191505092915050565b5f6020820190508181035f830152610f978184610f47565b905092915050565b5f8160f81b9050919050565b5f610fb582610f9f565b9050919050565b610fcd610fc882610bd2565b610fab565b82525050565b5f8160601b9050919050565b5f610fe982610fd3565b9050919050565b5f610ffa82610fdf565b9050919050565b61101261100d82610c27565b610ff0565b82525050565b5f8160c01b9050919050565b5f61102e82611018565b9050919050565b611046611041826109ef565b611024565b82525050565b5f819050919050565b61106661106182610927565b61104c565b82525050565b5f611077828a610fbc565b6001820191506110878289611001565b6014820191506110978288611035565b6008820191506110a78287611035565b6008820191506110b78286611055565b6020820191506110c78285611055565b6020820191506110d78284610db4565b60208201915081905098975050505050505050565b5f6110f78285610db4565b6020820191506111078284610db4565b6020820191508190509392505050565b61112081610c27565b82525050565b61112f81610bd2565b82525050565b5f60c0820190506111485f830189611117565b6111556020830188611126565b6111626040830187611117565b61116f60608301866109c7565b61117c6080830185610930565b61118960a0830184610a02565b979650505050505050565b5f61119f8285611001565b6014820191506111af8284611055565b6020820191508190509392505050565b5f6111c982610927565b91506111d483610927565b92508282019050808211156111ec576111eb610d1e565b5b92915050565b5f819050919050565b5f61121561121061120b846109ef565b6111f2565b610927565b9050919050565b611225816111fb565b82525050565b5f60408201905061123e5f830185610930565b61124b602083018461121c565b9392505050565b5f61125c826109ef565b9150611267836109ef565b9250828203905067ffffffffffffffff81111561128757611286610d1e565b5b92915050565b5f611297826109ef565b91506112a2836109ef565b9250828201905067ffffffffffffffff8111156112c2576112c1610d1e565b5b92915050565b5f6112d38288611035565b6008820191506112e38287611035565b6008820191506112f38286611035565b6008820191506113038285611035565b6008820191506113138284611035565b6008820191508190509695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b5f81905092915050565b5f61136782610f1f565b6113718185611353565b9350611381818560208601610f39565b80840191505092915050565b5f611398828561135d565b91506113a4828461135d565b9150819050939250505056fea26469706673582212202c505ecc93a7241b2f2bd0313c9b4557b0c122cb4975714e7c879d28af5bd45664736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01\x08W_5`\xE0\x1C\x80c\x7F\xA3\xA4\x0E\x11a\0\xA0W\x80c\xD5q\x9D\xC2\x11a\0oW\x80c\xD5q\x9D\xC2\x14a\x02\x82W\x80c\xD9\xDDg\xAB\x14a\x02\xB2W\x80c\xE1\xD6j\xFE\x14a\x02\xE2W\x80c\xE8\xEB\x1D\xC3\x14a\x02\xFEW\x80c\xEC\xA0g\xAD\x14a\x03\x1CWa\x01\x08V[\x80c\x7F\xA3\xA4\x0E\x14a\x02\x0CW\x80c\xA7\xB5\x1D\x19\x14a\x02*W\x80c\xAD\x9C\x0C.\x14a\x02FW\x80c\xB7R\xA7\xD1\x14a\x02dWa\x01\x08V[\x80c*\xA6\xDA\xE4\x11a\0\xDCW\x80c*\xA6\xDA\xE4\x14a\x01\x96W\x80c/\x1E\xC5\xE9\x14a\x01\xB4W\x80cO5\x9A7\x14a\x01\xD2W\x80c\\\x88\xE8s\x14a\x01\xF0Wa\x01\x08V[\x80b\x84\x12\x0C\x14a\x01\x0CW\x80c\x04\xDD{\x8B\x14a\x01*W\x80c\x06\xF10V\x14a\x01HW\x80c\x16\xBFUy\x14a\x01fW[_\x80\xFD[a\x01\x14a\x03:V[`@Qa\x01!\x91\x90a\t?V[`@Q\x80\x91\x03\x90\xF3[a\x012a\x03FV[`@Qa\x01?\x91\x90a\t?V[`@Q\x80\x91\x03\x90\xF3[a\x01Pa\x03KV[`@Qa\x01]\x91\x90a\t?V[`@Q\x80\x91\x03\x90\xF3[a\x01\x80`\x04\x806\x03\x81\x01\x90a\x01{\x91\x90a\t\x93V[a\x03WV[`@Qa\x01\x8D\x91\x90a\t\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x01\x9Ea\x03wV[`@Qa\x01\xAB\x91\x90a\t?V[`@Q\x80\x91\x03\x90\xF3[a\x01\xBCa\x03}V[`@Qa\x01\xC9\x91\x90a\n\x11V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDAa\x03\x83V[`@Qa\x01\xE7\x91\x90a\n\x11V[`@Q\x80\x91\x03\x90\xF3[a\x02\n`\x04\x806\x03\x81\x01\x90a\x02\x05\x91\x90a\x0BfV[a\x03\x88V[\0[a\x02\x14a\x05\rV[`@Qa\x02!\x91\x90a\t?V[`@Q\x80\x91\x03\x90\xF3[a\x02D`\x04\x806\x03\x81\x01\x90a\x02?\x91\x90a\x0CbV[a\x05\x13V[\0[a\x02Na\x06iV[`@Qa\x02[\x91\x90a\n\x11V[`@Q\x80\x91\x03\x90\xF3[a\x02la\x06oV[`@Qa\x02y\x91\x90a\n\x11V[`@Q\x80\x91\x03\x90\xF3[a\x02\x9C`\x04\x806\x03\x81\x01\x90a\x02\x97\x91\x90a\t\x93V[a\x06vV[`@Qa\x02\xA9\x91\x90a\t\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x02\xCC`\x04\x806\x03\x81\x01\x90a\x02\xC7\x91\x90a\t\x93V[a\x06\x96V[`@Qa\x02\xD9\x91\x90a\t\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x02\xFC`\x04\x806\x03\x81\x01\x90a\x02\xF7\x91\x90a\x0C\xCEV[a\x06\xBBV[\0[a\x03\x06a\x06\xEDV[`@Qa\x03\x13\x91\x90a\n\x11V[`@Q\x80\x91\x03\x90\xF3[a\x03$a\x06\xF4V[`@Qa\x031\x91\x90a\t?V[`@Q\x80\x91\x03\x90\xF3[_`\x03\x80T\x90P\x90P\x90V[_T\x81V[_`\x03\x80T\x90P\x90P\x90V[`\x03\x81\x81T\x81\x10a\x03fW_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[`\x01T\x81V[a\x0E\x10\x81V[`\x0C\x81V[\x81_\x81\x90UP\x80`\x01\x81\x90UP_`\x02\x80T\x90P\x90P_\x80a\x03\xAA\x86\x84a\x07\0V[\x91P\x91P_`\x03\x80T\x90P\x90P_\x80_\x1B\x90P_\x82\x11\x15a\x03\xF1W`\x03`\x01\x83a\x03\xD4\x91\x90a\rKV[\x81T\x81\x10a\x03\xE5Wa\x03\xE4a\r~V[[\x90_R` _ \x01T\x90P[_\x80_\x1B\x90P_\x86\x11\x15a\x04+W`\x02`\x01\x87a\x04\x0E\x91\x90a\rKV[\x81T\x81\x10a\x04\x1FWa\x04\x1Ea\r~V[[\x90_R` _ \x01T\x90P[_\x82\x86\x83`@Q` \x01a\x04A\x93\x92\x91\x90a\r\xCBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x03\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x86`\x04\x81\x90UP\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85`\x04T\x8A`\x01`@Qa\x04\xC1\x94\x93\x92\x91\x90a\x0E\xDCV[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8B`@Qa\x04\xF9\x91\x90a\x0F\x7FV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPV[`\x04T\x81V[_`\x02\x80T\x90P\x90P_\x82\x80Q\x90` \x01 \x90P_\x85\x85CB\x86H\x87`@Q` \x01a\x05E\x97\x96\x95\x94\x93\x92\x91\x90a\x10lV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_\x80_\x1B\x90P_\x84\x11\x15a\x05\x97W`\x02`\x01\x85a\x05z\x91\x90a\rKV[\x81T\x81\x10a\x05\x8BWa\x05\x8Aa\r~V[[\x90_R` _ \x01T\x90P[`\x02\x81\x83`@Q` \x01a\x05\xAC\x92\x91\x90a\x10\xECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x80\x84\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE10\x8A\x8A\x88HB`@Qa\x06 \x96\x95\x94\x93\x92\x91\x90a\x115V[`@Q\x80\x91\x03\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x06X\x91\x90a\x0F\x7FV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[a\x1C \x81V[b\x01Q\x80\x81V[`\x02\x81\x81T\x81\x10a\x06\x85W_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[_`\x03\x82\x81T\x81\x10a\x06\xABWa\x06\xAAa\r~V[[\x90_R` _ \x01T\x90P\x91\x90PV[a\x06\xE8`\x0C\x84\x84\x84`@Q` \x01a\x06\xD4\x92\x91\x90a\x11\x94V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x05\x13V[PPPV[b\x01\xCC\xCC\x81V[_`\x02\x80T\x90P\x90P\x90V[_a\x07\ta\x08\xDBV[_\x84Q`(a\x07\x18\x91\x90a\x11\xBFV[\x90Pb\x01\xCC\xCCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\x07qW\x80b\x01\xCC\xCC`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07h\x92\x91\x90a\x12+V[`@Q\x80\x91\x03\x90\xFD[a\x07ya\x08\xDBV[b\x01Q\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15a\x07\xBCWb\x01Q\x80Ba\x07\x9D\x91\x90a\x12RV[\x81_\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[a\x0E\x10Ba\x07\xCA\x91\x90a\x12\x8DV[\x81` \x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x1C g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x11\x15a\x08+Wa\x1C Ca\x08\x0B\x91\x90a\x12RV[\x81`@\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[`\x0CCa\x088\x91\x90a\x12\x8DV[\x81``\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x89`@Q` \x01a\x08\x80\x95\x94\x93\x92\x91\x90a\x12\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\x08\xA3Wa\x08\xA2a\x13&V[[\x80\x87`@Q` \x01a\x08\xB6\x92\x91\x90a\x13\x8DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[`@Q\x80`\x80\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_\x81\x90P\x91\x90PV[a\t9\x81a\t'V[\x82RPPV[_` \x82\x01\x90Pa\tR_\x83\x01\x84a\t0V[\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[a\tr\x81a\t'V[\x81\x14a\t|W_\x80\xFD[PV[_\x815\x90Pa\t\x8D\x81a\tiV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\t\xA8Wa\t\xA7a\taV[[_a\t\xB5\x84\x82\x85\x01a\t\x7FV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\t\xD0\x81a\t\xBEV[\x82RPPV[_` \x82\x01\x90Pa\t\xE9_\x83\x01\x84a\t\xC7V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\n\x0B\x81a\t\xEFV[\x82RPPV[_` \x82\x01\x90Pa\n$_\x83\x01\x84a\n\x02V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\nx\x82a\n2V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\n\x97Wa\n\x96a\nBV[[\x80`@RPPPV[_a\n\xA9a\tXV[\x90Pa\n\xB5\x82\x82a\noV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\n\xD4Wa\n\xD3a\nBV[[a\n\xDD\x82a\n2V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0B\na\x0B\x05\x84a\n\xBAV[a\n\xA0V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0B&Wa\x0B%a\n.V[[a\x0B1\x84\x82\x85a\n\xEAV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0BMWa\x0BLa\n*V[[\x815a\x0B]\x84\x82` \x86\x01a\n\xF8V[\x91PP\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x0B}Wa\x0B|a\taV[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x9AWa\x0B\x99a\teV[[a\x0B\xA6\x86\x82\x87\x01a\x0B9V[\x93PP` a\x0B\xB7\x86\x82\x87\x01a\t\x7FV[\x92PP`@a\x0B\xC8\x86\x82\x87\x01a\t\x7FV[\x91PP\x92P\x92P\x92V[_`\xFF\x82\x16\x90P\x91\x90PV[a\x0B\xE7\x81a\x0B\xD2V[\x81\x14a\x0B\xF1W_\x80\xFD[PV[_\x815\x90Pa\x0C\x02\x81a\x0B\xDEV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0C1\x82a\x0C\x08V[\x90P\x91\x90PV[a\x0CA\x81a\x0C'V[\x81\x14a\x0CKW_\x80\xFD[PV[_\x815\x90Pa\x0C\\\x81a\x0C8V[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x0CyWa\x0Cxa\taV[[_a\x0C\x86\x86\x82\x87\x01a\x0B\xF4V[\x93PP` a\x0C\x97\x86\x82\x87\x01a\x0CNV[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xB8Wa\x0C\xB7a\teV[[a\x0C\xC4\x86\x82\x87\x01a\x0B9V[\x91PP\x92P\x92P\x92V[_\x80_``\x84\x86\x03\x12\x15a\x0C\xE5Wa\x0C\xE4a\taV[[_a\x0C\xF2\x86\x82\x87\x01a\x0CNV[\x93PP` a\r\x03\x86\x82\x87\x01a\x0CNV[\x92PP`@a\r\x14\x86\x82\x87\x01a\t\x7FV[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\rU\x82a\t'V[\x91Pa\r`\x83a\t'V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\rxWa\rwa\r\x1EV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[a\r\xC5a\r\xC0\x82a\t\xBEV[a\r\xABV[\x82RPPV[_a\r\xD6\x82\x86a\r\xB4V[` \x82\x01\x91Pa\r\xE6\x82\x85a\r\xB4V[` \x82\x01\x91Pa\r\xF6\x82\x84a\r\xB4V[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[a\x0E\x10\x81a\t\xEFV[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\x0E*_\x85\x01\x82a\x0E\x07V[P` \x82\x01Qa\x0E=` \x85\x01\x82a\x0E\x07V[P`@\x82\x01Qa\x0EP`@\x85\x01\x82a\x0E\x07V[P``\x82\x01Qa\x0Ec``\x85\x01\x82a\x0E\x07V[PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x0E\xA7Wa\x0E\xA6a\x0EiV[[PV[_\x81\x90Pa\x0E\xB7\x82a\x0E\x96V[\x91\x90PV[_a\x0E\xC6\x82a\x0E\xAAV[\x90P\x91\x90PV[a\x0E\xD6\x81a\x0E\xBCV[\x82RPPV[_`\xE0\x82\x01\x90Pa\x0E\xEF_\x83\x01\x87a\t\xC7V[a\x0E\xFC` \x83\x01\x86a\t0V[a\x0F\t`@\x83\x01\x85a\x0E\x16V[a\x0F\x16`\xC0\x83\x01\x84a\x0E\xCDV[\x95\x94PPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x0FQ\x82a\x0F\x1FV[a\x0F[\x81\x85a\x0F)V[\x93Pa\x0Fk\x81\x85` \x86\x01a\x0F9V[a\x0Ft\x81a\n2V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0F\x97\x81\x84a\x0FGV[\x90P\x92\x91PPV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\x0F\xB5\x82a\x0F\x9FV[\x90P\x91\x90PV[a\x0F\xCDa\x0F\xC8\x82a\x0B\xD2V[a\x0F\xABV[\x82RPPV[_\x81``\x1B\x90P\x91\x90PV[_a\x0F\xE9\x82a\x0F\xD3V[\x90P\x91\x90PV[_a\x0F\xFA\x82a\x0F\xDFV[\x90P\x91\x90PV[a\x10\x12a\x10\r\x82a\x0C'V[a\x0F\xF0V[\x82RPPV[_\x81`\xC0\x1B\x90P\x91\x90PV[_a\x10.\x82a\x10\x18V[\x90P\x91\x90PV[a\x10Fa\x10A\x82a\t\xEFV[a\x10$V[\x82RPPV[_\x81\x90P\x91\x90PV[a\x10fa\x10a\x82a\t'V[a\x10LV[\x82RPPV[_a\x10w\x82\x8Aa\x0F\xBCV[`\x01\x82\x01\x91Pa\x10\x87\x82\x89a\x10\x01V[`\x14\x82\x01\x91Pa\x10\x97\x82\x88a\x105V[`\x08\x82\x01\x91Pa\x10\xA7\x82\x87a\x105V[`\x08\x82\x01\x91Pa\x10\xB7\x82\x86a\x10UV[` \x82\x01\x91Pa\x10\xC7\x82\x85a\x10UV[` \x82\x01\x91Pa\x10\xD7\x82\x84a\r\xB4V[` \x82\x01\x91P\x81\x90P\x98\x97PPPPPPPPV[_a\x10\xF7\x82\x85a\r\xB4V[` \x82\x01\x91Pa\x11\x07\x82\x84a\r\xB4V[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[a\x11 \x81a\x0C'V[\x82RPPV[a\x11/\x81a\x0B\xD2V[\x82RPPV[_`\xC0\x82\x01\x90Pa\x11H_\x83\x01\x89a\x11\x17V[a\x11U` \x83\x01\x88a\x11&V[a\x11b`@\x83\x01\x87a\x11\x17V[a\x11o``\x83\x01\x86a\t\xC7V[a\x11|`\x80\x83\x01\x85a\t0V[a\x11\x89`\xA0\x83\x01\x84a\n\x02V[\x97\x96PPPPPPPV[_a\x11\x9F\x82\x85a\x10\x01V[`\x14\x82\x01\x91Pa\x11\xAF\x82\x84a\x10UV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[_a\x11\xC9\x82a\t'V[\x91Pa\x11\xD4\x83a\t'V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x11\xECWa\x11\xEBa\r\x1EV[[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x12\x15a\x12\x10a\x12\x0B\x84a\t\xEFV[a\x11\xF2V[a\t'V[\x90P\x91\x90PV[a\x12%\x81a\x11\xFBV[\x82RPPV[_`@\x82\x01\x90Pa\x12>_\x83\x01\x85a\t0V[a\x12K` \x83\x01\x84a\x12\x1CV[\x93\x92PPPV[_a\x12\\\x82a\t\xEFV[\x91Pa\x12g\x83a\t\xEFV[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x87Wa\x12\x86a\r\x1EV[[\x92\x91PPV[_a\x12\x97\x82a\t\xEFV[\x91Pa\x12\xA2\x83a\t\xEFV[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xC2Wa\x12\xC1a\r\x1EV[[\x92\x91PPV[_a\x12\xD3\x82\x88a\x105V[`\x08\x82\x01\x91Pa\x12\xE3\x82\x87a\x105V[`\x08\x82\x01\x91Pa\x12\xF3\x82\x86a\x105V[`\x08\x82\x01\x91Pa\x13\x03\x82\x85a\x105V[`\x08\x82\x01\x91Pa\x13\x13\x82\x84a\x105V[`\x08\x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[_\x81\x90P\x92\x91PPV[_a\x13g\x82a\x0F\x1FV[a\x13q\x81\x85a\x13SV[\x93Pa\x13\x81\x81\x85` \x86\x01a\x0F9V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x13\x98\x82\x85a\x13]V[\x91Pa\x13\xA4\x82\x84a\x13]V[\x91P\x81\x90P\x93\x92PPPV\xFE\xA2dipfsX\"\x12 ,P^\xCC\x93\xA7$\x1B/+\xD01<\x9BEW\xB0\xC1\"\xCBIuqN|\x87\x9D(\xAF[\xD4VdsolcC\0\x08\x19\x003",
    );
    /**Custom error with signature `DataTooLarge(uint256,uint256)` and selector `0x4634691b`.
```solidity
error DataTooLarge(uint256 dataLength, uint256 maxDataLength);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DataTooLarge {
        #[allow(missing_docs)]
        pub dataLength: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxDataLength: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<DataTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: DataTooLarge) -> Self {
                (value.dataLength, value.maxDataLength)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DataTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    dataLength: tuple.0,
                    maxDataLength: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DataTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DataTooLarge(uint256,uint256)";
            const SELECTOR: [u8; 4] = [70u8, 52u8, 105u8, 27u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.dataLength),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxDataLength),
                )
            }
        }
    };
    /**Event with signature `InboxMessageDelivered(uint256,bytes)` and selector `0xff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b`.
```solidity
event InboxMessageDelivered(uint256 indexed messageNum, bytes data);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct InboxMessageDelivered {
        #[allow(missing_docs)]
        pub messageNum: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for InboxMessageDelivered {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "InboxMessageDelivered(uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                255u8,
                100u8,
                144u8,
                95u8,
                115u8,
                166u8,
                127u8,
                181u8,
                148u8,
                224u8,
                249u8,
                64u8,
                168u8,
                7u8,
                90u8,
                134u8,
                13u8,
                180u8,
                137u8,
                173u8,
                153u8,
                30u8,
                3u8,
                47u8,
                72u8,
                200u8,
                17u8,
                35u8,
                235u8,
                82u8,
                214u8,
                11u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    messageNum: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.messageNum.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.messageNum);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for InboxMessageDelivered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&InboxMessageDelivered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &InboxMessageDelivered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `MessageDelivered(uint256,bytes32,address,uint8,address,bytes32,uint256,uint64)` and selector `0x5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1`.
```solidity
event MessageDelivered(uint256 indexed messageIndex, bytes32 indexed beforeInboxAcc, address inbox, uint8 kind, address sender, bytes32 messageDataHash, uint256 baseFeeL1, uint64 timestamp);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MessageDelivered {
        #[allow(missing_docs)]
        pub messageIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub beforeInboxAcc: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub inbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub kind: u8,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub messageDataHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub baseFeeL1: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub timestamp: u64,
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
        impl alloy_sol_types::SolEvent for MessageDelivered {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "MessageDelivered(uint256,bytes32,address,uint8,address,bytes32,uint256,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                94u8,
                60u8,
                19u8,
                17u8,
                234u8,
                68u8,
                38u8,
                100u8,
                232u8,
                177u8,
                97u8,
                27u8,
                250u8,
                190u8,
                246u8,
                89u8,
                18u8,
                14u8,
                167u8,
                160u8,
                162u8,
                207u8,
                192u8,
                102u8,
                119u8,
                0u8,
                190u8,
                188u8,
                105u8,
                203u8,
                255u8,
                225u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    messageIndex: topics.1,
                    beforeInboxAcc: topics.2,
                    inbox: data.0,
                    kind: data.1,
                    sender: data.2,
                    messageDataHash: data.3,
                    baseFeeL1: data.4,
                    timestamp: data.5,
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
                        &self.inbox,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.kind),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.messageDataHash),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseFeeL1),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.timestamp),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.messageIndex.clone(),
                    self.beforeInboxAcc.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.messageIndex);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.beforeInboxAcc);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MessageDelivered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MessageDelivered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MessageDelivered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SequencerBatchData(uint256,bytes)` and selector `0xfe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c20`.
```solidity
event SequencerBatchData(uint256 indexed batchSequenceNumber, bytes data);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SequencerBatchData {
        #[allow(missing_docs)]
        pub batchSequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for SequencerBatchData {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "SequencerBatchData(uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                254u8,
                50u8,
                92u8,
                161u8,
                239u8,
                228u8,
                197u8,
                193u8,
                6u8,
                44u8,
                152u8,
                28u8,
                62u8,
                231u8,
                75u8,
                120u8,
                29u8,
                235u8,
                228u8,
                234u8,
                148u8,
                64u8,
                48u8,
                106u8,
                150u8,
                210u8,
                165u8,
                87u8,
                89u8,
                198u8,
                108u8,
                32u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    batchSequenceNumber: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.batchSequenceNumber.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.batchSequenceNumber,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SequencerBatchData {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SequencerBatchData> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SequencerBatchData) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SequencerBatchDelivered(uint256,bytes32,bytes32,bytes32,uint256,(uint64,uint64,uint64,uint64),uint8)` and selector `0x7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7`.
```solidity
event SequencerBatchDelivered(uint256 indexed batchSequenceNumber, bytes32 indexed beforeAcc, bytes32 indexed afterAcc, bytes32 delayedAcc, uint256 afterDelayedMessagesRead, IBridge.TimeBounds timeBounds, IBridge.BatchDataLocation dataLocation);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SequencerBatchDelivered {
        #[allow(missing_docs)]
        pub batchSequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub beforeAcc: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub afterAcc: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub delayedAcc: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub timeBounds: <IBridge::TimeBounds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub dataLocation: <IBridge::BatchDataLocation as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for SequencerBatchDelivered {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                IBridge::TimeBounds,
                IBridge::BatchDataLocation,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "SequencerBatchDelivered(uint256,bytes32,bytes32,bytes32,uint256,(uint64,uint64,uint64,uint64),uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                115u8,
                148u8,
                244u8,
                161u8,
                154u8,
                19u8,
                199u8,
                185u8,
                43u8,
                91u8,
                183u8,
                16u8,
                51u8,
                36u8,
                83u8,
                5u8,
                148u8,
                110u8,
                247u8,
                132u8,
                82u8,
                247u8,
                180u8,
                152u8,
                106u8,
                193u8,
                57u8,
                11u8,
                93u8,
                244u8,
                235u8,
                215u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    batchSequenceNumber: topics.1,
                    beforeAcc: topics.2,
                    afterAcc: topics.3,
                    delayedAcc: data.0,
                    afterDelayedMessagesRead: data.1,
                    timeBounds: data.2,
                    dataLocation: data.3,
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.delayedAcc),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.afterDelayedMessagesRead,
                    ),
                    <IBridge::TimeBounds as alloy_sol_types::SolType>::tokenize(
                        &self.timeBounds,
                    ),
                    <IBridge::BatchDataLocation as alloy_sol_types::SolType>::tokenize(
                        &self.dataLocation,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.batchSequenceNumber.clone(),
                    self.beforeAcc.clone(),
                    self.afterAcc.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.batchSequenceNumber,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.beforeAcc);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.afterAcc);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SequencerBatchDelivered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SequencerBatchDelivered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SequencerBatchDelivered,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(uint256 chainId, string chainConfig);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub chainConfig: alloy::sol_types::private::String,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::String,
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
                    (value.chainId, value.chainConfig)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainId: tuple.0,
                        chainConfig: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
            );
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
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.chainConfig,
                    ),
                )
            }
        }
    };
    /**Function with signature `batchCount()` and selector `0x06f13056`.
```solidity
function batchCount() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct batchCountCall {}
    ///Container type for the return parameters of the [`batchCount()`](batchCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct batchCountReturn {
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
            impl ::core::convert::From<batchCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: batchCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for batchCountCall {
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
            impl ::core::convert::From<batchCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: batchCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for batchCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for batchCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = batchCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "batchCount()";
            const SELECTOR: [u8; 4] = [6u8, 241u8, 48u8, 86u8];
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
    /**Function with signature `delayBlocks()` and selector `0xad9c0c2e`.
```solidity
function delayBlocks() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delayBlocksCall {}
    ///Container type for the return parameters of the [`delayBlocks()`](delayBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delayBlocksReturn {
        #[allow(missing_docs)]
        pub _0: u64,
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
            impl ::core::convert::From<delayBlocksCall> for UnderlyingRustTuple<'_> {
                fn from(value: delayBlocksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delayBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delayBlocksReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delayBlocksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delayBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delayBlocksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delayBlocksReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delayBlocks()";
            const SELECTOR: [u8; 4] = [173u8, 156u8, 12u8, 46u8];
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
    /**Function with signature `delaySeconds()` and selector `0xb752a7d1`.
```solidity
function delaySeconds() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delaySecondsCall {}
    ///Container type for the return parameters of the [`delaySeconds()`](delaySecondsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delaySecondsReturn {
        #[allow(missing_docs)]
        pub _0: u64,
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
            impl ::core::convert::From<delaySecondsCall> for UnderlyingRustTuple<'_> {
                fn from(value: delaySecondsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delaySecondsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delaySecondsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delaySecondsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delaySecondsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delaySecondsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delaySecondsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delaySeconds()";
            const SELECTOR: [u8; 4] = [183u8, 82u8, 167u8, 209u8];
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
    /**Function with signature `delayedInboxAccs(uint256)` and selector `0xd5719dc2`.
```solidity
function delayedInboxAccs(uint256) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delayedInboxAccsCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`delayedInboxAccs(uint256)`](delayedInboxAccsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delayedInboxAccsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<delayedInboxAccsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: delayedInboxAccsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delayedInboxAccsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delayedInboxAccsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: delayedInboxAccsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delayedInboxAccsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delayedInboxAccsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delayedInboxAccsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delayedInboxAccs(uint256)";
            const SELECTOR: [u8; 4] = [213u8, 113u8, 157u8, 194u8];
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
    /**Function with signature `delayedMessageCount()` and selector `0xeca067ad`.
```solidity
function delayedMessageCount() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delayedMessageCountCall {}
    ///Container type for the return parameters of the [`delayedMessageCount()`](delayedMessageCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delayedMessageCountReturn {
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
            impl ::core::convert::From<delayedMessageCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: delayedMessageCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delayedMessageCountCall {
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
            impl ::core::convert::From<delayedMessageCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: delayedMessageCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delayedMessageCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delayedMessageCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delayedMessageCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delayedMessageCount()";
            const SELECTOR: [u8; 4] = [236u8, 160u8, 103u8, 173u8];
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
    /**Function with signature `deliverMessage(uint8,address,bytes)` and selector `0xa7b51d19`.
```solidity
function deliverMessage(uint8 kind, address sender, bytes memory messageData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deliverMessageCall {
        #[allow(missing_docs)]
        pub kind: u8,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub messageData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`deliverMessage(uint8,address,bytes)`](deliverMessageCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deliverMessageReturn {}
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
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
            impl ::core::convert::From<deliverMessageCall> for UnderlyingRustTuple<'_> {
                fn from(value: deliverMessageCall) -> Self {
                    (value.kind, value.sender, value.messageData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deliverMessageCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kind: tuple.0,
                        sender: tuple.1,
                        messageData: tuple.2,
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
            impl ::core::convert::From<deliverMessageReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deliverMessageReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deliverMessageReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deliverMessageCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deliverMessageReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deliverMessage(uint8,address,bytes)";
            const SELECTOR: [u8; 4] = [167u8, 181u8, 29u8, 25u8];
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.kind),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.messageData,
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
    /**Function with signature `depositEth(address,address,uint256)` and selector `0xe1d66afe`.
```solidity
function depositEth(address src, address dest, uint256 value) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositEthCall {
        #[allow(missing_docs)]
        pub src: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub dest: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`depositEth(address,address,uint256)`](depositEthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositEthReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<depositEthCall> for UnderlyingRustTuple<'_> {
                fn from(value: depositEthCall) -> Self {
                    (value.src, value.dest, value.value)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositEthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        src: tuple.0,
                        dest: tuple.1,
                        value: tuple.2,
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
            impl ::core::convert::From<depositEthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: depositEthReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositEthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositEthCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositEthReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositEth(address,address,uint256)";
            const SELECTOR: [u8; 4] = [225u8, 214u8, 106u8, 254u8];
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
                        &self.src,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.dest,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
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
    /**Function with signature `futureBlocks()` and selector `0x4f359a37`.
```solidity
function futureBlocks() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct futureBlocksCall {}
    ///Container type for the return parameters of the [`futureBlocks()`](futureBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct futureBlocksReturn {
        #[allow(missing_docs)]
        pub _0: u64,
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
            impl ::core::convert::From<futureBlocksCall> for UnderlyingRustTuple<'_> {
                fn from(value: futureBlocksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for futureBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<futureBlocksReturn> for UnderlyingRustTuple<'_> {
                fn from(value: futureBlocksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for futureBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for futureBlocksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = futureBlocksReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "futureBlocks()";
            const SELECTOR: [u8; 4] = [79u8, 53u8, 154u8, 55u8];
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
    /**Function with signature `futureSeconds()` and selector `0x2f1ec5e9`.
```solidity
function futureSeconds() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct futureSecondsCall {}
    ///Container type for the return parameters of the [`futureSeconds()`](futureSecondsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct futureSecondsReturn {
        #[allow(missing_docs)]
        pub _0: u64,
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
            impl ::core::convert::From<futureSecondsCall> for UnderlyingRustTuple<'_> {
                fn from(value: futureSecondsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for futureSecondsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<futureSecondsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: futureSecondsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for futureSecondsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for futureSecondsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = futureSecondsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "futureSeconds()";
            const SELECTOR: [u8; 4] = [47u8, 30u8, 197u8, 233u8];
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
    /**Function with signature `inboxAccs(uint256)` and selector `0xd9dd67ab`.
```solidity
function inboxAccs(uint256 index) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct inboxAccsCall {
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`inboxAccs(uint256)`](inboxAccsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct inboxAccsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<inboxAccsCall> for UnderlyingRustTuple<'_> {
                fn from(value: inboxAccsCall) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for inboxAccsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<inboxAccsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: inboxAccsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for inboxAccsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for inboxAccsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = inboxAccsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "inboxAccs(uint256)";
            const SELECTOR: [u8; 4] = [217u8, 221u8, 103u8, 171u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
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
    /**Function with signature `lastProcessedSequencingBlock()` and selector `0x04dd7b8b`.
```solidity
function lastProcessedSequencingBlock() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProcessedSequencingBlockCall {}
    ///Container type for the return parameters of the [`lastProcessedSequencingBlock()`](lastProcessedSequencingBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProcessedSequencingBlockReturn {
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
            impl ::core::convert::From<lastProcessedSequencingBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastProcessedSequencingBlockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastProcessedSequencingBlockCall {
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
            impl ::core::convert::From<lastProcessedSequencingBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastProcessedSequencingBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastProcessedSequencingBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastProcessedSequencingBlockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastProcessedSequencingBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastProcessedSequencingBlock()";
            const SELECTOR: [u8; 4] = [4u8, 221u8, 123u8, 139u8];
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
    /**Function with signature `lastProcessedSettlementBlock()` and selector `0x2aa6dae4`.
```solidity
function lastProcessedSettlementBlock() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProcessedSettlementBlockCall {}
    ///Container type for the return parameters of the [`lastProcessedSettlementBlock()`](lastProcessedSettlementBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProcessedSettlementBlockReturn {
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
            impl ::core::convert::From<lastProcessedSettlementBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastProcessedSettlementBlockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastProcessedSettlementBlockCall {
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
            impl ::core::convert::From<lastProcessedSettlementBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastProcessedSettlementBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastProcessedSettlementBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastProcessedSettlementBlockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastProcessedSettlementBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastProcessedSettlementBlock()";
            const SELECTOR: [u8; 4] = [42u8, 166u8, 218u8, 228u8];
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
    /**Function with signature `maxDataSize()` and selector `0xe8eb1dc3`.
```solidity
function maxDataSize() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxDataSizeCall {}
    ///Container type for the return parameters of the [`maxDataSize()`](maxDataSizeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxDataSizeReturn {
        #[allow(missing_docs)]
        pub _0: u64,
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
            impl ::core::convert::From<maxDataSizeCall> for UnderlyingRustTuple<'_> {
                fn from(value: maxDataSizeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for maxDataSizeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<maxDataSizeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: maxDataSizeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for maxDataSizeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for maxDataSizeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = maxDataSizeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "maxDataSize()";
            const SELECTOR: [u8; 4] = [232u8, 235u8, 29u8, 195u8];
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
    /**Function with signature `postBatch(bytes,uint256,uint256)` and selector `0x5c88e873`.
```solidity
function postBatch(bytes memory data, uint256 _lastProcessedSequencingBlock, uint256 _lastProcessedSettlementBlock) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postBatchCall {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _lastProcessedSequencingBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _lastProcessedSettlementBlock: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`postBatch(bytes,uint256,uint256)`](postBatchCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postBatchReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<postBatchCall> for UnderlyingRustTuple<'_> {
                fn from(value: postBatchCall) -> Self {
                    (
                        value.data,
                        value._lastProcessedSequencingBlock,
                        value._lastProcessedSettlementBlock,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for postBatchCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        data: tuple.0,
                        _lastProcessedSequencingBlock: tuple.1,
                        _lastProcessedSettlementBlock: tuple.2,
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
            impl ::core::convert::From<postBatchReturn> for UnderlyingRustTuple<'_> {
                fn from(value: postBatchReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for postBatchReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for postBatchCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = postBatchReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "postBatch(bytes,uint256,uint256)";
            const SELECTOR: [u8; 4] = [92u8, 136u8, 232u8, 115u8];
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._lastProcessedSequencingBlock,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._lastProcessedSettlementBlock,
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
    /**Function with signature `sequencerInboxAccs(uint256)` and selector `0x16bf5579`.
```solidity
function sequencerInboxAccs(uint256) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sequencerInboxAccsCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`sequencerInboxAccs(uint256)`](sequencerInboxAccsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sequencerInboxAccsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<sequencerInboxAccsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: sequencerInboxAccsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sequencerInboxAccsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<sequencerInboxAccsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: sequencerInboxAccsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sequencerInboxAccsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for sequencerInboxAccsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = sequencerInboxAccsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "sequencerInboxAccs(uint256)";
            const SELECTOR: [u8; 4] = [22u8, 191u8, 85u8, 121u8];
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
    /**Function with signature `sequencerMessageCount()` and selector `0x0084120c`.
```solidity
function sequencerMessageCount() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sequencerMessageCountCall {}
    ///Container type for the return parameters of the [`sequencerMessageCount()`](sequencerMessageCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sequencerMessageCountReturn {
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
            impl ::core::convert::From<sequencerMessageCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: sequencerMessageCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sequencerMessageCountCall {
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
            impl ::core::convert::From<sequencerMessageCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: sequencerMessageCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sequencerMessageCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for sequencerMessageCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = sequencerMessageCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "sequencerMessageCount()";
            const SELECTOR: [u8; 4] = [0u8, 132u8, 18u8, 12u8];
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
    /**Function with signature `totalDelayedMessagesRead()` and selector `0x7fa3a40e`.
```solidity
function totalDelayedMessagesRead() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalDelayedMessagesReadCall {}
    ///Container type for the return parameters of the [`totalDelayedMessagesRead()`](totalDelayedMessagesReadCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalDelayedMessagesReadReturn {
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
            impl ::core::convert::From<totalDelayedMessagesReadCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalDelayedMessagesReadCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalDelayedMessagesReadCall {
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
            impl ::core::convert::From<totalDelayedMessagesReadReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalDelayedMessagesReadReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalDelayedMessagesReadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalDelayedMessagesReadCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = totalDelayedMessagesReadReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "totalDelayedMessagesRead()";
            const SELECTOR: [u8; 4] = [127u8, 163u8, 164u8, 14u8];
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
    ///Container for all the [`Rollup`](self) function calls.
    pub enum RollupCalls {
        #[allow(missing_docs)]
        batchCount(batchCountCall),
        #[allow(missing_docs)]
        delayBlocks(delayBlocksCall),
        #[allow(missing_docs)]
        delaySeconds(delaySecondsCall),
        #[allow(missing_docs)]
        delayedInboxAccs(delayedInboxAccsCall),
        #[allow(missing_docs)]
        delayedMessageCount(delayedMessageCountCall),
        #[allow(missing_docs)]
        deliverMessage(deliverMessageCall),
        #[allow(missing_docs)]
        depositEth(depositEthCall),
        #[allow(missing_docs)]
        futureBlocks(futureBlocksCall),
        #[allow(missing_docs)]
        futureSeconds(futureSecondsCall),
        #[allow(missing_docs)]
        inboxAccs(inboxAccsCall),
        #[allow(missing_docs)]
        lastProcessedSequencingBlock(lastProcessedSequencingBlockCall),
        #[allow(missing_docs)]
        lastProcessedSettlementBlock(lastProcessedSettlementBlockCall),
        #[allow(missing_docs)]
        maxDataSize(maxDataSizeCall),
        #[allow(missing_docs)]
        postBatch(postBatchCall),
        #[allow(missing_docs)]
        sequencerInboxAccs(sequencerInboxAccsCall),
        #[allow(missing_docs)]
        sequencerMessageCount(sequencerMessageCountCall),
        #[allow(missing_docs)]
        totalDelayedMessagesRead(totalDelayedMessagesReadCall),
    }
    #[automatically_derived]
    impl RollupCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 132u8, 18u8, 12u8],
            [4u8, 221u8, 123u8, 139u8],
            [6u8, 241u8, 48u8, 86u8],
            [22u8, 191u8, 85u8, 121u8],
            [42u8, 166u8, 218u8, 228u8],
            [47u8, 30u8, 197u8, 233u8],
            [79u8, 53u8, 154u8, 55u8],
            [92u8, 136u8, 232u8, 115u8],
            [127u8, 163u8, 164u8, 14u8],
            [167u8, 181u8, 29u8, 25u8],
            [173u8, 156u8, 12u8, 46u8],
            [183u8, 82u8, 167u8, 209u8],
            [213u8, 113u8, 157u8, 194u8],
            [217u8, 221u8, 103u8, 171u8],
            [225u8, 214u8, 106u8, 254u8],
            [232u8, 235u8, 29u8, 195u8],
            [236u8, 160u8, 103u8, 173u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RollupCalls {
        const NAME: &'static str = "RollupCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 17usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::batchCount(_) => {
                    <batchCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delayBlocks(_) => {
                    <delayBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delaySeconds(_) => {
                    <delaySecondsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delayedInboxAccs(_) => {
                    <delayedInboxAccsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delayedMessageCount(_) => {
                    <delayedMessageCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deliverMessage(_) => {
                    <deliverMessageCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositEth(_) => {
                    <depositEthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::futureBlocks(_) => {
                    <futureBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::futureSeconds(_) => {
                    <futureSecondsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::inboxAccs(_) => {
                    <inboxAccsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastProcessedSequencingBlock(_) => {
                    <lastProcessedSequencingBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastProcessedSettlementBlock(_) => {
                    <lastProcessedSettlementBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maxDataSize(_) => {
                    <maxDataSizeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::postBatch(_) => {
                    <postBatchCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::sequencerInboxAccs(_) => {
                    <sequencerInboxAccsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::sequencerMessageCount(_) => {
                    <sequencerMessageCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::totalDelayedMessagesRead(_) => {
                    <totalDelayedMessagesReadCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<RollupCalls>] = &[
                {
                    fn sequencerMessageCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <sequencerMessageCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::sequencerMessageCount)
                    }
                    sequencerMessageCount
                },
                {
                    fn lastProcessedSequencingBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <lastProcessedSequencingBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::lastProcessedSequencingBlock)
                    }
                    lastProcessedSequencingBlock
                },
                {
                    fn batchCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <batchCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::batchCount)
                    }
                    batchCount
                },
                {
                    fn sequencerInboxAccs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <sequencerInboxAccsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::sequencerInboxAccs)
                    }
                    sequencerInboxAccs
                },
                {
                    fn lastProcessedSettlementBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <lastProcessedSettlementBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::lastProcessedSettlementBlock)
                    }
                    lastProcessedSettlementBlock
                },
                {
                    fn futureSeconds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <futureSecondsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::futureSeconds)
                    }
                    futureSeconds
                },
                {
                    fn futureBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <futureBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::futureBlocks)
                    }
                    futureBlocks
                },
                {
                    fn postBatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <postBatchCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::postBatch)
                    }
                    postBatch
                },
                {
                    fn totalDelayedMessagesRead(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <totalDelayedMessagesReadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::totalDelayedMessagesRead)
                    }
                    totalDelayedMessagesRead
                },
                {
                    fn deliverMessage(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <deliverMessageCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::deliverMessage)
                    }
                    deliverMessage
                },
                {
                    fn delayBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <delayBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::delayBlocks)
                    }
                    delayBlocks
                },
                {
                    fn delaySeconds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <delaySecondsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::delaySeconds)
                    }
                    delaySeconds
                },
                {
                    fn delayedInboxAccs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <delayedInboxAccsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::delayedInboxAccs)
                    }
                    delayedInboxAccs
                },
                {
                    fn inboxAccs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <inboxAccsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::inboxAccs)
                    }
                    inboxAccs
                },
                {
                    fn depositEth(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <depositEthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::depositEth)
                    }
                    depositEth
                },
                {
                    fn maxDataSize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <maxDataSizeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::maxDataSize)
                    }
                    maxDataSize
                },
                {
                    fn delayedMessageCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <delayedMessageCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::delayedMessageCount)
                    }
                    delayedMessageCount
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
                Self::batchCount(inner) => {
                    <batchCountCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delayBlocks(inner) => {
                    <delayBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delaySeconds(inner) => {
                    <delaySecondsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delayedInboxAccs(inner) => {
                    <delayedInboxAccsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delayedMessageCount(inner) => {
                    <delayedMessageCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deliverMessage(inner) => {
                    <deliverMessageCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::depositEth(inner) => {
                    <depositEthCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::futureBlocks(inner) => {
                    <futureBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::futureSeconds(inner) => {
                    <futureSecondsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::inboxAccs(inner) => {
                    <inboxAccsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::lastProcessedSequencingBlock(inner) => {
                    <lastProcessedSequencingBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lastProcessedSettlementBlock(inner) => {
                    <lastProcessedSettlementBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::maxDataSize(inner) => {
                    <maxDataSizeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::postBatch(inner) => {
                    <postBatchCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::sequencerInboxAccs(inner) => {
                    <sequencerInboxAccsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::sequencerMessageCount(inner) => {
                    <sequencerMessageCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::totalDelayedMessagesRead(inner) => {
                    <totalDelayedMessagesReadCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::batchCount(inner) => {
                    <batchCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delayBlocks(inner) => {
                    <delayBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delaySeconds(inner) => {
                    <delaySecondsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delayedInboxAccs(inner) => {
                    <delayedInboxAccsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delayedMessageCount(inner) => {
                    <delayedMessageCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deliverMessage(inner) => {
                    <deliverMessageCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::depositEth(inner) => {
                    <depositEthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::futureBlocks(inner) => {
                    <futureBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::futureSeconds(inner) => {
                    <futureSecondsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::inboxAccs(inner) => {
                    <inboxAccsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastProcessedSequencingBlock(inner) => {
                    <lastProcessedSequencingBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastProcessedSettlementBlock(inner) => {
                    <lastProcessedSettlementBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::maxDataSize(inner) => {
                    <maxDataSizeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::postBatch(inner) => {
                    <postBatchCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::sequencerInboxAccs(inner) => {
                    <sequencerInboxAccsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::sequencerMessageCount(inner) => {
                    <sequencerMessageCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::totalDelayedMessagesRead(inner) => {
                    <totalDelayedMessagesReadCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Rollup`](self) custom errors.
    pub enum RollupErrors {
        #[allow(missing_docs)]
        DataTooLarge(DataTooLarge),
    }
    #[automatically_derived]
    impl RollupErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[70u8, 52u8, 105u8, 27u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RollupErrors {
        const NAME: &'static str = "RollupErrors";
        const MIN_DATA_LENGTH: usize = 64usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DataTooLarge(_) => {
                    <DataTooLarge as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<RollupErrors>] = &[
                {
                    fn DataTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupErrors> {
                        <DataTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupErrors::DataTooLarge)
                    }
                    DataTooLarge
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
                Self::DataTooLarge(inner) => {
                    <DataTooLarge as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::DataTooLarge(inner) => {
                    <DataTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Rollup`](self) events.
    pub enum RollupEvents {
        #[allow(missing_docs)]
        InboxMessageDelivered(InboxMessageDelivered),
        #[allow(missing_docs)]
        MessageDelivered(MessageDelivered),
        #[allow(missing_docs)]
        SequencerBatchData(SequencerBatchData),
        #[allow(missing_docs)]
        SequencerBatchDelivered(SequencerBatchDelivered),
    }
    #[automatically_derived]
    impl RollupEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                94u8,
                60u8,
                19u8,
                17u8,
                234u8,
                68u8,
                38u8,
                100u8,
                232u8,
                177u8,
                97u8,
                27u8,
                250u8,
                190u8,
                246u8,
                89u8,
                18u8,
                14u8,
                167u8,
                160u8,
                162u8,
                207u8,
                192u8,
                102u8,
                119u8,
                0u8,
                190u8,
                188u8,
                105u8,
                203u8,
                255u8,
                225u8,
            ],
            [
                115u8,
                148u8,
                244u8,
                161u8,
                154u8,
                19u8,
                199u8,
                185u8,
                43u8,
                91u8,
                183u8,
                16u8,
                51u8,
                36u8,
                83u8,
                5u8,
                148u8,
                110u8,
                247u8,
                132u8,
                82u8,
                247u8,
                180u8,
                152u8,
                106u8,
                193u8,
                57u8,
                11u8,
                93u8,
                244u8,
                235u8,
                215u8,
            ],
            [
                254u8,
                50u8,
                92u8,
                161u8,
                239u8,
                228u8,
                197u8,
                193u8,
                6u8,
                44u8,
                152u8,
                28u8,
                62u8,
                231u8,
                75u8,
                120u8,
                29u8,
                235u8,
                228u8,
                234u8,
                148u8,
                64u8,
                48u8,
                106u8,
                150u8,
                210u8,
                165u8,
                87u8,
                89u8,
                198u8,
                108u8,
                32u8,
            ],
            [
                255u8,
                100u8,
                144u8,
                95u8,
                115u8,
                166u8,
                127u8,
                181u8,
                148u8,
                224u8,
                249u8,
                64u8,
                168u8,
                7u8,
                90u8,
                134u8,
                13u8,
                180u8,
                137u8,
                173u8,
                153u8,
                30u8,
                3u8,
                47u8,
                72u8,
                200u8,
                17u8,
                35u8,
                235u8,
                82u8,
                214u8,
                11u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for RollupEvents {
        const NAME: &'static str = "RollupEvents";
        const COUNT: usize = 4usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <InboxMessageDelivered as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <InboxMessageDelivered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::InboxMessageDelivered)
                }
                Some(<MessageDelivered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MessageDelivered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MessageDelivered)
                }
                Some(
                    <SequencerBatchData as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SequencerBatchData as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SequencerBatchData)
                }
                Some(
                    <SequencerBatchDelivered as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SequencerBatchDelivered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SequencerBatchDelivered)
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
    impl alloy_sol_types::private::IntoLogData for RollupEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::InboxMessageDelivered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MessageDelivered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SequencerBatchData(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SequencerBatchDelivered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::InboxMessageDelivered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MessageDelivered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SequencerBatchData(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SequencerBatchDelivered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Rollup`](self) contract instance.

See the [wrapper's documentation](`RollupInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> RollupInstance<T, P, N> {
        RollupInstance::<T, P, N>::new(address, provider)
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
        chainId: alloy::sol_types::private::primitives::aliases::U256,
        chainConfig: alloy::sol_types::private::String,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<RollupInstance<T, P, N>>,
    > {
        RollupInstance::<T, P, N>::deploy(provider, chainId, chainConfig)
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
        chainId: alloy::sol_types::private::primitives::aliases::U256,
        chainConfig: alloy::sol_types::private::String,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        RollupInstance::<T, P, N>::deploy_builder(provider, chainId, chainConfig)
    }
    /**A [`Rollup`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Rollup`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct RollupInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for RollupInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("RollupInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > RollupInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Rollup`](self) contract instance.

See the [wrapper's documentation](`RollupInstance`) for more details.*/
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
            chainId: alloy::sol_types::private::primitives::aliases::U256,
            chainConfig: alloy::sol_types::private::String,
        ) -> alloy_contract::Result<RollupInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, chainId, chainConfig);
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
            chainId: alloy::sol_types::private::primitives::aliases::U256,
            chainConfig: alloy::sol_types::private::String,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            chainId,
                            chainConfig,
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
    impl<T, P: ::core::clone::Clone, N> RollupInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> RollupInstance<T, P, N> {
            RollupInstance {
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
    > RollupInstance<T, P, N> {
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
        ///Creates a new call builder for the [`batchCount`] function.
        pub fn batchCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, batchCountCall, N> {
            self.call_builder(&batchCountCall {})
        }
        ///Creates a new call builder for the [`delayBlocks`] function.
        pub fn delayBlocks(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delayBlocksCall, N> {
            self.call_builder(&delayBlocksCall {})
        }
        ///Creates a new call builder for the [`delaySeconds`] function.
        pub fn delaySeconds(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delaySecondsCall, N> {
            self.call_builder(&delaySecondsCall {})
        }
        ///Creates a new call builder for the [`delayedInboxAccs`] function.
        pub fn delayedInboxAccs(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, delayedInboxAccsCall, N> {
            self.call_builder(&delayedInboxAccsCall { _0 })
        }
        ///Creates a new call builder for the [`delayedMessageCount`] function.
        pub fn delayedMessageCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delayedMessageCountCall, N> {
            self.call_builder(&delayedMessageCountCall {})
        }
        ///Creates a new call builder for the [`deliverMessage`] function.
        pub fn deliverMessage(
            &self,
            kind: u8,
            sender: alloy::sol_types::private::Address,
            messageData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, deliverMessageCall, N> {
            self.call_builder(
                &deliverMessageCall {
                    kind,
                    sender,
                    messageData,
                },
            )
        }
        ///Creates a new call builder for the [`depositEth`] function.
        pub fn depositEth(
            &self,
            src: alloy::sol_types::private::Address,
            dest: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositEthCall, N> {
            self.call_builder(&depositEthCall { src, dest, value })
        }
        ///Creates a new call builder for the [`futureBlocks`] function.
        pub fn futureBlocks(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, futureBlocksCall, N> {
            self.call_builder(&futureBlocksCall {})
        }
        ///Creates a new call builder for the [`futureSeconds`] function.
        pub fn futureSeconds(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, futureSecondsCall, N> {
            self.call_builder(&futureSecondsCall {})
        }
        ///Creates a new call builder for the [`inboxAccs`] function.
        pub fn inboxAccs(
            &self,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, inboxAccsCall, N> {
            self.call_builder(&inboxAccsCall { index })
        }
        ///Creates a new call builder for the [`lastProcessedSequencingBlock`] function.
        pub fn lastProcessedSequencingBlock(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, lastProcessedSequencingBlockCall, N> {
            self.call_builder(
                &lastProcessedSequencingBlockCall {
                },
            )
        }
        ///Creates a new call builder for the [`lastProcessedSettlementBlock`] function.
        pub fn lastProcessedSettlementBlock(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, lastProcessedSettlementBlockCall, N> {
            self.call_builder(
                &lastProcessedSettlementBlockCall {
                },
            )
        }
        ///Creates a new call builder for the [`maxDataSize`] function.
        pub fn maxDataSize(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, maxDataSizeCall, N> {
            self.call_builder(&maxDataSizeCall {})
        }
        ///Creates a new call builder for the [`postBatch`] function.
        pub fn postBatch(
            &self,
            data: alloy::sol_types::private::Bytes,
            _lastProcessedSequencingBlock: alloy::sol_types::private::primitives::aliases::U256,
            _lastProcessedSettlementBlock: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, postBatchCall, N> {
            self.call_builder(
                &postBatchCall {
                    data,
                    _lastProcessedSequencingBlock,
                    _lastProcessedSettlementBlock,
                },
            )
        }
        ///Creates a new call builder for the [`sequencerInboxAccs`] function.
        pub fn sequencerInboxAccs(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, sequencerInboxAccsCall, N> {
            self.call_builder(&sequencerInboxAccsCall { _0 })
        }
        ///Creates a new call builder for the [`sequencerMessageCount`] function.
        pub fn sequencerMessageCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, sequencerMessageCountCall, N> {
            self.call_builder(&sequencerMessageCountCall {})
        }
        ///Creates a new call builder for the [`totalDelayedMessagesRead`] function.
        pub fn totalDelayedMessagesRead(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, totalDelayedMessagesReadCall, N> {
            self.call_builder(&totalDelayedMessagesReadCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > RollupInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`InboxMessageDelivered`] event.
        pub fn InboxMessageDelivered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, InboxMessageDelivered, N> {
            self.event_filter::<InboxMessageDelivered>()
        }
        ///Creates a new event filter for the [`MessageDelivered`] event.
        pub fn MessageDelivered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MessageDelivered, N> {
            self.event_filter::<MessageDelivered>()
        }
        ///Creates a new event filter for the [`SequencerBatchData`] event.
        pub fn SequencerBatchData_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SequencerBatchData, N> {
            self.event_filter::<SequencerBatchData>()
        }
        ///Creates a new event filter for the [`SequencerBatchDelivered`] event.
        pub fn SequencerBatchDelivered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SequencerBatchDelivered, N> {
            self.event_filter::<SequencerBatchDelivered>()
        }
    }
}
