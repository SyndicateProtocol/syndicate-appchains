///Module containing a contract's types and functions.
/**

```solidity
library RequireCompositeModule {
    type CheckType is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod RequireCompositeModule {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckType(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<CheckType> for u8 {
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
        impl CheckType {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
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
        impl From<u8> for CheckType {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<CheckType> for u8 {
            fn from(value: CheckType) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for CheckType {
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
        impl alloy_sol_types::EventTopic for CheckType {
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
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`RequireCompositeModule`](self) contract instance.

See the [wrapper's documentation](`RequireCompositeModuleInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> RequireCompositeModuleInstance<P, N> {
        RequireCompositeModuleInstance::<P, N>::new(address, provider)
    }
    /**A [`RequireCompositeModule`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`RequireCompositeModule`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct RequireCompositeModuleInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for RequireCompositeModuleInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("RequireCompositeModuleInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > RequireCompositeModuleInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`RequireCompositeModule`](self) contract instance.

See the [wrapper's documentation](`RequireCompositeModuleInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
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
    impl<P: ::core::clone::Clone, N> RequireCompositeModuleInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> RequireCompositeModuleInstance<P, N> {
            RequireCompositeModuleInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > RequireCompositeModuleInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > RequireCompositeModuleInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library StdInvariant {
    struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
    struct FuzzInterface { address addr; string[] artifacts; }
    struct FuzzSelector { address addr; bytes4[] selectors; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod StdInvariant {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzArtifactSelector {
        #[allow(missing_docs)]
        pub artifact: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub selectors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
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
        impl ::core::convert::From<FuzzArtifactSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzArtifactSelector) -> Self {
                (value.artifact, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzArtifactSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    artifact: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzArtifactSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzArtifactSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.artifact,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.selectors),
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
        impl alloy_sol_types::SolType for FuzzArtifactSelector {
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
        impl alloy_sol_types::SolStruct for FuzzArtifactSelector {
            const NAME: &'static str = "FuzzArtifactSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzArtifactSelector(string artifact,bytes4[] selectors)",
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.artifact,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.selectors)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzArtifactSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifact,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.selectors,
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
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifact,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.selectors,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct FuzzInterface { address addr; string[] artifacts; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzInterface {
        #[allow(missing_docs)]
        pub addr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub artifacts: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
        impl ::core::convert::From<FuzzInterface> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzInterface) -> Self {
                (value.addr, value.artifacts)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzInterface {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    artifacts: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzInterface {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzInterface {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.artifacts),
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
        impl alloy_sol_types::SolType for FuzzInterface {
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
        impl alloy_sol_types::SolStruct for FuzzInterface {
            const NAME: &'static str = "FuzzInterface";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzInterface(address addr,string[] artifacts)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.artifacts)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzInterface {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifacts,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::String,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifacts,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct FuzzSelector { address addr; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzSelector {
        #[allow(missing_docs)]
        pub addr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub selectors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
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
        impl ::core::convert::From<FuzzSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzSelector) -> Self {
                (value.addr, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.selectors),
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
        impl alloy_sol_types::SolType for FuzzSelector {
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
        impl alloy_sol_types::SolStruct for FuzzSelector {
            const NAME: &'static str = "FuzzSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzSelector(address addr,bytes4[] selectors)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.selectors)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.selectors,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.selectors,
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
    /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> StdInvariantInstance<P, N> {
        StdInvariantInstance::<P, N>::new(address, provider)
    }
    /**A [`StdInvariant`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StdInvariant`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StdInvariantInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for StdInvariantInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StdInvariantInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
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
    impl<P: ::core::clone::Clone, N> StdInvariantInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StdInvariantInstance<P, N> {
            StdInvariantInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library RequireCompositeModule {
    type CheckType is uint8;
}

library StdInvariant {
    struct FuzzArtifactSelector {
        string artifact;
        bytes4[] selectors;
    }
    struct FuzzInterface {
        address addr;
        string[] artifacts;
    }
    struct FuzzSelector {
        address addr;
        bytes4[] selectors;
    }
}

interface RequireCompositeModuleTest {
    event CheckTypeUpdated(address indexed check, RequireCompositeModule.CheckType indexed oldType, RequireCompositeModule.CheckType indexed newType);
    event PermissionCheckAdded(address indexed check);
    event PermissionCheckAddedWithType(address indexed check, RequireCompositeModule.CheckType indexed checkType);
    event PermissionCheckRemoved(address indexed check);
    event log(string);
    event log_address(address);
    event log_array(uint256[] val);
    event log_array(int256[] val);
    event log_array(address[] val);
    event log_bytes(bytes);
    event log_bytes32(bytes32);
    event log_int(int256);
    event log_named_address(string key, address val);
    event log_named_array(string key, uint256[] val);
    event log_named_array(string key, int256[] val);
    event log_named_array(string key, address[] val);
    event log_named_bytes(string key, bytes val);
    event log_named_bytes32(string key, bytes32 val);
    event log_named_decimal_int(string key, int256 val, uint256 decimals);
    event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
    event log_named_int(string key, int256 val);
    event log_named_string(string key, string val);
    event log_named_uint(string key, uint256 val);
    event log_string(string);
    event log_uint(uint256);
    event logs(bytes);

    function IS_TEST() external view returns (bool);
    function admin() external view returns (address);
    function alwaysAllowModule() external view returns (address);
    function alwaysDenyModule() external view returns (address);
    function compositeModule() external view returns (address);
    function configModule1() external view returns (address);
    function configModule2() external view returns (address);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function setUp() external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testAddPermissionCheck() external;
    function testAddPermissionCheckWithType() external;
    function testCannotAddSameCheckTwice() external;
    function testCannotAddZeroAddress() external;
    function testCannotRemoveNonExistentCheck() external;
    function testCannotUpdateTypeForNonExistentCheck() external;
    function testConstructor() external view;
    function testGetAllPermissionChecksWithTypes() external;
    function testIsAllowedMixedChecksAllPass() external;
    function testIsAllowedMixedChecksAndFailOrPass() external;
    function testIsAllowedMixedChecksAndPassOrFail() external;
    function testIsAllowedNoChecks() external view;
    function testIsAllowedOnlyAndChecksOneFails() external;
    function testIsAllowedOnlyAndChecksPass() external;
    function testIsAllowedOnlyOrChecksAllFail() external;
    function testIsAllowedOnlyOrChecksOnePass() external;
    function testModifyChecksDuringExecution() external;
    function testNoEventWhenTypeNotChanged() external;
    function testOnlyOwnerCanAddChecks() external;
    function testRemovePermissionCheck() external;
    function testUpdateCheckType() external;
    function user1() external view returns (address);
    function user2() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "IS_TEST",
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
    "name": "admin",
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
    "name": "alwaysAllowModule",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockPermissionModule"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "alwaysDenyModule",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockPermissionModule"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "compositeModule",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract RequireCompositeModule"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "configModule1",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockPermissionModule"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "configModule2",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockPermissionModule"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "failed",
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
    "name": "setUp",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "targetArtifactSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifactSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzArtifactSelector[]",
        "components": [
          {
            "name": "artifact",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetInterfaces",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedInterfaces_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzInterface[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "artifacts",
            "type": "string[]",
            "internalType": "string[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "testAddPermissionCheck",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testAddPermissionCheckWithType",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testCannotAddSameCheckTwice",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testCannotAddZeroAddress",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testCannotRemoveNonExistentCheck",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testCannotUpdateTypeForNonExistentCheck",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testConstructor",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "testGetAllPermissionChecksWithTypes",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedMixedChecksAllPass",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedMixedChecksAndFailOrPass",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedMixedChecksAndPassOrFail",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedNoChecks",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "testIsAllowedOnlyAndChecksOneFails",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedOnlyAndChecksPass",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedOnlyOrChecksAllFail",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedOnlyOrChecksOnePass",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testModifyChecksDuringExecution",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testNoEventWhenTypeNotChanged",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testOnlyOwnerCanAddChecks",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testRemovePermissionCheck",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testUpdateCheckType",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "user1",
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
    "name": "user2",
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
    "type": "event",
    "name": "CheckTypeUpdated",
    "inputs": [
      {
        "name": "check",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldType",
        "type": "uint8",
        "indexed": true,
        "internalType": "enum RequireCompositeModule.CheckType"
      },
      {
        "name": "newType",
        "type": "uint8",
        "indexed": true,
        "internalType": "enum RequireCompositeModule.CheckType"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PermissionCheckAdded",
    "inputs": [
      {
        "name": "check",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PermissionCheckAddedWithType",
    "inputs": [
      {
        "name": "check",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "checkType",
        "type": "uint8",
        "indexed": true,
        "internalType": "enum RequireCompositeModule.CheckType"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PermissionCheckRemoved",
    "inputs": [
      {
        "name": "check",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_address",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes32",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_int",
    "inputs": [
      {
        "name": "",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_address",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes32",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_string",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_string",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_uint",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "logs",
    "inputs": [
      {
        "name": "",
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
pub mod RequireCompositeModuleTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60808060405234602f57600160ff19600c541617600c55600160ff19601f541617601f5561642b90816100348239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f905f3560e01c9081625bda9714613e70575080630a9254e414613a9c5780630c22bdf81461373657806313a7f30e1461355b57806318563fca146135345780631d759713146133695780631ed7831c146132eb57806325388d16146130c25780632ade388014612ece578063315f411814612ea757806332ee689a14612baa57806334d923db14612b5e57806339fe398d146129745780633e4c68ae146127795780633e5e3c23146126fb5780633f6afde6146126d15780633f7286f4146126535780634412deeb1461242657806366d9a9a0146122e95780636b44511a146122c257806370c27dfa146120e257806385226c8114612058578063859021a414611d10578063916a17c614611c665780639461922e14611aaa5780639732e261146118df5780639abdf47a1461169e5780639faae1a91461147e5780639fda5caf14610e4a578063a9c4d10414610e23578063ac1717b014610dfc578063b0464fdc14610d52578063b5508aa914610cc8578063b9edb1af14610ca1578063ba414fa614610c7c578063c166e043146108ca578063c2e9f2e414610814578063e20c9f7114610786578063f7a4d60114610428578063f818989514610236578063f851a440146102105763fa7626d4146101eb575f80fd5b3461020d578060031936011261020d57602060ff601f54166040519015158152f35b80fd5b503461020d578060031936011261020d5760206001600160a01b03815416604051908152f35b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757610410575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d7576103fb575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f75782916044839260405194859384927f0838bbd40000000000000000000000000000000000000000000000000000000084526004840152600160248401525af180156103d7576103e2575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b816103d091614285565b61020d5780f35b6040513d84823e3d90fd5b816103ec91614285565b61020d57805f610371565b5050fd5b8161040591614285565b61020d57805f610304565b8161041a91614285565b61020d57805f6102aa565b50fd5b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757610771575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d75761075c575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517f491cc7c200000000000000000000000000000000000000000000000000000000815260016004820152600160248201526001604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757610747575b506001600160a01b036023541660405190600183827fa09388b78c1a36296c3bc4cc7a25b5768b1d9e0e628ed553e26c26aaedf946228280a46001600160a01b03601f5460081c16803b156107425760448385819381957f0838bbd40000000000000000000000000000000000000000000000000000000084526004840152600160248401525af180156103d75761072d575b50506001600160a01b03601f5460081c1660206001600160a01b03602354166024604051809481937f21284f7e00000000000000000000000000000000000000000000000000000000835260048301525afa9081156103d75782916106fe575b5060028110156106d15761067e90614977565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526021600452fd5b610720915060203d602011610726575b6107188183614285565b810190614409565b5f61066b565b503d61070e565b8161073791614285565b61020d57805f61060b565b505050fd5b8161075191614285565b61020d57805f610578565b8161076691614285565b61020d57805f6104f0565b8161077b91614285565b61020d57805f61049c565b503461020d578060031936011261020d5760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106107f5576107f1856107e581870382614285565b6040519182918261404e565b0390f35b82546001600160a01b03168452602090930192600192830192016107ce565b503461020d578060031936011261020d57600460206001600160a01b03601f5460081c16604051928380927f8da5cb5b0000000000000000000000000000000000000000000000000000000082525afa80156103d757829061088b575b61088891506001600160a01b036020541690614adb565b80f35b506020813d6020116108c2575b816108a560209383614285565b810103126108be576108b961088891614320565b610871565b5080fd5b3d9150610898565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757610c67575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517f491cc7c20000000000000000000000000000000000000000000000000000000081528181806109a960048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757610c52575b506001600160a01b0360235416604051907f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e78380a2737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610425577f491cc7c2000000000000000000000000000000000000000000000000000000008152818180610a6860048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757610c3d575b506001600160a01b03602354166040519082817fab1bfc946643fef3ad4e44e43027bc5acf93baafa545c3ed0370761d4bb425da8280a36001600160a01b03601f5460081c16803b1561074257604483858193819563052eefd160e01b84526004840152600160248401525af180156103d757610c28575b50506001600160a01b03601f5460081c166040517f1b42c7110000000000000000000000000000000000000000000000000000000081528281600481855afa8015610c1d57610b706001600160a01b03916020938691610bfb575b50610b6b8151614977565b6143bf565b5116610b886001600160a01b03602354168092614adb565b6024604051809481937f21284f7e00000000000000000000000000000000000000000000000000000000835260048301525afa9081156103d7578291610bdc575b5060028110156106d15761067e906149ee565b610bf5915060203d602011610726576107188183614285565b5f610bc9565b610c1791503d8088833e610c0f8183614285565b810190614399565b5f610b60565b6040513d85823e3d90fd5b81610c3291614285565b61020d57805f610b05565b81610c4791614285565b61020d57805f610a8d565b81610c5c91614285565b61020d57805f6109ce565b81610c7191614285565b61020d57805f61093e565b503461020d578060031936011261020d576020610c97614822565b6040519015158152f35b503461020d578060031936011261020d5760206001600160a01b0360225416604051908152f35b503461020d578060031936011261020d57601954610ce581614308565b91610cf36040519384614285565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b838310610d3557604051806107f18782614128565b600160208192610d4485614421565b815201920192019190610d20565b503461020d578060031936011261020d57601c54610d6f81614308565b91610d7d6040519384614285565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310610dbf57604051806107f187826141a5565b60026020600192604051610dd28161423c565b6001600160a01b038654168152610dea858701614544565b83820152815201920192019190610daa565b503461020d578060031936011261020d5760206001600160a01b0360215416604051908152f35b503461020d578060031936011261020d5760206001600160a01b0360245416604051908152f35b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757611469575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d757611454575b506001600160a01b03601f5460081c166001600160a01b0360255416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d75761143f575b506001600160a01b0360255416803b15610425578180916024604051809481937f2dcfafd70000000000000000000000000000000000000000000000000000000083528160048401525af180156103d75761142a575b506001600160a01b03601f5460081c166001600160a01b0360265416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d757611415575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757611400575b50506110e560206001600160a01b03601f5460081c166001600160a01b0360215416906001600160a01b0360225416604051926110ac8585614285565b5f8452604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016142de565b03915afa80156103d7576111009183916113e1575b506148fb565b806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576113cc575b506001600160a01b0360265416803b15610425578180916024604051809481937f2dcfafd70000000000000000000000000000000000000000000000000000000083528160048401525af180156103d7576113b7575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576113a2575b506001600160a01b03602154166112926040519161122b602084614285565b5f83526112666040519384927ff59be56c00000000000000000000000000000000000000000000000000000000602085015260248401614524565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282614285565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557816112ed91604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190614090565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761138d575b505061134f60206001600160a01b03601f5460081c166001600160a01b0360215416906001600160a01b0360225416604051926110ac8585614285565b03915afa80156103d757611361575080f35b6113829060203d602011611386575b61137a8183614285565b8101906142c6565b5080f35b503d611370565b8161139791614285565b61020d57805f611312565b816113ac91614285565b61020d57805f61120c565b816113c191614285565b61020d57805f6111b9565b816113d691614285565b61020d57805f611163565b6113fa915060203d6020116113865761137a8183614285565b5f6110fa565b8161140a91614285565b61020d57805f61106f565b8161141f91614285565b61020d57805f61101c565b8161143491614285565b61020d57805f610fc2565b8161144991614285565b61020d57805f610f6c565b8161145e91614285565b61020d57805f610f12565b8161147391614285565b61020d57805f610ebe565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757611689575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d757611674575b506001600160a01b03601f5460081c166001600160a01b0360255416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d75761165f575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761164a575b505061163060206001600160a01b03601f5460081c166001600160a01b0360215416906001600160a01b0360225416604051926110ac8585614285565b03915afa80156103d7576108889183916113e157506148fb565b8161165491614285565b61020d57805f6115f3565b8161166991614285565b61020d57805f6115a0565b8161167e91614285565b61020d57805f611546565b8161169391614285565b61020d57805f6114f2565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576118ca575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d7576118b5575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fa2d86a1e000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576118a0575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d7576103e2575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b816118aa91614285565b61020d57805f6117f8565b816118bf91614285565b61020d57805f611766565b816118d491614285565b61020d57805f611712565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757611a95575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fe6c4247b000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757611a80575b506001600160a01b03601f5460081c16803b156104255781809160446040518094819363052eefd160e01b8352816004840152600160248401525af180156103d7576103e2575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b81611a8a91614285565b61020d57805f6119e5565b81611a9f91614285565b61020d57805f611953565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757611c51575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d757611c3c575b506001600160a01b03601f5460081c166001600160a01b0360245416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d7576113b7575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576113a257506001600160a01b03602154166112926040519161122b602084614285565b81611c4691614285565b61020d57805f611b72565b81611c5b91614285565b61020d57805f611b1e565b503461020d578060031936011261020d57601d54611c8381614308565b91611c916040519384614285565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310611cd357604051806107f187826141a5565b60026020600192604051611ce68161423c565b6001600160a01b038654168152611cfe858701614544565b83820152815201920192019190611cbe565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757612043575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d75761202e575b506001600160a01b03601f5460081c166001600160a01b0360245416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d757612019575b506004906001600160a01b03601f5460081c16604051928380927fa213fd220000000000000000000000000000000000000000000000000000000082525afa9081156103d75782908392611f41575b506001600160a01b03611ec482611e9b611ed89451614a64565b611ea58551614a64565b611ebf83611eb2836143bf565b5116846024541690614adb565b6143f9565b51166001600160a01b036023541690614adb565b611ee9611ee4826143bf565b6147e8565b6002811015611f1457611f0291611ebf611ee492614977565b60028110156106d15761067e906149ee565b6024837f4e487b710000000000000000000000000000000000000000000000000000000081526021600452fd5b9150503d8083833e611f538183614285565b81019060408183031261201557805167ffffffffffffffff811161200d5782611f7d918301614334565b9060208101519067ffffffffffffffff821161201157019180601f8401121561200d57825190611fac82614308565b93611fba6040519586614285565b82855260208086019360051b82010191821161200957602001915b818310611fec575050506001600160a01b03611e81565b8251600281101561200557815260209283019201611fd5565b8680fd5b8580fd5b8380fd5b8480fd5b8280fd5b8161202391614285565b61020d57805f611e32565b8161203891614285565b61020d57805f611dd8565b8161204d91614285565b61020d57805f611d84565b503461020d578060031936011261020d57601a5461207581614308565b916120836040519384614285565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b8383106120c557604051806107f18782614128565b6001602081926120d485614421565b8152019201920191906120b0565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576122ad575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d757612298575b506001600160a01b03601f5460081c166001600160a01b0360245416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d75761165f575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761164a57505061163060206001600160a01b03601f5460081c166001600160a01b0360215416906001600160a01b0360225416604051926110ac8585614285565b816122a291614285565b61020d57805f6121b0565b816122b791614285565b61020d57805f612156565b503461020d578060031936011261020d5760206001600160a01b0360235416604051908152f35b503461020d578060031936011261020d57601b5461230681614308565b6123136040519182614285565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b8383106123eb57868587604051928392602084019060208552518091526040840160408260051b8601019392905b82821061238057505050500390f35b919360206123db827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc06001959799849503018652885190836123cb8351604084526040840190614090565b92015190848184039101526140d3565b9601920192018594939192612371565b600260206001926040516123fe8161423c565b61240786614421565b8152612414858701614544565b83820152815201920192019190612343565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761263e575b506001600160a01b03601f5460081c166001600160a01b0360245416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d757612629575b506001600160a01b0360255416803b15610425578180916024604051809481937f2dcfafd70000000000000000000000000000000000000000000000000000000083528160048401525af180156103d757612614575b506001600160a01b03601f5460081c166001600160a01b0360255416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d7576113b7575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576113a257506001600160a01b03602154166112926040519161122b602084614285565b8161261e91614285565b61020d57805f61254a565b8161263391614285565b61020d57805f6124f4565b8161264891614285565b61020d57805f61249a565b503461020d578060031936011261020d5760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b8181106126b2576107f1856107e581870382614285565b82546001600160a01b031684526020909301926001928301920161269b565b503461020d578060031936011261020d5760206001600160a01b03601f5460081c16604051908152f35b503461020d578060031936011261020d5760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b81811061275a576107f1856107e581870382614285565b82546001600160a01b0316845260209093019260019283019201612743565b503461020d578060031936011261020d57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761295f575b506001600160a01b0360215416604051907f118cdaa7000000000000000000000000000000000000000000000000000000006020830152602482015260248152612838604482614285565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610425578161289391604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190614090565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576118a057506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d7576103e2575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b8161296991614285565b61020d57805f6127ed565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757612b49575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f3d0f293d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757612b34575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f75782916024839260405194859384927f04f386f400000000000000000000000000000000000000000000000000000000845260048401525af180156103d7576103e2575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b81612b3e91614285565b61020d57805f612a7a565b81612b5391614285565b61020d57805f6129e8565b503461020d578060031936011261020d5761163060206001600160a01b03601f5460081c166001600160a01b0360215416906001600160a01b0360225416604051926110ac8585614285565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757612e92575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d757612e7d575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517f491cc7c20000000000000000000000000000000000000000000000000000000081526001600482015281602482015281604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757612e68575b506001600160a01b036023541660405190807fb5d68ca46372bbe6ec138d3d0423608269b3117496a46268f86080cdbcbea9be8480a26001600160a01b03601f5460081c16803b156107425760248385819381957f04f386f400000000000000000000000000000000000000000000000000000000845260048401525af180156103d757612e53575b50506001600160a01b03601f5460081c166040517f1b42c7110000000000000000000000000000000000000000000000000000000081528281600481855afa8015610c1d57612dd8918491612e39575b50516149ee565b60206001600160a01b03602354166024604051809481937f21284f7e00000000000000000000000000000000000000000000000000000000835260048301525afa9081156103d7578291610bdc575060028110156106d15761067e906149ee565b612e4d91503d8086833e610c0f8183614285565b5f612dd1565b81612e5d91614285565b61020d57805f612d81565b81612e7291614285565b61020d57805f612cf8565b81612e8791614285565b61020d57805f612c72565b81612e9c91614285565b61020d57805f612c1e565b503461020d578060031936011261020d5760206001600160a01b0360255416604051908152f35b503461020d578060031936011261020d57601e54612eeb81614308565b612ef86040519182614285565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b8383106130395786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310612f645786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110612ff057505050505060208060019297019301930190928695949293612f57565b909192939460208061302c837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa087600196030189528951614090565b9701950193929101612fcc565b6040516130458161423c565b6001600160a01b03835416815260018301805461306181614308565b9161306f6040519384614285565b8183528a526020808b20908b9084015b8382106130a5575050505060019282602092836002950152815201920192019190612f28565b6001602081926130b486614421565b81520193019101909161307f565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576132d6575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d7576132c1575b506001600160a01b03601f5460081c166001600160a01b0360245416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d7576132ac575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757613297575b506001600160a01b03602454166112926001600160a01b03602154169161126660405161325f602082614285565b5f81526040519485937f0cecaaea000000000000000000000000000000000000000000000000000000006020860152602485016142de565b816132a191614285565b61020d57805f613231565b816132b691614285565b61020d57805f6131de565b816132cb91614285565b61020d57805f61318a565b816132e091614285565b61020d57805f613136565b503461020d578060031936011261020d5760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b81811061334a576107f1856107e581870382614285565b82546001600160a01b0316845260209093019260019283019201613333565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761351f575b506001600160a01b03601f5460081c166001600160a01b0360245416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d75761350a575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d7576132ac575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761329757506001600160a01b03602454166112926001600160a01b03602154169161126660405161325f602082614285565b8161351491614285565b61020d57805f613431565b8161352991614285565b61020d57805f6133dd565b503461020d578060031936011261020d5760206001600160a01b0360265416604051908152f35b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757613721575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f3d0f293d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103fb57506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f75782916044839260405194859384927f0838bbd40000000000000000000000000000000000000000000000000000000084526004840152600160248401525af180156103d7576103e2575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b8161372b91614285565b61020d57805f6135cf565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757613a87575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517f491cc7c200000000000000000000000000000000000000000000000000000000815281818061381560048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757613a72575b506001600160a01b0360235416604051907f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e78380a2737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610425577f491cc7c20000000000000000000000000000000000000000000000000000000081528181806138d460048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757613a5d575b506001600160a01b0360235416604051906001817fab1bfc946643fef3ad4e44e43027bc5acf93baafa545c3ed0370761d4bb425da8580a36001600160a01b03601f5460081c16803b156107425760648385819381956258e30f60e91b8452600484015260016024840152600160448401525af180156103d757613a48575b50506001600160a01b03601f5460081c166040517f1b42c7110000000000000000000000000000000000000000000000000000000081528281600481855afa8015610c1d576139dd6001600160a01b03916020938691610bfb5750610b6b8151614977565b51166139f56001600160a01b03602354168092614adb565b6024604051809481937f21284f7e00000000000000000000000000000000000000000000000000000000835260048301525afa9081156103d75782916106fe575060028110156106d15761067e90614977565b81613a5291614285565b61020d57805f613978565b81613a6791614285565b61020d57805f6138f9565b81613a7c91614285565b61020d57805f61383a565b81613a9191614285565b61020d57805f6137aa565b503461020d578060031936011261020d5760017fffffffffffffffffffffffff0000000000000000000000000000000000000000602054161760205560027fffffffffffffffffffffffff0000000000000000000000000000000000000000602154161760215560037fffffffffffffffffffffffff00000000000000000000000000000000000000006022541617602255737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516303223eab60e11b815260016004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757613e5b575b50506001600160a01b0360205416604051906116e4908183019183831067ffffffffffffffff841117613e0157918391602093614b5e8439815203019082f08015613df4577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f556040516101e9908181019080821067ffffffffffffffff831117613e2e5760208161624293858583396001815203019084f08015610c1d576001600160a01b03167fffffffffffffffffffffffff0000000000000000000000000000000000000000602354161760235560405182810181811067ffffffffffffffff821117613e0157816020918585833986815203019084f08015610c1d576001600160a01b03167fffffffffffffffffffffffff0000000000000000000000000000000000000000602454161760245560405182810181811067ffffffffffffffff821117613e015781602091858583396001815203019084f08015610c1d576001600160a01b03167fffffffffffffffffffffffff00000000000000000000000000000000000000006025541617602555604051918083019183831067ffffffffffffffff841117613e015791839160209383396001815203019082f08015613df4576001600160a01b03167fffffffffffffffffffffffff00000000000000000000000000000000000000006026541617602655737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b50604051903d90823e3d90fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b81613e6591614285565b61020d57805f613b86565b90503461404a575f60031936011261404a576001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561404a576303223eab60e11b825260048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561403f5761402c575b50806001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d757614017575b506001600160a01b03601f5460081c166001600160a01b0360255416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d75761165f575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761164a57505061163060206001600160a01b03601f5460081c166001600160a01b0360215416906001600160a01b0360225416604051926110ac8585614285565b8161402191614285565b61020d57805f613f35565b61403891505f90614285565b5f5f613ee0565b6040513d5f823e3d90fd5b5f80fd5b60206040818301928281528451809452019201905f5b8181106140715750505090565b82516001600160a01b0316845260209384019390920191600101614064565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b8181106140f05750505090565b82517fffffffff00000000000000000000000000000000000000000000000000000000168452602093840193909201916001016140e3565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061415a57505050505090565b9091929394602080614196837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187528951614090565b9701930193019193929061414b565b602081016020825282518091526040820191602060408360051b8301019401925f915b8383106141d757505050505090565b909192939460208061422d837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b038151168452015191818582015201906140d3565b970193019301919392906141c8565b6040810190811067ffffffffffffffff82111761425857604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761425857604052565b9081602091031261404a5751801515810361404a5790565b6001600160a01b036143059493816060941683521660208201528160408201520190614090565b90565b67ffffffffffffffff81116142585760051b60200190565b51906001600160a01b038216820361404a57565b9080601f8301121561404a57815161434b81614308565b926143596040519485614285565b81845260208085019260051b82010192831161404a57602001905b8282106143815750505090565b6020809161438e84614320565b815201910190614374565b9060208282031261404a57815167ffffffffffffffff811161404a576143059201614334565b8051156143cc5760200190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b8051600110156143cc5760400190565b9081602091031261404a5751600281101561404a5790565b90604051915f8154908160011c926001831692831561451a575b6020851084146144ed5784875286939081156144ad5750600114614469575b5061446792500383614285565b565b90505f9291925260205f20905f915b818310614491575050906020614467928201015f61445a565b6020919350806001915483858901015201910190918492614478565b602093506144679592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f61445a565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f169361443b565b6040906001600160a01b0361430594931681528160208201520190614090565b90604051918281549182825260208201905f5260205f20925f905b80600783011061475b57614467945491818110614725575b8181106146ef575b8181106146b9575b818110614683575b81811061464d575b818110614617575b8181106145e2575b106145b5575b500383614285565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f6145ad565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b1681520193016145a7565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b16815201930161459f565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b168152019301614597565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b16815201930161458f565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b168152019301614587565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b16815201930161457f565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b168152019301614577565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e082015201940192018592939161455f565b5160028110156147f55790565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60085460ff1680156148315790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561403f575f916148c9575b50151590565b90506020813d6020116148f3575b816148e460209383614285565b8101031261404a57515f6148c3565b3d91506148d7565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561404a57604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561403f5761496d5750565b5f61446791614285565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561404a57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152600160248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561403f5761496d5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561404a57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561403f5761496d5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561404a57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152600260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561403f5761496d5750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561404a576001600160a01b039081604051937f515361f60000000000000000000000000000000000000000000000000000000085521660048401521660248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561403f5761496d575056fe60803460b857601f6116e438819003918201601f19168301916001600160401b0383118484101760bc5780849260209460405283398101031260b857516001600160a01b0381169081900360b857801560a5575f80546001600160a01b031981168317825560405192916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a361161390816100d18239f35b631e4fbdf760e01b5f525f60045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c806304f386f41461094a578063052eefd11461083e5780630838bbd4146108065780631b42c711146106e957806321284f7e1461069a578063715018a61461061e5780637a3979dc1461058b5780638da5cb5b14610559578063a213fd221461035a578063a26b4a881461033f578063b1c61e001461016f5763f2fde38b1461009d575f80fd5b3461016b57602060031936011261016b5773ffffffffffffffffffffffffffffffffffffffff6100cb610aa4565b6100d3611093565b16801561013f5773ffffffffffffffffffffffffffffffffffffffff5f54827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f80fd5b3461016b57606060031936011261016b57610188610aa4565b60243590600282101561016b5760443590811515820361016b576101aa611093565b6101b2611093565b73ffffffffffffffffffffffffffffffffffffffff8116918215610317576101d9826110df565b6102ef57602860015410156102c757156102b9576101f69061152a565b1561025b57807f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e75f80a2805f5260036020526102358260405f20610b1d565b7fab1bfc946643fef3ad4e44e43027bc5acf93baafa545c3ed0370761d4bb425da5f80a3005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f41646472657373206e6f742061646465640000000000000000000000000000006044820152fd5b6102c29061142a565b6101f6565b7f13d867a2000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fa2d86a1e000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe6c4247b000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461016b575f60031936011261016b57602060405160288152f35b3461016b575f60031936011261016b5760015461037681610c70565b9061038081610c58565b61038d6040519182610bea565b81815261039982610c58565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe060208301910136823760015f9081527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff165b8482108061053a575b1561052c5761043e8287610d19565b73ffffffffffffffffffffffffffffffffffffffff82168091525f52600360205260ff60405f205416906104728386610d19565b9160028110156104ff5761048692526113c9565b901561049c576104969091610cbf565b90610426565b5050906104bc9392505b6020604051948594604086526040860190610ac7565b918483038286015251918281520191905f5b8181106104dc575050500390f35b91935091602080826104f16001948851610b10565b0194019101918493926104ce565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5050906104bc9392506104a6565b5073ffffffffffffffffffffffffffffffffffffffff8116151561042f565b3461016b575f60031936011261016b57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b3461016b57606060031936011261016b576105a4610aa4565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361016b576044359067ffffffffffffffff821161016b573660238301121561016b5781600401359067ffffffffffffffff821161016b57366024838501011161016b576020936024610614940191610de7565b6040519015158152f35b3461016b575f60031936011261016b57610636611093565b5f73ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461016b57602060031936011261016b5773ffffffffffffffffffffffffffffffffffffffff6106c8610aa4565b165f526003602052602060ff60405f2054166106e76040518092610b10565bf35b3461016b575f60031936011261016b5760015461070581610c70565b60015f9081527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5491929173ffffffffffffffffffffffffffffffffffffffff165b828210806107e7575b156107db576107a5906107878386610d19565b73ffffffffffffffffffffffffffffffffffffffff821690526113c9565b90156107bb576107b59091610cbf565b9061076b565b5050506107d7905b604051918291602083526020830190610ac7565b0390f35b5050506107d7906107c3565b5073ffffffffffffffffffffffffffffffffffffffff81161515610774565b3461016b57604060031936011261016b5761081f610aa4565b602435600281101561016b5761083c91610837611093565b610b54565b005b3461016b57604060031936011261016b57610857610aa4565b60243590811515820361016b5761086c611093565b610874611093565b73ffffffffffffffffffffffffffffffffffffffff81169182156103175761089b826110df565b6102ef57602860015410156102c7571561093c576108b89061152a565b1561025b57805f917f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e78380a28082526003602052604082207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0081541690557fab1bfc946643fef3ad4e44e43027bc5acf93baafa545c3ed0370761d4bb425da8280a3005b6109459061142a565b6108b8565b3461016b57602060031936011261016b57610963610aa4565b61096b611093565b610973611093565b73ffffffffffffffffffffffffffffffffffffffff81169081156103175761099a816110df565b15610a7c5773ffffffffffffffffffffffffffffffffffffffff6109be83926112b4565b1603610a1e57807fb5d68ca46372bbe6ec138d3d0423608269b3117496a46268f86080cdbcbea9be5f80a25f52600360205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0081541690555f80f35b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601360248201527f41646472657373206e6f742072656d6f766564000000000000000000000000006044820152fd5b7f3d0f293d000000000000000000000000000000000000000000000000000000005f5260045ffd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361016b57565b90602080835192838152019201905f5b818110610ae45750505090565b825173ffffffffffffffffffffffffffffffffffffffff16845260209384019390920191600101610ad7565b9060028210156104ff5752565b9060028110156104ff5760ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008354169116179055565b73ffffffffffffffffffffffffffffffffffffffff811690811561031757610b7b906110df565b15610a7c57805f52600360205260ff60405f2054169060028310156104ff5760028210156104ff57828214610be557805f526003602052610bbf8360405f20610b1d565b7fa09388b78c1a36296c3bc4cc7a25b5768b1d9e0e628ed553e26c26aaedf946225f80a4565b505050565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610c2b57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b67ffffffffffffffff8111610c2b5760051b60200190565b90610c7a82610c58565b610c876040519182610bea565b8281527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0610cb58294610c58565b0190602036910137565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8114610cec5760010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b8051821015610d2d5760209160051b010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b9081602091031261016b5751801515810361016b5790565b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b73ffffffffffffffffffffffffffffffffffffffff610de49593816060941683521660208201528160408201520191610d72565b90565b60015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5491949390929173ffffffffffffffffffffffffffffffffffffffff168015611089575f9586915b73ffffffffffffffffffffffffffffffffffffffff8116801561107c57805f52600360205260ff60405f20541660028110156104ff5785918791610ff05790602091858a610ed5604051968795869485947f7a3979dc00000000000000000000000000000000000000000000000000000000865260048601610db0565b03915afa908115610fe5575f91610fb7575b5015610f7e57610ef6906113c9565b90610e5857505090919293945b81610f75575b50610f1657505050600190565b73ffffffffffffffffffffffffffffffffffffffff9291610f71916040519485947ff59be56c000000000000000000000000000000000000000000000000000000008652166004850152604060248501526044840191610d72565b0390fd5b9050155f610f09565b8590610f7185876040519485947f0cecaaea00000000000000000000000000000000000000000000000000000000865260048601610db0565b610fd8915060203d8111610fde575b610fd08183610bea565b810190610d5a565b5f610ee7565b503d610fc6565b6040513d5f823e3d90fd5b9450602090600195858a611033604051968795869485947f7a3979dc00000000000000000000000000000000000000000000000000000000865260048601610db0565b03915afa908115610fe5575f9161105e575b50611054575b610ef6906113c9565b919650869161104b565b611076915060203d8111610fde57610fd08183610bea565b5f611045565b5050509091929394610f03565b5060019450505050565b73ffffffffffffffffffffffffffffffffffffffff5f541633036110b357565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff16805f52600260205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615806111a2575b1561119c5760015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff160361119857600190565b5f90565b50600190565b50805f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615611129565b60010173ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f205416158061126a575b15611263575f805260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2054169116145f1461119857600190565b5050600190565b5073ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615611223565b73ffffffffffffffffffffffffffffffffffffffff8116801580156113b7575b6113b1575f90815260026020818152604080842084805280835281852080546001808852848820805473ffffffffffffffffffffffffffffffffffffffff908116808b52898952878b208b80528952878b208054929095167fffffffffffffffffffffffff00000000000000000000000000000000000000009283168117909555938a52978752858920828a5287529490972080548716909117905580548516905590915280549091169055547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8101908111610cec5760015590565b50505f90565b506113c38260016111d7565b156112d4565b6113d48160016111d7565b6113df57505f905f90565b73ffffffffffffffffffffffffffffffffffffffff165f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f205416908115159190565b6114358160016111d7565b1580611519575b61144557505f90565b7f6ee3efecae883df2d7ccda22610b4ca771a299e707cb0d65c4ec97dc4e6668ad805473ffffffffffffffffffffffffffffffffffffffff9283165f818152600260208181526040808420600180865281845282862080547fffffffffffffffffffffffff000000000000000000000000000000000000000090811690915589548116881790995598909616808552928252808420978452968152868320805487169094179093558180529290915292909220805490911690911790555b60015460018101809111610cec57600155600190565b506115255f60016111d7565b61143c565b6115358160016111d7565b1580611602575b61154557505f90565b7f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d805473ffffffffffffffffffffffffffffffffffffffff9283165f81815260026020818152604080842084805280835281852080547fffffffffffffffffffffffff00000000000000000000000000000000000000009081169091558854811687179098559790951680845291815284832083805281528483208054871690941790935560018252949091522080549091169091179055611503565b5061160e5f60016111d7565b61153c56608034606057601f6101e938819003918201601f19168301916001600160401b038311848410176064578084926020946040528339810103126060575180151580910360605760ff80195f54169116175f5560405161017090816100798239f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610012575f80fd5b5f3560e01c80632dcfafd7146100bf57637a3979dc14610030575f80fd5b346100bb5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100bb5761006761012a565b5061007061014d565b5060443567ffffffffffffffff81116100bb57366023820112156100bb57806004013567ffffffffffffffff81116100bb57369101602401116100bb5760209060ff5f541615158152f35b5f80fd5b346100bb5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100bb576004358015158091036100bb5760ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff005f54169116175f555f80f35b6004359073ffffffffffffffffffffffffffffffffffffffff821682036100bb57565b6024359073ffffffffffffffffffffffffffffffffffffffff821682036100bb5756
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`/W`\x01`\xFF\x19`\x0CT\x16\x17`\x0CU`\x01`\xFF\x19`\x1FT\x16\x17`\x1FUad+\x90\x81a\x004\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81b[\xDA\x97\x14a>pWP\x80c\n\x92T\xE4\x14a:\x9CW\x80c\x0C\"\xBD\xF8\x14a76W\x80c\x13\xA7\xF3\x0E\x14a5[W\x80c\x18V?\xCA\x14a54W\x80c\x1Du\x97\x13\x14a3iW\x80c\x1E\xD7\x83\x1C\x14a2\xEBW\x80c%8\x8D\x16\x14a0\xC2W\x80c*\xDE8\x80\x14a.\xCEW\x80c1_A\x18\x14a.\xA7W\x80c2\xEEh\x9A\x14a+\xAAW\x80c4\xD9#\xDB\x14a+^W\x80c9\xFE9\x8D\x14a)tW\x80c>Lh\xAE\x14a'yW\x80c>^<#\x14a&\xFBW\x80c?j\xFD\xE6\x14a&\xD1W\x80c?r\x86\xF4\x14a&SW\x80cD\x12\xDE\xEB\x14a$&W\x80cf\xD9\xA9\xA0\x14a\"\xE9W\x80ckDQ\x1A\x14a\"\xC2W\x80cp\xC2}\xFA\x14a \xE2W\x80c\x85\"l\x81\x14a XW\x80c\x85\x90!\xA4\x14a\x1D\x10W\x80c\x91j\x17\xC6\x14a\x1CfW\x80c\x94a\x92.\x14a\x1A\xAAW\x80c\x972\xE2a\x14a\x18\xDFW\x80c\x9A\xBD\xF4z\x14a\x16\x9EW\x80c\x9F\xAA\xE1\xA9\x14a\x14~W\x80c\x9F\xDA\\\xAF\x14a\x0EJW\x80c\xA9\xC4\xD1\x04\x14a\x0E#W\x80c\xAC\x17\x17\xB0\x14a\r\xFCW\x80c\xB0FO\xDC\x14a\rRW\x80c\xB5P\x8A\xA9\x14a\x0C\xC8W\x80c\xB9\xED\xB1\xAF\x14a\x0C\xA1W\x80c\xBAAO\xA6\x14a\x0C|W\x80c\xC1f\xE0C\x14a\x08\xCAW\x80c\xC2\xE9\xF2\xE4\x14a\x08\x14W\x80c\xE2\x0C\x9Fq\x14a\x07\x86W\x80c\xF7\xA4\xD6\x01\x14a\x04(W\x80c\xF8\x18\x98\x95\x14a\x026W\x80c\xF8Q\xA4@\x14a\x02\x10Wc\xFAv&\xD4\x14a\x01\xEBW_\x80\xFD[4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x04\x10W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xFBW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x088\xBB\xD4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xE2W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[\x81a\x03\xD0\x91aB\x85V[a\x02\rW\x80\xF3[`@Q=\x84\x82>=\x90\xFD[\x81a\x03\xEC\x91aB\x85V[a\x02\rW\x80_a\x03qV[PP\xFD[\x81a\x04\x05\x91aB\x85V[a\x02\rW\x80_a\x03\x04V[\x81a\x04\x1A\x91aB\x85V[a\x02\rW\x80_a\x02\xAAV[P\xFD[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x07qW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x07\\W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R`\x01`$\x82\x01R`\x01`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x07GW[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90`\x01\x83\x82\x7F\xA0\x93\x88\xB7\x8C\x1A6)l;\xC4\xCCz%\xB5v\x8B\x1D\x9E\x0Eb\x8E\xD5S\xE2l&\xAA\xED\xF9F\"\x82\x80\xA4`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07BW`D\x83\x85\x81\x93\x81\x95\x7F\x088\xBB\xD4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x07-W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x94\x81\x93\x7F!(O~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x03\xD7W\x82\x91a\x06\xFEW[P`\x02\x81\x10\x15a\x06\xD1Wa\x06~\x90aIwV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`!`\x04R\xFD[a\x07 \x91P` =` \x11a\x07&W[a\x07\x18\x81\x83aB\x85V[\x81\x01\x90aD\tV[_a\x06kV[P=a\x07\x0EV[\x81a\x077\x91aB\x85V[a\x02\rW\x80_a\x06\x0BV[PPP\xFD[\x81a\x07Q\x91aB\x85V[a\x02\rW\x80_a\x05xV[\x81a\x07f\x91aB\x85V[a\x02\rW\x80_a\x04\xF0V[\x81a\x07{\x91aB\x85V[a\x02\rW\x80_a\x04\x9CV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x07\xF5Wa\x07\xF1\x85a\x07\xE5\x81\x87\x03\x82aB\x85V[`@Q\x91\x82\x91\x82a@NV[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x07\xCEV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03\xD7W\x82\x90a\x08\x8BW[a\x08\x88\x91P`\x01`\x01`\xA0\x1B\x03` T\x16\x90aJ\xDBV[\x80\xF3[P` \x81=` \x11a\x08\xC2W[\x81a\x08\xA5` \x93\x83aB\x85V[\x81\x01\x03\x12a\x08\xBEWa\x08\xB9a\x08\x88\x91aC V[a\x08qV[P\x80\xFD[=\x91Pa\x08\x98V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x0CgW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a\t\xA9`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x0CRW[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7\x83\x80\xA2sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a\nh`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x0C=W[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x82\x81\x7F\xAB\x1B\xFC\x94fC\xFE\xF3\xADND\xE40'\xBCZ\xCF\x93\xBA\xAF\xA5E\xC3\xED\x03pv\x1DK\xB4%\xDA\x82\x80\xA3`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07BW`D\x83\x85\x81\x93\x81\x95c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x0C(W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81`\x04\x81\x85Z\xFA\x80\x15a\x0C\x1DWa\x0Bp`\x01`\x01`\xA0\x1B\x03\x91` \x93\x86\x91a\x0B\xFBW[Pa\x0Bk\x81QaIwV[aC\xBFV[Q\x16a\x0B\x88`\x01`\x01`\xA0\x1B\x03`#T\x16\x80\x92aJ\xDBV[`$`@Q\x80\x94\x81\x93\x7F!(O~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x03\xD7W\x82\x91a\x0B\xDCW[P`\x02\x81\x10\x15a\x06\xD1Wa\x06~\x90aI\xEEV[a\x0B\xF5\x91P` =` \x11a\x07&Wa\x07\x18\x81\x83aB\x85V[_a\x0B\xC9V[a\x0C\x17\x91P=\x80\x88\x83>a\x0C\x0F\x81\x83aB\x85V[\x81\x01\x90aC\x99V[_a\x0B`V[`@Q=\x85\x82>=\x90\xFD[\x81a\x0C2\x91aB\x85V[a\x02\rW\x80_a\x0B\x05V[\x81a\x0CG\x91aB\x85V[a\x02\rW\x80_a\n\x8DV[\x81a\x0C\\\x91aB\x85V[a\x02\rW\x80_a\t\xCEV[\x81a\x0Cq\x91aB\x85V[a\x02\rW\x80_a\t>V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` a\x0C\x97aH\"V[`@Q\x90\x15\x15\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x19Ta\x0C\xE5\x81aC\x08V[\x91a\x0C\xF3`@Q\x93\x84aB\x85V[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\r5W`@Q\x80a\x07\xF1\x87\x82aA(V[`\x01` \x81\x92a\rD\x85aD!V[\x81R\x01\x92\x01\x92\x01\x91\x90a\r V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x1CTa\ro\x81aC\x08V[\x91a\r}`@Q\x93\x84aB\x85V[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\r\xBFW`@Q\x80a\x07\xF1\x87\x82aA\xA5V[`\x02` `\x01\x92`@Qa\r\xD2\x81aB<V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\r\xEA\x85\x87\x01aEDV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\r\xAAV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x14iW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x14TW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x14?W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x04%W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F-\xCF\xAF\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x14*W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`&T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x14\x15W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x14\0W[PPa\x10\xE5` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x10\xAC\x85\x85aB\x85V[_\x84R`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01aB\xDEV[\x03\x91Z\xFA\x80\x15a\x03\xD7Wa\x11\0\x91\x83\x91a\x13\xE1W[PaH\xFBV[\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x13\xCCW[P`\x01`\x01`\xA0\x1B\x03`&T\x16\x80;\x15a\x04%W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F-\xCF\xAF\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x13\xB7W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x13\xA2W[P`\x01`\x01`\xA0\x1B\x03`!T\x16a\x12\x92`@Q\x91a\x12+` \x84aB\x85V[_\x83Ra\x12f`@Q\x93\x84\x92\x7F\xF5\x9B\xE5l\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x85\x01R`$\x84\x01aE$V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82aB\x85V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W\x81a\x12\xED\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a@\x90V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x13\x8DW[PPa\x13O` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x10\xAC\x85\x85aB\x85V[\x03\x91Z\xFA\x80\x15a\x03\xD7Wa\x13aWP\x80\xF3[a\x13\x82\x90` =` \x11a\x13\x86W[a\x13z\x81\x83aB\x85V[\x81\x01\x90aB\xC6V[P\x80\xF3[P=a\x13pV[\x81a\x13\x97\x91aB\x85V[a\x02\rW\x80_a\x13\x12V[\x81a\x13\xAC\x91aB\x85V[a\x02\rW\x80_a\x12\x0CV[\x81a\x13\xC1\x91aB\x85V[a\x02\rW\x80_a\x11\xB9V[\x81a\x13\xD6\x91aB\x85V[a\x02\rW\x80_a\x11cV[a\x13\xFA\x91P` =` \x11a\x13\x86Wa\x13z\x81\x83aB\x85V[_a\x10\xFAV[\x81a\x14\n\x91aB\x85V[a\x02\rW\x80_a\x10oV[\x81a\x14\x1F\x91aB\x85V[a\x02\rW\x80_a\x10\x1CV[\x81a\x144\x91aB\x85V[a\x02\rW\x80_a\x0F\xC2V[\x81a\x14I\x91aB\x85V[a\x02\rW\x80_a\x0FlV[\x81a\x14^\x91aB\x85V[a\x02\rW\x80_a\x0F\x12V[\x81a\x14s\x91aB\x85V[a\x02\rW\x80_a\x0E\xBEV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x16\x89W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x16tW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x16_W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x16JW[PPa\x160` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x10\xAC\x85\x85aB\x85V[\x03\x91Z\xFA\x80\x15a\x03\xD7Wa\x08\x88\x91\x83\x91a\x13\xE1WPaH\xFBV[\x81a\x16T\x91aB\x85V[a\x02\rW\x80_a\x15\xF3V[\x81a\x16i\x91aB\x85V[a\x02\rW\x80_a\x15\xA0V[\x81a\x16~\x91aB\x85V[a\x02\rW\x80_a\x15FV[\x81a\x16\x93\x91aB\x85V[a\x02\rW\x80_a\x14\xF2V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x18\xCAW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x18\xB5W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xA2\xD8j\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x18\xA0W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xE2WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[\x81a\x18\xAA\x91aB\x85V[a\x02\rW\x80_a\x17\xF8V[\x81a\x18\xBF\x91aB\x85V[a\x02\rW\x80_a\x17fV[\x81a\x18\xD4\x91aB\x85V[a\x02\rW\x80_a\x17\x12V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x1A\x95W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x1A\x80W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x04%W\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x81`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xE2WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[\x81a\x1A\x8A\x91aB\x85V[a\x02\rW\x80_a\x19\xE5V[\x81a\x1A\x9F\x91aB\x85V[a\x02\rW\x80_a\x19SV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x1CQW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x1C<W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x13\xB7WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x13\xA2WP`\x01`\x01`\xA0\x1B\x03`!T\x16a\x12\x92`@Q\x91a\x12+` \x84aB\x85V[\x81a\x1CF\x91aB\x85V[a\x02\rW\x80_a\x1BrV[\x81a\x1C[\x91aB\x85V[a\x02\rW\x80_a\x1B\x1EV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x1DTa\x1C\x83\x81aC\x08V[\x91a\x1C\x91`@Q\x93\x84aB\x85V[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x1C\xD3W`@Q\x80a\x07\xF1\x87\x82aA\xA5V[`\x02` `\x01\x92`@Qa\x1C\xE6\x81aB<V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x1C\xFE\x85\x87\x01aEDV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x1C\xBEV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa CW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa .W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa \x19W[P`\x04\x90`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xA2\x13\xFD\"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x03\xD7W\x82\x90\x83\x92a\x1FAW[P`\x01`\x01`\xA0\x1B\x03a\x1E\xC4\x82a\x1E\x9Ba\x1E\xD8\x94QaJdV[a\x1E\xA5\x85QaJdV[a\x1E\xBF\x83a\x1E\xB2\x83aC\xBFV[Q\x16\x84`$T\x16\x90aJ\xDBV[aC\xF9V[Q\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x90aJ\xDBV[a\x1E\xE9a\x1E\xE4\x82aC\xBFV[aG\xE8V[`\x02\x81\x10\x15a\x1F\x14Wa\x1F\x02\x91a\x1E\xBFa\x1E\xE4\x92aIwV[`\x02\x81\x10\x15a\x06\xD1Wa\x06~\x90aI\xEEV[`$\x83\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`!`\x04R\xFD[\x91PP=\x80\x83\x83>a\x1FS\x81\x83aB\x85V[\x81\x01\x90`@\x81\x83\x03\x12a \x15W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a \rW\x82a\x1F}\x91\x83\x01aC4V[\x90` \x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a \x11W\x01\x91\x80`\x1F\x84\x01\x12\x15a \rW\x82Q\x90a\x1F\xAC\x82aC\x08V[\x93a\x1F\xBA`@Q\x95\x86aB\x85V[\x82\x85R` \x80\x86\x01\x93`\x05\x1B\x82\x01\x01\x91\x82\x11a \tW` \x01\x91[\x81\x83\x10a\x1F\xECWPPP`\x01`\x01`\xA0\x1B\x03a\x1E\x81V[\x82Q`\x02\x81\x10\x15a \x05W\x81R` \x92\x83\x01\x92\x01a\x1F\xD5V[\x86\x80\xFD[\x85\x80\xFD[\x83\x80\xFD[\x84\x80\xFD[\x82\x80\xFD[\x81a #\x91aB\x85V[a\x02\rW\x80_a\x1E2V[\x81a 8\x91aB\x85V[a\x02\rW\x80_a\x1D\xD8V[\x81a M\x91aB\x85V[a\x02\rW\x80_a\x1D\x84V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x1ATa u\x81aC\x08V[\x91a \x83`@Q\x93\x84aB\x85V[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a \xC5W`@Q\x80a\x07\xF1\x87\x82aA(V[`\x01` \x81\x92a \xD4\x85aD!V[\x81R\x01\x92\x01\x92\x01\x91\x90a \xB0V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\"\xADW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\"\x98W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x16_WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x16JWPPa\x160` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x10\xAC\x85\x85aB\x85V[\x81a\"\xA2\x91aB\x85V[a\x02\rW\x80_a!\xB0V[\x81a\"\xB7\x91aB\x85V[a\x02\rW\x80_a!VV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x1BTa#\x06\x81aC\x08V[a#\x13`@Q\x91\x82aB\x85V[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a#\xEBW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a#\x80WPPPP\x03\x90\xF3[\x91\x93` a#\xDB\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a#\xCB\x83Q`@\x84R`@\x84\x01\x90a@\x90V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra@\xD3V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a#qV[`\x02` `\x01\x92`@Qa#\xFE\x81aB<V[a$\x07\x86aD!V[\x81Ra$\x14\x85\x87\x01aEDV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a#CV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa&>W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa&)W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x04%W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F-\xCF\xAF\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa&\x14W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x13\xB7WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x13\xA2WP`\x01`\x01`\xA0\x1B\x03`!T\x16a\x12\x92`@Q\x91a\x12+` \x84aB\x85V[\x81a&\x1E\x91aB\x85V[a\x02\rW\x80_a%JV[\x81a&3\x91aB\x85V[a\x02\rW\x80_a$\xF4V[\x81a&H\x91aB\x85V[a\x02\rW\x80_a$\x9AV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a&\xB2Wa\x07\xF1\x85a\x07\xE5\x81\x87\x03\x82aB\x85V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a&\x9BV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a'ZWa\x07\xF1\x85a\x07\xE5\x81\x87\x03\x82aB\x85V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a'CV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa)_W[P`\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01R`$\x81Ra(8`D\x82aB\x85V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W\x81a(\x93\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a@\x90V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x18\xA0WP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xE2WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[\x81a)i\x91aB\x85V[a\x02\rW\x80_a'\xEDV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa+IW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa+4W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xE2WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[\x81a+>\x91aB\x85V[a\x02\rW\x80_a*zV[\x81a+S\x91aB\x85V[a\x02\rW\x80_a)\xE8V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rWa\x160` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x10\xAC\x85\x85aB\x85V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa.\x92W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa.}W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R\x81`$\x82\x01R\x81`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa.hW[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x80\x7F\xB5\xD6\x8C\xA4cr\xBB\xE6\xEC\x13\x8D=\x04#`\x82i\xB3\x11t\x96\xA4bh\xF8`\x80\xCD\xBC\xBE\xA9\xBE\x84\x80\xA2`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07BW`$\x83\x85\x81\x93\x81\x95\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa.SW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81`\x04\x81\x85Z\xFA\x80\x15a\x0C\x1DWa-\xD8\x91\x84\x91a.9W[PQaI\xEEV[` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x94\x81\x93\x7F!(O~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x03\xD7W\x82\x91a\x0B\xDCWP`\x02\x81\x10\x15a\x06\xD1Wa\x06~\x90aI\xEEV[a.M\x91P=\x80\x86\x83>a\x0C\x0F\x81\x83aB\x85V[_a-\xD1V[\x81a.]\x91aB\x85V[a\x02\rW\x80_a-\x81V[\x81a.r\x91aB\x85V[a\x02\rW\x80_a,\xF8V[\x81a.\x87\x91aB\x85V[a\x02\rW\x80_a,rV[\x81a.\x9C\x91aB\x85V[a\x02\rW\x80_a,\x1EV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x1ETa.\xEB\x81aC\x08V[a.\xF8`@Q\x91\x82aB\x85V[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a09W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a/dW\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a/\xF0WPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a/WV[\x90\x91\x92\x93\x94` \x80a0,\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa@\x90V[\x97\x01\x95\x01\x93\x92\x91\x01a/\xCCV[`@Qa0E\x81aB<V[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta0a\x81aC\x08V[\x91a0o`@Q\x93\x84aB\x85V[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a0\xA5WPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a/(V[`\x01` \x81\x92a0\xB4\x86aD!V[\x81R\x01\x93\x01\x91\x01\x90\x91a0\x7FV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa2\xD6W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa2\xC1W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa2\xACW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa2\x97W[P`\x01`\x01`\xA0\x1B\x03`$T\x16a\x12\x92`\x01`\x01`\xA0\x1B\x03`!T\x16\x91a\x12f`@Qa2_` \x82aB\x85V[_\x81R`@Q\x94\x85\x93\x7F\x0C\xEC\xAA\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x86\x01R`$\x85\x01aB\xDEV[\x81a2\xA1\x91aB\x85V[a\x02\rW\x80_a21V[\x81a2\xB6\x91aB\x85V[a\x02\rW\x80_a1\xDEV[\x81a2\xCB\x91aB\x85V[a\x02\rW\x80_a1\x8AV[\x81a2\xE0\x91aB\x85V[a\x02\rW\x80_a16V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a3JWa\x07\xF1\x85a\x07\xE5\x81\x87\x03\x82aB\x85V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a33V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa5\x1FW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa5\nW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa2\xACWPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa2\x97WP`\x01`\x01`\xA0\x1B\x03`$T\x16a\x12\x92`\x01`\x01`\xA0\x1B\x03`!T\x16\x91a\x12f`@Qa2_` \x82aB\x85V[\x81a5\x14\x91aB\x85V[a\x02\rW\x80_a41V[\x81a5)\x91aB\x85V[a\x02\rW\x80_a3\xDDV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`&T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa7!W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xFBWP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x088\xBB\xD4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xE2WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[\x81a7+\x91aB\x85V[a\x02\rW\x80_a5\xCFV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa:\x87W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a8\x15`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa:rW[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7\x83\x80\xA2sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a8\xD4`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa:]W[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90`\x01\x81\x7F\xAB\x1B\xFC\x94fC\xFE\xF3\xADND\xE40'\xBCZ\xCF\x93\xBA\xAF\xA5E\xC3\xED\x03pv\x1DK\xB4%\xDA\x85\x80\xA3`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07BW`d\x83\x85\x81\x93\x81\x95bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa:HW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81`\x04\x81\x85Z\xFA\x80\x15a\x0C\x1DWa9\xDD`\x01`\x01`\xA0\x1B\x03\x91` \x93\x86\x91a\x0B\xFBWPa\x0Bk\x81QaIwV[Q\x16a9\xF5`\x01`\x01`\xA0\x1B\x03`#T\x16\x80\x92aJ\xDBV[`$`@Q\x80\x94\x81\x93\x7F!(O~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x03\xD7W\x82\x91a\x06\xFEWP`\x02\x81\x10\x15a\x06\xD1Wa\x06~\x90aIwV[\x81a:R\x91aB\x85V[a\x02\rW\x80_a9xV[\x81a:g\x91aB\x85V[a\x02\rW\x80_a8\xF9V[\x81a:|\x91aB\x85V[a\x02\rW\x80_a8:V[\x81a:\x91\x91aB\x85V[a\x02\rW\x80_a7\xAAV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U`\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"T\x16\x17`\"Usq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa>[W[PP`\x01`\x01`\xA0\x1B\x03` T\x16`@Q\x90a\x16\xE4\x90\x81\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a>\x01W\x91\x83\x91` \x93aK^\x849\x81R\x03\x01\x90\x82\xF0\x80\x15a=\xF4W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU`@Qa\x01\xE9\x90\x81\x81\x01\x90\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a>.W` \x81abB\x93\x85\x85\x839`\x01\x81R\x03\x01\x90\x84\xF0\x80\x15a\x0C\x1DW`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`#T\x16\x17`#U`@Q\x82\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a>\x01W\x81` \x91\x85\x85\x839\x86\x81R\x03\x01\x90\x84\xF0\x80\x15a\x0C\x1DW`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$T\x16\x17`$U`@Q\x82\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a>\x01W\x81` \x91\x85\x85\x839`\x01\x81R\x03\x01\x90\x84\xF0\x80\x15a\x0C\x1DW`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`%T\x16\x17`%U`@Q\x91\x80\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a>\x01W\x91\x83\x91` \x93\x839`\x01\x81R\x03\x01\x90\x82\xF0\x80\x15a=\xF4W`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`&T\x16\x17`&Usq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81a>e\x91aB\x85V[a\x02\rW\x80_a;\x86V[\x90P4a@JW_`\x03\x196\x01\x12a@JW`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a@JWc\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a@?Wa@,W[P\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa@\x17W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x16_WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x16JWPPa\x160` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x10\xAC\x85\x85aB\x85V[\x81a@!\x91aB\x85V[a\x02\rW\x80_a?5V[a@8\x91P_\x90aB\x85V[__a>\xE0V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a@qWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a@dV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a@\xF0WPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a@\xE3V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aAZWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aA\x96\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa@\x90V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aAKV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aA\xD7WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aB-\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a@\xD3V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aA\xC8V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aBXW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aBXW`@RV[\x90\x81` \x91\x03\x12a@JWQ\x80\x15\x15\x81\x03a@JW\x90V[`\x01`\x01`\xA0\x1B\x03aC\x05\x94\x93\x81``\x94\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a@\x90V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11aBXW`\x05\x1B` \x01\x90V[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a@JWV[\x90\x80`\x1F\x83\x01\x12\x15a@JW\x81QaCK\x81aC\x08V[\x92aCY`@Q\x94\x85aB\x85V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a@JW` \x01\x90[\x82\x82\x10aC\x81WPPP\x90V[` \x80\x91aC\x8E\x84aC V[\x81R\x01\x91\x01\x90aCtV[\x90` \x82\x82\x03\x12a@JW\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a@JWaC\x05\x92\x01aC4V[\x80Q\x15aC\xCCW` \x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80Q`\x01\x10\x15aC\xCCW`@\x01\x90V[\x90\x81` \x91\x03\x12a@JWQ`\x02\x81\x10\x15a@JW\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15aE\x1AW[` \x85\x10\x84\x14aD\xEDW\x84\x87R\x86\x93\x90\x81\x15aD\xADWP`\x01\x14aDiW[PaDg\x92P\x03\x83aB\x85V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10aD\x91WPP\x90` aDg\x92\x82\x01\x01_aDZV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92aDxV[` \x93PaDg\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_aDZV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93aD;V[`@\x90`\x01`\x01`\xA0\x1B\x03aC\x05\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a@\x90V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10aG[WaDg\x94T\x91\x81\x81\x10aG%W[\x81\x81\x10aF\xEFW[\x81\x81\x10aF\xB9W[\x81\x81\x10aF\x83W[\x81\x81\x10aFMW[\x81\x81\x10aF\x17W[\x81\x81\x10aE\xE2W[\x10aE\xB5W[P\x03\x83aB\x85V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_aE\xADV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01aE\xA7V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01aE\x9FV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01aE\x97V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01aE\x8FV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01aE\x87V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01aE\x7FV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01aEwV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91aE_V[Q`\x02\x81\x10\x15aG\xF5W\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x08T`\xFF\x16\x80\x15aH1W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a@?W_\x91aH\xC9W[P\x15\x15\x90V[\x90P` \x81=` \x11aH\xF3W[\x81aH\xE4` \x93\x83aB\x85V[\x81\x01\x03\x12a@JWQ_aH\xC3V[=\x91PaH\xD7V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a@JW`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a@?WaImWPV[_aDg\x91aB\x85V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a@JW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x01`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a@?WaImWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a@JW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a@?WaImWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a@JW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x02`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a@?WaImWPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a@JW`\x01`\x01`\xA0\x1B\x03\x90\x81`@Q\x93\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R\x16`\x04\x84\x01R\x16`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a@?WaImWPV\xFE`\x804`\xB8W`\x1Fa\x16\xE48\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`\xBCW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`\xB8WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03`\xB8W\x80\x15`\xA5W_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x82U`@Q\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3a\x16\x13\x90\x81a\0\xD1\x829\xF3[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x04\xF3\x86\xF4\x14a\tJW\x80c\x05.\xEF\xD1\x14a\x08>W\x80c\x088\xBB\xD4\x14a\x08\x06W\x80c\x1BB\xC7\x11\x14a\x06\xE9W\x80c!(O~\x14a\x06\x9AW\x80cqP\x18\xA6\x14a\x06\x1EW\x80cz9y\xDC\x14a\x05\x8BW\x80c\x8D\xA5\xCB[\x14a\x05YW\x80c\xA2\x13\xFD\"\x14a\x03ZW\x80c\xA2kJ\x88\x14a\x03?W\x80c\xB1\xC6\x1E\0\x14a\x01oWc\xF2\xFD\xE3\x8B\x14a\0\x9DW_\x80\xFD[4a\x01kW` `\x03\x196\x01\x12a\x01kWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\xCBa\n\xA4V[a\0\xD3a\x10\x93V[\x16\x80\x15a\x01?Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x80\xFD[4a\x01kW```\x03\x196\x01\x12a\x01kWa\x01\x88a\n\xA4V[`$5\x90`\x02\x82\x10\x15a\x01kW`D5\x90\x81\x15\x15\x82\x03a\x01kWa\x01\xAAa\x10\x93V[a\x01\xB2a\x10\x93V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x15a\x03\x17Wa\x01\xD9\x82a\x10\xDFV[a\x02\xEFW`(`\x01T\x10\x15a\x02\xC7W\x15a\x02\xB9Wa\x01\xF6\x90a\x15*V[\x15a\x02[W\x80\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7_\x80\xA2\x80_R`\x03` Ra\x025\x82`@_ a\x0B\x1DV[\x7F\xAB\x1B\xFC\x94fC\xFE\xF3\xADND\xE40'\xBCZ\xCF\x93\xBA\xAF\xA5E\xC3\xED\x03pv\x1DK\xB4%\xDA_\x80\xA3\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FAddress not added\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[a\x02\xC2\x90a\x14*V[a\x01\xF6V[\x7F\x13\xD8g\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xA2\xD8j\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01kW_`\x03\x196\x01\x12a\x01kW` `@Q`(\x81R\xF3[4a\x01kW_`\x03\x196\x01\x12a\x01kW`\x01Ta\x03v\x81a\x0CpV[\x90a\x03\x80\x81a\x0CXV[a\x03\x8D`@Q\x91\x82a\x0B\xEAV[\x81\x81Ra\x03\x99\x82a\x0CXV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0` \x83\x01\x91\x016\x827`\x01_\x90\x81R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x84\x82\x10\x80a\x05:W[\x15a\x05,Wa\x04>\x82\x87a\r\x19V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x91R_R`\x03` R`\xFF`@_ T\x16\x90a\x04r\x83\x86a\r\x19V[\x91`\x02\x81\x10\x15a\x04\xFFWa\x04\x86\x92Ra\x13\xC9V[\x90\x15a\x04\x9CWa\x04\x96\x90\x91a\x0C\xBFV[\x90a\x04&V[PP\x90a\x04\xBC\x93\x92P[` `@Q\x94\x85\x94`@\x86R`@\x86\x01\x90a\n\xC7V[\x91\x84\x83\x03\x82\x86\x01RQ\x91\x82\x81R\x01\x91\x90_[\x81\x81\x10a\x04\xDCWPPP\x03\x90\xF3[\x91\x93P\x91` \x80\x82a\x04\xF1`\x01\x94\x88Qa\x0B\x10V[\x01\x94\x01\x91\x01\x91\x84\x93\x92a\x04\xCEV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[PP\x90a\x04\xBC\x93\x92Pa\x04\xA6V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15\x15a\x04/V[4a\x01kW_`\x03\x196\x01\x12a\x01kW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\x01kW```\x03\x196\x01\x12a\x01kWa\x05\xA4a\n\xA4V[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01kW`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01kW6`#\x83\x01\x12\x15a\x01kW\x81`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01kW6`$\x83\x85\x01\x01\x11a\x01kW` \x93`$a\x06\x14\x94\x01\x91a\r\xE7V[`@Q\x90\x15\x15\x81R\xF3[4a\x01kW_`\x03\x196\x01\x12a\x01kWa\x066a\x10\x93V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01kW` `\x03\x196\x01\x12a\x01kWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x06\xC8a\n\xA4V[\x16_R`\x03` R` `\xFF`@_ T\x16a\x06\xE7`@Q\x80\x92a\x0B\x10V[\xF3[4a\x01kW_`\x03\x196\x01\x12a\x01kW`\x01Ta\x07\x05\x81a\x0CpV[`\x01_\x90\x81R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DT\x91\x92\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x82\x82\x10\x80a\x07\xE7W[\x15a\x07\xDBWa\x07\xA5\x90a\x07\x87\x83\x86a\r\x19V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90Ra\x13\xC9V[\x90\x15a\x07\xBBWa\x07\xB5\x90\x91a\x0C\xBFV[\x90a\x07kV[PPPa\x07\xD7\x90[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\n\xC7V[\x03\x90\xF3[PPPa\x07\xD7\x90a\x07\xC3V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15\x15a\x07tV[4a\x01kW`@`\x03\x196\x01\x12a\x01kWa\x08\x1Fa\n\xA4V[`$5`\x02\x81\x10\x15a\x01kWa\x08<\x91a\x087a\x10\x93V[a\x0BTV[\0[4a\x01kW`@`\x03\x196\x01\x12a\x01kWa\x08Wa\n\xA4V[`$5\x90\x81\x15\x15\x82\x03a\x01kWa\x08la\x10\x93V[a\x08ta\x10\x93V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x15a\x03\x17Wa\x08\x9B\x82a\x10\xDFV[a\x02\xEFW`(`\x01T\x10\x15a\x02\xC7W\x15a\t<Wa\x08\xB8\x90a\x15*V[\x15a\x02[W\x80_\x91\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7\x83\x80\xA2\x80\x82R`\x03` R`@\x82 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90U\x7F\xAB\x1B\xFC\x94fC\xFE\xF3\xADND\xE40'\xBCZ\xCF\x93\xBA\xAF\xA5E\xC3\xED\x03pv\x1DK\xB4%\xDA\x82\x80\xA3\0[a\tE\x90a\x14*V[a\x08\xB8V[4a\x01kW` `\x03\x196\x01\x12a\x01kWa\tca\n\xA4V[a\tka\x10\x93V[a\tsa\x10\x93V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a\x03\x17Wa\t\x9A\x81a\x10\xDFV[\x15a\n|Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\t\xBE\x83\x92a\x12\xB4V[\x16\x03a\n\x1EW\x80\x7F\xB5\xD6\x8C\xA4cr\xBB\xE6\xEC\x13\x8D=\x04#`\x82i\xB3\x11t\x96\xA4bh\xF8`\x80\xCD\xBC\xBE\xA9\xBE_\x80\xA2_R`\x03` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90U_\x80\xF3[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FAddress not removed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01kWV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\n\xE4WPPP\x90V[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n\xD7V[\x90`\x02\x82\x10\x15a\x04\xFFWRV[\x90`\x02\x81\x10\x15a\x04\xFFW`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83T\x16\x91\x16\x17\x90UV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a\x03\x17Wa\x0B{\x90a\x10\xDFV[\x15a\n|W\x80_R`\x03` R`\xFF`@_ T\x16\x90`\x02\x83\x10\x15a\x04\xFFW`\x02\x82\x10\x15a\x04\xFFW\x82\x82\x14a\x0B\xE5W\x80_R`\x03` Ra\x0B\xBF\x83`@_ a\x0B\x1DV[\x7F\xA0\x93\x88\xB7\x8C\x1A6)l;\xC4\xCCz%\xB5v\x8B\x1D\x9E\x0Eb\x8E\xD5S\xE2l&\xAA\xED\xF9F\"_\x80\xA4V[PPPV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C+W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0C+W`\x05\x1B` \x01\x90V[\x90a\x0Cz\x82a\x0CXV[a\x0C\x87`@Q\x91\x82a\x0B\xEAV[\x82\x81R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0a\x0C\xB5\x82\x94a\x0CXV[\x01\x90` 6\x91\x017V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x0C\xECW`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80Q\x82\x10\x15a\r-W` \x91`\x05\x1B\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x90\x81` \x91\x03\x12a\x01kWQ\x80\x15\x15\x81\x03a\x01kW\x90V[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\r\xE4\x95\x93\x81``\x94\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x91a\rrV[\x90V[`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DT\x91\x94\x93\x90\x92\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a\x10\x89W_\x95\x86\x91[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15a\x10|W\x80_R`\x03` R`\xFF`@_ T\x16`\x02\x81\x10\x15a\x04\xFFW\x85\x91\x87\x91a\x0F\xF0W\x90` \x91\x85\x8Aa\x0E\xD5`@Q\x96\x87\x95\x86\x94\x85\x94\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\x04\x86\x01a\r\xB0V[\x03\x91Z\xFA\x90\x81\x15a\x0F\xE5W_\x91a\x0F\xB7W[P\x15a\x0F~Wa\x0E\xF6\x90a\x13\xC9V[\x90a\x0EXWPP\x90\x91\x92\x93\x94[\x81a\x0FuW[Pa\x0F\x16WPPP`\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x91a\x0Fq\x91`@Q\x94\x85\x94\x7F\xF5\x9B\xE5l\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x16`\x04\x85\x01R`@`$\x85\x01R`D\x84\x01\x91a\rrV[\x03\x90\xFD[\x90P\x15_a\x0F\tV[\x85\x90a\x0Fq\x85\x87`@Q\x94\x85\x94\x7F\x0C\xEC\xAA\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\x04\x86\x01a\r\xB0V[a\x0F\xD8\x91P` =\x81\x11a\x0F\xDEW[a\x0F\xD0\x81\x83a\x0B\xEAV[\x81\x01\x90a\rZV[_a\x0E\xE7V[P=a\x0F\xC6V[`@Q=_\x82>=\x90\xFD[\x94P` \x90`\x01\x95\x85\x8Aa\x103`@Q\x96\x87\x95\x86\x94\x85\x94\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\x04\x86\x01a\r\xB0V[\x03\x91Z\xFA\x90\x81\x15a\x0F\xE5W_\x91a\x10^W[Pa\x10TW[a\x0E\xF6\x90a\x13\xC9V[\x91\x96P\x86\x91a\x10KV[a\x10v\x91P` =\x81\x11a\x0F\xDEWa\x0F\xD0\x81\x83a\x0B\xEAV[_a\x10EV[PPP\x90\x91\x92\x93\x94a\x0F\x03V[P`\x01\x94PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\x10\xB3WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80_R`\x02` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x11\xA2W[\x15a\x11\x9CW`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x11\x98W`\x01\x90V[_\x90V[P`\x01\x90V[P\x80_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\x11)V[`\x01\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x12jW[\x15a\x12cW_\x80R` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ T\x16\x91\x16\x14_\x14a\x11\x98W`\x01\x90V[PP`\x01\x90V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\x12#V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x80\x15a\x13\xB7W[a\x13\xB1W_\x90\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T`\x01\x80\x88R\x84\x88 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x8BR\x89\x89R\x87\x8B \x8B\x80R\x89R\x87\x8B \x80T\x92\x90\x95\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81\x17\x90\x95U\x93\x8AR\x97\x87R\x85\x89 \x82\x8AR\x87R\x94\x90\x97 \x80T\x87\x16\x90\x91\x17\x90U\x80T\x85\x16\x90U\x90\x91R\x80T\x90\x91\x16\x90UT\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a\x0C\xECW`\x01U\x90V[PP_\x90V[Pa\x13\xC3\x82`\x01a\x11\xD7V[\x15a\x12\xD4V[a\x13\xD4\x81`\x01a\x11\xD7V[a\x13\xDFWP_\x90_\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x90\x81\x15\x15\x91\x90V[a\x145\x81`\x01a\x11\xD7V[\x15\x80a\x15\x19W[a\x14EWP_\x90V[\x7Fn\xE3\xEF\xEC\xAE\x88=\xF2\xD7\xCC\xDA\"a\x0BL\xA7q\xA2\x99\xE7\x07\xCB\re\xC4\xEC\x97\xDCNfh\xAD\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 `\x01\x80\x86R\x81\x84R\x82\x86 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x89T\x81\x16\x88\x17\x90\x99U\x98\x90\x96\x16\x80\x85R\x92\x82R\x80\x84 \x97\x84R\x96\x81R\x86\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U\x81\x80R\x92\x90\x91R\x92\x90\x92 \x80T\x90\x91\x16\x90\x91\x17\x90U[`\x01T`\x01\x81\x01\x80\x91\x11a\x0C\xECW`\x01U`\x01\x90V[Pa\x15%_`\x01a\x11\xD7V[a\x14<V[a\x155\x81`\x01a\x11\xD7V[\x15\x80a\x16\x02W[a\x15EWP_\x90V[\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9D\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x88T\x81\x16\x87\x17\x90\x98U\x97\x90\x95\x16\x80\x84R\x91\x81R\x84\x83 \x83\x80R\x81R\x84\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U`\x01\x82R\x94\x90\x91R \x80T\x90\x91\x16\x90\x91\x17\x90Ua\x15\x03V[Pa\x16\x0E_`\x01a\x11\xD7V[a\x15<V`\x804``W`\x1Fa\x01\xE98\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`dW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12``WQ\x80\x15\x15\x80\x91\x03``W`\xFF\x80\x19_T\x16\x91\x16\x17_U`@Qa\x01p\x90\x81a\0y\x829\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x80c-\xCF\xAF\xD7\x14a\0\xBFWcz9y\xDC\x14a\x000W_\x80\xFD[4a\0\xBBW``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0\xBBWa\0ga\x01*V[Pa\0pa\x01MV[P`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xBBW6`#\x82\x01\x12\x15a\0\xBBW\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xBBW6\x91\x01`$\x01\x11a\0\xBBW` \x90`\xFF_T\x16\x15\x15\x81R\xF3[_\x80\xFD[4a\0\xBBW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0\xBBW`\x045\x80\x15\x15\x80\x91\x03a\0\xBBW`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0_T\x16\x91\x16\x17_U_\x80\xF3[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0\xBBWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0\xBBWV",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f905f3560e01c9081625bda9714613e70575080630a9254e414613a9c5780630c22bdf81461373657806313a7f30e1461355b57806318563fca146135345780631d759713146133695780631ed7831c146132eb57806325388d16146130c25780632ade388014612ece578063315f411814612ea757806332ee689a14612baa57806334d923db14612b5e57806339fe398d146129745780633e4c68ae146127795780633e5e3c23146126fb5780633f6afde6146126d15780633f7286f4146126535780634412deeb1461242657806366d9a9a0146122e95780636b44511a146122c257806370c27dfa146120e257806385226c8114612058578063859021a414611d10578063916a17c614611c665780639461922e14611aaa5780639732e261146118df5780639abdf47a1461169e5780639faae1a91461147e5780639fda5caf14610e4a578063a9c4d10414610e23578063ac1717b014610dfc578063b0464fdc14610d52578063b5508aa914610cc8578063b9edb1af14610ca1578063ba414fa614610c7c578063c166e043146108ca578063c2e9f2e414610814578063e20c9f7114610786578063f7a4d60114610428578063f818989514610236578063f851a440146102105763fa7626d4146101eb575f80fd5b3461020d578060031936011261020d57602060ff601f54166040519015158152f35b80fd5b503461020d578060031936011261020d5760206001600160a01b03815416604051908152f35b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757610410575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d7576103fb575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f75782916044839260405194859384927f0838bbd40000000000000000000000000000000000000000000000000000000084526004840152600160248401525af180156103d7576103e2575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b816103d091614285565b61020d5780f35b6040513d84823e3d90fd5b816103ec91614285565b61020d57805f610371565b5050fd5b8161040591614285565b61020d57805f610304565b8161041a91614285565b61020d57805f6102aa565b50fd5b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757610771575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d75761075c575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517f491cc7c200000000000000000000000000000000000000000000000000000000815260016004820152600160248201526001604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757610747575b506001600160a01b036023541660405190600183827fa09388b78c1a36296c3bc4cc7a25b5768b1d9e0e628ed553e26c26aaedf946228280a46001600160a01b03601f5460081c16803b156107425760448385819381957f0838bbd40000000000000000000000000000000000000000000000000000000084526004840152600160248401525af180156103d75761072d575b50506001600160a01b03601f5460081c1660206001600160a01b03602354166024604051809481937f21284f7e00000000000000000000000000000000000000000000000000000000835260048301525afa9081156103d75782916106fe575b5060028110156106d15761067e90614977565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526021600452fd5b610720915060203d602011610726575b6107188183614285565b810190614409565b5f61066b565b503d61070e565b8161073791614285565b61020d57805f61060b565b505050fd5b8161075191614285565b61020d57805f610578565b8161076691614285565b61020d57805f6104f0565b8161077b91614285565b61020d57805f61049c565b503461020d578060031936011261020d5760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106107f5576107f1856107e581870382614285565b6040519182918261404e565b0390f35b82546001600160a01b03168452602090930192600192830192016107ce565b503461020d578060031936011261020d57600460206001600160a01b03601f5460081c16604051928380927f8da5cb5b0000000000000000000000000000000000000000000000000000000082525afa80156103d757829061088b575b61088891506001600160a01b036020541690614adb565b80f35b506020813d6020116108c2575b816108a560209383614285565b810103126108be576108b961088891614320565b610871565b5080fd5b3d9150610898565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757610c67575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517f491cc7c20000000000000000000000000000000000000000000000000000000081528181806109a960048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757610c52575b506001600160a01b0360235416604051907f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e78380a2737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610425577f491cc7c2000000000000000000000000000000000000000000000000000000008152818180610a6860048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757610c3d575b506001600160a01b03602354166040519082817fab1bfc946643fef3ad4e44e43027bc5acf93baafa545c3ed0370761d4bb425da8280a36001600160a01b03601f5460081c16803b1561074257604483858193819563052eefd160e01b84526004840152600160248401525af180156103d757610c28575b50506001600160a01b03601f5460081c166040517f1b42c7110000000000000000000000000000000000000000000000000000000081528281600481855afa8015610c1d57610b706001600160a01b03916020938691610bfb575b50610b6b8151614977565b6143bf565b5116610b886001600160a01b03602354168092614adb565b6024604051809481937f21284f7e00000000000000000000000000000000000000000000000000000000835260048301525afa9081156103d7578291610bdc575b5060028110156106d15761067e906149ee565b610bf5915060203d602011610726576107188183614285565b5f610bc9565b610c1791503d8088833e610c0f8183614285565b810190614399565b5f610b60565b6040513d85823e3d90fd5b81610c3291614285565b61020d57805f610b05565b81610c4791614285565b61020d57805f610a8d565b81610c5c91614285565b61020d57805f6109ce565b81610c7191614285565b61020d57805f61093e565b503461020d578060031936011261020d576020610c97614822565b6040519015158152f35b503461020d578060031936011261020d5760206001600160a01b0360225416604051908152f35b503461020d578060031936011261020d57601954610ce581614308565b91610cf36040519384614285565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b838310610d3557604051806107f18782614128565b600160208192610d4485614421565b815201920192019190610d20565b503461020d578060031936011261020d57601c54610d6f81614308565b91610d7d6040519384614285565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310610dbf57604051806107f187826141a5565b60026020600192604051610dd28161423c565b6001600160a01b038654168152610dea858701614544565b83820152815201920192019190610daa565b503461020d578060031936011261020d5760206001600160a01b0360215416604051908152f35b503461020d578060031936011261020d5760206001600160a01b0360245416604051908152f35b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757611469575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d757611454575b506001600160a01b03601f5460081c166001600160a01b0360255416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d75761143f575b506001600160a01b0360255416803b15610425578180916024604051809481937f2dcfafd70000000000000000000000000000000000000000000000000000000083528160048401525af180156103d75761142a575b506001600160a01b03601f5460081c166001600160a01b0360265416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d757611415575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757611400575b50506110e560206001600160a01b03601f5460081c166001600160a01b0360215416906001600160a01b0360225416604051926110ac8585614285565b5f8452604051958694859384937f7a3979dc000000000000000000000000000000000000000000000000000000008552600485016142de565b03915afa80156103d7576111009183916113e1575b506148fb565b806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576113cc575b506001600160a01b0360265416803b15610425578180916024604051809481937f2dcfafd70000000000000000000000000000000000000000000000000000000083528160048401525af180156103d7576113b7575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576113a2575b506001600160a01b03602154166112926040519161122b602084614285565b5f83526112666040519384927ff59be56c00000000000000000000000000000000000000000000000000000000602085015260248401614524565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282614285565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557816112ed91604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190614090565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761138d575b505061134f60206001600160a01b03601f5460081c166001600160a01b0360215416906001600160a01b0360225416604051926110ac8585614285565b03915afa80156103d757611361575080f35b6113829060203d602011611386575b61137a8183614285565b8101906142c6565b5080f35b503d611370565b8161139791614285565b61020d57805f611312565b816113ac91614285565b61020d57805f61120c565b816113c191614285565b61020d57805f6111b9565b816113d691614285565b61020d57805f611163565b6113fa915060203d6020116113865761137a8183614285565b5f6110fa565b8161140a91614285565b61020d57805f61106f565b8161141f91614285565b61020d57805f61101c565b8161143491614285565b61020d57805f610fc2565b8161144991614285565b61020d57805f610f6c565b8161145e91614285565b61020d57805f610f12565b8161147391614285565b61020d57805f610ebe565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757611689575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d757611674575b506001600160a01b03601f5460081c166001600160a01b0360255416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d75761165f575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761164a575b505061163060206001600160a01b03601f5460081c166001600160a01b0360215416906001600160a01b0360225416604051926110ac8585614285565b03915afa80156103d7576108889183916113e157506148fb565b8161165491614285565b61020d57805f6115f3565b8161166991614285565b61020d57805f6115a0565b8161167e91614285565b61020d57805f611546565b8161169391614285565b61020d57805f6114f2565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576118ca575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d7576118b5575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fa2d86a1e000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576118a0575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d7576103e2575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b816118aa91614285565b61020d57805f6117f8565b816118bf91614285565b61020d57805f611766565b816118d491614285565b61020d57805f611712565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757611a95575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fe6c4247b000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757611a80575b506001600160a01b03601f5460081c16803b156104255781809160446040518094819363052eefd160e01b8352816004840152600160248401525af180156103d7576103e2575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b81611a8a91614285565b61020d57805f6119e5565b81611a9f91614285565b61020d57805f611953565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757611c51575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d757611c3c575b506001600160a01b03601f5460081c166001600160a01b0360245416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d7576113b7575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576113a257506001600160a01b03602154166112926040519161122b602084614285565b81611c4691614285565b61020d57805f611b72565b81611c5b91614285565b61020d57805f611b1e565b503461020d578060031936011261020d57601d54611c8381614308565b91611c916040519384614285565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310611cd357604051806107f187826141a5565b60026020600192604051611ce68161423c565b6001600160a01b038654168152611cfe858701614544565b83820152815201920192019190611cbe565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757612043575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d75761202e575b506001600160a01b03601f5460081c166001600160a01b0360245416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d757612019575b506004906001600160a01b03601f5460081c16604051928380927fa213fd220000000000000000000000000000000000000000000000000000000082525afa9081156103d75782908392611f41575b506001600160a01b03611ec482611e9b611ed89451614a64565b611ea58551614a64565b611ebf83611eb2836143bf565b5116846024541690614adb565b6143f9565b51166001600160a01b036023541690614adb565b611ee9611ee4826143bf565b6147e8565b6002811015611f1457611f0291611ebf611ee492614977565b60028110156106d15761067e906149ee565b6024837f4e487b710000000000000000000000000000000000000000000000000000000081526021600452fd5b9150503d8083833e611f538183614285565b81019060408183031261201557805167ffffffffffffffff811161200d5782611f7d918301614334565b9060208101519067ffffffffffffffff821161201157019180601f8401121561200d57825190611fac82614308565b93611fba6040519586614285565b82855260208086019360051b82010191821161200957602001915b818310611fec575050506001600160a01b03611e81565b8251600281101561200557815260209283019201611fd5565b8680fd5b8580fd5b8380fd5b8480fd5b8280fd5b8161202391614285565b61020d57805f611e32565b8161203891614285565b61020d57805f611dd8565b8161204d91614285565b61020d57805f611d84565b503461020d578060031936011261020d57601a5461207581614308565b916120836040519384614285565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b8383106120c557604051806107f18782614128565b6001602081926120d485614421565b8152019201920191906120b0565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576122ad575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d757612298575b506001600160a01b03601f5460081c166001600160a01b0360245416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d75761165f575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761164a57505061163060206001600160a01b03601f5460081c166001600160a01b0360215416906001600160a01b0360225416604051926110ac8585614285565b816122a291614285565b61020d57805f6121b0565b816122b791614285565b61020d57805f612156565b503461020d578060031936011261020d5760206001600160a01b0360235416604051908152f35b503461020d578060031936011261020d57601b5461230681614308565b6123136040519182614285565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b8383106123eb57868587604051928392602084019060208552518091526040840160408260051b8601019392905b82821061238057505050500390f35b919360206123db827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc06001959799849503018652885190836123cb8351604084526040840190614090565b92015190848184039101526140d3565b9601920192018594939192612371565b600260206001926040516123fe8161423c565b61240786614421565b8152612414858701614544565b83820152815201920192019190612343565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761263e575b506001600160a01b03601f5460081c166001600160a01b0360245416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d757612629575b506001600160a01b0360255416803b15610425578180916024604051809481937f2dcfafd70000000000000000000000000000000000000000000000000000000083528160048401525af180156103d757612614575b506001600160a01b03601f5460081c166001600160a01b0360255416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d7576113b7575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576113a257506001600160a01b03602154166112926040519161122b602084614285565b8161261e91614285565b61020d57805f61254a565b8161263391614285565b61020d57805f6124f4565b8161264891614285565b61020d57805f61249a565b503461020d578060031936011261020d5760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b8181106126b2576107f1856107e581870382614285565b82546001600160a01b031684526020909301926001928301920161269b565b503461020d578060031936011261020d5760206001600160a01b03601f5460081c16604051908152f35b503461020d578060031936011261020d5760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b81811061275a576107f1856107e581870382614285565b82546001600160a01b0316845260209093019260019283019201612743565b503461020d578060031936011261020d57806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761295f575b506001600160a01b0360215416604051907f118cdaa7000000000000000000000000000000000000000000000000000000006020830152602482015260248152612838604482614285565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610425578161289391604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190614090565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576118a057506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d7576103e2575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b8161296991614285565b61020d57805f6127ed565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757612b49575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f3d0f293d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757612b34575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f75782916024839260405194859384927f04f386f400000000000000000000000000000000000000000000000000000000845260048401525af180156103d7576103e2575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b81612b3e91614285565b61020d57805f612a7a565b81612b5391614285565b61020d57805f6129e8565b503461020d578060031936011261020d5761163060206001600160a01b03601f5460081c166001600160a01b0360215416906001600160a01b0360225416604051926110ac8585614285565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757612e92575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d757612e7d575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517f491cc7c20000000000000000000000000000000000000000000000000000000081526001600482015281602482015281604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757612e68575b506001600160a01b036023541660405190807fb5d68ca46372bbe6ec138d3d0423608269b3117496a46268f86080cdbcbea9be8480a26001600160a01b03601f5460081c16803b156107425760248385819381957f04f386f400000000000000000000000000000000000000000000000000000000845260048401525af180156103d757612e53575b50506001600160a01b03601f5460081c166040517f1b42c7110000000000000000000000000000000000000000000000000000000081528281600481855afa8015610c1d57612dd8918491612e39575b50516149ee565b60206001600160a01b03602354166024604051809481937f21284f7e00000000000000000000000000000000000000000000000000000000835260048301525afa9081156103d7578291610bdc575060028110156106d15761067e906149ee565b612e4d91503d8086833e610c0f8183614285565b5f612dd1565b81612e5d91614285565b61020d57805f612d81565b81612e7291614285565b61020d57805f612cf8565b81612e8791614285565b61020d57805f612c72565b81612e9c91614285565b61020d57805f612c1e565b503461020d578060031936011261020d5760206001600160a01b0360255416604051908152f35b503461020d578060031936011261020d57601e54612eeb81614308565b612ef86040519182614285565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b8383106130395786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310612f645786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110612ff057505050505060208060019297019301930190928695949293612f57565b909192939460208061302c837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa087600196030189528951614090565b9701950193929101612fcc565b6040516130458161423c565b6001600160a01b03835416815260018301805461306181614308565b9161306f6040519384614285565b8183528a526020808b20908b9084015b8382106130a5575050505060019282602092836002950152815201920192019190612f28565b6001602081926130b486614421565b81520193019101909161307f565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576132d6575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d7576132c1575b506001600160a01b03601f5460081c166001600160a01b0360245416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d7576132ac575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757613297575b506001600160a01b03602454166112926001600160a01b03602154169161126660405161325f602082614285565b5f81526040519485937f0cecaaea000000000000000000000000000000000000000000000000000000006020860152602485016142de565b816132a191614285565b61020d57805f613231565b816132b691614285565b61020d57805f6131de565b816132cb91614285565b61020d57805f61318a565b816132e091614285565b61020d57805f613136565b503461020d578060031936011261020d5760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b81811061334a576107f1856107e581870382614285565b82546001600160a01b0316845260209093019260019283019201613333565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761351f575b506001600160a01b03601f5460081c166001600160a01b0360245416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d75761350a575b506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f75782916064839260405194859384926258e30f60e91b8452600484015260016024840152600160448401525af180156103d7576132ac575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761329757506001600160a01b03602454166112926001600160a01b03602154169161126660405161325f602082614285565b8161351491614285565b61020d57805f613431565b8161352991614285565b61020d57805f6133dd565b503461020d578060031936011261020d5760206001600160a01b0360265416604051908152f35b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757613721575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f3d0f293d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103fb57506001600160a01b03601f5460081c166001600160a01b0360235416813b156103f75782916044839260405194859384927f0838bbd40000000000000000000000000000000000000000000000000000000084526004840152600160248401525af180156103d7576103e2575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b8161372b91614285565b61020d57805f6135cf565b503461020d578060031936011261020d57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042557604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757613a87575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040517f491cc7c200000000000000000000000000000000000000000000000000000000815281818061381560048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757613a72575b506001600160a01b0360235416604051907f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e78380a2737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610425577f491cc7c20000000000000000000000000000000000000000000000000000000081528181806138d460048201906001606060808401938281528260208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757613a5d575b506001600160a01b0360235416604051906001817fab1bfc946643fef3ad4e44e43027bc5acf93baafa545c3ed0370761d4bb425da8580a36001600160a01b03601f5460081c16803b156107425760648385819381956258e30f60e91b8452600484015260016024840152600160448401525af180156103d757613a48575b50506001600160a01b03601f5460081c166040517f1b42c7110000000000000000000000000000000000000000000000000000000081528281600481855afa8015610c1d576139dd6001600160a01b03916020938691610bfb5750610b6b8151614977565b51166139f56001600160a01b03602354168092614adb565b6024604051809481937f21284f7e00000000000000000000000000000000000000000000000000000000835260048301525afa9081156103d75782916106fe575060028110156106d15761067e90614977565b81613a5291614285565b61020d57805f613978565b81613a6791614285565b61020d57805f6138f9565b81613a7c91614285565b61020d57805f61383a565b81613a9191614285565b61020d57805f6137aa565b503461020d578060031936011261020d5760017fffffffffffffffffffffffff0000000000000000000000000000000000000000602054161760205560027fffffffffffffffffffffffff0000000000000000000000000000000000000000602154161760215560037fffffffffffffffffffffffff00000000000000000000000000000000000000006022541617602255737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516303223eab60e11b815260016004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d757613e5b575b50506001600160a01b0360205416604051906116e4908183019183831067ffffffffffffffff841117613e0157918391602093614b5e8439815203019082f08015613df4577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f556040516101e9908181019080821067ffffffffffffffff831117613e2e5760208161624293858583396001815203019084f08015610c1d576001600160a01b03167fffffffffffffffffffffffff0000000000000000000000000000000000000000602354161760235560405182810181811067ffffffffffffffff821117613e0157816020918585833986815203019084f08015610c1d576001600160a01b03167fffffffffffffffffffffffff0000000000000000000000000000000000000000602454161760245560405182810181811067ffffffffffffffff821117613e015781602091858583396001815203019084f08015610c1d576001600160a01b03167fffffffffffffffffffffffff00000000000000000000000000000000000000006025541617602555604051918083019183831067ffffffffffffffff841117613e015791839160209383396001815203019082f08015613df4576001600160a01b03167fffffffffffffffffffffffff00000000000000000000000000000000000000006026541617602655737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d7576103c65750f35b50604051903d90823e3d90fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b81613e6591614285565b61020d57805f613b86565b90503461404a575f60031936011261404a576001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561404a576303223eab60e11b825260048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561403f5761402c575b50806001600160a01b03601f5460081c166001600160a01b0360235416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d757614017575b506001600160a01b03601f5460081c166001600160a01b0360255416813b156103f757829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156103d75761165f575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561020d57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103d75761164a57505061163060206001600160a01b03601f5460081c166001600160a01b0360215416906001600160a01b0360225416604051926110ac8585614285565b8161402191614285565b61020d57805f613f35565b61403891505f90614285565b5f5f613ee0565b6040513d5f823e3d90fd5b5f80fd5b60206040818301928281528451809452019201905f5b8181106140715750505090565b82516001600160a01b0316845260209384019390920191600101614064565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b8181106140f05750505090565b82517fffffffff00000000000000000000000000000000000000000000000000000000168452602093840193909201916001016140e3565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061415a57505050505090565b9091929394602080614196837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187528951614090565b9701930193019193929061414b565b602081016020825282518091526040820191602060408360051b8301019401925f915b8383106141d757505050505090565b909192939460208061422d837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b038151168452015191818582015201906140d3565b970193019301919392906141c8565b6040810190811067ffffffffffffffff82111761425857604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761425857604052565b9081602091031261404a5751801515810361404a5790565b6001600160a01b036143059493816060941683521660208201528160408201520190614090565b90565b67ffffffffffffffff81116142585760051b60200190565b51906001600160a01b038216820361404a57565b9080601f8301121561404a57815161434b81614308565b926143596040519485614285565b81845260208085019260051b82010192831161404a57602001905b8282106143815750505090565b6020809161438e84614320565b815201910190614374565b9060208282031261404a57815167ffffffffffffffff811161404a576143059201614334565b8051156143cc5760200190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b8051600110156143cc5760400190565b9081602091031261404a5751600281101561404a5790565b90604051915f8154908160011c926001831692831561451a575b6020851084146144ed5784875286939081156144ad5750600114614469575b5061446792500383614285565b565b90505f9291925260205f20905f915b818310614491575050906020614467928201015f61445a565b6020919350806001915483858901015201910190918492614478565b602093506144679592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f61445a565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f169361443b565b6040906001600160a01b0361430594931681528160208201520190614090565b90604051918281549182825260208201905f5260205f20925f905b80600783011061475b57614467945491818110614725575b8181106146ef575b8181106146b9575b818110614683575b81811061464d575b818110614617575b8181106145e2575b106145b5575b500383614285565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f6145ad565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b1681520193016145a7565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b16815201930161459f565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b168152019301614597565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b16815201930161458f565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b168152019301614587565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b16815201930161457f565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b168152019301614577565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e082015201940192018592939161455f565b5160028110156147f55790565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60085460ff1680156148315790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561403f575f916148c9575b50151590565b90506020813d6020116148f3575b816148e460209383614285565b8101031261404a57515f6148c3565b3d91506148d7565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561404a57604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561403f5761496d5750565b5f61446791614285565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561404a57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152600160248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561403f5761496d5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561404a57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561403f5761496d5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561404a57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152600260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561403f5761496d5750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561404a576001600160a01b039081604051937f515361f60000000000000000000000000000000000000000000000000000000085521660048401521660248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561403f5761496d575056fe60803460b857601f6116e438819003918201601f19168301916001600160401b0383118484101760bc5780849260209460405283398101031260b857516001600160a01b0381169081900360b857801560a5575f80546001600160a01b031981168317825560405192916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a361161390816100d18239f35b631e4fbdf760e01b5f525f60045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c806304f386f41461094a578063052eefd11461083e5780630838bbd4146108065780631b42c711146106e957806321284f7e1461069a578063715018a61461061e5780637a3979dc1461058b5780638da5cb5b14610559578063a213fd221461035a578063a26b4a881461033f578063b1c61e001461016f5763f2fde38b1461009d575f80fd5b3461016b57602060031936011261016b5773ffffffffffffffffffffffffffffffffffffffff6100cb610aa4565b6100d3611093565b16801561013f5773ffffffffffffffffffffffffffffffffffffffff5f54827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f80fd5b3461016b57606060031936011261016b57610188610aa4565b60243590600282101561016b5760443590811515820361016b576101aa611093565b6101b2611093565b73ffffffffffffffffffffffffffffffffffffffff8116918215610317576101d9826110df565b6102ef57602860015410156102c757156102b9576101f69061152a565b1561025b57807f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e75f80a2805f5260036020526102358260405f20610b1d565b7fab1bfc946643fef3ad4e44e43027bc5acf93baafa545c3ed0370761d4bb425da5f80a3005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f41646472657373206e6f742061646465640000000000000000000000000000006044820152fd5b6102c29061142a565b6101f6565b7f13d867a2000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fa2d86a1e000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe6c4247b000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461016b575f60031936011261016b57602060405160288152f35b3461016b575f60031936011261016b5760015461037681610c70565b9061038081610c58565b61038d6040519182610bea565b81815261039982610c58565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe060208301910136823760015f9081527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff165b8482108061053a575b1561052c5761043e8287610d19565b73ffffffffffffffffffffffffffffffffffffffff82168091525f52600360205260ff60405f205416906104728386610d19565b9160028110156104ff5761048692526113c9565b901561049c576104969091610cbf565b90610426565b5050906104bc9392505b6020604051948594604086526040860190610ac7565b918483038286015251918281520191905f5b8181106104dc575050500390f35b91935091602080826104f16001948851610b10565b0194019101918493926104ce565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5050906104bc9392506104a6565b5073ffffffffffffffffffffffffffffffffffffffff8116151561042f565b3461016b575f60031936011261016b57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b3461016b57606060031936011261016b576105a4610aa4565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361016b576044359067ffffffffffffffff821161016b573660238301121561016b5781600401359067ffffffffffffffff821161016b57366024838501011161016b576020936024610614940191610de7565b6040519015158152f35b3461016b575f60031936011261016b57610636611093565b5f73ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461016b57602060031936011261016b5773ffffffffffffffffffffffffffffffffffffffff6106c8610aa4565b165f526003602052602060ff60405f2054166106e76040518092610b10565bf35b3461016b575f60031936011261016b5760015461070581610c70565b60015f9081527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5491929173ffffffffffffffffffffffffffffffffffffffff165b828210806107e7575b156107db576107a5906107878386610d19565b73ffffffffffffffffffffffffffffffffffffffff821690526113c9565b90156107bb576107b59091610cbf565b9061076b565b5050506107d7905b604051918291602083526020830190610ac7565b0390f35b5050506107d7906107c3565b5073ffffffffffffffffffffffffffffffffffffffff81161515610774565b3461016b57604060031936011261016b5761081f610aa4565b602435600281101561016b5761083c91610837611093565b610b54565b005b3461016b57604060031936011261016b57610857610aa4565b60243590811515820361016b5761086c611093565b610874611093565b73ffffffffffffffffffffffffffffffffffffffff81169182156103175761089b826110df565b6102ef57602860015410156102c7571561093c576108b89061152a565b1561025b57805f917f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e78380a28082526003602052604082207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0081541690557fab1bfc946643fef3ad4e44e43027bc5acf93baafa545c3ed0370761d4bb425da8280a3005b6109459061142a565b6108b8565b3461016b57602060031936011261016b57610963610aa4565b61096b611093565b610973611093565b73ffffffffffffffffffffffffffffffffffffffff81169081156103175761099a816110df565b15610a7c5773ffffffffffffffffffffffffffffffffffffffff6109be83926112b4565b1603610a1e57807fb5d68ca46372bbe6ec138d3d0423608269b3117496a46268f86080cdbcbea9be5f80a25f52600360205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0081541690555f80f35b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601360248201527f41646472657373206e6f742072656d6f766564000000000000000000000000006044820152fd5b7f3d0f293d000000000000000000000000000000000000000000000000000000005f5260045ffd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361016b57565b90602080835192838152019201905f5b818110610ae45750505090565b825173ffffffffffffffffffffffffffffffffffffffff16845260209384019390920191600101610ad7565b9060028210156104ff5752565b9060028110156104ff5760ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008354169116179055565b73ffffffffffffffffffffffffffffffffffffffff811690811561031757610b7b906110df565b15610a7c57805f52600360205260ff60405f2054169060028310156104ff5760028210156104ff57828214610be557805f526003602052610bbf8360405f20610b1d565b7fa09388b78c1a36296c3bc4cc7a25b5768b1d9e0e628ed553e26c26aaedf946225f80a4565b505050565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610c2b57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b67ffffffffffffffff8111610c2b5760051b60200190565b90610c7a82610c58565b610c876040519182610bea565b8281527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0610cb58294610c58565b0190602036910137565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8114610cec5760010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b8051821015610d2d5760209160051b010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b9081602091031261016b5751801515810361016b5790565b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b73ffffffffffffffffffffffffffffffffffffffff610de49593816060941683521660208201528160408201520191610d72565b90565b60015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5491949390929173ffffffffffffffffffffffffffffffffffffffff168015611089575f9586915b73ffffffffffffffffffffffffffffffffffffffff8116801561107c57805f52600360205260ff60405f20541660028110156104ff5785918791610ff05790602091858a610ed5604051968795869485947f7a3979dc00000000000000000000000000000000000000000000000000000000865260048601610db0565b03915afa908115610fe5575f91610fb7575b5015610f7e57610ef6906113c9565b90610e5857505090919293945b81610f75575b50610f1657505050600190565b73ffffffffffffffffffffffffffffffffffffffff9291610f71916040519485947ff59be56c000000000000000000000000000000000000000000000000000000008652166004850152604060248501526044840191610d72565b0390fd5b9050155f610f09565b8590610f7185876040519485947f0cecaaea00000000000000000000000000000000000000000000000000000000865260048601610db0565b610fd8915060203d8111610fde575b610fd08183610bea565b810190610d5a565b5f610ee7565b503d610fc6565b6040513d5f823e3d90fd5b9450602090600195858a611033604051968795869485947f7a3979dc00000000000000000000000000000000000000000000000000000000865260048601610db0565b03915afa908115610fe5575f9161105e575b50611054575b610ef6906113c9565b919650869161104b565b611076915060203d8111610fde57610fd08183610bea565b5f611045565b5050509091929394610f03565b5060019450505050565b73ffffffffffffffffffffffffffffffffffffffff5f541633036110b357565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff16805f52600260205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615806111a2575b1561119c5760015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff160361119857600190565b5f90565b50600190565b50805f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615611129565b60010173ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f205416158061126a575b15611263575f805260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2054169116145f1461119857600190565b5050600190565b5073ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615611223565b73ffffffffffffffffffffffffffffffffffffffff8116801580156113b7575b6113b1575f90815260026020818152604080842084805280835281852080546001808852848820805473ffffffffffffffffffffffffffffffffffffffff908116808b52898952878b208b80528952878b208054929095167fffffffffffffffffffffffff00000000000000000000000000000000000000009283168117909555938a52978752858920828a5287529490972080548716909117905580548516905590915280549091169055547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8101908111610cec5760015590565b50505f90565b506113c38260016111d7565b156112d4565b6113d48160016111d7565b6113df57505f905f90565b73ffffffffffffffffffffffffffffffffffffffff165f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f205416908115159190565b6114358160016111d7565b1580611519575b61144557505f90565b7f6ee3efecae883df2d7ccda22610b4ca771a299e707cb0d65c4ec97dc4e6668ad805473ffffffffffffffffffffffffffffffffffffffff9283165f818152600260208181526040808420600180865281845282862080547fffffffffffffffffffffffff000000000000000000000000000000000000000090811690915589548116881790995598909616808552928252808420978452968152868320805487169094179093558180529290915292909220805490911690911790555b60015460018101809111610cec57600155600190565b506115255f60016111d7565b61143c565b6115358160016111d7565b1580611602575b61154557505f90565b7f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d805473ffffffffffffffffffffffffffffffffffffffff9283165f81815260026020818152604080842084805280835281852080547fffffffffffffffffffffffff00000000000000000000000000000000000000009081169091558854811687179098559790951680845291815284832083805281528483208054871690941790935560018252949091522080549091169091179055611503565b5061160e5f60016111d7565b61153c56608034606057601f6101e938819003918201601f19168301916001600160401b038311848410176064578084926020946040528339810103126060575180151580910360605760ff80195f54169116175f5560405161017090816100798239f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610012575f80fd5b5f3560e01c80632dcfafd7146100bf57637a3979dc14610030575f80fd5b346100bb5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100bb5761006761012a565b5061007061014d565b5060443567ffffffffffffffff81116100bb57366023820112156100bb57806004013567ffffffffffffffff81116100bb57369101602401116100bb5760209060ff5f541615158152f35b5f80fd5b346100bb5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100bb576004358015158091036100bb5760ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff005f54169116175f555f80f35b6004359073ffffffffffffffffffffffffffffffffffffffff821682036100bb57565b6024359073ffffffffffffffffffffffffffffffffffffffff821682036100bb5756
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81b[\xDA\x97\x14a>pWP\x80c\n\x92T\xE4\x14a:\x9CW\x80c\x0C\"\xBD\xF8\x14a76W\x80c\x13\xA7\xF3\x0E\x14a5[W\x80c\x18V?\xCA\x14a54W\x80c\x1Du\x97\x13\x14a3iW\x80c\x1E\xD7\x83\x1C\x14a2\xEBW\x80c%8\x8D\x16\x14a0\xC2W\x80c*\xDE8\x80\x14a.\xCEW\x80c1_A\x18\x14a.\xA7W\x80c2\xEEh\x9A\x14a+\xAAW\x80c4\xD9#\xDB\x14a+^W\x80c9\xFE9\x8D\x14a)tW\x80c>Lh\xAE\x14a'yW\x80c>^<#\x14a&\xFBW\x80c?j\xFD\xE6\x14a&\xD1W\x80c?r\x86\xF4\x14a&SW\x80cD\x12\xDE\xEB\x14a$&W\x80cf\xD9\xA9\xA0\x14a\"\xE9W\x80ckDQ\x1A\x14a\"\xC2W\x80cp\xC2}\xFA\x14a \xE2W\x80c\x85\"l\x81\x14a XW\x80c\x85\x90!\xA4\x14a\x1D\x10W\x80c\x91j\x17\xC6\x14a\x1CfW\x80c\x94a\x92.\x14a\x1A\xAAW\x80c\x972\xE2a\x14a\x18\xDFW\x80c\x9A\xBD\xF4z\x14a\x16\x9EW\x80c\x9F\xAA\xE1\xA9\x14a\x14~W\x80c\x9F\xDA\\\xAF\x14a\x0EJW\x80c\xA9\xC4\xD1\x04\x14a\x0E#W\x80c\xAC\x17\x17\xB0\x14a\r\xFCW\x80c\xB0FO\xDC\x14a\rRW\x80c\xB5P\x8A\xA9\x14a\x0C\xC8W\x80c\xB9\xED\xB1\xAF\x14a\x0C\xA1W\x80c\xBAAO\xA6\x14a\x0C|W\x80c\xC1f\xE0C\x14a\x08\xCAW\x80c\xC2\xE9\xF2\xE4\x14a\x08\x14W\x80c\xE2\x0C\x9Fq\x14a\x07\x86W\x80c\xF7\xA4\xD6\x01\x14a\x04(W\x80c\xF8\x18\x98\x95\x14a\x026W\x80c\xF8Q\xA4@\x14a\x02\x10Wc\xFAv&\xD4\x14a\x01\xEBW_\x80\xFD[4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x04\x10W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xFBW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x088\xBB\xD4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xE2W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[\x81a\x03\xD0\x91aB\x85V[a\x02\rW\x80\xF3[`@Q=\x84\x82>=\x90\xFD[\x81a\x03\xEC\x91aB\x85V[a\x02\rW\x80_a\x03qV[PP\xFD[\x81a\x04\x05\x91aB\x85V[a\x02\rW\x80_a\x03\x04V[\x81a\x04\x1A\x91aB\x85V[a\x02\rW\x80_a\x02\xAAV[P\xFD[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x07qW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x07\\W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R`\x01`$\x82\x01R`\x01`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x07GW[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90`\x01\x83\x82\x7F\xA0\x93\x88\xB7\x8C\x1A6)l;\xC4\xCCz%\xB5v\x8B\x1D\x9E\x0Eb\x8E\xD5S\xE2l&\xAA\xED\xF9F\"\x82\x80\xA4`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07BW`D\x83\x85\x81\x93\x81\x95\x7F\x088\xBB\xD4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x07-W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x94\x81\x93\x7F!(O~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x03\xD7W\x82\x91a\x06\xFEW[P`\x02\x81\x10\x15a\x06\xD1Wa\x06~\x90aIwV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`!`\x04R\xFD[a\x07 \x91P` =` \x11a\x07&W[a\x07\x18\x81\x83aB\x85V[\x81\x01\x90aD\tV[_a\x06kV[P=a\x07\x0EV[\x81a\x077\x91aB\x85V[a\x02\rW\x80_a\x06\x0BV[PPP\xFD[\x81a\x07Q\x91aB\x85V[a\x02\rW\x80_a\x05xV[\x81a\x07f\x91aB\x85V[a\x02\rW\x80_a\x04\xF0V[\x81a\x07{\x91aB\x85V[a\x02\rW\x80_a\x04\x9CV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x07\xF5Wa\x07\xF1\x85a\x07\xE5\x81\x87\x03\x82aB\x85V[`@Q\x91\x82\x91\x82a@NV[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x07\xCEV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03\xD7W\x82\x90a\x08\x8BW[a\x08\x88\x91P`\x01`\x01`\xA0\x1B\x03` T\x16\x90aJ\xDBV[\x80\xF3[P` \x81=` \x11a\x08\xC2W[\x81a\x08\xA5` \x93\x83aB\x85V[\x81\x01\x03\x12a\x08\xBEWa\x08\xB9a\x08\x88\x91aC V[a\x08qV[P\x80\xFD[=\x91Pa\x08\x98V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x0CgW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a\t\xA9`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x0CRW[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7\x83\x80\xA2sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a\nh`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x0C=W[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x82\x81\x7F\xAB\x1B\xFC\x94fC\xFE\xF3\xADND\xE40'\xBCZ\xCF\x93\xBA\xAF\xA5E\xC3\xED\x03pv\x1DK\xB4%\xDA\x82\x80\xA3`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07BW`D\x83\x85\x81\x93\x81\x95c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x0C(W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81`\x04\x81\x85Z\xFA\x80\x15a\x0C\x1DWa\x0Bp`\x01`\x01`\xA0\x1B\x03\x91` \x93\x86\x91a\x0B\xFBW[Pa\x0Bk\x81QaIwV[aC\xBFV[Q\x16a\x0B\x88`\x01`\x01`\xA0\x1B\x03`#T\x16\x80\x92aJ\xDBV[`$`@Q\x80\x94\x81\x93\x7F!(O~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x03\xD7W\x82\x91a\x0B\xDCW[P`\x02\x81\x10\x15a\x06\xD1Wa\x06~\x90aI\xEEV[a\x0B\xF5\x91P` =` \x11a\x07&Wa\x07\x18\x81\x83aB\x85V[_a\x0B\xC9V[a\x0C\x17\x91P=\x80\x88\x83>a\x0C\x0F\x81\x83aB\x85V[\x81\x01\x90aC\x99V[_a\x0B`V[`@Q=\x85\x82>=\x90\xFD[\x81a\x0C2\x91aB\x85V[a\x02\rW\x80_a\x0B\x05V[\x81a\x0CG\x91aB\x85V[a\x02\rW\x80_a\n\x8DV[\x81a\x0C\\\x91aB\x85V[a\x02\rW\x80_a\t\xCEV[\x81a\x0Cq\x91aB\x85V[a\x02\rW\x80_a\t>V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` a\x0C\x97aH\"V[`@Q\x90\x15\x15\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x19Ta\x0C\xE5\x81aC\x08V[\x91a\x0C\xF3`@Q\x93\x84aB\x85V[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\r5W`@Q\x80a\x07\xF1\x87\x82aA(V[`\x01` \x81\x92a\rD\x85aD!V[\x81R\x01\x92\x01\x92\x01\x91\x90a\r V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x1CTa\ro\x81aC\x08V[\x91a\r}`@Q\x93\x84aB\x85V[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\r\xBFW`@Q\x80a\x07\xF1\x87\x82aA\xA5V[`\x02` `\x01\x92`@Qa\r\xD2\x81aB<V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\r\xEA\x85\x87\x01aEDV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\r\xAAV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x14iW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x14TW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x14?W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x04%W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F-\xCF\xAF\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x14*W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`&T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x14\x15W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x14\0W[PPa\x10\xE5` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x10\xAC\x85\x85aB\x85V[_\x84R`@Q\x95\x86\x94\x85\x93\x84\x93\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01aB\xDEV[\x03\x91Z\xFA\x80\x15a\x03\xD7Wa\x11\0\x91\x83\x91a\x13\xE1W[PaH\xFBV[\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x13\xCCW[P`\x01`\x01`\xA0\x1B\x03`&T\x16\x80;\x15a\x04%W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F-\xCF\xAF\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x13\xB7W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x13\xA2W[P`\x01`\x01`\xA0\x1B\x03`!T\x16a\x12\x92`@Q\x91a\x12+` \x84aB\x85V[_\x83Ra\x12f`@Q\x93\x84\x92\x7F\xF5\x9B\xE5l\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x85\x01R`$\x84\x01aE$V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82aB\x85V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W\x81a\x12\xED\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a@\x90V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x13\x8DW[PPa\x13O` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x10\xAC\x85\x85aB\x85V[\x03\x91Z\xFA\x80\x15a\x03\xD7Wa\x13aWP\x80\xF3[a\x13\x82\x90` =` \x11a\x13\x86W[a\x13z\x81\x83aB\x85V[\x81\x01\x90aB\xC6V[P\x80\xF3[P=a\x13pV[\x81a\x13\x97\x91aB\x85V[a\x02\rW\x80_a\x13\x12V[\x81a\x13\xAC\x91aB\x85V[a\x02\rW\x80_a\x12\x0CV[\x81a\x13\xC1\x91aB\x85V[a\x02\rW\x80_a\x11\xB9V[\x81a\x13\xD6\x91aB\x85V[a\x02\rW\x80_a\x11cV[a\x13\xFA\x91P` =` \x11a\x13\x86Wa\x13z\x81\x83aB\x85V[_a\x10\xFAV[\x81a\x14\n\x91aB\x85V[a\x02\rW\x80_a\x10oV[\x81a\x14\x1F\x91aB\x85V[a\x02\rW\x80_a\x10\x1CV[\x81a\x144\x91aB\x85V[a\x02\rW\x80_a\x0F\xC2V[\x81a\x14I\x91aB\x85V[a\x02\rW\x80_a\x0FlV[\x81a\x14^\x91aB\x85V[a\x02\rW\x80_a\x0F\x12V[\x81a\x14s\x91aB\x85V[a\x02\rW\x80_a\x0E\xBEV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x16\x89W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x16tW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x16_W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x16JW[PPa\x160` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x10\xAC\x85\x85aB\x85V[\x03\x91Z\xFA\x80\x15a\x03\xD7Wa\x08\x88\x91\x83\x91a\x13\xE1WPaH\xFBV[\x81a\x16T\x91aB\x85V[a\x02\rW\x80_a\x15\xF3V[\x81a\x16i\x91aB\x85V[a\x02\rW\x80_a\x15\xA0V[\x81a\x16~\x91aB\x85V[a\x02\rW\x80_a\x15FV[\x81a\x16\x93\x91aB\x85V[a\x02\rW\x80_a\x14\xF2V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x18\xCAW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x18\xB5W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xA2\xD8j\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x18\xA0W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xE2WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[\x81a\x18\xAA\x91aB\x85V[a\x02\rW\x80_a\x17\xF8V[\x81a\x18\xBF\x91aB\x85V[a\x02\rW\x80_a\x17fV[\x81a\x18\xD4\x91aB\x85V[a\x02\rW\x80_a\x17\x12V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x1A\x95W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x1A\x80W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x04%W\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x81`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xE2WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[\x81a\x1A\x8A\x91aB\x85V[a\x02\rW\x80_a\x19\xE5V[\x81a\x1A\x9F\x91aB\x85V[a\x02\rW\x80_a\x19SV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x1CQW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x1C<W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x13\xB7WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x13\xA2WP`\x01`\x01`\xA0\x1B\x03`!T\x16a\x12\x92`@Q\x91a\x12+` \x84aB\x85V[\x81a\x1CF\x91aB\x85V[a\x02\rW\x80_a\x1BrV[\x81a\x1C[\x91aB\x85V[a\x02\rW\x80_a\x1B\x1EV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x1DTa\x1C\x83\x81aC\x08V[\x91a\x1C\x91`@Q\x93\x84aB\x85V[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x1C\xD3W`@Q\x80a\x07\xF1\x87\x82aA\xA5V[`\x02` `\x01\x92`@Qa\x1C\xE6\x81aB<V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x1C\xFE\x85\x87\x01aEDV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x1C\xBEV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa CW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa .W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa \x19W[P`\x04\x90`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xA2\x13\xFD\"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x03\xD7W\x82\x90\x83\x92a\x1FAW[P`\x01`\x01`\xA0\x1B\x03a\x1E\xC4\x82a\x1E\x9Ba\x1E\xD8\x94QaJdV[a\x1E\xA5\x85QaJdV[a\x1E\xBF\x83a\x1E\xB2\x83aC\xBFV[Q\x16\x84`$T\x16\x90aJ\xDBV[aC\xF9V[Q\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x90aJ\xDBV[a\x1E\xE9a\x1E\xE4\x82aC\xBFV[aG\xE8V[`\x02\x81\x10\x15a\x1F\x14Wa\x1F\x02\x91a\x1E\xBFa\x1E\xE4\x92aIwV[`\x02\x81\x10\x15a\x06\xD1Wa\x06~\x90aI\xEEV[`$\x83\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`!`\x04R\xFD[\x91PP=\x80\x83\x83>a\x1FS\x81\x83aB\x85V[\x81\x01\x90`@\x81\x83\x03\x12a \x15W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a \rW\x82a\x1F}\x91\x83\x01aC4V[\x90` \x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a \x11W\x01\x91\x80`\x1F\x84\x01\x12\x15a \rW\x82Q\x90a\x1F\xAC\x82aC\x08V[\x93a\x1F\xBA`@Q\x95\x86aB\x85V[\x82\x85R` \x80\x86\x01\x93`\x05\x1B\x82\x01\x01\x91\x82\x11a \tW` \x01\x91[\x81\x83\x10a\x1F\xECWPPP`\x01`\x01`\xA0\x1B\x03a\x1E\x81V[\x82Q`\x02\x81\x10\x15a \x05W\x81R` \x92\x83\x01\x92\x01a\x1F\xD5V[\x86\x80\xFD[\x85\x80\xFD[\x83\x80\xFD[\x84\x80\xFD[\x82\x80\xFD[\x81a #\x91aB\x85V[a\x02\rW\x80_a\x1E2V[\x81a 8\x91aB\x85V[a\x02\rW\x80_a\x1D\xD8V[\x81a M\x91aB\x85V[a\x02\rW\x80_a\x1D\x84V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x1ATa u\x81aC\x08V[\x91a \x83`@Q\x93\x84aB\x85V[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a \xC5W`@Q\x80a\x07\xF1\x87\x82aA(V[`\x01` \x81\x92a \xD4\x85aD!V[\x81R\x01\x92\x01\x92\x01\x91\x90a \xB0V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\"\xADW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\"\x98W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x16_WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x16JWPPa\x160` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x10\xAC\x85\x85aB\x85V[\x81a\"\xA2\x91aB\x85V[a\x02\rW\x80_a!\xB0V[\x81a\"\xB7\x91aB\x85V[a\x02\rW\x80_a!VV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x1BTa#\x06\x81aC\x08V[a#\x13`@Q\x91\x82aB\x85V[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a#\xEBW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a#\x80WPPPP\x03\x90\xF3[\x91\x93` a#\xDB\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a#\xCB\x83Q`@\x84R`@\x84\x01\x90a@\x90V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra@\xD3V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a#qV[`\x02` `\x01\x92`@Qa#\xFE\x81aB<V[a$\x07\x86aD!V[\x81Ra$\x14\x85\x87\x01aEDV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a#CV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa&>W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa&)W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x04%W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F-\xCF\xAF\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa&\x14W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x13\xB7WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x13\xA2WP`\x01`\x01`\xA0\x1B\x03`!T\x16a\x12\x92`@Q\x91a\x12+` \x84aB\x85V[\x81a&\x1E\x91aB\x85V[a\x02\rW\x80_a%JV[\x81a&3\x91aB\x85V[a\x02\rW\x80_a$\xF4V[\x81a&H\x91aB\x85V[a\x02\rW\x80_a$\x9AV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a&\xB2Wa\x07\xF1\x85a\x07\xE5\x81\x87\x03\x82aB\x85V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a&\x9BV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a'ZWa\x07\xF1\x85a\x07\xE5\x81\x87\x03\x82aB\x85V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a'CV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa)_W[P`\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01R`$\x81Ra(8`D\x82aB\x85V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W\x81a(\x93\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a@\x90V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x18\xA0WP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xE2WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[\x81a)i\x91aB\x85V[a\x02\rW\x80_a'\xEDV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa+IW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa+4W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xE2WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[\x81a+>\x91aB\x85V[a\x02\rW\x80_a*zV[\x81a+S\x91aB\x85V[a\x02\rW\x80_a)\xE8V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rWa\x160` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x10\xAC\x85\x85aB\x85V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa.\x92W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa.}W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R\x81`$\x82\x01R\x81`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa.hW[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x80\x7F\xB5\xD6\x8C\xA4cr\xBB\xE6\xEC\x13\x8D=\x04#`\x82i\xB3\x11t\x96\xA4bh\xF8`\x80\xCD\xBC\xBE\xA9\xBE\x84\x80\xA2`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07BW`$\x83\x85\x81\x93\x81\x95\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa.SW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81`\x04\x81\x85Z\xFA\x80\x15a\x0C\x1DWa-\xD8\x91\x84\x91a.9W[PQaI\xEEV[` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x94\x81\x93\x7F!(O~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x03\xD7W\x82\x91a\x0B\xDCWP`\x02\x81\x10\x15a\x06\xD1Wa\x06~\x90aI\xEEV[a.M\x91P=\x80\x86\x83>a\x0C\x0F\x81\x83aB\x85V[_a-\xD1V[\x81a.]\x91aB\x85V[a\x02\rW\x80_a-\x81V[\x81a.r\x91aB\x85V[a\x02\rW\x80_a,\xF8V[\x81a.\x87\x91aB\x85V[a\x02\rW\x80_a,rV[\x81a.\x9C\x91aB\x85V[a\x02\rW\x80_a,\x1EV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x1ETa.\xEB\x81aC\x08V[a.\xF8`@Q\x91\x82aB\x85V[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a09W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a/dW\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a/\xF0WPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a/WV[\x90\x91\x92\x93\x94` \x80a0,\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa@\x90V[\x97\x01\x95\x01\x93\x92\x91\x01a/\xCCV[`@Qa0E\x81aB<V[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta0a\x81aC\x08V[\x91a0o`@Q\x93\x84aB\x85V[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a0\xA5WPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a/(V[`\x01` \x81\x92a0\xB4\x86aD!V[\x81R\x01\x93\x01\x91\x01\x90\x91a0\x7FV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa2\xD6W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa2\xC1W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa2\xACW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa2\x97W[P`\x01`\x01`\xA0\x1B\x03`$T\x16a\x12\x92`\x01`\x01`\xA0\x1B\x03`!T\x16\x91a\x12f`@Qa2_` \x82aB\x85V[_\x81R`@Q\x94\x85\x93\x7F\x0C\xEC\xAA\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x86\x01R`$\x85\x01aB\xDEV[\x81a2\xA1\x91aB\x85V[a\x02\rW\x80_a21V[\x81a2\xB6\x91aB\x85V[a\x02\rW\x80_a1\xDEV[\x81a2\xCB\x91aB\x85V[a\x02\rW\x80_a1\x8AV[\x81a2\xE0\x91aB\x85V[a\x02\rW\x80_a16V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a3JWa\x07\xF1\x85a\x07\xE5\x81\x87\x03\x82aB\x85V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a33V[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa5\x1FW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`$T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa5\nW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa2\xACWPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa2\x97WP`\x01`\x01`\xA0\x1B\x03`$T\x16a\x12\x92`\x01`\x01`\xA0\x1B\x03`!T\x16\x91a\x12f`@Qa2_` \x82aB\x85V[\x81a5\x14\x91aB\x85V[a\x02\rW\x80_a41V[\x81a5)\x91aB\x85V[a\x02\rW\x80_a3\xDDV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\x01`\x01`\xA0\x1B\x03`&T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa7!W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xFBWP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x088\xBB\xD4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x03\xE2WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[\x81a7+\x91aB\x85V[a\x02\rW\x80_a5\xCFV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa:\x87W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a8\x15`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa:rW[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7\x83\x80\xA2sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04%W\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a8\xD4`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R\x82` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa:]W[P`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90`\x01\x81\x7F\xAB\x1B\xFC\x94fC\xFE\xF3\xADND\xE40'\xBCZ\xCF\x93\xBA\xAF\xA5E\xC3\xED\x03pv\x1DK\xB4%\xDA\x85\x80\xA3`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07BW`d\x83\x85\x81\x93\x81\x95bX\xE3\x0F`\xE9\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R`\x01`D\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa:HW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81`\x04\x81\x85Z\xFA\x80\x15a\x0C\x1DWa9\xDD`\x01`\x01`\xA0\x1B\x03\x91` \x93\x86\x91a\x0B\xFBWPa\x0Bk\x81QaIwV[Q\x16a9\xF5`\x01`\x01`\xA0\x1B\x03`#T\x16\x80\x92aJ\xDBV[`$`@Q\x80\x94\x81\x93\x7F!(O~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x03\xD7W\x82\x91a\x06\xFEWP`\x02\x81\x10\x15a\x06\xD1Wa\x06~\x90aIwV[\x81a:R\x91aB\x85V[a\x02\rW\x80_a9xV[\x81a:g\x91aB\x85V[a\x02\rW\x80_a8\xF9V[\x81a:|\x91aB\x85V[a\x02\rW\x80_a8:V[\x81a:\x91\x91aB\x85V[a\x02\rW\x80_a7\xAAV[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U`\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"T\x16\x17`\"Usq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa>[W[PP`\x01`\x01`\xA0\x1B\x03` T\x16`@Q\x90a\x16\xE4\x90\x81\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a>\x01W\x91\x83\x91` \x93aK^\x849\x81R\x03\x01\x90\x82\xF0\x80\x15a=\xF4W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU`@Qa\x01\xE9\x90\x81\x81\x01\x90\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a>.W` \x81abB\x93\x85\x85\x839`\x01\x81R\x03\x01\x90\x84\xF0\x80\x15a\x0C\x1DW`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`#T\x16\x17`#U`@Q\x82\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a>\x01W\x81` \x91\x85\x85\x839\x86\x81R\x03\x01\x90\x84\xF0\x80\x15a\x0C\x1DW`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$T\x16\x17`$U`@Q\x82\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a>\x01W\x81` \x91\x85\x85\x839`\x01\x81R\x03\x01\x90\x84\xF0\x80\x15a\x0C\x1DW`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`%T\x16\x17`%U`@Q\x91\x80\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a>\x01W\x91\x83\x91` \x93\x839`\x01\x81R\x03\x01\x90\x82\xF0\x80\x15a=\xF4W`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`&T\x16\x17`&Usq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x03\xC6WP\xF3[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81a>e\x91aB\x85V[a\x02\rW\x80_a;\x86V[\x90P4a@JW_`\x03\x196\x01\x12a@JW`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a@JWc\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a@?Wa@,W[P\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa@\x17W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x81;\x15a\x03\xF7W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x03\xD7Wa\x16_WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02\rW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03\xD7Wa\x16JWPPa\x160` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x10\xAC\x85\x85aB\x85V[\x81a@!\x91aB\x85V[a\x02\rW\x80_a?5V[a@8\x91P_\x90aB\x85V[__a>\xE0V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a@qWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a@dV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a@\xF0WPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a@\xE3V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aAZWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aA\x96\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa@\x90V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aAKV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aA\xD7WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aB-\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a@\xD3V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aA\xC8V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aBXW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aBXW`@RV[\x90\x81` \x91\x03\x12a@JWQ\x80\x15\x15\x81\x03a@JW\x90V[`\x01`\x01`\xA0\x1B\x03aC\x05\x94\x93\x81``\x94\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a@\x90V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11aBXW`\x05\x1B` \x01\x90V[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a@JWV[\x90\x80`\x1F\x83\x01\x12\x15a@JW\x81QaCK\x81aC\x08V[\x92aCY`@Q\x94\x85aB\x85V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a@JW` \x01\x90[\x82\x82\x10aC\x81WPPP\x90V[` \x80\x91aC\x8E\x84aC V[\x81R\x01\x91\x01\x90aCtV[\x90` \x82\x82\x03\x12a@JW\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a@JWaC\x05\x92\x01aC4V[\x80Q\x15aC\xCCW` \x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80Q`\x01\x10\x15aC\xCCW`@\x01\x90V[\x90\x81` \x91\x03\x12a@JWQ`\x02\x81\x10\x15a@JW\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15aE\x1AW[` \x85\x10\x84\x14aD\xEDW\x84\x87R\x86\x93\x90\x81\x15aD\xADWP`\x01\x14aDiW[PaDg\x92P\x03\x83aB\x85V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10aD\x91WPP\x90` aDg\x92\x82\x01\x01_aDZV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92aDxV[` \x93PaDg\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_aDZV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93aD;V[`@\x90`\x01`\x01`\xA0\x1B\x03aC\x05\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a@\x90V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10aG[WaDg\x94T\x91\x81\x81\x10aG%W[\x81\x81\x10aF\xEFW[\x81\x81\x10aF\xB9W[\x81\x81\x10aF\x83W[\x81\x81\x10aFMW[\x81\x81\x10aF\x17W[\x81\x81\x10aE\xE2W[\x10aE\xB5W[P\x03\x83aB\x85V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_aE\xADV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01aE\xA7V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01aE\x9FV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01aE\x97V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01aE\x8FV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01aE\x87V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01aE\x7FV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01aEwV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91aE_V[Q`\x02\x81\x10\x15aG\xF5W\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x08T`\xFF\x16\x80\x15aH1W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a@?W_\x91aH\xC9W[P\x15\x15\x90V[\x90P` \x81=` \x11aH\xF3W[\x81aH\xE4` \x93\x83aB\x85V[\x81\x01\x03\x12a@JWQ_aH\xC3V[=\x91PaH\xD7V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a@JW`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a@?WaImWPV[_aDg\x91aB\x85V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a@JW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x01`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a@?WaImWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a@JW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a@?WaImWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a@JW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x02`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a@?WaImWPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a@JW`\x01`\x01`\xA0\x1B\x03\x90\x81`@Q\x93\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R\x16`\x04\x84\x01R\x16`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a@?WaImWPV\xFE`\x804`\xB8W`\x1Fa\x16\xE48\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`\xBCW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`\xB8WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03`\xB8W\x80\x15`\xA5W_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x82U`@Q\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3a\x16\x13\x90\x81a\0\xD1\x829\xF3[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x04\xF3\x86\xF4\x14a\tJW\x80c\x05.\xEF\xD1\x14a\x08>W\x80c\x088\xBB\xD4\x14a\x08\x06W\x80c\x1BB\xC7\x11\x14a\x06\xE9W\x80c!(O~\x14a\x06\x9AW\x80cqP\x18\xA6\x14a\x06\x1EW\x80cz9y\xDC\x14a\x05\x8BW\x80c\x8D\xA5\xCB[\x14a\x05YW\x80c\xA2\x13\xFD\"\x14a\x03ZW\x80c\xA2kJ\x88\x14a\x03?W\x80c\xB1\xC6\x1E\0\x14a\x01oWc\xF2\xFD\xE3\x8B\x14a\0\x9DW_\x80\xFD[4a\x01kW` `\x03\x196\x01\x12a\x01kWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\xCBa\n\xA4V[a\0\xD3a\x10\x93V[\x16\x80\x15a\x01?Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x80\xFD[4a\x01kW```\x03\x196\x01\x12a\x01kWa\x01\x88a\n\xA4V[`$5\x90`\x02\x82\x10\x15a\x01kW`D5\x90\x81\x15\x15\x82\x03a\x01kWa\x01\xAAa\x10\x93V[a\x01\xB2a\x10\x93V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x15a\x03\x17Wa\x01\xD9\x82a\x10\xDFV[a\x02\xEFW`(`\x01T\x10\x15a\x02\xC7W\x15a\x02\xB9Wa\x01\xF6\x90a\x15*V[\x15a\x02[W\x80\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7_\x80\xA2\x80_R`\x03` Ra\x025\x82`@_ a\x0B\x1DV[\x7F\xAB\x1B\xFC\x94fC\xFE\xF3\xADND\xE40'\xBCZ\xCF\x93\xBA\xAF\xA5E\xC3\xED\x03pv\x1DK\xB4%\xDA_\x80\xA3\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FAddress not added\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[a\x02\xC2\x90a\x14*V[a\x01\xF6V[\x7F\x13\xD8g\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xA2\xD8j\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01kW_`\x03\x196\x01\x12a\x01kW` `@Q`(\x81R\xF3[4a\x01kW_`\x03\x196\x01\x12a\x01kW`\x01Ta\x03v\x81a\x0CpV[\x90a\x03\x80\x81a\x0CXV[a\x03\x8D`@Q\x91\x82a\x0B\xEAV[\x81\x81Ra\x03\x99\x82a\x0CXV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0` \x83\x01\x91\x016\x827`\x01_\x90\x81R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x84\x82\x10\x80a\x05:W[\x15a\x05,Wa\x04>\x82\x87a\r\x19V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x91R_R`\x03` R`\xFF`@_ T\x16\x90a\x04r\x83\x86a\r\x19V[\x91`\x02\x81\x10\x15a\x04\xFFWa\x04\x86\x92Ra\x13\xC9V[\x90\x15a\x04\x9CWa\x04\x96\x90\x91a\x0C\xBFV[\x90a\x04&V[PP\x90a\x04\xBC\x93\x92P[` `@Q\x94\x85\x94`@\x86R`@\x86\x01\x90a\n\xC7V[\x91\x84\x83\x03\x82\x86\x01RQ\x91\x82\x81R\x01\x91\x90_[\x81\x81\x10a\x04\xDCWPPP\x03\x90\xF3[\x91\x93P\x91` \x80\x82a\x04\xF1`\x01\x94\x88Qa\x0B\x10V[\x01\x94\x01\x91\x01\x91\x84\x93\x92a\x04\xCEV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[PP\x90a\x04\xBC\x93\x92Pa\x04\xA6V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15\x15a\x04/V[4a\x01kW_`\x03\x196\x01\x12a\x01kW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\x01kW```\x03\x196\x01\x12a\x01kWa\x05\xA4a\n\xA4V[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01kW`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01kW6`#\x83\x01\x12\x15a\x01kW\x81`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01kW6`$\x83\x85\x01\x01\x11a\x01kW` \x93`$a\x06\x14\x94\x01\x91a\r\xE7V[`@Q\x90\x15\x15\x81R\xF3[4a\x01kW_`\x03\x196\x01\x12a\x01kWa\x066a\x10\x93V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01kW` `\x03\x196\x01\x12a\x01kWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x06\xC8a\n\xA4V[\x16_R`\x03` R` `\xFF`@_ T\x16a\x06\xE7`@Q\x80\x92a\x0B\x10V[\xF3[4a\x01kW_`\x03\x196\x01\x12a\x01kW`\x01Ta\x07\x05\x81a\x0CpV[`\x01_\x90\x81R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DT\x91\x92\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x82\x82\x10\x80a\x07\xE7W[\x15a\x07\xDBWa\x07\xA5\x90a\x07\x87\x83\x86a\r\x19V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90Ra\x13\xC9V[\x90\x15a\x07\xBBWa\x07\xB5\x90\x91a\x0C\xBFV[\x90a\x07kV[PPPa\x07\xD7\x90[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\n\xC7V[\x03\x90\xF3[PPPa\x07\xD7\x90a\x07\xC3V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15\x15a\x07tV[4a\x01kW`@`\x03\x196\x01\x12a\x01kWa\x08\x1Fa\n\xA4V[`$5`\x02\x81\x10\x15a\x01kWa\x08<\x91a\x087a\x10\x93V[a\x0BTV[\0[4a\x01kW`@`\x03\x196\x01\x12a\x01kWa\x08Wa\n\xA4V[`$5\x90\x81\x15\x15\x82\x03a\x01kWa\x08la\x10\x93V[a\x08ta\x10\x93V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x15a\x03\x17Wa\x08\x9B\x82a\x10\xDFV[a\x02\xEFW`(`\x01T\x10\x15a\x02\xC7W\x15a\t<Wa\x08\xB8\x90a\x15*V[\x15a\x02[W\x80_\x91\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7\x83\x80\xA2\x80\x82R`\x03` R`@\x82 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90U\x7F\xAB\x1B\xFC\x94fC\xFE\xF3\xADND\xE40'\xBCZ\xCF\x93\xBA\xAF\xA5E\xC3\xED\x03pv\x1DK\xB4%\xDA\x82\x80\xA3\0[a\tE\x90a\x14*V[a\x08\xB8V[4a\x01kW` `\x03\x196\x01\x12a\x01kWa\tca\n\xA4V[a\tka\x10\x93V[a\tsa\x10\x93V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a\x03\x17Wa\t\x9A\x81a\x10\xDFV[\x15a\n|Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\t\xBE\x83\x92a\x12\xB4V[\x16\x03a\n\x1EW\x80\x7F\xB5\xD6\x8C\xA4cr\xBB\xE6\xEC\x13\x8D=\x04#`\x82i\xB3\x11t\x96\xA4bh\xF8`\x80\xCD\xBC\xBE\xA9\xBE_\x80\xA2_R`\x03` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90U_\x80\xF3[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FAddress not removed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01kWV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\n\xE4WPPP\x90V[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n\xD7V[\x90`\x02\x82\x10\x15a\x04\xFFWRV[\x90`\x02\x81\x10\x15a\x04\xFFW`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83T\x16\x91\x16\x17\x90UV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a\x03\x17Wa\x0B{\x90a\x10\xDFV[\x15a\n|W\x80_R`\x03` R`\xFF`@_ T\x16\x90`\x02\x83\x10\x15a\x04\xFFW`\x02\x82\x10\x15a\x04\xFFW\x82\x82\x14a\x0B\xE5W\x80_R`\x03` Ra\x0B\xBF\x83`@_ a\x0B\x1DV[\x7F\xA0\x93\x88\xB7\x8C\x1A6)l;\xC4\xCCz%\xB5v\x8B\x1D\x9E\x0Eb\x8E\xD5S\xE2l&\xAA\xED\xF9F\"_\x80\xA4V[PPPV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C+W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0C+W`\x05\x1B` \x01\x90V[\x90a\x0Cz\x82a\x0CXV[a\x0C\x87`@Q\x91\x82a\x0B\xEAV[\x82\x81R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0a\x0C\xB5\x82\x94a\x0CXV[\x01\x90` 6\x91\x017V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x0C\xECW`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80Q\x82\x10\x15a\r-W` \x91`\x05\x1B\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x90\x81` \x91\x03\x12a\x01kWQ\x80\x15\x15\x81\x03a\x01kW\x90V[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\r\xE4\x95\x93\x81``\x94\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x91a\rrV[\x90V[`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DT\x91\x94\x93\x90\x92\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a\x10\x89W_\x95\x86\x91[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15a\x10|W\x80_R`\x03` R`\xFF`@_ T\x16`\x02\x81\x10\x15a\x04\xFFW\x85\x91\x87\x91a\x0F\xF0W\x90` \x91\x85\x8Aa\x0E\xD5`@Q\x96\x87\x95\x86\x94\x85\x94\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\x04\x86\x01a\r\xB0V[\x03\x91Z\xFA\x90\x81\x15a\x0F\xE5W_\x91a\x0F\xB7W[P\x15a\x0F~Wa\x0E\xF6\x90a\x13\xC9V[\x90a\x0EXWPP\x90\x91\x92\x93\x94[\x81a\x0FuW[Pa\x0F\x16WPPP`\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x91a\x0Fq\x91`@Q\x94\x85\x94\x7F\xF5\x9B\xE5l\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x16`\x04\x85\x01R`@`$\x85\x01R`D\x84\x01\x91a\rrV[\x03\x90\xFD[\x90P\x15_a\x0F\tV[\x85\x90a\x0Fq\x85\x87`@Q\x94\x85\x94\x7F\x0C\xEC\xAA\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\x04\x86\x01a\r\xB0V[a\x0F\xD8\x91P` =\x81\x11a\x0F\xDEW[a\x0F\xD0\x81\x83a\x0B\xEAV[\x81\x01\x90a\rZV[_a\x0E\xE7V[P=a\x0F\xC6V[`@Q=_\x82>=\x90\xFD[\x94P` \x90`\x01\x95\x85\x8Aa\x103`@Q\x96\x87\x95\x86\x94\x85\x94\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\x04\x86\x01a\r\xB0V[\x03\x91Z\xFA\x90\x81\x15a\x0F\xE5W_\x91a\x10^W[Pa\x10TW[a\x0E\xF6\x90a\x13\xC9V[\x91\x96P\x86\x91a\x10KV[a\x10v\x91P` =\x81\x11a\x0F\xDEWa\x0F\xD0\x81\x83a\x0B\xEAV[_a\x10EV[PPP\x90\x91\x92\x93\x94a\x0F\x03V[P`\x01\x94PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\x10\xB3WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80_R`\x02` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x11\xA2W[\x15a\x11\x9CW`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x11\x98W`\x01\x90V[_\x90V[P`\x01\x90V[P\x80_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\x11)V[`\x01\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x12jW[\x15a\x12cW_\x80R` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ T\x16\x91\x16\x14_\x14a\x11\x98W`\x01\x90V[PP`\x01\x90V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\x12#V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x80\x15a\x13\xB7W[a\x13\xB1W_\x90\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T`\x01\x80\x88R\x84\x88 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x8BR\x89\x89R\x87\x8B \x8B\x80R\x89R\x87\x8B \x80T\x92\x90\x95\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81\x17\x90\x95U\x93\x8AR\x97\x87R\x85\x89 \x82\x8AR\x87R\x94\x90\x97 \x80T\x87\x16\x90\x91\x17\x90U\x80T\x85\x16\x90U\x90\x91R\x80T\x90\x91\x16\x90UT\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a\x0C\xECW`\x01U\x90V[PP_\x90V[Pa\x13\xC3\x82`\x01a\x11\xD7V[\x15a\x12\xD4V[a\x13\xD4\x81`\x01a\x11\xD7V[a\x13\xDFWP_\x90_\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x90\x81\x15\x15\x91\x90V[a\x145\x81`\x01a\x11\xD7V[\x15\x80a\x15\x19W[a\x14EWP_\x90V[\x7Fn\xE3\xEF\xEC\xAE\x88=\xF2\xD7\xCC\xDA\"a\x0BL\xA7q\xA2\x99\xE7\x07\xCB\re\xC4\xEC\x97\xDCNfh\xAD\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 `\x01\x80\x86R\x81\x84R\x82\x86 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x89T\x81\x16\x88\x17\x90\x99U\x98\x90\x96\x16\x80\x85R\x92\x82R\x80\x84 \x97\x84R\x96\x81R\x86\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U\x81\x80R\x92\x90\x91R\x92\x90\x92 \x80T\x90\x91\x16\x90\x91\x17\x90U[`\x01T`\x01\x81\x01\x80\x91\x11a\x0C\xECW`\x01U`\x01\x90V[Pa\x15%_`\x01a\x11\xD7V[a\x14<V[a\x155\x81`\x01a\x11\xD7V[\x15\x80a\x16\x02W[a\x15EWP_\x90V[\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9D\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x88T\x81\x16\x87\x17\x90\x98U\x97\x90\x95\x16\x80\x84R\x91\x81R\x84\x83 \x83\x80R\x81R\x84\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U`\x01\x82R\x94\x90\x91R \x80T\x90\x91\x16\x90\x91\x17\x90Ua\x15\x03V[Pa\x16\x0E_`\x01a\x11\xD7V[a\x15<V`\x804``W`\x1Fa\x01\xE98\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`dW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12``WQ\x80\x15\x15\x80\x91\x03``W`\xFF\x80\x19_T\x16\x91\x16\x17_U`@Qa\x01p\x90\x81a\0y\x829\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x80c-\xCF\xAF\xD7\x14a\0\xBFWcz9y\xDC\x14a\x000W_\x80\xFD[4a\0\xBBW``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0\xBBWa\0ga\x01*V[Pa\0pa\x01MV[P`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xBBW6`#\x82\x01\x12\x15a\0\xBBW\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xBBW6\x91\x01`$\x01\x11a\0\xBBW` \x90`\xFF_T\x16\x15\x15\x81R\xF3[_\x80\xFD[4a\0\xBBW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0\xBBW`\x045\x80\x15\x15\x80\x91\x03a\0\xBBW`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0_T\x16\x91\x16\x17_U_\x80\xF3[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0\xBBWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0\xBBWV",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CheckTypeUpdated(address,uint8,uint8)` and selector `0xa09388b78c1a36296c3bc4cc7a25b5768b1d9e0e628ed553e26c26aaedf94622`.
```solidity
event CheckTypeUpdated(address indexed check, RequireCompositeModule.CheckType indexed oldType, RequireCompositeModule.CheckType indexed newType);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CheckTypeUpdated {
        #[allow(missing_docs)]
        pub check: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldType: <RequireCompositeModule::CheckType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub newType: <RequireCompositeModule::CheckType as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for CheckTypeUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                RequireCompositeModule::CheckType,
                RequireCompositeModule::CheckType,
            );
            const SIGNATURE: &'static str = "CheckTypeUpdated(address,uint8,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                160u8, 147u8, 136u8, 183u8, 140u8, 26u8, 54u8, 41u8, 108u8, 59u8, 196u8,
                204u8, 122u8, 37u8, 181u8, 118u8, 139u8, 29u8, 158u8, 14u8, 98u8, 142u8,
                213u8, 83u8, 226u8, 108u8, 38u8, 170u8, 237u8, 249u8, 70u8, 34u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    check: topics.1,
                    oldType: topics.2,
                    newType: topics.3,
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
                    self.check.clone(),
                    self.oldType.clone(),
                    self.newType.clone(),
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
                    &self.check,
                );
                out[2usize] = <RequireCompositeModule::CheckType as alloy_sol_types::EventTopic>::encode_topic(
                    &self.oldType,
                );
                out[3usize] = <RequireCompositeModule::CheckType as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newType,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CheckTypeUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CheckTypeUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CheckTypeUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PermissionCheckAdded(address)` and selector `0x62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e7`.
```solidity
event PermissionCheckAdded(address indexed check);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PermissionCheckAdded {
        #[allow(missing_docs)]
        pub check: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for PermissionCheckAdded {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "PermissionCheckAdded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                98u8, 16u8, 28u8, 204u8, 193u8, 134u8, 77u8, 52u8, 146u8, 41u8, 0u8,
                112u8, 244u8, 219u8, 241u8, 104u8, 121u8, 222u8, 120u8, 97u8, 172u8,
                181u8, 220u8, 184u8, 24u8, 11u8, 85u8, 210u8, 237u8, 124u8, 215u8, 231u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { check: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.check.clone())
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
                    &self.check,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PermissionCheckAdded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PermissionCheckAdded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PermissionCheckAdded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PermissionCheckAddedWithType(address,uint8)` and selector `0xab1bfc946643fef3ad4e44e43027bc5acf93baafa545c3ed0370761d4bb425da`.
```solidity
event PermissionCheckAddedWithType(address indexed check, RequireCompositeModule.CheckType indexed checkType);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PermissionCheckAddedWithType {
        #[allow(missing_docs)]
        pub check: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub checkType: <RequireCompositeModule::CheckType as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for PermissionCheckAddedWithType {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                RequireCompositeModule::CheckType,
            );
            const SIGNATURE: &'static str = "PermissionCheckAddedWithType(address,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                171u8, 27u8, 252u8, 148u8, 102u8, 67u8, 254u8, 243u8, 173u8, 78u8, 68u8,
                228u8, 48u8, 39u8, 188u8, 90u8, 207u8, 147u8, 186u8, 175u8, 165u8, 69u8,
                195u8, 237u8, 3u8, 112u8, 118u8, 29u8, 75u8, 180u8, 37u8, 218u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    check: topics.1,
                    checkType: topics.2,
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
                (Self::SIGNATURE_HASH.into(), self.check.clone(), self.checkType.clone())
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
                    &self.check,
                );
                out[2usize] = <RequireCompositeModule::CheckType as alloy_sol_types::EventTopic>::encode_topic(
                    &self.checkType,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PermissionCheckAddedWithType {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PermissionCheckAddedWithType> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &PermissionCheckAddedWithType,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PermissionCheckRemoved(address)` and selector `0xb5d68ca46372bbe6ec138d3d0423608269b3117496a46268f86080cdbcbea9be`.
```solidity
event PermissionCheckRemoved(address indexed check);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PermissionCheckRemoved {
        #[allow(missing_docs)]
        pub check: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for PermissionCheckRemoved {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "PermissionCheckRemoved(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                181u8, 214u8, 140u8, 164u8, 99u8, 114u8, 187u8, 230u8, 236u8, 19u8,
                141u8, 61u8, 4u8, 35u8, 96u8, 130u8, 105u8, 179u8, 17u8, 116u8, 150u8,
                164u8, 98u8, 104u8, 248u8, 96u8, 128u8, 205u8, 188u8, 190u8, 169u8, 190u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { check: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.check.clone())
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
                    &self.check,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PermissionCheckRemoved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PermissionCheckRemoved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PermissionCheckRemoved) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log(string)` and selector `0x41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50`.
```solidity
event log(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8,
                214u8, 9u8, 203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8,
                202u8, 240u8, 233u8, 177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_address(address)` and selector `0x7ae74c527414ae135fd97047b12921a5ec3911b804197855d67e25c7b75ee6f3`.
```solidity
event log_address(address);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_address {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_address {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_address(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8,
                71u8, 177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8,
                120u8, 85u8, 214u8, 126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_array(uint256[])` and selector `0xfb102865d50addddf69da9b5aa1bced66c80cf869a5c8d0471a467e18ce9cab1`.
```solidity
event log_array(uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_0 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8,
                181u8, 170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8,
                92u8, 141u8, 4u8, 113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_array(int256[])` and selector `0x890a82679b470f2bd82816ed9b161f97d8b967f37fa3647c21d5bf39749e2dd5`.
```solidity
event log_array(int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_1 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8,
                237u8, 155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8,
                163u8, 100u8, 124u8, 33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_array(address[])` and selector `0x40e1840f5769073d61bd01372d9b75baa9842d5629a0c99ff103be1178a8e9e2`.
```solidity
event log_array(address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_2 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl alloy_sol_types::SolEvent for log_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8,
                45u8, 155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8,
                159u8, 241u8, 3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_bytes(bytes)` and selector `0x23b62ad0584d24a75f0bf3560391ef5659ec6db1269c56e11aa241d637f19b20`.
```solidity
event log_bytes(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for log_bytes {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8,
                86u8, 3u8, 145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8,
                86u8, 225u8, 26u8, 162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_bytes32(bytes32)` and selector `0xe81699b85113eea1c73e10588b2b035e55893369632173afd43feb192fac64e3`.
```solidity
event log_bytes32(bytes32);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes32 {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_bytes32 {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes32(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8,
                88u8, 139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8,
                115u8, 175u8, 212u8, 63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_int(int256)` and selector `0x0eb5d52624c8d28ada9fc55a8c502ed5aa3fbe2fb6e91b71b5f376882b1d2fb8`.
```solidity
event log_int(int256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_int {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::I256,
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
        impl alloy_sol_types::SolEvent for log_int {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_int(int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8,
                90u8, 140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8,
                27u8, 113u8, 181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_address(string,address)` and selector `0x9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f`.
```solidity
event log_named_address(string key, address val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_address {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for log_named_address {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_address(string,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8,
                16u8, 143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8,
                67u8, 122u8, 97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_array(string,uint256[])` and selector `0x00aaa39c9ffb5f567a4534380c737075702e1f7f14107fc95328e3b56c0325fb`.
```solidity
event log_named_array(string key, uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_0 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8,
                56u8, 12u8, 115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8,
                127u8, 201u8, 83u8, 40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_array(string,int256[])` and selector `0xa73eda09662f46dde729be4611385ff34fe6c44fbbc6f7e17b042b59a3445b57`.
```solidity
event log_named_array(string key, int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_1 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8,
                70u8, 17u8, 56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8,
                247u8, 225u8, 123u8, 4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_array(string,address[])` and selector `0x3bcfb2ae2e8d132dd1fce7cf278a9a19756a9fceabe470df3bdabb4bc577d1bd`.
```solidity
event log_named_array(string key, address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_2 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl alloy_sol_types::SolEvent for log_named_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8,
                207u8, 39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8,
                228u8, 112u8, 223u8, 59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_bytes(string,bytes)` and selector `0xd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf18`.
```solidity
event log_named_bytes(string key, bytes val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for log_named_bytes {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes(string,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8,
                217u8, 79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8,
                89u8, 79u8, 213u8, 99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_bytes32(string,bytes32)` and selector `0xafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f99`.
```solidity
event log_named_bytes32(string key, bytes32 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes32 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for log_named_bytes32 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes32(string,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8,
                111u8, 146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8,
                221u8, 184u8, 211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_decimal_int(string,int256,uint256)` and selector `0x5da6ce9d51151ba10c09a559ef24d520b9dac5c5b8810ae8434e4d0d86411a95`.
```solidity
event log_named_decimal_int(string key, int256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_decimal_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_int(string,int256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8,
                89u8, 239u8, 36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8,
                10u8, 232u8, 67u8, 78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
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
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_decimal_uint(string,uint256,uint256)` and selector `0xeb8ba43ced7537421946bd43e828b8b2b8428927aa8f801c13d934bf11aca57b`.
```solidity
event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_decimal_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_uint(string,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8,
                67u8, 232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8,
                128u8, 28u8, 19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
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
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_int(string,int256)` and selector `0x2fe632779174374378442a8e978bccfbdcc1d6b2b0d81f7e8eb776ab2286f168`.
```solidity
event log_named_int(string key, int256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
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
        impl alloy_sol_types::SolEvent for log_named_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_int(string,int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8,
                142u8, 151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8,
                216u8, 31u8, 126u8, 142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_string(string,string)` and selector `0x280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf3583`.
```solidity
event log_named_string(string key, string val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_string {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for log_named_string {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_string(string,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8,
                101u8, 141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8,
                243u8, 120u8, 83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_uint(string,uint256)` and selector `0xb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8`.
```solidity
event log_named_uint(string key, uint256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_uint(string,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8,
                253u8, 68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8,
                197u8, 108u8, 129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_string(string)` and selector `0x0b2e13ff20ac7b474198655583edf70dedd2c1dc980e329c4fbb2fc0748b796b`.
```solidity
event log_string(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_string {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_string {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_string(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8,
                85u8, 131u8, 237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8,
                50u8, 156u8, 79u8, 187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_uint(uint256)` and selector `0x2cab9790510fd8bdfbd2115288db33fec66691d476efc5427cfd4c0969301755`.
```solidity
event log_uint(uint256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_uint {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_uint {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_uint(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8,
                82u8, 136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8,
                239u8, 197u8, 66u8, 124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `logs(bytes)` and selector `0xe7950ede0394b9f2ce4a5a1bf5a7e1852411f7e6661b4308c913c4bfd11027e4`.
```solidity
event logs(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct logs {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for logs {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "logs(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8,
                27u8, 245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8,
                67u8, 8u8, 201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for logs {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&logs> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &logs) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `IS_TEST()` and selector `0xfa7626d4`.
```solidity
function IS_TEST() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`IS_TEST()`](IS_TESTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTReturn {
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
            impl ::core::convert::From<IS_TESTCall> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<IS_TESTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for IS_TESTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IS_TEST()";
            const SELECTOR: [u8; 4] = [250u8, 118u8, 38u8, 212u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: IS_TESTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: IS_TESTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `admin()` and selector `0xf851a440`.
```solidity
function admin() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct adminCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`admin()`](adminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct adminReturn {
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
            impl ::core::convert::From<adminCall> for UnderlyingRustTuple<'_> {
                fn from(value: adminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for adminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<adminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: adminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for adminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for adminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "admin()";
            const SELECTOR: [u8; 4] = [248u8, 81u8, 164u8, 64u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: adminReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: adminReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `alwaysAllowModule()` and selector `0x6b44511a`.
```solidity
function alwaysAllowModule() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct alwaysAllowModuleCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`alwaysAllowModule()`](alwaysAllowModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct alwaysAllowModuleReturn {
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
            impl ::core::convert::From<alwaysAllowModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: alwaysAllowModuleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for alwaysAllowModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<alwaysAllowModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: alwaysAllowModuleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for alwaysAllowModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for alwaysAllowModuleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "alwaysAllowModule()";
            const SELECTOR: [u8; 4] = [107u8, 68u8, 81u8, 26u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: alwaysAllowModuleReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: alwaysAllowModuleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `alwaysDenyModule()` and selector `0xa9c4d104`.
```solidity
function alwaysDenyModule() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct alwaysDenyModuleCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`alwaysDenyModule()`](alwaysDenyModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct alwaysDenyModuleReturn {
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
            impl ::core::convert::From<alwaysDenyModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: alwaysDenyModuleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for alwaysDenyModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<alwaysDenyModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: alwaysDenyModuleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for alwaysDenyModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for alwaysDenyModuleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "alwaysDenyModule()";
            const SELECTOR: [u8; 4] = [169u8, 196u8, 209u8, 4u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: alwaysDenyModuleReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: alwaysDenyModuleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `compositeModule()` and selector `0x3f6afde6`.
```solidity
function compositeModule() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct compositeModuleCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`compositeModule()`](compositeModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct compositeModuleReturn {
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
            impl ::core::convert::From<compositeModuleCall> for UnderlyingRustTuple<'_> {
                fn from(value: compositeModuleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for compositeModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<compositeModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: compositeModuleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for compositeModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for compositeModuleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "compositeModule()";
            const SELECTOR: [u8; 4] = [63u8, 106u8, 253u8, 230u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: compositeModuleReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: compositeModuleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `configModule1()` and selector `0x315f4118`.
```solidity
function configModule1() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configModule1Call;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`configModule1()`](configModule1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configModule1Return {
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
            impl ::core::convert::From<configModule1Call> for UnderlyingRustTuple<'_> {
                fn from(value: configModule1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configModule1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<configModule1Return> for UnderlyingRustTuple<'_> {
                fn from(value: configModule1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configModule1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for configModule1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "configModule1()";
            const SELECTOR: [u8; 4] = [49u8, 95u8, 65u8, 24u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: configModule1Return = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: configModule1Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `configModule2()` and selector `0x18563fca`.
```solidity
function configModule2() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configModule2Call;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`configModule2()`](configModule2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configModule2Return {
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
            impl ::core::convert::From<configModule2Call> for UnderlyingRustTuple<'_> {
                fn from(value: configModule2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configModule2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<configModule2Return> for UnderlyingRustTuple<'_> {
                fn from(value: configModule2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configModule2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for configModule2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "configModule2()";
            const SELECTOR: [u8; 4] = [24u8, 86u8, 63u8, 202u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: configModule2Return = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: configModule2Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeArtifacts()` and selector `0xb5508aa9`.
```solidity
function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeArtifacts()`](excludeArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsReturn {
        #[allow(missing_docs)]
        pub excludedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeArtifactsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<excludeArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsReturn) -> Self {
                    (value.excludedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::String,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeArtifacts()";
            const SELECTOR: [u8; 4] = [181u8, 80u8, 138u8, 169u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: excludeArtifactsReturn = r.into();
                        r.excludedArtifacts_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: excludeArtifactsReturn = r.into();
                        r.excludedArtifacts_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeContracts()` and selector `0xe20c9f71`.
```solidity
function excludeContracts() external view returns (address[] memory excludedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeContracts()`](excludeContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsReturn {
        #[allow(missing_docs)]
        pub excludedContracts_: alloy::sol_types::private::Vec<
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeContractsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<excludeContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsReturn) -> Self {
                    (value.excludedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeContracts()";
            const SELECTOR: [u8; 4] = [226u8, 12u8, 159u8, 113u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: excludeContractsReturn = r.into();
                        r.excludedContracts_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: excludeContractsReturn = r.into();
                        r.excludedContracts_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeSelectors()` and selector `0xb0464fdc`.
```solidity
function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeSelectors()`](excludeSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsReturn {
        #[allow(missing_docs)]
        pub excludedSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<excludeSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsReturn) -> Self {
                    (value.excludedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSelectors()";
            const SELECTOR: [u8; 4] = [176u8, 70u8, 79u8, 220u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StdInvariant::FuzzSelector,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: excludeSelectorsReturn = r.into();
                        r.excludedSelectors_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: excludeSelectorsReturn = r.into();
                        r.excludedSelectors_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeSenders()` and selector `0x1ed7831c`.
```solidity
function excludeSenders() external view returns (address[] memory excludedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeSenders()`](excludeSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersReturn {
        #[allow(missing_docs)]
        pub excludedSenders_: alloy::sol_types::private::Vec<
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<excludeSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersReturn) -> Self {
                    (value.excludedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { excludedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSenders()";
            const SELECTOR: [u8; 4] = [30u8, 215u8, 131u8, 28u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: excludeSendersReturn = r.into();
                        r.excludedSenders_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: excludeSendersReturn = r.into();
                        r.excludedSenders_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `failed()` and selector `0xba414fa6`.
```solidity
function failed() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`failed()`](failedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedReturn {
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
            impl ::core::convert::From<failedCall> for UnderlyingRustTuple<'_> {
                fn from(value: failedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<failedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: failedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for failedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "failed()";
            const SELECTOR: [u8; 4] = [186u8, 65u8, 79u8, 166u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: failedReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: failedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setUp()` and selector `0x0a9254e4`.
```solidity
function setUp() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpCall;
    ///Container type for the return parameters of the [`setUp()`](setUpCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpReturn {}
    #[allow(
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
            impl ::core::convert::From<setUpCall> for UnderlyingRustTuple<'_> {
                fn from(value: setUpCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<setUpReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setUpReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setUpReturn {
            fn _tokenize(
                &self,
            ) -> <setUpCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setUpCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setUpReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setUp()";
            const SELECTOR: [u8; 4] = [10u8, 146u8, 84u8, 228u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setUpReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`.
```solidity
function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetArtifactSelectors()`](targetArtifactSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsReturn {
        #[allow(missing_docs)]
        pub targetedArtifactSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetArtifactSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsReturn) -> Self {
                    (value.targetedArtifactSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifactSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifactSelectors()";
            const SELECTOR: [u8; 4] = [102u8, 217u8, 169u8, 160u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StdInvariant::FuzzArtifactSelector,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetArtifactSelectorsReturn = r.into();
                        r.targetedArtifactSelectors_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: targetArtifactSelectorsReturn = r.into();
                        r.targetedArtifactSelectors_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetArtifacts()` and selector `0x85226c81`.
```solidity
function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetArtifacts()`](targetArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsReturn {
        #[allow(missing_docs)]
        pub targetedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<targetArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsReturn) -> Self {
                    (value.targetedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::String,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifacts()";
            const SELECTOR: [u8; 4] = [133u8, 34u8, 108u8, 129u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetArtifactsReturn = r.into();
                        r.targetedArtifacts_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: targetArtifactsReturn = r.into();
                        r.targetedArtifacts_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetContracts()` and selector `0x3f7286f4`.
```solidity
function targetContracts() external view returns (address[] memory targetedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetContracts()`](targetContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsReturn {
        #[allow(missing_docs)]
        pub targetedContracts_: alloy::sol_types::private::Vec<
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetContractsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<targetContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsReturn) -> Self {
                    (value.targetedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetContracts()";
            const SELECTOR: [u8; 4] = [63u8, 114u8, 134u8, 244u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetContractsReturn = r.into();
                        r.targetedContracts_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: targetContractsReturn = r.into();
                        r.targetedContracts_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetInterfaces()` and selector `0x2ade3880`.
```solidity
function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetInterfaces()`](targetInterfacesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesReturn {
        #[allow(missing_docs)]
        pub targetedInterfaces_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetInterfacesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetInterfacesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesReturn) -> Self {
                    (value.targetedInterfaces_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedInterfaces_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetInterfacesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetInterfaces()";
            const SELECTOR: [u8; 4] = [42u8, 222u8, 56u8, 128u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StdInvariant::FuzzInterface,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetInterfacesReturn = r.into();
                        r.targetedInterfaces_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: targetInterfacesReturn = r.into();
                        r.targetedInterfaces_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetSelectors()` and selector `0x916a17c6`.
```solidity
function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetSelectors()`](targetSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsReturn {
        #[allow(missing_docs)]
        pub targetedSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetSelectorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsReturn) -> Self {
                    (value.targetedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSelectors()";
            const SELECTOR: [u8; 4] = [145u8, 106u8, 23u8, 198u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StdInvariant::FuzzSelector,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetSelectorsReturn = r.into();
                        r.targetedSelectors_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: targetSelectorsReturn = r.into();
                        r.targetedSelectors_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetSenders()` and selector `0x3e5e3c23`.
```solidity
function targetSenders() external view returns (address[] memory targetedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetSenders()`](targetSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersReturn {
        #[allow(missing_docs)]
        pub targetedSenders_: alloy::sol_types::private::Vec<
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<targetSendersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersReturn) -> Self {
                    (value.targetedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { targetedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSenders()";
            const SELECTOR: [u8; 4] = [62u8, 94u8, 60u8, 35u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetSendersReturn = r.into();
                        r.targetedSenders_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: targetSendersReturn = r.into();
                        r.targetedSenders_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testAddPermissionCheck()` and selector `0xc166e043`.
```solidity
function testAddPermissionCheck() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testAddPermissionCheckCall;
    ///Container type for the return parameters of the [`testAddPermissionCheck()`](testAddPermissionCheckCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testAddPermissionCheckReturn {}
    #[allow(
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
            impl ::core::convert::From<testAddPermissionCheckCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testAddPermissionCheckCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testAddPermissionCheckCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testAddPermissionCheckReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testAddPermissionCheckReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testAddPermissionCheckReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testAddPermissionCheckReturn {
            fn _tokenize(
                &self,
            ) -> <testAddPermissionCheckCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testAddPermissionCheckCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testAddPermissionCheckReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testAddPermissionCheck()";
            const SELECTOR: [u8; 4] = [193u8, 102u8, 224u8, 67u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testAddPermissionCheckReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testAddPermissionCheckWithType()` and selector `0x0c22bdf8`.
```solidity
function testAddPermissionCheckWithType() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testAddPermissionCheckWithTypeCall;
    ///Container type for the return parameters of the [`testAddPermissionCheckWithType()`](testAddPermissionCheckWithTypeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testAddPermissionCheckWithTypeReturn {}
    #[allow(
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
            impl ::core::convert::From<testAddPermissionCheckWithTypeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testAddPermissionCheckWithTypeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testAddPermissionCheckWithTypeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testAddPermissionCheckWithTypeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testAddPermissionCheckWithTypeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testAddPermissionCheckWithTypeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testAddPermissionCheckWithTypeReturn {
            fn _tokenize(
                &self,
            ) -> <testAddPermissionCheckWithTypeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testAddPermissionCheckWithTypeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testAddPermissionCheckWithTypeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testAddPermissionCheckWithType()";
            const SELECTOR: [u8; 4] = [12u8, 34u8, 189u8, 248u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testAddPermissionCheckWithTypeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testCannotAddSameCheckTwice()` and selector `0x9abdf47a`.
```solidity
function testCannotAddSameCheckTwice() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCannotAddSameCheckTwiceCall;
    ///Container type for the return parameters of the [`testCannotAddSameCheckTwice()`](testCannotAddSameCheckTwiceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCannotAddSameCheckTwiceReturn {}
    #[allow(
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
            impl ::core::convert::From<testCannotAddSameCheckTwiceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCannotAddSameCheckTwiceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCannotAddSameCheckTwiceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testCannotAddSameCheckTwiceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCannotAddSameCheckTwiceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCannotAddSameCheckTwiceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testCannotAddSameCheckTwiceReturn {
            fn _tokenize(
                &self,
            ) -> <testCannotAddSameCheckTwiceCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testCannotAddSameCheckTwiceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testCannotAddSameCheckTwiceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testCannotAddSameCheckTwice()";
            const SELECTOR: [u8; 4] = [154u8, 189u8, 244u8, 122u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testCannotAddSameCheckTwiceReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testCannotAddZeroAddress()` and selector `0x9732e261`.
```solidity
function testCannotAddZeroAddress() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCannotAddZeroAddressCall;
    ///Container type for the return parameters of the [`testCannotAddZeroAddress()`](testCannotAddZeroAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCannotAddZeroAddressReturn {}
    #[allow(
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
            impl ::core::convert::From<testCannotAddZeroAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCannotAddZeroAddressCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCannotAddZeroAddressCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testCannotAddZeroAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCannotAddZeroAddressReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCannotAddZeroAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testCannotAddZeroAddressReturn {
            fn _tokenize(
                &self,
            ) -> <testCannotAddZeroAddressCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testCannotAddZeroAddressCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testCannotAddZeroAddressReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testCannotAddZeroAddress()";
            const SELECTOR: [u8; 4] = [151u8, 50u8, 226u8, 97u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testCannotAddZeroAddressReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testCannotRemoveNonExistentCheck()` and selector `0x39fe398d`.
```solidity
function testCannotRemoveNonExistentCheck() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCannotRemoveNonExistentCheckCall;
    ///Container type for the return parameters of the [`testCannotRemoveNonExistentCheck()`](testCannotRemoveNonExistentCheckCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCannotRemoveNonExistentCheckReturn {}
    #[allow(
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
            impl ::core::convert::From<testCannotRemoveNonExistentCheckCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCannotRemoveNonExistentCheckCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCannotRemoveNonExistentCheckCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testCannotRemoveNonExistentCheckReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCannotRemoveNonExistentCheckReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCannotRemoveNonExistentCheckReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testCannotRemoveNonExistentCheckReturn {
            fn _tokenize(
                &self,
            ) -> <testCannotRemoveNonExistentCheckCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testCannotRemoveNonExistentCheckCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testCannotRemoveNonExistentCheckReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testCannotRemoveNonExistentCheck()";
            const SELECTOR: [u8; 4] = [57u8, 254u8, 57u8, 141u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testCannotRemoveNonExistentCheckReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testCannotUpdateTypeForNonExistentCheck()` and selector `0x13a7f30e`.
```solidity
function testCannotUpdateTypeForNonExistentCheck() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCannotUpdateTypeForNonExistentCheckCall;
    ///Container type for the return parameters of the [`testCannotUpdateTypeForNonExistentCheck()`](testCannotUpdateTypeForNonExistentCheckCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCannotUpdateTypeForNonExistentCheckReturn {}
    #[allow(
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
            impl ::core::convert::From<testCannotUpdateTypeForNonExistentCheckCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCannotUpdateTypeForNonExistentCheckCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCannotUpdateTypeForNonExistentCheckCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testCannotUpdateTypeForNonExistentCheckReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCannotUpdateTypeForNonExistentCheckReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCannotUpdateTypeForNonExistentCheckReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testCannotUpdateTypeForNonExistentCheckReturn {
            fn _tokenize(
                &self,
            ) -> <testCannotUpdateTypeForNonExistentCheckCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testCannotUpdateTypeForNonExistentCheckCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testCannotUpdateTypeForNonExistentCheckReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testCannotUpdateTypeForNonExistentCheck()";
            const SELECTOR: [u8; 4] = [19u8, 167u8, 243u8, 14u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testCannotUpdateTypeForNonExistentCheckReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testConstructor()` and selector `0xc2e9f2e4`.
```solidity
function testConstructor() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testConstructorCall;
    ///Container type for the return parameters of the [`testConstructor()`](testConstructorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testConstructorReturn {}
    #[allow(
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
            impl ::core::convert::From<testConstructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: testConstructorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for testConstructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testConstructorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testConstructorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testConstructorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testConstructorReturn {
            fn _tokenize(
                &self,
            ) -> <testConstructorCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testConstructorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testConstructorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testConstructor()";
            const SELECTOR: [u8; 4] = [194u8, 233u8, 242u8, 228u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testConstructorReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testGetAllPermissionChecksWithTypes()` and selector `0x859021a4`.
```solidity
function testGetAllPermissionChecksWithTypes() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testGetAllPermissionChecksWithTypesCall;
    ///Container type for the return parameters of the [`testGetAllPermissionChecksWithTypes()`](testGetAllPermissionChecksWithTypesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testGetAllPermissionChecksWithTypesReturn {}
    #[allow(
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
            impl ::core::convert::From<testGetAllPermissionChecksWithTypesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testGetAllPermissionChecksWithTypesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testGetAllPermissionChecksWithTypesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testGetAllPermissionChecksWithTypesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testGetAllPermissionChecksWithTypesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testGetAllPermissionChecksWithTypesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testGetAllPermissionChecksWithTypesReturn {
            fn _tokenize(
                &self,
            ) -> <testGetAllPermissionChecksWithTypesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testGetAllPermissionChecksWithTypesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testGetAllPermissionChecksWithTypesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testGetAllPermissionChecksWithTypes()";
            const SELECTOR: [u8; 4] = [133u8, 144u8, 33u8, 164u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testGetAllPermissionChecksWithTypesReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testIsAllowedMixedChecksAllPass()` and selector `0x9faae1a9`.
```solidity
function testIsAllowedMixedChecksAllPass() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedMixedChecksAllPassCall;
    ///Container type for the return parameters of the [`testIsAllowedMixedChecksAllPass()`](testIsAllowedMixedChecksAllPassCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedMixedChecksAllPassReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedMixedChecksAllPassCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedMixedChecksAllPassCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedMixedChecksAllPassCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testIsAllowedMixedChecksAllPassReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedMixedChecksAllPassReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedMixedChecksAllPassReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedMixedChecksAllPassReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedMixedChecksAllPassCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedMixedChecksAllPassCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedMixedChecksAllPassReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedMixedChecksAllPass()";
            const SELECTOR: [u8; 4] = [159u8, 170u8, 225u8, 169u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedMixedChecksAllPassReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testIsAllowedMixedChecksAndFailOrPass()` and selector `0x1d759713`.
```solidity
function testIsAllowedMixedChecksAndFailOrPass() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedMixedChecksAndFailOrPassCall;
    ///Container type for the return parameters of the [`testIsAllowedMixedChecksAndFailOrPass()`](testIsAllowedMixedChecksAndFailOrPassCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedMixedChecksAndFailOrPassReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedMixedChecksAndFailOrPassCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedMixedChecksAndFailOrPassCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedMixedChecksAndFailOrPassCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testIsAllowedMixedChecksAndFailOrPassReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedMixedChecksAndFailOrPassReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedMixedChecksAndFailOrPassReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedMixedChecksAndFailOrPassReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedMixedChecksAndFailOrPassCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedMixedChecksAndFailOrPassCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedMixedChecksAndFailOrPassReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedMixedChecksAndFailOrPass()";
            const SELECTOR: [u8; 4] = [29u8, 117u8, 151u8, 19u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedMixedChecksAndFailOrPassReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testIsAllowedMixedChecksAndPassOrFail()` and selector `0x9461922e`.
```solidity
function testIsAllowedMixedChecksAndPassOrFail() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedMixedChecksAndPassOrFailCall;
    ///Container type for the return parameters of the [`testIsAllowedMixedChecksAndPassOrFail()`](testIsAllowedMixedChecksAndPassOrFailCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedMixedChecksAndPassOrFailReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedMixedChecksAndPassOrFailCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedMixedChecksAndPassOrFailCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedMixedChecksAndPassOrFailCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testIsAllowedMixedChecksAndPassOrFailReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedMixedChecksAndPassOrFailReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedMixedChecksAndPassOrFailReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedMixedChecksAndPassOrFailReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedMixedChecksAndPassOrFailCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedMixedChecksAndPassOrFailCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedMixedChecksAndPassOrFailReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedMixedChecksAndPassOrFail()";
            const SELECTOR: [u8; 4] = [148u8, 97u8, 146u8, 46u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedMixedChecksAndPassOrFailReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testIsAllowedNoChecks()` and selector `0x34d923db`.
```solidity
function testIsAllowedNoChecks() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedNoChecksCall;
    ///Container type for the return parameters of the [`testIsAllowedNoChecks()`](testIsAllowedNoChecksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedNoChecksReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedNoChecksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedNoChecksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedNoChecksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testIsAllowedNoChecksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedNoChecksReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedNoChecksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedNoChecksReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedNoChecksCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedNoChecksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedNoChecksReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedNoChecks()";
            const SELECTOR: [u8; 4] = [52u8, 217u8, 35u8, 219u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedNoChecksReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testIsAllowedOnlyAndChecksOneFails()` and selector `0x25388d16`.
```solidity
function testIsAllowedOnlyAndChecksOneFails() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedOnlyAndChecksOneFailsCall;
    ///Container type for the return parameters of the [`testIsAllowedOnlyAndChecksOneFails()`](testIsAllowedOnlyAndChecksOneFailsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedOnlyAndChecksOneFailsReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedOnlyAndChecksOneFailsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedOnlyAndChecksOneFailsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedOnlyAndChecksOneFailsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testIsAllowedOnlyAndChecksOneFailsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedOnlyAndChecksOneFailsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedOnlyAndChecksOneFailsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedOnlyAndChecksOneFailsReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedOnlyAndChecksOneFailsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedOnlyAndChecksOneFailsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedOnlyAndChecksOneFailsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedOnlyAndChecksOneFails()";
            const SELECTOR: [u8; 4] = [37u8, 56u8, 141u8, 22u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedOnlyAndChecksOneFailsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testIsAllowedOnlyAndChecksPass()` and selector `0x005bda97`.
```solidity
function testIsAllowedOnlyAndChecksPass() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedOnlyAndChecksPassCall;
    ///Container type for the return parameters of the [`testIsAllowedOnlyAndChecksPass()`](testIsAllowedOnlyAndChecksPassCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedOnlyAndChecksPassReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedOnlyAndChecksPassCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedOnlyAndChecksPassCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedOnlyAndChecksPassCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testIsAllowedOnlyAndChecksPassReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedOnlyAndChecksPassReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedOnlyAndChecksPassReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedOnlyAndChecksPassReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedOnlyAndChecksPassCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedOnlyAndChecksPassCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedOnlyAndChecksPassReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedOnlyAndChecksPass()";
            const SELECTOR: [u8; 4] = [0u8, 91u8, 218u8, 151u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedOnlyAndChecksPassReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testIsAllowedOnlyOrChecksAllFail()` and selector `0x4412deeb`.
```solidity
function testIsAllowedOnlyOrChecksAllFail() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedOnlyOrChecksAllFailCall;
    ///Container type for the return parameters of the [`testIsAllowedOnlyOrChecksAllFail()`](testIsAllowedOnlyOrChecksAllFailCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedOnlyOrChecksAllFailReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedOnlyOrChecksAllFailCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedOnlyOrChecksAllFailCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedOnlyOrChecksAllFailCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testIsAllowedOnlyOrChecksAllFailReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedOnlyOrChecksAllFailReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedOnlyOrChecksAllFailReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedOnlyOrChecksAllFailReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedOnlyOrChecksAllFailCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedOnlyOrChecksAllFailCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedOnlyOrChecksAllFailReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedOnlyOrChecksAllFail()";
            const SELECTOR: [u8; 4] = [68u8, 18u8, 222u8, 235u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedOnlyOrChecksAllFailReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testIsAllowedOnlyOrChecksOnePass()` and selector `0x70c27dfa`.
```solidity
function testIsAllowedOnlyOrChecksOnePass() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedOnlyOrChecksOnePassCall;
    ///Container type for the return parameters of the [`testIsAllowedOnlyOrChecksOnePass()`](testIsAllowedOnlyOrChecksOnePassCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedOnlyOrChecksOnePassReturn {}
    #[allow(
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
            impl ::core::convert::From<testIsAllowedOnlyOrChecksOnePassCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedOnlyOrChecksOnePassCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedOnlyOrChecksOnePassCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testIsAllowedOnlyOrChecksOnePassReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedOnlyOrChecksOnePassReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedOnlyOrChecksOnePassReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedOnlyOrChecksOnePassReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedOnlyOrChecksOnePassCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedOnlyOrChecksOnePassCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedOnlyOrChecksOnePassReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedOnlyOrChecksOnePass()";
            const SELECTOR: [u8; 4] = [112u8, 194u8, 125u8, 250u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedOnlyOrChecksOnePassReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testModifyChecksDuringExecution()` and selector `0x9fda5caf`.
```solidity
function testModifyChecksDuringExecution() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testModifyChecksDuringExecutionCall;
    ///Container type for the return parameters of the [`testModifyChecksDuringExecution()`](testModifyChecksDuringExecutionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testModifyChecksDuringExecutionReturn {}
    #[allow(
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
            impl ::core::convert::From<testModifyChecksDuringExecutionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testModifyChecksDuringExecutionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testModifyChecksDuringExecutionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testModifyChecksDuringExecutionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testModifyChecksDuringExecutionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testModifyChecksDuringExecutionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testModifyChecksDuringExecutionReturn {
            fn _tokenize(
                &self,
            ) -> <testModifyChecksDuringExecutionCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testModifyChecksDuringExecutionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testModifyChecksDuringExecutionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testModifyChecksDuringExecution()";
            const SELECTOR: [u8; 4] = [159u8, 218u8, 92u8, 175u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testModifyChecksDuringExecutionReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testNoEventWhenTypeNotChanged()` and selector `0xf8189895`.
```solidity
function testNoEventWhenTypeNotChanged() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testNoEventWhenTypeNotChangedCall;
    ///Container type for the return parameters of the [`testNoEventWhenTypeNotChanged()`](testNoEventWhenTypeNotChangedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testNoEventWhenTypeNotChangedReturn {}
    #[allow(
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
            impl ::core::convert::From<testNoEventWhenTypeNotChangedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testNoEventWhenTypeNotChangedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testNoEventWhenTypeNotChangedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testNoEventWhenTypeNotChangedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testNoEventWhenTypeNotChangedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testNoEventWhenTypeNotChangedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testNoEventWhenTypeNotChangedReturn {
            fn _tokenize(
                &self,
            ) -> <testNoEventWhenTypeNotChangedCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testNoEventWhenTypeNotChangedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testNoEventWhenTypeNotChangedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testNoEventWhenTypeNotChanged()";
            const SELECTOR: [u8; 4] = [248u8, 24u8, 152u8, 149u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testNoEventWhenTypeNotChangedReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testOnlyOwnerCanAddChecks()` and selector `0x3e4c68ae`.
```solidity
function testOnlyOwnerCanAddChecks() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testOnlyOwnerCanAddChecksCall;
    ///Container type for the return parameters of the [`testOnlyOwnerCanAddChecks()`](testOnlyOwnerCanAddChecksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testOnlyOwnerCanAddChecksReturn {}
    #[allow(
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
            impl ::core::convert::From<testOnlyOwnerCanAddChecksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testOnlyOwnerCanAddChecksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testOnlyOwnerCanAddChecksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testOnlyOwnerCanAddChecksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testOnlyOwnerCanAddChecksReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testOnlyOwnerCanAddChecksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testOnlyOwnerCanAddChecksReturn {
            fn _tokenize(
                &self,
            ) -> <testOnlyOwnerCanAddChecksCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testOnlyOwnerCanAddChecksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testOnlyOwnerCanAddChecksReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testOnlyOwnerCanAddChecks()";
            const SELECTOR: [u8; 4] = [62u8, 76u8, 104u8, 174u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testOnlyOwnerCanAddChecksReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testRemovePermissionCheck()` and selector `0x32ee689a`.
```solidity
function testRemovePermissionCheck() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRemovePermissionCheckCall;
    ///Container type for the return parameters of the [`testRemovePermissionCheck()`](testRemovePermissionCheckCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRemovePermissionCheckReturn {}
    #[allow(
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
            impl ::core::convert::From<testRemovePermissionCheckCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRemovePermissionCheckCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRemovePermissionCheckCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testRemovePermissionCheckReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRemovePermissionCheckReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRemovePermissionCheckReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testRemovePermissionCheckReturn {
            fn _tokenize(
                &self,
            ) -> <testRemovePermissionCheckCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testRemovePermissionCheckCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testRemovePermissionCheckReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testRemovePermissionCheck()";
            const SELECTOR: [u8; 4] = [50u8, 238u8, 104u8, 154u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testRemovePermissionCheckReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testUpdateCheckType()` and selector `0xf7a4d601`.
```solidity
function testUpdateCheckType() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateCheckTypeCall;
    ///Container type for the return parameters of the [`testUpdateCheckType()`](testUpdateCheckTypeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateCheckTypeReturn {}
    #[allow(
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
            impl ::core::convert::From<testUpdateCheckTypeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateCheckTypeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateCheckTypeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<testUpdateCheckTypeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateCheckTypeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateCheckTypeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testUpdateCheckTypeReturn {
            fn _tokenize(
                &self,
            ) -> <testUpdateCheckTypeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testUpdateCheckTypeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testUpdateCheckTypeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testUpdateCheckType()";
            const SELECTOR: [u8; 4] = [247u8, 164u8, 214u8, 1u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testUpdateCheckTypeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `user1()` and selector `0xac1717b0`.
```solidity
function user1() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct user1Call;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`user1()`](user1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct user1Return {
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
            impl ::core::convert::From<user1Call> for UnderlyingRustTuple<'_> {
                fn from(value: user1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for user1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<user1Return> for UnderlyingRustTuple<'_> {
                fn from(value: user1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for user1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for user1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "user1()";
            const SELECTOR: [u8; 4] = [172u8, 23u8, 23u8, 176u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: user1Return = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: user1Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `user2()` and selector `0xb9edb1af`.
```solidity
function user2() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct user2Call;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`user2()`](user2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct user2Return {
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
            impl ::core::convert::From<user2Call> for UnderlyingRustTuple<'_> {
                fn from(value: user2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for user2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<user2Return> for UnderlyingRustTuple<'_> {
                fn from(value: user2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for user2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for user2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "user2()";
            const SELECTOR: [u8; 4] = [185u8, 237u8, 177u8, 175u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: user2Return = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: user2Return = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`RequireCompositeModuleTest`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum RequireCompositeModuleTestCalls {
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        admin(adminCall),
        #[allow(missing_docs)]
        alwaysAllowModule(alwaysAllowModuleCall),
        #[allow(missing_docs)]
        alwaysDenyModule(alwaysDenyModuleCall),
        #[allow(missing_docs)]
        compositeModule(compositeModuleCall),
        #[allow(missing_docs)]
        configModule1(configModule1Call),
        #[allow(missing_docs)]
        configModule2(configModule2Call),
        #[allow(missing_docs)]
        excludeArtifacts(excludeArtifactsCall),
        #[allow(missing_docs)]
        excludeContracts(excludeContractsCall),
        #[allow(missing_docs)]
        excludeSelectors(excludeSelectorsCall),
        #[allow(missing_docs)]
        excludeSenders(excludeSendersCall),
        #[allow(missing_docs)]
        failed(failedCall),
        #[allow(missing_docs)]
        setUp(setUpCall),
        #[allow(missing_docs)]
        targetArtifactSelectors(targetArtifactSelectorsCall),
        #[allow(missing_docs)]
        targetArtifacts(targetArtifactsCall),
        #[allow(missing_docs)]
        targetContracts(targetContractsCall),
        #[allow(missing_docs)]
        targetInterfaces(targetInterfacesCall),
        #[allow(missing_docs)]
        targetSelectors(targetSelectorsCall),
        #[allow(missing_docs)]
        targetSenders(targetSendersCall),
        #[allow(missing_docs)]
        testAddPermissionCheck(testAddPermissionCheckCall),
        #[allow(missing_docs)]
        testAddPermissionCheckWithType(testAddPermissionCheckWithTypeCall),
        #[allow(missing_docs)]
        testCannotAddSameCheckTwice(testCannotAddSameCheckTwiceCall),
        #[allow(missing_docs)]
        testCannotAddZeroAddress(testCannotAddZeroAddressCall),
        #[allow(missing_docs)]
        testCannotRemoveNonExistentCheck(testCannotRemoveNonExistentCheckCall),
        #[allow(missing_docs)]
        testCannotUpdateTypeForNonExistentCheck(
            testCannotUpdateTypeForNonExistentCheckCall,
        ),
        #[allow(missing_docs)]
        testConstructor(testConstructorCall),
        #[allow(missing_docs)]
        testGetAllPermissionChecksWithTypes(testGetAllPermissionChecksWithTypesCall),
        #[allow(missing_docs)]
        testIsAllowedMixedChecksAllPass(testIsAllowedMixedChecksAllPassCall),
        #[allow(missing_docs)]
        testIsAllowedMixedChecksAndFailOrPass(testIsAllowedMixedChecksAndFailOrPassCall),
        #[allow(missing_docs)]
        testIsAllowedMixedChecksAndPassOrFail(testIsAllowedMixedChecksAndPassOrFailCall),
        #[allow(missing_docs)]
        testIsAllowedNoChecks(testIsAllowedNoChecksCall),
        #[allow(missing_docs)]
        testIsAllowedOnlyAndChecksOneFails(testIsAllowedOnlyAndChecksOneFailsCall),
        #[allow(missing_docs)]
        testIsAllowedOnlyAndChecksPass(testIsAllowedOnlyAndChecksPassCall),
        #[allow(missing_docs)]
        testIsAllowedOnlyOrChecksAllFail(testIsAllowedOnlyOrChecksAllFailCall),
        #[allow(missing_docs)]
        testIsAllowedOnlyOrChecksOnePass(testIsAllowedOnlyOrChecksOnePassCall),
        #[allow(missing_docs)]
        testModifyChecksDuringExecution(testModifyChecksDuringExecutionCall),
        #[allow(missing_docs)]
        testNoEventWhenTypeNotChanged(testNoEventWhenTypeNotChangedCall),
        #[allow(missing_docs)]
        testOnlyOwnerCanAddChecks(testOnlyOwnerCanAddChecksCall),
        #[allow(missing_docs)]
        testRemovePermissionCheck(testRemovePermissionCheckCall),
        #[allow(missing_docs)]
        testUpdateCheckType(testUpdateCheckTypeCall),
        #[allow(missing_docs)]
        user1(user1Call),
        #[allow(missing_docs)]
        user2(user2Call),
    }
    #[automatically_derived]
    impl RequireCompositeModuleTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 91u8, 218u8, 151u8],
            [10u8, 146u8, 84u8, 228u8],
            [12u8, 34u8, 189u8, 248u8],
            [19u8, 167u8, 243u8, 14u8],
            [24u8, 86u8, 63u8, 202u8],
            [29u8, 117u8, 151u8, 19u8],
            [30u8, 215u8, 131u8, 28u8],
            [37u8, 56u8, 141u8, 22u8],
            [42u8, 222u8, 56u8, 128u8],
            [49u8, 95u8, 65u8, 24u8],
            [50u8, 238u8, 104u8, 154u8],
            [52u8, 217u8, 35u8, 219u8],
            [57u8, 254u8, 57u8, 141u8],
            [62u8, 76u8, 104u8, 174u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 106u8, 253u8, 230u8],
            [63u8, 114u8, 134u8, 244u8],
            [68u8, 18u8, 222u8, 235u8],
            [102u8, 217u8, 169u8, 160u8],
            [107u8, 68u8, 81u8, 26u8],
            [112u8, 194u8, 125u8, 250u8],
            [133u8, 34u8, 108u8, 129u8],
            [133u8, 144u8, 33u8, 164u8],
            [145u8, 106u8, 23u8, 198u8],
            [148u8, 97u8, 146u8, 46u8],
            [151u8, 50u8, 226u8, 97u8],
            [154u8, 189u8, 244u8, 122u8],
            [159u8, 170u8, 225u8, 169u8],
            [159u8, 218u8, 92u8, 175u8],
            [169u8, 196u8, 209u8, 4u8],
            [172u8, 23u8, 23u8, 176u8],
            [176u8, 70u8, 79u8, 220u8],
            [181u8, 80u8, 138u8, 169u8],
            [185u8, 237u8, 177u8, 175u8],
            [186u8, 65u8, 79u8, 166u8],
            [193u8, 102u8, 224u8, 67u8],
            [194u8, 233u8, 242u8, 228u8],
            [226u8, 12u8, 159u8, 113u8],
            [247u8, 164u8, 214u8, 1u8],
            [248u8, 24u8, 152u8, 149u8],
            [248u8, 81u8, 164u8, 64u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RequireCompositeModuleTestCalls {
        const NAME: &'static str = "RequireCompositeModuleTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 42usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::admin(_) => <adminCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::alwaysAllowModule(_) => {
                    <alwaysAllowModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::alwaysDenyModule(_) => {
                    <alwaysDenyModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::compositeModule(_) => {
                    <compositeModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::configModule1(_) => {
                    <configModule1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::configModule2(_) => {
                    <configModule2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeArtifacts(_) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeContracts(_) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSelectors(_) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSenders(_) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setUp(_) => <setUpCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::targetArtifactSelectors(_) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifacts(_) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetContracts(_) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetInterfaces(_) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSelectors(_) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSenders(_) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testAddPermissionCheck(_) => {
                    <testAddPermissionCheckCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testAddPermissionCheckWithType(_) => {
                    <testAddPermissionCheckWithTypeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testCannotAddSameCheckTwice(_) => {
                    <testCannotAddSameCheckTwiceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testCannotAddZeroAddress(_) => {
                    <testCannotAddZeroAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testCannotRemoveNonExistentCheck(_) => {
                    <testCannotRemoveNonExistentCheckCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testCannotUpdateTypeForNonExistentCheck(_) => {
                    <testCannotUpdateTypeForNonExistentCheckCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testConstructor(_) => {
                    <testConstructorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testGetAllPermissionChecksWithTypes(_) => {
                    <testGetAllPermissionChecksWithTypesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedMixedChecksAllPass(_) => {
                    <testIsAllowedMixedChecksAllPassCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedMixedChecksAndFailOrPass(_) => {
                    <testIsAllowedMixedChecksAndFailOrPassCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedMixedChecksAndPassOrFail(_) => {
                    <testIsAllowedMixedChecksAndPassOrFailCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedNoChecks(_) => {
                    <testIsAllowedNoChecksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedOnlyAndChecksOneFails(_) => {
                    <testIsAllowedOnlyAndChecksOneFailsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedOnlyAndChecksPass(_) => {
                    <testIsAllowedOnlyAndChecksPassCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedOnlyOrChecksAllFail(_) => {
                    <testIsAllowedOnlyOrChecksAllFailCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedOnlyOrChecksOnePass(_) => {
                    <testIsAllowedOnlyOrChecksOnePassCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testModifyChecksDuringExecution(_) => {
                    <testModifyChecksDuringExecutionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testNoEventWhenTypeNotChanged(_) => {
                    <testNoEventWhenTypeNotChangedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testOnlyOwnerCanAddChecks(_) => {
                    <testOnlyOwnerCanAddChecksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testRemovePermissionCheck(_) => {
                    <testRemovePermissionCheckCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testUpdateCheckType(_) => {
                    <testUpdateCheckTypeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::user1(_) => <user1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::user2(_) => <user2Call as alloy_sol_types::SolCall>::SELECTOR,
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
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls>] = &[
                {
                    fn testIsAllowedOnlyAndChecksPass(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedOnlyAndChecksPassCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedOnlyAndChecksPass,
                            )
                    }
                    testIsAllowedOnlyAndChecksPass
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RequireCompositeModuleTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn testAddPermissionCheckWithType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testAddPermissionCheckWithTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testAddPermissionCheckWithType,
                            )
                    }
                    testAddPermissionCheckWithType
                },
                {
                    fn testCannotUpdateTypeForNonExistentCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testCannotUpdateTypeForNonExistentCheckCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testCannotUpdateTypeForNonExistentCheck,
                            )
                    }
                    testCannotUpdateTypeForNonExistentCheck
                },
                {
                    fn configModule2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <configModule2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::configModule2)
                    }
                    configModule2
                },
                {
                    fn testIsAllowedMixedChecksAndFailOrPass(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedMixedChecksAndFailOrPassCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedMixedChecksAndFailOrPass,
                            )
                    }
                    testIsAllowedMixedChecksAndFailOrPass
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn testIsAllowedOnlyAndChecksOneFails(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedOnlyAndChecksOneFailsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedOnlyAndChecksOneFails,
                            )
                    }
                    testIsAllowedOnlyAndChecksOneFails
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn configModule1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <configModule1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::configModule1)
                    }
                    configModule1
                },
                {
                    fn testRemovePermissionCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testRemovePermissionCheckCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testRemovePermissionCheck,
                            )
                    }
                    testRemovePermissionCheck
                },
                {
                    fn testIsAllowedNoChecks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedNoChecksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::testIsAllowedNoChecks)
                    }
                    testIsAllowedNoChecks
                },
                {
                    fn testCannotRemoveNonExistentCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testCannotRemoveNonExistentCheckCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testCannotRemoveNonExistentCheck,
                            )
                    }
                    testCannotRemoveNonExistentCheck
                },
                {
                    fn testOnlyOwnerCanAddChecks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testOnlyOwnerCanAddChecksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testOnlyOwnerCanAddChecks,
                            )
                    }
                    testOnlyOwnerCanAddChecks
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn compositeModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <compositeModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::compositeModule)
                    }
                    compositeModule
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn testIsAllowedOnlyOrChecksAllFail(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedOnlyOrChecksAllFailCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedOnlyOrChecksAllFail,
                            )
                    }
                    testIsAllowedOnlyOrChecksAllFail
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::targetArtifactSelectors,
                            )
                    }
                    targetArtifactSelectors
                },
                {
                    fn alwaysAllowModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <alwaysAllowModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::alwaysAllowModule)
                    }
                    alwaysAllowModule
                },
                {
                    fn testIsAllowedOnlyOrChecksOnePass(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedOnlyOrChecksOnePassCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedOnlyOrChecksOnePass,
                            )
                    }
                    testIsAllowedOnlyOrChecksOnePass
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn testGetAllPermissionChecksWithTypes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testGetAllPermissionChecksWithTypesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testGetAllPermissionChecksWithTypes,
                            )
                    }
                    testGetAllPermissionChecksWithTypes
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn testIsAllowedMixedChecksAndPassOrFail(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedMixedChecksAndPassOrFailCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedMixedChecksAndPassOrFail,
                            )
                    }
                    testIsAllowedMixedChecksAndPassOrFail
                },
                {
                    fn testCannotAddZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testCannotAddZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testCannotAddZeroAddress,
                            )
                    }
                    testCannotAddZeroAddress
                },
                {
                    fn testCannotAddSameCheckTwice(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testCannotAddSameCheckTwiceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testCannotAddSameCheckTwice,
                            )
                    }
                    testCannotAddSameCheckTwice
                },
                {
                    fn testIsAllowedMixedChecksAllPass(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedMixedChecksAllPassCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedMixedChecksAllPass,
                            )
                    }
                    testIsAllowedMixedChecksAllPass
                },
                {
                    fn testModifyChecksDuringExecution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testModifyChecksDuringExecutionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testModifyChecksDuringExecution,
                            )
                    }
                    testModifyChecksDuringExecution
                },
                {
                    fn alwaysDenyModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <alwaysDenyModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::alwaysDenyModule)
                    }
                    alwaysDenyModule
                },
                {
                    fn user1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <user1Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RequireCompositeModuleTestCalls::user1)
                    }
                    user1
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn user2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <user2Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RequireCompositeModuleTestCalls::user2)
                    }
                    user2
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RequireCompositeModuleTestCalls::failed)
                    }
                    failed
                },
                {
                    fn testAddPermissionCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testAddPermissionCheckCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::testAddPermissionCheck)
                    }
                    testAddPermissionCheck
                },
                {
                    fn testConstructor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testConstructorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::testConstructor)
                    }
                    testConstructor
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn testUpdateCheckType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testUpdateCheckTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::testUpdateCheckType)
                    }
                    testUpdateCheckType
                },
                {
                    fn testNoEventWhenTypeNotChanged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testNoEventWhenTypeNotChangedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testNoEventWhenTypeNotChanged,
                            )
                    }
                    testNoEventWhenTypeNotChanged
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RequireCompositeModuleTestCalls::admin)
                    }
                    admin
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RequireCompositeModuleTestCalls::IS_TEST)
                    }
                    IS_TEST
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
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls>] = &[
                {
                    fn testIsAllowedOnlyAndChecksPass(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedOnlyAndChecksPassCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedOnlyAndChecksPass,
                            )
                    }
                    testIsAllowedOnlyAndChecksPass
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn testAddPermissionCheckWithType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testAddPermissionCheckWithTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testAddPermissionCheckWithType,
                            )
                    }
                    testAddPermissionCheckWithType
                },
                {
                    fn testCannotUpdateTypeForNonExistentCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testCannotUpdateTypeForNonExistentCheckCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testCannotUpdateTypeForNonExistentCheck,
                            )
                    }
                    testCannotUpdateTypeForNonExistentCheck
                },
                {
                    fn configModule2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <configModule2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::configModule2)
                    }
                    configModule2
                },
                {
                    fn testIsAllowedMixedChecksAndFailOrPass(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedMixedChecksAndFailOrPassCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedMixedChecksAndFailOrPass,
                            )
                    }
                    testIsAllowedMixedChecksAndFailOrPass
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn testIsAllowedOnlyAndChecksOneFails(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedOnlyAndChecksOneFailsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedOnlyAndChecksOneFails,
                            )
                    }
                    testIsAllowedOnlyAndChecksOneFails
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn configModule1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <configModule1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::configModule1)
                    }
                    configModule1
                },
                {
                    fn testRemovePermissionCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testRemovePermissionCheckCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testRemovePermissionCheck,
                            )
                    }
                    testRemovePermissionCheck
                },
                {
                    fn testIsAllowedNoChecks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedNoChecksCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::testIsAllowedNoChecks)
                    }
                    testIsAllowedNoChecks
                },
                {
                    fn testCannotRemoveNonExistentCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testCannotRemoveNonExistentCheckCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testCannotRemoveNonExistentCheck,
                            )
                    }
                    testCannotRemoveNonExistentCheck
                },
                {
                    fn testOnlyOwnerCanAddChecks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testOnlyOwnerCanAddChecksCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testOnlyOwnerCanAddChecks,
                            )
                    }
                    testOnlyOwnerCanAddChecks
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn compositeModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <compositeModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::compositeModule)
                    }
                    compositeModule
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn testIsAllowedOnlyOrChecksAllFail(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedOnlyOrChecksAllFailCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedOnlyOrChecksAllFail,
                            )
                    }
                    testIsAllowedOnlyOrChecksAllFail
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::targetArtifactSelectors,
                            )
                    }
                    targetArtifactSelectors
                },
                {
                    fn alwaysAllowModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <alwaysAllowModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::alwaysAllowModule)
                    }
                    alwaysAllowModule
                },
                {
                    fn testIsAllowedOnlyOrChecksOnePass(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedOnlyOrChecksOnePassCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedOnlyOrChecksOnePass,
                            )
                    }
                    testIsAllowedOnlyOrChecksOnePass
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn testGetAllPermissionChecksWithTypes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testGetAllPermissionChecksWithTypesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testGetAllPermissionChecksWithTypes,
                            )
                    }
                    testGetAllPermissionChecksWithTypes
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn testIsAllowedMixedChecksAndPassOrFail(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedMixedChecksAndPassOrFailCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedMixedChecksAndPassOrFail,
                            )
                    }
                    testIsAllowedMixedChecksAndPassOrFail
                },
                {
                    fn testCannotAddZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testCannotAddZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testCannotAddZeroAddress,
                            )
                    }
                    testCannotAddZeroAddress
                },
                {
                    fn testCannotAddSameCheckTwice(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testCannotAddSameCheckTwiceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testCannotAddSameCheckTwice,
                            )
                    }
                    testCannotAddSameCheckTwice
                },
                {
                    fn testIsAllowedMixedChecksAllPass(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testIsAllowedMixedChecksAllPassCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testIsAllowedMixedChecksAllPass,
                            )
                    }
                    testIsAllowedMixedChecksAllPass
                },
                {
                    fn testModifyChecksDuringExecution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testModifyChecksDuringExecutionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testModifyChecksDuringExecution,
                            )
                    }
                    testModifyChecksDuringExecution
                },
                {
                    fn alwaysDenyModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <alwaysDenyModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::alwaysDenyModule)
                    }
                    alwaysDenyModule
                },
                {
                    fn user1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <user1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::user1)
                    }
                    user1
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn user2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <user2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::user2)
                    }
                    user2
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::failed)
                    }
                    failed
                },
                {
                    fn testAddPermissionCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testAddPermissionCheckCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::testAddPermissionCheck)
                    }
                    testAddPermissionCheck
                },
                {
                    fn testConstructor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testConstructorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::testConstructor)
                    }
                    testConstructor
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn testUpdateCheckType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testUpdateCheckTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::testUpdateCheckType)
                    }
                    testUpdateCheckType
                },
                {
                    fn testNoEventWhenTypeNotChanged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <testNoEventWhenTypeNotChangedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireCompositeModuleTestCalls::testNoEventWhenTypeNotChanged,
                            )
                    }
                    testNoEventWhenTypeNotChanged
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::admin)
                    }
                    admin
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireCompositeModuleTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireCompositeModuleTestCalls::IS_TEST)
                    }
                    IS_TEST
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
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::admin(inner) => {
                    <adminCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::alwaysAllowModule(inner) => {
                    <alwaysAllowModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::alwaysDenyModule(inner) => {
                    <alwaysDenyModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::compositeModule(inner) => {
                    <compositeModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::configModule1(inner) => {
                    <configModule1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::configModule2(inner) => {
                    <configModule2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testAddPermissionCheck(inner) => {
                    <testAddPermissionCheckCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testAddPermissionCheckWithType(inner) => {
                    <testAddPermissionCheckWithTypeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testCannotAddSameCheckTwice(inner) => {
                    <testCannotAddSameCheckTwiceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testCannotAddZeroAddress(inner) => {
                    <testCannotAddZeroAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testCannotRemoveNonExistentCheck(inner) => {
                    <testCannotRemoveNonExistentCheckCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testCannotUpdateTypeForNonExistentCheck(inner) => {
                    <testCannotUpdateTypeForNonExistentCheckCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testConstructor(inner) => {
                    <testConstructorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testGetAllPermissionChecksWithTypes(inner) => {
                    <testGetAllPermissionChecksWithTypesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedMixedChecksAllPass(inner) => {
                    <testIsAllowedMixedChecksAllPassCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedMixedChecksAndFailOrPass(inner) => {
                    <testIsAllowedMixedChecksAndFailOrPassCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedMixedChecksAndPassOrFail(inner) => {
                    <testIsAllowedMixedChecksAndPassOrFailCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedNoChecks(inner) => {
                    <testIsAllowedNoChecksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedOnlyAndChecksOneFails(inner) => {
                    <testIsAllowedOnlyAndChecksOneFailsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedOnlyAndChecksPass(inner) => {
                    <testIsAllowedOnlyAndChecksPassCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedOnlyOrChecksAllFail(inner) => {
                    <testIsAllowedOnlyOrChecksAllFailCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedOnlyOrChecksOnePass(inner) => {
                    <testIsAllowedOnlyOrChecksOnePassCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testModifyChecksDuringExecution(inner) => {
                    <testModifyChecksDuringExecutionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testNoEventWhenTypeNotChanged(inner) => {
                    <testNoEventWhenTypeNotChangedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testOnlyOwnerCanAddChecks(inner) => {
                    <testOnlyOwnerCanAddChecksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testRemovePermissionCheck(inner) => {
                    <testRemovePermissionCheckCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testUpdateCheckType(inner) => {
                    <testUpdateCheckTypeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::user1(inner) => {
                    <user1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::user2(inner) => {
                    <user2Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::admin(inner) => {
                    <adminCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::alwaysAllowModule(inner) => {
                    <alwaysAllowModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::alwaysDenyModule(inner) => {
                    <alwaysDenyModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::compositeModule(inner) => {
                    <compositeModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::configModule1(inner) => {
                    <configModule1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::configModule2(inner) => {
                    <configModule2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testAddPermissionCheck(inner) => {
                    <testAddPermissionCheckCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testAddPermissionCheckWithType(inner) => {
                    <testAddPermissionCheckWithTypeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testCannotAddSameCheckTwice(inner) => {
                    <testCannotAddSameCheckTwiceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testCannotAddZeroAddress(inner) => {
                    <testCannotAddZeroAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testCannotRemoveNonExistentCheck(inner) => {
                    <testCannotRemoveNonExistentCheckCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testCannotUpdateTypeForNonExistentCheck(inner) => {
                    <testCannotUpdateTypeForNonExistentCheckCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testConstructor(inner) => {
                    <testConstructorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testGetAllPermissionChecksWithTypes(inner) => {
                    <testGetAllPermissionChecksWithTypesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedMixedChecksAllPass(inner) => {
                    <testIsAllowedMixedChecksAllPassCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedMixedChecksAndFailOrPass(inner) => {
                    <testIsAllowedMixedChecksAndFailOrPassCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedMixedChecksAndPassOrFail(inner) => {
                    <testIsAllowedMixedChecksAndPassOrFailCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedNoChecks(inner) => {
                    <testIsAllowedNoChecksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedOnlyAndChecksOneFails(inner) => {
                    <testIsAllowedOnlyAndChecksOneFailsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedOnlyAndChecksPass(inner) => {
                    <testIsAllowedOnlyAndChecksPassCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedOnlyOrChecksAllFail(inner) => {
                    <testIsAllowedOnlyOrChecksAllFailCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedOnlyOrChecksOnePass(inner) => {
                    <testIsAllowedOnlyOrChecksOnePassCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testModifyChecksDuringExecution(inner) => {
                    <testModifyChecksDuringExecutionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testNoEventWhenTypeNotChanged(inner) => {
                    <testNoEventWhenTypeNotChangedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testOnlyOwnerCanAddChecks(inner) => {
                    <testOnlyOwnerCanAddChecksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testRemovePermissionCheck(inner) => {
                    <testRemovePermissionCheckCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testUpdateCheckType(inner) => {
                    <testUpdateCheckTypeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::user1(inner) => {
                    <user1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::user2(inner) => {
                    <user2Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`RequireCompositeModuleTest`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum RequireCompositeModuleTestEvents {
        #[allow(missing_docs)]
        CheckTypeUpdated(CheckTypeUpdated),
        #[allow(missing_docs)]
        PermissionCheckAdded(PermissionCheckAdded),
        #[allow(missing_docs)]
        PermissionCheckAddedWithType(PermissionCheckAddedWithType),
        #[allow(missing_docs)]
        PermissionCheckRemoved(PermissionCheckRemoved),
        #[allow(missing_docs)]
        log(log),
        #[allow(missing_docs)]
        log_address(log_address),
        #[allow(missing_docs)]
        log_array_0(log_array_0),
        #[allow(missing_docs)]
        log_array_1(log_array_1),
        #[allow(missing_docs)]
        log_array_2(log_array_2),
        #[allow(missing_docs)]
        log_bytes(log_bytes),
        #[allow(missing_docs)]
        log_bytes32(log_bytes32),
        #[allow(missing_docs)]
        log_int(log_int),
        #[allow(missing_docs)]
        log_named_address(log_named_address),
        #[allow(missing_docs)]
        log_named_array_0(log_named_array_0),
        #[allow(missing_docs)]
        log_named_array_1(log_named_array_1),
        #[allow(missing_docs)]
        log_named_array_2(log_named_array_2),
        #[allow(missing_docs)]
        log_named_bytes(log_named_bytes),
        #[allow(missing_docs)]
        log_named_bytes32(log_named_bytes32),
        #[allow(missing_docs)]
        log_named_decimal_int(log_named_decimal_int),
        #[allow(missing_docs)]
        log_named_decimal_uint(log_named_decimal_uint),
        #[allow(missing_docs)]
        log_named_int(log_named_int),
        #[allow(missing_docs)]
        log_named_string(log_named_string),
        #[allow(missing_docs)]
        log_named_uint(log_named_uint),
        #[allow(missing_docs)]
        log_string(log_string),
        #[allow(missing_docs)]
        log_uint(log_uint),
        #[allow(missing_docs)]
        logs(logs),
    }
    #[automatically_derived]
    impl RequireCompositeModuleTestEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8,
                56u8, 12u8, 115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8,
                127u8, 201u8, 83u8, 40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
            ],
            [
                11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8,
                85u8, 131u8, 237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8,
                50u8, 156u8, 79u8, 187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
            ],
            [
                14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8,
                90u8, 140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8,
                27u8, 113u8, 181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
            ],
            [
                35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8,
                86u8, 3u8, 145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8,
                86u8, 225u8, 26u8, 162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
            ],
            [
                40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8,
                101u8, 141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8,
                243u8, 120u8, 83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
            ],
            [
                44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8,
                82u8, 136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8,
                239u8, 197u8, 66u8, 124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
            ],
            [
                47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8,
                142u8, 151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8,
                216u8, 31u8, 126u8, 142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
            ],
            [
                59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8,
                207u8, 39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8,
                228u8, 112u8, 223u8, 59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
            ],
            [
                64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8,
                45u8, 155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8,
                159u8, 241u8, 3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
            ],
            [
                65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8,
                214u8, 9u8, 203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8,
                202u8, 240u8, 233u8, 177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
            ],
            [
                93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8,
                89u8, 239u8, 36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8,
                10u8, 232u8, 67u8, 78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
            ],
            [
                98u8, 16u8, 28u8, 204u8, 193u8, 134u8, 77u8, 52u8, 146u8, 41u8, 0u8,
                112u8, 244u8, 219u8, 241u8, 104u8, 121u8, 222u8, 120u8, 97u8, 172u8,
                181u8, 220u8, 184u8, 24u8, 11u8, 85u8, 210u8, 237u8, 124u8, 215u8, 231u8,
            ],
            [
                122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8,
                71u8, 177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8,
                120u8, 85u8, 214u8, 126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
            ],
            [
                137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8,
                237u8, 155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8,
                163u8, 100u8, 124u8, 33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
            ],
            [
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8,
                16u8, 143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8,
                67u8, 122u8, 97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
            ],
            [
                160u8, 147u8, 136u8, 183u8, 140u8, 26u8, 54u8, 41u8, 108u8, 59u8, 196u8,
                204u8, 122u8, 37u8, 181u8, 118u8, 139u8, 29u8, 158u8, 14u8, 98u8, 142u8,
                213u8, 83u8, 226u8, 108u8, 38u8, 170u8, 237u8, 249u8, 70u8, 34u8,
            ],
            [
                167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8,
                70u8, 17u8, 56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8,
                247u8, 225u8, 123u8, 4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
            ],
            [
                171u8, 27u8, 252u8, 148u8, 102u8, 67u8, 254u8, 243u8, 173u8, 78u8, 68u8,
                228u8, 48u8, 39u8, 188u8, 90u8, 207u8, 147u8, 186u8, 175u8, 165u8, 69u8,
                195u8, 237u8, 3u8, 112u8, 118u8, 29u8, 75u8, 180u8, 37u8, 218u8,
            ],
            [
                175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8,
                111u8, 146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8,
                221u8, 184u8, 211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
            ],
            [
                178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8,
                253u8, 68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8,
                197u8, 108u8, 129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
            ],
            [
                181u8, 214u8, 140u8, 164u8, 99u8, 114u8, 187u8, 230u8, 236u8, 19u8,
                141u8, 61u8, 4u8, 35u8, 96u8, 130u8, 105u8, 179u8, 17u8, 116u8, 150u8,
                164u8, 98u8, 104u8, 248u8, 96u8, 128u8, 205u8, 188u8, 190u8, 169u8, 190u8,
            ],
            [
                210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8,
                217u8, 79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8,
                89u8, 79u8, 213u8, 99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
            ],
            [
                231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8,
                27u8, 245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8,
                67u8, 8u8, 201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
            ],
            [
                232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8,
                88u8, 139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8,
                115u8, 175u8, 212u8, 63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
            ],
            [
                235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8,
                67u8, 232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8,
                128u8, 28u8, 19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
            ],
            [
                251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8,
                181u8, 170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8,
                92u8, 141u8, 4u8, 113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for RequireCompositeModuleTestEvents {
        const NAME: &'static str = "RequireCompositeModuleTestEvents";
        const COUNT: usize = 26usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<CheckTypeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CheckTypeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CheckTypeUpdated)
                }
                Some(
                    <PermissionCheckAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PermissionCheckAdded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PermissionCheckAdded)
                }
                Some(
                    <PermissionCheckAddedWithType as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PermissionCheckAddedWithType as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PermissionCheckAddedWithType)
                }
                Some(
                    <PermissionCheckRemoved as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PermissionCheckRemoved as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PermissionCheckRemoved)
                }
                Some(<log as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::log)
                }
                Some(<log_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_address)
                }
                Some(<log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_array_0)
                }
                Some(<log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_array_1)
                }
                Some(<log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_array_2)
                }
                Some(<log_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_bytes)
                }
                Some(<log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_bytes32)
                }
                Some(<log_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_int as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::log_int)
                }
                Some(
                    <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_address)
                }
                Some(
                    <log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_array_0)
                }
                Some(
                    <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_array_1)
                }
                Some(
                    <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_array_2)
                }
                Some(<log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_bytes)
                }
                Some(
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_bytes32)
                }
                Some(
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_decimal_int)
                }
                Some(
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_decimal_uint)
                }
                Some(<log_named_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_int)
                }
                Some(<log_named_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_string)
                }
                Some(<log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_uint)
                }
                Some(<log_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_string)
                }
                Some(<log_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_uint as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::log_uint)
                }
                Some(<logs as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <logs as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::logs)
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
    impl alloy_sol_types::private::IntoLogData for RequireCompositeModuleTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::CheckTypeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PermissionCheckAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PermissionCheckAddedWithType(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PermissionCheckRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::CheckTypeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PermissionCheckAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PermissionCheckAddedWithType(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PermissionCheckRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`RequireCompositeModuleTest`](self) contract instance.

See the [wrapper's documentation](`RequireCompositeModuleTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> RequireCompositeModuleTestInstance<P, N> {
        RequireCompositeModuleTestInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<RequireCompositeModuleTestInstance<P, N>>,
    > {
        RequireCompositeModuleTestInstance::<P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        RequireCompositeModuleTestInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`RequireCompositeModuleTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`RequireCompositeModuleTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct RequireCompositeModuleTestInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for RequireCompositeModuleTestInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("RequireCompositeModuleTestInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > RequireCompositeModuleTestInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`RequireCompositeModuleTest`](self) contract instance.

See the [wrapper's documentation](`RequireCompositeModuleTestInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<RequireCompositeModuleTestInstance<P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<P, N> {
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
    impl<P: ::core::clone::Clone, N> RequireCompositeModuleTestInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> RequireCompositeModuleTestInstance<P, N> {
            RequireCompositeModuleTestInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > RequireCompositeModuleTestInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<&P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall)
        }
        ///Creates a new call builder for the [`admin`] function.
        pub fn admin(&self) -> alloy_contract::SolCallBuilder<&P, adminCall, N> {
            self.call_builder(&adminCall)
        }
        ///Creates a new call builder for the [`alwaysAllowModule`] function.
        pub fn alwaysAllowModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, alwaysAllowModuleCall, N> {
            self.call_builder(&alwaysAllowModuleCall)
        }
        ///Creates a new call builder for the [`alwaysDenyModule`] function.
        pub fn alwaysDenyModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, alwaysDenyModuleCall, N> {
            self.call_builder(&alwaysDenyModuleCall)
        }
        ///Creates a new call builder for the [`compositeModule`] function.
        pub fn compositeModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, compositeModuleCall, N> {
            self.call_builder(&compositeModuleCall)
        }
        ///Creates a new call builder for the [`configModule1`] function.
        pub fn configModule1(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, configModule1Call, N> {
            self.call_builder(&configModule1Call)
        }
        ///Creates a new call builder for the [`configModule2`] function.
        pub fn configModule2(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, configModule2Call, N> {
            self.call_builder(&configModule2Call)
        }
        ///Creates a new call builder for the [`excludeArtifacts`] function.
        pub fn excludeArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, excludeArtifactsCall, N> {
            self.call_builder(&excludeArtifactsCall)
        }
        ///Creates a new call builder for the [`excludeContracts`] function.
        pub fn excludeContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, excludeContractsCall, N> {
            self.call_builder(&excludeContractsCall)
        }
        ///Creates a new call builder for the [`excludeSelectors`] function.
        pub fn excludeSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, excludeSelectorsCall, N> {
            self.call_builder(&excludeSelectorsCall)
        }
        ///Creates a new call builder for the [`excludeSenders`] function.
        pub fn excludeSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, excludeSendersCall, N> {
            self.call_builder(&excludeSendersCall)
        }
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<&P, failedCall, N> {
            self.call_builder(&failedCall)
        }
        ///Creates a new call builder for the [`setUp`] function.
        pub fn setUp(&self) -> alloy_contract::SolCallBuilder<&P, setUpCall, N> {
            self.call_builder(&setUpCall)
        }
        ///Creates a new call builder for the [`targetArtifactSelectors`] function.
        pub fn targetArtifactSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetArtifactSelectorsCall, N> {
            self.call_builder(&targetArtifactSelectorsCall)
        }
        ///Creates a new call builder for the [`targetArtifacts`] function.
        pub fn targetArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetArtifactsCall, N> {
            self.call_builder(&targetArtifactsCall)
        }
        ///Creates a new call builder for the [`targetContracts`] function.
        pub fn targetContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetContractsCall, N> {
            self.call_builder(&targetContractsCall)
        }
        ///Creates a new call builder for the [`targetInterfaces`] function.
        pub fn targetInterfaces(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetInterfacesCall, N> {
            self.call_builder(&targetInterfacesCall)
        }
        ///Creates a new call builder for the [`targetSelectors`] function.
        pub fn targetSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetSelectorsCall, N> {
            self.call_builder(&targetSelectorsCall)
        }
        ///Creates a new call builder for the [`targetSenders`] function.
        pub fn targetSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetSendersCall, N> {
            self.call_builder(&targetSendersCall)
        }
        ///Creates a new call builder for the [`testAddPermissionCheck`] function.
        pub fn testAddPermissionCheck(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testAddPermissionCheckCall, N> {
            self.call_builder(&testAddPermissionCheckCall)
        }
        ///Creates a new call builder for the [`testAddPermissionCheckWithType`] function.
        pub fn testAddPermissionCheckWithType(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testAddPermissionCheckWithTypeCall, N> {
            self.call_builder(&testAddPermissionCheckWithTypeCall)
        }
        ///Creates a new call builder for the [`testCannotAddSameCheckTwice`] function.
        pub fn testCannotAddSameCheckTwice(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testCannotAddSameCheckTwiceCall, N> {
            self.call_builder(&testCannotAddSameCheckTwiceCall)
        }
        ///Creates a new call builder for the [`testCannotAddZeroAddress`] function.
        pub fn testCannotAddZeroAddress(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testCannotAddZeroAddressCall, N> {
            self.call_builder(&testCannotAddZeroAddressCall)
        }
        ///Creates a new call builder for the [`testCannotRemoveNonExistentCheck`] function.
        pub fn testCannotRemoveNonExistentCheck(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testCannotRemoveNonExistentCheckCall,
            N,
        > {
            self.call_builder(&testCannotRemoveNonExistentCheckCall)
        }
        ///Creates a new call builder for the [`testCannotUpdateTypeForNonExistentCheck`] function.
        pub fn testCannotUpdateTypeForNonExistentCheck(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testCannotUpdateTypeForNonExistentCheckCall,
            N,
        > {
            self.call_builder(&testCannotUpdateTypeForNonExistentCheckCall)
        }
        ///Creates a new call builder for the [`testConstructor`] function.
        pub fn testConstructor(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testConstructorCall, N> {
            self.call_builder(&testConstructorCall)
        }
        ///Creates a new call builder for the [`testGetAllPermissionChecksWithTypes`] function.
        pub fn testGetAllPermissionChecksWithTypes(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testGetAllPermissionChecksWithTypesCall,
            N,
        > {
            self.call_builder(&testGetAllPermissionChecksWithTypesCall)
        }
        ///Creates a new call builder for the [`testIsAllowedMixedChecksAllPass`] function.
        pub fn testIsAllowedMixedChecksAllPass(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedMixedChecksAllPassCall, N> {
            self.call_builder(&testIsAllowedMixedChecksAllPassCall)
        }
        ///Creates a new call builder for the [`testIsAllowedMixedChecksAndFailOrPass`] function.
        pub fn testIsAllowedMixedChecksAndFailOrPass(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testIsAllowedMixedChecksAndFailOrPassCall,
            N,
        > {
            self.call_builder(&testIsAllowedMixedChecksAndFailOrPassCall)
        }
        ///Creates a new call builder for the [`testIsAllowedMixedChecksAndPassOrFail`] function.
        pub fn testIsAllowedMixedChecksAndPassOrFail(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testIsAllowedMixedChecksAndPassOrFailCall,
            N,
        > {
            self.call_builder(&testIsAllowedMixedChecksAndPassOrFailCall)
        }
        ///Creates a new call builder for the [`testIsAllowedNoChecks`] function.
        pub fn testIsAllowedNoChecks(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedNoChecksCall, N> {
            self.call_builder(&testIsAllowedNoChecksCall)
        }
        ///Creates a new call builder for the [`testIsAllowedOnlyAndChecksOneFails`] function.
        pub fn testIsAllowedOnlyAndChecksOneFails(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testIsAllowedOnlyAndChecksOneFailsCall,
            N,
        > {
            self.call_builder(&testIsAllowedOnlyAndChecksOneFailsCall)
        }
        ///Creates a new call builder for the [`testIsAllowedOnlyAndChecksPass`] function.
        pub fn testIsAllowedOnlyAndChecksPass(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedOnlyAndChecksPassCall, N> {
            self.call_builder(&testIsAllowedOnlyAndChecksPassCall)
        }
        ///Creates a new call builder for the [`testIsAllowedOnlyOrChecksAllFail`] function.
        pub fn testIsAllowedOnlyOrChecksAllFail(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testIsAllowedOnlyOrChecksAllFailCall,
            N,
        > {
            self.call_builder(&testIsAllowedOnlyOrChecksAllFailCall)
        }
        ///Creates a new call builder for the [`testIsAllowedOnlyOrChecksOnePass`] function.
        pub fn testIsAllowedOnlyOrChecksOnePass(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testIsAllowedOnlyOrChecksOnePassCall,
            N,
        > {
            self.call_builder(&testIsAllowedOnlyOrChecksOnePassCall)
        }
        ///Creates a new call builder for the [`testModifyChecksDuringExecution`] function.
        pub fn testModifyChecksDuringExecution(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testModifyChecksDuringExecutionCall, N> {
            self.call_builder(&testModifyChecksDuringExecutionCall)
        }
        ///Creates a new call builder for the [`testNoEventWhenTypeNotChanged`] function.
        pub fn testNoEventWhenTypeNotChanged(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testNoEventWhenTypeNotChangedCall, N> {
            self.call_builder(&testNoEventWhenTypeNotChangedCall)
        }
        ///Creates a new call builder for the [`testOnlyOwnerCanAddChecks`] function.
        pub fn testOnlyOwnerCanAddChecks(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testOnlyOwnerCanAddChecksCall, N> {
            self.call_builder(&testOnlyOwnerCanAddChecksCall)
        }
        ///Creates a new call builder for the [`testRemovePermissionCheck`] function.
        pub fn testRemovePermissionCheck(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testRemovePermissionCheckCall, N> {
            self.call_builder(&testRemovePermissionCheckCall)
        }
        ///Creates a new call builder for the [`testUpdateCheckType`] function.
        pub fn testUpdateCheckType(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testUpdateCheckTypeCall, N> {
            self.call_builder(&testUpdateCheckTypeCall)
        }
        ///Creates a new call builder for the [`user1`] function.
        pub fn user1(&self) -> alloy_contract::SolCallBuilder<&P, user1Call, N> {
            self.call_builder(&user1Call)
        }
        ///Creates a new call builder for the [`user2`] function.
        pub fn user2(&self) -> alloy_contract::SolCallBuilder<&P, user2Call, N> {
            self.call_builder(&user2Call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > RequireCompositeModuleTestInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`CheckTypeUpdated`] event.
        pub fn CheckTypeUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, CheckTypeUpdated, N> {
            self.event_filter::<CheckTypeUpdated>()
        }
        ///Creates a new event filter for the [`PermissionCheckAdded`] event.
        pub fn PermissionCheckAdded_filter(
            &self,
        ) -> alloy_contract::Event<&P, PermissionCheckAdded, N> {
            self.event_filter::<PermissionCheckAdded>()
        }
        ///Creates a new event filter for the [`PermissionCheckAddedWithType`] event.
        pub fn PermissionCheckAddedWithType_filter(
            &self,
        ) -> alloy_contract::Event<&P, PermissionCheckAddedWithType, N> {
            self.event_filter::<PermissionCheckAddedWithType>()
        }
        ///Creates a new event filter for the [`PermissionCheckRemoved`] event.
        pub fn PermissionCheckRemoved_filter(
            &self,
        ) -> alloy_contract::Event<&P, PermissionCheckRemoved, N> {
            self.event_filter::<PermissionCheckRemoved>()
        }
        ///Creates a new event filter for the [`log`] event.
        pub fn log_filter(&self) -> alloy_contract::Event<&P, log, N> {
            self.event_filter::<log>()
        }
        ///Creates a new event filter for the [`log_address`] event.
        pub fn log_address_filter(&self) -> alloy_contract::Event<&P, log_address, N> {
            self.event_filter::<log_address>()
        }
        ///Creates a new event filter for the [`log_array_0`] event.
        pub fn log_array_0_filter(&self) -> alloy_contract::Event<&P, log_array_0, N> {
            self.event_filter::<log_array_0>()
        }
        ///Creates a new event filter for the [`log_array_1`] event.
        pub fn log_array_1_filter(&self) -> alloy_contract::Event<&P, log_array_1, N> {
            self.event_filter::<log_array_1>()
        }
        ///Creates a new event filter for the [`log_array_2`] event.
        pub fn log_array_2_filter(&self) -> alloy_contract::Event<&P, log_array_2, N> {
            self.event_filter::<log_array_2>()
        }
        ///Creates a new event filter for the [`log_bytes`] event.
        pub fn log_bytes_filter(&self) -> alloy_contract::Event<&P, log_bytes, N> {
            self.event_filter::<log_bytes>()
        }
        ///Creates a new event filter for the [`log_bytes32`] event.
        pub fn log_bytes32_filter(&self) -> alloy_contract::Event<&P, log_bytes32, N> {
            self.event_filter::<log_bytes32>()
        }
        ///Creates a new event filter for the [`log_int`] event.
        pub fn log_int_filter(&self) -> alloy_contract::Event<&P, log_int, N> {
            self.event_filter::<log_int>()
        }
        ///Creates a new event filter for the [`log_named_address`] event.
        pub fn log_named_address_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_address, N> {
            self.event_filter::<log_named_address>()
        }
        ///Creates a new event filter for the [`log_named_array_0`] event.
        pub fn log_named_array_0_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_array_0, N> {
            self.event_filter::<log_named_array_0>()
        }
        ///Creates a new event filter for the [`log_named_array_1`] event.
        pub fn log_named_array_1_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_array_1, N> {
            self.event_filter::<log_named_array_1>()
        }
        ///Creates a new event filter for the [`log_named_array_2`] event.
        pub fn log_named_array_2_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_array_2, N> {
            self.event_filter::<log_named_array_2>()
        }
        ///Creates a new event filter for the [`log_named_bytes`] event.
        pub fn log_named_bytes_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_bytes, N> {
            self.event_filter::<log_named_bytes>()
        }
        ///Creates a new event filter for the [`log_named_bytes32`] event.
        pub fn log_named_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_bytes32, N> {
            self.event_filter::<log_named_bytes32>()
        }
        ///Creates a new event filter for the [`log_named_decimal_int`] event.
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_decimal_int, N> {
            self.event_filter::<log_named_decimal_int>()
        }
        ///Creates a new event filter for the [`log_named_decimal_uint`] event.
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_decimal_uint, N> {
            self.event_filter::<log_named_decimal_uint>()
        }
        ///Creates a new event filter for the [`log_named_int`] event.
        pub fn log_named_int_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_int, N> {
            self.event_filter::<log_named_int>()
        }
        ///Creates a new event filter for the [`log_named_string`] event.
        pub fn log_named_string_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_string, N> {
            self.event_filter::<log_named_string>()
        }
        ///Creates a new event filter for the [`log_named_uint`] event.
        pub fn log_named_uint_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_uint, N> {
            self.event_filter::<log_named_uint>()
        }
        ///Creates a new event filter for the [`log_string`] event.
        pub fn log_string_filter(&self) -> alloy_contract::Event<&P, log_string, N> {
            self.event_filter::<log_string>()
        }
        ///Creates a new event filter for the [`log_uint`] event.
        pub fn log_uint_filter(&self) -> alloy_contract::Event<&P, log_uint, N> {
            self.event_filter::<log_uint>()
        }
        ///Creates a new event filter for the [`logs`] event.
        pub fn logs_filter(&self) -> alloy_contract::Event<&P, logs, N> {
            self.event_filter::<logs>()
        }
    }
}
