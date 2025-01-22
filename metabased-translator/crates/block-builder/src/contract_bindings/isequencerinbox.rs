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
///Module containing a contract's types and functions.
/**

```solidity
library Messages {
    struct Message { uint8 kind; address sender; uint64 blockNumber; uint64 timestamp; uint256 inboxSeqNum; uint256 baseFeeL1; bytes32 messageDataHash; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Messages {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Message { uint8 kind; address sender; uint64 blockNumber; uint64 timestamp; uint256 inboxSeqNum; uint256 baseFeeL1; bytes32 messageDataHash; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Message {
        pub kind: u8,
        pub sender: alloy::sol_types::private::Address,
        pub blockNumber: u64,
        pub timestamp: u64,
        pub inboxSeqNum: alloy::sol_types::private::primitives::aliases::U256,
        pub baseFeeL1: alloy::sol_types::private::primitives::aliases::U256,
        pub messageDataHash: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u8,
            alloy::sol_types::private::Address,
            u64,
            u64,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<Message> for UnderlyingRustTuple<'_> {
            fn from(value: Message) -> Self {
                (
                    value.kind,
                    value.sender,
                    value.blockNumber,
                    value.timestamp,
                    value.inboxSeqNum,
                    value.baseFeeL1,
                    value.messageDataHash,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Message {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    kind: tuple.0,
                    sender: tuple.1,
                    blockNumber: tuple.2,
                    timestamp: tuple.3,
                    inboxSeqNum: tuple.4,
                    baseFeeL1: tuple.5,
                    messageDataHash: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Message {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Message {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.kind),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.timestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.inboxSeqNum),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseFeeL1),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.messageDataHash),
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
        impl alloy_sol_types::SolType for Message {
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
        impl alloy_sol_types::SolStruct for Message {
            const NAME: &'static str = "Message";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Message(uint8 kind,address sender,uint64 blockNumber,uint64 timestamp,uint256 inboxSeqNum,uint256 baseFeeL1,bytes32 messageDataHash)",
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
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.kind)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sender,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.blockNumber)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.timestamp)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.inboxSeqNum)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.baseFeeL1)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.messageDataHash,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Message {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.kind)
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sender,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blockNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.timestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.inboxSeqNum,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.baseFeeL1,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.messageDataHash,
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
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.kind,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sender,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.blockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.timestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.inboxSeqNum,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.baseFeeL1,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.messageDataHash,
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
    /**Creates a new wrapper around an on-chain [`Messages`](self) contract instance.

See the [wrapper's documentation](`MessagesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> MessagesInstance<T, P, N> {
        MessagesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`Messages`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Messages`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct MessagesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for MessagesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("MessagesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > MessagesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Messages`](self) contract instance.

See the [wrapper's documentation](`MessagesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> MessagesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> MessagesInstance<T, P, N> {
            MessagesInstance {
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
    > MessagesInstance<T, P, N> {
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
    > MessagesInstance<T, P, N> {
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

library Messages {
    struct Message {
        uint8 kind;
        address sender;
        uint64 blockNumber;
        uint64 timestamp;
        uint256 inboxSeqNum;
        uint256 baseFeeL1;
        bytes32 messageDataHash;
    }
}

interface ISequencerInbox {
    struct BufferConfig {
        uint64 threshold;
        uint64 max;
        uint64 replenishRateInBasis;
    }
    struct DelayProof {
        bytes32 beforeDelayedAcc;
        Messages.Message delayedMessage;
    }
    struct MaxTimeVariation {
        uint256 delayBlocks;
        uint256 futureBlocks;
        uint256 delaySeconds;
        uint256 futureSeconds;
    }

    event BatchPosterManagerSet(address newBatchPosterManager);
    event BatchPosterSet(address batchPoster, bool isBatchPoster);
    event BufferConfigSet(BufferConfig bufferConfig);
    event InboxMessageDelivered(uint256 indexed messageNum, bytes data);
    event InboxMessageDeliveredFromOrigin(uint256 indexed messageNum);
    event InvalidateKeyset(bytes32 indexed keysetHash);
    event MaxTimeVariationSet(MaxTimeVariation maxTimeVariation);
    event OwnerFunctionCalled(uint256 indexed id);
    event SequencerBatchData(uint256 indexed batchSequenceNumber, bytes data);
    event SequencerBatchDelivered(uint256 indexed batchSequenceNumber, bytes32 indexed beforeAcc, bytes32 indexed afterAcc, bytes32 delayedAcc, uint256 afterDelayedMessagesRead, IBridge.TimeBounds timeBounds, IBridge.BatchDataLocation dataLocation);
    event SequencerSet(address addr, bool isSequencer);
    event SetValidKeyset(bytes32 indexed keysetHash, bytes keysetBytes);

    function BROTLI_MESSAGE_HEADER_FLAG() external view returns (bytes1);
    function DAS_MESSAGE_HEADER_FLAG() external view returns (bytes1);
    function DATA_AUTHENTICATED_FLAG() external view returns (bytes1);
    function DATA_BLOB_HEADER_FLAG() external view returns (bytes1);
    function HEADER_LENGTH() external view returns (uint256);
    function TREE_DAS_MESSAGE_HEADER_FLAG() external view returns (bytes1);
    function ZERO_HEAVY_MESSAGE_HEADER_FLAG() external view returns (bytes1);
    function addSequencerL2Batch(uint256 sequenceNumber, bytes memory data, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount) external;
    function addSequencerL2BatchDelayProof(uint256 sequenceNumber, bytes memory data, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount, DelayProof memory delayProof) external;
    function addSequencerL2BatchFromBlobs(uint256 sequenceNumber, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount) external;
    function addSequencerL2BatchFromBlobsDelayProof(uint256 sequenceNumber, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount, DelayProof memory delayProof) external;
    function addSequencerL2BatchFromOrigin(uint256 sequenceNumber, bytes memory data, uint256 afterDelayedMessagesRead, address gasRefunder) external;
    function addSequencerL2BatchFromOrigin(uint256 sequenceNumber, bytes memory data, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount) external;
    function addSequencerL2BatchFromOriginDelayProof(uint256 sequenceNumber, bytes memory data, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount, DelayProof memory delayProof) external;
    function batchCount() external view returns (uint256);
    function batchPosterManager() external view returns (address);
    function bridge() external view returns (address);
    function dasKeySetInfo(bytes32) external view returns (bool, uint64);
    function forceInclusion(uint256 _totalDelayedMessagesRead, uint8 kind, uint64[2] memory l1BlockAndTime, uint256 baseFeeL1, address sender, bytes32 messageDataHash) external;
    function forceInclusionDeadline(uint64 blockNumber) external view returns (uint64 blockNumberDeadline);
    function getKeysetCreationBlock(bytes32 ksHash) external view returns (uint256);
    function inboxAccs(uint256 index) external view returns (bytes32);
    function initialize(address bridge_, MaxTimeVariation memory maxTimeVariation_, BufferConfig memory bufferConfig_) external;
    function invalidateKeysetHash(bytes32 ksHash) external;
    function isBatchPoster(address) external view returns (bool);
    function isDelayBufferable() external view returns (bool);
    function isSequencer(address) external view returns (bool);
    function isValidKeysetHash(bytes32 ksHash) external view returns (bool);
    function maxDataSize() external view returns (uint256);
    function maxTimeVariation() external view returns (uint256 delayBlocks, uint256 futureBlocks, uint256 delaySeconds, uint256 futureSeconds);
    function removeDelayAfterFork() external;
    function rollup() external view returns (address);
    function setBatchPosterManager(address newBatchPosterManager) external;
    function setIsBatchPoster(address addr, bool isBatchPoster_) external;
    function setIsSequencer(address addr, bool isSequencer_) external;
    function setMaxTimeVariation(MaxTimeVariation memory maxTimeVariation_) external;
    function setValidKeyset(bytes memory keysetBytes) external;
    function totalDelayedMessagesRead() external view returns (uint256);
    function updateRollupAddress() external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "BROTLI_MESSAGE_HEADER_FLAG",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes1",
        "internalType": "bytes1"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DAS_MESSAGE_HEADER_FLAG",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes1",
        "internalType": "bytes1"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DATA_AUTHENTICATED_FLAG",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes1",
        "internalType": "bytes1"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DATA_BLOB_HEADER_FLAG",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes1",
        "internalType": "bytes1"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "HEADER_LENGTH",
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
    "name": "TREE_DAS_MESSAGE_HEADER_FLAG",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes1",
        "internalType": "bytes1"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ZERO_HEAVY_MESSAGE_HEADER_FLAG",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes1",
        "internalType": "bytes1"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "addSequencerL2Batch",
    "inputs": [
      {
        "name": "sequenceNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "afterDelayedMessagesRead",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "gasRefunder",
        "type": "address",
        "internalType": "contract IGasRefunder"
      },
      {
        "name": "prevMessageCount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "newMessageCount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addSequencerL2BatchDelayProof",
    "inputs": [
      {
        "name": "sequenceNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "afterDelayedMessagesRead",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "gasRefunder",
        "type": "address",
        "internalType": "contract IGasRefunder"
      },
      {
        "name": "prevMessageCount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "newMessageCount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "delayProof",
        "type": "tuple",
        "internalType": "struct DelayProof",
        "components": [
          {
            "name": "beforeDelayedAcc",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "delayedMessage",
            "type": "tuple",
            "internalType": "struct Messages.Message",
            "components": [
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
                "name": "blockNumber",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "timestamp",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "inboxSeqNum",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "baseFeeL1",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "messageDataHash",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addSequencerL2BatchFromBlobs",
    "inputs": [
      {
        "name": "sequenceNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "afterDelayedMessagesRead",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "gasRefunder",
        "type": "address",
        "internalType": "contract IGasRefunder"
      },
      {
        "name": "prevMessageCount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "newMessageCount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addSequencerL2BatchFromBlobsDelayProof",
    "inputs": [
      {
        "name": "sequenceNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "afterDelayedMessagesRead",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "gasRefunder",
        "type": "address",
        "internalType": "contract IGasRefunder"
      },
      {
        "name": "prevMessageCount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "newMessageCount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "delayProof",
        "type": "tuple",
        "internalType": "struct DelayProof",
        "components": [
          {
            "name": "beforeDelayedAcc",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "delayedMessage",
            "type": "tuple",
            "internalType": "struct Messages.Message",
            "components": [
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
                "name": "blockNumber",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "timestamp",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "inboxSeqNum",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "baseFeeL1",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "messageDataHash",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addSequencerL2BatchFromOrigin",
    "inputs": [
      {
        "name": "sequenceNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "afterDelayedMessagesRead",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "gasRefunder",
        "type": "address",
        "internalType": "contract IGasRefunder"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addSequencerL2BatchFromOrigin",
    "inputs": [
      {
        "name": "sequenceNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "afterDelayedMessagesRead",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "gasRefunder",
        "type": "address",
        "internalType": "contract IGasRefunder"
      },
      {
        "name": "prevMessageCount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "newMessageCount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addSequencerL2BatchFromOriginDelayProof",
    "inputs": [
      {
        "name": "sequenceNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "afterDelayedMessagesRead",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "gasRefunder",
        "type": "address",
        "internalType": "contract IGasRefunder"
      },
      {
        "name": "prevMessageCount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "newMessageCount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "delayProof",
        "type": "tuple",
        "internalType": "struct DelayProof",
        "components": [
          {
            "name": "beforeDelayedAcc",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "delayedMessage",
            "type": "tuple",
            "internalType": "struct Messages.Message",
            "components": [
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
                "name": "blockNumber",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "timestamp",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "inboxSeqNum",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "baseFeeL1",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "messageDataHash",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          }
        ]
      }
    ],
    "outputs": [],
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
    "name": "batchPosterManager",
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
    "name": "bridge",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBridge"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "dasKeySetInfo",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      },
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
    "name": "forceInclusion",
    "inputs": [
      {
        "name": "_totalDelayedMessagesRead",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "kind",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "l1BlockAndTime",
        "type": "uint64[2]",
        "internalType": "uint64[2]"
      },
      {
        "name": "baseFeeL1",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "messageDataHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "forceInclusionDeadline",
    "inputs": [
      {
        "name": "blockNumber",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "blockNumberDeadline",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getKeysetCreationBlock",
    "inputs": [
      {
        "name": "ksHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
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
    "name": "initialize",
    "inputs": [
      {
        "name": "bridge_",
        "type": "address",
        "internalType": "contract IBridge"
      },
      {
        "name": "maxTimeVariation_",
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
        "name": "bufferConfig_",
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
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "invalidateKeysetHash",
    "inputs": [
      {
        "name": "ksHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isBatchPoster",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isDelayBufferable",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isSequencer",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isValidKeysetHash",
    "inputs": [
      {
        "name": "ksHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "maxTimeVariation",
    "inputs": [],
    "outputs": [
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
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "removeDelayAfterFork",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "rollup",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IOwnable"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setBatchPosterManager",
    "inputs": [
      {
        "name": "newBatchPosterManager",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setIsBatchPoster",
    "inputs": [
      {
        "name": "addr",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "isBatchPoster_",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setIsSequencer",
    "inputs": [
      {
        "name": "addr",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "isSequencer_",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setMaxTimeVariation",
    "inputs": [
      {
        "name": "maxTimeVariation_",
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
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setValidKeyset",
    "inputs": [
      {
        "name": "keysetBytes",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "type": "function",
    "name": "updateRollupAddress",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "BatchPosterManagerSet",
    "inputs": [
      {
        "name": "newBatchPosterManager",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BatchPosterSet",
    "inputs": [
      {
        "name": "batchPoster",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "isBatchPoster",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BufferConfigSet",
    "inputs": [
      {
        "name": "bufferConfig",
        "type": "tuple",
        "indexed": false,
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
    "anonymous": false
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
    "name": "InboxMessageDeliveredFromOrigin",
    "inputs": [
      {
        "name": "messageNum",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "InvalidateKeyset",
    "inputs": [
      {
        "name": "keysetHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MaxTimeVariationSet",
    "inputs": [
      {
        "name": "maxTimeVariation",
        "type": "tuple",
        "indexed": false,
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
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnerFunctionCalled",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
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
    "type": "event",
    "name": "SequencerSet",
    "inputs": [
      {
        "name": "addr",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "isSequencer",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SetValidKeyset",
    "inputs": [
      {
        "name": "keysetHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "keysetBytes",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
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
pub mod ISequencerInbox {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /**```solidity
struct BufferConfig { uint64 threshold; uint64 max; uint64 replenishRateInBasis; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BufferConfig {
        pub threshold: u64,
        pub max: u64,
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
struct DelayProof { bytes32 beforeDelayedAcc; Messages.Message delayedMessage; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelayProof {
        pub beforeDelayedAcc: alloy::sol_types::private::FixedBytes<32>,
        pub delayedMessage: <Messages::Message as alloy::sol_types::SolType>::RustType,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            Messages::Message,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            <Messages::Message as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<DelayProof> for UnderlyingRustTuple<'_> {
            fn from(value: DelayProof) -> Self {
                (value.beforeDelayedAcc, value.delayedMessage)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelayProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    beforeDelayedAcc: tuple.0,
                    delayedMessage: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DelayProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DelayProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.beforeDelayedAcc),
                    <Messages::Message as alloy_sol_types::SolType>::tokenize(
                        &self.delayedMessage,
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
        impl alloy_sol_types::SolType for DelayProof {
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
        impl alloy_sol_types::SolStruct for DelayProof {
            const NAME: &'static str = "DelayProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DelayProof(bytes32 beforeDelayedAcc,Messages.Message delayedMessage)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <Messages::Message as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <Messages::Message as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.beforeDelayedAcc,
                        )
                        .0,
                    <Messages::Message as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delayedMessage,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DelayProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.beforeDelayedAcc,
                    )
                    + <Messages::Message as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delayedMessage,
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
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.beforeDelayedAcc,
                    out,
                );
                <Messages::Message as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delayedMessage,
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
struct MaxTimeVariation { uint256 delayBlocks; uint256 futureBlocks; uint256 delaySeconds; uint256 futureSeconds; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxTimeVariation {
        pub delayBlocks: alloy::sol_types::private::primitives::aliases::U256,
        pub futureBlocks: alloy::sol_types::private::primitives::aliases::U256,
        pub delaySeconds: alloy::sol_types::private::primitives::aliases::U256,
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
    /**Event with signature `BatchPosterManagerSet(address)` and selector `0x3cd6c184800297a0f2b00926a683cbe76890bb7fd01480ac0a10ed6c8f7f6659`.
```solidity
event BatchPosterManagerSet(address newBatchPosterManager);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BatchPosterManagerSet {
        #[allow(missing_docs)]
        pub newBatchPosterManager: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for BatchPosterManagerSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "BatchPosterManagerSet(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                60u8,
                214u8,
                193u8,
                132u8,
                128u8,
                2u8,
                151u8,
                160u8,
                242u8,
                176u8,
                9u8,
                38u8,
                166u8,
                131u8,
                203u8,
                231u8,
                104u8,
                144u8,
                187u8,
                127u8,
                208u8,
                20u8,
                128u8,
                172u8,
                10u8,
                16u8,
                237u8,
                108u8,
                143u8,
                127u8,
                102u8,
                89u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    newBatchPosterManager: data.0,
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
                        &self.newBatchPosterManager,
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
        impl alloy_sol_types::private::IntoLogData for BatchPosterManagerSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BatchPosterManagerSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BatchPosterManagerSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `BatchPosterSet(address,bool)` and selector `0x28bcc5626d357efe966b4b0876aa1ee8ab99e26da4f131f6a2623f1800701c21`.
```solidity
event BatchPosterSet(address batchPoster, bool isBatchPoster);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BatchPosterSet {
        #[allow(missing_docs)]
        pub batchPoster: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub isBatchPoster: bool,
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
        impl alloy_sol_types::SolEvent for BatchPosterSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "BatchPosterSet(address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8,
                188u8,
                197u8,
                98u8,
                109u8,
                53u8,
                126u8,
                254u8,
                150u8,
                107u8,
                75u8,
                8u8,
                118u8,
                170u8,
                30u8,
                232u8,
                171u8,
                153u8,
                226u8,
                109u8,
                164u8,
                241u8,
                49u8,
                246u8,
                162u8,
                98u8,
                63u8,
                24u8,
                0u8,
                112u8,
                28u8,
                33u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    batchPoster: data.0,
                    isBatchPoster: data.1,
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
                        &self.batchPoster,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isBatchPoster,
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
        impl alloy_sol_types::private::IntoLogData for BatchPosterSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BatchPosterSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BatchPosterSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `BufferConfigSet((uint64,uint64,uint64))` and selector `0xaa7a2d8175dee3b637814ad6346005dfcc357165396fb8327f649effe8abcf85`.
```solidity
event BufferConfigSet(BufferConfig bufferConfig);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BufferConfigSet {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for BufferConfigSet {
            type DataTuple<'a> = (BufferConfig,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "BufferConfigSet((uint64,uint64,uint64))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                170u8,
                122u8,
                45u8,
                129u8,
                117u8,
                222u8,
                227u8,
                182u8,
                55u8,
                129u8,
                74u8,
                214u8,
                52u8,
                96u8,
                5u8,
                223u8,
                204u8,
                53u8,
                113u8,
                101u8,
                57u8,
                111u8,
                184u8,
                50u8,
                127u8,
                100u8,
                158u8,
                255u8,
                232u8,
                171u8,
                207u8,
                133u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { bufferConfig: data.0 }
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
                    <BufferConfig as alloy_sol_types::SolType>::tokenize(
                        &self.bufferConfig,
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
        impl alloy_sol_types::private::IntoLogData for BufferConfigSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BufferConfigSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BufferConfigSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
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
    /**Event with signature `InboxMessageDeliveredFromOrigin(uint256)` and selector `0xab532385be8f1005a4b6ba8fa20a2245facb346134ac739fe9a5198dc1580b9c`.
```solidity
event InboxMessageDeliveredFromOrigin(uint256 indexed messageNum);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct InboxMessageDeliveredFromOrigin {
        #[allow(missing_docs)]
        pub messageNum: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for InboxMessageDeliveredFromOrigin {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "InboxMessageDeliveredFromOrigin(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                171u8,
                83u8,
                35u8,
                133u8,
                190u8,
                143u8,
                16u8,
                5u8,
                164u8,
                182u8,
                186u8,
                143u8,
                162u8,
                10u8,
                34u8,
                69u8,
                250u8,
                203u8,
                52u8,
                97u8,
                52u8,
                172u8,
                115u8,
                159u8,
                233u8,
                165u8,
                25u8,
                141u8,
                193u8,
                88u8,
                11u8,
                156u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { messageNum: topics.1 }
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
        impl alloy_sol_types::private::IntoLogData for InboxMessageDeliveredFromOrigin {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&InboxMessageDeliveredFromOrigin>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &InboxMessageDeliveredFromOrigin,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `InvalidateKeyset(bytes32)` and selector `0x5cb4218b272fd214168ac43e90fb4d05d6c36f0b17ffb4c2dd07c234d744eb2a`.
```solidity
event InvalidateKeyset(bytes32 indexed keysetHash);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct InvalidateKeyset {
        #[allow(missing_docs)]
        pub keysetHash: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for InvalidateKeyset {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "InvalidateKeyset(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                92u8,
                180u8,
                33u8,
                139u8,
                39u8,
                47u8,
                210u8,
                20u8,
                22u8,
                138u8,
                196u8,
                62u8,
                144u8,
                251u8,
                77u8,
                5u8,
                214u8,
                195u8,
                111u8,
                11u8,
                23u8,
                255u8,
                180u8,
                194u8,
                221u8,
                7u8,
                194u8,
                52u8,
                215u8,
                68u8,
                235u8,
                42u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { keysetHash: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.keysetHash.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.keysetHash);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for InvalidateKeyset {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&InvalidateKeyset> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &InvalidateKeyset) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `MaxTimeVariationSet((uint256,uint256,uint256,uint256))` and selector `0xaa6a58dad31128ff7ecc2b80987ee6e003df80bc50cd8d0b0d1af0e07da6d19d`.
```solidity
event MaxTimeVariationSet(MaxTimeVariation maxTimeVariation);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MaxTimeVariationSet {
        #[allow(missing_docs)]
        pub maxTimeVariation: <MaxTimeVariation as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for MaxTimeVariationSet {
            type DataTuple<'a> = (MaxTimeVariation,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MaxTimeVariationSet((uint256,uint256,uint256,uint256))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                170u8,
                106u8,
                88u8,
                218u8,
                211u8,
                17u8,
                40u8,
                255u8,
                126u8,
                204u8,
                43u8,
                128u8,
                152u8,
                126u8,
                230u8,
                224u8,
                3u8,
                223u8,
                128u8,
                188u8,
                80u8,
                205u8,
                141u8,
                11u8,
                13u8,
                26u8,
                240u8,
                224u8,
                125u8,
                166u8,
                209u8,
                157u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { maxTimeVariation: data.0 }
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
                    <MaxTimeVariation as alloy_sol_types::SolType>::tokenize(
                        &self.maxTimeVariation,
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
        impl alloy_sol_types::private::IntoLogData for MaxTimeVariationSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MaxTimeVariationSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MaxTimeVariationSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnerFunctionCalled(uint256)` and selector `0xea8787f128d10b2cc0317b0c3960f9ad447f7f6c1ed189db1083ccffd20f456e`.
```solidity
event OwnerFunctionCalled(uint256 indexed id);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnerFunctionCalled {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for OwnerFunctionCalled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "OwnerFunctionCalled(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                234u8,
                135u8,
                135u8,
                241u8,
                40u8,
                209u8,
                11u8,
                44u8,
                192u8,
                49u8,
                123u8,
                12u8,
                57u8,
                96u8,
                249u8,
                173u8,
                68u8,
                127u8,
                127u8,
                108u8,
                30u8,
                209u8,
                137u8,
                219u8,
                16u8,
                131u8,
                204u8,
                255u8,
                210u8,
                15u8,
                69u8,
                110u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { id: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.id.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.id);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnerFunctionCalled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnerFunctionCalled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnerFunctionCalled) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `SequencerSet(address,bool)` and selector `0xeb12a9a53eec138c91b27b4f912a257bd690c18fc8bde744be92a0365eb9b87e`.
```solidity
event SequencerSet(address addr, bool isSequencer);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SequencerSet {
        #[allow(missing_docs)]
        pub addr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub isSequencer: bool,
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
        impl alloy_sol_types::SolEvent for SequencerSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SequencerSet(address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                235u8,
                18u8,
                169u8,
                165u8,
                62u8,
                236u8,
                19u8,
                140u8,
                145u8,
                178u8,
                123u8,
                79u8,
                145u8,
                42u8,
                37u8,
                123u8,
                214u8,
                144u8,
                193u8,
                143u8,
                200u8,
                189u8,
                231u8,
                68u8,
                190u8,
                146u8,
                160u8,
                54u8,
                94u8,
                185u8,
                184u8,
                126u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    addr: data.0,
                    isSequencer: data.1,
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
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isSequencer,
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
        impl alloy_sol_types::private::IntoLogData for SequencerSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SequencerSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SequencerSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SetValidKeyset(bytes32,bytes)` and selector `0xabca9b7986bc22ad0160eb0cb88ae75411eacfba4052af0b457a9335ef655722`.
```solidity
event SetValidKeyset(bytes32 indexed keysetHash, bytes keysetBytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SetValidKeyset {
        #[allow(missing_docs)]
        pub keysetHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub keysetBytes: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for SetValidKeyset {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "SetValidKeyset(bytes32,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                171u8,
                202u8,
                155u8,
                121u8,
                134u8,
                188u8,
                34u8,
                173u8,
                1u8,
                96u8,
                235u8,
                12u8,
                184u8,
                138u8,
                231u8,
                84u8,
                17u8,
                234u8,
                207u8,
                186u8,
                64u8,
                82u8,
                175u8,
                11u8,
                69u8,
                122u8,
                147u8,
                53u8,
                239u8,
                101u8,
                87u8,
                34u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    keysetHash: topics.1,
                    keysetBytes: data.0,
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
                        &self.keysetBytes,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.keysetHash.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.keysetHash);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SetValidKeyset {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SetValidKeyset> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SetValidKeyset) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `BROTLI_MESSAGE_HEADER_FLAG()` and selector `0x16af91a7`.
```solidity
function BROTLI_MESSAGE_HEADER_FLAG() external view returns (bytes1);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BROTLI_MESSAGE_HEADER_FLAGCall {}
    ///Container type for the return parameters of the [`BROTLI_MESSAGE_HEADER_FLAG()`](BROTLI_MESSAGE_HEADER_FLAGCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BROTLI_MESSAGE_HEADER_FLAGReturn {
        pub _0: alloy::sol_types::private::FixedBytes<1>,
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
            impl ::core::convert::From<BROTLI_MESSAGE_HEADER_FLAGCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: BROTLI_MESSAGE_HEADER_FLAGCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for BROTLI_MESSAGE_HEADER_FLAGCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<1>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<1>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<BROTLI_MESSAGE_HEADER_FLAGReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: BROTLI_MESSAGE_HEADER_FLAGReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for BROTLI_MESSAGE_HEADER_FLAGReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for BROTLI_MESSAGE_HEADER_FLAGCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = BROTLI_MESSAGE_HEADER_FLAGReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<1>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BROTLI_MESSAGE_HEADER_FLAG()";
            const SELECTOR: [u8; 4] = [22u8, 175u8, 145u8, 167u8];
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
    /**Function with signature `DAS_MESSAGE_HEADER_FLAG()` and selector `0xf60a5091`.
```solidity
function DAS_MESSAGE_HEADER_FLAG() external view returns (bytes1);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DAS_MESSAGE_HEADER_FLAGCall {}
    ///Container type for the return parameters of the [`DAS_MESSAGE_HEADER_FLAG()`](DAS_MESSAGE_HEADER_FLAGCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DAS_MESSAGE_HEADER_FLAGReturn {
        pub _0: alloy::sol_types::private::FixedBytes<1>,
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
            impl ::core::convert::From<DAS_MESSAGE_HEADER_FLAGCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DAS_MESSAGE_HEADER_FLAGCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DAS_MESSAGE_HEADER_FLAGCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<1>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<1>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DAS_MESSAGE_HEADER_FLAGReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DAS_MESSAGE_HEADER_FLAGReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DAS_MESSAGE_HEADER_FLAGReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DAS_MESSAGE_HEADER_FLAGCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DAS_MESSAGE_HEADER_FLAGReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<1>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DAS_MESSAGE_HEADER_FLAG()";
            const SELECTOR: [u8; 4] = [246u8, 10u8, 80u8, 145u8];
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
    /**Function with signature `DATA_AUTHENTICATED_FLAG()` and selector `0xe5a358c8`.
```solidity
function DATA_AUTHENTICATED_FLAG() external view returns (bytes1);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DATA_AUTHENTICATED_FLAGCall {}
    ///Container type for the return parameters of the [`DATA_AUTHENTICATED_FLAG()`](DATA_AUTHENTICATED_FLAGCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DATA_AUTHENTICATED_FLAGReturn {
        pub _0: alloy::sol_types::private::FixedBytes<1>,
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
            impl ::core::convert::From<DATA_AUTHENTICATED_FLAGCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DATA_AUTHENTICATED_FLAGCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DATA_AUTHENTICATED_FLAGCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<1>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<1>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DATA_AUTHENTICATED_FLAGReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DATA_AUTHENTICATED_FLAGReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DATA_AUTHENTICATED_FLAGReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DATA_AUTHENTICATED_FLAGCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DATA_AUTHENTICATED_FLAGReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<1>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DATA_AUTHENTICATED_FLAG()";
            const SELECTOR: [u8; 4] = [229u8, 163u8, 88u8, 200u8];
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
    /**Function with signature `DATA_BLOB_HEADER_FLAG()` and selector `0x2cbf74e5`.
```solidity
function DATA_BLOB_HEADER_FLAG() external view returns (bytes1);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DATA_BLOB_HEADER_FLAGCall {}
    ///Container type for the return parameters of the [`DATA_BLOB_HEADER_FLAG()`](DATA_BLOB_HEADER_FLAGCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DATA_BLOB_HEADER_FLAGReturn {
        pub _0: alloy::sol_types::private::FixedBytes<1>,
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
            impl ::core::convert::From<DATA_BLOB_HEADER_FLAGCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DATA_BLOB_HEADER_FLAGCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DATA_BLOB_HEADER_FLAGCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<1>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<1>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DATA_BLOB_HEADER_FLAGReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DATA_BLOB_HEADER_FLAGReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DATA_BLOB_HEADER_FLAGReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DATA_BLOB_HEADER_FLAGCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DATA_BLOB_HEADER_FLAGReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<1>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DATA_BLOB_HEADER_FLAG()";
            const SELECTOR: [u8; 4] = [44u8, 191u8, 116u8, 229u8];
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
    /**Function with signature `HEADER_LENGTH()` and selector `0x27957a49`.
```solidity
function HEADER_LENGTH() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct HEADER_LENGTHCall {}
    ///Container type for the return parameters of the [`HEADER_LENGTH()`](HEADER_LENGTHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct HEADER_LENGTHReturn {
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
            impl ::core::convert::From<HEADER_LENGTHCall> for UnderlyingRustTuple<'_> {
                fn from(value: HEADER_LENGTHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for HEADER_LENGTHCall {
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
            impl ::core::convert::From<HEADER_LENGTHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: HEADER_LENGTHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for HEADER_LENGTHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for HEADER_LENGTHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = HEADER_LENGTHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "HEADER_LENGTH()";
            const SELECTOR: [u8; 4] = [39u8, 149u8, 122u8, 73u8];
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
    /**Function with signature `TREE_DAS_MESSAGE_HEADER_FLAG()` and selector `0x6c890450`.
```solidity
function TREE_DAS_MESSAGE_HEADER_FLAG() external view returns (bytes1);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TREE_DAS_MESSAGE_HEADER_FLAGCall {}
    ///Container type for the return parameters of the [`TREE_DAS_MESSAGE_HEADER_FLAG()`](TREE_DAS_MESSAGE_HEADER_FLAGCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TREE_DAS_MESSAGE_HEADER_FLAGReturn {
        pub _0: alloy::sol_types::private::FixedBytes<1>,
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
            impl ::core::convert::From<TREE_DAS_MESSAGE_HEADER_FLAGCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: TREE_DAS_MESSAGE_HEADER_FLAGCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TREE_DAS_MESSAGE_HEADER_FLAGCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<1>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<1>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<TREE_DAS_MESSAGE_HEADER_FLAGReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: TREE_DAS_MESSAGE_HEADER_FLAGReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TREE_DAS_MESSAGE_HEADER_FLAGReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TREE_DAS_MESSAGE_HEADER_FLAGCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = TREE_DAS_MESSAGE_HEADER_FLAGReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<1>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TREE_DAS_MESSAGE_HEADER_FLAG()";
            const SELECTOR: [u8; 4] = [108u8, 137u8, 4u8, 80u8];
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
    /**Function with signature `ZERO_HEAVY_MESSAGE_HEADER_FLAG()` and selector `0x02c99275`.
```solidity
function ZERO_HEAVY_MESSAGE_HEADER_FLAG() external view returns (bytes1);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZERO_HEAVY_MESSAGE_HEADER_FLAGCall {}
    ///Container type for the return parameters of the [`ZERO_HEAVY_MESSAGE_HEADER_FLAG()`](ZERO_HEAVY_MESSAGE_HEADER_FLAGCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZERO_HEAVY_MESSAGE_HEADER_FLAGReturn {
        pub _0: alloy::sol_types::private::FixedBytes<1>,
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
            impl ::core::convert::From<ZERO_HEAVY_MESSAGE_HEADER_FLAGCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ZERO_HEAVY_MESSAGE_HEADER_FLAGCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ZERO_HEAVY_MESSAGE_HEADER_FLAGCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<1>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<1>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ZERO_HEAVY_MESSAGE_HEADER_FLAGReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ZERO_HEAVY_MESSAGE_HEADER_FLAGReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ZERO_HEAVY_MESSAGE_HEADER_FLAGReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ZERO_HEAVY_MESSAGE_HEADER_FLAGCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ZERO_HEAVY_MESSAGE_HEADER_FLAGReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<1>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZERO_HEAVY_MESSAGE_HEADER_FLAG()";
            const SELECTOR: [u8; 4] = [2u8, 201u8, 146u8, 117u8];
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
    /**Function with signature `addSequencerL2Batch(uint256,bytes,uint256,address,uint256,uint256)` and selector `0xe0bc9729`.
```solidity
function addSequencerL2Batch(uint256 sequenceNumber, bytes memory data, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchCall {
        pub sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        pub data: alloy::sol_types::private::Bytes,
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        pub gasRefunder: alloy::sol_types::private::Address,
        pub prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        pub newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`addSequencerL2Batch(uint256,bytes,uint256,address,uint256,uint256)`](addSequencerL2BatchCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<addSequencerL2BatchCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchCall) -> Self {
                    (
                        value.sequenceNumber,
                        value.data,
                        value.afterDelayedMessagesRead,
                        value.gasRefunder,
                        value.prevMessageCount,
                        value.newMessageCount,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sequenceNumber: tuple.0,
                        data: tuple.1,
                        afterDelayedMessagesRead: tuple.2,
                        gasRefunder: tuple.3,
                        prevMessageCount: tuple.4,
                        newMessageCount: tuple.5,
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
            impl ::core::convert::From<addSequencerL2BatchReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addSequencerL2BatchCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addSequencerL2BatchReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addSequencerL2Batch(uint256,bytes,uint256,address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [224u8, 188u8, 151u8, 41u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.sequenceNumber),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.afterDelayedMessagesRead,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.gasRefunder,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevMessageCount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMessageCount),
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
    /**Function with signature `addSequencerL2BatchDelayProof(uint256,bytes,uint256,address,uint256,uint256,(bytes32,(uint8,address,uint64,uint64,uint256,uint256,bytes32)))` and selector `0x6e620055`.
```solidity
function addSequencerL2BatchDelayProof(uint256 sequenceNumber, bytes memory data, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount, DelayProof memory delayProof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchDelayProofCall {
        pub sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        pub data: alloy::sol_types::private::Bytes,
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        pub gasRefunder: alloy::sol_types::private::Address,
        pub prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        pub newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        pub delayProof: <DelayProof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`addSequencerL2BatchDelayProof(uint256,bytes,uint256,address,uint256,uint256,(bytes32,(uint8,address,uint64,uint64,uint256,uint256,bytes32)))`](addSequencerL2BatchDelayProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchDelayProofReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                DelayProof,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                <DelayProof as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<addSequencerL2BatchDelayProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchDelayProofCall) -> Self {
                    (
                        value.sequenceNumber,
                        value.data,
                        value.afterDelayedMessagesRead,
                        value.gasRefunder,
                        value.prevMessageCount,
                        value.newMessageCount,
                        value.delayProof,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchDelayProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sequenceNumber: tuple.0,
                        data: tuple.1,
                        afterDelayedMessagesRead: tuple.2,
                        gasRefunder: tuple.3,
                        prevMessageCount: tuple.4,
                        newMessageCount: tuple.5,
                        delayProof: tuple.6,
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
            impl ::core::convert::From<addSequencerL2BatchDelayProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchDelayProofReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchDelayProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addSequencerL2BatchDelayProofCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                DelayProof,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addSequencerL2BatchDelayProofReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addSequencerL2BatchDelayProof(uint256,bytes,uint256,address,uint256,uint256,(bytes32,(uint8,address,uint64,uint64,uint256,uint256,bytes32)))";
            const SELECTOR: [u8; 4] = [110u8, 98u8, 0u8, 85u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.sequenceNumber),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.afterDelayedMessagesRead,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.gasRefunder,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevMessageCount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMessageCount),
                    <DelayProof as alloy_sol_types::SolType>::tokenize(&self.delayProof),
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
    /**Function with signature `addSequencerL2BatchFromBlobs(uint256,uint256,address,uint256,uint256)` and selector `0x3e5aa082`.
```solidity
function addSequencerL2BatchFromBlobs(uint256 sequenceNumber, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchFromBlobsCall {
        pub sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        pub gasRefunder: alloy::sol_types::private::Address,
        pub prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        pub newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`addSequencerL2BatchFromBlobs(uint256,uint256,address,uint256,uint256)`](addSequencerL2BatchFromBlobsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchFromBlobsReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<addSequencerL2BatchFromBlobsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchFromBlobsCall) -> Self {
                    (
                        value.sequenceNumber,
                        value.afterDelayedMessagesRead,
                        value.gasRefunder,
                        value.prevMessageCount,
                        value.newMessageCount,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchFromBlobsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sequenceNumber: tuple.0,
                        afterDelayedMessagesRead: tuple.1,
                        gasRefunder: tuple.2,
                        prevMessageCount: tuple.3,
                        newMessageCount: tuple.4,
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
            impl ::core::convert::From<addSequencerL2BatchFromBlobsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchFromBlobsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchFromBlobsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addSequencerL2BatchFromBlobsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addSequencerL2BatchFromBlobsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addSequencerL2BatchFromBlobs(uint256,uint256,address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [62u8, 90u8, 160u8, 130u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.sequenceNumber),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.afterDelayedMessagesRead,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.gasRefunder,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevMessageCount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMessageCount),
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
    /**Function with signature `addSequencerL2BatchFromBlobsDelayProof(uint256,uint256,address,uint256,uint256,(bytes32,(uint8,address,uint64,uint64,uint256,uint256,bytes32)))` and selector `0x917cf8ac`.
```solidity
function addSequencerL2BatchFromBlobsDelayProof(uint256 sequenceNumber, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount, DelayProof memory delayProof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchFromBlobsDelayProofCall {
        pub sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        pub gasRefunder: alloy::sol_types::private::Address,
        pub prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        pub newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        pub delayProof: <DelayProof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`addSequencerL2BatchFromBlobsDelayProof(uint256,uint256,address,uint256,uint256,(bytes32,(uint8,address,uint64,uint64,uint256,uint256,bytes32)))`](addSequencerL2BatchFromBlobsDelayProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchFromBlobsDelayProofReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                DelayProof,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                <DelayProof as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<addSequencerL2BatchFromBlobsDelayProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchFromBlobsDelayProofCall) -> Self {
                    (
                        value.sequenceNumber,
                        value.afterDelayedMessagesRead,
                        value.gasRefunder,
                        value.prevMessageCount,
                        value.newMessageCount,
                        value.delayProof,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchFromBlobsDelayProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sequenceNumber: tuple.0,
                        afterDelayedMessagesRead: tuple.1,
                        gasRefunder: tuple.2,
                        prevMessageCount: tuple.3,
                        newMessageCount: tuple.4,
                        delayProof: tuple.5,
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
            impl ::core::convert::From<addSequencerL2BatchFromBlobsDelayProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchFromBlobsDelayProofReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchFromBlobsDelayProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addSequencerL2BatchFromBlobsDelayProofCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                DelayProof,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addSequencerL2BatchFromBlobsDelayProofReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addSequencerL2BatchFromBlobsDelayProof(uint256,uint256,address,uint256,uint256,(bytes32,(uint8,address,uint64,uint64,uint256,uint256,bytes32)))";
            const SELECTOR: [u8; 4] = [145u8, 124u8, 248u8, 172u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.sequenceNumber),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.afterDelayedMessagesRead,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.gasRefunder,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevMessageCount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMessageCount),
                    <DelayProof as alloy_sol_types::SolType>::tokenize(&self.delayProof),
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
    /**Function with signature `addSequencerL2BatchFromOrigin(uint256,bytes,uint256,address)` and selector `0x6f12b0c9`.
```solidity
function addSequencerL2BatchFromOrigin(uint256 sequenceNumber, bytes memory data, uint256 afterDelayedMessagesRead, address gasRefunder) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchFromOrigin_0Call {
        pub sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        pub data: alloy::sol_types::private::Bytes,
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        pub gasRefunder: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`addSequencerL2BatchFromOrigin(uint256,bytes,uint256,address)`](addSequencerL2BatchFromOrigin_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchFromOrigin_0Return {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<addSequencerL2BatchFromOrigin_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchFromOrigin_0Call) -> Self {
                    (
                        value.sequenceNumber,
                        value.data,
                        value.afterDelayedMessagesRead,
                        value.gasRefunder,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchFromOrigin_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sequenceNumber: tuple.0,
                        data: tuple.1,
                        afterDelayedMessagesRead: tuple.2,
                        gasRefunder: tuple.3,
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
            impl ::core::convert::From<addSequencerL2BatchFromOrigin_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchFromOrigin_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchFromOrigin_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addSequencerL2BatchFromOrigin_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addSequencerL2BatchFromOrigin_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addSequencerL2BatchFromOrigin(uint256,bytes,uint256,address)";
            const SELECTOR: [u8; 4] = [111u8, 18u8, 176u8, 201u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.sequenceNumber),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.afterDelayedMessagesRead,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.gasRefunder,
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
    /**Function with signature `addSequencerL2BatchFromOrigin(uint256,bytes,uint256,address,uint256,uint256)` and selector `0x8f111f3c`.
```solidity
function addSequencerL2BatchFromOrigin(uint256 sequenceNumber, bytes memory data, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchFromOrigin_1Call {
        pub sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        pub data: alloy::sol_types::private::Bytes,
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        pub gasRefunder: alloy::sol_types::private::Address,
        pub prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        pub newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`addSequencerL2BatchFromOrigin(uint256,bytes,uint256,address,uint256,uint256)`](addSequencerL2BatchFromOrigin_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchFromOrigin_1Return {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<addSequencerL2BatchFromOrigin_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchFromOrigin_1Call) -> Self {
                    (
                        value.sequenceNumber,
                        value.data,
                        value.afterDelayedMessagesRead,
                        value.gasRefunder,
                        value.prevMessageCount,
                        value.newMessageCount,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchFromOrigin_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sequenceNumber: tuple.0,
                        data: tuple.1,
                        afterDelayedMessagesRead: tuple.2,
                        gasRefunder: tuple.3,
                        prevMessageCount: tuple.4,
                        newMessageCount: tuple.5,
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
            impl ::core::convert::From<addSequencerL2BatchFromOrigin_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchFromOrigin_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchFromOrigin_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addSequencerL2BatchFromOrigin_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addSequencerL2BatchFromOrigin_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addSequencerL2BatchFromOrigin(uint256,bytes,uint256,address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [143u8, 17u8, 31u8, 60u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.sequenceNumber),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.afterDelayedMessagesRead,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.gasRefunder,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevMessageCount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMessageCount),
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
    /**Function with signature `addSequencerL2BatchFromOriginDelayProof(uint256,bytes,uint256,address,uint256,uint256,(bytes32,(uint8,address,uint64,uint64,uint256,uint256,bytes32)))` and selector `0x69cacded`.
```solidity
function addSequencerL2BatchFromOriginDelayProof(uint256 sequenceNumber, bytes memory data, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount, DelayProof memory delayProof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchFromOriginDelayProofCall {
        pub sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        pub data: alloy::sol_types::private::Bytes,
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        pub gasRefunder: alloy::sol_types::private::Address,
        pub prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        pub newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        pub delayProof: <DelayProof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`addSequencerL2BatchFromOriginDelayProof(uint256,bytes,uint256,address,uint256,uint256,(bytes32,(uint8,address,uint64,uint64,uint256,uint256,bytes32)))`](addSequencerL2BatchFromOriginDelayProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchFromOriginDelayProofReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                DelayProof,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                <DelayProof as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<addSequencerL2BatchFromOriginDelayProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchFromOriginDelayProofCall) -> Self {
                    (
                        value.sequenceNumber,
                        value.data,
                        value.afterDelayedMessagesRead,
                        value.gasRefunder,
                        value.prevMessageCount,
                        value.newMessageCount,
                        value.delayProof,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchFromOriginDelayProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sequenceNumber: tuple.0,
                        data: tuple.1,
                        afterDelayedMessagesRead: tuple.2,
                        gasRefunder: tuple.3,
                        prevMessageCount: tuple.4,
                        newMessageCount: tuple.5,
                        delayProof: tuple.6,
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
            impl ::core::convert::From<addSequencerL2BatchFromOriginDelayProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSequencerL2BatchFromOriginDelayProofReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchFromOriginDelayProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addSequencerL2BatchFromOriginDelayProofCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                DelayProof,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addSequencerL2BatchFromOriginDelayProofReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addSequencerL2BatchFromOriginDelayProof(uint256,bytes,uint256,address,uint256,uint256,(bytes32,(uint8,address,uint64,uint64,uint256,uint256,bytes32)))";
            const SELECTOR: [u8; 4] = [105u8, 202u8, 205u8, 237u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.sequenceNumber),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.afterDelayedMessagesRead,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.gasRefunder,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevMessageCount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMessageCount),
                    <DelayProof as alloy_sol_types::SolType>::tokenize(&self.delayProof),
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
    /**Function with signature `batchPosterManager()` and selector `0xcc2a1a0c`.
```solidity
function batchPosterManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct batchPosterManagerCall {}
    ///Container type for the return parameters of the [`batchPosterManager()`](batchPosterManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct batchPosterManagerReturn {
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
            impl ::core::convert::From<batchPosterManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: batchPosterManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for batchPosterManagerCall {
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
            impl ::core::convert::From<batchPosterManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: batchPosterManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for batchPosterManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for batchPosterManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = batchPosterManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "batchPosterManager()";
            const SELECTOR: [u8; 4] = [204u8, 42u8, 26u8, 12u8];
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
    /**Function with signature `bridge()` and selector `0xe78cea92`.
```solidity
function bridge() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bridgeCall {}
    ///Container type for the return parameters of the [`bridge()`](bridgeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bridgeReturn {
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
            impl ::core::convert::From<bridgeCall> for UnderlyingRustTuple<'_> {
                fn from(value: bridgeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bridgeCall {
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
            impl ::core::convert::From<bridgeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bridgeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bridgeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bridgeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bridgeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bridge()";
            const SELECTOR: [u8; 4] = [231u8, 140u8, 234u8, 146u8];
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
    /**Function with signature `dasKeySetInfo(bytes32)` and selector `0x715ea34b`.
```solidity
function dasKeySetInfo(bytes32) external view returns (bool, uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct dasKeySetInfoCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`dasKeySetInfo(bytes32)`](dasKeySetInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct dasKeySetInfoReturn {
        pub _0: bool,
        pub _1: u64,
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
            impl ::core::convert::From<dasKeySetInfoCall> for UnderlyingRustTuple<'_> {
                fn from(value: dasKeySetInfoCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for dasKeySetInfoCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, u64);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<dasKeySetInfoReturn> for UnderlyingRustTuple<'_> {
                fn from(value: dasKeySetInfoReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for dasKeySetInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for dasKeySetInfoCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = dasKeySetInfoReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "dasKeySetInfo(bytes32)";
            const SELECTOR: [u8; 4] = [113u8, 94u8, 163u8, 75u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
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
    /**Function with signature `forceInclusion(uint256,uint8,uint64[2],uint256,address,bytes32)` and selector `0xf1981578`.
```solidity
function forceInclusion(uint256 _totalDelayedMessagesRead, uint8 kind, uint64[2] memory l1BlockAndTime, uint256 baseFeeL1, address sender, bytes32 messageDataHash) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct forceInclusionCall {
        pub _totalDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        pub kind: u8,
        pub l1BlockAndTime: [u64; 2usize],
        pub baseFeeL1: alloy::sol_types::private::primitives::aliases::U256,
        pub sender: alloy::sol_types::private::Address,
        pub messageDataHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`forceInclusion(uint256,uint8,uint64[2],uint256,address,bytes32)`](forceInclusionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct forceInclusionReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<64>,
                    2usize,
                >,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                [u64; 2usize],
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<forceInclusionCall> for UnderlyingRustTuple<'_> {
                fn from(value: forceInclusionCall) -> Self {
                    (
                        value._totalDelayedMessagesRead,
                        value.kind,
                        value.l1BlockAndTime,
                        value.baseFeeL1,
                        value.sender,
                        value.messageDataHash,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for forceInclusionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _totalDelayedMessagesRead: tuple.0,
                        kind: tuple.1,
                        l1BlockAndTime: tuple.2,
                        baseFeeL1: tuple.3,
                        sender: tuple.4,
                        messageDataHash: tuple.5,
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
            impl ::core::convert::From<forceInclusionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: forceInclusionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for forceInclusionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for forceInclusionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<64>,
                    2usize,
                >,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = forceInclusionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "forceInclusion(uint256,uint8,uint64[2],uint256,address,bytes32)";
            const SELECTOR: [u8; 4] = [241u8, 152u8, 21u8, 120u8];
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._totalDelayedMessagesRead,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.kind),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<64>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.l1BlockAndTime),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseFeeL1),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.messageDataHash),
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
    /**Function with signature `forceInclusionDeadline(uint64)` and selector `0xdd44e6e0`.
```solidity
function forceInclusionDeadline(uint64 blockNumber) external view returns (uint64 blockNumberDeadline);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct forceInclusionDeadlineCall {
        pub blockNumber: u64,
    }
    ///Container type for the return parameters of the [`forceInclusionDeadline(uint64)`](forceInclusionDeadlineCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct forceInclusionDeadlineReturn {
        pub blockNumberDeadline: u64,
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
            impl ::core::convert::From<forceInclusionDeadlineCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: forceInclusionDeadlineCall) -> Self {
                    (value.blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for forceInclusionDeadlineCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blockNumber: tuple.0 }
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
            impl ::core::convert::From<forceInclusionDeadlineReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: forceInclusionDeadlineReturn) -> Self {
                    (value.blockNumberDeadline,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for forceInclusionDeadlineReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blockNumberDeadline: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for forceInclusionDeadlineCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = forceInclusionDeadlineReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "forceInclusionDeadline(uint64)";
            const SELECTOR: [u8; 4] = [221u8, 68u8, 230u8, 224u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
    /**Function with signature `getKeysetCreationBlock(bytes32)` and selector `0x258f0495`.
```solidity
function getKeysetCreationBlock(bytes32 ksHash) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeysetCreationBlockCall {
        pub ksHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getKeysetCreationBlock(bytes32)`](getKeysetCreationBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeysetCreationBlockReturn {
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
            impl ::core::convert::From<getKeysetCreationBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeysetCreationBlockCall) -> Self {
                    (value.ksHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeysetCreationBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ksHash: tuple.0 }
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
            impl ::core::convert::From<getKeysetCreationBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeysetCreationBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeysetCreationBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeysetCreationBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getKeysetCreationBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeysetCreationBlock(bytes32)";
            const SELECTOR: [u8; 4] = [37u8, 143u8, 4u8, 149u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ksHash),
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
    /**Function with signature `initialize(address,(uint256,uint256,uint256,uint256),(uint64,uint64,uint64))` and selector `0x1ad87e45`.
```solidity
function initialize(address bridge_, MaxTimeVariation memory maxTimeVariation_, BufferConfig memory bufferConfig_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub bridge_: alloy::sol_types::private::Address,
        pub maxTimeVariation_: <MaxTimeVariation as alloy::sol_types::SolType>::RustType,
        pub bufferConfig_: <BufferConfig as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initialize(address,(uint256,uint256,uint256,uint256),(uint64,uint64,uint64))`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
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
                MaxTimeVariation,
                BufferConfig,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <MaxTimeVariation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.bridge_, value.maxTimeVariation_, value.bufferConfig_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        bridge_: tuple.0,
                        maxTimeVariation_: tuple.1,
                        bufferConfig_: tuple.2,
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
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                MaxTimeVariation,
                BufferConfig,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,(uint256,uint256,uint256,uint256),(uint64,uint64,uint64))";
            const SELECTOR: [u8; 4] = [26u8, 216u8, 126u8, 69u8];
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
                        &self.bridge_,
                    ),
                    <MaxTimeVariation as alloy_sol_types::SolType>::tokenize(
                        &self.maxTimeVariation_,
                    ),
                    <BufferConfig as alloy_sol_types::SolType>::tokenize(
                        &self.bufferConfig_,
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
    /**Function with signature `invalidateKeysetHash(bytes32)` and selector `0x84420860`.
```solidity
function invalidateKeysetHash(bytes32 ksHash) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct invalidateKeysetHashCall {
        pub ksHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`invalidateKeysetHash(bytes32)`](invalidateKeysetHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct invalidateKeysetHashReturn {}
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
            impl ::core::convert::From<invalidateKeysetHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: invalidateKeysetHashCall) -> Self {
                    (value.ksHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for invalidateKeysetHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ksHash: tuple.0 }
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
            impl ::core::convert::From<invalidateKeysetHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: invalidateKeysetHashReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for invalidateKeysetHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for invalidateKeysetHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = invalidateKeysetHashReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "invalidateKeysetHash(bytes32)";
            const SELECTOR: [u8; 4] = [132u8, 66u8, 8u8, 96u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ksHash),
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
    /**Function with signature `isBatchPoster(address)` and selector `0x71c3e6fe`.
```solidity
function isBatchPoster(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isBatchPosterCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isBatchPoster(address)`](isBatchPosterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isBatchPosterReturn {
        pub _0: bool,
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
            impl ::core::convert::From<isBatchPosterCall> for UnderlyingRustTuple<'_> {
                fn from(value: isBatchPosterCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isBatchPosterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isBatchPosterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isBatchPosterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isBatchPosterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isBatchPosterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isBatchPosterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isBatchPoster(address)";
            const SELECTOR: [u8; 4] = [113u8, 195u8, 230u8, 254u8];
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
                        &self._0,
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
    /**Function with signature `isDelayBufferable()` and selector `0x4b678a66`.
```solidity
function isDelayBufferable() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDelayBufferableCall {}
    ///Container type for the return parameters of the [`isDelayBufferable()`](isDelayBufferableCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDelayBufferableReturn {
        pub _0: bool,
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
            impl ::core::convert::From<isDelayBufferableCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isDelayBufferableCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isDelayBufferableCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isDelayBufferableReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isDelayBufferableReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isDelayBufferableReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isDelayBufferableCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isDelayBufferableReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isDelayBufferable()";
            const SELECTOR: [u8; 4] = [75u8, 103u8, 138u8, 102u8];
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
    /**Function with signature `isSequencer(address)` and selector `0x6d46e987`.
```solidity
function isSequencer(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isSequencerCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isSequencer(address)`](isSequencerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isSequencerReturn {
        pub _0: bool,
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
            impl ::core::convert::From<isSequencerCall> for UnderlyingRustTuple<'_> {
                fn from(value: isSequencerCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isSequencerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isSequencerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isSequencerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isSequencerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isSequencerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isSequencerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isSequencer(address)";
            const SELECTOR: [u8; 4] = [109u8, 70u8, 233u8, 135u8];
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
                        &self._0,
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
    /**Function with signature `isValidKeysetHash(bytes32)` and selector `0x1637be48`.
```solidity
function isValidKeysetHash(bytes32 ksHash) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidKeysetHashCall {
        pub ksHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isValidKeysetHash(bytes32)`](isValidKeysetHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidKeysetHashReturn {
        pub _0: bool,
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
            impl ::core::convert::From<isValidKeysetHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidKeysetHashCall) -> Self {
                    (value.ksHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidKeysetHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ksHash: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidKeysetHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidKeysetHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidKeysetHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidKeysetHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isValidKeysetHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidKeysetHash(bytes32)";
            const SELECTOR: [u8; 4] = [22u8, 55u8, 190u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ksHash),
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
function maxDataSize() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxDataSizeCall {}
    ///Container type for the return parameters of the [`maxDataSize()`](maxDataSizeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxDataSizeReturn {
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
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
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
    /**Function with signature `maxTimeVariation()` and selector `0xebea461d`.
```solidity
function maxTimeVariation() external view returns (uint256 delayBlocks, uint256 futureBlocks, uint256 delaySeconds, uint256 futureSeconds);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxTimeVariationCall {}
    ///Container type for the return parameters of the [`maxTimeVariation()`](maxTimeVariationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxTimeVariationReturn {
        pub delayBlocks: alloy::sol_types::private::primitives::aliases::U256,
        pub futureBlocks: alloy::sol_types::private::primitives::aliases::U256,
        pub delaySeconds: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<maxTimeVariationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: maxTimeVariationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maxTimeVariationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
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
            impl ::core::convert::From<maxTimeVariationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: maxTimeVariationReturn) -> Self {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maxTimeVariationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        delayBlocks: tuple.0,
                        futureBlocks: tuple.1,
                        delaySeconds: tuple.2,
                        futureSeconds: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for maxTimeVariationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = maxTimeVariationReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "maxTimeVariation()";
            const SELECTOR: [u8; 4] = [235u8, 234u8, 70u8, 29u8];
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
    /**Function with signature `removeDelayAfterFork()` and selector `0x96cc5c78`.
```solidity
function removeDelayAfterFork() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeDelayAfterForkCall {}
    ///Container type for the return parameters of the [`removeDelayAfterFork()`](removeDelayAfterForkCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeDelayAfterForkReturn {}
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
            impl ::core::convert::From<removeDelayAfterForkCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeDelayAfterForkCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeDelayAfterForkCall {
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
            impl ::core::convert::From<removeDelayAfterForkReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeDelayAfterForkReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeDelayAfterForkReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeDelayAfterForkCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeDelayAfterForkReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeDelayAfterFork()";
            const SELECTOR: [u8; 4] = [150u8, 204u8, 92u8, 120u8];
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
    /**Function with signature `rollup()` and selector `0xcb23bcb5`.
```solidity
function rollup() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rollupCall {}
    ///Container type for the return parameters of the [`rollup()`](rollupCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rollupReturn {
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
            impl ::core::convert::From<rollupCall> for UnderlyingRustTuple<'_> {
                fn from(value: rollupCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rollupCall {
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
            impl ::core::convert::From<rollupReturn> for UnderlyingRustTuple<'_> {
                fn from(value: rollupReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rollupReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rollupCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rollupReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rollup()";
            const SELECTOR: [u8; 4] = [203u8, 35u8, 188u8, 181u8];
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
    /**Function with signature `setBatchPosterManager(address)` and selector `0x1ff64790`.
```solidity
function setBatchPosterManager(address newBatchPosterManager) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBatchPosterManagerCall {
        pub newBatchPosterManager: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setBatchPosterManager(address)`](setBatchPosterManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBatchPosterManagerReturn {}
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
            impl ::core::convert::From<setBatchPosterManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setBatchPosterManagerCall) -> Self {
                    (value.newBatchPosterManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setBatchPosterManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newBatchPosterManager: tuple.0,
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
            impl ::core::convert::From<setBatchPosterManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setBatchPosterManagerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setBatchPosterManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setBatchPosterManagerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setBatchPosterManagerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setBatchPosterManager(address)";
            const SELECTOR: [u8; 4] = [31u8, 246u8, 71u8, 144u8];
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
                        &self.newBatchPosterManager,
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
    /**Function with signature `setIsBatchPoster(address,bool)` and selector `0x6e7df3e7`.
```solidity
function setIsBatchPoster(address addr, bool isBatchPoster_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setIsBatchPosterCall {
        pub addr: alloy::sol_types::private::Address,
        pub isBatchPoster_: bool,
    }
    ///Container type for the return parameters of the [`setIsBatchPoster(address,bool)`](setIsBatchPosterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setIsBatchPosterReturn {}
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
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setIsBatchPosterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setIsBatchPosterCall) -> Self {
                    (value.addr, value.isBatchPoster_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setIsBatchPosterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        addr: tuple.0,
                        isBatchPoster_: tuple.1,
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
            impl ::core::convert::From<setIsBatchPosterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setIsBatchPosterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setIsBatchPosterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setIsBatchPosterCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setIsBatchPosterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setIsBatchPoster(address,bool)";
            const SELECTOR: [u8; 4] = [110u8, 125u8, 243u8, 231u8];
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
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isBatchPoster_,
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
    /**Function with signature `setIsSequencer(address,bool)` and selector `0x1f956632`.
```solidity
function setIsSequencer(address addr, bool isSequencer_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setIsSequencerCall {
        pub addr: alloy::sol_types::private::Address,
        pub isSequencer_: bool,
    }
    ///Container type for the return parameters of the [`setIsSequencer(address,bool)`](setIsSequencerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setIsSequencerReturn {}
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
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setIsSequencerCall> for UnderlyingRustTuple<'_> {
                fn from(value: setIsSequencerCall) -> Self {
                    (value.addr, value.isSequencer_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setIsSequencerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        addr: tuple.0,
                        isSequencer_: tuple.1,
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
            impl ::core::convert::From<setIsSequencerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setIsSequencerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setIsSequencerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setIsSequencerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setIsSequencerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setIsSequencer(address,bool)";
            const SELECTOR: [u8; 4] = [31u8, 149u8, 102u8, 50u8];
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
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isSequencer_,
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
    /**Function with signature `setMaxTimeVariation((uint256,uint256,uint256,uint256))` and selector `0xb31761f8`.
```solidity
function setMaxTimeVariation(MaxTimeVariation memory maxTimeVariation_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMaxTimeVariationCall {
        pub maxTimeVariation_: <MaxTimeVariation as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setMaxTimeVariation((uint256,uint256,uint256,uint256))`](setMaxTimeVariationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMaxTimeVariationReturn {}
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
            type UnderlyingSolTuple<'a> = (MaxTimeVariation,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <MaxTimeVariation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<setMaxTimeVariationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setMaxTimeVariationCall) -> Self {
                    (value.maxTimeVariation_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setMaxTimeVariationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { maxTimeVariation_: tuple.0 }
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
            impl ::core::convert::From<setMaxTimeVariationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setMaxTimeVariationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setMaxTimeVariationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setMaxTimeVariationCall {
            type Parameters<'a> = (MaxTimeVariation,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setMaxTimeVariationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setMaxTimeVariation((uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [179u8, 23u8, 97u8, 248u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <MaxTimeVariation as alloy_sol_types::SolType>::tokenize(
                        &self.maxTimeVariation_,
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
    /**Function with signature `setValidKeyset(bytes)` and selector `0xd1ce8da8`.
```solidity
function setValidKeyset(bytes memory keysetBytes) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setValidKeysetCall {
        pub keysetBytes: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`setValidKeyset(bytes)`](setValidKeysetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setValidKeysetReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setValidKeysetCall> for UnderlyingRustTuple<'_> {
                fn from(value: setValidKeysetCall) -> Self {
                    (value.keysetBytes,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setValidKeysetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keysetBytes: tuple.0 }
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
            impl ::core::convert::From<setValidKeysetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setValidKeysetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setValidKeysetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setValidKeysetCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setValidKeysetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setValidKeyset(bytes)";
            const SELECTOR: [u8; 4] = [209u8, 206u8, 141u8, 168u8];
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
                        &self.keysetBytes,
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
    /**Function with signature `updateRollupAddress()` and selector `0x6ae71f12`.
```solidity
function updateRollupAddress() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateRollupAddressCall {}
    ///Container type for the return parameters of the [`updateRollupAddress()`](updateRollupAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateRollupAddressReturn {}
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
            impl ::core::convert::From<updateRollupAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateRollupAddressCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateRollupAddressCall {
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
            impl ::core::convert::From<updateRollupAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateRollupAddressReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateRollupAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateRollupAddressCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateRollupAddressReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateRollupAddress()";
            const SELECTOR: [u8; 4] = [106u8, 231u8, 31u8, 18u8];
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
    ///Container for all the [`ISequencerInbox`](self) function calls.
    pub enum ISequencerInboxCalls {
        BROTLI_MESSAGE_HEADER_FLAG(BROTLI_MESSAGE_HEADER_FLAGCall),
        DAS_MESSAGE_HEADER_FLAG(DAS_MESSAGE_HEADER_FLAGCall),
        DATA_AUTHENTICATED_FLAG(DATA_AUTHENTICATED_FLAGCall),
        DATA_BLOB_HEADER_FLAG(DATA_BLOB_HEADER_FLAGCall),
        HEADER_LENGTH(HEADER_LENGTHCall),
        TREE_DAS_MESSAGE_HEADER_FLAG(TREE_DAS_MESSAGE_HEADER_FLAGCall),
        ZERO_HEAVY_MESSAGE_HEADER_FLAG(ZERO_HEAVY_MESSAGE_HEADER_FLAGCall),
        addSequencerL2Batch(addSequencerL2BatchCall),
        addSequencerL2BatchDelayProof(addSequencerL2BatchDelayProofCall),
        addSequencerL2BatchFromBlobs(addSequencerL2BatchFromBlobsCall),
        addSequencerL2BatchFromBlobsDelayProof(
            addSequencerL2BatchFromBlobsDelayProofCall,
        ),
        addSequencerL2BatchFromOrigin_0(addSequencerL2BatchFromOrigin_0Call),
        addSequencerL2BatchFromOrigin_1(addSequencerL2BatchFromOrigin_1Call),
        addSequencerL2BatchFromOriginDelayProof(
            addSequencerL2BatchFromOriginDelayProofCall,
        ),
        batchCount(batchCountCall),
        batchPosterManager(batchPosterManagerCall),
        bridge(bridgeCall),
        dasKeySetInfo(dasKeySetInfoCall),
        forceInclusion(forceInclusionCall),
        forceInclusionDeadline(forceInclusionDeadlineCall),
        getKeysetCreationBlock(getKeysetCreationBlockCall),
        inboxAccs(inboxAccsCall),
        initialize(initializeCall),
        invalidateKeysetHash(invalidateKeysetHashCall),
        isBatchPoster(isBatchPosterCall),
        isDelayBufferable(isDelayBufferableCall),
        isSequencer(isSequencerCall),
        isValidKeysetHash(isValidKeysetHashCall),
        maxDataSize(maxDataSizeCall),
        maxTimeVariation(maxTimeVariationCall),
        removeDelayAfterFork(removeDelayAfterForkCall),
        rollup(rollupCall),
        setBatchPosterManager(setBatchPosterManagerCall),
        setIsBatchPoster(setIsBatchPosterCall),
        setIsSequencer(setIsSequencerCall),
        setMaxTimeVariation(setMaxTimeVariationCall),
        setValidKeyset(setValidKeysetCall),
        totalDelayedMessagesRead(totalDelayedMessagesReadCall),
        updateRollupAddress(updateRollupAddressCall),
    }
    #[automatically_derived]
    impl ISequencerInboxCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [2u8, 201u8, 146u8, 117u8],
            [6u8, 241u8, 48u8, 86u8],
            [22u8, 55u8, 190u8, 72u8],
            [22u8, 175u8, 145u8, 167u8],
            [26u8, 216u8, 126u8, 69u8],
            [31u8, 149u8, 102u8, 50u8],
            [31u8, 246u8, 71u8, 144u8],
            [37u8, 143u8, 4u8, 149u8],
            [39u8, 149u8, 122u8, 73u8],
            [44u8, 191u8, 116u8, 229u8],
            [62u8, 90u8, 160u8, 130u8],
            [75u8, 103u8, 138u8, 102u8],
            [105u8, 202u8, 205u8, 237u8],
            [106u8, 231u8, 31u8, 18u8],
            [108u8, 137u8, 4u8, 80u8],
            [109u8, 70u8, 233u8, 135u8],
            [110u8, 98u8, 0u8, 85u8],
            [110u8, 125u8, 243u8, 231u8],
            [111u8, 18u8, 176u8, 201u8],
            [113u8, 94u8, 163u8, 75u8],
            [113u8, 195u8, 230u8, 254u8],
            [127u8, 163u8, 164u8, 14u8],
            [132u8, 66u8, 8u8, 96u8],
            [143u8, 17u8, 31u8, 60u8],
            [145u8, 124u8, 248u8, 172u8],
            [150u8, 204u8, 92u8, 120u8],
            [179u8, 23u8, 97u8, 248u8],
            [203u8, 35u8, 188u8, 181u8],
            [204u8, 42u8, 26u8, 12u8],
            [209u8, 206u8, 141u8, 168u8],
            [217u8, 221u8, 103u8, 171u8],
            [221u8, 68u8, 230u8, 224u8],
            [224u8, 188u8, 151u8, 41u8],
            [229u8, 163u8, 88u8, 200u8],
            [231u8, 140u8, 234u8, 146u8],
            [232u8, 235u8, 29u8, 195u8],
            [235u8, 234u8, 70u8, 29u8],
            [241u8, 152u8, 21u8, 120u8],
            [246u8, 10u8, 80u8, 145u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ISequencerInboxCalls {
        const NAME: &'static str = "ISequencerInboxCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 39usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BROTLI_MESSAGE_HEADER_FLAG(_) => {
                    <BROTLI_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DAS_MESSAGE_HEADER_FLAG(_) => {
                    <DAS_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DATA_AUTHENTICATED_FLAG(_) => {
                    <DATA_AUTHENTICATED_FLAGCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DATA_BLOB_HEADER_FLAG(_) => {
                    <DATA_BLOB_HEADER_FLAGCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::HEADER_LENGTH(_) => {
                    <HEADER_LENGTHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::TREE_DAS_MESSAGE_HEADER_FLAG(_) => {
                    <TREE_DAS_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ZERO_HEAVY_MESSAGE_HEADER_FLAG(_) => {
                    <ZERO_HEAVY_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addSequencerL2Batch(_) => {
                    <addSequencerL2BatchCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addSequencerL2BatchDelayProof(_) => {
                    <addSequencerL2BatchDelayProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addSequencerL2BatchFromBlobs(_) => {
                    <addSequencerL2BatchFromBlobsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addSequencerL2BatchFromBlobsDelayProof(_) => {
                    <addSequencerL2BatchFromBlobsDelayProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addSequencerL2BatchFromOrigin_0(_) => {
                    <addSequencerL2BatchFromOrigin_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addSequencerL2BatchFromOrigin_1(_) => {
                    <addSequencerL2BatchFromOrigin_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addSequencerL2BatchFromOriginDelayProof(_) => {
                    <addSequencerL2BatchFromOriginDelayProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::batchCount(_) => {
                    <batchCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::batchPosterManager(_) => {
                    <batchPosterManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::bridge(_) => <bridgeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::dasKeySetInfo(_) => {
                    <dasKeySetInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::forceInclusion(_) => {
                    <forceInclusionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::forceInclusionDeadline(_) => {
                    <forceInclusionDeadlineCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeysetCreationBlock(_) => {
                    <getKeysetCreationBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::inboxAccs(_) => {
                    <inboxAccsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::invalidateKeysetHash(_) => {
                    <invalidateKeysetHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isBatchPoster(_) => {
                    <isBatchPosterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isDelayBufferable(_) => {
                    <isDelayBufferableCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isSequencer(_) => {
                    <isSequencerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidKeysetHash(_) => {
                    <isValidKeysetHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maxDataSize(_) => {
                    <maxDataSizeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maxTimeVariation(_) => {
                    <maxTimeVariationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeDelayAfterFork(_) => {
                    <removeDelayAfterForkCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rollup(_) => <rollupCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setBatchPosterManager(_) => {
                    <setBatchPosterManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setIsBatchPoster(_) => {
                    <setIsBatchPosterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setIsSequencer(_) => {
                    <setIsSequencerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setMaxTimeVariation(_) => {
                    <setMaxTimeVariationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setValidKeyset(_) => {
                    <setValidKeysetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::totalDelayedMessagesRead(_) => {
                    <totalDelayedMessagesReadCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateRollupAddress(_) => {
                    <updateRollupAddressCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ISequencerInboxCalls>] = &[
                {
                    fn ZERO_HEAVY_MESSAGE_HEADER_FLAG(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <ZERO_HEAVY_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::ZERO_HEAVY_MESSAGE_HEADER_FLAG)
                    }
                    ZERO_HEAVY_MESSAGE_HEADER_FLAG
                },
                {
                    fn batchCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <batchCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::batchCount)
                    }
                    batchCount
                },
                {
                    fn isValidKeysetHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <isValidKeysetHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::isValidKeysetHash)
                    }
                    isValidKeysetHash
                },
                {
                    fn BROTLI_MESSAGE_HEADER_FLAG(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <BROTLI_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::BROTLI_MESSAGE_HEADER_FLAG)
                    }
                    BROTLI_MESSAGE_HEADER_FLAG
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::initialize)
                    }
                    initialize
                },
                {
                    fn setIsSequencer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <setIsSequencerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::setIsSequencer)
                    }
                    setIsSequencer
                },
                {
                    fn setBatchPosterManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <setBatchPosterManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::setBatchPosterManager)
                    }
                    setBatchPosterManager
                },
                {
                    fn getKeysetCreationBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <getKeysetCreationBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::getKeysetCreationBlock)
                    }
                    getKeysetCreationBlock
                },
                {
                    fn HEADER_LENGTH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <HEADER_LENGTHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::HEADER_LENGTH)
                    }
                    HEADER_LENGTH
                },
                {
                    fn DATA_BLOB_HEADER_FLAG(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <DATA_BLOB_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::DATA_BLOB_HEADER_FLAG)
                    }
                    DATA_BLOB_HEADER_FLAG
                },
                {
                    fn addSequencerL2BatchFromBlobs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <addSequencerL2BatchFromBlobsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::addSequencerL2BatchFromBlobs)
                    }
                    addSequencerL2BatchFromBlobs
                },
                {
                    fn isDelayBufferable(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <isDelayBufferableCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::isDelayBufferable)
                    }
                    isDelayBufferable
                },
                {
                    fn addSequencerL2BatchFromOriginDelayProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <addSequencerL2BatchFromOriginDelayProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ISequencerInboxCalls::addSequencerL2BatchFromOriginDelayProof,
                            )
                    }
                    addSequencerL2BatchFromOriginDelayProof
                },
                {
                    fn updateRollupAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <updateRollupAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::updateRollupAddress)
                    }
                    updateRollupAddress
                },
                {
                    fn TREE_DAS_MESSAGE_HEADER_FLAG(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <TREE_DAS_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::TREE_DAS_MESSAGE_HEADER_FLAG)
                    }
                    TREE_DAS_MESSAGE_HEADER_FLAG
                },
                {
                    fn isSequencer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <isSequencerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::isSequencer)
                    }
                    isSequencer
                },
                {
                    fn addSequencerL2BatchDelayProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <addSequencerL2BatchDelayProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::addSequencerL2BatchDelayProof)
                    }
                    addSequencerL2BatchDelayProof
                },
                {
                    fn setIsBatchPoster(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <setIsBatchPosterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::setIsBatchPoster)
                    }
                    setIsBatchPoster
                },
                {
                    fn addSequencerL2BatchFromOrigin_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <addSequencerL2BatchFromOrigin_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::addSequencerL2BatchFromOrigin_0)
                    }
                    addSequencerL2BatchFromOrigin_0
                },
                {
                    fn dasKeySetInfo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <dasKeySetInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::dasKeySetInfo)
                    }
                    dasKeySetInfo
                },
                {
                    fn isBatchPoster(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <isBatchPosterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::isBatchPoster)
                    }
                    isBatchPoster
                },
                {
                    fn totalDelayedMessagesRead(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <totalDelayedMessagesReadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::totalDelayedMessagesRead)
                    }
                    totalDelayedMessagesRead
                },
                {
                    fn invalidateKeysetHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <invalidateKeysetHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::invalidateKeysetHash)
                    }
                    invalidateKeysetHash
                },
                {
                    fn addSequencerL2BatchFromOrigin_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <addSequencerL2BatchFromOrigin_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::addSequencerL2BatchFromOrigin_1)
                    }
                    addSequencerL2BatchFromOrigin_1
                },
                {
                    fn addSequencerL2BatchFromBlobsDelayProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <addSequencerL2BatchFromBlobsDelayProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ISequencerInboxCalls::addSequencerL2BatchFromBlobsDelayProof,
                            )
                    }
                    addSequencerL2BatchFromBlobsDelayProof
                },
                {
                    fn removeDelayAfterFork(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <removeDelayAfterForkCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::removeDelayAfterFork)
                    }
                    removeDelayAfterFork
                },
                {
                    fn setMaxTimeVariation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <setMaxTimeVariationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::setMaxTimeVariation)
                    }
                    setMaxTimeVariation
                },
                {
                    fn rollup(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <rollupCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::rollup)
                    }
                    rollup
                },
                {
                    fn batchPosterManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <batchPosterManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::batchPosterManager)
                    }
                    batchPosterManager
                },
                {
                    fn setValidKeyset(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <setValidKeysetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::setValidKeyset)
                    }
                    setValidKeyset
                },
                {
                    fn inboxAccs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <inboxAccsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::inboxAccs)
                    }
                    inboxAccs
                },
                {
                    fn forceInclusionDeadline(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <forceInclusionDeadlineCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::forceInclusionDeadline)
                    }
                    forceInclusionDeadline
                },
                {
                    fn addSequencerL2Batch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <addSequencerL2BatchCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::addSequencerL2Batch)
                    }
                    addSequencerL2Batch
                },
                {
                    fn DATA_AUTHENTICATED_FLAG(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <DATA_AUTHENTICATED_FLAGCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::DATA_AUTHENTICATED_FLAG)
                    }
                    DATA_AUTHENTICATED_FLAG
                },
                {
                    fn bridge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <bridgeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::bridge)
                    }
                    bridge
                },
                {
                    fn maxDataSize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <maxDataSizeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::maxDataSize)
                    }
                    maxDataSize
                },
                {
                    fn maxTimeVariation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <maxTimeVariationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::maxTimeVariation)
                    }
                    maxTimeVariation
                },
                {
                    fn forceInclusion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <forceInclusionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::forceInclusion)
                    }
                    forceInclusion
                },
                {
                    fn DAS_MESSAGE_HEADER_FLAG(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ISequencerInboxCalls> {
                        <DAS_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ISequencerInboxCalls::DAS_MESSAGE_HEADER_FLAG)
                    }
                    DAS_MESSAGE_HEADER_FLAG
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
                Self::BROTLI_MESSAGE_HEADER_FLAG(inner) => {
                    <BROTLI_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DAS_MESSAGE_HEADER_FLAG(inner) => {
                    <DAS_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DATA_AUTHENTICATED_FLAG(inner) => {
                    <DATA_AUTHENTICATED_FLAGCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DATA_BLOB_HEADER_FLAG(inner) => {
                    <DATA_BLOB_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::HEADER_LENGTH(inner) => {
                    <HEADER_LENGTHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TREE_DAS_MESSAGE_HEADER_FLAG(inner) => {
                    <TREE_DAS_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZERO_HEAVY_MESSAGE_HEADER_FLAG(inner) => {
                    <ZERO_HEAVY_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addSequencerL2Batch(inner) => {
                    <addSequencerL2BatchCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addSequencerL2BatchDelayProof(inner) => {
                    <addSequencerL2BatchDelayProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addSequencerL2BatchFromBlobs(inner) => {
                    <addSequencerL2BatchFromBlobsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addSequencerL2BatchFromBlobsDelayProof(inner) => {
                    <addSequencerL2BatchFromBlobsDelayProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addSequencerL2BatchFromOrigin_0(inner) => {
                    <addSequencerL2BatchFromOrigin_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addSequencerL2BatchFromOrigin_1(inner) => {
                    <addSequencerL2BatchFromOrigin_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addSequencerL2BatchFromOriginDelayProof(inner) => {
                    <addSequencerL2BatchFromOriginDelayProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::batchCount(inner) => {
                    <batchCountCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::batchPosterManager(inner) => {
                    <batchPosterManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::bridge(inner) => {
                    <bridgeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::dasKeySetInfo(inner) => {
                    <dasKeySetInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::forceInclusion(inner) => {
                    <forceInclusionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::forceInclusionDeadline(inner) => {
                    <forceInclusionDeadlineCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKeysetCreationBlock(inner) => {
                    <getKeysetCreationBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::inboxAccs(inner) => {
                    <inboxAccsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::invalidateKeysetHash(inner) => {
                    <invalidateKeysetHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isBatchPoster(inner) => {
                    <isBatchPosterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isDelayBufferable(inner) => {
                    <isDelayBufferableCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isSequencer(inner) => {
                    <isSequencerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isValidKeysetHash(inner) => {
                    <isValidKeysetHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::maxDataSize(inner) => {
                    <maxDataSizeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::maxTimeVariation(inner) => {
                    <maxTimeVariationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeDelayAfterFork(inner) => {
                    <removeDelayAfterForkCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rollup(inner) => {
                    <rollupCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setBatchPosterManager(inner) => {
                    <setBatchPosterManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setIsBatchPoster(inner) => {
                    <setIsBatchPosterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setIsSequencer(inner) => {
                    <setIsSequencerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setMaxTimeVariation(inner) => {
                    <setMaxTimeVariationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setValidKeyset(inner) => {
                    <setValidKeysetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::totalDelayedMessagesRead(inner) => {
                    <totalDelayedMessagesReadCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateRollupAddress(inner) => {
                    <updateRollupAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BROTLI_MESSAGE_HEADER_FLAG(inner) => {
                    <BROTLI_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DAS_MESSAGE_HEADER_FLAG(inner) => {
                    <DAS_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DATA_AUTHENTICATED_FLAG(inner) => {
                    <DATA_AUTHENTICATED_FLAGCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DATA_BLOB_HEADER_FLAG(inner) => {
                    <DATA_BLOB_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::HEADER_LENGTH(inner) => {
                    <HEADER_LENGTHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TREE_DAS_MESSAGE_HEADER_FLAG(inner) => {
                    <TREE_DAS_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZERO_HEAVY_MESSAGE_HEADER_FLAG(inner) => {
                    <ZERO_HEAVY_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addSequencerL2Batch(inner) => {
                    <addSequencerL2BatchCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addSequencerL2BatchDelayProof(inner) => {
                    <addSequencerL2BatchDelayProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addSequencerL2BatchFromBlobs(inner) => {
                    <addSequencerL2BatchFromBlobsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addSequencerL2BatchFromBlobsDelayProof(inner) => {
                    <addSequencerL2BatchFromBlobsDelayProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addSequencerL2BatchFromOrigin_0(inner) => {
                    <addSequencerL2BatchFromOrigin_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addSequencerL2BatchFromOrigin_1(inner) => {
                    <addSequencerL2BatchFromOrigin_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addSequencerL2BatchFromOriginDelayProof(inner) => {
                    <addSequencerL2BatchFromOriginDelayProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::batchPosterManager(inner) => {
                    <batchPosterManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::bridge(inner) => {
                    <bridgeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::dasKeySetInfo(inner) => {
                    <dasKeySetInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::forceInclusion(inner) => {
                    <forceInclusionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::forceInclusionDeadline(inner) => {
                    <forceInclusionDeadlineCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKeysetCreationBlock(inner) => {
                    <getKeysetCreationBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::invalidateKeysetHash(inner) => {
                    <invalidateKeysetHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isBatchPoster(inner) => {
                    <isBatchPosterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isDelayBufferable(inner) => {
                    <isDelayBufferableCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isSequencer(inner) => {
                    <isSequencerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isValidKeysetHash(inner) => {
                    <isValidKeysetHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::maxTimeVariation(inner) => {
                    <maxTimeVariationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeDelayAfterFork(inner) => {
                    <removeDelayAfterForkCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rollup(inner) => {
                    <rollupCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setBatchPosterManager(inner) => {
                    <setBatchPosterManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setIsBatchPoster(inner) => {
                    <setIsBatchPosterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setIsSequencer(inner) => {
                    <setIsSequencerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setMaxTimeVariation(inner) => {
                    <setMaxTimeVariationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setValidKeyset(inner) => {
                    <setValidKeysetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::updateRollupAddress(inner) => {
                    <updateRollupAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ISequencerInbox`](self) events.
    pub enum ISequencerInboxEvents {
        BatchPosterManagerSet(BatchPosterManagerSet),
        BatchPosterSet(BatchPosterSet),
        BufferConfigSet(BufferConfigSet),
        InboxMessageDelivered(InboxMessageDelivered),
        InboxMessageDeliveredFromOrigin(InboxMessageDeliveredFromOrigin),
        InvalidateKeyset(InvalidateKeyset),
        MaxTimeVariationSet(MaxTimeVariationSet),
        OwnerFunctionCalled(OwnerFunctionCalled),
        SequencerBatchData(SequencerBatchData),
        SequencerBatchDelivered(SequencerBatchDelivered),
        SequencerSet(SequencerSet),
        SetValidKeyset(SetValidKeyset),
    }
    #[automatically_derived]
    impl ISequencerInboxEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                40u8,
                188u8,
                197u8,
                98u8,
                109u8,
                53u8,
                126u8,
                254u8,
                150u8,
                107u8,
                75u8,
                8u8,
                118u8,
                170u8,
                30u8,
                232u8,
                171u8,
                153u8,
                226u8,
                109u8,
                164u8,
                241u8,
                49u8,
                246u8,
                162u8,
                98u8,
                63u8,
                24u8,
                0u8,
                112u8,
                28u8,
                33u8,
            ],
            [
                60u8,
                214u8,
                193u8,
                132u8,
                128u8,
                2u8,
                151u8,
                160u8,
                242u8,
                176u8,
                9u8,
                38u8,
                166u8,
                131u8,
                203u8,
                231u8,
                104u8,
                144u8,
                187u8,
                127u8,
                208u8,
                20u8,
                128u8,
                172u8,
                10u8,
                16u8,
                237u8,
                108u8,
                143u8,
                127u8,
                102u8,
                89u8,
            ],
            [
                92u8,
                180u8,
                33u8,
                139u8,
                39u8,
                47u8,
                210u8,
                20u8,
                22u8,
                138u8,
                196u8,
                62u8,
                144u8,
                251u8,
                77u8,
                5u8,
                214u8,
                195u8,
                111u8,
                11u8,
                23u8,
                255u8,
                180u8,
                194u8,
                221u8,
                7u8,
                194u8,
                52u8,
                215u8,
                68u8,
                235u8,
                42u8,
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
                170u8,
                106u8,
                88u8,
                218u8,
                211u8,
                17u8,
                40u8,
                255u8,
                126u8,
                204u8,
                43u8,
                128u8,
                152u8,
                126u8,
                230u8,
                224u8,
                3u8,
                223u8,
                128u8,
                188u8,
                80u8,
                205u8,
                141u8,
                11u8,
                13u8,
                26u8,
                240u8,
                224u8,
                125u8,
                166u8,
                209u8,
                157u8,
            ],
            [
                170u8,
                122u8,
                45u8,
                129u8,
                117u8,
                222u8,
                227u8,
                182u8,
                55u8,
                129u8,
                74u8,
                214u8,
                52u8,
                96u8,
                5u8,
                223u8,
                204u8,
                53u8,
                113u8,
                101u8,
                57u8,
                111u8,
                184u8,
                50u8,
                127u8,
                100u8,
                158u8,
                255u8,
                232u8,
                171u8,
                207u8,
                133u8,
            ],
            [
                171u8,
                83u8,
                35u8,
                133u8,
                190u8,
                143u8,
                16u8,
                5u8,
                164u8,
                182u8,
                186u8,
                143u8,
                162u8,
                10u8,
                34u8,
                69u8,
                250u8,
                203u8,
                52u8,
                97u8,
                52u8,
                172u8,
                115u8,
                159u8,
                233u8,
                165u8,
                25u8,
                141u8,
                193u8,
                88u8,
                11u8,
                156u8,
            ],
            [
                171u8,
                202u8,
                155u8,
                121u8,
                134u8,
                188u8,
                34u8,
                173u8,
                1u8,
                96u8,
                235u8,
                12u8,
                184u8,
                138u8,
                231u8,
                84u8,
                17u8,
                234u8,
                207u8,
                186u8,
                64u8,
                82u8,
                175u8,
                11u8,
                69u8,
                122u8,
                147u8,
                53u8,
                239u8,
                101u8,
                87u8,
                34u8,
            ],
            [
                234u8,
                135u8,
                135u8,
                241u8,
                40u8,
                209u8,
                11u8,
                44u8,
                192u8,
                49u8,
                123u8,
                12u8,
                57u8,
                96u8,
                249u8,
                173u8,
                68u8,
                127u8,
                127u8,
                108u8,
                30u8,
                209u8,
                137u8,
                219u8,
                16u8,
                131u8,
                204u8,
                255u8,
                210u8,
                15u8,
                69u8,
                110u8,
            ],
            [
                235u8,
                18u8,
                169u8,
                165u8,
                62u8,
                236u8,
                19u8,
                140u8,
                145u8,
                178u8,
                123u8,
                79u8,
                145u8,
                42u8,
                37u8,
                123u8,
                214u8,
                144u8,
                193u8,
                143u8,
                200u8,
                189u8,
                231u8,
                68u8,
                190u8,
                146u8,
                160u8,
                54u8,
                94u8,
                185u8,
                184u8,
                126u8,
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
    impl alloy_sol_types::SolEventInterface for ISequencerInboxEvents {
        const NAME: &'static str = "ISequencerInboxEvents";
        const COUNT: usize = 12usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <BatchPosterManagerSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <BatchPosterManagerSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BatchPosterManagerSet)
                }
                Some(<BatchPosterSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BatchPosterSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BatchPosterSet)
                }
                Some(<BufferConfigSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BufferConfigSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BufferConfigSet)
                }
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
                Some(
                    <InboxMessageDeliveredFromOrigin as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <InboxMessageDeliveredFromOrigin as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::InboxMessageDeliveredFromOrigin)
                }
                Some(<InvalidateKeyset as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <InvalidateKeyset as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::InvalidateKeyset)
                }
                Some(
                    <MaxTimeVariationSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MaxTimeVariationSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MaxTimeVariationSet)
                }
                Some(
                    <OwnerFunctionCalled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnerFunctionCalled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnerFunctionCalled)
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
                Some(<SequencerSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SequencerSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SequencerSet)
                }
                Some(<SetValidKeyset as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SetValidKeyset as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SetValidKeyset)
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
    impl alloy_sol_types::private::IntoLogData for ISequencerInboxEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::BatchPosterManagerSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BatchPosterSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BufferConfigSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::InboxMessageDelivered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::InboxMessageDeliveredFromOrigin(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::InvalidateKeyset(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MaxTimeVariationSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnerFunctionCalled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SequencerBatchData(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SequencerBatchDelivered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SequencerSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SetValidKeyset(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::BatchPosterManagerSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BatchPosterSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BufferConfigSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::InboxMessageDelivered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::InboxMessageDeliveredFromOrigin(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::InvalidateKeyset(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MaxTimeVariationSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnerFunctionCalled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SequencerBatchData(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SequencerBatchDelivered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SequencerSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SetValidKeyset(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
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
        Output = alloy_contract::Result<ISequencerInboxInstance<T, P, N>>,
    > {
        ISequencerInboxInstance::<T, P, N>::deploy(provider)
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
        ISequencerInboxInstance::<T, P, N>::deploy_builder(provider)
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
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<ISequencerInboxInstance<T, P, N>> {
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
        ///Creates a new call builder for the [`BROTLI_MESSAGE_HEADER_FLAG`] function.
        pub fn BROTLI_MESSAGE_HEADER_FLAG(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, BROTLI_MESSAGE_HEADER_FLAGCall, N> {
            self.call_builder(&BROTLI_MESSAGE_HEADER_FLAGCall {})
        }
        ///Creates a new call builder for the [`DAS_MESSAGE_HEADER_FLAG`] function.
        pub fn DAS_MESSAGE_HEADER_FLAG(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DAS_MESSAGE_HEADER_FLAGCall, N> {
            self.call_builder(&DAS_MESSAGE_HEADER_FLAGCall {})
        }
        ///Creates a new call builder for the [`DATA_AUTHENTICATED_FLAG`] function.
        pub fn DATA_AUTHENTICATED_FLAG(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DATA_AUTHENTICATED_FLAGCall, N> {
            self.call_builder(&DATA_AUTHENTICATED_FLAGCall {})
        }
        ///Creates a new call builder for the [`DATA_BLOB_HEADER_FLAG`] function.
        pub fn DATA_BLOB_HEADER_FLAG(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DATA_BLOB_HEADER_FLAGCall, N> {
            self.call_builder(&DATA_BLOB_HEADER_FLAGCall {})
        }
        ///Creates a new call builder for the [`HEADER_LENGTH`] function.
        pub fn HEADER_LENGTH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, HEADER_LENGTHCall, N> {
            self.call_builder(&HEADER_LENGTHCall {})
        }
        ///Creates a new call builder for the [`TREE_DAS_MESSAGE_HEADER_FLAG`] function.
        pub fn TREE_DAS_MESSAGE_HEADER_FLAG(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, TREE_DAS_MESSAGE_HEADER_FLAGCall, N> {
            self.call_builder(
                &TREE_DAS_MESSAGE_HEADER_FLAGCall {
                },
            )
        }
        ///Creates a new call builder for the [`ZERO_HEAVY_MESSAGE_HEADER_FLAG`] function.
        pub fn ZERO_HEAVY_MESSAGE_HEADER_FLAG(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            ZERO_HEAVY_MESSAGE_HEADER_FLAGCall,
            N,
        > {
            self.call_builder(
                &ZERO_HEAVY_MESSAGE_HEADER_FLAGCall {
                },
            )
        }
        ///Creates a new call builder for the [`addSequencerL2Batch`] function.
        pub fn addSequencerL2Batch(
            &self,
            sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
            gasRefunder: alloy::sol_types::private::Address,
            prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
            newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, addSequencerL2BatchCall, N> {
            self.call_builder(
                &addSequencerL2BatchCall {
                    sequenceNumber,
                    data,
                    afterDelayedMessagesRead,
                    gasRefunder,
                    prevMessageCount,
                    newMessageCount,
                },
            )
        }
        ///Creates a new call builder for the [`addSequencerL2BatchDelayProof`] function.
        pub fn addSequencerL2BatchDelayProof(
            &self,
            sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
            gasRefunder: alloy::sol_types::private::Address,
            prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
            newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
            delayProof: <DelayProof as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            addSequencerL2BatchDelayProofCall,
            N,
        > {
            self.call_builder(
                &addSequencerL2BatchDelayProofCall {
                    sequenceNumber,
                    data,
                    afterDelayedMessagesRead,
                    gasRefunder,
                    prevMessageCount,
                    newMessageCount,
                    delayProof,
                },
            )
        }
        ///Creates a new call builder for the [`addSequencerL2BatchFromBlobs`] function.
        pub fn addSequencerL2BatchFromBlobs(
            &self,
            sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
            afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
            gasRefunder: alloy::sol_types::private::Address,
            prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
            newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, addSequencerL2BatchFromBlobsCall, N> {
            self.call_builder(
                &addSequencerL2BatchFromBlobsCall {
                    sequenceNumber,
                    afterDelayedMessagesRead,
                    gasRefunder,
                    prevMessageCount,
                    newMessageCount,
                },
            )
        }
        ///Creates a new call builder for the [`addSequencerL2BatchFromBlobsDelayProof`] function.
        pub fn addSequencerL2BatchFromBlobsDelayProof(
            &self,
            sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
            afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
            gasRefunder: alloy::sol_types::private::Address,
            prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
            newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
            delayProof: <DelayProof as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            addSequencerL2BatchFromBlobsDelayProofCall,
            N,
        > {
            self.call_builder(
                &addSequencerL2BatchFromBlobsDelayProofCall {
                    sequenceNumber,
                    afterDelayedMessagesRead,
                    gasRefunder,
                    prevMessageCount,
                    newMessageCount,
                    delayProof,
                },
            )
        }
        ///Creates a new call builder for the [`addSequencerL2BatchFromOrigin_0`] function.
        pub fn addSequencerL2BatchFromOrigin_0(
            &self,
            sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
            gasRefunder: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            addSequencerL2BatchFromOrigin_0Call,
            N,
        > {
            self.call_builder(
                &addSequencerL2BatchFromOrigin_0Call {
                    sequenceNumber,
                    data,
                    afterDelayedMessagesRead,
                    gasRefunder,
                },
            )
        }
        ///Creates a new call builder for the [`addSequencerL2BatchFromOrigin_1`] function.
        pub fn addSequencerL2BatchFromOrigin_1(
            &self,
            sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
            gasRefunder: alloy::sol_types::private::Address,
            prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
            newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            addSequencerL2BatchFromOrigin_1Call,
            N,
        > {
            self.call_builder(
                &addSequencerL2BatchFromOrigin_1Call {
                    sequenceNumber,
                    data,
                    afterDelayedMessagesRead,
                    gasRefunder,
                    prevMessageCount,
                    newMessageCount,
                },
            )
        }
        ///Creates a new call builder for the [`addSequencerL2BatchFromOriginDelayProof`] function.
        pub fn addSequencerL2BatchFromOriginDelayProof(
            &self,
            sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
            gasRefunder: alloy::sol_types::private::Address,
            prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
            newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
            delayProof: <DelayProof as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            addSequencerL2BatchFromOriginDelayProofCall,
            N,
        > {
            self.call_builder(
                &addSequencerL2BatchFromOriginDelayProofCall {
                    sequenceNumber,
                    data,
                    afterDelayedMessagesRead,
                    gasRefunder,
                    prevMessageCount,
                    newMessageCount,
                    delayProof,
                },
            )
        }
        ///Creates a new call builder for the [`batchCount`] function.
        pub fn batchCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, batchCountCall, N> {
            self.call_builder(&batchCountCall {})
        }
        ///Creates a new call builder for the [`batchPosterManager`] function.
        pub fn batchPosterManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, batchPosterManagerCall, N> {
            self.call_builder(&batchPosterManagerCall {})
        }
        ///Creates a new call builder for the [`bridge`] function.
        pub fn bridge(&self) -> alloy_contract::SolCallBuilder<T, &P, bridgeCall, N> {
            self.call_builder(&bridgeCall {})
        }
        ///Creates a new call builder for the [`dasKeySetInfo`] function.
        pub fn dasKeySetInfo(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, dasKeySetInfoCall, N> {
            self.call_builder(&dasKeySetInfoCall { _0 })
        }
        ///Creates a new call builder for the [`forceInclusion`] function.
        pub fn forceInclusion(
            &self,
            _totalDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
            kind: u8,
            l1BlockAndTime: [u64; 2usize],
            baseFeeL1: alloy::sol_types::private::primitives::aliases::U256,
            sender: alloy::sol_types::private::Address,
            messageDataHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, forceInclusionCall, N> {
            self.call_builder(
                &forceInclusionCall {
                    _totalDelayedMessagesRead,
                    kind,
                    l1BlockAndTime,
                    baseFeeL1,
                    sender,
                    messageDataHash,
                },
            )
        }
        ///Creates a new call builder for the [`forceInclusionDeadline`] function.
        pub fn forceInclusionDeadline(
            &self,
            blockNumber: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, forceInclusionDeadlineCall, N> {
            self.call_builder(
                &forceInclusionDeadlineCall {
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getKeysetCreationBlock`] function.
        pub fn getKeysetCreationBlock(
            &self,
            ksHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getKeysetCreationBlockCall, N> {
            self.call_builder(
                &getKeysetCreationBlockCall {
                    ksHash,
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
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            bridge_: alloy::sol_types::private::Address,
            maxTimeVariation_: <MaxTimeVariation as alloy::sol_types::SolType>::RustType,
            bufferConfig_: <BufferConfig as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    bridge_,
                    maxTimeVariation_,
                    bufferConfig_,
                },
            )
        }
        ///Creates a new call builder for the [`invalidateKeysetHash`] function.
        pub fn invalidateKeysetHash(
            &self,
            ksHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, invalidateKeysetHashCall, N> {
            self.call_builder(&invalidateKeysetHashCall { ksHash })
        }
        ///Creates a new call builder for the [`isBatchPoster`] function.
        pub fn isBatchPoster(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isBatchPosterCall, N> {
            self.call_builder(&isBatchPosterCall { _0 })
        }
        ///Creates a new call builder for the [`isDelayBufferable`] function.
        pub fn isDelayBufferable(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, isDelayBufferableCall, N> {
            self.call_builder(&isDelayBufferableCall {})
        }
        ///Creates a new call builder for the [`isSequencer`] function.
        pub fn isSequencer(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isSequencerCall, N> {
            self.call_builder(&isSequencerCall { _0 })
        }
        ///Creates a new call builder for the [`isValidKeysetHash`] function.
        pub fn isValidKeysetHash(
            &self,
            ksHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidKeysetHashCall, N> {
            self.call_builder(&isValidKeysetHashCall { ksHash })
        }
        ///Creates a new call builder for the [`maxDataSize`] function.
        pub fn maxDataSize(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, maxDataSizeCall, N> {
            self.call_builder(&maxDataSizeCall {})
        }
        ///Creates a new call builder for the [`maxTimeVariation`] function.
        pub fn maxTimeVariation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, maxTimeVariationCall, N> {
            self.call_builder(&maxTimeVariationCall {})
        }
        ///Creates a new call builder for the [`removeDelayAfterFork`] function.
        pub fn removeDelayAfterFork(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeDelayAfterForkCall, N> {
            self.call_builder(&removeDelayAfterForkCall {})
        }
        ///Creates a new call builder for the [`rollup`] function.
        pub fn rollup(&self) -> alloy_contract::SolCallBuilder<T, &P, rollupCall, N> {
            self.call_builder(&rollupCall {})
        }
        ///Creates a new call builder for the [`setBatchPosterManager`] function.
        pub fn setBatchPosterManager(
            &self,
            newBatchPosterManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setBatchPosterManagerCall, N> {
            self.call_builder(
                &setBatchPosterManagerCall {
                    newBatchPosterManager,
                },
            )
        }
        ///Creates a new call builder for the [`setIsBatchPoster`] function.
        pub fn setIsBatchPoster(
            &self,
            addr: alloy::sol_types::private::Address,
            isBatchPoster_: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setIsBatchPosterCall, N> {
            self.call_builder(
                &setIsBatchPosterCall {
                    addr,
                    isBatchPoster_,
                },
            )
        }
        ///Creates a new call builder for the [`setIsSequencer`] function.
        pub fn setIsSequencer(
            &self,
            addr: alloy::sol_types::private::Address,
            isSequencer_: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setIsSequencerCall, N> {
            self.call_builder(
                &setIsSequencerCall {
                    addr,
                    isSequencer_,
                },
            )
        }
        ///Creates a new call builder for the [`setMaxTimeVariation`] function.
        pub fn setMaxTimeVariation(
            &self,
            maxTimeVariation_: <MaxTimeVariation as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setMaxTimeVariationCall, N> {
            self.call_builder(
                &setMaxTimeVariationCall {
                    maxTimeVariation_,
                },
            )
        }
        ///Creates a new call builder for the [`setValidKeyset`] function.
        pub fn setValidKeyset(
            &self,
            keysetBytes: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, setValidKeysetCall, N> {
            self.call_builder(&setValidKeysetCall { keysetBytes })
        }
        ///Creates a new call builder for the [`totalDelayedMessagesRead`] function.
        pub fn totalDelayedMessagesRead(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, totalDelayedMessagesReadCall, N> {
            self.call_builder(&totalDelayedMessagesReadCall {})
        }
        ///Creates a new call builder for the [`updateRollupAddress`] function.
        pub fn updateRollupAddress(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateRollupAddressCall, N> {
            self.call_builder(&updateRollupAddressCall {})
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
        ///Creates a new event filter for the [`BatchPosterManagerSet`] event.
        pub fn BatchPosterManagerSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BatchPosterManagerSet, N> {
            self.event_filter::<BatchPosterManagerSet>()
        }
        ///Creates a new event filter for the [`BatchPosterSet`] event.
        pub fn BatchPosterSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BatchPosterSet, N> {
            self.event_filter::<BatchPosterSet>()
        }
        ///Creates a new event filter for the [`BufferConfigSet`] event.
        pub fn BufferConfigSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BufferConfigSet, N> {
            self.event_filter::<BufferConfigSet>()
        }
        ///Creates a new event filter for the [`InboxMessageDelivered`] event.
        pub fn InboxMessageDelivered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, InboxMessageDelivered, N> {
            self.event_filter::<InboxMessageDelivered>()
        }
        ///Creates a new event filter for the [`InboxMessageDeliveredFromOrigin`] event.
        pub fn InboxMessageDeliveredFromOrigin_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, InboxMessageDeliveredFromOrigin, N> {
            self.event_filter::<InboxMessageDeliveredFromOrigin>()
        }
        ///Creates a new event filter for the [`InvalidateKeyset`] event.
        pub fn InvalidateKeyset_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, InvalidateKeyset, N> {
            self.event_filter::<InvalidateKeyset>()
        }
        ///Creates a new event filter for the [`MaxTimeVariationSet`] event.
        pub fn MaxTimeVariationSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MaxTimeVariationSet, N> {
            self.event_filter::<MaxTimeVariationSet>()
        }
        ///Creates a new event filter for the [`OwnerFunctionCalled`] event.
        pub fn OwnerFunctionCalled_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnerFunctionCalled, N> {
            self.event_filter::<OwnerFunctionCalled>()
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
        ///Creates a new event filter for the [`SequencerSet`] event.
        pub fn SequencerSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SequencerSet, N> {
            self.event_filter::<SequencerSet>()
        }
        ///Creates a new event filter for the [`SetValidKeyset`] event.
        pub fn SetValidKeyset_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SetValidKeyset, N> {
            self.event_filter::<SetValidKeyset>()
        }
    }
}
