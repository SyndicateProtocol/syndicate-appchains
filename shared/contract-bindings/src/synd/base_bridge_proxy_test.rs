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

interface BaseBridgeProxyTest {
    event BridgeExecuted(address indexed token, uint256 amount, address indexed target);
    event BridgeStatusUpdated(bool active);
    event BridgeTargetUpdated(address indexed oldTarget, address indexed newTarget);
    event DailyLimitReset(uint256 day, uint256 previousUsed);
    event DailyLimitUpdated(uint256 oldLimit, uint256 newLimit);
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

    function DAILY_LIMIT() external view returns (uint256);
    function IS_TEST() external view returns (bool);
    function MAX_SINGLE_TRANSFER() external view returns (uint256);
    function admin() external view returns (address);
    function bridgeProxy() external view returns (address);
    function caller() external view returns (address);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function newTarget() external view returns (address);
    function setUp() external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetBridge() external view returns (address);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testFuzz_ExecuteBridge_ValidAmounts(uint256 amount) external;
    function testFuzz_SetDailyLimit_ValidValues(uint256 newLimit) external;
    function test_Constructor_RoleAssignment() external view;
    function test_Constructor_Success() external view;
    function test_DailyLimit_Cumulative() external;
    function test_DailyLimit_Reset() external;
    function test_ExecuteBridge_MultipleTransfers() external;
    function test_ExecuteBridge_ReentrancyProtection() external;
    function test_ExecuteBridge_Success() external;
    function test_GetBridgeInfo() external view;
    function test_GetDailyUsage_AfterTransfer() external;
    function test_GetDailyUsage_Initial() external view;
    function test_RecoverTokens_Success() external;
    function test_RevertWhen_Constructor_ZeroAdmin() external;
    function test_RevertWhen_Constructor_ZeroCaller() external;
    function test_RevertWhen_ExecuteBridge_BridgeInactive() external;
    function test_RevertWhen_ExecuteBridge_ExceedsDailyLimit() external;
    function test_RevertWhen_ExecuteBridge_ExceedsMaxSingle() external;
    function test_RevertWhen_ExecuteBridge_UnauthorizedCaller() external;
    function test_RevertWhen_ExecuteBridge_ZeroAmount() external;
    function test_RevertWhen_ExecuteBridge_ZeroToken() external;
    function test_RevertWhen_RecoverTokens_NotAdmin() external;
    function test_RevertWhen_RecoverTokens_ZeroAddress() external;
    function test_RevertWhen_SetBridgeTarget_NotAdmin() external;
    function test_RevertWhen_SetBridgeTarget_ZeroAddress() external;
    function test_SetBridgeActive_Success() external;
    function test_SetBridgeTarget_Success() external;
    function test_SetDailyLimit_Success() external;
    function test_SetMaxSingleTransfer_Success() external;
    function token() external view returns (address);
    function user() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "DAILY_LIMIT",
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
    "name": "MAX_SINGLE_TRANSFER",
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
    "name": "bridgeProxy",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockBridgeProxy"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "caller",
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
    "name": "newTarget",
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
    "name": "targetBridge",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockTargetBridge"
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
    "name": "testFuzz_ExecuteBridge_ValidAmounts",
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
    "name": "testFuzz_SetDailyLimit_ValidValues",
    "inputs": [
      {
        "name": "newLimit",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_Constructor_RoleAssignment",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "test_Constructor_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "test_DailyLimit_Cumulative",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_DailyLimit_Reset",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_ExecuteBridge_MultipleTransfers",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_ExecuteBridge_ReentrancyProtection",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_ExecuteBridge_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_GetBridgeInfo",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "test_GetDailyUsage_AfterTransfer",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_GetDailyUsage_Initial",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "test_RecoverTokens_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_Constructor_ZeroAdmin",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_Constructor_ZeroCaller",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_ExecuteBridge_BridgeInactive",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_ExecuteBridge_ExceedsDailyLimit",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_ExecuteBridge_ExceedsMaxSingle",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_ExecuteBridge_UnauthorizedCaller",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_ExecuteBridge_ZeroAmount",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_ExecuteBridge_ZeroToken",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_RecoverTokens_NotAdmin",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_RecoverTokens_ZeroAddress",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_SetBridgeTarget_NotAdmin",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_SetBridgeTarget_ZeroAddress",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_SetBridgeActive_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_SetBridgeTarget_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_SetDailyLimit_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_SetMaxSingleTransfer_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "token",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ERC20Mock"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "user",
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
    "name": "BridgeExecuted",
    "inputs": [
      {
        "name": "token",
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
        "name": "target",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BridgeStatusUpdated",
    "inputs": [
      {
        "name": "active",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BridgeTargetUpdated",
    "inputs": [
      {
        "name": "oldTarget",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newTarget",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DailyLimitReset",
    "inputs": [
      {
        "name": "day",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "previousUsed",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DailyLimitUpdated",
    "inputs": [
      {
        "name": "oldLimit",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newLimit",
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
pub mod BaseBridgeProxyTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60808060405234606f57600c805460ff199081166001908117909255601f80549091169091179055602280546001600160a01b031990811661123417909155602380548216615678179055602480548216619abc1790556025805490911661def017905561959890816100748239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f905f3560e01c908163089ca9e014615ce157508063094f27a014615ada5780630a9254e4146158675780630f57280f146152745780631074a21f1461513d578063117e3b421461511857806313217f90146150f157806313a86f1a14614e065780631ed7831c14614d8857806323e1ebe714614ac7578063248ec32614614aa157806329365968146148a75780632ade3880146147225780632ed211831461457c5780633e5e3c23146144fe5780633f7286f414614480578063497b39181461418f5780634a61cf2914613e155780634f8632ba14613dee57806366d9a9a014613cb15780636f8cece414613b2657806385226c8114613a945780638b58cbae146137785780638f58a63f146136c2578063916a17c614613618578063956d980814613523578063959b337d146134fc578063a30ff4c214613242578063a3d4485b14613218578063a3fb171514612f60578063b0464fdc14612eb6578063b44dc9d614612b06578063b5508aa914612a74578063b55d42bc1461274c578063b9b5bd681461209e578063ba414fa614612079578063be6da53e14611ed6578063cffb048b14611ad9578063d3075c49146114e6578063d3b76bc914611376578063db9b708c146111ef578063dccc57f114610e8f578063e20c9f7114610e01578063e86b4fa714610ad5578063e9d3d5861461092f578063f3ed2b05146106b5578063f851a4401461068e578063f97e8467146102c1578063fa7626d41461029e578063fc0c546a146102785763fc9c8d391461024f575f80fd5b3461027557806003193601126102755760206001600160a01b0360235416604051908152f35b80fd5b503461027557806003193601126102755760206001600160a01b03815416604051908152f35b5034610275578060031936011261027557602060ff601f54166040519015158152f35b5034610275578060031936011261027557806001600160a01b03602054166001600160a01b03601f5460081c16813b1561068a5782916044839260405194859384927f40c10f19000000000000000000000000000000000000000000000000000000008452600484015269152d02c7e14af680000060248401525af1801561059757610675575b506024906001600160a01b036020541660206001600160a01b036022541691604051948580927f70a082310000000000000000000000000000000000000000000000000000000082528560048301525afa92831561059757829361063e575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610621576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610629575b506001600160a01b03601f5460081c166001600160a01b03602054166001600160a01b0360225416823b1561062557606484928360405195869485937f61b0a56e000000000000000000000000000000000000000000000000000000008552600485015269152d02c7e14af6800000602485015260448401525af180156105975761060c575b50506001600160a01b0360205416906001600160a01b0360225416604051907f70a082310000000000000000000000000000000000000000000000000000000082526004820152602081602481865afa9081156106015784916105cf575b5069152d02c7e14af680000082018092116105a25790610501916169f6565b60206001600160a01b03601f5460081c166024604051809481937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa801561059757829061055f575b61055c9150616980565b80f35b506020813d60201161058f575b8161057960209383616219565b8101031261058b5761055c9051610552565b5f80fd5b3d915061056c565b6040513d84823e3d90fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d6020116105f9575b816105ea60209383616219565b8101031261058b57515f6104e2565b3d91506105dd565b6040513d86823e3d90fd5b8161061691616219565b61062157815f610484565b5080fd5b8380fd5b8161063391616219565b61062157815f6103fe565b915091506020813d60201161066d575b8161065b60209383616219565b8101031261058b57829051915f6103a7565b3d915061064e565b8161067f91616219565b61027557805f610348565b5050fd5b503461027557806003193601126102755760206001600160a01b0360225416604051908152f35b5034610275578060031936011261027557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152600160048201526001602482015281604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761091a575b506001600160a01b03602154166001600160a01b0360255416604051917fb07f8b1b85042d74022c867c836edeb0bcd70e135b0042390d2b1fd1082980698480a36001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561068a5763ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610905575b506001600160a01b03601f5460081c166001600160a01b0360255416813b1561068a5782916024839260405194859384927f6bcc8c1400000000000000000000000000000000000000000000000000000000845260048401525af18015610597576108f0575b5050600460206001600160a01b03601f5460081c16604051928380927fc9f5b63e0000000000000000000000000000000000000000000000000000000082525afa80156105975782906108b5575b61055c91506001600160a01b036025541690616af1565b506020813d6020116108e8575b816108cf60209383616219565b81010312610621576108e361055c916166cd565b61089e565b3d91506108c2565b816108fa91616219565b61027557805f610850565b8161090f91616219565b61027557805f6107ea565b8161092491616219565b61027557805f61074b565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610abd575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f1f2a2005000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610aa8575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b84526004840152816024840152606060448401528160648401525af1801561059757610a975750f35b81610aa191616219565b6102755780f35b81610ab291616219565b61027557805f610a35565b81610ac791616219565b61027557805f6109a3565b50fd5b5034610275578060031936011261027557806001600160a01b0360245416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610dec575b50600460206001600160a01b03601f5460081c16604051928380927fa217fddf0000000000000000000000000000000000000000000000000000000082525afa908115610597578291610db7575b50610c1c6001600160a01b0360245416610c0e6040519384927fe2517d3f00000000000000000000000000000000000000000000000000000000602085015260248401602090939291936001600160a01b0360408201951681520152565b03601f198101835282616219565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad25781610c7791604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190616067565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610da2575b506001600160a01b03601f5460081c166001600160a01b03602054166001600160a01b0360225416823b15610d9d57606484928360405195869485937f61b0a56e00000000000000000000000000000000000000000000000000000000855260048501526103e8602485015260448401525af1801561059757610d88575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610a975750f35b81610d9291616219565b61027557805f610d1a565b505050fd5b81610dac91616219565b61027557805f610c9c565b9150506020813d602011610de4575b81610dd360209383616219565b8101031261058b578190515f610bb0565b3d9150610dc6565b81610df691616219565b61027557805f610b62565b503461027557806003193601126102755760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b818110610e7057610e6c85610e6081870382616219565b60405191829182616025565b0390f35b82546001600160a01b0316845260209093019260019283019201610e49565b50346102755780600319360112610275576001600160a01b03601f5460081c166040517fa217fddf000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156111045783916111bd575b506022546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b03166024820181905290602081604481865afa8015610601578490611182575b610f4f9150616b73565b6040517f118c38c7000000000000000000000000000000000000000000000000000000008152602081600481865afa90811561060157849161114e575b506040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101919091526001600160a01b03909116602482015260208180604481015b0381855afa801561110457839061110f575b610ff09150616b73565b6040517f3462fac3000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156111045783916110d0575b506023546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529060209082908180604481015b03915afa8015610597578290611095575b61055c9150616b73565b506020813d6020116110c8575b816110af60209383616219565b81010312610621576110c361055c9161623c565b61108b565b3d91506110a2565b90506020813d6020116110fc575b816110eb60209383616219565b8101031261058b575161107a61102d565b3d91506110de565b6040513d85823e3d90fd5b506020813d602011611146575b8161112960209383616219565b810103126111425761113d610ff09161623c565b610fe6565b8280fd5b3d915061111c565b90506020813d60201161117a575b8161116960209383616219565b8101031261058b5751610fd4610f8c565b3d915061115c565b506020813d6020116111b5575b8161119c60209383616219565b81010312610625576111b0610f4f9161623c565b610f45565b3d915061118f565b90506020813d6020116111e7575b816111d860209383616219565b8101031261058b57515f610eec565b3d91506111cb565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757611361575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761134c575b506001600160a01b03601f5460081c16803b15610ad25781809160846040518094819363062da2e360e21b83528160048401526103e86024840152606060448401528160648401525af1801561059757610a975750f35b8161135691616219565b61027557805f6112f5565b8161136b91616219565b61027557805f611263565b5034610275578060031936011261027557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576114d1575b50506001600160a01b03602254166001600160a01b03602154169060405191611999918284019284841067ffffffffffffffff8511176114a4579160a093918593617bff85398252856020830152604082015269d3c21bcecceda100000060608201526a0422ca8b0a00a425000000608082015203019082f0156114985780f35b604051903d90823e3d90fd5b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b816114db91616219565b61027557805f611417565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757611ac4575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526a01a784379d99db4200000060248401525af1801561059757611a8d575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757611a78575b506001600160a01b03601f5460081c166001600160a01b036020541690803b1561068a5760405163062da2e360e21b81526001600160a01b0392909216600483015269d3c21bcecceda10000006024830152606060448301525f606483015282908290608490829084905af1801561059757611a63575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa8015610597578290611a2f575b6116fa91506167f5565b620151804201804211611a02578190737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907fe5d6bf020000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576119ed575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f491cc7c20000000000000000000000000000000000000000000000000000000081528181806117e360048201906001606060808401935f81525f60208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576119d8575b50507fda4e39dd56d72c2ee3d132e0146bc39e905e78e3bc64c40190421c7b2bcef2ab60408051620151804204815269d3c21bcecceda10000006020820152a1806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576119c3575b506001600160a01b03601f5460081c166001600160a01b036020541690803b1561068a5760405163062da2e360e21b81526001600160a01b0392909216600483015269d3c21bcecceda10000006024830152606060448301525f606483015282908290608490829084905af18015610597576119ae575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa801561059757829061197a575b61055c91506167f5565b506020813d6020116119a6575b8161199460209383616219565b8101031261058b5761055c9051611970565b3d9150611987565b816119b891616219565b61027557805f611922565b816119cd91616219565b61027557805f6118ab565b816119e291616219565b61027557805f611808565b816119f791616219565b61027557805f611778565b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b506020813d602011611a5b575b81611a4960209383616219565b8101031261058b576116fa90516116f0565b3d9150611a3c565b81611a6d91616219565b61027557805f6116a2565b81611a8291616219565b61027557805f61162b565b6020813d602011611abc575b81611aa660209383616219565b8101031261062157611ab79061623c565b6115c7565b3d9150611a99565b81611ace91616219565b61027557805f61155a565b503461027557602060031936011261027557611b0369d3c21bcecceda10000006001600435616c1c565b60409082808351611b148582616219565b600c81527f426f756e6420726573756c74000000000000000000000000000000000000000060208201528451611b9381611b7f60208201947fb60e72cc0000000000000000000000000000000000000000000000000000000086528960248401526064830190616067565b87604483015203601f198101835282616219565b51906a636f6e736f6c652e6c6f675afa50826001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156106215783519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015611e5e57611ec1575b505060208054601f5484517f095ea7b30000000000000000000000000000000000000000000000000000000081526001600160a01b0360089290921c821660048201526024810185905292918391168187816044810103925af18015611eb457611e7d575b50826001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156106215783519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015611e5e57611e68575b506001600160a01b03601f5460081c166001600160a01b0360205416813b15611142578291608483928751948593849263062da2e360e21b84526004840152886024840152606060448401528160648401525af18015611e5e57611e49575b50506001600160a01b03601f5460081c169180517ff681a862000000000000000000000000000000000000000000000000000000008152602081600481875afa908115611e3f578591611e0c575b50600493611d8b846020936169f6565b8251948580927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa908115611e0357508390611dcf575b61055c92506169f6565b506020823d602011611dfb575b81611de960209383616219565b8101031261058b5761055c9151611dc5565b3d9150611ddc565b513d85823e3d90fd5b90506020813d602011611e37575b81611e2760209383616219565b8101031261058b57516004611d7b565b3d9150611e1a565b82513d87823e3d90fd5b81611e5391616219565b61114257825f611d2d565b84513d84823e3d90fd5b81611e7291616219565b61114257825f611cce565b6020813d602011611eac575b81611e9660209383616219565b8101031261062557611ea79061623c565b611c6b565b3d9150611e89565b50505051903d90823e3d90fd5b81611ecb91616219565b61114257825f611c06565b5034610275578060031936011261027557806001600160a01b0360245416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757612064575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f5c427cd9000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761204f575b506001600160a01b03601f5460081c166001600160a01b036020541690803b1561068a5760405163062da2e360e21b81526001600160a01b0390921660048301526103e86024830152606060448301525f606483015282908290818381608481015b03925af1801561059757610a975750f35b8161205991616219565b61027557805f611fdc565b8161206e91616219565b61027557805f611f4a565b5034610275578060031936011261027557602061209461671c565b6040519015158152f35b5034610275578060031936011261027557604051602080820152600960408201527f74657374206461746100000000000000000000000000000000000000000000006060820152606081526120f4608082616219565b816001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610621576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757612737575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526969e10de76676d080000060248401525af1801561110457612700575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561062157816040517f491cc7c2000000000000000000000000000000000000000000000000000000008152600160048201528160248201526001604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576126eb575b50506001600160a01b03602054166001600160a01b0360215416907f3dba401dc1abbf017086864b33ba815f53ab3cdbb9366ba6bcd8ec342ddd98e860206040516969e10de76676d08000008152a3816001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610621576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576126d6575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561114257829060405192839163062da2e360e21b835260048301526969e10de76676d080000060248301526060604483015281838161235c606482018a616067565b03925af18015610597576126c1575b50506001600160a01b03601f5460081c166040517f1033b4cc000000000000000000000000000000000000000000000000000000008152602081600481855afa801561060157849061268d575b6123c29150616900565b6040517ff681a862000000000000000000000000000000000000000000000000000000008152602081600481855afa8015610601578490612659575b6124089150616900565b82604051927fb16e7849000000000000000000000000000000000000000000000000000000008452602084600481865afa93841561059757829461261d575b5061245e6001600160a01b03602054168095616af1565b6040517fcc3dc0610000000000000000000000000000000000000000000000000000000081528281600481875afa9081156111045783916125c8575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611142576124fa839161250c60405194859384937f97624631000000000000000000000000000000000000000000000000000000008552604060048601526044850190616067565b90600319848303016024850152616067565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610597576125b3575b50506020906024604051809481937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa801561059757829061257f575b61055c9150616900565b506020813d6020116125ab575b8161259960209383616219565b8101031261058b5761055c9051612575565b3d915061258c565b816125bd91616219565b61114257825f612530565b90503d8084833e6125d98183616219565b8101906020818303126106255780519067ffffffffffffffff821161261957019080601f830112156106255781516126139260200161666a565b5f61249a565b8480fd5b9093506020813d602011612651575b8161263960209383616219565b810103126106215761264a906166cd565b925f612447565b3d915061262c565b506020813d602011612685575b8161267360209383616219565b8101031261058b5761240890516123fe565b3d9150612666565b506020813d6020116126b9575b816126a760209383616219565b8101031261058b576123c290516123b8565b3d915061269a565b816126cb91616219565b61062157815f61236b565b816126e091616219565b61062157815f6122fb565b816126f591616219565b61062157815f612249565b6020813d60201161272f575b8161271960209383616219565b810103126111425761272a9061623c565b6121c3565b3d915061270c565b8161274191616219565b61062157815f612157565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757612a5f575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b3000000000000000000000000000000000000000000000000000000008452600484015269152d02c7e14af680000060248401525af1801561059757612a28575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757612a13575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b8452600484015269152d02c7e14af68000006024840152606060448401528160648401525af18015610597576129fe575b50600460206001600160a01b03601f5460081c16604051928380927ff681a8620000000000000000000000000000000000000000000000000000000082525afa9081156105975782916129c9575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f98296c54000000000000000000000000000000000000000000000000000000008252600482015269152d02c7e14af680000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561059757610a975750f35b9150506020813d6020116129f6575b816129e560209383616219565b8101031261058b578190515f612948565b3d91506129d8565b81612a0891616219565b61027557805f6128fa565b81612a1d91616219565b61027557805f612890565b6020813d602011612a57575b81612a4160209383616219565b8101031261062157612a529061623c565b61282c565b3d9150612a34565b81612a6991616219565b61027557805f6127c0565b5034610275578060031936011261027557601954612a918161629e565b91612a9f6040519384616219565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b838310612ae95760405160208082528190610e6c9082018861608c565b600160208192612af8856162b6565b815201920192019190612acc565b50346102755780600319360112610275576001600160a01b03601f5460081c166040517fc9f5b63e000000000000000000000000000000000000000000000000000000008152602081600481855afa8015611104578390612e7b575b612b7991506001600160a01b036021541690616af1565b6040517f65d7a3c90000000000000000000000000000000000000000000000000000000081528281600481855afa8015611104578390612e39575b612bc79150612bc16166e1565b90616a6c565b6040517f36b089d8000000000000000000000000000000000000000000000000000000008152602081600481855afa8015611104578390612e05575b612c0d91506167f5565b6040517f67eeba0c000000000000000000000000000000000000000000000000000000008152602081600481855afa8015611104578390612dd1575b612c53915061687f565b6040517fead93c8f000000000000000000000000000000000000000000000000000000008152602081600481855afa8015611104578390612d96575b612c999150616b73565b6040517f1033b4cc000000000000000000000000000000000000000000000000000000008152602081600481855afa908115611104578391612d63575b50600491612ce5602092616980565b604051928380927f1259a5c80000000000000000000000000000000000000000000000000000000082525afa8015610597578290612d2f575b61055c9150620151804204906169f6565b506020813d602011612d5b575b81612d4960209383616219565b8101031261058b5761055c9051612d1e565b3d9150612d3c565b90506020813d602011612d8e575b81612d7e60209383616219565b8101031261058b57516004612cd6565b3d9150612d71565b506020813d602011612dc9575b81612db060209383616219565b8101031261114257612dc4612c999161623c565b612c8f565b3d9150612da3565b506020813d602011612dfd575b81612deb60209383616219565b8101031261058b57612c539051612c49565b3d9150612dde565b506020813d602011612e31575b81612e1f60209383616219565b8101031261058b57612c0d9051612c03565b3d9150612e12565b503d8084833e612e498183616219565b8101906020818303126106255780519167ffffffffffffffff831161261957612bc792612e7692016166b0565b612bb4565b506020813d602011612eae575b81612e9560209383616219565b8101031261114257612ea9612b79916166cd565b612b62565b3d9150612e88565b5034610275578060031936011261027557601c54612ed38161629e565b91612ee16040519384616219565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310612f235760405180610e6c8782616139565b60026020600192604051612f36816161d0565b6001600160a01b038654168152612f4e8587016163b9565b83820152815201920192019190612f0e565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757613203575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b3000000000000000000000000000000000000000000000000000000008452600484015269d3c21bcecceda100000160248401525af18015610597576131cc575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576131b7575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f70d168bc000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576131a2575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b8452600484015269d3c21bcecceda10000016024840152606060448401528160648401525af1801561059757610a975750f35b816131ac91616219565b61027557805f613136565b816131c191616219565b61027557805f6130a4565b6020813d6020116131fb575b816131e560209383616219565b81010312610621576131f69061623c565b613040565b3d91506131d8565b8161320d91616219565b61027557805f612fd4565b503461027557806003193601126102755760206001600160a01b03601f5460081c16604051908152f35b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576134e7575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526a0422ca8b0a00a42500000160248401525af18015610597576134b0575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761349b575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f70d168bc000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757613486575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b845260048401526a0422ca8b0a00a4250000016024840152606060448401528160648401525af1801561059757610a975750f35b8161349091616219565b61027557805f613419565b816134a591616219565b61027557805f613387565b6020813d6020116134df575b816134c960209383616219565b81010312610621576134da9061623c565b613323565b3d91506134bc565b816134f191616219565b61027557805f6132b6565b503461027557806003193601126102755760206001600160a01b0360215416604051908152f35b50346102755780600319360112610275576004816001600160a01b03601f5460081c16604051928380927fede7cebd0000000000000000000000000000000000000000000000000000000082525afa80156105975782839284926135ad575b5061055c926135966135a892612bc16166e1565b6001600160a01b036021541690616af1565b616b73565b925050503d8083833e6135c08183616219565b8101906060818303126111425780519167ffffffffffffffff8311610625576135f16135a89161055c9484016166b0565b61359661360c6040613605602087016166cd565b950161623c565b93949192506135829050565b5034610275578060031936011261027557601d546136358161629e565b916136436040519384616219565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b8383106136855760405180610e6c8782616139565b60026020600192604051613698816161d0565b6001600160a01b0386541681526136b08587016163b9565b83820152815201920192019190613670565b5034610275578060031936011261027557600460606001600160a01b03601f5460081c16604051928380927ffb8c4b510000000000000000000000000000000000000000000000000000000082525afa9081156105975761055c91838490859261373b575b61373692935061373690616980565b61687f565b5050506137366137656137369260603d606011613771575b61375d8183616219565b810190616249565b91935090915082613727565b503d613753565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757613a7f575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526969e10de76676d080000060248401525af1801561059757613a48575b50806001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b84526004840152693f870857a3e0e38000006024840152606060448401528160648401525af1801561059757613a33575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b84526004840152692a5a058fc295ed0000006024840152606060448401528160648401525af1801561059757613a1e575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757613a09575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa801561059757829061257f5761055c9150616900565b81613a1391616219565b61027557805f6139b2565b81613a2891616219565b61027557805f613946565b81613a3d91616219565b61027557805f6138dc565b6020813d602011613a77575b81613a6160209383616219565b8101031261062157613a729061623c565b613871565b3d9150613a54565b81613a8991616219565b61027557805f613805565b5034610275578060031936011261027557601a54613ab18161629e565b91613abf6040519384616219565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b838310613b095760405160208082528190610e6c9082018861608c565b600160208192613b18856162b6565b815201920192019190613aec565b5034610275578060031936011261027557806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757613c9c575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757613c87575b506001600160a01b03601f5460081c16803b15610ad2578180916024604051809481937f6bcc8c140000000000000000000000000000000000000000000000000000000083528160048401525af1801561059757610a975750f35b81613c9191616219565b61027557805f613c2c565b81613ca691616219565b61027557805f613b9a565b5034610275578060031936011261027557601b54613cce8161629e565b613cdb6040519182616219565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b838310613db357868587604051928392602084019060208552518091526040840160408260051b8601019392905b828210613d4857505050500390f35b91936020613da3827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0600195979984950301865288519083613d938351604084526040840190616067565b92015190848184039101526160e4565b9601920192018594939192613d39565b60026020600192604051613dc6816161d0565b613dcf866162b6565b8152613ddc8587016163b9565b83820152815201920192019190613d0b565b503461027557806003193601126102755760206001600160a01b0360245416604051908152f35b5034610275578060031936011261027557806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761417a575b506001600160a01b03601f5460081c16803b15610ad2578180916024604051809481937f5ab1d61c0000000000000000000000000000000000000000000000000000000083528160048401525af1801561059757614165575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757614150575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526103e860248401525af1801561059757614119575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757614104575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f7bea20b2000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761204f57506001600160a01b03601f5460081c166001600160a01b036020541690803b1561068a5760405163062da2e360e21b81526001600160a01b0390921660048301526103e86024830152606060448301525f6064830152829082908183816084810161203e565b8161410e91616219565b61027557805f61400d565b6020813d602011614148575b8161413260209383616219565b81010312610621576141439061623c565b613fa9565b3d9150614125565b8161415a91616219565b61027557805f613f45565b8161416f91616219565b61027557805f613ee2565b8161418491616219565b61027557805f613e89565b5034610275578060031936011261027557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f491cc7c200000000000000000000000000000000000000000000000000000000815281818061420960048201906001606060808401935f81525f60208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761446b575b50507f207c4cbdf55ec315a13f0d5e047732ec5d947da056e706593aa509909941cedf604080516a0422ca8b0a00a42500000081526a084595161401484a0000006020820152a1806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757614456575b506001600160a01b03601f5460081c16803b15610ad2578180916024604051809481937fb20d30a90000000000000000000000000000000000000000000000000000000083526a084595161401484a00000060048401525af1801561059757614441575b50600460206001600160a01b03601f5460081c16604051928380927f67eeba0c0000000000000000000000000000000000000000000000000000000082525afa90811561059757829161440c575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526a084595161401484a00000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561059757610a975750f35b9150506020813d602011614439575b8161442860209383616219565b8101031261058b578190515f61438a565b3d915061441b565b8161444b91616219565b61027557805f61433c565b8161446091616219565b61027557805f6142d8565b8161447591616219565b61027557805f61422e565b503461027557806003193601126102755760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b8181106144df57610e6c85610e6081870382616219565b82546001600160a01b03168452602090930192600192830192016144c8565b503461027557806003193601126102755760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b81811061455d57610e6c85610e6081870382616219565b82546001600160a01b0316845260209093019260019283019201614546565b5034610275578060031936011261027557806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761470d575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576146f8575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a5782916064839260405194859384927f61b0a56e00000000000000000000000000000000000000000000000000000000845260048401526103e860248401528160448401525af1801561059757610a975750f35b8161470291616219565b61027557805f614682565b8161471791616219565b61027557805f6145f0565b5034610275578060031936011261027557601e5461473f8161629e565b61474c6040519182616219565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b83831061481e57868587604051928392602084019060208552518091526040840160408260051b8601019392905b8282106147b957505050500390f35b9193602061480e827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc060019597998495030186526040838a516001600160a01b0381511684520151918185820152019061608c565b96019201920185949391926147aa565b60405161482a816161d0565b6001600160a01b0383541681526001830180546148468161629e565b916148546040519384616219565b8183528a526020808b20908b9084015b83821061488a57505050506001928260209283600295015281520192019201919061477c565b600160208192614899866162b6565b815201930191019091614864565b5034610275576020600319360112610275576148d76fffffffffffffffffffffffffffffffff6001600435616c1c565b604090828083516148e88582616219565b600c81527f426f756e6420726573756c7400000000000000000000000000000000000000006020820152845161495381611b7f60208201947fb60e72cc0000000000000000000000000000000000000000000000000000000086528960248401526064830190616067565b51906a636f6e736f6c652e6c6f675afa50826001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156106215783519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015611e5e57614a8c575b506001600160a01b03601f5460081c16803b156106215781809160248651809481937fb20d30a90000000000000000000000000000000000000000000000000000000083528860048401525af18015611e5e57614a77575b505060049160206001600160a01b03601f5460081c168251948580927f67eeba0c0000000000000000000000000000000000000000000000000000000082525afa908115611e0357508390611dcf5761055c92506169f6565b81614a8191616219565b61114257825f614a1e565b81614a9691616219565b61114257825f6149c6565b503461027557806003193601126102755760206040516a0422ca8b0a00a4250000008152f35b5034610275578060031936011261027557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152818180614b4160048201906001606060808401935f81525f60208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757614d73575b50507fb3418989d06835b5c215eebb4d54ed6be7bbb66eb4807164740a2e082fa782d56020604051838152a1806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757614d5e575b506001600160a01b03601f5460081c16803b15610ad2578180916024604051809481937f5ab1d61c0000000000000000000000000000000000000000000000000000000083528160048401525af1801561059757614d49575b50600460206001600160a01b03601f5460081c16604051928380927fead93c8f0000000000000000000000000000000000000000000000000000000082525afa908115610597578291614d0f575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907fa5982885000000000000000000000000000000000000000000000000000000008252151560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561059757610a975750f35b90506020813d602011614d41575b81614d2a60209383616219565b81010312610ad257614d3b9061623c565b5f614c9c565b3d9150614d1d565b81614d5391616219565b61027557805f614c4e565b81614d6891616219565b61027557805f614bf5565b81614d7d91616219565b61027557805f614b66565b503461027557806003193601126102755760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b818110614de757610e6c85610e6081870382616219565b82546001600160a01b0316845260209093019260019283019201614dd0565b5034610275578060031936011261027557806001600160a01b0360245416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576150dc575b50600460206001600160a01b03601f5460081c16604051928380927f118c38c70000000000000000000000000000000000000000000000000000000082525afa9081156105975782916150a7575b50614f3f6001600160a01b0360245416610c0e6040519384927fe2517d3f00000000000000000000000000000000000000000000000000000000602085015260248401602090939291936001600160a01b0360408201951681520152565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad25781614f9a91604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190616067565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757615092575b506001600160a01b03601f5460081c166001600160a01b0360255416813b1561068a5782916024839260405194859384927f6bcc8c1400000000000000000000000000000000000000000000000000000000845260048401525af1801561059757610d88575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610a975750f35b8161509c91616219565b61027557805f614fbf565b9150506020813d6020116150d4575b816150c360209383616219565b8101031261058b578190515f614ee1565b3d91506150b6565b816150e691616219565b61027557805f614e93565b503461027557806003193601126102755760206001600160a01b0360255416604051908152f35b5034610275578060031936011261027557602060405169d3c21bcecceda10000008152f35b5034610275578060031936011261027557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761525f575b50506001600160a01b03602354166001600160a01b03602154169060405191611999918284019284841067ffffffffffffffff8511176114a4579160a093918593617bff85398683526020830152604082015269d3c21bcecceda100000060608201526a0422ca8b0a00a425000000608082015203019082f0156114985780f35b8161526991616219565b61027557805f6151de565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757615852575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526a0422ca8b0a00a42500000060248401525af180156105975761581b575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757615806575b505b600a811061577d5750737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757615768575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa8015610597578290615734575b61549f915061687f565b806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761571f575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b30000000000000000000000000000000000000000000000000000000084526004840152600160248401525af18015610597576156e8575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576156d3575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f70d168bc000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576156be575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b8452600484015260016024840152606060448401528160648401525af1801561059757610a975750f35b816156c891616219565b61027557805f61565b565b816156dd91616219565b61027557805f6155c9565b6020813d602011615717575b8161570160209383616219565b81010312610621576157129061623c565b615565565b3d91506156f4565b8161572991616219565b61027557805f615502565b506020813d602011615760575b8161574e60209383616219565b8101031261058b5761549f9051615495565b3d9150615741565b8161577291616219565b61027557805f615447565b816001600160a01b03601f5460081c166001600160a01b0360205416813b1561114257829160848392604051948593849263062da2e360e21b845260048401526969e10de76676d08000006024840152606060448401528160648401525af18015610597576157f1575b50506001016153d4565b816157fb91616219565b61062157815f6157e7565b8161581091616219565b61027557805f6153d2565b6020813d60201161584a575b8161583460209383616219565b81010312610621576158459061623c565b615355565b3d9150615827565b8161585c91616219565b61027557805f6152e8565b5034610275578060031936011261027557604051610c2c8082019082821067ffffffffffffffff831117615aad57908291616ddd8339039082f08015615a73576001600160a01b03167fffffffffffffffffffffffff000000000000000000000000000000000000000060205416176020556040516101f68082019082821067ffffffffffffffff831117615aad57908291617a098339039082f08015615a73576001600160a01b0316807fffffffffffffffffffffffff000000000000000000000000000000000000000060215416176021556001600160a01b03602254166001600160a01b03602354169160405192611999928385019385851067ffffffffffffffff861117615a80579185939160a09593617bff863983526020830152604082015269d3c21bcecceda100000060608201526a0422ca8b0a00a425000000608082015203019082f08015615a73577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f55806001600160a01b03602054166001600160a01b0360235416813b1561068a5782916044839260405194859384927f40c10f1900000000000000000000000000000000000000000000000000000000845260048401526a084595161401484a00000060248401525af1801561059757610a975750f35b50604051903d90823e3d90fd5b6024877f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b5034610275578060031936011261027557806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757615ccc575b506001600160a01b03601f5460081c16803b15610ad2578180916024604051809481937f632214900000000000000000000000000000000000000000000000000000000083526a01a784379d99db4200000060048401525af1801561059757615cb7575b50600460206001600160a01b03601f5460081c16604051928380927f36b089d80000000000000000000000000000000000000000000000000000000082525afa908115610597578291615c82575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526a01a784379d99db4200000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561059757610a975750f35b9150506020813d602011615caf575b81615c9e60209383616219565b8101031261058b578190515f615c00565b3d9150615c91565b81615cc191616219565b61027557805f615bb2565b81615cd691616219565b61027557805f615b4e565b90503461058b575f60031936011261058b576001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b5763ca669fa760e01b825260048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561601a57616007575b508060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b3000000000000000000000000000000000000000000000000000000008452600484015269d3c21bcecceda100000060248401525af1801561059757615fd0575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757615fbb575b506001600160a01b03601f5460081c166001600160a01b036020541690803b1561068a5760405163062da2e360e21b81526001600160a01b0392909216600483015269d3c21bcecceda10000006024830152606060448301525f606483015282908290608490829084905af1801561059757615fa6575b50600460606001600160a01b03601f5460081c16604051928380927ffb8c4b510000000000000000000000000000000000000000000000000000000082525afa8015610597578283908492615f79575b615ef8929350613736906167f5565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526a034f086f3b33b68400000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561059757610a975750f35b505050615ef8615f9a6137369260603d6060116137715761375d8183616219565b91935090915082615ee9565b81615fb091616219565b61027557805f615e99565b81615fc591616219565b61027557805f615e22565b6020813d602011615fff575b81615fe960209383616219565b8101031261062157615ffa9061623c565b615dbe565b3d9150615fdc565b61601391505f90616219565b5f5f615d51565b6040513d5f823e3d90fd5b60206040818301928281528451809452019201905f5b8181106160485750505090565b82516001600160a01b031684526020938401939092019160010161603b565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b9080602083519182815201916020808360051b8301019401925f915b8383106160b757505050505090565b90919293946020806160d583601f1986600196030187528951616067565b970193019301919392906160a8565b90602080835192838152019201905f5b8181106161015750505090565b82517fffffffff00000000000000000000000000000000000000000000000000000000168452602093840193909201916001016160f4565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061616b57505050505090565b90919293946020806161c1837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b038151168452015191818582015201906160e4565b9701930193019193929061615c565b6040810190811067ffffffffffffffff8211176161ec57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f601f19910116810190811067ffffffffffffffff8211176161ec57604052565b5190811515820361058b57565b9081606091031261058b578051916040602083015192015190565b9190820391821161627157565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b67ffffffffffffffff81116161ec5760051b60200190565b90604051915f8154908160011c92600183169283156163af575b60208510841461638257848752869390811561634257506001146162fe575b506162fc92500383616219565b565b90505f9291925260205f20905f915b8183106163265750509060206162fc928201015f6162ef565b602091935080600191548385890101520191019091849261630d565b602093506162fc9592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f6162ef565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f16936162d0565b90604051918281549182825260208201905f5260205f20925f905b8060078301106165d0576162fc94549181811061659a575b818110616564575b81811061652e575b8181106164f8575b8181106164c2575b81811061648c575b818110616457575b1061642a575b500383616219565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f616422565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b16815201930161641c565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301616414565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b16815201930161640c565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301616404565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b1681520193016163fc565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b1681520193016163f4565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b1681520193016163ec565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e08201520194019201859293916163d4565b9190820180921161627157565b92919267ffffffffffffffff82116161ec5760405191616694601f8201601f191660200184616219565b82948184528183011161058b578281602093845f96015e010152565b9080601f8301121561058b5781516166ca9260200161666a565b90565b51906001600160a01b038216820361058b57565b604051906166f0604083616219565b600b82527f4d6f636b204272696467650000000000000000000000000000000000000000006020830152565b60085460ff16801561672b5790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561601a575f916167c3575b50151590565b90506020813d6020116167ed575b816167de60209383616219565b8101031261058b57515f6167bd565b3d91506167d1565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b57604051907f98296c54000000000000000000000000000000000000000000000000000000008252600482015269d3c21bcecceda100000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b5f6162fc91616219565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526a0422ca8b0a00a42500000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526969e10de76676d080000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b57604051917f98296c54000000000000000000000000000000000000000000000000000000008352600483015260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b576124fa5f91616acb60405194859384937ff320d963000000000000000000000000000000000000000000000000000000008552604060048601526044850190616067565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b576001600160a01b039081604051937f515361f60000000000000000000000000000000000000000000000000000000085521660048401521660248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b57604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b8115616bef570690565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f90838311616d585782811091821580616d4e575b616d4657616c3f8486616264565b926001840180941161627157600383111580616d3d575b616d2e5760031983101580616d24575b616d135785831115616cca57505090616c8284616c8793616264565b616be5565b908115616cc557616c98925061665d565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116162715790565b505090565b959492919095616cdb575b50505050565b83949550616c8290616ced9394616264565b908115616cc557616cfe9250616264565b6001810180911161627157905f808080616cd5565b505090506166ca9291501990616264565b5082198411616c66565b50509190506166ca925061665d565b50828411616c56565b509250505090565b5084821115616c31565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603e60248201527f5374645574696c7320626f756e642875696e743235362c75696e743235362c7560448201527f696e74323536293a204d6178206973206c657373207468616e206d696e2e00006064820152fdfe60806040523461031357604080519081016001600160401b03811182821017610226576040908152600982526845524332304d6f636b60b81b602083015280519081016001600160401b038111828210176102265760405260048152634532304d60e01b602082015281516001600160401b03811161022657600354600181811c91168015610309575b602082101461020857601f81116102a6575b50602092601f821160011461024557928192935f9261023a575b50508160011b915f199060031b1c1916176003555b80516001600160401b03811161022657600454600181811c9116801561021c575b602082101461020857601f81116101a5575b50602091601f8211600114610145579181925f9261013a575b50508160011b915f199060031b1c1916176004555b60405161091490816103188239f35b015190505f80610116565b601f1982169260045f52805f20915f5b85811061018d57508360019510610175575b505050811b0160045561012b565b01515f1960f88460031b161c191690555f8080610167565b91926020600181928685015181550194019201610155565b60045f527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b601f830160051c810191602084106101fe575b601f0160051c01905b8181106101f357506100fd565b5f81556001016101e6565b90915081906101dd565b634e487b7160e01b5f52602260045260245ffd5b90607f16906100eb565b634e487b7160e01b5f52604160045260245ffd5b015190505f806100b5565b601f1982169360035f52805f20915f5b86811061028e5750836001959610610276575b505050811b016003556100ca565b01515f1960f88460031b161c191690555f8080610268565b91926020600181928685015181550194019201610255565b60035f527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b601f830160051c810191602084106102ff575b601f0160051c01905b8181106102f4575061009b565b5f81556001016102e7565b90915081906102de565b90607f1690610089565b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c90816306fdde031461070357508063095ea7b31461067457806318160ddd1461065757806323b872dd146104e1578063313ce567146104c657806340c10f19146103e557806370a08231146103a157806395d89b41146102265780639dc29fac14610138578063a9059cbb146101075763dd62ed3e14610095575f80fd5b34610103576040600319360112610103576100ae610804565b73ffffffffffffffffffffffffffffffffffffffff6100cb610827565b91165f52600160205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060405f2054604051908152f35b5f80fd5b346101035760406003193601126101035761012d610123610804565b602435903361084a565b602060405160018152f35b3461010357604060031936011261010357610151610804565b73ffffffffffffffffffffffffffffffffffffffff602435911680156101fa57805f525f60205260405f20548281106101c8576020835f947fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef938587528684520360408620558060025403600255604051908152a3005b907fe450d38c000000000000000000000000000000000000000000000000000000005f5260045260245260445260645ffd5b7f96c6fd1e000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b34610103575f600319360112610103576040515f600454908160011c60018316928315610397575b60208210841461036a57818552849390811561032857506001146102cc575b5003601f01601f191681019067ffffffffffffffff82118183101761029f5761029b829182604052826107da565b0390f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60045f90815291507f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b5b81831061030c5750508101602001601f1961026d565b60209193508060019154838588010152019101909183926102f6565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208581019190915291151560051b84019091019150601f19905061026d565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b90607f169061024e565b346101035760206003193601126101035773ffffffffffffffffffffffffffffffffffffffff6103cf610804565b165f525f602052602060405f2054604051908152f35b34610103576040600319360112610103576103fe610804565b73ffffffffffffffffffffffffffffffffffffffff16602435811561049a576002549080820180921161046d5760207fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef915f9360025584845283825260408420818154019055604051908152a3005b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7fec442f05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b34610103575f60031936011261010357602060405160128152f35b34610103576060600319360112610103576104fa610804565b610502610827565b6044359073ffffffffffffffffffffffffffffffffffffffff831692835f52600160205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260405f20547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff811061057e575b5061012d935061084a565b8381106106235784156105f75733156105cb5761012d945f52600160205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f526020528360405f209103905584610573565b7f94280d62000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7fe602df05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b83907ffb8f41b2000000000000000000000000000000000000000000000000000000005f523360045260245260445260645ffd5b34610103575f600319360112610103576020600254604051908152f35b346101035760406003193601126101035761068d610804565b6024359033156105f75773ffffffffffffffffffffffffffffffffffffffff169081156105cb57335f52600160205260405f20825f526020528060405f20556040519081527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560203392a3602060405160018152f35b34610103575f600319360112610103575f600354908160011c600183169283156107d0575b60208210841461036a5781855284939081156103285750600114610774575003601f01601f191681019067ffffffffffffffff82118183101761029f5761029b829182604052826107da565b60035f90815291507fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b5b8183106107b45750508101602001601f1961026d565b602091935080600191548385880101520191019091839261079e565b90607f1690610728565b601f19601f602060409481855280519182918282880152018686015e5f8582860101520116010190565b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361010357565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361010357565b73ffffffffffffffffffffffffffffffffffffffff169081156101fa5773ffffffffffffffffffffffffffffffffffffffff1691821561049a57815f525f60205260405f20548181106108e257817fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef92602092855f525f84520360405f2055845f525f825260405f20818154019055604051908152a3565b827fe450d38c000000000000000000000000000000000000000000000000000000005f5260045260245260445260645ffd608080604052346015576101dc908161001a8239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c9081636813d78714610171578163bcdb83d814610080575063d3072d821461003d575f80fd5b3461007c575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007c57602060ff5f54166040519015158152f35b5f80fd5b3461007c5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007c5760043573ffffffffffffffffffffffffffffffffffffffff81160361007c5760443567ffffffffffffffff811161007c573660238201121561007c57806004013567ffffffffffffffff811161007c573691016024011161007c5760ff5f541661011557005b807f08c379a0000000000000000000000000000000000000000000000000000000006064925260206004820152601460248201527f54617267657420627269646765206661696c65640000000000000000000000006044820152fd5b3461007c5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007c5760043580151580910361007c5760ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff005f54169116175f555f80f36080604052346102a357604051601f61199938819003918201601f19168301916001600160401b0383118484101761026f5780849260a0946040528339810103126102a35761004d816102a7565b610059602083016102a7565b91610066604082016102a7565b916080606083015192015190604051946040860186811060018060401b0382111761026f57604052600b86526a4d6f636b2042726964676560a81b6020870152600180556001600160a01b038216158015610292575b610283576100d6826100d06100dc946102bb565b50610331565b506103c4565b5083516001600160401b03811161026f57600754600181811c91168015610265575b602082101461025157601f81116101ee575b50602094601f821160011461018b579481929394955f92610180575b50508160011b915f199060031b1c1916176007555b60025491600355600455600160a01b9160018060a01b03169060018060a81b03191617176002556201518042046006556040516114c190816104588239f35b015190505f8061012c565b601f1982169560075f52805f20915f5b8881106101d6575083600195969798106101be575b505050811b01600755610141565b01515f1960f88460031b161c191690555f80806101b0565b9192602060018192868501518155019401920161019b565b60075f527fa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c688601f830160051c81019160208410610247575b601f0160051c01905b81811061023c5750610110565b5f815560010161022f565b9091508190610226565b634e487b7160e01b5f52602260045260245ffd5b90607f16906100fe565b634e487b7160e01b5f52604160045260245ffd5b63d92e233d60e01b5f5260045ffd5b506001600160a01b038116156100bc565b5f80fd5b51906001600160a01b03821682036102a357565b6001600160a01b0381165f9081525f5160206119795f395f51905f52602052604090205460ff1661032c576001600160a01b03165f8181525f5160206119795f395f51905f5260205260408120805460ff191660011790553391905f5160206119195f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f5160206119595f395f51905f52602052604090205460ff1661032c576001600160a01b03165f8181525f5160206119595f395f51905f5260205260408120805460ff191660011790553391907f751b795d24b92e3d92d1d0d8f2885f4e9c9c269da350af36ae6b49069babf4bf905f5160206119195f395f51905f529080a4600190565b6001600160a01b0381165f9081525f5160206119395f395f51905f52602052604090205460ff1661032c576001600160a01b03165f8181525f5160206119395f395f51905f5260205260408120805460ff191660011790553391907fc074beb46b251f73c0fec16eba75d6bc0042d09ee17a740ebbee308dbf87f82f905f5160206119195f395f51905f529080a460019056fe6080806040526004361015610012575f80fd5b5f3560e01c90816301ffc9a714610edf575080631033b4cc14610ec2578063118c38c714610e885780631259a5c814610e6b57806318b68b8c14610951578063248a9ca31461091f5780632f2ff15d146108e25780633462fac3146108a857806336568abe1461083e57806336b089d8146108215780635ab1d61c1461078957806361b0a56e1461067e578063632214901461065d57806365d7a3c91461064257806367eeba0c146106255780636813d787146105d65780636bcc8c141461050a57806391d14854146104b4578063a217fddf1461049a578063b16e784914610467578063b20d30a914610413578063c9f5b63e146103e0578063cc3dc061146102e0578063d3072d82146102be578063d547741f1461027a578063ead93c8f14610255578063ede7cebd146101f4578063f681a862146101d75763fb8c4b511461015b575f80fd5b346101d3575f6003193601126101d35760055460045490808211156101c9578082039180831161019c57606092905b60405192835260208301526040820152f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b6060915f9061018a565b5f80fd5b346101d3575f6003193601126101d3576020600954604051908152f35b346101d3575f6003193601126101d357610229600254610212611082565b9060ff604051938493606085526060850190611157565b9173ffffffffffffffffffffffffffffffffffffffff8116602085015260a01c16151560408301520390f35b346101d3575f6003193601126101d357602060ff60025460a01c166040519015158152f35b346101d35760406003193601126101d3576102bc600435610299610fa0565b906102b76102b2825f525f602052600160405f20015490565b61122f565b611367565b005b346101d3575f6003193601126101d357602060ff600854166040519015158152f35b346101d3575f6003193601126101d3576040515f600b5461030081610fc3565b808452906001811690811561039e5750600114610340575b61033c8361032881850382611014565b604051918291602083526020830190611157565b0390f35b919050600b5f527f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9915f905b80821061038457509091508101602001610328610318565b91926001816020925483858801015201910190929161036c565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208086019190915291151560051b840190910191506103289050610318565b346101d3575f6003193601126101d357602073ffffffffffffffffffffffffffffffffffffffff60025416604051908152f35b346101d35760206003193601126101d3577f207c4cbdf55ec315a13f0d5e047732ec5d947da056e706593aa509909941cedf60406004356104526111a7565b600454908060045582519182526020820152a1005b346101d3575f6003193601126101d357602073ffffffffffffffffffffffffffffffffffffffff600a5416604051908152f35b346101d3575f6003193601126101d35760206040515f8152f35b346101d35760406003193601126101d3576104cd610fa0565b6004355f525f60205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060ff60405f2054166040519015158152f35b346101d35760206003193601126101d35773ffffffffffffffffffffffffffffffffffffffff610538610f7d565b6105406111a7565b1680156105ae5773ffffffffffffffffffffffffffffffffffffffff600254827fffffffffffffffffffffffff0000000000000000000000000000000000000000821617600255167fb07f8b1b85042d74022c867c836edeb0bcd70e135b0042390d2b1fd1082980695f80a3005b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101d35760206003193601126101d3576004358015158091036101d35760ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00600854169116176008555f80f35b346101d3575f6003193601126101d3576020600454604051908152f35b346101d3575f6003193601126101d35761033c610328611082565b346101d35760206003193601126101d3576106766111a7565b600435600355005b346101d35760606003193601126101d357610697610f7d565b60443573ffffffffffffffffffffffffffffffffffffffff81168091036101d357335f9081527fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5602052604090205460ff16156107595780156105ae5773ffffffffffffffffffffffffffffffffffffffff6102bc92604051927fa9059cbb0000000000000000000000000000000000000000000000000000000060208501526024840152602435604484015260448352610753606484611014565b1661142f565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b346101d35760206003193601126101d3576004358015158091036101d35760207fb3418989d06835b5c215eebb4d54ed6be7bbb66eb4807164740a2e082fa782d5916107d36111a7565b6002547fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff74ff00000000000000000000000000000000000000008360a01b16911617600255604051908152a1005b346101d3575f6003193601126101d3576020600354604051908152f35b346101d35760406003193601126101d357610857610fa0565b3373ffffffffffffffffffffffffffffffffffffffff821603610880576102bc90600435611367565b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101d3575f6003193601126101d35760206040517fc074beb46b251f73c0fec16eba75d6bc0042d09ee17a740ebbee308dbf87f82f8152f35b346101d35760406003193601126101d3576102bc600435610901610fa0565b9061091a6102b2825f525f602052600160405f20015490565b611295565b346101d35760206003193601126101d35760206109496004355f525f602052600160405f20015490565b604051908152f35b346101d35760606003193601126101d35761096a610f7d565b6024356044359167ffffffffffffffff83116101d357366023840112156101d35782600401359067ffffffffffffffff82116101d35736602483860101116101d357600260015414610e4357600260015560ff60025460a01c1615610e1b57335f9081527ffe0c7bd5092c7a6e463488fa239f32058c87d43e336e2e584a272132cf8dc926602052604090205460ff1615610df35773ffffffffffffffffffffffffffffffffffffffff169081156105ae578215610dcb576003548311610d64576201518042046006548111610d8c575b50610a488360055461119a565b60045410610d6457610aa36040517f23b872dd00000000000000000000000000000000000000000000000000000000602082015233602482015230604482015284606482015260648152610a9d608482611014565b8361142f565b60ff60085416610d0657817fffffffffffffffffffffffff0000000000000000000000000000000000000000600a541617600a5582600955610ae6600b54610fc3565b601f8111610c65575b505f601f8211600114610ba157819293945f92610b93575b50507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8260011b9260031b1c191617600b555b610b468260055461119a565b6005557f3dba401dc1abbf017086864b33ba815f53ab3cdbb9366ba6bcd8ec342ddd98e8602073ffffffffffffffffffffffffffffffffffffffff6002541693604051908152a360018055005b602492500101358480610b07565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0821694600b5f527f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9915f5b878110610c4a575083600195969710610c0f575b505050811b01600b55610b3a565b01602401357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600384901b60f8161c19169055848080610c01565b90926020600181926024878701013581550194019101610bed565b600b5f52601f820160051c7f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9019060208310610cde575b601f0160051c7f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db901905b818110610cd35750610aef565b5f8155600101610cc6565b7f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db99150610c9c565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601c60248201527f4d6f636b2062726964676520657865637574696f6e206661696c6564000000006044820152fd5b7f70d168bc000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fda4e39dd56d72c2ee3d132e0146bc39e905e78e3bc64c40190421c7b2bcef2ab60406005548151908482526020820152a15f60055560065584610a3b565b7f1f2a2005000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f5c427cd9000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f7bea20b2000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101d3575f6003193601126101d3576020600654604051908152f35b346101d3575f6003193601126101d35760206040517f751b795d24b92e3d92d1d0d8f2885f4e9c9c269da350af36ae6b49069babf4bf8152f35b346101d3575f6003193601126101d3576020600554604051908152f35b346101d35760206003193601126101d357600435907fffffffff0000000000000000000000000000000000000000000000000000000082168092036101d357817f7965db0b0000000000000000000000000000000000000000000000000000000060209314908115610f53575b5015158152f35b7f01ffc9a70000000000000000000000000000000000000000000000000000000091501483610f4c565b6004359073ffffffffffffffffffffffffffffffffffffffff821682036101d357565b6024359073ffffffffffffffffffffffffffffffffffffffff821682036101d357565b90600182811c9216801561100a575b6020831014610fdd57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f1691610fd2565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761105557604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051905f826007549161109583610fc3565b808352926001811690811561111a57506001146110bb575b6110b992500383611014565b565b5060075f90815290917fa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c6885b8183106110fe5750509060206110b9928201016110ad565b60209193508060019154838589010152019101909184926110e6565b602092506110b99491507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001682840152151560051b8201016110ad565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b9190820180921161019c57565b335f9081527fdfff91bcee88f6ea00b2726f6d062a509c32835793470c55bed3bcc74c0f5d37602052604090205460ff16156111df57565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f751b795d24b92e3d92d1d0d8f2885f4e9c9c269da350af36ae6b49069babf4bf60245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f205416156112665750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f1461136157805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f1461136157805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190565b905f602091828151910182855af1156114b6575f513d6114ad575073ffffffffffffffffffffffffffffffffffffffff81163b155b61146b5750565b73ffffffffffffffffffffffffffffffffffffffff907f5274afe7000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b60011415611464565b6040513d5f823e3d90fd2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0dfe0c7bd5092c7a6e463488fa239f32058c87d43e336e2e584a272132cf8dc926dfff91bcee88f6ea00b2726f6d062a509c32835793470c55bed3bcc74c0f5d37ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`oW`\x0C\x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U`\x1F\x80T\x90\x91\x16\x90\x91\x17\x90U`\"\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16a\x124\x17\x90\x91U`#\x80T\x82\x16aVx\x17\x90U`$\x80T\x82\x16a\x9A\xBC\x17\x90U`%\x80T\x90\x91\x16a\xDE\xF0\x17\x90Ua\x95\x98\x90\x81a\0t\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x08\x9C\xA9\xE0\x14a\\\xE1WP\x80c\tO'\xA0\x14aZ\xDAW\x80c\n\x92T\xE4\x14aXgW\x80c\x0FW(\x0F\x14aRtW\x80c\x10t\xA2\x1F\x14aQ=W\x80c\x11~;B\x14aQ\x18W\x80c\x13!\x7F\x90\x14aP\xF1W\x80c\x13\xA8o\x1A\x14aN\x06W\x80c\x1E\xD7\x83\x1C\x14aM\x88W\x80c#\xE1\xEB\xE7\x14aJ\xC7W\x80c$\x8E\xC3&\x14aJ\xA1W\x80c)6Yh\x14aH\xA7W\x80c*\xDE8\x80\x14aG\"W\x80c.\xD2\x11\x83\x14aE|W\x80c>^<#\x14aD\xFEW\x80c?r\x86\xF4\x14aD\x80W\x80cI{9\x18\x14aA\x8FW\x80cJa\xCF)\x14a>\x15W\x80cO\x862\xBA\x14a=\xEEW\x80cf\xD9\xA9\xA0\x14a<\xB1W\x80co\x8C\xEC\xE4\x14a;&W\x80c\x85\"l\x81\x14a:\x94W\x80c\x8BX\xCB\xAE\x14a7xW\x80c\x8FX\xA6?\x14a6\xC2W\x80c\x91j\x17\xC6\x14a6\x18W\x80c\x95m\x98\x08\x14a5#W\x80c\x95\x9B3}\x14a4\xFCW\x80c\xA3\x0F\xF4\xC2\x14a2BW\x80c\xA3\xD4H[\x14a2\x18W\x80c\xA3\xFB\x17\x15\x14a/`W\x80c\xB0FO\xDC\x14a.\xB6W\x80c\xB4M\xC9\xD6\x14a+\x06W\x80c\xB5P\x8A\xA9\x14a*tW\x80c\xB5]B\xBC\x14a'LW\x80c\xB9\xB5\xBDh\x14a \x9EW\x80c\xBAAO\xA6\x14a yW\x80c\xBEm\xA5>\x14a\x1E\xD6W\x80c\xCF\xFB\x04\x8B\x14a\x1A\xD9W\x80c\xD3\x07\\I\x14a\x14\xE6W\x80c\xD3\xB7k\xC9\x14a\x13vW\x80c\xDB\x9Bp\x8C\x14a\x11\xEFW\x80c\xDC\xCCW\xF1\x14a\x0E\x8FW\x80c\xE2\x0C\x9Fq\x14a\x0E\x01W\x80c\xE8kO\xA7\x14a\n\xD5W\x80c\xE9\xD3\xD5\x86\x14a\t/W\x80c\xF3\xED+\x05\x14a\x06\xB5W\x80c\xF8Q\xA4@\x14a\x06\x8EW\x80c\xF9~\x84g\x14a\x02\xC1W\x80c\xFAv&\xD4\x14a\x02\x9EW\x80c\xFC\x0CTj\x14a\x02xWc\xFC\x9C\x8D9\x14a\x02OW_\x80\xFD[4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[\x80\xFD[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x81;\x15a\x06\x8AW\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\x06uW[P`$\x90`\x01`\x01`\xA0\x1B\x03` T\x16` `\x01`\x01`\xA0\x1B\x03`\"T\x16\x91`@Q\x94\x85\x80\x92\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x85`\x04\x83\x01RZ\xFA\x92\x83\x15a\x05\x97W\x82\x93a\x06>W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x06)W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x82;\x15a\x06%W`d\x84\x92\x83`@Q\x95\x86\x94\x85\x93\x7Fa\xB0\xA5n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x85\x01R`D\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\x06\x0CW[PP`\x01`\x01`\xA0\x1B\x03` T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x90\x81\x15a\x06\x01W\x84\x91a\x05\xCFW[Pi\x15-\x02\xC7\xE1J\xF6\x80\0\0\x82\x01\x80\x92\x11a\x05\xA2W\x90a\x05\x01\x91ai\xF6V[` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`$`@Q\x80\x94\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x80\x15a\x05\x97W\x82\x90a\x05_W[a\x05\\\x91Pai\x80V[\x80\xF3[P` \x81=` \x11a\x05\x8FW[\x81a\x05y` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa\x05\\\x90Qa\x05RV[_\x80\xFD[=\x91Pa\x05lV[`@Q=\x84\x82>=\x90\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11a\x05\xF9W[\x81a\x05\xEA` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQ_a\x04\xE2V[=\x91Pa\x05\xDDV[`@Q=\x86\x82>=\x90\xFD[\x81a\x06\x16\x91ab\x19V[a\x06!W\x81_a\x04\x84V[P\x80\xFD[\x83\x80\xFD[\x81a\x063\x91ab\x19V[a\x06!W\x81_a\x03\xFEV[\x91P\x91P` \x81=` \x11a\x06mW[\x81a\x06[` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BW\x82\x90Q\x91_a\x03\xA7V[=\x91Pa\x06NV[\x81a\x06\x7F\x91ab\x19V[a\x02uW\x80_a\x03HV[PP\xFD[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R`\x01`$\x82\x01R\x81`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\t\x1AW[P`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x91\x7F\xB0\x7F\x8B\x1B\x85\x04-t\x02,\x86|\x83n\xDE\xB0\xBC\xD7\x0E\x13[\0B9\r+\x1F\xD1\x08)\x80i\x84\x80\xA3`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x8AWc\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\t\x05W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x81;\x15a\x06\x8AW\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7Fk\xCC\x8C\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\x08\xF0W[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xC9\xF5\xB6>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x90a\x08\xB5W[a\x05\\\x91P`\x01`\x01`\xA0\x1B\x03`%T\x16\x90aj\xF1V[P` \x81=` \x11a\x08\xE8W[\x81a\x08\xCF` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa\x08\xE3a\x05\\\x91af\xCDV[a\x08\x9EV[=\x91Pa\x08\xC2V[\x81a\x08\xFA\x91ab\x19V[a\x02uW\x80_a\x08PV[\x81a\t\x0F\x91ab\x19V[a\x02uW\x80_a\x07\xEAV[\x81a\t$\x91ab\x19V[a\x02uW\x80_a\x07KV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\n\xBDW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\n\xA8W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01R\x81`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a\n\xA1\x91ab\x19V[a\x02uW\x80\xF3[\x81a\n\xB2\x91ab\x19V[a\x02uW\x80_a\n5V[\x81a\n\xC7\x91ab\x19V[a\x02uW\x80_a\t\xA3V[P\xFD[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`$T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\r\xECW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97W\x82\x91a\r\xB7W[Pa\x0C\x1C`\x01`\x01`\xA0\x1B\x03`$T\x16a\x0C\x0E`@Q\x93\x84\x92\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x85\x01R`$\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03`\x1F\x19\x81\x01\x83R\x82ab\x19V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W\x81a\x0Cw\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a`gV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\r\xA2W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x82;\x15a\r\x9DW`d\x84\x92\x83`@Q\x95\x86\x94\x85\x93\x7Fa\xB0\xA5n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01Ra\x03\xE8`$\x85\x01R`D\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\r\x88W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a\r\x92\x91ab\x19V[a\x02uW\x80_a\r\x1AV[PPP\xFD[\x81a\r\xAC\x91ab\x19V[a\x02uW\x80_a\x0C\x9CV[\x91PP` \x81=` \x11a\r\xE4W[\x81a\r\xD3` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BW\x81\x90Q_a\x0B\xB0V[=\x91Pa\r\xC6V[\x81a\r\xF6\x91ab\x19V[a\x02uW\x80_a\x0BbV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x0EpWa\x0El\x85a\x0E`\x81\x87\x03\x82ab\x19V[`@Q\x91\x82\x91\x82a`%V[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x0EIV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x11\x04W\x83\x91a\x11\xBDW[P`\"T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01\x81\x90R\x90` \x81`D\x81\x86Z\xFA\x80\x15a\x06\x01W\x84\x90a\x11\x82W[a\x0FO\x91PaksV[`@Q\x7F\x11\x8C8\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x06\x01W\x84\x91a\x11NW[P`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R` \x81\x80`D\x81\x01[\x03\x81\x85Z\xFA\x80\x15a\x11\x04W\x83\x90a\x11\x0FW[a\x0F\xF0\x91PaksV[`@Q\x7F4b\xFA\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x11\x04W\x83\x91a\x10\xD0W[P`#T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90` \x90\x82\x90\x81\x80`D\x81\x01[\x03\x91Z\xFA\x80\x15a\x05\x97W\x82\x90a\x10\x95W[a\x05\\\x91PaksV[P` \x81=` \x11a\x10\xC8W[\x81a\x10\xAF` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa\x10\xC3a\x05\\\x91ab<V[a\x10\x8BV[=\x91Pa\x10\xA2V[\x90P` \x81=` \x11a\x10\xFCW[\x81a\x10\xEB` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQa\x10za\x10-V[=\x91Pa\x10\xDEV[`@Q=\x85\x82>=\x90\xFD[P` \x81=` \x11a\x11FW[\x81a\x11)` \x93\x83ab\x19V[\x81\x01\x03\x12a\x11BWa\x11=a\x0F\xF0\x91ab<V[a\x0F\xE6V[\x82\x80\xFD[=\x91Pa\x11\x1CV[\x90P` \x81=` \x11a\x11zW[\x81a\x11i` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQa\x0F\xD4a\x0F\x8CV[=\x91Pa\x11\\V[P` \x81=` \x11a\x11\xB5W[\x81a\x11\x9C` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06%Wa\x11\xB0a\x0FO\x91ab<V[a\x0FEV[=\x91Pa\x11\x8FV[\x90P` \x81=` \x11a\x11\xE7W[\x81a\x11\xD8` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQ_a\x0E\xECV[=\x91Pa\x11\xCBV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x13aW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x13LW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\n\xD2W\x81\x80\x91`\x84`@Q\x80\x94\x81\x93c\x06-\xA2\xE3`\xE2\x1B\x83R\x81`\x04\x84\x01Ra\x03\xE8`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a\x13V\x91ab\x19V[a\x02uW\x80_a\x12\xF5V[\x81a\x13k\x91ab\x19V[a\x02uW\x80_a\x12cV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x14\xD1W[PP`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`@Q\x91a\x19\x99\x91\x82\x84\x01\x92\x84\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x14\xA4W\x91`\xA0\x93\x91\x85\x93a{\xFF\x859\x82R\x85` \x83\x01R`@\x82\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0``\x82\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`\x80\x82\x01R\x03\x01\x90\x82\xF0\x15a\x14\x98W\x80\xF3[`@Q\x90=\x90\x82>=\x90\xFD[`$\x86\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81a\x14\xDB\x91ab\x19V[a\x02uW\x80_a\x14\x17V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x1A\xC4W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rj\x01\xA7\x847\x9D\x99\xDBB\0\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\x1A\x8DW[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x1AxW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x90\x80;\x15a\x06\x8AW`@Qc\x06-\xA2\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05\x97Wa\x1AcW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x90a\x1A/W[a\x16\xFA\x91Pag\xF5V[b\x01Q\x80B\x01\x80B\x11a\x1A\x02W\x81\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x19\xEDW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a\x17\xE3`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93_\x81R_` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x19\xD8W[PP\x7F\xDAN9\xDDV\xD7,.\xE3\xD12\xE0\x14k\xC3\x9E\x90^x\xE3\xBCd\xC4\x01\x90B\x1C{+\xCE\xF2\xAB`@\x80Qb\x01Q\x80B\x04\x81Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0` \x82\x01R\xA1\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x19\xC3W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x90\x80;\x15a\x06\x8AW`@Qc\x06-\xA2\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05\x97Wa\x19\xAEW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x90a\x19zW[a\x05\\\x91Pag\xF5V[P` \x81=` \x11a\x19\xA6W[\x81a\x19\x94` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa\x05\\\x90Qa\x19pV[=\x91Pa\x19\x87V[\x81a\x19\xB8\x91ab\x19V[a\x02uW\x80_a\x19\"V[\x81a\x19\xCD\x91ab\x19V[a\x02uW\x80_a\x18\xABV[\x81a\x19\xE2\x91ab\x19V[a\x02uW\x80_a\x18\x08V[\x81a\x19\xF7\x91ab\x19V[a\x02uW\x80_a\x17xV[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[P` \x81=` \x11a\x1A[W[\x81a\x1AI` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa\x16\xFA\x90Qa\x16\xF0V[=\x91Pa\x1A<V[\x81a\x1Am\x91ab\x19V[a\x02uW\x80_a\x16\xA2V[\x81a\x1A\x82\x91ab\x19V[a\x02uW\x80_a\x16+V[` \x81=` \x11a\x1A\xBCW[\x81a\x1A\xA6` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa\x1A\xB7\x90ab<V[a\x15\xC7V[=\x91Pa\x1A\x99V[\x81a\x1A\xCE\x91ab\x19V[a\x02uW\x80_a\x15ZV[P4a\x02uW` `\x03\x196\x01\x12a\x02uWa\x1B\x03i\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`\x01`\x045al\x1CV[`@\x90\x82\x80\x83Qa\x1B\x14\x85\x82ab\x19V[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x84Qa\x1B\x93\x81a\x1B\x7F` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x89`$\x84\x01R`d\x83\x01\x90a`gV[\x87`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82ab\x19V[Q\x90jconsole.logZ\xFAP\x82`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W\x83Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x1E^Wa\x1E\xC1W[PP` \x80T`\x1FT\x84Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03`\x08\x92\x90\x92\x1C\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x92\x91\x83\x91\x16\x81\x87\x81`D\x81\x01\x03\x92Z\xF1\x80\x15a\x1E\xB4Wa\x1E}W[P\x82`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W\x83Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x1E^Wa\x1EhW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x11BW\x82\x91`\x84\x83\x92\x87Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01R\x88`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x1E^Wa\x1EIW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x91\x80Q\x7F\xF6\x81\xA8b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA\x90\x81\x15a\x1E?W\x85\x91a\x1E\x0CW[P`\x04\x93a\x1D\x8B\x84` \x93ai\xF6V[\x82Q\x94\x85\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x1E\x03WP\x83\x90a\x1D\xCFW[a\x05\\\x92Pai\xF6V[P` \x82=` \x11a\x1D\xFBW[\x81a\x1D\xE9` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa\x05\\\x91Qa\x1D\xC5V[=\x91Pa\x1D\xDCV[Q=\x85\x82>=\x90\xFD[\x90P` \x81=` \x11a\x1E7W[\x81a\x1E'` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQ`\x04a\x1D{V[=\x91Pa\x1E\x1AV[\x82Q=\x87\x82>=\x90\xFD[\x81a\x1ES\x91ab\x19V[a\x11BW\x82_a\x1D-V[\x84Q=\x84\x82>=\x90\xFD[\x81a\x1Er\x91ab\x19V[a\x11BW\x82_a\x1C\xCEV[` \x81=` \x11a\x1E\xACW[\x81a\x1E\x96` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06%Wa\x1E\xA7\x90ab<V[a\x1CkV[=\x91Pa\x1E\x89V[PPPQ\x90=\x90\x82>=\x90\xFD[\x81a\x1E\xCB\x91ab\x19V[a\x11BW\x82_a\x1C\x06V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`$T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa dW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\\B|\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa OW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x90\x80;\x15a\x06\x8AW`@Qc\x06-\xA2\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01Ra\x03\xE8`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90\x81\x83\x81`\x84\x81\x01[\x03\x92Z\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a Y\x91ab\x19V[a\x02uW\x80_a\x1F\xDCV[\x81a n\x91ab\x19V[a\x02uW\x80_a\x1FJV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` a \x94ag\x1CV[`@Q\x90\x15\x15\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`@Q` \x80\x82\x01R`\t`@\x82\x01R\x7Ftest data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R``\x81Ra \xF4`\x80\x82ab\x19V[\x81`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa'7W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x11\x04Wa'\0W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W\x81`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R\x81`$\x82\x01R`\x01`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa&\xEBW[PP`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x7F=\xBA@\x1D\xC1\xAB\xBF\x01p\x86\x86K3\xBA\x81_S\xAB<\xDB\xB96k\xA6\xBC\xD8\xEC4-\xDD\x98\xE8` `@Qii\xE1\r\xE7fv\xD0\x80\0\0\x81R\xA3\x81`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa&\xD6W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x11BW\x82\x90`@Q\x92\x83\x91c\x06-\xA2\xE3`\xE2\x1B\x83R`\x04\x83\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x83\x01R```D\x83\x01R\x81\x83\x81a#\\`d\x82\x01\x8Aa`gV[\x03\x92Z\xF1\x80\x15a\x05\x97Wa&\xC1W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x06\x01W\x84\x90a&\x8DW[a#\xC2\x91Pai\0V[`@Q\x7F\xF6\x81\xA8b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x06\x01W\x84\x90a&YW[a$\x08\x91Pai\0V[\x82`@Q\x92\x7F\xB1nxI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R` \x84`\x04\x81\x86Z\xFA\x93\x84\x15a\x05\x97W\x82\x94a&\x1DW[Pa$^`\x01`\x01`\xA0\x1B\x03` T\x16\x80\x95aj\xF1V[`@Q\x7F\xCC=\xC0a\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81`\x04\x81\x87Z\xFA\x90\x81\x15a\x11\x04W\x83\x91a%\xC8W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x11BWa$\xFA\x83\x91a%\x0C`@Q\x94\x85\x93\x84\x93\x7F\x97bF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`@`\x04\x86\x01R`D\x85\x01\x90a`gV[\x90`\x03\x19\x84\x83\x03\x01`$\x85\x01Ra`gV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05\x97Wa%\xB3W[PP` \x90`$`@Q\x80\x94\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x80\x15a\x05\x97W\x82\x90a%\x7FW[a\x05\\\x91Pai\0V[P` \x81=` \x11a%\xABW[\x81a%\x99` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa\x05\\\x90Qa%uV[=\x91Pa%\x8CV[\x81a%\xBD\x91ab\x19V[a\x11BW\x82_a%0V[\x90P=\x80\x84\x83>a%\xD9\x81\x83ab\x19V[\x81\x01\x90` \x81\x83\x03\x12a\x06%W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a&\x19W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x06%W\x81Qa&\x13\x92` \x01afjV[_a$\x9AV[\x84\x80\xFD[\x90\x93P` \x81=` \x11a&QW[\x81a&9` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa&J\x90af\xCDV[\x92_a$GV[=\x91Pa&,V[P` \x81=` \x11a&\x85W[\x81a&s` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa$\x08\x90Qa#\xFEV[=\x91Pa&fV[P` \x81=` \x11a&\xB9W[\x81a&\xA7` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa#\xC2\x90Qa#\xB8V[=\x91Pa&\x9AV[\x81a&\xCB\x91ab\x19V[a\x06!W\x81_a#kV[\x81a&\xE0\x91ab\x19V[a\x06!W\x81_a\"\xFBV[\x81a&\xF5\x91ab\x19V[a\x06!W\x81_a\"IV[` \x81=` \x11a'/W[\x81a'\x19` \x93\x83ab\x19V[\x81\x01\x03\x12a\x11BWa'*\x90ab<V[a!\xC3V[=\x91Pa'\x0CV[\x81a'A\x91ab\x19V[a\x06!W\x81_a!WV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa*_W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa*(W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa*\x13W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa)\xFEW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xF6\x81\xA8b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97W\x82\x91a)\xC9W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x91PP` \x81=` \x11a)\xF6W[\x81a)\xE5` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BW\x81\x90Q_a)HV[=\x91Pa)\xD8V[\x81a*\x08\x91ab\x19V[a\x02uW\x80_a(\xFAV[\x81a*\x1D\x91ab\x19V[a\x02uW\x80_a(\x90V[` \x81=` \x11a*WW[\x81a*A` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa*R\x90ab<V[a(,V[=\x91Pa*4V[\x81a*i\x91ab\x19V[a\x02uW\x80_a'\xC0V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x19Ta*\x91\x81ab\x9EV[\x91a*\x9F`@Q\x93\x84ab\x19V[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a*\xE9W`@Q` \x80\x82R\x81\x90a\x0El\x90\x82\x01\x88a`\x8CV[`\x01` \x81\x92a*\xF8\x85ab\xB6V[\x81R\x01\x92\x01\x92\x01\x91\x90a*\xCCV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xC9\xF5\xB6>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x11\x04W\x83\x90a.{W[a+y\x91P`\x01`\x01`\xA0\x1B\x03`!T\x16\x90aj\xF1V[`@Q\x7Fe\xD7\xA3\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81`\x04\x81\x85Z\xFA\x80\x15a\x11\x04W\x83\x90a.9W[a+\xC7\x91Pa+\xC1af\xE1V[\x90ajlV[`@Q\x7F6\xB0\x89\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x11\x04W\x83\x90a.\x05W[a,\r\x91Pag\xF5V[`@Q\x7Fg\xEE\xBA\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x11\x04W\x83\x90a-\xD1W[a,S\x91Pah\x7FV[`@Q\x7F\xEA\xD9<\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x11\x04W\x83\x90a-\x96W[a,\x99\x91PaksV[`@Q\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x11\x04W\x83\x91a-cW[P`\x04\x91a,\xE5` \x92ai\x80V[`@Q\x92\x83\x80\x92\x7F\x12Y\xA5\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x90a-/W[a\x05\\\x91Pb\x01Q\x80B\x04\x90ai\xF6V[P` \x81=` \x11a-[W[\x81a-I` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa\x05\\\x90Qa-\x1EV[=\x91Pa-<V[\x90P` \x81=` \x11a-\x8EW[\x81a-~` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQ`\x04a,\xD6V[=\x91Pa-qV[P` \x81=` \x11a-\xC9W[\x81a-\xB0` \x93\x83ab\x19V[\x81\x01\x03\x12a\x11BWa-\xC4a,\x99\x91ab<V[a,\x8FV[=\x91Pa-\xA3V[P` \x81=` \x11a-\xFDW[\x81a-\xEB` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa,S\x90Qa,IV[=\x91Pa-\xDEV[P` \x81=` \x11a.1W[\x81a.\x1F` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa,\r\x90Qa,\x03V[=\x91Pa.\x12V[P=\x80\x84\x83>a.I\x81\x83ab\x19V[\x81\x01\x90` \x81\x83\x03\x12a\x06%W\x80Q\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a&\x19Wa+\xC7\x92a.v\x92\x01af\xB0V[a+\xB4V[P` \x81=` \x11a.\xAEW[\x81a.\x95` \x93\x83ab\x19V[\x81\x01\x03\x12a\x11BWa.\xA9a+y\x91af\xCDV[a+bV[=\x91Pa.\x88V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x1CTa.\xD3\x81ab\x9EV[\x91a.\xE1`@Q\x93\x84ab\x19V[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a/#W`@Q\x80a\x0El\x87\x82aa9V[`\x02` `\x01\x92`@Qa/6\x81aa\xD0V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra/N\x85\x87\x01ac\xB9V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a/\x0EV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa2\x03W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\x01`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa1\xCCW[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa1\xB7W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7Fp\xD1h\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa1\xA2W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\x01`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a1\xAC\x91ab\x19V[a\x02uW\x80_a16V[\x81a1\xC1\x91ab\x19V[a\x02uW\x80_a0\xA4V[` \x81=` \x11a1\xFBW[\x81a1\xE5` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa1\xF6\x90ab<V[a0@V[=\x91Pa1\xD8V[\x81a2\r\x91ab\x19V[a\x02uW\x80_a/\xD4V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa4\xE7W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\x01`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa4\xB0W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa4\x9BW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7Fp\xD1h\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa4\x86W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\x01`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a4\x90\x91ab\x19V[a\x02uW\x80_a4\x19V[\x81a4\xA5\x91ab\x19V[a\x02uW\x80_a3\x87V[` \x81=` \x11a4\xDFW[\x81a4\xC9` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa4\xDA\x90ab<V[a3#V[=\x91Pa4\xBCV[\x81a4\xF1\x91ab\x19V[a\x02uW\x80_a2\xB6V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x04\x81`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xED\xE7\xCE\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x83\x92\x84\x92a5\xADW[Pa\x05\\\x92a5\x96a5\xA8\x92a+\xC1af\xE1V[`\x01`\x01`\xA0\x1B\x03`!T\x16\x90aj\xF1V[aksV[\x92PPP=\x80\x83\x83>a5\xC0\x81\x83ab\x19V[\x81\x01\x90``\x81\x83\x03\x12a\x11BW\x80Q\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x06%Wa5\xF1a5\xA8\x91a\x05\\\x94\x84\x01af\xB0V[a5\x96a6\x0C`@a6\x05` \x87\x01af\xCDV[\x95\x01ab<V[\x93\x94\x91\x92Pa5\x82\x90PV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x1DTa65\x81ab\x9EV[\x91a6C`@Q\x93\x84ab\x19V[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a6\x85W`@Q\x80a\x0El\x87\x82aa9V[`\x02` `\x01\x92`@Qa6\x98\x81aa\xD0V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra6\xB0\x85\x87\x01ac\xB9V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a6pV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x04```\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xFB\x8CKQ\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97Wa\x05\\\x91\x83\x84\x90\x85\x92a7;W[a76\x92\x93Pa76\x90ai\x80V[ah\x7FV[PPPa76a7ea76\x92``=``\x11a7qW[a7]\x81\x83ab\x19V[\x81\x01\x90abIV[\x91\x93P\x90\x91P\x82a7'V[P=a7SV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa:\x7FW[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa:HW[P\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01Ri?\x87\x08W\xA3\xE0\xE3\x80\0\0`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa:3W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01Ri*Z\x05\x8F\xC2\x95\xED\0\0\0`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa:\x1EW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa:\tW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x90a%\x7FWa\x05\\\x91Pai\0V[\x81a:\x13\x91ab\x19V[a\x02uW\x80_a9\xB2V[\x81a:(\x91ab\x19V[a\x02uW\x80_a9FV[\x81a:=\x91ab\x19V[a\x02uW\x80_a8\xDCV[` \x81=` \x11a:wW[\x81a:a` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa:r\x90ab<V[a8qV[=\x91Pa:TV[\x81a:\x89\x91ab\x19V[a\x02uW\x80_a8\x05V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x1ATa:\xB1\x81ab\x9EV[\x91a:\xBF`@Q\x93\x84ab\x19V[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a;\tW`@Q` \x80\x82R\x81\x90a\x0El\x90\x82\x01\x88a`\x8CV[`\x01` \x81\x92a;\x18\x85ab\xB6V[\x81R\x01\x92\x01\x92\x01\x91\x90a:\xECV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa<\x9CW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa<\x87W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\n\xD2W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7Fk\xCC\x8C\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a<\x91\x91ab\x19V[a\x02uW\x80_a<,V[\x81a<\xA6\x91ab\x19V[a\x02uW\x80_a;\x9AV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x1BTa<\xCE\x81ab\x9EV[a<\xDB`@Q\x91\x82ab\x19V[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a=\xB3W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a=HWPPPP\x03\x90\xF3[\x91\x93` a=\xA3\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a=\x93\x83Q`@\x84R`@\x84\x01\x90a`gV[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra`\xE4V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a=9V[`\x02` `\x01\x92`@Qa=\xC6\x81aa\xD0V[a=\xCF\x86ab\xB6V[\x81Ra=\xDC\x85\x87\x01ac\xB9V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a=\x0BV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x90\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaAzW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\n\xD2W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7FZ\xB1\xD6\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97WaAeW[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaAPW[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ra\x03\xE8`$\x84\x01RZ\xF1\x80\x15a\x05\x97WaA\x19W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaA\x04W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F{\xEA \xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa OWP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x90\x80;\x15a\x06\x8AW`@Qc\x06-\xA2\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01Ra\x03\xE8`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90\x81\x83\x81`\x84\x81\x01a >V[\x81aA\x0E\x91ab\x19V[a\x02uW\x80_a@\rV[` \x81=` \x11aAHW[\x81aA2` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!WaAC\x90ab<V[a?\xA9V[=\x91PaA%V[\x81aAZ\x91ab\x19V[a\x02uW\x80_a?EV[\x81aAo\x91ab\x19V[a\x02uW\x80_a>\xE2V[\x81aA\x84\x91ab\x19V[a\x02uW\x80_a>\x89V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80aB\t`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93_\x81R_` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaDkW[PP\x7F |L\xBD\xF5^\xC3\x15\xA1?\r^\x04w2\xEC]\x94}\xA0V\xE7\x06Y:\xA5\t\x90\x99A\xCE\xDF`@\x80Qj\x04\"\xCA\x8B\n\0\xA4%\0\0\0\x81Rj\x08E\x95\x16\x14\x01HJ\0\0\0` \x82\x01R\xA1\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaDVW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\n\xD2W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xB2\r0\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rj\x08E\x95\x16\x14\x01HJ\0\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97WaDAW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7Fg\xEE\xBA\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97W\x82\x91aD\x0CW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rj\x08E\x95\x16\x14\x01HJ\0\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x91PP` \x81=` \x11aD9W[\x81aD(` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BW\x81\x90Q_aC\x8AV[=\x91PaD\x1BV[\x81aDK\x91ab\x19V[a\x02uW\x80_aC<V[\x81aD`\x91ab\x19V[a\x02uW\x80_aB\xD8V[\x81aDu\x91ab\x19V[a\x02uW\x80_aB.V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10aD\xDFWa\x0El\x85a\x0E`\x81\x87\x03\x82ab\x19V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aD\xC8V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10aE]Wa\x0El\x85a\x0E`\x81\x87\x03\x82ab\x19V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aEFV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaG\rW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaF\xF8W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92\x7Fa\xB0\xA5n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ra\x03\xE8`$\x84\x01R\x81`D\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81aG\x02\x91ab\x19V[a\x02uW\x80_aF\x82V[\x81aG\x17\x91ab\x19V[a\x02uW\x80_aE\xF0V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x1ETaG?\x81ab\x9EV[aGL`@Q\x91\x82ab\x19V[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10aH\x1EW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10aG\xB9WPPPP\x03\x90\xF3[\x91\x93` aH\x0E\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R`@\x83\x8AQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a`\x8CV[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92aG\xAAV[`@QaH*\x81aa\xD0V[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80TaHF\x81ab\x9EV[\x91aHT`@Q\x93\x84ab\x19V[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10aH\x8AWPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90aG|V[`\x01` \x81\x92aH\x99\x86ab\xB6V[\x81R\x01\x93\x01\x91\x01\x90\x91aHdV[P4a\x02uW` `\x03\x196\x01\x12a\x02uWaH\xD7o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\x045al\x1CV[`@\x90\x82\x80\x83QaH\xE8\x85\x82ab\x19V[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x84QaIS\x81a\x1B\x7F` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x89`$\x84\x01R`d\x83\x01\x90a`gV[Q\x90jconsole.logZ\xFAP\x82`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W\x83Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x1E^WaJ\x8CW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x06!W\x81\x80\x91`$\x86Q\x80\x94\x81\x93\x7F\xB2\r0\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x88`\x04\x84\x01RZ\xF1\x80\x15a\x1E^WaJwW[PP`\x04\x91` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x82Q\x94\x85\x80\x92\x7Fg\xEE\xBA\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x1E\x03WP\x83\x90a\x1D\xCFWa\x05\\\x92Pai\xF6V[\x81aJ\x81\x91ab\x19V[a\x11BW\x82_aJ\x1EV[\x81aJ\x96\x91ab\x19V[a\x11BW\x82_aI\xC6V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `@Qj\x04\"\xCA\x8B\n\0\xA4%\0\0\0\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80aKA`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93_\x81R_` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaMsW[PP\x7F\xB3A\x89\x89\xD0h5\xB5\xC2\x15\xEE\xBBMT\xEDk\xE7\xBB\xB6n\xB4\x80qdt\n.\x08/\xA7\x82\xD5` `@Q\x83\x81R\xA1\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaM^W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\n\xD2W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7FZ\xB1\xD6\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97WaMIW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xEA\xD9<\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97W\x82\x91aM\x0FW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x90P` \x81=` \x11aMAW[\x81aM*` \x93\x83ab\x19V[\x81\x01\x03\x12a\n\xD2WaM;\x90ab<V[_aL\x9CV[=\x91PaM\x1DV[\x81aMS\x91ab\x19V[a\x02uW\x80_aLNV[\x81aMh\x91ab\x19V[a\x02uW\x80_aK\xF5V[\x81aM}\x91ab\x19V[a\x02uW\x80_aKfV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10aM\xE7Wa\x0El\x85a\x0E`\x81\x87\x03\x82ab\x19V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aM\xD0V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`$T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaP\xDCW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x11\x8C8\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97W\x82\x91aP\xA7W[PaO?`\x01`\x01`\xA0\x1B\x03`$T\x16a\x0C\x0E`@Q\x93\x84\x92\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x85\x01R`$\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W\x81aO\x9A\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a`gV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaP\x92W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x81;\x15a\x06\x8AW\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7Fk\xCC\x8C\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\r\x88WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81aP\x9C\x91ab\x19V[a\x02uW\x80_aO\xBFV[\x91PP` \x81=` \x11aP\xD4W[\x81aP\xC3` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BW\x81\x90Q_aN\xE1V[=\x91PaP\xB6V[\x81aP\xE6\x91ab\x19V[a\x02uW\x80_aN\x93V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `@Qi\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaR_W[PP`\x01`\x01`\xA0\x1B\x03`#T\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`@Q\x91a\x19\x99\x91\x82\x84\x01\x92\x84\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x14\xA4W\x91`\xA0\x93\x91\x85\x93a{\xFF\x859\x86\x83R` \x83\x01R`@\x82\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0``\x82\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`\x80\x82\x01R\x03\x01\x90\x82\xF0\x15a\x14\x98W\x80\xF3[\x81aRi\x91ab\x19V[a\x02uW\x80_aQ\xDEV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaXRW[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97WaX\x1BW[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaX\x06W[P[`\n\x81\x10aW}WPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaWhW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x90aW4W[aT\x9F\x91Pah\x7FV[\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaW\x1FW[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x05\x97WaV\xE8W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaV\xD3W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7Fp\xD1h\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaV\xBEW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81aV\xC8\x91ab\x19V[a\x02uW\x80_aV[V[\x81aV\xDD\x91ab\x19V[a\x02uW\x80_aU\xC9V[` \x81=` \x11aW\x17W[\x81aW\x01` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!WaW\x12\x90ab<V[aUeV[=\x91PaV\xF4V[\x81aW)\x91ab\x19V[a\x02uW\x80_aU\x02V[P` \x81=` \x11aW`W[\x81aWN` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWaT\x9F\x90QaT\x95V[=\x91PaWAV[\x81aWr\x91ab\x19V[a\x02uW\x80_aTGV[\x81`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x11BW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97WaW\xF1W[PP`\x01\x01aS\xD4V[\x81aW\xFB\x91ab\x19V[a\x06!W\x81_aW\xE7V[\x81aX\x10\x91ab\x19V[a\x02uW\x80_aS\xD2V[` \x81=` \x11aXJW[\x81aX4` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!WaXE\x90ab<V[aSUV[=\x91PaX'V[\x81aX\\\x91ab\x19V[a\x02uW\x80_aR\xE8V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`@Qa\x0C,\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17aZ\xADW\x90\x82\x91am\xDD\x839\x03\x90\x82\xF0\x80\x15aZsW`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U`@Qa\x01\xF6\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17aZ\xADW\x90\x82\x91az\t\x839\x03\x90\x82\xF0\x80\x15aZsW`\x01`\x01`\xA0\x1B\x03\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x91`@Q\x92a\x19\x99\x92\x83\x85\x01\x93\x85\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17aZ\x80W\x91\x85\x93\x91`\xA0\x95\x93a{\xFF\x869\x83R` \x83\x01R`@\x82\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0``\x82\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`\x80\x82\x01R\x03\x01\x90\x82\xF0\x80\x15aZsW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU\x80`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x06\x8AW\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rj\x08E\x95\x16\x14\x01HJ\0\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x87\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\\\xCCW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\n\xD2W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7Fc\"\x14\x90\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rj\x01\xA7\x847\x9D\x99\xDBB\0\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\\\xB7W[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F6\xB0\x89\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97W\x82\x91a\\\x82W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rj\x01\xA7\x847\x9D\x99\xDBB\0\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x91PP` \x81=` \x11a\\\xAFW[\x81a\\\x9E` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BW\x81\x90Q_a\\\0V[=\x91Pa\\\x91V[\x81a\\\xC1\x91ab\x19V[a\x02uW\x80_a[\xB2V[\x81a\\\xD6\x91ab\x19V[a\x02uW\x80_a[NV[\x90P4a\x05\x8BW_`\x03\x196\x01\x12a\x05\x8BW`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BWc\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a`\x1AWa`\x07W[P\x80` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa_\xD0W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa_\xBBW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x90\x80;\x15a\x06\x8AW`@Qc\x06-\xA2\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05\x97Wa_\xA6W[P`\x04```\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xFB\x8CKQ\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x83\x90\x84\x92a_yW[a^\xF8\x92\x93Pa76\x90ag\xF5V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rj\x03O\x08o;3\xB6\x84\0\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05\x97Wa\n\x97WP\xF3[PPPa^\xF8a_\x9Aa76\x92``=``\x11a7qWa7]\x81\x83ab\x19V[\x91\x93P\x90\x91P\x82a^\xE9V[\x81a_\xB0\x91ab\x19V[a\x02uW\x80_a^\x99V[\x81a_\xC5\x91ab\x19V[a\x02uW\x80_a^\"V[` \x81=` \x11a_\xFFW[\x81a_\xE9` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa_\xFA\x90ab<V[a]\xBEV[=\x91Pa_\xDCV[a`\x13\x91P_\x90ab\x19V[__a]QV[`@Q=_\x82>=\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a`HWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a`;V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a`\xB7WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a`\xD5\x83`\x1F\x19\x86`\x01\x96\x03\x01\x87R\x89Qa`gV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a`\xA8V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10aa\x01WPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a`\xF4V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aakWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aa\xC1\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a`\xE4V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aa\\V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aa\xECW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aa\xECW`@RV[Q\x90\x81\x15\x15\x82\x03a\x05\x8BWV[\x90\x81``\x91\x03\x12a\x05\x8BW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x91\x90\x82\x03\x91\x82\x11abqWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11aa\xECW`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15ac\xAFW[` \x85\x10\x84\x14ac\x82W\x84\x87R\x86\x93\x90\x81\x15acBWP`\x01\x14ab\xFEW[Pab\xFC\x92P\x03\x83ab\x19V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10ac&WPP\x90` ab\xFC\x92\x82\x01\x01_ab\xEFV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92ac\rV[` \x93Pab\xFC\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_ab\xEFV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93ab\xD0V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10ae\xD0Wab\xFC\x94T\x91\x81\x81\x10ae\x9AW[\x81\x81\x10aedW[\x81\x81\x10ae.W[\x81\x81\x10ad\xF8W[\x81\x81\x10ad\xC2W[\x81\x81\x10ad\x8CW[\x81\x81\x10adWW[\x10ad*W[P\x03\x83ab\x19V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_ad\"V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01ad\x1CV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01ad\x14V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01ad\x0CV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01ad\x04V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01ac\xFCV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01ac\xF4V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01ac\xECV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91ac\xD4V[\x91\x90\x82\x01\x80\x92\x11abqWV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11aa\xECW`@Q\x91af\x94`\x1F\x82\x01`\x1F\x19\x16` \x01\x84ab\x19V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x05\x8BW\x82\x81` \x93\x84_\x96\x01^\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x05\x8BW\x81Qaf\xCA\x92` \x01afjV[\x90V[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x05\x8BWV[`@Q\x90af\xF0`@\x83ab\x19V[`\x0B\x82R\x7FMock Bridge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RV[`\x08T`\xFF\x16\x80\x15ag+W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a`\x1AW_\x91ag\xC3W[P\x15\x15\x90V[\x90P` \x81=` \x11ag\xEDW[\x81ag\xDE` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQ_ag\xBDV[=\x91Pag\xD1V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[_ab\xFC\x91ab\x19V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BWa$\xFA_\x91aj\xCB`@Q\x94\x85\x93\x84\x93\x7F\xF3 \xD9c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`@`\x04\x86\x01R`D\x85\x01\x90a`gV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`\x01`\x01`\xA0\x1B\x03\x90\x81`@Q\x93\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R\x16`\x04\x84\x01R\x16`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[\x81\x15ak\xEFW\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x90\x83\x83\x11amXW\x82\x81\x10\x91\x82\x15\x80amNW[amFWal?\x84\x86abdV[\x92`\x01\x84\x01\x80\x94\x11abqW`\x03\x83\x11\x15\x80am=W[am.W`\x03\x19\x83\x10\x15\x80am$W[am\x13W\x85\x83\x11\x15al\xCAWPP\x90al\x82\x84al\x87\x93abdV[ak\xE5V[\x90\x81\x15al\xC5Wal\x98\x92Paf]V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11abqW\x90V[PP\x90V[\x95\x94\x92\x91\x90\x95al\xDBW[PPPPV[\x83\x94\x95Pal\x82\x90al\xED\x93\x94abdV[\x90\x81\x15al\xC5Wal\xFE\x92PabdV[`\x01\x81\x01\x80\x91\x11abqW\x90_\x80\x80\x80al\xD5V[PP\x90Paf\xCA\x92\x91P\x19\x90abdV[P\x82\x19\x84\x11alfV[PP\x91\x90Paf\xCA\x92Paf]V[P\x82\x84\x11alVV[P\x92PPP\x90V[P\x84\x82\x11\x15al1V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R\xFD\xFE`\x80`@R4a\x03\x13W`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17a\x02&W`@\x90\x81R`\t\x82RhERC20Mock`\xB8\x1B` \x83\x01R\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17a\x02&W`@R`\x04\x81RcE20M`\xE0\x1B` \x82\x01R\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x02&W`\x03T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x03\tW[` \x82\x10\x14a\x02\x08W`\x1F\x81\x11a\x02\xA6W[P` \x92`\x1F\x82\x11`\x01\x14a\x02EW\x92\x81\x92\x93_\x92a\x02:W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x03U[\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x02&W`\x04T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x02\x1CW[` \x82\x10\x14a\x02\x08W`\x1F\x81\x11a\x01\xA5W[P` \x91`\x1F\x82\x11`\x01\x14a\x01EW\x91\x81\x92_\x92a\x01:W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x04U[`@Qa\t\x14\x90\x81a\x03\x18\x829\xF3[\x01Q\x90P_\x80a\x01\x16V[`\x1F\x19\x82\x16\x92`\x04_R\x80_ \x91_[\x85\x81\x10a\x01\x8DWP\x83`\x01\x95\x10a\x01uW[PPP\x81\x1B\x01`\x04Ua\x01+V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x01gV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x01UV[`\x04_R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B`\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x01\xFEW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x01\xF3WPa\0\xFDV[_\x81U`\x01\x01a\x01\xE6V[\x90\x91P\x81\x90a\x01\xDDV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x7F\x16\x90a\0\xEBV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x01Q\x90P_\x80a\0\xB5V[`\x1F\x19\x82\x16\x93`\x03_R\x80_ \x91_[\x86\x81\x10a\x02\x8EWP\x83`\x01\x95\x96\x10a\x02vW[PPP\x81\x1B\x01`\x03Ua\0\xCAV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02hV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x02UV[`\x03_R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[`\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x02\xFFW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x02\xF4WPa\0\x9BV[_\x81U`\x01\x01a\x02\xE7V[\x90\x91P\x81\x90a\x02\xDEV[\x90`\x7F\x16\x90a\0\x89V[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x06\xFD\xDE\x03\x14a\x07\x03WP\x80c\t^\xA7\xB3\x14a\x06tW\x80c\x18\x16\r\xDD\x14a\x06WW\x80c#\xB8r\xDD\x14a\x04\xE1W\x80c1<\xE5g\x14a\x04\xC6W\x80c@\xC1\x0F\x19\x14a\x03\xE5W\x80cp\xA0\x821\x14a\x03\xA1W\x80c\x95\xD8\x9BA\x14a\x02&W\x80c\x9D\xC2\x9F\xAC\x14a\x018W\x80c\xA9\x05\x9C\xBB\x14a\x01\x07Wc\xDDb\xED>\x14a\0\x95W_\x80\xFD[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\0\xAEa\x08\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\xCBa\x08'V[\x91\x16_R`\x01` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `@_ T`@Q\x90\x81R\xF3[_\x80\xFD[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x01-a\x01#a\x08\x04V[`$5\x903a\x08JV[` `@Q`\x01\x81R\xF3[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x01Qa\x08\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x91\x16\x80\x15a\x01\xFAW\x80_R_` R`@_ T\x82\x81\x10a\x01\xC8W` \x83_\x94\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93\x85\x87R\x86\x84R\x03`@\x86 U\x80`\x02T\x03`\x02U`@Q\x90\x81R\xA3\0[\x90\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`DR`d_\xFD[\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W`@Q_`\x04T\x90\x81`\x01\x1C`\x01\x83\x16\x92\x83\x15a\x03\x97W[` \x82\x10\x84\x14a\x03jW\x81\x85R\x84\x93\x90\x81\x15a\x03(WP`\x01\x14a\x02\xCCW[P\x03`\x1F\x01`\x1F\x19\x16\x81\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x81\x83\x10\x17a\x02\x9FWa\x02\x9B\x82\x91\x82`@R\x82a\x07\xDAV[\x03\x90\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x04_\x90\x81R\x91P\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B[\x81\x83\x10a\x03\x0CWPP\x81\x01` \x01`\x1F\x19a\x02mV[` \x91\x93P\x80`\x01\x91T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x91\x83\x92a\x02\xF6V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x85\x81\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91P`\x1F\x19\x90Pa\x02mV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x90`\x7F\x16\x90a\x02NV[4a\x01\x03W` `\x03\x196\x01\x12a\x01\x03Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x03\xCFa\x08\x04V[\x16_R_` R` `@_ T`@Q\x90\x81R\xF3[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x03\xFEa\x08\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`$5\x81\x15a\x04\x9AW`\x02T\x90\x80\x82\x01\x80\x92\x11a\x04mW` \x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91_\x93`\x02U\x84\x84R\x83\x82R`@\x84 \x81\x81T\x01\x90U`@Q\x90\x81R\xA3\0[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W` `@Q`\x12\x81R\xF3[4a\x01\x03W```\x03\x196\x01\x12a\x01\x03Wa\x04\xFAa\x08\x04V[a\x05\x02a\x08'V[`D5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x92\x83_R`\x01` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`@_ T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x10a\x05~W[Pa\x01-\x93Pa\x08JV[\x83\x81\x10a\x06#W\x84\x15a\x05\xF7W3\x15a\x05\xCBWa\x01-\x94_R`\x01` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R\x83`@_ \x91\x03\x90U\x84a\x05sV[\x7F\x94(\rb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\xE6\x02\xDF\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x83\x90\x7F\xFB\x8FA\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`DR`d_\xFD[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W` `\x02T`@Q\x90\x81R\xF3[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x06\x8Da\x08\x04V[`$5\x903\x15a\x05\xF7Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x05\xCBW3_R`\x01` R`@_ \x82_R` R\x80`@_ U`@Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` 3\x92\xA3` `@Q`\x01\x81R\xF3[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W_`\x03T\x90\x81`\x01\x1C`\x01\x83\x16\x92\x83\x15a\x07\xD0W[` \x82\x10\x84\x14a\x03jW\x81\x85R\x84\x93\x90\x81\x15a\x03(WP`\x01\x14a\x07tWP\x03`\x1F\x01`\x1F\x19\x16\x81\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x81\x83\x10\x17a\x02\x9FWa\x02\x9B\x82\x91\x82`@R\x82a\x07\xDAV[`\x03_\x90\x81R\x91P\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[[\x81\x83\x10a\x07\xB4WPP\x81\x01` \x01`\x1F\x19a\x02mV[` \x91\x93P\x80`\x01\x91T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x91\x83\x92a\x07\x9EV[\x90`\x7F\x16\x90a\x07(V[`\x1F\x19`\x1F` `@\x94\x81\x85R\x80Q\x91\x82\x91\x82\x82\x88\x01R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\x03WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\x03WV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x01\xFAWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x82\x15a\x04\x9AW\x81_R_` R`@_ T\x81\x81\x10a\x08\xE2W\x81\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92` \x92\x85_R_\x84R\x03`@_ U\x84_R_\x82R`@_ \x81\x81T\x01\x90U`@Q\x90\x81R\xA3V[\x82\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`DR`d_\xFD`\x80\x80`@R4`\x15Wa\x01\xDC\x90\x81a\0\x1A\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81ch\x13\xD7\x87\x14a\x01qW\x81c\xBC\xDB\x83\xD8\x14a\0\x80WPc\xD3\x07-\x82\x14a\0=W_\x80\xFD[4a\0|W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0|W` `\xFF_T\x16`@Q\x90\x15\x15\x81R\xF3[_\x80\xFD[4a\0|W``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0|W`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\0|W`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0|W6`#\x82\x01\x12\x15a\0|W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0|W6\x91\x01`$\x01\x11a\0|W`\xFF_T\x16a\x01\x15W\0[\x80\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x92R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FTarget bridge failed\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[4a\0|W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0|W`\x045\x80\x15\x15\x80\x91\x03a\0|W`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0_T\x16\x91\x16\x17_U_\x80\xF3`\x80`@R4a\x02\xA3W`@Q`\x1Fa\x19\x998\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x02oW\x80\x84\x92`\xA0\x94`@R\x839\x81\x01\x03\x12a\x02\xA3Wa\0M\x81a\x02\xA7V[a\0Y` \x83\x01a\x02\xA7V[\x91a\0f`@\x82\x01a\x02\xA7V[\x91`\x80``\x83\x01Q\x92\x01Q\x90`@Q\x94`@\x86\x01\x86\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\x02oW`@R`\x0B\x86RjMock Bridge`\xA8\x1B` \x87\x01R`\x01\x80U`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15a\x02\x92W[a\x02\x83Wa\0\xD6\x82a\0\xD0a\0\xDC\x94a\x02\xBBV[Pa\x031V[Pa\x03\xC4V[P\x83Q`\x01`\x01`@\x1B\x03\x81\x11a\x02oW`\x07T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x02eW[` \x82\x10\x14a\x02QW`\x1F\x81\x11a\x01\xEEW[P` \x94`\x1F\x82\x11`\x01\x14a\x01\x8BW\x94\x81\x92\x93\x94\x95_\x92a\x01\x80W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x07U[`\x02T\x91`\x03U`\x04U`\x01`\xA0\x1B\x91`\x01\x80`\xA0\x1B\x03\x16\x90`\x01\x80`\xA8\x1B\x03\x19\x16\x17\x17`\x02Ub\x01Q\x80B\x04`\x06U`@Qa\x14\xC1\x90\x81a\x04X\x829\xF3[\x01Q\x90P_\x80a\x01,V[`\x1F\x19\x82\x16\x95`\x07_R\x80_ \x91_[\x88\x81\x10a\x01\xD6WP\x83`\x01\x95\x96\x97\x98\x10a\x01\xBEW[PPP\x81\x1B\x01`\x07Ua\x01AV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x01\xB0V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x01\x9BV[`\x07_R\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88`\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x02GW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x02<WPa\x01\x10V[_\x81U`\x01\x01a\x02/V[\x90\x91P\x81\x90a\x02&V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x7F\x16\x90a\0\xFEV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\0\xBCV[_\x80\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xA3WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x19y_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x03,W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x19y_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a\x19\x19_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x19Y_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x03,W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x19Y_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7Fu\x1By]$\xB9.=\x92\xD1\xD0\xD8\xF2\x88_N\x9C\x9C&\x9D\xA3P\xAF6\xAEkI\x06\x9B\xAB\xF4\xBF\x90_Q` a\x19\x19_9_Q\x90_R\x90\x80\xA4`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x199_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x03,W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x199_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\xC0t\xBE\xB4k%\x1Fs\xC0\xFE\xC1n\xBAu\xD6\xBC\0B\xD0\x9E\xE1zt\x0E\xBB\xEE0\x8D\xBF\x87\xF8/\x90_Q` a\x19\x19_9_Q\x90_R\x90\x80\xA4`\x01\x90V\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x01\xFF\xC9\xA7\x14a\x0E\xDFWP\x80c\x103\xB4\xCC\x14a\x0E\xC2W\x80c\x11\x8C8\xC7\x14a\x0E\x88W\x80c\x12Y\xA5\xC8\x14a\x0EkW\x80c\x18\xB6\x8B\x8C\x14a\tQW\x80c$\x8A\x9C\xA3\x14a\t\x1FW\x80c//\xF1]\x14a\x08\xE2W\x80c4b\xFA\xC3\x14a\x08\xA8W\x80c6V\x8A\xBE\x14a\x08>W\x80c6\xB0\x89\xD8\x14a\x08!W\x80cZ\xB1\xD6\x1C\x14a\x07\x89W\x80ca\xB0\xA5n\x14a\x06~W\x80cc\"\x14\x90\x14a\x06]W\x80ce\xD7\xA3\xC9\x14a\x06BW\x80cg\xEE\xBA\x0C\x14a\x06%W\x80ch\x13\xD7\x87\x14a\x05\xD6W\x80ck\xCC\x8C\x14\x14a\x05\nW\x80c\x91\xD1HT\x14a\x04\xB4W\x80c\xA2\x17\xFD\xDF\x14a\x04\x9AW\x80c\xB1nxI\x14a\x04gW\x80c\xB2\r0\xA9\x14a\x04\x13W\x80c\xC9\xF5\xB6>\x14a\x03\xE0W\x80c\xCC=\xC0a\x14a\x02\xE0W\x80c\xD3\x07-\x82\x14a\x02\xBEW\x80c\xD5Gt\x1F\x14a\x02zW\x80c\xEA\xD9<\x8F\x14a\x02UW\x80c\xED\xE7\xCE\xBD\x14a\x01\xF4W\x80c\xF6\x81\xA8b\x14a\x01\xD7Wc\xFB\x8CKQ\x14a\x01[W_\x80\xFD[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W`\x05T`\x04T\x90\x80\x82\x11\x15a\x01\xC9W\x80\x82\x03\x91\x80\x83\x11a\x01\x9CW``\x92\x90[`@Q\x92\x83R` \x83\x01R`@\x82\x01R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[``\x91_\x90a\x01\x8AV[_\x80\xFD[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\tT`@Q\x90\x81R\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3Wa\x02)`\x02Ta\x02\x12a\x10\x82V[\x90`\xFF`@Q\x93\x84\x93``\x85R``\x85\x01\x90a\x11WV[\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16` \x85\x01R`\xA0\x1C\x16\x15\x15`@\x83\x01R\x03\x90\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\xFF`\x02T`\xA0\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x02\xBC`\x045a\x02\x99a\x0F\xA0V[\x90a\x02\xB7a\x02\xB2\x82_R_` R`\x01`@_ \x01T\x90V[a\x12/V[a\x13gV[\0[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\xFF`\x08T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W`@Q_`\x0BTa\x03\0\x81a\x0F\xC3V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x03\x9EWP`\x01\x14a\x03@W[a\x03<\x83a\x03(\x81\x85\x03\x82a\x10\x14V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x11WV[\x03\x90\xF3[\x91\x90P`\x0B_R\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x91_\x90[\x80\x82\x10a\x03\x84WP\x90\x91P\x81\x01` \x01a\x03(a\x03\x18V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x03lV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x03(\x90Pa\x03\x18V[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16`@Q\x90\x81R\xF3[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W\x7F |L\xBD\xF5^\xC3\x15\xA1?\r^\x04w2\xEC]\x94}\xA0V\xE7\x06Y:\xA5\t\x90\x99A\xCE\xDF`@`\x045a\x04Ra\x11\xA7V[`\x04T\x90\x80`\x04U\x82Q\x91\x82R` \x82\x01R\xA1\0[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\nT\x16`@Q\x90\x81R\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `@Q_\x81R\xF3[4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x04\xCDa\x0F\xA0V[`\x045_R_` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x058a\x0F}V[a\x05@a\x11\xA7V[\x16\x80\x15a\x05\xAEWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17`\x02U\x16\x7F\xB0\x7F\x8B\x1B\x85\x04-t\x02,\x86|\x83n\xDE\xB0\xBC\xD7\x0E\x13[\0B9\r+\x1F\xD1\x08)\x80i_\x80\xA3\0[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W`\x045\x80\x15\x15\x80\x91\x03a\x01\xD3W`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x08T\x16\x91\x16\x17`\x08U_\x80\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\x04T`@Q\x90\x81R\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3Wa\x03<a\x03(a\x10\x82V[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3Wa\x06va\x11\xA7V[`\x045`\x03U\0[4a\x01\xD3W```\x03\x196\x01\x12a\x01\xD3Wa\x06\x97a\x0F}V[`D5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x01\xD3W3_\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16\x15a\x07YW\x80\x15a\x05\xAEWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x02\xBC\x92`@Q\x92\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x85\x01R`$\x84\x01R`$5`D\x84\x01R`D\x83Ra\x07S`d\x84a\x10\x14V[\x16a\x14/V[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W`\x045\x80\x15\x15\x80\x91\x03a\x01\xD3W` \x7F\xB3A\x89\x89\xD0h5\xB5\xC2\x15\xEE\xBBMT\xEDk\xE7\xBB\xB6n\xB4\x80qdt\n.\x08/\xA7\x82\xD5\x91a\x07\xD3a\x11\xA7V[`\x02T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\xA0\x1B\x16\x91\x16\x17`\x02U`@Q\x90\x81R\xA1\0[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\x03T`@Q\x90\x81R\xF3[4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x08Wa\x0F\xA0V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\x08\x80Wa\x02\xBC\x90`\x045a\x13gV[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `@Q\x7F\xC0t\xBE\xB4k%\x1Fs\xC0\xFE\xC1n\xBAu\xD6\xBC\0B\xD0\x9E\xE1zt\x0E\xBB\xEE0\x8D\xBF\x87\xF8/\x81R\xF3[4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x02\xBC`\x045a\t\x01a\x0F\xA0V[\x90a\t\x1Aa\x02\xB2\x82_R_` R`\x01`@_ \x01T\x90V[a\x12\x95V[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W` a\tI`\x045_R_` R`\x01`@_ \x01T\x90V[`@Q\x90\x81R\xF3[4a\x01\xD3W```\x03\x196\x01\x12a\x01\xD3Wa\tja\x0F}V[`$5`D5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xD3W6`#\x84\x01\x12\x15a\x01\xD3W\x82`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xD3W6`$\x83\x86\x01\x01\x11a\x01\xD3W`\x02`\x01T\x14a\x0ECW`\x02`\x01U`\xFF`\x02T`\xA0\x1C\x16\x15a\x0E\x1BW3_\x90\x81R\x7F\xFE\x0C{\xD5\t,znF4\x88\xFA#\x9F2\x05\x8C\x87\xD4>3n.XJ'!2\xCF\x8D\xC9&` R`@\x90 T`\xFF\x16\x15a\r\xF3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x05\xAEW\x82\x15a\r\xCBW`\x03T\x83\x11a\rdWb\x01Q\x80B\x04`\x06T\x81\x11a\r\x8CW[Pa\nH\x83`\x05Ta\x11\x9AV[`\x04T\x10a\rdWa\n\xA3`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R3`$\x82\x01R0`D\x82\x01R\x84`d\x82\x01R`d\x81Ra\n\x9D`\x84\x82a\x10\x14V[\x83a\x14/V[`\xFF`\x08T\x16a\r\x06W\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\nT\x16\x17`\nU\x82`\tUa\n\xE6`\x0BTa\x0F\xC3V[`\x1F\x81\x11a\x0CeW[P_`\x1F\x82\x11`\x01\x14a\x0B\xA1W\x81\x92\x93\x94_\x92a\x0B\x93W[PP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17`\x0BU[a\x0BF\x82`\x05Ta\x11\x9AV[`\x05U\x7F=\xBA@\x1D\xC1\xAB\xBF\x01p\x86\x86K3\xBA\x81_S\xAB<\xDB\xB96k\xA6\xBC\xD8\xEC4-\xDD\x98\xE8` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16\x93`@Q\x90\x81R\xA3`\x01\x80U\0[`$\x92P\x01\x015\x84\x80a\x0B\x07V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x16\x94`\x0B_R\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x91_[\x87\x81\x10a\x0CJWP\x83`\x01\x95\x96\x97\x10a\x0C\x0FW[PPP\x81\x1B\x01`\x0BUa\x0B:V[\x01`$\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x84\x90\x1B`\xF8\x16\x1C\x19\x16\x90U\x84\x80\x80a\x0C\x01V[\x90\x92` `\x01\x81\x92`$\x87\x87\x01\x015\x81U\x01\x94\x01\x91\x01a\x0B\xEDV[`\x0B_R`\x1F\x82\x01`\x05\x1C\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x01\x90` \x83\x10a\x0C\xDEW[`\x1F\x01`\x05\x1C\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x01\x90[\x81\x81\x10a\x0C\xD3WPa\n\xEFV[_\x81U`\x01\x01a\x0C\xC6V[\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x91Pa\x0C\x9CV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FMock bridge execution failed\0\0\0\0`D\x82\x01R\xFD[\x7Fp\xD1h\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xDAN9\xDDV\xD7,.\xE3\xD12\xE0\x14k\xC3\x9E\x90^x\xE3\xBCd\xC4\x01\x90B\x1C{+\xCE\xF2\xAB`@`\x05T\x81Q\x90\x84\x82R` \x82\x01R\xA1_`\x05U`\x06U\x84a\n;V[\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\\B|\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F{\xEA \xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\x06T`@Q\x90\x81R\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `@Q\x7Fu\x1By]$\xB9.=\x92\xD1\xD0\xD8\xF2\x88_N\x9C\x9C&\x9D\xA3P\xAF6\xAEkI\x06\x9B\xAB\xF4\xBF\x81R\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\x05T`@Q\x90\x81R\xF3[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W`\x045\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x80\x92\x03a\x01\xD3W\x81\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x93\x14\x90\x81\x15a\x0FSW[P\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x83a\x0FLV[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xD3WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xD3WV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x10\nW[` \x83\x10\x14a\x0F\xDDWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x0F\xD2V[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10UW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q\x90_\x82`\x07T\x91a\x10\x95\x83a\x0F\xC3V[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a\x11\x1AWP`\x01\x14a\x10\xBBW[a\x10\xB9\x92P\x03\x83a\x10\x14V[V[P`\x07_\x90\x81R\x90\x91\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88[\x81\x83\x10a\x10\xFEWPP\x90` a\x10\xB9\x92\x82\x01\x01a\x10\xADV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x10\xE6V[` \x92Pa\x10\xB9\x94\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01a\x10\xADV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x91\x90\x82\x01\x80\x92\x11a\x01\x9CWV[3_\x90\x81R\x7F\xDF\xFF\x91\xBC\xEE\x88\xF6\xEA\0\xB2rom\x06*P\x9C2\x83W\x93G\x0CU\xBE\xD3\xBC\xC7L\x0F]7` R`@\x90 T`\xFF\x16\x15a\x11\xDFWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7Fu\x1By]$\xB9.=\x92\xD1\xD0\xD8\xF2\x88_N\x9C\x9C&\x9D\xA3P\xAF6\xAEkI\x06\x9B\xAB\xF4\xBF`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x12fWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x13aW\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x13aW\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V[\x90_` \x91\x82\x81Q\x91\x01\x82\x85Z\xF1\x15a\x14\xB6W_Q=a\x14\xADWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;\x15[a\x14kWPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[`\x01\x14\x15a\x14dV[`@Q=_\x82>=\x90\xFD/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\xFE\x0C{\xD5\t,znF4\x88\xFA#\x9F2\x05\x8C\x87\xD4>3n.XJ'!2\xCF\x8D\xC9&\xDF\xFF\x91\xBC\xEE\x88\xF6\xEA\0\xB2rom\x06*P\x9C2\x83W\x93G\x0CU\xBE\xD3\xBC\xC7L\x0F]7\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f905f3560e01c908163089ca9e014615ce157508063094f27a014615ada5780630a9254e4146158675780630f57280f146152745780631074a21f1461513d578063117e3b421461511857806313217f90146150f157806313a86f1a14614e065780631ed7831c14614d8857806323e1ebe714614ac7578063248ec32614614aa157806329365968146148a75780632ade3880146147225780632ed211831461457c5780633e5e3c23146144fe5780633f7286f414614480578063497b39181461418f5780634a61cf2914613e155780634f8632ba14613dee57806366d9a9a014613cb15780636f8cece414613b2657806385226c8114613a945780638b58cbae146137785780638f58a63f146136c2578063916a17c614613618578063956d980814613523578063959b337d146134fc578063a30ff4c214613242578063a3d4485b14613218578063a3fb171514612f60578063b0464fdc14612eb6578063b44dc9d614612b06578063b5508aa914612a74578063b55d42bc1461274c578063b9b5bd681461209e578063ba414fa614612079578063be6da53e14611ed6578063cffb048b14611ad9578063d3075c49146114e6578063d3b76bc914611376578063db9b708c146111ef578063dccc57f114610e8f578063e20c9f7114610e01578063e86b4fa714610ad5578063e9d3d5861461092f578063f3ed2b05146106b5578063f851a4401461068e578063f97e8467146102c1578063fa7626d41461029e578063fc0c546a146102785763fc9c8d391461024f575f80fd5b3461027557806003193601126102755760206001600160a01b0360235416604051908152f35b80fd5b503461027557806003193601126102755760206001600160a01b03815416604051908152f35b5034610275578060031936011261027557602060ff601f54166040519015158152f35b5034610275578060031936011261027557806001600160a01b03602054166001600160a01b03601f5460081c16813b1561068a5782916044839260405194859384927f40c10f19000000000000000000000000000000000000000000000000000000008452600484015269152d02c7e14af680000060248401525af1801561059757610675575b506024906001600160a01b036020541660206001600160a01b036022541691604051948580927f70a082310000000000000000000000000000000000000000000000000000000082528560048301525afa92831561059757829361063e575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610621576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610629575b506001600160a01b03601f5460081c166001600160a01b03602054166001600160a01b0360225416823b1561062557606484928360405195869485937f61b0a56e000000000000000000000000000000000000000000000000000000008552600485015269152d02c7e14af6800000602485015260448401525af180156105975761060c575b50506001600160a01b0360205416906001600160a01b0360225416604051907f70a082310000000000000000000000000000000000000000000000000000000082526004820152602081602481865afa9081156106015784916105cf575b5069152d02c7e14af680000082018092116105a25790610501916169f6565b60206001600160a01b03601f5460081c166024604051809481937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa801561059757829061055f575b61055c9150616980565b80f35b506020813d60201161058f575b8161057960209383616219565b8101031261058b5761055c9051610552565b5f80fd5b3d915061056c565b6040513d84823e3d90fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d6020116105f9575b816105ea60209383616219565b8101031261058b57515f6104e2565b3d91506105dd565b6040513d86823e3d90fd5b8161061691616219565b61062157815f610484565b5080fd5b8380fd5b8161063391616219565b61062157815f6103fe565b915091506020813d60201161066d575b8161065b60209383616219565b8101031261058b57829051915f6103a7565b3d915061064e565b8161067f91616219565b61027557805f610348565b5050fd5b503461027557806003193601126102755760206001600160a01b0360225416604051908152f35b5034610275578060031936011261027557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152600160048201526001602482015281604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761091a575b506001600160a01b03602154166001600160a01b0360255416604051917fb07f8b1b85042d74022c867c836edeb0bcd70e135b0042390d2b1fd1082980698480a36001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561068a5763ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610905575b506001600160a01b03601f5460081c166001600160a01b0360255416813b1561068a5782916024839260405194859384927f6bcc8c1400000000000000000000000000000000000000000000000000000000845260048401525af18015610597576108f0575b5050600460206001600160a01b03601f5460081c16604051928380927fc9f5b63e0000000000000000000000000000000000000000000000000000000082525afa80156105975782906108b5575b61055c91506001600160a01b036025541690616af1565b506020813d6020116108e8575b816108cf60209383616219565b81010312610621576108e361055c916166cd565b61089e565b3d91506108c2565b816108fa91616219565b61027557805f610850565b8161090f91616219565b61027557805f6107ea565b8161092491616219565b61027557805f61074b565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610abd575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f1f2a2005000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610aa8575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b84526004840152816024840152606060448401528160648401525af1801561059757610a975750f35b81610aa191616219565b6102755780f35b81610ab291616219565b61027557805f610a35565b81610ac791616219565b61027557805f6109a3565b50fd5b5034610275578060031936011261027557806001600160a01b0360245416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610dec575b50600460206001600160a01b03601f5460081c16604051928380927fa217fddf0000000000000000000000000000000000000000000000000000000082525afa908115610597578291610db7575b50610c1c6001600160a01b0360245416610c0e6040519384927fe2517d3f00000000000000000000000000000000000000000000000000000000602085015260248401602090939291936001600160a01b0360408201951681520152565b03601f198101835282616219565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad25781610c7791604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190616067565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610da2575b506001600160a01b03601f5460081c166001600160a01b03602054166001600160a01b0360225416823b15610d9d57606484928360405195869485937f61b0a56e00000000000000000000000000000000000000000000000000000000855260048501526103e8602485015260448401525af1801561059757610d88575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610a975750f35b81610d9291616219565b61027557805f610d1a565b505050fd5b81610dac91616219565b61027557805f610c9c565b9150506020813d602011610de4575b81610dd360209383616219565b8101031261058b578190515f610bb0565b3d9150610dc6565b81610df691616219565b61027557805f610b62565b503461027557806003193601126102755760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b818110610e7057610e6c85610e6081870382616219565b60405191829182616025565b0390f35b82546001600160a01b0316845260209093019260019283019201610e49565b50346102755780600319360112610275576001600160a01b03601f5460081c166040517fa217fddf000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156111045783916111bd575b506022546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b03166024820181905290602081604481865afa8015610601578490611182575b610f4f9150616b73565b6040517f118c38c7000000000000000000000000000000000000000000000000000000008152602081600481865afa90811561060157849161114e575b506040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101919091526001600160a01b03909116602482015260208180604481015b0381855afa801561110457839061110f575b610ff09150616b73565b6040517f3462fac3000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156111045783916110d0575b506023546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529060209082908180604481015b03915afa8015610597578290611095575b61055c9150616b73565b506020813d6020116110c8575b816110af60209383616219565b81010312610621576110c361055c9161623c565b61108b565b3d91506110a2565b90506020813d6020116110fc575b816110eb60209383616219565b8101031261058b575161107a61102d565b3d91506110de565b6040513d85823e3d90fd5b506020813d602011611146575b8161112960209383616219565b810103126111425761113d610ff09161623c565b610fe6565b8280fd5b3d915061111c565b90506020813d60201161117a575b8161116960209383616219565b8101031261058b5751610fd4610f8c565b3d915061115c565b506020813d6020116111b5575b8161119c60209383616219565b81010312610625576111b0610f4f9161623c565b610f45565b3d915061118f565b90506020813d6020116111e7575b816111d860209383616219565b8101031261058b57515f610eec565b3d91506111cb565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757611361575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761134c575b506001600160a01b03601f5460081c16803b15610ad25781809160846040518094819363062da2e360e21b83528160048401526103e86024840152606060448401528160648401525af1801561059757610a975750f35b8161135691616219565b61027557805f6112f5565b8161136b91616219565b61027557805f611263565b5034610275578060031936011261027557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576114d1575b50506001600160a01b03602254166001600160a01b03602154169060405191611999918284019284841067ffffffffffffffff8511176114a4579160a093918593617bff85398252856020830152604082015269d3c21bcecceda100000060608201526a0422ca8b0a00a425000000608082015203019082f0156114985780f35b604051903d90823e3d90fd5b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b816114db91616219565b61027557805f611417565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757611ac4575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526a01a784379d99db4200000060248401525af1801561059757611a8d575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757611a78575b506001600160a01b03601f5460081c166001600160a01b036020541690803b1561068a5760405163062da2e360e21b81526001600160a01b0392909216600483015269d3c21bcecceda10000006024830152606060448301525f606483015282908290608490829084905af1801561059757611a63575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa8015610597578290611a2f575b6116fa91506167f5565b620151804201804211611a02578190737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907fe5d6bf020000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576119ed575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f491cc7c20000000000000000000000000000000000000000000000000000000081528181806117e360048201906001606060808401935f81525f60208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576119d8575b50507fda4e39dd56d72c2ee3d132e0146bc39e905e78e3bc64c40190421c7b2bcef2ab60408051620151804204815269d3c21bcecceda10000006020820152a1806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576119c3575b506001600160a01b03601f5460081c166001600160a01b036020541690803b1561068a5760405163062da2e360e21b81526001600160a01b0392909216600483015269d3c21bcecceda10000006024830152606060448301525f606483015282908290608490829084905af18015610597576119ae575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa801561059757829061197a575b61055c91506167f5565b506020813d6020116119a6575b8161199460209383616219565b8101031261058b5761055c9051611970565b3d9150611987565b816119b891616219565b61027557805f611922565b816119cd91616219565b61027557805f6118ab565b816119e291616219565b61027557805f611808565b816119f791616219565b61027557805f611778565b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b506020813d602011611a5b575b81611a4960209383616219565b8101031261058b576116fa90516116f0565b3d9150611a3c565b81611a6d91616219565b61027557805f6116a2565b81611a8291616219565b61027557805f61162b565b6020813d602011611abc575b81611aa660209383616219565b8101031261062157611ab79061623c565b6115c7565b3d9150611a99565b81611ace91616219565b61027557805f61155a565b503461027557602060031936011261027557611b0369d3c21bcecceda10000006001600435616c1c565b60409082808351611b148582616219565b600c81527f426f756e6420726573756c74000000000000000000000000000000000000000060208201528451611b9381611b7f60208201947fb60e72cc0000000000000000000000000000000000000000000000000000000086528960248401526064830190616067565b87604483015203601f198101835282616219565b51906a636f6e736f6c652e6c6f675afa50826001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156106215783519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015611e5e57611ec1575b505060208054601f5484517f095ea7b30000000000000000000000000000000000000000000000000000000081526001600160a01b0360089290921c821660048201526024810185905292918391168187816044810103925af18015611eb457611e7d575b50826001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156106215783519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015611e5e57611e68575b506001600160a01b03601f5460081c166001600160a01b0360205416813b15611142578291608483928751948593849263062da2e360e21b84526004840152886024840152606060448401528160648401525af18015611e5e57611e49575b50506001600160a01b03601f5460081c169180517ff681a862000000000000000000000000000000000000000000000000000000008152602081600481875afa908115611e3f578591611e0c575b50600493611d8b846020936169f6565b8251948580927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa908115611e0357508390611dcf575b61055c92506169f6565b506020823d602011611dfb575b81611de960209383616219565b8101031261058b5761055c9151611dc5565b3d9150611ddc565b513d85823e3d90fd5b90506020813d602011611e37575b81611e2760209383616219565b8101031261058b57516004611d7b565b3d9150611e1a565b82513d87823e3d90fd5b81611e5391616219565b61114257825f611d2d565b84513d84823e3d90fd5b81611e7291616219565b61114257825f611cce565b6020813d602011611eac575b81611e9660209383616219565b8101031261062557611ea79061623c565b611c6b565b3d9150611e89565b50505051903d90823e3d90fd5b81611ecb91616219565b61114257825f611c06565b5034610275578060031936011261027557806001600160a01b0360245416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757612064575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f5c427cd9000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761204f575b506001600160a01b03601f5460081c166001600160a01b036020541690803b1561068a5760405163062da2e360e21b81526001600160a01b0390921660048301526103e86024830152606060448301525f606483015282908290818381608481015b03925af1801561059757610a975750f35b8161205991616219565b61027557805f611fdc565b8161206e91616219565b61027557805f611f4a565b5034610275578060031936011261027557602061209461671c565b6040519015158152f35b5034610275578060031936011261027557604051602080820152600960408201527f74657374206461746100000000000000000000000000000000000000000000006060820152606081526120f4608082616219565b816001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610621576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757612737575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526969e10de76676d080000060248401525af1801561110457612700575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561062157816040517f491cc7c2000000000000000000000000000000000000000000000000000000008152600160048201528160248201526001604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576126eb575b50506001600160a01b03602054166001600160a01b0360215416907f3dba401dc1abbf017086864b33ba815f53ab3cdbb9366ba6bcd8ec342ddd98e860206040516969e10de76676d08000008152a3816001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610621576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576126d6575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561114257829060405192839163062da2e360e21b835260048301526969e10de76676d080000060248301526060604483015281838161235c606482018a616067565b03925af18015610597576126c1575b50506001600160a01b03601f5460081c166040517f1033b4cc000000000000000000000000000000000000000000000000000000008152602081600481855afa801561060157849061268d575b6123c29150616900565b6040517ff681a862000000000000000000000000000000000000000000000000000000008152602081600481855afa8015610601578490612659575b6124089150616900565b82604051927fb16e7849000000000000000000000000000000000000000000000000000000008452602084600481865afa93841561059757829461261d575b5061245e6001600160a01b03602054168095616af1565b6040517fcc3dc0610000000000000000000000000000000000000000000000000000000081528281600481875afa9081156111045783916125c8575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611142576124fa839161250c60405194859384937f97624631000000000000000000000000000000000000000000000000000000008552604060048601526044850190616067565b90600319848303016024850152616067565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610597576125b3575b50506020906024604051809481937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa801561059757829061257f575b61055c9150616900565b506020813d6020116125ab575b8161259960209383616219565b8101031261058b5761055c9051612575565b3d915061258c565b816125bd91616219565b61114257825f612530565b90503d8084833e6125d98183616219565b8101906020818303126106255780519067ffffffffffffffff821161261957019080601f830112156106255781516126139260200161666a565b5f61249a565b8480fd5b9093506020813d602011612651575b8161263960209383616219565b810103126106215761264a906166cd565b925f612447565b3d915061262c565b506020813d602011612685575b8161267360209383616219565b8101031261058b5761240890516123fe565b3d9150612666565b506020813d6020116126b9575b816126a760209383616219565b8101031261058b576123c290516123b8565b3d915061269a565b816126cb91616219565b61062157815f61236b565b816126e091616219565b61062157815f6122fb565b816126f591616219565b61062157815f612249565b6020813d60201161272f575b8161271960209383616219565b810103126111425761272a9061623c565b6121c3565b3d915061270c565b8161274191616219565b61062157815f612157565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757612a5f575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b3000000000000000000000000000000000000000000000000000000008452600484015269152d02c7e14af680000060248401525af1801561059757612a28575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757612a13575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b8452600484015269152d02c7e14af68000006024840152606060448401528160648401525af18015610597576129fe575b50600460206001600160a01b03601f5460081c16604051928380927ff681a8620000000000000000000000000000000000000000000000000000000082525afa9081156105975782916129c9575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f98296c54000000000000000000000000000000000000000000000000000000008252600482015269152d02c7e14af680000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561059757610a975750f35b9150506020813d6020116129f6575b816129e560209383616219565b8101031261058b578190515f612948565b3d91506129d8565b81612a0891616219565b61027557805f6128fa565b81612a1d91616219565b61027557805f612890565b6020813d602011612a57575b81612a4160209383616219565b8101031261062157612a529061623c565b61282c565b3d9150612a34565b81612a6991616219565b61027557805f6127c0565b5034610275578060031936011261027557601954612a918161629e565b91612a9f6040519384616219565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b838310612ae95760405160208082528190610e6c9082018861608c565b600160208192612af8856162b6565b815201920192019190612acc565b50346102755780600319360112610275576001600160a01b03601f5460081c166040517fc9f5b63e000000000000000000000000000000000000000000000000000000008152602081600481855afa8015611104578390612e7b575b612b7991506001600160a01b036021541690616af1565b6040517f65d7a3c90000000000000000000000000000000000000000000000000000000081528281600481855afa8015611104578390612e39575b612bc79150612bc16166e1565b90616a6c565b6040517f36b089d8000000000000000000000000000000000000000000000000000000008152602081600481855afa8015611104578390612e05575b612c0d91506167f5565b6040517f67eeba0c000000000000000000000000000000000000000000000000000000008152602081600481855afa8015611104578390612dd1575b612c53915061687f565b6040517fead93c8f000000000000000000000000000000000000000000000000000000008152602081600481855afa8015611104578390612d96575b612c999150616b73565b6040517f1033b4cc000000000000000000000000000000000000000000000000000000008152602081600481855afa908115611104578391612d63575b50600491612ce5602092616980565b604051928380927f1259a5c80000000000000000000000000000000000000000000000000000000082525afa8015610597578290612d2f575b61055c9150620151804204906169f6565b506020813d602011612d5b575b81612d4960209383616219565b8101031261058b5761055c9051612d1e565b3d9150612d3c565b90506020813d602011612d8e575b81612d7e60209383616219565b8101031261058b57516004612cd6565b3d9150612d71565b506020813d602011612dc9575b81612db060209383616219565b8101031261114257612dc4612c999161623c565b612c8f565b3d9150612da3565b506020813d602011612dfd575b81612deb60209383616219565b8101031261058b57612c539051612c49565b3d9150612dde565b506020813d602011612e31575b81612e1f60209383616219565b8101031261058b57612c0d9051612c03565b3d9150612e12565b503d8084833e612e498183616219565b8101906020818303126106255780519167ffffffffffffffff831161261957612bc792612e7692016166b0565b612bb4565b506020813d602011612eae575b81612e9560209383616219565b8101031261114257612ea9612b79916166cd565b612b62565b3d9150612e88565b5034610275578060031936011261027557601c54612ed38161629e565b91612ee16040519384616219565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310612f235760405180610e6c8782616139565b60026020600192604051612f36816161d0565b6001600160a01b038654168152612f4e8587016163b9565b83820152815201920192019190612f0e565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757613203575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b3000000000000000000000000000000000000000000000000000000008452600484015269d3c21bcecceda100000160248401525af18015610597576131cc575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576131b7575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f70d168bc000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576131a2575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b8452600484015269d3c21bcecceda10000016024840152606060448401528160648401525af1801561059757610a975750f35b816131ac91616219565b61027557805f613136565b816131c191616219565b61027557805f6130a4565b6020813d6020116131fb575b816131e560209383616219565b81010312610621576131f69061623c565b613040565b3d91506131d8565b8161320d91616219565b61027557805f612fd4565b503461027557806003193601126102755760206001600160a01b03601f5460081c16604051908152f35b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576134e7575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526a0422ca8b0a00a42500000160248401525af18015610597576134b0575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761349b575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f70d168bc000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757613486575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b845260048401526a0422ca8b0a00a4250000016024840152606060448401528160648401525af1801561059757610a975750f35b8161349091616219565b61027557805f613419565b816134a591616219565b61027557805f613387565b6020813d6020116134df575b816134c960209383616219565b81010312610621576134da9061623c565b613323565b3d91506134bc565b816134f191616219565b61027557805f6132b6565b503461027557806003193601126102755760206001600160a01b0360215416604051908152f35b50346102755780600319360112610275576004816001600160a01b03601f5460081c16604051928380927fede7cebd0000000000000000000000000000000000000000000000000000000082525afa80156105975782839284926135ad575b5061055c926135966135a892612bc16166e1565b6001600160a01b036021541690616af1565b616b73565b925050503d8083833e6135c08183616219565b8101906060818303126111425780519167ffffffffffffffff8311610625576135f16135a89161055c9484016166b0565b61359661360c6040613605602087016166cd565b950161623c565b93949192506135829050565b5034610275578060031936011261027557601d546136358161629e565b916136436040519384616219565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b8383106136855760405180610e6c8782616139565b60026020600192604051613698816161d0565b6001600160a01b0386541681526136b08587016163b9565b83820152815201920192019190613670565b5034610275578060031936011261027557600460606001600160a01b03601f5460081c16604051928380927ffb8c4b510000000000000000000000000000000000000000000000000000000082525afa9081156105975761055c91838490859261373b575b61373692935061373690616980565b61687f565b5050506137366137656137369260603d606011613771575b61375d8183616219565b810190616249565b91935090915082613727565b503d613753565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757613a7f575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526969e10de76676d080000060248401525af1801561059757613a48575b50806001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b84526004840152693f870857a3e0e38000006024840152606060448401528160648401525af1801561059757613a33575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b84526004840152692a5a058fc295ed0000006024840152606060448401528160648401525af1801561059757613a1e575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757613a09575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa801561059757829061257f5761055c9150616900565b81613a1391616219565b61027557805f6139b2565b81613a2891616219565b61027557805f613946565b81613a3d91616219565b61027557805f6138dc565b6020813d602011613a77575b81613a6160209383616219565b8101031261062157613a729061623c565b613871565b3d9150613a54565b81613a8991616219565b61027557805f613805565b5034610275578060031936011261027557601a54613ab18161629e565b91613abf6040519384616219565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b838310613b095760405160208082528190610e6c9082018861608c565b600160208192613b18856162b6565b815201920192019190613aec565b5034610275578060031936011261027557806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757613c9c575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757613c87575b506001600160a01b03601f5460081c16803b15610ad2578180916024604051809481937f6bcc8c140000000000000000000000000000000000000000000000000000000083528160048401525af1801561059757610a975750f35b81613c9191616219565b61027557805f613c2c565b81613ca691616219565b61027557805f613b9a565b5034610275578060031936011261027557601b54613cce8161629e565b613cdb6040519182616219565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b838310613db357868587604051928392602084019060208552518091526040840160408260051b8601019392905b828210613d4857505050500390f35b91936020613da3827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0600195979984950301865288519083613d938351604084526040840190616067565b92015190848184039101526160e4565b9601920192018594939192613d39565b60026020600192604051613dc6816161d0565b613dcf866162b6565b8152613ddc8587016163b9565b83820152815201920192019190613d0b565b503461027557806003193601126102755760206001600160a01b0360245416604051908152f35b5034610275578060031936011261027557806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761417a575b506001600160a01b03601f5460081c16803b15610ad2578180916024604051809481937f5ab1d61c0000000000000000000000000000000000000000000000000000000083528160048401525af1801561059757614165575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757614150575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526103e860248401525af1801561059757614119575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757614104575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f7bea20b2000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761204f57506001600160a01b03601f5460081c166001600160a01b036020541690803b1561068a5760405163062da2e360e21b81526001600160a01b0390921660048301526103e86024830152606060448301525f6064830152829082908183816084810161203e565b8161410e91616219565b61027557805f61400d565b6020813d602011614148575b8161413260209383616219565b81010312610621576141439061623c565b613fa9565b3d9150614125565b8161415a91616219565b61027557805f613f45565b8161416f91616219565b61027557805f613ee2565b8161418491616219565b61027557805f613e89565b5034610275578060031936011261027557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f491cc7c200000000000000000000000000000000000000000000000000000000815281818061420960048201906001606060808401935f81525f60208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761446b575b50507f207c4cbdf55ec315a13f0d5e047732ec5d947da056e706593aa509909941cedf604080516a0422ca8b0a00a42500000081526a084595161401484a0000006020820152a1806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757614456575b506001600160a01b03601f5460081c16803b15610ad2578180916024604051809481937fb20d30a90000000000000000000000000000000000000000000000000000000083526a084595161401484a00000060048401525af1801561059757614441575b50600460206001600160a01b03601f5460081c16604051928380927f67eeba0c0000000000000000000000000000000000000000000000000000000082525afa90811561059757829161440c575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526a084595161401484a00000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561059757610a975750f35b9150506020813d602011614439575b8161442860209383616219565b8101031261058b578190515f61438a565b3d915061441b565b8161444b91616219565b61027557805f61433c565b8161446091616219565b61027557805f6142d8565b8161447591616219565b61027557805f61422e565b503461027557806003193601126102755760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b8181106144df57610e6c85610e6081870382616219565b82546001600160a01b03168452602090930192600192830192016144c8565b503461027557806003193601126102755760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b81811061455d57610e6c85610e6081870382616219565b82546001600160a01b0316845260209093019260019283019201614546565b5034610275578060031936011261027557806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761470d575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576146f8575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a5782916064839260405194859384927f61b0a56e00000000000000000000000000000000000000000000000000000000845260048401526103e860248401528160448401525af1801561059757610a975750f35b8161470291616219565b61027557805f614682565b8161471791616219565b61027557805f6145f0565b5034610275578060031936011261027557601e5461473f8161629e565b61474c6040519182616219565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b83831061481e57868587604051928392602084019060208552518091526040840160408260051b8601019392905b8282106147b957505050500390f35b9193602061480e827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc060019597998495030186526040838a516001600160a01b0381511684520151918185820152019061608c565b96019201920185949391926147aa565b60405161482a816161d0565b6001600160a01b0383541681526001830180546148468161629e565b916148546040519384616219565b8183528a526020808b20908b9084015b83821061488a57505050506001928260209283600295015281520192019201919061477c565b600160208192614899866162b6565b815201930191019091614864565b5034610275576020600319360112610275576148d76fffffffffffffffffffffffffffffffff6001600435616c1c565b604090828083516148e88582616219565b600c81527f426f756e6420726573756c7400000000000000000000000000000000000000006020820152845161495381611b7f60208201947fb60e72cc0000000000000000000000000000000000000000000000000000000086528960248401526064830190616067565b51906a636f6e736f6c652e6c6f675afa50826001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156106215783519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015611e5e57614a8c575b506001600160a01b03601f5460081c16803b156106215781809160248651809481937fb20d30a90000000000000000000000000000000000000000000000000000000083528860048401525af18015611e5e57614a77575b505060049160206001600160a01b03601f5460081c168251948580927f67eeba0c0000000000000000000000000000000000000000000000000000000082525afa908115611e0357508390611dcf5761055c92506169f6565b81614a8191616219565b61114257825f614a1e565b81614a9691616219565b61114257825f6149c6565b503461027557806003193601126102755760206040516a0422ca8b0a00a4250000008152f35b5034610275578060031936011261027557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152818180614b4160048201906001606060808401935f81525f60208201525f60408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757614d73575b50507fb3418989d06835b5c215eebb4d54ed6be7bbb66eb4807164740a2e082fa782d56020604051838152a1806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757614d5e575b506001600160a01b03601f5460081c16803b15610ad2578180916024604051809481937f5ab1d61c0000000000000000000000000000000000000000000000000000000083528160048401525af1801561059757614d49575b50600460206001600160a01b03601f5460081c16604051928380927fead93c8f0000000000000000000000000000000000000000000000000000000082525afa908115610597578291614d0f575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907fa5982885000000000000000000000000000000000000000000000000000000008252151560048201528181602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561059757610a975750f35b90506020813d602011614d41575b81614d2a60209383616219565b81010312610ad257614d3b9061623c565b5f614c9c565b3d9150614d1d565b81614d5391616219565b61027557805f614c4e565b81614d6891616219565b61027557805f614bf5565b81614d7d91616219565b61027557805f614b66565b503461027557806003193601126102755760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b818110614de757610e6c85610e6081870382616219565b82546001600160a01b0316845260209093019260019283019201614dd0565b5034610275578060031936011261027557806001600160a01b0360245416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576150dc575b50600460206001600160a01b03601f5460081c16604051928380927f118c38c70000000000000000000000000000000000000000000000000000000082525afa9081156105975782916150a7575b50614f3f6001600160a01b0360245416610c0e6040519384927fe2517d3f00000000000000000000000000000000000000000000000000000000602085015260248401602090939291936001600160a01b0360408201951681520152565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad25781614f9a91604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190616067565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757615092575b506001600160a01b03601f5460081c166001600160a01b0360255416813b1561068a5782916024839260405194859384927f6bcc8c1400000000000000000000000000000000000000000000000000000000845260048401525af1801561059757610d88575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757610a975750f35b8161509c91616219565b61027557805f614fbf565b9150506020813d6020116150d4575b816150c360209383616219565b8101031261058b578190515f614ee1565b3d91506150b6565b816150e691616219565b61027557805f614e93565b503461027557806003193601126102755760206001600160a01b0360255416604051908152f35b5034610275578060031936011261027557602060405169d3c21bcecceda10000008152f35b5034610275578060031936011261027557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761525f575b50506001600160a01b03602354166001600160a01b03602154169060405191611999918284019284841067ffffffffffffffff8511176114a4579160a093918593617bff85398683526020830152604082015269d3c21bcecceda100000060608201526a0422ca8b0a00a425000000608082015203019082f0156114985780f35b8161526991616219565b61027557805f6151de565b5034610275578060031936011261027557806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757615852575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526a0422ca8b0a00a42500000060248401525af180156105975761581b575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757615806575b505b600a811061577d5750737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757615768575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa8015610597578290615734575b61549f915061687f565b806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156105975761571f575b5060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b30000000000000000000000000000000000000000000000000000000084526004840152600160248401525af18015610597576156e8575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576156d3575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561027557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f70d168bc000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610597576156be575b506001600160a01b03601f5460081c166001600160a01b0360205416813b1561068a57829160848392604051948593849263062da2e360e21b8452600484015260016024840152606060448401528160648401525af1801561059757610a975750f35b816156c891616219565b61027557805f61565b565b816156dd91616219565b61027557805f6155c9565b6020813d602011615717575b8161570160209383616219565b81010312610621576157129061623c565b615565565b3d91506156f4565b8161572991616219565b61027557805f615502565b506020813d602011615760575b8161574e60209383616219565b8101031261058b5761549f9051615495565b3d9150615741565b8161577291616219565b61027557805f615447565b816001600160a01b03601f5460081c166001600160a01b0360205416813b1561114257829160848392604051948593849263062da2e360e21b845260048401526969e10de76676d08000006024840152606060448401528160648401525af18015610597576157f1575b50506001016153d4565b816157fb91616219565b61062157815f6157e7565b8161581091616219565b61027557805f6153d2565b6020813d60201161584a575b8161583460209383616219565b81010312610621576158459061623c565b615355565b3d9150615827565b8161585c91616219565b61027557805f6152e8565b5034610275578060031936011261027557604051610c2c8082019082821067ffffffffffffffff831117615aad57908291616ddd8339039082f08015615a73576001600160a01b03167fffffffffffffffffffffffff000000000000000000000000000000000000000060205416176020556040516101f68082019082821067ffffffffffffffff831117615aad57908291617a098339039082f08015615a73576001600160a01b0316807fffffffffffffffffffffffff000000000000000000000000000000000000000060215416176021556001600160a01b03602254166001600160a01b03602354169160405192611999928385019385851067ffffffffffffffff861117615a80579185939160a09593617bff863983526020830152604082015269d3c21bcecceda100000060608201526a0422ca8b0a00a425000000608082015203019082f08015615a73577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f55806001600160a01b03602054166001600160a01b0360235416813b1561068a5782916044839260405194859384927f40c10f1900000000000000000000000000000000000000000000000000000000845260048401526a084595161401484a00000060248401525af1801561059757610a975750f35b50604051903d90823e3d90fd5b6024877f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b5034610275578060031936011261027557806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757615ccc575b506001600160a01b03601f5460081c16803b15610ad2578180916024604051809481937f632214900000000000000000000000000000000000000000000000000000000083526a01a784379d99db4200000060048401525af1801561059757615cb7575b50600460206001600160a01b03601f5460081c16604051928380927f36b089d80000000000000000000000000000000000000000000000000000000082525afa908115610597578291615c82575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526a01a784379d99db4200000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561059757610a975750f35b9150506020813d602011615caf575b81615c9e60209383616219565b8101031261058b578190515f615c00565b3d9150615c91565b81615cc191616219565b61027557805f615bb2565b81615cd691616219565b61027557805f615b4e565b90503461058b575f60031936011261058b576001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b5763ca669fa760e01b825260048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561601a57616007575b508060206001600160a01b0381541660446001600160a01b03601f5460081c1660405194859384927f095ea7b3000000000000000000000000000000000000000000000000000000008452600484015269d3c21bcecceda100000060248401525af1801561059757615fd0575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad2576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561059757615fbb575b506001600160a01b03601f5460081c166001600160a01b036020541690803b1561068a5760405163062da2e360e21b81526001600160a01b0392909216600483015269d3c21bcecceda10000006024830152606060448301525f606483015282908290608490829084905af1801561059757615fa6575b50600460606001600160a01b03601f5460081c16604051928380927ffb8c4b510000000000000000000000000000000000000000000000000000000082525afa8015610597578283908492615f79575b615ef8929350613736906167f5565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ad257604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526a034f086f3b33b68400000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561059757610a975750f35b505050615ef8615f9a6137369260603d6060116137715761375d8183616219565b91935090915082615ee9565b81615fb091616219565b61027557805f615e99565b81615fc591616219565b61027557805f615e22565b6020813d602011615fff575b81615fe960209383616219565b8101031261062157615ffa9061623c565b615dbe565b3d9150615fdc565b61601391505f90616219565b5f5f615d51565b6040513d5f823e3d90fd5b60206040818301928281528451809452019201905f5b8181106160485750505090565b82516001600160a01b031684526020938401939092019160010161603b565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b9080602083519182815201916020808360051b8301019401925f915b8383106160b757505050505090565b90919293946020806160d583601f1986600196030187528951616067565b970193019301919392906160a8565b90602080835192838152019201905f5b8181106161015750505090565b82517fffffffff00000000000000000000000000000000000000000000000000000000168452602093840193909201916001016160f4565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061616b57505050505090565b90919293946020806161c1837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b038151168452015191818582015201906160e4565b9701930193019193929061615c565b6040810190811067ffffffffffffffff8211176161ec57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f601f19910116810190811067ffffffffffffffff8211176161ec57604052565b5190811515820361058b57565b9081606091031261058b578051916040602083015192015190565b9190820391821161627157565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b67ffffffffffffffff81116161ec5760051b60200190565b90604051915f8154908160011c92600183169283156163af575b60208510841461638257848752869390811561634257506001146162fe575b506162fc92500383616219565b565b90505f9291925260205f20905f915b8183106163265750509060206162fc928201015f6162ef565b602091935080600191548385890101520191019091849261630d565b602093506162fc9592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f6162ef565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f16936162d0565b90604051918281549182825260208201905f5260205f20925f905b8060078301106165d0576162fc94549181811061659a575b818110616564575b81811061652e575b8181106164f8575b8181106164c2575b81811061648c575b818110616457575b1061642a575b500383616219565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f616422565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b16815201930161641c565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301616414565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b16815201930161640c565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301616404565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b1681520193016163fc565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b1681520193016163f4565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b1681520193016163ec565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e08201520194019201859293916163d4565b9190820180921161627157565b92919267ffffffffffffffff82116161ec5760405191616694601f8201601f191660200184616219565b82948184528183011161058b578281602093845f96015e010152565b9080601f8301121561058b5781516166ca9260200161666a565b90565b51906001600160a01b038216820361058b57565b604051906166f0604083616219565b600b82527f4d6f636b204272696467650000000000000000000000000000000000000000006020830152565b60085460ff16801561672b5790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561601a575f916167c3575b50151590565b90506020813d6020116167ed575b816167de60209383616219565b8101031261058b57515f6167bd565b3d91506167d1565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b57604051907f98296c54000000000000000000000000000000000000000000000000000000008252600482015269d3c21bcecceda100000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b5f6162fc91616219565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526a0422ca8b0a00a42500000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526969e10de76676d080000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b57604051917f98296c54000000000000000000000000000000000000000000000000000000008352600483015260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b576124fa5f91616acb60405194859384937ff320d963000000000000000000000000000000000000000000000000000000008552604060048601526044850190616067565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b576001600160a01b039081604051937f515361f60000000000000000000000000000000000000000000000000000000085521660048401521660248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058b57604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561601a576168755750565b8115616bef570690565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f90838311616d585782811091821580616d4e575b616d4657616c3f8486616264565b926001840180941161627157600383111580616d3d575b616d2e5760031983101580616d24575b616d135785831115616cca57505090616c8284616c8793616264565b616be5565b908115616cc557616c98925061665d565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116162715790565b505090565b959492919095616cdb575b50505050565b83949550616c8290616ced9394616264565b908115616cc557616cfe9250616264565b6001810180911161627157905f808080616cd5565b505090506166ca9291501990616264565b5082198411616c66565b50509190506166ca925061665d565b50828411616c56565b509250505090565b5084821115616c31565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603e60248201527f5374645574696c7320626f756e642875696e743235362c75696e743235362c7560448201527f696e74323536293a204d6178206973206c657373207468616e206d696e2e00006064820152fdfe60806040523461031357604080519081016001600160401b03811182821017610226576040908152600982526845524332304d6f636b60b81b602083015280519081016001600160401b038111828210176102265760405260048152634532304d60e01b602082015281516001600160401b03811161022657600354600181811c91168015610309575b602082101461020857601f81116102a6575b50602092601f821160011461024557928192935f9261023a575b50508160011b915f199060031b1c1916176003555b80516001600160401b03811161022657600454600181811c9116801561021c575b602082101461020857601f81116101a5575b50602091601f8211600114610145579181925f9261013a575b50508160011b915f199060031b1c1916176004555b60405161091490816103188239f35b015190505f80610116565b601f1982169260045f52805f20915f5b85811061018d57508360019510610175575b505050811b0160045561012b565b01515f1960f88460031b161c191690555f8080610167565b91926020600181928685015181550194019201610155565b60045f527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b601f830160051c810191602084106101fe575b601f0160051c01905b8181106101f357506100fd565b5f81556001016101e6565b90915081906101dd565b634e487b7160e01b5f52602260045260245ffd5b90607f16906100eb565b634e487b7160e01b5f52604160045260245ffd5b015190505f806100b5565b601f1982169360035f52805f20915f5b86811061028e5750836001959610610276575b505050811b016003556100ca565b01515f1960f88460031b161c191690555f8080610268565b91926020600181928685015181550194019201610255565b60035f527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b601f830160051c810191602084106102ff575b601f0160051c01905b8181106102f4575061009b565b5f81556001016102e7565b90915081906102de565b90607f1690610089565b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c90816306fdde031461070357508063095ea7b31461067457806318160ddd1461065757806323b872dd146104e1578063313ce567146104c657806340c10f19146103e557806370a08231146103a157806395d89b41146102265780639dc29fac14610138578063a9059cbb146101075763dd62ed3e14610095575f80fd5b34610103576040600319360112610103576100ae610804565b73ffffffffffffffffffffffffffffffffffffffff6100cb610827565b91165f52600160205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060405f2054604051908152f35b5f80fd5b346101035760406003193601126101035761012d610123610804565b602435903361084a565b602060405160018152f35b3461010357604060031936011261010357610151610804565b73ffffffffffffffffffffffffffffffffffffffff602435911680156101fa57805f525f60205260405f20548281106101c8576020835f947fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef938587528684520360408620558060025403600255604051908152a3005b907fe450d38c000000000000000000000000000000000000000000000000000000005f5260045260245260445260645ffd5b7f96c6fd1e000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b34610103575f600319360112610103576040515f600454908160011c60018316928315610397575b60208210841461036a57818552849390811561032857506001146102cc575b5003601f01601f191681019067ffffffffffffffff82118183101761029f5761029b829182604052826107da565b0390f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60045f90815291507f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b5b81831061030c5750508101602001601f1961026d565b60209193508060019154838588010152019101909183926102f6565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208581019190915291151560051b84019091019150601f19905061026d565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b90607f169061024e565b346101035760206003193601126101035773ffffffffffffffffffffffffffffffffffffffff6103cf610804565b165f525f602052602060405f2054604051908152f35b34610103576040600319360112610103576103fe610804565b73ffffffffffffffffffffffffffffffffffffffff16602435811561049a576002549080820180921161046d5760207fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef915f9360025584845283825260408420818154019055604051908152a3005b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7fec442f05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b34610103575f60031936011261010357602060405160128152f35b34610103576060600319360112610103576104fa610804565b610502610827565b6044359073ffffffffffffffffffffffffffffffffffffffff831692835f52600160205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260405f20547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff811061057e575b5061012d935061084a565b8381106106235784156105f75733156105cb5761012d945f52600160205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f526020528360405f209103905584610573565b7f94280d62000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7fe602df05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b83907ffb8f41b2000000000000000000000000000000000000000000000000000000005f523360045260245260445260645ffd5b34610103575f600319360112610103576020600254604051908152f35b346101035760406003193601126101035761068d610804565b6024359033156105f75773ffffffffffffffffffffffffffffffffffffffff169081156105cb57335f52600160205260405f20825f526020528060405f20556040519081527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560203392a3602060405160018152f35b34610103575f600319360112610103575f600354908160011c600183169283156107d0575b60208210841461036a5781855284939081156103285750600114610774575003601f01601f191681019067ffffffffffffffff82118183101761029f5761029b829182604052826107da565b60035f90815291507fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b5b8183106107b45750508101602001601f1961026d565b602091935080600191548385880101520191019091839261079e565b90607f1690610728565b601f19601f602060409481855280519182918282880152018686015e5f8582860101520116010190565b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361010357565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361010357565b73ffffffffffffffffffffffffffffffffffffffff169081156101fa5773ffffffffffffffffffffffffffffffffffffffff1691821561049a57815f525f60205260405f20548181106108e257817fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef92602092855f525f84520360405f2055845f525f825260405f20818154019055604051908152a3565b827fe450d38c000000000000000000000000000000000000000000000000000000005f5260045260245260445260645ffd608080604052346015576101dc908161001a8239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c9081636813d78714610171578163bcdb83d814610080575063d3072d821461003d575f80fd5b3461007c575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007c57602060ff5f54166040519015158152f35b5f80fd5b3461007c5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007c5760043573ffffffffffffffffffffffffffffffffffffffff81160361007c5760443567ffffffffffffffff811161007c573660238201121561007c57806004013567ffffffffffffffff811161007c573691016024011161007c5760ff5f541661011557005b807f08c379a0000000000000000000000000000000000000000000000000000000006064925260206004820152601460248201527f54617267657420627269646765206661696c65640000000000000000000000006044820152fd5b3461007c5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007c5760043580151580910361007c5760ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff005f54169116175f555f80f36080604052346102a357604051601f61199938819003918201601f19168301916001600160401b0383118484101761026f5780849260a0946040528339810103126102a35761004d816102a7565b610059602083016102a7565b91610066604082016102a7565b916080606083015192015190604051946040860186811060018060401b0382111761026f57604052600b86526a4d6f636b2042726964676560a81b6020870152600180556001600160a01b038216158015610292575b610283576100d6826100d06100dc946102bb565b50610331565b506103c4565b5083516001600160401b03811161026f57600754600181811c91168015610265575b602082101461025157601f81116101ee575b50602094601f821160011461018b579481929394955f92610180575b50508160011b915f199060031b1c1916176007555b60025491600355600455600160a01b9160018060a01b03169060018060a81b03191617176002556201518042046006556040516114c190816104588239f35b015190505f8061012c565b601f1982169560075f52805f20915f5b8881106101d6575083600195969798106101be575b505050811b01600755610141565b01515f1960f88460031b161c191690555f80806101b0565b9192602060018192868501518155019401920161019b565b60075f527fa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c688601f830160051c81019160208410610247575b601f0160051c01905b81811061023c5750610110565b5f815560010161022f565b9091508190610226565b634e487b7160e01b5f52602260045260245ffd5b90607f16906100fe565b634e487b7160e01b5f52604160045260245ffd5b63d92e233d60e01b5f5260045ffd5b506001600160a01b038116156100bc565b5f80fd5b51906001600160a01b03821682036102a357565b6001600160a01b0381165f9081525f5160206119795f395f51905f52602052604090205460ff1661032c576001600160a01b03165f8181525f5160206119795f395f51905f5260205260408120805460ff191660011790553391905f5160206119195f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f5160206119595f395f51905f52602052604090205460ff1661032c576001600160a01b03165f8181525f5160206119595f395f51905f5260205260408120805460ff191660011790553391907f751b795d24b92e3d92d1d0d8f2885f4e9c9c269da350af36ae6b49069babf4bf905f5160206119195f395f51905f529080a4600190565b6001600160a01b0381165f9081525f5160206119395f395f51905f52602052604090205460ff1661032c576001600160a01b03165f8181525f5160206119395f395f51905f5260205260408120805460ff191660011790553391907fc074beb46b251f73c0fec16eba75d6bc0042d09ee17a740ebbee308dbf87f82f905f5160206119195f395f51905f529080a460019056fe6080806040526004361015610012575f80fd5b5f3560e01c90816301ffc9a714610edf575080631033b4cc14610ec2578063118c38c714610e885780631259a5c814610e6b57806318b68b8c14610951578063248a9ca31461091f5780632f2ff15d146108e25780633462fac3146108a857806336568abe1461083e57806336b089d8146108215780635ab1d61c1461078957806361b0a56e1461067e578063632214901461065d57806365d7a3c91461064257806367eeba0c146106255780636813d787146105d65780636bcc8c141461050a57806391d14854146104b4578063a217fddf1461049a578063b16e784914610467578063b20d30a914610413578063c9f5b63e146103e0578063cc3dc061146102e0578063d3072d82146102be578063d547741f1461027a578063ead93c8f14610255578063ede7cebd146101f4578063f681a862146101d75763fb8c4b511461015b575f80fd5b346101d3575f6003193601126101d35760055460045490808211156101c9578082039180831161019c57606092905b60405192835260208301526040820152f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b6060915f9061018a565b5f80fd5b346101d3575f6003193601126101d3576020600954604051908152f35b346101d3575f6003193601126101d357610229600254610212611082565b9060ff604051938493606085526060850190611157565b9173ffffffffffffffffffffffffffffffffffffffff8116602085015260a01c16151560408301520390f35b346101d3575f6003193601126101d357602060ff60025460a01c166040519015158152f35b346101d35760406003193601126101d3576102bc600435610299610fa0565b906102b76102b2825f525f602052600160405f20015490565b61122f565b611367565b005b346101d3575f6003193601126101d357602060ff600854166040519015158152f35b346101d3575f6003193601126101d3576040515f600b5461030081610fc3565b808452906001811690811561039e5750600114610340575b61033c8361032881850382611014565b604051918291602083526020830190611157565b0390f35b919050600b5f527f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9915f905b80821061038457509091508101602001610328610318565b91926001816020925483858801015201910190929161036c565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208086019190915291151560051b840190910191506103289050610318565b346101d3575f6003193601126101d357602073ffffffffffffffffffffffffffffffffffffffff60025416604051908152f35b346101d35760206003193601126101d3577f207c4cbdf55ec315a13f0d5e047732ec5d947da056e706593aa509909941cedf60406004356104526111a7565b600454908060045582519182526020820152a1005b346101d3575f6003193601126101d357602073ffffffffffffffffffffffffffffffffffffffff600a5416604051908152f35b346101d3575f6003193601126101d35760206040515f8152f35b346101d35760406003193601126101d3576104cd610fa0565b6004355f525f60205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060ff60405f2054166040519015158152f35b346101d35760206003193601126101d35773ffffffffffffffffffffffffffffffffffffffff610538610f7d565b6105406111a7565b1680156105ae5773ffffffffffffffffffffffffffffffffffffffff600254827fffffffffffffffffffffffff0000000000000000000000000000000000000000821617600255167fb07f8b1b85042d74022c867c836edeb0bcd70e135b0042390d2b1fd1082980695f80a3005b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101d35760206003193601126101d3576004358015158091036101d35760ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00600854169116176008555f80f35b346101d3575f6003193601126101d3576020600454604051908152f35b346101d3575f6003193601126101d35761033c610328611082565b346101d35760206003193601126101d3576106766111a7565b600435600355005b346101d35760606003193601126101d357610697610f7d565b60443573ffffffffffffffffffffffffffffffffffffffff81168091036101d357335f9081527fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5602052604090205460ff16156107595780156105ae5773ffffffffffffffffffffffffffffffffffffffff6102bc92604051927fa9059cbb0000000000000000000000000000000000000000000000000000000060208501526024840152602435604484015260448352610753606484611014565b1661142f565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b346101d35760206003193601126101d3576004358015158091036101d35760207fb3418989d06835b5c215eebb4d54ed6be7bbb66eb4807164740a2e082fa782d5916107d36111a7565b6002547fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff74ff00000000000000000000000000000000000000008360a01b16911617600255604051908152a1005b346101d3575f6003193601126101d3576020600354604051908152f35b346101d35760406003193601126101d357610857610fa0565b3373ffffffffffffffffffffffffffffffffffffffff821603610880576102bc90600435611367565b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101d3575f6003193601126101d35760206040517fc074beb46b251f73c0fec16eba75d6bc0042d09ee17a740ebbee308dbf87f82f8152f35b346101d35760406003193601126101d3576102bc600435610901610fa0565b9061091a6102b2825f525f602052600160405f20015490565b611295565b346101d35760206003193601126101d35760206109496004355f525f602052600160405f20015490565b604051908152f35b346101d35760606003193601126101d35761096a610f7d565b6024356044359167ffffffffffffffff83116101d357366023840112156101d35782600401359067ffffffffffffffff82116101d35736602483860101116101d357600260015414610e4357600260015560ff60025460a01c1615610e1b57335f9081527ffe0c7bd5092c7a6e463488fa239f32058c87d43e336e2e584a272132cf8dc926602052604090205460ff1615610df35773ffffffffffffffffffffffffffffffffffffffff169081156105ae578215610dcb576003548311610d64576201518042046006548111610d8c575b50610a488360055461119a565b60045410610d6457610aa36040517f23b872dd00000000000000000000000000000000000000000000000000000000602082015233602482015230604482015284606482015260648152610a9d608482611014565b8361142f565b60ff60085416610d0657817fffffffffffffffffffffffff0000000000000000000000000000000000000000600a541617600a5582600955610ae6600b54610fc3565b601f8111610c65575b505f601f8211600114610ba157819293945f92610b93575b50507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8260011b9260031b1c191617600b555b610b468260055461119a565b6005557f3dba401dc1abbf017086864b33ba815f53ab3cdbb9366ba6bcd8ec342ddd98e8602073ffffffffffffffffffffffffffffffffffffffff6002541693604051908152a360018055005b602492500101358480610b07565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0821694600b5f527f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9915f5b878110610c4a575083600195969710610c0f575b505050811b01600b55610b3a565b01602401357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600384901b60f8161c19169055848080610c01565b90926020600181926024878701013581550194019101610bed565b600b5f52601f820160051c7f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db9019060208310610cde575b601f0160051c7f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db901905b818110610cd35750610aef565b5f8155600101610cc6565b7f0175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db99150610c9c565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601c60248201527f4d6f636b2062726964676520657865637574696f6e206661696c6564000000006044820152fd5b7f70d168bc000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fda4e39dd56d72c2ee3d132e0146bc39e905e78e3bc64c40190421c7b2bcef2ab60406005548151908482526020820152a15f60055560065584610a3b565b7f1f2a2005000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f5c427cd9000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f7bea20b2000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101d3575f6003193601126101d3576020600654604051908152f35b346101d3575f6003193601126101d35760206040517f751b795d24b92e3d92d1d0d8f2885f4e9c9c269da350af36ae6b49069babf4bf8152f35b346101d3575f6003193601126101d3576020600554604051908152f35b346101d35760206003193601126101d357600435907fffffffff0000000000000000000000000000000000000000000000000000000082168092036101d357817f7965db0b0000000000000000000000000000000000000000000000000000000060209314908115610f53575b5015158152f35b7f01ffc9a70000000000000000000000000000000000000000000000000000000091501483610f4c565b6004359073ffffffffffffffffffffffffffffffffffffffff821682036101d357565b6024359073ffffffffffffffffffffffffffffffffffffffff821682036101d357565b90600182811c9216801561100a575b6020831014610fdd57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f1691610fd2565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761105557604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051905f826007549161109583610fc3565b808352926001811690811561111a57506001146110bb575b6110b992500383611014565b565b5060075f90815290917fa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c6885b8183106110fe5750509060206110b9928201016110ad565b60209193508060019154838589010152019101909184926110e6565b602092506110b99491507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001682840152151560051b8201016110ad565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b9190820180921161019c57565b335f9081527fdfff91bcee88f6ea00b2726f6d062a509c32835793470c55bed3bcc74c0f5d37602052604090205460ff16156111df57565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f751b795d24b92e3d92d1d0d8f2885f4e9c9c269da350af36ae6b49069babf4bf60245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f205416156112665750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f1461136157805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f1461136157805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190565b905f602091828151910182855af1156114b6575f513d6114ad575073ffffffffffffffffffffffffffffffffffffffff81163b155b61146b5750565b73ffffffffffffffffffffffffffffffffffffffff907f5274afe7000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b60011415611464565b6040513d5f823e3d90fd2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0dfe0c7bd5092c7a6e463488fa239f32058c87d43e336e2e584a272132cf8dc926dfff91bcee88f6ea00b2726f6d062a509c32835793470c55bed3bcc74c0f5d37ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x08\x9C\xA9\xE0\x14a\\\xE1WP\x80c\tO'\xA0\x14aZ\xDAW\x80c\n\x92T\xE4\x14aXgW\x80c\x0FW(\x0F\x14aRtW\x80c\x10t\xA2\x1F\x14aQ=W\x80c\x11~;B\x14aQ\x18W\x80c\x13!\x7F\x90\x14aP\xF1W\x80c\x13\xA8o\x1A\x14aN\x06W\x80c\x1E\xD7\x83\x1C\x14aM\x88W\x80c#\xE1\xEB\xE7\x14aJ\xC7W\x80c$\x8E\xC3&\x14aJ\xA1W\x80c)6Yh\x14aH\xA7W\x80c*\xDE8\x80\x14aG\"W\x80c.\xD2\x11\x83\x14aE|W\x80c>^<#\x14aD\xFEW\x80c?r\x86\xF4\x14aD\x80W\x80cI{9\x18\x14aA\x8FW\x80cJa\xCF)\x14a>\x15W\x80cO\x862\xBA\x14a=\xEEW\x80cf\xD9\xA9\xA0\x14a<\xB1W\x80co\x8C\xEC\xE4\x14a;&W\x80c\x85\"l\x81\x14a:\x94W\x80c\x8BX\xCB\xAE\x14a7xW\x80c\x8FX\xA6?\x14a6\xC2W\x80c\x91j\x17\xC6\x14a6\x18W\x80c\x95m\x98\x08\x14a5#W\x80c\x95\x9B3}\x14a4\xFCW\x80c\xA3\x0F\xF4\xC2\x14a2BW\x80c\xA3\xD4H[\x14a2\x18W\x80c\xA3\xFB\x17\x15\x14a/`W\x80c\xB0FO\xDC\x14a.\xB6W\x80c\xB4M\xC9\xD6\x14a+\x06W\x80c\xB5P\x8A\xA9\x14a*tW\x80c\xB5]B\xBC\x14a'LW\x80c\xB9\xB5\xBDh\x14a \x9EW\x80c\xBAAO\xA6\x14a yW\x80c\xBEm\xA5>\x14a\x1E\xD6W\x80c\xCF\xFB\x04\x8B\x14a\x1A\xD9W\x80c\xD3\x07\\I\x14a\x14\xE6W\x80c\xD3\xB7k\xC9\x14a\x13vW\x80c\xDB\x9Bp\x8C\x14a\x11\xEFW\x80c\xDC\xCCW\xF1\x14a\x0E\x8FW\x80c\xE2\x0C\x9Fq\x14a\x0E\x01W\x80c\xE8kO\xA7\x14a\n\xD5W\x80c\xE9\xD3\xD5\x86\x14a\t/W\x80c\xF3\xED+\x05\x14a\x06\xB5W\x80c\xF8Q\xA4@\x14a\x06\x8EW\x80c\xF9~\x84g\x14a\x02\xC1W\x80c\xFAv&\xD4\x14a\x02\x9EW\x80c\xFC\x0CTj\x14a\x02xWc\xFC\x9C\x8D9\x14a\x02OW_\x80\xFD[4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[\x80\xFD[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x81;\x15a\x06\x8AW\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\x06uW[P`$\x90`\x01`\x01`\xA0\x1B\x03` T\x16` `\x01`\x01`\xA0\x1B\x03`\"T\x16\x91`@Q\x94\x85\x80\x92\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x85`\x04\x83\x01RZ\xFA\x92\x83\x15a\x05\x97W\x82\x93a\x06>W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x06)W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x82;\x15a\x06%W`d\x84\x92\x83`@Q\x95\x86\x94\x85\x93\x7Fa\xB0\xA5n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x85\x01R`D\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\x06\x0CW[PP`\x01`\x01`\xA0\x1B\x03` T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x90\x81\x15a\x06\x01W\x84\x91a\x05\xCFW[Pi\x15-\x02\xC7\xE1J\xF6\x80\0\0\x82\x01\x80\x92\x11a\x05\xA2W\x90a\x05\x01\x91ai\xF6V[` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`$`@Q\x80\x94\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x80\x15a\x05\x97W\x82\x90a\x05_W[a\x05\\\x91Pai\x80V[\x80\xF3[P` \x81=` \x11a\x05\x8FW[\x81a\x05y` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa\x05\\\x90Qa\x05RV[_\x80\xFD[=\x91Pa\x05lV[`@Q=\x84\x82>=\x90\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11a\x05\xF9W[\x81a\x05\xEA` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQ_a\x04\xE2V[=\x91Pa\x05\xDDV[`@Q=\x86\x82>=\x90\xFD[\x81a\x06\x16\x91ab\x19V[a\x06!W\x81_a\x04\x84V[P\x80\xFD[\x83\x80\xFD[\x81a\x063\x91ab\x19V[a\x06!W\x81_a\x03\xFEV[\x91P\x91P` \x81=` \x11a\x06mW[\x81a\x06[` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BW\x82\x90Q\x91_a\x03\xA7V[=\x91Pa\x06NV[\x81a\x06\x7F\x91ab\x19V[a\x02uW\x80_a\x03HV[PP\xFD[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R`\x01`$\x82\x01R\x81`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\t\x1AW[P`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x91\x7F\xB0\x7F\x8B\x1B\x85\x04-t\x02,\x86|\x83n\xDE\xB0\xBC\xD7\x0E\x13[\0B9\r+\x1F\xD1\x08)\x80i\x84\x80\xA3`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x8AWc\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\t\x05W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x81;\x15a\x06\x8AW\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7Fk\xCC\x8C\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\x08\xF0W[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xC9\xF5\xB6>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x90a\x08\xB5W[a\x05\\\x91P`\x01`\x01`\xA0\x1B\x03`%T\x16\x90aj\xF1V[P` \x81=` \x11a\x08\xE8W[\x81a\x08\xCF` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa\x08\xE3a\x05\\\x91af\xCDV[a\x08\x9EV[=\x91Pa\x08\xC2V[\x81a\x08\xFA\x91ab\x19V[a\x02uW\x80_a\x08PV[\x81a\t\x0F\x91ab\x19V[a\x02uW\x80_a\x07\xEAV[\x81a\t$\x91ab\x19V[a\x02uW\x80_a\x07KV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\n\xBDW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\n\xA8W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01R\x81`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a\n\xA1\x91ab\x19V[a\x02uW\x80\xF3[\x81a\n\xB2\x91ab\x19V[a\x02uW\x80_a\n5V[\x81a\n\xC7\x91ab\x19V[a\x02uW\x80_a\t\xA3V[P\xFD[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`$T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\r\xECW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97W\x82\x91a\r\xB7W[Pa\x0C\x1C`\x01`\x01`\xA0\x1B\x03`$T\x16a\x0C\x0E`@Q\x93\x84\x92\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x85\x01R`$\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03`\x1F\x19\x81\x01\x83R\x82ab\x19V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W\x81a\x0Cw\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a`gV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\r\xA2W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x82;\x15a\r\x9DW`d\x84\x92\x83`@Q\x95\x86\x94\x85\x93\x7Fa\xB0\xA5n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x04\x85\x01Ra\x03\xE8`$\x85\x01R`D\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\r\x88W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a\r\x92\x91ab\x19V[a\x02uW\x80_a\r\x1AV[PPP\xFD[\x81a\r\xAC\x91ab\x19V[a\x02uW\x80_a\x0C\x9CV[\x91PP` \x81=` \x11a\r\xE4W[\x81a\r\xD3` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BW\x81\x90Q_a\x0B\xB0V[=\x91Pa\r\xC6V[\x81a\r\xF6\x91ab\x19V[a\x02uW\x80_a\x0BbV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x0EpWa\x0El\x85a\x0E`\x81\x87\x03\x82ab\x19V[`@Q\x91\x82\x91\x82a`%V[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x0EIV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x11\x04W\x83\x91a\x11\xBDW[P`\"T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01\x81\x90R\x90` \x81`D\x81\x86Z\xFA\x80\x15a\x06\x01W\x84\x90a\x11\x82W[a\x0FO\x91PaksV[`@Q\x7F\x11\x8C8\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x06\x01W\x84\x91a\x11NW[P`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R` \x81\x80`D\x81\x01[\x03\x81\x85Z\xFA\x80\x15a\x11\x04W\x83\x90a\x11\x0FW[a\x0F\xF0\x91PaksV[`@Q\x7F4b\xFA\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x11\x04W\x83\x91a\x10\xD0W[P`#T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90` \x90\x82\x90\x81\x80`D\x81\x01[\x03\x91Z\xFA\x80\x15a\x05\x97W\x82\x90a\x10\x95W[a\x05\\\x91PaksV[P` \x81=` \x11a\x10\xC8W[\x81a\x10\xAF` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa\x10\xC3a\x05\\\x91ab<V[a\x10\x8BV[=\x91Pa\x10\xA2V[\x90P` \x81=` \x11a\x10\xFCW[\x81a\x10\xEB` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQa\x10za\x10-V[=\x91Pa\x10\xDEV[`@Q=\x85\x82>=\x90\xFD[P` \x81=` \x11a\x11FW[\x81a\x11)` \x93\x83ab\x19V[\x81\x01\x03\x12a\x11BWa\x11=a\x0F\xF0\x91ab<V[a\x0F\xE6V[\x82\x80\xFD[=\x91Pa\x11\x1CV[\x90P` \x81=` \x11a\x11zW[\x81a\x11i` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQa\x0F\xD4a\x0F\x8CV[=\x91Pa\x11\\V[P` \x81=` \x11a\x11\xB5W[\x81a\x11\x9C` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06%Wa\x11\xB0a\x0FO\x91ab<V[a\x0FEV[=\x91Pa\x11\x8FV[\x90P` \x81=` \x11a\x11\xE7W[\x81a\x11\xD8` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQ_a\x0E\xECV[=\x91Pa\x11\xCBV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x13aW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x13LW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\n\xD2W\x81\x80\x91`\x84`@Q\x80\x94\x81\x93c\x06-\xA2\xE3`\xE2\x1B\x83R\x81`\x04\x84\x01Ra\x03\xE8`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a\x13V\x91ab\x19V[a\x02uW\x80_a\x12\xF5V[\x81a\x13k\x91ab\x19V[a\x02uW\x80_a\x12cV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x14\xD1W[PP`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`@Q\x91a\x19\x99\x91\x82\x84\x01\x92\x84\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x14\xA4W\x91`\xA0\x93\x91\x85\x93a{\xFF\x859\x82R\x85` \x83\x01R`@\x82\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0``\x82\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`\x80\x82\x01R\x03\x01\x90\x82\xF0\x15a\x14\x98W\x80\xF3[`@Q\x90=\x90\x82>=\x90\xFD[`$\x86\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81a\x14\xDB\x91ab\x19V[a\x02uW\x80_a\x14\x17V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x1A\xC4W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rj\x01\xA7\x847\x9D\x99\xDBB\0\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\x1A\x8DW[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x1AxW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x90\x80;\x15a\x06\x8AW`@Qc\x06-\xA2\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05\x97Wa\x1AcW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x90a\x1A/W[a\x16\xFA\x91Pag\xF5V[b\x01Q\x80B\x01\x80B\x11a\x1A\x02W\x81\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x19\xEDW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80a\x17\xE3`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93_\x81R_` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x19\xD8W[PP\x7F\xDAN9\xDDV\xD7,.\xE3\xD12\xE0\x14k\xC3\x9E\x90^x\xE3\xBCd\xC4\x01\x90B\x1C{+\xCE\xF2\xAB`@\x80Qb\x01Q\x80B\x04\x81Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0` \x82\x01R\xA1\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\x19\xC3W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x90\x80;\x15a\x06\x8AW`@Qc\x06-\xA2\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05\x97Wa\x19\xAEW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x90a\x19zW[a\x05\\\x91Pag\xF5V[P` \x81=` \x11a\x19\xA6W[\x81a\x19\x94` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa\x05\\\x90Qa\x19pV[=\x91Pa\x19\x87V[\x81a\x19\xB8\x91ab\x19V[a\x02uW\x80_a\x19\"V[\x81a\x19\xCD\x91ab\x19V[a\x02uW\x80_a\x18\xABV[\x81a\x19\xE2\x91ab\x19V[a\x02uW\x80_a\x18\x08V[\x81a\x19\xF7\x91ab\x19V[a\x02uW\x80_a\x17xV[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[P` \x81=` \x11a\x1A[W[\x81a\x1AI` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa\x16\xFA\x90Qa\x16\xF0V[=\x91Pa\x1A<V[\x81a\x1Am\x91ab\x19V[a\x02uW\x80_a\x16\xA2V[\x81a\x1A\x82\x91ab\x19V[a\x02uW\x80_a\x16+V[` \x81=` \x11a\x1A\xBCW[\x81a\x1A\xA6` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa\x1A\xB7\x90ab<V[a\x15\xC7V[=\x91Pa\x1A\x99V[\x81a\x1A\xCE\x91ab\x19V[a\x02uW\x80_a\x15ZV[P4a\x02uW` `\x03\x196\x01\x12a\x02uWa\x1B\x03i\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`\x01`\x045al\x1CV[`@\x90\x82\x80\x83Qa\x1B\x14\x85\x82ab\x19V[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x84Qa\x1B\x93\x81a\x1B\x7F` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x89`$\x84\x01R`d\x83\x01\x90a`gV[\x87`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82ab\x19V[Q\x90jconsole.logZ\xFAP\x82`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W\x83Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x1E^Wa\x1E\xC1W[PP` \x80T`\x1FT\x84Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03`\x08\x92\x90\x92\x1C\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x92\x91\x83\x91\x16\x81\x87\x81`D\x81\x01\x03\x92Z\xF1\x80\x15a\x1E\xB4Wa\x1E}W[P\x82`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W\x83Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x1E^Wa\x1EhW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x11BW\x82\x91`\x84\x83\x92\x87Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01R\x88`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x1E^Wa\x1EIW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x91\x80Q\x7F\xF6\x81\xA8b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA\x90\x81\x15a\x1E?W\x85\x91a\x1E\x0CW[P`\x04\x93a\x1D\x8B\x84` \x93ai\xF6V[\x82Q\x94\x85\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x1E\x03WP\x83\x90a\x1D\xCFW[a\x05\\\x92Pai\xF6V[P` \x82=` \x11a\x1D\xFBW[\x81a\x1D\xE9` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa\x05\\\x91Qa\x1D\xC5V[=\x91Pa\x1D\xDCV[Q=\x85\x82>=\x90\xFD[\x90P` \x81=` \x11a\x1E7W[\x81a\x1E'` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQ`\x04a\x1D{V[=\x91Pa\x1E\x1AV[\x82Q=\x87\x82>=\x90\xFD[\x81a\x1ES\x91ab\x19V[a\x11BW\x82_a\x1D-V[\x84Q=\x84\x82>=\x90\xFD[\x81a\x1Er\x91ab\x19V[a\x11BW\x82_a\x1C\xCEV[` \x81=` \x11a\x1E\xACW[\x81a\x1E\x96` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06%Wa\x1E\xA7\x90ab<V[a\x1CkV[=\x91Pa\x1E\x89V[PPPQ\x90=\x90\x82>=\x90\xFD[\x81a\x1E\xCB\x91ab\x19V[a\x11BW\x82_a\x1C\x06V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`$T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa dW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\\B|\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa OW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x90\x80;\x15a\x06\x8AW`@Qc\x06-\xA2\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01Ra\x03\xE8`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90\x81\x83\x81`\x84\x81\x01[\x03\x92Z\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a Y\x91ab\x19V[a\x02uW\x80_a\x1F\xDCV[\x81a n\x91ab\x19V[a\x02uW\x80_a\x1FJV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` a \x94ag\x1CV[`@Q\x90\x15\x15\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`@Q` \x80\x82\x01R`\t`@\x82\x01R\x7Ftest data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R``\x81Ra \xF4`\x80\x82ab\x19V[\x81`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa'7W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x11\x04Wa'\0W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W\x81`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R\x81`$\x82\x01R`\x01`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa&\xEBW[PP`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x7F=\xBA@\x1D\xC1\xAB\xBF\x01p\x86\x86K3\xBA\x81_S\xAB<\xDB\xB96k\xA6\xBC\xD8\xEC4-\xDD\x98\xE8` `@Qii\xE1\r\xE7fv\xD0\x80\0\0\x81R\xA3\x81`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa&\xD6W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x11BW\x82\x90`@Q\x92\x83\x91c\x06-\xA2\xE3`\xE2\x1B\x83R`\x04\x83\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x83\x01R```D\x83\x01R\x81\x83\x81a#\\`d\x82\x01\x8Aa`gV[\x03\x92Z\xF1\x80\x15a\x05\x97Wa&\xC1W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x06\x01W\x84\x90a&\x8DW[a#\xC2\x91Pai\0V[`@Q\x7F\xF6\x81\xA8b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x06\x01W\x84\x90a&YW[a$\x08\x91Pai\0V[\x82`@Q\x92\x7F\xB1nxI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R` \x84`\x04\x81\x86Z\xFA\x93\x84\x15a\x05\x97W\x82\x94a&\x1DW[Pa$^`\x01`\x01`\xA0\x1B\x03` T\x16\x80\x95aj\xF1V[`@Q\x7F\xCC=\xC0a\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81`\x04\x81\x87Z\xFA\x90\x81\x15a\x11\x04W\x83\x91a%\xC8W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x11BWa$\xFA\x83\x91a%\x0C`@Q\x94\x85\x93\x84\x93\x7F\x97bF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`@`\x04\x86\x01R`D\x85\x01\x90a`gV[\x90`\x03\x19\x84\x83\x03\x01`$\x85\x01Ra`gV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05\x97Wa%\xB3W[PP` \x90`$`@Q\x80\x94\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x80\x15a\x05\x97W\x82\x90a%\x7FW[a\x05\\\x91Pai\0V[P` \x81=` \x11a%\xABW[\x81a%\x99` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa\x05\\\x90Qa%uV[=\x91Pa%\x8CV[\x81a%\xBD\x91ab\x19V[a\x11BW\x82_a%0V[\x90P=\x80\x84\x83>a%\xD9\x81\x83ab\x19V[\x81\x01\x90` \x81\x83\x03\x12a\x06%W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a&\x19W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x06%W\x81Qa&\x13\x92` \x01afjV[_a$\x9AV[\x84\x80\xFD[\x90\x93P` \x81=` \x11a&QW[\x81a&9` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa&J\x90af\xCDV[\x92_a$GV[=\x91Pa&,V[P` \x81=` \x11a&\x85W[\x81a&s` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa$\x08\x90Qa#\xFEV[=\x91Pa&fV[P` \x81=` \x11a&\xB9W[\x81a&\xA7` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa#\xC2\x90Qa#\xB8V[=\x91Pa&\x9AV[\x81a&\xCB\x91ab\x19V[a\x06!W\x81_a#kV[\x81a&\xE0\x91ab\x19V[a\x06!W\x81_a\"\xFBV[\x81a&\xF5\x91ab\x19V[a\x06!W\x81_a\"IV[` \x81=` \x11a'/W[\x81a'\x19` \x93\x83ab\x19V[\x81\x01\x03\x12a\x11BWa'*\x90ab<V[a!\xC3V[=\x91Pa'\x0CV[\x81a'A\x91ab\x19V[a\x06!W\x81_a!WV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa*_W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa*(W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa*\x13W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa)\xFEW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xF6\x81\xA8b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97W\x82\x91a)\xC9W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x91PP` \x81=` \x11a)\xF6W[\x81a)\xE5` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BW\x81\x90Q_a)HV[=\x91Pa)\xD8V[\x81a*\x08\x91ab\x19V[a\x02uW\x80_a(\xFAV[\x81a*\x1D\x91ab\x19V[a\x02uW\x80_a(\x90V[` \x81=` \x11a*WW[\x81a*A` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa*R\x90ab<V[a(,V[=\x91Pa*4V[\x81a*i\x91ab\x19V[a\x02uW\x80_a'\xC0V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x19Ta*\x91\x81ab\x9EV[\x91a*\x9F`@Q\x93\x84ab\x19V[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a*\xE9W`@Q` \x80\x82R\x81\x90a\x0El\x90\x82\x01\x88a`\x8CV[`\x01` \x81\x92a*\xF8\x85ab\xB6V[\x81R\x01\x92\x01\x92\x01\x91\x90a*\xCCV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xC9\xF5\xB6>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x11\x04W\x83\x90a.{W[a+y\x91P`\x01`\x01`\xA0\x1B\x03`!T\x16\x90aj\xF1V[`@Q\x7Fe\xD7\xA3\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81`\x04\x81\x85Z\xFA\x80\x15a\x11\x04W\x83\x90a.9W[a+\xC7\x91Pa+\xC1af\xE1V[\x90ajlV[`@Q\x7F6\xB0\x89\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x11\x04W\x83\x90a.\x05W[a,\r\x91Pag\xF5V[`@Q\x7Fg\xEE\xBA\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x11\x04W\x83\x90a-\xD1W[a,S\x91Pah\x7FV[`@Q\x7F\xEA\xD9<\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x11\x04W\x83\x90a-\x96W[a,\x99\x91PaksV[`@Q\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x11\x04W\x83\x91a-cW[P`\x04\x91a,\xE5` \x92ai\x80V[`@Q\x92\x83\x80\x92\x7F\x12Y\xA5\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x90a-/W[a\x05\\\x91Pb\x01Q\x80B\x04\x90ai\xF6V[P` \x81=` \x11a-[W[\x81a-I` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa\x05\\\x90Qa-\x1EV[=\x91Pa-<V[\x90P` \x81=` \x11a-\x8EW[\x81a-~` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQ`\x04a,\xD6V[=\x91Pa-qV[P` \x81=` \x11a-\xC9W[\x81a-\xB0` \x93\x83ab\x19V[\x81\x01\x03\x12a\x11BWa-\xC4a,\x99\x91ab<V[a,\x8FV[=\x91Pa-\xA3V[P` \x81=` \x11a-\xFDW[\x81a-\xEB` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa,S\x90Qa,IV[=\x91Pa-\xDEV[P` \x81=` \x11a.1W[\x81a.\x1F` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWa,\r\x90Qa,\x03V[=\x91Pa.\x12V[P=\x80\x84\x83>a.I\x81\x83ab\x19V[\x81\x01\x90` \x81\x83\x03\x12a\x06%W\x80Q\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a&\x19Wa+\xC7\x92a.v\x92\x01af\xB0V[a+\xB4V[P` \x81=` \x11a.\xAEW[\x81a.\x95` \x93\x83ab\x19V[\x81\x01\x03\x12a\x11BWa.\xA9a+y\x91af\xCDV[a+bV[=\x91Pa.\x88V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x1CTa.\xD3\x81ab\x9EV[\x91a.\xE1`@Q\x93\x84ab\x19V[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a/#W`@Q\x80a\x0El\x87\x82aa9V[`\x02` `\x01\x92`@Qa/6\x81aa\xD0V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra/N\x85\x87\x01ac\xB9V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a/\x0EV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa2\x03W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\x01`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa1\xCCW[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa1\xB7W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7Fp\xD1h\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa1\xA2W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\x01`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a1\xAC\x91ab\x19V[a\x02uW\x80_a16V[\x81a1\xC1\x91ab\x19V[a\x02uW\x80_a0\xA4V[` \x81=` \x11a1\xFBW[\x81a1\xE5` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa1\xF6\x90ab<V[a0@V[=\x91Pa1\xD8V[\x81a2\r\x91ab\x19V[a\x02uW\x80_a/\xD4V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa4\xE7W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\x01`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa4\xB0W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa4\x9BW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7Fp\xD1h\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa4\x86W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\x01`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a4\x90\x91ab\x19V[a\x02uW\x80_a4\x19V[\x81a4\xA5\x91ab\x19V[a\x02uW\x80_a3\x87V[` \x81=` \x11a4\xDFW[\x81a4\xC9` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa4\xDA\x90ab<V[a3#V[=\x91Pa4\xBCV[\x81a4\xF1\x91ab\x19V[a\x02uW\x80_a2\xB6V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x04\x81`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xED\xE7\xCE\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x83\x92\x84\x92a5\xADW[Pa\x05\\\x92a5\x96a5\xA8\x92a+\xC1af\xE1V[`\x01`\x01`\xA0\x1B\x03`!T\x16\x90aj\xF1V[aksV[\x92PPP=\x80\x83\x83>a5\xC0\x81\x83ab\x19V[\x81\x01\x90``\x81\x83\x03\x12a\x11BW\x80Q\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x06%Wa5\xF1a5\xA8\x91a\x05\\\x94\x84\x01af\xB0V[a5\x96a6\x0C`@a6\x05` \x87\x01af\xCDV[\x95\x01ab<V[\x93\x94\x91\x92Pa5\x82\x90PV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x1DTa65\x81ab\x9EV[\x91a6C`@Q\x93\x84ab\x19V[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a6\x85W`@Q\x80a\x0El\x87\x82aa9V[`\x02` `\x01\x92`@Qa6\x98\x81aa\xD0V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra6\xB0\x85\x87\x01ac\xB9V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a6pV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x04```\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xFB\x8CKQ\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97Wa\x05\\\x91\x83\x84\x90\x85\x92a7;W[a76\x92\x93Pa76\x90ai\x80V[ah\x7FV[PPPa76a7ea76\x92``=``\x11a7qW[a7]\x81\x83ab\x19V[\x81\x01\x90abIV[\x91\x93P\x90\x91P\x82a7'V[P=a7SV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa:\x7FW[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa:HW[P\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01Ri?\x87\x08W\xA3\xE0\xE3\x80\0\0`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa:3W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01Ri*Z\x05\x8F\xC2\x95\xED\0\0\0`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa:\x1EW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa:\tW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x90a%\x7FWa\x05\\\x91Pai\0V[\x81a:\x13\x91ab\x19V[a\x02uW\x80_a9\xB2V[\x81a:(\x91ab\x19V[a\x02uW\x80_a9FV[\x81a:=\x91ab\x19V[a\x02uW\x80_a8\xDCV[` \x81=` \x11a:wW[\x81a:a` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa:r\x90ab<V[a8qV[=\x91Pa:TV[\x81a:\x89\x91ab\x19V[a\x02uW\x80_a8\x05V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x1ATa:\xB1\x81ab\x9EV[\x91a:\xBF`@Q\x93\x84ab\x19V[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a;\tW`@Q` \x80\x82R\x81\x90a\x0El\x90\x82\x01\x88a`\x8CV[`\x01` \x81\x92a;\x18\x85ab\xB6V[\x81R\x01\x92\x01\x92\x01\x91\x90a:\xECV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa<\x9CW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa<\x87W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\n\xD2W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7Fk\xCC\x8C\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81a<\x91\x91ab\x19V[a\x02uW\x80_a<,V[\x81a<\xA6\x91ab\x19V[a\x02uW\x80_a;\x9AV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x1BTa<\xCE\x81ab\x9EV[a<\xDB`@Q\x91\x82ab\x19V[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a=\xB3W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a=HWPPPP\x03\x90\xF3[\x91\x93` a=\xA3\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a=\x93\x83Q`@\x84R`@\x84\x01\x90a`gV[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra`\xE4V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a=9V[`\x02` `\x01\x92`@Qa=\xC6\x81aa\xD0V[a=\xCF\x86ab\xB6V[\x81Ra=\xDC\x85\x87\x01ac\xB9V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a=\x0BV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x90\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaAzW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\n\xD2W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7FZ\xB1\xD6\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97WaAeW[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaAPW[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ra\x03\xE8`$\x84\x01RZ\xF1\x80\x15a\x05\x97WaA\x19W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaA\x04W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F{\xEA \xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa OWP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x90\x80;\x15a\x06\x8AW`@Qc\x06-\xA2\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01Ra\x03\xE8`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90\x81\x83\x81`\x84\x81\x01a >V[\x81aA\x0E\x91ab\x19V[a\x02uW\x80_a@\rV[` \x81=` \x11aAHW[\x81aA2` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!WaAC\x90ab<V[a?\xA9V[=\x91PaA%V[\x81aAZ\x91ab\x19V[a\x02uW\x80_a?EV[\x81aAo\x91ab\x19V[a\x02uW\x80_a>\xE2V[\x81aA\x84\x91ab\x19V[a\x02uW\x80_a>\x89V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80aB\t`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93_\x81R_` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaDkW[PP\x7F |L\xBD\xF5^\xC3\x15\xA1?\r^\x04w2\xEC]\x94}\xA0V\xE7\x06Y:\xA5\t\x90\x99A\xCE\xDF`@\x80Qj\x04\"\xCA\x8B\n\0\xA4%\0\0\0\x81Rj\x08E\x95\x16\x14\x01HJ\0\0\0` \x82\x01R\xA1\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaDVW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\n\xD2W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\xB2\r0\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rj\x08E\x95\x16\x14\x01HJ\0\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97WaDAW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7Fg\xEE\xBA\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97W\x82\x91aD\x0CW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rj\x08E\x95\x16\x14\x01HJ\0\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x91PP` \x81=` \x11aD9W[\x81aD(` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BW\x81\x90Q_aC\x8AV[=\x91PaD\x1BV[\x81aDK\x91ab\x19V[a\x02uW\x80_aC<V[\x81aD`\x91ab\x19V[a\x02uW\x80_aB\xD8V[\x81aDu\x91ab\x19V[a\x02uW\x80_aB.V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10aD\xDFWa\x0El\x85a\x0E`\x81\x87\x03\x82ab\x19V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aD\xC8V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10aE]Wa\x0El\x85a\x0E`\x81\x87\x03\x82ab\x19V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aEFV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaG\rW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaF\xF8W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`d\x83\x92`@Q\x94\x85\x93\x84\x92\x7Fa\xB0\xA5n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ra\x03\xE8`$\x84\x01R\x81`D\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81aG\x02\x91ab\x19V[a\x02uW\x80_aF\x82V[\x81aG\x17\x91ab\x19V[a\x02uW\x80_aE\xF0V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`\x1ETaG?\x81ab\x9EV[aGL`@Q\x91\x82ab\x19V[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10aH\x1EW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10aG\xB9WPPPP\x03\x90\xF3[\x91\x93` aH\x0E\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R`@\x83\x8AQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a`\x8CV[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92aG\xAAV[`@QaH*\x81aa\xD0V[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80TaHF\x81ab\x9EV[\x91aHT`@Q\x93\x84ab\x19V[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10aH\x8AWPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90aG|V[`\x01` \x81\x92aH\x99\x86ab\xB6V[\x81R\x01\x93\x01\x91\x01\x90\x91aHdV[P4a\x02uW` `\x03\x196\x01\x12a\x02uWaH\xD7o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\x045al\x1CV[`@\x90\x82\x80\x83QaH\xE8\x85\x82ab\x19V[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x84QaIS\x81a\x1B\x7F` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x89`$\x84\x01R`d\x83\x01\x90a`gV[Q\x90jconsole.logZ\xFAP\x82`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06!W\x83Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x1E^WaJ\x8CW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x06!W\x81\x80\x91`$\x86Q\x80\x94\x81\x93\x7F\xB2\r0\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x88`\x04\x84\x01RZ\xF1\x80\x15a\x1E^WaJwW[PP`\x04\x91` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x82Q\x94\x85\x80\x92\x7Fg\xEE\xBA\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x1E\x03WP\x83\x90a\x1D\xCFWa\x05\\\x92Pai\xF6V[\x81aJ\x81\x91ab\x19V[a\x11BW\x82_aJ\x1EV[\x81aJ\x96\x91ab\x19V[a\x11BW\x82_aI\xC6V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `@Qj\x04\"\xCA\x8B\n\0\xA4%\0\0\0\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81\x80aKA`\x04\x82\x01\x90`\x01```\x80\x84\x01\x93_\x81R_` \x82\x01R_`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaMsW[PP\x7F\xB3A\x89\x89\xD0h5\xB5\xC2\x15\xEE\xBBMT\xEDk\xE7\xBB\xB6n\xB4\x80qdt\n.\x08/\xA7\x82\xD5` `@Q\x83\x81R\xA1\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaM^W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\n\xD2W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7FZ\xB1\xD6\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97WaMIW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xEA\xD9<\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97W\x82\x91aM\x0FW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R\x81\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x90P` \x81=` \x11aMAW[\x81aM*` \x93\x83ab\x19V[\x81\x01\x03\x12a\n\xD2WaM;\x90ab<V[_aL\x9CV[=\x91PaM\x1DV[\x81aMS\x91ab\x19V[a\x02uW\x80_aLNV[\x81aMh\x91ab\x19V[a\x02uW\x80_aK\xF5V[\x81aM}\x91ab\x19V[a\x02uW\x80_aKfV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10aM\xE7Wa\x0El\x85a\x0E`\x81\x87\x03\x82ab\x19V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aM\xD0V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`$T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaP\xDCW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x11\x8C8\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97W\x82\x91aP\xA7W[PaO?`\x01`\x01`\xA0\x1B\x03`$T\x16a\x0C\x0E`@Q\x93\x84\x92\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x85\x01R`$\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W\x81aO\x9A\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a`gV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaP\x92W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x81;\x15a\x06\x8AW\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7Fk\xCC\x8C\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\r\x88WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81aP\x9C\x91ab\x19V[a\x02uW\x80_aO\xBFV[\x91PP` \x81=` \x11aP\xD4W[\x81aP\xC3` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BW\x81\x90Q_aN\xE1V[=\x91PaP\xB6V[\x81aP\xE6\x91ab\x19V[a\x02uW\x80_aN\x93V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW` `@Qi\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0\x81R\xF3[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaR_W[PP`\x01`\x01`\xA0\x1B\x03`#T\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`@Q\x91a\x19\x99\x91\x82\x84\x01\x92\x84\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x14\xA4W\x91`\xA0\x93\x91\x85\x93a{\xFF\x859\x86\x83R` \x83\x01R`@\x82\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0``\x82\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`\x80\x82\x01R\x03\x01\x90\x82\xF0\x15a\x14\x98W\x80\xF3[\x81aRi\x91ab\x19V[a\x02uW\x80_aQ\xDEV[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaXRW[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97WaX\x1BW[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaX\x06W[P[`\n\x81\x10aW}WPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaWhW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x90aW4W[aT\x9F\x91Pah\x7FV[\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaW\x1FW[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x05\x97WaV\xE8W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaV\xD3W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x02uW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7Fp\xD1h\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97WaV\xBEW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x06\x8AW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x81aV\xC8\x91ab\x19V[a\x02uW\x80_aV[V[\x81aV\xDD\x91ab\x19V[a\x02uW\x80_aU\xC9V[` \x81=` \x11aW\x17W[\x81aW\x01` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!WaW\x12\x90ab<V[aUeV[=\x91PaV\xF4V[\x81aW)\x91ab\x19V[a\x02uW\x80_aU\x02V[P` \x81=` \x11aW`W[\x81aWN` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWaT\x9F\x90QaT\x95V[=\x91PaWAV[\x81aWr\x91ab\x19V[a\x02uW\x80_aTGV[\x81`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x81;\x15a\x11BW\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92c\x06-\xA2\xE3`\xE2\x1B\x84R`\x04\x84\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x05\x97WaW\xF1W[PP`\x01\x01aS\xD4V[\x81aW\xFB\x91ab\x19V[a\x06!W\x81_aW\xE7V[\x81aX\x10\x91ab\x19V[a\x02uW\x80_aS\xD2V[` \x81=` \x11aXJW[\x81aX4` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!WaXE\x90ab<V[aSUV[=\x91PaX'V[\x81aX\\\x91ab\x19V[a\x02uW\x80_aR\xE8V[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW`@Qa\x0C,\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17aZ\xADW\x90\x82\x91am\xDD\x839\x03\x90\x82\xF0\x80\x15aZsW`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U`@Qa\x01\xF6\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17aZ\xADW\x90\x82\x91az\t\x839\x03\x90\x82\xF0\x80\x15aZsW`\x01`\x01`\xA0\x1B\x03\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x91`@Q\x92a\x19\x99\x92\x83\x85\x01\x93\x85\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17aZ\x80W\x91\x85\x93\x91`\xA0\x95\x93a{\xFF\x869\x83R` \x83\x01R`@\x82\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0``\x82\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`\x80\x82\x01R\x03\x01\x90\x82\xF0\x80\x15aZsW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU\x80`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x06\x8AW\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rj\x08E\x95\x16\x14\x01HJ\0\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\n\x97WP\xF3[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x87\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P4a\x02uW\x80`\x03\x196\x01\x12a\x02uW\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa\\\xCCW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\n\xD2W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7Fc\"\x14\x90\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rj\x01\xA7\x847\x9D\x99\xDBB\0\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05\x97Wa\\\xB7W[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F6\xB0\x89\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05\x97W\x82\x91a\\\x82W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rj\x01\xA7\x847\x9D\x99\xDBB\0\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05\x97Wa\n\x97WP\xF3[\x91PP` \x81=` \x11a\\\xAFW[\x81a\\\x9E` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BW\x81\x90Q_a\\\0V[=\x91Pa\\\x91V[\x81a\\\xC1\x91ab\x19V[a\x02uW\x80_a[\xB2V[\x81a\\\xD6\x91ab\x19V[a\x02uW\x80_a[NV[\x90P4a\x05\x8BW_`\x03\x196\x01\x12a\x05\x8BW`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BWc\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a`\x1AWa`\x07W[P\x80` `\x01`\x01`\xA0\x1B\x03\x81T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x84\x01RZ\xF1\x80\x15a\x05\x97Wa_\xD0W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05\x97Wa_\xBBW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03` T\x16\x90\x80;\x15a\x06\x8AW`@Qc\x06-\xA2\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05\x97Wa_\xA6W[P`\x04```\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xFB\x8CKQ\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05\x97W\x82\x83\x90\x84\x92a_yW[a^\xF8\x92\x93Pa76\x90ag\xF5V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\n\xD2W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rj\x03O\x08o;3\xB6\x84\0\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05\x97Wa\n\x97WP\xF3[PPPa^\xF8a_\x9Aa76\x92``=``\x11a7qWa7]\x81\x83ab\x19V[\x91\x93P\x90\x91P\x82a^\xE9V[\x81a_\xB0\x91ab\x19V[a\x02uW\x80_a^\x99V[\x81a_\xC5\x91ab\x19V[a\x02uW\x80_a^\"V[` \x81=` \x11a_\xFFW[\x81a_\xE9` \x93\x83ab\x19V[\x81\x01\x03\x12a\x06!Wa_\xFA\x90ab<V[a]\xBEV[=\x91Pa_\xDCV[a`\x13\x91P_\x90ab\x19V[__a]QV[`@Q=_\x82>=\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a`HWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a`;V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a`\xB7WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a`\xD5\x83`\x1F\x19\x86`\x01\x96\x03\x01\x87R\x89Qa`gV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a`\xA8V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10aa\x01WPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a`\xF4V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aakWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aa\xC1\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a`\xE4V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aa\\V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aa\xECW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aa\xECW`@RV[Q\x90\x81\x15\x15\x82\x03a\x05\x8BWV[\x90\x81``\x91\x03\x12a\x05\x8BW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x91\x90\x82\x03\x91\x82\x11abqWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11aa\xECW`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15ac\xAFW[` \x85\x10\x84\x14ac\x82W\x84\x87R\x86\x93\x90\x81\x15acBWP`\x01\x14ab\xFEW[Pab\xFC\x92P\x03\x83ab\x19V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10ac&WPP\x90` ab\xFC\x92\x82\x01\x01_ab\xEFV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92ac\rV[` \x93Pab\xFC\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_ab\xEFV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93ab\xD0V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10ae\xD0Wab\xFC\x94T\x91\x81\x81\x10ae\x9AW[\x81\x81\x10aedW[\x81\x81\x10ae.W[\x81\x81\x10ad\xF8W[\x81\x81\x10ad\xC2W[\x81\x81\x10ad\x8CW[\x81\x81\x10adWW[\x10ad*W[P\x03\x83ab\x19V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_ad\"V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01ad\x1CV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01ad\x14V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01ad\x0CV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01ad\x04V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01ac\xFCV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01ac\xF4V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01ac\xECV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91ac\xD4V[\x91\x90\x82\x01\x80\x92\x11abqWV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11aa\xECW`@Q\x91af\x94`\x1F\x82\x01`\x1F\x19\x16` \x01\x84ab\x19V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x05\x8BW\x82\x81` \x93\x84_\x96\x01^\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x05\x8BW\x81Qaf\xCA\x92` \x01afjV[\x90V[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x05\x8BWV[`@Q\x90af\xF0`@\x83ab\x19V[`\x0B\x82R\x7FMock Bridge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RV[`\x08T`\xFF\x16\x80\x15ag+W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a`\x1AW_\x91ag\xC3W[P\x15\x15\x90V[\x90P` \x81=` \x11ag\xEDW[\x81ag\xDE` \x93\x83ab\x19V[\x81\x01\x03\x12a\x05\x8BWQ_ag\xBDV[=\x91Pag\xD1V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[_ab\xFC\x91ab\x19V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BWa$\xFA_\x91aj\xCB`@Q\x94\x85\x93\x84\x93\x7F\xF3 \xD9c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`@`\x04\x86\x01R`D\x85\x01\x90a`gV[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`\x01`\x01`\xA0\x1B\x03\x90\x81`@Q\x93\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R\x16`\x04\x84\x01R\x16`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x8BW`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a`\x1AWahuWPV[\x81\x15ak\xEFW\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x90\x83\x83\x11amXW\x82\x81\x10\x91\x82\x15\x80amNW[amFWal?\x84\x86abdV[\x92`\x01\x84\x01\x80\x94\x11abqW`\x03\x83\x11\x15\x80am=W[am.W`\x03\x19\x83\x10\x15\x80am$W[am\x13W\x85\x83\x11\x15al\xCAWPP\x90al\x82\x84al\x87\x93abdV[ak\xE5V[\x90\x81\x15al\xC5Wal\x98\x92Paf]V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11abqW\x90V[PP\x90V[\x95\x94\x92\x91\x90\x95al\xDBW[PPPPV[\x83\x94\x95Pal\x82\x90al\xED\x93\x94abdV[\x90\x81\x15al\xC5Wal\xFE\x92PabdV[`\x01\x81\x01\x80\x91\x11abqW\x90_\x80\x80\x80al\xD5V[PP\x90Paf\xCA\x92\x91P\x19\x90abdV[P\x82\x19\x84\x11alfV[PP\x91\x90Paf\xCA\x92Paf]V[P\x82\x84\x11alVV[P\x92PPP\x90V[P\x84\x82\x11\x15al1V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R\xFD\xFE`\x80`@R4a\x03\x13W`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17a\x02&W`@\x90\x81R`\t\x82RhERC20Mock`\xB8\x1B` \x83\x01R\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17a\x02&W`@R`\x04\x81RcE20M`\xE0\x1B` \x82\x01R\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x02&W`\x03T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x03\tW[` \x82\x10\x14a\x02\x08W`\x1F\x81\x11a\x02\xA6W[P` \x92`\x1F\x82\x11`\x01\x14a\x02EW\x92\x81\x92\x93_\x92a\x02:W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x03U[\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x02&W`\x04T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x02\x1CW[` \x82\x10\x14a\x02\x08W`\x1F\x81\x11a\x01\xA5W[P` \x91`\x1F\x82\x11`\x01\x14a\x01EW\x91\x81\x92_\x92a\x01:W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x04U[`@Qa\t\x14\x90\x81a\x03\x18\x829\xF3[\x01Q\x90P_\x80a\x01\x16V[`\x1F\x19\x82\x16\x92`\x04_R\x80_ \x91_[\x85\x81\x10a\x01\x8DWP\x83`\x01\x95\x10a\x01uW[PPP\x81\x1B\x01`\x04Ua\x01+V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x01gV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x01UV[`\x04_R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B`\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x01\xFEW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x01\xF3WPa\0\xFDV[_\x81U`\x01\x01a\x01\xE6V[\x90\x91P\x81\x90a\x01\xDDV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x7F\x16\x90a\0\xEBV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x01Q\x90P_\x80a\0\xB5V[`\x1F\x19\x82\x16\x93`\x03_R\x80_ \x91_[\x86\x81\x10a\x02\x8EWP\x83`\x01\x95\x96\x10a\x02vW[PPP\x81\x1B\x01`\x03Ua\0\xCAV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02hV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x02UV[`\x03_R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[`\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x02\xFFW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x02\xF4WPa\0\x9BV[_\x81U`\x01\x01a\x02\xE7V[\x90\x91P\x81\x90a\x02\xDEV[\x90`\x7F\x16\x90a\0\x89V[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x06\xFD\xDE\x03\x14a\x07\x03WP\x80c\t^\xA7\xB3\x14a\x06tW\x80c\x18\x16\r\xDD\x14a\x06WW\x80c#\xB8r\xDD\x14a\x04\xE1W\x80c1<\xE5g\x14a\x04\xC6W\x80c@\xC1\x0F\x19\x14a\x03\xE5W\x80cp\xA0\x821\x14a\x03\xA1W\x80c\x95\xD8\x9BA\x14a\x02&W\x80c\x9D\xC2\x9F\xAC\x14a\x018W\x80c\xA9\x05\x9C\xBB\x14a\x01\x07Wc\xDDb\xED>\x14a\0\x95W_\x80\xFD[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\0\xAEa\x08\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\xCBa\x08'V[\x91\x16_R`\x01` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `@_ T`@Q\x90\x81R\xF3[_\x80\xFD[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x01-a\x01#a\x08\x04V[`$5\x903a\x08JV[` `@Q`\x01\x81R\xF3[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x01Qa\x08\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x91\x16\x80\x15a\x01\xFAW\x80_R_` R`@_ T\x82\x81\x10a\x01\xC8W` \x83_\x94\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93\x85\x87R\x86\x84R\x03`@\x86 U\x80`\x02T\x03`\x02U`@Q\x90\x81R\xA3\0[\x90\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`DR`d_\xFD[\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W`@Q_`\x04T\x90\x81`\x01\x1C`\x01\x83\x16\x92\x83\x15a\x03\x97W[` \x82\x10\x84\x14a\x03jW\x81\x85R\x84\x93\x90\x81\x15a\x03(WP`\x01\x14a\x02\xCCW[P\x03`\x1F\x01`\x1F\x19\x16\x81\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x81\x83\x10\x17a\x02\x9FWa\x02\x9B\x82\x91\x82`@R\x82a\x07\xDAV[\x03\x90\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x04_\x90\x81R\x91P\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B[\x81\x83\x10a\x03\x0CWPP\x81\x01` \x01`\x1F\x19a\x02mV[` \x91\x93P\x80`\x01\x91T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x91\x83\x92a\x02\xF6V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x85\x81\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91P`\x1F\x19\x90Pa\x02mV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x90`\x7F\x16\x90a\x02NV[4a\x01\x03W` `\x03\x196\x01\x12a\x01\x03Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x03\xCFa\x08\x04V[\x16_R_` R` `@_ T`@Q\x90\x81R\xF3[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x03\xFEa\x08\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`$5\x81\x15a\x04\x9AW`\x02T\x90\x80\x82\x01\x80\x92\x11a\x04mW` \x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91_\x93`\x02U\x84\x84R\x83\x82R`@\x84 \x81\x81T\x01\x90U`@Q\x90\x81R\xA3\0[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W` `@Q`\x12\x81R\xF3[4a\x01\x03W```\x03\x196\x01\x12a\x01\x03Wa\x04\xFAa\x08\x04V[a\x05\x02a\x08'V[`D5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x92\x83_R`\x01` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`@_ T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x10a\x05~W[Pa\x01-\x93Pa\x08JV[\x83\x81\x10a\x06#W\x84\x15a\x05\xF7W3\x15a\x05\xCBWa\x01-\x94_R`\x01` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R\x83`@_ \x91\x03\x90U\x84a\x05sV[\x7F\x94(\rb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\xE6\x02\xDF\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x83\x90\x7F\xFB\x8FA\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`DR`d_\xFD[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W` `\x02T`@Q\x90\x81R\xF3[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x06\x8Da\x08\x04V[`$5\x903\x15a\x05\xF7Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x05\xCBW3_R`\x01` R`@_ \x82_R` R\x80`@_ U`@Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` 3\x92\xA3` `@Q`\x01\x81R\xF3[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W_`\x03T\x90\x81`\x01\x1C`\x01\x83\x16\x92\x83\x15a\x07\xD0W[` \x82\x10\x84\x14a\x03jW\x81\x85R\x84\x93\x90\x81\x15a\x03(WP`\x01\x14a\x07tWP\x03`\x1F\x01`\x1F\x19\x16\x81\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x81\x83\x10\x17a\x02\x9FWa\x02\x9B\x82\x91\x82`@R\x82a\x07\xDAV[`\x03_\x90\x81R\x91P\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[[\x81\x83\x10a\x07\xB4WPP\x81\x01` \x01`\x1F\x19a\x02mV[` \x91\x93P\x80`\x01\x91T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x91\x83\x92a\x07\x9EV[\x90`\x7F\x16\x90a\x07(V[`\x1F\x19`\x1F` `@\x94\x81\x85R\x80Q\x91\x82\x91\x82\x82\x88\x01R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\x03WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\x03WV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x01\xFAWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x82\x15a\x04\x9AW\x81_R_` R`@_ T\x81\x81\x10a\x08\xE2W\x81\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92` \x92\x85_R_\x84R\x03`@_ U\x84_R_\x82R`@_ \x81\x81T\x01\x90U`@Q\x90\x81R\xA3V[\x82\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`DR`d_\xFD`\x80\x80`@R4`\x15Wa\x01\xDC\x90\x81a\0\x1A\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81ch\x13\xD7\x87\x14a\x01qW\x81c\xBC\xDB\x83\xD8\x14a\0\x80WPc\xD3\x07-\x82\x14a\0=W_\x80\xFD[4a\0|W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0|W` `\xFF_T\x16`@Q\x90\x15\x15\x81R\xF3[_\x80\xFD[4a\0|W``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0|W`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\0|W`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0|W6`#\x82\x01\x12\x15a\0|W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0|W6\x91\x01`$\x01\x11a\0|W`\xFF_T\x16a\x01\x15W\0[\x80\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x92R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FTarget bridge failed\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[4a\0|W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\0|W`\x045\x80\x15\x15\x80\x91\x03a\0|W`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0_T\x16\x91\x16\x17_U_\x80\xF3`\x80`@R4a\x02\xA3W`@Q`\x1Fa\x19\x998\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x02oW\x80\x84\x92`\xA0\x94`@R\x839\x81\x01\x03\x12a\x02\xA3Wa\0M\x81a\x02\xA7V[a\0Y` \x83\x01a\x02\xA7V[\x91a\0f`@\x82\x01a\x02\xA7V[\x91`\x80``\x83\x01Q\x92\x01Q\x90`@Q\x94`@\x86\x01\x86\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\x02oW`@R`\x0B\x86RjMock Bridge`\xA8\x1B` \x87\x01R`\x01\x80U`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15a\x02\x92W[a\x02\x83Wa\0\xD6\x82a\0\xD0a\0\xDC\x94a\x02\xBBV[Pa\x031V[Pa\x03\xC4V[P\x83Q`\x01`\x01`@\x1B\x03\x81\x11a\x02oW`\x07T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x02eW[` \x82\x10\x14a\x02QW`\x1F\x81\x11a\x01\xEEW[P` \x94`\x1F\x82\x11`\x01\x14a\x01\x8BW\x94\x81\x92\x93\x94\x95_\x92a\x01\x80W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x07U[`\x02T\x91`\x03U`\x04U`\x01`\xA0\x1B\x91`\x01\x80`\xA0\x1B\x03\x16\x90`\x01\x80`\xA8\x1B\x03\x19\x16\x17\x17`\x02Ub\x01Q\x80B\x04`\x06U`@Qa\x14\xC1\x90\x81a\x04X\x829\xF3[\x01Q\x90P_\x80a\x01,V[`\x1F\x19\x82\x16\x95`\x07_R\x80_ \x91_[\x88\x81\x10a\x01\xD6WP\x83`\x01\x95\x96\x97\x98\x10a\x01\xBEW[PPP\x81\x1B\x01`\x07Ua\x01AV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x01\xB0V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x01\x9BV[`\x07_R\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88`\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x02GW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x02<WPa\x01\x10V[_\x81U`\x01\x01a\x02/V[\x90\x91P\x81\x90a\x02&V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x7F\x16\x90a\0\xFEV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\0\xBCV[_\x80\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xA3WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x19y_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x03,W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x19y_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a\x19\x19_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x19Y_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x03,W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x19Y_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7Fu\x1By]$\xB9.=\x92\xD1\xD0\xD8\xF2\x88_N\x9C\x9C&\x9D\xA3P\xAF6\xAEkI\x06\x9B\xAB\xF4\xBF\x90_Q` a\x19\x19_9_Q\x90_R\x90\x80\xA4`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x199_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x03,W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x199_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\xC0t\xBE\xB4k%\x1Fs\xC0\xFE\xC1n\xBAu\xD6\xBC\0B\xD0\x9E\xE1zt\x0E\xBB\xEE0\x8D\xBF\x87\xF8/\x90_Q` a\x19\x19_9_Q\x90_R\x90\x80\xA4`\x01\x90V\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x01\xFF\xC9\xA7\x14a\x0E\xDFWP\x80c\x103\xB4\xCC\x14a\x0E\xC2W\x80c\x11\x8C8\xC7\x14a\x0E\x88W\x80c\x12Y\xA5\xC8\x14a\x0EkW\x80c\x18\xB6\x8B\x8C\x14a\tQW\x80c$\x8A\x9C\xA3\x14a\t\x1FW\x80c//\xF1]\x14a\x08\xE2W\x80c4b\xFA\xC3\x14a\x08\xA8W\x80c6V\x8A\xBE\x14a\x08>W\x80c6\xB0\x89\xD8\x14a\x08!W\x80cZ\xB1\xD6\x1C\x14a\x07\x89W\x80ca\xB0\xA5n\x14a\x06~W\x80cc\"\x14\x90\x14a\x06]W\x80ce\xD7\xA3\xC9\x14a\x06BW\x80cg\xEE\xBA\x0C\x14a\x06%W\x80ch\x13\xD7\x87\x14a\x05\xD6W\x80ck\xCC\x8C\x14\x14a\x05\nW\x80c\x91\xD1HT\x14a\x04\xB4W\x80c\xA2\x17\xFD\xDF\x14a\x04\x9AW\x80c\xB1nxI\x14a\x04gW\x80c\xB2\r0\xA9\x14a\x04\x13W\x80c\xC9\xF5\xB6>\x14a\x03\xE0W\x80c\xCC=\xC0a\x14a\x02\xE0W\x80c\xD3\x07-\x82\x14a\x02\xBEW\x80c\xD5Gt\x1F\x14a\x02zW\x80c\xEA\xD9<\x8F\x14a\x02UW\x80c\xED\xE7\xCE\xBD\x14a\x01\xF4W\x80c\xF6\x81\xA8b\x14a\x01\xD7Wc\xFB\x8CKQ\x14a\x01[W_\x80\xFD[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W`\x05T`\x04T\x90\x80\x82\x11\x15a\x01\xC9W\x80\x82\x03\x91\x80\x83\x11a\x01\x9CW``\x92\x90[`@Q\x92\x83R` \x83\x01R`@\x82\x01R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[``\x91_\x90a\x01\x8AV[_\x80\xFD[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\tT`@Q\x90\x81R\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3Wa\x02)`\x02Ta\x02\x12a\x10\x82V[\x90`\xFF`@Q\x93\x84\x93``\x85R``\x85\x01\x90a\x11WV[\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16` \x85\x01R`\xA0\x1C\x16\x15\x15`@\x83\x01R\x03\x90\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\xFF`\x02T`\xA0\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x02\xBC`\x045a\x02\x99a\x0F\xA0V[\x90a\x02\xB7a\x02\xB2\x82_R_` R`\x01`@_ \x01T\x90V[a\x12/V[a\x13gV[\0[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\xFF`\x08T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W`@Q_`\x0BTa\x03\0\x81a\x0F\xC3V[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x03\x9EWP`\x01\x14a\x03@W[a\x03<\x83a\x03(\x81\x85\x03\x82a\x10\x14V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x11WV[\x03\x90\xF3[\x91\x90P`\x0B_R\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x91_\x90[\x80\x82\x10a\x03\x84WP\x90\x91P\x81\x01` \x01a\x03(a\x03\x18V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x03lV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x03(\x90Pa\x03\x18V[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16`@Q\x90\x81R\xF3[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W\x7F |L\xBD\xF5^\xC3\x15\xA1?\r^\x04w2\xEC]\x94}\xA0V\xE7\x06Y:\xA5\t\x90\x99A\xCE\xDF`@`\x045a\x04Ra\x11\xA7V[`\x04T\x90\x80`\x04U\x82Q\x91\x82R` \x82\x01R\xA1\0[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\nT\x16`@Q\x90\x81R\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `@Q_\x81R\xF3[4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x04\xCDa\x0F\xA0V[`\x045_R_` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x058a\x0F}V[a\x05@a\x11\xA7V[\x16\x80\x15a\x05\xAEWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17`\x02U\x16\x7F\xB0\x7F\x8B\x1B\x85\x04-t\x02,\x86|\x83n\xDE\xB0\xBC\xD7\x0E\x13[\0B9\r+\x1F\xD1\x08)\x80i_\x80\xA3\0[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W`\x045\x80\x15\x15\x80\x91\x03a\x01\xD3W`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x08T\x16\x91\x16\x17`\x08U_\x80\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\x04T`@Q\x90\x81R\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3Wa\x03<a\x03(a\x10\x82V[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3Wa\x06va\x11\xA7V[`\x045`\x03U\0[4a\x01\xD3W```\x03\x196\x01\x12a\x01\xD3Wa\x06\x97a\x0F}V[`D5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x01\xD3W3_\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16\x15a\x07YW\x80\x15a\x05\xAEWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x02\xBC\x92`@Q\x92\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x85\x01R`$\x84\x01R`$5`D\x84\x01R`D\x83Ra\x07S`d\x84a\x10\x14V[\x16a\x14/V[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W`\x045\x80\x15\x15\x80\x91\x03a\x01\xD3W` \x7F\xB3A\x89\x89\xD0h5\xB5\xC2\x15\xEE\xBBMT\xEDk\xE7\xBB\xB6n\xB4\x80qdt\n.\x08/\xA7\x82\xD5\x91a\x07\xD3a\x11\xA7V[`\x02T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\xA0\x1B\x16\x91\x16\x17`\x02U`@Q\x90\x81R\xA1\0[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\x03T`@Q\x90\x81R\xF3[4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x08Wa\x0F\xA0V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\x08\x80Wa\x02\xBC\x90`\x045a\x13gV[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `@Q\x7F\xC0t\xBE\xB4k%\x1Fs\xC0\xFE\xC1n\xBAu\xD6\xBC\0B\xD0\x9E\xE1zt\x0E\xBB\xEE0\x8D\xBF\x87\xF8/\x81R\xF3[4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x02\xBC`\x045a\t\x01a\x0F\xA0V[\x90a\t\x1Aa\x02\xB2\x82_R_` R`\x01`@_ \x01T\x90V[a\x12\x95V[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W` a\tI`\x045_R_` R`\x01`@_ \x01T\x90V[`@Q\x90\x81R\xF3[4a\x01\xD3W```\x03\x196\x01\x12a\x01\xD3Wa\tja\x0F}V[`$5`D5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xD3W6`#\x84\x01\x12\x15a\x01\xD3W\x82`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xD3W6`$\x83\x86\x01\x01\x11a\x01\xD3W`\x02`\x01T\x14a\x0ECW`\x02`\x01U`\xFF`\x02T`\xA0\x1C\x16\x15a\x0E\x1BW3_\x90\x81R\x7F\xFE\x0C{\xD5\t,znF4\x88\xFA#\x9F2\x05\x8C\x87\xD4>3n.XJ'!2\xCF\x8D\xC9&` R`@\x90 T`\xFF\x16\x15a\r\xF3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x05\xAEW\x82\x15a\r\xCBW`\x03T\x83\x11a\rdWb\x01Q\x80B\x04`\x06T\x81\x11a\r\x8CW[Pa\nH\x83`\x05Ta\x11\x9AV[`\x04T\x10a\rdWa\n\xA3`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R3`$\x82\x01R0`D\x82\x01R\x84`d\x82\x01R`d\x81Ra\n\x9D`\x84\x82a\x10\x14V[\x83a\x14/V[`\xFF`\x08T\x16a\r\x06W\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\nT\x16\x17`\nU\x82`\tUa\n\xE6`\x0BTa\x0F\xC3V[`\x1F\x81\x11a\x0CeW[P_`\x1F\x82\x11`\x01\x14a\x0B\xA1W\x81\x92\x93\x94_\x92a\x0B\x93W[PP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17`\x0BU[a\x0BF\x82`\x05Ta\x11\x9AV[`\x05U\x7F=\xBA@\x1D\xC1\xAB\xBF\x01p\x86\x86K3\xBA\x81_S\xAB<\xDB\xB96k\xA6\xBC\xD8\xEC4-\xDD\x98\xE8` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16\x93`@Q\x90\x81R\xA3`\x01\x80U\0[`$\x92P\x01\x015\x84\x80a\x0B\x07V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x16\x94`\x0B_R\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x91_[\x87\x81\x10a\x0CJWP\x83`\x01\x95\x96\x97\x10a\x0C\x0FW[PPP\x81\x1B\x01`\x0BUa\x0B:V[\x01`$\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x84\x90\x1B`\xF8\x16\x1C\x19\x16\x90U\x84\x80\x80a\x0C\x01V[\x90\x92` `\x01\x81\x92`$\x87\x87\x01\x015\x81U\x01\x94\x01\x91\x01a\x0B\xEDV[`\x0B_R`\x1F\x82\x01`\x05\x1C\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x01\x90` \x83\x10a\x0C\xDEW[`\x1F\x01`\x05\x1C\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x01\x90[\x81\x81\x10a\x0C\xD3WPa\n\xEFV[_\x81U`\x01\x01a\x0C\xC6V[\x7F\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB9\x91Pa\x0C\x9CV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FMock bridge execution failed\0\0\0\0`D\x82\x01R\xFD[\x7Fp\xD1h\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xDAN9\xDDV\xD7,.\xE3\xD12\xE0\x14k\xC3\x9E\x90^x\xE3\xBCd\xC4\x01\x90B\x1C{+\xCE\xF2\xAB`@`\x05T\x81Q\x90\x84\x82R` \x82\x01R\xA1_`\x05U`\x06U\x84a\n;V[\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\\B|\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F{\xEA \xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\x06T`@Q\x90\x81R\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `@Q\x7Fu\x1By]$\xB9.=\x92\xD1\xD0\xD8\xF2\x88_N\x9C\x9C&\x9D\xA3P\xAF6\xAEkI\x06\x9B\xAB\xF4\xBF\x81R\xF3[4a\x01\xD3W_`\x03\x196\x01\x12a\x01\xD3W` `\x05T`@Q\x90\x81R\xF3[4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W`\x045\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x80\x92\x03a\x01\xD3W\x81\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x93\x14\x90\x81\x15a\x0FSW[P\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x83a\x0FLV[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xD3WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xD3WV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x10\nW[` \x83\x10\x14a\x0F\xDDWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x0F\xD2V[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10UW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q\x90_\x82`\x07T\x91a\x10\x95\x83a\x0F\xC3V[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a\x11\x1AWP`\x01\x14a\x10\xBBW[a\x10\xB9\x92P\x03\x83a\x10\x14V[V[P`\x07_\x90\x81R\x90\x91\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88[\x81\x83\x10a\x10\xFEWPP\x90` a\x10\xB9\x92\x82\x01\x01a\x10\xADV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x10\xE6V[` \x92Pa\x10\xB9\x94\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01a\x10\xADV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x91\x90\x82\x01\x80\x92\x11a\x01\x9CWV[3_\x90\x81R\x7F\xDF\xFF\x91\xBC\xEE\x88\xF6\xEA\0\xB2rom\x06*P\x9C2\x83W\x93G\x0CU\xBE\xD3\xBC\xC7L\x0F]7` R`@\x90 T`\xFF\x16\x15a\x11\xDFWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7Fu\x1By]$\xB9.=\x92\xD1\xD0\xD8\xF2\x88_N\x9C\x9C&\x9D\xA3P\xAF6\xAEkI\x06\x9B\xAB\xF4\xBF`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x12fWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x13aW\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x13aW\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V[\x90_` \x91\x82\x81Q\x91\x01\x82\x85Z\xF1\x15a\x14\xB6W_Q=a\x14\xADWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;\x15[a\x14kWPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[`\x01\x14\x15a\x14dV[`@Q=_\x82>=\x90\xFD/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\xFE\x0C{\xD5\t,znF4\x88\xFA#\x9F2\x05\x8C\x87\xD4>3n.XJ'!2\xCF\x8D\xC9&\xDF\xFF\x91\xBC\xEE\x88\xF6\xEA\0\xB2rom\x06*P\x9C2\x83W\x93G\x0CU\xBE\xD3\xBC\xC7L\x0F]7\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `BridgeExecuted(address,uint256,address)` and selector `0x3dba401dc1abbf017086864b33ba815f53ab3cdbb9366ba6bcd8ec342ddd98e8`.
```solidity
event BridgeExecuted(address indexed token, uint256 amount, address indexed target);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BridgeExecuted {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for BridgeExecuted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "BridgeExecuted(address,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                61u8, 186u8, 64u8, 29u8, 193u8, 171u8, 191u8, 1u8, 112u8, 134u8, 134u8,
                75u8, 51u8, 186u8, 129u8, 95u8, 83u8, 171u8, 60u8, 219u8, 185u8, 54u8,
                107u8, 166u8, 188u8, 216u8, 236u8, 52u8, 45u8, 221u8, 152u8, 232u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    token: topics.1,
                    amount: data.0,
                    target: topics.2,
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
                (Self::SIGNATURE_HASH.into(), self.token.clone(), self.target.clone())
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
                    &self.token,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.target,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BridgeExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BridgeExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BridgeExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `BridgeStatusUpdated(bool)` and selector `0xb3418989d06835b5c215eebb4d54ed6be7bbb66eb4807164740a2e082fa782d5`.
```solidity
event BridgeStatusUpdated(bool active);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BridgeStatusUpdated {
        #[allow(missing_docs)]
        pub active: bool,
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
        impl alloy_sol_types::SolEvent for BridgeStatusUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "BridgeStatusUpdated(bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                179u8, 65u8, 137u8, 137u8, 208u8, 104u8, 53u8, 181u8, 194u8, 21u8, 238u8,
                187u8, 77u8, 84u8, 237u8, 107u8, 231u8, 187u8, 182u8, 110u8, 180u8,
                128u8, 113u8, 100u8, 116u8, 10u8, 46u8, 8u8, 47u8, 167u8, 130u8, 213u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { active: data.0 }
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
                        &self.active,
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
        impl alloy_sol_types::private::IntoLogData for BridgeStatusUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BridgeStatusUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BridgeStatusUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `BridgeTargetUpdated(address,address)` and selector `0xb07f8b1b85042d74022c867c836edeb0bcd70e135b0042390d2b1fd108298069`.
```solidity
event BridgeTargetUpdated(address indexed oldTarget, address indexed newTarget);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BridgeTargetUpdated {
        #[allow(missing_docs)]
        pub oldTarget: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newTarget: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for BridgeTargetUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "BridgeTargetUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                176u8, 127u8, 139u8, 27u8, 133u8, 4u8, 45u8, 116u8, 2u8, 44u8, 134u8,
                124u8, 131u8, 110u8, 222u8, 176u8, 188u8, 215u8, 14u8, 19u8, 91u8, 0u8,
                66u8, 57u8, 13u8, 43u8, 31u8, 209u8, 8u8, 41u8, 128u8, 105u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldTarget: topics.1,
                    newTarget: topics.2,
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
                    self.oldTarget.clone(),
                    self.newTarget.clone(),
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
                    &self.oldTarget,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newTarget,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BridgeTargetUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BridgeTargetUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BridgeTargetUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DailyLimitReset(uint256,uint256)` and selector `0xda4e39dd56d72c2ee3d132e0146bc39e905e78e3bc64c40190421c7b2bcef2ab`.
```solidity
event DailyLimitReset(uint256 day, uint256 previousUsed);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DailyLimitReset {
        #[allow(missing_docs)]
        pub day: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub previousUsed: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for DailyLimitReset {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "DailyLimitReset(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                218u8, 78u8, 57u8, 221u8, 86u8, 215u8, 44u8, 46u8, 227u8, 209u8, 50u8,
                224u8, 20u8, 107u8, 195u8, 158u8, 144u8, 94u8, 120u8, 227u8, 188u8,
                100u8, 196u8, 1u8, 144u8, 66u8, 28u8, 123u8, 43u8, 206u8, 242u8, 171u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    day: data.0,
                    previousUsed: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.day),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.previousUsed),
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
        impl alloy_sol_types::private::IntoLogData for DailyLimitReset {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DailyLimitReset> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DailyLimitReset) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DailyLimitUpdated(uint256,uint256)` and selector `0x207c4cbdf55ec315a13f0d5e047732ec5d947da056e706593aa509909941cedf`.
```solidity
event DailyLimitUpdated(uint256 oldLimit, uint256 newLimit);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DailyLimitUpdated {
        #[allow(missing_docs)]
        pub oldLimit: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newLimit: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for DailyLimitUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "DailyLimitUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                32u8, 124u8, 76u8, 189u8, 245u8, 94u8, 195u8, 21u8, 161u8, 63u8, 13u8,
                94u8, 4u8, 119u8, 50u8, 236u8, 93u8, 148u8, 125u8, 160u8, 86u8, 231u8,
                6u8, 89u8, 58u8, 165u8, 9u8, 144u8, 153u8, 65u8, 206u8, 223u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldLimit: data.0,
                    newLimit: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.oldLimit),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newLimit),
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
        impl alloy_sol_types::private::IntoLogData for DailyLimitUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DailyLimitUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DailyLimitUpdated) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `DAILY_LIMIT()` and selector `0x248ec326`.
```solidity
function DAILY_LIMIT() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DAILY_LIMITCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`DAILY_LIMIT()`](DAILY_LIMITCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DAILY_LIMITReturn {
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
            impl ::core::convert::From<DAILY_LIMITCall> for UnderlyingRustTuple<'_> {
                fn from(value: DAILY_LIMITCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DAILY_LIMITCall {
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
            impl ::core::convert::From<DAILY_LIMITReturn> for UnderlyingRustTuple<'_> {
                fn from(value: DAILY_LIMITReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DAILY_LIMITReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DAILY_LIMITCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DAILY_LIMIT()";
            const SELECTOR: [u8; 4] = [36u8, 142u8, 195u8, 38u8];
            #[inline]
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
                        let r: DAILY_LIMITReturn = r.into();
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
                        let r: DAILY_LIMITReturn = r.into();
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
    /**Function with signature `MAX_SINGLE_TRANSFER()` and selector `0x117e3b42`.
```solidity
function MAX_SINGLE_TRANSFER() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_SINGLE_TRANSFERCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`MAX_SINGLE_TRANSFER()`](MAX_SINGLE_TRANSFERCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_SINGLE_TRANSFERReturn {
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
            impl ::core::convert::From<MAX_SINGLE_TRANSFERCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_SINGLE_TRANSFERCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_SINGLE_TRANSFERCall {
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
            impl ::core::convert::From<MAX_SINGLE_TRANSFERReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_SINGLE_TRANSFERReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_SINGLE_TRANSFERReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_SINGLE_TRANSFERCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_SINGLE_TRANSFER()";
            const SELECTOR: [u8; 4] = [17u8, 126u8, 59u8, 66u8];
            #[inline]
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
                        let r: MAX_SINGLE_TRANSFERReturn = r.into();
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
                        let r: MAX_SINGLE_TRANSFERReturn = r.into();
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
    /**Function with signature `bridgeProxy()` and selector `0xa3d4485b`.
```solidity
function bridgeProxy() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bridgeProxyCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`bridgeProxy()`](bridgeProxyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bridgeProxyReturn {
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
            impl ::core::convert::From<bridgeProxyCall> for UnderlyingRustTuple<'_> {
                fn from(value: bridgeProxyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bridgeProxyCall {
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
            impl ::core::convert::From<bridgeProxyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bridgeProxyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bridgeProxyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bridgeProxyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bridgeProxy()";
            const SELECTOR: [u8; 4] = [163u8, 212u8, 72u8, 91u8];
            #[inline]
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
                        let r: bridgeProxyReturn = r.into();
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
                        let r: bridgeProxyReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `caller()` and selector `0xfc9c8d39`.
```solidity
function caller() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct callerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`caller()`](callerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct callerReturn {
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
            impl ::core::convert::From<callerCall> for UnderlyingRustTuple<'_> {
                fn from(value: callerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for callerCall {
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
            impl ::core::convert::From<callerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: callerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for callerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for callerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "caller()";
            const SELECTOR: [u8; 4] = [252u8, 156u8, 141u8, 57u8];
            #[inline]
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
                        let r: callerReturn = r.into();
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
                        let r: callerReturn = r.into();
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
    /**Function with signature `newTarget()` and selector `0x13217f90`.
```solidity
function newTarget() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newTargetCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`newTarget()`](newTargetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newTargetReturn {
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
            impl ::core::convert::From<newTargetCall> for UnderlyingRustTuple<'_> {
                fn from(value: newTargetCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for newTargetCall {
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
            impl ::core::convert::From<newTargetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: newTargetReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for newTargetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for newTargetCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "newTarget()";
            const SELECTOR: [u8; 4] = [19u8, 33u8, 127u8, 144u8];
            #[inline]
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
                        let r: newTargetReturn = r.into();
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
                        let r: newTargetReturn = r.into();
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
    /**Function with signature `targetBridge()` and selector `0x959b337d`.
```solidity
function targetBridge() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetBridgeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetBridge()`](targetBridgeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetBridgeReturn {
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
            impl ::core::convert::From<targetBridgeCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetBridgeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetBridgeCall {
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
            impl ::core::convert::From<targetBridgeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetBridgeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetBridgeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetBridgeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetBridge()";
            const SELECTOR: [u8; 4] = [149u8, 155u8, 51u8, 125u8];
            #[inline]
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
                        let r: targetBridgeReturn = r.into();
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
                        let r: targetBridgeReturn = r.into();
                        r._0
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
    /**Function with signature `testFuzz_ExecuteBridge_ValidAmounts(uint256)` and selector `0xcffb048b`.
```solidity
function testFuzz_ExecuteBridge_ValidAmounts(uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_ExecuteBridge_ValidAmountsCall {
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`testFuzz_ExecuteBridge_ValidAmounts(uint256)`](testFuzz_ExecuteBridge_ValidAmountsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_ExecuteBridge_ValidAmountsReturn {}
    #[allow(
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
            impl ::core::convert::From<testFuzz_ExecuteBridge_ValidAmountsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_ExecuteBridge_ValidAmountsCall) -> Self {
                    (value.amount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_ExecuteBridge_ValidAmountsCall {
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
            impl ::core::convert::From<testFuzz_ExecuteBridge_ValidAmountsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_ExecuteBridge_ValidAmountsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_ExecuteBridge_ValidAmountsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzz_ExecuteBridge_ValidAmountsReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzz_ExecuteBridge_ValidAmountsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzz_ExecuteBridge_ValidAmountsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzz_ExecuteBridge_ValidAmountsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzz_ExecuteBridge_ValidAmounts(uint256)";
            const SELECTOR: [u8; 4] = [207u8, 251u8, 4u8, 139u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzz_ExecuteBridge_ValidAmountsReturn::_tokenize(ret)
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
    /**Function with signature `testFuzz_SetDailyLimit_ValidValues(uint256)` and selector `0x29365968`.
```solidity
function testFuzz_SetDailyLimit_ValidValues(uint256 newLimit) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_SetDailyLimit_ValidValuesCall {
        #[allow(missing_docs)]
        pub newLimit: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`testFuzz_SetDailyLimit_ValidValues(uint256)`](testFuzz_SetDailyLimit_ValidValuesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_SetDailyLimit_ValidValuesReturn {}
    #[allow(
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
            impl ::core::convert::From<testFuzz_SetDailyLimit_ValidValuesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_SetDailyLimit_ValidValuesCall) -> Self {
                    (value.newLimit,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_SetDailyLimit_ValidValuesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newLimit: tuple.0 }
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
            impl ::core::convert::From<testFuzz_SetDailyLimit_ValidValuesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_SetDailyLimit_ValidValuesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_SetDailyLimit_ValidValuesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzz_SetDailyLimit_ValidValuesReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzz_SetDailyLimit_ValidValuesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzz_SetDailyLimit_ValidValuesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzz_SetDailyLimit_ValidValuesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzz_SetDailyLimit_ValidValues(uint256)";
            const SELECTOR: [u8; 4] = [41u8, 54u8, 89u8, 104u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.newLimit),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzz_SetDailyLimit_ValidValuesReturn::_tokenize(ret)
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
    /**Function with signature `test_Constructor_RoleAssignment()` and selector `0xdccc57f1`.
```solidity
function test_Constructor_RoleAssignment() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Constructor_RoleAssignmentCall;
    ///Container type for the return parameters of the [`test_Constructor_RoleAssignment()`](test_Constructor_RoleAssignmentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Constructor_RoleAssignmentReturn {}
    #[allow(
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
            impl ::core::convert::From<test_Constructor_RoleAssignmentCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Constructor_RoleAssignmentCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Constructor_RoleAssignmentCall {
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
            impl ::core::convert::From<test_Constructor_RoleAssignmentReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Constructor_RoleAssignmentReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Constructor_RoleAssignmentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Constructor_RoleAssignmentReturn {
            fn _tokenize(
                &self,
            ) -> <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Constructor_RoleAssignmentCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Constructor_RoleAssignmentReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Constructor_RoleAssignment()";
            const SELECTOR: [u8; 4] = [220u8, 204u8, 87u8, 241u8];
            #[inline]
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
                test_Constructor_RoleAssignmentReturn::_tokenize(ret)
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
    /**Function with signature `test_Constructor_Success()` and selector `0xb44dc9d6`.
```solidity
function test_Constructor_Success() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Constructor_SuccessCall;
    ///Container type for the return parameters of the [`test_Constructor_Success()`](test_Constructor_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Constructor_SuccessReturn {}
    #[allow(
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
            impl ::core::convert::From<test_Constructor_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Constructor_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Constructor_SuccessCall {
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
            impl ::core::convert::From<test_Constructor_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Constructor_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Constructor_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Constructor_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_Constructor_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Constructor_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Constructor_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Constructor_Success()";
            const SELECTOR: [u8; 4] = [180u8, 77u8, 201u8, 214u8];
            #[inline]
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
                test_Constructor_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_DailyLimit_Cumulative()` and selector `0x0f57280f`.
```solidity
function test_DailyLimit_Cumulative() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_DailyLimit_CumulativeCall;
    ///Container type for the return parameters of the [`test_DailyLimit_Cumulative()`](test_DailyLimit_CumulativeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_DailyLimit_CumulativeReturn {}
    #[allow(
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
            impl ::core::convert::From<test_DailyLimit_CumulativeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_DailyLimit_CumulativeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_DailyLimit_CumulativeCall {
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
            impl ::core::convert::From<test_DailyLimit_CumulativeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_DailyLimit_CumulativeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_DailyLimit_CumulativeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_DailyLimit_CumulativeReturn {
            fn _tokenize(
                &self,
            ) -> <test_DailyLimit_CumulativeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_DailyLimit_CumulativeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_DailyLimit_CumulativeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_DailyLimit_Cumulative()";
            const SELECTOR: [u8; 4] = [15u8, 87u8, 40u8, 15u8];
            #[inline]
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
                test_DailyLimit_CumulativeReturn::_tokenize(ret)
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
    /**Function with signature `test_DailyLimit_Reset()` and selector `0xd3075c49`.
```solidity
function test_DailyLimit_Reset() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_DailyLimit_ResetCall;
    ///Container type for the return parameters of the [`test_DailyLimit_Reset()`](test_DailyLimit_ResetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_DailyLimit_ResetReturn {}
    #[allow(
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
            impl ::core::convert::From<test_DailyLimit_ResetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_DailyLimit_ResetCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_DailyLimit_ResetCall {
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
            impl ::core::convert::From<test_DailyLimit_ResetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_DailyLimit_ResetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_DailyLimit_ResetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_DailyLimit_ResetReturn {
            fn _tokenize(
                &self,
            ) -> <test_DailyLimit_ResetCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_DailyLimit_ResetCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_DailyLimit_ResetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_DailyLimit_Reset()";
            const SELECTOR: [u8; 4] = [211u8, 7u8, 92u8, 73u8];
            #[inline]
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
                test_DailyLimit_ResetReturn::_tokenize(ret)
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
    /**Function with signature `test_ExecuteBridge_MultipleTransfers()` and selector `0x8b58cbae`.
```solidity
function test_ExecuteBridge_MultipleTransfers() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_MultipleTransfersCall;
    ///Container type for the return parameters of the [`test_ExecuteBridge_MultipleTransfers()`](test_ExecuteBridge_MultipleTransfersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_MultipleTransfersReturn {}
    #[allow(
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
            impl ::core::convert::From<test_ExecuteBridge_MultipleTransfersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteBridge_MultipleTransfersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_MultipleTransfersCall {
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
            impl ::core::convert::From<test_ExecuteBridge_MultipleTransfersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteBridge_MultipleTransfersReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_MultipleTransfersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_ExecuteBridge_MultipleTransfersReturn {
            fn _tokenize(
                &self,
            ) -> <test_ExecuteBridge_MultipleTransfersCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_ExecuteBridge_MultipleTransfersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_ExecuteBridge_MultipleTransfersReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_ExecuteBridge_MultipleTransfers()";
            const SELECTOR: [u8; 4] = [139u8, 88u8, 203u8, 174u8];
            #[inline]
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
                test_ExecuteBridge_MultipleTransfersReturn::_tokenize(ret)
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
    /**Function with signature `test_ExecuteBridge_ReentrancyProtection()` and selector `0xb55d42bc`.
```solidity
function test_ExecuteBridge_ReentrancyProtection() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_ReentrancyProtectionCall;
    ///Container type for the return parameters of the [`test_ExecuteBridge_ReentrancyProtection()`](test_ExecuteBridge_ReentrancyProtectionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_ReentrancyProtectionReturn {}
    #[allow(
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
            impl ::core::convert::From<test_ExecuteBridge_ReentrancyProtectionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteBridge_ReentrancyProtectionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_ReentrancyProtectionCall {
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
            impl ::core::convert::From<test_ExecuteBridge_ReentrancyProtectionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteBridge_ReentrancyProtectionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_ReentrancyProtectionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_ExecuteBridge_ReentrancyProtectionReturn {
            fn _tokenize(
                &self,
            ) -> <test_ExecuteBridge_ReentrancyProtectionCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_ExecuteBridge_ReentrancyProtectionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_ExecuteBridge_ReentrancyProtectionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_ExecuteBridge_ReentrancyProtection()";
            const SELECTOR: [u8; 4] = [181u8, 93u8, 66u8, 188u8];
            #[inline]
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
                test_ExecuteBridge_ReentrancyProtectionReturn::_tokenize(ret)
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
    /**Function with signature `test_ExecuteBridge_Success()` and selector `0xb9b5bd68`.
```solidity
function test_ExecuteBridge_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_SuccessCall;
    ///Container type for the return parameters of the [`test_ExecuteBridge_Success()`](test_ExecuteBridge_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_SuccessReturn {}
    #[allow(
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
            impl ::core::convert::From<test_ExecuteBridge_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteBridge_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_SuccessCall {
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
            impl ::core::convert::From<test_ExecuteBridge_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteBridge_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_ExecuteBridge_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_ExecuteBridge_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_ExecuteBridge_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_ExecuteBridge_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_ExecuteBridge_Success()";
            const SELECTOR: [u8; 4] = [185u8, 181u8, 189u8, 104u8];
            #[inline]
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
                test_ExecuteBridge_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_GetBridgeInfo()` and selector `0x956d9808`.
```solidity
function test_GetBridgeInfo() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetBridgeInfoCall;
    ///Container type for the return parameters of the [`test_GetBridgeInfo()`](test_GetBridgeInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetBridgeInfoReturn {}
    #[allow(
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
            impl ::core::convert::From<test_GetBridgeInfoCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetBridgeInfoCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetBridgeInfoCall {
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
            impl ::core::convert::From<test_GetBridgeInfoReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetBridgeInfoReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetBridgeInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_GetBridgeInfoReturn {
            fn _tokenize(
                &self,
            ) -> <test_GetBridgeInfoCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_GetBridgeInfoCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_GetBridgeInfoReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_GetBridgeInfo()";
            const SELECTOR: [u8; 4] = [149u8, 109u8, 152u8, 8u8];
            #[inline]
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
                test_GetBridgeInfoReturn::_tokenize(ret)
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
    /**Function with signature `test_GetDailyUsage_AfterTransfer()` and selector `0x089ca9e0`.
```solidity
function test_GetDailyUsage_AfterTransfer() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetDailyUsage_AfterTransferCall;
    ///Container type for the return parameters of the [`test_GetDailyUsage_AfterTransfer()`](test_GetDailyUsage_AfterTransferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetDailyUsage_AfterTransferReturn {}
    #[allow(
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
            impl ::core::convert::From<test_GetDailyUsage_AfterTransferCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetDailyUsage_AfterTransferCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetDailyUsage_AfterTransferCall {
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
            impl ::core::convert::From<test_GetDailyUsage_AfterTransferReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetDailyUsage_AfterTransferReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetDailyUsage_AfterTransferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_GetDailyUsage_AfterTransferReturn {
            fn _tokenize(
                &self,
            ) -> <test_GetDailyUsage_AfterTransferCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_GetDailyUsage_AfterTransferCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_GetDailyUsage_AfterTransferReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_GetDailyUsage_AfterTransfer()";
            const SELECTOR: [u8; 4] = [8u8, 156u8, 169u8, 224u8];
            #[inline]
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
                test_GetDailyUsage_AfterTransferReturn::_tokenize(ret)
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
    /**Function with signature `test_GetDailyUsage_Initial()` and selector `0x8f58a63f`.
```solidity
function test_GetDailyUsage_Initial() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetDailyUsage_InitialCall;
    ///Container type for the return parameters of the [`test_GetDailyUsage_Initial()`](test_GetDailyUsage_InitialCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetDailyUsage_InitialReturn {}
    #[allow(
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
            impl ::core::convert::From<test_GetDailyUsage_InitialCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetDailyUsage_InitialCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetDailyUsage_InitialCall {
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
            impl ::core::convert::From<test_GetDailyUsage_InitialReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetDailyUsage_InitialReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetDailyUsage_InitialReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_GetDailyUsage_InitialReturn {
            fn _tokenize(
                &self,
            ) -> <test_GetDailyUsage_InitialCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_GetDailyUsage_InitialCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_GetDailyUsage_InitialReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_GetDailyUsage_Initial()";
            const SELECTOR: [u8; 4] = [143u8, 88u8, 166u8, 63u8];
            #[inline]
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
                test_GetDailyUsage_InitialReturn::_tokenize(ret)
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
    /**Function with signature `test_RecoverTokens_Success()` and selector `0xf97e8467`.
```solidity
function test_RecoverTokens_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RecoverTokens_SuccessCall;
    ///Container type for the return parameters of the [`test_RecoverTokens_Success()`](test_RecoverTokens_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RecoverTokens_SuccessReturn {}
    #[allow(
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
            impl ::core::convert::From<test_RecoverTokens_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RecoverTokens_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RecoverTokens_SuccessCall {
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
            impl ::core::convert::From<test_RecoverTokens_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RecoverTokens_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RecoverTokens_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RecoverTokens_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_RecoverTokens_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RecoverTokens_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RecoverTokens_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RecoverTokens_Success()";
            const SELECTOR: [u8; 4] = [249u8, 126u8, 132u8, 103u8];
            #[inline]
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
                test_RecoverTokens_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_Constructor_ZeroAdmin()` and selector `0x1074a21f`.
```solidity
function test_RevertWhen_Constructor_ZeroAdmin() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Constructor_ZeroAdminCall;
    ///Container type for the return parameters of the [`test_RevertWhen_Constructor_ZeroAdmin()`](test_RevertWhen_Constructor_ZeroAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Constructor_ZeroAdminReturn {}
    #[allow(
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
            impl ::core::convert::From<test_RevertWhen_Constructor_ZeroAdminCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Constructor_ZeroAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Constructor_ZeroAdminCall {
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
            impl ::core::convert::From<test_RevertWhen_Constructor_ZeroAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Constructor_ZeroAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Constructor_ZeroAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_Constructor_ZeroAdminReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_Constructor_ZeroAdminCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_Constructor_ZeroAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_Constructor_ZeroAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_Constructor_ZeroAdmin()";
            const SELECTOR: [u8; 4] = [16u8, 116u8, 162u8, 31u8];
            #[inline]
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
                test_RevertWhen_Constructor_ZeroAdminReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_Constructor_ZeroCaller()` and selector `0xd3b76bc9`.
```solidity
function test_RevertWhen_Constructor_ZeroCaller() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Constructor_ZeroCallerCall;
    ///Container type for the return parameters of the [`test_RevertWhen_Constructor_ZeroCaller()`](test_RevertWhen_Constructor_ZeroCallerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Constructor_ZeroCallerReturn {}
    #[allow(
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
            impl ::core::convert::From<test_RevertWhen_Constructor_ZeroCallerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Constructor_ZeroCallerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Constructor_ZeroCallerCall {
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
            impl ::core::convert::From<test_RevertWhen_Constructor_ZeroCallerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Constructor_ZeroCallerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Constructor_ZeroCallerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_Constructor_ZeroCallerReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_Constructor_ZeroCallerCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_Constructor_ZeroCallerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_Constructor_ZeroCallerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_Constructor_ZeroCaller()";
            const SELECTOR: [u8; 4] = [211u8, 183u8, 107u8, 201u8];
            #[inline]
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
                test_RevertWhen_Constructor_ZeroCallerReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_ExecuteBridge_BridgeInactive()` and selector `0x4a61cf29`.
```solidity
function test_RevertWhen_ExecuteBridge_BridgeInactive() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_BridgeInactiveCall;
    ///Container type for the return parameters of the [`test_RevertWhen_ExecuteBridge_BridgeInactive()`](test_RevertWhen_ExecuteBridge_BridgeInactiveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_BridgeInactiveReturn {}
    #[allow(
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
            impl ::core::convert::From<test_RevertWhen_ExecuteBridge_BridgeInactiveCall>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_ExecuteBridge_BridgeInactiveCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_BridgeInactiveCall {
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
                test_RevertWhen_ExecuteBridge_BridgeInactiveReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_ExecuteBridge_BridgeInactiveReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_BridgeInactiveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_ExecuteBridge_BridgeInactiveReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_ExecuteBridge_BridgeInactiveCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_RevertWhen_ExecuteBridge_BridgeInactiveCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_ExecuteBridge_BridgeInactiveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_ExecuteBridge_BridgeInactive()";
            const SELECTOR: [u8; 4] = [74u8, 97u8, 207u8, 41u8];
            #[inline]
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
                test_RevertWhen_ExecuteBridge_BridgeInactiveReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_ExecuteBridge_ExceedsDailyLimit()` and selector `0xa30ff4c2`.
```solidity
function test_RevertWhen_ExecuteBridge_ExceedsDailyLimit() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall;
    ///Container type for the return parameters of the [`test_RevertWhen_ExecuteBridge_ExceedsDailyLimit()`](test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_ExceedsDailyLimitReturn {}
    #[allow(
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
            impl ::core::convert::From<
                test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall {
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
                test_RevertWhen_ExecuteBridge_ExceedsDailyLimitReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_ExecuteBridge_ExceedsDailyLimitReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_ExceedsDailyLimitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_ExecuteBridge_ExceedsDailyLimitReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_ExecuteBridge_ExceedsDailyLimitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_ExecuteBridge_ExceedsDailyLimit()";
            const SELECTOR: [u8; 4] = [163u8, 15u8, 244u8, 194u8];
            #[inline]
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
                test_RevertWhen_ExecuteBridge_ExceedsDailyLimitReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_ExecuteBridge_ExceedsMaxSingle()` and selector `0xa3fb1715`.
```solidity
function test_RevertWhen_ExecuteBridge_ExceedsMaxSingle() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall;
    ///Container type for the return parameters of the [`test_RevertWhen_ExecuteBridge_ExceedsMaxSingle()`](test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_ExceedsMaxSingleReturn {}
    #[allow(
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
            impl ::core::convert::From<
                test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall {
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
                test_RevertWhen_ExecuteBridge_ExceedsMaxSingleReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_ExecuteBridge_ExceedsMaxSingleReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_ExceedsMaxSingleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_ExecuteBridge_ExceedsMaxSingleReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_ExecuteBridge_ExceedsMaxSingleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_ExecuteBridge_ExceedsMaxSingle()";
            const SELECTOR: [u8; 4] = [163u8, 251u8, 23u8, 21u8];
            #[inline]
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
                test_RevertWhen_ExecuteBridge_ExceedsMaxSingleReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_ExecuteBridge_UnauthorizedCaller()` and selector `0xbe6da53e`.
```solidity
function test_RevertWhen_ExecuteBridge_UnauthorizedCaller() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall;
    ///Container type for the return parameters of the [`test_RevertWhen_ExecuteBridge_UnauthorizedCaller()`](test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_UnauthorizedCallerReturn {}
    #[allow(
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
            impl ::core::convert::From<
                test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall {
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
                test_RevertWhen_ExecuteBridge_UnauthorizedCallerReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_ExecuteBridge_UnauthorizedCallerReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_UnauthorizedCallerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_ExecuteBridge_UnauthorizedCallerReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_ExecuteBridge_UnauthorizedCallerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_ExecuteBridge_UnauthorizedCaller()";
            const SELECTOR: [u8; 4] = [190u8, 109u8, 165u8, 62u8];
            #[inline]
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
                test_RevertWhen_ExecuteBridge_UnauthorizedCallerReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_ExecuteBridge_ZeroAmount()` and selector `0xe9d3d586`.
```solidity
function test_RevertWhen_ExecuteBridge_ZeroAmount() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_ZeroAmountCall;
    ///Container type for the return parameters of the [`test_RevertWhen_ExecuteBridge_ZeroAmount()`](test_RevertWhen_ExecuteBridge_ZeroAmountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_ZeroAmountReturn {}
    #[allow(
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
            impl ::core::convert::From<test_RevertWhen_ExecuteBridge_ZeroAmountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_ExecuteBridge_ZeroAmountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_ZeroAmountCall {
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
            impl ::core::convert::From<test_RevertWhen_ExecuteBridge_ZeroAmountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_ExecuteBridge_ZeroAmountReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_ZeroAmountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_ExecuteBridge_ZeroAmountReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_ExecuteBridge_ZeroAmountCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_ExecuteBridge_ZeroAmountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_ExecuteBridge_ZeroAmountReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_ExecuteBridge_ZeroAmount()";
            const SELECTOR: [u8; 4] = [233u8, 211u8, 213u8, 134u8];
            #[inline]
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
                test_RevertWhen_ExecuteBridge_ZeroAmountReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_ExecuteBridge_ZeroToken()` and selector `0xdb9b708c`.
```solidity
function test_RevertWhen_ExecuteBridge_ZeroToken() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_ZeroTokenCall;
    ///Container type for the return parameters of the [`test_RevertWhen_ExecuteBridge_ZeroToken()`](test_RevertWhen_ExecuteBridge_ZeroTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_ZeroTokenReturn {}
    #[allow(
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
            impl ::core::convert::From<test_RevertWhen_ExecuteBridge_ZeroTokenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_ExecuteBridge_ZeroTokenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_ZeroTokenCall {
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
            impl ::core::convert::From<test_RevertWhen_ExecuteBridge_ZeroTokenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_ExecuteBridge_ZeroTokenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_ZeroTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_ExecuteBridge_ZeroTokenReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_ExecuteBridge_ZeroTokenCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_ExecuteBridge_ZeroTokenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_ExecuteBridge_ZeroTokenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_ExecuteBridge_ZeroToken()";
            const SELECTOR: [u8; 4] = [219u8, 155u8, 112u8, 140u8];
            #[inline]
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
                test_RevertWhen_ExecuteBridge_ZeroTokenReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_RecoverTokens_NotAdmin()` and selector `0xe86b4fa7`.
```solidity
function test_RevertWhen_RecoverTokens_NotAdmin() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_RecoverTokens_NotAdminCall;
    ///Container type for the return parameters of the [`test_RevertWhen_RecoverTokens_NotAdmin()`](test_RevertWhen_RecoverTokens_NotAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_RecoverTokens_NotAdminReturn {}
    #[allow(
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
            impl ::core::convert::From<test_RevertWhen_RecoverTokens_NotAdminCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_RecoverTokens_NotAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_RecoverTokens_NotAdminCall {
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
            impl ::core::convert::From<test_RevertWhen_RecoverTokens_NotAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_RecoverTokens_NotAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_RecoverTokens_NotAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_RecoverTokens_NotAdminReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_RecoverTokens_NotAdminCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_RecoverTokens_NotAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_RecoverTokens_NotAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_RecoverTokens_NotAdmin()";
            const SELECTOR: [u8; 4] = [232u8, 107u8, 79u8, 167u8];
            #[inline]
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
                test_RevertWhen_RecoverTokens_NotAdminReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_RecoverTokens_ZeroAddress()` and selector `0x2ed21183`.
```solidity
function test_RevertWhen_RecoverTokens_ZeroAddress() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_RecoverTokens_ZeroAddressCall;
    ///Container type for the return parameters of the [`test_RevertWhen_RecoverTokens_ZeroAddress()`](test_RevertWhen_RecoverTokens_ZeroAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_RecoverTokens_ZeroAddressReturn {}
    #[allow(
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
            impl ::core::convert::From<test_RevertWhen_RecoverTokens_ZeroAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_RecoverTokens_ZeroAddressCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_RecoverTokens_ZeroAddressCall {
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
            impl ::core::convert::From<test_RevertWhen_RecoverTokens_ZeroAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_RecoverTokens_ZeroAddressReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_RecoverTokens_ZeroAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_RecoverTokens_ZeroAddressReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_RecoverTokens_ZeroAddressCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_RecoverTokens_ZeroAddressCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_RecoverTokens_ZeroAddressReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_RecoverTokens_ZeroAddress()";
            const SELECTOR: [u8; 4] = [46u8, 210u8, 17u8, 131u8];
            #[inline]
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
                test_RevertWhen_RecoverTokens_ZeroAddressReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_SetBridgeTarget_NotAdmin()` and selector `0x13a86f1a`.
```solidity
function test_RevertWhen_SetBridgeTarget_NotAdmin() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_SetBridgeTarget_NotAdminCall;
    ///Container type for the return parameters of the [`test_RevertWhen_SetBridgeTarget_NotAdmin()`](test_RevertWhen_SetBridgeTarget_NotAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_SetBridgeTarget_NotAdminReturn {}
    #[allow(
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
            impl ::core::convert::From<test_RevertWhen_SetBridgeTarget_NotAdminCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_SetBridgeTarget_NotAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_SetBridgeTarget_NotAdminCall {
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
            impl ::core::convert::From<test_RevertWhen_SetBridgeTarget_NotAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_SetBridgeTarget_NotAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_SetBridgeTarget_NotAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_SetBridgeTarget_NotAdminReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_SetBridgeTarget_NotAdminCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_SetBridgeTarget_NotAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_SetBridgeTarget_NotAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_SetBridgeTarget_NotAdmin()";
            const SELECTOR: [u8; 4] = [19u8, 168u8, 111u8, 26u8];
            #[inline]
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
                test_RevertWhen_SetBridgeTarget_NotAdminReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_SetBridgeTarget_ZeroAddress()` and selector `0x6f8cece4`.
```solidity
function test_RevertWhen_SetBridgeTarget_ZeroAddress() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_SetBridgeTarget_ZeroAddressCall;
    ///Container type for the return parameters of the [`test_RevertWhen_SetBridgeTarget_ZeroAddress()`](test_RevertWhen_SetBridgeTarget_ZeroAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_SetBridgeTarget_ZeroAddressReturn {}
    #[allow(
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
            impl ::core::convert::From<test_RevertWhen_SetBridgeTarget_ZeroAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_SetBridgeTarget_ZeroAddressCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_SetBridgeTarget_ZeroAddressCall {
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
            impl ::core::convert::From<test_RevertWhen_SetBridgeTarget_ZeroAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_SetBridgeTarget_ZeroAddressReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_SetBridgeTarget_ZeroAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_SetBridgeTarget_ZeroAddressReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_SetBridgeTarget_ZeroAddressCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_RevertWhen_SetBridgeTarget_ZeroAddressCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_SetBridgeTarget_ZeroAddressReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_SetBridgeTarget_ZeroAddress()";
            const SELECTOR: [u8; 4] = [111u8, 140u8, 236u8, 228u8];
            #[inline]
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
                test_RevertWhen_SetBridgeTarget_ZeroAddressReturn::_tokenize(ret)
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
    /**Function with signature `test_SetBridgeActive_Success()` and selector `0x23e1ebe7`.
```solidity
function test_SetBridgeActive_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetBridgeActive_SuccessCall;
    ///Container type for the return parameters of the [`test_SetBridgeActive_Success()`](test_SetBridgeActive_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetBridgeActive_SuccessReturn {}
    #[allow(
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
            impl ::core::convert::From<test_SetBridgeActive_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetBridgeActive_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetBridgeActive_SuccessCall {
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
            impl ::core::convert::From<test_SetBridgeActive_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetBridgeActive_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetBridgeActive_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_SetBridgeActive_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_SetBridgeActive_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_SetBridgeActive_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_SetBridgeActive_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_SetBridgeActive_Success()";
            const SELECTOR: [u8; 4] = [35u8, 225u8, 235u8, 231u8];
            #[inline]
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
                test_SetBridgeActive_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_SetBridgeTarget_Success()` and selector `0xf3ed2b05`.
```solidity
function test_SetBridgeTarget_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetBridgeTarget_SuccessCall;
    ///Container type for the return parameters of the [`test_SetBridgeTarget_Success()`](test_SetBridgeTarget_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetBridgeTarget_SuccessReturn {}
    #[allow(
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
            impl ::core::convert::From<test_SetBridgeTarget_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetBridgeTarget_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetBridgeTarget_SuccessCall {
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
            impl ::core::convert::From<test_SetBridgeTarget_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetBridgeTarget_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetBridgeTarget_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_SetBridgeTarget_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_SetBridgeTarget_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_SetBridgeTarget_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_SetBridgeTarget_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_SetBridgeTarget_Success()";
            const SELECTOR: [u8; 4] = [243u8, 237u8, 43u8, 5u8];
            #[inline]
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
                test_SetBridgeTarget_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_SetDailyLimit_Success()` and selector `0x497b3918`.
```solidity
function test_SetDailyLimit_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetDailyLimit_SuccessCall;
    ///Container type for the return parameters of the [`test_SetDailyLimit_Success()`](test_SetDailyLimit_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetDailyLimit_SuccessReturn {}
    #[allow(
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
            impl ::core::convert::From<test_SetDailyLimit_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetDailyLimit_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetDailyLimit_SuccessCall {
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
            impl ::core::convert::From<test_SetDailyLimit_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetDailyLimit_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetDailyLimit_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_SetDailyLimit_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_SetDailyLimit_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_SetDailyLimit_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_SetDailyLimit_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_SetDailyLimit_Success()";
            const SELECTOR: [u8; 4] = [73u8, 123u8, 57u8, 24u8];
            #[inline]
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
                test_SetDailyLimit_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_SetMaxSingleTransfer_Success()` and selector `0x094f27a0`.
```solidity
function test_SetMaxSingleTransfer_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetMaxSingleTransfer_SuccessCall;
    ///Container type for the return parameters of the [`test_SetMaxSingleTransfer_Success()`](test_SetMaxSingleTransfer_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetMaxSingleTransfer_SuccessReturn {}
    #[allow(
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
            impl ::core::convert::From<test_SetMaxSingleTransfer_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetMaxSingleTransfer_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetMaxSingleTransfer_SuccessCall {
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
            impl ::core::convert::From<test_SetMaxSingleTransfer_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetMaxSingleTransfer_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetMaxSingleTransfer_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_SetMaxSingleTransfer_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_SetMaxSingleTransfer_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_SetMaxSingleTransfer_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_SetMaxSingleTransfer_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_SetMaxSingleTransfer_Success()";
            const SELECTOR: [u8; 4] = [9u8, 79u8, 39u8, 160u8];
            #[inline]
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
                test_SetMaxSingleTransfer_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `token()` and selector `0xfc0c546a`.
```solidity
function token() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`token()`](tokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenReturn {
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
            impl ::core::convert::From<tokenCall> for UnderlyingRustTuple<'_> {
                fn from(value: tokenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenCall {
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
            impl ::core::convert::From<tokenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tokenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "token()";
            const SELECTOR: [u8; 4] = [252u8, 12u8, 84u8, 106u8];
            #[inline]
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
                        let r: tokenReturn = r.into();
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
                        let r: tokenReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `user()` and selector `0x4f8632ba`.
```solidity
function user() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`user()`](userCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userReturn {
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
            impl ::core::convert::From<userCall> for UnderlyingRustTuple<'_> {
                fn from(value: userCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for userCall {
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
            impl ::core::convert::From<userReturn> for UnderlyingRustTuple<'_> {
                fn from(value: userReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for userReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for userCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "user()";
            const SELECTOR: [u8; 4] = [79u8, 134u8, 50u8, 186u8];
            #[inline]
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
                        let r: userReturn = r.into();
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
                        let r: userReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`BaseBridgeProxyTest`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum BaseBridgeProxyTestCalls {
        #[allow(missing_docs)]
        DAILY_LIMIT(DAILY_LIMITCall),
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        MAX_SINGLE_TRANSFER(MAX_SINGLE_TRANSFERCall),
        #[allow(missing_docs)]
        admin(adminCall),
        #[allow(missing_docs)]
        bridgeProxy(bridgeProxyCall),
        #[allow(missing_docs)]
        caller(callerCall),
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
        newTarget(newTargetCall),
        #[allow(missing_docs)]
        setUp(setUpCall),
        #[allow(missing_docs)]
        targetArtifactSelectors(targetArtifactSelectorsCall),
        #[allow(missing_docs)]
        targetArtifacts(targetArtifactsCall),
        #[allow(missing_docs)]
        targetBridge(targetBridgeCall),
        #[allow(missing_docs)]
        targetContracts(targetContractsCall),
        #[allow(missing_docs)]
        targetInterfaces(targetInterfacesCall),
        #[allow(missing_docs)]
        targetSelectors(targetSelectorsCall),
        #[allow(missing_docs)]
        targetSenders(targetSendersCall),
        #[allow(missing_docs)]
        testFuzz_ExecuteBridge_ValidAmounts(testFuzz_ExecuteBridge_ValidAmountsCall),
        #[allow(missing_docs)]
        testFuzz_SetDailyLimit_ValidValues(testFuzz_SetDailyLimit_ValidValuesCall),
        #[allow(missing_docs)]
        test_Constructor_RoleAssignment(test_Constructor_RoleAssignmentCall),
        #[allow(missing_docs)]
        test_Constructor_Success(test_Constructor_SuccessCall),
        #[allow(missing_docs)]
        test_DailyLimit_Cumulative(test_DailyLimit_CumulativeCall),
        #[allow(missing_docs)]
        test_DailyLimit_Reset(test_DailyLimit_ResetCall),
        #[allow(missing_docs)]
        test_ExecuteBridge_MultipleTransfers(test_ExecuteBridge_MultipleTransfersCall),
        #[allow(missing_docs)]
        test_ExecuteBridge_ReentrancyProtection(
            test_ExecuteBridge_ReentrancyProtectionCall,
        ),
        #[allow(missing_docs)]
        test_ExecuteBridge_Success(test_ExecuteBridge_SuccessCall),
        #[allow(missing_docs)]
        test_GetBridgeInfo(test_GetBridgeInfoCall),
        #[allow(missing_docs)]
        test_GetDailyUsage_AfterTransfer(test_GetDailyUsage_AfterTransferCall),
        #[allow(missing_docs)]
        test_GetDailyUsage_Initial(test_GetDailyUsage_InitialCall),
        #[allow(missing_docs)]
        test_RecoverTokens_Success(test_RecoverTokens_SuccessCall),
        #[allow(missing_docs)]
        test_RevertWhen_Constructor_ZeroAdmin(test_RevertWhen_Constructor_ZeroAdminCall),
        #[allow(missing_docs)]
        test_RevertWhen_Constructor_ZeroCaller(
            test_RevertWhen_Constructor_ZeroCallerCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_ExecuteBridge_BridgeInactive(
            test_RevertWhen_ExecuteBridge_BridgeInactiveCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_ExecuteBridge_ExceedsDailyLimit(
            test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_ExecuteBridge_ExceedsMaxSingle(
            test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_ExecuteBridge_UnauthorizedCaller(
            test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_ExecuteBridge_ZeroAmount(
            test_RevertWhen_ExecuteBridge_ZeroAmountCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_ExecuteBridge_ZeroToken(
            test_RevertWhen_ExecuteBridge_ZeroTokenCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_RecoverTokens_NotAdmin(
            test_RevertWhen_RecoverTokens_NotAdminCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_RecoverTokens_ZeroAddress(
            test_RevertWhen_RecoverTokens_ZeroAddressCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_SetBridgeTarget_NotAdmin(
            test_RevertWhen_SetBridgeTarget_NotAdminCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_SetBridgeTarget_ZeroAddress(
            test_RevertWhen_SetBridgeTarget_ZeroAddressCall,
        ),
        #[allow(missing_docs)]
        test_SetBridgeActive_Success(test_SetBridgeActive_SuccessCall),
        #[allow(missing_docs)]
        test_SetBridgeTarget_Success(test_SetBridgeTarget_SuccessCall),
        #[allow(missing_docs)]
        test_SetDailyLimit_Success(test_SetDailyLimit_SuccessCall),
        #[allow(missing_docs)]
        test_SetMaxSingleTransfer_Success(test_SetMaxSingleTransfer_SuccessCall),
        #[allow(missing_docs)]
        token(tokenCall),
        #[allow(missing_docs)]
        user(userCall),
    }
    impl BaseBridgeProxyTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 156u8, 169u8, 224u8],
            [9u8, 79u8, 39u8, 160u8],
            [10u8, 146u8, 84u8, 228u8],
            [15u8, 87u8, 40u8, 15u8],
            [16u8, 116u8, 162u8, 31u8],
            [17u8, 126u8, 59u8, 66u8],
            [19u8, 33u8, 127u8, 144u8],
            [19u8, 168u8, 111u8, 26u8],
            [30u8, 215u8, 131u8, 28u8],
            [35u8, 225u8, 235u8, 231u8],
            [36u8, 142u8, 195u8, 38u8],
            [41u8, 54u8, 89u8, 104u8],
            [42u8, 222u8, 56u8, 128u8],
            [46u8, 210u8, 17u8, 131u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [73u8, 123u8, 57u8, 24u8],
            [74u8, 97u8, 207u8, 41u8],
            [79u8, 134u8, 50u8, 186u8],
            [102u8, 217u8, 169u8, 160u8],
            [111u8, 140u8, 236u8, 228u8],
            [133u8, 34u8, 108u8, 129u8],
            [139u8, 88u8, 203u8, 174u8],
            [143u8, 88u8, 166u8, 63u8],
            [145u8, 106u8, 23u8, 198u8],
            [149u8, 109u8, 152u8, 8u8],
            [149u8, 155u8, 51u8, 125u8],
            [163u8, 15u8, 244u8, 194u8],
            [163u8, 212u8, 72u8, 91u8],
            [163u8, 251u8, 23u8, 21u8],
            [176u8, 70u8, 79u8, 220u8],
            [180u8, 77u8, 201u8, 214u8],
            [181u8, 80u8, 138u8, 169u8],
            [181u8, 93u8, 66u8, 188u8],
            [185u8, 181u8, 189u8, 104u8],
            [186u8, 65u8, 79u8, 166u8],
            [190u8, 109u8, 165u8, 62u8],
            [207u8, 251u8, 4u8, 139u8],
            [211u8, 7u8, 92u8, 73u8],
            [211u8, 183u8, 107u8, 201u8],
            [219u8, 155u8, 112u8, 140u8],
            [220u8, 204u8, 87u8, 241u8],
            [226u8, 12u8, 159u8, 113u8],
            [232u8, 107u8, 79u8, 167u8],
            [233u8, 211u8, 213u8, 134u8],
            [243u8, 237u8, 43u8, 5u8],
            [248u8, 81u8, 164u8, 64u8],
            [249u8, 126u8, 132u8, 103u8],
            [250u8, 118u8, 38u8, 212u8],
            [252u8, 12u8, 84u8, 106u8],
            [252u8, 156u8, 141u8, 57u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(test_GetDailyUsage_AfterTransfer),
            ::core::stringify!(test_SetMaxSingleTransfer_Success),
            ::core::stringify!(setUp),
            ::core::stringify!(test_DailyLimit_Cumulative),
            ::core::stringify!(test_RevertWhen_Constructor_ZeroAdmin),
            ::core::stringify!(MAX_SINGLE_TRANSFER),
            ::core::stringify!(newTarget),
            ::core::stringify!(test_RevertWhen_SetBridgeTarget_NotAdmin),
            ::core::stringify!(excludeSenders),
            ::core::stringify!(test_SetBridgeActive_Success),
            ::core::stringify!(DAILY_LIMIT),
            ::core::stringify!(testFuzz_SetDailyLimit_ValidValues),
            ::core::stringify!(targetInterfaces),
            ::core::stringify!(test_RevertWhen_RecoverTokens_ZeroAddress),
            ::core::stringify!(targetSenders),
            ::core::stringify!(targetContracts),
            ::core::stringify!(test_SetDailyLimit_Success),
            ::core::stringify!(test_RevertWhen_ExecuteBridge_BridgeInactive),
            ::core::stringify!(user),
            ::core::stringify!(targetArtifactSelectors),
            ::core::stringify!(test_RevertWhen_SetBridgeTarget_ZeroAddress),
            ::core::stringify!(targetArtifacts),
            ::core::stringify!(test_ExecuteBridge_MultipleTransfers),
            ::core::stringify!(test_GetDailyUsage_Initial),
            ::core::stringify!(targetSelectors),
            ::core::stringify!(test_GetBridgeInfo),
            ::core::stringify!(targetBridge),
            ::core::stringify!(test_RevertWhen_ExecuteBridge_ExceedsDailyLimit),
            ::core::stringify!(bridgeProxy),
            ::core::stringify!(test_RevertWhen_ExecuteBridge_ExceedsMaxSingle),
            ::core::stringify!(excludeSelectors),
            ::core::stringify!(test_Constructor_Success),
            ::core::stringify!(excludeArtifacts),
            ::core::stringify!(test_ExecuteBridge_ReentrancyProtection),
            ::core::stringify!(test_ExecuteBridge_Success),
            ::core::stringify!(failed),
            ::core::stringify!(test_RevertWhen_ExecuteBridge_UnauthorizedCaller),
            ::core::stringify!(testFuzz_ExecuteBridge_ValidAmounts),
            ::core::stringify!(test_DailyLimit_Reset),
            ::core::stringify!(test_RevertWhen_Constructor_ZeroCaller),
            ::core::stringify!(test_RevertWhen_ExecuteBridge_ZeroToken),
            ::core::stringify!(test_Constructor_RoleAssignment),
            ::core::stringify!(excludeContracts),
            ::core::stringify!(test_RevertWhen_RecoverTokens_NotAdmin),
            ::core::stringify!(test_RevertWhen_ExecuteBridge_ZeroAmount),
            ::core::stringify!(test_SetBridgeTarget_Success),
            ::core::stringify!(admin),
            ::core::stringify!(test_RecoverTokens_Success),
            ::core::stringify!(IS_TEST),
            ::core::stringify!(token),
            ::core::stringify!(caller),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <test_GetDailyUsage_AfterTransferCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_SetMaxSingleTransfer_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setUpCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_DailyLimit_CumulativeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_Constructor_ZeroAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <MAX_SINGLE_TRANSFERCall as alloy_sol_types::SolCall>::SIGNATURE,
            <newTargetCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_SetBridgeTarget_NotAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_SetBridgeActive_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DAILY_LIMITCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testFuzz_SetDailyLimit_ValidValuesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetInterfacesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_RecoverTokens_ZeroAddressCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_SetDailyLimit_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_ExecuteBridge_BridgeInactiveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <userCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_SetBridgeTarget_ZeroAddressCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_ExecuteBridge_MultipleTransfersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_GetDailyUsage_InitialCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_GetBridgeInfoCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetBridgeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall as alloy_sol_types::SolCall>::SIGNATURE,
            <bridgeProxyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Constructor_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_ExecuteBridge_ReentrancyProtectionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_ExecuteBridge_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <failedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testFuzz_ExecuteBridge_ValidAmountsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_DailyLimit_ResetCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_Constructor_ZeroCallerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_ExecuteBridge_ZeroTokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_RecoverTokens_NotAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_ExecuteBridge_ZeroAmountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_SetBridgeTarget_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <adminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RecoverTokens_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <IS_TESTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <tokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <callerCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for BaseBridgeProxyTestCalls {
        const NAME: &'static str = "BaseBridgeProxyTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 51usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DAILY_LIMIT(_) => {
                    <DAILY_LIMITCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::MAX_SINGLE_TRANSFER(_) => {
                    <MAX_SINGLE_TRANSFERCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::admin(_) => <adminCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::bridgeProxy(_) => {
                    <bridgeProxyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::caller(_) => <callerCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::newTarget(_) => {
                    <newTargetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setUp(_) => <setUpCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::targetArtifactSelectors(_) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifacts(_) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetBridge(_) => {
                    <targetBridgeCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::testFuzz_ExecuteBridge_ValidAmounts(_) => {
                    <testFuzz_ExecuteBridge_ValidAmountsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testFuzz_SetDailyLimit_ValidValues(_) => {
                    <testFuzz_SetDailyLimit_ValidValuesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Constructor_RoleAssignment(_) => {
                    <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Constructor_Success(_) => {
                    <test_Constructor_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_DailyLimit_Cumulative(_) => {
                    <test_DailyLimit_CumulativeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_DailyLimit_Reset(_) => {
                    <test_DailyLimit_ResetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_ExecuteBridge_MultipleTransfers(_) => {
                    <test_ExecuteBridge_MultipleTransfersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_ExecuteBridge_ReentrancyProtection(_) => {
                    <test_ExecuteBridge_ReentrancyProtectionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_ExecuteBridge_Success(_) => {
                    <test_ExecuteBridge_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_GetBridgeInfo(_) => {
                    <test_GetBridgeInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_GetDailyUsage_AfterTransfer(_) => {
                    <test_GetDailyUsage_AfterTransferCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_GetDailyUsage_Initial(_) => {
                    <test_GetDailyUsage_InitialCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RecoverTokens_Success(_) => {
                    <test_RecoverTokens_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_Constructor_ZeroAdmin(_) => {
                    <test_RevertWhen_Constructor_ZeroAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_Constructor_ZeroCaller(_) => {
                    <test_RevertWhen_Constructor_ZeroCallerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_ExecuteBridge_BridgeInactive(_) => {
                    <test_RevertWhen_ExecuteBridge_BridgeInactiveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_ExecuteBridge_ExceedsDailyLimit(_) => {
                    <test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_ExecuteBridge_ExceedsMaxSingle(_) => {
                    <test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_ExecuteBridge_UnauthorizedCaller(_) => {
                    <test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_ExecuteBridge_ZeroAmount(_) => {
                    <test_RevertWhen_ExecuteBridge_ZeroAmountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_ExecuteBridge_ZeroToken(_) => {
                    <test_RevertWhen_ExecuteBridge_ZeroTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_RecoverTokens_NotAdmin(_) => {
                    <test_RevertWhen_RecoverTokens_NotAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_RecoverTokens_ZeroAddress(_) => {
                    <test_RevertWhen_RecoverTokens_ZeroAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_SetBridgeTarget_NotAdmin(_) => {
                    <test_RevertWhen_SetBridgeTarget_NotAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_SetBridgeTarget_ZeroAddress(_) => {
                    <test_RevertWhen_SetBridgeTarget_ZeroAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_SetBridgeActive_Success(_) => {
                    <test_SetBridgeActive_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_SetBridgeTarget_Success(_) => {
                    <test_SetBridgeTarget_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_SetDailyLimit_Success(_) => {
                    <test_SetDailyLimit_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_SetMaxSingleTransfer_Success(_) => {
                    <test_SetMaxSingleTransfer_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::token(_) => <tokenCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::user(_) => <userCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls>] = &[
                {
                    fn test_GetDailyUsage_AfterTransfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_GetDailyUsage_AfterTransferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_GetDailyUsage_AfterTransfer,
                            )
                    }
                    test_GetDailyUsage_AfterTransfer
                },
                {
                    fn test_SetMaxSingleTransfer_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_SetMaxSingleTransfer_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_SetMaxSingleTransfer_Success,
                            )
                    }
                    test_SetMaxSingleTransfer_Success
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BaseBridgeProxyTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn test_DailyLimit_Cumulative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_DailyLimit_CumulativeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_DailyLimit_Cumulative)
                    }
                    test_DailyLimit_Cumulative
                },
                {
                    fn test_RevertWhen_Constructor_ZeroAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_Constructor_ZeroAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_Constructor_ZeroAdmin,
                            )
                    }
                    test_RevertWhen_Constructor_ZeroAdmin
                },
                {
                    fn MAX_SINGLE_TRANSFER(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <MAX_SINGLE_TRANSFERCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::MAX_SINGLE_TRANSFER)
                    }
                    MAX_SINGLE_TRANSFER
                },
                {
                    fn newTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <newTargetCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BaseBridgeProxyTestCalls::newTarget)
                    }
                    newTarget
                },
                {
                    fn test_RevertWhen_SetBridgeTarget_NotAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_SetBridgeTarget_NotAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_SetBridgeTarget_NotAdmin,
                            )
                    }
                    test_RevertWhen_SetBridgeTarget_NotAdmin
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn test_SetBridgeActive_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_SetBridgeActive_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_SetBridgeActive_Success)
                    }
                    test_SetBridgeActive_Success
                },
                {
                    fn DAILY_LIMIT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <DAILY_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::DAILY_LIMIT)
                    }
                    DAILY_LIMIT
                },
                {
                    fn testFuzz_SetDailyLimit_ValidValues(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <testFuzz_SetDailyLimit_ValidValuesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::testFuzz_SetDailyLimit_ValidValues,
                            )
                    }
                    testFuzz_SetDailyLimit_ValidValues
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn test_RevertWhen_RecoverTokens_ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_RecoverTokens_ZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_RecoverTokens_ZeroAddress,
                            )
                    }
                    test_RevertWhen_RecoverTokens_ZeroAddress
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn test_SetDailyLimit_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_SetDailyLimit_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_SetDailyLimit_Success)
                    }
                    test_SetDailyLimit_Success
                },
                {
                    fn test_RevertWhen_ExecuteBridge_BridgeInactive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_BridgeInactiveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_BridgeInactive,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_BridgeInactive
                },
                {
                    fn user(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <userCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BaseBridgeProxyTestCalls::user)
                    }
                    user
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn test_RevertWhen_SetBridgeTarget_ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_SetBridgeTarget_ZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_SetBridgeTarget_ZeroAddress,
                            )
                    }
                    test_RevertWhen_SetBridgeTarget_ZeroAddress
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn test_ExecuteBridge_MultipleTransfers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_ExecuteBridge_MultipleTransfersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_ExecuteBridge_MultipleTransfers,
                            )
                    }
                    test_ExecuteBridge_MultipleTransfers
                },
                {
                    fn test_GetDailyUsage_Initial(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_GetDailyUsage_InitialCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_GetDailyUsage_Initial)
                    }
                    test_GetDailyUsage_Initial
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn test_GetBridgeInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_GetBridgeInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_GetBridgeInfo)
                    }
                    test_GetBridgeInfo
                },
                {
                    fn targetBridge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetBridgeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetBridge)
                    }
                    targetBridge
                },
                {
                    fn test_RevertWhen_ExecuteBridge_ExceedsDailyLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_ExceedsDailyLimit,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_ExceedsDailyLimit
                },
                {
                    fn bridgeProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <bridgeProxyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::bridgeProxy)
                    }
                    bridgeProxy
                },
                {
                    fn test_RevertWhen_ExecuteBridge_ExceedsMaxSingle(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_ExceedsMaxSingle,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_ExceedsMaxSingle
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn test_Constructor_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_Constructor_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_Constructor_Success)
                    }
                    test_Constructor_Success
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn test_ExecuteBridge_ReentrancyProtection(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_ExecuteBridge_ReentrancyProtectionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_ExecuteBridge_ReentrancyProtection,
                            )
                    }
                    test_ExecuteBridge_ReentrancyProtection
                },
                {
                    fn test_ExecuteBridge_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_ExecuteBridge_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_ExecuteBridge_Success)
                    }
                    test_ExecuteBridge_Success
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BaseBridgeProxyTestCalls::failed)
                    }
                    failed
                },
                {
                    fn test_RevertWhen_ExecuteBridge_UnauthorizedCaller(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_UnauthorizedCaller,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_UnauthorizedCaller
                },
                {
                    fn testFuzz_ExecuteBridge_ValidAmounts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <testFuzz_ExecuteBridge_ValidAmountsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::testFuzz_ExecuteBridge_ValidAmounts,
                            )
                    }
                    testFuzz_ExecuteBridge_ValidAmounts
                },
                {
                    fn test_DailyLimit_Reset(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_DailyLimit_ResetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_DailyLimit_Reset)
                    }
                    test_DailyLimit_Reset
                },
                {
                    fn test_RevertWhen_Constructor_ZeroCaller(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_Constructor_ZeroCallerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_Constructor_ZeroCaller,
                            )
                    }
                    test_RevertWhen_Constructor_ZeroCaller
                },
                {
                    fn test_RevertWhen_ExecuteBridge_ZeroToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_ZeroTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_ZeroToken,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_ZeroToken
                },
                {
                    fn test_Constructor_RoleAssignment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_Constructor_RoleAssignment,
                            )
                    }
                    test_Constructor_RoleAssignment
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn test_RevertWhen_RecoverTokens_NotAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_RecoverTokens_NotAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_RecoverTokens_NotAdmin,
                            )
                    }
                    test_RevertWhen_RecoverTokens_NotAdmin
                },
                {
                    fn test_RevertWhen_ExecuteBridge_ZeroAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_ZeroAmountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_ZeroAmount,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_ZeroAmount
                },
                {
                    fn test_SetBridgeTarget_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_SetBridgeTarget_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_SetBridgeTarget_Success)
                    }
                    test_SetBridgeTarget_Success
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BaseBridgeProxyTestCalls::admin)
                    }
                    admin
                },
                {
                    fn test_RecoverTokens_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RecoverTokens_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_RecoverTokens_Success)
                    }
                    test_RecoverTokens_Success
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BaseBridgeProxyTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn token(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <tokenCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BaseBridgeProxyTestCalls::token)
                    }
                    token
                },
                {
                    fn caller(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <callerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BaseBridgeProxyTestCalls::caller)
                    }
                    caller
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
            ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls>] = &[
                {
                    fn test_GetDailyUsage_AfterTransfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_GetDailyUsage_AfterTransferCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_GetDailyUsage_AfterTransfer,
                            )
                    }
                    test_GetDailyUsage_AfterTransfer
                },
                {
                    fn test_SetMaxSingleTransfer_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_SetMaxSingleTransfer_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_SetMaxSingleTransfer_Success,
                            )
                    }
                    test_SetMaxSingleTransfer_Success
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn test_DailyLimit_Cumulative(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_DailyLimit_CumulativeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_DailyLimit_Cumulative)
                    }
                    test_DailyLimit_Cumulative
                },
                {
                    fn test_RevertWhen_Constructor_ZeroAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_Constructor_ZeroAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_Constructor_ZeroAdmin,
                            )
                    }
                    test_RevertWhen_Constructor_ZeroAdmin
                },
                {
                    fn MAX_SINGLE_TRANSFER(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <MAX_SINGLE_TRANSFERCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::MAX_SINGLE_TRANSFER)
                    }
                    MAX_SINGLE_TRANSFER
                },
                {
                    fn newTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <newTargetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::newTarget)
                    }
                    newTarget
                },
                {
                    fn test_RevertWhen_SetBridgeTarget_NotAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_SetBridgeTarget_NotAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_SetBridgeTarget_NotAdmin,
                            )
                    }
                    test_RevertWhen_SetBridgeTarget_NotAdmin
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn test_SetBridgeActive_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_SetBridgeActive_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_SetBridgeActive_Success)
                    }
                    test_SetBridgeActive_Success
                },
                {
                    fn DAILY_LIMIT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <DAILY_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::DAILY_LIMIT)
                    }
                    DAILY_LIMIT
                },
                {
                    fn testFuzz_SetDailyLimit_ValidValues(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <testFuzz_SetDailyLimit_ValidValuesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::testFuzz_SetDailyLimit_ValidValues,
                            )
                    }
                    testFuzz_SetDailyLimit_ValidValues
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn test_RevertWhen_RecoverTokens_ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_RecoverTokens_ZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_RecoverTokens_ZeroAddress,
                            )
                    }
                    test_RevertWhen_RecoverTokens_ZeroAddress
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn test_SetDailyLimit_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_SetDailyLimit_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_SetDailyLimit_Success)
                    }
                    test_SetDailyLimit_Success
                },
                {
                    fn test_RevertWhen_ExecuteBridge_BridgeInactive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_BridgeInactiveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_BridgeInactive,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_BridgeInactive
                },
                {
                    fn user(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <userCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::user)
                    }
                    user
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn test_RevertWhen_SetBridgeTarget_ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_SetBridgeTarget_ZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_SetBridgeTarget_ZeroAddress,
                            )
                    }
                    test_RevertWhen_SetBridgeTarget_ZeroAddress
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn test_ExecuteBridge_MultipleTransfers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_ExecuteBridge_MultipleTransfersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_ExecuteBridge_MultipleTransfers,
                            )
                    }
                    test_ExecuteBridge_MultipleTransfers
                },
                {
                    fn test_GetDailyUsage_Initial(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_GetDailyUsage_InitialCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_GetDailyUsage_Initial)
                    }
                    test_GetDailyUsage_Initial
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn test_GetBridgeInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_GetBridgeInfoCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_GetBridgeInfo)
                    }
                    test_GetBridgeInfo
                },
                {
                    fn targetBridge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <targetBridgeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::targetBridge)
                    }
                    targetBridge
                },
                {
                    fn test_RevertWhen_ExecuteBridge_ExceedsDailyLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_ExceedsDailyLimit,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_ExceedsDailyLimit
                },
                {
                    fn bridgeProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <bridgeProxyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::bridgeProxy)
                    }
                    bridgeProxy
                },
                {
                    fn test_RevertWhen_ExecuteBridge_ExceedsMaxSingle(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_ExceedsMaxSingle,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_ExceedsMaxSingle
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn test_Constructor_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_Constructor_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_Constructor_Success)
                    }
                    test_Constructor_Success
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn test_ExecuteBridge_ReentrancyProtection(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_ExecuteBridge_ReentrancyProtectionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_ExecuteBridge_ReentrancyProtection,
                            )
                    }
                    test_ExecuteBridge_ReentrancyProtection
                },
                {
                    fn test_ExecuteBridge_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_ExecuteBridge_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_ExecuteBridge_Success)
                    }
                    test_ExecuteBridge_Success
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::failed)
                    }
                    failed
                },
                {
                    fn test_RevertWhen_ExecuteBridge_UnauthorizedCaller(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_UnauthorizedCaller,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_UnauthorizedCaller
                },
                {
                    fn testFuzz_ExecuteBridge_ValidAmounts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <testFuzz_ExecuteBridge_ValidAmountsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::testFuzz_ExecuteBridge_ValidAmounts,
                            )
                    }
                    testFuzz_ExecuteBridge_ValidAmounts
                },
                {
                    fn test_DailyLimit_Reset(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_DailyLimit_ResetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_DailyLimit_Reset)
                    }
                    test_DailyLimit_Reset
                },
                {
                    fn test_RevertWhen_Constructor_ZeroCaller(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_Constructor_ZeroCallerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_Constructor_ZeroCaller,
                            )
                    }
                    test_RevertWhen_Constructor_ZeroCaller
                },
                {
                    fn test_RevertWhen_ExecuteBridge_ZeroToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_ZeroTokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_ZeroToken,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_ZeroToken
                },
                {
                    fn test_Constructor_RoleAssignment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_Constructor_RoleAssignment,
                            )
                    }
                    test_Constructor_RoleAssignment
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn test_RevertWhen_RecoverTokens_NotAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_RecoverTokens_NotAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_RecoverTokens_NotAdmin,
                            )
                    }
                    test_RevertWhen_RecoverTokens_NotAdmin
                },
                {
                    fn test_RevertWhen_ExecuteBridge_ZeroAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_ZeroAmountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BaseBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_ZeroAmount,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_ZeroAmount
                },
                {
                    fn test_SetBridgeTarget_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_SetBridgeTarget_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_SetBridgeTarget_Success)
                    }
                    test_SetBridgeTarget_Success
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::admin)
                    }
                    admin
                },
                {
                    fn test_RecoverTokens_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <test_RecoverTokens_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::test_RecoverTokens_Success)
                    }
                    test_RecoverTokens_Success
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn token(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <tokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::token)
                    }
                    token
                },
                {
                    fn caller(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BaseBridgeProxyTestCalls> {
                        <callerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BaseBridgeProxyTestCalls::caller)
                    }
                    caller
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
                Self::DAILY_LIMIT(inner) => {
                    <DAILY_LIMITCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::MAX_SINGLE_TRANSFER(inner) => {
                    <MAX_SINGLE_TRANSFERCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::admin(inner) => {
                    <adminCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bridgeProxy(inner) => {
                    <bridgeProxyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::caller(inner) => {
                    <callerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::newTarget(inner) => {
                    <newTargetCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::targetBridge(inner) => {
                    <targetBridgeCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::testFuzz_ExecuteBridge_ValidAmounts(inner) => {
                    <testFuzz_ExecuteBridge_ValidAmountsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testFuzz_SetDailyLimit_ValidValues(inner) => {
                    <testFuzz_SetDailyLimit_ValidValuesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Constructor_RoleAssignment(inner) => {
                    <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Constructor_Success(inner) => {
                    <test_Constructor_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_DailyLimit_Cumulative(inner) => {
                    <test_DailyLimit_CumulativeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_DailyLimit_Reset(inner) => {
                    <test_DailyLimit_ResetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_ExecuteBridge_MultipleTransfers(inner) => {
                    <test_ExecuteBridge_MultipleTransfersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_ExecuteBridge_ReentrancyProtection(inner) => {
                    <test_ExecuteBridge_ReentrancyProtectionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_ExecuteBridge_Success(inner) => {
                    <test_ExecuteBridge_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_GetBridgeInfo(inner) => {
                    <test_GetBridgeInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_GetDailyUsage_AfterTransfer(inner) => {
                    <test_GetDailyUsage_AfterTransferCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_GetDailyUsage_Initial(inner) => {
                    <test_GetDailyUsage_InitialCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RecoverTokens_Success(inner) => {
                    <test_RecoverTokens_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_Constructor_ZeroAdmin(inner) => {
                    <test_RevertWhen_Constructor_ZeroAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_Constructor_ZeroCaller(inner) => {
                    <test_RevertWhen_Constructor_ZeroCallerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_BridgeInactive(inner) => {
                    <test_RevertWhen_ExecuteBridge_BridgeInactiveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_ExceedsDailyLimit(inner) => {
                    <test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_ExceedsMaxSingle(inner) => {
                    <test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_UnauthorizedCaller(inner) => {
                    <test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_ZeroAmount(inner) => {
                    <test_RevertWhen_ExecuteBridge_ZeroAmountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_ZeroToken(inner) => {
                    <test_RevertWhen_ExecuteBridge_ZeroTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_RecoverTokens_NotAdmin(inner) => {
                    <test_RevertWhen_RecoverTokens_NotAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_RecoverTokens_ZeroAddress(inner) => {
                    <test_RevertWhen_RecoverTokens_ZeroAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_SetBridgeTarget_NotAdmin(inner) => {
                    <test_RevertWhen_SetBridgeTarget_NotAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_SetBridgeTarget_ZeroAddress(inner) => {
                    <test_RevertWhen_SetBridgeTarget_ZeroAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_SetBridgeActive_Success(inner) => {
                    <test_SetBridgeActive_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_SetBridgeTarget_Success(inner) => {
                    <test_SetBridgeTarget_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_SetDailyLimit_Success(inner) => {
                    <test_SetDailyLimit_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_SetMaxSingleTransfer_Success(inner) => {
                    <test_SetMaxSingleTransfer_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::token(inner) => {
                    <tokenCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::user(inner) => {
                    <userCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::DAILY_LIMIT(inner) => {
                    <DAILY_LIMITCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::MAX_SINGLE_TRANSFER(inner) => {
                    <MAX_SINGLE_TRANSFERCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::admin(inner) => {
                    <adminCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::bridgeProxy(inner) => {
                    <bridgeProxyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::caller(inner) => {
                    <callerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::newTarget(inner) => {
                    <newTargetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::targetBridge(inner) => {
                    <targetBridgeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::testFuzz_ExecuteBridge_ValidAmounts(inner) => {
                    <testFuzz_ExecuteBridge_ValidAmountsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testFuzz_SetDailyLimit_ValidValues(inner) => {
                    <testFuzz_SetDailyLimit_ValidValuesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_Constructor_RoleAssignment(inner) => {
                    <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_Constructor_Success(inner) => {
                    <test_Constructor_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_DailyLimit_Cumulative(inner) => {
                    <test_DailyLimit_CumulativeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_DailyLimit_Reset(inner) => {
                    <test_DailyLimit_ResetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_ExecuteBridge_MultipleTransfers(inner) => {
                    <test_ExecuteBridge_MultipleTransfersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_ExecuteBridge_ReentrancyProtection(inner) => {
                    <test_ExecuteBridge_ReentrancyProtectionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_ExecuteBridge_Success(inner) => {
                    <test_ExecuteBridge_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_GetBridgeInfo(inner) => {
                    <test_GetBridgeInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_GetDailyUsage_AfterTransfer(inner) => {
                    <test_GetDailyUsage_AfterTransferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_GetDailyUsage_Initial(inner) => {
                    <test_GetDailyUsage_InitialCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RecoverTokens_Success(inner) => {
                    <test_RecoverTokens_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_Constructor_ZeroAdmin(inner) => {
                    <test_RevertWhen_Constructor_ZeroAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_Constructor_ZeroCaller(inner) => {
                    <test_RevertWhen_Constructor_ZeroCallerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_BridgeInactive(inner) => {
                    <test_RevertWhen_ExecuteBridge_BridgeInactiveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_ExceedsDailyLimit(inner) => {
                    <test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_ExceedsMaxSingle(inner) => {
                    <test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_UnauthorizedCaller(inner) => {
                    <test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_ZeroAmount(inner) => {
                    <test_RevertWhen_ExecuteBridge_ZeroAmountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_ZeroToken(inner) => {
                    <test_RevertWhen_ExecuteBridge_ZeroTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_RecoverTokens_NotAdmin(inner) => {
                    <test_RevertWhen_RecoverTokens_NotAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_RecoverTokens_ZeroAddress(inner) => {
                    <test_RevertWhen_RecoverTokens_ZeroAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_SetBridgeTarget_NotAdmin(inner) => {
                    <test_RevertWhen_SetBridgeTarget_NotAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_SetBridgeTarget_ZeroAddress(inner) => {
                    <test_RevertWhen_SetBridgeTarget_ZeroAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_SetBridgeActive_Success(inner) => {
                    <test_SetBridgeActive_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_SetBridgeTarget_Success(inner) => {
                    <test_SetBridgeTarget_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_SetDailyLimit_Success(inner) => {
                    <test_SetDailyLimit_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_SetMaxSingleTransfer_Success(inner) => {
                    <test_SetMaxSingleTransfer_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::token(inner) => {
                    <tokenCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::user(inner) => {
                    <userCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`BaseBridgeProxyTest`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum BaseBridgeProxyTestEvents {
        #[allow(missing_docs)]
        BridgeExecuted(BridgeExecuted),
        #[allow(missing_docs)]
        BridgeStatusUpdated(BridgeStatusUpdated),
        #[allow(missing_docs)]
        BridgeTargetUpdated(BridgeTargetUpdated),
        #[allow(missing_docs)]
        DailyLimitReset(DailyLimitReset),
        #[allow(missing_docs)]
        DailyLimitUpdated(DailyLimitUpdated),
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
    impl BaseBridgeProxyTestEvents {
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
                32u8, 124u8, 76u8, 189u8, 245u8, 94u8, 195u8, 21u8, 161u8, 63u8, 13u8,
                94u8, 4u8, 119u8, 50u8, 236u8, 93u8, 148u8, 125u8, 160u8, 86u8, 231u8,
                6u8, 89u8, 58u8, 165u8, 9u8, 144u8, 153u8, 65u8, 206u8, 223u8,
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
                61u8, 186u8, 64u8, 29u8, 193u8, 171u8, 191u8, 1u8, 112u8, 134u8, 134u8,
                75u8, 51u8, 186u8, 129u8, 95u8, 83u8, 171u8, 60u8, 219u8, 185u8, 54u8,
                107u8, 166u8, 188u8, 216u8, 236u8, 52u8, 45u8, 221u8, 152u8, 232u8,
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
                176u8, 127u8, 139u8, 27u8, 133u8, 4u8, 45u8, 116u8, 2u8, 44u8, 134u8,
                124u8, 131u8, 110u8, 222u8, 176u8, 188u8, 215u8, 14u8, 19u8, 91u8, 0u8,
                66u8, 57u8, 13u8, 43u8, 31u8, 209u8, 8u8, 41u8, 128u8, 105u8,
            ],
            [
                178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8,
                253u8, 68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8,
                197u8, 108u8, 129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
            ],
            [
                179u8, 65u8, 137u8, 137u8, 208u8, 104u8, 53u8, 181u8, 194u8, 21u8, 238u8,
                187u8, 77u8, 84u8, 237u8, 107u8, 231u8, 187u8, 182u8, 110u8, 180u8,
                128u8, 113u8, 100u8, 116u8, 10u8, 46u8, 8u8, 47u8, 167u8, 130u8, 213u8,
            ],
            [
                210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8,
                217u8, 79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8,
                89u8, 79u8, 213u8, 99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
            ],
            [
                218u8, 78u8, 57u8, 221u8, 86u8, 215u8, 44u8, 46u8, 227u8, 209u8, 50u8,
                224u8, 20u8, 107u8, 195u8, 158u8, 144u8, 94u8, 120u8, 227u8, 188u8,
                100u8, 196u8, 1u8, 144u8, 66u8, 28u8, 123u8, 43u8, 206u8, 242u8, 171u8,
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
            ::core::stringify!(log_string),
            ::core::stringify!(log_int),
            ::core::stringify!(DailyLimitUpdated),
            ::core::stringify!(log_bytes),
            ::core::stringify!(log_named_string),
            ::core::stringify!(log_uint),
            ::core::stringify!(log_named_int),
            ::core::stringify!(log_named_array_2),
            ::core::stringify!(BridgeExecuted),
            ::core::stringify!(log_array_2),
            ::core::stringify!(log),
            ::core::stringify!(log_named_decimal_int),
            ::core::stringify!(log_address),
            ::core::stringify!(log_array_1),
            ::core::stringify!(log_named_address),
            ::core::stringify!(log_named_array_1),
            ::core::stringify!(log_named_bytes32),
            ::core::stringify!(BridgeTargetUpdated),
            ::core::stringify!(log_named_uint),
            ::core::stringify!(BridgeStatusUpdated),
            ::core::stringify!(log_named_bytes),
            ::core::stringify!(DailyLimitReset),
            ::core::stringify!(logs),
            ::core::stringify!(log_bytes32),
            ::core::stringify!(log_named_decimal_uint),
            ::core::stringify!(log_array_0),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_string as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <DailyLimitUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_bytes as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_string as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE,
            <BridgeExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_address as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE,
            <BridgeTargetUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <BridgeStatusUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE,
            <DailyLimitReset as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for BaseBridgeProxyTestEvents {
        const NAME: &'static str = "BaseBridgeProxyTestEvents";
        const COUNT: usize = 27usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<BridgeExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BridgeExecuted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::BridgeExecuted)
                }
                Some(
                    <BridgeStatusUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <BridgeStatusUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::BridgeStatusUpdated)
                }
                Some(
                    <BridgeTargetUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <BridgeTargetUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::BridgeTargetUpdated)
                }
                Some(<DailyLimitReset as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DailyLimitReset as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DailyLimitReset)
                }
                Some(
                    <DailyLimitUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DailyLimitUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DailyLimitUpdated)
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
    impl alloy_sol_types::private::IntoLogData for BaseBridgeProxyTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::BridgeExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BridgeStatusUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BridgeTargetUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DailyLimitReset(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DailyLimitUpdated(inner) => {
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
                Self::BridgeExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BridgeStatusUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BridgeTargetUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DailyLimitReset(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DailyLimitUpdated(inner) => {
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
    /**Creates a new wrapper around an on-chain [`BaseBridgeProxyTest`](self) contract instance.

See the [wrapper's documentation](`BaseBridgeProxyTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> BaseBridgeProxyTestInstance<P, N> {
        BaseBridgeProxyTestInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<BaseBridgeProxyTestInstance<P, N>>,
    > {
        BaseBridgeProxyTestInstance::<P, N>::deploy(__provider)
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
        BaseBridgeProxyTestInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`BaseBridgeProxyTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BaseBridgeProxyTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BaseBridgeProxyTestInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for BaseBridgeProxyTestInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BaseBridgeProxyTestInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > BaseBridgeProxyTestInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`BaseBridgeProxyTest`](self) contract instance.

See the [wrapper's documentation](`BaseBridgeProxyTestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<BaseBridgeProxyTestInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> BaseBridgeProxyTestInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BaseBridgeProxyTestInstance<P, N> {
            BaseBridgeProxyTestInstance {
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
    > BaseBridgeProxyTestInstance<P, N> {
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
        ///Creates a new call builder for the [`DAILY_LIMIT`] function.
        pub fn DAILY_LIMIT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DAILY_LIMITCall, N> {
            self.call_builder(&DAILY_LIMITCall)
        }
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<&P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall)
        }
        ///Creates a new call builder for the [`MAX_SINGLE_TRANSFER`] function.
        pub fn MAX_SINGLE_TRANSFER(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, MAX_SINGLE_TRANSFERCall, N> {
            self.call_builder(&MAX_SINGLE_TRANSFERCall)
        }
        ///Creates a new call builder for the [`admin`] function.
        pub fn admin(&self) -> alloy_contract::SolCallBuilder<&P, adminCall, N> {
            self.call_builder(&adminCall)
        }
        ///Creates a new call builder for the [`bridgeProxy`] function.
        pub fn bridgeProxy(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, bridgeProxyCall, N> {
            self.call_builder(&bridgeProxyCall)
        }
        ///Creates a new call builder for the [`caller`] function.
        pub fn caller(&self) -> alloy_contract::SolCallBuilder<&P, callerCall, N> {
            self.call_builder(&callerCall)
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
        ///Creates a new call builder for the [`newTarget`] function.
        pub fn newTarget(&self) -> alloy_contract::SolCallBuilder<&P, newTargetCall, N> {
            self.call_builder(&newTargetCall)
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
        ///Creates a new call builder for the [`targetBridge`] function.
        pub fn targetBridge(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetBridgeCall, N> {
            self.call_builder(&targetBridgeCall)
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
        ///Creates a new call builder for the [`testFuzz_ExecuteBridge_ValidAmounts`] function.
        pub fn testFuzz_ExecuteBridge_ValidAmounts(
            &self,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testFuzz_ExecuteBridge_ValidAmountsCall,
            N,
        > {
            self.call_builder(
                &testFuzz_ExecuteBridge_ValidAmountsCall {
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`testFuzz_SetDailyLimit_ValidValues`] function.
        pub fn testFuzz_SetDailyLimit_ValidValues(
            &self,
            newLimit: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testFuzz_SetDailyLimit_ValidValuesCall,
            N,
        > {
            self.call_builder(
                &testFuzz_SetDailyLimit_ValidValuesCall {
                    newLimit,
                },
            )
        }
        ///Creates a new call builder for the [`test_Constructor_RoleAssignment`] function.
        pub fn test_Constructor_RoleAssignment(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_Constructor_RoleAssignmentCall, N> {
            self.call_builder(&test_Constructor_RoleAssignmentCall)
        }
        ///Creates a new call builder for the [`test_Constructor_Success`] function.
        pub fn test_Constructor_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_Constructor_SuccessCall, N> {
            self.call_builder(&test_Constructor_SuccessCall)
        }
        ///Creates a new call builder for the [`test_DailyLimit_Cumulative`] function.
        pub fn test_DailyLimit_Cumulative(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_DailyLimit_CumulativeCall, N> {
            self.call_builder(&test_DailyLimit_CumulativeCall)
        }
        ///Creates a new call builder for the [`test_DailyLimit_Reset`] function.
        pub fn test_DailyLimit_Reset(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_DailyLimit_ResetCall, N> {
            self.call_builder(&test_DailyLimit_ResetCall)
        }
        ///Creates a new call builder for the [`test_ExecuteBridge_MultipleTransfers`] function.
        pub fn test_ExecuteBridge_MultipleTransfers(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_ExecuteBridge_MultipleTransfersCall,
            N,
        > {
            self.call_builder(&test_ExecuteBridge_MultipleTransfersCall)
        }
        ///Creates a new call builder for the [`test_ExecuteBridge_ReentrancyProtection`] function.
        pub fn test_ExecuteBridge_ReentrancyProtection(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_ExecuteBridge_ReentrancyProtectionCall,
            N,
        > {
            self.call_builder(&test_ExecuteBridge_ReentrancyProtectionCall)
        }
        ///Creates a new call builder for the [`test_ExecuteBridge_Success`] function.
        pub fn test_ExecuteBridge_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_ExecuteBridge_SuccessCall, N> {
            self.call_builder(&test_ExecuteBridge_SuccessCall)
        }
        ///Creates a new call builder for the [`test_GetBridgeInfo`] function.
        pub fn test_GetBridgeInfo(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_GetBridgeInfoCall, N> {
            self.call_builder(&test_GetBridgeInfoCall)
        }
        ///Creates a new call builder for the [`test_GetDailyUsage_AfterTransfer`] function.
        pub fn test_GetDailyUsage_AfterTransfer(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_GetDailyUsage_AfterTransferCall,
            N,
        > {
            self.call_builder(&test_GetDailyUsage_AfterTransferCall)
        }
        ///Creates a new call builder for the [`test_GetDailyUsage_Initial`] function.
        pub fn test_GetDailyUsage_Initial(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_GetDailyUsage_InitialCall, N> {
            self.call_builder(&test_GetDailyUsage_InitialCall)
        }
        ///Creates a new call builder for the [`test_RecoverTokens_Success`] function.
        pub fn test_RecoverTokens_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_RecoverTokens_SuccessCall, N> {
            self.call_builder(&test_RecoverTokens_SuccessCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_Constructor_ZeroAdmin`] function.
        pub fn test_RevertWhen_Constructor_ZeroAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_Constructor_ZeroAdminCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_Constructor_ZeroAdminCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_Constructor_ZeroCaller`] function.
        pub fn test_RevertWhen_Constructor_ZeroCaller(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_Constructor_ZeroCallerCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_Constructor_ZeroCallerCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_ExecuteBridge_BridgeInactive`] function.
        pub fn test_RevertWhen_ExecuteBridge_BridgeInactive(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_ExecuteBridge_BridgeInactiveCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_ExecuteBridge_BridgeInactiveCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_ExecuteBridge_ExceedsDailyLimit`] function.
        pub fn test_RevertWhen_ExecuteBridge_ExceedsDailyLimit(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_ExecuteBridge_ExceedsDailyLimitCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_ExecuteBridge_ExceedsMaxSingle`] function.
        pub fn test_RevertWhen_ExecuteBridge_ExceedsMaxSingle(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_ExecuteBridge_ExceedsMaxSingleCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_ExecuteBridge_UnauthorizedCaller`] function.
        pub fn test_RevertWhen_ExecuteBridge_UnauthorizedCaller(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_ExecuteBridge_UnauthorizedCallerCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_ExecuteBridge_ZeroAmount`] function.
        pub fn test_RevertWhen_ExecuteBridge_ZeroAmount(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_ExecuteBridge_ZeroAmountCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_ExecuteBridge_ZeroAmountCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_ExecuteBridge_ZeroToken`] function.
        pub fn test_RevertWhen_ExecuteBridge_ZeroToken(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_ExecuteBridge_ZeroTokenCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_ExecuteBridge_ZeroTokenCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_RecoverTokens_NotAdmin`] function.
        pub fn test_RevertWhen_RecoverTokens_NotAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_RecoverTokens_NotAdminCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_RecoverTokens_NotAdminCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_RecoverTokens_ZeroAddress`] function.
        pub fn test_RevertWhen_RecoverTokens_ZeroAddress(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_RecoverTokens_ZeroAddressCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_RecoverTokens_ZeroAddressCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_SetBridgeTarget_NotAdmin`] function.
        pub fn test_RevertWhen_SetBridgeTarget_NotAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_SetBridgeTarget_NotAdminCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_SetBridgeTarget_NotAdminCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_SetBridgeTarget_ZeroAddress`] function.
        pub fn test_RevertWhen_SetBridgeTarget_ZeroAddress(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_SetBridgeTarget_ZeroAddressCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_SetBridgeTarget_ZeroAddressCall)
        }
        ///Creates a new call builder for the [`test_SetBridgeActive_Success`] function.
        pub fn test_SetBridgeActive_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_SetBridgeActive_SuccessCall, N> {
            self.call_builder(&test_SetBridgeActive_SuccessCall)
        }
        ///Creates a new call builder for the [`test_SetBridgeTarget_Success`] function.
        pub fn test_SetBridgeTarget_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_SetBridgeTarget_SuccessCall, N> {
            self.call_builder(&test_SetBridgeTarget_SuccessCall)
        }
        ///Creates a new call builder for the [`test_SetDailyLimit_Success`] function.
        pub fn test_SetDailyLimit_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_SetDailyLimit_SuccessCall, N> {
            self.call_builder(&test_SetDailyLimit_SuccessCall)
        }
        ///Creates a new call builder for the [`test_SetMaxSingleTransfer_Success`] function.
        pub fn test_SetMaxSingleTransfer_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_SetMaxSingleTransfer_SuccessCall,
            N,
        > {
            self.call_builder(&test_SetMaxSingleTransfer_SuccessCall)
        }
        ///Creates a new call builder for the [`token`] function.
        pub fn token(&self) -> alloy_contract::SolCallBuilder<&P, tokenCall, N> {
            self.call_builder(&tokenCall)
        }
        ///Creates a new call builder for the [`user`] function.
        pub fn user(&self) -> alloy_contract::SolCallBuilder<&P, userCall, N> {
            self.call_builder(&userCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > BaseBridgeProxyTestInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`BridgeExecuted`] event.
        pub fn BridgeExecuted_filter(
            &self,
        ) -> alloy_contract::Event<&P, BridgeExecuted, N> {
            self.event_filter::<BridgeExecuted>()
        }
        ///Creates a new event filter for the [`BridgeStatusUpdated`] event.
        pub fn BridgeStatusUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, BridgeStatusUpdated, N> {
            self.event_filter::<BridgeStatusUpdated>()
        }
        ///Creates a new event filter for the [`BridgeTargetUpdated`] event.
        pub fn BridgeTargetUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, BridgeTargetUpdated, N> {
            self.event_filter::<BridgeTargetUpdated>()
        }
        ///Creates a new event filter for the [`DailyLimitReset`] event.
        pub fn DailyLimitReset_filter(
            &self,
        ) -> alloy_contract::Event<&P, DailyLimitReset, N> {
            self.event_filter::<DailyLimitReset>()
        }
        ///Creates a new event filter for the [`DailyLimitUpdated`] event.
        pub fn DailyLimitUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, DailyLimitUpdated, N> {
            self.event_filter::<DailyLimitUpdated>()
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
