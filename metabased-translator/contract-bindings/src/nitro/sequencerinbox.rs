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
        #[allow(missing_docs)]
        pub kind: u8,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub blockNumber: u64,
        #[allow(missing_docs)]
        pub timestamp: u64,
        #[allow(missing_docs)]
        pub inboxSeqNum: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseFeeL1: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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

library ISequencerInbox {
    struct MaxTimeVariation {
        uint256 delayBlocks;
        uint256 futureBlocks;
        uint256 delaySeconds;
        uint256 futureSeconds;
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

interface SequencerInbox {
    struct BufferConfig {
        uint64 threshold;
        uint64 max;
        uint64 replenishRateInBasis;
    }
    struct DelayProof {
        bytes32 beforeDelayedAcc;
        Messages.Message delayedMessage;
    }

    error AlreadyInit();
    error AlreadyValidDASKeyset(bytes32);
    error BadBufferConfig();
    error BadMaxTimeVariation();
    error BadSequencerNumber(uint256 stored, uint256 received);
    error DataBlobsNotSupported();
    error DataTooLarge(uint256 dataLength, uint256 maxDataLength);
    error DelayProofRequired();
    error DelayedBackwards();
    error DelayedTooFar();
    error Deprecated();
    error ExtraGasNotUint64();
    error ForceIncludeBlockTooSoon();
    error HadZeroInit();
    error IncorrectMessagePreimage();
    error InitParamZero(string name);
    error InvalidDelayedAccPreimage();
    error InvalidHeaderFlag(bytes1);
    error KeysetTooLarge();
    error MissingDataHashes();
    error NativeTokenMismatch();
    error NoSuchKeyset(bytes32);
    error NotBatchPoster();
    error NotBatchPosterManager(address);
    error NotCodelessOrigin();
    error NotDelayBufferable();
    error NotForked();
    error NotOwner(address sender, address owner);
    error RollupNotChanged();

    event BatchPosterManagerSet(address newBatchPosterManager);
    event BatchPosterSet(address batchPoster, bool isBatchPoster);
    event BufferConfigSet(BufferConfig bufferConfig);
    event InboxMessageDelivered(uint256 indexed messageNum, bytes data);
    event InboxMessageDeliveredFromOrigin(uint256 indexed messageNum);
    event InvalidateKeyset(bytes32 indexed keysetHash);
    event MaxTimeVariationSet(ISequencerInbox.MaxTimeVariation maxTimeVariation);
    event OwnerFunctionCalled(uint256 indexed id);
    event SequencerBatchData(uint256 indexed batchSequenceNumber, bytes data);
    event SequencerBatchDelivered(uint256 indexed batchSequenceNumber, bytes32 indexed beforeAcc, bytes32 indexed afterAcc, bytes32 delayedAcc, uint256 afterDelayedMessagesRead, IBridge.TimeBounds timeBounds, IBridge.BatchDataLocation dataLocation);
    event SequencerSet(address addr, bool isSequencer);
    event SetValidKeyset(bytes32 indexed keysetHash, bytes keysetBytes);

    constructor(uint256 _maxDataSize, address reader4844_, bool _isUsingFeeToken, bool _isDelayBufferable);

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
    function addSequencerL2BatchFromOrigin(uint256, bytes memory, uint256, address) external pure;
    function addSequencerL2BatchFromOrigin(uint256 sequenceNumber, bytes memory data, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount) external;
    function addSequencerL2BatchFromOriginDelayProof(uint256 sequenceNumber, bytes memory data, uint256 afterDelayedMessagesRead, address gasRefunder, uint256 prevMessageCount, uint256 newMessageCount, DelayProof memory delayProof) external;
    function batchCount() external view returns (uint256);
    function batchPosterManager() external view returns (address);
    function bridge() external view returns (address);
    function buffer() external view returns (uint64 bufferBlocks, uint64 max, uint64 threshold, uint64 prevBlockNumber, uint64 replenishRateInBasis, uint64 prevSequencedBlockNumber);
    function dasKeySetInfo(bytes32) external view returns (bool isValidKeyset, uint64 creationBlock);
    function forceInclusion(uint256 _totalDelayedMessagesRead, uint8 kind, uint64[2] memory l1BlockAndTime, uint256 baseFeeL1, address sender, bytes32 messageDataHash) external;
    function forceInclusionDeadline(uint64 blockNumber) external view returns (uint64);
    function getKeysetCreationBlock(bytes32 ksHash) external view returns (uint256);
    function inboxAccs(uint256 index) external view returns (bytes32);
    function initialize(address bridge_, ISequencerInbox.MaxTimeVariation memory maxTimeVariation_, BufferConfig memory bufferConfig_) external;
    function invalidateKeysetHash(bytes32 ksHash) external;
    function isBatchPoster(address) external view returns (bool);
    function isDelayBufferable() external view returns (bool);
    function isSequencer(address) external view returns (bool);
    function isUsingFeeToken() external view returns (bool);
    function isValidKeysetHash(bytes32 ksHash) external view returns (bool);
    function maxDataSize() external view returns (uint256);
    function maxTimeVariation() external view returns (uint256, uint256, uint256, uint256);
    function postUpgradeInit(BufferConfig memory bufferConfig_) external;
    function reader4844() external view returns (address);
    function removeDelayAfterFork() external;
    function rollup() external view returns (address);
    function setBatchPosterManager(address newBatchPosterManager) external;
    function setBufferConfig(BufferConfig memory bufferConfig_) external;
    function setIsBatchPoster(address addr, bool isBatchPoster_) external;
    function setIsSequencer(address addr, bool isSequencer_) external;
    function setMaxTimeVariation(ISequencerInbox.MaxTimeVariation memory maxTimeVariation_) external;
    function setValidKeyset(bytes memory keysetBytes) external;
    function totalDelayedMessagesRead() external view returns (uint256);
    function updateRollupAddress() external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_maxDataSize",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "reader4844_",
        "type": "address",
        "internalType": "contract IReader4844"
      },
      {
        "name": "_isUsingFeeToken",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "_isDelayBufferable",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
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
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "contract IGasRefunder"
      }
    ],
    "outputs": [],
    "stateMutability": "pure"
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
    "name": "buffer",
    "inputs": [],
    "outputs": [
      {
        "name": "bufferBlocks",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "max",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "threshold",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "prevBlockNumber",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "replenishRateInBasis",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "prevSequencedBlockNumber",
        "type": "uint64",
        "internalType": "uint64"
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
        "name": "isValidKeyset",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "creationBlock",
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
        "name": "",
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
    "name": "isUsingFeeToken",
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
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
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
    "name": "postUpgradeInit",
    "inputs": [
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
    "name": "reader4844",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IReader4844"
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
    "name": "setBufferConfig",
    "inputs": [
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
  },
  {
    "type": "error",
    "name": "AlreadyInit",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AlreadyValidDASKeyset",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "BadBufferConfig",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BadMaxTimeVariation",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BadSequencerNumber",
    "inputs": [
      {
        "name": "stored",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "received",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "DataBlobsNotSupported",
    "inputs": []
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
  },
  {
    "type": "error",
    "name": "DelayProofRequired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "DelayedBackwards",
    "inputs": []
  },
  {
    "type": "error",
    "name": "DelayedTooFar",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Deprecated",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExtraGasNotUint64",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ForceIncludeBlockTooSoon",
    "inputs": []
  },
  {
    "type": "error",
    "name": "HadZeroInit",
    "inputs": []
  },
  {
    "type": "error",
    "name": "IncorrectMessagePreimage",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InitParamZero",
    "inputs": [
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidDelayedAccPreimage",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidHeaderFlag",
    "inputs": [
      {
        "name": "",
        "type": "bytes1",
        "internalType": "bytes1"
      }
    ]
  },
  {
    "type": "error",
    "name": "KeysetTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MissingDataHashes",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NativeTokenMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoSuchKeyset",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotBatchPoster",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotBatchPosterManager",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotCodelessOrigin",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotDelayBufferable",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotForked",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotOwner",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "RollupNotChanged",
    "inputs": []
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
pub mod SequencerInbox {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610180604052306080526202000060a052466101005261001d6100f0565b15156101205234801561002e575f5ffd5b5060405161529538038061529583398101604081905261004d91610198565b60e08490526101205115610088576001600160a01b03831615610083576040516386657a5360e01b815260040160405180910390fd5b6100d0565b6001600160a01b0383166100d0576040516380fc2c0360e01b815260206004820152600a60248201526914995859195c8d0e0d0d60b21b604482015260640160405180910390fd5b6001600160a01b0390921660c05215156101405215156101605250610206565b60408051600481526024810182526020810180516001600160e01b03166302881c7960e11b17905290515f918291829160649161012d91906101f0565b5f60405180830381855afa9150503d805f8114610165576040519150601f19603f3d011682016040523d82523d5f602084013e61016a565b606091505b509150915081801561017d575080516020145b9250505090565b80518015158114610193575f5ffd5b919050565b5f5f5f5f608085870312156101ab575f5ffd5b845160208601519094506001600160a01b03811681146101c9575f5ffd5b92506101d760408601610184565b91506101e560608601610184565b905092959194509250565b5f82518060208501845e5f920191825250919050565b60805160a05160c05160e05161010051610120516101405161016051614f856103105f395f818161042301528181610b6b015281816113c3015281816118a301528181611f63015281816123c00152818161298401528181612b0f01528181612f50015261318e01525f81816105dc01528181610a2c015281816132f9015261396901525f818161276d015281816132920152613c6501525f818161221b01526135ed01525f81816106f601528181613f420152613f9701525f81816105770152818161106f01528181611f0e01528181613a660152613b3a01525f818161124b0152818161157d01528181611e06015261211401525f8181610878015261229a0152614f855ff3fe608060405234801561000f575f5ffd5b50600436106102e3575f3560e01c80637fa3a40e11610187578063d1ce8da8116100dd578063e78cea9211610093578063edaafe201161006e578063edaafe2014610740578063f1981578146107c9578063f60a5091146107dc575f5ffd5b8063e78cea92146106de578063e8eb1dc3146106f1578063ebea461d14610718575f5ffd5b8063dd44e6e0116100c3578063dd44e6e014610678578063e0bc9729146106a4578063e5a358c8146106b7575f5ffd5b8063d1ce8da814610652578063d9dd67ab14610665575f5ffd5b806392d9f7821161013d578063b31761f811610118578063b31761f814610619578063cb23bcb51461062c578063cc2a1a0c1461063f575f5ffd5b806392d9f782146105d757806396cc5c78146105fe578063a655d93714610606575f5ffd5b80638d910dde1161016d5780638d910dde146105725780638f111f3c146105b1578063917cf8ac146105c4575f5ffd5b80637fa3a40e14610557578063844208601461055f575f5ffd5b80633e5aa0821161023c5780636d46e987116101f25780636f12b0c9116101cd5780636f12b0c9146104cf578063715ea34b146104e257806371c3e6fe14610535575f5ffd5b80636d46e987146104875780636e620055146104a95780636e7df3e7146104bc575f5ffd5b806369cacded1161022257806369cacded146104455780636ae71f12146104585780636c89045014610460575f5ffd5b80633e5aa0821461040b5780634b678a661461041e575f5ffd5b80631f9566321161029c57806327957a491161027757806327957a49146103c95780632cbf74e5146103d15780632f3985a7146103f8575f5ffd5b80631f956632146103905780631ff64790146103a3578063258f0495146103b6575f5ffd5b80631637be48116102cc5780631637be481461034257806316af91a7146103745780631ad87e451461037b575f5ffd5b806302c99275146102e757806306f130561461032c575b5f5ffd5b61030e7f200000000000000000000000000000000000000000000000000000000000000081565b6040516001600160f81b031990911681526020015b60405180910390f35b6103346107e7565b604051908152602001610323565b61036461035036600461444f565b5f9081526008602052604090205460ff1690565b6040519015158152602001610323565b61030e5f81565b61038e610389366004614594565b61086e565b005b61038e61039e3660046145f0565b610b9e565b61038e6103b1366004614627565b610d04565b6103346103c436600461444f565b610ecc565b610334602881565b61030e7f500000000000000000000000000000000000000000000000000000000000000081565b61038e610406366004614642565b610f38565b61038e61041936600461465c565b61106c565b6103647f000000000000000000000000000000000000000000000000000000000000000081565b61038e6104533660046146fd565b61134e565b61038e611682565b61030e7f080000000000000000000000000000000000000000000000000000000000000081565b610364610495366004614627565b60096020525f908152604090205460ff1681565b61038e6104b73660046146fd565b611852565b61038e6104ca3660046145f0565b611901565b61038e6104dd366004614785565b611a67565b6105156104f036600461444f565b60086020525f908152604090205460ff811690610100900467ffffffffffffffff1682565b60408051921515835267ffffffffffffffff909116602083015201610323565b610364610543366004614627565b60036020525f908152604090205460ff1681565b6103345f5481565b61038e61056d36600461444f565b611a99565b6105997f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610323565b61038e6105bf3660046147eb565b611c06565b61038e6105d2366004614862565b611f0b565b6103647f000000000000000000000000000000000000000000000000000000000000000081565b61038e612218565b61038e610614366004614642565b612290565b61038e6106273660046148ba565b61244e565b600254610599906001600160a01b031681565b600b54610599906001600160a01b031681565b61038e61066036600461491d565b6125a7565b61033461067336600461444f565b6128e9565b61068b61068636600461495c565b612973565b60405167ffffffffffffffff9091168152602001610323565b61038e6106b23660046147eb565b6129d4565b61030e7f400000000000000000000000000000000000000000000000000000000000000081565b600154610599906001600160a01b031681565b6103347f000000000000000000000000000000000000000000000000000000000000000081565b610720612a59565b604080519485526020850193909352918301526060820152608001610323565b600c54600d546107869167ffffffffffffffff8082169268010000000000000000808404831693600160801b8104841693600160c01b9091048116928082169290041686565b6040805167ffffffffffffffff978816815295871660208701529386169385019390935290841660608401528316608083015290911660a082015260c001610323565b61038e6107d7366004614985565b612a8e565b61030e600160ff1b81565b600154604080517e84120c00000000000000000000000000000000000000000000000000000000815290515f926001600160a01b0316916284120c9160048083019260209291908290030181865afa158015610845573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061086991906149e8565b905090565b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016300361092b576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602c60248201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060448201527f64656c656761746563616c6c000000000000000000000000000000000000000060648201526084015b60405180910390fd5b6001546001600160a01b03161561096e576040517fef34ca5c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001600160a01b0383166109ae576040517f1ad0f74300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5f9050836001600160a01b031663e1758bd86040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015610a0c575060408051601f3d908101601f19168201909252610a09918101906149ff565b60015b15610a27576001600160a01b03811615610a2557600191505b505b8015157f0000000000000000000000000000000000000000000000000000000000000000151514610a84576040517fc3e31f8d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b038616908117909155604080517fcb23bcb5000000000000000000000000000000000000000000000000000000008152905163cb23bcb5916004808201926020929091908290030181865afa158015610b01573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b2591906149ff565b6002805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b0392909216919091179055610b69610b64368590038501856148ba565b612e22565b7f000000000000000000000000000000000000000000000000000000000000000015610b9857610b9882612f4e565b50505050565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610bee573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c1291906149ff565b6001600160a01b0316336001600160a01b031614158015610c3e5750600b546001600160a01b03163314155b15610c77576040517f660b3b42000000000000000000000000000000000000000000000000000000008152336004820152602401610922565b6001600160a01b0382165f81815260096020908152604091829020805460ff19168515159081179091558251938452908301527feb12a9a53eec138c91b27b4f912a257bd690c18fc8bde744be92a0365eb9b87e910160405180910390a16040516004907fea8787f128d10b2cc0317b0c3960f9ad447f7f6c1ed189db1083ccffd20f456e905f90a25050565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610d54573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d7891906149ff565b6001600160a01b0316336001600160a01b031614610e405760025460408051638da5cb5b60e01b8152905133926001600160a01b031691638da5cb5b9160048083019260209291908290030181865afa158015610dd7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dfb91906149ff565b6040517f23295f0e0000000000000000000000000000000000000000000000000000000081526001600160a01b03928316600482015291166024820152604401610922565b600b805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b0383169081179091556040519081527f3cd6c184800297a0f2b00926a683cbe76890bb7fd01480ac0a10ed6c8f7f66599060200160405180910390a16040516005907fea8787f128d10b2cc0317b0c3960f9ad447f7f6c1ed189db1083ccffd20f456e905f90a250565b5f81815260086020908152604080832081518083019092525460ff811615158252610100900467ffffffffffffffff16918101829052908203610f245760405162f20c5d60e01b815260048101849052602401610922565b6020015167ffffffffffffffff1692915050565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f88573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fac91906149ff565b6001600160a01b0316336001600160a01b03161461100b5760025460408051638da5cb5b60e01b8152905133926001600160a01b031691638da5cb5b9160048083019260209291908290030181865afa158015610dd7573d5f5f3e3d5ffd5b61101481612f4e565b60408051825167ffffffffffffffff908116825260208085015182169083015283830151168183015290517faa7a2d8175dee3b637814ad6346005dfcc357165396fb8327f649effe8abcf859181900360600190a150565b827f00000000000000000000000000000000000000000000000000000000000000005f5a335f9081526003602052604090205490915060ff166110c257604051632dd9fc9760e01b815260040160405180910390fd5b6110cb8761318b565b156110e957604051630e5da8fb60e01b815260040160405180910390fd5b6110f5888887876131d1565b6001600160a01b0383161561134457365f602061111383601f614a2e565b61111d9190614a41565b905061020061112d600283614b3b565b6111379190614a41565b611142826006614b49565b61114c9190614a2e565b6111569084614a2e565b925061116061332b565b61116c575f915061129a565b6001600160a01b0384161561129a57836001600160a01b031663e83a2d826040518163ffffffff1660e01b81526004015f60405180830381865afa9250505080156111d857506040513d5f823e601f3d908101601f191682016040526111d59190810190614b60565b60015b1561129a57805115611298575f856001600160a01b0316631f6d6ef76040518163ffffffff1660e01b8152600401602060405180830381865afa158015611221573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061124591906149e8565b905048817f000000000000000000000000000000000000000000000000000000000000000084516112769190614b49565b6112809190614b49565b61128a9190614a41565b6112949086614a2e565b9450505b505b846001600160a01b031663e3db8a49335a6112b59087614c01565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e085901b1681526001600160a01b0390921660048301526024820152604481018590526064016020604051808303815f875af115801561131c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113409190614c14565b5050505b5050505050505050565b835f5f5a905061135c61332b565b611392576040517fc8958ead00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526003602052604090205460ff166113c157604051632dd9fc9760e01b815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006113ff57604051631192b39960e31b815260040160405180910390fd5b6114178861141236879003870187614c2f565b61333d565b6114278b8b8b8b8a8a6001613445565b6001600160a01b0383161561134057365f602061144583601f614a2e565b61144f9190614a41565b905061020061145f600283614b3b565b6114699190614a41565b611474826006614b49565b61147e9190614a2e565b6114889084614a2e565b925061149261332b565b61149e575f91506115cc565b6001600160a01b038416156115cc57836001600160a01b031663e83a2d826040518163ffffffff1660e01b81526004015f60405180830381865afa92505050801561150a57506040513d5f823e601f3d908101601f191682016040526115079190810190614b60565b60015b156115cc578051156115ca575f856001600160a01b0316631f6d6ef76040518163ffffffff1660e01b8152600401602060405180830381865afa158015611553573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061157791906149e8565b905048817f000000000000000000000000000000000000000000000000000000000000000084516115a89190614b49565b6115b29190614b49565b6115bc9190614a41565b6115c69086614a2e565b9450505b505b846001600160a01b031663e3db8a49335a6115e79087614c01565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e085901b1681526001600160a01b0390921660048301526024820152604481018590526064016020604051808303815f875af115801561164e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116729190614c14565b5050505050505050505050505050565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116f691906149ff565b6001600160a01b0316336001600160a01b0316146117555760025460408051638da5cb5b60e01b8152905133926001600160a01b031691638da5cb5b9160048083019260209291908290030181865afa158015610dd7573d5f5f3e3d5ffd5b600154604080517fcb23bcb500000000000000000000000000000000000000000000000000000000815290515f926001600160a01b03169163cb23bcb59160048083019260209291908290030181865afa1580156117b5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117d991906149ff565b6002549091506001600160a01b03808316911603611823576040517fd054909f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6002805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b0392909216919091179055565b835f5f5a335f9081526003602052604090205490915060ff1615801561188357506002546001600160a01b03163314155b156118a157604051632dd9fc9760e01b815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006118df57604051631192b39960e31b815260040160405180910390fd5b6118f28861141236879003870187614c2f565b6114278b8b8b8b8a8a5f613445565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611951573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061197591906149ff565b6001600160a01b0316336001600160a01b0316141580156119a15750600b546001600160a01b03163314155b156119da576040517f660b3b42000000000000000000000000000000000000000000000000000000008152336004820152602401610922565b6001600160a01b0382165f81815260036020908152604091829020805460ff19168515159081179091558251938452908301527f28bcc5626d357efe966b4b0876aa1ee8ab99e26da4f131f6a2623f1800701c21910160405180910390a16040516001907fea8787f128d10b2cc0317b0c3960f9ad447f7f6c1ed189db1083ccffd20f456e905f90a25050565b6040517fc73b9d7c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611ae9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b0d91906149ff565b6001600160a01b0316336001600160a01b031614611b6c5760025460408051638da5cb5b60e01b8152905133926001600160a01b031691638da5cb5b9160048083019260209291908290030181865afa158015610dd7573d5f5f3e3d5ffd5b5f8181526008602052604090205460ff16611b9c5760405162f20c5d60e01b815260048101829052602401610922565b5f81815260086020526040808220805460ff191690555182917f5cb4218b272fd214168ac43e90fb4d05d6c36f0b17ffb4c2dd07c234d744eb2a91a26040516003907fea8787f128d10b2cc0317b0c3960f9ad447f7f6c1ed189db1083ccffd20f456e905f90a250565b825f5f5a9050611c1461332b565b611c4a576040517fc8958ead00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526003602052604090205460ff16611c7957604051632dd9fc9760e01b815260040160405180910390fd5b611c828761318b565b15611ca057604051630e5da8fb60e01b815260040160405180910390fd5b611cb08a8a8a8a89896001613445565b6001600160a01b03831615611eff57365f6020611cce83601f614a2e565b611cd89190614a41565b9050610200611ce8600283614b3b565b611cf29190614a41565b611cfd826006614b49565b611d079190614a2e565b611d119084614a2e565b9250611d1b61332b565b611d27575f9150611e55565b6001600160a01b03841615611e5557836001600160a01b031663e83a2d826040518163ffffffff1660e01b81526004015f60405180830381865afa925050508015611d9357506040513d5f823e601f3d908101601f19168201604052611d909190810190614b60565b60015b15611e5557805115611e53575f856001600160a01b0316631f6d6ef76040518163ffffffff1660e01b8152600401602060405180830381865afa158015611ddc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e0091906149e8565b905048817f00000000000000000000000000000000000000000000000000000000000000008451611e319190614b49565b611e3b9190614b49565b611e459190614a41565b611e4f9086614a2e565b9450505b505b846001600160a01b031663e3db8a49335a611e709087614c01565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e085901b1681526001600160a01b0390921660048301526024820152604481018590526064016020604051808303815f875af1158015611ed7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611efb9190614c14565b5050505b50505050505050505050565b837f00000000000000000000000000000000000000000000000000000000000000005f5a335f9081526003602052604090205490915060ff16611f6157604051632dd9fc9760e01b815260040160405180910390fd5b7f0000000000000000000000000000000000000000000000000000000000000000611f9f57604051631192b39960e31b815260040160405180910390fd5b611fb28861141236879003870187614c2f565b611fbe898988886131d1565b6001600160a01b0383161561220d57365f6020611fdc83601f614a2e565b611fe69190614a41565b9050610200611ff6600283614b3b565b6120009190614a41565b61200b826006614b49565b6120159190614a2e565b61201f9084614a2e565b925061202961332b565b612035575f9150612163565b6001600160a01b0384161561216357836001600160a01b031663e83a2d826040518163ffffffff1660e01b81526004015f60405180830381865afa9250505080156120a157506040513d5f823e601f3d908101601f1916820160405261209e9190810190614b60565b60015b1561216357805115612161575f856001600160a01b0316631f6d6ef76040518163ffffffff1660e01b8152600401602060405180830381865afa1580156120ea573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061210e91906149e8565b905048817f0000000000000000000000000000000000000000000000000000000000000000845161213f9190614b49565b6121499190614b49565b6121539190614a41565b61215d9086614a2e565b9450505b505b846001600160a01b031663e3db8a49335a61217e9087614c01565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e085901b1681526001600160a01b0390921660048301526024820152604481018590526064016020604051808303815f875af11580156121e5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122099190614c14565b5050505b505050505050505050565b467f000000000000000000000000000000000000000000000000000000000000000003612271576040517fa301bb0600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7801000000000000000100000000000000010000000000000001600a55565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163003612348576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602c60248201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060448201527f64656c656761746563616c6c00000000000000000000000000000000000000006064820152608401610922565b7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61038054336001600160a01b038216146123be576040517f23295f0e0000000000000000000000000000000000000000000000000000000081523360048201526001600160a01b0382166024820152604401610922565b7f00000000000000000000000000000000000000000000000000000000000000006123fc57604051631192b39960e31b815260040160405180910390fd5b600c5467ffffffffffffffff1615612440576040517fef34ca5c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61244983612f4e565b505050565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561249e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124c291906149ff565b6001600160a01b0316336001600160a01b0316146125215760025460408051638da5cb5b60e01b8152905133926001600160a01b031691638da5cb5b9160048083019260209291908290030181865afa158015610dd7573d5f5f3e3d5ffd5b61252a81612e22565b60408051825181526020808401519082015282820151818301526060808401519082015290517faa6a58dad31128ff7ecc2b80987ee6e003df80bc50cd8d0b0d1af0e07da6d19d9181900360800190a16040515f907fea8787f128d10b2cc0317b0c3960f9ad447f7f6c1ed189db1083ccffd20f456e908290a250565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156125f7573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061261b91906149ff565b6001600160a01b0316336001600160a01b03161461267a5760025460408051638da5cb5b60e01b8152905133926001600160a01b031691638da5cb5b9160048083019260209291908290030181865afa158015610dd7573d5f5f3e3d5ffd5b5f828260405161268b929190614cdd565b6040519081900381207ffe000000000000000000000000000000000000000000000000000000000000006020830152602182015260410160408051601f1981840301815291905280516020909101209050600160ff1b811862010000831061271f576040517fb3d1f41200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8181526008602052604090205460ff161561276a576040517ffa2fddda00000000000000000000000000000000000000000000000000000000815260048101829052602401610922565b437f0000000000000000000000000000000000000000000000000000000000000000156127f55760646001600160a01b031663a3b1b31d6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156127ce573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127f291906149e8565b90505b6040805180820182526001815267ffffffffffffffff83811660208084019182525f87815260089091528490209251835491517fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000009092169015157fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000ff161761010091909216021790555182907fabca9b7986bc22ad0160eb0cb88ae75411eacfba4052af0b457a9335ef655722906128af9088908890614cec565b60405180910390a26040516002907fea8787f128d10b2cc0317b0c3960f9ad447f7f6c1ed189db1083ccffd20f456e905f90a25050505050565b6001546040517f16bf5579000000000000000000000000000000000000000000000000000000008152600481018390525f916001600160a01b0316906316bf557990602401602060405180830381865afa158015612949573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061296d91906149e8565b92915050565b600a545f9067ffffffffffffffff167f0000000000000000000000000000000000000000000000000000000000000000156129c3575f6129b4600c85613569565b90506129bf816135b7565b9150505b6129cd8184614d1a565b9392505050565b825f5f5a335f9081526003602052604090205490915060ff16158015612a0557506002546001600160a01b03163314155b15612a2357604051632dd9fc9760e01b815260040160405180910390fd5b612a2c8761318b565b15612a4a57604051630e5da8fb60e01b815260040160405180910390fd5b611cb08a8a8a8a89895f613445565b5f5f5f5f5f5f5f5f612a696135e6565b67ffffffffffffffff9384169b50918316995082169750169450505050505b90919293565b5f548611612ac8576040517f7d73e6fa00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f612afd8684612adb602089018961495c565b612aeb60408a0160208b0161495c565b612af660018d614c01565b898861365c565b600a5490915067ffffffffffffffff167f000000000000000000000000000000000000000000000000000000000000000015612b6e57612b4b612b43602088018861495c565b600c90613700565b600c54612b619067ffffffffffffffff166135b7565b67ffffffffffffffff1690505b4381612b7d602089018961495c565b67ffffffffffffffff16612b919190614a2e565b10612bc8576040517fad3515d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6001891115612c4e576001546001600160a01b031663d5719dc2612bee60028c614c01565b6040518263ffffffff1660e01b8152600401612c0c91815260200190565b602060405180830381865afa158015612c27573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c4b91906149e8565b90505b60408051602080820184905281830186905282518083038401815260609092019092528051910120600180546001600160a01b03169063d5719dc290612c94908d614c01565b6040518263ffffffff1660e01b8152600401612cb291815260200190565b602060405180830381865afa158015612ccd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cf191906149e8565b14612d28576040517f13947fd700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5f612d338b613786565b915091505f8b90505f60015f9054906101000a90046001600160a01b03166001600160a01b0316635fca4a166040518163ffffffff1660e01b8152600401602060405180830381865afa158015612d8c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612db091906149e8565b9050805f808080612dc489888388806137c9565b93509350935093508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7855f548d6002604051612e069493929190614d4e565b60405180910390a4505050505050505050505050505050505050565b805167ffffffffffffffff1080612e445750602081015167ffffffffffffffff105b80612e5a5750604081015167ffffffffffffffff105b80612e705750606081015167ffffffffffffffff105b15612ea7576040517f09cfba7500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8051600a80546020840151604085015160609095015167ffffffffffffffff908116600160c01b0277ffffffffffffffffffffffffffffffffffffffffffffffff968216600160801b02969096166fffffffffffffffffffffffffffffffff92821668010000000000000000027fffffffffffffffffffffffffffffffff000000000000000000000000000000009094169190951617919091171691909117919091179055565b7f0000000000000000000000000000000000000000000000000000000000000000612f8c57604051631192b39960e31b815260040160405180910390fd5b612f95816139a7565b612fcb576040517fda1c8eb600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600c5467ffffffffffffffff161580612ff757506020810151600c5467ffffffffffffffff9182169116115b15613023576020810151600c805467ffffffffffffffff191667ffffffffffffffff9092169190911790555b8051600c5467ffffffffffffffff91821691161015613060578051600c805467ffffffffffffffff191667ffffffffffffffff9092169190911790555b602081810151600c805484517fffffffffffffffff00000000000000000000000000000000ffffffffffffffff9091166801000000000000000067ffffffffffffffff948516027fffffffffffffffff0000000000000000ffffffffffffffffffffffffffffffff1617600160801b91841691909102179055604080840151600d805467ffffffffffffffff191691909316179091555f5460015482517feca067ad000000000000000000000000000000000000000000000000000000008152925191936001600160a01b039091169263eca067ad92600480830193928290030181865afa158015613154573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061317891906149e8565b0361318857613188600c43613700565b50565b5f7f000000000000000000000000000000000000000000000000000000000000000080156131b957505f5482115b801561296d57506131ca600c613a0e565b1592915050565b5f5f5f6131dd86613a40565b9250925092505f5f5f5f6131f4878b5f8c8c6137c9565b93509350935093508a841415801561320d57505f198b14155b1561324e576040517fac7411c900000000000000000000000000000000000000000000000000000000815260048101859052602481018c9052604401610922565b80838c7f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7855f548b60036040516132889493929190614d4e565b60405180910390a47f0000000000000000000000000000000000000000000000000000000000000000156132e8576040517f86657a5300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6132f061332b565b801561331a57507f0000000000000000000000000000000000000000000000000000000000000000155b156113405761134087854888613c62565b5f3332148015610869575050333b1590565b5f5482111561344157613350600c613e9c565b15613441576001545f80546040517fd5719dc200000000000000000000000000000000000000000000000000000000815291926001600160a01b03169163d5719dc2916133a39160040190815260200190565b602060405180830381865afa1580156133be573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133e291906149e8565b90506133f681835f01518460200151613ecc565b61342c576040517fc334542d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60208201516040015161244990600c90613700565b5050565b5f5f613452888888613f10565b915091505f5f5f5f613473868b8961346a575f61346c565b8d5b8c8c6137c9565b93509350935093508c841415801561348c57505f198d14155b156134cd576040517fac7411c900000000000000000000000000000000000000000000000000000000815260048101859052602481018e9052604401610922565b8083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7855f548a8d613501576001613503565b5f5b6040516135139493929190614d4e565b60405180910390a486611efb57837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d8d604051613552929190614cec565b60405180910390a250505050505050505050505050565b815460018301545f916129cd9167ffffffffffffffff600160c01b8304811692868216928282169268010000000000000000808304821693600160801b810483169391900482169116614118565b600a545f9067ffffffffffffffff908116908316106135e257600a5467ffffffffffffffff1661296d565b5090565b5f808080467f00000000000000000000000000000000000000000000000000000000000000001461362257506001925082915081905080612a88565b5050600a5467ffffffffffffffff808216935068010000000000000000820481169250600160801b8204811691600160c01b900416612a88565b6040516001600160f81b031960f889901b1660208201526bffffffffffffffffffffffff19606088901b1660218201527fffffffffffffffff00000000000000000000000000000000000000000000000060c087811b8216603584015286901b16603d8201526045810184905260658101839052608581018290525f9060a5016040516020818303038152906040528051906020012090505b979650505050505050565b61370a8282613569565b825467ffffffffffffffff928316600160c01b0277ffffffffffffffffffffffffffffffff000000000000000090911691831691909117178255600190910180544390921668010000000000000000027fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff909216919091179055565b604080516080810182525f80825260208201819052918101829052606081018290525f5f6137b3856141d9565b8151602090920191909120969095509350505050565b5f5f5f5f5f54881015613808576040517f7d73e6fa00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60015f9054906101000a90046001600160a01b03166001600160a01b031663eca067ad6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613858573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061387c91906149e8565b8811156138b5576040517f925f8bd300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001546040517f86598a56000000000000000000000000000000000000000000000000000000008152600481018b9052602481018a905260448101889052606481018790526001600160a01b03909116906386598a56906084016080604051808303815f875af115801561392b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061394f9190614dd6565b5f8c905592965090945092509050861580159061398a57507f0000000000000000000000000000000000000000000000000000000000000000155b1561399b5761399b8985485f613c62565b95509550955095915050565b80515f9067ffffffffffffffff16158015906139d05750602082015167ffffffffffffffff1615155b80156139ec5750612710826040015167ffffffffffffffff1611155b801561296d5750506020810151905167ffffffffffffffff9182169116111590565b80545f9067ffffffffffffffff600160801b8204811691613a3891600160c01b9091041643614c01565b111592915050565b604080516080810182525f80825260208201819052918101829052606081018290525f5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e83a2d826040518163ffffffff1660e01b81526004015f60405180830381865afa158015613abf573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613ae69190810190614b60565b905080515f03613b22576040517f3cd27eb600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5f613b2d876141d9565b915091505f8351620200007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316631f6d6ef76040518163ffffffff1660e01b8152600401602060405180830381865afa158015613b94573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613bb891906149e8565b613bc29190614b49565b613bcc9190614b49565b60405190915083907f500000000000000000000000000000000000000000000000000000000000000090613c04908790602001614e09565b60408051601f1981840301815290829052613c23939291602001614e55565b60405160208183030381529060405280519060200120825f4811613c47575f613c51565b613c514884614a41565b965096509650505050509193909250565b327f000000000000000000000000000000000000000000000000000000000000000015613d05575f606c6001600160a01b031663c6f7de0e6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613cc7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613ceb91906149e8565b9050613cf74882614a41565b613d019084614a2e565b9250505b67ffffffffffffffff821115613d47576040517f04d5501200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b604080514260208201526bffffffffffffffffffffffff19606084901b16918101919091526054810186905260748101859052609481018490527fffffffffffffffff00000000000000000000000000000000000000000000000060c084901b1660b48201525f9060bc0160408051808303601f1901815290829052600154815160208301207f7a88b1070000000000000000000000000000000000000000000000000000000084526001600160a01b03868116600486015260248501919091529193505f92911690637a88b107906044016020604051808303815f875af1158015613e35573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e5991906149e8565b9050807fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b83604051613e8b9190614e84565b60405180910390a250505050505050565b5f613ea682613a0e565b158061296d5750505467ffffffffffffffff680100000000000000008204811691161090565b5f613f0683613eda846142ae565b604080516020808201949094528082019290925280518083038201815260609092019052805191012090565b9093149392505050565b604080516080810182525f80825260208201819052918101829052606081018290525f613f3e856028614a2e565b90507f0000000000000000000000000000000000000000000000000000000000000000811115613fc3576040517f4634691b000000000000000000000000000000000000000000000000000000008152600481018290527f00000000000000000000000000000000000000000000000000000000000000006024820152604401610922565b5f5f613fce866141d9565b909250905086156140dc57613ffd88885f818110613fee57613fee614d3a565b9050013560f81c60f81b6142d9565b6140545787875f81811061401357614013614d3a565b6040517f6b3333560000000000000000000000000000000000000000000000000000000081529201356001600160f81b031916600483015250602401610922565b600160ff1b88885f8161406957614069614d3a565b6001600160f81b03199201359290921616158015915061408a575060218710155b156140dc575f61409e602160018a8c614eb9565b6140a791614ee0565b5f8181526008602052604090205490915060ff166140da5760405162f20c5d60e01b815260048101829052602401610922565b505b8188886040516020016140f193929190614efd565b6040516020818303038152906040528051906020012081945094505050505b935093915050565b5f5f888811614127575f614131565b6141318989614c01565b90505f898711614141575f61414b565b61414b8a88614c01565b905061271061415a8584614b49565b6141649190614a41565b61416e9089614a2e565b97505f86821161417e575f614188565b6141888783614c01565b9050828111156141955750815b808911156141ca576141a7818a614c01565b9850868911156141ca578589116141be57886141c0565b855b93505050506136f5565b50949998505050505050505050565b604080516080810182525f8082526020820181905291810182905260608082018390529161420561436b565b90505f815f01518260200151836040015184606001518860405160200161428395949392919060c095861b7fffffffffffffffff000000000000000000000000000000000000000000000000908116825294861b8516600882015292851b8416601084015290841b8316601883015290921b16602082015260280190565b604051602081830303815290604052905060288151146142a5576142a5614f1b565b94909350915050565b5f61296d825f015183602001518460400151856060015186608001518760a001518860c0015161365c565b5f6001600160f81b0319821615806142fe57506001600160f81b03198216600160ff1b145b8061433257506001600160f81b031982167f8800000000000000000000000000000000000000000000000000000000000000145b8061296d57506001600160f81b031982167f20000000000000000000000000000000000000000000000000000000000000001492915050565b604080516080810182525f808252602082018190529181018290526060810191909152604080516080810182525f8082526020820181905291810182905260608101919091525f5f5f5f6143bd6135e6565b93509350935093508167ffffffffffffffff164211156143ee576143e18242614f2f565b67ffffffffffffffff1685525b6143f88142614d1a565b67ffffffffffffffff9081166020870152841643111561442c5761441c8443614f2f565b67ffffffffffffffff1660408601525b6144368343614d1a565b67ffffffffffffffff1660608601525092949350505050565b5f6020828403121561445f575f5ffd5b5035919050565b6001600160a01b0381168114613188575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b6040805190810167ffffffffffffffff811182821017156144b1576144b161447a565b60405290565b60405160e0810167ffffffffffffffff811182821017156144b1576144b161447a565b604051601f8201601f1916810167ffffffffffffffff811182821017156145035761450361447a565b604052919050565b803567ffffffffffffffff81168114614522575f5ffd5b919050565b5f60608284031215614537575f5ffd5b6040516060810167ffffffffffffffff8111828210171561455a5761455a61447a565b6040529050806145698361450b565b81526145776020840161450b565b60208201526145886040840161450b565b60408201525092915050565b5f5f5f8385036101008112156145a8575f5ffd5b84356145b381614466565b93506080601f19820112156145c6575f5ffd5b506020840191506145da8560a08601614527565b90509250925092565b8015158114613188575f5ffd5b5f5f60408385031215614601575f5ffd5b823561460c81614466565b9150602083013561461c816145e3565b809150509250929050565b5f60208284031215614637575f5ffd5b81356129cd81614466565b5f60608284031215614652575f5ffd5b6129cd8383614527565b5f5f5f5f5f60a08688031215614670575f5ffd5b8535945060208601359350604086013561468981614466565b94979396509394606081013594506080013592915050565b5f5f83601f8401126146b1575f5ffd5b50813567ffffffffffffffff8111156146c8575f5ffd5b6020830191508360208285010111156146df575f5ffd5b9250929050565b5f61010082840312156146f7575f5ffd5b50919050565b5f5f5f5f5f5f5f5f6101c0898b031215614715575f5ffd5b88359750602089013567ffffffffffffffff811115614732575f5ffd5b61473e8b828c016146a1565b90985096505060408901359450606089013561475981614466565b93506080890135925060a089013591506147768a60c08b016146e6565b90509295985092959890939650565b5f5f5f5f5f60808688031215614799575f5ffd5b85359450602086013567ffffffffffffffff8111156147b6575f5ffd5b6147c2888289016146a1565b9095509350506040860135915060608601356147dd81614466565b809150509295509295909350565b5f5f5f5f5f5f5f60c0888a031215614801575f5ffd5b87359650602088013567ffffffffffffffff81111561481e575f5ffd5b61482a8a828b016146a1565b90975095505060408801359350606088013561484581614466565b969995985093969295946080840135945060a09093013592915050565b5f5f5f5f5f5f6101a08789031215614878575f5ffd5b8635955060208701359450604087013561489181614466565b935060608701359250608087013591506148ae8860a089016146e6565b90509295509295509295565b5f60808284031280156148cb575f5ffd5b506040516080810167ffffffffffffffff811182821017156148ef576148ef61447a565b6040908152833582526020808501359083015283810135908201526060928301359281019290925250919050565b5f5f6020838503121561492e575f5ffd5b823567ffffffffffffffff811115614944575f5ffd5b614950858286016146a1565b90969095509350505050565b5f6020828403121561496c575f5ffd5b6129cd8261450b565b803560ff81168114614522575f5ffd5b5f5f5f5f5f5f60e0878903121561499a575f5ffd5b863595506149aa60208801614975565b945060808701888111156149bc575f5ffd5b60408801945035925060a08701356149d381614466565b9598949750929591949360c090920135925050565b5f602082840312156149f8575f5ffd5b5051919050565b5f60208284031215614a0f575f5ffd5b81516129cd81614466565b634e487b7160e01b5f52601160045260245ffd5b8082018082111561296d5761296d614a1a565b5f82614a5b57634e487b7160e01b5f52601260045260245ffd5b500490565b6001815b600184111561411057808504811115614a7f57614a7f614a1a565b6001841615614a8d57908102905b60019390931c928002614a64565b5f82614aa95750600161296d565b81614ab557505f61296d565b8160018114614acb5760028114614ad557614af1565b600191505061296d565b60ff841115614ae657614ae6614a1a565b50506001821b61296d565b5060208310610133831016604e8410600b8410161715614b14575081810a61296d565b614b205f198484614a60565b805f1904821115614b3357614b33614a1a565b029392505050565b5f6129cd60ff841683614a9b565b808202811582820484141761296d5761296d614a1a565b5f60208284031215614b70575f5ffd5b815167ffffffffffffffff811115614b86575f5ffd5b8201601f81018413614b96575f5ffd5b805167ffffffffffffffff811115614bb057614bb061447a565b8060051b614bc0602082016144da565b91825260208184018101929081019087841115614bdb575f5ffd5b6020850194505b838510156136f557845180835260209586019590935090910190614be2565b8181038181111561296d5761296d614a1a565b5f60208284031215614c24575f5ffd5b81516129cd816145e3565b5f81830361010081128015614c42575f5ffd5b50614c4b61448e565b8335815260e0601f1983011215614c60575f5ffd5b614c686144b7565b9150614c7660208501614975565b82526040840135614c8681614466565b6020830152614c976060850161450b565b6040830152614ca86080850161450b565b606083015260a084810135608084015260c0808601359184019190915260e09094013593820193909352602083015250919050565b818382375f9101908152919050565b60208152816020820152818360408301375f818301604090810191909152601f909201601f19160101919050565b67ffffffffffffffff818116838216019081111561296d5761296d614a1a565b634e487b7160e01b5f52603260045260245ffd5b5f60e08201905085825284602083015267ffffffffffffffff845116604083015267ffffffffffffffff602085015116606083015267ffffffffffffffff604085015116608083015267ffffffffffffffff60608501511660a083015260048310614dc757634e487b7160e01b5f52602160045260245ffd5b8260c083015295945050505050565b5f5f5f5f60808587031215614de9575f5ffd5b505082516020840151604085015160609095015191969095509092509050565b81515f90829060208501835b82811015614e33578151845260209384019390910190600101614e15565b509195945050505050565b5f81518060208401855e5f93019283525090919050565b5f614e608286614e3e565b6001600160f81b031985168152614e7a6001820185614e3e565b9695505050505050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b5f5f85851115614ec7575f5ffd5b83861115614ed3575f5ffd5b5050820193919092039150565b8035602083101561296d575f19602084900360031b1b1692915050565b5f614f088286614e3e565b838582375f930192835250909392505050565b634e487b7160e01b5f52600160045260245ffd5b67ffffffffffffffff828116828216039081111561296d5761296d614a1a56fea264697066735822122038e99cf4eb42b1bd091544a0482c80ae5681aaff1e73e5b858eba14b32894aee64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x80`@R0`\x80Rb\x02\0\0`\xA0RFa\x01\0Ra\0\x1Da\0\xF0V[\x15\x15a\x01 R4\x80\x15a\0.W__\xFD[P`@QaR\x958\x03\x80aR\x95\x839\x81\x01`@\x81\x90Ra\0M\x91a\x01\x98V[`\xE0\x84\x90Ra\x01 Q\x15a\0\x88W`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\0\x83W`@Qc\x86ezS`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\0\xD0V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\0\xD0W`@Qc\x80\xFC,\x03`\xE0\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x14\x99XY\x19\\\x8D\x0E\r\r`\xB2\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\xC0R\x15\x15a\x01@R\x15\x15a\x01`RPa\x02\x06V[`@\x80Q`\x04\x81R`$\x81\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x02\x88\x1Cy`\xE1\x1B\x17\x90R\x90Q_\x91\x82\x91\x82\x91`d\x91a\x01-\x91\x90a\x01\xF0V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a\x01eW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x01jV[``\x91P[P\x91P\x91P\x81\x80\x15a\x01}WP\x80Q` \x14[\x92PPP\x90V[\x80Q\x80\x15\x15\x81\x14a\x01\x93W__\xFD[\x91\x90PV[____`\x80\x85\x87\x03\x12\x15a\x01\xABW__\xFD[\x84Q` \x86\x01Q\x90\x94P`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xC9W__\xFD[\x92Pa\x01\xD7`@\x86\x01a\x01\x84V[\x91Pa\x01\xE5``\x86\x01a\x01\x84V[\x90P\x92\x95\x91\x94P\x92PV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`QaO\x85a\x03\x10_9_\x81\x81a\x04#\x01R\x81\x81a\x0Bk\x01R\x81\x81a\x13\xC3\x01R\x81\x81a\x18\xA3\x01R\x81\x81a\x1Fc\x01R\x81\x81a#\xC0\x01R\x81\x81a)\x84\x01R\x81\x81a+\x0F\x01R\x81\x81a/P\x01Ra1\x8E\x01R_\x81\x81a\x05\xDC\x01R\x81\x81a\n,\x01R\x81\x81a2\xF9\x01Ra9i\x01R_\x81\x81a'm\x01R\x81\x81a2\x92\x01Ra<e\x01R_\x81\x81a\"\x1B\x01Ra5\xED\x01R_\x81\x81a\x06\xF6\x01R\x81\x81a?B\x01Ra?\x97\x01R_\x81\x81a\x05w\x01R\x81\x81a\x10o\x01R\x81\x81a\x1F\x0E\x01R\x81\x81a:f\x01Ra;:\x01R_\x81\x81a\x12K\x01R\x81\x81a\x15}\x01R\x81\x81a\x1E\x06\x01Ra!\x14\x01R_\x81\x81a\x08x\x01Ra\"\x9A\x01RaO\x85_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\xE3W_5`\xE0\x1C\x80c\x7F\xA3\xA4\x0E\x11a\x01\x87W\x80c\xD1\xCE\x8D\xA8\x11a\0\xDDW\x80c\xE7\x8C\xEA\x92\x11a\0\x93W\x80c\xED\xAA\xFE \x11a\0nW\x80c\xED\xAA\xFE \x14a\x07@W\x80c\xF1\x98\x15x\x14a\x07\xC9W\x80c\xF6\nP\x91\x14a\x07\xDCW__\xFD[\x80c\xE7\x8C\xEA\x92\x14a\x06\xDEW\x80c\xE8\xEB\x1D\xC3\x14a\x06\xF1W\x80c\xEB\xEAF\x1D\x14a\x07\x18W__\xFD[\x80c\xDDD\xE6\xE0\x11a\0\xC3W\x80c\xDDD\xE6\xE0\x14a\x06xW\x80c\xE0\xBC\x97)\x14a\x06\xA4W\x80c\xE5\xA3X\xC8\x14a\x06\xB7W__\xFD[\x80c\xD1\xCE\x8D\xA8\x14a\x06RW\x80c\xD9\xDDg\xAB\x14a\x06eW__\xFD[\x80c\x92\xD9\xF7\x82\x11a\x01=W\x80c\xB3\x17a\xF8\x11a\x01\x18W\x80c\xB3\x17a\xF8\x14a\x06\x19W\x80c\xCB#\xBC\xB5\x14a\x06,W\x80c\xCC*\x1A\x0C\x14a\x06?W__\xFD[\x80c\x92\xD9\xF7\x82\x14a\x05\xD7W\x80c\x96\xCC\\x\x14a\x05\xFEW\x80c\xA6U\xD97\x14a\x06\x06W__\xFD[\x80c\x8D\x91\r\xDE\x11a\x01mW\x80c\x8D\x91\r\xDE\x14a\x05rW\x80c\x8F\x11\x1F<\x14a\x05\xB1W\x80c\x91|\xF8\xAC\x14a\x05\xC4W__\xFD[\x80c\x7F\xA3\xA4\x0E\x14a\x05WW\x80c\x84B\x08`\x14a\x05_W__\xFD[\x80c>Z\xA0\x82\x11a\x02<W\x80cmF\xE9\x87\x11a\x01\xF2W\x80co\x12\xB0\xC9\x11a\x01\xCDW\x80co\x12\xB0\xC9\x14a\x04\xCFW\x80cq^\xA3K\x14a\x04\xE2W\x80cq\xC3\xE6\xFE\x14a\x055W__\xFD[\x80cmF\xE9\x87\x14a\x04\x87W\x80cnb\0U\x14a\x04\xA9W\x80cn}\xF3\xE7\x14a\x04\xBCW__\xFD[\x80ci\xCA\xCD\xED\x11a\x02\"W\x80ci\xCA\xCD\xED\x14a\x04EW\x80cj\xE7\x1F\x12\x14a\x04XW\x80cl\x89\x04P\x14a\x04`W__\xFD[\x80c>Z\xA0\x82\x14a\x04\x0BW\x80cKg\x8Af\x14a\x04\x1EW__\xFD[\x80c\x1F\x95f2\x11a\x02\x9CW\x80c'\x95zI\x11a\x02wW\x80c'\x95zI\x14a\x03\xC9W\x80c,\xBFt\xE5\x14a\x03\xD1W\x80c/9\x85\xA7\x14a\x03\xF8W__\xFD[\x80c\x1F\x95f2\x14a\x03\x90W\x80c\x1F\xF6G\x90\x14a\x03\xA3W\x80c%\x8F\x04\x95\x14a\x03\xB6W__\xFD[\x80c\x167\xBEH\x11a\x02\xCCW\x80c\x167\xBEH\x14a\x03BW\x80c\x16\xAF\x91\xA7\x14a\x03tW\x80c\x1A\xD8~E\x14a\x03{W__\xFD[\x80c\x02\xC9\x92u\x14a\x02\xE7W\x80c\x06\xF10V\x14a\x03,W[__\xFD[a\x03\x0E\x7F \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x034a\x07\xE7V[`@Q\x90\x81R` \x01a\x03#V[a\x03da\x03P6`\x04aDOV[_\x90\x81R`\x08` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x03#V[a\x03\x0E_\x81V[a\x03\x8Ea\x03\x896`\x04aE\x94V[a\x08nV[\0[a\x03\x8Ea\x03\x9E6`\x04aE\xF0V[a\x0B\x9EV[a\x03\x8Ea\x03\xB16`\x04aF'V[a\r\x04V[a\x034a\x03\xC46`\x04aDOV[a\x0E\xCCV[a\x034`(\x81V[a\x03\x0E\x7FP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x8Ea\x04\x066`\x04aFBV[a\x0F8V[a\x03\x8Ea\x04\x196`\x04aF\\V[a\x10lV[a\x03d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x8Ea\x04S6`\x04aF\xFDV[a\x13NV[a\x03\x8Ea\x16\x82V[a\x03\x0E\x7F\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03da\x04\x956`\x04aF'V[`\t` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\x8Ea\x04\xB76`\x04aF\xFDV[a\x18RV[a\x03\x8Ea\x04\xCA6`\x04aE\xF0V[a\x19\x01V[a\x03\x8Ea\x04\xDD6`\x04aG\x85V[a\x1AgV[a\x05\x15a\x04\xF06`\x04aDOV[`\x08` R_\x90\x81R`@\x90 T`\xFF\x81\x16\x90a\x01\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82V[`@\x80Q\x92\x15\x15\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x03#V[a\x03da\x05C6`\x04aF'V[`\x03` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x034_T\x81V[a\x03\x8Ea\x05m6`\x04aDOV[a\x1A\x99V[a\x05\x99\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03#V[a\x03\x8Ea\x05\xBF6`\x04aG\xEBV[a\x1C\x06V[a\x03\x8Ea\x05\xD26`\x04aHbV[a\x1F\x0BV[a\x03d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x8Ea\"\x18V[a\x03\x8Ea\x06\x146`\x04aFBV[a\"\x90V[a\x03\x8Ea\x06'6`\x04aH\xBAV[a$NV[`\x02Ta\x05\x99\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x0BTa\x05\x99\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x8Ea\x06`6`\x04aI\x1DV[a%\xA7V[a\x034a\x06s6`\x04aDOV[a(\xE9V[a\x06\x8Ba\x06\x866`\x04aI\\V[a)sV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03#V[a\x03\x8Ea\x06\xB26`\x04aG\xEBV[a)\xD4V[a\x03\x0E\x7F@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01Ta\x05\x99\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x034\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x07 a*YV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x03#V[`\x0CT`\rTa\x07\x86\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x92h\x01\0\0\0\0\0\0\0\0\x80\x84\x04\x83\x16\x93`\x01`\x80\x1B\x81\x04\x84\x16\x93`\x01`\xC0\x1B\x90\x91\x04\x81\x16\x92\x80\x82\x16\x92\x90\x04\x16\x86V[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16\x81R\x95\x87\x16` \x87\x01R\x93\x86\x16\x93\x85\x01\x93\x90\x93R\x90\x84\x16``\x84\x01R\x83\x16`\x80\x83\x01R\x90\x91\x16`\xA0\x82\x01R`\xC0\x01a\x03#V[a\x03\x8Ea\x07\xD76`\x04aI\x85V[a*\x8EV[a\x03\x0E`\x01`\xFF\x1B\x81V[`\x01T`@\x80Q~\x84\x12\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91b\x84\x12\x0C\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x08EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08i\x91\x90aI\xE8V[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\t+W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\tnW`@Q\x7F\xEF4\xCA\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16a\t\xAEW`@Q\x7F\x1A\xD0\xF7C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16c\xE1u\x8B\xD8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\n\x0CWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\n\t\x91\x81\x01\x90aI\xFFV[`\x01[\x15a\n'W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\n%W`\x01\x91P[P[\x80\x15\x15\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15\x15\x14a\n\x84W`@Q\x7F\xC3\xE3\x1F\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x81\x17\x90\x91U`@\x80Q\x7F\xCB#\xBC\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qc\xCB#\xBC\xB5\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0B\x01W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B%\x91\x90aI\xFFV[`\x02\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x0Bia\x0Bd6\x85\x90\x03\x85\x01\x85aH\xBAV[a.\"V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0B\x98Wa\x0B\x98\x82a/NV[PPPPV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x12\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x0C>WP`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x0CwW`@Q\x7Ff\x0B;B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x01a\t\"V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\t` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x7F\xEB\x12\xA9\xA5>\xEC\x13\x8C\x91\xB2{O\x91*%{\xD6\x90\xC1\x8F\xC8\xBD\xE7D\xBE\x92\xA06^\xB9\xB8~\x91\x01`@Q\x80\x91\x03\x90\xA1`@Q`\x04\x90\x7F\xEA\x87\x87\xF1(\xD1\x0B,\xC01{\x0C9`\xF9\xADD\x7F\x7Fl\x1E\xD1\x89\xDB\x10\x83\xCC\xFF\xD2\x0FEn\x90_\x90\xA2PPV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rTW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rx\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E@W`\x02T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xFB\x91\x90aI\xFFV[`@Q\x7F#)_\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\t\"V[`\x0B\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F<\xD6\xC1\x84\x80\x02\x97\xA0\xF2\xB0\t&\xA6\x83\xCB\xE7h\x90\xBB\x7F\xD0\x14\x80\xAC\n\x10\xEDl\x8F\x7FfY\x90` \x01`@Q\x80\x91\x03\x90\xA1`@Q`\x05\x90\x7F\xEA\x87\x87\xF1(\xD1\x0B,\xC01{\x0C9`\xF9\xADD\x7F\x7Fl\x1E\xD1\x89\xDB\x10\x83\xCC\xFF\xD2\x0FEn\x90_\x90\xA2PV[_\x81\x81R`\x08` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92RT`\xFF\x81\x16\x15\x15\x82Ra\x01\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x90\x82\x03a\x0F$W`@Qb\xF2\x0C]`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\t\"V[` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xAC\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10\x0BW`\x02T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\xD7W=__>=_\xFD[a\x10\x14\x81a/NV[`@\x80Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x82\x16\x90\x83\x01R\x83\x83\x01Q\x16\x81\x83\x01R\x90Q\x7F\xAAz-\x81u\xDE\xE3\xB67\x81J\xD64`\x05\xDF\xCC5qe9o\xB82\x7Fd\x9E\xFF\xE8\xAB\xCF\x85\x91\x81\x90\x03``\x01\x90\xA1PV[\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Z3_\x90\x81R`\x03` R`@\x90 T\x90\x91P`\xFF\x16a\x10\xC2W`@Qc-\xD9\xFC\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xCB\x87a1\x8BV[\x15a\x10\xE9W`@Qc\x0E]\xA8\xFB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xF5\x88\x88\x87\x87a1\xD1V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x13DW6_` a\x11\x13\x83`\x1FaJ.V[a\x11\x1D\x91\x90aJAV[\x90Pa\x02\0a\x11-`\x02\x83aK;V[a\x117\x91\x90aJAV[a\x11B\x82`\x06aKIV[a\x11L\x91\x90aJ.V[a\x11V\x90\x84aJ.V[\x92Pa\x11`a3+V[a\x11lW_\x91Pa\x12\x9AV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x12\x9AW\x83`\x01`\x01`\xA0\x1B\x03\x16c\xE8:-\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x11\xD8WP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\xD5\x91\x90\x81\x01\x90aK`V[`\x01[\x15a\x12\x9AW\x80Q\x15a\x12\x98W_\x85`\x01`\x01`\xA0\x1B\x03\x16c\x1Fmn\xF7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12E\x91\x90aI\xE8V[\x90PH\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84Qa\x12v\x91\x90aKIV[a\x12\x80\x91\x90aKIV[a\x12\x8A\x91\x90aJAV[a\x12\x94\x90\x86aJ.V[\x94PP[P[\x84`\x01`\x01`\xA0\x1B\x03\x16c\xE3\xDB\x8AI3Za\x12\xB5\x90\x87aL\x01V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x81\x01\x85\x90R`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13@\x91\x90aL\x14V[PPP[PPPPPPPPV[\x83__Z\x90Pa\x13\\a3+V[a\x13\x92W`@Q\x7F\xC8\x95\x8E\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x03` R`@\x90 T`\xFF\x16a\x13\xC1W`@Qc-\xD9\xFC\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13\xFFW`@Qc\x11\x92\xB3\x99`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\x17\x88a\x14\x126\x87\x90\x03\x87\x01\x87aL/V[a3=V[a\x14'\x8B\x8B\x8B\x8B\x8A\x8A`\x01a4EV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x13@W6_` a\x14E\x83`\x1FaJ.V[a\x14O\x91\x90aJAV[\x90Pa\x02\0a\x14_`\x02\x83aK;V[a\x14i\x91\x90aJAV[a\x14t\x82`\x06aKIV[a\x14~\x91\x90aJ.V[a\x14\x88\x90\x84aJ.V[\x92Pa\x14\x92a3+V[a\x14\x9EW_\x91Pa\x15\xCCV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x15\xCCW\x83`\x01`\x01`\xA0\x1B\x03\x16c\xE8:-\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x15\nWP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x15\x07\x91\x90\x81\x01\x90aK`V[`\x01[\x15a\x15\xCCW\x80Q\x15a\x15\xCAW_\x85`\x01`\x01`\xA0\x1B\x03\x16c\x1Fmn\xF7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15SW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15w\x91\x90aI\xE8V[\x90PH\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84Qa\x15\xA8\x91\x90aKIV[a\x15\xB2\x91\x90aKIV[a\x15\xBC\x91\x90aJAV[a\x15\xC6\x90\x86aJ.V[\x94PP[P[\x84`\x01`\x01`\xA0\x1B\x03\x16c\xE3\xDB\x8AI3Za\x15\xE7\x90\x87aL\x01V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x81\x01\x85\x90R`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16NW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16r\x91\x90aL\x14V[PPPPPPPPPPPPPPV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xF6\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17UW`\x02T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\xD7W=__>=_\xFD[`\x01T`@\x80Q\x7F\xCB#\xBC\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xCB#\xBC\xB5\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x17\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xD9\x91\x90aI\xFFV[`\x02T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x03a\x18#W`@Q\x7F\xD0T\x90\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x83__Z3_\x90\x81R`\x03` R`@\x90 T\x90\x91P`\xFF\x16\x15\x80\x15a\x18\x83WP`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x18\xA1W`@Qc-\xD9\xFC\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x18\xDFW`@Qc\x11\x92\xB3\x99`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\xF2\x88a\x14\x126\x87\x90\x03\x87\x01\x87aL/V[a\x14'\x8B\x8B\x8B\x8B\x8A\x8A_a4EV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19QW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19u\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x19\xA1WP`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x19\xDAW`@Q\x7Ff\x0B;B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x01a\t\"V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x7F(\xBC\xC5bm5~\xFE\x96kK\x08v\xAA\x1E\xE8\xAB\x99\xE2m\xA4\xF11\xF6\xA2b?\x18\0p\x1C!\x91\x01`@Q\x80\x91\x03\x90\xA1`@Q`\x01\x90\x7F\xEA\x87\x87\xF1(\xD1\x0B,\xC01{\x0C9`\xF9\xADD\x7F\x7Fl\x1E\xD1\x89\xDB\x10\x83\xCC\xFF\xD2\x0FEn\x90_\x90\xA2PPV[`@Q\x7F\xC7;\x9D|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xE9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\r\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1BlW`\x02T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\xD7W=__>=_\xFD[_\x81\x81R`\x08` R`@\x90 T`\xFF\x16a\x1B\x9CW`@Qb\xF2\x0C]`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\"V[_\x81\x81R`\x08` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x82\x91\x7F\\\xB4!\x8B'/\xD2\x14\x16\x8A\xC4>\x90\xFBM\x05\xD6\xC3o\x0B\x17\xFF\xB4\xC2\xDD\x07\xC24\xD7D\xEB*\x91\xA2`@Q`\x03\x90\x7F\xEA\x87\x87\xF1(\xD1\x0B,\xC01{\x0C9`\xF9\xADD\x7F\x7Fl\x1E\xD1\x89\xDB\x10\x83\xCC\xFF\xD2\x0FEn\x90_\x90\xA2PV[\x82__Z\x90Pa\x1C\x14a3+V[a\x1CJW`@Q\x7F\xC8\x95\x8E\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x03` R`@\x90 T`\xFF\x16a\x1CyW`@Qc-\xD9\xFC\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\x82\x87a1\x8BV[\x15a\x1C\xA0W`@Qc\x0E]\xA8\xFB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\xB0\x8A\x8A\x8A\x8A\x89\x89`\x01a4EV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x1E\xFFW6_` a\x1C\xCE\x83`\x1FaJ.V[a\x1C\xD8\x91\x90aJAV[\x90Pa\x02\0a\x1C\xE8`\x02\x83aK;V[a\x1C\xF2\x91\x90aJAV[a\x1C\xFD\x82`\x06aKIV[a\x1D\x07\x91\x90aJ.V[a\x1D\x11\x90\x84aJ.V[\x92Pa\x1D\x1Ba3+V[a\x1D'W_\x91Pa\x1EUV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x1EUW\x83`\x01`\x01`\xA0\x1B\x03\x16c\xE8:-\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1D\x93WP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D\x90\x91\x90\x81\x01\x90aK`V[`\x01[\x15a\x1EUW\x80Q\x15a\x1ESW_\x85`\x01`\x01`\xA0\x1B\x03\x16c\x1Fmn\xF7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xDCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\0\x91\x90aI\xE8V[\x90PH\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84Qa\x1E1\x91\x90aKIV[a\x1E;\x91\x90aKIV[a\x1EE\x91\x90aJAV[a\x1EO\x90\x86aJ.V[\x94PP[P[\x84`\x01`\x01`\xA0\x1B\x03\x16c\xE3\xDB\x8AI3Za\x1Ep\x90\x87aL\x01V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x81\x01\x85\x90R`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xFB\x91\x90aL\x14V[PPP[PPPPPPPPPPV[\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Z3_\x90\x81R`\x03` R`@\x90 T\x90\x91P`\xFF\x16a\x1FaW`@Qc-\xD9\xFC\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1F\x9FW`@Qc\x11\x92\xB3\x99`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\xB2\x88a\x14\x126\x87\x90\x03\x87\x01\x87aL/V[a\x1F\xBE\x89\x89\x88\x88a1\xD1V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\"\rW6_` a\x1F\xDC\x83`\x1FaJ.V[a\x1F\xE6\x91\x90aJAV[\x90Pa\x02\0a\x1F\xF6`\x02\x83aK;V[a \0\x91\x90aJAV[a \x0B\x82`\x06aKIV[a \x15\x91\x90aJ.V[a \x1F\x90\x84aJ.V[\x92Pa )a3+V[a 5W_\x91Pa!cV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a!cW\x83`\x01`\x01`\xA0\x1B\x03\x16c\xE8:-\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a \xA1WP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra \x9E\x91\x90\x81\x01\x90aK`V[`\x01[\x15a!cW\x80Q\x15a!aW_\x85`\x01`\x01`\xA0\x1B\x03\x16c\x1Fmn\xF7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xEAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x0E\x91\x90aI\xE8V[\x90PH\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84Qa!?\x91\x90aKIV[a!I\x91\x90aKIV[a!S\x91\x90aJAV[a!]\x90\x86aJ.V[\x94PP[P[\x84`\x01`\x01`\xA0\x1B\x03\x16c\xE3\xDB\x8AI3Za!~\x90\x87aL\x01V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x81\x01\x85\x90R`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\xE5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\t\x91\x90aL\x14V[PPP[PPPPPPPPPV[F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a\"qW`@Q\x7F\xA3\x01\xBB\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[x\x01\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01`\nUV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a#HW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\"V[\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03\x80T3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a#\xBEW`@Q\x7F#)_\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`D\x01a\t\"V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#\xFCW`@Qc\x11\x92\xB3\x99`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a$@W`@Q\x7F\xEF4\xCA\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$I\x83a/NV[PPPV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xC2\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a%!W`\x02T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\xD7W=__>=_\xFD[a%*\x81a.\"V[`@\x80Q\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R\x82\x82\x01Q\x81\x83\x01R``\x80\x84\x01Q\x90\x82\x01R\x90Q\x7F\xAAjX\xDA\xD3\x11(\xFF~\xCC+\x80\x98~\xE6\xE0\x03\xDF\x80\xBCP\xCD\x8D\x0B\r\x1A\xF0\xE0}\xA6\xD1\x9D\x91\x81\x90\x03`\x80\x01\x90\xA1`@Q_\x90\x7F\xEA\x87\x87\xF1(\xD1\x0B,\xC01{\x0C9`\xF9\xADD\x7F\x7Fl\x1E\xD1\x89\xDB\x10\x83\xCC\xFF\xD2\x0FEn\x90\x82\x90\xA2PV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xF7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x1B\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a&zW`\x02T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\xD7W=__>=_\xFD[_\x82\x82`@Qa&\x8B\x92\x91\x90aL\xDDV[`@Q\x90\x81\x90\x03\x81 \x7F\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`!\x82\x01R`A\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P`\x01`\xFF\x1B\x81\x18b\x01\0\0\x83\x10a'\x1FW`@Q\x7F\xB3\xD1\xF4\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x08` R`@\x90 T`\xFF\x16\x15a'jW`@Q\x7F\xFA/\xDD\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\"V[C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a'\xF5W`d`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xB1\xB3\x1D`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xF2\x91\x90aI\xE8V[\x90P[`@\x80Q\x80\x82\x01\x82R`\x01\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16` \x80\x84\x01\x91\x82R_\x87\x81R`\x08\x90\x91R\x84\x90 \x92Q\x83T\x91Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x90\x92\x16\x90\x15\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\x16\x17a\x01\0\x91\x90\x92\x16\x02\x17\x90UQ\x82\x90\x7F\xAB\xCA\x9By\x86\xBC\"\xAD\x01`\xEB\x0C\xB8\x8A\xE7T\x11\xEA\xCF\xBA@R\xAF\x0BEz\x935\xEFeW\"\x90a(\xAF\x90\x88\x90\x88\x90aL\xECV[`@Q\x80\x91\x03\x90\xA2`@Q`\x02\x90\x7F\xEA\x87\x87\xF1(\xD1\x0B,\xC01{\x0C9`\xF9\xADD\x7F\x7Fl\x1E\xD1\x89\xDB\x10\x83\xCC\xFF\xD2\x0FEn\x90_\x90\xA2PPPPPV[`\x01T`@Q\x7F\x16\xBFUy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x16\xBFUy\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)IW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)m\x91\x90aI\xE8V[\x92\x91PPV[`\nT_\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a)\xC3W_a)\xB4`\x0C\x85a5iV[\x90Pa)\xBF\x81a5\xB7V[\x91PP[a)\xCD\x81\x84aM\x1AV[\x93\x92PPPV[\x82__Z3_\x90\x81R`\x03` R`@\x90 T\x90\x91P`\xFF\x16\x15\x80\x15a*\x05WP`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a*#W`@Qc-\xD9\xFC\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a*,\x87a1\x8BV[\x15a*JW`@Qc\x0E]\xA8\xFB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\xB0\x8A\x8A\x8A\x8A\x89\x89_a4EV[________a*ia5\xE6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x9BP\x91\x83\x16\x99P\x82\x16\x97P\x16\x94PPPPP[\x90\x91\x92\x93V[_T\x86\x11a*\xC8W`@Q\x7F}s\xE6\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a*\xFD\x86\x84a*\xDB` \x89\x01\x89aI\\V[a*\xEB`@\x8A\x01` \x8B\x01aI\\V[a*\xF6`\x01\x8DaL\x01V[\x89\x88a6\\V[`\nT\x90\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a+nWa+Ka+C` \x88\x01\x88aI\\V[`\x0C\x90a7\0V[`\x0CTa+a\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a5\xB7V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[C\x81a+}` \x89\x01\x89aI\\V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a+\x91\x91\x90aJ.V[\x10a+\xC8W`@Q\x7F\xAD5\x15\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\x01\x89\x11\x15a,NW`\x01T`\x01`\x01`\xA0\x1B\x03\x16c\xD5q\x9D\xC2a+\xEE`\x02\x8CaL\x01V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\x0C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,K\x91\x90aI\xE8V[\x90P[`@\x80Q` \x80\x82\x01\x84\x90R\x81\x83\x01\x86\x90R\x82Q\x80\x83\x03\x84\x01\x81R``\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD5q\x9D\xC2\x90a,\x94\x90\x8DaL\x01V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xB2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xCDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xF1\x91\x90aI\xE8V[\x14a-(W`@Q\x7F\x13\x94\x7F\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a-3\x8Ba7\x86V[\x91P\x91P_\x8B\x90P_`\x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c_\xCAJ\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\x8CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xB0\x91\x90aI\xE8V[\x90P\x80_\x80\x80\x80a-\xC4\x89\x88\x83\x88\x80a7\xC9V[\x93P\x93P\x93P\x93P\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85_T\x8D`\x02`@Qa.\x06\x94\x93\x92\x91\x90aMNV[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPPPPPPPPPPV[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10\x80a.DWP` \x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10[\x80a.ZWP`@\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10[\x80a.pWP``\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10[\x15a.\xA7W`@Q\x7F\t\xCF\xBAu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\n\x80T` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x01`\xC0\x1B\x02w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96\x82\x16`\x01`\x80\x1B\x02\x96\x90\x96\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x82\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x91\x90\x95\x16\x17\x91\x90\x91\x17\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90UV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a/\x8CW`@Qc\x11\x92\xB3\x99`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a/\x95\x81a9\xA7V[a/\xCBW`@Q\x7F\xDA\x1C\x8E\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80a/\xF7WP` \x81\x01Q`\x0CTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x11[\x15a0#W` \x81\x01Q`\x0C\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[\x80Q`\x0CTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x10\x15a0`W\x80Q`\x0C\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[` \x81\x81\x01Q`\x0C\x80T\x84Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16h\x01\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17`\x01`\x80\x1B\x91\x84\x16\x91\x90\x91\x02\x17\x90U`@\x80\x84\x01Q`\r\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x93\x16\x17\x90\x91U_T`\x01T\x82Q\x7F\xEC\xA0g\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92Q\x91\x93`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c\xEC\xA0g\xAD\x92`\x04\x80\x83\x01\x93\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a1TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1x\x91\x90aI\xE8V[\x03a1\x88Wa1\x88`\x0CCa7\0V[PV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x15a1\xB9WP_T\x82\x11[\x80\x15a)mWPa1\xCA`\x0Ca:\x0EV[\x15\x92\x91PPV[___a1\xDD\x86a:@V[\x92P\x92P\x92P____a1\xF4\x87\x8B_\x8C\x8Ca7\xC9V[\x93P\x93P\x93P\x93P\x8A\x84\x14\x15\x80\x15a2\rWP_\x19\x8B\x14\x15[\x15a2NW`@Q\x7F\xACt\x11\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x8C\x90R`D\x01a\t\"V[\x80\x83\x8C\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85_T\x8B`\x03`@Qa2\x88\x94\x93\x92\x91\x90aMNV[`@Q\x80\x91\x03\x90\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a2\xE8W`@Q\x7F\x86ezS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a2\xF0a3+V[\x80\x15a3\x1AWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15[\x15a\x13@Wa\x13@\x87\x85H\x88a<bV[_32\x14\x80\x15a\x08iWPP3;\x15\x90V[_T\x82\x11\x15a4AWa3P`\x0Ca>\x9CV[\x15a4AW`\x01T_\x80T`@Q\x7F\xD5q\x9D\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xD5q\x9D\xC2\x91a3\xA3\x91`\x04\x01\x90\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xBEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xE2\x91\x90aI\xE8V[\x90Pa3\xF6\x81\x83_\x01Q\x84` \x01Qa>\xCCV[a4,W`@Q\x7F\xC34T-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Q`@\x01Qa$I\x90`\x0C\x90a7\0V[PPV[__a4R\x88\x88\x88a?\x10V[\x91P\x91P____a4s\x86\x8B\x89a4jW_a4lV[\x8D[\x8C\x8Ca7\xC9V[\x93P\x93P\x93P\x93P\x8C\x84\x14\x15\x80\x15a4\x8CWP_\x19\x8D\x14\x15[\x15a4\xCDW`@Q\x7F\xACt\x11\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x8E\x90R`D\x01a\t\"V[\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85_T\x8A\x8Da5\x01W`\x01a5\x03V[_[`@Qa5\x13\x94\x93\x92\x91\x90aMNV[`@Q\x80\x91\x03\x90\xA4\x86a\x1E\xFBW\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D\x8D`@Qa5R\x92\x91\x90aL\xECV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPV[\x81T`\x01\x83\x01T_\x91a)\xCD\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x83\x04\x81\x16\x92\x86\x82\x16\x92\x82\x82\x16\x92h\x01\0\0\0\0\0\0\0\0\x80\x83\x04\x82\x16\x93`\x01`\x80\x1B\x81\x04\x83\x16\x93\x91\x90\x04\x82\x16\x91\x16aA\x18V[`\nT_\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x83\x16\x10a5\xE2W`\nTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a)mV[P\x90V[_\x80\x80\x80F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a6\"WP`\x01\x92P\x82\x91P\x81\x90P\x80a*\x88V[PP`\nTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x93Ph\x01\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x92P`\x01`\x80\x1B\x82\x04\x81\x16\x91`\x01`\xC0\x1B\x90\x04\x16a*\x88V[`@Q`\x01`\x01`\xF8\x1B\x03\x19`\xF8\x89\x90\x1B\x16` \x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x88\x90\x1B\x16`!\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x87\x81\x1B\x82\x16`5\x84\x01R\x86\x90\x1B\x16`=\x82\x01R`E\x81\x01\x84\x90R`e\x81\x01\x83\x90R`\x85\x81\x01\x82\x90R_\x90`\xA5\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x97\x96PPPPPPPV[a7\n\x82\x82a5iV[\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x01`\xC0\x1B\x02w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x90\x91\x16\x91\x83\x16\x91\x90\x91\x17\x17\x82U`\x01\x90\x91\x01\x80TC\x90\x92\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R__a7\xB3\x85aA\xD9V[\x81Q` \x90\x92\x01\x91\x90\x91 \x96\x90\x95P\x93PPPPV[_____T\x88\x10\x15a8\x08W`@Q\x7F}s\xE6\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEC\xA0g\xAD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8XW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8|\x91\x90aI\xE8V[\x88\x11\x15a8\xB5W`@Q\x7F\x92_\x8B\xD3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@Q\x7F\x86Y\x8AV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8B\x90R`$\x81\x01\x8A\x90R`D\x81\x01\x88\x90R`d\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x86Y\x8AV\x90`\x84\x01`\x80`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a9+W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9O\x91\x90aM\xD6V[_\x8C\x90U\x92\x96P\x90\x94P\x92P\x90P\x86\x15\x80\x15\x90a9\x8AWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15[\x15a9\x9BWa9\x9B\x89\x85H_a<bV[\x95P\x95P\x95P\x95\x91PPV[\x80Q_\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a9\xD0WP` \x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15[\x80\x15a9\xECWPa'\x10\x82`@\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15[\x80\x15a)mWPP` \x81\x01Q\x90Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x11\x15\x90V[\x80T_\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x91a:8\x91`\x01`\xC0\x1B\x90\x91\x04\x16CaL\x01V[\x11\x15\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R__\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE8:-\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\xBFW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra:\xE6\x91\x90\x81\x01\x90aK`V[\x90P\x80Q_\x03a;\"W`@Q\x7F<\xD2~\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a;-\x87aA\xD9V[\x91P\x91P_\x83Qb\x02\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x1Fmn\xF7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xB8\x91\x90aI\xE8V[a;\xC2\x91\x90aKIV[a;\xCC\x91\x90aKIV[`@Q\x90\x91P\x83\x90\x7FP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a<\x04\x90\x87\x90` \x01aN\tV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra<#\x93\x92\x91` \x01aNUV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82_H\x11a<GW_a<QV[a<QH\x84aJAV[\x96P\x96P\x96PPPPP\x91\x93\x90\x92PV[2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a=\x05W_`l`\x01`\x01`\xA0\x1B\x03\x16c\xC6\xF7\xDE\x0E`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xEB\x91\x90aI\xE8V[\x90Pa<\xF7H\x82aJAV[a=\x01\x90\x84aJ.V[\x92PP[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a=GW`@Q\x7F\x04\xD5P\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80QB` \x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x90\x1B\x16\x91\x81\x01\x91\x90\x91R`T\x81\x01\x86\x90R`t\x81\x01\x85\x90R`\x94\x81\x01\x84\x90R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x84\x90\x1B\x16`\xB4\x82\x01R_\x90`\xBC\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R`\x01T\x81Q` \x83\x01 \x7Fz\x88\xB1\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x86\x01R`$\x85\x01\x91\x90\x91R\x91\x93P_\x92\x91\x16\x90cz\x88\xB1\x07\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a>5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>Y\x91\x90aI\xE8V[\x90P\x80\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x83`@Qa>\x8B\x91\x90aN\x84V[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[_a>\xA6\x82a:\x0EV[\x15\x80a)mWPPTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\x01\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91\x16\x10\x90V[_a?\x06\x83a>\xDA\x84aB\xAEV[`@\x80Q` \x80\x82\x01\x94\x90\x94R\x80\x82\x01\x92\x90\x92R\x80Q\x80\x83\x03\x82\x01\x81R``\x90\x92\x01\x90R\x80Q\x91\x01 \x90V[\x90\x93\x14\x93\x92PPPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R_a?>\x85`(aJ.V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a?\xC3W`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R`D\x01a\t\"V[__a?\xCE\x86aA\xD9V[\x90\x92P\x90P\x86\x15a@\xDCWa?\xFD\x88\x88_\x81\x81\x10a?\xEEWa?\xEEaM:V[\x90P\x015`\xF8\x1C`\xF8\x1BaB\xD9V[a@TW\x87\x87_\x81\x81\x10a@\x13Wa@\x13aM:V[`@Q\x7Fk33V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x015`\x01`\x01`\xF8\x1B\x03\x19\x16`\x04\x83\x01RP`$\x01a\t\"V[`\x01`\xFF\x1B\x88\x88_\x81a@iWa@iaM:V[`\x01`\x01`\xF8\x1B\x03\x19\x92\x015\x92\x90\x92\x16\x16\x15\x80\x15\x91Pa@\x8AWP`!\x87\x10\x15[\x15a@\xDCW_a@\x9E`!`\x01\x8A\x8CaN\xB9V[a@\xA7\x91aN\xE0V[_\x81\x81R`\x08` R`@\x90 T\x90\x91P`\xFF\x16a@\xDAW`@Qb\xF2\x0C]`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\"V[P[\x81\x88\x88`@Q` \x01a@\xF1\x93\x92\x91\x90aN\xFDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81\x94P\x94PPPP[\x93P\x93\x91PPV[__\x88\x88\x11aA'W_aA1V[aA1\x89\x89aL\x01V[\x90P_\x89\x87\x11aAAW_aAKV[aAK\x8A\x88aL\x01V[\x90Pa'\x10aAZ\x85\x84aKIV[aAd\x91\x90aJAV[aAn\x90\x89aJ.V[\x97P_\x86\x82\x11aA~W_aA\x88V[aA\x88\x87\x83aL\x01V[\x90P\x82\x81\x11\x15aA\x95WP\x81[\x80\x89\x11\x15aA\xCAWaA\xA7\x81\x8AaL\x01V[\x98P\x86\x89\x11\x15aA\xCAW\x85\x89\x11aA\xBEW\x88aA\xC0V[\x85[\x93PPPPa6\xF5V[P\x94\x99\x98PPPPPPPPPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x80\x82\x01\x83\x90R\x91aB\x05aCkV[\x90P_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x88`@Q` \x01aB\x83\x95\x94\x93\x92\x91\x90`\xC0\x95\x86\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x82R\x94\x86\x1B\x85\x16`\x08\x82\x01R\x92\x85\x1B\x84\x16`\x10\x84\x01R\x90\x84\x1B\x83\x16`\x18\x83\x01R\x90\x92\x1B\x16` \x82\x01R`(\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14aB\xA5WaB\xA5aO\x1BV[\x94\x90\x93P\x91PPV[_a)m\x82_\x01Q\x83` \x01Q\x84`@\x01Q\x85``\x01Q\x86`\x80\x01Q\x87`\xA0\x01Q\x88`\xC0\x01Qa6\\V[_`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x15\x80aB\xFEWP`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\x01`\xFF\x1B\x14[\x80aC2WP`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x7F\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14[\x80a)mWP`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x7F \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R____aC\xBDa5\xE6V[\x93P\x93P\x93P\x93P\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15aC\xEEWaC\xE1\x82BaO/V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85R[aC\xF8\x81BaM\x1AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16` \x87\x01R\x84\x16C\x11\x15aD,WaD\x1C\x84CaO/V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x86\x01R[aD6\x83CaM\x1AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x86\x01RP\x92\x94\x93PPPPV[_` \x82\x84\x03\x12\x15aD_W__\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a1\x88W__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aD\xB1WaD\xB1aDzV[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aD\xB1WaD\xB1aDzV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aE\x03WaE\x03aDzV[`@R\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aE\"W__\xFD[\x91\x90PV[_``\x82\x84\x03\x12\x15aE7W__\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aEZWaEZaDzV[`@R\x90P\x80aEi\x83aE\x0BV[\x81RaEw` \x84\x01aE\x0BV[` \x82\x01RaE\x88`@\x84\x01aE\x0BV[`@\x82\x01RP\x92\x91PPV[___\x83\x85\x03a\x01\0\x81\x12\x15aE\xA8W__\xFD[\x845aE\xB3\x81aDfV[\x93P`\x80`\x1F\x19\x82\x01\x12\x15aE\xC6W__\xFD[P` \x84\x01\x91PaE\xDA\x85`\xA0\x86\x01aE'V[\x90P\x92P\x92P\x92V[\x80\x15\x15\x81\x14a1\x88W__\xFD[__`@\x83\x85\x03\x12\x15aF\x01W__\xFD[\x825aF\x0C\x81aDfV[\x91P` \x83\x015aF\x1C\x81aE\xE3V[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15aF7W__\xFD[\x815a)\xCD\x81aDfV[_``\x82\x84\x03\x12\x15aFRW__\xFD[a)\xCD\x83\x83aE'V[_____`\xA0\x86\x88\x03\x12\x15aFpW__\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015aF\x89\x81aDfV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[__\x83`\x1F\x84\x01\x12aF\xB1W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF\xC8W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aF\xDFW__\xFD[\x92P\x92\x90PV[_a\x01\0\x82\x84\x03\x12\x15aF\xF7W__\xFD[P\x91\x90PV[________a\x01\xC0\x89\x8B\x03\x12\x15aG\x15W__\xFD[\x885\x97P` \x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG2W__\xFD[aG>\x8B\x82\x8C\x01aF\xA1V[\x90\x98P\x96PP`@\x89\x015\x94P``\x89\x015aGY\x81aDfV[\x93P`\x80\x89\x015\x92P`\xA0\x89\x015\x91PaGv\x8A`\xC0\x8B\x01aF\xE6V[\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_____`\x80\x86\x88\x03\x12\x15aG\x99W__\xFD[\x855\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\xB6W__\xFD[aG\xC2\x88\x82\x89\x01aF\xA1V[\x90\x95P\x93PP`@\x86\x015\x91P``\x86\x015aG\xDD\x81aDfV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_______`\xC0\x88\x8A\x03\x12\x15aH\x01W__\xFD[\x875\x96P` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\x1EW__\xFD[aH*\x8A\x82\x8B\x01aF\xA1V[\x90\x97P\x95PP`@\x88\x015\x93P``\x88\x015aHE\x81aDfV[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\x80\x84\x015\x94P`\xA0\x90\x93\x015\x92\x91PPV[______a\x01\xA0\x87\x89\x03\x12\x15aHxW__\xFD[\x865\x95P` \x87\x015\x94P`@\x87\x015aH\x91\x81aDfV[\x93P``\x87\x015\x92P`\x80\x87\x015\x91PaH\xAE\x88`\xA0\x89\x01aF\xE6V[\x90P\x92\x95P\x92\x95P\x92\x95V[_`\x80\x82\x84\x03\x12\x80\x15aH\xCBW__\xFD[P`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aH\xEFWaH\xEFaDzV[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x83\x81\x015\x90\x82\x01R``\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[__` \x83\x85\x03\x12\x15aI.W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aIDW__\xFD[aIP\x85\x82\x86\x01aF\xA1V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15aIlW__\xFD[a)\xCD\x82aE\x0BV[\x805`\xFF\x81\x16\x81\x14aE\"W__\xFD[______`\xE0\x87\x89\x03\x12\x15aI\x9AW__\xFD[\x865\x95PaI\xAA` \x88\x01aIuV[\x94P`\x80\x87\x01\x88\x81\x11\x15aI\xBCW__\xFD[`@\x88\x01\x94P5\x92P`\xA0\x87\x015aI\xD3\x81aDfV[\x95\x98\x94\x97P\x92\x95\x91\x94\x93`\xC0\x90\x92\x015\x92PPV[_` \x82\x84\x03\x12\x15aI\xF8W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15aJ\x0FW__\xFD[\x81Qa)\xCD\x81aDfV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a)mWa)maJ\x1AV[_\x82aJ[WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01\x81[`\x01\x84\x11\x15aA\x10W\x80\x85\x04\x81\x11\x15aJ\x7FWaJ\x7FaJ\x1AV[`\x01\x84\x16\x15aJ\x8DW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aJdV[_\x82aJ\xA9WP`\x01a)mV[\x81aJ\xB5WP_a)mV[\x81`\x01\x81\x14aJ\xCBW`\x02\x81\x14aJ\xD5WaJ\xF1V[`\x01\x91PPa)mV[`\xFF\x84\x11\x15aJ\xE6WaJ\xE6aJ\x1AV[PP`\x01\x82\x1Ba)mV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aK\x14WP\x81\x81\na)mV[aK _\x19\x84\x84aJ`V[\x80_\x19\x04\x82\x11\x15aK3WaK3aJ\x1AV[\x02\x93\x92PPPV[_a)\xCD`\xFF\x84\x16\x83aJ\x9BV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a)mWa)maJ\x1AV[_` \x82\x84\x03\x12\x15aKpW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK\x86W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aK\x96W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK\xB0WaK\xB0aDzV[\x80`\x05\x1BaK\xC0` \x82\x01aD\xDAV[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x87\x84\x11\x15aK\xDBW__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15a6\xF5W\x84Q\x80\x83R` \x95\x86\x01\x95\x90\x93P\x90\x91\x01\x90aK\xE2V[\x81\x81\x03\x81\x81\x11\x15a)mWa)maJ\x1AV[_` \x82\x84\x03\x12\x15aL$W__\xFD[\x81Qa)\xCD\x81aE\xE3V[_\x81\x83\x03a\x01\0\x81\x12\x80\x15aLBW__\xFD[PaLKaD\x8EV[\x835\x81R`\xE0`\x1F\x19\x83\x01\x12\x15aL`W__\xFD[aLhaD\xB7V[\x91PaLv` \x85\x01aIuV[\x82R`@\x84\x015aL\x86\x81aDfV[` \x83\x01RaL\x97``\x85\x01aE\x0BV[`@\x83\x01RaL\xA8`\x80\x85\x01aE\x0BV[``\x83\x01R`\xA0\x84\x81\x015`\x80\x84\x01R`\xC0\x80\x86\x015\x91\x84\x01\x91\x90\x91R`\xE0\x90\x94\x015\x93\x82\x01\x93\x90\x93R` \x83\x01RP\x91\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a)mWa)maJ\x1AV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_`\xE0\x82\x01\x90P\x85\x82R\x84` \x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84Q\x16`@\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x85\x01Q\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x85\x01Q\x16`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x85\x01Q\x16`\xA0\x83\x01R`\x04\x83\x10aM\xC7WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x82`\xC0\x83\x01R\x95\x94PPPPPV[____`\x80\x85\x87\x03\x12\x15aM\xE9W__\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[\x81Q_\x90\x82\x90` \x85\x01\x83[\x82\x81\x10\x15aN3W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aN\x15V[P\x91\x95\x94PPPPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_aN`\x82\x86aN>V[`\x01`\x01`\xF8\x1B\x03\x19\x85\x16\x81RaNz`\x01\x82\x01\x85aN>V[\x96\x95PPPPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[__\x85\x85\x11\x15aN\xC7W__\xFD[\x83\x86\x11\x15aN\xD3W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a)mW_\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[_aO\x08\x82\x86aN>V[\x83\x85\x827_\x93\x01\x92\x83RP\x90\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a)mWa)maJ\x1AV\xFE\xA2dipfsX\"\x12 8\xE9\x9C\xF4\xEBB\xB1\xBD\t\x15D\xA0H,\x80\xAEV\x81\xAA\xFF\x1Es\xE5\xB8X\xEB\xA1K2\x89J\xEEdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106102e3575f3560e01c80637fa3a40e11610187578063d1ce8da8116100dd578063e78cea9211610093578063edaafe201161006e578063edaafe2014610740578063f1981578146107c9578063f60a5091146107dc575f5ffd5b8063e78cea92146106de578063e8eb1dc3146106f1578063ebea461d14610718575f5ffd5b8063dd44e6e0116100c3578063dd44e6e014610678578063e0bc9729146106a4578063e5a358c8146106b7575f5ffd5b8063d1ce8da814610652578063d9dd67ab14610665575f5ffd5b806392d9f7821161013d578063b31761f811610118578063b31761f814610619578063cb23bcb51461062c578063cc2a1a0c1461063f575f5ffd5b806392d9f782146105d757806396cc5c78146105fe578063a655d93714610606575f5ffd5b80638d910dde1161016d5780638d910dde146105725780638f111f3c146105b1578063917cf8ac146105c4575f5ffd5b80637fa3a40e14610557578063844208601461055f575f5ffd5b80633e5aa0821161023c5780636d46e987116101f25780636f12b0c9116101cd5780636f12b0c9146104cf578063715ea34b146104e257806371c3e6fe14610535575f5ffd5b80636d46e987146104875780636e620055146104a95780636e7df3e7146104bc575f5ffd5b806369cacded1161022257806369cacded146104455780636ae71f12146104585780636c89045014610460575f5ffd5b80633e5aa0821461040b5780634b678a661461041e575f5ffd5b80631f9566321161029c57806327957a491161027757806327957a49146103c95780632cbf74e5146103d15780632f3985a7146103f8575f5ffd5b80631f956632146103905780631ff64790146103a3578063258f0495146103b6575f5ffd5b80631637be48116102cc5780631637be481461034257806316af91a7146103745780631ad87e451461037b575f5ffd5b806302c99275146102e757806306f130561461032c575b5f5ffd5b61030e7f200000000000000000000000000000000000000000000000000000000000000081565b6040516001600160f81b031990911681526020015b60405180910390f35b6103346107e7565b604051908152602001610323565b61036461035036600461444f565b5f9081526008602052604090205460ff1690565b6040519015158152602001610323565b61030e5f81565b61038e610389366004614594565b61086e565b005b61038e61039e3660046145f0565b610b9e565b61038e6103b1366004614627565b610d04565b6103346103c436600461444f565b610ecc565b610334602881565b61030e7f500000000000000000000000000000000000000000000000000000000000000081565b61038e610406366004614642565b610f38565b61038e61041936600461465c565b61106c565b6103647f000000000000000000000000000000000000000000000000000000000000000081565b61038e6104533660046146fd565b61134e565b61038e611682565b61030e7f080000000000000000000000000000000000000000000000000000000000000081565b610364610495366004614627565b60096020525f908152604090205460ff1681565b61038e6104b73660046146fd565b611852565b61038e6104ca3660046145f0565b611901565b61038e6104dd366004614785565b611a67565b6105156104f036600461444f565b60086020525f908152604090205460ff811690610100900467ffffffffffffffff1682565b60408051921515835267ffffffffffffffff909116602083015201610323565b610364610543366004614627565b60036020525f908152604090205460ff1681565b6103345f5481565b61038e61056d36600461444f565b611a99565b6105997f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610323565b61038e6105bf3660046147eb565b611c06565b61038e6105d2366004614862565b611f0b565b6103647f000000000000000000000000000000000000000000000000000000000000000081565b61038e612218565b61038e610614366004614642565b612290565b61038e6106273660046148ba565b61244e565b600254610599906001600160a01b031681565b600b54610599906001600160a01b031681565b61038e61066036600461491d565b6125a7565b61033461067336600461444f565b6128e9565b61068b61068636600461495c565b612973565b60405167ffffffffffffffff9091168152602001610323565b61038e6106b23660046147eb565b6129d4565b61030e7f400000000000000000000000000000000000000000000000000000000000000081565b600154610599906001600160a01b031681565b6103347f000000000000000000000000000000000000000000000000000000000000000081565b610720612a59565b604080519485526020850193909352918301526060820152608001610323565b600c54600d546107869167ffffffffffffffff8082169268010000000000000000808404831693600160801b8104841693600160c01b9091048116928082169290041686565b6040805167ffffffffffffffff978816815295871660208701529386169385019390935290841660608401528316608083015290911660a082015260c001610323565b61038e6107d7366004614985565b612a8e565b61030e600160ff1b81565b600154604080517e84120c00000000000000000000000000000000000000000000000000000000815290515f926001600160a01b0316916284120c9160048083019260209291908290030181865afa158015610845573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061086991906149e8565b905090565b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016300361092b576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602c60248201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060448201527f64656c656761746563616c6c000000000000000000000000000000000000000060648201526084015b60405180910390fd5b6001546001600160a01b03161561096e576040517fef34ca5c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001600160a01b0383166109ae576040517f1ad0f74300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5f9050836001600160a01b031663e1758bd86040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015610a0c575060408051601f3d908101601f19168201909252610a09918101906149ff565b60015b15610a27576001600160a01b03811615610a2557600191505b505b8015157f0000000000000000000000000000000000000000000000000000000000000000151514610a84576040517fc3e31f8d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b038616908117909155604080517fcb23bcb5000000000000000000000000000000000000000000000000000000008152905163cb23bcb5916004808201926020929091908290030181865afa158015610b01573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b2591906149ff565b6002805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b0392909216919091179055610b69610b64368590038501856148ba565b612e22565b7f000000000000000000000000000000000000000000000000000000000000000015610b9857610b9882612f4e565b50505050565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610bee573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c1291906149ff565b6001600160a01b0316336001600160a01b031614158015610c3e5750600b546001600160a01b03163314155b15610c77576040517f660b3b42000000000000000000000000000000000000000000000000000000008152336004820152602401610922565b6001600160a01b0382165f81815260096020908152604091829020805460ff19168515159081179091558251938452908301527feb12a9a53eec138c91b27b4f912a257bd690c18fc8bde744be92a0365eb9b87e910160405180910390a16040516004907fea8787f128d10b2cc0317b0c3960f9ad447f7f6c1ed189db1083ccffd20f456e905f90a25050565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610d54573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d7891906149ff565b6001600160a01b0316336001600160a01b031614610e405760025460408051638da5cb5b60e01b8152905133926001600160a01b031691638da5cb5b9160048083019260209291908290030181865afa158015610dd7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dfb91906149ff565b6040517f23295f0e0000000000000000000000000000000000000000000000000000000081526001600160a01b03928316600482015291166024820152604401610922565b600b805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b0383169081179091556040519081527f3cd6c184800297a0f2b00926a683cbe76890bb7fd01480ac0a10ed6c8f7f66599060200160405180910390a16040516005907fea8787f128d10b2cc0317b0c3960f9ad447f7f6c1ed189db1083ccffd20f456e905f90a250565b5f81815260086020908152604080832081518083019092525460ff811615158252610100900467ffffffffffffffff16918101829052908203610f245760405162f20c5d60e01b815260048101849052602401610922565b6020015167ffffffffffffffff1692915050565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f88573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fac91906149ff565b6001600160a01b0316336001600160a01b03161461100b5760025460408051638da5cb5b60e01b8152905133926001600160a01b031691638da5cb5b9160048083019260209291908290030181865afa158015610dd7573d5f5f3e3d5ffd5b61101481612f4e565b60408051825167ffffffffffffffff908116825260208085015182169083015283830151168183015290517faa7a2d8175dee3b637814ad6346005dfcc357165396fb8327f649effe8abcf859181900360600190a150565b827f00000000000000000000000000000000000000000000000000000000000000005f5a335f9081526003602052604090205490915060ff166110c257604051632dd9fc9760e01b815260040160405180910390fd5b6110cb8761318b565b156110e957604051630e5da8fb60e01b815260040160405180910390fd5b6110f5888887876131d1565b6001600160a01b0383161561134457365f602061111383601f614a2e565b61111d9190614a41565b905061020061112d600283614b3b565b6111379190614a41565b611142826006614b49565b61114c9190614a2e565b6111569084614a2e565b925061116061332b565b61116c575f915061129a565b6001600160a01b0384161561129a57836001600160a01b031663e83a2d826040518163ffffffff1660e01b81526004015f60405180830381865afa9250505080156111d857506040513d5f823e601f3d908101601f191682016040526111d59190810190614b60565b60015b1561129a57805115611298575f856001600160a01b0316631f6d6ef76040518163ffffffff1660e01b8152600401602060405180830381865afa158015611221573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061124591906149e8565b905048817f000000000000000000000000000000000000000000000000000000000000000084516112769190614b49565b6112809190614b49565b61128a9190614a41565b6112949086614a2e565b9450505b505b846001600160a01b031663e3db8a49335a6112b59087614c01565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e085901b1681526001600160a01b0390921660048301526024820152604481018590526064016020604051808303815f875af115801561131c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113409190614c14565b5050505b5050505050505050565b835f5f5a905061135c61332b565b611392576040517fc8958ead00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526003602052604090205460ff166113c157604051632dd9fc9760e01b815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006113ff57604051631192b39960e31b815260040160405180910390fd5b6114178861141236879003870187614c2f565b61333d565b6114278b8b8b8b8a8a6001613445565b6001600160a01b0383161561134057365f602061144583601f614a2e565b61144f9190614a41565b905061020061145f600283614b3b565b6114699190614a41565b611474826006614b49565b61147e9190614a2e565b6114889084614a2e565b925061149261332b565b61149e575f91506115cc565b6001600160a01b038416156115cc57836001600160a01b031663e83a2d826040518163ffffffff1660e01b81526004015f60405180830381865afa92505050801561150a57506040513d5f823e601f3d908101601f191682016040526115079190810190614b60565b60015b156115cc578051156115ca575f856001600160a01b0316631f6d6ef76040518163ffffffff1660e01b8152600401602060405180830381865afa158015611553573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061157791906149e8565b905048817f000000000000000000000000000000000000000000000000000000000000000084516115a89190614b49565b6115b29190614b49565b6115bc9190614a41565b6115c69086614a2e565b9450505b505b846001600160a01b031663e3db8a49335a6115e79087614c01565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e085901b1681526001600160a01b0390921660048301526024820152604481018590526064016020604051808303815f875af115801561164e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116729190614c14565b5050505050505050505050505050565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116f691906149ff565b6001600160a01b0316336001600160a01b0316146117555760025460408051638da5cb5b60e01b8152905133926001600160a01b031691638da5cb5b9160048083019260209291908290030181865afa158015610dd7573d5f5f3e3d5ffd5b600154604080517fcb23bcb500000000000000000000000000000000000000000000000000000000815290515f926001600160a01b03169163cb23bcb59160048083019260209291908290030181865afa1580156117b5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117d991906149ff565b6002549091506001600160a01b03808316911603611823576040517fd054909f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6002805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b0392909216919091179055565b835f5f5a335f9081526003602052604090205490915060ff1615801561188357506002546001600160a01b03163314155b156118a157604051632dd9fc9760e01b815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006118df57604051631192b39960e31b815260040160405180910390fd5b6118f28861141236879003870187614c2f565b6114278b8b8b8b8a8a5f613445565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611951573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061197591906149ff565b6001600160a01b0316336001600160a01b0316141580156119a15750600b546001600160a01b03163314155b156119da576040517f660b3b42000000000000000000000000000000000000000000000000000000008152336004820152602401610922565b6001600160a01b0382165f81815260036020908152604091829020805460ff19168515159081179091558251938452908301527f28bcc5626d357efe966b4b0876aa1ee8ab99e26da4f131f6a2623f1800701c21910160405180910390a16040516001907fea8787f128d10b2cc0317b0c3960f9ad447f7f6c1ed189db1083ccffd20f456e905f90a25050565b6040517fc73b9d7c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611ae9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b0d91906149ff565b6001600160a01b0316336001600160a01b031614611b6c5760025460408051638da5cb5b60e01b8152905133926001600160a01b031691638da5cb5b9160048083019260209291908290030181865afa158015610dd7573d5f5f3e3d5ffd5b5f8181526008602052604090205460ff16611b9c5760405162f20c5d60e01b815260048101829052602401610922565b5f81815260086020526040808220805460ff191690555182917f5cb4218b272fd214168ac43e90fb4d05d6c36f0b17ffb4c2dd07c234d744eb2a91a26040516003907fea8787f128d10b2cc0317b0c3960f9ad447f7f6c1ed189db1083ccffd20f456e905f90a250565b825f5f5a9050611c1461332b565b611c4a576040517fc8958ead00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526003602052604090205460ff16611c7957604051632dd9fc9760e01b815260040160405180910390fd5b611c828761318b565b15611ca057604051630e5da8fb60e01b815260040160405180910390fd5b611cb08a8a8a8a89896001613445565b6001600160a01b03831615611eff57365f6020611cce83601f614a2e565b611cd89190614a41565b9050610200611ce8600283614b3b565b611cf29190614a41565b611cfd826006614b49565b611d079190614a2e565b611d119084614a2e565b9250611d1b61332b565b611d27575f9150611e55565b6001600160a01b03841615611e5557836001600160a01b031663e83a2d826040518163ffffffff1660e01b81526004015f60405180830381865afa925050508015611d9357506040513d5f823e601f3d908101601f19168201604052611d909190810190614b60565b60015b15611e5557805115611e53575f856001600160a01b0316631f6d6ef76040518163ffffffff1660e01b8152600401602060405180830381865afa158015611ddc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e0091906149e8565b905048817f00000000000000000000000000000000000000000000000000000000000000008451611e319190614b49565b611e3b9190614b49565b611e459190614a41565b611e4f9086614a2e565b9450505b505b846001600160a01b031663e3db8a49335a611e709087614c01565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e085901b1681526001600160a01b0390921660048301526024820152604481018590526064016020604051808303815f875af1158015611ed7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611efb9190614c14565b5050505b50505050505050505050565b837f00000000000000000000000000000000000000000000000000000000000000005f5a335f9081526003602052604090205490915060ff16611f6157604051632dd9fc9760e01b815260040160405180910390fd5b7f0000000000000000000000000000000000000000000000000000000000000000611f9f57604051631192b39960e31b815260040160405180910390fd5b611fb28861141236879003870187614c2f565b611fbe898988886131d1565b6001600160a01b0383161561220d57365f6020611fdc83601f614a2e565b611fe69190614a41565b9050610200611ff6600283614b3b565b6120009190614a41565b61200b826006614b49565b6120159190614a2e565b61201f9084614a2e565b925061202961332b565b612035575f9150612163565b6001600160a01b0384161561216357836001600160a01b031663e83a2d826040518163ffffffff1660e01b81526004015f60405180830381865afa9250505080156120a157506040513d5f823e601f3d908101601f1916820160405261209e9190810190614b60565b60015b1561216357805115612161575f856001600160a01b0316631f6d6ef76040518163ffffffff1660e01b8152600401602060405180830381865afa1580156120ea573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061210e91906149e8565b905048817f0000000000000000000000000000000000000000000000000000000000000000845161213f9190614b49565b6121499190614b49565b6121539190614a41565b61215d9086614a2e565b9450505b505b846001600160a01b031663e3db8a49335a61217e9087614c01565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e085901b1681526001600160a01b0390921660048301526024820152604481018590526064016020604051808303815f875af11580156121e5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122099190614c14565b5050505b505050505050505050565b467f000000000000000000000000000000000000000000000000000000000000000003612271576040517fa301bb0600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7801000000000000000100000000000000010000000000000001600a55565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163003612348576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602c60248201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060448201527f64656c656761746563616c6c00000000000000000000000000000000000000006064820152608401610922565b7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61038054336001600160a01b038216146123be576040517f23295f0e0000000000000000000000000000000000000000000000000000000081523360048201526001600160a01b0382166024820152604401610922565b7f00000000000000000000000000000000000000000000000000000000000000006123fc57604051631192b39960e31b815260040160405180910390fd5b600c5467ffffffffffffffff1615612440576040517fef34ca5c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61244983612f4e565b505050565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561249e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124c291906149ff565b6001600160a01b0316336001600160a01b0316146125215760025460408051638da5cb5b60e01b8152905133926001600160a01b031691638da5cb5b9160048083019260209291908290030181865afa158015610dd7573d5f5f3e3d5ffd5b61252a81612e22565b60408051825181526020808401519082015282820151818301526060808401519082015290517faa6a58dad31128ff7ecc2b80987ee6e003df80bc50cd8d0b0d1af0e07da6d19d9181900360800190a16040515f907fea8787f128d10b2cc0317b0c3960f9ad447f7f6c1ed189db1083ccffd20f456e908290a250565b60025f9054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156125f7573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061261b91906149ff565b6001600160a01b0316336001600160a01b03161461267a5760025460408051638da5cb5b60e01b8152905133926001600160a01b031691638da5cb5b9160048083019260209291908290030181865afa158015610dd7573d5f5f3e3d5ffd5b5f828260405161268b929190614cdd565b6040519081900381207ffe000000000000000000000000000000000000000000000000000000000000006020830152602182015260410160408051601f1981840301815291905280516020909101209050600160ff1b811862010000831061271f576040517fb3d1f41200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8181526008602052604090205460ff161561276a576040517ffa2fddda00000000000000000000000000000000000000000000000000000000815260048101829052602401610922565b437f0000000000000000000000000000000000000000000000000000000000000000156127f55760646001600160a01b031663a3b1b31d6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156127ce573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127f291906149e8565b90505b6040805180820182526001815267ffffffffffffffff83811660208084019182525f87815260089091528490209251835491517fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000009092169015157fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000ff161761010091909216021790555182907fabca9b7986bc22ad0160eb0cb88ae75411eacfba4052af0b457a9335ef655722906128af9088908890614cec565b60405180910390a26040516002907fea8787f128d10b2cc0317b0c3960f9ad447f7f6c1ed189db1083ccffd20f456e905f90a25050505050565b6001546040517f16bf5579000000000000000000000000000000000000000000000000000000008152600481018390525f916001600160a01b0316906316bf557990602401602060405180830381865afa158015612949573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061296d91906149e8565b92915050565b600a545f9067ffffffffffffffff167f0000000000000000000000000000000000000000000000000000000000000000156129c3575f6129b4600c85613569565b90506129bf816135b7565b9150505b6129cd8184614d1a565b9392505050565b825f5f5a335f9081526003602052604090205490915060ff16158015612a0557506002546001600160a01b03163314155b15612a2357604051632dd9fc9760e01b815260040160405180910390fd5b612a2c8761318b565b15612a4a57604051630e5da8fb60e01b815260040160405180910390fd5b611cb08a8a8a8a89895f613445565b5f5f5f5f5f5f5f5f612a696135e6565b67ffffffffffffffff9384169b50918316995082169750169450505050505b90919293565b5f548611612ac8576040517f7d73e6fa00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f612afd8684612adb602089018961495c565b612aeb60408a0160208b0161495c565b612af660018d614c01565b898861365c565b600a5490915067ffffffffffffffff167f000000000000000000000000000000000000000000000000000000000000000015612b6e57612b4b612b43602088018861495c565b600c90613700565b600c54612b619067ffffffffffffffff166135b7565b67ffffffffffffffff1690505b4381612b7d602089018961495c565b67ffffffffffffffff16612b919190614a2e565b10612bc8576040517fad3515d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6001891115612c4e576001546001600160a01b031663d5719dc2612bee60028c614c01565b6040518263ffffffff1660e01b8152600401612c0c91815260200190565b602060405180830381865afa158015612c27573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c4b91906149e8565b90505b60408051602080820184905281830186905282518083038401815260609092019092528051910120600180546001600160a01b03169063d5719dc290612c94908d614c01565b6040518263ffffffff1660e01b8152600401612cb291815260200190565b602060405180830381865afa158015612ccd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cf191906149e8565b14612d28576040517f13947fd700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5f612d338b613786565b915091505f8b90505f60015f9054906101000a90046001600160a01b03166001600160a01b0316635fca4a166040518163ffffffff1660e01b8152600401602060405180830381865afa158015612d8c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612db091906149e8565b9050805f808080612dc489888388806137c9565b93509350935093508083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7855f548d6002604051612e069493929190614d4e565b60405180910390a4505050505050505050505050505050505050565b805167ffffffffffffffff1080612e445750602081015167ffffffffffffffff105b80612e5a5750604081015167ffffffffffffffff105b80612e705750606081015167ffffffffffffffff105b15612ea7576040517f09cfba7500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8051600a80546020840151604085015160609095015167ffffffffffffffff908116600160c01b0277ffffffffffffffffffffffffffffffffffffffffffffffff968216600160801b02969096166fffffffffffffffffffffffffffffffff92821668010000000000000000027fffffffffffffffffffffffffffffffff000000000000000000000000000000009094169190951617919091171691909117919091179055565b7f0000000000000000000000000000000000000000000000000000000000000000612f8c57604051631192b39960e31b815260040160405180910390fd5b612f95816139a7565b612fcb576040517fda1c8eb600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600c5467ffffffffffffffff161580612ff757506020810151600c5467ffffffffffffffff9182169116115b15613023576020810151600c805467ffffffffffffffff191667ffffffffffffffff9092169190911790555b8051600c5467ffffffffffffffff91821691161015613060578051600c805467ffffffffffffffff191667ffffffffffffffff9092169190911790555b602081810151600c805484517fffffffffffffffff00000000000000000000000000000000ffffffffffffffff9091166801000000000000000067ffffffffffffffff948516027fffffffffffffffff0000000000000000ffffffffffffffffffffffffffffffff1617600160801b91841691909102179055604080840151600d805467ffffffffffffffff191691909316179091555f5460015482517feca067ad000000000000000000000000000000000000000000000000000000008152925191936001600160a01b039091169263eca067ad92600480830193928290030181865afa158015613154573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061317891906149e8565b0361318857613188600c43613700565b50565b5f7f000000000000000000000000000000000000000000000000000000000000000080156131b957505f5482115b801561296d57506131ca600c613a0e565b1592915050565b5f5f5f6131dd86613a40565b9250925092505f5f5f5f6131f4878b5f8c8c6137c9565b93509350935093508a841415801561320d57505f198b14155b1561324e576040517fac7411c900000000000000000000000000000000000000000000000000000000815260048101859052602481018c9052604401610922565b80838c7f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7855f548b60036040516132889493929190614d4e565b60405180910390a47f0000000000000000000000000000000000000000000000000000000000000000156132e8576040517f86657a5300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6132f061332b565b801561331a57507f0000000000000000000000000000000000000000000000000000000000000000155b156113405761134087854888613c62565b5f3332148015610869575050333b1590565b5f5482111561344157613350600c613e9c565b15613441576001545f80546040517fd5719dc200000000000000000000000000000000000000000000000000000000815291926001600160a01b03169163d5719dc2916133a39160040190815260200190565b602060405180830381865afa1580156133be573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133e291906149e8565b90506133f681835f01518460200151613ecc565b61342c576040517fc334542d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60208201516040015161244990600c90613700565b5050565b5f5f613452888888613f10565b915091505f5f5f5f613473868b8961346a575f61346c565b8d5b8c8c6137c9565b93509350935093508c841415801561348c57505f198d14155b156134cd576040517fac7411c900000000000000000000000000000000000000000000000000000000815260048101859052602481018e9052604401610922565b8083857f7394f4a19a13c7b92b5bb71033245305946ef78452f7b4986ac1390b5df4ebd7855f548a8d613501576001613503565b5f5b6040516135139493929190614d4e565b60405180910390a486611efb57837ffe325ca1efe4c5c1062c981c3ee74b781debe4ea9440306a96d2a55759c66c208d8d604051613552929190614cec565b60405180910390a250505050505050505050505050565b815460018301545f916129cd9167ffffffffffffffff600160c01b8304811692868216928282169268010000000000000000808304821693600160801b810483169391900482169116614118565b600a545f9067ffffffffffffffff908116908316106135e257600a5467ffffffffffffffff1661296d565b5090565b5f808080467f00000000000000000000000000000000000000000000000000000000000000001461362257506001925082915081905080612a88565b5050600a5467ffffffffffffffff808216935068010000000000000000820481169250600160801b8204811691600160c01b900416612a88565b6040516001600160f81b031960f889901b1660208201526bffffffffffffffffffffffff19606088901b1660218201527fffffffffffffffff00000000000000000000000000000000000000000000000060c087811b8216603584015286901b16603d8201526045810184905260658101839052608581018290525f9060a5016040516020818303038152906040528051906020012090505b979650505050505050565b61370a8282613569565b825467ffffffffffffffff928316600160c01b0277ffffffffffffffffffffffffffffffff000000000000000090911691831691909117178255600190910180544390921668010000000000000000027fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff909216919091179055565b604080516080810182525f80825260208201819052918101829052606081018290525f5f6137b3856141d9565b8151602090920191909120969095509350505050565b5f5f5f5f5f54881015613808576040517f7d73e6fa00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60015f9054906101000a90046001600160a01b03166001600160a01b031663eca067ad6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613858573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061387c91906149e8565b8811156138b5576040517f925f8bd300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001546040517f86598a56000000000000000000000000000000000000000000000000000000008152600481018b9052602481018a905260448101889052606481018790526001600160a01b03909116906386598a56906084016080604051808303815f875af115801561392b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061394f9190614dd6565b5f8c905592965090945092509050861580159061398a57507f0000000000000000000000000000000000000000000000000000000000000000155b1561399b5761399b8985485f613c62565b95509550955095915050565b80515f9067ffffffffffffffff16158015906139d05750602082015167ffffffffffffffff1615155b80156139ec5750612710826040015167ffffffffffffffff1611155b801561296d5750506020810151905167ffffffffffffffff9182169116111590565b80545f9067ffffffffffffffff600160801b8204811691613a3891600160c01b9091041643614c01565b111592915050565b604080516080810182525f80825260208201819052918101829052606081018290525f5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e83a2d826040518163ffffffff1660e01b81526004015f60405180830381865afa158015613abf573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613ae69190810190614b60565b905080515f03613b22576040517f3cd27eb600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5f613b2d876141d9565b915091505f8351620200007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316631f6d6ef76040518163ffffffff1660e01b8152600401602060405180830381865afa158015613b94573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613bb891906149e8565b613bc29190614b49565b613bcc9190614b49565b60405190915083907f500000000000000000000000000000000000000000000000000000000000000090613c04908790602001614e09565b60408051601f1981840301815290829052613c23939291602001614e55565b60405160208183030381529060405280519060200120825f4811613c47575f613c51565b613c514884614a41565b965096509650505050509193909250565b327f000000000000000000000000000000000000000000000000000000000000000015613d05575f606c6001600160a01b031663c6f7de0e6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613cc7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613ceb91906149e8565b9050613cf74882614a41565b613d019084614a2e565b9250505b67ffffffffffffffff821115613d47576040517f04d5501200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b604080514260208201526bffffffffffffffffffffffff19606084901b16918101919091526054810186905260748101859052609481018490527fffffffffffffffff00000000000000000000000000000000000000000000000060c084901b1660b48201525f9060bc0160408051808303601f1901815290829052600154815160208301207f7a88b1070000000000000000000000000000000000000000000000000000000084526001600160a01b03868116600486015260248501919091529193505f92911690637a88b107906044016020604051808303815f875af1158015613e35573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e5991906149e8565b9050807fff64905f73a67fb594e0f940a8075a860db489ad991e032f48c81123eb52d60b83604051613e8b9190614e84565b60405180910390a250505050505050565b5f613ea682613a0e565b158061296d5750505467ffffffffffffffff680100000000000000008204811691161090565b5f613f0683613eda846142ae565b604080516020808201949094528082019290925280518083038201815260609092019052805191012090565b9093149392505050565b604080516080810182525f80825260208201819052918101829052606081018290525f613f3e856028614a2e565b90507f0000000000000000000000000000000000000000000000000000000000000000811115613fc3576040517f4634691b000000000000000000000000000000000000000000000000000000008152600481018290527f00000000000000000000000000000000000000000000000000000000000000006024820152604401610922565b5f5f613fce866141d9565b909250905086156140dc57613ffd88885f818110613fee57613fee614d3a565b9050013560f81c60f81b6142d9565b6140545787875f81811061401357614013614d3a565b6040517f6b3333560000000000000000000000000000000000000000000000000000000081529201356001600160f81b031916600483015250602401610922565b600160ff1b88885f8161406957614069614d3a565b6001600160f81b03199201359290921616158015915061408a575060218710155b156140dc575f61409e602160018a8c614eb9565b6140a791614ee0565b5f8181526008602052604090205490915060ff166140da5760405162f20c5d60e01b815260048101829052602401610922565b505b8188886040516020016140f193929190614efd565b6040516020818303038152906040528051906020012081945094505050505b935093915050565b5f5f888811614127575f614131565b6141318989614c01565b90505f898711614141575f61414b565b61414b8a88614c01565b905061271061415a8584614b49565b6141649190614a41565b61416e9089614a2e565b97505f86821161417e575f614188565b6141888783614c01565b9050828111156141955750815b808911156141ca576141a7818a614c01565b9850868911156141ca578589116141be57886141c0565b855b93505050506136f5565b50949998505050505050505050565b604080516080810182525f8082526020820181905291810182905260608082018390529161420561436b565b90505f815f01518260200151836040015184606001518860405160200161428395949392919060c095861b7fffffffffffffffff000000000000000000000000000000000000000000000000908116825294861b8516600882015292851b8416601084015290841b8316601883015290921b16602082015260280190565b604051602081830303815290604052905060288151146142a5576142a5614f1b565b94909350915050565b5f61296d825f015183602001518460400151856060015186608001518760a001518860c0015161365c565b5f6001600160f81b0319821615806142fe57506001600160f81b03198216600160ff1b145b8061433257506001600160f81b031982167f8800000000000000000000000000000000000000000000000000000000000000145b8061296d57506001600160f81b031982167f20000000000000000000000000000000000000000000000000000000000000001492915050565b604080516080810182525f808252602082018190529181018290526060810191909152604080516080810182525f8082526020820181905291810182905260608101919091525f5f5f5f6143bd6135e6565b93509350935093508167ffffffffffffffff164211156143ee576143e18242614f2f565b67ffffffffffffffff1685525b6143f88142614d1a565b67ffffffffffffffff9081166020870152841643111561442c5761441c8443614f2f565b67ffffffffffffffff1660408601525b6144368343614d1a565b67ffffffffffffffff1660608601525092949350505050565b5f6020828403121561445f575f5ffd5b5035919050565b6001600160a01b0381168114613188575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b6040805190810167ffffffffffffffff811182821017156144b1576144b161447a565b60405290565b60405160e0810167ffffffffffffffff811182821017156144b1576144b161447a565b604051601f8201601f1916810167ffffffffffffffff811182821017156145035761450361447a565b604052919050565b803567ffffffffffffffff81168114614522575f5ffd5b919050565b5f60608284031215614537575f5ffd5b6040516060810167ffffffffffffffff8111828210171561455a5761455a61447a565b6040529050806145698361450b565b81526145776020840161450b565b60208201526145886040840161450b565b60408201525092915050565b5f5f5f8385036101008112156145a8575f5ffd5b84356145b381614466565b93506080601f19820112156145c6575f5ffd5b506020840191506145da8560a08601614527565b90509250925092565b8015158114613188575f5ffd5b5f5f60408385031215614601575f5ffd5b823561460c81614466565b9150602083013561461c816145e3565b809150509250929050565b5f60208284031215614637575f5ffd5b81356129cd81614466565b5f60608284031215614652575f5ffd5b6129cd8383614527565b5f5f5f5f5f60a08688031215614670575f5ffd5b8535945060208601359350604086013561468981614466565b94979396509394606081013594506080013592915050565b5f5f83601f8401126146b1575f5ffd5b50813567ffffffffffffffff8111156146c8575f5ffd5b6020830191508360208285010111156146df575f5ffd5b9250929050565b5f61010082840312156146f7575f5ffd5b50919050565b5f5f5f5f5f5f5f5f6101c0898b031215614715575f5ffd5b88359750602089013567ffffffffffffffff811115614732575f5ffd5b61473e8b828c016146a1565b90985096505060408901359450606089013561475981614466565b93506080890135925060a089013591506147768a60c08b016146e6565b90509295985092959890939650565b5f5f5f5f5f60808688031215614799575f5ffd5b85359450602086013567ffffffffffffffff8111156147b6575f5ffd5b6147c2888289016146a1565b9095509350506040860135915060608601356147dd81614466565b809150509295509295909350565b5f5f5f5f5f5f5f60c0888a031215614801575f5ffd5b87359650602088013567ffffffffffffffff81111561481e575f5ffd5b61482a8a828b016146a1565b90975095505060408801359350606088013561484581614466565b969995985093969295946080840135945060a09093013592915050565b5f5f5f5f5f5f6101a08789031215614878575f5ffd5b8635955060208701359450604087013561489181614466565b935060608701359250608087013591506148ae8860a089016146e6565b90509295509295509295565b5f60808284031280156148cb575f5ffd5b506040516080810167ffffffffffffffff811182821017156148ef576148ef61447a565b6040908152833582526020808501359083015283810135908201526060928301359281019290925250919050565b5f5f6020838503121561492e575f5ffd5b823567ffffffffffffffff811115614944575f5ffd5b614950858286016146a1565b90969095509350505050565b5f6020828403121561496c575f5ffd5b6129cd8261450b565b803560ff81168114614522575f5ffd5b5f5f5f5f5f5f60e0878903121561499a575f5ffd5b863595506149aa60208801614975565b945060808701888111156149bc575f5ffd5b60408801945035925060a08701356149d381614466565b9598949750929591949360c090920135925050565b5f602082840312156149f8575f5ffd5b5051919050565b5f60208284031215614a0f575f5ffd5b81516129cd81614466565b634e487b7160e01b5f52601160045260245ffd5b8082018082111561296d5761296d614a1a565b5f82614a5b57634e487b7160e01b5f52601260045260245ffd5b500490565b6001815b600184111561411057808504811115614a7f57614a7f614a1a565b6001841615614a8d57908102905b60019390931c928002614a64565b5f82614aa95750600161296d565b81614ab557505f61296d565b8160018114614acb5760028114614ad557614af1565b600191505061296d565b60ff841115614ae657614ae6614a1a565b50506001821b61296d565b5060208310610133831016604e8410600b8410161715614b14575081810a61296d565b614b205f198484614a60565b805f1904821115614b3357614b33614a1a565b029392505050565b5f6129cd60ff841683614a9b565b808202811582820484141761296d5761296d614a1a565b5f60208284031215614b70575f5ffd5b815167ffffffffffffffff811115614b86575f5ffd5b8201601f81018413614b96575f5ffd5b805167ffffffffffffffff811115614bb057614bb061447a565b8060051b614bc0602082016144da565b91825260208184018101929081019087841115614bdb575f5ffd5b6020850194505b838510156136f557845180835260209586019590935090910190614be2565b8181038181111561296d5761296d614a1a565b5f60208284031215614c24575f5ffd5b81516129cd816145e3565b5f81830361010081128015614c42575f5ffd5b50614c4b61448e565b8335815260e0601f1983011215614c60575f5ffd5b614c686144b7565b9150614c7660208501614975565b82526040840135614c8681614466565b6020830152614c976060850161450b565b6040830152614ca86080850161450b565b606083015260a084810135608084015260c0808601359184019190915260e09094013593820193909352602083015250919050565b818382375f9101908152919050565b60208152816020820152818360408301375f818301604090810191909152601f909201601f19160101919050565b67ffffffffffffffff818116838216019081111561296d5761296d614a1a565b634e487b7160e01b5f52603260045260245ffd5b5f60e08201905085825284602083015267ffffffffffffffff845116604083015267ffffffffffffffff602085015116606083015267ffffffffffffffff604085015116608083015267ffffffffffffffff60608501511660a083015260048310614dc757634e487b7160e01b5f52602160045260245ffd5b8260c083015295945050505050565b5f5f5f5f60808587031215614de9575f5ffd5b505082516020840151604085015160609095015191969095509092509050565b81515f90829060208501835b82811015614e33578151845260209384019390910190600101614e15565b509195945050505050565b5f81518060208401855e5f93019283525090919050565b5f614e608286614e3e565b6001600160f81b031985168152614e7a6001820185614e3e565b9695505050505050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b5f5f85851115614ec7575f5ffd5b83861115614ed3575f5ffd5b5050820193919092039150565b8035602083101561296d575f19602084900360031b1b1692915050565b5f614f088286614e3e565b838582375f930192835250909392505050565b634e487b7160e01b5f52600160045260245ffd5b67ffffffffffffffff828116828216039081111561296d5761296d614a1a56fea264697066735822122038e99cf4eb42b1bd091544a0482c80ae5681aaff1e73e5b858eba14b32894aee64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\xE3W_5`\xE0\x1C\x80c\x7F\xA3\xA4\x0E\x11a\x01\x87W\x80c\xD1\xCE\x8D\xA8\x11a\0\xDDW\x80c\xE7\x8C\xEA\x92\x11a\0\x93W\x80c\xED\xAA\xFE \x11a\0nW\x80c\xED\xAA\xFE \x14a\x07@W\x80c\xF1\x98\x15x\x14a\x07\xC9W\x80c\xF6\nP\x91\x14a\x07\xDCW__\xFD[\x80c\xE7\x8C\xEA\x92\x14a\x06\xDEW\x80c\xE8\xEB\x1D\xC3\x14a\x06\xF1W\x80c\xEB\xEAF\x1D\x14a\x07\x18W__\xFD[\x80c\xDDD\xE6\xE0\x11a\0\xC3W\x80c\xDDD\xE6\xE0\x14a\x06xW\x80c\xE0\xBC\x97)\x14a\x06\xA4W\x80c\xE5\xA3X\xC8\x14a\x06\xB7W__\xFD[\x80c\xD1\xCE\x8D\xA8\x14a\x06RW\x80c\xD9\xDDg\xAB\x14a\x06eW__\xFD[\x80c\x92\xD9\xF7\x82\x11a\x01=W\x80c\xB3\x17a\xF8\x11a\x01\x18W\x80c\xB3\x17a\xF8\x14a\x06\x19W\x80c\xCB#\xBC\xB5\x14a\x06,W\x80c\xCC*\x1A\x0C\x14a\x06?W__\xFD[\x80c\x92\xD9\xF7\x82\x14a\x05\xD7W\x80c\x96\xCC\\x\x14a\x05\xFEW\x80c\xA6U\xD97\x14a\x06\x06W__\xFD[\x80c\x8D\x91\r\xDE\x11a\x01mW\x80c\x8D\x91\r\xDE\x14a\x05rW\x80c\x8F\x11\x1F<\x14a\x05\xB1W\x80c\x91|\xF8\xAC\x14a\x05\xC4W__\xFD[\x80c\x7F\xA3\xA4\x0E\x14a\x05WW\x80c\x84B\x08`\x14a\x05_W__\xFD[\x80c>Z\xA0\x82\x11a\x02<W\x80cmF\xE9\x87\x11a\x01\xF2W\x80co\x12\xB0\xC9\x11a\x01\xCDW\x80co\x12\xB0\xC9\x14a\x04\xCFW\x80cq^\xA3K\x14a\x04\xE2W\x80cq\xC3\xE6\xFE\x14a\x055W__\xFD[\x80cmF\xE9\x87\x14a\x04\x87W\x80cnb\0U\x14a\x04\xA9W\x80cn}\xF3\xE7\x14a\x04\xBCW__\xFD[\x80ci\xCA\xCD\xED\x11a\x02\"W\x80ci\xCA\xCD\xED\x14a\x04EW\x80cj\xE7\x1F\x12\x14a\x04XW\x80cl\x89\x04P\x14a\x04`W__\xFD[\x80c>Z\xA0\x82\x14a\x04\x0BW\x80cKg\x8Af\x14a\x04\x1EW__\xFD[\x80c\x1F\x95f2\x11a\x02\x9CW\x80c'\x95zI\x11a\x02wW\x80c'\x95zI\x14a\x03\xC9W\x80c,\xBFt\xE5\x14a\x03\xD1W\x80c/9\x85\xA7\x14a\x03\xF8W__\xFD[\x80c\x1F\x95f2\x14a\x03\x90W\x80c\x1F\xF6G\x90\x14a\x03\xA3W\x80c%\x8F\x04\x95\x14a\x03\xB6W__\xFD[\x80c\x167\xBEH\x11a\x02\xCCW\x80c\x167\xBEH\x14a\x03BW\x80c\x16\xAF\x91\xA7\x14a\x03tW\x80c\x1A\xD8~E\x14a\x03{W__\xFD[\x80c\x02\xC9\x92u\x14a\x02\xE7W\x80c\x06\xF10V\x14a\x03,W[__\xFD[a\x03\x0E\x7F \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x034a\x07\xE7V[`@Q\x90\x81R` \x01a\x03#V[a\x03da\x03P6`\x04aDOV[_\x90\x81R`\x08` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x03#V[a\x03\x0E_\x81V[a\x03\x8Ea\x03\x896`\x04aE\x94V[a\x08nV[\0[a\x03\x8Ea\x03\x9E6`\x04aE\xF0V[a\x0B\x9EV[a\x03\x8Ea\x03\xB16`\x04aF'V[a\r\x04V[a\x034a\x03\xC46`\x04aDOV[a\x0E\xCCV[a\x034`(\x81V[a\x03\x0E\x7FP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x8Ea\x04\x066`\x04aFBV[a\x0F8V[a\x03\x8Ea\x04\x196`\x04aF\\V[a\x10lV[a\x03d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x8Ea\x04S6`\x04aF\xFDV[a\x13NV[a\x03\x8Ea\x16\x82V[a\x03\x0E\x7F\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03da\x04\x956`\x04aF'V[`\t` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\x8Ea\x04\xB76`\x04aF\xFDV[a\x18RV[a\x03\x8Ea\x04\xCA6`\x04aE\xF0V[a\x19\x01V[a\x03\x8Ea\x04\xDD6`\x04aG\x85V[a\x1AgV[a\x05\x15a\x04\xF06`\x04aDOV[`\x08` R_\x90\x81R`@\x90 T`\xFF\x81\x16\x90a\x01\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82V[`@\x80Q\x92\x15\x15\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x03#V[a\x03da\x05C6`\x04aF'V[`\x03` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x034_T\x81V[a\x03\x8Ea\x05m6`\x04aDOV[a\x1A\x99V[a\x05\x99\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03#V[a\x03\x8Ea\x05\xBF6`\x04aG\xEBV[a\x1C\x06V[a\x03\x8Ea\x05\xD26`\x04aHbV[a\x1F\x0BV[a\x03d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x8Ea\"\x18V[a\x03\x8Ea\x06\x146`\x04aFBV[a\"\x90V[a\x03\x8Ea\x06'6`\x04aH\xBAV[a$NV[`\x02Ta\x05\x99\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x0BTa\x05\x99\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x8Ea\x06`6`\x04aI\x1DV[a%\xA7V[a\x034a\x06s6`\x04aDOV[a(\xE9V[a\x06\x8Ba\x06\x866`\x04aI\\V[a)sV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03#V[a\x03\x8Ea\x06\xB26`\x04aG\xEBV[a)\xD4V[a\x03\x0E\x7F@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01Ta\x05\x99\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x034\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x07 a*YV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x03#V[`\x0CT`\rTa\x07\x86\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x92h\x01\0\0\0\0\0\0\0\0\x80\x84\x04\x83\x16\x93`\x01`\x80\x1B\x81\x04\x84\x16\x93`\x01`\xC0\x1B\x90\x91\x04\x81\x16\x92\x80\x82\x16\x92\x90\x04\x16\x86V[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16\x81R\x95\x87\x16` \x87\x01R\x93\x86\x16\x93\x85\x01\x93\x90\x93R\x90\x84\x16``\x84\x01R\x83\x16`\x80\x83\x01R\x90\x91\x16`\xA0\x82\x01R`\xC0\x01a\x03#V[a\x03\x8Ea\x07\xD76`\x04aI\x85V[a*\x8EV[a\x03\x0E`\x01`\xFF\x1B\x81V[`\x01T`@\x80Q~\x84\x12\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91b\x84\x12\x0C\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x08EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08i\x91\x90aI\xE8V[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\t+W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\tnW`@Q\x7F\xEF4\xCA\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16a\t\xAEW`@Q\x7F\x1A\xD0\xF7C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16c\xE1u\x8B\xD8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\n\x0CWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\n\t\x91\x81\x01\x90aI\xFFV[`\x01[\x15a\n'W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\n%W`\x01\x91P[P[\x80\x15\x15\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15\x15\x14a\n\x84W`@Q\x7F\xC3\xE3\x1F\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x81\x17\x90\x91U`@\x80Q\x7F\xCB#\xBC\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qc\xCB#\xBC\xB5\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0B\x01W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B%\x91\x90aI\xFFV[`\x02\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x0Bia\x0Bd6\x85\x90\x03\x85\x01\x85aH\xBAV[a.\"V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0B\x98Wa\x0B\x98\x82a/NV[PPPPV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x12\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x0C>WP`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x0CwW`@Q\x7Ff\x0B;B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x01a\t\"V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\t` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x7F\xEB\x12\xA9\xA5>\xEC\x13\x8C\x91\xB2{O\x91*%{\xD6\x90\xC1\x8F\xC8\xBD\xE7D\xBE\x92\xA06^\xB9\xB8~\x91\x01`@Q\x80\x91\x03\x90\xA1`@Q`\x04\x90\x7F\xEA\x87\x87\xF1(\xD1\x0B,\xC01{\x0C9`\xF9\xADD\x7F\x7Fl\x1E\xD1\x89\xDB\x10\x83\xCC\xFF\xD2\x0FEn\x90_\x90\xA2PPV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rTW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rx\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E@W`\x02T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xFB\x91\x90aI\xFFV[`@Q\x7F#)_\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\t\"V[`\x0B\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F<\xD6\xC1\x84\x80\x02\x97\xA0\xF2\xB0\t&\xA6\x83\xCB\xE7h\x90\xBB\x7F\xD0\x14\x80\xAC\n\x10\xEDl\x8F\x7FfY\x90` \x01`@Q\x80\x91\x03\x90\xA1`@Q`\x05\x90\x7F\xEA\x87\x87\xF1(\xD1\x0B,\xC01{\x0C9`\xF9\xADD\x7F\x7Fl\x1E\xD1\x89\xDB\x10\x83\xCC\xFF\xD2\x0FEn\x90_\x90\xA2PV[_\x81\x81R`\x08` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92RT`\xFF\x81\x16\x15\x15\x82Ra\x01\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x90\x82\x03a\x0F$W`@Qb\xF2\x0C]`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\t\"V[` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xAC\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10\x0BW`\x02T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\xD7W=__>=_\xFD[a\x10\x14\x81a/NV[`@\x80Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x82\x16\x90\x83\x01R\x83\x83\x01Q\x16\x81\x83\x01R\x90Q\x7F\xAAz-\x81u\xDE\xE3\xB67\x81J\xD64`\x05\xDF\xCC5qe9o\xB82\x7Fd\x9E\xFF\xE8\xAB\xCF\x85\x91\x81\x90\x03``\x01\x90\xA1PV[\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Z3_\x90\x81R`\x03` R`@\x90 T\x90\x91P`\xFF\x16a\x10\xC2W`@Qc-\xD9\xFC\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xCB\x87a1\x8BV[\x15a\x10\xE9W`@Qc\x0E]\xA8\xFB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xF5\x88\x88\x87\x87a1\xD1V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x13DW6_` a\x11\x13\x83`\x1FaJ.V[a\x11\x1D\x91\x90aJAV[\x90Pa\x02\0a\x11-`\x02\x83aK;V[a\x117\x91\x90aJAV[a\x11B\x82`\x06aKIV[a\x11L\x91\x90aJ.V[a\x11V\x90\x84aJ.V[\x92Pa\x11`a3+V[a\x11lW_\x91Pa\x12\x9AV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x12\x9AW\x83`\x01`\x01`\xA0\x1B\x03\x16c\xE8:-\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x11\xD8WP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\xD5\x91\x90\x81\x01\x90aK`V[`\x01[\x15a\x12\x9AW\x80Q\x15a\x12\x98W_\x85`\x01`\x01`\xA0\x1B\x03\x16c\x1Fmn\xF7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12E\x91\x90aI\xE8V[\x90PH\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84Qa\x12v\x91\x90aKIV[a\x12\x80\x91\x90aKIV[a\x12\x8A\x91\x90aJAV[a\x12\x94\x90\x86aJ.V[\x94PP[P[\x84`\x01`\x01`\xA0\x1B\x03\x16c\xE3\xDB\x8AI3Za\x12\xB5\x90\x87aL\x01V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x81\x01\x85\x90R`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13@\x91\x90aL\x14V[PPP[PPPPPPPPV[\x83__Z\x90Pa\x13\\a3+V[a\x13\x92W`@Q\x7F\xC8\x95\x8E\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x03` R`@\x90 T`\xFF\x16a\x13\xC1W`@Qc-\xD9\xFC\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13\xFFW`@Qc\x11\x92\xB3\x99`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\x17\x88a\x14\x126\x87\x90\x03\x87\x01\x87aL/V[a3=V[a\x14'\x8B\x8B\x8B\x8B\x8A\x8A`\x01a4EV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x13@W6_` a\x14E\x83`\x1FaJ.V[a\x14O\x91\x90aJAV[\x90Pa\x02\0a\x14_`\x02\x83aK;V[a\x14i\x91\x90aJAV[a\x14t\x82`\x06aKIV[a\x14~\x91\x90aJ.V[a\x14\x88\x90\x84aJ.V[\x92Pa\x14\x92a3+V[a\x14\x9EW_\x91Pa\x15\xCCV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x15\xCCW\x83`\x01`\x01`\xA0\x1B\x03\x16c\xE8:-\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x15\nWP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x15\x07\x91\x90\x81\x01\x90aK`V[`\x01[\x15a\x15\xCCW\x80Q\x15a\x15\xCAW_\x85`\x01`\x01`\xA0\x1B\x03\x16c\x1Fmn\xF7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15SW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15w\x91\x90aI\xE8V[\x90PH\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84Qa\x15\xA8\x91\x90aKIV[a\x15\xB2\x91\x90aKIV[a\x15\xBC\x91\x90aJAV[a\x15\xC6\x90\x86aJ.V[\x94PP[P[\x84`\x01`\x01`\xA0\x1B\x03\x16c\xE3\xDB\x8AI3Za\x15\xE7\x90\x87aL\x01V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x81\x01\x85\x90R`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16NW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16r\x91\x90aL\x14V[PPPPPPPPPPPPPPV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xF6\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17UW`\x02T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\xD7W=__>=_\xFD[`\x01T`@\x80Q\x7F\xCB#\xBC\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xCB#\xBC\xB5\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x17\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xD9\x91\x90aI\xFFV[`\x02T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x03a\x18#W`@Q\x7F\xD0T\x90\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x83__Z3_\x90\x81R`\x03` R`@\x90 T\x90\x91P`\xFF\x16\x15\x80\x15a\x18\x83WP`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x18\xA1W`@Qc-\xD9\xFC\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x18\xDFW`@Qc\x11\x92\xB3\x99`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\xF2\x88a\x14\x126\x87\x90\x03\x87\x01\x87aL/V[a\x14'\x8B\x8B\x8B\x8B\x8A\x8A_a4EV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19QW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19u\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x19\xA1WP`\x0BT`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x19\xDAW`@Q\x7Ff\x0B;B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x01a\t\"V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x7F(\xBC\xC5bm5~\xFE\x96kK\x08v\xAA\x1E\xE8\xAB\x99\xE2m\xA4\xF11\xF6\xA2b?\x18\0p\x1C!\x91\x01`@Q\x80\x91\x03\x90\xA1`@Q`\x01\x90\x7F\xEA\x87\x87\xF1(\xD1\x0B,\xC01{\x0C9`\xF9\xADD\x7F\x7Fl\x1E\xD1\x89\xDB\x10\x83\xCC\xFF\xD2\x0FEn\x90_\x90\xA2PPV[`@Q\x7F\xC7;\x9D|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xE9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\r\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1BlW`\x02T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\xD7W=__>=_\xFD[_\x81\x81R`\x08` R`@\x90 T`\xFF\x16a\x1B\x9CW`@Qb\xF2\x0C]`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\"V[_\x81\x81R`\x08` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x82\x91\x7F\\\xB4!\x8B'/\xD2\x14\x16\x8A\xC4>\x90\xFBM\x05\xD6\xC3o\x0B\x17\xFF\xB4\xC2\xDD\x07\xC24\xD7D\xEB*\x91\xA2`@Q`\x03\x90\x7F\xEA\x87\x87\xF1(\xD1\x0B,\xC01{\x0C9`\xF9\xADD\x7F\x7Fl\x1E\xD1\x89\xDB\x10\x83\xCC\xFF\xD2\x0FEn\x90_\x90\xA2PV[\x82__Z\x90Pa\x1C\x14a3+V[a\x1CJW`@Q\x7F\xC8\x95\x8E\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x03` R`@\x90 T`\xFF\x16a\x1CyW`@Qc-\xD9\xFC\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\x82\x87a1\x8BV[\x15a\x1C\xA0W`@Qc\x0E]\xA8\xFB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\xB0\x8A\x8A\x8A\x8A\x89\x89`\x01a4EV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x1E\xFFW6_` a\x1C\xCE\x83`\x1FaJ.V[a\x1C\xD8\x91\x90aJAV[\x90Pa\x02\0a\x1C\xE8`\x02\x83aK;V[a\x1C\xF2\x91\x90aJAV[a\x1C\xFD\x82`\x06aKIV[a\x1D\x07\x91\x90aJ.V[a\x1D\x11\x90\x84aJ.V[\x92Pa\x1D\x1Ba3+V[a\x1D'W_\x91Pa\x1EUV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x1EUW\x83`\x01`\x01`\xA0\x1B\x03\x16c\xE8:-\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1D\x93WP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D\x90\x91\x90\x81\x01\x90aK`V[`\x01[\x15a\x1EUW\x80Q\x15a\x1ESW_\x85`\x01`\x01`\xA0\x1B\x03\x16c\x1Fmn\xF7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xDCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\0\x91\x90aI\xE8V[\x90PH\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84Qa\x1E1\x91\x90aKIV[a\x1E;\x91\x90aKIV[a\x1EE\x91\x90aJAV[a\x1EO\x90\x86aJ.V[\x94PP[P[\x84`\x01`\x01`\xA0\x1B\x03\x16c\xE3\xDB\x8AI3Za\x1Ep\x90\x87aL\x01V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x81\x01\x85\x90R`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xFB\x91\x90aL\x14V[PPP[PPPPPPPPPPV[\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Z3_\x90\x81R`\x03` R`@\x90 T\x90\x91P`\xFF\x16a\x1FaW`@Qc-\xD9\xFC\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1F\x9FW`@Qc\x11\x92\xB3\x99`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\xB2\x88a\x14\x126\x87\x90\x03\x87\x01\x87aL/V[a\x1F\xBE\x89\x89\x88\x88a1\xD1V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\"\rW6_` a\x1F\xDC\x83`\x1FaJ.V[a\x1F\xE6\x91\x90aJAV[\x90Pa\x02\0a\x1F\xF6`\x02\x83aK;V[a \0\x91\x90aJAV[a \x0B\x82`\x06aKIV[a \x15\x91\x90aJ.V[a \x1F\x90\x84aJ.V[\x92Pa )a3+V[a 5W_\x91Pa!cV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a!cW\x83`\x01`\x01`\xA0\x1B\x03\x16c\xE8:-\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a \xA1WP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra \x9E\x91\x90\x81\x01\x90aK`V[`\x01[\x15a!cW\x80Q\x15a!aW_\x85`\x01`\x01`\xA0\x1B\x03\x16c\x1Fmn\xF7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xEAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x0E\x91\x90aI\xE8V[\x90PH\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84Qa!?\x91\x90aKIV[a!I\x91\x90aKIV[a!S\x91\x90aJAV[a!]\x90\x86aJ.V[\x94PP[P[\x84`\x01`\x01`\xA0\x1B\x03\x16c\xE3\xDB\x8AI3Za!~\x90\x87aL\x01V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x81\x01\x85\x90R`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\xE5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\t\x91\x90aL\x14V[PPP[PPPPPPPPPV[F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a\"qW`@Q\x7F\xA3\x01\xBB\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[x\x01\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01`\nUV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a#HW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\"V[\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03\x80T3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a#\xBEW`@Q\x7F#)_\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`D\x01a\t\"V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a#\xFCW`@Qc\x11\x92\xB3\x99`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a$@W`@Q\x7F\xEF4\xCA\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$I\x83a/NV[PPPV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xC2\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a%!W`\x02T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\xD7W=__>=_\xFD[a%*\x81a.\"V[`@\x80Q\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R\x82\x82\x01Q\x81\x83\x01R``\x80\x84\x01Q\x90\x82\x01R\x90Q\x7F\xAAjX\xDA\xD3\x11(\xFF~\xCC+\x80\x98~\xE6\xE0\x03\xDF\x80\xBCP\xCD\x8D\x0B\r\x1A\xF0\xE0}\xA6\xD1\x9D\x91\x81\x90\x03`\x80\x01\x90\xA1`@Q_\x90\x7F\xEA\x87\x87\xF1(\xD1\x0B,\xC01{\x0C9`\xF9\xADD\x7F\x7Fl\x1E\xD1\x89\xDB\x10\x83\xCC\xFF\xD2\x0FEn\x90\x82\x90\xA2PV[`\x02_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xF7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x1B\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a&zW`\x02T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\xD7W=__>=_\xFD[_\x82\x82`@Qa&\x8B\x92\x91\x90aL\xDDV[`@Q\x90\x81\x90\x03\x81 \x7F\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`!\x82\x01R`A\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P`\x01`\xFF\x1B\x81\x18b\x01\0\0\x83\x10a'\x1FW`@Q\x7F\xB3\xD1\xF4\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x08` R`@\x90 T`\xFF\x16\x15a'jW`@Q\x7F\xFA/\xDD\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\"V[C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a'\xF5W`d`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xB1\xB3\x1D`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xF2\x91\x90aI\xE8V[\x90P[`@\x80Q\x80\x82\x01\x82R`\x01\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16` \x80\x84\x01\x91\x82R_\x87\x81R`\x08\x90\x91R\x84\x90 \x92Q\x83T\x91Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x90\x92\x16\x90\x15\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\x16\x17a\x01\0\x91\x90\x92\x16\x02\x17\x90UQ\x82\x90\x7F\xAB\xCA\x9By\x86\xBC\"\xAD\x01`\xEB\x0C\xB8\x8A\xE7T\x11\xEA\xCF\xBA@R\xAF\x0BEz\x935\xEFeW\"\x90a(\xAF\x90\x88\x90\x88\x90aL\xECV[`@Q\x80\x91\x03\x90\xA2`@Q`\x02\x90\x7F\xEA\x87\x87\xF1(\xD1\x0B,\xC01{\x0C9`\xF9\xADD\x7F\x7Fl\x1E\xD1\x89\xDB\x10\x83\xCC\xFF\xD2\x0FEn\x90_\x90\xA2PPPPPV[`\x01T`@Q\x7F\x16\xBFUy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x16\xBFUy\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)IW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)m\x91\x90aI\xE8V[\x92\x91PPV[`\nT_\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a)\xC3W_a)\xB4`\x0C\x85a5iV[\x90Pa)\xBF\x81a5\xB7V[\x91PP[a)\xCD\x81\x84aM\x1AV[\x93\x92PPPV[\x82__Z3_\x90\x81R`\x03` R`@\x90 T\x90\x91P`\xFF\x16\x15\x80\x15a*\x05WP`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a*#W`@Qc-\xD9\xFC\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a*,\x87a1\x8BV[\x15a*JW`@Qc\x0E]\xA8\xFB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\xB0\x8A\x8A\x8A\x8A\x89\x89_a4EV[________a*ia5\xE6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x9BP\x91\x83\x16\x99P\x82\x16\x97P\x16\x94PPPPP[\x90\x91\x92\x93V[_T\x86\x11a*\xC8W`@Q\x7F}s\xE6\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a*\xFD\x86\x84a*\xDB` \x89\x01\x89aI\\V[a*\xEB`@\x8A\x01` \x8B\x01aI\\V[a*\xF6`\x01\x8DaL\x01V[\x89\x88a6\\V[`\nT\x90\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a+nWa+Ka+C` \x88\x01\x88aI\\V[`\x0C\x90a7\0V[`\x0CTa+a\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a5\xB7V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[C\x81a+}` \x89\x01\x89aI\\V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a+\x91\x91\x90aJ.V[\x10a+\xC8W`@Q\x7F\xAD5\x15\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\x01\x89\x11\x15a,NW`\x01T`\x01`\x01`\xA0\x1B\x03\x16c\xD5q\x9D\xC2a+\xEE`\x02\x8CaL\x01V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\x0C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,K\x91\x90aI\xE8V[\x90P[`@\x80Q` \x80\x82\x01\x84\x90R\x81\x83\x01\x86\x90R\x82Q\x80\x83\x03\x84\x01\x81R``\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD5q\x9D\xC2\x90a,\x94\x90\x8DaL\x01V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xB2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xCDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xF1\x91\x90aI\xE8V[\x14a-(W`@Q\x7F\x13\x94\x7F\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a-3\x8Ba7\x86V[\x91P\x91P_\x8B\x90P_`\x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c_\xCAJ\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\x8CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xB0\x91\x90aI\xE8V[\x90P\x80_\x80\x80\x80a-\xC4\x89\x88\x83\x88\x80a7\xC9V[\x93P\x93P\x93P\x93P\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85_T\x8D`\x02`@Qa.\x06\x94\x93\x92\x91\x90aMNV[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPPPPPPPPPPV[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10\x80a.DWP` \x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10[\x80a.ZWP`@\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10[\x80a.pWP``\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10[\x15a.\xA7W`@Q\x7F\t\xCF\xBAu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\n\x80T` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x01`\xC0\x1B\x02w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96\x82\x16`\x01`\x80\x1B\x02\x96\x90\x96\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x82\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x91\x90\x95\x16\x17\x91\x90\x91\x17\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90UV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a/\x8CW`@Qc\x11\x92\xB3\x99`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a/\x95\x81a9\xA7V[a/\xCBW`@Q\x7F\xDA\x1C\x8E\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0CTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80a/\xF7WP` \x81\x01Q`\x0CTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x11[\x15a0#W` \x81\x01Q`\x0C\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[\x80Q`\x0CTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x10\x15a0`W\x80Q`\x0C\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[` \x81\x81\x01Q`\x0C\x80T\x84Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16h\x01\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17`\x01`\x80\x1B\x91\x84\x16\x91\x90\x91\x02\x17\x90U`@\x80\x84\x01Q`\r\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x93\x16\x17\x90\x91U_T`\x01T\x82Q\x7F\xEC\xA0g\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92Q\x91\x93`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c\xEC\xA0g\xAD\x92`\x04\x80\x83\x01\x93\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a1TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1x\x91\x90aI\xE8V[\x03a1\x88Wa1\x88`\x0CCa7\0V[PV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x15a1\xB9WP_T\x82\x11[\x80\x15a)mWPa1\xCA`\x0Ca:\x0EV[\x15\x92\x91PPV[___a1\xDD\x86a:@V[\x92P\x92P\x92P____a1\xF4\x87\x8B_\x8C\x8Ca7\xC9V[\x93P\x93P\x93P\x93P\x8A\x84\x14\x15\x80\x15a2\rWP_\x19\x8B\x14\x15[\x15a2NW`@Q\x7F\xACt\x11\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x8C\x90R`D\x01a\t\"V[\x80\x83\x8C\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85_T\x8B`\x03`@Qa2\x88\x94\x93\x92\x91\x90aMNV[`@Q\x80\x91\x03\x90\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a2\xE8W`@Q\x7F\x86ezS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a2\xF0a3+V[\x80\x15a3\x1AWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15[\x15a\x13@Wa\x13@\x87\x85H\x88a<bV[_32\x14\x80\x15a\x08iWPP3;\x15\x90V[_T\x82\x11\x15a4AWa3P`\x0Ca>\x9CV[\x15a4AW`\x01T_\x80T`@Q\x7F\xD5q\x9D\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xD5q\x9D\xC2\x91a3\xA3\x91`\x04\x01\x90\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xBEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xE2\x91\x90aI\xE8V[\x90Pa3\xF6\x81\x83_\x01Q\x84` \x01Qa>\xCCV[a4,W`@Q\x7F\xC34T-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Q`@\x01Qa$I\x90`\x0C\x90a7\0V[PPV[__a4R\x88\x88\x88a?\x10V[\x91P\x91P____a4s\x86\x8B\x89a4jW_a4lV[\x8D[\x8C\x8Ca7\xC9V[\x93P\x93P\x93P\x93P\x8C\x84\x14\x15\x80\x15a4\x8CWP_\x19\x8D\x14\x15[\x15a4\xCDW`@Q\x7F\xACt\x11\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x8E\x90R`D\x01a\t\"V[\x80\x83\x85\x7Fs\x94\xF4\xA1\x9A\x13\xC7\xB9+[\xB7\x103$S\x05\x94n\xF7\x84R\xF7\xB4\x98j\xC19\x0B]\xF4\xEB\xD7\x85_T\x8A\x8Da5\x01W`\x01a5\x03V[_[`@Qa5\x13\x94\x93\x92\x91\x90aMNV[`@Q\x80\x91\x03\x90\xA4\x86a\x1E\xFBW\x83\x7F\xFE2\\\xA1\xEF\xE4\xC5\xC1\x06,\x98\x1C>\xE7Kx\x1D\xEB\xE4\xEA\x94@0j\x96\xD2\xA5WY\xC6l \x8D\x8D`@Qa5R\x92\x91\x90aL\xECV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPV[\x81T`\x01\x83\x01T_\x91a)\xCD\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x83\x04\x81\x16\x92\x86\x82\x16\x92\x82\x82\x16\x92h\x01\0\0\0\0\0\0\0\0\x80\x83\x04\x82\x16\x93`\x01`\x80\x1B\x81\x04\x83\x16\x93\x91\x90\x04\x82\x16\x91\x16aA\x18V[`\nT_\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x83\x16\x10a5\xE2W`\nTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a)mV[P\x90V[_\x80\x80\x80F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a6\"WP`\x01\x92P\x82\x91P\x81\x90P\x80a*\x88V[PP`\nTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x93Ph\x01\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x92P`\x01`\x80\x1B\x82\x04\x81\x16\x91`\x01`\xC0\x1B\x90\x04\x16a*\x88V[`@Q`\x01`\x01`\xF8\x1B\x03\x19`\xF8\x89\x90\x1B\x16` \x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x88\x90\x1B\x16`!\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x87\x81\x1B\x82\x16`5\x84\x01R\x86\x90\x1B\x16`=\x82\x01R`E\x81\x01\x84\x90R`e\x81\x01\x83\x90R`\x85\x81\x01\x82\x90R_\x90`\xA5\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x97\x96PPPPPPPV[a7\n\x82\x82a5iV[\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x01`\xC0\x1B\x02w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x90\x91\x16\x91\x83\x16\x91\x90\x91\x17\x17\x82U`\x01\x90\x91\x01\x80TC\x90\x92\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R__a7\xB3\x85aA\xD9V[\x81Q` \x90\x92\x01\x91\x90\x91 \x96\x90\x95P\x93PPPPV[_____T\x88\x10\x15a8\x08W`@Q\x7F}s\xE6\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEC\xA0g\xAD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8XW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8|\x91\x90aI\xE8V[\x88\x11\x15a8\xB5W`@Q\x7F\x92_\x8B\xD3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`@Q\x7F\x86Y\x8AV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8B\x90R`$\x81\x01\x8A\x90R`D\x81\x01\x88\x90R`d\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x86Y\x8AV\x90`\x84\x01`\x80`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a9+W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9O\x91\x90aM\xD6V[_\x8C\x90U\x92\x96P\x90\x94P\x92P\x90P\x86\x15\x80\x15\x90a9\x8AWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15[\x15a9\x9BWa9\x9B\x89\x85H_a<bV[\x95P\x95P\x95P\x95\x91PPV[\x80Q_\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a9\xD0WP` \x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15[\x80\x15a9\xECWPa'\x10\x82`@\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15[\x80\x15a)mWPP` \x81\x01Q\x90Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x11\x15\x90V[\x80T_\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\x80\x1B\x82\x04\x81\x16\x91a:8\x91`\x01`\xC0\x1B\x90\x91\x04\x16CaL\x01V[\x11\x15\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R__\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE8:-\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\xBFW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra:\xE6\x91\x90\x81\x01\x90aK`V[\x90P\x80Q_\x03a;\"W`@Q\x7F<\xD2~\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a;-\x87aA\xD9V[\x91P\x91P_\x83Qb\x02\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x1Fmn\xF7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xB8\x91\x90aI\xE8V[a;\xC2\x91\x90aKIV[a;\xCC\x91\x90aKIV[`@Q\x90\x91P\x83\x90\x7FP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a<\x04\x90\x87\x90` \x01aN\tV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra<#\x93\x92\x91` \x01aNUV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82_H\x11a<GW_a<QV[a<QH\x84aJAV[\x96P\x96P\x96PPPPP\x91\x93\x90\x92PV[2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a=\x05W_`l`\x01`\x01`\xA0\x1B\x03\x16c\xC6\xF7\xDE\x0E`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xEB\x91\x90aI\xE8V[\x90Pa<\xF7H\x82aJAV[a=\x01\x90\x84aJ.V[\x92PP[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a=GW`@Q\x7F\x04\xD5P\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80QB` \x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x90\x1B\x16\x91\x81\x01\x91\x90\x91R`T\x81\x01\x86\x90R`t\x81\x01\x85\x90R`\x94\x81\x01\x84\x90R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x84\x90\x1B\x16`\xB4\x82\x01R_\x90`\xBC\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R`\x01T\x81Q` \x83\x01 \x7Fz\x88\xB1\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x86\x01R`$\x85\x01\x91\x90\x91R\x91\x93P_\x92\x91\x16\x90cz\x88\xB1\x07\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a>5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>Y\x91\x90aI\xE8V[\x90P\x80\x7F\xFFd\x90_s\xA6\x7F\xB5\x94\xE0\xF9@\xA8\x07Z\x86\r\xB4\x89\xAD\x99\x1E\x03/H\xC8\x11#\xEBR\xD6\x0B\x83`@Qa>\x8B\x91\x90aN\x84V[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[_a>\xA6\x82a:\x0EV[\x15\x80a)mWPPTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\x01\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91\x16\x10\x90V[_a?\x06\x83a>\xDA\x84aB\xAEV[`@\x80Q` \x80\x82\x01\x94\x90\x94R\x80\x82\x01\x92\x90\x92R\x80Q\x80\x83\x03\x82\x01\x81R``\x90\x92\x01\x90R\x80Q\x91\x01 \x90V[\x90\x93\x14\x93\x92PPPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R_a?>\x85`(aJ.V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a?\xC3W`@Q\x7FF4i\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R`D\x01a\t\"V[__a?\xCE\x86aA\xD9V[\x90\x92P\x90P\x86\x15a@\xDCWa?\xFD\x88\x88_\x81\x81\x10a?\xEEWa?\xEEaM:V[\x90P\x015`\xF8\x1C`\xF8\x1BaB\xD9V[a@TW\x87\x87_\x81\x81\x10a@\x13Wa@\x13aM:V[`@Q\x7Fk33V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x015`\x01`\x01`\xF8\x1B\x03\x19\x16`\x04\x83\x01RP`$\x01a\t\"V[`\x01`\xFF\x1B\x88\x88_\x81a@iWa@iaM:V[`\x01`\x01`\xF8\x1B\x03\x19\x92\x015\x92\x90\x92\x16\x16\x15\x80\x15\x91Pa@\x8AWP`!\x87\x10\x15[\x15a@\xDCW_a@\x9E`!`\x01\x8A\x8CaN\xB9V[a@\xA7\x91aN\xE0V[_\x81\x81R`\x08` R`@\x90 T\x90\x91P`\xFF\x16a@\xDAW`@Qb\xF2\x0C]`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\"V[P[\x81\x88\x88`@Q` \x01a@\xF1\x93\x92\x91\x90aN\xFDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81\x94P\x94PPPP[\x93P\x93\x91PPV[__\x88\x88\x11aA'W_aA1V[aA1\x89\x89aL\x01V[\x90P_\x89\x87\x11aAAW_aAKV[aAK\x8A\x88aL\x01V[\x90Pa'\x10aAZ\x85\x84aKIV[aAd\x91\x90aJAV[aAn\x90\x89aJ.V[\x97P_\x86\x82\x11aA~W_aA\x88V[aA\x88\x87\x83aL\x01V[\x90P\x82\x81\x11\x15aA\x95WP\x81[\x80\x89\x11\x15aA\xCAWaA\xA7\x81\x8AaL\x01V[\x98P\x86\x89\x11\x15aA\xCAW\x85\x89\x11aA\xBEW\x88aA\xC0V[\x85[\x93PPPPa6\xF5V[P\x94\x99\x98PPPPPPPPPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x80\x82\x01\x83\x90R\x91aB\x05aCkV[\x90P_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x88`@Q` \x01aB\x83\x95\x94\x93\x92\x91\x90`\xC0\x95\x86\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x82R\x94\x86\x1B\x85\x16`\x08\x82\x01R\x92\x85\x1B\x84\x16`\x10\x84\x01R\x90\x84\x1B\x83\x16`\x18\x83\x01R\x90\x92\x1B\x16` \x82\x01R`(\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`(\x81Q\x14aB\xA5WaB\xA5aO\x1BV[\x94\x90\x93P\x91PPV[_a)m\x82_\x01Q\x83` \x01Q\x84`@\x01Q\x85``\x01Q\x86`\x80\x01Q\x87`\xA0\x01Q\x88`\xC0\x01Qa6\\V[_`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x15\x80aB\xFEWP`\x01`\x01`\xF8\x1B\x03\x19\x82\x16`\x01`\xFF\x1B\x14[\x80aC2WP`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x7F\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14[\x80a)mWP`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x7F \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R____aC\xBDa5\xE6V[\x93P\x93P\x93P\x93P\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x15aC\xEEWaC\xE1\x82BaO/V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85R[aC\xF8\x81BaM\x1AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16` \x87\x01R\x84\x16C\x11\x15aD,WaD\x1C\x84CaO/V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x86\x01R[aD6\x83CaM\x1AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x86\x01RP\x92\x94\x93PPPPV[_` \x82\x84\x03\x12\x15aD_W__\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a1\x88W__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aD\xB1WaD\xB1aDzV[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aD\xB1WaD\xB1aDzV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aE\x03WaE\x03aDzV[`@R\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aE\"W__\xFD[\x91\x90PV[_``\x82\x84\x03\x12\x15aE7W__\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aEZWaEZaDzV[`@R\x90P\x80aEi\x83aE\x0BV[\x81RaEw` \x84\x01aE\x0BV[` \x82\x01RaE\x88`@\x84\x01aE\x0BV[`@\x82\x01RP\x92\x91PPV[___\x83\x85\x03a\x01\0\x81\x12\x15aE\xA8W__\xFD[\x845aE\xB3\x81aDfV[\x93P`\x80`\x1F\x19\x82\x01\x12\x15aE\xC6W__\xFD[P` \x84\x01\x91PaE\xDA\x85`\xA0\x86\x01aE'V[\x90P\x92P\x92P\x92V[\x80\x15\x15\x81\x14a1\x88W__\xFD[__`@\x83\x85\x03\x12\x15aF\x01W__\xFD[\x825aF\x0C\x81aDfV[\x91P` \x83\x015aF\x1C\x81aE\xE3V[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15aF7W__\xFD[\x815a)\xCD\x81aDfV[_``\x82\x84\x03\x12\x15aFRW__\xFD[a)\xCD\x83\x83aE'V[_____`\xA0\x86\x88\x03\x12\x15aFpW__\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015aF\x89\x81aDfV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[__\x83`\x1F\x84\x01\x12aF\xB1W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF\xC8W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aF\xDFW__\xFD[\x92P\x92\x90PV[_a\x01\0\x82\x84\x03\x12\x15aF\xF7W__\xFD[P\x91\x90PV[________a\x01\xC0\x89\x8B\x03\x12\x15aG\x15W__\xFD[\x885\x97P` \x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG2W__\xFD[aG>\x8B\x82\x8C\x01aF\xA1V[\x90\x98P\x96PP`@\x89\x015\x94P``\x89\x015aGY\x81aDfV[\x93P`\x80\x89\x015\x92P`\xA0\x89\x015\x91PaGv\x8A`\xC0\x8B\x01aF\xE6V[\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_____`\x80\x86\x88\x03\x12\x15aG\x99W__\xFD[\x855\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\xB6W__\xFD[aG\xC2\x88\x82\x89\x01aF\xA1V[\x90\x95P\x93PP`@\x86\x015\x91P``\x86\x015aG\xDD\x81aDfV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_______`\xC0\x88\x8A\x03\x12\x15aH\x01W__\xFD[\x875\x96P` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\x1EW__\xFD[aH*\x8A\x82\x8B\x01aF\xA1V[\x90\x97P\x95PP`@\x88\x015\x93P``\x88\x015aHE\x81aDfV[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\x80\x84\x015\x94P`\xA0\x90\x93\x015\x92\x91PPV[______a\x01\xA0\x87\x89\x03\x12\x15aHxW__\xFD[\x865\x95P` \x87\x015\x94P`@\x87\x015aH\x91\x81aDfV[\x93P``\x87\x015\x92P`\x80\x87\x015\x91PaH\xAE\x88`\xA0\x89\x01aF\xE6V[\x90P\x92\x95P\x92\x95P\x92\x95V[_`\x80\x82\x84\x03\x12\x80\x15aH\xCBW__\xFD[P`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aH\xEFWaH\xEFaDzV[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x83\x81\x015\x90\x82\x01R``\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[__` \x83\x85\x03\x12\x15aI.W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aIDW__\xFD[aIP\x85\x82\x86\x01aF\xA1V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15aIlW__\xFD[a)\xCD\x82aE\x0BV[\x805`\xFF\x81\x16\x81\x14aE\"W__\xFD[______`\xE0\x87\x89\x03\x12\x15aI\x9AW__\xFD[\x865\x95PaI\xAA` \x88\x01aIuV[\x94P`\x80\x87\x01\x88\x81\x11\x15aI\xBCW__\xFD[`@\x88\x01\x94P5\x92P`\xA0\x87\x015aI\xD3\x81aDfV[\x95\x98\x94\x97P\x92\x95\x91\x94\x93`\xC0\x90\x92\x015\x92PPV[_` \x82\x84\x03\x12\x15aI\xF8W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15aJ\x0FW__\xFD[\x81Qa)\xCD\x81aDfV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a)mWa)maJ\x1AV[_\x82aJ[WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01\x81[`\x01\x84\x11\x15aA\x10W\x80\x85\x04\x81\x11\x15aJ\x7FWaJ\x7FaJ\x1AV[`\x01\x84\x16\x15aJ\x8DW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aJdV[_\x82aJ\xA9WP`\x01a)mV[\x81aJ\xB5WP_a)mV[\x81`\x01\x81\x14aJ\xCBW`\x02\x81\x14aJ\xD5WaJ\xF1V[`\x01\x91PPa)mV[`\xFF\x84\x11\x15aJ\xE6WaJ\xE6aJ\x1AV[PP`\x01\x82\x1Ba)mV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aK\x14WP\x81\x81\na)mV[aK _\x19\x84\x84aJ`V[\x80_\x19\x04\x82\x11\x15aK3WaK3aJ\x1AV[\x02\x93\x92PPPV[_a)\xCD`\xFF\x84\x16\x83aJ\x9BV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a)mWa)maJ\x1AV[_` \x82\x84\x03\x12\x15aKpW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK\x86W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aK\x96W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK\xB0WaK\xB0aDzV[\x80`\x05\x1BaK\xC0` \x82\x01aD\xDAV[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x87\x84\x11\x15aK\xDBW__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15a6\xF5W\x84Q\x80\x83R` \x95\x86\x01\x95\x90\x93P\x90\x91\x01\x90aK\xE2V[\x81\x81\x03\x81\x81\x11\x15a)mWa)maJ\x1AV[_` \x82\x84\x03\x12\x15aL$W__\xFD[\x81Qa)\xCD\x81aE\xE3V[_\x81\x83\x03a\x01\0\x81\x12\x80\x15aLBW__\xFD[PaLKaD\x8EV[\x835\x81R`\xE0`\x1F\x19\x83\x01\x12\x15aL`W__\xFD[aLhaD\xB7V[\x91PaLv` \x85\x01aIuV[\x82R`@\x84\x015aL\x86\x81aDfV[` \x83\x01RaL\x97``\x85\x01aE\x0BV[`@\x83\x01RaL\xA8`\x80\x85\x01aE\x0BV[``\x83\x01R`\xA0\x84\x81\x015`\x80\x84\x01R`\xC0\x80\x86\x015\x91\x84\x01\x91\x90\x91R`\xE0\x90\x94\x015\x93\x82\x01\x93\x90\x93R` \x83\x01RP\x91\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a)mWa)maJ\x1AV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_`\xE0\x82\x01\x90P\x85\x82R\x84` \x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84Q\x16`@\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x85\x01Q\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x85\x01Q\x16`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x85\x01Q\x16`\xA0\x83\x01R`\x04\x83\x10aM\xC7WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x82`\xC0\x83\x01R\x95\x94PPPPPV[____`\x80\x85\x87\x03\x12\x15aM\xE9W__\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[\x81Q_\x90\x82\x90` \x85\x01\x83[\x82\x81\x10\x15aN3W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aN\x15V[P\x91\x95\x94PPPPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_aN`\x82\x86aN>V[`\x01`\x01`\xF8\x1B\x03\x19\x85\x16\x81RaNz`\x01\x82\x01\x85aN>V[\x96\x95PPPPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[__\x85\x85\x11\x15aN\xC7W__\xFD[\x83\x86\x11\x15aN\xD3W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a)mW_\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[_aO\x08\x82\x86aN>V[\x83\x85\x827_\x93\x01\x92\x83RP\x90\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a)mWa)maJ\x1AV\xFE\xA2dipfsX\"\x12 8\xE9\x9C\xF4\xEBB\xB1\xBD\t\x15D\xA0H,\x80\xAEV\x81\xAA\xFF\x1Es\xE5\xB8X\xEB\xA1K2\x89J\xEEdsolcC\0\x08\x1C\x003",
    );
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
    /**```solidity
struct DelayProof { bytes32 beforeDelayedAcc; Messages.Message delayedMessage; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelayProof {
        #[allow(missing_docs)]
        pub beforeDelayedAcc: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
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
    /**Custom error with signature `AlreadyInit()` and selector `0xef34ca5c`.
```solidity
error AlreadyInit();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyInit {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<AlreadyInit> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyInit) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyInit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyInit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyInit()";
            const SELECTOR: [u8; 4] = [239u8, 52u8, 202u8, 92u8];
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
        }
    };
    /**Custom error with signature `AlreadyValidDASKeyset(bytes32)` and selector `0xfa2fddda`.
```solidity
error AlreadyValidDASKeyset(bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyValidDASKeyset {
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
        impl ::core::convert::From<AlreadyValidDASKeyset> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyValidDASKeyset) -> Self {
                (value._0,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyValidDASKeyset {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { _0: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyValidDASKeyset {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyValidDASKeyset(bytes32)";
            const SELECTOR: [u8; 4] = [250u8, 47u8, 221u8, 218u8];
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
        }
    };
    /**Custom error with signature `BadBufferConfig()` and selector `0xda1c8eb6`.
```solidity
error BadBufferConfig();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BadBufferConfig {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<BadBufferConfig> for UnderlyingRustTuple<'_> {
            fn from(value: BadBufferConfig) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BadBufferConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BadBufferConfig {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BadBufferConfig()";
            const SELECTOR: [u8; 4] = [218u8, 28u8, 142u8, 182u8];
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
        }
    };
    /**Custom error with signature `BadMaxTimeVariation()` and selector `0x09cfba75`.
```solidity
error BadMaxTimeVariation();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BadMaxTimeVariation {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<BadMaxTimeVariation> for UnderlyingRustTuple<'_> {
            fn from(value: BadMaxTimeVariation) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BadMaxTimeVariation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BadMaxTimeVariation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BadMaxTimeVariation()";
            const SELECTOR: [u8; 4] = [9u8, 207u8, 186u8, 117u8];
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
        }
    };
    /**Custom error with signature `BadSequencerNumber(uint256,uint256)` and selector `0xac7411c9`.
```solidity
error BadSequencerNumber(uint256 stored, uint256 received);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BadSequencerNumber {
        #[allow(missing_docs)]
        pub stored: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub received: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<BadSequencerNumber> for UnderlyingRustTuple<'_> {
            fn from(value: BadSequencerNumber) -> Self {
                (value.stored, value.received)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BadSequencerNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    stored: tuple.0,
                    received: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BadSequencerNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BadSequencerNumber(uint256,uint256)";
            const SELECTOR: [u8; 4] = [172u8, 116u8, 17u8, 201u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.stored),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.received),
                )
            }
        }
    };
    /**Custom error with signature `DataBlobsNotSupported()` and selector `0x86657a53`.
```solidity
error DataBlobsNotSupported();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DataBlobsNotSupported {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<DataBlobsNotSupported> for UnderlyingRustTuple<'_> {
            fn from(value: DataBlobsNotSupported) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DataBlobsNotSupported {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DataBlobsNotSupported {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DataBlobsNotSupported()";
            const SELECTOR: [u8; 4] = [134u8, 101u8, 122u8, 83u8];
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
        }
    };
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
    /**Custom error with signature `DelayProofRequired()` and selector `0x0e5da8fb`.
```solidity
error DelayProofRequired();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelayProofRequired {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<DelayProofRequired> for UnderlyingRustTuple<'_> {
            fn from(value: DelayProofRequired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelayProofRequired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DelayProofRequired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DelayProofRequired()";
            const SELECTOR: [u8; 4] = [14u8, 93u8, 168u8, 251u8];
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
        }
    };
    /**Custom error with signature `DelayedBackwards()` and selector `0x7d73e6fa`.
```solidity
error DelayedBackwards();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelayedBackwards {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<DelayedBackwards> for UnderlyingRustTuple<'_> {
            fn from(value: DelayedBackwards) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelayedBackwards {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DelayedBackwards {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DelayedBackwards()";
            const SELECTOR: [u8; 4] = [125u8, 115u8, 230u8, 250u8];
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
        }
    };
    /**Custom error with signature `DelayedTooFar()` and selector `0x925f8bd3`.
```solidity
error DelayedTooFar();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelayedTooFar {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<DelayedTooFar> for UnderlyingRustTuple<'_> {
            fn from(value: DelayedTooFar) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelayedTooFar {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DelayedTooFar {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DelayedTooFar()";
            const SELECTOR: [u8; 4] = [146u8, 95u8, 139u8, 211u8];
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
        }
    };
    /**Custom error with signature `Deprecated()` and selector `0xc73b9d7c`.
```solidity
error Deprecated();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Deprecated {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<Deprecated> for UnderlyingRustTuple<'_> {
            fn from(value: Deprecated) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Deprecated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Deprecated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Deprecated()";
            const SELECTOR: [u8; 4] = [199u8, 59u8, 157u8, 124u8];
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
        }
    };
    /**Custom error with signature `ExtraGasNotUint64()` and selector `0x04d55012`.
```solidity
error ExtraGasNotUint64();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExtraGasNotUint64 {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<ExtraGasNotUint64> for UnderlyingRustTuple<'_> {
            fn from(value: ExtraGasNotUint64) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExtraGasNotUint64 {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExtraGasNotUint64 {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExtraGasNotUint64()";
            const SELECTOR: [u8; 4] = [4u8, 213u8, 80u8, 18u8];
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
        }
    };
    /**Custom error with signature `ForceIncludeBlockTooSoon()` and selector `0xad3515d9`.
```solidity
error ForceIncludeBlockTooSoon();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ForceIncludeBlockTooSoon {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<ForceIncludeBlockTooSoon>
        for UnderlyingRustTuple<'_> {
            fn from(value: ForceIncludeBlockTooSoon) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ForceIncludeBlockTooSoon {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ForceIncludeBlockTooSoon {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ForceIncludeBlockTooSoon()";
            const SELECTOR: [u8; 4] = [173u8, 53u8, 21u8, 217u8];
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
        }
    };
    /**Custom error with signature `HadZeroInit()` and selector `0x1ad0f743`.
```solidity
error HadZeroInit();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct HadZeroInit {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<HadZeroInit> for UnderlyingRustTuple<'_> {
            fn from(value: HadZeroInit) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for HadZeroInit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for HadZeroInit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "HadZeroInit()";
            const SELECTOR: [u8; 4] = [26u8, 208u8, 247u8, 67u8];
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
        }
    };
    /**Custom error with signature `IncorrectMessagePreimage()` and selector `0x13947fd7`.
```solidity
error IncorrectMessagePreimage();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IncorrectMessagePreimage {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<IncorrectMessagePreimage>
        for UnderlyingRustTuple<'_> {
            fn from(value: IncorrectMessagePreimage) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for IncorrectMessagePreimage {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IncorrectMessagePreimage {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IncorrectMessagePreimage()";
            const SELECTOR: [u8; 4] = [19u8, 148u8, 127u8, 215u8];
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
        }
    };
    /**Custom error with signature `InitParamZero(string)` and selector `0x80fc2c03`.
```solidity
error InitParamZero(string name);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InitParamZero {
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InitParamZero> for UnderlyingRustTuple<'_> {
            fn from(value: InitParamZero) -> Self {
                (value.name,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InitParamZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { name: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InitParamZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InitParamZero(string)";
            const SELECTOR: [u8; 4] = [128u8, 252u8, 44u8, 3u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `InvalidDelayedAccPreimage()` and selector `0xc334542d`.
```solidity
error InvalidDelayedAccPreimage();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidDelayedAccPreimage {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<InvalidDelayedAccPreimage>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidDelayedAccPreimage) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidDelayedAccPreimage {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidDelayedAccPreimage {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidDelayedAccPreimage()";
            const SELECTOR: [u8; 4] = [195u8, 52u8, 84u8, 45u8];
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
        }
    };
    /**Custom error with signature `InvalidHeaderFlag(bytes1)` and selector `0x6b333356`.
```solidity
error InvalidHeaderFlag(bytes1);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHeaderFlag {
        #[allow(missing_docs)]
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
        impl ::core::convert::From<InvalidHeaderFlag> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHeaderFlag) -> Self {
                (value._0,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidHeaderFlag {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { _0: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHeaderFlag {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHeaderFlag(bytes1)";
            const SELECTOR: [u8; 4] = [107u8, 51u8, 51u8, 86u8];
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
                        1,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
        }
    };
    /**Custom error with signature `KeysetTooLarge()` and selector `0xb3d1f412`.
```solidity
error KeysetTooLarge();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeysetTooLarge {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<KeysetTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: KeysetTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeysetTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeysetTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeysetTooLarge()";
            const SELECTOR: [u8; 4] = [179u8, 209u8, 244u8, 18u8];
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
        }
    };
    /**Custom error with signature `MissingDataHashes()` and selector `0x3cd27eb6`.
```solidity
error MissingDataHashes();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MissingDataHashes {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<MissingDataHashes> for UnderlyingRustTuple<'_> {
            fn from(value: MissingDataHashes) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MissingDataHashes {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MissingDataHashes {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MissingDataHashes()";
            const SELECTOR: [u8; 4] = [60u8, 210u8, 126u8, 182u8];
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
        }
    };
    /**Custom error with signature `NativeTokenMismatch()` and selector `0xc3e31f8d`.
```solidity
error NativeTokenMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NativeTokenMismatch {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<NativeTokenMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: NativeTokenMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NativeTokenMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NativeTokenMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NativeTokenMismatch()";
            const SELECTOR: [u8; 4] = [195u8, 227u8, 31u8, 141u8];
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
        }
    };
    /**Custom error with signature `NoSuchKeyset(bytes32)` and selector `0x00f20c5d`.
```solidity
error NoSuchKeyset(bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoSuchKeyset {
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
        impl ::core::convert::From<NoSuchKeyset> for UnderlyingRustTuple<'_> {
            fn from(value: NoSuchKeyset) -> Self {
                (value._0,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoSuchKeyset {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { _0: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoSuchKeyset {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoSuchKeyset(bytes32)";
            const SELECTOR: [u8; 4] = [0u8, 242u8, 12u8, 93u8];
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
        }
    };
    /**Custom error with signature `NotBatchPoster()` and selector `0x2dd9fc97`.
```solidity
error NotBatchPoster();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotBatchPoster {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<NotBatchPoster> for UnderlyingRustTuple<'_> {
            fn from(value: NotBatchPoster) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotBatchPoster {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotBatchPoster {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotBatchPoster()";
            const SELECTOR: [u8; 4] = [45u8, 217u8, 252u8, 151u8];
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
        }
    };
    /**Custom error with signature `NotBatchPosterManager(address)` and selector `0x660b3b42`.
```solidity
error NotBatchPosterManager(address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotBatchPosterManager {
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
        impl ::core::convert::From<NotBatchPosterManager> for UnderlyingRustTuple<'_> {
            fn from(value: NotBatchPosterManager) -> Self {
                (value._0,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotBatchPosterManager {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { _0: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotBatchPosterManager {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotBatchPosterManager(address)";
            const SELECTOR: [u8; 4] = [102u8, 11u8, 59u8, 66u8];
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
        }
    };
    /**Custom error with signature `NotCodelessOrigin()` and selector `0xc8958ead`.
```solidity
error NotCodelessOrigin();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotCodelessOrigin {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<NotCodelessOrigin> for UnderlyingRustTuple<'_> {
            fn from(value: NotCodelessOrigin) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotCodelessOrigin {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotCodelessOrigin {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotCodelessOrigin()";
            const SELECTOR: [u8; 4] = [200u8, 149u8, 142u8, 173u8];
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
        }
    };
    /**Custom error with signature `NotDelayBufferable()` and selector `0x8c959cc8`.
```solidity
error NotDelayBufferable();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotDelayBufferable {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<NotDelayBufferable> for UnderlyingRustTuple<'_> {
            fn from(value: NotDelayBufferable) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotDelayBufferable {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotDelayBufferable {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotDelayBufferable()";
            const SELECTOR: [u8; 4] = [140u8, 149u8, 156u8, 200u8];
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
        }
    };
    /**Custom error with signature `NotForked()` and selector `0xa301bb06`.
```solidity
error NotForked();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotForked {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<NotForked> for UnderlyingRustTuple<'_> {
            fn from(value: NotForked) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotForked {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotForked {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotForked()";
            const SELECTOR: [u8; 4] = [163u8, 1u8, 187u8, 6u8];
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
        }
    };
    /**Custom error with signature `NotOwner(address,address)` and selector `0x23295f0e`.
```solidity
error NotOwner(address sender, address owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotOwner {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<NotOwner> for UnderlyingRustTuple<'_> {
            fn from(value: NotOwner) -> Self {
                (value.sender, value.owner)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    sender: tuple.0,
                    owner: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotOwner(address,address)";
            const SELECTOR: [u8; 4] = [35u8, 41u8, 95u8, 14u8];
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
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `RollupNotChanged()` and selector `0xd054909f`.
```solidity
error RollupNotChanged();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RollupNotChanged {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<RollupNotChanged> for UnderlyingRustTuple<'_> {
            fn from(value: RollupNotChanged) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RollupNotChanged {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RollupNotChanged {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RollupNotChanged()";
            const SELECTOR: [u8; 4] = [208u8, 84u8, 144u8, 159u8];
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
event MaxTimeVariationSet(ISequencerInbox.MaxTimeVariation maxTimeVariation);
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
        pub maxTimeVariation: <ISequencerInbox::MaxTimeVariation as alloy::sol_types::SolType>::RustType,
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
            type DataTuple<'a> = (ISequencerInbox::MaxTimeVariation,);
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
                    <ISequencerInbox::MaxTimeVariation as alloy_sol_types::SolType>::tokenize(
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
    /**Constructor`.
```solidity
constructor(uint256 _maxDataSize, address reader4844_, bool _isUsingFeeToken, bool _isDelayBufferable);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _maxDataSize: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub reader4844_: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _isUsingFeeToken: bool,
        #[allow(missing_docs)]
        pub _isDelayBufferable: bool,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                bool,
                bool,
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
                    (
                        value._maxDataSize,
                        value.reader4844_,
                        value._isUsingFeeToken,
                        value._isDelayBufferable,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _maxDataSize: tuple.0,
                        reader4844_: tuple.1,
                        _isUsingFeeToken: tuple.2,
                        _isDelayBufferable: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
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
                    > as alloy_sol_types::SolType>::tokenize(&self._maxDataSize),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.reader4844_,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._isUsingFeeToken,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._isDelayBufferable,
                    ),
                )
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
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasRefunder: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasRefunder: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasRefunder: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasRefunder: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
function addSequencerL2BatchFromOrigin(uint256, bytes memory, uint256, address) external pure;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSequencerL2BatchFromOrigin_0Call {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _2: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _3: alloy::sol_types::private::Address,
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
                    (value._0, value._1, value._2, value._3)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSequencerL2BatchFromOrigin_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                        _3: tuple.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._2),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._3,
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
        #[allow(missing_docs)]
        pub sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasRefunder: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub sequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub afterDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasRefunder: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub prevMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newMessageCount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
    /**Function with signature `buffer()` and selector `0xedaafe20`.
```solidity
function buffer() external view returns (uint64 bufferBlocks, uint64 max, uint64 threshold, uint64 prevBlockNumber, uint64 replenishRateInBasis, uint64 prevSequencedBlockNumber);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bufferCall {}
    ///Container type for the return parameters of the [`buffer()`](bufferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bufferReturn {
        #[allow(missing_docs)]
        pub bufferBlocks: u64,
        #[allow(missing_docs)]
        pub max: u64,
        #[allow(missing_docs)]
        pub threshold: u64,
        #[allow(missing_docs)]
        pub prevBlockNumber: u64,
        #[allow(missing_docs)]
        pub replenishRateInBasis: u64,
        #[allow(missing_docs)]
        pub prevSequencedBlockNumber: u64,
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
            impl ::core::convert::From<bufferCall> for UnderlyingRustTuple<'_> {
                fn from(value: bufferCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bufferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, u64, u64, u64, u64, u64);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<bufferReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bufferReturn) -> Self {
                    (
                        value.bufferBlocks,
                        value.max,
                        value.threshold,
                        value.prevBlockNumber,
                        value.replenishRateInBasis,
                        value.prevSequencedBlockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bufferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        bufferBlocks: tuple.0,
                        max: tuple.1,
                        threshold: tuple.2,
                        prevBlockNumber: tuple.3,
                        replenishRateInBasis: tuple.4,
                        prevSequencedBlockNumber: tuple.5,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bufferCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bufferReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "buffer()";
            const SELECTOR: [u8; 4] = [237u8, 170u8, 254u8, 32u8];
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
function dasKeySetInfo(bytes32) external view returns (bool isValidKeyset, uint64 creationBlock);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct dasKeySetInfoCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`dasKeySetInfo(bytes32)`](dasKeySetInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct dasKeySetInfoReturn {
        #[allow(missing_docs)]
        pub isValidKeyset: bool,
        #[allow(missing_docs)]
        pub creationBlock: u64,
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
                    (value.isValidKeyset, value.creationBlock)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for dasKeySetInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        isValidKeyset: tuple.0,
                        creationBlock: tuple.1,
                    }
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
        #[allow(missing_docs)]
        pub _totalDelayedMessagesRead: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kind: u8,
        #[allow(missing_docs)]
        pub l1BlockAndTime: [u64; 2usize],
        #[allow(missing_docs)]
        pub baseFeeL1: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
function forceInclusionDeadline(uint64 blockNumber) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct forceInclusionDeadlineCall {
        #[allow(missing_docs)]
        pub blockNumber: u64,
    }
    ///Container type for the return parameters of the [`forceInclusionDeadline(uint64)`](forceInclusionDeadlineCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct forceInclusionDeadlineReturn {
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for forceInclusionDeadlineReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
        #[allow(missing_docs)]
        pub ksHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getKeysetCreationBlock(bytes32)`](getKeysetCreationBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeysetCreationBlockReturn {
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
    /**Function with signature `initialize(address,(uint256,uint256,uint256,uint256),(uint64,uint64,uint64))` and selector `0x1ad87e45`.
```solidity
function initialize(address bridge_, ISequencerInbox.MaxTimeVariation memory maxTimeVariation_, BufferConfig memory bufferConfig_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub bridge_: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub maxTimeVariation_: <ISequencerInbox::MaxTimeVariation as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
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
                ISequencerInbox::MaxTimeVariation,
                BufferConfig,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
                ISequencerInbox::MaxTimeVariation,
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
                    <ISequencerInbox::MaxTimeVariation as alloy_sol_types::SolType>::tokenize(
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
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isBatchPoster(address)`](isBatchPosterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isBatchPosterReturn {
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isSequencer(address)`](isSequencerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isSequencerReturn {
        #[allow(missing_docs)]
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
    /**Function with signature `isUsingFeeToken()` and selector `0x92d9f782`.
```solidity
function isUsingFeeToken() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isUsingFeeTokenCall {}
    ///Container type for the return parameters of the [`isUsingFeeToken()`](isUsingFeeTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isUsingFeeTokenReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<isUsingFeeTokenCall> for UnderlyingRustTuple<'_> {
                fn from(value: isUsingFeeTokenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isUsingFeeTokenCall {
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
            impl ::core::convert::From<isUsingFeeTokenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isUsingFeeTokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isUsingFeeTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isUsingFeeTokenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isUsingFeeTokenReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isUsingFeeToken()";
            const SELECTOR: [u8; 4] = [146u8, 217u8, 247u8, 130u8];
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
    /**Function with signature `isValidKeysetHash(bytes32)` and selector `0x1637be48`.
```solidity
function isValidKeysetHash(bytes32 ksHash) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidKeysetHashCall {
        #[allow(missing_docs)]
        pub ksHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isValidKeysetHash(bytes32)`](isValidKeysetHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidKeysetHashReturn {
        #[allow(missing_docs)]
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
function maxTimeVariation() external view returns (uint256, uint256, uint256, uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxTimeVariationCall {}
    ///Container type for the return parameters of the [`maxTimeVariation()`](maxTimeVariationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxTimeVariationReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _2: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _3: alloy::sol_types::private::primitives::aliases::U256,
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
                    (value._0, value._1, value._2, value._3)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maxTimeVariationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                        _3: tuple.3,
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
    /**Function with signature `postUpgradeInit((uint64,uint64,uint64))` and selector `0xa655d937`.
```solidity
function postUpgradeInit(BufferConfig memory bufferConfig_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postUpgradeInitCall {
        #[allow(missing_docs)]
        pub bufferConfig_: <BufferConfig as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`postUpgradeInit((uint64,uint64,uint64))`](postUpgradeInitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postUpgradeInitReturn {}
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
            type UnderlyingSolTuple<'a> = (BufferConfig,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<postUpgradeInitCall> for UnderlyingRustTuple<'_> {
                fn from(value: postUpgradeInitCall) -> Self {
                    (value.bufferConfig_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for postUpgradeInitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { bufferConfig_: tuple.0 }
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
            impl ::core::convert::From<postUpgradeInitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: postUpgradeInitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for postUpgradeInitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for postUpgradeInitCall {
            type Parameters<'a> = (BufferConfig,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = postUpgradeInitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "postUpgradeInit((uint64,uint64,uint64))";
            const SELECTOR: [u8; 4] = [166u8, 85u8, 217u8, 55u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
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
    /**Function with signature `reader4844()` and selector `0x8d910dde`.
```solidity
function reader4844() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reader4844Call {}
    ///Container type for the return parameters of the [`reader4844()`](reader4844Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reader4844Return {
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
            impl ::core::convert::From<reader4844Call> for UnderlyingRustTuple<'_> {
                fn from(value: reader4844Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reader4844Call {
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
            impl ::core::convert::From<reader4844Return> for UnderlyingRustTuple<'_> {
                fn from(value: reader4844Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reader4844Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for reader4844Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reader4844Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reader4844()";
            const SELECTOR: [u8; 4] = [141u8, 145u8, 13u8, 222u8];
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
        #[allow(missing_docs)]
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
    /**Function with signature `setBufferConfig((uint64,uint64,uint64))` and selector `0x2f3985a7`.
```solidity
function setBufferConfig(BufferConfig memory bufferConfig_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBufferConfigCall {
        #[allow(missing_docs)]
        pub bufferConfig_: <BufferConfig as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setBufferConfig((uint64,uint64,uint64))`](setBufferConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBufferConfigReturn {}
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
            type UnderlyingSolTuple<'a> = (BufferConfig,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<setBufferConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: setBufferConfigCall) -> Self {
                    (value.bufferConfig_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBufferConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { bufferConfig_: tuple.0 }
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
            impl ::core::convert::From<setBufferConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setBufferConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setBufferConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setBufferConfigCall {
            type Parameters<'a> = (BufferConfig,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setBufferConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setBufferConfig((uint64,uint64,uint64))";
            const SELECTOR: [u8; 4] = [47u8, 57u8, 133u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
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
    /**Function with signature `setIsBatchPoster(address,bool)` and selector `0x6e7df3e7`.
```solidity
function setIsBatchPoster(address addr, bool isBatchPoster_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setIsBatchPosterCall {
        #[allow(missing_docs)]
        pub addr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub addr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
function setMaxTimeVariation(ISequencerInbox.MaxTimeVariation memory maxTimeVariation_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMaxTimeVariationCall {
        #[allow(missing_docs)]
        pub maxTimeVariation_: <ISequencerInbox::MaxTimeVariation as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (ISequencerInbox::MaxTimeVariation,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISequencerInbox::MaxTimeVariation as alloy::sol_types::SolType>::RustType,
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
            type Parameters<'a> = (ISequencerInbox::MaxTimeVariation,);
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
                    <ISequencerInbox::MaxTimeVariation as alloy_sol_types::SolType>::tokenize(
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
        #[allow(missing_docs)]
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
    ///Container for all the [`SequencerInbox`](self) function calls.
    pub enum SequencerInboxCalls {
        #[allow(missing_docs)]
        BROTLI_MESSAGE_HEADER_FLAG(BROTLI_MESSAGE_HEADER_FLAGCall),
        #[allow(missing_docs)]
        DAS_MESSAGE_HEADER_FLAG(DAS_MESSAGE_HEADER_FLAGCall),
        #[allow(missing_docs)]
        DATA_AUTHENTICATED_FLAG(DATA_AUTHENTICATED_FLAGCall),
        #[allow(missing_docs)]
        DATA_BLOB_HEADER_FLAG(DATA_BLOB_HEADER_FLAGCall),
        #[allow(missing_docs)]
        HEADER_LENGTH(HEADER_LENGTHCall),
        #[allow(missing_docs)]
        TREE_DAS_MESSAGE_HEADER_FLAG(TREE_DAS_MESSAGE_HEADER_FLAGCall),
        #[allow(missing_docs)]
        ZERO_HEAVY_MESSAGE_HEADER_FLAG(ZERO_HEAVY_MESSAGE_HEADER_FLAGCall),
        #[allow(missing_docs)]
        addSequencerL2Batch(addSequencerL2BatchCall),
        #[allow(missing_docs)]
        addSequencerL2BatchDelayProof(addSequencerL2BatchDelayProofCall),
        #[allow(missing_docs)]
        addSequencerL2BatchFromBlobs(addSequencerL2BatchFromBlobsCall),
        #[allow(missing_docs)]
        addSequencerL2BatchFromBlobsDelayProof(
            addSequencerL2BatchFromBlobsDelayProofCall,
        ),
        #[allow(missing_docs)]
        addSequencerL2BatchFromOrigin_0(addSequencerL2BatchFromOrigin_0Call),
        #[allow(missing_docs)]
        addSequencerL2BatchFromOrigin_1(addSequencerL2BatchFromOrigin_1Call),
        #[allow(missing_docs)]
        addSequencerL2BatchFromOriginDelayProof(
            addSequencerL2BatchFromOriginDelayProofCall,
        ),
        #[allow(missing_docs)]
        batchCount(batchCountCall),
        #[allow(missing_docs)]
        batchPosterManager(batchPosterManagerCall),
        #[allow(missing_docs)]
        bridge(bridgeCall),
        #[allow(missing_docs)]
        buffer(bufferCall),
        #[allow(missing_docs)]
        dasKeySetInfo(dasKeySetInfoCall),
        #[allow(missing_docs)]
        forceInclusion(forceInclusionCall),
        #[allow(missing_docs)]
        forceInclusionDeadline(forceInclusionDeadlineCall),
        #[allow(missing_docs)]
        getKeysetCreationBlock(getKeysetCreationBlockCall),
        #[allow(missing_docs)]
        inboxAccs(inboxAccsCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        invalidateKeysetHash(invalidateKeysetHashCall),
        #[allow(missing_docs)]
        isBatchPoster(isBatchPosterCall),
        #[allow(missing_docs)]
        isDelayBufferable(isDelayBufferableCall),
        #[allow(missing_docs)]
        isSequencer(isSequencerCall),
        #[allow(missing_docs)]
        isUsingFeeToken(isUsingFeeTokenCall),
        #[allow(missing_docs)]
        isValidKeysetHash(isValidKeysetHashCall),
        #[allow(missing_docs)]
        maxDataSize(maxDataSizeCall),
        #[allow(missing_docs)]
        maxTimeVariation(maxTimeVariationCall),
        #[allow(missing_docs)]
        postUpgradeInit(postUpgradeInitCall),
        #[allow(missing_docs)]
        reader4844(reader4844Call),
        #[allow(missing_docs)]
        removeDelayAfterFork(removeDelayAfterForkCall),
        #[allow(missing_docs)]
        rollup(rollupCall),
        #[allow(missing_docs)]
        setBatchPosterManager(setBatchPosterManagerCall),
        #[allow(missing_docs)]
        setBufferConfig(setBufferConfigCall),
        #[allow(missing_docs)]
        setIsBatchPoster(setIsBatchPosterCall),
        #[allow(missing_docs)]
        setIsSequencer(setIsSequencerCall),
        #[allow(missing_docs)]
        setMaxTimeVariation(setMaxTimeVariationCall),
        #[allow(missing_docs)]
        setValidKeyset(setValidKeysetCall),
        #[allow(missing_docs)]
        totalDelayedMessagesRead(totalDelayedMessagesReadCall),
        #[allow(missing_docs)]
        updateRollupAddress(updateRollupAddressCall),
    }
    #[automatically_derived]
    impl SequencerInboxCalls {
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
            [47u8, 57u8, 133u8, 167u8],
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
            [141u8, 145u8, 13u8, 222u8],
            [143u8, 17u8, 31u8, 60u8],
            [145u8, 124u8, 248u8, 172u8],
            [146u8, 217u8, 247u8, 130u8],
            [150u8, 204u8, 92u8, 120u8],
            [166u8, 85u8, 217u8, 55u8],
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
            [237u8, 170u8, 254u8, 32u8],
            [241u8, 152u8, 21u8, 120u8],
            [246u8, 10u8, 80u8, 145u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SequencerInboxCalls {
        const NAME: &'static str = "SequencerInboxCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 44usize;
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
                Self::buffer(_) => <bufferCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::isUsingFeeToken(_) => {
                    <isUsingFeeTokenCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::postUpgradeInit(_) => {
                    <postUpgradeInitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reader4844(_) => {
                    <reader4844Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeDelayAfterFork(_) => {
                    <removeDelayAfterForkCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rollup(_) => <rollupCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setBatchPosterManager(_) => {
                    <setBatchPosterManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setBufferConfig(_) => {
                    <setBufferConfigCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<SequencerInboxCalls>] = &[
                {
                    fn ZERO_HEAVY_MESSAGE_HEADER_FLAG(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <ZERO_HEAVY_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::ZERO_HEAVY_MESSAGE_HEADER_FLAG)
                    }
                    ZERO_HEAVY_MESSAGE_HEADER_FLAG
                },
                {
                    fn batchCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <batchCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::batchCount)
                    }
                    batchCount
                },
                {
                    fn isValidKeysetHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <isValidKeysetHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::isValidKeysetHash)
                    }
                    isValidKeysetHash
                },
                {
                    fn BROTLI_MESSAGE_HEADER_FLAG(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <BROTLI_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::BROTLI_MESSAGE_HEADER_FLAG)
                    }
                    BROTLI_MESSAGE_HEADER_FLAG
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::initialize)
                    }
                    initialize
                },
                {
                    fn setIsSequencer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <setIsSequencerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::setIsSequencer)
                    }
                    setIsSequencer
                },
                {
                    fn setBatchPosterManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <setBatchPosterManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::setBatchPosterManager)
                    }
                    setBatchPosterManager
                },
                {
                    fn getKeysetCreationBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <getKeysetCreationBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::getKeysetCreationBlock)
                    }
                    getKeysetCreationBlock
                },
                {
                    fn HEADER_LENGTH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <HEADER_LENGTHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::HEADER_LENGTH)
                    }
                    HEADER_LENGTH
                },
                {
                    fn DATA_BLOB_HEADER_FLAG(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <DATA_BLOB_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::DATA_BLOB_HEADER_FLAG)
                    }
                    DATA_BLOB_HEADER_FLAG
                },
                {
                    fn setBufferConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <setBufferConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::setBufferConfig)
                    }
                    setBufferConfig
                },
                {
                    fn addSequencerL2BatchFromBlobs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <addSequencerL2BatchFromBlobsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::addSequencerL2BatchFromBlobs)
                    }
                    addSequencerL2BatchFromBlobs
                },
                {
                    fn isDelayBufferable(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <isDelayBufferableCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::isDelayBufferable)
                    }
                    isDelayBufferable
                },
                {
                    fn addSequencerL2BatchFromOriginDelayProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <addSequencerL2BatchFromOriginDelayProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SequencerInboxCalls::addSequencerL2BatchFromOriginDelayProof,
                            )
                    }
                    addSequencerL2BatchFromOriginDelayProof
                },
                {
                    fn updateRollupAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <updateRollupAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::updateRollupAddress)
                    }
                    updateRollupAddress
                },
                {
                    fn TREE_DAS_MESSAGE_HEADER_FLAG(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <TREE_DAS_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::TREE_DAS_MESSAGE_HEADER_FLAG)
                    }
                    TREE_DAS_MESSAGE_HEADER_FLAG
                },
                {
                    fn isSequencer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <isSequencerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::isSequencer)
                    }
                    isSequencer
                },
                {
                    fn addSequencerL2BatchDelayProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <addSequencerL2BatchDelayProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::addSequencerL2BatchDelayProof)
                    }
                    addSequencerL2BatchDelayProof
                },
                {
                    fn setIsBatchPoster(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <setIsBatchPosterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::setIsBatchPoster)
                    }
                    setIsBatchPoster
                },
                {
                    fn addSequencerL2BatchFromOrigin_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <addSequencerL2BatchFromOrigin_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::addSequencerL2BatchFromOrigin_0)
                    }
                    addSequencerL2BatchFromOrigin_0
                },
                {
                    fn dasKeySetInfo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <dasKeySetInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::dasKeySetInfo)
                    }
                    dasKeySetInfo
                },
                {
                    fn isBatchPoster(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <isBatchPosterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::isBatchPoster)
                    }
                    isBatchPoster
                },
                {
                    fn totalDelayedMessagesRead(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <totalDelayedMessagesReadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::totalDelayedMessagesRead)
                    }
                    totalDelayedMessagesRead
                },
                {
                    fn invalidateKeysetHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <invalidateKeysetHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::invalidateKeysetHash)
                    }
                    invalidateKeysetHash
                },
                {
                    fn reader4844(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <reader4844Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::reader4844)
                    }
                    reader4844
                },
                {
                    fn addSequencerL2BatchFromOrigin_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <addSequencerL2BatchFromOrigin_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::addSequencerL2BatchFromOrigin_1)
                    }
                    addSequencerL2BatchFromOrigin_1
                },
                {
                    fn addSequencerL2BatchFromBlobsDelayProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <addSequencerL2BatchFromBlobsDelayProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SequencerInboxCalls::addSequencerL2BatchFromBlobsDelayProof,
                            )
                    }
                    addSequencerL2BatchFromBlobsDelayProof
                },
                {
                    fn isUsingFeeToken(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <isUsingFeeTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::isUsingFeeToken)
                    }
                    isUsingFeeToken
                },
                {
                    fn removeDelayAfterFork(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <removeDelayAfterForkCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::removeDelayAfterFork)
                    }
                    removeDelayAfterFork
                },
                {
                    fn postUpgradeInit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <postUpgradeInitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::postUpgradeInit)
                    }
                    postUpgradeInit
                },
                {
                    fn setMaxTimeVariation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <setMaxTimeVariationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::setMaxTimeVariation)
                    }
                    setMaxTimeVariation
                },
                {
                    fn rollup(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <rollupCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::rollup)
                    }
                    rollup
                },
                {
                    fn batchPosterManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <batchPosterManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::batchPosterManager)
                    }
                    batchPosterManager
                },
                {
                    fn setValidKeyset(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <setValidKeysetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::setValidKeyset)
                    }
                    setValidKeyset
                },
                {
                    fn inboxAccs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <inboxAccsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::inboxAccs)
                    }
                    inboxAccs
                },
                {
                    fn forceInclusionDeadline(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <forceInclusionDeadlineCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::forceInclusionDeadline)
                    }
                    forceInclusionDeadline
                },
                {
                    fn addSequencerL2Batch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <addSequencerL2BatchCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::addSequencerL2Batch)
                    }
                    addSequencerL2Batch
                },
                {
                    fn DATA_AUTHENTICATED_FLAG(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <DATA_AUTHENTICATED_FLAGCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::DATA_AUTHENTICATED_FLAG)
                    }
                    DATA_AUTHENTICATED_FLAG
                },
                {
                    fn bridge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <bridgeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::bridge)
                    }
                    bridge
                },
                {
                    fn maxDataSize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <maxDataSizeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::maxDataSize)
                    }
                    maxDataSize
                },
                {
                    fn maxTimeVariation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <maxTimeVariationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::maxTimeVariation)
                    }
                    maxTimeVariation
                },
                {
                    fn buffer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <bufferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::buffer)
                    }
                    buffer
                },
                {
                    fn forceInclusion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <forceInclusionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::forceInclusion)
                    }
                    forceInclusion
                },
                {
                    fn DAS_MESSAGE_HEADER_FLAG(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxCalls> {
                        <DAS_MESSAGE_HEADER_FLAGCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxCalls::DAS_MESSAGE_HEADER_FLAG)
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
                Self::buffer(inner) => {
                    <bufferCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::isUsingFeeToken(inner) => {
                    <isUsingFeeTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::postUpgradeInit(inner) => {
                    <postUpgradeInitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::reader4844(inner) => {
                    <reader4844Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::setBufferConfig(inner) => {
                    <setBufferConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::buffer(inner) => {
                    <bufferCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::isUsingFeeToken(inner) => {
                    <isUsingFeeTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::postUpgradeInit(inner) => {
                    <postUpgradeInitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::reader4844(inner) => {
                    <reader4844Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setBufferConfig(inner) => {
                    <setBufferConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`SequencerInbox`](self) custom errors.
    pub enum SequencerInboxErrors {
        #[allow(missing_docs)]
        AlreadyInit(AlreadyInit),
        #[allow(missing_docs)]
        AlreadyValidDASKeyset(AlreadyValidDASKeyset),
        #[allow(missing_docs)]
        BadBufferConfig(BadBufferConfig),
        #[allow(missing_docs)]
        BadMaxTimeVariation(BadMaxTimeVariation),
        #[allow(missing_docs)]
        BadSequencerNumber(BadSequencerNumber),
        #[allow(missing_docs)]
        DataBlobsNotSupported(DataBlobsNotSupported),
        #[allow(missing_docs)]
        DataTooLarge(DataTooLarge),
        #[allow(missing_docs)]
        DelayProofRequired(DelayProofRequired),
        #[allow(missing_docs)]
        DelayedBackwards(DelayedBackwards),
        #[allow(missing_docs)]
        DelayedTooFar(DelayedTooFar),
        #[allow(missing_docs)]
        Deprecated(Deprecated),
        #[allow(missing_docs)]
        ExtraGasNotUint64(ExtraGasNotUint64),
        #[allow(missing_docs)]
        ForceIncludeBlockTooSoon(ForceIncludeBlockTooSoon),
        #[allow(missing_docs)]
        HadZeroInit(HadZeroInit),
        #[allow(missing_docs)]
        IncorrectMessagePreimage(IncorrectMessagePreimage),
        #[allow(missing_docs)]
        InitParamZero(InitParamZero),
        #[allow(missing_docs)]
        InvalidDelayedAccPreimage(InvalidDelayedAccPreimage),
        #[allow(missing_docs)]
        InvalidHeaderFlag(InvalidHeaderFlag),
        #[allow(missing_docs)]
        KeysetTooLarge(KeysetTooLarge),
        #[allow(missing_docs)]
        MissingDataHashes(MissingDataHashes),
        #[allow(missing_docs)]
        NativeTokenMismatch(NativeTokenMismatch),
        #[allow(missing_docs)]
        NoSuchKeyset(NoSuchKeyset),
        #[allow(missing_docs)]
        NotBatchPoster(NotBatchPoster),
        #[allow(missing_docs)]
        NotBatchPosterManager(NotBatchPosterManager),
        #[allow(missing_docs)]
        NotCodelessOrigin(NotCodelessOrigin),
        #[allow(missing_docs)]
        NotDelayBufferable(NotDelayBufferable),
        #[allow(missing_docs)]
        NotForked(NotForked),
        #[allow(missing_docs)]
        NotOwner(NotOwner),
        #[allow(missing_docs)]
        RollupNotChanged(RollupNotChanged),
    }
    #[automatically_derived]
    impl SequencerInboxErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 242u8, 12u8, 93u8],
            [4u8, 213u8, 80u8, 18u8],
            [9u8, 207u8, 186u8, 117u8],
            [14u8, 93u8, 168u8, 251u8],
            [19u8, 148u8, 127u8, 215u8],
            [26u8, 208u8, 247u8, 67u8],
            [35u8, 41u8, 95u8, 14u8],
            [45u8, 217u8, 252u8, 151u8],
            [60u8, 210u8, 126u8, 182u8],
            [70u8, 52u8, 105u8, 27u8],
            [102u8, 11u8, 59u8, 66u8],
            [107u8, 51u8, 51u8, 86u8],
            [125u8, 115u8, 230u8, 250u8],
            [128u8, 252u8, 44u8, 3u8],
            [134u8, 101u8, 122u8, 83u8],
            [140u8, 149u8, 156u8, 200u8],
            [146u8, 95u8, 139u8, 211u8],
            [163u8, 1u8, 187u8, 6u8],
            [172u8, 116u8, 17u8, 201u8],
            [173u8, 53u8, 21u8, 217u8],
            [179u8, 209u8, 244u8, 18u8],
            [195u8, 52u8, 84u8, 45u8],
            [195u8, 227u8, 31u8, 141u8],
            [199u8, 59u8, 157u8, 124u8],
            [200u8, 149u8, 142u8, 173u8],
            [208u8, 84u8, 144u8, 159u8],
            [218u8, 28u8, 142u8, 182u8],
            [239u8, 52u8, 202u8, 92u8],
            [250u8, 47u8, 221u8, 218u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SequencerInboxErrors {
        const NAME: &'static str = "SequencerInboxErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyInit(_) => {
                    <AlreadyInit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AlreadyValidDASKeyset(_) => {
                    <AlreadyValidDASKeyset as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BadBufferConfig(_) => {
                    <BadBufferConfig as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BadMaxTimeVariation(_) => {
                    <BadMaxTimeVariation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BadSequencerNumber(_) => {
                    <BadSequencerNumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DataBlobsNotSupported(_) => {
                    <DataBlobsNotSupported as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DataTooLarge(_) => {
                    <DataTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DelayProofRequired(_) => {
                    <DelayProofRequired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DelayedBackwards(_) => {
                    <DelayedBackwards as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DelayedTooFar(_) => {
                    <DelayedTooFar as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Deprecated(_) => {
                    <Deprecated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExtraGasNotUint64(_) => {
                    <ExtraGasNotUint64 as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ForceIncludeBlockTooSoon(_) => {
                    <ForceIncludeBlockTooSoon as alloy_sol_types::SolError>::SELECTOR
                }
                Self::HadZeroInit(_) => {
                    <HadZeroInit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IncorrectMessagePreimage(_) => {
                    <IncorrectMessagePreimage as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InitParamZero(_) => {
                    <InitParamZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidDelayedAccPreimage(_) => {
                    <InvalidDelayedAccPreimage as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHeaderFlag(_) => {
                    <InvalidHeaderFlag as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeysetTooLarge(_) => {
                    <KeysetTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MissingDataHashes(_) => {
                    <MissingDataHashes as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NativeTokenMismatch(_) => {
                    <NativeTokenMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoSuchKeyset(_) => {
                    <NoSuchKeyset as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotBatchPoster(_) => {
                    <NotBatchPoster as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotBatchPosterManager(_) => {
                    <NotBatchPosterManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotCodelessOrigin(_) => {
                    <NotCodelessOrigin as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotDelayBufferable(_) => {
                    <NotDelayBufferable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotForked(_) => <NotForked as alloy_sol_types::SolError>::SELECTOR,
                Self::NotOwner(_) => <NotOwner as alloy_sol_types::SolError>::SELECTOR,
                Self::RollupNotChanged(_) => {
                    <RollupNotChanged as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<SequencerInboxErrors>] = &[
                {
                    fn NoSuchKeyset(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <NoSuchKeyset as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::NoSuchKeyset)
                    }
                    NoSuchKeyset
                },
                {
                    fn ExtraGasNotUint64(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <ExtraGasNotUint64 as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::ExtraGasNotUint64)
                    }
                    ExtraGasNotUint64
                },
                {
                    fn BadMaxTimeVariation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <BadMaxTimeVariation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::BadMaxTimeVariation)
                    }
                    BadMaxTimeVariation
                },
                {
                    fn DelayProofRequired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <DelayProofRequired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::DelayProofRequired)
                    }
                    DelayProofRequired
                },
                {
                    fn IncorrectMessagePreimage(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <IncorrectMessagePreimage as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::IncorrectMessagePreimage)
                    }
                    IncorrectMessagePreimage
                },
                {
                    fn HadZeroInit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <HadZeroInit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::HadZeroInit)
                    }
                    HadZeroInit
                },
                {
                    fn NotOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <NotOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::NotOwner)
                    }
                    NotOwner
                },
                {
                    fn NotBatchPoster(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <NotBatchPoster as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::NotBatchPoster)
                    }
                    NotBatchPoster
                },
                {
                    fn MissingDataHashes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <MissingDataHashes as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::MissingDataHashes)
                    }
                    MissingDataHashes
                },
                {
                    fn DataTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <DataTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::DataTooLarge)
                    }
                    DataTooLarge
                },
                {
                    fn NotBatchPosterManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <NotBatchPosterManager as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::NotBatchPosterManager)
                    }
                    NotBatchPosterManager
                },
                {
                    fn InvalidHeaderFlag(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <InvalidHeaderFlag as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::InvalidHeaderFlag)
                    }
                    InvalidHeaderFlag
                },
                {
                    fn DelayedBackwards(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <DelayedBackwards as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::DelayedBackwards)
                    }
                    DelayedBackwards
                },
                {
                    fn InitParamZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <InitParamZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::InitParamZero)
                    }
                    InitParamZero
                },
                {
                    fn DataBlobsNotSupported(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <DataBlobsNotSupported as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::DataBlobsNotSupported)
                    }
                    DataBlobsNotSupported
                },
                {
                    fn NotDelayBufferable(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <NotDelayBufferable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::NotDelayBufferable)
                    }
                    NotDelayBufferable
                },
                {
                    fn DelayedTooFar(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <DelayedTooFar as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::DelayedTooFar)
                    }
                    DelayedTooFar
                },
                {
                    fn NotForked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <NotForked as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::NotForked)
                    }
                    NotForked
                },
                {
                    fn BadSequencerNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <BadSequencerNumber as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::BadSequencerNumber)
                    }
                    BadSequencerNumber
                },
                {
                    fn ForceIncludeBlockTooSoon(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <ForceIncludeBlockTooSoon as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::ForceIncludeBlockTooSoon)
                    }
                    ForceIncludeBlockTooSoon
                },
                {
                    fn KeysetTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <KeysetTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::KeysetTooLarge)
                    }
                    KeysetTooLarge
                },
                {
                    fn InvalidDelayedAccPreimage(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <InvalidDelayedAccPreimage as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::InvalidDelayedAccPreimage)
                    }
                    InvalidDelayedAccPreimage
                },
                {
                    fn NativeTokenMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <NativeTokenMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::NativeTokenMismatch)
                    }
                    NativeTokenMismatch
                },
                {
                    fn Deprecated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <Deprecated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::Deprecated)
                    }
                    Deprecated
                },
                {
                    fn NotCodelessOrigin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <NotCodelessOrigin as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::NotCodelessOrigin)
                    }
                    NotCodelessOrigin
                },
                {
                    fn RollupNotChanged(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <RollupNotChanged as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::RollupNotChanged)
                    }
                    RollupNotChanged
                },
                {
                    fn BadBufferConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <BadBufferConfig as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::BadBufferConfig)
                    }
                    BadBufferConfig
                },
                {
                    fn AlreadyInit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <AlreadyInit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::AlreadyInit)
                    }
                    AlreadyInit
                },
                {
                    fn AlreadyValidDASKeyset(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SequencerInboxErrors> {
                        <AlreadyValidDASKeyset as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SequencerInboxErrors::AlreadyValidDASKeyset)
                    }
                    AlreadyValidDASKeyset
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
                Self::AlreadyInit(inner) => {
                    <AlreadyInit as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::AlreadyValidDASKeyset(inner) => {
                    <AlreadyValidDASKeyset as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BadBufferConfig(inner) => {
                    <BadBufferConfig as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BadMaxTimeVariation(inner) => {
                    <BadMaxTimeVariation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BadSequencerNumber(inner) => {
                    <BadSequencerNumber as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DataBlobsNotSupported(inner) => {
                    <DataBlobsNotSupported as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DataTooLarge(inner) => {
                    <DataTooLarge as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::DelayProofRequired(inner) => {
                    <DelayProofRequired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DelayedBackwards(inner) => {
                    <DelayedBackwards as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DelayedTooFar(inner) => {
                    <DelayedTooFar as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::Deprecated(inner) => {
                    <Deprecated as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ExtraGasNotUint64(inner) => {
                    <ExtraGasNotUint64 as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ForceIncludeBlockTooSoon(inner) => {
                    <ForceIncludeBlockTooSoon as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::HadZeroInit(inner) => {
                    <HadZeroInit as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::IncorrectMessagePreimage(inner) => {
                    <IncorrectMessagePreimage as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InitParamZero(inner) => {
                    <InitParamZero as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidDelayedAccPreimage(inner) => {
                    <InvalidDelayedAccPreimage as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidHeaderFlag(inner) => {
                    <InvalidHeaderFlag as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KeysetTooLarge(inner) => {
                    <KeysetTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MissingDataHashes(inner) => {
                    <MissingDataHashes as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NativeTokenMismatch(inner) => {
                    <NativeTokenMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoSuchKeyset(inner) => {
                    <NoSuchKeyset as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotBatchPoster(inner) => {
                    <NotBatchPoster as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotBatchPosterManager(inner) => {
                    <NotBatchPosterManager as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotCodelessOrigin(inner) => {
                    <NotCodelessOrigin as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotDelayBufferable(inner) => {
                    <NotDelayBufferable as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotForked(inner) => {
                    <NotForked as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotOwner(inner) => {
                    <NotOwner as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::RollupNotChanged(inner) => {
                    <RollupNotChanged as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadyInit(inner) => {
                    <AlreadyInit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AlreadyValidDASKeyset(inner) => {
                    <AlreadyValidDASKeyset as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BadBufferConfig(inner) => {
                    <BadBufferConfig as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BadMaxTimeVariation(inner) => {
                    <BadMaxTimeVariation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BadSequencerNumber(inner) => {
                    <BadSequencerNumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DataBlobsNotSupported(inner) => {
                    <DataBlobsNotSupported as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DataTooLarge(inner) => {
                    <DataTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DelayProofRequired(inner) => {
                    <DelayProofRequired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DelayedBackwards(inner) => {
                    <DelayedBackwards as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DelayedTooFar(inner) => {
                    <DelayedTooFar as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Deprecated(inner) => {
                    <Deprecated as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ExtraGasNotUint64(inner) => {
                    <ExtraGasNotUint64 as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ForceIncludeBlockTooSoon(inner) => {
                    <ForceIncludeBlockTooSoon as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::HadZeroInit(inner) => {
                    <HadZeroInit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IncorrectMessagePreimage(inner) => {
                    <IncorrectMessagePreimage as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InitParamZero(inner) => {
                    <InitParamZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidDelayedAccPreimage(inner) => {
                    <InvalidDelayedAccPreimage as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHeaderFlag(inner) => {
                    <InvalidHeaderFlag as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KeysetTooLarge(inner) => {
                    <KeysetTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MissingDataHashes(inner) => {
                    <MissingDataHashes as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NativeTokenMismatch(inner) => {
                    <NativeTokenMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoSuchKeyset(inner) => {
                    <NoSuchKeyset as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotBatchPoster(inner) => {
                    <NotBatchPoster as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotBatchPosterManager(inner) => {
                    <NotBatchPosterManager as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotCodelessOrigin(inner) => {
                    <NotCodelessOrigin as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotDelayBufferable(inner) => {
                    <NotDelayBufferable as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotForked(inner) => {
                    <NotForked as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::NotOwner(inner) => {
                    <NotOwner as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::RollupNotChanged(inner) => {
                    <RollupNotChanged as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SequencerInbox`](self) events.
    pub enum SequencerInboxEvents {
        #[allow(missing_docs)]
        BatchPosterManagerSet(BatchPosterManagerSet),
        #[allow(missing_docs)]
        BatchPosterSet(BatchPosterSet),
        #[allow(missing_docs)]
        BufferConfigSet(BufferConfigSet),
        #[allow(missing_docs)]
        InboxMessageDelivered(InboxMessageDelivered),
        #[allow(missing_docs)]
        InboxMessageDeliveredFromOrigin(InboxMessageDeliveredFromOrigin),
        #[allow(missing_docs)]
        InvalidateKeyset(InvalidateKeyset),
        #[allow(missing_docs)]
        MaxTimeVariationSet(MaxTimeVariationSet),
        #[allow(missing_docs)]
        OwnerFunctionCalled(OwnerFunctionCalled),
        #[allow(missing_docs)]
        SequencerBatchData(SequencerBatchData),
        #[allow(missing_docs)]
        SequencerBatchDelivered(SequencerBatchDelivered),
        #[allow(missing_docs)]
        SequencerSet(SequencerSet),
        #[allow(missing_docs)]
        SetValidKeyset(SetValidKeyset),
    }
    #[automatically_derived]
    impl SequencerInboxEvents {
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
    impl alloy_sol_types::SolEventInterface for SequencerInboxEvents {
        const NAME: &'static str = "SequencerInboxEvents";
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
    impl alloy_sol_types::private::IntoLogData for SequencerInboxEvents {
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
    /**Creates a new wrapper around an on-chain [`SequencerInbox`](self) contract instance.

See the [wrapper's documentation](`SequencerInboxInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SequencerInboxInstance<T, P, N> {
        SequencerInboxInstance::<T, P, N>::new(address, provider)
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
        _maxDataSize: alloy::sol_types::private::primitives::aliases::U256,
        reader4844_: alloy::sol_types::private::Address,
        _isUsingFeeToken: bool,
        _isDelayBufferable: bool,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SequencerInboxInstance<T, P, N>>,
    > {
        SequencerInboxInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _maxDataSize,
            reader4844_,
            _isUsingFeeToken,
            _isDelayBufferable,
        )
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
        _maxDataSize: alloy::sol_types::private::primitives::aliases::U256,
        reader4844_: alloy::sol_types::private::Address,
        _isUsingFeeToken: bool,
        _isDelayBufferable: bool,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        SequencerInboxInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _maxDataSize,
            reader4844_,
            _isUsingFeeToken,
            _isDelayBufferable,
        )
    }
    /**A [`SequencerInbox`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SequencerInbox`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SequencerInboxInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SequencerInboxInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SequencerInboxInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SequencerInboxInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`SequencerInbox`](self) contract instance.

See the [wrapper's documentation](`SequencerInboxInstance`) for more details.*/
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
            _maxDataSize: alloy::sol_types::private::primitives::aliases::U256,
            reader4844_: alloy::sol_types::private::Address,
            _isUsingFeeToken: bool,
            _isDelayBufferable: bool,
        ) -> alloy_contract::Result<SequencerInboxInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _maxDataSize,
                reader4844_,
                _isUsingFeeToken,
                _isDelayBufferable,
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
            _maxDataSize: alloy::sol_types::private::primitives::aliases::U256,
            reader4844_: alloy::sol_types::private::Address,
            _isUsingFeeToken: bool,
            _isDelayBufferable: bool,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _maxDataSize,
                            reader4844_,
                            _isUsingFeeToken,
                            _isDelayBufferable,
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
    impl<T, P: ::core::clone::Clone, N> SequencerInboxInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SequencerInboxInstance<T, P, N> {
            SequencerInboxInstance {
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
    > SequencerInboxInstance<T, P, N> {
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
            _0: alloy::sol_types::private::primitives::aliases::U256,
            _1: alloy::sol_types::private::Bytes,
            _2: alloy::sol_types::private::primitives::aliases::U256,
            _3: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            addSequencerL2BatchFromOrigin_0Call,
            N,
        > {
            self.call_builder(
                &addSequencerL2BatchFromOrigin_0Call {
                    _0,
                    _1,
                    _2,
                    _3,
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
        ///Creates a new call builder for the [`buffer`] function.
        pub fn buffer(&self) -> alloy_contract::SolCallBuilder<T, &P, bufferCall, N> {
            self.call_builder(&bufferCall {})
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
            maxTimeVariation_: <ISequencerInbox::MaxTimeVariation as alloy::sol_types::SolType>::RustType,
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
        ///Creates a new call builder for the [`isUsingFeeToken`] function.
        pub fn isUsingFeeToken(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, isUsingFeeTokenCall, N> {
            self.call_builder(&isUsingFeeTokenCall {})
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
        ///Creates a new call builder for the [`postUpgradeInit`] function.
        pub fn postUpgradeInit(
            &self,
            bufferConfig_: <BufferConfig as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, postUpgradeInitCall, N> {
            self.call_builder(
                &postUpgradeInitCall {
                    bufferConfig_,
                },
            )
        }
        ///Creates a new call builder for the [`reader4844`] function.
        pub fn reader4844(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, reader4844Call, N> {
            self.call_builder(&reader4844Call {})
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
        ///Creates a new call builder for the [`setBufferConfig`] function.
        pub fn setBufferConfig(
            &self,
            bufferConfig_: <BufferConfig as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setBufferConfigCall, N> {
            self.call_builder(
                &setBufferConfigCall {
                    bufferConfig_,
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
            maxTimeVariation_: <ISequencerInbox::MaxTimeVariation as alloy::sol_types::SolType>::RustType,
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
    > SequencerInboxInstance<T, P, N> {
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
