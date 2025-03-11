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
    function getSourceChainsProcessedBlocks() external view returns (uint64 _seqBlockNumber, uint256 _seqBlockHash, uint64 _setBlockNumber, uint256 _setBlockHash);
    function inboxAccs(uint256 index) external view returns (bytes32);
    function maxDataSize() external view returns (uint64);
    function postBatch(bytes memory data, uint64 _seqBlockNumber, uint256 _seqBlockHash, uint64 _setBlockNumber, uint256 _setBlockHash) external;
    function seqBlockHash() external view returns (uint256);
    function seqBlockNumber() external view returns (uint64);
    function sequencerInboxAccs(uint256) external view returns (bytes32);
    function sequencerMessageCount() external view returns (uint256);
    function setBlockHash() external view returns (uint256);
    function setBlockNumber() external view returns (uint64);
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
    "name": "getSourceChainsProcessedBlocks",
    "inputs": [],
    "outputs": [
      {
        "name": "_seqBlockNumber",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "_seqBlockHash",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_setBlockNumber",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "_setBlockHash",
        "type": "uint256",
        "internalType": "uint256"
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
        "name": "_seqBlockNumber",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "_seqBlockHash",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_setBlockNumber",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "_setBlockHash",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "seqBlockHash",
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
    "name": "seqBlockNumber",
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
    "name": "setBlockHash",
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
    "name": "setBlockNumber",
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
    ///0x60806040525f805f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055505f6001555f60025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055505f600355348015610066575f80fd5b506040516126673803806126678339818101604052810190610088919061084d565b5f8151116100cb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016100c290610901565b60405180910390fd5b5f600190505f4890505f848383866040516020016100ec94939291906109c3565b604051602081830303815290604052905061010f600b5f8361016060201b60201c565b6101566040518060400160405280600681526020017e0b0080020300000000000000000000000000000000000000000000000000008152505f805f806102b660201b60201c565b505050505061108a565b5f60048054905090505f828051906020012090505f858543428648876040516020016101929796959493929190610af1565b6040516020818303038152906040528051906020012090505f805f1b90505f8411156101e45760046001856101c79190610b9e565b815481106101d8576101d7610bd1565b5b905f5260205f20015490505b600481836040516020016101f9929190610bfe565b60405160208183030381529060405280519060200120908060018154018082558091505060019003905f5260205f20015f909190919091505580847f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1308a8a88484260405161026d96959493929190610c74565b60405180910390a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b866040516102a59190610d25565b60405180910390a250505050505050565b835f806101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550826001819055505f8267ffffffffffffffff161115610326578160025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550806003819055505b5f60048054905090505f8061034188846104a660201b60201c565b915091505f60058054905090505f805f1b90505f82111561038857600560018361036b9190610b9e565b8154811061037c5761037b610bd1565b5b905f5260205f20015490505b5f805f1b90505f8611156103c25760046001876103a59190610b9e565b815481106103b6576103b5610bd1565b5b905f5260205f20015490505b5f8286836040516020016103d893929190610d45565b604051602081830303815290604052805190602001209050600581908060018154018082558091505060019003905f5260205f20015f9091909190915055866006819055508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7856006548a60016040516104589493929190610e56565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d6040516104909190610d25565b60405180910390a2505050505050505050505050565b5f6104af610681565b5f845160286104be9190610e99565b90506201cccc67ffffffffffffffff1681111561051757806201cccc6040517f4634691b00000000000000000000000000000000000000000000000000000000815260040161050e929190610f05565b60405180910390fd5b61051f610681565b6201518067ffffffffffffffff164211156105625762015180426105439190610f2c565b815f019067ffffffffffffffff16908167ffffffffffffffff16815250505b610e10426105709190610f67565b816020019067ffffffffffffffff16908167ffffffffffffffff1681525050611c2067ffffffffffffffff164311156105d157611c20436105b19190610f2c565b816040019067ffffffffffffffff16908167ffffffffffffffff16815250505b600c436105de9190610f67565b816060019067ffffffffffffffff16908167ffffffffffffffff16815250505f815f015182602001518360400151846060015189604051602001610626959493929190610fa2565b6040516020818303038152906040529050602881511461064957610648611000565b5b808760405160200161065c929190611067565b6040516020818303038152906040528051906020012082945094505050509250929050565b60405180608001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681525090565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b6106f0816106de565b81146106fa575f80fd5b50565b5f8151905061070b816106e7565b92915050565b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61075f82610719565b810181811067ffffffffffffffff8211171561077e5761077d610729565b5b80604052505050565b5f6107906106cd565b905061079c8282610756565b919050565b5f67ffffffffffffffff8211156107bb576107ba610729565b5b6107c482610719565b9050602081019050919050565b8281835e5f83830152505050565b5f6107f16107ec846107a1565b610787565b90508281526020810184848401111561080d5761080c610715565b5b6108188482856107d1565b509392505050565b5f82601f83011261083457610833610711565b5b81516108448482602086016107df565b91505092915050565b5f8060408385031215610863576108626106d6565b5b5f610870858286016106fd565b925050602083015167ffffffffffffffff811115610891576108906106da565b5b61089d85828601610820565b9150509250929050565b5f82825260208201905092915050565b7f454d5054595f434841494e5f434f4e46494700000000000000000000000000005f82015250565b5f6108eb6012836108a7565b91506108f6826108b7565b602082019050919050565b5f6020820190508181035f830152610918816108df565b9050919050565b5f819050919050565b610939610934826106de565b61091f565b82525050565b5f60ff82169050919050565b5f8160f81b9050919050565b5f6109618261094b565b9050919050565b6109796109748261093f565b610957565b82525050565b5f81519050919050565b5f81905092915050565b5f61099d8261097f565b6109a78185610989565b93506109b78185602086016107d1565b80840191505092915050565b5f6109ce8287610928565b6020820191506109de8286610968565b6001820191506109ee8285610928565b6020820191506109fe8284610993565b915081905095945050505050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610a3582610a0c565b9050919050565b5f8160601b9050919050565b5f610a5282610a3c565b9050919050565b5f610a6382610a48565b9050919050565b610a7b610a7682610a2b565b610a59565b82525050565b5f67ffffffffffffffff82169050919050565b5f8160c01b9050919050565b5f610aaa82610a94565b9050919050565b610ac2610abd82610a81565b610aa0565b82525050565b5f819050919050565b5f819050919050565b610aeb610ae682610ac8565b610ad1565b82525050565b5f610afc828a610968565b600182019150610b0c8289610a6a565b601482019150610b1c8288610ab1565b600882019150610b2c8287610ab1565b600882019150610b3c8286610928565b602082019150610b4c8285610928565b602082019150610b5c8284610ada565b60208201915081905098975050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610ba8826106de565b9150610bb3836106de565b9250828203905081811115610bcb57610bca610b71565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f610c098285610ada565b602082019150610c198284610ada565b6020820191508190509392505050565b610c3281610a2b565b82525050565b610c418161093f565b82525050565b610c5081610ac8565b82525050565b610c5f816106de565b82525050565b610c6e81610a81565b82525050565b5f60c082019050610c875f830189610c29565b610c946020830188610c38565b610ca16040830187610c29565b610cae6060830186610c47565b610cbb6080830185610c56565b610cc860a0830184610c65565b979650505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f610cf782610cd3565b610d018185610cdd565b9350610d118185602086016107d1565b610d1a81610719565b840191505092915050565b5f6020820190508181035f830152610d3d8184610ced565b905092915050565b5f610d508286610ada565b602082019150610d608285610ada565b602082019150610d708284610ada565b602082019150819050949350505050565b610d8a81610a81565b82525050565b608082015f820151610da45f850182610d81565b506020820151610db76020850182610d81565b506040820151610dca6040850182610d81565b506060820151610ddd6060850182610d81565b50505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60048110610e2157610e20610de3565b5b50565b5f819050610e3182610e10565b919050565b5f610e4082610e24565b9050919050565b610e5081610e36565b82525050565b5f60e082019050610e695f830187610c47565b610e766020830186610c56565b610e836040830185610d90565b610e9060c0830184610e47565b95945050505050565b5f610ea3826106de565b9150610eae836106de565b9250828201905080821115610ec657610ec5610b71565b5b92915050565b5f819050919050565b5f610eef610eea610ee584610a81565b610ecc565b6106de565b9050919050565b610eff81610ed5565b82525050565b5f604082019050610f185f830185610c56565b610f256020830184610ef6565b9392505050565b5f610f3682610a81565b9150610f4183610a81565b9250828203905067ffffffffffffffff811115610f6157610f60610b71565b5b92915050565b5f610f7182610a81565b9150610f7c83610a81565b9250828201905067ffffffffffffffff811115610f9c57610f9b610b71565b5b92915050565b5f610fad8288610ab1565b600882019150610fbd8287610ab1565b600882019150610fcd8286610ab1565b600882019150610fdd8285610ab1565b600882019150610fed8284610ab1565b6008820191508190509695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b5f81905092915050565b5f61104182610cd3565b61104b818561102d565b935061105b8185602086016107d1565b80840191505092915050565b5f6110728285611037565b915061107e8284611037565b91508190509392505050565b6115d0806110975f395ff3fe608060405234801561000f575f80fd5b5060043610610129575f3560e01c8063a7b51d19116100ab578063d9dd67ab1161006f578063d9dd67ab14610312578063e1d66afe14610342578063e8eb1dc31461035e578063eca067ad1461037c578063fbf6eaa51461039a57610129565b8063a7b51d1914610269578063ad9c0c2e14610285578063b752a7d1146102a3578063d5719dc2146102c1578063d5954c34146102f157610129565b806316bf5579116100f257806316bf5579146101c157806318db3940146101f15780632f1ec5e91461020f5780634f359a371461022d5780637fa3a40e1461024b57610129565b806284120c1461012d57806304f1c8541461014b578063056daaa614610169578063061d12c01461018757806306f13056146101a3575b5f80fd5b6101356103b8565b6040516101429190610a95565b60405180910390f35b6101536103c4565b6040516101609190610a95565b60405180910390f35b6101716103ca565b60405161017e9190610ad0565b60405180910390f35b6101a1600480360381019061019c9190610c8a565b6103e1565b005b6101ab6105cb565b6040516101b89190610a95565b60405180910390f35b6101db60048036038101906101d69190610d1d565b6105d7565b6040516101e89190610d60565b60405180910390f35b6101f96105f7565b6040516102069190610a95565b60405180910390f35b6102176105fd565b6040516102249190610ad0565b60405180910390f35b610235610603565b6040516102429190610ad0565b60405180910390f35b610253610608565b6040516102609190610a95565b60405180910390f35b610283600480360381019061027e9190610e09565b61060e565b005b61028d610764565b60405161029a9190610ad0565b60405180910390f35b6102ab61076a565b6040516102b89190610ad0565b60405180910390f35b6102db60048036038101906102d69190610d1d565b610771565b6040516102e89190610d60565b60405180910390f35b6102f9610791565b6040516103099493929190610e75565b60405180910390f35b61032c60048036038101906103279190610d1d565b6107d3565b6040516103399190610d60565b60405180910390f35b61035c60048036038101906103579190610eb8565b6107f8565b005b61036661082a565b6040516103739190610ad0565b60405180910390f35b610384610831565b6040516103919190610a95565b60405180910390f35b6103a261083d565b6040516103af9190610ad0565b60405180910390f35b5f600580549050905090565b60035481565b5f8054906101000a900467ffffffffffffffff1681565b835f806101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550826001819055505f8267ffffffffffffffff161115610451578160025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550806003819055505b5f60048054905090505f806104668884610856565b915091505f60058054905090505f805f1b90505f8211156104ad5760056001836104909190610f35565b815481106104a1576104a0610f68565b5b905f5260205f20015490505b5f805f1b90505f8611156104e75760046001876104ca9190610f35565b815481106104db576104da610f68565b5b905f5260205f20015490505b5f8286836040516020016104fd93929190610fb5565b604051602081830303815290604052805190602001209050600581908060018154018082558091505060019003905f5260205f20015f9091909190915055866006819055508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7856006548a600160405161057d94939291906110c6565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d6040516105b59190611169565b60405180910390a2505050505050505050505050565b5f600580549050905090565b600581815481106105e6575f80fd5b905f5260205f20015f915090505481565b60015481565b610e1081565b600c81565b60065481565b5f60048054905090505f828051906020012090505f858543428648876040516020016106409796959493929190611256565b6040516020818303038152906040528051906020012090505f805f1b90505f8411156106925760046001856106759190610f35565b8154811061068657610685610f68565b5b905f5260205f20015490505b600481836040516020016106a79291906112d6565b60405160208183030381529060405280519060200120908060018154018082558091505060019003905f5260205f20015f909190919091505580847f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1308a8a88484260405161071b9695949392919061131f565b60405180910390a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b866040516107539190611169565b60405180910390a250505050505050565b611c2081565b6201518081565b60048181548110610780575f80fd5b905f5260205f20015f915090505481565b5f805f805f8054906101000a900467ffffffffffffffff1660015460025f9054906101000a900467ffffffffffffffff16600354935093509350935090919293565b5f600582815481106107e8576107e7610f68565b5b905f5260205f2001549050919050565b610825600c84848460405160200161081192919061137e565b60405160208183030381529060405261060e565b505050565b6201cccc81565b5f600480549050905090565b60025f9054906101000a900467ffffffffffffffff1681565b5f61085f610a31565b5f8451602861086e91906113a9565b90506201cccc67ffffffffffffffff168111156108c757806201cccc6040517f4634691b0000000000000000000000000000000000000000000000000000000081526004016108be929190611415565b60405180910390fd5b6108cf610a31565b6201518067ffffffffffffffff164211156109125762015180426108f3919061143c565b815f019067ffffffffffffffff16908167ffffffffffffffff16815250505b610e10426109209190611477565b816020019067ffffffffffffffff16908167ffffffffffffffff1681525050611c2067ffffffffffffffff1643111561098157611c2043610961919061143c565b816040019067ffffffffffffffff16908167ffffffffffffffff16815250505b600c4361098e9190611477565b816060019067ffffffffffffffff16908167ffffffffffffffff16815250505f815f0151826020015183604001518460600151896040516020016109d69594939291906114b2565b604051602081830303815290604052905060288151146109f9576109f8611510565b5b8087604051602001610a0c929190611577565b6040516020818303038152906040528051906020012082945094505050509250929050565b60405180608001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681525090565b5f819050919050565b610a8f81610a7d565b82525050565b5f602082019050610aa85f830184610a86565b92915050565b5f67ffffffffffffffff82169050919050565b610aca81610aae565b82525050565b5f602082019050610ae35f830184610ac1565b92915050565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610b4882610b02565b810181811067ffffffffffffffff82111715610b6757610b66610b12565b5b80604052505050565b5f610b79610ae9565b9050610b858282610b3f565b919050565b5f67ffffffffffffffff821115610ba457610ba3610b12565b5b610bad82610b02565b9050602081019050919050565b828183375f83830152505050565b5f610bda610bd584610b8a565b610b70565b905082815260208101848484011115610bf657610bf5610afe565b5b610c01848285610bba565b509392505050565b5f82601f830112610c1d57610c1c610afa565b5b8135610c2d848260208601610bc8565b91505092915050565b610c3f81610aae565b8114610c49575f80fd5b50565b5f81359050610c5a81610c36565b92915050565b610c6981610a7d565b8114610c73575f80fd5b50565b5f81359050610c8481610c60565b92915050565b5f805f805f60a08688031215610ca357610ca2610af2565b5b5f86013567ffffffffffffffff811115610cc057610cbf610af6565b5b610ccc88828901610c09565b9550506020610cdd88828901610c4c565b9450506040610cee88828901610c76565b9350506060610cff88828901610c4c565b9250506080610d1088828901610c76565b9150509295509295909350565b5f60208284031215610d3257610d31610af2565b5b5f610d3f84828501610c76565b91505092915050565b5f819050919050565b610d5a81610d48565b82525050565b5f602082019050610d735f830184610d51565b92915050565b5f60ff82169050919050565b610d8e81610d79565b8114610d98575f80fd5b50565b5f81359050610da981610d85565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610dd882610daf565b9050919050565b610de881610dce565b8114610df2575f80fd5b50565b5f81359050610e0381610ddf565b92915050565b5f805f60608486031215610e2057610e1f610af2565b5b5f610e2d86828701610d9b565b9350506020610e3e86828701610df5565b925050604084013567ffffffffffffffff811115610e5f57610e5e610af6565b5b610e6b86828701610c09565b9150509250925092565b5f608082019050610e885f830187610ac1565b610e956020830186610a86565b610ea26040830185610ac1565b610eaf6060830184610a86565b95945050505050565b5f805f60608486031215610ecf57610ece610af2565b5b5f610edc86828701610df5565b9350506020610eed86828701610df5565b9250506040610efe86828701610c76565b9150509250925092565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610f3f82610a7d565b9150610f4a83610a7d565b9250828203905081811115610f6257610f61610f08565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b610faf610faa82610d48565b610f95565b82525050565b5f610fc08286610f9e565b602082019150610fd08285610f9e565b602082019150610fe08284610f9e565b602082019150819050949350505050565b610ffa81610aae565b82525050565b608082015f8201516110145f850182610ff1565b5060208201516110276020850182610ff1565b50604082015161103a6040850182610ff1565b50606082015161104d6060850182610ff1565b50505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6004811061109157611090611053565b5b50565b5f8190506110a182611080565b919050565b5f6110b082611094565b9050919050565b6110c0816110a6565b82525050565b5f60e0820190506110d95f830187610d51565b6110e66020830186610a86565b6110f36040830185611000565b61110060c08301846110b7565b95945050505050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f61113b82611109565b6111458185611113565b9350611155818560208601611123565b61115e81610b02565b840191505092915050565b5f6020820190508181035f8301526111818184611131565b905092915050565b5f8160f81b9050919050565b5f61119f82611189565b9050919050565b6111b76111b282610d79565b611195565b82525050565b5f8160601b9050919050565b5f6111d3826111bd565b9050919050565b5f6111e4826111c9565b9050919050565b6111fc6111f782610dce565b6111da565b82525050565b5f8160c01b9050919050565b5f61121882611202565b9050919050565b61123061122b82610aae565b61120e565b82525050565b5f819050919050565b61125061124b82610a7d565b611236565b82525050565b5f611261828a6111a6565b60018201915061127182896111eb565b601482019150611281828861121f565b600882019150611291828761121f565b6008820191506112a1828661123f565b6020820191506112b1828561123f565b6020820191506112c18284610f9e565b60208201915081905098975050505050505050565b5f6112e18285610f9e565b6020820191506112f18284610f9e565b6020820191508190509392505050565b61130a81610dce565b82525050565b61131981610d79565b82525050565b5f60c0820190506113325f830189611301565b61133f6020830188611310565b61134c6040830187611301565b6113596060830186610d51565b6113666080830185610a86565b61137360a0830184610ac1565b979650505050505050565b5f61138982856111eb565b601482019150611399828461123f565b6020820191508190509392505050565b5f6113b382610a7d565b91506113be83610a7d565b92508282019050808211156113d6576113d5610f08565b5b92915050565b5f819050919050565b5f6113ff6113fa6113f584610aae565b6113dc565b610a7d565b9050919050565b61140f816113e5565b82525050565b5f6040820190506114285f830185610a86565b6114356020830184611406565b9392505050565b5f61144682610aae565b915061145183610aae565b9250828203905067ffffffffffffffff81111561147157611470610f08565b5b92915050565b5f61148182610aae565b915061148c83610aae565b9250828201905067ffffffffffffffff8111156114ac576114ab610f08565b5b92915050565b5f6114bd828861121f565b6008820191506114cd828761121f565b6008820191506114dd828661121f565b6008820191506114ed828561121f565b6008820191506114fd828461121f565b6008820191508190509695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b5f81905092915050565b5f61155182611109565b61155b818561153d565b935061156b818560208601611123565b80840191505092915050565b5f6115828285611547565b915061158e8284611547565b9150819050939250505056fea26469706673582212205b98e34ee3d8c00edada0c08343e1fa7e4098372d4c1cab060df673425d39c5364736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R_\x80_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_`\x01U_`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_`\x03U4\x80\x15a\0fW_\x80\xFD[P`@Qa&g8\x03\x80a&g\x839\x81\x81\x01`@R\x81\x01\x90a\0\x88\x91\x90a\x08MV[_\x81Q\x11a\0\xCBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xC2\x90a\t\x01V[`@Q\x80\x91\x03\x90\xFD[_`\x01\x90P_H\x90P_\x84\x83\x83\x86`@Q` \x01a\0\xEC\x94\x93\x92\x91\x90a\t\xC3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x01\x0F`\x0B_\x83a\x01`` \x1B` \x1CV[a\x01V`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01~\x0B\0\x80\x02\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP_\x80_\x80a\x02\xB6` \x1B` \x1CV[PPPPPa\x10\x8AV[_`\x04\x80T\x90P\x90P_\x82\x80Q\x90` \x01 \x90P_\x85\x85CB\x86H\x87`@Q` \x01a\x01\x92\x97\x96\x95\x94\x93\x92\x91\x90a\n\xF1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_\x80_\x1B\x90P_\x84\x11\x15a\x01\xE4W`\x04`\x01\x85a\x01\xC7\x91\x90a\x0B\x9EV[\x81T\x81\x10a\x01\xD8Wa\x01\xD7a\x0B\xD1V[[\x90_R` _ \x01T\x90P[`\x04\x81\x83`@Q` \x01a\x01\xF9\x92\x91\x90a\x0B\xFEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x80\x84\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE10\x8A\x8A\x88HB`@Qa\x02m\x96\x95\x94\x93\x92\x91\x90a\x0CtV[`@Q\x80\x91\x03\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x02\xA5\x91\x90a\r%V[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[\x83_\x80a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82`\x01\x81\x90UP_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x03&W\x81`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x03\x81\x90UP[_`\x04\x80T\x90P\x90P_\x80a\x03A\x88\x84a\x04\xA6` \x1B` \x1CV[\x91P\x91P_`\x05\x80T\x90P\x90P_\x80_\x1B\x90P_\x82\x11\x15a\x03\x88W`\x05`\x01\x83a\x03k\x91\x90a\x0B\x9EV[\x81T\x81\x10a\x03|Wa\x03{a\x0B\xD1V[[\x90_R` _ \x01T\x90P[_\x80_\x1B\x90P_\x86\x11\x15a\x03\xC2W`\x04`\x01\x87a\x03\xA5\x91\x90a\x0B\x9EV[\x81T\x81\x10a\x03\xB6Wa\x03\xB5a\x0B\xD1V[[\x90_R` _ \x01T\x90P[_\x82\x86\x83`@Q` \x01a\x03\xD8\x93\x92\x91\x90a\rEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x05\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x86`\x06\x81\x90UP\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85`\x06T\x8A`\x01`@Qa\x04X\x94\x93\x92\x91\x90a\x0EVV[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D`@Qa\x04\x90\x91\x90a\r%V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPV[_a\x04\xAFa\x06\x81V[_\x84Q`(a\x04\xBE\x91\x90a\x0E\x99V[\x90Pb\x01\xCC\xCCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\x05\x17W\x80b\x01\xCC\xCC`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\x0E\x92\x91\x90a\x0F\x05V[`@Q\x80\x91\x03\x90\xFD[a\x05\x1Fa\x06\x81V[b\x01Q\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15a\x05bWb\x01Q\x80Ba\x05C\x91\x90a\x0F,V[\x81_\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[a\x0E\x10Ba\x05p\x91\x90a\x0FgV[\x81` \x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x1C g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x11\x15a\x05\xD1Wa\x1C Ca\x05\xB1\x91\x90a\x0F,V[\x81`@\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[`\x0CCa\x05\xDE\x91\x90a\x0FgV[\x81``\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x89`@Q` \x01a\x06&\x95\x94\x93\x92\x91\x90a\x0F\xA2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\x06IWa\x06Ha\x10\0V[[\x80\x87`@Q` \x01a\x06\\\x92\x91\x90a\x10gV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[`@Q\x80`\x80\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[a\x06\xF0\x81a\x06\xDEV[\x81\x14a\x06\xFAW_\x80\xFD[PV[_\x81Q\x90Pa\x07\x0B\x81a\x06\xE7V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x07_\x82a\x07\x19V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x07~Wa\x07}a\x07)V[[\x80`@RPPPV[_a\x07\x90a\x06\xCDV[\x90Pa\x07\x9C\x82\x82a\x07VV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\xBBWa\x07\xBAa\x07)V[[a\x07\xC4\x82a\x07\x19V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x07\xF1a\x07\xEC\x84a\x07\xA1V[a\x07\x87V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x08\rWa\x08\x0Ca\x07\x15V[[a\x08\x18\x84\x82\x85a\x07\xD1V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x084Wa\x083a\x07\x11V[[\x81Qa\x08D\x84\x82` \x86\x01a\x07\xDFV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x08cWa\x08ba\x06\xD6V[[_a\x08p\x85\x82\x86\x01a\x06\xFDV[\x92PP` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x91Wa\x08\x90a\x06\xDAV[[a\x08\x9D\x85\x82\x86\x01a\x08 V[\x91PP\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FEMPTY_CHAIN_CONFIG\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x08\xEB`\x12\x83a\x08\xA7V[\x91Pa\x08\xF6\x82a\x08\xB7V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\t\x18\x81a\x08\xDFV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\t9a\t4\x82a\x06\xDEV[a\t\x1FV[\x82RPPV[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\ta\x82a\tKV[\x90P\x91\x90PV[a\tya\tt\x82a\t?V[a\tWV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_a\t\x9D\x82a\t\x7FV[a\t\xA7\x81\x85a\t\x89V[\x93Pa\t\xB7\x81\x85` \x86\x01a\x07\xD1V[\x80\x84\x01\x91PP\x92\x91PPV[_a\t\xCE\x82\x87a\t(V[` \x82\x01\x91Pa\t\xDE\x82\x86a\thV[`\x01\x82\x01\x91Pa\t\xEE\x82\x85a\t(V[` \x82\x01\x91Pa\t\xFE\x82\x84a\t\x93V[\x91P\x81\x90P\x95\x94PPPPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\n5\x82a\n\x0CV[\x90P\x91\x90PV[_\x81``\x1B\x90P\x91\x90PV[_a\nR\x82a\n<V[\x90P\x91\x90PV[_a\nc\x82a\nHV[\x90P\x91\x90PV[a\n{a\nv\x82a\n+V[a\nYV[\x82RPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xC0\x1B\x90P\x91\x90PV[_a\n\xAA\x82a\n\x94V[\x90P\x91\x90PV[a\n\xC2a\n\xBD\x82a\n\x81V[a\n\xA0V[\x82RPPV[_\x81\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\n\xEBa\n\xE6\x82a\n\xC8V[a\n\xD1V[\x82RPPV[_a\n\xFC\x82\x8Aa\thV[`\x01\x82\x01\x91Pa\x0B\x0C\x82\x89a\njV[`\x14\x82\x01\x91Pa\x0B\x1C\x82\x88a\n\xB1V[`\x08\x82\x01\x91Pa\x0B,\x82\x87a\n\xB1V[`\x08\x82\x01\x91Pa\x0B<\x82\x86a\t(V[` \x82\x01\x91Pa\x0BL\x82\x85a\t(V[` \x82\x01\x91Pa\x0B\\\x82\x84a\n\xDAV[` \x82\x01\x91P\x81\x90P\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x0B\xA8\x82a\x06\xDEV[\x91Pa\x0B\xB3\x83a\x06\xDEV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x0B\xCBWa\x0B\xCAa\x0BqV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_a\x0C\t\x82\x85a\n\xDAV[` \x82\x01\x91Pa\x0C\x19\x82\x84a\n\xDAV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[a\x0C2\x81a\n+V[\x82RPPV[a\x0CA\x81a\t?V[\x82RPPV[a\x0CP\x81a\n\xC8V[\x82RPPV[a\x0C_\x81a\x06\xDEV[\x82RPPV[a\x0Cn\x81a\n\x81V[\x82RPPV[_`\xC0\x82\x01\x90Pa\x0C\x87_\x83\x01\x89a\x0C)V[a\x0C\x94` \x83\x01\x88a\x0C8V[a\x0C\xA1`@\x83\x01\x87a\x0C)V[a\x0C\xAE``\x83\x01\x86a\x0CGV[a\x0C\xBB`\x80\x83\x01\x85a\x0CVV[a\x0C\xC8`\xA0\x83\x01\x84a\x0CeV[\x97\x96PPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\x0C\xF7\x82a\x0C\xD3V[a\r\x01\x81\x85a\x0C\xDDV[\x93Pa\r\x11\x81\x85` \x86\x01a\x07\xD1V[a\r\x1A\x81a\x07\x19V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\r=\x81\x84a\x0C\xEDV[\x90P\x92\x91PPV[_a\rP\x82\x86a\n\xDAV[` \x82\x01\x91Pa\r`\x82\x85a\n\xDAV[` \x82\x01\x91Pa\rp\x82\x84a\n\xDAV[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[a\r\x8A\x81a\n\x81V[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\r\xA4_\x85\x01\x82a\r\x81V[P` \x82\x01Qa\r\xB7` \x85\x01\x82a\r\x81V[P`@\x82\x01Qa\r\xCA`@\x85\x01\x82a\r\x81V[P``\x82\x01Qa\r\xDD``\x85\x01\x82a\r\x81V[PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x0E!Wa\x0E a\r\xE3V[[PV[_\x81\x90Pa\x0E1\x82a\x0E\x10V[\x91\x90PV[_a\x0E@\x82a\x0E$V[\x90P\x91\x90PV[a\x0EP\x81a\x0E6V[\x82RPPV[_`\xE0\x82\x01\x90Pa\x0Ei_\x83\x01\x87a\x0CGV[a\x0Ev` \x83\x01\x86a\x0CVV[a\x0E\x83`@\x83\x01\x85a\r\x90V[a\x0E\x90`\xC0\x83\x01\x84a\x0EGV[\x95\x94PPPPPV[_a\x0E\xA3\x82a\x06\xDEV[\x91Pa\x0E\xAE\x83a\x06\xDEV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x0E\xC6Wa\x0E\xC5a\x0BqV[[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x0E\xEFa\x0E\xEAa\x0E\xE5\x84a\n\x81V[a\x0E\xCCV[a\x06\xDEV[\x90P\x91\x90PV[a\x0E\xFF\x81a\x0E\xD5V[\x82RPPV[_`@\x82\x01\x90Pa\x0F\x18_\x83\x01\x85a\x0CVV[a\x0F%` \x83\x01\x84a\x0E\xF6V[\x93\x92PPPV[_a\x0F6\x82a\n\x81V[\x91Pa\x0FA\x83a\n\x81V[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FaWa\x0F`a\x0BqV[[\x92\x91PPV[_a\x0Fq\x82a\n\x81V[\x91Pa\x0F|\x83a\n\x81V[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x9CWa\x0F\x9Ba\x0BqV[[\x92\x91PPV[_a\x0F\xAD\x82\x88a\n\xB1V[`\x08\x82\x01\x91Pa\x0F\xBD\x82\x87a\n\xB1V[`\x08\x82\x01\x91Pa\x0F\xCD\x82\x86a\n\xB1V[`\x08\x82\x01\x91Pa\x0F\xDD\x82\x85a\n\xB1V[`\x08\x82\x01\x91Pa\x0F\xED\x82\x84a\n\xB1V[`\x08\x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[_\x81\x90P\x92\x91PPV[_a\x10A\x82a\x0C\xD3V[a\x10K\x81\x85a\x10-V[\x93Pa\x10[\x81\x85` \x86\x01a\x07\xD1V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x10r\x82\x85a\x107V[\x91Pa\x10~\x82\x84a\x107V[\x91P\x81\x90P\x93\x92PPPV[a\x15\xD0\x80a\x10\x97_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01)W_5`\xE0\x1C\x80c\xA7\xB5\x1D\x19\x11a\0\xABW\x80c\xD9\xDDg\xAB\x11a\0oW\x80c\xD9\xDDg\xAB\x14a\x03\x12W\x80c\xE1\xD6j\xFE\x14a\x03BW\x80c\xE8\xEB\x1D\xC3\x14a\x03^W\x80c\xEC\xA0g\xAD\x14a\x03|W\x80c\xFB\xF6\xEA\xA5\x14a\x03\x9AWa\x01)V[\x80c\xA7\xB5\x1D\x19\x14a\x02iW\x80c\xAD\x9C\x0C.\x14a\x02\x85W\x80c\xB7R\xA7\xD1\x14a\x02\xA3W\x80c\xD5q\x9D\xC2\x14a\x02\xC1W\x80c\xD5\x95L4\x14a\x02\xF1Wa\x01)V[\x80c\x16\xBFUy\x11a\0\xF2W\x80c\x16\xBFUy\x14a\x01\xC1W\x80c\x18\xDB9@\x14a\x01\xF1W\x80c/\x1E\xC5\xE9\x14a\x02\x0FW\x80cO5\x9A7\x14a\x02-W\x80c\x7F\xA3\xA4\x0E\x14a\x02KWa\x01)V[\x80b\x84\x12\x0C\x14a\x01-W\x80c\x04\xF1\xC8T\x14a\x01KW\x80c\x05m\xAA\xA6\x14a\x01iW\x80c\x06\x1D\x12\xC0\x14a\x01\x87W\x80c\x06\xF10V\x14a\x01\xA3W[_\x80\xFD[a\x015a\x03\xB8V[`@Qa\x01B\x91\x90a\n\x95V[`@Q\x80\x91\x03\x90\xF3[a\x01Sa\x03\xC4V[`@Qa\x01`\x91\x90a\n\x95V[`@Q\x80\x91\x03\x90\xF3[a\x01qa\x03\xCAV[`@Qa\x01~\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA1`\x04\x806\x03\x81\x01\x90a\x01\x9C\x91\x90a\x0C\x8AV[a\x03\xE1V[\0[a\x01\xABa\x05\xCBV[`@Qa\x01\xB8\x91\x90a\n\x95V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDB`\x04\x806\x03\x81\x01\x90a\x01\xD6\x91\x90a\r\x1DV[a\x05\xD7V[`@Qa\x01\xE8\x91\x90a\r`V[`@Q\x80\x91\x03\x90\xF3[a\x01\xF9a\x05\xF7V[`@Qa\x02\x06\x91\x90a\n\x95V[`@Q\x80\x91\x03\x90\xF3[a\x02\x17a\x05\xFDV[`@Qa\x02$\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[a\x025a\x06\x03V[`@Qa\x02B\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[a\x02Sa\x06\x08V[`@Qa\x02`\x91\x90a\n\x95V[`@Q\x80\x91\x03\x90\xF3[a\x02\x83`\x04\x806\x03\x81\x01\x90a\x02~\x91\x90a\x0E\tV[a\x06\x0EV[\0[a\x02\x8Da\x07dV[`@Qa\x02\x9A\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[a\x02\xABa\x07jV[`@Qa\x02\xB8\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[a\x02\xDB`\x04\x806\x03\x81\x01\x90a\x02\xD6\x91\x90a\r\x1DV[a\x07qV[`@Qa\x02\xE8\x91\x90a\r`V[`@Q\x80\x91\x03\x90\xF3[a\x02\xF9a\x07\x91V[`@Qa\x03\t\x94\x93\x92\x91\x90a\x0EuV[`@Q\x80\x91\x03\x90\xF3[a\x03,`\x04\x806\x03\x81\x01\x90a\x03'\x91\x90a\r\x1DV[a\x07\xD3V[`@Qa\x039\x91\x90a\r`V[`@Q\x80\x91\x03\x90\xF3[a\x03\\`\x04\x806\x03\x81\x01\x90a\x03W\x91\x90a\x0E\xB8V[a\x07\xF8V[\0[a\x03fa\x08*V[`@Qa\x03s\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[a\x03\x84a\x081V[`@Qa\x03\x91\x91\x90a\n\x95V[`@Q\x80\x91\x03\x90\xF3[a\x03\xA2a\x08=V[`@Qa\x03\xAF\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[_`\x05\x80T\x90P\x90P\x90V[`\x03T\x81V[_\x80T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[\x83_\x80a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82`\x01\x81\x90UP_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x04QW\x81`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x03\x81\x90UP[_`\x04\x80T\x90P\x90P_\x80a\x04f\x88\x84a\x08VV[\x91P\x91P_`\x05\x80T\x90P\x90P_\x80_\x1B\x90P_\x82\x11\x15a\x04\xADW`\x05`\x01\x83a\x04\x90\x91\x90a\x0F5V[\x81T\x81\x10a\x04\xA1Wa\x04\xA0a\x0FhV[[\x90_R` _ \x01T\x90P[_\x80_\x1B\x90P_\x86\x11\x15a\x04\xE7W`\x04`\x01\x87a\x04\xCA\x91\x90a\x0F5V[\x81T\x81\x10a\x04\xDBWa\x04\xDAa\x0FhV[[\x90_R` _ \x01T\x90P[_\x82\x86\x83`@Q` \x01a\x04\xFD\x93\x92\x91\x90a\x0F\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x05\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x86`\x06\x81\x90UP\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85`\x06T\x8A`\x01`@Qa\x05}\x94\x93\x92\x91\x90a\x10\xC6V[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D`@Qa\x05\xB5\x91\x90a\x11iV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPV[_`\x05\x80T\x90P\x90P\x90V[`\x05\x81\x81T\x81\x10a\x05\xE6W_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[`\x01T\x81V[a\x0E\x10\x81V[`\x0C\x81V[`\x06T\x81V[_`\x04\x80T\x90P\x90P_\x82\x80Q\x90` \x01 \x90P_\x85\x85CB\x86H\x87`@Q` \x01a\x06@\x97\x96\x95\x94\x93\x92\x91\x90a\x12VV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_\x80_\x1B\x90P_\x84\x11\x15a\x06\x92W`\x04`\x01\x85a\x06u\x91\x90a\x0F5V[\x81T\x81\x10a\x06\x86Wa\x06\x85a\x0FhV[[\x90_R` _ \x01T\x90P[`\x04\x81\x83`@Q` \x01a\x06\xA7\x92\x91\x90a\x12\xD6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x80\x84\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE10\x8A\x8A\x88HB`@Qa\x07\x1B\x96\x95\x94\x93\x92\x91\x90a\x13\x1FV[`@Q\x80\x91\x03\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x07S\x91\x90a\x11iV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[a\x1C \x81V[b\x01Q\x80\x81V[`\x04\x81\x81T\x81\x10a\x07\x80W_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[_\x80_\x80_\x80T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01T`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03T\x93P\x93P\x93P\x93P\x90\x91\x92\x93V[_`\x05\x82\x81T\x81\x10a\x07\xE8Wa\x07\xE7a\x0FhV[[\x90_R` _ \x01T\x90P\x91\x90PV[a\x08%`\x0C\x84\x84\x84`@Q` \x01a\x08\x11\x92\x91\x90a\x13~V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x06\x0EV[PPPV[b\x01\xCC\xCC\x81V[_`\x04\x80T\x90P\x90P\x90V[`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[_a\x08_a\n1V[_\x84Q`(a\x08n\x91\x90a\x13\xA9V[\x90Pb\x01\xCC\xCCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\x08\xC7W\x80b\x01\xCC\xCC`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xBE\x92\x91\x90a\x14\x15V[`@Q\x80\x91\x03\x90\xFD[a\x08\xCFa\n1V[b\x01Q\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15a\t\x12Wb\x01Q\x80Ba\x08\xF3\x91\x90a\x14<V[\x81_\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[a\x0E\x10Ba\t \x91\x90a\x14wV[\x81` \x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x1C g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x11\x15a\t\x81Wa\x1C Ca\ta\x91\x90a\x14<V[\x81`@\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[`\x0CCa\t\x8E\x91\x90a\x14wV[\x81``\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x89`@Q` \x01a\t\xD6\x95\x94\x93\x92\x91\x90a\x14\xB2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\t\xF9Wa\t\xF8a\x15\x10V[[\x80\x87`@Q` \x01a\n\x0C\x92\x91\x90a\x15wV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[`@Q\x80`\x80\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_\x81\x90P\x91\x90PV[a\n\x8F\x81a\n}V[\x82RPPV[_` \x82\x01\x90Pa\n\xA8_\x83\x01\x84a\n\x86V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\n\xCA\x81a\n\xAEV[\x82RPPV[_` \x82\x01\x90Pa\n\xE3_\x83\x01\x84a\n\xC1V[\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0BH\x82a\x0B\x02V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0BgWa\x0Bfa\x0B\x12V[[\x80`@RPPPV[_a\x0Bya\n\xE9V[\x90Pa\x0B\x85\x82\x82a\x0B?V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\xA4Wa\x0B\xA3a\x0B\x12V[[a\x0B\xAD\x82a\x0B\x02V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0B\xDAa\x0B\xD5\x84a\x0B\x8AV[a\x0BpV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0B\xF6Wa\x0B\xF5a\n\xFEV[[a\x0C\x01\x84\x82\x85a\x0B\xBAV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0C\x1DWa\x0C\x1Ca\n\xFAV[[\x815a\x0C-\x84\x82` \x86\x01a\x0B\xC8V[\x91PP\x92\x91PPV[a\x0C?\x81a\n\xAEV[\x81\x14a\x0CIW_\x80\xFD[PV[_\x815\x90Pa\x0CZ\x81a\x0C6V[\x92\x91PPV[a\x0Ci\x81a\n}V[\x81\x14a\x0CsW_\x80\xFD[PV[_\x815\x90Pa\x0C\x84\x81a\x0C`V[\x92\x91PPV[_\x80_\x80_`\xA0\x86\x88\x03\x12\x15a\x0C\xA3Wa\x0C\xA2a\n\xF2V[[_\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xC0Wa\x0C\xBFa\n\xF6V[[a\x0C\xCC\x88\x82\x89\x01a\x0C\tV[\x95PP` a\x0C\xDD\x88\x82\x89\x01a\x0CLV[\x94PP`@a\x0C\xEE\x88\x82\x89\x01a\x0CvV[\x93PP``a\x0C\xFF\x88\x82\x89\x01a\x0CLV[\x92PP`\x80a\r\x10\x88\x82\x89\x01a\x0CvV[\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a\r2Wa\r1a\n\xF2V[[_a\r?\x84\x82\x85\x01a\x0CvV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\rZ\x81a\rHV[\x82RPPV[_` \x82\x01\x90Pa\rs_\x83\x01\x84a\rQV[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\r\x8E\x81a\ryV[\x81\x14a\r\x98W_\x80\xFD[PV[_\x815\x90Pa\r\xA9\x81a\r\x85V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\r\xD8\x82a\r\xAFV[\x90P\x91\x90PV[a\r\xE8\x81a\r\xCEV[\x81\x14a\r\xF2W_\x80\xFD[PV[_\x815\x90Pa\x0E\x03\x81a\r\xDFV[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x0E Wa\x0E\x1Fa\n\xF2V[[_a\x0E-\x86\x82\x87\x01a\r\x9BV[\x93PP` a\x0E>\x86\x82\x87\x01a\r\xF5V[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E_Wa\x0E^a\n\xF6V[[a\x0Ek\x86\x82\x87\x01a\x0C\tV[\x91PP\x92P\x92P\x92V[_`\x80\x82\x01\x90Pa\x0E\x88_\x83\x01\x87a\n\xC1V[a\x0E\x95` \x83\x01\x86a\n\x86V[a\x0E\xA2`@\x83\x01\x85a\n\xC1V[a\x0E\xAF``\x83\x01\x84a\n\x86V[\x95\x94PPPPPV[_\x80_``\x84\x86\x03\x12\x15a\x0E\xCFWa\x0E\xCEa\n\xF2V[[_a\x0E\xDC\x86\x82\x87\x01a\r\xF5V[\x93PP` a\x0E\xED\x86\x82\x87\x01a\r\xF5V[\x92PP`@a\x0E\xFE\x86\x82\x87\x01a\x0CvV[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x0F?\x82a\n}V[\x91Pa\x0FJ\x83a\n}V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x0FbWa\x0Faa\x0F\x08V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[a\x0F\xAFa\x0F\xAA\x82a\rHV[a\x0F\x95V[\x82RPPV[_a\x0F\xC0\x82\x86a\x0F\x9EV[` \x82\x01\x91Pa\x0F\xD0\x82\x85a\x0F\x9EV[` \x82\x01\x91Pa\x0F\xE0\x82\x84a\x0F\x9EV[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[a\x0F\xFA\x81a\n\xAEV[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\x10\x14_\x85\x01\x82a\x0F\xF1V[P` \x82\x01Qa\x10'` \x85\x01\x82a\x0F\xF1V[P`@\x82\x01Qa\x10:`@\x85\x01\x82a\x0F\xF1V[P``\x82\x01Qa\x10M``\x85\x01\x82a\x0F\xF1V[PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x10\x91Wa\x10\x90a\x10SV[[PV[_\x81\x90Pa\x10\xA1\x82a\x10\x80V[\x91\x90PV[_a\x10\xB0\x82a\x10\x94V[\x90P\x91\x90PV[a\x10\xC0\x81a\x10\xA6V[\x82RPPV[_`\xE0\x82\x01\x90Pa\x10\xD9_\x83\x01\x87a\rQV[a\x10\xE6` \x83\x01\x86a\n\x86V[a\x10\xF3`@\x83\x01\x85a\x10\0V[a\x11\0`\xC0\x83\x01\x84a\x10\xB7V[\x95\x94PPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x11;\x82a\x11\tV[a\x11E\x81\x85a\x11\x13V[\x93Pa\x11U\x81\x85` \x86\x01a\x11#V[a\x11^\x81a\x0B\x02V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x11\x81\x81\x84a\x111V[\x90P\x92\x91PPV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\x11\x9F\x82a\x11\x89V[\x90P\x91\x90PV[a\x11\xB7a\x11\xB2\x82a\ryV[a\x11\x95V[\x82RPPV[_\x81``\x1B\x90P\x91\x90PV[_a\x11\xD3\x82a\x11\xBDV[\x90P\x91\x90PV[_a\x11\xE4\x82a\x11\xC9V[\x90P\x91\x90PV[a\x11\xFCa\x11\xF7\x82a\r\xCEV[a\x11\xDAV[\x82RPPV[_\x81`\xC0\x1B\x90P\x91\x90PV[_a\x12\x18\x82a\x12\x02V[\x90P\x91\x90PV[a\x120a\x12+\x82a\n\xAEV[a\x12\x0EV[\x82RPPV[_\x81\x90P\x91\x90PV[a\x12Pa\x12K\x82a\n}V[a\x126V[\x82RPPV[_a\x12a\x82\x8Aa\x11\xA6V[`\x01\x82\x01\x91Pa\x12q\x82\x89a\x11\xEBV[`\x14\x82\x01\x91Pa\x12\x81\x82\x88a\x12\x1FV[`\x08\x82\x01\x91Pa\x12\x91\x82\x87a\x12\x1FV[`\x08\x82\x01\x91Pa\x12\xA1\x82\x86a\x12?V[` \x82\x01\x91Pa\x12\xB1\x82\x85a\x12?V[` \x82\x01\x91Pa\x12\xC1\x82\x84a\x0F\x9EV[` \x82\x01\x91P\x81\x90P\x98\x97PPPPPPPPV[_a\x12\xE1\x82\x85a\x0F\x9EV[` \x82\x01\x91Pa\x12\xF1\x82\x84a\x0F\x9EV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[a\x13\n\x81a\r\xCEV[\x82RPPV[a\x13\x19\x81a\ryV[\x82RPPV[_`\xC0\x82\x01\x90Pa\x132_\x83\x01\x89a\x13\x01V[a\x13?` \x83\x01\x88a\x13\x10V[a\x13L`@\x83\x01\x87a\x13\x01V[a\x13Y``\x83\x01\x86a\rQV[a\x13f`\x80\x83\x01\x85a\n\x86V[a\x13s`\xA0\x83\x01\x84a\n\xC1V[\x97\x96PPPPPPPV[_a\x13\x89\x82\x85a\x11\xEBV[`\x14\x82\x01\x91Pa\x13\x99\x82\x84a\x12?V[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[_a\x13\xB3\x82a\n}V[\x91Pa\x13\xBE\x83a\n}V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x13\xD6Wa\x13\xD5a\x0F\x08V[[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x13\xFFa\x13\xFAa\x13\xF5\x84a\n\xAEV[a\x13\xDCV[a\n}V[\x90P\x91\x90PV[a\x14\x0F\x81a\x13\xE5V[\x82RPPV[_`@\x82\x01\x90Pa\x14(_\x83\x01\x85a\n\x86V[a\x145` \x83\x01\x84a\x14\x06V[\x93\x92PPPV[_a\x14F\x82a\n\xAEV[\x91Pa\x14Q\x83a\n\xAEV[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14qWa\x14pa\x0F\x08V[[\x92\x91PPV[_a\x14\x81\x82a\n\xAEV[\x91Pa\x14\x8C\x83a\n\xAEV[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xACWa\x14\xABa\x0F\x08V[[\x92\x91PPV[_a\x14\xBD\x82\x88a\x12\x1FV[`\x08\x82\x01\x91Pa\x14\xCD\x82\x87a\x12\x1FV[`\x08\x82\x01\x91Pa\x14\xDD\x82\x86a\x12\x1FV[`\x08\x82\x01\x91Pa\x14\xED\x82\x85a\x12\x1FV[`\x08\x82\x01\x91Pa\x14\xFD\x82\x84a\x12\x1FV[`\x08\x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[_\x81\x90P\x92\x91PPV[_a\x15Q\x82a\x11\tV[a\x15[\x81\x85a\x15=V[\x93Pa\x15k\x81\x85` \x86\x01a\x11#V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x15\x82\x82\x85a\x15GV[\x91Pa\x15\x8E\x82\x84a\x15GV[\x91P\x81\x90P\x93\x92PPPV\xFE\xA2dipfsX\"\x12 [\x98\xE3N\xE3\xD8\xC0\x0E\xDA\xDA\x0C\x084>\x1F\xA7\xE4\t\x83r\xD4\xC1\xCA\xB0`\xDFg4%\xD3\x9CSdsolcC\0\x08\x19\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610129575f3560e01c8063a7b51d19116100ab578063d9dd67ab1161006f578063d9dd67ab14610312578063e1d66afe14610342578063e8eb1dc31461035e578063eca067ad1461037c578063fbf6eaa51461039a57610129565b8063a7b51d1914610269578063ad9c0c2e14610285578063b752a7d1146102a3578063d5719dc2146102c1578063d5954c34146102f157610129565b806316bf5579116100f257806316bf5579146101c157806318db3940146101f15780632f1ec5e91461020f5780634f359a371461022d5780637fa3a40e1461024b57610129565b806284120c1461012d57806304f1c8541461014b578063056daaa614610169578063061d12c01461018757806306f13056146101a3575b5f80fd5b6101356103b8565b6040516101429190610a95565b60405180910390f35b6101536103c4565b6040516101609190610a95565b60405180910390f35b6101716103ca565b60405161017e9190610ad0565b60405180910390f35b6101a1600480360381019061019c9190610c8a565b6103e1565b005b6101ab6105cb565b6040516101b89190610a95565b60405180910390f35b6101db60048036038101906101d69190610d1d565b6105d7565b6040516101e89190610d60565b60405180910390f35b6101f96105f7565b6040516102069190610a95565b60405180910390f35b6102176105fd565b6040516102249190610ad0565b60405180910390f35b610235610603565b6040516102429190610ad0565b60405180910390f35b610253610608565b6040516102609190610a95565b60405180910390f35b610283600480360381019061027e9190610e09565b61060e565b005b61028d610764565b60405161029a9190610ad0565b60405180910390f35b6102ab61076a565b6040516102b89190610ad0565b60405180910390f35b6102db60048036038101906102d69190610d1d565b610771565b6040516102e89190610d60565b60405180910390f35b6102f9610791565b6040516103099493929190610e75565b60405180910390f35b61032c60048036038101906103279190610d1d565b6107d3565b6040516103399190610d60565b60405180910390f35b61035c60048036038101906103579190610eb8565b6107f8565b005b61036661082a565b6040516103739190610ad0565b60405180910390f35b610384610831565b6040516103919190610a95565b60405180910390f35b6103a261083d565b6040516103af9190610ad0565b60405180910390f35b5f600580549050905090565b60035481565b5f8054906101000a900467ffffffffffffffff1681565b835f806101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550826001819055505f8267ffffffffffffffff161115610451578160025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550806003819055505b5f60048054905090505f806104668884610856565b915091505f60058054905090505f805f1b90505f8211156104ad5760056001836104909190610f35565b815481106104a1576104a0610f68565b5b905f5260205f20015490505b5f805f1b90505f8611156104e75760046001876104ca9190610f35565b815481106104db576104da610f68565b5b905f5260205f20015490505b5f8286836040516020016104fd93929190610fb5565b604051602081830303815290604052805190602001209050600581908060018154018082558091505060019003905f5260205f20015f9091909190915055866006819055508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7856006548a600160405161057d94939291906110c6565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d6040516105b59190611169565b60405180910390a2505050505050505050505050565b5f600580549050905090565b600581815481106105e6575f80fd5b905f5260205f20015f915090505481565b60015481565b610e1081565b600c81565b60065481565b5f60048054905090505f828051906020012090505f858543428648876040516020016106409796959493929190611256565b6040516020818303038152906040528051906020012090505f805f1b90505f8411156106925760046001856106759190610f35565b8154811061068657610685610f68565b5b905f5260205f20015490505b600481836040516020016106a79291906112d6565b60405160208183030381529060405280519060200120908060018154018082558091505060019003905f5260205f20015f909190919091505580847f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1308a8a88484260405161071b9695949392919061131f565b60405180910390a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b866040516107539190611169565b60405180910390a250505050505050565b611c2081565b6201518081565b60048181548110610780575f80fd5b905f5260205f20015f915090505481565b5f805f805f8054906101000a900467ffffffffffffffff1660015460025f9054906101000a900467ffffffffffffffff16600354935093509350935090919293565b5f600582815481106107e8576107e7610f68565b5b905f5260205f2001549050919050565b610825600c84848460405160200161081192919061137e565b60405160208183030381529060405261060e565b505050565b6201cccc81565b5f600480549050905090565b60025f9054906101000a900467ffffffffffffffff1681565b5f61085f610a31565b5f8451602861086e91906113a9565b90506201cccc67ffffffffffffffff168111156108c757806201cccc6040517f4634691b0000000000000000000000000000000000000000000000000000000081526004016108be929190611415565b60405180910390fd5b6108cf610a31565b6201518067ffffffffffffffff164211156109125762015180426108f3919061143c565b815f019067ffffffffffffffff16908167ffffffffffffffff16815250505b610e10426109209190611477565b816020019067ffffffffffffffff16908167ffffffffffffffff1681525050611c2067ffffffffffffffff1643111561098157611c2043610961919061143c565b816040019067ffffffffffffffff16908167ffffffffffffffff16815250505b600c4361098e9190611477565b816060019067ffffffffffffffff16908167ffffffffffffffff16815250505f815f0151826020015183604001518460600151896040516020016109d69594939291906114b2565b604051602081830303815290604052905060288151146109f9576109f8611510565b5b8087604051602001610a0c929190611577565b6040516020818303038152906040528051906020012082945094505050509250929050565b60405180608001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681525090565b5f819050919050565b610a8f81610a7d565b82525050565b5f602082019050610aa85f830184610a86565b92915050565b5f67ffffffffffffffff82169050919050565b610aca81610aae565b82525050565b5f602082019050610ae35f830184610ac1565b92915050565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610b4882610b02565b810181811067ffffffffffffffff82111715610b6757610b66610b12565b5b80604052505050565b5f610b79610ae9565b9050610b858282610b3f565b919050565b5f67ffffffffffffffff821115610ba457610ba3610b12565b5b610bad82610b02565b9050602081019050919050565b828183375f83830152505050565b5f610bda610bd584610b8a565b610b70565b905082815260208101848484011115610bf657610bf5610afe565b5b610c01848285610bba565b509392505050565b5f82601f830112610c1d57610c1c610afa565b5b8135610c2d848260208601610bc8565b91505092915050565b610c3f81610aae565b8114610c49575f80fd5b50565b5f81359050610c5a81610c36565b92915050565b610c6981610a7d565b8114610c73575f80fd5b50565b5f81359050610c8481610c60565b92915050565b5f805f805f60a08688031215610ca357610ca2610af2565b5b5f86013567ffffffffffffffff811115610cc057610cbf610af6565b5b610ccc88828901610c09565b9550506020610cdd88828901610c4c565b9450506040610cee88828901610c76565b9350506060610cff88828901610c4c565b9250506080610d1088828901610c76565b9150509295509295909350565b5f60208284031215610d3257610d31610af2565b5b5f610d3f84828501610c76565b91505092915050565b5f819050919050565b610d5a81610d48565b82525050565b5f602082019050610d735f830184610d51565b92915050565b5f60ff82169050919050565b610d8e81610d79565b8114610d98575f80fd5b50565b5f81359050610da981610d85565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610dd882610daf565b9050919050565b610de881610dce565b8114610df2575f80fd5b50565b5f81359050610e0381610ddf565b92915050565b5f805f60608486031215610e2057610e1f610af2565b5b5f610e2d86828701610d9b565b9350506020610e3e86828701610df5565b925050604084013567ffffffffffffffff811115610e5f57610e5e610af6565b5b610e6b86828701610c09565b9150509250925092565b5f608082019050610e885f830187610ac1565b610e956020830186610a86565b610ea26040830185610ac1565b610eaf6060830184610a86565b95945050505050565b5f805f60608486031215610ecf57610ece610af2565b5b5f610edc86828701610df5565b9350506020610eed86828701610df5565b9250506040610efe86828701610c76565b9150509250925092565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610f3f82610a7d565b9150610f4a83610a7d565b9250828203905081811115610f6257610f61610f08565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b610faf610faa82610d48565b610f95565b82525050565b5f610fc08286610f9e565b602082019150610fd08285610f9e565b602082019150610fe08284610f9e565b602082019150819050949350505050565b610ffa81610aae565b82525050565b608082015f8201516110145f850182610ff1565b5060208201516110276020850182610ff1565b50604082015161103a6040850182610ff1565b50606082015161104d6060850182610ff1565b50505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6004811061109157611090611053565b5b50565b5f8190506110a182611080565b919050565b5f6110b082611094565b9050919050565b6110c0816110a6565b82525050565b5f60e0820190506110d95f830187610d51565b6110e66020830186610a86565b6110f36040830185611000565b61110060c08301846110b7565b95945050505050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f61113b82611109565b6111458185611113565b9350611155818560208601611123565b61115e81610b02565b840191505092915050565b5f6020820190508181035f8301526111818184611131565b905092915050565b5f8160f81b9050919050565b5f61119f82611189565b9050919050565b6111b76111b282610d79565b611195565b82525050565b5f8160601b9050919050565b5f6111d3826111bd565b9050919050565b5f6111e4826111c9565b9050919050565b6111fc6111f782610dce565b6111da565b82525050565b5f8160c01b9050919050565b5f61121882611202565b9050919050565b61123061122b82610aae565b61120e565b82525050565b5f819050919050565b61125061124b82610a7d565b611236565b82525050565b5f611261828a6111a6565b60018201915061127182896111eb565b601482019150611281828861121f565b600882019150611291828761121f565b6008820191506112a1828661123f565b6020820191506112b1828561123f565b6020820191506112c18284610f9e565b60208201915081905098975050505050505050565b5f6112e18285610f9e565b6020820191506112f18284610f9e565b6020820191508190509392505050565b61130a81610dce565b82525050565b61131981610d79565b82525050565b5f60c0820190506113325f830189611301565b61133f6020830188611310565b61134c6040830187611301565b6113596060830186610d51565b6113666080830185610a86565b61137360a0830184610ac1565b979650505050505050565b5f61138982856111eb565b601482019150611399828461123f565b6020820191508190509392505050565b5f6113b382610a7d565b91506113be83610a7d565b92508282019050808211156113d6576113d5610f08565b5b92915050565b5f819050919050565b5f6113ff6113fa6113f584610aae565b6113dc565b610a7d565b9050919050565b61140f816113e5565b82525050565b5f6040820190506114285f830185610a86565b6114356020830184611406565b9392505050565b5f61144682610aae565b915061145183610aae565b9250828203905067ffffffffffffffff81111561147157611470610f08565b5b92915050565b5f61148182610aae565b915061148c83610aae565b9250828201905067ffffffffffffffff8111156114ac576114ab610f08565b5b92915050565b5f6114bd828861121f565b6008820191506114cd828761121f565b6008820191506114dd828661121f565b6008820191506114ed828561121f565b6008820191506114fd828461121f565b6008820191508190509695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b5f81905092915050565b5f61155182611109565b61155b818561153d565b935061156b818560208601611123565b80840191505092915050565b5f6115828285611547565b915061158e8284611547565b9150819050939250505056fea26469706673582212205b98e34ee3d8c00edada0c08343e1fa7e4098372d4c1cab060df673425d39c5364736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01)W_5`\xE0\x1C\x80c\xA7\xB5\x1D\x19\x11a\0\xABW\x80c\xD9\xDDg\xAB\x11a\0oW\x80c\xD9\xDDg\xAB\x14a\x03\x12W\x80c\xE1\xD6j\xFE\x14a\x03BW\x80c\xE8\xEB\x1D\xC3\x14a\x03^W\x80c\xEC\xA0g\xAD\x14a\x03|W\x80c\xFB\xF6\xEA\xA5\x14a\x03\x9AWa\x01)V[\x80c\xA7\xB5\x1D\x19\x14a\x02iW\x80c\xAD\x9C\x0C.\x14a\x02\x85W\x80c\xB7R\xA7\xD1\x14a\x02\xA3W\x80c\xD5q\x9D\xC2\x14a\x02\xC1W\x80c\xD5\x95L4\x14a\x02\xF1Wa\x01)V[\x80c\x16\xBFUy\x11a\0\xF2W\x80c\x16\xBFUy\x14a\x01\xC1W\x80c\x18\xDB9@\x14a\x01\xF1W\x80c/\x1E\xC5\xE9\x14a\x02\x0FW\x80cO5\x9A7\x14a\x02-W\x80c\x7F\xA3\xA4\x0E\x14a\x02KWa\x01)V[\x80b\x84\x12\x0C\x14a\x01-W\x80c\x04\xF1\xC8T\x14a\x01KW\x80c\x05m\xAA\xA6\x14a\x01iW\x80c\x06\x1D\x12\xC0\x14a\x01\x87W\x80c\x06\xF10V\x14a\x01\xA3W[_\x80\xFD[a\x015a\x03\xB8V[`@Qa\x01B\x91\x90a\n\x95V[`@Q\x80\x91\x03\x90\xF3[a\x01Sa\x03\xC4V[`@Qa\x01`\x91\x90a\n\x95V[`@Q\x80\x91\x03\x90\xF3[a\x01qa\x03\xCAV[`@Qa\x01~\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA1`\x04\x806\x03\x81\x01\x90a\x01\x9C\x91\x90a\x0C\x8AV[a\x03\xE1V[\0[a\x01\xABa\x05\xCBV[`@Qa\x01\xB8\x91\x90a\n\x95V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDB`\x04\x806\x03\x81\x01\x90a\x01\xD6\x91\x90a\r\x1DV[a\x05\xD7V[`@Qa\x01\xE8\x91\x90a\r`V[`@Q\x80\x91\x03\x90\xF3[a\x01\xF9a\x05\xF7V[`@Qa\x02\x06\x91\x90a\n\x95V[`@Q\x80\x91\x03\x90\xF3[a\x02\x17a\x05\xFDV[`@Qa\x02$\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[a\x025a\x06\x03V[`@Qa\x02B\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[a\x02Sa\x06\x08V[`@Qa\x02`\x91\x90a\n\x95V[`@Q\x80\x91\x03\x90\xF3[a\x02\x83`\x04\x806\x03\x81\x01\x90a\x02~\x91\x90a\x0E\tV[a\x06\x0EV[\0[a\x02\x8Da\x07dV[`@Qa\x02\x9A\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[a\x02\xABa\x07jV[`@Qa\x02\xB8\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[a\x02\xDB`\x04\x806\x03\x81\x01\x90a\x02\xD6\x91\x90a\r\x1DV[a\x07qV[`@Qa\x02\xE8\x91\x90a\r`V[`@Q\x80\x91\x03\x90\xF3[a\x02\xF9a\x07\x91V[`@Qa\x03\t\x94\x93\x92\x91\x90a\x0EuV[`@Q\x80\x91\x03\x90\xF3[a\x03,`\x04\x806\x03\x81\x01\x90a\x03'\x91\x90a\r\x1DV[a\x07\xD3V[`@Qa\x039\x91\x90a\r`V[`@Q\x80\x91\x03\x90\xF3[a\x03\\`\x04\x806\x03\x81\x01\x90a\x03W\x91\x90a\x0E\xB8V[a\x07\xF8V[\0[a\x03fa\x08*V[`@Qa\x03s\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[a\x03\x84a\x081V[`@Qa\x03\x91\x91\x90a\n\x95V[`@Q\x80\x91\x03\x90\xF3[a\x03\xA2a\x08=V[`@Qa\x03\xAF\x91\x90a\n\xD0V[`@Q\x80\x91\x03\x90\xF3[_`\x05\x80T\x90P\x90P\x90V[`\x03T\x81V[_\x80T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[\x83_\x80a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82`\x01\x81\x90UP_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x04QW\x81`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x03\x81\x90UP[_`\x04\x80T\x90P\x90P_\x80a\x04f\x88\x84a\x08VV[\x91P\x91P_`\x05\x80T\x90P\x90P_\x80_\x1B\x90P_\x82\x11\x15a\x04\xADW`\x05`\x01\x83a\x04\x90\x91\x90a\x0F5V[\x81T\x81\x10a\x04\xA1Wa\x04\xA0a\x0FhV[[\x90_R` _ \x01T\x90P[_\x80_\x1B\x90P_\x86\x11\x15a\x04\xE7W`\x04`\x01\x87a\x04\xCA\x91\x90a\x0F5V[\x81T\x81\x10a\x04\xDBWa\x04\xDAa\x0FhV[[\x90_R` _ \x01T\x90P[_\x82\x86\x83`@Q` \x01a\x04\xFD\x93\x92\x91\x90a\x0F\xB5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x05\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x86`\x06\x81\x90UP\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85`\x06T\x8A`\x01`@Qa\x05}\x94\x93\x92\x91\x90a\x10\xC6V[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D`@Qa\x05\xB5\x91\x90a\x11iV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPV[_`\x05\x80T\x90P\x90P\x90V[`\x05\x81\x81T\x81\x10a\x05\xE6W_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[`\x01T\x81V[a\x0E\x10\x81V[`\x0C\x81V[`\x06T\x81V[_`\x04\x80T\x90P\x90P_\x82\x80Q\x90` \x01 \x90P_\x85\x85CB\x86H\x87`@Q` \x01a\x06@\x97\x96\x95\x94\x93\x92\x91\x90a\x12VV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_\x80_\x1B\x90P_\x84\x11\x15a\x06\x92W`\x04`\x01\x85a\x06u\x91\x90a\x0F5V[\x81T\x81\x10a\x06\x86Wa\x06\x85a\x0FhV[[\x90_R` _ \x01T\x90P[`\x04\x81\x83`@Q` \x01a\x06\xA7\x92\x91\x90a\x12\xD6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x80\x84\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE10\x8A\x8A\x88HB`@Qa\x07\x1B\x96\x95\x94\x93\x92\x91\x90a\x13\x1FV[`@Q\x80\x91\x03\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x07S\x91\x90a\x11iV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[a\x1C \x81V[b\x01Q\x80\x81V[`\x04\x81\x81T\x81\x10a\x07\x80W_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[_\x80_\x80_\x80T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01T`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03T\x93P\x93P\x93P\x93P\x90\x91\x92\x93V[_`\x05\x82\x81T\x81\x10a\x07\xE8Wa\x07\xE7a\x0FhV[[\x90_R` _ \x01T\x90P\x91\x90PV[a\x08%`\x0C\x84\x84\x84`@Q` \x01a\x08\x11\x92\x91\x90a\x13~V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x06\x0EV[PPPV[b\x01\xCC\xCC\x81V[_`\x04\x80T\x90P\x90P\x90V[`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[_a\x08_a\n1V[_\x84Q`(a\x08n\x91\x90a\x13\xA9V[\x90Pb\x01\xCC\xCCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\x08\xC7W\x80b\x01\xCC\xCC`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xBE\x92\x91\x90a\x14\x15V[`@Q\x80\x91\x03\x90\xFD[a\x08\xCFa\n1V[b\x01Q\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15a\t\x12Wb\x01Q\x80Ba\x08\xF3\x91\x90a\x14<V[\x81_\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[a\x0E\x10Ba\t \x91\x90a\x14wV[\x81` \x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x1C g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x11\x15a\t\x81Wa\x1C Ca\ta\x91\x90a\x14<V[\x81`@\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[`\x0CCa\t\x8E\x91\x90a\x14wV[\x81``\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x89`@Q` \x01a\t\xD6\x95\x94\x93\x92\x91\x90a\x14\xB2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\t\xF9Wa\t\xF8a\x15\x10V[[\x80\x87`@Q` \x01a\n\x0C\x92\x91\x90a\x15wV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[`@Q\x80`\x80\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_\x81\x90P\x91\x90PV[a\n\x8F\x81a\n}V[\x82RPPV[_` \x82\x01\x90Pa\n\xA8_\x83\x01\x84a\n\x86V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\n\xCA\x81a\n\xAEV[\x82RPPV[_` \x82\x01\x90Pa\n\xE3_\x83\x01\x84a\n\xC1V[\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0BH\x82a\x0B\x02V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0BgWa\x0Bfa\x0B\x12V[[\x80`@RPPPV[_a\x0Bya\n\xE9V[\x90Pa\x0B\x85\x82\x82a\x0B?V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\xA4Wa\x0B\xA3a\x0B\x12V[[a\x0B\xAD\x82a\x0B\x02V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0B\xDAa\x0B\xD5\x84a\x0B\x8AV[a\x0BpV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0B\xF6Wa\x0B\xF5a\n\xFEV[[a\x0C\x01\x84\x82\x85a\x0B\xBAV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0C\x1DWa\x0C\x1Ca\n\xFAV[[\x815a\x0C-\x84\x82` \x86\x01a\x0B\xC8V[\x91PP\x92\x91PPV[a\x0C?\x81a\n\xAEV[\x81\x14a\x0CIW_\x80\xFD[PV[_\x815\x90Pa\x0CZ\x81a\x0C6V[\x92\x91PPV[a\x0Ci\x81a\n}V[\x81\x14a\x0CsW_\x80\xFD[PV[_\x815\x90Pa\x0C\x84\x81a\x0C`V[\x92\x91PPV[_\x80_\x80_`\xA0\x86\x88\x03\x12\x15a\x0C\xA3Wa\x0C\xA2a\n\xF2V[[_\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xC0Wa\x0C\xBFa\n\xF6V[[a\x0C\xCC\x88\x82\x89\x01a\x0C\tV[\x95PP` a\x0C\xDD\x88\x82\x89\x01a\x0CLV[\x94PP`@a\x0C\xEE\x88\x82\x89\x01a\x0CvV[\x93PP``a\x0C\xFF\x88\x82\x89\x01a\x0CLV[\x92PP`\x80a\r\x10\x88\x82\x89\x01a\x0CvV[\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a\r2Wa\r1a\n\xF2V[[_a\r?\x84\x82\x85\x01a\x0CvV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\rZ\x81a\rHV[\x82RPPV[_` \x82\x01\x90Pa\rs_\x83\x01\x84a\rQV[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\r\x8E\x81a\ryV[\x81\x14a\r\x98W_\x80\xFD[PV[_\x815\x90Pa\r\xA9\x81a\r\x85V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\r\xD8\x82a\r\xAFV[\x90P\x91\x90PV[a\r\xE8\x81a\r\xCEV[\x81\x14a\r\xF2W_\x80\xFD[PV[_\x815\x90Pa\x0E\x03\x81a\r\xDFV[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x0E Wa\x0E\x1Fa\n\xF2V[[_a\x0E-\x86\x82\x87\x01a\r\x9BV[\x93PP` a\x0E>\x86\x82\x87\x01a\r\xF5V[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E_Wa\x0E^a\n\xF6V[[a\x0Ek\x86\x82\x87\x01a\x0C\tV[\x91PP\x92P\x92P\x92V[_`\x80\x82\x01\x90Pa\x0E\x88_\x83\x01\x87a\n\xC1V[a\x0E\x95` \x83\x01\x86a\n\x86V[a\x0E\xA2`@\x83\x01\x85a\n\xC1V[a\x0E\xAF``\x83\x01\x84a\n\x86V[\x95\x94PPPPPV[_\x80_``\x84\x86\x03\x12\x15a\x0E\xCFWa\x0E\xCEa\n\xF2V[[_a\x0E\xDC\x86\x82\x87\x01a\r\xF5V[\x93PP` a\x0E\xED\x86\x82\x87\x01a\r\xF5V[\x92PP`@a\x0E\xFE\x86\x82\x87\x01a\x0CvV[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x0F?\x82a\n}V[\x91Pa\x0FJ\x83a\n}V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x0FbWa\x0Faa\x0F\x08V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[a\x0F\xAFa\x0F\xAA\x82a\rHV[a\x0F\x95V[\x82RPPV[_a\x0F\xC0\x82\x86a\x0F\x9EV[` \x82\x01\x91Pa\x0F\xD0\x82\x85a\x0F\x9EV[` \x82\x01\x91Pa\x0F\xE0\x82\x84a\x0F\x9EV[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[a\x0F\xFA\x81a\n\xAEV[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\x10\x14_\x85\x01\x82a\x0F\xF1V[P` \x82\x01Qa\x10'` \x85\x01\x82a\x0F\xF1V[P`@\x82\x01Qa\x10:`@\x85\x01\x82a\x0F\xF1V[P``\x82\x01Qa\x10M``\x85\x01\x82a\x0F\xF1V[PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x10\x91Wa\x10\x90a\x10SV[[PV[_\x81\x90Pa\x10\xA1\x82a\x10\x80V[\x91\x90PV[_a\x10\xB0\x82a\x10\x94V[\x90P\x91\x90PV[a\x10\xC0\x81a\x10\xA6V[\x82RPPV[_`\xE0\x82\x01\x90Pa\x10\xD9_\x83\x01\x87a\rQV[a\x10\xE6` \x83\x01\x86a\n\x86V[a\x10\xF3`@\x83\x01\x85a\x10\0V[a\x11\0`\xC0\x83\x01\x84a\x10\xB7V[\x95\x94PPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x11;\x82a\x11\tV[a\x11E\x81\x85a\x11\x13V[\x93Pa\x11U\x81\x85` \x86\x01a\x11#V[a\x11^\x81a\x0B\x02V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x11\x81\x81\x84a\x111V[\x90P\x92\x91PPV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\x11\x9F\x82a\x11\x89V[\x90P\x91\x90PV[a\x11\xB7a\x11\xB2\x82a\ryV[a\x11\x95V[\x82RPPV[_\x81``\x1B\x90P\x91\x90PV[_a\x11\xD3\x82a\x11\xBDV[\x90P\x91\x90PV[_a\x11\xE4\x82a\x11\xC9V[\x90P\x91\x90PV[a\x11\xFCa\x11\xF7\x82a\r\xCEV[a\x11\xDAV[\x82RPPV[_\x81`\xC0\x1B\x90P\x91\x90PV[_a\x12\x18\x82a\x12\x02V[\x90P\x91\x90PV[a\x120a\x12+\x82a\n\xAEV[a\x12\x0EV[\x82RPPV[_\x81\x90P\x91\x90PV[a\x12Pa\x12K\x82a\n}V[a\x126V[\x82RPPV[_a\x12a\x82\x8Aa\x11\xA6V[`\x01\x82\x01\x91Pa\x12q\x82\x89a\x11\xEBV[`\x14\x82\x01\x91Pa\x12\x81\x82\x88a\x12\x1FV[`\x08\x82\x01\x91Pa\x12\x91\x82\x87a\x12\x1FV[`\x08\x82\x01\x91Pa\x12\xA1\x82\x86a\x12?V[` \x82\x01\x91Pa\x12\xB1\x82\x85a\x12?V[` \x82\x01\x91Pa\x12\xC1\x82\x84a\x0F\x9EV[` \x82\x01\x91P\x81\x90P\x98\x97PPPPPPPPV[_a\x12\xE1\x82\x85a\x0F\x9EV[` \x82\x01\x91Pa\x12\xF1\x82\x84a\x0F\x9EV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[a\x13\n\x81a\r\xCEV[\x82RPPV[a\x13\x19\x81a\ryV[\x82RPPV[_`\xC0\x82\x01\x90Pa\x132_\x83\x01\x89a\x13\x01V[a\x13?` \x83\x01\x88a\x13\x10V[a\x13L`@\x83\x01\x87a\x13\x01V[a\x13Y``\x83\x01\x86a\rQV[a\x13f`\x80\x83\x01\x85a\n\x86V[a\x13s`\xA0\x83\x01\x84a\n\xC1V[\x97\x96PPPPPPPV[_a\x13\x89\x82\x85a\x11\xEBV[`\x14\x82\x01\x91Pa\x13\x99\x82\x84a\x12?V[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[_a\x13\xB3\x82a\n}V[\x91Pa\x13\xBE\x83a\n}V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x13\xD6Wa\x13\xD5a\x0F\x08V[[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x13\xFFa\x13\xFAa\x13\xF5\x84a\n\xAEV[a\x13\xDCV[a\n}V[\x90P\x91\x90PV[a\x14\x0F\x81a\x13\xE5V[\x82RPPV[_`@\x82\x01\x90Pa\x14(_\x83\x01\x85a\n\x86V[a\x145` \x83\x01\x84a\x14\x06V[\x93\x92PPPV[_a\x14F\x82a\n\xAEV[\x91Pa\x14Q\x83a\n\xAEV[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14qWa\x14pa\x0F\x08V[[\x92\x91PPV[_a\x14\x81\x82a\n\xAEV[\x91Pa\x14\x8C\x83a\n\xAEV[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xACWa\x14\xABa\x0F\x08V[[\x92\x91PPV[_a\x14\xBD\x82\x88a\x12\x1FV[`\x08\x82\x01\x91Pa\x14\xCD\x82\x87a\x12\x1FV[`\x08\x82\x01\x91Pa\x14\xDD\x82\x86a\x12\x1FV[`\x08\x82\x01\x91Pa\x14\xED\x82\x85a\x12\x1FV[`\x08\x82\x01\x91Pa\x14\xFD\x82\x84a\x12\x1FV[`\x08\x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[_\x81\x90P\x92\x91PPV[_a\x15Q\x82a\x11\tV[a\x15[\x81\x85a\x15=V[\x93Pa\x15k\x81\x85` \x86\x01a\x11#V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x15\x82\x82\x85a\x15GV[\x91Pa\x15\x8E\x82\x84a\x15GV[\x91P\x81\x90P\x93\x92PPPV\xFE\xA2dipfsX\"\x12 [\x98\xE3N\xE3\xD8\xC0\x0E\xDA\xDA\x0C\x084>\x1F\xA7\xE4\t\x83r\xD4\xC1\xCA\xB0`\xDFg4%\xD3\x9CSdsolcC\0\x08\x19\x003",
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
    /**Function with signature `getSourceChainsProcessedBlocks()` and selector `0xd5954c34`.
```solidity
function getSourceChainsProcessedBlocks() external view returns (uint64 _seqBlockNumber, uint256 _seqBlockHash, uint64 _setBlockNumber, uint256 _setBlockHash);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSourceChainsProcessedBlocksCall {}
    ///Container type for the return parameters of the [`getSourceChainsProcessedBlocks()`](getSourceChainsProcessedBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSourceChainsProcessedBlocksReturn {
        #[allow(missing_docs)]
        pub _seqBlockNumber: u64,
        #[allow(missing_docs)]
        pub _seqBlockHash: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _setBlockNumber: u64,
        #[allow(missing_docs)]
        pub _setBlockHash: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getSourceChainsProcessedBlocksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSourceChainsProcessedBlocksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSourceChainsProcessedBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::primitives::aliases::U256,
                u64,
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
            impl ::core::convert::From<getSourceChainsProcessedBlocksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSourceChainsProcessedBlocksReturn) -> Self {
                    (
                        value._seqBlockNumber,
                        value._seqBlockHash,
                        value._setBlockNumber,
                        value._setBlockHash,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSourceChainsProcessedBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _seqBlockNumber: tuple.0,
                        _seqBlockHash: tuple.1,
                        _setBlockNumber: tuple.2,
                        _setBlockHash: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSourceChainsProcessedBlocksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSourceChainsProcessedBlocksReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSourceChainsProcessedBlocks()";
            const SELECTOR: [u8; 4] = [213u8, 149u8, 76u8, 52u8];
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
    /**Function with signature `postBatch(bytes,uint64,uint256,uint64,uint256)` and selector `0x061d12c0`.
```solidity
function postBatch(bytes memory data, uint64 _seqBlockNumber, uint256 _seqBlockHash, uint64 _setBlockNumber, uint256 _setBlockHash) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postBatchCall {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _seqBlockNumber: u64,
        #[allow(missing_docs)]
        pub _seqBlockHash: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _setBlockNumber: u64,
        #[allow(missing_docs)]
        pub _setBlockHash: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`postBatch(bytes,uint64,uint256,uint64,uint256)`](postBatchCall) function.
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
                u64,
                alloy::sol_types::private::primitives::aliases::U256,
                u64,
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
                        value._seqBlockNumber,
                        value._seqBlockHash,
                        value._setBlockNumber,
                        value._setBlockHash,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for postBatchCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        data: tuple.0,
                        _seqBlockNumber: tuple.1,
                        _seqBlockHash: tuple.2,
                        _setBlockNumber: tuple.3,
                        _setBlockHash: tuple.4,
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
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
            const SIGNATURE: &'static str = "postBatch(bytes,uint64,uint256,uint64,uint256)";
            const SELECTOR: [u8; 4] = [6u8, 29u8, 18u8, 192u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._seqBlockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._seqBlockHash),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._setBlockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._setBlockHash),
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
    /**Function with signature `seqBlockHash()` and selector `0x18db3940`.
```solidity
function seqBlockHash() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct seqBlockHashCall {}
    ///Container type for the return parameters of the [`seqBlockHash()`](seqBlockHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct seqBlockHashReturn {
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
            impl ::core::convert::From<seqBlockHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: seqBlockHashCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for seqBlockHashCall {
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
            impl ::core::convert::From<seqBlockHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: seqBlockHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for seqBlockHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for seqBlockHashCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = seqBlockHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "seqBlockHash()";
            const SELECTOR: [u8; 4] = [24u8, 219u8, 57u8, 64u8];
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
    /**Function with signature `seqBlockNumber()` and selector `0x056daaa6`.
```solidity
function seqBlockNumber() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct seqBlockNumberCall {}
    ///Container type for the return parameters of the [`seqBlockNumber()`](seqBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct seqBlockNumberReturn {
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
            impl ::core::convert::From<seqBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: seqBlockNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for seqBlockNumberCall {
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
            impl ::core::convert::From<seqBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: seqBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for seqBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for seqBlockNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = seqBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "seqBlockNumber()";
            const SELECTOR: [u8; 4] = [5u8, 109u8, 170u8, 166u8];
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
    /**Function with signature `setBlockHash()` and selector `0x04f1c854`.
```solidity
function setBlockHash() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBlockHashCall {}
    ///Container type for the return parameters of the [`setBlockHash()`](setBlockHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBlockHashReturn {
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
            impl ::core::convert::From<setBlockHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: setBlockHashCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBlockHashCall {
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
            impl ::core::convert::From<setBlockHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setBlockHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBlockHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setBlockHashCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setBlockHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setBlockHash()";
            const SELECTOR: [u8; 4] = [4u8, 241u8, 200u8, 84u8];
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
    /**Function with signature `setBlockNumber()` and selector `0xfbf6eaa5`.
```solidity
function setBlockNumber() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBlockNumberCall {}
    ///Container type for the return parameters of the [`setBlockNumber()`](setBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBlockNumberReturn {
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
            impl ::core::convert::From<setBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: setBlockNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBlockNumberCall {
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
            impl ::core::convert::From<setBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setBlockNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setBlockNumber()";
            const SELECTOR: [u8; 4] = [251u8, 246u8, 234u8, 165u8];
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
        getSourceChainsProcessedBlocks(getSourceChainsProcessedBlocksCall),
        #[allow(missing_docs)]
        inboxAccs(inboxAccsCall),
        #[allow(missing_docs)]
        maxDataSize(maxDataSizeCall),
        #[allow(missing_docs)]
        postBatch(postBatchCall),
        #[allow(missing_docs)]
        seqBlockHash(seqBlockHashCall),
        #[allow(missing_docs)]
        seqBlockNumber(seqBlockNumberCall),
        #[allow(missing_docs)]
        sequencerInboxAccs(sequencerInboxAccsCall),
        #[allow(missing_docs)]
        sequencerMessageCount(sequencerMessageCountCall),
        #[allow(missing_docs)]
        setBlockHash(setBlockHashCall),
        #[allow(missing_docs)]
        setBlockNumber(setBlockNumberCall),
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
            [4u8, 241u8, 200u8, 84u8],
            [5u8, 109u8, 170u8, 166u8],
            [6u8, 29u8, 18u8, 192u8],
            [6u8, 241u8, 48u8, 86u8],
            [22u8, 191u8, 85u8, 121u8],
            [24u8, 219u8, 57u8, 64u8],
            [47u8, 30u8, 197u8, 233u8],
            [79u8, 53u8, 154u8, 55u8],
            [127u8, 163u8, 164u8, 14u8],
            [167u8, 181u8, 29u8, 25u8],
            [173u8, 156u8, 12u8, 46u8],
            [183u8, 82u8, 167u8, 209u8],
            [213u8, 113u8, 157u8, 194u8],
            [213u8, 149u8, 76u8, 52u8],
            [217u8, 221u8, 103u8, 171u8],
            [225u8, 214u8, 106u8, 254u8],
            [232u8, 235u8, 29u8, 195u8],
            [236u8, 160u8, 103u8, 173u8],
            [251u8, 246u8, 234u8, 165u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RollupCalls {
        const NAME: &'static str = "RollupCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 20usize;
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
                Self::getSourceChainsProcessedBlocks(_) => {
                    <getSourceChainsProcessedBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::inboxAccs(_) => {
                    <inboxAccsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maxDataSize(_) => {
                    <maxDataSizeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::postBatch(_) => {
                    <postBatchCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::seqBlockHash(_) => {
                    <seqBlockHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::seqBlockNumber(_) => {
                    <seqBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::sequencerInboxAccs(_) => {
                    <sequencerInboxAccsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::sequencerMessageCount(_) => {
                    <sequencerMessageCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setBlockHash(_) => {
                    <setBlockHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setBlockNumber(_) => {
                    <setBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn setBlockHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <setBlockHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::setBlockHash)
                    }
                    setBlockHash
                },
                {
                    fn seqBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <seqBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::seqBlockNumber)
                    }
                    seqBlockNumber
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
                    fn seqBlockHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <seqBlockHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::seqBlockHash)
                    }
                    seqBlockHash
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
                    fn getSourceChainsProcessedBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <getSourceChainsProcessedBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::getSourceChainsProcessedBlocks)
                    }
                    getSourceChainsProcessedBlocks
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
                {
                    fn setBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <setBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::setBlockNumber)
                    }
                    setBlockNumber
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
                Self::getSourceChainsProcessedBlocks(inner) => {
                    <getSourceChainsProcessedBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::inboxAccs(inner) => {
                    <inboxAccsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::maxDataSize(inner) => {
                    <maxDataSizeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::postBatch(inner) => {
                    <postBatchCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::seqBlockHash(inner) => {
                    <seqBlockHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::seqBlockNumber(inner) => {
                    <seqBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::setBlockHash(inner) => {
                    <setBlockHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setBlockNumber(inner) => {
                    <setBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getSourceChainsProcessedBlocks(inner) => {
                    <getSourceChainsProcessedBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::seqBlockHash(inner) => {
                    <seqBlockHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::seqBlockNumber(inner) => {
                    <seqBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setBlockHash(inner) => {
                    <setBlockHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setBlockNumber(inner) => {
                    <setBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        ///Creates a new call builder for the [`getSourceChainsProcessedBlocks`] function.
        pub fn getSourceChainsProcessedBlocks(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getSourceChainsProcessedBlocksCall,
            N,
        > {
            self.call_builder(
                &getSourceChainsProcessedBlocksCall {
                },
            )
        }
        ///Creates a new call builder for the [`inboxAccs`] function.
        pub fn inboxAccs(
            &self,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, inboxAccsCall, N> {
            self.call_builder(&inboxAccsCall { index })
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
            _seqBlockNumber: u64,
            _seqBlockHash: alloy::sol_types::private::primitives::aliases::U256,
            _setBlockNumber: u64,
            _setBlockHash: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, postBatchCall, N> {
            self.call_builder(
                &postBatchCall {
                    data,
                    _seqBlockNumber,
                    _seqBlockHash,
                    _setBlockNumber,
                    _setBlockHash,
                },
            )
        }
        ///Creates a new call builder for the [`seqBlockHash`] function.
        pub fn seqBlockHash(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, seqBlockHashCall, N> {
            self.call_builder(&seqBlockHashCall {})
        }
        ///Creates a new call builder for the [`seqBlockNumber`] function.
        pub fn seqBlockNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, seqBlockNumberCall, N> {
            self.call_builder(&seqBlockNumberCall {})
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
        ///Creates a new call builder for the [`setBlockHash`] function.
        pub fn setBlockHash(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, setBlockHashCall, N> {
            self.call_builder(&setBlockHashCall {})
        }
        ///Creates a new call builder for the [`setBlockNumber`] function.
        pub fn setBlockNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, setBlockNumberCall, N> {
            self.call_builder(&setBlockNumberCall {})
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
