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
        pub minTimestamp: u64,
        pub maxTimestamp: u64,
        pub minBlockNumber: u64,
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
    ///0x60806040525f80546001600160401b031990811682556001829055600280549091169055600355348015610031575f5ffd5b506040516114a13803806114a183398101604081905261005091610657565b5f81511161009a5760405162461bcd60e51b8152602060048201526012602482015271454d5054595f434841494e5f434f4e46494760701b60448201526064015b60405180910390fd5b6100cd600b5f8460015f866040516020016100b89493929190610729565b60408051601f198184030181529190526100fd565b6040805180820190915260068152640b0080020360d01b60208201526100f6905f8080806102db565b50506108c4565b60045481516020808401919091206040517fff0000000000000000000000000000000000000000000000000000000000000060f888901b16928101929092526001600160601b0319606086901b1660218301526001600160c01b03194360c090811b8216603585015242901b16603d830152604582018390525f6065830181905260858301829052909160a50160408051601f19818403018152919052805160209091012090505f83156101d45760046101b860018661076d565b815481106101c8576101c8610786565b905f5260205f20015490505b6040805160208082018490528183018590528251808303840181526060830180855281519190920120600480546001810182555f9182527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b01919091553090915260ff8a1660808301526001600160a01b03891660a083015260c0820186905260e0820152426001600160401b03166101008201529051829186917f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1918190036101200190a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b866040516102ca919061079a565b60405180910390a250505050505050565b5f80546001600160401b0319166001600160401b0386811691909117909155600184905582161561032757600280546001600160401b0319166001600160401b03841617905560038190555b6004545f806103368884610491565b60055491935091505f811561036e57600561035260018461076d565b8154811061036257610362610786565b905f5260205f20015490505b5f851561039e57600461038260018861076d565b8154811061039257610392610786565b905f5260205f20015490505b604080516020808201859052818301889052606080830185905283518084039091018152608090920192839052815191012060058054600180820183555f929092527f036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db001829055600689905590918291859187917f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7916104439188918e918d916107cf565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d60405161047b919061079a565b60405180910390a2505050505050505050505050565b604080516080810182525f80825260208201819052918101829052606081018290525f845160286104c29190610843565b90506201cccc8111156104f457604051634634691b60e01b8152600481018290526201cccc6024820152604401610091565b604080516080810182525f8082526020820181905291810182905260608101919091526201518042111561053b5761052f6201518042610856565b6001600160401b031681525b610547610e1042610875565b6001600160401b03166020820152611c2043111561057a5761056b611c2043610856565b6001600160401b031660408201525b610585600c43610875565b6001600160401b0381166060830152815160208084015160408086015181516001600160c01b031960c096871b81169582019590955292851b84166028840152841b8316603083015293831b821660388201529188901b16918101919091525f906048016040516020818303038152906040529050602881511461060b5761060b610894565b808760405160200161061e9291906108a8565b6040516020818303038152906040528051906020012082945094505050509250929050565b634e487b7160e01b5f52604160045260245ffd5b5f5f60408385031215610668575f5ffd5b825160208401519092506001600160401b03811115610685575f5ffd5b8301601f81018513610695575f5ffd5b80516001600160401b038111156106ae576106ae610643565b604051601f8201601f19908116603f011681016001600160401b03811182821017156106dc576106dc610643565b6040528181528282016020018710156106f3575f5ffd5b8160208401602083015e5f602083830101528093505050509250929050565b5f81518060208401855e5f93019283525090919050565b84815260ff60f81b8460f81b1660208201528260218201525f61074f6041830184610712565b9695505050505050565b634e487b7160e01b5f52601160045260245ffd5b8181038181111561078057610780610759565b92915050565b634e487b7160e01b5f52603260045260245ffd5b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b848152602080820185905283516001600160401b039081166040808501919091529185015181166060808501919091529185015181166080840152908401511660a082015260e081016004831061083457634e487b7160e01b5f52602160045260245ffd5b8260c083015295945050505050565b8082018082111561078057610780610759565b6001600160401b03828116828216039081111561078057610780610759565b6001600160401b03818116838216019081111561078057610780610759565b634e487b7160e01b5f52600160045260245ffd5b5f6108bc6108b68386610712565b84610712565b949350505050565b610bd0806108d15f395ff3fe608060405234801561000f575f5ffd5b5060043610610126575f3560e01c8063a7b51d19116100a9578063d9dd67ab1161006e578063d9dd67ab14610237578063e1d66afe1461024a578063e8eb1dc31461025d578063eca067ad14610267578063fbf6eaa51461026f575f5ffd5b8063a7b51d19146101bf578063ad9c0c2e146101d2578063b752a7d1146101db578063d5719dc2146101e5578063d5954c34146101f8575f5ffd5b806316bf5579116100ef57806316bf55791461018957806318db39401461019c5780632f1ec5e9146101a55780634f359a37146101ae5780637fa3a40e146101b6575f5ffd5b806284120c1461012a57806304f1c85414610141578063056daaa61461014a578063061d12c01461017457806306f130561461012a575b5f5ffd5b6005545b6040519081526020015b60405180910390f35b61012e60035481565b5f5461015c906001600160401b031681565b6040516001600160401b039091168152602001610138565b61018761018236600461090d565b610282565b005b61012e610197366004610977565b61043a565b61012e60015481565b61015c610e1081565b61015c600c81565b61012e60065481565b6101876101cd3660046109a4565b610459565b61015c611c2081565b61015c6201518081565b61012e6101f3366004610977565b610624565b5f54600154600254600354604080516001600160401b039586168152602081019490945293909116928201929092526060810191909152608001610138565b61012e610245366004610977565b610633565b610187610258366004610a04565b610657565b61015c6201cccc81565b60045461012e565b60025461015c906001600160401b031681565b5f805467ffffffffffffffff19166001600160401b038681169190911790915560018490558216156102d0576002805467ffffffffffffffff19166001600160401b03841617905560038190555b6004545f806102df888461069f565b60055491935091505f81156103175760056102fb600184610a52565b8154811061030b5761030b610a6b565b905f5260205f20015490505b5f851561034757600461032b600188610a52565b8154811061033b5761033b610a6b565b905f5260205f20015490505b604080516020808201859052818301889052606080830185905283518084039091018152608090920192839052815191012060058054600180820183555f929092527f036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db001829055600689905590918291859187917f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7916103ec9188918e918d91610a7f565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d6040516104249190610b03565b60405180910390a2505050505050505050505050565b60058181548110610449575f80fd5b5f91825260209091200154905081565b60045481516020808401919091206040516001600160f81b031960f888901b16928101929092526bffffffffffffffffffffffff19606086901b1660218301526001600160c01b03194360c090811b8216603585015242901b16603d830152604582018390525f6065830181905260858301829052909160a50160408051601f19818403018152919052805160209091012090505f831561051d576004610501600186610a52565b8154811061051157610511610a6b565b905f5260205f20015490505b6040805160208082018490528183018590528251808303840181526060830180855281519190920120600480546001810182555f9182527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b01919091553090915260ff8a1660808301526001600160a01b03891660a083015260c0820186905260e0820152426001600160401b03166101008201529051829186917f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1918190036101200190a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b866040516106139190610b03565b60405180910390a250505050505050565b60048181548110610449575f80fd5b5f6005828154811061064757610647610a6b565b905f5260205f2001549050919050565b6040516bffffffffffffffffffffffff19606084901b1660208201526034810182905261069a90600c908590605401604051602081830303815290604052610459565b505050565b604080516080810182525f80825260208201819052918101829052606081018290525f845160286106d09190610b38565b90506201cccc81111561070657604051634634691b60e01b8152600481018290526201cccc602482015260440160405180910390fd5b604080516080810182525f8082526020820181905291810182905260608101919091526201518042111561074d576107416201518042610b4b565b6001600160401b031681525b610759610e1042610b6a565b6001600160401b03166020820152611c2043111561078c5761077d611c2043610b4b565b6001600160401b031660408201525b610797600c43610b6a565b6001600160401b0381166060830152815160208084015160408086015181516001600160c01b031960c096871b81169582019590955292851b84166028840152841b8316603083015293831b821660388201529188901b16918101919091525f906048016040516020818303038152906040529050602881511461081d5761081d610b89565b8087604051602001610830929190610bb4565b6040516020818303038152906040528051906020012082945094505050509250929050565b634e487b7160e01b5f52604160045260245ffd5b5f82601f830112610878575f5ffd5b81356001600160401b0381111561089157610891610855565b604051601f8201601f19908116603f011681016001600160401b03811182821017156108bf576108bf610855565b6040528181528382016020018510156108d6575f5ffd5b816020850160208301375f918101602001919091529392505050565b80356001600160401b0381168114610908575f5ffd5b919050565b5f5f5f5f5f60a08688031215610921575f5ffd5b85356001600160401b03811115610936575f5ffd5b61094288828901610869565b955050610951602087016108f2565b935060408601359250610966606087016108f2565b949793965091946080013592915050565b5f60208284031215610987575f5ffd5b5035919050565b80356001600160a01b0381168114610908575f5ffd5b5f5f5f606084860312156109b6575f5ffd5b833560ff811681146109c6575f5ffd5b92506109d46020850161098e565b915060408401356001600160401b038111156109ee575f5ffd5b6109fa86828701610869565b9150509250925092565b5f5f5f60608486031215610a16575f5ffd5b610a1f8461098e565b9250610a2d6020850161098e565b929592945050506040919091013590565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610a6557610a65610a3e565b92915050565b634e487b7160e01b5f52603260045260245ffd5b5f60e0820190508582528460208301526001600160401b0384511660408301526001600160401b0360208501511660608301526001600160401b0360408501511660808301526001600160401b0360608501511660a083015260048310610af457634e487b7160e01b5f52602160045260245ffd5b8260c083015295945050505050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80820180821115610a6557610a65610a3e565b6001600160401b038281168282160390811115610a6557610a65610a3e565b6001600160401b038181168382160190811115610a6557610a65610a3e565b634e487b7160e01b5f52600160045260245ffd5b5f81518060208401855e5f93019283525090919050565b5f610bc8610bc28386610b9d565b84610b9d565b94935050505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R_\x80T`\x01`\x01`@\x1B\x03\x19\x90\x81\x16\x82U`\x01\x82\x90U`\x02\x80T\x90\x91\x16\x90U`\x03U4\x80\x15a\x001W__\xFD[P`@Qa\x14\xA18\x03\x80a\x14\xA1\x839\x81\x01`@\x81\x90Ra\0P\x91a\x06WV[_\x81Q\x11a\0\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqEMPTY_CHAIN_CONFIG`p\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\0\xCD`\x0B_\x84`\x01_\x86`@Q` \x01a\0\xB8\x94\x93\x92\x91\x90a\x07)V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\0\xFDV[`@\x80Q\x80\x82\x01\x90\x91R`\x06\x81Rd\x0B\0\x80\x02\x03`\xD0\x1B` \x82\x01Ra\0\xF6\x90_\x80\x80\x80a\x02\xDBV[PPa\x08\xC4V[`\x04T\x81Q` \x80\x84\x01\x91\x90\x91 `@Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF8\x88\x90\x1B\x16\x92\x81\x01\x92\x90\x92R`\x01`\x01``\x1B\x03\x19``\x86\x90\x1B\x16`!\x83\x01R`\x01`\x01`\xC0\x1B\x03\x19C`\xC0\x90\x81\x1B\x82\x16`5\x85\x01RB\x90\x1B\x16`=\x83\x01R`E\x82\x01\x83\x90R_`e\x83\x01\x81\x90R`\x85\x83\x01\x82\x90R\x90\x91`\xA5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P_\x83\x15a\x01\xD4W`\x04a\x01\xB8`\x01\x86a\x07mV[\x81T\x81\x10a\x01\xC8Wa\x01\xC8a\x07\x86V[\x90_R` _ \x01T\x90P[`@\x80Q` \x80\x82\x01\x84\x90R\x81\x83\x01\x85\x90R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x80\x85R\x81Q\x91\x90\x92\x01 `\x04\x80T`\x01\x81\x01\x82U_\x91\x82R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01\x91\x90\x91U0\x90\x91R`\xFF\x8A\x16`\x80\x83\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16`\xA0\x83\x01R`\xC0\x82\x01\x86\x90R`\xE0\x82\x01RB`\x01`\x01`@\x1B\x03\x16a\x01\0\x82\x01R\x90Q\x82\x91\x86\x91\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE1\x91\x81\x90\x03a\x01 \x01\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x02\xCA\x91\x90a\x07\x9AV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[_\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x86\x81\x16\x91\x90\x91\x17\x90\x91U`\x01\x84\x90U\x82\x16\x15a\x03'W`\x02\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x84\x16\x17\x90U`\x03\x81\x90U[`\x04T_\x80a\x036\x88\x84a\x04\x91V[`\x05T\x91\x93P\x91P_\x81\x15a\x03nW`\x05a\x03R`\x01\x84a\x07mV[\x81T\x81\x10a\x03bWa\x03ba\x07\x86V[\x90_R` _ \x01T\x90P[_\x85\x15a\x03\x9EW`\x04a\x03\x82`\x01\x88a\x07mV[\x81T\x81\x10a\x03\x92Wa\x03\x92a\x07\x86V[\x90_R` _ \x01T\x90P[`@\x80Q` \x80\x82\x01\x85\x90R\x81\x83\x01\x88\x90R``\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x92\x83\x90R\x81Q\x91\x01 `\x05\x80T`\x01\x80\x82\x01\x83U_\x92\x90\x92R\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB0\x01\x82\x90U`\x06\x89\x90U\x90\x91\x82\x91\x85\x91\x87\x91\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x91a\x04C\x91\x88\x91\x8E\x91\x8D\x91a\x07\xCFV[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D`@Qa\x04{\x91\x90a\x07\x9AV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R_\x84Q`(a\x04\xC2\x91\x90a\x08CV[\x90Pb\x01\xCC\xCC\x81\x11\x15a\x04\xF4W`@QcF4i\x1B`\xE0\x1B\x81R`\x04\x81\x01\x82\x90Rb\x01\xCC\xCC`$\x82\x01R`D\x01a\0\x91V[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Rb\x01Q\x80B\x11\x15a\x05;Wa\x05/b\x01Q\x80Ba\x08VV[`\x01`\x01`@\x1B\x03\x16\x81R[a\x05Ga\x0E\x10Ba\x08uV[`\x01`\x01`@\x1B\x03\x16` \x82\x01Ra\x1C C\x11\x15a\x05zWa\x05ka\x1C Ca\x08VV[`\x01`\x01`@\x1B\x03\x16`@\x82\x01R[a\x05\x85`\x0CCa\x08uV[`\x01`\x01`@\x1B\x03\x81\x16``\x83\x01R\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xC0\x1B\x03\x19`\xC0\x96\x87\x1B\x81\x16\x95\x82\x01\x95\x90\x95R\x92\x85\x1B\x84\x16`(\x84\x01R\x84\x1B\x83\x16`0\x83\x01R\x93\x83\x1B\x82\x16`8\x82\x01R\x91\x88\x90\x1B\x16\x91\x81\x01\x91\x90\x91R_\x90`H\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\x06\x0BWa\x06\x0Ba\x08\x94V[\x80\x87`@Q` \x01a\x06\x1E\x92\x91\x90a\x08\xA8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__`@\x83\x85\x03\x12\x15a\x06hW__\xFD[\x82Q` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\x85W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x06\x95W__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xAEWa\x06\xAEa\x06CV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x06\xDCWa\x06\xDCa\x06CV[`@R\x81\x81R\x82\x82\x01` \x01\x87\x10\x15a\x06\xF3W__\xFD[\x81` \x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x84\x81R`\xFF`\xF8\x1B\x84`\xF8\x1B\x16` \x82\x01R\x82`!\x82\x01R_a\x07O`A\x83\x01\x84a\x07\x12V[\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07\x80Wa\x07\x80a\x07YV[\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x84\x81R` \x80\x82\x01\x85\x90R\x83Q`\x01`\x01`@\x1B\x03\x90\x81\x16`@\x80\x85\x01\x91\x90\x91R\x91\x85\x01Q\x81\x16``\x80\x85\x01\x91\x90\x91R\x91\x85\x01Q\x81\x16`\x80\x84\x01R\x90\x84\x01Q\x16`\xA0\x82\x01R`\xE0\x81\x01`\x04\x83\x10a\x084WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x82`\xC0\x83\x01R\x95\x94PPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x07\x80Wa\x07\x80a\x07YV[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x07\x80Wa\x07\x80a\x07YV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x07\x80Wa\x07\x80a\x07YV[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[_a\x08\xBCa\x08\xB6\x83\x86a\x07\x12V[\x84a\x07\x12V[\x94\x93PPPPV[a\x0B\xD0\x80a\x08\xD1_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01&W_5`\xE0\x1C\x80c\xA7\xB5\x1D\x19\x11a\0\xA9W\x80c\xD9\xDDg\xAB\x11a\0nW\x80c\xD9\xDDg\xAB\x14a\x027W\x80c\xE1\xD6j\xFE\x14a\x02JW\x80c\xE8\xEB\x1D\xC3\x14a\x02]W\x80c\xEC\xA0g\xAD\x14a\x02gW\x80c\xFB\xF6\xEA\xA5\x14a\x02oW__\xFD[\x80c\xA7\xB5\x1D\x19\x14a\x01\xBFW\x80c\xAD\x9C\x0C.\x14a\x01\xD2W\x80c\xB7R\xA7\xD1\x14a\x01\xDBW\x80c\xD5q\x9D\xC2\x14a\x01\xE5W\x80c\xD5\x95L4\x14a\x01\xF8W__\xFD[\x80c\x16\xBFUy\x11a\0\xEFW\x80c\x16\xBFUy\x14a\x01\x89W\x80c\x18\xDB9@\x14a\x01\x9CW\x80c/\x1E\xC5\xE9\x14a\x01\xA5W\x80cO5\x9A7\x14a\x01\xAEW\x80c\x7F\xA3\xA4\x0E\x14a\x01\xB6W__\xFD[\x80b\x84\x12\x0C\x14a\x01*W\x80c\x04\xF1\xC8T\x14a\x01AW\x80c\x05m\xAA\xA6\x14a\x01JW\x80c\x06\x1D\x12\xC0\x14a\x01tW\x80c\x06\xF10V\x14a\x01*W[__\xFD[`\x05T[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01.`\x03T\x81V[_Ta\x01\\\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x018V[a\x01\x87a\x01\x826`\x04a\t\rV[a\x02\x82V[\0[a\x01.a\x01\x976`\x04a\twV[a\x04:V[a\x01.`\x01T\x81V[a\x01\\a\x0E\x10\x81V[a\x01\\`\x0C\x81V[a\x01.`\x06T\x81V[a\x01\x87a\x01\xCD6`\x04a\t\xA4V[a\x04YV[a\x01\\a\x1C \x81V[a\x01\\b\x01Q\x80\x81V[a\x01.a\x01\xF36`\x04a\twV[a\x06$V[_T`\x01T`\x02T`\x03T`@\x80Q`\x01`\x01`@\x1B\x03\x95\x86\x16\x81R` \x81\x01\x94\x90\x94R\x93\x90\x91\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x91\x90\x91R`\x80\x01a\x018V[a\x01.a\x02E6`\x04a\twV[a\x063V[a\x01\x87a\x02X6`\x04a\n\x04V[a\x06WV[a\x01\\b\x01\xCC\xCC\x81V[`\x04Ta\x01.V[`\x02Ta\x01\\\x90`\x01`\x01`@\x1B\x03\x16\x81V[_\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x86\x81\x16\x91\x90\x91\x17\x90\x91U`\x01\x84\x90U\x82\x16\x15a\x02\xD0W`\x02\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x84\x16\x17\x90U`\x03\x81\x90U[`\x04T_\x80a\x02\xDF\x88\x84a\x06\x9FV[`\x05T\x91\x93P\x91P_\x81\x15a\x03\x17W`\x05a\x02\xFB`\x01\x84a\nRV[\x81T\x81\x10a\x03\x0BWa\x03\x0Ba\nkV[\x90_R` _ \x01T\x90P[_\x85\x15a\x03GW`\x04a\x03+`\x01\x88a\nRV[\x81T\x81\x10a\x03;Wa\x03;a\nkV[\x90_R` _ \x01T\x90P[`@\x80Q` \x80\x82\x01\x85\x90R\x81\x83\x01\x88\x90R``\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x92\x83\x90R\x81Q\x91\x01 `\x05\x80T`\x01\x80\x82\x01\x83U_\x92\x90\x92R\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB0\x01\x82\x90U`\x06\x89\x90U\x90\x91\x82\x91\x85\x91\x87\x91\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x91a\x03\xEC\x91\x88\x91\x8E\x91\x8D\x91a\n\x7FV[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D`@Qa\x04$\x91\x90a\x0B\x03V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPV[`\x05\x81\x81T\x81\x10a\x04IW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\x04T\x81Q` \x80\x84\x01\x91\x90\x91 `@Q`\x01`\x01`\xF8\x1B\x03\x19`\xF8\x88\x90\x1B\x16\x92\x81\x01\x92\x90\x92Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x86\x90\x1B\x16`!\x83\x01R`\x01`\x01`\xC0\x1B\x03\x19C`\xC0\x90\x81\x1B\x82\x16`5\x85\x01RB\x90\x1B\x16`=\x83\x01R`E\x82\x01\x83\x90R_`e\x83\x01\x81\x90R`\x85\x83\x01\x82\x90R\x90\x91`\xA5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P_\x83\x15a\x05\x1DW`\x04a\x05\x01`\x01\x86a\nRV[\x81T\x81\x10a\x05\x11Wa\x05\x11a\nkV[\x90_R` _ \x01T\x90P[`@\x80Q` \x80\x82\x01\x84\x90R\x81\x83\x01\x85\x90R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x80\x85R\x81Q\x91\x90\x92\x01 `\x04\x80T`\x01\x81\x01\x82U_\x91\x82R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01\x91\x90\x91U0\x90\x91R`\xFF\x8A\x16`\x80\x83\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16`\xA0\x83\x01R`\xC0\x82\x01\x86\x90R`\xE0\x82\x01RB`\x01`\x01`@\x1B\x03\x16a\x01\0\x82\x01R\x90Q\x82\x91\x86\x91\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE1\x91\x81\x90\x03a\x01 \x01\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x06\x13\x91\x90a\x0B\x03V[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[`\x04\x81\x81T\x81\x10a\x04IW_\x80\xFD[_`\x05\x82\x81T\x81\x10a\x06GWa\x06Ga\nkV[\x90_R` _ \x01T\x90P\x91\x90PV[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x90\x1B\x16` \x82\x01R`4\x81\x01\x82\x90Ra\x06\x9A\x90`\x0C\x90\x85\x90`T\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x04YV[PPPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R_\x84Q`(a\x06\xD0\x91\x90a\x0B8V[\x90Pb\x01\xCC\xCC\x81\x11\x15a\x07\x06W`@QcF4i\x1B`\xE0\x1B\x81R`\x04\x81\x01\x82\x90Rb\x01\xCC\xCC`$\x82\x01R`D\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Rb\x01Q\x80B\x11\x15a\x07MWa\x07Ab\x01Q\x80Ba\x0BKV[`\x01`\x01`@\x1B\x03\x16\x81R[a\x07Ya\x0E\x10Ba\x0BjV[`\x01`\x01`@\x1B\x03\x16` \x82\x01Ra\x1C C\x11\x15a\x07\x8CWa\x07}a\x1C Ca\x0BKV[`\x01`\x01`@\x1B\x03\x16`@\x82\x01R[a\x07\x97`\x0CCa\x0BjV[`\x01`\x01`@\x1B\x03\x81\x16``\x83\x01R\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xC0\x1B\x03\x19`\xC0\x96\x87\x1B\x81\x16\x95\x82\x01\x95\x90\x95R\x92\x85\x1B\x84\x16`(\x84\x01R\x84\x1B\x83\x16`0\x83\x01R\x93\x83\x1B\x82\x16`8\x82\x01R\x91\x88\x90\x1B\x16\x91\x81\x01\x91\x90\x91R_\x90`H\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\x08\x1DWa\x08\x1Da\x0B\x89V[\x80\x87`@Q` \x01a\x080\x92\x91\x90a\x0B\xB4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\x08xW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\x91Wa\x08\x91a\x08UV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x08\xBFWa\x08\xBFa\x08UV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x08\xD6W__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\t\x08W__\xFD[\x91\x90PV[_____`\xA0\x86\x88\x03\x12\x15a\t!W__\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15a\t6W__\xFD[a\tB\x88\x82\x89\x01a\x08iV[\x95PPa\tQ` \x87\x01a\x08\xF2V[\x93P`@\x86\x015\x92Pa\tf``\x87\x01a\x08\xF2V[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[_` \x82\x84\x03\x12\x15a\t\x87W__\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\x08W__\xFD[___``\x84\x86\x03\x12\x15a\t\xB6W__\xFD[\x835`\xFF\x81\x16\x81\x14a\t\xC6W__\xFD[\x92Pa\t\xD4` \x85\x01a\t\x8EV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xEEW__\xFD[a\t\xFA\x86\x82\x87\x01a\x08iV[\x91PP\x92P\x92P\x92V[___``\x84\x86\x03\x12\x15a\n\x16W__\xFD[a\n\x1F\x84a\t\x8EV[\x92Pa\n-` \x85\x01a\t\x8EV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\neWa\nea\n>V[\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_`\xE0\x82\x01\x90P\x85\x82R\x84` \x83\x01R`\x01`\x01`@\x1B\x03\x84Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03` \x85\x01Q\x16``\x83\x01R`\x01`\x01`@\x1B\x03`@\x85\x01Q\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03``\x85\x01Q\x16`\xA0\x83\x01R`\x04\x83\x10a\n\xF4WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x82`\xC0\x83\x01R\x95\x94PPPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\neWa\nea\n>V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\neWa\nea\n>V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\neWa\nea\n>V[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x0B\xC8a\x0B\xC2\x83\x86a\x0B\x9DV[\x84a\x0B\x9DV[\x94\x93PPPPV",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610126575f3560e01c8063a7b51d19116100a9578063d9dd67ab1161006e578063d9dd67ab14610237578063e1d66afe1461024a578063e8eb1dc31461025d578063eca067ad14610267578063fbf6eaa51461026f575f5ffd5b8063a7b51d19146101bf578063ad9c0c2e146101d2578063b752a7d1146101db578063d5719dc2146101e5578063d5954c34146101f8575f5ffd5b806316bf5579116100ef57806316bf55791461018957806318db39401461019c5780632f1ec5e9146101a55780634f359a37146101ae5780637fa3a40e146101b6575f5ffd5b806284120c1461012a57806304f1c85414610141578063056daaa61461014a578063061d12c01461017457806306f130561461012a575b5f5ffd5b6005545b6040519081526020015b60405180910390f35b61012e60035481565b5f5461015c906001600160401b031681565b6040516001600160401b039091168152602001610138565b61018761018236600461090d565b610282565b005b61012e610197366004610977565b61043a565b61012e60015481565b61015c610e1081565b61015c600c81565b61012e60065481565b6101876101cd3660046109a4565b610459565b61015c611c2081565b61015c6201518081565b61012e6101f3366004610977565b610624565b5f54600154600254600354604080516001600160401b039586168152602081019490945293909116928201929092526060810191909152608001610138565b61012e610245366004610977565b610633565b610187610258366004610a04565b610657565b61015c6201cccc81565b60045461012e565b60025461015c906001600160401b031681565b5f805467ffffffffffffffff19166001600160401b038681169190911790915560018490558216156102d0576002805467ffffffffffffffff19166001600160401b03841617905560038190555b6004545f806102df888461069f565b60055491935091505f81156103175760056102fb600184610a52565b8154811061030b5761030b610a6b565b905f5260205f20015490505b5f851561034757600461032b600188610a52565b8154811061033b5761033b610a6b565b905f5260205f20015490505b604080516020808201859052818301889052606080830185905283518084039091018152608090920192839052815191012060058054600180820183555f929092527f036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db001829055600689905590918291859187917f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7916103ec9188918e918d91610a7f565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d6040516104249190610b03565b60405180910390a2505050505050505050505050565b60058181548110610449575f80fd5b5f91825260209091200154905081565b60045481516020808401919091206040516001600160f81b031960f888901b16928101929092526bffffffffffffffffffffffff19606086901b1660218301526001600160c01b03194360c090811b8216603585015242901b16603d830152604582018390525f6065830181905260858301829052909160a50160408051601f19818403018152919052805160209091012090505f831561051d576004610501600186610a52565b8154811061051157610511610a6b565b905f5260205f20015490505b6040805160208082018490528183018590528251808303840181526060830180855281519190920120600480546001810182555f9182527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b01919091553090915260ff8a1660808301526001600160a01b03891660a083015260c0820186905260e0820152426001600160401b03166101008201529051829186917f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1918190036101200190a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b866040516106139190610b03565b60405180910390a250505050505050565b60048181548110610449575f80fd5b5f6005828154811061064757610647610a6b565b905f5260205f2001549050919050565b6040516bffffffffffffffffffffffff19606084901b1660208201526034810182905261069a90600c908590605401604051602081830303815290604052610459565b505050565b604080516080810182525f80825260208201819052918101829052606081018290525f845160286106d09190610b38565b90506201cccc81111561070657604051634634691b60e01b8152600481018290526201cccc602482015260440160405180910390fd5b604080516080810182525f8082526020820181905291810182905260608101919091526201518042111561074d576107416201518042610b4b565b6001600160401b031681525b610759610e1042610b6a565b6001600160401b03166020820152611c2043111561078c5761077d611c2043610b4b565b6001600160401b031660408201525b610797600c43610b6a565b6001600160401b0381166060830152815160208084015160408086015181516001600160c01b031960c096871b81169582019590955292851b84166028840152841b8316603083015293831b821660388201529188901b16918101919091525f906048016040516020818303038152906040529050602881511461081d5761081d610b89565b8087604051602001610830929190610bb4565b6040516020818303038152906040528051906020012082945094505050509250929050565b634e487b7160e01b5f52604160045260245ffd5b5f82601f830112610878575f5ffd5b81356001600160401b0381111561089157610891610855565b604051601f8201601f19908116603f011681016001600160401b03811182821017156108bf576108bf610855565b6040528181528382016020018510156108d6575f5ffd5b816020850160208301375f918101602001919091529392505050565b80356001600160401b0381168114610908575f5ffd5b919050565b5f5f5f5f5f60a08688031215610921575f5ffd5b85356001600160401b03811115610936575f5ffd5b61094288828901610869565b955050610951602087016108f2565b935060408601359250610966606087016108f2565b949793965091946080013592915050565b5f60208284031215610987575f5ffd5b5035919050565b80356001600160a01b0381168114610908575f5ffd5b5f5f5f606084860312156109b6575f5ffd5b833560ff811681146109c6575f5ffd5b92506109d46020850161098e565b915060408401356001600160401b038111156109ee575f5ffd5b6109fa86828701610869565b9150509250925092565b5f5f5f60608486031215610a16575f5ffd5b610a1f8461098e565b9250610a2d6020850161098e565b929592945050506040919091013590565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610a6557610a65610a3e565b92915050565b634e487b7160e01b5f52603260045260245ffd5b5f60e0820190508582528460208301526001600160401b0384511660408301526001600160401b0360208501511660608301526001600160401b0360408501511660808301526001600160401b0360608501511660a083015260048310610af457634e487b7160e01b5f52602160045260245ffd5b8260c083015295945050505050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80820180821115610a6557610a65610a3e565b6001600160401b038281168282160390811115610a6557610a65610a3e565b6001600160401b038181168382160190811115610a6557610a65610a3e565b634e487b7160e01b5f52600160045260245ffd5b5f81518060208401855e5f93019283525090919050565b5f610bc8610bc28386610b9d565b84610b9d565b94935050505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01&W_5`\xE0\x1C\x80c\xA7\xB5\x1D\x19\x11a\0\xA9W\x80c\xD9\xDDg\xAB\x11a\0nW\x80c\xD9\xDDg\xAB\x14a\x027W\x80c\xE1\xD6j\xFE\x14a\x02JW\x80c\xE8\xEB\x1D\xC3\x14a\x02]W\x80c\xEC\xA0g\xAD\x14a\x02gW\x80c\xFB\xF6\xEA\xA5\x14a\x02oW__\xFD[\x80c\xA7\xB5\x1D\x19\x14a\x01\xBFW\x80c\xAD\x9C\x0C.\x14a\x01\xD2W\x80c\xB7R\xA7\xD1\x14a\x01\xDBW\x80c\xD5q\x9D\xC2\x14a\x01\xE5W\x80c\xD5\x95L4\x14a\x01\xF8W__\xFD[\x80c\x16\xBFUy\x11a\0\xEFW\x80c\x16\xBFUy\x14a\x01\x89W\x80c\x18\xDB9@\x14a\x01\x9CW\x80c/\x1E\xC5\xE9\x14a\x01\xA5W\x80cO5\x9A7\x14a\x01\xAEW\x80c\x7F\xA3\xA4\x0E\x14a\x01\xB6W__\xFD[\x80b\x84\x12\x0C\x14a\x01*W\x80c\x04\xF1\xC8T\x14a\x01AW\x80c\x05m\xAA\xA6\x14a\x01JW\x80c\x06\x1D\x12\xC0\x14a\x01tW\x80c\x06\xF10V\x14a\x01*W[__\xFD[`\x05T[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01.`\x03T\x81V[_Ta\x01\\\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x018V[a\x01\x87a\x01\x826`\x04a\t\rV[a\x02\x82V[\0[a\x01.a\x01\x976`\x04a\twV[a\x04:V[a\x01.`\x01T\x81V[a\x01\\a\x0E\x10\x81V[a\x01\\`\x0C\x81V[a\x01.`\x06T\x81V[a\x01\x87a\x01\xCD6`\x04a\t\xA4V[a\x04YV[a\x01\\a\x1C \x81V[a\x01\\b\x01Q\x80\x81V[a\x01.a\x01\xF36`\x04a\twV[a\x06$V[_T`\x01T`\x02T`\x03T`@\x80Q`\x01`\x01`@\x1B\x03\x95\x86\x16\x81R` \x81\x01\x94\x90\x94R\x93\x90\x91\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x91\x90\x91R`\x80\x01a\x018V[a\x01.a\x02E6`\x04a\twV[a\x063V[a\x01\x87a\x02X6`\x04a\n\x04V[a\x06WV[a\x01\\b\x01\xCC\xCC\x81V[`\x04Ta\x01.V[`\x02Ta\x01\\\x90`\x01`\x01`@\x1B\x03\x16\x81V[_\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x86\x81\x16\x91\x90\x91\x17\x90\x91U`\x01\x84\x90U\x82\x16\x15a\x02\xD0W`\x02\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x84\x16\x17\x90U`\x03\x81\x90U[`\x04T_\x80a\x02\xDF\x88\x84a\x06\x9FV[`\x05T\x91\x93P\x91P_\x81\x15a\x03\x17W`\x05a\x02\xFB`\x01\x84a\nRV[\x81T\x81\x10a\x03\x0BWa\x03\x0Ba\nkV[\x90_R` _ \x01T\x90P[_\x85\x15a\x03GW`\x04a\x03+`\x01\x88a\nRV[\x81T\x81\x10a\x03;Wa\x03;a\nkV[\x90_R` _ \x01T\x90P[`@\x80Q` \x80\x82\x01\x85\x90R\x81\x83\x01\x88\x90R``\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x92\x83\x90R\x81Q\x91\x01 `\x05\x80T`\x01\x80\x82\x01\x83U_\x92\x90\x92R\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB0\x01\x82\x90U`\x06\x89\x90U\x90\x91\x82\x91\x85\x91\x87\x91\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x91a\x03\xEC\x91\x88\x91\x8E\x91\x8D\x91a\n\x7FV[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D`@Qa\x04$\x91\x90a\x0B\x03V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPV[`\x05\x81\x81T\x81\x10a\x04IW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\x04T\x81Q` \x80\x84\x01\x91\x90\x91 `@Q`\x01`\x01`\xF8\x1B\x03\x19`\xF8\x88\x90\x1B\x16\x92\x81\x01\x92\x90\x92Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x86\x90\x1B\x16`!\x83\x01R`\x01`\x01`\xC0\x1B\x03\x19C`\xC0\x90\x81\x1B\x82\x16`5\x85\x01RB\x90\x1B\x16`=\x83\x01R`E\x82\x01\x83\x90R_`e\x83\x01\x81\x90R`\x85\x83\x01\x82\x90R\x90\x91`\xA5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P_\x83\x15a\x05\x1DW`\x04a\x05\x01`\x01\x86a\nRV[\x81T\x81\x10a\x05\x11Wa\x05\x11a\nkV[\x90_R` _ \x01T\x90P[`@\x80Q` \x80\x82\x01\x84\x90R\x81\x83\x01\x85\x90R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x80\x85R\x81Q\x91\x90\x92\x01 `\x04\x80T`\x01\x81\x01\x82U_\x91\x82R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01\x91\x90\x91U0\x90\x91R`\xFF\x8A\x16`\x80\x83\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16`\xA0\x83\x01R`\xC0\x82\x01\x86\x90R`\xE0\x82\x01RB`\x01`\x01`@\x1B\x03\x16a\x01\0\x82\x01R\x90Q\x82\x91\x86\x91\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE1\x91\x81\x90\x03a\x01 \x01\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x06\x13\x91\x90a\x0B\x03V[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[`\x04\x81\x81T\x81\x10a\x04IW_\x80\xFD[_`\x05\x82\x81T\x81\x10a\x06GWa\x06Ga\nkV[\x90_R` _ \x01T\x90P\x91\x90PV[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x90\x1B\x16` \x82\x01R`4\x81\x01\x82\x90Ra\x06\x9A\x90`\x0C\x90\x85\x90`T\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x04YV[PPPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R_\x84Q`(a\x06\xD0\x91\x90a\x0B8V[\x90Pb\x01\xCC\xCC\x81\x11\x15a\x07\x06W`@QcF4i\x1B`\xE0\x1B\x81R`\x04\x81\x01\x82\x90Rb\x01\xCC\xCC`$\x82\x01R`D\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Rb\x01Q\x80B\x11\x15a\x07MWa\x07Ab\x01Q\x80Ba\x0BKV[`\x01`\x01`@\x1B\x03\x16\x81R[a\x07Ya\x0E\x10Ba\x0BjV[`\x01`\x01`@\x1B\x03\x16` \x82\x01Ra\x1C C\x11\x15a\x07\x8CWa\x07}a\x1C Ca\x0BKV[`\x01`\x01`@\x1B\x03\x16`@\x82\x01R[a\x07\x97`\x0CCa\x0BjV[`\x01`\x01`@\x1B\x03\x81\x16``\x83\x01R\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xC0\x1B\x03\x19`\xC0\x96\x87\x1B\x81\x16\x95\x82\x01\x95\x90\x95R\x92\x85\x1B\x84\x16`(\x84\x01R\x84\x1B\x83\x16`0\x83\x01R\x93\x83\x1B\x82\x16`8\x82\x01R\x91\x88\x90\x1B\x16\x91\x81\x01\x91\x90\x91R_\x90`H\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\x08\x1DWa\x08\x1Da\x0B\x89V[\x80\x87`@Q` \x01a\x080\x92\x91\x90a\x0B\xB4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\x08xW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\x91Wa\x08\x91a\x08UV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x08\xBFWa\x08\xBFa\x08UV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x08\xD6W__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\t\x08W__\xFD[\x91\x90PV[_____`\xA0\x86\x88\x03\x12\x15a\t!W__\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15a\t6W__\xFD[a\tB\x88\x82\x89\x01a\x08iV[\x95PPa\tQ` \x87\x01a\x08\xF2V[\x93P`@\x86\x015\x92Pa\tf``\x87\x01a\x08\xF2V[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[_` \x82\x84\x03\x12\x15a\t\x87W__\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\x08W__\xFD[___``\x84\x86\x03\x12\x15a\t\xB6W__\xFD[\x835`\xFF\x81\x16\x81\x14a\t\xC6W__\xFD[\x92Pa\t\xD4` \x85\x01a\t\x8EV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xEEW__\xFD[a\t\xFA\x86\x82\x87\x01a\x08iV[\x91PP\x92P\x92P\x92V[___``\x84\x86\x03\x12\x15a\n\x16W__\xFD[a\n\x1F\x84a\t\x8EV[\x92Pa\n-` \x85\x01a\t\x8EV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\neWa\nea\n>V[\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_`\xE0\x82\x01\x90P\x85\x82R\x84` \x83\x01R`\x01`\x01`@\x1B\x03\x84Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03` \x85\x01Q\x16``\x83\x01R`\x01`\x01`@\x1B\x03`@\x85\x01Q\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03``\x85\x01Q\x16`\xA0\x83\x01R`\x04\x83\x10a\n\xF4WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x82`\xC0\x83\x01R\x95\x94PPPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\neWa\nea\n>V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\neWa\nea\n>V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\neWa\nea\n>V[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x0B\xC8a\x0B\xC2\x83\x86a\x0B\x9DV[\x84a\x0B\x9DV[\x94\x93PPPPV",
    );
    /**Custom error with signature `DataTooLarge(uint256,uint256)` and selector `0x4634691b`.
```solidity
error DataTooLarge(uint256 dataLength, uint256 maxDataLength);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DataTooLarge {
        pub dataLength: alloy::sol_types::private::primitives::aliases::U256,
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
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
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
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`delayedInboxAccs(uint256)`](delayedInboxAccsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delayedInboxAccsReturn {
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
        pub kind: u8,
        pub sender: alloy::sol_types::private::Address,
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
        pub src: alloy::sol_types::private::Address,
        pub dest: alloy::sol_types::private::Address,
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
        pub _seqBlockNumber: u64,
        pub _seqBlockHash: alloy::sol_types::private::primitives::aliases::U256,
        pub _setBlockNumber: u64,
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
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`inboxAccs(uint256)`](inboxAccsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct inboxAccsReturn {
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
        pub data: alloy::sol_types::private::Bytes,
        pub _seqBlockNumber: u64,
        pub _seqBlockHash: alloy::sol_types::private::primitives::aliases::U256,
        pub _setBlockNumber: u64,
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
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`sequencerInboxAccs(uint256)`](sequencerInboxAccsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sequencerInboxAccsReturn {
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
        batchCount(batchCountCall),
        delayBlocks(delayBlocksCall),
        delaySeconds(delaySecondsCall),
        delayedInboxAccs(delayedInboxAccsCall),
        delayedMessageCount(delayedMessageCountCall),
        deliverMessage(deliverMessageCall),
        depositEth(depositEthCall),
        futureBlocks(futureBlocksCall),
        futureSeconds(futureSecondsCall),
        getSourceChainsProcessedBlocks(getSourceChainsProcessedBlocksCall),
        inboxAccs(inboxAccsCall),
        maxDataSize(maxDataSizeCall),
        postBatch(postBatchCall),
        seqBlockHash(seqBlockHashCall),
        seqBlockNumber(seqBlockNumberCall),
        sequencerInboxAccs(sequencerInboxAccsCall),
        sequencerMessageCount(sequencerMessageCountCall),
        setBlockHash(setBlockHashCall),
        setBlockNumber(setBlockNumberCall),
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
        InboxMessageDelivered(InboxMessageDelivered),
        MessageDelivered(MessageDelivered),
        SequencerBatchData(SequencerBatchData),
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
