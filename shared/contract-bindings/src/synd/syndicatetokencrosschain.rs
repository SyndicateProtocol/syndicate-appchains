///Module containing a contract's types and functions.
/**

```solidity
library Checkpoints {
    struct Checkpoint208 { uint48 _key; uint208 _value; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Checkpoints {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Checkpoint208 { uint48 _key; uint208 _value; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Checkpoint208 {
        #[allow(missing_docs)]
        pub _key: alloy::sol_types::private::primitives::aliases::U48,
        #[allow(missing_docs)]
        pub _value: alloy::sol_types::private::primitives::aliases::U208,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<48>,
            alloy::sol_types::sol_data::Uint<208>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U48,
            alloy::sol_types::private::primitives::aliases::U208,
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
        impl ::core::convert::From<Checkpoint208> for UnderlyingRustTuple<'_> {
            fn from(value: Checkpoint208) -> Self {
                (value._key, value._value)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Checkpoint208 {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    _key: tuple.0,
                    _value: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Checkpoint208 {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Checkpoint208 {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self._key),
                    <alloy::sol_types::sol_data::Uint<
                        208,
                    > as alloy_sol_types::SolType>::tokenize(&self._value),
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
        impl alloy_sol_types::SolType for Checkpoint208 {
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
        impl alloy_sol_types::SolStruct for Checkpoint208 {
            const NAME: &'static str = "Checkpoint208";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Checkpoint208(uint48 _key,uint208 _value)",
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
                        48,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self._key)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        208,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self._value)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Checkpoint208 {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust._key)
                    + <alloy::sol_types::sol_data::Uint<
                        208,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust._value,
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
                    48,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust._key,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    208,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust._value,
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
    /**Creates a new wrapper around an on-chain [`Checkpoints`](self) contract instance.

See the [wrapper's documentation](`CheckpointsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> CheckpointsInstance<T, P, N> {
        CheckpointsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`Checkpoints`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Checkpoints`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct CheckpointsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for CheckpointsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("CheckpointsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > CheckpointsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Checkpoints`](self) contract instance.

See the [wrapper's documentation](`CheckpointsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> CheckpointsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> CheckpointsInstance<T, P, N> {
            CheckpointsInstance {
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
    > CheckpointsInstance<T, P, N> {
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
    > CheckpointsInstance<T, P, N> {
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
library IBridgeRateLimiter {
    struct BridgeConfig { uint256 dailyMintLimit; uint256 dailyBurnLimit; bool isActive; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IBridgeRateLimiter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct BridgeConfig { uint256 dailyMintLimit; uint256 dailyBurnLimit; bool isActive; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BridgeConfig {
        #[allow(missing_docs)]
        pub dailyMintLimit: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub dailyBurnLimit: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub isActive: bool,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<BridgeConfig> for UnderlyingRustTuple<'_> {
            fn from(value: BridgeConfig) -> Self {
                (value.dailyMintLimit, value.dailyBurnLimit, value.isActive)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BridgeConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    dailyMintLimit: tuple.0,
                    dailyBurnLimit: tuple.1,
                    isActive: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BridgeConfig {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BridgeConfig {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.dailyMintLimit),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.dailyBurnLimit),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isActive,
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
        impl alloy_sol_types::SolType for BridgeConfig {
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
        impl alloy_sol_types::SolStruct for BridgeConfig {
            const NAME: &'static str = "BridgeConfig";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BridgeConfig(uint256 dailyMintLimit,uint256 dailyBurnLimit,bool isActive)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.dailyMintLimit,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.dailyBurnLimit,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.isActive,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BridgeConfig {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.dailyMintLimit,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.dailyBurnLimit,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.isActive,
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
                    &rust.dailyMintLimit,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.dailyBurnLimit,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.isActive,
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
    /**Creates a new wrapper around an on-chain [`IBridgeRateLimiter`](self) contract instance.

See the [wrapper's documentation](`IBridgeRateLimiterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IBridgeRateLimiterInstance<T, P, N> {
        IBridgeRateLimiterInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IBridgeRateLimiter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IBridgeRateLimiter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IBridgeRateLimiterInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IBridgeRateLimiterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IBridgeRateLimiterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IBridgeRateLimiterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IBridgeRateLimiter`](self) contract instance.

See the [wrapper's documentation](`IBridgeRateLimiterInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IBridgeRateLimiterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IBridgeRateLimiterInstance<T, P, N> {
            IBridgeRateLimiterInstance {
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
    > IBridgeRateLimiterInstance<T, P, N> {
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
    > IBridgeRateLimiterInstance<T, P, N> {
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
library Checkpoints {
    struct Checkpoint208 {
        uint48 _key;
        uint208 _value;
    }
}

library IBridgeRateLimiter {
    struct BridgeConfig {
        uint256 dailyMintLimit;
        uint256 dailyBurnLimit;
        bool isActive;
    }
}

interface SyndicateTokenCrosschain {
    error AccessControlBadConfirmation();
    error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
    error BridgeMustBeContract();
    error BridgeNotActive(address bridge);
    error BurnOnlyDuringLockPeriod();
    error CannotAddSelfAsBridge();
    error CheckpointUnorderedInsertion();
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC20ExceededSafeSupply(uint256 increasedSupply, uint256 cap);
    error ERC20InsufficientAllowance(address spender, uint256 allowance, uint256 needed);
    error ERC20InsufficientBalance(address sender, uint256 balance, uint256 needed);
    error ERC20InvalidApprover(address approver);
    error ERC20InvalidReceiver(address receiver);
    error ERC20InvalidSender(address sender);
    error ERC20InvalidSpender(address spender);
    error ERC2612ExpiredSignature(uint256 deadline);
    error ERC2612InvalidSigner(address signer, address owner);
    error ERC5805FutureLookup(uint256 timepoint, uint48 clock);
    error ERC6372InconsistentClock();
    error ExceedsTotalSupply();
    error InsufficientBurnLimit(address bridge, uint256 requested, uint256 available);
    error InsufficientEmissionBudget();
    error InsufficientMintLimit(address bridge, uint256 requested, uint256 available);
    error InvalidAccountNonce(address account, uint256 currentNonce);
    error InvalidShortString();
    error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
    error StringTooLong(string str);
    error TransfersLocked();
    error UnauthorizedBridge(address bridge);
    error UnlockTimestampInPast();
    error UnlockTimestampTooLate();
    error UnreasonableBurnLimit();
    error UnreasonableMintLimit();
    error VotesExpiredSignature(uint256 expiry);
    error ZeroAddress();
    error ZeroAmount();

    event Approval(address indexed owner, address indexed spender, uint256 value);
    event BridgeActiveStatusChanged(address indexed bridge, bool isActive);
    event BridgeAdded(address indexed bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit);
    event BridgeLimitsSet(address indexed bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit);
    event BridgeRemoved(address indexed bridge);
    event CrosschainBurn(address indexed from, uint256 amount, address indexed bridge);
    event CrosschainMint(address indexed to, uint256 amount, address indexed bridge);
    event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);
    event DelegateVotesChanged(address indexed delegate, uint256 previousVotes, uint256 newVotes);
    event EIP712DomainChanged();
    event EmissionBudgetAllocated(address indexed bridge, uint256 amount);
    event EmissionBudgetConsumed(address indexed bridge, uint256 amount);
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event TokensBurnedByManager(address indexed from, uint256 amount, address indexed burner);
    event Transfer(address indexed from, address indexed to, uint256 value);
    event UnlockTimestampUpdated(uint256 oldTimestamp, uint256 newTimestamp, address indexed updatedBy);

    constructor(address defaultAdmin, address syndTreasuryAddress);

    function AIRDROP_MANAGER_ROLE() external view returns (bytes32);
    function BRIDGE_MANAGER_ROLE() external view returns (bytes32);
    function CLOCK_MODE() external view returns (string memory);
    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function DOMAIN_SEPARATOR() external view returns (bytes32);
    function EMISSION_BUDGET_MANAGER_ROLE() external view returns (bytes32);
    function EMISSION_MINTER_ROLE() external view returns (bytes32);
    function INITIAL_MINT_SUPPLY() external view returns (uint256);
    function MAX_LOCK_DURATION() external view returns (uint256);
    function TOTAL_SUPPLY() external view returns (uint256);
    function allocateEmissionBudget(address bridge, uint256 amount) external;
    function allowance(address owner, address spender) external view returns (uint256);
    function approve(address spender, uint256 value) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
    function bridgeConfigs(address) external view returns (uint256 dailyMintLimit, uint256 dailyBurnLimit, bool isActive);
    function bridgeEmissionBudgets(address) external view returns (uint256);
    function burn(uint256 amount) external;
    function burnFrom(address from, uint256 amount) external;
    function checkpoints(address account, uint32 pos) external view returns (Checkpoints.Checkpoint208 memory);
    function clock() external view returns (uint48);
    function crosschainBurn(address from, uint256 amount) external;
    function crosschainMint(address to, uint256 amount) external;
    function decimals() external view returns (uint8);
    function delegate(address delegatee) external;
    function delegateBySig(address delegatee, uint256 nonce, uint256 expiry, uint8 v, bytes32 r, bytes32 s) external;
    function delegates(address account) external view returns (address);
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getAllBridges() external view returns (address[] memory allBridges);
    function getAvailableBurnLimit(address bridge) external view returns (uint256 available);
    function getAvailableMintLimit(address bridge) external view returns (uint256 available);
    function getBridgeAtIndex(uint256 index) external view returns (address bridge);
    function getBridgeConfig(address bridge) external view returns (IBridgeRateLimiter.BridgeConfig memory config);
    function getBridgeCount() external view returns (uint256 count);
    function getCurrentTotalSupply() external view returns (uint256);
    function getEmissionBudget(address bridge) external view returns (uint256 budget);
    function getPastTotalSupply(uint256 timepoint) external view returns (uint256);
    function getPastVotes(address account, uint256 timepoint) external view returns (uint256);
    function getPastVotingPower(address account, uint256 blockNumber) external view returns (uint256);
    function getRemainingLockTime() external view returns (uint256);
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    function getVotes(address account) external view returns (uint256);
    function getVotingPower(address account) external view returns (uint256);
    function grantRole(bytes32 role, address account) external;
    function hasRole(bytes32 role, address account) external view returns (bool);
    function hourlyBurnUsage(address, uint256) external view returns (uint256);
    function hourlyMintUsage(address, uint256) external view returns (uint256);
    function isBridgeAuthorized(address bridge) external view returns (bool authorized);
    function maxLockTimestamp() external view returns (uint256);
    function mint(address to, uint256 amount) external;
    function name() external view returns (string memory);
    function nonces(address owner) external view returns (uint256);
    function numCheckpoints(address account) external view returns (uint32);
    function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
    function removeBridge(address bridge) external;
    function renounceRole(bytes32 role, address callerConfirmation) external;
    function revokeRole(bytes32 role, address account) external;
    function setBridgeActive(address bridge, bool isActive) external;
    function setBridgeLimits(address bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit) external;
    function setUnlockTimestamp(uint256 newUnlockTimestamp) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function symbol() external view returns (string memory);
    function totalSupply() external view returns (uint256);
    function transfer(address to, uint256 value) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
    function transfersLocked() external view returns (bool);
    function unlockTimestamp() external view returns (uint256);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "defaultAdmin",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "syndTreasuryAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "AIRDROP_MANAGER_ROLE",
    "inputs": [],
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
    "name": "BRIDGE_MANAGER_ROLE",
    "inputs": [],
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
    "name": "CLOCK_MODE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DEFAULT_ADMIN_ROLE",
    "inputs": [],
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
    "name": "DOMAIN_SEPARATOR",
    "inputs": [],
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
    "name": "EMISSION_BUDGET_MANAGER_ROLE",
    "inputs": [],
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
    "name": "EMISSION_MINTER_ROLE",
    "inputs": [],
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
    "name": "INITIAL_MINT_SUPPLY",
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
    "name": "MAX_LOCK_DURATION",
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
    "name": "TOTAL_SUPPLY",
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
    "name": "allocateEmissionBudget",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "allowance",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
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
    "name": "approve",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "balanceOf",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
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
    "name": "bridgeConfigs",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "dailyMintLimit",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "dailyBurnLimit",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "isActive",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "bridgeEmissionBudgets",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "burn",
    "inputs": [
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "burnFrom",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "checkpoints",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "pos",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Checkpoints.Checkpoint208",
        "components": [
          {
            "name": "_key",
            "type": "uint48",
            "internalType": "uint48"
          },
          {
            "name": "_value",
            "type": "uint208",
            "internalType": "uint208"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "clock",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint48",
        "internalType": "uint48"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "crosschainBurn",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "crosschainMint",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "decimals",
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
    "name": "delegate",
    "inputs": [
      {
        "name": "delegatee",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegateBySig",
    "inputs": [
      {
        "name": "delegatee",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "nonce",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "v",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "r",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegates",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
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
    "name": "eip712Domain",
    "inputs": [],
    "outputs": [
      {
        "name": "fields",
        "type": "bytes1",
        "internalType": "bytes1"
      },
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "version",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "verifyingContract",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "extensions",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllBridges",
    "inputs": [],
    "outputs": [
      {
        "name": "allBridges",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAvailableBurnLimit",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "available",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAvailableMintLimit",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "available",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getBridgeAtIndex",
    "inputs": [
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getBridgeConfig",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "config",
        "type": "tuple",
        "internalType": "struct IBridgeRateLimiter.BridgeConfig",
        "components": [
          {
            "name": "dailyMintLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "dailyBurnLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "isActive",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getBridgeCount",
    "inputs": [],
    "outputs": [
      {
        "name": "count",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCurrentTotalSupply",
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
    "name": "getEmissionBudget",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "budget",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPastTotalSupply",
    "inputs": [
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "getPastVotes",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "getPastVotingPower",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "blockNumber",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "getRemainingLockTime",
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
    "name": "getRoleAdmin",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "getVotes",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
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
    "name": "getVotingPower",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
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
    "name": "grantRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "hasRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
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
    "name": "hourlyBurnUsage",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "hourlyMintUsage",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "isBridgeAuthorized",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "authorized",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "maxLockTimestamp",
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
    "name": "mint",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "name",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nonces",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
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
    "name": "numCheckpoints",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "permit",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "v",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "r",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeBridge",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "callerConfirmation",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "revokeRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setBridgeActive",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "isActive",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setBridgeLimits",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "dailyMintLimit",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "dailyBurnLimit",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setUnlockTimestamp",
    "inputs": [
      {
        "name": "newUnlockTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
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
    "name": "symbol",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "totalSupply",
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
    "name": "transfer",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferFrom",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transfersLocked",
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
    "name": "unlockTimestamp",
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
    "name": "Approval",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BridgeActiveStatusChanged",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "isActive",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BridgeAdded",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "dailyMintLimit",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "dailyBurnLimit",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BridgeLimitsSet",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "dailyMintLimit",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "dailyBurnLimit",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BridgeRemoved",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CrosschainBurn",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "bridge",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CrosschainMint",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "bridge",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DelegateChanged",
    "inputs": [
      {
        "name": "delegator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "fromDelegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "toDelegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DelegateVotesChanged",
    "inputs": [
      {
        "name": "delegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "previousVotes",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newVotes",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EIP712DomainChanged",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EmissionBudgetAllocated",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EmissionBudgetConsumed",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleAdminChanged",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "previousAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "newAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleGranted",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleRevoked",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TokensBurnedByManager",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "burner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Transfer",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UnlockTimestampUpdated",
    "inputs": [
      {
        "name": "oldTimestamp",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newTimestamp",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "updatedBy",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AccessControlBadConfirmation",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AccessControlUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "neededRole",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "BridgeMustBeContract",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BridgeNotActive",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "BurnOnlyDuringLockPeriod",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotAddSelfAsBridge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CheckpointUnorderedInsertion",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureLength",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureS",
    "inputs": [
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20ExceededSafeSupply",
    "inputs": [
      {
        "name": "increasedSupply",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "cap",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InsufficientAllowance",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "allowance",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "needed",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InsufficientBalance",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "balance",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "needed",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InvalidApprover",
    "inputs": [
      {
        "name": "approver",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InvalidReceiver",
    "inputs": [
      {
        "name": "receiver",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InvalidSender",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InvalidSpender",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC2612ExpiredSignature",
    "inputs": [
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC2612InvalidSigner",
    "inputs": [
      {
        "name": "signer",
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
    "name": "ERC5805FutureLookup",
    "inputs": [
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "clock",
        "type": "uint48",
        "internalType": "uint48"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC6372InconsistentClock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExceedsTotalSupply",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientBurnLimit",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "requested",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "available",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InsufficientEmissionBudget",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientMintLimit",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "requested",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "available",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidAccountNonce",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "currentNonce",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidShortString",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SafeCastOverflowedUintDowncast",
    "inputs": [
      {
        "name": "bits",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "StringTooLong",
    "inputs": [
      {
        "name": "str",
        "type": "string",
        "internalType": "string"
      }
    ]
  },
  {
    "type": "error",
    "name": "TransfersLocked",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnauthorizedBridge",
    "inputs": [
      {
        "name": "bridge",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "UnlockTimestampInPast",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnlockTimestampTooLate",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnreasonableBurnLimit",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnreasonableMintLimit",
    "inputs": []
  },
  {
    "type": "error",
    "name": "VotesExpiredSignature",
    "inputs": [
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ZeroAddress",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroAmount",
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
pub mod SyndicateTokenCrosschain {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101806040523461007d5761001b6100156100e2565b9061011e565b6040516140d69081610f3e8239608051816131d5015260a05181613292015260c051816131a6015260e051816132240152610100518161324a015261012051816119550152610140518161197e01526101605181818161183801526118810152f35b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b601f909101601f19168101906001600160401b038211908210176100b857604052565b610081565b604051906100cc604083610095565b565b51906001600160a01b038216820361007d57565b6150d4906040823803928382519485926100fc8285610095565b83398101031261007d5761011b6020610114846100ce565b93016100ce565b90565b610126610323565b61012e610323565b906101376102f9565b906314d6539160e21b602083015261014d61030e565b603160f81b60208201908152845190949193916001600160401b0382116100b8576101828261017d600354610373565b6103ab565b602090601f83116001146102725791806101b6926101be95945f92610267575b50508160011b915f199060031b1c19161790565b60035561044a565b6101c7816107bc565b610120526101d4826108ae565b610140526020815191012060e052519020610100524660a0526101f56109a0565b6080523060c0526001600160a01b03811615610258576001600160a01b03821615610258576102466102559261022a4261035e565b610160526102375f600c55565b61024083610523565b506106cf565b61024f81610599565b50610634565b50565b63d92e233d60e01b5f5260045ffd5b015190505f806101a2565b60035f52601f19831691907fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b925f5b8181106102e157509160019391856101be979694106102c9575b505050811b0160035561044a565b01515f1960f88460031b161c191690555f80806102bb565b929360206001819287860151815501950193016102a1565b60405190610308604083610095565b60048252565b6040519061031d604083610095565b60018252565b60405190610332604083610095565b600982526853796e64696361746560b81b6020830152565b634e487b7160e01b5f52601160045260245ffd5b90629e3400820180921161036e57565b61034a565b90600182811c921680156103a1575b602083101461038d57565b634e487b7160e01b5f52602260045260245ffd5b91607f1691610382565b601f81116103b7575050565b60035f5260205f20906020601f840160051c830193106103f1575b601f0160051c01905b8181106103e6575050565b5f81556001016103db565b90915081906103d2565b601f821161040857505050565b5f5260205f20906020601f840160051c83019310610440575b601f0160051c01905b818110610435575050565b5f815560010161042a565b9091508190610421565b80519091906001600160401b0381116100b8576104738161046c600454610373565b60046103fb565b602092601f82116001146104a7576104a2929382915f926102675750508160011b915f199060031b1c19161790565b600455565b60045f52601f198216937f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b915f5b86811061050b57508360019596106104f3575b505050811b01600455565b01515f1960f88460031b161c191690555f80806104e8565b919260206001819286850151815501940192016104d5565b6001600160a01b0381165f9081525f5160206150945f395f51905f52602052604090205460ff16610594576001600160a01b03165f8181525f5160206150945f395f51905f5260205260408120805460ff191660011790553391905f5160206150145f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f5160206150545f395f51905f52602052604090205460ff16610594576001600160a01b0381165f9081525f5160206150545f395f51905f5260205260409020805460ff1916600117905533906001600160a01b03167fcce296b040332a080e6df13515a3ec2869e21cd20f7344af0987ddb938d8bd215f5160206150145f395f51905f525f80a4600190565b6001600160a01b0381165f9081525f5160206150345f395f51905f52602052604090205460ff16610594576001600160a01b0381165f9081525f5160206150345f395f51905f5260205260409020805460ff1916600117905533906001600160a01b03167f820372a9faf82db3cc5fc36ffab5f096eef69b95fbf50591e0d71447aa1ba7005f5160206150145f395f51905f525f80a4600190565b6001600160a01b03811680156107a9576002546b02f90193ef3075fa98000000810180911161036e576002556001600160a01b0382165f9081526020819052604090206b02f90193ef3075fa9800000081540190555f7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef604051806107656b02f90193ef3075fa98000000829190602083019252565b0390a36002546001600160d01b03908181116107945750506b02f90193ef3075fa980000006100cc915f610a69565b630e58ae9360e11b5f5260045260245260445ffd5b63ec442f0560e01b5f525f60045260245ffd5b908151602081105f146107d457509061011b906109fe565b6001600160401b0381116100b8576107f8816107f1600654610373565b60066103fb565b602092601f821160011461082f57610827929382915f926102675750508160011b915f199060031b1c19161790565b60065560ff90565b60065f52601f198216937ff652222313e28459528d920b65115c16c04f3efc82aaedc97be59f3f377c0d3f915f5b868110610896575083600195961061087e575b505050811b0160065560ff90565b01515f1960f88460031b161c191690555f8080610870565b9192602060018192868501518155019401920161085d565b908151602081105f146108c657509061011b906109fe565b6001600160401b0381116100b8576108ea816108e3600754610373565b60076103fb565b602092601f821160011461092157610919929382915f926102675750508160011b915f199060031b1c19161790565b60075560ff90565b60075f52601f198216937fa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c688915f5b8681106109885750836001959610610970575b505050811b0160075560ff90565b01515f1960f88460031b161c191690555f8080610962565b9192602060018192868501518155019401920161094f565b60e051610100516040519060208201927f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8452604083015260608201524660808201523060a082015260a081526109f860c082610095565b51902090565b601f815111610a29576020815191015160208210610a1a571790565b5f198260200360031b1b161790565b604460209160405192839163305a27a960e01b83528160048401528051918291826024860152018484015e5f828201840152601f01601f19168101030190fd5b9091906001600160a01b03168015610ad0575b6100cc926001600160a01b0316908115610ab8575b5f90815260096020526040808220549282529020546001600160a01b039081169116610c9a565b610ac9610ac484610b6b565b610b9c565b5050610a91565b610ad982610b6b565b9265ffffffffffff4311610b5357600b5480610b1d5750610b13610b036100cc955f5b6001610ee1565b65ffffffffffff4316600b610e0b565b9050509250610a7c565b93845f1981011161036e57600b5f525f5160206150745f395f51905f52909401546100cc94610b1391610b03919060301c610afc565b6306dfcc6560e41b5f5260306004524360245260445ffd5b6001600160d01b038111610b85576001600160d01b031690565b6306dfcc6560e41b5f5260d060045260245260445ffd5b65ffffffffffff4311610b5357600b5480610bc65750610b03610bc2915f5b6002610ee1565b9091565b805f1981011161036e57600b5f525f5160206150745f395f51905f520154610bc291610b039160301c610bbb565b65ffffffffffff4311610b5357805480610c285750610c18610bc2925f6002610ee1565b9065ffffffffffff431690610e0b565b805f1981011161036e575f82815260209020015f190154610bc292610c189160301c610bbb565b65ffffffffffff4311610b5357805480610c735750610c18610bc2925f6001610ee1565b805f1981011161036e575f82815260209020015f190154610bc292610c189160301c610afc565b6001600160a01b03808316939291908116908185141580610d8d575b610cc2575b5050505050565b81610d33575b505082610cd7575b8080610cbb565b6001600160a01b03165f908152600a602052604090205f5160206150b45f395f51905f5291610d1091610d0a9091610b6b565b90610c4f565b604080516001600160d01b039384168152919092166020820152a25f8080610cd0565b6001600160a01b03165f908152600a602052604090205f5160206150b45f395f51905f5290610d6b90610d6586610b6b565b90610bf4565b604080516001600160d01b039384168152919092166020820152a25f80610cc8565b50831515610cb6565b5f1981019190821161036e57565b908154680100000000000000008110156100b85760018101808455811015610df7575f9283526020928390208251929093015160301b65ffffffffffff191665ffffffffffff9290921691909117910155565b634e487b7160e01b5f52603260045260245ffd5b80549293928015610eb757610e22610e2d91610d96565b825f5260205f200190565b8054603081901c9365ffffffffffff91821692918116808411610ea857879303610e745750610e7092509065ffffffffffff82549181199060301b169116179055565b9190565b915050610e7091610e94610e866100bd565b65ffffffffffff9093168352565b6001600160d01b0386166020830152610da4565b632520601d60e01b5f5260045ffd5b5090610edc91610ec8610e866100bd565b6001600160d01b0385166020830152610da4565b5f9190565b91909180600114610f2357600214610f0757634e487b7160e01b5f52605160045260245ffd5b6001600160d01b039081169181169190910390811161036e5790565b506001600160d01b039182169082160190811161036e579056fe60806040526004361015610011575f80fd5b5f3560e01c806301042d7a1461042057806301ffc9a71461041b57806304df017d14610416578063050732fb146103f857806306fdde0314610411578063095ea7b31461040c57806318160ddd1461031757806318bf50771461040757806323b872dd14610402578063248a9ca3146103fd5780632869366b146103f85780632b8c49e3146103f35780632f2ff15d146103ee57806330d3e8eb146103e9578063313ce567146103e45780633644e515146103df57806336568abe146103da5780633a46b1a81461032657806340c10f19146103d5578063427ac0ca146103d057806342966c68146103cb5780634bf5d7e9146103c65780634f1bfc9e146103c1578063587cde1e146103bc5780635a4239e9146103b75780635a5db1bb146103b25780635c19a95c146103ad5780635d4c6285146103a857806363a0daac146103a3578063651455341461039e5780636fcfff451461039957806370a082311461039457806372cbdcc81461038f57806378fb7fd21461038a57806379cc6790146103855780637a8cd156146103805780637ecebe001461037b57806383f1211b146103765780638426adf214610371578063844c90261461036c57806384b0196e146103675780638a542521146103625780638d3343d61461035d5780638e539e8c14610358578063902d55a51461035357806391d148541461034e57806391ddadf41461034957806394aa22f21461034457806395d89b411461033f5780639ab24eb01461031c5780639b7ef64b1461033a578063a217fddf14610335578063a9059cbb14610330578063aa082a9d1461032b578063b0ca253e14610326578063b7cdc61c14610321578063bb4d44361461031c578063c02ae75414610317578063c3cda52014610312578063c4fc45a81461030d578063c9ab000614610308578063d505accf14610303578063d547741f146102fe578063dd62ed3e146102f9578063f1127ed8146102f45763f75e8512146102ef575f80fd5b61245f565b612396565b61233d565b6122ff565b6121a5565b6120b6565b61201b565b611ed4565b61080b565b611dd4565b611e7a565b610dbf565b611e5d565b611e37565b611e1d565b611df7565b611d2f565b611c65565b611c3a565b611bea565b611bc4565b611ae8565b611aae565b611a74565b61193d565b61185b565b611821565b6117fd565b6117c5565b6117ab565b611702565b61166f565b6115ef565b611578565b6114fd565b6114e0565b6112dd565b611297565b611275565b61119b565b6110b1565b611070565b611053565b610faa565b610f86565b610f32565b610ed1565b610d62565b610d48565b610d2d565b610c65565b610c20565b6109fe565b610671565b6109cb565b610993565b610828565b6107da565b610700565b6105a0565b61049b565b610455565b600435906001600160a01b038216820361043b57565b5f80fd5b602435906001600160a01b038216820361043b57565b3461043b57604060031936011261043b5761046e610425565b6001600160a01b0360243591165f52601160205260405f20905f52602052602060405f2054604051908152f35b3461043b57602060031936011261043b576004357fffffffff00000000000000000000000000000000000000000000000000000000811680910361043b577f33331994000000000000000000000000000000000000000000000000000000008114908115610576575b8115610519575b506040519015158152602090f35b7f7965db0b0000000000000000000000000000000000000000000000000000000081149150811561054c575b505f61050b565b7f01ffc9a7000000000000000000000000000000000000000000000000000000009150145f610545565b7fb2752ac90000000000000000000000000000000000000000000000000000000081149150610504565b3461043b57602060031936011261043b576001600160a01b036105c1610425565b6105c96127df565b168015610649576105d98161355d565b1561061e57805f52600d6020525f60026040822082815582600182015501557f5d9d5034656cb3ebfb0655057cd7f9b4077a9b42ff42ce223cbac5bc586d21265f80a2005b7f6585b60d000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b57602060031936011261043b576001600160a01b03610692610425565b165f526010602052602060405f2054604051908152f35b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b9060206106fd9281815201906106a9565b90565b3461043b575f60031936011261043b576040515f600354610720816124e3565b80845290600181169081156107b65750600114610758575b6107548361074881850382612620565b604051918291826106ec565b0390f35b60035f9081527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b939250905b80821061079c57509091508101602001610748610738565b919260018160209254838588010152019101909291610784565b60ff191660208086019190915291151560051b840190910191506107489050610738565b3461043b57604060031936011261043b576108006107f6610425565b60243590336135f8565b602060405160018152f35b3461043b575f60031936011261043b576020600254604051908152f35b3461043b57604060031936011261043b57610841610425565b602435906001600160a01b03811690811561064957821561096b576b033b2e3c9fd0803ce80000006108758460025461266f565b11610943576108848333612ac8565b61088e8333612c14565b610896612775565b80610907575b6108df57826108aa91612ca1565b60405191825233917fde22baff038e3a3e08407cbdf617deed74e869a7ba517df611e33131c6e6ea049080602081015b0390a3005b7fdb89e3f4000000000000000000000000000000000000000000000000000000005f5260045ffd5b50335f9081527f740796d87e4f86cc94671768c744956045fe855093291e79194c96ac478040aa602052604090205460ff161561089c565b1590565b7f177e3fc3000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f1f2a2005000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b57606060031936011261043b576108006109af610425565b6109b761043f565b604435916109c6833383612da0565b612ec4565b3461043b57602060031936011261043b5760206109f66004355f526005602052600160405f20015490565b604051908152f35b3461043b57604060031936011261043b57610a17610425565b602435906001600160a01b03811690811561064957821561096b57335f908152600d60205260409020610a6861093f610a57335b6001600160a01b031690565b5f52600f60205260405f2054151590565b8015610c09575b610bdd57610e104204905f5f5b60188110610b8257506001610a91878361266f565b920154809211610b2b575050610ae791610ace8592610ac1336001600160a01b03165f52601260205260405f2090565b905f5260205260405f2090565b610ad983825461266f565b9055833303610b1b57613033565b60405191825233917fb90795a66650155983e242cac3e1ac1a4dc26f8ed2987f3ce416a34e00111fd49080602081016108da565b610b26823383612da0565b613033565b610b7691869180821115610b7957610b429161270e565b905b7fe5fe97a2000000000000000000000000000000000000000000000000000000005f5233600452602452604452606490565b5ffd5b50505f90610b44565b80841015610b93575b600101610a7c565b90610bd5600191610bce610bb8336001600160a01b03165f52601260205260405f2090565b610bc2868961270e565b5f5260205260405f2090565b549061266f565b919050610b8b565b7f6585b60d000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b50610c1b61093f600283015460ff1690565b610a6f565b3461043b57604060031936011261043b57610c63600435610c3f61043f565b90610c5e610c59825f526005602052600160405f20015490565b612a67565b613102565b005b3461043b57602060031936011261043b57610c7e610425565b6001600160a01b0381165f52600d602052610c9b60405f2061269c565b90610e104204915f915f5b60188110610ce5578360208401518181115f14610cda5761075491610cca9161270e565b6040519081529081906020820190565b50506107545f610cca565b80851015610cf6575b600101610ca6565b92610d25600191610bce610d1b856001600160a01b03165f52601260205260405f2090565b610bc2888a61270e565b939050610cee565b3461043b575f60031936011261043b57602060405160128152f35b3461043b575f60031936011261043b5760206109f661319c565b3461043b57604060031936011261043b57600435610d7e61043f565b336001600160a01b03821603610d9757610c63916132b8565b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b57604060031936011261043b57610dd8610425565b6001600160a01b0360243591165f52600a602052610df960405f209161334a565b8154905f829160058411610e79575b610e1393508461382b565b80610e42575050602079ffffffffffffffffffffffffffffffffffffffffffffffffffff5f5b16604051908152f35b602091610e6979ffffffffffffffffffffffffffffffffffffffffffffffffffff92612700565b905f52825f20015460301c610e39565b9192610e84816136b6565b8103908111610ecc57610e1393855f5265ffffffffffff8260205f2001541665ffffffffffff8516105f14610eba575091610e08565b929150610ec690612661565b90610e08565b612499565b3461043b57604060031936011261043b57610eea610425565b602435610ef5612867565b6001600160a01b0382161561064957801561096b57600254818101809111610ecc576b033b2e3c9fd0803ce80000001061094357610c6391612ca1565b3461043b57602060031936011261043b576001600160a01b03610f53610425565b165f52600d602052606060405f2080549060ff600260018301549201541690604051928352602083015215156040820152f35b3461043b57602060031936011261043b57600435801561096b57610c639033613033565b3461043b575f60031936011261043b57610fc34361366e565b65ffffffffffff80610fd44361366e565b1691160361102b57610754604051610fed604082612620565b601d81527f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c7400000060208201526040519182916020835260208301906106a9565b7f6ff07140000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b575f60031936011261043b576020604051629e34008152f35b3461043b57602060031936011261043b576001600160a01b03611091610425565b165f52600960205260206001600160a01b0360405f205416604051908152f35b3461043b57604060031936011261043b576110ca610425565b6024356110d56128ef565b6001600160a01b03821691821561064957811561096b5761110161093f6001600160a01b038516610a57565b611167577f9ca03dbd5193fbb7974173cedd0bdf6841dd14c3cbfa735aab77ff1dd1139fb391611145611162926001600160a01b03165f52601060205260405f2090565b61115082825461266f565b90556040519081529081906020820190565b0390a2005b7f6585b60d000000000000000000000000000000000000000000000000000000005f526001600160a01b031660045260245ffd5b3461043b57602060031936011261043b57600435600e548110156111f1576001600160a01b036111cd610754926134d8565b90549060031b1c16604051918291829190916001600160a01b036020820193169052565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602d60248201527f53796e646963617465546f6b656e43726f7373636861696e3a20696e6465782060448201527f6f7574206f6620626f756e6473000000000000000000000000000000000000006064820152fd5b3461043b57602060031936011261043b57610c63611291610425565b3361339c565b3461043b57604060031936011261043b576112b0610425565b6001600160a01b0360243591165f52601260205260405f20905f52602052602060405f2054604051908152f35b3461043b57606060031936011261043b576112f6610425565b602435906044356113056127df565b6001600160a01b038216928315610649573384146114b857823b15611490575f198114158061147b575b611453575f198214158061143e575b611416576113c58361138261137d610a4b7faa807d0abf30d91968c7478c66b6d82521a106af13eda036e2036da9af168958976001600160a01b031690565b6139e2565b6113de575b6113c061139261267c565b918483528560208401526113a96040840160019052565b6001600160a01b03165f52600d60205260405f2090565b61271b565b6040805191825260208201929092529081908101611162565b604080518481526020810186905287917fdb03f97dc5840a71e69be7470e4761af10a1237973e81c12d0dc2813895a652691a2611387565b7f58ccad00000000000000000000000000000000000000000000000000000000005f5260045ffd5b506b033b2e3c9fd0803ce8000000821161133e565b7f0a395c01000000000000000000000000000000000000000000000000000000005f5260045ffd5b506b033b2e3c9fd0803ce8000000811161132f565b7f825431da000000000000000000000000000000000000000000000000000000005f5260045ffd5b7ffb8ce8c9000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b575f60031936011261043b576020600e54604051908152f35b3461043b57602060031936011261043b576001600160a01b0361151e610425565b165f52600a60205260405f205463ffffffff81116115485760405163ffffffff9091168152602090f35b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52602060045260245260445ffd5b3461043b57602060031936011261043b5760206109f6611596610425565b6001600160a01b03165f525f60205260405f205490565b60206040818301928281528451809452019201905f5b8181106115d05750505090565b82516001600160a01b03168452602093840193909201916001016115c3565b3461043b575f60031936011261043b57604051806020600e54918281520190600e5f527fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3fd905f5b818110611659576107548561164d81870382612620565b604051918291826115ad565b8254845260209093019260019283019201611636565b3461043b57602060031936011261043b576107546001600160a01b03611693610425565b16805f52600d6020526116dd610a5760405f2092604060ff60028251966116b9886125e3565b8054885260018101546020890152015416940193151584526001600160a01b031690565b90816116f7575b5060405190151581529081906020820190565b51151590505f6116e4565b3461043b57604060031936011261043b5761171b610425565b60243590611727612977565b6001600160a01b03811690811561064957821561096b57611746612775565b15611783578261175591613033565b6040519182527fbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a260203393a3005b7fb8b5ca2d000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b575f60031936011261043b5760206109f6612749565b3461043b57602060031936011261043b576001600160a01b036117e6610425565b165f526008602052602060405f2054604051908152f35b3461043b575f60031936011261043b576020611817612775565b6040519015158152f35b3461043b575f60031936011261043b5760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b3461043b57602060031936011261043b576004356118776129ff565b42811115611915577f000000000000000000000000000000000000000000000000000000000000000081116118ed577fdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec611162600c549280600c5560405191829133958360209093929193604081019481520152565b7fef69af65000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fa5658353000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b575f60031936011261043b57611a1b6119797f0000000000000000000000000000000000000000000000000000000000000000613a4b565b6119a27f0000000000000000000000000000000000000000000000000000000000000000613ac4565b60206040516119b18282612620565b5f815281611a29818301947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe083013687376040519788977f0f00000000000000000000000000000000000000000000000000000000000000895260e0858a015260e08901906106a9565b9087820360408901526106a9565b914660608701523060808701525f60a087015285830360c087015251918281520192915f5b828110611a5d57505050500390f35b835185528695509381019392810192600101611a4e565b3461043b575f60031936011261043b5760206040517f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a6748152f35b3461043b575f60031936011261043b5760206040517f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb8152f35b3461043b57602060031936011261043b57611b0460043561334a565b600b54905f829160058411611b70575b611b209350600b61382b565b80611b4e575060205f5b79ffffffffffffffffffffffffffffffffffffffffffffffffffff60405191168152f35b611b6b611b5c602092612700565b600b5f52825f20015460301c90565b611b2a565b9192611b7b816136b6565b8103908111610ecc57611b2093600b5f5265ffffffffffff8260205f2001541665ffffffffffff8516105f14611bb2575091611b14565b929150611bbe90612661565b90611b14565b3461043b575f60031936011261043b5760206040516b033b2e3c9fd0803ce80000008152f35b3461043b57604060031936011261043b57602060ff611c2e600435611c0d61043f565b905f526005845260405f20906001600160a01b03165f5260205260405f2090565b54166040519015158152f35b3461043b575f60031936011261043b576020611c554361366e565b65ffffffffffff60405191168152f35b3461043b57602060031936011261043b57611c7e610425565b6001600160a01b0381165f52600d60205260405f2060ff600260405192611ca4846125e3565b805484526001810154602085015201541615156040820152610e1042045f925f5b60188110611ce757505050518181115f14610cda5761075491610cca9161270e565b80831015611cf8575b600101611cc5565b93611d27600191610bce611d1d856001600160a01b03165f52601160205260405f2090565b610bc2898861270e565b949050611cf0565b3461043b575f60031936011261043b576040515f600454611d4f816124e3565b80845290600181169081156107b65750600114611d76576107548361074881850382612620565b60045f9081527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b939250905b808210611dba57509091508101602001610748610738565b919260018160209254838588010152019101909291611da2565b3461043b57602060031936011261043b5760206109f6611df2610425565b61278c565b3461043b575f60031936011261043b5760206040516b02f90193ef3075fa980000008152f35b3461043b575f60031936011261043b5760206040515f8152f35b3461043b57604060031936011261043b57610800611e53610425565b6024359033612ec4565b3461043b575f60031936011261043b576020600c54604051908152f35b3461043b575f60031936011261043b5760206040517f820372a9faf82db3cc5fc36ffab5f096eef69b95fbf50591e0d71447aa1ba7008152f35b6064359060ff8216820361043b57565b6084359060ff8216820361043b57565b3461043b5760c060031936011261043b57611eed610425565b60243590604435611efc611eb4565b6084359060a43592804211611ff05791611f829391611f74611f799460405160208101917fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf83526001600160a01b038a1660408301528a6060830152608082015260808152611f6c60a082612620565b51902061345b565b613afb565b90929192613bbf565b611fa6816001600160a01b03165f52600860205260405f2080549060018201905590565b809303611fb757610c63925061339c565b6001600160a01b0391507f752d88c0000000000000000000000000000000000000000000000000000000005f521660045260245260445ffd5b7f4683af0e000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b3461043b57602060031936011261043b576001600160a01b0361203c610425565b5f6040805161204a816125e3565b8281528260208201520152165f52600d60205261075460405f2060ff600260405192612075846125e3565b805484526001810154602085015201541615156040820152604051918291829190916040806060830194805184526020810151602085015201511515910152565b3461043b57604060031936011261043b576120cf610425565b602435801515810361043b576120e36127df565b6001600160a01b03821691821561064957612109835f52600f60205260405f2054151590565b1561217957816121677f9c8668db324845065d2b9a2a183bd3141f63018f548282daf18da49ccbf88c33936002612154611162956001600160a01b03165f52600d60205260405f2090565b019060ff60ff1983541691151516179055565b60405190151581529081906020820190565b827f6585b60d000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b3461043b5760e060031936011261043b576121be610425565b6121c661043f565b60443590606435926121d6611ec4565b60a43560c435908642116122d35761227f9261227a61220f866001600160a01b03165f52600860205260405f2080549060018201905590565b9860405160208101917f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983526001600160a01b0389169b8c60408401526001600160a01b038b1660608401528b608084015260a083015260c082015260c08152611f6c60e082612620565b61349c565b936001600160a01b0385160361229957610c6393506135f8565b7f4b800e46000000000000000000000000000000000000000000000000000000005f526001600160a01b038085166004521660245260445ffd5b867f62791302000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b3461043b57604060031936011261043b57610c6360043561231e61043f565b90612338610c59825f526005602052600160405f20015490565b6132b8565b3461043b57604060031936011261043b57602061238d61235b610425565b6001600160a01b0361236b61043f565b91165f526001835260405f20906001600160a01b03165f5260205260405f2090565b54604051908152f35b3461043b57604060031936011261043b576123af610425565b6024359063ffffffff8216820361043b57610754916001600160a01b036123fc926123d86127c7565b506123e16127c7565b50165f52600a60205260405f206123f66127c7565b506134f5565b506040519061240a82612604565b5465ffffffffffff8116825260301c602082015260405191829182919091602079ffffffffffffffffffffffffffffffffffffffffffffffffffff81604084019565ffffffffffff8151168552015116910152565b3461043b575f60031936011261043b5760206040517fcce296b040332a080e6df13515a3ec2869e21cd20f7344af0987ddb938d8bd218152f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b916124df918354905f199060031b92831b921b19161790565b9055565b90600182811c9216801561252a575b60208310146124fd57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f16916124f2565b5f9291815491612543836124e3565b8083529260018116908115612598575060011461255f57505050565b5f9081526020812093945091925b83831061257e575060209250010190565b60018160209294939454838587010152019101919061256d565b9050602094955060ff1991509291921683830152151560051b010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6060810190811067ffffffffffffffff8211176125ff57604052565b6125b6565b6040810190811067ffffffffffffffff8211176125ff57604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176125ff57604052565b9060018201809211610ecc57565b91908201809211610ecc57565b6040519061268b606083612620565b565b6040519061268b604083612620565b906040516126a9816125e3565b604060ff6002839580548552600181015460208601520154161515910152565b81156126d3570490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b905f198201918211610ecc57565b91908203918211610ecc57565b6002604061268b9380518455602081015160018501550151151591019060ff60ff1983541691151516179055565b600c548015801561276b575b61276657428103908111610ecc5790565b505f90565b5080421015612755565b600c548015159081612785575090565b9050421090565b6001600160a01b03165f52600a60205279ffffffffffffffffffffffffffffffffffffffffffffffffffff6127c360405f20613432565b1690565b604051906127d482612604565b5f6020838281520152565b335f9081527feba6e018211a769a99711ab6d90ad4f6d858947b3b2817034e6718b42f4a51c2602052604090205460ff161561281757565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527fcce296b040332a080e6df13515a3ec2869e21cd20f7344af0987ddb938d8bd2160245260445ffd5b335f9081527f9a6bf48bb840e78fe8e7afd10d3d391a91738a9e6524f6fdfa1a3aba9dc03fb1602052604090205460ff161561289f57565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb60245260445ffd5b335f9081527f9e9333a5e45b2fd53e7d1bf86c11c6f010527cce37ba59992c60689f2659c9a1602052604090205460ff161561292757565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f820372a9faf82db3cc5fc36ffab5f096eef69b95fbf50591e0d71447aa1ba70060245260445ffd5b335f9081527f740796d87e4f86cc94671768c744956045fe855093291e79194c96ac478040aa602052604090205460ff16156129af57565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67460245260445ffd5b335f9081527f05b8ccbb9d4d8fb16ea74ce3c29a41f1b461fbdaff4714a0d9a8eb05499746bc602052604090205460ff1615612a3757565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b805f52600560205260ff612a8f3360405f20906001600160a01b03165f5260205260405f2090565b541615612a995750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b612ae3816001600160a01b03165f52600d60205260405f2090565b90612afc61093f610a576001600160a01b038416610a4b565b8015612bfd575b61116757610e104204915f5f5b60188110612bb55750612b23858261266f565b9154809211612b585750506124df91610ac1612b50926001600160a01b03165f52601160205260405f2090565b91825461266f565b610b769492935080821115612bac57612b709161270e565b915b7f40ed367b000000000000000000000000000000000000000000000000000000005f526001600160a01b0316600452602452604452606490565b50505f91612b72565b80851015612bc6575b600101612b10565b90612bf5600191610bce612beb876001600160a01b03165f52601160205260405f2090565b610bc2868a61270e565b919050612bbe565b50612c0f61093f600284015460ff1690565b612b03565b6001600160a01b031690815f52601060205260405f20548111612c7957815f52601060205260405f20805491808303928311610ecc577fbc23ec7f1313150b047bff83d0845b0564baa134698dd11bb0acd0f7d416de7d9260209255604051908152a2565b7f7ade115c000000000000000000000000000000000000000000000000000000005f5260045ffd5b91906001600160a01b0383168015612d7457600254828101809111610ecc57600255612cdd846001600160a01b03165f525f60205260405f2090565b8054830190556040518281525f907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a36002549279ffffffffffffffffffffffffffffffffffffffffffffffffffff808511612d44575061268b9293505f613e1b565b7f1cb15d26000000000000000000000000000000000000000000000000000000005f52600485905260245260445ffd5b7fec442f05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6001600160a01b03909291921690815f526001602052612dd48360405f20906001600160a01b03165f5260205260405f2090565b545f198110612de4575b50505050565b818110612e89578215612e5d576001600160a01b03841615612e3157612e27925f526001602052039160405f20906001600160a01b03165f5260205260405f2090565b555f808080612dde565b7f94280d62000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7fe602df05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6001600160a01b03847ffb8f41b2000000000000000000000000000000000000000000000000000000005f521660045260245260445260645ffd5b9291906001600160a01b038416938415613007576001600160a01b0382168015612d7457612ef0612775565b80612fcf575b6108df57612f14826001600160a01b03165f525f60205260405f2090565b5495848710612f90578461268b969703612f3e846001600160a01b03165f525f60205260405f2090565b55612f59846001600160a01b03165f525f60205260405f2090565b8054860190556040518581527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a3613e1b565b7fe450d38c000000000000000000000000000000000000000000000000000000005f526001600160a01b0383166004526024879052604485905260645ffd5b50335f9081527f740796d87e4f86cc94671768c744956045fe855093291e79194c96ac478040aa602052604090205460ff1615612ef6565b7f96c6fd1e000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6001600160a01b03811680156130075761305d826001600160a01b03165f525f60205260405f2090565b548381106130c557915f80928561268b96950361308a846001600160a01b03165f525f60205260405f2090565b556002805486900390556040518581527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a3613e1b565b7fe450d38c000000000000000000000000000000000000000000000000000000005f526001600160a01b038316600452602452604483905260645ffd5b805f52600560205260ff61312a8360405f20906001600160a01b03165f5260205260405f2090565b541661319657805f5260056020526131568260405f20906001600160a01b03165f5260205260405f2090565b600160ff198254161790556001600160a01b03339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001630148061328f575b156131f7577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f82527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a0815261328960c082612620565b51902090565b507f000000000000000000000000000000000000000000000000000000000000000046146131ce565b805f52600560205260ff6132e08360405f20906001600160a01b03165f5260205260405f2090565b54161561319657805f52600560205261330d8260405f20906001600160a01b03165f5260205260405f2090565b60ff1981541690556001600160a01b03339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190565b65ffffffffffff61335a4361366e565b168082101561336d57506106fd9061366e565b907fecd3f81e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b6001600160a01b038181165f81815260096020526040812080548685167fffffffffffffffffffffffff00000000000000000000000000000000000000008216811790925561268b9694169461342c9390928691907f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9080a46001600160a01b03165f525f60205260405f205490565b9161388f565b80548061343f5750505f90565b805f19810111610ecc575f19915f5260205f2001015460301c90565b60429061346661319c565b90604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b916106fd9391611f7993613afb565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b600e548110156134f057600e5f5260205f2001905f90565b6134ab565b80548210156134f0575f5260205f2001905f90565b80548015613530575f19019061352082826134f5565b5f1982549160031b1b1916905555565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b5f818152600f6020526040902054908115613196575f19820190828211610ecc57600e54925f198401938411610ecc5783835f956135b795036135bd575b5050506135a8600e61350a565b600f905f5260205260405f2090565b55600190565b6135a86135e9916135df6135d56135ef95600e6134f5565b90549060031b1c90565b928391600e6134f5565b906124c6565b555f808061359b565b6001600160a01b0316908115612e5d576001600160a01b038116928315612e3157806136617f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92593855f52600160205260405f20906001600160a01b03165f5260205260405f2090565b55604051908152602090a3565b65ffffffffffff81116136865765ffffffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52603060045260245260445ffd5b60018111156106fd578060017001000000000000000000000000000000008310156137e9575b61378f61378561377b61377161376761375d61374c6137969760048a6801000000000000000061379b9c10156137dc575b6401000000008110156137cf575b620100008110156137c2575b6101008110156137b5575b60108110156137a8575b10156137a0575b60030260011c90565b613756818b6126c9565b0160011c90565b613756818a6126c9565b61375681896126c9565b61375681886126c9565b61375681876126c9565b61375681866126c9565b80936126c9565b821190565b900390565b60011b613743565b60041c9160021b9161373c565b60081c9160041b91613732565b60101c9160081b91613727565b60201c9160101b9161371b565b60401c9160201b9161370d565b505061379b61379661378f61378561377b61377161376761375d61374c6138108a60801c90565b98506801000000000000000097506136dc9650505050505050565b91905b83821061383b5750505090565b9091928083169080841860011c8201809211610ecc57845f5265ffffffffffff8260205f2001541665ffffffffffff8416105f1461387d5750925b919061382e565b93925061388990612661565b91613876565b91906001600160a01b038116926001600160a01b0381169084821415806139d9575b6138bd575b5050505050565b81613963575b5050826138d2575b80806138b6565b61395861393f7fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a7249361393961393379ffffffffffffffffffffffffffffffffffffffffffffffffffff956001600160a01b03165f52600a60205260405f2090565b91613c86565b90613d5a565b6040805192851683529316602082015291829190820190565b0390a25f80806138cb565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff6139cf61393f6139c07fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724946001600160a01b03165f52600a60205260405f2090565b6139c988613c86565b90613cf6565b0390a25f806138c3565b508315156138b1565b5f818152600f602052604090205461276657600e54680100000000000000008110156125ff57613a34613a1e826001859401600e55600e6134f5565b81939154905f199060031b92831b921b19161790565b9055600e54905f52600f60205260405f2055600190565b60ff8114613aaa5760ff811690601f8211613a825760405191613a6f604084612620565b6020808452838101919036833783525290565b7fb3512b0c000000000000000000000000000000000000000000000000000000005f5260045ffd5b506040516106fd81613abd816006612534565b0382612620565b60ff8114613ae85760ff811690601f8211613a825760405191613a6f604084612620565b506040516106fd81613abd816007612534565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411613b7d579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15613b72575f516001600160a01b03811615613b6857905f905f90565b505f906001905f90565b6040513d5f823e3d90fd5b5050505f9160039190565b60041115613b9257565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b613bc881613b88565b80613bd1575050565b613bda81613b88565b60018103613c0a577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b613c1381613b88565b60028103613c4757507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b80613c53600392613b88565b14613c5b5750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b79ffffffffffffffffffffffffffffffffffffffffffffffffffff8111613cc65779ffffffffffffffffffffffffffffffffffffffffffffffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f5260d060045260245260445ffd5b90613d004361366e565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff80613d2685613432565b92169116039079ffffffffffffffffffffffffffffffffffffffffffffffffffff8211610ecc57613d5692613fa3565b9091565b90613d644361366e565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff80613d8a85613432565b92169116019079ffffffffffffffffffffffffffffffffffffffffffffffffffff8211610ecc57613d5692613fa3565b613dc34361366e565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff80613dea600b613432565b921691160379ffffffffffffffffffffffffffffffffffffffffffffffffffff8111610ecc57613d5691600b613fa3565b9091906001600160a01b03168015613e8c575b6001600160a01b0361268b9316908115613e74575b5f5260096020526001600160a01b0360405f205416905f5260096020526001600160a01b0360405f2054169061388f565b613e85613e8084613c86565b613dba565b5050613e43565b613e9582613c86565b92613e9f4361366e565b9379ffffffffffffffffffffffffffffffffffffffffffffffffffff80613ec6600b613432565b92169116019079ffffffffffffffffffffffffffffffffffffffffffffffffffff8211610ecc5761268b946001600160a01b0392613f0591600b613fa3565b905050935050613e2e565b8054680100000000000000008110156125ff57613f32916001820181556134f5565b613f775781516020929092015160301b7fffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000001665ffffffffffff92909216919091179055565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b8054929392801561409957613fba613fc591612700565b825f5260205f200190565b8054603081901c9365ffffffffffff918216929181168084116140715787930361402a575061402692509065ffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffffffff00000000000083549260301b169116179055565b9190565b9150506140269161404a61403c61268d565b65ffffffffffff9093168352565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff86166020830152613f10565b7f2520601d000000000000000000000000000000000000000000000000000000005f5260045ffd5b50906140d1916140aa61403c61268d565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff85166020830152613f10565b5f9190562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d9e9333a5e45b2fd53e7d1bf86c11c6f010527cce37ba59992c60689f2659c9a1eba6e018211a769a99711ab6d90ad4f6d858947b3b2817034e6718b42f4a51c20175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db805b8ccbb9d4d8fb16ea74ce3c29a41f1b461fbdaff4714a0d9a8eb05499746bcdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x80`@R4a\0}Wa\0\x1Ba\0\x15a\0\xE2V[\x90a\x01\x1EV[`@Qa@\xD6\x90\x81a\x0F>\x829`\x80Q\x81a1\xD5\x01R`\xA0Q\x81a2\x92\x01R`\xC0Q\x81a1\xA6\x01R`\xE0Q\x81a2$\x01Ra\x01\0Q\x81a2J\x01Ra\x01 Q\x81a\x19U\x01Ra\x01@Q\x81a\x19~\x01Ra\x01`Q\x81\x81\x81a\x188\x01Ra\x18\x81\x01R\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\0\xB8W`@RV[a\0\x81V[`@Q\x90a\0\xCC`@\x83a\0\x95V[V[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0}WV[aP\xD4\x90`@\x828\x03\x92\x83\x82Q\x94\x85\x92a\0\xFC\x82\x85a\0\x95V[\x839\x81\x01\x03\x12a\0}Wa\x01\x1B` a\x01\x14\x84a\0\xCEV[\x93\x01a\0\xCEV[\x90V[a\x01&a\x03#V[a\x01.a\x03#V[\x90a\x017a\x02\xF9V[\x90c\x14\xD6S\x91`\xE2\x1B` \x83\x01Ra\x01Ma\x03\x0EV[`1`\xF8\x1B` \x82\x01\x90\x81R\x84Q\x90\x94\x91\x93\x91`\x01`\x01`@\x1B\x03\x82\x11a\0\xB8Wa\x01\x82\x82a\x01}`\x03Ta\x03sV[a\x03\xABV[` \x90`\x1F\x83\x11`\x01\x14a\x02rW\x91\x80a\x01\xB6\x92a\x01\xBE\x95\x94_\x92a\x02gW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[`\x03Ua\x04JV[a\x01\xC7\x81a\x07\xBCV[a\x01 Ra\x01\xD4\x82a\x08\xAEV[a\x01@R` \x81Q\x91\x01 `\xE0RQ\x90 a\x01\0RF`\xA0Ra\x01\xF5a\t\xA0V[`\x80R0`\xC0R`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x02XW`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x02XWa\x02Fa\x02U\x92a\x02*Ba\x03^V[a\x01`Ra\x027_`\x0CUV[a\x02@\x83a\x05#V[Pa\x06\xCFV[a\x02O\x81a\x05\x99V[Pa\x064V[PV[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[\x01Q\x90P_\x80a\x01\xA2V[`\x03_R`\x1F\x19\x83\x16\x91\x90\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x92_[\x81\x81\x10a\x02\xE1WP\x91`\x01\x93\x91\x85a\x01\xBE\x97\x96\x94\x10a\x02\xC9W[PPP\x81\x1B\x01`\x03Ua\x04JV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02\xBBV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x02\xA1V[`@Q\x90a\x03\x08`@\x83a\0\x95V[`\x04\x82RV[`@Q\x90a\x03\x1D`@\x83a\0\x95V[`\x01\x82RV[`@Q\x90a\x032`@\x83a\0\x95V[`\t\x82RhSyndicate`\xB8\x1B` \x83\x01RV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90b\x9E4\0\x82\x01\x80\x92\x11a\x03nWV[a\x03JV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x03\xA1W[` \x83\x10\x14a\x03\x8DWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x03\x82V[`\x1F\x81\x11a\x03\xB7WPPV[`\x03_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x03\xF1W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x03\xE6WPPV[_\x81U`\x01\x01a\x03\xDBV[\x90\x91P\x81\x90a\x03\xD2V[`\x1F\x82\x11a\x04\x08WPPPV[_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x04@W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x045WPPV[_\x81U`\x01\x01a\x04*V[\x90\x91P\x81\x90a\x04!V[\x80Q\x90\x91\x90`\x01`\x01`@\x1B\x03\x81\x11a\0\xB8Wa\x04s\x81a\x04l`\x04Ta\x03sV[`\x04a\x03\xFBV[` \x92`\x1F\x82\x11`\x01\x14a\x04\xA7Wa\x04\xA2\x92\x93\x82\x91_\x92a\x02gWPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[`\x04UV[`\x04_R`\x1F\x19\x82\x16\x93\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x91_[\x86\x81\x10a\x05\x0BWP\x83`\x01\x95\x96\x10a\x04\xF3W[PPP\x81\x1B\x01`\x04UV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x04\xE8V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x04\xD5V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` aP\x94_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x05\x94W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` aP\x94_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` aP\x14_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` aPT_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x05\x94W`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` aPT_9_Q\x90_R` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\xCC\xE2\x96\xB0@3*\x08\x0Em\xF15\x15\xA3\xEC(i\xE2\x1C\xD2\x0FsD\xAF\t\x87\xDD\xB98\xD8\xBD!_Q` aP\x14_9_Q\x90_R_\x80\xA4`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` aP4_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x05\x94W`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` aP4_9_Q\x90_R` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x82\x03r\xA9\xFA\xF8-\xB3\xCC_\xC3o\xFA\xB5\xF0\x96\xEE\xF6\x9B\x95\xFB\xF5\x05\x91\xE0\xD7\x14G\xAA\x1B\xA7\0_Q` aP\x14_9_Q\x90_R_\x80\xA4`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x15a\x07\xA9W`\x02Tk\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0\x81\x01\x80\x91\x11a\x03nW`\x02U`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x81\x90R`@\x90 k\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0\x81T\x01\x90U_\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`@Q\x80a\x07ek\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA3`\x02T`\x01`\x01`\xD0\x1B\x03\x90\x81\x81\x11a\x07\x94WPPk\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0a\0\xCC\x91_a\niV[c\x0EX\xAE\x93`\xE1\x1B_R`\x04R`$R`D_\xFD[c\xECD/\x05`\xE0\x1B_R_`\x04R`$_\xFD[\x90\x81Q` \x81\x10_\x14a\x07\xD4WP\x90a\x01\x1B\x90a\t\xFEV[`\x01`\x01`@\x1B\x03\x81\x11a\0\xB8Wa\x07\xF8\x81a\x07\xF1`\x06Ta\x03sV[`\x06a\x03\xFBV[` \x92`\x1F\x82\x11`\x01\x14a\x08/Wa\x08'\x92\x93\x82\x91_\x92a\x02gWPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[`\x06U`\xFF\x90V[`\x06_R`\x1F\x19\x82\x16\x93\x7F\xF6R\"#\x13\xE2\x84YR\x8D\x92\x0Be\x11\\\x16\xC0O>\xFC\x82\xAA\xED\xC9{\xE5\x9F?7|\r?\x91_[\x86\x81\x10a\x08\x96WP\x83`\x01\x95\x96\x10a\x08~W[PPP\x81\x1B\x01`\x06U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x08pV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x08]V[\x90\x81Q` \x81\x10_\x14a\x08\xC6WP\x90a\x01\x1B\x90a\t\xFEV[`\x01`\x01`@\x1B\x03\x81\x11a\0\xB8Wa\x08\xEA\x81a\x08\xE3`\x07Ta\x03sV[`\x07a\x03\xFBV[` \x92`\x1F\x82\x11`\x01\x14a\t!Wa\t\x19\x92\x93\x82\x91_\x92a\x02gWPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[`\x07U`\xFF\x90V[`\x07_R`\x1F\x19\x82\x16\x93\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88\x91_[\x86\x81\x10a\t\x88WP\x83`\x01\x95\x96\x10a\tpW[PPP\x81\x1B\x01`\x07U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\tbV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\tOV[`\xE0Qa\x01\0Q`@Q\x90` \x82\x01\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x84R`@\x83\x01R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\t\xF8`\xC0\x82a\0\x95V[Q\x90 \x90V[`\x1F\x81Q\x11a\n)W` \x81Q\x91\x01Q` \x82\x10a\n\x1AW\x17\x90V[_\x19\x82` \x03`\x03\x1B\x1B\x16\x17\x90V[`D` \x91`@Q\x92\x83\x91c0Z'\xA9`\xE0\x1B\x83R\x81`\x04\x84\x01R\x80Q\x91\x82\x91\x82`$\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xFD[\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\n\xD0W[a\0\xCC\x92`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\n\xB8W[_\x90\x81R`\t` R`@\x80\x82 T\x92\x82R\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16a\x0C\x9AV[a\n\xC9a\n\xC4\x84a\x0BkV[a\x0B\x9CV[PPa\n\x91V[a\n\xD9\x82a\x0BkV[\x92e\xFF\xFF\xFF\xFF\xFF\xFFC\x11a\x0BSW`\x0BT\x80a\x0B\x1DWPa\x0B\x13a\x0B\x03a\0\xCC\x95_[`\x01a\x0E\xE1V[e\xFF\xFF\xFF\xFF\xFF\xFFC\x16`\x0Ba\x0E\x0BV[\x90PP\x92Pa\n|V[\x93\x84_\x19\x81\x01\x11a\x03nW`\x0B_R_Q` aPt_9_Q\x90_R\x90\x94\x01Ta\0\xCC\x94a\x0B\x13\x91a\x0B\x03\x91\x90`0\x1Ca\n\xFCV[c\x06\xDF\xCCe`\xE4\x1B_R`0`\x04RC`$R`D_\xFD[`\x01`\x01`\xD0\x1B\x03\x81\x11a\x0B\x85W`\x01`\x01`\xD0\x1B\x03\x16\x90V[c\x06\xDF\xCCe`\xE4\x1B_R`\xD0`\x04R`$R`D_\xFD[e\xFF\xFF\xFF\xFF\xFF\xFFC\x11a\x0BSW`\x0BT\x80a\x0B\xC6WPa\x0B\x03a\x0B\xC2\x91_[`\x02a\x0E\xE1V[\x90\x91V[\x80_\x19\x81\x01\x11a\x03nW`\x0B_R_Q` aPt_9_Q\x90_R\x01Ta\x0B\xC2\x91a\x0B\x03\x91`0\x1Ca\x0B\xBBV[e\xFF\xFF\xFF\xFF\xFF\xFFC\x11a\x0BSW\x80T\x80a\x0C(WPa\x0C\x18a\x0B\xC2\x92_`\x02a\x0E\xE1V[\x90e\xFF\xFF\xFF\xFF\xFF\xFFC\x16\x90a\x0E\x0BV[\x80_\x19\x81\x01\x11a\x03nW_\x82\x81R` \x90 \x01_\x19\x01Ta\x0B\xC2\x92a\x0C\x18\x91`0\x1Ca\x0B\xBBV[e\xFF\xFF\xFF\xFF\xFF\xFFC\x11a\x0BSW\x80T\x80a\x0CsWPa\x0C\x18a\x0B\xC2\x92_`\x01a\x0E\xE1V[\x80_\x19\x81\x01\x11a\x03nW_\x82\x81R` \x90 \x01_\x19\x01Ta\x0B\xC2\x92a\x0C\x18\x91`0\x1Ca\n\xFCV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x93\x92\x91\x90\x81\x16\x90\x81\x85\x14\x15\x80a\r\x8DW[a\x0C\xC2W[PPPPPV[\x81a\r3W[PP\x82a\x0C\xD7W[\x80\x80a\x0C\xBBV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\n` R`@\x90 _Q` aP\xB4_9_Q\x90_R\x91a\r\x10\x91a\r\n\x90\x91a\x0BkV[\x90a\x0COV[`@\x80Q`\x01`\x01`\xD0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R\xA2_\x80\x80a\x0C\xD0V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\n` R`@\x90 _Q` aP\xB4_9_Q\x90_R\x90a\rk\x90a\re\x86a\x0BkV[\x90a\x0B\xF4V[`@\x80Q`\x01`\x01`\xD0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R\xA2_\x80a\x0C\xC8V[P\x83\x15\x15a\x0C\xB6V[_\x19\x81\x01\x91\x90\x82\x11a\x03nWV[\x90\x81Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\0\xB8W`\x01\x81\x01\x80\x84U\x81\x10\x15a\r\xF7W_\x92\x83R` \x92\x83\x90 \x82Q\x92\x90\x93\x01Q`0\x1Be\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x91\x01UV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80T\x92\x93\x92\x80\x15a\x0E\xB7Wa\x0E\"a\x0E-\x91a\r\x96V[\x82_R` _ \x01\x90V[\x80T`0\x81\x90\x1C\x93e\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92\x91\x81\x16\x80\x84\x11a\x0E\xA8W\x87\x93\x03a\x0EtWPa\x0Ep\x92P\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90`0\x1B\x16\x91\x16\x17\x90UV[\x91\x90V[\x91PPa\x0Ep\x91a\x0E\x94a\x0E\x86a\0\xBDV[e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[`\x01`\x01`\xD0\x1B\x03\x86\x16` \x83\x01Ra\r\xA4V[c% `\x1D`\xE0\x1B_R`\x04_\xFD[P\x90a\x0E\xDC\x91a\x0E\xC8a\x0E\x86a\0\xBDV[`\x01`\x01`\xD0\x1B\x03\x85\x16` \x83\x01Ra\r\xA4V[_\x91\x90V[\x91\x90\x91\x80`\x01\x14a\x0F#W`\x02\x14a\x0F\x07WcNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[`\x01`\x01`\xD0\x1B\x03\x90\x81\x16\x91\x81\x16\x91\x90\x91\x03\x90\x81\x11a\x03nW\x90V[P`\x01`\x01`\xD0\x1B\x03\x91\x82\x16\x90\x82\x16\x01\x90\x81\x11a\x03nW\x90V\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x01\x04-z\x14a\x04 W\x80c\x01\xFF\xC9\xA7\x14a\x04\x1BW\x80c\x04\xDF\x01}\x14a\x04\x16W\x80c\x05\x072\xFB\x14a\x03\xF8W\x80c\x06\xFD\xDE\x03\x14a\x04\x11W\x80c\t^\xA7\xB3\x14a\x04\x0CW\x80c\x18\x16\r\xDD\x14a\x03\x17W\x80c\x18\xBFPw\x14a\x04\x07W\x80c#\xB8r\xDD\x14a\x04\x02W\x80c$\x8A\x9C\xA3\x14a\x03\xFDW\x80c(i6k\x14a\x03\xF8W\x80c+\x8CI\xE3\x14a\x03\xF3W\x80c//\xF1]\x14a\x03\xEEW\x80c0\xD3\xE8\xEB\x14a\x03\xE9W\x80c1<\xE5g\x14a\x03\xE4W\x80c6D\xE5\x15\x14a\x03\xDFW\x80c6V\x8A\xBE\x14a\x03\xDAW\x80c:F\xB1\xA8\x14a\x03&W\x80c@\xC1\x0F\x19\x14a\x03\xD5W\x80cBz\xC0\xCA\x14a\x03\xD0W\x80cB\x96lh\x14a\x03\xCBW\x80cK\xF5\xD7\xE9\x14a\x03\xC6W\x80cO\x1B\xFC\x9E\x14a\x03\xC1W\x80cX|\xDE\x1E\x14a\x03\xBCW\x80cZB9\xE9\x14a\x03\xB7W\x80cZ]\xB1\xBB\x14a\x03\xB2W\x80c\\\x19\xA9\\\x14a\x03\xADW\x80c]Lb\x85\x14a\x03\xA8W\x80cc\xA0\xDA\xAC\x14a\x03\xA3W\x80ce\x14U4\x14a\x03\x9EW\x80co\xCF\xFFE\x14a\x03\x99W\x80cp\xA0\x821\x14a\x03\x94W\x80cr\xCB\xDC\xC8\x14a\x03\x8FW\x80cx\xFB\x7F\xD2\x14a\x03\x8AW\x80cy\xCCg\x90\x14a\x03\x85W\x80cz\x8C\xD1V\x14a\x03\x80W\x80c~\xCE\xBE\0\x14a\x03{W\x80c\x83\xF1!\x1B\x14a\x03vW\x80c\x84&\xAD\xF2\x14a\x03qW\x80c\x84L\x90&\x14a\x03lW\x80c\x84\xB0\x19n\x14a\x03gW\x80c\x8AT%!\x14a\x03bW\x80c\x8D3C\xD6\x14a\x03]W\x80c\x8ES\x9E\x8C\x14a\x03XW\x80c\x90-U\xA5\x14a\x03SW\x80c\x91\xD1HT\x14a\x03NW\x80c\x91\xDD\xAD\xF4\x14a\x03IW\x80c\x94\xAA\"\xF2\x14a\x03DW\x80c\x95\xD8\x9BA\x14a\x03?W\x80c\x9A\xB2N\xB0\x14a\x03\x1CW\x80c\x9B~\xF6K\x14a\x03:W\x80c\xA2\x17\xFD\xDF\x14a\x035W\x80c\xA9\x05\x9C\xBB\x14a\x030W\x80c\xAA\x08*\x9D\x14a\x03+W\x80c\xB0\xCA%>\x14a\x03&W\x80c\xB7\xCD\xC6\x1C\x14a\x03!W\x80c\xBBMD6\x14a\x03\x1CW\x80c\xC0*\xE7T\x14a\x03\x17W\x80c\xC3\xCD\xA5 \x14a\x03\x12W\x80c\xC4\xFCE\xA8\x14a\x03\rW\x80c\xC9\xAB\0\x06\x14a\x03\x08W\x80c\xD5\x05\xAC\xCF\x14a\x03\x03W\x80c\xD5Gt\x1F\x14a\x02\xFEW\x80c\xDDb\xED>\x14a\x02\xF9W\x80c\xF1\x12~\xD8\x14a\x02\xF4Wc\xF7^\x85\x12\x14a\x02\xEFW_\x80\xFD[a$_V[a#\x96V[a#=V[a\"\xFFV[a!\xA5V[a \xB6V[a \x1BV[a\x1E\xD4V[a\x08\x0BV[a\x1D\xD4V[a\x1EzV[a\r\xBFV[a\x1E]V[a\x1E7V[a\x1E\x1DV[a\x1D\xF7V[a\x1D/V[a\x1CeV[a\x1C:V[a\x1B\xEAV[a\x1B\xC4V[a\x1A\xE8V[a\x1A\xAEV[a\x1AtV[a\x19=V[a\x18[V[a\x18!V[a\x17\xFDV[a\x17\xC5V[a\x17\xABV[a\x17\x02V[a\x16oV[a\x15\xEFV[a\x15xV[a\x14\xFDV[a\x14\xE0V[a\x12\xDDV[a\x12\x97V[a\x12uV[a\x11\x9BV[a\x10\xB1V[a\x10pV[a\x10SV[a\x0F\xAAV[a\x0F\x86V[a\x0F2V[a\x0E\xD1V[a\rbV[a\rHV[a\r-V[a\x0CeV[a\x0C V[a\t\xFEV[a\x06qV[a\t\xCBV[a\t\x93V[a\x08(V[a\x07\xDAV[a\x07\0V[a\x05\xA0V[a\x04\x9BV[a\x04UV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x04;WV[_\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x04;WV[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x04na\x04%V[`\x01`\x01`\xA0\x1B\x03`$5\x91\x16_R`\x11` R`@_ \x90_R` R` `@_ T`@Q\x90\x81R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x045\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x80\x91\x03a\x04;W\x7F33\x19\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14\x90\x81\x15a\x05vW[\x81\x15a\x05\x19W[P`@Q\x90\x15\x15\x81R` \x90\xF3[\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14\x91P\x81\x15a\x05LW[P_a\x05\x0BV[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14_a\x05EV[\x7F\xB2u*\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14\x91Pa\x05\x04V[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a\x05\xC1a\x04%V[a\x05\xC9a'\xDFV[\x16\x80\x15a\x06IWa\x05\xD9\x81a5]V[\x15a\x06\x1EW\x80_R`\r` R_`\x02`@\x82 \x82\x81U\x82`\x01\x82\x01U\x01U\x7F]\x9DP4el\xB3\xEB\xFB\x06U\x05|\xD7\xF9\xB4\x07z\x9BB\xFFB\xCE\"<\xBA\xC5\xBCXm!&_\x80\xA2\0[\x7Fe\x85\xB6\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a\x06\x92a\x04%V[\x16_R`\x10` R` `@_ T`@Q\x90\x81R\xF3[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` a\x06\xFD\x92\x81\x81R\x01\x90a\x06\xA9V[\x90V[4a\x04;W_`\x03\x196\x01\x12a\x04;W`@Q_`\x03Ta\x07 \x81a$\xE3V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x07\xB6WP`\x01\x14a\x07XW[a\x07T\x83a\x07H\x81\x85\x03\x82a& V[`@Q\x91\x82\x91\x82a\x06\xECV[\x03\x90\xF3[`\x03_\x90\x81R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x93\x92P\x90[\x80\x82\x10a\x07\x9CWP\x90\x91P\x81\x01` \x01a\x07Ha\x078V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x07\x84V[`\xFF\x19\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x07H\x90Pa\x078V[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x08\0a\x07\xF6a\x04%V[`$5\x903a5\xF8V[` `@Q`\x01\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `\x02T`@Q\x90\x81R\xF3[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x08Aa\x04%V[`$5\x90`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a\x06IW\x82\x15a\tkWk\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x08u\x84`\x02Ta&oV[\x11a\tCWa\x08\x84\x833a*\xC8V[a\x08\x8E\x833a,\x14V[a\x08\x96a'uV[\x80a\t\x07W[a\x08\xDFW\x82a\x08\xAA\x91a,\xA1V[`@Q\x91\x82R3\x91\x7F\xDE\"\xBA\xFF\x03\x8E:>\x08@|\xBD\xF6\x17\xDE\xEDt\xE8i\xA7\xBAQ}\xF6\x11\xE311\xC6\xE6\xEA\x04\x90\x80` \x81\x01[\x03\x90\xA3\0[\x7F\xDB\x89\xE3\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P3_\x90\x81R\x7Ft\x07\x96\xD8~O\x86\xCC\x94g\x17h\xC7D\x95`E\xFE\x85P\x93)\x1Ey\x19L\x96\xACG\x80@\xAA` R`@\x90 T`\xFF\x16\x15a\x08\x9CV[\x15\x90V[\x7F\x17~?\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W```\x03\x196\x01\x12a\x04;Wa\x08\0a\t\xAFa\x04%V[a\t\xB7a\x04?V[`D5\x91a\t\xC6\x833\x83a-\xA0V[a.\xC4V[4a\x04;W` `\x03\x196\x01\x12a\x04;W` a\t\xF6`\x045_R`\x05` R`\x01`@_ \x01T\x90V[`@Q\x90\x81R\xF3[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\n\x17a\x04%V[`$5\x90`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a\x06IW\x82\x15a\tkW3_\x90\x81R`\r` R`@\x90 a\nha\t?a\nW3[`\x01`\x01`\xA0\x1B\x03\x16\x90V[_R`\x0F` R`@_ T\x15\x15\x90V[\x80\x15a\x0C\tW[a\x0B\xDDWa\x0E\x10B\x04\x90__[`\x18\x81\x10a\x0B\x82WP`\x01a\n\x91\x87\x83a&oV[\x92\x01T\x80\x92\x11a\x0B+WPPa\n\xE7\x91a\n\xCE\x85\x92a\n\xC13`\x01`\x01`\xA0\x1B\x03\x16_R`\x12` R`@_ \x90V[\x90_R` R`@_ \x90V[a\n\xD9\x83\x82Ta&oV[\x90U\x833\x03a\x0B\x1BWa03V[`@Q\x91\x82R3\x91\x7F\xB9\x07\x95\xA6fP\x15Y\x83\xE2B\xCA\xC3\xE1\xAC\x1AM\xC2o\x8E\xD2\x98\x7F<\xE4\x16\xA3N\0\x11\x1F\xD4\x90\x80` \x81\x01a\x08\xDAV[a\x0B&\x823\x83a-\xA0V[a03V[a\x0Bv\x91\x86\x91\x80\x82\x11\x15a\x0ByWa\x0BB\x91a'\x0EV[\x90[\x7F\xE5\xFE\x97\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`DR`d\x90V[_\xFD[PP_\x90a\x0BDV[\x80\x84\x10\x15a\x0B\x93W[`\x01\x01a\n|V[\x90a\x0B\xD5`\x01\x91a\x0B\xCEa\x0B\xB83`\x01`\x01`\xA0\x1B\x03\x16_R`\x12` R`@_ \x90V[a\x0B\xC2\x86\x89a'\x0EV[_R` R`@_ \x90V[T\x90a&oV[\x91\x90Pa\x0B\x8BV[\x7Fe\x85\xB6\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[Pa\x0C\x1Ba\t?`\x02\x83\x01T`\xFF\x16\x90V[a\noV[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x0Cc`\x045a\x0C?a\x04?V[\x90a\x0C^a\x0CY\x82_R`\x05` R`\x01`@_ \x01T\x90V[a*gV[a1\x02V[\0[4a\x04;W` `\x03\x196\x01\x12a\x04;Wa\x0C~a\x04%V[`\x01`\x01`\xA0\x1B\x03\x81\x16_R`\r` Ra\x0C\x9B`@_ a&\x9CV[\x90a\x0E\x10B\x04\x91_\x91_[`\x18\x81\x10a\x0C\xE5W\x83` \x84\x01Q\x81\x81\x11_\x14a\x0C\xDAWa\x07T\x91a\x0C\xCA\x91a'\x0EV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[PPa\x07T_a\x0C\xCAV[\x80\x85\x10\x15a\x0C\xF6W[`\x01\x01a\x0C\xA6V[\x92a\r%`\x01\x91a\x0B\xCEa\r\x1B\x85`\x01`\x01`\xA0\x1B\x03\x16_R`\x12` R`@_ \x90V[a\x0B\xC2\x88\x8Aa'\x0EV[\x93\x90Pa\x0C\xEEV[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q`\x12\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` a\t\xF6a1\x9CV[4a\x04;W`@`\x03\x196\x01\x12a\x04;W`\x045a\r~a\x04?V[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\r\x97Wa\x0Cc\x91a2\xB8V[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\r\xD8a\x04%V[`\x01`\x01`\xA0\x1B\x03`$5\x91\x16_R`\n` Ra\r\xF9`@_ \x91a3JV[\x81T\x90_\x82\x91`\x05\x84\x11a\x0EyW[a\x0E\x13\x93P\x84a8+V[\x80a\x0EBWPP` y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_[\x16`@Q\x90\x81R\xF3[` \x91a\x0Eiy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a'\0V[\x90_R\x82_ \x01T`0\x1Ca\x0E9V[\x91\x92a\x0E\x84\x81a6\xB6V[\x81\x03\x90\x81\x11a\x0E\xCCWa\x0E\x13\x93\x85_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x0E\xBAWP\x91a\x0E\x08V[\x92\x91Pa\x0E\xC6\x90a&aV[\x90a\x0E\x08V[a$\x99V[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x0E\xEAa\x04%V[`$5a\x0E\xF5a(gV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x06IW\x80\x15a\tkW`\x02T\x81\x81\x01\x80\x91\x11a\x0E\xCCWk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x10a\tCWa\x0Cc\x91a,\xA1V[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a\x0FSa\x04%V[\x16_R`\r` R```@_ \x80T\x90`\xFF`\x02`\x01\x83\x01T\x92\x01T\x16\x90`@Q\x92\x83R` \x83\x01R\x15\x15`@\x82\x01R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x045\x80\x15a\tkWa\x0Cc\x903a03V[4a\x04;W_`\x03\x196\x01\x12a\x04;Wa\x0F\xC3Ca6nV[e\xFF\xFF\xFF\xFF\xFF\xFF\x80a\x0F\xD4Ca6nV[\x16\x91\x16\x03a\x10+Wa\x07T`@Qa\x0F\xED`@\x82a& V[`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\xA9V[\x7Fo\xF0q@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Qb\x9E4\0\x81R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a\x10\x91a\x04%V[\x16_R`\t` R` `\x01`\x01`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x10\xCAa\x04%V[`$5a\x10\xD5a(\xEFV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x91\x82\x15a\x06IW\x81\x15a\tkWa\x11\x01a\t?`\x01`\x01`\xA0\x1B\x03\x85\x16a\nWV[a\x11gW\x7F\x9C\xA0=\xBDQ\x93\xFB\xB7\x97As\xCE\xDD\x0B\xDFhA\xDD\x14\xC3\xCB\xFAsZ\xABw\xFF\x1D\xD1\x13\x9F\xB3\x91a\x11Ea\x11b\x92`\x01`\x01`\xA0\x1B\x03\x16_R`\x10` R`@_ \x90V[a\x11P\x82\x82Ta&oV[\x90U`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA2\0[\x7Fe\x85\xB6\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x16`\x04R`$_\xFD[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x045`\x0ET\x81\x10\x15a\x11\xF1W`\x01`\x01`\xA0\x1B\x03a\x11\xCDa\x07T\x92a4\xD8V[\x90T\x90`\x03\x1B\x1C\x16`@Q\x91\x82\x91\x82\x91\x90\x91`\x01`\x01`\xA0\x1B\x03` \x82\x01\x93\x16\x90RV[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FSyndicateTokenCrosschain: index `D\x82\x01R\x7Fout of bounds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[4a\x04;W` `\x03\x196\x01\x12a\x04;Wa\x0Cca\x12\x91a\x04%V[3a3\x9CV[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x12\xB0a\x04%V[`\x01`\x01`\xA0\x1B\x03`$5\x91\x16_R`\x12` R`@_ \x90_R` R` `@_ T`@Q\x90\x81R\xF3[4a\x04;W```\x03\x196\x01\x12a\x04;Wa\x12\xF6a\x04%V[`$5\x90`D5a\x13\x05a'\xDFV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x92\x83\x15a\x06IW3\x84\x14a\x14\xB8W\x82;\x15a\x14\x90W_\x19\x81\x14\x15\x80a\x14{W[a\x14SW_\x19\x82\x14\x15\x80a\x14>W[a\x14\x16Wa\x13\xC5\x83a\x13\x82a\x13}a\nK\x7F\xAA\x80}\n\xBF0\xD9\x19h\xC7G\x8Cf\xB6\xD8%!\xA1\x06\xAF\x13\xED\xA06\xE2\x03m\xA9\xAF\x16\x89X\x97`\x01`\x01`\xA0\x1B\x03\x16\x90V[a9\xE2V[a\x13\xDEW[a\x13\xC0a\x13\x92a&|V[\x91\x84\x83R\x85` \x84\x01Ra\x13\xA9`@\x84\x01`\x01\x90RV[`\x01`\x01`\xA0\x1B\x03\x16_R`\r` R`@_ \x90V[a'\x1BV[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90\x81\x01a\x11bV[`@\x80Q\x84\x81R` \x81\x01\x86\x90R\x87\x91\x7F\xDB\x03\xF9}\xC5\x84\nq\xE6\x9B\xE7G\x0EGa\xAF\x10\xA1#ys\xE8\x1C\x12\xD0\xDC(\x13\x89Ze&\x91\xA2a\x13\x87V[\x7FX\xCC\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82\x11a\x13>V[\x7F\n9\\\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x11a\x13/V[\x7F\x82T1\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xFB\x8C\xE8\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `\x0ET`@Q\x90\x81R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a\x15\x1Ea\x04%V[\x16_R`\n` R`@_ Tc\xFF\xFF\xFF\xFF\x81\x11a\x15HW`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x90\xF3[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R` `\x04R`$R`D_\xFD[4a\x04;W` `\x03\x196\x01\x12a\x04;W` a\t\xF6a\x15\x96a\x04%V[`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ T\x90V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x15\xD0WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x15\xC3V[4a\x04;W_`\x03\x196\x01\x12a\x04;W`@Q\x80` `\x0ET\x91\x82\x81R\x01\x90`\x0E_R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x90_[\x81\x81\x10a\x16YWa\x07T\x85a\x16M\x81\x87\x03\x82a& V[`@Q\x91\x82\x91\x82a\x15\xADV[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x166V[4a\x04;W` `\x03\x196\x01\x12a\x04;Wa\x07T`\x01`\x01`\xA0\x1B\x03a\x16\x93a\x04%V[\x16\x80_R`\r` Ra\x16\xDDa\nW`@_ \x92`@`\xFF`\x02\x82Q\x96a\x16\xB9\x88a%\xE3V[\x80T\x88R`\x01\x81\x01T` \x89\x01R\x01T\x16\x94\x01\x93\x15\x15\x84R`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90\x81a\x16\xF7W[P`@Q\x90\x15\x15\x81R\x90\x81\x90` \x82\x01\x90V[Q\x15\x15\x90P_a\x16\xE4V[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x17\x1Ba\x04%V[`$5\x90a\x17'a)wV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a\x06IW\x82\x15a\tkWa\x17Fa'uV[\x15a\x17\x83W\x82a\x17U\x91a03V[`@Q\x91\x82R\x7F\xBE\xF4\xF8\x1C\x18\x14\xC6A\xED\xE8^\xBA\xAC\xF1\x9D\x04\x8B,[U\x98\n\xDF\xA6\xEF\x0F\x95le\x135\xA2` 3\x93\xA3\0[\x7F\xB8\xB5\xCA-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W_`\x03\x196\x01\x12a\x04;W` a\t\xF6a'IV[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a\x17\xE6a\x04%V[\x16_R`\x08` R` `@_ T`@Q\x90\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` a\x18\x17a'uV[`@Q\x90\x15\x15\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x045a\x18wa)\xFFV[B\x81\x11\x15a\x19\x15W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11a\x18\xEDW\x7F\xDDh\x96\xDC\xF1\xD4\xB3\x11\xCC\xA8}\xD1\x9B\xBB\xA2\xEA\x9C\xE2\xF8g\xC1V\x88x\xA0C\x8Af\xA1\xAF\xEE\xECa\x11b`\x0CT\x92\x80`\x0CU`@Q\x91\x82\x913\x95\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x7F\xEFi\xAFe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xA5e\x83S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W_`\x03\x196\x01\x12a\x04;Wa\x1A\x1Ba\x19y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a:KV[a\x19\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a:\xC4V[` `@Qa\x19\xB1\x82\x82a& V[_\x81R\x81a\x1A)\x81\x83\x01\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x83\x016\x877`@Q\x97\x88\x97\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R`\xE0\x85\x8A\x01R`\xE0\x89\x01\x90a\x06\xA9V[\x90\x87\x82\x03`@\x89\x01Ra\x06\xA9V[\x91F``\x87\x01R0`\x80\x87\x01R_`\xA0\x87\x01R\x85\x83\x03`\xC0\x87\x01RQ\x91\x82\x81R\x01\x92\x91_[\x82\x81\x10a\x1A]WPPPP\x03\x90\xF3[\x83Q\x85R\x86\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x1ANV[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x81R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;Wa\x1B\x04`\x045a3JV[`\x0BT\x90_\x82\x91`\x05\x84\x11a\x1BpW[a\x1B \x93P`\x0Ba8+V[\x80a\x1BNWP` _[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[a\x1Bka\x1B\\` \x92a'\0V[`\x0B_R\x82_ \x01T`0\x1C\x90V[a\x1B*V[\x91\x92a\x1B{\x81a6\xB6V[\x81\x03\x90\x81\x11a\x0E\xCCWa\x1B \x93`\x0B_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x1B\xB2WP\x91a\x1B\x14V[\x92\x91Pa\x1B\xBE\x90a&aV[\x90a\x1B\x14V[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Qk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81R\xF3[4a\x04;W`@`\x03\x196\x01\x12a\x04;W` `\xFFa\x1C.`\x045a\x1C\ra\x04?V[\x90_R`\x05\x84R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` a\x1CUCa6nV[e\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;Wa\x1C~a\x04%V[`\x01`\x01`\xA0\x1B\x03\x81\x16_R`\r` R`@_ `\xFF`\x02`@Q\x92a\x1C\xA4\x84a%\xE3V[\x80T\x84R`\x01\x81\x01T` \x85\x01R\x01T\x16\x15\x15`@\x82\x01Ra\x0E\x10B\x04_\x92_[`\x18\x81\x10a\x1C\xE7WPPPQ\x81\x81\x11_\x14a\x0C\xDAWa\x07T\x91a\x0C\xCA\x91a'\x0EV[\x80\x83\x10\x15a\x1C\xF8W[`\x01\x01a\x1C\xC5V[\x93a\x1D'`\x01\x91a\x0B\xCEa\x1D\x1D\x85`\x01`\x01`\xA0\x1B\x03\x16_R`\x11` R`@_ \x90V[a\x0B\xC2\x89\x88a'\x0EV[\x94\x90Pa\x1C\xF0V[4a\x04;W_`\x03\x196\x01\x12a\x04;W`@Q_`\x04Ta\x1DO\x81a$\xE3V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x07\xB6WP`\x01\x14a\x1DvWa\x07T\x83a\x07H\x81\x85\x03\x82a& V[`\x04_\x90\x81R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x93\x92P\x90[\x80\x82\x10a\x1D\xBAWP\x90\x91P\x81\x01` \x01a\x07Ha\x078V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x1D\xA2V[4a\x04;W` `\x03\x196\x01\x12a\x04;W` a\t\xF6a\x1D\xF2a\x04%V[a'\x8CV[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Qk\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q_\x81R\xF3[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x08\0a\x1ESa\x04%V[`$5\x903a.\xC4V[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `\x0CT`@Q\x90\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q\x7F\x82\x03r\xA9\xFA\xF8-\xB3\xCC_\xC3o\xFA\xB5\xF0\x96\xEE\xF6\x9B\x95\xFB\xF5\x05\x91\xE0\xD7\x14G\xAA\x1B\xA7\0\x81R\xF3[`d5\x90`\xFF\x82\x16\x82\x03a\x04;WV[`\x845\x90`\xFF\x82\x16\x82\x03a\x04;WV[4a\x04;W`\xC0`\x03\x196\x01\x12a\x04;Wa\x1E\xEDa\x04%V[`$5\x90`D5a\x1E\xFCa\x1E\xB4V[`\x845\x90`\xA45\x92\x80B\x11a\x1F\xF0W\x91a\x1F\x82\x93\x91a\x1Fta\x1Fy\x94`@Q` \x81\x01\x91\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x83R`\x01`\x01`\xA0\x1B\x03\x8A\x16`@\x83\x01R\x8A``\x83\x01R`\x80\x82\x01R`\x80\x81Ra\x1Fl`\xA0\x82a& V[Q\x90 a4[V[a:\xFBV[\x90\x92\x91\x92a;\xBFV[a\x1F\xA6\x81`\x01`\x01`\xA0\x1B\x03\x16_R`\x08` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[\x80\x93\x03a\x1F\xB7Wa\x0Cc\x92Pa3\x9CV[`\x01`\x01`\xA0\x1B\x03\x91P\x7Fu-\x88\xC0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$R`D_\xFD[\x7FF\x83\xAF\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a <a\x04%V[_`@\x80Qa J\x81a%\xE3V[\x82\x81R\x82` \x82\x01R\x01R\x16_R`\r` Ra\x07T`@_ `\xFF`\x02`@Q\x92a u\x84a%\xE3V[\x80T\x84R`\x01\x81\x01T` \x85\x01R\x01T\x16\x15\x15`@\x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91`@\x80``\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R\x01Q\x15\x15\x91\x01RV[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa \xCFa\x04%V[`$5\x80\x15\x15\x81\x03a\x04;Wa \xE3a'\xDFV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x91\x82\x15a\x06IWa!\t\x83_R`\x0F` R`@_ T\x15\x15\x90V[\x15a!yW\x81a!g\x7F\x9C\x86h\xDB2HE\x06]+\x9A*\x18;\xD3\x14\x1Fc\x01\x8FT\x82\x82\xDA\xF1\x8D\xA4\x9C\xCB\xF8\x8C3\x93`\x02a!Ta\x11b\x95`\x01`\x01`\xA0\x1B\x03\x16_R`\r` R`@_ \x90V[\x01\x90`\xFF`\xFF\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[`@Q\x90\x15\x15\x81R\x90\x81\x90` \x82\x01\x90V[\x82\x7Fe\x85\xB6\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x04;W`\xE0`\x03\x196\x01\x12a\x04;Wa!\xBEa\x04%V[a!\xC6a\x04?V[`D5\x90`d5\x92a!\xD6a\x1E\xC4V[`\xA45`\xC45\x90\x86B\x11a\"\xD3Wa\"\x7F\x92a\"za\"\x0F\x86`\x01`\x01`\xA0\x1B\x03\x16_R`\x08` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[\x98`@Q` \x81\x01\x91\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R`\x01`\x01`\xA0\x1B\x03\x89\x16\x9B\x8C`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x8B\x16``\x84\x01R\x8B`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\x1Fl`\xE0\x82a& V[a4\x9CV[\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x03a\"\x99Wa\x0Cc\x93Pa5\xF8V[\x7FK\x80\x0EF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x04R\x16`$R`D_\xFD[\x86\x7Fby\x13\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x0Cc`\x045a#\x1Ea\x04?V[\x90a#8a\x0CY\x82_R`\x05` R`\x01`@_ \x01T\x90V[a2\xB8V[4a\x04;W`@`\x03\x196\x01\x12a\x04;W` a#\x8Da#[a\x04%V[`\x01`\x01`\xA0\x1B\x03a#ka\x04?V[\x91\x16_R`\x01\x83R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa#\xAFa\x04%V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x04;Wa\x07T\x91`\x01`\x01`\xA0\x1B\x03a#\xFC\x92a#\xD8a'\xC7V[Pa#\xE1a'\xC7V[P\x16_R`\n` R`@_ a#\xF6a'\xC7V[Pa4\xF5V[P`@Q\x90a$\n\x82a&\x04V[Te\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82R`0\x1C` \x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91` y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`@\x84\x01\x95e\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x01Q\x16\x91\x01RV[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q\x7F\xCC\xE2\x96\xB0@3*\x08\x0Em\xF15\x15\xA3\xEC(i\xE2\x1C\xD2\x0FsD\xAF\t\x87\xDD\xB98\xD8\xBD!\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x91a$\xDF\x91\x83T\x90_\x19\x90`\x03\x1B\x92\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x90UV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a%*W[` \x83\x10\x14a$\xFDWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a$\xF2V[_\x92\x91\x81T\x91a%C\x83a$\xE3V[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a%\x98WP`\x01\x14a%_WPPPV[_\x90\x81R` \x81 \x93\x94P\x91\x92[\x83\x83\x10a%~WP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a%mV[\x90P` \x94\x95P`\xFF\x19\x91P\x92\x91\x92\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a%\xFFW`@RV[a%\xB6V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a%\xFFW`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a%\xFFW`@RV[\x90`\x01\x82\x01\x80\x92\x11a\x0E\xCCWV[\x91\x90\x82\x01\x80\x92\x11a\x0E\xCCWV[`@Q\x90a&\x8B``\x83a& V[V[`@Q\x90a&\x8B`@\x83a& V[\x90`@Qa&\xA9\x81a%\xE3V[`@`\xFF`\x02\x83\x95\x80T\x85R`\x01\x81\x01T` \x86\x01R\x01T\x16\x15\x15\x91\x01RV[\x81\x15a&\xD3W\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x90_\x19\x82\x01\x91\x82\x11a\x0E\xCCWV[\x91\x90\x82\x03\x91\x82\x11a\x0E\xCCWV[`\x02`@a&\x8B\x93\x80Q\x84U` \x81\x01Q`\x01\x85\x01U\x01Q\x15\x15\x91\x01\x90`\xFF`\xFF\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[`\x0CT\x80\x15\x80\x15a'kW[a'fWB\x81\x03\x90\x81\x11a\x0E\xCCW\x90V[P_\x90V[P\x80B\x10\x15a'UV[`\x0CT\x80\x15\x15\x90\x81a'\x85WP\x90V[\x90PB\x10\x90V[`\x01`\x01`\xA0\x1B\x03\x16_R`\n` Ry\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa'\xC3`@_ a42V[\x16\x90V[`@Q\x90a'\xD4\x82a&\x04V[_` \x83\x82\x81R\x01RV[3_\x90\x81R\x7F\xEB\xA6\xE0\x18!\x1Av\x9A\x99q\x1A\xB6\xD9\n\xD4\xF6\xD8X\x94{;(\x17\x03Ng\x18\xB4/JQ\xC2` R`@\x90 T`\xFF\x16\x15a(\x17WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\xCC\xE2\x96\xB0@3*\x08\x0Em\xF15\x15\xA3\xEC(i\xE2\x1C\xD2\x0FsD\xAF\t\x87\xDD\xB98\xD8\xBD!`$R`D_\xFD[3_\x90\x81R\x7F\x9Ak\xF4\x8B\xB8@\xE7\x8F\xE8\xE7\xAF\xD1\r=9\x1A\x91s\x8A\x9Ee$\xF6\xFD\xFA\x1A:\xBA\x9D\xC0?\xB1` R`@\x90 T`\xFF\x16\x15a(\x9FWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB`$R`D_\xFD[3_\x90\x81R\x7F\x9E\x933\xA5\xE4[/\xD5>}\x1B\xF8l\x11\xC6\xF0\x10R|\xCE7\xBAY\x99,`h\x9F&Y\xC9\xA1` R`@\x90 T`\xFF\x16\x15a)'WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x82\x03r\xA9\xFA\xF8-\xB3\xCC_\xC3o\xFA\xB5\xF0\x96\xEE\xF6\x9B\x95\xFB\xF5\x05\x91\xE0\xD7\x14G\xAA\x1B\xA7\0`$R`D_\xFD[3_\x90\x81R\x7Ft\x07\x96\xD8~O\x86\xCC\x94g\x17h\xC7D\x95`E\xFE\x85P\x93)\x1Ey\x19L\x96\xACG\x80@\xAA` R`@\x90 T`\xFF\x16\x15a)\xAFWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t`$R`D_\xFD[3_\x90\x81R\x7F\x05\xB8\xCC\xBB\x9DM\x8F\xB1n\xA7L\xE3\xC2\x9AA\xF1\xB4a\xFB\xDA\xFFG\x14\xA0\xD9\xA8\xEB\x05I\x97F\xBC` R`@\x90 T`\xFF\x16\x15a*7WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[\x80_R`\x05` R`\xFFa*\x8F3`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16\x15a*\x99WPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[a*\xE3\x81`\x01`\x01`\xA0\x1B\x03\x16_R`\r` R`@_ \x90V[\x90a*\xFCa\t?a\nW`\x01`\x01`\xA0\x1B\x03\x84\x16a\nKV[\x80\x15a+\xFDW[a\x11gWa\x0E\x10B\x04\x91__[`\x18\x81\x10a+\xB5WPa+#\x85\x82a&oV[\x91T\x80\x92\x11a+XWPPa$\xDF\x91a\n\xC1a+P\x92`\x01`\x01`\xA0\x1B\x03\x16_R`\x11` R`@_ \x90V[\x91\x82Ta&oV[a\x0Bv\x94\x92\x93P\x80\x82\x11\x15a+\xACWa+p\x91a'\x0EV[\x91[\x7F@\xED6{\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x16`\x04R`$R`DR`d\x90V[PP_\x91a+rV[\x80\x85\x10\x15a+\xC6W[`\x01\x01a+\x10V[\x90a+\xF5`\x01\x91a\x0B\xCEa+\xEB\x87`\x01`\x01`\xA0\x1B\x03\x16_R`\x11` R`@_ \x90V[a\x0B\xC2\x86\x8Aa'\x0EV[\x91\x90Pa+\xBEV[Pa,\x0Fa\t?`\x02\x84\x01T`\xFF\x16\x90V[a+\x03V[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81_R`\x10` R`@_ T\x81\x11a,yW\x81_R`\x10` R`@_ \x80T\x91\x80\x83\x03\x92\x83\x11a\x0E\xCCW\x7F\xBC#\xEC\x7F\x13\x13\x15\x0B\x04{\xFF\x83\xD0\x84[\x05d\xBA\xA14i\x8D\xD1\x1B\xB0\xAC\xD0\xF7\xD4\x16\xDE}\x92` \x92U`@Q\x90\x81R\xA2V[\x7Fz\xDE\x11\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x80\x15a-tW`\x02T\x82\x81\x01\x80\x91\x11a\x0E\xCCW`\x02Ua,\xDD\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[\x80T\x83\x01\x90U`@Q\x82\x81R_\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3`\x02T\x92y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x11a-DWPa&\x8B\x92\x93P_a>\x1BV[\x7F\x1C\xB1]&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x85\x90R`$R`D_\xFD[\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x92\x91\x92\x16\x90\x81_R`\x01` Ra-\xD4\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T_\x19\x81\x10a-\xE4W[PPPPV[\x81\x81\x10a.\x89W\x82\x15a.]W`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a.1Wa.'\x92_R`\x01` R\x03\x91`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[U_\x80\x80\x80a-\xDEV[\x7F\x94(\rb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\xE6\x02\xDF\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x7F\xFB\x8FA\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$R`DR`d_\xFD[\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x93\x84\x15a0\x07W`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x15a-tWa.\xF0a'uV[\x80a/\xCFW[a\x08\xDFWa/\x14\x82`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[T\x95\x84\x87\x10a/\x90W\x84a&\x8B\x96\x97\x03a/>\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[Ua/Y\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[\x80T\x86\x01\x90U`@Q\x85\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3a>\x1BV[\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04R`$\x87\x90R`D\x85\x90R`d_\xFD[P3_\x90\x81R\x7Ft\x07\x96\xD8~O\x86\xCC\x94g\x17h\xC7D\x95`E\xFE\x85P\x93)\x1Ey\x19L\x96\xACG\x80@\xAA` R`@\x90 T`\xFF\x16\x15a.\xF6V[\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x15a0\x07Wa0]\x82`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[T\x83\x81\x10a0\xC5W\x91_\x80\x92\x85a&\x8B\x96\x95\x03a0\x8A\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[U`\x02\x80T\x86\x90\x03\x90U`@Q\x85\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3a>\x1BV[\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04R`$R`D\x83\x90R`d_\xFD[\x80_R`\x05` R`\xFFa1*\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16a1\x96W\x80_R`\x05` Ra1V\x82`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[`\x01`\xFF\x19\x82T\x16\x17\x90U`\x01`\x01`\xA0\x1B\x033\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x14\x80a2\x8FW[\x15a1\xF7W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra2\x89`\xC0\x82a& V[Q\x90 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a1\xCEV[\x80_R`\x05` R`\xFFa2\xE0\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16\x15a1\x96W\x80_R`\x05` Ra3\r\x82`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[`\xFF\x19\x81T\x16\x90U`\x01`\x01`\xA0\x1B\x033\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V[e\xFF\xFF\xFF\xFF\xFF\xFFa3ZCa6nV[\x16\x80\x82\x10\x15a3mWPa\x06\xFD\x90a6nV[\x90\x7F\xEC\xD3\xF8\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\t` R`@\x81 \x80T\x86\x85\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x81\x17\x90\x92Ua&\x8B\x96\x94\x16\x94a4,\x93\x90\x92\x86\x91\x90\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x90\x80\xA4`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ T\x90V[\x91a8\x8FV[\x80T\x80a4?WPP_\x90V[\x80_\x19\x81\x01\x11a\x0E\xCCW_\x19\x91_R` _ \x01\x01T`0\x1C\x90V[`B\x90a4fa1\x9CV[\x90`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[\x91a\x06\xFD\x93\x91a\x1Fy\x93a:\xFBV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`\x0ET\x81\x10\x15a4\xF0W`\x0E_R` _ \x01\x90_\x90V[a4\xABV[\x80T\x82\x10\x15a4\xF0W_R` _ \x01\x90_\x90V[\x80T\x80\x15a50W_\x19\x01\x90a5 \x82\x82a4\xF5V[_\x19\x82T\x91`\x03\x1B\x1B\x19\x16\x90UUV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[_\x81\x81R`\x0F` R`@\x90 T\x90\x81\x15a1\x96W_\x19\x82\x01\x90\x82\x82\x11a\x0E\xCCW`\x0ET\x92_\x19\x84\x01\x93\x84\x11a\x0E\xCCW\x83\x83_\x95a5\xB7\x95\x03a5\xBDW[PPPa5\xA8`\x0Ea5\nV[`\x0F\x90_R` R`@_ \x90V[U`\x01\x90V[a5\xA8a5\xE9\x91a5\xDFa5\xD5a5\xEF\x95`\x0Ea4\xF5V[\x90T\x90`\x03\x1B\x1C\x90V[\x92\x83\x91`\x0Ea4\xF5V[\x90a$\xC6V[U_\x80\x80a5\x9BV[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a.]W`\x01`\x01`\xA0\x1B\x03\x81\x16\x92\x83\x15a.1W\x80a6a\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93\x85_R`\x01` R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[U`@Q\x90\x81R` \x90\xA3V[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a6\x86We\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`0`\x04R`$R`D_\xFD[`\x01\x81\x11\x15a\x06\xFDW\x80`\x01p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x10\x15a7\xE9W[a7\x8Fa7\x85a7{a7qa7ga7]a7La7\x96\x97`\x04\x8Ah\x01\0\0\0\0\0\0\0\0a7\x9B\x9C\x10\x15a7\xDCW[d\x01\0\0\0\0\x81\x10\x15a7\xCFW[b\x01\0\0\x81\x10\x15a7\xC2W[a\x01\0\x81\x10\x15a7\xB5W[`\x10\x81\x10\x15a7\xA8W[\x10\x15a7\xA0W[`\x03\x02`\x01\x1C\x90V[a7V\x81\x8Ba&\xC9V[\x01`\x01\x1C\x90V[a7V\x81\x8Aa&\xC9V[a7V\x81\x89a&\xC9V[a7V\x81\x88a&\xC9V[a7V\x81\x87a&\xC9V[a7V\x81\x86a&\xC9V[\x80\x93a&\xC9V[\x82\x11\x90V[\x90\x03\x90V[`\x01\x1Ba7CV[`\x04\x1C\x91`\x02\x1B\x91a7<V[`\x08\x1C\x91`\x04\x1B\x91a72V[`\x10\x1C\x91`\x08\x1B\x91a7'V[` \x1C\x91`\x10\x1B\x91a7\x1BV[`@\x1C\x91` \x1B\x91a7\rV[PPa7\x9Ba7\x96a7\x8Fa7\x85a7{a7qa7ga7]a7La8\x10\x8A`\x80\x1C\x90V[\x98Ph\x01\0\0\0\0\0\0\0\0\x97Pa6\xDC\x96PPPPPPPV[\x91\x90[\x83\x82\x10a8;WPPP\x90V[\x90\x91\x92\x80\x83\x16\x90\x80\x84\x18`\x01\x1C\x82\x01\x80\x92\x11a\x0E\xCCW\x84_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x10_\x14a8}WP\x92[\x91\x90a8.V[\x93\x92Pa8\x89\x90a&aV[\x91a8vV[\x91\x90`\x01`\x01`\xA0\x1B\x03\x81\x16\x92`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x84\x82\x14\x15\x80a9\xD9W[a8\xBDW[PPPPPV[\x81a9cW[PP\x82a8\xD2W[\x80\x80a8\xB6V[a9Xa9?\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x93a99a93y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95`\x01`\x01`\xA0\x1B\x03\x16_R`\n` R`@_ \x90V[\x91a<\x86V[\x90a=ZV[`@\x80Q\x92\x85\x16\x83R\x93\x16` \x82\x01R\x91\x82\x91\x90\x82\x01\x90V[\x03\x90\xA2_\x80\x80a8\xCBV[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa9\xCFa9?a9\xC0\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x94`\x01`\x01`\xA0\x1B\x03\x16_R`\n` R`@_ \x90V[a9\xC9\x88a<\x86V[\x90a<\xF6V[\x03\x90\xA2_\x80a8\xC3V[P\x83\x15\x15a8\xB1V[_\x81\x81R`\x0F` R`@\x90 Ta'fW`\x0ETh\x01\0\0\0\0\0\0\0\0\x81\x10\x15a%\xFFWa:4a:\x1E\x82`\x01\x85\x94\x01`\x0EU`\x0Ea4\xF5V[\x81\x93\x91T\x90_\x19\x90`\x03\x1B\x92\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x90U`\x0ET\x90_R`\x0F` R`@_ U`\x01\x90V[`\xFF\x81\x14a:\xAAW`\xFF\x81\x16\x90`\x1F\x82\x11a:\x82W`@Q\x91a:o`@\x84a& V[` \x80\x84R\x83\x81\x01\x91\x906\x837\x83RR\x90V[\x7F\xB3Q+\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P`@Qa\x06\xFD\x81a:\xBD\x81`\x06a%4V[\x03\x82a& V[`\xFF\x81\x14a:\xE8W`\xFF\x81\x16\x90`\x1F\x82\x11a:\x82W`@Q\x91a:o`@\x84a& V[P`@Qa\x06\xFD\x81a:\xBD\x81`\x07a%4V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a;}W\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a;rW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a;hW\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[`@Q=_\x82>=\x90\xFD[PPP_\x91`\x03\x91\x90V[`\x04\x11\x15a;\x92WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[a;\xC8\x81a;\x88V[\x80a;\xD1WPPV[a;\xDA\x81a;\x88V[`\x01\x81\x03a<\nW\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[a<\x13\x81a;\x88V[`\x02\x81\x03a<GWP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80a<S`\x03\x92a;\x88V[\x14a<[WPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a<\xC6Wy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\xD0`\x04R`$R`D_\xFD[\x90a=\0Ca6nV[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a=&\x85a42V[\x92\x16\x91\x16\x03\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0E\xCCWa=V\x92a?\xA3V[\x90\x91V[\x90a=dCa6nV[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a=\x8A\x85a42V[\x92\x16\x91\x16\x01\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0E\xCCWa=V\x92a?\xA3V[a=\xC3Ca6nV[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a=\xEA`\x0Ba42V[\x92\x16\x91\x16\x03y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0E\xCCWa=V\x91`\x0Ba?\xA3V[\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a>\x8CW[`\x01`\x01`\xA0\x1B\x03a&\x8B\x93\x16\x90\x81\x15a>tW[_R`\t` R`\x01`\x01`\xA0\x1B\x03`@_ T\x16\x90_R`\t` R`\x01`\x01`\xA0\x1B\x03`@_ T\x16\x90a8\x8FV[a>\x85a>\x80\x84a<\x86V[a=\xBAV[PPa>CV[a>\x95\x82a<\x86V[\x92a>\x9FCa6nV[\x93y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a>\xC6`\x0Ba42V[\x92\x16\x91\x16\x01\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0E\xCCWa&\x8B\x94`\x01`\x01`\xA0\x1B\x03\x92a?\x05\x91`\x0Ba?\xA3V[\x90PP\x93PPa>.V[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a%\xFFWa?2\x91`\x01\x82\x01\x81Ua4\xF5V[a?wW\x81Q` \x92\x90\x92\x01Q`0\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x80T\x92\x93\x92\x80\x15a@\x99Wa?\xBAa?\xC5\x91a'\0V[\x82_R` _ \x01\x90V[\x80T`0\x81\x90\x1C\x93e\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92\x91\x81\x16\x80\x84\x11a@qW\x87\x93\x03a@*WPa@&\x92P\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x83T\x92`0\x1B\x16\x91\x16\x17\x90UV[\x91\x90V[\x91PPa@&\x91a@Ja@<a&\x8DV[e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16` \x83\x01Ra?\x10V[\x7F% `\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P\x90a@\xD1\x91a@\xAAa@<a&\x8DV[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16` \x83\x01Ra?\x10V[_\x91\x90V/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x9E\x933\xA5\xE4[/\xD5>}\x1B\xF8l\x11\xC6\xF0\x10R|\xCE7\xBAY\x99,`h\x9F&Y\xC9\xA1\xEB\xA6\xE0\x18!\x1Av\x9A\x99q\x1A\xB6\xD9\n\xD4\xF6\xD8X\x94{;(\x17\x03Ng\x18\xB4/JQ\xC2\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB8\x05\xB8\xCC\xBB\x9DM\x8F\xB1n\xA7L\xE3\xC2\x9AA\xF1\xB4a\xFB\xDA\xFFG\x14\xA0\xD9\xA8\xEB\x05I\x97F\xBC\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c806301042d7a1461042057806301ffc9a71461041b57806304df017d14610416578063050732fb146103f857806306fdde0314610411578063095ea7b31461040c57806318160ddd1461031757806318bf50771461040757806323b872dd14610402578063248a9ca3146103fd5780632869366b146103f85780632b8c49e3146103f35780632f2ff15d146103ee57806330d3e8eb146103e9578063313ce567146103e45780633644e515146103df57806336568abe146103da5780633a46b1a81461032657806340c10f19146103d5578063427ac0ca146103d057806342966c68146103cb5780634bf5d7e9146103c65780634f1bfc9e146103c1578063587cde1e146103bc5780635a4239e9146103b75780635a5db1bb146103b25780635c19a95c146103ad5780635d4c6285146103a857806363a0daac146103a3578063651455341461039e5780636fcfff451461039957806370a082311461039457806372cbdcc81461038f57806378fb7fd21461038a57806379cc6790146103855780637a8cd156146103805780637ecebe001461037b57806383f1211b146103765780638426adf214610371578063844c90261461036c57806384b0196e146103675780638a542521146103625780638d3343d61461035d5780638e539e8c14610358578063902d55a51461035357806391d148541461034e57806391ddadf41461034957806394aa22f21461034457806395d89b411461033f5780639ab24eb01461031c5780639b7ef64b1461033a578063a217fddf14610335578063a9059cbb14610330578063aa082a9d1461032b578063b0ca253e14610326578063b7cdc61c14610321578063bb4d44361461031c578063c02ae75414610317578063c3cda52014610312578063c4fc45a81461030d578063c9ab000614610308578063d505accf14610303578063d547741f146102fe578063dd62ed3e146102f9578063f1127ed8146102f45763f75e8512146102ef575f80fd5b61245f565b612396565b61233d565b6122ff565b6121a5565b6120b6565b61201b565b611ed4565b61080b565b611dd4565b611e7a565b610dbf565b611e5d565b611e37565b611e1d565b611df7565b611d2f565b611c65565b611c3a565b611bea565b611bc4565b611ae8565b611aae565b611a74565b61193d565b61185b565b611821565b6117fd565b6117c5565b6117ab565b611702565b61166f565b6115ef565b611578565b6114fd565b6114e0565b6112dd565b611297565b611275565b61119b565b6110b1565b611070565b611053565b610faa565b610f86565b610f32565b610ed1565b610d62565b610d48565b610d2d565b610c65565b610c20565b6109fe565b610671565b6109cb565b610993565b610828565b6107da565b610700565b6105a0565b61049b565b610455565b600435906001600160a01b038216820361043b57565b5f80fd5b602435906001600160a01b038216820361043b57565b3461043b57604060031936011261043b5761046e610425565b6001600160a01b0360243591165f52601160205260405f20905f52602052602060405f2054604051908152f35b3461043b57602060031936011261043b576004357fffffffff00000000000000000000000000000000000000000000000000000000811680910361043b577f33331994000000000000000000000000000000000000000000000000000000008114908115610576575b8115610519575b506040519015158152602090f35b7f7965db0b0000000000000000000000000000000000000000000000000000000081149150811561054c575b505f61050b565b7f01ffc9a7000000000000000000000000000000000000000000000000000000009150145f610545565b7fb2752ac90000000000000000000000000000000000000000000000000000000081149150610504565b3461043b57602060031936011261043b576001600160a01b036105c1610425565b6105c96127df565b168015610649576105d98161355d565b1561061e57805f52600d6020525f60026040822082815582600182015501557f5d9d5034656cb3ebfb0655057cd7f9b4077a9b42ff42ce223cbac5bc586d21265f80a2005b7f6585b60d000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b57602060031936011261043b576001600160a01b03610692610425565b165f526010602052602060405f2054604051908152f35b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b9060206106fd9281815201906106a9565b90565b3461043b575f60031936011261043b576040515f600354610720816124e3565b80845290600181169081156107b65750600114610758575b6107548361074881850382612620565b604051918291826106ec565b0390f35b60035f9081527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b939250905b80821061079c57509091508101602001610748610738565b919260018160209254838588010152019101909291610784565b60ff191660208086019190915291151560051b840190910191506107489050610738565b3461043b57604060031936011261043b576108006107f6610425565b60243590336135f8565b602060405160018152f35b3461043b575f60031936011261043b576020600254604051908152f35b3461043b57604060031936011261043b57610841610425565b602435906001600160a01b03811690811561064957821561096b576b033b2e3c9fd0803ce80000006108758460025461266f565b11610943576108848333612ac8565b61088e8333612c14565b610896612775565b80610907575b6108df57826108aa91612ca1565b60405191825233917fde22baff038e3a3e08407cbdf617deed74e869a7ba517df611e33131c6e6ea049080602081015b0390a3005b7fdb89e3f4000000000000000000000000000000000000000000000000000000005f5260045ffd5b50335f9081527f740796d87e4f86cc94671768c744956045fe855093291e79194c96ac478040aa602052604090205460ff161561089c565b1590565b7f177e3fc3000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f1f2a2005000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b57606060031936011261043b576108006109af610425565b6109b761043f565b604435916109c6833383612da0565b612ec4565b3461043b57602060031936011261043b5760206109f66004355f526005602052600160405f20015490565b604051908152f35b3461043b57604060031936011261043b57610a17610425565b602435906001600160a01b03811690811561064957821561096b57335f908152600d60205260409020610a6861093f610a57335b6001600160a01b031690565b5f52600f60205260405f2054151590565b8015610c09575b610bdd57610e104204905f5f5b60188110610b8257506001610a91878361266f565b920154809211610b2b575050610ae791610ace8592610ac1336001600160a01b03165f52601260205260405f2090565b905f5260205260405f2090565b610ad983825461266f565b9055833303610b1b57613033565b60405191825233917fb90795a66650155983e242cac3e1ac1a4dc26f8ed2987f3ce416a34e00111fd49080602081016108da565b610b26823383612da0565b613033565b610b7691869180821115610b7957610b429161270e565b905b7fe5fe97a2000000000000000000000000000000000000000000000000000000005f5233600452602452604452606490565b5ffd5b50505f90610b44565b80841015610b93575b600101610a7c565b90610bd5600191610bce610bb8336001600160a01b03165f52601260205260405f2090565b610bc2868961270e565b5f5260205260405f2090565b549061266f565b919050610b8b565b7f6585b60d000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b50610c1b61093f600283015460ff1690565b610a6f565b3461043b57604060031936011261043b57610c63600435610c3f61043f565b90610c5e610c59825f526005602052600160405f20015490565b612a67565b613102565b005b3461043b57602060031936011261043b57610c7e610425565b6001600160a01b0381165f52600d602052610c9b60405f2061269c565b90610e104204915f915f5b60188110610ce5578360208401518181115f14610cda5761075491610cca9161270e565b6040519081529081906020820190565b50506107545f610cca565b80851015610cf6575b600101610ca6565b92610d25600191610bce610d1b856001600160a01b03165f52601260205260405f2090565b610bc2888a61270e565b939050610cee565b3461043b575f60031936011261043b57602060405160128152f35b3461043b575f60031936011261043b5760206109f661319c565b3461043b57604060031936011261043b57600435610d7e61043f565b336001600160a01b03821603610d9757610c63916132b8565b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b57604060031936011261043b57610dd8610425565b6001600160a01b0360243591165f52600a602052610df960405f209161334a565b8154905f829160058411610e79575b610e1393508461382b565b80610e42575050602079ffffffffffffffffffffffffffffffffffffffffffffffffffff5f5b16604051908152f35b602091610e6979ffffffffffffffffffffffffffffffffffffffffffffffffffff92612700565b905f52825f20015460301c610e39565b9192610e84816136b6565b8103908111610ecc57610e1393855f5265ffffffffffff8260205f2001541665ffffffffffff8516105f14610eba575091610e08565b929150610ec690612661565b90610e08565b612499565b3461043b57604060031936011261043b57610eea610425565b602435610ef5612867565b6001600160a01b0382161561064957801561096b57600254818101809111610ecc576b033b2e3c9fd0803ce80000001061094357610c6391612ca1565b3461043b57602060031936011261043b576001600160a01b03610f53610425565b165f52600d602052606060405f2080549060ff600260018301549201541690604051928352602083015215156040820152f35b3461043b57602060031936011261043b57600435801561096b57610c639033613033565b3461043b575f60031936011261043b57610fc34361366e565b65ffffffffffff80610fd44361366e565b1691160361102b57610754604051610fed604082612620565b601d81527f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c7400000060208201526040519182916020835260208301906106a9565b7f6ff07140000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b575f60031936011261043b576020604051629e34008152f35b3461043b57602060031936011261043b576001600160a01b03611091610425565b165f52600960205260206001600160a01b0360405f205416604051908152f35b3461043b57604060031936011261043b576110ca610425565b6024356110d56128ef565b6001600160a01b03821691821561064957811561096b5761110161093f6001600160a01b038516610a57565b611167577f9ca03dbd5193fbb7974173cedd0bdf6841dd14c3cbfa735aab77ff1dd1139fb391611145611162926001600160a01b03165f52601060205260405f2090565b61115082825461266f565b90556040519081529081906020820190565b0390a2005b7f6585b60d000000000000000000000000000000000000000000000000000000005f526001600160a01b031660045260245ffd5b3461043b57602060031936011261043b57600435600e548110156111f1576001600160a01b036111cd610754926134d8565b90549060031b1c16604051918291829190916001600160a01b036020820193169052565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602d60248201527f53796e646963617465546f6b656e43726f7373636861696e3a20696e6465782060448201527f6f7574206f6620626f756e6473000000000000000000000000000000000000006064820152fd5b3461043b57602060031936011261043b57610c63611291610425565b3361339c565b3461043b57604060031936011261043b576112b0610425565b6001600160a01b0360243591165f52601260205260405f20905f52602052602060405f2054604051908152f35b3461043b57606060031936011261043b576112f6610425565b602435906044356113056127df565b6001600160a01b038216928315610649573384146114b857823b15611490575f198114158061147b575b611453575f198214158061143e575b611416576113c58361138261137d610a4b7faa807d0abf30d91968c7478c66b6d82521a106af13eda036e2036da9af168958976001600160a01b031690565b6139e2565b6113de575b6113c061139261267c565b918483528560208401526113a96040840160019052565b6001600160a01b03165f52600d60205260405f2090565b61271b565b6040805191825260208201929092529081908101611162565b604080518481526020810186905287917fdb03f97dc5840a71e69be7470e4761af10a1237973e81c12d0dc2813895a652691a2611387565b7f58ccad00000000000000000000000000000000000000000000000000000000005f5260045ffd5b506b033b2e3c9fd0803ce8000000821161133e565b7f0a395c01000000000000000000000000000000000000000000000000000000005f5260045ffd5b506b033b2e3c9fd0803ce8000000811161132f565b7f825431da000000000000000000000000000000000000000000000000000000005f5260045ffd5b7ffb8ce8c9000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b575f60031936011261043b576020600e54604051908152f35b3461043b57602060031936011261043b576001600160a01b0361151e610425565b165f52600a60205260405f205463ffffffff81116115485760405163ffffffff9091168152602090f35b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52602060045260245260445ffd5b3461043b57602060031936011261043b5760206109f6611596610425565b6001600160a01b03165f525f60205260405f205490565b60206040818301928281528451809452019201905f5b8181106115d05750505090565b82516001600160a01b03168452602093840193909201916001016115c3565b3461043b575f60031936011261043b57604051806020600e54918281520190600e5f527fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3fd905f5b818110611659576107548561164d81870382612620565b604051918291826115ad565b8254845260209093019260019283019201611636565b3461043b57602060031936011261043b576107546001600160a01b03611693610425565b16805f52600d6020526116dd610a5760405f2092604060ff60028251966116b9886125e3565b8054885260018101546020890152015416940193151584526001600160a01b031690565b90816116f7575b5060405190151581529081906020820190565b51151590505f6116e4565b3461043b57604060031936011261043b5761171b610425565b60243590611727612977565b6001600160a01b03811690811561064957821561096b57611746612775565b15611783578261175591613033565b6040519182527fbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a260203393a3005b7fb8b5ca2d000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b575f60031936011261043b5760206109f6612749565b3461043b57602060031936011261043b576001600160a01b036117e6610425565b165f526008602052602060405f2054604051908152f35b3461043b575f60031936011261043b576020611817612775565b6040519015158152f35b3461043b575f60031936011261043b5760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b3461043b57602060031936011261043b576004356118776129ff565b42811115611915577f000000000000000000000000000000000000000000000000000000000000000081116118ed577fdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec611162600c549280600c5560405191829133958360209093929193604081019481520152565b7fef69af65000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fa5658353000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461043b575f60031936011261043b57611a1b6119797f0000000000000000000000000000000000000000000000000000000000000000613a4b565b6119a27f0000000000000000000000000000000000000000000000000000000000000000613ac4565b60206040516119b18282612620565b5f815281611a29818301947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe083013687376040519788977f0f00000000000000000000000000000000000000000000000000000000000000895260e0858a015260e08901906106a9565b9087820360408901526106a9565b914660608701523060808701525f60a087015285830360c087015251918281520192915f5b828110611a5d57505050500390f35b835185528695509381019392810192600101611a4e565b3461043b575f60031936011261043b5760206040517f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a6748152f35b3461043b575f60031936011261043b5760206040517f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb8152f35b3461043b57602060031936011261043b57611b0460043561334a565b600b54905f829160058411611b70575b611b209350600b61382b565b80611b4e575060205f5b79ffffffffffffffffffffffffffffffffffffffffffffffffffff60405191168152f35b611b6b611b5c602092612700565b600b5f52825f20015460301c90565b611b2a565b9192611b7b816136b6565b8103908111610ecc57611b2093600b5f5265ffffffffffff8260205f2001541665ffffffffffff8516105f14611bb2575091611b14565b929150611bbe90612661565b90611b14565b3461043b575f60031936011261043b5760206040516b033b2e3c9fd0803ce80000008152f35b3461043b57604060031936011261043b57602060ff611c2e600435611c0d61043f565b905f526005845260405f20906001600160a01b03165f5260205260405f2090565b54166040519015158152f35b3461043b575f60031936011261043b576020611c554361366e565b65ffffffffffff60405191168152f35b3461043b57602060031936011261043b57611c7e610425565b6001600160a01b0381165f52600d60205260405f2060ff600260405192611ca4846125e3565b805484526001810154602085015201541615156040820152610e1042045f925f5b60188110611ce757505050518181115f14610cda5761075491610cca9161270e565b80831015611cf8575b600101611cc5565b93611d27600191610bce611d1d856001600160a01b03165f52601160205260405f2090565b610bc2898861270e565b949050611cf0565b3461043b575f60031936011261043b576040515f600454611d4f816124e3565b80845290600181169081156107b65750600114611d76576107548361074881850382612620565b60045f9081527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b939250905b808210611dba57509091508101602001610748610738565b919260018160209254838588010152019101909291611da2565b3461043b57602060031936011261043b5760206109f6611df2610425565b61278c565b3461043b575f60031936011261043b5760206040516b02f90193ef3075fa980000008152f35b3461043b575f60031936011261043b5760206040515f8152f35b3461043b57604060031936011261043b57610800611e53610425565b6024359033612ec4565b3461043b575f60031936011261043b576020600c54604051908152f35b3461043b575f60031936011261043b5760206040517f820372a9faf82db3cc5fc36ffab5f096eef69b95fbf50591e0d71447aa1ba7008152f35b6064359060ff8216820361043b57565b6084359060ff8216820361043b57565b3461043b5760c060031936011261043b57611eed610425565b60243590604435611efc611eb4565b6084359060a43592804211611ff05791611f829391611f74611f799460405160208101917fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf83526001600160a01b038a1660408301528a6060830152608082015260808152611f6c60a082612620565b51902061345b565b613afb565b90929192613bbf565b611fa6816001600160a01b03165f52600860205260405f2080549060018201905590565b809303611fb757610c63925061339c565b6001600160a01b0391507f752d88c0000000000000000000000000000000000000000000000000000000005f521660045260245260445ffd5b7f4683af0e000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b3461043b57602060031936011261043b576001600160a01b0361203c610425565b5f6040805161204a816125e3565b8281528260208201520152165f52600d60205261075460405f2060ff600260405192612075846125e3565b805484526001810154602085015201541615156040820152604051918291829190916040806060830194805184526020810151602085015201511515910152565b3461043b57604060031936011261043b576120cf610425565b602435801515810361043b576120e36127df565b6001600160a01b03821691821561064957612109835f52600f60205260405f2054151590565b1561217957816121677f9c8668db324845065d2b9a2a183bd3141f63018f548282daf18da49ccbf88c33936002612154611162956001600160a01b03165f52600d60205260405f2090565b019060ff60ff1983541691151516179055565b60405190151581529081906020820190565b827f6585b60d000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b3461043b5760e060031936011261043b576121be610425565b6121c661043f565b60443590606435926121d6611ec4565b60a43560c435908642116122d35761227f9261227a61220f866001600160a01b03165f52600860205260405f2080549060018201905590565b9860405160208101917f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983526001600160a01b0389169b8c60408401526001600160a01b038b1660608401528b608084015260a083015260c082015260c08152611f6c60e082612620565b61349c565b936001600160a01b0385160361229957610c6393506135f8565b7f4b800e46000000000000000000000000000000000000000000000000000000005f526001600160a01b038085166004521660245260445ffd5b867f62791302000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b3461043b57604060031936011261043b57610c6360043561231e61043f565b90612338610c59825f526005602052600160405f20015490565b6132b8565b3461043b57604060031936011261043b57602061238d61235b610425565b6001600160a01b0361236b61043f565b91165f526001835260405f20906001600160a01b03165f5260205260405f2090565b54604051908152f35b3461043b57604060031936011261043b576123af610425565b6024359063ffffffff8216820361043b57610754916001600160a01b036123fc926123d86127c7565b506123e16127c7565b50165f52600a60205260405f206123f66127c7565b506134f5565b506040519061240a82612604565b5465ffffffffffff8116825260301c602082015260405191829182919091602079ffffffffffffffffffffffffffffffffffffffffffffffffffff81604084019565ffffffffffff8151168552015116910152565b3461043b575f60031936011261043b5760206040517fcce296b040332a080e6df13515a3ec2869e21cd20f7344af0987ddb938d8bd218152f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b916124df918354905f199060031b92831b921b19161790565b9055565b90600182811c9216801561252a575b60208310146124fd57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f16916124f2565b5f9291815491612543836124e3565b8083529260018116908115612598575060011461255f57505050565b5f9081526020812093945091925b83831061257e575060209250010190565b60018160209294939454838587010152019101919061256d565b9050602094955060ff1991509291921683830152151560051b010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6060810190811067ffffffffffffffff8211176125ff57604052565b6125b6565b6040810190811067ffffffffffffffff8211176125ff57604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176125ff57604052565b9060018201809211610ecc57565b91908201809211610ecc57565b6040519061268b606083612620565b565b6040519061268b604083612620565b906040516126a9816125e3565b604060ff6002839580548552600181015460208601520154161515910152565b81156126d3570490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b905f198201918211610ecc57565b91908203918211610ecc57565b6002604061268b9380518455602081015160018501550151151591019060ff60ff1983541691151516179055565b600c548015801561276b575b61276657428103908111610ecc5790565b505f90565b5080421015612755565b600c548015159081612785575090565b9050421090565b6001600160a01b03165f52600a60205279ffffffffffffffffffffffffffffffffffffffffffffffffffff6127c360405f20613432565b1690565b604051906127d482612604565b5f6020838281520152565b335f9081527feba6e018211a769a99711ab6d90ad4f6d858947b3b2817034e6718b42f4a51c2602052604090205460ff161561281757565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527fcce296b040332a080e6df13515a3ec2869e21cd20f7344af0987ddb938d8bd2160245260445ffd5b335f9081527f9a6bf48bb840e78fe8e7afd10d3d391a91738a9e6524f6fdfa1a3aba9dc03fb1602052604090205460ff161561289f57565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb60245260445ffd5b335f9081527f9e9333a5e45b2fd53e7d1bf86c11c6f010527cce37ba59992c60689f2659c9a1602052604090205460ff161561292757565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f820372a9faf82db3cc5fc36ffab5f096eef69b95fbf50591e0d71447aa1ba70060245260445ffd5b335f9081527f740796d87e4f86cc94671768c744956045fe855093291e79194c96ac478040aa602052604090205460ff16156129af57565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67460245260445ffd5b335f9081527f05b8ccbb9d4d8fb16ea74ce3c29a41f1b461fbdaff4714a0d9a8eb05499746bc602052604090205460ff1615612a3757565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b805f52600560205260ff612a8f3360405f20906001600160a01b03165f5260205260405f2090565b541615612a995750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b612ae3816001600160a01b03165f52600d60205260405f2090565b90612afc61093f610a576001600160a01b038416610a4b565b8015612bfd575b61116757610e104204915f5f5b60188110612bb55750612b23858261266f565b9154809211612b585750506124df91610ac1612b50926001600160a01b03165f52601160205260405f2090565b91825461266f565b610b769492935080821115612bac57612b709161270e565b915b7f40ed367b000000000000000000000000000000000000000000000000000000005f526001600160a01b0316600452602452604452606490565b50505f91612b72565b80851015612bc6575b600101612b10565b90612bf5600191610bce612beb876001600160a01b03165f52601160205260405f2090565b610bc2868a61270e565b919050612bbe565b50612c0f61093f600284015460ff1690565b612b03565b6001600160a01b031690815f52601060205260405f20548111612c7957815f52601060205260405f20805491808303928311610ecc577fbc23ec7f1313150b047bff83d0845b0564baa134698dd11bb0acd0f7d416de7d9260209255604051908152a2565b7f7ade115c000000000000000000000000000000000000000000000000000000005f5260045ffd5b91906001600160a01b0383168015612d7457600254828101809111610ecc57600255612cdd846001600160a01b03165f525f60205260405f2090565b8054830190556040518281525f907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a36002549279ffffffffffffffffffffffffffffffffffffffffffffffffffff808511612d44575061268b9293505f613e1b565b7f1cb15d26000000000000000000000000000000000000000000000000000000005f52600485905260245260445ffd5b7fec442f05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6001600160a01b03909291921690815f526001602052612dd48360405f20906001600160a01b03165f5260205260405f2090565b545f198110612de4575b50505050565b818110612e89578215612e5d576001600160a01b03841615612e3157612e27925f526001602052039160405f20906001600160a01b03165f5260205260405f2090565b555f808080612dde565b7f94280d62000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7fe602df05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6001600160a01b03847ffb8f41b2000000000000000000000000000000000000000000000000000000005f521660045260245260445260645ffd5b9291906001600160a01b038416938415613007576001600160a01b0382168015612d7457612ef0612775565b80612fcf575b6108df57612f14826001600160a01b03165f525f60205260405f2090565b5495848710612f90578461268b969703612f3e846001600160a01b03165f525f60205260405f2090565b55612f59846001600160a01b03165f525f60205260405f2090565b8054860190556040518581527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a3613e1b565b7fe450d38c000000000000000000000000000000000000000000000000000000005f526001600160a01b0383166004526024879052604485905260645ffd5b50335f9081527f740796d87e4f86cc94671768c744956045fe855093291e79194c96ac478040aa602052604090205460ff1615612ef6565b7f96c6fd1e000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6001600160a01b03811680156130075761305d826001600160a01b03165f525f60205260405f2090565b548381106130c557915f80928561268b96950361308a846001600160a01b03165f525f60205260405f2090565b556002805486900390556040518581527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a3613e1b565b7fe450d38c000000000000000000000000000000000000000000000000000000005f526001600160a01b038316600452602452604483905260645ffd5b805f52600560205260ff61312a8360405f20906001600160a01b03165f5260205260405f2090565b541661319657805f5260056020526131568260405f20906001600160a01b03165f5260205260405f2090565b600160ff198254161790556001600160a01b03339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001630148061328f575b156131f7577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f82527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a0815261328960c082612620565b51902090565b507f000000000000000000000000000000000000000000000000000000000000000046146131ce565b805f52600560205260ff6132e08360405f20906001600160a01b03165f5260205260405f2090565b54161561319657805f52600560205261330d8260405f20906001600160a01b03165f5260205260405f2090565b60ff1981541690556001600160a01b03339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190565b65ffffffffffff61335a4361366e565b168082101561336d57506106fd9061366e565b907fecd3f81e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b6001600160a01b038181165f81815260096020526040812080548685167fffffffffffffffffffffffff00000000000000000000000000000000000000008216811790925561268b9694169461342c9390928691907f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9080a46001600160a01b03165f525f60205260405f205490565b9161388f565b80548061343f5750505f90565b805f19810111610ecc575f19915f5260205f2001015460301c90565b60429061346661319c565b90604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b916106fd9391611f7993613afb565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b600e548110156134f057600e5f5260205f2001905f90565b6134ab565b80548210156134f0575f5260205f2001905f90565b80548015613530575f19019061352082826134f5565b5f1982549160031b1b1916905555565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b5f818152600f6020526040902054908115613196575f19820190828211610ecc57600e54925f198401938411610ecc5783835f956135b795036135bd575b5050506135a8600e61350a565b600f905f5260205260405f2090565b55600190565b6135a86135e9916135df6135d56135ef95600e6134f5565b90549060031b1c90565b928391600e6134f5565b906124c6565b555f808061359b565b6001600160a01b0316908115612e5d576001600160a01b038116928315612e3157806136617f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92593855f52600160205260405f20906001600160a01b03165f5260205260405f2090565b55604051908152602090a3565b65ffffffffffff81116136865765ffffffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52603060045260245260445ffd5b60018111156106fd578060017001000000000000000000000000000000008310156137e9575b61378f61378561377b61377161376761375d61374c6137969760048a6801000000000000000061379b9c10156137dc575b6401000000008110156137cf575b620100008110156137c2575b6101008110156137b5575b60108110156137a8575b10156137a0575b60030260011c90565b613756818b6126c9565b0160011c90565b613756818a6126c9565b61375681896126c9565b61375681886126c9565b61375681876126c9565b61375681866126c9565b80936126c9565b821190565b900390565b60011b613743565b60041c9160021b9161373c565b60081c9160041b91613732565b60101c9160081b91613727565b60201c9160101b9161371b565b60401c9160201b9161370d565b505061379b61379661378f61378561377b61377161376761375d61374c6138108a60801c90565b98506801000000000000000097506136dc9650505050505050565b91905b83821061383b5750505090565b9091928083169080841860011c8201809211610ecc57845f5265ffffffffffff8260205f2001541665ffffffffffff8416105f1461387d5750925b919061382e565b93925061388990612661565b91613876565b91906001600160a01b038116926001600160a01b0381169084821415806139d9575b6138bd575b5050505050565b81613963575b5050826138d2575b80806138b6565b61395861393f7fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a7249361393961393379ffffffffffffffffffffffffffffffffffffffffffffffffffff956001600160a01b03165f52600a60205260405f2090565b91613c86565b90613d5a565b6040805192851683529316602082015291829190820190565b0390a25f80806138cb565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff6139cf61393f6139c07fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724946001600160a01b03165f52600a60205260405f2090565b6139c988613c86565b90613cf6565b0390a25f806138c3565b508315156138b1565b5f818152600f602052604090205461276657600e54680100000000000000008110156125ff57613a34613a1e826001859401600e55600e6134f5565b81939154905f199060031b92831b921b19161790565b9055600e54905f52600f60205260405f2055600190565b60ff8114613aaa5760ff811690601f8211613a825760405191613a6f604084612620565b6020808452838101919036833783525290565b7fb3512b0c000000000000000000000000000000000000000000000000000000005f5260045ffd5b506040516106fd81613abd816006612534565b0382612620565b60ff8114613ae85760ff811690601f8211613a825760405191613a6f604084612620565b506040516106fd81613abd816007612534565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411613b7d579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15613b72575f516001600160a01b03811615613b6857905f905f90565b505f906001905f90565b6040513d5f823e3d90fd5b5050505f9160039190565b60041115613b9257565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b613bc881613b88565b80613bd1575050565b613bda81613b88565b60018103613c0a577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b613c1381613b88565b60028103613c4757507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b80613c53600392613b88565b14613c5b5750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b79ffffffffffffffffffffffffffffffffffffffffffffffffffff8111613cc65779ffffffffffffffffffffffffffffffffffffffffffffffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f5260d060045260245260445ffd5b90613d004361366e565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff80613d2685613432565b92169116039079ffffffffffffffffffffffffffffffffffffffffffffffffffff8211610ecc57613d5692613fa3565b9091565b90613d644361366e565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff80613d8a85613432565b92169116019079ffffffffffffffffffffffffffffffffffffffffffffffffffff8211610ecc57613d5692613fa3565b613dc34361366e565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff80613dea600b613432565b921691160379ffffffffffffffffffffffffffffffffffffffffffffffffffff8111610ecc57613d5691600b613fa3565b9091906001600160a01b03168015613e8c575b6001600160a01b0361268b9316908115613e74575b5f5260096020526001600160a01b0360405f205416905f5260096020526001600160a01b0360405f2054169061388f565b613e85613e8084613c86565b613dba565b5050613e43565b613e9582613c86565b92613e9f4361366e565b9379ffffffffffffffffffffffffffffffffffffffffffffffffffff80613ec6600b613432565b92169116019079ffffffffffffffffffffffffffffffffffffffffffffffffffff8211610ecc5761268b946001600160a01b0392613f0591600b613fa3565b905050935050613e2e565b8054680100000000000000008110156125ff57613f32916001820181556134f5565b613f775781516020929092015160301b7fffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000001665ffffffffffff92909216919091179055565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b8054929392801561409957613fba613fc591612700565b825f5260205f200190565b8054603081901c9365ffffffffffff918216929181168084116140715787930361402a575061402692509065ffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffffffff00000000000083549260301b169116179055565b9190565b9150506140269161404a61403c61268d565b65ffffffffffff9093168352565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff86166020830152613f10565b7f2520601d000000000000000000000000000000000000000000000000000000005f5260045ffd5b50906140d1916140aa61403c61268d565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff85166020830152613f10565b5f919056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x01\x04-z\x14a\x04 W\x80c\x01\xFF\xC9\xA7\x14a\x04\x1BW\x80c\x04\xDF\x01}\x14a\x04\x16W\x80c\x05\x072\xFB\x14a\x03\xF8W\x80c\x06\xFD\xDE\x03\x14a\x04\x11W\x80c\t^\xA7\xB3\x14a\x04\x0CW\x80c\x18\x16\r\xDD\x14a\x03\x17W\x80c\x18\xBFPw\x14a\x04\x07W\x80c#\xB8r\xDD\x14a\x04\x02W\x80c$\x8A\x9C\xA3\x14a\x03\xFDW\x80c(i6k\x14a\x03\xF8W\x80c+\x8CI\xE3\x14a\x03\xF3W\x80c//\xF1]\x14a\x03\xEEW\x80c0\xD3\xE8\xEB\x14a\x03\xE9W\x80c1<\xE5g\x14a\x03\xE4W\x80c6D\xE5\x15\x14a\x03\xDFW\x80c6V\x8A\xBE\x14a\x03\xDAW\x80c:F\xB1\xA8\x14a\x03&W\x80c@\xC1\x0F\x19\x14a\x03\xD5W\x80cBz\xC0\xCA\x14a\x03\xD0W\x80cB\x96lh\x14a\x03\xCBW\x80cK\xF5\xD7\xE9\x14a\x03\xC6W\x80cO\x1B\xFC\x9E\x14a\x03\xC1W\x80cX|\xDE\x1E\x14a\x03\xBCW\x80cZB9\xE9\x14a\x03\xB7W\x80cZ]\xB1\xBB\x14a\x03\xB2W\x80c\\\x19\xA9\\\x14a\x03\xADW\x80c]Lb\x85\x14a\x03\xA8W\x80cc\xA0\xDA\xAC\x14a\x03\xA3W\x80ce\x14U4\x14a\x03\x9EW\x80co\xCF\xFFE\x14a\x03\x99W\x80cp\xA0\x821\x14a\x03\x94W\x80cr\xCB\xDC\xC8\x14a\x03\x8FW\x80cx\xFB\x7F\xD2\x14a\x03\x8AW\x80cy\xCCg\x90\x14a\x03\x85W\x80cz\x8C\xD1V\x14a\x03\x80W\x80c~\xCE\xBE\0\x14a\x03{W\x80c\x83\xF1!\x1B\x14a\x03vW\x80c\x84&\xAD\xF2\x14a\x03qW\x80c\x84L\x90&\x14a\x03lW\x80c\x84\xB0\x19n\x14a\x03gW\x80c\x8AT%!\x14a\x03bW\x80c\x8D3C\xD6\x14a\x03]W\x80c\x8ES\x9E\x8C\x14a\x03XW\x80c\x90-U\xA5\x14a\x03SW\x80c\x91\xD1HT\x14a\x03NW\x80c\x91\xDD\xAD\xF4\x14a\x03IW\x80c\x94\xAA\"\xF2\x14a\x03DW\x80c\x95\xD8\x9BA\x14a\x03?W\x80c\x9A\xB2N\xB0\x14a\x03\x1CW\x80c\x9B~\xF6K\x14a\x03:W\x80c\xA2\x17\xFD\xDF\x14a\x035W\x80c\xA9\x05\x9C\xBB\x14a\x030W\x80c\xAA\x08*\x9D\x14a\x03+W\x80c\xB0\xCA%>\x14a\x03&W\x80c\xB7\xCD\xC6\x1C\x14a\x03!W\x80c\xBBMD6\x14a\x03\x1CW\x80c\xC0*\xE7T\x14a\x03\x17W\x80c\xC3\xCD\xA5 \x14a\x03\x12W\x80c\xC4\xFCE\xA8\x14a\x03\rW\x80c\xC9\xAB\0\x06\x14a\x03\x08W\x80c\xD5\x05\xAC\xCF\x14a\x03\x03W\x80c\xD5Gt\x1F\x14a\x02\xFEW\x80c\xDDb\xED>\x14a\x02\xF9W\x80c\xF1\x12~\xD8\x14a\x02\xF4Wc\xF7^\x85\x12\x14a\x02\xEFW_\x80\xFD[a$_V[a#\x96V[a#=V[a\"\xFFV[a!\xA5V[a \xB6V[a \x1BV[a\x1E\xD4V[a\x08\x0BV[a\x1D\xD4V[a\x1EzV[a\r\xBFV[a\x1E]V[a\x1E7V[a\x1E\x1DV[a\x1D\xF7V[a\x1D/V[a\x1CeV[a\x1C:V[a\x1B\xEAV[a\x1B\xC4V[a\x1A\xE8V[a\x1A\xAEV[a\x1AtV[a\x19=V[a\x18[V[a\x18!V[a\x17\xFDV[a\x17\xC5V[a\x17\xABV[a\x17\x02V[a\x16oV[a\x15\xEFV[a\x15xV[a\x14\xFDV[a\x14\xE0V[a\x12\xDDV[a\x12\x97V[a\x12uV[a\x11\x9BV[a\x10\xB1V[a\x10pV[a\x10SV[a\x0F\xAAV[a\x0F\x86V[a\x0F2V[a\x0E\xD1V[a\rbV[a\rHV[a\r-V[a\x0CeV[a\x0C V[a\t\xFEV[a\x06qV[a\t\xCBV[a\t\x93V[a\x08(V[a\x07\xDAV[a\x07\0V[a\x05\xA0V[a\x04\x9BV[a\x04UV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x04;WV[_\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x04;WV[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x04na\x04%V[`\x01`\x01`\xA0\x1B\x03`$5\x91\x16_R`\x11` R`@_ \x90_R` R` `@_ T`@Q\x90\x81R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x045\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x80\x91\x03a\x04;W\x7F33\x19\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14\x90\x81\x15a\x05vW[\x81\x15a\x05\x19W[P`@Q\x90\x15\x15\x81R` \x90\xF3[\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14\x91P\x81\x15a\x05LW[P_a\x05\x0BV[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14_a\x05EV[\x7F\xB2u*\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x14\x91Pa\x05\x04V[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a\x05\xC1a\x04%V[a\x05\xC9a'\xDFV[\x16\x80\x15a\x06IWa\x05\xD9\x81a5]V[\x15a\x06\x1EW\x80_R`\r` R_`\x02`@\x82 \x82\x81U\x82`\x01\x82\x01U\x01U\x7F]\x9DP4el\xB3\xEB\xFB\x06U\x05|\xD7\xF9\xB4\x07z\x9BB\xFFB\xCE\"<\xBA\xC5\xBCXm!&_\x80\xA2\0[\x7Fe\x85\xB6\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a\x06\x92a\x04%V[\x16_R`\x10` R` `@_ T`@Q\x90\x81R\xF3[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` a\x06\xFD\x92\x81\x81R\x01\x90a\x06\xA9V[\x90V[4a\x04;W_`\x03\x196\x01\x12a\x04;W`@Q_`\x03Ta\x07 \x81a$\xE3V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x07\xB6WP`\x01\x14a\x07XW[a\x07T\x83a\x07H\x81\x85\x03\x82a& V[`@Q\x91\x82\x91\x82a\x06\xECV[\x03\x90\xF3[`\x03_\x90\x81R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x93\x92P\x90[\x80\x82\x10a\x07\x9CWP\x90\x91P\x81\x01` \x01a\x07Ha\x078V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x07\x84V[`\xFF\x19\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x07H\x90Pa\x078V[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x08\0a\x07\xF6a\x04%V[`$5\x903a5\xF8V[` `@Q`\x01\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `\x02T`@Q\x90\x81R\xF3[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x08Aa\x04%V[`$5\x90`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a\x06IW\x82\x15a\tkWk\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x08u\x84`\x02Ta&oV[\x11a\tCWa\x08\x84\x833a*\xC8V[a\x08\x8E\x833a,\x14V[a\x08\x96a'uV[\x80a\t\x07W[a\x08\xDFW\x82a\x08\xAA\x91a,\xA1V[`@Q\x91\x82R3\x91\x7F\xDE\"\xBA\xFF\x03\x8E:>\x08@|\xBD\xF6\x17\xDE\xEDt\xE8i\xA7\xBAQ}\xF6\x11\xE311\xC6\xE6\xEA\x04\x90\x80` \x81\x01[\x03\x90\xA3\0[\x7F\xDB\x89\xE3\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P3_\x90\x81R\x7Ft\x07\x96\xD8~O\x86\xCC\x94g\x17h\xC7D\x95`E\xFE\x85P\x93)\x1Ey\x19L\x96\xACG\x80@\xAA` R`@\x90 T`\xFF\x16\x15a\x08\x9CV[\x15\x90V[\x7F\x17~?\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W```\x03\x196\x01\x12a\x04;Wa\x08\0a\t\xAFa\x04%V[a\t\xB7a\x04?V[`D5\x91a\t\xC6\x833\x83a-\xA0V[a.\xC4V[4a\x04;W` `\x03\x196\x01\x12a\x04;W` a\t\xF6`\x045_R`\x05` R`\x01`@_ \x01T\x90V[`@Q\x90\x81R\xF3[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\n\x17a\x04%V[`$5\x90`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a\x06IW\x82\x15a\tkW3_\x90\x81R`\r` R`@\x90 a\nha\t?a\nW3[`\x01`\x01`\xA0\x1B\x03\x16\x90V[_R`\x0F` R`@_ T\x15\x15\x90V[\x80\x15a\x0C\tW[a\x0B\xDDWa\x0E\x10B\x04\x90__[`\x18\x81\x10a\x0B\x82WP`\x01a\n\x91\x87\x83a&oV[\x92\x01T\x80\x92\x11a\x0B+WPPa\n\xE7\x91a\n\xCE\x85\x92a\n\xC13`\x01`\x01`\xA0\x1B\x03\x16_R`\x12` R`@_ \x90V[\x90_R` R`@_ \x90V[a\n\xD9\x83\x82Ta&oV[\x90U\x833\x03a\x0B\x1BWa03V[`@Q\x91\x82R3\x91\x7F\xB9\x07\x95\xA6fP\x15Y\x83\xE2B\xCA\xC3\xE1\xAC\x1AM\xC2o\x8E\xD2\x98\x7F<\xE4\x16\xA3N\0\x11\x1F\xD4\x90\x80` \x81\x01a\x08\xDAV[a\x0B&\x823\x83a-\xA0V[a03V[a\x0Bv\x91\x86\x91\x80\x82\x11\x15a\x0ByWa\x0BB\x91a'\x0EV[\x90[\x7F\xE5\xFE\x97\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`DR`d\x90V[_\xFD[PP_\x90a\x0BDV[\x80\x84\x10\x15a\x0B\x93W[`\x01\x01a\n|V[\x90a\x0B\xD5`\x01\x91a\x0B\xCEa\x0B\xB83`\x01`\x01`\xA0\x1B\x03\x16_R`\x12` R`@_ \x90V[a\x0B\xC2\x86\x89a'\x0EV[_R` R`@_ \x90V[T\x90a&oV[\x91\x90Pa\x0B\x8BV[\x7Fe\x85\xB6\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[Pa\x0C\x1Ba\t?`\x02\x83\x01T`\xFF\x16\x90V[a\noV[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x0Cc`\x045a\x0C?a\x04?V[\x90a\x0C^a\x0CY\x82_R`\x05` R`\x01`@_ \x01T\x90V[a*gV[a1\x02V[\0[4a\x04;W` `\x03\x196\x01\x12a\x04;Wa\x0C~a\x04%V[`\x01`\x01`\xA0\x1B\x03\x81\x16_R`\r` Ra\x0C\x9B`@_ a&\x9CV[\x90a\x0E\x10B\x04\x91_\x91_[`\x18\x81\x10a\x0C\xE5W\x83` \x84\x01Q\x81\x81\x11_\x14a\x0C\xDAWa\x07T\x91a\x0C\xCA\x91a'\x0EV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[PPa\x07T_a\x0C\xCAV[\x80\x85\x10\x15a\x0C\xF6W[`\x01\x01a\x0C\xA6V[\x92a\r%`\x01\x91a\x0B\xCEa\r\x1B\x85`\x01`\x01`\xA0\x1B\x03\x16_R`\x12` R`@_ \x90V[a\x0B\xC2\x88\x8Aa'\x0EV[\x93\x90Pa\x0C\xEEV[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q`\x12\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` a\t\xF6a1\x9CV[4a\x04;W`@`\x03\x196\x01\x12a\x04;W`\x045a\r~a\x04?V[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\r\x97Wa\x0Cc\x91a2\xB8V[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\r\xD8a\x04%V[`\x01`\x01`\xA0\x1B\x03`$5\x91\x16_R`\n` Ra\r\xF9`@_ \x91a3JV[\x81T\x90_\x82\x91`\x05\x84\x11a\x0EyW[a\x0E\x13\x93P\x84a8+V[\x80a\x0EBWPP` y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_[\x16`@Q\x90\x81R\xF3[` \x91a\x0Eiy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a'\0V[\x90_R\x82_ \x01T`0\x1Ca\x0E9V[\x91\x92a\x0E\x84\x81a6\xB6V[\x81\x03\x90\x81\x11a\x0E\xCCWa\x0E\x13\x93\x85_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x0E\xBAWP\x91a\x0E\x08V[\x92\x91Pa\x0E\xC6\x90a&aV[\x90a\x0E\x08V[a$\x99V[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x0E\xEAa\x04%V[`$5a\x0E\xF5a(gV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x06IW\x80\x15a\tkW`\x02T\x81\x81\x01\x80\x91\x11a\x0E\xCCWk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x10a\tCWa\x0Cc\x91a,\xA1V[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a\x0FSa\x04%V[\x16_R`\r` R```@_ \x80T\x90`\xFF`\x02`\x01\x83\x01T\x92\x01T\x16\x90`@Q\x92\x83R` \x83\x01R\x15\x15`@\x82\x01R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x045\x80\x15a\tkWa\x0Cc\x903a03V[4a\x04;W_`\x03\x196\x01\x12a\x04;Wa\x0F\xC3Ca6nV[e\xFF\xFF\xFF\xFF\xFF\xFF\x80a\x0F\xD4Ca6nV[\x16\x91\x16\x03a\x10+Wa\x07T`@Qa\x0F\xED`@\x82a& V[`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\xA9V[\x7Fo\xF0q@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Qb\x9E4\0\x81R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a\x10\x91a\x04%V[\x16_R`\t` R` `\x01`\x01`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x10\xCAa\x04%V[`$5a\x10\xD5a(\xEFV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x91\x82\x15a\x06IW\x81\x15a\tkWa\x11\x01a\t?`\x01`\x01`\xA0\x1B\x03\x85\x16a\nWV[a\x11gW\x7F\x9C\xA0=\xBDQ\x93\xFB\xB7\x97As\xCE\xDD\x0B\xDFhA\xDD\x14\xC3\xCB\xFAsZ\xABw\xFF\x1D\xD1\x13\x9F\xB3\x91a\x11Ea\x11b\x92`\x01`\x01`\xA0\x1B\x03\x16_R`\x10` R`@_ \x90V[a\x11P\x82\x82Ta&oV[\x90U`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA2\0[\x7Fe\x85\xB6\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x16`\x04R`$_\xFD[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x045`\x0ET\x81\x10\x15a\x11\xF1W`\x01`\x01`\xA0\x1B\x03a\x11\xCDa\x07T\x92a4\xD8V[\x90T\x90`\x03\x1B\x1C\x16`@Q\x91\x82\x91\x82\x91\x90\x91`\x01`\x01`\xA0\x1B\x03` \x82\x01\x93\x16\x90RV[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FSyndicateTokenCrosschain: index `D\x82\x01R\x7Fout of bounds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[4a\x04;W` `\x03\x196\x01\x12a\x04;Wa\x0Cca\x12\x91a\x04%V[3a3\x9CV[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x12\xB0a\x04%V[`\x01`\x01`\xA0\x1B\x03`$5\x91\x16_R`\x12` R`@_ \x90_R` R` `@_ T`@Q\x90\x81R\xF3[4a\x04;W```\x03\x196\x01\x12a\x04;Wa\x12\xF6a\x04%V[`$5\x90`D5a\x13\x05a'\xDFV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x92\x83\x15a\x06IW3\x84\x14a\x14\xB8W\x82;\x15a\x14\x90W_\x19\x81\x14\x15\x80a\x14{W[a\x14SW_\x19\x82\x14\x15\x80a\x14>W[a\x14\x16Wa\x13\xC5\x83a\x13\x82a\x13}a\nK\x7F\xAA\x80}\n\xBF0\xD9\x19h\xC7G\x8Cf\xB6\xD8%!\xA1\x06\xAF\x13\xED\xA06\xE2\x03m\xA9\xAF\x16\x89X\x97`\x01`\x01`\xA0\x1B\x03\x16\x90V[a9\xE2V[a\x13\xDEW[a\x13\xC0a\x13\x92a&|V[\x91\x84\x83R\x85` \x84\x01Ra\x13\xA9`@\x84\x01`\x01\x90RV[`\x01`\x01`\xA0\x1B\x03\x16_R`\r` R`@_ \x90V[a'\x1BV[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90\x81\x01a\x11bV[`@\x80Q\x84\x81R` \x81\x01\x86\x90R\x87\x91\x7F\xDB\x03\xF9}\xC5\x84\nq\xE6\x9B\xE7G\x0EGa\xAF\x10\xA1#ys\xE8\x1C\x12\xD0\xDC(\x13\x89Ze&\x91\xA2a\x13\x87V[\x7FX\xCC\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x82\x11a\x13>V[\x7F\n9\\\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81\x11a\x13/V[\x7F\x82T1\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xFB\x8C\xE8\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `\x0ET`@Q\x90\x81R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a\x15\x1Ea\x04%V[\x16_R`\n` R`@_ Tc\xFF\xFF\xFF\xFF\x81\x11a\x15HW`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x90\xF3[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R` `\x04R`$R`D_\xFD[4a\x04;W` `\x03\x196\x01\x12a\x04;W` a\t\xF6a\x15\x96a\x04%V[`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ T\x90V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x15\xD0WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x15\xC3V[4a\x04;W_`\x03\x196\x01\x12a\x04;W`@Q\x80` `\x0ET\x91\x82\x81R\x01\x90`\x0E_R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x90_[\x81\x81\x10a\x16YWa\x07T\x85a\x16M\x81\x87\x03\x82a& V[`@Q\x91\x82\x91\x82a\x15\xADV[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x166V[4a\x04;W` `\x03\x196\x01\x12a\x04;Wa\x07T`\x01`\x01`\xA0\x1B\x03a\x16\x93a\x04%V[\x16\x80_R`\r` Ra\x16\xDDa\nW`@_ \x92`@`\xFF`\x02\x82Q\x96a\x16\xB9\x88a%\xE3V[\x80T\x88R`\x01\x81\x01T` \x89\x01R\x01T\x16\x94\x01\x93\x15\x15\x84R`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90\x81a\x16\xF7W[P`@Q\x90\x15\x15\x81R\x90\x81\x90` \x82\x01\x90V[Q\x15\x15\x90P_a\x16\xE4V[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x17\x1Ba\x04%V[`$5\x90a\x17'a)wV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a\x06IW\x82\x15a\tkWa\x17Fa'uV[\x15a\x17\x83W\x82a\x17U\x91a03V[`@Q\x91\x82R\x7F\xBE\xF4\xF8\x1C\x18\x14\xC6A\xED\xE8^\xBA\xAC\xF1\x9D\x04\x8B,[U\x98\n\xDF\xA6\xEF\x0F\x95le\x135\xA2` 3\x93\xA3\0[\x7F\xB8\xB5\xCA-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W_`\x03\x196\x01\x12a\x04;W` a\t\xF6a'IV[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a\x17\xE6a\x04%V[\x16_R`\x08` R` `@_ T`@Q\x90\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` a\x18\x17a'uV[`@Q\x90\x15\x15\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x045a\x18wa)\xFFV[B\x81\x11\x15a\x19\x15W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11a\x18\xEDW\x7F\xDDh\x96\xDC\xF1\xD4\xB3\x11\xCC\xA8}\xD1\x9B\xBB\xA2\xEA\x9C\xE2\xF8g\xC1V\x88x\xA0C\x8Af\xA1\xAF\xEE\xECa\x11b`\x0CT\x92\x80`\x0CU`@Q\x91\x82\x913\x95\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x7F\xEFi\xAFe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xA5e\x83S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04;W_`\x03\x196\x01\x12a\x04;Wa\x1A\x1Ba\x19y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a:KV[a\x19\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a:\xC4V[` `@Qa\x19\xB1\x82\x82a& V[_\x81R\x81a\x1A)\x81\x83\x01\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x83\x016\x877`@Q\x97\x88\x97\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R`\xE0\x85\x8A\x01R`\xE0\x89\x01\x90a\x06\xA9V[\x90\x87\x82\x03`@\x89\x01Ra\x06\xA9V[\x91F``\x87\x01R0`\x80\x87\x01R_`\xA0\x87\x01R\x85\x83\x03`\xC0\x87\x01RQ\x91\x82\x81R\x01\x92\x91_[\x82\x81\x10a\x1A]WPPPP\x03\x90\xF3[\x83Q\x85R\x86\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x1ANV[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x81R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;Wa\x1B\x04`\x045a3JV[`\x0BT\x90_\x82\x91`\x05\x84\x11a\x1BpW[a\x1B \x93P`\x0Ba8+V[\x80a\x1BNWP` _[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[a\x1Bka\x1B\\` \x92a'\0V[`\x0B_R\x82_ \x01T`0\x1C\x90V[a\x1B*V[\x91\x92a\x1B{\x81a6\xB6V[\x81\x03\x90\x81\x11a\x0E\xCCWa\x1B \x93`\x0B_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x1B\xB2WP\x91a\x1B\x14V[\x92\x91Pa\x1B\xBE\x90a&aV[\x90a\x1B\x14V[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Qk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81R\xF3[4a\x04;W`@`\x03\x196\x01\x12a\x04;W` `\xFFa\x1C.`\x045a\x1C\ra\x04?V[\x90_R`\x05\x84R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` a\x1CUCa6nV[e\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x04;W` `\x03\x196\x01\x12a\x04;Wa\x1C~a\x04%V[`\x01`\x01`\xA0\x1B\x03\x81\x16_R`\r` R`@_ `\xFF`\x02`@Q\x92a\x1C\xA4\x84a%\xE3V[\x80T\x84R`\x01\x81\x01T` \x85\x01R\x01T\x16\x15\x15`@\x82\x01Ra\x0E\x10B\x04_\x92_[`\x18\x81\x10a\x1C\xE7WPPPQ\x81\x81\x11_\x14a\x0C\xDAWa\x07T\x91a\x0C\xCA\x91a'\x0EV[\x80\x83\x10\x15a\x1C\xF8W[`\x01\x01a\x1C\xC5V[\x93a\x1D'`\x01\x91a\x0B\xCEa\x1D\x1D\x85`\x01`\x01`\xA0\x1B\x03\x16_R`\x11` R`@_ \x90V[a\x0B\xC2\x89\x88a'\x0EV[\x94\x90Pa\x1C\xF0V[4a\x04;W_`\x03\x196\x01\x12a\x04;W`@Q_`\x04Ta\x1DO\x81a$\xE3V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x07\xB6WP`\x01\x14a\x1DvWa\x07T\x83a\x07H\x81\x85\x03\x82a& V[`\x04_\x90\x81R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x93\x92P\x90[\x80\x82\x10a\x1D\xBAWP\x90\x91P\x81\x01` \x01a\x07Ha\x078V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x1D\xA2V[4a\x04;W` `\x03\x196\x01\x12a\x04;W` a\t\xF6a\x1D\xF2a\x04%V[a'\x8CV[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Qk\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q_\x81R\xF3[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x08\0a\x1ESa\x04%V[`$5\x903a.\xC4V[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `\x0CT`@Q\x90\x81R\xF3[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q\x7F\x82\x03r\xA9\xFA\xF8-\xB3\xCC_\xC3o\xFA\xB5\xF0\x96\xEE\xF6\x9B\x95\xFB\xF5\x05\x91\xE0\xD7\x14G\xAA\x1B\xA7\0\x81R\xF3[`d5\x90`\xFF\x82\x16\x82\x03a\x04;WV[`\x845\x90`\xFF\x82\x16\x82\x03a\x04;WV[4a\x04;W`\xC0`\x03\x196\x01\x12a\x04;Wa\x1E\xEDa\x04%V[`$5\x90`D5a\x1E\xFCa\x1E\xB4V[`\x845\x90`\xA45\x92\x80B\x11a\x1F\xF0W\x91a\x1F\x82\x93\x91a\x1Fta\x1Fy\x94`@Q` \x81\x01\x91\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x83R`\x01`\x01`\xA0\x1B\x03\x8A\x16`@\x83\x01R\x8A``\x83\x01R`\x80\x82\x01R`\x80\x81Ra\x1Fl`\xA0\x82a& V[Q\x90 a4[V[a:\xFBV[\x90\x92\x91\x92a;\xBFV[a\x1F\xA6\x81`\x01`\x01`\xA0\x1B\x03\x16_R`\x08` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[\x80\x93\x03a\x1F\xB7Wa\x0Cc\x92Pa3\x9CV[`\x01`\x01`\xA0\x1B\x03\x91P\x7Fu-\x88\xC0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$R`D_\xFD[\x7FF\x83\xAF\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x04;W` `\x03\x196\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03a <a\x04%V[_`@\x80Qa J\x81a%\xE3V[\x82\x81R\x82` \x82\x01R\x01R\x16_R`\r` Ra\x07T`@_ `\xFF`\x02`@Q\x92a u\x84a%\xE3V[\x80T\x84R`\x01\x81\x01T` \x85\x01R\x01T\x16\x15\x15`@\x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91`@\x80``\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R\x01Q\x15\x15\x91\x01RV[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa \xCFa\x04%V[`$5\x80\x15\x15\x81\x03a\x04;Wa \xE3a'\xDFV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x91\x82\x15a\x06IWa!\t\x83_R`\x0F` R`@_ T\x15\x15\x90V[\x15a!yW\x81a!g\x7F\x9C\x86h\xDB2HE\x06]+\x9A*\x18;\xD3\x14\x1Fc\x01\x8FT\x82\x82\xDA\xF1\x8D\xA4\x9C\xCB\xF8\x8C3\x93`\x02a!Ta\x11b\x95`\x01`\x01`\xA0\x1B\x03\x16_R`\r` R`@_ \x90V[\x01\x90`\xFF`\xFF\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[`@Q\x90\x15\x15\x81R\x90\x81\x90` \x82\x01\x90V[\x82\x7Fe\x85\xB6\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x04;W`\xE0`\x03\x196\x01\x12a\x04;Wa!\xBEa\x04%V[a!\xC6a\x04?V[`D5\x90`d5\x92a!\xD6a\x1E\xC4V[`\xA45`\xC45\x90\x86B\x11a\"\xD3Wa\"\x7F\x92a\"za\"\x0F\x86`\x01`\x01`\xA0\x1B\x03\x16_R`\x08` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[\x98`@Q` \x81\x01\x91\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R`\x01`\x01`\xA0\x1B\x03\x89\x16\x9B\x8C`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x8B\x16``\x84\x01R\x8B`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\x1Fl`\xE0\x82a& V[a4\x9CV[\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x03a\"\x99Wa\x0Cc\x93Pa5\xF8V[\x7FK\x80\x0EF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x04R\x16`$R`D_\xFD[\x86\x7Fby\x13\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa\x0Cc`\x045a#\x1Ea\x04?V[\x90a#8a\x0CY\x82_R`\x05` R`\x01`@_ \x01T\x90V[a2\xB8V[4a\x04;W`@`\x03\x196\x01\x12a\x04;W` a#\x8Da#[a\x04%V[`\x01`\x01`\xA0\x1B\x03a#ka\x04?V[\x91\x16_R`\x01\x83R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[4a\x04;W`@`\x03\x196\x01\x12a\x04;Wa#\xAFa\x04%V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x04;Wa\x07T\x91`\x01`\x01`\xA0\x1B\x03a#\xFC\x92a#\xD8a'\xC7V[Pa#\xE1a'\xC7V[P\x16_R`\n` R`@_ a#\xF6a'\xC7V[Pa4\xF5V[P`@Q\x90a$\n\x82a&\x04V[Te\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82R`0\x1C` \x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91` y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`@\x84\x01\x95e\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x01Q\x16\x91\x01RV[4a\x04;W_`\x03\x196\x01\x12a\x04;W` `@Q\x7F\xCC\xE2\x96\xB0@3*\x08\x0Em\xF15\x15\xA3\xEC(i\xE2\x1C\xD2\x0FsD\xAF\t\x87\xDD\xB98\xD8\xBD!\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x91a$\xDF\x91\x83T\x90_\x19\x90`\x03\x1B\x92\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x90UV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a%*W[` \x83\x10\x14a$\xFDWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a$\xF2V[_\x92\x91\x81T\x91a%C\x83a$\xE3V[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a%\x98WP`\x01\x14a%_WPPPV[_\x90\x81R` \x81 \x93\x94P\x91\x92[\x83\x83\x10a%~WP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a%mV[\x90P` \x94\x95P`\xFF\x19\x91P\x92\x91\x92\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a%\xFFW`@RV[a%\xB6V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a%\xFFW`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a%\xFFW`@RV[\x90`\x01\x82\x01\x80\x92\x11a\x0E\xCCWV[\x91\x90\x82\x01\x80\x92\x11a\x0E\xCCWV[`@Q\x90a&\x8B``\x83a& V[V[`@Q\x90a&\x8B`@\x83a& V[\x90`@Qa&\xA9\x81a%\xE3V[`@`\xFF`\x02\x83\x95\x80T\x85R`\x01\x81\x01T` \x86\x01R\x01T\x16\x15\x15\x91\x01RV[\x81\x15a&\xD3W\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x90_\x19\x82\x01\x91\x82\x11a\x0E\xCCWV[\x91\x90\x82\x03\x91\x82\x11a\x0E\xCCWV[`\x02`@a&\x8B\x93\x80Q\x84U` \x81\x01Q`\x01\x85\x01U\x01Q\x15\x15\x91\x01\x90`\xFF`\xFF\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[`\x0CT\x80\x15\x80\x15a'kW[a'fWB\x81\x03\x90\x81\x11a\x0E\xCCW\x90V[P_\x90V[P\x80B\x10\x15a'UV[`\x0CT\x80\x15\x15\x90\x81a'\x85WP\x90V[\x90PB\x10\x90V[`\x01`\x01`\xA0\x1B\x03\x16_R`\n` Ry\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa'\xC3`@_ a42V[\x16\x90V[`@Q\x90a'\xD4\x82a&\x04V[_` \x83\x82\x81R\x01RV[3_\x90\x81R\x7F\xEB\xA6\xE0\x18!\x1Av\x9A\x99q\x1A\xB6\xD9\n\xD4\xF6\xD8X\x94{;(\x17\x03Ng\x18\xB4/JQ\xC2` R`@\x90 T`\xFF\x16\x15a(\x17WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\xCC\xE2\x96\xB0@3*\x08\x0Em\xF15\x15\xA3\xEC(i\xE2\x1C\xD2\x0FsD\xAF\t\x87\xDD\xB98\xD8\xBD!`$R`D_\xFD[3_\x90\x81R\x7F\x9Ak\xF4\x8B\xB8@\xE7\x8F\xE8\xE7\xAF\xD1\r=9\x1A\x91s\x8A\x9Ee$\xF6\xFD\xFA\x1A:\xBA\x9D\xC0?\xB1` R`@\x90 T`\xFF\x16\x15a(\x9FWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB`$R`D_\xFD[3_\x90\x81R\x7F\x9E\x933\xA5\xE4[/\xD5>}\x1B\xF8l\x11\xC6\xF0\x10R|\xCE7\xBAY\x99,`h\x9F&Y\xC9\xA1` R`@\x90 T`\xFF\x16\x15a)'WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x82\x03r\xA9\xFA\xF8-\xB3\xCC_\xC3o\xFA\xB5\xF0\x96\xEE\xF6\x9B\x95\xFB\xF5\x05\x91\xE0\xD7\x14G\xAA\x1B\xA7\0`$R`D_\xFD[3_\x90\x81R\x7Ft\x07\x96\xD8~O\x86\xCC\x94g\x17h\xC7D\x95`E\xFE\x85P\x93)\x1Ey\x19L\x96\xACG\x80@\xAA` R`@\x90 T`\xFF\x16\x15a)\xAFWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t`$R`D_\xFD[3_\x90\x81R\x7F\x05\xB8\xCC\xBB\x9DM\x8F\xB1n\xA7L\xE3\xC2\x9AA\xF1\xB4a\xFB\xDA\xFFG\x14\xA0\xD9\xA8\xEB\x05I\x97F\xBC` R`@\x90 T`\xFF\x16\x15a*7WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[\x80_R`\x05` R`\xFFa*\x8F3`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16\x15a*\x99WPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[a*\xE3\x81`\x01`\x01`\xA0\x1B\x03\x16_R`\r` R`@_ \x90V[\x90a*\xFCa\t?a\nW`\x01`\x01`\xA0\x1B\x03\x84\x16a\nKV[\x80\x15a+\xFDW[a\x11gWa\x0E\x10B\x04\x91__[`\x18\x81\x10a+\xB5WPa+#\x85\x82a&oV[\x91T\x80\x92\x11a+XWPPa$\xDF\x91a\n\xC1a+P\x92`\x01`\x01`\xA0\x1B\x03\x16_R`\x11` R`@_ \x90V[\x91\x82Ta&oV[a\x0Bv\x94\x92\x93P\x80\x82\x11\x15a+\xACWa+p\x91a'\x0EV[\x91[\x7F@\xED6{\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x16`\x04R`$R`DR`d\x90V[PP_\x91a+rV[\x80\x85\x10\x15a+\xC6W[`\x01\x01a+\x10V[\x90a+\xF5`\x01\x91a\x0B\xCEa+\xEB\x87`\x01`\x01`\xA0\x1B\x03\x16_R`\x11` R`@_ \x90V[a\x0B\xC2\x86\x8Aa'\x0EV[\x91\x90Pa+\xBEV[Pa,\x0Fa\t?`\x02\x84\x01T`\xFF\x16\x90V[a+\x03V[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81_R`\x10` R`@_ T\x81\x11a,yW\x81_R`\x10` R`@_ \x80T\x91\x80\x83\x03\x92\x83\x11a\x0E\xCCW\x7F\xBC#\xEC\x7F\x13\x13\x15\x0B\x04{\xFF\x83\xD0\x84[\x05d\xBA\xA14i\x8D\xD1\x1B\xB0\xAC\xD0\xF7\xD4\x16\xDE}\x92` \x92U`@Q\x90\x81R\xA2V[\x7Fz\xDE\x11\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x80\x15a-tW`\x02T\x82\x81\x01\x80\x91\x11a\x0E\xCCW`\x02Ua,\xDD\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[\x80T\x83\x01\x90U`@Q\x82\x81R_\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3`\x02T\x92y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x11a-DWPa&\x8B\x92\x93P_a>\x1BV[\x7F\x1C\xB1]&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x85\x90R`$R`D_\xFD[\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x92\x91\x92\x16\x90\x81_R`\x01` Ra-\xD4\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T_\x19\x81\x10a-\xE4W[PPPPV[\x81\x81\x10a.\x89W\x82\x15a.]W`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a.1Wa.'\x92_R`\x01` R\x03\x91`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[U_\x80\x80\x80a-\xDEV[\x7F\x94(\rb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\xE6\x02\xDF\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x7F\xFB\x8FA\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$R`DR`d_\xFD[\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x93\x84\x15a0\x07W`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x15a-tWa.\xF0a'uV[\x80a/\xCFW[a\x08\xDFWa/\x14\x82`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[T\x95\x84\x87\x10a/\x90W\x84a&\x8B\x96\x97\x03a/>\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[Ua/Y\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[\x80T\x86\x01\x90U`@Q\x85\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3a>\x1BV[\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04R`$\x87\x90R`D\x85\x90R`d_\xFD[P3_\x90\x81R\x7Ft\x07\x96\xD8~O\x86\xCC\x94g\x17h\xC7D\x95`E\xFE\x85P\x93)\x1Ey\x19L\x96\xACG\x80@\xAA` R`@\x90 T`\xFF\x16\x15a.\xF6V[\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x15a0\x07Wa0]\x82`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[T\x83\x81\x10a0\xC5W\x91_\x80\x92\x85a&\x8B\x96\x95\x03a0\x8A\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[U`\x02\x80T\x86\x90\x03\x90U`@Q\x85\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3a>\x1BV[\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04R`$R`D\x83\x90R`d_\xFD[\x80_R`\x05` R`\xFFa1*\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16a1\x96W\x80_R`\x05` Ra1V\x82`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[`\x01`\xFF\x19\x82T\x16\x17\x90U`\x01`\x01`\xA0\x1B\x033\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x14\x80a2\x8FW[\x15a1\xF7W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra2\x89`\xC0\x82a& V[Q\x90 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a1\xCEV[\x80_R`\x05` R`\xFFa2\xE0\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16\x15a1\x96W\x80_R`\x05` Ra3\r\x82`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[`\xFF\x19\x81T\x16\x90U`\x01`\x01`\xA0\x1B\x033\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V[e\xFF\xFF\xFF\xFF\xFF\xFFa3ZCa6nV[\x16\x80\x82\x10\x15a3mWPa\x06\xFD\x90a6nV[\x90\x7F\xEC\xD3\xF8\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\t` R`@\x81 \x80T\x86\x85\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x81\x17\x90\x92Ua&\x8B\x96\x94\x16\x94a4,\x93\x90\x92\x86\x91\x90\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x90\x80\xA4`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ T\x90V[\x91a8\x8FV[\x80T\x80a4?WPP_\x90V[\x80_\x19\x81\x01\x11a\x0E\xCCW_\x19\x91_R` _ \x01\x01T`0\x1C\x90V[`B\x90a4fa1\x9CV[\x90`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[\x91a\x06\xFD\x93\x91a\x1Fy\x93a:\xFBV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`\x0ET\x81\x10\x15a4\xF0W`\x0E_R` _ \x01\x90_\x90V[a4\xABV[\x80T\x82\x10\x15a4\xF0W_R` _ \x01\x90_\x90V[\x80T\x80\x15a50W_\x19\x01\x90a5 \x82\x82a4\xF5V[_\x19\x82T\x91`\x03\x1B\x1B\x19\x16\x90UUV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[_\x81\x81R`\x0F` R`@\x90 T\x90\x81\x15a1\x96W_\x19\x82\x01\x90\x82\x82\x11a\x0E\xCCW`\x0ET\x92_\x19\x84\x01\x93\x84\x11a\x0E\xCCW\x83\x83_\x95a5\xB7\x95\x03a5\xBDW[PPPa5\xA8`\x0Ea5\nV[`\x0F\x90_R` R`@_ \x90V[U`\x01\x90V[a5\xA8a5\xE9\x91a5\xDFa5\xD5a5\xEF\x95`\x0Ea4\xF5V[\x90T\x90`\x03\x1B\x1C\x90V[\x92\x83\x91`\x0Ea4\xF5V[\x90a$\xC6V[U_\x80\x80a5\x9BV[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a.]W`\x01`\x01`\xA0\x1B\x03\x81\x16\x92\x83\x15a.1W\x80a6a\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93\x85_R`\x01` R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[U`@Q\x90\x81R` \x90\xA3V[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a6\x86We\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`0`\x04R`$R`D_\xFD[`\x01\x81\x11\x15a\x06\xFDW\x80`\x01p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x10\x15a7\xE9W[a7\x8Fa7\x85a7{a7qa7ga7]a7La7\x96\x97`\x04\x8Ah\x01\0\0\0\0\0\0\0\0a7\x9B\x9C\x10\x15a7\xDCW[d\x01\0\0\0\0\x81\x10\x15a7\xCFW[b\x01\0\0\x81\x10\x15a7\xC2W[a\x01\0\x81\x10\x15a7\xB5W[`\x10\x81\x10\x15a7\xA8W[\x10\x15a7\xA0W[`\x03\x02`\x01\x1C\x90V[a7V\x81\x8Ba&\xC9V[\x01`\x01\x1C\x90V[a7V\x81\x8Aa&\xC9V[a7V\x81\x89a&\xC9V[a7V\x81\x88a&\xC9V[a7V\x81\x87a&\xC9V[a7V\x81\x86a&\xC9V[\x80\x93a&\xC9V[\x82\x11\x90V[\x90\x03\x90V[`\x01\x1Ba7CV[`\x04\x1C\x91`\x02\x1B\x91a7<V[`\x08\x1C\x91`\x04\x1B\x91a72V[`\x10\x1C\x91`\x08\x1B\x91a7'V[` \x1C\x91`\x10\x1B\x91a7\x1BV[`@\x1C\x91` \x1B\x91a7\rV[PPa7\x9Ba7\x96a7\x8Fa7\x85a7{a7qa7ga7]a7La8\x10\x8A`\x80\x1C\x90V[\x98Ph\x01\0\0\0\0\0\0\0\0\x97Pa6\xDC\x96PPPPPPPV[\x91\x90[\x83\x82\x10a8;WPPP\x90V[\x90\x91\x92\x80\x83\x16\x90\x80\x84\x18`\x01\x1C\x82\x01\x80\x92\x11a\x0E\xCCW\x84_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x10_\x14a8}WP\x92[\x91\x90a8.V[\x93\x92Pa8\x89\x90a&aV[\x91a8vV[\x91\x90`\x01`\x01`\xA0\x1B\x03\x81\x16\x92`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x84\x82\x14\x15\x80a9\xD9W[a8\xBDW[PPPPPV[\x81a9cW[PP\x82a8\xD2W[\x80\x80a8\xB6V[a9Xa9?\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x93a99a93y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95`\x01`\x01`\xA0\x1B\x03\x16_R`\n` R`@_ \x90V[\x91a<\x86V[\x90a=ZV[`@\x80Q\x92\x85\x16\x83R\x93\x16` \x82\x01R\x91\x82\x91\x90\x82\x01\x90V[\x03\x90\xA2_\x80\x80a8\xCBV[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa9\xCFa9?a9\xC0\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x94`\x01`\x01`\xA0\x1B\x03\x16_R`\n` R`@_ \x90V[a9\xC9\x88a<\x86V[\x90a<\xF6V[\x03\x90\xA2_\x80a8\xC3V[P\x83\x15\x15a8\xB1V[_\x81\x81R`\x0F` R`@\x90 Ta'fW`\x0ETh\x01\0\0\0\0\0\0\0\0\x81\x10\x15a%\xFFWa:4a:\x1E\x82`\x01\x85\x94\x01`\x0EU`\x0Ea4\xF5V[\x81\x93\x91T\x90_\x19\x90`\x03\x1B\x92\x83\x1B\x92\x1B\x19\x16\x17\x90V[\x90U`\x0ET\x90_R`\x0F` R`@_ U`\x01\x90V[`\xFF\x81\x14a:\xAAW`\xFF\x81\x16\x90`\x1F\x82\x11a:\x82W`@Q\x91a:o`@\x84a& V[` \x80\x84R\x83\x81\x01\x91\x906\x837\x83RR\x90V[\x7F\xB3Q+\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P`@Qa\x06\xFD\x81a:\xBD\x81`\x06a%4V[\x03\x82a& V[`\xFF\x81\x14a:\xE8W`\xFF\x81\x16\x90`\x1F\x82\x11a:\x82W`@Q\x91a:o`@\x84a& V[P`@Qa\x06\xFD\x81a:\xBD\x81`\x07a%4V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a;}W\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a;rW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a;hW\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[`@Q=_\x82>=\x90\xFD[PPP_\x91`\x03\x91\x90V[`\x04\x11\x15a;\x92WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[a;\xC8\x81a;\x88V[\x80a;\xD1WPPV[a;\xDA\x81a;\x88V[`\x01\x81\x03a<\nW\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[a<\x13\x81a;\x88V[`\x02\x81\x03a<GWP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80a<S`\x03\x92a;\x88V[\x14a<[WPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a<\xC6Wy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\xD0`\x04R`$R`D_\xFD[\x90a=\0Ca6nV[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a=&\x85a42V[\x92\x16\x91\x16\x03\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0E\xCCWa=V\x92a?\xA3V[\x90\x91V[\x90a=dCa6nV[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a=\x8A\x85a42V[\x92\x16\x91\x16\x01\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0E\xCCWa=V\x92a?\xA3V[a=\xC3Ca6nV[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a=\xEA`\x0Ba42V[\x92\x16\x91\x16\x03y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0E\xCCWa=V\x91`\x0Ba?\xA3V[\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a>\x8CW[`\x01`\x01`\xA0\x1B\x03a&\x8B\x93\x16\x90\x81\x15a>tW[_R`\t` R`\x01`\x01`\xA0\x1B\x03`@_ T\x16\x90_R`\t` R`\x01`\x01`\xA0\x1B\x03`@_ T\x16\x90a8\x8FV[a>\x85a>\x80\x84a<\x86V[a=\xBAV[PPa>CV[a>\x95\x82a<\x86V[\x92a>\x9FCa6nV[\x93y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a>\xC6`\x0Ba42V[\x92\x16\x91\x16\x01\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0E\xCCWa&\x8B\x94`\x01`\x01`\xA0\x1B\x03\x92a?\x05\x91`\x0Ba?\xA3V[\x90PP\x93PPa>.V[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a%\xFFWa?2\x91`\x01\x82\x01\x81Ua4\xF5V[a?wW\x81Q` \x92\x90\x92\x01Q`0\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x80T\x92\x93\x92\x80\x15a@\x99Wa?\xBAa?\xC5\x91a'\0V[\x82_R` _ \x01\x90V[\x80T`0\x81\x90\x1C\x93e\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92\x91\x81\x16\x80\x84\x11a@qW\x87\x93\x03a@*WPa@&\x92P\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x83T\x92`0\x1B\x16\x91\x16\x17\x90UV[\x91\x90V[\x91PPa@&\x91a@Ja@<a&\x8DV[e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16` \x83\x01Ra?\x10V[\x7F% `\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P\x90a@\xD1\x91a@\xAAa@<a&\x8DV[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16` \x83\x01Ra?\x10V[_\x91\x90V",
    );
    /**Custom error with signature `AccessControlBadConfirmation()` and selector `0x6697b232`.
```solidity
error AccessControlBadConfirmation();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlBadConfirmation {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<AccessControlBadConfirmation>
        for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlBadConfirmation) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AccessControlBadConfirmation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlBadConfirmation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlBadConfirmation()";
            const SELECTOR: [u8; 4] = [102u8, 151u8, 178u8, 50u8];
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
    /**Custom error with signature `AccessControlUnauthorizedAccount(address,bytes32)` and selector `0xe2517d3f`.
```solidity
error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlUnauthorizedAccount {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub neededRole: alloy::sol_types::private::FixedBytes<32>,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<AccessControlUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlUnauthorizedAccount) -> Self {
                (value.account, value.neededRole)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AccessControlUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    account: tuple.0,
                    neededRole: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlUnauthorizedAccount(address,bytes32)";
            const SELECTOR: [u8; 4] = [226u8, 81u8, 125u8, 63u8];
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
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.neededRole),
                )
            }
        }
    };
    /**Custom error with signature `BridgeMustBeContract()` and selector `0x825431da`.
```solidity
error BridgeMustBeContract();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BridgeMustBeContract {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<BridgeMustBeContract> for UnderlyingRustTuple<'_> {
            fn from(value: BridgeMustBeContract) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BridgeMustBeContract {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BridgeMustBeContract {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BridgeMustBeContract()";
            const SELECTOR: [u8; 4] = [130u8, 84u8, 49u8, 218u8];
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
    /**Custom error with signature `BridgeNotActive(address)` and selector `0xefda0e06`.
```solidity
error BridgeNotActive(address bridge);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BridgeNotActive {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<BridgeNotActive> for UnderlyingRustTuple<'_> {
            fn from(value: BridgeNotActive) -> Self {
                (value.bridge,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BridgeNotActive {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { bridge: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BridgeNotActive {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BridgeNotActive(address)";
            const SELECTOR: [u8; 4] = [239u8, 218u8, 14u8, 6u8];
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
                        &self.bridge,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `BurnOnlyDuringLockPeriod()` and selector `0xb8b5ca2d`.
```solidity
error BurnOnlyDuringLockPeriod();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BurnOnlyDuringLockPeriod {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<BurnOnlyDuringLockPeriod>
        for UnderlyingRustTuple<'_> {
            fn from(value: BurnOnlyDuringLockPeriod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for BurnOnlyDuringLockPeriod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BurnOnlyDuringLockPeriod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BurnOnlyDuringLockPeriod()";
            const SELECTOR: [u8; 4] = [184u8, 181u8, 202u8, 45u8];
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
    /**Custom error with signature `CannotAddSelfAsBridge()` and selector `0xfb8ce8c9`.
```solidity
error CannotAddSelfAsBridge();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotAddSelfAsBridge {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<CannotAddSelfAsBridge> for UnderlyingRustTuple<'_> {
            fn from(value: CannotAddSelfAsBridge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotAddSelfAsBridge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotAddSelfAsBridge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotAddSelfAsBridge()";
            const SELECTOR: [u8; 4] = [251u8, 140u8, 232u8, 201u8];
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
    /**Custom error with signature `CheckpointUnorderedInsertion()` and selector `0x2520601d`.
```solidity
error CheckpointUnorderedInsertion();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckpointUnorderedInsertion {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<CheckpointUnorderedInsertion>
        for UnderlyingRustTuple<'_> {
            fn from(value: CheckpointUnorderedInsertion) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CheckpointUnorderedInsertion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CheckpointUnorderedInsertion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CheckpointUnorderedInsertion()";
            const SELECTOR: [u8; 4] = [37u8, 32u8, 96u8, 29u8];
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
    /**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
```solidity
error ECDSAInvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignature {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<ECDSAInvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignature()";
            const SELECTOR: [u8; 4] = [246u8, 69u8, 238u8, 223u8];
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
    /**Custom error with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`.
```solidity
error ECDSAInvalidSignatureLength(uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureLength {
        #[allow(missing_docs)]
        pub length: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<ECDSAInvalidSignatureLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureLength) -> Self {
                (value.length,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ECDSAInvalidSignatureLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { length: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureLength(uint256)";
            const SELECTOR: [u8; 4] = [252u8, 230u8, 152u8, 247u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
        }
    };
    /**Custom error with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`.
```solidity
error ECDSAInvalidSignatureS(bytes32 s);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureS {
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<ECDSAInvalidSignatureS> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureS) -> Self {
                (value.s,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { s: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureS(bytes32)";
            const SELECTOR: [u8; 4] = [215u8, 139u8, 206u8, 12u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
        }
    };
    /**Custom error with signature `ERC20ExceededSafeSupply(uint256,uint256)` and selector `0x1cb15d26`.
```solidity
error ERC20ExceededSafeSupply(uint256 increasedSupply, uint256 cap);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20ExceededSafeSupply {
        #[allow(missing_docs)]
        pub increasedSupply: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub cap: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<ERC20ExceededSafeSupply> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20ExceededSafeSupply) -> Self {
                (value.increasedSupply, value.cap)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20ExceededSafeSupply {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    increasedSupply: tuple.0,
                    cap: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20ExceededSafeSupply {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20ExceededSafeSupply(uint256,uint256)";
            const SELECTOR: [u8; 4] = [28u8, 177u8, 93u8, 38u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.increasedSupply),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.cap),
                )
            }
        }
    };
    /**Custom error with signature `ERC20InsufficientAllowance(address,uint256,uint256)` and selector `0xfb8f41b2`.
```solidity
error ERC20InsufficientAllowance(address spender, uint256 allowance, uint256 needed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InsufficientAllowance {
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub allowance: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub needed: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<ERC20InsufficientAllowance>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InsufficientAllowance) -> Self {
                (value.spender, value.allowance, value.needed)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC20InsufficientAllowance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    spender: tuple.0,
                    allowance: tuple.1,
                    needed: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InsufficientAllowance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InsufficientAllowance(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [251u8, 143u8, 65u8, 178u8];
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
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.allowance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.needed),
                )
            }
        }
    };
    /**Custom error with signature `ERC20InsufficientBalance(address,uint256,uint256)` and selector `0xe450d38c`.
```solidity
error ERC20InsufficientBalance(address sender, uint256 balance, uint256 needed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InsufficientBalance {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub balance: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub needed: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<ERC20InsufficientBalance>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InsufficientBalance) -> Self {
                (value.sender, value.balance, value.needed)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC20InsufficientBalance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    sender: tuple.0,
                    balance: tuple.1,
                    needed: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InsufficientBalance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InsufficientBalance(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [228u8, 80u8, 211u8, 140u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.balance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.needed),
                )
            }
        }
    };
    /**Custom error with signature `ERC20InvalidApprover(address)` and selector `0xe602df05`.
```solidity
error ERC20InvalidApprover(address approver);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InvalidApprover {
        #[allow(missing_docs)]
        pub approver: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<ERC20InvalidApprover> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InvalidApprover) -> Self {
                (value.approver,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20InvalidApprover {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { approver: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InvalidApprover {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InvalidApprover(address)";
            const SELECTOR: [u8; 4] = [230u8, 2u8, 223u8, 5u8];
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
                        &self.approver,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `ERC20InvalidReceiver(address)` and selector `0xec442f05`.
```solidity
error ERC20InvalidReceiver(address receiver);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InvalidReceiver {
        #[allow(missing_docs)]
        pub receiver: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<ERC20InvalidReceiver> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InvalidReceiver) -> Self {
                (value.receiver,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20InvalidReceiver {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { receiver: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InvalidReceiver {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InvalidReceiver(address)";
            const SELECTOR: [u8; 4] = [236u8, 68u8, 47u8, 5u8];
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
                        &self.receiver,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `ERC20InvalidSender(address)` and selector `0x96c6fd1e`.
```solidity
error ERC20InvalidSender(address sender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InvalidSender {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<ERC20InvalidSender> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InvalidSender) -> Self {
                (value.sender,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20InvalidSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { sender: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InvalidSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InvalidSender(address)";
            const SELECTOR: [u8; 4] = [150u8, 198u8, 253u8, 30u8];
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
                )
            }
        }
    };
    /**Custom error with signature `ERC20InvalidSpender(address)` and selector `0x94280d62`.
```solidity
error ERC20InvalidSpender(address spender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InvalidSpender {
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<ERC20InvalidSpender> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InvalidSpender) -> Self {
                (value.spender,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20InvalidSpender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { spender: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InvalidSpender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InvalidSpender(address)";
            const SELECTOR: [u8; 4] = [148u8, 40u8, 13u8, 98u8];
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
                        &self.spender,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `ERC2612ExpiredSignature(uint256)` and selector `0x62791302`.
```solidity
error ERC2612ExpiredSignature(uint256 deadline);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC2612ExpiredSignature {
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<ERC2612ExpiredSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ERC2612ExpiredSignature) -> Self {
                (value.deadline,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC2612ExpiredSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { deadline: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC2612ExpiredSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC2612ExpiredSignature(uint256)";
            const SELECTOR: [u8; 4] = [98u8, 121u8, 19u8, 2u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
        }
    };
    /**Custom error with signature `ERC2612InvalidSigner(address,address)` and selector `0x4b800e46`.
```solidity
error ERC2612InvalidSigner(address signer, address owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC2612InvalidSigner {
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<ERC2612InvalidSigner> for UnderlyingRustTuple<'_> {
            fn from(value: ERC2612InvalidSigner) -> Self {
                (value.signer, value.owner)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC2612InvalidSigner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signer: tuple.0,
                    owner: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC2612InvalidSigner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC2612InvalidSigner(address,address)";
            const SELECTOR: [u8; 4] = [75u8, 128u8, 14u8, 70u8];
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
                        &self.signer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `ERC5805FutureLookup(uint256,uint48)` and selector `0xecd3f81e`.
```solidity
error ERC5805FutureLookup(uint256 timepoint, uint48 clock);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC5805FutureLookup {
        #[allow(missing_docs)]
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub clock: alloy::sol_types::private::primitives::aliases::U48,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<48>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U48,
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
        impl ::core::convert::From<ERC5805FutureLookup> for UnderlyingRustTuple<'_> {
            fn from(value: ERC5805FutureLookup) -> Self {
                (value.timepoint, value.clock)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC5805FutureLookup {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    timepoint: tuple.0,
                    clock: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC5805FutureLookup {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC5805FutureLookup(uint256,uint48)";
            const SELECTOR: [u8; 4] = [236u8, 211u8, 248u8, 30u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self.clock),
                )
            }
        }
    };
    /**Custom error with signature `ERC6372InconsistentClock()` and selector `0x6ff07140`.
```solidity
error ERC6372InconsistentClock();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC6372InconsistentClock {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<ERC6372InconsistentClock>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC6372InconsistentClock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC6372InconsistentClock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC6372InconsistentClock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC6372InconsistentClock()";
            const SELECTOR: [u8; 4] = [111u8, 240u8, 113u8, 64u8];
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
    /**Custom error with signature `ExceedsTotalSupply()` and selector `0x177e3fc3`.
```solidity
error ExceedsTotalSupply();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExceedsTotalSupply {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<ExceedsTotalSupply> for UnderlyingRustTuple<'_> {
            fn from(value: ExceedsTotalSupply) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExceedsTotalSupply {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExceedsTotalSupply {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExceedsTotalSupply()";
            const SELECTOR: [u8; 4] = [23u8, 126u8, 63u8, 195u8];
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
    /**Custom error with signature `InsufficientBurnLimit(address,uint256,uint256)` and selector `0xe5fe97a2`.
```solidity
error InsufficientBurnLimit(address bridge, uint256 requested, uint256 available);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientBurnLimit {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub requested: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub available: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<InsufficientBurnLimit> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientBurnLimit) -> Self {
                (value.bridge, value.requested, value.available)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientBurnLimit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bridge: tuple.0,
                    requested: tuple.1,
                    available: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientBurnLimit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientBurnLimit(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [229u8, 254u8, 151u8, 162u8];
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
                        &self.bridge,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requested),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.available),
                )
            }
        }
    };
    /**Custom error with signature `InsufficientEmissionBudget()` and selector `0x7ade115c`.
```solidity
error InsufficientEmissionBudget();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientEmissionBudget {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<InsufficientEmissionBudget>
        for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientEmissionBudget) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InsufficientEmissionBudget {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientEmissionBudget {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientEmissionBudget()";
            const SELECTOR: [u8; 4] = [122u8, 222u8, 17u8, 92u8];
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
    /**Custom error with signature `InsufficientMintLimit(address,uint256,uint256)` and selector `0x40ed367b`.
```solidity
error InsufficientMintLimit(address bridge, uint256 requested, uint256 available);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientMintLimit {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub requested: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub available: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<InsufficientMintLimit> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientMintLimit) -> Self {
                (value.bridge, value.requested, value.available)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientMintLimit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bridge: tuple.0,
                    requested: tuple.1,
                    available: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientMintLimit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientMintLimit(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [64u8, 237u8, 54u8, 123u8];
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
                        &self.bridge,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requested),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.available),
                )
            }
        }
    };
    /**Custom error with signature `InvalidAccountNonce(address,uint256)` and selector `0x752d88c0`.
```solidity
error InvalidAccountNonce(address account, uint256 currentNonce);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAccountNonce {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub currentNonce: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<InvalidAccountNonce> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAccountNonce) -> Self {
                (value.account, value.currentNonce)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAccountNonce {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    account: tuple.0,
                    currentNonce: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAccountNonce {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAccountNonce(address,uint256)";
            const SELECTOR: [u8; 4] = [117u8, 45u8, 136u8, 192u8];
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
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentNonce),
                )
            }
        }
    };
    /**Custom error with signature `InvalidShortString()` and selector `0xb3512b0c`.
```solidity
error InvalidShortString();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidShortString {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<InvalidShortString> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidShortString) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidShortString {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidShortString {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidShortString()";
            const SELECTOR: [u8; 4] = [179u8, 81u8, 43u8, 12u8];
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
    /**Custom error with signature `SafeCastOverflowedUintDowncast(uint8,uint256)` and selector `0x6dfcc650`.
```solidity
error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SafeCastOverflowedUintDowncast {
        #[allow(missing_docs)]
        pub bits: u8,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u8,
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
        impl ::core::convert::From<SafeCastOverflowedUintDowncast>
        for UnderlyingRustTuple<'_> {
            fn from(value: SafeCastOverflowedUintDowncast) -> Self {
                (value.bits, value.value)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SafeCastOverflowedUintDowncast {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bits: tuple.0,
                    value: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SafeCastOverflowedUintDowncast {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SafeCastOverflowedUintDowncast(uint8,uint256)";
            const SELECTOR: [u8; 4] = [109u8, 252u8, 198u8, 80u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.bits),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
        }
    };
    /**Custom error with signature `StringTooLong(string)` and selector `0x305a27a9`.
```solidity
error StringTooLong(string str);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StringTooLong {
        #[allow(missing_docs)]
        pub str: alloy::sol_types::private::String,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<StringTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: StringTooLong) -> Self {
                (value.str,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StringTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { str: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StringTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StringTooLong(string)";
            const SELECTOR: [u8; 4] = [48u8, 90u8, 39u8, 169u8];
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
                        &self.str,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `TransfersLocked()` and selector `0xdb89e3f4`.
```solidity
error TransfersLocked();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TransfersLocked {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<TransfersLocked> for UnderlyingRustTuple<'_> {
            fn from(value: TransfersLocked) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TransfersLocked {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TransfersLocked {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TransfersLocked()";
            const SELECTOR: [u8; 4] = [219u8, 137u8, 227u8, 244u8];
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
    /**Custom error with signature `UnauthorizedBridge(address)` and selector `0x6585b60d`.
```solidity
error UnauthorizedBridge(address bridge);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnauthorizedBridge {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<UnauthorizedBridge> for UnderlyingRustTuple<'_> {
            fn from(value: UnauthorizedBridge) -> Self {
                (value.bridge,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnauthorizedBridge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { bridge: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnauthorizedBridge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnauthorizedBridge(address)";
            const SELECTOR: [u8; 4] = [101u8, 133u8, 182u8, 13u8];
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
                        &self.bridge,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `UnlockTimestampInPast()` and selector `0xa5658353`.
```solidity
error UnlockTimestampInPast();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnlockTimestampInPast {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<UnlockTimestampInPast> for UnderlyingRustTuple<'_> {
            fn from(value: UnlockTimestampInPast) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnlockTimestampInPast {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnlockTimestampInPast {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnlockTimestampInPast()";
            const SELECTOR: [u8; 4] = [165u8, 101u8, 131u8, 83u8];
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
    /**Custom error with signature `UnlockTimestampTooLate()` and selector `0xef69af65`.
```solidity
error UnlockTimestampTooLate();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnlockTimestampTooLate {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<UnlockTimestampTooLate> for UnderlyingRustTuple<'_> {
            fn from(value: UnlockTimestampTooLate) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnlockTimestampTooLate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnlockTimestampTooLate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnlockTimestampTooLate()";
            const SELECTOR: [u8; 4] = [239u8, 105u8, 175u8, 101u8];
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
    /**Custom error with signature `UnreasonableBurnLimit()` and selector `0x58ccad00`.
```solidity
error UnreasonableBurnLimit();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnreasonableBurnLimit {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<UnreasonableBurnLimit> for UnderlyingRustTuple<'_> {
            fn from(value: UnreasonableBurnLimit) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnreasonableBurnLimit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnreasonableBurnLimit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnreasonableBurnLimit()";
            const SELECTOR: [u8; 4] = [88u8, 204u8, 173u8, 0u8];
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
    /**Custom error with signature `UnreasonableMintLimit()` and selector `0x0a395c01`.
```solidity
error UnreasonableMintLimit();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnreasonableMintLimit {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<UnreasonableMintLimit> for UnderlyingRustTuple<'_> {
            fn from(value: UnreasonableMintLimit) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnreasonableMintLimit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnreasonableMintLimit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnreasonableMintLimit()";
            const SELECTOR: [u8; 4] = [10u8, 57u8, 92u8, 1u8];
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
    /**Custom error with signature `VotesExpiredSignature(uint256)` and selector `0x4683af0e`.
```solidity
error VotesExpiredSignature(uint256 expiry);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VotesExpiredSignature {
        #[allow(missing_docs)]
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<VotesExpiredSignature> for UnderlyingRustTuple<'_> {
            fn from(value: VotesExpiredSignature) -> Self {
                (value.expiry,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for VotesExpiredSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { expiry: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for VotesExpiredSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "VotesExpiredSignature(uint256)";
            const SELECTOR: [u8; 4] = [70u8, 131u8, 175u8, 14u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                )
            }
        }
    };
    /**Custom error with signature `ZeroAddress()` and selector `0xd92e233d`.
```solidity
error ZeroAddress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAddress {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<ZeroAddress> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroAddress) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroAddress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroAddress()";
            const SELECTOR: [u8; 4] = [217u8, 46u8, 35u8, 61u8];
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
    /**Custom error with signature `ZeroAmount()` and selector `0x1f2a2005`.
```solidity
error ZeroAmount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAmount {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<ZeroAmount> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroAmount()";
            const SELECTOR: [u8; 4] = [31u8, 42u8, 32u8, 5u8];
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
    /**Event with signature `Approval(address,address,uint256)` and selector `0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925`.
```solidity
event Approval(address indexed owner, address indexed spender, uint256 value);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Approval {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Approval {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Approval(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                140u8,
                91u8,
                225u8,
                229u8,
                235u8,
                236u8,
                125u8,
                91u8,
                209u8,
                79u8,
                113u8,
                66u8,
                125u8,
                30u8,
                132u8,
                243u8,
                221u8,
                3u8,
                20u8,
                192u8,
                247u8,
                178u8,
                41u8,
                30u8,
                91u8,
                32u8,
                10u8,
                200u8,
                199u8,
                195u8,
                185u8,
                37u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    owner: topics.1,
                    spender: topics.2,
                    value: data.0,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.owner.clone(), self.spender.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.owner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.spender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Approval {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Approval> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Approval) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `BridgeActiveStatusChanged(address,bool)` and selector `0x9c8668db324845065d2b9a2a183bd3141f63018f548282daf18da49ccbf88c33`.
```solidity
event BridgeActiveStatusChanged(address indexed bridge, bool isActive);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BridgeActiveStatusChanged {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub isActive: bool,
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
        impl alloy_sol_types::SolEvent for BridgeActiveStatusChanged {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "BridgeActiveStatusChanged(address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8,
                134u8,
                104u8,
                219u8,
                50u8,
                72u8,
                69u8,
                6u8,
                93u8,
                43u8,
                154u8,
                42u8,
                24u8,
                59u8,
                211u8,
                20u8,
                31u8,
                99u8,
                1u8,
                143u8,
                84u8,
                130u8,
                130u8,
                218u8,
                241u8,
                141u8,
                164u8,
                156u8,
                203u8,
                248u8,
                140u8,
                51u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    bridge: topics.1,
                    isActive: data.0,
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isActive,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.bridge.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.bridge,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BridgeActiveStatusChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BridgeActiveStatusChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &BridgeActiveStatusChanged,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `BridgeAdded(address,uint256,uint256)` and selector `0xdb03f97dc5840a71e69be7470e4761af10a1237973e81c12d0dc2813895a6526`.
```solidity
event BridgeAdded(address indexed bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BridgeAdded {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub dailyMintLimit: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub dailyBurnLimit: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for BridgeAdded {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "BridgeAdded(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                219u8,
                3u8,
                249u8,
                125u8,
                197u8,
                132u8,
                10u8,
                113u8,
                230u8,
                155u8,
                231u8,
                71u8,
                14u8,
                71u8,
                97u8,
                175u8,
                16u8,
                161u8,
                35u8,
                121u8,
                115u8,
                232u8,
                28u8,
                18u8,
                208u8,
                220u8,
                40u8,
                19u8,
                137u8,
                90u8,
                101u8,
                38u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    bridge: topics.1,
                    dailyMintLimit: data.0,
                    dailyBurnLimit: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.dailyMintLimit),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.dailyBurnLimit),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.bridge.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.bridge,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BridgeAdded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BridgeAdded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BridgeAdded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `BridgeLimitsSet(address,uint256,uint256)` and selector `0xaa807d0abf30d91968c7478c66b6d82521a106af13eda036e2036da9af168958`.
```solidity
event BridgeLimitsSet(address indexed bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BridgeLimitsSet {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub dailyMintLimit: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub dailyBurnLimit: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for BridgeLimitsSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "BridgeLimitsSet(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                170u8,
                128u8,
                125u8,
                10u8,
                191u8,
                48u8,
                217u8,
                25u8,
                104u8,
                199u8,
                71u8,
                140u8,
                102u8,
                182u8,
                216u8,
                37u8,
                33u8,
                161u8,
                6u8,
                175u8,
                19u8,
                237u8,
                160u8,
                54u8,
                226u8,
                3u8,
                109u8,
                169u8,
                175u8,
                22u8,
                137u8,
                88u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    bridge: topics.1,
                    dailyMintLimit: data.0,
                    dailyBurnLimit: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.dailyMintLimit),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.dailyBurnLimit),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.bridge.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.bridge,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BridgeLimitsSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BridgeLimitsSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BridgeLimitsSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `BridgeRemoved(address)` and selector `0x5d9d5034656cb3ebfb0655057cd7f9b4077a9b42ff42ce223cbac5bc586d2126`.
```solidity
event BridgeRemoved(address indexed bridge);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BridgeRemoved {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for BridgeRemoved {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "BridgeRemoved(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8,
                157u8,
                80u8,
                52u8,
                101u8,
                108u8,
                179u8,
                235u8,
                251u8,
                6u8,
                85u8,
                5u8,
                124u8,
                215u8,
                249u8,
                180u8,
                7u8,
                122u8,
                155u8,
                66u8,
                255u8,
                66u8,
                206u8,
                34u8,
                60u8,
                186u8,
                197u8,
                188u8,
                88u8,
                109u8,
                33u8,
                38u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { bridge: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.bridge.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.bridge,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BridgeRemoved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BridgeRemoved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BridgeRemoved) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `CrosschainBurn(address,uint256,address)` and selector `0xb90795a66650155983e242cac3e1ac1a4dc26f8ed2987f3ce416a34e00111fd4`.
```solidity
event CrosschainBurn(address indexed from, uint256 amount, address indexed bridge);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CrosschainBurn {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for CrosschainBurn {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "CrosschainBurn(address,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                185u8,
                7u8,
                149u8,
                166u8,
                102u8,
                80u8,
                21u8,
                89u8,
                131u8,
                226u8,
                66u8,
                202u8,
                195u8,
                225u8,
                172u8,
                26u8,
                77u8,
                194u8,
                111u8,
                142u8,
                210u8,
                152u8,
                127u8,
                60u8,
                228u8,
                22u8,
                163u8,
                78u8,
                0u8,
                17u8,
                31u8,
                212u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    from: topics.1,
                    amount: data.0,
                    bridge: topics.2,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.from.clone(), self.bridge.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.from,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.bridge,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CrosschainBurn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CrosschainBurn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CrosschainBurn) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `CrosschainMint(address,uint256,address)` and selector `0xde22baff038e3a3e08407cbdf617deed74e869a7ba517df611e33131c6e6ea04`.
```solidity
event CrosschainMint(address indexed to, uint256 amount, address indexed bridge);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CrosschainMint {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for CrosschainMint {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "CrosschainMint(address,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                222u8,
                34u8,
                186u8,
                255u8,
                3u8,
                142u8,
                58u8,
                62u8,
                8u8,
                64u8,
                124u8,
                189u8,
                246u8,
                23u8,
                222u8,
                237u8,
                116u8,
                232u8,
                105u8,
                167u8,
                186u8,
                81u8,
                125u8,
                246u8,
                17u8,
                227u8,
                49u8,
                49u8,
                198u8,
                230u8,
                234u8,
                4u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    to: topics.1,
                    amount: data.0,
                    bridge: topics.2,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.to.clone(), self.bridge.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.bridge,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CrosschainMint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CrosschainMint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CrosschainMint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DelegateChanged(address,address,address)` and selector `0x3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f`.
```solidity
event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DelegateChanged {
        #[allow(missing_docs)]
        pub delegator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub fromDelegate: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub toDelegate: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for DelegateChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DelegateChanged(address,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8,
                52u8,
                232u8,
                162u8,
                230u8,
                217u8,
                126u8,
                146u8,
                154u8,
                126u8,
                84u8,
                1u8,
                30u8,
                165u8,
                72u8,
                93u8,
                125u8,
                25u8,
                109u8,
                213u8,
                240u8,
                186u8,
                77u8,
                78u8,
                249u8,
                88u8,
                3u8,
                232u8,
                227u8,
                252u8,
                37u8,
                127u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    delegator: topics.1,
                    fromDelegate: topics.2,
                    toDelegate: topics.3,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.delegator.clone(),
                    self.fromDelegate.clone(),
                    self.toDelegate.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.delegator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.fromDelegate,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.toDelegate,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DelegateChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelegateChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DelegateChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DelegateVotesChanged(address,uint256,uint256)` and selector `0xdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724`.
```solidity
event DelegateVotesChanged(address indexed delegate, uint256 previousVotes, uint256 newVotes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DelegateVotesChanged {
        #[allow(missing_docs)]
        pub delegate: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub previousVotes: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newVotes: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for DelegateVotesChanged {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DelegateVotesChanged(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                222u8,
                194u8,
                186u8,
                205u8,
                210u8,
                240u8,
                91u8,
                89u8,
                222u8,
                52u8,
                218u8,
                155u8,
                82u8,
                61u8,
                255u8,
                139u8,
                228u8,
                46u8,
                94u8,
                56u8,
                232u8,
                24u8,
                200u8,
                47u8,
                219u8,
                11u8,
                174u8,
                119u8,
                67u8,
                135u8,
                167u8,
                36u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    delegate: topics.1,
                    previousVotes: data.0,
                    newVotes: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.previousVotes),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newVotes),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.delegate.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.delegate,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DelegateVotesChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelegateVotesChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DelegateVotesChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EIP712DomainChanged()` and selector `0x0a6387c9ea3628b88a633bb4f3b151770f70085117a15f9bf3787cda53f13d31`.
```solidity
event EIP712DomainChanged();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EIP712DomainChanged {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EIP712DomainChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EIP712DomainChanged()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                10u8,
                99u8,
                135u8,
                201u8,
                234u8,
                54u8,
                40u8,
                184u8,
                138u8,
                99u8,
                59u8,
                180u8,
                243u8,
                177u8,
                81u8,
                119u8,
                15u8,
                112u8,
                8u8,
                81u8,
                23u8,
                161u8,
                95u8,
                155u8,
                243u8,
                120u8,
                124u8,
                218u8,
                83u8,
                241u8,
                61u8,
                49u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
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
        impl alloy_sol_types::private::IntoLogData for EIP712DomainChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EIP712DomainChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EIP712DomainChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EmissionBudgetAllocated(address,uint256)` and selector `0x9ca03dbd5193fbb7974173cedd0bdf6841dd14c3cbfa735aab77ff1dd1139fb3`.
```solidity
event EmissionBudgetAllocated(address indexed bridge, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EmissionBudgetAllocated {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for EmissionBudgetAllocated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "EmissionBudgetAllocated(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8,
                160u8,
                61u8,
                189u8,
                81u8,
                147u8,
                251u8,
                183u8,
                151u8,
                65u8,
                115u8,
                206u8,
                221u8,
                11u8,
                223u8,
                104u8,
                65u8,
                221u8,
                20u8,
                195u8,
                203u8,
                250u8,
                115u8,
                90u8,
                171u8,
                119u8,
                255u8,
                29u8,
                209u8,
                19u8,
                159u8,
                179u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    bridge: topics.1,
                    amount: data.0,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.bridge.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.bridge,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EmissionBudgetAllocated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EmissionBudgetAllocated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &EmissionBudgetAllocated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EmissionBudgetConsumed(address,uint256)` and selector `0xbc23ec7f1313150b047bff83d0845b0564baa134698dd11bb0acd0f7d416de7d`.
```solidity
event EmissionBudgetConsumed(address indexed bridge, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EmissionBudgetConsumed {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for EmissionBudgetConsumed {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "EmissionBudgetConsumed(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                188u8,
                35u8,
                236u8,
                127u8,
                19u8,
                19u8,
                21u8,
                11u8,
                4u8,
                123u8,
                255u8,
                131u8,
                208u8,
                132u8,
                91u8,
                5u8,
                100u8,
                186u8,
                161u8,
                52u8,
                105u8,
                141u8,
                209u8,
                27u8,
                176u8,
                172u8,
                208u8,
                247u8,
                212u8,
                22u8,
                222u8,
                125u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    bridge: topics.1,
                    amount: data.0,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.bridge.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.bridge,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EmissionBudgetConsumed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EmissionBudgetConsumed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EmissionBudgetConsumed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RoleAdminChanged(bytes32,bytes32,bytes32)` and selector `0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff`.
```solidity
event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleAdminChanged {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub previousAdminRole: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub newAdminRole: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for RoleAdminChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "RoleAdminChanged(bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                189u8,
                121u8,
                184u8,
                111u8,
                254u8,
                10u8,
                184u8,
                232u8,
                119u8,
                97u8,
                81u8,
                81u8,
                66u8,
                23u8,
                205u8,
                124u8,
                172u8,
                213u8,
                44u8,
                144u8,
                159u8,
                102u8,
                71u8,
                92u8,
                58u8,
                244u8,
                78u8,
                18u8,
                159u8,
                11u8,
                0u8,
                255u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    previousAdminRole: topics.2,
                    newAdminRole: topics.3,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.previousAdminRole.clone(),
                    self.newAdminRole.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.previousAdminRole);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.newAdminRole);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleAdminChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleAdminChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleAdminChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RoleGranted(bytes32,address,address)` and selector `0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d`.
```solidity
event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleGranted {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RoleGranted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleGranted(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8,
                135u8,
                136u8,
                17u8,
                126u8,
                126u8,
                255u8,
                29u8,
                130u8,
                233u8,
                38u8,
                236u8,
                121u8,
                73u8,
                1u8,
                209u8,
                124u8,
                120u8,
                2u8,
                74u8,
                80u8,
                39u8,
                9u8,
                64u8,
                48u8,
                69u8,
                64u8,
                167u8,
                51u8,
                101u8,
                111u8,
                13u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleGranted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleGranted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleGranted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RoleRevoked(bytes32,address,address)` and selector `0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b`.
```solidity
event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleRevoked {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RoleRevoked {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleRevoked(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                246u8,
                57u8,
                31u8,
                92u8,
                50u8,
                217u8,
                198u8,
                157u8,
                42u8,
                71u8,
                234u8,
                103u8,
                11u8,
                68u8,
                41u8,
                116u8,
                181u8,
                57u8,
                53u8,
                209u8,
                237u8,
                199u8,
                253u8,
                100u8,
                235u8,
                33u8,
                224u8,
                71u8,
                168u8,
                57u8,
                23u8,
                27u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleRevoked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleRevoked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleRevoked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TokensBurnedByManager(address,uint256,address)` and selector `0xbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a2`.
```solidity
event TokensBurnedByManager(address indexed from, uint256 amount, address indexed burner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TokensBurnedByManager {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub burner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for TokensBurnedByManager {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TokensBurnedByManager(address,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                190u8,
                244u8,
                248u8,
                28u8,
                24u8,
                20u8,
                198u8,
                65u8,
                237u8,
                232u8,
                94u8,
                186u8,
                172u8,
                241u8,
                157u8,
                4u8,
                139u8,
                44u8,
                91u8,
                85u8,
                152u8,
                10u8,
                223u8,
                166u8,
                239u8,
                15u8,
                149u8,
                108u8,
                101u8,
                19u8,
                53u8,
                162u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    from: topics.1,
                    amount: data.0,
                    burner: topics.2,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.from.clone(), self.burner.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.from,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.burner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TokensBurnedByManager {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TokensBurnedByManager> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TokensBurnedByManager) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Transfer(address,address,uint256)` and selector `0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef`.
```solidity
event Transfer(address indexed from, address indexed to, uint256 value);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Transfer {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Transfer {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Transfer(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                221u8,
                242u8,
                82u8,
                173u8,
                27u8,
                226u8,
                200u8,
                155u8,
                105u8,
                194u8,
                176u8,
                104u8,
                252u8,
                55u8,
                141u8,
                170u8,
                149u8,
                43u8,
                167u8,
                241u8,
                99u8,
                196u8,
                161u8,
                22u8,
                40u8,
                245u8,
                90u8,
                77u8,
                245u8,
                35u8,
                179u8,
                239u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    from: topics.1,
                    to: topics.2,
                    value: data.0,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.from.clone(), self.to.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.from,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Transfer {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Transfer> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Transfer) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UnlockTimestampUpdated(uint256,uint256,address)` and selector `0xdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec`.
```solidity
event UnlockTimestampUpdated(uint256 oldTimestamp, uint256 newTimestamp, address indexed updatedBy);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UnlockTimestampUpdated {
        #[allow(missing_docs)]
        pub oldTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub updatedBy: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for UnlockTimestampUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "UnlockTimestampUpdated(uint256,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                221u8,
                104u8,
                150u8,
                220u8,
                241u8,
                212u8,
                179u8,
                17u8,
                204u8,
                168u8,
                125u8,
                209u8,
                155u8,
                187u8,
                162u8,
                234u8,
                156u8,
                226u8,
                248u8,
                103u8,
                193u8,
                86u8,
                136u8,
                120u8,
                160u8,
                67u8,
                138u8,
                102u8,
                161u8,
                175u8,
                238u8,
                236u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldTimestamp: data.0,
                    newTimestamp: data.1,
                    updatedBy: topics.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newTimestamp),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.updatedBy.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.updatedBy,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UnlockTimestampUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UnlockTimestampUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UnlockTimestampUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address defaultAdmin, address syndTreasuryAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub defaultAdmin: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub syndTreasuryAddress: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.defaultAdmin, value.syndTreasuryAddress)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        defaultAdmin: tuple.0,
                        syndTreasuryAddress: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.defaultAdmin,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.syndTreasuryAddress,
                    ),
                )
            }
        }
    };
    /**Function with signature `AIRDROP_MANAGER_ROLE()` and selector `0x8a542521`.
```solidity
function AIRDROP_MANAGER_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AIRDROP_MANAGER_ROLECall {}
    ///Container type for the return parameters of the [`AIRDROP_MANAGER_ROLE()`](AIRDROP_MANAGER_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AIRDROP_MANAGER_ROLEReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<AIRDROP_MANAGER_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: AIRDROP_MANAGER_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for AIRDROP_MANAGER_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<AIRDROP_MANAGER_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: AIRDROP_MANAGER_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for AIRDROP_MANAGER_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for AIRDROP_MANAGER_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = AIRDROP_MANAGER_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AIRDROP_MANAGER_ROLE()";
            const SELECTOR: [u8; 4] = [138u8, 84u8, 37u8, 33u8];
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
    /**Function with signature `BRIDGE_MANAGER_ROLE()` and selector `0xf75e8512`.
```solidity
function BRIDGE_MANAGER_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BRIDGE_MANAGER_ROLECall {}
    ///Container type for the return parameters of the [`BRIDGE_MANAGER_ROLE()`](BRIDGE_MANAGER_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BRIDGE_MANAGER_ROLEReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<BRIDGE_MANAGER_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: BRIDGE_MANAGER_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for BRIDGE_MANAGER_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<BRIDGE_MANAGER_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: BRIDGE_MANAGER_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for BRIDGE_MANAGER_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for BRIDGE_MANAGER_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = BRIDGE_MANAGER_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BRIDGE_MANAGER_ROLE()";
            const SELECTOR: [u8; 4] = [247u8, 94u8, 133u8, 18u8];
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
    /**Function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`.
```solidity
function CLOCK_MODE() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOCK_MODECall {}
    ///Container type for the return parameters of the [`CLOCK_MODE()`](CLOCK_MODECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOCK_MODEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<CLOCK_MODECall> for UnderlyingRustTuple<'_> {
                fn from(value: CLOCK_MODECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOCK_MODECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<CLOCK_MODEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: CLOCK_MODEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOCK_MODEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CLOCK_MODECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = CLOCK_MODEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CLOCK_MODE()";
            const SELECTOR: [u8; 4] = [75u8, 245u8, 215u8, 233u8];
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
    /**Function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`.
```solidity
function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLECall {}
    ///Container type for the return parameters of the [`DEFAULT_ADMIN_ROLE()`](DEFAULT_ADMIN_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLEReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<DEFAULT_ADMIN_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<DEFAULT_ADMIN_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEFAULT_ADMIN_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DEFAULT_ADMIN_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEFAULT_ADMIN_ROLE()";
            const SELECTOR: [u8; 4] = [162u8, 23u8, 253u8, 223u8];
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
    /**Function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`.
```solidity
function DOMAIN_SEPARATOR() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_SEPARATORCall {}
    ///Container type for the return parameters of the [`DOMAIN_SEPARATOR()`](DOMAIN_SEPARATORCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_SEPARATORReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<DOMAIN_SEPARATORCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_SEPARATORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DOMAIN_SEPARATORCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<DOMAIN_SEPARATORReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_SEPARATORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DOMAIN_SEPARATORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DOMAIN_SEPARATORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DOMAIN_SEPARATORReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DOMAIN_SEPARATOR()";
            const SELECTOR: [u8; 4] = [54u8, 68u8, 229u8, 21u8];
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
    /**Function with signature `EMISSION_BUDGET_MANAGER_ROLE()` and selector `0xb7cdc61c`.
```solidity
function EMISSION_BUDGET_MANAGER_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EMISSION_BUDGET_MANAGER_ROLECall {}
    ///Container type for the return parameters of the [`EMISSION_BUDGET_MANAGER_ROLE()`](EMISSION_BUDGET_MANAGER_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EMISSION_BUDGET_MANAGER_ROLEReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<EMISSION_BUDGET_MANAGER_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: EMISSION_BUDGET_MANAGER_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EMISSION_BUDGET_MANAGER_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<EMISSION_BUDGET_MANAGER_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: EMISSION_BUDGET_MANAGER_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EMISSION_BUDGET_MANAGER_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EMISSION_BUDGET_MANAGER_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = EMISSION_BUDGET_MANAGER_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EMISSION_BUDGET_MANAGER_ROLE()";
            const SELECTOR: [u8; 4] = [183u8, 205u8, 198u8, 28u8];
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
    /**Function with signature `EMISSION_MINTER_ROLE()` and selector `0x8d3343d6`.
```solidity
function EMISSION_MINTER_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EMISSION_MINTER_ROLECall {}
    ///Container type for the return parameters of the [`EMISSION_MINTER_ROLE()`](EMISSION_MINTER_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EMISSION_MINTER_ROLEReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<EMISSION_MINTER_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: EMISSION_MINTER_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EMISSION_MINTER_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<EMISSION_MINTER_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: EMISSION_MINTER_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EMISSION_MINTER_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EMISSION_MINTER_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = EMISSION_MINTER_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EMISSION_MINTER_ROLE()";
            const SELECTOR: [u8; 4] = [141u8, 51u8, 67u8, 214u8];
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
    /**Function with signature `INITIAL_MINT_SUPPLY()` and selector `0x9b7ef64b`.
```solidity
function INITIAL_MINT_SUPPLY() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct INITIAL_MINT_SUPPLYCall {}
    ///Container type for the return parameters of the [`INITIAL_MINT_SUPPLY()`](INITIAL_MINT_SUPPLYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct INITIAL_MINT_SUPPLYReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<INITIAL_MINT_SUPPLYCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: INITIAL_MINT_SUPPLYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for INITIAL_MINT_SUPPLYCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<INITIAL_MINT_SUPPLYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: INITIAL_MINT_SUPPLYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for INITIAL_MINT_SUPPLYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for INITIAL_MINT_SUPPLYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = INITIAL_MINT_SUPPLYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "INITIAL_MINT_SUPPLY()";
            const SELECTOR: [u8; 4] = [155u8, 126u8, 246u8, 75u8];
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
    /**Function with signature `MAX_LOCK_DURATION()` and selector `0x4f1bfc9e`.
```solidity
function MAX_LOCK_DURATION() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_LOCK_DURATIONCall {}
    ///Container type for the return parameters of the [`MAX_LOCK_DURATION()`](MAX_LOCK_DURATIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_LOCK_DURATIONReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<MAX_LOCK_DURATIONCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_LOCK_DURATIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_LOCK_DURATIONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<MAX_LOCK_DURATIONReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_LOCK_DURATIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_LOCK_DURATIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_LOCK_DURATIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_LOCK_DURATIONReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_LOCK_DURATION()";
            const SELECTOR: [u8; 4] = [79u8, 27u8, 252u8, 158u8];
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
    /**Function with signature `TOTAL_SUPPLY()` and selector `0x902d55a5`.
```solidity
function TOTAL_SUPPLY() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TOTAL_SUPPLYCall {}
    ///Container type for the return parameters of the [`TOTAL_SUPPLY()`](TOTAL_SUPPLYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TOTAL_SUPPLYReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<TOTAL_SUPPLYCall> for UnderlyingRustTuple<'_> {
                fn from(value: TOTAL_SUPPLYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TOTAL_SUPPLYCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<TOTAL_SUPPLYReturn> for UnderlyingRustTuple<'_> {
                fn from(value: TOTAL_SUPPLYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TOTAL_SUPPLYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TOTAL_SUPPLYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = TOTAL_SUPPLYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TOTAL_SUPPLY()";
            const SELECTOR: [u8; 4] = [144u8, 45u8, 85u8, 165u8];
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
    /**Function with signature `allocateEmissionBudget(address,uint256)` and selector `0x5a4239e9`.
```solidity
function allocateEmissionBudget(address bridge, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocateEmissionBudgetCall {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`allocateEmissionBudget(address,uint256)`](allocateEmissionBudgetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocateEmissionBudgetReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<allocateEmissionBudgetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocateEmissionBudgetCall) -> Self {
                    (value.bridge, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocateEmissionBudgetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        bridge: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<allocateEmissionBudgetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocateEmissionBudgetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocateEmissionBudgetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocateEmissionBudgetCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allocateEmissionBudgetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allocateEmissionBudget(address,uint256)";
            const SELECTOR: [u8; 4] = [90u8, 66u8, 57u8, 233u8];
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
                        &self.bridge,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `allowance(address,address)` and selector `0xdd62ed3e`.
```solidity
function allowance(address owner, address spender) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allowanceCall {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`allowance(address,address)`](allowanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allowanceReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<allowanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: allowanceCall) -> Self {
                    (value.owner, value.spender)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        spender: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<allowanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allowanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allowanceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allowanceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allowance(address,address)";
            const SELECTOR: [u8; 4] = [221u8, 98u8, 237u8, 62u8];
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
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.spender,
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
    /**Function with signature `approve(address,uint256)` and selector `0x095ea7b3`.
```solidity
function approve(address spender, uint256 value) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveCall {
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`approve(address,uint256)`](approveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveReturn {
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<approveCall> for UnderlyingRustTuple<'_> {
                fn from(value: approveCall) -> Self {
                    (value.spender, value.value)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        spender: tuple.0,
                        value: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<approveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: approveReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for approveCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = approveReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "approve(address,uint256)";
            const SELECTOR: [u8; 4] = [9u8, 94u8, 167u8, 179u8];
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
                        &self.spender,
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
    /**Function with signature `balanceOf(address)` and selector `0x70a08231`.
```solidity
function balanceOf(address account) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`balanceOf(address)`](balanceOfCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<balanceOfCall> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<balanceOfReturn> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for balanceOfCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = balanceOfReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "balanceOf(address)";
            const SELECTOR: [u8; 4] = [112u8, 160u8, 130u8, 49u8];
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
                        &self.account,
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
    /**Function with signature `bridgeConfigs(address)` and selector `0x427ac0ca`.
```solidity
function bridgeConfigs(address) external view returns (uint256 dailyMintLimit, uint256 dailyBurnLimit, bool isActive);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bridgeConfigsCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`bridgeConfigs(address)`](bridgeConfigsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bridgeConfigsReturn {
        #[allow(missing_docs)]
        pub dailyMintLimit: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub dailyBurnLimit: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub isActive: bool,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<bridgeConfigsCall> for UnderlyingRustTuple<'_> {
                fn from(value: bridgeConfigsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bridgeConfigsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<bridgeConfigsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bridgeConfigsReturn) -> Self {
                    (value.dailyMintLimit, value.dailyBurnLimit, value.isActive)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bridgeConfigsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dailyMintLimit: tuple.0,
                        dailyBurnLimit: tuple.1,
                        isActive: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bridgeConfigsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bridgeConfigsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bridgeConfigs(address)";
            const SELECTOR: [u8; 4] = [66u8, 122u8, 192u8, 202u8];
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
    /**Function with signature `bridgeEmissionBudgets(address)` and selector `0x2869366b`.
```solidity
function bridgeEmissionBudgets(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bridgeEmissionBudgetsCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`bridgeEmissionBudgets(address)`](bridgeEmissionBudgetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bridgeEmissionBudgetsReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<bridgeEmissionBudgetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: bridgeEmissionBudgetsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bridgeEmissionBudgetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<bridgeEmissionBudgetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: bridgeEmissionBudgetsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bridgeEmissionBudgetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bridgeEmissionBudgetsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bridgeEmissionBudgetsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bridgeEmissionBudgets(address)";
            const SELECTOR: [u8; 4] = [40u8, 105u8, 54u8, 107u8];
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
    /**Function with signature `burn(uint256)` and selector `0x42966c68`.
```solidity
function burn(uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnCall {
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`burn(uint256)`](burnCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<burnCall> for UnderlyingRustTuple<'_> {
                fn from(value: burnCall) -> Self {
                    (value.amount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amount: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<burnReturn> for UnderlyingRustTuple<'_> {
                fn from(value: burnReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for burnCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = burnReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "burn(uint256)";
            const SELECTOR: [u8; 4] = [66u8, 150u8, 108u8, 104u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `burnFrom(address,uint256)` and selector `0x79cc6790`.
```solidity
function burnFrom(address from, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnFromCall {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`burnFrom(address,uint256)`](burnFromCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnFromReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<burnFromCall> for UnderlyingRustTuple<'_> {
                fn from(value: burnFromCall) -> Self {
                    (value.from, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnFromCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<burnFromReturn> for UnderlyingRustTuple<'_> {
                fn from(value: burnFromReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnFromReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for burnFromCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = burnFromReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "burnFrom(address,uint256)";
            const SELECTOR: [u8; 4] = [121u8, 204u8, 103u8, 144u8];
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
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `checkpoints(address,uint32)` and selector `0xf1127ed8`.
```solidity
function checkpoints(address account, uint32 pos) external view returns (Checkpoints.Checkpoint208 memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointsCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub pos: u32,
    }
    ///Container type for the return parameters of the [`checkpoints(address,uint32)`](checkpointsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointsReturn {
        #[allow(missing_docs)]
        pub _0: <Checkpoints::Checkpoint208 as alloy::sol_types::SolType>::RustType,
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkpointsCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkpointsCall) -> Self {
                    (value.account, value.pos)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkpointsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        pos: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Checkpoints::Checkpoint208,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Checkpoints::Checkpoint208 as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<checkpointsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkpointsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkpointsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkpointsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkpointsReturn;
            type ReturnTuple<'a> = (Checkpoints::Checkpoint208,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkpoints(address,uint32)";
            const SELECTOR: [u8; 4] = [241u8, 18u8, 126u8, 216u8];
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
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.pos),
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
    /**Function with signature `clock()` and selector `0x91ddadf4`.
```solidity
function clock() external view returns (uint48);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockCall {}
    ///Container type for the return parameters of the [`clock()`](clockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U48,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<clockCall> for UnderlyingRustTuple<'_> {
                fn from(value: clockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U48,
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
            impl ::core::convert::From<clockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: clockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = clockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clock()";
            const SELECTOR: [u8; 4] = [145u8, 221u8, 173u8, 244u8];
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
    /**Function with signature `crosschainBurn(address,uint256)` and selector `0x2b8c49e3`.
```solidity
function crosschainBurn(address from, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct crosschainBurnCall {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`crosschainBurn(address,uint256)`](crosschainBurnCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct crosschainBurnReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<crosschainBurnCall> for UnderlyingRustTuple<'_> {
                fn from(value: crosschainBurnCall) -> Self {
                    (value.from, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for crosschainBurnCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<crosschainBurnReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: crosschainBurnReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for crosschainBurnReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for crosschainBurnCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = crosschainBurnReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "crosschainBurn(address,uint256)";
            const SELECTOR: [u8; 4] = [43u8, 140u8, 73u8, 227u8];
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
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `crosschainMint(address,uint256)` and selector `0x18bf5077`.
```solidity
function crosschainMint(address to, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct crosschainMintCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`crosschainMint(address,uint256)`](crosschainMintCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct crosschainMintReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<crosschainMintCall> for UnderlyingRustTuple<'_> {
                fn from(value: crosschainMintCall) -> Self {
                    (value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for crosschainMintCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<crosschainMintReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: crosschainMintReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for crosschainMintReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for crosschainMintCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = crosschainMintReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "crosschainMint(address,uint256)";
            const SELECTOR: [u8; 4] = [24u8, 191u8, 80u8, 119u8];
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
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `decimals()` and selector `0x313ce567`.
```solidity
function decimals() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decimalsCall {}
    ///Container type for the return parameters of the [`decimals()`](decimalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decimalsReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<decimalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: decimalsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decimalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<decimalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: decimalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decimalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for decimalsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = decimalsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "decimals()";
            const SELECTOR: [u8; 4] = [49u8, 60u8, 229u8, 103u8];
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
    /**Function with signature `delegate(address)` and selector `0x5c19a95c`.
```solidity
function delegate(address delegatee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateCall {
        #[allow(missing_docs)]
        pub delegatee: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegate(address)`](delegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<delegateCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateCall) -> Self {
                    (value.delegatee,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { delegatee: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<delegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegate(address)";
            const SELECTOR: [u8; 4] = [92u8, 25u8, 169u8, 92u8];
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
                        &self.delegatee,
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
    /**Function with signature `delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xc3cda520`.
```solidity
function delegateBySig(address delegatee, uint256 nonce, uint256 expiry, uint8 v, bytes32 r, bytes32 s) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateBySigCall {
        #[allow(missing_docs)]
        pub delegatee: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub v: u8,
        #[allow(missing_docs)]
        pub r: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)`](delegateBySigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateBySigReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<delegateBySigCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateBySigCall) -> Self {
                    (
                        value.delegatee,
                        value.nonce,
                        value.expiry,
                        value.v,
                        value.r,
                        value.s,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateBySigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        delegatee: tuple.0,
                        nonce: tuple.1,
                        expiry: tuple.2,
                        v: tuple.3,
                        r: tuple.4,
                        s: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<delegateBySigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateBySigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateBySigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateBySigCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateBySigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [195u8, 205u8, 165u8, 32u8];
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
                        &self.delegatee,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.v),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.r),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
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
    /**Function with signature `delegates(address)` and selector `0x587cde1e`.
```solidity
function delegates(address account) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatesCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegates(address)`](delegatesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatesReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<delegatesCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegatesCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<delegatesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegatesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegatesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegatesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegates(address)";
            const SELECTOR: [u8; 4] = [88u8, 124u8, 222u8, 30u8];
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
                        &self.account,
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
    /**Function with signature `eip712Domain()` and selector `0x84b0196e`.
```solidity
function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainCall {}
    ///Container type for the return parameters of the [`eip712Domain()`](eip712DomainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainReturn {
        #[allow(missing_docs)]
        pub fields: alloy::sol_types::private::FixedBytes<1>,
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub verifyingContract: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub extensions: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<eip712DomainCall> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<1>,
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<eip712DomainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainReturn) -> Self {
                    (
                        value.fields,
                        value.name,
                        value.version,
                        value.chainId,
                        value.verifyingContract,
                        value.salt,
                        value.extensions,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        fields: tuple.0,
                        name: tuple.1,
                        version: tuple.2,
                        chainId: tuple.3,
                        verifyingContract: tuple.4,
                        salt: tuple.5,
                        extensions: tuple.6,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eip712DomainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eip712DomainReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eip712Domain()";
            const SELECTOR: [u8; 4] = [132u8, 176u8, 25u8, 110u8];
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
    /**Function with signature `getAllBridges()` and selector `0x72cbdcc8`.
```solidity
function getAllBridges() external view returns (address[] memory allBridges);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllBridgesCall {}
    ///Container type for the return parameters of the [`getAllBridges()`](getAllBridgesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllBridgesReturn {
        #[allow(missing_docs)]
        pub allBridges: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getAllBridgesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAllBridgesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllBridgesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getAllBridgesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getAllBridgesReturn) -> Self {
                    (value.allBridges,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllBridgesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { allBridges: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllBridgesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllBridgesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllBridges()";
            const SELECTOR: [u8; 4] = [114u8, 203u8, 220u8, 200u8];
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
    /**Function with signature `getAvailableBurnLimit(address)` and selector `0x30d3e8eb`.
```solidity
function getAvailableBurnLimit(address bridge) external view returns (uint256 available);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAvailableBurnLimitCall {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAvailableBurnLimit(address)`](getAvailableBurnLimitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAvailableBurnLimitReturn {
        #[allow(missing_docs)]
        pub available: alloy::sol_types::private::primitives::aliases::U256,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getAvailableBurnLimitCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAvailableBurnLimitCall) -> Self {
                    (value.bridge,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAvailableBurnLimitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { bridge: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getAvailableBurnLimitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAvailableBurnLimitReturn) -> Self {
                    (value.available,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAvailableBurnLimitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { available: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAvailableBurnLimitCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAvailableBurnLimitReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAvailableBurnLimit(address)";
            const SELECTOR: [u8; 4] = [48u8, 211u8, 232u8, 235u8];
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
                        &self.bridge,
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
    /**Function with signature `getAvailableMintLimit(address)` and selector `0x94aa22f2`.
```solidity
function getAvailableMintLimit(address bridge) external view returns (uint256 available);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAvailableMintLimitCall {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAvailableMintLimit(address)`](getAvailableMintLimitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAvailableMintLimitReturn {
        #[allow(missing_docs)]
        pub available: alloy::sol_types::private::primitives::aliases::U256,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getAvailableMintLimitCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAvailableMintLimitCall) -> Self {
                    (value.bridge,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAvailableMintLimitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { bridge: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getAvailableMintLimitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAvailableMintLimitReturn) -> Self {
                    (value.available,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAvailableMintLimitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { available: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAvailableMintLimitCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAvailableMintLimitReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAvailableMintLimit(address)";
            const SELECTOR: [u8; 4] = [148u8, 170u8, 34u8, 242u8];
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
                        &self.bridge,
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
    /**Function with signature `getBridgeAtIndex(uint256)` and selector `0x5a5db1bb`.
```solidity
function getBridgeAtIndex(uint256 index) external view returns (address bridge);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBridgeAtIndexCall {
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getBridgeAtIndex(uint256)`](getBridgeAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBridgeAtIndexReturn {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getBridgeAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBridgeAtIndexCall) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBridgeAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getBridgeAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBridgeAtIndexReturn) -> Self {
                    (value.bridge,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBridgeAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { bridge: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBridgeAtIndexCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBridgeAtIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBridgeAtIndex(uint256)";
            const SELECTOR: [u8; 4] = [90u8, 93u8, 177u8, 187u8];
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
    /**Function with signature `getBridgeConfig(address)` and selector `0xc4fc45a8`.
```solidity
function getBridgeConfig(address bridge) external view returns (IBridgeRateLimiter.BridgeConfig memory config);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBridgeConfigCall {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getBridgeConfig(address)`](getBridgeConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBridgeConfigReturn {
        #[allow(missing_docs)]
        pub config: <IBridgeRateLimiter::BridgeConfig as alloy::sol_types::SolType>::RustType,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getBridgeConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBridgeConfigCall) -> Self {
                    (value.bridge,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBridgeConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { bridge: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (IBridgeRateLimiter::BridgeConfig,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IBridgeRateLimiter::BridgeConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getBridgeConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBridgeConfigReturn) -> Self {
                    (value.config,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBridgeConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { config: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBridgeConfigCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBridgeConfigReturn;
            type ReturnTuple<'a> = (IBridgeRateLimiter::BridgeConfig,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBridgeConfig(address)";
            const SELECTOR: [u8; 4] = [196u8, 252u8, 69u8, 168u8];
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
                        &self.bridge,
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
    /**Function with signature `getBridgeCount()` and selector `0x65145534`.
```solidity
function getBridgeCount() external view returns (uint256 count);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBridgeCountCall {}
    ///Container type for the return parameters of the [`getBridgeCount()`](getBridgeCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBridgeCountReturn {
        #[allow(missing_docs)]
        pub count: alloy::sol_types::private::primitives::aliases::U256,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getBridgeCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBridgeCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBridgeCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getBridgeCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBridgeCountReturn) -> Self {
                    (value.count,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBridgeCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { count: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBridgeCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBridgeCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBridgeCount()";
            const SELECTOR: [u8; 4] = [101u8, 20u8, 85u8, 52u8];
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
    /**Function with signature `getCurrentTotalSupply()` and selector `0xc02ae754`.
```solidity
function getCurrentTotalSupply() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalSupplyCall {}
    ///Container type for the return parameters of the [`getCurrentTotalSupply()`](getCurrentTotalSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalSupplyReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getCurrentTotalSupplyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalSupplyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentTotalSupplyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getCurrentTotalSupplyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalSupplyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentTotalSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentTotalSupplyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentTotalSupplyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentTotalSupply()";
            const SELECTOR: [u8; 4] = [192u8, 42u8, 231u8, 84u8];
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
    /**Function with signature `getEmissionBudget(address)` and selector `0x050732fb`.
```solidity
function getEmissionBudget(address bridge) external view returns (uint256 budget);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEmissionBudgetCall {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getEmissionBudget(address)`](getEmissionBudgetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEmissionBudgetReturn {
        #[allow(missing_docs)]
        pub budget: alloy::sol_types::private::primitives::aliases::U256,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getEmissionBudgetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getEmissionBudgetCall) -> Self {
                    (value.bridge,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getEmissionBudgetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { bridge: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getEmissionBudgetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getEmissionBudgetReturn) -> Self {
                    (value.budget,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getEmissionBudgetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { budget: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getEmissionBudgetCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getEmissionBudgetReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getEmissionBudget(address)";
            const SELECTOR: [u8; 4] = [5u8, 7u8, 50u8, 251u8];
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
                        &self.bridge,
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
    /**Function with signature `getPastTotalSupply(uint256)` and selector `0x8e539e8c`.
```solidity
function getPastTotalSupply(uint256 timepoint) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastTotalSupplyCall {
        #[allow(missing_docs)]
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getPastTotalSupply(uint256)`](getPastTotalSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastTotalSupplyReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getPastTotalSupplyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPastTotalSupplyCall) -> Self {
                    (value.timepoint,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPastTotalSupplyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { timepoint: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getPastTotalSupplyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPastTotalSupplyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPastTotalSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPastTotalSupplyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPastTotalSupplyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPastTotalSupply(uint256)";
            const SELECTOR: [u8; 4] = [142u8, 83u8, 158u8, 140u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
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
    /**Function with signature `getPastVotes(address,uint256)` and selector `0x3a46b1a8`.
```solidity
function getPastVotes(address account, uint256 timepoint) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotesCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getPastVotes(address,uint256)`](getPastVotesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotesReturn {
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getPastVotesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotesCall) -> Self {
                    (value.account, value.timepoint)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPastVotesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        timepoint: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getPastVotesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPastVotesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPastVotesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPastVotesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPastVotes(address,uint256)";
            const SELECTOR: [u8; 4] = [58u8, 70u8, 177u8, 168u8];
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
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
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
    /**Function with signature `getPastVotingPower(address,uint256)` and selector `0xb0ca253e`.
```solidity
function getPastVotingPower(address account, uint256 blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotingPowerCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getPastVotingPower(address,uint256)`](getPastVotingPowerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotingPowerReturn {
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getPastVotingPowerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotingPowerCall) -> Self {
                    (value.account, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPastVotingPowerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        blockNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getPastVotingPowerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotingPowerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPastVotingPowerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPastVotingPowerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPastVotingPowerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPastVotingPower(address,uint256)";
            const SELECTOR: [u8; 4] = [176u8, 202u8, 37u8, 62u8];
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
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
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
    /**Function with signature `getRemainingLockTime()` and selector `0x7a8cd156`.
```solidity
function getRemainingLockTime() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRemainingLockTimeCall {}
    ///Container type for the return parameters of the [`getRemainingLockTime()`](getRemainingLockTimeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRemainingLockTimeReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getRemainingLockTimeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRemainingLockTimeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRemainingLockTimeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getRemainingLockTimeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRemainingLockTimeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRemainingLockTimeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRemainingLockTimeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRemainingLockTimeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRemainingLockTime()";
            const SELECTOR: [u8; 4] = [122u8, 140u8, 209u8, 86u8];
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
    /**Function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`.
```solidity
function getRoleAdmin(bytes32 role) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getRoleAdmin(bytes32)`](getRoleAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getRoleAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminCall) -> Self {
                    (value.role,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getRoleAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRoleAdminReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleAdmin(bytes32)";
            const SELECTOR: [u8; 4] = [36u8, 138u8, 156u8, 163u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
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
    /**Function with signature `getVotes(address)` and selector `0x9ab24eb0`.
```solidity
function getVotes(address account) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getVotes(address)`](getVotesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getVotesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getVotesCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getVotesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getVotesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getVotesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getVotesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getVotes(address)";
            const SELECTOR: [u8; 4] = [154u8, 178u8, 78u8, 176u8];
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
                        &self.account,
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
    /**Function with signature `getVotingPower(address)` and selector `0xbb4d4436`.
```solidity
function getVotingPower(address account) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotingPowerCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getVotingPower(address)`](getVotingPowerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotingPowerReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getVotingPowerCall> for UnderlyingRustTuple<'_> {
                fn from(value: getVotingPowerCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotingPowerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getVotingPowerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getVotingPowerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getVotingPowerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getVotingPowerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getVotingPowerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getVotingPower(address)";
            const SELECTOR: [u8; 4] = [187u8, 77u8, 68u8, 54u8];
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
                        &self.account,
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
    /**Function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`.
```solidity
function grantRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`grantRole(bytes32,address)`](grantRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<grantRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<grantRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for grantRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = grantRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "grantRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [47u8, 47u8, 241u8, 93u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
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
    /**Function with signature `hasRole(bytes32,address)` and selector `0x91d14854`.
```solidity
function hasRole(bytes32 role, address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`hasRole(bytes32,address)`](hasRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleReturn {
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<hasRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<hasRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = hasRoleReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [145u8, 209u8, 72u8, 84u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
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
    /**Function with signature `hourlyBurnUsage(address,uint256)` and selector `0x5d4c6285`.
```solidity
function hourlyBurnUsage(address, uint256) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hourlyBurnUsageCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`hourlyBurnUsage(address,uint256)`](hourlyBurnUsageCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hourlyBurnUsageReturn {
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<hourlyBurnUsageCall> for UnderlyingRustTuple<'_> {
                fn from(value: hourlyBurnUsageCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hourlyBurnUsageCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<hourlyBurnUsageReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: hourlyBurnUsageReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for hourlyBurnUsageReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hourlyBurnUsageCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = hourlyBurnUsageReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hourlyBurnUsage(address,uint256)";
            const SELECTOR: [u8; 4] = [93u8, 76u8, 98u8, 133u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
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
    /**Function with signature `hourlyMintUsage(address,uint256)` and selector `0x01042d7a`.
```solidity
function hourlyMintUsage(address, uint256) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hourlyMintUsageCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`hourlyMintUsage(address,uint256)`](hourlyMintUsageCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hourlyMintUsageReturn {
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<hourlyMintUsageCall> for UnderlyingRustTuple<'_> {
                fn from(value: hourlyMintUsageCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hourlyMintUsageCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<hourlyMintUsageReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: hourlyMintUsageReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for hourlyMintUsageReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hourlyMintUsageCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = hourlyMintUsageReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hourlyMintUsage(address,uint256)";
            const SELECTOR: [u8; 4] = [1u8, 4u8, 45u8, 122u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
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
    /**Function with signature `isBridgeAuthorized(address)` and selector `0x78fb7fd2`.
```solidity
function isBridgeAuthorized(address bridge) external view returns (bool authorized);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isBridgeAuthorizedCall {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isBridgeAuthorized(address)`](isBridgeAuthorizedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isBridgeAuthorizedReturn {
        #[allow(missing_docs)]
        pub authorized: bool,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<isBridgeAuthorizedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isBridgeAuthorizedCall) -> Self {
                    (value.bridge,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isBridgeAuthorizedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { bridge: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<isBridgeAuthorizedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isBridgeAuthorizedReturn) -> Self {
                    (value.authorized,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isBridgeAuthorizedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { authorized: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isBridgeAuthorizedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isBridgeAuthorizedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isBridgeAuthorized(address)";
            const SELECTOR: [u8; 4] = [120u8, 251u8, 127u8, 210u8];
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
                        &self.bridge,
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
    /**Function with signature `maxLockTimestamp()` and selector `0x8426adf2`.
```solidity
function maxLockTimestamp() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxLockTimestampCall {}
    ///Container type for the return parameters of the [`maxLockTimestamp()`](maxLockTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxLockTimestampReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<maxLockTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: maxLockTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maxLockTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<maxLockTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: maxLockTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maxLockTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for maxLockTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = maxLockTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "maxLockTimestamp()";
            const SELECTOR: [u8; 4] = [132u8, 38u8, 173u8, 242u8];
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
    /**Function with signature `mint(address,uint256)` and selector `0x40c10f19`.
```solidity
function mint(address to, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mintCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`mint(address,uint256)`](mintCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mintReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<mintCall> for UnderlyingRustTuple<'_> {
                fn from(value: mintCall) -> Self {
                    (value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<mintReturn> for UnderlyingRustTuple<'_> {
                fn from(value: mintReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mintCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mintReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mint(address,uint256)";
            const SELECTOR: [u8; 4] = [64u8, 193u8, 15u8, 25u8];
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
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `name()` and selector `0x06fdde03`.
```solidity
function name() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nameCall {}
    ///Container type for the return parameters of the [`name()`](nameCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nameReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<nameCall> for UnderlyingRustTuple<'_> {
                fn from(value: nameCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<nameReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nameReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nameCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = nameReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "name()";
            const SELECTOR: [u8; 4] = [6u8, 253u8, 222u8, 3u8];
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
    /**Function with signature `nonces(address)` and selector `0x7ecebe00`.
```solidity
function nonces(address owner) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noncesCall {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`nonces(address)`](noncesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noncesReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<noncesCall> for UnderlyingRustTuple<'_> {
                fn from(value: noncesCall) -> Self {
                    (value.owner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noncesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { owner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<noncesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: noncesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noncesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for noncesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = noncesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nonces(address)";
            const SELECTOR: [u8; 4] = [126u8, 206u8, 190u8, 0u8];
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
                        &self.owner,
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
    /**Function with signature `numCheckpoints(address)` and selector `0x6fcfff45`.
```solidity
function numCheckpoints(address account) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numCheckpointsCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`numCheckpoints(address)`](numCheckpointsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numCheckpointsReturn {
        #[allow(missing_docs)]
        pub _0: u32,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<numCheckpointsCall> for UnderlyingRustTuple<'_> {
                fn from(value: numCheckpointsCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for numCheckpointsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<numCheckpointsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: numCheckpointsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for numCheckpointsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for numCheckpointsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = numCheckpointsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "numCheckpoints(address)";
            const SELECTOR: [u8; 4] = [111u8, 207u8, 255u8, 69u8];
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
                        &self.account,
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
    /**Function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`.
```solidity
function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permitCall {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub v: u8,
        #[allow(missing_docs)]
        pub r: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`permit(address,address,uint256,uint256,uint8,bytes32,bytes32)`](permitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permitReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<permitCall> for UnderlyingRustTuple<'_> {
                fn from(value: permitCall) -> Self {
                    (
                        value.owner,
                        value.spender,
                        value.value,
                        value.deadline,
                        value.v,
                        value.r,
                        value.s,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        spender: tuple.1,
                        value: tuple.2,
                        deadline: tuple.3,
                        v: tuple.4,
                        r: tuple.5,
                        s: tuple.6,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<permitReturn> for UnderlyingRustTuple<'_> {
                fn from(value: permitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for permitCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = permitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [213u8, 5u8, 172u8, 207u8];
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
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.v),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.r),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
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
    /**Function with signature `removeBridge(address)` and selector `0x04df017d`.
```solidity
function removeBridge(address bridge) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeBridgeCall {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`removeBridge(address)`](removeBridgeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeBridgeReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<removeBridgeCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeBridgeCall) -> Self {
                    (value.bridge,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeBridgeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { bridge: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<removeBridgeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeBridgeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeBridgeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeBridgeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeBridgeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeBridge(address)";
            const SELECTOR: [u8; 4] = [4u8, 223u8, 1u8, 125u8];
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
                        &self.bridge,
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
    /**Function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`.
```solidity
function renounceRole(bytes32 role, address callerConfirmation) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub callerConfirmation: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`renounceRole(bytes32,address)`](renounceRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<renounceRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleCall) -> Self {
                    (value.role, value.callerConfirmation)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        callerConfirmation: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<renounceRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [54u8, 86u8, 138u8, 190u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.callerConfirmation,
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
    /**Function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`.
```solidity
function revokeRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`revokeRole(bytes32,address)`](revokeRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<revokeRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<revokeRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revokeRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revokeRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revokeRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [213u8, 71u8, 116u8, 31u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
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
    /**Function with signature `setBridgeActive(address,bool)` and selector `0xc9ab0006`.
```solidity
function setBridgeActive(address bridge, bool isActive) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBridgeActiveCall {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub isActive: bool,
    }
    ///Container type for the return parameters of the [`setBridgeActive(address,bool)`](setBridgeActiveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBridgeActiveReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<setBridgeActiveCall> for UnderlyingRustTuple<'_> {
                fn from(value: setBridgeActiveCall) -> Self {
                    (value.bridge, value.isActive)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBridgeActiveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        bridge: tuple.0,
                        isActive: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<setBridgeActiveReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setBridgeActiveReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setBridgeActiveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setBridgeActiveCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setBridgeActiveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setBridgeActive(address,bool)";
            const SELECTOR: [u8; 4] = [201u8, 171u8, 0u8, 6u8];
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
                        &self.bridge,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isActive,
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
    /**Function with signature `setBridgeLimits(address,uint256,uint256)` and selector `0x63a0daac`.
```solidity
function setBridgeLimits(address bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBridgeLimitsCall {
        #[allow(missing_docs)]
        pub bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub dailyMintLimit: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub dailyBurnLimit: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setBridgeLimits(address,uint256,uint256)`](setBridgeLimitsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBridgeLimitsReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<setBridgeLimitsCall> for UnderlyingRustTuple<'_> {
                fn from(value: setBridgeLimitsCall) -> Self {
                    (value.bridge, value.dailyMintLimit, value.dailyBurnLimit)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBridgeLimitsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        bridge: tuple.0,
                        dailyMintLimit: tuple.1,
                        dailyBurnLimit: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<setBridgeLimitsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setBridgeLimitsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setBridgeLimitsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setBridgeLimitsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setBridgeLimitsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setBridgeLimits(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [99u8, 160u8, 218u8, 172u8];
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
                        &self.bridge,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.dailyMintLimit),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.dailyBurnLimit),
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
    /**Function with signature `setUnlockTimestamp(uint256)` and selector `0x844c9026`.
```solidity
function setUnlockTimestamp(uint256 newUnlockTimestamp) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUnlockTimestampCall {
        #[allow(missing_docs)]
        pub newUnlockTimestamp: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setUnlockTimestamp(uint256)`](setUnlockTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUnlockTimestampReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<setUnlockTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setUnlockTimestampCall) -> Self {
                    (value.newUnlockTimestamp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setUnlockTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newUnlockTimestamp: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<setUnlockTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setUnlockTimestampReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setUnlockTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setUnlockTimestampCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setUnlockTimestampReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setUnlockTimestamp(uint256)";
            const SELECTOR: [u8; 4] = [132u8, 76u8, 144u8, 38u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.newUnlockTimestamp),
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
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
```solidity
function supportsInterface(bytes4 interfaceId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceCall {
        #[allow(missing_docs)]
        pub interfaceId: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`supportsInterface(bytes4)`](supportsInterfaceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceReturn {
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { interfaceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<supportsInterfaceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = supportsInterfaceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsInterface(bytes4)";
            const SELECTOR: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
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
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.interfaceId),
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
    /**Function with signature `symbol()` and selector `0x95d89b41`.
```solidity
function symbol() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct symbolCall {}
    ///Container type for the return parameters of the [`symbol()`](symbolCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct symbolReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<symbolCall> for UnderlyingRustTuple<'_> {
                fn from(value: symbolCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<symbolReturn> for UnderlyingRustTuple<'_> {
                fn from(value: symbolReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for symbolCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = symbolReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "symbol()";
            const SELECTOR: [u8; 4] = [149u8, 216u8, 155u8, 65u8];
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
    /**Function with signature `totalSupply()` and selector `0x18160ddd`.
```solidity
function totalSupply() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalSupplyCall {}
    ///Container type for the return parameters of the [`totalSupply()`](totalSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalSupplyReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<totalSupplyCall> for UnderlyingRustTuple<'_> {
                fn from(value: totalSupplyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalSupplyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<totalSupplyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: totalSupplyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalSupplyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = totalSupplyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "totalSupply()";
            const SELECTOR: [u8; 4] = [24u8, 22u8, 13u8, 221u8];
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
    /**Function with signature `transfer(address,uint256)` and selector `0xa9059cbb`.
```solidity
function transfer(address to, uint256 value) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`transfer(address,uint256)`](transferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferReturn {
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<transferCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferCall) -> Self {
                    (value.to, value.value)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        value: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<transferReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transfer(address,uint256)";
            const SELECTOR: [u8; 4] = [169u8, 5u8, 156u8, 187u8];
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
                        &self.to,
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
    /**Function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`.
```solidity
function transferFrom(address from, address to, uint256 value) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferFromCall {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`transferFrom(address,address,uint256)`](transferFromCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferFromReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<transferFromCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromCall) -> Self {
                    (value.from, value.to, value.value)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        to: tuple.1,
                        value: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<transferFromReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferFromCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferFromReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferFrom(address,address,uint256)";
            const SELECTOR: [u8; 4] = [35u8, 184u8, 114u8, 221u8];
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
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
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
    /**Function with signature `transfersLocked()` and selector `0x83f1211b`.
```solidity
function transfersLocked() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transfersLockedCall {}
    ///Container type for the return parameters of the [`transfersLocked()`](transfersLockedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transfersLockedReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<transfersLockedCall> for UnderlyingRustTuple<'_> {
                fn from(value: transfersLockedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transfersLockedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<transfersLockedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transfersLockedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transfersLockedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transfersLockedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transfersLockedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transfersLocked()";
            const SELECTOR: [u8; 4] = [131u8, 241u8, 33u8, 27u8];
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
    /**Function with signature `unlockTimestamp()` and selector `0xaa082a9d`.
```solidity
function unlockTimestamp() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unlockTimestampCall {}
    ///Container type for the return parameters of the [`unlockTimestamp()`](unlockTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unlockTimestampReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<unlockTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: unlockTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unlockTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<unlockTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: unlockTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for unlockTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unlockTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unlockTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unlockTimestamp()";
            const SELECTOR: [u8; 4] = [170u8, 8u8, 42u8, 157u8];
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
    ///Container for all the [`SyndicateTokenCrosschain`](self) function calls.
    pub enum SyndicateTokenCrosschainCalls {
        #[allow(missing_docs)]
        AIRDROP_MANAGER_ROLE(AIRDROP_MANAGER_ROLECall),
        #[allow(missing_docs)]
        BRIDGE_MANAGER_ROLE(BRIDGE_MANAGER_ROLECall),
        #[allow(missing_docs)]
        CLOCK_MODE(CLOCK_MODECall),
        #[allow(missing_docs)]
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        #[allow(missing_docs)]
        DOMAIN_SEPARATOR(DOMAIN_SEPARATORCall),
        #[allow(missing_docs)]
        EMISSION_BUDGET_MANAGER_ROLE(EMISSION_BUDGET_MANAGER_ROLECall),
        #[allow(missing_docs)]
        EMISSION_MINTER_ROLE(EMISSION_MINTER_ROLECall),
        #[allow(missing_docs)]
        INITIAL_MINT_SUPPLY(INITIAL_MINT_SUPPLYCall),
        #[allow(missing_docs)]
        MAX_LOCK_DURATION(MAX_LOCK_DURATIONCall),
        #[allow(missing_docs)]
        TOTAL_SUPPLY(TOTAL_SUPPLYCall),
        #[allow(missing_docs)]
        allocateEmissionBudget(allocateEmissionBudgetCall),
        #[allow(missing_docs)]
        allowance(allowanceCall),
        #[allow(missing_docs)]
        approve(approveCall),
        #[allow(missing_docs)]
        balanceOf(balanceOfCall),
        #[allow(missing_docs)]
        bridgeConfigs(bridgeConfigsCall),
        #[allow(missing_docs)]
        bridgeEmissionBudgets(bridgeEmissionBudgetsCall),
        #[allow(missing_docs)]
        burn(burnCall),
        #[allow(missing_docs)]
        burnFrom(burnFromCall),
        #[allow(missing_docs)]
        checkpoints(checkpointsCall),
        #[allow(missing_docs)]
        clock(clockCall),
        #[allow(missing_docs)]
        crosschainBurn(crosschainBurnCall),
        #[allow(missing_docs)]
        crosschainMint(crosschainMintCall),
        #[allow(missing_docs)]
        decimals(decimalsCall),
        #[allow(missing_docs)]
        delegate(delegateCall),
        #[allow(missing_docs)]
        delegateBySig(delegateBySigCall),
        #[allow(missing_docs)]
        delegates(delegatesCall),
        #[allow(missing_docs)]
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        getAllBridges(getAllBridgesCall),
        #[allow(missing_docs)]
        getAvailableBurnLimit(getAvailableBurnLimitCall),
        #[allow(missing_docs)]
        getAvailableMintLimit(getAvailableMintLimitCall),
        #[allow(missing_docs)]
        getBridgeAtIndex(getBridgeAtIndexCall),
        #[allow(missing_docs)]
        getBridgeConfig(getBridgeConfigCall),
        #[allow(missing_docs)]
        getBridgeCount(getBridgeCountCall),
        #[allow(missing_docs)]
        getCurrentTotalSupply(getCurrentTotalSupplyCall),
        #[allow(missing_docs)]
        getEmissionBudget(getEmissionBudgetCall),
        #[allow(missing_docs)]
        getPastTotalSupply(getPastTotalSupplyCall),
        #[allow(missing_docs)]
        getPastVotes(getPastVotesCall),
        #[allow(missing_docs)]
        getPastVotingPower(getPastVotingPowerCall),
        #[allow(missing_docs)]
        getRemainingLockTime(getRemainingLockTimeCall),
        #[allow(missing_docs)]
        getRoleAdmin(getRoleAdminCall),
        #[allow(missing_docs)]
        getVotes(getVotesCall),
        #[allow(missing_docs)]
        getVotingPower(getVotingPowerCall),
        #[allow(missing_docs)]
        grantRole(grantRoleCall),
        #[allow(missing_docs)]
        hasRole(hasRoleCall),
        #[allow(missing_docs)]
        hourlyBurnUsage(hourlyBurnUsageCall),
        #[allow(missing_docs)]
        hourlyMintUsage(hourlyMintUsageCall),
        #[allow(missing_docs)]
        isBridgeAuthorized(isBridgeAuthorizedCall),
        #[allow(missing_docs)]
        maxLockTimestamp(maxLockTimestampCall),
        #[allow(missing_docs)]
        mint(mintCall),
        #[allow(missing_docs)]
        name(nameCall),
        #[allow(missing_docs)]
        nonces(noncesCall),
        #[allow(missing_docs)]
        numCheckpoints(numCheckpointsCall),
        #[allow(missing_docs)]
        permit(permitCall),
        #[allow(missing_docs)]
        removeBridge(removeBridgeCall),
        #[allow(missing_docs)]
        renounceRole(renounceRoleCall),
        #[allow(missing_docs)]
        revokeRole(revokeRoleCall),
        #[allow(missing_docs)]
        setBridgeActive(setBridgeActiveCall),
        #[allow(missing_docs)]
        setBridgeLimits(setBridgeLimitsCall),
        #[allow(missing_docs)]
        setUnlockTimestamp(setUnlockTimestampCall),
        #[allow(missing_docs)]
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        symbol(symbolCall),
        #[allow(missing_docs)]
        totalSupply(totalSupplyCall),
        #[allow(missing_docs)]
        transfer(transferCall),
        #[allow(missing_docs)]
        transferFrom(transferFromCall),
        #[allow(missing_docs)]
        transfersLocked(transfersLockedCall),
        #[allow(missing_docs)]
        unlockTimestamp(unlockTimestampCall),
    }
    #[automatically_derived]
    impl SyndicateTokenCrosschainCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 4u8, 45u8, 122u8],
            [1u8, 255u8, 201u8, 167u8],
            [4u8, 223u8, 1u8, 125u8],
            [5u8, 7u8, 50u8, 251u8],
            [6u8, 253u8, 222u8, 3u8],
            [9u8, 94u8, 167u8, 179u8],
            [24u8, 22u8, 13u8, 221u8],
            [24u8, 191u8, 80u8, 119u8],
            [35u8, 184u8, 114u8, 221u8],
            [36u8, 138u8, 156u8, 163u8],
            [40u8, 105u8, 54u8, 107u8],
            [43u8, 140u8, 73u8, 227u8],
            [47u8, 47u8, 241u8, 93u8],
            [48u8, 211u8, 232u8, 235u8],
            [49u8, 60u8, 229u8, 103u8],
            [54u8, 68u8, 229u8, 21u8],
            [54u8, 86u8, 138u8, 190u8],
            [58u8, 70u8, 177u8, 168u8],
            [64u8, 193u8, 15u8, 25u8],
            [66u8, 122u8, 192u8, 202u8],
            [66u8, 150u8, 108u8, 104u8],
            [75u8, 245u8, 215u8, 233u8],
            [79u8, 27u8, 252u8, 158u8],
            [88u8, 124u8, 222u8, 30u8],
            [90u8, 66u8, 57u8, 233u8],
            [90u8, 93u8, 177u8, 187u8],
            [92u8, 25u8, 169u8, 92u8],
            [93u8, 76u8, 98u8, 133u8],
            [99u8, 160u8, 218u8, 172u8],
            [101u8, 20u8, 85u8, 52u8],
            [111u8, 207u8, 255u8, 69u8],
            [112u8, 160u8, 130u8, 49u8],
            [114u8, 203u8, 220u8, 200u8],
            [120u8, 251u8, 127u8, 210u8],
            [121u8, 204u8, 103u8, 144u8],
            [122u8, 140u8, 209u8, 86u8],
            [126u8, 206u8, 190u8, 0u8],
            [131u8, 241u8, 33u8, 27u8],
            [132u8, 38u8, 173u8, 242u8],
            [132u8, 76u8, 144u8, 38u8],
            [132u8, 176u8, 25u8, 110u8],
            [138u8, 84u8, 37u8, 33u8],
            [141u8, 51u8, 67u8, 214u8],
            [142u8, 83u8, 158u8, 140u8],
            [144u8, 45u8, 85u8, 165u8],
            [145u8, 209u8, 72u8, 84u8],
            [145u8, 221u8, 173u8, 244u8],
            [148u8, 170u8, 34u8, 242u8],
            [149u8, 216u8, 155u8, 65u8],
            [154u8, 178u8, 78u8, 176u8],
            [155u8, 126u8, 246u8, 75u8],
            [162u8, 23u8, 253u8, 223u8],
            [169u8, 5u8, 156u8, 187u8],
            [170u8, 8u8, 42u8, 157u8],
            [176u8, 202u8, 37u8, 62u8],
            [183u8, 205u8, 198u8, 28u8],
            [187u8, 77u8, 68u8, 54u8],
            [192u8, 42u8, 231u8, 84u8],
            [195u8, 205u8, 165u8, 32u8],
            [196u8, 252u8, 69u8, 168u8],
            [201u8, 171u8, 0u8, 6u8],
            [213u8, 5u8, 172u8, 207u8],
            [213u8, 71u8, 116u8, 31u8],
            [221u8, 98u8, 237u8, 62u8],
            [241u8, 18u8, 126u8, 216u8],
            [247u8, 94u8, 133u8, 18u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SyndicateTokenCrosschainCalls {
        const NAME: &'static str = "SyndicateTokenCrosschainCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 66usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AIRDROP_MANAGER_ROLE(_) => {
                    <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::BRIDGE_MANAGER_ROLE(_) => {
                    <BRIDGE_MANAGER_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::CLOCK_MODE(_) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DOMAIN_SEPARATOR(_) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::EMISSION_BUDGET_MANAGER_ROLE(_) => {
                    <EMISSION_BUDGET_MANAGER_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::EMISSION_MINTER_ROLE(_) => {
                    <EMISSION_MINTER_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::INITIAL_MINT_SUPPLY(_) => {
                    <INITIAL_MINT_SUPPLYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_LOCK_DURATION(_) => {
                    <MAX_LOCK_DURATIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::TOTAL_SUPPLY(_) => {
                    <TOTAL_SUPPLYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocateEmissionBudget(_) => {
                    <allocateEmissionBudgetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allowance(_) => {
                    <allowanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::approve(_) => <approveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::balanceOf(_) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::bridgeConfigs(_) => {
                    <bridgeConfigsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::bridgeEmissionBudgets(_) => {
                    <bridgeEmissionBudgetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::burn(_) => <burnCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::burnFrom(_) => <burnFromCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::checkpoints(_) => {
                    <checkpointsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::clock(_) => <clockCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::crosschainBurn(_) => {
                    <crosschainBurnCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::crosschainMint(_) => {
                    <crosschainMintCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::decimals(_) => <decimalsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegate(_) => <delegateCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegateBySig(_) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegates(_) => {
                    <delegatesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllBridges(_) => {
                    <getAllBridgesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAvailableBurnLimit(_) => {
                    <getAvailableBurnLimitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAvailableMintLimit(_) => {
                    <getAvailableMintLimitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBridgeAtIndex(_) => {
                    <getBridgeAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBridgeConfig(_) => {
                    <getBridgeConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBridgeCount(_) => {
                    <getBridgeCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentTotalSupply(_) => {
                    <getCurrentTotalSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getEmissionBudget(_) => {
                    <getEmissionBudgetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPastTotalSupply(_) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPastVotes(_) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPastVotingPower(_) => {
                    <getPastVotingPowerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRemainingLockTime(_) => {
                    <getRemainingLockTimeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleAdmin(_) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVotes(_) => <getVotesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getVotingPower(_) => {
                    <getVotingPowerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::grantRole(_) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasRole(_) => <hasRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::hourlyBurnUsage(_) => {
                    <hourlyBurnUsageCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hourlyMintUsage(_) => {
                    <hourlyMintUsageCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isBridgeAuthorized(_) => {
                    <isBridgeAuthorizedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maxLockTimestamp(_) => {
                    <maxLockTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mint(_) => <mintCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::name(_) => <nameCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nonces(_) => <noncesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::numCheckpoints(_) => {
                    <numCheckpointsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::permit(_) => <permitCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::removeBridge(_) => {
                    <removeBridgeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceRole(_) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revokeRole(_) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setBridgeActive(_) => {
                    <setBridgeActiveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setBridgeLimits(_) => {
                    <setBridgeLimitsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setUnlockTimestamp(_) => {
                    <setUnlockTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::symbol(_) => <symbolCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::totalSupply(_) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transfer(_) => <transferCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::transferFrom(_) => {
                    <transferFromCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transfersLocked(_) => {
                    <transfersLockedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unlockTimestamp(_) => {
                    <unlockTimestampCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls>] = &[
                {
                    fn hourlyMintUsage(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <hourlyMintUsageCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::hourlyMintUsage)
                    }
                    hourlyMintUsage
                },
                {
                    fn supportsInterface(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn removeBridge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <removeBridgeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::removeBridge)
                    }
                    removeBridge
                },
                {
                    fn getEmissionBudget(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getEmissionBudgetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getEmissionBudget)
                    }
                    getEmissionBudget
                },
                {
                    fn name(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <nameCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::name)
                    }
                    name
                },
                {
                    fn approve(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <approveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::approve)
                    }
                    approve
                },
                {
                    fn totalSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <totalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::totalSupply)
                    }
                    totalSupply
                },
                {
                    fn crosschainMint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <crosschainMintCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::crosschainMint)
                    }
                    crosschainMint
                },
                {
                    fn transferFrom(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <transferFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::transferFrom)
                    }
                    transferFrom
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn bridgeEmissionBudgets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <bridgeEmissionBudgetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::bridgeEmissionBudgets)
                    }
                    bridgeEmissionBudgets
                },
                {
                    fn crosschainBurn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <crosschainBurnCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::crosschainBurn)
                    }
                    crosschainBurn
                },
                {
                    fn grantRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn getAvailableBurnLimit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getAvailableBurnLimitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getAvailableBurnLimit)
                    }
                    getAvailableBurnLimit
                },
                {
                    fn decimals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <decimalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::decimals)
                    }
                    decimals
                },
                {
                    fn DOMAIN_SEPARATOR(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::DOMAIN_SEPARATOR)
                    }
                    DOMAIN_SEPARATOR
                },
                {
                    fn renounceRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn getPastVotes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getPastVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getPastVotes)
                    }
                    getPastVotes
                },
                {
                    fn mint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <mintCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::mint)
                    }
                    mint
                },
                {
                    fn bridgeConfigs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <bridgeConfigsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::bridgeConfigs)
                    }
                    bridgeConfigs
                },
                {
                    fn burn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <burnCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::burn)
                    }
                    burn
                },
                {
                    fn CLOCK_MODE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::CLOCK_MODE)
                    }
                    CLOCK_MODE
                },
                {
                    fn MAX_LOCK_DURATION(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <MAX_LOCK_DURATIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::MAX_LOCK_DURATION)
                    }
                    MAX_LOCK_DURATION
                },
                {
                    fn delegates(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <delegatesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::delegates)
                    }
                    delegates
                },
                {
                    fn allocateEmissionBudget(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <allocateEmissionBudgetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::allocateEmissionBudget)
                    }
                    allocateEmissionBudget
                },
                {
                    fn getBridgeAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getBridgeAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getBridgeAtIndex)
                    }
                    getBridgeAtIndex
                },
                {
                    fn delegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <delegateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::delegate)
                    }
                    delegate
                },
                {
                    fn hourlyBurnUsage(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <hourlyBurnUsageCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::hourlyBurnUsage)
                    }
                    hourlyBurnUsage
                },
                {
                    fn setBridgeLimits(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <setBridgeLimitsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::setBridgeLimits)
                    }
                    setBridgeLimits
                },
                {
                    fn getBridgeCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getBridgeCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getBridgeCount)
                    }
                    getBridgeCount
                },
                {
                    fn numCheckpoints(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <numCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::numCheckpoints)
                    }
                    numCheckpoints
                },
                {
                    fn balanceOf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn getAllBridges(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getAllBridgesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getAllBridges)
                    }
                    getAllBridges
                },
                {
                    fn isBridgeAuthorized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <isBridgeAuthorizedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::isBridgeAuthorized)
                    }
                    isBridgeAuthorized
                },
                {
                    fn burnFrom(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <burnFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::burnFrom)
                    }
                    burnFrom
                },
                {
                    fn getRemainingLockTime(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getRemainingLockTimeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getRemainingLockTime)
                    }
                    getRemainingLockTime
                },
                {
                    fn nonces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <noncesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::nonces)
                    }
                    nonces
                },
                {
                    fn transfersLocked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <transfersLockedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::transfersLocked)
                    }
                    transfersLocked
                },
                {
                    fn maxLockTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <maxLockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::maxLockTimestamp)
                    }
                    maxLockTimestamp
                },
                {
                    fn setUnlockTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <setUnlockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::setUnlockTimestamp)
                    }
                    setUnlockTimestamp
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn AIRDROP_MANAGER_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::AIRDROP_MANAGER_ROLE)
                    }
                    AIRDROP_MANAGER_ROLE
                },
                {
                    fn EMISSION_MINTER_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <EMISSION_MINTER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::EMISSION_MINTER_ROLE)
                    }
                    EMISSION_MINTER_ROLE
                },
                {
                    fn getPastTotalSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getPastTotalSupply)
                    }
                    getPastTotalSupply
                },
                {
                    fn TOTAL_SUPPLY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <TOTAL_SUPPLYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::TOTAL_SUPPLY)
                    }
                    TOTAL_SUPPLY
                },
                {
                    fn hasRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn clock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <clockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::clock)
                    }
                    clock
                },
                {
                    fn getAvailableMintLimit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getAvailableMintLimitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getAvailableMintLimit)
                    }
                    getAvailableMintLimit
                },
                {
                    fn symbol(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <symbolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::symbol)
                    }
                    symbol
                },
                {
                    fn getVotes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getVotes)
                    }
                    getVotes
                },
                {
                    fn INITIAL_MINT_SUPPLY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <INITIAL_MINT_SUPPLYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::INITIAL_MINT_SUPPLY)
                    }
                    INITIAL_MINT_SUPPLY
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn transfer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <transferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::transfer)
                    }
                    transfer
                },
                {
                    fn unlockTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <unlockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::unlockTimestamp)
                    }
                    unlockTimestamp
                },
                {
                    fn getPastVotingPower(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getPastVotingPowerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getPastVotingPower)
                    }
                    getPastVotingPower
                },
                {
                    fn EMISSION_BUDGET_MANAGER_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <EMISSION_BUDGET_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateTokenCrosschainCalls::EMISSION_BUDGET_MANAGER_ROLE,
                            )
                    }
                    EMISSION_BUDGET_MANAGER_ROLE
                },
                {
                    fn getVotingPower(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getVotingPowerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getVotingPower)
                    }
                    getVotingPower
                },
                {
                    fn getCurrentTotalSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getCurrentTotalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getCurrentTotalSupply)
                    }
                    getCurrentTotalSupply
                },
                {
                    fn delegateBySig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <delegateBySigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::delegateBySig)
                    }
                    delegateBySig
                },
                {
                    fn getBridgeConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <getBridgeConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::getBridgeConfig)
                    }
                    getBridgeConfig
                },
                {
                    fn setBridgeActive(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <setBridgeActiveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::setBridgeActive)
                    }
                    setBridgeActive
                },
                {
                    fn permit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <permitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::permit)
                    }
                    permit
                },
                {
                    fn revokeRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn allowance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <allowanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::allowance)
                    }
                    allowance
                },
                {
                    fn checkpoints(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <checkpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::checkpoints)
                    }
                    checkpoints
                },
                {
                    fn BRIDGE_MANAGER_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainCalls> {
                        <BRIDGE_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainCalls::BRIDGE_MANAGER_ROLE)
                    }
                    BRIDGE_MANAGER_ROLE
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
                Self::AIRDROP_MANAGER_ROLE(inner) => {
                    <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BRIDGE_MANAGER_ROLE(inner) => {
                    <BRIDGE_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CLOCK_MODE(inner) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DOMAIN_SEPARATOR(inner) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EMISSION_BUDGET_MANAGER_ROLE(inner) => {
                    <EMISSION_BUDGET_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EMISSION_MINTER_ROLE(inner) => {
                    <EMISSION_MINTER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::INITIAL_MINT_SUPPLY(inner) => {
                    <INITIAL_MINT_SUPPLYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_LOCK_DURATION(inner) => {
                    <MAX_LOCK_DURATIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TOTAL_SUPPLY(inner) => {
                    <TOTAL_SUPPLYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocateEmissionBudget(inner) => {
                    <allocateEmissionBudgetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allowance(inner) => {
                    <allowanceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bridgeConfigs(inner) => {
                    <bridgeConfigsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::bridgeEmissionBudgets(inner) => {
                    <bridgeEmissionBudgetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::burn(inner) => {
                    <burnCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::burnFrom(inner) => {
                    <burnFromCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::checkpoints(inner) => {
                    <checkpointsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::clock(inner) => {
                    <clockCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::crosschainBurn(inner) => {
                    <crosschainBurnCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::crosschainMint(inner) => {
                    <crosschainMintCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::decimals(inner) => {
                    <decimalsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegate(inner) => {
                    <delegateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegateBySig(inner) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegates(inner) => {
                    <delegatesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllBridges(inner) => {
                    <getAllBridgesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAvailableBurnLimit(inner) => {
                    <getAvailableBurnLimitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAvailableMintLimit(inner) => {
                    <getAvailableMintLimitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBridgeAtIndex(inner) => {
                    <getBridgeAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBridgeConfig(inner) => {
                    <getBridgeConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBridgeCount(inner) => {
                    <getBridgeCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentTotalSupply(inner) => {
                    <getCurrentTotalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getEmissionBudget(inner) => {
                    <getEmissionBudgetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPastTotalSupply(inner) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPastVotes(inner) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPastVotingPower(inner) => {
                    <getPastVotingPowerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRemainingLockTime(inner) => {
                    <getRemainingLockTimeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getVotes(inner) => {
                    <getVotesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getVotingPower(inner) => {
                    <getVotingPowerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hourlyBurnUsage(inner) => {
                    <hourlyBurnUsageCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::hourlyMintUsage(inner) => {
                    <hourlyMintUsageCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isBridgeAuthorized(inner) => {
                    <isBridgeAuthorizedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::maxLockTimestamp(inner) => {
                    <maxLockTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::mint(inner) => {
                    <mintCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nonces(inner) => {
                    <noncesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::numCheckpoints(inner) => {
                    <numCheckpointsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::permit(inner) => {
                    <permitCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::removeBridge(inner) => {
                    <removeBridgeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setBridgeActive(inner) => {
                    <setBridgeActiveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setBridgeLimits(inner) => {
                    <setBridgeLimitsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setUnlockTimestamp(inner) => {
                    <setUnlockTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::symbol(inner) => {
                    <symbolCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::totalSupply(inner) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transfersLocked(inner) => {
                    <transfersLockedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unlockTimestamp(inner) => {
                    <unlockTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AIRDROP_MANAGER_ROLE(inner) => {
                    <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BRIDGE_MANAGER_ROLE(inner) => {
                    <BRIDGE_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CLOCK_MODE(inner) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DOMAIN_SEPARATOR(inner) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EMISSION_BUDGET_MANAGER_ROLE(inner) => {
                    <EMISSION_BUDGET_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EMISSION_MINTER_ROLE(inner) => {
                    <EMISSION_MINTER_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::INITIAL_MINT_SUPPLY(inner) => {
                    <INITIAL_MINT_SUPPLYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_LOCK_DURATION(inner) => {
                    <MAX_LOCK_DURATIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TOTAL_SUPPLY(inner) => {
                    <TOTAL_SUPPLYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allocateEmissionBudget(inner) => {
                    <allocateEmissionBudgetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allowance(inner) => {
                    <allowanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::bridgeConfigs(inner) => {
                    <bridgeConfigsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::bridgeEmissionBudgets(inner) => {
                    <bridgeEmissionBudgetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::burn(inner) => {
                    <burnCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::burnFrom(inner) => {
                    <burnFromCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkpoints(inner) => {
                    <checkpointsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::clock(inner) => {
                    <clockCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::crosschainBurn(inner) => {
                    <crosschainBurnCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::crosschainMint(inner) => {
                    <crosschainMintCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::decimals(inner) => {
                    <decimalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegate(inner) => {
                    <delegateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegateBySig(inner) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegates(inner) => {
                    <delegatesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllBridges(inner) => {
                    <getAllBridgesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAvailableBurnLimit(inner) => {
                    <getAvailableBurnLimitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAvailableMintLimit(inner) => {
                    <getAvailableMintLimitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBridgeAtIndex(inner) => {
                    <getBridgeAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBridgeConfig(inner) => {
                    <getBridgeConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBridgeCount(inner) => {
                    <getBridgeCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentTotalSupply(inner) => {
                    <getCurrentTotalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getEmissionBudget(inner) => {
                    <getEmissionBudgetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPastTotalSupply(inner) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPastVotes(inner) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPastVotingPower(inner) => {
                    <getPastVotingPowerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRemainingLockTime(inner) => {
                    <getRemainingLockTimeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getVotes(inner) => {
                    <getVotesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getVotingPower(inner) => {
                    <getVotingPowerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::hourlyBurnUsage(inner) => {
                    <hourlyBurnUsageCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hourlyMintUsage(inner) => {
                    <hourlyMintUsageCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isBridgeAuthorized(inner) => {
                    <isBridgeAuthorizedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::maxLockTimestamp(inner) => {
                    <maxLockTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mint(inner) => {
                    <mintCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::nonces(inner) => {
                    <noncesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::numCheckpoints(inner) => {
                    <numCheckpointsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::permit(inner) => {
                    <permitCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::removeBridge(inner) => {
                    <removeBridgeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setBridgeActive(inner) => {
                    <setBridgeActiveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setBridgeLimits(inner) => {
                    <setBridgeLimitsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setUnlockTimestamp(inner) => {
                    <setUnlockTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::symbol(inner) => {
                    <symbolCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::totalSupply(inner) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transfersLocked(inner) => {
                    <transfersLockedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unlockTimestamp(inner) => {
                    <unlockTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SyndicateTokenCrosschain`](self) custom errors.
    pub enum SyndicateTokenCrosschainErrors {
        #[allow(missing_docs)]
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        #[allow(missing_docs)]
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        #[allow(missing_docs)]
        BridgeMustBeContract(BridgeMustBeContract),
        #[allow(missing_docs)]
        BridgeNotActive(BridgeNotActive),
        #[allow(missing_docs)]
        BurnOnlyDuringLockPeriod(BurnOnlyDuringLockPeriod),
        #[allow(missing_docs)]
        CannotAddSelfAsBridge(CannotAddSelfAsBridge),
        #[allow(missing_docs)]
        CheckpointUnorderedInsertion(CheckpointUnorderedInsertion),
        #[allow(missing_docs)]
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        #[allow(missing_docs)]
        ERC20ExceededSafeSupply(ERC20ExceededSafeSupply),
        #[allow(missing_docs)]
        ERC20InsufficientAllowance(ERC20InsufficientAllowance),
        #[allow(missing_docs)]
        ERC20InsufficientBalance(ERC20InsufficientBalance),
        #[allow(missing_docs)]
        ERC20InvalidApprover(ERC20InvalidApprover),
        #[allow(missing_docs)]
        ERC20InvalidReceiver(ERC20InvalidReceiver),
        #[allow(missing_docs)]
        ERC20InvalidSender(ERC20InvalidSender),
        #[allow(missing_docs)]
        ERC20InvalidSpender(ERC20InvalidSpender),
        #[allow(missing_docs)]
        ERC2612ExpiredSignature(ERC2612ExpiredSignature),
        #[allow(missing_docs)]
        ERC2612InvalidSigner(ERC2612InvalidSigner),
        #[allow(missing_docs)]
        ERC5805FutureLookup(ERC5805FutureLookup),
        #[allow(missing_docs)]
        ERC6372InconsistentClock(ERC6372InconsistentClock),
        #[allow(missing_docs)]
        ExceedsTotalSupply(ExceedsTotalSupply),
        #[allow(missing_docs)]
        InsufficientBurnLimit(InsufficientBurnLimit),
        #[allow(missing_docs)]
        InsufficientEmissionBudget(InsufficientEmissionBudget),
        #[allow(missing_docs)]
        InsufficientMintLimit(InsufficientMintLimit),
        #[allow(missing_docs)]
        InvalidAccountNonce(InvalidAccountNonce),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        SafeCastOverflowedUintDowncast(SafeCastOverflowedUintDowncast),
        #[allow(missing_docs)]
        StringTooLong(StringTooLong),
        #[allow(missing_docs)]
        TransfersLocked(TransfersLocked),
        #[allow(missing_docs)]
        UnauthorizedBridge(UnauthorizedBridge),
        #[allow(missing_docs)]
        UnlockTimestampInPast(UnlockTimestampInPast),
        #[allow(missing_docs)]
        UnlockTimestampTooLate(UnlockTimestampTooLate),
        #[allow(missing_docs)]
        UnreasonableBurnLimit(UnreasonableBurnLimit),
        #[allow(missing_docs)]
        UnreasonableMintLimit(UnreasonableMintLimit),
        #[allow(missing_docs)]
        VotesExpiredSignature(VotesExpiredSignature),
        #[allow(missing_docs)]
        ZeroAddress(ZeroAddress),
        #[allow(missing_docs)]
        ZeroAmount(ZeroAmount),
    }
    #[automatically_derived]
    impl SyndicateTokenCrosschainErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [10u8, 57u8, 92u8, 1u8],
            [23u8, 126u8, 63u8, 195u8],
            [28u8, 177u8, 93u8, 38u8],
            [31u8, 42u8, 32u8, 5u8],
            [37u8, 32u8, 96u8, 29u8],
            [48u8, 90u8, 39u8, 169u8],
            [64u8, 237u8, 54u8, 123u8],
            [70u8, 131u8, 175u8, 14u8],
            [75u8, 128u8, 14u8, 70u8],
            [88u8, 204u8, 173u8, 0u8],
            [98u8, 121u8, 19u8, 2u8],
            [101u8, 133u8, 182u8, 13u8],
            [102u8, 151u8, 178u8, 50u8],
            [109u8, 252u8, 198u8, 80u8],
            [111u8, 240u8, 113u8, 64u8],
            [117u8, 45u8, 136u8, 192u8],
            [122u8, 222u8, 17u8, 92u8],
            [130u8, 84u8, 49u8, 218u8],
            [148u8, 40u8, 13u8, 98u8],
            [150u8, 198u8, 253u8, 30u8],
            [165u8, 101u8, 131u8, 83u8],
            [179u8, 81u8, 43u8, 12u8],
            [184u8, 181u8, 202u8, 45u8],
            [215u8, 139u8, 206u8, 12u8],
            [217u8, 46u8, 35u8, 61u8],
            [219u8, 137u8, 227u8, 244u8],
            [226u8, 81u8, 125u8, 63u8],
            [228u8, 80u8, 211u8, 140u8],
            [229u8, 254u8, 151u8, 162u8],
            [230u8, 2u8, 223u8, 5u8],
            [236u8, 68u8, 47u8, 5u8],
            [236u8, 211u8, 248u8, 30u8],
            [239u8, 105u8, 175u8, 101u8],
            [239u8, 218u8, 14u8, 6u8],
            [246u8, 69u8, 238u8, 223u8],
            [251u8, 140u8, 232u8, 201u8],
            [251u8, 143u8, 65u8, 178u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SyndicateTokenCrosschainErrors {
        const NAME: &'static str = "SyndicateTokenCrosschainErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 38usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccessControlBadConfirmation(_) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AccessControlUnauthorizedAccount(_) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BridgeMustBeContract(_) => {
                    <BridgeMustBeContract as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BridgeNotActive(_) => {
                    <BridgeNotActive as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BurnOnlyDuringLockPeriod(_) => {
                    <BurnOnlyDuringLockPeriod as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotAddSelfAsBridge(_) => {
                    <CannotAddSelfAsBridge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CheckpointUnorderedInsertion(_) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignature(_) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureLength(_) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureS(_) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20ExceededSafeSupply(_) => {
                    <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InsufficientAllowance(_) => {
                    <ERC20InsufficientAllowance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InsufficientBalance(_) => {
                    <ERC20InsufficientBalance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InvalidApprover(_) => {
                    <ERC20InvalidApprover as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InvalidReceiver(_) => {
                    <ERC20InvalidReceiver as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InvalidSender(_) => {
                    <ERC20InvalidSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InvalidSpender(_) => {
                    <ERC20InvalidSpender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC2612ExpiredSignature(_) => {
                    <ERC2612ExpiredSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC2612InvalidSigner(_) => {
                    <ERC2612InvalidSigner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC5805FutureLookup(_) => {
                    <ERC5805FutureLookup as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC6372InconsistentClock(_) => {
                    <ERC6372InconsistentClock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExceedsTotalSupply(_) => {
                    <ExceedsTotalSupply as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientBurnLimit(_) => {
                    <InsufficientBurnLimit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientEmissionBudget(_) => {
                    <InsufficientEmissionBudget as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientMintLimit(_) => {
                    <InsufficientMintLimit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidAccountNonce(_) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SafeCastOverflowedUintDowncast(_) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StringTooLong(_) => {
                    <StringTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TransfersLocked(_) => {
                    <TransfersLocked as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnauthorizedBridge(_) => {
                    <UnauthorizedBridge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnlockTimestampInPast(_) => {
                    <UnlockTimestampInPast as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnlockTimestampTooLate(_) => {
                    <UnlockTimestampTooLate as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnreasonableBurnLimit(_) => {
                    <UnreasonableBurnLimit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnreasonableMintLimit(_) => {
                    <UnreasonableMintLimit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::VotesExpiredSignature(_) => {
                    <VotesExpiredSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroAddress(_) => {
                    <ZeroAddress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroAmount(_) => {
                    <ZeroAmount as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors>] = &[
                {
                    fn UnreasonableMintLimit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <UnreasonableMintLimit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::UnreasonableMintLimit)
                    }
                    UnreasonableMintLimit
                },
                {
                    fn ExceedsTotalSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ExceedsTotalSupply as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::ExceedsTotalSupply)
                    }
                    ExceedsTotalSupply
                },
                {
                    fn ERC20ExceededSafeSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::ERC20ExceededSafeSupply)
                    }
                    ERC20ExceededSafeSupply
                },
                {
                    fn ZeroAmount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ZeroAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::ZeroAmount)
                    }
                    ZeroAmount
                },
                {
                    fn CheckpointUnorderedInsertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateTokenCrosschainErrors::CheckpointUnorderedInsertion,
                            )
                    }
                    CheckpointUnorderedInsertion
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn InsufficientMintLimit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <InsufficientMintLimit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::InsufficientMintLimit)
                    }
                    InsufficientMintLimit
                },
                {
                    fn VotesExpiredSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <VotesExpiredSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::VotesExpiredSignature)
                    }
                    VotesExpiredSignature
                },
                {
                    fn ERC2612InvalidSigner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ERC2612InvalidSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::ERC2612InvalidSigner)
                    }
                    ERC2612InvalidSigner
                },
                {
                    fn UnreasonableBurnLimit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <UnreasonableBurnLimit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::UnreasonableBurnLimit)
                    }
                    UnreasonableBurnLimit
                },
                {
                    fn ERC2612ExpiredSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ERC2612ExpiredSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::ERC2612ExpiredSignature)
                    }
                    ERC2612ExpiredSignature
                },
                {
                    fn UnauthorizedBridge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <UnauthorizedBridge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::UnauthorizedBridge)
                    }
                    UnauthorizedBridge
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateTokenCrosschainErrors::AccessControlBadConfirmation,
                            )
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn SafeCastOverflowedUintDowncast(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateTokenCrosschainErrors::SafeCastOverflowedUintDowncast,
                            )
                    }
                    SafeCastOverflowedUintDowncast
                },
                {
                    fn ERC6372InconsistentClock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateTokenCrosschainErrors::ERC6372InconsistentClock,
                            )
                    }
                    ERC6372InconsistentClock
                },
                {
                    fn InvalidAccountNonce(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <InvalidAccountNonce as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::InvalidAccountNonce)
                    }
                    InvalidAccountNonce
                },
                {
                    fn InsufficientEmissionBudget(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <InsufficientEmissionBudget as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateTokenCrosschainErrors::InsufficientEmissionBudget,
                            )
                    }
                    InsufficientEmissionBudget
                },
                {
                    fn BridgeMustBeContract(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <BridgeMustBeContract as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::BridgeMustBeContract)
                    }
                    BridgeMustBeContract
                },
                {
                    fn ERC20InvalidSpender(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ERC20InvalidSpender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::ERC20InvalidSpender)
                    }
                    ERC20InvalidSpender
                },
                {
                    fn ERC20InvalidSender(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ERC20InvalidSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::ERC20InvalidSender)
                    }
                    ERC20InvalidSender
                },
                {
                    fn UnlockTimestampInPast(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <UnlockTimestampInPast as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::UnlockTimestampInPast)
                    }
                    UnlockTimestampInPast
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn BurnOnlyDuringLockPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <BurnOnlyDuringLockPeriod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateTokenCrosschainErrors::BurnOnlyDuringLockPeriod,
                            )
                    }
                    BurnOnlyDuringLockPeriod
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn ZeroAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn TransfersLocked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <TransfersLocked as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::TransfersLocked)
                    }
                    TransfersLocked
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateTokenCrosschainErrors::AccessControlUnauthorizedAccount,
                            )
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn ERC20InsufficientBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ERC20InsufficientBalance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateTokenCrosschainErrors::ERC20InsufficientBalance,
                            )
                    }
                    ERC20InsufficientBalance
                },
                {
                    fn InsufficientBurnLimit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <InsufficientBurnLimit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::InsufficientBurnLimit)
                    }
                    InsufficientBurnLimit
                },
                {
                    fn ERC20InvalidApprover(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ERC20InvalidApprover as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::ERC20InvalidApprover)
                    }
                    ERC20InvalidApprover
                },
                {
                    fn ERC20InvalidReceiver(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ERC20InvalidReceiver as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::ERC20InvalidReceiver)
                    }
                    ERC20InvalidReceiver
                },
                {
                    fn ERC5805FutureLookup(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::ERC5805FutureLookup)
                    }
                    ERC5805FutureLookup
                },
                {
                    fn UnlockTimestampTooLate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <UnlockTimestampTooLate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::UnlockTimestampTooLate)
                    }
                    UnlockTimestampTooLate
                },
                {
                    fn BridgeNotActive(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <BridgeNotActive as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::BridgeNotActive)
                    }
                    BridgeNotActive
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn CannotAddSelfAsBridge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <CannotAddSelfAsBridge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCrosschainErrors::CannotAddSelfAsBridge)
                    }
                    CannotAddSelfAsBridge
                },
                {
                    fn ERC20InsufficientAllowance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ERC20InsufficientAllowance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateTokenCrosschainErrors::ERC20InsufficientAllowance,
                            )
                    }
                    ERC20InsufficientAllowance
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCrosschainErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SyndicateTokenCrosschainErrors::ECDSAInvalidSignatureLength,
                            )
                    }
                    ECDSAInvalidSignatureLength
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
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BridgeMustBeContract(inner) => {
                    <BridgeMustBeContract as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BridgeNotActive(inner) => {
                    <BridgeNotActive as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BurnOnlyDuringLockPeriod(inner) => {
                    <BurnOnlyDuringLockPeriod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotAddSelfAsBridge(inner) => {
                    <CannotAddSelfAsBridge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CheckpointUnorderedInsertion(inner) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20ExceededSafeSupply(inner) => {
                    <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InsufficientAllowance(inner) => {
                    <ERC20InsufficientAllowance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InsufficientBalance(inner) => {
                    <ERC20InsufficientBalance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InvalidApprover(inner) => {
                    <ERC20InvalidApprover as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InvalidReceiver(inner) => {
                    <ERC20InvalidReceiver as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InvalidSender(inner) => {
                    <ERC20InvalidSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InvalidSpender(inner) => {
                    <ERC20InvalidSpender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC2612ExpiredSignature(inner) => {
                    <ERC2612ExpiredSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC2612InvalidSigner(inner) => {
                    <ERC2612InvalidSigner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC5805FutureLookup(inner) => {
                    <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC6372InconsistentClock(inner) => {
                    <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ExceedsTotalSupply(inner) => {
                    <ExceedsTotalSupply as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientBurnLimit(inner) => {
                    <InsufficientBurnLimit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientEmissionBudget(inner) => {
                    <InsufficientEmissionBudget as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientMintLimit(inner) => {
                    <InsufficientMintLimit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidAccountNonce(inner) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::TransfersLocked(inner) => {
                    <TransfersLocked as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnauthorizedBridge(inner) => {
                    <UnauthorizedBridge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnlockTimestampInPast(inner) => {
                    <UnlockTimestampInPast as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnlockTimestampTooLate(inner) => {
                    <UnlockTimestampTooLate as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnreasonableBurnLimit(inner) => {
                    <UnreasonableBurnLimit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnreasonableMintLimit(inner) => {
                    <UnreasonableMintLimit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::VotesExpiredSignature(inner) => {
                    <VotesExpiredSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroAddress(inner) => {
                    <ZeroAddress as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroAmount(inner) => {
                    <ZeroAmount as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BridgeMustBeContract(inner) => {
                    <BridgeMustBeContract as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BridgeNotActive(inner) => {
                    <BridgeNotActive as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BurnOnlyDuringLockPeriod(inner) => {
                    <BurnOnlyDuringLockPeriod as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotAddSelfAsBridge(inner) => {
                    <CannotAddSelfAsBridge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CheckpointUnorderedInsertion(inner) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20ExceededSafeSupply(inner) => {
                    <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InsufficientAllowance(inner) => {
                    <ERC20InsufficientAllowance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InsufficientBalance(inner) => {
                    <ERC20InsufficientBalance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InvalidApprover(inner) => {
                    <ERC20InvalidApprover as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InvalidReceiver(inner) => {
                    <ERC20InvalidReceiver as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InvalidSender(inner) => {
                    <ERC20InvalidSender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InvalidSpender(inner) => {
                    <ERC20InvalidSpender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC2612ExpiredSignature(inner) => {
                    <ERC2612ExpiredSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC2612InvalidSigner(inner) => {
                    <ERC2612InvalidSigner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC5805FutureLookup(inner) => {
                    <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC6372InconsistentClock(inner) => {
                    <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExceedsTotalSupply(inner) => {
                    <ExceedsTotalSupply as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientBurnLimit(inner) => {
                    <InsufficientBurnLimit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientEmissionBudget(inner) => {
                    <InsufficientEmissionBudget as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientMintLimit(inner) => {
                    <InsufficientMintLimit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidAccountNonce(inner) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TransfersLocked(inner) => {
                    <TransfersLocked as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnauthorizedBridge(inner) => {
                    <UnauthorizedBridge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnlockTimestampInPast(inner) => {
                    <UnlockTimestampInPast as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnlockTimestampTooLate(inner) => {
                    <UnlockTimestampTooLate as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnreasonableBurnLimit(inner) => {
                    <UnreasonableBurnLimit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnreasonableMintLimit(inner) => {
                    <UnreasonableMintLimit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::VotesExpiredSignature(inner) => {
                    <VotesExpiredSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroAddress(inner) => {
                    <ZeroAddress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroAmount(inner) => {
                    <ZeroAmount as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`SyndicateTokenCrosschain`](self) events.
    pub enum SyndicateTokenCrosschainEvents {
        #[allow(missing_docs)]
        Approval(Approval),
        #[allow(missing_docs)]
        BridgeActiveStatusChanged(BridgeActiveStatusChanged),
        #[allow(missing_docs)]
        BridgeAdded(BridgeAdded),
        #[allow(missing_docs)]
        BridgeLimitsSet(BridgeLimitsSet),
        #[allow(missing_docs)]
        BridgeRemoved(BridgeRemoved),
        #[allow(missing_docs)]
        CrosschainBurn(CrosschainBurn),
        #[allow(missing_docs)]
        CrosschainMint(CrosschainMint),
        #[allow(missing_docs)]
        DelegateChanged(DelegateChanged),
        #[allow(missing_docs)]
        DelegateVotesChanged(DelegateVotesChanged),
        #[allow(missing_docs)]
        EIP712DomainChanged(EIP712DomainChanged),
        #[allow(missing_docs)]
        EmissionBudgetAllocated(EmissionBudgetAllocated),
        #[allow(missing_docs)]
        EmissionBudgetConsumed(EmissionBudgetConsumed),
        #[allow(missing_docs)]
        RoleAdminChanged(RoleAdminChanged),
        #[allow(missing_docs)]
        RoleGranted(RoleGranted),
        #[allow(missing_docs)]
        RoleRevoked(RoleRevoked),
        #[allow(missing_docs)]
        TokensBurnedByManager(TokensBurnedByManager),
        #[allow(missing_docs)]
        Transfer(Transfer),
        #[allow(missing_docs)]
        UnlockTimestampUpdated(UnlockTimestampUpdated),
    }
    #[automatically_derived]
    impl SyndicateTokenCrosschainEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                10u8,
                99u8,
                135u8,
                201u8,
                234u8,
                54u8,
                40u8,
                184u8,
                138u8,
                99u8,
                59u8,
                180u8,
                243u8,
                177u8,
                81u8,
                119u8,
                15u8,
                112u8,
                8u8,
                81u8,
                23u8,
                161u8,
                95u8,
                155u8,
                243u8,
                120u8,
                124u8,
                218u8,
                83u8,
                241u8,
                61u8,
                49u8,
            ],
            [
                47u8,
                135u8,
                136u8,
                17u8,
                126u8,
                126u8,
                255u8,
                29u8,
                130u8,
                233u8,
                38u8,
                236u8,
                121u8,
                73u8,
                1u8,
                209u8,
                124u8,
                120u8,
                2u8,
                74u8,
                80u8,
                39u8,
                9u8,
                64u8,
                48u8,
                69u8,
                64u8,
                167u8,
                51u8,
                101u8,
                111u8,
                13u8,
            ],
            [
                49u8,
                52u8,
                232u8,
                162u8,
                230u8,
                217u8,
                126u8,
                146u8,
                154u8,
                126u8,
                84u8,
                1u8,
                30u8,
                165u8,
                72u8,
                93u8,
                125u8,
                25u8,
                109u8,
                213u8,
                240u8,
                186u8,
                77u8,
                78u8,
                249u8,
                88u8,
                3u8,
                232u8,
                227u8,
                252u8,
                37u8,
                127u8,
            ],
            [
                93u8,
                157u8,
                80u8,
                52u8,
                101u8,
                108u8,
                179u8,
                235u8,
                251u8,
                6u8,
                85u8,
                5u8,
                124u8,
                215u8,
                249u8,
                180u8,
                7u8,
                122u8,
                155u8,
                66u8,
                255u8,
                66u8,
                206u8,
                34u8,
                60u8,
                186u8,
                197u8,
                188u8,
                88u8,
                109u8,
                33u8,
                38u8,
            ],
            [
                140u8,
                91u8,
                225u8,
                229u8,
                235u8,
                236u8,
                125u8,
                91u8,
                209u8,
                79u8,
                113u8,
                66u8,
                125u8,
                30u8,
                132u8,
                243u8,
                221u8,
                3u8,
                20u8,
                192u8,
                247u8,
                178u8,
                41u8,
                30u8,
                91u8,
                32u8,
                10u8,
                200u8,
                199u8,
                195u8,
                185u8,
                37u8,
            ],
            [
                156u8,
                134u8,
                104u8,
                219u8,
                50u8,
                72u8,
                69u8,
                6u8,
                93u8,
                43u8,
                154u8,
                42u8,
                24u8,
                59u8,
                211u8,
                20u8,
                31u8,
                99u8,
                1u8,
                143u8,
                84u8,
                130u8,
                130u8,
                218u8,
                241u8,
                141u8,
                164u8,
                156u8,
                203u8,
                248u8,
                140u8,
                51u8,
            ],
            [
                156u8,
                160u8,
                61u8,
                189u8,
                81u8,
                147u8,
                251u8,
                183u8,
                151u8,
                65u8,
                115u8,
                206u8,
                221u8,
                11u8,
                223u8,
                104u8,
                65u8,
                221u8,
                20u8,
                195u8,
                203u8,
                250u8,
                115u8,
                90u8,
                171u8,
                119u8,
                255u8,
                29u8,
                209u8,
                19u8,
                159u8,
                179u8,
            ],
            [
                170u8,
                128u8,
                125u8,
                10u8,
                191u8,
                48u8,
                217u8,
                25u8,
                104u8,
                199u8,
                71u8,
                140u8,
                102u8,
                182u8,
                216u8,
                37u8,
                33u8,
                161u8,
                6u8,
                175u8,
                19u8,
                237u8,
                160u8,
                54u8,
                226u8,
                3u8,
                109u8,
                169u8,
                175u8,
                22u8,
                137u8,
                88u8,
            ],
            [
                185u8,
                7u8,
                149u8,
                166u8,
                102u8,
                80u8,
                21u8,
                89u8,
                131u8,
                226u8,
                66u8,
                202u8,
                195u8,
                225u8,
                172u8,
                26u8,
                77u8,
                194u8,
                111u8,
                142u8,
                210u8,
                152u8,
                127u8,
                60u8,
                228u8,
                22u8,
                163u8,
                78u8,
                0u8,
                17u8,
                31u8,
                212u8,
            ],
            [
                188u8,
                35u8,
                236u8,
                127u8,
                19u8,
                19u8,
                21u8,
                11u8,
                4u8,
                123u8,
                255u8,
                131u8,
                208u8,
                132u8,
                91u8,
                5u8,
                100u8,
                186u8,
                161u8,
                52u8,
                105u8,
                141u8,
                209u8,
                27u8,
                176u8,
                172u8,
                208u8,
                247u8,
                212u8,
                22u8,
                222u8,
                125u8,
            ],
            [
                189u8,
                121u8,
                184u8,
                111u8,
                254u8,
                10u8,
                184u8,
                232u8,
                119u8,
                97u8,
                81u8,
                81u8,
                66u8,
                23u8,
                205u8,
                124u8,
                172u8,
                213u8,
                44u8,
                144u8,
                159u8,
                102u8,
                71u8,
                92u8,
                58u8,
                244u8,
                78u8,
                18u8,
                159u8,
                11u8,
                0u8,
                255u8,
            ],
            [
                190u8,
                244u8,
                248u8,
                28u8,
                24u8,
                20u8,
                198u8,
                65u8,
                237u8,
                232u8,
                94u8,
                186u8,
                172u8,
                241u8,
                157u8,
                4u8,
                139u8,
                44u8,
                91u8,
                85u8,
                152u8,
                10u8,
                223u8,
                166u8,
                239u8,
                15u8,
                149u8,
                108u8,
                101u8,
                19u8,
                53u8,
                162u8,
            ],
            [
                219u8,
                3u8,
                249u8,
                125u8,
                197u8,
                132u8,
                10u8,
                113u8,
                230u8,
                155u8,
                231u8,
                71u8,
                14u8,
                71u8,
                97u8,
                175u8,
                16u8,
                161u8,
                35u8,
                121u8,
                115u8,
                232u8,
                28u8,
                18u8,
                208u8,
                220u8,
                40u8,
                19u8,
                137u8,
                90u8,
                101u8,
                38u8,
            ],
            [
                221u8,
                104u8,
                150u8,
                220u8,
                241u8,
                212u8,
                179u8,
                17u8,
                204u8,
                168u8,
                125u8,
                209u8,
                155u8,
                187u8,
                162u8,
                234u8,
                156u8,
                226u8,
                248u8,
                103u8,
                193u8,
                86u8,
                136u8,
                120u8,
                160u8,
                67u8,
                138u8,
                102u8,
                161u8,
                175u8,
                238u8,
                236u8,
            ],
            [
                221u8,
                242u8,
                82u8,
                173u8,
                27u8,
                226u8,
                200u8,
                155u8,
                105u8,
                194u8,
                176u8,
                104u8,
                252u8,
                55u8,
                141u8,
                170u8,
                149u8,
                43u8,
                167u8,
                241u8,
                99u8,
                196u8,
                161u8,
                22u8,
                40u8,
                245u8,
                90u8,
                77u8,
                245u8,
                35u8,
                179u8,
                239u8,
            ],
            [
                222u8,
                34u8,
                186u8,
                255u8,
                3u8,
                142u8,
                58u8,
                62u8,
                8u8,
                64u8,
                124u8,
                189u8,
                246u8,
                23u8,
                222u8,
                237u8,
                116u8,
                232u8,
                105u8,
                167u8,
                186u8,
                81u8,
                125u8,
                246u8,
                17u8,
                227u8,
                49u8,
                49u8,
                198u8,
                230u8,
                234u8,
                4u8,
            ],
            [
                222u8,
                194u8,
                186u8,
                205u8,
                210u8,
                240u8,
                91u8,
                89u8,
                222u8,
                52u8,
                218u8,
                155u8,
                82u8,
                61u8,
                255u8,
                139u8,
                228u8,
                46u8,
                94u8,
                56u8,
                232u8,
                24u8,
                200u8,
                47u8,
                219u8,
                11u8,
                174u8,
                119u8,
                67u8,
                135u8,
                167u8,
                36u8,
            ],
            [
                246u8,
                57u8,
                31u8,
                92u8,
                50u8,
                217u8,
                198u8,
                157u8,
                42u8,
                71u8,
                234u8,
                103u8,
                11u8,
                68u8,
                41u8,
                116u8,
                181u8,
                57u8,
                53u8,
                209u8,
                237u8,
                199u8,
                253u8,
                100u8,
                235u8,
                33u8,
                224u8,
                71u8,
                168u8,
                57u8,
                23u8,
                27u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for SyndicateTokenCrosschainEvents {
        const NAME: &'static str = "SyndicateTokenCrosschainEvents";
        const COUNT: usize = 18usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Approval as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Approval as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Approval)
                }
                Some(
                    <BridgeActiveStatusChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <BridgeActiveStatusChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BridgeActiveStatusChanged)
                }
                Some(<BridgeAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BridgeAdded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BridgeAdded)
                }
                Some(<BridgeLimitsSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BridgeLimitsSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BridgeLimitsSet)
                }
                Some(<BridgeRemoved as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BridgeRemoved as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BridgeRemoved)
                }
                Some(<CrosschainBurn as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CrosschainBurn as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::CrosschainBurn)
                }
                Some(<CrosschainMint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CrosschainMint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::CrosschainMint)
                }
                Some(<DelegateChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DelegateChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DelegateChanged)
                }
                Some(
                    <DelegateVotesChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DelegateVotesChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DelegateVotesChanged)
                }
                Some(
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EIP712DomainChanged)
                }
                Some(
                    <EmissionBudgetAllocated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EmissionBudgetAllocated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EmissionBudgetAllocated)
                }
                Some(
                    <EmissionBudgetConsumed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EmissionBudgetConsumed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EmissionBudgetConsumed)
                }
                Some(<RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleAdminChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RoleAdminChanged)
                }
                Some(<RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleGranted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RoleGranted)
                }
                Some(<RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleRevoked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RoleRevoked)
                }
                Some(
                    <TokensBurnedByManager as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TokensBurnedByManager as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TokensBurnedByManager)
                }
                Some(<Transfer as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Transfer as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Transfer)
                }
                Some(
                    <UnlockTimestampUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UnlockTimestampUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UnlockTimestampUpdated)
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
    impl alloy_sol_types::private::IntoLogData for SyndicateTokenCrosschainEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BridgeActiveStatusChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BridgeAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BridgeLimitsSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BridgeRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CrosschainBurn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CrosschainMint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DelegateChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DelegateVotesChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EmissionBudgetAllocated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EmissionBudgetConsumed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TokensBurnedByManager(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UnlockTimestampUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BridgeActiveStatusChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BridgeAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BridgeLimitsSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BridgeRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CrosschainBurn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CrosschainMint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DelegateChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DelegateVotesChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EmissionBudgetAllocated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EmissionBudgetConsumed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TokensBurnedByManager(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UnlockTimestampUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SyndicateTokenCrosschain`](self) contract instance.

See the [wrapper's documentation](`SyndicateTokenCrosschainInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SyndicateTokenCrosschainInstance<T, P, N> {
        SyndicateTokenCrosschainInstance::<T, P, N>::new(address, provider)
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
        defaultAdmin: alloy::sol_types::private::Address,
        syndTreasuryAddress: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SyndicateTokenCrosschainInstance<T, P, N>>,
    > {
        SyndicateTokenCrosschainInstance::<
            T,
            P,
            N,
        >::deploy(provider, defaultAdmin, syndTreasuryAddress)
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
        defaultAdmin: alloy::sol_types::private::Address,
        syndTreasuryAddress: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        SyndicateTokenCrosschainInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, defaultAdmin, syndTreasuryAddress)
    }
    /**A [`SyndicateTokenCrosschain`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SyndicateTokenCrosschain`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SyndicateTokenCrosschainInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SyndicateTokenCrosschainInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SyndicateTokenCrosschainInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SyndicateTokenCrosschainInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`SyndicateTokenCrosschain`](self) contract instance.

See the [wrapper's documentation](`SyndicateTokenCrosschainInstance`) for more details.*/
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
            defaultAdmin: alloy::sol_types::private::Address,
            syndTreasuryAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<SyndicateTokenCrosschainInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                defaultAdmin,
                syndTreasuryAddress,
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
            defaultAdmin: alloy::sol_types::private::Address,
            syndTreasuryAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            defaultAdmin,
                            syndTreasuryAddress,
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
    impl<T, P: ::core::clone::Clone, N> SyndicateTokenCrosschainInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SyndicateTokenCrosschainInstance<T, P, N> {
            SyndicateTokenCrosschainInstance {
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
    > SyndicateTokenCrosschainInstance<T, P, N> {
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
        ///Creates a new call builder for the [`AIRDROP_MANAGER_ROLE`] function.
        pub fn AIRDROP_MANAGER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, AIRDROP_MANAGER_ROLECall, N> {
            self.call_builder(&AIRDROP_MANAGER_ROLECall {})
        }
        ///Creates a new call builder for the [`BRIDGE_MANAGER_ROLE`] function.
        pub fn BRIDGE_MANAGER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, BRIDGE_MANAGER_ROLECall, N> {
            self.call_builder(&BRIDGE_MANAGER_ROLECall {})
        }
        ///Creates a new call builder for the [`CLOCK_MODE`] function.
        pub fn CLOCK_MODE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, CLOCK_MODECall, N> {
            self.call_builder(&CLOCK_MODECall {})
        }
        ///Creates a new call builder for the [`DEFAULT_ADMIN_ROLE`] function.
        pub fn DEFAULT_ADMIN_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DEFAULT_ADMIN_ROLECall, N> {
            self.call_builder(&DEFAULT_ADMIN_ROLECall {})
        }
        ///Creates a new call builder for the [`DOMAIN_SEPARATOR`] function.
        pub fn DOMAIN_SEPARATOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DOMAIN_SEPARATORCall, N> {
            self.call_builder(&DOMAIN_SEPARATORCall {})
        }
        ///Creates a new call builder for the [`EMISSION_BUDGET_MANAGER_ROLE`] function.
        pub fn EMISSION_BUDGET_MANAGER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, EMISSION_BUDGET_MANAGER_ROLECall, N> {
            self.call_builder(
                &EMISSION_BUDGET_MANAGER_ROLECall {
                },
            )
        }
        ///Creates a new call builder for the [`EMISSION_MINTER_ROLE`] function.
        pub fn EMISSION_MINTER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, EMISSION_MINTER_ROLECall, N> {
            self.call_builder(&EMISSION_MINTER_ROLECall {})
        }
        ///Creates a new call builder for the [`INITIAL_MINT_SUPPLY`] function.
        pub fn INITIAL_MINT_SUPPLY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, INITIAL_MINT_SUPPLYCall, N> {
            self.call_builder(&INITIAL_MINT_SUPPLYCall {})
        }
        ///Creates a new call builder for the [`MAX_LOCK_DURATION`] function.
        pub fn MAX_LOCK_DURATION(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_LOCK_DURATIONCall, N> {
            self.call_builder(&MAX_LOCK_DURATIONCall {})
        }
        ///Creates a new call builder for the [`TOTAL_SUPPLY`] function.
        pub fn TOTAL_SUPPLY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, TOTAL_SUPPLYCall, N> {
            self.call_builder(&TOTAL_SUPPLYCall {})
        }
        ///Creates a new call builder for the [`allocateEmissionBudget`] function.
        pub fn allocateEmissionBudget(
            &self,
            bridge: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocateEmissionBudgetCall, N> {
            self.call_builder(
                &allocateEmissionBudgetCall {
                    bridge,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`allowance`] function.
        pub fn allowance(
            &self,
            owner: alloy::sol_types::private::Address,
            spender: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, allowanceCall, N> {
            self.call_builder(&allowanceCall { owner, spender })
        }
        ///Creates a new call builder for the [`approve`] function.
        pub fn approve(
            &self,
            spender: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, approveCall, N> {
            self.call_builder(&approveCall { spender, value })
        }
        ///Creates a new call builder for the [`balanceOf`] function.
        pub fn balanceOf(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, balanceOfCall, N> {
            self.call_builder(&balanceOfCall { account })
        }
        ///Creates a new call builder for the [`bridgeConfigs`] function.
        pub fn bridgeConfigs(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, bridgeConfigsCall, N> {
            self.call_builder(&bridgeConfigsCall { _0 })
        }
        ///Creates a new call builder for the [`bridgeEmissionBudgets`] function.
        pub fn bridgeEmissionBudgets(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, bridgeEmissionBudgetsCall, N> {
            self.call_builder(&bridgeEmissionBudgetsCall { _0 })
        }
        ///Creates a new call builder for the [`burn`] function.
        pub fn burn(
            &self,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, burnCall, N> {
            self.call_builder(&burnCall { amount })
        }
        ///Creates a new call builder for the [`burnFrom`] function.
        pub fn burnFrom(
            &self,
            from: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, burnFromCall, N> {
            self.call_builder(&burnFromCall { from, amount })
        }
        ///Creates a new call builder for the [`checkpoints`] function.
        pub fn checkpoints(
            &self,
            account: alloy::sol_types::private::Address,
            pos: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkpointsCall, N> {
            self.call_builder(&checkpointsCall { account, pos })
        }
        ///Creates a new call builder for the [`clock`] function.
        pub fn clock(&self) -> alloy_contract::SolCallBuilder<T, &P, clockCall, N> {
            self.call_builder(&clockCall {})
        }
        ///Creates a new call builder for the [`crosschainBurn`] function.
        pub fn crosschainBurn(
            &self,
            from: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, crosschainBurnCall, N> {
            self.call_builder(&crosschainBurnCall { from, amount })
        }
        ///Creates a new call builder for the [`crosschainMint`] function.
        pub fn crosschainMint(
            &self,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, crosschainMintCall, N> {
            self.call_builder(&crosschainMintCall { to, amount })
        }
        ///Creates a new call builder for the [`decimals`] function.
        pub fn decimals(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, decimalsCall, N> {
            self.call_builder(&decimalsCall {})
        }
        ///Creates a new call builder for the [`delegate`] function.
        pub fn delegate(
            &self,
            delegatee: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateCall, N> {
            self.call_builder(&delegateCall { delegatee })
        }
        ///Creates a new call builder for the [`delegateBySig`] function.
        pub fn delegateBySig(
            &self,
            delegatee: alloy::sol_types::private::Address,
            nonce: alloy::sol_types::private::primitives::aliases::U256,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
            v: u8,
            r: alloy::sol_types::private::FixedBytes<32>,
            s: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateBySigCall, N> {
            self.call_builder(
                &delegateBySigCall {
                    delegatee,
                    nonce,
                    expiry,
                    v,
                    r,
                    s,
                },
            )
        }
        ///Creates a new call builder for the [`delegates`] function.
        pub fn delegates(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegatesCall, N> {
            self.call_builder(&delegatesCall { account })
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall {})
        }
        ///Creates a new call builder for the [`getAllBridges`] function.
        pub fn getAllBridges(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllBridgesCall, N> {
            self.call_builder(&getAllBridgesCall {})
        }
        ///Creates a new call builder for the [`getAvailableBurnLimit`] function.
        pub fn getAvailableBurnLimit(
            &self,
            bridge: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAvailableBurnLimitCall, N> {
            self.call_builder(
                &getAvailableBurnLimitCall {
                    bridge,
                },
            )
        }
        ///Creates a new call builder for the [`getAvailableMintLimit`] function.
        pub fn getAvailableMintLimit(
            &self,
            bridge: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAvailableMintLimitCall, N> {
            self.call_builder(
                &getAvailableMintLimitCall {
                    bridge,
                },
            )
        }
        ///Creates a new call builder for the [`getBridgeAtIndex`] function.
        pub fn getBridgeAtIndex(
            &self,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBridgeAtIndexCall, N> {
            self.call_builder(&getBridgeAtIndexCall { index })
        }
        ///Creates a new call builder for the [`getBridgeConfig`] function.
        pub fn getBridgeConfig(
            &self,
            bridge: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBridgeConfigCall, N> {
            self.call_builder(&getBridgeConfigCall { bridge })
        }
        ///Creates a new call builder for the [`getBridgeCount`] function.
        pub fn getBridgeCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBridgeCountCall, N> {
            self.call_builder(&getBridgeCountCall {})
        }
        ///Creates a new call builder for the [`getCurrentTotalSupply`] function.
        pub fn getCurrentTotalSupply(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentTotalSupplyCall, N> {
            self.call_builder(&getCurrentTotalSupplyCall {})
        }
        ///Creates a new call builder for the [`getEmissionBudget`] function.
        pub fn getEmissionBudget(
            &self,
            bridge: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getEmissionBudgetCall, N> {
            self.call_builder(&getEmissionBudgetCall { bridge })
        }
        ///Creates a new call builder for the [`getPastTotalSupply`] function.
        pub fn getPastTotalSupply(
            &self,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPastTotalSupplyCall, N> {
            self.call_builder(
                &getPastTotalSupplyCall {
                    timepoint,
                },
            )
        }
        ///Creates a new call builder for the [`getPastVotes`] function.
        pub fn getPastVotes(
            &self,
            account: alloy::sol_types::private::Address,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPastVotesCall, N> {
            self.call_builder(
                &getPastVotesCall {
                    account,
                    timepoint,
                },
            )
        }
        ///Creates a new call builder for the [`getPastVotingPower`] function.
        pub fn getPastVotingPower(
            &self,
            account: alloy::sol_types::private::Address,
            blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPastVotingPowerCall, N> {
            self.call_builder(
                &getPastVotingPowerCall {
                    account,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getRemainingLockTime`] function.
        pub fn getRemainingLockTime(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRemainingLockTimeCall, N> {
            self.call_builder(&getRemainingLockTimeCall {})
        }
        ///Creates a new call builder for the [`getRoleAdmin`] function.
        pub fn getRoleAdmin(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRoleAdminCall, N> {
            self.call_builder(&getRoleAdminCall { role })
        }
        ///Creates a new call builder for the [`getVotes`] function.
        pub fn getVotes(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getVotesCall, N> {
            self.call_builder(&getVotesCall { account })
        }
        ///Creates a new call builder for the [`getVotingPower`] function.
        pub fn getVotingPower(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getVotingPowerCall, N> {
            self.call_builder(&getVotingPowerCall { account })
        }
        ///Creates a new call builder for the [`grantRole`] function.
        pub fn grantRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, grantRoleCall, N> {
            self.call_builder(&grantRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hasRole`] function.
        pub fn hasRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, hasRoleCall, N> {
            self.call_builder(&hasRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hourlyBurnUsage`] function.
        pub fn hourlyBurnUsage(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, hourlyBurnUsageCall, N> {
            self.call_builder(&hourlyBurnUsageCall { _0, _1 })
        }
        ///Creates a new call builder for the [`hourlyMintUsage`] function.
        pub fn hourlyMintUsage(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, hourlyMintUsageCall, N> {
            self.call_builder(&hourlyMintUsageCall { _0, _1 })
        }
        ///Creates a new call builder for the [`isBridgeAuthorized`] function.
        pub fn isBridgeAuthorized(
            &self,
            bridge: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isBridgeAuthorizedCall, N> {
            self.call_builder(&isBridgeAuthorizedCall { bridge })
        }
        ///Creates a new call builder for the [`maxLockTimestamp`] function.
        pub fn maxLockTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, maxLockTimestampCall, N> {
            self.call_builder(&maxLockTimestampCall {})
        }
        ///Creates a new call builder for the [`mint`] function.
        pub fn mint(
            &self,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, mintCall, N> {
            self.call_builder(&mintCall { to, amount })
        }
        ///Creates a new call builder for the [`name`] function.
        pub fn name(&self) -> alloy_contract::SolCallBuilder<T, &P, nameCall, N> {
            self.call_builder(&nameCall {})
        }
        ///Creates a new call builder for the [`nonces`] function.
        pub fn nonces(
            &self,
            owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, noncesCall, N> {
            self.call_builder(&noncesCall { owner })
        }
        ///Creates a new call builder for the [`numCheckpoints`] function.
        pub fn numCheckpoints(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, numCheckpointsCall, N> {
            self.call_builder(&numCheckpointsCall { account })
        }
        ///Creates a new call builder for the [`permit`] function.
        pub fn permit(
            &self,
            owner: alloy::sol_types::private::Address,
            spender: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
            v: u8,
            r: alloy::sol_types::private::FixedBytes<32>,
            s: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, permitCall, N> {
            self.call_builder(
                &permitCall {
                    owner,
                    spender,
                    value,
                    deadline,
                    v,
                    r,
                    s,
                },
            )
        }
        ///Creates a new call builder for the [`removeBridge`] function.
        pub fn removeBridge(
            &self,
            bridge: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeBridgeCall, N> {
            self.call_builder(&removeBridgeCall { bridge })
        }
        ///Creates a new call builder for the [`renounceRole`] function.
        pub fn renounceRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            callerConfirmation: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceRoleCall, N> {
            self.call_builder(
                &renounceRoleCall {
                    role,
                    callerConfirmation,
                },
            )
        }
        ///Creates a new call builder for the [`revokeRole`] function.
        pub fn revokeRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, revokeRoleCall, N> {
            self.call_builder(&revokeRoleCall { role, account })
        }
        ///Creates a new call builder for the [`setBridgeActive`] function.
        pub fn setBridgeActive(
            &self,
            bridge: alloy::sol_types::private::Address,
            isActive: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setBridgeActiveCall, N> {
            self.call_builder(
                &setBridgeActiveCall {
                    bridge,
                    isActive,
                },
            )
        }
        ///Creates a new call builder for the [`setBridgeLimits`] function.
        pub fn setBridgeLimits(
            &self,
            bridge: alloy::sol_types::private::Address,
            dailyMintLimit: alloy::sol_types::private::primitives::aliases::U256,
            dailyBurnLimit: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setBridgeLimitsCall, N> {
            self.call_builder(
                &setBridgeLimitsCall {
                    bridge,
                    dailyMintLimit,
                    dailyBurnLimit,
                },
            )
        }
        ///Creates a new call builder for the [`setUnlockTimestamp`] function.
        pub fn setUnlockTimestamp(
            &self,
            newUnlockTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setUnlockTimestampCall, N> {
            self.call_builder(
                &setUnlockTimestampCall {
                    newUnlockTimestamp,
                },
            )
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
        }
        ///Creates a new call builder for the [`symbol`] function.
        pub fn symbol(&self) -> alloy_contract::SolCallBuilder<T, &P, symbolCall, N> {
            self.call_builder(&symbolCall {})
        }
        ///Creates a new call builder for the [`totalSupply`] function.
        pub fn totalSupply(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, totalSupplyCall, N> {
            self.call_builder(&totalSupplyCall {})
        }
        ///Creates a new call builder for the [`transfer`] function.
        pub fn transfer(
            &self,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferCall, N> {
            self.call_builder(&transferCall { to, value })
        }
        ///Creates a new call builder for the [`transferFrom`] function.
        pub fn transferFrom(
            &self,
            from: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferFromCall, N> {
            self.call_builder(
                &transferFromCall {
                    from,
                    to,
                    value,
                },
            )
        }
        ///Creates a new call builder for the [`transfersLocked`] function.
        pub fn transfersLocked(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, transfersLockedCall, N> {
            self.call_builder(&transfersLockedCall {})
        }
        ///Creates a new call builder for the [`unlockTimestamp`] function.
        pub fn unlockTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, unlockTimestampCall, N> {
            self.call_builder(&unlockTimestampCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SyndicateTokenCrosschainInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Approval`] event.
        pub fn Approval_filter(&self) -> alloy_contract::Event<T, &P, Approval, N> {
            self.event_filter::<Approval>()
        }
        ///Creates a new event filter for the [`BridgeActiveStatusChanged`] event.
        pub fn BridgeActiveStatusChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BridgeActiveStatusChanged, N> {
            self.event_filter::<BridgeActiveStatusChanged>()
        }
        ///Creates a new event filter for the [`BridgeAdded`] event.
        pub fn BridgeAdded_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BridgeAdded, N> {
            self.event_filter::<BridgeAdded>()
        }
        ///Creates a new event filter for the [`BridgeLimitsSet`] event.
        pub fn BridgeLimitsSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BridgeLimitsSet, N> {
            self.event_filter::<BridgeLimitsSet>()
        }
        ///Creates a new event filter for the [`BridgeRemoved`] event.
        pub fn BridgeRemoved_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BridgeRemoved, N> {
            self.event_filter::<BridgeRemoved>()
        }
        ///Creates a new event filter for the [`CrosschainBurn`] event.
        pub fn CrosschainBurn_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, CrosschainBurn, N> {
            self.event_filter::<CrosschainBurn>()
        }
        ///Creates a new event filter for the [`CrosschainMint`] event.
        pub fn CrosschainMint_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, CrosschainMint, N> {
            self.event_filter::<CrosschainMint>()
        }
        ///Creates a new event filter for the [`DelegateChanged`] event.
        pub fn DelegateChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DelegateChanged, N> {
            self.event_filter::<DelegateChanged>()
        }
        ///Creates a new event filter for the [`DelegateVotesChanged`] event.
        pub fn DelegateVotesChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DelegateVotesChanged, N> {
            self.event_filter::<DelegateVotesChanged>()
        }
        ///Creates a new event filter for the [`EIP712DomainChanged`] event.
        pub fn EIP712DomainChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EIP712DomainChanged, N> {
            self.event_filter::<EIP712DomainChanged>()
        }
        ///Creates a new event filter for the [`EmissionBudgetAllocated`] event.
        pub fn EmissionBudgetAllocated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EmissionBudgetAllocated, N> {
            self.event_filter::<EmissionBudgetAllocated>()
        }
        ///Creates a new event filter for the [`EmissionBudgetConsumed`] event.
        pub fn EmissionBudgetConsumed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EmissionBudgetConsumed, N> {
            self.event_filter::<EmissionBudgetConsumed>()
        }
        ///Creates a new event filter for the [`RoleAdminChanged`] event.
        pub fn RoleAdminChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RoleAdminChanged, N> {
            self.event_filter::<RoleAdminChanged>()
        }
        ///Creates a new event filter for the [`RoleGranted`] event.
        pub fn RoleGranted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RoleGranted, N> {
            self.event_filter::<RoleGranted>()
        }
        ///Creates a new event filter for the [`RoleRevoked`] event.
        pub fn RoleRevoked_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RoleRevoked, N> {
            self.event_filter::<RoleRevoked>()
        }
        ///Creates a new event filter for the [`TokensBurnedByManager`] event.
        pub fn TokensBurnedByManager_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TokensBurnedByManager, N> {
            self.event_filter::<TokensBurnedByManager>()
        }
        ///Creates a new event filter for the [`Transfer`] event.
        pub fn Transfer_filter(&self) -> alloy_contract::Event<T, &P, Transfer, N> {
            self.event_filter::<Transfer>()
        }
        ///Creates a new event filter for the [`UnlockTimestampUpdated`] event.
        pub fn UnlockTimestampUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UnlockTimestampUpdated, N> {
            self.event_filter::<UnlockTimestampUpdated>()
        }
    }
}
