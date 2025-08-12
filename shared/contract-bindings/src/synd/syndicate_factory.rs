///Module containing a contract's types and functions.
/**

```solidity
library GasTracker {
    struct AggregateGasPeriod { uint256 totalGasUsed; uint256 totalGasCost; uint256 activeAppchains; }
    struct AppchainGasPeriod { uint256 startTimestamp; uint256 endTimestamp; uint256 totalGasUsed; uint256 totalGasCost; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod GasTracker {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct AggregateGasPeriod { uint256 totalGasUsed; uint256 totalGasCost; uint256 activeAppchains; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AggregateGasPeriod {
        #[allow(missing_docs)]
        pub totalGasUsed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalGasCost: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub activeAppchains: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<AggregateGasPeriod> for UnderlyingRustTuple<'_> {
            fn from(value: AggregateGasPeriod) -> Self {
                (value.totalGasUsed, value.totalGasCost, value.activeAppchains)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AggregateGasPeriod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    totalGasUsed: tuple.0,
                    totalGasCost: tuple.1,
                    activeAppchains: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AggregateGasPeriod {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AggregateGasPeriod {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalGasUsed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalGasCost),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.activeAppchains),
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
        impl alloy_sol_types::SolType for AggregateGasPeriod {
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
        impl alloy_sol_types::SolStruct for AggregateGasPeriod {
            const NAME: &'static str = "AggregateGasPeriod";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AggregateGasPeriod(uint256 totalGasUsed,uint256 totalGasCost,uint256 activeAppchains)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.totalGasUsed)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.totalGasCost)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.activeAppchains,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AggregateGasPeriod {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalGasUsed,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalGasCost,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.activeAppchains,
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
                    &rust.totalGasUsed,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalGasCost,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.activeAppchains,
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
struct AppchainGasPeriod { uint256 startTimestamp; uint256 endTimestamp; uint256 totalGasUsed; uint256 totalGasCost; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AppchainGasPeriod {
        #[allow(missing_docs)]
        pub startTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub endTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalGasUsed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalGasCost: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<AppchainGasPeriod> for UnderlyingRustTuple<'_> {
            fn from(value: AppchainGasPeriod) -> Self {
                (
                    value.startTimestamp,
                    value.endTimestamp,
                    value.totalGasUsed,
                    value.totalGasCost,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AppchainGasPeriod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    startTimestamp: tuple.0,
                    endTimestamp: tuple.1,
                    totalGasUsed: tuple.2,
                    totalGasCost: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AppchainGasPeriod {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AppchainGasPeriod {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.endTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalGasUsed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalGasCost),
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
        impl alloy_sol_types::SolType for AppchainGasPeriod {
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
        impl alloy_sol_types::SolStruct for AppchainGasPeriod {
            const NAME: &'static str = "AppchainGasPeriod";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AppchainGasPeriod(uint256 startTimestamp,uint256 endTimestamp,uint256 totalGasUsed,uint256 totalGasCost)",
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
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.endTimestamp)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.totalGasUsed)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.totalGasCost)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AppchainGasPeriod {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.endTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalGasUsed,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalGasCost,
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
                    &rust.startTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.endTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalGasUsed,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalGasCost,
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
    /**Creates a new wrapper around an on-chain [`GasTracker`](self) contract instance.

See the [wrapper's documentation](`GasTrackerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GasTrackerInstance<P, N> {
        GasTrackerInstance::<P, N>::new(address, provider)
    }
    /**A [`GasTracker`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GasTracker`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GasTrackerInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for GasTrackerInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GasTrackerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > GasTrackerInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`GasTracker`](self) contract instance.

See the [wrapper's documentation](`GasTrackerInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> GasTrackerInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> GasTrackerInstance<P, N> {
            GasTrackerInstance {
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
    > GasTrackerInstance<P, N> {
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
    > GasTrackerInstance<P, N> {
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
library GasTracker {
    struct AggregateGasPeriod {
        uint256 totalGasUsed;
        uint256 totalGasCost;
        uint256 activeAppchains;
    }
    struct AppchainGasPeriod {
        uint256 startTimestamp;
        uint256 endTimestamp;
        uint256 totalGasUsed;
        uint256 totalGasCost;
    }
}

interface SyndicateFactory {
    type NamespaceState is uint8;

    error AccessControlBadConfirmation();
    error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
    error ChainIdAlreadyExists();
    error Create2EmptyBytecode();
    error EnforcedPause();
    error ExpectedPause();
    error FailedDeployment();
    error GasTrackingDisabled();
    error InsufficientBalance(uint256 balance, uint256 needed);
    error InvalidAppchainId();
    error InvalidGasAmount();
    error StringsInvalidChar();
    error ZeroAddress();

    event ChainIdManuallyMarked(uint256 indexed chainId);
    event EpochFinalized(uint256 indexed epoch, uint256 totalGasUsed, uint256 activeAppchains);
    event GasTracked(uint256 indexed epoch, uint256 indexed appchainId, uint256 gasUsed, uint256 gasPrice);
    event NamespaceConfigUpdated(uint256 oldNamespacePrefix, uint256 newNamespacePrefix);
    event NewEpochStarted(uint256 indexed epoch, uint256 startTimestamp);
    event Paused(address account);
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event SyndicateSequencingChainCreated(uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress);
    event Unpaused(address account);

    constructor(address admin);

    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function MANAGER_ROLE() external view returns (bytes32);
    function PERIOD_DURATION() external view returns (uint256);
    function aggregatePeriods(uint256) external view returns (uint256 totalGasUsed, uint256 totalGasCost, uint256 activeAppchains);
    function appchainContracts(uint256) external view returns (address);
    function appchainList(uint256) external view returns (uint256);
    function appchainPeriods(uint256, uint256) external view returns (uint256 startTimestamp, uint256 endTimestamp, uint256 totalGasUsed, uint256 totalGasCost);
    function computeSequencingChainAddress(bytes32 salt, uint256 chainId) external view returns (address);
    function createSyndicateSequencingChain(uint256 appchainId, address admin, address permissionModule, bytes32 salt) external returns (address sequencingChain, uint256 actualChainId);
    function cumulativeGasFeesByAppchain(uint256) external view returns (uint256);
    function currentEpoch() external view returns (uint256);
    function currentEpochStart() external view returns (uint256);
    function disableGasTracking() external;
    function enableGasTracking() external;
    function gasTrackingEnabled() external view returns (bool);
    function gasTrackingInitialized() external view returns (bool);
    function getAppchainContract(uint256 chainId) external view returns (address);
    function getAppchainCumulativeFees(uint256 appchainId) external view returns (uint256 totalFees);
    function getAppchainEpochData(uint256 epoch, uint256 appchainId) external view returns (GasTracker.AppchainGasPeriod memory period);
    function getBytecode(uint256 chainId) external pure returns (bytes memory);
    function getCurrentAppchainData(uint256 appchainId) external view returns (GasTracker.AppchainGasPeriod memory period);
    function getCurrentEpochAggregate() external view returns (GasTracker.AggregateGasPeriod memory aggregate);
    function getCurrentEpochTimeRemaining() external view returns (uint256 timeRemaining);
    function getEpochAggregate(uint256 epoch) external view returns (GasTracker.AggregateGasPeriod memory aggregate);
    function getKnownAppchains() external view returns (uint256[] memory appchains);
    function getNextChainId() external view returns (uint256);
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    function getTotalCumulativeFees() external view returns (uint256 totalFees);
    function grantRole(bytes32 role, address account) external;
    function hasRole(bytes32 role, address account) external view returns (bool);
    function isChainIdUsed(uint256 chainId) external view returns (uint256);
    function knownAppchains(uint256) external view returns (bool);
    function markChainIdAsUsed(uint256 chainId) external;
    function namespacePrefix() external view returns (uint256);
    function nextAutoChainId() external view returns (uint256);
    function pause() external;
    function paused() external view returns (bool);
    function renounceRole(bytes32 role, address callerConfirmation) external;
    function revokeRole(bytes32 role, address account) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function totalCumulativeGasFees() external view returns (uint256);
    function trackAppchainGas(uint256 appchainId, uint256 gasUsed) external;
    function trackGas(uint256 appchainId, uint256 gasUsed) external;
    function unpause() external;
    function updateNamespaceConfig(uint256 newPrefix) external;
    function usedChainIds(uint256) external view returns (bool);
    function usedNamespaces(uint256) external view returns (NamespaceState);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "admin",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
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
    "name": "MANAGER_ROLE",
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
    "name": "PERIOD_DURATION",
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
    "name": "aggregatePeriods",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "totalGasUsed",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "totalGasCost",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "activeAppchains",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "appchainContracts",
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
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "appchainList",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "appchainPeriods",
    "inputs": [
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
    "outputs": [
      {
        "name": "startTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "endTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "totalGasUsed",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "totalGasCost",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "computeSequencingChainAddress",
    "inputs": [
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "createSyndicateSequencingChain",
    "inputs": [
      {
        "name": "appchainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "admin",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "permissionModule",
        "type": "address",
        "internalType": "contract IRequirementModule"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "sequencingChain",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "actualChainId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "cumulativeGasFeesByAppchain",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "currentEpoch",
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
    "name": "currentEpochStart",
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
    "name": "disableGasTracking",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "enableGasTracking",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "gasTrackingEnabled",
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
    "name": "gasTrackingInitialized",
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
    "name": "getAppchainContract",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "getAppchainCumulativeFees",
    "inputs": [
      {
        "name": "appchainId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "totalFees",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAppchainEpochData",
    "inputs": [
      {
        "name": "epoch",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "appchainId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "period",
        "type": "tuple",
        "internalType": "struct GasTracker.AppchainGasPeriod",
        "components": [
          {
            "name": "startTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "endTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "totalGasUsed",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "totalGasCost",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getBytecode",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "getCurrentAppchainData",
    "inputs": [
      {
        "name": "appchainId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "period",
        "type": "tuple",
        "internalType": "struct GasTracker.AppchainGasPeriod",
        "components": [
          {
            "name": "startTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "endTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "totalGasUsed",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "totalGasCost",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCurrentEpochAggregate",
    "inputs": [],
    "outputs": [
      {
        "name": "aggregate",
        "type": "tuple",
        "internalType": "struct GasTracker.AggregateGasPeriod",
        "components": [
          {
            "name": "totalGasUsed",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "totalGasCost",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "activeAppchains",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCurrentEpochTimeRemaining",
    "inputs": [],
    "outputs": [
      {
        "name": "timeRemaining",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getEpochAggregate",
    "inputs": [
      {
        "name": "epoch",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "aggregate",
        "type": "tuple",
        "internalType": "struct GasTracker.AggregateGasPeriod",
        "components": [
          {
            "name": "totalGasUsed",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "totalGasCost",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "activeAppchains",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getKnownAppchains",
    "inputs": [],
    "outputs": [
      {
        "name": "appchains",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getNextChainId",
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
    "name": "getTotalCumulativeFees",
    "inputs": [],
    "outputs": [
      {
        "name": "totalFees",
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
    "name": "isChainIdUsed",
    "inputs": [
      {
        "name": "chainId",
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
    "name": "knownAppchains",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "markChainIdAsUsed",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "namespacePrefix",
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
    "name": "nextAutoChainId",
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
    "name": "pause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
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
    "name": "totalCumulativeGasFees",
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
    "name": "trackAppchainGas",
    "inputs": [
      {
        "name": "appchainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "gasUsed",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "trackGas",
    "inputs": [
      {
        "name": "appchainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "gasUsed",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unpause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateNamespaceConfig",
    "inputs": [
      {
        "name": "newPrefix",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "usedChainIds",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "usedNamespaces",
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
        "type": "uint8",
        "internalType": "enum NamespaceState"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "ChainIdManuallyMarked",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EpochFinalized",
    "inputs": [
      {
        "name": "epoch",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "totalGasUsed",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "activeAppchains",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "GasTracked",
    "inputs": [
      {
        "name": "epoch",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "appchainId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "gasUsed",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "gasPrice",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NamespaceConfigUpdated",
    "inputs": [
      {
        "name": "oldNamespacePrefix",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newNamespacePrefix",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NewEpochStarted",
    "inputs": [
      {
        "name": "epoch",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "startTimestamp",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
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
    "name": "SyndicateSequencingChainCreated",
    "inputs": [
      {
        "name": "appchainId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "sequencingChainAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "permissionModuleAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
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
    "name": "ChainIdAlreadyExists",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Create2EmptyBytecode",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EnforcedPause",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpectedPause",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FailedDeployment",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GasTrackingDisabled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientBalance",
    "inputs": [
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
    "name": "InvalidAppchainId",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidGasAmount",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StringsInvalidChar",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroAddress",
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
pub mod SyndicateFactory {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60803461019757601f61318e38819003918201601f19168301916001600160401b0383118484101761019b5780849260209460405283398101031261019757516001600160a01b0381168082036101975760ff196001541660015561010061ff0019600454161760045515610188578061007b610081926101af565b50610225565b506101fe5f52600e6020525f51602061312e5f395f51905f525460ff1660038110156101745760026100b49114156102b8565b6101fe5f52600e6020525f51602061312e5f395f51905f52805460ff191660011790556033805b61012d577fd9e1239177bfbd2aebf5d0f20fc075e6df5a502c59d121acd57342c783e313646040600b546101fe600b5581519081526101fe6020820152a16001600c55604051612e0990816103058239f35b805f52600e60205260ff60405f205416906003821015610174576101566001600a9314156102b8565b805f52600e60205260405f20600260ff1982541617905504806100db565b634e487b7160e01b5f52602160045260245ffd5b63d92e233d60e01b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b6001600160a01b0381165f9081525f51602061316e5f395f51905f52602052604090205460ff16610220576001600160a01b03165f8181525f51602061316e5f395f51905f5260205260408120805460ff191660011790553391905f51602061310e5f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f51602061314e5f395f51905f52602052604090205460ff16610220576001600160a01b03165f8181525f51602061314e5f395f51905f5260205260408120805460ff191660011790553391907f241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b08905f51602061310e5f395f51905f529080a4600190565b156102bf57565b60405162461bcd60e51b815260206004820152601c60248201527f6e616d65737061636520636f6c6c6973696f6e206465746563746564000000006044820152606490fdfe6080806040526004361015610012575f80fd5b5f905f3560e01c90816301ffc9a7146111785750806304cf85de146110c25780630d5869ee146110935780631351f9c3146110795780631bd5002c146110625780631f43fc8b14611003578063248a9ca314610fd9578063297f4c6414610fbc5780632cd799bd14610f705780632f2ff15d14610f3357806336568abe14610ec9578063365f16e314610e905780633b6ab2a914610e6e5780633bfec7a214610e015780633f4ba83a14610d825780634654548814610d585780635467cb4814610d165780635c975abb14610cf457806361a8c8c414610cd75780636558954f14610cba5780636ff6f6c014610c1c5780637232c13314610c795780637667180814610c5c57806378fc30cc14610c1c5780638456cb5914610bc457806384fab62b14610b9f5780638c39aaa414610a325780638de6e28c14610a185780638e9fea39146109d757806391d148541461098157806393c5f0e114610952578063999d71d4146109355780639c984b6d146108f6578063a217fddf146108dc578063a4ccf948146108bf578063b963ebf31461085c578063ba26992a1461080e578063c52c210c14610778578063c80dc41e146106fe578063cc541821146106ad578063d547741f14610669578063d9852abf146103d9578063dcf93475146103aa578063de1f453e14610362578063e3e793701461029a578063ec87621c1461025f5763f96fc51e14610223575f80fd5b3461025c57602060031936011261025c5760043590600a5482101561025c57602061024d8361124f565b90549060031b1c604051908152f35b80fd5b503461025c578060031936011261025c5760206040517f241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b088152f35b503461025c576102a936611216565b818352600f60205273ffffffffffffffffffffffffffffffffffffffff60408420541633036102de576102db916113a4565b80f35b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603960248201527f53796e646963617465466163746f72793a2063616c6c6572206e6f742061757460448201527f686f72697a656420666f72207468697320617070636861696e000000000000006064820152fd5b503461025c578060031936011261025c5761037b6118eb565b6101007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff600454161760045580f35b503461025c57602060031936011261025c5760ff60406020926004358152600984522054166040519015158152f35b50346105a45760806003193601126105a457600435906103f761122c565b916044359273ffffffffffffffffffffffffffffffffffffffff84168094036105a45773ffffffffffffffffffffffffffffffffffffffff90610438611d17565b169283158015610661575b61063957811591821561063357506104596117b2565b915b825f52600d60205260ff60405f20541661060b57825f52600d60205260405f20600160ff198254161790556105f8575b61049482611726565b8051156105d0578051606435916020015ff53d15198115166105995773ffffffffffffffffffffffffffffffffffffffff169081156105a857825f52600f60205260405f20827fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055813b156105a457604051947f485cc95500000000000000000000000000000000000000000000000000000000865260048601528060248601525f8560448183865af194851561059957604095610584575b5081837f49b21f1e4190db8b0a933c951ed013de222c847c15461754682daa2eab1fdbd287519680a482526020820152f35b6105919194505f906112f9565b5f925f610552565b6040513d5f823e3d90fd5b5f80fd5b7fb06ebf3d000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4ca249dc000000000000000000000000000000000000000000000000000000005f5260045ffd5b610603600c54611396565b600c5561048b565b7f24591d89000000000000000000000000000000000000000000000000000000005f5260045ffd5b9161045b565b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b508015610443565b346105a45760406003193601126105a4576106ab60043561068861122c565b906106a66106a1825f525f602052600160405f20015490565b611953565b611a67565b005b346105a4576106bb36611216565b905f52600560205260405f20905f52602052608060405f208054906001810154906003600282015491015491604051938452602084015260408301526060820152f35b346105a45760206003193601126105a457610717611794565b506004355f52600660205261077460405f2060026040519161073883611294565b80548352600181015460208401520154604082015260405191829182919091604080606083019480518452602081015160208501520151910152565b0390f35b346105a45760206003193601126105a45761079161183f565b506002545f52600560205260405f206004355f5260205261077460405f206003604051916107be836112dd565b805483526001810154602084015260028101546040840152015460608201526040519182918291909160608060808301948051845260208101516020850152604081015160408501520151910152565b346105a45760206003193601126105a4576020610854600435805f526007835260405f2054906002545f526005845260405f20905f528352600360405f2001549061131c565b604051908152f35b346105a45760206003193601126105a4576004355f52600e60205260ff60405f2054166040516003821015610892576020918152f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b346105a4575f6003193601126105a4576020600854604051908152f35b346105a4575f6003193601126105a45760206040515f8152f35b346105a45761090436611216565b9061090d61183f565b505f52600560205260405f20905f5260205261077460405f206003604051916107be836112dd565b346105a4575f6003193601126105a4576020600c54604051908152f35b346105a4575f6003193601126105a45760206108546008546002545f5260068352600160405f2001549061131c565b346105a45760406003193601126105a45761099a61122c565b6004355f525f60205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060ff60405f2054166040519015158152f35b346105a45760206003193601126105a4576004355f526006602052606060405f20805490600260018201549101549060405192835260208301526040820152f35b346105a4575f6003193601126105a45760206108546117b2565b346105a45760206003193601126105a457600435610a4e611863565b8015610b1b57805f52600e60205260ff60405f2054166003811015610892576002610a7a911415611b11565b805f52600e60205260405f20600160ff19825416179055600a8104805b610ad4577fd9e1239177bfbd2aebf5d0f20fc075e6df5a502c59d121acd57342c783e31364604083600b549080600b5582519182526020820152a1005b805f52600e60205260ff60405f20541690600382101561089257610afd6001600a931415611b11565b805f52600e60205260405f20600260ff198254161790550480610a97565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f6e616d65737061636520707265666978206f66203020697320666f726269646460448201527f656e0000000000000000000000000000000000000000000000000000000000006064820152fd5b346105a4575f6003193601126105a457602060ff60045460081c166040519015158152f35b346105a4575f6003193601126105a457610bdc6118eb565b610be4611d17565b600160ff19815416176001557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1005b346105a45760206003193601126105a4576004355f52600f602052602073ffffffffffffffffffffffffffffffffffffffff60405f205416604051908152f35b346105a4575f6003193601126105a4576020600254604051908152f35b346105a45760206003193601126105a4576004355f52600d60205260ff60405f2054165f14610cb257602060015b60ff60405191168152f35b60205f610ca7565b346105a4575f6003193601126105a457602060405162278d008152f35b346105a4575f6003193601126105a4576020600354604051908152f35b346105a4575f6003193601126105a457602060ff600154166040519015158152f35b346105a4575f6003193601126105a457610d2e6118eb565b600480547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff169055005b346105a45760206003193601126105a4576004355f526007602052602060405f2054604051908152f35b346105a4575f6003193601126105a457610d9a6118eb565b60015460ff811615610dd95760ff19166001557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6020604051338152a1005b7f8dfc202b000000000000000000000000000000000000000000000000000000005f5260045ffd5b346105a45760206003193601126105a457600435610e1d611863565b805f52600d60205260ff60405f20541661060b57805f52600d60205260405f20600160ff198254161790557f21c25cd0a54e1609e3e68b335693eefb694d5ef17cc32b106f913f8a1f1b80585f80a2005b346105a4575f6003193601126105a457602060ff600454166040519015158152f35b346105a4575f6003193601126105a457610ea8611794565b506002545f52600660205261077460405f2060026040519161073883611294565b346105a45760406003193601126105a457610ee261122c565b3373ffffffffffffffffffffffffffffffffffffffff821603610f0b576106ab90600435611a67565b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b346105a45760406003193601126105a4576106ab600435610f5261122c565b90610f6b6106a1825f525f602052600160405f20015490565b6119b9565b346105a45760206003193601126105a45760206040610f90600435611726565b601f19601f835194859381855280519182918282880152018686015e5f85828601015201168101030190f35b346105a4575f6003193601126105a4576020600b54604051908152f35b346105a45760206003193601126105a45760206108546004355f525f602052600160405f20015490565b346105a45760406003193601126105a457602073ffffffffffffffffffffffffffffffffffffffff6055600b61103a602435611726565b848151910120604051906040820152600435858201523081520160ff81532016604051908152f35b346105a4576106ab61107336611216565b906113a4565b346105a4575f6003193601126105a4576020610854611356565b346105a45760206003193601126105a4576004355f52600d602052602060ff60405f2054166040519015158152f35b346105a4575f6003193601126105a457604051806020600a5492838152018092600a5f527fc65a7bb8d6351c1cf70c95a316cc6a92839c986682d98bc35f958f4883f9d2a8905f5b81811061116257505050816111209103826112f9565b604051918291602083019060208452518091526040830191905f5b818110611149575050500390f35b825184528594506020938401939092019160010161113b565b825484526020909301926001928301920161110a565b346105a45760206003193601126105a457600435907fffffffff0000000000000000000000000000000000000000000000000000000082168092036105a457817f7965db0b00000000000000000000000000000000000000000000000000000000602093149081156111ec575b5015158152f35b7f01ffc9a700000000000000000000000000000000000000000000000000000000915014836111e5565b60031960409101126105a4576004359060243590565b6024359073ffffffffffffffffffffffffffffffffffffffff821682036105a457565b600a5481101561126757600a5f5260205f2001905f90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b6060810190811067ffffffffffffffff8211176112b057604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6080810190811067ffffffffffffffff8211176112b057604052565b90601f601f19910116810190811067ffffffffffffffff8211176112b057604052565b9190820180921161132957565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b60ff60045416156113925760035462278d00810190818111611329578142101561138c5742900362278d00019081116113295790565b50505f90565b5f90565b5f1981146113295760010190565b9060045460ff8160081c16156116fe5782156116d65781156116ae5760ff8116158061165f575b505060035462278d0081018091116113295780421015611538575b50815f52600960205260ff60405f205416156114e5575b3a903a156114dc575b8181028181048303611329577f229e82b33e0aff76844f66f626d42ad2e6239af34eb0f46d1365fb5aef796db0916040916002545f526005602052825f20865f5260205281835f208054156114d2575b6003600282019161146884845461131c565b83550161147684825461131c565b90556002545f526006602052845f209261149183855461131c565b84556114a26001850191825461131c565b905554146114bd575b506002549382519182526020820152a3565b6002016114ca8154611396565b90555f6114ab565b6003548155611456565b60019150611406565b815f52600960205260405f20600160ff19825416179055600a54680100000000000000008110156112b0578060016115209201600a5561124f565b81549060031b905f1985831b921b19161790556113fd565b90916002545f52600660205260405f20925f5b600a548110156115cc578061156160019261124f565b90549060031b1c6002545f52600560205260405f20815f5260205260405f2080541515806115c1575b611597575b50500161154b565b80878560039301550154905f5260076020526115b860405f2091825461131c565b90555f8061158f565b50838101541561158a565b509150916040611627916115e6600182015460085461131c565b6008557fb463d19ecf455be65365092cf8e1db6934a0334cf8cd532ddf9964d01f36b5b26002549282600285945491015482519182526020820152a2611396565b80600255426003557fa5af2fb7730b51b5fdaf8140d2891589f9d0582d84c8d97dd4e18a3acfe4772f6020604051428152a25f6113e6565b61166a575b806113cb565b60ff1960019116176004555f600255426003555f7fa5af2fb7730b51b5fdaf8140d2891589f9d0582d84c8d97dd4e18a3acfe4772f6020604051428152a25f611664565b7f748c183d000000000000000000000000000000000000000000000000000000005f5260045ffd5b7ff6b4131c000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f80653665000000000000000000000000000000000000000000000000000000005f5260045ffd5b611791610f3b9160206040519161173f828601846112f9565b84835281830194611ece86396040518281019182528281526117626040826112f9565b6040519586945180918587015e840190838201905f8252519283915e01015f815203601f1981018352826112f9565b90565b604051906117a182611294565b5f6040838281528260208201520152565b61180e60206118066117c5600b54611b92565b82806117d2600c54611b92565b6040519584879551918291018487015e8401908282015f8152815193849201905e01015f815203601f1981018352826112f9565b805190611d4b565b90156118175790565b7f94e2737e000000000000000000000000000000000000000000000000000000005f5260045ffd5b6040519061184c826112dd565b5f6060838281528260208201528260408201520152565b335f9081527fe84508f2c7fa9c351146748b3025cb78b45df37d868e48c6a75102fecdeee645602052604090205460ff161561189b57565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b0860245260445ffd5b335f9081527fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5602052604090205460ff161561192357565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f2054161561198a5750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f1461138c57805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f20600160ff1982541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f1461138c57805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060ff19815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190565b15611b1857565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601c60248201527f6e616d65737061636520636f6c6c6973696f6e206465746563746564000000006044820152fd5b67ffffffffffffffff81116112b057601f01601f191660200190565b805f917a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000821015611cef575b806d04ee2d6d415b85acef8100000000600a921015611cd4575b662386f26fc10000811015611cc0575b6305f5e100811015611caf575b612710811015611ca0575b6064811015611c92575b1015611c87575b600a5f1960216001850194601f19611c3c611c2688611b76565b97611c34604051998a6112f9565b808952611b76565b013660208801378501015b01917f30313233343536373839616263646566000000000000000000000000000000008282061a8353048015611c82575f19600a9192611c47565b505090565b600190910190611c0c565b606460029104930192611c05565b61271060049104930192611bfb565b6305f5e10060089104930192611bf0565b662386f26fc1000060109104930192611be3565b6d04ee2d6d415b85acef810000000060209104930192611bd3565b50604091507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008104611bb9565b60ff60015416611d2357565b7fd93c0665000000000000000000000000000000000000000000000000000000005f5260045ffd5b90815181118015611d71575b611d69575f611d6592611d78565b9091565b50505f905f90565b505f611d57565b5f9392905b818310611d8d5750505060019190565b90919360ff611dc37fff000000000000000000000000000000000000000000000000000000000000006020888601015116611e01565b169060098211611df557600a810290808204600a149015171561132957600191611dec9161131c565b94019190611d7d565b5050505090505f905f90565b60f81c602f811180611ec3575b15611e3b577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd00160ff1690565b6060811180611eb9575b15611e72577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa90160ff1690565b6040811180611eaf575b15611ea9577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc90160ff1690565b5060ff90565b5060478110611e7c565b5060678110611e45565b50603a8110611e0e56fe60c0346101a257601f610f3b38819003918201601f19168301916001600160401b0383118484101761017b578084926020946040528339810103126101a25751331561018f575f8054336001600160a01b03198216811783556040519290916001600160a01b0316907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a36001805460ff60a01b1916905560ff8181016001600160401b0381118382101761017b578291610e3c833903905ff0801561017057600180546001600160a01b0319166001600160a01b0392909216919091179055801561012b576080523360a052604051610c9590816101a78239608051818181610139015281816102f901526107e9015260a05181818161029e015281816103ee015261078e0152f35b60405162461bcd60e51b815260206004820152601860248201527f41707020636861696e2049442063616e6e6f74206265203000000000000000006044820152606490fd5b6040513d5f823e3d90fd5b634e487b7160e01b5f52604160045260245ffd5b631e4fbdf760e01b5f525f60045260245ffd5b5f80fdfe60806040526004361015610011575f80fd5b5f5f3560e01c8063366cbab71461085657806346e2cc09146106f3578063485cc955146106085780635b3cd6e2146105d4578063715018a6146105565780637a3979dc146104fa578063804e5123146104455780638da5cb5b14610412578063c45a0155146103c1578063cdafb97814610213578063d4f0eb4d1461015c578063d8781342146101215763f2fde38b146100a9575f80fd5b3461011e57602060031936011261011e576100c2610913565b6100ca610be0565b73ffffffffffffffffffffffffffffffffffffffff8116156100f2576100ef90610c2c565b80f35b6024827f1e4fbdf700000000000000000000000000000000000000000000000000000000815280600452fd5b80fd5b503461011e578060031936011261011e5760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b503461011e57602060031936011261011e5773ffffffffffffffffffffffffffffffffffffffff61018b610913565b610193610be0565b1680156101eb57807fffffffffffffffffffffffff000000000000000000000000000000000000000060015416176001557f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b98280a280f35b6004827f5cfe78fe000000000000000000000000000000000000000000000000000000008152fd5b503461011e57602060031936011261011e5760043567ffffffffffffffff81116103bd57366023820112156103bd5780600401359067ffffffffffffffff82116103b9576024810190602436918460051b0101116103b9579082915a91835b81811061034057505050610287905a90610a15565b73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690813b1561033c5782916044839260405194859384927fe3e793700000000000000000000000000000000000000000000000000000000084527f0000000000000000000000000000000000000000000000000000000000000000600485015260248401525af161032b5750f35b8161033591610959565b61011e5780f35b5050fd5b8293945061035d610355828460019596610b53565b903233610a8d565b61036d575b019084939291610272565b61038161037b828587610b53565b906109c7565b7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f604051806103b13394826108cb565b0390a2610362565b8280fd5b5080fd5b503461011e578060031936011261011e57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b503461011e578060031936011261011e5773ffffffffffffffffffffffffffffffffffffffff6020915416604051908152f35b503461011e57602060031936011261011e5760043567ffffffffffffffff81116103bd5761047790369060040161089d565b61048381833233610a8d565b156104d2578291610498610287925a926109c7565b7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f604051806104c83394826108cb565b0390a25a90610a15565b6004837fdc741458000000000000000000000000000000000000000000000000000000008152fd5b503461011e57606060031936011261011e57610514610913565b61051c610936565b916044359067ffffffffffffffff821161011e57602061054c8585610544366004880161089d565b929091610a8d565b6040519015158152f35b503461011e578060031936011261011e5761056f610be0565b8073ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b503461011e578060031936011261011e57602073ffffffffffffffffffffffffffffffffffffffff60015416604051908152f35b503461011e57604060031936011261011e57610622610913565b61062a610936565b610632610be0565b6001549060ff8260a01c166106cb5773ffffffffffffffffffffffffffffffffffffffff169081156106a3577fffffffffffffffffffffff000000000000000000000000000000000000000000161774010000000000000000000000000000000000000000176001556100ca610be0565b6004847f5cfe78fe000000000000000000000000000000000000000000000000000000008152fd5b6004847f0dc149f0000000000000000000000000000000000000000000000000000000008152fd5b503461082a57602060031936011261082a5760043567ffffffffffffffff811161082a5761072590369060040161089d565b9061073282823233610a8d565b1561082e57610777917f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f6104c85a936040519182916020835233956020840191610a4f565b73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690813b1561082a575f916044839260405194859384927fe3e793700000000000000000000000000000000000000000000000000000000084527f0000000000000000000000000000000000000000000000000000000000000000600485015260248401525af161081c575080f35b61082891505f90610959565b005b5f80fd5b7fdc741458000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461082a57602060031936011261082a5760043567ffffffffffffffff811161082a5761088d61037b61089992369060040161089d565b604051918291826108cb565b0390f35b9181601f8401121561082a5782359167ffffffffffffffff831161082a576020838186019501011161082a57565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602060409481855280519182918282880152018686015e5f8582860101520116010190565b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361082a57565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361082a57565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761099a57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6021610a1291836040519485925f60208501528484013781015f8382015203017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282610959565b90565b91908203918211610a2257565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b9273ffffffffffffffffffffffffffffffffffffffff8094610afb602095836001541694604051988997889687967f7a3979dc000000000000000000000000000000000000000000000000000000008852166004870152166024850152606060448501526064840191610a4f565b03915afa908115610b48575f91610b10575090565b90506020813d602011610b40575b81610b2b60209383610959565b8101031261082a5751801515810361082a5790565b3d9150610b1e565b6040513d5f823e3d90fd5b9190811015610bb35760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18136030182121561082a57019081359167ffffffffffffffff831161082a57602001823603811361082a579190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff5f54163303610c0057565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff805f54921691827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3566080806040523460145760e690816100198239f35b5f80fdfe60808060405260043610156011575f80fd5b5f3560e01c637a3979dc146023575f80fd5b34609e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112609e57605660a2565b50605d60c4565b5060443567ffffffffffffffff8111609e5736602382011215609e57806004013567ffffffffffffffff8111609e5736910160240111609e57805f60209252f35b5f80fd5b6004359073ffffffffffffffffffffffffffffffffffffffff82168203609e57565b6024359073ffffffffffffffffffffffffffffffffffffffff82168203609e57562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d52087672763f9a3bcb6419060d6eabd7f0bc8f0e3528e055873483cdc2e860b4e84508f2c7fa9c351146748b3025cb78b45df37d868e48c6a75102fecdeee645ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x804a\x01\x97W`\x1Fa1\x8E8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\x9BW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\x01\x97WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x82\x03a\x01\x97W`\xFF\x19`\x01T\x16`\x01Ua\x01\0a\xFF\0\x19`\x04T\x16\x17`\x04U\x15a\x01\x88W\x80a\0{a\0\x81\x92a\x01\xAFV[Pa\x02%V[Pa\x01\xFE_R`\x0E` R_Q` a1._9_Q\x90_RT`\xFF\x16`\x03\x81\x10\x15a\x01tW`\x02a\0\xB4\x91\x14\x15a\x02\xB8V[a\x01\xFE_R`\x0E` R_Q` a1._9_Q\x90_R\x80T`\xFF\x19\x16`\x01\x17\x90U`3\x80[a\x01-W\x7F\xD9\xE1#\x91w\xBF\xBD*\xEB\xF5\xD0\xF2\x0F\xC0u\xE6\xDFZP,Y\xD1!\xAC\xD5sB\xC7\x83\xE3\x13d`@`\x0BTa\x01\xFE`\x0BU\x81Q\x90\x81Ra\x01\xFE` \x82\x01R\xA1`\x01`\x0CU`@Qa.\t\x90\x81a\x03\x05\x829\xF3[\x80_R`\x0E` R`\xFF`@_ T\x16\x90`\x03\x82\x10\x15a\x01tWa\x01V`\x01`\n\x93\x14\x15a\x02\xB8V[\x80_R`\x0E` R`@_ `\x02`\xFF\x19\x82T\x16\x17\x90U\x04\x80a\0\xDBV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a1n_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x02 W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a1n_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a1\x0E_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a1N_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x02 W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a1N_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F$\x1E\xCF\x16\xD7\x9D\x0F\x8D\xBF\xB9,\xBC\x07\xFE\x17\x84\x04%\x97l\xF0f\x7F\x02/\xE9\x87|\xAA\x83\x1B\x08\x90_Q` a1\x0E_9_Q\x90_R\x90\x80\xA4`\x01\x90V[\x15a\x02\xBFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fnamespace collision detected\0\0\0\0`D\x82\x01R`d\x90\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x01\xFF\xC9\xA7\x14a\x11xWP\x80c\x04\xCF\x85\xDE\x14a\x10\xC2W\x80c\rXi\xEE\x14a\x10\x93W\x80c\x13Q\xF9\xC3\x14a\x10yW\x80c\x1B\xD5\0,\x14a\x10bW\x80c\x1FC\xFC\x8B\x14a\x10\x03W\x80c$\x8A\x9C\xA3\x14a\x0F\xD9W\x80c)\x7FLd\x14a\x0F\xBCW\x80c,\xD7\x99\xBD\x14a\x0FpW\x80c//\xF1]\x14a\x0F3W\x80c6V\x8A\xBE\x14a\x0E\xC9W\x80c6_\x16\xE3\x14a\x0E\x90W\x80c;j\xB2\xA9\x14a\x0EnW\x80c;\xFE\xC7\xA2\x14a\x0E\x01W\x80c?K\xA8:\x14a\r\x82W\x80cFTT\x88\x14a\rXW\x80cTg\xCBH\x14a\r\x16W\x80c\\\x97Z\xBB\x14a\x0C\xF4W\x80ca\xA8\xC8\xC4\x14a\x0C\xD7W\x80ceX\x95O\x14a\x0C\xBAW\x80co\xF6\xF6\xC0\x14a\x0C\x1CW\x80cr2\xC13\x14a\x0CyW\x80cvg\x18\x08\x14a\x0C\\W\x80cx\xFC0\xCC\x14a\x0C\x1CW\x80c\x84V\xCBY\x14a\x0B\xC4W\x80c\x84\xFA\xB6+\x14a\x0B\x9FW\x80c\x8C9\xAA\xA4\x14a\n2W\x80c\x8D\xE6\xE2\x8C\x14a\n\x18W\x80c\x8E\x9F\xEA9\x14a\t\xD7W\x80c\x91\xD1HT\x14a\t\x81W\x80c\x93\xC5\xF0\xE1\x14a\tRW\x80c\x99\x9Dq\xD4\x14a\t5W\x80c\x9C\x98Km\x14a\x08\xF6W\x80c\xA2\x17\xFD\xDF\x14a\x08\xDCW\x80c\xA4\xCC\xF9H\x14a\x08\xBFW\x80c\xB9c\xEB\xF3\x14a\x08\\W\x80c\xBA&\x99*\x14a\x08\x0EW\x80c\xC5,!\x0C\x14a\x07xW\x80c\xC8\r\xC4\x1E\x14a\x06\xFEW\x80c\xCCT\x18!\x14a\x06\xADW\x80c\xD5Gt\x1F\x14a\x06iW\x80c\xD9\x85*\xBF\x14a\x03\xD9W\x80c\xDC\xF94u\x14a\x03\xAAW\x80c\xDE\x1FE>\x14a\x03bW\x80c\xE3\xE7\x93p\x14a\x02\x9AW\x80c\xEC\x87b\x1C\x14a\x02_Wc\xF9o\xC5\x1E\x14a\x02#W_\x80\xFD[4a\x02\\W` `\x03\x196\x01\x12a\x02\\W`\x045\x90`\nT\x82\x10\x15a\x02\\W` a\x02M\x83a\x12OV[\x90T\x90`\x03\x1B\x1C`@Q\x90\x81R\xF3[\x80\xFD[P4a\x02\\W\x80`\x03\x196\x01\x12a\x02\\W` `@Q\x7F$\x1E\xCF\x16\xD7\x9D\x0F\x8D\xBF\xB9,\xBC\x07\xFE\x17\x84\x04%\x97l\xF0f\x7F\x02/\xE9\x87|\xAA\x83\x1B\x08\x81R\xF3[P4a\x02\\Wa\x02\xA96a\x12\x16V[\x81\x83R`\x0F` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x84 T\x163\x03a\x02\xDEWa\x02\xDB\x91a\x13\xA4V[\x80\xF3[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FSyndicateFactory: caller not aut`D\x82\x01R\x7Fhorized for this appchain\0\0\0\0\0\0\0`d\x82\x01R\xFD[P4a\x02\\W\x80`\x03\x196\x01\x12a\x02\\Wa\x03{a\x18\xEBV[a\x01\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF`\x04T\x16\x17`\x04U\x80\xF3[P4a\x02\\W` `\x03\x196\x01\x12a\x02\\W`\xFF`@` \x92`\x045\x81R`\t\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x05\xA4W`\x80`\x03\x196\x01\x12a\x05\xA4W`\x045\x90a\x03\xF7a\x12,V[\x91`D5\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x80\x94\x03a\x05\xA4Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90a\x048a\x1D\x17V[\x16\x92\x83\x15\x80\x15a\x06aW[a\x069W\x81\x15\x91\x82\x15a\x063WPa\x04Ya\x17\xB2V[\x91[\x82_R`\r` R`\xFF`@_ T\x16a\x06\x0BW\x82_R`\r` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90Ua\x05\xF8W[a\x04\x94\x82a\x17&V[\x80Q\x15a\x05\xD0W\x80Q`d5\x91` \x01_\xF5=\x15\x19\x81\x15\x16a\x05\x99Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x05\xA8W\x82_R`\x0F` R`@_ \x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90U\x81;\x15a\x05\xA4W`@Q\x94\x7FH\\\xC9U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\x04\x86\x01R\x80`$\x86\x01R_\x85`D\x81\x83\x86Z\xF1\x94\x85\x15a\x05\x99W`@\x95a\x05\x84W[P\x81\x83\x7FI\xB2\x1F\x1EA\x90\xDB\x8B\n\x93<\x95\x1E\xD0\x13\xDE\",\x84|\x15F\x17Th-\xAA.\xAB\x1F\xDB\xD2\x87Q\x96\x80\xA4\x82R` \x82\x01R\xF3[a\x05\x91\x91\x94P_\x90a\x12\xF9V[_\x92_a\x05RV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x7F\xB0n\xBF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FL\xA2I\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[a\x06\x03`\x0CTa\x13\x96V[`\x0CUa\x04\x8BV[\x7F$Y\x1D\x89\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x91a\x04[V[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P\x80\x15a\x04CV[4a\x05\xA4W`@`\x03\x196\x01\x12a\x05\xA4Wa\x06\xAB`\x045a\x06\x88a\x12,V[\x90a\x06\xA6a\x06\xA1\x82_R_` R`\x01`@_ \x01T\x90V[a\x19SV[a\x1AgV[\0[4a\x05\xA4Wa\x06\xBB6a\x12\x16V[\x90_R`\x05` R`@_ \x90_R` R`\x80`@_ \x80T\x90`\x01\x81\x01T\x90`\x03`\x02\x82\x01T\x91\x01T\x91`@Q\x93\x84R` \x84\x01R`@\x83\x01R``\x82\x01R\xF3[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4Wa\x07\x17a\x17\x94V[P`\x045_R`\x06` Ra\x07t`@_ `\x02`@Q\x91a\x078\x83a\x12\x94V[\x80T\x83R`\x01\x81\x01T` \x84\x01R\x01T`@\x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91`@\x80``\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R\x01Q\x91\x01RV[\x03\x90\xF3[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4Wa\x07\x91a\x18?V[P`\x02T_R`\x05` R`@_ `\x045_R` Ra\x07t`@_ `\x03`@Q\x91a\x07\xBE\x83a\x12\xDDV[\x80T\x83R`\x01\x81\x01T` \x84\x01R`\x02\x81\x01T`@\x84\x01R\x01T``\x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91``\x80`\x80\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91\x01RV[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W` a\x08T`\x045\x80_R`\x07\x83R`@_ T\x90`\x02T_R`\x05\x84R`@_ \x90_R\x83R`\x03`@_ \x01T\x90a\x13\x1CV[`@Q\x90\x81R\xF3[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045_R`\x0E` R`\xFF`@_ T\x16`@Q`\x03\x82\x10\x15a\x08\x92W` \x91\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\x08T`@Q\x90\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `@Q_\x81R\xF3[4a\x05\xA4Wa\t\x046a\x12\x16V[\x90a\t\ra\x18?V[P_R`\x05` R`@_ \x90_R` Ra\x07t`@_ `\x03`@Q\x91a\x07\xBE\x83a\x12\xDDV[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\x0CT`@Q\x90\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` a\x08T`\x08T`\x02T_R`\x06\x83R`\x01`@_ \x01T\x90a\x13\x1CV[4a\x05\xA4W`@`\x03\x196\x01\x12a\x05\xA4Wa\t\x9Aa\x12,V[`\x045_R_` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045_R`\x06` R```@_ \x80T\x90`\x02`\x01\x82\x01T\x91\x01T\x90`@Q\x92\x83R` \x83\x01R`@\x82\x01R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` a\x08Ta\x17\xB2V[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045a\nNa\x18cV[\x80\x15a\x0B\x1BW\x80_R`\x0E` R`\xFF`@_ T\x16`\x03\x81\x10\x15a\x08\x92W`\x02a\nz\x91\x14\x15a\x1B\x11V[\x80_R`\x0E` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`\n\x81\x04\x80[a\n\xD4W\x7F\xD9\xE1#\x91w\xBF\xBD*\xEB\xF5\xD0\xF2\x0F\xC0u\xE6\xDFZP,Y\xD1!\xAC\xD5sB\xC7\x83\xE3\x13d`@\x83`\x0BT\x90\x80`\x0BU\x82Q\x91\x82R` \x82\x01R\xA1\0[\x80_R`\x0E` R`\xFF`@_ T\x16\x90`\x03\x82\x10\x15a\x08\x92Wa\n\xFD`\x01`\n\x93\x14\x15a\x1B\x11V[\x80_R`\x0E` R`@_ `\x02`\xFF\x19\x82T\x16\x17\x90U\x04\x80a\n\x97V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Fnamespace prefix of 0 is forbidd`D\x82\x01R\x7Fen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\xFF`\x04T`\x08\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4Wa\x0B\xDCa\x18\xEBV[a\x0B\xE4a\x1D\x17V[`\x01`\xFF\x19\x81T\x16\x17`\x01U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045_R`\x0F` R` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16`@Q\x90\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\x02T`@Q\x90\x81R\xF3[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045_R`\r` R`\xFF`@_ T\x16_\x14a\x0C\xB2W` `\x01[`\xFF`@Q\x91\x16\x81R\xF3[` _a\x0C\xA7V[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `@Qb'\x8D\0\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\x03T`@Q\x90\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\xFF`\x01T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4Wa\r.a\x18\xEBV[`\x04\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U\0[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045_R`\x07` R` `@_ T`@Q\x90\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4Wa\r\x9Aa\x18\xEBV[`\x01T`\xFF\x81\x16\x15a\r\xD9W`\xFF\x19\x16`\x01U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\0[\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045a\x0E\x1Da\x18cV[\x80_R`\r` R`\xFF`@_ T\x16a\x06\x0BW\x80_R`\r` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F!\xC2\\\xD0\xA5N\x16\t\xE3\xE6\x8B3V\x93\xEE\xFBiM^\xF1|\xC3+\x10o\x91?\x8A\x1F\x1B\x80X_\x80\xA2\0[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\xFF`\x04T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4Wa\x0E\xA8a\x17\x94V[P`\x02T_R`\x06` Ra\x07t`@_ `\x02`@Q\x91a\x078\x83a\x12\x94V[4a\x05\xA4W`@`\x03\x196\x01\x12a\x05\xA4Wa\x0E\xE2a\x12,V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\x0F\x0BWa\x06\xAB\x90`\x045a\x1AgV[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x05\xA4W`@`\x03\x196\x01\x12a\x05\xA4Wa\x06\xAB`\x045a\x0FRa\x12,V[\x90a\x0Fka\x06\xA1\x82_R_` R`\x01`@_ \x01T\x90V[a\x19\xB9V[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W` `@a\x0F\x90`\x045a\x17&V[`\x1F\x19`\x1F\x83Q\x94\x85\x93\x81\x85R\x80Q\x91\x82\x91\x82\x82\x88\x01R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x81\x01\x03\x01\x90\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\x0BT`@Q\x90\x81R\xF3[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W` a\x08T`\x045_R_` R`\x01`@_ \x01T\x90V[4a\x05\xA4W`@`\x03\x196\x01\x12a\x05\xA4W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`U`\x0Ba\x10:`$5a\x17&V[\x84\x81Q\x91\x01 `@Q\x90`@\x82\x01R`\x045\x85\x82\x01R0\x81R\x01`\xFF\x81S \x16`@Q\x90\x81R\xF3[4a\x05\xA4Wa\x06\xABa\x10s6a\x12\x16V[\x90a\x13\xA4V[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` a\x08Ta\x13VV[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045_R`\r` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W`@Q\x80` `\nT\x92\x83\x81R\x01\x80\x92`\n_R\x7F\xC6Z{\xB8\xD65\x1C\x1C\xF7\x0C\x95\xA3\x16\xCCj\x92\x83\x9C\x98f\x82\xD9\x8B\xC3_\x95\x8FH\x83\xF9\xD2\xA8\x90_[\x81\x81\x10a\x11bWPPP\x81a\x11 \x91\x03\x82a\x12\xF9V[`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x11IWPPP\x03\x90\xF3[\x82Q\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x11;V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x11\nV[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x80\x92\x03a\x05\xA4W\x81\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x93\x14\x90\x81\x15a\x11\xECW[P\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x83a\x11\xE5V[`\x03\x19`@\x91\x01\x12a\x05\xA4W`\x045\x90`$5\x90V[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x05\xA4WV[`\nT\x81\x10\x15a\x12gW`\n_R` _ \x01\x90_\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x12\xB0W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x12\xB0W`@RV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x12\xB0W`@RV[\x91\x90\x82\x01\x80\x92\x11a\x13)WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[`\xFF`\x04T\x16\x15a\x13\x92W`\x03Tb'\x8D\0\x81\x01\x90\x81\x81\x11a\x13)W\x81B\x10\x15a\x13\x8CWB\x90\x03b'\x8D\0\x01\x90\x81\x11a\x13)W\x90V[PP_\x90V[_\x90V[_\x19\x81\x14a\x13)W`\x01\x01\x90V[\x90`\x04T`\xFF\x81`\x08\x1C\x16\x15a\x16\xFEW\x82\x15a\x16\xD6W\x81\x15a\x16\xAEW`\xFF\x81\x16\x15\x80a\x16_W[PP`\x03Tb'\x8D\0\x81\x01\x80\x91\x11a\x13)W\x80B\x10\x15a\x158W[P\x81_R`\t` R`\xFF`@_ T\x16\x15a\x14\xE5W[:\x90:\x15a\x14\xDCW[\x81\x81\x02\x81\x81\x04\x83\x03a\x13)W\x7F\"\x9E\x82\xB3>\n\xFFv\x84Of\xF6&\xD4*\xD2\xE6#\x9A\xF3N\xB0\xF4m\x13e\xFBZ\xEFym\xB0\x91`@\x91`\x02T_R`\x05` R\x82_ \x86_R` R\x81\x83_ \x80T\x15a\x14\xD2W[`\x03`\x02\x82\x01\x91a\x14h\x84\x84Ta\x13\x1CV[\x83U\x01a\x14v\x84\x82Ta\x13\x1CV[\x90U`\x02T_R`\x06` R\x84_ \x92a\x14\x91\x83\x85Ta\x13\x1CV[\x84Ua\x14\xA2`\x01\x85\x01\x91\x82Ta\x13\x1CV[\x90UT\x14a\x14\xBDW[P`\x02T\x93\x82Q\x91\x82R` \x82\x01R\xA3V[`\x02\x01a\x14\xCA\x81Ta\x13\x96V[\x90U_a\x14\xABV[`\x03T\x81Ua\x14VV[`\x01\x91Pa\x14\x06V[\x81_R`\t` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`\nTh\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x12\xB0W\x80`\x01a\x15 \x92\x01`\nUa\x12OV[\x81T\x90`\x03\x1B\x90_\x19\x85\x83\x1B\x92\x1B\x19\x16\x17\x90Ua\x13\xFDV[\x90\x91`\x02T_R`\x06` R`@_ \x92_[`\nT\x81\x10\x15a\x15\xCCW\x80a\x15a`\x01\x92a\x12OV[\x90T\x90`\x03\x1B\x1C`\x02T_R`\x05` R`@_ \x81_R` R`@_ \x80T\x15\x15\x80a\x15\xC1W[a\x15\x97W[PP\x01a\x15KV[\x80\x87\x85`\x03\x93\x01U\x01T\x90_R`\x07` Ra\x15\xB8`@_ \x91\x82Ta\x13\x1CV[\x90U_\x80a\x15\x8FV[P\x83\x81\x01T\x15a\x15\x8AV[P\x91P\x91`@a\x16'\x91a\x15\xE6`\x01\x82\x01T`\x08Ta\x13\x1CV[`\x08U\x7F\xB4c\xD1\x9E\xCFE[\xE6Se\t,\xF8\xE1\xDBi4\xA03L\xF8\xCDS-\xDF\x99d\xD0\x1F6\xB5\xB2`\x02T\x92\x82`\x02\x85\x94T\x91\x01T\x82Q\x91\x82R` \x82\x01R\xA2a\x13\x96V[\x80`\x02UB`\x03U\x7F\xA5\xAF/\xB7s\x0BQ\xB5\xFD\xAF\x81@\xD2\x89\x15\x89\xF9\xD0X-\x84\xC8\xD9}\xD4\xE1\x8A:\xCF\xE4w/` `@QB\x81R\xA2_a\x13\xE6V[a\x16jW[\x80a\x13\xCBV[`\xFF\x19`\x01\x91\x16\x17`\x04U_`\x02UB`\x03U_\x7F\xA5\xAF/\xB7s\x0BQ\xB5\xFD\xAF\x81@\xD2\x89\x15\x89\xF9\xD0X-\x84\xC8\xD9}\xD4\xE1\x8A:\xCF\xE4w/` `@QB\x81R\xA2_a\x16dV[\x7Ft\x8C\x18=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xF6\xB4\x13\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x80e6e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[a\x17\x91a\x0F;\x91` `@Q\x91a\x17?\x82\x86\x01\x84a\x12\xF9V[\x84\x83R\x81\x83\x01\x94a\x1E\xCE\x869`@Q\x82\x81\x01\x91\x82R\x82\x81Ra\x17b`@\x82a\x12\xF9V[`@Q\x95\x86\x94Q\x80\x91\x85\x87\x01^\x84\x01\x90\x83\x82\x01\x90_\x82RQ\x92\x83\x91^\x01\x01_\x81R\x03`\x1F\x19\x81\x01\x83R\x82a\x12\xF9V[\x90V[`@Q\x90a\x17\xA1\x82a\x12\x94V[_`@\x83\x82\x81R\x82` \x82\x01R\x01RV[a\x18\x0E` a\x18\x06a\x17\xC5`\x0BTa\x1B\x92V[\x82\x80a\x17\xD2`\x0CTa\x1B\x92V[`@Q\x95\x84\x87\x95Q\x91\x82\x91\x01\x84\x87\x01^\x84\x01\x90\x82\x82\x01_\x81R\x81Q\x93\x84\x92\x01\x90^\x01\x01_\x81R\x03`\x1F\x19\x81\x01\x83R\x82a\x12\xF9V[\x80Q\x90a\x1DKV[\x90\x15a\x18\x17W\x90V[\x7F\x94\xE2s~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`@Q\x90a\x18L\x82a\x12\xDDV[_``\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[3_\x90\x81R\x7F\xE8E\x08\xF2\xC7\xFA\x9C5\x11Ft\x8B0%\xCBx\xB4]\xF3}\x86\x8EH\xC6\xA7Q\x02\xFE\xCD\xEE\xE6E` R`@\x90 T`\xFF\x16\x15a\x18\x9BWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F$\x1E\xCF\x16\xD7\x9D\x0F\x8D\xBF\xB9,\xBC\x07\xFE\x17\x84\x04%\x97l\xF0f\x7F\x02/\xE9\x87|\xAA\x83\x1B\x08`$R`D_\xFD[3_\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16\x15a\x19#WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x19\x8AWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x13\x8CW\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x13\x8CW\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\xFF\x19\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V[\x15a\x1B\x18WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fnamespace collision detected\0\0\0\0`D\x82\x01R\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x12\xB0W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x80_\x91z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x82\x10\x15a\x1C\xEFW[\x80m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0`\n\x92\x10\x15a\x1C\xD4W[f#\x86\xF2o\xC1\0\0\x81\x10\x15a\x1C\xC0W[c\x05\xF5\xE1\0\x81\x10\x15a\x1C\xAFW[a'\x10\x81\x10\x15a\x1C\xA0W[`d\x81\x10\x15a\x1C\x92W[\x10\x15a\x1C\x87W[`\n_\x19`!`\x01\x85\x01\x94`\x1F\x19a\x1C<a\x1C&\x88a\x1BvV[\x97a\x1C4`@Q\x99\x8Aa\x12\xF9V[\x80\x89Ra\x1BvV[\x016` \x88\x017\x85\x01\x01[\x01\x91\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x82\x06\x1A\x83S\x04\x80\x15a\x1C\x82W_\x19`\n\x91\x92a\x1CGV[PP\x90V[`\x01\x90\x91\x01\x90a\x1C\x0CV[`d`\x02\x91\x04\x93\x01\x92a\x1C\x05V[a'\x10`\x04\x91\x04\x93\x01\x92a\x1B\xFBV[c\x05\xF5\xE1\0`\x08\x91\x04\x93\x01\x92a\x1B\xF0V[f#\x86\xF2o\xC1\0\0`\x10\x91\x04\x93\x01\x92a\x1B\xE3V[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0` \x91\x04\x93\x01\x92a\x1B\xD3V[P`@\x91Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x81\x04a\x1B\xB9V[`\xFF`\x01T\x16a\x1D#WV[\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90\x81Q\x81\x11\x80\x15a\x1DqW[a\x1DiW_a\x1De\x92a\x1DxV[\x90\x91V[PP_\x90_\x90V[P_a\x1DWV[_\x93\x92\x90[\x81\x83\x10a\x1D\x8DWPPP`\x01\x91\x90V[\x90\x91\x93`\xFFa\x1D\xC3\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x88\x86\x01\x01Q\x16a\x1E\x01V[\x16\x90`\t\x82\x11a\x1D\xF5W`\n\x81\x02\x90\x80\x82\x04`\n\x14\x90\x15\x17\x15a\x13)W`\x01\x91a\x1D\xEC\x91a\x13\x1CV[\x94\x01\x91\x90a\x1D}V[PPPP\x90P_\x90_\x90V[`\xF8\x1C`/\x81\x11\x80a\x1E\xC3W[\x15a\x1E;W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xD0\x01`\xFF\x16\x90V[``\x81\x11\x80a\x1E\xB9W[\x15a\x1ErW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA9\x01`\xFF\x16\x90V[`@\x81\x11\x80a\x1E\xAFW[\x15a\x1E\xA9W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC9\x01`\xFF\x16\x90V[P`\xFF\x90V[P`G\x81\x10a\x1E|V[P`g\x81\x10a\x1EEV[P`:\x81\x10a\x1E\x0EV\xFE`\xC04a\x01\xA2W`\x1Fa\x0F;8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01{W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\x01\xA2WQ3\x15a\x01\x8FW_\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x83U`@Q\x92\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3`\x01\x80T`\xFF`\xA0\x1B\x19\x16\x90U`\xFF\x81\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x83\x82\x10\x17a\x01{W\x82\x91a\x0E<\x839\x03\x90_\xF0\x80\x15a\x01pW`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80\x15a\x01+W`\x80R3`\xA0R`@Qa\x0C\x95\x90\x81a\x01\xA7\x829`\x80Q\x81\x81\x81a\x019\x01R\x81\x81a\x02\xF9\x01Ra\x07\xE9\x01R`\xA0Q\x81\x81\x81a\x02\x9E\x01R\x81\x81a\x03\xEE\x01Ra\x07\x8E\x01R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`@Q=_\x82>=\x90\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c6l\xBA\xB7\x14a\x08VW\x80cF\xE2\xCC\t\x14a\x06\xF3W\x80cH\\\xC9U\x14a\x06\x08W\x80c[<\xD6\xE2\x14a\x05\xD4W\x80cqP\x18\xA6\x14a\x05VW\x80cz9y\xDC\x14a\x04\xFAW\x80c\x80NQ#\x14a\x04EW\x80c\x8D\xA5\xCB[\x14a\x04\x12W\x80c\xC4Z\x01U\x14a\x03\xC1W\x80c\xCD\xAF\xB9x\x14a\x02\x13W\x80c\xD4\xF0\xEBM\x14a\x01\\W\x80c\xD8x\x13B\x14a\x01!Wc\xF2\xFD\xE3\x8B\x14a\0\xA9W_\x80\xFD[4a\x01\x1EW` `\x03\x196\x01\x12a\x01\x1EWa\0\xC2a\t\x13V[a\0\xCAa\x0B\xE0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a\0\xF2Wa\0\xEF\x90a\x0C,V[\x80\xF3[`$\x82\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x80`\x04R\xFD[\x80\xFD[P4a\x01\x1EW\x80`\x03\x196\x01\x12a\x01\x1EW` `@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xF3[P4a\x01\x1EW` `\x03\x196\x01\x12a\x01\x1EWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\x8Ba\t\x13V[a\x01\x93a\x0B\xE0V[\x16\x80\x15a\x01\xEBW\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01T\x16\x17`\x01U\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x82\x80\xA2\x80\xF3[`\x04\x82\x7F\\\xFEx\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x01\x1EW` `\x03\x196\x01\x12a\x01\x1EW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\xBDW6`#\x82\x01\x12\x15a\x03\xBDW\x80`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03\xB9W`$\x81\x01\x90`$6\x91\x84`\x05\x1B\x01\x01\x11a\x03\xB9W\x90\x82\x91Z\x91\x83[\x81\x81\x10a\x03@WPPPa\x02\x87\x90Z\x90a\n\x15V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x81;\x15a\x03<W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\xE3\xE7\x93p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x85\x01R`$\x84\x01RZ\xF1a\x03+WP\xF3[\x81a\x035\x91a\tYV[a\x01\x1EW\x80\xF3[PP\xFD[\x82\x93\x94Pa\x03]a\x03U\x82\x84`\x01\x95\x96a\x0BSV[\x9023a\n\x8DV[a\x03mW[\x01\x90\x84\x93\x92\x91a\x02rV[a\x03\x81a\x03{\x82\x85\x87a\x0BSV[\x90a\t\xC7V[\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F`@Q\x80a\x03\xB13\x94\x82a\x08\xCBV[\x03\x90\xA2a\x03bV[\x82\x80\xFD[P\x80\xFD[P4a\x01\x1EW\x80`\x03\x196\x01\x12a\x01\x1EW` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x01\x1EW\x80`\x03\x196\x01\x12a\x01\x1EWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x91T\x16`@Q\x90\x81R\xF3[P4a\x01\x1EW` `\x03\x196\x01\x12a\x01\x1EW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\xBDWa\x04w\x906\x90`\x04\x01a\x08\x9DV[a\x04\x83\x81\x8323a\n\x8DV[\x15a\x04\xD2W\x82\x91a\x04\x98a\x02\x87\x92Z\x92a\t\xC7V[\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F`@Q\x80a\x04\xC83\x94\x82a\x08\xCBV[\x03\x90\xA2Z\x90a\n\x15V[`\x04\x83\x7F\xDCt\x14X\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x01\x1EW```\x03\x196\x01\x12a\x01\x1EWa\x05\x14a\t\x13V[a\x05\x1Ca\t6V[\x91`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\x1EW` a\x05L\x85\x85a\x05D6`\x04\x88\x01a\x08\x9DV[\x92\x90\x91a\n\x8DV[`@Q\x90\x15\x15\x81R\xF3[P4a\x01\x1EW\x80`\x03\x196\x01\x12a\x01\x1EWa\x05oa\x0B\xE0V[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P4a\x01\x1EW\x80`\x03\x196\x01\x12a\x01\x1EW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01T\x16`@Q\x90\x81R\xF3[P4a\x01\x1EW`@`\x03\x196\x01\x12a\x01\x1EWa\x06\"a\t\x13V[a\x06*a\t6V[a\x062a\x0B\xE0V[`\x01T\x90`\xFF\x82`\xA0\x1C\x16a\x06\xCBWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x06\xA3W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17`\x01Ua\0\xCAa\x0B\xE0V[`\x04\x84\x7F\\\xFEx\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`\x04\x84\x7F\r\xC1I\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x08*W` `\x03\x196\x01\x12a\x08*W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08*Wa\x07%\x906\x90`\x04\x01a\x08\x9DV[\x90a\x072\x82\x8223a\n\x8DV[\x15a\x08.Wa\x07w\x91\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7Fa\x04\xC8Z\x93`@Q\x91\x82\x91` \x83R3\x95` \x84\x01\x91a\nOV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x81;\x15a\x08*W_\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\xE3\xE7\x93p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x85\x01R`$\x84\x01RZ\xF1a\x08\x1CWP\x80\xF3[a\x08(\x91P_\x90a\tYV[\0[_\x80\xFD[\x7F\xDCt\x14X\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x08*W` `\x03\x196\x01\x12a\x08*W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08*Wa\x08\x8Da\x03{a\x08\x99\x926\x90`\x04\x01a\x08\x9DV[`@Q\x91\x82\x91\x82a\x08\xCBV[\x03\x90\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x08*W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x08*W` \x83\x81\x86\x01\x95\x01\x01\x11a\x08*WV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` `@\x94\x81\x85R\x80Q\x91\x82\x91\x82\x82\x88\x01R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x08*WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x08*WV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\x9AW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`!a\n\x12\x91\x83`@Q\x94\x85\x92_` \x85\x01R\x84\x84\x017\x81\x01_\x83\x82\x01R\x03\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a\tYV[\x90V[\x91\x90\x82\x03\x91\x82\x11a\n\"WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x94a\n\xFB` \x95\x83`\x01T\x16\x94`@Q\x98\x89\x97\x88\x96\x87\x96\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88R\x16`\x04\x87\x01R\x16`$\x85\x01R```D\x85\x01R`d\x84\x01\x91a\nOV[\x03\x91Z\xFA\x90\x81\x15a\x0BHW_\x91a\x0B\x10WP\x90V[\x90P` \x81=` \x11a\x0B@W[\x81a\x0B+` \x93\x83a\tYV[\x81\x01\x03\x12a\x08*WQ\x80\x15\x15\x81\x03a\x08*W\x90V[=\x91Pa\x0B\x1EV[`@Q=_\x82>=\x90\xFD[\x91\x90\x81\x10\x15a\x0B\xB3W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x08*W\x01\x90\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x08*W` \x01\x826\x03\x81\x13a\x08*W\x91\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\x0C\0WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80_T\x92\x16\x91\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V`\x80\x80`@R4`\x14W`\xE6\x90\x81a\0\x19\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15`\x11W_\x80\xFD[_5`\xE0\x1Ccz9y\xDC\x14`#W_\x80\xFD[4`\x9EW``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12`\x9EW`V`\xA2V[P`]`\xC4V[P`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x9EW6`#\x82\x01\x12\x15`\x9EW\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x9EW6\x91\x01`$\x01\x11`\x9EW\x80_` \x92R\xF3[_\x80\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03`\x9EWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03`\x9EWV/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\rR\x08vrv?\x9A;\xCBd\x19\x06\rn\xAB\xD7\xF0\xBC\x8F\x0E5(\xE0U\x874\x83\xCD\xC2\xE8`\xB4\xE8E\x08\xF2\xC7\xFA\x9C5\x11Ft\x8B0%\xCBx\xB4]\xF3}\x86\x8EH\xC6\xA7Q\x02\xFE\xCD\xEE\xE6E\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f905f3560e01c90816301ffc9a7146111785750806304cf85de146110c25780630d5869ee146110935780631351f9c3146110795780631bd5002c146110625780631f43fc8b14611003578063248a9ca314610fd9578063297f4c6414610fbc5780632cd799bd14610f705780632f2ff15d14610f3357806336568abe14610ec9578063365f16e314610e905780633b6ab2a914610e6e5780633bfec7a214610e015780633f4ba83a14610d825780634654548814610d585780635467cb4814610d165780635c975abb14610cf457806361a8c8c414610cd75780636558954f14610cba5780636ff6f6c014610c1c5780637232c13314610c795780637667180814610c5c57806378fc30cc14610c1c5780638456cb5914610bc457806384fab62b14610b9f5780638c39aaa414610a325780638de6e28c14610a185780638e9fea39146109d757806391d148541461098157806393c5f0e114610952578063999d71d4146109355780639c984b6d146108f6578063a217fddf146108dc578063a4ccf948146108bf578063b963ebf31461085c578063ba26992a1461080e578063c52c210c14610778578063c80dc41e146106fe578063cc541821146106ad578063d547741f14610669578063d9852abf146103d9578063dcf93475146103aa578063de1f453e14610362578063e3e793701461029a578063ec87621c1461025f5763f96fc51e14610223575f80fd5b3461025c57602060031936011261025c5760043590600a5482101561025c57602061024d8361124f565b90549060031b1c604051908152f35b80fd5b503461025c578060031936011261025c5760206040517f241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b088152f35b503461025c576102a936611216565b818352600f60205273ffffffffffffffffffffffffffffffffffffffff60408420541633036102de576102db916113a4565b80f35b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603960248201527f53796e646963617465466163746f72793a2063616c6c6572206e6f742061757460448201527f686f72697a656420666f72207468697320617070636861696e000000000000006064820152fd5b503461025c578060031936011261025c5761037b6118eb565b6101007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff600454161760045580f35b503461025c57602060031936011261025c5760ff60406020926004358152600984522054166040519015158152f35b50346105a45760806003193601126105a457600435906103f761122c565b916044359273ffffffffffffffffffffffffffffffffffffffff84168094036105a45773ffffffffffffffffffffffffffffffffffffffff90610438611d17565b169283158015610661575b61063957811591821561063357506104596117b2565b915b825f52600d60205260ff60405f20541661060b57825f52600d60205260405f20600160ff198254161790556105f8575b61049482611726565b8051156105d0578051606435916020015ff53d15198115166105995773ffffffffffffffffffffffffffffffffffffffff169081156105a857825f52600f60205260405f20827fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055813b156105a457604051947f485cc95500000000000000000000000000000000000000000000000000000000865260048601528060248601525f8560448183865af194851561059957604095610584575b5081837f49b21f1e4190db8b0a933c951ed013de222c847c15461754682daa2eab1fdbd287519680a482526020820152f35b6105919194505f906112f9565b5f925f610552565b6040513d5f823e3d90fd5b5f80fd5b7fb06ebf3d000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4ca249dc000000000000000000000000000000000000000000000000000000005f5260045ffd5b610603600c54611396565b600c5561048b565b7f24591d89000000000000000000000000000000000000000000000000000000005f5260045ffd5b9161045b565b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b508015610443565b346105a45760406003193601126105a4576106ab60043561068861122c565b906106a66106a1825f525f602052600160405f20015490565b611953565b611a67565b005b346105a4576106bb36611216565b905f52600560205260405f20905f52602052608060405f208054906001810154906003600282015491015491604051938452602084015260408301526060820152f35b346105a45760206003193601126105a457610717611794565b506004355f52600660205261077460405f2060026040519161073883611294565b80548352600181015460208401520154604082015260405191829182919091604080606083019480518452602081015160208501520151910152565b0390f35b346105a45760206003193601126105a45761079161183f565b506002545f52600560205260405f206004355f5260205261077460405f206003604051916107be836112dd565b805483526001810154602084015260028101546040840152015460608201526040519182918291909160608060808301948051845260208101516020850152604081015160408501520151910152565b346105a45760206003193601126105a4576020610854600435805f526007835260405f2054906002545f526005845260405f20905f528352600360405f2001549061131c565b604051908152f35b346105a45760206003193601126105a4576004355f52600e60205260ff60405f2054166040516003821015610892576020918152f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b346105a4575f6003193601126105a4576020600854604051908152f35b346105a4575f6003193601126105a45760206040515f8152f35b346105a45761090436611216565b9061090d61183f565b505f52600560205260405f20905f5260205261077460405f206003604051916107be836112dd565b346105a4575f6003193601126105a4576020600c54604051908152f35b346105a4575f6003193601126105a45760206108546008546002545f5260068352600160405f2001549061131c565b346105a45760406003193601126105a45761099a61122c565b6004355f525f60205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060ff60405f2054166040519015158152f35b346105a45760206003193601126105a4576004355f526006602052606060405f20805490600260018201549101549060405192835260208301526040820152f35b346105a4575f6003193601126105a45760206108546117b2565b346105a45760206003193601126105a457600435610a4e611863565b8015610b1b57805f52600e60205260ff60405f2054166003811015610892576002610a7a911415611b11565b805f52600e60205260405f20600160ff19825416179055600a8104805b610ad4577fd9e1239177bfbd2aebf5d0f20fc075e6df5a502c59d121acd57342c783e31364604083600b549080600b5582519182526020820152a1005b805f52600e60205260ff60405f20541690600382101561089257610afd6001600a931415611b11565b805f52600e60205260405f20600260ff198254161790550480610a97565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f6e616d65737061636520707265666978206f66203020697320666f726269646460448201527f656e0000000000000000000000000000000000000000000000000000000000006064820152fd5b346105a4575f6003193601126105a457602060ff60045460081c166040519015158152f35b346105a4575f6003193601126105a457610bdc6118eb565b610be4611d17565b600160ff19815416176001557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1005b346105a45760206003193601126105a4576004355f52600f602052602073ffffffffffffffffffffffffffffffffffffffff60405f205416604051908152f35b346105a4575f6003193601126105a4576020600254604051908152f35b346105a45760206003193601126105a4576004355f52600d60205260ff60405f2054165f14610cb257602060015b60ff60405191168152f35b60205f610ca7565b346105a4575f6003193601126105a457602060405162278d008152f35b346105a4575f6003193601126105a4576020600354604051908152f35b346105a4575f6003193601126105a457602060ff600154166040519015158152f35b346105a4575f6003193601126105a457610d2e6118eb565b600480547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff169055005b346105a45760206003193601126105a4576004355f526007602052602060405f2054604051908152f35b346105a4575f6003193601126105a457610d9a6118eb565b60015460ff811615610dd95760ff19166001557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6020604051338152a1005b7f8dfc202b000000000000000000000000000000000000000000000000000000005f5260045ffd5b346105a45760206003193601126105a457600435610e1d611863565b805f52600d60205260ff60405f20541661060b57805f52600d60205260405f20600160ff198254161790557f21c25cd0a54e1609e3e68b335693eefb694d5ef17cc32b106f913f8a1f1b80585f80a2005b346105a4575f6003193601126105a457602060ff600454166040519015158152f35b346105a4575f6003193601126105a457610ea8611794565b506002545f52600660205261077460405f2060026040519161073883611294565b346105a45760406003193601126105a457610ee261122c565b3373ffffffffffffffffffffffffffffffffffffffff821603610f0b576106ab90600435611a67565b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b346105a45760406003193601126105a4576106ab600435610f5261122c565b90610f6b6106a1825f525f602052600160405f20015490565b6119b9565b346105a45760206003193601126105a45760206040610f90600435611726565b601f19601f835194859381855280519182918282880152018686015e5f85828601015201168101030190f35b346105a4575f6003193601126105a4576020600b54604051908152f35b346105a45760206003193601126105a45760206108546004355f525f602052600160405f20015490565b346105a45760406003193601126105a457602073ffffffffffffffffffffffffffffffffffffffff6055600b61103a602435611726565b848151910120604051906040820152600435858201523081520160ff81532016604051908152f35b346105a4576106ab61107336611216565b906113a4565b346105a4575f6003193601126105a4576020610854611356565b346105a45760206003193601126105a4576004355f52600d602052602060ff60405f2054166040519015158152f35b346105a4575f6003193601126105a457604051806020600a5492838152018092600a5f527fc65a7bb8d6351c1cf70c95a316cc6a92839c986682d98bc35f958f4883f9d2a8905f5b81811061116257505050816111209103826112f9565b604051918291602083019060208452518091526040830191905f5b818110611149575050500390f35b825184528594506020938401939092019160010161113b565b825484526020909301926001928301920161110a565b346105a45760206003193601126105a457600435907fffffffff0000000000000000000000000000000000000000000000000000000082168092036105a457817f7965db0b00000000000000000000000000000000000000000000000000000000602093149081156111ec575b5015158152f35b7f01ffc9a700000000000000000000000000000000000000000000000000000000915014836111e5565b60031960409101126105a4576004359060243590565b6024359073ffffffffffffffffffffffffffffffffffffffff821682036105a457565b600a5481101561126757600a5f5260205f2001905f90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b6060810190811067ffffffffffffffff8211176112b057604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6080810190811067ffffffffffffffff8211176112b057604052565b90601f601f19910116810190811067ffffffffffffffff8211176112b057604052565b9190820180921161132957565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b60ff60045416156113925760035462278d00810190818111611329578142101561138c5742900362278d00019081116113295790565b50505f90565b5f90565b5f1981146113295760010190565b9060045460ff8160081c16156116fe5782156116d65781156116ae5760ff8116158061165f575b505060035462278d0081018091116113295780421015611538575b50815f52600960205260ff60405f205416156114e5575b3a903a156114dc575b8181028181048303611329577f229e82b33e0aff76844f66f626d42ad2e6239af34eb0f46d1365fb5aef796db0916040916002545f526005602052825f20865f5260205281835f208054156114d2575b6003600282019161146884845461131c565b83550161147684825461131c565b90556002545f526006602052845f209261149183855461131c565b84556114a26001850191825461131c565b905554146114bd575b506002549382519182526020820152a3565b6002016114ca8154611396565b90555f6114ab565b6003548155611456565b60019150611406565b815f52600960205260405f20600160ff19825416179055600a54680100000000000000008110156112b0578060016115209201600a5561124f565b81549060031b905f1985831b921b19161790556113fd565b90916002545f52600660205260405f20925f5b600a548110156115cc578061156160019261124f565b90549060031b1c6002545f52600560205260405f20815f5260205260405f2080541515806115c1575b611597575b50500161154b565b80878560039301550154905f5260076020526115b860405f2091825461131c565b90555f8061158f565b50838101541561158a565b509150916040611627916115e6600182015460085461131c565b6008557fb463d19ecf455be65365092cf8e1db6934a0334cf8cd532ddf9964d01f36b5b26002549282600285945491015482519182526020820152a2611396565b80600255426003557fa5af2fb7730b51b5fdaf8140d2891589f9d0582d84c8d97dd4e18a3acfe4772f6020604051428152a25f6113e6565b61166a575b806113cb565b60ff1960019116176004555f600255426003555f7fa5af2fb7730b51b5fdaf8140d2891589f9d0582d84c8d97dd4e18a3acfe4772f6020604051428152a25f611664565b7f748c183d000000000000000000000000000000000000000000000000000000005f5260045ffd5b7ff6b4131c000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f80653665000000000000000000000000000000000000000000000000000000005f5260045ffd5b611791610f3b9160206040519161173f828601846112f9565b84835281830194611ece86396040518281019182528281526117626040826112f9565b6040519586945180918587015e840190838201905f8252519283915e01015f815203601f1981018352826112f9565b90565b604051906117a182611294565b5f6040838281528260208201520152565b61180e60206118066117c5600b54611b92565b82806117d2600c54611b92565b6040519584879551918291018487015e8401908282015f8152815193849201905e01015f815203601f1981018352826112f9565b805190611d4b565b90156118175790565b7f94e2737e000000000000000000000000000000000000000000000000000000005f5260045ffd5b6040519061184c826112dd565b5f6060838281528260208201528260408201520152565b335f9081527fe84508f2c7fa9c351146748b3025cb78b45df37d868e48c6a75102fecdeee645602052604090205460ff161561189b57565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b0860245260445ffd5b335f9081527fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5602052604090205460ff161561192357565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f2054161561198a5750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f1461138c57805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f20600160ff1982541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f1461138c57805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060ff19815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190565b15611b1857565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601c60248201527f6e616d65737061636520636f6c6c6973696f6e206465746563746564000000006044820152fd5b67ffffffffffffffff81116112b057601f01601f191660200190565b805f917a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000821015611cef575b806d04ee2d6d415b85acef8100000000600a921015611cd4575b662386f26fc10000811015611cc0575b6305f5e100811015611caf575b612710811015611ca0575b6064811015611c92575b1015611c87575b600a5f1960216001850194601f19611c3c611c2688611b76565b97611c34604051998a6112f9565b808952611b76565b013660208801378501015b01917f30313233343536373839616263646566000000000000000000000000000000008282061a8353048015611c82575f19600a9192611c47565b505090565b600190910190611c0c565b606460029104930192611c05565b61271060049104930192611bfb565b6305f5e10060089104930192611bf0565b662386f26fc1000060109104930192611be3565b6d04ee2d6d415b85acef810000000060209104930192611bd3565b50604091507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008104611bb9565b60ff60015416611d2357565b7fd93c0665000000000000000000000000000000000000000000000000000000005f5260045ffd5b90815181118015611d71575b611d69575f611d6592611d78565b9091565b50505f905f90565b505f611d57565b5f9392905b818310611d8d5750505060019190565b90919360ff611dc37fff000000000000000000000000000000000000000000000000000000000000006020888601015116611e01565b169060098211611df557600a810290808204600a149015171561132957600191611dec9161131c565b94019190611d7d565b5050505090505f905f90565b60f81c602f811180611ec3575b15611e3b577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd00160ff1690565b6060811180611eb9575b15611e72577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa90160ff1690565b6040811180611eaf575b15611ea9577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc90160ff1690565b5060ff90565b5060478110611e7c565b5060678110611e45565b50603a8110611e0e56fe60c0346101a257601f610f3b38819003918201601f19168301916001600160401b0383118484101761017b578084926020946040528339810103126101a25751331561018f575f8054336001600160a01b03198216811783556040519290916001600160a01b0316907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a36001805460ff60a01b1916905560ff8181016001600160401b0381118382101761017b578291610e3c833903905ff0801561017057600180546001600160a01b0319166001600160a01b0392909216919091179055801561012b576080523360a052604051610c9590816101a78239608051818181610139015281816102f901526107e9015260a05181818161029e015281816103ee015261078e0152f35b60405162461bcd60e51b815260206004820152601860248201527f41707020636861696e2049442063616e6e6f74206265203000000000000000006044820152606490fd5b6040513d5f823e3d90fd5b634e487b7160e01b5f52604160045260245ffd5b631e4fbdf760e01b5f525f60045260245ffd5b5f80fdfe60806040526004361015610011575f80fd5b5f5f3560e01c8063366cbab71461085657806346e2cc09146106f3578063485cc955146106085780635b3cd6e2146105d4578063715018a6146105565780637a3979dc146104fa578063804e5123146104455780638da5cb5b14610412578063c45a0155146103c1578063cdafb97814610213578063d4f0eb4d1461015c578063d8781342146101215763f2fde38b146100a9575f80fd5b3461011e57602060031936011261011e576100c2610913565b6100ca610be0565b73ffffffffffffffffffffffffffffffffffffffff8116156100f2576100ef90610c2c565b80f35b6024827f1e4fbdf700000000000000000000000000000000000000000000000000000000815280600452fd5b80fd5b503461011e578060031936011261011e5760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b503461011e57602060031936011261011e5773ffffffffffffffffffffffffffffffffffffffff61018b610913565b610193610be0565b1680156101eb57807fffffffffffffffffffffffff000000000000000000000000000000000000000060015416176001557f253580f806741c11b3d4aa60d9cacc5bef0cebb35748767fe23f11916e2f04b98280a280f35b6004827f5cfe78fe000000000000000000000000000000000000000000000000000000008152fd5b503461011e57602060031936011261011e5760043567ffffffffffffffff81116103bd57366023820112156103bd5780600401359067ffffffffffffffff82116103b9576024810190602436918460051b0101116103b9579082915a91835b81811061034057505050610287905a90610a15565b73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690813b1561033c5782916044839260405194859384927fe3e793700000000000000000000000000000000000000000000000000000000084527f0000000000000000000000000000000000000000000000000000000000000000600485015260248401525af161032b5750f35b8161033591610959565b61011e5780f35b5050fd5b8293945061035d610355828460019596610b53565b903233610a8d565b61036d575b019084939291610272565b61038161037b828587610b53565b906109c7565b7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f604051806103b13394826108cb565b0390a2610362565b8280fd5b5080fd5b503461011e578060031936011261011e57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b503461011e578060031936011261011e5773ffffffffffffffffffffffffffffffffffffffff6020915416604051908152f35b503461011e57602060031936011261011e5760043567ffffffffffffffff81116103bd5761047790369060040161089d565b61048381833233610a8d565b156104d2578291610498610287925a926109c7565b7f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f604051806104c83394826108cb565b0390a25a90610a15565b6004837fdc741458000000000000000000000000000000000000000000000000000000008152fd5b503461011e57606060031936011261011e57610514610913565b61051c610936565b916044359067ffffffffffffffff821161011e57602061054c8585610544366004880161089d565b929091610a8d565b6040519015158152f35b503461011e578060031936011261011e5761056f610be0565b8073ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b503461011e578060031936011261011e57602073ffffffffffffffffffffffffffffffffffffffff60015416604051908152f35b503461011e57604060031936011261011e57610622610913565b61062a610936565b610632610be0565b6001549060ff8260a01c166106cb5773ffffffffffffffffffffffffffffffffffffffff169081156106a3577fffffffffffffffffffffff000000000000000000000000000000000000000000161774010000000000000000000000000000000000000000176001556100ca610be0565b6004847f5cfe78fe000000000000000000000000000000000000000000000000000000008152fd5b6004847f0dc149f0000000000000000000000000000000000000000000000000000000008152fd5b503461082a57602060031936011261082a5760043567ffffffffffffffff811161082a5761072590369060040161089d565b9061073282823233610a8d565b1561082e57610777917f83363b78bdfbb23e2a61db7accc3c01fda29c5c5ec81888003cb962912618a7f6104c85a936040519182916020835233956020840191610a4f565b73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690813b1561082a575f916044839260405194859384927fe3e793700000000000000000000000000000000000000000000000000000000084527f0000000000000000000000000000000000000000000000000000000000000000600485015260248401525af161081c575080f35b61082891505f90610959565b005b5f80fd5b7fdc741458000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461082a57602060031936011261082a5760043567ffffffffffffffff811161082a5761088d61037b61089992369060040161089d565b604051918291826108cb565b0390f35b9181601f8401121561082a5782359167ffffffffffffffff831161082a576020838186019501011161082a57565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602060409481855280519182918282880152018686015e5f8582860101520116010190565b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361082a57565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361082a57565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761099a57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6021610a1291836040519485925f60208501528484013781015f8382015203017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282610959565b90565b91908203918211610a2257565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b9273ffffffffffffffffffffffffffffffffffffffff8094610afb602095836001541694604051988997889687967f7a3979dc000000000000000000000000000000000000000000000000000000008852166004870152166024850152606060448501526064840191610a4f565b03915afa908115610b48575f91610b10575090565b90506020813d602011610b40575b81610b2b60209383610959565b8101031261082a5751801515810361082a5790565b3d9150610b1e565b6040513d5f823e3d90fd5b9190811015610bb35760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18136030182121561082a57019081359167ffffffffffffffff831161082a57602001823603811361082a579190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff5f54163303610c0057565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff805f54921691827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3566080806040523460145760e690816100198239f35b5f80fdfe60808060405260043610156011575f80fd5b5f3560e01c637a3979dc146023575f80fd5b34609e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112609e57605660a2565b50605d60c4565b5060443567ffffffffffffffff8111609e5736602382011215609e57806004013567ffffffffffffffff8111609e5736910160240111609e57805f60209252f35b5f80fd5b6004359073ffffffffffffffffffffffffffffffffffffffff82168203609e57565b6024359073ffffffffffffffffffffffffffffffffffffffff82168203609e5756
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x01\xFF\xC9\xA7\x14a\x11xWP\x80c\x04\xCF\x85\xDE\x14a\x10\xC2W\x80c\rXi\xEE\x14a\x10\x93W\x80c\x13Q\xF9\xC3\x14a\x10yW\x80c\x1B\xD5\0,\x14a\x10bW\x80c\x1FC\xFC\x8B\x14a\x10\x03W\x80c$\x8A\x9C\xA3\x14a\x0F\xD9W\x80c)\x7FLd\x14a\x0F\xBCW\x80c,\xD7\x99\xBD\x14a\x0FpW\x80c//\xF1]\x14a\x0F3W\x80c6V\x8A\xBE\x14a\x0E\xC9W\x80c6_\x16\xE3\x14a\x0E\x90W\x80c;j\xB2\xA9\x14a\x0EnW\x80c;\xFE\xC7\xA2\x14a\x0E\x01W\x80c?K\xA8:\x14a\r\x82W\x80cFTT\x88\x14a\rXW\x80cTg\xCBH\x14a\r\x16W\x80c\\\x97Z\xBB\x14a\x0C\xF4W\x80ca\xA8\xC8\xC4\x14a\x0C\xD7W\x80ceX\x95O\x14a\x0C\xBAW\x80co\xF6\xF6\xC0\x14a\x0C\x1CW\x80cr2\xC13\x14a\x0CyW\x80cvg\x18\x08\x14a\x0C\\W\x80cx\xFC0\xCC\x14a\x0C\x1CW\x80c\x84V\xCBY\x14a\x0B\xC4W\x80c\x84\xFA\xB6+\x14a\x0B\x9FW\x80c\x8C9\xAA\xA4\x14a\n2W\x80c\x8D\xE6\xE2\x8C\x14a\n\x18W\x80c\x8E\x9F\xEA9\x14a\t\xD7W\x80c\x91\xD1HT\x14a\t\x81W\x80c\x93\xC5\xF0\xE1\x14a\tRW\x80c\x99\x9Dq\xD4\x14a\t5W\x80c\x9C\x98Km\x14a\x08\xF6W\x80c\xA2\x17\xFD\xDF\x14a\x08\xDCW\x80c\xA4\xCC\xF9H\x14a\x08\xBFW\x80c\xB9c\xEB\xF3\x14a\x08\\W\x80c\xBA&\x99*\x14a\x08\x0EW\x80c\xC5,!\x0C\x14a\x07xW\x80c\xC8\r\xC4\x1E\x14a\x06\xFEW\x80c\xCCT\x18!\x14a\x06\xADW\x80c\xD5Gt\x1F\x14a\x06iW\x80c\xD9\x85*\xBF\x14a\x03\xD9W\x80c\xDC\xF94u\x14a\x03\xAAW\x80c\xDE\x1FE>\x14a\x03bW\x80c\xE3\xE7\x93p\x14a\x02\x9AW\x80c\xEC\x87b\x1C\x14a\x02_Wc\xF9o\xC5\x1E\x14a\x02#W_\x80\xFD[4a\x02\\W` `\x03\x196\x01\x12a\x02\\W`\x045\x90`\nT\x82\x10\x15a\x02\\W` a\x02M\x83a\x12OV[\x90T\x90`\x03\x1B\x1C`@Q\x90\x81R\xF3[\x80\xFD[P4a\x02\\W\x80`\x03\x196\x01\x12a\x02\\W` `@Q\x7F$\x1E\xCF\x16\xD7\x9D\x0F\x8D\xBF\xB9,\xBC\x07\xFE\x17\x84\x04%\x97l\xF0f\x7F\x02/\xE9\x87|\xAA\x83\x1B\x08\x81R\xF3[P4a\x02\\Wa\x02\xA96a\x12\x16V[\x81\x83R`\x0F` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x84 T\x163\x03a\x02\xDEWa\x02\xDB\x91a\x13\xA4V[\x80\xF3[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FSyndicateFactory: caller not aut`D\x82\x01R\x7Fhorized for this appchain\0\0\0\0\0\0\0`d\x82\x01R\xFD[P4a\x02\\W\x80`\x03\x196\x01\x12a\x02\\Wa\x03{a\x18\xEBV[a\x01\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF`\x04T\x16\x17`\x04U\x80\xF3[P4a\x02\\W` `\x03\x196\x01\x12a\x02\\W`\xFF`@` \x92`\x045\x81R`\t\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x05\xA4W`\x80`\x03\x196\x01\x12a\x05\xA4W`\x045\x90a\x03\xF7a\x12,V[\x91`D5\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x80\x94\x03a\x05\xA4Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90a\x048a\x1D\x17V[\x16\x92\x83\x15\x80\x15a\x06aW[a\x069W\x81\x15\x91\x82\x15a\x063WPa\x04Ya\x17\xB2V[\x91[\x82_R`\r` R`\xFF`@_ T\x16a\x06\x0BW\x82_R`\r` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90Ua\x05\xF8W[a\x04\x94\x82a\x17&V[\x80Q\x15a\x05\xD0W\x80Q`d5\x91` \x01_\xF5=\x15\x19\x81\x15\x16a\x05\x99Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x05\xA8W\x82_R`\x0F` R`@_ \x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90U\x81;\x15a\x05\xA4W`@Q\x94\x7FH\\\xC9U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\x04\x86\x01R\x80`$\x86\x01R_\x85`D\x81\x83\x86Z\xF1\x94\x85\x15a\x05\x99W`@\x95a\x05\x84W[P\x81\x83\x7FI\xB2\x1F\x1EA\x90\xDB\x8B\n\x93<\x95\x1E\xD0\x13\xDE\",\x84|\x15F\x17Th-\xAA.\xAB\x1F\xDB\xD2\x87Q\x96\x80\xA4\x82R` \x82\x01R\xF3[a\x05\x91\x91\x94P_\x90a\x12\xF9V[_\x92_a\x05RV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x7F\xB0n\xBF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FL\xA2I\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[a\x06\x03`\x0CTa\x13\x96V[`\x0CUa\x04\x8BV[\x7F$Y\x1D\x89\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x91a\x04[V[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P\x80\x15a\x04CV[4a\x05\xA4W`@`\x03\x196\x01\x12a\x05\xA4Wa\x06\xAB`\x045a\x06\x88a\x12,V[\x90a\x06\xA6a\x06\xA1\x82_R_` R`\x01`@_ \x01T\x90V[a\x19SV[a\x1AgV[\0[4a\x05\xA4Wa\x06\xBB6a\x12\x16V[\x90_R`\x05` R`@_ \x90_R` R`\x80`@_ \x80T\x90`\x01\x81\x01T\x90`\x03`\x02\x82\x01T\x91\x01T\x91`@Q\x93\x84R` \x84\x01R`@\x83\x01R``\x82\x01R\xF3[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4Wa\x07\x17a\x17\x94V[P`\x045_R`\x06` Ra\x07t`@_ `\x02`@Q\x91a\x078\x83a\x12\x94V[\x80T\x83R`\x01\x81\x01T` \x84\x01R\x01T`@\x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91`@\x80``\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R\x01Q\x91\x01RV[\x03\x90\xF3[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4Wa\x07\x91a\x18?V[P`\x02T_R`\x05` R`@_ `\x045_R` Ra\x07t`@_ `\x03`@Q\x91a\x07\xBE\x83a\x12\xDDV[\x80T\x83R`\x01\x81\x01T` \x84\x01R`\x02\x81\x01T`@\x84\x01R\x01T``\x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91``\x80`\x80\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91\x01RV[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W` a\x08T`\x045\x80_R`\x07\x83R`@_ T\x90`\x02T_R`\x05\x84R`@_ \x90_R\x83R`\x03`@_ \x01T\x90a\x13\x1CV[`@Q\x90\x81R\xF3[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045_R`\x0E` R`\xFF`@_ T\x16`@Q`\x03\x82\x10\x15a\x08\x92W` \x91\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\x08T`@Q\x90\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `@Q_\x81R\xF3[4a\x05\xA4Wa\t\x046a\x12\x16V[\x90a\t\ra\x18?V[P_R`\x05` R`@_ \x90_R` Ra\x07t`@_ `\x03`@Q\x91a\x07\xBE\x83a\x12\xDDV[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\x0CT`@Q\x90\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` a\x08T`\x08T`\x02T_R`\x06\x83R`\x01`@_ \x01T\x90a\x13\x1CV[4a\x05\xA4W`@`\x03\x196\x01\x12a\x05\xA4Wa\t\x9Aa\x12,V[`\x045_R_` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045_R`\x06` R```@_ \x80T\x90`\x02`\x01\x82\x01T\x91\x01T\x90`@Q\x92\x83R` \x83\x01R`@\x82\x01R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` a\x08Ta\x17\xB2V[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045a\nNa\x18cV[\x80\x15a\x0B\x1BW\x80_R`\x0E` R`\xFF`@_ T\x16`\x03\x81\x10\x15a\x08\x92W`\x02a\nz\x91\x14\x15a\x1B\x11V[\x80_R`\x0E` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`\n\x81\x04\x80[a\n\xD4W\x7F\xD9\xE1#\x91w\xBF\xBD*\xEB\xF5\xD0\xF2\x0F\xC0u\xE6\xDFZP,Y\xD1!\xAC\xD5sB\xC7\x83\xE3\x13d`@\x83`\x0BT\x90\x80`\x0BU\x82Q\x91\x82R` \x82\x01R\xA1\0[\x80_R`\x0E` R`\xFF`@_ T\x16\x90`\x03\x82\x10\x15a\x08\x92Wa\n\xFD`\x01`\n\x93\x14\x15a\x1B\x11V[\x80_R`\x0E` R`@_ `\x02`\xFF\x19\x82T\x16\x17\x90U\x04\x80a\n\x97V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Fnamespace prefix of 0 is forbidd`D\x82\x01R\x7Fen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\xFF`\x04T`\x08\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4Wa\x0B\xDCa\x18\xEBV[a\x0B\xE4a\x1D\x17V[`\x01`\xFF\x19\x81T\x16\x17`\x01U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045_R`\x0F` R` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16`@Q\x90\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\x02T`@Q\x90\x81R\xF3[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045_R`\r` R`\xFF`@_ T\x16_\x14a\x0C\xB2W` `\x01[`\xFF`@Q\x91\x16\x81R\xF3[` _a\x0C\xA7V[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `@Qb'\x8D\0\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\x03T`@Q\x90\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\xFF`\x01T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4Wa\r.a\x18\xEBV[`\x04\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U\0[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045_R`\x07` R` `@_ T`@Q\x90\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4Wa\r\x9Aa\x18\xEBV[`\x01T`\xFF\x81\x16\x15a\r\xD9W`\xFF\x19\x16`\x01U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\0[\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045a\x0E\x1Da\x18cV[\x80_R`\r` R`\xFF`@_ T\x16a\x06\x0BW\x80_R`\r` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F!\xC2\\\xD0\xA5N\x16\t\xE3\xE6\x8B3V\x93\xEE\xFBiM^\xF1|\xC3+\x10o\x91?\x8A\x1F\x1B\x80X_\x80\xA2\0[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\xFF`\x04T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4Wa\x0E\xA8a\x17\x94V[P`\x02T_R`\x06` Ra\x07t`@_ `\x02`@Q\x91a\x078\x83a\x12\x94V[4a\x05\xA4W`@`\x03\x196\x01\x12a\x05\xA4Wa\x0E\xE2a\x12,V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\x0F\x0BWa\x06\xAB\x90`\x045a\x1AgV[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x05\xA4W`@`\x03\x196\x01\x12a\x05\xA4Wa\x06\xAB`\x045a\x0FRa\x12,V[\x90a\x0Fka\x06\xA1\x82_R_` R`\x01`@_ \x01T\x90V[a\x19\xB9V[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W` `@a\x0F\x90`\x045a\x17&V[`\x1F\x19`\x1F\x83Q\x94\x85\x93\x81\x85R\x80Q\x91\x82\x91\x82\x82\x88\x01R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x81\x01\x03\x01\x90\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` `\x0BT`@Q\x90\x81R\xF3[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W` a\x08T`\x045_R_` R`\x01`@_ \x01T\x90V[4a\x05\xA4W`@`\x03\x196\x01\x12a\x05\xA4W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`U`\x0Ba\x10:`$5a\x17&V[\x84\x81Q\x91\x01 `@Q\x90`@\x82\x01R`\x045\x85\x82\x01R0\x81R\x01`\xFF\x81S \x16`@Q\x90\x81R\xF3[4a\x05\xA4Wa\x06\xABa\x10s6a\x12\x16V[\x90a\x13\xA4V[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W` a\x08Ta\x13VV[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045_R`\r` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x05\xA4W_`\x03\x196\x01\x12a\x05\xA4W`@Q\x80` `\nT\x92\x83\x81R\x01\x80\x92`\n_R\x7F\xC6Z{\xB8\xD65\x1C\x1C\xF7\x0C\x95\xA3\x16\xCCj\x92\x83\x9C\x98f\x82\xD9\x8B\xC3_\x95\x8FH\x83\xF9\xD2\xA8\x90_[\x81\x81\x10a\x11bWPPP\x81a\x11 \x91\x03\x82a\x12\xF9V[`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x11IWPPP\x03\x90\xF3[\x82Q\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x11;V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x11\nV[4a\x05\xA4W` `\x03\x196\x01\x12a\x05\xA4W`\x045\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x80\x92\x03a\x05\xA4W\x81\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x93\x14\x90\x81\x15a\x11\xECW[P\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x83a\x11\xE5V[`\x03\x19`@\x91\x01\x12a\x05\xA4W`\x045\x90`$5\x90V[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x05\xA4WV[`\nT\x81\x10\x15a\x12gW`\n_R` _ \x01\x90_\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x12\xB0W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x12\xB0W`@RV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x12\xB0W`@RV[\x91\x90\x82\x01\x80\x92\x11a\x13)WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[`\xFF`\x04T\x16\x15a\x13\x92W`\x03Tb'\x8D\0\x81\x01\x90\x81\x81\x11a\x13)W\x81B\x10\x15a\x13\x8CWB\x90\x03b'\x8D\0\x01\x90\x81\x11a\x13)W\x90V[PP_\x90V[_\x90V[_\x19\x81\x14a\x13)W`\x01\x01\x90V[\x90`\x04T`\xFF\x81`\x08\x1C\x16\x15a\x16\xFEW\x82\x15a\x16\xD6W\x81\x15a\x16\xAEW`\xFF\x81\x16\x15\x80a\x16_W[PP`\x03Tb'\x8D\0\x81\x01\x80\x91\x11a\x13)W\x80B\x10\x15a\x158W[P\x81_R`\t` R`\xFF`@_ T\x16\x15a\x14\xE5W[:\x90:\x15a\x14\xDCW[\x81\x81\x02\x81\x81\x04\x83\x03a\x13)W\x7F\"\x9E\x82\xB3>\n\xFFv\x84Of\xF6&\xD4*\xD2\xE6#\x9A\xF3N\xB0\xF4m\x13e\xFBZ\xEFym\xB0\x91`@\x91`\x02T_R`\x05` R\x82_ \x86_R` R\x81\x83_ \x80T\x15a\x14\xD2W[`\x03`\x02\x82\x01\x91a\x14h\x84\x84Ta\x13\x1CV[\x83U\x01a\x14v\x84\x82Ta\x13\x1CV[\x90U`\x02T_R`\x06` R\x84_ \x92a\x14\x91\x83\x85Ta\x13\x1CV[\x84Ua\x14\xA2`\x01\x85\x01\x91\x82Ta\x13\x1CV[\x90UT\x14a\x14\xBDW[P`\x02T\x93\x82Q\x91\x82R` \x82\x01R\xA3V[`\x02\x01a\x14\xCA\x81Ta\x13\x96V[\x90U_a\x14\xABV[`\x03T\x81Ua\x14VV[`\x01\x91Pa\x14\x06V[\x81_R`\t` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`\nTh\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x12\xB0W\x80`\x01a\x15 \x92\x01`\nUa\x12OV[\x81T\x90`\x03\x1B\x90_\x19\x85\x83\x1B\x92\x1B\x19\x16\x17\x90Ua\x13\xFDV[\x90\x91`\x02T_R`\x06` R`@_ \x92_[`\nT\x81\x10\x15a\x15\xCCW\x80a\x15a`\x01\x92a\x12OV[\x90T\x90`\x03\x1B\x1C`\x02T_R`\x05` R`@_ \x81_R` R`@_ \x80T\x15\x15\x80a\x15\xC1W[a\x15\x97W[PP\x01a\x15KV[\x80\x87\x85`\x03\x93\x01U\x01T\x90_R`\x07` Ra\x15\xB8`@_ \x91\x82Ta\x13\x1CV[\x90U_\x80a\x15\x8FV[P\x83\x81\x01T\x15a\x15\x8AV[P\x91P\x91`@a\x16'\x91a\x15\xE6`\x01\x82\x01T`\x08Ta\x13\x1CV[`\x08U\x7F\xB4c\xD1\x9E\xCFE[\xE6Se\t,\xF8\xE1\xDBi4\xA03L\xF8\xCDS-\xDF\x99d\xD0\x1F6\xB5\xB2`\x02T\x92\x82`\x02\x85\x94T\x91\x01T\x82Q\x91\x82R` \x82\x01R\xA2a\x13\x96V[\x80`\x02UB`\x03U\x7F\xA5\xAF/\xB7s\x0BQ\xB5\xFD\xAF\x81@\xD2\x89\x15\x89\xF9\xD0X-\x84\xC8\xD9}\xD4\xE1\x8A:\xCF\xE4w/` `@QB\x81R\xA2_a\x13\xE6V[a\x16jW[\x80a\x13\xCBV[`\xFF\x19`\x01\x91\x16\x17`\x04U_`\x02UB`\x03U_\x7F\xA5\xAF/\xB7s\x0BQ\xB5\xFD\xAF\x81@\xD2\x89\x15\x89\xF9\xD0X-\x84\xC8\xD9}\xD4\xE1\x8A:\xCF\xE4w/` `@QB\x81R\xA2_a\x16dV[\x7Ft\x8C\x18=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xF6\xB4\x13\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x80e6e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[a\x17\x91a\x0F;\x91` `@Q\x91a\x17?\x82\x86\x01\x84a\x12\xF9V[\x84\x83R\x81\x83\x01\x94a\x1E\xCE\x869`@Q\x82\x81\x01\x91\x82R\x82\x81Ra\x17b`@\x82a\x12\xF9V[`@Q\x95\x86\x94Q\x80\x91\x85\x87\x01^\x84\x01\x90\x83\x82\x01\x90_\x82RQ\x92\x83\x91^\x01\x01_\x81R\x03`\x1F\x19\x81\x01\x83R\x82a\x12\xF9V[\x90V[`@Q\x90a\x17\xA1\x82a\x12\x94V[_`@\x83\x82\x81R\x82` \x82\x01R\x01RV[a\x18\x0E` a\x18\x06a\x17\xC5`\x0BTa\x1B\x92V[\x82\x80a\x17\xD2`\x0CTa\x1B\x92V[`@Q\x95\x84\x87\x95Q\x91\x82\x91\x01\x84\x87\x01^\x84\x01\x90\x82\x82\x01_\x81R\x81Q\x93\x84\x92\x01\x90^\x01\x01_\x81R\x03`\x1F\x19\x81\x01\x83R\x82a\x12\xF9V[\x80Q\x90a\x1DKV[\x90\x15a\x18\x17W\x90V[\x7F\x94\xE2s~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`@Q\x90a\x18L\x82a\x12\xDDV[_``\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[3_\x90\x81R\x7F\xE8E\x08\xF2\xC7\xFA\x9C5\x11Ft\x8B0%\xCBx\xB4]\xF3}\x86\x8EH\xC6\xA7Q\x02\xFE\xCD\xEE\xE6E` R`@\x90 T`\xFF\x16\x15a\x18\x9BWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F$\x1E\xCF\x16\xD7\x9D\x0F\x8D\xBF\xB9,\xBC\x07\xFE\x17\x84\x04%\x97l\xF0f\x7F\x02/\xE9\x87|\xAA\x83\x1B\x08`$R`D_\xFD[3_\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16\x15a\x19#WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x19\x8AWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x13\x8CW\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x13\x8CW\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\xFF\x19\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V[\x15a\x1B\x18WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fnamespace collision detected\0\0\0\0`D\x82\x01R\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x12\xB0W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x80_\x91z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x82\x10\x15a\x1C\xEFW[\x80m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0`\n\x92\x10\x15a\x1C\xD4W[f#\x86\xF2o\xC1\0\0\x81\x10\x15a\x1C\xC0W[c\x05\xF5\xE1\0\x81\x10\x15a\x1C\xAFW[a'\x10\x81\x10\x15a\x1C\xA0W[`d\x81\x10\x15a\x1C\x92W[\x10\x15a\x1C\x87W[`\n_\x19`!`\x01\x85\x01\x94`\x1F\x19a\x1C<a\x1C&\x88a\x1BvV[\x97a\x1C4`@Q\x99\x8Aa\x12\xF9V[\x80\x89Ra\x1BvV[\x016` \x88\x017\x85\x01\x01[\x01\x91\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x82\x06\x1A\x83S\x04\x80\x15a\x1C\x82W_\x19`\n\x91\x92a\x1CGV[PP\x90V[`\x01\x90\x91\x01\x90a\x1C\x0CV[`d`\x02\x91\x04\x93\x01\x92a\x1C\x05V[a'\x10`\x04\x91\x04\x93\x01\x92a\x1B\xFBV[c\x05\xF5\xE1\0`\x08\x91\x04\x93\x01\x92a\x1B\xF0V[f#\x86\xF2o\xC1\0\0`\x10\x91\x04\x93\x01\x92a\x1B\xE3V[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0` \x91\x04\x93\x01\x92a\x1B\xD3V[P`@\x91Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x81\x04a\x1B\xB9V[`\xFF`\x01T\x16a\x1D#WV[\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90\x81Q\x81\x11\x80\x15a\x1DqW[a\x1DiW_a\x1De\x92a\x1DxV[\x90\x91V[PP_\x90_\x90V[P_a\x1DWV[_\x93\x92\x90[\x81\x83\x10a\x1D\x8DWPPP`\x01\x91\x90V[\x90\x91\x93`\xFFa\x1D\xC3\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x88\x86\x01\x01Q\x16a\x1E\x01V[\x16\x90`\t\x82\x11a\x1D\xF5W`\n\x81\x02\x90\x80\x82\x04`\n\x14\x90\x15\x17\x15a\x13)W`\x01\x91a\x1D\xEC\x91a\x13\x1CV[\x94\x01\x91\x90a\x1D}V[PPPP\x90P_\x90_\x90V[`\xF8\x1C`/\x81\x11\x80a\x1E\xC3W[\x15a\x1E;W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xD0\x01`\xFF\x16\x90V[``\x81\x11\x80a\x1E\xB9W[\x15a\x1ErW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA9\x01`\xFF\x16\x90V[`@\x81\x11\x80a\x1E\xAFW[\x15a\x1E\xA9W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC9\x01`\xFF\x16\x90V[P`\xFF\x90V[P`G\x81\x10a\x1E|V[P`g\x81\x10a\x1EEV[P`:\x81\x10a\x1E\x0EV\xFE`\xC04a\x01\xA2W`\x1Fa\x0F;8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01{W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\x01\xA2WQ3\x15a\x01\x8FW_\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x83U`@Q\x92\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3`\x01\x80T`\xFF`\xA0\x1B\x19\x16\x90U`\xFF\x81\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x83\x82\x10\x17a\x01{W\x82\x91a\x0E<\x839\x03\x90_\xF0\x80\x15a\x01pW`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80\x15a\x01+W`\x80R3`\xA0R`@Qa\x0C\x95\x90\x81a\x01\xA7\x829`\x80Q\x81\x81\x81a\x019\x01R\x81\x81a\x02\xF9\x01Ra\x07\xE9\x01R`\xA0Q\x81\x81\x81a\x02\x9E\x01R\x81\x81a\x03\xEE\x01Ra\x07\x8E\x01R\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FApp chain ID cannot be 0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`@Q=_\x82>=\x90\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c6l\xBA\xB7\x14a\x08VW\x80cF\xE2\xCC\t\x14a\x06\xF3W\x80cH\\\xC9U\x14a\x06\x08W\x80c[<\xD6\xE2\x14a\x05\xD4W\x80cqP\x18\xA6\x14a\x05VW\x80cz9y\xDC\x14a\x04\xFAW\x80c\x80NQ#\x14a\x04EW\x80c\x8D\xA5\xCB[\x14a\x04\x12W\x80c\xC4Z\x01U\x14a\x03\xC1W\x80c\xCD\xAF\xB9x\x14a\x02\x13W\x80c\xD4\xF0\xEBM\x14a\x01\\W\x80c\xD8x\x13B\x14a\x01!Wc\xF2\xFD\xE3\x8B\x14a\0\xA9W_\x80\xFD[4a\x01\x1EW` `\x03\x196\x01\x12a\x01\x1EWa\0\xC2a\t\x13V[a\0\xCAa\x0B\xE0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a\0\xF2Wa\0\xEF\x90a\x0C,V[\x80\xF3[`$\x82\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x80`\x04R\xFD[\x80\xFD[P4a\x01\x1EW\x80`\x03\x196\x01\x12a\x01\x1EW` `@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xF3[P4a\x01\x1EW` `\x03\x196\x01\x12a\x01\x1EWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\x8Ba\t\x13V[a\x01\x93a\x0B\xE0V[\x16\x80\x15a\x01\xEBW\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01T\x16\x17`\x01U\x7F%5\x80\xF8\x06t\x1C\x11\xB3\xD4\xAA`\xD9\xCA\xCC[\xEF\x0C\xEB\xB3WHv\x7F\xE2?\x11\x91n/\x04\xB9\x82\x80\xA2\x80\xF3[`\x04\x82\x7F\\\xFEx\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x01\x1EW` `\x03\x196\x01\x12a\x01\x1EW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\xBDW6`#\x82\x01\x12\x15a\x03\xBDW\x80`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03\xB9W`$\x81\x01\x90`$6\x91\x84`\x05\x1B\x01\x01\x11a\x03\xB9W\x90\x82\x91Z\x91\x83[\x81\x81\x10a\x03@WPPPa\x02\x87\x90Z\x90a\n\x15V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x81;\x15a\x03<W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\xE3\xE7\x93p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x85\x01R`$\x84\x01RZ\xF1a\x03+WP\xF3[\x81a\x035\x91a\tYV[a\x01\x1EW\x80\xF3[PP\xFD[\x82\x93\x94Pa\x03]a\x03U\x82\x84`\x01\x95\x96a\x0BSV[\x9023a\n\x8DV[a\x03mW[\x01\x90\x84\x93\x92\x91a\x02rV[a\x03\x81a\x03{\x82\x85\x87a\x0BSV[\x90a\t\xC7V[\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F`@Q\x80a\x03\xB13\x94\x82a\x08\xCBV[\x03\x90\xA2a\x03bV[\x82\x80\xFD[P\x80\xFD[P4a\x01\x1EW\x80`\x03\x196\x01\x12a\x01\x1EW` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x01\x1EW\x80`\x03\x196\x01\x12a\x01\x1EWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x91T\x16`@Q\x90\x81R\xF3[P4a\x01\x1EW` `\x03\x196\x01\x12a\x01\x1EW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\xBDWa\x04w\x906\x90`\x04\x01a\x08\x9DV[a\x04\x83\x81\x8323a\n\x8DV[\x15a\x04\xD2W\x82\x91a\x04\x98a\x02\x87\x92Z\x92a\t\xC7V[\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7F`@Q\x80a\x04\xC83\x94\x82a\x08\xCBV[\x03\x90\xA2Z\x90a\n\x15V[`\x04\x83\x7F\xDCt\x14X\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x01\x1EW```\x03\x196\x01\x12a\x01\x1EWa\x05\x14a\t\x13V[a\x05\x1Ca\t6V[\x91`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\x1EW` a\x05L\x85\x85a\x05D6`\x04\x88\x01a\x08\x9DV[\x92\x90\x91a\n\x8DV[`@Q\x90\x15\x15\x81R\xF3[P4a\x01\x1EW\x80`\x03\x196\x01\x12a\x01\x1EWa\x05oa\x0B\xE0V[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P4a\x01\x1EW\x80`\x03\x196\x01\x12a\x01\x1EW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01T\x16`@Q\x90\x81R\xF3[P4a\x01\x1EW`@`\x03\x196\x01\x12a\x01\x1EWa\x06\"a\t\x13V[a\x06*a\t6V[a\x062a\x0B\xE0V[`\x01T\x90`\xFF\x82`\xA0\x1C\x16a\x06\xCBWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x06\xA3W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17`\x01Ua\0\xCAa\x0B\xE0V[`\x04\x84\x7F\\\xFEx\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`\x04\x84\x7F\r\xC1I\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x08*W` `\x03\x196\x01\x12a\x08*W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08*Wa\x07%\x906\x90`\x04\x01a\x08\x9DV[\x90a\x072\x82\x8223a\n\x8DV[\x15a\x08.Wa\x07w\x91\x7F\x836;x\xBD\xFB\xB2>*a\xDBz\xCC\xC3\xC0\x1F\xDA)\xC5\xC5\xEC\x81\x88\x80\x03\xCB\x96)\x12a\x8A\x7Fa\x04\xC8Z\x93`@Q\x91\x82\x91` \x83R3\x95` \x84\x01\x91a\nOV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x81;\x15a\x08*W_\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\xE3\xE7\x93p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x85\x01R`$\x84\x01RZ\xF1a\x08\x1CWP\x80\xF3[a\x08(\x91P_\x90a\tYV[\0[_\x80\xFD[\x7F\xDCt\x14X\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x08*W` `\x03\x196\x01\x12a\x08*W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08*Wa\x08\x8Da\x03{a\x08\x99\x926\x90`\x04\x01a\x08\x9DV[`@Q\x91\x82\x91\x82a\x08\xCBV[\x03\x90\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x08*W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x08*W` \x83\x81\x86\x01\x95\x01\x01\x11a\x08*WV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` `@\x94\x81\x85R\x80Q\x91\x82\x91\x82\x82\x88\x01R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x08*WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x08*WV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\x9AW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`!a\n\x12\x91\x83`@Q\x94\x85\x92_` \x85\x01R\x84\x84\x017\x81\x01_\x83\x82\x01R\x03\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a\tYV[\x90V[\x91\x90\x82\x03\x91\x82\x11a\n\"WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x94a\n\xFB` \x95\x83`\x01T\x16\x94`@Q\x98\x89\x97\x88\x96\x87\x96\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88R\x16`\x04\x87\x01R\x16`$\x85\x01R```D\x85\x01R`d\x84\x01\x91a\nOV[\x03\x91Z\xFA\x90\x81\x15a\x0BHW_\x91a\x0B\x10WP\x90V[\x90P` \x81=` \x11a\x0B@W[\x81a\x0B+` \x93\x83a\tYV[\x81\x01\x03\x12a\x08*WQ\x80\x15\x15\x81\x03a\x08*W\x90V[=\x91Pa\x0B\x1EV[`@Q=_\x82>=\x90\xFD[\x91\x90\x81\x10\x15a\x0B\xB3W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x08*W\x01\x90\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x08*W` \x01\x826\x03\x81\x13a\x08*W\x91\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\x0C\0WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80_T\x92\x16\x91\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V`\x80\x80`@R4`\x14W`\xE6\x90\x81a\0\x19\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15`\x11W_\x80\xFD[_5`\xE0\x1Ccz9y\xDC\x14`#W_\x80\xFD[4`\x9EW``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12`\x9EW`V`\xA2V[P`]`\xC4V[P`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x9EW6`#\x82\x01\x12\x15`\x9EW\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x9EW6\x91\x01`$\x01\x11`\x9EW\x80_` \x92R\xF3[_\x80\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03`\x9EWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03`\x9EWV",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NamespaceState(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<NamespaceState> for u8 {
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
        impl NamespaceState {
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
        impl From<u8> for NamespaceState {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<NamespaceState> for u8 {
            fn from(value: NamespaceState) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for NamespaceState {
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
        impl alloy_sol_types::EventTopic for NamespaceState {
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AccessControlBadConfirmation()` and selector `0x6697b232`.
```solidity
error AccessControlBadConfirmation();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlBadConfirmation;
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
                Self
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ChainIdAlreadyExists()` and selector `0x24591d89`.
```solidity
error ChainIdAlreadyExists();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChainIdAlreadyExists;
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
        impl ::core::convert::From<ChainIdAlreadyExists> for UnderlyingRustTuple<'_> {
            fn from(value: ChainIdAlreadyExists) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ChainIdAlreadyExists {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ChainIdAlreadyExists {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ChainIdAlreadyExists()";
            const SELECTOR: [u8; 4] = [36u8, 89u8, 29u8, 137u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `Create2EmptyBytecode()` and selector `0x4ca249dc`.
```solidity
error Create2EmptyBytecode();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Create2EmptyBytecode;
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
        impl ::core::convert::From<Create2EmptyBytecode> for UnderlyingRustTuple<'_> {
            fn from(value: Create2EmptyBytecode) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Create2EmptyBytecode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Create2EmptyBytecode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Create2EmptyBytecode()";
            const SELECTOR: [u8; 4] = [76u8, 162u8, 73u8, 220u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EnforcedPause()` and selector `0xd93c0665`.
```solidity
error EnforcedPause();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EnforcedPause;
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
        impl ::core::convert::From<EnforcedPause> for UnderlyingRustTuple<'_> {
            fn from(value: EnforcedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EnforcedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EnforcedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EnforcedPause()";
            const SELECTOR: [u8; 4] = [217u8, 60u8, 6u8, 101u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ExpectedPause()` and selector `0x8dfc202b`.
```solidity
error ExpectedPause();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpectedPause;
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
        impl ::core::convert::From<ExpectedPause> for UnderlyingRustTuple<'_> {
            fn from(value: ExpectedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpectedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpectedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpectedPause()";
            const SELECTOR: [u8; 4] = [141u8, 252u8, 32u8, 43u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `FailedDeployment()` and selector `0xb06ebf3d`.
```solidity
error FailedDeployment();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedDeployment;
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
        impl ::core::convert::From<FailedDeployment> for UnderlyingRustTuple<'_> {
            fn from(value: FailedDeployment) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedDeployment {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FailedDeployment {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FailedDeployment()";
            const SELECTOR: [u8; 4] = [176u8, 110u8, 191u8, 61u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `GasTrackingDisabled()` and selector `0x80653665`.
```solidity
error GasTrackingDisabled();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GasTrackingDisabled;
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
        impl ::core::convert::From<GasTrackingDisabled> for UnderlyingRustTuple<'_> {
            fn from(value: GasTrackingDisabled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GasTrackingDisabled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GasTrackingDisabled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GasTrackingDisabled()";
            const SELECTOR: [u8; 4] = [128u8, 101u8, 54u8, 101u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InsufficientBalance(uint256,uint256)` and selector `0xcf479181`.
```solidity
error InsufficientBalance(uint256 balance, uint256 needed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientBalance {
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
        impl ::core::convert::From<InsufficientBalance> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientBalance) -> Self {
                (value.balance, value.needed)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientBalance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    balance: tuple.0,
                    needed: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientBalance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientBalance(uint256,uint256)";
            const SELECTOR: [u8; 4] = [207u8, 71u8, 145u8, 129u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.balance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.needed),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidAppchainId()` and selector `0xf6b4131c`.
```solidity
error InvalidAppchainId();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAppchainId;
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
        impl ::core::convert::From<InvalidAppchainId> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAppchainId) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAppchainId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAppchainId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAppchainId()";
            const SELECTOR: [u8; 4] = [246u8, 180u8, 19u8, 28u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidGasAmount()` and selector `0x748c183d`.
```solidity
error InvalidGasAmount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidGasAmount;
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
        impl ::core::convert::From<InvalidGasAmount> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidGasAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidGasAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidGasAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidGasAmount()";
            const SELECTOR: [u8; 4] = [116u8, 140u8, 24u8, 61u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `StringsInvalidChar()` and selector `0x94e2737e`.
```solidity
error StringsInvalidChar();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StringsInvalidChar;
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
        impl ::core::convert::From<StringsInvalidChar> for UnderlyingRustTuple<'_> {
            fn from(value: StringsInvalidChar) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StringsInvalidChar {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StringsInvalidChar {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StringsInvalidChar()";
            const SELECTOR: [u8; 4] = [148u8, 226u8, 115u8, 126u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroAddress()` and selector `0xd92e233d`.
```solidity
error ZeroAddress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAddress;
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
                Self
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ChainIdManuallyMarked(uint256)` and selector `0x21c25cd0a54e1609e3e68b335693eefb694d5ef17cc32b106f913f8a1f1b8058`.
```solidity
event ChainIdManuallyMarked(uint256 indexed chainId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChainIdManuallyMarked {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for ChainIdManuallyMarked {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "ChainIdManuallyMarked(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                33u8, 194u8, 92u8, 208u8, 165u8, 78u8, 22u8, 9u8, 227u8, 230u8, 139u8,
                51u8, 86u8, 147u8, 238u8, 251u8, 105u8, 77u8, 94u8, 241u8, 124u8, 195u8,
                43u8, 16u8, 111u8, 145u8, 63u8, 138u8, 31u8, 27u8, 128u8, 88u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { chainId: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.chainId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.chainId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChainIdManuallyMarked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChainIdManuallyMarked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChainIdManuallyMarked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EpochFinalized(uint256,uint256,uint256)` and selector `0xb463d19ecf455be65365092cf8e1db6934a0334cf8cd532ddf9964d01f36b5b2`.
```solidity
event EpochFinalized(uint256 indexed epoch, uint256 totalGasUsed, uint256 activeAppchains);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EpochFinalized {
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalGasUsed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub activeAppchains: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for EpochFinalized {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "EpochFinalized(uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                180u8, 99u8, 209u8, 158u8, 207u8, 69u8, 91u8, 230u8, 83u8, 101u8, 9u8,
                44u8, 248u8, 225u8, 219u8, 105u8, 52u8, 160u8, 51u8, 76u8, 248u8, 205u8,
                83u8, 45u8, 223u8, 153u8, 100u8, 208u8, 31u8, 54u8, 181u8, 178u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    epoch: topics.1,
                    totalGasUsed: data.0,
                    activeAppchains: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.totalGasUsed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.activeAppchains),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.epoch.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.epoch);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EpochFinalized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EpochFinalized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EpochFinalized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `GasTracked(uint256,uint256,uint256,uint256)` and selector `0x229e82b33e0aff76844f66f626d42ad2e6239af34eb0f46d1365fb5aef796db0`.
```solidity
event GasTracked(uint256 indexed epoch, uint256 indexed appchainId, uint256 gasUsed, uint256 gasPrice);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct GasTracked {
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub appchainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasUsed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasPrice: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for GasTracked {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "GasTracked(uint256,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                34u8, 158u8, 130u8, 179u8, 62u8, 10u8, 255u8, 118u8, 132u8, 79u8, 102u8,
                246u8, 38u8, 212u8, 42u8, 210u8, 230u8, 35u8, 154u8, 243u8, 78u8, 176u8,
                244u8, 109u8, 19u8, 101u8, 251u8, 90u8, 239u8, 121u8, 109u8, 176u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    epoch: topics.1,
                    appchainId: topics.2,
                    gasUsed: data.0,
                    gasPrice: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.gasUsed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasPrice),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.epoch.clone(),
                    self.appchainId.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.epoch);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.appchainId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for GasTracked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&GasTracked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &GasTracked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `NamespaceConfigUpdated(uint256,uint256)` and selector `0xd9e1239177bfbd2aebf5d0f20fc075e6df5a502c59d121acd57342c783e31364`.
```solidity
event NamespaceConfigUpdated(uint256 oldNamespacePrefix, uint256 newNamespacePrefix);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NamespaceConfigUpdated {
        #[allow(missing_docs)]
        pub oldNamespacePrefix: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newNamespacePrefix: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for NamespaceConfigUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "NamespaceConfigUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                217u8, 225u8, 35u8, 145u8, 119u8, 191u8, 189u8, 42u8, 235u8, 245u8,
                208u8, 242u8, 15u8, 192u8, 117u8, 230u8, 223u8, 90u8, 80u8, 44u8, 89u8,
                209u8, 33u8, 172u8, 213u8, 115u8, 66u8, 199u8, 131u8, 227u8, 19u8, 100u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldNamespacePrefix: data.0,
                    newNamespacePrefix: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.oldNamespacePrefix),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newNamespacePrefix),
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
        impl alloy_sol_types::private::IntoLogData for NamespaceConfigUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NamespaceConfigUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NamespaceConfigUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `NewEpochStarted(uint256,uint256)` and selector `0xa5af2fb7730b51b5fdaf8140d2891589f9d0582d84c8d97dd4e18a3acfe4772f`.
```solidity
event NewEpochStarted(uint256 indexed epoch, uint256 startTimestamp);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewEpochStarted {
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub startTimestamp: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for NewEpochStarted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "NewEpochStarted(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                165u8, 175u8, 47u8, 183u8, 115u8, 11u8, 81u8, 181u8, 253u8, 175u8, 129u8,
                64u8, 210u8, 137u8, 21u8, 137u8, 249u8, 208u8, 88u8, 45u8, 132u8, 200u8,
                217u8, 125u8, 212u8, 225u8, 138u8, 58u8, 207u8, 228u8, 119u8, 47u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    epoch: topics.1,
                    startTimestamp: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.epoch.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.epoch);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewEpochStarted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewEpochStarted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewEpochStarted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Paused(address)` and selector `0x62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258`.
```solidity
event Paused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Paused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8,
                2u8, 112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8,
                71u8, 84u8, 235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
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
                        &self.account,
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
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8,
                81u8, 66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8,
                71u8, 92u8, 58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SyndicateSequencingChainCreated(uint256,address,address)` and selector `0x49b21f1e4190db8b0a933c951ed013de222c847c15461754682daa2eab1fdbd2`.
```solidity
event SyndicateSequencingChainCreated(uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SyndicateSequencingChainCreated {
        #[allow(missing_docs)]
        pub appchainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sequencingChainAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub permissionModuleAddress: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for SyndicateSequencingChainCreated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SyndicateSequencingChainCreated(uint256,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                73u8, 178u8, 31u8, 30u8, 65u8, 144u8, 219u8, 139u8, 10u8, 147u8, 60u8,
                149u8, 30u8, 208u8, 19u8, 222u8, 34u8, 44u8, 132u8, 124u8, 21u8, 70u8,
                23u8, 84u8, 104u8, 45u8, 170u8, 46u8, 171u8, 31u8, 219u8, 210u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    appchainId: topics.1,
                    sequencingChainAddress: topics.2,
                    permissionModuleAddress: topics.3,
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
                    self.appchainId.clone(),
                    self.sequencingChainAddress.clone(),
                    self.permissionModuleAddress.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.appchainId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sequencingChainAddress,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.permissionModuleAddress,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SyndicateSequencingChainCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SyndicateSequencingChainCreated>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SyndicateSequencingChainCreated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Unpaused(address)` and selector `0x5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa`.
```solidity
event Unpaused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Unpaused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8,
                167u8, 131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8,
                78u8, 83u8, 123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
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
                        &self.account,
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
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address admin);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub admin: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.admin,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { admin: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
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
                        &self.admin,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`.
```solidity
function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    Self
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
            type Return = alloy::sol_types::private::FixedBytes<32>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: DEFAULT_ADMIN_ROLEReturn = r.into();
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
                        let r: DEFAULT_ADMIN_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `MANAGER_ROLE()` and selector `0xec87621c`.
```solidity
function MANAGER_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MANAGER_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`MANAGER_ROLE()`](MANAGER_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MANAGER_ROLEReturn {
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
            impl ::core::convert::From<MANAGER_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: MANAGER_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MANAGER_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<MANAGER_ROLEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MANAGER_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MANAGER_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MANAGER_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MANAGER_ROLE()";
            const SELECTOR: [u8; 4] = [236u8, 135u8, 98u8, 28u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: MANAGER_ROLEReturn = r.into();
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
                        let r: MANAGER_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `PERIOD_DURATION()` and selector `0x6558954f`.
```solidity
function PERIOD_DURATION() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PERIOD_DURATIONCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`PERIOD_DURATION()`](PERIOD_DURATIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PERIOD_DURATIONReturn {
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
            impl ::core::convert::From<PERIOD_DURATIONCall> for UnderlyingRustTuple<'_> {
                fn from(value: PERIOD_DURATIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PERIOD_DURATIONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<PERIOD_DURATIONReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: PERIOD_DURATIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PERIOD_DURATIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PERIOD_DURATIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PERIOD_DURATION()";
            const SELECTOR: [u8; 4] = [101u8, 88u8, 149u8, 79u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: PERIOD_DURATIONReturn = r.into();
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
                        let r: PERIOD_DURATIONReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `aggregatePeriods(uint256)` and selector `0x8e9fea39`.
```solidity
function aggregatePeriods(uint256) external view returns (uint256 totalGasUsed, uint256 totalGasCost, uint256 activeAppchains);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregatePeriodsCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`aggregatePeriods(uint256)`](aggregatePeriodsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregatePeriodsReturn {
        #[allow(missing_docs)]
        pub totalGasUsed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalGasCost: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub activeAppchains: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<aggregatePeriodsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: aggregatePeriodsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for aggregatePeriodsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<aggregatePeriodsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: aggregatePeriodsReturn) -> Self {
                    (value.totalGasUsed, value.totalGasCost, value.activeAppchains)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for aggregatePeriodsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        totalGasUsed: tuple.0,
                        totalGasCost: tuple.1,
                        activeAppchains: tuple.2,
                    }
                }
            }
        }
        impl aggregatePeriodsReturn {
            fn _tokenize(
                &self,
            ) -> <aggregatePeriodsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalGasUsed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalGasCost),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.activeAppchains),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for aggregatePeriodsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = aggregatePeriodsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "aggregatePeriods(uint256)";
            const SELECTOR: [u8; 4] = [142u8, 159u8, 234u8, 57u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                aggregatePeriodsReturn::_tokenize(ret)
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
    /**Function with signature `appchainContracts(uint256)` and selector `0x6ff6f6c0`.
```solidity
function appchainContracts(uint256) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct appchainContractsCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`appchainContracts(uint256)`](appchainContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct appchainContractsReturn {
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
            impl ::core::convert::From<appchainContractsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: appchainContractsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for appchainContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<appchainContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: appchainContractsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for appchainContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for appchainContractsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "appchainContracts(uint256)";
            const SELECTOR: [u8; 4] = [111u8, 246u8, 246u8, 192u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
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
                        let r: appchainContractsReturn = r.into();
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
                        let r: appchainContractsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `appchainList(uint256)` and selector `0xf96fc51e`.
```solidity
function appchainList(uint256) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct appchainListCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`appchainList(uint256)`](appchainListCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct appchainListReturn {
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
            impl ::core::convert::From<appchainListCall> for UnderlyingRustTuple<'_> {
                fn from(value: appchainListCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for appchainListCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<appchainListReturn> for UnderlyingRustTuple<'_> {
                fn from(value: appchainListReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for appchainListReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for appchainListCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "appchainList(uint256)";
            const SELECTOR: [u8; 4] = [249u8, 111u8, 197u8, 30u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: appchainListReturn = r.into();
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
                        let r: appchainListReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `appchainPeriods(uint256,uint256)` and selector `0xcc541821`.
```solidity
function appchainPeriods(uint256, uint256) external view returns (uint256 startTimestamp, uint256 endTimestamp, uint256 totalGasUsed, uint256 totalGasCost);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct appchainPeriodsCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`appchainPeriods(uint256,uint256)`](appchainPeriodsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct appchainPeriodsReturn {
        #[allow(missing_docs)]
        pub startTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub endTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalGasUsed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalGasCost: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<appchainPeriodsCall> for UnderlyingRustTuple<'_> {
                fn from(value: appchainPeriodsCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for appchainPeriodsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<appchainPeriodsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: appchainPeriodsReturn) -> Self {
                    (
                        value.startTimestamp,
                        value.endTimestamp,
                        value.totalGasUsed,
                        value.totalGasCost,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for appchainPeriodsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        startTimestamp: tuple.0,
                        endTimestamp: tuple.1,
                        totalGasUsed: tuple.2,
                        totalGasCost: tuple.3,
                    }
                }
            }
        }
        impl appchainPeriodsReturn {
            fn _tokenize(
                &self,
            ) -> <appchainPeriodsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.endTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalGasUsed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalGasCost),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for appchainPeriodsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = appchainPeriodsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "appchainPeriods(uint256,uint256)";
            const SELECTOR: [u8; 4] = [204u8, 84u8, 24u8, 33u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                appchainPeriodsReturn::_tokenize(ret)
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
    /**Function with signature `computeSequencingChainAddress(bytes32,uint256)` and selector `0x1f43fc8b`.
```solidity
function computeSequencingChainAddress(bytes32 salt, uint256 chainId) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct computeSequencingChainAddressCall {
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`computeSequencingChainAddress(bytes32,uint256)`](computeSequencingChainAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct computeSequencingChainAddressReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<computeSequencingChainAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: computeSequencingChainAddressCall) -> Self {
                    (value.salt, value.chainId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for computeSequencingChainAddressCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        salt: tuple.0,
                        chainId: tuple.1,
                    }
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
            impl ::core::convert::From<computeSequencingChainAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: computeSequencingChainAddressReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for computeSequencingChainAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for computeSequencingChainAddressCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "computeSequencingChainAddress(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [31u8, 67u8, 252u8, 139u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                )
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
                        let r: computeSequencingChainAddressReturn = r.into();
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
                        let r: computeSequencingChainAddressReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `createSyndicateSequencingChain(uint256,address,address,bytes32)` and selector `0xd9852abf`.
```solidity
function createSyndicateSequencingChain(uint256 appchainId, address admin, address permissionModule, bytes32 salt) external returns (address sequencingChain, uint256 actualChainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createSyndicateSequencingChainCall {
        #[allow(missing_docs)]
        pub appchainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub admin: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub permissionModule: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`createSyndicateSequencingChain(uint256,address,address,bytes32)`](createSyndicateSequencingChainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createSyndicateSequencingChainReturn {
        #[allow(missing_docs)]
        pub sequencingChain: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub actualChainId: alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<createSyndicateSequencingChainCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createSyndicateSequencingChainCall) -> Self {
                    (value.appchainId, value.admin, value.permissionModule, value.salt)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createSyndicateSequencingChainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        appchainId: tuple.0,
                        admin: tuple.1,
                        permissionModule: tuple.2,
                        salt: tuple.3,
                    }
                }
            }
        }
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
            impl ::core::convert::From<createSyndicateSequencingChainReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createSyndicateSequencingChainReturn) -> Self {
                    (value.sequencingChain, value.actualChainId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createSyndicateSequencingChainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sequencingChain: tuple.0,
                        actualChainId: tuple.1,
                    }
                }
            }
        }
        impl createSyndicateSequencingChainReturn {
            fn _tokenize(
                &self,
            ) -> <createSyndicateSequencingChainCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sequencingChain,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actualChainId),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createSyndicateSequencingChainCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createSyndicateSequencingChainReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createSyndicateSequencingChain(uint256,address,address,bytes32)";
            const SELECTOR: [u8; 4] = [217u8, 133u8, 42u8, 191u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.appchainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.admin,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.permissionModule,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                createSyndicateSequencingChainReturn::_tokenize(ret)
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
    /**Function with signature `cumulativeGasFeesByAppchain(uint256)` and selector `0x46545488`.
```solidity
function cumulativeGasFeesByAppchain(uint256) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeGasFeesByAppchainCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`cumulativeGasFeesByAppchain(uint256)`](cumulativeGasFeesByAppchainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeGasFeesByAppchainReturn {
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
            impl ::core::convert::From<cumulativeGasFeesByAppchainCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeGasFeesByAppchainCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeGasFeesByAppchainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<cumulativeGasFeesByAppchainReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeGasFeesByAppchainReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeGasFeesByAppchainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cumulativeGasFeesByAppchainCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cumulativeGasFeesByAppchain(uint256)";
            const SELECTOR: [u8; 4] = [70u8, 84u8, 84u8, 136u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: cumulativeGasFeesByAppchainReturn = r.into();
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
                        let r: cumulativeGasFeesByAppchainReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `currentEpoch()` and selector `0x76671808`.
```solidity
function currentEpoch() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentEpochCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`currentEpoch()`](currentEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentEpochReturn {
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
            impl ::core::convert::From<currentEpochCall> for UnderlyingRustTuple<'_> {
                fn from(value: currentEpochCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<currentEpochReturn> for UnderlyingRustTuple<'_> {
                fn from(value: currentEpochReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentEpochCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentEpoch()";
            const SELECTOR: [u8; 4] = [118u8, 103u8, 24u8, 8u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: currentEpochReturn = r.into();
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
                        let r: currentEpochReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `currentEpochStart()` and selector `0x61a8c8c4`.
```solidity
function currentEpochStart() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentEpochStartCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`currentEpochStart()`](currentEpochStartCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentEpochStartReturn {
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
            impl ::core::convert::From<currentEpochStartCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentEpochStartCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentEpochStartCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<currentEpochStartReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentEpochStartReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentEpochStartReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentEpochStartCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentEpochStart()";
            const SELECTOR: [u8; 4] = [97u8, 168u8, 200u8, 196u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: currentEpochStartReturn = r.into();
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
                        let r: currentEpochStartReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `disableGasTracking()` and selector `0x5467cb48`.
```solidity
function disableGasTracking() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableGasTrackingCall;
    ///Container type for the return parameters of the [`disableGasTracking()`](disableGasTrackingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableGasTrackingReturn {}
    #[allow(
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
            impl ::core::convert::From<disableGasTrackingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: disableGasTrackingCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disableGasTrackingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<disableGasTrackingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: disableGasTrackingReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disableGasTrackingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl disableGasTrackingReturn {
            fn _tokenize(
                &self,
            ) -> <disableGasTrackingCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disableGasTrackingCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = disableGasTrackingReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disableGasTracking()";
            const SELECTOR: [u8; 4] = [84u8, 103u8, 203u8, 72u8];
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
                disableGasTrackingReturn::_tokenize(ret)
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
    /**Function with signature `enableGasTracking()` and selector `0xde1f453e`.
```solidity
function enableGasTracking() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableGasTrackingCall;
    ///Container type for the return parameters of the [`enableGasTracking()`](enableGasTrackingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableGasTrackingReturn {}
    #[allow(
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
            impl ::core::convert::From<enableGasTrackingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: enableGasTrackingCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for enableGasTrackingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<enableGasTrackingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: enableGasTrackingReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for enableGasTrackingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl enableGasTrackingReturn {
            fn _tokenize(
                &self,
            ) -> <enableGasTrackingCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for enableGasTrackingCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = enableGasTrackingReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "enableGasTracking()";
            const SELECTOR: [u8; 4] = [222u8, 31u8, 69u8, 62u8];
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
                enableGasTrackingReturn::_tokenize(ret)
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
    /**Function with signature `gasTrackingEnabled()` and selector `0x84fab62b`.
```solidity
function gasTrackingEnabled() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasTrackingEnabledCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`gasTrackingEnabled()`](gasTrackingEnabledCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasTrackingEnabledReturn {
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
            impl ::core::convert::From<gasTrackingEnabledCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: gasTrackingEnabledCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for gasTrackingEnabledCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<gasTrackingEnabledReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: gasTrackingEnabledReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for gasTrackingEnabledReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for gasTrackingEnabledCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "gasTrackingEnabled()";
            const SELECTOR: [u8; 4] = [132u8, 250u8, 182u8, 43u8];
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
                        let r: gasTrackingEnabledReturn = r.into();
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
                        let r: gasTrackingEnabledReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `gasTrackingInitialized()` and selector `0x3b6ab2a9`.
```solidity
function gasTrackingInitialized() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasTrackingInitializedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`gasTrackingInitialized()`](gasTrackingInitializedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasTrackingInitializedReturn {
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
            impl ::core::convert::From<gasTrackingInitializedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: gasTrackingInitializedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for gasTrackingInitializedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<gasTrackingInitializedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: gasTrackingInitializedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for gasTrackingInitializedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for gasTrackingInitializedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "gasTrackingInitialized()";
            const SELECTOR: [u8; 4] = [59u8, 106u8, 178u8, 169u8];
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
                        let r: gasTrackingInitializedReturn = r.into();
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
                        let r: gasTrackingInitializedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getAppchainContract(uint256)` and selector `0x78fc30cc`.
```solidity
function getAppchainContract(uint256 chainId) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAppchainContractCall {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getAppchainContract(uint256)`](getAppchainContractCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAppchainContractReturn {
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
            impl ::core::convert::From<getAppchainContractCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAppchainContractCall) -> Self {
                    (value.chainId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAppchainContractCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { chainId: tuple.0 }
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
            impl ::core::convert::From<getAppchainContractReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAppchainContractReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAppchainContractReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAppchainContractCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAppchainContract(uint256)";
            const SELECTOR: [u8; 4] = [120u8, 252u8, 48u8, 204u8];
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
                )
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
                        let r: getAppchainContractReturn = r.into();
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
                        let r: getAppchainContractReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getAppchainCumulativeFees(uint256)` and selector `0xba26992a`.
```solidity
function getAppchainCumulativeFees(uint256 appchainId) external view returns (uint256 totalFees);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAppchainCumulativeFeesCall {
        #[allow(missing_docs)]
        pub appchainId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getAppchainCumulativeFees(uint256)`](getAppchainCumulativeFeesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAppchainCumulativeFeesReturn {
        #[allow(missing_docs)]
        pub totalFees: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getAppchainCumulativeFeesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAppchainCumulativeFeesCall) -> Self {
                    (value.appchainId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAppchainCumulativeFeesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { appchainId: tuple.0 }
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
            impl ::core::convert::From<getAppchainCumulativeFeesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAppchainCumulativeFeesReturn) -> Self {
                    (value.totalFees,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAppchainCumulativeFeesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { totalFees: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAppchainCumulativeFeesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAppchainCumulativeFees(uint256)";
            const SELECTOR: [u8; 4] = [186u8, 38u8, 153u8, 42u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.appchainId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getAppchainCumulativeFeesReturn = r.into();
                        r.totalFees
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
                        let r: getAppchainCumulativeFeesReturn = r.into();
                        r.totalFees
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getAppchainEpochData(uint256,uint256)` and selector `0x9c984b6d`.
```solidity
function getAppchainEpochData(uint256 epoch, uint256 appchainId) external view returns (GasTracker.AppchainGasPeriod memory period);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAppchainEpochDataCall {
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub appchainId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getAppchainEpochData(uint256,uint256)`](getAppchainEpochDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAppchainEpochDataReturn {
        #[allow(missing_docs)]
        pub period: <GasTracker::AppchainGasPeriod as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAppchainEpochDataCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAppchainEpochDataCall) -> Self {
                    (value.epoch, value.appchainId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAppchainEpochDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        epoch: tuple.0,
                        appchainId: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (GasTracker::AppchainGasPeriod,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GasTracker::AppchainGasPeriod as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAppchainEpochDataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAppchainEpochDataReturn) -> Self {
                    (value.period,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAppchainEpochDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { period: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAppchainEpochDataCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <GasTracker::AppchainGasPeriod as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (GasTracker::AppchainGasPeriod,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAppchainEpochData(uint256,uint256)";
            const SELECTOR: [u8; 4] = [156u8, 152u8, 75u8, 109u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.appchainId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <GasTracker::AppchainGasPeriod as alloy_sol_types::SolType>::tokenize(
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
                        let r: getAppchainEpochDataReturn = r.into();
                        r.period
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
                        let r: getAppchainEpochDataReturn = r.into();
                        r.period
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getBytecode(uint256)` and selector `0x2cd799bd`.
```solidity
function getBytecode(uint256 chainId) external pure returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBytecodeCall {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getBytecode(uint256)`](getBytecodeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBytecodeReturn {
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
            impl ::core::convert::From<getBytecodeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBytecodeCall) -> Self {
                    (value.chainId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBytecodeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { chainId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getBytecodeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBytecodeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBytecodeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBytecodeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Bytes;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBytecode(uint256)";
            const SELECTOR: [u8; 4] = [44u8, 215u8, 153u8, 189u8];
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
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
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
                        let r: getBytecodeReturn = r.into();
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
                        let r: getBytecodeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCurrentAppchainData(uint256)` and selector `0xc52c210c`.
```solidity
function getCurrentAppchainData(uint256 appchainId) external view returns (GasTracker.AppchainGasPeriod memory period);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentAppchainDataCall {
        #[allow(missing_docs)]
        pub appchainId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCurrentAppchainData(uint256)`](getCurrentAppchainDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentAppchainDataReturn {
        #[allow(missing_docs)]
        pub period: <GasTracker::AppchainGasPeriod as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCurrentAppchainDataCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentAppchainDataCall) -> Self {
                    (value.appchainId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentAppchainDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { appchainId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (GasTracker::AppchainGasPeriod,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GasTracker::AppchainGasPeriod as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCurrentAppchainDataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentAppchainDataReturn) -> Self {
                    (value.period,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentAppchainDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { period: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentAppchainDataCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <GasTracker::AppchainGasPeriod as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (GasTracker::AppchainGasPeriod,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentAppchainData(uint256)";
            const SELECTOR: [u8; 4] = [197u8, 44u8, 33u8, 12u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.appchainId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <GasTracker::AppchainGasPeriod as alloy_sol_types::SolType>::tokenize(
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
                        let r: getCurrentAppchainDataReturn = r.into();
                        r.period
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
                        let r: getCurrentAppchainDataReturn = r.into();
                        r.period
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCurrentEpochAggregate()` and selector `0x365f16e3`.
```solidity
function getCurrentEpochAggregate() external view returns (GasTracker.AggregateGasPeriod memory aggregate);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentEpochAggregateCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCurrentEpochAggregate()`](getCurrentEpochAggregateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentEpochAggregateReturn {
        #[allow(missing_docs)]
        pub aggregate: <GasTracker::AggregateGasPeriod as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCurrentEpochAggregateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentEpochAggregateCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentEpochAggregateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (GasTracker::AggregateGasPeriod,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GasTracker::AggregateGasPeriod as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCurrentEpochAggregateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentEpochAggregateReturn) -> Self {
                    (value.aggregate,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentEpochAggregateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { aggregate: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentEpochAggregateCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <GasTracker::AggregateGasPeriod as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (GasTracker::AggregateGasPeriod,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentEpochAggregate()";
            const SELECTOR: [u8; 4] = [54u8, 95u8, 22u8, 227u8];
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
                    <GasTracker::AggregateGasPeriod as alloy_sol_types::SolType>::tokenize(
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
                        let r: getCurrentEpochAggregateReturn = r.into();
                        r.aggregate
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
                        let r: getCurrentEpochAggregateReturn = r.into();
                        r.aggregate
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCurrentEpochTimeRemaining()` and selector `0x1351f9c3`.
```solidity
function getCurrentEpochTimeRemaining() external view returns (uint256 timeRemaining);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentEpochTimeRemainingCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCurrentEpochTimeRemaining()`](getCurrentEpochTimeRemainingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentEpochTimeRemainingReturn {
        #[allow(missing_docs)]
        pub timeRemaining: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getCurrentEpochTimeRemainingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentEpochTimeRemainingCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentEpochTimeRemainingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<getCurrentEpochTimeRemainingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentEpochTimeRemainingReturn) -> Self {
                    (value.timeRemaining,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentEpochTimeRemainingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { timeRemaining: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentEpochTimeRemainingCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentEpochTimeRemaining()";
            const SELECTOR: [u8; 4] = [19u8, 81u8, 249u8, 195u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getCurrentEpochTimeRemainingReturn = r.into();
                        r.timeRemaining
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
                        let r: getCurrentEpochTimeRemainingReturn = r.into();
                        r.timeRemaining
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getEpochAggregate(uint256)` and selector `0xc80dc41e`.
```solidity
function getEpochAggregate(uint256 epoch) external view returns (GasTracker.AggregateGasPeriod memory aggregate);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEpochAggregateCall {
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getEpochAggregate(uint256)`](getEpochAggregateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEpochAggregateReturn {
        #[allow(missing_docs)]
        pub aggregate: <GasTracker::AggregateGasPeriod as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getEpochAggregateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getEpochAggregateCall) -> Self {
                    (value.epoch,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getEpochAggregateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { epoch: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (GasTracker::AggregateGasPeriod,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GasTracker::AggregateGasPeriod as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getEpochAggregateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getEpochAggregateReturn) -> Self {
                    (value.aggregate,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getEpochAggregateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { aggregate: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getEpochAggregateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <GasTracker::AggregateGasPeriod as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (GasTracker::AggregateGasPeriod,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getEpochAggregate(uint256)";
            const SELECTOR: [u8; 4] = [200u8, 13u8, 196u8, 30u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <GasTracker::AggregateGasPeriod as alloy_sol_types::SolType>::tokenize(
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
                        let r: getEpochAggregateReturn = r.into();
                        r.aggregate
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
                        let r: getEpochAggregateReturn = r.into();
                        r.aggregate
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKnownAppchains()` and selector `0x04cf85de`.
```solidity
function getKnownAppchains() external view returns (uint256[] memory appchains);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKnownAppchainsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKnownAppchains()`](getKnownAppchainsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKnownAppchainsReturn {
        #[allow(missing_docs)]
        pub appchains: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<getKnownAppchainsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKnownAppchainsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKnownAppchainsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getKnownAppchainsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKnownAppchainsReturn) -> Self {
                    (value.appchains,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKnownAppchainsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { appchains: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKnownAppchainsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKnownAppchains()";
            const SELECTOR: [u8; 4] = [4u8, 207u8, 133u8, 222u8];
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
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getKnownAppchainsReturn = r.into();
                        r.appchains
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
                        let r: getKnownAppchainsReturn = r.into();
                        r.appchains
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getNextChainId()` and selector `0x8de6e28c`.
```solidity
function getNextChainId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNextChainIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getNextChainId()`](getNextChainIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNextChainIdReturn {
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
            impl ::core::convert::From<getNextChainIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getNextChainIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getNextChainIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<getNextChainIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getNextChainIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getNextChainIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getNextChainIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getNextChainId()";
            const SELECTOR: [u8; 4] = [141u8, 230u8, 226u8, 140u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getNextChainIdReturn = r.into();
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
                        let r: getNextChainIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = alloy::sol_types::private::FixedBytes<32>;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRoleAdminReturn = r.into();
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
                        let r: getRoleAdminReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getTotalCumulativeFees()` and selector `0x93c5f0e1`.
```solidity
function getTotalCumulativeFees() external view returns (uint256 totalFees);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalCumulativeFeesCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getTotalCumulativeFees()`](getTotalCumulativeFeesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalCumulativeFeesReturn {
        #[allow(missing_docs)]
        pub totalFees: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getTotalCumulativeFeesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalCumulativeFeesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalCumulativeFeesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<getTotalCumulativeFeesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalCumulativeFeesReturn) -> Self {
                    (value.totalFees,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalCumulativeFeesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { totalFees: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalCumulativeFeesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalCumulativeFees()";
            const SELECTOR: [u8; 4] = [147u8, 197u8, 240u8, 225u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getTotalCumulativeFeesReturn = r.into();
                        r.totalFees
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
                        let r: getTotalCumulativeFeesReturn = r.into();
                        r.totalFees
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        impl grantRoleReturn {
            fn _tokenize(
                &self,
            ) -> <grantRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                grantRoleReturn::_tokenize(ret)
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = bool;
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
                        let r: hasRoleReturn = r.into();
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
                        let r: hasRoleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isChainIdUsed(uint256)` and selector `0x7232c133`.
```solidity
function isChainIdUsed(uint256 chainId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isChainIdUsedCall {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isChainIdUsed(uint256)`](isChainIdUsedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isChainIdUsedReturn {
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
            impl ::core::convert::From<isChainIdUsedCall> for UnderlyingRustTuple<'_> {
                fn from(value: isChainIdUsedCall) -> Self {
                    (value.chainId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isChainIdUsedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { chainId: tuple.0 }
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
            impl ::core::convert::From<isChainIdUsedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isChainIdUsedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isChainIdUsedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isChainIdUsedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isChainIdUsed(uint256)";
            const SELECTOR: [u8; 4] = [114u8, 50u8, 193u8, 51u8];
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
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isChainIdUsedReturn = r.into();
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
                        let r: isChainIdUsedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `knownAppchains(uint256)` and selector `0xdcf93475`.
```solidity
function knownAppchains(uint256) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct knownAppchainsCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`knownAppchains(uint256)`](knownAppchainsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct knownAppchainsReturn {
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
            impl ::core::convert::From<knownAppchainsCall> for UnderlyingRustTuple<'_> {
                fn from(value: knownAppchainsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for knownAppchainsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<knownAppchainsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: knownAppchainsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for knownAppchainsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for knownAppchainsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "knownAppchains(uint256)";
            const SELECTOR: [u8; 4] = [220u8, 249u8, 52u8, 117u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
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
                        let r: knownAppchainsReturn = r.into();
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
                        let r: knownAppchainsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `markChainIdAsUsed(uint256)` and selector `0x3bfec7a2`.
```solidity
function markChainIdAsUsed(uint256 chainId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct markChainIdAsUsedCall {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`markChainIdAsUsed(uint256)`](markChainIdAsUsedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct markChainIdAsUsedReturn {}
    #[allow(
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
            impl ::core::convert::From<markChainIdAsUsedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: markChainIdAsUsedCall) -> Self {
                    (value.chainId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for markChainIdAsUsedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { chainId: tuple.0 }
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
            impl ::core::convert::From<markChainIdAsUsedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: markChainIdAsUsedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for markChainIdAsUsedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl markChainIdAsUsedReturn {
            fn _tokenize(
                &self,
            ) -> <markChainIdAsUsedCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for markChainIdAsUsedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = markChainIdAsUsedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "markChainIdAsUsed(uint256)";
            const SELECTOR: [u8; 4] = [59u8, 254u8, 199u8, 162u8];
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
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                markChainIdAsUsedReturn::_tokenize(ret)
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
    /**Function with signature `namespacePrefix()` and selector `0x297f4c64`.
```solidity
function namespacePrefix() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct namespacePrefixCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`namespacePrefix()`](namespacePrefixCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct namespacePrefixReturn {
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
            impl ::core::convert::From<namespacePrefixCall> for UnderlyingRustTuple<'_> {
                fn from(value: namespacePrefixCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for namespacePrefixCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<namespacePrefixReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: namespacePrefixReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for namespacePrefixReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for namespacePrefixCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "namespacePrefix()";
            const SELECTOR: [u8; 4] = [41u8, 127u8, 76u8, 100u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: namespacePrefixReturn = r.into();
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
                        let r: namespacePrefixReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `nextAutoChainId()` and selector `0x999d71d4`.
```solidity
function nextAutoChainId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextAutoChainIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`nextAutoChainId()`](nextAutoChainIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextAutoChainIdReturn {
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
            impl ::core::convert::From<nextAutoChainIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: nextAutoChainIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextAutoChainIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<nextAutoChainIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: nextAutoChainIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nextAutoChainIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nextAutoChainIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nextAutoChainId()";
            const SELECTOR: [u8; 4] = [153u8, 157u8, 113u8, 212u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: nextAutoChainIdReturn = r.into();
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
                        let r: nextAutoChainIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pause()` and selector `0x8456cb59`.
```solidity
function pause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall;
    ///Container type for the return parameters of the [`pause()`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
    #[allow(
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
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl pauseReturn {
            fn _tokenize(
                &self,
            ) -> <pauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause()";
            const SELECTOR: [u8; 4] = [132u8, 86u8, 203u8, 89u8];
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
                pauseReturn::_tokenize(ret)
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
    /**Function with signature `paused()` and selector `0x5c975abb`.
```solidity
function paused() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`paused()`](pausedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedReturn {
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
            impl ::core::convert::From<pausedCall> for UnderlyingRustTuple<'_> {
                fn from(value: pausedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<pausedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pausedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pausedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
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
                        let r: pausedReturn = r.into();
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
                        let r: pausedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
        impl renounceRoleReturn {
            fn _tokenize(
                &self,
            ) -> <renounceRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceRoleReturn::_tokenize(ret)
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
        impl revokeRoleReturn {
            fn _tokenize(
                &self,
            ) -> <revokeRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                revokeRoleReturn::_tokenize(ret)
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = bool;
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
                        let r: supportsInterfaceReturn = r.into();
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
                        let r: supportsInterfaceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `totalCumulativeGasFees()` and selector `0xa4ccf948`.
```solidity
function totalCumulativeGasFees() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalCumulativeGasFeesCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`totalCumulativeGasFees()`](totalCumulativeGasFeesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalCumulativeGasFeesReturn {
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
            impl ::core::convert::From<totalCumulativeGasFeesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalCumulativeGasFeesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalCumulativeGasFeesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<totalCumulativeGasFeesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalCumulativeGasFeesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalCumulativeGasFeesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalCumulativeGasFeesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "totalCumulativeGasFees()";
            const SELECTOR: [u8; 4] = [164u8, 204u8, 249u8, 72u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: totalCumulativeGasFeesReturn = r.into();
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
                        let r: totalCumulativeGasFeesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `trackAppchainGas(uint256,uint256)` and selector `0xe3e79370`.
```solidity
function trackAppchainGas(uint256 appchainId, uint256 gasUsed) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trackAppchainGasCall {
        #[allow(missing_docs)]
        pub appchainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasUsed: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`trackAppchainGas(uint256,uint256)`](trackAppchainGasCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trackAppchainGasReturn {}
    #[allow(
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
            impl ::core::convert::From<trackAppchainGasCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: trackAppchainGasCall) -> Self {
                    (value.appchainId, value.gasUsed)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for trackAppchainGasCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        appchainId: tuple.0,
                        gasUsed: tuple.1,
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
            impl ::core::convert::From<trackAppchainGasReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: trackAppchainGasReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for trackAppchainGasReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl trackAppchainGasReturn {
            fn _tokenize(
                &self,
            ) -> <trackAppchainGasCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for trackAppchainGasCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = trackAppchainGasReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "trackAppchainGas(uint256,uint256)";
            const SELECTOR: [u8; 4] = [227u8, 231u8, 147u8, 112u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.appchainId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasUsed),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                trackAppchainGasReturn::_tokenize(ret)
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
    /**Function with signature `trackGas(uint256,uint256)` and selector `0x1bd5002c`.
```solidity
function trackGas(uint256 appchainId, uint256 gasUsed) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trackGasCall {
        #[allow(missing_docs)]
        pub appchainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasUsed: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`trackGas(uint256,uint256)`](trackGasCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trackGasReturn {}
    #[allow(
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
            impl ::core::convert::From<trackGasCall> for UnderlyingRustTuple<'_> {
                fn from(value: trackGasCall) -> Self {
                    (value.appchainId, value.gasUsed)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for trackGasCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        appchainId: tuple.0,
                        gasUsed: tuple.1,
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
            impl ::core::convert::From<trackGasReturn> for UnderlyingRustTuple<'_> {
                fn from(value: trackGasReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for trackGasReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl trackGasReturn {
            fn _tokenize(
                &self,
            ) -> <trackGasCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for trackGasCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = trackGasReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "trackGas(uint256,uint256)";
            const SELECTOR: [u8; 4] = [27u8, 213u8, 0u8, 44u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.appchainId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasUsed),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                trackGasReturn::_tokenize(ret)
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
    /**Function with signature `unpause()` and selector `0x3f4ba83a`.
```solidity
function unpause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall;
    ///Container type for the return parameters of the [`unpause()`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
    #[allow(
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
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl unpauseReturn {
            fn _tokenize(
                &self,
            ) -> <unpauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause()";
            const SELECTOR: [u8; 4] = [63u8, 75u8, 168u8, 58u8];
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
                unpauseReturn::_tokenize(ret)
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
    /**Function with signature `updateNamespaceConfig(uint256)` and selector `0x8c39aaa4`.
```solidity
function updateNamespaceConfig(uint256 newPrefix) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateNamespaceConfigCall {
        #[allow(missing_docs)]
        pub newPrefix: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateNamespaceConfig(uint256)`](updateNamespaceConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateNamespaceConfigReturn {}
    #[allow(
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
            impl ::core::convert::From<updateNamespaceConfigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateNamespaceConfigCall) -> Self {
                    (value.newPrefix,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateNamespaceConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPrefix: tuple.0 }
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
            impl ::core::convert::From<updateNamespaceConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateNamespaceConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateNamespaceConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateNamespaceConfigReturn {
            fn _tokenize(
                &self,
            ) -> <updateNamespaceConfigCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateNamespaceConfigCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateNamespaceConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateNamespaceConfig(uint256)";
            const SELECTOR: [u8; 4] = [140u8, 57u8, 170u8, 164u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.newPrefix),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateNamespaceConfigReturn::_tokenize(ret)
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
    /**Function with signature `usedChainIds(uint256)` and selector `0x0d5869ee`.
```solidity
function usedChainIds(uint256) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct usedChainIdsCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`usedChainIds(uint256)`](usedChainIdsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct usedChainIdsReturn {
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
            impl ::core::convert::From<usedChainIdsCall> for UnderlyingRustTuple<'_> {
                fn from(value: usedChainIdsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for usedChainIdsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<usedChainIdsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: usedChainIdsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for usedChainIdsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for usedChainIdsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "usedChainIds(uint256)";
            const SELECTOR: [u8; 4] = [13u8, 88u8, 105u8, 238u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
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
                        let r: usedChainIdsReturn = r.into();
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
                        let r: usedChainIdsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `usedNamespaces(uint256)` and selector `0xb963ebf3`.
```solidity
function usedNamespaces(uint256) external view returns (NamespaceState);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct usedNamespacesCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`usedNamespaces(uint256)`](usedNamespacesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct usedNamespacesReturn {
        #[allow(missing_docs)]
        pub _0: <NamespaceState as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<usedNamespacesCall> for UnderlyingRustTuple<'_> {
                fn from(value: usedNamespacesCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for usedNamespacesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (NamespaceState,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <NamespaceState as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<usedNamespacesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: usedNamespacesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for usedNamespacesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for usedNamespacesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <NamespaceState as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (NamespaceState,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "usedNamespaces(uint256)";
            const SELECTOR: [u8; 4] = [185u8, 99u8, 235u8, 243u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<NamespaceState as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: usedNamespacesReturn = r.into();
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
                        let r: usedNamespacesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`SyndicateFactory`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum SyndicateFactoryCalls {
        #[allow(missing_docs)]
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        #[allow(missing_docs)]
        MANAGER_ROLE(MANAGER_ROLECall),
        #[allow(missing_docs)]
        PERIOD_DURATION(PERIOD_DURATIONCall),
        #[allow(missing_docs)]
        aggregatePeriods(aggregatePeriodsCall),
        #[allow(missing_docs)]
        appchainContracts(appchainContractsCall),
        #[allow(missing_docs)]
        appchainList(appchainListCall),
        #[allow(missing_docs)]
        appchainPeriods(appchainPeriodsCall),
        #[allow(missing_docs)]
        computeSequencingChainAddress(computeSequencingChainAddressCall),
        #[allow(missing_docs)]
        createSyndicateSequencingChain(createSyndicateSequencingChainCall),
        #[allow(missing_docs)]
        cumulativeGasFeesByAppchain(cumulativeGasFeesByAppchainCall),
        #[allow(missing_docs)]
        currentEpoch(currentEpochCall),
        #[allow(missing_docs)]
        currentEpochStart(currentEpochStartCall),
        #[allow(missing_docs)]
        disableGasTracking(disableGasTrackingCall),
        #[allow(missing_docs)]
        enableGasTracking(enableGasTrackingCall),
        #[allow(missing_docs)]
        gasTrackingEnabled(gasTrackingEnabledCall),
        #[allow(missing_docs)]
        gasTrackingInitialized(gasTrackingInitializedCall),
        #[allow(missing_docs)]
        getAppchainContract(getAppchainContractCall),
        #[allow(missing_docs)]
        getAppchainCumulativeFees(getAppchainCumulativeFeesCall),
        #[allow(missing_docs)]
        getAppchainEpochData(getAppchainEpochDataCall),
        #[allow(missing_docs)]
        getBytecode(getBytecodeCall),
        #[allow(missing_docs)]
        getCurrentAppchainData(getCurrentAppchainDataCall),
        #[allow(missing_docs)]
        getCurrentEpochAggregate(getCurrentEpochAggregateCall),
        #[allow(missing_docs)]
        getCurrentEpochTimeRemaining(getCurrentEpochTimeRemainingCall),
        #[allow(missing_docs)]
        getEpochAggregate(getEpochAggregateCall),
        #[allow(missing_docs)]
        getKnownAppchains(getKnownAppchainsCall),
        #[allow(missing_docs)]
        getNextChainId(getNextChainIdCall),
        #[allow(missing_docs)]
        getRoleAdmin(getRoleAdminCall),
        #[allow(missing_docs)]
        getTotalCumulativeFees(getTotalCumulativeFeesCall),
        #[allow(missing_docs)]
        grantRole(grantRoleCall),
        #[allow(missing_docs)]
        hasRole(hasRoleCall),
        #[allow(missing_docs)]
        isChainIdUsed(isChainIdUsedCall),
        #[allow(missing_docs)]
        knownAppchains(knownAppchainsCall),
        #[allow(missing_docs)]
        markChainIdAsUsed(markChainIdAsUsedCall),
        #[allow(missing_docs)]
        namespacePrefix(namespacePrefixCall),
        #[allow(missing_docs)]
        nextAutoChainId(nextAutoChainIdCall),
        #[allow(missing_docs)]
        pause(pauseCall),
        #[allow(missing_docs)]
        paused(pausedCall),
        #[allow(missing_docs)]
        renounceRole(renounceRoleCall),
        #[allow(missing_docs)]
        revokeRole(revokeRoleCall),
        #[allow(missing_docs)]
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        totalCumulativeGasFees(totalCumulativeGasFeesCall),
        #[allow(missing_docs)]
        trackAppchainGas(trackAppchainGasCall),
        #[allow(missing_docs)]
        trackGas(trackGasCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
        #[allow(missing_docs)]
        updateNamespaceConfig(updateNamespaceConfigCall),
        #[allow(missing_docs)]
        usedChainIds(usedChainIdsCall),
        #[allow(missing_docs)]
        usedNamespaces(usedNamespacesCall),
    }
    #[automatically_derived]
    impl SyndicateFactoryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 255u8, 201u8, 167u8],
            [4u8, 207u8, 133u8, 222u8],
            [13u8, 88u8, 105u8, 238u8],
            [19u8, 81u8, 249u8, 195u8],
            [27u8, 213u8, 0u8, 44u8],
            [31u8, 67u8, 252u8, 139u8],
            [36u8, 138u8, 156u8, 163u8],
            [41u8, 127u8, 76u8, 100u8],
            [44u8, 215u8, 153u8, 189u8],
            [47u8, 47u8, 241u8, 93u8],
            [54u8, 86u8, 138u8, 190u8],
            [54u8, 95u8, 22u8, 227u8],
            [59u8, 106u8, 178u8, 169u8],
            [59u8, 254u8, 199u8, 162u8],
            [63u8, 75u8, 168u8, 58u8],
            [70u8, 84u8, 84u8, 136u8],
            [84u8, 103u8, 203u8, 72u8],
            [92u8, 151u8, 90u8, 187u8],
            [97u8, 168u8, 200u8, 196u8],
            [101u8, 88u8, 149u8, 79u8],
            [111u8, 246u8, 246u8, 192u8],
            [114u8, 50u8, 193u8, 51u8],
            [118u8, 103u8, 24u8, 8u8],
            [120u8, 252u8, 48u8, 204u8],
            [132u8, 86u8, 203u8, 89u8],
            [132u8, 250u8, 182u8, 43u8],
            [140u8, 57u8, 170u8, 164u8],
            [141u8, 230u8, 226u8, 140u8],
            [142u8, 159u8, 234u8, 57u8],
            [145u8, 209u8, 72u8, 84u8],
            [147u8, 197u8, 240u8, 225u8],
            [153u8, 157u8, 113u8, 212u8],
            [156u8, 152u8, 75u8, 109u8],
            [162u8, 23u8, 253u8, 223u8],
            [164u8, 204u8, 249u8, 72u8],
            [185u8, 99u8, 235u8, 243u8],
            [186u8, 38u8, 153u8, 42u8],
            [197u8, 44u8, 33u8, 12u8],
            [200u8, 13u8, 196u8, 30u8],
            [204u8, 84u8, 24u8, 33u8],
            [213u8, 71u8, 116u8, 31u8],
            [217u8, 133u8, 42u8, 191u8],
            [220u8, 249u8, 52u8, 117u8],
            [222u8, 31u8, 69u8, 62u8],
            [227u8, 231u8, 147u8, 112u8],
            [236u8, 135u8, 98u8, 28u8],
            [249u8, 111u8, 197u8, 30u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SyndicateFactoryCalls {
        const NAME: &'static str = "SyndicateFactoryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 47usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MANAGER_ROLE(_) => {
                    <MANAGER_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PERIOD_DURATION(_) => {
                    <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::aggregatePeriods(_) => {
                    <aggregatePeriodsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::appchainContracts(_) => {
                    <appchainContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::appchainList(_) => {
                    <appchainListCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::appchainPeriods(_) => {
                    <appchainPeriodsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::computeSequencingChainAddress(_) => {
                    <computeSequencingChainAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createSyndicateSequencingChain(_) => {
                    <createSyndicateSequencingChainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cumulativeGasFeesByAppchain(_) => {
                    <cumulativeGasFeesByAppchainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentEpoch(_) => {
                    <currentEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentEpochStart(_) => {
                    <currentEpochStartCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disableGasTracking(_) => {
                    <disableGasTrackingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::enableGasTracking(_) => {
                    <enableGasTrackingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::gasTrackingEnabled(_) => {
                    <gasTrackingEnabledCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::gasTrackingInitialized(_) => {
                    <gasTrackingInitializedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAppchainContract(_) => {
                    <getAppchainContractCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAppchainCumulativeFees(_) => {
                    <getAppchainCumulativeFeesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAppchainEpochData(_) => {
                    <getAppchainEpochDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBytecode(_) => {
                    <getBytecodeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentAppchainData(_) => {
                    <getCurrentAppchainDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentEpochAggregate(_) => {
                    <getCurrentEpochAggregateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentEpochTimeRemaining(_) => {
                    <getCurrentEpochTimeRemainingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getEpochAggregate(_) => {
                    <getEpochAggregateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKnownAppchains(_) => {
                    <getKnownAppchainsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getNextChainId(_) => {
                    <getNextChainIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleAdmin(_) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalCumulativeFees(_) => {
                    <getTotalCumulativeFeesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::grantRole(_) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasRole(_) => <hasRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::isChainIdUsed(_) => {
                    <isChainIdUsedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::knownAppchains(_) => {
                    <knownAppchainsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::markChainIdAsUsed(_) => {
                    <markChainIdAsUsedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::namespacePrefix(_) => {
                    <namespacePrefixCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nextAutoChainId(_) => {
                    <nextAutoChainIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::renounceRole(_) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revokeRole(_) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::totalCumulativeGasFees(_) => {
                    <totalCumulativeGasFeesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::trackAppchainGas(_) => {
                    <trackAppchainGasCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::trackGas(_) => <trackGasCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateNamespaceConfig(_) => {
                    <updateNamespaceConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::usedChainIds(_) => {
                    <usedChainIdsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::usedNamespaces(_) => {
                    <usedNamespacesCall as alloy_sol_types::SolCall>::SELECTOR
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
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<SyndicateFactoryCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn getKnownAppchains(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getKnownAppchainsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getKnownAppchains)
                    }
                    getKnownAppchains
                },
                {
                    fn usedChainIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <usedChainIdsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::usedChainIds)
                    }
                    usedChainIds
                },
                {
                    fn getCurrentEpochTimeRemaining(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getCurrentEpochTimeRemainingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getCurrentEpochTimeRemaining)
                    }
                    getCurrentEpochTimeRemaining
                },
                {
                    fn trackGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <trackGasCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateFactoryCalls::trackGas)
                    }
                    trackGas
                },
                {
                    fn computeSequencingChainAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <computeSequencingChainAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::computeSequencingChainAddress)
                    }
                    computeSequencingChainAddress
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn namespacePrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <namespacePrefixCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::namespacePrefix)
                    }
                    namespacePrefix
                },
                {
                    fn getBytecode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getBytecodeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getBytecode)
                    }
                    getBytecode
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateFactoryCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn getCurrentEpochAggregate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getCurrentEpochAggregateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getCurrentEpochAggregate)
                    }
                    getCurrentEpochAggregate
                },
                {
                    fn gasTrackingInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <gasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::gasTrackingInitialized)
                    }
                    gasTrackingInitialized
                },
                {
                    fn markChainIdAsUsed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <markChainIdAsUsedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::markChainIdAsUsed)
                    }
                    markChainIdAsUsed
                },
                {
                    fn unpause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateFactoryCalls::unpause)
                    }
                    unpause
                },
                {
                    fn cumulativeGasFeesByAppchain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <cumulativeGasFeesByAppchainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::cumulativeGasFeesByAppchain)
                    }
                    cumulativeGasFeesByAppchain
                },
                {
                    fn disableGasTracking(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <disableGasTrackingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::disableGasTracking)
                    }
                    disableGasTracking
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateFactoryCalls::paused)
                    }
                    paused
                },
                {
                    fn currentEpochStart(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <currentEpochStartCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::currentEpochStart)
                    }
                    currentEpochStart
                },
                {
                    fn PERIOD_DURATION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::PERIOD_DURATION)
                    }
                    PERIOD_DURATION
                },
                {
                    fn appchainContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <appchainContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::appchainContracts)
                    }
                    appchainContracts
                },
                {
                    fn isChainIdUsed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <isChainIdUsedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::isChainIdUsed)
                    }
                    isChainIdUsed
                },
                {
                    fn currentEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <currentEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::currentEpoch)
                    }
                    currentEpoch
                },
                {
                    fn getAppchainContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getAppchainContractCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getAppchainContract)
                    }
                    getAppchainContract
                },
                {
                    fn pause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateFactoryCalls::pause)
                    }
                    pause
                },
                {
                    fn gasTrackingEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <gasTrackingEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::gasTrackingEnabled)
                    }
                    gasTrackingEnabled
                },
                {
                    fn updateNamespaceConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <updateNamespaceConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::updateNamespaceConfig)
                    }
                    updateNamespaceConfig
                },
                {
                    fn getNextChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getNextChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getNextChainId)
                    }
                    getNextChainId
                },
                {
                    fn aggregatePeriods(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <aggregatePeriodsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::aggregatePeriods)
                    }
                    aggregatePeriods
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateFactoryCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn getTotalCumulativeFees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getTotalCumulativeFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getTotalCumulativeFees)
                    }
                    getTotalCumulativeFees
                },
                {
                    fn nextAutoChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <nextAutoChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::nextAutoChainId)
                    }
                    nextAutoChainId
                },
                {
                    fn getAppchainEpochData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getAppchainEpochDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getAppchainEpochData)
                    }
                    getAppchainEpochData
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn totalCumulativeGasFees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <totalCumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::totalCumulativeGasFees)
                    }
                    totalCumulativeGasFees
                },
                {
                    fn usedNamespaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <usedNamespacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::usedNamespaces)
                    }
                    usedNamespaces
                },
                {
                    fn getAppchainCumulativeFees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getAppchainCumulativeFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getAppchainCumulativeFees)
                    }
                    getAppchainCumulativeFees
                },
                {
                    fn getCurrentAppchainData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getCurrentAppchainDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getCurrentAppchainData)
                    }
                    getCurrentAppchainData
                },
                {
                    fn getEpochAggregate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getEpochAggregateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getEpochAggregate)
                    }
                    getEpochAggregate
                },
                {
                    fn appchainPeriods(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <appchainPeriodsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::appchainPeriods)
                    }
                    appchainPeriods
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn createSyndicateSequencingChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <createSyndicateSequencingChainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::createSyndicateSequencingChain)
                    }
                    createSyndicateSequencingChain
                },
                {
                    fn knownAppchains(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <knownAppchainsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::knownAppchains)
                    }
                    knownAppchains
                },
                {
                    fn enableGasTracking(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <enableGasTrackingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::enableGasTracking)
                    }
                    enableGasTracking
                },
                {
                    fn trackAppchainGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <trackAppchainGasCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::trackAppchainGas)
                    }
                    trackAppchainGas
                },
                {
                    fn MANAGER_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::MANAGER_ROLE)
                    }
                    MANAGER_ROLE
                },
                {
                    fn appchainList(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <appchainListCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryCalls::appchainList)
                    }
                    appchainList
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
            ) -> alloy_sol_types::Result<SyndicateFactoryCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn getKnownAppchains(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getKnownAppchainsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getKnownAppchains)
                    }
                    getKnownAppchains
                },
                {
                    fn usedChainIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <usedChainIdsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::usedChainIds)
                    }
                    usedChainIds
                },
                {
                    fn getCurrentEpochTimeRemaining(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getCurrentEpochTimeRemainingCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getCurrentEpochTimeRemaining)
                    }
                    getCurrentEpochTimeRemaining
                },
                {
                    fn trackGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <trackGasCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::trackGas)
                    }
                    trackGas
                },
                {
                    fn computeSequencingChainAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <computeSequencingChainAddressCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::computeSequencingChainAddress)
                    }
                    computeSequencingChainAddress
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn namespacePrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <namespacePrefixCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::namespacePrefix)
                    }
                    namespacePrefix
                },
                {
                    fn getBytecode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getBytecodeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getBytecode)
                    }
                    getBytecode
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn getCurrentEpochAggregate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getCurrentEpochAggregateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getCurrentEpochAggregate)
                    }
                    getCurrentEpochAggregate
                },
                {
                    fn gasTrackingInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <gasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::gasTrackingInitialized)
                    }
                    gasTrackingInitialized
                },
                {
                    fn markChainIdAsUsed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <markChainIdAsUsedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::markChainIdAsUsed)
                    }
                    markChainIdAsUsed
                },
                {
                    fn unpause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::unpause)
                    }
                    unpause
                },
                {
                    fn cumulativeGasFeesByAppchain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <cumulativeGasFeesByAppchainCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::cumulativeGasFeesByAppchain)
                    }
                    cumulativeGasFeesByAppchain
                },
                {
                    fn disableGasTracking(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <disableGasTrackingCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::disableGasTracking)
                    }
                    disableGasTracking
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::paused)
                    }
                    paused
                },
                {
                    fn currentEpochStart(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <currentEpochStartCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::currentEpochStart)
                    }
                    currentEpochStart
                },
                {
                    fn PERIOD_DURATION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::PERIOD_DURATION)
                    }
                    PERIOD_DURATION
                },
                {
                    fn appchainContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <appchainContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::appchainContracts)
                    }
                    appchainContracts
                },
                {
                    fn isChainIdUsed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <isChainIdUsedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::isChainIdUsed)
                    }
                    isChainIdUsed
                },
                {
                    fn currentEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <currentEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::currentEpoch)
                    }
                    currentEpoch
                },
                {
                    fn getAppchainContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getAppchainContractCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getAppchainContract)
                    }
                    getAppchainContract
                },
                {
                    fn pause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::pause)
                    }
                    pause
                },
                {
                    fn gasTrackingEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <gasTrackingEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::gasTrackingEnabled)
                    }
                    gasTrackingEnabled
                },
                {
                    fn updateNamespaceConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <updateNamespaceConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::updateNamespaceConfig)
                    }
                    updateNamespaceConfig
                },
                {
                    fn getNextChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getNextChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getNextChainId)
                    }
                    getNextChainId
                },
                {
                    fn aggregatePeriods(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <aggregatePeriodsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::aggregatePeriods)
                    }
                    aggregatePeriods
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn getTotalCumulativeFees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getTotalCumulativeFeesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getTotalCumulativeFees)
                    }
                    getTotalCumulativeFees
                },
                {
                    fn nextAutoChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <nextAutoChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::nextAutoChainId)
                    }
                    nextAutoChainId
                },
                {
                    fn getAppchainEpochData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getAppchainEpochDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getAppchainEpochData)
                    }
                    getAppchainEpochData
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn totalCumulativeGasFees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <totalCumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::totalCumulativeGasFees)
                    }
                    totalCumulativeGasFees
                },
                {
                    fn usedNamespaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <usedNamespacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::usedNamespaces)
                    }
                    usedNamespaces
                },
                {
                    fn getAppchainCumulativeFees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getAppchainCumulativeFeesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getAppchainCumulativeFees)
                    }
                    getAppchainCumulativeFees
                },
                {
                    fn getCurrentAppchainData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getCurrentAppchainDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getCurrentAppchainData)
                    }
                    getCurrentAppchainData
                },
                {
                    fn getEpochAggregate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <getEpochAggregateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::getEpochAggregate)
                    }
                    getEpochAggregate
                },
                {
                    fn appchainPeriods(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <appchainPeriodsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::appchainPeriods)
                    }
                    appchainPeriods
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn createSyndicateSequencingChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <createSyndicateSequencingChainCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::createSyndicateSequencingChain)
                    }
                    createSyndicateSequencingChain
                },
                {
                    fn knownAppchains(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <knownAppchainsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::knownAppchains)
                    }
                    knownAppchains
                },
                {
                    fn enableGasTracking(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <enableGasTrackingCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::enableGasTracking)
                    }
                    enableGasTracking
                },
                {
                    fn trackAppchainGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <trackAppchainGasCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::trackAppchainGas)
                    }
                    trackAppchainGas
                },
                {
                    fn MANAGER_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::MANAGER_ROLE)
                    }
                    MANAGER_ROLE
                },
                {
                    fn appchainList(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryCalls> {
                        <appchainListCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryCalls::appchainList)
                    }
                    appchainList
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
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MANAGER_ROLE(inner) => {
                    <MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PERIOD_DURATION(inner) => {
                    <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::aggregatePeriods(inner) => {
                    <aggregatePeriodsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::appchainContracts(inner) => {
                    <appchainContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::appchainList(inner) => {
                    <appchainListCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::appchainPeriods(inner) => {
                    <appchainPeriodsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::computeSequencingChainAddress(inner) => {
                    <computeSequencingChainAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createSyndicateSequencingChain(inner) => {
                    <createSyndicateSequencingChainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cumulativeGasFeesByAppchain(inner) => {
                    <cumulativeGasFeesByAppchainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentEpoch(inner) => {
                    <currentEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentEpochStart(inner) => {
                    <currentEpochStartCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disableGasTracking(inner) => {
                    <disableGasTrackingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::enableGasTracking(inner) => {
                    <enableGasTrackingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::gasTrackingEnabled(inner) => {
                    <gasTrackingEnabledCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::gasTrackingInitialized(inner) => {
                    <gasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAppchainContract(inner) => {
                    <getAppchainContractCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAppchainCumulativeFees(inner) => {
                    <getAppchainCumulativeFeesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAppchainEpochData(inner) => {
                    <getAppchainEpochDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBytecode(inner) => {
                    <getBytecodeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentAppchainData(inner) => {
                    <getCurrentAppchainDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentEpochAggregate(inner) => {
                    <getCurrentEpochAggregateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentEpochTimeRemaining(inner) => {
                    <getCurrentEpochTimeRemainingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getEpochAggregate(inner) => {
                    <getEpochAggregateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKnownAppchains(inner) => {
                    <getKnownAppchainsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getNextChainId(inner) => {
                    <getNextChainIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalCumulativeFees(inner) => {
                    <getTotalCumulativeFeesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isChainIdUsed(inner) => {
                    <isChainIdUsedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::knownAppchains(inner) => {
                    <knownAppchainsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::markChainIdAsUsed(inner) => {
                    <markChainIdAsUsedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::namespacePrefix(inner) => {
                    <namespacePrefixCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nextAutoChainId(inner) => {
                    <nextAutoChainIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::totalCumulativeGasFees(inner) => {
                    <totalCumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::trackAppchainGas(inner) => {
                    <trackAppchainGasCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::trackGas(inner) => {
                    <trackGasCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateNamespaceConfig(inner) => {
                    <updateNamespaceConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::usedChainIds(inner) => {
                    <usedChainIdsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::usedNamespaces(inner) => {
                    <usedNamespacesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MANAGER_ROLE(inner) => {
                    <MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PERIOD_DURATION(inner) => {
                    <PERIOD_DURATIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::aggregatePeriods(inner) => {
                    <aggregatePeriodsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::appchainContracts(inner) => {
                    <appchainContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::appchainList(inner) => {
                    <appchainListCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::appchainPeriods(inner) => {
                    <appchainPeriodsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::computeSequencingChainAddress(inner) => {
                    <computeSequencingChainAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createSyndicateSequencingChain(inner) => {
                    <createSyndicateSequencingChainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cumulativeGasFeesByAppchain(inner) => {
                    <cumulativeGasFeesByAppchainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currentEpoch(inner) => {
                    <currentEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currentEpochStart(inner) => {
                    <currentEpochStartCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::disableGasTracking(inner) => {
                    <disableGasTrackingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::enableGasTracking(inner) => {
                    <enableGasTrackingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::gasTrackingEnabled(inner) => {
                    <gasTrackingEnabledCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::gasTrackingInitialized(inner) => {
                    <gasTrackingInitializedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAppchainContract(inner) => {
                    <getAppchainContractCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAppchainCumulativeFees(inner) => {
                    <getAppchainCumulativeFeesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAppchainEpochData(inner) => {
                    <getAppchainEpochDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBytecode(inner) => {
                    <getBytecodeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentAppchainData(inner) => {
                    <getCurrentAppchainDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentEpochAggregate(inner) => {
                    <getCurrentEpochAggregateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentEpochTimeRemaining(inner) => {
                    <getCurrentEpochTimeRemainingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getEpochAggregate(inner) => {
                    <getEpochAggregateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKnownAppchains(inner) => {
                    <getKnownAppchainsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getNextChainId(inner) => {
                    <getNextChainIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getTotalCumulativeFees(inner) => {
                    <getTotalCumulativeFeesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isChainIdUsed(inner) => {
                    <isChainIdUsedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::knownAppchains(inner) => {
                    <knownAppchainsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::markChainIdAsUsed(inner) => {
                    <markChainIdAsUsedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::namespacePrefix(inner) => {
                    <namespacePrefixCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nextAutoChainId(inner) => {
                    <nextAutoChainIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::totalCumulativeGasFees(inner) => {
                    <totalCumulativeGasFeesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::trackAppchainGas(inner) => {
                    <trackAppchainGasCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::trackGas(inner) => {
                    <trackGasCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateNamespaceConfig(inner) => {
                    <updateNamespaceConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::usedChainIds(inner) => {
                    <usedChainIdsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::usedNamespaces(inner) => {
                    <usedNamespacesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SyndicateFactory`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SyndicateFactoryErrors {
        #[allow(missing_docs)]
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        #[allow(missing_docs)]
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        #[allow(missing_docs)]
        ChainIdAlreadyExists(ChainIdAlreadyExists),
        #[allow(missing_docs)]
        Create2EmptyBytecode(Create2EmptyBytecode),
        #[allow(missing_docs)]
        EnforcedPause(EnforcedPause),
        #[allow(missing_docs)]
        ExpectedPause(ExpectedPause),
        #[allow(missing_docs)]
        FailedDeployment(FailedDeployment),
        #[allow(missing_docs)]
        GasTrackingDisabled(GasTrackingDisabled),
        #[allow(missing_docs)]
        InsufficientBalance(InsufficientBalance),
        #[allow(missing_docs)]
        InvalidAppchainId(InvalidAppchainId),
        #[allow(missing_docs)]
        InvalidGasAmount(InvalidGasAmount),
        #[allow(missing_docs)]
        StringsInvalidChar(StringsInvalidChar),
        #[allow(missing_docs)]
        ZeroAddress(ZeroAddress),
    }
    #[automatically_derived]
    impl SyndicateFactoryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [36u8, 89u8, 29u8, 137u8],
            [76u8, 162u8, 73u8, 220u8],
            [102u8, 151u8, 178u8, 50u8],
            [116u8, 140u8, 24u8, 61u8],
            [128u8, 101u8, 54u8, 101u8],
            [141u8, 252u8, 32u8, 43u8],
            [148u8, 226u8, 115u8, 126u8],
            [176u8, 110u8, 191u8, 61u8],
            [207u8, 71u8, 145u8, 129u8],
            [217u8, 46u8, 35u8, 61u8],
            [217u8, 60u8, 6u8, 101u8],
            [226u8, 81u8, 125u8, 63u8],
            [246u8, 180u8, 19u8, 28u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SyndicateFactoryErrors {
        const NAME: &'static str = "SyndicateFactoryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccessControlBadConfirmation(_) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AccessControlUnauthorizedAccount(_) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ChainIdAlreadyExists(_) => {
                    <ChainIdAlreadyExists as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Create2EmptyBytecode(_) => {
                    <Create2EmptyBytecode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EnforcedPause(_) => {
                    <EnforcedPause as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExpectedPause(_) => {
                    <ExpectedPause as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedDeployment(_) => {
                    <FailedDeployment as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GasTrackingDisabled(_) => {
                    <GasTrackingDisabled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientBalance(_) => {
                    <InsufficientBalance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidAppchainId(_) => {
                    <InvalidAppchainId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidGasAmount(_) => {
                    <InvalidGasAmount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StringsInvalidChar(_) => {
                    <StringsInvalidChar as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroAddress(_) => {
                    <ZeroAddress as alloy_sol_types::SolError>::SELECTOR
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
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<SyndicateFactoryErrors>] = &[
                {
                    fn ChainIdAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <ChainIdAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryErrors::ChainIdAlreadyExists)
                    }
                    ChainIdAlreadyExists
                },
                {
                    fn Create2EmptyBytecode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <Create2EmptyBytecode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryErrors::Create2EmptyBytecode)
                    }
                    Create2EmptyBytecode
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn InvalidGasAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <InvalidGasAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryErrors::InvalidGasAmount)
                    }
                    InvalidGasAmount
                },
                {
                    fn GasTrackingDisabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <GasTrackingDisabled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryErrors::GasTrackingDisabled)
                    }
                    GasTrackingDisabled
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn StringsInvalidChar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <StringsInvalidChar as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryErrors::StringsInvalidChar)
                    }
                    StringsInvalidChar
                },
                {
                    fn FailedDeployment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <FailedDeployment as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryErrors::FailedDeployment)
                    }
                    FailedDeployment
                },
                {
                    fn InsufficientBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <InsufficientBalance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryErrors::InsufficientBalance)
                    }
                    InsufficientBalance
                },
                {
                    fn ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(SyndicateFactoryErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SyndicateFactoryErrors::AccessControlUnauthorizedAccount,
                            )
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn InvalidAppchainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <InvalidAppchainId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateFactoryErrors::InvalidAppchainId)
                    }
                    InvalidAppchainId
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
            ) -> alloy_sol_types::Result<SyndicateFactoryErrors>] = &[
                {
                    fn ChainIdAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <ChainIdAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryErrors::ChainIdAlreadyExists)
                    }
                    ChainIdAlreadyExists
                },
                {
                    fn Create2EmptyBytecode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <Create2EmptyBytecode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryErrors::Create2EmptyBytecode)
                    }
                    Create2EmptyBytecode
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn InvalidGasAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <InvalidGasAmount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryErrors::InvalidGasAmount)
                    }
                    InvalidGasAmount
                },
                {
                    fn GasTrackingDisabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <GasTrackingDisabled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryErrors::GasTrackingDisabled)
                    }
                    GasTrackingDisabled
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn StringsInvalidChar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <StringsInvalidChar as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryErrors::StringsInvalidChar)
                    }
                    StringsInvalidChar
                },
                {
                    fn FailedDeployment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <FailedDeployment as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryErrors::FailedDeployment)
                    }
                    FailedDeployment
                },
                {
                    fn InsufficientBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <InsufficientBalance as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryErrors::InsufficientBalance)
                    }
                    InsufficientBalance
                },
                {
                    fn ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SyndicateFactoryErrors::AccessControlUnauthorizedAccount,
                            )
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn InvalidAppchainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateFactoryErrors> {
                        <InvalidAppchainId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateFactoryErrors::InvalidAppchainId)
                    }
                    InvalidAppchainId
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
                Self::ChainIdAlreadyExists(inner) => {
                    <ChainIdAlreadyExists as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Create2EmptyBytecode(inner) => {
                    <Create2EmptyBytecode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FailedDeployment(inner) => {
                    <FailedDeployment as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GasTrackingDisabled(inner) => {
                    <GasTrackingDisabled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientBalance(inner) => {
                    <InsufficientBalance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidAppchainId(inner) => {
                    <InvalidAppchainId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidGasAmount(inner) => {
                    <InvalidGasAmount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StringsInvalidChar(inner) => {
                    <StringsInvalidChar as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroAddress(inner) => {
                    <ZeroAddress as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::ChainIdAlreadyExists(inner) => {
                    <ChainIdAlreadyExists as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Create2EmptyBytecode(inner) => {
                    <Create2EmptyBytecode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedDeployment(inner) => {
                    <FailedDeployment as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GasTrackingDisabled(inner) => {
                    <GasTrackingDisabled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientBalance(inner) => {
                    <InsufficientBalance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidAppchainId(inner) => {
                    <InvalidAppchainId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidGasAmount(inner) => {
                    <InvalidGasAmount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StringsInvalidChar(inner) => {
                    <StringsInvalidChar as alloy_sol_types::SolError>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`SyndicateFactory`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SyndicateFactoryEvents {
        #[allow(missing_docs)]
        ChainIdManuallyMarked(ChainIdManuallyMarked),
        #[allow(missing_docs)]
        EpochFinalized(EpochFinalized),
        #[allow(missing_docs)]
        GasTracked(GasTracked),
        #[allow(missing_docs)]
        NamespaceConfigUpdated(NamespaceConfigUpdated),
        #[allow(missing_docs)]
        NewEpochStarted(NewEpochStarted),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        RoleAdminChanged(RoleAdminChanged),
        #[allow(missing_docs)]
        RoleGranted(RoleGranted),
        #[allow(missing_docs)]
        RoleRevoked(RoleRevoked),
        #[allow(missing_docs)]
        SyndicateSequencingChainCreated(SyndicateSequencingChainCreated),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl SyndicateFactoryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                33u8, 194u8, 92u8, 208u8, 165u8, 78u8, 22u8, 9u8, 227u8, 230u8, 139u8,
                51u8, 86u8, 147u8, 238u8, 251u8, 105u8, 77u8, 94u8, 241u8, 124u8, 195u8,
                43u8, 16u8, 111u8, 145u8, 63u8, 138u8, 31u8, 27u8, 128u8, 88u8,
            ],
            [
                34u8, 158u8, 130u8, 179u8, 62u8, 10u8, 255u8, 118u8, 132u8, 79u8, 102u8,
                246u8, 38u8, 212u8, 42u8, 210u8, 230u8, 35u8, 154u8, 243u8, 78u8, 176u8,
                244u8, 109u8, 19u8, 101u8, 251u8, 90u8, 239u8, 121u8, 109u8, 176u8,
            ],
            [
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ],
            [
                73u8, 178u8, 31u8, 30u8, 65u8, 144u8, 219u8, 139u8, 10u8, 147u8, 60u8,
                149u8, 30u8, 208u8, 19u8, 222u8, 34u8, 44u8, 132u8, 124u8, 21u8, 70u8,
                23u8, 84u8, 104u8, 45u8, 170u8, 46u8, 171u8, 31u8, 219u8, 210u8,
            ],
            [
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8,
                167u8, 131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8,
                78u8, 83u8, 123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ],
            [
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8,
                2u8, 112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8,
                71u8, 84u8, 235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ],
            [
                165u8, 175u8, 47u8, 183u8, 115u8, 11u8, 81u8, 181u8, 253u8, 175u8, 129u8,
                64u8, 210u8, 137u8, 21u8, 137u8, 249u8, 208u8, 88u8, 45u8, 132u8, 200u8,
                217u8, 125u8, 212u8, 225u8, 138u8, 58u8, 207u8, 228u8, 119u8, 47u8,
            ],
            [
                180u8, 99u8, 209u8, 158u8, 207u8, 69u8, 91u8, 230u8, 83u8, 101u8, 9u8,
                44u8, 248u8, 225u8, 219u8, 105u8, 52u8, 160u8, 51u8, 76u8, 248u8, 205u8,
                83u8, 45u8, 223u8, 153u8, 100u8, 208u8, 31u8, 54u8, 181u8, 178u8,
            ],
            [
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8,
                81u8, 66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8,
                71u8, 92u8, 58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ],
            [
                217u8, 225u8, 35u8, 145u8, 119u8, 191u8, 189u8, 42u8, 235u8, 245u8,
                208u8, 242u8, 15u8, 192u8, 117u8, 230u8, 223u8, 90u8, 80u8, 44u8, 89u8,
                209u8, 33u8, 172u8, 213u8, 115u8, 66u8, 199u8, 131u8, 227u8, 19u8, 100u8,
            ],
            [
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for SyndicateFactoryEvents {
        const NAME: &'static str = "SyndicateFactoryEvents";
        const COUNT: usize = 11usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <ChainIdManuallyMarked as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ChainIdManuallyMarked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ChainIdManuallyMarked)
                }
                Some(<EpochFinalized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EpochFinalized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EpochFinalized)
                }
                Some(<GasTracked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <GasTracked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::GasTracked)
                }
                Some(
                    <NamespaceConfigUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <NamespaceConfigUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::NamespaceConfigUpdated)
                }
                Some(<NewEpochStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewEpochStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::NewEpochStarted)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Paused)
                }
                Some(<RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleAdminChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleAdminChanged)
                }
                Some(<RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleGranted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleGranted)
                }
                Some(<RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleRevoked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleRevoked)
                }
                Some(
                    <SyndicateSequencingChainCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SyndicateSequencingChainCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SyndicateSequencingChainCreated)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Unpaused)
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
    impl alloy_sol_types::private::IntoLogData for SyndicateFactoryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ChainIdManuallyMarked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EpochFinalized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::GasTracked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NamespaceConfigUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewEpochStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
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
                Self::SyndicateSequencingChainCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ChainIdManuallyMarked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EpochFinalized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::GasTracked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NamespaceConfigUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewEpochStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
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
                Self::SyndicateSequencingChainCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SyndicateFactory`](self) contract instance.

See the [wrapper's documentation](`SyndicateFactoryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SyndicateFactoryInstance<P, N> {
        SyndicateFactoryInstance::<P, N>::new(address, provider)
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
        admin: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SyndicateFactoryInstance<P, N>>,
    > {
        SyndicateFactoryInstance::<P, N>::deploy(provider, admin)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        admin: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        SyndicateFactoryInstance::<P, N>::deploy_builder(provider, admin)
    }
    /**A [`SyndicateFactory`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SyndicateFactory`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SyndicateFactoryInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for SyndicateFactoryInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SyndicateFactoryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SyndicateFactoryInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`SyndicateFactory`](self) contract instance.

See the [wrapper's documentation](`SyndicateFactoryInstance`) for more details.*/
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
            admin: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<SyndicateFactoryInstance<P, N>> {
            let call_builder = Self::deploy_builder(provider, admin);
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
            admin: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { admin },
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
    impl<P: ::core::clone::Clone, N> SyndicateFactoryInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SyndicateFactoryInstance<P, N> {
            SyndicateFactoryInstance {
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
    > SyndicateFactoryInstance<P, N> {
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
        ///Creates a new call builder for the [`DEFAULT_ADMIN_ROLE`] function.
        pub fn DEFAULT_ADMIN_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DEFAULT_ADMIN_ROLECall, N> {
            self.call_builder(&DEFAULT_ADMIN_ROLECall)
        }
        ///Creates a new call builder for the [`MANAGER_ROLE`] function.
        pub fn MANAGER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, MANAGER_ROLECall, N> {
            self.call_builder(&MANAGER_ROLECall)
        }
        ///Creates a new call builder for the [`PERIOD_DURATION`] function.
        pub fn PERIOD_DURATION(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, PERIOD_DURATIONCall, N> {
            self.call_builder(&PERIOD_DURATIONCall)
        }
        ///Creates a new call builder for the [`aggregatePeriods`] function.
        pub fn aggregatePeriods(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, aggregatePeriodsCall, N> {
            self.call_builder(&aggregatePeriodsCall(_0))
        }
        ///Creates a new call builder for the [`appchainContracts`] function.
        pub fn appchainContracts(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, appchainContractsCall, N> {
            self.call_builder(&appchainContractsCall(_0))
        }
        ///Creates a new call builder for the [`appchainList`] function.
        pub fn appchainList(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, appchainListCall, N> {
            self.call_builder(&appchainListCall(_0))
        }
        ///Creates a new call builder for the [`appchainPeriods`] function.
        pub fn appchainPeriods(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, appchainPeriodsCall, N> {
            self.call_builder(&appchainPeriodsCall { _0, _1 })
        }
        ///Creates a new call builder for the [`computeSequencingChainAddress`] function.
        pub fn computeSequencingChainAddress(
            &self,
            salt: alloy::sol_types::private::FixedBytes<32>,
            chainId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, computeSequencingChainAddressCall, N> {
            self.call_builder(
                &computeSequencingChainAddressCall {
                    salt,
                    chainId,
                },
            )
        }
        ///Creates a new call builder for the [`createSyndicateSequencingChain`] function.
        pub fn createSyndicateSequencingChain(
            &self,
            appchainId: alloy::sol_types::private::primitives::aliases::U256,
            admin: alloy::sol_types::private::Address,
            permissionModule: alloy::sol_types::private::Address,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, createSyndicateSequencingChainCall, N> {
            self.call_builder(
                &createSyndicateSequencingChainCall {
                    appchainId,
                    admin,
                    permissionModule,
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`cumulativeGasFeesByAppchain`] function.
        pub fn cumulativeGasFeesByAppchain(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, cumulativeGasFeesByAppchainCall, N> {
            self.call_builder(&cumulativeGasFeesByAppchainCall(_0))
        }
        ///Creates a new call builder for the [`currentEpoch`] function.
        pub fn currentEpoch(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, currentEpochCall, N> {
            self.call_builder(&currentEpochCall)
        }
        ///Creates a new call builder for the [`currentEpochStart`] function.
        pub fn currentEpochStart(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, currentEpochStartCall, N> {
            self.call_builder(&currentEpochStartCall)
        }
        ///Creates a new call builder for the [`disableGasTracking`] function.
        pub fn disableGasTracking(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, disableGasTrackingCall, N> {
            self.call_builder(&disableGasTrackingCall)
        }
        ///Creates a new call builder for the [`enableGasTracking`] function.
        pub fn enableGasTracking(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, enableGasTrackingCall, N> {
            self.call_builder(&enableGasTrackingCall)
        }
        ///Creates a new call builder for the [`gasTrackingEnabled`] function.
        pub fn gasTrackingEnabled(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, gasTrackingEnabledCall, N> {
            self.call_builder(&gasTrackingEnabledCall)
        }
        ///Creates a new call builder for the [`gasTrackingInitialized`] function.
        pub fn gasTrackingInitialized(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, gasTrackingInitializedCall, N> {
            self.call_builder(&gasTrackingInitializedCall)
        }
        ///Creates a new call builder for the [`getAppchainContract`] function.
        pub fn getAppchainContract(
            &self,
            chainId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getAppchainContractCall, N> {
            self.call_builder(&getAppchainContractCall { chainId })
        }
        ///Creates a new call builder for the [`getAppchainCumulativeFees`] function.
        pub fn getAppchainCumulativeFees(
            &self,
            appchainId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getAppchainCumulativeFeesCall, N> {
            self.call_builder(
                &getAppchainCumulativeFeesCall {
                    appchainId,
                },
            )
        }
        ///Creates a new call builder for the [`getAppchainEpochData`] function.
        pub fn getAppchainEpochData(
            &self,
            epoch: alloy::sol_types::private::primitives::aliases::U256,
            appchainId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getAppchainEpochDataCall, N> {
            self.call_builder(
                &getAppchainEpochDataCall {
                    epoch,
                    appchainId,
                },
            )
        }
        ///Creates a new call builder for the [`getBytecode`] function.
        pub fn getBytecode(
            &self,
            chainId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getBytecodeCall, N> {
            self.call_builder(&getBytecodeCall { chainId })
        }
        ///Creates a new call builder for the [`getCurrentAppchainData`] function.
        pub fn getCurrentAppchainData(
            &self,
            appchainId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentAppchainDataCall, N> {
            self.call_builder(
                &getCurrentAppchainDataCall {
                    appchainId,
                },
            )
        }
        ///Creates a new call builder for the [`getCurrentEpochAggregate`] function.
        pub fn getCurrentEpochAggregate(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentEpochAggregateCall, N> {
            self.call_builder(&getCurrentEpochAggregateCall)
        }
        ///Creates a new call builder for the [`getCurrentEpochTimeRemaining`] function.
        pub fn getCurrentEpochTimeRemaining(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentEpochTimeRemainingCall, N> {
            self.call_builder(&getCurrentEpochTimeRemainingCall)
        }
        ///Creates a new call builder for the [`getEpochAggregate`] function.
        pub fn getEpochAggregate(
            &self,
            epoch: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getEpochAggregateCall, N> {
            self.call_builder(&getEpochAggregateCall { epoch })
        }
        ///Creates a new call builder for the [`getKnownAppchains`] function.
        pub fn getKnownAppchains(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getKnownAppchainsCall, N> {
            self.call_builder(&getKnownAppchainsCall)
        }
        ///Creates a new call builder for the [`getNextChainId`] function.
        pub fn getNextChainId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getNextChainIdCall, N> {
            self.call_builder(&getNextChainIdCall)
        }
        ///Creates a new call builder for the [`getRoleAdmin`] function.
        pub fn getRoleAdmin(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getRoleAdminCall, N> {
            self.call_builder(&getRoleAdminCall { role })
        }
        ///Creates a new call builder for the [`getTotalCumulativeFees`] function.
        pub fn getTotalCumulativeFees(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getTotalCumulativeFeesCall, N> {
            self.call_builder(&getTotalCumulativeFeesCall)
        }
        ///Creates a new call builder for the [`grantRole`] function.
        pub fn grantRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, grantRoleCall, N> {
            self.call_builder(&grantRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hasRole`] function.
        pub fn hasRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, hasRoleCall, N> {
            self.call_builder(&hasRoleCall { role, account })
        }
        ///Creates a new call builder for the [`isChainIdUsed`] function.
        pub fn isChainIdUsed(
            &self,
            chainId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isChainIdUsedCall, N> {
            self.call_builder(&isChainIdUsedCall { chainId })
        }
        ///Creates a new call builder for the [`knownAppchains`] function.
        pub fn knownAppchains(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, knownAppchainsCall, N> {
            self.call_builder(&knownAppchainsCall(_0))
        }
        ///Creates a new call builder for the [`markChainIdAsUsed`] function.
        pub fn markChainIdAsUsed(
            &self,
            chainId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, markChainIdAsUsedCall, N> {
            self.call_builder(&markChainIdAsUsedCall { chainId })
        }
        ///Creates a new call builder for the [`namespacePrefix`] function.
        pub fn namespacePrefix(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, namespacePrefixCall, N> {
            self.call_builder(&namespacePrefixCall)
        }
        ///Creates a new call builder for the [`nextAutoChainId`] function.
        pub fn nextAutoChainId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, nextAutoChainIdCall, N> {
            self.call_builder(&nextAutoChainIdCall)
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(&self) -> alloy_contract::SolCallBuilder<&P, pauseCall, N> {
            self.call_builder(&pauseCall)
        }
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<&P, pausedCall, N> {
            self.call_builder(&pausedCall)
        }
        ///Creates a new call builder for the [`renounceRole`] function.
        pub fn renounceRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            callerConfirmation: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, renounceRoleCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<&P, revokeRoleCall, N> {
            self.call_builder(&revokeRoleCall { role, account })
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<&P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
        }
        ///Creates a new call builder for the [`totalCumulativeGasFees`] function.
        pub fn totalCumulativeGasFees(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, totalCumulativeGasFeesCall, N> {
            self.call_builder(&totalCumulativeGasFeesCall)
        }
        ///Creates a new call builder for the [`trackAppchainGas`] function.
        pub fn trackAppchainGas(
            &self,
            appchainId: alloy::sol_types::private::primitives::aliases::U256,
            gasUsed: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, trackAppchainGasCall, N> {
            self.call_builder(
                &trackAppchainGasCall {
                    appchainId,
                    gasUsed,
                },
            )
        }
        ///Creates a new call builder for the [`trackGas`] function.
        pub fn trackGas(
            &self,
            appchainId: alloy::sol_types::private::primitives::aliases::U256,
            gasUsed: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, trackGasCall, N> {
            self.call_builder(
                &trackGasCall {
                    appchainId,
                    gasUsed,
                },
            )
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(&self) -> alloy_contract::SolCallBuilder<&P, unpauseCall, N> {
            self.call_builder(&unpauseCall)
        }
        ///Creates a new call builder for the [`updateNamespaceConfig`] function.
        pub fn updateNamespaceConfig(
            &self,
            newPrefix: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, updateNamespaceConfigCall, N> {
            self.call_builder(
                &updateNamespaceConfigCall {
                    newPrefix,
                },
            )
        }
        ///Creates a new call builder for the [`usedChainIds`] function.
        pub fn usedChainIds(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, usedChainIdsCall, N> {
            self.call_builder(&usedChainIdsCall(_0))
        }
        ///Creates a new call builder for the [`usedNamespaces`] function.
        pub fn usedNamespaces(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, usedNamespacesCall, N> {
            self.call_builder(&usedNamespacesCall(_0))
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SyndicateFactoryInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ChainIdManuallyMarked`] event.
        pub fn ChainIdManuallyMarked_filter(
            &self,
        ) -> alloy_contract::Event<&P, ChainIdManuallyMarked, N> {
            self.event_filter::<ChainIdManuallyMarked>()
        }
        ///Creates a new event filter for the [`EpochFinalized`] event.
        pub fn EpochFinalized_filter(
            &self,
        ) -> alloy_contract::Event<&P, EpochFinalized, N> {
            self.event_filter::<EpochFinalized>()
        }
        ///Creates a new event filter for the [`GasTracked`] event.
        pub fn GasTracked_filter(&self) -> alloy_contract::Event<&P, GasTracked, N> {
            self.event_filter::<GasTracked>()
        }
        ///Creates a new event filter for the [`NamespaceConfigUpdated`] event.
        pub fn NamespaceConfigUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, NamespaceConfigUpdated, N> {
            self.event_filter::<NamespaceConfigUpdated>()
        }
        ///Creates a new event filter for the [`NewEpochStarted`] event.
        pub fn NewEpochStarted_filter(
            &self,
        ) -> alloy_contract::Event<&P, NewEpochStarted, N> {
            self.event_filter::<NewEpochStarted>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<&P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`RoleAdminChanged`] event.
        pub fn RoleAdminChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, RoleAdminChanged, N> {
            self.event_filter::<RoleAdminChanged>()
        }
        ///Creates a new event filter for the [`RoleGranted`] event.
        pub fn RoleGranted_filter(&self) -> alloy_contract::Event<&P, RoleGranted, N> {
            self.event_filter::<RoleGranted>()
        }
        ///Creates a new event filter for the [`RoleRevoked`] event.
        pub fn RoleRevoked_filter(&self) -> alloy_contract::Event<&P, RoleRevoked, N> {
            self.event_filter::<RoleRevoked>()
        }
        ///Creates a new event filter for the [`SyndicateSequencingChainCreated`] event.
        pub fn SyndicateSequencingChainCreated_filter(
            &self,
        ) -> alloy_contract::Event<&P, SyndicateSequencingChainCreated, N> {
            self.event_filter::<SyndicateSequencingChainCreated>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<&P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
