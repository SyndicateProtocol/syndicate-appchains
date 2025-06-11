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

    function INITIALIZATION_MSG_TYPE() external view returns (uint8);
    function L1MessageType_ethDeposit() external view returns (uint8);
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
    "name": "INITIALIZATION_MSG_TYPE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "L1MessageType_ethDeposit",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
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
    ///0x6080604052346100305761001a610014610197565b90610485565b610022610035565b611c006114478239611c0090f35b61003b565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906100679061003f565b810190811060018060401b0382111761007f57604052565b610049565b90610097610090610035565b928361005d565b565b5f80fd5b5f80fd5b90565b6100ad816100a1565b036100b457565b5f80fd5b905051906100c5826100a4565b565b5f80fd5b5f80fd5b60018060401b0381116100eb576100e760209161003f565b0190565b610049565b90825f9392825e0152565b9092919261011061010b826100cf565b610084565b9381855260208501908284011161012c5761012a926100f0565b565b6100cb565b9080601f8301121561014f5781602061014c935191016100fb565b90565b6100c7565b9190916040818403126101925761016d835f83016100b8565b92602082015160018060401b03811161018d5761018a9201610131565b90565b61009d565b610099565b6101b5613047803803806101aa81610084565b928339810190610154565b9091565b5f1b90565b906101cf60018060401b03916101b9565b9181191691161790565b90565b60018060401b031690565b90565b6101fe6101f9610203926101d9565b6101e7565b6101dc565b90565b90565b9061021e610219610225926101ea565b610206565b82546101be565b9055565b906102355f19916101b9565b9181191691161790565b61025361024e610258926101d9565b6101e7565b6100a1565b90565b90565b9061027361026e61027a9261023f565b61025b565b8254610229565b9055565b90565b5190565b60209181520190565b5f7f454d5054595f434841494e5f434f4e4649470000000000000000000000000000910152565b6102c26012602092610285565b6102cb8161028e565b0190565b6102e49060208101905f8183039101526102b5565b90565b156102ee57565b6102f6610035565b62461bcd60e51b81528061030c600482016102cf565b0390fd5b90565b60ff1690565b61032d61032861033292610310565b6101e7565b610313565b90565b61033f600b610319565b90565b60018060a01b031690565b61036161035c610366926101d9565b6101e7565b610342565b90565b6103729061034d565b90565b90565b61038c61038761039192610375565b6101e7565b610313565b90565b90565b6103a36103a8916100a1565b610394565b9052565b60f81b90565b6103bb906103ac565b90565b6103ca6103cf91610313565b6103b2565b9052565b5190565b905090565b6104016103f8926020926103ef816103d3565b948580936103d7565b938491016100f0565b0190565b60016020936104298584610421610431966104389b9a98610397565b0180926103be565b018092610397565b01906103dc565b90565b9061044d610448836100cf565b610084565b918252565b5f640b0080020360d01b910152565b61046b600661043b565b9061047860208301610452565b565b610482610461565b90565b90610535916104945f5f610209565b61049f5f600161025e565b6104aa5f6002610209565b6104b55f600361025e565b6104e16104c96104c48461027e565b610281565b6104db6104d55f61023f565b916100a1565b116102e7565b6104e9610335565b6105306104f55f610369565b926105216105036001610378565b9561050d5f61023f565b610515610035565b97889460208601610405565b6020820181038252038461005d565b610991565b61056f5f5f5f5f9161056961056361055d61055761055161047a565b976101ea565b9361023f565b936101ea565b9361023f565b93610cf7565b565b5490565b60200190565b61058f61058a610594926100a1565b6101e7565b6101dc565b90565b6105a090610342565b90565b60601b90565b6105b2906105a3565b90565b6105be906105a9565b90565b6105cd6105d291610597565b6105b5565b9052565b60c01b90565b6105e5906105d6565b90565b6105f46105f9916101dc565b6105dc565b9052565b90565b90565b61060f610614916105fd565b610600565b9052565b946106696008602099989596610661828c9961065960146106719a6106516106799f8f9c90610649816001936103be565b0180926105c1565b0180926105e8565b0180926105e8565b018092610397565b018092610397565b018092610603565b0190565b61069161068c610696926101d9565b6101b9565b6105fd565b90565b6106ad6106a86106b292610375565b6101e7565b6100a1565b90565b634e487b7160e01b5f52601160045260245ffd5b6106d86106de919392936100a1565b926100a1565b82039182116106e957565b6106b5565b634e487b7160e01b5f52603260045260245ffd5b5f5260205f2090565b61071481610571565b82101561072e57610726600191610702565b910201905f90565b6106ee565b1c90565b90565b61074a90600861074f9302610733565b610737565b90565b9061075d915461073a565b90565b90565b60208161077561077d93839695610603565b018092610603565b0190565b5f5260205f2090565b5490565b6107978161078a565b8210156107b1576107a9600191610781565b910201905f90565b6106ee565b1b90565b919060086107d59102916107cf5f19846107b6565b926107b6565b9181191691161790565b6107e8906105fd565b90565b5f1c90565b6107f9906107eb565b90565b919061081261080d61081a936107df565b6107f0565b9083546107ba565b9055565b908154916801000000000000000083101561084e578261084691600161084c9501815561078e565b906107fc565b565b610049565b61086761086261086c92610342565b6101e7565b610342565b90565b61087890610853565b90565b6108849061086f565b90565b61089b6108966108a0926100a1565b6101e7565b6100a1565b90565b6108ac90610597565b9052565b6108b990610313565b9052565b6108c6906105fd565b9052565b6108d39061023f565b9052565b6108e0906101dc565b9052565b919461092c6109369298979561092260a09661091861093d9a61090e60c08a019e5f8b01906108a3565b60208901906108b0565b60408701906108a3565b60608501906108bd565b60808301906108ca565b01906108d7565b565b60209181520190565b6109676109706020936109759361095e81610281565b9384809361093f565b958691016100f0565b61003f565b0190565b61098e9160208201915f818403910152610948565b90565b9061099c6004610571565b91836109b06109aa82610281565b91610575565b2091816109fd82916109ee6109c44361057b565b6109cd4261057b565b896109d75f61023f565b918a936109e2610035565b98899760208901610618565b6020820181038252038261005d565b610a0f610a0982610281565b91610575565b2090610a1a5f61067d565b9185610a2e610a285f61023f565b916100a1565b11610b2f575b610a8690610a426004610760565b90610a6d85610a5e610a52610035565b93849260208401610763565b6020820181038252038261005d565b610a7f610a7982610281565b91610575565b209061081e565b849192610ae8610a953061087b565b9192955f610aa24261057b565b91610ad6610ad07f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe198610887565b986107df565b98610adf610035565b968796876108e4565b0390a3610b2a610b187fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b92610887565b92610b21610035565b91829182610979565b0390a2565b9150610a86610b5c610b566004610b5089610b4a6001610699565b906106c9565b9061070b565b90610752565b929050610a34565b610b78610b73610b7d926101dc565b6101e7565b6101dc565b90565b90610b95610b90610b9c92610b64565b610206565b82546101be565b9055565b90610bb5610bb0610bbc92610887565b61025b565b8254610229565b9055565b60209392610bdf8583610bd78295610be797610603565b018092610603565b018092610603565b0190565b90565b610bfa610bff916107eb565b610beb565b90565b610c0c9054610bee565b90565b610c18906100a1565b9052565b610c25906101dc565b9052565b90606080610c6f93610c415f8201515f860190610c1c565b610c5360208201516020860190610c1c565b610c6560408201516040860190610c1c565b0151910190610c1c565b565b634e487b7160e01b5f52602160045260245ffd5b60051115610c8f57565b610c71565b90610c9e82610c85565b565b610ca990610c94565b90565b610cb590610ca0565b9052565b610cee610cf594610ce460c094989795610cda60e086019a5f8701906108bd565b6020850190610c0f565b6040830190610c29565b0190610cac565b565b9391610d06610d0d925f610b80565b6001610ba0565b81610d20610d1a5f6101ea565b916101dc565b11610efa575b5050610d326004610571565b610d3d828290611237565b90610d486005610571565b92610d525f61067d565b84610d65610d5f5f61023f565b916100a1565b11610ecc575b610d745f61067d565b9282610d88610d825f61023f565b916100a1565b11610e97575b610dec90610dbd83610dae87610da2610035565b94859360208501610bc0565b6020820181038252038261005d565b610dcf610dc982610281565b91610575565b2092610de5610dde6005610760565b859061081e565b6006610ba0565b8490919293610dfb6006610c02565b9094610e506001610e3e610e38610e327f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd797610887565b976107df565b976107df565b97610e47610035565b94859485610cb9565b0390a4610e92610e807ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c2092610887565b92610e89610035565b91829182610979565b0390a2565b9250610dec610ec4610ebe6004610eb886610eb26001610699565b906106c9565b9061070b565b90610752565b939050610d8e565b50610ef5610eef6005610ee987610ee36001610699565b906106c9565b9061070b565b90610752565b610d6b565b610f08610f0f926002610b80565b6003610ba0565b5f80610d26565b5f90565b610f246080610084565b90565b5f90565b610f33610f1a565b90602080808085610f42610f27565b815201610f4d610f27565b815201610f58610f27565b815201610f63610f27565b81525050565b610f71610f2b565b90565b90565b610f8b610f86610f9092610f74565b6101e7565b6100a1565b90565b610f9d6028610f77565b90565b610faf610fb5919392936100a1565b926100a1565b8201809211610fc057565b6106b5565b90565b610fdc610fd7610fe192610fc5565b6101e7565b6101dc565b90565b610ff06201cccc610fc8565b90565b61100761100261100c926101dc565b6101e7565b6100a1565b90565b61101890610ff3565b9052565b91602061103d92949361103660408201965f830190610c0f565b019061100f565b565b90565b61105661105161105b9261103f565b6101e7565b6101dc565b90565b61106a62015180611042565b90565b61107961107f916101dc565b916101dc565b90039060018060401b03821161109157565b6106b5565b906110a0906101dc565b9052565b90565b6110bb6110b66110c0926110a4565b6101e7565b6101dc565b90565b6110ce610e106110a7565b90565b6110dd6110e3916101dc565b916101dc565b019060018060401b0382116110f457565b6106b5565b90565b61111061110b611115926110f9565b6101e7565b6101dc565b90565b611123611c206110fc565b90565b90565b61113d61113861114292611126565b6101e7565b6101dc565b90565b61114f600c611129565b90565b61115c90516101dc565b90565b60086111a194611191828099989596611189828761118161119999839c6105e8565b0180926105e8565b0180926105e8565b0180926105e8565b0180926105e8565b0190565b634e487b7160e01b5f52600160045260245ffd5b156111c057565b6111a5565b905090565b6111ef6111e6926020926111dd81610281565b948580936111c5565b938491016100f0565b0190565b6112019061120793926111ca565b906111ca565b90565b61122992916112359161121b610035565b9485926020840192836111f3565b9081038252038361005d565b565b919091611242610f16565b5061124b610f69565b50611266611257610f93565b61126083610281565b90610fa0565b8061128061127a611275610fe4565b610ff3565b916100a1565b1161141f57506113b390611292610f69565b93426112ad6112a76112a261105e565b610ff3565b916100a1565b116113f5575b6112d96112d06112c24261057b565b6112ca6110c3565b906110d1565b60208701611096565b436112f36112ed6112e8611118565b610ff3565b916100a1565b116113ca575b61131f6113166113084361057b565b611310611145565b906110d1565b60608701611096565b61138361132d5f8701611152565b61137461133c60208901611152565b9361134960408a01611152565b9061135f61135960608c01611152565b9161057b565b91611368610035565b9687956020870161115f565b6020820181038252038261005d565b6113ae61138f82610281565b6113a86113a261139d610f93565b6100a1565b916100a1565b146111b9565b61120a565b6113c56113bf82610281565b91610575565b209190565b6113f06113e76113d94361057b565b6113e1611118565b9061106d565b60408701611096565b6112f9565b61141a6114126114044261057b565b61140c61105e565b9061106d565b5f8701611096565b6112b3565b611427610fe4565b906114425f928392634634691b60e01b84526004840161101c565b0390fdfe60806040526004361015610013575b610cb8565b61001d5f3561017b565b806284120c1461017657806304f1c85414610171578063056daaa61461016c578063061d12c01461016757806306f130561461016257806316bf55791461015d57806318db3940146101585780632f1ec5e9146101535780633c53a3831461014e5780634f359a37146101495780637fa3a40e14610144578063a7b51d191461013f578063ad9c0c2e1461013a578063ae1a7d3014610135578063b752a7d114610130578063d5719dc21461012b578063d5954c3414610126578063d9dd67ab14610121578063e1d66afe1461011c578063e8eb1dc314610117578063eca067ad146101125763fbf6eaa50361000e57610c83565b610c3f565b610c0a565b610b9d565b610b2e565b610af5565b610a82565b610a21565b6109b3565b610947565b6108db565b6107f8565b6107b4565b61074b565b6106b7565b610647565b610603565b6104ed565b6104b6565b6102d9565b610234565b6101c3565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f91031261019957565b61018b565b90565b6101aa9061019e565b9052565b91906101c1905f602085019401906101a1565b565b346101f3576101d336600461018f565b6101ef6101de610cc0565b6101e6610181565b918291826101ae565b0390f35b610187565b1c90565b90565b61020f90600861021493026101f8565b6101fc565b90565b9061022291546101ff565b90565b61023160035f90610217565b90565b346102645761024436600461018f565b61026061024f610225565b610257610181565b918291826101ae565b0390f35b610187565b67ffffffffffffffff1690565b61028690600861028b93026101f8565b610269565b90565b906102999154610276565b90565b6102a75f5f9061028e565b90565b67ffffffffffffffff1690565b6102c0906102aa565b9052565b91906102d7905f602085019401906102b7565b565b34610309576102e936600461018f565b6103056102f461029c565b6102fc610181565b918291826102c4565b0390f35b610187565b5f80fd5b5f80fd5b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906103429061031a565b810190811067ffffffffffffffff82111761035c57604052565b610324565b9061037461036d610181565b9283610338565b565b67ffffffffffffffff81116103945761039060209161031a565b0190565b610324565b90825f939282370152565b909291926103b96103b482610376565b610361565b938185526020850190828401116103d5576103d392610399565b565b610316565b9080601f830112156103f8578160206103f5933591016103a4565b90565b610312565b610406816102aa565b0361040d57565b5f80fd5b9050359061041e826103fd565b565b6104298161019e565b0361043057565b5f80fd5b9050359061044182610420565b565b919060a0838203126104ac575f83013567ffffffffffffffff81116104a7578161046e9185016103da565b9261047c8260208301610411565b926104a461048d8460408501610434565b9361049b8160608601610411565b93608001610434565b90565b61030e565b61018b565b5f0190565b346104e8576104d26104c9366004610443565b939290926110b6565b6104da610181565b806104e4816104b1565b0390f35b610187565b3461051d576104fd36600461018f565b6105196105086112d5565b610510610181565b918291826101ae565b0390f35b610187565b9060208282031261053b57610538915f01610434565b90565b61018b565b634e487b7160e01b5f52603260045260245ffd5b5490565b5f5260205f2090565b61056a81610554565b8210156105845761057c600191610558565b910201905f90565b610540565b90565b61059c9060086105a193026101f8565b610589565b90565b906105af915461058c565b90565b60056105bd81610554565b8210156105da576105d7916105d191610561565b906105a4565b90565b5f80fd5b90565b6105ea906105de565b9052565b9190610601905f602085019401906105e1565b565b346106335761062f61061e610619366004610522565b6105b2565b610626610181565b918291826105ee565b0390f35b610187565b61064460015f90610217565b90565b346106775761065736600461018f565b610673610662610638565b61066a610181565b918291826101ae565b0390f35b610187565b90565b90565b61069661069161069b9261067c565b61067f565b6102aa565b90565b6106a9610e10610682565b90565b6106b461069e565b90565b346106e7576106c736600461018f565b6106e36106d26106ac565b6106da610181565b918291826102c4565b0390f35b610187565b90565b60ff1690565b61070961070461070e926106ec565b61067f565b6106ef565b90565b61071b600c6106f5565b90565b610726610711565b90565b610732906106ef565b9052565b9190610749905f60208501940190610729565b565b3461077b5761075b36600461018f565b61077761076661071e565b61076e610181565b91829182610736565b0390f35b610187565b61079461078f610799926106ec565b61067f565b6102aa565b90565b6107a6600c610780565b90565b6107b161079c565b90565b346107e4576107c436600461018f565b6107e06107cf6107a9565b6107d7610181565b918291826102c4565b0390f35b610187565b6107f560065f90610217565b90565b346108285761080836600461018f565b6108246108136107e9565b61081b610181565b918291826101ae565b0390f35b610187565b610836816106ef565b0361083d57565b5f80fd5b9050359061084e8261082d565b565b60018060a01b031690565b61086490610850565b90565b6108708161085b565b0361087757565b5f80fd5b9050359061088882610867565b565b916060838303126108d6576108a1825f8501610841565b926108af836020830161087b565b92604082013567ffffffffffffffff81116108d1576108ce92016103da565b90565b61030e565b61018b565b3461090a576108f46108ee36600461088a565b916114cc565b6108fc610181565b80610906816104b1565b0390f35b610187565b90565b61092661092161092b9261090f565b61067f565b6102aa565b90565b610939611c20610912565b90565b61094461092e565b90565b346109775761095736600461018f565b61097361096261093c565b61096a610181565b918291826102c4565b0390f35b610187565b90565b61099361098e6109989261097c565b61067f565b6106ef565b90565b6109a5600b61097f565b90565b6109b061099b565b90565b346109e3576109c336600461018f565b6109df6109ce6109a8565b6109d6610181565b91829182610736565b0390f35b610187565b90565b6109ff6109fa610a04926109e8565b61067f565b6102aa565b90565b610a13620151806109eb565b90565b610a1e610a07565b90565b34610a5157610a3136600461018f565b610a4d610a3c610a16565b610a44610181565b918291826102c4565b0390f35b610187565b6004610a6181610554565b821015610a7e57610a7b91610a7591610561565b906105a4565b90565b5f80fd5b34610ab257610aae610a9d610a98366004610522565b610a56565b610aa5610181565b918291826105ee565b0390f35b610187565b610aec610af394610ae2606094989795610ad8608086019a5f8701906102b7565b60208501906101a1565b60408301906102b7565b01906101a1565b565b34610b2957610b0536600461018f565b610b25610b106116c4565b90610b1c949294610181565b94859485610ab7565b0390f35b610187565b34610b5e57610b5a610b49610b44366004610522565b61171d565b610b51610181565b918291826105ee565b0390f35b610187565b9091606082840312610b9857610b95610b7e845f850161087b565b93610b8c816020860161087b565b93604001610434565b90565b61018b565b34610bcc57610bb6610bb0366004610b63565b9161175c565b610bbe610181565b80610bc8816104b1565b0390f35b610187565b90565b610be8610be3610bed92610bd1565b61067f565b6102aa565b90565b610bfc6201cccc610bd4565b90565b610c07610bf0565b90565b34610c3a57610c1a36600461018f565b610c36610c25610bff565b610c2d610181565b918291826102c4565b0390f35b610187565b34610c6f57610c4f36600461018f565b610c6b610c5a61179c565b610c62610181565b918291826101ae565b0390f35b610187565b610c8060025f9061028e565b90565b34610cb357610c9336600461018f565b610caf610c9e610c74565b610ca6610181565b918291826102c4565b0390f35b610187565b5f80fd5b5f90565b610cc8610cbc565b50610cd36005610554565b90565b5f1b90565b90610cee67ffffffffffffffff91610cd6565b9181191691161790565b610d0c610d07610d11926102aa565b61067f565b6102aa565b90565b90565b90610d2c610d27610d3392610cf8565b610d14565b8254610cdb565b9055565b90610d435f1991610cd6565b9181191691161790565b610d61610d5c610d669261019e565b61067f565b61019e565b90565b90565b90610d81610d7c610d8892610d4d565b610d69565b8254610d37565b9055565b90565b610da3610d9e610da892610d8c565b61067f565b6102aa565b90565b610dbf610dba610dc492610d8c565b610cd6565b6105de565b90565b610ddb610dd6610de092610d8c565b61067f565b61019e565b90565b90565b610dfa610df5610dff92610de3565b61067f565b61019e565b90565b634e487b7160e01b5f52601160045260245ffd5b610e25610e2b9193929361019e565b9261019e565b8203918211610e3657565b610e02565b90565b610e4a610e4f916105de565b610e3b565b9052565b60209392610e728583610e6a8295610e7a97610e3e565b018092610e3e565b018092610e3e565b0190565b60200190565b5190565b90565b5f5260205f2090565b5490565b610ea181610e94565b821015610ebb57610eb3600191610e8b565b910201905f90565b610540565b1b90565b91906008610edf910291610ed95f1984610ec0565b92610ec0565b9181191691161790565b610ef2906105de565b90565b5f1c90565b610f0390610ef5565b90565b9190610f1c610f17610f2493610ee9565b610efa565b908354610ec4565b9055565b9081549168010000000000000000831015610f585782610f50916001610f5695018155610e98565b90610f06565b565b610324565b610f69610f6e91610ef5565b6101fc565b90565b610f7b9054610f5d565b90565b610f87906102aa565b9052565b90606080610fd193610fa35f8201515f860190610f7e565b610fb560208201516020860190610f7e565b610fc760408201516040860190610f7e565b0151910190610f7e565b565b634e487b7160e01b5f52602160045260245ffd5b60051115610ff157565b610fd3565b9061100082610fe7565b565b61100b90610ff6565b90565b61101790611002565b9052565b6110506110579461104660c09498979561103c60e086019a5f8701906105e1565b60208501906101a1565b6040830190610f8b565b019061100e565b565b60209181520190565b90825f9392825e0152565b61108c61109560209361109a9361108381610e84565b93848093611059565b95869101611062565b61031a565b0190565b6110b39160208201915f81840391015261106d565b90565b93916110c56110cc925f610d17565b6001610d6c565b816110df6110d95f610d8f565b916102aa565b116112b9575b50506110f16004610554565b6110fc8282906119f1565b906111076005610554565b926111115f610dab565b8461112461111e5f610dc7565b9161019e565b1161128b575b6111335f610dab565b92826111476111415f610dc7565b9161019e565b11611256575b6111ab9061117c8361116d87611161610181565b94859360208501610e53565b60208201810382520382610338565b61118e61118882610e84565b91610e7e565b20926111a461119d6005610e88565b8590610f28565b6006610d6c565b84909192936111ba6006610f71565b909461120f60016111fd6111f76111f17f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd797610d4d565b97610ee9565b97610ee9565b97611206610181565b9485948561101b565b0390a461125161123f7ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c2092610d4d565b92611248610181565b9182918261109e565b0390a2565b92506111ab61128361127d6004611277866112716001610de6565b90610e16565b90610561565b906105a4565b93905061114d565b506112b46112ae60056112a8876112a26001610de6565b90610e16565b90610561565b906105a4565b61112a565b6112c76112ce926002610d17565b6003610d6c565b5f806110e5565b6112dd610cbc565b506112e86005610554565b90565b6112ff6112fa6113049261019e565b61067f565b6102aa565b90565b60f81b90565b61131690611307565b90565b61132561132a916106ef565b61130d565b9052565b60601b90565b61133d9061132e565b90565b61134990611334565b90565b61135861135d9161085b565b611340565b9052565b60c01b90565b61137090611361565b90565b61137f611384916102aa565b611367565b9052565b90565b61139761139c9161019e565b611388565b9052565b946113f160086020999895966113e9828c996113e160146113f99a6113d96114019f8f9c906113d181600193611319565b01809261134c565b018092611373565b018092611373565b01809261138b565b01809261138b565b018092610e3e565b0190565b60208161141761141f93839695610e3e565b018092610e3e565b0190565b61143761143261143c92610850565b61067f565b610850565b90565b61144890611423565b90565b6114549061143f565b90565b6114609061085b565b9052565b61146d90610dc7565b9052565b91946114b96114c3929897956114af60a0966114a56114ca9a61149b60c08a019e5f8b0190611457565b6020890190610729565b6040870190611457565b60608501906105e1565b6080830190611464565b01906102b7565b565b906114d76004610554565b91836114eb6114e582610e84565b91610e7e565b20918161153882916115296114ff436112eb565b611508426112eb565b896115125f610dc7565b918a9361151d610181565b988997602089016113a0565b60208201810382520382610338565b61154a61154482610e84565b91610e7e565b20906115555f610dab565b91856115696115635f610dc7565b9161019e565b1161166a575b6115c19061157d6004610e88565b906115a88561159961158d610181565b93849260208401611405565b60208201810382520382610338565b6115ba6115b482610e84565b91610e7e565b2090610f28565b8491926116236115d03061144b565b9192955f6115dd426112eb565b9161161161160b7f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe198610d4d565b98610ee9565b9861161a610181565b96879687611471565b0390a36116656116537fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b92610d4d565b9261165c610181565b9182918261109e565b0390a2565b91506115c1611697611691600461168b896116856001610de6565b90610e16565b90610561565b906105a4565b92905061156f565b5f90565b6116af6116b491610ef5565b610269565b90565b6116c190546116a3565b90565b6116cc61169f565b506116d5610cbc565b506116de61169f565b506116e7610cbc565b506116f15f6116b7565b906116fc6001610f71565b9161170760026116b7565b916117126003610f71565b9193929190565b5f90565b61173461173a9161172c611719565b506005610561565b906105a4565b90565b601481611750611758936020969561134c565b01809261138b565b0190565b9061179a929161179561176d610711565b919261178661177a610181565b9586926020840161173d565b60208201810382520384610338565b6114cc565b565b6117a4610cbc565b506117af6004610554565b90565b6117bc6080610361565b90565b5f90565b6117cb6117b2565b906020808080856117da6117bf565b8152016117e56117bf565b8152016117f06117bf565b8152016117fb6117bf565b81525050565b6118096117c3565b90565b90565b61182361181e6118289261180c565b61067f565b61019e565b90565b611835602861180f565b90565b61184761184d9193929361019e565b9261019e565b820180921161185857565b610e02565b61187161186c611876926102aa565b61067f565b61019e565b90565b6118829061185d565b9052565b9160206118a79294936118a060408201965f8301906101a1565b0190611879565b565b6118b56118bb916102aa565b916102aa565b90039067ffffffffffffffff82116118cf57565b610e02565b906118de906102aa565b9052565b6118ee6118f4916102aa565b916102aa565b019067ffffffffffffffff821161190757565b610e02565b61191690516102aa565b90565b600861195b9461194b828099989596611943828761193b61195399839c611373565b018092611373565b018092611373565b018092611373565b018092611373565b0190565b634e487b7160e01b5f52600160045260245ffd5b1561197a57565b61195f565b905090565b6119a96119a09260209261199781610e84565b9485809361197f565b93849101611062565b0190565b6119bb906119c19392611984565b90611984565b90565b6119e392916119ef916119d5610181565b9485926020840192836119ad565b90810382520383610338565b565b9190916119fc611719565b50611a05611801565b50611a20611a1161182b565b611a1a83610e84565b90611838565b80611a3a611a34611a2f610bf0565b61185d565b9161019e565b11611bd95750611b6d90611a4c611801565b9342611a67611a61611a5c610a07565b61185d565b9161019e565b11611baf575b611a93611a8a611a7c426112eb565b611a8461069e565b906118e2565b602087016118d4565b43611aad611aa7611aa261092e565b61185d565b9161019e565b11611b84575b611ad9611ad0611ac2436112eb565b611aca61079c565b906118e2565b606087016118d4565b611b3d611ae75f870161190c565b611b2e611af66020890161190c565b93611b0360408a0161190c565b90611b19611b1360608c0161190c565b916112eb565b91611b22610181565b96879560208701611919565b60208201810382520382610338565b611b68611b4982610e84565b611b62611b5c611b5761182b565b61019e565b9161019e565b14611973565b6119c4565b611b7f611b7982610e84565b91610e7e565b209190565b611baa611ba1611b93436112eb565b611b9b61092e565b906118a9565b604087016118d4565b611ab3565b611bd4611bcc611bbe426112eb565b611bc6610a07565b906118a9565b5f87016118d4565b611a6d565b611be1610bf0565b90611bfc5f928392634634691b60e01b845260048401611886565b0390fd
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4a\x000Wa\0\x1Aa\0\x14a\x01\x97V[\x90a\x04\x85V[a\0\"a\x005V[a\x1C\0a\x14G\x829a\x1C\0\x90\xF3[a\0;V[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0g\x90a\0?V[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x7FW`@RV[a\0IV[\x90a\0\x97a\0\x90a\x005V[\x92\x83a\0]V[V[_\x80\xFD[_\x80\xFD[\x90V[a\0\xAD\x81a\0\xA1V[\x03a\0\xB4WV[_\x80\xFD[\x90PQ\x90a\0\xC5\x82a\0\xA4V[V[_\x80\xFD[_\x80\xFD[`\x01\x80`@\x1B\x03\x81\x11a\0\xEBWa\0\xE7` \x91a\0?V[\x01\x90V[a\0IV[\x90\x82_\x93\x92\x82^\x01RV[\x90\x92\x91\x92a\x01\x10a\x01\x0B\x82a\0\xCFV[a\0\x84V[\x93\x81\x85R` \x85\x01\x90\x82\x84\x01\x11a\x01,Wa\x01*\x92a\0\xF0V[V[a\0\xCBV[\x90\x80`\x1F\x83\x01\x12\x15a\x01OW\x81` a\x01L\x93Q\x91\x01a\0\xFBV[\x90V[a\0\xC7V[\x91\x90\x91`@\x81\x84\x03\x12a\x01\x92Wa\x01m\x83_\x83\x01a\0\xB8V[\x92` \x82\x01Q`\x01\x80`@\x1B\x03\x81\x11a\x01\x8DWa\x01\x8A\x92\x01a\x011V[\x90V[a\0\x9DV[a\0\x99V[a\x01\xB5a0G\x808\x03\x80a\x01\xAA\x81a\0\x84V[\x92\x839\x81\x01\x90a\x01TV[\x90\x91V[_\x1B\x90V[\x90a\x01\xCF`\x01\x80`@\x1B\x03\x91a\x01\xB9V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[`\x01\x80`@\x1B\x03\x16\x90V[\x90V[a\x01\xFEa\x01\xF9a\x02\x03\x92a\x01\xD9V[a\x01\xE7V[a\x01\xDCV[\x90V[\x90V[\x90a\x02\x1Ea\x02\x19a\x02%\x92a\x01\xEAV[a\x02\x06V[\x82Ta\x01\xBEV[\x90UV[\x90a\x025_\x19\x91a\x01\xB9V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x02Sa\x02Na\x02X\x92a\x01\xD9V[a\x01\xE7V[a\0\xA1V[\x90V[\x90V[\x90a\x02sa\x02na\x02z\x92a\x02?V[a\x02[V[\x82Ta\x02)V[\x90UV[\x90V[Q\x90V[` \x91\x81R\x01\x90V[_\x7FEMPTY_CHAIN_CONFIG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x02\xC2`\x12` \x92a\x02\x85V[a\x02\xCB\x81a\x02\x8EV[\x01\x90V[a\x02\xE4\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x02\xB5V[\x90V[\x15a\x02\xEEWV[a\x02\xF6a\x005V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x03\x0C`\x04\x82\x01a\x02\xCFV[\x03\x90\xFD[\x90V[`\xFF\x16\x90V[a\x03-a\x03(a\x032\x92a\x03\x10V[a\x01\xE7V[a\x03\x13V[\x90V[a\x03?`\x0Ba\x03\x19V[\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x03aa\x03\\a\x03f\x92a\x01\xD9V[a\x01\xE7V[a\x03BV[\x90V[a\x03r\x90a\x03MV[\x90V[\x90V[a\x03\x8Ca\x03\x87a\x03\x91\x92a\x03uV[a\x01\xE7V[a\x03\x13V[\x90V[\x90V[a\x03\xA3a\x03\xA8\x91a\0\xA1V[a\x03\x94V[\x90RV[`\xF8\x1B\x90V[a\x03\xBB\x90a\x03\xACV[\x90V[a\x03\xCAa\x03\xCF\x91a\x03\x13V[a\x03\xB2V[\x90RV[Q\x90V[\x90P\x90V[a\x04\x01a\x03\xF8\x92` \x92a\x03\xEF\x81a\x03\xD3V[\x94\x85\x80\x93a\x03\xD7V[\x93\x84\x91\x01a\0\xF0V[\x01\x90V[`\x01` \x93a\x04)\x85\x84a\x04!a\x041\x96a\x048\x9B\x9A\x98a\x03\x97V[\x01\x80\x92a\x03\xBEV[\x01\x80\x92a\x03\x97V[\x01\x90a\x03\xDCV[\x90V[\x90a\x04Ma\x04H\x83a\0\xCFV[a\0\x84V[\x91\x82RV[_d\x0B\0\x80\x02\x03`\xD0\x1B\x91\x01RV[a\x04k`\x06a\x04;V[\x90a\x04x` \x83\x01a\x04RV[V[a\x04\x82a\x04aV[\x90V[\x90a\x055\x91a\x04\x94__a\x02\tV[a\x04\x9F_`\x01a\x02^V[a\x04\xAA_`\x02a\x02\tV[a\x04\xB5_`\x03a\x02^V[a\x04\xE1a\x04\xC9a\x04\xC4\x84a\x02~V[a\x02\x81V[a\x04\xDBa\x04\xD5_a\x02?V[\x91a\0\xA1V[\x11a\x02\xE7V[a\x04\xE9a\x035V[a\x050a\x04\xF5_a\x03iV[\x92a\x05!a\x05\x03`\x01a\x03xV[\x95a\x05\r_a\x02?V[a\x05\x15a\x005V[\x97\x88\x94` \x86\x01a\x04\x05V[` \x82\x01\x81\x03\x82R\x03\x84a\0]V[a\t\x91V[a\x05o____\x91a\x05ia\x05ca\x05]a\x05Wa\x05Qa\x04zV[\x97a\x01\xEAV[\x93a\x02?V[\x93a\x01\xEAV[\x93a\x02?V[\x93a\x0C\xF7V[V[T\x90V[` \x01\x90V[a\x05\x8Fa\x05\x8Aa\x05\x94\x92a\0\xA1V[a\x01\xE7V[a\x01\xDCV[\x90V[a\x05\xA0\x90a\x03BV[\x90V[``\x1B\x90V[a\x05\xB2\x90a\x05\xA3V[\x90V[a\x05\xBE\x90a\x05\xA9V[\x90V[a\x05\xCDa\x05\xD2\x91a\x05\x97V[a\x05\xB5V[\x90RV[`\xC0\x1B\x90V[a\x05\xE5\x90a\x05\xD6V[\x90V[a\x05\xF4a\x05\xF9\x91a\x01\xDCV[a\x05\xDCV[\x90RV[\x90V[\x90V[a\x06\x0Fa\x06\x14\x91a\x05\xFDV[a\x06\0V[\x90RV[\x94a\x06i`\x08` \x99\x98\x95\x96a\x06a\x82\x8C\x99a\x06Y`\x14a\x06q\x9Aa\x06Qa\x06y\x9F\x8F\x9C\x90a\x06I\x81`\x01\x93a\x03\xBEV[\x01\x80\x92a\x05\xC1V[\x01\x80\x92a\x05\xE8V[\x01\x80\x92a\x05\xE8V[\x01\x80\x92a\x03\x97V[\x01\x80\x92a\x03\x97V[\x01\x80\x92a\x06\x03V[\x01\x90V[a\x06\x91a\x06\x8Ca\x06\x96\x92a\x01\xD9V[a\x01\xB9V[a\x05\xFDV[\x90V[a\x06\xADa\x06\xA8a\x06\xB2\x92a\x03uV[a\x01\xE7V[a\0\xA1V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x06\xD8a\x06\xDE\x91\x93\x92\x93a\0\xA1V[\x92a\0\xA1V[\x82\x03\x91\x82\x11a\x06\xE9WV[a\x06\xB5V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_R` _ \x90V[a\x07\x14\x81a\x05qV[\x82\x10\x15a\x07.Wa\x07&`\x01\x91a\x07\x02V[\x91\x02\x01\x90_\x90V[a\x06\xEEV[\x1C\x90V[\x90V[a\x07J\x90`\x08a\x07O\x93\x02a\x073V[a\x077V[\x90V[\x90a\x07]\x91Ta\x07:V[\x90V[\x90V[` \x81a\x07ua\x07}\x93\x83\x96\x95a\x06\x03V[\x01\x80\x92a\x06\x03V[\x01\x90V[_R` _ \x90V[T\x90V[a\x07\x97\x81a\x07\x8AV[\x82\x10\x15a\x07\xB1Wa\x07\xA9`\x01\x91a\x07\x81V[\x91\x02\x01\x90_\x90V[a\x06\xEEV[\x1B\x90V[\x91\x90`\x08a\x07\xD5\x91\x02\x91a\x07\xCF_\x19\x84a\x07\xB6V[\x92a\x07\xB6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x07\xE8\x90a\x05\xFDV[\x90V[_\x1C\x90V[a\x07\xF9\x90a\x07\xEBV[\x90V[\x91\x90a\x08\x12a\x08\ra\x08\x1A\x93a\x07\xDFV[a\x07\xF0V[\x90\x83Ta\x07\xBAV[\x90UV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x08NW\x82a\x08F\x91`\x01a\x08L\x95\x01\x81Ua\x07\x8EV[\x90a\x07\xFCV[V[a\0IV[a\x08ga\x08ba\x08l\x92a\x03BV[a\x01\xE7V[a\x03BV[\x90V[a\x08x\x90a\x08SV[\x90V[a\x08\x84\x90a\x08oV[\x90V[a\x08\x9Ba\x08\x96a\x08\xA0\x92a\0\xA1V[a\x01\xE7V[a\0\xA1V[\x90V[a\x08\xAC\x90a\x05\x97V[\x90RV[a\x08\xB9\x90a\x03\x13V[\x90RV[a\x08\xC6\x90a\x05\xFDV[\x90RV[a\x08\xD3\x90a\x02?V[\x90RV[a\x08\xE0\x90a\x01\xDCV[\x90RV[\x91\x94a\t,a\t6\x92\x98\x97\x95a\t\"`\xA0\x96a\t\x18a\t=\x9Aa\t\x0E`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x08\xA3V[` \x89\x01\x90a\x08\xB0V[`@\x87\x01\x90a\x08\xA3V[``\x85\x01\x90a\x08\xBDV[`\x80\x83\x01\x90a\x08\xCAV[\x01\x90a\x08\xD7V[V[` \x91\x81R\x01\x90V[a\tga\tp` \x93a\tu\x93a\t^\x81a\x02\x81V[\x93\x84\x80\x93a\t?V[\x95\x86\x91\x01a\0\xF0V[a\0?V[\x01\x90V[a\t\x8E\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\tHV[\x90V[\x90a\t\x9C`\x04a\x05qV[\x91\x83a\t\xB0a\t\xAA\x82a\x02\x81V[\x91a\x05uV[ \x91\x81a\t\xFD\x82\x91a\t\xEEa\t\xC4Ca\x05{V[a\t\xCDBa\x05{V[\x89a\t\xD7_a\x02?V[\x91\x8A\x93a\t\xE2a\x005V[\x98\x89\x97` \x89\x01a\x06\x18V[` \x82\x01\x81\x03\x82R\x03\x82a\0]V[a\n\x0Fa\n\t\x82a\x02\x81V[\x91a\x05uV[ \x90a\n\x1A_a\x06}V[\x91\x85a\n.a\n(_a\x02?V[\x91a\0\xA1V[\x11a\x0B/W[a\n\x86\x90a\nB`\x04a\x07`V[\x90a\nm\x85a\n^a\nRa\x005V[\x93\x84\x92` \x84\x01a\x07cV[` \x82\x01\x81\x03\x82R\x03\x82a\0]V[a\n\x7Fa\ny\x82a\x02\x81V[\x91a\x05uV[ \x90a\x08\x1EV[\x84\x91\x92a\n\xE8a\n\x950a\x08{V[\x91\x92\x95_a\n\xA2Ba\x05{V[\x91a\n\xD6a\n\xD0\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE1\x98a\x08\x87V[\x98a\x07\xDFV[\x98a\n\xDFa\x005V[\x96\x87\x96\x87a\x08\xE4V[\x03\x90\xA3a\x0B*a\x0B\x18\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x92a\x08\x87V[\x92a\x0B!a\x005V[\x91\x82\x91\x82a\tyV[\x03\x90\xA2V[\x91Pa\n\x86a\x0B\\a\x0BV`\x04a\x0BP\x89a\x0BJ`\x01a\x06\x99V[\x90a\x06\xC9V[\x90a\x07\x0BV[\x90a\x07RV[\x92\x90Pa\n4V[a\x0Bxa\x0Bsa\x0B}\x92a\x01\xDCV[a\x01\xE7V[a\x01\xDCV[\x90V[\x90a\x0B\x95a\x0B\x90a\x0B\x9C\x92a\x0BdV[a\x02\x06V[\x82Ta\x01\xBEV[\x90UV[\x90a\x0B\xB5a\x0B\xB0a\x0B\xBC\x92a\x08\x87V[a\x02[V[\x82Ta\x02)V[\x90UV[` \x93\x92a\x0B\xDF\x85\x83a\x0B\xD7\x82\x95a\x0B\xE7\x97a\x06\x03V[\x01\x80\x92a\x06\x03V[\x01\x80\x92a\x06\x03V[\x01\x90V[\x90V[a\x0B\xFAa\x0B\xFF\x91a\x07\xEBV[a\x0B\xEBV[\x90V[a\x0C\x0C\x90Ta\x0B\xEEV[\x90V[a\x0C\x18\x90a\0\xA1V[\x90RV[a\x0C%\x90a\x01\xDCV[\x90RV[\x90``\x80a\x0Co\x93a\x0CA_\x82\x01Q_\x86\x01\x90a\x0C\x1CV[a\x0CS` \x82\x01Q` \x86\x01\x90a\x0C\x1CV[a\x0Ce`@\x82\x01Q`@\x86\x01\x90a\x0C\x1CV[\x01Q\x91\x01\x90a\x0C\x1CV[V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x05\x11\x15a\x0C\x8FWV[a\x0CqV[\x90a\x0C\x9E\x82a\x0C\x85V[V[a\x0C\xA9\x90a\x0C\x94V[\x90V[a\x0C\xB5\x90a\x0C\xA0V[\x90RV[a\x0C\xEEa\x0C\xF5\x94a\x0C\xE4`\xC0\x94\x98\x97\x95a\x0C\xDA`\xE0\x86\x01\x9A_\x87\x01\x90a\x08\xBDV[` \x85\x01\x90a\x0C\x0FV[`@\x83\x01\x90a\x0C)V[\x01\x90a\x0C\xACV[V[\x93\x91a\r\x06a\r\r\x92_a\x0B\x80V[`\x01a\x0B\xA0V[\x81a\r a\r\x1A_a\x01\xEAV[\x91a\x01\xDCV[\x11a\x0E\xFAW[PPa\r2`\x04a\x05qV[a\r=\x82\x82\x90a\x127V[\x90a\rH`\x05a\x05qV[\x92a\rR_a\x06}V[\x84a\rea\r__a\x02?V[\x91a\0\xA1V[\x11a\x0E\xCCW[a\rt_a\x06}V[\x92\x82a\r\x88a\r\x82_a\x02?V[\x91a\0\xA1V[\x11a\x0E\x97W[a\r\xEC\x90a\r\xBD\x83a\r\xAE\x87a\r\xA2a\x005V[\x94\x85\x93` \x85\x01a\x0B\xC0V[` \x82\x01\x81\x03\x82R\x03\x82a\0]V[a\r\xCFa\r\xC9\x82a\x02\x81V[\x91a\x05uV[ \x92a\r\xE5a\r\xDE`\x05a\x07`V[\x85\x90a\x08\x1EV[`\x06a\x0B\xA0V[\x84\x90\x91\x92\x93a\r\xFB`\x06a\x0C\x02V[\x90\x94a\x0EP`\x01a\x0E>a\x0E8a\x0E2\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x97a\x08\x87V[\x97a\x07\xDFV[\x97a\x07\xDFV[\x97a\x0EGa\x005V[\x94\x85\x94\x85a\x0C\xB9V[\x03\x90\xA4a\x0E\x92a\x0E\x80\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x92a\x08\x87V[\x92a\x0E\x89a\x005V[\x91\x82\x91\x82a\tyV[\x03\x90\xA2V[\x92Pa\r\xECa\x0E\xC4a\x0E\xBE`\x04a\x0E\xB8\x86a\x0E\xB2`\x01a\x06\x99V[\x90a\x06\xC9V[\x90a\x07\x0BV[\x90a\x07RV[\x93\x90Pa\r\x8EV[Pa\x0E\xF5a\x0E\xEF`\x05a\x0E\xE9\x87a\x0E\xE3`\x01a\x06\x99V[\x90a\x06\xC9V[\x90a\x07\x0BV[\x90a\x07RV[a\rkV[a\x0F\x08a\x0F\x0F\x92`\x02a\x0B\x80V[`\x03a\x0B\xA0V[_\x80a\r&V[_\x90V[a\x0F$`\x80a\0\x84V[\x90V[_\x90V[a\x0F3a\x0F\x1AV[\x90` \x80\x80\x80\x85a\x0FBa\x0F'V[\x81R\x01a\x0FMa\x0F'V[\x81R\x01a\x0FXa\x0F'V[\x81R\x01a\x0Fca\x0F'V[\x81RPPV[a\x0Fqa\x0F+V[\x90V[\x90V[a\x0F\x8Ba\x0F\x86a\x0F\x90\x92a\x0FtV[a\x01\xE7V[a\0\xA1V[\x90V[a\x0F\x9D`(a\x0FwV[\x90V[a\x0F\xAFa\x0F\xB5\x91\x93\x92\x93a\0\xA1V[\x92a\0\xA1V[\x82\x01\x80\x92\x11a\x0F\xC0WV[a\x06\xB5V[\x90V[a\x0F\xDCa\x0F\xD7a\x0F\xE1\x92a\x0F\xC5V[a\x01\xE7V[a\x01\xDCV[\x90V[a\x0F\xF0b\x01\xCC\xCCa\x0F\xC8V[\x90V[a\x10\x07a\x10\x02a\x10\x0C\x92a\x01\xDCV[a\x01\xE7V[a\0\xA1V[\x90V[a\x10\x18\x90a\x0F\xF3V[\x90RV[\x91` a\x10=\x92\x94\x93a\x106`@\x82\x01\x96_\x83\x01\x90a\x0C\x0FV[\x01\x90a\x10\x0FV[V[\x90V[a\x10Va\x10Qa\x10[\x92a\x10?V[a\x01\xE7V[a\x01\xDCV[\x90V[a\x10jb\x01Q\x80a\x10BV[\x90V[a\x10ya\x10\x7F\x91a\x01\xDCV[\x91a\x01\xDCV[\x90\x03\x90`\x01\x80`@\x1B\x03\x82\x11a\x10\x91WV[a\x06\xB5V[\x90a\x10\xA0\x90a\x01\xDCV[\x90RV[\x90V[a\x10\xBBa\x10\xB6a\x10\xC0\x92a\x10\xA4V[a\x01\xE7V[a\x01\xDCV[\x90V[a\x10\xCEa\x0E\x10a\x10\xA7V[\x90V[a\x10\xDDa\x10\xE3\x91a\x01\xDCV[\x91a\x01\xDCV[\x01\x90`\x01\x80`@\x1B\x03\x82\x11a\x10\xF4WV[a\x06\xB5V[\x90V[a\x11\x10a\x11\x0Ba\x11\x15\x92a\x10\xF9V[a\x01\xE7V[a\x01\xDCV[\x90V[a\x11#a\x1C a\x10\xFCV[\x90V[\x90V[a\x11=a\x118a\x11B\x92a\x11&V[a\x01\xE7V[a\x01\xDCV[\x90V[a\x11O`\x0Ca\x11)V[\x90V[a\x11\\\x90Qa\x01\xDCV[\x90V[`\x08a\x11\xA1\x94a\x11\x91\x82\x80\x99\x98\x95\x96a\x11\x89\x82\x87a\x11\x81a\x11\x99\x99\x83\x9Ca\x05\xE8V[\x01\x80\x92a\x05\xE8V[\x01\x80\x92a\x05\xE8V[\x01\x80\x92a\x05\xE8V[\x01\x80\x92a\x05\xE8V[\x01\x90V[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[\x15a\x11\xC0WV[a\x11\xA5V[\x90P\x90V[a\x11\xEFa\x11\xE6\x92` \x92a\x11\xDD\x81a\x02\x81V[\x94\x85\x80\x93a\x11\xC5V[\x93\x84\x91\x01a\0\xF0V[\x01\x90V[a\x12\x01\x90a\x12\x07\x93\x92a\x11\xCAV[\x90a\x11\xCAV[\x90V[a\x12)\x92\x91a\x125\x91a\x12\x1Ba\x005V[\x94\x85\x92` \x84\x01\x92\x83a\x11\xF3V[\x90\x81\x03\x82R\x03\x83a\0]V[V[\x91\x90\x91a\x12Ba\x0F\x16V[Pa\x12Ka\x0FiV[Pa\x12fa\x12Wa\x0F\x93V[a\x12`\x83a\x02\x81V[\x90a\x0F\xA0V[\x80a\x12\x80a\x12za\x12ua\x0F\xE4V[a\x0F\xF3V[\x91a\0\xA1V[\x11a\x14\x1FWPa\x13\xB3\x90a\x12\x92a\x0FiV[\x93Ba\x12\xADa\x12\xA7a\x12\xA2a\x10^V[a\x0F\xF3V[\x91a\0\xA1V[\x11a\x13\xF5W[a\x12\xD9a\x12\xD0a\x12\xC2Ba\x05{V[a\x12\xCAa\x10\xC3V[\x90a\x10\xD1V[` \x87\x01a\x10\x96V[Ca\x12\xF3a\x12\xEDa\x12\xE8a\x11\x18V[a\x0F\xF3V[\x91a\0\xA1V[\x11a\x13\xCAW[a\x13\x1Fa\x13\x16a\x13\x08Ca\x05{V[a\x13\x10a\x11EV[\x90a\x10\xD1V[``\x87\x01a\x10\x96V[a\x13\x83a\x13-_\x87\x01a\x11RV[a\x13ta\x13<` \x89\x01a\x11RV[\x93a\x13I`@\x8A\x01a\x11RV[\x90a\x13_a\x13Y``\x8C\x01a\x11RV[\x91a\x05{V[\x91a\x13ha\x005V[\x96\x87\x95` \x87\x01a\x11_V[` \x82\x01\x81\x03\x82R\x03\x82a\0]V[a\x13\xAEa\x13\x8F\x82a\x02\x81V[a\x13\xA8a\x13\xA2a\x13\x9Da\x0F\x93V[a\0\xA1V[\x91a\0\xA1V[\x14a\x11\xB9V[a\x12\nV[a\x13\xC5a\x13\xBF\x82a\x02\x81V[\x91a\x05uV[ \x91\x90V[a\x13\xF0a\x13\xE7a\x13\xD9Ca\x05{V[a\x13\xE1a\x11\x18V[\x90a\x10mV[`@\x87\x01a\x10\x96V[a\x12\xF9V[a\x14\x1Aa\x14\x12a\x14\x04Ba\x05{V[a\x14\x0Ca\x10^V[\x90a\x10mV[_\x87\x01a\x10\x96V[a\x12\xB3V[a\x14'a\x0F\xE4V[\x90a\x14B_\x92\x83\x92cF4i\x1B`\xE0\x1B\x84R`\x04\x84\x01a\x10\x1CV[\x03\x90\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x0C\xB8V[a\0\x1D_5a\x01{V[\x80b\x84\x12\x0C\x14a\x01vW\x80c\x04\xF1\xC8T\x14a\x01qW\x80c\x05m\xAA\xA6\x14a\x01lW\x80c\x06\x1D\x12\xC0\x14a\x01gW\x80c\x06\xF10V\x14a\x01bW\x80c\x16\xBFUy\x14a\x01]W\x80c\x18\xDB9@\x14a\x01XW\x80c/\x1E\xC5\xE9\x14a\x01SW\x80c<S\xA3\x83\x14a\x01NW\x80cO5\x9A7\x14a\x01IW\x80c\x7F\xA3\xA4\x0E\x14a\x01DW\x80c\xA7\xB5\x1D\x19\x14a\x01?W\x80c\xAD\x9C\x0C.\x14a\x01:W\x80c\xAE\x1A}0\x14a\x015W\x80c\xB7R\xA7\xD1\x14a\x010W\x80c\xD5q\x9D\xC2\x14a\x01+W\x80c\xD5\x95L4\x14a\x01&W\x80c\xD9\xDDg\xAB\x14a\x01!W\x80c\xE1\xD6j\xFE\x14a\x01\x1CW\x80c\xE8\xEB\x1D\xC3\x14a\x01\x17W\x80c\xEC\xA0g\xAD\x14a\x01\x12Wc\xFB\xF6\xEA\xA5\x03a\0\x0EWa\x0C\x83V[a\x0C?V[a\x0C\nV[a\x0B\x9DV[a\x0B.V[a\n\xF5V[a\n\x82V[a\n!V[a\t\xB3V[a\tGV[a\x08\xDBV[a\x07\xF8V[a\x07\xB4V[a\x07KV[a\x06\xB7V[a\x06GV[a\x06\x03V[a\x04\xEDV[a\x04\xB6V[a\x02\xD9V[a\x024V[a\x01\xC3V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01\x99WV[a\x01\x8BV[\x90V[a\x01\xAA\x90a\x01\x9EV[\x90RV[\x91\x90a\x01\xC1\x90_` \x85\x01\x94\x01\x90a\x01\xA1V[V[4a\x01\xF3Wa\x01\xD36`\x04a\x01\x8FV[a\x01\xEFa\x01\xDEa\x0C\xC0V[a\x01\xE6a\x01\x81V[\x91\x82\x91\x82a\x01\xAEV[\x03\x90\xF3[a\x01\x87V[\x1C\x90V[\x90V[a\x02\x0F\x90`\x08a\x02\x14\x93\x02a\x01\xF8V[a\x01\xFCV[\x90V[\x90a\x02\"\x91Ta\x01\xFFV[\x90V[a\x021`\x03_\x90a\x02\x17V[\x90V[4a\x02dWa\x02D6`\x04a\x01\x8FV[a\x02`a\x02Oa\x02%V[a\x02Wa\x01\x81V[\x91\x82\x91\x82a\x01\xAEV[\x03\x90\xF3[a\x01\x87V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\x86\x90`\x08a\x02\x8B\x93\x02a\x01\xF8V[a\x02iV[\x90V[\x90a\x02\x99\x91Ta\x02vV[\x90V[a\x02\xA7__\x90a\x02\x8EV[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\xC0\x90a\x02\xAAV[\x90RV[\x91\x90a\x02\xD7\x90_` \x85\x01\x94\x01\x90a\x02\xB7V[V[4a\x03\tWa\x02\xE96`\x04a\x01\x8FV[a\x03\x05a\x02\xF4a\x02\x9CV[a\x02\xFCa\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x03B\x90a\x03\x1AV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\\W`@RV[a\x03$V[\x90a\x03ta\x03ma\x01\x81V[\x92\x83a\x038V[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\x94Wa\x03\x90` \x91a\x03\x1AV[\x01\x90V[a\x03$V[\x90\x82_\x93\x92\x827\x01RV[\x90\x92\x91\x92a\x03\xB9a\x03\xB4\x82a\x03vV[a\x03aV[\x93\x81\x85R` \x85\x01\x90\x82\x84\x01\x11a\x03\xD5Wa\x03\xD3\x92a\x03\x99V[V[a\x03\x16V[\x90\x80`\x1F\x83\x01\x12\x15a\x03\xF8W\x81` a\x03\xF5\x935\x91\x01a\x03\xA4V[\x90V[a\x03\x12V[a\x04\x06\x81a\x02\xAAV[\x03a\x04\rWV[_\x80\xFD[\x90P5\x90a\x04\x1E\x82a\x03\xFDV[V[a\x04)\x81a\x01\x9EV[\x03a\x040WV[_\x80\xFD[\x90P5\x90a\x04A\x82a\x04 V[V[\x91\x90`\xA0\x83\x82\x03\x12a\x04\xACW_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xA7W\x81a\x04n\x91\x85\x01a\x03\xDAV[\x92a\x04|\x82` \x83\x01a\x04\x11V[\x92a\x04\xA4a\x04\x8D\x84`@\x85\x01a\x044V[\x93a\x04\x9B\x81``\x86\x01a\x04\x11V[\x93`\x80\x01a\x044V[\x90V[a\x03\x0EV[a\x01\x8BV[_\x01\x90V[4a\x04\xE8Wa\x04\xD2a\x04\xC96`\x04a\x04CV[\x93\x92\x90\x92a\x10\xB6V[a\x04\xDAa\x01\x81V[\x80a\x04\xE4\x81a\x04\xB1V[\x03\x90\xF3[a\x01\x87V[4a\x05\x1DWa\x04\xFD6`\x04a\x01\x8FV[a\x05\x19a\x05\x08a\x12\xD5V[a\x05\x10a\x01\x81V[\x91\x82\x91\x82a\x01\xAEV[\x03\x90\xF3[a\x01\x87V[\x90` \x82\x82\x03\x12a\x05;Wa\x058\x91_\x01a\x044V[\x90V[a\x01\x8BV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[T\x90V[_R` _ \x90V[a\x05j\x81a\x05TV[\x82\x10\x15a\x05\x84Wa\x05|`\x01\x91a\x05XV[\x91\x02\x01\x90_\x90V[a\x05@V[\x90V[a\x05\x9C\x90`\x08a\x05\xA1\x93\x02a\x01\xF8V[a\x05\x89V[\x90V[\x90a\x05\xAF\x91Ta\x05\x8CV[\x90V[`\x05a\x05\xBD\x81a\x05TV[\x82\x10\x15a\x05\xDAWa\x05\xD7\x91a\x05\xD1\x91a\x05aV[\x90a\x05\xA4V[\x90V[_\x80\xFD[\x90V[a\x05\xEA\x90a\x05\xDEV[\x90RV[\x91\x90a\x06\x01\x90_` \x85\x01\x94\x01\x90a\x05\xE1V[V[4a\x063Wa\x06/a\x06\x1Ea\x06\x196`\x04a\x05\"V[a\x05\xB2V[a\x06&a\x01\x81V[\x91\x82\x91\x82a\x05\xEEV[\x03\x90\xF3[a\x01\x87V[a\x06D`\x01_\x90a\x02\x17V[\x90V[4a\x06wWa\x06W6`\x04a\x01\x8FV[a\x06sa\x06ba\x068V[a\x06ja\x01\x81V[\x91\x82\x91\x82a\x01\xAEV[\x03\x90\xF3[a\x01\x87V[\x90V[\x90V[a\x06\x96a\x06\x91a\x06\x9B\x92a\x06|V[a\x06\x7FV[a\x02\xAAV[\x90V[a\x06\xA9a\x0E\x10a\x06\x82V[\x90V[a\x06\xB4a\x06\x9EV[\x90V[4a\x06\xE7Wa\x06\xC76`\x04a\x01\x8FV[a\x06\xE3a\x06\xD2a\x06\xACV[a\x06\xDAa\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[\x90V[`\xFF\x16\x90V[a\x07\ta\x07\x04a\x07\x0E\x92a\x06\xECV[a\x06\x7FV[a\x06\xEFV[\x90V[a\x07\x1B`\x0Ca\x06\xF5V[\x90V[a\x07&a\x07\x11V[\x90V[a\x072\x90a\x06\xEFV[\x90RV[\x91\x90a\x07I\x90_` \x85\x01\x94\x01\x90a\x07)V[V[4a\x07{Wa\x07[6`\x04a\x01\x8FV[a\x07wa\x07fa\x07\x1EV[a\x07na\x01\x81V[\x91\x82\x91\x82a\x076V[\x03\x90\xF3[a\x01\x87V[a\x07\x94a\x07\x8Fa\x07\x99\x92a\x06\xECV[a\x06\x7FV[a\x02\xAAV[\x90V[a\x07\xA6`\x0Ca\x07\x80V[\x90V[a\x07\xB1a\x07\x9CV[\x90V[4a\x07\xE4Wa\x07\xC46`\x04a\x01\x8FV[a\x07\xE0a\x07\xCFa\x07\xA9V[a\x07\xD7a\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[a\x07\xF5`\x06_\x90a\x02\x17V[\x90V[4a\x08(Wa\x08\x086`\x04a\x01\x8FV[a\x08$a\x08\x13a\x07\xE9V[a\x08\x1Ba\x01\x81V[\x91\x82\x91\x82a\x01\xAEV[\x03\x90\xF3[a\x01\x87V[a\x086\x81a\x06\xEFV[\x03a\x08=WV[_\x80\xFD[\x90P5\x90a\x08N\x82a\x08-V[V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08d\x90a\x08PV[\x90V[a\x08p\x81a\x08[V[\x03a\x08wWV[_\x80\xFD[\x90P5\x90a\x08\x88\x82a\x08gV[V[\x91``\x83\x83\x03\x12a\x08\xD6Wa\x08\xA1\x82_\x85\x01a\x08AV[\x92a\x08\xAF\x83` \x83\x01a\x08{V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08\xD1Wa\x08\xCE\x92\x01a\x03\xDAV[\x90V[a\x03\x0EV[a\x01\x8BV[4a\t\nWa\x08\xF4a\x08\xEE6`\x04a\x08\x8AV[\x91a\x14\xCCV[a\x08\xFCa\x01\x81V[\x80a\t\x06\x81a\x04\xB1V[\x03\x90\xF3[a\x01\x87V[\x90V[a\t&a\t!a\t+\x92a\t\x0FV[a\x06\x7FV[a\x02\xAAV[\x90V[a\t9a\x1C a\t\x12V[\x90V[a\tDa\t.V[\x90V[4a\twWa\tW6`\x04a\x01\x8FV[a\tsa\tba\t<V[a\tja\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[\x90V[a\t\x93a\t\x8Ea\t\x98\x92a\t|V[a\x06\x7FV[a\x06\xEFV[\x90V[a\t\xA5`\x0Ba\t\x7FV[\x90V[a\t\xB0a\t\x9BV[\x90V[4a\t\xE3Wa\t\xC36`\x04a\x01\x8FV[a\t\xDFa\t\xCEa\t\xA8V[a\t\xD6a\x01\x81V[\x91\x82\x91\x82a\x076V[\x03\x90\xF3[a\x01\x87V[\x90V[a\t\xFFa\t\xFAa\n\x04\x92a\t\xE8V[a\x06\x7FV[a\x02\xAAV[\x90V[a\n\x13b\x01Q\x80a\t\xEBV[\x90V[a\n\x1Ea\n\x07V[\x90V[4a\nQWa\n16`\x04a\x01\x8FV[a\nMa\n<a\n\x16V[a\nDa\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[`\x04a\na\x81a\x05TV[\x82\x10\x15a\n~Wa\n{\x91a\nu\x91a\x05aV[\x90a\x05\xA4V[\x90V[_\x80\xFD[4a\n\xB2Wa\n\xAEa\n\x9Da\n\x986`\x04a\x05\"V[a\nVV[a\n\xA5a\x01\x81V[\x91\x82\x91\x82a\x05\xEEV[\x03\x90\xF3[a\x01\x87V[a\n\xECa\n\xF3\x94a\n\xE2``\x94\x98\x97\x95a\n\xD8`\x80\x86\x01\x9A_\x87\x01\x90a\x02\xB7V[` \x85\x01\x90a\x01\xA1V[`@\x83\x01\x90a\x02\xB7V[\x01\x90a\x01\xA1V[V[4a\x0B)Wa\x0B\x056`\x04a\x01\x8FV[a\x0B%a\x0B\x10a\x16\xC4V[\x90a\x0B\x1C\x94\x92\x94a\x01\x81V[\x94\x85\x94\x85a\n\xB7V[\x03\x90\xF3[a\x01\x87V[4a\x0B^Wa\x0BZa\x0BIa\x0BD6`\x04a\x05\"V[a\x17\x1DV[a\x0BQa\x01\x81V[\x91\x82\x91\x82a\x05\xEEV[\x03\x90\xF3[a\x01\x87V[\x90\x91``\x82\x84\x03\x12a\x0B\x98Wa\x0B\x95a\x0B~\x84_\x85\x01a\x08{V[\x93a\x0B\x8C\x81` \x86\x01a\x08{V[\x93`@\x01a\x044V[\x90V[a\x01\x8BV[4a\x0B\xCCWa\x0B\xB6a\x0B\xB06`\x04a\x0BcV[\x91a\x17\\V[a\x0B\xBEa\x01\x81V[\x80a\x0B\xC8\x81a\x04\xB1V[\x03\x90\xF3[a\x01\x87V[\x90V[a\x0B\xE8a\x0B\xE3a\x0B\xED\x92a\x0B\xD1V[a\x06\x7FV[a\x02\xAAV[\x90V[a\x0B\xFCb\x01\xCC\xCCa\x0B\xD4V[\x90V[a\x0C\x07a\x0B\xF0V[\x90V[4a\x0C:Wa\x0C\x1A6`\x04a\x01\x8FV[a\x0C6a\x0C%a\x0B\xFFV[a\x0C-a\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[4a\x0CoWa\x0CO6`\x04a\x01\x8FV[a\x0Cka\x0CZa\x17\x9CV[a\x0Cba\x01\x81V[\x91\x82\x91\x82a\x01\xAEV[\x03\x90\xF3[a\x01\x87V[a\x0C\x80`\x02_\x90a\x02\x8EV[\x90V[4a\x0C\xB3Wa\x0C\x936`\x04a\x01\x8FV[a\x0C\xAFa\x0C\x9Ea\x0CtV[a\x0C\xA6a\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[_\x80\xFD[_\x90V[a\x0C\xC8a\x0C\xBCV[Pa\x0C\xD3`\x05a\x05TV[\x90V[_\x1B\x90V[\x90a\x0C\xEEg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x0C\xD6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\r\x0Ca\r\x07a\r\x11\x92a\x02\xAAV[a\x06\x7FV[a\x02\xAAV[\x90V[\x90V[\x90a\r,a\r'a\r3\x92a\x0C\xF8V[a\r\x14V[\x82Ta\x0C\xDBV[\x90UV[\x90a\rC_\x19\x91a\x0C\xD6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\raa\r\\a\rf\x92a\x01\x9EV[a\x06\x7FV[a\x01\x9EV[\x90V[\x90V[\x90a\r\x81a\r|a\r\x88\x92a\rMV[a\riV[\x82Ta\r7V[\x90UV[\x90V[a\r\xA3a\r\x9Ea\r\xA8\x92a\r\x8CV[a\x06\x7FV[a\x02\xAAV[\x90V[a\r\xBFa\r\xBAa\r\xC4\x92a\r\x8CV[a\x0C\xD6V[a\x05\xDEV[\x90V[a\r\xDBa\r\xD6a\r\xE0\x92a\r\x8CV[a\x06\x7FV[a\x01\x9EV[\x90V[\x90V[a\r\xFAa\r\xF5a\r\xFF\x92a\r\xE3V[a\x06\x7FV[a\x01\x9EV[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x0E%a\x0E+\x91\x93\x92\x93a\x01\x9EV[\x92a\x01\x9EV[\x82\x03\x91\x82\x11a\x0E6WV[a\x0E\x02V[\x90V[a\x0EJa\x0EO\x91a\x05\xDEV[a\x0E;V[\x90RV[` \x93\x92a\x0Er\x85\x83a\x0Ej\x82\x95a\x0Ez\x97a\x0E>V[\x01\x80\x92a\x0E>V[\x01\x80\x92a\x0E>V[\x01\x90V[` \x01\x90V[Q\x90V[\x90V[_R` _ \x90V[T\x90V[a\x0E\xA1\x81a\x0E\x94V[\x82\x10\x15a\x0E\xBBWa\x0E\xB3`\x01\x91a\x0E\x8BV[\x91\x02\x01\x90_\x90V[a\x05@V[\x1B\x90V[\x91\x90`\x08a\x0E\xDF\x91\x02\x91a\x0E\xD9_\x19\x84a\x0E\xC0V[\x92a\x0E\xC0V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x0E\xF2\x90a\x05\xDEV[\x90V[_\x1C\x90V[a\x0F\x03\x90a\x0E\xF5V[\x90V[\x91\x90a\x0F\x1Ca\x0F\x17a\x0F$\x93a\x0E\xE9V[a\x0E\xFAV[\x90\x83Ta\x0E\xC4V[\x90UV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x0FXW\x82a\x0FP\x91`\x01a\x0FV\x95\x01\x81Ua\x0E\x98V[\x90a\x0F\x06V[V[a\x03$V[a\x0Fia\x0Fn\x91a\x0E\xF5V[a\x01\xFCV[\x90V[a\x0F{\x90Ta\x0F]V[\x90V[a\x0F\x87\x90a\x02\xAAV[\x90RV[\x90``\x80a\x0F\xD1\x93a\x0F\xA3_\x82\x01Q_\x86\x01\x90a\x0F~V[a\x0F\xB5` \x82\x01Q` \x86\x01\x90a\x0F~V[a\x0F\xC7`@\x82\x01Q`@\x86\x01\x90a\x0F~V[\x01Q\x91\x01\x90a\x0F~V[V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x05\x11\x15a\x0F\xF1WV[a\x0F\xD3V[\x90a\x10\0\x82a\x0F\xE7V[V[a\x10\x0B\x90a\x0F\xF6V[\x90V[a\x10\x17\x90a\x10\x02V[\x90RV[a\x10Pa\x10W\x94a\x10F`\xC0\x94\x98\x97\x95a\x10<`\xE0\x86\x01\x9A_\x87\x01\x90a\x05\xE1V[` \x85\x01\x90a\x01\xA1V[`@\x83\x01\x90a\x0F\x8BV[\x01\x90a\x10\x0EV[V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[a\x10\x8Ca\x10\x95` \x93a\x10\x9A\x93a\x10\x83\x81a\x0E\x84V[\x93\x84\x80\x93a\x10YV[\x95\x86\x91\x01a\x10bV[a\x03\x1AV[\x01\x90V[a\x10\xB3\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x10mV[\x90V[\x93\x91a\x10\xC5a\x10\xCC\x92_a\r\x17V[`\x01a\rlV[\x81a\x10\xDFa\x10\xD9_a\r\x8FV[\x91a\x02\xAAV[\x11a\x12\xB9W[PPa\x10\xF1`\x04a\x05TV[a\x10\xFC\x82\x82\x90a\x19\xF1V[\x90a\x11\x07`\x05a\x05TV[\x92a\x11\x11_a\r\xABV[\x84a\x11$a\x11\x1E_a\r\xC7V[\x91a\x01\x9EV[\x11a\x12\x8BW[a\x113_a\r\xABV[\x92\x82a\x11Ga\x11A_a\r\xC7V[\x91a\x01\x9EV[\x11a\x12VW[a\x11\xAB\x90a\x11|\x83a\x11m\x87a\x11aa\x01\x81V[\x94\x85\x93` \x85\x01a\x0ESV[` \x82\x01\x81\x03\x82R\x03\x82a\x038V[a\x11\x8Ea\x11\x88\x82a\x0E\x84V[\x91a\x0E~V[ \x92a\x11\xA4a\x11\x9D`\x05a\x0E\x88V[\x85\x90a\x0F(V[`\x06a\rlV[\x84\x90\x91\x92\x93a\x11\xBA`\x06a\x0FqV[\x90\x94a\x12\x0F`\x01a\x11\xFDa\x11\xF7a\x11\xF1\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x97a\rMV[\x97a\x0E\xE9V[\x97a\x0E\xE9V[\x97a\x12\x06a\x01\x81V[\x94\x85\x94\x85a\x10\x1BV[\x03\x90\xA4a\x12Qa\x12?\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x92a\rMV[\x92a\x12Ha\x01\x81V[\x91\x82\x91\x82a\x10\x9EV[\x03\x90\xA2V[\x92Pa\x11\xABa\x12\x83a\x12}`\x04a\x12w\x86a\x12q`\x01a\r\xE6V[\x90a\x0E\x16V[\x90a\x05aV[\x90a\x05\xA4V[\x93\x90Pa\x11MV[Pa\x12\xB4a\x12\xAE`\x05a\x12\xA8\x87a\x12\xA2`\x01a\r\xE6V[\x90a\x0E\x16V[\x90a\x05aV[\x90a\x05\xA4V[a\x11*V[a\x12\xC7a\x12\xCE\x92`\x02a\r\x17V[`\x03a\rlV[_\x80a\x10\xE5V[a\x12\xDDa\x0C\xBCV[Pa\x12\xE8`\x05a\x05TV[\x90V[a\x12\xFFa\x12\xFAa\x13\x04\x92a\x01\x9EV[a\x06\x7FV[a\x02\xAAV[\x90V[`\xF8\x1B\x90V[a\x13\x16\x90a\x13\x07V[\x90V[a\x13%a\x13*\x91a\x06\xEFV[a\x13\rV[\x90RV[``\x1B\x90V[a\x13=\x90a\x13.V[\x90V[a\x13I\x90a\x134V[\x90V[a\x13Xa\x13]\x91a\x08[V[a\x13@V[\x90RV[`\xC0\x1B\x90V[a\x13p\x90a\x13aV[\x90V[a\x13\x7Fa\x13\x84\x91a\x02\xAAV[a\x13gV[\x90RV[\x90V[a\x13\x97a\x13\x9C\x91a\x01\x9EV[a\x13\x88V[\x90RV[\x94a\x13\xF1`\x08` \x99\x98\x95\x96a\x13\xE9\x82\x8C\x99a\x13\xE1`\x14a\x13\xF9\x9Aa\x13\xD9a\x14\x01\x9F\x8F\x9C\x90a\x13\xD1\x81`\x01\x93a\x13\x19V[\x01\x80\x92a\x13LV[\x01\x80\x92a\x13sV[\x01\x80\x92a\x13sV[\x01\x80\x92a\x13\x8BV[\x01\x80\x92a\x13\x8BV[\x01\x80\x92a\x0E>V[\x01\x90V[` \x81a\x14\x17a\x14\x1F\x93\x83\x96\x95a\x0E>V[\x01\x80\x92a\x0E>V[\x01\x90V[a\x147a\x142a\x14<\x92a\x08PV[a\x06\x7FV[a\x08PV[\x90V[a\x14H\x90a\x14#V[\x90V[a\x14T\x90a\x14?V[\x90V[a\x14`\x90a\x08[V[\x90RV[a\x14m\x90a\r\xC7V[\x90RV[\x91\x94a\x14\xB9a\x14\xC3\x92\x98\x97\x95a\x14\xAF`\xA0\x96a\x14\xA5a\x14\xCA\x9Aa\x14\x9B`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x14WV[` \x89\x01\x90a\x07)V[`@\x87\x01\x90a\x14WV[``\x85\x01\x90a\x05\xE1V[`\x80\x83\x01\x90a\x14dV[\x01\x90a\x02\xB7V[V[\x90a\x14\xD7`\x04a\x05TV[\x91\x83a\x14\xEBa\x14\xE5\x82a\x0E\x84V[\x91a\x0E~V[ \x91\x81a\x158\x82\x91a\x15)a\x14\xFFCa\x12\xEBV[a\x15\x08Ba\x12\xEBV[\x89a\x15\x12_a\r\xC7V[\x91\x8A\x93a\x15\x1Da\x01\x81V[\x98\x89\x97` \x89\x01a\x13\xA0V[` \x82\x01\x81\x03\x82R\x03\x82a\x038V[a\x15Ja\x15D\x82a\x0E\x84V[\x91a\x0E~V[ \x90a\x15U_a\r\xABV[\x91\x85a\x15ia\x15c_a\r\xC7V[\x91a\x01\x9EV[\x11a\x16jW[a\x15\xC1\x90a\x15}`\x04a\x0E\x88V[\x90a\x15\xA8\x85a\x15\x99a\x15\x8Da\x01\x81V[\x93\x84\x92` \x84\x01a\x14\x05V[` \x82\x01\x81\x03\x82R\x03\x82a\x038V[a\x15\xBAa\x15\xB4\x82a\x0E\x84V[\x91a\x0E~V[ \x90a\x0F(V[\x84\x91\x92a\x16#a\x15\xD00a\x14KV[\x91\x92\x95_a\x15\xDDBa\x12\xEBV[\x91a\x16\x11a\x16\x0B\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE1\x98a\rMV[\x98a\x0E\xE9V[\x98a\x16\x1Aa\x01\x81V[\x96\x87\x96\x87a\x14qV[\x03\x90\xA3a\x16ea\x16S\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x92a\rMV[\x92a\x16\\a\x01\x81V[\x91\x82\x91\x82a\x10\x9EV[\x03\x90\xA2V[\x91Pa\x15\xC1a\x16\x97a\x16\x91`\x04a\x16\x8B\x89a\x16\x85`\x01a\r\xE6V[\x90a\x0E\x16V[\x90a\x05aV[\x90a\x05\xA4V[\x92\x90Pa\x15oV[_\x90V[a\x16\xAFa\x16\xB4\x91a\x0E\xF5V[a\x02iV[\x90V[a\x16\xC1\x90Ta\x16\xA3V[\x90V[a\x16\xCCa\x16\x9FV[Pa\x16\xD5a\x0C\xBCV[Pa\x16\xDEa\x16\x9FV[Pa\x16\xE7a\x0C\xBCV[Pa\x16\xF1_a\x16\xB7V[\x90a\x16\xFC`\x01a\x0FqV[\x91a\x17\x07`\x02a\x16\xB7V[\x91a\x17\x12`\x03a\x0FqV[\x91\x93\x92\x91\x90V[_\x90V[a\x174a\x17:\x91a\x17,a\x17\x19V[P`\x05a\x05aV[\x90a\x05\xA4V[\x90V[`\x14\x81a\x17Pa\x17X\x93` \x96\x95a\x13LV[\x01\x80\x92a\x13\x8BV[\x01\x90V[\x90a\x17\x9A\x92\x91a\x17\x95a\x17ma\x07\x11V[\x91\x92a\x17\x86a\x17za\x01\x81V[\x95\x86\x92` \x84\x01a\x17=V[` \x82\x01\x81\x03\x82R\x03\x84a\x038V[a\x14\xCCV[V[a\x17\xA4a\x0C\xBCV[Pa\x17\xAF`\x04a\x05TV[\x90V[a\x17\xBC`\x80a\x03aV[\x90V[_\x90V[a\x17\xCBa\x17\xB2V[\x90` \x80\x80\x80\x85a\x17\xDAa\x17\xBFV[\x81R\x01a\x17\xE5a\x17\xBFV[\x81R\x01a\x17\xF0a\x17\xBFV[\x81R\x01a\x17\xFBa\x17\xBFV[\x81RPPV[a\x18\ta\x17\xC3V[\x90V[\x90V[a\x18#a\x18\x1Ea\x18(\x92a\x18\x0CV[a\x06\x7FV[a\x01\x9EV[\x90V[a\x185`(a\x18\x0FV[\x90V[a\x18Ga\x18M\x91\x93\x92\x93a\x01\x9EV[\x92a\x01\x9EV[\x82\x01\x80\x92\x11a\x18XWV[a\x0E\x02V[a\x18qa\x18la\x18v\x92a\x02\xAAV[a\x06\x7FV[a\x01\x9EV[\x90V[a\x18\x82\x90a\x18]V[\x90RV[\x91` a\x18\xA7\x92\x94\x93a\x18\xA0`@\x82\x01\x96_\x83\x01\x90a\x01\xA1V[\x01\x90a\x18yV[V[a\x18\xB5a\x18\xBB\x91a\x02\xAAV[\x91a\x02\xAAV[\x90\x03\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x18\xCFWV[a\x0E\x02V[\x90a\x18\xDE\x90a\x02\xAAV[\x90RV[a\x18\xEEa\x18\xF4\x91a\x02\xAAV[\x91a\x02\xAAV[\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x19\x07WV[a\x0E\x02V[a\x19\x16\x90Qa\x02\xAAV[\x90V[`\x08a\x19[\x94a\x19K\x82\x80\x99\x98\x95\x96a\x19C\x82\x87a\x19;a\x19S\x99\x83\x9Ca\x13sV[\x01\x80\x92a\x13sV[\x01\x80\x92a\x13sV[\x01\x80\x92a\x13sV[\x01\x80\x92a\x13sV[\x01\x90V[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[\x15a\x19zWV[a\x19_V[\x90P\x90V[a\x19\xA9a\x19\xA0\x92` \x92a\x19\x97\x81a\x0E\x84V[\x94\x85\x80\x93a\x19\x7FV[\x93\x84\x91\x01a\x10bV[\x01\x90V[a\x19\xBB\x90a\x19\xC1\x93\x92a\x19\x84V[\x90a\x19\x84V[\x90V[a\x19\xE3\x92\x91a\x19\xEF\x91a\x19\xD5a\x01\x81V[\x94\x85\x92` \x84\x01\x92\x83a\x19\xADV[\x90\x81\x03\x82R\x03\x83a\x038V[V[\x91\x90\x91a\x19\xFCa\x17\x19V[Pa\x1A\x05a\x18\x01V[Pa\x1A a\x1A\x11a\x18+V[a\x1A\x1A\x83a\x0E\x84V[\x90a\x188V[\x80a\x1A:a\x1A4a\x1A/a\x0B\xF0V[a\x18]V[\x91a\x01\x9EV[\x11a\x1B\xD9WPa\x1Bm\x90a\x1ALa\x18\x01V[\x93Ba\x1Aga\x1Aaa\x1A\\a\n\x07V[a\x18]V[\x91a\x01\x9EV[\x11a\x1B\xAFW[a\x1A\x93a\x1A\x8Aa\x1A|Ba\x12\xEBV[a\x1A\x84a\x06\x9EV[\x90a\x18\xE2V[` \x87\x01a\x18\xD4V[Ca\x1A\xADa\x1A\xA7a\x1A\xA2a\t.V[a\x18]V[\x91a\x01\x9EV[\x11a\x1B\x84W[a\x1A\xD9a\x1A\xD0a\x1A\xC2Ca\x12\xEBV[a\x1A\xCAa\x07\x9CV[\x90a\x18\xE2V[``\x87\x01a\x18\xD4V[a\x1B=a\x1A\xE7_\x87\x01a\x19\x0CV[a\x1B.a\x1A\xF6` \x89\x01a\x19\x0CV[\x93a\x1B\x03`@\x8A\x01a\x19\x0CV[\x90a\x1B\x19a\x1B\x13``\x8C\x01a\x19\x0CV[\x91a\x12\xEBV[\x91a\x1B\"a\x01\x81V[\x96\x87\x95` \x87\x01a\x19\x19V[` \x82\x01\x81\x03\x82R\x03\x82a\x038V[a\x1Bha\x1BI\x82a\x0E\x84V[a\x1Bba\x1B\\a\x1BWa\x18+V[a\x01\x9EV[\x91a\x01\x9EV[\x14a\x19sV[a\x19\xC4V[a\x1B\x7Fa\x1By\x82a\x0E\x84V[\x91a\x0E~V[ \x91\x90V[a\x1B\xAAa\x1B\xA1a\x1B\x93Ca\x12\xEBV[a\x1B\x9Ba\t.V[\x90a\x18\xA9V[`@\x87\x01a\x18\xD4V[a\x1A\xB3V[a\x1B\xD4a\x1B\xCCa\x1B\xBEBa\x12\xEBV[a\x1B\xC6a\n\x07V[\x90a\x18\xA9V[_\x87\x01a\x18\xD4V[a\x1AmV[a\x1B\xE1a\x0B\xF0V[\x90a\x1B\xFC_\x92\x83\x92cF4i\x1B`\xE0\x1B\x84R`\x04\x84\x01a\x18\x86V[\x03\x90\xFD",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b610cb8565b61001d5f3561017b565b806284120c1461017657806304f1c85414610171578063056daaa61461016c578063061d12c01461016757806306f130561461016257806316bf55791461015d57806318db3940146101585780632f1ec5e9146101535780633c53a3831461014e5780634f359a37146101495780637fa3a40e14610144578063a7b51d191461013f578063ad9c0c2e1461013a578063ae1a7d3014610135578063b752a7d114610130578063d5719dc21461012b578063d5954c3414610126578063d9dd67ab14610121578063e1d66afe1461011c578063e8eb1dc314610117578063eca067ad146101125763fbf6eaa50361000e57610c83565b610c3f565b610c0a565b610b9d565b610b2e565b610af5565b610a82565b610a21565b6109b3565b610947565b6108db565b6107f8565b6107b4565b61074b565b6106b7565b610647565b610603565b6104ed565b6104b6565b6102d9565b610234565b6101c3565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f91031261019957565b61018b565b90565b6101aa9061019e565b9052565b91906101c1905f602085019401906101a1565b565b346101f3576101d336600461018f565b6101ef6101de610cc0565b6101e6610181565b918291826101ae565b0390f35b610187565b1c90565b90565b61020f90600861021493026101f8565b6101fc565b90565b9061022291546101ff565b90565b61023160035f90610217565b90565b346102645761024436600461018f565b61026061024f610225565b610257610181565b918291826101ae565b0390f35b610187565b67ffffffffffffffff1690565b61028690600861028b93026101f8565b610269565b90565b906102999154610276565b90565b6102a75f5f9061028e565b90565b67ffffffffffffffff1690565b6102c0906102aa565b9052565b91906102d7905f602085019401906102b7565b565b34610309576102e936600461018f565b6103056102f461029c565b6102fc610181565b918291826102c4565b0390f35b610187565b5f80fd5b5f80fd5b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906103429061031a565b810190811067ffffffffffffffff82111761035c57604052565b610324565b9061037461036d610181565b9283610338565b565b67ffffffffffffffff81116103945761039060209161031a565b0190565b610324565b90825f939282370152565b909291926103b96103b482610376565b610361565b938185526020850190828401116103d5576103d392610399565b565b610316565b9080601f830112156103f8578160206103f5933591016103a4565b90565b610312565b610406816102aa565b0361040d57565b5f80fd5b9050359061041e826103fd565b565b6104298161019e565b0361043057565b5f80fd5b9050359061044182610420565b565b919060a0838203126104ac575f83013567ffffffffffffffff81116104a7578161046e9185016103da565b9261047c8260208301610411565b926104a461048d8460408501610434565b9361049b8160608601610411565b93608001610434565b90565b61030e565b61018b565b5f0190565b346104e8576104d26104c9366004610443565b939290926110b6565b6104da610181565b806104e4816104b1565b0390f35b610187565b3461051d576104fd36600461018f565b6105196105086112d5565b610510610181565b918291826101ae565b0390f35b610187565b9060208282031261053b57610538915f01610434565b90565b61018b565b634e487b7160e01b5f52603260045260245ffd5b5490565b5f5260205f2090565b61056a81610554565b8210156105845761057c600191610558565b910201905f90565b610540565b90565b61059c9060086105a193026101f8565b610589565b90565b906105af915461058c565b90565b60056105bd81610554565b8210156105da576105d7916105d191610561565b906105a4565b90565b5f80fd5b90565b6105ea906105de565b9052565b9190610601905f602085019401906105e1565b565b346106335761062f61061e610619366004610522565b6105b2565b610626610181565b918291826105ee565b0390f35b610187565b61064460015f90610217565b90565b346106775761065736600461018f565b610673610662610638565b61066a610181565b918291826101ae565b0390f35b610187565b90565b90565b61069661069161069b9261067c565b61067f565b6102aa565b90565b6106a9610e10610682565b90565b6106b461069e565b90565b346106e7576106c736600461018f565b6106e36106d26106ac565b6106da610181565b918291826102c4565b0390f35b610187565b90565b60ff1690565b61070961070461070e926106ec565b61067f565b6106ef565b90565b61071b600c6106f5565b90565b610726610711565b90565b610732906106ef565b9052565b9190610749905f60208501940190610729565b565b3461077b5761075b36600461018f565b61077761076661071e565b61076e610181565b91829182610736565b0390f35b610187565b61079461078f610799926106ec565b61067f565b6102aa565b90565b6107a6600c610780565b90565b6107b161079c565b90565b346107e4576107c436600461018f565b6107e06107cf6107a9565b6107d7610181565b918291826102c4565b0390f35b610187565b6107f560065f90610217565b90565b346108285761080836600461018f565b6108246108136107e9565b61081b610181565b918291826101ae565b0390f35b610187565b610836816106ef565b0361083d57565b5f80fd5b9050359061084e8261082d565b565b60018060a01b031690565b61086490610850565b90565b6108708161085b565b0361087757565b5f80fd5b9050359061088882610867565b565b916060838303126108d6576108a1825f8501610841565b926108af836020830161087b565b92604082013567ffffffffffffffff81116108d1576108ce92016103da565b90565b61030e565b61018b565b3461090a576108f46108ee36600461088a565b916114cc565b6108fc610181565b80610906816104b1565b0390f35b610187565b90565b61092661092161092b9261090f565b61067f565b6102aa565b90565b610939611c20610912565b90565b61094461092e565b90565b346109775761095736600461018f565b61097361096261093c565b61096a610181565b918291826102c4565b0390f35b610187565b90565b61099361098e6109989261097c565b61067f565b6106ef565b90565b6109a5600b61097f565b90565b6109b061099b565b90565b346109e3576109c336600461018f565b6109df6109ce6109a8565b6109d6610181565b91829182610736565b0390f35b610187565b90565b6109ff6109fa610a04926109e8565b61067f565b6102aa565b90565b610a13620151806109eb565b90565b610a1e610a07565b90565b34610a5157610a3136600461018f565b610a4d610a3c610a16565b610a44610181565b918291826102c4565b0390f35b610187565b6004610a6181610554565b821015610a7e57610a7b91610a7591610561565b906105a4565b90565b5f80fd5b34610ab257610aae610a9d610a98366004610522565b610a56565b610aa5610181565b918291826105ee565b0390f35b610187565b610aec610af394610ae2606094989795610ad8608086019a5f8701906102b7565b60208501906101a1565b60408301906102b7565b01906101a1565b565b34610b2957610b0536600461018f565b610b25610b106116c4565b90610b1c949294610181565b94859485610ab7565b0390f35b610187565b34610b5e57610b5a610b49610b44366004610522565b61171d565b610b51610181565b918291826105ee565b0390f35b610187565b9091606082840312610b9857610b95610b7e845f850161087b565b93610b8c816020860161087b565b93604001610434565b90565b61018b565b34610bcc57610bb6610bb0366004610b63565b9161175c565b610bbe610181565b80610bc8816104b1565b0390f35b610187565b90565b610be8610be3610bed92610bd1565b61067f565b6102aa565b90565b610bfc6201cccc610bd4565b90565b610c07610bf0565b90565b34610c3a57610c1a36600461018f565b610c36610c25610bff565b610c2d610181565b918291826102c4565b0390f35b610187565b34610c6f57610c4f36600461018f565b610c6b610c5a61179c565b610c62610181565b918291826101ae565b0390f35b610187565b610c8060025f9061028e565b90565b34610cb357610c9336600461018f565b610caf610c9e610c74565b610ca6610181565b918291826102c4565b0390f35b610187565b5f80fd5b5f90565b610cc8610cbc565b50610cd36005610554565b90565b5f1b90565b90610cee67ffffffffffffffff91610cd6565b9181191691161790565b610d0c610d07610d11926102aa565b61067f565b6102aa565b90565b90565b90610d2c610d27610d3392610cf8565b610d14565b8254610cdb565b9055565b90610d435f1991610cd6565b9181191691161790565b610d61610d5c610d669261019e565b61067f565b61019e565b90565b90565b90610d81610d7c610d8892610d4d565b610d69565b8254610d37565b9055565b90565b610da3610d9e610da892610d8c565b61067f565b6102aa565b90565b610dbf610dba610dc492610d8c565b610cd6565b6105de565b90565b610ddb610dd6610de092610d8c565b61067f565b61019e565b90565b90565b610dfa610df5610dff92610de3565b61067f565b61019e565b90565b634e487b7160e01b5f52601160045260245ffd5b610e25610e2b9193929361019e565b9261019e565b8203918211610e3657565b610e02565b90565b610e4a610e4f916105de565b610e3b565b9052565b60209392610e728583610e6a8295610e7a97610e3e565b018092610e3e565b018092610e3e565b0190565b60200190565b5190565b90565b5f5260205f2090565b5490565b610ea181610e94565b821015610ebb57610eb3600191610e8b565b910201905f90565b610540565b1b90565b91906008610edf910291610ed95f1984610ec0565b92610ec0565b9181191691161790565b610ef2906105de565b90565b5f1c90565b610f0390610ef5565b90565b9190610f1c610f17610f2493610ee9565b610efa565b908354610ec4565b9055565b9081549168010000000000000000831015610f585782610f50916001610f5695018155610e98565b90610f06565b565b610324565b610f69610f6e91610ef5565b6101fc565b90565b610f7b9054610f5d565b90565b610f87906102aa565b9052565b90606080610fd193610fa35f8201515f860190610f7e565b610fb560208201516020860190610f7e565b610fc760408201516040860190610f7e565b0151910190610f7e565b565b634e487b7160e01b5f52602160045260245ffd5b60051115610ff157565b610fd3565b9061100082610fe7565b565b61100b90610ff6565b90565b61101790611002565b9052565b6110506110579461104660c09498979561103c60e086019a5f8701906105e1565b60208501906101a1565b6040830190610f8b565b019061100e565b565b60209181520190565b90825f9392825e0152565b61108c61109560209361109a9361108381610e84565b93848093611059565b95869101611062565b61031a565b0190565b6110b39160208201915f81840391015261106d565b90565b93916110c56110cc925f610d17565b6001610d6c565b816110df6110d95f610d8f565b916102aa565b116112b9575b50506110f16004610554565b6110fc8282906119f1565b906111076005610554565b926111115f610dab565b8461112461111e5f610dc7565b9161019e565b1161128b575b6111335f610dab565b92826111476111415f610dc7565b9161019e565b11611256575b6111ab9061117c8361116d87611161610181565b94859360208501610e53565b60208201810382520382610338565b61118e61118882610e84565b91610e7e565b20926111a461119d6005610e88565b8590610f28565b6006610d6c565b84909192936111ba6006610f71565b909461120f60016111fd6111f76111f17f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd797610d4d565b97610ee9565b97610ee9565b97611206610181565b9485948561101b565b0390a461125161123f7ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c2092610d4d565b92611248610181565b9182918261109e565b0390a2565b92506111ab61128361127d6004611277866112716001610de6565b90610e16565b90610561565b906105a4565b93905061114d565b506112b46112ae60056112a8876112a26001610de6565b90610e16565b90610561565b906105a4565b61112a565b6112c76112ce926002610d17565b6003610d6c565b5f806110e5565b6112dd610cbc565b506112e86005610554565b90565b6112ff6112fa6113049261019e565b61067f565b6102aa565b90565b60f81b90565b61131690611307565b90565b61132561132a916106ef565b61130d565b9052565b60601b90565b61133d9061132e565b90565b61134990611334565b90565b61135861135d9161085b565b611340565b9052565b60c01b90565b61137090611361565b90565b61137f611384916102aa565b611367565b9052565b90565b61139761139c9161019e565b611388565b9052565b946113f160086020999895966113e9828c996113e160146113f99a6113d96114019f8f9c906113d181600193611319565b01809261134c565b018092611373565b018092611373565b01809261138b565b01809261138b565b018092610e3e565b0190565b60208161141761141f93839695610e3e565b018092610e3e565b0190565b61143761143261143c92610850565b61067f565b610850565b90565b61144890611423565b90565b6114549061143f565b90565b6114609061085b565b9052565b61146d90610dc7565b9052565b91946114b96114c3929897956114af60a0966114a56114ca9a61149b60c08a019e5f8b0190611457565b6020890190610729565b6040870190611457565b60608501906105e1565b6080830190611464565b01906102b7565b565b906114d76004610554565b91836114eb6114e582610e84565b91610e7e565b20918161153882916115296114ff436112eb565b611508426112eb565b896115125f610dc7565b918a9361151d610181565b988997602089016113a0565b60208201810382520382610338565b61154a61154482610e84565b91610e7e565b20906115555f610dab565b91856115696115635f610dc7565b9161019e565b1161166a575b6115c19061157d6004610e88565b906115a88561159961158d610181565b93849260208401611405565b60208201810382520382610338565b6115ba6115b482610e84565b91610e7e565b2090610f28565b8491926116236115d03061144b565b9192955f6115dd426112eb565b9161161161160b7f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe198610d4d565b98610ee9565b9861161a610181565b96879687611471565b0390a36116656116537fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b92610d4d565b9261165c610181565b9182918261109e565b0390a2565b91506115c1611697611691600461168b896116856001610de6565b90610e16565b90610561565b906105a4565b92905061156f565b5f90565b6116af6116b491610ef5565b610269565b90565b6116c190546116a3565b90565b6116cc61169f565b506116d5610cbc565b506116de61169f565b506116e7610cbc565b506116f15f6116b7565b906116fc6001610f71565b9161170760026116b7565b916117126003610f71565b9193929190565b5f90565b61173461173a9161172c611719565b506005610561565b906105a4565b90565b601481611750611758936020969561134c565b01809261138b565b0190565b9061179a929161179561176d610711565b919261178661177a610181565b9586926020840161173d565b60208201810382520384610338565b6114cc565b565b6117a4610cbc565b506117af6004610554565b90565b6117bc6080610361565b90565b5f90565b6117cb6117b2565b906020808080856117da6117bf565b8152016117e56117bf565b8152016117f06117bf565b8152016117fb6117bf565b81525050565b6118096117c3565b90565b90565b61182361181e6118289261180c565b61067f565b61019e565b90565b611835602861180f565b90565b61184761184d9193929361019e565b9261019e565b820180921161185857565b610e02565b61187161186c611876926102aa565b61067f565b61019e565b90565b6118829061185d565b9052565b9160206118a79294936118a060408201965f8301906101a1565b0190611879565b565b6118b56118bb916102aa565b916102aa565b90039067ffffffffffffffff82116118cf57565b610e02565b906118de906102aa565b9052565b6118ee6118f4916102aa565b916102aa565b019067ffffffffffffffff821161190757565b610e02565b61191690516102aa565b90565b600861195b9461194b828099989596611943828761193b61195399839c611373565b018092611373565b018092611373565b018092611373565b018092611373565b0190565b634e487b7160e01b5f52600160045260245ffd5b1561197a57565b61195f565b905090565b6119a96119a09260209261199781610e84565b9485809361197f565b93849101611062565b0190565b6119bb906119c19392611984565b90611984565b90565b6119e392916119ef916119d5610181565b9485926020840192836119ad565b90810382520383610338565b565b9190916119fc611719565b50611a05611801565b50611a20611a1161182b565b611a1a83610e84565b90611838565b80611a3a611a34611a2f610bf0565b61185d565b9161019e565b11611bd95750611b6d90611a4c611801565b9342611a67611a61611a5c610a07565b61185d565b9161019e565b11611baf575b611a93611a8a611a7c426112eb565b611a8461069e565b906118e2565b602087016118d4565b43611aad611aa7611aa261092e565b61185d565b9161019e565b11611b84575b611ad9611ad0611ac2436112eb565b611aca61079c565b906118e2565b606087016118d4565b611b3d611ae75f870161190c565b611b2e611af66020890161190c565b93611b0360408a0161190c565b90611b19611b1360608c0161190c565b916112eb565b91611b22610181565b96879560208701611919565b60208201810382520382610338565b611b68611b4982610e84565b611b62611b5c611b5761182b565b61019e565b9161019e565b14611973565b6119c4565b611b7f611b7982610e84565b91610e7e565b209190565b611baa611ba1611b93436112eb565b611b9b61092e565b906118a9565b604087016118d4565b611ab3565b611bd4611bcc611bbe426112eb565b611bc6610a07565b906118a9565b5f87016118d4565b611a6d565b611be1610bf0565b90611bfc5f928392634634691b60e01b845260048401611886565b0390fd
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x0C\xB8V[a\0\x1D_5a\x01{V[\x80b\x84\x12\x0C\x14a\x01vW\x80c\x04\xF1\xC8T\x14a\x01qW\x80c\x05m\xAA\xA6\x14a\x01lW\x80c\x06\x1D\x12\xC0\x14a\x01gW\x80c\x06\xF10V\x14a\x01bW\x80c\x16\xBFUy\x14a\x01]W\x80c\x18\xDB9@\x14a\x01XW\x80c/\x1E\xC5\xE9\x14a\x01SW\x80c<S\xA3\x83\x14a\x01NW\x80cO5\x9A7\x14a\x01IW\x80c\x7F\xA3\xA4\x0E\x14a\x01DW\x80c\xA7\xB5\x1D\x19\x14a\x01?W\x80c\xAD\x9C\x0C.\x14a\x01:W\x80c\xAE\x1A}0\x14a\x015W\x80c\xB7R\xA7\xD1\x14a\x010W\x80c\xD5q\x9D\xC2\x14a\x01+W\x80c\xD5\x95L4\x14a\x01&W\x80c\xD9\xDDg\xAB\x14a\x01!W\x80c\xE1\xD6j\xFE\x14a\x01\x1CW\x80c\xE8\xEB\x1D\xC3\x14a\x01\x17W\x80c\xEC\xA0g\xAD\x14a\x01\x12Wc\xFB\xF6\xEA\xA5\x03a\0\x0EWa\x0C\x83V[a\x0C?V[a\x0C\nV[a\x0B\x9DV[a\x0B.V[a\n\xF5V[a\n\x82V[a\n!V[a\t\xB3V[a\tGV[a\x08\xDBV[a\x07\xF8V[a\x07\xB4V[a\x07KV[a\x06\xB7V[a\x06GV[a\x06\x03V[a\x04\xEDV[a\x04\xB6V[a\x02\xD9V[a\x024V[a\x01\xC3V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01\x99WV[a\x01\x8BV[\x90V[a\x01\xAA\x90a\x01\x9EV[\x90RV[\x91\x90a\x01\xC1\x90_` \x85\x01\x94\x01\x90a\x01\xA1V[V[4a\x01\xF3Wa\x01\xD36`\x04a\x01\x8FV[a\x01\xEFa\x01\xDEa\x0C\xC0V[a\x01\xE6a\x01\x81V[\x91\x82\x91\x82a\x01\xAEV[\x03\x90\xF3[a\x01\x87V[\x1C\x90V[\x90V[a\x02\x0F\x90`\x08a\x02\x14\x93\x02a\x01\xF8V[a\x01\xFCV[\x90V[\x90a\x02\"\x91Ta\x01\xFFV[\x90V[a\x021`\x03_\x90a\x02\x17V[\x90V[4a\x02dWa\x02D6`\x04a\x01\x8FV[a\x02`a\x02Oa\x02%V[a\x02Wa\x01\x81V[\x91\x82\x91\x82a\x01\xAEV[\x03\x90\xF3[a\x01\x87V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\x86\x90`\x08a\x02\x8B\x93\x02a\x01\xF8V[a\x02iV[\x90V[\x90a\x02\x99\x91Ta\x02vV[\x90V[a\x02\xA7__\x90a\x02\x8EV[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\xC0\x90a\x02\xAAV[\x90RV[\x91\x90a\x02\xD7\x90_` \x85\x01\x94\x01\x90a\x02\xB7V[V[4a\x03\tWa\x02\xE96`\x04a\x01\x8FV[a\x03\x05a\x02\xF4a\x02\x9CV[a\x02\xFCa\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x03B\x90a\x03\x1AV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\\W`@RV[a\x03$V[\x90a\x03ta\x03ma\x01\x81V[\x92\x83a\x038V[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\x94Wa\x03\x90` \x91a\x03\x1AV[\x01\x90V[a\x03$V[\x90\x82_\x93\x92\x827\x01RV[\x90\x92\x91\x92a\x03\xB9a\x03\xB4\x82a\x03vV[a\x03aV[\x93\x81\x85R` \x85\x01\x90\x82\x84\x01\x11a\x03\xD5Wa\x03\xD3\x92a\x03\x99V[V[a\x03\x16V[\x90\x80`\x1F\x83\x01\x12\x15a\x03\xF8W\x81` a\x03\xF5\x935\x91\x01a\x03\xA4V[\x90V[a\x03\x12V[a\x04\x06\x81a\x02\xAAV[\x03a\x04\rWV[_\x80\xFD[\x90P5\x90a\x04\x1E\x82a\x03\xFDV[V[a\x04)\x81a\x01\x9EV[\x03a\x040WV[_\x80\xFD[\x90P5\x90a\x04A\x82a\x04 V[V[\x91\x90`\xA0\x83\x82\x03\x12a\x04\xACW_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xA7W\x81a\x04n\x91\x85\x01a\x03\xDAV[\x92a\x04|\x82` \x83\x01a\x04\x11V[\x92a\x04\xA4a\x04\x8D\x84`@\x85\x01a\x044V[\x93a\x04\x9B\x81``\x86\x01a\x04\x11V[\x93`\x80\x01a\x044V[\x90V[a\x03\x0EV[a\x01\x8BV[_\x01\x90V[4a\x04\xE8Wa\x04\xD2a\x04\xC96`\x04a\x04CV[\x93\x92\x90\x92a\x10\xB6V[a\x04\xDAa\x01\x81V[\x80a\x04\xE4\x81a\x04\xB1V[\x03\x90\xF3[a\x01\x87V[4a\x05\x1DWa\x04\xFD6`\x04a\x01\x8FV[a\x05\x19a\x05\x08a\x12\xD5V[a\x05\x10a\x01\x81V[\x91\x82\x91\x82a\x01\xAEV[\x03\x90\xF3[a\x01\x87V[\x90` \x82\x82\x03\x12a\x05;Wa\x058\x91_\x01a\x044V[\x90V[a\x01\x8BV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[T\x90V[_R` _ \x90V[a\x05j\x81a\x05TV[\x82\x10\x15a\x05\x84Wa\x05|`\x01\x91a\x05XV[\x91\x02\x01\x90_\x90V[a\x05@V[\x90V[a\x05\x9C\x90`\x08a\x05\xA1\x93\x02a\x01\xF8V[a\x05\x89V[\x90V[\x90a\x05\xAF\x91Ta\x05\x8CV[\x90V[`\x05a\x05\xBD\x81a\x05TV[\x82\x10\x15a\x05\xDAWa\x05\xD7\x91a\x05\xD1\x91a\x05aV[\x90a\x05\xA4V[\x90V[_\x80\xFD[\x90V[a\x05\xEA\x90a\x05\xDEV[\x90RV[\x91\x90a\x06\x01\x90_` \x85\x01\x94\x01\x90a\x05\xE1V[V[4a\x063Wa\x06/a\x06\x1Ea\x06\x196`\x04a\x05\"V[a\x05\xB2V[a\x06&a\x01\x81V[\x91\x82\x91\x82a\x05\xEEV[\x03\x90\xF3[a\x01\x87V[a\x06D`\x01_\x90a\x02\x17V[\x90V[4a\x06wWa\x06W6`\x04a\x01\x8FV[a\x06sa\x06ba\x068V[a\x06ja\x01\x81V[\x91\x82\x91\x82a\x01\xAEV[\x03\x90\xF3[a\x01\x87V[\x90V[\x90V[a\x06\x96a\x06\x91a\x06\x9B\x92a\x06|V[a\x06\x7FV[a\x02\xAAV[\x90V[a\x06\xA9a\x0E\x10a\x06\x82V[\x90V[a\x06\xB4a\x06\x9EV[\x90V[4a\x06\xE7Wa\x06\xC76`\x04a\x01\x8FV[a\x06\xE3a\x06\xD2a\x06\xACV[a\x06\xDAa\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[\x90V[`\xFF\x16\x90V[a\x07\ta\x07\x04a\x07\x0E\x92a\x06\xECV[a\x06\x7FV[a\x06\xEFV[\x90V[a\x07\x1B`\x0Ca\x06\xF5V[\x90V[a\x07&a\x07\x11V[\x90V[a\x072\x90a\x06\xEFV[\x90RV[\x91\x90a\x07I\x90_` \x85\x01\x94\x01\x90a\x07)V[V[4a\x07{Wa\x07[6`\x04a\x01\x8FV[a\x07wa\x07fa\x07\x1EV[a\x07na\x01\x81V[\x91\x82\x91\x82a\x076V[\x03\x90\xF3[a\x01\x87V[a\x07\x94a\x07\x8Fa\x07\x99\x92a\x06\xECV[a\x06\x7FV[a\x02\xAAV[\x90V[a\x07\xA6`\x0Ca\x07\x80V[\x90V[a\x07\xB1a\x07\x9CV[\x90V[4a\x07\xE4Wa\x07\xC46`\x04a\x01\x8FV[a\x07\xE0a\x07\xCFa\x07\xA9V[a\x07\xD7a\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[a\x07\xF5`\x06_\x90a\x02\x17V[\x90V[4a\x08(Wa\x08\x086`\x04a\x01\x8FV[a\x08$a\x08\x13a\x07\xE9V[a\x08\x1Ba\x01\x81V[\x91\x82\x91\x82a\x01\xAEV[\x03\x90\xF3[a\x01\x87V[a\x086\x81a\x06\xEFV[\x03a\x08=WV[_\x80\xFD[\x90P5\x90a\x08N\x82a\x08-V[V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08d\x90a\x08PV[\x90V[a\x08p\x81a\x08[V[\x03a\x08wWV[_\x80\xFD[\x90P5\x90a\x08\x88\x82a\x08gV[V[\x91``\x83\x83\x03\x12a\x08\xD6Wa\x08\xA1\x82_\x85\x01a\x08AV[\x92a\x08\xAF\x83` \x83\x01a\x08{V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08\xD1Wa\x08\xCE\x92\x01a\x03\xDAV[\x90V[a\x03\x0EV[a\x01\x8BV[4a\t\nWa\x08\xF4a\x08\xEE6`\x04a\x08\x8AV[\x91a\x14\xCCV[a\x08\xFCa\x01\x81V[\x80a\t\x06\x81a\x04\xB1V[\x03\x90\xF3[a\x01\x87V[\x90V[a\t&a\t!a\t+\x92a\t\x0FV[a\x06\x7FV[a\x02\xAAV[\x90V[a\t9a\x1C a\t\x12V[\x90V[a\tDa\t.V[\x90V[4a\twWa\tW6`\x04a\x01\x8FV[a\tsa\tba\t<V[a\tja\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[\x90V[a\t\x93a\t\x8Ea\t\x98\x92a\t|V[a\x06\x7FV[a\x06\xEFV[\x90V[a\t\xA5`\x0Ba\t\x7FV[\x90V[a\t\xB0a\t\x9BV[\x90V[4a\t\xE3Wa\t\xC36`\x04a\x01\x8FV[a\t\xDFa\t\xCEa\t\xA8V[a\t\xD6a\x01\x81V[\x91\x82\x91\x82a\x076V[\x03\x90\xF3[a\x01\x87V[\x90V[a\t\xFFa\t\xFAa\n\x04\x92a\t\xE8V[a\x06\x7FV[a\x02\xAAV[\x90V[a\n\x13b\x01Q\x80a\t\xEBV[\x90V[a\n\x1Ea\n\x07V[\x90V[4a\nQWa\n16`\x04a\x01\x8FV[a\nMa\n<a\n\x16V[a\nDa\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[`\x04a\na\x81a\x05TV[\x82\x10\x15a\n~Wa\n{\x91a\nu\x91a\x05aV[\x90a\x05\xA4V[\x90V[_\x80\xFD[4a\n\xB2Wa\n\xAEa\n\x9Da\n\x986`\x04a\x05\"V[a\nVV[a\n\xA5a\x01\x81V[\x91\x82\x91\x82a\x05\xEEV[\x03\x90\xF3[a\x01\x87V[a\n\xECa\n\xF3\x94a\n\xE2``\x94\x98\x97\x95a\n\xD8`\x80\x86\x01\x9A_\x87\x01\x90a\x02\xB7V[` \x85\x01\x90a\x01\xA1V[`@\x83\x01\x90a\x02\xB7V[\x01\x90a\x01\xA1V[V[4a\x0B)Wa\x0B\x056`\x04a\x01\x8FV[a\x0B%a\x0B\x10a\x16\xC4V[\x90a\x0B\x1C\x94\x92\x94a\x01\x81V[\x94\x85\x94\x85a\n\xB7V[\x03\x90\xF3[a\x01\x87V[4a\x0B^Wa\x0BZa\x0BIa\x0BD6`\x04a\x05\"V[a\x17\x1DV[a\x0BQa\x01\x81V[\x91\x82\x91\x82a\x05\xEEV[\x03\x90\xF3[a\x01\x87V[\x90\x91``\x82\x84\x03\x12a\x0B\x98Wa\x0B\x95a\x0B~\x84_\x85\x01a\x08{V[\x93a\x0B\x8C\x81` \x86\x01a\x08{V[\x93`@\x01a\x044V[\x90V[a\x01\x8BV[4a\x0B\xCCWa\x0B\xB6a\x0B\xB06`\x04a\x0BcV[\x91a\x17\\V[a\x0B\xBEa\x01\x81V[\x80a\x0B\xC8\x81a\x04\xB1V[\x03\x90\xF3[a\x01\x87V[\x90V[a\x0B\xE8a\x0B\xE3a\x0B\xED\x92a\x0B\xD1V[a\x06\x7FV[a\x02\xAAV[\x90V[a\x0B\xFCb\x01\xCC\xCCa\x0B\xD4V[\x90V[a\x0C\x07a\x0B\xF0V[\x90V[4a\x0C:Wa\x0C\x1A6`\x04a\x01\x8FV[a\x0C6a\x0C%a\x0B\xFFV[a\x0C-a\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[4a\x0CoWa\x0CO6`\x04a\x01\x8FV[a\x0Cka\x0CZa\x17\x9CV[a\x0Cba\x01\x81V[\x91\x82\x91\x82a\x01\xAEV[\x03\x90\xF3[a\x01\x87V[a\x0C\x80`\x02_\x90a\x02\x8EV[\x90V[4a\x0C\xB3Wa\x0C\x936`\x04a\x01\x8FV[a\x0C\xAFa\x0C\x9Ea\x0CtV[a\x0C\xA6a\x01\x81V[\x91\x82\x91\x82a\x02\xC4V[\x03\x90\xF3[a\x01\x87V[_\x80\xFD[_\x90V[a\x0C\xC8a\x0C\xBCV[Pa\x0C\xD3`\x05a\x05TV[\x90V[_\x1B\x90V[\x90a\x0C\xEEg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x0C\xD6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\r\x0Ca\r\x07a\r\x11\x92a\x02\xAAV[a\x06\x7FV[a\x02\xAAV[\x90V[\x90V[\x90a\r,a\r'a\r3\x92a\x0C\xF8V[a\r\x14V[\x82Ta\x0C\xDBV[\x90UV[\x90a\rC_\x19\x91a\x0C\xD6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\raa\r\\a\rf\x92a\x01\x9EV[a\x06\x7FV[a\x01\x9EV[\x90V[\x90V[\x90a\r\x81a\r|a\r\x88\x92a\rMV[a\riV[\x82Ta\r7V[\x90UV[\x90V[a\r\xA3a\r\x9Ea\r\xA8\x92a\r\x8CV[a\x06\x7FV[a\x02\xAAV[\x90V[a\r\xBFa\r\xBAa\r\xC4\x92a\r\x8CV[a\x0C\xD6V[a\x05\xDEV[\x90V[a\r\xDBa\r\xD6a\r\xE0\x92a\r\x8CV[a\x06\x7FV[a\x01\x9EV[\x90V[\x90V[a\r\xFAa\r\xF5a\r\xFF\x92a\r\xE3V[a\x06\x7FV[a\x01\x9EV[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x0E%a\x0E+\x91\x93\x92\x93a\x01\x9EV[\x92a\x01\x9EV[\x82\x03\x91\x82\x11a\x0E6WV[a\x0E\x02V[\x90V[a\x0EJa\x0EO\x91a\x05\xDEV[a\x0E;V[\x90RV[` \x93\x92a\x0Er\x85\x83a\x0Ej\x82\x95a\x0Ez\x97a\x0E>V[\x01\x80\x92a\x0E>V[\x01\x80\x92a\x0E>V[\x01\x90V[` \x01\x90V[Q\x90V[\x90V[_R` _ \x90V[T\x90V[a\x0E\xA1\x81a\x0E\x94V[\x82\x10\x15a\x0E\xBBWa\x0E\xB3`\x01\x91a\x0E\x8BV[\x91\x02\x01\x90_\x90V[a\x05@V[\x1B\x90V[\x91\x90`\x08a\x0E\xDF\x91\x02\x91a\x0E\xD9_\x19\x84a\x0E\xC0V[\x92a\x0E\xC0V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x0E\xF2\x90a\x05\xDEV[\x90V[_\x1C\x90V[a\x0F\x03\x90a\x0E\xF5V[\x90V[\x91\x90a\x0F\x1Ca\x0F\x17a\x0F$\x93a\x0E\xE9V[a\x0E\xFAV[\x90\x83Ta\x0E\xC4V[\x90UV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x0FXW\x82a\x0FP\x91`\x01a\x0FV\x95\x01\x81Ua\x0E\x98V[\x90a\x0F\x06V[V[a\x03$V[a\x0Fia\x0Fn\x91a\x0E\xF5V[a\x01\xFCV[\x90V[a\x0F{\x90Ta\x0F]V[\x90V[a\x0F\x87\x90a\x02\xAAV[\x90RV[\x90``\x80a\x0F\xD1\x93a\x0F\xA3_\x82\x01Q_\x86\x01\x90a\x0F~V[a\x0F\xB5` \x82\x01Q` \x86\x01\x90a\x0F~V[a\x0F\xC7`@\x82\x01Q`@\x86\x01\x90a\x0F~V[\x01Q\x91\x01\x90a\x0F~V[V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x05\x11\x15a\x0F\xF1WV[a\x0F\xD3V[\x90a\x10\0\x82a\x0F\xE7V[V[a\x10\x0B\x90a\x0F\xF6V[\x90V[a\x10\x17\x90a\x10\x02V[\x90RV[a\x10Pa\x10W\x94a\x10F`\xC0\x94\x98\x97\x95a\x10<`\xE0\x86\x01\x9A_\x87\x01\x90a\x05\xE1V[` \x85\x01\x90a\x01\xA1V[`@\x83\x01\x90a\x0F\x8BV[\x01\x90a\x10\x0EV[V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[a\x10\x8Ca\x10\x95` \x93a\x10\x9A\x93a\x10\x83\x81a\x0E\x84V[\x93\x84\x80\x93a\x10YV[\x95\x86\x91\x01a\x10bV[a\x03\x1AV[\x01\x90V[a\x10\xB3\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x10mV[\x90V[\x93\x91a\x10\xC5a\x10\xCC\x92_a\r\x17V[`\x01a\rlV[\x81a\x10\xDFa\x10\xD9_a\r\x8FV[\x91a\x02\xAAV[\x11a\x12\xB9W[PPa\x10\xF1`\x04a\x05TV[a\x10\xFC\x82\x82\x90a\x19\xF1V[\x90a\x11\x07`\x05a\x05TV[\x92a\x11\x11_a\r\xABV[\x84a\x11$a\x11\x1E_a\r\xC7V[\x91a\x01\x9EV[\x11a\x12\x8BW[a\x113_a\r\xABV[\x92\x82a\x11Ga\x11A_a\r\xC7V[\x91a\x01\x9EV[\x11a\x12VW[a\x11\xAB\x90a\x11|\x83a\x11m\x87a\x11aa\x01\x81V[\x94\x85\x93` \x85\x01a\x0ESV[` \x82\x01\x81\x03\x82R\x03\x82a\x038V[a\x11\x8Ea\x11\x88\x82a\x0E\x84V[\x91a\x0E~V[ \x92a\x11\xA4a\x11\x9D`\x05a\x0E\x88V[\x85\x90a\x0F(V[`\x06a\rlV[\x84\x90\x91\x92\x93a\x11\xBA`\x06a\x0FqV[\x90\x94a\x12\x0F`\x01a\x11\xFDa\x11\xF7a\x11\xF1\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x97a\rMV[\x97a\x0E\xE9V[\x97a\x0E\xE9V[\x97a\x12\x06a\x01\x81V[\x94\x85\x94\x85a\x10\x1BV[\x03\x90\xA4a\x12Qa\x12?\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x92a\rMV[\x92a\x12Ha\x01\x81V[\x91\x82\x91\x82a\x10\x9EV[\x03\x90\xA2V[\x92Pa\x11\xABa\x12\x83a\x12}`\x04a\x12w\x86a\x12q`\x01a\r\xE6V[\x90a\x0E\x16V[\x90a\x05aV[\x90a\x05\xA4V[\x93\x90Pa\x11MV[Pa\x12\xB4a\x12\xAE`\x05a\x12\xA8\x87a\x12\xA2`\x01a\r\xE6V[\x90a\x0E\x16V[\x90a\x05aV[\x90a\x05\xA4V[a\x11*V[a\x12\xC7a\x12\xCE\x92`\x02a\r\x17V[`\x03a\rlV[_\x80a\x10\xE5V[a\x12\xDDa\x0C\xBCV[Pa\x12\xE8`\x05a\x05TV[\x90V[a\x12\xFFa\x12\xFAa\x13\x04\x92a\x01\x9EV[a\x06\x7FV[a\x02\xAAV[\x90V[`\xF8\x1B\x90V[a\x13\x16\x90a\x13\x07V[\x90V[a\x13%a\x13*\x91a\x06\xEFV[a\x13\rV[\x90RV[``\x1B\x90V[a\x13=\x90a\x13.V[\x90V[a\x13I\x90a\x134V[\x90V[a\x13Xa\x13]\x91a\x08[V[a\x13@V[\x90RV[`\xC0\x1B\x90V[a\x13p\x90a\x13aV[\x90V[a\x13\x7Fa\x13\x84\x91a\x02\xAAV[a\x13gV[\x90RV[\x90V[a\x13\x97a\x13\x9C\x91a\x01\x9EV[a\x13\x88V[\x90RV[\x94a\x13\xF1`\x08` \x99\x98\x95\x96a\x13\xE9\x82\x8C\x99a\x13\xE1`\x14a\x13\xF9\x9Aa\x13\xD9a\x14\x01\x9F\x8F\x9C\x90a\x13\xD1\x81`\x01\x93a\x13\x19V[\x01\x80\x92a\x13LV[\x01\x80\x92a\x13sV[\x01\x80\x92a\x13sV[\x01\x80\x92a\x13\x8BV[\x01\x80\x92a\x13\x8BV[\x01\x80\x92a\x0E>V[\x01\x90V[` \x81a\x14\x17a\x14\x1F\x93\x83\x96\x95a\x0E>V[\x01\x80\x92a\x0E>V[\x01\x90V[a\x147a\x142a\x14<\x92a\x08PV[a\x06\x7FV[a\x08PV[\x90V[a\x14H\x90a\x14#V[\x90V[a\x14T\x90a\x14?V[\x90V[a\x14`\x90a\x08[V[\x90RV[a\x14m\x90a\r\xC7V[\x90RV[\x91\x94a\x14\xB9a\x14\xC3\x92\x98\x97\x95a\x14\xAF`\xA0\x96a\x14\xA5a\x14\xCA\x9Aa\x14\x9B`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x14WV[` \x89\x01\x90a\x07)V[`@\x87\x01\x90a\x14WV[``\x85\x01\x90a\x05\xE1V[`\x80\x83\x01\x90a\x14dV[\x01\x90a\x02\xB7V[V[\x90a\x14\xD7`\x04a\x05TV[\x91\x83a\x14\xEBa\x14\xE5\x82a\x0E\x84V[\x91a\x0E~V[ \x91\x81a\x158\x82\x91a\x15)a\x14\xFFCa\x12\xEBV[a\x15\x08Ba\x12\xEBV[\x89a\x15\x12_a\r\xC7V[\x91\x8A\x93a\x15\x1Da\x01\x81V[\x98\x89\x97` \x89\x01a\x13\xA0V[` \x82\x01\x81\x03\x82R\x03\x82a\x038V[a\x15Ja\x15D\x82a\x0E\x84V[\x91a\x0E~V[ \x90a\x15U_a\r\xABV[\x91\x85a\x15ia\x15c_a\r\xC7V[\x91a\x01\x9EV[\x11a\x16jW[a\x15\xC1\x90a\x15}`\x04a\x0E\x88V[\x90a\x15\xA8\x85a\x15\x99a\x15\x8Da\x01\x81V[\x93\x84\x92` \x84\x01a\x14\x05V[` \x82\x01\x81\x03\x82R\x03\x82a\x038V[a\x15\xBAa\x15\xB4\x82a\x0E\x84V[\x91a\x0E~V[ \x90a\x0F(V[\x84\x91\x92a\x16#a\x15\xD00a\x14KV[\x91\x92\x95_a\x15\xDDBa\x12\xEBV[\x91a\x16\x11a\x16\x0B\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE1\x98a\rMV[\x98a\x0E\xE9V[\x98a\x16\x1Aa\x01\x81V[\x96\x87\x96\x87a\x14qV[\x03\x90\xA3a\x16ea\x16S\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x92a\rMV[\x92a\x16\\a\x01\x81V[\x91\x82\x91\x82a\x10\x9EV[\x03\x90\xA2V[\x91Pa\x15\xC1a\x16\x97a\x16\x91`\x04a\x16\x8B\x89a\x16\x85`\x01a\r\xE6V[\x90a\x0E\x16V[\x90a\x05aV[\x90a\x05\xA4V[\x92\x90Pa\x15oV[_\x90V[a\x16\xAFa\x16\xB4\x91a\x0E\xF5V[a\x02iV[\x90V[a\x16\xC1\x90Ta\x16\xA3V[\x90V[a\x16\xCCa\x16\x9FV[Pa\x16\xD5a\x0C\xBCV[Pa\x16\xDEa\x16\x9FV[Pa\x16\xE7a\x0C\xBCV[Pa\x16\xF1_a\x16\xB7V[\x90a\x16\xFC`\x01a\x0FqV[\x91a\x17\x07`\x02a\x16\xB7V[\x91a\x17\x12`\x03a\x0FqV[\x91\x93\x92\x91\x90V[_\x90V[a\x174a\x17:\x91a\x17,a\x17\x19V[P`\x05a\x05aV[\x90a\x05\xA4V[\x90V[`\x14\x81a\x17Pa\x17X\x93` \x96\x95a\x13LV[\x01\x80\x92a\x13\x8BV[\x01\x90V[\x90a\x17\x9A\x92\x91a\x17\x95a\x17ma\x07\x11V[\x91\x92a\x17\x86a\x17za\x01\x81V[\x95\x86\x92` \x84\x01a\x17=V[` \x82\x01\x81\x03\x82R\x03\x84a\x038V[a\x14\xCCV[V[a\x17\xA4a\x0C\xBCV[Pa\x17\xAF`\x04a\x05TV[\x90V[a\x17\xBC`\x80a\x03aV[\x90V[_\x90V[a\x17\xCBa\x17\xB2V[\x90` \x80\x80\x80\x85a\x17\xDAa\x17\xBFV[\x81R\x01a\x17\xE5a\x17\xBFV[\x81R\x01a\x17\xF0a\x17\xBFV[\x81R\x01a\x17\xFBa\x17\xBFV[\x81RPPV[a\x18\ta\x17\xC3V[\x90V[\x90V[a\x18#a\x18\x1Ea\x18(\x92a\x18\x0CV[a\x06\x7FV[a\x01\x9EV[\x90V[a\x185`(a\x18\x0FV[\x90V[a\x18Ga\x18M\x91\x93\x92\x93a\x01\x9EV[\x92a\x01\x9EV[\x82\x01\x80\x92\x11a\x18XWV[a\x0E\x02V[a\x18qa\x18la\x18v\x92a\x02\xAAV[a\x06\x7FV[a\x01\x9EV[\x90V[a\x18\x82\x90a\x18]V[\x90RV[\x91` a\x18\xA7\x92\x94\x93a\x18\xA0`@\x82\x01\x96_\x83\x01\x90a\x01\xA1V[\x01\x90a\x18yV[V[a\x18\xB5a\x18\xBB\x91a\x02\xAAV[\x91a\x02\xAAV[\x90\x03\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x18\xCFWV[a\x0E\x02V[\x90a\x18\xDE\x90a\x02\xAAV[\x90RV[a\x18\xEEa\x18\xF4\x91a\x02\xAAV[\x91a\x02\xAAV[\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x19\x07WV[a\x0E\x02V[a\x19\x16\x90Qa\x02\xAAV[\x90V[`\x08a\x19[\x94a\x19K\x82\x80\x99\x98\x95\x96a\x19C\x82\x87a\x19;a\x19S\x99\x83\x9Ca\x13sV[\x01\x80\x92a\x13sV[\x01\x80\x92a\x13sV[\x01\x80\x92a\x13sV[\x01\x80\x92a\x13sV[\x01\x90V[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[\x15a\x19zWV[a\x19_V[\x90P\x90V[a\x19\xA9a\x19\xA0\x92` \x92a\x19\x97\x81a\x0E\x84V[\x94\x85\x80\x93a\x19\x7FV[\x93\x84\x91\x01a\x10bV[\x01\x90V[a\x19\xBB\x90a\x19\xC1\x93\x92a\x19\x84V[\x90a\x19\x84V[\x90V[a\x19\xE3\x92\x91a\x19\xEF\x91a\x19\xD5a\x01\x81V[\x94\x85\x92` \x84\x01\x92\x83a\x19\xADV[\x90\x81\x03\x82R\x03\x83a\x038V[V[\x91\x90\x91a\x19\xFCa\x17\x19V[Pa\x1A\x05a\x18\x01V[Pa\x1A a\x1A\x11a\x18+V[a\x1A\x1A\x83a\x0E\x84V[\x90a\x188V[\x80a\x1A:a\x1A4a\x1A/a\x0B\xF0V[a\x18]V[\x91a\x01\x9EV[\x11a\x1B\xD9WPa\x1Bm\x90a\x1ALa\x18\x01V[\x93Ba\x1Aga\x1Aaa\x1A\\a\n\x07V[a\x18]V[\x91a\x01\x9EV[\x11a\x1B\xAFW[a\x1A\x93a\x1A\x8Aa\x1A|Ba\x12\xEBV[a\x1A\x84a\x06\x9EV[\x90a\x18\xE2V[` \x87\x01a\x18\xD4V[Ca\x1A\xADa\x1A\xA7a\x1A\xA2a\t.V[a\x18]V[\x91a\x01\x9EV[\x11a\x1B\x84W[a\x1A\xD9a\x1A\xD0a\x1A\xC2Ca\x12\xEBV[a\x1A\xCAa\x07\x9CV[\x90a\x18\xE2V[``\x87\x01a\x18\xD4V[a\x1B=a\x1A\xE7_\x87\x01a\x19\x0CV[a\x1B.a\x1A\xF6` \x89\x01a\x19\x0CV[\x93a\x1B\x03`@\x8A\x01a\x19\x0CV[\x90a\x1B\x19a\x1B\x13``\x8C\x01a\x19\x0CV[\x91a\x12\xEBV[\x91a\x1B\"a\x01\x81V[\x96\x87\x95` \x87\x01a\x19\x19V[` \x82\x01\x81\x03\x82R\x03\x82a\x038V[a\x1Bha\x1BI\x82a\x0E\x84V[a\x1Bba\x1B\\a\x1BWa\x18+V[a\x01\x9EV[\x91a\x01\x9EV[\x14a\x19sV[a\x19\xC4V[a\x1B\x7Fa\x1By\x82a\x0E\x84V[\x91a\x0E~V[ \x91\x90V[a\x1B\xAAa\x1B\xA1a\x1B\x93Ca\x12\xEBV[a\x1B\x9Ba\t.V[\x90a\x18\xA9V[`@\x87\x01a\x18\xD4V[a\x1A\xB3V[a\x1B\xD4a\x1B\xCCa\x1B\xBEBa\x12\xEBV[a\x1B\xC6a\n\x07V[\x90a\x18\xA9V[_\x87\x01a\x18\xD4V[a\x1AmV[a\x1B\xE1a\x0B\xF0V[\x90a\x1B\xFC_\x92\x83\x92cF4i\x1B`\xE0\x1B\x84R`\x04\x84\x01a\x18\x86V[\x03\x90\xFD",
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
    /**Function with signature `INITIALIZATION_MSG_TYPE()` and selector `0xae1a7d30`.
```solidity
function INITIALIZATION_MSG_TYPE() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct INITIALIZATION_MSG_TYPECall {}
    ///Container type for the return parameters of the [`INITIALIZATION_MSG_TYPE()`](INITIALIZATION_MSG_TYPECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct INITIALIZATION_MSG_TYPEReturn {
        #[allow(missing_docs)]
        pub _0: u8,
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
            impl ::core::convert::From<INITIALIZATION_MSG_TYPECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: INITIALIZATION_MSG_TYPECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for INITIALIZATION_MSG_TYPECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<INITIALIZATION_MSG_TYPEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: INITIALIZATION_MSG_TYPEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for INITIALIZATION_MSG_TYPEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for INITIALIZATION_MSG_TYPECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = INITIALIZATION_MSG_TYPEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "INITIALIZATION_MSG_TYPE()";
            const SELECTOR: [u8; 4] = [174u8, 26u8, 125u8, 48u8];
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
    /**Function with signature `L1MessageType_ethDeposit()` and selector `0x3c53a383`.
```solidity
function L1MessageType_ethDeposit() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L1MessageType_ethDepositCall {}
    ///Container type for the return parameters of the [`L1MessageType_ethDeposit()`](L1MessageType_ethDepositCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L1MessageType_ethDepositReturn {
        #[allow(missing_docs)]
        pub _0: u8,
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
            impl ::core::convert::From<L1MessageType_ethDepositCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: L1MessageType_ethDepositCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for L1MessageType_ethDepositCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<L1MessageType_ethDepositReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: L1MessageType_ethDepositReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for L1MessageType_ethDepositReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for L1MessageType_ethDepositCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = L1MessageType_ethDepositReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "L1MessageType_ethDeposit()";
            const SELECTOR: [u8; 4] = [60u8, 83u8, 163u8, 131u8];
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
        INITIALIZATION_MSG_TYPE(INITIALIZATION_MSG_TYPECall),
        #[allow(missing_docs)]
        L1MessageType_ethDeposit(L1MessageType_ethDepositCall),
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
            [60u8, 83u8, 163u8, 131u8],
            [79u8, 53u8, 154u8, 55u8],
            [127u8, 163u8, 164u8, 14u8],
            [167u8, 181u8, 29u8, 25u8],
            [173u8, 156u8, 12u8, 46u8],
            [174u8, 26u8, 125u8, 48u8],
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
        const COUNT: usize = 22usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::INITIALIZATION_MSG_TYPE(_) => {
                    <INITIALIZATION_MSG_TYPECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::L1MessageType_ethDeposit(_) => {
                    <L1MessageType_ethDepositCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                    fn L1MessageType_ethDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <L1MessageType_ethDepositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::L1MessageType_ethDeposit)
                    }
                    L1MessageType_ethDeposit
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
                    fn INITIALIZATION_MSG_TYPE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <INITIALIZATION_MSG_TYPECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::INITIALIZATION_MSG_TYPE)
                    }
                    INITIALIZATION_MSG_TYPE
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
                Self::INITIALIZATION_MSG_TYPE(inner) => {
                    <INITIALIZATION_MSG_TYPECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::L1MessageType_ethDeposit(inner) => {
                    <L1MessageType_ethDepositCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
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
                Self::INITIALIZATION_MSG_TYPE(inner) => {
                    <INITIALIZATION_MSG_TYPECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::L1MessageType_ethDeposit(inner) => {
                    <L1MessageType_ethDepositCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
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
        ///Creates a new call builder for the [`INITIALIZATION_MSG_TYPE`] function.
        pub fn INITIALIZATION_MSG_TYPE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, INITIALIZATION_MSG_TYPECall, N> {
            self.call_builder(&INITIALIZATION_MSG_TYPECall {})
        }
        ///Creates a new call builder for the [`L1MessageType_ethDeposit`] function.
        pub fn L1MessageType_ethDeposit(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, L1MessageType_ethDepositCall, N> {
            self.call_builder(&L1MessageType_ethDepositCall {})
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
