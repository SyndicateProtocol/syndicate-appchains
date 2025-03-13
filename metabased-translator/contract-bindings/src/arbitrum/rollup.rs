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
    ///0x60806040525f5f5f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055505f6001555f60025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055505f600355348015610066575f5ffd5b506040516126963803806126968339818101604052810190610088919061083e565b5f8151116100cb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016100c2906108f2565b60405180910390fd5b610103600b5f8460015f866040516020016100e994939291906109b4565b60405160208183030381529060405261015160201b60201c565b61014a6040518060400160405280600681526020017e0b0080020300000000000000000000000000000000000000000000000000008152505f5f5f5f6102a760201b60201c565b50506110b4565b5f60048054905090505f828051906020012090505f85854342865f876040516020016101839796959493929190610ae2565b6040516020818303038152906040528051906020012090505f5f5f1b90505f8411156101d55760046001856101b89190610b8f565b815481106101c9576101c8610bc2565b5b905f5260205f20015490505b600481836040516020016101ea929190610bef565b60405160208183030381529060405280519060200120908060018154018082558091505060019003905f5260205f20015f909190919091505580847f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1308a8a885f4260405161025e96959493929190610c98565b60405180910390a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b866040516102969190610d49565b60405180910390a250505050505050565b835f5f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550826001819055505f8267ffffffffffffffff161115610317578160025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550806003819055505b5f60048054905090505f5f610332888461049760201b60201c565b915091505f60058054905090505f5f5f1b90505f82111561037957600560018361035c9190610b8f565b8154811061036d5761036c610bc2565b5b905f5260205f20015490505b5f5f5f1b90505f8611156103b35760046001876103969190610b8f565b815481106103a7576103a6610bc2565b5b905f5260205f20015490505b5f8286836040516020016103c993929190610d69565b604051602081830303815290604052805190602001209050600581908060018154018082558091505060019003905f5260205f20015f9091909190915055866006819055508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7856006548a60016040516104499493929190610e89565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d6040516104819190610d49565b60405180910390a2505050505050505050505050565b5f6104a0610672565b5f845160286104af9190610ecc565b90506201cccc67ffffffffffffffff1681111561050857806201cccc6040517f4634691b0000000000000000000000000000000000000000000000000000000081526004016104ff929190610f2f565b60405180910390fd5b610510610672565b6201518067ffffffffffffffff164211156105535762015180426105349190610f56565b815f019067ffffffffffffffff16908167ffffffffffffffff16815250505b610e10426105619190610f91565b816020019067ffffffffffffffff16908167ffffffffffffffff1681525050611c2067ffffffffffffffff164311156105c257611c20436105a29190610f56565b816040019067ffffffffffffffff16908167ffffffffffffffff16815250505b600c436105cf9190610f91565b816060019067ffffffffffffffff16908167ffffffffffffffff16815250505f815f015182602001518360400151846060015189604051602001610617959493929190610fcc565b6040516020818303038152906040529050602881511461063a5761063961102a565b5b808760405160200161064d929190611091565b6040516020818303038152906040528051906020012082945094505050509250929050565b60405180608001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681525090565b5f604051905090565b5f5ffd5b5f5ffd5b5f819050919050565b6106e1816106cf565b81146106eb575f5ffd5b50565b5f815190506106fc816106d8565b92915050565b5f5ffd5b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6107508261070a565b810181811067ffffffffffffffff8211171561076f5761076e61071a565b5b80604052505050565b5f6107816106be565b905061078d8282610747565b919050565b5f67ffffffffffffffff8211156107ac576107ab61071a565b5b6107b58261070a565b9050602081019050919050565b8281835e5f83830152505050565b5f6107e26107dd84610792565b610778565b9050828152602081018484840111156107fe576107fd610706565b5b6108098482856107c2565b509392505050565b5f82601f83011261082557610824610702565b5b81516108358482602086016107d0565b91505092915050565b5f5f60408385031215610854576108536106c7565b5b5f610861858286016106ee565b925050602083015167ffffffffffffffff811115610882576108816106cb565b5b61088e85828601610811565b9150509250929050565b5f82825260208201905092915050565b7f454d5054595f434841494e5f434f4e46494700000000000000000000000000005f82015250565b5f6108dc601283610898565b91506108e7826108a8565b602082019050919050565b5f6020820190508181035f830152610909816108d0565b9050919050565b5f819050919050565b61092a610925826106cf565b610910565b82525050565b5f60ff82169050919050565b5f8160f81b9050919050565b5f6109528261093c565b9050919050565b61096a61096582610930565b610948565b82525050565b5f81519050919050565b5f81905092915050565b5f61098e82610970565b610998818561097a565b93506109a88185602086016107c2565b80840191505092915050565b5f6109bf8287610919565b6020820191506109cf8286610959565b6001820191506109df8285610919565b6020820191506109ef8284610984565b915081905095945050505050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610a26826109fd565b9050919050565b5f8160601b9050919050565b5f610a4382610a2d565b9050919050565b5f610a5482610a39565b9050919050565b610a6c610a6782610a1c565b610a4a565b82525050565b5f67ffffffffffffffff82169050919050565b5f8160c01b9050919050565b5f610a9b82610a85565b9050919050565b610ab3610aae82610a72565b610a91565b82525050565b5f819050919050565b5f819050919050565b610adc610ad782610ab9565b610ac2565b82525050565b5f610aed828a610959565b600182019150610afd8289610a5b565b601482019150610b0d8288610aa2565b600882019150610b1d8287610aa2565b600882019150610b2d8286610919565b602082019150610b3d8285610919565b602082019150610b4d8284610acb565b60208201915081905098975050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610b99826106cf565b9150610ba4836106cf565b9250828203905081811115610bbc57610bbb610b62565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f610bfa8285610acb565b602082019150610c0a8284610acb565b6020820191508190509392505050565b610c2381610a1c565b82525050565b610c3281610930565b82525050565b610c4181610ab9565b82525050565b5f819050919050565b5f819050919050565b5f610c73610c6e610c6984610c47565b610c50565b6106cf565b9050919050565b610c8381610c59565b82525050565b610c9281610a72565b82525050565b5f60c082019050610cab5f830189610c1a565b610cb86020830188610c29565b610cc56040830187610c1a565b610cd26060830186610c38565b610cdf6080830185610c7a565b610cec60a0830184610c89565b979650505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f610d1b82610cf7565b610d258185610d01565b9350610d358185602086016107c2565b610d3e8161070a565b840191505092915050565b5f6020820190508181035f830152610d618184610d11565b905092915050565b5f610d748286610acb565b602082019150610d848285610acb565b602082019150610d948284610acb565b602082019150819050949350505050565b610dae816106cf565b82525050565b610dbd81610a72565b82525050565b608082015f820151610dd75f850182610db4565b506020820151610dea6020850182610db4565b506040820151610dfd6040850182610db4565b506060820151610e106060850182610db4565b50505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60048110610e5457610e53610e16565b5b50565b5f819050610e6482610e43565b919050565b5f610e7382610e57565b9050919050565b610e8381610e69565b82525050565b5f60e082019050610e9c5f830187610c38565b610ea96020830186610da5565b610eb66040830185610dc3565b610ec360c0830184610e7a565b95945050505050565b5f610ed6826106cf565b9150610ee1836106cf565b9250828201905080821115610ef957610ef8610b62565b5b92915050565b5f610f19610f14610f0f84610a72565b610c50565b6106cf565b9050919050565b610f2981610eff565b82525050565b5f604082019050610f425f830185610da5565b610f4f6020830184610f20565b9392505050565b5f610f6082610a72565b9150610f6b83610a72565b9250828203905067ffffffffffffffff811115610f8b57610f8a610b62565b5b92915050565b5f610f9b82610a72565b9150610fa683610a72565b9250828201905067ffffffffffffffff811115610fc657610fc5610b62565b5b92915050565b5f610fd78288610aa2565b600882019150610fe78287610aa2565b600882019150610ff78286610aa2565b6008820191506110078285610aa2565b6008820191506110178284610aa2565b6008820191508190509695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b5f81905092915050565b5f61106b82610cf7565b6110758185611057565b93506110858185602086016107c2565b80840191505092915050565b5f61109c8285611061565b91506110a88284611061565b91508190509392505050565b6115d5806110c15f395ff3fe608060405234801561000f575f5ffd5b5060043610610129575f3560e01c8063a7b51d19116100ab578063d9dd67ab1161006f578063d9dd67ab14610312578063e1d66afe14610342578063e8eb1dc31461035e578063eca067ad1461037c578063fbf6eaa51461039a57610129565b8063a7b51d1914610269578063ad9c0c2e14610285578063b752a7d1146102a3578063d5719dc2146102c1578063d5954c34146102f157610129565b806316bf5579116100f257806316bf5579146101c157806318db3940146101f15780632f1ec5e91461020f5780634f359a371461022d5780637fa3a40e1461024b57610129565b806284120c1461012d57806304f1c8541461014b578063056daaa614610169578063061d12c01461018757806306f13056146101a3575b5f5ffd5b6101356103b8565b6040516101429190610a97565b60405180910390f35b6101536103c4565b6040516101609190610a97565b60405180910390f35b6101716103ca565b60405161017e9190610ad2565b60405180910390f35b6101a1600480360381019061019c9190610c8c565b6103e2565b005b6101ab6105cc565b6040516101b89190610a97565b60405180910390f35b6101db60048036038101906101d69190610d1f565b6105d8565b6040516101e89190610d62565b60405180910390f35b6101f96105f8565b6040516102069190610a97565b60405180910390f35b6102176105fe565b6040516102249190610ad2565b60405180910390f35b610235610604565b6040516102429190610ad2565b60405180910390f35b610253610609565b6040516102609190610a97565b60405180910390f35b610283600480360381019061027e9190610e0b565b61060f565b005b61028d610765565b60405161029a9190610ad2565b60405180910390f35b6102ab61076b565b6040516102b89190610ad2565b60405180910390f35b6102db60048036038101906102d69190610d1f565b610772565b6040516102e89190610d62565b60405180910390f35b6102f9610792565b6040516103099493929190610e77565b60405180910390f35b61032c60048036038101906103279190610d1f565b6107d5565b6040516103399190610d62565b60405180910390f35b61035c60048036038101906103579190610eba565b6107fa565b005b61036661082c565b6040516103739190610ad2565b60405180910390f35b610384610833565b6040516103919190610a97565b60405180910390f35b6103a261083f565b6040516103af9190610ad2565b60405180910390f35b5f600580549050905090565b60035481565b5f5f9054906101000a900467ffffffffffffffff1681565b835f5f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550826001819055505f8267ffffffffffffffff161115610452578160025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550806003819055505b5f60048054905090505f5f6104678884610858565b915091505f60058054905090505f5f5f1b90505f8211156104ae5760056001836104919190610f37565b815481106104a2576104a1610f6a565b5b905f5260205f20015490505b5f5f5f1b90505f8611156104e85760046001876104cb9190610f37565b815481106104dc576104db610f6a565b5b905f5260205f20015490505b5f8286836040516020016104fe93929190610fb7565b604051602081830303815290604052805190602001209050600581908060018154018082558091505060019003905f5260205f20015f9091909190915055866006819055508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7856006548a600160405161057e94939291906110c8565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d6040516105b6919061116b565b60405180910390a2505050505050505050505050565b5f600580549050905090565b600581815481106105e7575f80fd5b905f5260205f20015f915090505481565b60015481565b610e1081565b600c81565b60065481565b5f60048054905090505f828051906020012090505f85854342865f876040516020016106419796959493929190611258565b6040516020818303038152906040528051906020012090505f5f5f1b90505f8411156106935760046001856106769190610f37565b8154811061068757610686610f6a565b5b905f5260205f20015490505b600481836040516020016106a89291906112d8565b60405160208183030381529060405280519060200120908060018154018082558091505060019003905f5260205f20015f909190919091505580847f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1308a8a885f4260405161071c96959493929190611363565b60405180910390a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b86604051610754919061116b565b60405180910390a250505050505050565b611c2081565b6201518081565b60048181548110610781575f80fd5b905f5260205f20015f915090505481565b5f5f5f5f5f5f9054906101000a900467ffffffffffffffff1660015460025f9054906101000a900467ffffffffffffffff16600354935093509350935090919293565b5f600582815481106107ea576107e9610f6a565b5b905f5260205f2001549050919050565b610827600c8484846040516020016108139291906113c2565b60405160208183030381529060405261060f565b505050565b6201cccc81565b5f600480549050905090565b60025f9054906101000a900467ffffffffffffffff1681565b5f610861610a33565b5f8451602861087091906113ed565b90506201cccc67ffffffffffffffff168111156108c957806201cccc6040517f4634691b0000000000000000000000000000000000000000000000000000000081526004016108c0929190611450565b60405180910390fd5b6108d1610a33565b6201518067ffffffffffffffff164211156109145762015180426108f59190611477565b815f019067ffffffffffffffff16908167ffffffffffffffff16815250505b610e104261092291906114b2565b816020019067ffffffffffffffff16908167ffffffffffffffff1681525050611c2067ffffffffffffffff1643111561098357611c20436109639190611477565b816040019067ffffffffffffffff16908167ffffffffffffffff16815250505b600c4361099091906114b2565b816060019067ffffffffffffffff16908167ffffffffffffffff16815250505f815f0151826020015183604001518460600151896040516020016109d89594939291906114ed565b604051602081830303815290604052905060288151146109fb576109fa61154b565b5b8087604051602001610a0e9291906115b2565b6040516020818303038152906040528051906020012082945094505050509250929050565b60405180608001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681525090565b5f819050919050565b610a9181610a7f565b82525050565b5f602082019050610aaa5f830184610a88565b92915050565b5f67ffffffffffffffff82169050919050565b610acc81610ab0565b82525050565b5f602082019050610ae55f830184610ac3565b92915050565b5f604051905090565b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610b4a82610b04565b810181811067ffffffffffffffff82111715610b6957610b68610b14565b5b80604052505050565b5f610b7b610aeb565b9050610b878282610b41565b919050565b5f67ffffffffffffffff821115610ba657610ba5610b14565b5b610baf82610b04565b9050602081019050919050565b828183375f83830152505050565b5f610bdc610bd784610b8c565b610b72565b905082815260208101848484011115610bf857610bf7610b00565b5b610c03848285610bbc565b509392505050565b5f82601f830112610c1f57610c1e610afc565b5b8135610c2f848260208601610bca565b91505092915050565b610c4181610ab0565b8114610c4b575f5ffd5b50565b5f81359050610c5c81610c38565b92915050565b610c6b81610a7f565b8114610c75575f5ffd5b50565b5f81359050610c8681610c62565b92915050565b5f5f5f5f5f60a08688031215610ca557610ca4610af4565b5b5f86013567ffffffffffffffff811115610cc257610cc1610af8565b5b610cce88828901610c0b565b9550506020610cdf88828901610c4e565b9450506040610cf088828901610c78565b9350506060610d0188828901610c4e565b9250506080610d1288828901610c78565b9150509295509295909350565b5f60208284031215610d3457610d33610af4565b5b5f610d4184828501610c78565b91505092915050565b5f819050919050565b610d5c81610d4a565b82525050565b5f602082019050610d755f830184610d53565b92915050565b5f60ff82169050919050565b610d9081610d7b565b8114610d9a575f5ffd5b50565b5f81359050610dab81610d87565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610dda82610db1565b9050919050565b610dea81610dd0565b8114610df4575f5ffd5b50565b5f81359050610e0581610de1565b92915050565b5f5f5f60608486031215610e2257610e21610af4565b5b5f610e2f86828701610d9d565b9350506020610e4086828701610df7565b925050604084013567ffffffffffffffff811115610e6157610e60610af8565b5b610e6d86828701610c0b565b9150509250925092565b5f608082019050610e8a5f830187610ac3565b610e976020830186610a88565b610ea46040830185610ac3565b610eb16060830184610a88565b95945050505050565b5f5f5f60608486031215610ed157610ed0610af4565b5b5f610ede86828701610df7565b9350506020610eef86828701610df7565b9250506040610f0086828701610c78565b9150509250925092565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610f4182610a7f565b9150610f4c83610a7f565b9250828203905081811115610f6457610f63610f0a565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b610fb1610fac82610d4a565b610f97565b82525050565b5f610fc28286610fa0565b602082019150610fd28285610fa0565b602082019150610fe28284610fa0565b602082019150819050949350505050565b610ffc81610ab0565b82525050565b608082015f8201516110165f850182610ff3565b5060208201516110296020850182610ff3565b50604082015161103c6040850182610ff3565b50606082015161104f6060850182610ff3565b50505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6004811061109357611092611055565b5b50565b5f8190506110a382611082565b919050565b5f6110b282611096565b9050919050565b6110c2816110a8565b82525050565b5f60e0820190506110db5f830187610d53565b6110e86020830186610a88565b6110f56040830185611002565b61110260c08301846110b9565b95945050505050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f61113d8261110b565b6111478185611115565b9350611157818560208601611125565b61116081610b04565b840191505092915050565b5f6020820190508181035f8301526111838184611133565b905092915050565b5f8160f81b9050919050565b5f6111a18261118b565b9050919050565b6111b96111b482610d7b565b611197565b82525050565b5f8160601b9050919050565b5f6111d5826111bf565b9050919050565b5f6111e6826111cb565b9050919050565b6111fe6111f982610dd0565b6111dc565b82525050565b5f8160c01b9050919050565b5f61121a82611204565b9050919050565b61123261122d82610ab0565b611210565b82525050565b5f819050919050565b61125261124d82610a7f565b611238565b82525050565b5f611263828a6111a8565b60018201915061127382896111ed565b6014820191506112838288611221565b6008820191506112938287611221565b6008820191506112a38286611241565b6020820191506112b38285611241565b6020820191506112c38284610fa0565b60208201915081905098975050505050505050565b5f6112e38285610fa0565b6020820191506112f38284610fa0565b6020820191508190509392505050565b61130c81610dd0565b82525050565b61131b81610d7b565b82525050565b5f819050919050565b5f819050919050565b5f61134d61134861134384611321565b61132a565b610a7f565b9050919050565b61135d81611333565b82525050565b5f60c0820190506113765f830189611303565b6113836020830188611312565b6113906040830187611303565b61139d6060830186610d53565b6113aa6080830185611354565b6113b760a0830184610ac3565b979650505050505050565b5f6113cd82856111ed565b6014820191506113dd8284611241565b6020820191508190509392505050565b5f6113f782610a7f565b915061140283610a7f565b925082820190508082111561141a57611419610f0a565b5b92915050565b5f61143a61143561143084610ab0565b61132a565b610a7f565b9050919050565b61144a81611420565b82525050565b5f6040820190506114635f830185610a88565b6114706020830184611441565b9392505050565b5f61148182610ab0565b915061148c83610ab0565b9250828203905067ffffffffffffffff8111156114ac576114ab610f0a565b5b92915050565b5f6114bc82610ab0565b91506114c783610ab0565b9250828201905067ffffffffffffffff8111156114e7576114e6610f0a565b5b92915050565b5f6114f88288611221565b6008820191506115088287611221565b6008820191506115188286611221565b6008820191506115288285611221565b6008820191506115388284611221565b6008820191508190509695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b5f81905092915050565b5f61158c8261110b565b6115968185611578565b93506115a6818560208601611125565b80840191505092915050565b5f6115bd8285611582565b91506115c98284611582565b9150819050939250505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R___a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_`\x01U_`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_`\x03U4\x80\x15a\0fW__\xFD[P`@Qa&\x968\x03\x80a&\x96\x839\x81\x81\x01`@R\x81\x01\x90a\0\x88\x91\x90a\x08>V[_\x81Q\x11a\0\xCBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xC2\x90a\x08\xF2V[`@Q\x80\x91\x03\x90\xFD[a\x01\x03`\x0B_\x84`\x01_\x86`@Q` \x01a\0\xE9\x94\x93\x92\x91\x90a\t\xB4V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x01Q` \x1B` \x1CV[a\x01J`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01~\x0B\0\x80\x02\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP____a\x02\xA7` \x1B` \x1CV[PPa\x10\xB4V[_`\x04\x80T\x90P\x90P_\x82\x80Q\x90` \x01 \x90P_\x85\x85CB\x86_\x87`@Q` \x01a\x01\x83\x97\x96\x95\x94\x93\x92\x91\x90a\n\xE2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P___\x1B\x90P_\x84\x11\x15a\x01\xD5W`\x04`\x01\x85a\x01\xB8\x91\x90a\x0B\x8FV[\x81T\x81\x10a\x01\xC9Wa\x01\xC8a\x0B\xC2V[[\x90_R` _ \x01T\x90P[`\x04\x81\x83`@Q` \x01a\x01\xEA\x92\x91\x90a\x0B\xEFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x80\x84\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE10\x8A\x8A\x88_B`@Qa\x02^\x96\x95\x94\x93\x92\x91\x90a\x0C\x98V[`@Q\x80\x91\x03\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x02\x96\x91\x90a\rIV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[\x83__a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82`\x01\x81\x90UP_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x03\x17W\x81`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x03\x81\x90UP[_`\x04\x80T\x90P\x90P__a\x032\x88\x84a\x04\x97` \x1B` \x1CV[\x91P\x91P_`\x05\x80T\x90P\x90P___\x1B\x90P_\x82\x11\x15a\x03yW`\x05`\x01\x83a\x03\\\x91\x90a\x0B\x8FV[\x81T\x81\x10a\x03mWa\x03la\x0B\xC2V[[\x90_R` _ \x01T\x90P[___\x1B\x90P_\x86\x11\x15a\x03\xB3W`\x04`\x01\x87a\x03\x96\x91\x90a\x0B\x8FV[\x81T\x81\x10a\x03\xA7Wa\x03\xA6a\x0B\xC2V[[\x90_R` _ \x01T\x90P[_\x82\x86\x83`@Q` \x01a\x03\xC9\x93\x92\x91\x90a\riV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x05\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x86`\x06\x81\x90UP\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85`\x06T\x8A`\x01`@Qa\x04I\x94\x93\x92\x91\x90a\x0E\x89V[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D`@Qa\x04\x81\x91\x90a\rIV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPV[_a\x04\xA0a\x06rV[_\x84Q`(a\x04\xAF\x91\x90a\x0E\xCCV[\x90Pb\x01\xCC\xCCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\x05\x08W\x80b\x01\xCC\xCC`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xFF\x92\x91\x90a\x0F/V[`@Q\x80\x91\x03\x90\xFD[a\x05\x10a\x06rV[b\x01Q\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15a\x05SWb\x01Q\x80Ba\x054\x91\x90a\x0FVV[\x81_\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[a\x0E\x10Ba\x05a\x91\x90a\x0F\x91V[\x81` \x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x1C g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x11\x15a\x05\xC2Wa\x1C Ca\x05\xA2\x91\x90a\x0FVV[\x81`@\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[`\x0CCa\x05\xCF\x91\x90a\x0F\x91V[\x81``\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x89`@Q` \x01a\x06\x17\x95\x94\x93\x92\x91\x90a\x0F\xCCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\x06:Wa\x069a\x10*V[[\x80\x87`@Q` \x01a\x06M\x92\x91\x90a\x10\x91V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[`@Q\x80`\x80\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\x06\xE1\x81a\x06\xCFV[\x81\x14a\x06\xEBW__\xFD[PV[_\x81Q\x90Pa\x06\xFC\x81a\x06\xD8V[\x92\x91PPV[__\xFD[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x07P\x82a\x07\nV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x07oWa\x07na\x07\x1AV[[\x80`@RPPPV[_a\x07\x81a\x06\xBEV[\x90Pa\x07\x8D\x82\x82a\x07GV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\xACWa\x07\xABa\x07\x1AV[[a\x07\xB5\x82a\x07\nV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x07\xE2a\x07\xDD\x84a\x07\x92V[a\x07xV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x07\xFEWa\x07\xFDa\x07\x06V[[a\x08\t\x84\x82\x85a\x07\xC2V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x08%Wa\x08$a\x07\x02V[[\x81Qa\x085\x84\x82` \x86\x01a\x07\xD0V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x08TWa\x08Sa\x06\xC7V[[_a\x08a\x85\x82\x86\x01a\x06\xEEV[\x92PP` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x82Wa\x08\x81a\x06\xCBV[[a\x08\x8E\x85\x82\x86\x01a\x08\x11V[\x91PP\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FEMPTY_CHAIN_CONFIG\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x08\xDC`\x12\x83a\x08\x98V[\x91Pa\x08\xE7\x82a\x08\xA8V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\t\t\x81a\x08\xD0V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\t*a\t%\x82a\x06\xCFV[a\t\x10V[\x82RPPV[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\tR\x82a\t<V[\x90P\x91\x90PV[a\tja\te\x82a\t0V[a\tHV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_a\t\x8E\x82a\tpV[a\t\x98\x81\x85a\tzV[\x93Pa\t\xA8\x81\x85` \x86\x01a\x07\xC2V[\x80\x84\x01\x91PP\x92\x91PPV[_a\t\xBF\x82\x87a\t\x19V[` \x82\x01\x91Pa\t\xCF\x82\x86a\tYV[`\x01\x82\x01\x91Pa\t\xDF\x82\x85a\t\x19V[` \x82\x01\x91Pa\t\xEF\x82\x84a\t\x84V[\x91P\x81\x90P\x95\x94PPPPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\n&\x82a\t\xFDV[\x90P\x91\x90PV[_\x81``\x1B\x90P\x91\x90PV[_a\nC\x82a\n-V[\x90P\x91\x90PV[_a\nT\x82a\n9V[\x90P\x91\x90PV[a\nla\ng\x82a\n\x1CV[a\nJV[\x82RPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xC0\x1B\x90P\x91\x90PV[_a\n\x9B\x82a\n\x85V[\x90P\x91\x90PV[a\n\xB3a\n\xAE\x82a\nrV[a\n\x91V[\x82RPPV[_\x81\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\n\xDCa\n\xD7\x82a\n\xB9V[a\n\xC2V[\x82RPPV[_a\n\xED\x82\x8Aa\tYV[`\x01\x82\x01\x91Pa\n\xFD\x82\x89a\n[V[`\x14\x82\x01\x91Pa\x0B\r\x82\x88a\n\xA2V[`\x08\x82\x01\x91Pa\x0B\x1D\x82\x87a\n\xA2V[`\x08\x82\x01\x91Pa\x0B-\x82\x86a\t\x19V[` \x82\x01\x91Pa\x0B=\x82\x85a\t\x19V[` \x82\x01\x91Pa\x0BM\x82\x84a\n\xCBV[` \x82\x01\x91P\x81\x90P\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x0B\x99\x82a\x06\xCFV[\x91Pa\x0B\xA4\x83a\x06\xCFV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x0B\xBCWa\x0B\xBBa\x0BbV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_a\x0B\xFA\x82\x85a\n\xCBV[` \x82\x01\x91Pa\x0C\n\x82\x84a\n\xCBV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[a\x0C#\x81a\n\x1CV[\x82RPPV[a\x0C2\x81a\t0V[\x82RPPV[a\x0CA\x81a\n\xB9V[\x82RPPV[_\x81\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x0Csa\x0Cna\x0Ci\x84a\x0CGV[a\x0CPV[a\x06\xCFV[\x90P\x91\x90PV[a\x0C\x83\x81a\x0CYV[\x82RPPV[a\x0C\x92\x81a\nrV[\x82RPPV[_`\xC0\x82\x01\x90Pa\x0C\xAB_\x83\x01\x89a\x0C\x1AV[a\x0C\xB8` \x83\x01\x88a\x0C)V[a\x0C\xC5`@\x83\x01\x87a\x0C\x1AV[a\x0C\xD2``\x83\x01\x86a\x0C8V[a\x0C\xDF`\x80\x83\x01\x85a\x0CzV[a\x0C\xEC`\xA0\x83\x01\x84a\x0C\x89V[\x97\x96PPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\r\x1B\x82a\x0C\xF7V[a\r%\x81\x85a\r\x01V[\x93Pa\r5\x81\x85` \x86\x01a\x07\xC2V[a\r>\x81a\x07\nV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\ra\x81\x84a\r\x11V[\x90P\x92\x91PPV[_a\rt\x82\x86a\n\xCBV[` \x82\x01\x91Pa\r\x84\x82\x85a\n\xCBV[` \x82\x01\x91Pa\r\x94\x82\x84a\n\xCBV[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[a\r\xAE\x81a\x06\xCFV[\x82RPPV[a\r\xBD\x81a\nrV[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\r\xD7_\x85\x01\x82a\r\xB4V[P` \x82\x01Qa\r\xEA` \x85\x01\x82a\r\xB4V[P`@\x82\x01Qa\r\xFD`@\x85\x01\x82a\r\xB4V[P``\x82\x01Qa\x0E\x10``\x85\x01\x82a\r\xB4V[PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x0ETWa\x0ESa\x0E\x16V[[PV[_\x81\x90Pa\x0Ed\x82a\x0ECV[\x91\x90PV[_a\x0Es\x82a\x0EWV[\x90P\x91\x90PV[a\x0E\x83\x81a\x0EiV[\x82RPPV[_`\xE0\x82\x01\x90Pa\x0E\x9C_\x83\x01\x87a\x0C8V[a\x0E\xA9` \x83\x01\x86a\r\xA5V[a\x0E\xB6`@\x83\x01\x85a\r\xC3V[a\x0E\xC3`\xC0\x83\x01\x84a\x0EzV[\x95\x94PPPPPV[_a\x0E\xD6\x82a\x06\xCFV[\x91Pa\x0E\xE1\x83a\x06\xCFV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x0E\xF9Wa\x0E\xF8a\x0BbV[[\x92\x91PPV[_a\x0F\x19a\x0F\x14a\x0F\x0F\x84a\nrV[a\x0CPV[a\x06\xCFV[\x90P\x91\x90PV[a\x0F)\x81a\x0E\xFFV[\x82RPPV[_`@\x82\x01\x90Pa\x0FB_\x83\x01\x85a\r\xA5V[a\x0FO` \x83\x01\x84a\x0F V[\x93\x92PPPV[_a\x0F`\x82a\nrV[\x91Pa\x0Fk\x83a\nrV[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x8BWa\x0F\x8Aa\x0BbV[[\x92\x91PPV[_a\x0F\x9B\x82a\nrV[\x91Pa\x0F\xA6\x83a\nrV[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xC6Wa\x0F\xC5a\x0BbV[[\x92\x91PPV[_a\x0F\xD7\x82\x88a\n\xA2V[`\x08\x82\x01\x91Pa\x0F\xE7\x82\x87a\n\xA2V[`\x08\x82\x01\x91Pa\x0F\xF7\x82\x86a\n\xA2V[`\x08\x82\x01\x91Pa\x10\x07\x82\x85a\n\xA2V[`\x08\x82\x01\x91Pa\x10\x17\x82\x84a\n\xA2V[`\x08\x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[_\x81\x90P\x92\x91PPV[_a\x10k\x82a\x0C\xF7V[a\x10u\x81\x85a\x10WV[\x93Pa\x10\x85\x81\x85` \x86\x01a\x07\xC2V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x10\x9C\x82\x85a\x10aV[\x91Pa\x10\xA8\x82\x84a\x10aV[\x91P\x81\x90P\x93\x92PPPV[a\x15\xD5\x80a\x10\xC1_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01)W_5`\xE0\x1C\x80c\xA7\xB5\x1D\x19\x11a\0\xABW\x80c\xD9\xDDg\xAB\x11a\0oW\x80c\xD9\xDDg\xAB\x14a\x03\x12W\x80c\xE1\xD6j\xFE\x14a\x03BW\x80c\xE8\xEB\x1D\xC3\x14a\x03^W\x80c\xEC\xA0g\xAD\x14a\x03|W\x80c\xFB\xF6\xEA\xA5\x14a\x03\x9AWa\x01)V[\x80c\xA7\xB5\x1D\x19\x14a\x02iW\x80c\xAD\x9C\x0C.\x14a\x02\x85W\x80c\xB7R\xA7\xD1\x14a\x02\xA3W\x80c\xD5q\x9D\xC2\x14a\x02\xC1W\x80c\xD5\x95L4\x14a\x02\xF1Wa\x01)V[\x80c\x16\xBFUy\x11a\0\xF2W\x80c\x16\xBFUy\x14a\x01\xC1W\x80c\x18\xDB9@\x14a\x01\xF1W\x80c/\x1E\xC5\xE9\x14a\x02\x0FW\x80cO5\x9A7\x14a\x02-W\x80c\x7F\xA3\xA4\x0E\x14a\x02KWa\x01)V[\x80b\x84\x12\x0C\x14a\x01-W\x80c\x04\xF1\xC8T\x14a\x01KW\x80c\x05m\xAA\xA6\x14a\x01iW\x80c\x06\x1D\x12\xC0\x14a\x01\x87W\x80c\x06\xF10V\x14a\x01\xA3W[__\xFD[a\x015a\x03\xB8V[`@Qa\x01B\x91\x90a\n\x97V[`@Q\x80\x91\x03\x90\xF3[a\x01Sa\x03\xC4V[`@Qa\x01`\x91\x90a\n\x97V[`@Q\x80\x91\x03\x90\xF3[a\x01qa\x03\xCAV[`@Qa\x01~\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA1`\x04\x806\x03\x81\x01\x90a\x01\x9C\x91\x90a\x0C\x8CV[a\x03\xE2V[\0[a\x01\xABa\x05\xCCV[`@Qa\x01\xB8\x91\x90a\n\x97V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDB`\x04\x806\x03\x81\x01\x90a\x01\xD6\x91\x90a\r\x1FV[a\x05\xD8V[`@Qa\x01\xE8\x91\x90a\rbV[`@Q\x80\x91\x03\x90\xF3[a\x01\xF9a\x05\xF8V[`@Qa\x02\x06\x91\x90a\n\x97V[`@Q\x80\x91\x03\x90\xF3[a\x02\x17a\x05\xFEV[`@Qa\x02$\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x025a\x06\x04V[`@Qa\x02B\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x02Sa\x06\tV[`@Qa\x02`\x91\x90a\n\x97V[`@Q\x80\x91\x03\x90\xF3[a\x02\x83`\x04\x806\x03\x81\x01\x90a\x02~\x91\x90a\x0E\x0BV[a\x06\x0FV[\0[a\x02\x8Da\x07eV[`@Qa\x02\x9A\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x02\xABa\x07kV[`@Qa\x02\xB8\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x02\xDB`\x04\x806\x03\x81\x01\x90a\x02\xD6\x91\x90a\r\x1FV[a\x07rV[`@Qa\x02\xE8\x91\x90a\rbV[`@Q\x80\x91\x03\x90\xF3[a\x02\xF9a\x07\x92V[`@Qa\x03\t\x94\x93\x92\x91\x90a\x0EwV[`@Q\x80\x91\x03\x90\xF3[a\x03,`\x04\x806\x03\x81\x01\x90a\x03'\x91\x90a\r\x1FV[a\x07\xD5V[`@Qa\x039\x91\x90a\rbV[`@Q\x80\x91\x03\x90\xF3[a\x03\\`\x04\x806\x03\x81\x01\x90a\x03W\x91\x90a\x0E\xBAV[a\x07\xFAV[\0[a\x03fa\x08,V[`@Qa\x03s\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x03\x84a\x083V[`@Qa\x03\x91\x91\x90a\n\x97V[`@Q\x80\x91\x03\x90\xF3[a\x03\xA2a\x08?V[`@Qa\x03\xAF\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[_`\x05\x80T\x90P\x90P\x90V[`\x03T\x81V[__\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[\x83__a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82`\x01\x81\x90UP_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x04RW\x81`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x03\x81\x90UP[_`\x04\x80T\x90P\x90P__a\x04g\x88\x84a\x08XV[\x91P\x91P_`\x05\x80T\x90P\x90P___\x1B\x90P_\x82\x11\x15a\x04\xAEW`\x05`\x01\x83a\x04\x91\x91\x90a\x0F7V[\x81T\x81\x10a\x04\xA2Wa\x04\xA1a\x0FjV[[\x90_R` _ \x01T\x90P[___\x1B\x90P_\x86\x11\x15a\x04\xE8W`\x04`\x01\x87a\x04\xCB\x91\x90a\x0F7V[\x81T\x81\x10a\x04\xDCWa\x04\xDBa\x0FjV[[\x90_R` _ \x01T\x90P[_\x82\x86\x83`@Q` \x01a\x04\xFE\x93\x92\x91\x90a\x0F\xB7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x05\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x86`\x06\x81\x90UP\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85`\x06T\x8A`\x01`@Qa\x05~\x94\x93\x92\x91\x90a\x10\xC8V[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D`@Qa\x05\xB6\x91\x90a\x11kV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPV[_`\x05\x80T\x90P\x90P\x90V[`\x05\x81\x81T\x81\x10a\x05\xE7W_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[`\x01T\x81V[a\x0E\x10\x81V[`\x0C\x81V[`\x06T\x81V[_`\x04\x80T\x90P\x90P_\x82\x80Q\x90` \x01 \x90P_\x85\x85CB\x86_\x87`@Q` \x01a\x06A\x97\x96\x95\x94\x93\x92\x91\x90a\x12XV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P___\x1B\x90P_\x84\x11\x15a\x06\x93W`\x04`\x01\x85a\x06v\x91\x90a\x0F7V[\x81T\x81\x10a\x06\x87Wa\x06\x86a\x0FjV[[\x90_R` _ \x01T\x90P[`\x04\x81\x83`@Q` \x01a\x06\xA8\x92\x91\x90a\x12\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x80\x84\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE10\x8A\x8A\x88_B`@Qa\x07\x1C\x96\x95\x94\x93\x92\x91\x90a\x13cV[`@Q\x80\x91\x03\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x07T\x91\x90a\x11kV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[a\x1C \x81V[b\x01Q\x80\x81V[`\x04\x81\x81T\x81\x10a\x07\x81W_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[______\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01T`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03T\x93P\x93P\x93P\x93P\x90\x91\x92\x93V[_`\x05\x82\x81T\x81\x10a\x07\xEAWa\x07\xE9a\x0FjV[[\x90_R` _ \x01T\x90P\x91\x90PV[a\x08'`\x0C\x84\x84\x84`@Q` \x01a\x08\x13\x92\x91\x90a\x13\xC2V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x06\x0FV[PPPV[b\x01\xCC\xCC\x81V[_`\x04\x80T\x90P\x90P\x90V[`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[_a\x08aa\n3V[_\x84Q`(a\x08p\x91\x90a\x13\xEDV[\x90Pb\x01\xCC\xCCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\x08\xC9W\x80b\x01\xCC\xCC`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xC0\x92\x91\x90a\x14PV[`@Q\x80\x91\x03\x90\xFD[a\x08\xD1a\n3V[b\x01Q\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15a\t\x14Wb\x01Q\x80Ba\x08\xF5\x91\x90a\x14wV[\x81_\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[a\x0E\x10Ba\t\"\x91\x90a\x14\xB2V[\x81` \x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x1C g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x11\x15a\t\x83Wa\x1C Ca\tc\x91\x90a\x14wV[\x81`@\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[`\x0CCa\t\x90\x91\x90a\x14\xB2V[\x81``\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x89`@Q` \x01a\t\xD8\x95\x94\x93\x92\x91\x90a\x14\xEDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\t\xFBWa\t\xFAa\x15KV[[\x80\x87`@Q` \x01a\n\x0E\x92\x91\x90a\x15\xB2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[`@Q\x80`\x80\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_\x81\x90P\x91\x90PV[a\n\x91\x81a\n\x7FV[\x82RPPV[_` \x82\x01\x90Pa\n\xAA_\x83\x01\x84a\n\x88V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\n\xCC\x81a\n\xB0V[\x82RPPV[_` \x82\x01\x90Pa\n\xE5_\x83\x01\x84a\n\xC3V[\x92\x91PPV[_`@Q\x90P\x90V[__\xFD[__\xFD[__\xFD[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0BJ\x82a\x0B\x04V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0BiWa\x0Bha\x0B\x14V[[\x80`@RPPPV[_a\x0B{a\n\xEBV[\x90Pa\x0B\x87\x82\x82a\x0BAV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\xA6Wa\x0B\xA5a\x0B\x14V[[a\x0B\xAF\x82a\x0B\x04V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0B\xDCa\x0B\xD7\x84a\x0B\x8CV[a\x0BrV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0B\xF8Wa\x0B\xF7a\x0B\0V[[a\x0C\x03\x84\x82\x85a\x0B\xBCV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0C\x1FWa\x0C\x1Ea\n\xFCV[[\x815a\x0C/\x84\x82` \x86\x01a\x0B\xCAV[\x91PP\x92\x91PPV[a\x0CA\x81a\n\xB0V[\x81\x14a\x0CKW__\xFD[PV[_\x815\x90Pa\x0C\\\x81a\x0C8V[\x92\x91PPV[a\x0Ck\x81a\n\x7FV[\x81\x14a\x0CuW__\xFD[PV[_\x815\x90Pa\x0C\x86\x81a\x0CbV[\x92\x91PPV[_____`\xA0\x86\x88\x03\x12\x15a\x0C\xA5Wa\x0C\xA4a\n\xF4V[[_\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xC2Wa\x0C\xC1a\n\xF8V[[a\x0C\xCE\x88\x82\x89\x01a\x0C\x0BV[\x95PP` a\x0C\xDF\x88\x82\x89\x01a\x0CNV[\x94PP`@a\x0C\xF0\x88\x82\x89\x01a\x0CxV[\x93PP``a\r\x01\x88\x82\x89\x01a\x0CNV[\x92PP`\x80a\r\x12\x88\x82\x89\x01a\x0CxV[\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a\r4Wa\r3a\n\xF4V[[_a\rA\x84\x82\x85\x01a\x0CxV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\r\\\x81a\rJV[\x82RPPV[_` \x82\x01\x90Pa\ru_\x83\x01\x84a\rSV[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\r\x90\x81a\r{V[\x81\x14a\r\x9AW__\xFD[PV[_\x815\x90Pa\r\xAB\x81a\r\x87V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\r\xDA\x82a\r\xB1V[\x90P\x91\x90PV[a\r\xEA\x81a\r\xD0V[\x81\x14a\r\xF4W__\xFD[PV[_\x815\x90Pa\x0E\x05\x81a\r\xE1V[\x92\x91PPV[___``\x84\x86\x03\x12\x15a\x0E\"Wa\x0E!a\n\xF4V[[_a\x0E/\x86\x82\x87\x01a\r\x9DV[\x93PP` a\x0E@\x86\x82\x87\x01a\r\xF7V[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EaWa\x0E`a\n\xF8V[[a\x0Em\x86\x82\x87\x01a\x0C\x0BV[\x91PP\x92P\x92P\x92V[_`\x80\x82\x01\x90Pa\x0E\x8A_\x83\x01\x87a\n\xC3V[a\x0E\x97` \x83\x01\x86a\n\x88V[a\x0E\xA4`@\x83\x01\x85a\n\xC3V[a\x0E\xB1``\x83\x01\x84a\n\x88V[\x95\x94PPPPPV[___``\x84\x86\x03\x12\x15a\x0E\xD1Wa\x0E\xD0a\n\xF4V[[_a\x0E\xDE\x86\x82\x87\x01a\r\xF7V[\x93PP` a\x0E\xEF\x86\x82\x87\x01a\r\xF7V[\x92PP`@a\x0F\0\x86\x82\x87\x01a\x0CxV[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x0FA\x82a\n\x7FV[\x91Pa\x0FL\x83a\n\x7FV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x0FdWa\x0Fca\x0F\nV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[a\x0F\xB1a\x0F\xAC\x82a\rJV[a\x0F\x97V[\x82RPPV[_a\x0F\xC2\x82\x86a\x0F\xA0V[` \x82\x01\x91Pa\x0F\xD2\x82\x85a\x0F\xA0V[` \x82\x01\x91Pa\x0F\xE2\x82\x84a\x0F\xA0V[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[a\x0F\xFC\x81a\n\xB0V[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\x10\x16_\x85\x01\x82a\x0F\xF3V[P` \x82\x01Qa\x10)` \x85\x01\x82a\x0F\xF3V[P`@\x82\x01Qa\x10<`@\x85\x01\x82a\x0F\xF3V[P``\x82\x01Qa\x10O``\x85\x01\x82a\x0F\xF3V[PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x10\x93Wa\x10\x92a\x10UV[[PV[_\x81\x90Pa\x10\xA3\x82a\x10\x82V[\x91\x90PV[_a\x10\xB2\x82a\x10\x96V[\x90P\x91\x90PV[a\x10\xC2\x81a\x10\xA8V[\x82RPPV[_`\xE0\x82\x01\x90Pa\x10\xDB_\x83\x01\x87a\rSV[a\x10\xE8` \x83\x01\x86a\n\x88V[a\x10\xF5`@\x83\x01\x85a\x10\x02V[a\x11\x02`\xC0\x83\x01\x84a\x10\xB9V[\x95\x94PPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x11=\x82a\x11\x0BV[a\x11G\x81\x85a\x11\x15V[\x93Pa\x11W\x81\x85` \x86\x01a\x11%V[a\x11`\x81a\x0B\x04V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x11\x83\x81\x84a\x113V[\x90P\x92\x91PPV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\x11\xA1\x82a\x11\x8BV[\x90P\x91\x90PV[a\x11\xB9a\x11\xB4\x82a\r{V[a\x11\x97V[\x82RPPV[_\x81``\x1B\x90P\x91\x90PV[_a\x11\xD5\x82a\x11\xBFV[\x90P\x91\x90PV[_a\x11\xE6\x82a\x11\xCBV[\x90P\x91\x90PV[a\x11\xFEa\x11\xF9\x82a\r\xD0V[a\x11\xDCV[\x82RPPV[_\x81`\xC0\x1B\x90P\x91\x90PV[_a\x12\x1A\x82a\x12\x04V[\x90P\x91\x90PV[a\x122a\x12-\x82a\n\xB0V[a\x12\x10V[\x82RPPV[_\x81\x90P\x91\x90PV[a\x12Ra\x12M\x82a\n\x7FV[a\x128V[\x82RPPV[_a\x12c\x82\x8Aa\x11\xA8V[`\x01\x82\x01\x91Pa\x12s\x82\x89a\x11\xEDV[`\x14\x82\x01\x91Pa\x12\x83\x82\x88a\x12!V[`\x08\x82\x01\x91Pa\x12\x93\x82\x87a\x12!V[`\x08\x82\x01\x91Pa\x12\xA3\x82\x86a\x12AV[` \x82\x01\x91Pa\x12\xB3\x82\x85a\x12AV[` \x82\x01\x91Pa\x12\xC3\x82\x84a\x0F\xA0V[` \x82\x01\x91P\x81\x90P\x98\x97PPPPPPPPV[_a\x12\xE3\x82\x85a\x0F\xA0V[` \x82\x01\x91Pa\x12\xF3\x82\x84a\x0F\xA0V[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[a\x13\x0C\x81a\r\xD0V[\x82RPPV[a\x13\x1B\x81a\r{V[\x82RPPV[_\x81\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x13Ma\x13Ha\x13C\x84a\x13!V[a\x13*V[a\n\x7FV[\x90P\x91\x90PV[a\x13]\x81a\x133V[\x82RPPV[_`\xC0\x82\x01\x90Pa\x13v_\x83\x01\x89a\x13\x03V[a\x13\x83` \x83\x01\x88a\x13\x12V[a\x13\x90`@\x83\x01\x87a\x13\x03V[a\x13\x9D``\x83\x01\x86a\rSV[a\x13\xAA`\x80\x83\x01\x85a\x13TV[a\x13\xB7`\xA0\x83\x01\x84a\n\xC3V[\x97\x96PPPPPPPV[_a\x13\xCD\x82\x85a\x11\xEDV[`\x14\x82\x01\x91Pa\x13\xDD\x82\x84a\x12AV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[_a\x13\xF7\x82a\n\x7FV[\x91Pa\x14\x02\x83a\n\x7FV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x14\x1AWa\x14\x19a\x0F\nV[[\x92\x91PPV[_a\x14:a\x145a\x140\x84a\n\xB0V[a\x13*V[a\n\x7FV[\x90P\x91\x90PV[a\x14J\x81a\x14 V[\x82RPPV[_`@\x82\x01\x90Pa\x14c_\x83\x01\x85a\n\x88V[a\x14p` \x83\x01\x84a\x14AV[\x93\x92PPPV[_a\x14\x81\x82a\n\xB0V[\x91Pa\x14\x8C\x83a\n\xB0V[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xACWa\x14\xABa\x0F\nV[[\x92\x91PPV[_a\x14\xBC\x82a\n\xB0V[\x91Pa\x14\xC7\x83a\n\xB0V[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xE7Wa\x14\xE6a\x0F\nV[[\x92\x91PPV[_a\x14\xF8\x82\x88a\x12!V[`\x08\x82\x01\x91Pa\x15\x08\x82\x87a\x12!V[`\x08\x82\x01\x91Pa\x15\x18\x82\x86a\x12!V[`\x08\x82\x01\x91Pa\x15(\x82\x85a\x12!V[`\x08\x82\x01\x91Pa\x158\x82\x84a\x12!V[`\x08\x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[_\x81\x90P\x92\x91PPV[_a\x15\x8C\x82a\x11\x0BV[a\x15\x96\x81\x85a\x15xV[\x93Pa\x15\xA6\x81\x85` \x86\x01a\x11%V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x15\xBD\x82\x85a\x15\x82V[\x91Pa\x15\xC9\x82\x84a\x15\x82V[\x91P\x81\x90P\x93\x92PPPV",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610129575f3560e01c8063a7b51d19116100ab578063d9dd67ab1161006f578063d9dd67ab14610312578063e1d66afe14610342578063e8eb1dc31461035e578063eca067ad1461037c578063fbf6eaa51461039a57610129565b8063a7b51d1914610269578063ad9c0c2e14610285578063b752a7d1146102a3578063d5719dc2146102c1578063d5954c34146102f157610129565b806316bf5579116100f257806316bf5579146101c157806318db3940146101f15780632f1ec5e91461020f5780634f359a371461022d5780637fa3a40e1461024b57610129565b806284120c1461012d57806304f1c8541461014b578063056daaa614610169578063061d12c01461018757806306f13056146101a3575b5f5ffd5b6101356103b8565b6040516101429190610a97565b60405180910390f35b6101536103c4565b6040516101609190610a97565b60405180910390f35b6101716103ca565b60405161017e9190610ad2565b60405180910390f35b6101a1600480360381019061019c9190610c8c565b6103e2565b005b6101ab6105cc565b6040516101b89190610a97565b60405180910390f35b6101db60048036038101906101d69190610d1f565b6105d8565b6040516101e89190610d62565b60405180910390f35b6101f96105f8565b6040516102069190610a97565b60405180910390f35b6102176105fe565b6040516102249190610ad2565b60405180910390f35b610235610604565b6040516102429190610ad2565b60405180910390f35b610253610609565b6040516102609190610a97565b60405180910390f35b610283600480360381019061027e9190610e0b565b61060f565b005b61028d610765565b60405161029a9190610ad2565b60405180910390f35b6102ab61076b565b6040516102b89190610ad2565b60405180910390f35b6102db60048036038101906102d69190610d1f565b610772565b6040516102e89190610d62565b60405180910390f35b6102f9610792565b6040516103099493929190610e77565b60405180910390f35b61032c60048036038101906103279190610d1f565b6107d5565b6040516103399190610d62565b60405180910390f35b61035c60048036038101906103579190610eba565b6107fa565b005b61036661082c565b6040516103739190610ad2565b60405180910390f35b610384610833565b6040516103919190610a97565b60405180910390f35b6103a261083f565b6040516103af9190610ad2565b60405180910390f35b5f600580549050905090565b60035481565b5f5f9054906101000a900467ffffffffffffffff1681565b835f5f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550826001819055505f8267ffffffffffffffff161115610452578160025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550806003819055505b5f60048054905090505f5f6104678884610858565b915091505f60058054905090505f5f5f1b90505f8211156104ae5760056001836104919190610f37565b815481106104a2576104a1610f6a565b5b905f5260205f20015490505b5f5f5f1b90505f8611156104e85760046001876104cb9190610f37565b815481106104dc576104db610f6a565b5b905f5260205f20015490505b5f8286836040516020016104fe93929190610fb7565b604051602081830303815290604052805190602001209050600581908060018154018082558091505060019003905f5260205f20015f9091909190915055866006819055508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7856006548a600160405161057e94939291906110c8565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d6040516105b6919061116b565b60405180910390a2505050505050505050505050565b5f600580549050905090565b600581815481106105e7575f80fd5b905f5260205f20015f915090505481565b60015481565b610e1081565b600c81565b60065481565b5f60048054905090505f828051906020012090505f85854342865f876040516020016106419796959493929190611258565b6040516020818303038152906040528051906020012090505f5f5f1b90505f8411156106935760046001856106769190610f37565b8154811061068757610686610f6a565b5b905f5260205f20015490505b600481836040516020016106a89291906112d8565b60405160208183030381529060405280519060200120908060018154018082558091505060019003905f5260205f20015f909190919091505580847f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1308a8a885f4260405161071c96959493929190611363565b60405180910390a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b86604051610754919061116b565b60405180910390a250505050505050565b611c2081565b6201518081565b60048181548110610781575f80fd5b905f5260205f20015f915090505481565b5f5f5f5f5f5f9054906101000a900467ffffffffffffffff1660015460025f9054906101000a900467ffffffffffffffff16600354935093509350935090919293565b5f600582815481106107ea576107e9610f6a565b5b905f5260205f2001549050919050565b610827600c8484846040516020016108139291906113c2565b60405160208183030381529060405261060f565b505050565b6201cccc81565b5f600480549050905090565b60025f9054906101000a900467ffffffffffffffff1681565b5f610861610a33565b5f8451602861087091906113ed565b90506201cccc67ffffffffffffffff168111156108c957806201cccc6040517f4634691b0000000000000000000000000000000000000000000000000000000081526004016108c0929190611450565b60405180910390fd5b6108d1610a33565b6201518067ffffffffffffffff164211156109145762015180426108f59190611477565b815f019067ffffffffffffffff16908167ffffffffffffffff16815250505b610e104261092291906114b2565b816020019067ffffffffffffffff16908167ffffffffffffffff1681525050611c2067ffffffffffffffff1643111561098357611c20436109639190611477565b816040019067ffffffffffffffff16908167ffffffffffffffff16815250505b600c4361099091906114b2565b816060019067ffffffffffffffff16908167ffffffffffffffff16815250505f815f0151826020015183604001518460600151896040516020016109d89594939291906114ed565b604051602081830303815290604052905060288151146109fb576109fa61154b565b5b8087604051602001610a0e9291906115b2565b6040516020818303038152906040528051906020012082945094505050509250929050565b60405180608001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681525090565b5f819050919050565b610a9181610a7f565b82525050565b5f602082019050610aaa5f830184610a88565b92915050565b5f67ffffffffffffffff82169050919050565b610acc81610ab0565b82525050565b5f602082019050610ae55f830184610ac3565b92915050565b5f604051905090565b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610b4a82610b04565b810181811067ffffffffffffffff82111715610b6957610b68610b14565b5b80604052505050565b5f610b7b610aeb565b9050610b878282610b41565b919050565b5f67ffffffffffffffff821115610ba657610ba5610b14565b5b610baf82610b04565b9050602081019050919050565b828183375f83830152505050565b5f610bdc610bd784610b8c565b610b72565b905082815260208101848484011115610bf857610bf7610b00565b5b610c03848285610bbc565b509392505050565b5f82601f830112610c1f57610c1e610afc565b5b8135610c2f848260208601610bca565b91505092915050565b610c4181610ab0565b8114610c4b575f5ffd5b50565b5f81359050610c5c81610c38565b92915050565b610c6b81610a7f565b8114610c75575f5ffd5b50565b5f81359050610c8681610c62565b92915050565b5f5f5f5f5f60a08688031215610ca557610ca4610af4565b5b5f86013567ffffffffffffffff811115610cc257610cc1610af8565b5b610cce88828901610c0b565b9550506020610cdf88828901610c4e565b9450506040610cf088828901610c78565b9350506060610d0188828901610c4e565b9250506080610d1288828901610c78565b9150509295509295909350565b5f60208284031215610d3457610d33610af4565b5b5f610d4184828501610c78565b91505092915050565b5f819050919050565b610d5c81610d4a565b82525050565b5f602082019050610d755f830184610d53565b92915050565b5f60ff82169050919050565b610d9081610d7b565b8114610d9a575f5ffd5b50565b5f81359050610dab81610d87565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610dda82610db1565b9050919050565b610dea81610dd0565b8114610df4575f5ffd5b50565b5f81359050610e0581610de1565b92915050565b5f5f5f60608486031215610e2257610e21610af4565b5b5f610e2f86828701610d9d565b9350506020610e4086828701610df7565b925050604084013567ffffffffffffffff811115610e6157610e60610af8565b5b610e6d86828701610c0b565b9150509250925092565b5f608082019050610e8a5f830187610ac3565b610e976020830186610a88565b610ea46040830185610ac3565b610eb16060830184610a88565b95945050505050565b5f5f5f60608486031215610ed157610ed0610af4565b5b5f610ede86828701610df7565b9350506020610eef86828701610df7565b9250506040610f0086828701610c78565b9150509250925092565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610f4182610a7f565b9150610f4c83610a7f565b9250828203905081811115610f6457610f63610f0a565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b610fb1610fac82610d4a565b610f97565b82525050565b5f610fc28286610fa0565b602082019150610fd28285610fa0565b602082019150610fe28284610fa0565b602082019150819050949350505050565b610ffc81610ab0565b82525050565b608082015f8201516110165f850182610ff3565b5060208201516110296020850182610ff3565b50604082015161103c6040850182610ff3565b50606082015161104f6060850182610ff3565b50505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6004811061109357611092611055565b5b50565b5f8190506110a382611082565b919050565b5f6110b282611096565b9050919050565b6110c2816110a8565b82525050565b5f60e0820190506110db5f830187610d53565b6110e86020830186610a88565b6110f56040830185611002565b61110260c08301846110b9565b95945050505050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f61113d8261110b565b6111478185611115565b9350611157818560208601611125565b61116081610b04565b840191505092915050565b5f6020820190508181035f8301526111838184611133565b905092915050565b5f8160f81b9050919050565b5f6111a18261118b565b9050919050565b6111b96111b482610d7b565b611197565b82525050565b5f8160601b9050919050565b5f6111d5826111bf565b9050919050565b5f6111e6826111cb565b9050919050565b6111fe6111f982610dd0565b6111dc565b82525050565b5f8160c01b9050919050565b5f61121a82611204565b9050919050565b61123261122d82610ab0565b611210565b82525050565b5f819050919050565b61125261124d82610a7f565b611238565b82525050565b5f611263828a6111a8565b60018201915061127382896111ed565b6014820191506112838288611221565b6008820191506112938287611221565b6008820191506112a38286611241565b6020820191506112b38285611241565b6020820191506112c38284610fa0565b60208201915081905098975050505050505050565b5f6112e38285610fa0565b6020820191506112f38284610fa0565b6020820191508190509392505050565b61130c81610dd0565b82525050565b61131b81610d7b565b82525050565b5f819050919050565b5f819050919050565b5f61134d61134861134384611321565b61132a565b610a7f565b9050919050565b61135d81611333565b82525050565b5f60c0820190506113765f830189611303565b6113836020830188611312565b6113906040830187611303565b61139d6060830186610d53565b6113aa6080830185611354565b6113b760a0830184610ac3565b979650505050505050565b5f6113cd82856111ed565b6014820191506113dd8284611241565b6020820191508190509392505050565b5f6113f782610a7f565b915061140283610a7f565b925082820190508082111561141a57611419610f0a565b5b92915050565b5f61143a61143561143084610ab0565b61132a565b610a7f565b9050919050565b61144a81611420565b82525050565b5f6040820190506114635f830185610a88565b6114706020830184611441565b9392505050565b5f61148182610ab0565b915061148c83610ab0565b9250828203905067ffffffffffffffff8111156114ac576114ab610f0a565b5b92915050565b5f6114bc82610ab0565b91506114c783610ab0565b9250828201905067ffffffffffffffff8111156114e7576114e6610f0a565b5b92915050565b5f6114f88288611221565b6008820191506115088287611221565b6008820191506115188286611221565b6008820191506115288285611221565b6008820191506115388284611221565b6008820191508190509695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b5f81905092915050565b5f61158c8261110b565b6115968185611578565b93506115a6818560208601611125565b80840191505092915050565b5f6115bd8285611582565b91506115c98284611582565b9150819050939250505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01)W_5`\xE0\x1C\x80c\xA7\xB5\x1D\x19\x11a\0\xABW\x80c\xD9\xDDg\xAB\x11a\0oW\x80c\xD9\xDDg\xAB\x14a\x03\x12W\x80c\xE1\xD6j\xFE\x14a\x03BW\x80c\xE8\xEB\x1D\xC3\x14a\x03^W\x80c\xEC\xA0g\xAD\x14a\x03|W\x80c\xFB\xF6\xEA\xA5\x14a\x03\x9AWa\x01)V[\x80c\xA7\xB5\x1D\x19\x14a\x02iW\x80c\xAD\x9C\x0C.\x14a\x02\x85W\x80c\xB7R\xA7\xD1\x14a\x02\xA3W\x80c\xD5q\x9D\xC2\x14a\x02\xC1W\x80c\xD5\x95L4\x14a\x02\xF1Wa\x01)V[\x80c\x16\xBFUy\x11a\0\xF2W\x80c\x16\xBFUy\x14a\x01\xC1W\x80c\x18\xDB9@\x14a\x01\xF1W\x80c/\x1E\xC5\xE9\x14a\x02\x0FW\x80cO5\x9A7\x14a\x02-W\x80c\x7F\xA3\xA4\x0E\x14a\x02KWa\x01)V[\x80b\x84\x12\x0C\x14a\x01-W\x80c\x04\xF1\xC8T\x14a\x01KW\x80c\x05m\xAA\xA6\x14a\x01iW\x80c\x06\x1D\x12\xC0\x14a\x01\x87W\x80c\x06\xF10V\x14a\x01\xA3W[__\xFD[a\x015a\x03\xB8V[`@Qa\x01B\x91\x90a\n\x97V[`@Q\x80\x91\x03\x90\xF3[a\x01Sa\x03\xC4V[`@Qa\x01`\x91\x90a\n\x97V[`@Q\x80\x91\x03\x90\xF3[a\x01qa\x03\xCAV[`@Qa\x01~\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA1`\x04\x806\x03\x81\x01\x90a\x01\x9C\x91\x90a\x0C\x8CV[a\x03\xE2V[\0[a\x01\xABa\x05\xCCV[`@Qa\x01\xB8\x91\x90a\n\x97V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDB`\x04\x806\x03\x81\x01\x90a\x01\xD6\x91\x90a\r\x1FV[a\x05\xD8V[`@Qa\x01\xE8\x91\x90a\rbV[`@Q\x80\x91\x03\x90\xF3[a\x01\xF9a\x05\xF8V[`@Qa\x02\x06\x91\x90a\n\x97V[`@Q\x80\x91\x03\x90\xF3[a\x02\x17a\x05\xFEV[`@Qa\x02$\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x025a\x06\x04V[`@Qa\x02B\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x02Sa\x06\tV[`@Qa\x02`\x91\x90a\n\x97V[`@Q\x80\x91\x03\x90\xF3[a\x02\x83`\x04\x806\x03\x81\x01\x90a\x02~\x91\x90a\x0E\x0BV[a\x06\x0FV[\0[a\x02\x8Da\x07eV[`@Qa\x02\x9A\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x02\xABa\x07kV[`@Qa\x02\xB8\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x02\xDB`\x04\x806\x03\x81\x01\x90a\x02\xD6\x91\x90a\r\x1FV[a\x07rV[`@Qa\x02\xE8\x91\x90a\rbV[`@Q\x80\x91\x03\x90\xF3[a\x02\xF9a\x07\x92V[`@Qa\x03\t\x94\x93\x92\x91\x90a\x0EwV[`@Q\x80\x91\x03\x90\xF3[a\x03,`\x04\x806\x03\x81\x01\x90a\x03'\x91\x90a\r\x1FV[a\x07\xD5V[`@Qa\x039\x91\x90a\rbV[`@Q\x80\x91\x03\x90\xF3[a\x03\\`\x04\x806\x03\x81\x01\x90a\x03W\x91\x90a\x0E\xBAV[a\x07\xFAV[\0[a\x03fa\x08,V[`@Qa\x03s\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x03\x84a\x083V[`@Qa\x03\x91\x91\x90a\n\x97V[`@Q\x80\x91\x03\x90\xF3[a\x03\xA2a\x08?V[`@Qa\x03\xAF\x91\x90a\n\xD2V[`@Q\x80\x91\x03\x90\xF3[_`\x05\x80T\x90P\x90P\x90V[`\x03T\x81V[__\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[\x83__a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82`\x01\x81\x90UP_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x04RW\x81`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x03\x81\x90UP[_`\x04\x80T\x90P\x90P__a\x04g\x88\x84a\x08XV[\x91P\x91P_`\x05\x80T\x90P\x90P___\x1B\x90P_\x82\x11\x15a\x04\xAEW`\x05`\x01\x83a\x04\x91\x91\x90a\x0F7V[\x81T\x81\x10a\x04\xA2Wa\x04\xA1a\x0FjV[[\x90_R` _ \x01T\x90P[___\x1B\x90P_\x86\x11\x15a\x04\xE8W`\x04`\x01\x87a\x04\xCB\x91\x90a\x0F7V[\x81T\x81\x10a\x04\xDCWa\x04\xDBa\x0FjV[[\x90_R` _ \x01T\x90P[_\x82\x86\x83`@Q` \x01a\x04\xFE\x93\x92\x91\x90a\x0F\xB7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x05\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x86`\x06\x81\x90UP\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85`\x06T\x8A`\x01`@Qa\x05~\x94\x93\x92\x91\x90a\x10\xC8V[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D`@Qa\x05\xB6\x91\x90a\x11kV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPV[_`\x05\x80T\x90P\x90P\x90V[`\x05\x81\x81T\x81\x10a\x05\xE7W_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[`\x01T\x81V[a\x0E\x10\x81V[`\x0C\x81V[`\x06T\x81V[_`\x04\x80T\x90P\x90P_\x82\x80Q\x90` \x01 \x90P_\x85\x85CB\x86_\x87`@Q` \x01a\x06A\x97\x96\x95\x94\x93\x92\x91\x90a\x12XV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P___\x1B\x90P_\x84\x11\x15a\x06\x93W`\x04`\x01\x85a\x06v\x91\x90a\x0F7V[\x81T\x81\x10a\x06\x87Wa\x06\x86a\x0FjV[[\x90_R` _ \x01T\x90P[`\x04\x81\x83`@Q` \x01a\x06\xA8\x92\x91\x90a\x12\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x80\x84\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE10\x8A\x8A\x88_B`@Qa\x07\x1C\x96\x95\x94\x93\x92\x91\x90a\x13cV[`@Q\x80\x91\x03\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x07T\x91\x90a\x11kV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[a\x1C \x81V[b\x01Q\x80\x81V[`\x04\x81\x81T\x81\x10a\x07\x81W_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[______\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01T`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03T\x93P\x93P\x93P\x93P\x90\x91\x92\x93V[_`\x05\x82\x81T\x81\x10a\x07\xEAWa\x07\xE9a\x0FjV[[\x90_R` _ \x01T\x90P\x91\x90PV[a\x08'`\x0C\x84\x84\x84`@Q` \x01a\x08\x13\x92\x91\x90a\x13\xC2V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x06\x0FV[PPPV[b\x01\xCC\xCC\x81V[_`\x04\x80T\x90P\x90P\x90V[`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[_a\x08aa\n3V[_\x84Q`(a\x08p\x91\x90a\x13\xEDV[\x90Pb\x01\xCC\xCCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\x08\xC9W\x80b\x01\xCC\xCC`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xC0\x92\x91\x90a\x14PV[`@Q\x80\x91\x03\x90\xFD[a\x08\xD1a\n3V[b\x01Q\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15a\t\x14Wb\x01Q\x80Ba\x08\xF5\x91\x90a\x14wV[\x81_\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[a\x0E\x10Ba\t\"\x91\x90a\x14\xB2V[\x81` \x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x1C g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x11\x15a\t\x83Wa\x1C Ca\tc\x91\x90a\x14wV[\x81`@\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[`\x0CCa\t\x90\x91\x90a\x14\xB2V[\x81``\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x89`@Q` \x01a\t\xD8\x95\x94\x93\x92\x91\x90a\x14\xEDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\t\xFBWa\t\xFAa\x15KV[[\x80\x87`@Q` \x01a\n\x0E\x92\x91\x90a\x15\xB2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[`@Q\x80`\x80\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_\x81\x90P\x91\x90PV[a\n\x91\x81a\n\x7FV[\x82RPPV[_` \x82\x01\x90Pa\n\xAA_\x83\x01\x84a\n\x88V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\n\xCC\x81a\n\xB0V[\x82RPPV[_` \x82\x01\x90Pa\n\xE5_\x83\x01\x84a\n\xC3V[\x92\x91PPV[_`@Q\x90P\x90V[__\xFD[__\xFD[__\xFD[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0BJ\x82a\x0B\x04V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0BiWa\x0Bha\x0B\x14V[[\x80`@RPPPV[_a\x0B{a\n\xEBV[\x90Pa\x0B\x87\x82\x82a\x0BAV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\xA6Wa\x0B\xA5a\x0B\x14V[[a\x0B\xAF\x82a\x0B\x04V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0B\xDCa\x0B\xD7\x84a\x0B\x8CV[a\x0BrV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0B\xF8Wa\x0B\xF7a\x0B\0V[[a\x0C\x03\x84\x82\x85a\x0B\xBCV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0C\x1FWa\x0C\x1Ea\n\xFCV[[\x815a\x0C/\x84\x82` \x86\x01a\x0B\xCAV[\x91PP\x92\x91PPV[a\x0CA\x81a\n\xB0V[\x81\x14a\x0CKW__\xFD[PV[_\x815\x90Pa\x0C\\\x81a\x0C8V[\x92\x91PPV[a\x0Ck\x81a\n\x7FV[\x81\x14a\x0CuW__\xFD[PV[_\x815\x90Pa\x0C\x86\x81a\x0CbV[\x92\x91PPV[_____`\xA0\x86\x88\x03\x12\x15a\x0C\xA5Wa\x0C\xA4a\n\xF4V[[_\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xC2Wa\x0C\xC1a\n\xF8V[[a\x0C\xCE\x88\x82\x89\x01a\x0C\x0BV[\x95PP` a\x0C\xDF\x88\x82\x89\x01a\x0CNV[\x94PP`@a\x0C\xF0\x88\x82\x89\x01a\x0CxV[\x93PP``a\r\x01\x88\x82\x89\x01a\x0CNV[\x92PP`\x80a\r\x12\x88\x82\x89\x01a\x0CxV[\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a\r4Wa\r3a\n\xF4V[[_a\rA\x84\x82\x85\x01a\x0CxV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\r\\\x81a\rJV[\x82RPPV[_` \x82\x01\x90Pa\ru_\x83\x01\x84a\rSV[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\r\x90\x81a\r{V[\x81\x14a\r\x9AW__\xFD[PV[_\x815\x90Pa\r\xAB\x81a\r\x87V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\r\xDA\x82a\r\xB1V[\x90P\x91\x90PV[a\r\xEA\x81a\r\xD0V[\x81\x14a\r\xF4W__\xFD[PV[_\x815\x90Pa\x0E\x05\x81a\r\xE1V[\x92\x91PPV[___``\x84\x86\x03\x12\x15a\x0E\"Wa\x0E!a\n\xF4V[[_a\x0E/\x86\x82\x87\x01a\r\x9DV[\x93PP` a\x0E@\x86\x82\x87\x01a\r\xF7V[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EaWa\x0E`a\n\xF8V[[a\x0Em\x86\x82\x87\x01a\x0C\x0BV[\x91PP\x92P\x92P\x92V[_`\x80\x82\x01\x90Pa\x0E\x8A_\x83\x01\x87a\n\xC3V[a\x0E\x97` \x83\x01\x86a\n\x88V[a\x0E\xA4`@\x83\x01\x85a\n\xC3V[a\x0E\xB1``\x83\x01\x84a\n\x88V[\x95\x94PPPPPV[___``\x84\x86\x03\x12\x15a\x0E\xD1Wa\x0E\xD0a\n\xF4V[[_a\x0E\xDE\x86\x82\x87\x01a\r\xF7V[\x93PP` a\x0E\xEF\x86\x82\x87\x01a\r\xF7V[\x92PP`@a\x0F\0\x86\x82\x87\x01a\x0CxV[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x0FA\x82a\n\x7FV[\x91Pa\x0FL\x83a\n\x7FV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x0FdWa\x0Fca\x0F\nV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[a\x0F\xB1a\x0F\xAC\x82a\rJV[a\x0F\x97V[\x82RPPV[_a\x0F\xC2\x82\x86a\x0F\xA0V[` \x82\x01\x91Pa\x0F\xD2\x82\x85a\x0F\xA0V[` \x82\x01\x91Pa\x0F\xE2\x82\x84a\x0F\xA0V[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[a\x0F\xFC\x81a\n\xB0V[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\x10\x16_\x85\x01\x82a\x0F\xF3V[P` \x82\x01Qa\x10)` \x85\x01\x82a\x0F\xF3V[P`@\x82\x01Qa\x10<`@\x85\x01\x82a\x0F\xF3V[P``\x82\x01Qa\x10O``\x85\x01\x82a\x0F\xF3V[PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x10\x93Wa\x10\x92a\x10UV[[PV[_\x81\x90Pa\x10\xA3\x82a\x10\x82V[\x91\x90PV[_a\x10\xB2\x82a\x10\x96V[\x90P\x91\x90PV[a\x10\xC2\x81a\x10\xA8V[\x82RPPV[_`\xE0\x82\x01\x90Pa\x10\xDB_\x83\x01\x87a\rSV[a\x10\xE8` \x83\x01\x86a\n\x88V[a\x10\xF5`@\x83\x01\x85a\x10\x02V[a\x11\x02`\xC0\x83\x01\x84a\x10\xB9V[\x95\x94PPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x11=\x82a\x11\x0BV[a\x11G\x81\x85a\x11\x15V[\x93Pa\x11W\x81\x85` \x86\x01a\x11%V[a\x11`\x81a\x0B\x04V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x11\x83\x81\x84a\x113V[\x90P\x92\x91PPV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\x11\xA1\x82a\x11\x8BV[\x90P\x91\x90PV[a\x11\xB9a\x11\xB4\x82a\r{V[a\x11\x97V[\x82RPPV[_\x81``\x1B\x90P\x91\x90PV[_a\x11\xD5\x82a\x11\xBFV[\x90P\x91\x90PV[_a\x11\xE6\x82a\x11\xCBV[\x90P\x91\x90PV[a\x11\xFEa\x11\xF9\x82a\r\xD0V[a\x11\xDCV[\x82RPPV[_\x81`\xC0\x1B\x90P\x91\x90PV[_a\x12\x1A\x82a\x12\x04V[\x90P\x91\x90PV[a\x122a\x12-\x82a\n\xB0V[a\x12\x10V[\x82RPPV[_\x81\x90P\x91\x90PV[a\x12Ra\x12M\x82a\n\x7FV[a\x128V[\x82RPPV[_a\x12c\x82\x8Aa\x11\xA8V[`\x01\x82\x01\x91Pa\x12s\x82\x89a\x11\xEDV[`\x14\x82\x01\x91Pa\x12\x83\x82\x88a\x12!V[`\x08\x82\x01\x91Pa\x12\x93\x82\x87a\x12!V[`\x08\x82\x01\x91Pa\x12\xA3\x82\x86a\x12AV[` \x82\x01\x91Pa\x12\xB3\x82\x85a\x12AV[` \x82\x01\x91Pa\x12\xC3\x82\x84a\x0F\xA0V[` \x82\x01\x91P\x81\x90P\x98\x97PPPPPPPPV[_a\x12\xE3\x82\x85a\x0F\xA0V[` \x82\x01\x91Pa\x12\xF3\x82\x84a\x0F\xA0V[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[a\x13\x0C\x81a\r\xD0V[\x82RPPV[a\x13\x1B\x81a\r{V[\x82RPPV[_\x81\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x13Ma\x13Ha\x13C\x84a\x13!V[a\x13*V[a\n\x7FV[\x90P\x91\x90PV[a\x13]\x81a\x133V[\x82RPPV[_`\xC0\x82\x01\x90Pa\x13v_\x83\x01\x89a\x13\x03V[a\x13\x83` \x83\x01\x88a\x13\x12V[a\x13\x90`@\x83\x01\x87a\x13\x03V[a\x13\x9D``\x83\x01\x86a\rSV[a\x13\xAA`\x80\x83\x01\x85a\x13TV[a\x13\xB7`\xA0\x83\x01\x84a\n\xC3V[\x97\x96PPPPPPPV[_a\x13\xCD\x82\x85a\x11\xEDV[`\x14\x82\x01\x91Pa\x13\xDD\x82\x84a\x12AV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[_a\x13\xF7\x82a\n\x7FV[\x91Pa\x14\x02\x83a\n\x7FV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x14\x1AWa\x14\x19a\x0F\nV[[\x92\x91PPV[_a\x14:a\x145a\x140\x84a\n\xB0V[a\x13*V[a\n\x7FV[\x90P\x91\x90PV[a\x14J\x81a\x14 V[\x82RPPV[_`@\x82\x01\x90Pa\x14c_\x83\x01\x85a\n\x88V[a\x14p` \x83\x01\x84a\x14AV[\x93\x92PPPV[_a\x14\x81\x82a\n\xB0V[\x91Pa\x14\x8C\x83a\n\xB0V[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xACWa\x14\xABa\x0F\nV[[\x92\x91PPV[_a\x14\xBC\x82a\n\xB0V[\x91Pa\x14\xC7\x83a\n\xB0V[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xE7Wa\x14\xE6a\x0F\nV[[\x92\x91PPV[_a\x14\xF8\x82\x88a\x12!V[`\x08\x82\x01\x91Pa\x15\x08\x82\x87a\x12!V[`\x08\x82\x01\x91Pa\x15\x18\x82\x86a\x12!V[`\x08\x82\x01\x91Pa\x15(\x82\x85a\x12!V[`\x08\x82\x01\x91Pa\x158\x82\x84a\x12!V[`\x08\x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[_\x81\x90P\x92\x91PPV[_a\x15\x8C\x82a\x11\x0BV[a\x15\x96\x81\x85a\x15xV[\x93Pa\x15\xA6\x81\x85` \x86\x01a\x11%V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x15\xBD\x82\x85a\x15\x82V[\x91Pa\x15\xC9\x82\x84a\x15\x82V[\x91P\x81\x90P\x93\x92PPPV",
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
