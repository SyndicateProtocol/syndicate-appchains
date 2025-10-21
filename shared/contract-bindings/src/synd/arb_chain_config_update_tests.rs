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

interface ArbChainConfigUpdateTests {
    event AppchainBlockExplorerUrlUpdated(string newUrl);
    event ArbChainConfigCreated(uint256 indexed chainId, address configAddress);
    event DefaultSequencingChainWsRpcUrlUpdated(string newRpcUrl);
    event ImplementationUpgraded(address newImplementation);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
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

    function APPCHAIN_BLOCK_EXPLORER_URL() external view returns (string memory);
    function ARBITRUM_BRIDGE_ADDRESS() external view returns (address);
    function ARBITRUM_INBOX_ADDRESS() external view returns (address);
    function CHAIN_ID() external view returns (uint256);
    function DEFAULT_WS_RPC_URL() external view returns (string memory);
    function IS_TEST() external view returns (bool);
    function SEQUENCING_CHAIN_ID() external view returns (uint256);
    function SEQUENCING_CONTRACT_ADDRESS() external view returns (address);
    function SEQUENCING_START_BLOCK() external view returns (uint256);
    function SETTLEMENT_DELAY() external view returns (uint256);
    function SETTLEMENT_START_BLOCK() external view returns (uint256);
    function appchainOwner() external view returns (address);
    function chainConfig() external view returns (address);
    function configManager() external view returns (address);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function owner() external view returns (address);
    function setUp() external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testEmptyAppchainBlockExplorerUrl() external;
    function testEmptyDefaultRpcUrl() external;
    function testTransferOwnership() external;
    function testTransferOwnershipRevertOnNonOwner() external;
    function testTransferOwnershipRevertOnZeroAddress() external;
    function testUpdateAppchainBlockExplorerUrl() external;
    function testUpdateAppchainBlockExplorerUrlRevertOnNonOwner() external;
    function testUpdateDefaultSequencingChainRpcUrl() external;
    function testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner() external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "APPCHAIN_BLOCK_EXPLORER_URL",
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
    "name": "ARBITRUM_BRIDGE_ADDRESS",
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
    "name": "ARBITRUM_INBOX_ADDRESS",
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
    "name": "CHAIN_ID",
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
    "name": "DEFAULT_WS_RPC_URL",
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
    "name": "SEQUENCING_CHAIN_ID",
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
    "name": "SEQUENCING_CONTRACT_ADDRESS",
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
    "name": "SEQUENCING_START_BLOCK",
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
    "name": "SETTLEMENT_DELAY",
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
    "name": "SETTLEMENT_START_BLOCK",
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
    "name": "appchainOwner",
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
    "name": "chainConfig",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ArbChainConfig"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "configManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ArbConfigManager"
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
    "name": "testEmptyAppchainBlockExplorerUrl",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testEmptyDefaultRpcUrl",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testTransferOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testTransferOwnershipRevertOnNonOwner",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testTransferOwnershipRevertOnZeroAddress",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testUpdateAppchainBlockExplorerUrl",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testUpdateAppchainBlockExplorerUrlRevertOnNonOwner",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testUpdateDefaultSequencingChainRpcUrl",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "AppchainBlockExplorerUrlUpdated",
    "inputs": [
      {
        "name": "newUrl",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ArbChainConfigCreated",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "configAddress",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DefaultSequencingChainWsRpcUrlUpdated",
    "inputs": [
      {
        "name": "newRpcUrl",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ImplementationUpgraded",
    "inputs": [
      {
        "name": "newImplementation",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
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
pub mod ArbChainConfigUpdateTests {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60808060405234604957600c805460ff19166001179055601f80546001600160a81b031916610101179055602080546001600160a01b0319166002179055615e3a908161004e8239f35b5f80fdfe60806040526004361015610011575f80fd5b5f5f3560e01c80630743bf6e1461264e5780630a9254e4146121ba5780631ed7831c1461212f57806326df4e2514611f925780632ade388014611d8457806334784bf114611b375780633e5e3c2314611aac5780633f7286f414611a215780634dbdc85d1461186157806352069fc21461168057806357d1ba25146116645780635b7c15991461131657806360944ce61461113557806366d9a9a014610ff85780636806ba0614610fc85780636edd6c0914610fab57806385226c8114610f215780638522ac8c14610af257806385e1f4d014610ad45780638da5cb5b14610a9d578063916a17c6146109e6578063a3c6e1e7146109ca578063aa6a43d8146109ad578063b0464fdc146108f6578063b5508aa91461086c578063ba414fa614610847578063bf6db6f81461082a578063ca0ab075146107f6578063d1f4737c146107da578063d831975e146107a7578063e20c9f711461070c578063ebb43738146106d8578063f8a144be146106ba578063fa7626d414610697578063fdfc99df1461037a5763febc4432146101a6575f80fd5b3461033f578060031936011261033f57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517fca669fa70000000000000000000000000000000000000000000000000000000081526103e76004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610365575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f576040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f43616c6c6572206973206e6f7420746865206f776e657200000000000000000060448201528190818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610350575b5073ffffffffffffffffffffffffffffffffffffffff60225416803b1561034d578180916024604051809481937ff2fde38b0000000000000000000000000000000000000000000000000000000083526004808401525af180156103425761032e5750f35b81610338916126b2565b61033f5780f35b80fd5b6040513d84823e3d90fd5b50fd5b8161035a916126b2565b61033f57805f6102c9565b8161036f916126b2565b61033f57805f610228565b503461033f578060031936011261033f578073ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610682575b5050610421612ded565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561066957816040517f491cc7c200000000000000000000000000000000000000000000000000000000815281818061048a60048201906001606060808401938281525f60208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103425761066d575b50507f67c57ae6ad924cd093fb2f06bc0b28fd5879481051a9c203a44d27c8904d437c60405160208152806104e7602082018561272e565b0390a18173ffffffffffffffffffffffffffffffffffffffff60225416803b15610669578160405180927f290803560000000000000000000000000000000000000000000000000000000082526020600483015281838161054b602482018a61272e565b03925af1801561034257610654575b509060049173ffffffffffffffffffffffffffffffffffffffff60225416604051938480927fc7a760950000000000000000000000000000000000000000000000000000000082525afa8015610649576105bb928491610627575b50612f01565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103425761032e5750f35b61064391503d8086833e61063b81836126b2565b810190612971565b5f6105b5565b6040513d85823e3d90fd5b8161065e916126b2565b61066957815f61055a565b5080fd5b81610677916126b2565b61066957815f6104af565b8161068c916126b2565b61033f57805f610417565b503461033f578060031936011261033f57602060ff601f54166040519015158152f35b503461033f578060031936011261033f5760206040516209fbf18152f35b503461033f578060031936011261033f57602073ffffffffffffffffffffffffffffffffffffffff60225416604051908152f35b503461033f578060031936011261033f5760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b81811061077b576107778561076b818703826126b2565b60405191829182612771565b0390f35b825473ffffffffffffffffffffffffffffffffffffffff16845260209093019260019283019201610754565b503461033f578060031936011261033f57602073ffffffffffffffffffffffffffffffffffffffff815416604051908152f35b503461033f578060031936011261033f57602060405160648152f35b503461033f578060031936011261033f57602073ffffffffffffffffffffffffffffffffffffffff60215416604051908152f35b503461033f578060031936011261033f576020604051619abc8152f35b503461033f578060031936011261033f576020610862612e28565b6040519015158152f35b503461033f578060031936011261033f5760195461088981612a02565b9161089760405193846126b2565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b8383106108d957604051806107778782612850565b6001602081926108e885612a1a565b8152019201920191906108c4565b503461033f578060031936011261033f57601c5461091381612a02565b9161092160405193846126b2565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310610963576040518061077787826128cd565b6002602060019260405161097681612669565b73ffffffffffffffffffffffffffffffffffffffff865416815261099b858701612b1d565b8382015281520192019201919061094e565b503461033f578060031936011261033f5760206040516156788152f35b503461033f578060031936011261033f57602060405160c88152f35b503461033f578060031936011261033f57601d54610a0381612a02565b91610a1160405193846126b2565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310610a53576040518061077787826128cd565b60026020600192604051610a6681612669565b73ffffffffffffffffffffffffffffffffffffffff8654168152610a8b858701612b1d565b83820152815201920192019190610a3e565b503461033f578060031936011261033f57602073ffffffffffffffffffffffffffffffffffffffff601f5460081c16604051908152f35b503461033f578060031936011261033f5760206040516201e2408152f35b503461033f578060031936011261033f578073ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610f0c575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152600160048201526001602482015281604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610ef7575b5073ffffffffffffffffffffffffffffffffffffffff601f5460081c166004604051917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08480a373ffffffffffffffffffffffffffffffffffffffff60225416803b15610ef357816024818580947ff2fde38b0000000000000000000000000000000000000000000000000000000083526004808401525af1801561034257610ede575b506004602073ffffffffffffffffffffffffffffffffffffffff60225416604051928380927f8da5cb5b0000000000000000000000000000000000000000000000000000000082525afa908115610342578291610eaf575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d5773ffffffffffffffffffffffffffffffffffffffff604051917f515361f6000000000000000000000000000000000000000000000000000000008352166004820152600460248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561034257610e9a575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610e85575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517f06447d56000000000000000000000000000000000000000000000000000000008152600480820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610682575050610421612ded565b81610e8f916126b2565b61033f57805f610e0a565b81610ea4916126b2565b61033f57805f610d9e565b610ed1915060203d602011610ed7575b610ec981836126b2565b810190612dc1565b5f610d12565b503d610ebf565b81610ee8916126b2565b61033f57805f610cba565b5050fd5b81610f01916126b2565b61033f57805f610c16565b81610f16916126b2565b61033f57805f610b8f565b503461033f578060031936011261033f57601a54610f3e81612a02565b91610f4c60405193846126b2565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b838310610f8e57604051806107778782612850565b600160208192610f9d85612a1a565b815201920192019190610f79565b503461033f578060031936011261033f5760206040516112348152f35b503461033f578060031936011261033f57610777610fe4612815565b60405191829160208352602083019061272e565b503461033f578060031936011261033f57601b5461101581612a02565b61102260405191826126b2565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b8383106110fa57868587604051928392602084019060208552518091526040840160408260051b8601019392905b82821061108f57505050500390f35b919360206110ea827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc06001959799849503018652885190836110da835160408452604084019061272e565b92015190848184039101526127c0565b9601920192018594939192611080565b6002602060019260405161110d81612669565b61111686612a1a565b8152611123858701612b1d565b83820152815201920192019190611052565b503461033f578060031936011261033f57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517fca669fa70000000000000000000000000000000000000000000000000000000081526103e76004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257611301575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f576040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f43616c6c6572206973206e6f7420746865206f776e657200000000000000000060448201528190818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610342576112ec575b5073ffffffffffffffffffffffffffffffffffffffff60225416803b1561034d578180916064604051809481937f2908035600000000000000000000000000000000000000000000000000000000835260206004840152601b60248401527f68747470733a2f2f6e65772d6578616d706c652e636f6d2f727063000000000060448401525af180156103425761032e5750f35b816112f6916126b2565b61033f57805f611259565b8161130b916126b2565b61033f57805f6111b8565b503461033f578060031936011261033f578073ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103425761164f575b505080604080516113c482826126b2565b602081527f68747470733a2f2f6e65772d6578616d706c652e636f6d2f6578706c6f7265726020820152737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ef35781517f491cc7c200000000000000000000000000000000000000000000000000000000815283818061145560048201906001606060808401938281525f60208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156116135790849161163a575b50507f02585ebce918f656604dabb23332a6df1dcde119741f7c1f8fb37b191392a2a7825160208152806114b4602082018561272e565b0390a173ffffffffffffffffffffffffffffffffffffffff60225416803b156116355783835180927f4b8be3f700000000000000000000000000000000000000000000000000000000825260206004830152818381611516602482018961272e565b03925af1801561161357908491611620575b5060049073ffffffffffffffffffffffffffffffffffffffff602254168451928380927f0743bf6e0000000000000000000000000000000000000000000000000000000082525afa9081156116135790611589929185916115ff5750612f01565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d578051907f90c5013b000000000000000000000000000000000000000000000000000000008252828260048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156115f6575061032e5750f35b513d84823e3d90fd5b61064391503d8087833e61063b81836126b2565b50505051903d90823e3d90fd5b8161162a916126b2565b610ef357825f611528565b505050fd5b81611644916126b2565b610ef357825f61147d565b81611659916126b2565b61033f57805f6113b3565b503461033f578060031936011261033f576020604051600a8152f35b503461033f578060031936011261033f57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517fca669fa70000000000000000000000000000000000000000000000000000000081526103e76004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103425761184c575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f576040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f43616c6c6572206973206e6f7420746865206f776e657200000000000000000060448201528190818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257611837575b5073ffffffffffffffffffffffffffffffffffffffff60225416803b1561034d578180916064604051809481937f4b8be3f700000000000000000000000000000000000000000000000000000000835260206004840152602060248401527f68747470733a2f2f6e65772d6578616d706c652e636f6d2f6578706c6f72657260448401525af180156103425761032e5750f35b81611841916126b2565b61033f57805f6117a4565b81611856916126b2565b61033f57805f611703565b503461033f578060031936011261033f578073ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257611a0c575b5073ffffffffffffffffffffffffffffffffffffffff60225416803b1561034d578160405180927f4b8be3f7000000000000000000000000000000000000000000000000000000008252818381604060048201602081525f60208201520103925af18015610342576119f7575b5060049073ffffffffffffffffffffffffffffffffffffffff60225416604051928380927f0743bf6e0000000000000000000000000000000000000000000000000000000082525afa8015610342576105bb9183916119dd575b50604051906119d56020836126b2565b838252612f01565b6119f191503d8085833e61063b81836126b2565b5f6119c5565b81611a01916126b2565b61033f57805f61196b565b81611a16916126b2565b61033f57805f6118fe565b503461033f578060031936011261033f5760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110611a80576107778561076b818703826126b2565b825473ffffffffffffffffffffffffffffffffffffffff16845260209093019260019283019201611a69565b503461033f578060031936011261033f5760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b818110611b0b576107778561076b818703826126b2565b825473ffffffffffffffffffffffffffffffffffffffff16845260209093019260019283019201611af4565b503461033f578060031936011261033f578073ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257611d6f575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4e6577206f776e65722063616e6e6f74206265207a65726f20616464726573736044820152818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257611d5a575b5073ffffffffffffffffffffffffffffffffffffffff60225416803b1561034d578180916024604051809481937ff2fde38b0000000000000000000000000000000000000000000000000000000083528160048401525af1801561034257611d45575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103425761032e5750f35b81611d4f916126b2565b61033f57805f611cd7565b81611d64916126b2565b61033f57805f611c74565b81611d79916126b2565b61033f57805f611bd4565b503461033f578060031936011261033f57601e54611da181612a02565b611dae60405191826126b2565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b838310611efc5786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310611e1a5786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc09086929496030183528551906020604082019273ffffffffffffffffffffffffffffffffffffffff81511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110611eb357505050505060208060019297019301930190928695949293611e0d565b9091929394602080611eef837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08760019603018952895161272e565b9701950193929101611e8f565b604051611f0881612669565b73ffffffffffffffffffffffffffffffffffffffff8354168152600183018054611f3181612a02565b91611f3f60405193846126b2565b8183528a526020808b20908b9084015b838210611f75575050505060019282602092836002950152815201920192019190611dde565b600160208192611f8486612a1a565b815201930191019091611f4f565b503461033f578060031936011261033f578073ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103425761211a575b5073ffffffffffffffffffffffffffffffffffffffff60225416803b1561034d578160405180927f29080356000000000000000000000000000000000000000000000000000000008252818381604060048201602081525f60208201520103925af1801561034257612105575b5060049073ffffffffffffffffffffffffffffffffffffffff60225416604051928380927fc7a760950000000000000000000000000000000000000000000000000000000082525afa8015610342576105bb9183916119dd5750604051906119d56020836126b2565b8161210f916126b2565b61033f57805f61209c565b81612124916126b2565b61033f57805f61202f565b503461033f578060031936011261033f5760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b81811061218e576107778561076b818703826126b2565b825473ffffffffffffffffffffffffffffffffffffffff16845260209093019260019283019201612177565b503461264a575f60031936011261264a5773ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561264a57604051907f06447d5600000000000000000000000000000000000000000000000000000000825260048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561263f5761262c575b5073ffffffffffffffffffffffffffffffffffffffff601f5460081c1660405190612e97908183019183831067ffffffffffffffff8411176125ff57918391602093612fa38439815203019082f080156125f257737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561066957816040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610342576125dd575b505073ffffffffffffffffffffffffffffffffffffffff16807fffffffffffffffffffffffff000000000000000000000000000000000000000060215416176021558173ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561066957604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610342576125c8575b505060206124c09173ffffffffffffffffffffffffffffffffffffffff601f5460081c169073ffffffffffffffffffffffffffffffffffffffff83541685612428612815565b936124d36124346126f3565b604051988997889687957f6f04245500000000000000000000000000000000000000000000000000000000875260048701526201e24060248701526209fbf1604487015261123460648701526156786084870152600a60a4870152606460c4870152619abc60e487015260c861010487015261012486015261018061014486015261018485019061272e565b906003198483030161016485015261272e565b03925af19081156103425782916125a9575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561066957816040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257612594575b5073ffffffffffffffffffffffffffffffffffffffff82167fffffffffffffffffffffffff0000000000000000000000000000000000000000602254161760225580f35b8161259e916126b2565b61066957815f612550565b6125c2915060203d602011610ed757610ec981836126b2565b5f6124e5565b816125d2916126b2565b61066957815f6123e2565b816125e7916126b2565b61066957815f612314565b50604051903d90823e3d90fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b61263891505f906126b2565b5f5f612256565b6040513d5f823e3d90fd5b5f80fd5b3461264a575f60031936011261264a57610777610fe46126f3565b6040810190811067ffffffffffffffff82111761268557604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761268557604052565b604051906127026040836126b2565b601c82527f68747470733a2f2f6578616d706c652e636f6d2f6578706c6f726572000000006020830152565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b60206040818301928281528451809452019201905f5b8181106127945750505090565b825173ffffffffffffffffffffffffffffffffffffffff16845260209384019390920191600101612787565b90602080835192838152019201905f5b8181106127dd5750505090565b82517fffffffff00000000000000000000000000000000000000000000000000000000168452602093840193909201916001016127d0565b604051906128246040836126b2565b601782527f68747470733a2f2f6578616d706c652e636f6d2f7270630000000000000000006020830152565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061288257505050505090565b90919293946020806128be837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08660019603018752895161272e565b97019301930191939290612873565b602081016020825282518091526040820191602060408360051b8301019401925f915b8383106128ff57505050505090565b9091929394602080612962837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b5173ffffffffffffffffffffffffffffffffffffffff8151168452015191818582015201906127c0565b970193019301919392906128f0565b60208183031261264a5780519067ffffffffffffffff821161264a570181601f8201121561264a5780519067ffffffffffffffff821161268557604051926129e1601f84017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016602001856126b2565b8284526020838301011161264a57815f9260208093018386015e8301015290565b67ffffffffffffffff81116126855760051b60200190565b90604051915f8154908160011c9260018316928315612b13575b602085108414612ae6578487528693908115612aa65750600114612a62575b50612a60925003836126b2565b565b90505f9291925260205f20905f915b818310612a8a575050906020612a60928201015f612a53565b6020919350806001915483858901015201910190918492612a71565b60209350612a609592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f612a53565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f1693612a34565b90604051918281549182825260208201905f5260205f20925f905b806007830110612d3457612a60945491818110612cfe575b818110612cc8575b818110612c92575b818110612c5c575b818110612c26575b818110612bf0575b818110612bbb575b10612b8e575b5003836126b2565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f612b86565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b168152019301612b80565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301612b78565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b168152019301612b70565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301612b68565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b168152019301612b60565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b168152019301612b58565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b168152019301612b50565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e0820152019401920185929391612b38565b9081602091031261264a575173ffffffffffffffffffffffffffffffffffffffff8116810361264a5790565b60405190612dfc6040836126b2565b601b82527f68747470733a2f2f6e65772d6578616d706c652e636f6d2f72706300000000006020830152565b60085460ff168015612e375790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561263f575f91612ecf575b50151590565b90506020813d602011612ef9575b81612eea602093836126b2565b8101031261264a57515f612ec9565b3d9150612edd565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561264a57612f605f91612f7260405194859384937ff320d96300000000000000000000000000000000000000000000000000000000855260406004860152604485019061272e565b9060031984830301602485015261272e565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561263f57612f985750565b5f612a60916126b256fe60a03461016b57601f612e9738819003918201601f19168301916001600160401b038311848410176101445780849260209460405283398101031261016b57516001600160a01b0381169081900361016b578015610158575f80546001600160a01b031981168317825560405192916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a36116de8181016001600160401b038111838210176101445782916112b4833903905ff0801561013957604051906105058083016001600160401b0381118482101761014457604092849261299284396001600160a01b031681523060208201520301905ff080156101395760805260405161114490816101708239608051818181610215015281816105c2015281816108610152610a980152f35b6040513d5f823e3d90fd5b634e487b7160e01b5f52604160045260245ffd5b631e4fbdf760e01b5f525f60045260245ffd5b5f80fdfe60806040526004361015610011575f80fd5b5f5f3560e01c80630b04ebfd1461088557806359659e90146108355780636f04245514610457578063715018a6146103d957806383f94db7146101b75780638da5cb5b14610184578063a33a8b60146101445763f2fde38b14610072575f80fd5b346101415760206003193601126101415773ffffffffffffffffffffffffffffffffffffffff6100a06108c1565b6100a8610c20565b1680156101155773ffffffffffffffffffffffffffffffffffffffff8254827fffffffffffffffffffffffff00000000000000000000000000000000000000008216178455167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08380a380f35b6024827f1e4fbdf700000000000000000000000000000000000000000000000000000000815280600452fd5b80fd5b50346101415760206003193601126101415773ffffffffffffffffffffffffffffffffffffffff6040602092600435815260018452205416604051908152f35b503461014157806003193601126101415773ffffffffffffffffffffffffffffffffffffffff6020915416604051908152f35b5034610141576020600319360112610141576101d16108c1565b6101d9610c20565b73ffffffffffffffffffffffffffffffffffffffff8116908115610355573b156102d1578173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b156102c2578180916024604051809481937f3659cfe60000000000000000000000000000000000000000000000000000000083528860048401525af180156102c6576102ad575b507f51ea6ffdc9909d5ca341259f7221902e0676585d833e2bb21fa923c85e862886602083604051908152a180f35b816102b7916108e4565b6102c257815f61027e565b5080fd5b6040513d84823e3d90fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602160248201527f496d706c656d656e746174696f6e206d757374206265206120636f6e7472616360448201527f74000000000000000000000000000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602960248201527f4e657720696d706c656d656e746174696f6e2063616e6e6f74206265207a657260448201527f6f206164647265737300000000000000000000000000000000000000000000006064820152fd5b50346101415780600319360112610141576103f2610c20565b8073ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b3461078057610180600319360112610780576104716108c1565b602435906064359173ffffffffffffffffffffffffffffffffffffffff8316809303610780576084359173ffffffffffffffffffffffffffffffffffffffff83168093036107805760e4359073ffffffffffffffffffffffffffffffffffffffff821680920361078057610124359073ffffffffffffffffffffffffffffffffffffffff8216809203610780576101443567ffffffffffffffff81116107805761051f903690600401610925565b926101643567ffffffffffffffff811161078057610541903690600401610925565b610549610c20565b610554861515610999565b855f52600160205273ffffffffffffffffffffffffffffffffffffffff60405f2054166107b1576040516020810190878252602081526105956040826108e4565b5190206040516104d78082019082821067ffffffffffffffff83111761078457829161060d91610c6d84397f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1681526040602082018190525f9082015260600190565b03905ff580156107755773ffffffffffffffffffffffffffffffffffffffff1696865f52600160205260405f20887fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055873b15610780575f9573ffffffffffffffffffffffffffffffffffffffff9561071e9461070b936040519c8d998a997fbf79fd1c000000000000000000000000000000000000000000000000000000008b521660048a01528b60248a015260443560448a01526064890152608488015260a43560a488015260c43560c488015260e487015261010435610104870152610124860152610180610144860152610184850190610bdd565b9060031984830301610164850152610bdd565b038183865af192831561077557602093610765575b507feaf2b9d4fd6eba5a60870499f6335c6ab4826e029aff65ba0619329dbd421ec383604051848152a2604051908152f35b5f61076f916108e4565b5f610733565b6040513d5f823e3d90fd5b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602760248201527f436f6e66696720616c72656164792065786973747320666f722074686973206360448201527f6861696e204944000000000000000000000000000000000000000000000000006064820152fd5b34610780575f60031936011261078057602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346107805760206003193601126107805760206108a36004356109fe565b73ffffffffffffffffffffffffffffffffffffffff60405191168152f35b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361078057565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761078457604052565b81601f820112156107805780359067ffffffffffffffff8211610784576040519261097860207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f86011601856108e4565b8284526020838301011161078057815f926020809301838601378301015290565b156109a057565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f436861696e2049442063616e6e6f74206265207a65726f0000000000000000006044820152fd5b805f52600160205273ffffffffffffffffffffffffffffffffffffffff60405f205416610bb85780610a4673ffffffffffffffffffffffffffffffffffffffff921515610999565b6040516020810191825260208152610a5f6040826108e4565b5190206040516104d7610a7560208201836108e4565b8082526020820190610c6d8239610b60604051916020808401610b1185610ae58a7f0000000000000000000000000000000000000000000000000000000000000000168473ffffffffffffffffffffffffffffffffffffffff606092168152604060208201525f60408201520190565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081018752866108e4565b60405194859383850197518091895e840190838201905f8252519283915e01015f8152037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081018352826108e4565b5190206040519060208201927fff0000000000000000000000000000000000000000000000000000000000000084523060601b60218401526035830152605582015260558152610bb16075826108e4565b5190201690565b5f52600160205273ffffffffffffffffffffffffffffffffffffffff60405f20541690565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b73ffffffffffffffffffffffffffffffffffffffff5f54163303610c4057565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffdfe60a0806040526104d780380380916100178285610292565b833981016040828203126101eb5761002e826102c9565b602083015190926001600160401b0382116101eb57019080601f830112156101eb57815161005b816102dd565b926100696040519485610292565b8184526020840192602083830101116101eb57815f926020809301855e84010152823b15610274577fa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d5080546001600160a01b0319166001600160a01b038516908117909155604051635c60da1b60e01b8152909190602081600481865afa9081156101f7575f9161023a575b50803b1561021a5750817f1cf3b03a6cf19fa2baba4df148e9dcabedea7f8a5c07840e207e5c089be95d3e5f80a282511561020257602060049260405193848092635c60da1b60e01b82525afa9182156101f7575f926101ae575b505f809161018a945190845af43d156101a6573d9161016e836102dd565b9261017c6040519485610292565b83523d5f602085013e6102f8565b505b608052604051610180908161035782396080518160460152f35b6060916102f8565b9291506020833d6020116101ef575b816101ca60209383610292565b810103126101eb575f80916101e161018a956102c9565b9394509150610150565b5f80fd5b3d91506101bd565b6040513d5f823e3d90fd5b505050341561018c5763b398979f60e01b5f5260045ffd5b634c9c8ce360e01b5f9081526001600160a01b0391909116600452602490fd5b90506020813d60201161026c575b8161025560209383610292565b810103126101eb57610266906102c9565b5f6100f5565b3d9150610248565b631933b43b60e21b5f9081526001600160a01b038416600452602490fd5b601f909101601f19168101906001600160401b038211908210176102b557604052565b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036101eb57565b6001600160401b0381116102b557601f01601f191660200190565b9061031c575080511561030d57805190602001fd5b63d6bda27560e01b5f5260045ffd5b8151158061034d575b61032d575090565b639996b31560e01b5f9081526001600160a01b0391909116600452602490fd5b50803b1561032556fe60806040527f5c60da1b000000000000000000000000000000000000000000000000000000006080526020608060048173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa8015610107575f9015610163575060203d602011610100575b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f820116608001906080821067ffffffffffffffff8311176100d3576100ce91604052608001610112565b610163565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b503d610081565b6040513d5f823e3d90fd5b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80602091011261015f5760805173ffffffffffffffffffffffffffffffffffffffff8116810361015f5790565b5f80fd5b5f8091368280378136915af43d5f803e1561017c573d5ff35b3d5ffd6080806040523460aa575f5160206116be5f395f51905f525460ff8160401c16609b576002600160401b03196001600160401b038216016049575b60405161160f90816100af8239f35b6001600160401b0319166001600160401b039081175f5160206116be5f395f51905f525581527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602090a15f80603a565b63f92ee8a960e01b5f5260045ffd5b5f80fdfe60806040526004361015610011575f80fd5b5f3560e01c80630743bf6e1461120357806318b5ce81146111d057806329080356146110845780634b8be3f714610f2657806354fd4d5014610e8157806357d1ba2514610e645780636edd6c0914610e315780637240f9af14610cd157806385e1f4d014610cb45780638da5cb5b14610c82578063a3c6e1e714610c65578063aa6a43d814610c32578063bf6db6f814610bff578063bf79fd1c1461027b578063c7a7609514610183578063d1f4737c14610166578063f2fde38b146101005763f8a144be146100df575f80fd5b346100fc575f6003193601126100fc576020600654604051908152f35b5f80fd5b346100fc5760206003193601126100fc5761016461011c611397565b61013e73ffffffffffffffffffffffffffffffffffffffff5f54163314611410565b61015f73ffffffffffffffffffffffffffffffffffffffff82161515611524565b611589565b005b346100fc575f6003193601126100fc576020600854604051908152f35b346100fc575f6003193601126100fc576040515f600a546101a3816112a8565b808452906001811690811561023957506001146101db575b6101d7836101cb818503826112f9565b6040519182918261131c565b0390f35b600a5f9081527fc65a7bb8d6351c1cf70c95a316cc6a92839c986682d98bc35f958f4883f9d2a8939250905b80821061021f575090915081016020016101cb6101bb565b919260018160209254838588010152019101909291610207565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208086019190915291151560051b840190910191506101cb90506101bb565b346100fc576101806003193601126100fc57610295611397565b602435604435916064359273ffffffffffffffffffffffffffffffffffffffff84168094036100fc5760843573ffffffffffffffffffffffffffffffffffffffff81168091036100fc5760e43573ffffffffffffffffffffffffffffffffffffffff81168091036100fc57610124359173ffffffffffffffffffffffffffffffffffffffff83168093036100fc576101443567ffffffffffffffff81116100fc576103449036906004016113ba565b966101643567ffffffffffffffff81116100fc576103669036906004016113ba565b947ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00549760ff8960401c16159867ffffffffffffffff811680159081610bf7575b6001149081610bed575b159081610be4575b50610bbc578960017fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000008316177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0055610b67575b5073ffffffffffffffffffffffffffffffffffffffff881615610b09578015610aab578115610a275782156109a357831561091f57841561089b578515610817576005556006557fffffffffffffffffffffffff000000000000000000000000000000000000000060025416176002557fffffffffffffffffffffffff0000000000000000000000000000000000000000600354161760035560a43560075560c4356008557fffffffffffffffffffffffff00000000000000000000000000000000000000006004541617600455610104356009556104ec600c546112a8565b601f81116107d9575b50600a7f312e302e3000000000000000000000000000000000000000000000000000000001600c557fffffffffffffffffffffffff00000000000000000000000000000000000000006001541617600155835167ffffffffffffffff811161072b5761056b81610566600a546112a8565b61148b565b602094601f82116001146107585761059c9293949582915f92610699575b50505f198260011b9260031b1c19161790565b600a555b80519067ffffffffffffffff821161072b576105c6826105c1600b546112a8565b6114c5565b602090601f83116001146106a45791806105f89261060095945f926106995750505f198260011b9260031b1c19161790565b600b55611589565b61060657005b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160018152a1005b015190508680610589565b90601f19831691600b5f527f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9925f5b8181106107135750916001939185610600979694106106fb575b505050811b01600b55611589565b01515f1960f88460031b161c191690558580806106ed565b929360206001819287860151815501950193016106d3565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b601f19821695600a5f527fc65a7bb8d6351c1cf70c95a316cc6a92839c986682d98bc35f958f4883f9d2a8915f5b8881106107c1575083600195969798106107a9575b505050811b01600a556105a0565b01515f1960f88460031b161c1916905585808061079b565b91926020600181928685015181550194019201610786565b600c5f5261081190601f0160051c7fdf6966c971051c3d54ec59162606531493a51404a002842f56009d7e5cf4a8c790810190611475565b856104f5565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602d60248201527f496e697469616c20617070636861696e206f776e65722063616e6e6f7420626560448201527f207a65726f2061646472657373000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602a60248201527f53657175656e63696e6720636f6e747261637420616464726573732063616e6e60448201527f6f74206265207a65726f000000000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602560248201527f417262697472756d20696e626f7820616464726573732063616e6e6f7420626560448201527f207a65726f0000000000000000000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f417262697472756d2062726964676520616464726573732063616e6e6f74206260448201527f65207a65726f00000000000000000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f53657175656e63696e6720636861696e2049442063616e6e6f74206265207a6560448201527f726f0000000000000000000000000000000000000000000000000000000000006064820152fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f436861696e2049442063616e6e6f74206265207a65726f0000000000000000006044820152fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601c60248201527f4f776e65722063616e6e6f74206265207a65726f2061646472657373000000006044820152fd5b7fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000001668010000000000000001177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00558a61040c565b7ff92ee8a9000000000000000000000000000000000000000000000000000000005f5260045ffd5b9050158c6103b9565b303b1591506103b1565b8b91506103a7565b346100fc575f6003193601126100fc57602073ffffffffffffffffffffffffffffffffffffffff60045416604051908152f35b346100fc575f6003193601126100fc57602073ffffffffffffffffffffffffffffffffffffffff60035416604051908152f35b346100fc575f6003193601126100fc576020600954604051908152f35b346100fc575f6003193601126100fc57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b346100fc575f6003193601126100fc576020600554604051908152f35b346100fc57610cdf36611346565b610d0173ffffffffffffffffffffffffffffffffffffffff5f54163314611410565b67ffffffffffffffff811161072b57610d1b600c546112a8565b601f8111610dd8575b505f601f8211600114610d60578190610d50935f92610d555750505f198260011b9260031b1c19161790565b600c55005b013590508380610589565b601f198216927fdf6966c971051c3d54ec59162606531493a51404a002842f56009d7e5cf4a8c7915f5b858110610dc057508360019510610da7575b505050811b01600c55005b5f1960f88560031b161c19910135169055828080610d9c565b90926020600181928686013581550194019101610d8a565b600c5f52610e21907fdf6966c971051c3d54ec59162606531493a51404a002842f56009d7e5cf4a8c7601f840160051c81019160208510610e27575b601f0160051c0190611475565b82610d24565b9091508190610e14565b346100fc575f6003193601126100fc57602073ffffffffffffffffffffffffffffffffffffffff60025416604051908152f35b346100fc575f6003193601126100fc576020600754604051908152f35b346100fc575f6003193601126100fc576040515f600c54610ea1816112a8565b80845290600181169081156102395750600114610ec8576101d7836101cb818503826112f9565b600c5f9081527fdf6966c971051c3d54ec59162606531493a51404a002842f56009d7e5cf4a8c7939250905b808210610f0c575090915081016020016101cb6101bb565b919260018160209254838588010152019101909291610ef4565b346100fc57610f3436611346565b610f5673ffffffffffffffffffffffffffffffffffffffff5f54163314611410565b67ffffffffffffffff811161072b57610f74816105c1600b546112a8565b5f91601f8211600114610fe457610fc182807f02585ebce918f656604dabb23332a6df1dcde119741f7c1f8fb37b191392a2a7955f91610fd9575b505f198260011b9260031b1c19161790565b600b555b610fd4604051928392836114fd565b0390a1005b905083013586610faf565b601f198216600b5f527f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9905f5b81811061106c575093837f02585ebce918f656604dabb23332a6df1dcde119741f7c1f8fb37b191392a2a79510611053575b5050600182811b01600b55610fc5565b5f1960f88560031b161c19908301351690558380611043565b83860135835560209586019560019093019201611011565b346100fc5761109236611346565b6110b473ffffffffffffffffffffffffffffffffffffffff5f54163314611410565b67ffffffffffffffff811161072b576110d281610566600a546112a8565b5f91601f82116001146111305761111e82807f67c57ae6ad924cd093fb2f06bc0b28fd5879481051a9c203a44d27c8904d437c955f91610fd957505f198260011b9260031b1c19161790565b600a55610fd4604051928392836114fd565b601f198216600a5f527fc65a7bb8d6351c1cf70c95a316cc6a92839c986682d98bc35f958f4883f9d2a8905f5b8181106111b8575093837f67c57ae6ad924cd093fb2f06bc0b28fd5879481051a9c203a44d27c8904d437c951061119f575b5050600182811b01600a55610fc5565b5f1960f88560031b161c1990830135169055838061118f565b8386013583556020958601956001909301920161115d565b346100fc575f6003193601126100fc57602073ffffffffffffffffffffffffffffffffffffffff60015416604051908152f35b346100fc575f6003193601126100fc576040515f600b54611223816112a8565b8084529060018116908115610239575060011461124a576101d7836101cb818503826112f9565b600b5f9081527f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9939250905b80821061128e575090915081016020016101cb6101bb565b919260018160209254838588010152019101909291611276565b90600182811c921680156112ef575b60208310146112c257565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f16916112b7565b90601f601f19910116810190811067ffffffffffffffff82111761072b57604052565b601f19601f602060409481855280519182918282880152018686015e5f8582860101520116010190565b9060206003198301126100fc5760043567ffffffffffffffff81116100fc57826023820112156100fc5780600401359267ffffffffffffffff84116100fc57602484830101116100fc576024019190565b6004359073ffffffffffffffffffffffffffffffffffffffff821682036100fc57565b81601f820112156100fc5780359067ffffffffffffffff821161072b57604051926113ef6020601f19601f86011601856112f9565b828452602083830101116100fc57815f926020809301838601378301015290565b1561141757565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f43616c6c6572206973206e6f7420746865206f776e65720000000000000000006044820152fd5b818110611480575050565b5f8155600101611475565b90601f8211611498575050565b6114c391600a5f5260205f20906020601f840160051c83019310610e2757601f0160051c0190611475565b565b90601f82116114d2575050565b6114c391600b5f5260205f20906020601f840160051c83019310610e2757601f0160051c0190611475565b90601f83604094601f199360208652816020870152868601375f8582860101520116010190565b1561152b57565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4e6577206f776e65722063616e6e6f74206265207a65726f20616464726573736044820152fd5b73ffffffffffffffffffffffffffffffffffffffff166115aa811515611524565b73ffffffffffffffffffffffffffffffffffffffff5f54827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a356f0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0060803461013457601f61050538819003918201601f19168301916001600160401b03831184841017610138578084926040948552833981010312610134576100468161014c565b906001600160a01b039061005c9060200161014c565b16908115610121575f80546001600160a01b031981168417825560405193916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a3803b1561010157600180546001600160a01b0319166001600160a01b039290921691821790557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a26103a490816101618239f35b63211eb15960e21b5f9081526001600160a01b0391909116600452602490fd5b631e4fbdf760e01b5f525f60045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036101345756fe60806040526004361015610011575f80fd5b5f3560e01c80633659cfe61461027e5780635c60da1b1461022d578063715018a6146101935780638da5cb5b146101435763f2fde38b14610050575f80fd5b3461013f5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261013f5760043573ffffffffffffffffffffffffffffffffffffffff811680910361013f576100a8610358565b80156101135773ffffffffffffffffffffffffffffffffffffffff5f54827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f80fd5b3461013f575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261013f57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b3461013f575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261013f576101c9610358565b5f73ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461013f575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261013f57602073ffffffffffffffffffffffffffffffffffffffff60015416604051908152f35b3461013f5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261013f5760043573ffffffffffffffffffffffffffffffffffffffff81169081810361013f576102d7610358565b3b1561032d57807fffffffffffffffffffffffff000000000000000000000000000000000000000060015416176001557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a2005b7f847ac564000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff5f5416330361037857565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`IW`\x0C\x80T`\xFF\x19\x16`\x01\x17\x90U`\x1F\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16a\x01\x01\x17\x90U` \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x02\x17\x90Ua^:\x90\x81a\0N\x829\xF3[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x07C\xBFn\x14a&NW\x80c\n\x92T\xE4\x14a!\xBAW\x80c\x1E\xD7\x83\x1C\x14a!/W\x80c&\xDFN%\x14a\x1F\x92W\x80c*\xDE8\x80\x14a\x1D\x84W\x80c4xK\xF1\x14a\x1B7W\x80c>^<#\x14a\x1A\xACW\x80c?r\x86\xF4\x14a\x1A!W\x80cM\xBD\xC8]\x14a\x18aW\x80cR\x06\x9F\xC2\x14a\x16\x80W\x80cW\xD1\xBA%\x14a\x16dW\x80c[|\x15\x99\x14a\x13\x16W\x80c`\x94L\xE6\x14a\x115W\x80cf\xD9\xA9\xA0\x14a\x0F\xF8W\x80ch\x06\xBA\x06\x14a\x0F\xC8W\x80cn\xDDl\t\x14a\x0F\xABW\x80c\x85\"l\x81\x14a\x0F!W\x80c\x85\"\xAC\x8C\x14a\n\xF2W\x80c\x85\xE1\xF4\xD0\x14a\n\xD4W\x80c\x8D\xA5\xCB[\x14a\n\x9DW\x80c\x91j\x17\xC6\x14a\t\xE6W\x80c\xA3\xC6\xE1\xE7\x14a\t\xCAW\x80c\xAAjC\xD8\x14a\t\xADW\x80c\xB0FO\xDC\x14a\x08\xF6W\x80c\xB5P\x8A\xA9\x14a\x08lW\x80c\xBAAO\xA6\x14a\x08GW\x80c\xBFm\xB6\xF8\x14a\x08*W\x80c\xCA\n\xB0u\x14a\x07\xF6W\x80c\xD1\xF4s|\x14a\x07\xDAW\x80c\xD81\x97^\x14a\x07\xA7W\x80c\xE2\x0C\x9Fq\x14a\x07\x0CW\x80c\xEB\xB478\x14a\x06\xD8W\x80c\xF8\xA1D\xBE\x14a\x06\xBAW\x80c\xFAv&\xD4\x14a\x06\x97W\x80c\xFD\xFC\x99\xDF\x14a\x03zWc\xFE\xBCD2\x14a\x01\xA6W_\x80\xFD[4a\x03?W\x80`\x03\x196\x01\x12a\x03?Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x03\xE7`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x03eW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`D\x82\x01R\x81\x90\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x03PW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x03MW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xF2\xFD\xE3\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x80\x84\x01RZ\xF1\x80\x15a\x03BWa\x03.WP\xF3[\x81a\x038\x91a&\xB2V[a\x03?W\x80\xF3[\x80\xFD[`@Q=\x84\x82>=\x90\xFD[P\xFD[\x81a\x03Z\x91a&\xB2V[a\x03?W\x80_a\x02\xC9V[\x81a\x03o\x91a&\xB2V[a\x03?W\x80_a\x02(V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x06\x82W[PPa\x04!a-\xEDV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06iW\x81`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a\x04\x8A`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R_` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x06mW[PP\x7Fg\xC5z\xE6\xAD\x92L\xD0\x93\xFB/\x06\xBC\x0B(\xFDXyH\x10Q\xA9\xC2\x03\xA4M'\xC8\x90MC|`@Q` \x81R\x80a\x04\xE7` \x82\x01\x85a'.V[\x03\x90\xA1\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x06iW\x81`@Q\x80\x92\x7F)\x08\x03V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` `\x04\x83\x01R\x81\x83\x81a\x05K`$\x82\x01\x8Aa'.V[\x03\x92Z\xF1\x80\x15a\x03BWa\x06TW[P\x90`\x04\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16`@Q\x93\x84\x80\x92\x7F\xC7\xA7`\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06IWa\x05\xBB\x92\x84\x91a\x06'W[Pa/\x01V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x03.WP\xF3[a\x06C\x91P=\x80\x86\x83>a\x06;\x81\x83a&\xB2V[\x81\x01\x90a)qV[_a\x05\xB5V[`@Q=\x85\x82>=\x90\xFD[\x81a\x06^\x91a&\xB2V[a\x06iW\x81_a\x05ZV[P\x80\xFD[\x81a\x06w\x91a&\xB2V[a\x06iW\x81_a\x04\xAFV[\x81a\x06\x8C\x91a&\xB2V[a\x03?W\x80_a\x04\x17V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Qb\t\xFB\xF1\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16`@Q\x90\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x07{Wa\x07w\x85a\x07k\x81\x87\x03\x82a&\xB2V[`@Q\x91\x82\x91\x82a'qV[\x03\x90\xF3[\x82Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x07TV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x16`@Q\x90\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Q`d\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`!T\x16`@Q\x90\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Qa\x9A\xBC\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` a\x08ba.(V[`@Q\x90\x15\x15\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`\x19Ta\x08\x89\x81a*\x02V[\x91a\x08\x97`@Q\x93\x84a&\xB2V[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\x08\xD9W`@Q\x80a\x07w\x87\x82a(PV[`\x01` \x81\x92a\x08\xE8\x85a*\x1AV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x08\xC4V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`\x1CTa\t\x13\x81a*\x02V[\x91a\t!`@Q\x93\x84a&\xB2V[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\tcW`@Q\x80a\x07w\x87\x82a(\xCDV[`\x02` `\x01\x92`@Qa\tv\x81a&iV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86T\x16\x81Ra\t\x9B\x85\x87\x01a+\x1DV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\tNV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@QaVx\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Q`\xC8\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`\x1DTa\n\x03\x81a*\x02V[\x91a\n\x11`@Q\x93\x84a&\xB2V[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\nSW`@Q\x80a\x07w\x87\x82a(\xCDV[`\x02` `\x01\x92`@Qa\nf\x81a&iV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86T\x16\x81Ra\n\x8B\x85\x87\x01a+\x1DV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\n>V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Qb\x01\xE2@\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x0F\x0CW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R`\x01`$\x82\x01R\x81`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x0E\xF7W[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16`\x04`@Q\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x84\x80\xA3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x0E\xF3W\x81`$\x81\x85\x80\x94\x7F\xF2\xFD\xE3\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x80\x84\x01RZ\xF1\x80\x15a\x03BWa\x0E\xDEW[P`\x04` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16`@Q\x92\x83\x80\x92\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x03BW\x82\x91a\x0E\xAFW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01R`\x04`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03BWa\x0E\x9AW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x0E\x85W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x80\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x06\x82WPPa\x04!a-\xEDV[\x81a\x0E\x8F\x91a&\xB2V[a\x03?W\x80_a\x0E\nV[\x81a\x0E\xA4\x91a&\xB2V[a\x03?W\x80_a\r\x9EV[a\x0E\xD1\x91P` =` \x11a\x0E\xD7W[a\x0E\xC9\x81\x83a&\xB2V[\x81\x01\x90a-\xC1V[_a\r\x12V[P=a\x0E\xBFV[\x81a\x0E\xE8\x91a&\xB2V[a\x03?W\x80_a\x0C\xBAV[PP\xFD[\x81a\x0F\x01\x91a&\xB2V[a\x03?W\x80_a\x0C\x16V[\x81a\x0F\x16\x91a&\xB2V[a\x03?W\x80_a\x0B\x8FV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`\x1ATa\x0F>\x81a*\x02V[\x91a\x0FL`@Q\x93\x84a&\xB2V[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\x0F\x8EW`@Q\x80a\x07w\x87\x82a(PV[`\x01` \x81\x92a\x0F\x9D\x85a*\x1AV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x0FyV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Qa\x124\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?Wa\x07wa\x0F\xE4a(\x15V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a'.V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`\x1BTa\x10\x15\x81a*\x02V[a\x10\"`@Q\x91\x82a&\xB2V[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a\x10\xFAW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a\x10\x8FWPPPP\x03\x90\xF3[\x91\x93` a\x10\xEA\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a\x10\xDA\x83Q`@\x84R`@\x84\x01\x90a'.V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra'\xC0V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x10\x80V[`\x02` `\x01\x92`@Qa\x11\r\x81a&iV[a\x11\x16\x86a*\x1AV[\x81Ra\x11#\x85\x87\x01a+\x1DV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x10RV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x03\xE7`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x13\x01W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`D\x82\x01R\x81\x90\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x12\xECW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x03MW\x81\x80\x91`d`@Q\x80\x94\x81\x93\x7F)\x08\x03V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`\x1B`$\x84\x01R\x7Fhttps://new-example.com/rpc\0\0\0\0\0`D\x84\x01RZ\xF1\x80\x15a\x03BWa\x03.WP\xF3[\x81a\x12\xF6\x91a&\xB2V[a\x03?W\x80_a\x12YV[\x81a\x13\x0B\x91a&\xB2V[a\x03?W\x80_a\x11\xB8V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x16OW[PP\x80`@\x80Qa\x13\xC4\x82\x82a&\xB2V[` \x81R\x7Fhttps://new-example.com/explorer` \x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0E\xF3W\x81Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83\x81\x80a\x14U`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R_` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x16\x13W\x90\x84\x91a\x16:W[PP\x7F\x02X^\xBC\xE9\x18\xF6V`M\xAB\xB232\xA6\xDF\x1D\xCD\xE1\x19t\x1F|\x1F\x8F\xB3{\x19\x13\x92\xA2\xA7\x82Q` \x81R\x80a\x14\xB4` \x82\x01\x85a'.V[\x03\x90\xA1s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x165W\x83\x83Q\x80\x92\x7FK\x8B\xE3\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` `\x04\x83\x01R\x81\x83\x81a\x15\x16`$\x82\x01\x89a'.V[\x03\x92Z\xF1\x80\x15a\x16\x13W\x90\x84\x91a\x16 W[P`\x04\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x84Q\x92\x83\x80\x92\x7F\x07C\xBFn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x16\x13W\x90a\x15\x89\x92\x91\x85\x91a\x15\xFFWPa/\x01V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW\x80Q\x90\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x82\x82`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x15\xF6WPa\x03.WP\xF3[Q=\x84\x82>=\x90\xFD[a\x06C\x91P=\x80\x87\x83>a\x06;\x81\x83a&\xB2V[PPPQ\x90=\x90\x82>=\x90\xFD[\x81a\x16*\x91a&\xB2V[a\x0E\xF3W\x82_a\x15(V[PPP\xFD[\x81a\x16D\x91a&\xB2V[a\x0E\xF3W\x82_a\x14}V[\x81a\x16Y\x91a&\xB2V[a\x03?W\x80_a\x13\xB3V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Q`\n\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x03\xE7`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x18LW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`D\x82\x01R\x81\x90\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x187W[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x03MW\x81\x80\x91`d`@Q\x80\x94\x81\x93\x7FK\x8B\xE3\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R` `$\x84\x01R\x7Fhttps://new-example.com/explorer`D\x84\x01RZ\xF1\x80\x15a\x03BWa\x03.WP\xF3[\x81a\x18A\x91a&\xB2V[a\x03?W\x80_a\x17\xA4V[\x81a\x18V\x91a&\xB2V[a\x03?W\x80_a\x17\x03V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x1A\x0CW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x03MW\x81`@Q\x80\x92\x7FK\x8B\xE3\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x83\x81`@`\x04\x82\x01` \x81R_` \x82\x01R\x01\x03\x92Z\xF1\x80\x15a\x03BWa\x19\xF7W[P`\x04\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16`@Q\x92\x83\x80\x92\x7F\x07C\xBFn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03BWa\x05\xBB\x91\x83\x91a\x19\xDDW[P`@Q\x90a\x19\xD5` \x83a&\xB2V[\x83\x82Ra/\x01V[a\x19\xF1\x91P=\x80\x85\x83>a\x06;\x81\x83a&\xB2V[_a\x19\xC5V[\x81a\x1A\x01\x91a&\xB2V[a\x03?W\x80_a\x19kV[\x81a\x1A\x16\x91a&\xB2V[a\x03?W\x80_a\x18\xFEV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a\x1A\x80Wa\x07w\x85a\x07k\x81\x87\x03\x82a&\xB2V[\x82Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x1AiV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a\x1B\x0BWa\x07w\x85a\x07k\x81\x87\x03\x82a&\xB2V[\x82Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x1A\xF4V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x1DoW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FNew owner cannot be zero address`D\x82\x01R\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x1DZW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x03MW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xF2\xFD\xE3\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03BWa\x1DEW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x03.WP\xF3[\x81a\x1DO\x91a&\xB2V[a\x03?W\x80_a\x1C\xD7V[\x81a\x1Dd\x91a&\xB2V[a\x03?W\x80_a\x1CtV[\x81a\x1Dy\x91a&\xB2V[a\x03?W\x80_a\x1B\xD4V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`\x1ETa\x1D\xA1\x81a*\x02V[a\x1D\xAE`@Q\x91\x82a&\xB2V[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a\x1E\xFCW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a\x1E\x1AW\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a\x1E\xB3WPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a\x1E\rV[\x90\x91\x92\x93\x94` \x80a\x1E\xEF\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa'.V[\x97\x01\x95\x01\x93\x92\x91\x01a\x1E\x8FV[`@Qa\x1F\x08\x81a&iV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83T\x16\x81R`\x01\x83\x01\x80Ta\x1F1\x81a*\x02V[\x91a\x1F?`@Q\x93\x84a&\xB2V[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a\x1FuWPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x1D\xDEV[`\x01` \x81\x92a\x1F\x84\x86a*\x1AV[\x81R\x01\x93\x01\x91\x01\x90\x91a\x1FOV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa!\x1AW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x03MW\x81`@Q\x80\x92\x7F)\x08\x03V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x83\x81`@`\x04\x82\x01` \x81R_` \x82\x01R\x01\x03\x92Z\xF1\x80\x15a\x03BWa!\x05W[P`\x04\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16`@Q\x92\x83\x80\x92\x7F\xC7\xA7`\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03BWa\x05\xBB\x91\x83\x91a\x19\xDDWP`@Q\x90a\x19\xD5` \x83a&\xB2V[\x81a!\x0F\x91a&\xB2V[a\x03?W\x80_a \x9CV[\x81a!$\x91a&\xB2V[a\x03?W\x80_a /V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a!\x8EWa\x07w\x85a\x07k\x81\x87\x03\x82a&\xB2V[\x82Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a!wV[P4a&JW_`\x03\x196\x01\x12a&JWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a&JW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a&?Wa&,W[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16`@Q\x90a.\x97\x90\x81\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a%\xFFW\x91\x83\x91` \x93a/\xA3\x849\x81R\x03\x01\x90\x82\xF0\x80\x15a%\xF2Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06iW\x81`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa%\xDDW[PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06iW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa%\xC8W[PP` a$\xC0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83T\x16\x85a$(a(\x15V[\x93a$\xD3a$4a&\xF3V[`@Q\x98\x89\x97\x88\x96\x87\x95\x7Fo\x04$U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87R`\x04\x87\x01Rb\x01\xE2@`$\x87\x01Rb\t\xFB\xF1`D\x87\x01Ra\x124`d\x87\x01RaVx`\x84\x87\x01R`\n`\xA4\x87\x01R`d`\xC4\x87\x01Ra\x9A\xBC`\xE4\x87\x01R`\xC8a\x01\x04\x87\x01Ra\x01$\x86\x01Ra\x01\x80a\x01D\x86\x01Ra\x01\x84\x85\x01\x90a'.V[\x90`\x03\x19\x84\x83\x03\x01a\x01d\x85\x01Ra'.V[\x03\x92Z\xF1\x90\x81\x15a\x03BW\x82\x91a%\xA9W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06iW\x81`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa%\x94W[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"T\x16\x17`\"U\x80\xF3[\x81a%\x9E\x91a&\xB2V[a\x06iW\x81_a%PV[a%\xC2\x91P` =` \x11a\x0E\xD7Wa\x0E\xC9\x81\x83a&\xB2V[_a$\xE5V[\x81a%\xD2\x91a&\xB2V[a\x06iW\x81_a#\xE2V[\x81a%\xE7\x91a&\xB2V[a\x06iW\x81_a#\x14V[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[a&8\x91P_\x90a&\xB2V[__a\"VV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[4a&JW_`\x03\x196\x01\x12a&JWa\x07wa\x0F\xE4a&\xF3V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a&\x85W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a&\x85W`@RV[`@Q\x90a'\x02`@\x83a&\xB2V[`\x1C\x82R\x7Fhttps://example.com/explorer\0\0\0\0` \x83\x01RV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a'\x94WPPP\x90V[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a'\x87V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a'\xDDWPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a'\xD0V[`@Q\x90a($`@\x83a&\xB2V[`\x17\x82R\x7Fhttps://example.com/rpc\0\0\0\0\0\0\0\0\0` \x83\x01RV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a(\x82WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a(\xBE\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa'.V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a(sV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a(\xFFWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a)b\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a'\xC0V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a(\xF0V[` \x81\x83\x03\x12a&JW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a&JW\x01\x81`\x1F\x82\x01\x12\x15a&JW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a&\x85W`@Q\x92a)\xE1`\x1F\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x85a&\xB2V[\x82\x84R` \x83\x83\x01\x01\x11a&JW\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a&\x85W`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15a+\x13W[` \x85\x10\x84\x14a*\xE6W\x84\x87R\x86\x93\x90\x81\x15a*\xA6WP`\x01\x14a*bW[Pa*`\x92P\x03\x83a&\xB2V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a*\x8AWPP\x90` a*`\x92\x82\x01\x01_a*SV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a*qV[` \x93Pa*`\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a*SV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93a*4V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10a-4Wa*`\x94T\x91\x81\x81\x10a,\xFEW[\x81\x81\x10a,\xC8W[\x81\x81\x10a,\x92W[\x81\x81\x10a,\\W[\x81\x81\x10a,&W[\x81\x81\x10a+\xF0W[\x81\x81\x10a+\xBBW[\x10a+\x8EW[P\x03\x83a&\xB2V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_a+\x86V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01a+\x80V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01a+xV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01a+pV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01a+hV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01a+`V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01a+XV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01a+PV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91a+8V[\x90\x81` \x91\x03\x12a&JWQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a&JW\x90V[`@Q\x90a-\xFC`@\x83a&\xB2V[`\x1B\x82R\x7Fhttps://new-example.com/rpc\0\0\0\0\0` \x83\x01RV[`\x08T`\xFF\x16\x80\x15a.7W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a&?W_\x91a.\xCFW[P\x15\x15\x90V[\x90P` \x81=` \x11a.\xF9W[\x81a.\xEA` \x93\x83a&\xB2V[\x81\x01\x03\x12a&JWQ_a.\xC9V[=\x91Pa.\xDDV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a&JWa/`_\x91a/r`@Q\x94\x85\x93\x84\x93\x7F\xF3 \xD9c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`@`\x04\x86\x01R`D\x85\x01\x90a'.V[\x90`\x03\x19\x84\x83\x03\x01`$\x85\x01Ra'.V[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a&?Wa/\x98WPV[_a*`\x91a&\xB2V\xFE`\xA04a\x01kW`\x1Fa.\x978\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01DW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\x01kWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x01kW\x80\x15a\x01XW_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x82U`@Q\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3a\x16\xDE\x81\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x83\x82\x10\x17a\x01DW\x82\x91a\x12\xB4\x839\x03\x90_\xF0\x80\x15a\x019W`@Q\x90a\x05\x05\x80\x83\x01`\x01`\x01`@\x1B\x03\x81\x11\x84\x82\x10\x17a\x01DW`@\x92\x84\x92a)\x92\x849`\x01`\x01`\xA0\x1B\x03\x16\x81R0` \x82\x01R\x03\x01\x90_\xF0\x80\x15a\x019W`\x80R`@Qa\x11D\x90\x81a\x01p\x829`\x80Q\x81\x81\x81a\x02\x15\x01R\x81\x81a\x05\xC2\x01R\x81\x81a\x08a\x01Ra\n\x98\x01R\xF3[`@Q=_\x82>=\x90\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x0B\x04\xEB\xFD\x14a\x08\x85W\x80cYe\x9E\x90\x14a\x085W\x80co\x04$U\x14a\x04WW\x80cqP\x18\xA6\x14a\x03\xD9W\x80c\x83\xF9M\xB7\x14a\x01\xB7W\x80c\x8D\xA5\xCB[\x14a\x01\x84W\x80c\xA3:\x8B`\x14a\x01DWc\xF2\xFD\xE3\x8B\x14a\0rW_\x80\xFD[4a\x01AW` `\x03\x196\x01\x12a\x01AWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\xA0a\x08\xC1V[a\0\xA8a\x0C V[\x16\x80\x15a\x01\x15Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x84U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x83\x80\xA3\x80\xF3[`$\x82\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x80`\x04R\xFD[\x80\xFD[P4a\x01AW` `\x03\x196\x01\x12a\x01AWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@` \x92`\x045\x81R`\x01\x84R T\x16`@Q\x90\x81R\xF3[P4a\x01AW\x80`\x03\x196\x01\x12a\x01AWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x91T\x16`@Q\x90\x81R\xF3[P4a\x01AW` `\x03\x196\x01\x12a\x01AWa\x01\xD1a\x08\xC1V[a\x01\xD9a\x0C V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a\x03UW;\x15a\x02\xD1W\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x02\xC2W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F6Y\xCF\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x88`\x04\x84\x01RZ\xF1\x80\x15a\x02\xC6Wa\x02\xADW[P\x7FQ\xEAo\xFD\xC9\x90\x9D\\\xA3A%\x9Fr!\x90.\x06vX]\x83>+\xB2\x1F\xA9#\xC8^\x86(\x86` \x83`@Q\x90\x81R\xA1\x80\xF3[\x81a\x02\xB7\x91a\x08\xE4V[a\x02\xC2W\x81_a\x02~V[P\x80\xFD[`@Q=\x84\x82>=\x90\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FImplementation must be a contrac`D\x82\x01R\x7Ft\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FNew implementation cannot be zer`D\x82\x01R\x7Fo address\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[P4a\x01AW\x80`\x03\x196\x01\x12a\x01AWa\x03\xF2a\x0C V[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[4a\x07\x80Wa\x01\x80`\x03\x196\x01\x12a\x07\x80Wa\x04qa\x08\xC1V[`$5\x90`d5\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\x07\x80W`\x845\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\x07\x80W`\xE45\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x07\x80Wa\x01$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x07\x80Wa\x01D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x80Wa\x05\x1F\x906\x90`\x04\x01a\t%V[\x92a\x01d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x80Wa\x05A\x906\x90`\x04\x01a\t%V[a\x05Ia\x0C V[a\x05T\x86\x15\x15a\t\x99V[\x85_R`\x01` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16a\x07\xB1W`@Q` \x81\x01\x90\x87\x82R` \x81Ra\x05\x95`@\x82a\x08\xE4V[Q\x90 `@Qa\x04\xD7\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x07\x84W\x82\x91a\x06\r\x91a\x0Cm\x849\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`@` \x82\x01\x81\x90R_\x90\x82\x01R``\x01\x90V[\x03\x90_\xF5\x80\x15a\x07uWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x96\x86_R`\x01` R`@_ \x88\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90U\x87;\x15a\x07\x80W_\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95a\x07\x1E\x94a\x07\x0B\x93`@Q\x9C\x8D\x99\x8A\x99\x7F\xBFy\xFD\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8BR\x16`\x04\x8A\x01R\x8B`$\x8A\x01R`D5`D\x8A\x01R`d\x89\x01R`\x84\x88\x01R`\xA45`\xA4\x88\x01R`\xC45`\xC4\x88\x01R`\xE4\x87\x01Ra\x01\x045a\x01\x04\x87\x01Ra\x01$\x86\x01Ra\x01\x80a\x01D\x86\x01Ra\x01\x84\x85\x01\x90a\x0B\xDDV[\x90`\x03\x19\x84\x83\x03\x01a\x01d\x85\x01Ra\x0B\xDDV[\x03\x81\x83\x86Z\xF1\x92\x83\x15a\x07uW` \x93a\x07eW[P\x7F\xEA\xF2\xB9\xD4\xFDn\xBAZ`\x87\x04\x99\xF63\\j\xB4\x82n\x02\x9A\xFFe\xBA\x06\x192\x9D\xBDB\x1E\xC3\x83`@Q\x84\x81R\xA2`@Q\x90\x81R\xF3[_a\x07o\x91a\x08\xE4V[_a\x073V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FConfig already exists for this c`D\x82\x01R\x7Fhain ID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[4a\x07\x80W_`\x03\x196\x01\x12a\x07\x80W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x07\x80W` `\x03\x196\x01\x12a\x07\x80W` a\x08\xA3`\x045a\t\xFEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x07\x80WV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\x84W`@RV[\x81`\x1F\x82\x01\x12\x15a\x07\x80W\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x07\x84W`@Q\x92a\tx` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x86\x01\x16\x01\x85a\x08\xE4V[\x82\x84R` \x83\x83\x01\x01\x11a\x07\x80W\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[\x15a\t\xA0WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FChain ID cannot be zero\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x80_R`\x01` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16a\x0B\xB8W\x80a\nFs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x15\x15a\t\x99V[`@Q` \x81\x01\x91\x82R` \x81Ra\n_`@\x82a\x08\xE4V[Q\x90 `@Qa\x04\xD7a\nu` \x82\x01\x83a\x08\xE4V[\x80\x82R` \x82\x01\x90a\x0Cm\x829a\x0B``@Q\x91` \x80\x84\x01a\x0B\x11\x85a\n\xE5\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x92\x16\x81R`@` \x82\x01R_`@\x82\x01R\x01\x90V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x87R\x86a\x08\xE4V[`@Q\x94\x85\x93\x83\x85\x01\x97Q\x80\x91\x89^\x84\x01\x90\x83\x82\x01\x90_\x82RQ\x92\x83\x91^\x01\x01_\x81R\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a\x08\xE4V[Q\x90 `@Q\x90` \x82\x01\x92\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R0``\x1B`!\x84\x01R`5\x83\x01R`U\x82\x01R`U\x81Ra\x0B\xB1`u\x82a\x08\xE4V[Q\x90 \x16\x90V[_R`\x01` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x90V[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\x0C@WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD\xFE`\xA0\x80`@Ra\x04\xD7\x808\x03\x80\x91a\0\x17\x82\x85a\x02\x92V[\x839\x81\x01`@\x82\x82\x03\x12a\x01\xEBWa\0.\x82a\x02\xC9V[` \x83\x01Q\x90\x92`\x01`\x01`@\x1B\x03\x82\x11a\x01\xEBW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x01\xEBW\x81Qa\0[\x81a\x02\xDDV[\x92a\0i`@Q\x94\x85a\x02\x92V[\x81\x84R` \x84\x01\x92` \x83\x83\x01\x01\x11a\x01\xEBW\x81_\x92` \x80\x93\x01\x85^\x84\x01\x01R\x82;\x15a\x02tW\x7F\xA3\xF0\xADt\xE5B:\xEB\xFD\x80\xD3\xEFCFW\x835\xA9\xA7*\xEA\xEEY\xFFl\xB3X+5\x13=P\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`@Qc\\`\xDA\x1B`\xE0\x1B\x81R\x90\x91\x90` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x01\xF7W_\x91a\x02:W[P\x80;\x15a\x02\x1AWP\x81\x7F\x1C\xF3\xB0:l\xF1\x9F\xA2\xBA\xBAM\xF1H\xE9\xDC\xAB\xED\xEA\x7F\x8A\\\x07\x84\x0E ~\\\x08\x9B\xE9]>_\x80\xA2\x82Q\x15a\x02\x02W` `\x04\x92`@Q\x93\x84\x80\x92c\\`\xDA\x1B`\xE0\x1B\x82RZ\xFA\x91\x82\x15a\x01\xF7W_\x92a\x01\xAEW[P_\x80\x91a\x01\x8A\x94Q\x90\x84Z\xF4=\x15a\x01\xA6W=\x91a\x01n\x83a\x02\xDDV[\x92a\x01|`@Q\x94\x85a\x02\x92V[\x83R=_` \x85\x01>a\x02\xF8V[P[`\x80R`@Qa\x01\x80\x90\x81a\x03W\x829`\x80Q\x81`F\x01R\xF3[``\x91a\x02\xF8V[\x92\x91P` \x83=` \x11a\x01\xEFW[\x81a\x01\xCA` \x93\x83a\x02\x92V[\x81\x01\x03\x12a\x01\xEBW_\x80\x91a\x01\xE1a\x01\x8A\x95a\x02\xC9V[\x93\x94P\x91Pa\x01PV[_\x80\xFD[=\x91Pa\x01\xBDV[`@Q=_\x82>=\x90\xFD[PPP4\x15a\x01\x8CWc\xB3\x98\x97\x9F`\xE0\x1B_R`\x04_\xFD[cL\x9C\x8C\xE3`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[\x90P` \x81=` \x11a\x02lW[\x81a\x02U` \x93\x83a\x02\x92V[\x81\x01\x03\x12a\x01\xEBWa\x02f\x90a\x02\xC9V[_a\0\xF5V[=\x91Pa\x02HV[c\x193\xB4;`\xE2\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04R`$\x90\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x02\xB5W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xEBWV[`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB5W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x90a\x03\x1CWP\x80Q\x15a\x03\rW\x80Q\x90` \x01\xFD[c\xD6\xBD\xA2u`\xE0\x1B_R`\x04_\xFD[\x81Q\x15\x80a\x03MW[a\x03-WP\x90V[c\x99\x96\xB3\x15`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[P\x80;\x15a\x03%V\xFE`\x80`@R\x7F\\`\xDA\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80R` `\x80`\x04\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\x01\x07W_\x90\x15a\x01cWP` =` \x11a\x01\0W[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x82\x01\x16`\x80\x01\x90`\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\0\xD3Wa\0\xCE\x91`@R`\x80\x01a\x01\x12V[a\x01cV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[P=a\0\x81V[`@Q=_\x82>=\x90\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80` \x91\x01\x12a\x01_W`\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01_W\x90V[_\x80\xFD[_\x80\x916\x82\x807\x816\x91Z\xF4=_\x80>\x15a\x01|W=_\xF3[=_\xFD`\x80\x80`@R4`\xAAW_Q` a\x16\xBE_9_Q\x90_RT`\xFF\x81`@\x1C\x16`\x9BW`\x02`\x01`@\x1B\x03\x19`\x01`\x01`@\x1B\x03\x82\x16\x01`IW[`@Qa\x16\x0F\x90\x81a\0\xAF\x829\xF3[`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17_Q` a\x16\xBE_9_Q\x90_RU\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x90\xA1_\x80`:V[c\xF9.\xE8\xA9`\xE0\x1B_R`\x04_\xFD[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x07C\xBFn\x14a\x12\x03W\x80c\x18\xB5\xCE\x81\x14a\x11\xD0W\x80c)\x08\x03V\x14a\x10\x84W\x80cK\x8B\xE3\xF7\x14a\x0F&W\x80cT\xFDMP\x14a\x0E\x81W\x80cW\xD1\xBA%\x14a\x0EdW\x80cn\xDDl\t\x14a\x0E1W\x80cr@\xF9\xAF\x14a\x0C\xD1W\x80c\x85\xE1\xF4\xD0\x14a\x0C\xB4W\x80c\x8D\xA5\xCB[\x14a\x0C\x82W\x80c\xA3\xC6\xE1\xE7\x14a\x0CeW\x80c\xAAjC\xD8\x14a\x0C2W\x80c\xBFm\xB6\xF8\x14a\x0B\xFFW\x80c\xBFy\xFD\x1C\x14a\x02{W\x80c\xC7\xA7`\x95\x14a\x01\x83W\x80c\xD1\xF4s|\x14a\x01fW\x80c\xF2\xFD\xE3\x8B\x14a\x01\0Wc\xF8\xA1D\xBE\x14a\0\xDFW_\x80\xFD[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` `\x06T`@Q\x90\x81R\xF3[_\x80\xFD[4a\0\xFCW` `\x03\x196\x01\x12a\0\xFCWa\x01da\x01\x1Ca\x13\x97V[a\x01>s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x14a\x14\x10V[a\x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x15\x15a\x15$V[a\x15\x89V[\0[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` `\x08T`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW`@Q_`\nTa\x01\xA3\x81a\x12\xA8V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x029WP`\x01\x14a\x01\xDBW[a\x01\xD7\x83a\x01\xCB\x81\x85\x03\x82a\x12\xF9V[`@Q\x91\x82\x91\x82a\x13\x1CV[\x03\x90\xF3[`\n_\x90\x81R\x7F\xC6Z{\xB8\xD65\x1C\x1C\xF7\x0C\x95\xA3\x16\xCCj\x92\x83\x9C\x98f\x82\xD9\x8B\xC3_\x95\x8FH\x83\xF9\xD2\xA8\x93\x92P\x90[\x80\x82\x10a\x02\x1FWP\x90\x91P\x81\x01` \x01a\x01\xCBa\x01\xBBV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x02\x07V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x01\xCB\x90Pa\x01\xBBV[4a\0\xFCWa\x01\x80`\x03\x196\x01\x12a\0\xFCWa\x02\x95a\x13\x97V[`$5`D5\x91`d5\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x80\x94\x03a\0\xFCW`\x845s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\0\xFCW`\xE45s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\0\xFCWa\x01$5\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\0\xFCWa\x01D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFCWa\x03D\x906\x90`\x04\x01a\x13\xBAV[\x96a\x01d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFCWa\x03f\x906\x90`\x04\x01a\x13\xBAV[\x94\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x97`\xFF\x89`@\x1C\x16\x15\x98g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x90\x81a\x0B\xF7W[`\x01\x14\x90\x81a\x0B\xEDW[\x15\x90\x81a\x0B\xE4W[Pa\x0B\xBCW\x89`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x83\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x0BgW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x15a\x0B\tW\x80\x15a\n\xABW\x81\x15a\n'W\x82\x15a\t\xA3W\x83\x15a\t\x1FW\x84\x15a\x08\x9BW\x85\x15a\x08\x17W`\x05U`\x06U\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02T\x16\x17`\x02U\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x03T\x16\x17`\x03U`\xA45`\x07U`\xC45`\x08U\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04T\x16\x17`\x04Ua\x01\x045`\tUa\x04\xEC`\x0CTa\x12\xA8V[`\x1F\x81\x11a\x07\xD9W[P`\n\x7F1.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01`\x0CU\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01T\x16\x17`\x01U\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07+Wa\x05k\x81a\x05f`\nTa\x12\xA8V[a\x14\x8BV[` \x94`\x1F\x82\x11`\x01\x14a\x07XWa\x05\x9C\x92\x93\x94\x95\x82\x91_\x92a\x06\x99W[PP_\x19\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17\x90V[`\nU[\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x07+Wa\x05\xC6\x82a\x05\xC1`\x0BTa\x12\xA8V[a\x14\xC5V[` \x90`\x1F\x83\x11`\x01\x14a\x06\xA4W\x91\x80a\x05\xF8\x92a\x06\0\x95\x94_\x92a\x06\x99WPP_\x19\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17\x90V[`\x0BUa\x15\x89V[a\x06\x06W\0[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1\0[\x01Q\x90P\x86\x80a\x05\x89V[\x90`\x1F\x19\x83\x16\x91`\x0B_R\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x92_[\x81\x81\x10a\x07\x13WP\x91`\x01\x93\x91\x85a\x06\0\x97\x96\x94\x10a\x06\xFBW[PPP\x81\x1B\x01`\x0BUa\x15\x89V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x85\x80\x80a\x06\xEDV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x06\xD3V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x1F\x19\x82\x16\x95`\n_R\x7F\xC6Z{\xB8\xD65\x1C\x1C\xF7\x0C\x95\xA3\x16\xCCj\x92\x83\x9C\x98f\x82\xD9\x8B\xC3_\x95\x8FH\x83\xF9\xD2\xA8\x91_[\x88\x81\x10a\x07\xC1WP\x83`\x01\x95\x96\x97\x98\x10a\x07\xA9W[PPP\x81\x1B\x01`\nUa\x05\xA0V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x85\x80\x80a\x07\x9BV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x07\x86V[`\x0C_Ra\x08\x11\x90`\x1F\x01`\x05\x1C\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7\x90\x81\x01\x90a\x14uV[\x85a\x04\xF5V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FInitial appchain owner cannot be`D\x82\x01R\x7F zero address\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencing contract address cann`D\x82\x01R\x7Fot be zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FArbitrum inbox address cannot be`D\x82\x01R\x7F zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FArbitrum bridge address cannot b`D\x82\x01R\x7Fe zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FSequencing chain ID cannot be ze`D\x82\x01R\x7Fro\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FChain ID cannot be zero\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FOwner cannot be zero address\0\0\0\0`D\x82\x01R\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x01\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x8Aa\x04\x0CV[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90P\x15\x8Ca\x03\xB9V[0;\x15\x91Pa\x03\xB1V[\x8B\x91Pa\x03\xA7V[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x04T\x16`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03T\x16`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` `\tT`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` `\x05T`@Q\x90\x81R\xF3[4a\0\xFCWa\x0C\xDF6a\x13FV[a\r\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x14a\x14\x10V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07+Wa\r\x1B`\x0CTa\x12\xA8V[`\x1F\x81\x11a\r\xD8W[P_`\x1F\x82\x11`\x01\x14a\r`W\x81\x90a\rP\x93_\x92a\rUWPP_\x19\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17\x90V[`\x0CU\0[\x015\x90P\x83\x80a\x05\x89V[`\x1F\x19\x82\x16\x92\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7\x91_[\x85\x81\x10a\r\xC0WP\x83`\x01\x95\x10a\r\xA7W[PPP\x81\x1B\x01`\x0CU\0[_\x19`\xF8\x85`\x03\x1B\x16\x1C\x19\x91\x015\x16\x90U\x82\x80\x80a\r\x9CV[\x90\x92` `\x01\x81\x92\x86\x86\x015\x81U\x01\x94\x01\x91\x01a\r\x8AV[`\x0C_Ra\x0E!\x90\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x0E'W[`\x1F\x01`\x05\x1C\x01\x90a\x14uV[\x82a\r$V[\x90\x91P\x81\x90a\x0E\x14V[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` `\x07T`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW`@Q_`\x0CTa\x0E\xA1\x81a\x12\xA8V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x029WP`\x01\x14a\x0E\xC8Wa\x01\xD7\x83a\x01\xCB\x81\x85\x03\x82a\x12\xF9V[`\x0C_\x90\x81R\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7\x93\x92P\x90[\x80\x82\x10a\x0F\x0CWP\x90\x91P\x81\x01` \x01a\x01\xCBa\x01\xBBV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x0E\xF4V[4a\0\xFCWa\x0F46a\x13FV[a\x0FVs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x14a\x14\x10V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07+Wa\x0Ft\x81a\x05\xC1`\x0BTa\x12\xA8V[_\x91`\x1F\x82\x11`\x01\x14a\x0F\xE4Wa\x0F\xC1\x82\x80\x7F\x02X^\xBC\xE9\x18\xF6V`M\xAB\xB232\xA6\xDF\x1D\xCD\xE1\x19t\x1F|\x1F\x8F\xB3{\x19\x13\x92\xA2\xA7\x95_\x91a\x0F\xD9W[P_\x19\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17\x90V[`\x0BU[a\x0F\xD4`@Q\x92\x83\x92\x83a\x14\xFDV[\x03\x90\xA1\0[\x90P\x83\x015\x86a\x0F\xAFV[`\x1F\x19\x82\x16`\x0B_R\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x90_[\x81\x81\x10a\x10lWP\x93\x83\x7F\x02X^\xBC\xE9\x18\xF6V`M\xAB\xB232\xA6\xDF\x1D\xCD\xE1\x19t\x1F|\x1F\x8F\xB3{\x19\x13\x92\xA2\xA7\x95\x10a\x10SW[PP`\x01\x82\x81\x1B\x01`\x0BUa\x0F\xC5V[_\x19`\xF8\x85`\x03\x1B\x16\x1C\x19\x90\x83\x015\x16\x90U\x83\x80a\x10CV[\x83\x86\x015\x83U` \x95\x86\x01\x95`\x01\x90\x93\x01\x92\x01a\x10\x11V[4a\0\xFCWa\x10\x926a\x13FV[a\x10\xB4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x14a\x14\x10V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07+Wa\x10\xD2\x81a\x05f`\nTa\x12\xA8V[_\x91`\x1F\x82\x11`\x01\x14a\x110Wa\x11\x1E\x82\x80\x7Fg\xC5z\xE6\xAD\x92L\xD0\x93\xFB/\x06\xBC\x0B(\xFDXyH\x10Q\xA9\xC2\x03\xA4M'\xC8\x90MC|\x95_\x91a\x0F\xD9WP_\x19\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17\x90V[`\nUa\x0F\xD4`@Q\x92\x83\x92\x83a\x14\xFDV[`\x1F\x19\x82\x16`\n_R\x7F\xC6Z{\xB8\xD65\x1C\x1C\xF7\x0C\x95\xA3\x16\xCCj\x92\x83\x9C\x98f\x82\xD9\x8B\xC3_\x95\x8FH\x83\xF9\xD2\xA8\x90_[\x81\x81\x10a\x11\xB8WP\x93\x83\x7Fg\xC5z\xE6\xAD\x92L\xD0\x93\xFB/\x06\xBC\x0B(\xFDXyH\x10Q\xA9\xC2\x03\xA4M'\xC8\x90MC|\x95\x10a\x11\x9FW[PP`\x01\x82\x81\x1B\x01`\nUa\x0F\xC5V[_\x19`\xF8\x85`\x03\x1B\x16\x1C\x19\x90\x83\x015\x16\x90U\x83\x80a\x11\x8FV[\x83\x86\x015\x83U` \x95\x86\x01\x95`\x01\x90\x93\x01\x92\x01a\x11]V[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01T\x16`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW`@Q_`\x0BTa\x12#\x81a\x12\xA8V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x029WP`\x01\x14a\x12JWa\x01\xD7\x83a\x01\xCB\x81\x85\x03\x82a\x12\xF9V[`\x0B_\x90\x81R\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x93\x92P\x90[\x80\x82\x10a\x12\x8EWP\x90\x91P\x81\x01` \x01a\x01\xCBa\x01\xBBV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x12vV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x12\xEFW[` \x83\x10\x14a\x12\xC2WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x12\xB7V[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07+W`@RV[`\x1F\x19`\x1F` `@\x94\x81\x85R\x80Q\x91\x82\x91\x82\x82\x88\x01R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` `\x03\x19\x83\x01\x12a\0\xFCW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFCW\x82`#\x82\x01\x12\x15a\0\xFCW\x80`\x04\x015\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11a\0\xFCW`$\x84\x83\x01\x01\x11a\0\xFCW`$\x01\x91\x90V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0\xFCWV[\x81`\x1F\x82\x01\x12\x15a\0\xFCW\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x07+W`@Q\x92a\x13\xEF` `\x1F\x19`\x1F\x86\x01\x16\x01\x85a\x12\xF9V[\x82\x84R` \x83\x83\x01\x01\x11a\0\xFCW\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[\x15a\x14\x17WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x81\x81\x10a\x14\x80WPPV[_\x81U`\x01\x01a\x14uV[\x90`\x1F\x82\x11a\x14\x98WPPV[a\x14\xC3\x91`\n_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x0E'W`\x1F\x01`\x05\x1C\x01\x90a\x14uV[V[\x90`\x1F\x82\x11a\x14\xD2WPPV[a\x14\xC3\x91`\x0B_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x0E'W`\x1F\x01`\x05\x1C\x01\x90a\x14uV[\x90`\x1F\x83`@\x94`\x1F\x19\x93` \x86R\x81` \x87\x01R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x15a\x15+WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FNew owner cannot be zero address`D\x82\x01R\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x15\xAA\x81\x15\x15a\x15$V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0`\x804a\x014W`\x1Fa\x05\x058\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x018W\x80\x84\x92`@\x94\x85R\x839\x81\x01\x03\x12a\x014Wa\0F\x81a\x01LV[\x90`\x01`\x01`\xA0\x1B\x03\x90a\0\\\x90` \x01a\x01LV[\x16\x90\x81\x15a\x01!W_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x82U`@Q\x93\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3\x80;\x15a\x01\x01W`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2a\x03\xA4\x90\x81a\x01a\x829\xF3[c!\x1E\xB1Y`\xE2\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x014WV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c6Y\xCF\xE6\x14a\x02~W\x80c\\`\xDA\x1B\x14a\x02-W\x80cqP\x18\xA6\x14a\x01\x93W\x80c\x8D\xA5\xCB[\x14a\x01CWc\xF2\xFD\xE3\x8B\x14a\0PW_\x80\xFD[4a\x01?W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01?W`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x01?Wa\0\xA8a\x03XV[\x80\x15a\x01\x13Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x80\xFD[4a\x01?W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\x01?W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01?Wa\x01\xC9a\x03XV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01?W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01T\x16`@Q\x90\x81R\xF3[4a\x01?W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01?W`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x81\x03a\x01?Wa\x02\xD7a\x03XV[;\x15a\x03-W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01T\x16\x17`\x01U\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\0[\x7F\x84z\xC5d\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\x03xWV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f5f3560e01c80630743bf6e1461264e5780630a9254e4146121ba5780631ed7831c1461212f57806326df4e2514611f925780632ade388014611d8457806334784bf114611b375780633e5e3c2314611aac5780633f7286f414611a215780634dbdc85d1461186157806352069fc21461168057806357d1ba25146116645780635b7c15991461131657806360944ce61461113557806366d9a9a014610ff85780636806ba0614610fc85780636edd6c0914610fab57806385226c8114610f215780638522ac8c14610af257806385e1f4d014610ad45780638da5cb5b14610a9d578063916a17c6146109e6578063a3c6e1e7146109ca578063aa6a43d8146109ad578063b0464fdc146108f6578063b5508aa91461086c578063ba414fa614610847578063bf6db6f81461082a578063ca0ab075146107f6578063d1f4737c146107da578063d831975e146107a7578063e20c9f711461070c578063ebb43738146106d8578063f8a144be146106ba578063fa7626d414610697578063fdfc99df1461037a5763febc4432146101a6575f80fd5b3461033f578060031936011261033f57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517fca669fa70000000000000000000000000000000000000000000000000000000081526103e76004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610365575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f576040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f43616c6c6572206973206e6f7420746865206f776e657200000000000000000060448201528190818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610350575b5073ffffffffffffffffffffffffffffffffffffffff60225416803b1561034d578180916024604051809481937ff2fde38b0000000000000000000000000000000000000000000000000000000083526004808401525af180156103425761032e5750f35b81610338916126b2565b61033f5780f35b80fd5b6040513d84823e3d90fd5b50fd5b8161035a916126b2565b61033f57805f6102c9565b8161036f916126b2565b61033f57805f610228565b503461033f578060031936011261033f578073ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610682575b5050610421612ded565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561066957816040517f491cc7c200000000000000000000000000000000000000000000000000000000815281818061048a60048201906001606060808401938281525f60208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103425761066d575b50507f67c57ae6ad924cd093fb2f06bc0b28fd5879481051a9c203a44d27c8904d437c60405160208152806104e7602082018561272e565b0390a18173ffffffffffffffffffffffffffffffffffffffff60225416803b15610669578160405180927f290803560000000000000000000000000000000000000000000000000000000082526020600483015281838161054b602482018a61272e565b03925af1801561034257610654575b509060049173ffffffffffffffffffffffffffffffffffffffff60225416604051938480927fc7a760950000000000000000000000000000000000000000000000000000000082525afa8015610649576105bb928491610627575b50612f01565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103425761032e5750f35b61064391503d8086833e61063b81836126b2565b810190612971565b5f6105b5565b6040513d85823e3d90fd5b8161065e916126b2565b61066957815f61055a565b5080fd5b81610677916126b2565b61066957815f6104af565b8161068c916126b2565b61033f57805f610417565b503461033f578060031936011261033f57602060ff601f54166040519015158152f35b503461033f578060031936011261033f5760206040516209fbf18152f35b503461033f578060031936011261033f57602073ffffffffffffffffffffffffffffffffffffffff60225416604051908152f35b503461033f578060031936011261033f5760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b81811061077b576107778561076b818703826126b2565b60405191829182612771565b0390f35b825473ffffffffffffffffffffffffffffffffffffffff16845260209093019260019283019201610754565b503461033f578060031936011261033f57602073ffffffffffffffffffffffffffffffffffffffff815416604051908152f35b503461033f578060031936011261033f57602060405160648152f35b503461033f578060031936011261033f57602073ffffffffffffffffffffffffffffffffffffffff60215416604051908152f35b503461033f578060031936011261033f576020604051619abc8152f35b503461033f578060031936011261033f576020610862612e28565b6040519015158152f35b503461033f578060031936011261033f5760195461088981612a02565b9161089760405193846126b2565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b8383106108d957604051806107778782612850565b6001602081926108e885612a1a565b8152019201920191906108c4565b503461033f578060031936011261033f57601c5461091381612a02565b9161092160405193846126b2565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310610963576040518061077787826128cd565b6002602060019260405161097681612669565b73ffffffffffffffffffffffffffffffffffffffff865416815261099b858701612b1d565b8382015281520192019201919061094e565b503461033f578060031936011261033f5760206040516156788152f35b503461033f578060031936011261033f57602060405160c88152f35b503461033f578060031936011261033f57601d54610a0381612a02565b91610a1160405193846126b2565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310610a53576040518061077787826128cd565b60026020600192604051610a6681612669565b73ffffffffffffffffffffffffffffffffffffffff8654168152610a8b858701612b1d565b83820152815201920192019190610a3e565b503461033f578060031936011261033f57602073ffffffffffffffffffffffffffffffffffffffff601f5460081c16604051908152f35b503461033f578060031936011261033f5760206040516201e2408152f35b503461033f578060031936011261033f578073ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610f0c575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152600160048201526001602482015281604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610ef7575b5073ffffffffffffffffffffffffffffffffffffffff601f5460081c166004604051917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08480a373ffffffffffffffffffffffffffffffffffffffff60225416803b15610ef357816024818580947ff2fde38b0000000000000000000000000000000000000000000000000000000083526004808401525af1801561034257610ede575b506004602073ffffffffffffffffffffffffffffffffffffffff60225416604051928380927f8da5cb5b0000000000000000000000000000000000000000000000000000000082525afa908115610342578291610eaf575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d5773ffffffffffffffffffffffffffffffffffffffff604051917f515361f6000000000000000000000000000000000000000000000000000000008352166004820152600460248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561034257610e9a575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610e85575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517f06447d56000000000000000000000000000000000000000000000000000000008152600480820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257610682575050610421612ded565b81610e8f916126b2565b61033f57805f610e0a565b81610ea4916126b2565b61033f57805f610d9e565b610ed1915060203d602011610ed7575b610ec981836126b2565b810190612dc1565b5f610d12565b503d610ebf565b81610ee8916126b2565b61033f57805f610cba565b5050fd5b81610f01916126b2565b61033f57805f610c16565b81610f16916126b2565b61033f57805f610b8f565b503461033f578060031936011261033f57601a54610f3e81612a02565b91610f4c60405193846126b2565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b838310610f8e57604051806107778782612850565b600160208192610f9d85612a1a565b815201920192019190610f79565b503461033f578060031936011261033f5760206040516112348152f35b503461033f578060031936011261033f57610777610fe4612815565b60405191829160208352602083019061272e565b503461033f578060031936011261033f57601b5461101581612a02565b61102260405191826126b2565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b8383106110fa57868587604051928392602084019060208552518091526040840160408260051b8601019392905b82821061108f57505050500390f35b919360206110ea827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc06001959799849503018652885190836110da835160408452604084019061272e565b92015190848184039101526127c0565b9601920192018594939192611080565b6002602060019260405161110d81612669565b61111686612a1a565b8152611123858701612b1d565b83820152815201920192019190611052565b503461033f578060031936011261033f57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517fca669fa70000000000000000000000000000000000000000000000000000000081526103e76004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257611301575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f576040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f43616c6c6572206973206e6f7420746865206f776e657200000000000000000060448201528190818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610342576112ec575b5073ffffffffffffffffffffffffffffffffffffffff60225416803b1561034d578180916064604051809481937f2908035600000000000000000000000000000000000000000000000000000000835260206004840152601b60248401527f68747470733a2f2f6e65772d6578616d706c652e636f6d2f727063000000000060448401525af180156103425761032e5750f35b816112f6916126b2565b61033f57805f611259565b8161130b916126b2565b61033f57805f6111b8565b503461033f578060031936011261033f578073ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103425761164f575b505080604080516113c482826126b2565b602081527f68747470733a2f2f6e65772d6578616d706c652e636f6d2f6578706c6f7265726020820152737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ef35781517f491cc7c200000000000000000000000000000000000000000000000000000000815283818061145560048201906001606060808401938281525f60208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156116135790849161163a575b50507f02585ebce918f656604dabb23332a6df1dcde119741f7c1f8fb37b191392a2a7825160208152806114b4602082018561272e565b0390a173ffffffffffffffffffffffffffffffffffffffff60225416803b156116355783835180927f4b8be3f700000000000000000000000000000000000000000000000000000000825260206004830152818381611516602482018961272e565b03925af1801561161357908491611620575b5060049073ffffffffffffffffffffffffffffffffffffffff602254168451928380927f0743bf6e0000000000000000000000000000000000000000000000000000000082525afa9081156116135790611589929185916115ff5750612f01565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d578051907f90c5013b000000000000000000000000000000000000000000000000000000008252828260048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156115f6575061032e5750f35b513d84823e3d90fd5b61064391503d8087833e61063b81836126b2565b50505051903d90823e3d90fd5b8161162a916126b2565b610ef357825f611528565b505050fd5b81611644916126b2565b610ef357825f61147d565b81611659916126b2565b61033f57805f6113b3565b503461033f578060031936011261033f576020604051600a8152f35b503461033f578060031936011261033f57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517fca669fa70000000000000000000000000000000000000000000000000000000081526103e76004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103425761184c575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f576040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f43616c6c6572206973206e6f7420746865206f776e657200000000000000000060448201528190818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257611837575b5073ffffffffffffffffffffffffffffffffffffffff60225416803b1561034d578180916064604051809481937f4b8be3f700000000000000000000000000000000000000000000000000000000835260206004840152602060248401527f68747470733a2f2f6e65772d6578616d706c652e636f6d2f6578706c6f72657260448401525af180156103425761032e5750f35b81611841916126b2565b61033f57805f6117a4565b81611856916126b2565b61033f57805f611703565b503461033f578060031936011261033f578073ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257611a0c575b5073ffffffffffffffffffffffffffffffffffffffff60225416803b1561034d578160405180927f4b8be3f7000000000000000000000000000000000000000000000000000000008252818381604060048201602081525f60208201520103925af18015610342576119f7575b5060049073ffffffffffffffffffffffffffffffffffffffff60225416604051928380927f0743bf6e0000000000000000000000000000000000000000000000000000000082525afa8015610342576105bb9183916119dd575b50604051906119d56020836126b2565b838252612f01565b6119f191503d8085833e61063b81836126b2565b5f6119c5565b81611a01916126b2565b61033f57805f61196b565b81611a16916126b2565b61033f57805f6118fe565b503461033f578060031936011261033f5760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110611a80576107778561076b818703826126b2565b825473ffffffffffffffffffffffffffffffffffffffff16845260209093019260019283019201611a69565b503461033f578060031936011261033f5760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b818110611b0b576107778561076b818703826126b2565b825473ffffffffffffffffffffffffffffffffffffffff16845260209093019260019283019201611af4565b503461033f578060031936011261033f578073ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257611d6f575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4e6577206f776e65722063616e6e6f74206265207a65726f20616464726573736044820152818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257611d5a575b5073ffffffffffffffffffffffffffffffffffffffff60225416803b1561034d578180916024604051809481937ff2fde38b0000000000000000000000000000000000000000000000000000000083528160048401525af1801561034257611d45575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561033f57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103425761032e5750f35b81611d4f916126b2565b61033f57805f611cd7565b81611d64916126b2565b61033f57805f611c74565b81611d79916126b2565b61033f57805f611bd4565b503461033f578060031936011261033f57601e54611da181612a02565b611dae60405191826126b2565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b838310611efc5786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310611e1a5786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc09086929496030183528551906020604082019273ffffffffffffffffffffffffffffffffffffffff81511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110611eb357505050505060208060019297019301930190928695949293611e0d565b9091929394602080611eef837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08760019603018952895161272e565b9701950193929101611e8f565b604051611f0881612669565b73ffffffffffffffffffffffffffffffffffffffff8354168152600183018054611f3181612a02565b91611f3f60405193846126b2565b8183528a526020808b20908b9084015b838210611f75575050505060019282602092836002950152815201920192019190611dde565b600160208192611f8486612a1a565b815201930191019091611f4f565b503461033f578060031936011261033f578073ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561034d57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156103425761211a575b5073ffffffffffffffffffffffffffffffffffffffff60225416803b1561034d578160405180927f29080356000000000000000000000000000000000000000000000000000000008252818381604060048201602081525f60208201520103925af1801561034257612105575b5060049073ffffffffffffffffffffffffffffffffffffffff60225416604051928380927fc7a760950000000000000000000000000000000000000000000000000000000082525afa8015610342576105bb9183916119dd5750604051906119d56020836126b2565b8161210f916126b2565b61033f57805f61209c565b81612124916126b2565b61033f57805f61202f565b503461033f578060031936011261033f5760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b81811061218e576107778561076b818703826126b2565b825473ffffffffffffffffffffffffffffffffffffffff16845260209093019260019283019201612177565b503461264a575f60031936011261264a5773ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561264a57604051907f06447d5600000000000000000000000000000000000000000000000000000000825260048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561263f5761262c575b5073ffffffffffffffffffffffffffffffffffffffff601f5460081c1660405190612e97908183019183831067ffffffffffffffff8411176125ff57918391602093612fa38439815203019082f080156125f257737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561066957816040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610342576125dd575b505073ffffffffffffffffffffffffffffffffffffffff16807fffffffffffffffffffffffff000000000000000000000000000000000000000060215416176021558173ffffffffffffffffffffffffffffffffffffffff601f5460081c16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561066957604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610342576125c8575b505060206124c09173ffffffffffffffffffffffffffffffffffffffff601f5460081c169073ffffffffffffffffffffffffffffffffffffffff83541685612428612815565b936124d36124346126f3565b604051988997889687957f6f04245500000000000000000000000000000000000000000000000000000000875260048701526201e24060248701526209fbf1604487015261123460648701526156786084870152600a60a4870152606460c4870152619abc60e487015260c861010487015261012486015261018061014486015261018485019061272e565b906003198483030161016485015261272e565b03925af19081156103425782916125a9575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561066957816040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561034257612594575b5073ffffffffffffffffffffffffffffffffffffffff82167fffffffffffffffffffffffff0000000000000000000000000000000000000000602254161760225580f35b8161259e916126b2565b61066957815f612550565b6125c2915060203d602011610ed757610ec981836126b2565b5f6124e5565b816125d2916126b2565b61066957815f6123e2565b816125e7916126b2565b61066957815f612314565b50604051903d90823e3d90fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b61263891505f906126b2565b5f5f612256565b6040513d5f823e3d90fd5b5f80fd5b3461264a575f60031936011261264a57610777610fe46126f3565b6040810190811067ffffffffffffffff82111761268557604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761268557604052565b604051906127026040836126b2565b601c82527f68747470733a2f2f6578616d706c652e636f6d2f6578706c6f726572000000006020830152565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b60206040818301928281528451809452019201905f5b8181106127945750505090565b825173ffffffffffffffffffffffffffffffffffffffff16845260209384019390920191600101612787565b90602080835192838152019201905f5b8181106127dd5750505090565b82517fffffffff00000000000000000000000000000000000000000000000000000000168452602093840193909201916001016127d0565b604051906128246040836126b2565b601782527f68747470733a2f2f6578616d706c652e636f6d2f7270630000000000000000006020830152565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061288257505050505090565b90919293946020806128be837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08660019603018752895161272e565b97019301930191939290612873565b602081016020825282518091526040820191602060408360051b8301019401925f915b8383106128ff57505050505090565b9091929394602080612962837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b5173ffffffffffffffffffffffffffffffffffffffff8151168452015191818582015201906127c0565b970193019301919392906128f0565b60208183031261264a5780519067ffffffffffffffff821161264a570181601f8201121561264a5780519067ffffffffffffffff821161268557604051926129e1601f84017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016602001856126b2565b8284526020838301011161264a57815f9260208093018386015e8301015290565b67ffffffffffffffff81116126855760051b60200190565b90604051915f8154908160011c9260018316928315612b13575b602085108414612ae6578487528693908115612aa65750600114612a62575b50612a60925003836126b2565b565b90505f9291925260205f20905f915b818310612a8a575050906020612a60928201015f612a53565b6020919350806001915483858901015201910190918492612a71565b60209350612a609592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f612a53565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f1693612a34565b90604051918281549182825260208201905f5260205f20925f905b806007830110612d3457612a60945491818110612cfe575b818110612cc8575b818110612c92575b818110612c5c575b818110612c26575b818110612bf0575b818110612bbb575b10612b8e575b5003836126b2565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f612b86565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b168152019301612b80565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301612b78565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b168152019301612b70565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301612b68565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b168152019301612b60565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b168152019301612b58565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b168152019301612b50565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e0820152019401920185929391612b38565b9081602091031261264a575173ffffffffffffffffffffffffffffffffffffffff8116810361264a5790565b60405190612dfc6040836126b2565b601b82527f68747470733a2f2f6e65772d6578616d706c652e636f6d2f72706300000000006020830152565b60085460ff168015612e375790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561263f575f91612ecf575b50151590565b90506020813d602011612ef9575b81612eea602093836126b2565b8101031261264a57515f612ec9565b3d9150612edd565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561264a57612f605f91612f7260405194859384937ff320d96300000000000000000000000000000000000000000000000000000000855260406004860152604485019061272e565b9060031984830301602485015261272e565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561263f57612f985750565b5f612a60916126b256fe60a03461016b57601f612e9738819003918201601f19168301916001600160401b038311848410176101445780849260209460405283398101031261016b57516001600160a01b0381169081900361016b578015610158575f80546001600160a01b031981168317825560405192916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a36116de8181016001600160401b038111838210176101445782916112b4833903905ff0801561013957604051906105058083016001600160401b0381118482101761014457604092849261299284396001600160a01b031681523060208201520301905ff080156101395760805260405161114490816101708239608051818181610215015281816105c2015281816108610152610a980152f35b6040513d5f823e3d90fd5b634e487b7160e01b5f52604160045260245ffd5b631e4fbdf760e01b5f525f60045260245ffd5b5f80fdfe60806040526004361015610011575f80fd5b5f5f3560e01c80630b04ebfd1461088557806359659e90146108355780636f04245514610457578063715018a6146103d957806383f94db7146101b75780638da5cb5b14610184578063a33a8b60146101445763f2fde38b14610072575f80fd5b346101415760206003193601126101415773ffffffffffffffffffffffffffffffffffffffff6100a06108c1565b6100a8610c20565b1680156101155773ffffffffffffffffffffffffffffffffffffffff8254827fffffffffffffffffffffffff00000000000000000000000000000000000000008216178455167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08380a380f35b6024827f1e4fbdf700000000000000000000000000000000000000000000000000000000815280600452fd5b80fd5b50346101415760206003193601126101415773ffffffffffffffffffffffffffffffffffffffff6040602092600435815260018452205416604051908152f35b503461014157806003193601126101415773ffffffffffffffffffffffffffffffffffffffff6020915416604051908152f35b5034610141576020600319360112610141576101d16108c1565b6101d9610c20565b73ffffffffffffffffffffffffffffffffffffffff8116908115610355573b156102d1578173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b156102c2578180916024604051809481937f3659cfe60000000000000000000000000000000000000000000000000000000083528860048401525af180156102c6576102ad575b507f51ea6ffdc9909d5ca341259f7221902e0676585d833e2bb21fa923c85e862886602083604051908152a180f35b816102b7916108e4565b6102c257815f61027e565b5080fd5b6040513d84823e3d90fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602160248201527f496d706c656d656e746174696f6e206d757374206265206120636f6e7472616360448201527f74000000000000000000000000000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602960248201527f4e657720696d706c656d656e746174696f6e2063616e6e6f74206265207a657260448201527f6f206164647265737300000000000000000000000000000000000000000000006064820152fd5b50346101415780600319360112610141576103f2610c20565b8073ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b3461078057610180600319360112610780576104716108c1565b602435906064359173ffffffffffffffffffffffffffffffffffffffff8316809303610780576084359173ffffffffffffffffffffffffffffffffffffffff83168093036107805760e4359073ffffffffffffffffffffffffffffffffffffffff821680920361078057610124359073ffffffffffffffffffffffffffffffffffffffff8216809203610780576101443567ffffffffffffffff81116107805761051f903690600401610925565b926101643567ffffffffffffffff811161078057610541903690600401610925565b610549610c20565b610554861515610999565b855f52600160205273ffffffffffffffffffffffffffffffffffffffff60405f2054166107b1576040516020810190878252602081526105956040826108e4565b5190206040516104d78082019082821067ffffffffffffffff83111761078457829161060d91610c6d84397f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1681526040602082018190525f9082015260600190565b03905ff580156107755773ffffffffffffffffffffffffffffffffffffffff1696865f52600160205260405f20887fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055873b15610780575f9573ffffffffffffffffffffffffffffffffffffffff9561071e9461070b936040519c8d998a997fbf79fd1c000000000000000000000000000000000000000000000000000000008b521660048a01528b60248a015260443560448a01526064890152608488015260a43560a488015260c43560c488015260e487015261010435610104870152610124860152610180610144860152610184850190610bdd565b9060031984830301610164850152610bdd565b038183865af192831561077557602093610765575b507feaf2b9d4fd6eba5a60870499f6335c6ab4826e029aff65ba0619329dbd421ec383604051848152a2604051908152f35b5f61076f916108e4565b5f610733565b6040513d5f823e3d90fd5b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602760248201527f436f6e66696720616c72656164792065786973747320666f722074686973206360448201527f6861696e204944000000000000000000000000000000000000000000000000006064820152fd5b34610780575f60031936011261078057602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346107805760206003193601126107805760206108a36004356109fe565b73ffffffffffffffffffffffffffffffffffffffff60405191168152f35b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361078057565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761078457604052565b81601f820112156107805780359067ffffffffffffffff8211610784576040519261097860207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f86011601856108e4565b8284526020838301011161078057815f926020809301838601378301015290565b156109a057565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f436861696e2049442063616e6e6f74206265207a65726f0000000000000000006044820152fd5b805f52600160205273ffffffffffffffffffffffffffffffffffffffff60405f205416610bb85780610a4673ffffffffffffffffffffffffffffffffffffffff921515610999565b6040516020810191825260208152610a5f6040826108e4565b5190206040516104d7610a7560208201836108e4565b8082526020820190610c6d8239610b60604051916020808401610b1185610ae58a7f0000000000000000000000000000000000000000000000000000000000000000168473ffffffffffffffffffffffffffffffffffffffff606092168152604060208201525f60408201520190565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081018752866108e4565b60405194859383850197518091895e840190838201905f8252519283915e01015f8152037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081018352826108e4565b5190206040519060208201927fff0000000000000000000000000000000000000000000000000000000000000084523060601b60218401526035830152605582015260558152610bb16075826108e4565b5190201690565b5f52600160205273ffffffffffffffffffffffffffffffffffffffff60405f20541690565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b73ffffffffffffffffffffffffffffffffffffffff5f54163303610c4057565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffdfe60a0806040526104d780380380916100178285610292565b833981016040828203126101eb5761002e826102c9565b602083015190926001600160401b0382116101eb57019080601f830112156101eb57815161005b816102dd565b926100696040519485610292565b8184526020840192602083830101116101eb57815f926020809301855e84010152823b15610274577fa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d5080546001600160a01b0319166001600160a01b038516908117909155604051635c60da1b60e01b8152909190602081600481865afa9081156101f7575f9161023a575b50803b1561021a5750817f1cf3b03a6cf19fa2baba4df148e9dcabedea7f8a5c07840e207e5c089be95d3e5f80a282511561020257602060049260405193848092635c60da1b60e01b82525afa9182156101f7575f926101ae575b505f809161018a945190845af43d156101a6573d9161016e836102dd565b9261017c6040519485610292565b83523d5f602085013e6102f8565b505b608052604051610180908161035782396080518160460152f35b6060916102f8565b9291506020833d6020116101ef575b816101ca60209383610292565b810103126101eb575f80916101e161018a956102c9565b9394509150610150565b5f80fd5b3d91506101bd565b6040513d5f823e3d90fd5b505050341561018c5763b398979f60e01b5f5260045ffd5b634c9c8ce360e01b5f9081526001600160a01b0391909116600452602490fd5b90506020813d60201161026c575b8161025560209383610292565b810103126101eb57610266906102c9565b5f6100f5565b3d9150610248565b631933b43b60e21b5f9081526001600160a01b038416600452602490fd5b601f909101601f19168101906001600160401b038211908210176102b557604052565b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036101eb57565b6001600160401b0381116102b557601f01601f191660200190565b9061031c575080511561030d57805190602001fd5b63d6bda27560e01b5f5260045ffd5b8151158061034d575b61032d575090565b639996b31560e01b5f9081526001600160a01b0391909116600452602490fd5b50803b1561032556fe60806040527f5c60da1b000000000000000000000000000000000000000000000000000000006080526020608060048173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa8015610107575f9015610163575060203d602011610100575b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f820116608001906080821067ffffffffffffffff8311176100d3576100ce91604052608001610112565b610163565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b503d610081565b6040513d5f823e3d90fd5b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80602091011261015f5760805173ffffffffffffffffffffffffffffffffffffffff8116810361015f5790565b5f80fd5b5f8091368280378136915af43d5f803e1561017c573d5ff35b3d5ffd6080806040523460aa575f5160206116be5f395f51905f525460ff8160401c16609b576002600160401b03196001600160401b038216016049575b60405161160f90816100af8239f35b6001600160401b0319166001600160401b039081175f5160206116be5f395f51905f525581527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602090a15f80603a565b63f92ee8a960e01b5f5260045ffd5b5f80fdfe60806040526004361015610011575f80fd5b5f3560e01c80630743bf6e1461120357806318b5ce81146111d057806329080356146110845780634b8be3f714610f2657806354fd4d5014610e8157806357d1ba2514610e645780636edd6c0914610e315780637240f9af14610cd157806385e1f4d014610cb45780638da5cb5b14610c82578063a3c6e1e714610c65578063aa6a43d814610c32578063bf6db6f814610bff578063bf79fd1c1461027b578063c7a7609514610183578063d1f4737c14610166578063f2fde38b146101005763f8a144be146100df575f80fd5b346100fc575f6003193601126100fc576020600654604051908152f35b5f80fd5b346100fc5760206003193601126100fc5761016461011c611397565b61013e73ffffffffffffffffffffffffffffffffffffffff5f54163314611410565b61015f73ffffffffffffffffffffffffffffffffffffffff82161515611524565b611589565b005b346100fc575f6003193601126100fc576020600854604051908152f35b346100fc575f6003193601126100fc576040515f600a546101a3816112a8565b808452906001811690811561023957506001146101db575b6101d7836101cb818503826112f9565b6040519182918261131c565b0390f35b600a5f9081527fc65a7bb8d6351c1cf70c95a316cc6a92839c986682d98bc35f958f4883f9d2a8939250905b80821061021f575090915081016020016101cb6101bb565b919260018160209254838588010152019101909291610207565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208086019190915291151560051b840190910191506101cb90506101bb565b346100fc576101806003193601126100fc57610295611397565b602435604435916064359273ffffffffffffffffffffffffffffffffffffffff84168094036100fc5760843573ffffffffffffffffffffffffffffffffffffffff81168091036100fc5760e43573ffffffffffffffffffffffffffffffffffffffff81168091036100fc57610124359173ffffffffffffffffffffffffffffffffffffffff83168093036100fc576101443567ffffffffffffffff81116100fc576103449036906004016113ba565b966101643567ffffffffffffffff81116100fc576103669036906004016113ba565b947ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00549760ff8960401c16159867ffffffffffffffff811680159081610bf7575b6001149081610bed575b159081610be4575b50610bbc578960017fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000008316177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0055610b67575b5073ffffffffffffffffffffffffffffffffffffffff881615610b09578015610aab578115610a275782156109a357831561091f57841561089b578515610817576005556006557fffffffffffffffffffffffff000000000000000000000000000000000000000060025416176002557fffffffffffffffffffffffff0000000000000000000000000000000000000000600354161760035560a43560075560c4356008557fffffffffffffffffffffffff00000000000000000000000000000000000000006004541617600455610104356009556104ec600c546112a8565b601f81116107d9575b50600a7f312e302e3000000000000000000000000000000000000000000000000000000001600c557fffffffffffffffffffffffff00000000000000000000000000000000000000006001541617600155835167ffffffffffffffff811161072b5761056b81610566600a546112a8565b61148b565b602094601f82116001146107585761059c9293949582915f92610699575b50505f198260011b9260031b1c19161790565b600a555b80519067ffffffffffffffff821161072b576105c6826105c1600b546112a8565b6114c5565b602090601f83116001146106a45791806105f89261060095945f926106995750505f198260011b9260031b1c19161790565b600b55611589565b61060657005b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160018152a1005b015190508680610589565b90601f19831691600b5f527f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9925f5b8181106107135750916001939185610600979694106106fb575b505050811b01600b55611589565b01515f1960f88460031b161c191690558580806106ed565b929360206001819287860151815501950193016106d3565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b601f19821695600a5f527fc65a7bb8d6351c1cf70c95a316cc6a92839c986682d98bc35f958f4883f9d2a8915f5b8881106107c1575083600195969798106107a9575b505050811b01600a556105a0565b01515f1960f88460031b161c1916905585808061079b565b91926020600181928685015181550194019201610786565b600c5f5261081190601f0160051c7fdf6966c971051c3d54ec59162606531493a51404a002842f56009d7e5cf4a8c790810190611475565b856104f5565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602d60248201527f496e697469616c20617070636861696e206f776e65722063616e6e6f7420626560448201527f207a65726f2061646472657373000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602a60248201527f53657175656e63696e6720636f6e747261637420616464726573732063616e6e60448201527f6f74206265207a65726f000000000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602560248201527f417262697472756d20696e626f7820616464726573732063616e6e6f7420626560448201527f207a65726f0000000000000000000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f417262697472756d2062726964676520616464726573732063616e6e6f74206260448201527f65207a65726f00000000000000000000000000000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f53657175656e63696e6720636861696e2049442063616e6e6f74206265207a6560448201527f726f0000000000000000000000000000000000000000000000000000000000006064820152fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f436861696e2049442063616e6e6f74206265207a65726f0000000000000000006044820152fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601c60248201527f4f776e65722063616e6e6f74206265207a65726f2061646472657373000000006044820152fd5b7fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000001668010000000000000001177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00558a61040c565b7ff92ee8a9000000000000000000000000000000000000000000000000000000005f5260045ffd5b9050158c6103b9565b303b1591506103b1565b8b91506103a7565b346100fc575f6003193601126100fc57602073ffffffffffffffffffffffffffffffffffffffff60045416604051908152f35b346100fc575f6003193601126100fc57602073ffffffffffffffffffffffffffffffffffffffff60035416604051908152f35b346100fc575f6003193601126100fc576020600954604051908152f35b346100fc575f6003193601126100fc57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b346100fc575f6003193601126100fc576020600554604051908152f35b346100fc57610cdf36611346565b610d0173ffffffffffffffffffffffffffffffffffffffff5f54163314611410565b67ffffffffffffffff811161072b57610d1b600c546112a8565b601f8111610dd8575b505f601f8211600114610d60578190610d50935f92610d555750505f198260011b9260031b1c19161790565b600c55005b013590508380610589565b601f198216927fdf6966c971051c3d54ec59162606531493a51404a002842f56009d7e5cf4a8c7915f5b858110610dc057508360019510610da7575b505050811b01600c55005b5f1960f88560031b161c19910135169055828080610d9c565b90926020600181928686013581550194019101610d8a565b600c5f52610e21907fdf6966c971051c3d54ec59162606531493a51404a002842f56009d7e5cf4a8c7601f840160051c81019160208510610e27575b601f0160051c0190611475565b82610d24565b9091508190610e14565b346100fc575f6003193601126100fc57602073ffffffffffffffffffffffffffffffffffffffff60025416604051908152f35b346100fc575f6003193601126100fc576020600754604051908152f35b346100fc575f6003193601126100fc576040515f600c54610ea1816112a8565b80845290600181169081156102395750600114610ec8576101d7836101cb818503826112f9565b600c5f9081527fdf6966c971051c3d54ec59162606531493a51404a002842f56009d7e5cf4a8c7939250905b808210610f0c575090915081016020016101cb6101bb565b919260018160209254838588010152019101909291610ef4565b346100fc57610f3436611346565b610f5673ffffffffffffffffffffffffffffffffffffffff5f54163314611410565b67ffffffffffffffff811161072b57610f74816105c1600b546112a8565b5f91601f8211600114610fe457610fc182807f02585ebce918f656604dabb23332a6df1dcde119741f7c1f8fb37b191392a2a7955f91610fd9575b505f198260011b9260031b1c19161790565b600b555b610fd4604051928392836114fd565b0390a1005b905083013586610faf565b601f198216600b5f527f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9905f5b81811061106c575093837f02585ebce918f656604dabb23332a6df1dcde119741f7c1f8fb37b191392a2a79510611053575b5050600182811b01600b55610fc5565b5f1960f88560031b161c19908301351690558380611043565b83860135835560209586019560019093019201611011565b346100fc5761109236611346565b6110b473ffffffffffffffffffffffffffffffffffffffff5f54163314611410565b67ffffffffffffffff811161072b576110d281610566600a546112a8565b5f91601f82116001146111305761111e82807f67c57ae6ad924cd093fb2f06bc0b28fd5879481051a9c203a44d27c8904d437c955f91610fd957505f198260011b9260031b1c19161790565b600a55610fd4604051928392836114fd565b601f198216600a5f527fc65a7bb8d6351c1cf70c95a316cc6a92839c986682d98bc35f958f4883f9d2a8905f5b8181106111b8575093837f67c57ae6ad924cd093fb2f06bc0b28fd5879481051a9c203a44d27c8904d437c951061119f575b5050600182811b01600a55610fc5565b5f1960f88560031b161c1990830135169055838061118f565b8386013583556020958601956001909301920161115d565b346100fc575f6003193601126100fc57602073ffffffffffffffffffffffffffffffffffffffff60015416604051908152f35b346100fc575f6003193601126100fc576040515f600b54611223816112a8565b8084529060018116908115610239575060011461124a576101d7836101cb818503826112f9565b600b5f9081527f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9939250905b80821061128e575090915081016020016101cb6101bb565b919260018160209254838588010152019101909291611276565b90600182811c921680156112ef575b60208310146112c257565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f16916112b7565b90601f601f19910116810190811067ffffffffffffffff82111761072b57604052565b601f19601f602060409481855280519182918282880152018686015e5f8582860101520116010190565b9060206003198301126100fc5760043567ffffffffffffffff81116100fc57826023820112156100fc5780600401359267ffffffffffffffff84116100fc57602484830101116100fc576024019190565b6004359073ffffffffffffffffffffffffffffffffffffffff821682036100fc57565b81601f820112156100fc5780359067ffffffffffffffff821161072b57604051926113ef6020601f19601f86011601856112f9565b828452602083830101116100fc57815f926020809301838601378301015290565b1561141757565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f43616c6c6572206973206e6f7420746865206f776e65720000000000000000006044820152fd5b818110611480575050565b5f8155600101611475565b90601f8211611498575050565b6114c391600a5f5260205f20906020601f840160051c83019310610e2757601f0160051c0190611475565b565b90601f82116114d2575050565b6114c391600b5f5260205f20906020601f840160051c83019310610e2757601f0160051c0190611475565b90601f83604094601f199360208652816020870152868601375f8582860101520116010190565b1561152b57565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4e6577206f776e65722063616e6e6f74206265207a65726f20616464726573736044820152fd5b73ffffffffffffffffffffffffffffffffffffffff166115aa811515611524565b73ffffffffffffffffffffffffffffffffffffffff5f54827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a356f0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0060803461013457601f61050538819003918201601f19168301916001600160401b03831184841017610138578084926040948552833981010312610134576100468161014c565b906001600160a01b039061005c9060200161014c565b16908115610121575f80546001600160a01b031981168417825560405193916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a3803b1561010157600180546001600160a01b0319166001600160a01b039290921691821790557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a26103a490816101618239f35b63211eb15960e21b5f9081526001600160a01b0391909116600452602490fd5b631e4fbdf760e01b5f525f60045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036101345756fe60806040526004361015610011575f80fd5b5f3560e01c80633659cfe61461027e5780635c60da1b1461022d578063715018a6146101935780638da5cb5b146101435763f2fde38b14610050575f80fd5b3461013f5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261013f5760043573ffffffffffffffffffffffffffffffffffffffff811680910361013f576100a8610358565b80156101135773ffffffffffffffffffffffffffffffffffffffff5f54827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f80fd5b3461013f575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261013f57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b3461013f575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261013f576101c9610358565b5f73ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461013f575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261013f57602073ffffffffffffffffffffffffffffffffffffffff60015416604051908152f35b3461013f5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261013f5760043573ffffffffffffffffffffffffffffffffffffffff81169081810361013f576102d7610358565b3b1561032d57807fffffffffffffffffffffffff000000000000000000000000000000000000000060015416176001557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a2005b7f847ac564000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff5f5416330361037857565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x07C\xBFn\x14a&NW\x80c\n\x92T\xE4\x14a!\xBAW\x80c\x1E\xD7\x83\x1C\x14a!/W\x80c&\xDFN%\x14a\x1F\x92W\x80c*\xDE8\x80\x14a\x1D\x84W\x80c4xK\xF1\x14a\x1B7W\x80c>^<#\x14a\x1A\xACW\x80c?r\x86\xF4\x14a\x1A!W\x80cM\xBD\xC8]\x14a\x18aW\x80cR\x06\x9F\xC2\x14a\x16\x80W\x80cW\xD1\xBA%\x14a\x16dW\x80c[|\x15\x99\x14a\x13\x16W\x80c`\x94L\xE6\x14a\x115W\x80cf\xD9\xA9\xA0\x14a\x0F\xF8W\x80ch\x06\xBA\x06\x14a\x0F\xC8W\x80cn\xDDl\t\x14a\x0F\xABW\x80c\x85\"l\x81\x14a\x0F!W\x80c\x85\"\xAC\x8C\x14a\n\xF2W\x80c\x85\xE1\xF4\xD0\x14a\n\xD4W\x80c\x8D\xA5\xCB[\x14a\n\x9DW\x80c\x91j\x17\xC6\x14a\t\xE6W\x80c\xA3\xC6\xE1\xE7\x14a\t\xCAW\x80c\xAAjC\xD8\x14a\t\xADW\x80c\xB0FO\xDC\x14a\x08\xF6W\x80c\xB5P\x8A\xA9\x14a\x08lW\x80c\xBAAO\xA6\x14a\x08GW\x80c\xBFm\xB6\xF8\x14a\x08*W\x80c\xCA\n\xB0u\x14a\x07\xF6W\x80c\xD1\xF4s|\x14a\x07\xDAW\x80c\xD81\x97^\x14a\x07\xA7W\x80c\xE2\x0C\x9Fq\x14a\x07\x0CW\x80c\xEB\xB478\x14a\x06\xD8W\x80c\xF8\xA1D\xBE\x14a\x06\xBAW\x80c\xFAv&\xD4\x14a\x06\x97W\x80c\xFD\xFC\x99\xDF\x14a\x03zWc\xFE\xBCD2\x14a\x01\xA6W_\x80\xFD[4a\x03?W\x80`\x03\x196\x01\x12a\x03?Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x03\xE7`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x03eW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`D\x82\x01R\x81\x90\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x03PW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x03MW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xF2\xFD\xE3\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x80\x84\x01RZ\xF1\x80\x15a\x03BWa\x03.WP\xF3[\x81a\x038\x91a&\xB2V[a\x03?W\x80\xF3[\x80\xFD[`@Q=\x84\x82>=\x90\xFD[P\xFD[\x81a\x03Z\x91a&\xB2V[a\x03?W\x80_a\x02\xC9V[\x81a\x03o\x91a&\xB2V[a\x03?W\x80_a\x02(V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x06\x82W[PPa\x04!a-\xEDV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06iW\x81`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a\x04\x8A`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R_` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x06mW[PP\x7Fg\xC5z\xE6\xAD\x92L\xD0\x93\xFB/\x06\xBC\x0B(\xFDXyH\x10Q\xA9\xC2\x03\xA4M'\xC8\x90MC|`@Q` \x81R\x80a\x04\xE7` \x82\x01\x85a'.V[\x03\x90\xA1\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x06iW\x81`@Q\x80\x92\x7F)\x08\x03V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` `\x04\x83\x01R\x81\x83\x81a\x05K`$\x82\x01\x8Aa'.V[\x03\x92Z\xF1\x80\x15a\x03BWa\x06TW[P\x90`\x04\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16`@Q\x93\x84\x80\x92\x7F\xC7\xA7`\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06IWa\x05\xBB\x92\x84\x91a\x06'W[Pa/\x01V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x03.WP\xF3[a\x06C\x91P=\x80\x86\x83>a\x06;\x81\x83a&\xB2V[\x81\x01\x90a)qV[_a\x05\xB5V[`@Q=\x85\x82>=\x90\xFD[\x81a\x06^\x91a&\xB2V[a\x06iW\x81_a\x05ZV[P\x80\xFD[\x81a\x06w\x91a&\xB2V[a\x06iW\x81_a\x04\xAFV[\x81a\x06\x8C\x91a&\xB2V[a\x03?W\x80_a\x04\x17V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Qb\t\xFB\xF1\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16`@Q\x90\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x07{Wa\x07w\x85a\x07k\x81\x87\x03\x82a&\xB2V[`@Q\x91\x82\x91\x82a'qV[\x03\x90\xF3[\x82Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x07TV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x16`@Q\x90\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Q`d\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`!T\x16`@Q\x90\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Qa\x9A\xBC\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` a\x08ba.(V[`@Q\x90\x15\x15\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`\x19Ta\x08\x89\x81a*\x02V[\x91a\x08\x97`@Q\x93\x84a&\xB2V[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\x08\xD9W`@Q\x80a\x07w\x87\x82a(PV[`\x01` \x81\x92a\x08\xE8\x85a*\x1AV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x08\xC4V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`\x1CTa\t\x13\x81a*\x02V[\x91a\t!`@Q\x93\x84a&\xB2V[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\tcW`@Q\x80a\x07w\x87\x82a(\xCDV[`\x02` `\x01\x92`@Qa\tv\x81a&iV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86T\x16\x81Ra\t\x9B\x85\x87\x01a+\x1DV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\tNV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@QaVx\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Q`\xC8\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`\x1DTa\n\x03\x81a*\x02V[\x91a\n\x11`@Q\x93\x84a&\xB2V[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\nSW`@Q\x80a\x07w\x87\x82a(\xCDV[`\x02` `\x01\x92`@Qa\nf\x81a&iV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86T\x16\x81Ra\n\x8B\x85\x87\x01a+\x1DV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\n>V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Qb\x01\xE2@\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x0F\x0CW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R`\x01`$\x82\x01R\x81`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x0E\xF7W[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16`\x04`@Q\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x84\x80\xA3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x0E\xF3W\x81`$\x81\x85\x80\x94\x7F\xF2\xFD\xE3\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x80\x84\x01RZ\xF1\x80\x15a\x03BWa\x0E\xDEW[P`\x04` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16`@Q\x92\x83\x80\x92\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x03BW\x82\x91a\x0E\xAFW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01R`\x04`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x03BWa\x0E\x9AW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x0E\x85W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x80\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x06\x82WPPa\x04!a-\xEDV[\x81a\x0E\x8F\x91a&\xB2V[a\x03?W\x80_a\x0E\nV[\x81a\x0E\xA4\x91a&\xB2V[a\x03?W\x80_a\r\x9EV[a\x0E\xD1\x91P` =` \x11a\x0E\xD7W[a\x0E\xC9\x81\x83a&\xB2V[\x81\x01\x90a-\xC1V[_a\r\x12V[P=a\x0E\xBFV[\x81a\x0E\xE8\x91a&\xB2V[a\x03?W\x80_a\x0C\xBAV[PP\xFD[\x81a\x0F\x01\x91a&\xB2V[a\x03?W\x80_a\x0C\x16V[\x81a\x0F\x16\x91a&\xB2V[a\x03?W\x80_a\x0B\x8FV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`\x1ATa\x0F>\x81a*\x02V[\x91a\x0FL`@Q\x93\x84a&\xB2V[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\x0F\x8EW`@Q\x80a\x07w\x87\x82a(PV[`\x01` \x81\x92a\x0F\x9D\x85a*\x1AV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x0FyV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Qa\x124\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?Wa\x07wa\x0F\xE4a(\x15V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a'.V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`\x1BTa\x10\x15\x81a*\x02V[a\x10\"`@Q\x91\x82a&\xB2V[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a\x10\xFAW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a\x10\x8FWPPPP\x03\x90\xF3[\x91\x93` a\x10\xEA\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a\x10\xDA\x83Q`@\x84R`@\x84\x01\x90a'.V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra'\xC0V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x10\x80V[`\x02` `\x01\x92`@Qa\x11\r\x81a&iV[a\x11\x16\x86a*\x1AV[\x81Ra\x11#\x85\x87\x01a+\x1DV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x10RV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x03\xE7`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x13\x01W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`D\x82\x01R\x81\x90\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x12\xECW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x03MW\x81\x80\x91`d`@Q\x80\x94\x81\x93\x7F)\x08\x03V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`\x1B`$\x84\x01R\x7Fhttps://new-example.com/rpc\0\0\0\0\0`D\x84\x01RZ\xF1\x80\x15a\x03BWa\x03.WP\xF3[\x81a\x12\xF6\x91a&\xB2V[a\x03?W\x80_a\x12YV[\x81a\x13\x0B\x91a&\xB2V[a\x03?W\x80_a\x11\xB8V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x16OW[PP\x80`@\x80Qa\x13\xC4\x82\x82a&\xB2V[` \x81R\x7Fhttps://new-example.com/explorer` \x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0E\xF3W\x81Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83\x81\x80a\x14U`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93\x82\x81R_` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x16\x13W\x90\x84\x91a\x16:W[PP\x7F\x02X^\xBC\xE9\x18\xF6V`M\xAB\xB232\xA6\xDF\x1D\xCD\xE1\x19t\x1F|\x1F\x8F\xB3{\x19\x13\x92\xA2\xA7\x82Q` \x81R\x80a\x14\xB4` \x82\x01\x85a'.V[\x03\x90\xA1s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x165W\x83\x83Q\x80\x92\x7FK\x8B\xE3\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` `\x04\x83\x01R\x81\x83\x81a\x15\x16`$\x82\x01\x89a'.V[\x03\x92Z\xF1\x80\x15a\x16\x13W\x90\x84\x91a\x16 W[P`\x04\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x84Q\x92\x83\x80\x92\x7F\x07C\xBFn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x16\x13W\x90a\x15\x89\x92\x91\x85\x91a\x15\xFFWPa/\x01V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW\x80Q\x90\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x82\x82`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x15\xF6WPa\x03.WP\xF3[Q=\x84\x82>=\x90\xFD[a\x06C\x91P=\x80\x87\x83>a\x06;\x81\x83a&\xB2V[PPPQ\x90=\x90\x82>=\x90\xFD[\x81a\x16*\x91a&\xB2V[a\x0E\xF3W\x82_a\x15(V[PPP\xFD[\x81a\x16D\x91a&\xB2V[a\x0E\xF3W\x82_a\x14}V[\x81a\x16Y\x91a&\xB2V[a\x03?W\x80_a\x13\xB3V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W` `@Q`\n\x81R\xF3[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x03\xE7`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x18LW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`D\x82\x01R\x81\x90\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x187W[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x03MW\x81\x80\x91`d`@Q\x80\x94\x81\x93\x7FK\x8B\xE3\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R` `$\x84\x01R\x7Fhttps://new-example.com/explorer`D\x84\x01RZ\xF1\x80\x15a\x03BWa\x03.WP\xF3[\x81a\x18A\x91a&\xB2V[a\x03?W\x80_a\x17\xA4V[\x81a\x18V\x91a&\xB2V[a\x03?W\x80_a\x17\x03V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x1A\x0CW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x03MW\x81`@Q\x80\x92\x7FK\x8B\xE3\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x83\x81`@`\x04\x82\x01` \x81R_` \x82\x01R\x01\x03\x92Z\xF1\x80\x15a\x03BWa\x19\xF7W[P`\x04\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16`@Q\x92\x83\x80\x92\x7F\x07C\xBFn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03BWa\x05\xBB\x91\x83\x91a\x19\xDDW[P`@Q\x90a\x19\xD5` \x83a&\xB2V[\x83\x82Ra/\x01V[a\x19\xF1\x91P=\x80\x85\x83>a\x06;\x81\x83a&\xB2V[_a\x19\xC5V[\x81a\x1A\x01\x91a&\xB2V[a\x03?W\x80_a\x19kV[\x81a\x1A\x16\x91a&\xB2V[a\x03?W\x80_a\x18\xFEV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a\x1A\x80Wa\x07w\x85a\x07k\x81\x87\x03\x82a&\xB2V[\x82Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x1AiV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a\x1B\x0BWa\x07w\x85a\x07k\x81\x87\x03\x82a&\xB2V[\x82Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x1A\xF4V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x1DoW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FNew owner cannot be zero address`D\x82\x01R\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x1DZW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x03MW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xF2\xFD\xE3\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x03BWa\x1DEW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03?W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa\x03.WP\xF3[\x81a\x1DO\x91a&\xB2V[a\x03?W\x80_a\x1C\xD7V[\x81a\x1Dd\x91a&\xB2V[a\x03?W\x80_a\x1CtV[\x81a\x1Dy\x91a&\xB2V[a\x03?W\x80_a\x1B\xD4V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`\x1ETa\x1D\xA1\x81a*\x02V[a\x1D\xAE`@Q\x91\x82a&\xB2V[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a\x1E\xFCW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a\x1E\x1AW\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a\x1E\xB3WPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a\x1E\rV[\x90\x91\x92\x93\x94` \x80a\x1E\xEF\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa'.V[\x97\x01\x95\x01\x93\x92\x91\x01a\x1E\x8FV[`@Qa\x1F\x08\x81a&iV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83T\x16\x81R`\x01\x83\x01\x80Ta\x1F1\x81a*\x02V[\x91a\x1F?`@Q\x93\x84a&\xB2V[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a\x1FuWPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x1D\xDEV[`\x01` \x81\x92a\x1F\x84\x86a*\x1AV[\x81R\x01\x93\x01\x91\x01\x90\x91a\x1FOV[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x03MW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa!\x1AW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16\x80;\x15a\x03MW\x81`@Q\x80\x92\x7F)\x08\x03V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x83\x81`@`\x04\x82\x01` \x81R_` \x82\x01R\x01\x03\x92Z\xF1\x80\x15a\x03BWa!\x05W[P`\x04\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\"T\x16`@Q\x92\x83\x80\x92\x7F\xC7\xA7`\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x03BWa\x05\xBB\x91\x83\x91a\x19\xDDWP`@Q\x90a\x19\xD5` \x83a&\xB2V[\x81a!\x0F\x91a&\xB2V[a\x03?W\x80_a \x9CV[\x81a!$\x91a&\xB2V[a\x03?W\x80_a /V[P4a\x03?W\x80`\x03\x196\x01\x12a\x03?W`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a!\x8EWa\x07w\x85a\x07k\x81\x87\x03\x82a&\xB2V[\x82Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a!wV[P4a&JW_`\x03\x196\x01\x12a&JWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a&JW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a&?Wa&,W[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16`@Q\x90a.\x97\x90\x81\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a%\xFFW\x91\x83\x91` \x93a/\xA3\x849\x81R\x03\x01\x90\x82\xF0\x80\x15a%\xF2Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06iW\x81`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa%\xDDW[PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06iW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa%\xC8W[PP` a$\xC0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x1FT`\x08\x1C\x16\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83T\x16\x85a$(a(\x15V[\x93a$\xD3a$4a&\xF3V[`@Q\x98\x89\x97\x88\x96\x87\x95\x7Fo\x04$U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87R`\x04\x87\x01Rb\x01\xE2@`$\x87\x01Rb\t\xFB\xF1`D\x87\x01Ra\x124`d\x87\x01RaVx`\x84\x87\x01R`\n`\xA4\x87\x01R`d`\xC4\x87\x01Ra\x9A\xBC`\xE4\x87\x01R`\xC8a\x01\x04\x87\x01Ra\x01$\x86\x01Ra\x01\x80a\x01D\x86\x01Ra\x01\x84\x85\x01\x90a'.V[\x90`\x03\x19\x84\x83\x03\x01a\x01d\x85\x01Ra'.V[\x03\x92Z\xF1\x90\x81\x15a\x03BW\x82\x91a%\xA9W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06iW\x81`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x03BWa%\x94W[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"T\x16\x17`\"U\x80\xF3[\x81a%\x9E\x91a&\xB2V[a\x06iW\x81_a%PV[a%\xC2\x91P` =` \x11a\x0E\xD7Wa\x0E\xC9\x81\x83a&\xB2V[_a$\xE5V[\x81a%\xD2\x91a&\xB2V[a\x06iW\x81_a#\xE2V[\x81a%\xE7\x91a&\xB2V[a\x06iW\x81_a#\x14V[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[a&8\x91P_\x90a&\xB2V[__a\"VV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[4a&JW_`\x03\x196\x01\x12a&JWa\x07wa\x0F\xE4a&\xF3V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a&\x85W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a&\x85W`@RV[`@Q\x90a'\x02`@\x83a&\xB2V[`\x1C\x82R\x7Fhttps://example.com/explorer\0\0\0\0` \x83\x01RV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a'\x94WPPP\x90V[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a'\x87V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a'\xDDWPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a'\xD0V[`@Q\x90a($`@\x83a&\xB2V[`\x17\x82R\x7Fhttps://example.com/rpc\0\0\0\0\0\0\0\0\0` \x83\x01RV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a(\x82WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a(\xBE\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa'.V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a(sV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a(\xFFWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a)b\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a'\xC0V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a(\xF0V[` \x81\x83\x03\x12a&JW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a&JW\x01\x81`\x1F\x82\x01\x12\x15a&JW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a&\x85W`@Q\x92a)\xE1`\x1F\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x85a&\xB2V[\x82\x84R` \x83\x83\x01\x01\x11a&JW\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a&\x85W`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15a+\x13W[` \x85\x10\x84\x14a*\xE6W\x84\x87R\x86\x93\x90\x81\x15a*\xA6WP`\x01\x14a*bW[Pa*`\x92P\x03\x83a&\xB2V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a*\x8AWPP\x90` a*`\x92\x82\x01\x01_a*SV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a*qV[` \x93Pa*`\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a*SV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93a*4V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10a-4Wa*`\x94T\x91\x81\x81\x10a,\xFEW[\x81\x81\x10a,\xC8W[\x81\x81\x10a,\x92W[\x81\x81\x10a,\\W[\x81\x81\x10a,&W[\x81\x81\x10a+\xF0W[\x81\x81\x10a+\xBBW[\x10a+\x8EW[P\x03\x83a&\xB2V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_a+\x86V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01a+\x80V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01a+xV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01a+pV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01a+hV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01a+`V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01a+XV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01a+PV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91a+8V[\x90\x81` \x91\x03\x12a&JWQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a&JW\x90V[`@Q\x90a-\xFC`@\x83a&\xB2V[`\x1B\x82R\x7Fhttps://new-example.com/rpc\0\0\0\0\0` \x83\x01RV[`\x08T`\xFF\x16\x80\x15a.7W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a&?W_\x91a.\xCFW[P\x15\x15\x90V[\x90P` \x81=` \x11a.\xF9W[\x81a.\xEA` \x93\x83a&\xB2V[\x81\x01\x03\x12a&JWQ_a.\xC9V[=\x91Pa.\xDDV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a&JWa/`_\x91a/r`@Q\x94\x85\x93\x84\x93\x7F\xF3 \xD9c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`@`\x04\x86\x01R`D\x85\x01\x90a'.V[\x90`\x03\x19\x84\x83\x03\x01`$\x85\x01Ra'.V[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a&?Wa/\x98WPV[_a*`\x91a&\xB2V\xFE`\xA04a\x01kW`\x1Fa.\x978\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01DW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\x01kWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x01kW\x80\x15a\x01XW_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x82U`@Q\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3a\x16\xDE\x81\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x83\x82\x10\x17a\x01DW\x82\x91a\x12\xB4\x839\x03\x90_\xF0\x80\x15a\x019W`@Q\x90a\x05\x05\x80\x83\x01`\x01`\x01`@\x1B\x03\x81\x11\x84\x82\x10\x17a\x01DW`@\x92\x84\x92a)\x92\x849`\x01`\x01`\xA0\x1B\x03\x16\x81R0` \x82\x01R\x03\x01\x90_\xF0\x80\x15a\x019W`\x80R`@Qa\x11D\x90\x81a\x01p\x829`\x80Q\x81\x81\x81a\x02\x15\x01R\x81\x81a\x05\xC2\x01R\x81\x81a\x08a\x01Ra\n\x98\x01R\xF3[`@Q=_\x82>=\x90\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x0B\x04\xEB\xFD\x14a\x08\x85W\x80cYe\x9E\x90\x14a\x085W\x80co\x04$U\x14a\x04WW\x80cqP\x18\xA6\x14a\x03\xD9W\x80c\x83\xF9M\xB7\x14a\x01\xB7W\x80c\x8D\xA5\xCB[\x14a\x01\x84W\x80c\xA3:\x8B`\x14a\x01DWc\xF2\xFD\xE3\x8B\x14a\0rW_\x80\xFD[4a\x01AW` `\x03\x196\x01\x12a\x01AWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\xA0a\x08\xC1V[a\0\xA8a\x0C V[\x16\x80\x15a\x01\x15Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x84U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x83\x80\xA3\x80\xF3[`$\x82\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x80`\x04R\xFD[\x80\xFD[P4a\x01AW` `\x03\x196\x01\x12a\x01AWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@` \x92`\x045\x81R`\x01\x84R T\x16`@Q\x90\x81R\xF3[P4a\x01AW\x80`\x03\x196\x01\x12a\x01AWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x91T\x16`@Q\x90\x81R\xF3[P4a\x01AW` `\x03\x196\x01\x12a\x01AWa\x01\xD1a\x08\xC1V[a\x01\xD9a\x0C V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a\x03UW;\x15a\x02\xD1W\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x02\xC2W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F6Y\xCF\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x88`\x04\x84\x01RZ\xF1\x80\x15a\x02\xC6Wa\x02\xADW[P\x7FQ\xEAo\xFD\xC9\x90\x9D\\\xA3A%\x9Fr!\x90.\x06vX]\x83>+\xB2\x1F\xA9#\xC8^\x86(\x86` \x83`@Q\x90\x81R\xA1\x80\xF3[\x81a\x02\xB7\x91a\x08\xE4V[a\x02\xC2W\x81_a\x02~V[P\x80\xFD[`@Q=\x84\x82>=\x90\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FImplementation must be a contrac`D\x82\x01R\x7Ft\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FNew implementation cannot be zer`D\x82\x01R\x7Fo address\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[P4a\x01AW\x80`\x03\x196\x01\x12a\x01AWa\x03\xF2a\x0C V[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[4a\x07\x80Wa\x01\x80`\x03\x196\x01\x12a\x07\x80Wa\x04qa\x08\xC1V[`$5\x90`d5\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\x07\x80W`\x845\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\x07\x80W`\xE45\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x07\x80Wa\x01$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x07\x80Wa\x01D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x80Wa\x05\x1F\x906\x90`\x04\x01a\t%V[\x92a\x01d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x80Wa\x05A\x906\x90`\x04\x01a\t%V[a\x05Ia\x0C V[a\x05T\x86\x15\x15a\t\x99V[\x85_R`\x01` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16a\x07\xB1W`@Q` \x81\x01\x90\x87\x82R` \x81Ra\x05\x95`@\x82a\x08\xE4V[Q\x90 `@Qa\x04\xD7\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x07\x84W\x82\x91a\x06\r\x91a\x0Cm\x849\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`@` \x82\x01\x81\x90R_\x90\x82\x01R``\x01\x90V[\x03\x90_\xF5\x80\x15a\x07uWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x96\x86_R`\x01` R`@_ \x88\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90U\x87;\x15a\x07\x80W_\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95a\x07\x1E\x94a\x07\x0B\x93`@Q\x9C\x8D\x99\x8A\x99\x7F\xBFy\xFD\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8BR\x16`\x04\x8A\x01R\x8B`$\x8A\x01R`D5`D\x8A\x01R`d\x89\x01R`\x84\x88\x01R`\xA45`\xA4\x88\x01R`\xC45`\xC4\x88\x01R`\xE4\x87\x01Ra\x01\x045a\x01\x04\x87\x01Ra\x01$\x86\x01Ra\x01\x80a\x01D\x86\x01Ra\x01\x84\x85\x01\x90a\x0B\xDDV[\x90`\x03\x19\x84\x83\x03\x01a\x01d\x85\x01Ra\x0B\xDDV[\x03\x81\x83\x86Z\xF1\x92\x83\x15a\x07uW` \x93a\x07eW[P\x7F\xEA\xF2\xB9\xD4\xFDn\xBAZ`\x87\x04\x99\xF63\\j\xB4\x82n\x02\x9A\xFFe\xBA\x06\x192\x9D\xBDB\x1E\xC3\x83`@Q\x84\x81R\xA2`@Q\x90\x81R\xF3[_a\x07o\x91a\x08\xE4V[_a\x073V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FConfig already exists for this c`D\x82\x01R\x7Fhain ID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[4a\x07\x80W_`\x03\x196\x01\x12a\x07\x80W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x07\x80W` `\x03\x196\x01\x12a\x07\x80W` a\x08\xA3`\x045a\t\xFEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x07\x80WV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\x84W`@RV[\x81`\x1F\x82\x01\x12\x15a\x07\x80W\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x07\x84W`@Q\x92a\tx` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x86\x01\x16\x01\x85a\x08\xE4V[\x82\x84R` \x83\x83\x01\x01\x11a\x07\x80W\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[\x15a\t\xA0WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FChain ID cannot be zero\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x80_R`\x01` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16a\x0B\xB8W\x80a\nFs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x15\x15a\t\x99V[`@Q` \x81\x01\x91\x82R` \x81Ra\n_`@\x82a\x08\xE4V[Q\x90 `@Qa\x04\xD7a\nu` \x82\x01\x83a\x08\xE4V[\x80\x82R` \x82\x01\x90a\x0Cm\x829a\x0B``@Q\x91` \x80\x84\x01a\x0B\x11\x85a\n\xE5\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x92\x16\x81R`@` \x82\x01R_`@\x82\x01R\x01\x90V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x87R\x86a\x08\xE4V[`@Q\x94\x85\x93\x83\x85\x01\x97Q\x80\x91\x89^\x84\x01\x90\x83\x82\x01\x90_\x82RQ\x92\x83\x91^\x01\x01_\x81R\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a\x08\xE4V[Q\x90 `@Q\x90` \x82\x01\x92\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R0``\x1B`!\x84\x01R`5\x83\x01R`U\x82\x01R`U\x81Ra\x0B\xB1`u\x82a\x08\xE4V[Q\x90 \x16\x90V[_R`\x01` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x90V[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\x0C@WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD\xFE`\xA0\x80`@Ra\x04\xD7\x808\x03\x80\x91a\0\x17\x82\x85a\x02\x92V[\x839\x81\x01`@\x82\x82\x03\x12a\x01\xEBWa\0.\x82a\x02\xC9V[` \x83\x01Q\x90\x92`\x01`\x01`@\x1B\x03\x82\x11a\x01\xEBW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x01\xEBW\x81Qa\0[\x81a\x02\xDDV[\x92a\0i`@Q\x94\x85a\x02\x92V[\x81\x84R` \x84\x01\x92` \x83\x83\x01\x01\x11a\x01\xEBW\x81_\x92` \x80\x93\x01\x85^\x84\x01\x01R\x82;\x15a\x02tW\x7F\xA3\xF0\xADt\xE5B:\xEB\xFD\x80\xD3\xEFCFW\x835\xA9\xA7*\xEA\xEEY\xFFl\xB3X+5\x13=P\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`@Qc\\`\xDA\x1B`\xE0\x1B\x81R\x90\x91\x90` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x01\xF7W_\x91a\x02:W[P\x80;\x15a\x02\x1AWP\x81\x7F\x1C\xF3\xB0:l\xF1\x9F\xA2\xBA\xBAM\xF1H\xE9\xDC\xAB\xED\xEA\x7F\x8A\\\x07\x84\x0E ~\\\x08\x9B\xE9]>_\x80\xA2\x82Q\x15a\x02\x02W` `\x04\x92`@Q\x93\x84\x80\x92c\\`\xDA\x1B`\xE0\x1B\x82RZ\xFA\x91\x82\x15a\x01\xF7W_\x92a\x01\xAEW[P_\x80\x91a\x01\x8A\x94Q\x90\x84Z\xF4=\x15a\x01\xA6W=\x91a\x01n\x83a\x02\xDDV[\x92a\x01|`@Q\x94\x85a\x02\x92V[\x83R=_` \x85\x01>a\x02\xF8V[P[`\x80R`@Qa\x01\x80\x90\x81a\x03W\x829`\x80Q\x81`F\x01R\xF3[``\x91a\x02\xF8V[\x92\x91P` \x83=` \x11a\x01\xEFW[\x81a\x01\xCA` \x93\x83a\x02\x92V[\x81\x01\x03\x12a\x01\xEBW_\x80\x91a\x01\xE1a\x01\x8A\x95a\x02\xC9V[\x93\x94P\x91Pa\x01PV[_\x80\xFD[=\x91Pa\x01\xBDV[`@Q=_\x82>=\x90\xFD[PPP4\x15a\x01\x8CWc\xB3\x98\x97\x9F`\xE0\x1B_R`\x04_\xFD[cL\x9C\x8C\xE3`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[\x90P` \x81=` \x11a\x02lW[\x81a\x02U` \x93\x83a\x02\x92V[\x81\x01\x03\x12a\x01\xEBWa\x02f\x90a\x02\xC9V[_a\0\xF5V[=\x91Pa\x02HV[c\x193\xB4;`\xE2\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04R`$\x90\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x02\xB5W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xEBWV[`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB5W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x90a\x03\x1CWP\x80Q\x15a\x03\rW\x80Q\x90` \x01\xFD[c\xD6\xBD\xA2u`\xE0\x1B_R`\x04_\xFD[\x81Q\x15\x80a\x03MW[a\x03-WP\x90V[c\x99\x96\xB3\x15`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[P\x80;\x15a\x03%V\xFE`\x80`@R\x7F\\`\xDA\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80R` `\x80`\x04\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\x01\x07W_\x90\x15a\x01cWP` =` \x11a\x01\0W[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x82\x01\x16`\x80\x01\x90`\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\0\xD3Wa\0\xCE\x91`@R`\x80\x01a\x01\x12V[a\x01cV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[P=a\0\x81V[`@Q=_\x82>=\x90\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80` \x91\x01\x12a\x01_W`\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01_W\x90V[_\x80\xFD[_\x80\x916\x82\x807\x816\x91Z\xF4=_\x80>\x15a\x01|W=_\xF3[=_\xFD`\x80\x80`@R4`\xAAW_Q` a\x16\xBE_9_Q\x90_RT`\xFF\x81`@\x1C\x16`\x9BW`\x02`\x01`@\x1B\x03\x19`\x01`\x01`@\x1B\x03\x82\x16\x01`IW[`@Qa\x16\x0F\x90\x81a\0\xAF\x829\xF3[`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17_Q` a\x16\xBE_9_Q\x90_RU\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x90\xA1_\x80`:V[c\xF9.\xE8\xA9`\xE0\x1B_R`\x04_\xFD[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x07C\xBFn\x14a\x12\x03W\x80c\x18\xB5\xCE\x81\x14a\x11\xD0W\x80c)\x08\x03V\x14a\x10\x84W\x80cK\x8B\xE3\xF7\x14a\x0F&W\x80cT\xFDMP\x14a\x0E\x81W\x80cW\xD1\xBA%\x14a\x0EdW\x80cn\xDDl\t\x14a\x0E1W\x80cr@\xF9\xAF\x14a\x0C\xD1W\x80c\x85\xE1\xF4\xD0\x14a\x0C\xB4W\x80c\x8D\xA5\xCB[\x14a\x0C\x82W\x80c\xA3\xC6\xE1\xE7\x14a\x0CeW\x80c\xAAjC\xD8\x14a\x0C2W\x80c\xBFm\xB6\xF8\x14a\x0B\xFFW\x80c\xBFy\xFD\x1C\x14a\x02{W\x80c\xC7\xA7`\x95\x14a\x01\x83W\x80c\xD1\xF4s|\x14a\x01fW\x80c\xF2\xFD\xE3\x8B\x14a\x01\0Wc\xF8\xA1D\xBE\x14a\0\xDFW_\x80\xFD[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` `\x06T`@Q\x90\x81R\xF3[_\x80\xFD[4a\0\xFCW` `\x03\x196\x01\x12a\0\xFCWa\x01da\x01\x1Ca\x13\x97V[a\x01>s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x14a\x14\x10V[a\x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x15\x15a\x15$V[a\x15\x89V[\0[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` `\x08T`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW`@Q_`\nTa\x01\xA3\x81a\x12\xA8V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x029WP`\x01\x14a\x01\xDBW[a\x01\xD7\x83a\x01\xCB\x81\x85\x03\x82a\x12\xF9V[`@Q\x91\x82\x91\x82a\x13\x1CV[\x03\x90\xF3[`\n_\x90\x81R\x7F\xC6Z{\xB8\xD65\x1C\x1C\xF7\x0C\x95\xA3\x16\xCCj\x92\x83\x9C\x98f\x82\xD9\x8B\xC3_\x95\x8FH\x83\xF9\xD2\xA8\x93\x92P\x90[\x80\x82\x10a\x02\x1FWP\x90\x91P\x81\x01` \x01a\x01\xCBa\x01\xBBV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x02\x07V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x01\xCB\x90Pa\x01\xBBV[4a\0\xFCWa\x01\x80`\x03\x196\x01\x12a\0\xFCWa\x02\x95a\x13\x97V[`$5`D5\x91`d5\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x80\x94\x03a\0\xFCW`\x845s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\0\xFCW`\xE45s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\0\xFCWa\x01$5\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\0\xFCWa\x01D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFCWa\x03D\x906\x90`\x04\x01a\x13\xBAV[\x96a\x01d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFCWa\x03f\x906\x90`\x04\x01a\x13\xBAV[\x94\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x97`\xFF\x89`@\x1C\x16\x15\x98g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x90\x81a\x0B\xF7W[`\x01\x14\x90\x81a\x0B\xEDW[\x15\x90\x81a\x0B\xE4W[Pa\x0B\xBCW\x89`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x83\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x0BgW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x15a\x0B\tW\x80\x15a\n\xABW\x81\x15a\n'W\x82\x15a\t\xA3W\x83\x15a\t\x1FW\x84\x15a\x08\x9BW\x85\x15a\x08\x17W`\x05U`\x06U\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02T\x16\x17`\x02U\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x03T\x16\x17`\x03U`\xA45`\x07U`\xC45`\x08U\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04T\x16\x17`\x04Ua\x01\x045`\tUa\x04\xEC`\x0CTa\x12\xA8V[`\x1F\x81\x11a\x07\xD9W[P`\n\x7F1.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01`\x0CU\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01T\x16\x17`\x01U\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07+Wa\x05k\x81a\x05f`\nTa\x12\xA8V[a\x14\x8BV[` \x94`\x1F\x82\x11`\x01\x14a\x07XWa\x05\x9C\x92\x93\x94\x95\x82\x91_\x92a\x06\x99W[PP_\x19\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17\x90V[`\nU[\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x07+Wa\x05\xC6\x82a\x05\xC1`\x0BTa\x12\xA8V[a\x14\xC5V[` \x90`\x1F\x83\x11`\x01\x14a\x06\xA4W\x91\x80a\x05\xF8\x92a\x06\0\x95\x94_\x92a\x06\x99WPP_\x19\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17\x90V[`\x0BUa\x15\x89V[a\x06\x06W\0[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1\0[\x01Q\x90P\x86\x80a\x05\x89V[\x90`\x1F\x19\x83\x16\x91`\x0B_R\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x92_[\x81\x81\x10a\x07\x13WP\x91`\x01\x93\x91\x85a\x06\0\x97\x96\x94\x10a\x06\xFBW[PPP\x81\x1B\x01`\x0BUa\x15\x89V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x85\x80\x80a\x06\xEDV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x06\xD3V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x1F\x19\x82\x16\x95`\n_R\x7F\xC6Z{\xB8\xD65\x1C\x1C\xF7\x0C\x95\xA3\x16\xCCj\x92\x83\x9C\x98f\x82\xD9\x8B\xC3_\x95\x8FH\x83\xF9\xD2\xA8\x91_[\x88\x81\x10a\x07\xC1WP\x83`\x01\x95\x96\x97\x98\x10a\x07\xA9W[PPP\x81\x1B\x01`\nUa\x05\xA0V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x85\x80\x80a\x07\x9BV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x07\x86V[`\x0C_Ra\x08\x11\x90`\x1F\x01`\x05\x1C\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7\x90\x81\x01\x90a\x14uV[\x85a\x04\xF5V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FInitial appchain owner cannot be`D\x82\x01R\x7F zero address\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencing contract address cann`D\x82\x01R\x7Fot be zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FArbitrum inbox address cannot be`D\x82\x01R\x7F zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FArbitrum bridge address cannot b`D\x82\x01R\x7Fe zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FSequencing chain ID cannot be ze`D\x82\x01R\x7Fro\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FChain ID cannot be zero\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FOwner cannot be zero address\0\0\0\0`D\x82\x01R\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x01\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x8Aa\x04\x0CV[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90P\x15\x8Ca\x03\xB9V[0;\x15\x91Pa\x03\xB1V[\x8B\x91Pa\x03\xA7V[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x04T\x16`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03T\x16`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` `\tT`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` `\x05T`@Q\x90\x81R\xF3[4a\0\xFCWa\x0C\xDF6a\x13FV[a\r\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x14a\x14\x10V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07+Wa\r\x1B`\x0CTa\x12\xA8V[`\x1F\x81\x11a\r\xD8W[P_`\x1F\x82\x11`\x01\x14a\r`W\x81\x90a\rP\x93_\x92a\rUWPP_\x19\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17\x90V[`\x0CU\0[\x015\x90P\x83\x80a\x05\x89V[`\x1F\x19\x82\x16\x92\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7\x91_[\x85\x81\x10a\r\xC0WP\x83`\x01\x95\x10a\r\xA7W[PPP\x81\x1B\x01`\x0CU\0[_\x19`\xF8\x85`\x03\x1B\x16\x1C\x19\x91\x015\x16\x90U\x82\x80\x80a\r\x9CV[\x90\x92` `\x01\x81\x92\x86\x86\x015\x81U\x01\x94\x01\x91\x01a\r\x8AV[`\x0C_Ra\x0E!\x90\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x0E'W[`\x1F\x01`\x05\x1C\x01\x90a\x14uV[\x82a\r$V[\x90\x91P\x81\x90a\x0E\x14V[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` `\x07T`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW`@Q_`\x0CTa\x0E\xA1\x81a\x12\xA8V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x029WP`\x01\x14a\x0E\xC8Wa\x01\xD7\x83a\x01\xCB\x81\x85\x03\x82a\x12\xF9V[`\x0C_\x90\x81R\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7\x93\x92P\x90[\x80\x82\x10a\x0F\x0CWP\x90\x91P\x81\x01` \x01a\x01\xCBa\x01\xBBV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x0E\xF4V[4a\0\xFCWa\x0F46a\x13FV[a\x0FVs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x14a\x14\x10V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07+Wa\x0Ft\x81a\x05\xC1`\x0BTa\x12\xA8V[_\x91`\x1F\x82\x11`\x01\x14a\x0F\xE4Wa\x0F\xC1\x82\x80\x7F\x02X^\xBC\xE9\x18\xF6V`M\xAB\xB232\xA6\xDF\x1D\xCD\xE1\x19t\x1F|\x1F\x8F\xB3{\x19\x13\x92\xA2\xA7\x95_\x91a\x0F\xD9W[P_\x19\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17\x90V[`\x0BU[a\x0F\xD4`@Q\x92\x83\x92\x83a\x14\xFDV[\x03\x90\xA1\0[\x90P\x83\x015\x86a\x0F\xAFV[`\x1F\x19\x82\x16`\x0B_R\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x90_[\x81\x81\x10a\x10lWP\x93\x83\x7F\x02X^\xBC\xE9\x18\xF6V`M\xAB\xB232\xA6\xDF\x1D\xCD\xE1\x19t\x1F|\x1F\x8F\xB3{\x19\x13\x92\xA2\xA7\x95\x10a\x10SW[PP`\x01\x82\x81\x1B\x01`\x0BUa\x0F\xC5V[_\x19`\xF8\x85`\x03\x1B\x16\x1C\x19\x90\x83\x015\x16\x90U\x83\x80a\x10CV[\x83\x86\x015\x83U` \x95\x86\x01\x95`\x01\x90\x93\x01\x92\x01a\x10\x11V[4a\0\xFCWa\x10\x926a\x13FV[a\x10\xB4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x14a\x14\x10V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07+Wa\x10\xD2\x81a\x05f`\nTa\x12\xA8V[_\x91`\x1F\x82\x11`\x01\x14a\x110Wa\x11\x1E\x82\x80\x7Fg\xC5z\xE6\xAD\x92L\xD0\x93\xFB/\x06\xBC\x0B(\xFDXyH\x10Q\xA9\xC2\x03\xA4M'\xC8\x90MC|\x95_\x91a\x0F\xD9WP_\x19\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17\x90V[`\nUa\x0F\xD4`@Q\x92\x83\x92\x83a\x14\xFDV[`\x1F\x19\x82\x16`\n_R\x7F\xC6Z{\xB8\xD65\x1C\x1C\xF7\x0C\x95\xA3\x16\xCCj\x92\x83\x9C\x98f\x82\xD9\x8B\xC3_\x95\x8FH\x83\xF9\xD2\xA8\x90_[\x81\x81\x10a\x11\xB8WP\x93\x83\x7Fg\xC5z\xE6\xAD\x92L\xD0\x93\xFB/\x06\xBC\x0B(\xFDXyH\x10Q\xA9\xC2\x03\xA4M'\xC8\x90MC|\x95\x10a\x11\x9FW[PP`\x01\x82\x81\x1B\x01`\nUa\x0F\xC5V[_\x19`\xF8\x85`\x03\x1B\x16\x1C\x19\x90\x83\x015\x16\x90U\x83\x80a\x11\x8FV[\x83\x86\x015\x83U` \x95\x86\x01\x95`\x01\x90\x93\x01\x92\x01a\x11]V[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01T\x16`@Q\x90\x81R\xF3[4a\0\xFCW_`\x03\x196\x01\x12a\0\xFCW`@Q_`\x0BTa\x12#\x81a\x12\xA8V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x029WP`\x01\x14a\x12JWa\x01\xD7\x83a\x01\xCB\x81\x85\x03\x82a\x12\xF9V[`\x0B_\x90\x81R\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x93\x92P\x90[\x80\x82\x10a\x12\x8EWP\x90\x91P\x81\x01` \x01a\x01\xCBa\x01\xBBV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x12vV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x12\xEFW[` \x83\x10\x14a\x12\xC2WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x12\xB7V[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07+W`@RV[`\x1F\x19`\x1F` `@\x94\x81\x85R\x80Q\x91\x82\x91\x82\x82\x88\x01R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` `\x03\x19\x83\x01\x12a\0\xFCW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFCW\x82`#\x82\x01\x12\x15a\0\xFCW\x80`\x04\x015\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11a\0\xFCW`$\x84\x83\x01\x01\x11a\0\xFCW`$\x01\x91\x90V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0\xFCWV[\x81`\x1F\x82\x01\x12\x15a\0\xFCW\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x07+W`@Q\x92a\x13\xEF` `\x1F\x19`\x1F\x86\x01\x16\x01\x85a\x12\xF9V[\x82\x84R` \x83\x83\x01\x01\x11a\0\xFCW\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[\x15a\x14\x17WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x81\x81\x10a\x14\x80WPPV[_\x81U`\x01\x01a\x14uV[\x90`\x1F\x82\x11a\x14\x98WPPV[a\x14\xC3\x91`\n_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x0E'W`\x1F\x01`\x05\x1C\x01\x90a\x14uV[V[\x90`\x1F\x82\x11a\x14\xD2WPPV[a\x14\xC3\x91`\x0B_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x0E'W`\x1F\x01`\x05\x1C\x01\x90a\x14uV[\x90`\x1F\x83`@\x94`\x1F\x19\x93` \x86R\x81` \x87\x01R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x15a\x15+WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FNew owner cannot be zero address`D\x82\x01R\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x15\xAA\x81\x15\x15a\x15$V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0`\x804a\x014W`\x1Fa\x05\x058\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x018W\x80\x84\x92`@\x94\x85R\x839\x81\x01\x03\x12a\x014Wa\0F\x81a\x01LV[\x90`\x01`\x01`\xA0\x1B\x03\x90a\0\\\x90` \x01a\x01LV[\x16\x90\x81\x15a\x01!W_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x82U`@Q\x93\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3\x80;\x15a\x01\x01W`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2a\x03\xA4\x90\x81a\x01a\x829\xF3[c!\x1E\xB1Y`\xE2\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x014WV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c6Y\xCF\xE6\x14a\x02~W\x80c\\`\xDA\x1B\x14a\x02-W\x80cqP\x18\xA6\x14a\x01\x93W\x80c\x8D\xA5\xCB[\x14a\x01CWc\xF2\xFD\xE3\x8B\x14a\0PW_\x80\xFD[4a\x01?W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01?W`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x01?Wa\0\xA8a\x03XV[\x80\x15a\x01\x13Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x80\xFD[4a\x01?W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\x01?W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01?Wa\x01\xC9a\x03XV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01?W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01T\x16`@Q\x90\x81R\xF3[4a\x01?W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01?W`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x81\x03a\x01?Wa\x02\xD7a\x03XV[;\x15a\x03-W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01T\x16\x17`\x01U\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\0[\x7F\x84z\xC5d\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\x03xWV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `AppchainBlockExplorerUrlUpdated(string)` and selector `0x02585ebce918f656604dabb23332a6df1dcde119741f7c1f8fb37b191392a2a7`.
```solidity
event AppchainBlockExplorerUrlUpdated(string newUrl);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AppchainBlockExplorerUrlUpdated {
        #[allow(missing_docs)]
        pub newUrl: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for AppchainBlockExplorerUrlUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AppchainBlockExplorerUrlUpdated(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                2u8, 88u8, 94u8, 188u8, 233u8, 24u8, 246u8, 86u8, 96u8, 77u8, 171u8,
                178u8, 51u8, 50u8, 166u8, 223u8, 29u8, 205u8, 225u8, 25u8, 116u8, 31u8,
                124u8, 31u8, 143u8, 179u8, 123u8, 25u8, 19u8, 146u8, 162u8, 167u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { newUrl: data.0 }
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
                        &self.newUrl,
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
        impl alloy_sol_types::private::IntoLogData for AppchainBlockExplorerUrlUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AppchainBlockExplorerUrlUpdated>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &AppchainBlockExplorerUrlUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ArbChainConfigCreated(uint256,address)` and selector `0xeaf2b9d4fd6eba5a60870499f6335c6ab4826e029aff65ba0619329dbd421ec3`.
```solidity
event ArbChainConfigCreated(uint256 indexed chainId, address configAddress);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ArbChainConfigCreated {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub configAddress: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ArbChainConfigCreated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "ArbChainConfigCreated(uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                234u8, 242u8, 185u8, 212u8, 253u8, 110u8, 186u8, 90u8, 96u8, 135u8, 4u8,
                153u8, 246u8, 51u8, 92u8, 106u8, 180u8, 130u8, 110u8, 2u8, 154u8, 255u8,
                101u8, 186u8, 6u8, 25u8, 50u8, 157u8, 189u8, 66u8, 30u8, 195u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    chainId: topics.1,
                    configAddress: data.0,
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
                        &self.configAddress,
                    ),
                )
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
        impl alloy_sol_types::private::IntoLogData for ArbChainConfigCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ArbChainConfigCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ArbChainConfigCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DefaultSequencingChainWsRpcUrlUpdated(string)` and selector `0x67c57ae6ad924cd093fb2f06bc0b28fd5879481051a9c203a44d27c8904d437c`.
```solidity
event DefaultSequencingChainWsRpcUrlUpdated(string newRpcUrl);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DefaultSequencingChainWsRpcUrlUpdated {
        #[allow(missing_docs)]
        pub newRpcUrl: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for DefaultSequencingChainWsRpcUrlUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "DefaultSequencingChainWsRpcUrlUpdated(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                103u8, 197u8, 122u8, 230u8, 173u8, 146u8, 76u8, 208u8, 147u8, 251u8,
                47u8, 6u8, 188u8, 11u8, 40u8, 253u8, 88u8, 121u8, 72u8, 16u8, 81u8,
                169u8, 194u8, 3u8, 164u8, 77u8, 39u8, 200u8, 144u8, 77u8, 67u8, 124u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { newRpcUrl: data.0 }
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
                        &self.newRpcUrl,
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
        impl alloy_sol_types::private::IntoLogData
        for DefaultSequencingChainWsRpcUrlUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DefaultSequencingChainWsRpcUrlUpdated>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &DefaultSequencingChainWsRpcUrlUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ImplementationUpgraded(address)` and selector `0x51ea6ffdc9909d5ca341259f7221902e0676585d833e2bb21fa923c85e862886`.
```solidity
event ImplementationUpgraded(address newImplementation);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ImplementationUpgraded {
        #[allow(missing_docs)]
        pub newImplementation: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ImplementationUpgraded {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ImplementationUpgraded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                81u8, 234u8, 111u8, 253u8, 201u8, 144u8, 157u8, 92u8, 163u8, 65u8, 37u8,
                159u8, 114u8, 33u8, 144u8, 46u8, 6u8, 118u8, 88u8, 93u8, 131u8, 62u8,
                43u8, 178u8, 31u8, 169u8, 35u8, 200u8, 94u8, 134u8, 40u8, 134u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { newImplementation: data.0 }
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
                        &self.newImplementation,
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
        impl alloy_sol_types::private::IntoLogData for ImplementationUpgraded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ImplementationUpgraded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ImplementationUpgraded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
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
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
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
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `APPCHAIN_BLOCK_EXPLORER_URL()` and selector `0x0743bf6e`.
```solidity
function APPCHAIN_BLOCK_EXPLORER_URL() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct APPCHAIN_BLOCK_EXPLORER_URLCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`APPCHAIN_BLOCK_EXPLORER_URL()`](APPCHAIN_BLOCK_EXPLORER_URLCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct APPCHAIN_BLOCK_EXPLORER_URLReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<APPCHAIN_BLOCK_EXPLORER_URLCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: APPCHAIN_BLOCK_EXPLORER_URLCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for APPCHAIN_BLOCK_EXPLORER_URLCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
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
            impl ::core::convert::From<APPCHAIN_BLOCK_EXPLORER_URLReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: APPCHAIN_BLOCK_EXPLORER_URLReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for APPCHAIN_BLOCK_EXPLORER_URLReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for APPCHAIN_BLOCK_EXPLORER_URLCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "APPCHAIN_BLOCK_EXPLORER_URL()";
            const SELECTOR: [u8; 4] = [7u8, 67u8, 191u8, 110u8];
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
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
                        let r: APPCHAIN_BLOCK_EXPLORER_URLReturn = r.into();
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
                        let r: APPCHAIN_BLOCK_EXPLORER_URLReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ARBITRUM_BRIDGE_ADDRESS()` and selector `0x6edd6c09`.
```solidity
function ARBITRUM_BRIDGE_ADDRESS() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ARBITRUM_BRIDGE_ADDRESSCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ARBITRUM_BRIDGE_ADDRESS()`](ARBITRUM_BRIDGE_ADDRESSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ARBITRUM_BRIDGE_ADDRESSReturn {
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
            impl ::core::convert::From<ARBITRUM_BRIDGE_ADDRESSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ARBITRUM_BRIDGE_ADDRESSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ARBITRUM_BRIDGE_ADDRESSCall {
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
            impl ::core::convert::From<ARBITRUM_BRIDGE_ADDRESSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ARBITRUM_BRIDGE_ADDRESSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ARBITRUM_BRIDGE_ADDRESSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ARBITRUM_BRIDGE_ADDRESSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ARBITRUM_BRIDGE_ADDRESS()";
            const SELECTOR: [u8; 4] = [110u8, 221u8, 108u8, 9u8];
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
                        let r: ARBITRUM_BRIDGE_ADDRESSReturn = r.into();
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
                        let r: ARBITRUM_BRIDGE_ADDRESSReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ARBITRUM_INBOX_ADDRESS()` and selector `0xaa6a43d8`.
```solidity
function ARBITRUM_INBOX_ADDRESS() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ARBITRUM_INBOX_ADDRESSCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ARBITRUM_INBOX_ADDRESS()`](ARBITRUM_INBOX_ADDRESSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ARBITRUM_INBOX_ADDRESSReturn {
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
            impl ::core::convert::From<ARBITRUM_INBOX_ADDRESSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ARBITRUM_INBOX_ADDRESSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ARBITRUM_INBOX_ADDRESSCall {
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
            impl ::core::convert::From<ARBITRUM_INBOX_ADDRESSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ARBITRUM_INBOX_ADDRESSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ARBITRUM_INBOX_ADDRESSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ARBITRUM_INBOX_ADDRESSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ARBITRUM_INBOX_ADDRESS()";
            const SELECTOR: [u8; 4] = [170u8, 106u8, 67u8, 216u8];
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
                        let r: ARBITRUM_INBOX_ADDRESSReturn = r.into();
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
                        let r: ARBITRUM_INBOX_ADDRESSReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `CHAIN_ID()` and selector `0x85e1f4d0`.
```solidity
function CHAIN_ID() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CHAIN_IDCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`CHAIN_ID()`](CHAIN_IDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CHAIN_IDReturn {
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
            impl ::core::convert::From<CHAIN_IDCall> for UnderlyingRustTuple<'_> {
                fn from(value: CHAIN_IDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CHAIN_IDCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<CHAIN_IDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: CHAIN_IDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CHAIN_IDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CHAIN_IDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CHAIN_ID()";
            const SELECTOR: [u8; 4] = [133u8, 225u8, 244u8, 208u8];
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
                        let r: CHAIN_IDReturn = r.into();
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
                        let r: CHAIN_IDReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `DEFAULT_WS_RPC_URL()` and selector `0x6806ba06`.
```solidity
function DEFAULT_WS_RPC_URL() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_WS_RPC_URLCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`DEFAULT_WS_RPC_URL()`](DEFAULT_WS_RPC_URLCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_WS_RPC_URLReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DEFAULT_WS_RPC_URLCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_WS_RPC_URLCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_WS_RPC_URLCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
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
            impl ::core::convert::From<DEFAULT_WS_RPC_URLReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_WS_RPC_URLReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_WS_RPC_URLReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEFAULT_WS_RPC_URLCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEFAULT_WS_RPC_URL()";
            const SELECTOR: [u8; 4] = [104u8, 6u8, 186u8, 6u8];
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
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
                        let r: DEFAULT_WS_RPC_URLReturn = r.into();
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
                        let r: DEFAULT_WS_RPC_URLReturn = r.into();
                        r._0
                    })
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
    /**Function with signature `SEQUENCING_CHAIN_ID()` and selector `0xf8a144be`.
```solidity
function SEQUENCING_CHAIN_ID() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SEQUENCING_CHAIN_IDCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`SEQUENCING_CHAIN_ID()`](SEQUENCING_CHAIN_IDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SEQUENCING_CHAIN_IDReturn {
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
            impl ::core::convert::From<SEQUENCING_CHAIN_IDCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: SEQUENCING_CHAIN_IDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SEQUENCING_CHAIN_IDCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<SEQUENCING_CHAIN_IDReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: SEQUENCING_CHAIN_IDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SEQUENCING_CHAIN_IDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SEQUENCING_CHAIN_IDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SEQUENCING_CHAIN_ID()";
            const SELECTOR: [u8; 4] = [248u8, 161u8, 68u8, 190u8];
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
                        let r: SEQUENCING_CHAIN_IDReturn = r.into();
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
                        let r: SEQUENCING_CHAIN_IDReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `SEQUENCING_CONTRACT_ADDRESS()` and selector `0xbf6db6f8`.
```solidity
function SEQUENCING_CONTRACT_ADDRESS() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SEQUENCING_CONTRACT_ADDRESSCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`SEQUENCING_CONTRACT_ADDRESS()`](SEQUENCING_CONTRACT_ADDRESSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SEQUENCING_CONTRACT_ADDRESSReturn {
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
            impl ::core::convert::From<SEQUENCING_CONTRACT_ADDRESSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: SEQUENCING_CONTRACT_ADDRESSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SEQUENCING_CONTRACT_ADDRESSCall {
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
            impl ::core::convert::From<SEQUENCING_CONTRACT_ADDRESSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: SEQUENCING_CONTRACT_ADDRESSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SEQUENCING_CONTRACT_ADDRESSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SEQUENCING_CONTRACT_ADDRESSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SEQUENCING_CONTRACT_ADDRESS()";
            const SELECTOR: [u8; 4] = [191u8, 109u8, 182u8, 248u8];
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
                        let r: SEQUENCING_CONTRACT_ADDRESSReturn = r.into();
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
                        let r: SEQUENCING_CONTRACT_ADDRESSReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `SEQUENCING_START_BLOCK()` and selector `0xa3c6e1e7`.
```solidity
function SEQUENCING_START_BLOCK() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SEQUENCING_START_BLOCKCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`SEQUENCING_START_BLOCK()`](SEQUENCING_START_BLOCKCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SEQUENCING_START_BLOCKReturn {
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
            impl ::core::convert::From<SEQUENCING_START_BLOCKCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: SEQUENCING_START_BLOCKCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SEQUENCING_START_BLOCKCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<SEQUENCING_START_BLOCKReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: SEQUENCING_START_BLOCKReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SEQUENCING_START_BLOCKReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SEQUENCING_START_BLOCKCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SEQUENCING_START_BLOCK()";
            const SELECTOR: [u8; 4] = [163u8, 198u8, 225u8, 231u8];
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
                        let r: SEQUENCING_START_BLOCKReturn = r.into();
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
                        let r: SEQUENCING_START_BLOCKReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `SETTLEMENT_DELAY()` and selector `0x57d1ba25`.
```solidity
function SETTLEMENT_DELAY() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SETTLEMENT_DELAYCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`SETTLEMENT_DELAY()`](SETTLEMENT_DELAYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SETTLEMENT_DELAYReturn {
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
            impl ::core::convert::From<SETTLEMENT_DELAYCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: SETTLEMENT_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SETTLEMENT_DELAYCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<SETTLEMENT_DELAYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: SETTLEMENT_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SETTLEMENT_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SETTLEMENT_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SETTLEMENT_DELAY()";
            const SELECTOR: [u8; 4] = [87u8, 209u8, 186u8, 37u8];
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
                        let r: SETTLEMENT_DELAYReturn = r.into();
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
                        let r: SETTLEMENT_DELAYReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `SETTLEMENT_START_BLOCK()` and selector `0xd1f4737c`.
```solidity
function SETTLEMENT_START_BLOCK() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SETTLEMENT_START_BLOCKCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`SETTLEMENT_START_BLOCK()`](SETTLEMENT_START_BLOCKCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SETTLEMENT_START_BLOCKReturn {
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
            impl ::core::convert::From<SETTLEMENT_START_BLOCKCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: SETTLEMENT_START_BLOCKCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SETTLEMENT_START_BLOCKCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<SETTLEMENT_START_BLOCKReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: SETTLEMENT_START_BLOCKReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SETTLEMENT_START_BLOCKReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SETTLEMENT_START_BLOCKCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SETTLEMENT_START_BLOCK()";
            const SELECTOR: [u8; 4] = [209u8, 244u8, 115u8, 124u8];
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
                        let r: SETTLEMENT_START_BLOCKReturn = r.into();
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
                        let r: SETTLEMENT_START_BLOCKReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `appchainOwner()` and selector `0xd831975e`.
```solidity
function appchainOwner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct appchainOwnerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`appchainOwner()`](appchainOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct appchainOwnerReturn {
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
            impl ::core::convert::From<appchainOwnerCall> for UnderlyingRustTuple<'_> {
                fn from(value: appchainOwnerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for appchainOwnerCall {
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
            impl ::core::convert::From<appchainOwnerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: appchainOwnerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for appchainOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for appchainOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "appchainOwner()";
            const SELECTOR: [u8; 4] = [216u8, 49u8, 151u8, 94u8];
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
                        let r: appchainOwnerReturn = r.into();
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
                        let r: appchainOwnerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `chainConfig()` and selector `0xebb43738`.
```solidity
function chainConfig() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct chainConfigCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`chainConfig()`](chainConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct chainConfigReturn {
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
            impl ::core::convert::From<chainConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: chainConfigCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for chainConfigCall {
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
            impl ::core::convert::From<chainConfigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: chainConfigReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for chainConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for chainConfigCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "chainConfig()";
            const SELECTOR: [u8; 4] = [235u8, 180u8, 55u8, 56u8];
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
                        let r: chainConfigReturn = r.into();
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
                        let r: chainConfigReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `configManager()` and selector `0xca0ab075`.
```solidity
function configManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configManagerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`configManager()`](configManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configManagerReturn {
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
            impl ::core::convert::From<configManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: configManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configManagerCall {
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
            impl ::core::convert::From<configManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: configManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for configManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "configManager()";
            const SELECTOR: [u8; 4] = [202u8, 10u8, 176u8, 117u8];
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
                        let r: configManagerReturn = r.into();
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
                        let r: configManagerReturn = r.into();
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
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = alloy::sol_types::private::Address;
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
                        let r: ownerReturn = r.into();
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
                        let r: ownerReturn = r.into();
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
    /**Function with signature `testEmptyAppchainBlockExplorerUrl()` and selector `0x4dbdc85d`.
```solidity
function testEmptyAppchainBlockExplorerUrl() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testEmptyAppchainBlockExplorerUrlCall;
    ///Container type for the return parameters of the [`testEmptyAppchainBlockExplorerUrl()`](testEmptyAppchainBlockExplorerUrlCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testEmptyAppchainBlockExplorerUrlReturn {}
    #[allow(
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
            impl ::core::convert::From<testEmptyAppchainBlockExplorerUrlCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testEmptyAppchainBlockExplorerUrlCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testEmptyAppchainBlockExplorerUrlCall {
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
            impl ::core::convert::From<testEmptyAppchainBlockExplorerUrlReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testEmptyAppchainBlockExplorerUrlReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testEmptyAppchainBlockExplorerUrlReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testEmptyAppchainBlockExplorerUrlReturn {
            fn _tokenize(
                &self,
            ) -> <testEmptyAppchainBlockExplorerUrlCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testEmptyAppchainBlockExplorerUrlCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testEmptyAppchainBlockExplorerUrlReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testEmptyAppchainBlockExplorerUrl()";
            const SELECTOR: [u8; 4] = [77u8, 189u8, 200u8, 93u8];
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
                testEmptyAppchainBlockExplorerUrlReturn::_tokenize(ret)
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
    /**Function with signature `testEmptyDefaultRpcUrl()` and selector `0x26df4e25`.
```solidity
function testEmptyDefaultRpcUrl() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testEmptyDefaultRpcUrlCall;
    ///Container type for the return parameters of the [`testEmptyDefaultRpcUrl()`](testEmptyDefaultRpcUrlCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testEmptyDefaultRpcUrlReturn {}
    #[allow(
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
            impl ::core::convert::From<testEmptyDefaultRpcUrlCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testEmptyDefaultRpcUrlCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testEmptyDefaultRpcUrlCall {
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
            impl ::core::convert::From<testEmptyDefaultRpcUrlReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testEmptyDefaultRpcUrlReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testEmptyDefaultRpcUrlReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testEmptyDefaultRpcUrlReturn {
            fn _tokenize(
                &self,
            ) -> <testEmptyDefaultRpcUrlCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testEmptyDefaultRpcUrlCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testEmptyDefaultRpcUrlReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testEmptyDefaultRpcUrl()";
            const SELECTOR: [u8; 4] = [38u8, 223u8, 78u8, 37u8];
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
                testEmptyDefaultRpcUrlReturn::_tokenize(ret)
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
    /**Function with signature `testTransferOwnership()` and selector `0x8522ac8c`.
```solidity
function testTransferOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testTransferOwnershipCall;
    ///Container type for the return parameters of the [`testTransferOwnership()`](testTransferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testTransferOwnershipReturn {}
    #[allow(
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
            impl ::core::convert::From<testTransferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testTransferOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testTransferOwnershipCall {
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
            impl ::core::convert::From<testTransferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testTransferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testTransferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testTransferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <testTransferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testTransferOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testTransferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testTransferOwnership()";
            const SELECTOR: [u8; 4] = [133u8, 34u8, 172u8, 140u8];
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
                testTransferOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `testTransferOwnershipRevertOnNonOwner()` and selector `0xfebc4432`.
```solidity
function testTransferOwnershipRevertOnNonOwner() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testTransferOwnershipRevertOnNonOwnerCall;
    ///Container type for the return parameters of the [`testTransferOwnershipRevertOnNonOwner()`](testTransferOwnershipRevertOnNonOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testTransferOwnershipRevertOnNonOwnerReturn {}
    #[allow(
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
            impl ::core::convert::From<testTransferOwnershipRevertOnNonOwnerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testTransferOwnershipRevertOnNonOwnerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testTransferOwnershipRevertOnNonOwnerCall {
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
            impl ::core::convert::From<testTransferOwnershipRevertOnNonOwnerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testTransferOwnershipRevertOnNonOwnerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testTransferOwnershipRevertOnNonOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testTransferOwnershipRevertOnNonOwnerReturn {
            fn _tokenize(
                &self,
            ) -> <testTransferOwnershipRevertOnNonOwnerCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testTransferOwnershipRevertOnNonOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testTransferOwnershipRevertOnNonOwnerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testTransferOwnershipRevertOnNonOwner()";
            const SELECTOR: [u8; 4] = [254u8, 188u8, 68u8, 50u8];
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
                testTransferOwnershipRevertOnNonOwnerReturn::_tokenize(ret)
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
    /**Function with signature `testTransferOwnershipRevertOnZeroAddress()` and selector `0x34784bf1`.
```solidity
function testTransferOwnershipRevertOnZeroAddress() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testTransferOwnershipRevertOnZeroAddressCall;
    ///Container type for the return parameters of the [`testTransferOwnershipRevertOnZeroAddress()`](testTransferOwnershipRevertOnZeroAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testTransferOwnershipRevertOnZeroAddressReturn {}
    #[allow(
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
            impl ::core::convert::From<testTransferOwnershipRevertOnZeroAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testTransferOwnershipRevertOnZeroAddressCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testTransferOwnershipRevertOnZeroAddressCall {
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
            impl ::core::convert::From<testTransferOwnershipRevertOnZeroAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testTransferOwnershipRevertOnZeroAddressReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testTransferOwnershipRevertOnZeroAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testTransferOwnershipRevertOnZeroAddressReturn {
            fn _tokenize(
                &self,
            ) -> <testTransferOwnershipRevertOnZeroAddressCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testTransferOwnershipRevertOnZeroAddressCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testTransferOwnershipRevertOnZeroAddressReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testTransferOwnershipRevertOnZeroAddress()";
            const SELECTOR: [u8; 4] = [52u8, 120u8, 75u8, 241u8];
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
                testTransferOwnershipRevertOnZeroAddressReturn::_tokenize(ret)
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
    /**Function with signature `testUpdateAppchainBlockExplorerUrl()` and selector `0x5b7c1599`.
```solidity
function testUpdateAppchainBlockExplorerUrl() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateAppchainBlockExplorerUrlCall;
    ///Container type for the return parameters of the [`testUpdateAppchainBlockExplorerUrl()`](testUpdateAppchainBlockExplorerUrlCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateAppchainBlockExplorerUrlReturn {}
    #[allow(
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
            impl ::core::convert::From<testUpdateAppchainBlockExplorerUrlCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateAppchainBlockExplorerUrlCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateAppchainBlockExplorerUrlCall {
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
            impl ::core::convert::From<testUpdateAppchainBlockExplorerUrlReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateAppchainBlockExplorerUrlReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateAppchainBlockExplorerUrlReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testUpdateAppchainBlockExplorerUrlReturn {
            fn _tokenize(
                &self,
            ) -> <testUpdateAppchainBlockExplorerUrlCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testUpdateAppchainBlockExplorerUrlCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testUpdateAppchainBlockExplorerUrlReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testUpdateAppchainBlockExplorerUrl()";
            const SELECTOR: [u8; 4] = [91u8, 124u8, 21u8, 153u8];
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
                testUpdateAppchainBlockExplorerUrlReturn::_tokenize(ret)
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
    /**Function with signature `testUpdateAppchainBlockExplorerUrlRevertOnNonOwner()` and selector `0x52069fc2`.
```solidity
function testUpdateAppchainBlockExplorerUrlRevertOnNonOwner() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall;
    ///Container type for the return parameters of the [`testUpdateAppchainBlockExplorerUrlRevertOnNonOwner()`](testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerReturn {}
    #[allow(
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
            impl ::core::convert::From<
                testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall {
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
            impl ::core::convert::From<
                testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerReturn {
            fn _tokenize(
                &self,
            ) -> <testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testUpdateAppchainBlockExplorerUrlRevertOnNonOwner()";
            const SELECTOR: [u8; 4] = [82u8, 6u8, 159u8, 194u8];
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
                testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerReturn::_tokenize(ret)
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
    /**Function with signature `testUpdateDefaultSequencingChainRpcUrl()` and selector `0xfdfc99df`.
```solidity
function testUpdateDefaultSequencingChainRpcUrl() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateDefaultSequencingChainRpcUrlCall;
    ///Container type for the return parameters of the [`testUpdateDefaultSequencingChainRpcUrl()`](testUpdateDefaultSequencingChainRpcUrlCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateDefaultSequencingChainRpcUrlReturn {}
    #[allow(
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
            impl ::core::convert::From<testUpdateDefaultSequencingChainRpcUrlCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateDefaultSequencingChainRpcUrlCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateDefaultSequencingChainRpcUrlCall {
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
            impl ::core::convert::From<testUpdateDefaultSequencingChainRpcUrlReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testUpdateDefaultSequencingChainRpcUrlReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateDefaultSequencingChainRpcUrlReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testUpdateDefaultSequencingChainRpcUrlReturn {
            fn _tokenize(
                &self,
            ) -> <testUpdateDefaultSequencingChainRpcUrlCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testUpdateDefaultSequencingChainRpcUrlCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testUpdateDefaultSequencingChainRpcUrlReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testUpdateDefaultSequencingChainRpcUrl()";
            const SELECTOR: [u8; 4] = [253u8, 252u8, 153u8, 223u8];
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
                testUpdateDefaultSequencingChainRpcUrlReturn::_tokenize(ret)
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
    /**Function with signature `testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner()` and selector `0x60944ce6`.
```solidity
function testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall;
    ///Container type for the return parameters of the [`testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner()`](testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerReturn {}
    #[allow(
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
            impl ::core::convert::From<
                testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall {
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
            impl ::core::convert::From<
                testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerReturn {
            fn _tokenize(
                &self,
            ) -> <testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner()";
            const SELECTOR: [u8; 4] = [96u8, 148u8, 76u8, 230u8];
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
                testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerReturn::_tokenize(
                    ret,
                )
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
    ///Container for all the [`ArbChainConfigUpdateTests`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum ArbChainConfigUpdateTestsCalls {
        #[allow(missing_docs)]
        APPCHAIN_BLOCK_EXPLORER_URL(APPCHAIN_BLOCK_EXPLORER_URLCall),
        #[allow(missing_docs)]
        ARBITRUM_BRIDGE_ADDRESS(ARBITRUM_BRIDGE_ADDRESSCall),
        #[allow(missing_docs)]
        ARBITRUM_INBOX_ADDRESS(ARBITRUM_INBOX_ADDRESSCall),
        #[allow(missing_docs)]
        CHAIN_ID(CHAIN_IDCall),
        #[allow(missing_docs)]
        DEFAULT_WS_RPC_URL(DEFAULT_WS_RPC_URLCall),
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        SEQUENCING_CHAIN_ID(SEQUENCING_CHAIN_IDCall),
        #[allow(missing_docs)]
        SEQUENCING_CONTRACT_ADDRESS(SEQUENCING_CONTRACT_ADDRESSCall),
        #[allow(missing_docs)]
        SEQUENCING_START_BLOCK(SEQUENCING_START_BLOCKCall),
        #[allow(missing_docs)]
        SETTLEMENT_DELAY(SETTLEMENT_DELAYCall),
        #[allow(missing_docs)]
        SETTLEMENT_START_BLOCK(SETTLEMENT_START_BLOCKCall),
        #[allow(missing_docs)]
        appchainOwner(appchainOwnerCall),
        #[allow(missing_docs)]
        chainConfig(chainConfigCall),
        #[allow(missing_docs)]
        configManager(configManagerCall),
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
        owner(ownerCall),
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
        testEmptyAppchainBlockExplorerUrl(testEmptyAppchainBlockExplorerUrlCall),
        #[allow(missing_docs)]
        testEmptyDefaultRpcUrl(testEmptyDefaultRpcUrlCall),
        #[allow(missing_docs)]
        testTransferOwnership(testTransferOwnershipCall),
        #[allow(missing_docs)]
        testTransferOwnershipRevertOnNonOwner(testTransferOwnershipRevertOnNonOwnerCall),
        #[allow(missing_docs)]
        testTransferOwnershipRevertOnZeroAddress(
            testTransferOwnershipRevertOnZeroAddressCall,
        ),
        #[allow(missing_docs)]
        testUpdateAppchainBlockExplorerUrl(testUpdateAppchainBlockExplorerUrlCall),
        #[allow(missing_docs)]
        testUpdateAppchainBlockExplorerUrlRevertOnNonOwner(
            testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall,
        ),
        #[allow(missing_docs)]
        testUpdateDefaultSequencingChainRpcUrl(
            testUpdateDefaultSequencingChainRpcUrlCall,
        ),
        #[allow(missing_docs)]
        testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner(
            testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall,
        ),
    }
    #[automatically_derived]
    impl ArbChainConfigUpdateTestsCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [7u8, 67u8, 191u8, 110u8],
            [10u8, 146u8, 84u8, 228u8],
            [30u8, 215u8, 131u8, 28u8],
            [38u8, 223u8, 78u8, 37u8],
            [42u8, 222u8, 56u8, 128u8],
            [52u8, 120u8, 75u8, 241u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [77u8, 189u8, 200u8, 93u8],
            [82u8, 6u8, 159u8, 194u8],
            [87u8, 209u8, 186u8, 37u8],
            [91u8, 124u8, 21u8, 153u8],
            [96u8, 148u8, 76u8, 230u8],
            [102u8, 217u8, 169u8, 160u8],
            [104u8, 6u8, 186u8, 6u8],
            [110u8, 221u8, 108u8, 9u8],
            [133u8, 34u8, 108u8, 129u8],
            [133u8, 34u8, 172u8, 140u8],
            [133u8, 225u8, 244u8, 208u8],
            [141u8, 165u8, 203u8, 91u8],
            [145u8, 106u8, 23u8, 198u8],
            [163u8, 198u8, 225u8, 231u8],
            [170u8, 106u8, 67u8, 216u8],
            [176u8, 70u8, 79u8, 220u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [191u8, 109u8, 182u8, 248u8],
            [202u8, 10u8, 176u8, 117u8],
            [209u8, 244u8, 115u8, 124u8],
            [216u8, 49u8, 151u8, 94u8],
            [226u8, 12u8, 159u8, 113u8],
            [235u8, 180u8, 55u8, 56u8],
            [248u8, 161u8, 68u8, 190u8],
            [250u8, 118u8, 38u8, 212u8],
            [253u8, 252u8, 153u8, 223u8],
            [254u8, 188u8, 68u8, 50u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ArbChainConfigUpdateTestsCalls {
        const NAME: &'static str = "ArbChainConfigUpdateTestsCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 36usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::APPCHAIN_BLOCK_EXPLORER_URL(_) => {
                    <APPCHAIN_BLOCK_EXPLORER_URLCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ARBITRUM_BRIDGE_ADDRESS(_) => {
                    <ARBITRUM_BRIDGE_ADDRESSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ARBITRUM_INBOX_ADDRESS(_) => {
                    <ARBITRUM_INBOX_ADDRESSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::CHAIN_ID(_) => <CHAIN_IDCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::DEFAULT_WS_RPC_URL(_) => {
                    <DEFAULT_WS_RPC_URLCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::SEQUENCING_CHAIN_ID(_) => {
                    <SEQUENCING_CHAIN_IDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::SEQUENCING_CONTRACT_ADDRESS(_) => {
                    <SEQUENCING_CONTRACT_ADDRESSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::SEQUENCING_START_BLOCK(_) => {
                    <SEQUENCING_START_BLOCKCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::SETTLEMENT_DELAY(_) => {
                    <SETTLEMENT_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::SETTLEMENT_START_BLOCK(_) => {
                    <SETTLEMENT_START_BLOCKCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::appchainOwner(_) => {
                    <appchainOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::chainConfig(_) => {
                    <chainConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::configManager(_) => {
                    <configManagerCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::testEmptyAppchainBlockExplorerUrl(_) => {
                    <testEmptyAppchainBlockExplorerUrlCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testEmptyDefaultRpcUrl(_) => {
                    <testEmptyDefaultRpcUrlCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testTransferOwnership(_) => {
                    <testTransferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testTransferOwnershipRevertOnNonOwner(_) => {
                    <testTransferOwnershipRevertOnNonOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testTransferOwnershipRevertOnZeroAddress(_) => {
                    <testTransferOwnershipRevertOnZeroAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testUpdateAppchainBlockExplorerUrl(_) => {
                    <testUpdateAppchainBlockExplorerUrlCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testUpdateAppchainBlockExplorerUrlRevertOnNonOwner(_) => {
                    <testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testUpdateDefaultSequencingChainRpcUrl(_) => {
                    <testUpdateDefaultSequencingChainRpcUrlCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner(_) => {
                    <testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls>] = &[
                {
                    fn APPCHAIN_BLOCK_EXPLORER_URL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <APPCHAIN_BLOCK_EXPLORER_URLCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::APPCHAIN_BLOCK_EXPLORER_URL,
                            )
                    }
                    APPCHAIN_BLOCK_EXPLORER_URL
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ArbChainConfigUpdateTestsCalls::setUp)
                    }
                    setUp
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn testEmptyDefaultRpcUrl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testEmptyDefaultRpcUrlCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::testEmptyDefaultRpcUrl)
                    }
                    testEmptyDefaultRpcUrl
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn testTransferOwnershipRevertOnZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testTransferOwnershipRevertOnZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testTransferOwnershipRevertOnZeroAddress,
                            )
                    }
                    testTransferOwnershipRevertOnZeroAddress
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn testEmptyAppchainBlockExplorerUrl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testEmptyAppchainBlockExplorerUrlCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testEmptyAppchainBlockExplorerUrl,
                            )
                    }
                    testEmptyAppchainBlockExplorerUrl
                },
                {
                    fn testUpdateAppchainBlockExplorerUrlRevertOnNonOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testUpdateAppchainBlockExplorerUrlRevertOnNonOwner,
                            )
                    }
                    testUpdateAppchainBlockExplorerUrlRevertOnNonOwner
                },
                {
                    fn SETTLEMENT_DELAY(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <SETTLEMENT_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::SETTLEMENT_DELAY)
                    }
                    SETTLEMENT_DELAY
                },
                {
                    fn testUpdateAppchainBlockExplorerUrl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testUpdateAppchainBlockExplorerUrlCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testUpdateAppchainBlockExplorerUrl,
                            )
                    }
                    testUpdateAppchainBlockExplorerUrl
                },
                {
                    fn testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner,
                            )
                    }
                    testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn DEFAULT_WS_RPC_URL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <DEFAULT_WS_RPC_URLCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::DEFAULT_WS_RPC_URL)
                    }
                    DEFAULT_WS_RPC_URL
                },
                {
                    fn ARBITRUM_BRIDGE_ADDRESS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <ARBITRUM_BRIDGE_ADDRESSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::ARBITRUM_BRIDGE_ADDRESS)
                    }
                    ARBITRUM_BRIDGE_ADDRESS
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn testTransferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testTransferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::testTransferOwnership)
                    }
                    testTransferOwnership
                },
                {
                    fn CHAIN_ID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <CHAIN_IDCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ArbChainConfigUpdateTestsCalls::CHAIN_ID)
                    }
                    CHAIN_ID
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ArbChainConfigUpdateTestsCalls::owner)
                    }
                    owner
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn SEQUENCING_START_BLOCK(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <SEQUENCING_START_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::SEQUENCING_START_BLOCK)
                    }
                    SEQUENCING_START_BLOCK
                },
                {
                    fn ARBITRUM_INBOX_ADDRESS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <ARBITRUM_INBOX_ADDRESSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::ARBITRUM_INBOX_ADDRESS)
                    }
                    ARBITRUM_INBOX_ADDRESS
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ArbChainConfigUpdateTestsCalls::failed)
                    }
                    failed
                },
                {
                    fn SEQUENCING_CONTRACT_ADDRESS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <SEQUENCING_CONTRACT_ADDRESSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::SEQUENCING_CONTRACT_ADDRESS,
                            )
                    }
                    SEQUENCING_CONTRACT_ADDRESS
                },
                {
                    fn configManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <configManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::configManager)
                    }
                    configManager
                },
                {
                    fn SETTLEMENT_START_BLOCK(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <SETTLEMENT_START_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::SETTLEMENT_START_BLOCK)
                    }
                    SETTLEMENT_START_BLOCK
                },
                {
                    fn appchainOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <appchainOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::appchainOwner)
                    }
                    appchainOwner
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn chainConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <chainConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::chainConfig)
                    }
                    chainConfig
                },
                {
                    fn SEQUENCING_CHAIN_ID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <SEQUENCING_CHAIN_IDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::SEQUENCING_CHAIN_ID)
                    }
                    SEQUENCING_CHAIN_ID
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ArbChainConfigUpdateTestsCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn testUpdateDefaultSequencingChainRpcUrl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testUpdateDefaultSequencingChainRpcUrlCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testUpdateDefaultSequencingChainRpcUrl,
                            )
                    }
                    testUpdateDefaultSequencingChainRpcUrl
                },
                {
                    fn testTransferOwnershipRevertOnNonOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testTransferOwnershipRevertOnNonOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testTransferOwnershipRevertOnNonOwner,
                            )
                    }
                    testTransferOwnershipRevertOnNonOwner
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
            ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls>] = &[
                {
                    fn APPCHAIN_BLOCK_EXPLORER_URL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <APPCHAIN_BLOCK_EXPLORER_URLCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::APPCHAIN_BLOCK_EXPLORER_URL,
                            )
                    }
                    APPCHAIN_BLOCK_EXPLORER_URL
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::setUp)
                    }
                    setUp
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn testEmptyDefaultRpcUrl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testEmptyDefaultRpcUrlCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::testEmptyDefaultRpcUrl)
                    }
                    testEmptyDefaultRpcUrl
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn testTransferOwnershipRevertOnZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testTransferOwnershipRevertOnZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testTransferOwnershipRevertOnZeroAddress,
                            )
                    }
                    testTransferOwnershipRevertOnZeroAddress
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn testEmptyAppchainBlockExplorerUrl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testEmptyAppchainBlockExplorerUrlCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testEmptyAppchainBlockExplorerUrl,
                            )
                    }
                    testEmptyAppchainBlockExplorerUrl
                },
                {
                    fn testUpdateAppchainBlockExplorerUrlRevertOnNonOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testUpdateAppchainBlockExplorerUrlRevertOnNonOwner,
                            )
                    }
                    testUpdateAppchainBlockExplorerUrlRevertOnNonOwner
                },
                {
                    fn SETTLEMENT_DELAY(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <SETTLEMENT_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::SETTLEMENT_DELAY)
                    }
                    SETTLEMENT_DELAY
                },
                {
                    fn testUpdateAppchainBlockExplorerUrl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testUpdateAppchainBlockExplorerUrlCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testUpdateAppchainBlockExplorerUrl,
                            )
                    }
                    testUpdateAppchainBlockExplorerUrl
                },
                {
                    fn testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner,
                            )
                    }
                    testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn DEFAULT_WS_RPC_URL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <DEFAULT_WS_RPC_URLCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::DEFAULT_WS_RPC_URL)
                    }
                    DEFAULT_WS_RPC_URL
                },
                {
                    fn ARBITRUM_BRIDGE_ADDRESS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <ARBITRUM_BRIDGE_ADDRESSCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::ARBITRUM_BRIDGE_ADDRESS)
                    }
                    ARBITRUM_BRIDGE_ADDRESS
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn testTransferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testTransferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::testTransferOwnership)
                    }
                    testTransferOwnership
                },
                {
                    fn CHAIN_ID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <CHAIN_IDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::CHAIN_ID)
                    }
                    CHAIN_ID
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::owner)
                    }
                    owner
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn SEQUENCING_START_BLOCK(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <SEQUENCING_START_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::SEQUENCING_START_BLOCK)
                    }
                    SEQUENCING_START_BLOCK
                },
                {
                    fn ARBITRUM_INBOX_ADDRESS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <ARBITRUM_INBOX_ADDRESSCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::ARBITRUM_INBOX_ADDRESS)
                    }
                    ARBITRUM_INBOX_ADDRESS
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::failed)
                    }
                    failed
                },
                {
                    fn SEQUENCING_CONTRACT_ADDRESS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <SEQUENCING_CONTRACT_ADDRESSCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::SEQUENCING_CONTRACT_ADDRESS,
                            )
                    }
                    SEQUENCING_CONTRACT_ADDRESS
                },
                {
                    fn configManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <configManagerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::configManager)
                    }
                    configManager
                },
                {
                    fn SETTLEMENT_START_BLOCK(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <SETTLEMENT_START_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::SETTLEMENT_START_BLOCK)
                    }
                    SETTLEMENT_START_BLOCK
                },
                {
                    fn appchainOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <appchainOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::appchainOwner)
                    }
                    appchainOwner
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn chainConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <chainConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::chainConfig)
                    }
                    chainConfig
                },
                {
                    fn SEQUENCING_CHAIN_ID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <SEQUENCING_CHAIN_IDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::SEQUENCING_CHAIN_ID)
                    }
                    SEQUENCING_CHAIN_ID
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ArbChainConfigUpdateTestsCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn testUpdateDefaultSequencingChainRpcUrl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testUpdateDefaultSequencingChainRpcUrlCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testUpdateDefaultSequencingChainRpcUrl,
                            )
                    }
                    testUpdateDefaultSequencingChainRpcUrl
                },
                {
                    fn testTransferOwnershipRevertOnNonOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ArbChainConfigUpdateTestsCalls> {
                        <testTransferOwnershipRevertOnNonOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ArbChainConfigUpdateTestsCalls::testTransferOwnershipRevertOnNonOwner,
                            )
                    }
                    testTransferOwnershipRevertOnNonOwner
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
                Self::APPCHAIN_BLOCK_EXPLORER_URL(inner) => {
                    <APPCHAIN_BLOCK_EXPLORER_URLCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ARBITRUM_BRIDGE_ADDRESS(inner) => {
                    <ARBITRUM_BRIDGE_ADDRESSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ARBITRUM_INBOX_ADDRESS(inner) => {
                    <ARBITRUM_INBOX_ADDRESSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CHAIN_ID(inner) => {
                    <CHAIN_IDCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::DEFAULT_WS_RPC_URL(inner) => {
                    <DEFAULT_WS_RPC_URLCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::SEQUENCING_CHAIN_ID(inner) => {
                    <SEQUENCING_CHAIN_IDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SEQUENCING_CONTRACT_ADDRESS(inner) => {
                    <SEQUENCING_CONTRACT_ADDRESSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SEQUENCING_START_BLOCK(inner) => {
                    <SEQUENCING_START_BLOCKCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SETTLEMENT_DELAY(inner) => {
                    <SETTLEMENT_DELAYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SETTLEMENT_START_BLOCK(inner) => {
                    <SETTLEMENT_START_BLOCKCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::appchainOwner(inner) => {
                    <appchainOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::chainConfig(inner) => {
                    <chainConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::configManager(inner) => {
                    <configManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::testEmptyAppchainBlockExplorerUrl(inner) => {
                    <testEmptyAppchainBlockExplorerUrlCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testEmptyDefaultRpcUrl(inner) => {
                    <testEmptyDefaultRpcUrlCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testTransferOwnership(inner) => {
                    <testTransferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testTransferOwnershipRevertOnNonOwner(inner) => {
                    <testTransferOwnershipRevertOnNonOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testTransferOwnershipRevertOnZeroAddress(inner) => {
                    <testTransferOwnershipRevertOnZeroAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testUpdateAppchainBlockExplorerUrl(inner) => {
                    <testUpdateAppchainBlockExplorerUrlCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testUpdateAppchainBlockExplorerUrlRevertOnNonOwner(inner) => {
                    <testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testUpdateDefaultSequencingChainRpcUrl(inner) => {
                    <testUpdateDefaultSequencingChainRpcUrlCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner(inner) => {
                    <testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::APPCHAIN_BLOCK_EXPLORER_URL(inner) => {
                    <APPCHAIN_BLOCK_EXPLORER_URLCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ARBITRUM_BRIDGE_ADDRESS(inner) => {
                    <ARBITRUM_BRIDGE_ADDRESSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ARBITRUM_INBOX_ADDRESS(inner) => {
                    <ARBITRUM_INBOX_ADDRESSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CHAIN_ID(inner) => {
                    <CHAIN_IDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DEFAULT_WS_RPC_URL(inner) => {
                    <DEFAULT_WS_RPC_URLCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::SEQUENCING_CHAIN_ID(inner) => {
                    <SEQUENCING_CHAIN_IDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SEQUENCING_CONTRACT_ADDRESS(inner) => {
                    <SEQUENCING_CONTRACT_ADDRESSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SEQUENCING_START_BLOCK(inner) => {
                    <SEQUENCING_START_BLOCKCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SETTLEMENT_DELAY(inner) => {
                    <SETTLEMENT_DELAYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SETTLEMENT_START_BLOCK(inner) => {
                    <SETTLEMENT_START_BLOCKCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::appchainOwner(inner) => {
                    <appchainOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::chainConfig(inner) => {
                    <chainConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::configManager(inner) => {
                    <configManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::testEmptyAppchainBlockExplorerUrl(inner) => {
                    <testEmptyAppchainBlockExplorerUrlCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testEmptyDefaultRpcUrl(inner) => {
                    <testEmptyDefaultRpcUrlCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testTransferOwnership(inner) => {
                    <testTransferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testTransferOwnershipRevertOnNonOwner(inner) => {
                    <testTransferOwnershipRevertOnNonOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testTransferOwnershipRevertOnZeroAddress(inner) => {
                    <testTransferOwnershipRevertOnZeroAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testUpdateAppchainBlockExplorerUrl(inner) => {
                    <testUpdateAppchainBlockExplorerUrlCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testUpdateAppchainBlockExplorerUrlRevertOnNonOwner(inner) => {
                    <testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testUpdateDefaultSequencingChainRpcUrl(inner) => {
                    <testUpdateDefaultSequencingChainRpcUrlCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner(inner) => {
                    <testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ArbChainConfigUpdateTests`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum ArbChainConfigUpdateTestsEvents {
        #[allow(missing_docs)]
        AppchainBlockExplorerUrlUpdated(AppchainBlockExplorerUrlUpdated),
        #[allow(missing_docs)]
        ArbChainConfigCreated(ArbChainConfigCreated),
        #[allow(missing_docs)]
        DefaultSequencingChainWsRpcUrlUpdated(DefaultSequencingChainWsRpcUrlUpdated),
        #[allow(missing_docs)]
        ImplementationUpgraded(ImplementationUpgraded),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
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
    impl ArbChainConfigUpdateTestsEvents {
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
                2u8, 88u8, 94u8, 188u8, 233u8, 24u8, 246u8, 86u8, 96u8, 77u8, 171u8,
                178u8, 51u8, 50u8, 166u8, 223u8, 29u8, 205u8, 225u8, 25u8, 116u8, 31u8,
                124u8, 31u8, 143u8, 179u8, 123u8, 25u8, 19u8, 146u8, 162u8, 167u8,
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
                81u8, 234u8, 111u8, 253u8, 201u8, 144u8, 157u8, 92u8, 163u8, 65u8, 37u8,
                159u8, 114u8, 33u8, 144u8, 46u8, 6u8, 118u8, 88u8, 93u8, 131u8, 62u8,
                43u8, 178u8, 31u8, 169u8, 35u8, 200u8, 94u8, 134u8, 40u8, 134u8,
            ],
            [
                93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8,
                89u8, 239u8, 36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8,
                10u8, 232u8, 67u8, 78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
            ],
            [
                103u8, 197u8, 122u8, 230u8, 173u8, 146u8, 76u8, 208u8, 147u8, 251u8,
                47u8, 6u8, 188u8, 11u8, 40u8, 253u8, 88u8, 121u8, 72u8, 16u8, 81u8,
                169u8, 194u8, 3u8, 164u8, 77u8, 39u8, 200u8, 144u8, 77u8, 67u8, 124u8,
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
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8,
                16u8, 143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8,
                67u8, 122u8, 97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
            ],
            [
                167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8,
                70u8, 17u8, 56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8,
                247u8, 225u8, 123u8, 4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
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
                234u8, 242u8, 185u8, 212u8, 253u8, 110u8, 186u8, 90u8, 96u8, 135u8, 4u8,
                153u8, 246u8, 51u8, 92u8, 106u8, 180u8, 130u8, 110u8, 2u8, 154u8, 255u8,
                101u8, 186u8, 6u8, 25u8, 50u8, 157u8, 189u8, 66u8, 30u8, 195u8,
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
    impl alloy_sol_types::SolEventInterface for ArbChainConfigUpdateTestsEvents {
        const NAME: &'static str = "ArbChainConfigUpdateTestsEvents";
        const COUNT: usize = 27usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <AppchainBlockExplorerUrlUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <AppchainBlockExplorerUrlUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::AppchainBlockExplorerUrlUpdated)
                }
                Some(
                    <ArbChainConfigCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ArbChainConfigCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ArbChainConfigCreated)
                }
                Some(
                    <DefaultSequencingChainWsRpcUrlUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DefaultSequencingChainWsRpcUrlUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DefaultSequencingChainWsRpcUrlUpdated)
                }
                Some(
                    <ImplementationUpgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ImplementationUpgraded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ImplementationUpgraded)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferred)
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
    impl alloy_sol_types::private::IntoLogData for ArbChainConfigUpdateTestsEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AppchainBlockExplorerUrlUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ArbChainConfigCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DefaultSequencingChainWsRpcUrlUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ImplementationUpgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
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
                Self::AppchainBlockExplorerUrlUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ArbChainConfigCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DefaultSequencingChainWsRpcUrlUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ImplementationUpgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
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
    /**Creates a new wrapper around an on-chain [`ArbChainConfigUpdateTests`](self) contract instance.

See the [wrapper's documentation](`ArbChainConfigUpdateTestsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ArbChainConfigUpdateTestsInstance<P, N> {
        ArbChainConfigUpdateTestsInstance::<P, N>::new(address, provider)
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
        Output = alloy_contract::Result<ArbChainConfigUpdateTestsInstance<P, N>>,
    > {
        ArbChainConfigUpdateTestsInstance::<P, N>::deploy(provider)
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
        ArbChainConfigUpdateTestsInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`ArbChainConfigUpdateTests`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ArbChainConfigUpdateTests`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ArbChainConfigUpdateTestsInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ArbChainConfigUpdateTestsInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ArbChainConfigUpdateTestsInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ArbChainConfigUpdateTestsInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ArbChainConfigUpdateTests`](self) contract instance.

See the [wrapper's documentation](`ArbChainConfigUpdateTestsInstance`) for more details.*/
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
        ) -> alloy_contract::Result<ArbChainConfigUpdateTestsInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> ArbChainConfigUpdateTestsInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ArbChainConfigUpdateTestsInstance<P, N> {
            ArbChainConfigUpdateTestsInstance {
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
    > ArbChainConfigUpdateTestsInstance<P, N> {
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
        ///Creates a new call builder for the [`APPCHAIN_BLOCK_EXPLORER_URL`] function.
        pub fn APPCHAIN_BLOCK_EXPLORER_URL(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, APPCHAIN_BLOCK_EXPLORER_URLCall, N> {
            self.call_builder(&APPCHAIN_BLOCK_EXPLORER_URLCall)
        }
        ///Creates a new call builder for the [`ARBITRUM_BRIDGE_ADDRESS`] function.
        pub fn ARBITRUM_BRIDGE_ADDRESS(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, ARBITRUM_BRIDGE_ADDRESSCall, N> {
            self.call_builder(&ARBITRUM_BRIDGE_ADDRESSCall)
        }
        ///Creates a new call builder for the [`ARBITRUM_INBOX_ADDRESS`] function.
        pub fn ARBITRUM_INBOX_ADDRESS(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, ARBITRUM_INBOX_ADDRESSCall, N> {
            self.call_builder(&ARBITRUM_INBOX_ADDRESSCall)
        }
        ///Creates a new call builder for the [`CHAIN_ID`] function.
        pub fn CHAIN_ID(&self) -> alloy_contract::SolCallBuilder<&P, CHAIN_IDCall, N> {
            self.call_builder(&CHAIN_IDCall)
        }
        ///Creates a new call builder for the [`DEFAULT_WS_RPC_URL`] function.
        pub fn DEFAULT_WS_RPC_URL(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DEFAULT_WS_RPC_URLCall, N> {
            self.call_builder(&DEFAULT_WS_RPC_URLCall)
        }
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<&P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall)
        }
        ///Creates a new call builder for the [`SEQUENCING_CHAIN_ID`] function.
        pub fn SEQUENCING_CHAIN_ID(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, SEQUENCING_CHAIN_IDCall, N> {
            self.call_builder(&SEQUENCING_CHAIN_IDCall)
        }
        ///Creates a new call builder for the [`SEQUENCING_CONTRACT_ADDRESS`] function.
        pub fn SEQUENCING_CONTRACT_ADDRESS(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, SEQUENCING_CONTRACT_ADDRESSCall, N> {
            self.call_builder(&SEQUENCING_CONTRACT_ADDRESSCall)
        }
        ///Creates a new call builder for the [`SEQUENCING_START_BLOCK`] function.
        pub fn SEQUENCING_START_BLOCK(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, SEQUENCING_START_BLOCKCall, N> {
            self.call_builder(&SEQUENCING_START_BLOCKCall)
        }
        ///Creates a new call builder for the [`SETTLEMENT_DELAY`] function.
        pub fn SETTLEMENT_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, SETTLEMENT_DELAYCall, N> {
            self.call_builder(&SETTLEMENT_DELAYCall)
        }
        ///Creates a new call builder for the [`SETTLEMENT_START_BLOCK`] function.
        pub fn SETTLEMENT_START_BLOCK(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, SETTLEMENT_START_BLOCKCall, N> {
            self.call_builder(&SETTLEMENT_START_BLOCKCall)
        }
        ///Creates a new call builder for the [`appchainOwner`] function.
        pub fn appchainOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, appchainOwnerCall, N> {
            self.call_builder(&appchainOwnerCall)
        }
        ///Creates a new call builder for the [`chainConfig`] function.
        pub fn chainConfig(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, chainConfigCall, N> {
            self.call_builder(&chainConfigCall)
        }
        ///Creates a new call builder for the [`configManager`] function.
        pub fn configManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, configManagerCall, N> {
            self.call_builder(&configManagerCall)
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
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
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
        ///Creates a new call builder for the [`testEmptyAppchainBlockExplorerUrl`] function.
        pub fn testEmptyAppchainBlockExplorerUrl(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testEmptyAppchainBlockExplorerUrlCall,
            N,
        > {
            self.call_builder(&testEmptyAppchainBlockExplorerUrlCall)
        }
        ///Creates a new call builder for the [`testEmptyDefaultRpcUrl`] function.
        pub fn testEmptyDefaultRpcUrl(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testEmptyDefaultRpcUrlCall, N> {
            self.call_builder(&testEmptyDefaultRpcUrlCall)
        }
        ///Creates a new call builder for the [`testTransferOwnership`] function.
        pub fn testTransferOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testTransferOwnershipCall, N> {
            self.call_builder(&testTransferOwnershipCall)
        }
        ///Creates a new call builder for the [`testTransferOwnershipRevertOnNonOwner`] function.
        pub fn testTransferOwnershipRevertOnNonOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testTransferOwnershipRevertOnNonOwnerCall,
            N,
        > {
            self.call_builder(&testTransferOwnershipRevertOnNonOwnerCall)
        }
        ///Creates a new call builder for the [`testTransferOwnershipRevertOnZeroAddress`] function.
        pub fn testTransferOwnershipRevertOnZeroAddress(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testTransferOwnershipRevertOnZeroAddressCall,
            N,
        > {
            self.call_builder(&testTransferOwnershipRevertOnZeroAddressCall)
        }
        ///Creates a new call builder for the [`testUpdateAppchainBlockExplorerUrl`] function.
        pub fn testUpdateAppchainBlockExplorerUrl(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testUpdateAppchainBlockExplorerUrlCall,
            N,
        > {
            self.call_builder(&testUpdateAppchainBlockExplorerUrlCall)
        }
        ///Creates a new call builder for the [`testUpdateAppchainBlockExplorerUrlRevertOnNonOwner`] function.
        pub fn testUpdateAppchainBlockExplorerUrlRevertOnNonOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall,
            N,
        > {
            self.call_builder(&testUpdateAppchainBlockExplorerUrlRevertOnNonOwnerCall)
        }
        ///Creates a new call builder for the [`testUpdateDefaultSequencingChainRpcUrl`] function.
        pub fn testUpdateDefaultSequencingChainRpcUrl(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testUpdateDefaultSequencingChainRpcUrlCall,
            N,
        > {
            self.call_builder(&testUpdateDefaultSequencingChainRpcUrlCall)
        }
        ///Creates a new call builder for the [`testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner`] function.
        pub fn testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall,
            N,
        > {
            self.call_builder(
                &testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwnerCall,
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ArbChainConfigUpdateTestsInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AppchainBlockExplorerUrlUpdated`] event.
        pub fn AppchainBlockExplorerUrlUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, AppchainBlockExplorerUrlUpdated, N> {
            self.event_filter::<AppchainBlockExplorerUrlUpdated>()
        }
        ///Creates a new event filter for the [`ArbChainConfigCreated`] event.
        pub fn ArbChainConfigCreated_filter(
            &self,
        ) -> alloy_contract::Event<&P, ArbChainConfigCreated, N> {
            self.event_filter::<ArbChainConfigCreated>()
        }
        ///Creates a new event filter for the [`DefaultSequencingChainWsRpcUrlUpdated`] event.
        pub fn DefaultSequencingChainWsRpcUrlUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, DefaultSequencingChainWsRpcUrlUpdated, N> {
            self.event_filter::<DefaultSequencingChainWsRpcUrlUpdated>()
        }
        ///Creates a new event filter for the [`ImplementationUpgraded`] event.
        pub fn ImplementationUpgraded_filter(
            &self,
        ) -> alloy_contract::Event<&P, ImplementationUpgraded, N> {
            self.event_filter::<ImplementationUpgraded>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
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
