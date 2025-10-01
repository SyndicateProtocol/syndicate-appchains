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

interface EmissionsForkTest {
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
    function acceptedDiff() external view returns (uint256);
    function emissionsCalculator() external view returns (address);
    function emissionsScheduler() external view returns (address);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function expectedMintAmount_ChangeFactor098(uint256 epoch) external pure returns (uint256);
    function expectedMintAmount_ChangeFactor101(uint256 epoch) external pure returns (uint256);
    function expectedMintAmount_ChangeFactorMultiple(uint256 epoch) external pure returns (uint256);
    function failed() external view returns (bool);
    function round(uint256 _weiAmount) external pure returns (uint256);
    function setUp() external;
    function startEpoch() external view returns (uint256);
    function syndTokenAdmin() external view returns (address);
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function test_emissions_ChangeFactor098() external;
    function test_emissions_ChangeFactor101() external;
    function test_emissions_ChangeFactorFlat() external;
    function test_emissions_ChangeFactorMultiple() external;
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
    "name": "acceptedDiff",
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
    "name": "emissionsCalculator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract EmissionsCalculator"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "emissionsScheduler",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract EmissionsScheduler"
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
    "name": "expectedMintAmount_ChangeFactor098",
    "inputs": [
      {
        "name": "epoch",
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
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "expectedMintAmount_ChangeFactor101",
    "inputs": [
      {
        "name": "epoch",
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
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "expectedMintAmount_ChangeFactorMultiple",
    "inputs": [
      {
        "name": "epoch",
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
    "stateMutability": "pure"
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
    "name": "round",
    "inputs": [
      {
        "name": "_weiAmount",
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
    "stateMutability": "pure"
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
    "name": "startEpoch",
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
    "name": "syndTokenAdmin",
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
    "name": "test_emissions_ChangeFactor098",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_emissions_ChangeFactor101",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_emissions_ChangeFactorFlat",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_emissions_ChangeFactorMultiple",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
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
pub mod EmissionsForkTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60808060405234609557600c805460ff199081166001908117909255601f805490911690911790556003602055600a602155602280546001600160a01b0319908116731bab804803159ad84b8854581aa53ac72455614e1790915560238054821673243c63d5dbcf619ee36fde7ff63d1564d5665b411790556024805482169055602580549091169055616ff4908161009a8239f35b5f80fdfe60806040526004361015610011575f80fd5b5f5f3560e01c8063037d52d61461374757806306e99e851461372a578063078c665a146137045780630a9254e414612e0f5780631b80a4e51461245d5780631ed7831c146123df5780631f001db4146123b85780632312d7d7146123915780632ade38801461220c5780632ee8823d146121ed57806334d5f37b146121ce5780633e5e3c23146121505780633f7286f4146120d25780634c79cca6146120ab5780635184c5e71461192157806366d9a9a0146117e45780637e041c0414610fb957806385226c8114610f27578063916a17c614610e7d578063a2c8b17714610e60578063b0464fdc14610db6578063b5508aa914610d24578063ba414fa614610cff578063be22cc59146101eb578063e20c9f711461015d5763fa7626d414610138575f80fd5b3461015a578060031936011261015a57602060ff601f54166040519015158152f35b80fd5b503461015a578060031936011261015a5760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106101cc576101c8856101bc81870382613976565b60405191829182613765565b0390f35b82546001600160a01b03168452602090930192600192830192016101a5565b503461015a578060031936011261015a57806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561073757604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57610cea575b506001600160a01b0360245416803b15610737578180916024604051809481937fc63a0944000000000000000000000000000000000000000000000000000000008352670e043da61725000060048401525af1801561072c57610cd5575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57610cc0575b50506001600160a01b03602554166040519063f508e19d60e01b8252602082600481845afa918215610a00578392610c8c575b5081600460206001600160a01b0360245416604051928380927f766718080000000000000000000000000000000000000000000000000000000082525afa8015610b3f578590610c58575b6103ca91506144b5565b6040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa908115610b3f578591610c26575b50602054905f198201918211610bf95760209261042a6004959361042f93614535565b6144b5565b604051928380927f5adf00210000000000000000000000000000000000000000000000000000000082525afa8015610a0057610472918491610bda575b506145ab565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357816040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f811fcbd7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57610bc5575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57610bb0575b505b602f81111561082257506001600160a01b0360255416906040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa9081156107b85784916107f0575b5060205490602f82018092116107c357906105c291614535565b60405163f508e19d60e01b8152602081600481865afa9081156107b857849161077e575b50916105f6602092600494614535565b604051928380927fa4d7e31d0000000000000000000000000000000000000000000000000000000082525afa801561072c5761063991839161074f575b5061469b565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f4555892c000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c5761073a575b506001600160a01b0360255416803b15610737578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c5761071b5750f35b8161072591613976565b61015a5780f35b6040513d84823e3d90fd5b50fd5b8161074491613976565b61015a57805f6106c9565b610771915060203d602011610777575b6107698183613976565b810190613bef565b5f610633565b503d61075f565b9290506020833d6020116107b0575b8161079a60209383613976565b810103126107ac5791516105f66105e6565b5f80fd5b3d915061078d565b6040513d86823e3d90fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d60201161081a575b8161080b60209383613976565b810103126107ac57515f6105a8565b3d91506107fe565b90826001600160a01b0360255416602061083d815486613be2565b6024604051809481937f0175e23b00000000000000000000000000000000000000000000000000000000835260048301525afa90811561072c578291610b7c575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357604051907fe5d6bf020000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57610b67575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57610b4a575b50506001600160a01b03602554169060405163f508e19d60e01b8152602081600481865afa908115610b3f578591610b0c575b506109849161097f91613bd5565b613f24565b61098d83613d30565b90818103610a0b575b505060206004916040519283809263f508e19d60e01b82525afa8015610a005783906109cd575b6109c8915091613c07565b610554565b506020813d82116109f8575b816109e660209383613976565b810103126107ac576109c890516109bd565b3d91506109d9565b6040513d85823e3d90fd5b610a4e818386604051610a1f604082613976565b602081527f45706f63682025733a2045787065637465642025732c2041637475616c202573602082015261461d565b602154610a5b8184613be2565b8211928315610af8575b505050610a73575f80610996565b6040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602760248201527f4d696e7420616d6f756e74206973206e6f742077697468696e2061636365707460448201527f65642064696666000000000000000000000000000000000000000000000000006064820152608490fd5b610b03929350613bd5565b115f8080610a65565b90506020813d8211610b37575b81610b2660209383613976565b810103126107ac5751610984610971565b3d9150610b19565b6040513d87823e3d90fd5b81610b5491613976565b610b5f57825f61093e565b8280fd5b5080fd5b81610b7191613976565b610b5f57825f6108ee565b9150506020813d8211610ba8575b81610b9760209383613976565b810103126107ac578390515f61087e565b3d9150610b8a565b81610bba91613976565b610b6357815f610552565b81610bcf91613976565b610b6357815f610502565b610bf3915060203d602011610777576107698183613976565b5f61046c565b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d602011610c50575b81610c4160209383613976565b810103126107ac57515f610407565b3d9150610c34565b506020813d602011610c84575b81610c7260209383613976565b810103126107ac576103ca90516103c0565b3d9150610c65565b9091506020813d602011610cb8575b81610ca860209383613976565b810103126107ac5751905f610375565b3d9150610c9b565b81610cca91613976565b61015a57805f610342565b81610cdf91613976565b61015a57805f6102d6565b81610cf491613976565b61015a57805f610278565b503461015a578060031936011261015a576020610d1a6143dc565b6040519015158152f35b503461015a578060031936011261015a57601954610d4181613c15565b91610d4f6040519384613976565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b838310610d9957604051602080825281906101c8908201886137cc565b600160208192610da885613c2d565b815201920192019190610d7c565b503461015a578060031936011261015a57601c54610dd381613c15565b91610de16040519384613976565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310610e2357604051806101c88782613879565b60026020600192604051610e3681613910565b6001600160a01b038654168152610e4e858701614138565b83820152815201920192019190610e0e565b503461015a578060031936011261015a5760208054604051908152f35b503461015a578060031936011261015a57601d54610e9a81613c15565b91610ea86040519384613976565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310610eea57604051806101c88782613879565b60026020600192604051610efd81613910565b6001600160a01b038654168152610f15858701614138565b83820152815201920192019190610ed5565b503461015a578060031936011261015a57601a54610f4481613c15565b91610f526040519384613976565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b838310610f9c57604051602080825281906101c8908201886137cc565b600160208192610fab85613c2d565b815201920192019190610f7f565b503461015a578060031936011261015a57806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561073757604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c576117cf575b506001600160a01b0360245416803b15610737578180916024604051809481937fc63a0944000000000000000000000000000000000000000000000000000000008352670de0b6b3a764000060048401525af1801561072c576117ba575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c576117a5575b50506001600160a01b03602554166040519063f508e19d60e01b8252602082600481845afa918215610a00578392611771575b5081600460206001600160a01b0360245416604051928380927f766718080000000000000000000000000000000000000000000000000000000082525afa8015610b3f57859061173d575b61119891506144b5565b6040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa908115610b3f57859161170b575b50602054905f198201918211610bf95760209261042a600495936111f893614535565b604051928380927f5adf00210000000000000000000000000000000000000000000000000000000082525afa8015610a005761123a918491610bda57506145ab565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357816040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f811fcbd7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c576116f6575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c576116e1575b505b602f81111561138957506001600160a01b0360255416906040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa9081156107b85784916107f0575060205490602f82018092116107c357906105c291614535565b90826001600160a01b036025541660206113a4815486613be2565b6024604051809481937f0175e23b00000000000000000000000000000000000000000000000000000000835260048301525afa90811561072c5782916116ad575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357604051907fe5d6bf020000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57611698575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57611683575b50506001600160a01b03602554169060405163f508e19d60e01b8152602081600481865afa908115610b3f578591611650575b506114e69161097f91613bd5565b62196e6b810361155a575b5060206004916040519283809263f508e19d60e01b82525afa8015610a00578390611527575b611522915091613c07565b61131c565b506020813d8211611552575b8161154060209383613976565b810103126107ac576115229051611517565b3d9150611533565b5f806115d76115fa604051611570604082613976565b602081527f45706f63682025733a2045787065637465642025732c2041637475616c20257360208201526040519283917fa7a878530000000000000000000000000000000000000000000000000000000060208401526080602484015260a48301906137a7565b88604483015262196e6b606483015286608483015203601f198101835282613976565b6020815191016a636f6e736f6c652e6c6f675afa50602154908162196e6b018062196e6b11610bf9578111918215611639575b5050610a73575f6114f1565b62196e6b908103925082116107c357105f8061162d565b90506020813d821161167b575b8161166a60209383613976565b810103126107ac57516114e66114d8565b3d915061165d565b8161168d91613976565b610b5f57825f6114a5565b816116a291613976565b610b5f57825f611455565b9150506020813d82116116d9575b816116c860209383613976565b810103126107ac578390515f6113e5565b3d91506116bb565b816116eb91613976565b610b6357815f61131a565b8161170091613976565b610b6357815f6112ca565b90506020813d602011611735575b8161172660209383613976565b810103126107ac57515f6111d5565b3d9150611719565b506020813d602011611769575b8161175760209383613976565b810103126107ac57611198905161118e565b3d915061174a565b9091506020813d60201161179d575b8161178d60209383613976565b810103126107ac5751905f611143565b3d9150611780565b816117af91613976565b61015a57805f611110565b816117c491613976565b61015a57805f6110a4565b816117d991613976565b61015a57805f611046565b503461015a578060031936011261015a57601b5461180181613c15565b61180e6040519182613976565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b8383106118e657868587604051928392602084019060208552518091526040840160408260051b8601019392905b82821061187b57505050500390f35b919360206118d6827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc06001959799849503018652885190836118c683516040845260408401906137a7565b9201519084818403910152613824565b960192019201859493919261186c565b600260206001926040516118f981613910565b61190286613c2d565b815261190f858701614138565b8382015281520192019201919061183e565b503461015a578060031936011261015a57806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561073757604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612096575b506001600160a01b0360245416803b15610737578180916024604051809481937fc63a0944000000000000000000000000000000000000000000000000000000008352670d99a8cec7e2000060048401525af1801561072c57612081575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c5761206c575b50506001600160a01b03602554166040519063f508e19d60e01b8252602082600481845afa918215610a00578392612038575b5081600460206001600160a01b0360245416604051928380927f766718080000000000000000000000000000000000000000000000000000000082525afa8015610b3f578590612004575b611b0091506144b5565b6040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa908115610b3f578591611fd2575b50602054905f198201918211610bf95760209261042a60049593611b6093614535565b604051928380927f5adf00210000000000000000000000000000000000000000000000000000000082525afa8015610a0057611ba2918491610bda57506145ab565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357816040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f811fcbd7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57611fbd575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57611fa8575b505b602f811115611cf157506001600160a01b0360255416906040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa9081156107b85784916107f0575060205490602f82018092116107c357906105c291614535565b90826001600160a01b03602554166020611d0c815486613be2565b6024604051809481937f0175e23b00000000000000000000000000000000000000000000000000000000835260048301525afa90811561072c578291611f74575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357604051907fe5d6bf020000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57611f5f575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57611f4a575b50506001600160a01b03602554169060405163f508e19d60e01b8152602081600481865afa908115610b3f578591611f17575b50611e4e9161097f91613bd5565b611e57836139d7565b90818103611eca575b505060206004916040519283809263f508e19d60e01b82525afa8015610a00578390611e97575b611e92915091613c07565b611c84565b506020813d8211611ec2575b81611eb060209383613976565b810103126107ac57611e929051611e87565b3d9150611ea3565b611ede818386604051610a1f604082613976565b602154611eeb8184613be2565b8211928315611f03575b505050610a73575f80611e60565b611f0e929350613bd5565b115f8080611ef5565b90506020813d8211611f42575b81611f3160209383613976565b810103126107ac5751611e4e611e40565b3d9150611f24565b81611f5491613976565b610b5f57825f611e0d565b81611f6991613976565b610b5f57825f611dbd565b9150506020813d8211611fa0575b81611f8f60209383613976565b810103126107ac578390515f611d4d565b3d9150611f82565b81611fb291613976565b610b6357815f611c82565b81611fc791613976565b610b6357815f611c32565b90506020813d602011611ffc575b81611fed60209383613976565b810103126107ac57515f611b3d565b3d9150611fe0565b506020813d602011612030575b8161201e60209383613976565b810103126107ac57611b009051611af6565b3d9150612011565b9091506020813d602011612064575b8161205460209383613976565b810103126107ac5751905f611aab565b3d9150612047565b8161207691613976565b61015a57805f611a78565b8161208b91613976565b61015a57805f611a0c565b816120a091613976565b61015a57805f6119ae565b503461015a57602060031936011261015a5760206120ca600435613f44565b604051908152f35b503461015a578060031936011261015a5760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110612131576101c8856101bc81870382613976565b82546001600160a01b031684526020909301926001928301920161211a565b503461015a578060031936011261015a5760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b8181106121af576101c8856101bc81870382613976565b82546001600160a01b0316845260209093019260019283019201612198565b503461015a57602060031936011261015a5760206120ca600435613f24565b503461015a57602060031936011261015a5760206120ca600435613d30565b503461015a578060031936011261015a57601e5461222981613c15565b6122366040519182613976565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b83831061230857868587604051928392602084019060208552518091526040840160408260051b8601019392905b8282106122a357505050500390f35b919360206122f8827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc060019597998495030186526040838a516001600160a01b038151168452015191818582015201906137cc565b9601920192018594939192612294565b60405161231481613910565b6001600160a01b03835416815260018301805461233081613c15565b9161233e6040519384613976565b8183528a526020808b20908b9084015b838210612374575050505060019282602092836002950152815201920192019190612266565b60016020819261238386613c2d565b81520193019101909161234e565b503461015a578060031936011261015a5760206001600160a01b0360245416604051908152f35b503461015a578060031936011261015a5760206001600160a01b0360255416604051908152f35b503461015a578060031936011261015a5760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b81811061243e576101c8856101bc81870382613976565b82546001600160a01b0316845260209093019260019283019201612427565b503461015a578060031936011261015a57806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561073757604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612dfa575b506001600160a01b0360245416803b15610737578180916024604051809481937fc63a0944000000000000000000000000000000000000000000000000000000008352670de0b6b3a764000060048401525af1801561072c57612de5575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612dd0575b50506001600160a01b03602554166040519063f508e19d60e01b8252602082600481845afa918215610a00578392612d9c575b5081600460206001600160a01b0360245416604051928380927f766718080000000000000000000000000000000000000000000000000000000082525afa8015610b3f578590612d68575b61263c91506144b5565b6040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa908115610b3f578591612d36575b50602054905f198201918211610bf95760209261042a6004959361269c93614535565b604051928380927f5adf00210000000000000000000000000000000000000000000000000000000082525afa8015610a00576126de918491610bda57506145ab565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357816040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f811fcbd7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612d21575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57612d0c575b505b602f81111561282d57506001600160a01b0360255416906040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa9081156107b85784916107f0575060205490602f82018092116107c357906105c291614535565b9060048214612c01575b60158214612af6575b826001600160a01b0360255416602061285a815486613be2565b6024604051809481937f0175e23b00000000000000000000000000000000000000000000000000000000835260048301525afa90811561072c578291612ac2575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357604051907fe5d6bf020000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612aad575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57612a98575b50506001600160a01b03602554169060405163f508e19d60e01b8152602081600481865afa908115610b3f578591612a65575b5061299c9161097f91613bd5565b6129a583613f44565b90818103612a18575b505060206004916040519283809263f508e19d60e01b82525afa8015610a005783906129e5575b6129e0915091613c07565b6127c0565b506020813d8211612a10575b816129fe60209383613976565b810103126107ac576129e090516129d5565b3d91506129f1565b612a2c818386604051610a1f604082613976565b602154612a398184613be2565b8211928315612a51575b505050610a73575f806129ae565b612a5c929350613bd5565b115f8080612a43565b90506020813d8211612a90575b81612a7f60209383613976565b810103126107ac575161299c61298e565b3d9150612a72565b81612aa291613976565b610b5f57825f61295b565b81612ab791613976565b610b5f57825f61290b565b9150506020813d8211612aee575b81612add60209383613976565b810103126107ac578390515f61289b565b3d9150612ad0565b826001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612bec575b506001600160a01b0360245416803b15610b63578180916024604051809481937f43581010000000000000000000000000000000000000000000000000000000008352670dcef33a6f83800060048401525af1801561072c57612bd7575b5050612840565b81612be191613976565b610b5f57825f612bd0565b81612bf691613976565b610b5f57825f612b72565b826001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612cf7575b506001600160a01b0360245416803b15610b63578180916024604051809481937f43581010000000000000000000000000000000000000000000000000000000008352670e16011f4f05800060048401525af1801561072c57612ce2575b5050612837565b81612cec91613976565b610b5f57825f612cdb565b81612d0191613976565b610b5f57825f612c7d565b81612d1691613976565b610b6357815f6127be565b81612d2b91613976565b610b6357815f61276e565b90506020813d602011612d60575b81612d5160209383613976565b810103126107ac57515f612679565b3d9150612d44565b506020813d602011612d94575b81612d8260209383613976565b810103126107ac5761263c9051612632565b3d9150612d75565b9091506020813d602011612dc8575b81612db860209383613976565b810103126107ac5751905f6125e7565b3d9150612dab565b81612dda91613976565b61015a57805f6125b4565b81612def91613976565b61015a57805f612548565b81612e0491613976565b61015a57805f6124ea565b50346107ac575f6003193601126107ac576040517f9868003400000000000000000000000000000000000000000000000000000000815260206004820152601460248201527f68747470733a2f2f30787270632e696f2f65746800000000000000000000000060448201526020816064815f737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561369d576136d5575b506020547ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81019081116136a85762278d0081029080820462278d0014901517156136a85763688d46f0018063688d46f0116136a857737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ac57604051907fe5d6bf0200000000000000000000000000000000000000000000000000000000825260048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561369d5761368a575b506001600160a01b0360245416158015613677575b612f845780f35b5f80613027613035604051612f9a606082613976565b603581527f456d697373696f6e7320636f6e747261637473206e6f7420666f756e642c206460208201527f65706c6f79696e67206f6e657320746f20666f726b000000000000000000000060408201526040519283917f41304fac0000000000000000000000000000000000000000000000000000000060208401526020602484015260448301906137a7565b03601f198101835282613976565b6020815191016a636f6e736f6c652e6c6f675afa5060405160868082019082821067ffffffffffffffff83111761364a5790829161470e8339039082f080156135e3576001600160a01b03602254166001600160a01b03602354169060405191611342918284019284841067ffffffffffffffff85111761361d579160609391859361479485398252806020830152604082015203019083f0801561072c576001600160a01b031690817fffffffffffffffffffffffff00000000000000000000000000000000000000006024541617602455602054906001600160a01b0360235416906040519361151e938486019486861067ffffffffffffffff8711176135f057928694926001600160a01b039260c09795615ad688398552602085015216604083015280606083015280608083015260a082015203019082f080156135e3576001600160a01b03167fffffffffffffffffffffffff0000000000000000000000000000000000000000602554161760255580600460206001600160a01b0360225416604051928380927f8d3343d60000000000000000000000000000000000000000000000000000000082525afa90811561072c5782916135ae575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561353657604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610a00578391613599575b50506001600160a01b0360225416906001600160a01b0360245416823b1561351c576040517f2f2ff15d00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529082908290604490829084905af1801561072c57613584575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c5761356f575b50600460206001600160a01b0360245416604051928380927fdebe4f1f0000000000000000000000000000000000000000000000000000000082525afa90811561072c57829161353a575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561353657604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610a00578391613521575b50506001600160a01b0360245416906001600160a01b0360255416823b1561351c576040517f2f2ff15d00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529082908290604490829084905af1801561072c57613507575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c5761071b57505080f35b8161351191613976565b61015a57805f613497565b505050fd5b8161352b91613976565b61073757815f613420565b5050fd5b9150506020813d602011613567575b8161355660209383613976565b810103126107ac578190515f6133a1565b3d9150613549565b8161357991613976565b61015a57805f613356565b8161358e91613976565b61015a57805f6132ea565b816135a391613976565b61073757815f613273565b9150506020813d6020116135db575b816135ca60209383613976565b810103126107ac578190515f6131f4565b3d91506135bd565b50604051903d90823e3d90fd5b6024887f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b6024877f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b506001600160a01b036025541615612f7d565b61369691505f90613976565b5f5f612f68565b6040513d5f823e3d90fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b6020813d6020116136fc575b816136ee60209383613976565b810103126107ac5751612ea3565b3d91506136e1565b346107ac575f6003193601126107ac5760206001600160a01b0360235416604051908152f35b346107ac575f6003193601126107ac576020602154604051908152f35b346107ac5760206003193601126107ac5760206120ca6004356139d7565b60206040818301928281528451809452019201905f5b8181106137885750505090565b82516001600160a01b031684526020938401939092019160010161377b565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b9080602083519182815201916020808360051b8301019401925f915b8383106137f757505050505090565b909192939460208061381583601f19866001960301875289516137a7565b970193019301919392906137e8565b90602080835192838152019201905f5b8181106138415750505090565b82517fffffffff0000000000000000000000000000000000000000000000000000000016845260209384019390920191600101613834565b602081016020825282518091526040820191602060408360051b8301019401925f915b8383106138ab57505050505090565b9091929394602080613901837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b03815116845201519181858201520190613824565b9701930193019193929061389c565b6040810190811067ffffffffffffffff82111761392c57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610600810190811067ffffffffffffffff82111761392c57604052565b90601f601f19910116810190811067ffffffffffffffff82111761392c57604052565b9060308110156139aa5760051b0190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b604051906139e482613959565b6227536b825262268a1160208301526225c4bf60408301526225035f6060830152622445dd608083015262238c2660a08301526222d62560c0830152622223c860e0830152622174fc6101008301526220c9b0610120830152622021d0610140830152621f7d4c610160830152621edc12610180830152621e3e126101a0830152621da33a6101c0830152621d0b7c6101e0830152621c76c6610200830152621be50a610220830152621b5638610240830152621aca41610260830152621a41166102808301526219baab6102a0830152621936ef6102c08301526218b5d66102e0830152621837516103008301526217bb55610320830152621741d36103408301526216cac06103608301526216560e6103808301526215e3b16103a08301526215739e6103c0830152621505c96103e083015262149a27610400830152621430ab6104208301526213c94b610440830152621363fd610460830152621300b661048083015262129f6a6104a0830152621240116104c08301526211e2a06104e08301526211870e61050083015262112d506105208301526210d55e61054083015262107f2e61056083015262102ab7610580830152620fd7f16105a0830152620f86d46105c0830152620f37556105e08301526030811015613bcf57613bcb91613999565b5190565b50505f90565b919082039182116136a857565b919082018092116136a857565b908160209103126107ac575180151581036107ac5790565b5f1981146136a85760010190565b67ffffffffffffffff811161392c5760051b60200190565b90604051915f8154908160011c9260018316928315613d26575b602085108414613cf9578487528693908115613cb95750600114613c75575b50613c7392500383613976565b565b90505f9291925260205f20905f915b818310613c9d575050906020613c73928201015f613c66565b6020919350806001915483858901015201910190918492613c84565b60209350613c739592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f613c66565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f1693613c47565b60405190613d3d82613959565b6213f05282526214235d6020830152621456eb604083015262148afd60608301526214bf9460808301526214f4b260a083015262152a5760c08301526215608660e0830152621597406101008301526215ce866101208301526216065961014083015262163ebb610160830152621677ae6101808301526216b1326101a08301526216eb496101c0830152621725f66101e08301526217613861020083015262179d126102208301526217d986610240830152621816946102608301526218543e610280830152621892876102a08301526218d16f6102c0830152621910f76102e083015262195123610300830152621991f26103208301526219d368610340830152621a1585610360830152621a584c610380830152621a9bbd6103a0830152621adfdb6103c0830152621b24a86103e0830152621b6a25610400830152621bb053610420830152621bf735610440830152621c3ecd610460830152621c871c610480830152621cd0246104a0830152621d19e66104c0830152621d64666104e0830152621dafa5610500830152621dfba4610520830152621e4866610540830152621e95ec610560830152621ee439610580830152621f334e6105a0830152621f832e6105c0830152621fd3da6105e08301526030811015613bcf57613bcb91613999565b6706f05b59d3b2000081018091116136a857670de0b6b3a7640000900490565b60405190613f5182613959565b62196e6b825262196e6b602083015262196e6b604083015262196e6b60608301526212239960808301526212694060a08301526212aff360c08301526212f7b560e08301526213408b61010083015262138a796101208301526213d583610140830152621421ac61016083015262146efa6101808301526214bd716101a083015262150d166101c083015262155dec6101e08301526215aff861020083015262160340610220830152621657c76102408301526216ad93610260830152621704a8610280830152621e714c6102a0830152621e4a546102c0830152621e238f6102e0830152621dfcfb610300830152621dd698610320830152621db067610340830152621d8a67610360830152621d6497610380830152621d3ef76103a0830152621d19886103c0830152621cf4496103e0830152621ccf39610400830152621caa59610420830152621c85a7610440830152621c6125610460830152621c3cd2610480830152621c18ad6104a0830152621bf4b76104c0830152621bd0ee6104e0830152621bad53610500830152621b89e6610520830152621b66a7610540830152621b4394610560830152621b20ae610580830152621afdf56105a0830152621adb696105c0830152621ab9096105e08301526030811015613bcf57613bcb91613999565b90604051918281549182825260208201905f5260205f20925f905b80600783011061434f57613c73945491818110614319575b8181106142e3575b8181106142ad575b818110614277575b818110614241575b81811061420b575b8181106141d6575b106141a9575b500383613976565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f6141a1565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b16815201930161419b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301614193565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b16815201930161418b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301614183565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b16815201930161417b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b168152019301614173565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b16815201930161416b565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e0820152019401920185929391614153565b60085460ff1680156143eb5790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561369d575f91614483575b50151590565b90506020813d6020116144ad575b8161449e60209383613976565b810103126107ac57515f61447d565b3d9150614491565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ac57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561369d5761452b5750565b5f613c7391613976565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ac57604051917f98296c54000000000000000000000000000000000000000000000000000000008352600483015260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561369d5761452b5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ac57604051907fa5982885000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561369d5761452b5750565b61468490614666925f9586956040519586947fa7a878530000000000000000000000000000000000000000000000000000000060208701526080602487015260a48601906137a7565b9260448501526064840152608483015203601f198101835282613976565b6020815191016a636f6e736f6c652e6c6f675afa50565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ac57604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561369d5761452b575056fe60808060405234601357606e908160188239f35b5f80fdfe6004361015600b575f80fd5b5f3560e01c63eeec0e2414601d575f80fd5b34606a5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112606a5760043573ffffffffffffffffffffffffffffffffffffffff811603606a57005b5f80fd60a0346100d957601f61134238819003918201601f19168301916001600160401b038311848410176100dd578084926060946040528339810103126100d957610047816100f1565b61005f6040610058602085016100f1565b93016100f1565b906001600160a01b031680156100ca576001600160a01b038316156100ca576001600160a01b038216156100ca576100a39261009d91608052610105565b5061017b565b506040516110d3908161020f8239608051818181610321015281816107f10152610d540152f35b63d92e233d60e01b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036100d957565b6001600160a01b0381165f9081525f5160206113225f395f51905f52602052604090205460ff16610176576001600160a01b03165f8181525f5160206113225f395f51905f5260205260408120805460ff191660011790553391905f5160206112e25f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f5160206113025f395f51905f52602052604090205460ff16610176576001600160a01b03165f8181525f5160206113025f395f51905f5260205260408120805460ff191660011790553391907ff27f9368aa1b7697e50429f83722b88f2e5db8184d5e2cbcf060f4310bbd3e7d905f5160206112e25f395f51905f529080a460019056fe6080806040526004361015610012575f80fd5b5f905f3560e01c90816301ffc9a7146109fa57508063158ef93e146109d8578063248a9ca3146109ae5780632f2ff15d1461097157806336568abe14610905578063435810101461082f57806343a3f8a1146108155780635bdf6ca1146107c55780635f15c3c9146107aa578063766718081461078d578063891624861461075357806391d14854146106fd578063a088787d146106ba578063a217fddf146106a0578063ac12ce0714610683578063b198d0281461065e578063c63a094414610550578063d3f566ae14610256578063d547741f1461020f578063debe4f1f146101d4578063df0244b1146101b6578063e0e6169c1461019b578063e4b7fb7314610178578063eced5526146101555763fa391c6414610131575f80fd5b34610152578060031936011261015257602060306002541015604051908152f35b80fd5b50346101525780600319360112610152576020604051670de0b6b3a76400008152f35b50346101525780600319360112610152576020610193610d2c565b604051908152f35b50346101525780600319360112610152576020610193610cef565b50346101525780600319360112610152576020600354604051908152f35b503461015257806003193601126101525760206040517f076eb8b875b6ea839b087c4c0c1a4661b089d3b6ee2c1ef1b9cfa7fe1066d2068152f35b50346101525760406003193601126101525761025260043561022f610a98565b9061024d610248825f525f602052600160405f20015490565b610ed3565b61100b565b5080f35b5034610451576040600319360112610451576004359073ffffffffffffffffffffffffffffffffffffffff821680920361045157335f9081527f0e25390ff9535358a5e916dfe7d38266c83601af6e112105b22df4a90bf8910160205260409020546024359060ff16156105005760ff6004541615610482576002549060308210156104825783156104d8578082036104aa5750506102f3610b05565b908115610482576003548281018091116104555760035573ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b15610451575f80916044604051809481937f40c10f190000000000000000000000000000000000000000000000000000000083528960048401528860248401525af1801561044657610433575b5060025492837f160fc195d6e53691d30d804ce190dc09471891677e43433b91a7a6131c12a59a60406103c1610d2c565b8151908782526020820152a37fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83146104065750600160209201600255604051908152f35b807f4e487b7100000000000000000000000000000000000000000000000000000000602492526011600452fd5b61043f91505f90610c81565b5f5f610390565b6040513d5f823e3d90fd5b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7f9e91c9e7000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f30413a1a000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f076eb8b875b6ea839b087c4c0c1a4661b089d3b6ee2c1ef1b9cfa7fe1066d20660245260445ffd5b3461045157602060031936011261045157335f9081527fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb560205260409020546004359060ff161561062e5760045460ff8116610482578115610606577fc12c60abc216286ef25e34b1805a0c3dda73e4c2fd6cf360e807a7a9e73167399160017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00602093161760045580600155604051908152a1005b7feb769920000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b34610451575f6003193601126104515760206040516a422ca8b0a00a42500000008152f35b34610451575f600319360112610451576020600154604051908152f35b34610451575f6003193601126104515760206040515f8152f35b34610451575f6003193601126104515760a06002546030600354916106dd610d2c565b604051938285528360208601526040850152606084015210156080820152f35b3461045157604060031936011261045157610716610a98565b6004355f525f60205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060ff60405f2054166040519015158152f35b34610451575f6003193601126104515760206040517ff27f9368aa1b7697e50429f83722b88f2e5db8184d5e2cbcf060f4310bbd3e7d8152f35b34610451575f600319360112610451576020600254604051908152f35b34610451575f60031936011261045157602060405160308152f35b34610451575f60031936011261045157602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610451575f600319360112610451576020610193610b05565b3461045157602060031936011261045157335f9081527f7e97327757452d9e3315e395bc100e7da1f2d35106fffd38f23807747090efa960205260409020546004359060ff16156108b557801561060657806001556002546040519182527fb813ffbe387d6cf6e6a6f6c5f8905f766a0f1c6cd01c67312f709356c62597bd60203393a3005b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527ff27f9368aa1b7697e50429f83722b88f2e5db8184d5e2cbcf060f4310bbd3e7d60245260445ffd5b346104515760406003193601126104515761091e610a98565b3373ffffffffffffffffffffffffffffffffffffffff821603610949576109479060043561100b565b005b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461045157604060031936011261045157610947600435610990610a98565b906109a9610248825f525f602052600160405f20015490565b610f39565b346104515760206003193601126104515760206101936004355f525f602052600160405f20015490565b34610451575f60031936011261045157602060ff600454166040519015158152f35b3461045157602060031936011261045157600435907fffffffff00000000000000000000000000000000000000000000000000000000821680920361045157817f7965db0b0000000000000000000000000000000000000000000000000000000060209314908115610a6e575b5015158152f35b7f01ffc9a70000000000000000000000000000000000000000000000000000000091501483610a67565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361045157565b8115610ac5570490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b8181029291811591840414171561045557565b60ff60045416158015610c74575b610c70576002546030036030811161045557610b2d610d2c565b60018214610c6b5760015491670de0b6b3a76400008314610c605750610b51610cef565b670de0b6b3a7640000811115610c19577ffffffffffffffffffffffffffffffffffffffffffffffffff21f494c589c00008101908111610455575b6103e8811015610c1357506103e8905b670de0b6b3a7640000831115610beb577ffffffffffffffffffffffffffffffffffffffffffffffffff21f494c589c0000830192831161045557610be892610be391610af2565b610abb565b90565b91670de0b6b3a76400000391670de0b6b3a7640000831161045557610be892610be391610af2565b90610b9c565b670de0b6b3a764000003670de0b6b3a7640000811115610b8c577f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b90610be89250610abb565b905090565b5f90565b5060306002541015610b13565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610cc257604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b670de0b6b3a76400006002546001545b60308210610d0c57505090565b9091670de0b6b3a7640000610d2383600193610af2565b04920190610cff565b6040517f18160ddd0000000000000000000000000000000000000000000000000000000081527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16602082600481845afa918215610446575f92610e9e575b506020600491604051928380927f902d55a50000000000000000000000000000000000000000000000000000000082525afa908115610446575f91610e6c575b507fffffffffffffffffffffffffffffffffffffffffffbdd3574f5ff5bdb0000000810181811161045557821115610e645781036a422ca8b0a00a425000000001908111610455575b806a422ca8b0a00a4250000000115f14610e5f576a422ca8b0a00a4250000000036a422ca8b0a00a425000000081116104555790565b505f90565b50505f610e29565b90506020813d602011610e96575b81610e8760209383610c81565b8101031261045157515f610de0565b3d9150610e7a565b9091506020813d602011610ecb575b81610eba60209383610c81565b810103126104515751906020610da0565b3d9150610ead565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f20541615610f0a5750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f1461100557805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f1461100557805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d7e97327757452d9e3315e395bc100e7da1f2d35106fffd38f23807747090efa9ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb560c03461017b57601f61151e38819003918201601f19168301916001600160401b0383118484101761017f5780849260c09460405283398101031261017b5780519061004d60208201610193565b61005960408301610193565b61006560608401610193565b9261007e60a061007760808401610193565b9201610193565b6001805460ff1916815560025592851561016c576001600160a01b031691821561015d576001600160a01b031693841561015d576001600160a01b0316801561015d576001600160a01b0382161561015d576001600160a01b0384161561015d576101159461010f9360805260018060a01b0319600354161760035560018060a01b031960045416176004556101a7565b5061021d565b5060a05260405161120d90816102b18239608051818181610190015281816108eb01528181610c360152610ec8015260a05181818161034d015281816106b6015261093a0152f35b63d92e233d60e01b5f5260045ffd5b63d5b25b6360e01b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b038216820361017b57565b6001600160a01b0381165f9081525f5160206114fe5f395f51905f52602052604090205460ff16610218576001600160a01b03165f8181525f5160206114fe5f395f51905f5260205260408120805460ff191660011790553391905f5160206114be5f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f5160206114de5f395f51905f52602052604090205460ff16610218576001600160a01b03165f8181525f5160206114de5f395f51905f5260205260408120805460ff191660011790553391907f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a905f5160206114be5f395f51905f529080a460019056fe6080806040526004361015610012575f80fd5b5f905f3560e01c9081630175e23b14610cfb5750806301ffc9a714610c5a5780632312d7d714610c0a578063248a9ca314610be0578063284e1333146108915780632f2ff15d14610853578063311f5169146107e957806336568abe1461077e5780633f4ba83a146106df5780635adf0021146106995780635c975abb146106765780636548e9bc146105e45780636fd3c9f0146105b0578063781cd99d146105915780638406c0791461055d5780638456cb591461044957806391d14854146103f2578063a217fddf146103d6578063a4d7e31d146103b1578063a70b9f0c14610393578063b97dd9e214610370578063bccf24e314610335578063d5176d2314610292578063d547741f1461024b578063e63ab1e9146102105763f508e19d1461013c575f80fd5b3461020d578060031936011261020d57604051907fdf0244b100000000000000000000000000000000000000000000000000000000825260208260048173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa90811561020157906101ca575b602090604051908152f35b506020813d6020116101f9575b816101e460209383610e17565b810103126101f557602090516101bf565b5f80fd5b3d91506101d7565b604051903d90823e3d90fd5b80fd5b503461020d578060031936011261020d5760206040517f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a8152f35b503461020d57604060031936011261020d5761028e60043561026b610dc4565b90610289610284825f525f602052600160405f20015490565b61100d565b611145565b5080f35b503461020d57602060031936011261020d5760043562278d0081029080820462278d0014901517156103085763688d46f001908163688d46f0116102db57602082604051908152f35b807f4e487b7100000000000000000000000000000000000000000000000000000000602492526011600452fd5b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b503461020d578060031936011261020d5760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b503461020d578060031936011261020d57602061038b610f33565b604051908152f35b503461020d578060031936011261020d57602060405162278d008152f35b503461020d578060031936011261020d5760206103cc610e85565b6040519015158152f35b503461020d578060031936011261020d57602090604051908152f35b503461020d57604060031936011261020d5773ffffffffffffffffffffffffffffffffffffffff6040610423610dc4565b926004358152806020522091165f52602052602060ff60405f2054166040519015158152f35b503461020d578060031936011261020d577f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a8152806020526040812073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f2054161561050d576104b6610f71565b60017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416176001557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a180f35b807fe2517d3f0000000000000000000000000000000000000000000000000000000060449252336004527f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a602452fd5b503461020d578060031936011261020d57602073ffffffffffffffffffffffffffffffffffffffff60035416604051908152f35b503461020d578060031936011261020d57602060405163688d46f08152f35b503461020d578060031936011261020d57602073ffffffffffffffffffffffffffffffffffffffff60045416604051908152f35b503461020d57602060031936011261020d5773ffffffffffffffffffffffffffffffffffffffff610613610de7565b61061b610fa5565b16801561064e577fffffffffffffffffffffffff0000000000000000000000000000000000000000600354161760035580f35b6004827fd92e233d000000000000000000000000000000000000000000000000000000008152fd5b503461020d578060031936011261020d57602060ff600154166040519015158152f35b503461020d578060031936011261020d5760206106b4610f33565b7f00000000000000000000000000000000000000000000000000000000000000001115604051908152f35b503461020d578060031936011261020d576106f8610fa5565b60015460ff811615610756577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6020604051338152a180f35b6004827f8dfc202b000000000000000000000000000000000000000000000000000000008152fd5b503461020d57604060031936011261020d57610798610dc4565b3373ffffffffffffffffffffffffffffffffffffffff8216036107c15761028e90600435611145565b6004827f6697b232000000000000000000000000000000000000000000000000000000008152fd5b503461020d57602060031936011261020d5773ffffffffffffffffffffffffffffffffffffffff610818610de7565b610820610fa5565b16801561064e577fffffffffffffffffffffffff0000000000000000000000000000000000000000600454161760045580f35b503461020d57604060031936011261020d5761028e600435610873610dc4565b9061088c610284825f525f602052600160405f20015490565b611073565b50346101f5575f6003193601126101f5576108aa610f71565b6002805414610bb857600280556108bf610e85565b610b90576040517f766718080000000000000000000000000000000000000000000000000000000081527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16602082600481845afa918215610af5575f92610b5c575b507f00000000000000000000000000000000000000000000000000000000000000006109638184610e0a565b61096b610f33565b10610b34576003546040517fd3f566ae00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024810184905291602090839060449082905f905af1918215610af5575f92610b00575b5073ffffffffffffffffffffffffffffffffffffffff6003541690610a1873ffffffffffffffffffffffffffffffffffffffff600454169185610e0a565b823b156101f5576040517feeec0e2400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9290921660048301526024820152905f908290604490829084905af18015610af557610aba575b507fef80c279c178dd63cdaf5061224be86ee325c4f4406d802d043ed33b45b2f68f9160409182519182526020820152a1600160025580f35b604091935091610aeb5f7fef80c279c178dd63cdaf5061224be86ee325c4f4406d802d043ed33b45b2f68f94610e17565b5f93915091610a81565b6040513d5f823e3d90fd5b9091506020813d602011610b2c575b81610b1c60209383610e17565b810103126101f55751905f6109da565b3d9150610b0f565b7f811fcbd7000000000000000000000000000000000000000000000000000000005f5260045ffd5b9091506020813d602011610b88575b81610b7860209383610e17565b810103126101f55751905f610937565b3d9150610b6b565b7f4555892c000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101f55760206003193601126101f557602061038b6004355f525f602052600160405f20015490565b346101f5575f6003193601126101f557602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101f55760206003193601126101f5576004357fffffffff0000000000000000000000000000000000000000000000000000000081168091036101f557807f7965db0b0000000000000000000000000000000000000000000000000000000060209214908115610cd1575b506040519015158152f35b7f01ffc9a70000000000000000000000000000000000000000000000000000000091501482610cc6565b346101f55760206003193601126101f5576004358015610d9c577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8101908111610d6f5762278d0081029080820462278d001490151715610d6f5763688d46f001908163688d46f011610d6f576020918152f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7fd69368d4000000000000000000000000000000000000000000000000000000005f5260045ffd5b6024359073ffffffffffffffffffffffffffffffffffffffff821682036101f557565b6004359073ffffffffffffffffffffffffffffffffffffffff821682036101f557565b91908201809211610d6f57565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610e5857604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040517ffa391c6400000000000000000000000000000000000000000000000000000000815260208160048173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115610af5575f91610efb575090565b90506020813d602011610f2b575b81610f1660209383610e17565b810103126101f5575180151581036101f55790565b3d9150610f09565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff9772b9104201428111610d6f5762278d00900460018101809111610d6f5790565b60ff60015416610f7d57565b7fd93c0665000000000000000000000000000000000000000000000000000000005f5260045ffd5b335f9081527fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5602052604090205460ff1615610fdd57565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f205416156110445750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f1461113f57805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f1461113f57805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0df7c9542c591017a21c74b6f3fab6263c7952fc0aaf9db4c22a2a04ddc7f8674fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`\x95W`\x0C\x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U`\x1F\x80T\x90\x91\x16\x90\x91\x17\x90U`\x03` U`\n`!U`\"\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16s\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN\x17\x90\x91U`#\x80T\x82\x16s$<c\xD5\xDB\xCFa\x9E\xE3o\xDE\x7F\xF6=\x15d\xD5f[A\x17\x90U`$\x80T\x82\x16\x90U`%\x80T\x90\x91\x16\x90Uao\xF4\x90\x81a\0\x9A\x829\xF3[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x03}R\xD6\x14a7GW\x80c\x06\xE9\x9E\x85\x14a7*W\x80c\x07\x8CfZ\x14a7\x04W\x80c\n\x92T\xE4\x14a.\x0FW\x80c\x1B\x80\xA4\xE5\x14a$]W\x80c\x1E\xD7\x83\x1C\x14a#\xDFW\x80c\x1F\0\x1D\xB4\x14a#\xB8W\x80c#\x12\xD7\xD7\x14a#\x91W\x80c*\xDE8\x80\x14a\"\x0CW\x80c.\xE8\x82=\x14a!\xEDW\x80c4\xD5\xF3{\x14a!\xCEW\x80c>^<#\x14a!PW\x80c?r\x86\xF4\x14a \xD2W\x80cLy\xCC\xA6\x14a \xABW\x80cQ\x84\xC5\xE7\x14a\x19!W\x80cf\xD9\xA9\xA0\x14a\x17\xE4W\x80c~\x04\x1C\x04\x14a\x0F\xB9W\x80c\x85\"l\x81\x14a\x0F'W\x80c\x91j\x17\xC6\x14a\x0E}W\x80c\xA2\xC8\xB1w\x14a\x0E`W\x80c\xB0FO\xDC\x14a\r\xB6W\x80c\xB5P\x8A\xA9\x14a\r$W\x80c\xBAAO\xA6\x14a\x0C\xFFW\x80c\xBE\"\xCCY\x14a\x01\xEBW\x80c\xE2\x0C\x9Fq\x14a\x01]Wc\xFAv&\xD4\x14a\x018W_\x80\xFD[4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x01\xCCWa\x01\xC8\x85a\x01\xBC\x81\x87\x03\x82a9vV[`@Q\x91\x82\x91\x82a7eV[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x01\xA5V[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x077W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x0C\xEAW[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x80;\x15a\x077W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xC6:\tD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\x0E\x04=\xA6\x17%\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x07,Wa\x0C\xD5W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x0C\xC0W[PP`\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90c\xF5\x08\xE1\x9D`\xE0\x1B\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\n\0W\x83\x92a\x0C\x8CW[P\x81`\x04` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x92\x83\x80\x92\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x0B?W\x85\x90a\x0CXW[a\x03\xCA\x91PaD\xB5V[`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a\x0C&W[P` T\x90_\x19\x82\x01\x91\x82\x11a\x0B\xF9W` \x92a\x04*`\x04\x95\x93a\x04/\x93aE5V[aD\xB5V[`@Q\x92\x83\x80\x92\x7FZ\xDF\0!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\n\0Wa\x04r\x91\x84\x91a\x0B\xDAW[PaE\xABV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW\x81`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x81\x1F\xCB\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x0B\xC5W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x0B\xB0W[P[`/\x81\x11\x15a\x08\"WP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x07\xB8W\x84\x91a\x07\xF0W[P` T\x90`/\x82\x01\x80\x92\x11a\x07\xC3W\x90a\x05\xC2\x91aE5V[`@Qc\xF5\x08\xE1\x9D`\xE0\x1B\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x07\xB8W\x84\x91a\x07~W[P\x91a\x05\xF6` \x92`\x04\x94aE5V[`@Q\x92\x83\x80\x92\x7F\xA4\xD7\xE3\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x07,Wa\x069\x91\x83\x91a\x07OW[PaF\x9BV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FEU\x89,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x07:W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x077W\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x07\x1BWP\xF3[\x81a\x07%\x91a9vV[a\x01ZW\x80\xF3[`@Q=\x84\x82>=\x90\xFD[P\xFD[\x81a\x07D\x91a9vV[a\x01ZW\x80_a\x06\xC9V[a\x07q\x91P` =` \x11a\x07wW[a\x07i\x81\x83a9vV[\x81\x01\x90a;\xEFV[_a\x063V[P=a\x07_V[\x92\x90P` \x83=` \x11a\x07\xB0W[\x81a\x07\x9A` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x91Qa\x05\xF6a\x05\xE6V[_\x80\xFD[=\x91Pa\x07\x8DV[`@Q=\x86\x82>=\x90\xFD[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11a\x08\x1AW[\x81a\x08\x0B` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ_a\x05\xA8V[=\x91Pa\x07\xFEV[\x90\x82`\x01`\x01`\xA0\x1B\x03`%T\x16` a\x08=\x81T\x86a;\xE2V[`$`@Q\x80\x94\x81\x93\x7F\x01u\xE2;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x07,W\x82\x91a\x0B|W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x0BgW[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x0BJW[PP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Qc\xF5\x08\xE1\x9D`\xE0\x1B\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a\x0B\x0CW[Pa\t\x84\x91a\t\x7F\x91a;\xD5V[a?$V[a\t\x8D\x83a=0V[\x90\x81\x81\x03a\n\x0BW[PP` `\x04\x91`@Q\x92\x83\x80\x92c\xF5\x08\xE1\x9D`\xE0\x1B\x82RZ\xFA\x80\x15a\n\0W\x83\x90a\t\xCDW[a\t\xC8\x91P\x91a<\x07V[a\x05TV[P` \x81=\x82\x11a\t\xF8W[\x81a\t\xE6` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa\t\xC8\x90Qa\t\xBDV[=\x91Pa\t\xD9V[`@Q=\x85\x82>=\x90\xFD[a\nN\x81\x83\x86`@Qa\n\x1F`@\x82a9vV[` \x81R\x7FEpoch %s: Expected %s, Actual %s` \x82\x01RaF\x1DV[`!Ta\n[\x81\x84a;\xE2V[\x82\x11\x92\x83\x15a\n\xF8W[PPPa\nsW_\x80a\t\x96V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FMint amount is not within accept`D\x82\x01R\x7Fed diff\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[a\x0B\x03\x92\x93Pa;\xD5V[\x11_\x80\x80a\neV[\x90P` \x81=\x82\x11a\x0B7W[\x81a\x0B&` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQa\t\x84a\tqV[=\x91Pa\x0B\x19V[`@Q=\x87\x82>=\x90\xFD[\x81a\x0BT\x91a9vV[a\x0B_W\x82_a\t>V[\x82\x80\xFD[P\x80\xFD[\x81a\x0Bq\x91a9vV[a\x0B_W\x82_a\x08\xEEV[\x91PP` \x81=\x82\x11a\x0B\xA8W[\x81a\x0B\x97` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x83\x90Q_a\x08~V[=\x91Pa\x0B\x8AV[\x81a\x0B\xBA\x91a9vV[a\x0BcW\x81_a\x05RV[\x81a\x0B\xCF\x91a9vV[a\x0BcW\x81_a\x05\x02V[a\x0B\xF3\x91P` =` \x11a\x07wWa\x07i\x81\x83a9vV[_a\x04lV[`$\x86\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11a\x0CPW[\x81a\x0CA` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ_a\x04\x07V[=\x91Pa\x0C4V[P` \x81=` \x11a\x0C\x84W[\x81a\x0Cr` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa\x03\xCA\x90Qa\x03\xC0V[=\x91Pa\x0CeV[\x90\x91P` \x81=` \x11a\x0C\xB8W[\x81a\x0C\xA8` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ\x90_a\x03uV[=\x91Pa\x0C\x9BV[\x81a\x0C\xCA\x91a9vV[a\x01ZW\x80_a\x03BV[\x81a\x0C\xDF\x91a9vV[a\x01ZW\x80_a\x02\xD6V[\x81a\x0C\xF4\x91a9vV[a\x01ZW\x80_a\x02xV[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW` a\r\x1AaC\xDCV[`@Q\x90\x15\x15\x81R\xF3[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`\x19Ta\rA\x81a<\x15V[\x91a\rO`@Q\x93\x84a9vV[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\r\x99W`@Q` \x80\x82R\x81\x90a\x01\xC8\x90\x82\x01\x88a7\xCCV[`\x01` \x81\x92a\r\xA8\x85a<-V[\x81R\x01\x92\x01\x92\x01\x91\x90a\r|V[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`\x1CTa\r\xD3\x81a<\x15V[\x91a\r\xE1`@Q\x93\x84a9vV[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\x0E#W`@Q\x80a\x01\xC8\x87\x82a8yV[`\x02` `\x01\x92`@Qa\x0E6\x81a9\x10V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x0EN\x85\x87\x01aA8V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0E\x0EV[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW` \x80T`@Q\x90\x81R\xF3[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`\x1DTa\x0E\x9A\x81a<\x15V[\x91a\x0E\xA8`@Q\x93\x84a9vV[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x0E\xEAW`@Q\x80a\x01\xC8\x87\x82a8yV[`\x02` `\x01\x92`@Qa\x0E\xFD\x81a9\x10V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x0F\x15\x85\x87\x01aA8V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0E\xD5V[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`\x1ATa\x0FD\x81a<\x15V[\x91a\x0FR`@Q\x93\x84a9vV[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\x0F\x9CW`@Q` \x80\x82R\x81\x90a\x01\xC8\x90\x82\x01\x88a7\xCCV[`\x01` \x81\x92a\x0F\xAB\x85a<-V[\x81R\x01\x92\x01\x92\x01\x91\x90a\x0F\x7FV[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x077W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x17\xCFW[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x80;\x15a\x077W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xC6:\tD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\r\xE0\xB6\xB3\xA7d\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x07,Wa\x17\xBAW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x17\xA5W[PP`\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90c\xF5\x08\xE1\x9D`\xE0\x1B\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\n\0W\x83\x92a\x17qW[P\x81`\x04` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x92\x83\x80\x92\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x0B?W\x85\x90a\x17=W[a\x11\x98\x91PaD\xB5V[`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a\x17\x0BW[P` T\x90_\x19\x82\x01\x91\x82\x11a\x0B\xF9W` \x92a\x04*`\x04\x95\x93a\x11\xF8\x93aE5V[`@Q\x92\x83\x80\x92\x7FZ\xDF\0!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\n\0Wa\x12:\x91\x84\x91a\x0B\xDAWPaE\xABV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW\x81`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x81\x1F\xCB\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x16\xF6W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x16\xE1W[P[`/\x81\x11\x15a\x13\x89WP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x07\xB8W\x84\x91a\x07\xF0WP` T\x90`/\x82\x01\x80\x92\x11a\x07\xC3W\x90a\x05\xC2\x91aE5V[\x90\x82`\x01`\x01`\xA0\x1B\x03`%T\x16` a\x13\xA4\x81T\x86a;\xE2V[`$`@Q\x80\x94\x81\x93\x7F\x01u\xE2;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x07,W\x82\x91a\x16\xADW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x16\x98W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x16\x83W[PP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Qc\xF5\x08\xE1\x9D`\xE0\x1B\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a\x16PW[Pa\x14\xE6\x91a\t\x7F\x91a;\xD5V[b\x19nk\x81\x03a\x15ZW[P` `\x04\x91`@Q\x92\x83\x80\x92c\xF5\x08\xE1\x9D`\xE0\x1B\x82RZ\xFA\x80\x15a\n\0W\x83\x90a\x15'W[a\x15\"\x91P\x91a<\x07V[a\x13\x1CV[P` \x81=\x82\x11a\x15RW[\x81a\x15@` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa\x15\"\x90Qa\x15\x17V[=\x91Pa\x153V[_\x80a\x15\xD7a\x15\xFA`@Qa\x15p`@\x82a9vV[` \x81R\x7FEpoch %s: Expected %s, Actual %s` \x82\x01R`@Q\x92\x83\x91\x7F\xA7\xA8xS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R`\x80`$\x84\x01R`\xA4\x83\x01\x90a7\xA7V[\x88`D\x83\x01Rb\x19nk`d\x83\x01R\x86`\x84\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a9vV[` \x81Q\x91\x01jconsole.logZ\xFAP`!T\x90\x81b\x19nk\x01\x80b\x19nk\x11a\x0B\xF9W\x81\x11\x91\x82\x15a\x169W[PPa\nsW_a\x14\xF1V[b\x19nk\x90\x81\x03\x92P\x82\x11a\x07\xC3W\x10_\x80a\x16-V[\x90P` \x81=\x82\x11a\x16{W[\x81a\x16j` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQa\x14\xE6a\x14\xD8V[=\x91Pa\x16]V[\x81a\x16\x8D\x91a9vV[a\x0B_W\x82_a\x14\xA5V[\x81a\x16\xA2\x91a9vV[a\x0B_W\x82_a\x14UV[\x91PP` \x81=\x82\x11a\x16\xD9W[\x81a\x16\xC8` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x83\x90Q_a\x13\xE5V[=\x91Pa\x16\xBBV[\x81a\x16\xEB\x91a9vV[a\x0BcW\x81_a\x13\x1AV[\x81a\x17\0\x91a9vV[a\x0BcW\x81_a\x12\xCAV[\x90P` \x81=` \x11a\x175W[\x81a\x17&` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ_a\x11\xD5V[=\x91Pa\x17\x19V[P` \x81=` \x11a\x17iW[\x81a\x17W` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa\x11\x98\x90Qa\x11\x8EV[=\x91Pa\x17JV[\x90\x91P` \x81=` \x11a\x17\x9DW[\x81a\x17\x8D` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ\x90_a\x11CV[=\x91Pa\x17\x80V[\x81a\x17\xAF\x91a9vV[a\x01ZW\x80_a\x11\x10V[\x81a\x17\xC4\x91a9vV[a\x01ZW\x80_a\x10\xA4V[\x81a\x17\xD9\x91a9vV[a\x01ZW\x80_a\x10FV[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`\x1BTa\x18\x01\x81a<\x15V[a\x18\x0E`@Q\x91\x82a9vV[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a\x18\xE6W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a\x18{WPPPP\x03\x90\xF3[\x91\x93` a\x18\xD6\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a\x18\xC6\x83Q`@\x84R`@\x84\x01\x90a7\xA7V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra8$V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x18lV[`\x02` `\x01\x92`@Qa\x18\xF9\x81a9\x10V[a\x19\x02\x86a<-V[\x81Ra\x19\x0F\x85\x87\x01aA8V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x18>V[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x077W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa \x96W[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x80;\x15a\x077W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xC6:\tD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\r\x99\xA8\xCE\xC7\xE2\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x07,Wa \x81W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa lW[PP`\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90c\xF5\x08\xE1\x9D`\xE0\x1B\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\n\0W\x83\x92a 8W[P\x81`\x04` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x92\x83\x80\x92\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x0B?W\x85\x90a \x04W[a\x1B\0\x91PaD\xB5V[`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a\x1F\xD2W[P` T\x90_\x19\x82\x01\x91\x82\x11a\x0B\xF9W` \x92a\x04*`\x04\x95\x93a\x1B`\x93aE5V[`@Q\x92\x83\x80\x92\x7FZ\xDF\0!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\n\0Wa\x1B\xA2\x91\x84\x91a\x0B\xDAWPaE\xABV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW\x81`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x81\x1F\xCB\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x1F\xBDW[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x1F\xA8W[P[`/\x81\x11\x15a\x1C\xF1WP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x07\xB8W\x84\x91a\x07\xF0WP` T\x90`/\x82\x01\x80\x92\x11a\x07\xC3W\x90a\x05\xC2\x91aE5V[\x90\x82`\x01`\x01`\xA0\x1B\x03`%T\x16` a\x1D\x0C\x81T\x86a;\xE2V[`$`@Q\x80\x94\x81\x93\x7F\x01u\xE2;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x07,W\x82\x91a\x1FtW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x1F_W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x1FJW[PP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Qc\xF5\x08\xE1\x9D`\xE0\x1B\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a\x1F\x17W[Pa\x1EN\x91a\t\x7F\x91a;\xD5V[a\x1EW\x83a9\xD7V[\x90\x81\x81\x03a\x1E\xCAW[PP` `\x04\x91`@Q\x92\x83\x80\x92c\xF5\x08\xE1\x9D`\xE0\x1B\x82RZ\xFA\x80\x15a\n\0W\x83\x90a\x1E\x97W[a\x1E\x92\x91P\x91a<\x07V[a\x1C\x84V[P` \x81=\x82\x11a\x1E\xC2W[\x81a\x1E\xB0` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa\x1E\x92\x90Qa\x1E\x87V[=\x91Pa\x1E\xA3V[a\x1E\xDE\x81\x83\x86`@Qa\n\x1F`@\x82a9vV[`!Ta\x1E\xEB\x81\x84a;\xE2V[\x82\x11\x92\x83\x15a\x1F\x03W[PPPa\nsW_\x80a\x1E`V[a\x1F\x0E\x92\x93Pa;\xD5V[\x11_\x80\x80a\x1E\xF5V[\x90P` \x81=\x82\x11a\x1FBW[\x81a\x1F1` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQa\x1ENa\x1E@V[=\x91Pa\x1F$V[\x81a\x1FT\x91a9vV[a\x0B_W\x82_a\x1E\rV[\x81a\x1Fi\x91a9vV[a\x0B_W\x82_a\x1D\xBDV[\x91PP` \x81=\x82\x11a\x1F\xA0W[\x81a\x1F\x8F` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x83\x90Q_a\x1DMV[=\x91Pa\x1F\x82V[\x81a\x1F\xB2\x91a9vV[a\x0BcW\x81_a\x1C\x82V[\x81a\x1F\xC7\x91a9vV[a\x0BcW\x81_a\x1C2V[\x90P` \x81=` \x11a\x1F\xFCW[\x81a\x1F\xED` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ_a\x1B=V[=\x91Pa\x1F\xE0V[P` \x81=` \x11a 0W[\x81a \x1E` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa\x1B\0\x90Qa\x1A\xF6V[=\x91Pa \x11V[\x90\x91P` \x81=` \x11a dW[\x81a T` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ\x90_a\x1A\xABV[=\x91Pa GV[\x81a v\x91a9vV[a\x01ZW\x80_a\x1AxV[\x81a \x8B\x91a9vV[a\x01ZW\x80_a\x1A\x0CV[\x81a \xA0\x91a9vV[a\x01ZW\x80_a\x19\xAEV[P4a\x01ZW` `\x03\x196\x01\x12a\x01ZW` a \xCA`\x045a?DV[`@Q\x90\x81R\xF3[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a!1Wa\x01\xC8\x85a\x01\xBC\x81\x87\x03\x82a9vV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a!\x1AV[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a!\xAFWa\x01\xC8\x85a\x01\xBC\x81\x87\x03\x82a9vV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a!\x98V[P4a\x01ZW` `\x03\x196\x01\x12a\x01ZW` a \xCA`\x045a?$V[P4a\x01ZW` `\x03\x196\x01\x12a\x01ZW` a \xCA`\x045a=0V[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`\x1ETa\")\x81a<\x15V[a\"6`@Q\x91\x82a9vV[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a#\x08W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a\"\xA3WPPPP\x03\x90\xF3[\x91\x93` a\"\xF8\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R`@\x83\x8AQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a7\xCCV[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\"\x94V[`@Qa#\x14\x81a9\x10V[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta#0\x81a<\x15V[\x91a#>`@Q\x93\x84a9vV[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a#tWPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\"fV[`\x01` \x81\x92a#\x83\x86a<-V[\x81R\x01\x93\x01\x91\x01\x90\x91a#NV[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x90\x81R\xF3[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW` `\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90\x81R\xF3[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a$>Wa\x01\xC8\x85a\x01\xBC\x81\x87\x03\x82a9vV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a$'V[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x077W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa-\xFAW[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x80;\x15a\x077W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xC6:\tD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\r\xE0\xB6\xB3\xA7d\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x07,Wa-\xE5W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa-\xD0W[PP`\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90c\xF5\x08\xE1\x9D`\xE0\x1B\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\n\0W\x83\x92a-\x9CW[P\x81`\x04` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x92\x83\x80\x92\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x0B?W\x85\x90a-hW[a&<\x91PaD\xB5V[`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a-6W[P` T\x90_\x19\x82\x01\x91\x82\x11a\x0B\xF9W` \x92a\x04*`\x04\x95\x93a&\x9C\x93aE5V[`@Q\x92\x83\x80\x92\x7FZ\xDF\0!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\n\0Wa&\xDE\x91\x84\x91a\x0B\xDAWPaE\xABV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW\x81`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x81\x1F\xCB\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa-!W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa-\x0CW[P[`/\x81\x11\x15a(-WP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x07\xB8W\x84\x91a\x07\xF0WP` T\x90`/\x82\x01\x80\x92\x11a\x07\xC3W\x90a\x05\xC2\x91aE5V[\x90`\x04\x82\x14a,\x01W[`\x15\x82\x14a*\xF6W[\x82`\x01`\x01`\xA0\x1B\x03`%T\x16` a(Z\x81T\x86a;\xE2V[`$`@Q\x80\x94\x81\x93\x7F\x01u\xE2;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x07,W\x82\x91a*\xC2W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa*\xADW[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa*\x98W[PP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Qc\xF5\x08\xE1\x9D`\xE0\x1B\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a*eW[Pa)\x9C\x91a\t\x7F\x91a;\xD5V[a)\xA5\x83a?DV[\x90\x81\x81\x03a*\x18W[PP` `\x04\x91`@Q\x92\x83\x80\x92c\xF5\x08\xE1\x9D`\xE0\x1B\x82RZ\xFA\x80\x15a\n\0W\x83\x90a)\xE5W[a)\xE0\x91P\x91a<\x07V[a'\xC0V[P` \x81=\x82\x11a*\x10W[\x81a)\xFE` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa)\xE0\x90Qa)\xD5V[=\x91Pa)\xF1V[a*,\x81\x83\x86`@Qa\n\x1F`@\x82a9vV[`!Ta*9\x81\x84a;\xE2V[\x82\x11\x92\x83\x15a*QW[PPPa\nsW_\x80a)\xAEV[a*\\\x92\x93Pa;\xD5V[\x11_\x80\x80a*CV[\x90P` \x81=\x82\x11a*\x90W[\x81a*\x7F` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQa)\x9Ca)\x8EV[=\x91Pa*rV[\x81a*\xA2\x91a9vV[a\x0B_W\x82_a)[V[\x81a*\xB7\x91a9vV[a\x0B_W\x82_a)\x0BV[\x91PP` \x81=\x82\x11a*\xEEW[\x81a*\xDD` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x83\x90Q_a(\x9BV[=\x91Pa*\xD0V[\x82`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa+\xECW[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x80;\x15a\x0BcW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7FCX\x10\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\r\xCE\xF3:o\x83\x80\0`\x04\x84\x01RZ\xF1\x80\x15a\x07,Wa+\xD7W[PPa(@V[\x81a+\xE1\x91a9vV[a\x0B_W\x82_a+\xD0V[\x81a+\xF6\x91a9vV[a\x0B_W\x82_a+rV[\x82`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa,\xF7W[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x80;\x15a\x0BcW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7FCX\x10\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\x0E\x16\x01\x1FO\x05\x80\0`\x04\x84\x01RZ\xF1\x80\x15a\x07,Wa,\xE2W[PPa(7V[\x81a,\xEC\x91a9vV[a\x0B_W\x82_a,\xDBV[\x81a-\x01\x91a9vV[a\x0B_W\x82_a,}V[\x81a-\x16\x91a9vV[a\x0BcW\x81_a'\xBEV[\x81a-+\x91a9vV[a\x0BcW\x81_a'nV[\x90P` \x81=` \x11a-`W[\x81a-Q` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ_a&yV[=\x91Pa-DV[P` \x81=` \x11a-\x94W[\x81a-\x82` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa&<\x90Qa&2V[=\x91Pa-uV[\x90\x91P` \x81=` \x11a-\xC8W[\x81a-\xB8` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ\x90_a%\xE7V[=\x91Pa-\xABV[\x81a-\xDA\x91a9vV[a\x01ZW\x80_a%\xB4V[\x81a-\xEF\x91a9vV[a\x01ZW\x80_a%HV[\x81a.\x04\x91a9vV[a\x01ZW\x80_a$\xEAV[P4a\x07\xACW_`\x03\x196\x01\x12a\x07\xACW`@Q\x7F\x98h\x004\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Fhttps://0xrpc.io/eth\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R` \x81`d\x81_sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a6\x9DWa6\xD5W[P` T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x01\x90\x81\x11a6\xA8Wb'\x8D\0\x81\x02\x90\x80\x82\x04b'\x8D\0\x14\x90\x15\x17\x15a6\xA8Wch\x8DF\xF0\x01\x80ch\x8DF\xF0\x11a6\xA8Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xACW`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a6\x9DWa6\x8AW[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x15\x80\x15a6wW[a/\x84W\x80\xF3[_\x80a0'a05`@Qa/\x9A``\x82a9vV[`5\x81R\x7FEmissions contracts not found, d` \x82\x01R\x7Feploying ones to fork\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R`@Q\x92\x83\x91\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R` `$\x84\x01R`D\x83\x01\x90a7\xA7V[\x03`\x1F\x19\x81\x01\x83R\x82a9vV[` \x81Q\x91\x01jconsole.logZ\xFAP`@Q`\x86\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a6JW\x90\x82\x91aG\x0E\x839\x03\x90\x82\xF0\x80\x15a5\xE3W`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x90`@Q\x91a\x13B\x91\x82\x84\x01\x92\x84\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a6\x1DW\x91``\x93\x91\x85\x93aG\x94\x859\x82R\x80` \x83\x01R`@\x82\x01R\x03\x01\x90\x83\xF0\x80\x15a\x07,W`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$T\x16\x17`$U` T\x90`\x01`\x01`\xA0\x1B\x03`#T\x16\x90`@Q\x93a\x15\x1E\x93\x84\x86\x01\x94\x86\x86\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x11\x17a5\xF0W\x92\x86\x94\x92`\x01`\x01`\xA0\x1B\x03\x92`\xC0\x97\x95aZ\xD6\x889\x85R` \x85\x01R\x16`@\x83\x01R\x80``\x83\x01R\x80`\x80\x83\x01R`\xA0\x82\x01R\x03\x01\x90\x82\xF0\x80\x15a5\xE3W`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`%T\x16\x17`%U\x80`\x04` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92\x83\x80\x92\x7F\x8D3C\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x07,W\x82\x91a5\xAEW[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a56W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\n\0W\x83\x91a5\x99W[PP`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90`\x01`\x01`\xA0\x1B\x03`$T\x16\x82;\x15a5\x1CW`@Q\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x07,Wa5\x84W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa5oW[P`\x04` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x92\x83\x80\x92\x7F\xDE\xBEO\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x07,W\x82\x91a5:W[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a56W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\n\0W\x83\x91a5!W[PP`\x01`\x01`\xA0\x1B\x03`$T\x16\x90`\x01`\x01`\xA0\x1B\x03`%T\x16\x82;\x15a5\x1CW`@Q\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x07,Wa5\x07W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x07\x1BWPP\x80\xF3[\x81a5\x11\x91a9vV[a\x01ZW\x80_a4\x97V[PPP\xFD[\x81a5+\x91a9vV[a\x077W\x81_a4 V[PP\xFD[\x91PP` \x81=` \x11a5gW[\x81a5V` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x81\x90Q_a3\xA1V[=\x91Pa5IV[\x81a5y\x91a9vV[a\x01ZW\x80_a3VV[\x81a5\x8E\x91a9vV[a\x01ZW\x80_a2\xEAV[\x81a5\xA3\x91a9vV[a\x077W\x81_a2sV[\x91PP` \x81=` \x11a5\xDBW[\x81a5\xCA` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x81\x90Q_a1\xF4V[=\x91Pa5\xBDV[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x88\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[`$\x87\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x15a/}V[a6\x96\x91P_\x90a9vV[__a/hV[`@Q=_\x82>=\x90\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[` \x81=` \x11a6\xFCW[\x81a6\xEE` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQa.\xA3V[=\x91Pa6\xE1V[4a\x07\xACW_`\x03\x196\x01\x12a\x07\xACW` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[4a\x07\xACW_`\x03\x196\x01\x12a\x07\xACW` `!T`@Q\x90\x81R\xF3[4a\x07\xACW` `\x03\x196\x01\x12a\x07\xACW` a \xCA`\x045a9\xD7V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a7\x88WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a7{V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a7\xF7WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a8\x15\x83`\x1F\x19\x86`\x01\x96\x03\x01\x87R\x89Qa7\xA7V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a7\xE8V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a8AWPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a84V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a8\xABWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a9\x01\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a8$V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a8\x9CV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a9,W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x06\0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a9,W`@RV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a9,W`@RV[\x90`0\x81\x10\x15a9\xAAW`\x05\x1B\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`@Q\x90a9\xE4\x82a9YV[b'Sk\x82Rb&\x8A\x11` \x83\x01Rb%\xC4\xBF`@\x83\x01Rb%\x03_``\x83\x01Rb$E\xDD`\x80\x83\x01Rb#\x8C&`\xA0\x83\x01Rb\"\xD6%`\xC0\x83\x01Rb\"#\xC8`\xE0\x83\x01Rb!t\xFCa\x01\0\x83\x01Rb \xC9\xB0a\x01 \x83\x01Rb !\xD0a\x01@\x83\x01Rb\x1F}La\x01`\x83\x01Rb\x1E\xDC\x12a\x01\x80\x83\x01Rb\x1E>\x12a\x01\xA0\x83\x01Rb\x1D\xA3:a\x01\xC0\x83\x01Rb\x1D\x0B|a\x01\xE0\x83\x01Rb\x1Cv\xC6a\x02\0\x83\x01Rb\x1B\xE5\na\x02 \x83\x01Rb\x1BV8a\x02@\x83\x01Rb\x1A\xCAAa\x02`\x83\x01Rb\x1AA\x16a\x02\x80\x83\x01Rb\x19\xBA\xABa\x02\xA0\x83\x01Rb\x196\xEFa\x02\xC0\x83\x01Rb\x18\xB5\xD6a\x02\xE0\x83\x01Rb\x187Qa\x03\0\x83\x01Rb\x17\xBBUa\x03 \x83\x01Rb\x17A\xD3a\x03@\x83\x01Rb\x16\xCA\xC0a\x03`\x83\x01Rb\x16V\x0Ea\x03\x80\x83\x01Rb\x15\xE3\xB1a\x03\xA0\x83\x01Rb\x15s\x9Ea\x03\xC0\x83\x01Rb\x15\x05\xC9a\x03\xE0\x83\x01Rb\x14\x9A'a\x04\0\x83\x01Rb\x140\xABa\x04 \x83\x01Rb\x13\xC9Ka\x04@\x83\x01Rb\x13c\xFDa\x04`\x83\x01Rb\x13\0\xB6a\x04\x80\x83\x01Rb\x12\x9Fja\x04\xA0\x83\x01Rb\x12@\x11a\x04\xC0\x83\x01Rb\x11\xE2\xA0a\x04\xE0\x83\x01Rb\x11\x87\x0Ea\x05\0\x83\x01Rb\x11-Pa\x05 \x83\x01Rb\x10\xD5^a\x05@\x83\x01Rb\x10\x7F.a\x05`\x83\x01Rb\x10*\xB7a\x05\x80\x83\x01Rb\x0F\xD7\xF1a\x05\xA0\x83\x01Rb\x0F\x86\xD4a\x05\xC0\x83\x01Rb\x0F7Ua\x05\xE0\x83\x01R`0\x81\x10\x15a;\xCFWa;\xCB\x91a9\x99V[Q\x90V[PP_\x90V[\x91\x90\x82\x03\x91\x82\x11a6\xA8WV[\x91\x90\x82\x01\x80\x92\x11a6\xA8WV[\x90\x81` \x91\x03\x12a\x07\xACWQ\x80\x15\x15\x81\x03a\x07\xACW\x90V[_\x19\x81\x14a6\xA8W`\x01\x01\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a9,W`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15a=&W[` \x85\x10\x84\x14a<\xF9W\x84\x87R\x86\x93\x90\x81\x15a<\xB9WP`\x01\x14a<uW[Pa<s\x92P\x03\x83a9vV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a<\x9DWPP\x90` a<s\x92\x82\x01\x01_a<fV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a<\x84V[` \x93Pa<s\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a<fV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93a<GV[`@Q\x90a==\x82a9YV[b\x13\xF0R\x82Rb\x14#]` \x83\x01Rb\x14V\xEB`@\x83\x01Rb\x14\x8A\xFD``\x83\x01Rb\x14\xBF\x94`\x80\x83\x01Rb\x14\xF4\xB2`\xA0\x83\x01Rb\x15*W`\xC0\x83\x01Rb\x15`\x86`\xE0\x83\x01Rb\x15\x97@a\x01\0\x83\x01Rb\x15\xCE\x86a\x01 \x83\x01Rb\x16\x06Ya\x01@\x83\x01Rb\x16>\xBBa\x01`\x83\x01Rb\x16w\xAEa\x01\x80\x83\x01Rb\x16\xB12a\x01\xA0\x83\x01Rb\x16\xEBIa\x01\xC0\x83\x01Rb\x17%\xF6a\x01\xE0\x83\x01Rb\x17a8a\x02\0\x83\x01Rb\x17\x9D\x12a\x02 \x83\x01Rb\x17\xD9\x86a\x02@\x83\x01Rb\x18\x16\x94a\x02`\x83\x01Rb\x18T>a\x02\x80\x83\x01Rb\x18\x92\x87a\x02\xA0\x83\x01Rb\x18\xD1oa\x02\xC0\x83\x01Rb\x19\x10\xF7a\x02\xE0\x83\x01Rb\x19Q#a\x03\0\x83\x01Rb\x19\x91\xF2a\x03 \x83\x01Rb\x19\xD3ha\x03@\x83\x01Rb\x1A\x15\x85a\x03`\x83\x01Rb\x1AXLa\x03\x80\x83\x01Rb\x1A\x9B\xBDa\x03\xA0\x83\x01Rb\x1A\xDF\xDBa\x03\xC0\x83\x01Rb\x1B$\xA8a\x03\xE0\x83\x01Rb\x1Bj%a\x04\0\x83\x01Rb\x1B\xB0Sa\x04 \x83\x01Rb\x1B\xF75a\x04@\x83\x01Rb\x1C>\xCDa\x04`\x83\x01Rb\x1C\x87\x1Ca\x04\x80\x83\x01Rb\x1C\xD0$a\x04\xA0\x83\x01Rb\x1D\x19\xE6a\x04\xC0\x83\x01Rb\x1Ddfa\x04\xE0\x83\x01Rb\x1D\xAF\xA5a\x05\0\x83\x01Rb\x1D\xFB\xA4a\x05 \x83\x01Rb\x1EHfa\x05@\x83\x01Rb\x1E\x95\xECa\x05`\x83\x01Rb\x1E\xE49a\x05\x80\x83\x01Rb\x1F3Na\x05\xA0\x83\x01Rb\x1F\x83.a\x05\xC0\x83\x01Rb\x1F\xD3\xDAa\x05\xE0\x83\x01R`0\x81\x10\x15a;\xCFWa;\xCB\x91a9\x99V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x01\x80\x91\x11a6\xA8Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[`@Q\x90a?Q\x82a9YV[b\x19nk\x82Rb\x19nk` \x83\x01Rb\x19nk`@\x83\x01Rb\x19nk``\x83\x01Rb\x12#\x99`\x80\x83\x01Rb\x12i@`\xA0\x83\x01Rb\x12\xAF\xF3`\xC0\x83\x01Rb\x12\xF7\xB5`\xE0\x83\x01Rb\x13@\x8Ba\x01\0\x83\x01Rb\x13\x8Aya\x01 \x83\x01Rb\x13\xD5\x83a\x01@\x83\x01Rb\x14!\xACa\x01`\x83\x01Rb\x14n\xFAa\x01\x80\x83\x01Rb\x14\xBDqa\x01\xA0\x83\x01Rb\x15\r\x16a\x01\xC0\x83\x01Rb\x15]\xECa\x01\xE0\x83\x01Rb\x15\xAF\xF8a\x02\0\x83\x01Rb\x16\x03@a\x02 \x83\x01Rb\x16W\xC7a\x02@\x83\x01Rb\x16\xAD\x93a\x02`\x83\x01Rb\x17\x04\xA8a\x02\x80\x83\x01Rb\x1EqLa\x02\xA0\x83\x01Rb\x1EJTa\x02\xC0\x83\x01Rb\x1E#\x8Fa\x02\xE0\x83\x01Rb\x1D\xFC\xFBa\x03\0\x83\x01Rb\x1D\xD6\x98a\x03 \x83\x01Rb\x1D\xB0ga\x03@\x83\x01Rb\x1D\x8Aga\x03`\x83\x01Rb\x1Dd\x97a\x03\x80\x83\x01Rb\x1D>\xF7a\x03\xA0\x83\x01Rb\x1D\x19\x88a\x03\xC0\x83\x01Rb\x1C\xF4Ia\x03\xE0\x83\x01Rb\x1C\xCF9a\x04\0\x83\x01Rb\x1C\xAAYa\x04 \x83\x01Rb\x1C\x85\xA7a\x04@\x83\x01Rb\x1Ca%a\x04`\x83\x01Rb\x1C<\xD2a\x04\x80\x83\x01Rb\x1C\x18\xADa\x04\xA0\x83\x01Rb\x1B\xF4\xB7a\x04\xC0\x83\x01Rb\x1B\xD0\xEEa\x04\xE0\x83\x01Rb\x1B\xADSa\x05\0\x83\x01Rb\x1B\x89\xE6a\x05 \x83\x01Rb\x1Bf\xA7a\x05@\x83\x01Rb\x1BC\x94a\x05`\x83\x01Rb\x1B \xAEa\x05\x80\x83\x01Rb\x1A\xFD\xF5a\x05\xA0\x83\x01Rb\x1A\xDBia\x05\xC0\x83\x01Rb\x1A\xB9\ta\x05\xE0\x83\x01R`0\x81\x10\x15a;\xCFWa;\xCB\x91a9\x99V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10aCOWa<s\x94T\x91\x81\x81\x10aC\x19W[\x81\x81\x10aB\xE3W[\x81\x81\x10aB\xADW[\x81\x81\x10aBwW[\x81\x81\x10aBAW[\x81\x81\x10aB\x0BW[\x81\x81\x10aA\xD6W[\x10aA\xA9W[P\x03\x83a9vV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_aA\xA1V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01aA\x9BV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01aA\x93V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01aA\x8BV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01aA\x83V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01aA{V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01aAsV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01aAkV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91aASV[`\x08T`\xFF\x16\x80\x15aC\xEBW\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a6\x9DW_\x91aD\x83W[P\x15\x15\x90V[\x90P` \x81=` \x11aD\xADW[\x81aD\x9E` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ_aD}V[=\x91PaD\x91V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xACW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a6\x9DWaE+WPV[_a<s\x91a9vV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xACW`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a6\x9DWaE+WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xACW`@Q\x90\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a6\x9DWaE+WPV[aF\x84\x90aFf\x92_\x95\x86\x95`@Q\x95\x86\x94\x7F\xA7\xA8xS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x87\x01R`\x80`$\x87\x01R`\xA4\x86\x01\x90a7\xA7V[\x92`D\x85\x01R`d\x84\x01R`\x84\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a9vV[` \x81Q\x91\x01jconsole.logZ\xFAPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xACW`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a6\x9DWaE+WPV\xFE`\x80\x80`@R4`\x13W`n\x90\x81`\x18\x829\xF3[_\x80\xFD\xFE`\x046\x10\x15`\x0BW_\x80\xFD[_5`\xE0\x1Cc\xEE\xEC\x0E$\x14`\x1DW_\x80\xFD[4`jW`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12`jW`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03`jW\0[_\x80\xFD`\xA04a\0\xD9W`\x1Fa\x13B8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xDDW\x80\x84\x92``\x94`@R\x839\x81\x01\x03\x12a\0\xD9Wa\0G\x81a\0\xF1V[a\0_`@a\0X` \x85\x01a\0\xF1V[\x93\x01a\0\xF1V[\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\0\xCAW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\0\xCAW`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\0\xCAWa\0\xA3\x92a\0\x9D\x91`\x80Ra\x01\x05V[Pa\x01{V[P`@Qa\x10\xD3\x90\x81a\x02\x0F\x829`\x80Q\x81\x81\x81a\x03!\x01R\x81\x81a\x07\xF1\x01Ra\rT\x01R\xF3[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0\xD9WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x13\"_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x01vW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x13\"_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a\x12\xE2_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x13\x02_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x01vW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x13\x02_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\xF2\x7F\x93h\xAA\x1Bv\x97\xE5\x04)\xF87\"\xB8\x8F.]\xB8\x18M^,\xBC\xF0`\xF41\x0B\xBD>}\x90_Q` a\x12\xE2_9_Q\x90_R\x90\x80\xA4`\x01\x90V\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x01\xFF\xC9\xA7\x14a\t\xFAWP\x80c\x15\x8E\xF9>\x14a\t\xD8W\x80c$\x8A\x9C\xA3\x14a\t\xAEW\x80c//\xF1]\x14a\tqW\x80c6V\x8A\xBE\x14a\t\x05W\x80cCX\x10\x10\x14a\x08/W\x80cC\xA3\xF8\xA1\x14a\x08\x15W\x80c[\xDFl\xA1\x14a\x07\xC5W\x80c_\x15\xC3\xC9\x14a\x07\xAAW\x80cvg\x18\x08\x14a\x07\x8DW\x80c\x89\x16$\x86\x14a\x07SW\x80c\x91\xD1HT\x14a\x06\xFDW\x80c\xA0\x88x}\x14a\x06\xBAW\x80c\xA2\x17\xFD\xDF\x14a\x06\xA0W\x80c\xAC\x12\xCE\x07\x14a\x06\x83W\x80c\xB1\x98\xD0(\x14a\x06^W\x80c\xC6:\tD\x14a\x05PW\x80c\xD3\xF5f\xAE\x14a\x02VW\x80c\xD5Gt\x1F\x14a\x02\x0FW\x80c\xDE\xBEO\x1F\x14a\x01\xD4W\x80c\xDF\x02D\xB1\x14a\x01\xB6W\x80c\xE0\xE6\x16\x9C\x14a\x01\x9BW\x80c\xE4\xB7\xFBs\x14a\x01xW\x80c\xEC\xEDU&\x14a\x01UWc\xFA9\x1Cd\x14a\x011W_\x80\xFD[4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `0`\x02T\x10\x15`@Q\x90\x81R\xF3[\x80\xFD[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `@Qg\r\xE0\xB6\xB3\xA7d\0\0\x81R\xF3[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` a\x01\x93a\r,V[`@Q\x90\x81R\xF3[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` a\x01\x93a\x0C\xEFV[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `\x03T`@Q\x90\x81R\xF3[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `@Q\x7F\x07n\xB8\xB8u\xB6\xEA\x83\x9B\x08|L\x0C\x1AFa\xB0\x89\xD3\xB6\xEE,\x1E\xF1\xB9\xCF\xA7\xFE\x10f\xD2\x06\x81R\xF3[P4a\x01RW`@`\x03\x196\x01\x12a\x01RWa\x02R`\x045a\x02/a\n\x98V[\x90a\x02Ma\x02H\x82_R_` R`\x01`@_ \x01T\x90V[a\x0E\xD3V[a\x10\x0BV[P\x80\xF3[P4a\x04QW`@`\x03\x196\x01\x12a\x04QW`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x04QW3_\x90\x81R\x7F\x0E%9\x0F\xF9SSX\xA5\xE9\x16\xDF\xE7\xD3\x82f\xC86\x01\xAFn\x11!\x05\xB2-\xF4\xA9\x0B\xF8\x91\x01` R`@\x90 T`$5\x90`\xFF\x16\x15a\x05\0W`\xFF`\x04T\x16\x15a\x04\x82W`\x02T\x90`0\x82\x10\x15a\x04\x82W\x83\x15a\x04\xD8W\x80\x82\x03a\x04\xAAWPPa\x02\xF3a\x0B\x05V[\x90\x81\x15a\x04\x82W`\x03T\x82\x81\x01\x80\x91\x11a\x04UW`\x03Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x04QW_\x80\x91`D`@Q\x80\x94\x81\x93\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x89`\x04\x84\x01R\x88`$\x84\x01RZ\xF1\x80\x15a\x04FWa\x043W[P`\x02T\x92\x83\x7F\x16\x0F\xC1\x95\xD6\xE56\x91\xD3\r\x80L\xE1\x90\xDC\tG\x18\x91g~CC;\x91\xA7\xA6\x13\x1C\x12\xA5\x9A`@a\x03\xC1a\r,V[\x81Q\x90\x87\x82R` \x82\x01R\xA3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14a\x04\x06WP`\x01` \x92\x01`\x02U`@Q\x90\x81R\xF3[\x80\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x92R`\x11`\x04R\xFD[a\x04?\x91P_\x90a\x0C\x81V[__a\x03\x90V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7F\x9E\x91\xC9\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F0A:\x1A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x07n\xB8\xB8u\xB6\xEA\x83\x9B\x08|L\x0C\x1AFa\xB0\x89\xD3\xB6\xEE,\x1E\xF1\xB9\xCF\xA7\xFE\x10f\xD2\x06`$R`D_\xFD[4a\x04QW` `\x03\x196\x01\x12a\x04QW3_\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\x045\x90`\xFF\x16\x15a\x06.W`\x04T`\xFF\x81\x16a\x04\x82W\x81\x15a\x06\x06W\x7F\xC1,`\xAB\xC2\x16(n\xF2^4\xB1\x80Z\x0C=\xDAs\xE4\xC2\xFDl\xF3`\xE8\x07\xA7\xA9\xE71g9\x91`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0` \x93\x16\x17`\x04U\x80`\x01U`@Q\x90\x81R\xA1\0[\x7F\xEBv\x99 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@QjB,\xA8\xB0\xA0\nBP\0\0\0\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `\x01T`@Q\x90\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Q_\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW`\xA0`\x02T`0`\x03T\x91a\x06\xDDa\r,V[`@Q\x93\x82\x85R\x83` \x86\x01R`@\x85\x01R``\x84\x01R\x10\x15`\x80\x82\x01R\xF3[4a\x04QW`@`\x03\x196\x01\x12a\x04QWa\x07\x16a\n\x98V[`\x045_R_` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Q\x7F\xF2\x7F\x93h\xAA\x1Bv\x97\xE5\x04)\xF87\"\xB8\x8F.]\xB8\x18M^,\xBC\xF0`\xF41\x0B\xBD>}\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `\x02T`@Q\x90\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Q`0\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` a\x01\x93a\x0B\x05V[4a\x04QW` `\x03\x196\x01\x12a\x04QW3_\x90\x81R\x7F~\x972wWE-\x9E3\x15\xE3\x95\xBC\x10\x0E}\xA1\xF2\xD3Q\x06\xFF\xFD8\xF28\x07tp\x90\xEF\xA9` R`@\x90 T`\x045\x90`\xFF\x16\x15a\x08\xB5W\x80\x15a\x06\x06W\x80`\x01U`\x02T`@Q\x91\x82R\x7F\xB8\x13\xFF\xBE8}l\xF6\xE6\xA6\xF6\xC5\xF8\x90_vj\x0F\x1Cl\xD0\x1Cg1/p\x93V\xC6%\x97\xBD` 3\x93\xA3\0[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\xF2\x7F\x93h\xAA\x1Bv\x97\xE5\x04)\xF87\"\xB8\x8F.]\xB8\x18M^,\xBC\xF0`\xF41\x0B\xBD>}`$R`D_\xFD[4a\x04QW`@`\x03\x196\x01\x12a\x04QWa\t\x1Ea\n\x98V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\tIWa\tG\x90`\x045a\x10\x0BV[\0[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04QW`@`\x03\x196\x01\x12a\x04QWa\tG`\x045a\t\x90a\n\x98V[\x90a\t\xA9a\x02H\x82_R_` R`\x01`@_ \x01T\x90V[a\x0F9V[4a\x04QW` `\x03\x196\x01\x12a\x04QW` a\x01\x93`\x045_R_` R`\x01`@_ \x01T\x90V[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `\xFF`\x04T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04QW` `\x03\x196\x01\x12a\x04QW`\x045\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x80\x92\x03a\x04QW\x81\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x93\x14\x90\x81\x15a\nnW[P\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x83a\ngV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x04QWV[\x81\x15a\n\xC5W\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x04UWV[`\xFF`\x04T\x16\x15\x80\x15a\x0CtW[a\x0CpW`\x02T`0\x03`0\x81\x11a\x04UWa\x0B-a\r,V[`\x01\x82\x14a\x0CkW`\x01T\x91g\r\xE0\xB6\xB3\xA7d\0\0\x83\x14a\x0C`WPa\x0BQa\x0C\xEFV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x0C\x19W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2\x1FILX\x9C\0\0\x81\x01\x90\x81\x11a\x04UW[a\x03\xE8\x81\x10\x15a\x0C\x13WPa\x03\xE8\x90[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15a\x0B\xEBW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2\x1FILX\x9C\0\0\x83\x01\x92\x83\x11a\x04UWa\x0B\xE8\x92a\x0B\xE3\x91a\n\xF2V[a\n\xBBV[\x90V[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x03\x91g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11a\x04UWa\x0B\xE8\x92a\x0B\xE3\x91a\n\xF2V[\x90a\x0B\x9CV[g\r\xE0\xB6\xB3\xA7d\0\0\x03g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x0B\x8CW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x90a\x0B\xE8\x92Pa\n\xBBV[\x90P\x90V[_\x90V[P`0`\x02T\x10\x15a\x0B\x13V[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\xC2W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\r\xE0\xB6\xB3\xA7d\0\0`\x02T`\x01T[`0\x82\x10a\r\x0CWPP\x90V[\x90\x91g\r\xE0\xB6\xB3\xA7d\0\0a\r#\x83`\x01\x93a\n\xF2V[\x04\x92\x01\x90a\x0C\xFFV[`@Q\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\x04FW_\x92a\x0E\x9EW[P` `\x04\x91`@Q\x92\x83\x80\x92\x7F\x90-U\xA5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04FW_\x91a\x0ElW[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xBD\xD3WO_\xF5\xBD\xB0\0\0\0\x81\x01\x81\x81\x11a\x04UW\x82\x11\x15a\x0EdW\x81\x03jB,\xA8\xB0\xA0\nBP\0\0\0\x01\x90\x81\x11a\x04UW[\x80jB,\xA8\xB0\xA0\nBP\0\0\0\x11_\x14a\x0E_WjB,\xA8\xB0\xA0\nBP\0\0\0\x03jB,\xA8\xB0\xA0\nBP\0\0\0\x81\x11a\x04UW\x90V[P_\x90V[PP_a\x0E)V[\x90P` \x81=` \x11a\x0E\x96W[\x81a\x0E\x87` \x93\x83a\x0C\x81V[\x81\x01\x03\x12a\x04QWQ_a\r\xE0V[=\x91Pa\x0EzV[\x90\x91P` \x81=` \x11a\x0E\xCBW[\x81a\x0E\xBA` \x93\x83a\x0C\x81V[\x81\x01\x03\x12a\x04QWQ\x90` a\r\xA0V[=\x91Pa\x0E\xADV[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x0F\nWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x10\x05W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x10\x05W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r~\x972wWE-\x9E3\x15\xE3\x95\xBC\x10\x0E}\xA1\xF2\xD3Q\x06\xFF\xFD8\xF28\x07tp\x90\xEF\xA9\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5`\xC04a\x01{W`\x1Fa\x15\x1E8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\x7FW\x80\x84\x92`\xC0\x94`@R\x839\x81\x01\x03\x12a\x01{W\x80Q\x90a\0M` \x82\x01a\x01\x93V[a\0Y`@\x83\x01a\x01\x93V[a\0e``\x84\x01a\x01\x93V[\x92a\0~`\xA0a\0w`\x80\x84\x01a\x01\x93V[\x92\x01a\x01\x93V[`\x01\x80T`\xFF\x19\x16\x81U`\x02U\x92\x85\x15a\x01lW`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15a\x01]W`\x01`\x01`\xA0\x1B\x03\x16\x93\x84\x15a\x01]W`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x01]W`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x01]W`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x01]Wa\x01\x15\x94a\x01\x0F\x93`\x80R`\x01\x80`\xA0\x1B\x03\x19`\x03T\x16\x17`\x03U`\x01\x80`\xA0\x1B\x03\x19`\x04T\x16\x17`\x04Ua\x01\xA7V[Pa\x02\x1DV[P`\xA0R`@Qa\x12\r\x90\x81a\x02\xB1\x829`\x80Q\x81\x81\x81a\x01\x90\x01R\x81\x81a\x08\xEB\x01R\x81\x81a\x0C6\x01Ra\x0E\xC8\x01R`\xA0Q\x81\x81\x81a\x03M\x01R\x81\x81a\x06\xB6\x01Ra\t:\x01R\xF3[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[c\xD5\xB2[c`\xE0\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01{WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x14\xFE_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x02\x18W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x14\xFE_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a\x14\xBE_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x14\xDE_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x02\x18W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x14\xDE_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x90_Q` a\x14\xBE_9_Q\x90_R\x90\x80\xA4`\x01\x90V\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x01u\xE2;\x14a\x0C\xFBWP\x80c\x01\xFF\xC9\xA7\x14a\x0CZW\x80c#\x12\xD7\xD7\x14a\x0C\nW\x80c$\x8A\x9C\xA3\x14a\x0B\xE0W\x80c(N\x133\x14a\x08\x91W\x80c//\xF1]\x14a\x08SW\x80c1\x1FQi\x14a\x07\xE9W\x80c6V\x8A\xBE\x14a\x07~W\x80c?K\xA8:\x14a\x06\xDFW\x80cZ\xDF\0!\x14a\x06\x99W\x80c\\\x97Z\xBB\x14a\x06vW\x80ceH\xE9\xBC\x14a\x05\xE4W\x80co\xD3\xC9\xF0\x14a\x05\xB0W\x80cx\x1C\xD9\x9D\x14a\x05\x91W\x80c\x84\x06\xC0y\x14a\x05]W\x80c\x84V\xCBY\x14a\x04IW\x80c\x91\xD1HT\x14a\x03\xF2W\x80c\xA2\x17\xFD\xDF\x14a\x03\xD6W\x80c\xA4\xD7\xE3\x1D\x14a\x03\xB1W\x80c\xA7\x0B\x9F\x0C\x14a\x03\x93W\x80c\xB9}\xD9\xE2\x14a\x03pW\x80c\xBC\xCF$\xE3\x14a\x035W\x80c\xD5\x17m#\x14a\x02\x92W\x80c\xD5Gt\x1F\x14a\x02KW\x80c\xE6:\xB1\xE9\x14a\x02\x10Wc\xF5\x08\xE1\x9D\x14a\x01<W_\x80\xFD[4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`@Q\x90\x7F\xDF\x02D\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x02\x01W\x90a\x01\xCAW[` \x90`@Q\x90\x81R\xF3[P` \x81=` \x11a\x01\xF9W[\x81a\x01\xE4` \x93\x83a\x0E\x17V[\x81\x01\x03\x12a\x01\xF5W` \x90Qa\x01\xBFV[_\x80\xFD[=\x91Pa\x01\xD7V[`@Q\x90=\x90\x82>=\x90\xFD[\x80\xFD[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `@Q\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x81R\xF3[P4a\x02\rW`@`\x03\x196\x01\x12a\x02\rWa\x02\x8E`\x045a\x02ka\r\xC4V[\x90a\x02\x89a\x02\x84\x82_R_` R`\x01`@_ \x01T\x90V[a\x10\rV[a\x11EV[P\x80\xF3[P4a\x02\rW` `\x03\x196\x01\x12a\x02\rW`\x045b'\x8D\0\x81\x02\x90\x80\x82\x04b'\x8D\0\x14\x90\x15\x17\x15a\x03\x08Wch\x8DF\xF0\x01\x90\x81ch\x8DF\xF0\x11a\x02\xDBW` \x82`@Q\x90\x81R\xF3[\x80\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x92R`\x11`\x04R\xFD[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` a\x03\x8Ba\x0F3V[`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `@Qb'\x8D\0\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` a\x03\xCCa\x0E\x85V[`@Q\x90\x15\x15\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` \x90`@Q\x90\x81R\xF3[P4a\x02\rW`@`\x03\x196\x01\x12a\x02\rWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@a\x04#a\r\xC4V[\x92`\x045\x81R\x80` R \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x81R\x80` R`@\x81 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x05\rWa\x04\xB6a\x0FqV[`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x17`\x01U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\x80\xF3[\x80\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x92R3`\x04R\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*`$R\xFD[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `@Qch\x8DF\xF0\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x04T\x16`@Q\x90\x81R\xF3[P4a\x02\rW` `\x03\x196\x01\x12a\x02\rWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x06\x13a\r\xE7V[a\x06\x1Ba\x0F\xA5V[\x16\x80\x15a\x06NW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x03T\x16\x17`\x03U\x80\xF3[`\x04\x82\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\xFF`\x01T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` a\x06\xB4a\x0F3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x15`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rWa\x06\xF8a\x0F\xA5V[`\x01T`\xFF\x81\x16\x15a\x07VW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\x80\xF3[`\x04\x82\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x02\rW`@`\x03\x196\x01\x12a\x02\rWa\x07\x98a\r\xC4V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\x07\xC1Wa\x02\x8E\x90`\x045a\x11EV[`\x04\x82\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x02\rW` `\x03\x196\x01\x12a\x02\rWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x08\x18a\r\xE7V[a\x08 a\x0F\xA5V[\x16\x80\x15a\x06NW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04T\x16\x17`\x04U\x80\xF3[P4a\x02\rW`@`\x03\x196\x01\x12a\x02\rWa\x02\x8E`\x045a\x08sa\r\xC4V[\x90a\x08\x8Ca\x02\x84\x82_R_` R`\x01`@_ \x01T\x90V[a\x10sV[P4a\x01\xF5W_`\x03\x196\x01\x12a\x01\xF5Wa\x08\xAAa\x0FqV[`\x02\x80T\x14a\x0B\xB8W`\x02\x80Ua\x08\xBFa\x0E\x85V[a\x0B\x90W`@Q\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\n\xF5W_\x92a\x0B\\W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\tc\x81\x84a\x0E\nV[a\tka\x0F3V[\x10a\x0B4W`\x03T`@Q\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91` \x90\x83\x90`D\x90\x82\x90_\x90Z\xF1\x91\x82\x15a\n\xF5W_\x92a\x0B\0W[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03T\x16\x90a\n\x18s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x04T\x16\x91\x85a\x0E\nV[\x82;\x15a\x01\xF5W`@Q\x7F\xEE\xEC\x0E$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R\x90_\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\n\xF5Wa\n\xBAW[P\x7F\xEF\x80\xC2y\xC1x\xDDc\xCD\xAFPa\"K\xE8n\xE3%\xC4\xF4@m\x80-\x04>\xD3;E\xB2\xF6\x8F\x91`@\x91\x82Q\x91\x82R` \x82\x01R\xA1`\x01`\x02U\x80\xF3[`@\x91\x93P\x91a\n\xEB_\x7F\xEF\x80\xC2y\xC1x\xDDc\xCD\xAFPa\"K\xE8n\xE3%\xC4\xF4@m\x80-\x04>\xD3;E\xB2\xF6\x8F\x94a\x0E\x17V[_\x93\x91P\x91a\n\x81V[`@Q=_\x82>=\x90\xFD[\x90\x91P` \x81=` \x11a\x0B,W[\x81a\x0B\x1C` \x93\x83a\x0E\x17V[\x81\x01\x03\x12a\x01\xF5WQ\x90_a\t\xDAV[=\x91Pa\x0B\x0FV[\x7F\x81\x1F\xCB\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90\x91P` \x81=` \x11a\x0B\x88W[\x81a\x0Bx` \x93\x83a\x0E\x17V[\x81\x01\x03\x12a\x01\xF5WQ\x90_a\t7V[=\x91Pa\x0BkV[\x7FEU\x89,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W` a\x03\x8B`\x045_R_` R`\x01`@_ \x01T\x90V[4a\x01\xF5W_`\x03\x196\x01\x12a\x01\xF5W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W`\x045\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x80\x91\x03a\x01\xF5W\x80\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x92\x14\x90\x81\x15a\x0C\xD1W[P`@Q\x90\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x82a\x0C\xC6V[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W`\x045\x80\x15a\r\x9CW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a\roWb'\x8D\0\x81\x02\x90\x80\x82\x04b'\x8D\0\x14\x90\x15\x17\x15a\roWch\x8DF\xF0\x01\x90\x81ch\x8DF\xF0\x11a\roW` \x91\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7F\xD6\x93h\xD4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xF5WV[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xF5WV[\x91\x90\x82\x01\x80\x92\x11a\roWV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0EXW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q\x7F\xFA9\x1Cd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\n\xF5W_\x91a\x0E\xFBWP\x90V[\x90P` \x81=` \x11a\x0F+W[\x81a\x0F\x16` \x93\x83a\x0E\x17V[\x81\x01\x03\x12a\x01\xF5WQ\x80\x15\x15\x81\x03a\x01\xF5W\x90V[=\x91Pa\x0F\tV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97r\xB9\x10B\x01B\x81\x11a\roWb'\x8D\0\x90\x04`\x01\x81\x01\x80\x91\x11a\roW\x90V[`\xFF`\x01T\x16a\x0F}WV[\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[3_\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16\x15a\x0F\xDDWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x10DWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x11?W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x11?W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\xF7\xC9T,Y\x10\x17\xA2\x1Ct\xB6\xF3\xFA\xB6&<yR\xFC\n\xAF\x9D\xB4\xC2**\x04\xDD\xC7\xF8gO\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f5f3560e01c8063037d52d61461374757806306e99e851461372a578063078c665a146137045780630a9254e414612e0f5780631b80a4e51461245d5780631ed7831c146123df5780631f001db4146123b85780632312d7d7146123915780632ade38801461220c5780632ee8823d146121ed57806334d5f37b146121ce5780633e5e3c23146121505780633f7286f4146120d25780634c79cca6146120ab5780635184c5e71461192157806366d9a9a0146117e45780637e041c0414610fb957806385226c8114610f27578063916a17c614610e7d578063a2c8b17714610e60578063b0464fdc14610db6578063b5508aa914610d24578063ba414fa614610cff578063be22cc59146101eb578063e20c9f711461015d5763fa7626d414610138575f80fd5b3461015a578060031936011261015a57602060ff601f54166040519015158152f35b80fd5b503461015a578060031936011261015a5760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106101cc576101c8856101bc81870382613976565b60405191829182613765565b0390f35b82546001600160a01b03168452602090930192600192830192016101a5565b503461015a578060031936011261015a57806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561073757604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57610cea575b506001600160a01b0360245416803b15610737578180916024604051809481937fc63a0944000000000000000000000000000000000000000000000000000000008352670e043da61725000060048401525af1801561072c57610cd5575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57610cc0575b50506001600160a01b03602554166040519063f508e19d60e01b8252602082600481845afa918215610a00578392610c8c575b5081600460206001600160a01b0360245416604051928380927f766718080000000000000000000000000000000000000000000000000000000082525afa8015610b3f578590610c58575b6103ca91506144b5565b6040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa908115610b3f578591610c26575b50602054905f198201918211610bf95760209261042a6004959361042f93614535565b6144b5565b604051928380927f5adf00210000000000000000000000000000000000000000000000000000000082525afa8015610a0057610472918491610bda575b506145ab565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357816040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f811fcbd7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57610bc5575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57610bb0575b505b602f81111561082257506001600160a01b0360255416906040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa9081156107b85784916107f0575b5060205490602f82018092116107c357906105c291614535565b60405163f508e19d60e01b8152602081600481865afa9081156107b857849161077e575b50916105f6602092600494614535565b604051928380927fa4d7e31d0000000000000000000000000000000000000000000000000000000082525afa801561072c5761063991839161074f575b5061469b565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f4555892c000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c5761073a575b506001600160a01b0360255416803b15610737578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c5761071b5750f35b8161072591613976565b61015a5780f35b6040513d84823e3d90fd5b50fd5b8161074491613976565b61015a57805f6106c9565b610771915060203d602011610777575b6107698183613976565b810190613bef565b5f610633565b503d61075f565b9290506020833d6020116107b0575b8161079a60209383613976565b810103126107ac5791516105f66105e6565b5f80fd5b3d915061078d565b6040513d86823e3d90fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d60201161081a575b8161080b60209383613976565b810103126107ac57515f6105a8565b3d91506107fe565b90826001600160a01b0360255416602061083d815486613be2565b6024604051809481937f0175e23b00000000000000000000000000000000000000000000000000000000835260048301525afa90811561072c578291610b7c575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357604051907fe5d6bf020000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57610b67575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57610b4a575b50506001600160a01b03602554169060405163f508e19d60e01b8152602081600481865afa908115610b3f578591610b0c575b506109849161097f91613bd5565b613f24565b61098d83613d30565b90818103610a0b575b505060206004916040519283809263f508e19d60e01b82525afa8015610a005783906109cd575b6109c8915091613c07565b610554565b506020813d82116109f8575b816109e660209383613976565b810103126107ac576109c890516109bd565b3d91506109d9565b6040513d85823e3d90fd5b610a4e818386604051610a1f604082613976565b602081527f45706f63682025733a2045787065637465642025732c2041637475616c202573602082015261461d565b602154610a5b8184613be2565b8211928315610af8575b505050610a73575f80610996565b6040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602760248201527f4d696e7420616d6f756e74206973206e6f742077697468696e2061636365707460448201527f65642064696666000000000000000000000000000000000000000000000000006064820152608490fd5b610b03929350613bd5565b115f8080610a65565b90506020813d8211610b37575b81610b2660209383613976565b810103126107ac5751610984610971565b3d9150610b19565b6040513d87823e3d90fd5b81610b5491613976565b610b5f57825f61093e565b8280fd5b5080fd5b81610b7191613976565b610b5f57825f6108ee565b9150506020813d8211610ba8575b81610b9760209383613976565b810103126107ac578390515f61087e565b3d9150610b8a565b81610bba91613976565b610b6357815f610552565b81610bcf91613976565b610b6357815f610502565b610bf3915060203d602011610777576107698183613976565b5f61046c565b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d602011610c50575b81610c4160209383613976565b810103126107ac57515f610407565b3d9150610c34565b506020813d602011610c84575b81610c7260209383613976565b810103126107ac576103ca90516103c0565b3d9150610c65565b9091506020813d602011610cb8575b81610ca860209383613976565b810103126107ac5751905f610375565b3d9150610c9b565b81610cca91613976565b61015a57805f610342565b81610cdf91613976565b61015a57805f6102d6565b81610cf491613976565b61015a57805f610278565b503461015a578060031936011261015a576020610d1a6143dc565b6040519015158152f35b503461015a578060031936011261015a57601954610d4181613c15565b91610d4f6040519384613976565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b838310610d9957604051602080825281906101c8908201886137cc565b600160208192610da885613c2d565b815201920192019190610d7c565b503461015a578060031936011261015a57601c54610dd381613c15565b91610de16040519384613976565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310610e2357604051806101c88782613879565b60026020600192604051610e3681613910565b6001600160a01b038654168152610e4e858701614138565b83820152815201920192019190610e0e565b503461015a578060031936011261015a5760208054604051908152f35b503461015a578060031936011261015a57601d54610e9a81613c15565b91610ea86040519384613976565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310610eea57604051806101c88782613879565b60026020600192604051610efd81613910565b6001600160a01b038654168152610f15858701614138565b83820152815201920192019190610ed5565b503461015a578060031936011261015a57601a54610f4481613c15565b91610f526040519384613976565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b838310610f9c57604051602080825281906101c8908201886137cc565b600160208192610fab85613c2d565b815201920192019190610f7f565b503461015a578060031936011261015a57806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561073757604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c576117cf575b506001600160a01b0360245416803b15610737578180916024604051809481937fc63a0944000000000000000000000000000000000000000000000000000000008352670de0b6b3a764000060048401525af1801561072c576117ba575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c576117a5575b50506001600160a01b03602554166040519063f508e19d60e01b8252602082600481845afa918215610a00578392611771575b5081600460206001600160a01b0360245416604051928380927f766718080000000000000000000000000000000000000000000000000000000082525afa8015610b3f57859061173d575b61119891506144b5565b6040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa908115610b3f57859161170b575b50602054905f198201918211610bf95760209261042a600495936111f893614535565b604051928380927f5adf00210000000000000000000000000000000000000000000000000000000082525afa8015610a005761123a918491610bda57506145ab565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357816040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f811fcbd7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c576116f6575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c576116e1575b505b602f81111561138957506001600160a01b0360255416906040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa9081156107b85784916107f0575060205490602f82018092116107c357906105c291614535565b90826001600160a01b036025541660206113a4815486613be2565b6024604051809481937f0175e23b00000000000000000000000000000000000000000000000000000000835260048301525afa90811561072c5782916116ad575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357604051907fe5d6bf020000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57611698575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57611683575b50506001600160a01b03602554169060405163f508e19d60e01b8152602081600481865afa908115610b3f578591611650575b506114e69161097f91613bd5565b62196e6b810361155a575b5060206004916040519283809263f508e19d60e01b82525afa8015610a00578390611527575b611522915091613c07565b61131c565b506020813d8211611552575b8161154060209383613976565b810103126107ac576115229051611517565b3d9150611533565b5f806115d76115fa604051611570604082613976565b602081527f45706f63682025733a2045787065637465642025732c2041637475616c20257360208201526040519283917fa7a878530000000000000000000000000000000000000000000000000000000060208401526080602484015260a48301906137a7565b88604483015262196e6b606483015286608483015203601f198101835282613976565b6020815191016a636f6e736f6c652e6c6f675afa50602154908162196e6b018062196e6b11610bf9578111918215611639575b5050610a73575f6114f1565b62196e6b908103925082116107c357105f8061162d565b90506020813d821161167b575b8161166a60209383613976565b810103126107ac57516114e66114d8565b3d915061165d565b8161168d91613976565b610b5f57825f6114a5565b816116a291613976565b610b5f57825f611455565b9150506020813d82116116d9575b816116c860209383613976565b810103126107ac578390515f6113e5565b3d91506116bb565b816116eb91613976565b610b6357815f61131a565b8161170091613976565b610b6357815f6112ca565b90506020813d602011611735575b8161172660209383613976565b810103126107ac57515f6111d5565b3d9150611719565b506020813d602011611769575b8161175760209383613976565b810103126107ac57611198905161118e565b3d915061174a565b9091506020813d60201161179d575b8161178d60209383613976565b810103126107ac5751905f611143565b3d9150611780565b816117af91613976565b61015a57805f611110565b816117c491613976565b61015a57805f6110a4565b816117d991613976565b61015a57805f611046565b503461015a578060031936011261015a57601b5461180181613c15565b61180e6040519182613976565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b8383106118e657868587604051928392602084019060208552518091526040840160408260051b8601019392905b82821061187b57505050500390f35b919360206118d6827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc06001959799849503018652885190836118c683516040845260408401906137a7565b9201519084818403910152613824565b960192019201859493919261186c565b600260206001926040516118f981613910565b61190286613c2d565b815261190f858701614138565b8382015281520192019201919061183e565b503461015a578060031936011261015a57806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561073757604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612096575b506001600160a01b0360245416803b15610737578180916024604051809481937fc63a0944000000000000000000000000000000000000000000000000000000008352670d99a8cec7e2000060048401525af1801561072c57612081575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c5761206c575b50506001600160a01b03602554166040519063f508e19d60e01b8252602082600481845afa918215610a00578392612038575b5081600460206001600160a01b0360245416604051928380927f766718080000000000000000000000000000000000000000000000000000000082525afa8015610b3f578590612004575b611b0091506144b5565b6040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa908115610b3f578591611fd2575b50602054905f198201918211610bf95760209261042a60049593611b6093614535565b604051928380927f5adf00210000000000000000000000000000000000000000000000000000000082525afa8015610a0057611ba2918491610bda57506145ab565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357816040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f811fcbd7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57611fbd575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57611fa8575b505b602f811115611cf157506001600160a01b0360255416906040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa9081156107b85784916107f0575060205490602f82018092116107c357906105c291614535565b90826001600160a01b03602554166020611d0c815486613be2565b6024604051809481937f0175e23b00000000000000000000000000000000000000000000000000000000835260048301525afa90811561072c578291611f74575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357604051907fe5d6bf020000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57611f5f575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57611f4a575b50506001600160a01b03602554169060405163f508e19d60e01b8152602081600481865afa908115610b3f578591611f17575b50611e4e9161097f91613bd5565b611e57836139d7565b90818103611eca575b505060206004916040519283809263f508e19d60e01b82525afa8015610a00578390611e97575b611e92915091613c07565b611c84565b506020813d8211611ec2575b81611eb060209383613976565b810103126107ac57611e929051611e87565b3d9150611ea3565b611ede818386604051610a1f604082613976565b602154611eeb8184613be2565b8211928315611f03575b505050610a73575f80611e60565b611f0e929350613bd5565b115f8080611ef5565b90506020813d8211611f42575b81611f3160209383613976565b810103126107ac5751611e4e611e40565b3d9150611f24565b81611f5491613976565b610b5f57825f611e0d565b81611f6991613976565b610b5f57825f611dbd565b9150506020813d8211611fa0575b81611f8f60209383613976565b810103126107ac578390515f611d4d565b3d9150611f82565b81611fb291613976565b610b6357815f611c82565b81611fc791613976565b610b6357815f611c32565b90506020813d602011611ffc575b81611fed60209383613976565b810103126107ac57515f611b3d565b3d9150611fe0565b506020813d602011612030575b8161201e60209383613976565b810103126107ac57611b009051611af6565b3d9150612011565b9091506020813d602011612064575b8161205460209383613976565b810103126107ac5751905f611aab565b3d9150612047565b8161207691613976565b61015a57805f611a78565b8161208b91613976565b61015a57805f611a0c565b816120a091613976565b61015a57805f6119ae565b503461015a57602060031936011261015a5760206120ca600435613f44565b604051908152f35b503461015a578060031936011261015a5760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110612131576101c8856101bc81870382613976565b82546001600160a01b031684526020909301926001928301920161211a565b503461015a578060031936011261015a5760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b8181106121af576101c8856101bc81870382613976565b82546001600160a01b0316845260209093019260019283019201612198565b503461015a57602060031936011261015a5760206120ca600435613f24565b503461015a57602060031936011261015a5760206120ca600435613d30565b503461015a578060031936011261015a57601e5461222981613c15565b6122366040519182613976565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b83831061230857868587604051928392602084019060208552518091526040840160408260051b8601019392905b8282106122a357505050500390f35b919360206122f8827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc060019597998495030186526040838a516001600160a01b038151168452015191818582015201906137cc565b9601920192018594939192612294565b60405161231481613910565b6001600160a01b03835416815260018301805461233081613c15565b9161233e6040519384613976565b8183528a526020808b20908b9084015b838210612374575050505060019282602092836002950152815201920192019190612266565b60016020819261238386613c2d565b81520193019101909161234e565b503461015a578060031936011261015a5760206001600160a01b0360245416604051908152f35b503461015a578060031936011261015a5760206001600160a01b0360255416604051908152f35b503461015a578060031936011261015a5760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b81811061243e576101c8856101bc81870382613976565b82546001600160a01b0316845260209093019260019283019201612427565b503461015a578060031936011261015a57806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561073757604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612dfa575b506001600160a01b0360245416803b15610737578180916024604051809481937fc63a0944000000000000000000000000000000000000000000000000000000008352670de0b6b3a764000060048401525af1801561072c57612de5575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612dd0575b50506001600160a01b03602554166040519063f508e19d60e01b8252602082600481845afa918215610a00578392612d9c575b5081600460206001600160a01b0360245416604051928380927f766718080000000000000000000000000000000000000000000000000000000082525afa8015610b3f578590612d68575b61263c91506144b5565b6040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa908115610b3f578591612d36575b50602054905f198201918211610bf95760209261042a6004959361269c93614535565b604051928380927f5adf00210000000000000000000000000000000000000000000000000000000082525afa8015610a00576126de918491610bda57506145ab565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357816040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f811fcbd7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612d21575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57612d0c575b505b602f81111561282d57506001600160a01b0360255416906040517fb97dd9e2000000000000000000000000000000000000000000000000000000008152602081600481865afa9081156107b85784916107f0575060205490602f82018092116107c357906105c291614535565b9060048214612c01575b60158214612af6575b826001600160a01b0360255416602061285a815486613be2565b6024604051809481937f0175e23b00000000000000000000000000000000000000000000000000000000835260048301525afa90811561072c578291612ac2575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357604051907fe5d6bf020000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612aad575b506001600160a01b0360255416803b15610b63578180916004604051809481937f284e13330000000000000000000000000000000000000000000000000000000083525af1801561072c57612a98575b50506001600160a01b03602554169060405163f508e19d60e01b8152602081600481865afa908115610b3f578591612a65575b5061299c9161097f91613bd5565b6129a583613f44565b90818103612a18575b505060206004916040519283809263f508e19d60e01b82525afa8015610a005783906129e5575b6129e0915091613c07565b6127c0565b506020813d8211612a10575b816129fe60209383613976565b810103126107ac576129e090516129d5565b3d91506129f1565b612a2c818386604051610a1f604082613976565b602154612a398184613be2565b8211928315612a51575b505050610a73575f806129ae565b612a5c929350613bd5565b115f8080612a43565b90506020813d8211612a90575b81612a7f60209383613976565b810103126107ac575161299c61298e565b3d9150612a72565b81612aa291613976565b610b5f57825f61295b565b81612ab791613976565b610b5f57825f61290b565b9150506020813d8211612aee575b81612add60209383613976565b810103126107ac578390515f61289b565b3d9150612ad0565b826001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612bec575b506001600160a01b0360245416803b15610b63578180916024604051809481937f43581010000000000000000000000000000000000000000000000000000000008352670dcef33a6f83800060048401525af1801561072c57612bd7575b5050612840565b81612be191613976565b610b5f57825f612bd0565b81612bf691613976565b610b5f57825f612b72565b826001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610b6357604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c57612cf7575b506001600160a01b0360245416803b15610b63578180916024604051809481937f43581010000000000000000000000000000000000000000000000000000000008352670e16011f4f05800060048401525af1801561072c57612ce2575b5050612837565b81612cec91613976565b610b5f57825f612cdb565b81612d0191613976565b610b5f57825f612c7d565b81612d1691613976565b610b6357815f6127be565b81612d2b91613976565b610b6357815f61276e565b90506020813d602011612d60575b81612d5160209383613976565b810103126107ac57515f612679565b3d9150612d44565b506020813d602011612d94575b81612d8260209383613976565b810103126107ac5761263c9051612632565b3d9150612d75565b9091506020813d602011612dc8575b81612db860209383613976565b810103126107ac5751905f6125e7565b3d9150612dab565b81612dda91613976565b61015a57805f6125b4565b81612def91613976565b61015a57805f612548565b81612e0491613976565b61015a57805f6124ea565b50346107ac575f6003193601126107ac576040517f9868003400000000000000000000000000000000000000000000000000000000815260206004820152601460248201527f68747470733a2f2f30787270632e696f2f65746800000000000000000000000060448201526020816064815f737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561369d576136d5575b506020547ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81019081116136a85762278d0081029080820462278d0014901517156136a85763688d46f0018063688d46f0116136a857737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ac57604051907fe5d6bf0200000000000000000000000000000000000000000000000000000000825260048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561369d5761368a575b506001600160a01b0360245416158015613677575b612f845780f35b5f80613027613035604051612f9a606082613976565b603581527f456d697373696f6e7320636f6e747261637473206e6f7420666f756e642c206460208201527f65706c6f79696e67206f6e657320746f20666f726b000000000000000000000060408201526040519283917f41304fac0000000000000000000000000000000000000000000000000000000060208401526020602484015260448301906137a7565b03601f198101835282613976565b6020815191016a636f6e736f6c652e6c6f675afa5060405160868082019082821067ffffffffffffffff83111761364a5790829161470e8339039082f080156135e3576001600160a01b03602254166001600160a01b03602354169060405191611342918284019284841067ffffffffffffffff85111761361d579160609391859361479485398252806020830152604082015203019083f0801561072c576001600160a01b031690817fffffffffffffffffffffffff00000000000000000000000000000000000000006024541617602455602054906001600160a01b0360235416906040519361151e938486019486861067ffffffffffffffff8711176135f057928694926001600160a01b039260c09795615ad688398552602085015216604083015280606083015280608083015260a082015203019082f080156135e3576001600160a01b03167fffffffffffffffffffffffff0000000000000000000000000000000000000000602554161760255580600460206001600160a01b0360225416604051928380927f8d3343d60000000000000000000000000000000000000000000000000000000082525afa90811561072c5782916135ae575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561353657604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610a00578391613599575b50506001600160a01b0360225416906001600160a01b0360245416823b1561351c576040517f2f2ff15d00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529082908290604490829084905af1801561072c57613584575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c5761356f575b50600460206001600160a01b0360245416604051928380927fdebe4f1f0000000000000000000000000000000000000000000000000000000082525afa90811561072c57829161353a575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561353657604051907fca669fa70000000000000000000000000000000000000000000000000000000082526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610a00578391613521575b50506001600160a01b0360245416906001600160a01b0360255416823b1561351c576040517f2f2ff15d00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529082908290604490829084905af1801561072c57613507575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015a57806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561072c5761071b57505080f35b8161351191613976565b61015a57805f613497565b505050fd5b8161352b91613976565b61073757815f613420565b5050fd5b9150506020813d602011613567575b8161355660209383613976565b810103126107ac578190515f6133a1565b3d9150613549565b8161357991613976565b61015a57805f613356565b8161358e91613976565b61015a57805f6132ea565b816135a391613976565b61073757815f613273565b9150506020813d6020116135db575b816135ca60209383613976565b810103126107ac578190515f6131f4565b3d91506135bd565b50604051903d90823e3d90fd5b6024887f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b6024877f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b506001600160a01b036025541615612f7d565b61369691505f90613976565b5f5f612f68565b6040513d5f823e3d90fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b6020813d6020116136fc575b816136ee60209383613976565b810103126107ac5751612ea3565b3d91506136e1565b346107ac575f6003193601126107ac5760206001600160a01b0360235416604051908152f35b346107ac575f6003193601126107ac576020602154604051908152f35b346107ac5760206003193601126107ac5760206120ca6004356139d7565b60206040818301928281528451809452019201905f5b8181106137885750505090565b82516001600160a01b031684526020938401939092019160010161377b565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b9080602083519182815201916020808360051b8301019401925f915b8383106137f757505050505090565b909192939460208061381583601f19866001960301875289516137a7565b970193019301919392906137e8565b90602080835192838152019201905f5b8181106138415750505090565b82517fffffffff0000000000000000000000000000000000000000000000000000000016845260209384019390920191600101613834565b602081016020825282518091526040820191602060408360051b8301019401925f915b8383106138ab57505050505090565b9091929394602080613901837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b03815116845201519181858201520190613824565b9701930193019193929061389c565b6040810190811067ffffffffffffffff82111761392c57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610600810190811067ffffffffffffffff82111761392c57604052565b90601f601f19910116810190811067ffffffffffffffff82111761392c57604052565b9060308110156139aa5760051b0190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b604051906139e482613959565b6227536b825262268a1160208301526225c4bf60408301526225035f6060830152622445dd608083015262238c2660a08301526222d62560c0830152622223c860e0830152622174fc6101008301526220c9b0610120830152622021d0610140830152621f7d4c610160830152621edc12610180830152621e3e126101a0830152621da33a6101c0830152621d0b7c6101e0830152621c76c6610200830152621be50a610220830152621b5638610240830152621aca41610260830152621a41166102808301526219baab6102a0830152621936ef6102c08301526218b5d66102e0830152621837516103008301526217bb55610320830152621741d36103408301526216cac06103608301526216560e6103808301526215e3b16103a08301526215739e6103c0830152621505c96103e083015262149a27610400830152621430ab6104208301526213c94b610440830152621363fd610460830152621300b661048083015262129f6a6104a0830152621240116104c08301526211e2a06104e08301526211870e61050083015262112d506105208301526210d55e61054083015262107f2e61056083015262102ab7610580830152620fd7f16105a0830152620f86d46105c0830152620f37556105e08301526030811015613bcf57613bcb91613999565b5190565b50505f90565b919082039182116136a857565b919082018092116136a857565b908160209103126107ac575180151581036107ac5790565b5f1981146136a85760010190565b67ffffffffffffffff811161392c5760051b60200190565b90604051915f8154908160011c9260018316928315613d26575b602085108414613cf9578487528693908115613cb95750600114613c75575b50613c7392500383613976565b565b90505f9291925260205f20905f915b818310613c9d575050906020613c73928201015f613c66565b6020919350806001915483858901015201910190918492613c84565b60209350613c739592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f613c66565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f1693613c47565b60405190613d3d82613959565b6213f05282526214235d6020830152621456eb604083015262148afd60608301526214bf9460808301526214f4b260a083015262152a5760c08301526215608660e0830152621597406101008301526215ce866101208301526216065961014083015262163ebb610160830152621677ae6101808301526216b1326101a08301526216eb496101c0830152621725f66101e08301526217613861020083015262179d126102208301526217d986610240830152621816946102608301526218543e610280830152621892876102a08301526218d16f6102c0830152621910f76102e083015262195123610300830152621991f26103208301526219d368610340830152621a1585610360830152621a584c610380830152621a9bbd6103a0830152621adfdb6103c0830152621b24a86103e0830152621b6a25610400830152621bb053610420830152621bf735610440830152621c3ecd610460830152621c871c610480830152621cd0246104a0830152621d19e66104c0830152621d64666104e0830152621dafa5610500830152621dfba4610520830152621e4866610540830152621e95ec610560830152621ee439610580830152621f334e6105a0830152621f832e6105c0830152621fd3da6105e08301526030811015613bcf57613bcb91613999565b6706f05b59d3b2000081018091116136a857670de0b6b3a7640000900490565b60405190613f5182613959565b62196e6b825262196e6b602083015262196e6b604083015262196e6b60608301526212239960808301526212694060a08301526212aff360c08301526212f7b560e08301526213408b61010083015262138a796101208301526213d583610140830152621421ac61016083015262146efa6101808301526214bd716101a083015262150d166101c083015262155dec6101e08301526215aff861020083015262160340610220830152621657c76102408301526216ad93610260830152621704a8610280830152621e714c6102a0830152621e4a546102c0830152621e238f6102e0830152621dfcfb610300830152621dd698610320830152621db067610340830152621d8a67610360830152621d6497610380830152621d3ef76103a0830152621d19886103c0830152621cf4496103e0830152621ccf39610400830152621caa59610420830152621c85a7610440830152621c6125610460830152621c3cd2610480830152621c18ad6104a0830152621bf4b76104c0830152621bd0ee6104e0830152621bad53610500830152621b89e6610520830152621b66a7610540830152621b4394610560830152621b20ae610580830152621afdf56105a0830152621adb696105c0830152621ab9096105e08301526030811015613bcf57613bcb91613999565b90604051918281549182825260208201905f5260205f20925f905b80600783011061434f57613c73945491818110614319575b8181106142e3575b8181106142ad575b818110614277575b818110614241575b81811061420b575b8181106141d6575b106141a9575b500383613976565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f6141a1565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b16815201930161419b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301614193565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b16815201930161418b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301614183565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b16815201930161417b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b168152019301614173565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b16815201930161416b565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e0820152019401920185929391614153565b60085460ff1680156143eb5790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561369d575f91614483575b50151590565b90506020813d6020116144ad575b8161449e60209383613976565b810103126107ac57515f61447d565b3d9150614491565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ac57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561369d5761452b5750565b5f613c7391613976565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ac57604051917f98296c54000000000000000000000000000000000000000000000000000000008352600483015260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561369d5761452b5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ac57604051907fa5982885000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561369d5761452b5750565b61468490614666925f9586956040519586947fa7a878530000000000000000000000000000000000000000000000000000000060208701526080602487015260a48601906137a7565b9260448501526064840152608483015203601f198101835282613976565b6020815191016a636f6e736f6c652e6c6f675afa50565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ac57604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561369d5761452b575056fe60808060405234601357606e908160188239f35b5f80fdfe6004361015600b575f80fd5b5f3560e01c63eeec0e2414601d575f80fd5b34606a5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112606a5760043573ffffffffffffffffffffffffffffffffffffffff811603606a57005b5f80fd60a0346100d957601f61134238819003918201601f19168301916001600160401b038311848410176100dd578084926060946040528339810103126100d957610047816100f1565b61005f6040610058602085016100f1565b93016100f1565b906001600160a01b031680156100ca576001600160a01b038316156100ca576001600160a01b038216156100ca576100a39261009d91608052610105565b5061017b565b506040516110d3908161020f8239608051818181610321015281816107f10152610d540152f35b63d92e233d60e01b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036100d957565b6001600160a01b0381165f9081525f5160206113225f395f51905f52602052604090205460ff16610176576001600160a01b03165f8181525f5160206113225f395f51905f5260205260408120805460ff191660011790553391905f5160206112e25f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f5160206113025f395f51905f52602052604090205460ff16610176576001600160a01b03165f8181525f5160206113025f395f51905f5260205260408120805460ff191660011790553391907ff27f9368aa1b7697e50429f83722b88f2e5db8184d5e2cbcf060f4310bbd3e7d905f5160206112e25f395f51905f529080a460019056fe6080806040526004361015610012575f80fd5b5f905f3560e01c90816301ffc9a7146109fa57508063158ef93e146109d8578063248a9ca3146109ae5780632f2ff15d1461097157806336568abe14610905578063435810101461082f57806343a3f8a1146108155780635bdf6ca1146107c55780635f15c3c9146107aa578063766718081461078d578063891624861461075357806391d14854146106fd578063a088787d146106ba578063a217fddf146106a0578063ac12ce0714610683578063b198d0281461065e578063c63a094414610550578063d3f566ae14610256578063d547741f1461020f578063debe4f1f146101d4578063df0244b1146101b6578063e0e6169c1461019b578063e4b7fb7314610178578063eced5526146101555763fa391c6414610131575f80fd5b34610152578060031936011261015257602060306002541015604051908152f35b80fd5b50346101525780600319360112610152576020604051670de0b6b3a76400008152f35b50346101525780600319360112610152576020610193610d2c565b604051908152f35b50346101525780600319360112610152576020610193610cef565b50346101525780600319360112610152576020600354604051908152f35b503461015257806003193601126101525760206040517f076eb8b875b6ea839b087c4c0c1a4661b089d3b6ee2c1ef1b9cfa7fe1066d2068152f35b50346101525760406003193601126101525761025260043561022f610a98565b9061024d610248825f525f602052600160405f20015490565b610ed3565b61100b565b5080f35b5034610451576040600319360112610451576004359073ffffffffffffffffffffffffffffffffffffffff821680920361045157335f9081527f0e25390ff9535358a5e916dfe7d38266c83601af6e112105b22df4a90bf8910160205260409020546024359060ff16156105005760ff6004541615610482576002549060308210156104825783156104d8578082036104aa5750506102f3610b05565b908115610482576003548281018091116104555760035573ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b15610451575f80916044604051809481937f40c10f190000000000000000000000000000000000000000000000000000000083528960048401528860248401525af1801561044657610433575b5060025492837f160fc195d6e53691d30d804ce190dc09471891677e43433b91a7a6131c12a59a60406103c1610d2c565b8151908782526020820152a37fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83146104065750600160209201600255604051908152f35b807f4e487b7100000000000000000000000000000000000000000000000000000000602492526011600452fd5b61043f91505f90610c81565b5f5f610390565b6040513d5f823e3d90fd5b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7f9e91c9e7000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f30413a1a000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f076eb8b875b6ea839b087c4c0c1a4661b089d3b6ee2c1ef1b9cfa7fe1066d20660245260445ffd5b3461045157602060031936011261045157335f9081527fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb560205260409020546004359060ff161561062e5760045460ff8116610482578115610606577fc12c60abc216286ef25e34b1805a0c3dda73e4c2fd6cf360e807a7a9e73167399160017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00602093161760045580600155604051908152a1005b7feb769920000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b34610451575f6003193601126104515760206040516a422ca8b0a00a42500000008152f35b34610451575f600319360112610451576020600154604051908152f35b34610451575f6003193601126104515760206040515f8152f35b34610451575f6003193601126104515760a06002546030600354916106dd610d2c565b604051938285528360208601526040850152606084015210156080820152f35b3461045157604060031936011261045157610716610a98565b6004355f525f60205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060ff60405f2054166040519015158152f35b34610451575f6003193601126104515760206040517ff27f9368aa1b7697e50429f83722b88f2e5db8184d5e2cbcf060f4310bbd3e7d8152f35b34610451575f600319360112610451576020600254604051908152f35b34610451575f60031936011261045157602060405160308152f35b34610451575f60031936011261045157602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610451575f600319360112610451576020610193610b05565b3461045157602060031936011261045157335f9081527f7e97327757452d9e3315e395bc100e7da1f2d35106fffd38f23807747090efa960205260409020546004359060ff16156108b557801561060657806001556002546040519182527fb813ffbe387d6cf6e6a6f6c5f8905f766a0f1c6cd01c67312f709356c62597bd60203393a3005b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527ff27f9368aa1b7697e50429f83722b88f2e5db8184d5e2cbcf060f4310bbd3e7d60245260445ffd5b346104515760406003193601126104515761091e610a98565b3373ffffffffffffffffffffffffffffffffffffffff821603610949576109479060043561100b565b005b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461045157604060031936011261045157610947600435610990610a98565b906109a9610248825f525f602052600160405f20015490565b610f39565b346104515760206003193601126104515760206101936004355f525f602052600160405f20015490565b34610451575f60031936011261045157602060ff600454166040519015158152f35b3461045157602060031936011261045157600435907fffffffff00000000000000000000000000000000000000000000000000000000821680920361045157817f7965db0b0000000000000000000000000000000000000000000000000000000060209314908115610a6e575b5015158152f35b7f01ffc9a70000000000000000000000000000000000000000000000000000000091501483610a67565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361045157565b8115610ac5570490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b8181029291811591840414171561045557565b60ff60045416158015610c74575b610c70576002546030036030811161045557610b2d610d2c565b60018214610c6b5760015491670de0b6b3a76400008314610c605750610b51610cef565b670de0b6b3a7640000811115610c19577ffffffffffffffffffffffffffffffffffffffffffffffffff21f494c589c00008101908111610455575b6103e8811015610c1357506103e8905b670de0b6b3a7640000831115610beb577ffffffffffffffffffffffffffffffffffffffffffffffffff21f494c589c0000830192831161045557610be892610be391610af2565b610abb565b90565b91670de0b6b3a76400000391670de0b6b3a7640000831161045557610be892610be391610af2565b90610b9c565b670de0b6b3a764000003670de0b6b3a7640000811115610b8c577f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b90610be89250610abb565b905090565b5f90565b5060306002541015610b13565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610cc257604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b670de0b6b3a76400006002546001545b60308210610d0c57505090565b9091670de0b6b3a7640000610d2383600193610af2565b04920190610cff565b6040517f18160ddd0000000000000000000000000000000000000000000000000000000081527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16602082600481845afa918215610446575f92610e9e575b506020600491604051928380927f902d55a50000000000000000000000000000000000000000000000000000000082525afa908115610446575f91610e6c575b507fffffffffffffffffffffffffffffffffffffffffffbdd3574f5ff5bdb0000000810181811161045557821115610e645781036a422ca8b0a00a425000000001908111610455575b806a422ca8b0a00a4250000000115f14610e5f576a422ca8b0a00a4250000000036a422ca8b0a00a425000000081116104555790565b505f90565b50505f610e29565b90506020813d602011610e96575b81610e8760209383610c81565b8101031261045157515f610de0565b3d9150610e7a565b9091506020813d602011610ecb575b81610eba60209383610c81565b810103126104515751906020610da0565b3d9150610ead565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f20541615610f0a5750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f1461100557805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f1461100557805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d7e97327757452d9e3315e395bc100e7da1f2d35106fffd38f23807747090efa9ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb560c03461017b57601f61151e38819003918201601f19168301916001600160401b0383118484101761017f5780849260c09460405283398101031261017b5780519061004d60208201610193565b61005960408301610193565b61006560608401610193565b9261007e60a061007760808401610193565b9201610193565b6001805460ff1916815560025592851561016c576001600160a01b031691821561015d576001600160a01b031693841561015d576001600160a01b0316801561015d576001600160a01b0382161561015d576001600160a01b0384161561015d576101159461010f9360805260018060a01b0319600354161760035560018060a01b031960045416176004556101a7565b5061021d565b5060a05260405161120d90816102b18239608051818181610190015281816108eb01528181610c360152610ec8015260a05181818161034d015281816106b6015261093a0152f35b63d92e233d60e01b5f5260045ffd5b63d5b25b6360e01b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b038216820361017b57565b6001600160a01b0381165f9081525f5160206114fe5f395f51905f52602052604090205460ff16610218576001600160a01b03165f8181525f5160206114fe5f395f51905f5260205260408120805460ff191660011790553391905f5160206114be5f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f5160206114de5f395f51905f52602052604090205460ff16610218576001600160a01b03165f8181525f5160206114de5f395f51905f5260205260408120805460ff191660011790553391907f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a905f5160206114be5f395f51905f529080a460019056fe6080806040526004361015610012575f80fd5b5f905f3560e01c9081630175e23b14610cfb5750806301ffc9a714610c5a5780632312d7d714610c0a578063248a9ca314610be0578063284e1333146108915780632f2ff15d14610853578063311f5169146107e957806336568abe1461077e5780633f4ba83a146106df5780635adf0021146106995780635c975abb146106765780636548e9bc146105e45780636fd3c9f0146105b0578063781cd99d146105915780638406c0791461055d5780638456cb591461044957806391d14854146103f2578063a217fddf146103d6578063a4d7e31d146103b1578063a70b9f0c14610393578063b97dd9e214610370578063bccf24e314610335578063d5176d2314610292578063d547741f1461024b578063e63ab1e9146102105763f508e19d1461013c575f80fd5b3461020d578060031936011261020d57604051907fdf0244b100000000000000000000000000000000000000000000000000000000825260208260048173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa90811561020157906101ca575b602090604051908152f35b506020813d6020116101f9575b816101e460209383610e17565b810103126101f557602090516101bf565b5f80fd5b3d91506101d7565b604051903d90823e3d90fd5b80fd5b503461020d578060031936011261020d5760206040517f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a8152f35b503461020d57604060031936011261020d5761028e60043561026b610dc4565b90610289610284825f525f602052600160405f20015490565b61100d565b611145565b5080f35b503461020d57602060031936011261020d5760043562278d0081029080820462278d0014901517156103085763688d46f001908163688d46f0116102db57602082604051908152f35b807f4e487b7100000000000000000000000000000000000000000000000000000000602492526011600452fd5b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b503461020d578060031936011261020d5760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b503461020d578060031936011261020d57602061038b610f33565b604051908152f35b503461020d578060031936011261020d57602060405162278d008152f35b503461020d578060031936011261020d5760206103cc610e85565b6040519015158152f35b503461020d578060031936011261020d57602090604051908152f35b503461020d57604060031936011261020d5773ffffffffffffffffffffffffffffffffffffffff6040610423610dc4565b926004358152806020522091165f52602052602060ff60405f2054166040519015158152f35b503461020d578060031936011261020d577f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a8152806020526040812073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f2054161561050d576104b6610f71565b60017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416176001557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a180f35b807fe2517d3f0000000000000000000000000000000000000000000000000000000060449252336004527f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a602452fd5b503461020d578060031936011261020d57602073ffffffffffffffffffffffffffffffffffffffff60035416604051908152f35b503461020d578060031936011261020d57602060405163688d46f08152f35b503461020d578060031936011261020d57602073ffffffffffffffffffffffffffffffffffffffff60045416604051908152f35b503461020d57602060031936011261020d5773ffffffffffffffffffffffffffffffffffffffff610613610de7565b61061b610fa5565b16801561064e577fffffffffffffffffffffffff0000000000000000000000000000000000000000600354161760035580f35b6004827fd92e233d000000000000000000000000000000000000000000000000000000008152fd5b503461020d578060031936011261020d57602060ff600154166040519015158152f35b503461020d578060031936011261020d5760206106b4610f33565b7f00000000000000000000000000000000000000000000000000000000000000001115604051908152f35b503461020d578060031936011261020d576106f8610fa5565b60015460ff811615610756577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6020604051338152a180f35b6004827f8dfc202b000000000000000000000000000000000000000000000000000000008152fd5b503461020d57604060031936011261020d57610798610dc4565b3373ffffffffffffffffffffffffffffffffffffffff8216036107c15761028e90600435611145565b6004827f6697b232000000000000000000000000000000000000000000000000000000008152fd5b503461020d57602060031936011261020d5773ffffffffffffffffffffffffffffffffffffffff610818610de7565b610820610fa5565b16801561064e577fffffffffffffffffffffffff0000000000000000000000000000000000000000600454161760045580f35b503461020d57604060031936011261020d5761028e600435610873610dc4565b9061088c610284825f525f602052600160405f20015490565b611073565b50346101f5575f6003193601126101f5576108aa610f71565b6002805414610bb857600280556108bf610e85565b610b90576040517f766718080000000000000000000000000000000000000000000000000000000081527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16602082600481845afa918215610af5575f92610b5c575b507f00000000000000000000000000000000000000000000000000000000000000006109638184610e0a565b61096b610f33565b10610b34576003546040517fd3f566ae00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024810184905291602090839060449082905f905af1918215610af5575f92610b00575b5073ffffffffffffffffffffffffffffffffffffffff6003541690610a1873ffffffffffffffffffffffffffffffffffffffff600454169185610e0a565b823b156101f5576040517feeec0e2400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9290921660048301526024820152905f908290604490829084905af18015610af557610aba575b507fef80c279c178dd63cdaf5061224be86ee325c4f4406d802d043ed33b45b2f68f9160409182519182526020820152a1600160025580f35b604091935091610aeb5f7fef80c279c178dd63cdaf5061224be86ee325c4f4406d802d043ed33b45b2f68f94610e17565b5f93915091610a81565b6040513d5f823e3d90fd5b9091506020813d602011610b2c575b81610b1c60209383610e17565b810103126101f55751905f6109da565b3d9150610b0f565b7f811fcbd7000000000000000000000000000000000000000000000000000000005f5260045ffd5b9091506020813d602011610b88575b81610b7860209383610e17565b810103126101f55751905f610937565b3d9150610b6b565b7f4555892c000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101f55760206003193601126101f557602061038b6004355f525f602052600160405f20015490565b346101f5575f6003193601126101f557602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101f55760206003193601126101f5576004357fffffffff0000000000000000000000000000000000000000000000000000000081168091036101f557807f7965db0b0000000000000000000000000000000000000000000000000000000060209214908115610cd1575b506040519015158152f35b7f01ffc9a70000000000000000000000000000000000000000000000000000000091501482610cc6565b346101f55760206003193601126101f5576004358015610d9c577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8101908111610d6f5762278d0081029080820462278d001490151715610d6f5763688d46f001908163688d46f011610d6f576020918152f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7fd69368d4000000000000000000000000000000000000000000000000000000005f5260045ffd5b6024359073ffffffffffffffffffffffffffffffffffffffff821682036101f557565b6004359073ffffffffffffffffffffffffffffffffffffffff821682036101f557565b91908201809211610d6f57565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610e5857604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040517ffa391c6400000000000000000000000000000000000000000000000000000000815260208160048173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115610af5575f91610efb575090565b90506020813d602011610f2b575b81610f1660209383610e17565b810103126101f5575180151581036101f55790565b3d9150610f09565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff9772b9104201428111610d6f5762278d00900460018101809111610d6f5790565b60ff60015416610f7d57565b7fd93c0665000000000000000000000000000000000000000000000000000000005f5260045ffd5b335f9081527fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5602052604090205460ff1615610fdd57565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f205416156110445750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f1461113f57805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f1461113f57805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0df7c9542c591017a21c74b6f3fab6263c7952fc0aaf9db4c22a2a04ddc7f8674fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x03}R\xD6\x14a7GW\x80c\x06\xE9\x9E\x85\x14a7*W\x80c\x07\x8CfZ\x14a7\x04W\x80c\n\x92T\xE4\x14a.\x0FW\x80c\x1B\x80\xA4\xE5\x14a$]W\x80c\x1E\xD7\x83\x1C\x14a#\xDFW\x80c\x1F\0\x1D\xB4\x14a#\xB8W\x80c#\x12\xD7\xD7\x14a#\x91W\x80c*\xDE8\x80\x14a\"\x0CW\x80c.\xE8\x82=\x14a!\xEDW\x80c4\xD5\xF3{\x14a!\xCEW\x80c>^<#\x14a!PW\x80c?r\x86\xF4\x14a \xD2W\x80cLy\xCC\xA6\x14a \xABW\x80cQ\x84\xC5\xE7\x14a\x19!W\x80cf\xD9\xA9\xA0\x14a\x17\xE4W\x80c~\x04\x1C\x04\x14a\x0F\xB9W\x80c\x85\"l\x81\x14a\x0F'W\x80c\x91j\x17\xC6\x14a\x0E}W\x80c\xA2\xC8\xB1w\x14a\x0E`W\x80c\xB0FO\xDC\x14a\r\xB6W\x80c\xB5P\x8A\xA9\x14a\r$W\x80c\xBAAO\xA6\x14a\x0C\xFFW\x80c\xBE\"\xCCY\x14a\x01\xEBW\x80c\xE2\x0C\x9Fq\x14a\x01]Wc\xFAv&\xD4\x14a\x018W_\x80\xFD[4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x01\xCCWa\x01\xC8\x85a\x01\xBC\x81\x87\x03\x82a9vV[`@Q\x91\x82\x91\x82a7eV[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x01\xA5V[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x077W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x0C\xEAW[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x80;\x15a\x077W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xC6:\tD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\x0E\x04=\xA6\x17%\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x07,Wa\x0C\xD5W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x0C\xC0W[PP`\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90c\xF5\x08\xE1\x9D`\xE0\x1B\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\n\0W\x83\x92a\x0C\x8CW[P\x81`\x04` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x92\x83\x80\x92\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x0B?W\x85\x90a\x0CXW[a\x03\xCA\x91PaD\xB5V[`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a\x0C&W[P` T\x90_\x19\x82\x01\x91\x82\x11a\x0B\xF9W` \x92a\x04*`\x04\x95\x93a\x04/\x93aE5V[aD\xB5V[`@Q\x92\x83\x80\x92\x7FZ\xDF\0!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\n\0Wa\x04r\x91\x84\x91a\x0B\xDAW[PaE\xABV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW\x81`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x81\x1F\xCB\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x0B\xC5W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x0B\xB0W[P[`/\x81\x11\x15a\x08\"WP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x07\xB8W\x84\x91a\x07\xF0W[P` T\x90`/\x82\x01\x80\x92\x11a\x07\xC3W\x90a\x05\xC2\x91aE5V[`@Qc\xF5\x08\xE1\x9D`\xE0\x1B\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x07\xB8W\x84\x91a\x07~W[P\x91a\x05\xF6` \x92`\x04\x94aE5V[`@Q\x92\x83\x80\x92\x7F\xA4\xD7\xE3\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x07,Wa\x069\x91\x83\x91a\x07OW[PaF\x9BV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FEU\x89,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x07:W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x077W\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x07\x1BWP\xF3[\x81a\x07%\x91a9vV[a\x01ZW\x80\xF3[`@Q=\x84\x82>=\x90\xFD[P\xFD[\x81a\x07D\x91a9vV[a\x01ZW\x80_a\x06\xC9V[a\x07q\x91P` =` \x11a\x07wW[a\x07i\x81\x83a9vV[\x81\x01\x90a;\xEFV[_a\x063V[P=a\x07_V[\x92\x90P` \x83=` \x11a\x07\xB0W[\x81a\x07\x9A` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x91Qa\x05\xF6a\x05\xE6V[_\x80\xFD[=\x91Pa\x07\x8DV[`@Q=\x86\x82>=\x90\xFD[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11a\x08\x1AW[\x81a\x08\x0B` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ_a\x05\xA8V[=\x91Pa\x07\xFEV[\x90\x82`\x01`\x01`\xA0\x1B\x03`%T\x16` a\x08=\x81T\x86a;\xE2V[`$`@Q\x80\x94\x81\x93\x7F\x01u\xE2;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x07,W\x82\x91a\x0B|W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x0BgW[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x0BJW[PP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Qc\xF5\x08\xE1\x9D`\xE0\x1B\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a\x0B\x0CW[Pa\t\x84\x91a\t\x7F\x91a;\xD5V[a?$V[a\t\x8D\x83a=0V[\x90\x81\x81\x03a\n\x0BW[PP` `\x04\x91`@Q\x92\x83\x80\x92c\xF5\x08\xE1\x9D`\xE0\x1B\x82RZ\xFA\x80\x15a\n\0W\x83\x90a\t\xCDW[a\t\xC8\x91P\x91a<\x07V[a\x05TV[P` \x81=\x82\x11a\t\xF8W[\x81a\t\xE6` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa\t\xC8\x90Qa\t\xBDV[=\x91Pa\t\xD9V[`@Q=\x85\x82>=\x90\xFD[a\nN\x81\x83\x86`@Qa\n\x1F`@\x82a9vV[` \x81R\x7FEpoch %s: Expected %s, Actual %s` \x82\x01RaF\x1DV[`!Ta\n[\x81\x84a;\xE2V[\x82\x11\x92\x83\x15a\n\xF8W[PPPa\nsW_\x80a\t\x96V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FMint amount is not within accept`D\x82\x01R\x7Fed diff\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[a\x0B\x03\x92\x93Pa;\xD5V[\x11_\x80\x80a\neV[\x90P` \x81=\x82\x11a\x0B7W[\x81a\x0B&` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQa\t\x84a\tqV[=\x91Pa\x0B\x19V[`@Q=\x87\x82>=\x90\xFD[\x81a\x0BT\x91a9vV[a\x0B_W\x82_a\t>V[\x82\x80\xFD[P\x80\xFD[\x81a\x0Bq\x91a9vV[a\x0B_W\x82_a\x08\xEEV[\x91PP` \x81=\x82\x11a\x0B\xA8W[\x81a\x0B\x97` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x83\x90Q_a\x08~V[=\x91Pa\x0B\x8AV[\x81a\x0B\xBA\x91a9vV[a\x0BcW\x81_a\x05RV[\x81a\x0B\xCF\x91a9vV[a\x0BcW\x81_a\x05\x02V[a\x0B\xF3\x91P` =` \x11a\x07wWa\x07i\x81\x83a9vV[_a\x04lV[`$\x86\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11a\x0CPW[\x81a\x0CA` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ_a\x04\x07V[=\x91Pa\x0C4V[P` \x81=` \x11a\x0C\x84W[\x81a\x0Cr` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa\x03\xCA\x90Qa\x03\xC0V[=\x91Pa\x0CeV[\x90\x91P` \x81=` \x11a\x0C\xB8W[\x81a\x0C\xA8` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ\x90_a\x03uV[=\x91Pa\x0C\x9BV[\x81a\x0C\xCA\x91a9vV[a\x01ZW\x80_a\x03BV[\x81a\x0C\xDF\x91a9vV[a\x01ZW\x80_a\x02\xD6V[\x81a\x0C\xF4\x91a9vV[a\x01ZW\x80_a\x02xV[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW` a\r\x1AaC\xDCV[`@Q\x90\x15\x15\x81R\xF3[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`\x19Ta\rA\x81a<\x15V[\x91a\rO`@Q\x93\x84a9vV[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\r\x99W`@Q` \x80\x82R\x81\x90a\x01\xC8\x90\x82\x01\x88a7\xCCV[`\x01` \x81\x92a\r\xA8\x85a<-V[\x81R\x01\x92\x01\x92\x01\x91\x90a\r|V[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`\x1CTa\r\xD3\x81a<\x15V[\x91a\r\xE1`@Q\x93\x84a9vV[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\x0E#W`@Q\x80a\x01\xC8\x87\x82a8yV[`\x02` `\x01\x92`@Qa\x0E6\x81a9\x10V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x0EN\x85\x87\x01aA8V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0E\x0EV[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW` \x80T`@Q\x90\x81R\xF3[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`\x1DTa\x0E\x9A\x81a<\x15V[\x91a\x0E\xA8`@Q\x93\x84a9vV[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x0E\xEAW`@Q\x80a\x01\xC8\x87\x82a8yV[`\x02` `\x01\x92`@Qa\x0E\xFD\x81a9\x10V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x0F\x15\x85\x87\x01aA8V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0E\xD5V[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`\x1ATa\x0FD\x81a<\x15V[\x91a\x0FR`@Q\x93\x84a9vV[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\x0F\x9CW`@Q` \x80\x82R\x81\x90a\x01\xC8\x90\x82\x01\x88a7\xCCV[`\x01` \x81\x92a\x0F\xAB\x85a<-V[\x81R\x01\x92\x01\x92\x01\x91\x90a\x0F\x7FV[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x077W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x17\xCFW[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x80;\x15a\x077W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xC6:\tD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\r\xE0\xB6\xB3\xA7d\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x07,Wa\x17\xBAW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x17\xA5W[PP`\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90c\xF5\x08\xE1\x9D`\xE0\x1B\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\n\0W\x83\x92a\x17qW[P\x81`\x04` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x92\x83\x80\x92\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x0B?W\x85\x90a\x17=W[a\x11\x98\x91PaD\xB5V[`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a\x17\x0BW[P` T\x90_\x19\x82\x01\x91\x82\x11a\x0B\xF9W` \x92a\x04*`\x04\x95\x93a\x11\xF8\x93aE5V[`@Q\x92\x83\x80\x92\x7FZ\xDF\0!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\n\0Wa\x12:\x91\x84\x91a\x0B\xDAWPaE\xABV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW\x81`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x81\x1F\xCB\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x16\xF6W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x16\xE1W[P[`/\x81\x11\x15a\x13\x89WP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x07\xB8W\x84\x91a\x07\xF0WP` T\x90`/\x82\x01\x80\x92\x11a\x07\xC3W\x90a\x05\xC2\x91aE5V[\x90\x82`\x01`\x01`\xA0\x1B\x03`%T\x16` a\x13\xA4\x81T\x86a;\xE2V[`$`@Q\x80\x94\x81\x93\x7F\x01u\xE2;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x07,W\x82\x91a\x16\xADW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x16\x98W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x16\x83W[PP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Qc\xF5\x08\xE1\x9D`\xE0\x1B\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a\x16PW[Pa\x14\xE6\x91a\t\x7F\x91a;\xD5V[b\x19nk\x81\x03a\x15ZW[P` `\x04\x91`@Q\x92\x83\x80\x92c\xF5\x08\xE1\x9D`\xE0\x1B\x82RZ\xFA\x80\x15a\n\0W\x83\x90a\x15'W[a\x15\"\x91P\x91a<\x07V[a\x13\x1CV[P` \x81=\x82\x11a\x15RW[\x81a\x15@` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa\x15\"\x90Qa\x15\x17V[=\x91Pa\x153V[_\x80a\x15\xD7a\x15\xFA`@Qa\x15p`@\x82a9vV[` \x81R\x7FEpoch %s: Expected %s, Actual %s` \x82\x01R`@Q\x92\x83\x91\x7F\xA7\xA8xS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R`\x80`$\x84\x01R`\xA4\x83\x01\x90a7\xA7V[\x88`D\x83\x01Rb\x19nk`d\x83\x01R\x86`\x84\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a9vV[` \x81Q\x91\x01jconsole.logZ\xFAP`!T\x90\x81b\x19nk\x01\x80b\x19nk\x11a\x0B\xF9W\x81\x11\x91\x82\x15a\x169W[PPa\nsW_a\x14\xF1V[b\x19nk\x90\x81\x03\x92P\x82\x11a\x07\xC3W\x10_\x80a\x16-V[\x90P` \x81=\x82\x11a\x16{W[\x81a\x16j` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQa\x14\xE6a\x14\xD8V[=\x91Pa\x16]V[\x81a\x16\x8D\x91a9vV[a\x0B_W\x82_a\x14\xA5V[\x81a\x16\xA2\x91a9vV[a\x0B_W\x82_a\x14UV[\x91PP` \x81=\x82\x11a\x16\xD9W[\x81a\x16\xC8` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x83\x90Q_a\x13\xE5V[=\x91Pa\x16\xBBV[\x81a\x16\xEB\x91a9vV[a\x0BcW\x81_a\x13\x1AV[\x81a\x17\0\x91a9vV[a\x0BcW\x81_a\x12\xCAV[\x90P` \x81=` \x11a\x175W[\x81a\x17&` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ_a\x11\xD5V[=\x91Pa\x17\x19V[P` \x81=` \x11a\x17iW[\x81a\x17W` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa\x11\x98\x90Qa\x11\x8EV[=\x91Pa\x17JV[\x90\x91P` \x81=` \x11a\x17\x9DW[\x81a\x17\x8D` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ\x90_a\x11CV[=\x91Pa\x17\x80V[\x81a\x17\xAF\x91a9vV[a\x01ZW\x80_a\x11\x10V[\x81a\x17\xC4\x91a9vV[a\x01ZW\x80_a\x10\xA4V[\x81a\x17\xD9\x91a9vV[a\x01ZW\x80_a\x10FV[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`\x1BTa\x18\x01\x81a<\x15V[a\x18\x0E`@Q\x91\x82a9vV[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a\x18\xE6W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a\x18{WPPPP\x03\x90\xF3[\x91\x93` a\x18\xD6\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a\x18\xC6\x83Q`@\x84R`@\x84\x01\x90a7\xA7V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra8$V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x18lV[`\x02` `\x01\x92`@Qa\x18\xF9\x81a9\x10V[a\x19\x02\x86a<-V[\x81Ra\x19\x0F\x85\x87\x01aA8V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x18>V[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x077W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa \x96W[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x80;\x15a\x077W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xC6:\tD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\r\x99\xA8\xCE\xC7\xE2\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x07,Wa \x81W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa lW[PP`\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90c\xF5\x08\xE1\x9D`\xE0\x1B\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\n\0W\x83\x92a 8W[P\x81`\x04` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x92\x83\x80\x92\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x0B?W\x85\x90a \x04W[a\x1B\0\x91PaD\xB5V[`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a\x1F\xD2W[P` T\x90_\x19\x82\x01\x91\x82\x11a\x0B\xF9W` \x92a\x04*`\x04\x95\x93a\x1B`\x93aE5V[`@Q\x92\x83\x80\x92\x7FZ\xDF\0!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\n\0Wa\x1B\xA2\x91\x84\x91a\x0B\xDAWPaE\xABV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW\x81`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x81\x1F\xCB\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x1F\xBDW[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x1F\xA8W[P[`/\x81\x11\x15a\x1C\xF1WP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x07\xB8W\x84\x91a\x07\xF0WP` T\x90`/\x82\x01\x80\x92\x11a\x07\xC3W\x90a\x05\xC2\x91aE5V[\x90\x82`\x01`\x01`\xA0\x1B\x03`%T\x16` a\x1D\x0C\x81T\x86a;\xE2V[`$`@Q\x80\x94\x81\x93\x7F\x01u\xE2;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x07,W\x82\x91a\x1FtW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x1F_W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa\x1FJW[PP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Qc\xF5\x08\xE1\x9D`\xE0\x1B\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a\x1F\x17W[Pa\x1EN\x91a\t\x7F\x91a;\xD5V[a\x1EW\x83a9\xD7V[\x90\x81\x81\x03a\x1E\xCAW[PP` `\x04\x91`@Q\x92\x83\x80\x92c\xF5\x08\xE1\x9D`\xE0\x1B\x82RZ\xFA\x80\x15a\n\0W\x83\x90a\x1E\x97W[a\x1E\x92\x91P\x91a<\x07V[a\x1C\x84V[P` \x81=\x82\x11a\x1E\xC2W[\x81a\x1E\xB0` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa\x1E\x92\x90Qa\x1E\x87V[=\x91Pa\x1E\xA3V[a\x1E\xDE\x81\x83\x86`@Qa\n\x1F`@\x82a9vV[`!Ta\x1E\xEB\x81\x84a;\xE2V[\x82\x11\x92\x83\x15a\x1F\x03W[PPPa\nsW_\x80a\x1E`V[a\x1F\x0E\x92\x93Pa;\xD5V[\x11_\x80\x80a\x1E\xF5V[\x90P` \x81=\x82\x11a\x1FBW[\x81a\x1F1` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQa\x1ENa\x1E@V[=\x91Pa\x1F$V[\x81a\x1FT\x91a9vV[a\x0B_W\x82_a\x1E\rV[\x81a\x1Fi\x91a9vV[a\x0B_W\x82_a\x1D\xBDV[\x91PP` \x81=\x82\x11a\x1F\xA0W[\x81a\x1F\x8F` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x83\x90Q_a\x1DMV[=\x91Pa\x1F\x82V[\x81a\x1F\xB2\x91a9vV[a\x0BcW\x81_a\x1C\x82V[\x81a\x1F\xC7\x91a9vV[a\x0BcW\x81_a\x1C2V[\x90P` \x81=` \x11a\x1F\xFCW[\x81a\x1F\xED` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ_a\x1B=V[=\x91Pa\x1F\xE0V[P` \x81=` \x11a 0W[\x81a \x1E` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa\x1B\0\x90Qa\x1A\xF6V[=\x91Pa \x11V[\x90\x91P` \x81=` \x11a dW[\x81a T` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ\x90_a\x1A\xABV[=\x91Pa GV[\x81a v\x91a9vV[a\x01ZW\x80_a\x1AxV[\x81a \x8B\x91a9vV[a\x01ZW\x80_a\x1A\x0CV[\x81a \xA0\x91a9vV[a\x01ZW\x80_a\x19\xAEV[P4a\x01ZW` `\x03\x196\x01\x12a\x01ZW` a \xCA`\x045a?DV[`@Q\x90\x81R\xF3[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a!1Wa\x01\xC8\x85a\x01\xBC\x81\x87\x03\x82a9vV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a!\x1AV[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a!\xAFWa\x01\xC8\x85a\x01\xBC\x81\x87\x03\x82a9vV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a!\x98V[P4a\x01ZW` `\x03\x196\x01\x12a\x01ZW` a \xCA`\x045a?$V[P4a\x01ZW` `\x03\x196\x01\x12a\x01ZW` a \xCA`\x045a=0V[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`\x1ETa\")\x81a<\x15V[a\"6`@Q\x91\x82a9vV[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a#\x08W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a\"\xA3WPPPP\x03\x90\xF3[\x91\x93` a\"\xF8\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R`@\x83\x8AQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a7\xCCV[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\"\x94V[`@Qa#\x14\x81a9\x10V[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta#0\x81a<\x15V[\x91a#>`@Q\x93\x84a9vV[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a#tWPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\"fV[`\x01` \x81\x92a#\x83\x86a<-V[\x81R\x01\x93\x01\x91\x01\x90\x91a#NV[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x90\x81R\xF3[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW` `\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90\x81R\xF3[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a$>Wa\x01\xC8\x85a\x01\xBC\x81\x87\x03\x82a9vV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a$'V[P4a\x01ZW\x80`\x03\x196\x01\x12a\x01ZW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x077W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa-\xFAW[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x80;\x15a\x077W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xC6:\tD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\r\xE0\xB6\xB3\xA7d\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x07,Wa-\xE5W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa-\xD0W[PP`\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90c\xF5\x08\xE1\x9D`\xE0\x1B\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\n\0W\x83\x92a-\x9CW[P\x81`\x04` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x92\x83\x80\x92\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x0B?W\x85\x90a-hW[a&<\x91PaD\xB5V[`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a-6W[P` T\x90_\x19\x82\x01\x91\x82\x11a\x0B\xF9W` \x92a\x04*`\x04\x95\x93a&\x9C\x93aE5V[`@Q\x92\x83\x80\x92\x7FZ\xDF\0!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\n\0Wa&\xDE\x91\x84\x91a\x0B\xDAWPaE\xABV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW\x81`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x81\x1F\xCB\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa-!W[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa-\x0CW[P[`/\x81\x11\x15a(-WP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Q\x7F\xB9}\xD9\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x07\xB8W\x84\x91a\x07\xF0WP` T\x90`/\x82\x01\x80\x92\x11a\x07\xC3W\x90a\x05\xC2\x91aE5V[\x90`\x04\x82\x14a,\x01W[`\x15\x82\x14a*\xF6W[\x82`\x01`\x01`\xA0\x1B\x03`%T\x16` a(Z\x81T\x86a;\xE2V[`$`@Q\x80\x94\x81\x93\x7F\x01u\xE2;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x07,W\x82\x91a*\xC2W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa*\xADW[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x80;\x15a\x0BcW\x81\x80\x91`\x04`@Q\x80\x94\x81\x93\x7F(N\x133\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RZ\xF1\x80\x15a\x07,Wa*\x98W[PP`\x01`\x01`\xA0\x1B\x03`%T\x16\x90`@Qc\xF5\x08\xE1\x9D`\xE0\x1B\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0B?W\x85\x91a*eW[Pa)\x9C\x91a\t\x7F\x91a;\xD5V[a)\xA5\x83a?DV[\x90\x81\x81\x03a*\x18W[PP` `\x04\x91`@Q\x92\x83\x80\x92c\xF5\x08\xE1\x9D`\xE0\x1B\x82RZ\xFA\x80\x15a\n\0W\x83\x90a)\xE5W[a)\xE0\x91P\x91a<\x07V[a'\xC0V[P` \x81=\x82\x11a*\x10W[\x81a)\xFE` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa)\xE0\x90Qa)\xD5V[=\x91Pa)\xF1V[a*,\x81\x83\x86`@Qa\n\x1F`@\x82a9vV[`!Ta*9\x81\x84a;\xE2V[\x82\x11\x92\x83\x15a*QW[PPPa\nsW_\x80a)\xAEV[a*\\\x92\x93Pa;\xD5V[\x11_\x80\x80a*CV[\x90P` \x81=\x82\x11a*\x90W[\x81a*\x7F` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQa)\x9Ca)\x8EV[=\x91Pa*rV[\x81a*\xA2\x91a9vV[a\x0B_W\x82_a)[V[\x81a*\xB7\x91a9vV[a\x0B_W\x82_a)\x0BV[\x91PP` \x81=\x82\x11a*\xEEW[\x81a*\xDD` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x83\x90Q_a(\x9BV[=\x91Pa*\xD0V[\x82`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa+\xECW[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x80;\x15a\x0BcW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7FCX\x10\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\r\xCE\xF3:o\x83\x80\0`\x04\x84\x01RZ\xF1\x80\x15a\x07,Wa+\xD7W[PPa(@V[\x81a+\xE1\x91a9vV[a\x0B_W\x82_a+\xD0V[\x81a+\xF6\x91a9vV[a\x0B_W\x82_a+rV[\x82`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0BcW`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa,\xF7W[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x80;\x15a\x0BcW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7FCX\x10\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\x0E\x16\x01\x1FO\x05\x80\0`\x04\x84\x01RZ\xF1\x80\x15a\x07,Wa,\xE2W[PPa(7V[\x81a,\xEC\x91a9vV[a\x0B_W\x82_a,\xDBV[\x81a-\x01\x91a9vV[a\x0B_W\x82_a,}V[\x81a-\x16\x91a9vV[a\x0BcW\x81_a'\xBEV[\x81a-+\x91a9vV[a\x0BcW\x81_a'nV[\x90P` \x81=` \x11a-`W[\x81a-Q` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ_a&yV[=\x91Pa-DV[P` \x81=` \x11a-\x94W[\x81a-\x82` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWa&<\x90Qa&2V[=\x91Pa-uV[\x90\x91P` \x81=` \x11a-\xC8W[\x81a-\xB8` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ\x90_a%\xE7V[=\x91Pa-\xABV[\x81a-\xDA\x91a9vV[a\x01ZW\x80_a%\xB4V[\x81a-\xEF\x91a9vV[a\x01ZW\x80_a%HV[\x81a.\x04\x91a9vV[a\x01ZW\x80_a$\xEAV[P4a\x07\xACW_`\x03\x196\x01\x12a\x07\xACW`@Q\x7F\x98h\x004\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Fhttps://0xrpc.io/eth\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R` \x81`d\x81_sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a6\x9DWa6\xD5W[P` T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x01\x90\x81\x11a6\xA8Wb'\x8D\0\x81\x02\x90\x80\x82\x04b'\x8D\0\x14\x90\x15\x17\x15a6\xA8Wch\x8DF\xF0\x01\x80ch\x8DF\xF0\x11a6\xA8Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xACW`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a6\x9DWa6\x8AW[P`\x01`\x01`\xA0\x1B\x03`$T\x16\x15\x80\x15a6wW[a/\x84W\x80\xF3[_\x80a0'a05`@Qa/\x9A``\x82a9vV[`5\x81R\x7FEmissions contracts not found, d` \x82\x01R\x7Feploying ones to fork\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R`@Q\x92\x83\x91\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R` `$\x84\x01R`D\x83\x01\x90a7\xA7V[\x03`\x1F\x19\x81\x01\x83R\x82a9vV[` \x81Q\x91\x01jconsole.logZ\xFAP`@Q`\x86\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a6JW\x90\x82\x91aG\x0E\x839\x03\x90\x82\xF0\x80\x15a5\xE3W`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x90`@Q\x91a\x13B\x91\x82\x84\x01\x92\x84\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a6\x1DW\x91``\x93\x91\x85\x93aG\x94\x859\x82R\x80` \x83\x01R`@\x82\x01R\x03\x01\x90\x83\xF0\x80\x15a\x07,W`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$T\x16\x17`$U` T\x90`\x01`\x01`\xA0\x1B\x03`#T\x16\x90`@Q\x93a\x15\x1E\x93\x84\x86\x01\x94\x86\x86\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x11\x17a5\xF0W\x92\x86\x94\x92`\x01`\x01`\xA0\x1B\x03\x92`\xC0\x97\x95aZ\xD6\x889\x85R` \x85\x01R\x16`@\x83\x01R\x80``\x83\x01R\x80`\x80\x83\x01R`\xA0\x82\x01R\x03\x01\x90\x82\xF0\x80\x15a5\xE3W`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`%T\x16\x17`%U\x80`\x04` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92\x83\x80\x92\x7F\x8D3C\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x07,W\x82\x91a5\xAEW[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a56W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\n\0W\x83\x91a5\x99W[PP`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90`\x01`\x01`\xA0\x1B\x03`$T\x16\x82;\x15a5\x1CW`@Q\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x07,Wa5\x84W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa5oW[P`\x04` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x92\x83\x80\x92\x7F\xDE\xBEO\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x07,W\x82\x91a5:W[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a56W`@Q\x90\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\n\0W\x83\x91a5!W[PP`\x01`\x01`\xA0\x1B\x03`$T\x16\x90`\x01`\x01`\xA0\x1B\x03`%T\x16\x82;\x15a5\x1CW`@Q\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x07,Wa5\x07W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01ZW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x07,Wa\x07\x1BWPP\x80\xF3[\x81a5\x11\x91a9vV[a\x01ZW\x80_a4\x97V[PPP\xFD[\x81a5+\x91a9vV[a\x077W\x81_a4 V[PP\xFD[\x91PP` \x81=` \x11a5gW[\x81a5V` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x81\x90Q_a3\xA1V[=\x91Pa5IV[\x81a5y\x91a9vV[a\x01ZW\x80_a3VV[\x81a5\x8E\x91a9vV[a\x01ZW\x80_a2\xEAV[\x81a5\xA3\x91a9vV[a\x077W\x81_a2sV[\x91PP` \x81=` \x11a5\xDBW[\x81a5\xCA` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACW\x81\x90Q_a1\xF4V[=\x91Pa5\xBDV[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x88\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[`$\x87\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P`\x01`\x01`\xA0\x1B\x03`%T\x16\x15a/}V[a6\x96\x91P_\x90a9vV[__a/hV[`@Q=_\x82>=\x90\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[` \x81=` \x11a6\xFCW[\x81a6\xEE` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQa.\xA3V[=\x91Pa6\xE1V[4a\x07\xACW_`\x03\x196\x01\x12a\x07\xACW` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[4a\x07\xACW_`\x03\x196\x01\x12a\x07\xACW` `!T`@Q\x90\x81R\xF3[4a\x07\xACW` `\x03\x196\x01\x12a\x07\xACW` a \xCA`\x045a9\xD7V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a7\x88WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a7{V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a7\xF7WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a8\x15\x83`\x1F\x19\x86`\x01\x96\x03\x01\x87R\x89Qa7\xA7V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a7\xE8V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a8AWPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a84V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a8\xABWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a9\x01\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a8$V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a8\x9CV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a9,W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x06\0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a9,W`@RV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a9,W`@RV[\x90`0\x81\x10\x15a9\xAAW`\x05\x1B\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`@Q\x90a9\xE4\x82a9YV[b'Sk\x82Rb&\x8A\x11` \x83\x01Rb%\xC4\xBF`@\x83\x01Rb%\x03_``\x83\x01Rb$E\xDD`\x80\x83\x01Rb#\x8C&`\xA0\x83\x01Rb\"\xD6%`\xC0\x83\x01Rb\"#\xC8`\xE0\x83\x01Rb!t\xFCa\x01\0\x83\x01Rb \xC9\xB0a\x01 \x83\x01Rb !\xD0a\x01@\x83\x01Rb\x1F}La\x01`\x83\x01Rb\x1E\xDC\x12a\x01\x80\x83\x01Rb\x1E>\x12a\x01\xA0\x83\x01Rb\x1D\xA3:a\x01\xC0\x83\x01Rb\x1D\x0B|a\x01\xE0\x83\x01Rb\x1Cv\xC6a\x02\0\x83\x01Rb\x1B\xE5\na\x02 \x83\x01Rb\x1BV8a\x02@\x83\x01Rb\x1A\xCAAa\x02`\x83\x01Rb\x1AA\x16a\x02\x80\x83\x01Rb\x19\xBA\xABa\x02\xA0\x83\x01Rb\x196\xEFa\x02\xC0\x83\x01Rb\x18\xB5\xD6a\x02\xE0\x83\x01Rb\x187Qa\x03\0\x83\x01Rb\x17\xBBUa\x03 \x83\x01Rb\x17A\xD3a\x03@\x83\x01Rb\x16\xCA\xC0a\x03`\x83\x01Rb\x16V\x0Ea\x03\x80\x83\x01Rb\x15\xE3\xB1a\x03\xA0\x83\x01Rb\x15s\x9Ea\x03\xC0\x83\x01Rb\x15\x05\xC9a\x03\xE0\x83\x01Rb\x14\x9A'a\x04\0\x83\x01Rb\x140\xABa\x04 \x83\x01Rb\x13\xC9Ka\x04@\x83\x01Rb\x13c\xFDa\x04`\x83\x01Rb\x13\0\xB6a\x04\x80\x83\x01Rb\x12\x9Fja\x04\xA0\x83\x01Rb\x12@\x11a\x04\xC0\x83\x01Rb\x11\xE2\xA0a\x04\xE0\x83\x01Rb\x11\x87\x0Ea\x05\0\x83\x01Rb\x11-Pa\x05 \x83\x01Rb\x10\xD5^a\x05@\x83\x01Rb\x10\x7F.a\x05`\x83\x01Rb\x10*\xB7a\x05\x80\x83\x01Rb\x0F\xD7\xF1a\x05\xA0\x83\x01Rb\x0F\x86\xD4a\x05\xC0\x83\x01Rb\x0F7Ua\x05\xE0\x83\x01R`0\x81\x10\x15a;\xCFWa;\xCB\x91a9\x99V[Q\x90V[PP_\x90V[\x91\x90\x82\x03\x91\x82\x11a6\xA8WV[\x91\x90\x82\x01\x80\x92\x11a6\xA8WV[\x90\x81` \x91\x03\x12a\x07\xACWQ\x80\x15\x15\x81\x03a\x07\xACW\x90V[_\x19\x81\x14a6\xA8W`\x01\x01\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a9,W`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15a=&W[` \x85\x10\x84\x14a<\xF9W\x84\x87R\x86\x93\x90\x81\x15a<\xB9WP`\x01\x14a<uW[Pa<s\x92P\x03\x83a9vV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a<\x9DWPP\x90` a<s\x92\x82\x01\x01_a<fV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a<\x84V[` \x93Pa<s\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a<fV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93a<GV[`@Q\x90a==\x82a9YV[b\x13\xF0R\x82Rb\x14#]` \x83\x01Rb\x14V\xEB`@\x83\x01Rb\x14\x8A\xFD``\x83\x01Rb\x14\xBF\x94`\x80\x83\x01Rb\x14\xF4\xB2`\xA0\x83\x01Rb\x15*W`\xC0\x83\x01Rb\x15`\x86`\xE0\x83\x01Rb\x15\x97@a\x01\0\x83\x01Rb\x15\xCE\x86a\x01 \x83\x01Rb\x16\x06Ya\x01@\x83\x01Rb\x16>\xBBa\x01`\x83\x01Rb\x16w\xAEa\x01\x80\x83\x01Rb\x16\xB12a\x01\xA0\x83\x01Rb\x16\xEBIa\x01\xC0\x83\x01Rb\x17%\xF6a\x01\xE0\x83\x01Rb\x17a8a\x02\0\x83\x01Rb\x17\x9D\x12a\x02 \x83\x01Rb\x17\xD9\x86a\x02@\x83\x01Rb\x18\x16\x94a\x02`\x83\x01Rb\x18T>a\x02\x80\x83\x01Rb\x18\x92\x87a\x02\xA0\x83\x01Rb\x18\xD1oa\x02\xC0\x83\x01Rb\x19\x10\xF7a\x02\xE0\x83\x01Rb\x19Q#a\x03\0\x83\x01Rb\x19\x91\xF2a\x03 \x83\x01Rb\x19\xD3ha\x03@\x83\x01Rb\x1A\x15\x85a\x03`\x83\x01Rb\x1AXLa\x03\x80\x83\x01Rb\x1A\x9B\xBDa\x03\xA0\x83\x01Rb\x1A\xDF\xDBa\x03\xC0\x83\x01Rb\x1B$\xA8a\x03\xE0\x83\x01Rb\x1Bj%a\x04\0\x83\x01Rb\x1B\xB0Sa\x04 \x83\x01Rb\x1B\xF75a\x04@\x83\x01Rb\x1C>\xCDa\x04`\x83\x01Rb\x1C\x87\x1Ca\x04\x80\x83\x01Rb\x1C\xD0$a\x04\xA0\x83\x01Rb\x1D\x19\xE6a\x04\xC0\x83\x01Rb\x1Ddfa\x04\xE0\x83\x01Rb\x1D\xAF\xA5a\x05\0\x83\x01Rb\x1D\xFB\xA4a\x05 \x83\x01Rb\x1EHfa\x05@\x83\x01Rb\x1E\x95\xECa\x05`\x83\x01Rb\x1E\xE49a\x05\x80\x83\x01Rb\x1F3Na\x05\xA0\x83\x01Rb\x1F\x83.a\x05\xC0\x83\x01Rb\x1F\xD3\xDAa\x05\xE0\x83\x01R`0\x81\x10\x15a;\xCFWa;\xCB\x91a9\x99V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x01\x80\x91\x11a6\xA8Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[`@Q\x90a?Q\x82a9YV[b\x19nk\x82Rb\x19nk` \x83\x01Rb\x19nk`@\x83\x01Rb\x19nk``\x83\x01Rb\x12#\x99`\x80\x83\x01Rb\x12i@`\xA0\x83\x01Rb\x12\xAF\xF3`\xC0\x83\x01Rb\x12\xF7\xB5`\xE0\x83\x01Rb\x13@\x8Ba\x01\0\x83\x01Rb\x13\x8Aya\x01 \x83\x01Rb\x13\xD5\x83a\x01@\x83\x01Rb\x14!\xACa\x01`\x83\x01Rb\x14n\xFAa\x01\x80\x83\x01Rb\x14\xBDqa\x01\xA0\x83\x01Rb\x15\r\x16a\x01\xC0\x83\x01Rb\x15]\xECa\x01\xE0\x83\x01Rb\x15\xAF\xF8a\x02\0\x83\x01Rb\x16\x03@a\x02 \x83\x01Rb\x16W\xC7a\x02@\x83\x01Rb\x16\xAD\x93a\x02`\x83\x01Rb\x17\x04\xA8a\x02\x80\x83\x01Rb\x1EqLa\x02\xA0\x83\x01Rb\x1EJTa\x02\xC0\x83\x01Rb\x1E#\x8Fa\x02\xE0\x83\x01Rb\x1D\xFC\xFBa\x03\0\x83\x01Rb\x1D\xD6\x98a\x03 \x83\x01Rb\x1D\xB0ga\x03@\x83\x01Rb\x1D\x8Aga\x03`\x83\x01Rb\x1Dd\x97a\x03\x80\x83\x01Rb\x1D>\xF7a\x03\xA0\x83\x01Rb\x1D\x19\x88a\x03\xC0\x83\x01Rb\x1C\xF4Ia\x03\xE0\x83\x01Rb\x1C\xCF9a\x04\0\x83\x01Rb\x1C\xAAYa\x04 \x83\x01Rb\x1C\x85\xA7a\x04@\x83\x01Rb\x1Ca%a\x04`\x83\x01Rb\x1C<\xD2a\x04\x80\x83\x01Rb\x1C\x18\xADa\x04\xA0\x83\x01Rb\x1B\xF4\xB7a\x04\xC0\x83\x01Rb\x1B\xD0\xEEa\x04\xE0\x83\x01Rb\x1B\xADSa\x05\0\x83\x01Rb\x1B\x89\xE6a\x05 \x83\x01Rb\x1Bf\xA7a\x05@\x83\x01Rb\x1BC\x94a\x05`\x83\x01Rb\x1B \xAEa\x05\x80\x83\x01Rb\x1A\xFD\xF5a\x05\xA0\x83\x01Rb\x1A\xDBia\x05\xC0\x83\x01Rb\x1A\xB9\ta\x05\xE0\x83\x01R`0\x81\x10\x15a;\xCFWa;\xCB\x91a9\x99V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10aCOWa<s\x94T\x91\x81\x81\x10aC\x19W[\x81\x81\x10aB\xE3W[\x81\x81\x10aB\xADW[\x81\x81\x10aBwW[\x81\x81\x10aBAW[\x81\x81\x10aB\x0BW[\x81\x81\x10aA\xD6W[\x10aA\xA9W[P\x03\x83a9vV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_aA\xA1V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01aA\x9BV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01aA\x93V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01aA\x8BV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01aA\x83V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01aA{V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01aAsV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01aAkV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91aASV[`\x08T`\xFF\x16\x80\x15aC\xEBW\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a6\x9DW_\x91aD\x83W[P\x15\x15\x90V[\x90P` \x81=` \x11aD\xADW[\x81aD\x9E` \x93\x83a9vV[\x81\x01\x03\x12a\x07\xACWQ_aD}V[=\x91PaD\x91V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xACW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a6\x9DWaE+WPV[_a<s\x91a9vV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xACW`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a6\x9DWaE+WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xACW`@Q\x90\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a6\x9DWaE+WPV[aF\x84\x90aFf\x92_\x95\x86\x95`@Q\x95\x86\x94\x7F\xA7\xA8xS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x87\x01R`\x80`$\x87\x01R`\xA4\x86\x01\x90a7\xA7V[\x92`D\x85\x01R`d\x84\x01R`\x84\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a9vV[` \x81Q\x91\x01jconsole.logZ\xFAPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xACW`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a6\x9DWaE+WPV\xFE`\x80\x80`@R4`\x13W`n\x90\x81`\x18\x829\xF3[_\x80\xFD\xFE`\x046\x10\x15`\x0BW_\x80\xFD[_5`\xE0\x1Cc\xEE\xEC\x0E$\x14`\x1DW_\x80\xFD[4`jW`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12`jW`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03`jW\0[_\x80\xFD`\xA04a\0\xD9W`\x1Fa\x13B8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xDDW\x80\x84\x92``\x94`@R\x839\x81\x01\x03\x12a\0\xD9Wa\0G\x81a\0\xF1V[a\0_`@a\0X` \x85\x01a\0\xF1V[\x93\x01a\0\xF1V[\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\0\xCAW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\0\xCAW`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\0\xCAWa\0\xA3\x92a\0\x9D\x91`\x80Ra\x01\x05V[Pa\x01{V[P`@Qa\x10\xD3\x90\x81a\x02\x0F\x829`\x80Q\x81\x81\x81a\x03!\x01R\x81\x81a\x07\xF1\x01Ra\rT\x01R\xF3[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0\xD9WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x13\"_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x01vW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x13\"_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a\x12\xE2_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x13\x02_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x01vW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x13\x02_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\xF2\x7F\x93h\xAA\x1Bv\x97\xE5\x04)\xF87\"\xB8\x8F.]\xB8\x18M^,\xBC\xF0`\xF41\x0B\xBD>}\x90_Q` a\x12\xE2_9_Q\x90_R\x90\x80\xA4`\x01\x90V\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x01\xFF\xC9\xA7\x14a\t\xFAWP\x80c\x15\x8E\xF9>\x14a\t\xD8W\x80c$\x8A\x9C\xA3\x14a\t\xAEW\x80c//\xF1]\x14a\tqW\x80c6V\x8A\xBE\x14a\t\x05W\x80cCX\x10\x10\x14a\x08/W\x80cC\xA3\xF8\xA1\x14a\x08\x15W\x80c[\xDFl\xA1\x14a\x07\xC5W\x80c_\x15\xC3\xC9\x14a\x07\xAAW\x80cvg\x18\x08\x14a\x07\x8DW\x80c\x89\x16$\x86\x14a\x07SW\x80c\x91\xD1HT\x14a\x06\xFDW\x80c\xA0\x88x}\x14a\x06\xBAW\x80c\xA2\x17\xFD\xDF\x14a\x06\xA0W\x80c\xAC\x12\xCE\x07\x14a\x06\x83W\x80c\xB1\x98\xD0(\x14a\x06^W\x80c\xC6:\tD\x14a\x05PW\x80c\xD3\xF5f\xAE\x14a\x02VW\x80c\xD5Gt\x1F\x14a\x02\x0FW\x80c\xDE\xBEO\x1F\x14a\x01\xD4W\x80c\xDF\x02D\xB1\x14a\x01\xB6W\x80c\xE0\xE6\x16\x9C\x14a\x01\x9BW\x80c\xE4\xB7\xFBs\x14a\x01xW\x80c\xEC\xEDU&\x14a\x01UWc\xFA9\x1Cd\x14a\x011W_\x80\xFD[4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `0`\x02T\x10\x15`@Q\x90\x81R\xF3[\x80\xFD[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `@Qg\r\xE0\xB6\xB3\xA7d\0\0\x81R\xF3[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` a\x01\x93a\r,V[`@Q\x90\x81R\xF3[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` a\x01\x93a\x0C\xEFV[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `\x03T`@Q\x90\x81R\xF3[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `@Q\x7F\x07n\xB8\xB8u\xB6\xEA\x83\x9B\x08|L\x0C\x1AFa\xB0\x89\xD3\xB6\xEE,\x1E\xF1\xB9\xCF\xA7\xFE\x10f\xD2\x06\x81R\xF3[P4a\x01RW`@`\x03\x196\x01\x12a\x01RWa\x02R`\x045a\x02/a\n\x98V[\x90a\x02Ma\x02H\x82_R_` R`\x01`@_ \x01T\x90V[a\x0E\xD3V[a\x10\x0BV[P\x80\xF3[P4a\x04QW`@`\x03\x196\x01\x12a\x04QW`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x04QW3_\x90\x81R\x7F\x0E%9\x0F\xF9SSX\xA5\xE9\x16\xDF\xE7\xD3\x82f\xC86\x01\xAFn\x11!\x05\xB2-\xF4\xA9\x0B\xF8\x91\x01` R`@\x90 T`$5\x90`\xFF\x16\x15a\x05\0W`\xFF`\x04T\x16\x15a\x04\x82W`\x02T\x90`0\x82\x10\x15a\x04\x82W\x83\x15a\x04\xD8W\x80\x82\x03a\x04\xAAWPPa\x02\xF3a\x0B\x05V[\x90\x81\x15a\x04\x82W`\x03T\x82\x81\x01\x80\x91\x11a\x04UW`\x03Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x04QW_\x80\x91`D`@Q\x80\x94\x81\x93\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x89`\x04\x84\x01R\x88`$\x84\x01RZ\xF1\x80\x15a\x04FWa\x043W[P`\x02T\x92\x83\x7F\x16\x0F\xC1\x95\xD6\xE56\x91\xD3\r\x80L\xE1\x90\xDC\tG\x18\x91g~CC;\x91\xA7\xA6\x13\x1C\x12\xA5\x9A`@a\x03\xC1a\r,V[\x81Q\x90\x87\x82R` \x82\x01R\xA3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14a\x04\x06WP`\x01` \x92\x01`\x02U`@Q\x90\x81R\xF3[\x80\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x92R`\x11`\x04R\xFD[a\x04?\x91P_\x90a\x0C\x81V[__a\x03\x90V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7F\x9E\x91\xC9\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F0A:\x1A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x07n\xB8\xB8u\xB6\xEA\x83\x9B\x08|L\x0C\x1AFa\xB0\x89\xD3\xB6\xEE,\x1E\xF1\xB9\xCF\xA7\xFE\x10f\xD2\x06`$R`D_\xFD[4a\x04QW` `\x03\x196\x01\x12a\x04QW3_\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\x045\x90`\xFF\x16\x15a\x06.W`\x04T`\xFF\x81\x16a\x04\x82W\x81\x15a\x06\x06W\x7F\xC1,`\xAB\xC2\x16(n\xF2^4\xB1\x80Z\x0C=\xDAs\xE4\xC2\xFDl\xF3`\xE8\x07\xA7\xA9\xE71g9\x91`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0` \x93\x16\x17`\x04U\x80`\x01U`@Q\x90\x81R\xA1\0[\x7F\xEBv\x99 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@QjB,\xA8\xB0\xA0\nBP\0\0\0\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `\x01T`@Q\x90\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Q_\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW`\xA0`\x02T`0`\x03T\x91a\x06\xDDa\r,V[`@Q\x93\x82\x85R\x83` \x86\x01R`@\x85\x01R``\x84\x01R\x10\x15`\x80\x82\x01R\xF3[4a\x04QW`@`\x03\x196\x01\x12a\x04QWa\x07\x16a\n\x98V[`\x045_R_` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Q\x7F\xF2\x7F\x93h\xAA\x1Bv\x97\xE5\x04)\xF87\"\xB8\x8F.]\xB8\x18M^,\xBC\xF0`\xF41\x0B\xBD>}\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `\x02T`@Q\x90\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Q`0\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` a\x01\x93a\x0B\x05V[4a\x04QW` `\x03\x196\x01\x12a\x04QW3_\x90\x81R\x7F~\x972wWE-\x9E3\x15\xE3\x95\xBC\x10\x0E}\xA1\xF2\xD3Q\x06\xFF\xFD8\xF28\x07tp\x90\xEF\xA9` R`@\x90 T`\x045\x90`\xFF\x16\x15a\x08\xB5W\x80\x15a\x06\x06W\x80`\x01U`\x02T`@Q\x91\x82R\x7F\xB8\x13\xFF\xBE8}l\xF6\xE6\xA6\xF6\xC5\xF8\x90_vj\x0F\x1Cl\xD0\x1Cg1/p\x93V\xC6%\x97\xBD` 3\x93\xA3\0[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\xF2\x7F\x93h\xAA\x1Bv\x97\xE5\x04)\xF87\"\xB8\x8F.]\xB8\x18M^,\xBC\xF0`\xF41\x0B\xBD>}`$R`D_\xFD[4a\x04QW`@`\x03\x196\x01\x12a\x04QWa\t\x1Ea\n\x98V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\tIWa\tG\x90`\x045a\x10\x0BV[\0[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04QW`@`\x03\x196\x01\x12a\x04QWa\tG`\x045a\t\x90a\n\x98V[\x90a\t\xA9a\x02H\x82_R_` R`\x01`@_ \x01T\x90V[a\x0F9V[4a\x04QW` `\x03\x196\x01\x12a\x04QW` a\x01\x93`\x045_R_` R`\x01`@_ \x01T\x90V[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `\xFF`\x04T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04QW` `\x03\x196\x01\x12a\x04QW`\x045\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x80\x92\x03a\x04QW\x81\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x93\x14\x90\x81\x15a\nnW[P\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x83a\ngV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x04QWV[\x81\x15a\n\xC5W\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x04UWV[`\xFF`\x04T\x16\x15\x80\x15a\x0CtW[a\x0CpW`\x02T`0\x03`0\x81\x11a\x04UWa\x0B-a\r,V[`\x01\x82\x14a\x0CkW`\x01T\x91g\r\xE0\xB6\xB3\xA7d\0\0\x83\x14a\x0C`WPa\x0BQa\x0C\xEFV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x0C\x19W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2\x1FILX\x9C\0\0\x81\x01\x90\x81\x11a\x04UW[a\x03\xE8\x81\x10\x15a\x0C\x13WPa\x03\xE8\x90[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15a\x0B\xEBW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2\x1FILX\x9C\0\0\x83\x01\x92\x83\x11a\x04UWa\x0B\xE8\x92a\x0B\xE3\x91a\n\xF2V[a\n\xBBV[\x90V[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x03\x91g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11a\x04UWa\x0B\xE8\x92a\x0B\xE3\x91a\n\xF2V[\x90a\x0B\x9CV[g\r\xE0\xB6\xB3\xA7d\0\0\x03g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x0B\x8CW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x90a\x0B\xE8\x92Pa\n\xBBV[\x90P\x90V[_\x90V[P`0`\x02T\x10\x15a\x0B\x13V[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\xC2W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\r\xE0\xB6\xB3\xA7d\0\0`\x02T`\x01T[`0\x82\x10a\r\x0CWPP\x90V[\x90\x91g\r\xE0\xB6\xB3\xA7d\0\0a\r#\x83`\x01\x93a\n\xF2V[\x04\x92\x01\x90a\x0C\xFFV[`@Q\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\x04FW_\x92a\x0E\x9EW[P` `\x04\x91`@Q\x92\x83\x80\x92\x7F\x90-U\xA5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04FW_\x91a\x0ElW[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xBD\xD3WO_\xF5\xBD\xB0\0\0\0\x81\x01\x81\x81\x11a\x04UW\x82\x11\x15a\x0EdW\x81\x03jB,\xA8\xB0\xA0\nBP\0\0\0\x01\x90\x81\x11a\x04UW[\x80jB,\xA8\xB0\xA0\nBP\0\0\0\x11_\x14a\x0E_WjB,\xA8\xB0\xA0\nBP\0\0\0\x03jB,\xA8\xB0\xA0\nBP\0\0\0\x81\x11a\x04UW\x90V[P_\x90V[PP_a\x0E)V[\x90P` \x81=` \x11a\x0E\x96W[\x81a\x0E\x87` \x93\x83a\x0C\x81V[\x81\x01\x03\x12a\x04QWQ_a\r\xE0V[=\x91Pa\x0EzV[\x90\x91P` \x81=` \x11a\x0E\xCBW[\x81a\x0E\xBA` \x93\x83a\x0C\x81V[\x81\x01\x03\x12a\x04QWQ\x90` a\r\xA0V[=\x91Pa\x0E\xADV[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x0F\nWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x10\x05W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x10\x05W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r~\x972wWE-\x9E3\x15\xE3\x95\xBC\x10\x0E}\xA1\xF2\xD3Q\x06\xFF\xFD8\xF28\x07tp\x90\xEF\xA9\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5`\xC04a\x01{W`\x1Fa\x15\x1E8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\x7FW\x80\x84\x92`\xC0\x94`@R\x839\x81\x01\x03\x12a\x01{W\x80Q\x90a\0M` \x82\x01a\x01\x93V[a\0Y`@\x83\x01a\x01\x93V[a\0e``\x84\x01a\x01\x93V[\x92a\0~`\xA0a\0w`\x80\x84\x01a\x01\x93V[\x92\x01a\x01\x93V[`\x01\x80T`\xFF\x19\x16\x81U`\x02U\x92\x85\x15a\x01lW`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15a\x01]W`\x01`\x01`\xA0\x1B\x03\x16\x93\x84\x15a\x01]W`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x01]W`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x01]W`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x01]Wa\x01\x15\x94a\x01\x0F\x93`\x80R`\x01\x80`\xA0\x1B\x03\x19`\x03T\x16\x17`\x03U`\x01\x80`\xA0\x1B\x03\x19`\x04T\x16\x17`\x04Ua\x01\xA7V[Pa\x02\x1DV[P`\xA0R`@Qa\x12\r\x90\x81a\x02\xB1\x829`\x80Q\x81\x81\x81a\x01\x90\x01R\x81\x81a\x08\xEB\x01R\x81\x81a\x0C6\x01Ra\x0E\xC8\x01R`\xA0Q\x81\x81\x81a\x03M\x01R\x81\x81a\x06\xB6\x01Ra\t:\x01R\xF3[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[c\xD5\xB2[c`\xE0\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01{WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x14\xFE_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x02\x18W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x14\xFE_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a\x14\xBE_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x14\xDE_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x02\x18W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x14\xDE_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x90_Q` a\x14\xBE_9_Q\x90_R\x90\x80\xA4`\x01\x90V\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x01u\xE2;\x14a\x0C\xFBWP\x80c\x01\xFF\xC9\xA7\x14a\x0CZW\x80c#\x12\xD7\xD7\x14a\x0C\nW\x80c$\x8A\x9C\xA3\x14a\x0B\xE0W\x80c(N\x133\x14a\x08\x91W\x80c//\xF1]\x14a\x08SW\x80c1\x1FQi\x14a\x07\xE9W\x80c6V\x8A\xBE\x14a\x07~W\x80c?K\xA8:\x14a\x06\xDFW\x80cZ\xDF\0!\x14a\x06\x99W\x80c\\\x97Z\xBB\x14a\x06vW\x80ceH\xE9\xBC\x14a\x05\xE4W\x80co\xD3\xC9\xF0\x14a\x05\xB0W\x80cx\x1C\xD9\x9D\x14a\x05\x91W\x80c\x84\x06\xC0y\x14a\x05]W\x80c\x84V\xCBY\x14a\x04IW\x80c\x91\xD1HT\x14a\x03\xF2W\x80c\xA2\x17\xFD\xDF\x14a\x03\xD6W\x80c\xA4\xD7\xE3\x1D\x14a\x03\xB1W\x80c\xA7\x0B\x9F\x0C\x14a\x03\x93W\x80c\xB9}\xD9\xE2\x14a\x03pW\x80c\xBC\xCF$\xE3\x14a\x035W\x80c\xD5\x17m#\x14a\x02\x92W\x80c\xD5Gt\x1F\x14a\x02KW\x80c\xE6:\xB1\xE9\x14a\x02\x10Wc\xF5\x08\xE1\x9D\x14a\x01<W_\x80\xFD[4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW`@Q\x90\x7F\xDF\x02D\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x02\x01W\x90a\x01\xCAW[` \x90`@Q\x90\x81R\xF3[P` \x81=` \x11a\x01\xF9W[\x81a\x01\xE4` \x93\x83a\x0E\x17V[\x81\x01\x03\x12a\x01\xF5W` \x90Qa\x01\xBFV[_\x80\xFD[=\x91Pa\x01\xD7V[`@Q\x90=\x90\x82>=\x90\xFD[\x80\xFD[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `@Q\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x81R\xF3[P4a\x02\rW`@`\x03\x196\x01\x12a\x02\rWa\x02\x8E`\x045a\x02ka\r\xC4V[\x90a\x02\x89a\x02\x84\x82_R_` R`\x01`@_ \x01T\x90V[a\x10\rV[a\x11EV[P\x80\xF3[P4a\x02\rW` `\x03\x196\x01\x12a\x02\rW`\x045b'\x8D\0\x81\x02\x90\x80\x82\x04b'\x8D\0\x14\x90\x15\x17\x15a\x03\x08Wch\x8DF\xF0\x01\x90\x81ch\x8DF\xF0\x11a\x02\xDBW` \x82`@Q\x90\x81R\xF3[\x80\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x92R`\x11`\x04R\xFD[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` a\x03\x8Ba\x0F3V[`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `@Qb'\x8D\0\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` a\x03\xCCa\x0E\x85V[`@Q\x90\x15\x15\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` \x90`@Q\x90\x81R\xF3[P4a\x02\rW`@`\x03\x196\x01\x12a\x02\rWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@a\x04#a\r\xC4V[\x92`\x045\x81R\x80` R \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x81R\x80` R`@\x81 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x05\rWa\x04\xB6a\x0FqV[`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x17`\x01U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\x80\xF3[\x80\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x92R3`\x04R\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*`$R\xFD[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03T\x16`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `@Qch\x8DF\xF0\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x04T\x16`@Q\x90\x81R\xF3[P4a\x02\rW` `\x03\x196\x01\x12a\x02\rWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x06\x13a\r\xE7V[a\x06\x1Ba\x0F\xA5V[\x16\x80\x15a\x06NW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x03T\x16\x17`\x03U\x80\xF3[`\x04\x82\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` `\xFF`\x01T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rW` a\x06\xB4a\x0F3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x15`@Q\x90\x81R\xF3[P4a\x02\rW\x80`\x03\x196\x01\x12a\x02\rWa\x06\xF8a\x0F\xA5V[`\x01T`\xFF\x81\x16\x15a\x07VW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\x80\xF3[`\x04\x82\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x02\rW`@`\x03\x196\x01\x12a\x02\rWa\x07\x98a\r\xC4V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\x07\xC1Wa\x02\x8E\x90`\x045a\x11EV[`\x04\x82\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x02\rW` `\x03\x196\x01\x12a\x02\rWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x08\x18a\r\xE7V[a\x08 a\x0F\xA5V[\x16\x80\x15a\x06NW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04T\x16\x17`\x04U\x80\xF3[P4a\x02\rW`@`\x03\x196\x01\x12a\x02\rWa\x02\x8E`\x045a\x08sa\r\xC4V[\x90a\x08\x8Ca\x02\x84\x82_R_` R`\x01`@_ \x01T\x90V[a\x10sV[P4a\x01\xF5W_`\x03\x196\x01\x12a\x01\xF5Wa\x08\xAAa\x0FqV[`\x02\x80T\x14a\x0B\xB8W`\x02\x80Ua\x08\xBFa\x0E\x85V[a\x0B\x90W`@Q\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\n\xF5W_\x92a\x0B\\W[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\tc\x81\x84a\x0E\nV[a\tka\x0F3V[\x10a\x0B4W`\x03T`@Q\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91` \x90\x83\x90`D\x90\x82\x90_\x90Z\xF1\x91\x82\x15a\n\xF5W_\x92a\x0B\0W[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03T\x16\x90a\n\x18s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x04T\x16\x91\x85a\x0E\nV[\x82;\x15a\x01\xF5W`@Q\x7F\xEE\xEC\x0E$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R\x90_\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\n\xF5Wa\n\xBAW[P\x7F\xEF\x80\xC2y\xC1x\xDDc\xCD\xAFPa\"K\xE8n\xE3%\xC4\xF4@m\x80-\x04>\xD3;E\xB2\xF6\x8F\x91`@\x91\x82Q\x91\x82R` \x82\x01R\xA1`\x01`\x02U\x80\xF3[`@\x91\x93P\x91a\n\xEB_\x7F\xEF\x80\xC2y\xC1x\xDDc\xCD\xAFPa\"K\xE8n\xE3%\xC4\xF4@m\x80-\x04>\xD3;E\xB2\xF6\x8F\x94a\x0E\x17V[_\x93\x91P\x91a\n\x81V[`@Q=_\x82>=\x90\xFD[\x90\x91P` \x81=` \x11a\x0B,W[\x81a\x0B\x1C` \x93\x83a\x0E\x17V[\x81\x01\x03\x12a\x01\xF5WQ\x90_a\t\xDAV[=\x91Pa\x0B\x0FV[\x7F\x81\x1F\xCB\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90\x91P` \x81=` \x11a\x0B\x88W[\x81a\x0Bx` \x93\x83a\x0E\x17V[\x81\x01\x03\x12a\x01\xF5WQ\x90_a\t7V[=\x91Pa\x0BkV[\x7FEU\x89,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W` a\x03\x8B`\x045_R_` R`\x01`@_ \x01T\x90V[4a\x01\xF5W_`\x03\x196\x01\x12a\x01\xF5W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W`\x045\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x80\x91\x03a\x01\xF5W\x80\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x92\x14\x90\x81\x15a\x0C\xD1W[P`@Q\x90\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x82a\x0C\xC6V[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W`\x045\x80\x15a\r\x9CW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a\roWb'\x8D\0\x81\x02\x90\x80\x82\x04b'\x8D\0\x14\x90\x15\x17\x15a\roWch\x8DF\xF0\x01\x90\x81ch\x8DF\xF0\x11a\roW` \x91\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7F\xD6\x93h\xD4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xF5WV[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xF5WV[\x91\x90\x82\x01\x80\x92\x11a\roWV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0EXW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q\x7F\xFA9\x1Cd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\n\xF5W_\x91a\x0E\xFBWP\x90V[\x90P` \x81=` \x11a\x0F+W[\x81a\x0F\x16` \x93\x83a\x0E\x17V[\x81\x01\x03\x12a\x01\xF5WQ\x80\x15\x15\x81\x03a\x01\xF5W\x90V[=\x91Pa\x0F\tV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97r\xB9\x10B\x01B\x81\x11a\roWb'\x8D\0\x90\x04`\x01\x81\x01\x80\x91\x11a\roW\x90V[`\xFF`\x01T\x16a\x0F}WV[\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[3_\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16\x15a\x0F\xDDWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x10DWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x11?W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x11?W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\xF7\xC9T,Y\x10\x17\xA2\x1Ct\xB6\xF3\xFA\xB6&<yR\xFC\n\xAF\x9D\xB4\xC2**\x04\xDD\xC7\xF8gO\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5",
    );
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
    /**Function with signature `acceptedDiff()` and selector `0x06e99e85`.
```solidity
function acceptedDiff() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptedDiffCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`acceptedDiff()`](acceptedDiffCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptedDiffReturn {
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
            impl ::core::convert::From<acceptedDiffCall> for UnderlyingRustTuple<'_> {
                fn from(value: acceptedDiffCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for acceptedDiffCall {
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
            impl ::core::convert::From<acceptedDiffReturn> for UnderlyingRustTuple<'_> {
                fn from(value: acceptedDiffReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for acceptedDiffReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptedDiffCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptedDiff()";
            const SELECTOR: [u8; 4] = [6u8, 233u8, 158u8, 133u8];
            #[inline]
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
                        let r: acceptedDiffReturn = r.into();
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
                        let r: acceptedDiffReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `emissionsCalculator()` and selector `0x2312d7d7`.
```solidity
function emissionsCalculator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emissionsCalculatorCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`emissionsCalculator()`](emissionsCalculatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emissionsCalculatorReturn {
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
            impl ::core::convert::From<emissionsCalculatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: emissionsCalculatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for emissionsCalculatorCall {
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
            impl ::core::convert::From<emissionsCalculatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: emissionsCalculatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for emissionsCalculatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emissionsCalculatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emissionsCalculator()";
            const SELECTOR: [u8; 4] = [35u8, 18u8, 215u8, 215u8];
            #[inline]
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
                        let r: emissionsCalculatorReturn = r.into();
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
                        let r: emissionsCalculatorReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `emissionsScheduler()` and selector `0x1f001db4`.
```solidity
function emissionsScheduler() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emissionsSchedulerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`emissionsScheduler()`](emissionsSchedulerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emissionsSchedulerReturn {
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
            impl ::core::convert::From<emissionsSchedulerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: emissionsSchedulerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for emissionsSchedulerCall {
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
            impl ::core::convert::From<emissionsSchedulerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: emissionsSchedulerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for emissionsSchedulerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emissionsSchedulerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emissionsScheduler()";
            const SELECTOR: [u8; 4] = [31u8, 0u8, 29u8, 180u8];
            #[inline]
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
                        let r: emissionsSchedulerReturn = r.into();
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
                        let r: emissionsSchedulerReturn = r.into();
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
    /**Function with signature `expectedMintAmount_ChangeFactor098(uint256)` and selector `0x037d52d6`.
```solidity
function expectedMintAmount_ChangeFactor098(uint256 epoch) external pure returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct expectedMintAmount_ChangeFactor098Call {
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`expectedMintAmount_ChangeFactor098(uint256)`](expectedMintAmount_ChangeFactor098Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct expectedMintAmount_ChangeFactor098Return {
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
            impl ::core::convert::From<expectedMintAmount_ChangeFactor098Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: expectedMintAmount_ChangeFactor098Call) -> Self {
                    (value.epoch,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for expectedMintAmount_ChangeFactor098Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { epoch: tuple.0 }
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
            impl ::core::convert::From<expectedMintAmount_ChangeFactor098Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: expectedMintAmount_ChangeFactor098Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for expectedMintAmount_ChangeFactor098Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for expectedMintAmount_ChangeFactor098Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "expectedMintAmount_ChangeFactor098(uint256)";
            const SELECTOR: [u8; 4] = [3u8, 125u8, 82u8, 214u8];
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
                        let r: expectedMintAmount_ChangeFactor098Return = r.into();
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
                        let r: expectedMintAmount_ChangeFactor098Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `expectedMintAmount_ChangeFactor101(uint256)` and selector `0x2ee8823d`.
```solidity
function expectedMintAmount_ChangeFactor101(uint256 epoch) external pure returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct expectedMintAmount_ChangeFactor101Call {
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`expectedMintAmount_ChangeFactor101(uint256)`](expectedMintAmount_ChangeFactor101Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct expectedMintAmount_ChangeFactor101Return {
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
            impl ::core::convert::From<expectedMintAmount_ChangeFactor101Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: expectedMintAmount_ChangeFactor101Call) -> Self {
                    (value.epoch,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for expectedMintAmount_ChangeFactor101Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { epoch: tuple.0 }
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
            impl ::core::convert::From<expectedMintAmount_ChangeFactor101Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: expectedMintAmount_ChangeFactor101Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for expectedMintAmount_ChangeFactor101Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for expectedMintAmount_ChangeFactor101Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "expectedMintAmount_ChangeFactor101(uint256)";
            const SELECTOR: [u8; 4] = [46u8, 232u8, 130u8, 61u8];
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
                        let r: expectedMintAmount_ChangeFactor101Return = r.into();
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
                        let r: expectedMintAmount_ChangeFactor101Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `expectedMintAmount_ChangeFactorMultiple(uint256)` and selector `0x4c79cca6`.
```solidity
function expectedMintAmount_ChangeFactorMultiple(uint256 epoch) external pure returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct expectedMintAmount_ChangeFactorMultipleCall {
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`expectedMintAmount_ChangeFactorMultiple(uint256)`](expectedMintAmount_ChangeFactorMultipleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct expectedMintAmount_ChangeFactorMultipleReturn {
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
            impl ::core::convert::From<expectedMintAmount_ChangeFactorMultipleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: expectedMintAmount_ChangeFactorMultipleCall) -> Self {
                    (value.epoch,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for expectedMintAmount_ChangeFactorMultipleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { epoch: tuple.0 }
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
            impl ::core::convert::From<expectedMintAmount_ChangeFactorMultipleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: expectedMintAmount_ChangeFactorMultipleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for expectedMintAmount_ChangeFactorMultipleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for expectedMintAmount_ChangeFactorMultipleCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "expectedMintAmount_ChangeFactorMultiple(uint256)";
            const SELECTOR: [u8; 4] = [76u8, 121u8, 204u8, 166u8];
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
                        let r: expectedMintAmount_ChangeFactorMultipleReturn = r.into();
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
                        let r: expectedMintAmount_ChangeFactorMultipleReturn = r.into();
                        r._0
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
    /**Function with signature `round(uint256)` and selector `0x34d5f37b`.
```solidity
function round(uint256 _weiAmount) external pure returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct roundCall {
        #[allow(missing_docs)]
        pub _weiAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`round(uint256)`](roundCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct roundReturn {
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
            impl ::core::convert::From<roundCall> for UnderlyingRustTuple<'_> {
                fn from(value: roundCall) -> Self {
                    (value._weiAmount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for roundCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _weiAmount: tuple.0 }
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
            impl ::core::convert::From<roundReturn> for UnderlyingRustTuple<'_> {
                fn from(value: roundReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for roundReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for roundCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "round(uint256)";
            const SELECTOR: [u8; 4] = [52u8, 213u8, 243u8, 123u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._weiAmount),
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
                        let r: roundReturn = r.into();
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
                        let r: roundReturn = r.into();
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
    /**Function with signature `startEpoch()` and selector `0xa2c8b177`.
```solidity
function startEpoch() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startEpochCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`startEpoch()`](startEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startEpochReturn {
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
            impl ::core::convert::From<startEpochCall> for UnderlyingRustTuple<'_> {
                fn from(value: startEpochCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startEpochCall {
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
            impl ::core::convert::From<startEpochReturn> for UnderlyingRustTuple<'_> {
                fn from(value: startEpochReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startEpochCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startEpoch()";
            const SELECTOR: [u8; 4] = [162u8, 200u8, 177u8, 119u8];
            #[inline]
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
                        let r: startEpochReturn = r.into();
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
                        let r: startEpochReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `syndTokenAdmin()` and selector `0x078c665a`.
```solidity
function syndTokenAdmin() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct syndTokenAdminCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`syndTokenAdmin()`](syndTokenAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct syndTokenAdminReturn {
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
            impl ::core::convert::From<syndTokenAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: syndTokenAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for syndTokenAdminCall {
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
            impl ::core::convert::From<syndTokenAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: syndTokenAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for syndTokenAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for syndTokenAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "syndTokenAdmin()";
            const SELECTOR: [u8; 4] = [7u8, 140u8, 102u8, 90u8];
            #[inline]
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
                        let r: syndTokenAdminReturn = r.into();
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
                        let r: syndTokenAdminReturn = r.into();
                        r._0
                    })
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
    /**Function with signature `test_emissions_ChangeFactor098()` and selector `0x5184c5e7`.
```solidity
function test_emissions_ChangeFactor098() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_emissions_ChangeFactor098Call;
    ///Container type for the return parameters of the [`test_emissions_ChangeFactor098()`](test_emissions_ChangeFactor098Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_emissions_ChangeFactor098Return {}
    #[allow(
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
            impl ::core::convert::From<test_emissions_ChangeFactor098Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_emissions_ChangeFactor098Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_emissions_ChangeFactor098Call {
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
            impl ::core::convert::From<test_emissions_ChangeFactor098Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_emissions_ChangeFactor098Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_emissions_ChangeFactor098Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_emissions_ChangeFactor098Return {
            fn _tokenize(
                &self,
            ) -> <test_emissions_ChangeFactor098Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_emissions_ChangeFactor098Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_emissions_ChangeFactor098Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_emissions_ChangeFactor098()";
            const SELECTOR: [u8; 4] = [81u8, 132u8, 197u8, 231u8];
            #[inline]
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
                test_emissions_ChangeFactor098Return::_tokenize(ret)
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
    /**Function with signature `test_emissions_ChangeFactor101()` and selector `0xbe22cc59`.
```solidity
function test_emissions_ChangeFactor101() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_emissions_ChangeFactor101Call;
    ///Container type for the return parameters of the [`test_emissions_ChangeFactor101()`](test_emissions_ChangeFactor101Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_emissions_ChangeFactor101Return {}
    #[allow(
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
            impl ::core::convert::From<test_emissions_ChangeFactor101Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_emissions_ChangeFactor101Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_emissions_ChangeFactor101Call {
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
            impl ::core::convert::From<test_emissions_ChangeFactor101Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_emissions_ChangeFactor101Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_emissions_ChangeFactor101Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_emissions_ChangeFactor101Return {
            fn _tokenize(
                &self,
            ) -> <test_emissions_ChangeFactor101Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_emissions_ChangeFactor101Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_emissions_ChangeFactor101Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_emissions_ChangeFactor101()";
            const SELECTOR: [u8; 4] = [190u8, 34u8, 204u8, 89u8];
            #[inline]
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
                test_emissions_ChangeFactor101Return::_tokenize(ret)
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
    /**Function with signature `test_emissions_ChangeFactorFlat()` and selector `0x7e041c04`.
```solidity
function test_emissions_ChangeFactorFlat() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_emissions_ChangeFactorFlatCall;
    ///Container type for the return parameters of the [`test_emissions_ChangeFactorFlat()`](test_emissions_ChangeFactorFlatCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_emissions_ChangeFactorFlatReturn {}
    #[allow(
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
            impl ::core::convert::From<test_emissions_ChangeFactorFlatCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_emissions_ChangeFactorFlatCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_emissions_ChangeFactorFlatCall {
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
            impl ::core::convert::From<test_emissions_ChangeFactorFlatReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_emissions_ChangeFactorFlatReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_emissions_ChangeFactorFlatReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_emissions_ChangeFactorFlatReturn {
            fn _tokenize(
                &self,
            ) -> <test_emissions_ChangeFactorFlatCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_emissions_ChangeFactorFlatCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_emissions_ChangeFactorFlatReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_emissions_ChangeFactorFlat()";
            const SELECTOR: [u8; 4] = [126u8, 4u8, 28u8, 4u8];
            #[inline]
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
                test_emissions_ChangeFactorFlatReturn::_tokenize(ret)
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
    /**Function with signature `test_emissions_ChangeFactorMultiple()` and selector `0x1b80a4e5`.
```solidity
function test_emissions_ChangeFactorMultiple() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_emissions_ChangeFactorMultipleCall;
    ///Container type for the return parameters of the [`test_emissions_ChangeFactorMultiple()`](test_emissions_ChangeFactorMultipleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_emissions_ChangeFactorMultipleReturn {}
    #[allow(
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
            impl ::core::convert::From<test_emissions_ChangeFactorMultipleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_emissions_ChangeFactorMultipleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_emissions_ChangeFactorMultipleCall {
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
            impl ::core::convert::From<test_emissions_ChangeFactorMultipleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_emissions_ChangeFactorMultipleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_emissions_ChangeFactorMultipleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_emissions_ChangeFactorMultipleReturn {
            fn _tokenize(
                &self,
            ) -> <test_emissions_ChangeFactorMultipleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_emissions_ChangeFactorMultipleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_emissions_ChangeFactorMultipleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_emissions_ChangeFactorMultiple()";
            const SELECTOR: [u8; 4] = [27u8, 128u8, 164u8, 229u8];
            #[inline]
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
                test_emissions_ChangeFactorMultipleReturn::_tokenize(ret)
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
    ///Container for all the [`EmissionsForkTest`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum EmissionsForkTestCalls {
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        acceptedDiff(acceptedDiffCall),
        #[allow(missing_docs)]
        emissionsCalculator(emissionsCalculatorCall),
        #[allow(missing_docs)]
        emissionsScheduler(emissionsSchedulerCall),
        #[allow(missing_docs)]
        excludeArtifacts(excludeArtifactsCall),
        #[allow(missing_docs)]
        excludeContracts(excludeContractsCall),
        #[allow(missing_docs)]
        excludeSelectors(excludeSelectorsCall),
        #[allow(missing_docs)]
        excludeSenders(excludeSendersCall),
        #[allow(missing_docs)]
        expectedMintAmount_ChangeFactor098(expectedMintAmount_ChangeFactor098Call),
        #[allow(missing_docs)]
        expectedMintAmount_ChangeFactor101(expectedMintAmount_ChangeFactor101Call),
        #[allow(missing_docs)]
        expectedMintAmount_ChangeFactorMultiple(
            expectedMintAmount_ChangeFactorMultipleCall,
        ),
        #[allow(missing_docs)]
        failed(failedCall),
        #[allow(missing_docs)]
        round(roundCall),
        #[allow(missing_docs)]
        setUp(setUpCall),
        #[allow(missing_docs)]
        startEpoch(startEpochCall),
        #[allow(missing_docs)]
        syndTokenAdmin(syndTokenAdminCall),
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
        test_emissions_ChangeFactor098(test_emissions_ChangeFactor098Call),
        #[allow(missing_docs)]
        test_emissions_ChangeFactor101(test_emissions_ChangeFactor101Call),
        #[allow(missing_docs)]
        test_emissions_ChangeFactorFlat(test_emissions_ChangeFactorFlatCall),
        #[allow(missing_docs)]
        test_emissions_ChangeFactorMultiple(test_emissions_ChangeFactorMultipleCall),
    }
    #[automatically_derived]
    impl EmissionsForkTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 125u8, 82u8, 214u8],
            [6u8, 233u8, 158u8, 133u8],
            [7u8, 140u8, 102u8, 90u8],
            [10u8, 146u8, 84u8, 228u8],
            [27u8, 128u8, 164u8, 229u8],
            [30u8, 215u8, 131u8, 28u8],
            [31u8, 0u8, 29u8, 180u8],
            [35u8, 18u8, 215u8, 215u8],
            [42u8, 222u8, 56u8, 128u8],
            [46u8, 232u8, 130u8, 61u8],
            [52u8, 213u8, 243u8, 123u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [76u8, 121u8, 204u8, 166u8],
            [81u8, 132u8, 197u8, 231u8],
            [102u8, 217u8, 169u8, 160u8],
            [126u8, 4u8, 28u8, 4u8],
            [133u8, 34u8, 108u8, 129u8],
            [145u8, 106u8, 23u8, 198u8],
            [162u8, 200u8, 177u8, 119u8],
            [176u8, 70u8, 79u8, 220u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [190u8, 34u8, 204u8, 89u8],
            [226u8, 12u8, 159u8, 113u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EmissionsForkTestCalls {
        const NAME: &'static str = "EmissionsForkTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 26usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::acceptedDiff(_) => {
                    <acceptedDiffCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::emissionsCalculator(_) => {
                    <emissionsCalculatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::emissionsScheduler(_) => {
                    <emissionsSchedulerCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::expectedMintAmount_ChangeFactor098(_) => {
                    <expectedMintAmount_ChangeFactor098Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::expectedMintAmount_ChangeFactor101(_) => {
                    <expectedMintAmount_ChangeFactor101Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::expectedMintAmount_ChangeFactorMultiple(_) => {
                    <expectedMintAmount_ChangeFactorMultipleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::round(_) => <roundCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setUp(_) => <setUpCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::startEpoch(_) => {
                    <startEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::syndTokenAdmin(_) => {
                    <syndTokenAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                Self::test_emissions_ChangeFactor098(_) => {
                    <test_emissions_ChangeFactor098Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_emissions_ChangeFactor101(_) => {
                    <test_emissions_ChangeFactor101Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_emissions_ChangeFactorFlat(_) => {
                    <test_emissions_ChangeFactorFlatCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_emissions_ChangeFactorMultiple(_) => {
                    <test_emissions_ChangeFactorMultipleCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<EmissionsForkTestCalls>] = &[
                {
                    fn expectedMintAmount_ChangeFactor098(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <expectedMintAmount_ChangeFactor098Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsForkTestCalls::expectedMintAmount_ChangeFactor098,
                            )
                    }
                    expectedMintAmount_ChangeFactor098
                },
                {
                    fn acceptedDiff(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <acceptedDiffCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::acceptedDiff)
                    }
                    acceptedDiff
                },
                {
                    fn syndTokenAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <syndTokenAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::syndTokenAdmin)
                    }
                    syndTokenAdmin
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EmissionsForkTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn test_emissions_ChangeFactorMultiple(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <test_emissions_ChangeFactorMultipleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsForkTestCalls::test_emissions_ChangeFactorMultiple,
                            )
                    }
                    test_emissions_ChangeFactorMultiple
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn emissionsScheduler(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <emissionsSchedulerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::emissionsScheduler)
                    }
                    emissionsScheduler
                },
                {
                    fn emissionsCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <emissionsCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::emissionsCalculator)
                    }
                    emissionsCalculator
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn expectedMintAmount_ChangeFactor101(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <expectedMintAmount_ChangeFactor101Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsForkTestCalls::expectedMintAmount_ChangeFactor101,
                            )
                    }
                    expectedMintAmount_ChangeFactor101
                },
                {
                    fn round(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <roundCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EmissionsForkTestCalls::round)
                    }
                    round
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn expectedMintAmount_ChangeFactorMultiple(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <expectedMintAmount_ChangeFactorMultipleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsForkTestCalls::expectedMintAmount_ChangeFactorMultiple,
                            )
                    }
                    expectedMintAmount_ChangeFactorMultiple
                },
                {
                    fn test_emissions_ChangeFactor098(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <test_emissions_ChangeFactor098Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::test_emissions_ChangeFactor098)
                    }
                    test_emissions_ChangeFactor098
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn test_emissions_ChangeFactorFlat(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <test_emissions_ChangeFactorFlatCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::test_emissions_ChangeFactorFlat)
                    }
                    test_emissions_ChangeFactorFlat
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn startEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <startEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::startEpoch)
                    }
                    startEpoch
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EmissionsForkTestCalls::failed)
                    }
                    failed
                },
                {
                    fn test_emissions_ChangeFactor101(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <test_emissions_ChangeFactor101Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::test_emissions_ChangeFactor101)
                    }
                    test_emissions_ChangeFactor101
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsForkTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EmissionsForkTestCalls::IS_TEST)
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
            ) -> alloy_sol_types::Result<EmissionsForkTestCalls>] = &[
                {
                    fn expectedMintAmount_ChangeFactor098(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <expectedMintAmount_ChangeFactor098Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsForkTestCalls::expectedMintAmount_ChangeFactor098,
                            )
                    }
                    expectedMintAmount_ChangeFactor098
                },
                {
                    fn acceptedDiff(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <acceptedDiffCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::acceptedDiff)
                    }
                    acceptedDiff
                },
                {
                    fn syndTokenAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <syndTokenAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::syndTokenAdmin)
                    }
                    syndTokenAdmin
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn test_emissions_ChangeFactorMultiple(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <test_emissions_ChangeFactorMultipleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsForkTestCalls::test_emissions_ChangeFactorMultiple,
                            )
                    }
                    test_emissions_ChangeFactorMultiple
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn emissionsScheduler(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <emissionsSchedulerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::emissionsScheduler)
                    }
                    emissionsScheduler
                },
                {
                    fn emissionsCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <emissionsCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::emissionsCalculator)
                    }
                    emissionsCalculator
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn expectedMintAmount_ChangeFactor101(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <expectedMintAmount_ChangeFactor101Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsForkTestCalls::expectedMintAmount_ChangeFactor101,
                            )
                    }
                    expectedMintAmount_ChangeFactor101
                },
                {
                    fn round(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <roundCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::round)
                    }
                    round
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn expectedMintAmount_ChangeFactorMultiple(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <expectedMintAmount_ChangeFactorMultipleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsForkTestCalls::expectedMintAmount_ChangeFactorMultiple,
                            )
                    }
                    expectedMintAmount_ChangeFactorMultiple
                },
                {
                    fn test_emissions_ChangeFactor098(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <test_emissions_ChangeFactor098Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::test_emissions_ChangeFactor098)
                    }
                    test_emissions_ChangeFactor098
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn test_emissions_ChangeFactorFlat(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <test_emissions_ChangeFactorFlatCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::test_emissions_ChangeFactorFlat)
                    }
                    test_emissions_ChangeFactorFlat
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn startEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <startEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::startEpoch)
                    }
                    startEpoch
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::failed)
                    }
                    failed
                },
                {
                    fn test_emissions_ChangeFactor101(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <test_emissions_ChangeFactor101Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::test_emissions_ChangeFactor101)
                    }
                    test_emissions_ChangeFactor101
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsForkTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsForkTestCalls::IS_TEST)
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
                Self::acceptedDiff(inner) => {
                    <acceptedDiffCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::emissionsCalculator(inner) => {
                    <emissionsCalculatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::emissionsScheduler(inner) => {
                    <emissionsSchedulerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::expectedMintAmount_ChangeFactor098(inner) => {
                    <expectedMintAmount_ChangeFactor098Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::expectedMintAmount_ChangeFactor101(inner) => {
                    <expectedMintAmount_ChangeFactor101Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::expectedMintAmount_ChangeFactorMultiple(inner) => {
                    <expectedMintAmount_ChangeFactorMultipleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::round(inner) => {
                    <roundCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::startEpoch(inner) => {
                    <startEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::syndTokenAdmin(inner) => {
                    <syndTokenAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::test_emissions_ChangeFactor098(inner) => {
                    <test_emissions_ChangeFactor098Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_emissions_ChangeFactor101(inner) => {
                    <test_emissions_ChangeFactor101Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_emissions_ChangeFactorFlat(inner) => {
                    <test_emissions_ChangeFactorFlatCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_emissions_ChangeFactorMultiple(inner) => {
                    <test_emissions_ChangeFactorMultipleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::acceptedDiff(inner) => {
                    <acceptedDiffCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emissionsCalculator(inner) => {
                    <emissionsCalculatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emissionsScheduler(inner) => {
                    <emissionsSchedulerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::expectedMintAmount_ChangeFactor098(inner) => {
                    <expectedMintAmount_ChangeFactor098Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::expectedMintAmount_ChangeFactor101(inner) => {
                    <expectedMintAmount_ChangeFactor101Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::expectedMintAmount_ChangeFactorMultiple(inner) => {
                    <expectedMintAmount_ChangeFactorMultipleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::round(inner) => {
                    <roundCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::startEpoch(inner) => {
                    <startEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::syndTokenAdmin(inner) => {
                    <syndTokenAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::test_emissions_ChangeFactor098(inner) => {
                    <test_emissions_ChangeFactor098Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_emissions_ChangeFactor101(inner) => {
                    <test_emissions_ChangeFactor101Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_emissions_ChangeFactorFlat(inner) => {
                    <test_emissions_ChangeFactorFlatCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_emissions_ChangeFactorMultiple(inner) => {
                    <test_emissions_ChangeFactorMultipleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`EmissionsForkTest`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum EmissionsForkTestEvents {
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
    impl EmissionsForkTestEvents {
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
    impl alloy_sol_types::SolEventInterface for EmissionsForkTestEvents {
        const NAME: &'static str = "EmissionsForkTestEvents";
        const COUNT: usize = 22usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
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
    impl alloy_sol_types::private::IntoLogData for EmissionsForkTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
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
    /**Creates a new wrapper around an on-chain [`EmissionsForkTest`](self) contract instance.

See the [wrapper's documentation](`EmissionsForkTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EmissionsForkTestInstance<P, N> {
        EmissionsForkTestInstance::<P, N>::new(address, provider)
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
        Output = alloy_contract::Result<EmissionsForkTestInstance<P, N>>,
    > {
        EmissionsForkTestInstance::<P, N>::deploy(provider)
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
        EmissionsForkTestInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`EmissionsForkTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EmissionsForkTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EmissionsForkTestInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for EmissionsForkTestInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EmissionsForkTestInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > EmissionsForkTestInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`EmissionsForkTest`](self) contract instance.

See the [wrapper's documentation](`EmissionsForkTestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<EmissionsForkTestInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> EmissionsForkTestInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EmissionsForkTestInstance<P, N> {
            EmissionsForkTestInstance {
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
    > EmissionsForkTestInstance<P, N> {
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
        ///Creates a new call builder for the [`acceptedDiff`] function.
        pub fn acceptedDiff(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, acceptedDiffCall, N> {
            self.call_builder(&acceptedDiffCall)
        }
        ///Creates a new call builder for the [`emissionsCalculator`] function.
        pub fn emissionsCalculator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, emissionsCalculatorCall, N> {
            self.call_builder(&emissionsCalculatorCall)
        }
        ///Creates a new call builder for the [`emissionsScheduler`] function.
        pub fn emissionsScheduler(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, emissionsSchedulerCall, N> {
            self.call_builder(&emissionsSchedulerCall)
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
        ///Creates a new call builder for the [`expectedMintAmount_ChangeFactor098`] function.
        pub fn expectedMintAmount_ChangeFactor098(
            &self,
            epoch: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            expectedMintAmount_ChangeFactor098Call,
            N,
        > {
            self.call_builder(
                &expectedMintAmount_ChangeFactor098Call {
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`expectedMintAmount_ChangeFactor101`] function.
        pub fn expectedMintAmount_ChangeFactor101(
            &self,
            epoch: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            expectedMintAmount_ChangeFactor101Call,
            N,
        > {
            self.call_builder(
                &expectedMintAmount_ChangeFactor101Call {
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`expectedMintAmount_ChangeFactorMultiple`] function.
        pub fn expectedMintAmount_ChangeFactorMultiple(
            &self,
            epoch: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            expectedMintAmount_ChangeFactorMultipleCall,
            N,
        > {
            self.call_builder(
                &expectedMintAmount_ChangeFactorMultipleCall {
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<&P, failedCall, N> {
            self.call_builder(&failedCall)
        }
        ///Creates a new call builder for the [`round`] function.
        pub fn round(
            &self,
            _weiAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, roundCall, N> {
            self.call_builder(&roundCall { _weiAmount })
        }
        ///Creates a new call builder for the [`setUp`] function.
        pub fn setUp(&self) -> alloy_contract::SolCallBuilder<&P, setUpCall, N> {
            self.call_builder(&setUpCall)
        }
        ///Creates a new call builder for the [`startEpoch`] function.
        pub fn startEpoch(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startEpochCall, N> {
            self.call_builder(&startEpochCall)
        }
        ///Creates a new call builder for the [`syndTokenAdmin`] function.
        pub fn syndTokenAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, syndTokenAdminCall, N> {
            self.call_builder(&syndTokenAdminCall)
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
        ///Creates a new call builder for the [`test_emissions_ChangeFactor098`] function.
        pub fn test_emissions_ChangeFactor098(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_emissions_ChangeFactor098Call, N> {
            self.call_builder(&test_emissions_ChangeFactor098Call)
        }
        ///Creates a new call builder for the [`test_emissions_ChangeFactor101`] function.
        pub fn test_emissions_ChangeFactor101(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_emissions_ChangeFactor101Call, N> {
            self.call_builder(&test_emissions_ChangeFactor101Call)
        }
        ///Creates a new call builder for the [`test_emissions_ChangeFactorFlat`] function.
        pub fn test_emissions_ChangeFactorFlat(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_emissions_ChangeFactorFlatCall, N> {
            self.call_builder(&test_emissions_ChangeFactorFlatCall)
        }
        ///Creates a new call builder for the [`test_emissions_ChangeFactorMultiple`] function.
        pub fn test_emissions_ChangeFactorMultiple(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_emissions_ChangeFactorMultipleCall,
            N,
        > {
            self.call_builder(&test_emissions_ChangeFactorMultipleCall)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > EmissionsForkTestInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
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
