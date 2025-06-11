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
    ///0x6080604052346100305761001a610014610197565b90610485565b610022610035565b611b1f6114478239611b1f90f35b61003b565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906100679061003f565b810190811060018060401b0382111761007f57604052565b610049565b90610097610090610035565b928361005d565b565b5f80fd5b5f80fd5b90565b6100ad816100a1565b036100b457565b5f80fd5b905051906100c5826100a4565b565b5f80fd5b5f80fd5b60018060401b0381116100eb576100e760209161003f565b0190565b610049565b90825f9392825e0152565b9092919261011061010b826100cf565b610084565b9381855260208501908284011161012c5761012a926100f0565b565b6100cb565b9080601f8301121561014f5781602061014c935191016100fb565b90565b6100c7565b9190916040818403126101925761016d835f83016100b8565b92602082015160018060401b03811161018d5761018a9201610131565b90565b61009d565b610099565b6101b5612f66803803806101aa81610084565b928339810190610154565b9091565b5f1b90565b906101cf60018060401b03916101b9565b9181191691161790565b90565b60018060401b031690565b90565b6101fe6101f9610203926101d9565b6101e7565b6101dc565b90565b90565b9061021e610219610225926101ea565b610206565b82546101be565b9055565b906102355f19916101b9565b9181191691161790565b61025361024e610258926101d9565b6101e7565b6100a1565b90565b90565b9061027361026e61027a9261023f565b61025b565b8254610229565b9055565b90565b5190565b60209181520190565b5f7f454d5054595f434841494e5f434f4e4649470000000000000000000000000000910152565b6102c26012602092610285565b6102cb8161028e565b0190565b6102e49060208101905f8183039101526102b5565b90565b156102ee57565b6102f6610035565b62461bcd60e51b81528061030c600482016102cf565b0390fd5b90565b60ff1690565b61032d61032861033292610310565b6101e7565b610313565b90565b61033f600b610319565b90565b60018060a01b031690565b61036161035c610366926101d9565b6101e7565b610342565b90565b6103729061034d565b90565b90565b61038c61038761039192610375565b6101e7565b610313565b90565b90565b6103a36103a8916100a1565b610394565b9052565b60f81b90565b6103bb906103ac565b90565b6103ca6103cf91610313565b6103b2565b9052565b5190565b905090565b6104016103f8926020926103ef816103d3565b948580936103d7565b938491016100f0565b0190565b60016020936104298584610421610431966104389b9a98610397565b0180926103be565b018092610397565b01906103dc565b90565b9061044d610448836100cf565b610084565b918252565b5f640b0080020360d01b910152565b61046b600661043b565b9061047860208301610452565b565b610482610461565b90565b90610535916104945f5f610209565b61049f5f600161025e565b6104aa5f6002610209565b6104b55f600361025e565b6104e16104c96104c48461027e565b610281565b6104db6104d55f61023f565b916100a1565b116102e7565b6104e9610335565b6105306104f55f610369565b926105216105036001610378565b9561050d5f61023f565b610515610035565b97889460208601610405565b6020820181038252038461005d565b610991565b61056f5f5f5f5f9161056961056361055d61055761055161047a565b976101ea565b9361023f565b936101ea565b9361023f565b93610cf7565b565b5490565b60200190565b61058f61058a610594926100a1565b6101e7565b6101dc565b90565b6105a090610342565b90565b60601b90565b6105b2906105a3565b90565b6105be906105a9565b90565b6105cd6105d291610597565b6105b5565b9052565b60c01b90565b6105e5906105d6565b90565b6105f46105f9916101dc565b6105dc565b9052565b90565b90565b61060f610614916105fd565b610600565b9052565b946106696008602099989596610661828c9961065960146106719a6106516106799f8f9c90610649816001936103be565b0180926105c1565b0180926105e8565b0180926105e8565b018092610397565b018092610397565b018092610603565b0190565b61069161068c610696926101d9565b6101b9565b6105fd565b90565b6106ad6106a86106b292610375565b6101e7565b6100a1565b90565b634e487b7160e01b5f52601160045260245ffd5b6106d86106de919392936100a1565b926100a1565b82039182116106e957565b6106b5565b634e487b7160e01b5f52603260045260245ffd5b5f5260205f2090565b61071481610571565b82101561072e57610726600191610702565b910201905f90565b6106ee565b1c90565b90565b61074a90600861074f9302610733565b610737565b90565b9061075d915461073a565b90565b90565b60208161077561077d93839695610603565b018092610603565b0190565b5f5260205f2090565b5490565b6107978161078a565b8210156107b1576107a9600191610781565b910201905f90565b6106ee565b1b90565b919060086107d59102916107cf5f19846107b6565b926107b6565b9181191691161790565b6107e8906105fd565b90565b5f1c90565b6107f9906107eb565b90565b919061081261080d61081a936107df565b6107f0565b9083546107ba565b9055565b908154916801000000000000000083101561084e578261084691600161084c9501815561078e565b906107fc565b565b610049565b61086761086261086c92610342565b6101e7565b610342565b90565b61087890610853565b90565b6108849061086f565b90565b61089b6108966108a0926100a1565b6101e7565b6100a1565b90565b6108ac90610597565b9052565b6108b990610313565b9052565b6108c6906105fd565b9052565b6108d39061023f565b9052565b6108e0906101dc565b9052565b919461092c6109369298979561092260a09661091861093d9a61090e60c08a019e5f8b01906108a3565b60208901906108b0565b60408701906108a3565b60608501906108bd565b60808301906108ca565b01906108d7565b565b60209181520190565b6109676109706020936109759361095e81610281565b9384809361093f565b958691016100f0565b61003f565b0190565b61098e9160208201915f818403910152610948565b90565b9061099c6004610571565b91836109b06109aa82610281565b91610575565b2091816109fd82916109ee6109c44361057b565b6109cd4261057b565b896109d75f61023f565b918a936109e2610035565b98899760208901610618565b6020820181038252038261005d565b610a0f610a0982610281565b91610575565b2090610a1a5f61067d565b9185610a2e610a285f61023f565b916100a1565b11610b2f575b610a8690610a426004610760565b90610a6d85610a5e610a52610035565b93849260208401610763565b6020820181038252038261005d565b610a7f610a7982610281565b91610575565b209061081e565b849192610ae8610a953061087b565b9192955f610aa24261057b565b91610ad6610ad07f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe198610887565b986107df565b98610adf610035565b968796876108e4565b0390a3610b2a610b187fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b92610887565b92610b21610035565b91829182610979565b0390a2565b9150610a86610b5c610b566004610b5089610b4a6001610699565b906106c9565b9061070b565b90610752565b929050610a34565b610b78610b73610b7d926101dc565b6101e7565b6101dc565b90565b90610b95610b90610b9c92610b64565b610206565b82546101be565b9055565b90610bb5610bb0610bbc92610887565b61025b565b8254610229565b9055565b60209392610bdf8583610bd78295610be797610603565b018092610603565b018092610603565b0190565b90565b610bfa610bff916107eb565b610beb565b90565b610c0c9054610bee565b90565b610c18906100a1565b9052565b610c25906101dc565b9052565b90606080610c6f93610c415f8201515f860190610c1c565b610c5360208201516020860190610c1c565b610c6560408201516040860190610c1c565b0151910190610c1c565b565b634e487b7160e01b5f52602160045260245ffd5b60051115610c8f57565b610c71565b90610c9e82610c85565b565b610ca990610c94565b90565b610cb590610ca0565b9052565b610cee610cf594610ce460c094989795610cda60e086019a5f8701906108bd565b6020850190610c0f565b6040830190610c29565b0190610cac565b565b9391610d06610d0d925f610b80565b6001610ba0565b81610d20610d1a5f6101ea565b916101dc565b11610efa575b5050610d326004610571565b610d3d828290611237565b90610d486005610571565b92610d525f61067d565b84610d65610d5f5f61023f565b916100a1565b11610ecc575b610d745f61067d565b9282610d88610d825f61023f565b916100a1565b11610e97575b610dec90610dbd83610dae87610da2610035565b94859360208501610bc0565b6020820181038252038261005d565b610dcf610dc982610281565b91610575565b2092610de5610dde6005610760565b859061081e565b6006610ba0565b8490919293610dfb6006610c02565b9094610e506001610e3e610e38610e327f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd797610887565b976107df565b976107df565b97610e47610035565b94859485610cb9565b0390a4610e92610e807ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c2092610887565b92610e89610035565b91829182610979565b0390a2565b9250610dec610ec4610ebe6004610eb886610eb26001610699565b906106c9565b9061070b565b90610752565b939050610d8e565b50610ef5610eef6005610ee987610ee36001610699565b906106c9565b9061070b565b90610752565b610d6b565b610f08610f0f926002610b80565b6003610ba0565b5f80610d26565b5f90565b610f246080610084565b90565b5f90565b610f33610f1a565b90602080808085610f42610f27565b815201610f4d610f27565b815201610f58610f27565b815201610f63610f27565b81525050565b610f71610f2b565b90565b90565b610f8b610f86610f9092610f74565b6101e7565b6100a1565b90565b610f9d6028610f77565b90565b610faf610fb5919392936100a1565b926100a1565b8201809211610fc057565b6106b5565b90565b610fdc610fd7610fe192610fc5565b6101e7565b6101dc565b90565b610ff06201cccc610fc8565b90565b61100761100261100c926101dc565b6101e7565b6100a1565b90565b61101890610ff3565b9052565b91602061103d92949361103660408201965f830190610c0f565b019061100f565b565b90565b61105661105161105b9261103f565b6101e7565b6101dc565b90565b61106a62015180611042565b90565b61107961107f916101dc565b916101dc565b90039060018060401b03821161109157565b6106b5565b906110a0906101dc565b9052565b90565b6110bb6110b66110c0926110a4565b6101e7565b6101dc565b90565b6110ce610e106110a7565b90565b6110dd6110e3916101dc565b916101dc565b019060018060401b0382116110f457565b6106b5565b90565b61111061110b611115926110f9565b6101e7565b6101dc565b90565b611123611c206110fc565b90565b90565b61113d61113861114292611126565b6101e7565b6101dc565b90565b61114f600c611129565b90565b61115c90516101dc565b90565b60086111a194611191828099989596611189828761118161119999839c6105e8565b0180926105e8565b0180926105e8565b0180926105e8565b0180926105e8565b0190565b634e487b7160e01b5f52600160045260245ffd5b156111c057565b6111a5565b905090565b6111ef6111e6926020926111dd81610281565b948580936111c5565b938491016100f0565b0190565b6112019061120793926111ca565b906111ca565b90565b61122992916112359161121b610035565b9485926020840192836111f3565b9081038252038361005d565b565b919091611242610f16565b5061124b610f69565b50611266611257610f93565b61126083610281565b90610fa0565b8061128061127a611275610fe4565b610ff3565b916100a1565b1161141f57506113b390611292610f69565b93426112ad6112a76112a261105e565b610ff3565b916100a1565b116113f5575b6112d96112d06112c24261057b565b6112ca6110c3565b906110d1565b60208701611096565b436112f36112ed6112e8611118565b610ff3565b916100a1565b116113ca575b61131f6113166113084361057b565b611310611145565b906110d1565b60608701611096565b61138361132d5f8701611152565b61137461133c60208901611152565b9361134960408a01611152565b9061135f61135960608c01611152565b9161057b565b91611368610035565b9687956020870161115f565b6020820181038252038261005d565b6113ae61138f82610281565b6113a86113a261139d610f93565b6100a1565b916100a1565b146111b9565b61120a565b6113c56113bf82610281565b91610575565b209190565b6113f06113e76113d94361057b565b6113e1611118565b9061106d565b60408701611096565b6112f9565b61141a6114126114044261057b565b61140c61105e565b9061106d565b5f8701611096565b6112b3565b611427610fe4565b906114425f928392634634691b60e01b84526004840161101c565b0390fdfe60806040526004361015610013575b610ba1565b61001d5f3561015b565b806284120c1461015657806304f1c85414610151578063056daaa61461014c578063061d12c01461014757806306f130561461014257806316bf55791461013d57806318db3940146101385780632f1ec5e9146101335780634f359a371461012e5780637fa3a40e14610129578063a7b51d1914610124578063ad9c0c2e1461011f578063b752a7d11461011a578063d5719dc214610115578063d5954c3414610110578063d9dd67ab1461010b578063e1d66afe14610106578063e8eb1dc314610101578063eca067ad146100fc5763fbf6eaa50361000e57610b6c565b610b28565b610af3565b610a86565b610a17565b6109de565b61096b565b61090a565b61089c565b610830565b610747565b610703565b610697565b610627565b6105e3565b6104cd565b610496565b6102b9565b610214565b6101a3565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f91031261017957565b61016b565b90565b61018a9061017e565b9052565b91906101a1905f60208501940190610181565b565b346101d3576101b336600461016f565b6101cf6101be610ba9565b6101c6610161565b9182918261018e565b0390f35b610167565b1c90565b90565b6101ef9060086101f493026101d8565b6101dc565b90565b9061020291546101df565b90565b61021160035f906101f7565b90565b346102445761022436600461016f565b61024061022f610205565b610237610161565b9182918261018e565b0390f35b610167565b67ffffffffffffffff1690565b61026690600861026b93026101d8565b610249565b90565b906102799154610256565b90565b6102875f5f9061026e565b90565b67ffffffffffffffff1690565b6102a09061028a565b9052565b91906102b7905f60208501940190610297565b565b346102e9576102c936600461016f565b6102e56102d461027c565b6102dc610161565b918291826102a4565b0390f35b610167565b5f80fd5b5f80fd5b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b90610322906102fa565b810190811067ffffffffffffffff82111761033c57604052565b610304565b9061035461034d610161565b9283610318565b565b67ffffffffffffffff8111610374576103706020916102fa565b0190565b610304565b90825f939282370152565b9092919261039961039482610356565b610341565b938185526020850190828401116103b5576103b392610379565b565b6102f6565b9080601f830112156103d8578160206103d593359101610384565b90565b6102f2565b6103e68161028a565b036103ed57565b5f80fd5b905035906103fe826103dd565b565b6104098161017e565b0361041057565b5f80fd5b9050359061042182610400565b565b919060a08382031261048c575f83013567ffffffffffffffff8111610487578161044e9185016103ba565b9261045c82602083016103f1565b9261048461046d8460408501610414565b9361047b81606086016103f1565b93608001610414565b90565b6102ee565b61016b565b5f0190565b346104c8576104b26104a9366004610423565b93929092610f9f565b6104ba610161565b806104c481610491565b0390f35b610167565b346104fd576104dd36600461016f565b6104f96104e86111be565b6104f0610161565b9182918261018e565b0390f35b610167565b9060208282031261051b57610518915f01610414565b90565b61016b565b634e487b7160e01b5f52603260045260245ffd5b5490565b5f5260205f2090565b61054a81610534565b8210156105645761055c600191610538565b910201905f90565b610520565b90565b61057c90600861058193026101d8565b610569565b90565b9061058f915461056c565b90565b600561059d81610534565b8210156105ba576105b7916105b191610541565b90610584565b90565b5f80fd5b90565b6105ca906105be565b9052565b91906105e1905f602085019401906105c1565b565b346106135761060f6105fe6105f9366004610502565b610592565b610606610161565b918291826105ce565b0390f35b610167565b61062460015f906101f7565b90565b346106575761063736600461016f565b610653610642610618565b61064a610161565b9182918261018e565b0390f35b610167565b90565b90565b61067661067161067b9261065c565b61065f565b61028a565b90565b610689610e10610662565b90565b61069461067e565b90565b346106c7576106a736600461016f565b6106c36106b261068c565b6106ba610161565b918291826102a4565b0390f35b610167565b90565b6106e36106de6106e8926106cc565b61065f565b61028a565b90565b6106f5600c6106cf565b90565b6107006106eb565b90565b346107335761071336600461016f565b61072f61071e6106f8565b610726610161565b918291826102a4565b0390f35b610167565b61074460065f906101f7565b90565b346107775761075736600461016f565b610773610762610738565b61076a610161565b9182918261018e565b0390f35b610167565b60ff1690565b61078b8161077c565b0361079257565b5f80fd5b905035906107a382610782565b565b60018060a01b031690565b6107b9906107a5565b90565b6107c5816107b0565b036107cc57565b5f80fd5b905035906107dd826107bc565b565b9160608383031261082b576107f6825f8501610796565b9261080483602083016107d0565b92604082013567ffffffffffffffff81116108265761082392016103ba565b90565b6102ee565b61016b565b3461085f576108496108433660046107df565b916113c2565b610851610161565b8061085b81610491565b0390f35b610167565b90565b61087b61087661088092610864565b61065f565b61028a565b90565b61088e611c20610867565b90565b610899610883565b90565b346108cc576108ac36600461016f565b6108c86108b7610891565b6108bf610161565b918291826102a4565b0390f35b610167565b90565b6108e86108e36108ed926108d1565b61065f565b61028a565b90565b6108fc620151806108d4565b90565b6109076108f0565b90565b3461093a5761091a36600461016f565b6109366109256108ff565b61092d610161565b918291826102a4565b0390f35b610167565b600461094a81610534565b821015610967576109649161095e91610541565b90610584565b90565b5f80fd5b3461099b57610997610986610981366004610502565b61093f565b61098e610161565b918291826105ce565b0390f35b610167565b6109d56109dc946109cb6060949897956109c1608086019a5f870190610297565b6020850190610181565b6040830190610297565b0190610181565b565b34610a12576109ee36600461016f565b610a0e6109f96115ba565b90610a05949294610161565b948594856109a0565b0390f35b610167565b34610a4757610a43610a32610a2d366004610502565b611613565b610a3a610161565b918291826105ce565b0390f35b610167565b9091606082840312610a8157610a7e610a67845f85016107d0565b93610a7581602086016107d0565b93604001610414565b90565b61016b565b34610ab557610a9f610a99366004610a4c565b9161167b565b610aa7610161565b80610ab181610491565b0390f35b610167565b90565b610ad1610acc610ad692610aba565b61065f565b61028a565b90565b610ae56201cccc610abd565b90565b610af0610ad9565b90565b34610b2357610b0336600461016f565b610b1f610b0e610ae8565b610b16610161565b918291826102a4565b0390f35b610167565b34610b5857610b3836600461016f565b610b54610b436116bb565b610b4b610161565b9182918261018e565b0390f35b610167565b610b6960025f9061026e565b90565b34610b9c57610b7c36600461016f565b610b98610b87610b5d565b610b8f610161565b918291826102a4565b0390f35b610167565b5f80fd5b5f90565b610bb1610ba5565b50610bbc6005610534565b90565b5f1b90565b90610bd767ffffffffffffffff91610bbf565b9181191691161790565b610bf5610bf0610bfa9261028a565b61065f565b61028a565b90565b90565b90610c15610c10610c1c92610be1565b610bfd565b8254610bc4565b9055565b90610c2c5f1991610bbf565b9181191691161790565b610c4a610c45610c4f9261017e565b61065f565b61017e565b90565b90565b90610c6a610c65610c7192610c36565b610c52565b8254610c20565b9055565b90565b610c8c610c87610c9192610c75565b61065f565b61028a565b90565b610ca8610ca3610cad92610c75565b610bbf565b6105be565b90565b610cc4610cbf610cc992610c75565b61065f565b61017e565b90565b90565b610ce3610cde610ce892610ccc565b61065f565b61017e565b90565b634e487b7160e01b5f52601160045260245ffd5b610d0e610d149193929361017e565b9261017e565b8203918211610d1f57565b610ceb565b90565b610d33610d38916105be565b610d24565b9052565b60209392610d5b8583610d538295610d6397610d27565b018092610d27565b018092610d27565b0190565b60200190565b5190565b90565b5f5260205f2090565b5490565b610d8a81610d7d565b821015610da457610d9c600191610d74565b910201905f90565b610520565b1b90565b91906008610dc8910291610dc25f1984610da9565b92610da9565b9181191691161790565b610ddb906105be565b90565b5f1c90565b610dec90610dde565b90565b9190610e05610e00610e0d93610dd2565b610de3565b908354610dad565b9055565b9081549168010000000000000000831015610e415782610e39916001610e3f95018155610d81565b90610def565b565b610304565b610e52610e5791610dde565b6101dc565b90565b610e649054610e46565b90565b610e709061028a565b9052565b90606080610eba93610e8c5f8201515f860190610e67565b610e9e60208201516020860190610e67565b610eb060408201516040860190610e67565b0151910190610e67565b565b634e487b7160e01b5f52602160045260245ffd5b60051115610eda57565b610ebc565b90610ee982610ed0565b565b610ef490610edf565b90565b610f0090610eeb565b9052565b610f39610f4094610f2f60c094989795610f2560e086019a5f8701906105c1565b6020850190610181565b6040830190610e74565b0190610ef7565b565b60209181520190565b90825f9392825e0152565b610f75610f7e602093610f8393610f6c81610d6d565b93848093610f42565b95869101610f4b565b6102fa565b0190565b610f9c9160208201915f818403910152610f56565b90565b9391610fae610fb5925f610c00565b6001610c55565b81610fc8610fc25f610c78565b9161028a565b116111a2575b5050610fda6004610534565b610fe5828290611910565b90610ff06005610534565b92610ffa5f610c94565b8461100d6110075f610cb0565b9161017e565b11611174575b61101c5f610c94565b928261103061102a5f610cb0565b9161017e565b1161113f575b61109490611065836110568761104a610161565b94859360208501610d3c565b60208201810382520382610318565b61107761107182610d6d565b91610d67565b209261108d6110866005610d71565b8590610e11565b6006610c55565b84909192936110a36006610e5a565b90946110f860016110e66110e06110da7f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd797610c36565b97610dd2565b97610dd2565b976110ef610161565b94859485610f04565b0390a461113a6111287ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c2092610c36565b92611131610161565b91829182610f87565b0390a2565b925061109461116c61116660046111608661115a6001610ccf565b90610cff565b90610541565b90610584565b939050611036565b5061119d61119760056111918761118b6001610ccf565b90610cff565b90610541565b90610584565b611013565b6111b06111b7926002610c00565b6003610c55565b5f80610fce565b6111c6610ba5565b506111d16005610534565b90565b6111e86111e36111ed9261017e565b61065f565b61028a565b90565b60f81b90565b6111ff906111f0565b90565b61120e6112139161077c565b6111f6565b9052565b60601b90565b61122690611217565b90565b6112329061121d565b90565b611241611246916107b0565b611229565b9052565b60c01b90565b6112599061124a565b90565b61126861126d9161028a565b611250565b9052565b90565b6112806112859161017e565b611271565b9052565b946112da60086020999895966112d2828c996112ca60146112e29a6112c26112ea9f8f9c906112ba81600193611202565b018092611235565b01809261125c565b01809261125c565b018092611274565b018092611274565b018092610d27565b0190565b60208161130061130893839695610d27565b018092610d27565b0190565b61132061131b611325926107a5565b61065f565b6107a5565b90565b6113319061130c565b90565b61133d90611328565b90565b611349906107b0565b9052565b6113569061077c565b9052565b61136390610cb0565b9052565b91946113af6113b9929897956113a560a09661139b6113c09a61139160c08a019e5f8b0190611340565b602089019061134d565b6040870190611340565b60608501906105c1565b608083019061135a565b0190610297565b565b906113cd6004610534565b91836113e16113db82610d6d565b91610d67565b20918161142e829161141f6113f5436111d4565b6113fe426111d4565b896114085f610cb0565b918a93611413610161565b98899760208901611289565b60208201810382520382610318565b61144061143a82610d6d565b91610d67565b209061144b5f610c94565b918561145f6114595f610cb0565b9161017e565b11611560575b6114b7906114736004610d71565b9061149e8561148f611483610161565b938492602084016112ee565b60208201810382520382610318565b6114b06114aa82610d6d565b91610d67565b2090610e11565b8491926115196114c630611334565b9192955f6114d3426111d4565b916115076115017f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe198610c36565b98610dd2565b98611510610161565b96879687611367565b0390a361155b6115497fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b92610c36565b92611552610161565b91829182610f87565b0390a2565b91506114b761158d61158760046115818961157b6001610ccf565b90610cff565b90610541565b90610584565b929050611465565b5f90565b6115a56115aa91610dde565b610249565b90565b6115b79054611599565b90565b6115c2611595565b506115cb610ba5565b506115d4611595565b506115dd610ba5565b506115e75f6115ad565b906115f26001610e5a565b916115fd60026115ad565b916116086003610e5a565b9193929190565b5f90565b61162a6116309161162261160f565b506005610541565b90610584565b90565b61164761164261164c926106cc565b61065f565b61077c565b90565b611659600c611633565b90565b60148161166f6116779360209695611235565b018092611274565b0190565b906116b992916116b461168c61164f565b91926116a5611699610161565b9586926020840161165c565b60208201810382520384610318565b6113c2565b565b6116c3610ba5565b506116ce6004610534565b90565b6116db6080610341565b90565b5f90565b6116ea6116d1565b906020808080856116f96116de565b8152016117046116de565b81520161170f6116de565b81520161171a6116de565b81525050565b6117286116e2565b90565b90565b61174261173d6117479261172b565b61065f565b61017e565b90565b611754602861172e565b90565b61176661176c9193929361017e565b9261017e565b820180921161177757565b610ceb565b61179061178b6117959261028a565b61065f565b61017e565b90565b6117a19061177c565b9052565b9160206117c69294936117bf60408201965f830190610181565b0190611798565b565b6117d46117da9161028a565b9161028a565b90039067ffffffffffffffff82116117ee57565b610ceb565b906117fd9061028a565b9052565b61180d6118139161028a565b9161028a565b019067ffffffffffffffff821161182657565b610ceb565b611835905161028a565b90565b600861187a9461186a828099989596611862828761185a61187299839c61125c565b01809261125c565b01809261125c565b01809261125c565b01809261125c565b0190565b634e487b7160e01b5f52600160045260245ffd5b1561189957565b61187e565b905090565b6118c86118bf926020926118b681610d6d565b9485809361189e565b93849101610f4b565b0190565b6118da906118e093926118a3565b906118a3565b90565b611902929161190e916118f4610161565b9485926020840192836118cc565b90810382520383610318565b565b91909161191b61160f565b50611924611720565b5061193f61193061174a565b61193983610d6d565b90611757565b8061195961195361194e610ad9565b61177c565b9161017e565b11611af85750611a8c9061196b611720565b934261198661198061197b6108f0565b61177c565b9161017e565b11611ace575b6119b26119a961199b426111d4565b6119a361067e565b90611801565b602087016117f3565b436119cc6119c66119c1610883565b61177c565b9161017e565b11611aa3575b6119f86119ef6119e1436111d4565b6119e96106eb565b90611801565b606087016117f3565b611a5c611a065f870161182b565b611a4d611a156020890161182b565b93611a2260408a0161182b565b90611a38611a3260608c0161182b565b916111d4565b91611a41610161565b96879560208701611838565b60208201810382520382610318565b611a87611a6882610d6d565b611a81611a7b611a7661174a565b61017e565b9161017e565b14611892565b6118e3565b611a9e611a9882610d6d565b91610d67565b209190565b611ac9611ac0611ab2436111d4565b611aba610883565b906117c8565b604087016117f3565b6119d2565b611af3611aeb611add426111d4565b611ae56108f0565b906117c8565b5f87016117f3565b61198c565b611b00610ad9565b90611b1b5f928392634634691b60e01b8452600484016117a5565b0390fd
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4a\x000Wa\0\x1Aa\0\x14a\x01\x97V[\x90a\x04\x85V[a\0\"a\x005V[a\x1B\x1Fa\x14G\x829a\x1B\x1F\x90\xF3[a\0;V[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0g\x90a\0?V[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x7FW`@RV[a\0IV[\x90a\0\x97a\0\x90a\x005V[\x92\x83a\0]V[V[_\x80\xFD[_\x80\xFD[\x90V[a\0\xAD\x81a\0\xA1V[\x03a\0\xB4WV[_\x80\xFD[\x90PQ\x90a\0\xC5\x82a\0\xA4V[V[_\x80\xFD[_\x80\xFD[`\x01\x80`@\x1B\x03\x81\x11a\0\xEBWa\0\xE7` \x91a\0?V[\x01\x90V[a\0IV[\x90\x82_\x93\x92\x82^\x01RV[\x90\x92\x91\x92a\x01\x10a\x01\x0B\x82a\0\xCFV[a\0\x84V[\x93\x81\x85R` \x85\x01\x90\x82\x84\x01\x11a\x01,Wa\x01*\x92a\0\xF0V[V[a\0\xCBV[\x90\x80`\x1F\x83\x01\x12\x15a\x01OW\x81` a\x01L\x93Q\x91\x01a\0\xFBV[\x90V[a\0\xC7V[\x91\x90\x91`@\x81\x84\x03\x12a\x01\x92Wa\x01m\x83_\x83\x01a\0\xB8V[\x92` \x82\x01Q`\x01\x80`@\x1B\x03\x81\x11a\x01\x8DWa\x01\x8A\x92\x01a\x011V[\x90V[a\0\x9DV[a\0\x99V[a\x01\xB5a/f\x808\x03\x80a\x01\xAA\x81a\0\x84V[\x92\x839\x81\x01\x90a\x01TV[\x90\x91V[_\x1B\x90V[\x90a\x01\xCF`\x01\x80`@\x1B\x03\x91a\x01\xB9V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[`\x01\x80`@\x1B\x03\x16\x90V[\x90V[a\x01\xFEa\x01\xF9a\x02\x03\x92a\x01\xD9V[a\x01\xE7V[a\x01\xDCV[\x90V[\x90V[\x90a\x02\x1Ea\x02\x19a\x02%\x92a\x01\xEAV[a\x02\x06V[\x82Ta\x01\xBEV[\x90UV[\x90a\x025_\x19\x91a\x01\xB9V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x02Sa\x02Na\x02X\x92a\x01\xD9V[a\x01\xE7V[a\0\xA1V[\x90V[\x90V[\x90a\x02sa\x02na\x02z\x92a\x02?V[a\x02[V[\x82Ta\x02)V[\x90UV[\x90V[Q\x90V[` \x91\x81R\x01\x90V[_\x7FEMPTY_CHAIN_CONFIG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x02\xC2`\x12` \x92a\x02\x85V[a\x02\xCB\x81a\x02\x8EV[\x01\x90V[a\x02\xE4\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x02\xB5V[\x90V[\x15a\x02\xEEWV[a\x02\xF6a\x005V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x03\x0C`\x04\x82\x01a\x02\xCFV[\x03\x90\xFD[\x90V[`\xFF\x16\x90V[a\x03-a\x03(a\x032\x92a\x03\x10V[a\x01\xE7V[a\x03\x13V[\x90V[a\x03?`\x0Ba\x03\x19V[\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x03aa\x03\\a\x03f\x92a\x01\xD9V[a\x01\xE7V[a\x03BV[\x90V[a\x03r\x90a\x03MV[\x90V[\x90V[a\x03\x8Ca\x03\x87a\x03\x91\x92a\x03uV[a\x01\xE7V[a\x03\x13V[\x90V[\x90V[a\x03\xA3a\x03\xA8\x91a\0\xA1V[a\x03\x94V[\x90RV[`\xF8\x1B\x90V[a\x03\xBB\x90a\x03\xACV[\x90V[a\x03\xCAa\x03\xCF\x91a\x03\x13V[a\x03\xB2V[\x90RV[Q\x90V[\x90P\x90V[a\x04\x01a\x03\xF8\x92` \x92a\x03\xEF\x81a\x03\xD3V[\x94\x85\x80\x93a\x03\xD7V[\x93\x84\x91\x01a\0\xF0V[\x01\x90V[`\x01` \x93a\x04)\x85\x84a\x04!a\x041\x96a\x048\x9B\x9A\x98a\x03\x97V[\x01\x80\x92a\x03\xBEV[\x01\x80\x92a\x03\x97V[\x01\x90a\x03\xDCV[\x90V[\x90a\x04Ma\x04H\x83a\0\xCFV[a\0\x84V[\x91\x82RV[_d\x0B\0\x80\x02\x03`\xD0\x1B\x91\x01RV[a\x04k`\x06a\x04;V[\x90a\x04x` \x83\x01a\x04RV[V[a\x04\x82a\x04aV[\x90V[\x90a\x055\x91a\x04\x94__a\x02\tV[a\x04\x9F_`\x01a\x02^V[a\x04\xAA_`\x02a\x02\tV[a\x04\xB5_`\x03a\x02^V[a\x04\xE1a\x04\xC9a\x04\xC4\x84a\x02~V[a\x02\x81V[a\x04\xDBa\x04\xD5_a\x02?V[\x91a\0\xA1V[\x11a\x02\xE7V[a\x04\xE9a\x035V[a\x050a\x04\xF5_a\x03iV[\x92a\x05!a\x05\x03`\x01a\x03xV[\x95a\x05\r_a\x02?V[a\x05\x15a\x005V[\x97\x88\x94` \x86\x01a\x04\x05V[` \x82\x01\x81\x03\x82R\x03\x84a\0]V[a\t\x91V[a\x05o____\x91a\x05ia\x05ca\x05]a\x05Wa\x05Qa\x04zV[\x97a\x01\xEAV[\x93a\x02?V[\x93a\x01\xEAV[\x93a\x02?V[\x93a\x0C\xF7V[V[T\x90V[` \x01\x90V[a\x05\x8Fa\x05\x8Aa\x05\x94\x92a\0\xA1V[a\x01\xE7V[a\x01\xDCV[\x90V[a\x05\xA0\x90a\x03BV[\x90V[``\x1B\x90V[a\x05\xB2\x90a\x05\xA3V[\x90V[a\x05\xBE\x90a\x05\xA9V[\x90V[a\x05\xCDa\x05\xD2\x91a\x05\x97V[a\x05\xB5V[\x90RV[`\xC0\x1B\x90V[a\x05\xE5\x90a\x05\xD6V[\x90V[a\x05\xF4a\x05\xF9\x91a\x01\xDCV[a\x05\xDCV[\x90RV[\x90V[\x90V[a\x06\x0Fa\x06\x14\x91a\x05\xFDV[a\x06\0V[\x90RV[\x94a\x06i`\x08` \x99\x98\x95\x96a\x06a\x82\x8C\x99a\x06Y`\x14a\x06q\x9Aa\x06Qa\x06y\x9F\x8F\x9C\x90a\x06I\x81`\x01\x93a\x03\xBEV[\x01\x80\x92a\x05\xC1V[\x01\x80\x92a\x05\xE8V[\x01\x80\x92a\x05\xE8V[\x01\x80\x92a\x03\x97V[\x01\x80\x92a\x03\x97V[\x01\x80\x92a\x06\x03V[\x01\x90V[a\x06\x91a\x06\x8Ca\x06\x96\x92a\x01\xD9V[a\x01\xB9V[a\x05\xFDV[\x90V[a\x06\xADa\x06\xA8a\x06\xB2\x92a\x03uV[a\x01\xE7V[a\0\xA1V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x06\xD8a\x06\xDE\x91\x93\x92\x93a\0\xA1V[\x92a\0\xA1V[\x82\x03\x91\x82\x11a\x06\xE9WV[a\x06\xB5V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_R` _ \x90V[a\x07\x14\x81a\x05qV[\x82\x10\x15a\x07.Wa\x07&`\x01\x91a\x07\x02V[\x91\x02\x01\x90_\x90V[a\x06\xEEV[\x1C\x90V[\x90V[a\x07J\x90`\x08a\x07O\x93\x02a\x073V[a\x077V[\x90V[\x90a\x07]\x91Ta\x07:V[\x90V[\x90V[` \x81a\x07ua\x07}\x93\x83\x96\x95a\x06\x03V[\x01\x80\x92a\x06\x03V[\x01\x90V[_R` _ \x90V[T\x90V[a\x07\x97\x81a\x07\x8AV[\x82\x10\x15a\x07\xB1Wa\x07\xA9`\x01\x91a\x07\x81V[\x91\x02\x01\x90_\x90V[a\x06\xEEV[\x1B\x90V[\x91\x90`\x08a\x07\xD5\x91\x02\x91a\x07\xCF_\x19\x84a\x07\xB6V[\x92a\x07\xB6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x07\xE8\x90a\x05\xFDV[\x90V[_\x1C\x90V[a\x07\xF9\x90a\x07\xEBV[\x90V[\x91\x90a\x08\x12a\x08\ra\x08\x1A\x93a\x07\xDFV[a\x07\xF0V[\x90\x83Ta\x07\xBAV[\x90UV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x08NW\x82a\x08F\x91`\x01a\x08L\x95\x01\x81Ua\x07\x8EV[\x90a\x07\xFCV[V[a\0IV[a\x08ga\x08ba\x08l\x92a\x03BV[a\x01\xE7V[a\x03BV[\x90V[a\x08x\x90a\x08SV[\x90V[a\x08\x84\x90a\x08oV[\x90V[a\x08\x9Ba\x08\x96a\x08\xA0\x92a\0\xA1V[a\x01\xE7V[a\0\xA1V[\x90V[a\x08\xAC\x90a\x05\x97V[\x90RV[a\x08\xB9\x90a\x03\x13V[\x90RV[a\x08\xC6\x90a\x05\xFDV[\x90RV[a\x08\xD3\x90a\x02?V[\x90RV[a\x08\xE0\x90a\x01\xDCV[\x90RV[\x91\x94a\t,a\t6\x92\x98\x97\x95a\t\"`\xA0\x96a\t\x18a\t=\x9Aa\t\x0E`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x08\xA3V[` \x89\x01\x90a\x08\xB0V[`@\x87\x01\x90a\x08\xA3V[``\x85\x01\x90a\x08\xBDV[`\x80\x83\x01\x90a\x08\xCAV[\x01\x90a\x08\xD7V[V[` \x91\x81R\x01\x90V[a\tga\tp` \x93a\tu\x93a\t^\x81a\x02\x81V[\x93\x84\x80\x93a\t?V[\x95\x86\x91\x01a\0\xF0V[a\0?V[\x01\x90V[a\t\x8E\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\tHV[\x90V[\x90a\t\x9C`\x04a\x05qV[\x91\x83a\t\xB0a\t\xAA\x82a\x02\x81V[\x91a\x05uV[ \x91\x81a\t\xFD\x82\x91a\t\xEEa\t\xC4Ca\x05{V[a\t\xCDBa\x05{V[\x89a\t\xD7_a\x02?V[\x91\x8A\x93a\t\xE2a\x005V[\x98\x89\x97` \x89\x01a\x06\x18V[` \x82\x01\x81\x03\x82R\x03\x82a\0]V[a\n\x0Fa\n\t\x82a\x02\x81V[\x91a\x05uV[ \x90a\n\x1A_a\x06}V[\x91\x85a\n.a\n(_a\x02?V[\x91a\0\xA1V[\x11a\x0B/W[a\n\x86\x90a\nB`\x04a\x07`V[\x90a\nm\x85a\n^a\nRa\x005V[\x93\x84\x92` \x84\x01a\x07cV[` \x82\x01\x81\x03\x82R\x03\x82a\0]V[a\n\x7Fa\ny\x82a\x02\x81V[\x91a\x05uV[ \x90a\x08\x1EV[\x84\x91\x92a\n\xE8a\n\x950a\x08{V[\x91\x92\x95_a\n\xA2Ba\x05{V[\x91a\n\xD6a\n\xD0\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE1\x98a\x08\x87V[\x98a\x07\xDFV[\x98a\n\xDFa\x005V[\x96\x87\x96\x87a\x08\xE4V[\x03\x90\xA3a\x0B*a\x0B\x18\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x92a\x08\x87V[\x92a\x0B!a\x005V[\x91\x82\x91\x82a\tyV[\x03\x90\xA2V[\x91Pa\n\x86a\x0B\\a\x0BV`\x04a\x0BP\x89a\x0BJ`\x01a\x06\x99V[\x90a\x06\xC9V[\x90a\x07\x0BV[\x90a\x07RV[\x92\x90Pa\n4V[a\x0Bxa\x0Bsa\x0B}\x92a\x01\xDCV[a\x01\xE7V[a\x01\xDCV[\x90V[\x90a\x0B\x95a\x0B\x90a\x0B\x9C\x92a\x0BdV[a\x02\x06V[\x82Ta\x01\xBEV[\x90UV[\x90a\x0B\xB5a\x0B\xB0a\x0B\xBC\x92a\x08\x87V[a\x02[V[\x82Ta\x02)V[\x90UV[` \x93\x92a\x0B\xDF\x85\x83a\x0B\xD7\x82\x95a\x0B\xE7\x97a\x06\x03V[\x01\x80\x92a\x06\x03V[\x01\x80\x92a\x06\x03V[\x01\x90V[\x90V[a\x0B\xFAa\x0B\xFF\x91a\x07\xEBV[a\x0B\xEBV[\x90V[a\x0C\x0C\x90Ta\x0B\xEEV[\x90V[a\x0C\x18\x90a\0\xA1V[\x90RV[a\x0C%\x90a\x01\xDCV[\x90RV[\x90``\x80a\x0Co\x93a\x0CA_\x82\x01Q_\x86\x01\x90a\x0C\x1CV[a\x0CS` \x82\x01Q` \x86\x01\x90a\x0C\x1CV[a\x0Ce`@\x82\x01Q`@\x86\x01\x90a\x0C\x1CV[\x01Q\x91\x01\x90a\x0C\x1CV[V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x05\x11\x15a\x0C\x8FWV[a\x0CqV[\x90a\x0C\x9E\x82a\x0C\x85V[V[a\x0C\xA9\x90a\x0C\x94V[\x90V[a\x0C\xB5\x90a\x0C\xA0V[\x90RV[a\x0C\xEEa\x0C\xF5\x94a\x0C\xE4`\xC0\x94\x98\x97\x95a\x0C\xDA`\xE0\x86\x01\x9A_\x87\x01\x90a\x08\xBDV[` \x85\x01\x90a\x0C\x0FV[`@\x83\x01\x90a\x0C)V[\x01\x90a\x0C\xACV[V[\x93\x91a\r\x06a\r\r\x92_a\x0B\x80V[`\x01a\x0B\xA0V[\x81a\r a\r\x1A_a\x01\xEAV[\x91a\x01\xDCV[\x11a\x0E\xFAW[PPa\r2`\x04a\x05qV[a\r=\x82\x82\x90a\x127V[\x90a\rH`\x05a\x05qV[\x92a\rR_a\x06}V[\x84a\rea\r__a\x02?V[\x91a\0\xA1V[\x11a\x0E\xCCW[a\rt_a\x06}V[\x92\x82a\r\x88a\r\x82_a\x02?V[\x91a\0\xA1V[\x11a\x0E\x97W[a\r\xEC\x90a\r\xBD\x83a\r\xAE\x87a\r\xA2a\x005V[\x94\x85\x93` \x85\x01a\x0B\xC0V[` \x82\x01\x81\x03\x82R\x03\x82a\0]V[a\r\xCFa\r\xC9\x82a\x02\x81V[\x91a\x05uV[ \x92a\r\xE5a\r\xDE`\x05a\x07`V[\x85\x90a\x08\x1EV[`\x06a\x0B\xA0V[\x84\x90\x91\x92\x93a\r\xFB`\x06a\x0C\x02V[\x90\x94a\x0EP`\x01a\x0E>a\x0E8a\x0E2\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x97a\x08\x87V[\x97a\x07\xDFV[\x97a\x07\xDFV[\x97a\x0EGa\x005V[\x94\x85\x94\x85a\x0C\xB9V[\x03\x90\xA4a\x0E\x92a\x0E\x80\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x92a\x08\x87V[\x92a\x0E\x89a\x005V[\x91\x82\x91\x82a\tyV[\x03\x90\xA2V[\x92Pa\r\xECa\x0E\xC4a\x0E\xBE`\x04a\x0E\xB8\x86a\x0E\xB2`\x01a\x06\x99V[\x90a\x06\xC9V[\x90a\x07\x0BV[\x90a\x07RV[\x93\x90Pa\r\x8EV[Pa\x0E\xF5a\x0E\xEF`\x05a\x0E\xE9\x87a\x0E\xE3`\x01a\x06\x99V[\x90a\x06\xC9V[\x90a\x07\x0BV[\x90a\x07RV[a\rkV[a\x0F\x08a\x0F\x0F\x92`\x02a\x0B\x80V[`\x03a\x0B\xA0V[_\x80a\r&V[_\x90V[a\x0F$`\x80a\0\x84V[\x90V[_\x90V[a\x0F3a\x0F\x1AV[\x90` \x80\x80\x80\x85a\x0FBa\x0F'V[\x81R\x01a\x0FMa\x0F'V[\x81R\x01a\x0FXa\x0F'V[\x81R\x01a\x0Fca\x0F'V[\x81RPPV[a\x0Fqa\x0F+V[\x90V[\x90V[a\x0F\x8Ba\x0F\x86a\x0F\x90\x92a\x0FtV[a\x01\xE7V[a\0\xA1V[\x90V[a\x0F\x9D`(a\x0FwV[\x90V[a\x0F\xAFa\x0F\xB5\x91\x93\x92\x93a\0\xA1V[\x92a\0\xA1V[\x82\x01\x80\x92\x11a\x0F\xC0WV[a\x06\xB5V[\x90V[a\x0F\xDCa\x0F\xD7a\x0F\xE1\x92a\x0F\xC5V[a\x01\xE7V[a\x01\xDCV[\x90V[a\x0F\xF0b\x01\xCC\xCCa\x0F\xC8V[\x90V[a\x10\x07a\x10\x02a\x10\x0C\x92a\x01\xDCV[a\x01\xE7V[a\0\xA1V[\x90V[a\x10\x18\x90a\x0F\xF3V[\x90RV[\x91` a\x10=\x92\x94\x93a\x106`@\x82\x01\x96_\x83\x01\x90a\x0C\x0FV[\x01\x90a\x10\x0FV[V[\x90V[a\x10Va\x10Qa\x10[\x92a\x10?V[a\x01\xE7V[a\x01\xDCV[\x90V[a\x10jb\x01Q\x80a\x10BV[\x90V[a\x10ya\x10\x7F\x91a\x01\xDCV[\x91a\x01\xDCV[\x90\x03\x90`\x01\x80`@\x1B\x03\x82\x11a\x10\x91WV[a\x06\xB5V[\x90a\x10\xA0\x90a\x01\xDCV[\x90RV[\x90V[a\x10\xBBa\x10\xB6a\x10\xC0\x92a\x10\xA4V[a\x01\xE7V[a\x01\xDCV[\x90V[a\x10\xCEa\x0E\x10a\x10\xA7V[\x90V[a\x10\xDDa\x10\xE3\x91a\x01\xDCV[\x91a\x01\xDCV[\x01\x90`\x01\x80`@\x1B\x03\x82\x11a\x10\xF4WV[a\x06\xB5V[\x90V[a\x11\x10a\x11\x0Ba\x11\x15\x92a\x10\xF9V[a\x01\xE7V[a\x01\xDCV[\x90V[a\x11#a\x1C a\x10\xFCV[\x90V[\x90V[a\x11=a\x118a\x11B\x92a\x11&V[a\x01\xE7V[a\x01\xDCV[\x90V[a\x11O`\x0Ca\x11)V[\x90V[a\x11\\\x90Qa\x01\xDCV[\x90V[`\x08a\x11\xA1\x94a\x11\x91\x82\x80\x99\x98\x95\x96a\x11\x89\x82\x87a\x11\x81a\x11\x99\x99\x83\x9Ca\x05\xE8V[\x01\x80\x92a\x05\xE8V[\x01\x80\x92a\x05\xE8V[\x01\x80\x92a\x05\xE8V[\x01\x80\x92a\x05\xE8V[\x01\x90V[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[\x15a\x11\xC0WV[a\x11\xA5V[\x90P\x90V[a\x11\xEFa\x11\xE6\x92` \x92a\x11\xDD\x81a\x02\x81V[\x94\x85\x80\x93a\x11\xC5V[\x93\x84\x91\x01a\0\xF0V[\x01\x90V[a\x12\x01\x90a\x12\x07\x93\x92a\x11\xCAV[\x90a\x11\xCAV[\x90V[a\x12)\x92\x91a\x125\x91a\x12\x1Ba\x005V[\x94\x85\x92` \x84\x01\x92\x83a\x11\xF3V[\x90\x81\x03\x82R\x03\x83a\0]V[V[\x91\x90\x91a\x12Ba\x0F\x16V[Pa\x12Ka\x0FiV[Pa\x12fa\x12Wa\x0F\x93V[a\x12`\x83a\x02\x81V[\x90a\x0F\xA0V[\x80a\x12\x80a\x12za\x12ua\x0F\xE4V[a\x0F\xF3V[\x91a\0\xA1V[\x11a\x14\x1FWPa\x13\xB3\x90a\x12\x92a\x0FiV[\x93Ba\x12\xADa\x12\xA7a\x12\xA2a\x10^V[a\x0F\xF3V[\x91a\0\xA1V[\x11a\x13\xF5W[a\x12\xD9a\x12\xD0a\x12\xC2Ba\x05{V[a\x12\xCAa\x10\xC3V[\x90a\x10\xD1V[` \x87\x01a\x10\x96V[Ca\x12\xF3a\x12\xEDa\x12\xE8a\x11\x18V[a\x0F\xF3V[\x91a\0\xA1V[\x11a\x13\xCAW[a\x13\x1Fa\x13\x16a\x13\x08Ca\x05{V[a\x13\x10a\x11EV[\x90a\x10\xD1V[``\x87\x01a\x10\x96V[a\x13\x83a\x13-_\x87\x01a\x11RV[a\x13ta\x13<` \x89\x01a\x11RV[\x93a\x13I`@\x8A\x01a\x11RV[\x90a\x13_a\x13Y``\x8C\x01a\x11RV[\x91a\x05{V[\x91a\x13ha\x005V[\x96\x87\x95` \x87\x01a\x11_V[` \x82\x01\x81\x03\x82R\x03\x82a\0]V[a\x13\xAEa\x13\x8F\x82a\x02\x81V[a\x13\xA8a\x13\xA2a\x13\x9Da\x0F\x93V[a\0\xA1V[\x91a\0\xA1V[\x14a\x11\xB9V[a\x12\nV[a\x13\xC5a\x13\xBF\x82a\x02\x81V[\x91a\x05uV[ \x91\x90V[a\x13\xF0a\x13\xE7a\x13\xD9Ca\x05{V[a\x13\xE1a\x11\x18V[\x90a\x10mV[`@\x87\x01a\x10\x96V[a\x12\xF9V[a\x14\x1Aa\x14\x12a\x14\x04Ba\x05{V[a\x14\x0Ca\x10^V[\x90a\x10mV[_\x87\x01a\x10\x96V[a\x12\xB3V[a\x14'a\x0F\xE4V[\x90a\x14B_\x92\x83\x92cF4i\x1B`\xE0\x1B\x84R`\x04\x84\x01a\x10\x1CV[\x03\x90\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x0B\xA1V[a\0\x1D_5a\x01[V[\x80b\x84\x12\x0C\x14a\x01VW\x80c\x04\xF1\xC8T\x14a\x01QW\x80c\x05m\xAA\xA6\x14a\x01LW\x80c\x06\x1D\x12\xC0\x14a\x01GW\x80c\x06\xF10V\x14a\x01BW\x80c\x16\xBFUy\x14a\x01=W\x80c\x18\xDB9@\x14a\x018W\x80c/\x1E\xC5\xE9\x14a\x013W\x80cO5\x9A7\x14a\x01.W\x80c\x7F\xA3\xA4\x0E\x14a\x01)W\x80c\xA7\xB5\x1D\x19\x14a\x01$W\x80c\xAD\x9C\x0C.\x14a\x01\x1FW\x80c\xB7R\xA7\xD1\x14a\x01\x1AW\x80c\xD5q\x9D\xC2\x14a\x01\x15W\x80c\xD5\x95L4\x14a\x01\x10W\x80c\xD9\xDDg\xAB\x14a\x01\x0BW\x80c\xE1\xD6j\xFE\x14a\x01\x06W\x80c\xE8\xEB\x1D\xC3\x14a\x01\x01W\x80c\xEC\xA0g\xAD\x14a\0\xFCWc\xFB\xF6\xEA\xA5\x03a\0\x0EWa\x0BlV[a\x0B(V[a\n\xF3V[a\n\x86V[a\n\x17V[a\t\xDEV[a\tkV[a\t\nV[a\x08\x9CV[a\x080V[a\x07GV[a\x07\x03V[a\x06\x97V[a\x06'V[a\x05\xE3V[a\x04\xCDV[a\x04\x96V[a\x02\xB9V[a\x02\x14V[a\x01\xA3V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01yWV[a\x01kV[\x90V[a\x01\x8A\x90a\x01~V[\x90RV[\x91\x90a\x01\xA1\x90_` \x85\x01\x94\x01\x90a\x01\x81V[V[4a\x01\xD3Wa\x01\xB36`\x04a\x01oV[a\x01\xCFa\x01\xBEa\x0B\xA9V[a\x01\xC6a\x01aV[\x91\x82\x91\x82a\x01\x8EV[\x03\x90\xF3[a\x01gV[\x1C\x90V[\x90V[a\x01\xEF\x90`\x08a\x01\xF4\x93\x02a\x01\xD8V[a\x01\xDCV[\x90V[\x90a\x02\x02\x91Ta\x01\xDFV[\x90V[a\x02\x11`\x03_\x90a\x01\xF7V[\x90V[4a\x02DWa\x02$6`\x04a\x01oV[a\x02@a\x02/a\x02\x05V[a\x027a\x01aV[\x91\x82\x91\x82a\x01\x8EV[\x03\x90\xF3[a\x01gV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02f\x90`\x08a\x02k\x93\x02a\x01\xD8V[a\x02IV[\x90V[\x90a\x02y\x91Ta\x02VV[\x90V[a\x02\x87__\x90a\x02nV[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\xA0\x90a\x02\x8AV[\x90RV[\x91\x90a\x02\xB7\x90_` \x85\x01\x94\x01\x90a\x02\x97V[V[4a\x02\xE9Wa\x02\xC96`\x04a\x01oV[a\x02\xE5a\x02\xD4a\x02|V[a\x02\xDCa\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x03\"\x90a\x02\xFAV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03<W`@RV[a\x03\x04V[\x90a\x03Ta\x03Ma\x01aV[\x92\x83a\x03\x18V[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03tWa\x03p` \x91a\x02\xFAV[\x01\x90V[a\x03\x04V[\x90\x82_\x93\x92\x827\x01RV[\x90\x92\x91\x92a\x03\x99a\x03\x94\x82a\x03VV[a\x03AV[\x93\x81\x85R` \x85\x01\x90\x82\x84\x01\x11a\x03\xB5Wa\x03\xB3\x92a\x03yV[V[a\x02\xF6V[\x90\x80`\x1F\x83\x01\x12\x15a\x03\xD8W\x81` a\x03\xD5\x935\x91\x01a\x03\x84V[\x90V[a\x02\xF2V[a\x03\xE6\x81a\x02\x8AV[\x03a\x03\xEDWV[_\x80\xFD[\x90P5\x90a\x03\xFE\x82a\x03\xDDV[V[a\x04\t\x81a\x01~V[\x03a\x04\x10WV[_\x80\xFD[\x90P5\x90a\x04!\x82a\x04\0V[V[\x91\x90`\xA0\x83\x82\x03\x12a\x04\x8CW_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x87W\x81a\x04N\x91\x85\x01a\x03\xBAV[\x92a\x04\\\x82` \x83\x01a\x03\xF1V[\x92a\x04\x84a\x04m\x84`@\x85\x01a\x04\x14V[\x93a\x04{\x81``\x86\x01a\x03\xF1V[\x93`\x80\x01a\x04\x14V[\x90V[a\x02\xEEV[a\x01kV[_\x01\x90V[4a\x04\xC8Wa\x04\xB2a\x04\xA96`\x04a\x04#V[\x93\x92\x90\x92a\x0F\x9FV[a\x04\xBAa\x01aV[\x80a\x04\xC4\x81a\x04\x91V[\x03\x90\xF3[a\x01gV[4a\x04\xFDWa\x04\xDD6`\x04a\x01oV[a\x04\xF9a\x04\xE8a\x11\xBEV[a\x04\xF0a\x01aV[\x91\x82\x91\x82a\x01\x8EV[\x03\x90\xF3[a\x01gV[\x90` \x82\x82\x03\x12a\x05\x1BWa\x05\x18\x91_\x01a\x04\x14V[\x90V[a\x01kV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[T\x90V[_R` _ \x90V[a\x05J\x81a\x054V[\x82\x10\x15a\x05dWa\x05\\`\x01\x91a\x058V[\x91\x02\x01\x90_\x90V[a\x05 V[\x90V[a\x05|\x90`\x08a\x05\x81\x93\x02a\x01\xD8V[a\x05iV[\x90V[\x90a\x05\x8F\x91Ta\x05lV[\x90V[`\x05a\x05\x9D\x81a\x054V[\x82\x10\x15a\x05\xBAWa\x05\xB7\x91a\x05\xB1\x91a\x05AV[\x90a\x05\x84V[\x90V[_\x80\xFD[\x90V[a\x05\xCA\x90a\x05\xBEV[\x90RV[\x91\x90a\x05\xE1\x90_` \x85\x01\x94\x01\x90a\x05\xC1V[V[4a\x06\x13Wa\x06\x0Fa\x05\xFEa\x05\xF96`\x04a\x05\x02V[a\x05\x92V[a\x06\x06a\x01aV[\x91\x82\x91\x82a\x05\xCEV[\x03\x90\xF3[a\x01gV[a\x06$`\x01_\x90a\x01\xF7V[\x90V[4a\x06WWa\x0676`\x04a\x01oV[a\x06Sa\x06Ba\x06\x18V[a\x06Ja\x01aV[\x91\x82\x91\x82a\x01\x8EV[\x03\x90\xF3[a\x01gV[\x90V[\x90V[a\x06va\x06qa\x06{\x92a\x06\\V[a\x06_V[a\x02\x8AV[\x90V[a\x06\x89a\x0E\x10a\x06bV[\x90V[a\x06\x94a\x06~V[\x90V[4a\x06\xC7Wa\x06\xA76`\x04a\x01oV[a\x06\xC3a\x06\xB2a\x06\x8CV[a\x06\xBAa\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[\x90V[a\x06\xE3a\x06\xDEa\x06\xE8\x92a\x06\xCCV[a\x06_V[a\x02\x8AV[\x90V[a\x06\xF5`\x0Ca\x06\xCFV[\x90V[a\x07\0a\x06\xEBV[\x90V[4a\x073Wa\x07\x136`\x04a\x01oV[a\x07/a\x07\x1Ea\x06\xF8V[a\x07&a\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[a\x07D`\x06_\x90a\x01\xF7V[\x90V[4a\x07wWa\x07W6`\x04a\x01oV[a\x07sa\x07ba\x078V[a\x07ja\x01aV[\x91\x82\x91\x82a\x01\x8EV[\x03\x90\xF3[a\x01gV[`\xFF\x16\x90V[a\x07\x8B\x81a\x07|V[\x03a\x07\x92WV[_\x80\xFD[\x90P5\x90a\x07\xA3\x82a\x07\x82V[V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x07\xB9\x90a\x07\xA5V[\x90V[a\x07\xC5\x81a\x07\xB0V[\x03a\x07\xCCWV[_\x80\xFD[\x90P5\x90a\x07\xDD\x82a\x07\xBCV[V[\x91``\x83\x83\x03\x12a\x08+Wa\x07\xF6\x82_\x85\x01a\x07\x96V[\x92a\x08\x04\x83` \x83\x01a\x07\xD0V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08&Wa\x08#\x92\x01a\x03\xBAV[\x90V[a\x02\xEEV[a\x01kV[4a\x08_Wa\x08Ia\x08C6`\x04a\x07\xDFV[\x91a\x13\xC2V[a\x08Qa\x01aV[\x80a\x08[\x81a\x04\x91V[\x03\x90\xF3[a\x01gV[\x90V[a\x08{a\x08va\x08\x80\x92a\x08dV[a\x06_V[a\x02\x8AV[\x90V[a\x08\x8Ea\x1C a\x08gV[\x90V[a\x08\x99a\x08\x83V[\x90V[4a\x08\xCCWa\x08\xAC6`\x04a\x01oV[a\x08\xC8a\x08\xB7a\x08\x91V[a\x08\xBFa\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[\x90V[a\x08\xE8a\x08\xE3a\x08\xED\x92a\x08\xD1V[a\x06_V[a\x02\x8AV[\x90V[a\x08\xFCb\x01Q\x80a\x08\xD4V[\x90V[a\t\x07a\x08\xF0V[\x90V[4a\t:Wa\t\x1A6`\x04a\x01oV[a\t6a\t%a\x08\xFFV[a\t-a\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[`\x04a\tJ\x81a\x054V[\x82\x10\x15a\tgWa\td\x91a\t^\x91a\x05AV[\x90a\x05\x84V[\x90V[_\x80\xFD[4a\t\x9BWa\t\x97a\t\x86a\t\x816`\x04a\x05\x02V[a\t?V[a\t\x8Ea\x01aV[\x91\x82\x91\x82a\x05\xCEV[\x03\x90\xF3[a\x01gV[a\t\xD5a\t\xDC\x94a\t\xCB``\x94\x98\x97\x95a\t\xC1`\x80\x86\x01\x9A_\x87\x01\x90a\x02\x97V[` \x85\x01\x90a\x01\x81V[`@\x83\x01\x90a\x02\x97V[\x01\x90a\x01\x81V[V[4a\n\x12Wa\t\xEE6`\x04a\x01oV[a\n\x0Ea\t\xF9a\x15\xBAV[\x90a\n\x05\x94\x92\x94a\x01aV[\x94\x85\x94\x85a\t\xA0V[\x03\x90\xF3[a\x01gV[4a\nGWa\nCa\n2a\n-6`\x04a\x05\x02V[a\x16\x13V[a\n:a\x01aV[\x91\x82\x91\x82a\x05\xCEV[\x03\x90\xF3[a\x01gV[\x90\x91``\x82\x84\x03\x12a\n\x81Wa\n~a\ng\x84_\x85\x01a\x07\xD0V[\x93a\nu\x81` \x86\x01a\x07\xD0V[\x93`@\x01a\x04\x14V[\x90V[a\x01kV[4a\n\xB5Wa\n\x9Fa\n\x996`\x04a\nLV[\x91a\x16{V[a\n\xA7a\x01aV[\x80a\n\xB1\x81a\x04\x91V[\x03\x90\xF3[a\x01gV[\x90V[a\n\xD1a\n\xCCa\n\xD6\x92a\n\xBAV[a\x06_V[a\x02\x8AV[\x90V[a\n\xE5b\x01\xCC\xCCa\n\xBDV[\x90V[a\n\xF0a\n\xD9V[\x90V[4a\x0B#Wa\x0B\x036`\x04a\x01oV[a\x0B\x1Fa\x0B\x0Ea\n\xE8V[a\x0B\x16a\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[4a\x0BXWa\x0B86`\x04a\x01oV[a\x0BTa\x0BCa\x16\xBBV[a\x0BKa\x01aV[\x91\x82\x91\x82a\x01\x8EV[\x03\x90\xF3[a\x01gV[a\x0Bi`\x02_\x90a\x02nV[\x90V[4a\x0B\x9CWa\x0B|6`\x04a\x01oV[a\x0B\x98a\x0B\x87a\x0B]V[a\x0B\x8Fa\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[_\x80\xFD[_\x90V[a\x0B\xB1a\x0B\xA5V[Pa\x0B\xBC`\x05a\x054V[\x90V[_\x1B\x90V[\x90a\x0B\xD7g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x0B\xBFV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x0B\xF5a\x0B\xF0a\x0B\xFA\x92a\x02\x8AV[a\x06_V[a\x02\x8AV[\x90V[\x90V[\x90a\x0C\x15a\x0C\x10a\x0C\x1C\x92a\x0B\xE1V[a\x0B\xFDV[\x82Ta\x0B\xC4V[\x90UV[\x90a\x0C,_\x19\x91a\x0B\xBFV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x0CJa\x0CEa\x0CO\x92a\x01~V[a\x06_V[a\x01~V[\x90V[\x90V[\x90a\x0Cja\x0Cea\x0Cq\x92a\x0C6V[a\x0CRV[\x82Ta\x0C V[\x90UV[\x90V[a\x0C\x8Ca\x0C\x87a\x0C\x91\x92a\x0CuV[a\x06_V[a\x02\x8AV[\x90V[a\x0C\xA8a\x0C\xA3a\x0C\xAD\x92a\x0CuV[a\x0B\xBFV[a\x05\xBEV[\x90V[a\x0C\xC4a\x0C\xBFa\x0C\xC9\x92a\x0CuV[a\x06_V[a\x01~V[\x90V[\x90V[a\x0C\xE3a\x0C\xDEa\x0C\xE8\x92a\x0C\xCCV[a\x06_V[a\x01~V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\r\x0Ea\r\x14\x91\x93\x92\x93a\x01~V[\x92a\x01~V[\x82\x03\x91\x82\x11a\r\x1FWV[a\x0C\xEBV[\x90V[a\r3a\r8\x91a\x05\xBEV[a\r$V[\x90RV[` \x93\x92a\r[\x85\x83a\rS\x82\x95a\rc\x97a\r'V[\x01\x80\x92a\r'V[\x01\x80\x92a\r'V[\x01\x90V[` \x01\x90V[Q\x90V[\x90V[_R` _ \x90V[T\x90V[a\r\x8A\x81a\r}V[\x82\x10\x15a\r\xA4Wa\r\x9C`\x01\x91a\rtV[\x91\x02\x01\x90_\x90V[a\x05 V[\x1B\x90V[\x91\x90`\x08a\r\xC8\x91\x02\x91a\r\xC2_\x19\x84a\r\xA9V[\x92a\r\xA9V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\r\xDB\x90a\x05\xBEV[\x90V[_\x1C\x90V[a\r\xEC\x90a\r\xDEV[\x90V[\x91\x90a\x0E\x05a\x0E\0a\x0E\r\x93a\r\xD2V[a\r\xE3V[\x90\x83Ta\r\xADV[\x90UV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x0EAW\x82a\x0E9\x91`\x01a\x0E?\x95\x01\x81Ua\r\x81V[\x90a\r\xEFV[V[a\x03\x04V[a\x0ERa\x0EW\x91a\r\xDEV[a\x01\xDCV[\x90V[a\x0Ed\x90Ta\x0EFV[\x90V[a\x0Ep\x90a\x02\x8AV[\x90RV[\x90``\x80a\x0E\xBA\x93a\x0E\x8C_\x82\x01Q_\x86\x01\x90a\x0EgV[a\x0E\x9E` \x82\x01Q` \x86\x01\x90a\x0EgV[a\x0E\xB0`@\x82\x01Q`@\x86\x01\x90a\x0EgV[\x01Q\x91\x01\x90a\x0EgV[V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x05\x11\x15a\x0E\xDAWV[a\x0E\xBCV[\x90a\x0E\xE9\x82a\x0E\xD0V[V[a\x0E\xF4\x90a\x0E\xDFV[\x90V[a\x0F\0\x90a\x0E\xEBV[\x90RV[a\x0F9a\x0F@\x94a\x0F/`\xC0\x94\x98\x97\x95a\x0F%`\xE0\x86\x01\x9A_\x87\x01\x90a\x05\xC1V[` \x85\x01\x90a\x01\x81V[`@\x83\x01\x90a\x0EtV[\x01\x90a\x0E\xF7V[V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[a\x0Fua\x0F~` \x93a\x0F\x83\x93a\x0Fl\x81a\rmV[\x93\x84\x80\x93a\x0FBV[\x95\x86\x91\x01a\x0FKV[a\x02\xFAV[\x01\x90V[a\x0F\x9C\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x0FVV[\x90V[\x93\x91a\x0F\xAEa\x0F\xB5\x92_a\x0C\0V[`\x01a\x0CUV[\x81a\x0F\xC8a\x0F\xC2_a\x0CxV[\x91a\x02\x8AV[\x11a\x11\xA2W[PPa\x0F\xDA`\x04a\x054V[a\x0F\xE5\x82\x82\x90a\x19\x10V[\x90a\x0F\xF0`\x05a\x054V[\x92a\x0F\xFA_a\x0C\x94V[\x84a\x10\ra\x10\x07_a\x0C\xB0V[\x91a\x01~V[\x11a\x11tW[a\x10\x1C_a\x0C\x94V[\x92\x82a\x100a\x10*_a\x0C\xB0V[\x91a\x01~V[\x11a\x11?W[a\x10\x94\x90a\x10e\x83a\x10V\x87a\x10Ja\x01aV[\x94\x85\x93` \x85\x01a\r<V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\x18V[a\x10wa\x10q\x82a\rmV[\x91a\rgV[ \x92a\x10\x8Da\x10\x86`\x05a\rqV[\x85\x90a\x0E\x11V[`\x06a\x0CUV[\x84\x90\x91\x92\x93a\x10\xA3`\x06a\x0EZV[\x90\x94a\x10\xF8`\x01a\x10\xE6a\x10\xE0a\x10\xDA\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x97a\x0C6V[\x97a\r\xD2V[\x97a\r\xD2V[\x97a\x10\xEFa\x01aV[\x94\x85\x94\x85a\x0F\x04V[\x03\x90\xA4a\x11:a\x11(\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x92a\x0C6V[\x92a\x111a\x01aV[\x91\x82\x91\x82a\x0F\x87V[\x03\x90\xA2V[\x92Pa\x10\x94a\x11la\x11f`\x04a\x11`\x86a\x11Z`\x01a\x0C\xCFV[\x90a\x0C\xFFV[\x90a\x05AV[\x90a\x05\x84V[\x93\x90Pa\x106V[Pa\x11\x9Da\x11\x97`\x05a\x11\x91\x87a\x11\x8B`\x01a\x0C\xCFV[\x90a\x0C\xFFV[\x90a\x05AV[\x90a\x05\x84V[a\x10\x13V[a\x11\xB0a\x11\xB7\x92`\x02a\x0C\0V[`\x03a\x0CUV[_\x80a\x0F\xCEV[a\x11\xC6a\x0B\xA5V[Pa\x11\xD1`\x05a\x054V[\x90V[a\x11\xE8a\x11\xE3a\x11\xED\x92a\x01~V[a\x06_V[a\x02\x8AV[\x90V[`\xF8\x1B\x90V[a\x11\xFF\x90a\x11\xF0V[\x90V[a\x12\x0Ea\x12\x13\x91a\x07|V[a\x11\xF6V[\x90RV[``\x1B\x90V[a\x12&\x90a\x12\x17V[\x90V[a\x122\x90a\x12\x1DV[\x90V[a\x12Aa\x12F\x91a\x07\xB0V[a\x12)V[\x90RV[`\xC0\x1B\x90V[a\x12Y\x90a\x12JV[\x90V[a\x12ha\x12m\x91a\x02\x8AV[a\x12PV[\x90RV[\x90V[a\x12\x80a\x12\x85\x91a\x01~V[a\x12qV[\x90RV[\x94a\x12\xDA`\x08` \x99\x98\x95\x96a\x12\xD2\x82\x8C\x99a\x12\xCA`\x14a\x12\xE2\x9Aa\x12\xC2a\x12\xEA\x9F\x8F\x9C\x90a\x12\xBA\x81`\x01\x93a\x12\x02V[\x01\x80\x92a\x125V[\x01\x80\x92a\x12\\V[\x01\x80\x92a\x12\\V[\x01\x80\x92a\x12tV[\x01\x80\x92a\x12tV[\x01\x80\x92a\r'V[\x01\x90V[` \x81a\x13\0a\x13\x08\x93\x83\x96\x95a\r'V[\x01\x80\x92a\r'V[\x01\x90V[a\x13 a\x13\x1Ba\x13%\x92a\x07\xA5V[a\x06_V[a\x07\xA5V[\x90V[a\x131\x90a\x13\x0CV[\x90V[a\x13=\x90a\x13(V[\x90V[a\x13I\x90a\x07\xB0V[\x90RV[a\x13V\x90a\x07|V[\x90RV[a\x13c\x90a\x0C\xB0V[\x90RV[\x91\x94a\x13\xAFa\x13\xB9\x92\x98\x97\x95a\x13\xA5`\xA0\x96a\x13\x9Ba\x13\xC0\x9Aa\x13\x91`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x13@V[` \x89\x01\x90a\x13MV[`@\x87\x01\x90a\x13@V[``\x85\x01\x90a\x05\xC1V[`\x80\x83\x01\x90a\x13ZV[\x01\x90a\x02\x97V[V[\x90a\x13\xCD`\x04a\x054V[\x91\x83a\x13\xE1a\x13\xDB\x82a\rmV[\x91a\rgV[ \x91\x81a\x14.\x82\x91a\x14\x1Fa\x13\xF5Ca\x11\xD4V[a\x13\xFEBa\x11\xD4V[\x89a\x14\x08_a\x0C\xB0V[\x91\x8A\x93a\x14\x13a\x01aV[\x98\x89\x97` \x89\x01a\x12\x89V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\x18V[a\x14@a\x14:\x82a\rmV[\x91a\rgV[ \x90a\x14K_a\x0C\x94V[\x91\x85a\x14_a\x14Y_a\x0C\xB0V[\x91a\x01~V[\x11a\x15`W[a\x14\xB7\x90a\x14s`\x04a\rqV[\x90a\x14\x9E\x85a\x14\x8Fa\x14\x83a\x01aV[\x93\x84\x92` \x84\x01a\x12\xEEV[` \x82\x01\x81\x03\x82R\x03\x82a\x03\x18V[a\x14\xB0a\x14\xAA\x82a\rmV[\x91a\rgV[ \x90a\x0E\x11V[\x84\x91\x92a\x15\x19a\x14\xC60a\x134V[\x91\x92\x95_a\x14\xD3Ba\x11\xD4V[\x91a\x15\x07a\x15\x01\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE1\x98a\x0C6V[\x98a\r\xD2V[\x98a\x15\x10a\x01aV[\x96\x87\x96\x87a\x13gV[\x03\x90\xA3a\x15[a\x15I\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x92a\x0C6V[\x92a\x15Ra\x01aV[\x91\x82\x91\x82a\x0F\x87V[\x03\x90\xA2V[\x91Pa\x14\xB7a\x15\x8Da\x15\x87`\x04a\x15\x81\x89a\x15{`\x01a\x0C\xCFV[\x90a\x0C\xFFV[\x90a\x05AV[\x90a\x05\x84V[\x92\x90Pa\x14eV[_\x90V[a\x15\xA5a\x15\xAA\x91a\r\xDEV[a\x02IV[\x90V[a\x15\xB7\x90Ta\x15\x99V[\x90V[a\x15\xC2a\x15\x95V[Pa\x15\xCBa\x0B\xA5V[Pa\x15\xD4a\x15\x95V[Pa\x15\xDDa\x0B\xA5V[Pa\x15\xE7_a\x15\xADV[\x90a\x15\xF2`\x01a\x0EZV[\x91a\x15\xFD`\x02a\x15\xADV[\x91a\x16\x08`\x03a\x0EZV[\x91\x93\x92\x91\x90V[_\x90V[a\x16*a\x160\x91a\x16\"a\x16\x0FV[P`\x05a\x05AV[\x90a\x05\x84V[\x90V[a\x16Ga\x16Ba\x16L\x92a\x06\xCCV[a\x06_V[a\x07|V[\x90V[a\x16Y`\x0Ca\x163V[\x90V[`\x14\x81a\x16oa\x16w\x93` \x96\x95a\x125V[\x01\x80\x92a\x12tV[\x01\x90V[\x90a\x16\xB9\x92\x91a\x16\xB4a\x16\x8Ca\x16OV[\x91\x92a\x16\xA5a\x16\x99a\x01aV[\x95\x86\x92` \x84\x01a\x16\\V[` \x82\x01\x81\x03\x82R\x03\x84a\x03\x18V[a\x13\xC2V[V[a\x16\xC3a\x0B\xA5V[Pa\x16\xCE`\x04a\x054V[\x90V[a\x16\xDB`\x80a\x03AV[\x90V[_\x90V[a\x16\xEAa\x16\xD1V[\x90` \x80\x80\x80\x85a\x16\xF9a\x16\xDEV[\x81R\x01a\x17\x04a\x16\xDEV[\x81R\x01a\x17\x0Fa\x16\xDEV[\x81R\x01a\x17\x1Aa\x16\xDEV[\x81RPPV[a\x17(a\x16\xE2V[\x90V[\x90V[a\x17Ba\x17=a\x17G\x92a\x17+V[a\x06_V[a\x01~V[\x90V[a\x17T`(a\x17.V[\x90V[a\x17fa\x17l\x91\x93\x92\x93a\x01~V[\x92a\x01~V[\x82\x01\x80\x92\x11a\x17wWV[a\x0C\xEBV[a\x17\x90a\x17\x8Ba\x17\x95\x92a\x02\x8AV[a\x06_V[a\x01~V[\x90V[a\x17\xA1\x90a\x17|V[\x90RV[\x91` a\x17\xC6\x92\x94\x93a\x17\xBF`@\x82\x01\x96_\x83\x01\x90a\x01\x81V[\x01\x90a\x17\x98V[V[a\x17\xD4a\x17\xDA\x91a\x02\x8AV[\x91a\x02\x8AV[\x90\x03\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x17\xEEWV[a\x0C\xEBV[\x90a\x17\xFD\x90a\x02\x8AV[\x90RV[a\x18\ra\x18\x13\x91a\x02\x8AV[\x91a\x02\x8AV[\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x18&WV[a\x0C\xEBV[a\x185\x90Qa\x02\x8AV[\x90V[`\x08a\x18z\x94a\x18j\x82\x80\x99\x98\x95\x96a\x18b\x82\x87a\x18Za\x18r\x99\x83\x9Ca\x12\\V[\x01\x80\x92a\x12\\V[\x01\x80\x92a\x12\\V[\x01\x80\x92a\x12\\V[\x01\x80\x92a\x12\\V[\x01\x90V[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[\x15a\x18\x99WV[a\x18~V[\x90P\x90V[a\x18\xC8a\x18\xBF\x92` \x92a\x18\xB6\x81a\rmV[\x94\x85\x80\x93a\x18\x9EV[\x93\x84\x91\x01a\x0FKV[\x01\x90V[a\x18\xDA\x90a\x18\xE0\x93\x92a\x18\xA3V[\x90a\x18\xA3V[\x90V[a\x19\x02\x92\x91a\x19\x0E\x91a\x18\xF4a\x01aV[\x94\x85\x92` \x84\x01\x92\x83a\x18\xCCV[\x90\x81\x03\x82R\x03\x83a\x03\x18V[V[\x91\x90\x91a\x19\x1Ba\x16\x0FV[Pa\x19$a\x17 V[Pa\x19?a\x190a\x17JV[a\x199\x83a\rmV[\x90a\x17WV[\x80a\x19Ya\x19Sa\x19Na\n\xD9V[a\x17|V[\x91a\x01~V[\x11a\x1A\xF8WPa\x1A\x8C\x90a\x19ka\x17 V[\x93Ba\x19\x86a\x19\x80a\x19{a\x08\xF0V[a\x17|V[\x91a\x01~V[\x11a\x1A\xCEW[a\x19\xB2a\x19\xA9a\x19\x9BBa\x11\xD4V[a\x19\xA3a\x06~V[\x90a\x18\x01V[` \x87\x01a\x17\xF3V[Ca\x19\xCCa\x19\xC6a\x19\xC1a\x08\x83V[a\x17|V[\x91a\x01~V[\x11a\x1A\xA3W[a\x19\xF8a\x19\xEFa\x19\xE1Ca\x11\xD4V[a\x19\xE9a\x06\xEBV[\x90a\x18\x01V[``\x87\x01a\x17\xF3V[a\x1A\\a\x1A\x06_\x87\x01a\x18+V[a\x1AMa\x1A\x15` \x89\x01a\x18+V[\x93a\x1A\"`@\x8A\x01a\x18+V[\x90a\x1A8a\x1A2``\x8C\x01a\x18+V[\x91a\x11\xD4V[\x91a\x1AAa\x01aV[\x96\x87\x95` \x87\x01a\x188V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\x18V[a\x1A\x87a\x1Ah\x82a\rmV[a\x1A\x81a\x1A{a\x1Ava\x17JV[a\x01~V[\x91a\x01~V[\x14a\x18\x92V[a\x18\xE3V[a\x1A\x9Ea\x1A\x98\x82a\rmV[\x91a\rgV[ \x91\x90V[a\x1A\xC9a\x1A\xC0a\x1A\xB2Ca\x11\xD4V[a\x1A\xBAa\x08\x83V[\x90a\x17\xC8V[`@\x87\x01a\x17\xF3V[a\x19\xD2V[a\x1A\xF3a\x1A\xEBa\x1A\xDDBa\x11\xD4V[a\x1A\xE5a\x08\xF0V[\x90a\x17\xC8V[_\x87\x01a\x17\xF3V[a\x19\x8CV[a\x1B\0a\n\xD9V[\x90a\x1B\x1B_\x92\x83\x92cF4i\x1B`\xE0\x1B\x84R`\x04\x84\x01a\x17\xA5V[\x03\x90\xFD",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b610ba1565b61001d5f3561015b565b806284120c1461015657806304f1c85414610151578063056daaa61461014c578063061d12c01461014757806306f130561461014257806316bf55791461013d57806318db3940146101385780632f1ec5e9146101335780634f359a371461012e5780637fa3a40e14610129578063a7b51d1914610124578063ad9c0c2e1461011f578063b752a7d11461011a578063d5719dc214610115578063d5954c3414610110578063d9dd67ab1461010b578063e1d66afe14610106578063e8eb1dc314610101578063eca067ad146100fc5763fbf6eaa50361000e57610b6c565b610b28565b610af3565b610a86565b610a17565b6109de565b61096b565b61090a565b61089c565b610830565b610747565b610703565b610697565b610627565b6105e3565b6104cd565b610496565b6102b9565b610214565b6101a3565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f91031261017957565b61016b565b90565b61018a9061017e565b9052565b91906101a1905f60208501940190610181565b565b346101d3576101b336600461016f565b6101cf6101be610ba9565b6101c6610161565b9182918261018e565b0390f35b610167565b1c90565b90565b6101ef9060086101f493026101d8565b6101dc565b90565b9061020291546101df565b90565b61021160035f906101f7565b90565b346102445761022436600461016f565b61024061022f610205565b610237610161565b9182918261018e565b0390f35b610167565b67ffffffffffffffff1690565b61026690600861026b93026101d8565b610249565b90565b906102799154610256565b90565b6102875f5f9061026e565b90565b67ffffffffffffffff1690565b6102a09061028a565b9052565b91906102b7905f60208501940190610297565b565b346102e9576102c936600461016f565b6102e56102d461027c565b6102dc610161565b918291826102a4565b0390f35b610167565b5f80fd5b5f80fd5b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b90610322906102fa565b810190811067ffffffffffffffff82111761033c57604052565b610304565b9061035461034d610161565b9283610318565b565b67ffffffffffffffff8111610374576103706020916102fa565b0190565b610304565b90825f939282370152565b9092919261039961039482610356565b610341565b938185526020850190828401116103b5576103b392610379565b565b6102f6565b9080601f830112156103d8578160206103d593359101610384565b90565b6102f2565b6103e68161028a565b036103ed57565b5f80fd5b905035906103fe826103dd565b565b6104098161017e565b0361041057565b5f80fd5b9050359061042182610400565b565b919060a08382031261048c575f83013567ffffffffffffffff8111610487578161044e9185016103ba565b9261045c82602083016103f1565b9261048461046d8460408501610414565b9361047b81606086016103f1565b93608001610414565b90565b6102ee565b61016b565b5f0190565b346104c8576104b26104a9366004610423565b93929092610f9f565b6104ba610161565b806104c481610491565b0390f35b610167565b346104fd576104dd36600461016f565b6104f96104e86111be565b6104f0610161565b9182918261018e565b0390f35b610167565b9060208282031261051b57610518915f01610414565b90565b61016b565b634e487b7160e01b5f52603260045260245ffd5b5490565b5f5260205f2090565b61054a81610534565b8210156105645761055c600191610538565b910201905f90565b610520565b90565b61057c90600861058193026101d8565b610569565b90565b9061058f915461056c565b90565b600561059d81610534565b8210156105ba576105b7916105b191610541565b90610584565b90565b5f80fd5b90565b6105ca906105be565b9052565b91906105e1905f602085019401906105c1565b565b346106135761060f6105fe6105f9366004610502565b610592565b610606610161565b918291826105ce565b0390f35b610167565b61062460015f906101f7565b90565b346106575761063736600461016f565b610653610642610618565b61064a610161565b9182918261018e565b0390f35b610167565b90565b90565b61067661067161067b9261065c565b61065f565b61028a565b90565b610689610e10610662565b90565b61069461067e565b90565b346106c7576106a736600461016f565b6106c36106b261068c565b6106ba610161565b918291826102a4565b0390f35b610167565b90565b6106e36106de6106e8926106cc565b61065f565b61028a565b90565b6106f5600c6106cf565b90565b6107006106eb565b90565b346107335761071336600461016f565b61072f61071e6106f8565b610726610161565b918291826102a4565b0390f35b610167565b61074460065f906101f7565b90565b346107775761075736600461016f565b610773610762610738565b61076a610161565b9182918261018e565b0390f35b610167565b60ff1690565b61078b8161077c565b0361079257565b5f80fd5b905035906107a382610782565b565b60018060a01b031690565b6107b9906107a5565b90565b6107c5816107b0565b036107cc57565b5f80fd5b905035906107dd826107bc565b565b9160608383031261082b576107f6825f8501610796565b9261080483602083016107d0565b92604082013567ffffffffffffffff81116108265761082392016103ba565b90565b6102ee565b61016b565b3461085f576108496108433660046107df565b916113c2565b610851610161565b8061085b81610491565b0390f35b610167565b90565b61087b61087661088092610864565b61065f565b61028a565b90565b61088e611c20610867565b90565b610899610883565b90565b346108cc576108ac36600461016f565b6108c86108b7610891565b6108bf610161565b918291826102a4565b0390f35b610167565b90565b6108e86108e36108ed926108d1565b61065f565b61028a565b90565b6108fc620151806108d4565b90565b6109076108f0565b90565b3461093a5761091a36600461016f565b6109366109256108ff565b61092d610161565b918291826102a4565b0390f35b610167565b600461094a81610534565b821015610967576109649161095e91610541565b90610584565b90565b5f80fd5b3461099b57610997610986610981366004610502565b61093f565b61098e610161565b918291826105ce565b0390f35b610167565b6109d56109dc946109cb6060949897956109c1608086019a5f870190610297565b6020850190610181565b6040830190610297565b0190610181565b565b34610a12576109ee36600461016f565b610a0e6109f96115ba565b90610a05949294610161565b948594856109a0565b0390f35b610167565b34610a4757610a43610a32610a2d366004610502565b611613565b610a3a610161565b918291826105ce565b0390f35b610167565b9091606082840312610a8157610a7e610a67845f85016107d0565b93610a7581602086016107d0565b93604001610414565b90565b61016b565b34610ab557610a9f610a99366004610a4c565b9161167b565b610aa7610161565b80610ab181610491565b0390f35b610167565b90565b610ad1610acc610ad692610aba565b61065f565b61028a565b90565b610ae56201cccc610abd565b90565b610af0610ad9565b90565b34610b2357610b0336600461016f565b610b1f610b0e610ae8565b610b16610161565b918291826102a4565b0390f35b610167565b34610b5857610b3836600461016f565b610b54610b436116bb565b610b4b610161565b9182918261018e565b0390f35b610167565b610b6960025f9061026e565b90565b34610b9c57610b7c36600461016f565b610b98610b87610b5d565b610b8f610161565b918291826102a4565b0390f35b610167565b5f80fd5b5f90565b610bb1610ba5565b50610bbc6005610534565b90565b5f1b90565b90610bd767ffffffffffffffff91610bbf565b9181191691161790565b610bf5610bf0610bfa9261028a565b61065f565b61028a565b90565b90565b90610c15610c10610c1c92610be1565b610bfd565b8254610bc4565b9055565b90610c2c5f1991610bbf565b9181191691161790565b610c4a610c45610c4f9261017e565b61065f565b61017e565b90565b90565b90610c6a610c65610c7192610c36565b610c52565b8254610c20565b9055565b90565b610c8c610c87610c9192610c75565b61065f565b61028a565b90565b610ca8610ca3610cad92610c75565b610bbf565b6105be565b90565b610cc4610cbf610cc992610c75565b61065f565b61017e565b90565b90565b610ce3610cde610ce892610ccc565b61065f565b61017e565b90565b634e487b7160e01b5f52601160045260245ffd5b610d0e610d149193929361017e565b9261017e565b8203918211610d1f57565b610ceb565b90565b610d33610d38916105be565b610d24565b9052565b60209392610d5b8583610d538295610d6397610d27565b018092610d27565b018092610d27565b0190565b60200190565b5190565b90565b5f5260205f2090565b5490565b610d8a81610d7d565b821015610da457610d9c600191610d74565b910201905f90565b610520565b1b90565b91906008610dc8910291610dc25f1984610da9565b92610da9565b9181191691161790565b610ddb906105be565b90565b5f1c90565b610dec90610dde565b90565b9190610e05610e00610e0d93610dd2565b610de3565b908354610dad565b9055565b9081549168010000000000000000831015610e415782610e39916001610e3f95018155610d81565b90610def565b565b610304565b610e52610e5791610dde565b6101dc565b90565b610e649054610e46565b90565b610e709061028a565b9052565b90606080610eba93610e8c5f8201515f860190610e67565b610e9e60208201516020860190610e67565b610eb060408201516040860190610e67565b0151910190610e67565b565b634e487b7160e01b5f52602160045260245ffd5b60051115610eda57565b610ebc565b90610ee982610ed0565b565b610ef490610edf565b90565b610f0090610eeb565b9052565b610f39610f4094610f2f60c094989795610f2560e086019a5f8701906105c1565b6020850190610181565b6040830190610e74565b0190610ef7565b565b60209181520190565b90825f9392825e0152565b610f75610f7e602093610f8393610f6c81610d6d565b93848093610f42565b95869101610f4b565b6102fa565b0190565b610f9c9160208201915f818403910152610f56565b90565b9391610fae610fb5925f610c00565b6001610c55565b81610fc8610fc25f610c78565b9161028a565b116111a2575b5050610fda6004610534565b610fe5828290611910565b90610ff06005610534565b92610ffa5f610c94565b8461100d6110075f610cb0565b9161017e565b11611174575b61101c5f610c94565b928261103061102a5f610cb0565b9161017e565b1161113f575b61109490611065836110568761104a610161565b94859360208501610d3c565b60208201810382520382610318565b61107761107182610d6d565b91610d67565b209261108d6110866005610d71565b8590610e11565b6006610c55565b84909192936110a36006610e5a565b90946110f860016110e66110e06110da7f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd797610c36565b97610dd2565b97610dd2565b976110ef610161565b94859485610f04565b0390a461113a6111287ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c2092610c36565b92611131610161565b91829182610f87565b0390a2565b925061109461116c61116660046111608661115a6001610ccf565b90610cff565b90610541565b90610584565b939050611036565b5061119d61119760056111918761118b6001610ccf565b90610cff565b90610541565b90610584565b611013565b6111b06111b7926002610c00565b6003610c55565b5f80610fce565b6111c6610ba5565b506111d16005610534565b90565b6111e86111e36111ed9261017e565b61065f565b61028a565b90565b60f81b90565b6111ff906111f0565b90565b61120e6112139161077c565b6111f6565b9052565b60601b90565b61122690611217565b90565b6112329061121d565b90565b611241611246916107b0565b611229565b9052565b60c01b90565b6112599061124a565b90565b61126861126d9161028a565b611250565b9052565b90565b6112806112859161017e565b611271565b9052565b946112da60086020999895966112d2828c996112ca60146112e29a6112c26112ea9f8f9c906112ba81600193611202565b018092611235565b01809261125c565b01809261125c565b018092611274565b018092611274565b018092610d27565b0190565b60208161130061130893839695610d27565b018092610d27565b0190565b61132061131b611325926107a5565b61065f565b6107a5565b90565b6113319061130c565b90565b61133d90611328565b90565b611349906107b0565b9052565b6113569061077c565b9052565b61136390610cb0565b9052565b91946113af6113b9929897956113a560a09661139b6113c09a61139160c08a019e5f8b0190611340565b602089019061134d565b6040870190611340565b60608501906105c1565b608083019061135a565b0190610297565b565b906113cd6004610534565b91836113e16113db82610d6d565b91610d67565b20918161142e829161141f6113f5436111d4565b6113fe426111d4565b896114085f610cb0565b918a93611413610161565b98899760208901611289565b60208201810382520382610318565b61144061143a82610d6d565b91610d67565b209061144b5f610c94565b918561145f6114595f610cb0565b9161017e565b11611560575b6114b7906114736004610d71565b9061149e8561148f611483610161565b938492602084016112ee565b60208201810382520382610318565b6114b06114aa82610d6d565b91610d67565b2090610e11565b8491926115196114c630611334565b9192955f6114d3426111d4565b916115076115017f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe198610c36565b98610dd2565b98611510610161565b96879687611367565b0390a361155b6115497fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b92610c36565b92611552610161565b91829182610f87565b0390a2565b91506114b761158d61158760046115818961157b6001610ccf565b90610cff565b90610541565b90610584565b929050611465565b5f90565b6115a56115aa91610dde565b610249565b90565b6115b79054611599565b90565b6115c2611595565b506115cb610ba5565b506115d4611595565b506115dd610ba5565b506115e75f6115ad565b906115f26001610e5a565b916115fd60026115ad565b916116086003610e5a565b9193929190565b5f90565b61162a6116309161162261160f565b506005610541565b90610584565b90565b61164761164261164c926106cc565b61065f565b61077c565b90565b611659600c611633565b90565b60148161166f6116779360209695611235565b018092611274565b0190565b906116b992916116b461168c61164f565b91926116a5611699610161565b9586926020840161165c565b60208201810382520384610318565b6113c2565b565b6116c3610ba5565b506116ce6004610534565b90565b6116db6080610341565b90565b5f90565b6116ea6116d1565b906020808080856116f96116de565b8152016117046116de565b81520161170f6116de565b81520161171a6116de565b81525050565b6117286116e2565b90565b90565b61174261173d6117479261172b565b61065f565b61017e565b90565b611754602861172e565b90565b61176661176c9193929361017e565b9261017e565b820180921161177757565b610ceb565b61179061178b6117959261028a565b61065f565b61017e565b90565b6117a19061177c565b9052565b9160206117c69294936117bf60408201965f830190610181565b0190611798565b565b6117d46117da9161028a565b9161028a565b90039067ffffffffffffffff82116117ee57565b610ceb565b906117fd9061028a565b9052565b61180d6118139161028a565b9161028a565b019067ffffffffffffffff821161182657565b610ceb565b611835905161028a565b90565b600861187a9461186a828099989596611862828761185a61187299839c61125c565b01809261125c565b01809261125c565b01809261125c565b01809261125c565b0190565b634e487b7160e01b5f52600160045260245ffd5b1561189957565b61187e565b905090565b6118c86118bf926020926118b681610d6d565b9485809361189e565b93849101610f4b565b0190565b6118da906118e093926118a3565b906118a3565b90565b611902929161190e916118f4610161565b9485926020840192836118cc565b90810382520383610318565b565b91909161191b61160f565b50611924611720565b5061193f61193061174a565b61193983610d6d565b90611757565b8061195961195361194e610ad9565b61177c565b9161017e565b11611af85750611a8c9061196b611720565b934261198661198061197b6108f0565b61177c565b9161017e565b11611ace575b6119b26119a961199b426111d4565b6119a361067e565b90611801565b602087016117f3565b436119cc6119c66119c1610883565b61177c565b9161017e565b11611aa3575b6119f86119ef6119e1436111d4565b6119e96106eb565b90611801565b606087016117f3565b611a5c611a065f870161182b565b611a4d611a156020890161182b565b93611a2260408a0161182b565b90611a38611a3260608c0161182b565b916111d4565b91611a41610161565b96879560208701611838565b60208201810382520382610318565b611a87611a6882610d6d565b611a81611a7b611a7661174a565b61017e565b9161017e565b14611892565b6118e3565b611a9e611a9882610d6d565b91610d67565b209190565b611ac9611ac0611ab2436111d4565b611aba610883565b906117c8565b604087016117f3565b6119d2565b611af3611aeb611add426111d4565b611ae56108f0565b906117c8565b5f87016117f3565b61198c565b611b00610ad9565b90611b1b5f928392634634691b60e01b8452600484016117a5565b0390fd
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x0B\xA1V[a\0\x1D_5a\x01[V[\x80b\x84\x12\x0C\x14a\x01VW\x80c\x04\xF1\xC8T\x14a\x01QW\x80c\x05m\xAA\xA6\x14a\x01LW\x80c\x06\x1D\x12\xC0\x14a\x01GW\x80c\x06\xF10V\x14a\x01BW\x80c\x16\xBFUy\x14a\x01=W\x80c\x18\xDB9@\x14a\x018W\x80c/\x1E\xC5\xE9\x14a\x013W\x80cO5\x9A7\x14a\x01.W\x80c\x7F\xA3\xA4\x0E\x14a\x01)W\x80c\xA7\xB5\x1D\x19\x14a\x01$W\x80c\xAD\x9C\x0C.\x14a\x01\x1FW\x80c\xB7R\xA7\xD1\x14a\x01\x1AW\x80c\xD5q\x9D\xC2\x14a\x01\x15W\x80c\xD5\x95L4\x14a\x01\x10W\x80c\xD9\xDDg\xAB\x14a\x01\x0BW\x80c\xE1\xD6j\xFE\x14a\x01\x06W\x80c\xE8\xEB\x1D\xC3\x14a\x01\x01W\x80c\xEC\xA0g\xAD\x14a\0\xFCWc\xFB\xF6\xEA\xA5\x03a\0\x0EWa\x0BlV[a\x0B(V[a\n\xF3V[a\n\x86V[a\n\x17V[a\t\xDEV[a\tkV[a\t\nV[a\x08\x9CV[a\x080V[a\x07GV[a\x07\x03V[a\x06\x97V[a\x06'V[a\x05\xE3V[a\x04\xCDV[a\x04\x96V[a\x02\xB9V[a\x02\x14V[a\x01\xA3V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01yWV[a\x01kV[\x90V[a\x01\x8A\x90a\x01~V[\x90RV[\x91\x90a\x01\xA1\x90_` \x85\x01\x94\x01\x90a\x01\x81V[V[4a\x01\xD3Wa\x01\xB36`\x04a\x01oV[a\x01\xCFa\x01\xBEa\x0B\xA9V[a\x01\xC6a\x01aV[\x91\x82\x91\x82a\x01\x8EV[\x03\x90\xF3[a\x01gV[\x1C\x90V[\x90V[a\x01\xEF\x90`\x08a\x01\xF4\x93\x02a\x01\xD8V[a\x01\xDCV[\x90V[\x90a\x02\x02\x91Ta\x01\xDFV[\x90V[a\x02\x11`\x03_\x90a\x01\xF7V[\x90V[4a\x02DWa\x02$6`\x04a\x01oV[a\x02@a\x02/a\x02\x05V[a\x027a\x01aV[\x91\x82\x91\x82a\x01\x8EV[\x03\x90\xF3[a\x01gV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02f\x90`\x08a\x02k\x93\x02a\x01\xD8V[a\x02IV[\x90V[\x90a\x02y\x91Ta\x02VV[\x90V[a\x02\x87__\x90a\x02nV[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\xA0\x90a\x02\x8AV[\x90RV[\x91\x90a\x02\xB7\x90_` \x85\x01\x94\x01\x90a\x02\x97V[V[4a\x02\xE9Wa\x02\xC96`\x04a\x01oV[a\x02\xE5a\x02\xD4a\x02|V[a\x02\xDCa\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x03\"\x90a\x02\xFAV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03<W`@RV[a\x03\x04V[\x90a\x03Ta\x03Ma\x01aV[\x92\x83a\x03\x18V[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03tWa\x03p` \x91a\x02\xFAV[\x01\x90V[a\x03\x04V[\x90\x82_\x93\x92\x827\x01RV[\x90\x92\x91\x92a\x03\x99a\x03\x94\x82a\x03VV[a\x03AV[\x93\x81\x85R` \x85\x01\x90\x82\x84\x01\x11a\x03\xB5Wa\x03\xB3\x92a\x03yV[V[a\x02\xF6V[\x90\x80`\x1F\x83\x01\x12\x15a\x03\xD8W\x81` a\x03\xD5\x935\x91\x01a\x03\x84V[\x90V[a\x02\xF2V[a\x03\xE6\x81a\x02\x8AV[\x03a\x03\xEDWV[_\x80\xFD[\x90P5\x90a\x03\xFE\x82a\x03\xDDV[V[a\x04\t\x81a\x01~V[\x03a\x04\x10WV[_\x80\xFD[\x90P5\x90a\x04!\x82a\x04\0V[V[\x91\x90`\xA0\x83\x82\x03\x12a\x04\x8CW_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x87W\x81a\x04N\x91\x85\x01a\x03\xBAV[\x92a\x04\\\x82` \x83\x01a\x03\xF1V[\x92a\x04\x84a\x04m\x84`@\x85\x01a\x04\x14V[\x93a\x04{\x81``\x86\x01a\x03\xF1V[\x93`\x80\x01a\x04\x14V[\x90V[a\x02\xEEV[a\x01kV[_\x01\x90V[4a\x04\xC8Wa\x04\xB2a\x04\xA96`\x04a\x04#V[\x93\x92\x90\x92a\x0F\x9FV[a\x04\xBAa\x01aV[\x80a\x04\xC4\x81a\x04\x91V[\x03\x90\xF3[a\x01gV[4a\x04\xFDWa\x04\xDD6`\x04a\x01oV[a\x04\xF9a\x04\xE8a\x11\xBEV[a\x04\xF0a\x01aV[\x91\x82\x91\x82a\x01\x8EV[\x03\x90\xF3[a\x01gV[\x90` \x82\x82\x03\x12a\x05\x1BWa\x05\x18\x91_\x01a\x04\x14V[\x90V[a\x01kV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[T\x90V[_R` _ \x90V[a\x05J\x81a\x054V[\x82\x10\x15a\x05dWa\x05\\`\x01\x91a\x058V[\x91\x02\x01\x90_\x90V[a\x05 V[\x90V[a\x05|\x90`\x08a\x05\x81\x93\x02a\x01\xD8V[a\x05iV[\x90V[\x90a\x05\x8F\x91Ta\x05lV[\x90V[`\x05a\x05\x9D\x81a\x054V[\x82\x10\x15a\x05\xBAWa\x05\xB7\x91a\x05\xB1\x91a\x05AV[\x90a\x05\x84V[\x90V[_\x80\xFD[\x90V[a\x05\xCA\x90a\x05\xBEV[\x90RV[\x91\x90a\x05\xE1\x90_` \x85\x01\x94\x01\x90a\x05\xC1V[V[4a\x06\x13Wa\x06\x0Fa\x05\xFEa\x05\xF96`\x04a\x05\x02V[a\x05\x92V[a\x06\x06a\x01aV[\x91\x82\x91\x82a\x05\xCEV[\x03\x90\xF3[a\x01gV[a\x06$`\x01_\x90a\x01\xF7V[\x90V[4a\x06WWa\x0676`\x04a\x01oV[a\x06Sa\x06Ba\x06\x18V[a\x06Ja\x01aV[\x91\x82\x91\x82a\x01\x8EV[\x03\x90\xF3[a\x01gV[\x90V[\x90V[a\x06va\x06qa\x06{\x92a\x06\\V[a\x06_V[a\x02\x8AV[\x90V[a\x06\x89a\x0E\x10a\x06bV[\x90V[a\x06\x94a\x06~V[\x90V[4a\x06\xC7Wa\x06\xA76`\x04a\x01oV[a\x06\xC3a\x06\xB2a\x06\x8CV[a\x06\xBAa\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[\x90V[a\x06\xE3a\x06\xDEa\x06\xE8\x92a\x06\xCCV[a\x06_V[a\x02\x8AV[\x90V[a\x06\xF5`\x0Ca\x06\xCFV[\x90V[a\x07\0a\x06\xEBV[\x90V[4a\x073Wa\x07\x136`\x04a\x01oV[a\x07/a\x07\x1Ea\x06\xF8V[a\x07&a\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[a\x07D`\x06_\x90a\x01\xF7V[\x90V[4a\x07wWa\x07W6`\x04a\x01oV[a\x07sa\x07ba\x078V[a\x07ja\x01aV[\x91\x82\x91\x82a\x01\x8EV[\x03\x90\xF3[a\x01gV[`\xFF\x16\x90V[a\x07\x8B\x81a\x07|V[\x03a\x07\x92WV[_\x80\xFD[\x90P5\x90a\x07\xA3\x82a\x07\x82V[V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x07\xB9\x90a\x07\xA5V[\x90V[a\x07\xC5\x81a\x07\xB0V[\x03a\x07\xCCWV[_\x80\xFD[\x90P5\x90a\x07\xDD\x82a\x07\xBCV[V[\x91``\x83\x83\x03\x12a\x08+Wa\x07\xF6\x82_\x85\x01a\x07\x96V[\x92a\x08\x04\x83` \x83\x01a\x07\xD0V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08&Wa\x08#\x92\x01a\x03\xBAV[\x90V[a\x02\xEEV[a\x01kV[4a\x08_Wa\x08Ia\x08C6`\x04a\x07\xDFV[\x91a\x13\xC2V[a\x08Qa\x01aV[\x80a\x08[\x81a\x04\x91V[\x03\x90\xF3[a\x01gV[\x90V[a\x08{a\x08va\x08\x80\x92a\x08dV[a\x06_V[a\x02\x8AV[\x90V[a\x08\x8Ea\x1C a\x08gV[\x90V[a\x08\x99a\x08\x83V[\x90V[4a\x08\xCCWa\x08\xAC6`\x04a\x01oV[a\x08\xC8a\x08\xB7a\x08\x91V[a\x08\xBFa\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[\x90V[a\x08\xE8a\x08\xE3a\x08\xED\x92a\x08\xD1V[a\x06_V[a\x02\x8AV[\x90V[a\x08\xFCb\x01Q\x80a\x08\xD4V[\x90V[a\t\x07a\x08\xF0V[\x90V[4a\t:Wa\t\x1A6`\x04a\x01oV[a\t6a\t%a\x08\xFFV[a\t-a\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[`\x04a\tJ\x81a\x054V[\x82\x10\x15a\tgWa\td\x91a\t^\x91a\x05AV[\x90a\x05\x84V[\x90V[_\x80\xFD[4a\t\x9BWa\t\x97a\t\x86a\t\x816`\x04a\x05\x02V[a\t?V[a\t\x8Ea\x01aV[\x91\x82\x91\x82a\x05\xCEV[\x03\x90\xF3[a\x01gV[a\t\xD5a\t\xDC\x94a\t\xCB``\x94\x98\x97\x95a\t\xC1`\x80\x86\x01\x9A_\x87\x01\x90a\x02\x97V[` \x85\x01\x90a\x01\x81V[`@\x83\x01\x90a\x02\x97V[\x01\x90a\x01\x81V[V[4a\n\x12Wa\t\xEE6`\x04a\x01oV[a\n\x0Ea\t\xF9a\x15\xBAV[\x90a\n\x05\x94\x92\x94a\x01aV[\x94\x85\x94\x85a\t\xA0V[\x03\x90\xF3[a\x01gV[4a\nGWa\nCa\n2a\n-6`\x04a\x05\x02V[a\x16\x13V[a\n:a\x01aV[\x91\x82\x91\x82a\x05\xCEV[\x03\x90\xF3[a\x01gV[\x90\x91``\x82\x84\x03\x12a\n\x81Wa\n~a\ng\x84_\x85\x01a\x07\xD0V[\x93a\nu\x81` \x86\x01a\x07\xD0V[\x93`@\x01a\x04\x14V[\x90V[a\x01kV[4a\n\xB5Wa\n\x9Fa\n\x996`\x04a\nLV[\x91a\x16{V[a\n\xA7a\x01aV[\x80a\n\xB1\x81a\x04\x91V[\x03\x90\xF3[a\x01gV[\x90V[a\n\xD1a\n\xCCa\n\xD6\x92a\n\xBAV[a\x06_V[a\x02\x8AV[\x90V[a\n\xE5b\x01\xCC\xCCa\n\xBDV[\x90V[a\n\xF0a\n\xD9V[\x90V[4a\x0B#Wa\x0B\x036`\x04a\x01oV[a\x0B\x1Fa\x0B\x0Ea\n\xE8V[a\x0B\x16a\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[4a\x0BXWa\x0B86`\x04a\x01oV[a\x0BTa\x0BCa\x16\xBBV[a\x0BKa\x01aV[\x91\x82\x91\x82a\x01\x8EV[\x03\x90\xF3[a\x01gV[a\x0Bi`\x02_\x90a\x02nV[\x90V[4a\x0B\x9CWa\x0B|6`\x04a\x01oV[a\x0B\x98a\x0B\x87a\x0B]V[a\x0B\x8Fa\x01aV[\x91\x82\x91\x82a\x02\xA4V[\x03\x90\xF3[a\x01gV[_\x80\xFD[_\x90V[a\x0B\xB1a\x0B\xA5V[Pa\x0B\xBC`\x05a\x054V[\x90V[_\x1B\x90V[\x90a\x0B\xD7g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x0B\xBFV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x0B\xF5a\x0B\xF0a\x0B\xFA\x92a\x02\x8AV[a\x06_V[a\x02\x8AV[\x90V[\x90V[\x90a\x0C\x15a\x0C\x10a\x0C\x1C\x92a\x0B\xE1V[a\x0B\xFDV[\x82Ta\x0B\xC4V[\x90UV[\x90a\x0C,_\x19\x91a\x0B\xBFV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x0CJa\x0CEa\x0CO\x92a\x01~V[a\x06_V[a\x01~V[\x90V[\x90V[\x90a\x0Cja\x0Cea\x0Cq\x92a\x0C6V[a\x0CRV[\x82Ta\x0C V[\x90UV[\x90V[a\x0C\x8Ca\x0C\x87a\x0C\x91\x92a\x0CuV[a\x06_V[a\x02\x8AV[\x90V[a\x0C\xA8a\x0C\xA3a\x0C\xAD\x92a\x0CuV[a\x0B\xBFV[a\x05\xBEV[\x90V[a\x0C\xC4a\x0C\xBFa\x0C\xC9\x92a\x0CuV[a\x06_V[a\x01~V[\x90V[\x90V[a\x0C\xE3a\x0C\xDEa\x0C\xE8\x92a\x0C\xCCV[a\x06_V[a\x01~V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\r\x0Ea\r\x14\x91\x93\x92\x93a\x01~V[\x92a\x01~V[\x82\x03\x91\x82\x11a\r\x1FWV[a\x0C\xEBV[\x90V[a\r3a\r8\x91a\x05\xBEV[a\r$V[\x90RV[` \x93\x92a\r[\x85\x83a\rS\x82\x95a\rc\x97a\r'V[\x01\x80\x92a\r'V[\x01\x80\x92a\r'V[\x01\x90V[` \x01\x90V[Q\x90V[\x90V[_R` _ \x90V[T\x90V[a\r\x8A\x81a\r}V[\x82\x10\x15a\r\xA4Wa\r\x9C`\x01\x91a\rtV[\x91\x02\x01\x90_\x90V[a\x05 V[\x1B\x90V[\x91\x90`\x08a\r\xC8\x91\x02\x91a\r\xC2_\x19\x84a\r\xA9V[\x92a\r\xA9V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\r\xDB\x90a\x05\xBEV[\x90V[_\x1C\x90V[a\r\xEC\x90a\r\xDEV[\x90V[\x91\x90a\x0E\x05a\x0E\0a\x0E\r\x93a\r\xD2V[a\r\xE3V[\x90\x83Ta\r\xADV[\x90UV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x0EAW\x82a\x0E9\x91`\x01a\x0E?\x95\x01\x81Ua\r\x81V[\x90a\r\xEFV[V[a\x03\x04V[a\x0ERa\x0EW\x91a\r\xDEV[a\x01\xDCV[\x90V[a\x0Ed\x90Ta\x0EFV[\x90V[a\x0Ep\x90a\x02\x8AV[\x90RV[\x90``\x80a\x0E\xBA\x93a\x0E\x8C_\x82\x01Q_\x86\x01\x90a\x0EgV[a\x0E\x9E` \x82\x01Q` \x86\x01\x90a\x0EgV[a\x0E\xB0`@\x82\x01Q`@\x86\x01\x90a\x0EgV[\x01Q\x91\x01\x90a\x0EgV[V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x05\x11\x15a\x0E\xDAWV[a\x0E\xBCV[\x90a\x0E\xE9\x82a\x0E\xD0V[V[a\x0E\xF4\x90a\x0E\xDFV[\x90V[a\x0F\0\x90a\x0E\xEBV[\x90RV[a\x0F9a\x0F@\x94a\x0F/`\xC0\x94\x98\x97\x95a\x0F%`\xE0\x86\x01\x9A_\x87\x01\x90a\x05\xC1V[` \x85\x01\x90a\x01\x81V[`@\x83\x01\x90a\x0EtV[\x01\x90a\x0E\xF7V[V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[a\x0Fua\x0F~` \x93a\x0F\x83\x93a\x0Fl\x81a\rmV[\x93\x84\x80\x93a\x0FBV[\x95\x86\x91\x01a\x0FKV[a\x02\xFAV[\x01\x90V[a\x0F\x9C\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x0FVV[\x90V[\x93\x91a\x0F\xAEa\x0F\xB5\x92_a\x0C\0V[`\x01a\x0CUV[\x81a\x0F\xC8a\x0F\xC2_a\x0CxV[\x91a\x02\x8AV[\x11a\x11\xA2W[PPa\x0F\xDA`\x04a\x054V[a\x0F\xE5\x82\x82\x90a\x19\x10V[\x90a\x0F\xF0`\x05a\x054V[\x92a\x0F\xFA_a\x0C\x94V[\x84a\x10\ra\x10\x07_a\x0C\xB0V[\x91a\x01~V[\x11a\x11tW[a\x10\x1C_a\x0C\x94V[\x92\x82a\x100a\x10*_a\x0C\xB0V[\x91a\x01~V[\x11a\x11?W[a\x10\x94\x90a\x10e\x83a\x10V\x87a\x10Ja\x01aV[\x94\x85\x93` \x85\x01a\r<V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\x18V[a\x10wa\x10q\x82a\rmV[\x91a\rgV[ \x92a\x10\x8Da\x10\x86`\x05a\rqV[\x85\x90a\x0E\x11V[`\x06a\x0CUV[\x84\x90\x91\x92\x93a\x10\xA3`\x06a\x0EZV[\x90\x94a\x10\xF8`\x01a\x10\xE6a\x10\xE0a\x10\xDA\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x97a\x0C6V[\x97a\r\xD2V[\x97a\r\xD2V[\x97a\x10\xEFa\x01aV[\x94\x85\x94\x85a\x0F\x04V[\x03\x90\xA4a\x11:a\x11(\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x92a\x0C6V[\x92a\x111a\x01aV[\x91\x82\x91\x82a\x0F\x87V[\x03\x90\xA2V[\x92Pa\x10\x94a\x11la\x11f`\x04a\x11`\x86a\x11Z`\x01a\x0C\xCFV[\x90a\x0C\xFFV[\x90a\x05AV[\x90a\x05\x84V[\x93\x90Pa\x106V[Pa\x11\x9Da\x11\x97`\x05a\x11\x91\x87a\x11\x8B`\x01a\x0C\xCFV[\x90a\x0C\xFFV[\x90a\x05AV[\x90a\x05\x84V[a\x10\x13V[a\x11\xB0a\x11\xB7\x92`\x02a\x0C\0V[`\x03a\x0CUV[_\x80a\x0F\xCEV[a\x11\xC6a\x0B\xA5V[Pa\x11\xD1`\x05a\x054V[\x90V[a\x11\xE8a\x11\xE3a\x11\xED\x92a\x01~V[a\x06_V[a\x02\x8AV[\x90V[`\xF8\x1B\x90V[a\x11\xFF\x90a\x11\xF0V[\x90V[a\x12\x0Ea\x12\x13\x91a\x07|V[a\x11\xF6V[\x90RV[``\x1B\x90V[a\x12&\x90a\x12\x17V[\x90V[a\x122\x90a\x12\x1DV[\x90V[a\x12Aa\x12F\x91a\x07\xB0V[a\x12)V[\x90RV[`\xC0\x1B\x90V[a\x12Y\x90a\x12JV[\x90V[a\x12ha\x12m\x91a\x02\x8AV[a\x12PV[\x90RV[\x90V[a\x12\x80a\x12\x85\x91a\x01~V[a\x12qV[\x90RV[\x94a\x12\xDA`\x08` \x99\x98\x95\x96a\x12\xD2\x82\x8C\x99a\x12\xCA`\x14a\x12\xE2\x9Aa\x12\xC2a\x12\xEA\x9F\x8F\x9C\x90a\x12\xBA\x81`\x01\x93a\x12\x02V[\x01\x80\x92a\x125V[\x01\x80\x92a\x12\\V[\x01\x80\x92a\x12\\V[\x01\x80\x92a\x12tV[\x01\x80\x92a\x12tV[\x01\x80\x92a\r'V[\x01\x90V[` \x81a\x13\0a\x13\x08\x93\x83\x96\x95a\r'V[\x01\x80\x92a\r'V[\x01\x90V[a\x13 a\x13\x1Ba\x13%\x92a\x07\xA5V[a\x06_V[a\x07\xA5V[\x90V[a\x131\x90a\x13\x0CV[\x90V[a\x13=\x90a\x13(V[\x90V[a\x13I\x90a\x07\xB0V[\x90RV[a\x13V\x90a\x07|V[\x90RV[a\x13c\x90a\x0C\xB0V[\x90RV[\x91\x94a\x13\xAFa\x13\xB9\x92\x98\x97\x95a\x13\xA5`\xA0\x96a\x13\x9Ba\x13\xC0\x9Aa\x13\x91`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x13@V[` \x89\x01\x90a\x13MV[`@\x87\x01\x90a\x13@V[``\x85\x01\x90a\x05\xC1V[`\x80\x83\x01\x90a\x13ZV[\x01\x90a\x02\x97V[V[\x90a\x13\xCD`\x04a\x054V[\x91\x83a\x13\xE1a\x13\xDB\x82a\rmV[\x91a\rgV[ \x91\x81a\x14.\x82\x91a\x14\x1Fa\x13\xF5Ca\x11\xD4V[a\x13\xFEBa\x11\xD4V[\x89a\x14\x08_a\x0C\xB0V[\x91\x8A\x93a\x14\x13a\x01aV[\x98\x89\x97` \x89\x01a\x12\x89V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\x18V[a\x14@a\x14:\x82a\rmV[\x91a\rgV[ \x90a\x14K_a\x0C\x94V[\x91\x85a\x14_a\x14Y_a\x0C\xB0V[\x91a\x01~V[\x11a\x15`W[a\x14\xB7\x90a\x14s`\x04a\rqV[\x90a\x14\x9E\x85a\x14\x8Fa\x14\x83a\x01aV[\x93\x84\x92` \x84\x01a\x12\xEEV[` \x82\x01\x81\x03\x82R\x03\x82a\x03\x18V[a\x14\xB0a\x14\xAA\x82a\rmV[\x91a\rgV[ \x90a\x0E\x11V[\x84\x91\x92a\x15\x19a\x14\xC60a\x134V[\x91\x92\x95_a\x14\xD3Ba\x11\xD4V[\x91a\x15\x07a\x15\x01\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE1\x98a\x0C6V[\x98a\r\xD2V[\x98a\x15\x10a\x01aV[\x96\x87\x96\x87a\x13gV[\x03\x90\xA3a\x15[a\x15I\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x92a\x0C6V[\x92a\x15Ra\x01aV[\x91\x82\x91\x82a\x0F\x87V[\x03\x90\xA2V[\x91Pa\x14\xB7a\x15\x8Da\x15\x87`\x04a\x15\x81\x89a\x15{`\x01a\x0C\xCFV[\x90a\x0C\xFFV[\x90a\x05AV[\x90a\x05\x84V[\x92\x90Pa\x14eV[_\x90V[a\x15\xA5a\x15\xAA\x91a\r\xDEV[a\x02IV[\x90V[a\x15\xB7\x90Ta\x15\x99V[\x90V[a\x15\xC2a\x15\x95V[Pa\x15\xCBa\x0B\xA5V[Pa\x15\xD4a\x15\x95V[Pa\x15\xDDa\x0B\xA5V[Pa\x15\xE7_a\x15\xADV[\x90a\x15\xF2`\x01a\x0EZV[\x91a\x15\xFD`\x02a\x15\xADV[\x91a\x16\x08`\x03a\x0EZV[\x91\x93\x92\x91\x90V[_\x90V[a\x16*a\x160\x91a\x16\"a\x16\x0FV[P`\x05a\x05AV[\x90a\x05\x84V[\x90V[a\x16Ga\x16Ba\x16L\x92a\x06\xCCV[a\x06_V[a\x07|V[\x90V[a\x16Y`\x0Ca\x163V[\x90V[`\x14\x81a\x16oa\x16w\x93` \x96\x95a\x125V[\x01\x80\x92a\x12tV[\x01\x90V[\x90a\x16\xB9\x92\x91a\x16\xB4a\x16\x8Ca\x16OV[\x91\x92a\x16\xA5a\x16\x99a\x01aV[\x95\x86\x92` \x84\x01a\x16\\V[` \x82\x01\x81\x03\x82R\x03\x84a\x03\x18V[a\x13\xC2V[V[a\x16\xC3a\x0B\xA5V[Pa\x16\xCE`\x04a\x054V[\x90V[a\x16\xDB`\x80a\x03AV[\x90V[_\x90V[a\x16\xEAa\x16\xD1V[\x90` \x80\x80\x80\x85a\x16\xF9a\x16\xDEV[\x81R\x01a\x17\x04a\x16\xDEV[\x81R\x01a\x17\x0Fa\x16\xDEV[\x81R\x01a\x17\x1Aa\x16\xDEV[\x81RPPV[a\x17(a\x16\xE2V[\x90V[\x90V[a\x17Ba\x17=a\x17G\x92a\x17+V[a\x06_V[a\x01~V[\x90V[a\x17T`(a\x17.V[\x90V[a\x17fa\x17l\x91\x93\x92\x93a\x01~V[\x92a\x01~V[\x82\x01\x80\x92\x11a\x17wWV[a\x0C\xEBV[a\x17\x90a\x17\x8Ba\x17\x95\x92a\x02\x8AV[a\x06_V[a\x01~V[\x90V[a\x17\xA1\x90a\x17|V[\x90RV[\x91` a\x17\xC6\x92\x94\x93a\x17\xBF`@\x82\x01\x96_\x83\x01\x90a\x01\x81V[\x01\x90a\x17\x98V[V[a\x17\xD4a\x17\xDA\x91a\x02\x8AV[\x91a\x02\x8AV[\x90\x03\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x17\xEEWV[a\x0C\xEBV[\x90a\x17\xFD\x90a\x02\x8AV[\x90RV[a\x18\ra\x18\x13\x91a\x02\x8AV[\x91a\x02\x8AV[\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x18&WV[a\x0C\xEBV[a\x185\x90Qa\x02\x8AV[\x90V[`\x08a\x18z\x94a\x18j\x82\x80\x99\x98\x95\x96a\x18b\x82\x87a\x18Za\x18r\x99\x83\x9Ca\x12\\V[\x01\x80\x92a\x12\\V[\x01\x80\x92a\x12\\V[\x01\x80\x92a\x12\\V[\x01\x80\x92a\x12\\V[\x01\x90V[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[\x15a\x18\x99WV[a\x18~V[\x90P\x90V[a\x18\xC8a\x18\xBF\x92` \x92a\x18\xB6\x81a\rmV[\x94\x85\x80\x93a\x18\x9EV[\x93\x84\x91\x01a\x0FKV[\x01\x90V[a\x18\xDA\x90a\x18\xE0\x93\x92a\x18\xA3V[\x90a\x18\xA3V[\x90V[a\x19\x02\x92\x91a\x19\x0E\x91a\x18\xF4a\x01aV[\x94\x85\x92` \x84\x01\x92\x83a\x18\xCCV[\x90\x81\x03\x82R\x03\x83a\x03\x18V[V[\x91\x90\x91a\x19\x1Ba\x16\x0FV[Pa\x19$a\x17 V[Pa\x19?a\x190a\x17JV[a\x199\x83a\rmV[\x90a\x17WV[\x80a\x19Ya\x19Sa\x19Na\n\xD9V[a\x17|V[\x91a\x01~V[\x11a\x1A\xF8WPa\x1A\x8C\x90a\x19ka\x17 V[\x93Ba\x19\x86a\x19\x80a\x19{a\x08\xF0V[a\x17|V[\x91a\x01~V[\x11a\x1A\xCEW[a\x19\xB2a\x19\xA9a\x19\x9BBa\x11\xD4V[a\x19\xA3a\x06~V[\x90a\x18\x01V[` \x87\x01a\x17\xF3V[Ca\x19\xCCa\x19\xC6a\x19\xC1a\x08\x83V[a\x17|V[\x91a\x01~V[\x11a\x1A\xA3W[a\x19\xF8a\x19\xEFa\x19\xE1Ca\x11\xD4V[a\x19\xE9a\x06\xEBV[\x90a\x18\x01V[``\x87\x01a\x17\xF3V[a\x1A\\a\x1A\x06_\x87\x01a\x18+V[a\x1AMa\x1A\x15` \x89\x01a\x18+V[\x93a\x1A\"`@\x8A\x01a\x18+V[\x90a\x1A8a\x1A2``\x8C\x01a\x18+V[\x91a\x11\xD4V[\x91a\x1AAa\x01aV[\x96\x87\x95` \x87\x01a\x188V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\x18V[a\x1A\x87a\x1Ah\x82a\rmV[a\x1A\x81a\x1A{a\x1Ava\x17JV[a\x01~V[\x91a\x01~V[\x14a\x18\x92V[a\x18\xE3V[a\x1A\x9Ea\x1A\x98\x82a\rmV[\x91a\rgV[ \x91\x90V[a\x1A\xC9a\x1A\xC0a\x1A\xB2Ca\x11\xD4V[a\x1A\xBAa\x08\x83V[\x90a\x17\xC8V[`@\x87\x01a\x17\xF3V[a\x19\xD2V[a\x1A\xF3a\x1A\xEBa\x1A\xDDBa\x11\xD4V[a\x1A\xE5a\x08\xF0V[\x90a\x17\xC8V[_\x87\x01a\x17\xF3V[a\x19\x8CV[a\x1B\0a\n\xD9V[\x90a\x1B\x1B_\x92\x83\x92cF4i\x1B`\xE0\x1B\x84R`\x04\x84\x01a\x17\xA5V[\x03\x90\xFD",
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
