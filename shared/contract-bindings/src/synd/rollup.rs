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

    constructor(uint256 chainId, string chainConfig, address owner_);

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
    function owner() external view returns (address);
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
      },
      {
        "name": "owner_",
        "type": "address",
        "internalType": "address"
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
    ///0x6080604052346100305761001a6100146101df565b91610535565b610022610035565b611c9a6114cc8239611c9a90f35b61003b565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906100679061003f565b810190811060018060401b0382111761007f57604052565b610049565b90610097610090610035565b928361005d565b565b5f80fd5b5f80fd5b90565b6100ad816100a1565b036100b457565b5f80fd5b905051906100c5826100a4565b565b5f80fd5b5f80fd5b60018060401b0381116100eb576100e760209161003f565b0190565b610049565b90825f9392825e0152565b9092919261011061010b826100cf565b610084565b9381855260208501908284011161012c5761012a926100f0565b565b6100cb565b9080601f8301121561014f5781602061014c935191016100fb565b90565b6100c7565b60018060a01b031690565b61016890610154565b90565b6101748161015f565b0361017b57565b5f80fd5b9050519061018c8261016b565b565b90916060828403126101da576101a6835f84016100b8565b9260208301519060018060401b0382116101d5576101c9816101d2938601610131565b9360400161017f565b90565b61009d565b610099565b6101fd613166803803806101f281610084565b92833981019061018e565b909192565b5f1b90565b9061021860018060401b0391610202565b9181191691161790565b90565b60018060401b031690565b90565b61024761024261024c92610222565b610230565b610225565b90565b90565b9061026761026261026e92610233565b61024f565b8254610207565b9055565b9061027e5f1991610202565b9181191691161790565b61029c6102976102a192610222565b610230565b6100a1565b90565b90565b906102bc6102b76102c392610288565b6102a4565b8254610272565b9055565b906102d860018060a01b0391610202565b9181191691161790565b6102f66102f16102fb92610154565b610230565b610154565b90565b610307906102e2565b90565b610313906102fe565b90565b90565b9061032e6103296103359261030a565b610316565b82546102c7565b9055565b90565b5190565b60209181520190565b5f7f454d5054595f434841494e5f434f4e4649470000000000000000000000000000910152565b61037d6012602092610340565b61038681610349565b0190565b61039f9060208101905f818303910152610370565b90565b156103a957565b6103b1610035565b62461bcd60e51b8152806103c76004820161038a565b0390fd5b90565b60ff1690565b6103e86103e36103ed926103cb565b610230565b6103ce565b90565b6103fa600b6103d4565b90565b61041161040c61041692610222565b610230565b610154565b90565b610422906103fd565b90565b90565b61043c61043761044192610425565b610230565b6103ce565b90565b90565b610453610458916100a1565b610444565b9052565b60f81b90565b61046b9061045c565b90565b61047a61047f916103ce565b610462565b9052565b5190565b905090565b6104b16104a89260209261049f81610483565b94858093610487565b938491016100f0565b0190565b60016020936104d985846104d16104e1966104e89b9a98610447565b01809261046e565b018092610447565b019061048c565b90565b906104fd6104f8836100cf565b610084565b918252565b5f640b0080020360d01b910152565b61051b60066104eb565b9061052860208301610502565b565b610532610511565b90565b61056e6105ee936105465f5f610252565b6105515f60016102a7565b61055c5f6002610252565b6105675f60036102a7565b6004610319565b61059a61058261057d84610339565b61033c565b61059461058e5f610288565b916100a1565b116103a2565b6105a26103f0565b6105e96105ae5f610419565b926105da6105bc6001610428565b956105c65f610288565b6105ce610035565b978894602086016104b5565b6020820181038252038461005d565b610a16565b6106285f5f5f5f9161062261061c61061661061061060a61052a565b97610233565b93610288565b93610233565b93610288565b93610d7c565b565b5490565b60200190565b61064861064361064d926100a1565b610230565b610225565b90565b60601b90565b61065f90610650565b90565b61066b90610656565b90565b61067a61067f9161015f565b610662565b9052565b60c01b90565b61069290610683565b90565b6106a16106a691610225565b610689565b9052565b90565b90565b6106bc6106c1916106aa565b6106ad565b9052565b94610716600860209998959661070e828c99610706601461071e9a6106fe6107269f8f9c906106f68160019361046e565b01809261066e565b018092610695565b018092610695565b018092610447565b018092610447565b0180926106b0565b0190565b61073e61073961074392610222565b610202565b6106aa565b90565b61075a61075561075f92610425565b610230565b6100a1565b90565b634e487b7160e01b5f52601160045260245ffd5b61078561078b919392936100a1565b926100a1565b820391821161079657565b610762565b634e487b7160e01b5f52603260045260245ffd5b5f5260205f2090565b6107c18161062a565b8210156107db576107d36001916107af565b910201905f90565b61079b565b1c90565b90565b6107f79060086107fc93026107e0565b6107e4565b90565b9061080a91546107e7565b90565b90565b60208161082261082a938396956106b0565b0180926106b0565b0190565b5f5260205f2090565b5490565b61084481610837565b82101561085e5761085660019161082e565b910201905f90565b61079b565b1b90565b9190600861088291029161087c5f1984610863565b92610863565b9181191691161790565b610895906106aa565b90565b5f1c90565b6108a690610898565b90565b91906108bf6108ba6108c79361088c565b61089d565b908354610867565b9055565b90815491680100000000000000008310156108fb57826108f39160016108f99501815561083b565b906108a9565b565b610049565b610909906102fe565b90565b61092061091b610925926100a1565b610230565b6100a1565b90565b6109319061015f565b9052565b61093e906103ce565b9052565b61094b906106aa565b9052565b61095890610288565b9052565b61096590610225565b9052565b91946109b16109bb929897956109a760a09661099d6109c29a61099360c08a019e5f8b0190610928565b6020890190610935565b6040870190610928565b6060850190610942565b608083019061094f565b019061095c565b565b60209181520190565b6109ec6109f56020936109fa936109e38161033c565b938480936109c4565b958691016100f0565b61003f565b0190565b610a139160208201915f8184039101526109cd565b90565b90610a21600561062a565b9183610a35610a2f8261033c565b9161062e565b209181610a828291610a73610a4943610634565b610a5242610634565b89610a5c5f610288565b918a93610a67610035565b988997602089016106c5565b6020820181038252038261005d565b610a94610a8e8261033c565b9161062e565b2090610a9f5f61072a565b9185610ab3610aad5f610288565b916100a1565b11610bb4575b610b0b90610ac7600561080d565b90610af285610ae3610ad7610035565b93849260208401610810565b6020820181038252038261005d565b610b04610afe8261033c565b9161062e565b20906108cb565b849192610b6d610b1a30610900565b9192955f610b2742610634565b91610b5b610b557f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe19861090c565b9861088c565b98610b64610035565b96879687610969565b0390a3610baf610b9d7fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b9261090c565b92610ba6610035565b918291826109fe565b0390a2565b9150610b0b610be1610bdb6005610bd589610bcf6001610746565b90610776565b906107b8565b906107ff565b929050610ab9565b610bfd610bf8610c0292610225565b610230565b610225565b90565b90610c1a610c15610c2192610be9565b61024f565b8254610207565b9055565b90610c3a610c35610c419261090c565b6102a4565b8254610272565b9055565b60209392610c648583610c5c8295610c6c976106b0565b0180926106b0565b0180926106b0565b0190565b90565b610c7f610c8491610898565b610c70565b90565b610c919054610c73565b90565b610c9d906100a1565b9052565b610caa90610225565b9052565b90606080610cf493610cc65f8201515f860190610ca1565b610cd860208201516020860190610ca1565b610cea60408201516040860190610ca1565b0151910190610ca1565b565b634e487b7160e01b5f52602160045260245ffd5b60051115610d1457565b610cf6565b90610d2382610d0a565b565b610d2e90610d19565b90565b610d3a90610d25565b9052565b610d73610d7a94610d6960c094989795610d5f60e086019a5f870190610942565b6020850190610c94565b6040830190610cae565b0190610d31565b565b9391610d8b610d92925f610c05565b6001610c25565b81610da5610d9f5f610233565b91610225565b11610f7f575b5050610db7600561062a565b610dc28282906112bc565b90610dcd600661062a565b92610dd75f61072a565b84610dea610de45f610288565b916100a1565b11610f51575b610df95f61072a565b9282610e0d610e075f610288565b916100a1565b11610f1c575b610e7190610e4283610e3387610e27610035565b94859360208501610c45565b6020820181038252038261005d565b610e54610e4e8261033c565b9161062e565b2092610e6a610e63600661080d565b85906108cb565b6007610c25565b8490919293610e806007610c87565b9094610ed56001610ec3610ebd610eb77f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd79761090c565b9761088c565b9761088c565b97610ecc610035565b94859485610d3e565b0390a4610f17610f057ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c209261090c565b92610f0e610035565b918291826109fe565b0390a2565b9250610e71610f49610f436005610f3d86610f376001610746565b90610776565b906107b8565b906107ff565b939050610e13565b50610f7a610f746006610f6e87610f686001610746565b90610776565b906107b8565b906107ff565b610df0565b610f8d610f94926002610c05565b6003610c25565b5f80610dab565b5f90565b610fa96080610084565b90565b5f90565b610fb8610f9f565b90602080808085610fc7610fac565b815201610fd2610fac565b815201610fdd610fac565b815201610fe8610fac565b81525050565b610ff6610fb0565b90565b90565b61101061100b61101592610ff9565b610230565b6100a1565b90565b6110226028610ffc565b90565b61103461103a919392936100a1565b926100a1565b820180921161104557565b610762565b90565b61106161105c6110669261104a565b610230565b610225565b90565b6110756201cccc61104d565b90565b61108c61108761109192610225565b610230565b6100a1565b90565b61109d90611078565b9052565b9160206110c29294936110bb60408201965f830190610c94565b0190611094565b565b90565b6110db6110d66110e0926110c4565b610230565b610225565b90565b6110ef620151806110c7565b90565b6110fe61110491610225565b91610225565b90039060018060401b03821161111657565b610762565b9061112590610225565b9052565b90565b61114061113b61114592611129565b610230565b610225565b90565b611153610e1061112c565b90565b61116261116891610225565b91610225565b019060018060401b03821161117957565b610762565b90565b61119561119061119a9261117e565b610230565b610225565b90565b6111a8611c20611181565b90565b90565b6111c26111bd6111c7926111ab565b610230565b610225565b90565b6111d4600c6111ae565b90565b6111e19051610225565b90565b60086112269461121682809998959661120e828761120661121e99839c610695565b018092610695565b018092610695565b018092610695565b018092610695565b0190565b634e487b7160e01b5f52600160045260245ffd5b1561124557565b61122a565b905090565b61127461126b926020926112628161033c565b9485809361124a565b938491016100f0565b0190565b6112869061128c939261124f565b9061124f565b90565b6112ae92916112ba916112a0610035565b948592602084019283611278565b9081038252038361005d565b565b9190916112c7610f9b565b506112d0610fee565b506112eb6112dc611018565b6112e58361033c565b90611025565b806113056112ff6112fa611069565b611078565b916100a1565b116114a4575061143890611317610fee565b934261133261132c6113276110e3565b611078565b916100a1565b1161147a575b61135e61135561134742610634565b61134f611148565b90611156565b6020870161111b565b4361137861137261136d61119d565b611078565b916100a1565b1161144f575b6113a461139b61138d43610634565b6113956111ca565b90611156565b6060870161111b565b6114086113b25f87016111d7565b6113f96113c1602089016111d7565b936113ce60408a016111d7565b906113e46113de60608c016111d7565b91610634565b916113ed610035565b968795602087016111e4565b6020820181038252038261005d565b6114336114148261033c565b61142d611427611422611018565b6100a1565b916100a1565b1461123e565b61128f565b61144a6114448261033c565b9161062e565b209190565b61147561146c61145e43610634565b61146661119d565b906110f2565b6040870161111b565b61137e565b61149f61149761148942610634565b6114916110e3565b906110f2565b5f870161111b565b611338565b6114ac611069565b906114c75f928392634634691b60e01b8452600484016110a1565b0390fdfe60806040526004361015610013575b610d5f565b61001d5f3561018b565b806284120c1461018657806304f1c85414610181578063056daaa61461017c578063061d12c01461017757806306f130561461017257806316bf55791461016d57806318db3940146101685780632f1ec5e9146101635780633c53a3831461015e5780634f359a37146101595780637fa3a40e146101545780638da5cb5b1461014f578063a7b51d191461014a578063ad9c0c2e14610145578063ae1a7d3014610140578063b752a7d11461013b578063d5719dc214610136578063d5954c3414610131578063d9dd67ab1461012c578063e1d66afe14610127578063e8eb1dc314610122578063eca067ad1461011d5763fbf6eaa50361000e57610d2a565b610ce6565b610cb1565b610c44565b610bd5565b610b9c565b610b29565b610ac8565b610a5a565b6109ee565b610982565b6108b6565b610808565b6107c4565b61075b565b6106c7565b610657565b610613565b6104fd565b6104c6565b6102e9565b610244565b6101d3565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f9103126101a957565b61019b565b90565b6101ba906101ae565b9052565b91906101d1905f602085019401906101b1565b565b34610203576101e336600461019f565b6101ff6101ee610d67565b6101f6610191565b918291826101be565b0390f35b610197565b1c90565b90565b61021f9060086102249302610208565b61020c565b90565b90610232915461020f565b90565b61024160035f90610227565b90565b346102745761025436600461019f565b61027061025f610235565b610267610191565b918291826101be565b0390f35b610197565b67ffffffffffffffff1690565b61029690600861029b9302610208565b610279565b90565b906102a99154610286565b90565b6102b75f5f9061029e565b90565b67ffffffffffffffff1690565b6102d0906102ba565b9052565b91906102e7905f602085019401906102c7565b565b34610319576102f936600461019f565b6103156103046102ac565b61030c610191565b918291826102d4565b0390f35b610197565b5f80fd5b5f80fd5b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906103529061032a565b810190811067ffffffffffffffff82111761036c57604052565b610334565b9061038461037d610191565b9283610348565b565b67ffffffffffffffff81116103a4576103a060209161032a565b0190565b610334565b90825f939282370152565b909291926103c96103c482610386565b610371565b938185526020850190828401116103e5576103e3926103a9565b565b610326565b9080601f8301121561040857816020610405933591016103b4565b90565b610322565b610416816102ba565b0361041d57565b5f80fd5b9050359061042e8261040d565b565b610439816101ae565b0361044057565b5f80fd5b9050359061045182610430565b565b919060a0838203126104bc575f83013567ffffffffffffffff81116104b7578161047e9185016103ea565b9261048c8260208301610421565b926104b461049d8460408501610444565b936104ab8160608601610421565b93608001610444565b90565b61031e565b61019b565b5f0190565b346104f8576104e26104d9366004610453565b9392909261115d565b6104ea610191565b806104f4816104c1565b0390f35b610197565b3461052d5761050d36600461019f565b61052961051861137c565b610520610191565b918291826101be565b0390f35b610197565b9060208282031261054b57610548915f01610444565b90565b61019b565b634e487b7160e01b5f52603260045260245ffd5b5490565b5f5260205f2090565b61057a81610564565b8210156105945761058c600191610568565b910201905f90565b610550565b90565b6105ac9060086105b19302610208565b610599565b90565b906105bf915461059c565b90565b60066105cd81610564565b8210156105ea576105e7916105e191610571565b906105b4565b90565b5f80fd5b90565b6105fa906105ee565b9052565b9190610611905f602085019401906105f1565b565b346106435761063f61062e610629366004610532565b6105c2565b610636610191565b918291826105fe565b0390f35b610197565b61065460015f90610227565b90565b346106875761066736600461019f565b610683610672610648565b61067a610191565b918291826101be565b0390f35b610197565b90565b90565b6106a66106a16106ab9261068c565b61068f565b6102ba565b90565b6106b9610e10610692565b90565b6106c46106ae565b90565b346106f7576106d736600461019f565b6106f36106e26106bc565b6106ea610191565b918291826102d4565b0390f35b610197565b90565b60ff1690565b61071961071461071e926106fc565b61068f565b6106ff565b90565b61072b600c610705565b90565b610736610721565b90565b610742906106ff565b9052565b9190610759905f60208501940190610739565b565b3461078b5761076b36600461019f565b61078761077661072e565b61077e610191565b91829182610746565b0390f35b610197565b6107a461079f6107a9926106fc565b61068f565b6102ba565b90565b6107b6600c610790565b90565b6107c16107ac565b90565b346107f4576107d436600461019f565b6107f06107df6107b9565b6107e7610191565b918291826102d4565b0390f35b610197565b61080560075f90610227565b90565b346108385761081836600461019f565b6108346108236107f9565b61082b610191565b918291826101be565b0390f35b610197565b60018060a01b031690565b61085890600861085d9302610208565b61083d565b90565b9061086b9154610848565b90565b61087a60045f90610860565b90565b60018060a01b031690565b6108919061087d565b90565b61089d90610888565b9052565b91906108b4905f60208501940190610894565b565b346108e6576108c636600461019f565b6108e26108d161086e565b6108d9610191565b918291826108a1565b0390f35b610197565b6108f4816106ff565b036108fb57565b5f80fd5b9050359061090c826108eb565b565b61091781610888565b0361091e57565b5f80fd5b9050359061092f8261090e565b565b9160608383031261097d57610948825f85016108ff565b926109568360208301610922565b92604082013567ffffffffffffffff81116109785761097592016103ea565b90565b61031e565b61019b565b346109b15761099b610995366004610931565b91611566565b6109a3610191565b806109ad816104c1565b0390f35b610197565b90565b6109cd6109c86109d2926109b6565b61068f565b6102ba565b90565b6109e0611c206109b9565b90565b6109eb6109d5565b90565b34610a1e576109fe36600461019f565b610a1a610a096109e3565b610a11610191565b918291826102d4565b0390f35b610197565b90565b610a3a610a35610a3f92610a23565b61068f565b6106ff565b90565b610a4c600b610a26565b90565b610a57610a42565b90565b34610a8a57610a6a36600461019f565b610a86610a75610a4f565b610a7d610191565b91829182610746565b0390f35b610197565b90565b610aa6610aa1610aab92610a8f565b61068f565b6102ba565b90565b610aba62015180610a92565b90565b610ac5610aae565b90565b34610af857610ad836600461019f565b610af4610ae3610abd565b610aeb610191565b918291826102d4565b0390f35b610197565b6005610b0881610564565b821015610b2557610b2291610b1c91610571565b906105b4565b90565b5f80fd5b34610b5957610b55610b44610b3f366004610532565b610afd565b610b4c610191565b918291826105fe565b0390f35b610197565b610b93610b9a94610b89606094989795610b7f608086019a5f8701906102c7565b60208501906101b1565b60408301906102c7565b01906101b1565b565b34610bd057610bac36600461019f565b610bcc610bb761175e565b90610bc3949294610191565b94859485610b5e565b0390f35b610197565b34610c0557610c01610bf0610beb366004610532565b6117b7565b610bf8610191565b918291826105fe565b0390f35b610197565b9091606082840312610c3f57610c3c610c25845f8501610922565b93610c338160208601610922565b93604001610444565b90565b61019b565b34610c7357610c5d610c57366004610c0a565b916117f6565b610c65610191565b80610c6f816104c1565b0390f35b610197565b90565b610c8f610c8a610c9492610c78565b61068f565b6102ba565b90565b610ca36201cccc610c7b565b90565b610cae610c97565b90565b34610ce157610cc136600461019f565b610cdd610ccc610ca6565b610cd4610191565b918291826102d4565b0390f35b610197565b34610d1657610cf636600461019f565b610d12610d01611836565b610d09610191565b918291826101be565b0390f35b610197565b610d2760025f9061029e565b90565b34610d5a57610d3a36600461019f565b610d56610d45610d1b565b610d4d610191565b918291826102d4565b0390f35b610197565b5f80fd5b5f90565b610d6f610d63565b50610d7a6006610564565b90565b5f1b90565b90610d9567ffffffffffffffff91610d7d565b9181191691161790565b610db3610dae610db8926102ba565b61068f565b6102ba565b90565b90565b90610dd3610dce610dda92610d9f565b610dbb565b8254610d82565b9055565b90610dea5f1991610d7d565b9181191691161790565b610e08610e03610e0d926101ae565b61068f565b6101ae565b90565b90565b90610e28610e23610e2f92610df4565b610e10565b8254610dde565b9055565b90565b610e4a610e45610e4f92610e33565b61068f565b6102ba565b90565b610e66610e61610e6b92610e33565b610d7d565b6105ee565b90565b610e82610e7d610e8792610e33565b61068f565b6101ae565b90565b90565b610ea1610e9c610ea692610e8a565b61068f565b6101ae565b90565b634e487b7160e01b5f52601160045260245ffd5b610ecc610ed2919392936101ae565b926101ae565b8203918211610edd57565b610ea9565b90565b610ef1610ef6916105ee565b610ee2565b9052565b60209392610f198583610f118295610f2197610ee5565b018092610ee5565b018092610ee5565b0190565b60200190565b5190565b90565b5f5260205f2090565b5490565b610f4881610f3b565b821015610f6257610f5a600191610f32565b910201905f90565b610550565b1b90565b91906008610f86910291610f805f1984610f67565b92610f67565b9181191691161790565b610f99906105ee565b90565b5f1c90565b610faa90610f9c565b90565b9190610fc3610fbe610fcb93610f90565b610fa1565b908354610f6b565b9055565b9081549168010000000000000000831015610fff5782610ff7916001610ffd95018155610f3f565b90610fad565b565b610334565b61101061101591610f9c565b61020c565b90565b6110229054611004565b90565b61102e906102ba565b9052565b906060806110789361104a5f8201515f860190611025565b61105c60208201516020860190611025565b61106e60408201516040860190611025565b0151910190611025565b565b634e487b7160e01b5f52602160045260245ffd5b6005111561109857565b61107a565b906110a78261108e565b565b6110b29061109d565b90565b6110be906110a9565b9052565b6110f76110fe946110ed60c0949897956110e360e086019a5f8701906105f1565b60208501906101b1565b6040830190611032565b01906110b5565b565b60209181520190565b90825f9392825e0152565b61113361113c6020936111419361112a81610f2b565b93848093611100565b95869101611109565b61032a565b0190565b61115a9160208201915f818403910152611114565b90565b939161116c611173925f610dbe565b6001610e13565b816111866111805f610e36565b916102ba565b11611360575b50506111986005610564565b6111a3828290611a8b565b906111ae6006610564565b926111b85f610e52565b846111cb6111c55f610e6e565b916101ae565b11611332575b6111da5f610e52565b92826111ee6111e85f610e6e565b916101ae565b116112fd575b611252906112238361121487611208610191565b94859360208501610efa565b60208201810382520382610348565b61123561122f82610f2b565b91610f25565b209261124b6112446006610f2f565b8590610fcf565b6007610e13565b84909192936112616007611018565b90946112b660016112a461129e6112987f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd797610df4565b97610f90565b97610f90565b976112ad610191565b948594856110c2565b0390a46112f86112e67ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c2092610df4565b926112ef610191565b91829182611145565b0390a2565b925061125261132a611324600561131e866113186001610e8d565b90610ebd565b90610571565b906105b4565b9390506111f4565b5061135b611355600661134f876113496001610e8d565b90610ebd565b90610571565b906105b4565b6111d1565b61136e611375926002610dbe565b6003610e13565b5f8061118c565b611384610d63565b5061138f6006610564565b90565b6113a66113a16113ab926101ae565b61068f565b6102ba565b90565b60f81b90565b6113bd906113ae565b90565b6113cc6113d1916106ff565b6113b4565b9052565b60601b90565b6113e4906113d5565b90565b6113f0906113db565b90565b6113ff61140491610888565b6113e7565b9052565b60c01b90565b61141790611408565b90565b61142661142b916102ba565b61140e565b9052565b90565b61143e611443916101ae565b61142f565b9052565b946114986008602099989596611490828c9961148860146114a09a6114806114a89f8f9c90611478816001936113c0565b0180926113f3565b01809261141a565b01809261141a565b018092611432565b018092611432565b018092610ee5565b0190565b6020816114be6114c693839695610ee5565b018092610ee5565b0190565b6114de6114d96114e39261087d565b61068f565b61087d565b90565b6114ef906114ca565b90565b6114fb906114e6565b90565b61150790610e6e565b9052565b919461155361155d9298979561154960a09661153f6115649a61153560c08a019e5f8b0190610894565b6020890190610739565b6040870190610894565b60608501906105f1565b60808301906114fe565b01906102c7565b565b906115716005610564565b918361158561157f82610f2b565b91610f25565b2091816115d282916115c361159943611392565b6115a242611392565b896115ac5f610e6e565b918a936115b7610191565b98899760208901611447565b60208201810382520382610348565b6115e46115de82610f2b565b91610f25565b20906115ef5f610e52565b91856116036115fd5f610e6e565b916101ae565b11611704575b61165b906116176005610f2f565b9061164285611633611627610191565b938492602084016114ac565b60208201810382520382610348565b61165461164e82610f2b565b91610f25565b2090610fcf565b8491926116bd61166a306114f2565b9192955f61167742611392565b916116ab6116a57f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe198610df4565b98610f90565b986116b4610191565b9687968761150b565b0390a36116ff6116ed7fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b92610df4565b926116f6610191565b91829182611145565b0390a2565b915061165b61173161172b60056117258961171f6001610e8d565b90610ebd565b90610571565b906105b4565b929050611609565b5f90565b61174961174e91610f9c565b610279565b90565b61175b905461173d565b90565b611766611739565b5061176f610d63565b50611778611739565b50611781610d63565b5061178b5f611751565b906117966001611018565b916117a16002611751565b916117ac6003611018565b9193929190565b5f90565b6117ce6117d4916117c66117b3565b506006610571565b906105b4565b90565b6014816117ea6117f293602096956113f3565b018092611432565b0190565b90611834929161182f611807610721565b9192611820611814610191565b958692602084016117d7565b60208201810382520384610348565b611566565b565b61183e610d63565b506118496005610564565b90565b6118566080610371565b90565b5f90565b61186561184c565b90602080808085611874611859565b81520161187f611859565b81520161188a611859565b815201611895611859565b81525050565b6118a361185d565b90565b90565b6118bd6118b86118c2926118a6565b61068f565b6101ae565b90565b6118cf60286118a9565b90565b6118e16118e7919392936101ae565b926101ae565b82018092116118f257565b610ea9565b61190b611906611910926102ba565b61068f565b6101ae565b90565b61191c906118f7565b9052565b91602061194192949361193a60408201965f8301906101b1565b0190611913565b565b61194f611955916102ba565b916102ba565b90039067ffffffffffffffff821161196957565b610ea9565b90611978906102ba565b9052565b61198861198e916102ba565b916102ba565b019067ffffffffffffffff82116119a157565b610ea9565b6119b090516102ba565b90565b60086119f5946119e58280999895966119dd82876119d56119ed99839c61141a565b01809261141a565b01809261141a565b01809261141a565b01809261141a565b0190565b634e487b7160e01b5f52600160045260245ffd5b15611a1457565b6119f9565b905090565b611a43611a3a92602092611a3181610f2b565b94858093611a19565b93849101611109565b0190565b611a5590611a5b9392611a1e565b90611a1e565b90565b611a7d9291611a8991611a6f610191565b948592602084019283611a47565b90810382520383610348565b565b919091611a966117b3565b50611a9f61189b565b50611aba611aab6118c5565b611ab483610f2b565b906118d2565b80611ad4611ace611ac9610c97565b6118f7565b916101ae565b11611c735750611c0790611ae661189b565b9342611b01611afb611af6610aae565b6118f7565b916101ae565b11611c49575b611b2d611b24611b1642611392565b611b1e6106ae565b9061197c565b6020870161196e565b43611b47611b41611b3c6109d5565b6118f7565b916101ae565b11611c1e575b611b73611b6a611b5c43611392565b611b646107ac565b9061197c565b6060870161196e565b611bd7611b815f87016119a6565b611bc8611b90602089016119a6565b93611b9d60408a016119a6565b90611bb3611bad60608c016119a6565b91611392565b91611bbc610191565b968795602087016119b3565b60208201810382520382610348565b611c02611be382610f2b565b611bfc611bf6611bf16118c5565b6101ae565b916101ae565b14611a0d565b611a5e565b611c19611c1382610f2b565b91610f25565b209190565b611c44611c3b611c2d43611392565b611c356109d5565b90611943565b6040870161196e565b611b4d565b611c6e611c66611c5842611392565b611c60610aae565b90611943565b5f870161196e565b611b07565b611c7b610c97565b90611c965f928392634634691b60e01b845260048401611920565b0390fd
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4a\x000Wa\0\x1Aa\0\x14a\x01\xDFV[\x91a\x055V[a\0\"a\x005V[a\x1C\x9Aa\x14\xCC\x829a\x1C\x9A\x90\xF3[a\0;V[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0g\x90a\0?V[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\x7FW`@RV[a\0IV[\x90a\0\x97a\0\x90a\x005V[\x92\x83a\0]V[V[_\x80\xFD[_\x80\xFD[\x90V[a\0\xAD\x81a\0\xA1V[\x03a\0\xB4WV[_\x80\xFD[\x90PQ\x90a\0\xC5\x82a\0\xA4V[V[_\x80\xFD[_\x80\xFD[`\x01\x80`@\x1B\x03\x81\x11a\0\xEBWa\0\xE7` \x91a\0?V[\x01\x90V[a\0IV[\x90\x82_\x93\x92\x82^\x01RV[\x90\x92\x91\x92a\x01\x10a\x01\x0B\x82a\0\xCFV[a\0\x84V[\x93\x81\x85R` \x85\x01\x90\x82\x84\x01\x11a\x01,Wa\x01*\x92a\0\xF0V[V[a\0\xCBV[\x90\x80`\x1F\x83\x01\x12\x15a\x01OW\x81` a\x01L\x93Q\x91\x01a\0\xFBV[\x90V[a\0\xC7V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x01h\x90a\x01TV[\x90V[a\x01t\x81a\x01_V[\x03a\x01{WV[_\x80\xFD[\x90PQ\x90a\x01\x8C\x82a\x01kV[V[\x90\x91``\x82\x84\x03\x12a\x01\xDAWa\x01\xA6\x83_\x84\x01a\0\xB8V[\x92` \x83\x01Q\x90`\x01\x80`@\x1B\x03\x82\x11a\x01\xD5Wa\x01\xC9\x81a\x01\xD2\x93\x86\x01a\x011V[\x93`@\x01a\x01\x7FV[\x90V[a\0\x9DV[a\0\x99V[a\x01\xFDa1f\x808\x03\x80a\x01\xF2\x81a\0\x84V[\x92\x839\x81\x01\x90a\x01\x8EV[\x90\x91\x92V[_\x1B\x90V[\x90a\x02\x18`\x01\x80`@\x1B\x03\x91a\x02\x02V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[`\x01\x80`@\x1B\x03\x16\x90V[\x90V[a\x02Ga\x02Ba\x02L\x92a\x02\"V[a\x020V[a\x02%V[\x90V[\x90V[\x90a\x02ga\x02ba\x02n\x92a\x023V[a\x02OV[\x82Ta\x02\x07V[\x90UV[\x90a\x02~_\x19\x91a\x02\x02V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x02\x9Ca\x02\x97a\x02\xA1\x92a\x02\"V[a\x020V[a\0\xA1V[\x90V[\x90V[\x90a\x02\xBCa\x02\xB7a\x02\xC3\x92a\x02\x88V[a\x02\xA4V[\x82Ta\x02rV[\x90UV[\x90a\x02\xD8`\x01\x80`\xA0\x1B\x03\x91a\x02\x02V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x02\xF6a\x02\xF1a\x02\xFB\x92a\x01TV[a\x020V[a\x01TV[\x90V[a\x03\x07\x90a\x02\xE2V[\x90V[a\x03\x13\x90a\x02\xFEV[\x90V[\x90V[\x90a\x03.a\x03)a\x035\x92a\x03\nV[a\x03\x16V[\x82Ta\x02\xC7V[\x90UV[\x90V[Q\x90V[` \x91\x81R\x01\x90V[_\x7FEMPTY_CHAIN_CONFIG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x03}`\x12` \x92a\x03@V[a\x03\x86\x81a\x03IV[\x01\x90V[a\x03\x9F\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x03pV[\x90V[\x15a\x03\xA9WV[a\x03\xB1a\x005V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x03\xC7`\x04\x82\x01a\x03\x8AV[\x03\x90\xFD[\x90V[`\xFF\x16\x90V[a\x03\xE8a\x03\xE3a\x03\xED\x92a\x03\xCBV[a\x020V[a\x03\xCEV[\x90V[a\x03\xFA`\x0Ba\x03\xD4V[\x90V[a\x04\x11a\x04\x0Ca\x04\x16\x92a\x02\"V[a\x020V[a\x01TV[\x90V[a\x04\"\x90a\x03\xFDV[\x90V[\x90V[a\x04<a\x047a\x04A\x92a\x04%V[a\x020V[a\x03\xCEV[\x90V[\x90V[a\x04Sa\x04X\x91a\0\xA1V[a\x04DV[\x90RV[`\xF8\x1B\x90V[a\x04k\x90a\x04\\V[\x90V[a\x04za\x04\x7F\x91a\x03\xCEV[a\x04bV[\x90RV[Q\x90V[\x90P\x90V[a\x04\xB1a\x04\xA8\x92` \x92a\x04\x9F\x81a\x04\x83V[\x94\x85\x80\x93a\x04\x87V[\x93\x84\x91\x01a\0\xF0V[\x01\x90V[`\x01` \x93a\x04\xD9\x85\x84a\x04\xD1a\x04\xE1\x96a\x04\xE8\x9B\x9A\x98a\x04GV[\x01\x80\x92a\x04nV[\x01\x80\x92a\x04GV[\x01\x90a\x04\x8CV[\x90V[\x90a\x04\xFDa\x04\xF8\x83a\0\xCFV[a\0\x84V[\x91\x82RV[_d\x0B\0\x80\x02\x03`\xD0\x1B\x91\x01RV[a\x05\x1B`\x06a\x04\xEBV[\x90a\x05(` \x83\x01a\x05\x02V[V[a\x052a\x05\x11V[\x90V[a\x05na\x05\xEE\x93a\x05F__a\x02RV[a\x05Q_`\x01a\x02\xA7V[a\x05\\_`\x02a\x02RV[a\x05g_`\x03a\x02\xA7V[`\x04a\x03\x19V[a\x05\x9Aa\x05\x82a\x05}\x84a\x039V[a\x03<V[a\x05\x94a\x05\x8E_a\x02\x88V[\x91a\0\xA1V[\x11a\x03\xA2V[a\x05\xA2a\x03\xF0V[a\x05\xE9a\x05\xAE_a\x04\x19V[\x92a\x05\xDAa\x05\xBC`\x01a\x04(V[\x95a\x05\xC6_a\x02\x88V[a\x05\xCEa\x005V[\x97\x88\x94` \x86\x01a\x04\xB5V[` \x82\x01\x81\x03\x82R\x03\x84a\0]V[a\n\x16V[a\x06(____\x91a\x06\"a\x06\x1Ca\x06\x16a\x06\x10a\x06\na\x05*V[\x97a\x023V[\x93a\x02\x88V[\x93a\x023V[\x93a\x02\x88V[\x93a\r|V[V[T\x90V[` \x01\x90V[a\x06Ha\x06Ca\x06M\x92a\0\xA1V[a\x020V[a\x02%V[\x90V[``\x1B\x90V[a\x06_\x90a\x06PV[\x90V[a\x06k\x90a\x06VV[\x90V[a\x06za\x06\x7F\x91a\x01_V[a\x06bV[\x90RV[`\xC0\x1B\x90V[a\x06\x92\x90a\x06\x83V[\x90V[a\x06\xA1a\x06\xA6\x91a\x02%V[a\x06\x89V[\x90RV[\x90V[\x90V[a\x06\xBCa\x06\xC1\x91a\x06\xAAV[a\x06\xADV[\x90RV[\x94a\x07\x16`\x08` \x99\x98\x95\x96a\x07\x0E\x82\x8C\x99a\x07\x06`\x14a\x07\x1E\x9Aa\x06\xFEa\x07&\x9F\x8F\x9C\x90a\x06\xF6\x81`\x01\x93a\x04nV[\x01\x80\x92a\x06nV[\x01\x80\x92a\x06\x95V[\x01\x80\x92a\x06\x95V[\x01\x80\x92a\x04GV[\x01\x80\x92a\x04GV[\x01\x80\x92a\x06\xB0V[\x01\x90V[a\x07>a\x079a\x07C\x92a\x02\"V[a\x02\x02V[a\x06\xAAV[\x90V[a\x07Za\x07Ua\x07_\x92a\x04%V[a\x020V[a\0\xA1V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x07\x85a\x07\x8B\x91\x93\x92\x93a\0\xA1V[\x92a\0\xA1V[\x82\x03\x91\x82\x11a\x07\x96WV[a\x07bV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_R` _ \x90V[a\x07\xC1\x81a\x06*V[\x82\x10\x15a\x07\xDBWa\x07\xD3`\x01\x91a\x07\xAFV[\x91\x02\x01\x90_\x90V[a\x07\x9BV[\x1C\x90V[\x90V[a\x07\xF7\x90`\x08a\x07\xFC\x93\x02a\x07\xE0V[a\x07\xE4V[\x90V[\x90a\x08\n\x91Ta\x07\xE7V[\x90V[\x90V[` \x81a\x08\"a\x08*\x93\x83\x96\x95a\x06\xB0V[\x01\x80\x92a\x06\xB0V[\x01\x90V[_R` _ \x90V[T\x90V[a\x08D\x81a\x087V[\x82\x10\x15a\x08^Wa\x08V`\x01\x91a\x08.V[\x91\x02\x01\x90_\x90V[a\x07\x9BV[\x1B\x90V[\x91\x90`\x08a\x08\x82\x91\x02\x91a\x08|_\x19\x84a\x08cV[\x92a\x08cV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x08\x95\x90a\x06\xAAV[\x90V[_\x1C\x90V[a\x08\xA6\x90a\x08\x98V[\x90V[\x91\x90a\x08\xBFa\x08\xBAa\x08\xC7\x93a\x08\x8CV[a\x08\x9DV[\x90\x83Ta\x08gV[\x90UV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x08\xFBW\x82a\x08\xF3\x91`\x01a\x08\xF9\x95\x01\x81Ua\x08;V[\x90a\x08\xA9V[V[a\0IV[a\t\t\x90a\x02\xFEV[\x90V[a\t a\t\x1Ba\t%\x92a\0\xA1V[a\x020V[a\0\xA1V[\x90V[a\t1\x90a\x01_V[\x90RV[a\t>\x90a\x03\xCEV[\x90RV[a\tK\x90a\x06\xAAV[\x90RV[a\tX\x90a\x02\x88V[\x90RV[a\te\x90a\x02%V[\x90RV[\x91\x94a\t\xB1a\t\xBB\x92\x98\x97\x95a\t\xA7`\xA0\x96a\t\x9Da\t\xC2\x9Aa\t\x93`\xC0\x8A\x01\x9E_\x8B\x01\x90a\t(V[` \x89\x01\x90a\t5V[`@\x87\x01\x90a\t(V[``\x85\x01\x90a\tBV[`\x80\x83\x01\x90a\tOV[\x01\x90a\t\\V[V[` \x91\x81R\x01\x90V[a\t\xECa\t\xF5` \x93a\t\xFA\x93a\t\xE3\x81a\x03<V[\x93\x84\x80\x93a\t\xC4V[\x95\x86\x91\x01a\0\xF0V[a\0?V[\x01\x90V[a\n\x13\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\t\xCDV[\x90V[\x90a\n!`\x05a\x06*V[\x91\x83a\n5a\n/\x82a\x03<V[\x91a\x06.V[ \x91\x81a\n\x82\x82\x91a\nsa\nICa\x064V[a\nRBa\x064V[\x89a\n\\_a\x02\x88V[\x91\x8A\x93a\nga\x005V[\x98\x89\x97` \x89\x01a\x06\xC5V[` \x82\x01\x81\x03\x82R\x03\x82a\0]V[a\n\x94a\n\x8E\x82a\x03<V[\x91a\x06.V[ \x90a\n\x9F_a\x07*V[\x91\x85a\n\xB3a\n\xAD_a\x02\x88V[\x91a\0\xA1V[\x11a\x0B\xB4W[a\x0B\x0B\x90a\n\xC7`\x05a\x08\rV[\x90a\n\xF2\x85a\n\xE3a\n\xD7a\x005V[\x93\x84\x92` \x84\x01a\x08\x10V[` \x82\x01\x81\x03\x82R\x03\x82a\0]V[a\x0B\x04a\n\xFE\x82a\x03<V[\x91a\x06.V[ \x90a\x08\xCBV[\x84\x91\x92a\x0Bma\x0B\x1A0a\t\0V[\x91\x92\x95_a\x0B'Ba\x064V[\x91a\x0B[a\x0BU\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE1\x98a\t\x0CV[\x98a\x08\x8CV[\x98a\x0Bda\x005V[\x96\x87\x96\x87a\tiV[\x03\x90\xA3a\x0B\xAFa\x0B\x9D\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x92a\t\x0CV[\x92a\x0B\xA6a\x005V[\x91\x82\x91\x82a\t\xFEV[\x03\x90\xA2V[\x91Pa\x0B\x0Ba\x0B\xE1a\x0B\xDB`\x05a\x0B\xD5\x89a\x0B\xCF`\x01a\x07FV[\x90a\x07vV[\x90a\x07\xB8V[\x90a\x07\xFFV[\x92\x90Pa\n\xB9V[a\x0B\xFDa\x0B\xF8a\x0C\x02\x92a\x02%V[a\x020V[a\x02%V[\x90V[\x90a\x0C\x1Aa\x0C\x15a\x0C!\x92a\x0B\xE9V[a\x02OV[\x82Ta\x02\x07V[\x90UV[\x90a\x0C:a\x0C5a\x0CA\x92a\t\x0CV[a\x02\xA4V[\x82Ta\x02rV[\x90UV[` \x93\x92a\x0Cd\x85\x83a\x0C\\\x82\x95a\x0Cl\x97a\x06\xB0V[\x01\x80\x92a\x06\xB0V[\x01\x80\x92a\x06\xB0V[\x01\x90V[\x90V[a\x0C\x7Fa\x0C\x84\x91a\x08\x98V[a\x0CpV[\x90V[a\x0C\x91\x90Ta\x0CsV[\x90V[a\x0C\x9D\x90a\0\xA1V[\x90RV[a\x0C\xAA\x90a\x02%V[\x90RV[\x90``\x80a\x0C\xF4\x93a\x0C\xC6_\x82\x01Q_\x86\x01\x90a\x0C\xA1V[a\x0C\xD8` \x82\x01Q` \x86\x01\x90a\x0C\xA1V[a\x0C\xEA`@\x82\x01Q`@\x86\x01\x90a\x0C\xA1V[\x01Q\x91\x01\x90a\x0C\xA1V[V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x05\x11\x15a\r\x14WV[a\x0C\xF6V[\x90a\r#\x82a\r\nV[V[a\r.\x90a\r\x19V[\x90V[a\r:\x90a\r%V[\x90RV[a\rsa\rz\x94a\ri`\xC0\x94\x98\x97\x95a\r_`\xE0\x86\x01\x9A_\x87\x01\x90a\tBV[` \x85\x01\x90a\x0C\x94V[`@\x83\x01\x90a\x0C\xAEV[\x01\x90a\r1V[V[\x93\x91a\r\x8Ba\r\x92\x92_a\x0C\x05V[`\x01a\x0C%V[\x81a\r\xA5a\r\x9F_a\x023V[\x91a\x02%V[\x11a\x0F\x7FW[PPa\r\xB7`\x05a\x06*V[a\r\xC2\x82\x82\x90a\x12\xBCV[\x90a\r\xCD`\x06a\x06*V[\x92a\r\xD7_a\x07*V[\x84a\r\xEAa\r\xE4_a\x02\x88V[\x91a\0\xA1V[\x11a\x0FQW[a\r\xF9_a\x07*V[\x92\x82a\x0E\ra\x0E\x07_a\x02\x88V[\x91a\0\xA1V[\x11a\x0F\x1CW[a\x0Eq\x90a\x0EB\x83a\x0E3\x87a\x0E'a\x005V[\x94\x85\x93` \x85\x01a\x0CEV[` \x82\x01\x81\x03\x82R\x03\x82a\0]V[a\x0ETa\x0EN\x82a\x03<V[\x91a\x06.V[ \x92a\x0Eja\x0Ec`\x06a\x08\rV[\x85\x90a\x08\xCBV[`\x07a\x0C%V[\x84\x90\x91\x92\x93a\x0E\x80`\x07a\x0C\x87V[\x90\x94a\x0E\xD5`\x01a\x0E\xC3a\x0E\xBDa\x0E\xB7\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x97a\t\x0CV[\x97a\x08\x8CV[\x97a\x08\x8CV[\x97a\x0E\xCCa\x005V[\x94\x85\x94\x85a\r>V[\x03\x90\xA4a\x0F\x17a\x0F\x05\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x92a\t\x0CV[\x92a\x0F\x0Ea\x005V[\x91\x82\x91\x82a\t\xFEV[\x03\x90\xA2V[\x92Pa\x0Eqa\x0FIa\x0FC`\x05a\x0F=\x86a\x0F7`\x01a\x07FV[\x90a\x07vV[\x90a\x07\xB8V[\x90a\x07\xFFV[\x93\x90Pa\x0E\x13V[Pa\x0Fza\x0Ft`\x06a\x0Fn\x87a\x0Fh`\x01a\x07FV[\x90a\x07vV[\x90a\x07\xB8V[\x90a\x07\xFFV[a\r\xF0V[a\x0F\x8Da\x0F\x94\x92`\x02a\x0C\x05V[`\x03a\x0C%V[_\x80a\r\xABV[_\x90V[a\x0F\xA9`\x80a\0\x84V[\x90V[_\x90V[a\x0F\xB8a\x0F\x9FV[\x90` \x80\x80\x80\x85a\x0F\xC7a\x0F\xACV[\x81R\x01a\x0F\xD2a\x0F\xACV[\x81R\x01a\x0F\xDDa\x0F\xACV[\x81R\x01a\x0F\xE8a\x0F\xACV[\x81RPPV[a\x0F\xF6a\x0F\xB0V[\x90V[\x90V[a\x10\x10a\x10\x0Ba\x10\x15\x92a\x0F\xF9V[a\x020V[a\0\xA1V[\x90V[a\x10\"`(a\x0F\xFCV[\x90V[a\x104a\x10:\x91\x93\x92\x93a\0\xA1V[\x92a\0\xA1V[\x82\x01\x80\x92\x11a\x10EWV[a\x07bV[\x90V[a\x10aa\x10\\a\x10f\x92a\x10JV[a\x020V[a\x02%V[\x90V[a\x10ub\x01\xCC\xCCa\x10MV[\x90V[a\x10\x8Ca\x10\x87a\x10\x91\x92a\x02%V[a\x020V[a\0\xA1V[\x90V[a\x10\x9D\x90a\x10xV[\x90RV[\x91` a\x10\xC2\x92\x94\x93a\x10\xBB`@\x82\x01\x96_\x83\x01\x90a\x0C\x94V[\x01\x90a\x10\x94V[V[\x90V[a\x10\xDBa\x10\xD6a\x10\xE0\x92a\x10\xC4V[a\x020V[a\x02%V[\x90V[a\x10\xEFb\x01Q\x80a\x10\xC7V[\x90V[a\x10\xFEa\x11\x04\x91a\x02%V[\x91a\x02%V[\x90\x03\x90`\x01\x80`@\x1B\x03\x82\x11a\x11\x16WV[a\x07bV[\x90a\x11%\x90a\x02%V[\x90RV[\x90V[a\x11@a\x11;a\x11E\x92a\x11)V[a\x020V[a\x02%V[\x90V[a\x11Sa\x0E\x10a\x11,V[\x90V[a\x11ba\x11h\x91a\x02%V[\x91a\x02%V[\x01\x90`\x01\x80`@\x1B\x03\x82\x11a\x11yWV[a\x07bV[\x90V[a\x11\x95a\x11\x90a\x11\x9A\x92a\x11~V[a\x020V[a\x02%V[\x90V[a\x11\xA8a\x1C a\x11\x81V[\x90V[\x90V[a\x11\xC2a\x11\xBDa\x11\xC7\x92a\x11\xABV[a\x020V[a\x02%V[\x90V[a\x11\xD4`\x0Ca\x11\xAEV[\x90V[a\x11\xE1\x90Qa\x02%V[\x90V[`\x08a\x12&\x94a\x12\x16\x82\x80\x99\x98\x95\x96a\x12\x0E\x82\x87a\x12\x06a\x12\x1E\x99\x83\x9Ca\x06\x95V[\x01\x80\x92a\x06\x95V[\x01\x80\x92a\x06\x95V[\x01\x80\x92a\x06\x95V[\x01\x80\x92a\x06\x95V[\x01\x90V[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[\x15a\x12EWV[a\x12*V[\x90P\x90V[a\x12ta\x12k\x92` \x92a\x12b\x81a\x03<V[\x94\x85\x80\x93a\x12JV[\x93\x84\x91\x01a\0\xF0V[\x01\x90V[a\x12\x86\x90a\x12\x8C\x93\x92a\x12OV[\x90a\x12OV[\x90V[a\x12\xAE\x92\x91a\x12\xBA\x91a\x12\xA0a\x005V[\x94\x85\x92` \x84\x01\x92\x83a\x12xV[\x90\x81\x03\x82R\x03\x83a\0]V[V[\x91\x90\x91a\x12\xC7a\x0F\x9BV[Pa\x12\xD0a\x0F\xEEV[Pa\x12\xEBa\x12\xDCa\x10\x18V[a\x12\xE5\x83a\x03<V[\x90a\x10%V[\x80a\x13\x05a\x12\xFFa\x12\xFAa\x10iV[a\x10xV[\x91a\0\xA1V[\x11a\x14\xA4WPa\x148\x90a\x13\x17a\x0F\xEEV[\x93Ba\x132a\x13,a\x13'a\x10\xE3V[a\x10xV[\x91a\0\xA1V[\x11a\x14zW[a\x13^a\x13Ua\x13GBa\x064V[a\x13Oa\x11HV[\x90a\x11VV[` \x87\x01a\x11\x1BV[Ca\x13xa\x13ra\x13ma\x11\x9DV[a\x10xV[\x91a\0\xA1V[\x11a\x14OW[a\x13\xA4a\x13\x9Ba\x13\x8DCa\x064V[a\x13\x95a\x11\xCAV[\x90a\x11VV[``\x87\x01a\x11\x1BV[a\x14\x08a\x13\xB2_\x87\x01a\x11\xD7V[a\x13\xF9a\x13\xC1` \x89\x01a\x11\xD7V[\x93a\x13\xCE`@\x8A\x01a\x11\xD7V[\x90a\x13\xE4a\x13\xDE``\x8C\x01a\x11\xD7V[\x91a\x064V[\x91a\x13\xEDa\x005V[\x96\x87\x95` \x87\x01a\x11\xE4V[` \x82\x01\x81\x03\x82R\x03\x82a\0]V[a\x143a\x14\x14\x82a\x03<V[a\x14-a\x14'a\x14\"a\x10\x18V[a\0\xA1V[\x91a\0\xA1V[\x14a\x12>V[a\x12\x8FV[a\x14Ja\x14D\x82a\x03<V[\x91a\x06.V[ \x91\x90V[a\x14ua\x14la\x14^Ca\x064V[a\x14fa\x11\x9DV[\x90a\x10\xF2V[`@\x87\x01a\x11\x1BV[a\x13~V[a\x14\x9Fa\x14\x97a\x14\x89Ba\x064V[a\x14\x91a\x10\xE3V[\x90a\x10\xF2V[_\x87\x01a\x11\x1BV[a\x138V[a\x14\xACa\x10iV[\x90a\x14\xC7_\x92\x83\x92cF4i\x1B`\xE0\x1B\x84R`\x04\x84\x01a\x10\xA1V[\x03\x90\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\r_V[a\0\x1D_5a\x01\x8BV[\x80b\x84\x12\x0C\x14a\x01\x86W\x80c\x04\xF1\xC8T\x14a\x01\x81W\x80c\x05m\xAA\xA6\x14a\x01|W\x80c\x06\x1D\x12\xC0\x14a\x01wW\x80c\x06\xF10V\x14a\x01rW\x80c\x16\xBFUy\x14a\x01mW\x80c\x18\xDB9@\x14a\x01hW\x80c/\x1E\xC5\xE9\x14a\x01cW\x80c<S\xA3\x83\x14a\x01^W\x80cO5\x9A7\x14a\x01YW\x80c\x7F\xA3\xA4\x0E\x14a\x01TW\x80c\x8D\xA5\xCB[\x14a\x01OW\x80c\xA7\xB5\x1D\x19\x14a\x01JW\x80c\xAD\x9C\x0C.\x14a\x01EW\x80c\xAE\x1A}0\x14a\x01@W\x80c\xB7R\xA7\xD1\x14a\x01;W\x80c\xD5q\x9D\xC2\x14a\x016W\x80c\xD5\x95L4\x14a\x011W\x80c\xD9\xDDg\xAB\x14a\x01,W\x80c\xE1\xD6j\xFE\x14a\x01'W\x80c\xE8\xEB\x1D\xC3\x14a\x01\"W\x80c\xEC\xA0g\xAD\x14a\x01\x1DWc\xFB\xF6\xEA\xA5\x03a\0\x0EWa\r*V[a\x0C\xE6V[a\x0C\xB1V[a\x0CDV[a\x0B\xD5V[a\x0B\x9CV[a\x0B)V[a\n\xC8V[a\nZV[a\t\xEEV[a\t\x82V[a\x08\xB6V[a\x08\x08V[a\x07\xC4V[a\x07[V[a\x06\xC7V[a\x06WV[a\x06\x13V[a\x04\xFDV[a\x04\xC6V[a\x02\xE9V[a\x02DV[a\x01\xD3V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01\xA9WV[a\x01\x9BV[\x90V[a\x01\xBA\x90a\x01\xAEV[\x90RV[\x91\x90a\x01\xD1\x90_` \x85\x01\x94\x01\x90a\x01\xB1V[V[4a\x02\x03Wa\x01\xE36`\x04a\x01\x9FV[a\x01\xFFa\x01\xEEa\rgV[a\x01\xF6a\x01\x91V[\x91\x82\x91\x82a\x01\xBEV[\x03\x90\xF3[a\x01\x97V[\x1C\x90V[\x90V[a\x02\x1F\x90`\x08a\x02$\x93\x02a\x02\x08V[a\x02\x0CV[\x90V[\x90a\x022\x91Ta\x02\x0FV[\x90V[a\x02A`\x03_\x90a\x02'V[\x90V[4a\x02tWa\x02T6`\x04a\x01\x9FV[a\x02pa\x02_a\x025V[a\x02ga\x01\x91V[\x91\x82\x91\x82a\x01\xBEV[\x03\x90\xF3[a\x01\x97V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\x96\x90`\x08a\x02\x9B\x93\x02a\x02\x08V[a\x02yV[\x90V[\x90a\x02\xA9\x91Ta\x02\x86V[\x90V[a\x02\xB7__\x90a\x02\x9EV[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\xD0\x90a\x02\xBAV[\x90RV[\x91\x90a\x02\xE7\x90_` \x85\x01\x94\x01\x90a\x02\xC7V[V[4a\x03\x19Wa\x02\xF96`\x04a\x01\x9FV[a\x03\x15a\x03\x04a\x02\xACV[a\x03\x0Ca\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x03R\x90a\x03*V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03lW`@RV[a\x034V[\x90a\x03\x84a\x03}a\x01\x91V[\x92\x83a\x03HV[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\xA4Wa\x03\xA0` \x91a\x03*V[\x01\x90V[a\x034V[\x90\x82_\x93\x92\x827\x01RV[\x90\x92\x91\x92a\x03\xC9a\x03\xC4\x82a\x03\x86V[a\x03qV[\x93\x81\x85R` \x85\x01\x90\x82\x84\x01\x11a\x03\xE5Wa\x03\xE3\x92a\x03\xA9V[V[a\x03&V[\x90\x80`\x1F\x83\x01\x12\x15a\x04\x08W\x81` a\x04\x05\x935\x91\x01a\x03\xB4V[\x90V[a\x03\"V[a\x04\x16\x81a\x02\xBAV[\x03a\x04\x1DWV[_\x80\xFD[\x90P5\x90a\x04.\x82a\x04\rV[V[a\x049\x81a\x01\xAEV[\x03a\x04@WV[_\x80\xFD[\x90P5\x90a\x04Q\x82a\x040V[V[\x91\x90`\xA0\x83\x82\x03\x12a\x04\xBCW_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xB7W\x81a\x04~\x91\x85\x01a\x03\xEAV[\x92a\x04\x8C\x82` \x83\x01a\x04!V[\x92a\x04\xB4a\x04\x9D\x84`@\x85\x01a\x04DV[\x93a\x04\xAB\x81``\x86\x01a\x04!V[\x93`\x80\x01a\x04DV[\x90V[a\x03\x1EV[a\x01\x9BV[_\x01\x90V[4a\x04\xF8Wa\x04\xE2a\x04\xD96`\x04a\x04SV[\x93\x92\x90\x92a\x11]V[a\x04\xEAa\x01\x91V[\x80a\x04\xF4\x81a\x04\xC1V[\x03\x90\xF3[a\x01\x97V[4a\x05-Wa\x05\r6`\x04a\x01\x9FV[a\x05)a\x05\x18a\x13|V[a\x05 a\x01\x91V[\x91\x82\x91\x82a\x01\xBEV[\x03\x90\xF3[a\x01\x97V[\x90` \x82\x82\x03\x12a\x05KWa\x05H\x91_\x01a\x04DV[\x90V[a\x01\x9BV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[T\x90V[_R` _ \x90V[a\x05z\x81a\x05dV[\x82\x10\x15a\x05\x94Wa\x05\x8C`\x01\x91a\x05hV[\x91\x02\x01\x90_\x90V[a\x05PV[\x90V[a\x05\xAC\x90`\x08a\x05\xB1\x93\x02a\x02\x08V[a\x05\x99V[\x90V[\x90a\x05\xBF\x91Ta\x05\x9CV[\x90V[`\x06a\x05\xCD\x81a\x05dV[\x82\x10\x15a\x05\xEAWa\x05\xE7\x91a\x05\xE1\x91a\x05qV[\x90a\x05\xB4V[\x90V[_\x80\xFD[\x90V[a\x05\xFA\x90a\x05\xEEV[\x90RV[\x91\x90a\x06\x11\x90_` \x85\x01\x94\x01\x90a\x05\xF1V[V[4a\x06CWa\x06?a\x06.a\x06)6`\x04a\x052V[a\x05\xC2V[a\x066a\x01\x91V[\x91\x82\x91\x82a\x05\xFEV[\x03\x90\xF3[a\x01\x97V[a\x06T`\x01_\x90a\x02'V[\x90V[4a\x06\x87Wa\x06g6`\x04a\x01\x9FV[a\x06\x83a\x06ra\x06HV[a\x06za\x01\x91V[\x91\x82\x91\x82a\x01\xBEV[\x03\x90\xF3[a\x01\x97V[\x90V[\x90V[a\x06\xA6a\x06\xA1a\x06\xAB\x92a\x06\x8CV[a\x06\x8FV[a\x02\xBAV[\x90V[a\x06\xB9a\x0E\x10a\x06\x92V[\x90V[a\x06\xC4a\x06\xAEV[\x90V[4a\x06\xF7Wa\x06\xD76`\x04a\x01\x9FV[a\x06\xF3a\x06\xE2a\x06\xBCV[a\x06\xEAa\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[\x90V[`\xFF\x16\x90V[a\x07\x19a\x07\x14a\x07\x1E\x92a\x06\xFCV[a\x06\x8FV[a\x06\xFFV[\x90V[a\x07+`\x0Ca\x07\x05V[\x90V[a\x076a\x07!V[\x90V[a\x07B\x90a\x06\xFFV[\x90RV[\x91\x90a\x07Y\x90_` \x85\x01\x94\x01\x90a\x079V[V[4a\x07\x8BWa\x07k6`\x04a\x01\x9FV[a\x07\x87a\x07va\x07.V[a\x07~a\x01\x91V[\x91\x82\x91\x82a\x07FV[\x03\x90\xF3[a\x01\x97V[a\x07\xA4a\x07\x9Fa\x07\xA9\x92a\x06\xFCV[a\x06\x8FV[a\x02\xBAV[\x90V[a\x07\xB6`\x0Ca\x07\x90V[\x90V[a\x07\xC1a\x07\xACV[\x90V[4a\x07\xF4Wa\x07\xD46`\x04a\x01\x9FV[a\x07\xF0a\x07\xDFa\x07\xB9V[a\x07\xE7a\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[a\x08\x05`\x07_\x90a\x02'V[\x90V[4a\x088Wa\x08\x186`\x04a\x01\x9FV[a\x084a\x08#a\x07\xF9V[a\x08+a\x01\x91V[\x91\x82\x91\x82a\x01\xBEV[\x03\x90\xF3[a\x01\x97V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08X\x90`\x08a\x08]\x93\x02a\x02\x08V[a\x08=V[\x90V[\x90a\x08k\x91Ta\x08HV[\x90V[a\x08z`\x04_\x90a\x08`V[\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08\x91\x90a\x08}V[\x90V[a\x08\x9D\x90a\x08\x88V[\x90RV[\x91\x90a\x08\xB4\x90_` \x85\x01\x94\x01\x90a\x08\x94V[V[4a\x08\xE6Wa\x08\xC66`\x04a\x01\x9FV[a\x08\xE2a\x08\xD1a\x08nV[a\x08\xD9a\x01\x91V[\x91\x82\x91\x82a\x08\xA1V[\x03\x90\xF3[a\x01\x97V[a\x08\xF4\x81a\x06\xFFV[\x03a\x08\xFBWV[_\x80\xFD[\x90P5\x90a\t\x0C\x82a\x08\xEBV[V[a\t\x17\x81a\x08\x88V[\x03a\t\x1EWV[_\x80\xFD[\x90P5\x90a\t/\x82a\t\x0EV[V[\x91``\x83\x83\x03\x12a\t}Wa\tH\x82_\x85\x01a\x08\xFFV[\x92a\tV\x83` \x83\x01a\t\"V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\txWa\tu\x92\x01a\x03\xEAV[\x90V[a\x03\x1EV[a\x01\x9BV[4a\t\xB1Wa\t\x9Ba\t\x956`\x04a\t1V[\x91a\x15fV[a\t\xA3a\x01\x91V[\x80a\t\xAD\x81a\x04\xC1V[\x03\x90\xF3[a\x01\x97V[\x90V[a\t\xCDa\t\xC8a\t\xD2\x92a\t\xB6V[a\x06\x8FV[a\x02\xBAV[\x90V[a\t\xE0a\x1C a\t\xB9V[\x90V[a\t\xEBa\t\xD5V[\x90V[4a\n\x1EWa\t\xFE6`\x04a\x01\x9FV[a\n\x1Aa\n\ta\t\xE3V[a\n\x11a\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[\x90V[a\n:a\n5a\n?\x92a\n#V[a\x06\x8FV[a\x06\xFFV[\x90V[a\nL`\x0Ba\n&V[\x90V[a\nWa\nBV[\x90V[4a\n\x8AWa\nj6`\x04a\x01\x9FV[a\n\x86a\nua\nOV[a\n}a\x01\x91V[\x91\x82\x91\x82a\x07FV[\x03\x90\xF3[a\x01\x97V[\x90V[a\n\xA6a\n\xA1a\n\xAB\x92a\n\x8FV[a\x06\x8FV[a\x02\xBAV[\x90V[a\n\xBAb\x01Q\x80a\n\x92V[\x90V[a\n\xC5a\n\xAEV[\x90V[4a\n\xF8Wa\n\xD86`\x04a\x01\x9FV[a\n\xF4a\n\xE3a\n\xBDV[a\n\xEBa\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[`\x05a\x0B\x08\x81a\x05dV[\x82\x10\x15a\x0B%Wa\x0B\"\x91a\x0B\x1C\x91a\x05qV[\x90a\x05\xB4V[\x90V[_\x80\xFD[4a\x0BYWa\x0BUa\x0BDa\x0B?6`\x04a\x052V[a\n\xFDV[a\x0BLa\x01\x91V[\x91\x82\x91\x82a\x05\xFEV[\x03\x90\xF3[a\x01\x97V[a\x0B\x93a\x0B\x9A\x94a\x0B\x89``\x94\x98\x97\x95a\x0B\x7F`\x80\x86\x01\x9A_\x87\x01\x90a\x02\xC7V[` \x85\x01\x90a\x01\xB1V[`@\x83\x01\x90a\x02\xC7V[\x01\x90a\x01\xB1V[V[4a\x0B\xD0Wa\x0B\xAC6`\x04a\x01\x9FV[a\x0B\xCCa\x0B\xB7a\x17^V[\x90a\x0B\xC3\x94\x92\x94a\x01\x91V[\x94\x85\x94\x85a\x0B^V[\x03\x90\xF3[a\x01\x97V[4a\x0C\x05Wa\x0C\x01a\x0B\xF0a\x0B\xEB6`\x04a\x052V[a\x17\xB7V[a\x0B\xF8a\x01\x91V[\x91\x82\x91\x82a\x05\xFEV[\x03\x90\xF3[a\x01\x97V[\x90\x91``\x82\x84\x03\x12a\x0C?Wa\x0C<a\x0C%\x84_\x85\x01a\t\"V[\x93a\x0C3\x81` \x86\x01a\t\"V[\x93`@\x01a\x04DV[\x90V[a\x01\x9BV[4a\x0CsWa\x0C]a\x0CW6`\x04a\x0C\nV[\x91a\x17\xF6V[a\x0Cea\x01\x91V[\x80a\x0Co\x81a\x04\xC1V[\x03\x90\xF3[a\x01\x97V[\x90V[a\x0C\x8Fa\x0C\x8Aa\x0C\x94\x92a\x0CxV[a\x06\x8FV[a\x02\xBAV[\x90V[a\x0C\xA3b\x01\xCC\xCCa\x0C{V[\x90V[a\x0C\xAEa\x0C\x97V[\x90V[4a\x0C\xE1Wa\x0C\xC16`\x04a\x01\x9FV[a\x0C\xDDa\x0C\xCCa\x0C\xA6V[a\x0C\xD4a\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[4a\r\x16Wa\x0C\xF66`\x04a\x01\x9FV[a\r\x12a\r\x01a\x186V[a\r\ta\x01\x91V[\x91\x82\x91\x82a\x01\xBEV[\x03\x90\xF3[a\x01\x97V[a\r'`\x02_\x90a\x02\x9EV[\x90V[4a\rZWa\r:6`\x04a\x01\x9FV[a\rVa\rEa\r\x1BV[a\rMa\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[_\x80\xFD[_\x90V[a\roa\rcV[Pa\rz`\x06a\x05dV[\x90V[_\x1B\x90V[\x90a\r\x95g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91a\r}V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\r\xB3a\r\xAEa\r\xB8\x92a\x02\xBAV[a\x06\x8FV[a\x02\xBAV[\x90V[\x90V[\x90a\r\xD3a\r\xCEa\r\xDA\x92a\r\x9FV[a\r\xBBV[\x82Ta\r\x82V[\x90UV[\x90a\r\xEA_\x19\x91a\r}V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x0E\x08a\x0E\x03a\x0E\r\x92a\x01\xAEV[a\x06\x8FV[a\x01\xAEV[\x90V[\x90V[\x90a\x0E(a\x0E#a\x0E/\x92a\r\xF4V[a\x0E\x10V[\x82Ta\r\xDEV[\x90UV[\x90V[a\x0EJa\x0EEa\x0EO\x92a\x0E3V[a\x06\x8FV[a\x02\xBAV[\x90V[a\x0Efa\x0Eaa\x0Ek\x92a\x0E3V[a\r}V[a\x05\xEEV[\x90V[a\x0E\x82a\x0E}a\x0E\x87\x92a\x0E3V[a\x06\x8FV[a\x01\xAEV[\x90V[\x90V[a\x0E\xA1a\x0E\x9Ca\x0E\xA6\x92a\x0E\x8AV[a\x06\x8FV[a\x01\xAEV[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x0E\xCCa\x0E\xD2\x91\x93\x92\x93a\x01\xAEV[\x92a\x01\xAEV[\x82\x03\x91\x82\x11a\x0E\xDDWV[a\x0E\xA9V[\x90V[a\x0E\xF1a\x0E\xF6\x91a\x05\xEEV[a\x0E\xE2V[\x90RV[` \x93\x92a\x0F\x19\x85\x83a\x0F\x11\x82\x95a\x0F!\x97a\x0E\xE5V[\x01\x80\x92a\x0E\xE5V[\x01\x80\x92a\x0E\xE5V[\x01\x90V[` \x01\x90V[Q\x90V[\x90V[_R` _ \x90V[T\x90V[a\x0FH\x81a\x0F;V[\x82\x10\x15a\x0FbWa\x0FZ`\x01\x91a\x0F2V[\x91\x02\x01\x90_\x90V[a\x05PV[\x1B\x90V[\x91\x90`\x08a\x0F\x86\x91\x02\x91a\x0F\x80_\x19\x84a\x0FgV[\x92a\x0FgV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x0F\x99\x90a\x05\xEEV[\x90V[_\x1C\x90V[a\x0F\xAA\x90a\x0F\x9CV[\x90V[\x91\x90a\x0F\xC3a\x0F\xBEa\x0F\xCB\x93a\x0F\x90V[a\x0F\xA1V[\x90\x83Ta\x0FkV[\x90UV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x0F\xFFW\x82a\x0F\xF7\x91`\x01a\x0F\xFD\x95\x01\x81Ua\x0F?V[\x90a\x0F\xADV[V[a\x034V[a\x10\x10a\x10\x15\x91a\x0F\x9CV[a\x02\x0CV[\x90V[a\x10\"\x90Ta\x10\x04V[\x90V[a\x10.\x90a\x02\xBAV[\x90RV[\x90``\x80a\x10x\x93a\x10J_\x82\x01Q_\x86\x01\x90a\x10%V[a\x10\\` \x82\x01Q` \x86\x01\x90a\x10%V[a\x10n`@\x82\x01Q`@\x86\x01\x90a\x10%V[\x01Q\x91\x01\x90a\x10%V[V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x05\x11\x15a\x10\x98WV[a\x10zV[\x90a\x10\xA7\x82a\x10\x8EV[V[a\x10\xB2\x90a\x10\x9DV[\x90V[a\x10\xBE\x90a\x10\xA9V[\x90RV[a\x10\xF7a\x10\xFE\x94a\x10\xED`\xC0\x94\x98\x97\x95a\x10\xE3`\xE0\x86\x01\x9A_\x87\x01\x90a\x05\xF1V[` \x85\x01\x90a\x01\xB1V[`@\x83\x01\x90a\x102V[\x01\x90a\x10\xB5V[V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[a\x113a\x11<` \x93a\x11A\x93a\x11*\x81a\x0F+V[\x93\x84\x80\x93a\x11\0V[\x95\x86\x91\x01a\x11\tV[a\x03*V[\x01\x90V[a\x11Z\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x11\x14V[\x90V[\x93\x91a\x11la\x11s\x92_a\r\xBEV[`\x01a\x0E\x13V[\x81a\x11\x86a\x11\x80_a\x0E6V[\x91a\x02\xBAV[\x11a\x13`W[PPa\x11\x98`\x05a\x05dV[a\x11\xA3\x82\x82\x90a\x1A\x8BV[\x90a\x11\xAE`\x06a\x05dV[\x92a\x11\xB8_a\x0ERV[\x84a\x11\xCBa\x11\xC5_a\x0EnV[\x91a\x01\xAEV[\x11a\x132W[a\x11\xDA_a\x0ERV[\x92\x82a\x11\xEEa\x11\xE8_a\x0EnV[\x91a\x01\xAEV[\x11a\x12\xFDW[a\x12R\x90a\x12#\x83a\x12\x14\x87a\x12\x08a\x01\x91V[\x94\x85\x93` \x85\x01a\x0E\xFAV[` \x82\x01\x81\x03\x82R\x03\x82a\x03HV[a\x125a\x12/\x82a\x0F+V[\x91a\x0F%V[ \x92a\x12Ka\x12D`\x06a\x0F/V[\x85\x90a\x0F\xCFV[`\x07a\x0E\x13V[\x84\x90\x91\x92\x93a\x12a`\x07a\x10\x18V[\x90\x94a\x12\xB6`\x01a\x12\xA4a\x12\x9Ea\x12\x98\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x97a\r\xF4V[\x97a\x0F\x90V[\x97a\x0F\x90V[\x97a\x12\xADa\x01\x91V[\x94\x85\x94\x85a\x10\xC2V[\x03\x90\xA4a\x12\xF8a\x12\xE6\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x92a\r\xF4V[\x92a\x12\xEFa\x01\x91V[\x91\x82\x91\x82a\x11EV[\x03\x90\xA2V[\x92Pa\x12Ra\x13*a\x13$`\x05a\x13\x1E\x86a\x13\x18`\x01a\x0E\x8DV[\x90a\x0E\xBDV[\x90a\x05qV[\x90a\x05\xB4V[\x93\x90Pa\x11\xF4V[Pa\x13[a\x13U`\x06a\x13O\x87a\x13I`\x01a\x0E\x8DV[\x90a\x0E\xBDV[\x90a\x05qV[\x90a\x05\xB4V[a\x11\xD1V[a\x13na\x13u\x92`\x02a\r\xBEV[`\x03a\x0E\x13V[_\x80a\x11\x8CV[a\x13\x84a\rcV[Pa\x13\x8F`\x06a\x05dV[\x90V[a\x13\xA6a\x13\xA1a\x13\xAB\x92a\x01\xAEV[a\x06\x8FV[a\x02\xBAV[\x90V[`\xF8\x1B\x90V[a\x13\xBD\x90a\x13\xAEV[\x90V[a\x13\xCCa\x13\xD1\x91a\x06\xFFV[a\x13\xB4V[\x90RV[``\x1B\x90V[a\x13\xE4\x90a\x13\xD5V[\x90V[a\x13\xF0\x90a\x13\xDBV[\x90V[a\x13\xFFa\x14\x04\x91a\x08\x88V[a\x13\xE7V[\x90RV[`\xC0\x1B\x90V[a\x14\x17\x90a\x14\x08V[\x90V[a\x14&a\x14+\x91a\x02\xBAV[a\x14\x0EV[\x90RV[\x90V[a\x14>a\x14C\x91a\x01\xAEV[a\x14/V[\x90RV[\x94a\x14\x98`\x08` \x99\x98\x95\x96a\x14\x90\x82\x8C\x99a\x14\x88`\x14a\x14\xA0\x9Aa\x14\x80a\x14\xA8\x9F\x8F\x9C\x90a\x14x\x81`\x01\x93a\x13\xC0V[\x01\x80\x92a\x13\xF3V[\x01\x80\x92a\x14\x1AV[\x01\x80\x92a\x14\x1AV[\x01\x80\x92a\x142V[\x01\x80\x92a\x142V[\x01\x80\x92a\x0E\xE5V[\x01\x90V[` \x81a\x14\xBEa\x14\xC6\x93\x83\x96\x95a\x0E\xE5V[\x01\x80\x92a\x0E\xE5V[\x01\x90V[a\x14\xDEa\x14\xD9a\x14\xE3\x92a\x08}V[a\x06\x8FV[a\x08}V[\x90V[a\x14\xEF\x90a\x14\xCAV[\x90V[a\x14\xFB\x90a\x14\xE6V[\x90V[a\x15\x07\x90a\x0EnV[\x90RV[\x91\x94a\x15Sa\x15]\x92\x98\x97\x95a\x15I`\xA0\x96a\x15?a\x15d\x9Aa\x155`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x08\x94V[` \x89\x01\x90a\x079V[`@\x87\x01\x90a\x08\x94V[``\x85\x01\x90a\x05\xF1V[`\x80\x83\x01\x90a\x14\xFEV[\x01\x90a\x02\xC7V[V[\x90a\x15q`\x05a\x05dV[\x91\x83a\x15\x85a\x15\x7F\x82a\x0F+V[\x91a\x0F%V[ \x91\x81a\x15\xD2\x82\x91a\x15\xC3a\x15\x99Ca\x13\x92V[a\x15\xA2Ba\x13\x92V[\x89a\x15\xAC_a\x0EnV[\x91\x8A\x93a\x15\xB7a\x01\x91V[\x98\x89\x97` \x89\x01a\x14GV[` \x82\x01\x81\x03\x82R\x03\x82a\x03HV[a\x15\xE4a\x15\xDE\x82a\x0F+V[\x91a\x0F%V[ \x90a\x15\xEF_a\x0ERV[\x91\x85a\x16\x03a\x15\xFD_a\x0EnV[\x91a\x01\xAEV[\x11a\x17\x04W[a\x16[\x90a\x16\x17`\x05a\x0F/V[\x90a\x16B\x85a\x163a\x16'a\x01\x91V[\x93\x84\x92` \x84\x01a\x14\xACV[` \x82\x01\x81\x03\x82R\x03\x82a\x03HV[a\x16Ta\x16N\x82a\x0F+V[\x91a\x0F%V[ \x90a\x0F\xCFV[\x84\x91\x92a\x16\xBDa\x16j0a\x14\xF2V[\x91\x92\x95_a\x16wBa\x13\x92V[\x91a\x16\xABa\x16\xA5\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE1\x98a\r\xF4V[\x98a\x0F\x90V[\x98a\x16\xB4a\x01\x91V[\x96\x87\x96\x87a\x15\x0BV[\x03\x90\xA3a\x16\xFFa\x16\xED\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x92a\r\xF4V[\x92a\x16\xF6a\x01\x91V[\x91\x82\x91\x82a\x11EV[\x03\x90\xA2V[\x91Pa\x16[a\x171a\x17+`\x05a\x17%\x89a\x17\x1F`\x01a\x0E\x8DV[\x90a\x0E\xBDV[\x90a\x05qV[\x90a\x05\xB4V[\x92\x90Pa\x16\tV[_\x90V[a\x17Ia\x17N\x91a\x0F\x9CV[a\x02yV[\x90V[a\x17[\x90Ta\x17=V[\x90V[a\x17fa\x179V[Pa\x17oa\rcV[Pa\x17xa\x179V[Pa\x17\x81a\rcV[Pa\x17\x8B_a\x17QV[\x90a\x17\x96`\x01a\x10\x18V[\x91a\x17\xA1`\x02a\x17QV[\x91a\x17\xAC`\x03a\x10\x18V[\x91\x93\x92\x91\x90V[_\x90V[a\x17\xCEa\x17\xD4\x91a\x17\xC6a\x17\xB3V[P`\x06a\x05qV[\x90a\x05\xB4V[\x90V[`\x14\x81a\x17\xEAa\x17\xF2\x93` \x96\x95a\x13\xF3V[\x01\x80\x92a\x142V[\x01\x90V[\x90a\x184\x92\x91a\x18/a\x18\x07a\x07!V[\x91\x92a\x18 a\x18\x14a\x01\x91V[\x95\x86\x92` \x84\x01a\x17\xD7V[` \x82\x01\x81\x03\x82R\x03\x84a\x03HV[a\x15fV[V[a\x18>a\rcV[Pa\x18I`\x05a\x05dV[\x90V[a\x18V`\x80a\x03qV[\x90V[_\x90V[a\x18ea\x18LV[\x90` \x80\x80\x80\x85a\x18ta\x18YV[\x81R\x01a\x18\x7Fa\x18YV[\x81R\x01a\x18\x8Aa\x18YV[\x81R\x01a\x18\x95a\x18YV[\x81RPPV[a\x18\xA3a\x18]V[\x90V[\x90V[a\x18\xBDa\x18\xB8a\x18\xC2\x92a\x18\xA6V[a\x06\x8FV[a\x01\xAEV[\x90V[a\x18\xCF`(a\x18\xA9V[\x90V[a\x18\xE1a\x18\xE7\x91\x93\x92\x93a\x01\xAEV[\x92a\x01\xAEV[\x82\x01\x80\x92\x11a\x18\xF2WV[a\x0E\xA9V[a\x19\x0Ba\x19\x06a\x19\x10\x92a\x02\xBAV[a\x06\x8FV[a\x01\xAEV[\x90V[a\x19\x1C\x90a\x18\xF7V[\x90RV[\x91` a\x19A\x92\x94\x93a\x19:`@\x82\x01\x96_\x83\x01\x90a\x01\xB1V[\x01\x90a\x19\x13V[V[a\x19Oa\x19U\x91a\x02\xBAV[\x91a\x02\xBAV[\x90\x03\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x19iWV[a\x0E\xA9V[\x90a\x19x\x90a\x02\xBAV[\x90RV[a\x19\x88a\x19\x8E\x91a\x02\xBAV[\x91a\x02\xBAV[\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x19\xA1WV[a\x0E\xA9V[a\x19\xB0\x90Qa\x02\xBAV[\x90V[`\x08a\x19\xF5\x94a\x19\xE5\x82\x80\x99\x98\x95\x96a\x19\xDD\x82\x87a\x19\xD5a\x19\xED\x99\x83\x9Ca\x14\x1AV[\x01\x80\x92a\x14\x1AV[\x01\x80\x92a\x14\x1AV[\x01\x80\x92a\x14\x1AV[\x01\x80\x92a\x14\x1AV[\x01\x90V[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[\x15a\x1A\x14WV[a\x19\xF9V[\x90P\x90V[a\x1ACa\x1A:\x92` \x92a\x1A1\x81a\x0F+V[\x94\x85\x80\x93a\x1A\x19V[\x93\x84\x91\x01a\x11\tV[\x01\x90V[a\x1AU\x90a\x1A[\x93\x92a\x1A\x1EV[\x90a\x1A\x1EV[\x90V[a\x1A}\x92\x91a\x1A\x89\x91a\x1Aoa\x01\x91V[\x94\x85\x92` \x84\x01\x92\x83a\x1AGV[\x90\x81\x03\x82R\x03\x83a\x03HV[V[\x91\x90\x91a\x1A\x96a\x17\xB3V[Pa\x1A\x9Fa\x18\x9BV[Pa\x1A\xBAa\x1A\xABa\x18\xC5V[a\x1A\xB4\x83a\x0F+V[\x90a\x18\xD2V[\x80a\x1A\xD4a\x1A\xCEa\x1A\xC9a\x0C\x97V[a\x18\xF7V[\x91a\x01\xAEV[\x11a\x1CsWPa\x1C\x07\x90a\x1A\xE6a\x18\x9BV[\x93Ba\x1B\x01a\x1A\xFBa\x1A\xF6a\n\xAEV[a\x18\xF7V[\x91a\x01\xAEV[\x11a\x1CIW[a\x1B-a\x1B$a\x1B\x16Ba\x13\x92V[a\x1B\x1Ea\x06\xAEV[\x90a\x19|V[` \x87\x01a\x19nV[Ca\x1BGa\x1BAa\x1B<a\t\xD5V[a\x18\xF7V[\x91a\x01\xAEV[\x11a\x1C\x1EW[a\x1Bsa\x1Bja\x1B\\Ca\x13\x92V[a\x1Bda\x07\xACV[\x90a\x19|V[``\x87\x01a\x19nV[a\x1B\xD7a\x1B\x81_\x87\x01a\x19\xA6V[a\x1B\xC8a\x1B\x90` \x89\x01a\x19\xA6V[\x93a\x1B\x9D`@\x8A\x01a\x19\xA6V[\x90a\x1B\xB3a\x1B\xAD``\x8C\x01a\x19\xA6V[\x91a\x13\x92V[\x91a\x1B\xBCa\x01\x91V[\x96\x87\x95` \x87\x01a\x19\xB3V[` \x82\x01\x81\x03\x82R\x03\x82a\x03HV[a\x1C\x02a\x1B\xE3\x82a\x0F+V[a\x1B\xFCa\x1B\xF6a\x1B\xF1a\x18\xC5V[a\x01\xAEV[\x91a\x01\xAEV[\x14a\x1A\rV[a\x1A^V[a\x1C\x19a\x1C\x13\x82a\x0F+V[\x91a\x0F%V[ \x91\x90V[a\x1CDa\x1C;a\x1C-Ca\x13\x92V[a\x1C5a\t\xD5V[\x90a\x19CV[`@\x87\x01a\x19nV[a\x1BMV[a\x1Cna\x1Cfa\x1CXBa\x13\x92V[a\x1C`a\n\xAEV[\x90a\x19CV[_\x87\x01a\x19nV[a\x1B\x07V[a\x1C{a\x0C\x97V[\x90a\x1C\x96_\x92\x83\x92cF4i\x1B`\xE0\x1B\x84R`\x04\x84\x01a\x19 V[\x03\x90\xFD",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b610d5f565b61001d5f3561018b565b806284120c1461018657806304f1c85414610181578063056daaa61461017c578063061d12c01461017757806306f130561461017257806316bf55791461016d57806318db3940146101685780632f1ec5e9146101635780633c53a3831461015e5780634f359a37146101595780637fa3a40e146101545780638da5cb5b1461014f578063a7b51d191461014a578063ad9c0c2e14610145578063ae1a7d3014610140578063b752a7d11461013b578063d5719dc214610136578063d5954c3414610131578063d9dd67ab1461012c578063e1d66afe14610127578063e8eb1dc314610122578063eca067ad1461011d5763fbf6eaa50361000e57610d2a565b610ce6565b610cb1565b610c44565b610bd5565b610b9c565b610b29565b610ac8565b610a5a565b6109ee565b610982565b6108b6565b610808565b6107c4565b61075b565b6106c7565b610657565b610613565b6104fd565b6104c6565b6102e9565b610244565b6101d3565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f9103126101a957565b61019b565b90565b6101ba906101ae565b9052565b91906101d1905f602085019401906101b1565b565b34610203576101e336600461019f565b6101ff6101ee610d67565b6101f6610191565b918291826101be565b0390f35b610197565b1c90565b90565b61021f9060086102249302610208565b61020c565b90565b90610232915461020f565b90565b61024160035f90610227565b90565b346102745761025436600461019f565b61027061025f610235565b610267610191565b918291826101be565b0390f35b610197565b67ffffffffffffffff1690565b61029690600861029b9302610208565b610279565b90565b906102a99154610286565b90565b6102b75f5f9061029e565b90565b67ffffffffffffffff1690565b6102d0906102ba565b9052565b91906102e7905f602085019401906102c7565b565b34610319576102f936600461019f565b6103156103046102ac565b61030c610191565b918291826102d4565b0390f35b610197565b5f80fd5b5f80fd5b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906103529061032a565b810190811067ffffffffffffffff82111761036c57604052565b610334565b9061038461037d610191565b9283610348565b565b67ffffffffffffffff81116103a4576103a060209161032a565b0190565b610334565b90825f939282370152565b909291926103c96103c482610386565b610371565b938185526020850190828401116103e5576103e3926103a9565b565b610326565b9080601f8301121561040857816020610405933591016103b4565b90565b610322565b610416816102ba565b0361041d57565b5f80fd5b9050359061042e8261040d565b565b610439816101ae565b0361044057565b5f80fd5b9050359061045182610430565b565b919060a0838203126104bc575f83013567ffffffffffffffff81116104b7578161047e9185016103ea565b9261048c8260208301610421565b926104b461049d8460408501610444565b936104ab8160608601610421565b93608001610444565b90565b61031e565b61019b565b5f0190565b346104f8576104e26104d9366004610453565b9392909261115d565b6104ea610191565b806104f4816104c1565b0390f35b610197565b3461052d5761050d36600461019f565b61052961051861137c565b610520610191565b918291826101be565b0390f35b610197565b9060208282031261054b57610548915f01610444565b90565b61019b565b634e487b7160e01b5f52603260045260245ffd5b5490565b5f5260205f2090565b61057a81610564565b8210156105945761058c600191610568565b910201905f90565b610550565b90565b6105ac9060086105b19302610208565b610599565b90565b906105bf915461059c565b90565b60066105cd81610564565b8210156105ea576105e7916105e191610571565b906105b4565b90565b5f80fd5b90565b6105fa906105ee565b9052565b9190610611905f602085019401906105f1565b565b346106435761063f61062e610629366004610532565b6105c2565b610636610191565b918291826105fe565b0390f35b610197565b61065460015f90610227565b90565b346106875761066736600461019f565b610683610672610648565b61067a610191565b918291826101be565b0390f35b610197565b90565b90565b6106a66106a16106ab9261068c565b61068f565b6102ba565b90565b6106b9610e10610692565b90565b6106c46106ae565b90565b346106f7576106d736600461019f565b6106f36106e26106bc565b6106ea610191565b918291826102d4565b0390f35b610197565b90565b60ff1690565b61071961071461071e926106fc565b61068f565b6106ff565b90565b61072b600c610705565b90565b610736610721565b90565b610742906106ff565b9052565b9190610759905f60208501940190610739565b565b3461078b5761076b36600461019f565b61078761077661072e565b61077e610191565b91829182610746565b0390f35b610197565b6107a461079f6107a9926106fc565b61068f565b6102ba565b90565b6107b6600c610790565b90565b6107c16107ac565b90565b346107f4576107d436600461019f565b6107f06107df6107b9565b6107e7610191565b918291826102d4565b0390f35b610197565b61080560075f90610227565b90565b346108385761081836600461019f565b6108346108236107f9565b61082b610191565b918291826101be565b0390f35b610197565b60018060a01b031690565b61085890600861085d9302610208565b61083d565b90565b9061086b9154610848565b90565b61087a60045f90610860565b90565b60018060a01b031690565b6108919061087d565b90565b61089d90610888565b9052565b91906108b4905f60208501940190610894565b565b346108e6576108c636600461019f565b6108e26108d161086e565b6108d9610191565b918291826108a1565b0390f35b610197565b6108f4816106ff565b036108fb57565b5f80fd5b9050359061090c826108eb565b565b61091781610888565b0361091e57565b5f80fd5b9050359061092f8261090e565b565b9160608383031261097d57610948825f85016108ff565b926109568360208301610922565b92604082013567ffffffffffffffff81116109785761097592016103ea565b90565b61031e565b61019b565b346109b15761099b610995366004610931565b91611566565b6109a3610191565b806109ad816104c1565b0390f35b610197565b90565b6109cd6109c86109d2926109b6565b61068f565b6102ba565b90565b6109e0611c206109b9565b90565b6109eb6109d5565b90565b34610a1e576109fe36600461019f565b610a1a610a096109e3565b610a11610191565b918291826102d4565b0390f35b610197565b90565b610a3a610a35610a3f92610a23565b61068f565b6106ff565b90565b610a4c600b610a26565b90565b610a57610a42565b90565b34610a8a57610a6a36600461019f565b610a86610a75610a4f565b610a7d610191565b91829182610746565b0390f35b610197565b90565b610aa6610aa1610aab92610a8f565b61068f565b6102ba565b90565b610aba62015180610a92565b90565b610ac5610aae565b90565b34610af857610ad836600461019f565b610af4610ae3610abd565b610aeb610191565b918291826102d4565b0390f35b610197565b6005610b0881610564565b821015610b2557610b2291610b1c91610571565b906105b4565b90565b5f80fd5b34610b5957610b55610b44610b3f366004610532565b610afd565b610b4c610191565b918291826105fe565b0390f35b610197565b610b93610b9a94610b89606094989795610b7f608086019a5f8701906102c7565b60208501906101b1565b60408301906102c7565b01906101b1565b565b34610bd057610bac36600461019f565b610bcc610bb761175e565b90610bc3949294610191565b94859485610b5e565b0390f35b610197565b34610c0557610c01610bf0610beb366004610532565b6117b7565b610bf8610191565b918291826105fe565b0390f35b610197565b9091606082840312610c3f57610c3c610c25845f8501610922565b93610c338160208601610922565b93604001610444565b90565b61019b565b34610c7357610c5d610c57366004610c0a565b916117f6565b610c65610191565b80610c6f816104c1565b0390f35b610197565b90565b610c8f610c8a610c9492610c78565b61068f565b6102ba565b90565b610ca36201cccc610c7b565b90565b610cae610c97565b90565b34610ce157610cc136600461019f565b610cdd610ccc610ca6565b610cd4610191565b918291826102d4565b0390f35b610197565b34610d1657610cf636600461019f565b610d12610d01611836565b610d09610191565b918291826101be565b0390f35b610197565b610d2760025f9061029e565b90565b34610d5a57610d3a36600461019f565b610d56610d45610d1b565b610d4d610191565b918291826102d4565b0390f35b610197565b5f80fd5b5f90565b610d6f610d63565b50610d7a6006610564565b90565b5f1b90565b90610d9567ffffffffffffffff91610d7d565b9181191691161790565b610db3610dae610db8926102ba565b61068f565b6102ba565b90565b90565b90610dd3610dce610dda92610d9f565b610dbb565b8254610d82565b9055565b90610dea5f1991610d7d565b9181191691161790565b610e08610e03610e0d926101ae565b61068f565b6101ae565b90565b90565b90610e28610e23610e2f92610df4565b610e10565b8254610dde565b9055565b90565b610e4a610e45610e4f92610e33565b61068f565b6102ba565b90565b610e66610e61610e6b92610e33565b610d7d565b6105ee565b90565b610e82610e7d610e8792610e33565b61068f565b6101ae565b90565b90565b610ea1610e9c610ea692610e8a565b61068f565b6101ae565b90565b634e487b7160e01b5f52601160045260245ffd5b610ecc610ed2919392936101ae565b926101ae565b8203918211610edd57565b610ea9565b90565b610ef1610ef6916105ee565b610ee2565b9052565b60209392610f198583610f118295610f2197610ee5565b018092610ee5565b018092610ee5565b0190565b60200190565b5190565b90565b5f5260205f2090565b5490565b610f4881610f3b565b821015610f6257610f5a600191610f32565b910201905f90565b610550565b1b90565b91906008610f86910291610f805f1984610f67565b92610f67565b9181191691161790565b610f99906105ee565b90565b5f1c90565b610faa90610f9c565b90565b9190610fc3610fbe610fcb93610f90565b610fa1565b908354610f6b565b9055565b9081549168010000000000000000831015610fff5782610ff7916001610ffd95018155610f3f565b90610fad565b565b610334565b61101061101591610f9c565b61020c565b90565b6110229054611004565b90565b61102e906102ba565b9052565b906060806110789361104a5f8201515f860190611025565b61105c60208201516020860190611025565b61106e60408201516040860190611025565b0151910190611025565b565b634e487b7160e01b5f52602160045260245ffd5b6005111561109857565b61107a565b906110a78261108e565b565b6110b29061109d565b90565b6110be906110a9565b9052565b6110f76110fe946110ed60c0949897956110e360e086019a5f8701906105f1565b60208501906101b1565b6040830190611032565b01906110b5565b565b60209181520190565b90825f9392825e0152565b61113361113c6020936111419361112a81610f2b565b93848093611100565b95869101611109565b61032a565b0190565b61115a9160208201915f818403910152611114565b90565b939161116c611173925f610dbe565b6001610e13565b816111866111805f610e36565b916102ba565b11611360575b50506111986005610564565b6111a3828290611a8b565b906111ae6006610564565b926111b85f610e52565b846111cb6111c55f610e6e565b916101ae565b11611332575b6111da5f610e52565b92826111ee6111e85f610e6e565b916101ae565b116112fd575b611252906112238361121487611208610191565b94859360208501610efa565b60208201810382520382610348565b61123561122f82610f2b565b91610f25565b209261124b6112446006610f2f565b8590610fcf565b6007610e13565b84909192936112616007611018565b90946112b660016112a461129e6112987f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd797610df4565b97610f90565b97610f90565b976112ad610191565b948594856110c2565b0390a46112f86112e67ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c2092610df4565b926112ef610191565b91829182611145565b0390a2565b925061125261132a611324600561131e866113186001610e8d565b90610ebd565b90610571565b906105b4565b9390506111f4565b5061135b611355600661134f876113496001610e8d565b90610ebd565b90610571565b906105b4565b6111d1565b61136e611375926002610dbe565b6003610e13565b5f8061118c565b611384610d63565b5061138f6006610564565b90565b6113a66113a16113ab926101ae565b61068f565b6102ba565b90565b60f81b90565b6113bd906113ae565b90565b6113cc6113d1916106ff565b6113b4565b9052565b60601b90565b6113e4906113d5565b90565b6113f0906113db565b90565b6113ff61140491610888565b6113e7565b9052565b60c01b90565b61141790611408565b90565b61142661142b916102ba565b61140e565b9052565b90565b61143e611443916101ae565b61142f565b9052565b946114986008602099989596611490828c9961148860146114a09a6114806114a89f8f9c90611478816001936113c0565b0180926113f3565b01809261141a565b01809261141a565b018092611432565b018092611432565b018092610ee5565b0190565b6020816114be6114c693839695610ee5565b018092610ee5565b0190565b6114de6114d96114e39261087d565b61068f565b61087d565b90565b6114ef906114ca565b90565b6114fb906114e6565b90565b61150790610e6e565b9052565b919461155361155d9298979561154960a09661153f6115649a61153560c08a019e5f8b0190610894565b6020890190610739565b6040870190610894565b60608501906105f1565b60808301906114fe565b01906102c7565b565b906115716005610564565b918361158561157f82610f2b565b91610f25565b2091816115d282916115c361159943611392565b6115a242611392565b896115ac5f610e6e565b918a936115b7610191565b98899760208901611447565b60208201810382520382610348565b6115e46115de82610f2b565b91610f25565b20906115ef5f610e52565b91856116036115fd5f610e6e565b916101ae565b11611704575b61165b906116176005610f2f565b9061164285611633611627610191565b938492602084016114ac565b60208201810382520382610348565b61165461164e82610f2b565b91610f25565b2090610fcf565b8491926116bd61166a306114f2565b9192955f61167742611392565b916116ab6116a57f5e3c1311ea442664e8b1611bfabef659120ea7a0a2cfc0667700bebc69cbffe198610df4565b98610f90565b986116b4610191565b9687968761150b565b0390a36116ff6116ed7fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b92610df4565b926116f6610191565b91829182611145565b0390a2565b915061165b61173161172b60056117258961171f6001610e8d565b90610ebd565b90610571565b906105b4565b929050611609565b5f90565b61174961174e91610f9c565b610279565b90565b61175b905461173d565b90565b611766611739565b5061176f610d63565b50611778611739565b50611781610d63565b5061178b5f611751565b906117966001611018565b916117a16002611751565b916117ac6003611018565b9193929190565b5f90565b6117ce6117d4916117c66117b3565b506006610571565b906105b4565b90565b6014816117ea6117f293602096956113f3565b018092611432565b0190565b90611834929161182f611807610721565b9192611820611814610191565b958692602084016117d7565b60208201810382520384610348565b611566565b565b61183e610d63565b506118496005610564565b90565b6118566080610371565b90565b5f90565b61186561184c565b90602080808085611874611859565b81520161187f611859565b81520161188a611859565b815201611895611859565b81525050565b6118a361185d565b90565b90565b6118bd6118b86118c2926118a6565b61068f565b6101ae565b90565b6118cf60286118a9565b90565b6118e16118e7919392936101ae565b926101ae565b82018092116118f257565b610ea9565b61190b611906611910926102ba565b61068f565b6101ae565b90565b61191c906118f7565b9052565b91602061194192949361193a60408201965f8301906101b1565b0190611913565b565b61194f611955916102ba565b916102ba565b90039067ffffffffffffffff821161196957565b610ea9565b90611978906102ba565b9052565b61198861198e916102ba565b916102ba565b019067ffffffffffffffff82116119a157565b610ea9565b6119b090516102ba565b90565b60086119f5946119e58280999895966119dd82876119d56119ed99839c61141a565b01809261141a565b01809261141a565b01809261141a565b01809261141a565b0190565b634e487b7160e01b5f52600160045260245ffd5b15611a1457565b6119f9565b905090565b611a43611a3a92602092611a3181610f2b565b94858093611a19565b93849101611109565b0190565b611a5590611a5b9392611a1e565b90611a1e565b90565b611a7d9291611a8991611a6f610191565b948592602084019283611a47565b90810382520383610348565b565b919091611a966117b3565b50611a9f61189b565b50611aba611aab6118c5565b611ab483610f2b565b906118d2565b80611ad4611ace611ac9610c97565b6118f7565b916101ae565b11611c735750611c0790611ae661189b565b9342611b01611afb611af6610aae565b6118f7565b916101ae565b11611c49575b611b2d611b24611b1642611392565b611b1e6106ae565b9061197c565b6020870161196e565b43611b47611b41611b3c6109d5565b6118f7565b916101ae565b11611c1e575b611b73611b6a611b5c43611392565b611b646107ac565b9061197c565b6060870161196e565b611bd7611b815f87016119a6565b611bc8611b90602089016119a6565b93611b9d60408a016119a6565b90611bb3611bad60608c016119a6565b91611392565b91611bbc610191565b968795602087016119b3565b60208201810382520382610348565b611c02611be382610f2b565b611bfc611bf6611bf16118c5565b6101ae565b916101ae565b14611a0d565b611a5e565b611c19611c1382610f2b565b91610f25565b209190565b611c44611c3b611c2d43611392565b611c356109d5565b90611943565b6040870161196e565b611b4d565b611c6e611c66611c5842611392565b611c60610aae565b90611943565b5f870161196e565b611b07565b611c7b610c97565b90611c965f928392634634691b60e01b845260048401611920565b0390fd
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\r_V[a\0\x1D_5a\x01\x8BV[\x80b\x84\x12\x0C\x14a\x01\x86W\x80c\x04\xF1\xC8T\x14a\x01\x81W\x80c\x05m\xAA\xA6\x14a\x01|W\x80c\x06\x1D\x12\xC0\x14a\x01wW\x80c\x06\xF10V\x14a\x01rW\x80c\x16\xBFUy\x14a\x01mW\x80c\x18\xDB9@\x14a\x01hW\x80c/\x1E\xC5\xE9\x14a\x01cW\x80c<S\xA3\x83\x14a\x01^W\x80cO5\x9A7\x14a\x01YW\x80c\x7F\xA3\xA4\x0E\x14a\x01TW\x80c\x8D\xA5\xCB[\x14a\x01OW\x80c\xA7\xB5\x1D\x19\x14a\x01JW\x80c\xAD\x9C\x0C.\x14a\x01EW\x80c\xAE\x1A}0\x14a\x01@W\x80c\xB7R\xA7\xD1\x14a\x01;W\x80c\xD5q\x9D\xC2\x14a\x016W\x80c\xD5\x95L4\x14a\x011W\x80c\xD9\xDDg\xAB\x14a\x01,W\x80c\xE1\xD6j\xFE\x14a\x01'W\x80c\xE8\xEB\x1D\xC3\x14a\x01\"W\x80c\xEC\xA0g\xAD\x14a\x01\x1DWc\xFB\xF6\xEA\xA5\x03a\0\x0EWa\r*V[a\x0C\xE6V[a\x0C\xB1V[a\x0CDV[a\x0B\xD5V[a\x0B\x9CV[a\x0B)V[a\n\xC8V[a\nZV[a\t\xEEV[a\t\x82V[a\x08\xB6V[a\x08\x08V[a\x07\xC4V[a\x07[V[a\x06\xC7V[a\x06WV[a\x06\x13V[a\x04\xFDV[a\x04\xC6V[a\x02\xE9V[a\x02DV[a\x01\xD3V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01\xA9WV[a\x01\x9BV[\x90V[a\x01\xBA\x90a\x01\xAEV[\x90RV[\x91\x90a\x01\xD1\x90_` \x85\x01\x94\x01\x90a\x01\xB1V[V[4a\x02\x03Wa\x01\xE36`\x04a\x01\x9FV[a\x01\xFFa\x01\xEEa\rgV[a\x01\xF6a\x01\x91V[\x91\x82\x91\x82a\x01\xBEV[\x03\x90\xF3[a\x01\x97V[\x1C\x90V[\x90V[a\x02\x1F\x90`\x08a\x02$\x93\x02a\x02\x08V[a\x02\x0CV[\x90V[\x90a\x022\x91Ta\x02\x0FV[\x90V[a\x02A`\x03_\x90a\x02'V[\x90V[4a\x02tWa\x02T6`\x04a\x01\x9FV[a\x02pa\x02_a\x025V[a\x02ga\x01\x91V[\x91\x82\x91\x82a\x01\xBEV[\x03\x90\xF3[a\x01\x97V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\x96\x90`\x08a\x02\x9B\x93\x02a\x02\x08V[a\x02yV[\x90V[\x90a\x02\xA9\x91Ta\x02\x86V[\x90V[a\x02\xB7__\x90a\x02\x9EV[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\xD0\x90a\x02\xBAV[\x90RV[\x91\x90a\x02\xE7\x90_` \x85\x01\x94\x01\x90a\x02\xC7V[V[4a\x03\x19Wa\x02\xF96`\x04a\x01\x9FV[a\x03\x15a\x03\x04a\x02\xACV[a\x03\x0Ca\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[_\x80\xFD[_\x80\xFD[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x03R\x90a\x03*V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03lW`@RV[a\x034V[\x90a\x03\x84a\x03}a\x01\x91V[\x92\x83a\x03HV[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\xA4Wa\x03\xA0` \x91a\x03*V[\x01\x90V[a\x034V[\x90\x82_\x93\x92\x827\x01RV[\x90\x92\x91\x92a\x03\xC9a\x03\xC4\x82a\x03\x86V[a\x03qV[\x93\x81\x85R` \x85\x01\x90\x82\x84\x01\x11a\x03\xE5Wa\x03\xE3\x92a\x03\xA9V[V[a\x03&V[\x90\x80`\x1F\x83\x01\x12\x15a\x04\x08W\x81` a\x04\x05\x935\x91\x01a\x03\xB4V[\x90V[a\x03\"V[a\x04\x16\x81a\x02\xBAV[\x03a\x04\x1DWV[_\x80\xFD[\x90P5\x90a\x04.\x82a\x04\rV[V[a\x049\x81a\x01\xAEV[\x03a\x04@WV[_\x80\xFD[\x90P5\x90a\x04Q\x82a\x040V[V[\x91\x90`\xA0\x83\x82\x03\x12a\x04\xBCW_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\xB7W\x81a\x04~\x91\x85\x01a\x03\xEAV[\x92a\x04\x8C\x82` \x83\x01a\x04!V[\x92a\x04\xB4a\x04\x9D\x84`@\x85\x01a\x04DV[\x93a\x04\xAB\x81``\x86\x01a\x04!V[\x93`\x80\x01a\x04DV[\x90V[a\x03\x1EV[a\x01\x9BV[_\x01\x90V[4a\x04\xF8Wa\x04\xE2a\x04\xD96`\x04a\x04SV[\x93\x92\x90\x92a\x11]V[a\x04\xEAa\x01\x91V[\x80a\x04\xF4\x81a\x04\xC1V[\x03\x90\xF3[a\x01\x97V[4a\x05-Wa\x05\r6`\x04a\x01\x9FV[a\x05)a\x05\x18a\x13|V[a\x05 a\x01\x91V[\x91\x82\x91\x82a\x01\xBEV[\x03\x90\xF3[a\x01\x97V[\x90` \x82\x82\x03\x12a\x05KWa\x05H\x91_\x01a\x04DV[\x90V[a\x01\x9BV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[T\x90V[_R` _ \x90V[a\x05z\x81a\x05dV[\x82\x10\x15a\x05\x94Wa\x05\x8C`\x01\x91a\x05hV[\x91\x02\x01\x90_\x90V[a\x05PV[\x90V[a\x05\xAC\x90`\x08a\x05\xB1\x93\x02a\x02\x08V[a\x05\x99V[\x90V[\x90a\x05\xBF\x91Ta\x05\x9CV[\x90V[`\x06a\x05\xCD\x81a\x05dV[\x82\x10\x15a\x05\xEAWa\x05\xE7\x91a\x05\xE1\x91a\x05qV[\x90a\x05\xB4V[\x90V[_\x80\xFD[\x90V[a\x05\xFA\x90a\x05\xEEV[\x90RV[\x91\x90a\x06\x11\x90_` \x85\x01\x94\x01\x90a\x05\xF1V[V[4a\x06CWa\x06?a\x06.a\x06)6`\x04a\x052V[a\x05\xC2V[a\x066a\x01\x91V[\x91\x82\x91\x82a\x05\xFEV[\x03\x90\xF3[a\x01\x97V[a\x06T`\x01_\x90a\x02'V[\x90V[4a\x06\x87Wa\x06g6`\x04a\x01\x9FV[a\x06\x83a\x06ra\x06HV[a\x06za\x01\x91V[\x91\x82\x91\x82a\x01\xBEV[\x03\x90\xF3[a\x01\x97V[\x90V[\x90V[a\x06\xA6a\x06\xA1a\x06\xAB\x92a\x06\x8CV[a\x06\x8FV[a\x02\xBAV[\x90V[a\x06\xB9a\x0E\x10a\x06\x92V[\x90V[a\x06\xC4a\x06\xAEV[\x90V[4a\x06\xF7Wa\x06\xD76`\x04a\x01\x9FV[a\x06\xF3a\x06\xE2a\x06\xBCV[a\x06\xEAa\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[\x90V[`\xFF\x16\x90V[a\x07\x19a\x07\x14a\x07\x1E\x92a\x06\xFCV[a\x06\x8FV[a\x06\xFFV[\x90V[a\x07+`\x0Ca\x07\x05V[\x90V[a\x076a\x07!V[\x90V[a\x07B\x90a\x06\xFFV[\x90RV[\x91\x90a\x07Y\x90_` \x85\x01\x94\x01\x90a\x079V[V[4a\x07\x8BWa\x07k6`\x04a\x01\x9FV[a\x07\x87a\x07va\x07.V[a\x07~a\x01\x91V[\x91\x82\x91\x82a\x07FV[\x03\x90\xF3[a\x01\x97V[a\x07\xA4a\x07\x9Fa\x07\xA9\x92a\x06\xFCV[a\x06\x8FV[a\x02\xBAV[\x90V[a\x07\xB6`\x0Ca\x07\x90V[\x90V[a\x07\xC1a\x07\xACV[\x90V[4a\x07\xF4Wa\x07\xD46`\x04a\x01\x9FV[a\x07\xF0a\x07\xDFa\x07\xB9V[a\x07\xE7a\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[a\x08\x05`\x07_\x90a\x02'V[\x90V[4a\x088Wa\x08\x186`\x04a\x01\x9FV[a\x084a\x08#a\x07\xF9V[a\x08+a\x01\x91V[\x91\x82\x91\x82a\x01\xBEV[\x03\x90\xF3[a\x01\x97V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08X\x90`\x08a\x08]\x93\x02a\x02\x08V[a\x08=V[\x90V[\x90a\x08k\x91Ta\x08HV[\x90V[a\x08z`\x04_\x90a\x08`V[\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x08\x91\x90a\x08}V[\x90V[a\x08\x9D\x90a\x08\x88V[\x90RV[\x91\x90a\x08\xB4\x90_` \x85\x01\x94\x01\x90a\x08\x94V[V[4a\x08\xE6Wa\x08\xC66`\x04a\x01\x9FV[a\x08\xE2a\x08\xD1a\x08nV[a\x08\xD9a\x01\x91V[\x91\x82\x91\x82a\x08\xA1V[\x03\x90\xF3[a\x01\x97V[a\x08\xF4\x81a\x06\xFFV[\x03a\x08\xFBWV[_\x80\xFD[\x90P5\x90a\t\x0C\x82a\x08\xEBV[V[a\t\x17\x81a\x08\x88V[\x03a\t\x1EWV[_\x80\xFD[\x90P5\x90a\t/\x82a\t\x0EV[V[\x91``\x83\x83\x03\x12a\t}Wa\tH\x82_\x85\x01a\x08\xFFV[\x92a\tV\x83` \x83\x01a\t\"V[\x92`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\txWa\tu\x92\x01a\x03\xEAV[\x90V[a\x03\x1EV[a\x01\x9BV[4a\t\xB1Wa\t\x9Ba\t\x956`\x04a\t1V[\x91a\x15fV[a\t\xA3a\x01\x91V[\x80a\t\xAD\x81a\x04\xC1V[\x03\x90\xF3[a\x01\x97V[\x90V[a\t\xCDa\t\xC8a\t\xD2\x92a\t\xB6V[a\x06\x8FV[a\x02\xBAV[\x90V[a\t\xE0a\x1C a\t\xB9V[\x90V[a\t\xEBa\t\xD5V[\x90V[4a\n\x1EWa\t\xFE6`\x04a\x01\x9FV[a\n\x1Aa\n\ta\t\xE3V[a\n\x11a\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[\x90V[a\n:a\n5a\n?\x92a\n#V[a\x06\x8FV[a\x06\xFFV[\x90V[a\nL`\x0Ba\n&V[\x90V[a\nWa\nBV[\x90V[4a\n\x8AWa\nj6`\x04a\x01\x9FV[a\n\x86a\nua\nOV[a\n}a\x01\x91V[\x91\x82\x91\x82a\x07FV[\x03\x90\xF3[a\x01\x97V[\x90V[a\n\xA6a\n\xA1a\n\xAB\x92a\n\x8FV[a\x06\x8FV[a\x02\xBAV[\x90V[a\n\xBAb\x01Q\x80a\n\x92V[\x90V[a\n\xC5a\n\xAEV[\x90V[4a\n\xF8Wa\n\xD86`\x04a\x01\x9FV[a\n\xF4a\n\xE3a\n\xBDV[a\n\xEBa\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[`\x05a\x0B\x08\x81a\x05dV[\x82\x10\x15a\x0B%Wa\x0B\"\x91a\x0B\x1C\x91a\x05qV[\x90a\x05\xB4V[\x90V[_\x80\xFD[4a\x0BYWa\x0BUa\x0BDa\x0B?6`\x04a\x052V[a\n\xFDV[a\x0BLa\x01\x91V[\x91\x82\x91\x82a\x05\xFEV[\x03\x90\xF3[a\x01\x97V[a\x0B\x93a\x0B\x9A\x94a\x0B\x89``\x94\x98\x97\x95a\x0B\x7F`\x80\x86\x01\x9A_\x87\x01\x90a\x02\xC7V[` \x85\x01\x90a\x01\xB1V[`@\x83\x01\x90a\x02\xC7V[\x01\x90a\x01\xB1V[V[4a\x0B\xD0Wa\x0B\xAC6`\x04a\x01\x9FV[a\x0B\xCCa\x0B\xB7a\x17^V[\x90a\x0B\xC3\x94\x92\x94a\x01\x91V[\x94\x85\x94\x85a\x0B^V[\x03\x90\xF3[a\x01\x97V[4a\x0C\x05Wa\x0C\x01a\x0B\xF0a\x0B\xEB6`\x04a\x052V[a\x17\xB7V[a\x0B\xF8a\x01\x91V[\x91\x82\x91\x82a\x05\xFEV[\x03\x90\xF3[a\x01\x97V[\x90\x91``\x82\x84\x03\x12a\x0C?Wa\x0C<a\x0C%\x84_\x85\x01a\t\"V[\x93a\x0C3\x81` \x86\x01a\t\"V[\x93`@\x01a\x04DV[\x90V[a\x01\x9BV[4a\x0CsWa\x0C]a\x0CW6`\x04a\x0C\nV[\x91a\x17\xF6V[a\x0Cea\x01\x91V[\x80a\x0Co\x81a\x04\xC1V[\x03\x90\xF3[a\x01\x97V[\x90V[a\x0C\x8Fa\x0C\x8Aa\x0C\x94\x92a\x0CxV[a\x06\x8FV[a\x02\xBAV[\x90V[a\x0C\xA3b\x01\xCC\xCCa\x0C{V[\x90V[a\x0C\xAEa\x0C\x97V[\x90V[4a\x0C\xE1Wa\x0C\xC16`\x04a\x01\x9FV[a\x0C\xDDa\x0C\xCCa\x0C\xA6V[a\x0C\xD4a\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[4a\r\x16Wa\x0C\xF66`\x04a\x01\x9FV[a\r\x12a\r\x01a\x186V[a\r\ta\x01\x91V[\x91\x82\x91\x82a\x01\xBEV[\x03\x90\xF3[a\x01\x97V[a\r'`\x02_\x90a\x02\x9EV[\x90V[4a\rZWa\r:6`\x04a\x01\x9FV[a\rVa\rEa\r\x1BV[a\rMa\x01\x91V[\x91\x82\x91\x82a\x02\xD4V[\x03\x90\xF3[a\x01\x97V[_\x80\xFD[_\x90V[a\roa\rcV[Pa\rz`\x06a\x05dV[\x90V[_\x1B\x90V[\x90a\r\x95g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91a\r}V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\r\xB3a\r\xAEa\r\xB8\x92a\x02\xBAV[a\x06\x8FV[a\x02\xBAV[\x90V[\x90V[\x90a\r\xD3a\r\xCEa\r\xDA\x92a\r\x9FV[a\r\xBBV[\x82Ta\r\x82V[\x90UV[\x90a\r\xEA_\x19\x91a\r}V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x0E\x08a\x0E\x03a\x0E\r\x92a\x01\xAEV[a\x06\x8FV[a\x01\xAEV[\x90V[\x90V[\x90a\x0E(a\x0E#a\x0E/\x92a\r\xF4V[a\x0E\x10V[\x82Ta\r\xDEV[\x90UV[\x90V[a\x0EJa\x0EEa\x0EO\x92a\x0E3V[a\x06\x8FV[a\x02\xBAV[\x90V[a\x0Efa\x0Eaa\x0Ek\x92a\x0E3V[a\r}V[a\x05\xEEV[\x90V[a\x0E\x82a\x0E}a\x0E\x87\x92a\x0E3V[a\x06\x8FV[a\x01\xAEV[\x90V[\x90V[a\x0E\xA1a\x0E\x9Ca\x0E\xA6\x92a\x0E\x8AV[a\x06\x8FV[a\x01\xAEV[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x0E\xCCa\x0E\xD2\x91\x93\x92\x93a\x01\xAEV[\x92a\x01\xAEV[\x82\x03\x91\x82\x11a\x0E\xDDWV[a\x0E\xA9V[\x90V[a\x0E\xF1a\x0E\xF6\x91a\x05\xEEV[a\x0E\xE2V[\x90RV[` \x93\x92a\x0F\x19\x85\x83a\x0F\x11\x82\x95a\x0F!\x97a\x0E\xE5V[\x01\x80\x92a\x0E\xE5V[\x01\x80\x92a\x0E\xE5V[\x01\x90V[` \x01\x90V[Q\x90V[\x90V[_R` _ \x90V[T\x90V[a\x0FH\x81a\x0F;V[\x82\x10\x15a\x0FbWa\x0FZ`\x01\x91a\x0F2V[\x91\x02\x01\x90_\x90V[a\x05PV[\x1B\x90V[\x91\x90`\x08a\x0F\x86\x91\x02\x91a\x0F\x80_\x19\x84a\x0FgV[\x92a\x0FgV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x0F\x99\x90a\x05\xEEV[\x90V[_\x1C\x90V[a\x0F\xAA\x90a\x0F\x9CV[\x90V[\x91\x90a\x0F\xC3a\x0F\xBEa\x0F\xCB\x93a\x0F\x90V[a\x0F\xA1V[\x90\x83Ta\x0FkV[\x90UV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x0F\xFFW\x82a\x0F\xF7\x91`\x01a\x0F\xFD\x95\x01\x81Ua\x0F?V[\x90a\x0F\xADV[V[a\x034V[a\x10\x10a\x10\x15\x91a\x0F\x9CV[a\x02\x0CV[\x90V[a\x10\"\x90Ta\x10\x04V[\x90V[a\x10.\x90a\x02\xBAV[\x90RV[\x90``\x80a\x10x\x93a\x10J_\x82\x01Q_\x86\x01\x90a\x10%V[a\x10\\` \x82\x01Q` \x86\x01\x90a\x10%V[a\x10n`@\x82\x01Q`@\x86\x01\x90a\x10%V[\x01Q\x91\x01\x90a\x10%V[V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x05\x11\x15a\x10\x98WV[a\x10zV[\x90a\x10\xA7\x82a\x10\x8EV[V[a\x10\xB2\x90a\x10\x9DV[\x90V[a\x10\xBE\x90a\x10\xA9V[\x90RV[a\x10\xF7a\x10\xFE\x94a\x10\xED`\xC0\x94\x98\x97\x95a\x10\xE3`\xE0\x86\x01\x9A_\x87\x01\x90a\x05\xF1V[` \x85\x01\x90a\x01\xB1V[`@\x83\x01\x90a\x102V[\x01\x90a\x10\xB5V[V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[a\x113a\x11<` \x93a\x11A\x93a\x11*\x81a\x0F+V[\x93\x84\x80\x93a\x11\0V[\x95\x86\x91\x01a\x11\tV[a\x03*V[\x01\x90V[a\x11Z\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x11\x14V[\x90V[\x93\x91a\x11la\x11s\x92_a\r\xBEV[`\x01a\x0E\x13V[\x81a\x11\x86a\x11\x80_a\x0E6V[\x91a\x02\xBAV[\x11a\x13`W[PPa\x11\x98`\x05a\x05dV[a\x11\xA3\x82\x82\x90a\x1A\x8BV[\x90a\x11\xAE`\x06a\x05dV[\x92a\x11\xB8_a\x0ERV[\x84a\x11\xCBa\x11\xC5_a\x0EnV[\x91a\x01\xAEV[\x11a\x132W[a\x11\xDA_a\x0ERV[\x92\x82a\x11\xEEa\x11\xE8_a\x0EnV[\x91a\x01\xAEV[\x11a\x12\xFDW[a\x12R\x90a\x12#\x83a\x12\x14\x87a\x12\x08a\x01\x91V[\x94\x85\x93` \x85\x01a\x0E\xFAV[` \x82\x01\x81\x03\x82R\x03\x82a\x03HV[a\x125a\x12/\x82a\x0F+V[\x91a\x0F%V[ \x92a\x12Ka\x12D`\x06a\x0F/V[\x85\x90a\x0F\xCFV[`\x07a\x0E\x13V[\x84\x90\x91\x92\x93a\x12a`\x07a\x10\x18V[\x90\x94a\x12\xB6`\x01a\x12\xA4a\x12\x9Ea\x12\x98\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x97a\r\xF4V[\x97a\x0F\x90V[\x97a\x0F\x90V[\x97a\x12\xADa\x01\x91V[\x94\x85\x94\x85a\x10\xC2V[\x03\x90\xA4a\x12\xF8a\x12\xE6\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x92a\r\xF4V[\x92a\x12\xEFa\x01\x91V[\x91\x82\x91\x82a\x11EV[\x03\x90\xA2V[\x92Pa\x12Ra\x13*a\x13$`\x05a\x13\x1E\x86a\x13\x18`\x01a\x0E\x8DV[\x90a\x0E\xBDV[\x90a\x05qV[\x90a\x05\xB4V[\x93\x90Pa\x11\xF4V[Pa\x13[a\x13U`\x06a\x13O\x87a\x13I`\x01a\x0E\x8DV[\x90a\x0E\xBDV[\x90a\x05qV[\x90a\x05\xB4V[a\x11\xD1V[a\x13na\x13u\x92`\x02a\r\xBEV[`\x03a\x0E\x13V[_\x80a\x11\x8CV[a\x13\x84a\rcV[Pa\x13\x8F`\x06a\x05dV[\x90V[a\x13\xA6a\x13\xA1a\x13\xAB\x92a\x01\xAEV[a\x06\x8FV[a\x02\xBAV[\x90V[`\xF8\x1B\x90V[a\x13\xBD\x90a\x13\xAEV[\x90V[a\x13\xCCa\x13\xD1\x91a\x06\xFFV[a\x13\xB4V[\x90RV[``\x1B\x90V[a\x13\xE4\x90a\x13\xD5V[\x90V[a\x13\xF0\x90a\x13\xDBV[\x90V[a\x13\xFFa\x14\x04\x91a\x08\x88V[a\x13\xE7V[\x90RV[`\xC0\x1B\x90V[a\x14\x17\x90a\x14\x08V[\x90V[a\x14&a\x14+\x91a\x02\xBAV[a\x14\x0EV[\x90RV[\x90V[a\x14>a\x14C\x91a\x01\xAEV[a\x14/V[\x90RV[\x94a\x14\x98`\x08` \x99\x98\x95\x96a\x14\x90\x82\x8C\x99a\x14\x88`\x14a\x14\xA0\x9Aa\x14\x80a\x14\xA8\x9F\x8F\x9C\x90a\x14x\x81`\x01\x93a\x13\xC0V[\x01\x80\x92a\x13\xF3V[\x01\x80\x92a\x14\x1AV[\x01\x80\x92a\x14\x1AV[\x01\x80\x92a\x142V[\x01\x80\x92a\x142V[\x01\x80\x92a\x0E\xE5V[\x01\x90V[` \x81a\x14\xBEa\x14\xC6\x93\x83\x96\x95a\x0E\xE5V[\x01\x80\x92a\x0E\xE5V[\x01\x90V[a\x14\xDEa\x14\xD9a\x14\xE3\x92a\x08}V[a\x06\x8FV[a\x08}V[\x90V[a\x14\xEF\x90a\x14\xCAV[\x90V[a\x14\xFB\x90a\x14\xE6V[\x90V[a\x15\x07\x90a\x0EnV[\x90RV[\x91\x94a\x15Sa\x15]\x92\x98\x97\x95a\x15I`\xA0\x96a\x15?a\x15d\x9Aa\x155`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x08\x94V[` \x89\x01\x90a\x079V[`@\x87\x01\x90a\x08\x94V[``\x85\x01\x90a\x05\xF1V[`\x80\x83\x01\x90a\x14\xFEV[\x01\x90a\x02\xC7V[V[\x90a\x15q`\x05a\x05dV[\x91\x83a\x15\x85a\x15\x7F\x82a\x0F+V[\x91a\x0F%V[ \x91\x81a\x15\xD2\x82\x91a\x15\xC3a\x15\x99Ca\x13\x92V[a\x15\xA2Ba\x13\x92V[\x89a\x15\xAC_a\x0EnV[\x91\x8A\x93a\x15\xB7a\x01\x91V[\x98\x89\x97` \x89\x01a\x14GV[` \x82\x01\x81\x03\x82R\x03\x82a\x03HV[a\x15\xE4a\x15\xDE\x82a\x0F+V[\x91a\x0F%V[ \x90a\x15\xEF_a\x0ERV[\x91\x85a\x16\x03a\x15\xFD_a\x0EnV[\x91a\x01\xAEV[\x11a\x17\x04W[a\x16[\x90a\x16\x17`\x05a\x0F/V[\x90a\x16B\x85a\x163a\x16'a\x01\x91V[\x93\x84\x92` \x84\x01a\x14\xACV[` \x82\x01\x81\x03\x82R\x03\x82a\x03HV[a\x16Ta\x16N\x82a\x0F+V[\x91a\x0F%V[ \x90a\x0F\xCFV[\x84\x91\x92a\x16\xBDa\x16j0a\x14\xF2V[\x91\x92\x95_a\x16wBa\x13\x92V[\x91a\x16\xABa\x16\xA5\x7F^<\x13\x11\xEAD&d\xE8\xB1a\x1B\xFA\xBE\xF6Y\x12\x0E\xA7\xA0\xA2\xCF\xC0fw\0\xBE\xBCi\xCB\xFF\xE1\x98a\r\xF4V[\x98a\x0F\x90V[\x98a\x16\xB4a\x01\x91V[\x96\x87\x96\x87a\x15\x0BV[\x03\x90\xA3a\x16\xFFa\x16\xED\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x92a\r\xF4V[\x92a\x16\xF6a\x01\x91V[\x91\x82\x91\x82a\x11EV[\x03\x90\xA2V[\x91Pa\x16[a\x171a\x17+`\x05a\x17%\x89a\x17\x1F`\x01a\x0E\x8DV[\x90a\x0E\xBDV[\x90a\x05qV[\x90a\x05\xB4V[\x92\x90Pa\x16\tV[_\x90V[a\x17Ia\x17N\x91a\x0F\x9CV[a\x02yV[\x90V[a\x17[\x90Ta\x17=V[\x90V[a\x17fa\x179V[Pa\x17oa\rcV[Pa\x17xa\x179V[Pa\x17\x81a\rcV[Pa\x17\x8B_a\x17QV[\x90a\x17\x96`\x01a\x10\x18V[\x91a\x17\xA1`\x02a\x17QV[\x91a\x17\xAC`\x03a\x10\x18V[\x91\x93\x92\x91\x90V[_\x90V[a\x17\xCEa\x17\xD4\x91a\x17\xC6a\x17\xB3V[P`\x06a\x05qV[\x90a\x05\xB4V[\x90V[`\x14\x81a\x17\xEAa\x17\xF2\x93` \x96\x95a\x13\xF3V[\x01\x80\x92a\x142V[\x01\x90V[\x90a\x184\x92\x91a\x18/a\x18\x07a\x07!V[\x91\x92a\x18 a\x18\x14a\x01\x91V[\x95\x86\x92` \x84\x01a\x17\xD7V[` \x82\x01\x81\x03\x82R\x03\x84a\x03HV[a\x15fV[V[a\x18>a\rcV[Pa\x18I`\x05a\x05dV[\x90V[a\x18V`\x80a\x03qV[\x90V[_\x90V[a\x18ea\x18LV[\x90` \x80\x80\x80\x85a\x18ta\x18YV[\x81R\x01a\x18\x7Fa\x18YV[\x81R\x01a\x18\x8Aa\x18YV[\x81R\x01a\x18\x95a\x18YV[\x81RPPV[a\x18\xA3a\x18]V[\x90V[\x90V[a\x18\xBDa\x18\xB8a\x18\xC2\x92a\x18\xA6V[a\x06\x8FV[a\x01\xAEV[\x90V[a\x18\xCF`(a\x18\xA9V[\x90V[a\x18\xE1a\x18\xE7\x91\x93\x92\x93a\x01\xAEV[\x92a\x01\xAEV[\x82\x01\x80\x92\x11a\x18\xF2WV[a\x0E\xA9V[a\x19\x0Ba\x19\x06a\x19\x10\x92a\x02\xBAV[a\x06\x8FV[a\x01\xAEV[\x90V[a\x19\x1C\x90a\x18\xF7V[\x90RV[\x91` a\x19A\x92\x94\x93a\x19:`@\x82\x01\x96_\x83\x01\x90a\x01\xB1V[\x01\x90a\x19\x13V[V[a\x19Oa\x19U\x91a\x02\xBAV[\x91a\x02\xBAV[\x90\x03\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x19iWV[a\x0E\xA9V[\x90a\x19x\x90a\x02\xBAV[\x90RV[a\x19\x88a\x19\x8E\x91a\x02\xBAV[\x91a\x02\xBAV[\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x19\xA1WV[a\x0E\xA9V[a\x19\xB0\x90Qa\x02\xBAV[\x90V[`\x08a\x19\xF5\x94a\x19\xE5\x82\x80\x99\x98\x95\x96a\x19\xDD\x82\x87a\x19\xD5a\x19\xED\x99\x83\x9Ca\x14\x1AV[\x01\x80\x92a\x14\x1AV[\x01\x80\x92a\x14\x1AV[\x01\x80\x92a\x14\x1AV[\x01\x80\x92a\x14\x1AV[\x01\x90V[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[\x15a\x1A\x14WV[a\x19\xF9V[\x90P\x90V[a\x1ACa\x1A:\x92` \x92a\x1A1\x81a\x0F+V[\x94\x85\x80\x93a\x1A\x19V[\x93\x84\x91\x01a\x11\tV[\x01\x90V[a\x1AU\x90a\x1A[\x93\x92a\x1A\x1EV[\x90a\x1A\x1EV[\x90V[a\x1A}\x92\x91a\x1A\x89\x91a\x1Aoa\x01\x91V[\x94\x85\x92` \x84\x01\x92\x83a\x1AGV[\x90\x81\x03\x82R\x03\x83a\x03HV[V[\x91\x90\x91a\x1A\x96a\x17\xB3V[Pa\x1A\x9Fa\x18\x9BV[Pa\x1A\xBAa\x1A\xABa\x18\xC5V[a\x1A\xB4\x83a\x0F+V[\x90a\x18\xD2V[\x80a\x1A\xD4a\x1A\xCEa\x1A\xC9a\x0C\x97V[a\x18\xF7V[\x91a\x01\xAEV[\x11a\x1CsWPa\x1C\x07\x90a\x1A\xE6a\x18\x9BV[\x93Ba\x1B\x01a\x1A\xFBa\x1A\xF6a\n\xAEV[a\x18\xF7V[\x91a\x01\xAEV[\x11a\x1CIW[a\x1B-a\x1B$a\x1B\x16Ba\x13\x92V[a\x1B\x1Ea\x06\xAEV[\x90a\x19|V[` \x87\x01a\x19nV[Ca\x1BGa\x1BAa\x1B<a\t\xD5V[a\x18\xF7V[\x91a\x01\xAEV[\x11a\x1C\x1EW[a\x1Bsa\x1Bja\x1B\\Ca\x13\x92V[a\x1Bda\x07\xACV[\x90a\x19|V[``\x87\x01a\x19nV[a\x1B\xD7a\x1B\x81_\x87\x01a\x19\xA6V[a\x1B\xC8a\x1B\x90` \x89\x01a\x19\xA6V[\x93a\x1B\x9D`@\x8A\x01a\x19\xA6V[\x90a\x1B\xB3a\x1B\xAD``\x8C\x01a\x19\xA6V[\x91a\x13\x92V[\x91a\x1B\xBCa\x01\x91V[\x96\x87\x95` \x87\x01a\x19\xB3V[` \x82\x01\x81\x03\x82R\x03\x82a\x03HV[a\x1C\x02a\x1B\xE3\x82a\x0F+V[a\x1B\xFCa\x1B\xF6a\x1B\xF1a\x18\xC5V[a\x01\xAEV[\x91a\x01\xAEV[\x14a\x1A\rV[a\x1A^V[a\x1C\x19a\x1C\x13\x82a\x0F+V[\x91a\x0F%V[ \x91\x90V[a\x1CDa\x1C;a\x1C-Ca\x13\x92V[a\x1C5a\t\xD5V[\x90a\x19CV[`@\x87\x01a\x19nV[a\x1BMV[a\x1Cna\x1Cfa\x1CXBa\x13\x92V[a\x1C`a\n\xAEV[\x90a\x19CV[_\x87\x01a\x19nV[a\x1B\x07V[a\x1C{a\x0C\x97V[\x90a\x1C\x96_\x92\x83\x92cF4i\x1B`\xE0\x1B\x84R`\x04\x84\x01a\x19 V[\x03\x90\xFD",
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
constructor(uint256 chainId, string chainConfig, address owner_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub chainConfig: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub owner_: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::String,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.chainId, value.chainConfig, value.owner_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainId: tuple.0,
                        chainConfig: tuple.1,
                        owner_: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner_,
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
        owner(ownerCall),
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
            [141u8, 165u8, 203u8, 91u8],
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
        const COUNT: usize = 23usize;
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
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
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
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupCalls::owner)
                    }
                    owner
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
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
        owner_: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<RollupInstance<T, P, N>>,
    > {
        RollupInstance::<T, P, N>::deploy(provider, chainId, chainConfig, owner_)
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
        owner_: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        RollupInstance::<T, P, N>::deploy_builder(provider, chainId, chainConfig, owner_)
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
            owner_: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<RollupInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                chainId,
                chainConfig,
                owner_,
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
            chainId: alloy::sol_types::private::primitives::aliases::U256,
            chainConfig: alloy::sol_types::private::String,
            owner_: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            chainId,
                            chainConfig,
                            owner_,
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
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
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
