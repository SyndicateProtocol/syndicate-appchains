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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        __provider: P,
    ) -> StdInvariantInstance<P, N> {
        StdInvariantInstance::<P, N>::new(address, __provider)
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
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
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

interface AirdropScriptTest {
    event SlotFound(address who, bytes4 fsig, bytes32 keysHash, uint256 slot);
    event WARNING_UninitedSlot(address who, uint256 slot);
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

    function AIRDROP_MANAGER_ROLE() external view returns (bytes32);
    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function IS_TEST() external view returns (bool);
    function airdrop() external view returns (address);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function setUp() external;
    function syndToken() external view returns (address);
    function syndTokenAccessControl() external view returns (address);
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function test_DeployAirdrop() external view;
    function test_ExecuteAirdrop() external;
    function test_ExecuteAirdropBatches() external;
    function test_ExecuteAirdropWithArrayLengthMismatch() external;
    function test_ExecuteAirdropWithInsufficientAllowance() external;
    function test_TokenInformation() external view;
    function tokenHolder() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
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
    "name": "airdrop",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract Airdrop"
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
    "name": "syndToken",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IERC20"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "syndTokenAccessControl",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract AccessControl"
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
    "name": "test_DeployAirdrop",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "test_ExecuteAirdrop",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_ExecuteAirdropBatches",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_ExecuteAirdropWithArrayLengthMismatch",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_ExecuteAirdropWithInsufficientAllowance",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_TokenInformation",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "tokenHolder",
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
    "name": "SlotFound",
    "inputs": [
      {
        "name": "who",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "fsig",
        "type": "bytes4",
        "indexed": false,
        "internalType": "bytes4"
      },
      {
        "name": "keysHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "slot",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "WARNING_UninitedSlot",
    "inputs": [
      {
        "name": "who",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "slot",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
pub mod AirdropScriptTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60808060405234602f57600160ff19600c541617600c55600160ff19601f541617601f55615eaa90816100348239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f905f3560e01c9081630a9254e4146124aa575080631b7986d2146121345780631ed7831c146120b65780632ade388014611eba5780633884d63514611e905780633e5e3c2314611e125780633f7286f414611d94578063420a83e714611d6d57806359c620c614611aa05780636213821d14611a7a57806366d9a9a01461193557806381a8de7e1461184257806385226c81146117b057806385a468f714610d915780638a54252114610d56578063916a17c614610cac57806394a2a5df14610926578063a217fddf1461090a578063b0464fdc14610860578063b5508aa9146107c7578063ba414fa6146107a2578063bff8600f14610213578063e20c9f711461017d578063e52a2f1f146101565763fa7626d414610131575f80fd5b34610153578060031936011261015357602060ff601f54166040519015158152f35b80fd5b503461015357806003193601126101535760206001600160a01b0360215416604051908152f35b503461015357806003193601126101535760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106101f4576101f0856101dc81870382613589565b60405191829160208352602083019061338f565b0390f35b82546001600160a01b03168452602090930192600192830192016101c5565b503461015357806003193601126101535761022c614b1c565b8051600381019081811161078e5760020190811161077a576003900490826001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077657604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105645761075d575b5060208054601f546040517f095ea7b30000000000000000000000000000000000000000000000000000000081526001600160a01b0360089290921c821660048201526901b1ae4d6e2ef50000006024820152938492604492849291165af1801561075257610725575b50825b82811061056f5783737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015357806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105645761054f575b50506103ae614b1c565b6020546001600160a01b0316825b82518110156104e5576001600160a01b036103d782856136a2565b51511690604051917f70a082310000000000000000000000000000000000000000000000000000000083526004830152602082602481865afa9182156104da5785926104a1575b5061049b600192602061043184886136a2565b51015111156060906104466040519283613589565b602d82527f526563697069656e742073686f756c642068617665207265636569766564206160208301527f697264726f7020616d6f756e74000000000000000000000000000000000000006040830152614ab8565b016103bc565b91506020823d82116104d2575b816104bb60209383613589565b810103126104ce5790519061049b61041e565b5f80fd5b3d91506104ae565b6040513d87823e3d90fd5b8361054c6040516104f7606082613589565b602481527f42617463682061697264726f7020636f6d706c6574656420737563636573736660208201527f756c6c79000000000000000000000000000000000000000000000000000000006040820152614895565b80f35b8161055991613589565b6101535780826103a4565b6040513d84823e3d90fd5b60038102908082046003148115171561071157600382018083116106fd576105a5838288939087518091116106f5575b50613abc565b926105af84613610565b6105b885613610565b91839084905b87821061068e5750506001600160a01b03601f5460081c16803b1561068a5761061a93858094604051968795869485937f82947abe000000000000000000000000000000000000000000000000000000008552600485016136ce565b03925af1801561056457610675575b5060019261066f9150604061064081519182613589565b601f81527f4261746368206578656375746564207769746820726563697069656e74733a0060208201526149c5565b01610332565b8161067f91613589565b61068a57845f610629565b8480fd5b90916106ed6001916001600160a01b036106b16106ab87876135d7565b8d6136a2565b5151166106be86886136a2565b5260206106ce6106ab87876135d7565b5101516106db86896136a2565b526106e685886136a2565b51906135d7565b9201906105be565b90505f61059f565b602486634e487b7160e01b81526011600452fd5b602485634e487b7160e01b81526011600452fd5b6107469060203d60201161074b575b61073e8183613589565b8101906136b6565b61032f565b503d610734565b6040513d86823e3d90fd5b8161076791613589565b61077257825f6102c5565b8280fd5b5080fd5b602483634e487b7160e01b81526011600452fd5b602484634e487b7160e01b81526011600452fd5b503461015357806003193601126101535760206107bd613ac9565b6040519015158152f35b50346101535780600319360112610153576019546107e4816135f8565b916107f26040519384613589565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b83831061083457604051806101f08782613445565b6001602081926040516108528161084b8189613776565b0382613589565b81520192019201919061081f565b5034610153578060031936011261015357601c5461087d816135f8565b9161088b6040519384613589565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b8383106108cd57604051806101f087826134c2565b600260206001926040516108e081613559565b6001600160a01b0386541681526108f8858701613816565b838201528152019201920191906108b8565b5034610153578060031936011261015357602090604051908152f35b503461015357806003193601126101535761093f614b1c565b90604080519261094f8285613589565b60018452602061099d601f198401928336848901378451936109718686613589565b6001855236848601376001600160a01b0361098b82613641565b51511661099788613641565b52613641565b5101516109a982613641565b526001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ca8578251907f06447d560000000000000000000000000000000000000000000000000000000082526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c9b57908491610c86575b50506001600160a01b0360205416936001600160a01b03601f5460081c1694610a4f83613641565b51955f1987019687116106fd5784517f095ea7b30000000000000000000000000000000000000000000000000000000081526001600160a01b039190911660048201526024810196909652939485946020908290604490829089905af18015610c5f57610c69575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c5a5782517ff4844814000000000000000000000000000000000000000000000000000000008152848160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c5f57908591610c45575b50506001600160a01b03601f5460081c16610b3d83613641565b5190803b15610c4157610b82938680948751968795869485937f82947abe000000000000000000000000000000000000000000000000000000008552600485016136ce565b03925af18015610c3557908391610c20575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c1d578051907f90c5013b000000000000000000000000000000000000000000000000000000008252828260048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610c145750610c035750f35b81610c0d91613589565b6101535780f35b513d84823e3d90fd5b50fd5b81610c2a91613589565b610c1d57815f610b94565b505051903d90823e3d90fd5b8580fd5b81610c4f91613589565b610c5a57835f610b23565b505050fd5b84513d87823e3d90fd5b610c819060203d60201161074b5761073e8183613589565b610ab7565b81610c9091613589565b61077257825f610a27565b50505051903d90823e3d90fd5b8380fd5b5034610153578060031936011261015357601d54610cc9816135f8565b91610cd76040519384613589565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310610d1957604051806101f087826134c2565b60026020600192604051610d2c81613559565b6001600160a01b038654168152610d44858701613816565b83820152815201920192019190610d04565b503461015357806003193601126101535760206040517f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a6748152f35b5034610153578060031936011261015357610daa614b1c565b90610db58251613610565b610dbf8351613610565b8291825b8551841015610e1e57610e166001916001600160a01b03610de4878a6136a2565b515116610df187866136a2565b526020610dfe878a6136a2565b510151610e0b87876136a2565b526106e686866136a2565b930192610dc3565b90919250610e2c8251613610565b9184916001600160a01b0360205416925b8251811015610eef576001600160a01b03610e5882856136a2565b511690604051917f70a082310000000000000000000000000000000000000000000000000000000083526004830152602082602481885afa8015610ee4578890610eb2575b60019250610eab82886136a2565b5201610e3d565b506020823d8211610edc575b81610ecb60209383613589565b810103126104ce5760019151610e9d565b3d9150610ebe565b6040513d8a823e3d90fd5b509391926024959360206001600160a01b036022541691604051988980927f70a082310000000000000000000000000000000000000000000000000000000082528560048301525afa9687156104da57859761177c575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561068a57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152848160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104da57908591611767575b505060208054601f546040517f095ea7b30000000000000000000000000000000000000000000000000000000081526001600160a01b0360089290921c821660048201526024810189905292918391168188816044810103925af180156104da57906110a093929161174a575b506001600160a01b036020541660206001600160a01b03602254166001600160a01b03601f5460081c169283916040518098819482937fdd62ed3e000000000000000000000000000000000000000000000000000000008452600484019092916001600160a01b0360209181604085019616845216910152565b03915afa9384156116e1578694611716575b5061111d60609488604051916110c88884613589565b602383527f416c6c6f77616e63652073686f756c6420657175616c20746f74616c20616d6f60208401527f756e7400000000000000000000000000000000000000000000000000000000006040840152614a21565b803b15610c41578560405180927f82947abe00000000000000000000000000000000000000000000000000000000825281838161115f8d8c8b600485016136ce565b03925af180156116e157908691611701575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561068a576040517f90c5013b000000000000000000000000000000000000000000000000000000008152858160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156116e1579086916116ec575b5096956001600160a01b0360205416975b83518110156113625761121161120682856136a2565b516106e683896136a2565b908960206001600160a01b0361122784896136a2565b51166024604051809481937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa908115611357578991611325575b506040516001936112d49261127f8a84613589565b603383527f526563697069656e742062616c616e63652073686f756c6420696e637265617360208401527f652062792061697264726f7020616d6f756e74000000000000000000000000006040840152614a21565b61131f6112e182896136a2565b5160406112f081519182613589565b601881527f526563697069656e742072656365697665642053594e443a000000000000000060208201526149c5565b016111f0565b90506020813d821161134f575b8161133f60209383613589565b810103126104ce5751600161126a565b3d9150611332565b6040513d8b823e3d90fd5b86848382888c8e6001600160a01b0360225416916040517f70a08231000000000000000000000000000000000000000000000000000000008152836004820152602081602481865afa9182156116e157879187936116a2575b5061149094926113d16020959361143393613abc565b604051916113df8984613589565b603483527f546f6b656e20686f6c6465722062616c616e63652073686f756c642064656372878401527f6561736520627920746f74616c20616d6f756e740000000000000000000000006040840152614a21565b6001600160a01b03601f5460081c16916040518095819482937fdd62ed3e000000000000000000000000000000000000000000000000000000008452600484019092916001600160a01b0360209181604085019616845216910152565b03915afa908115611697578391611662575b50604051916114b18184613589565b602683527f416c6c6f77616e63652073686f756c64206265207a65726f206166746572206160208401527f697264726f7000000000000000000000000000000000000000000000000000006040840152737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ca857839161156660405194859384937f88b44c850000000000000000000000000000000000000000000000000000000085526004850152856024850152604484015260648301906133cb565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa80156105645761164d575b509161054c92506116106040926115d584516115a68682613589565b601e81527f41697264726f70206578656375746564207375636365737366756c6c793a00006020820152614895565b83516115e18582613589565b601281527f546f74616c2064697374726962757465643a000000000000000000000000000060208201526149c5565b519061161e81519182613589565b600b81527f526563697069656e74733a00000000000000000000000000000000000000000060208201526149c5565b8161165791613589565b61077257828461158a565b9250506020823d60201161168f575b8161167e60209383613589565b810103126104ce57849151866114a2565b3d9150611671565b6040513d85823e3d90fd5b949250929550506020833d6020116116d9575b816116c260209383613589565b810103126104ce57915187949186906114336113bb565b3d91506116b5565b6040513d88823e3d90fd5b816116f691613589565b61068a5784886111df565b8161170b91613589565b61068a578488611171565b9093506020813d602011611742575b8161173260209383613589565b810103126104ce575192886110b2565b3d9150611725565b6117629060203d60201161074b5761073e8183613589565b611026565b8161177191613589565b610ca8578387610fb9565b9096506020813d6020116117a8575b8161179860209383613589565b810103126104ce57519587610f46565b3d915061178b565b5034610153578060031936011261015357601a546117cd816135f8565b916117db6040519384613589565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b83831061181d57604051806101f08782613445565b6001602081926040516118348161084b8189613776565b815201920192019190611808565b503461015357806003193601126101535761054c6001600160a01b03601f5460081c166118ce604051611876606082613589565b602381527f41697264726f7020636f6e74726163742073686f756c64206265206465706c6f60208201527f79656400000000000000000000000000000000000000000000000000000000006040820152821515614ab8565b3b1515604051906118e0606083613589565b602182527f41697264726f7020636f6e74726163742073686f756c64206861766520636f6460208301527f65000000000000000000000000000000000000000000000000000000000000006040830152614ab8565b5034610153578060031936011261015357601b54611952816135f8565b61195f6040519182613589565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b838310611a3757868587604051928392602084019060208552518091526040840160408260051b8601019392905b8282106119cc57505050500390f35b91936020611a27827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0600195979984950301865288519083611a1783516040845260408401906133cb565b92015190848184039101526133f0565b96019201920185949391926119bd565b60026020600192604051611a4a81613559565b604051611a5b8161084b818a613776565b8152611a68858701613816565b8382015281520192019201919061198f565b503461015357806003193601126101535760206001600160a01b03815416604051908152f35b50346101535780600319360112610153578060046040611af48151611ac58382613589565b601581527f546f6b656e204e616d653a2053796e64696361746500000000000000000000006020820152614895565b611b328151611b038382613589565b601281527f546f6b656e2053796d626f6c3a2053594e4400000000000000000000000000006020820152614895565b611b708151611b418382613589565b601281527f546f6b656e20446563696d616c733a20313800000000000000000000000000006020820152614895565b611bae8151611b7f8382613589565b600e81527f546f6b656e20416464726573733a00000000000000000000000000000000000060208201526148f2565b60206001600160a01b038154168251938480927f18160ddd0000000000000000000000000000000000000000000000000000000082525afa918215611d63578392611d2c575b50611c34828251611c058482613589565b600d81527f546f74616c20537570706c793a0000000000000000000000000000000000000060208201526149c5565b805191611c42606084613589565b602783527f546f74616c20737570706c792073686f756c6420626520393230206d696c6c6960208401527f6f6e2053594e440000000000000000000000000000000000000000000000000082840152737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c5a57611d04928491835194859283927f88b44c8500000000000000000000000000000000000000000000000000000000845260048401526b02f90193ef3075fa9800000060248401526060604484015260648301906133cb565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115610c145750610c035750f35b925090506020823d602011611d5b575b81611d4960209383613589565b810103126104ce57829151905f611bf4565b3d9150611d3c565b81513d85823e3d90fd5b503461015357806003193601126101535760206001600160a01b0360225416604051908152f35b503461015357806003193601126101535760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110611df3576101f0856101dc81870382613589565b82546001600160a01b0316845260209093019260019283019201611ddc565b503461015357806003193601126101535760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b818110611e71576101f0856101dc81870382613589565b82546001600160a01b0316845260209093019260019283019201611e5a565b503461015357806003193601126101535760206001600160a01b03601f5460081c16604051908152f35b5034610153578060031936011261015357601e54611ed7816135f8565b611ee46040519182613589565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b8383106120255786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310611f505786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110611fdc57505050505060208060019297019301930190928695949293611f43565b9091929394602080612018837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0876001960301895289516133cb565b9701950193929101611fb8565b60405161203181613559565b6001600160a01b03835416815260018301805461204d816135f8565b9161205b6040519384613589565b8183528a526020808b20908b9084015b838210612091575050505060019282602092836002950152815201920192019190611f14565b6001602081926040516120a88161084b818a613776565b81520193019101909161206b565b503461015357806003193601126101535760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b818110612115576101f0856101dc81870382613589565b82546001600160a01b03168452602090930192600192830192016120fe565b503461015357806003193601126101535760405190612154606083613589565b600282526040366020840137604080519261216f8285613589565b6001845260208401601f19830136823773123456789012345678901234567890123456789061219d83613641565b527323456789012345678901234567890123456789016121bc83613662565b52683635c9adc5dea000006121d086613641565b526001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561068a578351907f06447d560000000000000000000000000000000000000000000000000000000082526004820152848160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c5f57908591612495575b5060206001600160a01b0381541660446001600160a01b03601f5460081c16875194859384927f095ea7b30000000000000000000000000000000000000000000000000000000084526004840152683635c9adc5dea0000060248401525af18015610c5f57612478575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ca85782517ff4844814000000000000000000000000000000000000000000000000000000008152848160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c5f57908591612463575b50506001600160a01b03601f5460081c1694853b1561068a5760206123938551947f82947abe000000000000000000000000000000000000000000000000000000008652731bab804803159ad84b8854581aa53ac72455614e600487015260806024870152608486019061338f565b916003198584030160448601525191828152019190855b81811061244d5750505081849581868185829650683635c9adc5dea00000606483015203925af18015610c3557908391610c20575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c1d578051907f90c5013b000000000000000000000000000000000000000000000000000000008252828260048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610c145750610c035750f35b82518452602093840193909201916001016123aa565b8161246d91613589565b610ca857835f612324565b6124909060203d60201161074b5761073e8183613589565b6122b8565b8161249f91613589565b610ca857835f61224e565b9050346104ce575f6003193601126104ce576020810173243c63d5dbcf619ee36fde7ff63d1564d5665b418152739697211552826d7714c0267d274f51984f39d060604083015260408252612500606083613589565b6125776040925f84516125138682613589565b601281527f53796e646963617465546f6b656e2e736f6c000000000000000000000000000060208201528551809481927f8d1cc9250000000000000000000000000000000000000000000000000000000083526020600484015260248301906133cb565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa918215613385575f92613329575b5090602080936125d99386519584879551918291018587015e840190838201905f8252519283915e01015f815203601f198101835282613589565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce575f61262b918351809381927fb4d6c78200000000000000000000000000000000000000000000000000000000835260048301614cf3565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561331f5761330a575b508180808080731bab804803159ad84b8854581aa53ac72455614e5af1612675614d1e565b9015613261578290737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077657816126cf918451809381927fb4d6c78200000000000000000000000000000000000000000000000000000000835260048301614cf3565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c9b5761324c575b505080516101b88082019082821067ffffffffffffffff83111761323857908291615cf28339039083f0801561322d577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f55731bab804803159ad84b8854581aa53ac72455614e7fffffffffffffffffffffffff00000000000000000000000000000000000000006020541617602055731bab804803159ad84b8854581aa53ac72455614e7fffffffffffffffffffffffff000000000000000000000000000000000000000060215416176021556001600160a01b036127f5613bad565b167fffffffffffffffffffffffff000000000000000000000000000000000000000060225416176022555f806128be6128e68451612834606082613589565b602a81527f4772616e74696e672041495244524f505f4d414e414745525f524f4c4520757360208201527f696e672061646d696e3a000000000000000000000000000000000000000000008682015285519283917f319af33300000000000000000000000000000000000000000000000000000000602084015287602484015260648301906133cb565b73243c63d5dbcf619ee36fde7ff63d1564d5665b41604483015203601f198101835282613589565b6020815191016a636f6e736f6c652e6c6f675afa50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610776578181517f06447d5600000000000000000000000000000000000000000000000000000000815273243c63d5dbcf619ee36fde7ff63d1564d5665b416004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c9b57613218575b506001600160a01b03602154166001600160a01b0360225416813b1561077257829160448392865194859384927f2f2ff15d0000000000000000000000000000000000000000000000000000000084527f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a674600485015260248401525af18015610c9b57613203575b506001600160a01b03602154166001600160a01b03601f5460081c16813b1561077257829160448392865194859384927f2f2ff15d0000000000000000000000000000000000000000000000000000000084527f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a674600485015260248401525af18015610c9b576131ee575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610776578181517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c9b576131d9575b50506001600160a01b03602154166001600160a01b03602254169082517f91d148540000000000000000000000000000000000000000000000000000000081527f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a6746004820152826024820152602081604481855afa908115610c5f5785916131ba575b5060206001600160a01b03601f5460081c1660448651809581937f91d148540000000000000000000000000000000000000000000000000000000083527f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a674600484015260248301525afa918215610c5f578592613199575b50156131165715613093578290612c678351612c14606082613589565b8481527f5375636365737366756c6c79206772616e7465642041495244524f505f4d414e60208201527f414745525f524f4c4520746f20686f6c64657220616e6420636f6e747261637485820152614895565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610776578251907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c9b5761307e575b50506001600160a01b036020541660206001600160a01b036022541660248451809481937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa908115610c3557839161304c575b506901b1ae4d6e2ef5000000809110612fef57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610772578282517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015612fe557612fd0575b5050612ded8251612dbe8482613589565b600f81527f536574757020636f6d706c6574653a00000000000000000000000000000000006020820152614895565b612e3a6001600160a01b03601f5460081c168351612e0b8582613589565b601181527f41697264726f7020636f6e74726163743a0000000000000000000000000000006020820152614960565b612e788251612e498482613589565b600b81527f53594e4420746f6b656e3a00000000000000000000000000000000000000000060208201526148f2565b6001600160a01b036022541691612ec4838251612e958482613589565b600d81527f546f6b656e20686f6c6465723a000000000000000000000000000000000000006020820152614960565b60206001600160a01b038154169360248351809681937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa928315612fc6578493612f90575b50612f5561054c938251612f268482613589565b601581527f546f6b656e20686f6c6465722062616c616e63653a000000000000000000000060208201526149c5565b612f6181519182613589565b601581527f546f74616c2061697264726f7020616d6f756e743a000000000000000000000060208201526149c5565b92506020833d602011612fbe575b81612fab60209383613589565b810103126104ce57915191612f55612f12565b3d9150612f9e565b81513d86823e3d90fd5b81612fda91613589565b61077257825f612dad565b84513d84823e3d90fd5b606482517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601d60248201527f496e73756666696369656e742062616c616e636520666f7220746573740000006044820152fd5b90506020813d602011613076575b8161306760209383613589565b810103126104ce57515f612d31565b3d915061305a565b8161308891613589565b61077657815f612cd5565b608482517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603860248201527f4661696c656420746f206772616e742041495244524f505f4d414e414745525f60448201527f524f4c4520746f2061697264726f7020636f6e747261637400000000000000006064820152fd5b608483517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603460248201527f4661696c656420746f206772616e742041495244524f505f4d414e414745525f60448201527f524f4c4520746f20746f6b656e20686f6c6465720000000000000000000000006064820152fd5b6131b391925060203d60201161074b5761073e8183613589565b905f612bf7565b6131d3915060203d60201161074b5761073e8183613589565b5f612b7f565b816131e391613589565b61077657815f612afc565b816131f891613589565b61077657815f612a91565b8161320d91613589565b61077657815f612a06565b8161322291613589565b61077657815f61297e565b5051903d90823e3d90fd5b602485634e487b7160e01b81526041600452fd5b8161325691613589565b61077657815f6126f4565b60a482517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152605860248201527f537464436865617473206465706c6f79436f6465546f28737472696e672c627960448201527f7465732c75696e743235362c61646472657373293a204661696c656420746f2060648201527f6372656174652072756e74696d652062797465636f64652e00000000000000006084820152fd5b6133179192505f90613589565b5f905f612650565b82513d5f823e3d90fd5b91503d805f843e61333a8184613589565b8201916020818403126104ce5780519067ffffffffffffffff82116104ce57019180601f840112156104ce5760209361337c859285846125d997519101614cbd565b9350935061259e565b84513d5f823e3d90fd5b90602080835192838152019201905f5b8181106133ac5750505090565b82516001600160a01b031684526020938401939092019160010161339f565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b81811061340d5750505090565b82517fffffffff0000000000000000000000000000000000000000000000000000000016845260209384019390920191600101613400565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061347757505050505090565b90919293946020806134b3837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0866001960301875289516133cb565b97019301930191939290613468565b602081016020825282518091526040820191602060408360051b8301019401925f915b8383106134f457505050505090565b909192939460208061354a837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b038151168452015191818582015201906133f0565b970193019301919392906134e5565b6040810190811067ffffffffffffffff82111761357557604052565b634e487b7160e01b5f52604160045260245ffd5b90601f601f19910116810190811067ffffffffffffffff82111761357557604052565b67ffffffffffffffff811161357557601f01601f191660200190565b908160209103126104ce575190565b919082018092116135e457565b634e487b7160e01b5f52601160045260245ffd5b67ffffffffffffffff81116135755760051b60200190565b9061361a826135f8565b6136276040519182613589565b828152601f1961363782946135f8565b0190602036910137565b80511561364e5760200190565b634e487b7160e01b5f52603260045260245ffd5b80516001101561364e5760400190565b80516002101561364e5760600190565b80516003101561364e5760800190565b80516004101561364e5760a00190565b805182101561364e5760209160051b010190565b908160209103126104ce575180151581036104ce5790565b939291906136fe90731bab804803159ad84b8854581aa53ac72455614e865260806020870152608086019061338f565b908482036040860152602080825193848152019101915f5b81811061372857505060609150930152565b8351835260209384019390920191600101613716565b90600182811c9216801561376c575b602083101461375857565b634e487b7160e01b5f52602260045260245ffd5b91607f169161374d565b5f92918154916137858361373e565b80835292600181169081156137da57506001146137a157505050565b5f9081526020812093945091925b8383106137c0575060209250010190565b6001816020929493945483858701015201910191906137af565b905060209495507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091509291921683830152151560051b010190565b90604051918281549182825260208201905f5260205f20925f905b806007830110613a2f576138879454918181106139f9575b8181106139c3575b81811061398d575b818110613957575b818110613921575b8181106138eb575b8181106138b6575b10613889575b500383613589565b565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f61387f565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b168152019301613879565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301613871565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b168152019301613869565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301613861565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b168152019301613859565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b168152019301613851565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b168152019301613849565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e0820152019401920185929391613831565b919082039182116135e457565b60085460ff168015613ad85790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115613ba2575f91613b70575b50151590565b90506020813d602011613b9a575b81613b8b60209383613589565b810103126104ce57515f613b6a565b3d9150613b7e565b6040513d5f823e3d90fd5b5f604051613bbc608082613589565b600381526060366020830137731234567890123456789012345678901234567890613be682613641565b52732345678901234567890123456789012345678901613c0582613662565b52733456789012345678901234567890123456789012613c2482613672565b526020546001600160a01b03165f5b8251811015613cf8576001600160a01b03613c4e82856136a2565b5116604051907f70a082310000000000000000000000000000000000000000000000000000000082526004820152602081602481865afa908115613ba2575f91613cc7575b506901b1ae4d6e2ef50000001115613cad57600101613c33565b90506001600160a01b039250613cc2916136a2565b511690565b90506020813d8211613cf0575b81613ce160209383613589565b810103126104ce57515f613c93565b3d9150613cd4565b505050604090815191613d0b8184613589565b600a8352602083017f74657374486f6c6465720000000000000000000000000000000000000000000081528151600a6020820192835e5f602a820152600a8152613d56602a82613589565b519020928151937fffa186490000000000000000000000000000000000000000000000000000000085526004850152602084602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa93841561331f575f94614851575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce5781517fc657c7180000000000000000000000000000000000000000000000000000000081525f8180613e186001600160a01b0389169586600484015287602484015260448301906133cb565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561484757614832575b508280835160208101907f70a0823100000000000000000000000000000000000000000000000000000000825284602482015260248152613e7f604482613589565b5190731bab804803159ad84b8854581aa53ac72455614e5afa50613eb3613ea4614d1e565b602080825183010191016135c8565b50731bab804803159ad84b8854581aa53ac72455614e7fffffffffffffffffffffffff000000000000000000000000000000000000000060115416176011556370a082317fffffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000600f541617600f55600e546801000000000000000081101561481e576001810180600e5581101561480a57600e84526020842001556001600160a01b0360115416600f5460e01b60105490613f6b614d64565b907fffffffff00000000000000000000000000000000000000000000000000000000613fa8856001600160a01b03165f52600d60205260405f2090565b91169081875260205284862085516020810190613fd881613fca888886614d4d565b03601f198101835282613589565b519020875260205260ff6003868820015416156147fc575b61400b846001600160a01b03165f52600d60205260405f2090565b908652602052613fca61402b858720938651928391602083019586614d4d565b519020845260205281832090600182015491600281015461404c81856135d7565b614615575b815485517f667f9d700000000000000000000000000000000000000000000000000000000081526001600160a01b03851660048201526024810182905294909190602086604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa95861561460b5788966145d7575b506001908201610100031b5f1901811b19851691737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156145d35786517f70ca10bb0000000000000000000000000000000000000000000000000000000081526001600160a01b038616600482015260248101919091526903635c9adc5dea00000090911b919091176044820152858160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156145c9579086916145b4575b505084806020600f5460e01b6141d26024614185600d614e3d565b8a519485917fffffffff00000000000000000000000000000000000000000000000000000000828401961686528051918291018484015e810186838201520301601f198101845283613589565b6001600160a01b03601154169151915afa6141eb614d1e565b906010548060051b907f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8116036145a05787928051602081115f1461459a57506020905b89925b8284106145125750505050159081156144fd575b506143d257505050507fffffffffffffffffffffffff0000000000000000000000000000000000000000601154166011557fffffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000600f5416600f55600e5481600e5580614390575b50806010557fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00601354166013556142e460145461373e565b806142ee57505090565b601f811160011461430157506014555b90565b60148252601f0160051c7fce6d7b5282bd9a3661ae061feed1dbda4e52ab073b1f9285be6e155d9c38d4ec017fce6d7b5282bd9a3661ae061feed1dbda4e52ab073b1f9285be6e155d9c38d4ed5b81811061438557505060148082528190557fce6d7b5282bd9a3661ae061feed1dbda4e52ab073b1f9285be6e155d9c38d4ec5590565b5f815560010161434f565b600e82527fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3fd908101905b8181106143c757506142ac565b8281556001016143ba565b548491737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077257614440855194859384937f70ca10bb000000000000000000000000000000000000000000000000000000008552600485016040919493926001600160a01b03606083019616825260208201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c35576144e8575b608482517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603360248201527f73746453746f726167652066696e642853746453746f72616765293a2046616960448201527f6c656420746f2077726974652076616c75652e000000000000000000000000006064820152fd5b6144f3838092613589565b6107765781614465565b6903635c9adc5dea000000915014155f614246565b9091929461452086836135d7565b83518110156145865760207fff000000000000000000000000000000000000000000000000000000000000009185010151168660031b87810460081488151715614572571c1794600101929190614232565b60248d634e487b7160e01b81526011600452fd5b60248c634e487b7160e01b81526032600452fd5b9061422f565b602488634e487b7160e01b81526011600452fd5b816145be91613589565b61068a57845f61416a565b85513d88823e3d90fd5b8780fd5b9095506020813d602011614603575b816145f360209383613589565b810103126104ce575194816140bd565b3d91506145e6565b87513d8a823e3d90fd5b61461f81856135d7565b6101000361010081116147e85760ff81116147e8576001901b85517f6900a3ae0000000000000000000000000000000000000000000000000000000081528160048201528781602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561460b578891614784575b509061472c606a60209389519485917f73746453746f726167652066696e642853746453746f72616765293a20506163828401527f6b656420736c6f742e2057652063616e2774206669742076616c7565206772658c8401527f61746572207468616e200000000000000000000000000000000000000000000060608401528051918291018484015e81018b838201520301601f198101845283613589565b6903635c9adc5dea00000010156147435750614051565b6147809086519182917f08c379a00000000000000000000000000000000000000000000000000000000083526020600484015260248301906133cb565b0390fd5b90503d8089833e6147958183613589565b8101906020818303126147e05780519067ffffffffffffffff82116147e4570181601f820112156147e057606a6147d760209493838661472c95519101614cbd565b9293505061468d565b8880fd5b8980fd5b602487634e487b7160e01b81526011600452fd5b6148046150a6565b50613ff0565b602484634e487b7160e01b81526032600452fd5b602484634e487b7160e01b81526041600452fd5b61483f9193505f90613589565b5f915f613e3d565b83513d5f823e3d90fd5b9093506020813d60201161488d575b8161486d60209383613589565b810103126104ce57516001600160a01b03811681036104ce57925f613db0565b3d9150614860565b5f613fca6148db82936040519283917f41304fac0000000000000000000000000000000000000000000000000000000060208401526020602484015260448301906133cb565b6020815191016a636f6e736f6c652e6c6f675afa50565b5f6149386148db82936040519283917f319af3330000000000000000000000000000000000000000000000000000000060208401526040602484015260648301906133cb565b731bab804803159ad84b8854581aa53ac72455614e604483015203601f198101835282613589565b6149b06148db5f939284936001600160a01b036040519485937f319af3330000000000000000000000000000000000000000000000000000000060208601526040602486015260648501906133cb565b9116604483015203601f198101835282613589565b614a0d6148db5f939284936040519384927fb60e72cc0000000000000000000000000000000000000000000000000000000060208501526040602485015260648401906133cb565b90604483015203601f198101835282613589565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce575f91614a8860405194859384937f88b44c85000000000000000000000000000000000000000000000000000000008552600485015260248401526060604484015260648301906133cb565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015613ba257614aae5750565b5f61388791613589565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce57614a88915f9160405193849283927fa34edc03000000000000000000000000000000000000000000000000000000008452151560048401526040602484015260448301906133cb565b604051614b2a60c082613589565b600581525f5b60a08110614c9a5750604051614b4581613559565b7312345678901234567890123456789012345678908152683635c9adc5dea000006020820152614b7482613641565b52614b7e81613641565b50604051614b8b81613559565b7323456789012345678901234567890123456789018152686c6b935b8bbd4000006020820152614bba82613662565b52614bc481613662565b50604051614bd181613559565b7334567890123456789012345678901234567890128152685150ae84a8cdf000006020820152614c0082613672565b52614c0a81613672565b50604051614c1781613559565b734567890123456789012345678901234567890123815268a2a15d09519be000006020820152614c4682613682565b52614c5081613682565b50604051614c5d81613559565b7356789012345678901234567890123456789012348152681b1ae4d6e2ef5000006020820152614c8c82613692565b52614c9681613692565b5090565b602090604051614ca981613559565b5f81525f8382015282828501015201614b30565b929192614cc9826135ac565b91614cd76040519384613589565b8294818452818301116104ce578281602093845f96015e010152565b9060406142fe92731bab804803159ad84b8854581aa53ac72455614e815281602082015201906133cb565b3d15614d48573d90614d2f826135ac565b91614d3d6040519384613589565b82523d5f602084013e565b606090565b60209291908391805192839101825e019081520190565b614d6f60145461373e565b614e2b576040519081826020600e549283815201600e5f5260205f20925f5b818110614e12575050614da392500383613589565b81518060051b90808204602014901517156135e457601f19614ddd614dc7836135ac565b92614dd56040519485613589565b8084526135ac565b013660208301375f5b8351811015614e0d5780614dfc600192866136a2565b5160208260051b8501015201614de6565b509150565b8454835260019485019487945060209093019201614d8e565b6040516142fe8161084b816014613776565b6007810190614e4c825461373e565b614ef157600191500190604051808360208295549384815201905f5260205f20925f5b818110614ed8575050614e8492500383613589565b81518060051b90808204602014901517156135e457601f19614ea8614dc7836135ac565b013660208301375f5b8351811015614e0d5780614ec7600192866136a2565b5160208260051b8501015201614eb1565b8454835260019485019487945060209093019201614e6f565b506142fe61084b9160405192838092613776565b905f806020600285015460e01b614f6e6024614f2088614e3d565b6040519485917fffffffff00000000000000000000000000000000000000000000000000000000828401961686528051918291018484015e810186838201520301601f198101845283613589565b6001600160a01b036004870154169151915afa6003614f8b614d1e565b9301548060051b907f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8116036135e4575f938051602081115f1461504357506020905b5f925b828410614fe057505050509190565b90919295614fee87836135d7565b835181101561364e5760207fff00000000000000000000000000000000000000000000000000000000000000918501015116908760031b91888304600814891517156135e4576001921c179601929190614fd1565b90614fce565b9080601f830112156104ce578151615060816135f8565b9261506e6040519485613589565b81845260208085019260051b8201019283116104ce57602001905b8282106150965750505090565b8151815260209182019101615089565b5f6001600160a01b036011541690600f5460e01b601054906150c8600d614e3d565b90845f52600d6020527fffffffff0000000000000000000000000000000000000000000000000000000060405f20911690815f5260205260405f20604051602081019061511a81613fca888886614d4d565b5190205f5260205260ff600360405f20015416615ac957737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce576040517f266cf1090000000000000000000000000000000000000000000000000000000081525f8160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015613ba257615ab4575b506151a5600d614f05565b90506040517f65bc9481000000000000000000000000000000000000000000000000000000008152866004820152858160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156116e1578691615a4e575b5080518061528a5760846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152604060248201527f73746453746f726167652066696e642853746453746f72616765293a204e6f2060448201527f73746f726167652075736520646574656374656420666f72207461726765742e6064820152fd5b80156147e857905f196152ef92019060206152a583836136a2565b516040517f667f9d700000000000000000000000000000000000000000000000000000000081526001600160a01b038c166004820152602481019190915293849081906044820190565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa928315610ee4578893615a1b575b5082156159c3575b61532882826136a2565b516011546040517f667f9d700000000000000000000000000000000000000000000000000000000081526001600160a01b0390911660048201819052602482018390529190602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa9081156159b8578b91615987575b506153a3600d614f05565b91909382155f14615980575f19905b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15615967576040517f70ca10bb0000000000000000000000000000000000000000000000000000000081526001600160a01b039190911660048201526024810185905260448101919091528c8160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156158e457908d9161596b575b505061544a600d614f05565b9390506001600160a01b0360115416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15615967576040517f70ca10bb0000000000000000000000000000000000000000000000000000000081526001600160a01b03919091166004820152602481019190915260448101919091528b8160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561595c57908c91615943575b505082615938575b505015615931578793889360ff60135416615761575b6001868601610100031b5f1901851b16841c810361575857509061558a917f9c9555b1e3102e3cf48f427d79cb678f5d9bd1ed0ad574389461e255f95170ed60808b89613fca61555f8d604051928391602083019586614d4d565b51902061556c86866136a2565b51906040519283528a602084015260408301526060820152a16136a2565b5190604051916080830183811067ffffffffffffffff8211176157445790600393929160405282526020820193845260408201908152606082019360018552898952600d60205260408920868a526020526040892060405160208101906155f681613fca8d8d86614d4d565b5190208a526020526040892092518355516001830155516002820155019051151560ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008354169116179055848452600d6020526040842081855260205260408420604051602081019061566f81613fca888886614d4d565b519020855260205260ff600360408620015416156156c0576040948452600d602052848420908452602052613fca6156b4858520938651928391602083019586614d4d565b51902082526020522090565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602f60248201527f73746453746f726167652066696e642853746453746f72616765293a20536c6f60448201527f74287329206e6f7420666f756e642e00000000000000000000000000000000006064820152fd5b602489634e487b7160e01b81526041600452fd5b9350915061528a565b94506157cd935061577283836136a2565b51946020866001600160a01b036011541660405197889283927f667f9d7000000000000000000000000000000000000000000000000000000000845260048401602090939291936001600160a01b0360408201951681520152565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa948515615926578a956158f3575b5061580086600d615b0a565b95909661580e81600d615c08565b9290916001600160a01b0360115416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156158ef576040517f70ca10bb0000000000000000000000000000000000000000000000000000000081526001600160a01b03919091166004820152602481019190915260448101919091528c8160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156158e457908d916158cb575b5050876158c3575b50949561550457509350915061528a565b96505f6158b2565b816158d591613589565b6158e0578b5f6158aa565b8b80fd5b6040513d8f823e3d90fd5b8e80fd5b9094506020813d821161591e575b8161590e60209383613589565b810103126104ce5751935f6157f4565b3d9150615901565b6040513d8c823e3d90fd5b915061528a565b141590505f806154ee565b8161594d91613589565b615958578a5f6154e6565b8a80fd5b6040513d8e823e3d90fd5b8d80fd5b8161597591613589565b6158e0578b5f61543e565b8c906153b2565b90506020813d82116159b0575b816159a160209383613589565b810103126104ce57515f615398565b3d9150615994565b6040513d8d823e3d90fd5b7f080fc4a96620c4462e705b23f346413fe3796bb63c6f8d8591baec0e231577a5615a136159f184846136a2565b51604080516001600160a01b038e168152602081019290925290918291820190565b0390a161531e565b9092506020813d8211615a46575b81615a3660209383613589565b810103126104ce5751915f615316565b3d9150615a29565b90503d8087833e615a5f8183613589565b8101604082820312615ab057815167ffffffffffffffff81116145d35781615a88918401615049565b9160208101519067ffffffffffffffff82116147e057615aa9929101615049565b505f6151fe565b8680fd5b615ac19194505f90613589565b5f925f61519a565b91939092505f52600d60205260405f20905f52602052613fca615afb60405f2093604051928391602083019586614d4d565b5190205f5260205260405f2090565b91905f5b6101008110615b2157505090505f905f90565b8060ff0360ff81116135e4576001901b6001600160a01b03600486015416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce576040517f70ca10bb0000000000000000000000000000000000000000000000000000000081526001600160a01b039190911660048201526024810184905260448101919091525f8160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015613ba257615bf8575b50615bd284614f05565b81615bee575b50615be557600101615b0e565b92505060019190565b905015155f615bd8565b5f615c0291613589565b5f615bc8565b91905f5b6101008110615c1f57505090505f905f90565b6001811b6001600160a01b03600486015416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce576040517f70ca10bb0000000000000000000000000000000000000000000000000000000081526001600160a01b039190911660048201526024810184905260448101919091525f8160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015613ba257615ce1575b50615cc484614f05565b81615cd7575b50615be557600101615c0c565b905015155f615cca565b5f615ceb91613589565b5f615cba56fe6080806040523460155761019e908161001a8239f35b5f80fdfe60806040526004361015610011575f80fd5b5f3560e01c6382947abe14610024575f80fd5b60807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100c15760043573ffffffffffffffffffffffffffffffffffffffff811681036100c15760243567ffffffffffffffff81116100c15761008e9036906004016100c5565b604435929167ffffffffffffffff84116100c1576100b36100bf9436906004016100c5565b929091606435946100f6565b005b5f80fd5b9181601f840112156100c15782359167ffffffffffffffff83116100c1576020808501948460051b0101116100c157565b918093959194036100c1577f23b872dd000000000000000000000000000000000000000000000000000000005f5233600452306024526044525f8060648180855af1156100c15791907fa9059cbb000000000000000000000000000000000000000000000000000000005f5260051b8101928103905b8035600452818103356024525f8060648180875af1156100c1576020019183831015610198579161016c565b5050505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`/W`\x01`\xFF\x19`\x0CT\x16\x17`\x0CU`\x01`\xFF\x19`\x1FT\x16\x17`\x1FUa^\xAA\x90\x81a\x004\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\n\x92T\xE4\x14a$\xAAWP\x80c\x1By\x86\xD2\x14a!4W\x80c\x1E\xD7\x83\x1C\x14a \xB6W\x80c*\xDE8\x80\x14a\x1E\xBAW\x80c8\x84\xD65\x14a\x1E\x90W\x80c>^<#\x14a\x1E\x12W\x80c?r\x86\xF4\x14a\x1D\x94W\x80cB\n\x83\xE7\x14a\x1DmW\x80cY\xC6 \xC6\x14a\x1A\xA0W\x80cb\x13\x82\x1D\x14a\x1AzW\x80cf\xD9\xA9\xA0\x14a\x195W\x80c\x81\xA8\xDE~\x14a\x18BW\x80c\x85\"l\x81\x14a\x17\xB0W\x80c\x85\xA4h\xF7\x14a\r\x91W\x80c\x8AT%!\x14a\rVW\x80c\x91j\x17\xC6\x14a\x0C\xACW\x80c\x94\xA2\xA5\xDF\x14a\t&W\x80c\xA2\x17\xFD\xDF\x14a\t\nW\x80c\xB0FO\xDC\x14a\x08`W\x80c\xB5P\x8A\xA9\x14a\x07\xC7W\x80c\xBAAO\xA6\x14a\x07\xA2W\x80c\xBF\xF8`\x0F\x14a\x02\x13W\x80c\xE2\x0C\x9Fq\x14a\x01}W\x80c\xE5*/\x1F\x14a\x01VWc\xFAv&\xD4\x14a\x011W_\x80\xFD[4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x01\xF4Wa\x01\xF0\x85a\x01\xDC\x81\x87\x03\x82a5\x89V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a3\x8FV[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x01\xC5V[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SWa\x02,aK\x1CV[\x80Q`\x03\x81\x01\x90\x81\x81\x11a\x07\x8EW`\x02\x01\x90\x81\x11a\x07zW`\x03\x90\x04\x90\x82`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07vW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05dWa\x07]W[P` \x80T`\x1FT`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03`\x08\x92\x90\x92\x1C\x82\x16`\x04\x82\x01Ri\x01\xB1\xAEMn.\xF5\0\0\0`$\x82\x01R\x93\x84\x92`D\x92\x84\x92\x91\x16Z\xF1\x80\x15a\x07RWa\x07%W[P\x82[\x82\x81\x10a\x05oW\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01SW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05dWa\x05OW[PPa\x03\xAEaK\x1CV[` T`\x01`\x01`\xA0\x1B\x03\x16\x82[\x82Q\x81\x10\x15a\x04\xE5W`\x01`\x01`\xA0\x1B\x03a\x03\xD7\x82\x85a6\xA2V[QQ\x16\x90`@Q\x91\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R` \x82`$\x81\x86Z\xFA\x91\x82\x15a\x04\xDAW\x85\x92a\x04\xA1W[Pa\x04\x9B`\x01\x92` a\x041\x84\x88a6\xA2V[Q\x01Q\x11\x15``\x90a\x04F`@Q\x92\x83a5\x89V[`-\x82R\x7FRecipient should have received a` \x83\x01R\x7Firdrop amount\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x83\x01RaJ\xB8V[\x01a\x03\xBCV[\x91P` \x82=\x82\x11a\x04\xD2W[\x81a\x04\xBB` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEW\x90Q\x90a\x04\x9Ba\x04\x1EV[_\x80\xFD[=\x91Pa\x04\xAEV[`@Q=\x87\x82>=\x90\xFD[\x83a\x05L`@Qa\x04\xF7``\x82a5\x89V[`$\x81R\x7FBatch airdrop completed successf` \x82\x01R\x7Fully\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RaH\x95V[\x80\xF3[\x81a\x05Y\x91a5\x89V[a\x01SW\x80\x82a\x03\xA4V[`@Q=\x84\x82>=\x90\xFD[`\x03\x81\x02\x90\x80\x82\x04`\x03\x14\x81\x15\x17\x15a\x07\x11W`\x03\x82\x01\x80\x83\x11a\x06\xFDWa\x05\xA5\x83\x82\x88\x93\x90\x87Q\x80\x91\x11a\x06\xF5W[Pa:\xBCV[\x92a\x05\xAF\x84a6\x10V[a\x05\xB8\x85a6\x10V[\x91\x83\x90\x84\x90[\x87\x82\x10a\x06\x8EWPP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x06\x8AWa\x06\x1A\x93\x85\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93\x7F\x82\x94z\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a6\xCEV[\x03\x92Z\xF1\x80\x15a\x05dWa\x06uW[P`\x01\x92a\x06o\x91P`@a\x06@\x81Q\x91\x82a5\x89V[`\x1F\x81R\x7FBatch executed with recipients:\0` \x82\x01RaI\xC5V[\x01a\x032V[\x81a\x06\x7F\x91a5\x89V[a\x06\x8AW\x84_a\x06)V[\x84\x80\xFD[\x90\x91a\x06\xED`\x01\x91`\x01`\x01`\xA0\x1B\x03a\x06\xB1a\x06\xAB\x87\x87a5\xD7V[\x8Da6\xA2V[QQ\x16a\x06\xBE\x86\x88a6\xA2V[R` a\x06\xCEa\x06\xAB\x87\x87a5\xD7V[Q\x01Qa\x06\xDB\x86\x89a6\xA2V[Ra\x06\xE6\x85\x88a6\xA2V[Q\x90a5\xD7V[\x92\x01\x90a\x05\xBEV[\x90P_a\x05\x9FV[`$\x86cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`$\x85cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[a\x07F\x90` =` \x11a\x07KW[a\x07>\x81\x83a5\x89V[\x81\x01\x90a6\xB6V[a\x03/V[P=a\x074V[`@Q=\x86\x82>=\x90\xFD[\x81a\x07g\x91a5\x89V[a\x07rW\x82_a\x02\xC5V[\x82\x80\xFD[P\x80\xFD[`$\x83cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`$\x84cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` a\x07\xBDa:\xC9V[`@Q\x90\x15\x15\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`\x19Ta\x07\xE4\x81a5\xF8V[\x91a\x07\xF2`@Q\x93\x84a5\x89V[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\x084W`@Q\x80a\x01\xF0\x87\x82a4EV[`\x01` \x81\x92`@Qa\x08R\x81a\x08K\x81\x89a7vV[\x03\x82a5\x89V[\x81R\x01\x92\x01\x92\x01\x91\x90a\x08\x1FV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`\x1CTa\x08}\x81a5\xF8V[\x91a\x08\x8B`@Q\x93\x84a5\x89V[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\x08\xCDW`@Q\x80a\x01\xF0\x87\x82a4\xC2V[`\x02` `\x01\x92`@Qa\x08\xE0\x81a5YV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x08\xF8\x85\x87\x01a8\x16V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x08\xB8V[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` \x90`@Q\x90\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SWa\t?aK\x1CV[\x90`@\x80Q\x92a\tO\x82\x85a5\x89V[`\x01\x84R` a\t\x9D`\x1F\x19\x84\x01\x92\x836\x84\x89\x017\x84Q\x93a\tq\x86\x86a5\x89V[`\x01\x85R6\x84\x86\x017`\x01`\x01`\xA0\x1B\x03a\t\x8B\x82a6AV[QQ\x16a\t\x97\x88a6AV[Ra6AV[Q\x01Qa\t\xA9\x82a6AV[R`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C\xA8W\x82Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C\x9BW\x90\x84\x91a\x0C\x86W[PP`\x01`\x01`\xA0\x1B\x03` T\x16\x93`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x94a\nO\x83a6AV[Q\x95_\x19\x87\x01\x96\x87\x11a\x06\xFDW\x84Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x96\x90\x96R\x93\x94\x85\x94` \x90\x82\x90`D\x90\x82\x90\x89\x90Z\xF1\x80\x15a\x0C_Wa\x0CiW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0CZW\x82Q\x7F\xF4\x84H\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C_W\x90\x85\x91a\x0CEW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16a\x0B=\x83a6AV[Q\x90\x80;\x15a\x0CAWa\x0B\x82\x93\x86\x80\x94\x87Q\x96\x87\x95\x86\x94\x85\x93\x7F\x82\x94z\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a6\xCEV[\x03\x92Z\xF1\x80\x15a\x0C5W\x90\x83\x91a\x0C W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C\x1DW\x80Q\x90\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x82\x82`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0C\x14WPa\x0C\x03WP\xF3[\x81a\x0C\r\x91a5\x89V[a\x01SW\x80\xF3[Q=\x84\x82>=\x90\xFD[P\xFD[\x81a\x0C*\x91a5\x89V[a\x0C\x1DW\x81_a\x0B\x94V[PPQ\x90=\x90\x82>=\x90\xFD[\x85\x80\xFD[\x81a\x0CO\x91a5\x89V[a\x0CZW\x83_a\x0B#V[PPP\xFD[\x84Q=\x87\x82>=\x90\xFD[a\x0C\x81\x90` =` \x11a\x07KWa\x07>\x81\x83a5\x89V[a\n\xB7V[\x81a\x0C\x90\x91a5\x89V[a\x07rW\x82_a\n'V[PPPQ\x90=\x90\x82>=\x90\xFD[\x83\x80\xFD[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`\x1DTa\x0C\xC9\x81a5\xF8V[\x91a\x0C\xD7`@Q\x93\x84a5\x89V[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\r\x19W`@Q\x80a\x01\xF0\x87\x82a4\xC2V[`\x02` `\x01\x92`@Qa\r,\x81a5YV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\rD\x85\x87\x01a8\x16V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\r\x04V[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` `@Q\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SWa\r\xAAaK\x1CV[\x90a\r\xB5\x82Qa6\x10V[a\r\xBF\x83Qa6\x10V[\x82\x91\x82[\x85Q\x84\x10\x15a\x0E\x1EWa\x0E\x16`\x01\x91`\x01`\x01`\xA0\x1B\x03a\r\xE4\x87\x8Aa6\xA2V[QQ\x16a\r\xF1\x87\x86a6\xA2V[R` a\r\xFE\x87\x8Aa6\xA2V[Q\x01Qa\x0E\x0B\x87\x87a6\xA2V[Ra\x06\xE6\x86\x86a6\xA2V[\x93\x01\x92a\r\xC3V[\x90\x91\x92Pa\x0E,\x82Qa6\x10V[\x91\x84\x91`\x01`\x01`\xA0\x1B\x03` T\x16\x92[\x82Q\x81\x10\x15a\x0E\xEFW`\x01`\x01`\xA0\x1B\x03a\x0EX\x82\x85a6\xA2V[Q\x16\x90`@Q\x91\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R` \x82`$\x81\x88Z\xFA\x80\x15a\x0E\xE4W\x88\x90a\x0E\xB2W[`\x01\x92Pa\x0E\xAB\x82\x88a6\xA2V[R\x01a\x0E=V[P` \x82=\x82\x11a\x0E\xDCW[\x81a\x0E\xCB` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEW`\x01\x91Qa\x0E\x9DV[=\x91Pa\x0E\xBEV[`@Q=\x8A\x82>=\x90\xFD[P\x93\x91\x92`$\x95\x93` `\x01`\x01`\xA0\x1B\x03`\"T\x16\x91`@Q\x98\x89\x80\x92\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x85`\x04\x83\x01RZ\xFA\x96\x87\x15a\x04\xDAW\x85\x97a\x17|W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x8AW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x84\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xDAW\x90\x85\x91a\x17gW[PP` \x80T`\x1FT`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03`\x08\x92\x90\x92\x1C\x82\x16`\x04\x82\x01R`$\x81\x01\x89\x90R\x92\x91\x83\x91\x16\x81\x88\x81`D\x81\x01\x03\x92Z\xF1\x80\x15a\x04\xDAW\x90a\x10\xA0\x93\x92\x91a\x17JW[P`\x01`\x01`\xA0\x1B\x03` T\x16` `\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x92\x83\x91`@Q\x80\x98\x81\x94\x82\x93\x7F\xDDb\xED>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91\x81`@\x85\x01\x96\x16\x84R\x16\x91\x01RV[\x03\x91Z\xFA\x93\x84\x15a\x16\xE1W\x86\x94a\x17\x16W[Pa\x11\x1D``\x94\x88`@Q\x91a\x10\xC8\x88\x84a5\x89V[`#\x83R\x7FAllowance should equal total amo` \x84\x01R\x7Funt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x84\x01RaJ!V[\x80;\x15a\x0CAW\x85`@Q\x80\x92\x7F\x82\x94z\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x83\x81a\x11_\x8D\x8C\x8B`\x04\x85\x01a6\xCEV[\x03\x92Z\xF1\x80\x15a\x16\xE1W\x90\x86\x91a\x17\x01W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x8AW`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x85\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x16\xE1W\x90\x86\x91a\x16\xECW[P\x96\x95`\x01`\x01`\xA0\x1B\x03` T\x16\x97[\x83Q\x81\x10\x15a\x13bWa\x12\x11a\x12\x06\x82\x85a6\xA2V[Qa\x06\xE6\x83\x89a6\xA2V[\x90\x89` `\x01`\x01`\xA0\x1B\x03a\x12'\x84\x89a6\xA2V[Q\x16`$`@Q\x80\x94\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x13WW\x89\x91a\x13%W[P`@Q`\x01\x93a\x12\xD4\x92a\x12\x7F\x8A\x84a5\x89V[`3\x83R\x7FRecipient balance should increas` \x84\x01R\x7Fe by airdrop amount\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x84\x01RaJ!V[a\x13\x1Fa\x12\xE1\x82\x89a6\xA2V[Q`@a\x12\xF0\x81Q\x91\x82a5\x89V[`\x18\x81R\x7FRecipient received SYND:\0\0\0\0\0\0\0\0` \x82\x01RaI\xC5V[\x01a\x11\xF0V[\x90P` \x81=\x82\x11a\x13OW[\x81a\x13?` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ`\x01a\x12jV[=\x91Pa\x132V[`@Q=\x8B\x82>=\x90\xFD[\x86\x84\x83\x82\x88\x8C\x8E`\x01`\x01`\xA0\x1B\x03`\"T\x16\x91`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x91\x82\x15a\x16\xE1W\x87\x91\x87\x93a\x16\xA2W[Pa\x14\x90\x94\x92a\x13\xD1` \x95\x93a\x143\x93a:\xBCV[`@Q\x91a\x13\xDF\x89\x84a5\x89V[`4\x83R\x7FToken holder balance should decr\x87\x84\x01R\x7Fease by total amount\0\0\0\0\0\0\0\0\0\0\0\0`@\x84\x01RaJ!V[`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x91`@Q\x80\x95\x81\x94\x82\x93\x7F\xDDb\xED>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91\x81`@\x85\x01\x96\x16\x84R\x16\x91\x01RV[\x03\x91Z\xFA\x90\x81\x15a\x16\x97W\x83\x91a\x16bW[P`@Q\x91a\x14\xB1\x81\x84a5\x89V[`&\x83R\x7FAllowance should be zero after a` \x84\x01R\x7Firdrop\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x84\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C\xA8W\x83\x91a\x15f`@Q\x94\x85\x93\x84\x93\x7F\x88\xB4L\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01R\x85`$\x85\x01R`D\x84\x01R`d\x83\x01\x90a3\xCBV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05dWa\x16MW[P\x91a\x05L\x92Pa\x16\x10`@\x92a\x15\xD5\x84Qa\x15\xA6\x86\x82a5\x89V[`\x1E\x81R\x7FAirdrop executed successfully:\0\0` \x82\x01RaH\x95V[\x83Qa\x15\xE1\x85\x82a5\x89V[`\x12\x81R\x7FTotal distributed:\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI\xC5V[Q\x90a\x16\x1E\x81Q\x91\x82a5\x89V[`\x0B\x81R\x7FRecipients:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI\xC5V[\x81a\x16W\x91a5\x89V[a\x07rW\x82\x84a\x15\x8AV[\x92PP` \x82=` \x11a\x16\x8FW[\x81a\x16~` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEW\x84\x91Q\x86a\x14\xA2V[=\x91Pa\x16qV[`@Q=\x85\x82>=\x90\xFD[\x94\x92P\x92\x95PP` \x83=` \x11a\x16\xD9W[\x81a\x16\xC2` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEW\x91Q\x87\x94\x91\x86\x90a\x143a\x13\xBBV[=\x91Pa\x16\xB5V[`@Q=\x88\x82>=\x90\xFD[\x81a\x16\xF6\x91a5\x89V[a\x06\x8AW\x84\x88a\x11\xDFV[\x81a\x17\x0B\x91a5\x89V[a\x06\x8AW\x84\x88a\x11qV[\x90\x93P` \x81=` \x11a\x17BW[\x81a\x172` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ\x92\x88a\x10\xB2V[=\x91Pa\x17%V[a\x17b\x90` =` \x11a\x07KWa\x07>\x81\x83a5\x89V[a\x10&V[\x81a\x17q\x91a5\x89V[a\x0C\xA8W\x83\x87a\x0F\xB9V[\x90\x96P` \x81=` \x11a\x17\xA8W[\x81a\x17\x98` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ\x95\x87a\x0FFV[=\x91Pa\x17\x8BV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`\x1ATa\x17\xCD\x81a5\xF8V[\x91a\x17\xDB`@Q\x93\x84a5\x89V[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\x18\x1DW`@Q\x80a\x01\xF0\x87\x82a4EV[`\x01` \x81\x92`@Qa\x184\x81a\x08K\x81\x89a7vV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x18\x08V[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SWa\x05L`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16a\x18\xCE`@Qa\x18v``\x82a5\x89V[`#\x81R\x7FAirdrop contract should be deplo` \x82\x01R\x7Fyed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x82\x15\x15aJ\xB8V[;\x15\x15`@Q\x90a\x18\xE0``\x83a5\x89V[`!\x82R\x7FAirdrop contract should have cod` \x83\x01R\x7Fe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x83\x01RaJ\xB8V[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`\x1BTa\x19R\x81a5\xF8V[a\x19_`@Q\x91\x82a5\x89V[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a\x1A7W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a\x19\xCCWPPPP\x03\x90\xF3[\x91\x93` a\x1A'\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a\x1A\x17\x83Q`@\x84R`@\x84\x01\x90a3\xCBV[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra3\xF0V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x19\xBDV[`\x02` `\x01\x92`@Qa\x1AJ\x81a5YV[`@Qa\x1A[\x81a\x08K\x81\x8Aa7vV[\x81Ra\x1Ah\x85\x87\x01a8\x16V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x19\x8FV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW\x80`\x04`@a\x1A\xF4\x81Qa\x1A\xC5\x83\x82a5\x89V[`\x15\x81R\x7FToken Name: Syndicate\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaH\x95V[a\x1B2\x81Qa\x1B\x03\x83\x82a5\x89V[`\x12\x81R\x7FToken Symbol: SYND\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaH\x95V[a\x1Bp\x81Qa\x1BA\x83\x82a5\x89V[`\x12\x81R\x7FToken Decimals: 18\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaH\x95V[a\x1B\xAE\x81Qa\x1B\x7F\x83\x82a5\x89V[`\x0E\x81R\x7FToken Address:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaH\xF2V[` `\x01`\x01`\xA0\x1B\x03\x81T\x16\x82Q\x93\x84\x80\x92\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x1DcW\x83\x92a\x1D,W[Pa\x1C4\x82\x82Qa\x1C\x05\x84\x82a5\x89V[`\r\x81R\x7FTotal Supply:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI\xC5V[\x80Q\x91a\x1CB``\x84a5\x89V[`'\x83R\x7FTotal supply should be 920 milli` \x84\x01R\x7Fon SYND\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0CZWa\x1D\x04\x92\x84\x91\x83Q\x94\x85\x92\x83\x92\x7F\x88\xB4L\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rk\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0`$\x84\x01R```D\x84\x01R`d\x83\x01\x90a3\xCBV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x0C\x14WPa\x0C\x03WP\xF3[\x92P\x90P` \x82=` \x11a\x1D[W[\x81a\x1DI` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEW\x82\x91Q\x90_a\x1B\xF4V[=\x91Pa\x1D<V[\x81Q=\x85\x82>=\x90\xFD[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a\x1D\xF3Wa\x01\xF0\x85a\x01\xDC\x81\x87\x03\x82a5\x89V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x1D\xDCV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a\x1EqWa\x01\xF0\x85a\x01\xDC\x81\x87\x03\x82a5\x89V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x1EZV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`\x1ETa\x1E\xD7\x81a5\xF8V[a\x1E\xE4`@Q\x91\x82a5\x89V[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a %W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a\x1FPW\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a\x1F\xDCWPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a\x1FCV[\x90\x91\x92\x93\x94` \x80a \x18\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa3\xCBV[\x97\x01\x95\x01\x93\x92\x91\x01a\x1F\xB8V[`@Qa 1\x81a5YV[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta M\x81a5\xF8V[\x91a [`@Q\x93\x84a5\x89V[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a \x91WPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x1F\x14V[`\x01` \x81\x92`@Qa \xA8\x81a\x08K\x81\x8Aa7vV[\x81R\x01\x93\x01\x91\x01\x90\x91a kV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a!\x15Wa\x01\xF0\x85a\x01\xDC\x81\x87\x03\x82a5\x89V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a \xFEV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`@Q\x90a!T``\x83a5\x89V[`\x02\x82R`@6` \x84\x017`@\x80Q\x92a!o\x82\x85a5\x89V[`\x01\x84R` \x84\x01`\x1F\x19\x83\x016\x827s\x124Vx\x90\x124Vx\x90\x124Vx\x90\x124Vx\x90a!\x9D\x83a6AV[Rs#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01a!\xBC\x83a6bV[Rh65\xC9\xAD\xC5\xDE\xA0\0\0a!\xD0\x86a6AV[R`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x8AW\x83Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x84\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C_W\x90\x85\x91a$\x95W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x87Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x0C_Wa$xW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C\xA8W\x82Q\x7F\xF4\x84H\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C_W\x90\x85\x91a$cW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x94\x85;\x15a\x06\x8AW` a#\x93\x85Q\x94\x7F\x82\x94z\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86Rs\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN`\x04\x87\x01R`\x80`$\x87\x01R`\x84\x86\x01\x90a3\x8FV[\x91`\x03\x19\x85\x84\x03\x01`D\x86\x01RQ\x91\x82\x81R\x01\x91\x90\x85[\x81\x81\x10a$MWPPP\x81\x84\x95\x81\x86\x81\x85\x82\x96Ph65\xC9\xAD\xC5\xDE\xA0\0\0`d\x83\x01R\x03\x92Z\xF1\x80\x15a\x0C5W\x90\x83\x91a\x0C WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C\x1DW\x80Q\x90\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x82\x82`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0C\x14WPa\x0C\x03WP\xF3[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a#\xAAV[\x81a$m\x91a5\x89V[a\x0C\xA8W\x83_a#$V[a$\x90\x90` =` \x11a\x07KWa\x07>\x81\x83a5\x89V[a\"\xB8V[\x81a$\x9F\x91a5\x89V[a\x0C\xA8W\x83_a\"NV[\x90P4a\x04\xCEW_`\x03\x196\x01\x12a\x04\xCEW` \x81\x01s$<c\xD5\xDB\xCFa\x9E\xE3o\xDE\x7F\xF6=\x15d\xD5f[A\x81Rs\x96\x97!\x15R\x82mw\x14\xC0&}'OQ\x98O9\xD0``@\x83\x01R`@\x82Ra%\0``\x83a5\x89V[a%w`@\x92_\x84Qa%\x13\x86\x82a5\x89V[`\x12\x81R\x7FSyndicateToken.sol\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x85Q\x80\x94\x81\x92\x7F\x8D\x1C\xC9%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a3\xCBV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x91\x82\x15a3\x85W_\x92a3)W[P\x90` \x80\x93a%\xD9\x93\x86Q\x95\x84\x87\x95Q\x91\x82\x91\x01\x85\x87\x01^\x84\x01\x90\x83\x82\x01\x90_\x82RQ\x92\x83\x91^\x01\x01_\x81R\x03`\x1F\x19\x81\x01\x83R\x82a5\x89V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEW_a&+\x91\x83Q\x80\x93\x81\x92\x7F\xB4\xD6\xC7\x82\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01aL\xF3V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a3\x1FWa3\nW[P\x81\x80\x80\x80\x80s\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaNZ\xF1a&uaM\x1EV[\x90\x15a2aW\x82\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07vW\x81a&\xCF\x91\x84Q\x80\x93\x81\x92\x7F\xB4\xD6\xC7\x82\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01aL\xF3V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C\x9BWa2LW[PP\x80Qa\x01\xB8\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a28W\x90\x82\x91a\\\xF2\x839\x03\x90\x83\xF0\x80\x15a2-W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FUs\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` Us\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`\x01`\x01`\xA0\x1B\x03a'\xF5a;\xADV[\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"T\x16\x17`\"U_\x80a(\xBEa(\xE6\x84Qa(4``\x82a5\x89V[`*\x81R\x7FGranting AIRDROP_MANAGER_ROLE us` \x82\x01R\x7Fing admin:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x82\x01R\x85Q\x92\x83\x91\x7F1\x9A\xF33\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R\x87`$\x84\x01R`d\x83\x01\x90a3\xCBV[s$<c\xD5\xDB\xCFa\x9E\xE3o\xDE\x7F\xF6=\x15d\xD5f[A`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a5\x89V[` \x81Q\x91\x01jconsole.logZ\xFAPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07vW\x81\x81Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs$<c\xD5\xDB\xCFa\x9E\xE3o\xDE\x7F\xF6=\x15d\xD5f[A`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C\x9BWa2\x18W[P`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x07rW\x82\x91`D\x83\x92\x86Q\x94\x85\x93\x84\x92\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t`\x04\x85\x01R`$\x84\x01RZ\xF1\x80\x15a\x0C\x9BWa2\x03W[P`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x81;\x15a\x07rW\x82\x91`D\x83\x92\x86Q\x94\x85\x93\x84\x92\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t`\x04\x85\x01R`$\x84\x01RZ\xF1\x80\x15a\x0C\x9BWa1\xEEW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07vW\x81\x81Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C\x9BWa1\xD9W[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90\x82Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t`\x04\x82\x01R\x82`$\x82\x01R` \x81`D\x81\x85Z\xFA\x90\x81\x15a\x0C_W\x85\x91a1\xBAW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D\x86Q\x80\x95\x81\x93\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t`\x04\x84\x01R`$\x83\x01RZ\xFA\x91\x82\x15a\x0C_W\x85\x92a1\x99W[P\x15a1\x16W\x15a0\x93W\x82\x90a,g\x83Qa,\x14``\x82a5\x89V[\x84\x81R\x7FSuccessfully granted AIRDROP_MAN` \x82\x01R\x7FAGER_ROLE to holder and contract\x85\x82\x01RaH\x95V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07vW\x82Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C\x9BWa0~W[PP`\x01`\x01`\xA0\x1B\x03` T\x16` `\x01`\x01`\xA0\x1B\x03`\"T\x16`$\x84Q\x80\x94\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x0C5W\x83\x91a0LW[Pi\x01\xB1\xAEMn.\xF5\0\0\0\x80\x91\x10a/\xEFWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07rW\x82\x82Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a/\xE5Wa/\xD0W[PPa-\xED\x82Qa-\xBE\x84\x82a5\x89V[`\x0F\x81R\x7FSetup complete:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaH\x95V[a.:`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x83Qa.\x0B\x85\x82a5\x89V[`\x11\x81R\x7FAirdrop contract:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI`V[a.x\x82Qa.I\x84\x82a5\x89V[`\x0B\x81R\x7FSYND token:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaH\xF2V[`\x01`\x01`\xA0\x1B\x03`\"T\x16\x91a.\xC4\x83\x82Qa.\x95\x84\x82a5\x89V[`\r\x81R\x7FToken holder:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI`V[` `\x01`\x01`\xA0\x1B\x03\x81T\x16\x93`$\x83Q\x80\x96\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x92\x83\x15a/\xC6W\x84\x93a/\x90W[Pa/Ua\x05L\x93\x82Qa/&\x84\x82a5\x89V[`\x15\x81R\x7FToken holder balance:\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI\xC5V[a/a\x81Q\x91\x82a5\x89V[`\x15\x81R\x7FTotal airdrop amount:\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI\xC5V[\x92P` \x83=` \x11a/\xBEW[\x81a/\xAB` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEW\x91Q\x91a/Ua/\x12V[=\x91Pa/\x9EV[\x81Q=\x86\x82>=\x90\xFD[\x81a/\xDA\x91a5\x89V[a\x07rW\x82_a-\xADV[\x84Q=\x84\x82>=\x90\xFD[`d\x82Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInsufficient balance for test\0\0\0`D\x82\x01R\xFD[\x90P` \x81=` \x11a0vW[\x81a0g` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ_a-1V[=\x91Pa0ZV[\x81a0\x88\x91a5\x89V[a\x07vW\x81_a,\xD5V[`\x84\x82Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FFailed to grant AIRDROP_MANAGER_`D\x82\x01R\x7FROLE to airdrop contract\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84\x83Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FFailed to grant AIRDROP_MANAGER_`D\x82\x01R\x7FROLE to token holder\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[a1\xB3\x91\x92P` =` \x11a\x07KWa\x07>\x81\x83a5\x89V[\x90_a+\xF7V[a1\xD3\x91P` =` \x11a\x07KWa\x07>\x81\x83a5\x89V[_a+\x7FV[\x81a1\xE3\x91a5\x89V[a\x07vW\x81_a*\xFCV[\x81a1\xF8\x91a5\x89V[a\x07vW\x81_a*\x91V[\x81a2\r\x91a5\x89V[a\x07vW\x81_a*\x06V[\x81a2\"\x91a5\x89V[a\x07vW\x81_a)~V[PQ\x90=\x90\x82>=\x90\xFD[`$\x85cNH{q`\xE0\x1B\x81R`A`\x04R\xFD[\x81a2V\x91a5\x89V[a\x07vW\x81_a&\xF4V[`\xA4\x82Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`X`$\x82\x01R\x7FStdCheats deployCodeTo(string,by`D\x82\x01R\x7Ftes,uint256,address): Failed to `d\x82\x01R\x7Fcreate runtime bytecode.\0\0\0\0\0\0\0\0`\x84\x82\x01R\xFD[a3\x17\x91\x92P_\x90a5\x89V[_\x90_a&PV[\x82Q=_\x82>=\x90\xFD[\x91P=\x80_\x84>a3:\x81\x84a5\x89V[\x82\x01\x91` \x81\x84\x03\x12a\x04\xCEW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x04\xCEW\x01\x91\x80`\x1F\x84\x01\x12\x15a\x04\xCEW` \x93a3|\x85\x92\x85\x84a%\xD9\x97Q\x91\x01aL\xBDV[\x93P\x93Pa%\x9EV[\x84Q=_\x82>=\x90\xFD[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a3\xACWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a3\x9FV[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a4\rWPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a4\0V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a4wWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a4\xB3\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa3\xCBV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a4hV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a4\xF4WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a5J\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a3\xF0V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a4\xE5V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a5uW`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a5uW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a5uW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x90\x81` \x91\x03\x12a\x04\xCEWQ\x90V[\x91\x90\x82\x01\x80\x92\x11a5\xE4WV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a5uW`\x05\x1B` \x01\x90V[\x90a6\x1A\x82a5\xF8V[a6'`@Q\x91\x82a5\x89V[\x82\x81R`\x1F\x19a67\x82\x94a5\xF8V[\x01\x90` 6\x91\x017V[\x80Q\x15a6NW` \x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q`\x01\x10\x15a6NW`@\x01\x90V[\x80Q`\x02\x10\x15a6NW``\x01\x90V[\x80Q`\x03\x10\x15a6NW`\x80\x01\x90V[\x80Q`\x04\x10\x15a6NW`\xA0\x01\x90V[\x80Q\x82\x10\x15a6NW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x04\xCEWQ\x80\x15\x15\x81\x03a\x04\xCEW\x90V[\x93\x92\x91\x90a6\xFE\x90s\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN\x86R`\x80` \x87\x01R`\x80\x86\x01\x90a3\x8FV[\x90\x84\x82\x03`@\x86\x01R` \x80\x82Q\x93\x84\x81R\x01\x91\x01\x91_[\x81\x81\x10a7(WPP``\x91P\x93\x01RV[\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a7\x16V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a7lW[` \x83\x10\x14a7XWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a7MV[_\x92\x91\x81T\x91a7\x85\x83a7>V[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a7\xDAWP`\x01\x14a7\xA1WPPPV[_\x90\x81R` \x81 \x93\x94P\x91\x92[\x83\x83\x10a7\xC0WP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a7\xAFV[\x90P` \x94\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x92\x91\x92\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10a:/Wa8\x87\x94T\x91\x81\x81\x10a9\xF9W[\x81\x81\x10a9\xC3W[\x81\x81\x10a9\x8DW[\x81\x81\x10a9WW[\x81\x81\x10a9!W[\x81\x81\x10a8\xEBW[\x81\x81\x10a8\xB6W[\x10a8\x89W[P\x03\x83a5\x89V[V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_a8\x7FV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01a8yV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01a8qV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01a8iV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01a8aV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01a8YV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01a8QV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01a8IV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91a81V[\x91\x90\x82\x03\x91\x82\x11a5\xE4WV[`\x08T`\xFF\x16\x80\x15a:\xD8W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a;\xA2W_\x91a;pW[P\x15\x15\x90V[\x90P` \x81=` \x11a;\x9AW[\x81a;\x8B` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ_a;jV[=\x91Pa;~V[`@Q=_\x82>=\x90\xFD[_`@Qa;\xBC`\x80\x82a5\x89V[`\x03\x81R``6` \x83\x017s\x124Vx\x90\x124Vx\x90\x124Vx\x90\x124Vx\x90a;\xE6\x82a6AV[Rs#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01a<\x05\x82a6bV[Rs4Vx\x90\x124Vx\x90\x124Vx\x90\x124Vx\x90\x12a<$\x82a6rV[R` T`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a<\xF8W`\x01`\x01`\xA0\x1B\x03a<N\x82\x85a6\xA2V[Q\x16`@Q\x90\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x90\x81\x15a;\xA2W_\x91a<\xC7W[Pi\x01\xB1\xAEMn.\xF5\0\0\0\x11\x15a<\xADW`\x01\x01a<3V[\x90P`\x01`\x01`\xA0\x1B\x03\x92Pa<\xC2\x91a6\xA2V[Q\x16\x90V[\x90P` \x81=\x82\x11a<\xF0W[\x81a<\xE1` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ_a<\x93V[=\x91Pa<\xD4V[PPP`@\x90\x81Q\x91a=\x0B\x81\x84a5\x89V[`\n\x83R` \x83\x01\x7FtestHolder\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81Q`\n` \x82\x01\x92\x83^_`*\x82\x01R`\n\x81Ra=V`*\x82a5\x89V[Q\x90 \x92\x81Q\x93\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01R` \x84`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x93\x84\x15a3\x1FW_\x94aHQW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEW\x81Q\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x81\x80a>\x18`\x01`\x01`\xA0\x1B\x03\x89\x16\x95\x86`\x04\x84\x01R\x87`$\x84\x01R`D\x83\x01\x90a3\xCBV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aHGWaH2W[P\x82\x80\x83Q` \x81\x01\x90\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x84`$\x82\x01R`$\x81Ra>\x7F`D\x82a5\x89V[Q\x90s\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaNZ\xFAPa>\xB3a>\xA4aM\x1EV[` \x80\x82Q\x83\x01\x01\x91\x01a5\xC8V[Ps\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x11T\x16\x17`\x11Ucp\xA0\x821\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0`\x0FT\x16\x17`\x0FU`\x0ETh\x01\0\0\0\0\0\0\0\0\x81\x10\x15aH\x1EW`\x01\x81\x01\x80`\x0EU\x81\x10\x15aH\nW`\x0E\x84R` \x84 \x01U`\x01`\x01`\xA0\x1B\x03`\x11T\x16`\x0FT`\xE0\x1B`\x10T\x90a?kaMdV[\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a?\xA8\x85`\x01`\x01`\xA0\x1B\x03\x16_R`\r` R`@_ \x90V[\x91\x16\x90\x81\x87R` R\x84\x86 \x85Q` \x81\x01\x90a?\xD8\x81a?\xCA\x88\x88\x86aMMV[\x03`\x1F\x19\x81\x01\x83R\x82a5\x89V[Q\x90 \x87R` R`\xFF`\x03\x86\x88 \x01T\x16\x15aG\xFCW[a@\x0B\x84`\x01`\x01`\xA0\x1B\x03\x16_R`\r` R`@_ \x90V[\x90\x86R` Ra?\xCAa@+\x85\x87 \x93\x86Q\x92\x83\x91` \x83\x01\x95\x86aMMV[Q\x90 \x84R` R\x81\x83 \x90`\x01\x82\x01T\x91`\x02\x81\x01Ta@L\x81\x85a5\xD7V[aF\x15W[\x81T\x85Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x94\x90\x91\x90` \x86`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x95\x86\x15aF\x0BW\x88\x96aE\xD7W[P`\x01\x90\x82\x01a\x01\0\x03\x1B_\x19\x01\x81\x1B\x19\x85\x16\x91sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15aE\xD3W\x86Q\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91Ri\x03c\\\x9A\xDC]\xEA\0\0\0\x90\x91\x1B\x91\x90\x91\x17`D\x82\x01R\x85\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aE\xC9W\x90\x86\x91aE\xB4W[PP\x84\x80` `\x0FT`\xE0\x1BaA\xD2`$aA\x85`\raN=V[\x8AQ\x94\x85\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01\x96\x16\x86R\x80Q\x91\x82\x91\x01\x84\x84\x01^\x81\x01\x86\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x84R\x83a5\x89V[`\x01`\x01`\xA0\x1B\x03`\x11T\x16\x91Q\x91Z\xFAaA\xEBaM\x1EV[\x90`\x10T\x80`\x05\x1B\x90\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03aE\xA0W\x87\x92\x80Q` \x81\x11_\x14aE\x9AWP` \x90[\x89\x92[\x82\x84\x10aE\x12WPPPP\x15\x90\x81\x15aD\xFDW[PaC\xD2WPPPP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x11T\x16`\x11U\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0`\x0FT\x16`\x0FU`\x0ET\x81`\x0EU\x80aC\x90W[P\x80`\x10U\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x13T\x16`\x13UaB\xE4`\x14Ta7>V[\x80aB\xEEWPP\x90V[`\x1F\x81\x11`\x01\x14aC\x01WP`\x14U[\x90V[`\x14\x82R`\x1F\x01`\x05\x1C\x7F\xCEm{R\x82\xBD\x9A6a\xAE\x06\x1F\xEE\xD1\xDB\xDANR\xAB\x07;\x1F\x92\x85\xBEn\x15]\x9C8\xD4\xEC\x01\x7F\xCEm{R\x82\xBD\x9A6a\xAE\x06\x1F\xEE\xD1\xDB\xDANR\xAB\x07;\x1F\x92\x85\xBEn\x15]\x9C8\xD4\xED[\x81\x81\x10aC\x85WPP`\x14\x80\x82R\x81\x90U\x7F\xCEm{R\x82\xBD\x9A6a\xAE\x06\x1F\xEE\xD1\xDB\xDANR\xAB\x07;\x1F\x92\x85\xBEn\x15]\x9C8\xD4\xECU\x90V[_\x81U`\x01\x01aCOV[`\x0E\x82R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x90\x81\x01\x90[\x81\x81\x10aC\xC7WPaB\xACV[\x82\x81U`\x01\x01aC\xBAV[T\x84\x91sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07rWaD@\x85Q\x94\x85\x93\x84\x93\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01`@\x91\x94\x93\x92`\x01`\x01`\xA0\x1B\x03``\x83\x01\x96\x16\x82R` \x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C5WaD\xE8W[`\x84\x82Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FstdStorage find(StdStorage): Fai`D\x82\x01R\x7Fled to write value.\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[aD\xF3\x83\x80\x92a5\x89V[a\x07vW\x81aDeV[i\x03c\\\x9A\xDC]\xEA\0\0\0\x91P\x14\x15_aBFV[\x90\x91\x92\x94aE \x86\x83a5\xD7V[\x83Q\x81\x10\x15aE\x86W` \x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x85\x01\x01Q\x16\x86`\x03\x1B\x87\x81\x04`\x08\x14\x88\x15\x17\x15aErW\x1C\x17\x94`\x01\x01\x92\x91\x90aB2V[`$\x8DcNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`$\x8CcNH{q`\xE0\x1B\x81R`2`\x04R\xFD[\x90aB/V[`$\x88cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[\x81aE\xBE\x91a5\x89V[a\x06\x8AW\x84_aAjV[\x85Q=\x88\x82>=\x90\xFD[\x87\x80\xFD[\x90\x95P` \x81=` \x11aF\x03W[\x81aE\xF3` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ\x94\x81a@\xBDV[=\x91PaE\xE6V[\x87Q=\x8A\x82>=\x90\xFD[aF\x1F\x81\x85a5\xD7V[a\x01\0\x03a\x01\0\x81\x11aG\xE8W`\xFF\x81\x11aG\xE8W`\x01\x90\x1B\x85Q\x7Fi\0\xA3\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81`\x04\x82\x01R\x87\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15aF\x0BW\x88\x91aG\x84W[P\x90aG,`j` \x93\x89Q\x94\x85\x91\x7FstdStorage find(StdStorage): Pac\x82\x84\x01R\x7Fked slot. We can't fit value gre\x8C\x84\x01R\x7Fater than \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x01R\x80Q\x91\x82\x91\x01\x84\x84\x01^\x81\x01\x8B\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x84R\x83a5\x89V[i\x03c\\\x9A\xDC]\xEA\0\0\0\x10\x15aGCWPa@QV[aG\x80\x90\x86Q\x91\x82\x91\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a3\xCBV[\x03\x90\xFD[\x90P=\x80\x89\x83>aG\x95\x81\x83a5\x89V[\x81\x01\x90` \x81\x83\x03\x12aG\xE0W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11aG\xE4W\x01\x81`\x1F\x82\x01\x12\x15aG\xE0W`jaG\xD7` \x94\x93\x83\x86aG,\x95Q\x91\x01aL\xBDV[\x92\x93PPaF\x8DV[\x88\x80\xFD[\x89\x80\xFD[`$\x87cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[aH\x04aP\xA6V[Pa?\xF0V[`$\x84cNH{q`\xE0\x1B\x81R`2`\x04R\xFD[`$\x84cNH{q`\xE0\x1B\x81R`A`\x04R\xFD[aH?\x91\x93P_\x90a5\x89V[_\x91_a>=V[\x83Q=_\x82>=\x90\xFD[\x90\x93P` \x81=` \x11aH\x8DW[\x81aHm` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x04\xCEW\x92_a=\xB0V[=\x91PaH`V[_a?\xCAaH\xDB\x82\x93`@Q\x92\x83\x91\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R` `$\x84\x01R`D\x83\x01\x90a3\xCBV[` \x81Q\x91\x01jconsole.logZ\xFAPV[_aI8aH\xDB\x82\x93`@Q\x92\x83\x91\x7F1\x9A\xF33\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R`@`$\x84\x01R`d\x83\x01\x90a3\xCBV[s\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a5\x89V[aI\xB0aH\xDB_\x93\x92\x84\x93`\x01`\x01`\xA0\x1B\x03`@Q\x94\x85\x93\x7F1\x9A\xF33\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x86\x01R`@`$\x86\x01R`d\x85\x01\x90a3\xCBV[\x91\x16`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a5\x89V[aJ\raH\xDB_\x93\x92\x84\x93`@Q\x93\x84\x92\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a3\xCBV[\x90`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a5\x89V[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEW_\x91aJ\x88`@Q\x94\x85\x93\x84\x93\x7F\x88\xB4L\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01R`$\x84\x01R```D\x84\x01R`d\x83\x01\x90a3\xCBV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a;\xA2WaJ\xAEWPV[_a8\x87\x91a5\x89V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEWaJ\x88\x91_\x91`@Q\x93\x84\x92\x83\x92\x7F\xA3N\xDC\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x15\x15`\x04\x84\x01R`@`$\x84\x01R`D\x83\x01\x90a3\xCBV[`@QaK*`\xC0\x82a5\x89V[`\x05\x81R_[`\xA0\x81\x10aL\x9AWP`@QaKE\x81a5YV[s\x124Vx\x90\x124Vx\x90\x124Vx\x90\x124Vx\x90\x81Rh65\xC9\xAD\xC5\xDE\xA0\0\0` \x82\x01RaKt\x82a6AV[RaK~\x81a6AV[P`@QaK\x8B\x81a5YV[s#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01\x81Rhlk\x93[\x8B\xBD@\0\0` \x82\x01RaK\xBA\x82a6bV[RaK\xC4\x81a6bV[P`@QaK\xD1\x81a5YV[s4Vx\x90\x124Vx\x90\x124Vx\x90\x124Vx\x90\x12\x81RhQP\xAE\x84\xA8\xCD\xF0\0\0` \x82\x01RaL\0\x82a6rV[RaL\n\x81a6rV[P`@QaL\x17\x81a5YV[sEg\x89\x01#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01#\x81Rh\xA2\xA1]\tQ\x9B\xE0\0\0` \x82\x01RaLF\x82a6\x82V[RaLP\x81a6\x82V[P`@QaL]\x81a5YV[sVx\x90\x124Vx\x90\x124Vx\x90\x124Vx\x90\x124\x81Rh\x1B\x1A\xE4\xD6\xE2\xEFP\0\0` \x82\x01RaL\x8C\x82a6\x92V[RaL\x96\x81a6\x92V[P\x90V[` \x90`@QaL\xA9\x81a5YV[_\x81R_\x83\x82\x01R\x82\x82\x85\x01\x01R\x01aK0V[\x92\x91\x92aL\xC9\x82a5\xACV[\x91aL\xD7`@Q\x93\x84a5\x89V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x04\xCEW\x82\x81` \x93\x84_\x96\x01^\x01\x01RV[\x90`@aB\xFE\x92s\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN\x81R\x81` \x82\x01R\x01\x90a3\xCBV[=\x15aMHW=\x90aM/\x82a5\xACV[\x91aM=`@Q\x93\x84a5\x89V[\x82R=_` \x84\x01>V[``\x90V[` \x92\x91\x90\x83\x91\x80Q\x92\x83\x91\x01\x82^\x01\x90\x81R\x01\x90V[aMo`\x14Ta7>V[aN+W`@Q\x90\x81\x82` `\x0ET\x92\x83\x81R\x01`\x0E_R` _ \x92_[\x81\x81\x10aN\x12WPPaM\xA3\x92P\x03\x83a5\x89V[\x81Q\x80`\x05\x1B\x90\x80\x82\x04` \x14\x90\x15\x17\x15a5\xE4W`\x1F\x19aM\xDDaM\xC7\x83a5\xACV[\x92aM\xD5`@Q\x94\x85a5\x89V[\x80\x84Ra5\xACV[\x016` \x83\x017_[\x83Q\x81\x10\x15aN\rW\x80aM\xFC`\x01\x92\x86a6\xA2V[Q` \x82`\x05\x1B\x85\x01\x01R\x01aM\xE6V[P\x91PV[\x84T\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01aM\x8EV[`@QaB\xFE\x81a\x08K\x81`\x14a7vV[`\x07\x81\x01\x90aNL\x82Ta7>V[aN\xF1W`\x01\x91P\x01\x90`@Q\x80\x83` \x82\x95T\x93\x84\x81R\x01\x90_R` _ \x92_[\x81\x81\x10aN\xD8WPPaN\x84\x92P\x03\x83a5\x89V[\x81Q\x80`\x05\x1B\x90\x80\x82\x04` \x14\x90\x15\x17\x15a5\xE4W`\x1F\x19aN\xA8aM\xC7\x83a5\xACV[\x016` \x83\x017_[\x83Q\x81\x10\x15aN\rW\x80aN\xC7`\x01\x92\x86a6\xA2V[Q` \x82`\x05\x1B\x85\x01\x01R\x01aN\xB1V[\x84T\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01aNoV[PaB\xFEa\x08K\x91`@Q\x92\x83\x80\x92a7vV[\x90_\x80` `\x02\x85\x01T`\xE0\x1BaOn`$aO \x88aN=V[`@Q\x94\x85\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01\x96\x16\x86R\x80Q\x91\x82\x91\x01\x84\x84\x01^\x81\x01\x86\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x84R\x83a5\x89V[`\x01`\x01`\xA0\x1B\x03`\x04\x87\x01T\x16\x91Q\x91Z\xFA`\x03aO\x8BaM\x1EV[\x93\x01T\x80`\x05\x1B\x90\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a5\xE4W_\x93\x80Q` \x81\x11_\x14aPCWP` \x90[_\x92[\x82\x84\x10aO\xE0WPPPP\x91\x90V[\x90\x91\x92\x95aO\xEE\x87\x83a5\xD7V[\x83Q\x81\x10\x15a6NW` \x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x85\x01\x01Q\x16\x90\x87`\x03\x1B\x91\x88\x83\x04`\x08\x14\x89\x15\x17\x15a5\xE4W`\x01\x92\x1C\x17\x96\x01\x92\x91\x90aO\xD1V[\x90aO\xCEV[\x90\x80`\x1F\x83\x01\x12\x15a\x04\xCEW\x81QaP`\x81a5\xF8V[\x92aPn`@Q\x94\x85a5\x89V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04\xCEW` \x01\x90[\x82\x82\x10aP\x96WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01aP\x89V[_`\x01`\x01`\xA0\x1B\x03`\x11T\x16\x90`\x0FT`\xE0\x1B`\x10T\x90aP\xC8`\raN=V[\x90\x84_R`\r` R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@_ \x91\x16\x90\x81_R` R`@_ `@Q` \x81\x01\x90aQ\x1A\x81a?\xCA\x88\x88\x86aMMV[Q\x90 _R` R`\xFF`\x03`@_ \x01T\x16aZ\xC9Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEW`@Q\x7F&l\xF1\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a;\xA2WaZ\xB4W[PaQ\xA5`\raO\x05V[\x90P`@Q\x7Fe\xBC\x94\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x86`\x04\x82\x01R\x85\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x16\xE1W\x86\x91aZNW[P\x80Q\x80aR\x8AW`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FstdStorage find(StdStorage): No `D\x82\x01R\x7Fstorage use detected for target.`d\x82\x01R\xFD[\x80\x15aG\xE8W\x90_\x19aR\xEF\x92\x01\x90` aR\xA5\x83\x83a6\xA2V[Q`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x93\x84\x90\x81\x90`D\x82\x01\x90V[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x92\x83\x15a\x0E\xE4W\x88\x93aZ\x1BW[P\x82\x15aY\xC3W[aS(\x82\x82a6\xA2V[Q`\x11T`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01\x81\x90R`$\x82\x01\x83\x90R\x91\x90` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15aY\xB8W\x8B\x91aY\x87W[PaS\xA3`\raO\x05V[\x91\x90\x93\x82\x15_\x14aY\x80W_\x19\x90[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15aYgW`@Q\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x81\x01\x91\x90\x91R\x8C\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aX\xE4W\x90\x8D\x91aYkW[PPaTJ`\raO\x05V[\x93\x90P`\x01`\x01`\xA0\x1B\x03`\x11T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15aYgW`@Q\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x81\x01\x91\x90\x91R\x8B\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aY\\W\x90\x8C\x91aYCW[PP\x82aY8W[PP\x15aY1W\x87\x93\x88\x93`\xFF`\x13T\x16aWaW[`\x01\x86\x86\x01a\x01\0\x03\x1B_\x19\x01\x85\x1B\x16\x84\x1C\x81\x03aWXWP\x90aU\x8A\x91\x7F\x9C\x95U\xB1\xE3\x10.<\xF4\x8FB}y\xCBg\x8F]\x9B\xD1\xED\n\xD5t8\x94a\xE2U\xF9Qp\xED`\x80\x8B\x89a?\xCAaU_\x8D`@Q\x92\x83\x91` \x83\x01\x95\x86aMMV[Q\x90 aUl\x86\x86a6\xA2V[Q\x90`@Q\x92\x83R\x8A` \x84\x01R`@\x83\x01R``\x82\x01R\xA1a6\xA2V[Q\x90`@Q\x91`\x80\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aWDW\x90`\x03\x93\x92\x91`@R\x82R` \x82\x01\x93\x84R`@\x82\x01\x90\x81R``\x82\x01\x93`\x01\x85R\x89\x89R`\r` R`@\x89 \x86\x8AR` R`@\x89 `@Q` \x81\x01\x90aU\xF6\x81a?\xCA\x8D\x8D\x86aMMV[Q\x90 \x8AR` R`@\x89 \x92Q\x83UQ`\x01\x83\x01UQ`\x02\x82\x01U\x01\x90Q\x15\x15`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83T\x16\x91\x16\x17\x90U\x84\x84R`\r` R`@\x84 \x81\x85R` R`@\x84 `@Q` \x81\x01\x90aVo\x81a?\xCA\x88\x88\x86aMMV[Q\x90 \x85R` R`\xFF`\x03`@\x86 \x01T\x16\x15aV\xC0W`@\x94\x84R`\r` R\x84\x84 \x90\x84R` Ra?\xCAaV\xB4\x85\x85 \x93\x86Q\x92\x83\x91` \x83\x01\x95\x86aMMV[Q\x90 \x82R` R \x90V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FstdStorage find(StdStorage): Slo`D\x82\x01R\x7Ft(s) not found.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`$\x89cNH{q`\xE0\x1B\x81R`A`\x04R\xFD[\x93P\x91PaR\x8AV[\x94PaW\xCD\x93PaWr\x83\x83a6\xA2V[Q\x94` \x86`\x01`\x01`\xA0\x1B\x03`\x11T\x16`@Q\x97\x88\x92\x83\x92\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x94\x85\x15aY&W\x8A\x95aX\xF3W[PaX\0\x86`\ra[\nV[\x95\x90\x96aX\x0E\x81`\ra\\\x08V[\x92\x90\x91`\x01`\x01`\xA0\x1B\x03`\x11T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15aX\xEFW`@Q\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x81\x01\x91\x90\x91R\x8C\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aX\xE4W\x90\x8D\x91aX\xCBW[PP\x87aX\xC3W[P\x94\x95aU\x04WP\x93P\x91PaR\x8AV[\x96P_aX\xB2V[\x81aX\xD5\x91a5\x89V[aX\xE0W\x8B_aX\xAAV[\x8B\x80\xFD[`@Q=\x8F\x82>=\x90\xFD[\x8E\x80\xFD[\x90\x94P` \x81=\x82\x11aY\x1EW[\x81aY\x0E` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ\x93_aW\xF4V[=\x91PaY\x01V[`@Q=\x8C\x82>=\x90\xFD[\x91PaR\x8AV[\x14\x15\x90P_\x80aT\xEEV[\x81aYM\x91a5\x89V[aYXW\x8A_aT\xE6V[\x8A\x80\xFD[`@Q=\x8E\x82>=\x90\xFD[\x8D\x80\xFD[\x81aYu\x91a5\x89V[aX\xE0W\x8B_aT>V[\x8C\x90aS\xB2V[\x90P` \x81=\x82\x11aY\xB0W[\x81aY\xA1` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ_aS\x98V[=\x91PaY\x94V[`@Q=\x8D\x82>=\x90\xFD[\x7F\x08\x0F\xC4\xA9f \xC4F.p[#\xF3FA?\xE3yk\xB6<o\x8D\x85\x91\xBA\xEC\x0E#\x15w\xA5aZ\x13aY\xF1\x84\x84a6\xA2V[Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8E\x16\x81R` \x81\x01\x92\x90\x92R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1aS\x1EV[\x90\x92P` \x81=\x82\x11aZFW[\x81aZ6` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ\x91_aS\x16V[=\x91PaZ)V[\x90P=\x80\x87\x83>aZ_\x81\x83a5\x89V[\x81\x01`@\x82\x82\x03\x12aZ\xB0W\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11aE\xD3W\x81aZ\x88\x91\x84\x01aPIV[\x91` \x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11aG\xE0WaZ\xA9\x92\x91\x01aPIV[P_aQ\xFEV[\x86\x80\xFD[aZ\xC1\x91\x94P_\x90a5\x89V[_\x92_aQ\x9AV[\x91\x93\x90\x92P_R`\r` R`@_ \x90_R` Ra?\xCAaZ\xFB`@_ \x93`@Q\x92\x83\x91` \x83\x01\x95\x86aMMV[Q\x90 _R` R`@_ \x90V[\x91\x90_[a\x01\0\x81\x10a[!WPP\x90P_\x90_\x90V[\x80`\xFF\x03`\xFF\x81\x11a5\xE4W`\x01\x90\x1B`\x01`\x01`\xA0\x1B\x03`\x04\x86\x01T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEW`@Q\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x81\x01\x91\x90\x91R_\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a;\xA2Wa[\xF8W[Pa[\xD2\x84aO\x05V[\x81a[\xEEW[Pa[\xE5W`\x01\x01a[\x0EV[\x92PP`\x01\x91\x90V[\x90P\x15\x15_a[\xD8V[_a\\\x02\x91a5\x89V[_a[\xC8V[\x91\x90_[a\x01\0\x81\x10a\\\x1FWPP\x90P_\x90_\x90V[`\x01\x81\x1B`\x01`\x01`\xA0\x1B\x03`\x04\x86\x01T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEW`@Q\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x81\x01\x91\x90\x91R_\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a;\xA2Wa\\\xE1W[Pa\\\xC4\x84aO\x05V[\x81a\\\xD7W[Pa[\xE5W`\x01\x01a\\\x0CV[\x90P\x15\x15_a\\\xCAV[_a\\\xEB\x91a5\x89V[_a\\\xBAV\xFE`\x80\x80`@R4`\x15Wa\x01\x9E\x90\x81a\0\x1A\x829\xF3[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1Cc\x82\x94z\xBE\x14a\0$W_\x80\xFD[`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0\xC1W`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\0\xC1W`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xC1Wa\0\x8E\x906\x90`\x04\x01a\0\xC5V[`D5\x92\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11a\0\xC1Wa\0\xB3a\0\xBF\x946\x90`\x04\x01a\0\xC5V[\x92\x90\x91`d5\x94a\0\xF6V[\0[_\x80\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\0\xC1W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xC1W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\0\xC1WV[\x91\x80\x93\x95\x91\x94\x03a\0\xC1W\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R0`$R`DR_\x80`d\x81\x80\x85Z\xF1\x15a\0\xC1W\x91\x90\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x05\x1B\x81\x01\x92\x81\x03\x90[\x805`\x04R\x81\x81\x035`$R_\x80`d\x81\x80\x87Z\xF1\x15a\0\xC1W` \x01\x91\x83\x83\x10\x15a\x01\x98W\x91a\x01lV[PPPPV",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f905f3560e01c9081630a9254e4146124aa575080631b7986d2146121345780631ed7831c146120b65780632ade388014611eba5780633884d63514611e905780633e5e3c2314611e125780633f7286f414611d94578063420a83e714611d6d57806359c620c614611aa05780636213821d14611a7a57806366d9a9a01461193557806381a8de7e1461184257806385226c81146117b057806385a468f714610d915780638a54252114610d56578063916a17c614610cac57806394a2a5df14610926578063a217fddf1461090a578063b0464fdc14610860578063b5508aa9146107c7578063ba414fa6146107a2578063bff8600f14610213578063e20c9f711461017d578063e52a2f1f146101565763fa7626d414610131575f80fd5b34610153578060031936011261015357602060ff601f54166040519015158152f35b80fd5b503461015357806003193601126101535760206001600160a01b0360215416604051908152f35b503461015357806003193601126101535760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106101f4576101f0856101dc81870382613589565b60405191829160208352602083019061338f565b0390f35b82546001600160a01b03168452602090930192600192830192016101c5565b503461015357806003193601126101535761022c614b1c565b8051600381019081811161078e5760020190811161077a576003900490826001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077657604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105645761075d575b5060208054601f546040517f095ea7b30000000000000000000000000000000000000000000000000000000081526001600160a01b0360089290921c821660048201526901b1ae4d6e2ef50000006024820152938492604492849291165af1801561075257610725575b50825b82811061056f5783737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561015357806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105645761054f575b50506103ae614b1c565b6020546001600160a01b0316825b82518110156104e5576001600160a01b036103d782856136a2565b51511690604051917f70a082310000000000000000000000000000000000000000000000000000000083526004830152602082602481865afa9182156104da5785926104a1575b5061049b600192602061043184886136a2565b51015111156060906104466040519283613589565b602d82527f526563697069656e742073686f756c642068617665207265636569766564206160208301527f697264726f7020616d6f756e74000000000000000000000000000000000000006040830152614ab8565b016103bc565b91506020823d82116104d2575b816104bb60209383613589565b810103126104ce5790519061049b61041e565b5f80fd5b3d91506104ae565b6040513d87823e3d90fd5b8361054c6040516104f7606082613589565b602481527f42617463682061697264726f7020636f6d706c6574656420737563636573736660208201527f756c6c79000000000000000000000000000000000000000000000000000000006040820152614895565b80f35b8161055991613589565b6101535780826103a4565b6040513d84823e3d90fd5b60038102908082046003148115171561071157600382018083116106fd576105a5838288939087518091116106f5575b50613abc565b926105af84613610565b6105b885613610565b91839084905b87821061068e5750506001600160a01b03601f5460081c16803b1561068a5761061a93858094604051968795869485937f82947abe000000000000000000000000000000000000000000000000000000008552600485016136ce565b03925af1801561056457610675575b5060019261066f9150604061064081519182613589565b601f81527f4261746368206578656375746564207769746820726563697069656e74733a0060208201526149c5565b01610332565b8161067f91613589565b61068a57845f610629565b8480fd5b90916106ed6001916001600160a01b036106b16106ab87876135d7565b8d6136a2565b5151166106be86886136a2565b5260206106ce6106ab87876135d7565b5101516106db86896136a2565b526106e685886136a2565b51906135d7565b9201906105be565b90505f61059f565b602486634e487b7160e01b81526011600452fd5b602485634e487b7160e01b81526011600452fd5b6107469060203d60201161074b575b61073e8183613589565b8101906136b6565b61032f565b503d610734565b6040513d86823e3d90fd5b8161076791613589565b61077257825f6102c5565b8280fd5b5080fd5b602483634e487b7160e01b81526011600452fd5b602484634e487b7160e01b81526011600452fd5b503461015357806003193601126101535760206107bd613ac9565b6040519015158152f35b50346101535780600319360112610153576019546107e4816135f8565b916107f26040519384613589565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b83831061083457604051806101f08782613445565b6001602081926040516108528161084b8189613776565b0382613589565b81520192019201919061081f565b5034610153578060031936011261015357601c5461087d816135f8565b9161088b6040519384613589565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b8383106108cd57604051806101f087826134c2565b600260206001926040516108e081613559565b6001600160a01b0386541681526108f8858701613816565b838201528152019201920191906108b8565b5034610153578060031936011261015357602090604051908152f35b503461015357806003193601126101535761093f614b1c565b90604080519261094f8285613589565b60018452602061099d601f198401928336848901378451936109718686613589565b6001855236848601376001600160a01b0361098b82613641565b51511661099788613641565b52613641565b5101516109a982613641565b526001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ca8578251907f06447d560000000000000000000000000000000000000000000000000000000082526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c9b57908491610c86575b50506001600160a01b0360205416936001600160a01b03601f5460081c1694610a4f83613641565b51955f1987019687116106fd5784517f095ea7b30000000000000000000000000000000000000000000000000000000081526001600160a01b039190911660048201526024810196909652939485946020908290604490829089905af18015610c5f57610c69575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c5a5782517ff4844814000000000000000000000000000000000000000000000000000000008152848160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c5f57908591610c45575b50506001600160a01b03601f5460081c16610b3d83613641565b5190803b15610c4157610b82938680948751968795869485937f82947abe000000000000000000000000000000000000000000000000000000008552600485016136ce565b03925af18015610c3557908391610c20575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c1d578051907f90c5013b000000000000000000000000000000000000000000000000000000008252828260048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610c145750610c035750f35b81610c0d91613589565b6101535780f35b513d84823e3d90fd5b50fd5b81610c2a91613589565b610c1d57815f610b94565b505051903d90823e3d90fd5b8580fd5b81610c4f91613589565b610c5a57835f610b23565b505050fd5b84513d87823e3d90fd5b610c819060203d60201161074b5761073e8183613589565b610ab7565b81610c9091613589565b61077257825f610a27565b50505051903d90823e3d90fd5b8380fd5b5034610153578060031936011261015357601d54610cc9816135f8565b91610cd76040519384613589565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310610d1957604051806101f087826134c2565b60026020600192604051610d2c81613559565b6001600160a01b038654168152610d44858701613816565b83820152815201920192019190610d04565b503461015357806003193601126101535760206040517f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a6748152f35b5034610153578060031936011261015357610daa614b1c565b90610db58251613610565b610dbf8351613610565b8291825b8551841015610e1e57610e166001916001600160a01b03610de4878a6136a2565b515116610df187866136a2565b526020610dfe878a6136a2565b510151610e0b87876136a2565b526106e686866136a2565b930192610dc3565b90919250610e2c8251613610565b9184916001600160a01b0360205416925b8251811015610eef576001600160a01b03610e5882856136a2565b511690604051917f70a082310000000000000000000000000000000000000000000000000000000083526004830152602082602481885afa8015610ee4578890610eb2575b60019250610eab82886136a2565b5201610e3d565b506020823d8211610edc575b81610ecb60209383613589565b810103126104ce5760019151610e9d565b3d9150610ebe565b6040513d8a823e3d90fd5b509391926024959360206001600160a01b036022541691604051988980927f70a082310000000000000000000000000000000000000000000000000000000082528560048301525afa9687156104da57859761177c575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561068a57604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152848160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104da57908591611767575b505060208054601f546040517f095ea7b30000000000000000000000000000000000000000000000000000000081526001600160a01b0360089290921c821660048201526024810189905292918391168188816044810103925af180156104da57906110a093929161174a575b506001600160a01b036020541660206001600160a01b03602254166001600160a01b03601f5460081c169283916040518098819482937fdd62ed3e000000000000000000000000000000000000000000000000000000008452600484019092916001600160a01b0360209181604085019616845216910152565b03915afa9384156116e1578694611716575b5061111d60609488604051916110c88884613589565b602383527f416c6c6f77616e63652073686f756c6420657175616c20746f74616c20616d6f60208401527f756e7400000000000000000000000000000000000000000000000000000000006040840152614a21565b803b15610c41578560405180927f82947abe00000000000000000000000000000000000000000000000000000000825281838161115f8d8c8b600485016136ce565b03925af180156116e157908691611701575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561068a576040517f90c5013b000000000000000000000000000000000000000000000000000000008152858160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156116e1579086916116ec575b5096956001600160a01b0360205416975b83518110156113625761121161120682856136a2565b516106e683896136a2565b908960206001600160a01b0361122784896136a2565b51166024604051809481937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa908115611357578991611325575b506040516001936112d49261127f8a84613589565b603383527f526563697069656e742062616c616e63652073686f756c6420696e637265617360208401527f652062792061697264726f7020616d6f756e74000000000000000000000000006040840152614a21565b61131f6112e182896136a2565b5160406112f081519182613589565b601881527f526563697069656e742072656365697665642053594e443a000000000000000060208201526149c5565b016111f0565b90506020813d821161134f575b8161133f60209383613589565b810103126104ce5751600161126a565b3d9150611332565b6040513d8b823e3d90fd5b86848382888c8e6001600160a01b0360225416916040517f70a08231000000000000000000000000000000000000000000000000000000008152836004820152602081602481865afa9182156116e157879187936116a2575b5061149094926113d16020959361143393613abc565b604051916113df8984613589565b603483527f546f6b656e20686f6c6465722062616c616e63652073686f756c642064656372878401527f6561736520627920746f74616c20616d6f756e740000000000000000000000006040840152614a21565b6001600160a01b03601f5460081c16916040518095819482937fdd62ed3e000000000000000000000000000000000000000000000000000000008452600484019092916001600160a01b0360209181604085019616845216910152565b03915afa908115611697578391611662575b50604051916114b18184613589565b602683527f416c6c6f77616e63652073686f756c64206265207a65726f206166746572206160208401527f697264726f7000000000000000000000000000000000000000000000000000006040840152737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ca857839161156660405194859384937f88b44c850000000000000000000000000000000000000000000000000000000085526004850152856024850152604484015260648301906133cb565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa80156105645761164d575b509161054c92506116106040926115d584516115a68682613589565b601e81527f41697264726f70206578656375746564207375636365737366756c6c793a00006020820152614895565b83516115e18582613589565b601281527f546f74616c2064697374726962757465643a000000000000000000000000000060208201526149c5565b519061161e81519182613589565b600b81527f526563697069656e74733a00000000000000000000000000000000000000000060208201526149c5565b8161165791613589565b61077257828461158a565b9250506020823d60201161168f575b8161167e60209383613589565b810103126104ce57849151866114a2565b3d9150611671565b6040513d85823e3d90fd5b949250929550506020833d6020116116d9575b816116c260209383613589565b810103126104ce57915187949186906114336113bb565b3d91506116b5565b6040513d88823e3d90fd5b816116f691613589565b61068a5784886111df565b8161170b91613589565b61068a578488611171565b9093506020813d602011611742575b8161173260209383613589565b810103126104ce575192886110b2565b3d9150611725565b6117629060203d60201161074b5761073e8183613589565b611026565b8161177191613589565b610ca8578387610fb9565b9096506020813d6020116117a8575b8161179860209383613589565b810103126104ce57519587610f46565b3d915061178b565b5034610153578060031936011261015357601a546117cd816135f8565b916117db6040519384613589565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b83831061181d57604051806101f08782613445565b6001602081926040516118348161084b8189613776565b815201920192019190611808565b503461015357806003193601126101535761054c6001600160a01b03601f5460081c166118ce604051611876606082613589565b602381527f41697264726f7020636f6e74726163742073686f756c64206265206465706c6f60208201527f79656400000000000000000000000000000000000000000000000000000000006040820152821515614ab8565b3b1515604051906118e0606083613589565b602182527f41697264726f7020636f6e74726163742073686f756c64206861766520636f6460208301527f65000000000000000000000000000000000000000000000000000000000000006040830152614ab8565b5034610153578060031936011261015357601b54611952816135f8565b61195f6040519182613589565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b838310611a3757868587604051928392602084019060208552518091526040840160408260051b8601019392905b8282106119cc57505050500390f35b91936020611a27827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0600195979984950301865288519083611a1783516040845260408401906133cb565b92015190848184039101526133f0565b96019201920185949391926119bd565b60026020600192604051611a4a81613559565b604051611a5b8161084b818a613776565b8152611a68858701613816565b8382015281520192019201919061198f565b503461015357806003193601126101535760206001600160a01b03815416604051908152f35b50346101535780600319360112610153578060046040611af48151611ac58382613589565b601581527f546f6b656e204e616d653a2053796e64696361746500000000000000000000006020820152614895565b611b328151611b038382613589565b601281527f546f6b656e2053796d626f6c3a2053594e4400000000000000000000000000006020820152614895565b611b708151611b418382613589565b601281527f546f6b656e20446563696d616c733a20313800000000000000000000000000006020820152614895565b611bae8151611b7f8382613589565b600e81527f546f6b656e20416464726573733a00000000000000000000000000000000000060208201526148f2565b60206001600160a01b038154168251938480927f18160ddd0000000000000000000000000000000000000000000000000000000082525afa918215611d63578392611d2c575b50611c34828251611c058482613589565b600d81527f546f74616c20537570706c793a0000000000000000000000000000000000000060208201526149c5565b805191611c42606084613589565b602783527f546f74616c20737570706c792073686f756c6420626520393230206d696c6c6960208401527f6f6e2053594e440000000000000000000000000000000000000000000000000082840152737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c5a57611d04928491835194859283927f88b44c8500000000000000000000000000000000000000000000000000000000845260048401526b02f90193ef3075fa9800000060248401526060604484015260648301906133cb565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115610c145750610c035750f35b925090506020823d602011611d5b575b81611d4960209383613589565b810103126104ce57829151905f611bf4565b3d9150611d3c565b81513d85823e3d90fd5b503461015357806003193601126101535760206001600160a01b0360225416604051908152f35b503461015357806003193601126101535760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110611df3576101f0856101dc81870382613589565b82546001600160a01b0316845260209093019260019283019201611ddc565b503461015357806003193601126101535760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b818110611e71576101f0856101dc81870382613589565b82546001600160a01b0316845260209093019260019283019201611e5a565b503461015357806003193601126101535760206001600160a01b03601f5460081c16604051908152f35b5034610153578060031936011261015357601e54611ed7816135f8565b611ee46040519182613589565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b8383106120255786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310611f505786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110611fdc57505050505060208060019297019301930190928695949293611f43565b9091929394602080612018837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0876001960301895289516133cb565b9701950193929101611fb8565b60405161203181613559565b6001600160a01b03835416815260018301805461204d816135f8565b9161205b6040519384613589565b8183528a526020808b20908b9084015b838210612091575050505060019282602092836002950152815201920192019190611f14565b6001602081926040516120a88161084b818a613776565b81520193019101909161206b565b503461015357806003193601126101535760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b818110612115576101f0856101dc81870382613589565b82546001600160a01b03168452602090930192600192830192016120fe565b503461015357806003193601126101535760405190612154606083613589565b600282526040366020840137604080519261216f8285613589565b6001845260208401601f19830136823773123456789012345678901234567890123456789061219d83613641565b527323456789012345678901234567890123456789016121bc83613662565b52683635c9adc5dea000006121d086613641565b526001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561068a578351907f06447d560000000000000000000000000000000000000000000000000000000082526004820152848160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c5f57908591612495575b5060206001600160a01b0381541660446001600160a01b03601f5460081c16875194859384927f095ea7b30000000000000000000000000000000000000000000000000000000084526004840152683635c9adc5dea0000060248401525af18015610c5f57612478575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ca85782517ff4844814000000000000000000000000000000000000000000000000000000008152848160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c5f57908591612463575b50506001600160a01b03601f5460081c1694853b1561068a5760206123938551947f82947abe000000000000000000000000000000000000000000000000000000008652731bab804803159ad84b8854581aa53ac72455614e600487015260806024870152608486019061338f565b916003198584030160448601525191828152019190855b81811061244d5750505081849581868185829650683635c9adc5dea00000606483015203925af18015610c3557908391610c20575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c1d578051907f90c5013b000000000000000000000000000000000000000000000000000000008252828260048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610c145750610c035750f35b82518452602093840193909201916001016123aa565b8161246d91613589565b610ca857835f612324565b6124909060203d60201161074b5761073e8183613589565b6122b8565b8161249f91613589565b610ca857835f61224e565b9050346104ce575f6003193601126104ce576020810173243c63d5dbcf619ee36fde7ff63d1564d5665b418152739697211552826d7714c0267d274f51984f39d060604083015260408252612500606083613589565b6125776040925f84516125138682613589565b601281527f53796e646963617465546f6b656e2e736f6c000000000000000000000000000060208201528551809481927f8d1cc9250000000000000000000000000000000000000000000000000000000083526020600484015260248301906133cb565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa918215613385575f92613329575b5090602080936125d99386519584879551918291018587015e840190838201905f8252519283915e01015f815203601f198101835282613589565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce575f61262b918351809381927fb4d6c78200000000000000000000000000000000000000000000000000000000835260048301614cf3565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561331f5761330a575b508180808080731bab804803159ad84b8854581aa53ac72455614e5af1612675614d1e565b9015613261578290737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077657816126cf918451809381927fb4d6c78200000000000000000000000000000000000000000000000000000000835260048301614cf3565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c9b5761324c575b505080516101b88082019082821067ffffffffffffffff83111761323857908291615cf28339039083f0801561322d577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f55731bab804803159ad84b8854581aa53ac72455614e7fffffffffffffffffffffffff00000000000000000000000000000000000000006020541617602055731bab804803159ad84b8854581aa53ac72455614e7fffffffffffffffffffffffff000000000000000000000000000000000000000060215416176021556001600160a01b036127f5613bad565b167fffffffffffffffffffffffff000000000000000000000000000000000000000060225416176022555f806128be6128e68451612834606082613589565b602a81527f4772616e74696e672041495244524f505f4d414e414745525f524f4c4520757360208201527f696e672061646d696e3a000000000000000000000000000000000000000000008682015285519283917f319af33300000000000000000000000000000000000000000000000000000000602084015287602484015260648301906133cb565b73243c63d5dbcf619ee36fde7ff63d1564d5665b41604483015203601f198101835282613589565b6020815191016a636f6e736f6c652e6c6f675afa50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610776578181517f06447d5600000000000000000000000000000000000000000000000000000000815273243c63d5dbcf619ee36fde7ff63d1564d5665b416004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c9b57613218575b506001600160a01b03602154166001600160a01b0360225416813b1561077257829160448392865194859384927f2f2ff15d0000000000000000000000000000000000000000000000000000000084527f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a674600485015260248401525af18015610c9b57613203575b506001600160a01b03602154166001600160a01b03601f5460081c16813b1561077257829160448392865194859384927f2f2ff15d0000000000000000000000000000000000000000000000000000000084527f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a674600485015260248401525af18015610c9b576131ee575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610776578181517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c9b576131d9575b50506001600160a01b03602154166001600160a01b03602254169082517f91d148540000000000000000000000000000000000000000000000000000000081527f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a6746004820152826024820152602081604481855afa908115610c5f5785916131ba575b5060206001600160a01b03601f5460081c1660448651809581937f91d148540000000000000000000000000000000000000000000000000000000083527f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a674600484015260248301525afa918215610c5f578592613199575b50156131165715613093578290612c678351612c14606082613589565b8481527f5375636365737366756c6c79206772616e7465642041495244524f505f4d414e60208201527f414745525f524f4c4520746f20686f6c64657220616e6420636f6e747261637485820152614895565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610776578251907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c9b5761307e575b50506001600160a01b036020541660206001600160a01b036022541660248451809481937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa908115610c3557839161304c575b506901b1ae4d6e2ef5000000809110612fef57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610772578282517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015612fe557612fd0575b5050612ded8251612dbe8482613589565b600f81527f536574757020636f6d706c6574653a00000000000000000000000000000000006020820152614895565b612e3a6001600160a01b03601f5460081c168351612e0b8582613589565b601181527f41697264726f7020636f6e74726163743a0000000000000000000000000000006020820152614960565b612e788251612e498482613589565b600b81527f53594e4420746f6b656e3a00000000000000000000000000000000000000000060208201526148f2565b6001600160a01b036022541691612ec4838251612e958482613589565b600d81527f546f6b656e20686f6c6465723a000000000000000000000000000000000000006020820152614960565b60206001600160a01b038154169360248351809681937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa928315612fc6578493612f90575b50612f5561054c938251612f268482613589565b601581527f546f6b656e20686f6c6465722062616c616e63653a000000000000000000000060208201526149c5565b612f6181519182613589565b601581527f546f74616c2061697264726f7020616d6f756e743a000000000000000000000060208201526149c5565b92506020833d602011612fbe575b81612fab60209383613589565b810103126104ce57915191612f55612f12565b3d9150612f9e565b81513d86823e3d90fd5b81612fda91613589565b61077257825f612dad565b84513d84823e3d90fd5b606482517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601d60248201527f496e73756666696369656e742062616c616e636520666f7220746573740000006044820152fd5b90506020813d602011613076575b8161306760209383613589565b810103126104ce57515f612d31565b3d915061305a565b8161308891613589565b61077657815f612cd5565b608482517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603860248201527f4661696c656420746f206772616e742041495244524f505f4d414e414745525f60448201527f524f4c4520746f2061697264726f7020636f6e747261637400000000000000006064820152fd5b608483517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603460248201527f4661696c656420746f206772616e742041495244524f505f4d414e414745525f60448201527f524f4c4520746f20746f6b656e20686f6c6465720000000000000000000000006064820152fd5b6131b391925060203d60201161074b5761073e8183613589565b905f612bf7565b6131d3915060203d60201161074b5761073e8183613589565b5f612b7f565b816131e391613589565b61077657815f612afc565b816131f891613589565b61077657815f612a91565b8161320d91613589565b61077657815f612a06565b8161322291613589565b61077657815f61297e565b5051903d90823e3d90fd5b602485634e487b7160e01b81526041600452fd5b8161325691613589565b61077657815f6126f4565b60a482517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152605860248201527f537464436865617473206465706c6f79436f6465546f28737472696e672c627960448201527f7465732c75696e743235362c61646472657373293a204661696c656420746f2060648201527f6372656174652072756e74696d652062797465636f64652e00000000000000006084820152fd5b6133179192505f90613589565b5f905f612650565b82513d5f823e3d90fd5b91503d805f843e61333a8184613589565b8201916020818403126104ce5780519067ffffffffffffffff82116104ce57019180601f840112156104ce5760209361337c859285846125d997519101614cbd565b9350935061259e565b84513d5f823e3d90fd5b90602080835192838152019201905f5b8181106133ac5750505090565b82516001600160a01b031684526020938401939092019160010161339f565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b81811061340d5750505090565b82517fffffffff0000000000000000000000000000000000000000000000000000000016845260209384019390920191600101613400565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061347757505050505090565b90919293946020806134b3837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0866001960301875289516133cb565b97019301930191939290613468565b602081016020825282518091526040820191602060408360051b8301019401925f915b8383106134f457505050505090565b909192939460208061354a837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b038151168452015191818582015201906133f0565b970193019301919392906134e5565b6040810190811067ffffffffffffffff82111761357557604052565b634e487b7160e01b5f52604160045260245ffd5b90601f601f19910116810190811067ffffffffffffffff82111761357557604052565b67ffffffffffffffff811161357557601f01601f191660200190565b908160209103126104ce575190565b919082018092116135e457565b634e487b7160e01b5f52601160045260245ffd5b67ffffffffffffffff81116135755760051b60200190565b9061361a826135f8565b6136276040519182613589565b828152601f1961363782946135f8565b0190602036910137565b80511561364e5760200190565b634e487b7160e01b5f52603260045260245ffd5b80516001101561364e5760400190565b80516002101561364e5760600190565b80516003101561364e5760800190565b80516004101561364e5760a00190565b805182101561364e5760209160051b010190565b908160209103126104ce575180151581036104ce5790565b939291906136fe90731bab804803159ad84b8854581aa53ac72455614e865260806020870152608086019061338f565b908482036040860152602080825193848152019101915f5b81811061372857505060609150930152565b8351835260209384019390920191600101613716565b90600182811c9216801561376c575b602083101461375857565b634e487b7160e01b5f52602260045260245ffd5b91607f169161374d565b5f92918154916137858361373e565b80835292600181169081156137da57506001146137a157505050565b5f9081526020812093945091925b8383106137c0575060209250010190565b6001816020929493945483858701015201910191906137af565b905060209495507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091509291921683830152151560051b010190565b90604051918281549182825260208201905f5260205f20925f905b806007830110613a2f576138879454918181106139f9575b8181106139c3575b81811061398d575b818110613957575b818110613921575b8181106138eb575b8181106138b6575b10613889575b500383613589565b565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f61387f565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b168152019301613879565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301613871565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b168152019301613869565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301613861565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b168152019301613859565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b168152019301613851565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b168152019301613849565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e0820152019401920185929391613831565b919082039182116135e457565b60085460ff168015613ad85790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115613ba2575f91613b70575b50151590565b90506020813d602011613b9a575b81613b8b60209383613589565b810103126104ce57515f613b6a565b3d9150613b7e565b6040513d5f823e3d90fd5b5f604051613bbc608082613589565b600381526060366020830137731234567890123456789012345678901234567890613be682613641565b52732345678901234567890123456789012345678901613c0582613662565b52733456789012345678901234567890123456789012613c2482613672565b526020546001600160a01b03165f5b8251811015613cf8576001600160a01b03613c4e82856136a2565b5116604051907f70a082310000000000000000000000000000000000000000000000000000000082526004820152602081602481865afa908115613ba2575f91613cc7575b506901b1ae4d6e2ef50000001115613cad57600101613c33565b90506001600160a01b039250613cc2916136a2565b511690565b90506020813d8211613cf0575b81613ce160209383613589565b810103126104ce57515f613c93565b3d9150613cd4565b505050604090815191613d0b8184613589565b600a8352602083017f74657374486f6c6465720000000000000000000000000000000000000000000081528151600a6020820192835e5f602a820152600a8152613d56602a82613589565b519020928151937fffa186490000000000000000000000000000000000000000000000000000000085526004850152602084602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa93841561331f575f94614851575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce5781517fc657c7180000000000000000000000000000000000000000000000000000000081525f8180613e186001600160a01b0389169586600484015287602484015260448301906133cb565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561484757614832575b508280835160208101907f70a0823100000000000000000000000000000000000000000000000000000000825284602482015260248152613e7f604482613589565b5190731bab804803159ad84b8854581aa53ac72455614e5afa50613eb3613ea4614d1e565b602080825183010191016135c8565b50731bab804803159ad84b8854581aa53ac72455614e7fffffffffffffffffffffffff000000000000000000000000000000000000000060115416176011556370a082317fffffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000600f541617600f55600e546801000000000000000081101561481e576001810180600e5581101561480a57600e84526020842001556001600160a01b0360115416600f5460e01b60105490613f6b614d64565b907fffffffff00000000000000000000000000000000000000000000000000000000613fa8856001600160a01b03165f52600d60205260405f2090565b91169081875260205284862085516020810190613fd881613fca888886614d4d565b03601f198101835282613589565b519020875260205260ff6003868820015416156147fc575b61400b846001600160a01b03165f52600d60205260405f2090565b908652602052613fca61402b858720938651928391602083019586614d4d565b519020845260205281832090600182015491600281015461404c81856135d7565b614615575b815485517f667f9d700000000000000000000000000000000000000000000000000000000081526001600160a01b03851660048201526024810182905294909190602086604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa95861561460b5788966145d7575b506001908201610100031b5f1901811b19851691737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156145d35786517f70ca10bb0000000000000000000000000000000000000000000000000000000081526001600160a01b038616600482015260248101919091526903635c9adc5dea00000090911b919091176044820152858160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156145c9579086916145b4575b505084806020600f5460e01b6141d26024614185600d614e3d565b8a519485917fffffffff00000000000000000000000000000000000000000000000000000000828401961686528051918291018484015e810186838201520301601f198101845283613589565b6001600160a01b03601154169151915afa6141eb614d1e565b906010548060051b907f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8116036145a05787928051602081115f1461459a57506020905b89925b8284106145125750505050159081156144fd575b506143d257505050507fffffffffffffffffffffffff0000000000000000000000000000000000000000601154166011557fffffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000600f5416600f55600e5481600e5580614390575b50806010557fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00601354166013556142e460145461373e565b806142ee57505090565b601f811160011461430157506014555b90565b60148252601f0160051c7fce6d7b5282bd9a3661ae061feed1dbda4e52ab073b1f9285be6e155d9c38d4ec017fce6d7b5282bd9a3661ae061feed1dbda4e52ab073b1f9285be6e155d9c38d4ed5b81811061438557505060148082528190557fce6d7b5282bd9a3661ae061feed1dbda4e52ab073b1f9285be6e155d9c38d4ec5590565b5f815560010161434f565b600e82527fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3fd908101905b8181106143c757506142ac565b8281556001016143ba565b548491737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561077257614440855194859384937f70ca10bb000000000000000000000000000000000000000000000000000000008552600485016040919493926001600160a01b03606083019616825260208201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610c35576144e8575b608482517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603360248201527f73746453746f726167652066696e642853746453746f72616765293a2046616960448201527f6c656420746f2077726974652076616c75652e000000000000000000000000006064820152fd5b6144f3838092613589565b6107765781614465565b6903635c9adc5dea000000915014155f614246565b9091929461452086836135d7565b83518110156145865760207fff000000000000000000000000000000000000000000000000000000000000009185010151168660031b87810460081488151715614572571c1794600101929190614232565b60248d634e487b7160e01b81526011600452fd5b60248c634e487b7160e01b81526032600452fd5b9061422f565b602488634e487b7160e01b81526011600452fd5b816145be91613589565b61068a57845f61416a565b85513d88823e3d90fd5b8780fd5b9095506020813d602011614603575b816145f360209383613589565b810103126104ce575194816140bd565b3d91506145e6565b87513d8a823e3d90fd5b61461f81856135d7565b6101000361010081116147e85760ff81116147e8576001901b85517f6900a3ae0000000000000000000000000000000000000000000000000000000081528160048201528781602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561460b578891614784575b509061472c606a60209389519485917f73746453746f726167652066696e642853746453746f72616765293a20506163828401527f6b656420736c6f742e2057652063616e2774206669742076616c7565206772658c8401527f61746572207468616e200000000000000000000000000000000000000000000060608401528051918291018484015e81018b838201520301601f198101845283613589565b6903635c9adc5dea00000010156147435750614051565b6147809086519182917f08c379a00000000000000000000000000000000000000000000000000000000083526020600484015260248301906133cb565b0390fd5b90503d8089833e6147958183613589565b8101906020818303126147e05780519067ffffffffffffffff82116147e4570181601f820112156147e057606a6147d760209493838661472c95519101614cbd565b9293505061468d565b8880fd5b8980fd5b602487634e487b7160e01b81526011600452fd5b6148046150a6565b50613ff0565b602484634e487b7160e01b81526032600452fd5b602484634e487b7160e01b81526041600452fd5b61483f9193505f90613589565b5f915f613e3d565b83513d5f823e3d90fd5b9093506020813d60201161488d575b8161486d60209383613589565b810103126104ce57516001600160a01b03811681036104ce57925f613db0565b3d9150614860565b5f613fca6148db82936040519283917f41304fac0000000000000000000000000000000000000000000000000000000060208401526020602484015260448301906133cb565b6020815191016a636f6e736f6c652e6c6f675afa50565b5f6149386148db82936040519283917f319af3330000000000000000000000000000000000000000000000000000000060208401526040602484015260648301906133cb565b731bab804803159ad84b8854581aa53ac72455614e604483015203601f198101835282613589565b6149b06148db5f939284936001600160a01b036040519485937f319af3330000000000000000000000000000000000000000000000000000000060208601526040602486015260648501906133cb565b9116604483015203601f198101835282613589565b614a0d6148db5f939284936040519384927fb60e72cc0000000000000000000000000000000000000000000000000000000060208501526040602485015260648401906133cb565b90604483015203601f198101835282613589565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce575f91614a8860405194859384937f88b44c85000000000000000000000000000000000000000000000000000000008552600485015260248401526060604484015260648301906133cb565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015613ba257614aae5750565b5f61388791613589565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce57614a88915f9160405193849283927fa34edc03000000000000000000000000000000000000000000000000000000008452151560048401526040602484015260448301906133cb565b604051614b2a60c082613589565b600581525f5b60a08110614c9a5750604051614b4581613559565b7312345678901234567890123456789012345678908152683635c9adc5dea000006020820152614b7482613641565b52614b7e81613641565b50604051614b8b81613559565b7323456789012345678901234567890123456789018152686c6b935b8bbd4000006020820152614bba82613662565b52614bc481613662565b50604051614bd181613559565b7334567890123456789012345678901234567890128152685150ae84a8cdf000006020820152614c0082613672565b52614c0a81613672565b50604051614c1781613559565b734567890123456789012345678901234567890123815268a2a15d09519be000006020820152614c4682613682565b52614c5081613682565b50604051614c5d81613559565b7356789012345678901234567890123456789012348152681b1ae4d6e2ef5000006020820152614c8c82613692565b52614c9681613692565b5090565b602090604051614ca981613559565b5f81525f8382015282828501015201614b30565b929192614cc9826135ac565b91614cd76040519384613589565b8294818452818301116104ce578281602093845f96015e010152565b9060406142fe92731bab804803159ad84b8854581aa53ac72455614e815281602082015201906133cb565b3d15614d48573d90614d2f826135ac565b91614d3d6040519384613589565b82523d5f602084013e565b606090565b60209291908391805192839101825e019081520190565b614d6f60145461373e565b614e2b576040519081826020600e549283815201600e5f5260205f20925f5b818110614e12575050614da392500383613589565b81518060051b90808204602014901517156135e457601f19614ddd614dc7836135ac565b92614dd56040519485613589565b8084526135ac565b013660208301375f5b8351811015614e0d5780614dfc600192866136a2565b5160208260051b8501015201614de6565b509150565b8454835260019485019487945060209093019201614d8e565b6040516142fe8161084b816014613776565b6007810190614e4c825461373e565b614ef157600191500190604051808360208295549384815201905f5260205f20925f5b818110614ed8575050614e8492500383613589565b81518060051b90808204602014901517156135e457601f19614ea8614dc7836135ac565b013660208301375f5b8351811015614e0d5780614ec7600192866136a2565b5160208260051b8501015201614eb1565b8454835260019485019487945060209093019201614e6f565b506142fe61084b9160405192838092613776565b905f806020600285015460e01b614f6e6024614f2088614e3d565b6040519485917fffffffff00000000000000000000000000000000000000000000000000000000828401961686528051918291018484015e810186838201520301601f198101845283613589565b6001600160a01b036004870154169151915afa6003614f8b614d1e565b9301548060051b907f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8116036135e4575f938051602081115f1461504357506020905b5f925b828410614fe057505050509190565b90919295614fee87836135d7565b835181101561364e5760207fff00000000000000000000000000000000000000000000000000000000000000918501015116908760031b91888304600814891517156135e4576001921c179601929190614fd1565b90614fce565b9080601f830112156104ce578151615060816135f8565b9261506e6040519485613589565b81845260208085019260051b8201019283116104ce57602001905b8282106150965750505090565b8151815260209182019101615089565b5f6001600160a01b036011541690600f5460e01b601054906150c8600d614e3d565b90845f52600d6020527fffffffff0000000000000000000000000000000000000000000000000000000060405f20911690815f5260205260405f20604051602081019061511a81613fca888886614d4d565b5190205f5260205260ff600360405f20015416615ac957737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce576040517f266cf1090000000000000000000000000000000000000000000000000000000081525f8160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015613ba257615ab4575b506151a5600d614f05565b90506040517f65bc9481000000000000000000000000000000000000000000000000000000008152866004820152858160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156116e1578691615a4e575b5080518061528a5760846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152604060248201527f73746453746f726167652066696e642853746453746f72616765293a204e6f2060448201527f73746f726167652075736520646574656374656420666f72207461726765742e6064820152fd5b80156147e857905f196152ef92019060206152a583836136a2565b516040517f667f9d700000000000000000000000000000000000000000000000000000000081526001600160a01b038c166004820152602481019190915293849081906044820190565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa928315610ee4578893615a1b575b5082156159c3575b61532882826136a2565b516011546040517f667f9d700000000000000000000000000000000000000000000000000000000081526001600160a01b0390911660048201819052602482018390529190602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa9081156159b8578b91615987575b506153a3600d614f05565b91909382155f14615980575f19905b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15615967576040517f70ca10bb0000000000000000000000000000000000000000000000000000000081526001600160a01b039190911660048201526024810185905260448101919091528c8160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156158e457908d9161596b575b505061544a600d614f05565b9390506001600160a01b0360115416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15615967576040517f70ca10bb0000000000000000000000000000000000000000000000000000000081526001600160a01b03919091166004820152602481019190915260448101919091528b8160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561595c57908c91615943575b505082615938575b505015615931578793889360ff60135416615761575b6001868601610100031b5f1901851b16841c810361575857509061558a917f9c9555b1e3102e3cf48f427d79cb678f5d9bd1ed0ad574389461e255f95170ed60808b89613fca61555f8d604051928391602083019586614d4d565b51902061556c86866136a2565b51906040519283528a602084015260408301526060820152a16136a2565b5190604051916080830183811067ffffffffffffffff8211176157445790600393929160405282526020820193845260408201908152606082019360018552898952600d60205260408920868a526020526040892060405160208101906155f681613fca8d8d86614d4d565b5190208a526020526040892092518355516001830155516002820155019051151560ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008354169116179055848452600d6020526040842081855260205260408420604051602081019061566f81613fca888886614d4d565b519020855260205260ff600360408620015416156156c0576040948452600d602052848420908452602052613fca6156b4858520938651928391602083019586614d4d565b51902082526020522090565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602f60248201527f73746453746f726167652066696e642853746453746f72616765293a20536c6f60448201527f74287329206e6f7420666f756e642e00000000000000000000000000000000006064820152fd5b602489634e487b7160e01b81526041600452fd5b9350915061528a565b94506157cd935061577283836136a2565b51946020866001600160a01b036011541660405197889283927f667f9d7000000000000000000000000000000000000000000000000000000000845260048401602090939291936001600160a01b0360408201951681520152565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa948515615926578a956158f3575b5061580086600d615b0a565b95909661580e81600d615c08565b9290916001600160a01b0360115416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156158ef576040517f70ca10bb0000000000000000000000000000000000000000000000000000000081526001600160a01b03919091166004820152602481019190915260448101919091528c8160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156158e457908d916158cb575b5050876158c3575b50949561550457509350915061528a565b96505f6158b2565b816158d591613589565b6158e0578b5f6158aa565b8b80fd5b6040513d8f823e3d90fd5b8e80fd5b9094506020813d821161591e575b8161590e60209383613589565b810103126104ce5751935f6157f4565b3d9150615901565b6040513d8c823e3d90fd5b915061528a565b141590505f806154ee565b8161594d91613589565b615958578a5f6154e6565b8a80fd5b6040513d8e823e3d90fd5b8d80fd5b8161597591613589565b6158e0578b5f61543e565b8c906153b2565b90506020813d82116159b0575b816159a160209383613589565b810103126104ce57515f615398565b3d9150615994565b6040513d8d823e3d90fd5b7f080fc4a96620c4462e705b23f346413fe3796bb63c6f8d8591baec0e231577a5615a136159f184846136a2565b51604080516001600160a01b038e168152602081019290925290918291820190565b0390a161531e565b9092506020813d8211615a46575b81615a3660209383613589565b810103126104ce5751915f615316565b3d9150615a29565b90503d8087833e615a5f8183613589565b8101604082820312615ab057815167ffffffffffffffff81116145d35781615a88918401615049565b9160208101519067ffffffffffffffff82116147e057615aa9929101615049565b505f6151fe565b8680fd5b615ac19194505f90613589565b5f925f61519a565b91939092505f52600d60205260405f20905f52602052613fca615afb60405f2093604051928391602083019586614d4d565b5190205f5260205260405f2090565b91905f5b6101008110615b2157505090505f905f90565b8060ff0360ff81116135e4576001901b6001600160a01b03600486015416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce576040517f70ca10bb0000000000000000000000000000000000000000000000000000000081526001600160a01b039190911660048201526024810184905260448101919091525f8160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015613ba257615bf8575b50615bd284614f05565b81615bee575b50615be557600101615b0e565b92505060019190565b905015155f615bd8565b5f615c0291613589565b5f615bc8565b91905f5b6101008110615c1f57505090505f905f90565b6001811b6001600160a01b03600486015416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104ce576040517f70ca10bb0000000000000000000000000000000000000000000000000000000081526001600160a01b039190911660048201526024810184905260448101919091525f8160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015613ba257615ce1575b50615cc484614f05565b81615cd7575b50615be557600101615c0c565b905015155f615cca565b5f615ceb91613589565b5f615cba56fe6080806040523460155761019e908161001a8239f35b5f80fdfe60806040526004361015610011575f80fd5b5f3560e01c6382947abe14610024575f80fd5b60807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100c15760043573ffffffffffffffffffffffffffffffffffffffff811681036100c15760243567ffffffffffffffff81116100c15761008e9036906004016100c5565b604435929167ffffffffffffffff84116100c1576100b36100bf9436906004016100c5565b929091606435946100f6565b005b5f80fd5b9181601f840112156100c15782359167ffffffffffffffff83116100c1576020808501948460051b0101116100c157565b918093959194036100c1577f23b872dd000000000000000000000000000000000000000000000000000000005f5233600452306024526044525f8060648180855af1156100c15791907fa9059cbb000000000000000000000000000000000000000000000000000000005f5260051b8101928103905b8035600452818103356024525f8060648180875af1156100c1576020019183831015610198579161016c565b5050505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\n\x92T\xE4\x14a$\xAAWP\x80c\x1By\x86\xD2\x14a!4W\x80c\x1E\xD7\x83\x1C\x14a \xB6W\x80c*\xDE8\x80\x14a\x1E\xBAW\x80c8\x84\xD65\x14a\x1E\x90W\x80c>^<#\x14a\x1E\x12W\x80c?r\x86\xF4\x14a\x1D\x94W\x80cB\n\x83\xE7\x14a\x1DmW\x80cY\xC6 \xC6\x14a\x1A\xA0W\x80cb\x13\x82\x1D\x14a\x1AzW\x80cf\xD9\xA9\xA0\x14a\x195W\x80c\x81\xA8\xDE~\x14a\x18BW\x80c\x85\"l\x81\x14a\x17\xB0W\x80c\x85\xA4h\xF7\x14a\r\x91W\x80c\x8AT%!\x14a\rVW\x80c\x91j\x17\xC6\x14a\x0C\xACW\x80c\x94\xA2\xA5\xDF\x14a\t&W\x80c\xA2\x17\xFD\xDF\x14a\t\nW\x80c\xB0FO\xDC\x14a\x08`W\x80c\xB5P\x8A\xA9\x14a\x07\xC7W\x80c\xBAAO\xA6\x14a\x07\xA2W\x80c\xBF\xF8`\x0F\x14a\x02\x13W\x80c\xE2\x0C\x9Fq\x14a\x01}W\x80c\xE5*/\x1F\x14a\x01VWc\xFAv&\xD4\x14a\x011W_\x80\xFD[4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x01\xF4Wa\x01\xF0\x85a\x01\xDC\x81\x87\x03\x82a5\x89V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a3\x8FV[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x01\xC5V[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SWa\x02,aK\x1CV[\x80Q`\x03\x81\x01\x90\x81\x81\x11a\x07\x8EW`\x02\x01\x90\x81\x11a\x07zW`\x03\x90\x04\x90\x82`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07vW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05dWa\x07]W[P` \x80T`\x1FT`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03`\x08\x92\x90\x92\x1C\x82\x16`\x04\x82\x01Ri\x01\xB1\xAEMn.\xF5\0\0\0`$\x82\x01R\x93\x84\x92`D\x92\x84\x92\x91\x16Z\xF1\x80\x15a\x07RWa\x07%W[P\x82[\x82\x81\x10a\x05oW\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01SW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05dWa\x05OW[PPa\x03\xAEaK\x1CV[` T`\x01`\x01`\xA0\x1B\x03\x16\x82[\x82Q\x81\x10\x15a\x04\xE5W`\x01`\x01`\xA0\x1B\x03a\x03\xD7\x82\x85a6\xA2V[QQ\x16\x90`@Q\x91\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R` \x82`$\x81\x86Z\xFA\x91\x82\x15a\x04\xDAW\x85\x92a\x04\xA1W[Pa\x04\x9B`\x01\x92` a\x041\x84\x88a6\xA2V[Q\x01Q\x11\x15``\x90a\x04F`@Q\x92\x83a5\x89V[`-\x82R\x7FRecipient should have received a` \x83\x01R\x7Firdrop amount\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x83\x01RaJ\xB8V[\x01a\x03\xBCV[\x91P` \x82=\x82\x11a\x04\xD2W[\x81a\x04\xBB` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEW\x90Q\x90a\x04\x9Ba\x04\x1EV[_\x80\xFD[=\x91Pa\x04\xAEV[`@Q=\x87\x82>=\x90\xFD[\x83a\x05L`@Qa\x04\xF7``\x82a5\x89V[`$\x81R\x7FBatch airdrop completed successf` \x82\x01R\x7Fully\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RaH\x95V[\x80\xF3[\x81a\x05Y\x91a5\x89V[a\x01SW\x80\x82a\x03\xA4V[`@Q=\x84\x82>=\x90\xFD[`\x03\x81\x02\x90\x80\x82\x04`\x03\x14\x81\x15\x17\x15a\x07\x11W`\x03\x82\x01\x80\x83\x11a\x06\xFDWa\x05\xA5\x83\x82\x88\x93\x90\x87Q\x80\x91\x11a\x06\xF5W[Pa:\xBCV[\x92a\x05\xAF\x84a6\x10V[a\x05\xB8\x85a6\x10V[\x91\x83\x90\x84\x90[\x87\x82\x10a\x06\x8EWPP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x06\x8AWa\x06\x1A\x93\x85\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93\x7F\x82\x94z\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a6\xCEV[\x03\x92Z\xF1\x80\x15a\x05dWa\x06uW[P`\x01\x92a\x06o\x91P`@a\x06@\x81Q\x91\x82a5\x89V[`\x1F\x81R\x7FBatch executed with recipients:\0` \x82\x01RaI\xC5V[\x01a\x032V[\x81a\x06\x7F\x91a5\x89V[a\x06\x8AW\x84_a\x06)V[\x84\x80\xFD[\x90\x91a\x06\xED`\x01\x91`\x01`\x01`\xA0\x1B\x03a\x06\xB1a\x06\xAB\x87\x87a5\xD7V[\x8Da6\xA2V[QQ\x16a\x06\xBE\x86\x88a6\xA2V[R` a\x06\xCEa\x06\xAB\x87\x87a5\xD7V[Q\x01Qa\x06\xDB\x86\x89a6\xA2V[Ra\x06\xE6\x85\x88a6\xA2V[Q\x90a5\xD7V[\x92\x01\x90a\x05\xBEV[\x90P_a\x05\x9FV[`$\x86cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`$\x85cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[a\x07F\x90` =` \x11a\x07KW[a\x07>\x81\x83a5\x89V[\x81\x01\x90a6\xB6V[a\x03/V[P=a\x074V[`@Q=\x86\x82>=\x90\xFD[\x81a\x07g\x91a5\x89V[a\x07rW\x82_a\x02\xC5V[\x82\x80\xFD[P\x80\xFD[`$\x83cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`$\x84cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` a\x07\xBDa:\xC9V[`@Q\x90\x15\x15\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`\x19Ta\x07\xE4\x81a5\xF8V[\x91a\x07\xF2`@Q\x93\x84a5\x89V[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\x084W`@Q\x80a\x01\xF0\x87\x82a4EV[`\x01` \x81\x92`@Qa\x08R\x81a\x08K\x81\x89a7vV[\x03\x82a5\x89V[\x81R\x01\x92\x01\x92\x01\x91\x90a\x08\x1FV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`\x1CTa\x08}\x81a5\xF8V[\x91a\x08\x8B`@Q\x93\x84a5\x89V[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\x08\xCDW`@Q\x80a\x01\xF0\x87\x82a4\xC2V[`\x02` `\x01\x92`@Qa\x08\xE0\x81a5YV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x08\xF8\x85\x87\x01a8\x16V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x08\xB8V[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` \x90`@Q\x90\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SWa\t?aK\x1CV[\x90`@\x80Q\x92a\tO\x82\x85a5\x89V[`\x01\x84R` a\t\x9D`\x1F\x19\x84\x01\x92\x836\x84\x89\x017\x84Q\x93a\tq\x86\x86a5\x89V[`\x01\x85R6\x84\x86\x017`\x01`\x01`\xA0\x1B\x03a\t\x8B\x82a6AV[QQ\x16a\t\x97\x88a6AV[Ra6AV[Q\x01Qa\t\xA9\x82a6AV[R`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C\xA8W\x82Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C\x9BW\x90\x84\x91a\x0C\x86W[PP`\x01`\x01`\xA0\x1B\x03` T\x16\x93`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x94a\nO\x83a6AV[Q\x95_\x19\x87\x01\x96\x87\x11a\x06\xFDW\x84Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x96\x90\x96R\x93\x94\x85\x94` \x90\x82\x90`D\x90\x82\x90\x89\x90Z\xF1\x80\x15a\x0C_Wa\x0CiW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0CZW\x82Q\x7F\xF4\x84H\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C_W\x90\x85\x91a\x0CEW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16a\x0B=\x83a6AV[Q\x90\x80;\x15a\x0CAWa\x0B\x82\x93\x86\x80\x94\x87Q\x96\x87\x95\x86\x94\x85\x93\x7F\x82\x94z\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01a6\xCEV[\x03\x92Z\xF1\x80\x15a\x0C5W\x90\x83\x91a\x0C W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C\x1DW\x80Q\x90\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x82\x82`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0C\x14WPa\x0C\x03WP\xF3[\x81a\x0C\r\x91a5\x89V[a\x01SW\x80\xF3[Q=\x84\x82>=\x90\xFD[P\xFD[\x81a\x0C*\x91a5\x89V[a\x0C\x1DW\x81_a\x0B\x94V[PPQ\x90=\x90\x82>=\x90\xFD[\x85\x80\xFD[\x81a\x0CO\x91a5\x89V[a\x0CZW\x83_a\x0B#V[PPP\xFD[\x84Q=\x87\x82>=\x90\xFD[a\x0C\x81\x90` =` \x11a\x07KWa\x07>\x81\x83a5\x89V[a\n\xB7V[\x81a\x0C\x90\x91a5\x89V[a\x07rW\x82_a\n'V[PPPQ\x90=\x90\x82>=\x90\xFD[\x83\x80\xFD[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`\x1DTa\x0C\xC9\x81a5\xF8V[\x91a\x0C\xD7`@Q\x93\x84a5\x89V[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\r\x19W`@Q\x80a\x01\xF0\x87\x82a4\xC2V[`\x02` `\x01\x92`@Qa\r,\x81a5YV[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\rD\x85\x87\x01a8\x16V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\r\x04V[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` `@Q\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SWa\r\xAAaK\x1CV[\x90a\r\xB5\x82Qa6\x10V[a\r\xBF\x83Qa6\x10V[\x82\x91\x82[\x85Q\x84\x10\x15a\x0E\x1EWa\x0E\x16`\x01\x91`\x01`\x01`\xA0\x1B\x03a\r\xE4\x87\x8Aa6\xA2V[QQ\x16a\r\xF1\x87\x86a6\xA2V[R` a\r\xFE\x87\x8Aa6\xA2V[Q\x01Qa\x0E\x0B\x87\x87a6\xA2V[Ra\x06\xE6\x86\x86a6\xA2V[\x93\x01\x92a\r\xC3V[\x90\x91\x92Pa\x0E,\x82Qa6\x10V[\x91\x84\x91`\x01`\x01`\xA0\x1B\x03` T\x16\x92[\x82Q\x81\x10\x15a\x0E\xEFW`\x01`\x01`\xA0\x1B\x03a\x0EX\x82\x85a6\xA2V[Q\x16\x90`@Q\x91\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R` \x82`$\x81\x88Z\xFA\x80\x15a\x0E\xE4W\x88\x90a\x0E\xB2W[`\x01\x92Pa\x0E\xAB\x82\x88a6\xA2V[R\x01a\x0E=V[P` \x82=\x82\x11a\x0E\xDCW[\x81a\x0E\xCB` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEW`\x01\x91Qa\x0E\x9DV[=\x91Pa\x0E\xBEV[`@Q=\x8A\x82>=\x90\xFD[P\x93\x91\x92`$\x95\x93` `\x01`\x01`\xA0\x1B\x03`\"T\x16\x91`@Q\x98\x89\x80\x92\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x85`\x04\x83\x01RZ\xFA\x96\x87\x15a\x04\xDAW\x85\x97a\x17|W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x8AW`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x84\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xDAW\x90\x85\x91a\x17gW[PP` \x80T`\x1FT`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03`\x08\x92\x90\x92\x1C\x82\x16`\x04\x82\x01R`$\x81\x01\x89\x90R\x92\x91\x83\x91\x16\x81\x88\x81`D\x81\x01\x03\x92Z\xF1\x80\x15a\x04\xDAW\x90a\x10\xA0\x93\x92\x91a\x17JW[P`\x01`\x01`\xA0\x1B\x03` T\x16` `\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x92\x83\x91`@Q\x80\x98\x81\x94\x82\x93\x7F\xDDb\xED>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91\x81`@\x85\x01\x96\x16\x84R\x16\x91\x01RV[\x03\x91Z\xFA\x93\x84\x15a\x16\xE1W\x86\x94a\x17\x16W[Pa\x11\x1D``\x94\x88`@Q\x91a\x10\xC8\x88\x84a5\x89V[`#\x83R\x7FAllowance should equal total amo` \x84\x01R\x7Funt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x84\x01RaJ!V[\x80;\x15a\x0CAW\x85`@Q\x80\x92\x7F\x82\x94z\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x83\x81a\x11_\x8D\x8C\x8B`\x04\x85\x01a6\xCEV[\x03\x92Z\xF1\x80\x15a\x16\xE1W\x90\x86\x91a\x17\x01W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x8AW`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x85\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x16\xE1W\x90\x86\x91a\x16\xECW[P\x96\x95`\x01`\x01`\xA0\x1B\x03` T\x16\x97[\x83Q\x81\x10\x15a\x13bWa\x12\x11a\x12\x06\x82\x85a6\xA2V[Qa\x06\xE6\x83\x89a6\xA2V[\x90\x89` `\x01`\x01`\xA0\x1B\x03a\x12'\x84\x89a6\xA2V[Q\x16`$`@Q\x80\x94\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x13WW\x89\x91a\x13%W[P`@Q`\x01\x93a\x12\xD4\x92a\x12\x7F\x8A\x84a5\x89V[`3\x83R\x7FRecipient balance should increas` \x84\x01R\x7Fe by airdrop amount\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x84\x01RaJ!V[a\x13\x1Fa\x12\xE1\x82\x89a6\xA2V[Q`@a\x12\xF0\x81Q\x91\x82a5\x89V[`\x18\x81R\x7FRecipient received SYND:\0\0\0\0\0\0\0\0` \x82\x01RaI\xC5V[\x01a\x11\xF0V[\x90P` \x81=\x82\x11a\x13OW[\x81a\x13?` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ`\x01a\x12jV[=\x91Pa\x132V[`@Q=\x8B\x82>=\x90\xFD[\x86\x84\x83\x82\x88\x8C\x8E`\x01`\x01`\xA0\x1B\x03`\"T\x16\x91`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x91\x82\x15a\x16\xE1W\x87\x91\x87\x93a\x16\xA2W[Pa\x14\x90\x94\x92a\x13\xD1` \x95\x93a\x143\x93a:\xBCV[`@Q\x91a\x13\xDF\x89\x84a5\x89V[`4\x83R\x7FToken holder balance should decr\x87\x84\x01R\x7Fease by total amount\0\0\0\0\0\0\0\0\0\0\0\0`@\x84\x01RaJ!V[`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x91`@Q\x80\x95\x81\x94\x82\x93\x7F\xDDb\xED>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91\x81`@\x85\x01\x96\x16\x84R\x16\x91\x01RV[\x03\x91Z\xFA\x90\x81\x15a\x16\x97W\x83\x91a\x16bW[P`@Q\x91a\x14\xB1\x81\x84a5\x89V[`&\x83R\x7FAllowance should be zero after a` \x84\x01R\x7Firdrop\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x84\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C\xA8W\x83\x91a\x15f`@Q\x94\x85\x93\x84\x93\x7F\x88\xB4L\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01R\x85`$\x85\x01R`D\x84\x01R`d\x83\x01\x90a3\xCBV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05dWa\x16MW[P\x91a\x05L\x92Pa\x16\x10`@\x92a\x15\xD5\x84Qa\x15\xA6\x86\x82a5\x89V[`\x1E\x81R\x7FAirdrop executed successfully:\0\0` \x82\x01RaH\x95V[\x83Qa\x15\xE1\x85\x82a5\x89V[`\x12\x81R\x7FTotal distributed:\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI\xC5V[Q\x90a\x16\x1E\x81Q\x91\x82a5\x89V[`\x0B\x81R\x7FRecipients:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI\xC5V[\x81a\x16W\x91a5\x89V[a\x07rW\x82\x84a\x15\x8AV[\x92PP` \x82=` \x11a\x16\x8FW[\x81a\x16~` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEW\x84\x91Q\x86a\x14\xA2V[=\x91Pa\x16qV[`@Q=\x85\x82>=\x90\xFD[\x94\x92P\x92\x95PP` \x83=` \x11a\x16\xD9W[\x81a\x16\xC2` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEW\x91Q\x87\x94\x91\x86\x90a\x143a\x13\xBBV[=\x91Pa\x16\xB5V[`@Q=\x88\x82>=\x90\xFD[\x81a\x16\xF6\x91a5\x89V[a\x06\x8AW\x84\x88a\x11\xDFV[\x81a\x17\x0B\x91a5\x89V[a\x06\x8AW\x84\x88a\x11qV[\x90\x93P` \x81=` \x11a\x17BW[\x81a\x172` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ\x92\x88a\x10\xB2V[=\x91Pa\x17%V[a\x17b\x90` =` \x11a\x07KWa\x07>\x81\x83a5\x89V[a\x10&V[\x81a\x17q\x91a5\x89V[a\x0C\xA8W\x83\x87a\x0F\xB9V[\x90\x96P` \x81=` \x11a\x17\xA8W[\x81a\x17\x98` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ\x95\x87a\x0FFV[=\x91Pa\x17\x8BV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`\x1ATa\x17\xCD\x81a5\xF8V[\x91a\x17\xDB`@Q\x93\x84a5\x89V[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\x18\x1DW`@Q\x80a\x01\xF0\x87\x82a4EV[`\x01` \x81\x92`@Qa\x184\x81a\x08K\x81\x89a7vV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x18\x08V[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SWa\x05L`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16a\x18\xCE`@Qa\x18v``\x82a5\x89V[`#\x81R\x7FAirdrop contract should be deplo` \x82\x01R\x7Fyed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x82\x15\x15aJ\xB8V[;\x15\x15`@Q\x90a\x18\xE0``\x83a5\x89V[`!\x82R\x7FAirdrop contract should have cod` \x83\x01R\x7Fe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x83\x01RaJ\xB8V[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`\x1BTa\x19R\x81a5\xF8V[a\x19_`@Q\x91\x82a5\x89V[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a\x1A7W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a\x19\xCCWPPPP\x03\x90\xF3[\x91\x93` a\x1A'\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a\x1A\x17\x83Q`@\x84R`@\x84\x01\x90a3\xCBV[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra3\xF0V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x19\xBDV[`\x02` `\x01\x92`@Qa\x1AJ\x81a5YV[`@Qa\x1A[\x81a\x08K\x81\x8Aa7vV[\x81Ra\x1Ah\x85\x87\x01a8\x16V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x19\x8FV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW\x80`\x04`@a\x1A\xF4\x81Qa\x1A\xC5\x83\x82a5\x89V[`\x15\x81R\x7FToken Name: Syndicate\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaH\x95V[a\x1B2\x81Qa\x1B\x03\x83\x82a5\x89V[`\x12\x81R\x7FToken Symbol: SYND\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaH\x95V[a\x1Bp\x81Qa\x1BA\x83\x82a5\x89V[`\x12\x81R\x7FToken Decimals: 18\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaH\x95V[a\x1B\xAE\x81Qa\x1B\x7F\x83\x82a5\x89V[`\x0E\x81R\x7FToken Address:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaH\xF2V[` `\x01`\x01`\xA0\x1B\x03\x81T\x16\x82Q\x93\x84\x80\x92\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x1DcW\x83\x92a\x1D,W[Pa\x1C4\x82\x82Qa\x1C\x05\x84\x82a5\x89V[`\r\x81R\x7FTotal Supply:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI\xC5V[\x80Q\x91a\x1CB``\x84a5\x89V[`'\x83R\x7FTotal supply should be 920 milli` \x84\x01R\x7Fon SYND\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0CZWa\x1D\x04\x92\x84\x91\x83Q\x94\x85\x92\x83\x92\x7F\x88\xB4L\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rk\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0`$\x84\x01R```D\x84\x01R`d\x83\x01\x90a3\xCBV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x0C\x14WPa\x0C\x03WP\xF3[\x92P\x90P` \x82=` \x11a\x1D[W[\x81a\x1DI` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEW\x82\x91Q\x90_a\x1B\xF4V[=\x91Pa\x1D<V[\x81Q=\x85\x82>=\x90\xFD[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a\x1D\xF3Wa\x01\xF0\x85a\x01\xDC\x81\x87\x03\x82a5\x89V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x1D\xDCV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a\x1EqWa\x01\xF0\x85a\x01\xDC\x81\x87\x03\x82a5\x89V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x1EZV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`\x1ETa\x1E\xD7\x81a5\xF8V[a\x1E\xE4`@Q\x91\x82a5\x89V[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a %W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a\x1FPW\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a\x1F\xDCWPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a\x1FCV[\x90\x91\x92\x93\x94` \x80a \x18\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa3\xCBV[\x97\x01\x95\x01\x93\x92\x91\x01a\x1F\xB8V[`@Qa 1\x81a5YV[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta M\x81a5\xF8V[\x91a [`@Q\x93\x84a5\x89V[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a \x91WPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x1F\x14V[`\x01` \x81\x92`@Qa \xA8\x81a\x08K\x81\x8Aa7vV[\x81R\x01\x93\x01\x91\x01\x90\x91a kV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a!\x15Wa\x01\xF0\x85a\x01\xDC\x81\x87\x03\x82a5\x89V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a \xFEV[P4a\x01SW\x80`\x03\x196\x01\x12a\x01SW`@Q\x90a!T``\x83a5\x89V[`\x02\x82R`@6` \x84\x017`@\x80Q\x92a!o\x82\x85a5\x89V[`\x01\x84R` \x84\x01`\x1F\x19\x83\x016\x827s\x124Vx\x90\x124Vx\x90\x124Vx\x90\x124Vx\x90a!\x9D\x83a6AV[Rs#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01a!\xBC\x83a6bV[Rh65\xC9\xAD\xC5\xDE\xA0\0\0a!\xD0\x86a6AV[R`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x8AW\x83Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x84\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C_W\x90\x85\x91a$\x95W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x87Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rh65\xC9\xAD\xC5\xDE\xA0\0\0`$\x84\x01RZ\xF1\x80\x15a\x0C_Wa$xW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C\xA8W\x82Q\x7F\xF4\x84H\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C_W\x90\x85\x91a$cW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x94\x85;\x15a\x06\x8AW` a#\x93\x85Q\x94\x7F\x82\x94z\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86Rs\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN`\x04\x87\x01R`\x80`$\x87\x01R`\x84\x86\x01\x90a3\x8FV[\x91`\x03\x19\x85\x84\x03\x01`D\x86\x01RQ\x91\x82\x81R\x01\x91\x90\x85[\x81\x81\x10a$MWPPP\x81\x84\x95\x81\x86\x81\x85\x82\x96Ph65\xC9\xAD\xC5\xDE\xA0\0\0`d\x83\x01R\x03\x92Z\xF1\x80\x15a\x0C5W\x90\x83\x91a\x0C WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C\x1DW\x80Q\x90\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x82\x82`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0C\x14WPa\x0C\x03WP\xF3[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a#\xAAV[\x81a$m\x91a5\x89V[a\x0C\xA8W\x83_a#$V[a$\x90\x90` =` \x11a\x07KWa\x07>\x81\x83a5\x89V[a\"\xB8V[\x81a$\x9F\x91a5\x89V[a\x0C\xA8W\x83_a\"NV[\x90P4a\x04\xCEW_`\x03\x196\x01\x12a\x04\xCEW` \x81\x01s$<c\xD5\xDB\xCFa\x9E\xE3o\xDE\x7F\xF6=\x15d\xD5f[A\x81Rs\x96\x97!\x15R\x82mw\x14\xC0&}'OQ\x98O9\xD0``@\x83\x01R`@\x82Ra%\0``\x83a5\x89V[a%w`@\x92_\x84Qa%\x13\x86\x82a5\x89V[`\x12\x81R\x7FSyndicateToken.sol\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x85Q\x80\x94\x81\x92\x7F\x8D\x1C\xC9%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a3\xCBV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x91\x82\x15a3\x85W_\x92a3)W[P\x90` \x80\x93a%\xD9\x93\x86Q\x95\x84\x87\x95Q\x91\x82\x91\x01\x85\x87\x01^\x84\x01\x90\x83\x82\x01\x90_\x82RQ\x92\x83\x91^\x01\x01_\x81R\x03`\x1F\x19\x81\x01\x83R\x82a5\x89V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEW_a&+\x91\x83Q\x80\x93\x81\x92\x7F\xB4\xD6\xC7\x82\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01aL\xF3V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a3\x1FWa3\nW[P\x81\x80\x80\x80\x80s\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaNZ\xF1a&uaM\x1EV[\x90\x15a2aW\x82\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07vW\x81a&\xCF\x91\x84Q\x80\x93\x81\x92\x7F\xB4\xD6\xC7\x82\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01aL\xF3V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C\x9BWa2LW[PP\x80Qa\x01\xB8\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a28W\x90\x82\x91a\\\xF2\x839\x03\x90\x83\xF0\x80\x15a2-W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FUs\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` Us\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`\x01`\x01`\xA0\x1B\x03a'\xF5a;\xADV[\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\"T\x16\x17`\"U_\x80a(\xBEa(\xE6\x84Qa(4``\x82a5\x89V[`*\x81R\x7FGranting AIRDROP_MANAGER_ROLE us` \x82\x01R\x7Fing admin:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x82\x01R\x85Q\x92\x83\x91\x7F1\x9A\xF33\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R\x87`$\x84\x01R`d\x83\x01\x90a3\xCBV[s$<c\xD5\xDB\xCFa\x9E\xE3o\xDE\x7F\xF6=\x15d\xD5f[A`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a5\x89V[` \x81Q\x91\x01jconsole.logZ\xFAPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07vW\x81\x81Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs$<c\xD5\xDB\xCFa\x9E\xE3o\xDE\x7F\xF6=\x15d\xD5f[A`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C\x9BWa2\x18W[P`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x81;\x15a\x07rW\x82\x91`D\x83\x92\x86Q\x94\x85\x93\x84\x92\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t`\x04\x85\x01R`$\x84\x01RZ\xF1\x80\x15a\x0C\x9BWa2\x03W[P`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x81;\x15a\x07rW\x82\x91`D\x83\x92\x86Q\x94\x85\x93\x84\x92\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t`\x04\x85\x01R`$\x84\x01RZ\xF1\x80\x15a\x0C\x9BWa1\xEEW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07vW\x81\x81Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C\x9BWa1\xD9W[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90\x82Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t`\x04\x82\x01R\x82`$\x82\x01R` \x81`D\x81\x85Z\xFA\x90\x81\x15a\x0C_W\x85\x91a1\xBAW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D\x86Q\x80\x95\x81\x93\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t`\x04\x84\x01R`$\x83\x01RZ\xFA\x91\x82\x15a\x0C_W\x85\x92a1\x99W[P\x15a1\x16W\x15a0\x93W\x82\x90a,g\x83Qa,\x14``\x82a5\x89V[\x84\x81R\x7FSuccessfully granted AIRDROP_MAN` \x82\x01R\x7FAGER_ROLE to holder and contract\x85\x82\x01RaH\x95V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07vW\x82Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C\x9BWa0~W[PP`\x01`\x01`\xA0\x1B\x03` T\x16` `\x01`\x01`\xA0\x1B\x03`\"T\x16`$\x84Q\x80\x94\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x0C5W\x83\x91a0LW[Pi\x01\xB1\xAEMn.\xF5\0\0\0\x80\x91\x10a/\xEFWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07rW\x82\x82Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a/\xE5Wa/\xD0W[PPa-\xED\x82Qa-\xBE\x84\x82a5\x89V[`\x0F\x81R\x7FSetup complete:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaH\x95V[a.:`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x83Qa.\x0B\x85\x82a5\x89V[`\x11\x81R\x7FAirdrop contract:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI`V[a.x\x82Qa.I\x84\x82a5\x89V[`\x0B\x81R\x7FSYND token:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaH\xF2V[`\x01`\x01`\xA0\x1B\x03`\"T\x16\x91a.\xC4\x83\x82Qa.\x95\x84\x82a5\x89V[`\r\x81R\x7FToken holder:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI`V[` `\x01`\x01`\xA0\x1B\x03\x81T\x16\x93`$\x83Q\x80\x96\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x92\x83\x15a/\xC6W\x84\x93a/\x90W[Pa/Ua\x05L\x93\x82Qa/&\x84\x82a5\x89V[`\x15\x81R\x7FToken holder balance:\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI\xC5V[a/a\x81Q\x91\x82a5\x89V[`\x15\x81R\x7FTotal airdrop amount:\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RaI\xC5V[\x92P` \x83=` \x11a/\xBEW[\x81a/\xAB` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEW\x91Q\x91a/Ua/\x12V[=\x91Pa/\x9EV[\x81Q=\x86\x82>=\x90\xFD[\x81a/\xDA\x91a5\x89V[a\x07rW\x82_a-\xADV[\x84Q=\x84\x82>=\x90\xFD[`d\x82Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInsufficient balance for test\0\0\0`D\x82\x01R\xFD[\x90P` \x81=` \x11a0vW[\x81a0g` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ_a-1V[=\x91Pa0ZV[\x81a0\x88\x91a5\x89V[a\x07vW\x81_a,\xD5V[`\x84\x82Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FFailed to grant AIRDROP_MANAGER_`D\x82\x01R\x7FROLE to airdrop contract\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84\x83Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FFailed to grant AIRDROP_MANAGER_`D\x82\x01R\x7FROLE to token holder\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[a1\xB3\x91\x92P` =` \x11a\x07KWa\x07>\x81\x83a5\x89V[\x90_a+\xF7V[a1\xD3\x91P` =` \x11a\x07KWa\x07>\x81\x83a5\x89V[_a+\x7FV[\x81a1\xE3\x91a5\x89V[a\x07vW\x81_a*\xFCV[\x81a1\xF8\x91a5\x89V[a\x07vW\x81_a*\x91V[\x81a2\r\x91a5\x89V[a\x07vW\x81_a*\x06V[\x81a2\"\x91a5\x89V[a\x07vW\x81_a)~V[PQ\x90=\x90\x82>=\x90\xFD[`$\x85cNH{q`\xE0\x1B\x81R`A`\x04R\xFD[\x81a2V\x91a5\x89V[a\x07vW\x81_a&\xF4V[`\xA4\x82Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`X`$\x82\x01R\x7FStdCheats deployCodeTo(string,by`D\x82\x01R\x7Ftes,uint256,address): Failed to `d\x82\x01R\x7Fcreate runtime bytecode.\0\0\0\0\0\0\0\0`\x84\x82\x01R\xFD[a3\x17\x91\x92P_\x90a5\x89V[_\x90_a&PV[\x82Q=_\x82>=\x90\xFD[\x91P=\x80_\x84>a3:\x81\x84a5\x89V[\x82\x01\x91` \x81\x84\x03\x12a\x04\xCEW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x04\xCEW\x01\x91\x80`\x1F\x84\x01\x12\x15a\x04\xCEW` \x93a3|\x85\x92\x85\x84a%\xD9\x97Q\x91\x01aL\xBDV[\x93P\x93Pa%\x9EV[\x84Q=_\x82>=\x90\xFD[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a3\xACWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a3\x9FV[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a4\rWPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a4\0V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a4wWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a4\xB3\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa3\xCBV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a4hV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a4\xF4WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a5J\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a3\xF0V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a4\xE5V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a5uW`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a5uW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a5uW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x90\x81` \x91\x03\x12a\x04\xCEWQ\x90V[\x91\x90\x82\x01\x80\x92\x11a5\xE4WV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a5uW`\x05\x1B` \x01\x90V[\x90a6\x1A\x82a5\xF8V[a6'`@Q\x91\x82a5\x89V[\x82\x81R`\x1F\x19a67\x82\x94a5\xF8V[\x01\x90` 6\x91\x017V[\x80Q\x15a6NW` \x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q`\x01\x10\x15a6NW`@\x01\x90V[\x80Q`\x02\x10\x15a6NW``\x01\x90V[\x80Q`\x03\x10\x15a6NW`\x80\x01\x90V[\x80Q`\x04\x10\x15a6NW`\xA0\x01\x90V[\x80Q\x82\x10\x15a6NW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x04\xCEWQ\x80\x15\x15\x81\x03a\x04\xCEW\x90V[\x93\x92\x91\x90a6\xFE\x90s\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN\x86R`\x80` \x87\x01R`\x80\x86\x01\x90a3\x8FV[\x90\x84\x82\x03`@\x86\x01R` \x80\x82Q\x93\x84\x81R\x01\x91\x01\x91_[\x81\x81\x10a7(WPP``\x91P\x93\x01RV[\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a7\x16V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a7lW[` \x83\x10\x14a7XWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a7MV[_\x92\x91\x81T\x91a7\x85\x83a7>V[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a7\xDAWP`\x01\x14a7\xA1WPPPV[_\x90\x81R` \x81 \x93\x94P\x91\x92[\x83\x83\x10a7\xC0WP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a7\xAFV[\x90P` \x94\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x92\x91\x92\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10a:/Wa8\x87\x94T\x91\x81\x81\x10a9\xF9W[\x81\x81\x10a9\xC3W[\x81\x81\x10a9\x8DW[\x81\x81\x10a9WW[\x81\x81\x10a9!W[\x81\x81\x10a8\xEBW[\x81\x81\x10a8\xB6W[\x10a8\x89W[P\x03\x83a5\x89V[V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_a8\x7FV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01a8yV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01a8qV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01a8iV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01a8aV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01a8YV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01a8QV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01a8IV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91a81V[\x91\x90\x82\x03\x91\x82\x11a5\xE4WV[`\x08T`\xFF\x16\x80\x15a:\xD8W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a;\xA2W_\x91a;pW[P\x15\x15\x90V[\x90P` \x81=` \x11a;\x9AW[\x81a;\x8B` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ_a;jV[=\x91Pa;~V[`@Q=_\x82>=\x90\xFD[_`@Qa;\xBC`\x80\x82a5\x89V[`\x03\x81R``6` \x83\x017s\x124Vx\x90\x124Vx\x90\x124Vx\x90\x124Vx\x90a;\xE6\x82a6AV[Rs#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01a<\x05\x82a6bV[Rs4Vx\x90\x124Vx\x90\x124Vx\x90\x124Vx\x90\x12a<$\x82a6rV[R` T`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a<\xF8W`\x01`\x01`\xA0\x1B\x03a<N\x82\x85a6\xA2V[Q\x16`@Q\x90\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x90\x81\x15a;\xA2W_\x91a<\xC7W[Pi\x01\xB1\xAEMn.\xF5\0\0\0\x11\x15a<\xADW`\x01\x01a<3V[\x90P`\x01`\x01`\xA0\x1B\x03\x92Pa<\xC2\x91a6\xA2V[Q\x16\x90V[\x90P` \x81=\x82\x11a<\xF0W[\x81a<\xE1` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ_a<\x93V[=\x91Pa<\xD4V[PPP`@\x90\x81Q\x91a=\x0B\x81\x84a5\x89V[`\n\x83R` \x83\x01\x7FtestHolder\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81Q`\n` \x82\x01\x92\x83^_`*\x82\x01R`\n\x81Ra=V`*\x82a5\x89V[Q\x90 \x92\x81Q\x93\x7F\xFF\xA1\x86I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01R` \x84`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x93\x84\x15a3\x1FW_\x94aHQW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEW\x81Q\x7F\xC6W\xC7\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x81\x80a>\x18`\x01`\x01`\xA0\x1B\x03\x89\x16\x95\x86`\x04\x84\x01R\x87`$\x84\x01R`D\x83\x01\x90a3\xCBV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aHGWaH2W[P\x82\x80\x83Q` \x81\x01\x90\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x84`$\x82\x01R`$\x81Ra>\x7F`D\x82a5\x89V[Q\x90s\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaNZ\xFAPa>\xB3a>\xA4aM\x1EV[` \x80\x82Q\x83\x01\x01\x91\x01a5\xC8V[Ps\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x11T\x16\x17`\x11Ucp\xA0\x821\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0`\x0FT\x16\x17`\x0FU`\x0ETh\x01\0\0\0\0\0\0\0\0\x81\x10\x15aH\x1EW`\x01\x81\x01\x80`\x0EU\x81\x10\x15aH\nW`\x0E\x84R` \x84 \x01U`\x01`\x01`\xA0\x1B\x03`\x11T\x16`\x0FT`\xE0\x1B`\x10T\x90a?kaMdV[\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a?\xA8\x85`\x01`\x01`\xA0\x1B\x03\x16_R`\r` R`@_ \x90V[\x91\x16\x90\x81\x87R` R\x84\x86 \x85Q` \x81\x01\x90a?\xD8\x81a?\xCA\x88\x88\x86aMMV[\x03`\x1F\x19\x81\x01\x83R\x82a5\x89V[Q\x90 \x87R` R`\xFF`\x03\x86\x88 \x01T\x16\x15aG\xFCW[a@\x0B\x84`\x01`\x01`\xA0\x1B\x03\x16_R`\r` R`@_ \x90V[\x90\x86R` Ra?\xCAa@+\x85\x87 \x93\x86Q\x92\x83\x91` \x83\x01\x95\x86aMMV[Q\x90 \x84R` R\x81\x83 \x90`\x01\x82\x01T\x91`\x02\x81\x01Ta@L\x81\x85a5\xD7V[aF\x15W[\x81T\x85Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x94\x90\x91\x90` \x86`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x95\x86\x15aF\x0BW\x88\x96aE\xD7W[P`\x01\x90\x82\x01a\x01\0\x03\x1B_\x19\x01\x81\x1B\x19\x85\x16\x91sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15aE\xD3W\x86Q\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91Ri\x03c\\\x9A\xDC]\xEA\0\0\0\x90\x91\x1B\x91\x90\x91\x17`D\x82\x01R\x85\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aE\xC9W\x90\x86\x91aE\xB4W[PP\x84\x80` `\x0FT`\xE0\x1BaA\xD2`$aA\x85`\raN=V[\x8AQ\x94\x85\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01\x96\x16\x86R\x80Q\x91\x82\x91\x01\x84\x84\x01^\x81\x01\x86\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x84R\x83a5\x89V[`\x01`\x01`\xA0\x1B\x03`\x11T\x16\x91Q\x91Z\xFAaA\xEBaM\x1EV[\x90`\x10T\x80`\x05\x1B\x90\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03aE\xA0W\x87\x92\x80Q` \x81\x11_\x14aE\x9AWP` \x90[\x89\x92[\x82\x84\x10aE\x12WPPPP\x15\x90\x81\x15aD\xFDW[PaC\xD2WPPPP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x11T\x16`\x11U\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0`\x0FT\x16`\x0FU`\x0ET\x81`\x0EU\x80aC\x90W[P\x80`\x10U\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x13T\x16`\x13UaB\xE4`\x14Ta7>V[\x80aB\xEEWPP\x90V[`\x1F\x81\x11`\x01\x14aC\x01WP`\x14U[\x90V[`\x14\x82R`\x1F\x01`\x05\x1C\x7F\xCEm{R\x82\xBD\x9A6a\xAE\x06\x1F\xEE\xD1\xDB\xDANR\xAB\x07;\x1F\x92\x85\xBEn\x15]\x9C8\xD4\xEC\x01\x7F\xCEm{R\x82\xBD\x9A6a\xAE\x06\x1F\xEE\xD1\xDB\xDANR\xAB\x07;\x1F\x92\x85\xBEn\x15]\x9C8\xD4\xED[\x81\x81\x10aC\x85WPP`\x14\x80\x82R\x81\x90U\x7F\xCEm{R\x82\xBD\x9A6a\xAE\x06\x1F\xEE\xD1\xDB\xDANR\xAB\x07;\x1F\x92\x85\xBEn\x15]\x9C8\xD4\xECU\x90V[_\x81U`\x01\x01aCOV[`\x0E\x82R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x90\x81\x01\x90[\x81\x81\x10aC\xC7WPaB\xACV[\x82\x81U`\x01\x01aC\xBAV[T\x84\x91sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07rWaD@\x85Q\x94\x85\x93\x84\x93\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01`@\x91\x94\x93\x92`\x01`\x01`\xA0\x1B\x03``\x83\x01\x96\x16\x82R` \x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x0C5WaD\xE8W[`\x84\x82Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FstdStorage find(StdStorage): Fai`D\x82\x01R\x7Fled to write value.\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[aD\xF3\x83\x80\x92a5\x89V[a\x07vW\x81aDeV[i\x03c\\\x9A\xDC]\xEA\0\0\0\x91P\x14\x15_aBFV[\x90\x91\x92\x94aE \x86\x83a5\xD7V[\x83Q\x81\x10\x15aE\x86W` \x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x85\x01\x01Q\x16\x86`\x03\x1B\x87\x81\x04`\x08\x14\x88\x15\x17\x15aErW\x1C\x17\x94`\x01\x01\x92\x91\x90aB2V[`$\x8DcNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`$\x8CcNH{q`\xE0\x1B\x81R`2`\x04R\xFD[\x90aB/V[`$\x88cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[\x81aE\xBE\x91a5\x89V[a\x06\x8AW\x84_aAjV[\x85Q=\x88\x82>=\x90\xFD[\x87\x80\xFD[\x90\x95P` \x81=` \x11aF\x03W[\x81aE\xF3` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ\x94\x81a@\xBDV[=\x91PaE\xE6V[\x87Q=\x8A\x82>=\x90\xFD[aF\x1F\x81\x85a5\xD7V[a\x01\0\x03a\x01\0\x81\x11aG\xE8W`\xFF\x81\x11aG\xE8W`\x01\x90\x1B\x85Q\x7Fi\0\xA3\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81`\x04\x82\x01R\x87\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15aF\x0BW\x88\x91aG\x84W[P\x90aG,`j` \x93\x89Q\x94\x85\x91\x7FstdStorage find(StdStorage): Pac\x82\x84\x01R\x7Fked slot. We can't fit value gre\x8C\x84\x01R\x7Fater than \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x01R\x80Q\x91\x82\x91\x01\x84\x84\x01^\x81\x01\x8B\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x84R\x83a5\x89V[i\x03c\\\x9A\xDC]\xEA\0\0\0\x10\x15aGCWPa@QV[aG\x80\x90\x86Q\x91\x82\x91\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a3\xCBV[\x03\x90\xFD[\x90P=\x80\x89\x83>aG\x95\x81\x83a5\x89V[\x81\x01\x90` \x81\x83\x03\x12aG\xE0W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11aG\xE4W\x01\x81`\x1F\x82\x01\x12\x15aG\xE0W`jaG\xD7` \x94\x93\x83\x86aG,\x95Q\x91\x01aL\xBDV[\x92\x93PPaF\x8DV[\x88\x80\xFD[\x89\x80\xFD[`$\x87cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[aH\x04aP\xA6V[Pa?\xF0V[`$\x84cNH{q`\xE0\x1B\x81R`2`\x04R\xFD[`$\x84cNH{q`\xE0\x1B\x81R`A`\x04R\xFD[aH?\x91\x93P_\x90a5\x89V[_\x91_a>=V[\x83Q=_\x82>=\x90\xFD[\x90\x93P` \x81=` \x11aH\x8DW[\x81aHm` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x04\xCEW\x92_a=\xB0V[=\x91PaH`V[_a?\xCAaH\xDB\x82\x93`@Q\x92\x83\x91\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R` `$\x84\x01R`D\x83\x01\x90a3\xCBV[` \x81Q\x91\x01jconsole.logZ\xFAPV[_aI8aH\xDB\x82\x93`@Q\x92\x83\x91\x7F1\x9A\xF33\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R`@`$\x84\x01R`d\x83\x01\x90a3\xCBV[s\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a5\x89V[aI\xB0aH\xDB_\x93\x92\x84\x93`\x01`\x01`\xA0\x1B\x03`@Q\x94\x85\x93\x7F1\x9A\xF33\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x86\x01R`@`$\x86\x01R`d\x85\x01\x90a3\xCBV[\x91\x16`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a5\x89V[aJ\raH\xDB_\x93\x92\x84\x93`@Q\x93\x84\x92\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a3\xCBV[\x90`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a5\x89V[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEW_\x91aJ\x88`@Q\x94\x85\x93\x84\x93\x7F\x88\xB4L\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01R`$\x84\x01R```D\x84\x01R`d\x83\x01\x90a3\xCBV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a;\xA2WaJ\xAEWPV[_a8\x87\x91a5\x89V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEWaJ\x88\x91_\x91`@Q\x93\x84\x92\x83\x92\x7F\xA3N\xDC\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x15\x15`\x04\x84\x01R`@`$\x84\x01R`D\x83\x01\x90a3\xCBV[`@QaK*`\xC0\x82a5\x89V[`\x05\x81R_[`\xA0\x81\x10aL\x9AWP`@QaKE\x81a5YV[s\x124Vx\x90\x124Vx\x90\x124Vx\x90\x124Vx\x90\x81Rh65\xC9\xAD\xC5\xDE\xA0\0\0` \x82\x01RaKt\x82a6AV[RaK~\x81a6AV[P`@QaK\x8B\x81a5YV[s#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01\x81Rhlk\x93[\x8B\xBD@\0\0` \x82\x01RaK\xBA\x82a6bV[RaK\xC4\x81a6bV[P`@QaK\xD1\x81a5YV[s4Vx\x90\x124Vx\x90\x124Vx\x90\x124Vx\x90\x12\x81RhQP\xAE\x84\xA8\xCD\xF0\0\0` \x82\x01RaL\0\x82a6rV[RaL\n\x81a6rV[P`@QaL\x17\x81a5YV[sEg\x89\x01#Eg\x89\x01#Eg\x89\x01#Eg\x89\x01#\x81Rh\xA2\xA1]\tQ\x9B\xE0\0\0` \x82\x01RaLF\x82a6\x82V[RaLP\x81a6\x82V[P`@QaL]\x81a5YV[sVx\x90\x124Vx\x90\x124Vx\x90\x124Vx\x90\x124\x81Rh\x1B\x1A\xE4\xD6\xE2\xEFP\0\0` \x82\x01RaL\x8C\x82a6\x92V[RaL\x96\x81a6\x92V[P\x90V[` \x90`@QaL\xA9\x81a5YV[_\x81R_\x83\x82\x01R\x82\x82\x85\x01\x01R\x01aK0V[\x92\x91\x92aL\xC9\x82a5\xACV[\x91aL\xD7`@Q\x93\x84a5\x89V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x04\xCEW\x82\x81` \x93\x84_\x96\x01^\x01\x01RV[\x90`@aB\xFE\x92s\x1B\xAB\x80H\x03\x15\x9A\xD8K\x88TX\x1A\xA5:\xC7$UaN\x81R\x81` \x82\x01R\x01\x90a3\xCBV[=\x15aMHW=\x90aM/\x82a5\xACV[\x91aM=`@Q\x93\x84a5\x89V[\x82R=_` \x84\x01>V[``\x90V[` \x92\x91\x90\x83\x91\x80Q\x92\x83\x91\x01\x82^\x01\x90\x81R\x01\x90V[aMo`\x14Ta7>V[aN+W`@Q\x90\x81\x82` `\x0ET\x92\x83\x81R\x01`\x0E_R` _ \x92_[\x81\x81\x10aN\x12WPPaM\xA3\x92P\x03\x83a5\x89V[\x81Q\x80`\x05\x1B\x90\x80\x82\x04` \x14\x90\x15\x17\x15a5\xE4W`\x1F\x19aM\xDDaM\xC7\x83a5\xACV[\x92aM\xD5`@Q\x94\x85a5\x89V[\x80\x84Ra5\xACV[\x016` \x83\x017_[\x83Q\x81\x10\x15aN\rW\x80aM\xFC`\x01\x92\x86a6\xA2V[Q` \x82`\x05\x1B\x85\x01\x01R\x01aM\xE6V[P\x91PV[\x84T\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01aM\x8EV[`@QaB\xFE\x81a\x08K\x81`\x14a7vV[`\x07\x81\x01\x90aNL\x82Ta7>V[aN\xF1W`\x01\x91P\x01\x90`@Q\x80\x83` \x82\x95T\x93\x84\x81R\x01\x90_R` _ \x92_[\x81\x81\x10aN\xD8WPPaN\x84\x92P\x03\x83a5\x89V[\x81Q\x80`\x05\x1B\x90\x80\x82\x04` \x14\x90\x15\x17\x15a5\xE4W`\x1F\x19aN\xA8aM\xC7\x83a5\xACV[\x016` \x83\x017_[\x83Q\x81\x10\x15aN\rW\x80aN\xC7`\x01\x92\x86a6\xA2V[Q` \x82`\x05\x1B\x85\x01\x01R\x01aN\xB1V[\x84T\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01aNoV[PaB\xFEa\x08K\x91`@Q\x92\x83\x80\x92a7vV[\x90_\x80` `\x02\x85\x01T`\xE0\x1BaOn`$aO \x88aN=V[`@Q\x94\x85\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01\x96\x16\x86R\x80Q\x91\x82\x91\x01\x84\x84\x01^\x81\x01\x86\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x84R\x83a5\x89V[`\x01`\x01`\xA0\x1B\x03`\x04\x87\x01T\x16\x91Q\x91Z\xFA`\x03aO\x8BaM\x1EV[\x93\x01T\x80`\x05\x1B\x90\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a5\xE4W_\x93\x80Q` \x81\x11_\x14aPCWP` \x90[_\x92[\x82\x84\x10aO\xE0WPPPP\x91\x90V[\x90\x91\x92\x95aO\xEE\x87\x83a5\xD7V[\x83Q\x81\x10\x15a6NW` \x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x85\x01\x01Q\x16\x90\x87`\x03\x1B\x91\x88\x83\x04`\x08\x14\x89\x15\x17\x15a5\xE4W`\x01\x92\x1C\x17\x96\x01\x92\x91\x90aO\xD1V[\x90aO\xCEV[\x90\x80`\x1F\x83\x01\x12\x15a\x04\xCEW\x81QaP`\x81a5\xF8V[\x92aPn`@Q\x94\x85a5\x89V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x04\xCEW` \x01\x90[\x82\x82\x10aP\x96WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01aP\x89V[_`\x01`\x01`\xA0\x1B\x03`\x11T\x16\x90`\x0FT`\xE0\x1B`\x10T\x90aP\xC8`\raN=V[\x90\x84_R`\r` R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@_ \x91\x16\x90\x81_R` R`@_ `@Q` \x81\x01\x90aQ\x1A\x81a?\xCA\x88\x88\x86aMMV[Q\x90 _R` R`\xFF`\x03`@_ \x01T\x16aZ\xC9Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEW`@Q\x7F&l\xF1\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a;\xA2WaZ\xB4W[PaQ\xA5`\raO\x05V[\x90P`@Q\x7Fe\xBC\x94\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x86`\x04\x82\x01R\x85\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x16\xE1W\x86\x91aZNW[P\x80Q\x80aR\x8AW`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FstdStorage find(StdStorage): No `D\x82\x01R\x7Fstorage use detected for target.`d\x82\x01R\xFD[\x80\x15aG\xE8W\x90_\x19aR\xEF\x92\x01\x90` aR\xA5\x83\x83a6\xA2V[Q`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x93\x84\x90\x81\x90`D\x82\x01\x90V[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x92\x83\x15a\x0E\xE4W\x88\x93aZ\x1BW[P\x82\x15aY\xC3W[aS(\x82\x82a6\xA2V[Q`\x11T`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01\x81\x90R`$\x82\x01\x83\x90R\x91\x90` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15aY\xB8W\x8B\x91aY\x87W[PaS\xA3`\raO\x05V[\x91\x90\x93\x82\x15_\x14aY\x80W_\x19\x90[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15aYgW`@Q\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x81\x01\x91\x90\x91R\x8C\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aX\xE4W\x90\x8D\x91aYkW[PPaTJ`\raO\x05V[\x93\x90P`\x01`\x01`\xA0\x1B\x03`\x11T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15aYgW`@Q\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x81\x01\x91\x90\x91R\x8B\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aY\\W\x90\x8C\x91aYCW[PP\x82aY8W[PP\x15aY1W\x87\x93\x88\x93`\xFF`\x13T\x16aWaW[`\x01\x86\x86\x01a\x01\0\x03\x1B_\x19\x01\x85\x1B\x16\x84\x1C\x81\x03aWXWP\x90aU\x8A\x91\x7F\x9C\x95U\xB1\xE3\x10.<\xF4\x8FB}y\xCBg\x8F]\x9B\xD1\xED\n\xD5t8\x94a\xE2U\xF9Qp\xED`\x80\x8B\x89a?\xCAaU_\x8D`@Q\x92\x83\x91` \x83\x01\x95\x86aMMV[Q\x90 aUl\x86\x86a6\xA2V[Q\x90`@Q\x92\x83R\x8A` \x84\x01R`@\x83\x01R``\x82\x01R\xA1a6\xA2V[Q\x90`@Q\x91`\x80\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aWDW\x90`\x03\x93\x92\x91`@R\x82R` \x82\x01\x93\x84R`@\x82\x01\x90\x81R``\x82\x01\x93`\x01\x85R\x89\x89R`\r` R`@\x89 \x86\x8AR` R`@\x89 `@Q` \x81\x01\x90aU\xF6\x81a?\xCA\x8D\x8D\x86aMMV[Q\x90 \x8AR` R`@\x89 \x92Q\x83UQ`\x01\x83\x01UQ`\x02\x82\x01U\x01\x90Q\x15\x15`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83T\x16\x91\x16\x17\x90U\x84\x84R`\r` R`@\x84 \x81\x85R` R`@\x84 `@Q` \x81\x01\x90aVo\x81a?\xCA\x88\x88\x86aMMV[Q\x90 \x85R` R`\xFF`\x03`@\x86 \x01T\x16\x15aV\xC0W`@\x94\x84R`\r` R\x84\x84 \x90\x84R` Ra?\xCAaV\xB4\x85\x85 \x93\x86Q\x92\x83\x91` \x83\x01\x95\x86aMMV[Q\x90 \x82R` R \x90V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FstdStorage find(StdStorage): Slo`D\x82\x01R\x7Ft(s) not found.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`$\x89cNH{q`\xE0\x1B\x81R`A`\x04R\xFD[\x93P\x91PaR\x8AV[\x94PaW\xCD\x93PaWr\x83\x83a6\xA2V[Q\x94` \x86`\x01`\x01`\xA0\x1B\x03`\x11T\x16`@Q\x97\x88\x92\x83\x92\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x94\x85\x15aY&W\x8A\x95aX\xF3W[PaX\0\x86`\ra[\nV[\x95\x90\x96aX\x0E\x81`\ra\\\x08V[\x92\x90\x91`\x01`\x01`\xA0\x1B\x03`\x11T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15aX\xEFW`@Q\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x81\x01\x91\x90\x91R\x8C\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aX\xE4W\x90\x8D\x91aX\xCBW[PP\x87aX\xC3W[P\x94\x95aU\x04WP\x93P\x91PaR\x8AV[\x96P_aX\xB2V[\x81aX\xD5\x91a5\x89V[aX\xE0W\x8B_aX\xAAV[\x8B\x80\xFD[`@Q=\x8F\x82>=\x90\xFD[\x8E\x80\xFD[\x90\x94P` \x81=\x82\x11aY\x1EW[\x81aY\x0E` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ\x93_aW\xF4V[=\x91PaY\x01V[`@Q=\x8C\x82>=\x90\xFD[\x91PaR\x8AV[\x14\x15\x90P_\x80aT\xEEV[\x81aYM\x91a5\x89V[aYXW\x8A_aT\xE6V[\x8A\x80\xFD[`@Q=\x8E\x82>=\x90\xFD[\x8D\x80\xFD[\x81aYu\x91a5\x89V[aX\xE0W\x8B_aT>V[\x8C\x90aS\xB2V[\x90P` \x81=\x82\x11aY\xB0W[\x81aY\xA1` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ_aS\x98V[=\x91PaY\x94V[`@Q=\x8D\x82>=\x90\xFD[\x7F\x08\x0F\xC4\xA9f \xC4F.p[#\xF3FA?\xE3yk\xB6<o\x8D\x85\x91\xBA\xEC\x0E#\x15w\xA5aZ\x13aY\xF1\x84\x84a6\xA2V[Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8E\x16\x81R` \x81\x01\x92\x90\x92R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1aS\x1EV[\x90\x92P` \x81=\x82\x11aZFW[\x81aZ6` \x93\x83a5\x89V[\x81\x01\x03\x12a\x04\xCEWQ\x91_aS\x16V[=\x91PaZ)V[\x90P=\x80\x87\x83>aZ_\x81\x83a5\x89V[\x81\x01`@\x82\x82\x03\x12aZ\xB0W\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11aE\xD3W\x81aZ\x88\x91\x84\x01aPIV[\x91` \x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11aG\xE0WaZ\xA9\x92\x91\x01aPIV[P_aQ\xFEV[\x86\x80\xFD[aZ\xC1\x91\x94P_\x90a5\x89V[_\x92_aQ\x9AV[\x91\x93\x90\x92P_R`\r` R`@_ \x90_R` Ra?\xCAaZ\xFB`@_ \x93`@Q\x92\x83\x91` \x83\x01\x95\x86aMMV[Q\x90 _R` R`@_ \x90V[\x91\x90_[a\x01\0\x81\x10a[!WPP\x90P_\x90_\x90V[\x80`\xFF\x03`\xFF\x81\x11a5\xE4W`\x01\x90\x1B`\x01`\x01`\xA0\x1B\x03`\x04\x86\x01T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEW`@Q\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x81\x01\x91\x90\x91R_\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a;\xA2Wa[\xF8W[Pa[\xD2\x84aO\x05V[\x81a[\xEEW[Pa[\xE5W`\x01\x01a[\x0EV[\x92PP`\x01\x91\x90V[\x90P\x15\x15_a[\xD8V[_a\\\x02\x91a5\x89V[_a[\xC8V[\x91\x90_[a\x01\0\x81\x10a\\\x1FWPP\x90P_\x90_\x90V[`\x01\x81\x1B`\x01`\x01`\xA0\x1B\x03`\x04\x86\x01T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xCEW`@Q\x7Fp\xCA\x10\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x81\x01\x91\x90\x91R_\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a;\xA2Wa\\\xE1W[Pa\\\xC4\x84aO\x05V[\x81a\\\xD7W[Pa[\xE5W`\x01\x01a\\\x0CV[\x90P\x15\x15_a\\\xCAV[_a\\\xEB\x91a5\x89V[_a\\\xBAV\xFE`\x80\x80`@R4`\x15Wa\x01\x9E\x90\x81a\0\x1A\x829\xF3[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1Cc\x82\x94z\xBE\x14a\0$W_\x80\xFD[`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0\xC1W`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\0\xC1W`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xC1Wa\0\x8E\x906\x90`\x04\x01a\0\xC5V[`D5\x92\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11a\0\xC1Wa\0\xB3a\0\xBF\x946\x90`\x04\x01a\0\xC5V[\x92\x90\x91`d5\x94a\0\xF6V[\0[_\x80\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\0\xC1W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xC1W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\0\xC1WV[\x91\x80\x93\x95\x91\x94\x03a\0\xC1W\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R0`$R`DR_\x80`d\x81\x80\x85Z\xF1\x15a\0\xC1W\x91\x90\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x05\x1B\x81\x01\x92\x81\x03\x90[\x805`\x04R\x81\x81\x035`$R_\x80`d\x81\x80\x87Z\xF1\x15a\0\xC1W` \x01\x91\x83\x83\x10\x15a\x01\x98W\x91a\x01lV[PPPPV",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SlotFound(address,bytes4,bytes32,uint256)` and selector `0x9c9555b1e3102e3cf48f427d79cb678f5d9bd1ed0ad574389461e255f95170ed`.
```solidity
event SlotFound(address who, bytes4 fsig, bytes32 keysHash, uint256 slot);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlotFound {
        #[allow(missing_docs)]
        pub who: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub fsig: alloy::sol_types::private::FixedBytes<4>,
        #[allow(missing_docs)]
        pub keysHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub slot: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for SlotFound {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlotFound(address,bytes4,bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8, 149u8, 85u8, 177u8, 227u8, 16u8, 46u8, 60u8, 244u8, 143u8, 66u8,
                125u8, 121u8, 203u8, 103u8, 143u8, 93u8, 155u8, 209u8, 237u8, 10u8,
                213u8, 116u8, 56u8, 148u8, 97u8, 226u8, 85u8, 249u8, 81u8, 112u8, 237u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    who: data.0,
                    fsig: data.1,
                    keysHash: data.2,
                    slot: data.3,
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
                        &self.who,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.fsig),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.keysHash),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
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
        impl alloy_sol_types::private::IntoLogData for SlotFound {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlotFound> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SlotFound) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `WARNING_UninitedSlot(address,uint256)` and selector `0x080fc4a96620c4462e705b23f346413fe3796bb63c6f8d8591baec0e231577a5`.
```solidity
event WARNING_UninitedSlot(address who, uint256 slot);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct WARNING_UninitedSlot {
        #[allow(missing_docs)]
        pub who: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub slot: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for WARNING_UninitedSlot {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "WARNING_UninitedSlot(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                8u8, 15u8, 196u8, 169u8, 102u8, 32u8, 196u8, 70u8, 46u8, 112u8, 91u8,
                35u8, 243u8, 70u8, 65u8, 63u8, 227u8, 121u8, 107u8, 182u8, 60u8, 111u8,
                141u8, 133u8, 145u8, 186u8, 236u8, 14u8, 35u8, 21u8, 119u8, 165u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { who: data.0, slot: data.1 }
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
                        &self.who,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
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
        impl alloy_sol_types::private::IntoLogData for WARNING_UninitedSlot {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&WARNING_UninitedSlot> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &WARNING_UninitedSlot) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `AIRDROP_MANAGER_ROLE()` and selector `0x8a542521`.
```solidity
function AIRDROP_MANAGER_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AIRDROP_MANAGER_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = alloy::sol_types::private::FixedBytes<32>;
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
                        let r: AIRDROP_MANAGER_ROLEReturn = r.into();
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
                        let r: AIRDROP_MANAGER_ROLEReturn = r.into();
                        r._0
                    })
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
    /**Function with signature `airdrop()` and selector `0x3884d635`.
```solidity
function airdrop() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct airdropCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`airdrop()`](airdropCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct airdropReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<airdropCall> for UnderlyingRustTuple<'_> {
                fn from(value: airdropCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for airdropCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<airdropReturn> for UnderlyingRustTuple<'_> {
                fn from(value: airdropReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for airdropReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for airdropCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "airdrop()";
            const SELECTOR: [u8; 4] = [56u8, 132u8, 214u8, 53u8];
            #[inline]
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
                        let r: airdropReturn = r.into();
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
                        let r: airdropReturn = r.into();
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
    /**Function with signature `syndToken()` and selector `0x6213821d`.
```solidity
function syndToken() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct syndTokenCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`syndToken()`](syndTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct syndTokenReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<syndTokenCall> for UnderlyingRustTuple<'_> {
                fn from(value: syndTokenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for syndTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<syndTokenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: syndTokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for syndTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for syndTokenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "syndToken()";
            const SELECTOR: [u8; 4] = [98u8, 19u8, 130u8, 29u8];
            #[inline]
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
                        let r: syndTokenReturn = r.into();
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
                        let r: syndTokenReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `syndTokenAccessControl()` and selector `0xe52a2f1f`.
```solidity
function syndTokenAccessControl() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct syndTokenAccessControlCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`syndTokenAccessControl()`](syndTokenAccessControlCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct syndTokenAccessControlReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<syndTokenAccessControlCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: syndTokenAccessControlCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for syndTokenAccessControlCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<syndTokenAccessControlReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: syndTokenAccessControlReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for syndTokenAccessControlReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for syndTokenAccessControlCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "syndTokenAccessControl()";
            const SELECTOR: [u8; 4] = [229u8, 42u8, 47u8, 31u8];
            #[inline]
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
                        let r: syndTokenAccessControlReturn = r.into();
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
                        let r: syndTokenAccessControlReturn = r.into();
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
    /**Function with signature `test_DeployAirdrop()` and selector `0x81a8de7e`.
```solidity
function test_DeployAirdrop() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_DeployAirdropCall;
    ///Container type for the return parameters of the [`test_DeployAirdrop()`](test_DeployAirdropCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_DeployAirdropReturn {}
    #[allow(
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
            impl ::core::convert::From<test_DeployAirdropCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_DeployAirdropCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_DeployAirdropCall {
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
            impl ::core::convert::From<test_DeployAirdropReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_DeployAirdropReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_DeployAirdropReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_DeployAirdropReturn {
            fn _tokenize(
                &self,
            ) -> <test_DeployAirdropCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_DeployAirdropCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_DeployAirdropReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_DeployAirdrop()";
            const SELECTOR: [u8; 4] = [129u8, 168u8, 222u8, 126u8];
            #[inline]
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
                test_DeployAirdropReturn::_tokenize(ret)
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
    /**Function with signature `test_ExecuteAirdrop()` and selector `0x85a468f7`.
```solidity
function test_ExecuteAirdrop() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteAirdropCall;
    ///Container type for the return parameters of the [`test_ExecuteAirdrop()`](test_ExecuteAirdropCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteAirdropReturn {}
    #[allow(
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
            impl ::core::convert::From<test_ExecuteAirdropCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteAirdropCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteAirdropCall {
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
            impl ::core::convert::From<test_ExecuteAirdropReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteAirdropReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteAirdropReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_ExecuteAirdropReturn {
            fn _tokenize(
                &self,
            ) -> <test_ExecuteAirdropCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_ExecuteAirdropCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_ExecuteAirdropReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_ExecuteAirdrop()";
            const SELECTOR: [u8; 4] = [133u8, 164u8, 104u8, 247u8];
            #[inline]
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
                test_ExecuteAirdropReturn::_tokenize(ret)
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
    /**Function with signature `test_ExecuteAirdropBatches()` and selector `0xbff8600f`.
```solidity
function test_ExecuteAirdropBatches() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteAirdropBatchesCall;
    ///Container type for the return parameters of the [`test_ExecuteAirdropBatches()`](test_ExecuteAirdropBatchesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteAirdropBatchesReturn {}
    #[allow(
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
            impl ::core::convert::From<test_ExecuteAirdropBatchesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteAirdropBatchesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteAirdropBatchesCall {
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
            impl ::core::convert::From<test_ExecuteAirdropBatchesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteAirdropBatchesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteAirdropBatchesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_ExecuteAirdropBatchesReturn {
            fn _tokenize(
                &self,
            ) -> <test_ExecuteAirdropBatchesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_ExecuteAirdropBatchesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_ExecuteAirdropBatchesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_ExecuteAirdropBatches()";
            const SELECTOR: [u8; 4] = [191u8, 248u8, 96u8, 15u8];
            #[inline]
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
                test_ExecuteAirdropBatchesReturn::_tokenize(ret)
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
    /**Function with signature `test_ExecuteAirdropWithArrayLengthMismatch()` and selector `0x1b7986d2`.
```solidity
function test_ExecuteAirdropWithArrayLengthMismatch() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteAirdropWithArrayLengthMismatchCall;
    ///Container type for the return parameters of the [`test_ExecuteAirdropWithArrayLengthMismatch()`](test_ExecuteAirdropWithArrayLengthMismatchCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteAirdropWithArrayLengthMismatchReturn {}
    #[allow(
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
            impl ::core::convert::From<test_ExecuteAirdropWithArrayLengthMismatchCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteAirdropWithArrayLengthMismatchCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteAirdropWithArrayLengthMismatchCall {
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
            impl ::core::convert::From<test_ExecuteAirdropWithArrayLengthMismatchReturn>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_ExecuteAirdropWithArrayLengthMismatchReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteAirdropWithArrayLengthMismatchReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_ExecuteAirdropWithArrayLengthMismatchReturn {
            fn _tokenize(
                &self,
            ) -> <test_ExecuteAirdropWithArrayLengthMismatchCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_ExecuteAirdropWithArrayLengthMismatchCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_ExecuteAirdropWithArrayLengthMismatchReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_ExecuteAirdropWithArrayLengthMismatch()";
            const SELECTOR: [u8; 4] = [27u8, 121u8, 134u8, 210u8];
            #[inline]
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
                test_ExecuteAirdropWithArrayLengthMismatchReturn::_tokenize(ret)
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
    /**Function with signature `test_ExecuteAirdropWithInsufficientAllowance()` and selector `0x94a2a5df`.
```solidity
function test_ExecuteAirdropWithInsufficientAllowance() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteAirdropWithInsufficientAllowanceCall;
    ///Container type for the return parameters of the [`test_ExecuteAirdropWithInsufficientAllowance()`](test_ExecuteAirdropWithInsufficientAllowanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteAirdropWithInsufficientAllowanceReturn {}
    #[allow(
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
            impl ::core::convert::From<test_ExecuteAirdropWithInsufficientAllowanceCall>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_ExecuteAirdropWithInsufficientAllowanceCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteAirdropWithInsufficientAllowanceCall {
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
            impl ::core::convert::From<
                test_ExecuteAirdropWithInsufficientAllowanceReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_ExecuteAirdropWithInsufficientAllowanceReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteAirdropWithInsufficientAllowanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_ExecuteAirdropWithInsufficientAllowanceReturn {
            fn _tokenize(
                &self,
            ) -> <test_ExecuteAirdropWithInsufficientAllowanceCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_ExecuteAirdropWithInsufficientAllowanceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_ExecuteAirdropWithInsufficientAllowanceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_ExecuteAirdropWithInsufficientAllowance()";
            const SELECTOR: [u8; 4] = [148u8, 162u8, 165u8, 223u8];
            #[inline]
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
                test_ExecuteAirdropWithInsufficientAllowanceReturn::_tokenize(ret)
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
    /**Function with signature `test_TokenInformation()` and selector `0x59c620c6`.
```solidity
function test_TokenInformation() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_TokenInformationCall;
    ///Container type for the return parameters of the [`test_TokenInformation()`](test_TokenInformationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_TokenInformationReturn {}
    #[allow(
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
            impl ::core::convert::From<test_TokenInformationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_TokenInformationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_TokenInformationCall {
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
            impl ::core::convert::From<test_TokenInformationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_TokenInformationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_TokenInformationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_TokenInformationReturn {
            fn _tokenize(
                &self,
            ) -> <test_TokenInformationCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_TokenInformationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_TokenInformationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_TokenInformation()";
            const SELECTOR: [u8; 4] = [89u8, 198u8, 32u8, 198u8];
            #[inline]
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
                test_TokenInformationReturn::_tokenize(ret)
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
    /**Function with signature `tokenHolder()` and selector `0x420a83e7`.
```solidity
function tokenHolder() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenHolderCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`tokenHolder()`](tokenHolderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenHolderReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tokenHolderCall> for UnderlyingRustTuple<'_> {
                fn from(value: tokenHolderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenHolderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<tokenHolderReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tokenHolderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenHolderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tokenHolderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tokenHolder()";
            const SELECTOR: [u8; 4] = [66u8, 10u8, 131u8, 231u8];
            #[inline]
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
                        let r: tokenHolderReturn = r.into();
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
                        let r: tokenHolderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`AirdropScriptTest`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum AirdropScriptTestCalls {
        #[allow(missing_docs)]
        AIRDROP_MANAGER_ROLE(AIRDROP_MANAGER_ROLECall),
        #[allow(missing_docs)]
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        airdrop(airdropCall),
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
        syndToken(syndTokenCall),
        #[allow(missing_docs)]
        syndTokenAccessControl(syndTokenAccessControlCall),
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
        test_DeployAirdrop(test_DeployAirdropCall),
        #[allow(missing_docs)]
        test_ExecuteAirdrop(test_ExecuteAirdropCall),
        #[allow(missing_docs)]
        test_ExecuteAirdropBatches(test_ExecuteAirdropBatchesCall),
        #[allow(missing_docs)]
        test_ExecuteAirdropWithArrayLengthMismatch(
            test_ExecuteAirdropWithArrayLengthMismatchCall,
        ),
        #[allow(missing_docs)]
        test_ExecuteAirdropWithInsufficientAllowance(
            test_ExecuteAirdropWithInsufficientAllowanceCall,
        ),
        #[allow(missing_docs)]
        test_TokenInformation(test_TokenInformationCall),
        #[allow(missing_docs)]
        tokenHolder(tokenHolderCall),
    }
    impl AirdropScriptTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [10u8, 146u8, 84u8, 228u8],
            [27u8, 121u8, 134u8, 210u8],
            [30u8, 215u8, 131u8, 28u8],
            [42u8, 222u8, 56u8, 128u8],
            [56u8, 132u8, 214u8, 53u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [66u8, 10u8, 131u8, 231u8],
            [89u8, 198u8, 32u8, 198u8],
            [98u8, 19u8, 130u8, 29u8],
            [102u8, 217u8, 169u8, 160u8],
            [129u8, 168u8, 222u8, 126u8],
            [133u8, 34u8, 108u8, 129u8],
            [133u8, 164u8, 104u8, 247u8],
            [138u8, 84u8, 37u8, 33u8],
            [145u8, 106u8, 23u8, 198u8],
            [148u8, 162u8, 165u8, 223u8],
            [162u8, 23u8, 253u8, 223u8],
            [176u8, 70u8, 79u8, 220u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [191u8, 248u8, 96u8, 15u8],
            [226u8, 12u8, 159u8, 113u8],
            [229u8, 42u8, 47u8, 31u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(setUp),
            ::core::stringify!(test_ExecuteAirdropWithArrayLengthMismatch),
            ::core::stringify!(excludeSenders),
            ::core::stringify!(targetInterfaces),
            ::core::stringify!(airdrop),
            ::core::stringify!(targetSenders),
            ::core::stringify!(targetContracts),
            ::core::stringify!(tokenHolder),
            ::core::stringify!(test_TokenInformation),
            ::core::stringify!(syndToken),
            ::core::stringify!(targetArtifactSelectors),
            ::core::stringify!(test_DeployAirdrop),
            ::core::stringify!(targetArtifacts),
            ::core::stringify!(test_ExecuteAirdrop),
            ::core::stringify!(AIRDROP_MANAGER_ROLE),
            ::core::stringify!(targetSelectors),
            ::core::stringify!(test_ExecuteAirdropWithInsufficientAllowance),
            ::core::stringify!(DEFAULT_ADMIN_ROLE),
            ::core::stringify!(excludeSelectors),
            ::core::stringify!(excludeArtifacts),
            ::core::stringify!(failed),
            ::core::stringify!(test_ExecuteAirdropBatches),
            ::core::stringify!(excludeContracts),
            ::core::stringify!(syndTokenAccessControl),
            ::core::stringify!(IS_TEST),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <setUpCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_ExecuteAirdropWithArrayLengthMismatchCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetInterfacesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <airdropCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <tokenHolderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_TokenInformationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <syndTokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_DeployAirdropCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_ExecuteAirdropCall as alloy_sol_types::SolCall>::SIGNATURE,
            <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_ExecuteAirdropWithInsufficientAllowanceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <failedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_ExecuteAirdropBatchesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <syndTokenAccessControlCall as alloy_sol_types::SolCall>::SIGNATURE,
            <IS_TESTCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AirdropScriptTestCalls {
        const NAME: &'static str = "AirdropScriptTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AIRDROP_MANAGER_ROLE(_) => {
                    <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::airdrop(_) => <airdropCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::syndToken(_) => {
                    <syndTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::syndTokenAccessControl(_) => {
                    <syndTokenAccessControlCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::test_DeployAirdrop(_) => {
                    <test_DeployAirdropCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_ExecuteAirdrop(_) => {
                    <test_ExecuteAirdropCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_ExecuteAirdropBatches(_) => {
                    <test_ExecuteAirdropBatchesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_ExecuteAirdropWithArrayLengthMismatch(_) => {
                    <test_ExecuteAirdropWithArrayLengthMismatchCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_ExecuteAirdropWithInsufficientAllowance(_) => {
                    <test_ExecuteAirdropWithInsufficientAllowanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_TokenInformation(_) => {
                    <test_TokenInformationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::tokenHolder(_) => {
                    <tokenHolderCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<AirdropScriptTestCalls>] = &[
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(AirdropScriptTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn test_ExecuteAirdropWithArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <test_ExecuteAirdropWithArrayLengthMismatchCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                AirdropScriptTestCalls::test_ExecuteAirdropWithArrayLengthMismatch,
                            )
                    }
                    test_ExecuteAirdropWithArrayLengthMismatch
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn airdrop(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <airdropCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(AirdropScriptTestCalls::airdrop)
                    }
                    airdrop
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn tokenHolder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <tokenHolderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::tokenHolder)
                    }
                    tokenHolder
                },
                {
                    fn test_TokenInformation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <test_TokenInformationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::test_TokenInformation)
                    }
                    test_TokenInformation
                },
                {
                    fn syndToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <syndTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(AirdropScriptTestCalls::syndToken)
                    }
                    syndToken
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn test_DeployAirdrop(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <test_DeployAirdropCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::test_DeployAirdrop)
                    }
                    test_DeployAirdrop
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn test_ExecuteAirdrop(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <test_ExecuteAirdropCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::test_ExecuteAirdrop)
                    }
                    test_ExecuteAirdrop
                },
                {
                    fn AIRDROP_MANAGER_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::AIRDROP_MANAGER_ROLE)
                    }
                    AIRDROP_MANAGER_ROLE
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn test_ExecuteAirdropWithInsufficientAllowance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <test_ExecuteAirdropWithInsufficientAllowanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                AirdropScriptTestCalls::test_ExecuteAirdropWithInsufficientAllowance,
                            )
                    }
                    test_ExecuteAirdropWithInsufficientAllowance
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(AirdropScriptTestCalls::failed)
                    }
                    failed
                },
                {
                    fn test_ExecuteAirdropBatches(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <test_ExecuteAirdropBatchesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::test_ExecuteAirdropBatches)
                    }
                    test_ExecuteAirdropBatches
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn syndTokenAccessControl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <syndTokenAccessControlCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(AirdropScriptTestCalls::syndTokenAccessControl)
                    }
                    syndTokenAccessControl
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(AirdropScriptTestCalls::IS_TEST)
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
            ) -> alloy_sol_types::Result<AirdropScriptTestCalls>] = &[
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn test_ExecuteAirdropWithArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <test_ExecuteAirdropWithArrayLengthMismatchCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                AirdropScriptTestCalls::test_ExecuteAirdropWithArrayLengthMismatch,
                            )
                    }
                    test_ExecuteAirdropWithArrayLengthMismatch
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn airdrop(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <airdropCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::airdrop)
                    }
                    airdrop
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn tokenHolder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <tokenHolderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::tokenHolder)
                    }
                    tokenHolder
                },
                {
                    fn test_TokenInformation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <test_TokenInformationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::test_TokenInformation)
                    }
                    test_TokenInformation
                },
                {
                    fn syndToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <syndTokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::syndToken)
                    }
                    syndToken
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn test_DeployAirdrop(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <test_DeployAirdropCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::test_DeployAirdrop)
                    }
                    test_DeployAirdrop
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn test_ExecuteAirdrop(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <test_ExecuteAirdropCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::test_ExecuteAirdrop)
                    }
                    test_ExecuteAirdrop
                },
                {
                    fn AIRDROP_MANAGER_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::AIRDROP_MANAGER_ROLE)
                    }
                    AIRDROP_MANAGER_ROLE
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn test_ExecuteAirdropWithInsufficientAllowance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <test_ExecuteAirdropWithInsufficientAllowanceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                AirdropScriptTestCalls::test_ExecuteAirdropWithInsufficientAllowance,
                            )
                    }
                    test_ExecuteAirdropWithInsufficientAllowance
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::failed)
                    }
                    failed
                },
                {
                    fn test_ExecuteAirdropBatches(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <test_ExecuteAirdropBatchesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::test_ExecuteAirdropBatches)
                    }
                    test_ExecuteAirdropBatches
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn syndTokenAccessControl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <syndTokenAccessControlCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::syndTokenAccessControl)
                    }
                    syndTokenAccessControl
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<AirdropScriptTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(AirdropScriptTestCalls::IS_TEST)
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
                Self::AIRDROP_MANAGER_ROLE(inner) => {
                    <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::airdrop(inner) => {
                    <airdropCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::syndToken(inner) => {
                    <syndTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::syndTokenAccessControl(inner) => {
                    <syndTokenAccessControlCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::test_DeployAirdrop(inner) => {
                    <test_DeployAirdropCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_ExecuteAirdrop(inner) => {
                    <test_ExecuteAirdropCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_ExecuteAirdropBatches(inner) => {
                    <test_ExecuteAirdropBatchesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_ExecuteAirdropWithArrayLengthMismatch(inner) => {
                    <test_ExecuteAirdropWithArrayLengthMismatchCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_ExecuteAirdropWithInsufficientAllowance(inner) => {
                    <test_ExecuteAirdropWithInsufficientAllowanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_TokenInformation(inner) => {
                    <test_TokenInformationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::tokenHolder(inner) => {
                    <tokenHolderCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::airdrop(inner) => {
                    <airdropCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::syndToken(inner) => {
                    <syndTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::syndTokenAccessControl(inner) => {
                    <syndTokenAccessControlCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::test_DeployAirdrop(inner) => {
                    <test_DeployAirdropCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_ExecuteAirdrop(inner) => {
                    <test_ExecuteAirdropCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_ExecuteAirdropBatches(inner) => {
                    <test_ExecuteAirdropBatchesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_ExecuteAirdropWithArrayLengthMismatch(inner) => {
                    <test_ExecuteAirdropWithArrayLengthMismatchCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_ExecuteAirdropWithInsufficientAllowance(inner) => {
                    <test_ExecuteAirdropWithInsufficientAllowanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_TokenInformation(inner) => {
                    <test_TokenInformationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::tokenHolder(inner) => {
                    <tokenHolderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`AirdropScriptTest`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum AirdropScriptTestEvents {
        #[allow(missing_docs)]
        SlotFound(SlotFound),
        #[allow(missing_docs)]
        WARNING_UninitedSlot(WARNING_UninitedSlot),
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
    impl AirdropScriptTestEvents {
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
                8u8, 15u8, 196u8, 169u8, 102u8, 32u8, 196u8, 70u8, 46u8, 112u8, 91u8,
                35u8, 243u8, 70u8, 65u8, 63u8, 227u8, 121u8, 107u8, 182u8, 60u8, 111u8,
                141u8, 133u8, 145u8, 186u8, 236u8, 14u8, 35u8, 21u8, 119u8, 165u8,
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
                156u8, 149u8, 85u8, 177u8, 227u8, 16u8, 46u8, 60u8, 244u8, 143u8, 66u8,
                125u8, 121u8, 203u8, 103u8, 143u8, 93u8, 155u8, 209u8, 237u8, 10u8,
                213u8, 116u8, 56u8, 148u8, 97u8, 226u8, 85u8, 249u8, 81u8, 112u8, 237u8,
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
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(log_named_array_0),
            ::core::stringify!(WARNING_UninitedSlot),
            ::core::stringify!(log_string),
            ::core::stringify!(log_int),
            ::core::stringify!(log_bytes),
            ::core::stringify!(log_named_string),
            ::core::stringify!(log_uint),
            ::core::stringify!(log_named_int),
            ::core::stringify!(log_named_array_2),
            ::core::stringify!(log_array_2),
            ::core::stringify!(log),
            ::core::stringify!(log_named_decimal_int),
            ::core::stringify!(log_address),
            ::core::stringify!(log_array_1),
            ::core::stringify!(log_named_address),
            ::core::stringify!(SlotFound),
            ::core::stringify!(log_named_array_1),
            ::core::stringify!(log_named_bytes32),
            ::core::stringify!(log_named_uint),
            ::core::stringify!(log_named_bytes),
            ::core::stringify!(logs),
            ::core::stringify!(log_bytes32),
            ::core::stringify!(log_named_decimal_uint),
            ::core::stringify!(log_array_0),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE,
            <WARNING_UninitedSlot as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_string as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_bytes as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_string as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_address as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE,
            <SlotFound as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE,
            <logs as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for AirdropScriptTestEvents {
        const NAME: &'static str = "AirdropScriptTestEvents";
        const COUNT: usize = 24usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<SlotFound as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SlotFound as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SlotFound)
                }
                Some(
                    <WARNING_UninitedSlot as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <WARNING_UninitedSlot as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::WARNING_UninitedSlot)
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
    impl alloy_sol_types::private::IntoLogData for AirdropScriptTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::SlotFound(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::WARNING_UninitedSlot(inner) => {
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
                Self::SlotFound(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WARNING_UninitedSlot(inner) => {
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
    /**Creates a new wrapper around an on-chain [`AirdropScriptTest`](self) contract instance.

See the [wrapper's documentation](`AirdropScriptTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> AirdropScriptTestInstance<P, N> {
        AirdropScriptTestInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<AirdropScriptTestInstance<P, N>>,
    > {
        AirdropScriptTestInstance::<P, N>::deploy(__provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        AirdropScriptTestInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`AirdropScriptTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`AirdropScriptTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct AirdropScriptTestInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for AirdropScriptTestInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("AirdropScriptTestInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > AirdropScriptTestInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`AirdropScriptTest`](self) contract instance.

See the [wrapper's documentation](`AirdropScriptTestInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
        ) -> alloy_contract::Result<AirdropScriptTestInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
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
    impl<P: ::core::clone::Clone, N> AirdropScriptTestInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> AirdropScriptTestInstance<P, N> {
            AirdropScriptTestInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > AirdropScriptTestInstance<P, N> {
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
        ///Creates a new call builder for the [`AIRDROP_MANAGER_ROLE`] function.
        pub fn AIRDROP_MANAGER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, AIRDROP_MANAGER_ROLECall, N> {
            self.call_builder(&AIRDROP_MANAGER_ROLECall)
        }
        ///Creates a new call builder for the [`DEFAULT_ADMIN_ROLE`] function.
        pub fn DEFAULT_ADMIN_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DEFAULT_ADMIN_ROLECall, N> {
            self.call_builder(&DEFAULT_ADMIN_ROLECall)
        }
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<&P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall)
        }
        ///Creates a new call builder for the [`airdrop`] function.
        pub fn airdrop(&self) -> alloy_contract::SolCallBuilder<&P, airdropCall, N> {
            self.call_builder(&airdropCall)
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
        ///Creates a new call builder for the [`syndToken`] function.
        pub fn syndToken(&self) -> alloy_contract::SolCallBuilder<&P, syndTokenCall, N> {
            self.call_builder(&syndTokenCall)
        }
        ///Creates a new call builder for the [`syndTokenAccessControl`] function.
        pub fn syndTokenAccessControl(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, syndTokenAccessControlCall, N> {
            self.call_builder(&syndTokenAccessControlCall)
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
        ///Creates a new call builder for the [`test_DeployAirdrop`] function.
        pub fn test_DeployAirdrop(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_DeployAirdropCall, N> {
            self.call_builder(&test_DeployAirdropCall)
        }
        ///Creates a new call builder for the [`test_ExecuteAirdrop`] function.
        pub fn test_ExecuteAirdrop(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_ExecuteAirdropCall, N> {
            self.call_builder(&test_ExecuteAirdropCall)
        }
        ///Creates a new call builder for the [`test_ExecuteAirdropBatches`] function.
        pub fn test_ExecuteAirdropBatches(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_ExecuteAirdropBatchesCall, N> {
            self.call_builder(&test_ExecuteAirdropBatchesCall)
        }
        ///Creates a new call builder for the [`test_ExecuteAirdropWithArrayLengthMismatch`] function.
        pub fn test_ExecuteAirdropWithArrayLengthMismatch(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_ExecuteAirdropWithArrayLengthMismatchCall,
            N,
        > {
            self.call_builder(&test_ExecuteAirdropWithArrayLengthMismatchCall)
        }
        ///Creates a new call builder for the [`test_ExecuteAirdropWithInsufficientAllowance`] function.
        pub fn test_ExecuteAirdropWithInsufficientAllowance(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_ExecuteAirdropWithInsufficientAllowanceCall,
            N,
        > {
            self.call_builder(&test_ExecuteAirdropWithInsufficientAllowanceCall)
        }
        ///Creates a new call builder for the [`test_TokenInformation`] function.
        pub fn test_TokenInformation(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_TokenInformationCall, N> {
            self.call_builder(&test_TokenInformationCall)
        }
        ///Creates a new call builder for the [`tokenHolder`] function.
        pub fn tokenHolder(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, tokenHolderCall, N> {
            self.call_builder(&tokenHolderCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > AirdropScriptTestInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`SlotFound`] event.
        pub fn SlotFound_filter(&self) -> alloy_contract::Event<&P, SlotFound, N> {
            self.event_filter::<SlotFound>()
        }
        ///Creates a new event filter for the [`WARNING_UninitedSlot`] event.
        pub fn WARNING_UninitedSlot_filter(
            &self,
        ) -> alloy_contract::Event<&P, WARNING_UninitedSlot, N> {
            self.event_filter::<WARNING_UninitedSlot>()
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
