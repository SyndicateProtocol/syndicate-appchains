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
    function maxDataSize() external view returns (uint64);
    function postBatch(bytes memory data, uint64 _seqBlockNum, uint256 _seqBlockHash, uint64 _setBlockNum, uint256 _setBlockHash) external;
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
        "name": "_seqBlockNum",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "_seqBlockHash",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_setBlockNum",
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
    ///0x60806040525f805f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055505f6001555f60025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055505f600355348015610066575f80fd5b50604051612a08380380612a088339818101604052810190610088919061099e565b5f8151116100cb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016100c290610a52565b60405180910390fd5b5f600190505f4890505f848383866040516020016100ec9493929190610b14565b604051602081830303815290604052905061010f600b5f8361016060201b60201c565b6101566040518060400160405280600681526020017e0b0080020300000000000000000000000000000000000000000000000000008152505f805f806102b660201b60201c565b50505050506112ab565b5f60048054905090505f828051906020012090505f858543428648876040516020016101929796959493929190610c42565b6040516020818303038152906040528051906020012090505f805f1b90505f8411156101e45760046001856101c79190610cef565b815481106101d8576101d7610d22565b5b905f5260205f20015490505b600481836040516020016101f9929190610d4f565b60405160208183030381529060405280519060200120908060018154018082558091505060019003905f5260205f20015f909190919091505580847f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1308a8a88484260405161026d96959493929190610dc5565b60405180910390a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b866040516102a59190610e76565b60405180910390a250505050505050565b5f805f9054906101000a900467ffffffffffffffff1667ffffffffffffffff161480610314575060015f8054906101000a900467ffffffffffffffff166102fd9190610e96565b67ffffffffffffffff168467ffffffffffffffff16145b610353576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161034a90610f1b565b60405180910390fd5b835f806101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550826001819055505f805f9054906101000a900467ffffffffffffffff1667ffffffffffffffff161115610477575f60025f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1614806104085750600160025f9054906101000a900467ffffffffffffffff166103f19190610e96565b67ffffffffffffffff168267ffffffffffffffff16145b610447576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161043e90610f83565b60405180910390fd5b8160025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550806003819055505b5f60048054905090505f8061049288846105f760201b60201c565b915091505f60058054905090505f805f1b90505f8211156104d95760056001836104bc9190610cef565b815481106104cd576104cc610d22565b5b905f5260205f20015490505b5f805f1b90505f8611156105135760046001876104f69190610cef565b8154811061050757610506610d22565b5b905f5260205f20015490505b5f82868360405160200161052993929190610fa1565b604051602081830303815290604052805190602001209050600581908060018154018082558091505060019003905f5260205f20015f9091909190915055866006819055508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7856006548a60016040516105a994939291906110b2565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d6040516105e19190610e76565b60405180910390a2505050505050505050505050565b5f6106006107d2565b5f8451602861060f91906110f5565b90506201cccc67ffffffffffffffff1681111561066857806201cccc6040517f4634691b00000000000000000000000000000000000000000000000000000000815260040161065f929190611161565b60405180910390fd5b6106706107d2565b6201518067ffffffffffffffff164211156106b35762015180426106949190611188565b815f019067ffffffffffffffff16908167ffffffffffffffff16815250505b610e10426106c19190610e96565b816020019067ffffffffffffffff16908167ffffffffffffffff1681525050611c2067ffffffffffffffff1643111561072257611c20436107029190611188565b816040019067ffffffffffffffff16908167ffffffffffffffff16815250505b600c4361072f9190610e96565b816060019067ffffffffffffffff16908167ffffffffffffffff16815250505f815f0151826020015183604001518460600151896040516020016107779594939291906111c3565b6040516020818303038152906040529050602881511461079a57610799611221565b5b80876040516020016107ad929190611288565b6040516020818303038152906040528051906020012082945094505050509250929050565b60405180608001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681525090565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b6108418161082f565b811461084b575f80fd5b50565b5f8151905061085c81610838565b92915050565b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6108b08261086a565b810181811067ffffffffffffffff821117156108cf576108ce61087a565b5b80604052505050565b5f6108e161081e565b90506108ed82826108a7565b919050565b5f67ffffffffffffffff82111561090c5761090b61087a565b5b6109158261086a565b9050602081019050919050565b8281835e5f83830152505050565b5f61094261093d846108f2565b6108d8565b90508281526020810184848401111561095e5761095d610866565b5b610969848285610922565b509392505050565b5f82601f83011261098557610984610862565b5b8151610995848260208601610930565b91505092915050565b5f80604083850312156109b4576109b3610827565b5b5f6109c18582860161084e565b925050602083015167ffffffffffffffff8111156109e2576109e161082b565b5b6109ee85828601610971565b9150509250929050565b5f82825260208201905092915050565b7f454d5054595f434841494e5f434f4e46494700000000000000000000000000005f82015250565b5f610a3c6012836109f8565b9150610a4782610a08565b602082019050919050565b5f6020820190508181035f830152610a6981610a30565b9050919050565b5f819050919050565b610a8a610a858261082f565b610a70565b82525050565b5f60ff82169050919050565b5f8160f81b9050919050565b5f610ab282610a9c565b9050919050565b610aca610ac582610a90565b610aa8565b82525050565b5f81519050919050565b5f81905092915050565b5f610aee82610ad0565b610af88185610ada565b9350610b08818560208601610922565b80840191505092915050565b5f610b1f8287610a79565b602082019150610b2f8286610ab9565b600182019150610b3f8285610a79565b602082019150610b4f8284610ae4565b915081905095945050505050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610b8682610b5d565b9050919050565b5f8160601b9050919050565b5f610ba382610b8d565b9050919050565b5f610bb482610b99565b9050919050565b610bcc610bc782610b7c565b610baa565b82525050565b5f67ffffffffffffffff82169050919050565b5f8160c01b9050919050565b5f610bfb82610be5565b9050919050565b610c13610c0e82610bd2565b610bf1565b82525050565b5f819050919050565b5f819050919050565b610c3c610c3782610c19565b610c22565b82525050565b5f610c4d828a610ab9565b600182019150610c5d8289610bbb565b601482019150610c6d8288610c02565b600882019150610c7d8287610c02565b600882019150610c8d8286610a79565b602082019150610c9d8285610a79565b602082019150610cad8284610c2b565b60208201915081905098975050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610cf98261082f565b9150610d048361082f565b9250828203905081811115610d1c57610d1b610cc2565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f610d5a8285610c2b565b602082019150610d6a8284610c2b565b6020820191508190509392505050565b610d8381610b7c565b82525050565b610d9281610a90565b82525050565b610da181610c19565b82525050565b610db08161082f565b82525050565b610dbf81610bd2565b82525050565b5f60c082019050610dd85f830189610d7a565b610de56020830188610d89565b610df26040830187610d7a565b610dff6060830186610d98565b610e0c6080830185610da7565b610e1960a0830184610db6565b979650505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f610e4882610e24565b610e528185610e2e565b9350610e62818560208601610922565b610e6b8161086a565b840191505092915050565b5f6020820190508181035f830152610e8e8184610e3e565b905092915050565b5f610ea082610bd2565b9150610eab83610bd2565b9250828201905067ffffffffffffffff811115610ecb57610eca610cc2565b5b92915050565b7f494e56414c49445f5345515f424c4f434b5f4e554d42455200000000000000005f82015250565b5f610f056018836109f8565b9150610f1082610ed1565b602082019050919050565b5f6020820190508181035f830152610f3281610ef9565b9050919050565b7f494e56414c49445f5345545f424c4f434b5f4e554d42455200000000000000005f82015250565b5f610f6d6018836109f8565b9150610f7882610f39565b602082019050919050565b5f6020820190508181035f830152610f9a81610f61565b9050919050565b5f610fac8286610c2b565b602082019150610fbc8285610c2b565b602082019150610fcc8284610c2b565b602082019150819050949350505050565b610fe681610bd2565b82525050565b608082015f8201516110005f850182610fdd565b5060208201516110136020850182610fdd565b5060408201516110266040850182610fdd565b5060608201516110396060850182610fdd565b50505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6004811061107d5761107c61103f565b5b50565b5f81905061108d8261106c565b919050565b5f61109c82611080565b9050919050565b6110ac81611092565b82525050565b5f60e0820190506110c55f830187610d98565b6110d26020830186610da7565b6110df6040830185610fec565b6110ec60c08301846110a3565b95945050505050565b5f6110ff8261082f565b915061110a8361082f565b925082820190508082111561112257611121610cc2565b5b92915050565b5f819050919050565b5f61114b61114661114184610bd2565b611128565b61082f565b9050919050565b61115b81611131565b82525050565b5f6040820190506111745f830185610da7565b6111816020830184611152565b9392505050565b5f61119282610bd2565b915061119d83610bd2565b9250828203905067ffffffffffffffff8111156111bd576111bc610cc2565b5b92915050565b5f6111ce8288610c02565b6008820191506111de8287610c02565b6008820191506111ee8286610c02565b6008820191506111fe8285610c02565b60088201915061120e8284610c02565b6008820191508190509695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b5f81905092915050565b5f61126282610e24565b61126c818561124e565b935061127c818560208601610922565b80840191505092915050565b5f6112938285611258565b915061129f8284611258565b91508190509392505050565b611750806112b85f395ff3fe608060405234801561000f575f80fd5b506004361061011e575f3560e01c80637fa3a40e116100ab578063d9dd67ab1161006f578063d9dd67ab146102e6578063e1d66afe14610316578063e8eb1dc314610332578063eca067ad14610350578063fbf6eaa51461036e5761011e565b80637fa3a40e14610240578063a7b51d191461025e578063ad9c0c2e1461027a578063b752a7d114610298578063d5719dc2146102b65761011e565b806306f13056116100f257806306f130561461019857806316bf5579146101b657806318db3940146101e65780632f1ec5e9146102045780634f359a37146102225761011e565b806284120c1461012257806304f1c85414610140578063056daaa61461015e578063061d12c01461017c575b5f80fd5b61012a61038c565b6040516101379190610b78565b60405180910390f35b610148610398565b6040516101559190610b78565b60405180910390f35b61016661039e565b6040516101739190610bb3565b60405180910390f35b61019660048036038101906101919190610d6d565b6103b5565b005b6101a06106f0565b6040516101ad9190610b78565b60405180910390f35b6101d060048036038101906101cb9190610e00565b6106fc565b6040516101dd9190610e43565b60405180910390f35b6101ee61071c565b6040516101fb9190610b78565b60405180910390f35b61020c610722565b6040516102199190610bb3565b60405180910390f35b61022a610728565b6040516102379190610bb3565b60405180910390f35b61024861072d565b6040516102559190610b78565b60405180910390f35b61027860048036038101906102739190610eec565b610733565b005b610282610889565b60405161028f9190610bb3565b60405180910390f35b6102a061088f565b6040516102ad9190610bb3565b60405180910390f35b6102d060048036038101906102cb9190610e00565b610896565b6040516102dd9190610e43565b60405180910390f35b61030060048036038101906102fb9190610e00565b6108b6565b60405161030d9190610e43565b60405180910390f35b610330600480360381019061032b9190610f58565b6108db565b005b61033a61090d565b6040516103479190610bb3565b60405180910390f35b610358610914565b6040516103659190610b78565b60405180910390f35b610376610920565b6040516103839190610bb3565b60405180910390f35b5f600580549050905090565b60035481565b5f8054906101000a900467ffffffffffffffff1681565b5f805f9054906101000a900467ffffffffffffffff1667ffffffffffffffff161480610413575060015f8054906101000a900467ffffffffffffffff166103fc9190610fd5565b67ffffffffffffffff168467ffffffffffffffff16145b610452576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104499061106a565b60405180910390fd5b835f806101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550826001819055505f805f9054906101000a900467ffffffffffffffff1667ffffffffffffffff161115610576575f60025f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1614806105075750600160025f9054906101000a900467ffffffffffffffff166104f09190610fd5565b67ffffffffffffffff168267ffffffffffffffff16145b610546576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161053d906110d2565b60405180910390fd5b8160025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550806003819055505b5f60048054905090505f8061058b8884610939565b915091505f60058054905090505f805f1b90505f8211156105d25760056001836105b591906110f0565b815481106105c6576105c5611123565b5b905f5260205f20015490505b5f805f1b90505f86111561060c5760046001876105ef91906110f0565b81548110610600576105ff611123565b5b905f5260205f20015490505b5f82868360405160200161062293929190611170565b604051602081830303815290604052805190602001209050600581908060018154018082558091505060019003905f5260205f20015f9091909190915055866006819055508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7856006548a60016040516106a29493929190611281565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d6040516106da9190611324565b60405180910390a2505050505050505050505050565b5f600580549050905090565b6005818154811061070b575f80fd5b905f5260205f20015f915090505481565b60015481565b610e1081565b600c81565b60065481565b5f60048054905090505f828051906020012090505f858543428648876040516020016107659796959493929190611411565b6040516020818303038152906040528051906020012090505f805f1b90505f8411156107b757600460018561079a91906110f0565b815481106107ab576107aa611123565b5b905f5260205f20015490505b600481836040516020016107cc929190611491565b60405160208183030381529060405280519060200120908060018154018082558091505060019003905f5260205f20015f909190919091505580847f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1308a8a884842604051610840969594939291906114da565b60405180910390a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b866040516108789190611324565b60405180910390a250505050505050565b611c2081565b6201518081565b600481815481106108a5575f80fd5b905f5260205f20015f915090505481565b5f600582815481106108cb576108ca611123565b5b905f5260205f2001549050919050565b610908600c8484846040516020016108f4929190611539565b604051602081830303815290604052610733565b505050565b6201cccc81565b5f600480549050905090565b60025f9054906101000a900467ffffffffffffffff1681565b5f610942610b14565b5f845160286109519190611564565b90506201cccc67ffffffffffffffff168111156109aa57806201cccc6040517f4634691b0000000000000000000000000000000000000000000000000000000081526004016109a19291906115d0565b60405180910390fd5b6109b2610b14565b6201518067ffffffffffffffff164211156109f55762015180426109d691906115f7565b815f019067ffffffffffffffff16908167ffffffffffffffff16815250505b610e1042610a039190610fd5565b816020019067ffffffffffffffff16908167ffffffffffffffff1681525050611c2067ffffffffffffffff16431115610a6457611c2043610a4491906115f7565b816040019067ffffffffffffffff16908167ffffffffffffffff16815250505b600c43610a719190610fd5565b816060019067ffffffffffffffff16908167ffffffffffffffff16815250505f815f015182602001518360400151846060015189604051602001610ab9959493929190611632565b60405160208183030381529060405290506028815114610adc57610adb611690565b5b8087604051602001610aef9291906116f7565b6040516020818303038152906040528051906020012082945094505050509250929050565b60405180608001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681525090565b5f819050919050565b610b7281610b60565b82525050565b5f602082019050610b8b5f830184610b69565b92915050565b5f67ffffffffffffffff82169050919050565b610bad81610b91565b82525050565b5f602082019050610bc65f830184610ba4565b92915050565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610c2b82610be5565b810181811067ffffffffffffffff82111715610c4a57610c49610bf5565b5b80604052505050565b5f610c5c610bcc565b9050610c688282610c22565b919050565b5f67ffffffffffffffff821115610c8757610c86610bf5565b5b610c9082610be5565b9050602081019050919050565b828183375f83830152505050565b5f610cbd610cb884610c6d565b610c53565b905082815260208101848484011115610cd957610cd8610be1565b5b610ce4848285610c9d565b509392505050565b5f82601f830112610d0057610cff610bdd565b5b8135610d10848260208601610cab565b91505092915050565b610d2281610b91565b8114610d2c575f80fd5b50565b5f81359050610d3d81610d19565b92915050565b610d4c81610b60565b8114610d56575f80fd5b50565b5f81359050610d6781610d43565b92915050565b5f805f805f60a08688031215610d8657610d85610bd5565b5b5f86013567ffffffffffffffff811115610da357610da2610bd9565b5b610daf88828901610cec565b9550506020610dc088828901610d2f565b9450506040610dd188828901610d59565b9350506060610de288828901610d2f565b9250506080610df388828901610d59565b9150509295509295909350565b5f60208284031215610e1557610e14610bd5565b5b5f610e2284828501610d59565b91505092915050565b5f819050919050565b610e3d81610e2b565b82525050565b5f602082019050610e565f830184610e34565b92915050565b5f60ff82169050919050565b610e7181610e5c565b8114610e7b575f80fd5b50565b5f81359050610e8c81610e68565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610ebb82610e92565b9050919050565b610ecb81610eb1565b8114610ed5575f80fd5b50565b5f81359050610ee681610ec2565b92915050565b5f805f60608486031215610f0357610f02610bd5565b5b5f610f1086828701610e7e565b9350506020610f2186828701610ed8565b925050604084013567ffffffffffffffff811115610f4257610f41610bd9565b5b610f4e86828701610cec565b9150509250925092565b5f805f60608486031215610f6f57610f6e610bd5565b5b5f610f7c86828701610ed8565b9350506020610f8d86828701610ed8565b9250506040610f9e86828701610d59565b9150509250925092565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610fdf82610b91565b9150610fea83610b91565b9250828201905067ffffffffffffffff81111561100a57611009610fa8565b5b92915050565b5f82825260208201905092915050565b7f494e56414c49445f5345515f424c4f434b5f4e554d42455200000000000000005f82015250565b5f611054601883611010565b915061105f82611020565b602082019050919050565b5f6020820190508181035f83015261108181611048565b9050919050565b7f494e56414c49445f5345545f424c4f434b5f4e554d42455200000000000000005f82015250565b5f6110bc601883611010565b91506110c782611088565b602082019050919050565b5f6020820190508181035f8301526110e9816110b0565b9050919050565b5f6110fa82610b60565b915061110583610b60565b925082820390508181111561111d5761111c610fa8565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b61116a61116582610e2b565b611150565b82525050565b5f61117b8286611159565b60208201915061118b8285611159565b60208201915061119b8284611159565b602082019150819050949350505050565b6111b581610b91565b82525050565b608082015f8201516111cf5f8501826111ac565b5060208201516111e260208501826111ac565b5060408201516111f560408501826111ac565b50606082015161120860608501826111ac565b50505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6004811061124c5761124b61120e565b5b50565b5f81905061125c8261123b565b919050565b5f61126b8261124f565b9050919050565b61127b81611261565b82525050565b5f60e0820190506112945f830187610e34565b6112a16020830186610b69565b6112ae60408301856111bb565b6112bb60c0830184611272565b95945050505050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f6112f6826112c4565b61130081856112ce565b93506113108185602086016112de565b61131981610be5565b840191505092915050565b5f6020820190508181035f83015261133c81846112ec565b905092915050565b5f8160f81b9050919050565b5f61135a82611344565b9050919050565b61137261136d82610e5c565b611350565b82525050565b5f8160601b9050919050565b5f61138e82611378565b9050919050565b5f61139f82611384565b9050919050565b6113b76113b282610eb1565b611395565b82525050565b5f8160c01b9050919050565b5f6113d3826113bd565b9050919050565b6113eb6113e682610b91565b6113c9565b82525050565b5f819050919050565b61140b61140682610b60565b6113f1565b82525050565b5f61141c828a611361565b60018201915061142c82896113a6565b60148201915061143c82886113da565b60088201915061144c82876113da565b60088201915061145c82866113fa565b60208201915061146c82856113fa565b60208201915061147c8284611159565b60208201915081905098975050505050505050565b5f61149c8285611159565b6020820191506114ac8284611159565b6020820191508190509392505050565b6114c581610eb1565b82525050565b6114d481610e5c565b82525050565b5f60c0820190506114ed5f8301896114bc565b6114fa60208301886114cb565b61150760408301876114bc565b6115146060830186610e34565b6115216080830185610b69565b61152e60a0830184610ba4565b979650505050505050565b5f61154482856113a6565b60148201915061155482846113fa565b6020820191508190509392505050565b5f61156e82610b60565b915061157983610b60565b925082820190508082111561159157611590610fa8565b5b92915050565b5f819050919050565b5f6115ba6115b56115b084610b91565b611597565b610b60565b9050919050565b6115ca816115a0565b82525050565b5f6040820190506115e35f830185610b69565b6115f060208301846115c1565b9392505050565b5f61160182610b91565b915061160c83610b91565b9250828203905067ffffffffffffffff81111561162c5761162b610fa8565b5b92915050565b5f61163d82886113da565b60088201915061164d82876113da565b60088201915061165d82866113da565b60088201915061166d82856113da565b60088201915061167d82846113da565b6008820191508190509695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b5f81905092915050565b5f6116d1826112c4565b6116db81856116bd565b93506116eb8185602086016112de565b80840191505092915050565b5f61170282856116c7565b915061170e82846116c7565b9150819050939250505056fea2646970667358221220b8ca7fb7227d5b5df988e9558de1343e6b2b9ddc06c18b342bd807faa086bf2f64736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R_\x80_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_`\x01U_`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_`\x03U4\x80\x15a\0fW_\x80\xFD[P`@Qa*\x088\x03\x80a*\x08\x839\x81\x81\x01`@R\x81\x01\x90a\0\x88\x91\x90a\t\x9EV[_\x81Q\x11a\0\xCBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xC2\x90a\nRV[`@Q\x80\x91\x03\x90\xFD[_`\x01\x90P_H\x90P_\x84\x83\x83\x86`@Q` \x01a\0\xEC\x94\x93\x92\x91\x90a\x0B\x14V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x01\x0F`\x0B_\x83a\x01`` \x1B` \x1CV[a\x01V`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01~\x0B\0\x80\x02\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP_\x80_\x80a\x02\xB6` \x1B` \x1CV[PPPPPa\x12\xABV[_`\x04\x80T\x90P\x90P_\x82\x80Q\x90` \x01 \x90P_\x85\x85CB\x86H\x87`@Q` \x01a\x01\x92\x97\x96\x95\x94\x93\x92\x91\x90a\x0CBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_\x80_\x1B\x90P_\x84\x11\x15a\x01\xE4W`\x04`\x01\x85a\x01\xC7\x91\x90a\x0C\xEFV[\x81T\x81\x10a\x01\xD8Wa\x01\xD7a\r\"V[[\x90_R` _ \x01T\x90P[`\x04\x81\x83`@Q` \x01a\x01\xF9\x92\x91\x90a\rOV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x80\x84\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE10\x8A\x8A\x88HB`@Qa\x02m\x96\x95\x94\x93\x92\x91\x90a\r\xC5V[`@Q\x80\x91\x03\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x02\xA5\x91\x90a\x0EvV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[_\x80_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x03\x14WP`\x01_\x80T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\xFD\x91\x90a\x0E\x96V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x03SW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03J\x90a\x0F\x1BV[`@Q\x80\x91\x03\x90\xFD[\x83_\x80a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82`\x01\x81\x90UP_\x80_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x04wW_`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x04\x08WP`\x01`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xF1\x91\x90a\x0E\x96V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x04GW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04>\x90a\x0F\x83V[`@Q\x80\x91\x03\x90\xFD[\x81`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x03\x81\x90UP[_`\x04\x80T\x90P\x90P_\x80a\x04\x92\x88\x84a\x05\xF7` \x1B` \x1CV[\x91P\x91P_`\x05\x80T\x90P\x90P_\x80_\x1B\x90P_\x82\x11\x15a\x04\xD9W`\x05`\x01\x83a\x04\xBC\x91\x90a\x0C\xEFV[\x81T\x81\x10a\x04\xCDWa\x04\xCCa\r\"V[[\x90_R` _ \x01T\x90P[_\x80_\x1B\x90P_\x86\x11\x15a\x05\x13W`\x04`\x01\x87a\x04\xF6\x91\x90a\x0C\xEFV[\x81T\x81\x10a\x05\x07Wa\x05\x06a\r\"V[[\x90_R` _ \x01T\x90P[_\x82\x86\x83`@Q` \x01a\x05)\x93\x92\x91\x90a\x0F\xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x05\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x86`\x06\x81\x90UP\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85`\x06T\x8A`\x01`@Qa\x05\xA9\x94\x93\x92\x91\x90a\x10\xB2V[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D`@Qa\x05\xE1\x91\x90a\x0EvV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPV[_a\x06\0a\x07\xD2V[_\x84Q`(a\x06\x0F\x91\x90a\x10\xF5V[\x90Pb\x01\xCC\xCCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\x06hW\x80b\x01\xCC\xCC`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06_\x92\x91\x90a\x11aV[`@Q\x80\x91\x03\x90\xFD[a\x06pa\x07\xD2V[b\x01Q\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15a\x06\xB3Wb\x01Q\x80Ba\x06\x94\x91\x90a\x11\x88V[\x81_\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[a\x0E\x10Ba\x06\xC1\x91\x90a\x0E\x96V[\x81` \x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x1C g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x11\x15a\x07\"Wa\x1C Ca\x07\x02\x91\x90a\x11\x88V[\x81`@\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[`\x0CCa\x07/\x91\x90a\x0E\x96V[\x81``\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x89`@Q` \x01a\x07w\x95\x94\x93\x92\x91\x90a\x11\xC3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\x07\x9AWa\x07\x99a\x12!V[[\x80\x87`@Q` \x01a\x07\xAD\x92\x91\x90a\x12\x88V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[`@Q\x80`\x80\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[a\x08A\x81a\x08/V[\x81\x14a\x08KW_\x80\xFD[PV[_\x81Q\x90Pa\x08\\\x81a\x088V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x08\xB0\x82a\x08jV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x08\xCFWa\x08\xCEa\x08zV[[\x80`@RPPPV[_a\x08\xE1a\x08\x1EV[\x90Pa\x08\xED\x82\x82a\x08\xA7V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\t\x0CWa\t\x0Ba\x08zV[[a\t\x15\x82a\x08jV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\tBa\t=\x84a\x08\xF2V[a\x08\xD8V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\t^Wa\t]a\x08fV[[a\ti\x84\x82\x85a\t\"V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\t\x85Wa\t\x84a\x08bV[[\x81Qa\t\x95\x84\x82` \x86\x01a\t0V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\t\xB4Wa\t\xB3a\x08'V[[_a\t\xC1\x85\x82\x86\x01a\x08NV[\x92PP` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xE2Wa\t\xE1a\x08+V[[a\t\xEE\x85\x82\x86\x01a\tqV[\x91PP\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FEMPTY_CHAIN_CONFIG\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\n<`\x12\x83a\t\xF8V[\x91Pa\nG\x82a\n\x08V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\ni\x81a\n0V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\n\x8Aa\n\x85\x82a\x08/V[a\npV[\x82RPPV[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\n\xB2\x82a\n\x9CV[\x90P\x91\x90PV[a\n\xCAa\n\xC5\x82a\n\x90V[a\n\xA8V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_a\n\xEE\x82a\n\xD0V[a\n\xF8\x81\x85a\n\xDAV[\x93Pa\x0B\x08\x81\x85` \x86\x01a\t\"V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x0B\x1F\x82\x87a\nyV[` \x82\x01\x91Pa\x0B/\x82\x86a\n\xB9V[`\x01\x82\x01\x91Pa\x0B?\x82\x85a\nyV[` \x82\x01\x91Pa\x0BO\x82\x84a\n\xE4V[\x91P\x81\x90P\x95\x94PPPPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0B\x86\x82a\x0B]V[\x90P\x91\x90PV[_\x81``\x1B\x90P\x91\x90PV[_a\x0B\xA3\x82a\x0B\x8DV[\x90P\x91\x90PV[_a\x0B\xB4\x82a\x0B\x99V[\x90P\x91\x90PV[a\x0B\xCCa\x0B\xC7\x82a\x0B|V[a\x0B\xAAV[\x82RPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xC0\x1B\x90P\x91\x90PV[_a\x0B\xFB\x82a\x0B\xE5V[\x90P\x91\x90PV[a\x0C\x13a\x0C\x0E\x82a\x0B\xD2V[a\x0B\xF1V[\x82RPPV[_\x81\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x0C<a\x0C7\x82a\x0C\x19V[a\x0C\"V[\x82RPPV[_a\x0CM\x82\x8Aa\n\xB9V[`\x01\x82\x01\x91Pa\x0C]\x82\x89a\x0B\xBBV[`\x14\x82\x01\x91Pa\x0Cm\x82\x88a\x0C\x02V[`\x08\x82\x01\x91Pa\x0C}\x82\x87a\x0C\x02V[`\x08\x82\x01\x91Pa\x0C\x8D\x82\x86a\nyV[` \x82\x01\x91Pa\x0C\x9D\x82\x85a\nyV[` \x82\x01\x91Pa\x0C\xAD\x82\x84a\x0C+V[` \x82\x01\x91P\x81\x90P\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x0C\xF9\x82a\x08/V[\x91Pa\r\x04\x83a\x08/V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\r\x1CWa\r\x1Ba\x0C\xC2V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_a\rZ\x82\x85a\x0C+V[` \x82\x01\x91Pa\rj\x82\x84a\x0C+V[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[a\r\x83\x81a\x0B|V[\x82RPPV[a\r\x92\x81a\n\x90V[\x82RPPV[a\r\xA1\x81a\x0C\x19V[\x82RPPV[a\r\xB0\x81a\x08/V[\x82RPPV[a\r\xBF\x81a\x0B\xD2V[\x82RPPV[_`\xC0\x82\x01\x90Pa\r\xD8_\x83\x01\x89a\rzV[a\r\xE5` \x83\x01\x88a\r\x89V[a\r\xF2`@\x83\x01\x87a\rzV[a\r\xFF``\x83\x01\x86a\r\x98V[a\x0E\x0C`\x80\x83\x01\x85a\r\xA7V[a\x0E\x19`\xA0\x83\x01\x84a\r\xB6V[\x97\x96PPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\x0EH\x82a\x0E$V[a\x0ER\x81\x85a\x0E.V[\x93Pa\x0Eb\x81\x85` \x86\x01a\t\"V[a\x0Ek\x81a\x08jV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0E\x8E\x81\x84a\x0E>V[\x90P\x92\x91PPV[_a\x0E\xA0\x82a\x0B\xD2V[\x91Pa\x0E\xAB\x83a\x0B\xD2V[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xCBWa\x0E\xCAa\x0C\xC2V[[\x92\x91PPV[\x7FINVALID_SEQ_BLOCK_NUMBER\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x0F\x05`\x18\x83a\t\xF8V[\x91Pa\x0F\x10\x82a\x0E\xD1V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0F2\x81a\x0E\xF9V[\x90P\x91\x90PV[\x7FINVALID_SET_BLOCK_NUMBER\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x0Fm`\x18\x83a\t\xF8V[\x91Pa\x0Fx\x82a\x0F9V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0F\x9A\x81a\x0FaV[\x90P\x91\x90PV[_a\x0F\xAC\x82\x86a\x0C+V[` \x82\x01\x91Pa\x0F\xBC\x82\x85a\x0C+V[` \x82\x01\x91Pa\x0F\xCC\x82\x84a\x0C+V[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[a\x0F\xE6\x81a\x0B\xD2V[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\x10\0_\x85\x01\x82a\x0F\xDDV[P` \x82\x01Qa\x10\x13` \x85\x01\x82a\x0F\xDDV[P`@\x82\x01Qa\x10&`@\x85\x01\x82a\x0F\xDDV[P``\x82\x01Qa\x109``\x85\x01\x82a\x0F\xDDV[PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x10}Wa\x10|a\x10?V[[PV[_\x81\x90Pa\x10\x8D\x82a\x10lV[\x91\x90PV[_a\x10\x9C\x82a\x10\x80V[\x90P\x91\x90PV[a\x10\xAC\x81a\x10\x92V[\x82RPPV[_`\xE0\x82\x01\x90Pa\x10\xC5_\x83\x01\x87a\r\x98V[a\x10\xD2` \x83\x01\x86a\r\xA7V[a\x10\xDF`@\x83\x01\x85a\x0F\xECV[a\x10\xEC`\xC0\x83\x01\x84a\x10\xA3V[\x95\x94PPPPPV[_a\x10\xFF\x82a\x08/V[\x91Pa\x11\n\x83a\x08/V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x11\"Wa\x11!a\x0C\xC2V[[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x11Ka\x11Fa\x11A\x84a\x0B\xD2V[a\x11(V[a\x08/V[\x90P\x91\x90PV[a\x11[\x81a\x111V[\x82RPPV[_`@\x82\x01\x90Pa\x11t_\x83\x01\x85a\r\xA7V[a\x11\x81` \x83\x01\x84a\x11RV[\x93\x92PPPV[_a\x11\x92\x82a\x0B\xD2V[\x91Pa\x11\x9D\x83a\x0B\xD2V[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xBDWa\x11\xBCa\x0C\xC2V[[\x92\x91PPV[_a\x11\xCE\x82\x88a\x0C\x02V[`\x08\x82\x01\x91Pa\x11\xDE\x82\x87a\x0C\x02V[`\x08\x82\x01\x91Pa\x11\xEE\x82\x86a\x0C\x02V[`\x08\x82\x01\x91Pa\x11\xFE\x82\x85a\x0C\x02V[`\x08\x82\x01\x91Pa\x12\x0E\x82\x84a\x0C\x02V[`\x08\x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[_\x81\x90P\x92\x91PPV[_a\x12b\x82a\x0E$V[a\x12l\x81\x85a\x12NV[\x93Pa\x12|\x81\x85` \x86\x01a\t\"V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x12\x93\x82\x85a\x12XV[\x91Pa\x12\x9F\x82\x84a\x12XV[\x91P\x81\x90P\x93\x92PPPV[a\x17P\x80a\x12\xB8_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01\x1EW_5`\xE0\x1C\x80c\x7F\xA3\xA4\x0E\x11a\0\xABW\x80c\xD9\xDDg\xAB\x11a\0oW\x80c\xD9\xDDg\xAB\x14a\x02\xE6W\x80c\xE1\xD6j\xFE\x14a\x03\x16W\x80c\xE8\xEB\x1D\xC3\x14a\x032W\x80c\xEC\xA0g\xAD\x14a\x03PW\x80c\xFB\xF6\xEA\xA5\x14a\x03nWa\x01\x1EV[\x80c\x7F\xA3\xA4\x0E\x14a\x02@W\x80c\xA7\xB5\x1D\x19\x14a\x02^W\x80c\xAD\x9C\x0C.\x14a\x02zW\x80c\xB7R\xA7\xD1\x14a\x02\x98W\x80c\xD5q\x9D\xC2\x14a\x02\xB6Wa\x01\x1EV[\x80c\x06\xF10V\x11a\0\xF2W\x80c\x06\xF10V\x14a\x01\x98W\x80c\x16\xBFUy\x14a\x01\xB6W\x80c\x18\xDB9@\x14a\x01\xE6W\x80c/\x1E\xC5\xE9\x14a\x02\x04W\x80cO5\x9A7\x14a\x02\"Wa\x01\x1EV[\x80b\x84\x12\x0C\x14a\x01\"W\x80c\x04\xF1\xC8T\x14a\x01@W\x80c\x05m\xAA\xA6\x14a\x01^W\x80c\x06\x1D\x12\xC0\x14a\x01|W[_\x80\xFD[a\x01*a\x03\x8CV[`@Qa\x017\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xF3[a\x01Ha\x03\x98V[`@Qa\x01U\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xF3[a\x01fa\x03\x9EV[`@Qa\x01s\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[a\x01\x96`\x04\x806\x03\x81\x01\x90a\x01\x91\x91\x90a\rmV[a\x03\xB5V[\0[a\x01\xA0a\x06\xF0V[`@Qa\x01\xAD\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xF3[a\x01\xD0`\x04\x806\x03\x81\x01\x90a\x01\xCB\x91\x90a\x0E\0V[a\x06\xFCV[`@Qa\x01\xDD\x91\x90a\x0ECV[`@Q\x80\x91\x03\x90\xF3[a\x01\xEEa\x07\x1CV[`@Qa\x01\xFB\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xF3[a\x02\x0Ca\x07\"V[`@Qa\x02\x19\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[a\x02*a\x07(V[`@Qa\x027\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[a\x02Ha\x07-V[`@Qa\x02U\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xF3[a\x02x`\x04\x806\x03\x81\x01\x90a\x02s\x91\x90a\x0E\xECV[a\x073V[\0[a\x02\x82a\x08\x89V[`@Qa\x02\x8F\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[a\x02\xA0a\x08\x8FV[`@Qa\x02\xAD\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[a\x02\xD0`\x04\x806\x03\x81\x01\x90a\x02\xCB\x91\x90a\x0E\0V[a\x08\x96V[`@Qa\x02\xDD\x91\x90a\x0ECV[`@Q\x80\x91\x03\x90\xF3[a\x03\0`\x04\x806\x03\x81\x01\x90a\x02\xFB\x91\x90a\x0E\0V[a\x08\xB6V[`@Qa\x03\r\x91\x90a\x0ECV[`@Q\x80\x91\x03\x90\xF3[a\x030`\x04\x806\x03\x81\x01\x90a\x03+\x91\x90a\x0FXV[a\x08\xDBV[\0[a\x03:a\t\rV[`@Qa\x03G\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[a\x03Xa\t\x14V[`@Qa\x03e\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xF3[a\x03va\t V[`@Qa\x03\x83\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[_`\x05\x80T\x90P\x90P\x90V[`\x03T\x81V[_\x80T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[_\x80_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x04\x13WP`\x01_\x80T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xFC\x91\x90a\x0F\xD5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x04RW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04I\x90a\x10jV[`@Q\x80\x91\x03\x90\xFD[\x83_\x80a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82`\x01\x81\x90UP_\x80_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x05vW_`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x05\x07WP`\x01`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\xF0\x91\x90a\x0F\xD5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x05FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05=\x90a\x10\xD2V[`@Q\x80\x91\x03\x90\xFD[\x81`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x03\x81\x90UP[_`\x04\x80T\x90P\x90P_\x80a\x05\x8B\x88\x84a\t9V[\x91P\x91P_`\x05\x80T\x90P\x90P_\x80_\x1B\x90P_\x82\x11\x15a\x05\xD2W`\x05`\x01\x83a\x05\xB5\x91\x90a\x10\xF0V[\x81T\x81\x10a\x05\xC6Wa\x05\xC5a\x11#V[[\x90_R` _ \x01T\x90P[_\x80_\x1B\x90P_\x86\x11\x15a\x06\x0CW`\x04`\x01\x87a\x05\xEF\x91\x90a\x10\xF0V[\x81T\x81\x10a\x06\0Wa\x05\xFFa\x11#V[[\x90_R` _ \x01T\x90P[_\x82\x86\x83`@Q` \x01a\x06\"\x93\x92\x91\x90a\x11pV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x05\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x86`\x06\x81\x90UP\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85`\x06T\x8A`\x01`@Qa\x06\xA2\x94\x93\x92\x91\x90a\x12\x81V[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D`@Qa\x06\xDA\x91\x90a\x13$V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPV[_`\x05\x80T\x90P\x90P\x90V[`\x05\x81\x81T\x81\x10a\x07\x0BW_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[`\x01T\x81V[a\x0E\x10\x81V[`\x0C\x81V[`\x06T\x81V[_`\x04\x80T\x90P\x90P_\x82\x80Q\x90` \x01 \x90P_\x85\x85CB\x86H\x87`@Q` \x01a\x07e\x97\x96\x95\x94\x93\x92\x91\x90a\x14\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_\x80_\x1B\x90P_\x84\x11\x15a\x07\xB7W`\x04`\x01\x85a\x07\x9A\x91\x90a\x10\xF0V[\x81T\x81\x10a\x07\xABWa\x07\xAAa\x11#V[[\x90_R` _ \x01T\x90P[`\x04\x81\x83`@Q` \x01a\x07\xCC\x92\x91\x90a\x14\x91V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x80\x84\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE10\x8A\x8A\x88HB`@Qa\x08@\x96\x95\x94\x93\x92\x91\x90a\x14\xDAV[`@Q\x80\x91\x03\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x08x\x91\x90a\x13$V[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[a\x1C \x81V[b\x01Q\x80\x81V[`\x04\x81\x81T\x81\x10a\x08\xA5W_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[_`\x05\x82\x81T\x81\x10a\x08\xCBWa\x08\xCAa\x11#V[[\x90_R` _ \x01T\x90P\x91\x90PV[a\t\x08`\x0C\x84\x84\x84`@Q` \x01a\x08\xF4\x92\x91\x90a\x159V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x073V[PPPV[b\x01\xCC\xCC\x81V[_`\x04\x80T\x90P\x90P\x90V[`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[_a\tBa\x0B\x14V[_\x84Q`(a\tQ\x91\x90a\x15dV[\x90Pb\x01\xCC\xCCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\t\xAAW\x80b\x01\xCC\xCC`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xA1\x92\x91\x90a\x15\xD0V[`@Q\x80\x91\x03\x90\xFD[a\t\xB2a\x0B\x14V[b\x01Q\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15a\t\xF5Wb\x01Q\x80Ba\t\xD6\x91\x90a\x15\xF7V[\x81_\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[a\x0E\x10Ba\n\x03\x91\x90a\x0F\xD5V[\x81` \x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x1C g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x11\x15a\ndWa\x1C Ca\nD\x91\x90a\x15\xF7V[\x81`@\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[`\x0CCa\nq\x91\x90a\x0F\xD5V[\x81``\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x89`@Q` \x01a\n\xB9\x95\x94\x93\x92\x91\x90a\x162V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\n\xDCWa\n\xDBa\x16\x90V[[\x80\x87`@Q` \x01a\n\xEF\x92\x91\x90a\x16\xF7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[`@Q\x80`\x80\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_\x81\x90P\x91\x90PV[a\x0Br\x81a\x0B`V[\x82RPPV[_` \x82\x01\x90Pa\x0B\x8B_\x83\x01\x84a\x0BiV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0B\xAD\x81a\x0B\x91V[\x82RPPV[_` \x82\x01\x90Pa\x0B\xC6_\x83\x01\x84a\x0B\xA4V[\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0C+\x82a\x0B\xE5V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0CJWa\x0CIa\x0B\xF5V[[\x80`@RPPPV[_a\x0C\\a\x0B\xCCV[\x90Pa\x0Ch\x82\x82a\x0C\"V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C\x87Wa\x0C\x86a\x0B\xF5V[[a\x0C\x90\x82a\x0B\xE5V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0C\xBDa\x0C\xB8\x84a\x0CmV[a\x0CSV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0C\xD9Wa\x0C\xD8a\x0B\xE1V[[a\x0C\xE4\x84\x82\x85a\x0C\x9DV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\r\0Wa\x0C\xFFa\x0B\xDDV[[\x815a\r\x10\x84\x82` \x86\x01a\x0C\xABV[\x91PP\x92\x91PPV[a\r\"\x81a\x0B\x91V[\x81\x14a\r,W_\x80\xFD[PV[_\x815\x90Pa\r=\x81a\r\x19V[\x92\x91PPV[a\rL\x81a\x0B`V[\x81\x14a\rVW_\x80\xFD[PV[_\x815\x90Pa\rg\x81a\rCV[\x92\x91PPV[_\x80_\x80_`\xA0\x86\x88\x03\x12\x15a\r\x86Wa\r\x85a\x0B\xD5V[[_\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xA3Wa\r\xA2a\x0B\xD9V[[a\r\xAF\x88\x82\x89\x01a\x0C\xECV[\x95PP` a\r\xC0\x88\x82\x89\x01a\r/V[\x94PP`@a\r\xD1\x88\x82\x89\x01a\rYV[\x93PP``a\r\xE2\x88\x82\x89\x01a\r/V[\x92PP`\x80a\r\xF3\x88\x82\x89\x01a\rYV[\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a\x0E\x15Wa\x0E\x14a\x0B\xD5V[[_a\x0E\"\x84\x82\x85\x01a\rYV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0E=\x81a\x0E+V[\x82RPPV[_` \x82\x01\x90Pa\x0EV_\x83\x01\x84a\x0E4V[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x0Eq\x81a\x0E\\V[\x81\x14a\x0E{W_\x80\xFD[PV[_\x815\x90Pa\x0E\x8C\x81a\x0EhV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0E\xBB\x82a\x0E\x92V[\x90P\x91\x90PV[a\x0E\xCB\x81a\x0E\xB1V[\x81\x14a\x0E\xD5W_\x80\xFD[PV[_\x815\x90Pa\x0E\xE6\x81a\x0E\xC2V[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x0F\x03Wa\x0F\x02a\x0B\xD5V[[_a\x0F\x10\x86\x82\x87\x01a\x0E~V[\x93PP` a\x0F!\x86\x82\x87\x01a\x0E\xD8V[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FBWa\x0FAa\x0B\xD9V[[a\x0FN\x86\x82\x87\x01a\x0C\xECV[\x91PP\x92P\x92P\x92V[_\x80_``\x84\x86\x03\x12\x15a\x0FoWa\x0Fna\x0B\xD5V[[_a\x0F|\x86\x82\x87\x01a\x0E\xD8V[\x93PP` a\x0F\x8D\x86\x82\x87\x01a\x0E\xD8V[\x92PP`@a\x0F\x9E\x86\x82\x87\x01a\rYV[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x0F\xDF\x82a\x0B\x91V[\x91Pa\x0F\xEA\x83a\x0B\x91V[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\nWa\x10\ta\x0F\xA8V[[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FINVALID_SEQ_BLOCK_NUMBER\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x10T`\x18\x83a\x10\x10V[\x91Pa\x10_\x82a\x10 V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x10\x81\x81a\x10HV[\x90P\x91\x90PV[\x7FINVALID_SET_BLOCK_NUMBER\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x10\xBC`\x18\x83a\x10\x10V[\x91Pa\x10\xC7\x82a\x10\x88V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x10\xE9\x81a\x10\xB0V[\x90P\x91\x90PV[_a\x10\xFA\x82a\x0B`V[\x91Pa\x11\x05\x83a\x0B`V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x11\x1DWa\x11\x1Ca\x0F\xA8V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[a\x11ja\x11e\x82a\x0E+V[a\x11PV[\x82RPPV[_a\x11{\x82\x86a\x11YV[` \x82\x01\x91Pa\x11\x8B\x82\x85a\x11YV[` \x82\x01\x91Pa\x11\x9B\x82\x84a\x11YV[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[a\x11\xB5\x81a\x0B\x91V[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\x11\xCF_\x85\x01\x82a\x11\xACV[P` \x82\x01Qa\x11\xE2` \x85\x01\x82a\x11\xACV[P`@\x82\x01Qa\x11\xF5`@\x85\x01\x82a\x11\xACV[P``\x82\x01Qa\x12\x08``\x85\x01\x82a\x11\xACV[PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x12LWa\x12Ka\x12\x0EV[[PV[_\x81\x90Pa\x12\\\x82a\x12;V[\x91\x90PV[_a\x12k\x82a\x12OV[\x90P\x91\x90PV[a\x12{\x81a\x12aV[\x82RPPV[_`\xE0\x82\x01\x90Pa\x12\x94_\x83\x01\x87a\x0E4V[a\x12\xA1` \x83\x01\x86a\x0BiV[a\x12\xAE`@\x83\x01\x85a\x11\xBBV[a\x12\xBB`\xC0\x83\x01\x84a\x12rV[\x95\x94PPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x12\xF6\x82a\x12\xC4V[a\x13\0\x81\x85a\x12\xCEV[\x93Pa\x13\x10\x81\x85` \x86\x01a\x12\xDEV[a\x13\x19\x81a\x0B\xE5V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x13<\x81\x84a\x12\xECV[\x90P\x92\x91PPV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\x13Z\x82a\x13DV[\x90P\x91\x90PV[a\x13ra\x13m\x82a\x0E\\V[a\x13PV[\x82RPPV[_\x81``\x1B\x90P\x91\x90PV[_a\x13\x8E\x82a\x13xV[\x90P\x91\x90PV[_a\x13\x9F\x82a\x13\x84V[\x90P\x91\x90PV[a\x13\xB7a\x13\xB2\x82a\x0E\xB1V[a\x13\x95V[\x82RPPV[_\x81`\xC0\x1B\x90P\x91\x90PV[_a\x13\xD3\x82a\x13\xBDV[\x90P\x91\x90PV[a\x13\xEBa\x13\xE6\x82a\x0B\x91V[a\x13\xC9V[\x82RPPV[_\x81\x90P\x91\x90PV[a\x14\x0Ba\x14\x06\x82a\x0B`V[a\x13\xF1V[\x82RPPV[_a\x14\x1C\x82\x8Aa\x13aV[`\x01\x82\x01\x91Pa\x14,\x82\x89a\x13\xA6V[`\x14\x82\x01\x91Pa\x14<\x82\x88a\x13\xDAV[`\x08\x82\x01\x91Pa\x14L\x82\x87a\x13\xDAV[`\x08\x82\x01\x91Pa\x14\\\x82\x86a\x13\xFAV[` \x82\x01\x91Pa\x14l\x82\x85a\x13\xFAV[` \x82\x01\x91Pa\x14|\x82\x84a\x11YV[` \x82\x01\x91P\x81\x90P\x98\x97PPPPPPPPV[_a\x14\x9C\x82\x85a\x11YV[` \x82\x01\x91Pa\x14\xAC\x82\x84a\x11YV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[a\x14\xC5\x81a\x0E\xB1V[\x82RPPV[a\x14\xD4\x81a\x0E\\V[\x82RPPV[_`\xC0\x82\x01\x90Pa\x14\xED_\x83\x01\x89a\x14\xBCV[a\x14\xFA` \x83\x01\x88a\x14\xCBV[a\x15\x07`@\x83\x01\x87a\x14\xBCV[a\x15\x14``\x83\x01\x86a\x0E4V[a\x15!`\x80\x83\x01\x85a\x0BiV[a\x15.`\xA0\x83\x01\x84a\x0B\xA4V[\x97\x96PPPPPPPV[_a\x15D\x82\x85a\x13\xA6V[`\x14\x82\x01\x91Pa\x15T\x82\x84a\x13\xFAV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[_a\x15n\x82a\x0B`V[\x91Pa\x15y\x83a\x0B`V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x15\x91Wa\x15\x90a\x0F\xA8V[[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x15\xBAa\x15\xB5a\x15\xB0\x84a\x0B\x91V[a\x15\x97V[a\x0B`V[\x90P\x91\x90PV[a\x15\xCA\x81a\x15\xA0V[\x82RPPV[_`@\x82\x01\x90Pa\x15\xE3_\x83\x01\x85a\x0BiV[a\x15\xF0` \x83\x01\x84a\x15\xC1V[\x93\x92PPPV[_a\x16\x01\x82a\x0B\x91V[\x91Pa\x16\x0C\x83a\x0B\x91V[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16,Wa\x16+a\x0F\xA8V[[\x92\x91PPV[_a\x16=\x82\x88a\x13\xDAV[`\x08\x82\x01\x91Pa\x16M\x82\x87a\x13\xDAV[`\x08\x82\x01\x91Pa\x16]\x82\x86a\x13\xDAV[`\x08\x82\x01\x91Pa\x16m\x82\x85a\x13\xDAV[`\x08\x82\x01\x91Pa\x16}\x82\x84a\x13\xDAV[`\x08\x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[_\x81\x90P\x92\x91PPV[_a\x16\xD1\x82a\x12\xC4V[a\x16\xDB\x81\x85a\x16\xBDV[\x93Pa\x16\xEB\x81\x85` \x86\x01a\x12\xDEV[\x80\x84\x01\x91PP\x92\x91PPV[_a\x17\x02\x82\x85a\x16\xC7V[\x91Pa\x17\x0E\x82\x84a\x16\xC7V[\x91P\x81\x90P\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xB8\xCA\x7F\xB7\"}[]\xF9\x88\xE9U\x8D\xE14>k+\x9D\xDC\x06\xC1\x8B4+\xD8\x07\xFA\xA0\x86\xBF/dsolcC\0\x08\x19\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b506004361061011e575f3560e01c80637fa3a40e116100ab578063d9dd67ab1161006f578063d9dd67ab146102e6578063e1d66afe14610316578063e8eb1dc314610332578063eca067ad14610350578063fbf6eaa51461036e5761011e565b80637fa3a40e14610240578063a7b51d191461025e578063ad9c0c2e1461027a578063b752a7d114610298578063d5719dc2146102b65761011e565b806306f13056116100f257806306f130561461019857806316bf5579146101b657806318db3940146101e65780632f1ec5e9146102045780634f359a37146102225761011e565b806284120c1461012257806304f1c85414610140578063056daaa61461015e578063061d12c01461017c575b5f80fd5b61012a61038c565b6040516101379190610b78565b60405180910390f35b610148610398565b6040516101559190610b78565b60405180910390f35b61016661039e565b6040516101739190610bb3565b60405180910390f35b61019660048036038101906101919190610d6d565b6103b5565b005b6101a06106f0565b6040516101ad9190610b78565b60405180910390f35b6101d060048036038101906101cb9190610e00565b6106fc565b6040516101dd9190610e43565b60405180910390f35b6101ee61071c565b6040516101fb9190610b78565b60405180910390f35b61020c610722565b6040516102199190610bb3565b60405180910390f35b61022a610728565b6040516102379190610bb3565b60405180910390f35b61024861072d565b6040516102559190610b78565b60405180910390f35b61027860048036038101906102739190610eec565b610733565b005b610282610889565b60405161028f9190610bb3565b60405180910390f35b6102a061088f565b6040516102ad9190610bb3565b60405180910390f35b6102d060048036038101906102cb9190610e00565b610896565b6040516102dd9190610e43565b60405180910390f35b61030060048036038101906102fb9190610e00565b6108b6565b60405161030d9190610e43565b60405180910390f35b610330600480360381019061032b9190610f58565b6108db565b005b61033a61090d565b6040516103479190610bb3565b60405180910390f35b610358610914565b6040516103659190610b78565b60405180910390f35b610376610920565b6040516103839190610bb3565b60405180910390f35b5f600580549050905090565b60035481565b5f8054906101000a900467ffffffffffffffff1681565b5f805f9054906101000a900467ffffffffffffffff1667ffffffffffffffff161480610413575060015f8054906101000a900467ffffffffffffffff166103fc9190610fd5565b67ffffffffffffffff168467ffffffffffffffff16145b610452576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104499061106a565b60405180910390fd5b835f806101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550826001819055505f805f9054906101000a900467ffffffffffffffff1667ffffffffffffffff161115610576575f60025f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1614806105075750600160025f9054906101000a900467ffffffffffffffff166104f09190610fd5565b67ffffffffffffffff168267ffffffffffffffff16145b610546576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161053d906110d2565b60405180910390fd5b8160025f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550806003819055505b5f60048054905090505f8061058b8884610939565b915091505f60058054905090505f805f1b90505f8211156105d25760056001836105b591906110f0565b815481106105c6576105c5611123565b5b905f5260205f20015490505b5f805f1b90505f86111561060c5760046001876105ef91906110f0565b81548110610600576105ff611123565b5b905f5260205f20015490505b5f82868360405160200161062293929190611170565b604051602081830303815290604052805190602001209050600581908060018154018082558091505060019003905f5260205f20015f9091909190915055866006819055508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7856006548a60016040516106a29493929190611281565b60405180910390a4837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d6040516106da9190611324565b60405180910390a2505050505050505050505050565b5f600580549050905090565b6005818154811061070b575f80fd5b905f5260205f20015f915090505481565b60015481565b610e1081565b600c81565b60065481565b5f60048054905090505f828051906020012090505f858543428648876040516020016107659796959493929190611411565b6040516020818303038152906040528051906020012090505f805f1b90505f8411156107b757600460018561079a91906110f0565b815481106107ab576107aa611123565b5b905f5260205f20015490505b600481836040516020016107cc929190611491565b60405160208183030381529060405280519060200120908060018154018082558091505060019003905f5260205f20015f909190919091505580847f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe1308a8a884842604051610840969594939291906114da565b60405180910390a3837fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b866040516108789190611324565b60405180910390a250505050505050565b611c2081565b6201518081565b600481815481106108a5575f80fd5b905f5260205f20015f915090505481565b5f600582815481106108cb576108ca611123565b5b905f5260205f2001549050919050565b610908600c8484846040516020016108f4929190611539565b604051602081830303815290604052610733565b505050565b6201cccc81565b5f600480549050905090565b60025f9054906101000a900467ffffffffffffffff1681565b5f610942610b14565b5f845160286109519190611564565b90506201cccc67ffffffffffffffff168111156109aa57806201cccc6040517f4634691b0000000000000000000000000000000000000000000000000000000081526004016109a19291906115d0565b60405180910390fd5b6109b2610b14565b6201518067ffffffffffffffff164211156109f55762015180426109d691906115f7565b815f019067ffffffffffffffff16908167ffffffffffffffff16815250505b610e1042610a039190610fd5565b816020019067ffffffffffffffff16908167ffffffffffffffff1681525050611c2067ffffffffffffffff16431115610a6457611c2043610a4491906115f7565b816040019067ffffffffffffffff16908167ffffffffffffffff16815250505b600c43610a719190610fd5565b816060019067ffffffffffffffff16908167ffffffffffffffff16815250505f815f015182602001518360400151846060015189604051602001610ab9959493929190611632565b60405160208183030381529060405290506028815114610adc57610adb611690565b5b8087604051602001610aef9291906116f7565b6040516020818303038152906040528051906020012082945094505050509250929050565b60405180608001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f67ffffffffffffffff1681525090565b5f819050919050565b610b7281610b60565b82525050565b5f602082019050610b8b5f830184610b69565b92915050565b5f67ffffffffffffffff82169050919050565b610bad81610b91565b82525050565b5f602082019050610bc65f830184610ba4565b92915050565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610c2b82610be5565b810181811067ffffffffffffffff82111715610c4a57610c49610bf5565b5b80604052505050565b5f610c5c610bcc565b9050610c688282610c22565b919050565b5f67ffffffffffffffff821115610c8757610c86610bf5565b5b610c9082610be5565b9050602081019050919050565b828183375f83830152505050565b5f610cbd610cb884610c6d565b610c53565b905082815260208101848484011115610cd957610cd8610be1565b5b610ce4848285610c9d565b509392505050565b5f82601f830112610d0057610cff610bdd565b5b8135610d10848260208601610cab565b91505092915050565b610d2281610b91565b8114610d2c575f80fd5b50565b5f81359050610d3d81610d19565b92915050565b610d4c81610b60565b8114610d56575f80fd5b50565b5f81359050610d6781610d43565b92915050565b5f805f805f60a08688031215610d8657610d85610bd5565b5b5f86013567ffffffffffffffff811115610da357610da2610bd9565b5b610daf88828901610cec565b9550506020610dc088828901610d2f565b9450506040610dd188828901610d59565b9350506060610de288828901610d2f565b9250506080610df388828901610d59565b9150509295509295909350565b5f60208284031215610e1557610e14610bd5565b5b5f610e2284828501610d59565b91505092915050565b5f819050919050565b610e3d81610e2b565b82525050565b5f602082019050610e565f830184610e34565b92915050565b5f60ff82169050919050565b610e7181610e5c565b8114610e7b575f80fd5b50565b5f81359050610e8c81610e68565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610ebb82610e92565b9050919050565b610ecb81610eb1565b8114610ed5575f80fd5b50565b5f81359050610ee681610ec2565b92915050565b5f805f60608486031215610f0357610f02610bd5565b5b5f610f1086828701610e7e565b9350506020610f2186828701610ed8565b925050604084013567ffffffffffffffff811115610f4257610f41610bd9565b5b610f4e86828701610cec565b9150509250925092565b5f805f60608486031215610f6f57610f6e610bd5565b5b5f610f7c86828701610ed8565b9350506020610f8d86828701610ed8565b9250506040610f9e86828701610d59565b9150509250925092565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610fdf82610b91565b9150610fea83610b91565b9250828201905067ffffffffffffffff81111561100a57611009610fa8565b5b92915050565b5f82825260208201905092915050565b7f494e56414c49445f5345515f424c4f434b5f4e554d42455200000000000000005f82015250565b5f611054601883611010565b915061105f82611020565b602082019050919050565b5f6020820190508181035f83015261108181611048565b9050919050565b7f494e56414c49445f5345545f424c4f434b5f4e554d42455200000000000000005f82015250565b5f6110bc601883611010565b91506110c782611088565b602082019050919050565b5f6020820190508181035f8301526110e9816110b0565b9050919050565b5f6110fa82610b60565b915061110583610b60565b925082820390508181111561111d5761111c610fa8565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b61116a61116582610e2b565b611150565b82525050565b5f61117b8286611159565b60208201915061118b8285611159565b60208201915061119b8284611159565b602082019150819050949350505050565b6111b581610b91565b82525050565b608082015f8201516111cf5f8501826111ac565b5060208201516111e260208501826111ac565b5060408201516111f560408501826111ac565b50606082015161120860608501826111ac565b50505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6004811061124c5761124b61120e565b5b50565b5f81905061125c8261123b565b919050565b5f61126b8261124f565b9050919050565b61127b81611261565b82525050565b5f60e0820190506112945f830187610e34565b6112a16020830186610b69565b6112ae60408301856111bb565b6112bb60c0830184611272565b95945050505050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f6112f6826112c4565b61130081856112ce565b93506113108185602086016112de565b61131981610be5565b840191505092915050565b5f6020820190508181035f83015261133c81846112ec565b905092915050565b5f8160f81b9050919050565b5f61135a82611344565b9050919050565b61137261136d82610e5c565b611350565b82525050565b5f8160601b9050919050565b5f61138e82611378565b9050919050565b5f61139f82611384565b9050919050565b6113b76113b282610eb1565b611395565b82525050565b5f8160c01b9050919050565b5f6113d3826113bd565b9050919050565b6113eb6113e682610b91565b6113c9565b82525050565b5f819050919050565b61140b61140682610b60565b6113f1565b82525050565b5f61141c828a611361565b60018201915061142c82896113a6565b60148201915061143c82886113da565b60088201915061144c82876113da565b60088201915061145c82866113fa565b60208201915061146c82856113fa565b60208201915061147c8284611159565b60208201915081905098975050505050505050565b5f61149c8285611159565b6020820191506114ac8284611159565b6020820191508190509392505050565b6114c581610eb1565b82525050565b6114d481610e5c565b82525050565b5f60c0820190506114ed5f8301896114bc565b6114fa60208301886114cb565b61150760408301876114bc565b6115146060830186610e34565b6115216080830185610b69565b61152e60a0830184610ba4565b979650505050505050565b5f61154482856113a6565b60148201915061155482846113fa565b6020820191508190509392505050565b5f61156e82610b60565b915061157983610b60565b925082820190508082111561159157611590610fa8565b5b92915050565b5f819050919050565b5f6115ba6115b56115b084610b91565b611597565b610b60565b9050919050565b6115ca816115a0565b82525050565b5f6040820190506115e35f830185610b69565b6115f060208301846115c1565b9392505050565b5f61160182610b91565b915061160c83610b91565b9250828203905067ffffffffffffffff81111561162c5761162b610fa8565b5b92915050565b5f61163d82886113da565b60088201915061164d82876113da565b60088201915061165d82866113da565b60088201915061166d82856113da565b60088201915061167d82846113da565b6008820191508190509695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b5f81905092915050565b5f6116d1826112c4565b6116db81856116bd565b93506116eb8185602086016112de565b80840191505092915050565b5f61170282856116c7565b915061170e82846116c7565b9150819050939250505056fea2646970667358221220b8ca7fb7227d5b5df988e9558de1343e6b2b9ddc06c18b342bd807faa086bf2f64736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01\x1EW_5`\xE0\x1C\x80c\x7F\xA3\xA4\x0E\x11a\0\xABW\x80c\xD9\xDDg\xAB\x11a\0oW\x80c\xD9\xDDg\xAB\x14a\x02\xE6W\x80c\xE1\xD6j\xFE\x14a\x03\x16W\x80c\xE8\xEB\x1D\xC3\x14a\x032W\x80c\xEC\xA0g\xAD\x14a\x03PW\x80c\xFB\xF6\xEA\xA5\x14a\x03nWa\x01\x1EV[\x80c\x7F\xA3\xA4\x0E\x14a\x02@W\x80c\xA7\xB5\x1D\x19\x14a\x02^W\x80c\xAD\x9C\x0C.\x14a\x02zW\x80c\xB7R\xA7\xD1\x14a\x02\x98W\x80c\xD5q\x9D\xC2\x14a\x02\xB6Wa\x01\x1EV[\x80c\x06\xF10V\x11a\0\xF2W\x80c\x06\xF10V\x14a\x01\x98W\x80c\x16\xBFUy\x14a\x01\xB6W\x80c\x18\xDB9@\x14a\x01\xE6W\x80c/\x1E\xC5\xE9\x14a\x02\x04W\x80cO5\x9A7\x14a\x02\"Wa\x01\x1EV[\x80b\x84\x12\x0C\x14a\x01\"W\x80c\x04\xF1\xC8T\x14a\x01@W\x80c\x05m\xAA\xA6\x14a\x01^W\x80c\x06\x1D\x12\xC0\x14a\x01|W[_\x80\xFD[a\x01*a\x03\x8CV[`@Qa\x017\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xF3[a\x01Ha\x03\x98V[`@Qa\x01U\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xF3[a\x01fa\x03\x9EV[`@Qa\x01s\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[a\x01\x96`\x04\x806\x03\x81\x01\x90a\x01\x91\x91\x90a\rmV[a\x03\xB5V[\0[a\x01\xA0a\x06\xF0V[`@Qa\x01\xAD\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xF3[a\x01\xD0`\x04\x806\x03\x81\x01\x90a\x01\xCB\x91\x90a\x0E\0V[a\x06\xFCV[`@Qa\x01\xDD\x91\x90a\x0ECV[`@Q\x80\x91\x03\x90\xF3[a\x01\xEEa\x07\x1CV[`@Qa\x01\xFB\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xF3[a\x02\x0Ca\x07\"V[`@Qa\x02\x19\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[a\x02*a\x07(V[`@Qa\x027\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[a\x02Ha\x07-V[`@Qa\x02U\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xF3[a\x02x`\x04\x806\x03\x81\x01\x90a\x02s\x91\x90a\x0E\xECV[a\x073V[\0[a\x02\x82a\x08\x89V[`@Qa\x02\x8F\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[a\x02\xA0a\x08\x8FV[`@Qa\x02\xAD\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[a\x02\xD0`\x04\x806\x03\x81\x01\x90a\x02\xCB\x91\x90a\x0E\0V[a\x08\x96V[`@Qa\x02\xDD\x91\x90a\x0ECV[`@Q\x80\x91\x03\x90\xF3[a\x03\0`\x04\x806\x03\x81\x01\x90a\x02\xFB\x91\x90a\x0E\0V[a\x08\xB6V[`@Qa\x03\r\x91\x90a\x0ECV[`@Q\x80\x91\x03\x90\xF3[a\x030`\x04\x806\x03\x81\x01\x90a\x03+\x91\x90a\x0FXV[a\x08\xDBV[\0[a\x03:a\t\rV[`@Qa\x03G\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[a\x03Xa\t\x14V[`@Qa\x03e\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xF3[a\x03va\t V[`@Qa\x03\x83\x91\x90a\x0B\xB3V[`@Q\x80\x91\x03\x90\xF3[_`\x05\x80T\x90P\x90P\x90V[`\x03T\x81V[_\x80T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[_\x80_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x04\x13WP`\x01_\x80T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xFC\x91\x90a\x0F\xD5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x04RW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04I\x90a\x10jV[`@Q\x80\x91\x03\x90\xFD[\x83_\x80a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82`\x01\x81\x90UP_\x80_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x05vW_`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x05\x07WP`\x01`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\xF0\x91\x90a\x0F\xD5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x05FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05=\x90a\x10\xD2V[`@Q\x80\x91\x03\x90\xFD[\x81`\x02_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x03\x81\x90UP[_`\x04\x80T\x90P\x90P_\x80a\x05\x8B\x88\x84a\t9V[\x91P\x91P_`\x05\x80T\x90P\x90P_\x80_\x1B\x90P_\x82\x11\x15a\x05\xD2W`\x05`\x01\x83a\x05\xB5\x91\x90a\x10\xF0V[\x81T\x81\x10a\x05\xC6Wa\x05\xC5a\x11#V[[\x90_R` _ \x01T\x90P[_\x80_\x1B\x90P_\x86\x11\x15a\x06\x0CW`\x04`\x01\x87a\x05\xEF\x91\x90a\x10\xF0V[\x81T\x81\x10a\x06\0Wa\x05\xFFa\x11#V[[\x90_R` _ \x01T\x90P[_\x82\x86\x83`@Q` \x01a\x06\"\x93\x92\x91\x90a\x11pV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x05\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x86`\x06\x81\x90UP\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85`\x06T\x8A`\x01`@Qa\x06\xA2\x94\x93\x92\x91\x90a\x12\x81V[`@Q\x80\x91\x03\x90\xA4\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D`@Qa\x06\xDA\x91\x90a\x13$V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPV[_`\x05\x80T\x90P\x90P\x90V[`\x05\x81\x81T\x81\x10a\x07\x0BW_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[`\x01T\x81V[a\x0E\x10\x81V[`\x0C\x81V[`\x06T\x81V[_`\x04\x80T\x90P\x90P_\x82\x80Q\x90` \x01 \x90P_\x85\x85CB\x86H\x87`@Q` \x01a\x07e\x97\x96\x95\x94\x93\x92\x91\x90a\x14\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_\x80_\x1B\x90P_\x84\x11\x15a\x07\xB7W`\x04`\x01\x85a\x07\x9A\x91\x90a\x10\xF0V[\x81T\x81\x10a\x07\xABWa\x07\xAAa\x11#V[[\x90_R` _ \x01T\x90P[`\x04\x81\x83`@Q` \x01a\x07\xCC\x92\x91\x90a\x14\x91V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x80\x84\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE10\x8A\x8A\x88HB`@Qa\x08@\x96\x95\x94\x93\x92\x91\x90a\x14\xDAV[`@Q\x80\x91\x03\x90\xA3\x83\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x86`@Qa\x08x\x91\x90a\x13$V[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[a\x1C \x81V[b\x01Q\x80\x81V[`\x04\x81\x81T\x81\x10a\x08\xA5W_\x80\xFD[\x90_R` _ \x01_\x91P\x90PT\x81V[_`\x05\x82\x81T\x81\x10a\x08\xCBWa\x08\xCAa\x11#V[[\x90_R` _ \x01T\x90P\x91\x90PV[a\t\x08`\x0C\x84\x84\x84`@Q` \x01a\x08\xF4\x92\x91\x90a\x159V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x073V[PPPV[b\x01\xCC\xCC\x81V[_`\x04\x80T\x90P\x90P\x90V[`\x02_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[_a\tBa\x0B\x14V[_\x84Q`(a\tQ\x91\x90a\x15dV[\x90Pb\x01\xCC\xCCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\t\xAAW\x80b\x01\xCC\xCC`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xA1\x92\x91\x90a\x15\xD0V[`@Q\x80\x91\x03\x90\xFD[a\t\xB2a\x0B\x14V[b\x01Q\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15a\t\xF5Wb\x01Q\x80Ba\t\xD6\x91\x90a\x15\xF7V[\x81_\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[a\x0E\x10Ba\n\x03\x91\x90a\x0F\xD5V[\x81` \x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x1C g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x11\x15a\ndWa\x1C Ca\nD\x91\x90a\x15\xF7V[\x81`@\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP[`\x0CCa\nq\x91\x90a\x0F\xD5V[\x81``\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x89`@Q` \x01a\n\xB9\x95\x94\x93\x92\x91\x90a\x162V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14a\n\xDCWa\n\xDBa\x16\x90V[[\x80\x87`@Q` \x01a\n\xEF\x92\x91\x90a\x16\xF7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x94P\x94PPPP\x92P\x92\x90PV[`@Q\x80`\x80\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_\x81\x90P\x91\x90PV[a\x0Br\x81a\x0B`V[\x82RPPV[_` \x82\x01\x90Pa\x0B\x8B_\x83\x01\x84a\x0BiV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0B\xAD\x81a\x0B\x91V[\x82RPPV[_` \x82\x01\x90Pa\x0B\xC6_\x83\x01\x84a\x0B\xA4V[\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0C+\x82a\x0B\xE5V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0CJWa\x0CIa\x0B\xF5V[[\x80`@RPPPV[_a\x0C\\a\x0B\xCCV[\x90Pa\x0Ch\x82\x82a\x0C\"V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C\x87Wa\x0C\x86a\x0B\xF5V[[a\x0C\x90\x82a\x0B\xE5V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x0C\xBDa\x0C\xB8\x84a\x0CmV[a\x0CSV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0C\xD9Wa\x0C\xD8a\x0B\xE1V[[a\x0C\xE4\x84\x82\x85a\x0C\x9DV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\r\0Wa\x0C\xFFa\x0B\xDDV[[\x815a\r\x10\x84\x82` \x86\x01a\x0C\xABV[\x91PP\x92\x91PPV[a\r\"\x81a\x0B\x91V[\x81\x14a\r,W_\x80\xFD[PV[_\x815\x90Pa\r=\x81a\r\x19V[\x92\x91PPV[a\rL\x81a\x0B`V[\x81\x14a\rVW_\x80\xFD[PV[_\x815\x90Pa\rg\x81a\rCV[\x92\x91PPV[_\x80_\x80_`\xA0\x86\x88\x03\x12\x15a\r\x86Wa\r\x85a\x0B\xD5V[[_\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xA3Wa\r\xA2a\x0B\xD9V[[a\r\xAF\x88\x82\x89\x01a\x0C\xECV[\x95PP` a\r\xC0\x88\x82\x89\x01a\r/V[\x94PP`@a\r\xD1\x88\x82\x89\x01a\rYV[\x93PP``a\r\xE2\x88\x82\x89\x01a\r/V[\x92PP`\x80a\r\xF3\x88\x82\x89\x01a\rYV[\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a\x0E\x15Wa\x0E\x14a\x0B\xD5V[[_a\x0E\"\x84\x82\x85\x01a\rYV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0E=\x81a\x0E+V[\x82RPPV[_` \x82\x01\x90Pa\x0EV_\x83\x01\x84a\x0E4V[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x0Eq\x81a\x0E\\V[\x81\x14a\x0E{W_\x80\xFD[PV[_\x815\x90Pa\x0E\x8C\x81a\x0EhV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0E\xBB\x82a\x0E\x92V[\x90P\x91\x90PV[a\x0E\xCB\x81a\x0E\xB1V[\x81\x14a\x0E\xD5W_\x80\xFD[PV[_\x815\x90Pa\x0E\xE6\x81a\x0E\xC2V[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x0F\x03Wa\x0F\x02a\x0B\xD5V[[_a\x0F\x10\x86\x82\x87\x01a\x0E~V[\x93PP` a\x0F!\x86\x82\x87\x01a\x0E\xD8V[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FBWa\x0FAa\x0B\xD9V[[a\x0FN\x86\x82\x87\x01a\x0C\xECV[\x91PP\x92P\x92P\x92V[_\x80_``\x84\x86\x03\x12\x15a\x0FoWa\x0Fna\x0B\xD5V[[_a\x0F|\x86\x82\x87\x01a\x0E\xD8V[\x93PP` a\x0F\x8D\x86\x82\x87\x01a\x0E\xD8V[\x92PP`@a\x0F\x9E\x86\x82\x87\x01a\rYV[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x0F\xDF\x82a\x0B\x91V[\x91Pa\x0F\xEA\x83a\x0B\x91V[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\nWa\x10\ta\x0F\xA8V[[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FINVALID_SEQ_BLOCK_NUMBER\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x10T`\x18\x83a\x10\x10V[\x91Pa\x10_\x82a\x10 V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x10\x81\x81a\x10HV[\x90P\x91\x90PV[\x7FINVALID_SET_BLOCK_NUMBER\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x10\xBC`\x18\x83a\x10\x10V[\x91Pa\x10\xC7\x82a\x10\x88V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x10\xE9\x81a\x10\xB0V[\x90P\x91\x90PV[_a\x10\xFA\x82a\x0B`V[\x91Pa\x11\x05\x83a\x0B`V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x11\x1DWa\x11\x1Ca\x0F\xA8V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[a\x11ja\x11e\x82a\x0E+V[a\x11PV[\x82RPPV[_a\x11{\x82\x86a\x11YV[` \x82\x01\x91Pa\x11\x8B\x82\x85a\x11YV[` \x82\x01\x91Pa\x11\x9B\x82\x84a\x11YV[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[a\x11\xB5\x81a\x0B\x91V[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\x11\xCF_\x85\x01\x82a\x11\xACV[P` \x82\x01Qa\x11\xE2` \x85\x01\x82a\x11\xACV[P`@\x82\x01Qa\x11\xF5`@\x85\x01\x82a\x11\xACV[P``\x82\x01Qa\x12\x08``\x85\x01\x82a\x11\xACV[PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x12LWa\x12Ka\x12\x0EV[[PV[_\x81\x90Pa\x12\\\x82a\x12;V[\x91\x90PV[_a\x12k\x82a\x12OV[\x90P\x91\x90PV[a\x12{\x81a\x12aV[\x82RPPV[_`\xE0\x82\x01\x90Pa\x12\x94_\x83\x01\x87a\x0E4V[a\x12\xA1` \x83\x01\x86a\x0BiV[a\x12\xAE`@\x83\x01\x85a\x11\xBBV[a\x12\xBB`\xC0\x83\x01\x84a\x12rV[\x95\x94PPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x12\xF6\x82a\x12\xC4V[a\x13\0\x81\x85a\x12\xCEV[\x93Pa\x13\x10\x81\x85` \x86\x01a\x12\xDEV[a\x13\x19\x81a\x0B\xE5V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x13<\x81\x84a\x12\xECV[\x90P\x92\x91PPV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\x13Z\x82a\x13DV[\x90P\x91\x90PV[a\x13ra\x13m\x82a\x0E\\V[a\x13PV[\x82RPPV[_\x81``\x1B\x90P\x91\x90PV[_a\x13\x8E\x82a\x13xV[\x90P\x91\x90PV[_a\x13\x9F\x82a\x13\x84V[\x90P\x91\x90PV[a\x13\xB7a\x13\xB2\x82a\x0E\xB1V[a\x13\x95V[\x82RPPV[_\x81`\xC0\x1B\x90P\x91\x90PV[_a\x13\xD3\x82a\x13\xBDV[\x90P\x91\x90PV[a\x13\xEBa\x13\xE6\x82a\x0B\x91V[a\x13\xC9V[\x82RPPV[_\x81\x90P\x91\x90PV[a\x14\x0Ba\x14\x06\x82a\x0B`V[a\x13\xF1V[\x82RPPV[_a\x14\x1C\x82\x8Aa\x13aV[`\x01\x82\x01\x91Pa\x14,\x82\x89a\x13\xA6V[`\x14\x82\x01\x91Pa\x14<\x82\x88a\x13\xDAV[`\x08\x82\x01\x91Pa\x14L\x82\x87a\x13\xDAV[`\x08\x82\x01\x91Pa\x14\\\x82\x86a\x13\xFAV[` \x82\x01\x91Pa\x14l\x82\x85a\x13\xFAV[` \x82\x01\x91Pa\x14|\x82\x84a\x11YV[` \x82\x01\x91P\x81\x90P\x98\x97PPPPPPPPV[_a\x14\x9C\x82\x85a\x11YV[` \x82\x01\x91Pa\x14\xAC\x82\x84a\x11YV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[a\x14\xC5\x81a\x0E\xB1V[\x82RPPV[a\x14\xD4\x81a\x0E\\V[\x82RPPV[_`\xC0\x82\x01\x90Pa\x14\xED_\x83\x01\x89a\x14\xBCV[a\x14\xFA` \x83\x01\x88a\x14\xCBV[a\x15\x07`@\x83\x01\x87a\x14\xBCV[a\x15\x14``\x83\x01\x86a\x0E4V[a\x15!`\x80\x83\x01\x85a\x0BiV[a\x15.`\xA0\x83\x01\x84a\x0B\xA4V[\x97\x96PPPPPPPV[_a\x15D\x82\x85a\x13\xA6V[`\x14\x82\x01\x91Pa\x15T\x82\x84a\x13\xFAV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[_a\x15n\x82a\x0B`V[\x91Pa\x15y\x83a\x0B`V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x15\x91Wa\x15\x90a\x0F\xA8V[[\x92\x91PPV[_\x81\x90P\x91\x90PV[_a\x15\xBAa\x15\xB5a\x15\xB0\x84a\x0B\x91V[a\x15\x97V[a\x0B`V[\x90P\x91\x90PV[a\x15\xCA\x81a\x15\xA0V[\x82RPPV[_`@\x82\x01\x90Pa\x15\xE3_\x83\x01\x85a\x0BiV[a\x15\xF0` \x83\x01\x84a\x15\xC1V[\x93\x92PPPV[_a\x16\x01\x82a\x0B\x91V[\x91Pa\x16\x0C\x83a\x0B\x91V[\x92P\x82\x82\x03\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16,Wa\x16+a\x0F\xA8V[[\x92\x91PPV[_a\x16=\x82\x88a\x13\xDAV[`\x08\x82\x01\x91Pa\x16M\x82\x87a\x13\xDAV[`\x08\x82\x01\x91Pa\x16]\x82\x86a\x13\xDAV[`\x08\x82\x01\x91Pa\x16m\x82\x85a\x13\xDAV[`\x08\x82\x01\x91Pa\x16}\x82\x84a\x13\xDAV[`\x08\x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[_\x81\x90P\x92\x91PPV[_a\x16\xD1\x82a\x12\xC4V[a\x16\xDB\x81\x85a\x16\xBDV[\x93Pa\x16\xEB\x81\x85` \x86\x01a\x12\xDEV[\x80\x84\x01\x91PP\x92\x91PPV[_a\x17\x02\x82\x85a\x16\xC7V[\x91Pa\x17\x0E\x82\x84a\x16\xC7V[\x91P\x81\x90P\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xB8\xCA\x7F\xB7\"}[]\xF9\x88\xE9U\x8D\xE14>k+\x9D\xDC\x06\xC1\x8B4+\xD8\x07\xFA\xA0\x86\xBF/dsolcC\0\x08\x19\x003",
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
function postBatch(bytes memory data, uint64 _seqBlockNum, uint256 _seqBlockHash, uint64 _setBlockNum, uint256 _setBlockHash) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postBatchCall {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _seqBlockNum: u64,
        #[allow(missing_docs)]
        pub _seqBlockHash: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _setBlockNum: u64,
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
                        value._seqBlockNum,
                        value._seqBlockHash,
                        value._setBlockNum,
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
                        _seqBlockNum: tuple.1,
                        _seqBlockHash: tuple.2,
                        _setBlockNum: tuple.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self._seqBlockNum),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._seqBlockHash),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._setBlockNum),
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
        const COUNT: usize = 19usize;
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
            _seqBlockNum: u64,
            _seqBlockHash: alloy::sol_types::private::primitives::aliases::U256,
            _setBlockNum: u64,
            _setBlockHash: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, postBatchCall, N> {
            self.call_builder(
                &postBatchCall {
                    data,
                    _seqBlockNum,
                    _seqBlockHash,
                    _setBlockNum,
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
