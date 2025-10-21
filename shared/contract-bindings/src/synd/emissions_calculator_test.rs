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

interface EmissionsCalculatorTest {
    event ChangeFactorSet(uint256 indexed epoch, uint256 changeFactor, address indexed setter);
    event EmissionMinted(uint256 indexed epoch, uint256 amount, uint256 remainingSupply, address indexed to);
    event EmissionsInitialized(uint256 defaultChangeFactor);
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

    function EMISSIONS_CAP() external view returns (uint256);
    function IS_TEST() external view returns (bool);
    function SCALE() external view returns (uint256);
    function TOTAL_EPOCHS() external view returns (uint256);
    function admin() external view returns (address);
    function calculator() external view returns (address);
    function changeFactorManager() external view returns (address);
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
    function testFuzz_ChangeFactor_ValidRange(uint256 changeFactor) external;
    function testFuzz_MultipleEpochs_EmissionSum(uint8 epochs) external;
    function test_CalculateAndMintEmission_FinalEpoch() external;
    function test_CalculateAndMintEmission_FirstEpoch() external;
    function test_CalculateCumulativeProduct() external;
    function test_Constructor_InitialSetup() external view;
    function test_Constructor_RoleAssignment() external view;
    function test_GetEmissionsInfo() external;
    function test_GetRemainingSupply() external view;
    function test_InitializeEmissions_Success() external;
    function test_Integration_FullEmissionCycle() external;
    function test_PreviewCurrentEmission() external;
    function test_RevertWhen_CalculateAndMintEmission_Completed() external;
    function test_RevertWhen_CalculateAndMintEmission_NotInitialized() external;
    function test_RevertWhen_CalculateAndMintEmission_ZeroAddress() external;
    function test_RevertWhen_Constructor_ZeroAddresses() external;
    function test_RevertWhen_InitializeEmissions_AlreadyInitialized() external;
    function test_RevertWhen_InitializeEmissions_InvalidChangeFactor() external;
    function test_RevertWhen_InitializeEmissions_NotAdmin() external;
    function test_SetChangeFactor_Success() external;
    function test_SetChangeFactor_Zero() external;
    function token() external view returns (address);
    function treasury() external view returns (address);
    function user() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "EMISSIONS_CAP",
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
    "name": "SCALE",
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
    "name": "TOTAL_EPOCHS",
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
    "name": "calculator",
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
    "name": "changeFactorManager",
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
    "name": "testFuzz_ChangeFactor_ValidRange",
    "inputs": [
      {
        "name": "changeFactor",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testFuzz_MultipleEpochs_EmissionSum",
    "inputs": [
      {
        "name": "epochs",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_CalculateAndMintEmission_FinalEpoch",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_CalculateAndMintEmission_FirstEpoch",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_CalculateCumulativeProduct",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_Constructor_InitialSetup",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
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
    "name": "test_GetEmissionsInfo",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_GetRemainingSupply",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "test_InitializeEmissions_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_Integration_FullEmissionCycle",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_PreviewCurrentEmission",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_CalculateAndMintEmission_Completed",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_CalculateAndMintEmission_NotInitialized",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_CalculateAndMintEmission_ZeroAddress",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_Constructor_ZeroAddresses",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_InitializeEmissions_AlreadyInitialized",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_InitializeEmissions_InvalidChangeFactor",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_InitializeEmissions_NotAdmin",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_SetChangeFactor_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_SetChangeFactor_Zero",
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
        "internalType": "contract SyndicateToken"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "treasury",
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
    "name": "ChangeFactorSet",
    "inputs": [
      {
        "name": "epoch",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "changeFactor",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "setter",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EmissionMinted",
    "inputs": [
      {
        "name": "epoch",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "remainingSupply",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EmissionsInitialized",
    "inputs": [
      {
        "name": "defaultChangeFactor",
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
pub mod EmissionsCalculatorTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60808060405234606f57600c805460ff199081166001908117909255601f80549091169091179055602180546001600160a01b031990811661123417909155602280548216615678179055602380548216619abc1790556024805490911661111117905561abbc90816100748239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f905f3560e01c9081630489160414614e0c5750806307e6233e146147415780630a6a8336146146a65780630a9254e414614257578063124fb3ce146140085780631ed7831c14613f8a5780632a8ea2e214613d925780632ade388014613b9e5780633e5e3c2314613b205780633f7286f414613aa257806347da5264146136ed5780634f8632ba146136c65780634fddb7a6146133015780635f15c3c9146132e55780635ff4c89914612d2c57806361d027b314612d0557806366a47d6514612b9357806366d9a9a014612a565780636d6d4436146127915780636f7c71ec14612506578063735fb47b146122a657806376029e7814611df057806385226c8114611d665780638f3b08f714611c12578063916a17c614611b6857806395a19046146117ea578063b0464fdc14611740578063b198d0281461171a578063b230c827146113a6578063b5508aa91461131c578063b72a6e9b14611091578063ba414fa61461106c578063befb967914611045578063cbe7fbac14610d00578063ce3e39c014610cd6578063d9a194701461091c578063dccc57f1146106d9578063e20c9f711461064b578063eced552614610628578063f851a44014610601578063fa7626d4146105de578063fc0c546a146105b85763fdc50aca146101f7575f80fd5b346105b55760206003193601126105b55760043560ff8116809103610587576102256030600160ff93615b48565b828060405161023381615194565b600c81527f426f756e6420726573756c74000000000000000000000000000000000000000060208201526040516102d2816102a060208201947fb60e72cc000000000000000000000000000000000000000000000000000000008652604060248401526064830190614fe8565b876044830152037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081018352826151dd565b51906a636f6e736f6c652e6c6f675afa5016816001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610587576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576105a0575b506001600160a01b03601f5460081c16803b156105875781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c5761058b575b5082915b808310610444578360048360206001600160a01b03601f5460081c16604051938480927fdf0244b10000000000000000000000000000000000000000000000000000000082525afa9182156104395783926103ff575b506103f7816103fc93615936565b6159ac565b80f35b91506020823d602011610431575b8161041a602093836151dd565b8101031261042d579051906103f76103e9565b5f80fd5b3d915061040d565b6040513d85823e3d90fd5b90836001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610587576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57610563575b5050601f546023546040517fd3f566ae0000000000000000000000000000000000000000000000000000000081526001600160a01b0391821660048201526024810186905291602091839160081c168188816044810103925af1908115610558578591610526575b5061051d90600192615267565b92019190610393565b90506020813d8211610550575b81610540602093836151dd565b8101031261042d57516001610510565b3d9150610533565b6040513d87823e3d90fd5b8161056d916151dd565b61057857835f6104a8565b8380fd5b6040513d84823e3d90fd5b5080fd5b81610595916151dd565b61058757815f61038f565b816105aa916151dd565b61058757815f610347565b80fd5b50346105b557806003193601126105b55760206001600160a01b03815416604051908152f35b50346105b557806003193601126105b557602060ff601f54166040519015158152f35b50346105b557806003193601126105b55760206001600160a01b0360215416604051908152f35b50346105b557806003193601126105b5576020604051670de0b6b3a76400008152f35b50346105b557806003193601126105b55760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106106ba576106b6856106aa818703826151dd565b60405191829182614fa6565b0390f35b82546001600160a01b0316845260209093019260019283019201610693565b50346105b557806003193601126105b5576001600160a01b03601f5460081c166040517fa217fddf000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104395783916108e9575b506021546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b03166024820152602081604481855afa80156104395783906108aa575b6107969150615a2d565b6040517f89162486000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610439578391610876575b506022546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529060209082908180604481015b03915afa801561057c57829061083b575b6103fc9150615a2d565b506020813d60201161086e575b81610855602093836151dd565b81010312610587576108696103fc9161538f565b610831565b3d9150610848565b90506020813d6020116108a2575b81610891602093836151dd565b8101031261042d57516108206107d3565b3d9150610884565b506020813d6020116108e1575b816108c4602093836151dd565b810103126108dd576108d86107969161538f565b61078c565b8280fd5b3d91506108b7565b90506020813d602011610914575b81610904602093836151dd565b8101031261042d57516020610736565b3d91506108f7565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57610cc1575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57610cac575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152600160048201528160248201526001604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57610c97575b506001600160a01b036022541680827fb813ffbe387d6cf6e6a6f6c5f8905f766a0f1c6cd01c67312f709356c62597bd6020604051670c7d713b49da00008152a3737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57610c82575b506001600160a01b03601f5460081c16803b15610c35578180916024604051809481937f43581010000000000000000000000000000000000000000000000000000000008352670c7d713b49da000060048401525af1801561057c57610c6d575b50600460206001600160a01b03601f5460081c16604051928380927fac12ce070000000000000000000000000000000000000000000000000000000082525afa90811561057c578291610c38575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c3557604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152670c7d713b49da000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561057c57610c245750f35b81610c2e916151dd565b6105b55780f35b50fd5b9150506020813d602011610c65575b81610c54602093836151dd565b8101031261042d578190515f610ba5565b3d9150610c47565b81610c77916151dd565b6105b557805f610b57565b81610c8c916151dd565b6105b557805f610af6565b81610ca1916151dd565b6105b557805f610a5f565b81610cb6916151dd565b6105b557805f6109d8565b81610ccb916151dd565b6105b557805f610990565b50346105b557806003193601126105b55760206001600160a01b03601f5460081c16604051908152f35b50346105b557806003193601126105b557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517f491cc7c200000000000000000000000000000000000000000000000000000000815281600482015281602482015281604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611030575b50507fc12c60abc216286ef25e34b1805a0c3dda73e4c2fd6cf360e807a7a9e73167396020604051670d2f13f7789f00008152a1806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c5761101b575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57611006575b506001600160a01b03601f5460081c166040517f158ef93e000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610439578391610fbd575b50600491610ecf602092615a2d565b604051928380927fac12ce070000000000000000000000000000000000000000000000000000000082525afa90811561057c578291610f88575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c3557604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152670d2f13f7789f000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561057c57610c245750f35b9150506020813d602011610fb5575b81610fa4602093836151dd565b8101031261042d578190515f610f09565b3d9150610f97565b90506020813d602011610ffe575b81610fd8602093836151dd565b81010312610ffa57600491610ecf610ff160209361538f565b92505091610ec0565b5050fd5b3d9150610fcb565b81611010916151dd565b6105b557805f610e73565b81611025916151dd565b6105b557805f610e2b565b8161103a916151dd565b6105b557805f610d94565b50346105b557806003193601126105b55760206001600160a01b0360225416604051908152f35b50346105b557806003193601126105b557602061108761566e565b6040519015158152f35b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611307575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c576112f2575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576112dd575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576112c8575b5060206001600160a01b03601f5460081c166044604051809481937fd3f566ae0000000000000000000000000000000000000000000000000000000083528160048401528160248401525af1801561057c5761129c575080f35b6112bd9060203d6020116112c1575b6112b581836151dd565b81019061521e565b5080f35b503d6112ab565b816112d2916151dd565b6105b557805f611242565b816112e7916151dd565b6105b557805f6111b0565b816112fc916151dd565b6105b557805f61114d565b81611311916151dd565b6105b557805f611105565b50346105b557806003193601126105b55760195461133981615274565b9161134760405193846151dd565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b83831061138957604051806106b68782615080565b6001602081926113988561528c565b815201920192019190611374565b50346105b557806003193601126105b557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611705575b50506001600160a01b03602154166001600160a01b036022541690604051611342928382019082821067ffffffffffffffff8311176116d857606091839161987a95878785398883526020830152604082015203019084f01561057c57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108dd57826040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576116c3575b50506001600160a01b03602054166001600160a01b036022541690604051918483019183831067ffffffffffffffff84111761168157918391606093878785398252876020830152604082015203019084f01561057c57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108dd57826040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576116ae575b50506001600160a01b03602054166001600160a01b036021541690604051938085019385851067ffffffffffffffff86111761168157918593916060959385398252602082015284604082015203019082f0156116755780f35b604051903d90823e3d90fd5b6024877f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b816116b8916151dd565b6108dd57825f61161b565b816116cd916151dd565b6108dd57825f611534565b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b8161170f916151dd565b6105b557805f611447565b50346105b557806003193601126105b55760206040516a422ca8b0a00a42500000008152f35b50346105b557806003193601126105b557601c5461175d81615274565b9161176b60405193846151dd565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b8383106117ad57604051806106b687826150fd565b600260206001926040516117c081615194565b6001600160a01b0386541681526117d88587016153ca565b83820152815201920192019190611798565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611b53575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57611b3e575b505b60308110611a375750806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611a22575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f9e91c9e7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611a0d575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927fd3f566ae0000000000000000000000000000000000000000000000000000000084526004840152603060248401525af1801561057c5761129c575080f35b81611a17916151dd565b6105b557805f6119a6565b81611a2c916151dd565b6105b557805f611914565b816001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610587576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611b29575b5050601f546023546040517fd3f566ae0000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152602481018490529291602091849160081c168186816044810103925af191821561043957600192611b0b575b50016118a8565b611b229060203d81116112c1576112b581836151dd565b505f611b04565b81611b33916151dd565b61058757815f611a9a565b81611b48916151dd565b6105b557805f6118a6565b81611b5d916151dd565b6105b557805f61185e565b50346105b557806003193601126105b557601d54611b8581615274565b91611b9360405193846151dd565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310611bd557604051806106b687826150fd565b60026020600192604051611be881615194565b6001600160a01b038654168152611c008587016153ca565b83820152815201920192019190611bc0565b50346105b557806003193601126105b557806001600160a01b0360245416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611d51575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517ff4844814000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611d3c575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57610c245750f35b81611d46916151dd565b6105b557805f611cf2565b81611d5b916151dd565b6105b557805f611c86565b50346105b557806003193601126105b557601a54611d8381615274565b91611d9160405193846151dd565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b838310611dd357604051806106b68782615080565b600160208192611de28561528c565b815201920192019190611dbe565b50346105b557806003193601126105b5576001600160a01b03601f5460081c16816040517f5bdf6ca1000000000000000000000000000000000000000000000000000000008152602081600481865afa90811561057c578291612264575b506001600160a01b036020541690737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108dd576040517f515361f60000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152911660248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561057c5761224f575b50506040517f5f15c3c9000000000000000000000000000000000000000000000000000000008152602081600481855afa801561043957839061221b575b611f2391506158bf565b6040517fb198d028000000000000000000000000000000000000000000000000000000008152602081600481855afa80156104395783906121e7575b611f699150615747565b816040517feced5526000000000000000000000000000000000000000000000000000000008152602081600481865afa90811561057c5782916121b2575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058757604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152670de0b6b3a764000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561057c5761219d575b50506040517f76671808000000000000000000000000000000000000000000000000000000008152602081600481855afa8015610439578390612169575b61206c9150615849565b6040517fdf0244b1000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610439578391612136575b506004916120b8602092615849565b604051928380927f158ef93e0000000000000000000000000000000000000000000000000000000082525afa801561057c5782906120fb575b6103fc9150615a9f565b506020813d60201161212e575b81612115602093836151dd565b81010312610587576121296103fc9161538f565b6120f1565b3d9150612108565b90506020813d602011612161575b81612151602093836151dd565b8101031261042d575160046120a9565b3d9150612144565b506020813d602011612195575b81612183602093836151dd565b8101031261042d5761206c9051612062565b3d9150612176565b816121a7916151dd565b61058757815f612024565b9150506020813d6020116121df575b816121ce602093836151dd565b8101031261042d578290515f611fa7565b3d91506121c1565b506020813d602011612213575b81612201602093836151dd565b8101031261042d57611f699051611f5f565b3d91506121f4565b506020813d602011612247575b81612235602093836151dd565b8101031261042d57611f239051611f19565b3d9150612228565b81612259916151dd565b61058757815f611edb565b90506020813d60201161229e575b8161227f602093836151dd565b8101031261058757516001600160a01b0381168103610587575f611e4e565b3d9150612272565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576124f1575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c576124dc575b506001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576124c7575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527feb769920000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576124b2575b506001600160a01b03601f5460081c16803b15610c35578180916024604051809481937f435810100000000000000000000000000000000000000000000000000000000083528160048401525af1801561057c57610c245750f35b816124bc916151dd565b6105b557805f612457565b816124d1916151dd565b6105b557805f6123c5565b816124e6916151dd565b6105b557805f612362565b816124fb916151dd565b6105b557805f61231a565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c5761277c575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57612767575b50600460206001600160a01b03601f5460081c16604051928380927f43a3f8a10000000000000000000000000000000000000000000000000000000082525afa90811561057c578291612732575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ffa576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561043957839161271d575b505060206001600160a01b03601f5460081c1660446001600160a01b036023541660405195869384927fd3f566ae00000000000000000000000000000000000000000000000000000000845260048401528160248401525af19081156104395783916126e7575b6103fc9250615936565b90506020823d602011612715575b81612702602093836151dd565b8101031261042d576103fc9151906126dd565b3d91506126f5565b81612727916151dd565b610c3557815f612676565b9150506020813d60201161275f575b8161274e602093836151dd565b8101031261042d578190515f612610565b3d9150612741565b81612771916151dd565b6105b557805f6125c2565b81612786916151dd565b6105b557805f61257a565b50346105b55760206003193601126105b557806127ba670de0b6b3a763ffff6001600435615b48565b81806040516127c881615194565b600c81527f426f756e6420726573756c7400000000000000000000000000000000000000006020820152604051612835816102a060208201947fb60e72cc000000000000000000000000000000000000000000000000000000008652604060248401526064830190614fe8565b51906a636f6e736f6c652e6c6f675afa506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ffa576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610439578391612a41575b50506001600160a01b03601f5460081c1690813b15610ffa57829160248392604051948593849263318e825160e21b845260048401525af1801561057c57612a2c575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57612a17575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927fd3f566ae00000000000000000000000000000000000000000000000000000000845260048401528160248401525af190811561057c5782916129da575b506a422ca8b0a00a4250000000816129d36103fc931515615a2d565b1115615a2d565b90506020813d602011612a0f575b816129f5602093836151dd565b8101031261042d57516a422ca8b0a00a42500000006129b7565b3d91506129e8565b81612a21916151dd565b6105b557805f612951565b81612a36916151dd565b6105b557805f6128ee565b81612a4b916151dd565b610c3557815f6128ab565b50346105b557806003193601126105b557601b54612a7381615274565b612a8060405191826151dd565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b838310612b5857868587604051928392602084019060208552518091526040840160408260051b8601019392905b828210612aed57505050500390f35b91936020612b48827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0600195979984950301865288519083612b388351604084526040840190614fe8565b920151908481840391015261502b565b9601920192018594939192612ade565b60026020600192604051612b6b81615194565b612b748661528c565b8152612b818587016153ca565b83820152815201920192019190612ab0565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57612cf0575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527feb769920000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57612cdb575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b83528160048401525af1801561057c57610c245750f35b81612ce5916151dd565b6105b557805f612c99565b81612cfa916151dd565b6105b557805f612c07565b50346105b557806003193601126105b55760206001600160a01b0360235416604051908152f35b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576132d0575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c576132bb575b505b602f81106131b4575080600460206001600160a01b03601f5460081c16604051928380927fe4b7fb730000000000000000000000000000000000000000000000000000000082525afa90811561057c57829161317f575b506001600160a01b03602054169060206001600160a01b03602354166024604051809581937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa918215610439578392613148575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15613143576040519063ca669fa760e01b82526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156130ed57849161312e575b505060206001600160a01b03601f5460081c1660446001600160a01b036023541660405196879384927fd3f566ae0000000000000000000000000000000000000000000000000000000084526004840152602f60248401525af19283156130ed5784936130f8575b50612f779083615936565b6001600160a01b03602054169060206001600160a01b03602354166024604051809581937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa9182156130ed5784926130b7575b50612fe792612fe191615267565b90615936565b6001600160a01b03601f5460081c166040517f76671808000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610439578391613084575b506004916130426020926158bf565b604051928380927ffa391c640000000000000000000000000000000000000000000000000000000082525afa801561057c57829061083b576103fc9150615a2d565b90506020813d6020116130af575b8161309f602093836151dd565b8101031261042d57516004613033565b3d9150613092565b9091506020813d6020116130e5575b816130d3602093836151dd565b8101031261042d575190612fe7612fd3565b3d91506130c6565b6040513d86823e3d90fd5b9092506020813d602011613126575b81613114602093836151dd565b8101031261042d575191612f77612f6c565b3d9150613107565b81613138916151dd565b610ffa57825f612f04565b505050fd5b925090506020823d602011613177575b81613165602093836151dd565b8101031261042d57829151905f612e9e565b3d9150613158565b9150506020813d6020116131ac575b8161319b602093836151dd565b8101031261042d578190515f612e41565b3d915061318e565b816001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610587576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576132a6575b5050601f546023546040517fd3f566ae0000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152602481018490529291602091849160081c168186816044810103925af191821561043957600192613288575b5001612dea565b61329f9060203d81116112c1576112b581836151dd565b505f613281565b816132b0916151dd565b61058757815f613217565b816132c5916151dd565b6105b557805f612de8565b816132da916151dd565b6105b557805f612da0565b50346105b557806003193601126105b557602060405160308152f35b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576136b1575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c5761369c575b5050600460a06001600160a01b03601f5460081c16604051928380927fa088787d0000000000000000000000000000000000000000000000000000000082525afa90811561057c57613441918384859086928794613661575b61343c949550613437929161342d61343292615849565b6158bf565b615849565b615747565b615a9f565b806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c5761364c575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927fd3f566ae00000000000000000000000000000000000000000000000000000000845260048401528160248401525af190811561057c57829161361a575b50600460a06001600160a01b03601f5460081c16604051928380927fa088787d0000000000000000000000000000000000000000000000000000000082525afa9182156104395783848591869487966135d1575b5091613571849261342d613576956157d2565b615936565b6a422ca8b0a00a425000000003906a422ca8b0a00a425000000082116135a4576103fc929161343c91615936565b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b61342d965061357195506135769350849250613605915060a03d60a011613613575b6135fd81836151dd565b81019061539c565b98509690945090925061355e565b503d6135f3565b90506020813d602011613644575b81613635602093836151dd565b8101031261042d57515f61350a565b3d9150613628565b81613656916151dd565b6105b557805f6134a4565b505050505061343c61343261343761368a61342d9460a03d60a011613613576135fd81836151dd565b93975092955091935090915084613416565b816136a6916151dd565b6105b557805f6133bd565b816136bb916151dd565b6105b557805f613375565b50346105b557806003193601126105b55760206001600160a01b0360245416604051908152f35b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57613a8d575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d99a8cec7e2000060048401525af1801561057c57613a78575b50815b6030811061395157506137be816159ac565b6001600160a01b03601f5460081c1690604051907fdf0244b1000000000000000000000000000000000000000000000000000000008252602082600481865afa80156130ed57849061391d575b6138159250615936565b6040517ffa391c64000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104395783916138d8575b50600491613861602092615a2d565b604051928380927fe4b7fb730000000000000000000000000000000000000000000000000000000082525afa801561057c5782906138a4575b6103fc9150615849565b506020813d6020116138d0575b816138be602093836151dd565b8101031261042d576103fc905161389a565b3d91506138b1565b90506020813d602011613915575b816138f3602093836151dd565b810103126108dd5760049161386161390c60209361538f565b92505091613852565b3d91506138e6565b506020823d602011613949575b81613937602093836151dd565b8101031261042d57613815915161380b565b3d915061392a565b90826001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610587576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57613a63575b5050601f546023546040517fd3f566ae0000000000000000000000000000000000000000000000000000000081526001600160a01b0391821660048201526024810185905291602091839160081c168187816044810103925af19081156130ed578491613a31575b50613a2a90600192615267565b91016137ac565b90506020813d8211613a5b575b81613a4b602093836151dd565b8101031261042d57516001613a1d565b3d9150613a3e565b81613a6d916151dd565b6108dd57825f6139b5565b81613a82916151dd565b6105b557805f6137a9565b81613a97916151dd565b6105b557805f613761565b50346105b557806003193601126105b55760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110613b01576106b6856106aa818703826151dd565b82546001600160a01b0316845260209093019260019283019201613aea565b50346105b557806003193601126105b55760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b818110613b7f576106b6856106aa818703826151dd565b82546001600160a01b0316845260209093019260019283019201613b68565b50346105b557806003193601126105b557601e54613bbb81615274565b613bc860405191826151dd565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b838310613d095786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310613c345786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110613cc057505050505060208060019297019301930190928695949293613c27565b9091929394602080613cfc837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa087600196030189528951614fe8565b9701950193929101613c9c565b604051613d1581615194565b6001600160a01b038354168152600183018054613d3181615274565b91613d3f60405193846151dd565b8183528a526020808b20908b9084015b838210613d75575050505060019282602092836002950152815201920192019190613bf8565b600160208192613d848661528c565b815201930191019091613d4f565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57613f75575b506001600160a01b03601f5460081c16803b15610c3557819060246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57613f60575b5060049060206001600160a01b03601f5460081c16604051938480927fe0e6169c0000000000000000000000000000000000000000000000000000000082525afa918215613f53578192613f1f575b50670d2f13f7789f000060015b60308110613ebc57506103fc9192615936565b90670d2f13f7789f0000810290808204670d2f13f7789f00001490151715613ef257670de0b6b3a7640000600191049101613ea9565b6024837f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b9091506020813d602011613f4b575b81613f3b602093836151dd565b8101031261042d5751905f613e9c565b3d9150613f2e565b50604051903d90823e3d90fd5b613f6b8280926151dd565b6105b5575f613e4d565b81613f7f916151dd565b6105b557805f613e06565b50346105b557806003193601126105b55760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b818110613fe9576106b6856106aa818703826151dd565b82546001600160a01b0316845260209093019260019283019201613fd2565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57614242575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c5761422d575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57614218575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f9e91c9e7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57614203575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670c7d713b49da000060048401525af1801561057c57610c245750f35b8161420d916151dd565b6105b557805f6141b9565b81614222916151dd565b6105b557805f614127565b81614237916151dd565b6105b557805f6140c4565b8161424c916151dd565b6105b557805f61407c565b50346105b557806003193601126105b5576001600160a01b03602154166001600160a01b036023541660405191613b718084019084821067ffffffffffffffff8311176116d857918493916142c593615d0986396001600160a01b0391821681529116602082015260400190565b039082f08015613f53576001600160a01b0316807fffffffffffffffffffffffff000000000000000000000000000000000000000060205416176020556001600160a01b0360215416906001600160a01b036022541660405192611342928385019385851067ffffffffffffffff86111761168157918593916060959361987a863983526020830152604082015203019082f08015613f53577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f5580600460206001600160a01b03815416604051928380927f8d3343d60000000000000000000000000000000000000000000000000000000082525afa90811561057c578291614671575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ffa576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561043957839161465c575b50506001600160a01b0360205416906001600160a01b03601f5460081c16823b15613143576040517f2f2ff15d00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529082908290604490829084905af1801561057c57614647575b50600460206001600160a01b03601f5460081c16604051928380927fdebe4f1f0000000000000000000000000000000000000000000000000000000082525afa90811561057c578291614612575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ffa576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156104395783916145fd575b50506001600160a01b03601f5460081c16906001600160a01b0360215416823b15613143576040517f2f2ff15d00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529082908290604490829084905af1801561057c57610c245750f35b81614607916151dd565b610c3557815f614581565b9150506020813d60201161463f575b8161462e602093836151dd565b8101031261042d578190515f61451b565b3d9150614621565b81614651916151dd565b6105b557805f6144cd565b81614666916151dd565b610c3557815f614453565b9150506020813d60201161469e575b8161468d602093836151dd565b8101031261042d578190515f6143ed565b3d9150614680565b50346105b557806003193601126105b557600460206001600160a01b03601f5460081c16604051928380927fe4b7fb730000000000000000000000000000000000000000000000000000000082525afa801561057c57829061470d575b6103fc9150615747565b506020813d602011614739575b81614727602093836151dd565b8101031261042d576103fc9051614703565b3d915061471a565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57614df7575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57614de2575b50506001600160a01b03601f5460081c16604051907fe4b7fb73000000000000000000000000000000000000000000000000000000008252602082600481845afa918215610439578392614dad575b50602060049161485b84615747565b604051928380927fe0e6169c0000000000000000000000000000000000000000000000000000000082525afa908115610439578391614d7b575b5066b1a2bc2ec50000820282810466b1a2bc2ec5000014831517156135a45781670de0b6b3a76400000391670de0b6b3a76400008311614d4e57670de0b6b3a764000014614d215704906001600160a01b03602054169060206001600160a01b03602354166024604051809581937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa9182156130ed578492614ced575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610578576040517f491cc7c2000000000000000000000000000000000000000000000000000000008152600160048201528460248201528460448201526001606482015284808260848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1918215613f53578592614cd4575b50506149ce9161522d565b837f160fc195d6e53691d30d804ce190dc09471891677e43433b91a7a6131c12a59a60406001600160a01b0360235416938151908782526020820152a3826001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610587576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57614cbf575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927fd3f566ae00000000000000000000000000000000000000000000000000000000845260048401528160248401525af180156130ed5783908590614c89575b614adf9250615936565b6001600160a01b03602054169060206001600160a01b03602354166024604051809581937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa9081156130ed5783928592614c50575b50614b4b92612fe191615267565b6001600160a01b03601f5460081c16906040517f76671808000000000000000000000000000000000000000000000000000000008152602081600481865afa9081156130ed578491614c1d575b50600492614ba76020926157d2565b604051938480927fdf0244b10000000000000000000000000000000000000000000000000000000082525afa8015610439578390614be9576103fc9250615936565b506020823d602011614c15575b81614c03602093836151dd565b8101031261042d576103fc91516126dd565b3d9150614bf6565b90506020813d602011614c48575b81614c38602093836151dd565b8101031261042d57516004614b98565b3d9150614c2b565b925090506020823d602011614c81575b81614c6d602093836151dd565b8101031261042d5790518291614b4b614b3d565b3d9150614c60565b50506020813d602011614cb7575b81614ca4602093836151dd565b8101031261042d5782614adf9151614ad5565b3d9150614c97565b81614cc9916151dd565b6108dd57825f614a6e565b81925090614ce1916151dd565b6105785782845f6149c3565b9091506020813d602011614d19575b81614d09602093836151dd565b8101031261042d5751905f61493b565b3d9150614cfc565b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526012600452fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d602011614da5575b81614d96602093836151dd565b8101031261042d57515f614895565b3d9150614d89565b9091506020813d602011614dda575b81614dc9602093836151dd565b8101031261042d575190602061484c565b3d9150614dbc565b81614dec916151dd565b6105b557805f6147fd565b81614e01916151dd565b6105b557805f6147b5565b90503461042d575f60031936011261042d576001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d5763ca669fa760e01b825260048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015614f9b57614f88575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f9e91c9e7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57614f73575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927fd3f566ae00000000000000000000000000000000000000000000000000000000845260048401528160248401525af1801561057c5761129c575080f35b81614f7d916151dd565b6105b557805f614f0d565b614f9491505f906151dd565b5f5f614e7c565b6040513d5f823e3d90fd5b60206040818301928281528451809452019201905f5b818110614fc95750505090565b82516001600160a01b0316845260209384019390920191600101614fbc565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b8181106150485750505090565b82517fffffffff000000000000000000000000000000000000000000000000000000001684526020938401939092019160010161503b565b602081016020825282518091526040820191602060408360051b8301019401925f915b8383106150b257505050505090565b90919293946020806150ee837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187528951614fe8565b970193019301919392906150a3565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061512f57505050505090565b9091929394602080615185837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b0381511684520151918185820152019061502b565b97019301930191939290615120565b6040810190811067ffffffffffffffff8211176151b057604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176151b057604052565b9081602091031261042d575190565b9190820391821161523a57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b9190820180921161523a57565b67ffffffffffffffff81116151b05760051b60200190565b90604051915f8154908160011c9260018316928315615385575b60208510841461535857848752869390811561531857506001146152d4575b506152d2925003836151dd565b565b90505f9291925260205f20905f915b8183106152fc5750509060206152d2928201015f6152c5565b60209193508060019154838589010152019101909184926152e3565b602093506152d29592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f6152c5565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f16936152a6565b5190811515820361042d57565b908160a091031261042d578051916020820151916040810151916153c760806060840151930161538f565b90565b90604051918281549182825260208201905f5260205f20925f905b8060078301106155e1576152d29454918181106155ab575b818110615575575b81811061553f575b818110615509575b8181106154d3575b81811061549d575b818110615468575b1061543b575b5003836151dd565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f615433565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b16815201930161542d565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301615425565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b16815201930161541d565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301615415565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b16815201930161540d565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b168152019301615405565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b1681520193016153fd565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e08201520194019201859293916153e5565b60085460ff16801561567d5790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115614f9b575f91615715575b50151590565b90506020813d60201161573f575b81615730602093836151dd565b8101031261042d57515f61570f565b3d9150615723565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526a422ca8b0a00a425000000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b5f6152d2916151dd565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152600160248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152603060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051917f98296c54000000000000000000000000000000000000000000000000000000008352600483015260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907f8466f41500000000000000000000000000000000000000000000000000000000825260048201526a422ca8b0a00a425000000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907fa5982885000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b8115615b1b570690565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f90838311615c845782811091821580615c7a575b615c7257615b6b848661522d565b926001840180941161523a57600383111580615c69575b615c5a5760031983101580615c50575b615c3f5785831115615bf657505090615bae84615bb39361522d565b615b11565b908115615bf157615bc49250615267565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff810190811161523a5790565b505090565b959492919095615c07575b50505050565b83949550615bae90615c19939461522d565b908115615bf157615c2a925061522d565b6001810180911161523a57905f808080615c01565b505090506153c7929150199061522d565b5082198411615b92565b50509190506153c79250615267565b50828411615b82565b509250505090565b5084821115615b5d565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603e60248201527f5374645574696c7320626f756e642875696e743235362c75696e743235362c7560448201527f696e74323536293a204d6178206973206c657373207468616e206d696e2e00006064820152fdfe6101806040523461007d5761001b6100156100e2565b90610198565b604051612d0b9081610e06823960805181611cdb015260a05181611d98015260c05181611cac015260e05181611d2a01526101005181611d5001526101205181610d6201526101405181610d8b015261016051818181610c400152610c890152f35b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b601f909101601f19168101906001600160401b038211908210176100b857604052565b610081565b604051906100cc604083610095565b565b51906001600160a01b038216820361007d57565b613b71906040823803928382519485926100fc8285610095565b83398101031261007d5761011b6020610114846100ce565b93016100ce565b90565b6040519061012d604083610095565b60048252565b60405190610142604083610095565b60018252565b60405190610157604083610095565b600982526853796e64696361746560b81b6020830152565b634e487b7160e01b5f52601160045260245ffd5b90629e3400820180921161019357565b61016f565b906101a1610148565b6101a9610148565b906101b261011e565b906314d6539160e21b60208301526101c8610133565b603160f81b60208201908152845190949193916001600160401b0382116100b8576101fd826101f860035461035e565b610396565b602090601f83116001146102d75791806102319261023995945f926102cc575b50508160011b915f199060031b1c19161790565b600355610435565b61024281610684565b6101205261024f82610776565b610140526020815191012060e052519020610100524660a052610270610868565b6080523060c0526001600160a01b038216156102bd576001600160a01b038116156102bd576102b76100cc926102a542610183565b610160526102b25f600c55565b61050e565b50610597565b63d92e233d60e01b5f5260045ffd5b015190505f8061021d565b60035f52601f19831691907fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b925f5b81811061034657509160019391856102399796941061032e575b505050811b01600355610435565b01515f1960f88460031b161c191690555f8080610320565b92936020600181928786015181550195019301610306565b90600182811c9216801561038c575b602083101461037857565b634e487b7160e01b5f52602260045260245ffd5b91607f169161036d565b601f81116103a2575050565b60035f5260205f20906020601f840160051c830193106103dc575b601f0160051c01905b8181106103d1575050565b5f81556001016103c6565b90915081906103bd565b601f82116103f357505050565b5f5260205f20906020601f840160051c8301931061042b575b601f0160051c01905b818110610420575050565b5f8155600101610415565b909150819061040c565b80519091906001600160401b0381116100b85761045e8161045760045461035e565b60046103e6565b602092601f82116001146104925761048d929382915f926102cc5750508160011b915f199060031b1c19161790565b600455565b60045f52601f198216937f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b915f5b8681106104f657508360019596106104de575b505050811b01600455565b01515f1960f88460031b161c191690555f80806104d3565b919260206001819286850151815501940192016104c0565b6001600160a01b0381165f9081525f516020613b315f395f51905f52602052604090205460ff16610592576001600160a01b03165f8181525f516020613b315f395f51905f5260205260408120805460ff191660011790553391907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d8180a4600190565b505f90565b6001600160a01b0381168015610671576002546b02f90193ef3075fa980000008101809111610193576002556001600160a01b0382165f9081526020819052604090206b02f90193ef3075fa9800000081540190555f7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef6040518061062d6b02f90193ef3075fa98000000829190602083019252565b0390a36002546001600160d01b039081811161065c5750506b02f90193ef3075fa980000006100cc915f610931565b630e58ae9360e11b5f5260045260245260445ffd5b63ec442f0560e01b5f525f60045260245ffd5b908151602081105f1461069c57509061011b906108c6565b6001600160401b0381116100b8576106c0816106b960065461035e565b60066103e6565b602092601f82116001146106f7576106ef929382915f926102cc5750508160011b915f199060031b1c19161790565b60065560ff90565b60065f52601f198216937ff652222313e28459528d920b65115c16c04f3efc82aaedc97be59f3f377c0d3f915f5b86811061075e5750836001959610610746575b505050811b0160065560ff90565b01515f1960f88460031b161c191690555f8080610738565b91926020600181928685015181550194019201610725565b908151602081105f1461078e57509061011b906108c6565b6001600160401b0381116100b8576107b2816107ab60075461035e565b60076103e6565b602092601f82116001146107e9576107e1929382915f926102cc5750508160011b915f199060031b1c19161790565b60075560ff90565b60075f52601f198216937fa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c688915f5b8681106108505750836001959610610838575b505050811b0160075560ff90565b01515f1960f88460031b161c191690555f808061082a565b91926020600181928685015181550194019201610817565b60e051610100516040519060208201927f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8452604083015260608201524660808201523060a082015260a081526108c060c082610095565b51902090565b601f8151116108f15760208151910151602082106108e2571790565b5f198260200360031b1b161790565b604460209160405192839163305a27a960e01b83528160048401528051918291826024860152018484015e5f828201840152601f01601f19168101030190fd5b9091906001600160a01b03168015610998575b6100cc926001600160a01b0316908115610980575b5f90815260096020526040808220549282529020546001600160a01b039081169116610b62565b61099161098c84610a33565b610a64565b5050610959565b6109a182610a33565b9265ffffffffffff4311610a1b57600b54806109e557506109db6109cb6100cc955f5b6001610da9565b65ffffffffffff4316600b610cd3565b9050509250610944565b93845f1981011161019357600b5f525f516020613b115f395f51905f52909401546100cc946109db916109cb919060301c6109c4565b6306dfcc6560e41b5f5260306004524360245260445ffd5b6001600160d01b038111610a4d576001600160d01b031690565b6306dfcc6560e41b5f5260d060045260245260445ffd5b65ffffffffffff4311610a1b57600b5480610a8e57506109cb610a8a915f5b6002610da9565b9091565b805f1981011161019357600b5f525f516020613b115f395f51905f520154610a8a916109cb9160301c610a83565b65ffffffffffff4311610a1b57805480610af05750610ae0610a8a925f6002610da9565b9065ffffffffffff431690610cd3565b805f19810111610193575f82815260209020015f190154610a8a92610ae09160301c610a83565b65ffffffffffff4311610a1b57805480610b3b5750610ae0610a8a925f6001610da9565b805f19810111610193575f82815260209020015f190154610a8a92610ae09160301c6109c4565b6001600160a01b03808316939291908116908185141580610c55575b610b8a575b5050505050565b81610bfb575b505082610b9f575b8080610b83565b6001600160a01b03165f908152600a602052604090205f516020613b515f395f51905f5291610bd891610bd29091610a33565b90610b17565b604080516001600160d01b039384168152919092166020820152a25f8080610b98565b6001600160a01b03165f908152600a602052604090205f516020613b515f395f51905f5290610c3390610c2d86610a33565b90610abc565b604080516001600160d01b039384168152919092166020820152a25f80610b90565b50831515610b7e565b5f1981019190821161019357565b908154680100000000000000008110156100b85760018101808455811015610cbf575f9283526020928390208251929093015160301b65ffffffffffff191665ffffffffffff9290921691909117910155565b634e487b7160e01b5f52603260045260245ffd5b80549293928015610d7f57610cea610cf591610c5e565b825f5260205f200190565b8054603081901c9365ffffffffffff91821692918116808411610d7057879303610d3c5750610d3892509065ffffffffffff82549181199060301b169116179055565b9190565b915050610d3891610d5c610d4e6100bd565b65ffffffffffff9093168352565b6001600160d01b0386166020830152610c6c565b632520601d60e01b5f5260045ffd5b5090610da491610d90610d4e6100bd565b6001600160d01b0385166020830152610c6c565b5f9190565b91909180600114610deb57600214610dcf57634e487b7160e01b5f52605160045260245ffd5b6001600160d01b03908116918116919091039081116101935790565b506001600160d01b0391821690821601908111610193579056fe60806040526004361015610011575f80fd5b5f3560e01c806301ffc9a7146102e557806306fdde03146102e0578063095ea7b3146102db57806318160ddd1461022c57806323b872dd146102d6578063248a9ca3146102d15780632f2ff15d146102cc578063313ce567146102c75780633644e515146102c257806336568abe146102bd5780633a46b1a81461023657806340c10f19146102b857806342966c68146102b35780634bf5d7e9146102ae5780634f1bfc9e146102a9578063587cde1e146102a45780635c19a95c1461029f5780636fcfff451461029a57806370a082311461029557806379cc6790146102905780637a8cd1561461028b5780637ecebe001461028657806383f1211b146102815780638426adf21461027c578063844c90261461027757806384b0196e146102725780638a5425211461026d5780638d3343d6146102685780638e539e8c14610263578063902d55a51461025e57806391d148541461025957806391ddadf41461025457806395d89b411461024f5780639ab24eb0146102315780639b7ef64b1461024a578063a217fddf14610245578063a9059cbb14610240578063aa082a9d1461023b578063b0ca253e14610236578063bb4d443614610231578063c02ae7541461022c578063c3cda52014610227578063d505accf14610222578063d547741f1461021d578063dd62ed3e146102185763f1127ed814610213575f80fd5b611515565b6114bc565b61147e565b611324565b6111dd565b610536565b611117565b610722565b6111a0565b61117a565b611160565b61113a565b611072565b611047565b610ff7565b610fd1565b610ef5565b610ebb565b610e81565b610d4a565b610c63565b610c29565b610c05565b610bcd565b610bb3565b610b0a565b610ad5565b610a5a565b610a38565b6109f7565b6109da565b610931565b61090d565b610834565b6106c5565b6106ab565b610690565b61064b565b610618565b610553565b610505565b6103e1565b34610386576020600319360112610386576004357fffffffff00000000000000000000000000000000000000000000000000000000811680910361038657807f7965db0b000000000000000000000000000000000000000000000000000000006020921490811561035c575b506040519015158152f35b7f01ffc9a7000000000000000000000000000000000000000000000000000000009150145f610351565b5f80fd5b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b9060206103de92818152019061038a565b90565b34610386575f600319360112610386576040515f6003546104018161160b565b80845290600181169081156104975750600114610439575b610435836104298185038261174a565b604051918291826103cd565b0390f35b60035f9081527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b939250905b80821061047d57509091508101602001610429610419565b919260018160209254838588010152019101909291610465565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208086019190915291151560051b840190910191506104299050610419565b600435906001600160a01b038216820361038657565b602435906001600160a01b038216820361038657565b346103865760406003193601126103865761052b6105216104d9565b602435903361209e565b602060405160018152f35b34610386575f600319360112610386576020600254604051908152f35b346103865760606003193601126103865761056c6104d9565b6105746104ef565b604435906001600160a01b0383165f5260016020526105a73360405f20906001600160a01b03165f5260205260405f2090565b54925f1984106105c8575b6105bc935061184e565b60405160018152602090f35b8284106105e4576105df836105bc9503338361216c565b6105b2565b82847ffb8f41b2000000000000000000000000000000000000000000000000000000005f523360045260245260445260645ffd5b346103865760206003193601126103865760206106436004355f526005602052600160405f20015490565b604051908152f35b346103865760406003193601126103865761068e60043561066a6104ef565b90610689610684825f526005602052600160405f20015490565b611b89565b611bea565b005b34610386575f60031936011261038657602060405160128152f35b34610386575f600319360112610386576020610643611ca2565b34610386576040600319360112610386576004356106e16104ef565b336001600160a01b038216036106fa5761068e91611dbe565b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b346103865760406003193601126103865761073b6104d9565b6001600160a01b0360243591165f52600a60205261075c60405f2091611e6e565b8154905f8291600584116107dc575b610776935084612487565b806107a5575050602079ffffffffffffffffffffffffffffffffffffffffffffffffffff5f5b16604051908152f35b6020916107cc79ffffffffffffffffffffffffffffffffffffffffffffffffffff926117aa565b905f52825f20015460301c61079c565b91926107e781612312565b810390811161082f5761077693855f5265ffffffffffff8260205f2001541665ffffffffffff8516105f1461081d57509161076b565b9291506108299061178b565b9061076b565b6115de565b346103865760406003193601126103865761084d6104d9565b602435610858611a11565b6001600160a01b038216156108e55780156108bd5760025481810180911161082f576b033b2e3c9fd0803ce8000000106108955761068e916121b3565b7f177e3fc3000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f1f2a2005000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b346103865760206003193601126103865760043580156108bd5761068e9033611ec0565b34610386575f6003193601126103865761094a43612293565b65ffffffffffff8061095b43612293565b169116036109b25761043560405161097460408261174a565b601d81527f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c74000000602082015260405191829160208352602083019061038a565b7f6ff07140000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610386575f600319360112610386576020604051629e34008152f35b34610386576020600319360112610386576001600160a01b03610a186104d9565b165f52600960205260206001600160a01b0360405f205416604051908152f35b346103865760206003193601126103865761068e610a546104d9565b33611f8f565b34610386576020600319360112610386576001600160a01b03610a7b6104d9565b165f52600a60205260405f205463ffffffff8111610aa55760405163ffffffff9091168152602090f35b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52602060045260245260445ffd5b34610386576020600319360112610386576020610643610af36104d9565b6001600160a01b03165f525f60205260405f205490565b3461038657604060031936011261038657610b236104d9565b60243590610b2f611a99565b6001600160a01b0381169081156108e55782156108bd57610b4e6117e4565b15610b8b5782610b5d91611ec0565b6040519182527fbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a260203393a3005b7fb8b5ca2d000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610386575f6003193601126103865760206106436117b8565b34610386576020600319360112610386576001600160a01b03610bee6104d9565b165f526008602052602060405f2054604051908152f35b34610386575f600319360112610386576020610c1f6117e4565b6040519015158152f35b34610386575f6003193601126103865760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b3461038657602060031936011261038657600435610c7f611b21565b42811115610d22577f00000000000000000000000000000000000000000000000000000000000000008111610cfa577fdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec610cf5600c549280600c5560405191829133958360209093929193604081019481520152565b0390a2005b7fef69af65000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fa5658353000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610386575f60031936011261038657610e28610d867f000000000000000000000000000000000000000000000000000000000000000061263e565b610daf7f00000000000000000000000000000000000000000000000000000000000000006126b7565b6020604051610dbe828261174a565b5f815281610e36818301947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe083013687376040519788977f0f00000000000000000000000000000000000000000000000000000000000000895260e0858a015260e089019061038a565b90878203604089015261038a565b914660608701523060808701525f60a087015285830360c087015251918281520192915f5b828110610e6a57505050500390f35b835185528695509381019392810192600101610e5b565b34610386575f6003193601126103865760206040517f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a6748152f35b34610386575f6003193601126103865760206040517f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb8152f35b3461038657602060031936011261038657610f11600435611e6e565b600b54905f829160058411610f7d575b610f2d9350600b612487565b80610f5b575060205f5b79ffffffffffffffffffffffffffffffffffffffffffffffffffff60405191168152f35b610f78610f696020926117aa565b600b5f52825f20015460301c90565b610f37565b9192610f8881612312565b810390811161082f57610f2d93600b5f5265ffffffffffff8260205f2001541665ffffffffffff8516105f14610fbf575091610f21565b929150610fcb9061178b565b90610f21565b34610386575f6003193601126103865760206040516b033b2e3c9fd0803ce80000008152f35b3461038657604060031936011261038657602060ff61103b60043561101a6104ef565b905f526005845260405f20906001600160a01b03165f5260205260405f2090565b54166040519015158152f35b34610386575f60031936011261038657602061106243612293565b65ffffffffffff60405191168152f35b34610386575f600319360112610386576040515f6004546110928161160b565b808452906001811690811561049757506001146110b957610435836104298185038261174a565b60045f9081527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b939250905b8082106110fd57509091508101602001610429610419565b9192600181602092548385880101520191019092916110e5565b346103865760206003193601126103865760206106436111356104d9565b6117fb565b34610386575f6003193601126103865760206040516b02f90193ef3075fa980000008152f35b34610386575f6003193601126103865760206040515f8152f35b346103865760406003193601126103865761052b6111966104d9565b602435903361184e565b34610386575f600319360112610386576020600c54604051908152f35b6064359060ff8216820361038657565b6084359060ff8216820361038657565b346103865760c0600319360112610386576111f66104d9565b602435906044356112056111bd565b6084359060a435928042116112f9579161128b939161127d6112829460405160208101917fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf83526001600160a01b038a1660408301528a606083015260808201526080815261127560a08261174a565b51902061204e565b6126ee565b909291926127b2565b6112af816001600160a01b03165f52600860205260405f2080549060018201905590565b8093036112c05761068e9250611f8f565b6001600160a01b0391507f752d88c0000000000000000000000000000000000000000000000000000000005f521660045260245260445ffd5b7f4683af0e000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b346103865760e06003193601126103865761133d6104d9565b6113456104ef565b60443590606435926113556111cd565b60a43560c43590864211611452576113fe926113f961138e866001600160a01b03165f52600860205260405f2080549060018201905590565b9860405160208101917f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983526001600160a01b0389169b8c60408401526001600160a01b038b1660608401528b608084015260a083015260c082015260c0815261127560e08261174a565b61208f565b936001600160a01b038516036114185761068e935061209e565b7f4b800e46000000000000000000000000000000000000000000000000000000005f526001600160a01b038085166004521660245260445ffd5b867f62791302000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b346103865760406003193601126103865761068e60043561149d6104ef565b906114b7610684825f526005602052600160405f20015490565b611dbe565b3461038657604060031936011261038657602061150c6114da6104d9565b6001600160a01b036114ea6104ef565b91165f526001835260405f20906001600160a01b03165f5260205260405f2090565b54604051908152f35b346103865760406003193601126103865761152e6104d9565b6024359063ffffffff8216820361038657610435916001600160a01b0361157b92611557611836565b50611560611836565b50165f52600a60205260405f20611575611836565b50612879565b506040519061158982611729565b5465ffffffffffff8116825260301c602082015260405191829182919091602079ffffffffffffffffffffffffffffffffffffffffffffffffffff81604084019565ffffffffffff8151168552015116910152565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b90600182811c92168015611652575b602083101461162557565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f169161161a565b5f929181549161166b8361160b565b80835292600181169081156116c0575060011461168757505050565b5f9081526020812093945091925b8383106116a6575060209250010190565b600181602092949394548385870101520191019190611695565b905060209495507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091509291921683830152151560051b010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040810190811067ffffffffffffffff82111761174557604052565b6116fc565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761174557604052565b906001820180921161082f57565b604051906117a860408361174a565b565b905f19820191821161082f57565b600c54801580156117da575b6117d55742810390811161082f5790565b505f90565b50804210156117c4565b600c5480151590816117f4575090565b9050421090565b6001600160a01b03165f52600a60205279ffffffffffffffffffffffffffffffffffffffffffffffffffff61183260405f20612025565b1690565b6040519061184382611729565b5f6020838281520152565b9291906001600160a01b0384169384156119e5576001600160a01b03821680156119b95761187a6117e4565b80611981575b6119595761189e826001600160a01b03165f525f60205260405f2090565b549584871061191a57846117a89697036118c8846001600160a01b03165f525f60205260405f2090565b556118e3846001600160a01b03165f525f60205260405f2090565b8054860190556040518581527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a3612a50565b7fe450d38c000000000000000000000000000000000000000000000000000000005f526001600160a01b0383166004526024879052604485905260645ffd5b7fdb89e3f4000000000000000000000000000000000000000000000000000000005f5260045ffd5b50335f9081527f740796d87e4f86cc94671768c744956045fe855093291e79194c96ac478040aa602052604090205460ff1615611880565b7fec442f05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7f96c6fd1e000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b335f9081527f9a6bf48bb840e78fe8e7afd10d3d391a91738a9e6524f6fdfa1a3aba9dc03fb1602052604090205460ff1615611a4957565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb60245260445ffd5b335f9081527f740796d87e4f86cc94671768c744956045fe855093291e79194c96ac478040aa602052604090205460ff1615611ad157565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67460245260445ffd5b335f9081527f05b8ccbb9d4d8fb16ea74ce3c29a41f1b461fbdaff4714a0d9a8eb05499746bc602052604090205460ff1615611b5957565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b805f52600560205260ff611bb13360405f20906001600160a01b03165f5260205260405f2090565b541615611bbb5750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f52600560205260ff611c128360405f20906001600160a01b03165f5260205260405f2090565b5416611c9c57805f526005602052611c3e8260405f20906001600160a01b03165f5260205260405f2090565b60017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008254161790556001600160a01b03339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016301480611d95575b15611cfd577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f82527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a08152611d8f60c08261174a565b51902090565b507f00000000000000000000000000000000000000000000000000000000000000004614611cd4565b805f52600560205260ff611de68360405f20906001600160a01b03165f5260205260405f2090565b541615611c9c57805f526005602052611e138260405f20906001600160a01b03165f5260205260405f2090565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0081541690556001600160a01b03339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190565b65ffffffffffff611e7e43612293565b1680821015611e9157506103de90612293565b907fecd3f81e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b6001600160a01b03811680156119e557611eea826001600160a01b03165f525f60205260405f2090565b54838110611f5257915f8092856117a8969503611f17846001600160a01b03165f525f60205260405f2090565b556002805486900390556040518581527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a3612a50565b7fe450d38c000000000000000000000000000000000000000000000000000000005f526001600160a01b038316600452602452604483905260645ffd5b6001600160a01b038181165f81815260096020526040812080548685167fffffffffffffffffffffffff0000000000000000000000000000000000000000821681179092556117a89694169461201f9390928691907f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9080a46001600160a01b03165f525f60205260405f205490565b916124eb565b8054806120325750505f90565b805f1981011161082f575f19915f5260205f2001015460301c90565b604290612059611ca2565b90604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b916103de9391611282936126ee565b6001600160a01b0316908115612140576001600160a01b03811692831561211457806121077f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92593855f52600160205260405f20906001600160a01b03165f5260205260405f2090565b55604051908152602090a3565b7f94280d62000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7fe602df05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6001600160a01b0316908115612140576001600160a01b03811615612114576121b0915f52600160205260405f20906001600160a01b03165f5260205260405f2090565b55565b919060025481810180911161082f576002556001600160a01b0383168061226e5781600254036002555b6040518281525f907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a36002549279ffffffffffffffffffffffffffffffffffffffffffffffffffff80851161223e57506117a89293505f612a50565b7f1cb15d26000000000000000000000000000000000000000000000000000000005f52600485905260245260445ffd5b612288846001600160a01b03165f525f60205260405f2090565b8281540190556121dd565b65ffffffffffff81116122ab5765ffffffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52603060045260245260445ffd5b81156122e5570490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b60018111156103de57806001700100000000000000000000000000000000831015612445575b6123eb6123e16123d76123cd6123c36123b96123a86123f29760048a680100000000000000006123f79c1015612438575b64010000000081101561242b575b6201000081101561241e575b610100811015612411575b6010811015612404575b10156123fc575b60030260011c90565b6123b2818b6122db565b0160011c90565b6123b2818a6122db565b6123b281896122db565b6123b281886122db565b6123b281876122db565b6123b281866122db565b80936122db565b821190565b900390565b60011b61239f565b60041c9160021b91612398565b60081c9160041b9161238e565b60101c9160081b91612383565b60201c9160101b91612377565b60401c9160201b91612369565b50506123f76123f26123eb6123e16123d76123cd6123c36123b96123a861246c8a60801c90565b98506801000000000000000097506123389650505050505050565b91905b8382106124975750505090565b9091928083169080841860011c820180921161082f57845f5265ffffffffffff8260205f2001541665ffffffffffff8416105f146124d95750925b919061248a565b9392506124e59061178b565b916124d2565b91906001600160a01b038116926001600160a01b038116908482141580612635575b612519575b5050505050565b816125bf575b50508261252e575b8080612512565b6125b461259b7fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a7249361259561258f79ffffffffffffffffffffffffffffffffffffffffffffffffffff956001600160a01b03165f52600a60205260405f2090565b916128bb565b9061298f565b6040805192851683529316602082015291829190820190565b0390a25f8080612527565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff61262b61259b61261c7fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724946001600160a01b03165f52600a60205260405f2090565b612625886128bb565b9061292b565b0390a25f8061251f565b5083151561250d565b60ff811461269d5760ff811690601f8211612675576040519161266260408461174a565b6020808452838101919036833783525290565b7fb3512b0c000000000000000000000000000000000000000000000000000000005f5260045ffd5b506040516103de816126b081600661165c565b038261174a565b60ff81146126db5760ff811690601f8211612675576040519161266260408461174a565b506040516103de816126b081600761165c565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411612770579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15612765575f516001600160a01b0381161561275b57905f905f90565b505f906001905f90565b6040513d5f823e3d90fd5b5050505f9160039190565b6004111561278557565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6127bb8161277b565b806127c4575050565b6127cd8161277b565b600181036127fd577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b6128068161277b565b6002810361283a57507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b8061284660039261277b565b1461284e5750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b805482101561288e575f5260205f2001905f90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b79ffffffffffffffffffffffffffffffffffffffffffffffffffff81116128fb5779ffffffffffffffffffffffffffffffffffffffffffffffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f5260d060045260245260445ffd5b9061293543612293565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff8061295b85612025565b92169116039079ffffffffffffffffffffffffffffffffffffffffffffffffffff821161082f5761298b92612bd8565b9091565b9061299943612293565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff806129bf85612025565b92169116019079ffffffffffffffffffffffffffffffffffffffffffffffffffff821161082f5761298b92612bd8565b6129f843612293565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff80612a1f600b612025565b921691160379ffffffffffffffffffffffffffffffffffffffffffffffffffff811161082f5761298b91600b612bd8565b9091906001600160a01b03168015612ac1575b6001600160a01b036117a89316908115612aa9575b5f5260096020526001600160a01b0360405f205416905f5260096020526001600160a01b0360405f205416906124eb565b612aba612ab5846128bb565b6129ef565b5050612a78565b612aca826128bb565b92612ad443612293565b9379ffffffffffffffffffffffffffffffffffffffffffffffffffff80612afb600b612025565b92169116019079ffffffffffffffffffffffffffffffffffffffffffffffffffff821161082f576117a8946001600160a01b0392612b3a91600b612bd8565b905050935050612a63565b80546801000000000000000081101561174557612b6791600182018155612879565b612bac5781516020929092015160301b7fffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000001665ffffffffffff92909216919091179055565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b80549293928015612cce57612bef612bfa916117aa565b825f5260205f200190565b8054603081901c9365ffffffffffff91821692918116808411612ca657879303612c5f5750612c5b92509065ffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffffffff00000000000083549260301b169116179055565b9190565b915050612c5b91612c7f612c71611799565b65ffffffffffff9093168352565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff86166020830152612b45565b7f2520601d000000000000000000000000000000000000000000000000000000005f5260045ffd5b5090612d0691612cdf612c71611799565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff85166020830152612b45565b5f9190560175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db805b8ccbb9d4d8fb16ea74ce3c29a41f1b461fbdaff4714a0d9a8eb05499746bcdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72460a0346100d957601f61134238819003918201601f19168301916001600160401b038311848410176100dd578084926060946040528339810103126100d957610047816100f1565b61005f6040610058602085016100f1565b93016100f1565b906001600160a01b031680156100ca576001600160a01b038316156100ca576001600160a01b038216156100ca576100a39261009d91608052610105565b5061017b565b506040516110d3908161020f8239608051818181610321015281816107f10152610d540152f35b63d92e233d60e01b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036100d957565b6001600160a01b0381165f9081525f5160206113225f395f51905f52602052604090205460ff16610176576001600160a01b03165f8181525f5160206113225f395f51905f5260205260408120805460ff191660011790553391905f5160206112e25f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f5160206113025f395f51905f52602052604090205460ff16610176576001600160a01b03165f8181525f5160206113025f395f51905f5260205260408120805460ff191660011790553391907ff27f9368aa1b7697e50429f83722b88f2e5db8184d5e2cbcf060f4310bbd3e7d905f5160206112e25f395f51905f529080a460019056fe6080806040526004361015610012575f80fd5b5f905f3560e01c90816301ffc9a7146109fa57508063158ef93e146109d8578063248a9ca3146109ae5780632f2ff15d1461097157806336568abe14610905578063435810101461082f57806343a3f8a1146108155780635bdf6ca1146107c55780635f15c3c9146107aa578063766718081461078d578063891624861461075357806391d14854146106fd578063a088787d146106ba578063a217fddf146106a0578063ac12ce0714610683578063b198d0281461065e578063c63a094414610550578063d3f566ae14610256578063d547741f1461020f578063debe4f1f146101d4578063df0244b1146101b6578063e0e6169c1461019b578063e4b7fb7314610178578063eced5526146101555763fa391c6414610131575f80fd5b34610152578060031936011261015257602060306002541015604051908152f35b80fd5b50346101525780600319360112610152576020604051670de0b6b3a76400008152f35b50346101525780600319360112610152576020610193610d2c565b604051908152f35b50346101525780600319360112610152576020610193610cef565b50346101525780600319360112610152576020600354604051908152f35b503461015257806003193601126101525760206040517f076eb8b875b6ea839b087c4c0c1a4661b089d3b6ee2c1ef1b9cfa7fe1066d2068152f35b50346101525760406003193601126101525761025260043561022f610a98565b9061024d610248825f525f602052600160405f20015490565b610ed3565b61100b565b5080f35b5034610451576040600319360112610451576004359073ffffffffffffffffffffffffffffffffffffffff821680920361045157335f9081527f0e25390ff9535358a5e916dfe7d38266c83601af6e112105b22df4a90bf8910160205260409020546024359060ff16156105005760ff6004541615610482576002549060308210156104825783156104d8578082036104aa5750506102f3610b05565b908115610482576003548281018091116104555760035573ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b15610451575f80916044604051809481937f40c10f190000000000000000000000000000000000000000000000000000000083528960048401528860248401525af1801561044657610433575b5060025492837f160fc195d6e53691d30d804ce190dc09471891677e43433b91a7a6131c12a59a60406103c1610d2c565b8151908782526020820152a37fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83146104065750600160209201600255604051908152f35b807f4e487b7100000000000000000000000000000000000000000000000000000000602492526011600452fd5b61043f91505f90610c81565b5f5f610390565b6040513d5f823e3d90fd5b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7f9e91c9e7000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f30413a1a000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f076eb8b875b6ea839b087c4c0c1a4661b089d3b6ee2c1ef1b9cfa7fe1066d20660245260445ffd5b3461045157602060031936011261045157335f9081527fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb560205260409020546004359060ff161561062e5760045460ff8116610482578115610606577fc12c60abc216286ef25e34b1805a0c3dda73e4c2fd6cf360e807a7a9e73167399160017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00602093161760045580600155604051908152a1005b7feb769920000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b34610451575f6003193601126104515760206040516a422ca8b0a00a42500000008152f35b34610451575f600319360112610451576020600154604051908152f35b34610451575f6003193601126104515760206040515f8152f35b34610451575f6003193601126104515760a06002546030600354916106dd610d2c565b604051938285528360208601526040850152606084015210156080820152f35b3461045157604060031936011261045157610716610a98565b6004355f525f60205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060ff60405f2054166040519015158152f35b34610451575f6003193601126104515760206040517ff27f9368aa1b7697e50429f83722b88f2e5db8184d5e2cbcf060f4310bbd3e7d8152f35b34610451575f600319360112610451576020600254604051908152f35b34610451575f60031936011261045157602060405160308152f35b34610451575f60031936011261045157602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610451575f600319360112610451576020610193610b05565b3461045157602060031936011261045157335f9081527f7e97327757452d9e3315e395bc100e7da1f2d35106fffd38f23807747090efa960205260409020546004359060ff16156108b557801561060657806001556002546040519182527fb813ffbe387d6cf6e6a6f6c5f8905f766a0f1c6cd01c67312f709356c62597bd60203393a3005b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527ff27f9368aa1b7697e50429f83722b88f2e5db8184d5e2cbcf060f4310bbd3e7d60245260445ffd5b346104515760406003193601126104515761091e610a98565b3373ffffffffffffffffffffffffffffffffffffffff821603610949576109479060043561100b565b005b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461045157604060031936011261045157610947600435610990610a98565b906109a9610248825f525f602052600160405f20015490565b610f39565b346104515760206003193601126104515760206101936004355f525f602052600160405f20015490565b34610451575f60031936011261045157602060ff600454166040519015158152f35b3461045157602060031936011261045157600435907fffffffff00000000000000000000000000000000000000000000000000000000821680920361045157817f7965db0b0000000000000000000000000000000000000000000000000000000060209314908115610a6e575b5015158152f35b7f01ffc9a70000000000000000000000000000000000000000000000000000000091501483610a67565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361045157565b8115610ac5570490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b8181029291811591840414171561045557565b60ff60045416158015610c74575b610c70576002546030036030811161045557610b2d610d2c565b60018214610c6b5760015491670de0b6b3a76400008314610c605750610b51610cef565b670de0b6b3a7640000811115610c19577ffffffffffffffffffffffffffffffffffffffffffffffffff21f494c589c00008101908111610455575b6103e8811015610c1357506103e8905b670de0b6b3a7640000831115610beb577ffffffffffffffffffffffffffffffffffffffffffffffffff21f494c589c0000830192831161045557610be892610be391610af2565b610abb565b90565b91670de0b6b3a76400000391670de0b6b3a7640000831161045557610be892610be391610af2565b90610b9c565b670de0b6b3a764000003670de0b6b3a7640000811115610b8c577f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b90610be89250610abb565b905090565b5f90565b5060306002541015610b13565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610cc257604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b670de0b6b3a76400006002546001545b60308210610d0c57505090565b9091670de0b6b3a7640000610d2383600193610af2565b04920190610cff565b6040517f18160ddd0000000000000000000000000000000000000000000000000000000081527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16602082600481845afa918215610446575f92610e9e575b506020600491604051928380927f902d55a50000000000000000000000000000000000000000000000000000000082525afa908115610446575f91610e6c575b507fffffffffffffffffffffffffffffffffffffffffffbdd3574f5ff5bdb0000000810181811161045557821115610e645781036a422ca8b0a00a425000000001908111610455575b806a422ca8b0a00a4250000000115f14610e5f576a422ca8b0a00a4250000000036a422ca8b0a00a425000000081116104555790565b505f90565b50505f610e29565b90506020813d602011610e96575b81610e8760209383610c81565b8101031261045157515f610de0565b3d9150610e7a565b9091506020813d602011610ecb575b81610eba60209383610c81565b810103126104515751906020610da0565b3d9150610ead565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f20541615610f0a5750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f1461100557805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f1461100557805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d7e97327757452d9e3315e395bc100e7da1f2d35106fffd38f23807747090efa9ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`oW`\x0C\x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U`\x1F\x80T\x90\x91\x16\x90\x91\x17\x90U`!\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16a\x124\x17\x90\x91U`\"\x80T\x82\x16aVx\x17\x90U`#\x80T\x82\x16a\x9A\xBC\x17\x90U`$\x80T\x90\x91\x16a\x11\x11\x17\x90Ua\xAB\xBC\x90\x81a\0t\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x04\x89\x16\x04\x14aN\x0CWP\x80c\x07\xE6#>\x14aGAW\x80c\nj\x836\x14aF\xA6W\x80c\n\x92T\xE4\x14aBWW\x80c\x12O\xB3\xCE\x14a@\x08W\x80c\x1E\xD7\x83\x1C\x14a?\x8AW\x80c*\x8E\xA2\xE2\x14a=\x92W\x80c*\xDE8\x80\x14a;\x9EW\x80c>^<#\x14a; W\x80c?r\x86\xF4\x14a:\xA2W\x80cG\xDARd\x14a6\xEDW\x80cO\x862\xBA\x14a6\xC6W\x80cO\xDD\xB7\xA6\x14a3\x01W\x80c_\x15\xC3\xC9\x14a2\xE5W\x80c_\xF4\xC8\x99\x14a-,W\x80ca\xD0'\xB3\x14a-\x05W\x80cf\xA4}e\x14a+\x93W\x80cf\xD9\xA9\xA0\x14a*VW\x80cmmD6\x14a'\x91W\x80co|q\xEC\x14a%\x06W\x80cs_\xB4{\x14a\"\xA6W\x80cv\x02\x9Ex\x14a\x1D\xF0W\x80c\x85\"l\x81\x14a\x1DfW\x80c\x8F;\x08\xF7\x14a\x1C\x12W\x80c\x91j\x17\xC6\x14a\x1BhW\x80c\x95\xA1\x90F\x14a\x17\xEAW\x80c\xB0FO\xDC\x14a\x17@W\x80c\xB1\x98\xD0(\x14a\x17\x1AW\x80c\xB20\xC8'\x14a\x13\xA6W\x80c\xB5P\x8A\xA9\x14a\x13\x1CW\x80c\xB7*n\x9B\x14a\x10\x91W\x80c\xBAAO\xA6\x14a\x10lW\x80c\xBE\xFB\x96y\x14a\x10EW\x80c\xCB\xE7\xFB\xAC\x14a\r\0W\x80c\xCE>9\xC0\x14a\x0C\xD6W\x80c\xD9\xA1\x94p\x14a\t\x1CW\x80c\xDC\xCCW\xF1\x14a\x06\xD9W\x80c\xE2\x0C\x9Fq\x14a\x06KW\x80c\xEC\xEDU&\x14a\x06(W\x80c\xF8Q\xA4@\x14a\x06\x01W\x80c\xFAv&\xD4\x14a\x05\xDEW\x80c\xFC\x0CTj\x14a\x05\xB8Wc\xFD\xC5\n\xCA\x14a\x01\xF7W_\x80\xFD[4a\x05\xB5W` `\x03\x196\x01\x12a\x05\xB5W`\x045`\xFF\x81\x16\x80\x91\x03a\x05\x87Wa\x02%`0`\x01`\xFF\x93a[HV[\x82\x80`@Qa\x023\x81aQ\x94V[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Qa\x02\xD2\x81a\x02\xA0` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`@`$\x84\x01R`d\x83\x01\x90aO\xE8V[\x87`D\x83\x01R\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82aQ\xDDV[Q\x90jconsole.logZ\xFAP\x16\x81`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x05\xA0W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x05\x87W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x05\x8BW[P\x82\x91[\x80\x83\x10a\x04DW\x83`\x04\x83` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F\xDF\x02D\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x049W\x83\x92a\x03\xFFW[Pa\x03\xF7\x81a\x03\xFC\x93aY6V[aY\xACV[\x80\xF3[\x91P` \x82=` \x11a\x041W[\x81a\x04\x1A` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x90Q\x90a\x03\xF7a\x03\xE9V[_\x80\xFD[=\x91Pa\x04\rV[`@Q=\x85\x82>=\x90\xFD[\x90\x83`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x05cW[PP`\x1FT`#T`@Q\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91` \x91\x83\x91`\x08\x1C\x16\x81\x88\x81`D\x81\x01\x03\x92Z\xF1\x90\x81\x15a\x05XW\x85\x91a\x05&W[Pa\x05\x1D\x90`\x01\x92aRgV[\x92\x01\x91\x90a\x03\x93V[\x90P` \x81=\x82\x11a\x05PW[\x81a\x05@` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ`\x01a\x05\x10V[=\x91Pa\x053V[`@Q=\x87\x82>=\x90\xFD[\x81a\x05m\x91aQ\xDDV[a\x05xW\x83_a\x04\xA8V[\x83\x80\xFD[`@Q=\x84\x82>=\x90\xFD[P\x80\xFD[\x81a\x05\x95\x91aQ\xDDV[a\x05\x87W\x81_a\x03\x8FV[\x81a\x05\xAA\x91aQ\xDDV[a\x05\x87W\x81_a\x03GV[\x80\xFD[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `@Qg\r\xE0\xB6\xB3\xA7d\0\0\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x06\xBAWa\x06\xB6\x85a\x06\xAA\x81\x87\x03\x82aQ\xDDV[`@Q\x91\x82\x91\x82aO\xA6V[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x06\x93V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x049W\x83\x91a\x08\xE9W[P`!T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R` \x81`D\x81\x85Z\xFA\x80\x15a\x049W\x83\x90a\x08\xAAW[a\x07\x96\x91PaZ-V[`@Q\x7F\x89\x16$\x86\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x049W\x83\x91a\x08vW[P`\"T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90` \x90\x82\x90\x81\x80`D\x81\x01[\x03\x91Z\xFA\x80\x15a\x05|W\x82\x90a\x08;W[a\x03\xFC\x91PaZ-V[P` \x81=` \x11a\x08nW[\x81a\x08U` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x05\x87Wa\x08ia\x03\xFC\x91aS\x8FV[a\x081V[=\x91Pa\x08HV[\x90P` \x81=` \x11a\x08\xA2W[\x81a\x08\x91` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQa\x08 a\x07\xD3V[=\x91Pa\x08\x84V[P` \x81=` \x11a\x08\xE1W[\x81a\x08\xC4` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x08\xDDWa\x08\xD8a\x07\x96\x91aS\x8FV[a\x07\x8CV[\x82\x80\xFD[=\x91Pa\x08\xB7V[\x90P` \x81=` \x11a\t\x14W[\x81a\t\x04` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ` a\x076V[=\x91Pa\x08\xF7V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x0C\xC1W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x0C\xACW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R\x81`$\x82\x01R`\x01`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x0C\x97W[P`\x01`\x01`\xA0\x1B\x03`\"T\x16\x80\x82\x7F\xB8\x13\xFF\xBE8}l\xF6\xE6\xA6\xF6\xC5\xF8\x90_vj\x0F\x1Cl\xD0\x1Cg1/p\x93V\xC6%\x97\xBD` `@Qg\x0C}q;I\xDA\0\0\x81R\xA3sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x0C\x82W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7FCX\x10\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\x0C}q;I\xDA\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x0CmW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xAC\x12\xCE\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|W\x82\x91a\x0C8W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rg\x0C}q;I\xDA\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05|Wa\x0C$WP\xF3[\x81a\x0C.\x91aQ\xDDV[a\x05\xB5W\x80\xF3[P\xFD[\x91PP` \x81=` \x11a\x0CeW[\x81a\x0CT` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x81\x90Q_a\x0B\xA5V[=\x91Pa\x0CGV[\x81a\x0Cw\x91aQ\xDDV[a\x05\xB5W\x80_a\x0BWV[\x81a\x0C\x8C\x91aQ\xDDV[a\x05\xB5W\x80_a\n\xF6V[\x81a\x0C\xA1\x91aQ\xDDV[a\x05\xB5W\x80_a\n_V[\x81a\x0C\xB6\x91aQ\xDDV[a\x05\xB5W\x80_a\t\xD8V[\x81a\x0C\xCB\x91aQ\xDDV[a\x05\xB5W\x80_a\t\x90V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81`\x04\x82\x01R\x81`$\x82\x01R\x81`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x100W[PP\x7F\xC1,`\xAB\xC2\x16(n\xF2^4\xB1\x80Z\x0C=\xDAs\xE4\xC2\xFDl\xF3`\xE8\x07\xA7\xA9\xE71g9` `@Qg\r/\x13\xF7x\x9F\0\0\x81R\xA1\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x10\x1BW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x10\x06W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x15\x8E\xF9>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x049W\x83\x91a\x0F\xBDW[P`\x04\x91a\x0E\xCF` \x92aZ-V[`@Q\x92\x83\x80\x92\x7F\xAC\x12\xCE\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|W\x82\x91a\x0F\x88W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rg\r/\x13\xF7x\x9F\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05|Wa\x0C$WP\xF3[\x91PP` \x81=` \x11a\x0F\xB5W[\x81a\x0F\xA4` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x81\x90Q_a\x0F\tV[=\x91Pa\x0F\x97V[\x90P` \x81=` \x11a\x0F\xFEW[\x81a\x0F\xD8` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x0F\xFAW`\x04\x91a\x0E\xCFa\x0F\xF1` \x93aS\x8FV[\x92PP\x91a\x0E\xC0V[PP\xFD[=\x91Pa\x0F\xCBV[\x81a\x10\x10\x91aQ\xDDV[a\x05\xB5W\x80_a\x0EsV[\x81a\x10%\x91aQ\xDDV[a\x05\xB5W\x80_a\x0E+V[\x81a\x10:\x91aQ\xDDV[a\x05\xB5W\x80_a\r\x94V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` a\x10\x87aVnV[`@Q\x90\x15\x15\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x13\x07W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x12\xF2W[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x12\xDDW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x12\xC8W[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`@Q\x80\x94\x81\x93\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a\x05|Wa\x12\x9CWP\x80\xF3[a\x12\xBD\x90` =` \x11a\x12\xC1W[a\x12\xB5\x81\x83aQ\xDDV[\x81\x01\x90aR\x1EV[P\x80\xF3[P=a\x12\xABV[\x81a\x12\xD2\x91aQ\xDDV[a\x05\xB5W\x80_a\x12BV[\x81a\x12\xE7\x91aQ\xDDV[a\x05\xB5W\x80_a\x11\xB0V[\x81a\x12\xFC\x91aQ\xDDV[a\x05\xB5W\x80_a\x11MV[\x81a\x13\x11\x91aQ\xDDV[a\x05\xB5W\x80_a\x11\x05V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x19Ta\x139\x81aRtV[\x91a\x13G`@Q\x93\x84aQ\xDDV[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\x13\x89W`@Q\x80a\x06\xB6\x87\x82aP\x80V[`\x01` \x81\x92a\x13\x98\x85aR\x8CV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x13tV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x17\x05W[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90`@Qa\x13B\x92\x83\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x16\xD8W``\x91\x83\x91a\x98z\x95\x87\x87\x859\x88\x83R` \x83\x01R`@\x82\x01R\x03\x01\x90\x84\xF0\x15a\x05|Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xDDW\x82`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x16\xC3W[PP`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90`@Q\x91\x84\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x16\x81W\x91\x83\x91``\x93\x87\x87\x859\x82R\x87` \x83\x01R`@\x82\x01R\x03\x01\x90\x84\xF0\x15a\x05|Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xDDW\x82`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x16\xAEW[PP`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`@Q\x93\x80\x85\x01\x93\x85\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x16\x81W\x91\x85\x93\x91``\x95\x93\x859\x82R` \x82\x01R\x84`@\x82\x01R\x03\x01\x90\x82\xF0\x15a\x16uW\x80\xF3[`@Q\x90=\x90\x82>=\x90\xFD[`$\x87\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81a\x16\xB8\x91aQ\xDDV[a\x08\xDDW\x82_a\x16\x1BV[\x81a\x16\xCD\x91aQ\xDDV[a\x08\xDDW\x82_a\x154V[`$\x86\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81a\x17\x0F\x91aQ\xDDV[a\x05\xB5W\x80_a\x14GV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `@QjB,\xA8\xB0\xA0\nBP\0\0\0\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x1CTa\x17]\x81aRtV[\x91a\x17k`@Q\x93\x84aQ\xDDV[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\x17\xADW`@Q\x80a\x06\xB6\x87\x82aP\xFDV[`\x02` `\x01\x92`@Qa\x17\xC0\x81aQ\x94V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x17\xD8\x85\x87\x01aS\xCAV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x17\x98V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x1BSW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x1B>W[P[`0\x81\x10a\x1A7WP\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x1A\"W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x9E\x91\xC9\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x1A\rW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`0`$\x84\x01RZ\xF1\x80\x15a\x05|Wa\x12\x9CWP\x80\xF3[\x81a\x1A\x17\x91aQ\xDDV[a\x05\xB5W\x80_a\x19\xA6V[\x81a\x1A,\x91aQ\xDDV[a\x05\xB5W\x80_a\x19\x14V[\x81`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x1B)W[PP`\x1FT`#T`@Q\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x92\x91` \x91\x84\x91`\x08\x1C\x16\x81\x86\x81`D\x81\x01\x03\x92Z\xF1\x91\x82\x15a\x049W`\x01\x92a\x1B\x0BW[P\x01a\x18\xA8V[a\x1B\"\x90` =\x81\x11a\x12\xC1Wa\x12\xB5\x81\x83aQ\xDDV[P_a\x1B\x04V[\x81a\x1B3\x91aQ\xDDV[a\x05\x87W\x81_a\x1A\x9AV[\x81a\x1BH\x91aQ\xDDV[a\x05\xB5W\x80_a\x18\xA6V[\x81a\x1B]\x91aQ\xDDV[a\x05\xB5W\x80_a\x18^V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x1DTa\x1B\x85\x81aRtV[\x91a\x1B\x93`@Q\x93\x84aQ\xDDV[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x1B\xD5W`@Q\x80a\x06\xB6\x87\x82aP\xFDV[`\x02` `\x01\x92`@Qa\x1B\xE8\x81aQ\x94V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x1C\0\x85\x87\x01aS\xCAV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x1B\xC0V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`$T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x1DQW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xF4\x84H\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x1D<W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x0C$WP\xF3[\x81a\x1DF\x91aQ\xDDV[a\x05\xB5W\x80_a\x1C\xF2V[\x81a\x1D[\x91aQ\xDDV[a\x05\xB5W\x80_a\x1C\x86V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x1ATa\x1D\x83\x81aRtV[\x91a\x1D\x91`@Q\x93\x84aQ\xDDV[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\x1D\xD3W`@Q\x80a\x06\xB6\x87\x82aP\x80V[`\x01` \x81\x92a\x1D\xE2\x85aR\x8CV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x1D\xBEV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x81`@Q\x7F[\xDFl\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x05|W\x82\x91a\"dW[P`\x01`\x01`\xA0\x1B\x03` T\x16\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xDDW`@Q\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05|Wa\"OW[PP`@Q\x7F_\x15\xC3\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x049W\x83\x90a\"\x1BW[a\x1F#\x91PaX\xBFV[`@Q\x7F\xB1\x98\xD0(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x049W\x83\x90a!\xE7W[a\x1Fi\x91PaWGV[\x81`@Q\x7F\xEC\xEDU&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x05|W\x82\x91a!\xB2W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05|Wa!\x9DW[PP`@Q\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x049W\x83\x90a!iW[a l\x91PaXIV[`@Q\x7F\xDF\x02D\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x049W\x83\x91a!6W[P`\x04\x91a \xB8` \x92aXIV[`@Q\x92\x83\x80\x92\x7F\x15\x8E\xF9>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05|W\x82\x90a \xFBW[a\x03\xFC\x91PaZ\x9FV[P` \x81=` \x11a!.W[\x81a!\x15` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x05\x87Wa!)a\x03\xFC\x91aS\x8FV[a \xF1V[=\x91Pa!\x08V[\x90P` \x81=` \x11a!aW[\x81a!Q` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ`\x04a \xA9V[=\x91Pa!DV[P` \x81=` \x11a!\x95W[\x81a!\x83` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa l\x90Qa bV[=\x91Pa!vV[\x81a!\xA7\x91aQ\xDDV[a\x05\x87W\x81_a $V[\x91PP` \x81=` \x11a!\xDFW[\x81a!\xCE` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x82\x90Q_a\x1F\xA7V[=\x91Pa!\xC1V[P` \x81=` \x11a\"\x13W[\x81a\"\x01` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa\x1Fi\x90Qa\x1F_V[=\x91Pa!\xF4V[P` \x81=` \x11a\"GW[\x81a\"5` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa\x1F#\x90Qa\x1F\x19V[=\x91Pa\"(V[\x81a\"Y\x91aQ\xDDV[a\x05\x87W\x81_a\x1E\xDBV[\x90P` \x81=` \x11a\"\x9EW[\x81a\"\x7F` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x05\x87WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x05\x87W_a\x1ENV[=\x91Pa\"rV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa$\xF1W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa$\xDCW[P`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa$\xC7W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xEBv\x99 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa$\xB2W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7FCX\x10\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x0C$WP\xF3[\x81a$\xBC\x91aQ\xDDV[a\x05\xB5W\x80_a$WV[\x81a$\xD1\x91aQ\xDDV[a\x05\xB5W\x80_a#\xC5V[\x81a$\xE6\x91aQ\xDDV[a\x05\xB5W\x80_a#bV[\x81a$\xFB\x91aQ\xDDV[a\x05\xB5W\x80_a#\x1AV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa'|W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa'gW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7FC\xA3\xF8\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|W\x82\x91a'2W[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0F\xFAW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x049W\x83\x91a'\x1DW[PP` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x95\x86\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x90\x81\x15a\x049W\x83\x91a&\xE7W[a\x03\xFC\x92PaY6V[\x90P` \x82=` \x11a'\x15W[\x81a'\x02` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa\x03\xFC\x91Q\x90a&\xDDV[=\x91Pa&\xF5V[\x81a''\x91aQ\xDDV[a\x0C5W\x81_a&vV[\x91PP` \x81=` \x11a'_W[\x81a'N` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x81\x90Q_a&\x10V[=\x91Pa'AV[\x81a'q\x91aQ\xDDV[a\x05\xB5W\x80_a%\xC2V[\x81a'\x86\x91aQ\xDDV[a\x05\xB5W\x80_a%zV[P4a\x05\xB5W` `\x03\x196\x01\x12a\x05\xB5W\x80a'\xBAg\r\xE0\xB6\xB3\xA7c\xFF\xFF`\x01`\x045a[HV[\x81\x80`@Qa'\xC8\x81aQ\x94V[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Qa(5\x81a\x02\xA0` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`@`$\x84\x01R`d\x83\x01\x90aO\xE8V[Q\x90jconsole.logZ\xFAP`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0F\xFAW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x049W\x83\x91a*AW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x0F\xFAW\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92c1\x8E\x82Q`\xE2\x1B\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa*,W[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa*\x17W[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x90\x81\x15a\x05|W\x82\x91a)\xDAW[PjB,\xA8\xB0\xA0\nBP\0\0\0\x81a)\xD3a\x03\xFC\x93\x15\x15aZ-V[\x11\x15aZ-V[\x90P` \x81=` \x11a*\x0FW[\x81a)\xF5` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQjB,\xA8\xB0\xA0\nBP\0\0\0a)\xB7V[=\x91Pa)\xE8V[\x81a*!\x91aQ\xDDV[a\x05\xB5W\x80_a)QV[\x81a*6\x91aQ\xDDV[a\x05\xB5W\x80_a(\xEEV[\x81a*K\x91aQ\xDDV[a\x0C5W\x81_a(\xABV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x1BTa*s\x81aRtV[a*\x80`@Q\x91\x82aQ\xDDV[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a+XW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a*\xEDWPPPP\x03\x90\xF3[\x91\x93` a+H\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a+8\x83Q`@\x84R`@\x84\x01\x90aO\xE8V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01RaP+V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a*\xDEV[`\x02` `\x01\x92`@Qa+k\x81aQ\x94V[a+t\x86aR\x8CV[\x81Ra+\x81\x85\x87\x01aS\xCAV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a*\xB0V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa,\xF0W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xEBv\x99 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa,\xDBW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x0C$WP\xF3[\x81a,\xE5\x91aQ\xDDV[a\x05\xB5W\x80_a,\x99V[\x81a,\xFA\x91aQ\xDDV[a\x05\xB5W\x80_a,\x07V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa2\xD0W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa2\xBBW[P[`/\x81\x10a1\xB4WP\x80`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xE4\xB7\xFBs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|W\x82\x91a1\x7FW[P`\x01`\x01`\xA0\x1B\x03` T\x16\x90` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x95\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a\x049W\x83\x92a1HW[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a1CW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a0\xEDW\x84\x91a1.W[PP` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x96\x87\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`/`$\x84\x01RZ\xF1\x92\x83\x15a0\xEDW\x84\x93a0\xF8W[Pa/w\x90\x83aY6V[`\x01`\x01`\xA0\x1B\x03` T\x16\x90` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x95\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a0\xEDW\x84\x92a0\xB7W[Pa/\xE7\x92a/\xE1\x91aRgV[\x90aY6V[`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x049W\x83\x91a0\x84W[P`\x04\x91a0B` \x92aX\xBFV[`@Q\x92\x83\x80\x92\x7F\xFA9\x1Cd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05|W\x82\x90a\x08;Wa\x03\xFC\x91PaZ-V[\x90P` \x81=` \x11a0\xAFW[\x81a0\x9F` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ`\x04a03V[=\x91Pa0\x92V[\x90\x91P` \x81=` \x11a0\xE5W[\x81a0\xD3` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ\x90a/\xE7a/\xD3V[=\x91Pa0\xC6V[`@Q=\x86\x82>=\x90\xFD[\x90\x92P` \x81=` \x11a1&W[\x81a1\x14` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ\x91a/wa/lV[=\x91Pa1\x07V[\x81a18\x91aQ\xDDV[a\x0F\xFAW\x82_a/\x04V[PPP\xFD[\x92P\x90P` \x82=` \x11a1wW[\x81a1e` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x82\x91Q\x90_a.\x9EV[=\x91Pa1XV[\x91PP` \x81=` \x11a1\xACW[\x81a1\x9B` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x81\x90Q_a.AV[=\x91Pa1\x8EV[\x81`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa2\xA6W[PP`\x1FT`#T`@Q\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x92\x91` \x91\x84\x91`\x08\x1C\x16\x81\x86\x81`D\x81\x01\x03\x92Z\xF1\x91\x82\x15a\x049W`\x01\x92a2\x88W[P\x01a-\xEAV[a2\x9F\x90` =\x81\x11a\x12\xC1Wa\x12\xB5\x81\x83aQ\xDDV[P_a2\x81V[\x81a2\xB0\x91aQ\xDDV[a\x05\x87W\x81_a2\x17V[\x81a2\xC5\x91aQ\xDDV[a\x05\xB5W\x80_a-\xE8V[\x81a2\xDA\x91aQ\xDDV[a\x05\xB5W\x80_a-\xA0V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `@Q`0\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa6\xB1W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa6\x9CW[PP`\x04`\xA0`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xA0\x88x}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|Wa4A\x91\x83\x84\x85\x90\x86\x92\x87\x94a6aW[a4<\x94\x95Pa47\x92\x91a4-a42\x92aXIV[aX\xBFV[aXIV[aWGV[aZ\x9FV[\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa6LW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x90\x81\x15a\x05|W\x82\x91a6\x1AW[P`\x04`\xA0`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xA0\x88x}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x049W\x83\x84\x85\x91\x86\x94\x87\x96a5\xD1W[P\x91a5q\x84\x92a4-a5v\x95aW\xD2V[aY6V[jB,\xA8\xB0\xA0\nBP\0\0\0\x03\x90jB,\xA8\xB0\xA0\nBP\0\0\0\x82\x11a5\xA4Wa\x03\xFC\x92\x91a4<\x91aY6V[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[a4-\x96Pa5q\x95Pa5v\x93P\x84\x92Pa6\x05\x91P`\xA0=`\xA0\x11a6\x13W[a5\xFD\x81\x83aQ\xDDV[\x81\x01\x90aS\x9CV[\x98P\x96\x90\x94P\x90\x92Pa5^V[P=a5\xF3V[\x90P` \x81=` \x11a6DW[\x81a65` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ_a5\nV[=\x91Pa6(V[\x81a6V\x91aQ\xDDV[a\x05\xB5W\x80_a4\xA4V[PPPPPa4<a42a47a6\x8Aa4-\x94`\xA0=`\xA0\x11a6\x13Wa5\xFD\x81\x83aQ\xDDV[\x93\x97P\x92\x95P\x91\x93P\x90\x91P\x84a4\x16V[\x81a6\xA6\x91aQ\xDDV[a\x05\xB5W\x80_a3\xBDV[\x81a6\xBB\x91aQ\xDDV[a\x05\xB5W\x80_a3uV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x90\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa:\x8DW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r\x99\xA8\xCE\xC7\xE2\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa:xW[P\x81[`0\x81\x10a9QWPa7\xBE\x81aY\xACV[`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`@Q\x90\x7F\xDF\x02D\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x86Z\xFA\x80\x15a0\xEDW\x84\x90a9\x1DW[a8\x15\x92PaY6V[`@Q\x7F\xFA9\x1Cd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x049W\x83\x91a8\xD8W[P`\x04\x91a8a` \x92aZ-V[`@Q\x92\x83\x80\x92\x7F\xE4\xB7\xFBs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05|W\x82\x90a8\xA4W[a\x03\xFC\x91PaXIV[P` \x81=` \x11a8\xD0W[\x81a8\xBE` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa\x03\xFC\x90Qa8\x9AV[=\x91Pa8\xB1V[\x90P` \x81=` \x11a9\x15W[\x81a8\xF3` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x08\xDDW`\x04\x91a8aa9\x0C` \x93aS\x8FV[\x92PP\x91a8RV[=\x91Pa8\xE6V[P` \x82=` \x11a9IW[\x81a97` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa8\x15\x91Qa8\x0BV[=\x91Pa9*V[\x90\x82`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa:cW[PP`\x1FT`#T`@Q\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91` \x91\x83\x91`\x08\x1C\x16\x81\x87\x81`D\x81\x01\x03\x92Z\xF1\x90\x81\x15a0\xEDW\x84\x91a:1W[Pa:*\x90`\x01\x92aRgV[\x91\x01a7\xACV[\x90P` \x81=\x82\x11a:[W[\x81a:K` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ`\x01a:\x1DV[=\x91Pa:>V[\x81a:m\x91aQ\xDDV[a\x08\xDDW\x82_a9\xB5V[\x81a:\x82\x91aQ\xDDV[a\x05\xB5W\x80_a7\xA9V[\x81a:\x97\x91aQ\xDDV[a\x05\xB5W\x80_a7aV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a;\x01Wa\x06\xB6\x85a\x06\xAA\x81\x87\x03\x82aQ\xDDV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a:\xEAV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a;\x7FWa\x06\xB6\x85a\x06\xAA\x81\x87\x03\x82aQ\xDDV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a;hV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x1ETa;\xBB\x81aRtV[a;\xC8`@Q\x91\x82aQ\xDDV[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a=\tW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a<4W\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a<\xC0WPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a<'V[\x90\x91\x92\x93\x94` \x80a<\xFC\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89QaO\xE8V[\x97\x01\x95\x01\x93\x92\x91\x01a<\x9CV[`@Qa=\x15\x81aQ\x94V[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta=1\x81aRtV[\x91a=?`@Q\x93\x84aQ\xDDV[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a=uWPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a;\xF8V[`\x01` \x81\x92a=\x84\x86aR\x8CV[\x81R\x01\x93\x01\x91\x01\x90\x91a=OV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa?uW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x90`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa?`W[P`\x04\x90` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F\xE0\xE6\x16\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a?SW\x81\x92a?\x1FW[Pg\r/\x13\xF7x\x9F\0\0`\x01[`0\x81\x10a>\xBCWPa\x03\xFC\x91\x92aY6V[\x90g\r/\x13\xF7x\x9F\0\0\x81\x02\x90\x80\x82\x04g\r/\x13\xF7x\x9F\0\0\x14\x90\x15\x17\x15a>\xF2Wg\r\xE0\xB6\xB3\xA7d\0\0`\x01\x91\x04\x91\x01a>\xA9V[`$\x83\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90\x91P` \x81=` \x11a?KW[\x81a?;` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ\x90_a>\x9CV[=\x91Pa?.V[P`@Q\x90=\x90\x82>=\x90\xFD[a?k\x82\x80\x92aQ\xDDV[a\x05\xB5W_a>MV[\x81a?\x7F\x91aQ\xDDV[a\x05\xB5W\x80_a>\x06V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a?\xE9Wa\x06\xB6\x85a\x06\xAA\x81\x87\x03\x82aQ\xDDV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a?\xD2V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|WaBBW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|WaB-W[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|WaB\x18W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x9E\x91\xC9\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|WaB\x03W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\x0C}q;I\xDA\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x0C$WP\xF3[\x81aB\r\x91aQ\xDDV[a\x05\xB5W\x80_aA\xB9V[\x81aB\"\x91aQ\xDDV[a\x05\xB5W\x80_aA'V[\x81aB7\x91aQ\xDDV[a\x05\xB5W\x80_a@\xC4V[\x81aBL\x91aQ\xDDV[a\x05\xB5W\x80_a@|V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x91a;q\x80\x84\x01\x90\x84\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x16\xD8W\x91\x84\x93\x91aB\xC5\x93a]\t\x869`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[\x03\x90\x82\xF0\x80\x15a?SW`\x01`\x01`\xA0\x1B\x03\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x13B\x92\x83\x85\x01\x93\x85\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x16\x81W\x91\x85\x93\x91``\x95\x93a\x98z\x869\x83R` \x83\x01R`@\x82\x01R\x03\x01\x90\x82\xF0\x80\x15a?SW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU\x80`\x04` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x92\x83\x80\x92\x7F\x8D3C\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|W\x82\x91aFqW[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0F\xFAW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x049W\x83\x91aF\\W[PP`\x01`\x01`\xA0\x1B\x03` T\x16\x90`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x82;\x15a1CW`@Q\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05|WaFGW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xDE\xBEO\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|W\x82\x91aF\x12W[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0F\xFAW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x049W\x83\x91aE\xFDW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`\x01`\x01`\xA0\x1B\x03`!T\x16\x82;\x15a1CW`@Q\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05|Wa\x0C$WP\xF3[\x81aF\x07\x91aQ\xDDV[a\x0C5W\x81_aE\x81V[\x91PP` \x81=` \x11aF?W[\x81aF.` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x81\x90Q_aE\x1BV[=\x91PaF!V[\x81aFQ\x91aQ\xDDV[a\x05\xB5W\x80_aD\xCDV[\x81aFf\x91aQ\xDDV[a\x0C5W\x81_aDSV[\x91PP` \x81=` \x11aF\x9EW[\x81aF\x8D` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x81\x90Q_aC\xEDV[=\x91PaF\x80V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xE4\xB7\xFBs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05|W\x82\x90aG\rW[a\x03\xFC\x91PaWGV[P` \x81=` \x11aG9W[\x81aG'` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa\x03\xFC\x90QaG\x03V[=\x91PaG\x1AV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|WaM\xF7W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|WaM\xE2W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x7F\xE4\xB7\xFBs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\x049W\x83\x92aM\xADW[P` `\x04\x91aH[\x84aWGV[`@Q\x92\x83\x80\x92\x7F\xE0\xE6\x16\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x049W\x83\x91aM{W[Pf\xB1\xA2\xBC.\xC5\0\0\x82\x02\x82\x81\x04f\xB1\xA2\xBC.\xC5\0\0\x14\x83\x15\x17\x15a5\xA4W\x81g\r\xE0\xB6\xB3\xA7d\0\0\x03\x91g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11aMNWg\r\xE0\xB6\xB3\xA7d\0\0\x14aM!W\x04\x90`\x01`\x01`\xA0\x1B\x03` T\x16\x90` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x95\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a0\xEDW\x84\x92aL\xEDW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05xW`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R\x84`$\x82\x01R\x84`D\x82\x01R`\x01`d\x82\x01R\x84\x80\x82`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x91\x82\x15a?SW\x85\x92aL\xD4W[PPaI\xCE\x91aR-V[\x83\x7F\x16\x0F\xC1\x95\xD6\xE56\x91\xD3\r\x80L\xE1\x90\xDC\tG\x18\x91g~CC;\x91\xA7\xA6\x13\x1C\x12\xA5\x9A`@`\x01`\x01`\xA0\x1B\x03`#T\x16\x93\x81Q\x90\x87\x82R` \x82\x01R\xA3\x82`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|WaL\xBFW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a0\xEDW\x83\x90\x85\x90aL\x89W[aJ\xDF\x92PaY6V[`\x01`\x01`\xA0\x1B\x03` T\x16\x90` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x95\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a0\xEDW\x83\x92\x85\x92aLPW[PaKK\x92a/\xE1\x91aRgV[`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`@Q\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a0\xEDW\x84\x91aL\x1DW[P`\x04\x92aK\xA7` \x92aW\xD2V[`@Q\x93\x84\x80\x92\x7F\xDF\x02D\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x049W\x83\x90aK\xE9Wa\x03\xFC\x92PaY6V[P` \x82=` \x11aL\x15W[\x81aL\x03` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa\x03\xFC\x91Qa&\xDDV[=\x91PaK\xF6V[\x90P` \x81=` \x11aLHW[\x81aL8` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ`\x04aK\x98V[=\x91PaL+V[\x92P\x90P` \x82=` \x11aL\x81W[\x81aLm` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x90Q\x82\x91aKKaK=V[=\x91PaL`V[PP` \x81=` \x11aL\xB7W[\x81aL\xA4` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x82aJ\xDF\x91QaJ\xD5V[=\x91PaL\x97V[\x81aL\xC9\x91aQ\xDDV[a\x08\xDDW\x82_aJnV[\x81\x92P\x90aL\xE1\x91aQ\xDDV[a\x05xW\x82\x84_aI\xC3V[\x90\x91P` \x81=` \x11aM\x19W[\x81aM\t` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ\x90_aI;V[=\x91PaL\xFCV[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x12`\x04R\xFD[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11aM\xA5W[\x81aM\x96` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ_aH\x95V[=\x91PaM\x89V[\x90\x91P` \x81=` \x11aM\xDAW[\x81aM\xC9` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ\x90` aHLV[=\x91PaM\xBCV[\x81aM\xEC\x91aQ\xDDV[a\x05\xB5W\x80_aG\xFDV[\x81aN\x01\x91aQ\xDDV[a\x05\xB5W\x80_aG\xB5V[\x90P4a\x04-W_`\x03\x196\x01\x12a\x04-W`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-Wc\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aO\x9BWaO\x88W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x9E\x91\xC9\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|WaOsW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a\x05|Wa\x12\x9CWP\x80\xF3[\x81aO}\x91aQ\xDDV[a\x05\xB5W\x80_aO\rV[aO\x94\x91P_\x90aQ\xDDV[__aN|V[`@Q=_\x82>=\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10aO\xC9WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aO\xBCV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10aPHWPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aP;V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aP\xB2WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aP\xEE\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89QaO\xE8V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aP\xA3V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aQ/WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aQ\x85\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90aP+V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aQ V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aQ\xB0W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aQ\xB0W`@RV[\x90\x81` \x91\x03\x12a\x04-WQ\x90V[\x91\x90\x82\x03\x91\x82\x11aR:WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x91\x90\x82\x01\x80\x92\x11aR:WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11aQ\xB0W`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15aS\x85W[` \x85\x10\x84\x14aSXW\x84\x87R\x86\x93\x90\x81\x15aS\x18WP`\x01\x14aR\xD4W[PaR\xD2\x92P\x03\x83aQ\xDDV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10aR\xFCWPP\x90` aR\xD2\x92\x82\x01\x01_aR\xC5V[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92aR\xE3V[` \x93PaR\xD2\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_aR\xC5V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93aR\xA6V[Q\x90\x81\x15\x15\x82\x03a\x04-WV[\x90\x81`\xA0\x91\x03\x12a\x04-W\x80Q\x91` \x82\x01Q\x91`@\x81\x01Q\x91aS\xC7`\x80``\x84\x01Q\x93\x01aS\x8FV[\x90V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10aU\xE1WaR\xD2\x94T\x91\x81\x81\x10aU\xABW[\x81\x81\x10aUuW[\x81\x81\x10aU?W[\x81\x81\x10aU\tW[\x81\x81\x10aT\xD3W[\x81\x81\x10aT\x9DW[\x81\x81\x10aThW[\x10aT;W[P\x03\x83aQ\xDDV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_aT3V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01aT-V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01aT%V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01aT\x1DV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01aT\x15V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01aT\rV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01aT\x05V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01aS\xFDV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91aS\xE5V[`\x08T`\xFF\x16\x80\x15aV}W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15aO\x9BW_\x91aW\x15W[P\x15\x15\x90V[\x90P` \x81=` \x11aW?W[\x81aW0` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ_aW\x0FV[=\x91PaW#V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01RjB,\xA8\xB0\xA0\nBP\0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[_aR\xD2\x91aQ\xDDV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x01`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\x84f\xF4\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01RjB,\xA8\xB0\xA0\nBP\0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[\x81\x15a[\x1BW\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x90\x83\x83\x11a\\\x84W\x82\x81\x10\x91\x82\x15\x80a\\zW[a\\rWa[k\x84\x86aR-V[\x92`\x01\x84\x01\x80\x94\x11aR:W`\x03\x83\x11\x15\x80a\\iW[a\\ZW`\x03\x19\x83\x10\x15\x80a\\PW[a\\?W\x85\x83\x11\x15a[\xF6WPP\x90a[\xAE\x84a[\xB3\x93aR-V[a[\x11V[\x90\x81\x15a[\xF1Wa[\xC4\x92PaRgV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11aR:W\x90V[PP\x90V[\x95\x94\x92\x91\x90\x95a\\\x07W[PPPPV[\x83\x94\x95Pa[\xAE\x90a\\\x19\x93\x94aR-V[\x90\x81\x15a[\xF1Wa\\*\x92PaR-V[`\x01\x81\x01\x80\x91\x11aR:W\x90_\x80\x80\x80a\\\x01V[PP\x90PaS\xC7\x92\x91P\x19\x90aR-V[P\x82\x19\x84\x11a[\x92V[PP\x91\x90PaS\xC7\x92PaRgV[P\x82\x84\x11a[\x82V[P\x92PPP\x90V[P\x84\x82\x11\x15a[]V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R\xFD\xFEa\x01\x80`@R4a\0}Wa\0\x1Ba\0\x15a\0\xE2V[\x90a\x01\x98V[`@Qa-\x0B\x90\x81a\x0E\x06\x829`\x80Q\x81a\x1C\xDB\x01R`\xA0Q\x81a\x1D\x98\x01R`\xC0Q\x81a\x1C\xAC\x01R`\xE0Q\x81a\x1D*\x01Ra\x01\0Q\x81a\x1DP\x01Ra\x01 Q\x81a\rb\x01Ra\x01@Q\x81a\r\x8B\x01Ra\x01`Q\x81\x81\x81a\x0C@\x01Ra\x0C\x89\x01R\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\0\xB8W`@RV[a\0\x81V[`@Q\x90a\0\xCC`@\x83a\0\x95V[V[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0}WV[a;q\x90`@\x828\x03\x92\x83\x82Q\x94\x85\x92a\0\xFC\x82\x85a\0\x95V[\x839\x81\x01\x03\x12a\0}Wa\x01\x1B` a\x01\x14\x84a\0\xCEV[\x93\x01a\0\xCEV[\x90V[`@Q\x90a\x01-`@\x83a\0\x95V[`\x04\x82RV[`@Q\x90a\x01B`@\x83a\0\x95V[`\x01\x82RV[`@Q\x90a\x01W`@\x83a\0\x95V[`\t\x82RhSyndicate`\xB8\x1B` \x83\x01RV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90b\x9E4\0\x82\x01\x80\x92\x11a\x01\x93WV[a\x01oV[\x90a\x01\xA1a\x01HV[a\x01\xA9a\x01HV[\x90a\x01\xB2a\x01\x1EV[\x90c\x14\xD6S\x91`\xE2\x1B` \x83\x01Ra\x01\xC8a\x013V[`1`\xF8\x1B` \x82\x01\x90\x81R\x84Q\x90\x94\x91\x93\x91`\x01`\x01`@\x1B\x03\x82\x11a\0\xB8Wa\x01\xFD\x82a\x01\xF8`\x03Ta\x03^V[a\x03\x96V[` \x90`\x1F\x83\x11`\x01\x14a\x02\xD7W\x91\x80a\x021\x92a\x029\x95\x94_\x92a\x02\xCCW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[`\x03Ua\x045V[a\x02B\x81a\x06\x84V[a\x01 Ra\x02O\x82a\x07vV[a\x01@R` \x81Q\x91\x01 `\xE0RQ\x90 a\x01\0RF`\xA0Ra\x02pa\x08hV[`\x80R0`\xC0R`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x02\xBDW`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x02\xBDWa\x02\xB7a\0\xCC\x92a\x02\xA5Ba\x01\x83V[a\x01`Ra\x02\xB2_`\x0CUV[a\x05\x0EV[Pa\x05\x97V[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[\x01Q\x90P_\x80a\x02\x1DV[`\x03_R`\x1F\x19\x83\x16\x91\x90\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x92_[\x81\x81\x10a\x03FWP\x91`\x01\x93\x91\x85a\x029\x97\x96\x94\x10a\x03.W[PPP\x81\x1B\x01`\x03Ua\x045V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x03 V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x03\x06V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x03\x8CW[` \x83\x10\x14a\x03xWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x03mV[`\x1F\x81\x11a\x03\xA2WPPV[`\x03_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x03\xDCW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x03\xD1WPPV[_\x81U`\x01\x01a\x03\xC6V[\x90\x91P\x81\x90a\x03\xBDV[`\x1F\x82\x11a\x03\xF3WPPPV[_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x04+W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x04 WPPV[_\x81U`\x01\x01a\x04\x15V[\x90\x91P\x81\x90a\x04\x0CV[\x80Q\x90\x91\x90`\x01`\x01`@\x1B\x03\x81\x11a\0\xB8Wa\x04^\x81a\x04W`\x04Ta\x03^V[`\x04a\x03\xE6V[` \x92`\x1F\x82\x11`\x01\x14a\x04\x92Wa\x04\x8D\x92\x93\x82\x91_\x92a\x02\xCCWPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[`\x04UV[`\x04_R`\x1F\x19\x82\x16\x93\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x91_[\x86\x81\x10a\x04\xF6WP\x83`\x01\x95\x96\x10a\x04\xDEW[PPP\x81\x1B\x01`\x04UV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x04\xD3V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x04\xC0V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a;1_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x05\x92W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a;1_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x15a\x06qW`\x02Tk\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0\x81\x01\x80\x91\x11a\x01\x93W`\x02U`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x81\x90R`@\x90 k\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0\x81T\x01\x90U_\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`@Q\x80a\x06-k\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA3`\x02T`\x01`\x01`\xD0\x1B\x03\x90\x81\x81\x11a\x06\\WPPk\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0a\0\xCC\x91_a\t1V[c\x0EX\xAE\x93`\xE1\x1B_R`\x04R`$R`D_\xFD[c\xECD/\x05`\xE0\x1B_R_`\x04R`$_\xFD[\x90\x81Q` \x81\x10_\x14a\x06\x9CWP\x90a\x01\x1B\x90a\x08\xC6V[`\x01`\x01`@\x1B\x03\x81\x11a\0\xB8Wa\x06\xC0\x81a\x06\xB9`\x06Ta\x03^V[`\x06a\x03\xE6V[` \x92`\x1F\x82\x11`\x01\x14a\x06\xF7Wa\x06\xEF\x92\x93\x82\x91_\x92a\x02\xCCWPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[`\x06U`\xFF\x90V[`\x06_R`\x1F\x19\x82\x16\x93\x7F\xF6R\"#\x13\xE2\x84YR\x8D\x92\x0Be\x11\\\x16\xC0O>\xFC\x82\xAA\xED\xC9{\xE5\x9F?7|\r?\x91_[\x86\x81\x10a\x07^WP\x83`\x01\x95\x96\x10a\x07FW[PPP\x81\x1B\x01`\x06U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x078V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x07%V[\x90\x81Q` \x81\x10_\x14a\x07\x8EWP\x90a\x01\x1B\x90a\x08\xC6V[`\x01`\x01`@\x1B\x03\x81\x11a\0\xB8Wa\x07\xB2\x81a\x07\xAB`\x07Ta\x03^V[`\x07a\x03\xE6V[` \x92`\x1F\x82\x11`\x01\x14a\x07\xE9Wa\x07\xE1\x92\x93\x82\x91_\x92a\x02\xCCWPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[`\x07U`\xFF\x90V[`\x07_R`\x1F\x19\x82\x16\x93\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88\x91_[\x86\x81\x10a\x08PWP\x83`\x01\x95\x96\x10a\x088W[PPP\x81\x1B\x01`\x07U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x08*V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x08\x17V[`\xE0Qa\x01\0Q`@Q\x90` \x82\x01\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x84R`@\x83\x01R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x08\xC0`\xC0\x82a\0\x95V[Q\x90 \x90V[`\x1F\x81Q\x11a\x08\xF1W` \x81Q\x91\x01Q` \x82\x10a\x08\xE2W\x17\x90V[_\x19\x82` \x03`\x03\x1B\x1B\x16\x17\x90V[`D` \x91`@Q\x92\x83\x91c0Z'\xA9`\xE0\x1B\x83R\x81`\x04\x84\x01R\x80Q\x91\x82\x91\x82`$\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xFD[\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\t\x98W[a\0\xCC\x92`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\t\x80W[_\x90\x81R`\t` R`@\x80\x82 T\x92\x82R\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16a\x0BbV[a\t\x91a\t\x8C\x84a\n3V[a\ndV[PPa\tYV[a\t\xA1\x82a\n3V[\x92e\xFF\xFF\xFF\xFF\xFF\xFFC\x11a\n\x1BW`\x0BT\x80a\t\xE5WPa\t\xDBa\t\xCBa\0\xCC\x95_[`\x01a\r\xA9V[e\xFF\xFF\xFF\xFF\xFF\xFFC\x16`\x0Ba\x0C\xD3V[\x90PP\x92Pa\tDV[\x93\x84_\x19\x81\x01\x11a\x01\x93W`\x0B_R_Q` a;\x11_9_Q\x90_R\x90\x94\x01Ta\0\xCC\x94a\t\xDB\x91a\t\xCB\x91\x90`0\x1Ca\t\xC4V[c\x06\xDF\xCCe`\xE4\x1B_R`0`\x04RC`$R`D_\xFD[`\x01`\x01`\xD0\x1B\x03\x81\x11a\nMW`\x01`\x01`\xD0\x1B\x03\x16\x90V[c\x06\xDF\xCCe`\xE4\x1B_R`\xD0`\x04R`$R`D_\xFD[e\xFF\xFF\xFF\xFF\xFF\xFFC\x11a\n\x1BW`\x0BT\x80a\n\x8EWPa\t\xCBa\n\x8A\x91_[`\x02a\r\xA9V[\x90\x91V[\x80_\x19\x81\x01\x11a\x01\x93W`\x0B_R_Q` a;\x11_9_Q\x90_R\x01Ta\n\x8A\x91a\t\xCB\x91`0\x1Ca\n\x83V[e\xFF\xFF\xFF\xFF\xFF\xFFC\x11a\n\x1BW\x80T\x80a\n\xF0WPa\n\xE0a\n\x8A\x92_`\x02a\r\xA9V[\x90e\xFF\xFF\xFF\xFF\xFF\xFFC\x16\x90a\x0C\xD3V[\x80_\x19\x81\x01\x11a\x01\x93W_\x82\x81R` \x90 \x01_\x19\x01Ta\n\x8A\x92a\n\xE0\x91`0\x1Ca\n\x83V[e\xFF\xFF\xFF\xFF\xFF\xFFC\x11a\n\x1BW\x80T\x80a\x0B;WPa\n\xE0a\n\x8A\x92_`\x01a\r\xA9V[\x80_\x19\x81\x01\x11a\x01\x93W_\x82\x81R` \x90 \x01_\x19\x01Ta\n\x8A\x92a\n\xE0\x91`0\x1Ca\t\xC4V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x93\x92\x91\x90\x81\x16\x90\x81\x85\x14\x15\x80a\x0CUW[a\x0B\x8AW[PPPPPV[\x81a\x0B\xFBW[PP\x82a\x0B\x9FW[\x80\x80a\x0B\x83V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\n` R`@\x90 _Q` a;Q_9_Q\x90_R\x91a\x0B\xD8\x91a\x0B\xD2\x90\x91a\n3V[\x90a\x0B\x17V[`@\x80Q`\x01`\x01`\xD0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R\xA2_\x80\x80a\x0B\x98V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\n` R`@\x90 _Q` a;Q_9_Q\x90_R\x90a\x0C3\x90a\x0C-\x86a\n3V[\x90a\n\xBCV[`@\x80Q`\x01`\x01`\xD0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R\xA2_\x80a\x0B\x90V[P\x83\x15\x15a\x0B~V[_\x19\x81\x01\x91\x90\x82\x11a\x01\x93WV[\x90\x81Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\0\xB8W`\x01\x81\x01\x80\x84U\x81\x10\x15a\x0C\xBFW_\x92\x83R` \x92\x83\x90 \x82Q\x92\x90\x93\x01Q`0\x1Be\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x91\x01UV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80T\x92\x93\x92\x80\x15a\r\x7FWa\x0C\xEAa\x0C\xF5\x91a\x0C^V[\x82_R` _ \x01\x90V[\x80T`0\x81\x90\x1C\x93e\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92\x91\x81\x16\x80\x84\x11a\rpW\x87\x93\x03a\r<WPa\r8\x92P\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90`0\x1B\x16\x91\x16\x17\x90UV[\x91\x90V[\x91PPa\r8\x91a\r\\a\rNa\0\xBDV[e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[`\x01`\x01`\xD0\x1B\x03\x86\x16` \x83\x01Ra\x0ClV[c% `\x1D`\xE0\x1B_R`\x04_\xFD[P\x90a\r\xA4\x91a\r\x90a\rNa\0\xBDV[`\x01`\x01`\xD0\x1B\x03\x85\x16` \x83\x01Ra\x0ClV[_\x91\x90V[\x91\x90\x91\x80`\x01\x14a\r\xEBW`\x02\x14a\r\xCFWcNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[`\x01`\x01`\xD0\x1B\x03\x90\x81\x16\x91\x81\x16\x91\x90\x91\x03\x90\x81\x11a\x01\x93W\x90V[P`\x01`\x01`\xD0\x1B\x03\x91\x82\x16\x90\x82\x16\x01\x90\x81\x11a\x01\x93W\x90V\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\x02\xE5W\x80c\x06\xFD\xDE\x03\x14a\x02\xE0W\x80c\t^\xA7\xB3\x14a\x02\xDBW\x80c\x18\x16\r\xDD\x14a\x02,W\x80c#\xB8r\xDD\x14a\x02\xD6W\x80c$\x8A\x9C\xA3\x14a\x02\xD1W\x80c//\xF1]\x14a\x02\xCCW\x80c1<\xE5g\x14a\x02\xC7W\x80c6D\xE5\x15\x14a\x02\xC2W\x80c6V\x8A\xBE\x14a\x02\xBDW\x80c:F\xB1\xA8\x14a\x026W\x80c@\xC1\x0F\x19\x14a\x02\xB8W\x80cB\x96lh\x14a\x02\xB3W\x80cK\xF5\xD7\xE9\x14a\x02\xAEW\x80cO\x1B\xFC\x9E\x14a\x02\xA9W\x80cX|\xDE\x1E\x14a\x02\xA4W\x80c\\\x19\xA9\\\x14a\x02\x9FW\x80co\xCF\xFFE\x14a\x02\x9AW\x80cp\xA0\x821\x14a\x02\x95W\x80cy\xCCg\x90\x14a\x02\x90W\x80cz\x8C\xD1V\x14a\x02\x8BW\x80c~\xCE\xBE\0\x14a\x02\x86W\x80c\x83\xF1!\x1B\x14a\x02\x81W\x80c\x84&\xAD\xF2\x14a\x02|W\x80c\x84L\x90&\x14a\x02wW\x80c\x84\xB0\x19n\x14a\x02rW\x80c\x8AT%!\x14a\x02mW\x80c\x8D3C\xD6\x14a\x02hW\x80c\x8ES\x9E\x8C\x14a\x02cW\x80c\x90-U\xA5\x14a\x02^W\x80c\x91\xD1HT\x14a\x02YW\x80c\x91\xDD\xAD\xF4\x14a\x02TW\x80c\x95\xD8\x9BA\x14a\x02OW\x80c\x9A\xB2N\xB0\x14a\x021W\x80c\x9B~\xF6K\x14a\x02JW\x80c\xA2\x17\xFD\xDF\x14a\x02EW\x80c\xA9\x05\x9C\xBB\x14a\x02@W\x80c\xAA\x08*\x9D\x14a\x02;W\x80c\xB0\xCA%>\x14a\x026W\x80c\xBBMD6\x14a\x021W\x80c\xC0*\xE7T\x14a\x02,W\x80c\xC3\xCD\xA5 \x14a\x02'W\x80c\xD5\x05\xAC\xCF\x14a\x02\"W\x80c\xD5Gt\x1F\x14a\x02\x1DW\x80c\xDDb\xED>\x14a\x02\x18Wc\xF1\x12~\xD8\x14a\x02\x13W_\x80\xFD[a\x15\x15V[a\x14\xBCV[a\x14~V[a\x13$V[a\x11\xDDV[a\x056V[a\x11\x17V[a\x07\"V[a\x11\xA0V[a\x11zV[a\x11`V[a\x11:V[a\x10rV[a\x10GV[a\x0F\xF7V[a\x0F\xD1V[a\x0E\xF5V[a\x0E\xBBV[a\x0E\x81V[a\rJV[a\x0CcV[a\x0C)V[a\x0C\x05V[a\x0B\xCDV[a\x0B\xB3V[a\x0B\nV[a\n\xD5V[a\nZV[a\n8V[a\t\xF7V[a\t\xDAV[a\t1V[a\t\rV[a\x084V[a\x06\xC5V[a\x06\xABV[a\x06\x90V[a\x06KV[a\x06\x18V[a\x05SV[a\x05\x05V[a\x03\xE1V[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W`\x045\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x80\x91\x03a\x03\x86W\x80\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x92\x14\x90\x81\x15a\x03\\W[P`@Q\x90\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14_a\x03QV[_\x80\xFD[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` a\x03\xDE\x92\x81\x81R\x01\x90a\x03\x8AV[\x90V[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W`@Q_`\x03Ta\x04\x01\x81a\x16\x0BV[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x04\x97WP`\x01\x14a\x049W[a\x045\x83a\x04)\x81\x85\x03\x82a\x17JV[`@Q\x91\x82\x91\x82a\x03\xCDV[\x03\x90\xF3[`\x03_\x90\x81R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x93\x92P\x90[\x80\x82\x10a\x04}WP\x90\x91P\x81\x01` \x01a\x04)a\x04\x19V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x04eV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x04)\x90Pa\x04\x19V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03\x86WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03\x86WV[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x05+a\x05!a\x04\xD9V[`$5\x903a \x9EV[` `@Q`\x01\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `\x02T`@Q\x90\x81R\xF3[4a\x03\x86W```\x03\x196\x01\x12a\x03\x86Wa\x05la\x04\xD9V[a\x05ta\x04\xEFV[`D5\x90`\x01`\x01`\xA0\x1B\x03\x83\x16_R`\x01` Ra\x05\xA73`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x92_\x19\x84\x10a\x05\xC8W[a\x05\xBC\x93Pa\x18NV[`@Q`\x01\x81R` \x90\xF3[\x82\x84\x10a\x05\xE4Wa\x05\xDF\x83a\x05\xBC\x95\x033\x83a!lV[a\x05\xB2V[\x82\x84\x7F\xFB\x8FA\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`DR`d_\xFD[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W` a\x06C`\x045_R`\x05` R`\x01`@_ \x01T\x90V[`@Q\x90\x81R\xF3[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x06\x8E`\x045a\x06ja\x04\xEFV[\x90a\x06\x89a\x06\x84\x82_R`\x05` R`\x01`@_ \x01T\x90V[a\x1B\x89V[a\x1B\xEAV[\0[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Q`\x12\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` a\x06Ca\x1C\xA2V[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86W`\x045a\x06\xE1a\x04\xEFV[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\x06\xFAWa\x06\x8E\x91a\x1D\xBEV[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x07;a\x04\xD9V[`\x01`\x01`\xA0\x1B\x03`$5\x91\x16_R`\n` Ra\x07\\`@_ \x91a\x1EnV[\x81T\x90_\x82\x91`\x05\x84\x11a\x07\xDCW[a\x07v\x93P\x84a$\x87V[\x80a\x07\xA5WPP` y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_[\x16`@Q\x90\x81R\xF3[` \x91a\x07\xCCy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a\x17\xAAV[\x90_R\x82_ \x01T`0\x1Ca\x07\x9CV[\x91\x92a\x07\xE7\x81a#\x12V[\x81\x03\x90\x81\x11a\x08/Wa\x07v\x93\x85_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x08\x1DWP\x91a\x07kV[\x92\x91Pa\x08)\x90a\x17\x8BV[\x90a\x07kV[a\x15\xDEV[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x08Ma\x04\xD9V[`$5a\x08Xa\x1A\x11V[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x08\xE5W\x80\x15a\x08\xBDW`\x02T\x81\x81\x01\x80\x91\x11a\x08/Wk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x10a\x08\x95Wa\x06\x8E\x91a!\xB3V[\x7F\x17~?\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W`\x045\x80\x15a\x08\xBDWa\x06\x8E\x903a\x1E\xC0V[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86Wa\tJCa\"\x93V[e\xFF\xFF\xFF\xFF\xFF\xFF\x80a\t[Ca\"\x93V[\x16\x91\x16\x03a\t\xB2Wa\x045`@Qa\tt`@\x82a\x17JV[`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x03\x8AV[\x7Fo\xF0q@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Qb\x9E4\0\x81R\xF3[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W`\x01`\x01`\xA0\x1B\x03a\n\x18a\x04\xD9V[\x16_R`\t` R` `\x01`\x01`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86Wa\x06\x8Ea\nTa\x04\xD9V[3a\x1F\x8FV[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W`\x01`\x01`\xA0\x1B\x03a\n{a\x04\xD9V[\x16_R`\n` R`@_ Tc\xFF\xFF\xFF\xFF\x81\x11a\n\xA5W`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x90\xF3[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R` `\x04R`$R`D_\xFD[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W` a\x06Ca\n\xF3a\x04\xD9V[`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ T\x90V[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x0B#a\x04\xD9V[`$5\x90a\x0B/a\x1A\x99V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a\x08\xE5W\x82\x15a\x08\xBDWa\x0BNa\x17\xE4V[\x15a\x0B\x8BW\x82a\x0B]\x91a\x1E\xC0V[`@Q\x91\x82R\x7F\xBE\xF4\xF8\x1C\x18\x14\xC6A\xED\xE8^\xBA\xAC\xF1\x9D\x04\x8B,[U\x98\n\xDF\xA6\xEF\x0F\x95le\x135\xA2` 3\x93\xA3\0[\x7F\xB8\xB5\xCA-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` a\x06Ca\x17\xB8V[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W`\x01`\x01`\xA0\x1B\x03a\x0B\xEEa\x04\xD9V[\x16_R`\x08` R` `@_ T`@Q\x90\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` a\x0C\x1Fa\x17\xE4V[`@Q\x90\x15\x15\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xF3[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W`\x045a\x0C\x7Fa\x1B!V[B\x81\x11\x15a\r\"W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11a\x0C\xFAW\x7F\xDDh\x96\xDC\xF1\xD4\xB3\x11\xCC\xA8}\xD1\x9B\xBB\xA2\xEA\x9C\xE2\xF8g\xC1V\x88x\xA0C\x8Af\xA1\xAF\xEE\xECa\x0C\xF5`\x0CT\x92\x80`\x0CU`@Q\x91\x82\x913\x95\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x03\x90\xA2\0[\x7F\xEFi\xAFe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xA5e\x83S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86Wa\x0E(a\r\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a&>V[a\r\xAF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a&\xB7V[` `@Qa\r\xBE\x82\x82a\x17JV[_\x81R\x81a\x0E6\x81\x83\x01\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x83\x016\x877`@Q\x97\x88\x97\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R`\xE0\x85\x8A\x01R`\xE0\x89\x01\x90a\x03\x8AV[\x90\x87\x82\x03`@\x89\x01Ra\x03\x8AV[\x91F``\x87\x01R0`\x80\x87\x01R_`\xA0\x87\x01R\x85\x83\x03`\xC0\x87\x01RQ\x91\x82\x81R\x01\x92\x91_[\x82\x81\x10a\x0EjWPPPP\x03\x90\xF3[\x83Q\x85R\x86\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x0E[V[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Q\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Q\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x81R\xF3[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86Wa\x0F\x11`\x045a\x1EnV[`\x0BT\x90_\x82\x91`\x05\x84\x11a\x0F}W[a\x0F-\x93P`\x0Ba$\x87V[\x80a\x0F[WP` _[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[a\x0Fxa\x0Fi` \x92a\x17\xAAV[`\x0B_R\x82_ \x01T`0\x1C\x90V[a\x0F7V[\x91\x92a\x0F\x88\x81a#\x12V[\x81\x03\x90\x81\x11a\x08/Wa\x0F-\x93`\x0B_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x0F\xBFWP\x91a\x0F!V[\x92\x91Pa\x0F\xCB\x90a\x17\x8BV[\x90a\x0F!V[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Qk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81R\xF3[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86W` `\xFFa\x10;`\x045a\x10\x1Aa\x04\xEFV[\x90_R`\x05\x84R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` a\x10bCa\"\x93V[e\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W`@Q_`\x04Ta\x10\x92\x81a\x16\x0BV[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x04\x97WP`\x01\x14a\x10\xB9Wa\x045\x83a\x04)\x81\x85\x03\x82a\x17JV[`\x04_\x90\x81R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x93\x92P\x90[\x80\x82\x10a\x10\xFDWP\x90\x91P\x81\x01` \x01a\x04)a\x04\x19V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x10\xE5V[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W` a\x06Ca\x115a\x04\xD9V[a\x17\xFBV[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Qk\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Q_\x81R\xF3[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x05+a\x11\x96a\x04\xD9V[`$5\x903a\x18NV[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `\x0CT`@Q\x90\x81R\xF3[`d5\x90`\xFF\x82\x16\x82\x03a\x03\x86WV[`\x845\x90`\xFF\x82\x16\x82\x03a\x03\x86WV[4a\x03\x86W`\xC0`\x03\x196\x01\x12a\x03\x86Wa\x11\xF6a\x04\xD9V[`$5\x90`D5a\x12\x05a\x11\xBDV[`\x845\x90`\xA45\x92\x80B\x11a\x12\xF9W\x91a\x12\x8B\x93\x91a\x12}a\x12\x82\x94`@Q` \x81\x01\x91\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x83R`\x01`\x01`\xA0\x1B\x03\x8A\x16`@\x83\x01R\x8A``\x83\x01R`\x80\x82\x01R`\x80\x81Ra\x12u`\xA0\x82a\x17JV[Q\x90 a NV[a&\xEEV[\x90\x92\x91\x92a'\xB2V[a\x12\xAF\x81`\x01`\x01`\xA0\x1B\x03\x16_R`\x08` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[\x80\x93\x03a\x12\xC0Wa\x06\x8E\x92Pa\x1F\x8FV[`\x01`\x01`\xA0\x1B\x03\x91P\x7Fu-\x88\xC0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$R`D_\xFD[\x7FF\x83\xAF\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x03\x86W`\xE0`\x03\x196\x01\x12a\x03\x86Wa\x13=a\x04\xD9V[a\x13Ea\x04\xEFV[`D5\x90`d5\x92a\x13Ua\x11\xCDV[`\xA45`\xC45\x90\x86B\x11a\x14RWa\x13\xFE\x92a\x13\xF9a\x13\x8E\x86`\x01`\x01`\xA0\x1B\x03\x16_R`\x08` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[\x98`@Q` \x81\x01\x91\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R`\x01`\x01`\xA0\x1B\x03\x89\x16\x9B\x8C`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x8B\x16``\x84\x01R\x8B`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\x12u`\xE0\x82a\x17JV[a \x8FV[\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x03a\x14\x18Wa\x06\x8E\x93Pa \x9EV[\x7FK\x80\x0EF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x04R\x16`$R`D_\xFD[\x86\x7Fby\x13\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x06\x8E`\x045a\x14\x9Da\x04\xEFV[\x90a\x14\xB7a\x06\x84\x82_R`\x05` R`\x01`@_ \x01T\x90V[a\x1D\xBEV[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86W` a\x15\x0Ca\x14\xDAa\x04\xD9V[`\x01`\x01`\xA0\x1B\x03a\x14\xEAa\x04\xEFV[\x91\x16_R`\x01\x83R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x15.a\x04\xD9V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03\x86Wa\x045\x91`\x01`\x01`\xA0\x1B\x03a\x15{\x92a\x15Wa\x186V[Pa\x15`a\x186V[P\x16_R`\n` R`@_ a\x15ua\x186V[Pa(yV[P`@Q\x90a\x15\x89\x82a\x17)V[Te\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82R`0\x1C` \x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91` y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`@\x84\x01\x95e\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x01Q\x16\x91\x01RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x16RW[` \x83\x10\x14a\x16%WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x16\x1AV[_\x92\x91\x81T\x91a\x16k\x83a\x16\x0BV[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a\x16\xC0WP`\x01\x14a\x16\x87WPPPV[_\x90\x81R` \x81 \x93\x94P\x91\x92[\x83\x83\x10a\x16\xA6WP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a\x16\x95V[\x90P` \x94\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x92\x91\x92\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x17EW`@RV[a\x16\xFCV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x17EW`@RV[\x90`\x01\x82\x01\x80\x92\x11a\x08/WV[`@Q\x90a\x17\xA8`@\x83a\x17JV[V[\x90_\x19\x82\x01\x91\x82\x11a\x08/WV[`\x0CT\x80\x15\x80\x15a\x17\xDAW[a\x17\xD5WB\x81\x03\x90\x81\x11a\x08/W\x90V[P_\x90V[P\x80B\x10\x15a\x17\xC4V[`\x0CT\x80\x15\x15\x90\x81a\x17\xF4WP\x90V[\x90PB\x10\x90V[`\x01`\x01`\xA0\x1B\x03\x16_R`\n` Ry\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x182`@_ a %V[\x16\x90V[`@Q\x90a\x18C\x82a\x17)V[_` \x83\x82\x81R\x01RV[\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x93\x84\x15a\x19\xE5W`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x15a\x19\xB9Wa\x18za\x17\xE4V[\x80a\x19\x81W[a\x19YWa\x18\x9E\x82`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[T\x95\x84\x87\x10a\x19\x1AW\x84a\x17\xA8\x96\x97\x03a\x18\xC8\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[Ua\x18\xE3\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[\x80T\x86\x01\x90U`@Q\x85\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3a*PV[\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04R`$\x87\x90R`D\x85\x90R`d_\xFD[\x7F\xDB\x89\xE3\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P3_\x90\x81R\x7Ft\x07\x96\xD8~O\x86\xCC\x94g\x17h\xC7D\x95`E\xFE\x85P\x93)\x1Ey\x19L\x96\xACG\x80@\xAA` R`@\x90 T`\xFF\x16\x15a\x18\x80V[\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[3_\x90\x81R\x7F\x9Ak\xF4\x8B\xB8@\xE7\x8F\xE8\xE7\xAF\xD1\r=9\x1A\x91s\x8A\x9Ee$\xF6\xFD\xFA\x1A:\xBA\x9D\xC0?\xB1` R`@\x90 T`\xFF\x16\x15a\x1AIWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB`$R`D_\xFD[3_\x90\x81R\x7Ft\x07\x96\xD8~O\x86\xCC\x94g\x17h\xC7D\x95`E\xFE\x85P\x93)\x1Ey\x19L\x96\xACG\x80@\xAA` R`@\x90 T`\xFF\x16\x15a\x1A\xD1WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t`$R`D_\xFD[3_\x90\x81R\x7F\x05\xB8\xCC\xBB\x9DM\x8F\xB1n\xA7L\xE3\xC2\x9AA\xF1\xB4a\xFB\xDA\xFFG\x14\xA0\xD9\xA8\xEB\x05I\x97F\xBC` R`@\x90 T`\xFF\x16\x15a\x1BYWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[\x80_R`\x05` R`\xFFa\x1B\xB13`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16\x15a\x1B\xBBWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R`\x05` R`\xFFa\x1C\x12\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16a\x1C\x9CW\x80_R`\x05` Ra\x1C>\x82`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90U`\x01`\x01`\xA0\x1B\x033\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x14\x80a\x1D\x95W[\x15a\x1C\xFDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x1D\x8F`\xC0\x82a\x17JV[Q\x90 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x1C\xD4V[\x80_R`\x05` R`\xFFa\x1D\xE6\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16\x15a\x1C\x9CW\x80_R`\x05` Ra\x1E\x13\x82`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90U`\x01`\x01`\xA0\x1B\x033\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V[e\xFF\xFF\xFF\xFF\xFF\xFFa\x1E~Ca\"\x93V[\x16\x80\x82\x10\x15a\x1E\x91WPa\x03\xDE\x90a\"\x93V[\x90\x7F\xEC\xD3\xF8\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x15a\x19\xE5Wa\x1E\xEA\x82`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[T\x83\x81\x10a\x1FRW\x91_\x80\x92\x85a\x17\xA8\x96\x95\x03a\x1F\x17\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[U`\x02\x80T\x86\x90\x03\x90U`@Q\x85\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3a*PV[\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04R`$R`D\x83\x90R`d_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\t` R`@\x81 \x80T\x86\x85\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x81\x17\x90\x92Ua\x17\xA8\x96\x94\x16\x94a \x1F\x93\x90\x92\x86\x91\x90\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x90\x80\xA4`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ T\x90V[\x91a$\xEBV[\x80T\x80a 2WPP_\x90V[\x80_\x19\x81\x01\x11a\x08/W_\x19\x91_R` _ \x01\x01T`0\x1C\x90V[`B\x90a Ya\x1C\xA2V[\x90`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[\x91a\x03\xDE\x93\x91a\x12\x82\x93a&\xEEV[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a!@W`\x01`\x01`\xA0\x1B\x03\x81\x16\x92\x83\x15a!\x14W\x80a!\x07\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93\x85_R`\x01` R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[U`@Q\x90\x81R` \x90\xA3V[\x7F\x94(\rb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\xE6\x02\xDF\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a!@W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a!\x14Wa!\xB0\x91_R`\x01` R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[UV[\x91\x90`\x02T\x81\x81\x01\x80\x91\x11a\x08/W`\x02U`\x01`\x01`\xA0\x1B\x03\x83\x16\x80a\"nW\x81`\x02T\x03`\x02U[`@Q\x82\x81R_\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3`\x02T\x92y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x11a\">WPa\x17\xA8\x92\x93P_a*PV[\x7F\x1C\xB1]&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x85\x90R`$R`D_\xFD[a\"\x88\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[\x82\x81T\x01\x90Ua!\xDDV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\"\xABWe\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`0`\x04R`$R`D_\xFD[\x81\x15a\"\xE5W\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[`\x01\x81\x11\x15a\x03\xDEW\x80`\x01p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x10\x15a$EW[a#\xEBa#\xE1a#\xD7a#\xCDa#\xC3a#\xB9a#\xA8a#\xF2\x97`\x04\x8Ah\x01\0\0\0\0\0\0\0\0a#\xF7\x9C\x10\x15a$8W[d\x01\0\0\0\0\x81\x10\x15a$+W[b\x01\0\0\x81\x10\x15a$\x1EW[a\x01\0\x81\x10\x15a$\x11W[`\x10\x81\x10\x15a$\x04W[\x10\x15a#\xFCW[`\x03\x02`\x01\x1C\x90V[a#\xB2\x81\x8Ba\"\xDBV[\x01`\x01\x1C\x90V[a#\xB2\x81\x8Aa\"\xDBV[a#\xB2\x81\x89a\"\xDBV[a#\xB2\x81\x88a\"\xDBV[a#\xB2\x81\x87a\"\xDBV[a#\xB2\x81\x86a\"\xDBV[\x80\x93a\"\xDBV[\x82\x11\x90V[\x90\x03\x90V[`\x01\x1Ba#\x9FV[`\x04\x1C\x91`\x02\x1B\x91a#\x98V[`\x08\x1C\x91`\x04\x1B\x91a#\x8EV[`\x10\x1C\x91`\x08\x1B\x91a#\x83V[` \x1C\x91`\x10\x1B\x91a#wV[`@\x1C\x91` \x1B\x91a#iV[PPa#\xF7a#\xF2a#\xEBa#\xE1a#\xD7a#\xCDa#\xC3a#\xB9a#\xA8a$l\x8A`\x80\x1C\x90V[\x98Ph\x01\0\0\0\0\0\0\0\0\x97Pa#8\x96PPPPPPPV[\x91\x90[\x83\x82\x10a$\x97WPPP\x90V[\x90\x91\x92\x80\x83\x16\x90\x80\x84\x18`\x01\x1C\x82\x01\x80\x92\x11a\x08/W\x84_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x10_\x14a$\xD9WP\x92[\x91\x90a$\x8AV[\x93\x92Pa$\xE5\x90a\x17\x8BV[\x91a$\xD2V[\x91\x90`\x01`\x01`\xA0\x1B\x03\x81\x16\x92`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x84\x82\x14\x15\x80a&5W[a%\x19W[PPPPPV[\x81a%\xBFW[PP\x82a%.W[\x80\x80a%\x12V[a%\xB4a%\x9B\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x93a%\x95a%\x8Fy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95`\x01`\x01`\xA0\x1B\x03\x16_R`\n` R`@_ \x90V[\x91a(\xBBV[\x90a)\x8FV[`@\x80Q\x92\x85\x16\x83R\x93\x16` \x82\x01R\x91\x82\x91\x90\x82\x01\x90V[\x03\x90\xA2_\x80\x80a%'V[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa&+a%\x9Ba&\x1C\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x94`\x01`\x01`\xA0\x1B\x03\x16_R`\n` R`@_ \x90V[a&%\x88a(\xBBV[\x90a)+V[\x03\x90\xA2_\x80a%\x1FV[P\x83\x15\x15a%\rV[`\xFF\x81\x14a&\x9DW`\xFF\x81\x16\x90`\x1F\x82\x11a&uW`@Q\x91a&b`@\x84a\x17JV[` \x80\x84R\x83\x81\x01\x91\x906\x837\x83RR\x90V[\x7F\xB3Q+\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P`@Qa\x03\xDE\x81a&\xB0\x81`\x06a\x16\\V[\x03\x82a\x17JV[`\xFF\x81\x14a&\xDBW`\xFF\x81\x16\x90`\x1F\x82\x11a&uW`@Q\x91a&b`@\x84a\x17JV[P`@Qa\x03\xDE\x81a&\xB0\x81`\x07a\x16\\V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a'pW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a'eW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a'[W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[`@Q=_\x82>=\x90\xFD[PPP_\x91`\x03\x91\x90V[`\x04\x11\x15a'\x85WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[a'\xBB\x81a'{V[\x80a'\xC4WPPV[a'\xCD\x81a'{V[`\x01\x81\x03a'\xFDW\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[a(\x06\x81a'{V[`\x02\x81\x03a(:WP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80a(F`\x03\x92a'{V[\x14a(NWPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80T\x82\x10\x15a(\x8EW_R` _ \x01\x90_\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a(\xFBWy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\xD0`\x04R`$R`D_\xFD[\x90a)5Ca\"\x93V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a)[\x85a %V[\x92\x16\x91\x16\x03\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x08/Wa)\x8B\x92a+\xD8V[\x90\x91V[\x90a)\x99Ca\"\x93V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a)\xBF\x85a %V[\x92\x16\x91\x16\x01\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x08/Wa)\x8B\x92a+\xD8V[a)\xF8Ca\"\x93V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a*\x1F`\x0Ba %V[\x92\x16\x91\x16\x03y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08/Wa)\x8B\x91`\x0Ba+\xD8V[\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a*\xC1W[`\x01`\x01`\xA0\x1B\x03a\x17\xA8\x93\x16\x90\x81\x15a*\xA9W[_R`\t` R`\x01`\x01`\xA0\x1B\x03`@_ T\x16\x90_R`\t` R`\x01`\x01`\xA0\x1B\x03`@_ T\x16\x90a$\xEBV[a*\xBAa*\xB5\x84a(\xBBV[a)\xEFV[PPa*xV[a*\xCA\x82a(\xBBV[\x92a*\xD4Ca\"\x93V[\x93y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a*\xFB`\x0Ba %V[\x92\x16\x91\x16\x01\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x08/Wa\x17\xA8\x94`\x01`\x01`\xA0\x1B\x03\x92a+:\x91`\x0Ba+\xD8V[\x90PP\x93PPa*cV[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x17EWa+g\x91`\x01\x82\x01\x81Ua(yV[a+\xACW\x81Q` \x92\x90\x92\x01Q`0\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x80T\x92\x93\x92\x80\x15a,\xCEWa+\xEFa+\xFA\x91a\x17\xAAV[\x82_R` _ \x01\x90V[\x80T`0\x81\x90\x1C\x93e\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92\x91\x81\x16\x80\x84\x11a,\xA6W\x87\x93\x03a,_WPa,[\x92P\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x83T\x92`0\x1B\x16\x91\x16\x17\x90UV[\x91\x90V[\x91PPa,[\x91a,\x7Fa,qa\x17\x99V[e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16` \x83\x01Ra+EV[\x7F% `\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P\x90a-\x06\x91a,\xDFa,qa\x17\x99V[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16` \x83\x01Ra+EV[_\x91\x90V\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB8\x05\xB8\xCC\xBB\x9DM\x8F\xB1n\xA7L\xE3\xC2\x9AA\xF1\xB4a\xFB\xDA\xFFG\x14\xA0\xD9\xA8\xEB\x05I\x97F\xBC\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$`\xA04a\0\xD9W`\x1Fa\x13B8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xDDW\x80\x84\x92``\x94`@R\x839\x81\x01\x03\x12a\0\xD9Wa\0G\x81a\0\xF1V[a\0_`@a\0X` \x85\x01a\0\xF1V[\x93\x01a\0\xF1V[\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\0\xCAW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\0\xCAW`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\0\xCAWa\0\xA3\x92a\0\x9D\x91`\x80Ra\x01\x05V[Pa\x01{V[P`@Qa\x10\xD3\x90\x81a\x02\x0F\x829`\x80Q\x81\x81\x81a\x03!\x01R\x81\x81a\x07\xF1\x01Ra\rT\x01R\xF3[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0\xD9WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x13\"_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x01vW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x13\"_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a\x12\xE2_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x13\x02_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x01vW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x13\x02_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\xF2\x7F\x93h\xAA\x1Bv\x97\xE5\x04)\xF87\"\xB8\x8F.]\xB8\x18M^,\xBC\xF0`\xF41\x0B\xBD>}\x90_Q` a\x12\xE2_9_Q\x90_R\x90\x80\xA4`\x01\x90V\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x01\xFF\xC9\xA7\x14a\t\xFAWP\x80c\x15\x8E\xF9>\x14a\t\xD8W\x80c$\x8A\x9C\xA3\x14a\t\xAEW\x80c//\xF1]\x14a\tqW\x80c6V\x8A\xBE\x14a\t\x05W\x80cCX\x10\x10\x14a\x08/W\x80cC\xA3\xF8\xA1\x14a\x08\x15W\x80c[\xDFl\xA1\x14a\x07\xC5W\x80c_\x15\xC3\xC9\x14a\x07\xAAW\x80cvg\x18\x08\x14a\x07\x8DW\x80c\x89\x16$\x86\x14a\x07SW\x80c\x91\xD1HT\x14a\x06\xFDW\x80c\xA0\x88x}\x14a\x06\xBAW\x80c\xA2\x17\xFD\xDF\x14a\x06\xA0W\x80c\xAC\x12\xCE\x07\x14a\x06\x83W\x80c\xB1\x98\xD0(\x14a\x06^W\x80c\xC6:\tD\x14a\x05PW\x80c\xD3\xF5f\xAE\x14a\x02VW\x80c\xD5Gt\x1F\x14a\x02\x0FW\x80c\xDE\xBEO\x1F\x14a\x01\xD4W\x80c\xDF\x02D\xB1\x14a\x01\xB6W\x80c\xE0\xE6\x16\x9C\x14a\x01\x9BW\x80c\xE4\xB7\xFBs\x14a\x01xW\x80c\xEC\xEDU&\x14a\x01UWc\xFA9\x1Cd\x14a\x011W_\x80\xFD[4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `0`\x02T\x10\x15`@Q\x90\x81R\xF3[\x80\xFD[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `@Qg\r\xE0\xB6\xB3\xA7d\0\0\x81R\xF3[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` a\x01\x93a\r,V[`@Q\x90\x81R\xF3[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` a\x01\x93a\x0C\xEFV[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `\x03T`@Q\x90\x81R\xF3[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `@Q\x7F\x07n\xB8\xB8u\xB6\xEA\x83\x9B\x08|L\x0C\x1AFa\xB0\x89\xD3\xB6\xEE,\x1E\xF1\xB9\xCF\xA7\xFE\x10f\xD2\x06\x81R\xF3[P4a\x01RW`@`\x03\x196\x01\x12a\x01RWa\x02R`\x045a\x02/a\n\x98V[\x90a\x02Ma\x02H\x82_R_` R`\x01`@_ \x01T\x90V[a\x0E\xD3V[a\x10\x0BV[P\x80\xF3[P4a\x04QW`@`\x03\x196\x01\x12a\x04QW`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x04QW3_\x90\x81R\x7F\x0E%9\x0F\xF9SSX\xA5\xE9\x16\xDF\xE7\xD3\x82f\xC86\x01\xAFn\x11!\x05\xB2-\xF4\xA9\x0B\xF8\x91\x01` R`@\x90 T`$5\x90`\xFF\x16\x15a\x05\0W`\xFF`\x04T\x16\x15a\x04\x82W`\x02T\x90`0\x82\x10\x15a\x04\x82W\x83\x15a\x04\xD8W\x80\x82\x03a\x04\xAAWPPa\x02\xF3a\x0B\x05V[\x90\x81\x15a\x04\x82W`\x03T\x82\x81\x01\x80\x91\x11a\x04UW`\x03Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x04QW_\x80\x91`D`@Q\x80\x94\x81\x93\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x89`\x04\x84\x01R\x88`$\x84\x01RZ\xF1\x80\x15a\x04FWa\x043W[P`\x02T\x92\x83\x7F\x16\x0F\xC1\x95\xD6\xE56\x91\xD3\r\x80L\xE1\x90\xDC\tG\x18\x91g~CC;\x91\xA7\xA6\x13\x1C\x12\xA5\x9A`@a\x03\xC1a\r,V[\x81Q\x90\x87\x82R` \x82\x01R\xA3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14a\x04\x06WP`\x01` \x92\x01`\x02U`@Q\x90\x81R\xF3[\x80\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x92R`\x11`\x04R\xFD[a\x04?\x91P_\x90a\x0C\x81V[__a\x03\x90V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7F\x9E\x91\xC9\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F0A:\x1A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x07n\xB8\xB8u\xB6\xEA\x83\x9B\x08|L\x0C\x1AFa\xB0\x89\xD3\xB6\xEE,\x1E\xF1\xB9\xCF\xA7\xFE\x10f\xD2\x06`$R`D_\xFD[4a\x04QW` `\x03\x196\x01\x12a\x04QW3_\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\x045\x90`\xFF\x16\x15a\x06.W`\x04T`\xFF\x81\x16a\x04\x82W\x81\x15a\x06\x06W\x7F\xC1,`\xAB\xC2\x16(n\xF2^4\xB1\x80Z\x0C=\xDAs\xE4\xC2\xFDl\xF3`\xE8\x07\xA7\xA9\xE71g9\x91`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0` \x93\x16\x17`\x04U\x80`\x01U`@Q\x90\x81R\xA1\0[\x7F\xEBv\x99 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@QjB,\xA8\xB0\xA0\nBP\0\0\0\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `\x01T`@Q\x90\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Q_\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW`\xA0`\x02T`0`\x03T\x91a\x06\xDDa\r,V[`@Q\x93\x82\x85R\x83` \x86\x01R`@\x85\x01R``\x84\x01R\x10\x15`\x80\x82\x01R\xF3[4a\x04QW`@`\x03\x196\x01\x12a\x04QWa\x07\x16a\n\x98V[`\x045_R_` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Q\x7F\xF2\x7F\x93h\xAA\x1Bv\x97\xE5\x04)\xF87\"\xB8\x8F.]\xB8\x18M^,\xBC\xF0`\xF41\x0B\xBD>}\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `\x02T`@Q\x90\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Q`0\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` a\x01\x93a\x0B\x05V[4a\x04QW` `\x03\x196\x01\x12a\x04QW3_\x90\x81R\x7F~\x972wWE-\x9E3\x15\xE3\x95\xBC\x10\x0E}\xA1\xF2\xD3Q\x06\xFF\xFD8\xF28\x07tp\x90\xEF\xA9` R`@\x90 T`\x045\x90`\xFF\x16\x15a\x08\xB5W\x80\x15a\x06\x06W\x80`\x01U`\x02T`@Q\x91\x82R\x7F\xB8\x13\xFF\xBE8}l\xF6\xE6\xA6\xF6\xC5\xF8\x90_vj\x0F\x1Cl\xD0\x1Cg1/p\x93V\xC6%\x97\xBD` 3\x93\xA3\0[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\xF2\x7F\x93h\xAA\x1Bv\x97\xE5\x04)\xF87\"\xB8\x8F.]\xB8\x18M^,\xBC\xF0`\xF41\x0B\xBD>}`$R`D_\xFD[4a\x04QW`@`\x03\x196\x01\x12a\x04QWa\t\x1Ea\n\x98V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\tIWa\tG\x90`\x045a\x10\x0BV[\0[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04QW`@`\x03\x196\x01\x12a\x04QWa\tG`\x045a\t\x90a\n\x98V[\x90a\t\xA9a\x02H\x82_R_` R`\x01`@_ \x01T\x90V[a\x0F9V[4a\x04QW` `\x03\x196\x01\x12a\x04QW` a\x01\x93`\x045_R_` R`\x01`@_ \x01T\x90V[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `\xFF`\x04T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04QW` `\x03\x196\x01\x12a\x04QW`\x045\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x80\x92\x03a\x04QW\x81\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x93\x14\x90\x81\x15a\nnW[P\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x83a\ngV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x04QWV[\x81\x15a\n\xC5W\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x04UWV[`\xFF`\x04T\x16\x15\x80\x15a\x0CtW[a\x0CpW`\x02T`0\x03`0\x81\x11a\x04UWa\x0B-a\r,V[`\x01\x82\x14a\x0CkW`\x01T\x91g\r\xE0\xB6\xB3\xA7d\0\0\x83\x14a\x0C`WPa\x0BQa\x0C\xEFV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x0C\x19W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2\x1FILX\x9C\0\0\x81\x01\x90\x81\x11a\x04UW[a\x03\xE8\x81\x10\x15a\x0C\x13WPa\x03\xE8\x90[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15a\x0B\xEBW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2\x1FILX\x9C\0\0\x83\x01\x92\x83\x11a\x04UWa\x0B\xE8\x92a\x0B\xE3\x91a\n\xF2V[a\n\xBBV[\x90V[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x03\x91g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11a\x04UWa\x0B\xE8\x92a\x0B\xE3\x91a\n\xF2V[\x90a\x0B\x9CV[g\r\xE0\xB6\xB3\xA7d\0\0\x03g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x0B\x8CW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x90a\x0B\xE8\x92Pa\n\xBBV[\x90P\x90V[_\x90V[P`0`\x02T\x10\x15a\x0B\x13V[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\xC2W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\r\xE0\xB6\xB3\xA7d\0\0`\x02T`\x01T[`0\x82\x10a\r\x0CWPP\x90V[\x90\x91g\r\xE0\xB6\xB3\xA7d\0\0a\r#\x83`\x01\x93a\n\xF2V[\x04\x92\x01\x90a\x0C\xFFV[`@Q\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\x04FW_\x92a\x0E\x9EW[P` `\x04\x91`@Q\x92\x83\x80\x92\x7F\x90-U\xA5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04FW_\x91a\x0ElW[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xBD\xD3WO_\xF5\xBD\xB0\0\0\0\x81\x01\x81\x81\x11a\x04UW\x82\x11\x15a\x0EdW\x81\x03jB,\xA8\xB0\xA0\nBP\0\0\0\x01\x90\x81\x11a\x04UW[\x80jB,\xA8\xB0\xA0\nBP\0\0\0\x11_\x14a\x0E_WjB,\xA8\xB0\xA0\nBP\0\0\0\x03jB,\xA8\xB0\xA0\nBP\0\0\0\x81\x11a\x04UW\x90V[P_\x90V[PP_a\x0E)V[\x90P` \x81=` \x11a\x0E\x96W[\x81a\x0E\x87` \x93\x83a\x0C\x81V[\x81\x01\x03\x12a\x04QWQ_a\r\xE0V[=\x91Pa\x0EzV[\x90\x91P` \x81=` \x11a\x0E\xCBW[\x81a\x0E\xBA` \x93\x83a\x0C\x81V[\x81\x01\x03\x12a\x04QWQ\x90` a\r\xA0V[=\x91Pa\x0E\xADV[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x0F\nWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x10\x05W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x10\x05W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r~\x972wWE-\x9E3\x15\xE3\x95\xBC\x10\x0E}\xA1\xF2\xD3Q\x06\xFF\xFD8\xF28\x07tp\x90\xEF\xA9\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f905f3560e01c9081630489160414614e0c5750806307e6233e146147415780630a6a8336146146a65780630a9254e414614257578063124fb3ce146140085780631ed7831c14613f8a5780632a8ea2e214613d925780632ade388014613b9e5780633e5e3c2314613b205780633f7286f414613aa257806347da5264146136ed5780634f8632ba146136c65780634fddb7a6146133015780635f15c3c9146132e55780635ff4c89914612d2c57806361d027b314612d0557806366a47d6514612b9357806366d9a9a014612a565780636d6d4436146127915780636f7c71ec14612506578063735fb47b146122a657806376029e7814611df057806385226c8114611d665780638f3b08f714611c12578063916a17c614611b6857806395a19046146117ea578063b0464fdc14611740578063b198d0281461171a578063b230c827146113a6578063b5508aa91461131c578063b72a6e9b14611091578063ba414fa61461106c578063befb967914611045578063cbe7fbac14610d00578063ce3e39c014610cd6578063d9a194701461091c578063dccc57f1146106d9578063e20c9f711461064b578063eced552614610628578063f851a44014610601578063fa7626d4146105de578063fc0c546a146105b85763fdc50aca146101f7575f80fd5b346105b55760206003193601126105b55760043560ff8116809103610587576102256030600160ff93615b48565b828060405161023381615194565b600c81527f426f756e6420726573756c74000000000000000000000000000000000000000060208201526040516102d2816102a060208201947fb60e72cc000000000000000000000000000000000000000000000000000000008652604060248401526064830190614fe8565b876044830152037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081018352826151dd565b51906a636f6e736f6c652e6c6f675afa5016816001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610587576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576105a0575b506001600160a01b03601f5460081c16803b156105875781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c5761058b575b5082915b808310610444578360048360206001600160a01b03601f5460081c16604051938480927fdf0244b10000000000000000000000000000000000000000000000000000000082525afa9182156104395783926103ff575b506103f7816103fc93615936565b6159ac565b80f35b91506020823d602011610431575b8161041a602093836151dd565b8101031261042d579051906103f76103e9565b5f80fd5b3d915061040d565b6040513d85823e3d90fd5b90836001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610587576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57610563575b5050601f546023546040517fd3f566ae0000000000000000000000000000000000000000000000000000000081526001600160a01b0391821660048201526024810186905291602091839160081c168188816044810103925af1908115610558578591610526575b5061051d90600192615267565b92019190610393565b90506020813d8211610550575b81610540602093836151dd565b8101031261042d57516001610510565b3d9150610533565b6040513d87823e3d90fd5b8161056d916151dd565b61057857835f6104a8565b8380fd5b6040513d84823e3d90fd5b5080fd5b81610595916151dd565b61058757815f61038f565b816105aa916151dd565b61058757815f610347565b80fd5b50346105b557806003193601126105b55760206001600160a01b03815416604051908152f35b50346105b557806003193601126105b557602060ff601f54166040519015158152f35b50346105b557806003193601126105b55760206001600160a01b0360215416604051908152f35b50346105b557806003193601126105b5576020604051670de0b6b3a76400008152f35b50346105b557806003193601126105b55760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106106ba576106b6856106aa818703826151dd565b60405191829182614fa6565b0390f35b82546001600160a01b0316845260209093019260019283019201610693565b50346105b557806003193601126105b5576001600160a01b03601f5460081c166040517fa217fddf000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104395783916108e9575b506021546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b03166024820152602081604481855afa80156104395783906108aa575b6107969150615a2d565b6040517f89162486000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610439578391610876575b506022546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529060209082908180604481015b03915afa801561057c57829061083b575b6103fc9150615a2d565b506020813d60201161086e575b81610855602093836151dd565b81010312610587576108696103fc9161538f565b610831565b3d9150610848565b90506020813d6020116108a2575b81610891602093836151dd565b8101031261042d57516108206107d3565b3d9150610884565b506020813d6020116108e1575b816108c4602093836151dd565b810103126108dd576108d86107969161538f565b61078c565b8280fd5b3d91506108b7565b90506020813d602011610914575b81610904602093836151dd565b8101031261042d57516020610736565b3d91506108f7565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57610cc1575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57610cac575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517f491cc7c2000000000000000000000000000000000000000000000000000000008152600160048201528160248201526001604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57610c97575b506001600160a01b036022541680827fb813ffbe387d6cf6e6a6f6c5f8905f766a0f1c6cd01c67312f709356c62597bd6020604051670c7d713b49da00008152a3737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57610c82575b506001600160a01b03601f5460081c16803b15610c35578180916024604051809481937f43581010000000000000000000000000000000000000000000000000000000008352670c7d713b49da000060048401525af1801561057c57610c6d575b50600460206001600160a01b03601f5460081c16604051928380927fac12ce070000000000000000000000000000000000000000000000000000000082525afa90811561057c578291610c38575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c3557604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152670c7d713b49da000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561057c57610c245750f35b81610c2e916151dd565b6105b55780f35b50fd5b9150506020813d602011610c65575b81610c54602093836151dd565b8101031261042d578190515f610ba5565b3d9150610c47565b81610c77916151dd565b6105b557805f610b57565b81610c8c916151dd565b6105b557805f610af6565b81610ca1916151dd565b6105b557805f610a5f565b81610cb6916151dd565b6105b557805f6109d8565b81610ccb916151dd565b6105b557805f610990565b50346105b557806003193601126105b55760206001600160a01b03601f5460081c16604051908152f35b50346105b557806003193601126105b557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517f491cc7c200000000000000000000000000000000000000000000000000000000815281600482015281602482015281604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611030575b50507fc12c60abc216286ef25e34b1805a0c3dda73e4c2fd6cf360e807a7a9e73167396020604051670d2f13f7789f00008152a1806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c5761101b575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57611006575b506001600160a01b03601f5460081c166040517f158ef93e000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610439578391610fbd575b50600491610ecf602092615a2d565b604051928380927fac12ce070000000000000000000000000000000000000000000000000000000082525afa90811561057c578291610f88575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c3557604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152670d2f13f7789f000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561057c57610c245750f35b9150506020813d602011610fb5575b81610fa4602093836151dd565b8101031261042d578190515f610f09565b3d9150610f97565b90506020813d602011610ffe575b81610fd8602093836151dd565b81010312610ffa57600491610ecf610ff160209361538f565b92505091610ec0565b5050fd5b3d9150610fcb565b81611010916151dd565b6105b557805f610e73565b81611025916151dd565b6105b557805f610e2b565b8161103a916151dd565b6105b557805f610d94565b50346105b557806003193601126105b55760206001600160a01b0360225416604051908152f35b50346105b557806003193601126105b557602061108761566e565b6040519015158152f35b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611307575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c576112f2575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576112dd575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576112c8575b5060206001600160a01b03601f5460081c166044604051809481937fd3f566ae0000000000000000000000000000000000000000000000000000000083528160048401528160248401525af1801561057c5761129c575080f35b6112bd9060203d6020116112c1575b6112b581836151dd565b81019061521e565b5080f35b503d6112ab565b816112d2916151dd565b6105b557805f611242565b816112e7916151dd565b6105b557805f6111b0565b816112fc916151dd565b6105b557805f61114d565b81611311916151dd565b6105b557805f611105565b50346105b557806003193601126105b55760195461133981615274565b9161134760405193846151dd565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b83831061138957604051806106b68782615080565b6001602081926113988561528c565b815201920192019190611374565b50346105b557806003193601126105b557737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611705575b50506001600160a01b03602154166001600160a01b036022541690604051611342928382019082821067ffffffffffffffff8311176116d857606091839161987a95878785398883526020830152604082015203019084f01561057c57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108dd57826040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576116c3575b50506001600160a01b03602054166001600160a01b036022541690604051918483019183831067ffffffffffffffff84111761168157918391606093878785398252876020830152604082015203019084f01561057c57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108dd57826040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fd92e233d000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576116ae575b50506001600160a01b03602054166001600160a01b036021541690604051938085019385851067ffffffffffffffff86111761168157918593916060959385398252602082015284604082015203019082f0156116755780f35b604051903d90823e3d90fd5b6024877f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b816116b8916151dd565b6108dd57825f61161b565b816116cd916151dd565b6108dd57825f611534565b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b8161170f916151dd565b6105b557805f611447565b50346105b557806003193601126105b55760206040516a422ca8b0a00a42500000008152f35b50346105b557806003193601126105b557601c5461175d81615274565b9161176b60405193846151dd565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b8383106117ad57604051806106b687826150fd565b600260206001926040516117c081615194565b6001600160a01b0386541681526117d88587016153ca565b83820152815201920192019190611798565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611b53575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57611b3e575b505b60308110611a375750806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611a22575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f9e91c9e7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611a0d575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927fd3f566ae0000000000000000000000000000000000000000000000000000000084526004840152603060248401525af1801561057c5761129c575080f35b81611a17916151dd565b6105b557805f6119a6565b81611a2c916151dd565b6105b557805f611914565b816001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610587576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611b29575b5050601f546023546040517fd3f566ae0000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152602481018490529291602091849160081c168186816044810103925af191821561043957600192611b0b575b50016118a8565b611b229060203d81116112c1576112b581836151dd565b505f611b04565b81611b33916151dd565b61058757815f611a9a565b81611b48916151dd565b6105b557805f6118a6565b81611b5d916151dd565b6105b557805f61185e565b50346105b557806003193601126105b557601d54611b8581615274565b91611b9360405193846151dd565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310611bd557604051806106b687826150fd565b60026020600192604051611be881615194565b6001600160a01b038654168152611c008587016153ca565b83820152815201920192019190611bc0565b50346105b557806003193601126105b557806001600160a01b0360245416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611d51575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517ff4844814000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57611d3c575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57610c245750f35b81611d46916151dd565b6105b557805f611cf2565b81611d5b916151dd565b6105b557805f611c86565b50346105b557806003193601126105b557601a54611d8381615274565b91611d9160405193846151dd565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b838310611dd357604051806106b68782615080565b600160208192611de28561528c565b815201920192019190611dbe565b50346105b557806003193601126105b5576001600160a01b03601f5460081c16816040517f5bdf6ca1000000000000000000000000000000000000000000000000000000008152602081600481865afa90811561057c578291612264575b506001600160a01b036020541690737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156108dd576040517f515361f60000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152911660248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561057c5761224f575b50506040517f5f15c3c9000000000000000000000000000000000000000000000000000000008152602081600481855afa801561043957839061221b575b611f2391506158bf565b6040517fb198d028000000000000000000000000000000000000000000000000000000008152602081600481855afa80156104395783906121e7575b611f699150615747565b816040517feced5526000000000000000000000000000000000000000000000000000000008152602081600481865afa90811561057c5782916121b2575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561058757604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152670de0b6b3a764000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561057c5761219d575b50506040517f76671808000000000000000000000000000000000000000000000000000000008152602081600481855afa8015610439578390612169575b61206c9150615849565b6040517fdf0244b1000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610439578391612136575b506004916120b8602092615849565b604051928380927f158ef93e0000000000000000000000000000000000000000000000000000000082525afa801561057c5782906120fb575b6103fc9150615a9f565b506020813d60201161212e575b81612115602093836151dd565b81010312610587576121296103fc9161538f565b6120f1565b3d9150612108565b90506020813d602011612161575b81612151602093836151dd565b8101031261042d575160046120a9565b3d9150612144565b506020813d602011612195575b81612183602093836151dd565b8101031261042d5761206c9051612062565b3d9150612176565b816121a7916151dd565b61058757815f612024565b9150506020813d6020116121df575b816121ce602093836151dd565b8101031261042d578290515f611fa7565b3d91506121c1565b506020813d602011612213575b81612201602093836151dd565b8101031261042d57611f699051611f5f565b3d91506121f4565b506020813d602011612247575b81612235602093836151dd565b8101031261042d57611f239051611f19565b3d9150612228565b81612259916151dd565b61058757815f611edb565b90506020813d60201161229e575b8161227f602093836151dd565b8101031261058757516001600160a01b0381168103610587575f611e4e565b3d9150612272565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576124f1575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c576124dc575b506001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576124c7575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527feb769920000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576124b2575b506001600160a01b03601f5460081c16803b15610c35578180916024604051809481937f435810100000000000000000000000000000000000000000000000000000000083528160048401525af1801561057c57610c245750f35b816124bc916151dd565b6105b557805f612457565b816124d1916151dd565b6105b557805f6123c5565b816124e6916151dd565b6105b557805f612362565b816124fb916151dd565b6105b557805f61231a565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c5761277c575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57612767575b50600460206001600160a01b03601f5460081c16604051928380927f43a3f8a10000000000000000000000000000000000000000000000000000000082525afa90811561057c578291612732575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ffa576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561043957839161271d575b505060206001600160a01b03601f5460081c1660446001600160a01b036023541660405195869384927fd3f566ae00000000000000000000000000000000000000000000000000000000845260048401528160248401525af19081156104395783916126e7575b6103fc9250615936565b90506020823d602011612715575b81612702602093836151dd565b8101031261042d576103fc9151906126dd565b3d91506126f5565b81612727916151dd565b610c3557815f612676565b9150506020813d60201161275f575b8161274e602093836151dd565b8101031261042d578190515f612610565b3d9150612741565b81612771916151dd565b6105b557805f6125c2565b81612786916151dd565b6105b557805f61257a565b50346105b55760206003193601126105b557806127ba670de0b6b3a763ffff6001600435615b48565b81806040516127c881615194565b600c81527f426f756e6420726573756c7400000000000000000000000000000000000000006020820152604051612835816102a060208201947fb60e72cc000000000000000000000000000000000000000000000000000000008652604060248401526064830190614fe8565b51906a636f6e736f6c652e6c6f675afa506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ffa576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610439578391612a41575b50506001600160a01b03601f5460081c1690813b15610ffa57829160248392604051948593849263318e825160e21b845260048401525af1801561057c57612a2c575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57612a17575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927fd3f566ae00000000000000000000000000000000000000000000000000000000845260048401528160248401525af190811561057c5782916129da575b506a422ca8b0a00a4250000000816129d36103fc931515615a2d565b1115615a2d565b90506020813d602011612a0f575b816129f5602093836151dd565b8101031261042d57516a422ca8b0a00a42500000006129b7565b3d91506129e8565b81612a21916151dd565b6105b557805f612951565b81612a36916151dd565b6105b557805f6128ee565b81612a4b916151dd565b610c3557815f6128ab565b50346105b557806003193601126105b557601b54612a7381615274565b612a8060405191826151dd565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b838310612b5857868587604051928392602084019060208552518091526040840160408260051b8601019392905b828210612aed57505050500390f35b91936020612b48827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0600195979984950301865288519083612b388351604084526040840190614fe8565b920151908481840391015261502b565b9601920192018594939192612ade565b60026020600192604051612b6b81615194565b612b748661528c565b8152612b818587016153ca565b83820152815201920192019190612ab0565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57612cf0575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527feb769920000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57612cdb575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b83528160048401525af1801561057c57610c245750f35b81612ce5916151dd565b6105b557805f612c99565b81612cfa916151dd565b6105b557805f612c07565b50346105b557806003193601126105b55760206001600160a01b0360235416604051908152f35b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576132d0575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c576132bb575b505b602f81106131b4575080600460206001600160a01b03601f5460081c16604051928380927fe4b7fb730000000000000000000000000000000000000000000000000000000082525afa90811561057c57829161317f575b506001600160a01b03602054169060206001600160a01b03602354166024604051809581937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa918215610439578392613148575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15613143576040519063ca669fa760e01b82526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156130ed57849161312e575b505060206001600160a01b03601f5460081c1660446001600160a01b036023541660405196879384927fd3f566ae0000000000000000000000000000000000000000000000000000000084526004840152602f60248401525af19283156130ed5784936130f8575b50612f779083615936565b6001600160a01b03602054169060206001600160a01b03602354166024604051809581937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa9182156130ed5784926130b7575b50612fe792612fe191615267565b90615936565b6001600160a01b03601f5460081c166040517f76671808000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610439578391613084575b506004916130426020926158bf565b604051928380927ffa391c640000000000000000000000000000000000000000000000000000000082525afa801561057c57829061083b576103fc9150615a2d565b90506020813d6020116130af575b8161309f602093836151dd565b8101031261042d57516004613033565b3d9150613092565b9091506020813d6020116130e5575b816130d3602093836151dd565b8101031261042d575190612fe7612fd3565b3d91506130c6565b6040513d86823e3d90fd5b9092506020813d602011613126575b81613114602093836151dd565b8101031261042d575191612f77612f6c565b3d9150613107565b81613138916151dd565b610ffa57825f612f04565b505050fd5b925090506020823d602011613177575b81613165602093836151dd565b8101031261042d57829151905f612e9e565b3d9150613158565b9150506020813d6020116131ac575b8161319b602093836151dd565b8101031261042d578190515f612e41565b3d915061318e565b816001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610587576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576132a6575b5050601f546023546040517fd3f566ae0000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152602481018490529291602091849160081c168186816044810103925af191821561043957600192613288575b5001612dea565b61329f9060203d81116112c1576112b581836151dd565b505f613281565b816132b0916151dd565b61058757815f613217565b816132c5916151dd565b6105b557805f612de8565b816132da916151dd565b6105b557805f612da0565b50346105b557806003193601126105b557602060405160308152f35b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c576136b1575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c5761369c575b5050600460a06001600160a01b03601f5460081c16604051928380927fa088787d0000000000000000000000000000000000000000000000000000000082525afa90811561057c57613441918384859086928794613661575b61343c949550613437929161342d61343292615849565b6158bf565b615849565b615747565b615a9f565b806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c5761364c575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927fd3f566ae00000000000000000000000000000000000000000000000000000000845260048401528160248401525af190811561057c57829161361a575b50600460a06001600160a01b03601f5460081c16604051928380927fa088787d0000000000000000000000000000000000000000000000000000000082525afa9182156104395783848591869487966135d1575b5091613571849261342d613576956157d2565b615936565b6a422ca8b0a00a425000000003906a422ca8b0a00a425000000082116135a4576103fc929161343c91615936565b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b61342d965061357195506135769350849250613605915060a03d60a011613613575b6135fd81836151dd565b81019061539c565b98509690945090925061355e565b503d6135f3565b90506020813d602011613644575b81613635602093836151dd565b8101031261042d57515f61350a565b3d9150613628565b81613656916151dd565b6105b557805f6134a4565b505050505061343c61343261343761368a61342d9460a03d60a011613613576135fd81836151dd565b93975092955091935090915084613416565b816136a6916151dd565b6105b557805f6133bd565b816136bb916151dd565b6105b557805f613375565b50346105b557806003193601126105b55760206001600160a01b0360245416604051908152f35b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57613a8d575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d99a8cec7e2000060048401525af1801561057c57613a78575b50815b6030811061395157506137be816159ac565b6001600160a01b03601f5460081c1690604051907fdf0244b1000000000000000000000000000000000000000000000000000000008252602082600481865afa80156130ed57849061391d575b6138159250615936565b6040517ffa391c64000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156104395783916138d8575b50600491613861602092615a2d565b604051928380927fe4b7fb730000000000000000000000000000000000000000000000000000000082525afa801561057c5782906138a4575b6103fc9150615849565b506020813d6020116138d0575b816138be602093836151dd565b8101031261042d576103fc905161389a565b3d91506138b1565b90506020813d602011613915575b816138f3602093836151dd565b810103126108dd5760049161386161390c60209361538f565b92505091613852565b3d91506138e6565b506020823d602011613949575b81613937602093836151dd565b8101031261042d57613815915161380b565b3d915061392a565b90826001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610587576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57613a63575b5050601f546023546040517fd3f566ae0000000000000000000000000000000000000000000000000000000081526001600160a01b0391821660048201526024810185905291602091839160081c168187816044810103925af19081156130ed578491613a31575b50613a2a90600192615267565b91016137ac565b90506020813d8211613a5b575b81613a4b602093836151dd565b8101031261042d57516001613a1d565b3d9150613a3e565b81613a6d916151dd565b6108dd57825f6139b5565b81613a82916151dd565b6105b557805f6137a9565b81613a97916151dd565b6105b557805f613761565b50346105b557806003193601126105b55760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110613b01576106b6856106aa818703826151dd565b82546001600160a01b0316845260209093019260019283019201613aea565b50346105b557806003193601126105b55760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b818110613b7f576106b6856106aa818703826151dd565b82546001600160a01b0316845260209093019260019283019201613b68565b50346105b557806003193601126105b557601e54613bbb81615274565b613bc860405191826151dd565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b838310613d095786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310613c345786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110613cc057505050505060208060019297019301930190928695949293613c27565b9091929394602080613cfc837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa087600196030189528951614fe8565b9701950193929101613c9c565b604051613d1581615194565b6001600160a01b038354168152600183018054613d3181615274565b91613d3f60405193846151dd565b8183528a526020808b20908b9084015b838210613d75575050505060019282602092836002950152815201920192019190613bf8565b600160208192613d848661528c565b815201930191019091613d4f565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57613f75575b506001600160a01b03601f5460081c16803b15610c3557819060246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57613f60575b5060049060206001600160a01b03601f5460081c16604051938480927fe0e6169c0000000000000000000000000000000000000000000000000000000082525afa918215613f53578192613f1f575b50670d2f13f7789f000060015b60308110613ebc57506103fc9192615936565b90670d2f13f7789f0000810290808204670d2f13f7789f00001490151715613ef257670de0b6b3a7640000600191049101613ea9565b6024837f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b9091506020813d602011613f4b575b81613f3b602093836151dd565b8101031261042d5751905f613e9c565b3d9150613f2e565b50604051903d90823e3d90fd5b613f6b8280926151dd565b6105b5575f613e4d565b81613f7f916151dd565b6105b557805f613e06565b50346105b557806003193601126105b55760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b818110613fe9576106b6856106aa818703826151dd565b82546001600160a01b0316845260209093019260019283019201613fd2565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57614242575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c5761422d575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57614218575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f9e91c9e7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57614203575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670c7d713b49da000060048401525af1801561057c57610c245750f35b8161420d916151dd565b6105b557805f6141b9565b81614222916151dd565b6105b557805f614127565b81614237916151dd565b6105b557805f6140c4565b8161424c916151dd565b6105b557805f61407c565b50346105b557806003193601126105b5576001600160a01b03602154166001600160a01b036023541660405191613b718084019084821067ffffffffffffffff8311176116d857918493916142c593615d0986396001600160a01b0391821681529116602082015260400190565b039082f08015613f53576001600160a01b0316807fffffffffffffffffffffffff000000000000000000000000000000000000000060205416176020556001600160a01b0360215416906001600160a01b036022541660405192611342928385019385851067ffffffffffffffff86111761168157918593916060959361987a863983526020830152604082015203019082f08015613f53577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f5580600460206001600160a01b03815416604051928380927f8d3343d60000000000000000000000000000000000000000000000000000000082525afa90811561057c578291614671575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ffa576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561043957839161465c575b50506001600160a01b0360205416906001600160a01b03601f5460081c16823b15613143576040517f2f2ff15d00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529082908290604490829084905af1801561057c57614647575b50600460206001600160a01b03601f5460081c16604051928380927fdebe4f1f0000000000000000000000000000000000000000000000000000000082525afa90811561057c578291614612575b506001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ffa576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156104395783916145fd575b50506001600160a01b03601f5460081c16906001600160a01b0360215416823b15613143576040517f2f2ff15d00000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529082908290604490829084905af1801561057c57610c245750f35b81614607916151dd565b610c3557815f614581565b9150506020813d60201161463f575b8161462e602093836151dd565b8101031261042d578190515f61451b565b3d9150614621565b81614651916151dd565b6105b557805f6144cd565b81614666916151dd565b610c3557815f614453565b9150506020813d60201161469e575b8161468d602093836151dd565b8101031261042d578190515f6143ed565b3d9150614680565b50346105b557806003193601126105b557600460206001600160a01b03601f5460081c16604051928380927fe4b7fb730000000000000000000000000000000000000000000000000000000082525afa801561057c57829061470d575b6103fc9150615747565b506020813d602011614739575b81614727602093836151dd565b8101031261042d576103fc9051614703565b3d915061471a565b50346105b557806003193601126105b557806001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c35576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57614df7575b506001600160a01b03601f5460081c16803b15610c355781809160246040518094819363318e825160e21b8352670d2f13f7789f000060048401525af1801561057c57614de2575b50506001600160a01b03601f5460081c16604051907fe4b7fb73000000000000000000000000000000000000000000000000000000008252602082600481845afa918215610439578392614dad575b50602060049161485b84615747565b604051928380927fe0e6169c0000000000000000000000000000000000000000000000000000000082525afa908115610439578391614d7b575b5066b1a2bc2ec50000820282810466b1a2bc2ec5000014831517156135a45781670de0b6b3a76400000391670de0b6b3a76400008311614d4e57670de0b6b3a764000014614d215704906001600160a01b03602054169060206001600160a01b03602354166024604051809581937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa9182156130ed578492614ced575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610578576040517f491cc7c2000000000000000000000000000000000000000000000000000000008152600160048201528460248201528460448201526001606482015284808260848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1918215613f53578592614cd4575b50506149ce9161522d565b837f160fc195d6e53691d30d804ce190dc09471891677e43433b91a7a6131c12a59a60406001600160a01b0360235416938151908782526020820152a3826001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610587576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57614cbf575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927fd3f566ae00000000000000000000000000000000000000000000000000000000845260048401528160248401525af180156130ed5783908590614c89575b614adf9250615936565b6001600160a01b03602054169060206001600160a01b03602354166024604051809581937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa9081156130ed5783928592614c50575b50614b4b92612fe191615267565b6001600160a01b03601f5460081c16906040517f76671808000000000000000000000000000000000000000000000000000000008152602081600481865afa9081156130ed578491614c1d575b50600492614ba76020926157d2565b604051938480927fdf0244b10000000000000000000000000000000000000000000000000000000082525afa8015610439578390614be9576103fc9250615936565b506020823d602011614c15575b81614c03602093836151dd565b8101031261042d576103fc91516126dd565b3d9150614bf6565b90506020813d602011614c48575b81614c38602093836151dd565b8101031261042d57516004614b98565b3d9150614c2b565b925090506020823d602011614c81575b81614c6d602093836151dd565b8101031261042d5790518291614b4b614b3d565b3d9150614c60565b50506020813d602011614cb7575b81614ca4602093836151dd565b8101031261042d5782614adf9151614ad5565b3d9150614c97565b81614cc9916151dd565b6108dd57825f614a6e565b81925090614ce1916151dd565b6105785782845f6149c3565b9091506020813d602011614d19575b81614d09602093836151dd565b8101031261042d5751905f61493b565b3d9150614cfc565b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526012600452fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d602011614da5575b81614d96602093836151dd565b8101031261042d57515f614895565b3d9150614d89565b9091506020813d602011614dda575b81614dc9602093836151dd565b8101031261042d575190602061484c565b3d9150614dbc565b81614dec916151dd565b6105b557805f6147fd565b81614e01916151dd565b6105b557805f6147b5565b90503461042d575f60031936011261042d576001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d5763ca669fa760e01b825260048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015614f9b57614f88575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105b557806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f9e91c9e7000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561057c57614f73575b5060206001600160a01b03601f5460081c1660446001600160a01b036023541660405194859384927fd3f566ae00000000000000000000000000000000000000000000000000000000845260048401528160248401525af1801561057c5761129c575080f35b81614f7d916151dd565b6105b557805f614f0d565b614f9491505f906151dd565b5f5f614e7c565b6040513d5f823e3d90fd5b60206040818301928281528451809452019201905f5b818110614fc95750505090565b82516001600160a01b0316845260209384019390920191600101614fbc565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b8181106150485750505090565b82517fffffffff000000000000000000000000000000000000000000000000000000001684526020938401939092019160010161503b565b602081016020825282518091526040820191602060408360051b8301019401925f915b8383106150b257505050505090565b90919293946020806150ee837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187528951614fe8565b970193019301919392906150a3565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061512f57505050505090565b9091929394602080615185837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b0381511684520151918185820152019061502b565b97019301930191939290615120565b6040810190811067ffffffffffffffff8211176151b057604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176151b057604052565b9081602091031261042d575190565b9190820391821161523a57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b9190820180921161523a57565b67ffffffffffffffff81116151b05760051b60200190565b90604051915f8154908160011c9260018316928315615385575b60208510841461535857848752869390811561531857506001146152d4575b506152d2925003836151dd565b565b90505f9291925260205f20905f915b8183106152fc5750509060206152d2928201015f6152c5565b60209193508060019154838589010152019101909184926152e3565b602093506152d29592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f6152c5565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f16936152a6565b5190811515820361042d57565b908160a091031261042d578051916020820151916040810151916153c760806060840151930161538f565b90565b90604051918281549182825260208201905f5260205f20925f905b8060078301106155e1576152d29454918181106155ab575b818110615575575b81811061553f575b818110615509575b8181106154d3575b81811061549d575b818110615468575b1061543b575b5003836151dd565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f615433565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b16815201930161542d565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301615425565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b16815201930161541d565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301615415565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b16815201930161540d565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b168152019301615405565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b1681520193016153fd565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e08201520194019201859293916153e5565b60085460ff16801561567d5790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115614f9b575f91615715575b50151590565b90506020813d60201161573f575b81615730602093836151dd565b8101031261042d57515f61570f565b3d9150615723565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526a422ca8b0a00a425000000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b5f6152d2916151dd565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152600160248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152603060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051917f98296c54000000000000000000000000000000000000000000000000000000008352600483015260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907f8466f41500000000000000000000000000000000000000000000000000000000825260048201526a422ca8b0a00a425000000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561042d57604051907fa5982885000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614f9b576157c85750565b8115615b1b570690565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f90838311615c845782811091821580615c7a575b615c7257615b6b848661522d565b926001840180941161523a57600383111580615c69575b615c5a5760031983101580615c50575b615c3f5785831115615bf657505090615bae84615bb39361522d565b615b11565b908115615bf157615bc49250615267565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff810190811161523a5790565b505090565b959492919095615c07575b50505050565b83949550615bae90615c19939461522d565b908115615bf157615c2a925061522d565b6001810180911161523a57905f808080615c01565b505090506153c7929150199061522d565b5082198411615b92565b50509190506153c79250615267565b50828411615b82565b509250505090565b5084821115615b5d565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603e60248201527f5374645574696c7320626f756e642875696e743235362c75696e743235362c7560448201527f696e74323536293a204d6178206973206c657373207468616e206d696e2e00006064820152fdfe6101806040523461007d5761001b6100156100e2565b90610198565b604051612d0b9081610e06823960805181611cdb015260a05181611d98015260c05181611cac015260e05181611d2a01526101005181611d5001526101205181610d6201526101405181610d8b015261016051818181610c400152610c890152f35b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b601f909101601f19168101906001600160401b038211908210176100b857604052565b610081565b604051906100cc604083610095565b565b51906001600160a01b038216820361007d57565b613b71906040823803928382519485926100fc8285610095565b83398101031261007d5761011b6020610114846100ce565b93016100ce565b90565b6040519061012d604083610095565b60048252565b60405190610142604083610095565b60018252565b60405190610157604083610095565b600982526853796e64696361746560b81b6020830152565b634e487b7160e01b5f52601160045260245ffd5b90629e3400820180921161019357565b61016f565b906101a1610148565b6101a9610148565b906101b261011e565b906314d6539160e21b60208301526101c8610133565b603160f81b60208201908152845190949193916001600160401b0382116100b8576101fd826101f860035461035e565b610396565b602090601f83116001146102d75791806102319261023995945f926102cc575b50508160011b915f199060031b1c19161790565b600355610435565b61024281610684565b6101205261024f82610776565b610140526020815191012060e052519020610100524660a052610270610868565b6080523060c0526001600160a01b038216156102bd576001600160a01b038116156102bd576102b76100cc926102a542610183565b610160526102b25f600c55565b61050e565b50610597565b63d92e233d60e01b5f5260045ffd5b015190505f8061021d565b60035f52601f19831691907fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b925f5b81811061034657509160019391856102399796941061032e575b505050811b01600355610435565b01515f1960f88460031b161c191690555f8080610320565b92936020600181928786015181550195019301610306565b90600182811c9216801561038c575b602083101461037857565b634e487b7160e01b5f52602260045260245ffd5b91607f169161036d565b601f81116103a2575050565b60035f5260205f20906020601f840160051c830193106103dc575b601f0160051c01905b8181106103d1575050565b5f81556001016103c6565b90915081906103bd565b601f82116103f357505050565b5f5260205f20906020601f840160051c8301931061042b575b601f0160051c01905b818110610420575050565b5f8155600101610415565b909150819061040c565b80519091906001600160401b0381116100b85761045e8161045760045461035e565b60046103e6565b602092601f82116001146104925761048d929382915f926102cc5750508160011b915f199060031b1c19161790565b600455565b60045f52601f198216937f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b915f5b8681106104f657508360019596106104de575b505050811b01600455565b01515f1960f88460031b161c191690555f80806104d3565b919260206001819286850151815501940192016104c0565b6001600160a01b0381165f9081525f516020613b315f395f51905f52602052604090205460ff16610592576001600160a01b03165f8181525f516020613b315f395f51905f5260205260408120805460ff191660011790553391907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d8180a4600190565b505f90565b6001600160a01b0381168015610671576002546b02f90193ef3075fa980000008101809111610193576002556001600160a01b0382165f9081526020819052604090206b02f90193ef3075fa9800000081540190555f7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef6040518061062d6b02f90193ef3075fa98000000829190602083019252565b0390a36002546001600160d01b039081811161065c5750506b02f90193ef3075fa980000006100cc915f610931565b630e58ae9360e11b5f5260045260245260445ffd5b63ec442f0560e01b5f525f60045260245ffd5b908151602081105f1461069c57509061011b906108c6565b6001600160401b0381116100b8576106c0816106b960065461035e565b60066103e6565b602092601f82116001146106f7576106ef929382915f926102cc5750508160011b915f199060031b1c19161790565b60065560ff90565b60065f52601f198216937ff652222313e28459528d920b65115c16c04f3efc82aaedc97be59f3f377c0d3f915f5b86811061075e5750836001959610610746575b505050811b0160065560ff90565b01515f1960f88460031b161c191690555f8080610738565b91926020600181928685015181550194019201610725565b908151602081105f1461078e57509061011b906108c6565b6001600160401b0381116100b8576107b2816107ab60075461035e565b60076103e6565b602092601f82116001146107e9576107e1929382915f926102cc5750508160011b915f199060031b1c19161790565b60075560ff90565b60075f52601f198216937fa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c688915f5b8681106108505750836001959610610838575b505050811b0160075560ff90565b01515f1960f88460031b161c191690555f808061082a565b91926020600181928685015181550194019201610817565b60e051610100516040519060208201927f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8452604083015260608201524660808201523060a082015260a081526108c060c082610095565b51902090565b601f8151116108f15760208151910151602082106108e2571790565b5f198260200360031b1b161790565b604460209160405192839163305a27a960e01b83528160048401528051918291826024860152018484015e5f828201840152601f01601f19168101030190fd5b9091906001600160a01b03168015610998575b6100cc926001600160a01b0316908115610980575b5f90815260096020526040808220549282529020546001600160a01b039081169116610b62565b61099161098c84610a33565b610a64565b5050610959565b6109a182610a33565b9265ffffffffffff4311610a1b57600b54806109e557506109db6109cb6100cc955f5b6001610da9565b65ffffffffffff4316600b610cd3565b9050509250610944565b93845f1981011161019357600b5f525f516020613b115f395f51905f52909401546100cc946109db916109cb919060301c6109c4565b6306dfcc6560e41b5f5260306004524360245260445ffd5b6001600160d01b038111610a4d576001600160d01b031690565b6306dfcc6560e41b5f5260d060045260245260445ffd5b65ffffffffffff4311610a1b57600b5480610a8e57506109cb610a8a915f5b6002610da9565b9091565b805f1981011161019357600b5f525f516020613b115f395f51905f520154610a8a916109cb9160301c610a83565b65ffffffffffff4311610a1b57805480610af05750610ae0610a8a925f6002610da9565b9065ffffffffffff431690610cd3565b805f19810111610193575f82815260209020015f190154610a8a92610ae09160301c610a83565b65ffffffffffff4311610a1b57805480610b3b5750610ae0610a8a925f6001610da9565b805f19810111610193575f82815260209020015f190154610a8a92610ae09160301c6109c4565b6001600160a01b03808316939291908116908185141580610c55575b610b8a575b5050505050565b81610bfb575b505082610b9f575b8080610b83565b6001600160a01b03165f908152600a602052604090205f516020613b515f395f51905f5291610bd891610bd29091610a33565b90610b17565b604080516001600160d01b039384168152919092166020820152a25f8080610b98565b6001600160a01b03165f908152600a602052604090205f516020613b515f395f51905f5290610c3390610c2d86610a33565b90610abc565b604080516001600160d01b039384168152919092166020820152a25f80610b90565b50831515610b7e565b5f1981019190821161019357565b908154680100000000000000008110156100b85760018101808455811015610cbf575f9283526020928390208251929093015160301b65ffffffffffff191665ffffffffffff9290921691909117910155565b634e487b7160e01b5f52603260045260245ffd5b80549293928015610d7f57610cea610cf591610c5e565b825f5260205f200190565b8054603081901c9365ffffffffffff91821692918116808411610d7057879303610d3c5750610d3892509065ffffffffffff82549181199060301b169116179055565b9190565b915050610d3891610d5c610d4e6100bd565b65ffffffffffff9093168352565b6001600160d01b0386166020830152610c6c565b632520601d60e01b5f5260045ffd5b5090610da491610d90610d4e6100bd565b6001600160d01b0385166020830152610c6c565b5f9190565b91909180600114610deb57600214610dcf57634e487b7160e01b5f52605160045260245ffd5b6001600160d01b03908116918116919091039081116101935790565b506001600160d01b0391821690821601908111610193579056fe60806040526004361015610011575f80fd5b5f3560e01c806301ffc9a7146102e557806306fdde03146102e0578063095ea7b3146102db57806318160ddd1461022c57806323b872dd146102d6578063248a9ca3146102d15780632f2ff15d146102cc578063313ce567146102c75780633644e515146102c257806336568abe146102bd5780633a46b1a81461023657806340c10f19146102b857806342966c68146102b35780634bf5d7e9146102ae5780634f1bfc9e146102a9578063587cde1e146102a45780635c19a95c1461029f5780636fcfff451461029a57806370a082311461029557806379cc6790146102905780637a8cd1561461028b5780637ecebe001461028657806383f1211b146102815780638426adf21461027c578063844c90261461027757806384b0196e146102725780638a5425211461026d5780638d3343d6146102685780638e539e8c14610263578063902d55a51461025e57806391d148541461025957806391ddadf41461025457806395d89b411461024f5780639ab24eb0146102315780639b7ef64b1461024a578063a217fddf14610245578063a9059cbb14610240578063aa082a9d1461023b578063b0ca253e14610236578063bb4d443614610231578063c02ae7541461022c578063c3cda52014610227578063d505accf14610222578063d547741f1461021d578063dd62ed3e146102185763f1127ed814610213575f80fd5b611515565b6114bc565b61147e565b611324565b6111dd565b610536565b611117565b610722565b6111a0565b61117a565b611160565b61113a565b611072565b611047565b610ff7565b610fd1565b610ef5565b610ebb565b610e81565b610d4a565b610c63565b610c29565b610c05565b610bcd565b610bb3565b610b0a565b610ad5565b610a5a565b610a38565b6109f7565b6109da565b610931565b61090d565b610834565b6106c5565b6106ab565b610690565b61064b565b610618565b610553565b610505565b6103e1565b34610386576020600319360112610386576004357fffffffff00000000000000000000000000000000000000000000000000000000811680910361038657807f7965db0b000000000000000000000000000000000000000000000000000000006020921490811561035c575b506040519015158152f35b7f01ffc9a7000000000000000000000000000000000000000000000000000000009150145f610351565b5f80fd5b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b9060206103de92818152019061038a565b90565b34610386575f600319360112610386576040515f6003546104018161160b565b80845290600181169081156104975750600114610439575b610435836104298185038261174a565b604051918291826103cd565b0390f35b60035f9081527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b939250905b80821061047d57509091508101602001610429610419565b919260018160209254838588010152019101909291610465565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208086019190915291151560051b840190910191506104299050610419565b600435906001600160a01b038216820361038657565b602435906001600160a01b038216820361038657565b346103865760406003193601126103865761052b6105216104d9565b602435903361209e565b602060405160018152f35b34610386575f600319360112610386576020600254604051908152f35b346103865760606003193601126103865761056c6104d9565b6105746104ef565b604435906001600160a01b0383165f5260016020526105a73360405f20906001600160a01b03165f5260205260405f2090565b54925f1984106105c8575b6105bc935061184e565b60405160018152602090f35b8284106105e4576105df836105bc9503338361216c565b6105b2565b82847ffb8f41b2000000000000000000000000000000000000000000000000000000005f523360045260245260445260645ffd5b346103865760206003193601126103865760206106436004355f526005602052600160405f20015490565b604051908152f35b346103865760406003193601126103865761068e60043561066a6104ef565b90610689610684825f526005602052600160405f20015490565b611b89565b611bea565b005b34610386575f60031936011261038657602060405160128152f35b34610386575f600319360112610386576020610643611ca2565b34610386576040600319360112610386576004356106e16104ef565b336001600160a01b038216036106fa5761068e91611dbe565b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b346103865760406003193601126103865761073b6104d9565b6001600160a01b0360243591165f52600a60205261075c60405f2091611e6e565b8154905f8291600584116107dc575b610776935084612487565b806107a5575050602079ffffffffffffffffffffffffffffffffffffffffffffffffffff5f5b16604051908152f35b6020916107cc79ffffffffffffffffffffffffffffffffffffffffffffffffffff926117aa565b905f52825f20015460301c61079c565b91926107e781612312565b810390811161082f5761077693855f5265ffffffffffff8260205f2001541665ffffffffffff8516105f1461081d57509161076b565b9291506108299061178b565b9061076b565b6115de565b346103865760406003193601126103865761084d6104d9565b602435610858611a11565b6001600160a01b038216156108e55780156108bd5760025481810180911161082f576b033b2e3c9fd0803ce8000000106108955761068e916121b3565b7f177e3fc3000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f1f2a2005000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b346103865760206003193601126103865760043580156108bd5761068e9033611ec0565b34610386575f6003193601126103865761094a43612293565b65ffffffffffff8061095b43612293565b169116036109b25761043560405161097460408261174a565b601d81527f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c74000000602082015260405191829160208352602083019061038a565b7f6ff07140000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610386575f600319360112610386576020604051629e34008152f35b34610386576020600319360112610386576001600160a01b03610a186104d9565b165f52600960205260206001600160a01b0360405f205416604051908152f35b346103865760206003193601126103865761068e610a546104d9565b33611f8f565b34610386576020600319360112610386576001600160a01b03610a7b6104d9565b165f52600a60205260405f205463ffffffff8111610aa55760405163ffffffff9091168152602090f35b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52602060045260245260445ffd5b34610386576020600319360112610386576020610643610af36104d9565b6001600160a01b03165f525f60205260405f205490565b3461038657604060031936011261038657610b236104d9565b60243590610b2f611a99565b6001600160a01b0381169081156108e55782156108bd57610b4e6117e4565b15610b8b5782610b5d91611ec0565b6040519182527fbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a260203393a3005b7fb8b5ca2d000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610386575f6003193601126103865760206106436117b8565b34610386576020600319360112610386576001600160a01b03610bee6104d9565b165f526008602052602060405f2054604051908152f35b34610386575f600319360112610386576020610c1f6117e4565b6040519015158152f35b34610386575f6003193601126103865760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b3461038657602060031936011261038657600435610c7f611b21565b42811115610d22577f00000000000000000000000000000000000000000000000000000000000000008111610cfa577fdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec610cf5600c549280600c5560405191829133958360209093929193604081019481520152565b0390a2005b7fef69af65000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fa5658353000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610386575f60031936011261038657610e28610d867f000000000000000000000000000000000000000000000000000000000000000061263e565b610daf7f00000000000000000000000000000000000000000000000000000000000000006126b7565b6020604051610dbe828261174a565b5f815281610e36818301947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe083013687376040519788977f0f00000000000000000000000000000000000000000000000000000000000000895260e0858a015260e089019061038a565b90878203604089015261038a565b914660608701523060808701525f60a087015285830360c087015251918281520192915f5b828110610e6a57505050500390f35b835185528695509381019392810192600101610e5b565b34610386575f6003193601126103865760206040517f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a6748152f35b34610386575f6003193601126103865760206040517f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb8152f35b3461038657602060031936011261038657610f11600435611e6e565b600b54905f829160058411610f7d575b610f2d9350600b612487565b80610f5b575060205f5b79ffffffffffffffffffffffffffffffffffffffffffffffffffff60405191168152f35b610f78610f696020926117aa565b600b5f52825f20015460301c90565b610f37565b9192610f8881612312565b810390811161082f57610f2d93600b5f5265ffffffffffff8260205f2001541665ffffffffffff8516105f14610fbf575091610f21565b929150610fcb9061178b565b90610f21565b34610386575f6003193601126103865760206040516b033b2e3c9fd0803ce80000008152f35b3461038657604060031936011261038657602060ff61103b60043561101a6104ef565b905f526005845260405f20906001600160a01b03165f5260205260405f2090565b54166040519015158152f35b34610386575f60031936011261038657602061106243612293565b65ffffffffffff60405191168152f35b34610386575f600319360112610386576040515f6004546110928161160b565b808452906001811690811561049757506001146110b957610435836104298185038261174a565b60045f9081527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b939250905b8082106110fd57509091508101602001610429610419565b9192600181602092548385880101520191019092916110e5565b346103865760206003193601126103865760206106436111356104d9565b6117fb565b34610386575f6003193601126103865760206040516b02f90193ef3075fa980000008152f35b34610386575f6003193601126103865760206040515f8152f35b346103865760406003193601126103865761052b6111966104d9565b602435903361184e565b34610386575f600319360112610386576020600c54604051908152f35b6064359060ff8216820361038657565b6084359060ff8216820361038657565b346103865760c0600319360112610386576111f66104d9565b602435906044356112056111bd565b6084359060a435928042116112f9579161128b939161127d6112829460405160208101917fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf83526001600160a01b038a1660408301528a606083015260808201526080815261127560a08261174a565b51902061204e565b6126ee565b909291926127b2565b6112af816001600160a01b03165f52600860205260405f2080549060018201905590565b8093036112c05761068e9250611f8f565b6001600160a01b0391507f752d88c0000000000000000000000000000000000000000000000000000000005f521660045260245260445ffd5b7f4683af0e000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b346103865760e06003193601126103865761133d6104d9565b6113456104ef565b60443590606435926113556111cd565b60a43560c43590864211611452576113fe926113f961138e866001600160a01b03165f52600860205260405f2080549060018201905590565b9860405160208101917f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983526001600160a01b0389169b8c60408401526001600160a01b038b1660608401528b608084015260a083015260c082015260c0815261127560e08261174a565b61208f565b936001600160a01b038516036114185761068e935061209e565b7f4b800e46000000000000000000000000000000000000000000000000000000005f526001600160a01b038085166004521660245260445ffd5b867f62791302000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b346103865760406003193601126103865761068e60043561149d6104ef565b906114b7610684825f526005602052600160405f20015490565b611dbe565b3461038657604060031936011261038657602061150c6114da6104d9565b6001600160a01b036114ea6104ef565b91165f526001835260405f20906001600160a01b03165f5260205260405f2090565b54604051908152f35b346103865760406003193601126103865761152e6104d9565b6024359063ffffffff8216820361038657610435916001600160a01b0361157b92611557611836565b50611560611836565b50165f52600a60205260405f20611575611836565b50612879565b506040519061158982611729565b5465ffffffffffff8116825260301c602082015260405191829182919091602079ffffffffffffffffffffffffffffffffffffffffffffffffffff81604084019565ffffffffffff8151168552015116910152565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b90600182811c92168015611652575b602083101461162557565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f169161161a565b5f929181549161166b8361160b565b80835292600181169081156116c0575060011461168757505050565b5f9081526020812093945091925b8383106116a6575060209250010190565b600181602092949394548385870101520191019190611695565b905060209495507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091509291921683830152151560051b010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040810190811067ffffffffffffffff82111761174557604052565b6116fc565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761174557604052565b906001820180921161082f57565b604051906117a860408361174a565b565b905f19820191821161082f57565b600c54801580156117da575b6117d55742810390811161082f5790565b505f90565b50804210156117c4565b600c5480151590816117f4575090565b9050421090565b6001600160a01b03165f52600a60205279ffffffffffffffffffffffffffffffffffffffffffffffffffff61183260405f20612025565b1690565b6040519061184382611729565b5f6020838281520152565b9291906001600160a01b0384169384156119e5576001600160a01b03821680156119b95761187a6117e4565b80611981575b6119595761189e826001600160a01b03165f525f60205260405f2090565b549584871061191a57846117a89697036118c8846001600160a01b03165f525f60205260405f2090565b556118e3846001600160a01b03165f525f60205260405f2090565b8054860190556040518581527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a3612a50565b7fe450d38c000000000000000000000000000000000000000000000000000000005f526001600160a01b0383166004526024879052604485905260645ffd5b7fdb89e3f4000000000000000000000000000000000000000000000000000000005f5260045ffd5b50335f9081527f740796d87e4f86cc94671768c744956045fe855093291e79194c96ac478040aa602052604090205460ff1615611880565b7fec442f05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7f96c6fd1e000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b335f9081527f9a6bf48bb840e78fe8e7afd10d3d391a91738a9e6524f6fdfa1a3aba9dc03fb1602052604090205460ff1615611a4957565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb60245260445ffd5b335f9081527f740796d87e4f86cc94671768c744956045fe855093291e79194c96ac478040aa602052604090205460ff1615611ad157565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67460245260445ffd5b335f9081527f05b8ccbb9d4d8fb16ea74ce3c29a41f1b461fbdaff4714a0d9a8eb05499746bc602052604090205460ff1615611b5957565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b805f52600560205260ff611bb13360405f20906001600160a01b03165f5260205260405f2090565b541615611bbb5750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f52600560205260ff611c128360405f20906001600160a01b03165f5260205260405f2090565b5416611c9c57805f526005602052611c3e8260405f20906001600160a01b03165f5260205260405f2090565b60017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008254161790556001600160a01b03339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016301480611d95575b15611cfd577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f82527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a08152611d8f60c08261174a565b51902090565b507f00000000000000000000000000000000000000000000000000000000000000004614611cd4565b805f52600560205260ff611de68360405f20906001600160a01b03165f5260205260405f2090565b541615611c9c57805f526005602052611e138260405f20906001600160a01b03165f5260205260405f2090565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0081541690556001600160a01b03339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190565b65ffffffffffff611e7e43612293565b1680821015611e9157506103de90612293565b907fecd3f81e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b6001600160a01b03811680156119e557611eea826001600160a01b03165f525f60205260405f2090565b54838110611f5257915f8092856117a8969503611f17846001600160a01b03165f525f60205260405f2090565b556002805486900390556040518581527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a3612a50565b7fe450d38c000000000000000000000000000000000000000000000000000000005f526001600160a01b038316600452602452604483905260645ffd5b6001600160a01b038181165f81815260096020526040812080548685167fffffffffffffffffffffffff0000000000000000000000000000000000000000821681179092556117a89694169461201f9390928691907f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9080a46001600160a01b03165f525f60205260405f205490565b916124eb565b8054806120325750505f90565b805f1981011161082f575f19915f5260205f2001015460301c90565b604290612059611ca2565b90604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b916103de9391611282936126ee565b6001600160a01b0316908115612140576001600160a01b03811692831561211457806121077f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92593855f52600160205260405f20906001600160a01b03165f5260205260405f2090565b55604051908152602090a3565b7f94280d62000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7fe602df05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6001600160a01b0316908115612140576001600160a01b03811615612114576121b0915f52600160205260405f20906001600160a01b03165f5260205260405f2090565b55565b919060025481810180911161082f576002556001600160a01b0383168061226e5781600254036002555b6040518281525f907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602090a36002549279ffffffffffffffffffffffffffffffffffffffffffffffffffff80851161223e57506117a89293505f612a50565b7f1cb15d26000000000000000000000000000000000000000000000000000000005f52600485905260245260445ffd5b612288846001600160a01b03165f525f60205260405f2090565b8281540190556121dd565b65ffffffffffff81116122ab5765ffffffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52603060045260245260445ffd5b81156122e5570490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b60018111156103de57806001700100000000000000000000000000000000831015612445575b6123eb6123e16123d76123cd6123c36123b96123a86123f29760048a680100000000000000006123f79c1015612438575b64010000000081101561242b575b6201000081101561241e575b610100811015612411575b6010811015612404575b10156123fc575b60030260011c90565b6123b2818b6122db565b0160011c90565b6123b2818a6122db565b6123b281896122db565b6123b281886122db565b6123b281876122db565b6123b281866122db565b80936122db565b821190565b900390565b60011b61239f565b60041c9160021b91612398565b60081c9160041b9161238e565b60101c9160081b91612383565b60201c9160101b91612377565b60401c9160201b91612369565b50506123f76123f26123eb6123e16123d76123cd6123c36123b96123a861246c8a60801c90565b98506801000000000000000097506123389650505050505050565b91905b8382106124975750505090565b9091928083169080841860011c820180921161082f57845f5265ffffffffffff8260205f2001541665ffffffffffff8416105f146124d95750925b919061248a565b9392506124e59061178b565b916124d2565b91906001600160a01b038116926001600160a01b038116908482141580612635575b612519575b5050505050565b816125bf575b50508261252e575b8080612512565b6125b461259b7fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a7249361259561258f79ffffffffffffffffffffffffffffffffffffffffffffffffffff956001600160a01b03165f52600a60205260405f2090565b916128bb565b9061298f565b6040805192851683529316602082015291829190820190565b0390a25f8080612527565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff61262b61259b61261c7fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724946001600160a01b03165f52600a60205260405f2090565b612625886128bb565b9061292b565b0390a25f8061251f565b5083151561250d565b60ff811461269d5760ff811690601f8211612675576040519161266260408461174a565b6020808452838101919036833783525290565b7fb3512b0c000000000000000000000000000000000000000000000000000000005f5260045ffd5b506040516103de816126b081600661165c565b038261174a565b60ff81146126db5760ff811690601f8211612675576040519161266260408461174a565b506040516103de816126b081600761165c565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411612770579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15612765575f516001600160a01b0381161561275b57905f905f90565b505f906001905f90565b6040513d5f823e3d90fd5b5050505f9160039190565b6004111561278557565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6127bb8161277b565b806127c4575050565b6127cd8161277b565b600181036127fd577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b6128068161277b565b6002810361283a57507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b8061284660039261277b565b1461284e5750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b805482101561288e575f5260205f2001905f90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b79ffffffffffffffffffffffffffffffffffffffffffffffffffff81116128fb5779ffffffffffffffffffffffffffffffffffffffffffffffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f5260d060045260245260445ffd5b9061293543612293565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff8061295b85612025565b92169116039079ffffffffffffffffffffffffffffffffffffffffffffffffffff821161082f5761298b92612bd8565b9091565b9061299943612293565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff806129bf85612025565b92169116019079ffffffffffffffffffffffffffffffffffffffffffffffffffff821161082f5761298b92612bd8565b6129f843612293565b9079ffffffffffffffffffffffffffffffffffffffffffffffffffff80612a1f600b612025565b921691160379ffffffffffffffffffffffffffffffffffffffffffffffffffff811161082f5761298b91600b612bd8565b9091906001600160a01b03168015612ac1575b6001600160a01b036117a89316908115612aa9575b5f5260096020526001600160a01b0360405f205416905f5260096020526001600160a01b0360405f205416906124eb565b612aba612ab5846128bb565b6129ef565b5050612a78565b612aca826128bb565b92612ad443612293565b9379ffffffffffffffffffffffffffffffffffffffffffffffffffff80612afb600b612025565b92169116019079ffffffffffffffffffffffffffffffffffffffffffffffffffff821161082f576117a8946001600160a01b0392612b3a91600b612bd8565b905050935050612a63565b80546801000000000000000081101561174557612b6791600182018155612879565b612bac5781516020929092015160301b7fffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000001665ffffffffffff92909216919091179055565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b80549293928015612cce57612bef612bfa916117aa565b825f5260205f200190565b8054603081901c9365ffffffffffff91821692918116808411612ca657879303612c5f5750612c5b92509065ffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffffffff00000000000083549260301b169116179055565b9190565b915050612c5b91612c7f612c71611799565b65ffffffffffff9093168352565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff86166020830152612b45565b7f2520601d000000000000000000000000000000000000000000000000000000005f5260045ffd5b5090612d0691612cdf612c71611799565b79ffffffffffffffffffffffffffffffffffffffffffffffffffff85166020830152612b45565b5f9190560175b7a638427703f0dbe7bb9bbf987a2551717b34e79f33b5b1008d1fa01db805b8ccbb9d4d8fb16ea74ce3c29a41f1b461fbdaff4714a0d9a8eb05499746bcdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72460a0346100d957601f61134238819003918201601f19168301916001600160401b038311848410176100dd578084926060946040528339810103126100d957610047816100f1565b61005f6040610058602085016100f1565b93016100f1565b906001600160a01b031680156100ca576001600160a01b038316156100ca576001600160a01b038216156100ca576100a39261009d91608052610105565b5061017b565b506040516110d3908161020f8239608051818181610321015281816107f10152610d540152f35b63d92e233d60e01b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036100d957565b6001600160a01b0381165f9081525f5160206113225f395f51905f52602052604090205460ff16610176576001600160a01b03165f8181525f5160206113225f395f51905f5260205260408120805460ff191660011790553391905f5160206112e25f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f5160206113025f395f51905f52602052604090205460ff16610176576001600160a01b03165f8181525f5160206113025f395f51905f5260205260408120805460ff191660011790553391907ff27f9368aa1b7697e50429f83722b88f2e5db8184d5e2cbcf060f4310bbd3e7d905f5160206112e25f395f51905f529080a460019056fe6080806040526004361015610012575f80fd5b5f905f3560e01c90816301ffc9a7146109fa57508063158ef93e146109d8578063248a9ca3146109ae5780632f2ff15d1461097157806336568abe14610905578063435810101461082f57806343a3f8a1146108155780635bdf6ca1146107c55780635f15c3c9146107aa578063766718081461078d578063891624861461075357806391d14854146106fd578063a088787d146106ba578063a217fddf146106a0578063ac12ce0714610683578063b198d0281461065e578063c63a094414610550578063d3f566ae14610256578063d547741f1461020f578063debe4f1f146101d4578063df0244b1146101b6578063e0e6169c1461019b578063e4b7fb7314610178578063eced5526146101555763fa391c6414610131575f80fd5b34610152578060031936011261015257602060306002541015604051908152f35b80fd5b50346101525780600319360112610152576020604051670de0b6b3a76400008152f35b50346101525780600319360112610152576020610193610d2c565b604051908152f35b50346101525780600319360112610152576020610193610cef565b50346101525780600319360112610152576020600354604051908152f35b503461015257806003193601126101525760206040517f076eb8b875b6ea839b087c4c0c1a4661b089d3b6ee2c1ef1b9cfa7fe1066d2068152f35b50346101525760406003193601126101525761025260043561022f610a98565b9061024d610248825f525f602052600160405f20015490565b610ed3565b61100b565b5080f35b5034610451576040600319360112610451576004359073ffffffffffffffffffffffffffffffffffffffff821680920361045157335f9081527f0e25390ff9535358a5e916dfe7d38266c83601af6e112105b22df4a90bf8910160205260409020546024359060ff16156105005760ff6004541615610482576002549060308210156104825783156104d8578082036104aa5750506102f3610b05565b908115610482576003548281018091116104555760035573ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b15610451575f80916044604051809481937f40c10f190000000000000000000000000000000000000000000000000000000083528960048401528860248401525af1801561044657610433575b5060025492837f160fc195d6e53691d30d804ce190dc09471891677e43433b91a7a6131c12a59a60406103c1610d2c565b8151908782526020820152a37fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83146104065750600160209201600255604051908152f35b807f4e487b7100000000000000000000000000000000000000000000000000000000602492526011600452fd5b61043f91505f90610c81565b5f5f610390565b6040513d5f823e3d90fd5b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7f9e91c9e7000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f30413a1a000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f076eb8b875b6ea839b087c4c0c1a4661b089d3b6ee2c1ef1b9cfa7fe1066d20660245260445ffd5b3461045157602060031936011261045157335f9081527fad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb560205260409020546004359060ff161561062e5760045460ff8116610482578115610606577fc12c60abc216286ef25e34b1805a0c3dda73e4c2fd6cf360e807a7a9e73167399160017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00602093161760045580600155604051908152a1005b7feb769920000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004525f60245260445ffd5b34610451575f6003193601126104515760206040516a422ca8b0a00a42500000008152f35b34610451575f600319360112610451576020600154604051908152f35b34610451575f6003193601126104515760206040515f8152f35b34610451575f6003193601126104515760a06002546030600354916106dd610d2c565b604051938285528360208601526040850152606084015210156080820152f35b3461045157604060031936011261045157610716610a98565b6004355f525f60205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060ff60405f2054166040519015158152f35b34610451575f6003193601126104515760206040517ff27f9368aa1b7697e50429f83722b88f2e5db8184d5e2cbcf060f4310bbd3e7d8152f35b34610451575f600319360112610451576020600254604051908152f35b34610451575f60031936011261045157602060405160308152f35b34610451575f60031936011261045157602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610451575f600319360112610451576020610193610b05565b3461045157602060031936011261045157335f9081527f7e97327757452d9e3315e395bc100e7da1f2d35106fffd38f23807747090efa960205260409020546004359060ff16156108b557801561060657806001556002546040519182527fb813ffbe387d6cf6e6a6f6c5f8905f766a0f1c6cd01c67312f709356c62597bd60203393a3005b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527ff27f9368aa1b7697e50429f83722b88f2e5db8184d5e2cbcf060f4310bbd3e7d60245260445ffd5b346104515760406003193601126104515761091e610a98565b3373ffffffffffffffffffffffffffffffffffffffff821603610949576109479060043561100b565b005b7f6697b232000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461045157604060031936011261045157610947600435610990610a98565b906109a9610248825f525f602052600160405f20015490565b610f39565b346104515760206003193601126104515760206101936004355f525f602052600160405f20015490565b34610451575f60031936011261045157602060ff600454166040519015158152f35b3461045157602060031936011261045157600435907fffffffff00000000000000000000000000000000000000000000000000000000821680920361045157817f7965db0b0000000000000000000000000000000000000000000000000000000060209314908115610a6e575b5015158152f35b7f01ffc9a70000000000000000000000000000000000000000000000000000000091501483610a67565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361045157565b8115610ac5570490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b8181029291811591840414171561045557565b60ff60045416158015610c74575b610c70576002546030036030811161045557610b2d610d2c565b60018214610c6b5760015491670de0b6b3a76400008314610c605750610b51610cef565b670de0b6b3a7640000811115610c19577ffffffffffffffffffffffffffffffffffffffffffffffffff21f494c589c00008101908111610455575b6103e8811015610c1357506103e8905b670de0b6b3a7640000831115610beb577ffffffffffffffffffffffffffffffffffffffffffffffffff21f494c589c0000830192831161045557610be892610be391610af2565b610abb565b90565b91670de0b6b3a76400000391670de0b6b3a7640000831161045557610be892610be391610af2565b90610b9c565b670de0b6b3a764000003670de0b6b3a7640000811115610b8c577f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b90610be89250610abb565b905090565b5f90565b5060306002541015610b13565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610cc257604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b670de0b6b3a76400006002546001545b60308210610d0c57505090565b9091670de0b6b3a7640000610d2383600193610af2565b04920190610cff565b6040517f18160ddd0000000000000000000000000000000000000000000000000000000081527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16602082600481845afa918215610446575f92610e9e575b506020600491604051928380927f902d55a50000000000000000000000000000000000000000000000000000000082525afa908115610446575f91610e6c575b507fffffffffffffffffffffffffffffffffffffffffffbdd3574f5ff5bdb0000000810181811161045557821115610e645781036a422ca8b0a00a425000000001908111610455575b806a422ca8b0a00a4250000000115f14610e5f576a422ca8b0a00a4250000000036a422ca8b0a00a425000000081116104555790565b505f90565b50505f610e29565b90506020813d602011610e96575b81610e8760209383610c81565b8101031261045157515f610de0565b3d9150610e7a565b9091506020813d602011610ecb575b81610eba60209383610c81565b810103126104515751906020610da0565b3d9150610ead565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f20541615610f0a5750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f1461100557805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f1461100557805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d7e97327757452d9e3315e395bc100e7da1f2d35106fffd38f23807747090efa9ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x04\x89\x16\x04\x14aN\x0CWP\x80c\x07\xE6#>\x14aGAW\x80c\nj\x836\x14aF\xA6W\x80c\n\x92T\xE4\x14aBWW\x80c\x12O\xB3\xCE\x14a@\x08W\x80c\x1E\xD7\x83\x1C\x14a?\x8AW\x80c*\x8E\xA2\xE2\x14a=\x92W\x80c*\xDE8\x80\x14a;\x9EW\x80c>^<#\x14a; W\x80c?r\x86\xF4\x14a:\xA2W\x80cG\xDARd\x14a6\xEDW\x80cO\x862\xBA\x14a6\xC6W\x80cO\xDD\xB7\xA6\x14a3\x01W\x80c_\x15\xC3\xC9\x14a2\xE5W\x80c_\xF4\xC8\x99\x14a-,W\x80ca\xD0'\xB3\x14a-\x05W\x80cf\xA4}e\x14a+\x93W\x80cf\xD9\xA9\xA0\x14a*VW\x80cmmD6\x14a'\x91W\x80co|q\xEC\x14a%\x06W\x80cs_\xB4{\x14a\"\xA6W\x80cv\x02\x9Ex\x14a\x1D\xF0W\x80c\x85\"l\x81\x14a\x1DfW\x80c\x8F;\x08\xF7\x14a\x1C\x12W\x80c\x91j\x17\xC6\x14a\x1BhW\x80c\x95\xA1\x90F\x14a\x17\xEAW\x80c\xB0FO\xDC\x14a\x17@W\x80c\xB1\x98\xD0(\x14a\x17\x1AW\x80c\xB20\xC8'\x14a\x13\xA6W\x80c\xB5P\x8A\xA9\x14a\x13\x1CW\x80c\xB7*n\x9B\x14a\x10\x91W\x80c\xBAAO\xA6\x14a\x10lW\x80c\xBE\xFB\x96y\x14a\x10EW\x80c\xCB\xE7\xFB\xAC\x14a\r\0W\x80c\xCE>9\xC0\x14a\x0C\xD6W\x80c\xD9\xA1\x94p\x14a\t\x1CW\x80c\xDC\xCCW\xF1\x14a\x06\xD9W\x80c\xE2\x0C\x9Fq\x14a\x06KW\x80c\xEC\xEDU&\x14a\x06(W\x80c\xF8Q\xA4@\x14a\x06\x01W\x80c\xFAv&\xD4\x14a\x05\xDEW\x80c\xFC\x0CTj\x14a\x05\xB8Wc\xFD\xC5\n\xCA\x14a\x01\xF7W_\x80\xFD[4a\x05\xB5W` `\x03\x196\x01\x12a\x05\xB5W`\x045`\xFF\x81\x16\x80\x91\x03a\x05\x87Wa\x02%`0`\x01`\xFF\x93a[HV[\x82\x80`@Qa\x023\x81aQ\x94V[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Qa\x02\xD2\x81a\x02\xA0` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`@`$\x84\x01R`d\x83\x01\x90aO\xE8V[\x87`D\x83\x01R\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82aQ\xDDV[Q\x90jconsole.logZ\xFAP\x16\x81`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x05\xA0W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x05\x87W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x05\x8BW[P\x82\x91[\x80\x83\x10a\x04DW\x83`\x04\x83` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F\xDF\x02D\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x049W\x83\x92a\x03\xFFW[Pa\x03\xF7\x81a\x03\xFC\x93aY6V[aY\xACV[\x80\xF3[\x91P` \x82=` \x11a\x041W[\x81a\x04\x1A` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x90Q\x90a\x03\xF7a\x03\xE9V[_\x80\xFD[=\x91Pa\x04\rV[`@Q=\x85\x82>=\x90\xFD[\x90\x83`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x05cW[PP`\x1FT`#T`@Q\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91` \x91\x83\x91`\x08\x1C\x16\x81\x88\x81`D\x81\x01\x03\x92Z\xF1\x90\x81\x15a\x05XW\x85\x91a\x05&W[Pa\x05\x1D\x90`\x01\x92aRgV[\x92\x01\x91\x90a\x03\x93V[\x90P` \x81=\x82\x11a\x05PW[\x81a\x05@` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ`\x01a\x05\x10V[=\x91Pa\x053V[`@Q=\x87\x82>=\x90\xFD[\x81a\x05m\x91aQ\xDDV[a\x05xW\x83_a\x04\xA8V[\x83\x80\xFD[`@Q=\x84\x82>=\x90\xFD[P\x80\xFD[\x81a\x05\x95\x91aQ\xDDV[a\x05\x87W\x81_a\x03\x8FV[\x81a\x05\xAA\x91aQ\xDDV[a\x05\x87W\x81_a\x03GV[\x80\xFD[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `@Qg\r\xE0\xB6\xB3\xA7d\0\0\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x06\xBAWa\x06\xB6\x85a\x06\xAA\x81\x87\x03\x82aQ\xDDV[`@Q\x91\x82\x91\x82aO\xA6V[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x06\x93V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x049W\x83\x91a\x08\xE9W[P`!T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R` \x81`D\x81\x85Z\xFA\x80\x15a\x049W\x83\x90a\x08\xAAW[a\x07\x96\x91PaZ-V[`@Q\x7F\x89\x16$\x86\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x049W\x83\x91a\x08vW[P`\"T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90` \x90\x82\x90\x81\x80`D\x81\x01[\x03\x91Z\xFA\x80\x15a\x05|W\x82\x90a\x08;W[a\x03\xFC\x91PaZ-V[P` \x81=` \x11a\x08nW[\x81a\x08U` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x05\x87Wa\x08ia\x03\xFC\x91aS\x8FV[a\x081V[=\x91Pa\x08HV[\x90P` \x81=` \x11a\x08\xA2W[\x81a\x08\x91` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQa\x08 a\x07\xD3V[=\x91Pa\x08\x84V[P` \x81=` \x11a\x08\xE1W[\x81a\x08\xC4` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x08\xDDWa\x08\xD8a\x07\x96\x91aS\x8FV[a\x07\x8CV[\x82\x80\xFD[=\x91Pa\x08\xB7V[\x90P` \x81=` \x11a\t\x14W[\x81a\t\x04` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ` a\x076V[=\x91Pa\x08\xF7V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x0C\xC1W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x0C\xACW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R\x81`$\x82\x01R`\x01`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x0C\x97W[P`\x01`\x01`\xA0\x1B\x03`\"T\x16\x80\x82\x7F\xB8\x13\xFF\xBE8}l\xF6\xE6\xA6\xF6\xC5\xF8\x90_vj\x0F\x1Cl\xD0\x1Cg1/p\x93V\xC6%\x97\xBD` `@Qg\x0C}q;I\xDA\0\0\x81R\xA3sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x0C\x82W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7FCX\x10\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rg\x0C}q;I\xDA\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x0CmW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xAC\x12\xCE\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|W\x82\x91a\x0C8W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rg\x0C}q;I\xDA\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05|Wa\x0C$WP\xF3[\x81a\x0C.\x91aQ\xDDV[a\x05\xB5W\x80\xF3[P\xFD[\x91PP` \x81=` \x11a\x0CeW[\x81a\x0CT` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x81\x90Q_a\x0B\xA5V[=\x91Pa\x0CGV[\x81a\x0Cw\x91aQ\xDDV[a\x05\xB5W\x80_a\x0BWV[\x81a\x0C\x8C\x91aQ\xDDV[a\x05\xB5W\x80_a\n\xF6V[\x81a\x0C\xA1\x91aQ\xDDV[a\x05\xB5W\x80_a\n_V[\x81a\x0C\xB6\x91aQ\xDDV[a\x05\xB5W\x80_a\t\xD8V[\x81a\x0C\xCB\x91aQ\xDDV[a\x05\xB5W\x80_a\t\x90V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81`\x04\x82\x01R\x81`$\x82\x01R\x81`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x100W[PP\x7F\xC1,`\xAB\xC2\x16(n\xF2^4\xB1\x80Z\x0C=\xDAs\xE4\xC2\xFDl\xF3`\xE8\x07\xA7\xA9\xE71g9` `@Qg\r/\x13\xF7x\x9F\0\0\x81R\xA1\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x10\x1BW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x10\x06W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\x15\x8E\xF9>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x049W\x83\x91a\x0F\xBDW[P`\x04\x91a\x0E\xCF` \x92aZ-V[`@Q\x92\x83\x80\x92\x7F\xAC\x12\xCE\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|W\x82\x91a\x0F\x88W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rg\r/\x13\xF7x\x9F\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05|Wa\x0C$WP\xF3[\x91PP` \x81=` \x11a\x0F\xB5W[\x81a\x0F\xA4` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x81\x90Q_a\x0F\tV[=\x91Pa\x0F\x97V[\x90P` \x81=` \x11a\x0F\xFEW[\x81a\x0F\xD8` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x0F\xFAW`\x04\x91a\x0E\xCFa\x0F\xF1` \x93aS\x8FV[\x92PP\x91a\x0E\xC0V[PP\xFD[=\x91Pa\x0F\xCBV[\x81a\x10\x10\x91aQ\xDDV[a\x05\xB5W\x80_a\x0EsV[\x81a\x10%\x91aQ\xDDV[a\x05\xB5W\x80_a\x0E+V[\x81a\x10:\x91aQ\xDDV[a\x05\xB5W\x80_a\r\x94V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` a\x10\x87aVnV[`@Q\x90\x15\x15\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x13\x07W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x12\xF2W[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x12\xDDW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x12\xC8W[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`@Q\x80\x94\x81\x93\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a\x05|Wa\x12\x9CWP\x80\xF3[a\x12\xBD\x90` =` \x11a\x12\xC1W[a\x12\xB5\x81\x83aQ\xDDV[\x81\x01\x90aR\x1EV[P\x80\xF3[P=a\x12\xABV[\x81a\x12\xD2\x91aQ\xDDV[a\x05\xB5W\x80_a\x12BV[\x81a\x12\xE7\x91aQ\xDDV[a\x05\xB5W\x80_a\x11\xB0V[\x81a\x12\xFC\x91aQ\xDDV[a\x05\xB5W\x80_a\x11MV[\x81a\x13\x11\x91aQ\xDDV[a\x05\xB5W\x80_a\x11\x05V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x19Ta\x139\x81aRtV[\x91a\x13G`@Q\x93\x84aQ\xDDV[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\x13\x89W`@Q\x80a\x06\xB6\x87\x82aP\x80V[`\x01` \x81\x92a\x13\x98\x85aR\x8CV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x13tV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x17\x05W[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90`@Qa\x13B\x92\x83\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x16\xD8W``\x91\x83\x91a\x98z\x95\x87\x87\x859\x88\x83R` \x83\x01R`@\x82\x01R\x03\x01\x90\x84\xF0\x15a\x05|Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xDDW\x82`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x16\xC3W[PP`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`\"T\x16\x90`@Q\x91\x84\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x16\x81W\x91\x83\x91``\x93\x87\x87\x859\x82R\x87` \x83\x01R`@\x82\x01R\x03\x01\x90\x84\xF0\x15a\x05|Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xDDW\x82`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x16\xAEW[PP`\x01`\x01`\xA0\x1B\x03` T\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`@Q\x93\x80\x85\x01\x93\x85\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x16\x81W\x91\x85\x93\x91``\x95\x93\x859\x82R` \x82\x01R\x84`@\x82\x01R\x03\x01\x90\x82\xF0\x15a\x16uW\x80\xF3[`@Q\x90=\x90\x82>=\x90\xFD[`$\x87\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81a\x16\xB8\x91aQ\xDDV[a\x08\xDDW\x82_a\x16\x1BV[\x81a\x16\xCD\x91aQ\xDDV[a\x08\xDDW\x82_a\x154V[`$\x86\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[\x81a\x17\x0F\x91aQ\xDDV[a\x05\xB5W\x80_a\x14GV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `@QjB,\xA8\xB0\xA0\nBP\0\0\0\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x1CTa\x17]\x81aRtV[\x91a\x17k`@Q\x93\x84aQ\xDDV[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\x17\xADW`@Q\x80a\x06\xB6\x87\x82aP\xFDV[`\x02` `\x01\x92`@Qa\x17\xC0\x81aQ\x94V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x17\xD8\x85\x87\x01aS\xCAV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x17\x98V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x1BSW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x1B>W[P[`0\x81\x10a\x1A7WP\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x1A\"W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x9E\x91\xC9\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x1A\rW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`0`$\x84\x01RZ\xF1\x80\x15a\x05|Wa\x12\x9CWP\x80\xF3[\x81a\x1A\x17\x91aQ\xDDV[a\x05\xB5W\x80_a\x19\xA6V[\x81a\x1A,\x91aQ\xDDV[a\x05\xB5W\x80_a\x19\x14V[\x81`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x1B)W[PP`\x1FT`#T`@Q\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x92\x91` \x91\x84\x91`\x08\x1C\x16\x81\x86\x81`D\x81\x01\x03\x92Z\xF1\x91\x82\x15a\x049W`\x01\x92a\x1B\x0BW[P\x01a\x18\xA8V[a\x1B\"\x90` =\x81\x11a\x12\xC1Wa\x12\xB5\x81\x83aQ\xDDV[P_a\x1B\x04V[\x81a\x1B3\x91aQ\xDDV[a\x05\x87W\x81_a\x1A\x9AV[\x81a\x1BH\x91aQ\xDDV[a\x05\xB5W\x80_a\x18\xA6V[\x81a\x1B]\x91aQ\xDDV[a\x05\xB5W\x80_a\x18^V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x1DTa\x1B\x85\x81aRtV[\x91a\x1B\x93`@Q\x93\x84aQ\xDDV[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x1B\xD5W`@Q\x80a\x06\xB6\x87\x82aP\xFDV[`\x02` `\x01\x92`@Qa\x1B\xE8\x81aQ\x94V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x1C\0\x85\x87\x01aS\xCAV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x1B\xC0V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`$T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x1DQW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xF4\x84H\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa\x1D<W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x0C$WP\xF3[\x81a\x1DF\x91aQ\xDDV[a\x05\xB5W\x80_a\x1C\xF2V[\x81a\x1D[\x91aQ\xDDV[a\x05\xB5W\x80_a\x1C\x86V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x1ATa\x1D\x83\x81aRtV[\x91a\x1D\x91`@Q\x93\x84aQ\xDDV[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\x1D\xD3W`@Q\x80a\x06\xB6\x87\x82aP\x80V[`\x01` \x81\x92a\x1D\xE2\x85aR\x8CV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x1D\xBEV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x81`@Q\x7F[\xDFl\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x05|W\x82\x91a\"dW[P`\x01`\x01`\xA0\x1B\x03` T\x16\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\xDDW`@Q\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05|Wa\"OW[PP`@Q\x7F_\x15\xC3\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x049W\x83\x90a\"\x1BW[a\x1F#\x91PaX\xBFV[`@Q\x7F\xB1\x98\xD0(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x049W\x83\x90a!\xE7W[a\x1Fi\x91PaWGV[\x81`@Q\x7F\xEC\xEDU&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x05|W\x82\x91a!\xB2W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x05|Wa!\x9DW[PP`@Q\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x049W\x83\x90a!iW[a l\x91PaXIV[`@Q\x7F\xDF\x02D\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x049W\x83\x91a!6W[P`\x04\x91a \xB8` \x92aXIV[`@Q\x92\x83\x80\x92\x7F\x15\x8E\xF9>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05|W\x82\x90a \xFBW[a\x03\xFC\x91PaZ\x9FV[P` \x81=` \x11a!.W[\x81a!\x15` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x05\x87Wa!)a\x03\xFC\x91aS\x8FV[a \xF1V[=\x91Pa!\x08V[\x90P` \x81=` \x11a!aW[\x81a!Q` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ`\x04a \xA9V[=\x91Pa!DV[P` \x81=` \x11a!\x95W[\x81a!\x83` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa l\x90Qa bV[=\x91Pa!vV[\x81a!\xA7\x91aQ\xDDV[a\x05\x87W\x81_a $V[\x91PP` \x81=` \x11a!\xDFW[\x81a!\xCE` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x82\x90Q_a\x1F\xA7V[=\x91Pa!\xC1V[P` \x81=` \x11a\"\x13W[\x81a\"\x01` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa\x1Fi\x90Qa\x1F_V[=\x91Pa!\xF4V[P` \x81=` \x11a\"GW[\x81a\"5` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa\x1F#\x90Qa\x1F\x19V[=\x91Pa\"(V[\x81a\"Y\x91aQ\xDDV[a\x05\x87W\x81_a\x1E\xDBV[\x90P` \x81=` \x11a\"\x9EW[\x81a\"\x7F` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x05\x87WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x05\x87W_a\x1ENV[=\x91Pa\"rV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa$\xF1W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa$\xDCW[P`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa$\xC7W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xEBv\x99 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa$\xB2W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7FCX\x10\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x0C$WP\xF3[\x81a$\xBC\x91aQ\xDDV[a\x05\xB5W\x80_a$WV[\x81a$\xD1\x91aQ\xDDV[a\x05\xB5W\x80_a#\xC5V[\x81a$\xE6\x91aQ\xDDV[a\x05\xB5W\x80_a#bV[\x81a$\xFB\x91aQ\xDDV[a\x05\xB5W\x80_a#\x1AV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa'|W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa'gW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7FC\xA3\xF8\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|W\x82\x91a'2W[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0F\xFAW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x049W\x83\x91a'\x1DW[PP` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x95\x86\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x90\x81\x15a\x049W\x83\x91a&\xE7W[a\x03\xFC\x92PaY6V[\x90P` \x82=` \x11a'\x15W[\x81a'\x02` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa\x03\xFC\x91Q\x90a&\xDDV[=\x91Pa&\xF5V[\x81a''\x91aQ\xDDV[a\x0C5W\x81_a&vV[\x91PP` \x81=` \x11a'_W[\x81a'N` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x81\x90Q_a&\x10V[=\x91Pa'AV[\x81a'q\x91aQ\xDDV[a\x05\xB5W\x80_a%\xC2V[\x81a'\x86\x91aQ\xDDV[a\x05\xB5W\x80_a%zV[P4a\x05\xB5W` `\x03\x196\x01\x12a\x05\xB5W\x80a'\xBAg\r\xE0\xB6\xB3\xA7c\xFF\xFF`\x01`\x045a[HV[\x81\x80`@Qa'\xC8\x81aQ\x94V[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Qa(5\x81a\x02\xA0` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`@`$\x84\x01R`d\x83\x01\x90aO\xE8V[Q\x90jconsole.logZ\xFAP`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0F\xFAW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x049W\x83\x91a*AW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x0F\xFAW\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92c1\x8E\x82Q`\xE2\x1B\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa*,W[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa*\x17W[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x90\x81\x15a\x05|W\x82\x91a)\xDAW[PjB,\xA8\xB0\xA0\nBP\0\0\0\x81a)\xD3a\x03\xFC\x93\x15\x15aZ-V[\x11\x15aZ-V[\x90P` \x81=` \x11a*\x0FW[\x81a)\xF5` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQjB,\xA8\xB0\xA0\nBP\0\0\0a)\xB7V[=\x91Pa)\xE8V[\x81a*!\x91aQ\xDDV[a\x05\xB5W\x80_a)QV[\x81a*6\x91aQ\xDDV[a\x05\xB5W\x80_a(\xEEV[\x81a*K\x91aQ\xDDV[a\x0C5W\x81_a(\xABV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x1BTa*s\x81aRtV[a*\x80`@Q\x91\x82aQ\xDDV[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a+XW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a*\xEDWPPPP\x03\x90\xF3[\x91\x93` a+H\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a+8\x83Q`@\x84R`@\x84\x01\x90aO\xE8V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01RaP+V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a*\xDEV[`\x02` `\x01\x92`@Qa+k\x81aQ\x94V[a+t\x86aR\x8CV[\x81Ra+\x81\x85\x87\x01aS\xCAV[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a*\xB0V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa,\xF0W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xEBv\x99 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa,\xDBW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x0C$WP\xF3[\x81a,\xE5\x91aQ\xDDV[a\x05\xB5W\x80_a,\x99V[\x81a,\xFA\x91aQ\xDDV[a\x05\xB5W\x80_a,\x07V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa2\xD0W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa2\xBBW[P[`/\x81\x10a1\xB4WP\x80`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xE4\xB7\xFBs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|W\x82\x91a1\x7FW[P`\x01`\x01`\xA0\x1B\x03` T\x16\x90` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x95\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a\x049W\x83\x92a1HW[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a1CW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a0\xEDW\x84\x91a1.W[PP` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x96\x87\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`/`$\x84\x01RZ\xF1\x92\x83\x15a0\xEDW\x84\x93a0\xF8W[Pa/w\x90\x83aY6V[`\x01`\x01`\xA0\x1B\x03` T\x16\x90` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x95\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a0\xEDW\x84\x92a0\xB7W[Pa/\xE7\x92a/\xE1\x91aRgV[\x90aY6V[`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x049W\x83\x91a0\x84W[P`\x04\x91a0B` \x92aX\xBFV[`@Q\x92\x83\x80\x92\x7F\xFA9\x1Cd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05|W\x82\x90a\x08;Wa\x03\xFC\x91PaZ-V[\x90P` \x81=` \x11a0\xAFW[\x81a0\x9F` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ`\x04a03V[=\x91Pa0\x92V[\x90\x91P` \x81=` \x11a0\xE5W[\x81a0\xD3` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ\x90a/\xE7a/\xD3V[=\x91Pa0\xC6V[`@Q=\x86\x82>=\x90\xFD[\x90\x92P` \x81=` \x11a1&W[\x81a1\x14` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ\x91a/wa/lV[=\x91Pa1\x07V[\x81a18\x91aQ\xDDV[a\x0F\xFAW\x82_a/\x04V[PPP\xFD[\x92P\x90P` \x82=` \x11a1wW[\x81a1e` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x82\x91Q\x90_a.\x9EV[=\x91Pa1XV[\x91PP` \x81=` \x11a1\xACW[\x81a1\x9B` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x81\x90Q_a.AV[=\x91Pa1\x8EV[\x81`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa2\xA6W[PP`\x1FT`#T`@Q\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x92\x91` \x91\x84\x91`\x08\x1C\x16\x81\x86\x81`D\x81\x01\x03\x92Z\xF1\x91\x82\x15a\x049W`\x01\x92a2\x88W[P\x01a-\xEAV[a2\x9F\x90` =\x81\x11a\x12\xC1Wa\x12\xB5\x81\x83aQ\xDDV[P_a2\x81V[\x81a2\xB0\x91aQ\xDDV[a\x05\x87W\x81_a2\x17V[\x81a2\xC5\x91aQ\xDDV[a\x05\xB5W\x80_a-\xE8V[\x81a2\xDA\x91aQ\xDDV[a\x05\xB5W\x80_a-\xA0V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `@Q`0\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa6\xB1W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa6\x9CW[PP`\x04`\xA0`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xA0\x88x}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|Wa4A\x91\x83\x84\x85\x90\x86\x92\x87\x94a6aW[a4<\x94\x95Pa47\x92\x91a4-a42\x92aXIV[aX\xBFV[aXIV[aWGV[aZ\x9FV[\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa6LW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x90\x81\x15a\x05|W\x82\x91a6\x1AW[P`\x04`\xA0`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xA0\x88x}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x049W\x83\x84\x85\x91\x86\x94\x87\x96a5\xD1W[P\x91a5q\x84\x92a4-a5v\x95aW\xD2V[aY6V[jB,\xA8\xB0\xA0\nBP\0\0\0\x03\x90jB,\xA8\xB0\xA0\nBP\0\0\0\x82\x11a5\xA4Wa\x03\xFC\x92\x91a4<\x91aY6V[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[a4-\x96Pa5q\x95Pa5v\x93P\x84\x92Pa6\x05\x91P`\xA0=`\xA0\x11a6\x13W[a5\xFD\x81\x83aQ\xDDV[\x81\x01\x90aS\x9CV[\x98P\x96\x90\x94P\x90\x92Pa5^V[P=a5\xF3V[\x90P` \x81=` \x11a6DW[\x81a65` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ_a5\nV[=\x91Pa6(V[\x81a6V\x91aQ\xDDV[a\x05\xB5W\x80_a4\xA4V[PPPPPa4<a42a47a6\x8Aa4-\x94`\xA0=`\xA0\x11a6\x13Wa5\xFD\x81\x83aQ\xDDV[\x93\x97P\x92\x95P\x91\x93P\x90\x91P\x84a4\x16V[\x81a6\xA6\x91aQ\xDDV[a\x05\xB5W\x80_a3\xBDV[\x81a6\xBB\x91aQ\xDDV[a\x05\xB5W\x80_a3uV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x90\x81R\xF3[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa:\x8DW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r\x99\xA8\xCE\xC7\xE2\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa:xW[P\x81[`0\x81\x10a9QWPa7\xBE\x81aY\xACV[`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`@Q\x90\x7F\xDF\x02D\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x86Z\xFA\x80\x15a0\xEDW\x84\x90a9\x1DW[a8\x15\x92PaY6V[`@Q\x7F\xFA9\x1Cd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x049W\x83\x91a8\xD8W[P`\x04\x91a8a` \x92aZ-V[`@Q\x92\x83\x80\x92\x7F\xE4\xB7\xFBs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05|W\x82\x90a8\xA4W[a\x03\xFC\x91PaXIV[P` \x81=` \x11a8\xD0W[\x81a8\xBE` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa\x03\xFC\x90Qa8\x9AV[=\x91Pa8\xB1V[\x90P` \x81=` \x11a9\x15W[\x81a8\xF3` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x08\xDDW`\x04\x91a8aa9\x0C` \x93aS\x8FV[\x92PP\x91a8RV[=\x91Pa8\xE6V[P` \x82=` \x11a9IW[\x81a97` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa8\x15\x91Qa8\x0BV[=\x91Pa9*V[\x90\x82`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa:cW[PP`\x1FT`#T`@Q\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91` \x91\x83\x91`\x08\x1C\x16\x81\x87\x81`D\x81\x01\x03\x92Z\xF1\x90\x81\x15a0\xEDW\x84\x91a:1W[Pa:*\x90`\x01\x92aRgV[\x91\x01a7\xACV[\x90P` \x81=\x82\x11a:[W[\x81a:K` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ`\x01a:\x1DV[=\x91Pa:>V[\x81a:m\x91aQ\xDDV[a\x08\xDDW\x82_a9\xB5V[\x81a:\x82\x91aQ\xDDV[a\x05\xB5W\x80_a7\xA9V[\x81a:\x97\x91aQ\xDDV[a\x05\xB5W\x80_a7aV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a;\x01Wa\x06\xB6\x85a\x06\xAA\x81\x87\x03\x82aQ\xDDV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a:\xEAV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a;\x7FWa\x06\xB6\x85a\x06\xAA\x81\x87\x03\x82aQ\xDDV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a;hV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x1ETa;\xBB\x81aRtV[a;\xC8`@Q\x91\x82aQ\xDDV[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a=\tW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a<4W\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a<\xC0WPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a<'V[\x90\x91\x92\x93\x94` \x80a<\xFC\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89QaO\xE8V[\x97\x01\x95\x01\x93\x92\x91\x01a<\x9CV[`@Qa=\x15\x81aQ\x94V[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta=1\x81aRtV[\x91a=?`@Q\x93\x84aQ\xDDV[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a=uWPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a;\xF8V[`\x01` \x81\x92a=\x84\x86aR\x8CV[\x81R\x01\x93\x01\x91\x01\x90\x91a=OV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|Wa?uW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x90`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa?`W[P`\x04\x90` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F\xE0\xE6\x16\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a?SW\x81\x92a?\x1FW[Pg\r/\x13\xF7x\x9F\0\0`\x01[`0\x81\x10a>\xBCWPa\x03\xFC\x91\x92aY6V[\x90g\r/\x13\xF7x\x9F\0\0\x81\x02\x90\x80\x82\x04g\r/\x13\xF7x\x9F\0\0\x14\x90\x15\x17\x15a>\xF2Wg\r\xE0\xB6\xB3\xA7d\0\0`\x01\x91\x04\x91\x01a>\xA9V[`$\x83\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90\x91P` \x81=` \x11a?KW[\x81a?;` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ\x90_a>\x9CV[=\x91Pa?.V[P`@Q\x90=\x90\x82>=\x90\xFD[a?k\x82\x80\x92aQ\xDDV[a\x05\xB5W_a>MV[\x81a?\x7F\x91aQ\xDDV[a\x05\xB5W\x80_a>\x06V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a?\xE9Wa\x06\xB6\x85a\x06\xAA\x81\x87\x03\x82aQ\xDDV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a?\xD2V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|WaBBW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|WaB-W[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|WaB\x18W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x9E\x91\xC9\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|WaB\x03W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\x0C}q;I\xDA\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|Wa\x0C$WP\xF3[\x81aB\r\x91aQ\xDDV[a\x05\xB5W\x80_aA\xB9V[\x81aB\"\x91aQ\xDDV[a\x05\xB5W\x80_aA'V[\x81aB7\x91aQ\xDDV[a\x05\xB5W\x80_a@\xC4V[\x81aBL\x91aQ\xDDV[a\x05\xB5W\x80_a@|V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x91a;q\x80\x84\x01\x90\x84\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x16\xD8W\x91\x84\x93\x91aB\xC5\x93a]\t\x869`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[\x03\x90\x82\xF0\x80\x15a?SW`\x01`\x01`\xA0\x1B\x03\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U`\x01`\x01`\xA0\x1B\x03`!T\x16\x90`\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x92a\x13B\x92\x83\x85\x01\x93\x85\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x16\x81W\x91\x85\x93\x91``\x95\x93a\x98z\x869\x83R` \x83\x01R`@\x82\x01R\x03\x01\x90\x82\xF0\x80\x15a?SW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU\x80`\x04` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x92\x83\x80\x92\x7F\x8D3C\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|W\x82\x91aFqW[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0F\xFAW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x049W\x83\x91aF\\W[PP`\x01`\x01`\xA0\x1B\x03` T\x16\x90`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x82;\x15a1CW`@Q\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05|WaFGW[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xDE\xBEO\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05|W\x82\x91aF\x12W[P`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0F\xFAW`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x049W\x83\x91aE\xFDW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`\x01`\x01`\xA0\x1B\x03`!T\x16\x82;\x15a1CW`@Q\x7F//\xF1]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x05|Wa\x0C$WP\xF3[\x81aF\x07\x91aQ\xDDV[a\x0C5W\x81_aE\x81V[\x91PP` \x81=` \x11aF?W[\x81aF.` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x81\x90Q_aE\x1BV[=\x91PaF!V[\x81aFQ\x91aQ\xDDV[a\x05\xB5W\x80_aD\xCDV[\x81aFf\x91aQ\xDDV[a\x0C5W\x81_aDSV[\x91PP` \x81=` \x11aF\x9EW[\x81aF\x8D` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x81\x90Q_aC\xEDV[=\x91PaF\x80V[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xE4\xB7\xFBs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05|W\x82\x90aG\rW[a\x03\xFC\x91PaWGV[P` \x81=` \x11aG9W[\x81aG'` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa\x03\xFC\x90QaG\x03V[=\x91PaG\x1AV[P4a\x05\xB5W\x80`\x03\x196\x01\x12a\x05\xB5W\x80`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0C5W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|WaM\xF7W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x0C5W\x81\x80\x91`$`@Q\x80\x94\x81\x93c1\x8E\x82Q`\xE2\x1B\x83Rg\r/\x13\xF7x\x9F\0\0`\x04\x84\x01RZ\xF1\x80\x15a\x05|WaM\xE2W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x7F\xE4\xB7\xFBs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\x049W\x83\x92aM\xADW[P` `\x04\x91aH[\x84aWGV[`@Q\x92\x83\x80\x92\x7F\xE0\xE6\x16\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x049W\x83\x91aM{W[Pf\xB1\xA2\xBC.\xC5\0\0\x82\x02\x82\x81\x04f\xB1\xA2\xBC.\xC5\0\0\x14\x83\x15\x17\x15a5\xA4W\x81g\r\xE0\xB6\xB3\xA7d\0\0\x03\x91g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11aMNWg\r\xE0\xB6\xB3\xA7d\0\0\x14aM!W\x04\x90`\x01`\x01`\xA0\x1B\x03` T\x16\x90` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x95\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a0\xEDW\x84\x92aL\xEDW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05xW`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R\x84`$\x82\x01R\x84`D\x82\x01R`\x01`d\x82\x01R\x84\x80\x82`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x91\x82\x15a?SW\x85\x92aL\xD4W[PPaI\xCE\x91aR-V[\x83\x7F\x16\x0F\xC1\x95\xD6\xE56\x91\xD3\r\x80L\xE1\x90\xDC\tG\x18\x91g~CC;\x91\xA7\xA6\x13\x1C\x12\xA5\x9A`@`\x01`\x01`\xA0\x1B\x03`#T\x16\x93\x81Q\x90\x87\x82R` \x82\x01R\xA3\x82`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x87W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|WaL\xBFW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a0\xEDW\x83\x90\x85\x90aL\x89W[aJ\xDF\x92PaY6V[`\x01`\x01`\xA0\x1B\x03` T\x16\x90` `\x01`\x01`\xA0\x1B\x03`#T\x16`$`@Q\x80\x95\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a0\xEDW\x83\x92\x85\x92aLPW[PaKK\x92a/\xE1\x91aRgV[`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`@Q\x7Fvg\x18\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a0\xEDW\x84\x91aL\x1DW[P`\x04\x92aK\xA7` \x92aW\xD2V[`@Q\x93\x84\x80\x92\x7F\xDF\x02D\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x049W\x83\x90aK\xE9Wa\x03\xFC\x92PaY6V[P` \x82=` \x11aL\x15W[\x81aL\x03` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-Wa\x03\xFC\x91Qa&\xDDV[=\x91PaK\xF6V[\x90P` \x81=` \x11aLHW[\x81aL8` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ`\x04aK\x98V[=\x91PaL+V[\x92P\x90P` \x82=` \x11aL\x81W[\x81aLm` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x90Q\x82\x91aKKaK=V[=\x91PaL`V[PP` \x81=` \x11aL\xB7W[\x81aL\xA4` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-W\x82aJ\xDF\x91QaJ\xD5V[=\x91PaL\x97V[\x81aL\xC9\x91aQ\xDDV[a\x08\xDDW\x82_aJnV[\x81\x92P\x90aL\xE1\x91aQ\xDDV[a\x05xW\x82\x84_aI\xC3V[\x90\x91P` \x81=` \x11aM\x19W[\x81aM\t` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ\x90_aI;V[=\x91PaL\xFCV[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x12`\x04R\xFD[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11aM\xA5W[\x81aM\x96` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ_aH\x95V[=\x91PaM\x89V[\x90\x91P` \x81=` \x11aM\xDAW[\x81aM\xC9` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ\x90` aHLV[=\x91PaM\xBCV[\x81aM\xEC\x91aQ\xDDV[a\x05\xB5W\x80_aG\xFDV[\x81aN\x01\x91aQ\xDDV[a\x05\xB5W\x80_aG\xB5V[\x90P4a\x04-W_`\x03\x196\x01\x12a\x04-W`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-Wc\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15aO\x9BWaO\x88W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xB5W\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x9E\x91\xC9\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x05|WaOsW[P` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`D`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x94\x85\x93\x84\x92\x7F\xD3\xF5f\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a\x05|Wa\x12\x9CWP\x80\xF3[\x81aO}\x91aQ\xDDV[a\x05\xB5W\x80_aO\rV[aO\x94\x91P_\x90aQ\xDDV[__aN|V[`@Q=_\x82>=\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10aO\xC9WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aO\xBCV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10aPHWPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aP;V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aP\xB2WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aP\xEE\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89QaO\xE8V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aP\xA3V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aQ/WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aQ\x85\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90aP+V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aQ V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aQ\xB0W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aQ\xB0W`@RV[\x90\x81` \x91\x03\x12a\x04-WQ\x90V[\x91\x90\x82\x03\x91\x82\x11aR:WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x91\x90\x82\x01\x80\x92\x11aR:WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11aQ\xB0W`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15aS\x85W[` \x85\x10\x84\x14aSXW\x84\x87R\x86\x93\x90\x81\x15aS\x18WP`\x01\x14aR\xD4W[PaR\xD2\x92P\x03\x83aQ\xDDV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10aR\xFCWPP\x90` aR\xD2\x92\x82\x01\x01_aR\xC5V[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92aR\xE3V[` \x93PaR\xD2\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_aR\xC5V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93aR\xA6V[Q\x90\x81\x15\x15\x82\x03a\x04-WV[\x90\x81`\xA0\x91\x03\x12a\x04-W\x80Q\x91` \x82\x01Q\x91`@\x81\x01Q\x91aS\xC7`\x80``\x84\x01Q\x93\x01aS\x8FV[\x90V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10aU\xE1WaR\xD2\x94T\x91\x81\x81\x10aU\xABW[\x81\x81\x10aUuW[\x81\x81\x10aU?W[\x81\x81\x10aU\tW[\x81\x81\x10aT\xD3W[\x81\x81\x10aT\x9DW[\x81\x81\x10aThW[\x10aT;W[P\x03\x83aQ\xDDV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_aT3V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01aT-V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01aT%V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01aT\x1DV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01aT\x15V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01aT\rV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01aT\x05V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01aS\xFDV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91aS\xE5V[`\x08T`\xFF\x16\x80\x15aV}W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15aO\x9BW_\x91aW\x15W[P\x15\x15\x90V[\x90P` \x81=` \x11aW?W[\x81aW0` \x93\x83aQ\xDDV[\x81\x01\x03\x12a\x04-WQ_aW\x0FV[=\x91PaW#V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01RjB,\xA8\xB0\xA0\nBP\0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[_aR\xD2\x91aQ\xDDV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x01`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\x84f\xF4\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01RjB,\xA8\xB0\xA0\nBP\0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04-W`@Q\x90\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aO\x9BWaW\xC8WPV[\x81\x15a[\x1BW\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x90\x83\x83\x11a\\\x84W\x82\x81\x10\x91\x82\x15\x80a\\zW[a\\rWa[k\x84\x86aR-V[\x92`\x01\x84\x01\x80\x94\x11aR:W`\x03\x83\x11\x15\x80a\\iW[a\\ZW`\x03\x19\x83\x10\x15\x80a\\PW[a\\?W\x85\x83\x11\x15a[\xF6WPP\x90a[\xAE\x84a[\xB3\x93aR-V[a[\x11V[\x90\x81\x15a[\xF1Wa[\xC4\x92PaRgV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11aR:W\x90V[PP\x90V[\x95\x94\x92\x91\x90\x95a\\\x07W[PPPPV[\x83\x94\x95Pa[\xAE\x90a\\\x19\x93\x94aR-V[\x90\x81\x15a[\xF1Wa\\*\x92PaR-V[`\x01\x81\x01\x80\x91\x11aR:W\x90_\x80\x80\x80a\\\x01V[PP\x90PaS\xC7\x92\x91P\x19\x90aR-V[P\x82\x19\x84\x11a[\x92V[PP\x91\x90PaS\xC7\x92PaRgV[P\x82\x84\x11a[\x82V[P\x92PPP\x90V[P\x84\x82\x11\x15a[]V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R\xFD\xFEa\x01\x80`@R4a\0}Wa\0\x1Ba\0\x15a\0\xE2V[\x90a\x01\x98V[`@Qa-\x0B\x90\x81a\x0E\x06\x829`\x80Q\x81a\x1C\xDB\x01R`\xA0Q\x81a\x1D\x98\x01R`\xC0Q\x81a\x1C\xAC\x01R`\xE0Q\x81a\x1D*\x01Ra\x01\0Q\x81a\x1DP\x01Ra\x01 Q\x81a\rb\x01Ra\x01@Q\x81a\r\x8B\x01Ra\x01`Q\x81\x81\x81a\x0C@\x01Ra\x0C\x89\x01R\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\0\xB8W`@RV[a\0\x81V[`@Q\x90a\0\xCC`@\x83a\0\x95V[V[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0}WV[a;q\x90`@\x828\x03\x92\x83\x82Q\x94\x85\x92a\0\xFC\x82\x85a\0\x95V[\x839\x81\x01\x03\x12a\0}Wa\x01\x1B` a\x01\x14\x84a\0\xCEV[\x93\x01a\0\xCEV[\x90V[`@Q\x90a\x01-`@\x83a\0\x95V[`\x04\x82RV[`@Q\x90a\x01B`@\x83a\0\x95V[`\x01\x82RV[`@Q\x90a\x01W`@\x83a\0\x95V[`\t\x82RhSyndicate`\xB8\x1B` \x83\x01RV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90b\x9E4\0\x82\x01\x80\x92\x11a\x01\x93WV[a\x01oV[\x90a\x01\xA1a\x01HV[a\x01\xA9a\x01HV[\x90a\x01\xB2a\x01\x1EV[\x90c\x14\xD6S\x91`\xE2\x1B` \x83\x01Ra\x01\xC8a\x013V[`1`\xF8\x1B` \x82\x01\x90\x81R\x84Q\x90\x94\x91\x93\x91`\x01`\x01`@\x1B\x03\x82\x11a\0\xB8Wa\x01\xFD\x82a\x01\xF8`\x03Ta\x03^V[a\x03\x96V[` \x90`\x1F\x83\x11`\x01\x14a\x02\xD7W\x91\x80a\x021\x92a\x029\x95\x94_\x92a\x02\xCCW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[`\x03Ua\x045V[a\x02B\x81a\x06\x84V[a\x01 Ra\x02O\x82a\x07vV[a\x01@R` \x81Q\x91\x01 `\xE0RQ\x90 a\x01\0RF`\xA0Ra\x02pa\x08hV[`\x80R0`\xC0R`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x02\xBDW`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x02\xBDWa\x02\xB7a\0\xCC\x92a\x02\xA5Ba\x01\x83V[a\x01`Ra\x02\xB2_`\x0CUV[a\x05\x0EV[Pa\x05\x97V[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[\x01Q\x90P_\x80a\x02\x1DV[`\x03_R`\x1F\x19\x83\x16\x91\x90\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x92_[\x81\x81\x10a\x03FWP\x91`\x01\x93\x91\x85a\x029\x97\x96\x94\x10a\x03.W[PPP\x81\x1B\x01`\x03Ua\x045V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x03 V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x03\x06V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x03\x8CW[` \x83\x10\x14a\x03xWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x03mV[`\x1F\x81\x11a\x03\xA2WPPV[`\x03_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x03\xDCW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x03\xD1WPPV[_\x81U`\x01\x01a\x03\xC6V[\x90\x91P\x81\x90a\x03\xBDV[`\x1F\x82\x11a\x03\xF3WPPPV[_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x04+W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x04 WPPV[_\x81U`\x01\x01a\x04\x15V[\x90\x91P\x81\x90a\x04\x0CV[\x80Q\x90\x91\x90`\x01`\x01`@\x1B\x03\x81\x11a\0\xB8Wa\x04^\x81a\x04W`\x04Ta\x03^V[`\x04a\x03\xE6V[` \x92`\x1F\x82\x11`\x01\x14a\x04\x92Wa\x04\x8D\x92\x93\x82\x91_\x92a\x02\xCCWPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[`\x04UV[`\x04_R`\x1F\x19\x82\x16\x93\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x91_[\x86\x81\x10a\x04\xF6WP\x83`\x01\x95\x96\x10a\x04\xDEW[PPP\x81\x1B\x01`\x04UV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x04\xD3V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x04\xC0V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a;1_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x05\x92W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a;1_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x15a\x06qW`\x02Tk\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0\x81\x01\x80\x91\x11a\x01\x93W`\x02U`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x81\x90R`@\x90 k\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0\x81T\x01\x90U_\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`@Q\x80a\x06-k\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA3`\x02T`\x01`\x01`\xD0\x1B\x03\x90\x81\x81\x11a\x06\\WPPk\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0a\0\xCC\x91_a\t1V[c\x0EX\xAE\x93`\xE1\x1B_R`\x04R`$R`D_\xFD[c\xECD/\x05`\xE0\x1B_R_`\x04R`$_\xFD[\x90\x81Q` \x81\x10_\x14a\x06\x9CWP\x90a\x01\x1B\x90a\x08\xC6V[`\x01`\x01`@\x1B\x03\x81\x11a\0\xB8Wa\x06\xC0\x81a\x06\xB9`\x06Ta\x03^V[`\x06a\x03\xE6V[` \x92`\x1F\x82\x11`\x01\x14a\x06\xF7Wa\x06\xEF\x92\x93\x82\x91_\x92a\x02\xCCWPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[`\x06U`\xFF\x90V[`\x06_R`\x1F\x19\x82\x16\x93\x7F\xF6R\"#\x13\xE2\x84YR\x8D\x92\x0Be\x11\\\x16\xC0O>\xFC\x82\xAA\xED\xC9{\xE5\x9F?7|\r?\x91_[\x86\x81\x10a\x07^WP\x83`\x01\x95\x96\x10a\x07FW[PPP\x81\x1B\x01`\x06U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x078V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x07%V[\x90\x81Q` \x81\x10_\x14a\x07\x8EWP\x90a\x01\x1B\x90a\x08\xC6V[`\x01`\x01`@\x1B\x03\x81\x11a\0\xB8Wa\x07\xB2\x81a\x07\xAB`\x07Ta\x03^V[`\x07a\x03\xE6V[` \x92`\x1F\x82\x11`\x01\x14a\x07\xE9Wa\x07\xE1\x92\x93\x82\x91_\x92a\x02\xCCWPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[`\x07U`\xFF\x90V[`\x07_R`\x1F\x19\x82\x16\x93\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88\x91_[\x86\x81\x10a\x08PWP\x83`\x01\x95\x96\x10a\x088W[PPP\x81\x1B\x01`\x07U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x08*V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x08\x17V[`\xE0Qa\x01\0Q`@Q\x90` \x82\x01\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x84R`@\x83\x01R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x08\xC0`\xC0\x82a\0\x95V[Q\x90 \x90V[`\x1F\x81Q\x11a\x08\xF1W` \x81Q\x91\x01Q` \x82\x10a\x08\xE2W\x17\x90V[_\x19\x82` \x03`\x03\x1B\x1B\x16\x17\x90V[`D` \x91`@Q\x92\x83\x91c0Z'\xA9`\xE0\x1B\x83R\x81`\x04\x84\x01R\x80Q\x91\x82\x91\x82`$\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xFD[\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\t\x98W[a\0\xCC\x92`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\t\x80W[_\x90\x81R`\t` R`@\x80\x82 T\x92\x82R\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16a\x0BbV[a\t\x91a\t\x8C\x84a\n3V[a\ndV[PPa\tYV[a\t\xA1\x82a\n3V[\x92e\xFF\xFF\xFF\xFF\xFF\xFFC\x11a\n\x1BW`\x0BT\x80a\t\xE5WPa\t\xDBa\t\xCBa\0\xCC\x95_[`\x01a\r\xA9V[e\xFF\xFF\xFF\xFF\xFF\xFFC\x16`\x0Ba\x0C\xD3V[\x90PP\x92Pa\tDV[\x93\x84_\x19\x81\x01\x11a\x01\x93W`\x0B_R_Q` a;\x11_9_Q\x90_R\x90\x94\x01Ta\0\xCC\x94a\t\xDB\x91a\t\xCB\x91\x90`0\x1Ca\t\xC4V[c\x06\xDF\xCCe`\xE4\x1B_R`0`\x04RC`$R`D_\xFD[`\x01`\x01`\xD0\x1B\x03\x81\x11a\nMW`\x01`\x01`\xD0\x1B\x03\x16\x90V[c\x06\xDF\xCCe`\xE4\x1B_R`\xD0`\x04R`$R`D_\xFD[e\xFF\xFF\xFF\xFF\xFF\xFFC\x11a\n\x1BW`\x0BT\x80a\n\x8EWPa\t\xCBa\n\x8A\x91_[`\x02a\r\xA9V[\x90\x91V[\x80_\x19\x81\x01\x11a\x01\x93W`\x0B_R_Q` a;\x11_9_Q\x90_R\x01Ta\n\x8A\x91a\t\xCB\x91`0\x1Ca\n\x83V[e\xFF\xFF\xFF\xFF\xFF\xFFC\x11a\n\x1BW\x80T\x80a\n\xF0WPa\n\xE0a\n\x8A\x92_`\x02a\r\xA9V[\x90e\xFF\xFF\xFF\xFF\xFF\xFFC\x16\x90a\x0C\xD3V[\x80_\x19\x81\x01\x11a\x01\x93W_\x82\x81R` \x90 \x01_\x19\x01Ta\n\x8A\x92a\n\xE0\x91`0\x1Ca\n\x83V[e\xFF\xFF\xFF\xFF\xFF\xFFC\x11a\n\x1BW\x80T\x80a\x0B;WPa\n\xE0a\n\x8A\x92_`\x01a\r\xA9V[\x80_\x19\x81\x01\x11a\x01\x93W_\x82\x81R` \x90 \x01_\x19\x01Ta\n\x8A\x92a\n\xE0\x91`0\x1Ca\t\xC4V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x93\x92\x91\x90\x81\x16\x90\x81\x85\x14\x15\x80a\x0CUW[a\x0B\x8AW[PPPPPV[\x81a\x0B\xFBW[PP\x82a\x0B\x9FW[\x80\x80a\x0B\x83V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\n` R`@\x90 _Q` a;Q_9_Q\x90_R\x91a\x0B\xD8\x91a\x0B\xD2\x90\x91a\n3V[\x90a\x0B\x17V[`@\x80Q`\x01`\x01`\xD0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R\xA2_\x80\x80a\x0B\x98V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\n` R`@\x90 _Q` a;Q_9_Q\x90_R\x90a\x0C3\x90a\x0C-\x86a\n3V[\x90a\n\xBCV[`@\x80Q`\x01`\x01`\xD0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R\xA2_\x80a\x0B\x90V[P\x83\x15\x15a\x0B~V[_\x19\x81\x01\x91\x90\x82\x11a\x01\x93WV[\x90\x81Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\0\xB8W`\x01\x81\x01\x80\x84U\x81\x10\x15a\x0C\xBFW_\x92\x83R` \x92\x83\x90 \x82Q\x92\x90\x93\x01Q`0\x1Be\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x91\x01UV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80T\x92\x93\x92\x80\x15a\r\x7FWa\x0C\xEAa\x0C\xF5\x91a\x0C^V[\x82_R` _ \x01\x90V[\x80T`0\x81\x90\x1C\x93e\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92\x91\x81\x16\x80\x84\x11a\rpW\x87\x93\x03a\r<WPa\r8\x92P\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90`0\x1B\x16\x91\x16\x17\x90UV[\x91\x90V[\x91PPa\r8\x91a\r\\a\rNa\0\xBDV[e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[`\x01`\x01`\xD0\x1B\x03\x86\x16` \x83\x01Ra\x0ClV[c% `\x1D`\xE0\x1B_R`\x04_\xFD[P\x90a\r\xA4\x91a\r\x90a\rNa\0\xBDV[`\x01`\x01`\xD0\x1B\x03\x85\x16` \x83\x01Ra\x0ClV[_\x91\x90V[\x91\x90\x91\x80`\x01\x14a\r\xEBW`\x02\x14a\r\xCFWcNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[`\x01`\x01`\xD0\x1B\x03\x90\x81\x16\x91\x81\x16\x91\x90\x91\x03\x90\x81\x11a\x01\x93W\x90V[P`\x01`\x01`\xD0\x1B\x03\x91\x82\x16\x90\x82\x16\x01\x90\x81\x11a\x01\x93W\x90V\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\x02\xE5W\x80c\x06\xFD\xDE\x03\x14a\x02\xE0W\x80c\t^\xA7\xB3\x14a\x02\xDBW\x80c\x18\x16\r\xDD\x14a\x02,W\x80c#\xB8r\xDD\x14a\x02\xD6W\x80c$\x8A\x9C\xA3\x14a\x02\xD1W\x80c//\xF1]\x14a\x02\xCCW\x80c1<\xE5g\x14a\x02\xC7W\x80c6D\xE5\x15\x14a\x02\xC2W\x80c6V\x8A\xBE\x14a\x02\xBDW\x80c:F\xB1\xA8\x14a\x026W\x80c@\xC1\x0F\x19\x14a\x02\xB8W\x80cB\x96lh\x14a\x02\xB3W\x80cK\xF5\xD7\xE9\x14a\x02\xAEW\x80cO\x1B\xFC\x9E\x14a\x02\xA9W\x80cX|\xDE\x1E\x14a\x02\xA4W\x80c\\\x19\xA9\\\x14a\x02\x9FW\x80co\xCF\xFFE\x14a\x02\x9AW\x80cp\xA0\x821\x14a\x02\x95W\x80cy\xCCg\x90\x14a\x02\x90W\x80cz\x8C\xD1V\x14a\x02\x8BW\x80c~\xCE\xBE\0\x14a\x02\x86W\x80c\x83\xF1!\x1B\x14a\x02\x81W\x80c\x84&\xAD\xF2\x14a\x02|W\x80c\x84L\x90&\x14a\x02wW\x80c\x84\xB0\x19n\x14a\x02rW\x80c\x8AT%!\x14a\x02mW\x80c\x8D3C\xD6\x14a\x02hW\x80c\x8ES\x9E\x8C\x14a\x02cW\x80c\x90-U\xA5\x14a\x02^W\x80c\x91\xD1HT\x14a\x02YW\x80c\x91\xDD\xAD\xF4\x14a\x02TW\x80c\x95\xD8\x9BA\x14a\x02OW\x80c\x9A\xB2N\xB0\x14a\x021W\x80c\x9B~\xF6K\x14a\x02JW\x80c\xA2\x17\xFD\xDF\x14a\x02EW\x80c\xA9\x05\x9C\xBB\x14a\x02@W\x80c\xAA\x08*\x9D\x14a\x02;W\x80c\xB0\xCA%>\x14a\x026W\x80c\xBBMD6\x14a\x021W\x80c\xC0*\xE7T\x14a\x02,W\x80c\xC3\xCD\xA5 \x14a\x02'W\x80c\xD5\x05\xAC\xCF\x14a\x02\"W\x80c\xD5Gt\x1F\x14a\x02\x1DW\x80c\xDDb\xED>\x14a\x02\x18Wc\xF1\x12~\xD8\x14a\x02\x13W_\x80\xFD[a\x15\x15V[a\x14\xBCV[a\x14~V[a\x13$V[a\x11\xDDV[a\x056V[a\x11\x17V[a\x07\"V[a\x11\xA0V[a\x11zV[a\x11`V[a\x11:V[a\x10rV[a\x10GV[a\x0F\xF7V[a\x0F\xD1V[a\x0E\xF5V[a\x0E\xBBV[a\x0E\x81V[a\rJV[a\x0CcV[a\x0C)V[a\x0C\x05V[a\x0B\xCDV[a\x0B\xB3V[a\x0B\nV[a\n\xD5V[a\nZV[a\n8V[a\t\xF7V[a\t\xDAV[a\t1V[a\t\rV[a\x084V[a\x06\xC5V[a\x06\xABV[a\x06\x90V[a\x06KV[a\x06\x18V[a\x05SV[a\x05\x05V[a\x03\xE1V[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W`\x045\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x80\x91\x03a\x03\x86W\x80\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x92\x14\x90\x81\x15a\x03\\W[P`@Q\x90\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14_a\x03QV[_\x80\xFD[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` a\x03\xDE\x92\x81\x81R\x01\x90a\x03\x8AV[\x90V[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W`@Q_`\x03Ta\x04\x01\x81a\x16\x0BV[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x04\x97WP`\x01\x14a\x049W[a\x045\x83a\x04)\x81\x85\x03\x82a\x17JV[`@Q\x91\x82\x91\x82a\x03\xCDV[\x03\x90\xF3[`\x03_\x90\x81R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x93\x92P\x90[\x80\x82\x10a\x04}WP\x90\x91P\x81\x01` \x01a\x04)a\x04\x19V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x04eV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x04)\x90Pa\x04\x19V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03\x86WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03\x86WV[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x05+a\x05!a\x04\xD9V[`$5\x903a \x9EV[` `@Q`\x01\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `\x02T`@Q\x90\x81R\xF3[4a\x03\x86W```\x03\x196\x01\x12a\x03\x86Wa\x05la\x04\xD9V[a\x05ta\x04\xEFV[`D5\x90`\x01`\x01`\xA0\x1B\x03\x83\x16_R`\x01` Ra\x05\xA73`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x92_\x19\x84\x10a\x05\xC8W[a\x05\xBC\x93Pa\x18NV[`@Q`\x01\x81R` \x90\xF3[\x82\x84\x10a\x05\xE4Wa\x05\xDF\x83a\x05\xBC\x95\x033\x83a!lV[a\x05\xB2V[\x82\x84\x7F\xFB\x8FA\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`DR`d_\xFD[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W` a\x06C`\x045_R`\x05` R`\x01`@_ \x01T\x90V[`@Q\x90\x81R\xF3[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x06\x8E`\x045a\x06ja\x04\xEFV[\x90a\x06\x89a\x06\x84\x82_R`\x05` R`\x01`@_ \x01T\x90V[a\x1B\x89V[a\x1B\xEAV[\0[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Q`\x12\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` a\x06Ca\x1C\xA2V[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86W`\x045a\x06\xE1a\x04\xEFV[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\x06\xFAWa\x06\x8E\x91a\x1D\xBEV[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x07;a\x04\xD9V[`\x01`\x01`\xA0\x1B\x03`$5\x91\x16_R`\n` Ra\x07\\`@_ \x91a\x1EnV[\x81T\x90_\x82\x91`\x05\x84\x11a\x07\xDCW[a\x07v\x93P\x84a$\x87V[\x80a\x07\xA5WPP` y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_[\x16`@Q\x90\x81R\xF3[` \x91a\x07\xCCy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a\x17\xAAV[\x90_R\x82_ \x01T`0\x1Ca\x07\x9CV[\x91\x92a\x07\xE7\x81a#\x12V[\x81\x03\x90\x81\x11a\x08/Wa\x07v\x93\x85_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x08\x1DWP\x91a\x07kV[\x92\x91Pa\x08)\x90a\x17\x8BV[\x90a\x07kV[a\x15\xDEV[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x08Ma\x04\xD9V[`$5a\x08Xa\x1A\x11V[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x08\xE5W\x80\x15a\x08\xBDW`\x02T\x81\x81\x01\x80\x91\x11a\x08/Wk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x10a\x08\x95Wa\x06\x8E\x91a!\xB3V[\x7F\x17~?\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W`\x045\x80\x15a\x08\xBDWa\x06\x8E\x903a\x1E\xC0V[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86Wa\tJCa\"\x93V[e\xFF\xFF\xFF\xFF\xFF\xFF\x80a\t[Ca\"\x93V[\x16\x91\x16\x03a\t\xB2Wa\x045`@Qa\tt`@\x82a\x17JV[`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x03\x8AV[\x7Fo\xF0q@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Qb\x9E4\0\x81R\xF3[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W`\x01`\x01`\xA0\x1B\x03a\n\x18a\x04\xD9V[\x16_R`\t` R` `\x01`\x01`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86Wa\x06\x8Ea\nTa\x04\xD9V[3a\x1F\x8FV[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W`\x01`\x01`\xA0\x1B\x03a\n{a\x04\xD9V[\x16_R`\n` R`@_ Tc\xFF\xFF\xFF\xFF\x81\x11a\n\xA5W`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x90\xF3[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R` `\x04R`$R`D_\xFD[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W` a\x06Ca\n\xF3a\x04\xD9V[`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ T\x90V[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x0B#a\x04\xD9V[`$5\x90a\x0B/a\x1A\x99V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a\x08\xE5W\x82\x15a\x08\xBDWa\x0BNa\x17\xE4V[\x15a\x0B\x8BW\x82a\x0B]\x91a\x1E\xC0V[`@Q\x91\x82R\x7F\xBE\xF4\xF8\x1C\x18\x14\xC6A\xED\xE8^\xBA\xAC\xF1\x9D\x04\x8B,[U\x98\n\xDF\xA6\xEF\x0F\x95le\x135\xA2` 3\x93\xA3\0[\x7F\xB8\xB5\xCA-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` a\x06Ca\x17\xB8V[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W`\x01`\x01`\xA0\x1B\x03a\x0B\xEEa\x04\xD9V[\x16_R`\x08` R` `@_ T`@Q\x90\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` a\x0C\x1Fa\x17\xE4V[`@Q\x90\x15\x15\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xF3[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W`\x045a\x0C\x7Fa\x1B!V[B\x81\x11\x15a\r\"W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11a\x0C\xFAW\x7F\xDDh\x96\xDC\xF1\xD4\xB3\x11\xCC\xA8}\xD1\x9B\xBB\xA2\xEA\x9C\xE2\xF8g\xC1V\x88x\xA0C\x8Af\xA1\xAF\xEE\xECa\x0C\xF5`\x0CT\x92\x80`\x0CU`@Q\x91\x82\x913\x95\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x03\x90\xA2\0[\x7F\xEFi\xAFe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xA5e\x83S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86Wa\x0E(a\r\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a&>V[a\r\xAF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a&\xB7V[` `@Qa\r\xBE\x82\x82a\x17JV[_\x81R\x81a\x0E6\x81\x83\x01\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x83\x016\x877`@Q\x97\x88\x97\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R`\xE0\x85\x8A\x01R`\xE0\x89\x01\x90a\x03\x8AV[\x90\x87\x82\x03`@\x89\x01Ra\x03\x8AV[\x91F``\x87\x01R0`\x80\x87\x01R_`\xA0\x87\x01R\x85\x83\x03`\xC0\x87\x01RQ\x91\x82\x81R\x01\x92\x91_[\x82\x81\x10a\x0EjWPPPP\x03\x90\xF3[\x83Q\x85R\x86\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x0E[V[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Q\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Q\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x81R\xF3[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86Wa\x0F\x11`\x045a\x1EnV[`\x0BT\x90_\x82\x91`\x05\x84\x11a\x0F}W[a\x0F-\x93P`\x0Ba$\x87V[\x80a\x0F[WP` _[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[a\x0Fxa\x0Fi` \x92a\x17\xAAV[`\x0B_R\x82_ \x01T`0\x1C\x90V[a\x0F7V[\x91\x92a\x0F\x88\x81a#\x12V[\x81\x03\x90\x81\x11a\x08/Wa\x0F-\x93`\x0B_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a\x0F\xBFWP\x91a\x0F!V[\x92\x91Pa\x0F\xCB\x90a\x17\x8BV[\x90a\x0F!V[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Qk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81R\xF3[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86W` `\xFFa\x10;`\x045a\x10\x1Aa\x04\xEFV[\x90_R`\x05\x84R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` a\x10bCa\"\x93V[e\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W`@Q_`\x04Ta\x10\x92\x81a\x16\x0BV[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x04\x97WP`\x01\x14a\x10\xB9Wa\x045\x83a\x04)\x81\x85\x03\x82a\x17JV[`\x04_\x90\x81R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x93\x92P\x90[\x80\x82\x10a\x10\xFDWP\x90\x91P\x81\x01` \x01a\x04)a\x04\x19V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x10\xE5V[4a\x03\x86W` `\x03\x196\x01\x12a\x03\x86W` a\x06Ca\x115a\x04\xD9V[a\x17\xFBV[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Qk\x02\xF9\x01\x93\xEF0u\xFA\x98\0\0\0\x81R\xF3[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `@Q_\x81R\xF3[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x05+a\x11\x96a\x04\xD9V[`$5\x903a\x18NV[4a\x03\x86W_`\x03\x196\x01\x12a\x03\x86W` `\x0CT`@Q\x90\x81R\xF3[`d5\x90`\xFF\x82\x16\x82\x03a\x03\x86WV[`\x845\x90`\xFF\x82\x16\x82\x03a\x03\x86WV[4a\x03\x86W`\xC0`\x03\x196\x01\x12a\x03\x86Wa\x11\xF6a\x04\xD9V[`$5\x90`D5a\x12\x05a\x11\xBDV[`\x845\x90`\xA45\x92\x80B\x11a\x12\xF9W\x91a\x12\x8B\x93\x91a\x12}a\x12\x82\x94`@Q` \x81\x01\x91\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x83R`\x01`\x01`\xA0\x1B\x03\x8A\x16`@\x83\x01R\x8A``\x83\x01R`\x80\x82\x01R`\x80\x81Ra\x12u`\xA0\x82a\x17JV[Q\x90 a NV[a&\xEEV[\x90\x92\x91\x92a'\xB2V[a\x12\xAF\x81`\x01`\x01`\xA0\x1B\x03\x16_R`\x08` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[\x80\x93\x03a\x12\xC0Wa\x06\x8E\x92Pa\x1F\x8FV[`\x01`\x01`\xA0\x1B\x03\x91P\x7Fu-\x88\xC0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$R`D_\xFD[\x7FF\x83\xAF\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x03\x86W`\xE0`\x03\x196\x01\x12a\x03\x86Wa\x13=a\x04\xD9V[a\x13Ea\x04\xEFV[`D5\x90`d5\x92a\x13Ua\x11\xCDV[`\xA45`\xC45\x90\x86B\x11a\x14RWa\x13\xFE\x92a\x13\xF9a\x13\x8E\x86`\x01`\x01`\xA0\x1B\x03\x16_R`\x08` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[\x98`@Q` \x81\x01\x91\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R`\x01`\x01`\xA0\x1B\x03\x89\x16\x9B\x8C`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x8B\x16``\x84\x01R\x8B`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\x12u`\xE0\x82a\x17JV[a \x8FV[\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x03a\x14\x18Wa\x06\x8E\x93Pa \x9EV[\x7FK\x80\x0EF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x04R\x16`$R`D_\xFD[\x86\x7Fby\x13\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x06\x8E`\x045a\x14\x9Da\x04\xEFV[\x90a\x14\xB7a\x06\x84\x82_R`\x05` R`\x01`@_ \x01T\x90V[a\x1D\xBEV[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86W` a\x15\x0Ca\x14\xDAa\x04\xD9V[`\x01`\x01`\xA0\x1B\x03a\x14\xEAa\x04\xEFV[\x91\x16_R`\x01\x83R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[4a\x03\x86W`@`\x03\x196\x01\x12a\x03\x86Wa\x15.a\x04\xD9V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03\x86Wa\x045\x91`\x01`\x01`\xA0\x1B\x03a\x15{\x92a\x15Wa\x186V[Pa\x15`a\x186V[P\x16_R`\n` R`@_ a\x15ua\x186V[Pa(yV[P`@Q\x90a\x15\x89\x82a\x17)V[Te\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82R`0\x1C` \x82\x01R`@Q\x91\x82\x91\x82\x91\x90\x91` y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`@\x84\x01\x95e\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x85R\x01Q\x16\x91\x01RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x16RW[` \x83\x10\x14a\x16%WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x16\x1AV[_\x92\x91\x81T\x91a\x16k\x83a\x16\x0BV[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a\x16\xC0WP`\x01\x14a\x16\x87WPPPV[_\x90\x81R` \x81 \x93\x94P\x91\x92[\x83\x83\x10a\x16\xA6WP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a\x16\x95V[\x90P` \x94\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x92\x91\x92\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x17EW`@RV[a\x16\xFCV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x17EW`@RV[\x90`\x01\x82\x01\x80\x92\x11a\x08/WV[`@Q\x90a\x17\xA8`@\x83a\x17JV[V[\x90_\x19\x82\x01\x91\x82\x11a\x08/WV[`\x0CT\x80\x15\x80\x15a\x17\xDAW[a\x17\xD5WB\x81\x03\x90\x81\x11a\x08/W\x90V[P_\x90V[P\x80B\x10\x15a\x17\xC4V[`\x0CT\x80\x15\x15\x90\x81a\x17\xF4WP\x90V[\x90PB\x10\x90V[`\x01`\x01`\xA0\x1B\x03\x16_R`\n` Ry\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x182`@_ a %V[\x16\x90V[`@Q\x90a\x18C\x82a\x17)V[_` \x83\x82\x81R\x01RV[\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x93\x84\x15a\x19\xE5W`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x15a\x19\xB9Wa\x18za\x17\xE4V[\x80a\x19\x81W[a\x19YWa\x18\x9E\x82`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[T\x95\x84\x87\x10a\x19\x1AW\x84a\x17\xA8\x96\x97\x03a\x18\xC8\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[Ua\x18\xE3\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[\x80T\x86\x01\x90U`@Q\x85\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3a*PV[\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04R`$\x87\x90R`D\x85\x90R`d_\xFD[\x7F\xDB\x89\xE3\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P3_\x90\x81R\x7Ft\x07\x96\xD8~O\x86\xCC\x94g\x17h\xC7D\x95`E\xFE\x85P\x93)\x1Ey\x19L\x96\xACG\x80@\xAA` R`@\x90 T`\xFF\x16\x15a\x18\x80V[\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[3_\x90\x81R\x7F\x9Ak\xF4\x8B\xB8@\xE7\x8F\xE8\xE7\xAF\xD1\r=9\x1A\x91s\x8A\x9Ee$\xF6\xFD\xFA\x1A:\xBA\x9D\xC0?\xB1` R`@\x90 T`\xFF\x16\x15a\x1AIWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB`$R`D_\xFD[3_\x90\x81R\x7Ft\x07\x96\xD8~O\x86\xCC\x94g\x17h\xC7D\x95`E\xFE\x85P\x93)\x1Ey\x19L\x96\xACG\x80@\xAA` R`@\x90 T`\xFF\x16\x15a\x1A\xD1WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t`$R`D_\xFD[3_\x90\x81R\x7F\x05\xB8\xCC\xBB\x9DM\x8F\xB1n\xA7L\xE3\xC2\x9AA\xF1\xB4a\xFB\xDA\xFFG\x14\xA0\xD9\xA8\xEB\x05I\x97F\xBC` R`@\x90 T`\xFF\x16\x15a\x1BYWV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[\x80_R`\x05` R`\xFFa\x1B\xB13`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16\x15a\x1B\xBBWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R`\x05` R`\xFFa\x1C\x12\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16a\x1C\x9CW\x80_R`\x05` Ra\x1C>\x82`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90U`\x01`\x01`\xA0\x1B\x033\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x14\x80a\x1D\x95W[\x15a\x1C\xFDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x1D\x8F`\xC0\x82a\x17JV[Q\x90 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x1C\xD4V[\x80_R`\x05` R`\xFFa\x1D\xE6\x83`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16\x15a\x1C\x9CW\x80_R`\x05` Ra\x1E\x13\x82`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90U`\x01`\x01`\xA0\x1B\x033\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V[e\xFF\xFF\xFF\xFF\xFF\xFFa\x1E~Ca\"\x93V[\x16\x80\x82\x10\x15a\x1E\x91WPa\x03\xDE\x90a\"\x93V[\x90\x7F\xEC\xD3\xF8\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x15a\x19\xE5Wa\x1E\xEA\x82`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[T\x83\x81\x10a\x1FRW\x91_\x80\x92\x85a\x17\xA8\x96\x95\x03a\x1F\x17\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[U`\x02\x80T\x86\x90\x03\x90U`@Q\x85\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3a*PV[\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04R`$R`D\x83\x90R`d_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\t` R`@\x81 \x80T\x86\x85\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x81\x17\x90\x92Ua\x17\xA8\x96\x94\x16\x94a \x1F\x93\x90\x92\x86\x91\x90\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x90\x80\xA4`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ T\x90V[\x91a$\xEBV[\x80T\x80a 2WPP_\x90V[\x80_\x19\x81\x01\x11a\x08/W_\x19\x91_R` _ \x01\x01T`0\x1C\x90V[`B\x90a Ya\x1C\xA2V[\x90`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[\x91a\x03\xDE\x93\x91a\x12\x82\x93a&\xEEV[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a!@W`\x01`\x01`\xA0\x1B\x03\x81\x16\x92\x83\x15a!\x14W\x80a!\x07\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93\x85_R`\x01` R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[U`@Q\x90\x81R` \x90\xA3V[\x7F\x94(\rb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\xE6\x02\xDF\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a!@W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a!\x14Wa!\xB0\x91_R`\x01` R`@_ \x90`\x01`\x01`\xA0\x1B\x03\x16_R` R`@_ \x90V[UV[\x91\x90`\x02T\x81\x81\x01\x80\x91\x11a\x08/W`\x02U`\x01`\x01`\xA0\x1B\x03\x83\x16\x80a\"nW\x81`\x02T\x03`\x02U[`@Q\x82\x81R_\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x90\xA3`\x02T\x92y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x11a\">WPa\x17\xA8\x92\x93P_a*PV[\x7F\x1C\xB1]&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x85\x90R`$R`D_\xFD[a\"\x88\x84`\x01`\x01`\xA0\x1B\x03\x16_R_` R`@_ \x90V[\x82\x81T\x01\x90Ua!\xDDV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\"\xABWe\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`0`\x04R`$R`D_\xFD[\x81\x15a\"\xE5W\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[`\x01\x81\x11\x15a\x03\xDEW\x80`\x01p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x10\x15a$EW[a#\xEBa#\xE1a#\xD7a#\xCDa#\xC3a#\xB9a#\xA8a#\xF2\x97`\x04\x8Ah\x01\0\0\0\0\0\0\0\0a#\xF7\x9C\x10\x15a$8W[d\x01\0\0\0\0\x81\x10\x15a$+W[b\x01\0\0\x81\x10\x15a$\x1EW[a\x01\0\x81\x10\x15a$\x11W[`\x10\x81\x10\x15a$\x04W[\x10\x15a#\xFCW[`\x03\x02`\x01\x1C\x90V[a#\xB2\x81\x8Ba\"\xDBV[\x01`\x01\x1C\x90V[a#\xB2\x81\x8Aa\"\xDBV[a#\xB2\x81\x89a\"\xDBV[a#\xB2\x81\x88a\"\xDBV[a#\xB2\x81\x87a\"\xDBV[a#\xB2\x81\x86a\"\xDBV[\x80\x93a\"\xDBV[\x82\x11\x90V[\x90\x03\x90V[`\x01\x1Ba#\x9FV[`\x04\x1C\x91`\x02\x1B\x91a#\x98V[`\x08\x1C\x91`\x04\x1B\x91a#\x8EV[`\x10\x1C\x91`\x08\x1B\x91a#\x83V[` \x1C\x91`\x10\x1B\x91a#wV[`@\x1C\x91` \x1B\x91a#iV[PPa#\xF7a#\xF2a#\xEBa#\xE1a#\xD7a#\xCDa#\xC3a#\xB9a#\xA8a$l\x8A`\x80\x1C\x90V[\x98Ph\x01\0\0\0\0\0\0\0\0\x97Pa#8\x96PPPPPPPV[\x91\x90[\x83\x82\x10a$\x97WPPP\x90V[\x90\x91\x92\x80\x83\x16\x90\x80\x84\x18`\x01\x1C\x82\x01\x80\x92\x11a\x08/W\x84_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82` _ \x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x10_\x14a$\xD9WP\x92[\x91\x90a$\x8AV[\x93\x92Pa$\xE5\x90a\x17\x8BV[\x91a$\xD2V[\x91\x90`\x01`\x01`\xA0\x1B\x03\x81\x16\x92`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x84\x82\x14\x15\x80a&5W[a%\x19W[PPPPPV[\x81a%\xBFW[PP\x82a%.W[\x80\x80a%\x12V[a%\xB4a%\x9B\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x93a%\x95a%\x8Fy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95`\x01`\x01`\xA0\x1B\x03\x16_R`\n` R`@_ \x90V[\x91a(\xBBV[\x90a)\x8FV[`@\x80Q\x92\x85\x16\x83R\x93\x16` \x82\x01R\x91\x82\x91\x90\x82\x01\x90V[\x03\x90\xA2_\x80\x80a%'V[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa&+a%\x9Ba&\x1C\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x94`\x01`\x01`\xA0\x1B\x03\x16_R`\n` R`@_ \x90V[a&%\x88a(\xBBV[\x90a)+V[\x03\x90\xA2_\x80a%\x1FV[P\x83\x15\x15a%\rV[`\xFF\x81\x14a&\x9DW`\xFF\x81\x16\x90`\x1F\x82\x11a&uW`@Q\x91a&b`@\x84a\x17JV[` \x80\x84R\x83\x81\x01\x91\x906\x837\x83RR\x90V[\x7F\xB3Q+\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P`@Qa\x03\xDE\x81a&\xB0\x81`\x06a\x16\\V[\x03\x82a\x17JV[`\xFF\x81\x14a&\xDBW`\xFF\x81\x16\x90`\x1F\x82\x11a&uW`@Q\x91a&b`@\x84a\x17JV[P`@Qa\x03\xDE\x81a&\xB0\x81`\x07a\x16\\V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a'pW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a'eW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a'[W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[`@Q=_\x82>=\x90\xFD[PPP_\x91`\x03\x91\x90V[`\x04\x11\x15a'\x85WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[a'\xBB\x81a'{V[\x80a'\xC4WPPV[a'\xCD\x81a'{V[`\x01\x81\x03a'\xFDW\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[a(\x06\x81a'{V[`\x02\x81\x03a(:WP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80a(F`\x03\x92a'{V[\x14a(NWPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80T\x82\x10\x15a(\x8EW_R` _ \x01\x90_\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a(\xFBWy\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\xD0`\x04R`$R`D_\xFD[\x90a)5Ca\"\x93V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a)[\x85a %V[\x92\x16\x91\x16\x03\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x08/Wa)\x8B\x92a+\xD8V[\x90\x91V[\x90a)\x99Ca\"\x93V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a)\xBF\x85a %V[\x92\x16\x91\x16\x01\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x08/Wa)\x8B\x92a+\xD8V[a)\xF8Ca\"\x93V[\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a*\x1F`\x0Ba %V[\x92\x16\x91\x16\x03y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08/Wa)\x8B\x91`\x0Ba+\xD8V[\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a*\xC1W[`\x01`\x01`\xA0\x1B\x03a\x17\xA8\x93\x16\x90\x81\x15a*\xA9W[_R`\t` R`\x01`\x01`\xA0\x1B\x03`@_ T\x16\x90_R`\t` R`\x01`\x01`\xA0\x1B\x03`@_ T\x16\x90a$\xEBV[a*\xBAa*\xB5\x84a(\xBBV[a)\xEFV[PPa*xV[a*\xCA\x82a(\xBBV[\x92a*\xD4Ca\"\x93V[\x93y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80a*\xFB`\x0Ba %V[\x92\x16\x91\x16\x01\x90y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x08/Wa\x17\xA8\x94`\x01`\x01`\xA0\x1B\x03\x92a+:\x91`\x0Ba+\xD8V[\x90PP\x93PPa*cV[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x17EWa+g\x91`\x01\x82\x01\x81Ua(yV[a+\xACW\x81Q` \x92\x90\x92\x01Q`0\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x80T\x92\x93\x92\x80\x15a,\xCEWa+\xEFa+\xFA\x91a\x17\xAAV[\x82_R` _ \x01\x90V[\x80T`0\x81\x90\x1C\x93e\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92\x91\x81\x16\x80\x84\x11a,\xA6W\x87\x93\x03a,_WPa,[\x92P\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x83T\x92`0\x1B\x16\x91\x16\x17\x90UV[\x91\x90V[\x91PPa,[\x91a,\x7Fa,qa\x17\x99V[e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16` \x83\x01Ra+EV[\x7F% `\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P\x90a-\x06\x91a,\xDFa,qa\x17\x99V[y\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16` \x83\x01Ra+EV[_\x91\x90V\x01u\xB7\xA68Bw\x03\xF0\xDB\xE7\xBB\x9B\xBF\x98z%Qq{4\xE7\x9F3\xB5\xB1\0\x8D\x1F\xA0\x1D\xB8\x05\xB8\xCC\xBB\x9DM\x8F\xB1n\xA7L\xE3\xC2\x9AA\xF1\xB4a\xFB\xDA\xFFG\x14\xA0\xD9\xA8\xEB\x05I\x97F\xBC\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$`\xA04a\0\xD9W`\x1Fa\x13B8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xDDW\x80\x84\x92``\x94`@R\x839\x81\x01\x03\x12a\0\xD9Wa\0G\x81a\0\xF1V[a\0_`@a\0X` \x85\x01a\0\xF1V[\x93\x01a\0\xF1V[\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\0\xCAW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\0\xCAW`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\0\xCAWa\0\xA3\x92a\0\x9D\x91`\x80Ra\x01\x05V[Pa\x01{V[P`@Qa\x10\xD3\x90\x81a\x02\x0F\x829`\x80Q\x81\x81\x81a\x03!\x01R\x81\x81a\x07\xF1\x01Ra\rT\x01R\xF3[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0\xD9WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x13\"_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x01vW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x13\"_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a\x12\xE2_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x13\x02_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x01vW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x13\x02_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\xF2\x7F\x93h\xAA\x1Bv\x97\xE5\x04)\xF87\"\xB8\x8F.]\xB8\x18M^,\xBC\xF0`\xF41\x0B\xBD>}\x90_Q` a\x12\xE2_9_Q\x90_R\x90\x80\xA4`\x01\x90V\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x01\xFF\xC9\xA7\x14a\t\xFAWP\x80c\x15\x8E\xF9>\x14a\t\xD8W\x80c$\x8A\x9C\xA3\x14a\t\xAEW\x80c//\xF1]\x14a\tqW\x80c6V\x8A\xBE\x14a\t\x05W\x80cCX\x10\x10\x14a\x08/W\x80cC\xA3\xF8\xA1\x14a\x08\x15W\x80c[\xDFl\xA1\x14a\x07\xC5W\x80c_\x15\xC3\xC9\x14a\x07\xAAW\x80cvg\x18\x08\x14a\x07\x8DW\x80c\x89\x16$\x86\x14a\x07SW\x80c\x91\xD1HT\x14a\x06\xFDW\x80c\xA0\x88x}\x14a\x06\xBAW\x80c\xA2\x17\xFD\xDF\x14a\x06\xA0W\x80c\xAC\x12\xCE\x07\x14a\x06\x83W\x80c\xB1\x98\xD0(\x14a\x06^W\x80c\xC6:\tD\x14a\x05PW\x80c\xD3\xF5f\xAE\x14a\x02VW\x80c\xD5Gt\x1F\x14a\x02\x0FW\x80c\xDE\xBEO\x1F\x14a\x01\xD4W\x80c\xDF\x02D\xB1\x14a\x01\xB6W\x80c\xE0\xE6\x16\x9C\x14a\x01\x9BW\x80c\xE4\xB7\xFBs\x14a\x01xW\x80c\xEC\xEDU&\x14a\x01UWc\xFA9\x1Cd\x14a\x011W_\x80\xFD[4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `0`\x02T\x10\x15`@Q\x90\x81R\xF3[\x80\xFD[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `@Qg\r\xE0\xB6\xB3\xA7d\0\0\x81R\xF3[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` a\x01\x93a\r,V[`@Q\x90\x81R\xF3[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` a\x01\x93a\x0C\xEFV[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `\x03T`@Q\x90\x81R\xF3[P4a\x01RW\x80`\x03\x196\x01\x12a\x01RW` `@Q\x7F\x07n\xB8\xB8u\xB6\xEA\x83\x9B\x08|L\x0C\x1AFa\xB0\x89\xD3\xB6\xEE,\x1E\xF1\xB9\xCF\xA7\xFE\x10f\xD2\x06\x81R\xF3[P4a\x01RW`@`\x03\x196\x01\x12a\x01RWa\x02R`\x045a\x02/a\n\x98V[\x90a\x02Ma\x02H\x82_R_` R`\x01`@_ \x01T\x90V[a\x0E\xD3V[a\x10\x0BV[P\x80\xF3[P4a\x04QW`@`\x03\x196\x01\x12a\x04QW`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x04QW3_\x90\x81R\x7F\x0E%9\x0F\xF9SSX\xA5\xE9\x16\xDF\xE7\xD3\x82f\xC86\x01\xAFn\x11!\x05\xB2-\xF4\xA9\x0B\xF8\x91\x01` R`@\x90 T`$5\x90`\xFF\x16\x15a\x05\0W`\xFF`\x04T\x16\x15a\x04\x82W`\x02T\x90`0\x82\x10\x15a\x04\x82W\x83\x15a\x04\xD8W\x80\x82\x03a\x04\xAAWPPa\x02\xF3a\x0B\x05V[\x90\x81\x15a\x04\x82W`\x03T\x82\x81\x01\x80\x91\x11a\x04UW`\x03Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x04QW_\x80\x91`D`@Q\x80\x94\x81\x93\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x89`\x04\x84\x01R\x88`$\x84\x01RZ\xF1\x80\x15a\x04FWa\x043W[P`\x02T\x92\x83\x7F\x16\x0F\xC1\x95\xD6\xE56\x91\xD3\r\x80L\xE1\x90\xDC\tG\x18\x91g~CC;\x91\xA7\xA6\x13\x1C\x12\xA5\x9A`@a\x03\xC1a\r,V[\x81Q\x90\x87\x82R` \x82\x01R\xA3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14a\x04\x06WP`\x01` \x92\x01`\x02U`@Q\x90\x81R\xF3[\x80\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x92R`\x11`\x04R\xFD[a\x04?\x91P_\x90a\x0C\x81V[__a\x03\x90V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7F\x9E\x91\xC9\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F0A:\x1A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\x07n\xB8\xB8u\xB6\xEA\x83\x9B\x08|L\x0C\x1AFa\xB0\x89\xD3\xB6\xEE,\x1E\xF1\xB9\xCF\xA7\xFE\x10f\xD2\x06`$R`D_\xFD[4a\x04QW` `\x03\x196\x01\x12a\x04QW3_\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\x045\x90`\xFF\x16\x15a\x06.W`\x04T`\xFF\x81\x16a\x04\x82W\x81\x15a\x06\x06W\x7F\xC1,`\xAB\xC2\x16(n\xF2^4\xB1\x80Z\x0C=\xDAs\xE4\xC2\xFDl\xF3`\xE8\x07\xA7\xA9\xE71g9\x91`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0` \x93\x16\x17`\x04U\x80`\x01U`@Q\x90\x81R\xA1\0[\x7F\xEBv\x99 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R_`$R`D_\xFD[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@QjB,\xA8\xB0\xA0\nBP\0\0\0\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `\x01T`@Q\x90\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Q_\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW`\xA0`\x02T`0`\x03T\x91a\x06\xDDa\r,V[`@Q\x93\x82\x85R\x83` \x86\x01R`@\x85\x01R``\x84\x01R\x10\x15`\x80\x82\x01R\xF3[4a\x04QW`@`\x03\x196\x01\x12a\x04QWa\x07\x16a\n\x98V[`\x045_R_` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Q\x7F\xF2\x7F\x93h\xAA\x1Bv\x97\xE5\x04)\xF87\"\xB8\x8F.]\xB8\x18M^,\xBC\xF0`\xF41\x0B\xBD>}\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `\x02T`@Q\x90\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Q`0\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x04QW_`\x03\x196\x01\x12a\x04QW` a\x01\x93a\x0B\x05V[4a\x04QW` `\x03\x196\x01\x12a\x04QW3_\x90\x81R\x7F~\x972wWE-\x9E3\x15\xE3\x95\xBC\x10\x0E}\xA1\xF2\xD3Q\x06\xFF\xFD8\xF28\x07tp\x90\xEF\xA9` R`@\x90 T`\x045\x90`\xFF\x16\x15a\x08\xB5W\x80\x15a\x06\x06W\x80`\x01U`\x02T`@Q\x91\x82R\x7F\xB8\x13\xFF\xBE8}l\xF6\xE6\xA6\xF6\xC5\xF8\x90_vj\x0F\x1Cl\xD0\x1Cg1/p\x93V\xC6%\x97\xBD` 3\x93\xA3\0[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7F\xF2\x7F\x93h\xAA\x1Bv\x97\xE5\x04)\xF87\"\xB8\x8F.]\xB8\x18M^,\xBC\xF0`\xF41\x0B\xBD>}`$R`D_\xFD[4a\x04QW`@`\x03\x196\x01\x12a\x04QWa\t\x1Ea\n\x98V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\tIWa\tG\x90`\x045a\x10\x0BV[\0[\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x04QW`@`\x03\x196\x01\x12a\x04QWa\tG`\x045a\t\x90a\n\x98V[\x90a\t\xA9a\x02H\x82_R_` R`\x01`@_ \x01T\x90V[a\x0F9V[4a\x04QW` `\x03\x196\x01\x12a\x04QW` a\x01\x93`\x045_R_` R`\x01`@_ \x01T\x90V[4a\x04QW_`\x03\x196\x01\x12a\x04QW` `\xFF`\x04T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04QW` `\x03\x196\x01\x12a\x04QW`\x045\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x80\x92\x03a\x04QW\x81\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x93\x14\x90\x81\x15a\nnW[P\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x83a\ngV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x04QWV[\x81\x15a\n\xC5W\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x04UWV[`\xFF`\x04T\x16\x15\x80\x15a\x0CtW[a\x0CpW`\x02T`0\x03`0\x81\x11a\x04UWa\x0B-a\r,V[`\x01\x82\x14a\x0CkW`\x01T\x91g\r\xE0\xB6\xB3\xA7d\0\0\x83\x14a\x0C`WPa\x0BQa\x0C\xEFV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x0C\x19W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2\x1FILX\x9C\0\0\x81\x01\x90\x81\x11a\x04UW[a\x03\xE8\x81\x10\x15a\x0C\x13WPa\x03\xE8\x90[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15a\x0B\xEBW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2\x1FILX\x9C\0\0\x83\x01\x92\x83\x11a\x04UWa\x0B\xE8\x92a\x0B\xE3\x91a\n\xF2V[a\n\xBBV[\x90V[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x03\x91g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11a\x04UWa\x0B\xE8\x92a\x0B\xE3\x91a\n\xF2V[\x90a\x0B\x9CV[g\r\xE0\xB6\xB3\xA7d\0\0\x03g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x0B\x8CW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x90a\x0B\xE8\x92Pa\n\xBBV[\x90P\x90V[_\x90V[P`0`\x02T\x10\x15a\x0B\x13V[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\xC2W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\r\xE0\xB6\xB3\xA7d\0\0`\x02T`\x01T[`0\x82\x10a\r\x0CWPP\x90V[\x90\x91g\r\xE0\xB6\xB3\xA7d\0\0a\r#\x83`\x01\x93a\n\xF2V[\x04\x92\x01\x90a\x0C\xFFV[`@Q\x7F\x18\x16\r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82`\x04\x81\x84Z\xFA\x91\x82\x15a\x04FW_\x92a\x0E\x9EW[P` `\x04\x91`@Q\x92\x83\x80\x92\x7F\x90-U\xA5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04FW_\x91a\x0ElW[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xBD\xD3WO_\xF5\xBD\xB0\0\0\0\x81\x01\x81\x81\x11a\x04UW\x82\x11\x15a\x0EdW\x81\x03jB,\xA8\xB0\xA0\nBP\0\0\0\x01\x90\x81\x11a\x04UW[\x80jB,\xA8\xB0\xA0\nBP\0\0\0\x11_\x14a\x0E_WjB,\xA8\xB0\xA0\nBP\0\0\0\x03jB,\xA8\xB0\xA0\nBP\0\0\0\x81\x11a\x04UW\x90V[P_\x90V[PP_a\x0E)V[\x90P` \x81=` \x11a\x0E\x96W[\x81a\x0E\x87` \x93\x83a\x0C\x81V[\x81\x01\x03\x12a\x04QWQ_a\r\xE0V[=\x91Pa\x0EzV[\x90\x91P` \x81=` \x11a\x0E\xCBW[\x81a\x0E\xBA` \x93\x83a\x0C\x81V[\x81\x01\x03\x12a\x04QWQ\x90` a\r\xA0V[=\x91Pa\x0E\xADV[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x0F\nWPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x10\x05W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x10\x05W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r~\x972wWE-\x9E3\x15\xE3\x95\xBC\x10\x0E}\xA1\xF2\xD3Q\x06\xFF\xFD8\xF28\x07tp\x90\xEF\xA9\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ChangeFactorSet(uint256,uint256,address)` and selector `0xb813ffbe387d6cf6e6a6f6c5f8905f766a0f1c6cd01c67312f709356c62597bd`.
```solidity
event ChangeFactorSet(uint256 indexed epoch, uint256 changeFactor, address indexed setter);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChangeFactorSet {
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub changeFactor: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub setter: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ChangeFactorSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ChangeFactorSet(uint256,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                184u8, 19u8, 255u8, 190u8, 56u8, 125u8, 108u8, 246u8, 230u8, 166u8,
                246u8, 197u8, 248u8, 144u8, 95u8, 118u8, 106u8, 15u8, 28u8, 108u8, 208u8,
                28u8, 103u8, 49u8, 47u8, 112u8, 147u8, 86u8, 198u8, 37u8, 151u8, 189u8,
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
                    changeFactor: data.0,
                    setter: topics.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.changeFactor),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.epoch.clone(), self.setter.clone())
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
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.setter,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChangeFactorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChangeFactorSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChangeFactorSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EmissionMinted(uint256,uint256,uint256,address)` and selector `0x160fc195d6e53691d30d804ce190dc09471891677e43433b91a7a6131c12a59a`.
```solidity
event EmissionMinted(uint256 indexed epoch, uint256 amount, uint256 remainingSupply, address indexed to);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EmissionMinted {
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub remainingSupply: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for EmissionMinted {
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
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "EmissionMinted(uint256,uint256,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                22u8, 15u8, 193u8, 149u8, 214u8, 229u8, 54u8, 145u8, 211u8, 13u8, 128u8,
                76u8, 225u8, 144u8, 220u8, 9u8, 71u8, 24u8, 145u8, 103u8, 126u8, 67u8,
                67u8, 59u8, 145u8, 167u8, 166u8, 19u8, 28u8, 18u8, 165u8, 154u8,
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
                    amount: data.0,
                    remainingSupply: data.1,
                    to: topics.2,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.remainingSupply),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.epoch.clone(), self.to.clone())
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
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EmissionMinted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EmissionMinted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EmissionMinted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EmissionsInitialized(uint256)` and selector `0xc12c60abc216286ef25e34b1805a0c3dda73e4c2fd6cf360e807a7a9e7316739`.
```solidity
event EmissionsInitialized(uint256 defaultChangeFactor);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EmissionsInitialized {
        #[allow(missing_docs)]
        pub defaultChangeFactor: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for EmissionsInitialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EmissionsInitialized(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                193u8, 44u8, 96u8, 171u8, 194u8, 22u8, 40u8, 110u8, 242u8, 94u8, 52u8,
                177u8, 128u8, 90u8, 12u8, 61u8, 218u8, 115u8, 228u8, 194u8, 253u8, 108u8,
                243u8, 96u8, 232u8, 7u8, 167u8, 169u8, 231u8, 49u8, 103u8, 57u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    defaultChangeFactor: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.defaultChangeFactor),
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
        impl alloy_sol_types::private::IntoLogData for EmissionsInitialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EmissionsInitialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EmissionsInitialized) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `EMISSIONS_CAP()` and selector `0xb198d028`.
```solidity
function EMISSIONS_CAP() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EMISSIONS_CAPCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`EMISSIONS_CAP()`](EMISSIONS_CAPCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EMISSIONS_CAPReturn {
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
            impl ::core::convert::From<EMISSIONS_CAPCall> for UnderlyingRustTuple<'_> {
                fn from(value: EMISSIONS_CAPCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for EMISSIONS_CAPCall {
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
            impl ::core::convert::From<EMISSIONS_CAPReturn> for UnderlyingRustTuple<'_> {
                fn from(value: EMISSIONS_CAPReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for EMISSIONS_CAPReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EMISSIONS_CAPCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EMISSIONS_CAP()";
            const SELECTOR: [u8; 4] = [177u8, 152u8, 208u8, 40u8];
            #[inline]
            fn new<'a>(
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
                        let r: EMISSIONS_CAPReturn = r.into();
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
                        let r: EMISSIONS_CAPReturn = r.into();
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
    /**Function with signature `SCALE()` and selector `0xeced5526`.
```solidity
function SCALE() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SCALECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`SCALE()`](SCALECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SCALEReturn {
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
            impl ::core::convert::From<SCALECall> for UnderlyingRustTuple<'_> {
                fn from(value: SCALECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SCALECall {
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
            impl ::core::convert::From<SCALEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: SCALEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SCALEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SCALECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SCALE()";
            const SELECTOR: [u8; 4] = [236u8, 237u8, 85u8, 38u8];
            #[inline]
            fn new<'a>(
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
                        let r: SCALEReturn = r.into();
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
                        let r: SCALEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `TOTAL_EPOCHS()` and selector `0x5f15c3c9`.
```solidity
function TOTAL_EPOCHS() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TOTAL_EPOCHSCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`TOTAL_EPOCHS()`](TOTAL_EPOCHSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TOTAL_EPOCHSReturn {
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
            impl ::core::convert::From<TOTAL_EPOCHSCall> for UnderlyingRustTuple<'_> {
                fn from(value: TOTAL_EPOCHSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TOTAL_EPOCHSCall {
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
            impl ::core::convert::From<TOTAL_EPOCHSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: TOTAL_EPOCHSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TOTAL_EPOCHSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TOTAL_EPOCHSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TOTAL_EPOCHS()";
            const SELECTOR: [u8; 4] = [95u8, 21u8, 195u8, 201u8];
            #[inline]
            fn new<'a>(
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
                        let r: TOTAL_EPOCHSReturn = r.into();
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
                        let r: TOTAL_EPOCHSReturn = r.into();
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
    /**Function with signature `calculator()` and selector `0xce3e39c0`.
```solidity
function calculator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculatorCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`calculator()`](calculatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculatorReturn {
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
            impl ::core::convert::From<calculatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculatorCall {
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
            impl ::core::convert::From<calculatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: calculatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculator()";
            const SELECTOR: [u8; 4] = [206u8, 62u8, 57u8, 192u8];
            #[inline]
            fn new<'a>(
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
                        let r: calculatorReturn = r.into();
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
                        let r: calculatorReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `changeFactorManager()` and selector `0xbefb9679`.
```solidity
function changeFactorManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct changeFactorManagerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`changeFactorManager()`](changeFactorManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct changeFactorManagerReturn {
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
            impl ::core::convert::From<changeFactorManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: changeFactorManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for changeFactorManagerCall {
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
            impl ::core::convert::From<changeFactorManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: changeFactorManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for changeFactorManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for changeFactorManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "changeFactorManager()";
            const SELECTOR: [u8; 4] = [190u8, 251u8, 150u8, 121u8];
            #[inline]
            fn new<'a>(
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
                        let r: changeFactorManagerReturn = r.into();
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
                        let r: changeFactorManagerReturn = r.into();
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
    /**Function with signature `testFuzz_ChangeFactor_ValidRange(uint256)` and selector `0x6d6d4436`.
```solidity
function testFuzz_ChangeFactor_ValidRange(uint256 changeFactor) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_ChangeFactor_ValidRangeCall {
        #[allow(missing_docs)]
        pub changeFactor: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`testFuzz_ChangeFactor_ValidRange(uint256)`](testFuzz_ChangeFactor_ValidRangeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_ChangeFactor_ValidRangeReturn {}
    #[allow(
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
            impl ::core::convert::From<testFuzz_ChangeFactor_ValidRangeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_ChangeFactor_ValidRangeCall) -> Self {
                    (value.changeFactor,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_ChangeFactor_ValidRangeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { changeFactor: tuple.0 }
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
            impl ::core::convert::From<testFuzz_ChangeFactor_ValidRangeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_ChangeFactor_ValidRangeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_ChangeFactor_ValidRangeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzz_ChangeFactor_ValidRangeReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzz_ChangeFactor_ValidRangeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzz_ChangeFactor_ValidRangeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzz_ChangeFactor_ValidRangeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzz_ChangeFactor_ValidRange(uint256)";
            const SELECTOR: [u8; 4] = [109u8, 109u8, 68u8, 54u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.changeFactor),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzz_ChangeFactor_ValidRangeReturn::_tokenize(ret)
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
    /**Function with signature `testFuzz_MultipleEpochs_EmissionSum(uint8)` and selector `0xfdc50aca`.
```solidity
function testFuzz_MultipleEpochs_EmissionSum(uint8 epochs) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_MultipleEpochs_EmissionSumCall {
        #[allow(missing_docs)]
        pub epochs: u8,
    }
    ///Container type for the return parameters of the [`testFuzz_MultipleEpochs_EmissionSum(uint8)`](testFuzz_MultipleEpochs_EmissionSumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_MultipleEpochs_EmissionSumReturn {}
    #[allow(
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
            impl ::core::convert::From<testFuzz_MultipleEpochs_EmissionSumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_MultipleEpochs_EmissionSumCall) -> Self {
                    (value.epochs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_MultipleEpochs_EmissionSumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { epochs: tuple.0 }
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
            impl ::core::convert::From<testFuzz_MultipleEpochs_EmissionSumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_MultipleEpochs_EmissionSumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_MultipleEpochs_EmissionSumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzz_MultipleEpochs_EmissionSumReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzz_MultipleEpochs_EmissionSumCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzz_MultipleEpochs_EmissionSumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzz_MultipleEpochs_EmissionSumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzz_MultipleEpochs_EmissionSum(uint8)";
            const SELECTOR: [u8; 4] = [253u8, 197u8, 10u8, 202u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.epochs),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzz_MultipleEpochs_EmissionSumReturn::_tokenize(ret)
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
    /**Function with signature `test_CalculateAndMintEmission_FinalEpoch()` and selector `0x5ff4c899`.
```solidity
function test_CalculateAndMintEmission_FinalEpoch() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_CalculateAndMintEmission_FinalEpochCall;
    ///Container type for the return parameters of the [`test_CalculateAndMintEmission_FinalEpoch()`](test_CalculateAndMintEmission_FinalEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_CalculateAndMintEmission_FinalEpochReturn {}
    #[allow(
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
            impl ::core::convert::From<test_CalculateAndMintEmission_FinalEpochCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_CalculateAndMintEmission_FinalEpochCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_CalculateAndMintEmission_FinalEpochCall {
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
            impl ::core::convert::From<test_CalculateAndMintEmission_FinalEpochReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_CalculateAndMintEmission_FinalEpochReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_CalculateAndMintEmission_FinalEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_CalculateAndMintEmission_FinalEpochReturn {
            fn _tokenize(
                &self,
            ) -> <test_CalculateAndMintEmission_FinalEpochCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_CalculateAndMintEmission_FinalEpochCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_CalculateAndMintEmission_FinalEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_CalculateAndMintEmission_FinalEpoch()";
            const SELECTOR: [u8; 4] = [95u8, 244u8, 200u8, 153u8];
            #[inline]
            fn new<'a>(
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
                test_CalculateAndMintEmission_FinalEpochReturn::_tokenize(ret)
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
    /**Function with signature `test_CalculateAndMintEmission_FirstEpoch()` and selector `0x07e6233e`.
```solidity
function test_CalculateAndMintEmission_FirstEpoch() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_CalculateAndMintEmission_FirstEpochCall;
    ///Container type for the return parameters of the [`test_CalculateAndMintEmission_FirstEpoch()`](test_CalculateAndMintEmission_FirstEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_CalculateAndMintEmission_FirstEpochReturn {}
    #[allow(
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
            impl ::core::convert::From<test_CalculateAndMintEmission_FirstEpochCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_CalculateAndMintEmission_FirstEpochCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_CalculateAndMintEmission_FirstEpochCall {
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
            impl ::core::convert::From<test_CalculateAndMintEmission_FirstEpochReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_CalculateAndMintEmission_FirstEpochReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_CalculateAndMintEmission_FirstEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_CalculateAndMintEmission_FirstEpochReturn {
            fn _tokenize(
                &self,
            ) -> <test_CalculateAndMintEmission_FirstEpochCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_CalculateAndMintEmission_FirstEpochCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_CalculateAndMintEmission_FirstEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_CalculateAndMintEmission_FirstEpoch()";
            const SELECTOR: [u8; 4] = [7u8, 230u8, 35u8, 62u8];
            #[inline]
            fn new<'a>(
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
                test_CalculateAndMintEmission_FirstEpochReturn::_tokenize(ret)
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
    /**Function with signature `test_CalculateCumulativeProduct()` and selector `0x2a8ea2e2`.
```solidity
function test_CalculateCumulativeProduct() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_CalculateCumulativeProductCall;
    ///Container type for the return parameters of the [`test_CalculateCumulativeProduct()`](test_CalculateCumulativeProductCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_CalculateCumulativeProductReturn {}
    #[allow(
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
            impl ::core::convert::From<test_CalculateCumulativeProductCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_CalculateCumulativeProductCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_CalculateCumulativeProductCall {
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
            impl ::core::convert::From<test_CalculateCumulativeProductReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_CalculateCumulativeProductReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_CalculateCumulativeProductReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_CalculateCumulativeProductReturn {
            fn _tokenize(
                &self,
            ) -> <test_CalculateCumulativeProductCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_CalculateCumulativeProductCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_CalculateCumulativeProductReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_CalculateCumulativeProduct()";
            const SELECTOR: [u8; 4] = [42u8, 142u8, 162u8, 226u8];
            #[inline]
            fn new<'a>(
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
                test_CalculateCumulativeProductReturn::_tokenize(ret)
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
    /**Function with signature `test_Constructor_InitialSetup()` and selector `0x76029e78`.
```solidity
function test_Constructor_InitialSetup() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Constructor_InitialSetupCall;
    ///Container type for the return parameters of the [`test_Constructor_InitialSetup()`](test_Constructor_InitialSetupCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Constructor_InitialSetupReturn {}
    #[allow(
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
            impl ::core::convert::From<test_Constructor_InitialSetupCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Constructor_InitialSetupCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Constructor_InitialSetupCall {
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
            impl ::core::convert::From<test_Constructor_InitialSetupReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Constructor_InitialSetupReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Constructor_InitialSetupReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Constructor_InitialSetupReturn {
            fn _tokenize(
                &self,
            ) -> <test_Constructor_InitialSetupCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Constructor_InitialSetupCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Constructor_InitialSetupReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Constructor_InitialSetup()";
            const SELECTOR: [u8; 4] = [118u8, 2u8, 158u8, 120u8];
            #[inline]
            fn new<'a>(
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
                test_Constructor_InitialSetupReturn::_tokenize(ret)
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
    /**Function with signature `test_GetEmissionsInfo()` and selector `0x4fddb7a6`.
```solidity
function test_GetEmissionsInfo() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetEmissionsInfoCall;
    ///Container type for the return parameters of the [`test_GetEmissionsInfo()`](test_GetEmissionsInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetEmissionsInfoReturn {}
    #[allow(
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
            impl ::core::convert::From<test_GetEmissionsInfoCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetEmissionsInfoCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetEmissionsInfoCall {
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
            impl ::core::convert::From<test_GetEmissionsInfoReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetEmissionsInfoReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetEmissionsInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_GetEmissionsInfoReturn {
            fn _tokenize(
                &self,
            ) -> <test_GetEmissionsInfoCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_GetEmissionsInfoCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_GetEmissionsInfoReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_GetEmissionsInfo()";
            const SELECTOR: [u8; 4] = [79u8, 221u8, 183u8, 166u8];
            #[inline]
            fn new<'a>(
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
                test_GetEmissionsInfoReturn::_tokenize(ret)
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
    /**Function with signature `test_GetRemainingSupply()` and selector `0x0a6a8336`.
```solidity
function test_GetRemainingSupply() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetRemainingSupplyCall;
    ///Container type for the return parameters of the [`test_GetRemainingSupply()`](test_GetRemainingSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetRemainingSupplyReturn {}
    #[allow(
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
            impl ::core::convert::From<test_GetRemainingSupplyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetRemainingSupplyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetRemainingSupplyCall {
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
            impl ::core::convert::From<test_GetRemainingSupplyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetRemainingSupplyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetRemainingSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_GetRemainingSupplyReturn {
            fn _tokenize(
                &self,
            ) -> <test_GetRemainingSupplyCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_GetRemainingSupplyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_GetRemainingSupplyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_GetRemainingSupply()";
            const SELECTOR: [u8; 4] = [10u8, 106u8, 131u8, 54u8];
            #[inline]
            fn new<'a>(
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
                test_GetRemainingSupplyReturn::_tokenize(ret)
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
    /**Function with signature `test_InitializeEmissions_Success()` and selector `0xcbe7fbac`.
```solidity
function test_InitializeEmissions_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_InitializeEmissions_SuccessCall;
    ///Container type for the return parameters of the [`test_InitializeEmissions_Success()`](test_InitializeEmissions_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_InitializeEmissions_SuccessReturn {}
    #[allow(
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
            impl ::core::convert::From<test_InitializeEmissions_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_InitializeEmissions_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_InitializeEmissions_SuccessCall {
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
            impl ::core::convert::From<test_InitializeEmissions_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_InitializeEmissions_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_InitializeEmissions_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_InitializeEmissions_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_InitializeEmissions_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_InitializeEmissions_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_InitializeEmissions_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_InitializeEmissions_Success()";
            const SELECTOR: [u8; 4] = [203u8, 231u8, 251u8, 172u8];
            #[inline]
            fn new<'a>(
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
                test_InitializeEmissions_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_Integration_FullEmissionCycle()` and selector `0x47da5264`.
```solidity
function test_Integration_FullEmissionCycle() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Integration_FullEmissionCycleCall;
    ///Container type for the return parameters of the [`test_Integration_FullEmissionCycle()`](test_Integration_FullEmissionCycleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Integration_FullEmissionCycleReturn {}
    #[allow(
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
            impl ::core::convert::From<test_Integration_FullEmissionCycleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Integration_FullEmissionCycleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Integration_FullEmissionCycleCall {
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
            impl ::core::convert::From<test_Integration_FullEmissionCycleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Integration_FullEmissionCycleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Integration_FullEmissionCycleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Integration_FullEmissionCycleReturn {
            fn _tokenize(
                &self,
            ) -> <test_Integration_FullEmissionCycleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Integration_FullEmissionCycleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Integration_FullEmissionCycleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Integration_FullEmissionCycle()";
            const SELECTOR: [u8; 4] = [71u8, 218u8, 82u8, 100u8];
            #[inline]
            fn new<'a>(
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
                test_Integration_FullEmissionCycleReturn::_tokenize(ret)
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
    /**Function with signature `test_PreviewCurrentEmission()` and selector `0x6f7c71ec`.
```solidity
function test_PreviewCurrentEmission() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_PreviewCurrentEmissionCall;
    ///Container type for the return parameters of the [`test_PreviewCurrentEmission()`](test_PreviewCurrentEmissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_PreviewCurrentEmissionReturn {}
    #[allow(
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
            impl ::core::convert::From<test_PreviewCurrentEmissionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_PreviewCurrentEmissionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_PreviewCurrentEmissionCall {
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
            impl ::core::convert::From<test_PreviewCurrentEmissionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_PreviewCurrentEmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_PreviewCurrentEmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_PreviewCurrentEmissionReturn {
            fn _tokenize(
                &self,
            ) -> <test_PreviewCurrentEmissionCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_PreviewCurrentEmissionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_PreviewCurrentEmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_PreviewCurrentEmission()";
            const SELECTOR: [u8; 4] = [111u8, 124u8, 113u8, 236u8];
            #[inline]
            fn new<'a>(
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
                test_PreviewCurrentEmissionReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_CalculateAndMintEmission_Completed()` and selector `0x95a19046`.
```solidity
function test_RevertWhen_CalculateAndMintEmission_Completed() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_CalculateAndMintEmission_CompletedCall;
    ///Container type for the return parameters of the [`test_RevertWhen_CalculateAndMintEmission_Completed()`](test_RevertWhen_CalculateAndMintEmission_CompletedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_CalculateAndMintEmission_CompletedReturn {}
    #[allow(
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
                test_RevertWhen_CalculateAndMintEmission_CompletedCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_CalculateAndMintEmission_CompletedCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_CalculateAndMintEmission_CompletedCall {
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
                test_RevertWhen_CalculateAndMintEmission_CompletedReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_CalculateAndMintEmission_CompletedReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_CalculateAndMintEmission_CompletedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_CalculateAndMintEmission_CompletedReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_CalculateAndMintEmission_CompletedCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_RevertWhen_CalculateAndMintEmission_CompletedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_CalculateAndMintEmission_CompletedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_CalculateAndMintEmission_Completed()";
            const SELECTOR: [u8; 4] = [149u8, 161u8, 144u8, 70u8];
            #[inline]
            fn new<'a>(
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
                test_RevertWhen_CalculateAndMintEmission_CompletedReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_CalculateAndMintEmission_NotInitialized()` and selector `0x04891604`.
```solidity
function test_RevertWhen_CalculateAndMintEmission_NotInitialized() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_CalculateAndMintEmission_NotInitializedCall;
    ///Container type for the return parameters of the [`test_RevertWhen_CalculateAndMintEmission_NotInitialized()`](test_RevertWhen_CalculateAndMintEmission_NotInitializedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_CalculateAndMintEmission_NotInitializedReturn {}
    #[allow(
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
                test_RevertWhen_CalculateAndMintEmission_NotInitializedCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_CalculateAndMintEmission_NotInitializedCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_CalculateAndMintEmission_NotInitializedCall {
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
                test_RevertWhen_CalculateAndMintEmission_NotInitializedReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_CalculateAndMintEmission_NotInitializedReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_CalculateAndMintEmission_NotInitializedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_CalculateAndMintEmission_NotInitializedReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_CalculateAndMintEmission_NotInitializedCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_RevertWhen_CalculateAndMintEmission_NotInitializedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_CalculateAndMintEmission_NotInitializedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_CalculateAndMintEmission_NotInitialized()";
            const SELECTOR: [u8; 4] = [4u8, 137u8, 22u8, 4u8];
            #[inline]
            fn new<'a>(
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
                test_RevertWhen_CalculateAndMintEmission_NotInitializedReturn::_tokenize(
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `test_RevertWhen_CalculateAndMintEmission_ZeroAddress()` and selector `0xb72a6e9b`.
```solidity
function test_RevertWhen_CalculateAndMintEmission_ZeroAddress() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall;
    ///Container type for the return parameters of the [`test_RevertWhen_CalculateAndMintEmission_ZeroAddress()`](test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_CalculateAndMintEmission_ZeroAddressReturn {}
    #[allow(
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
                test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall {
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
                test_RevertWhen_CalculateAndMintEmission_ZeroAddressReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_CalculateAndMintEmission_ZeroAddressReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_CalculateAndMintEmission_ZeroAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_CalculateAndMintEmission_ZeroAddressReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_CalculateAndMintEmission_ZeroAddressReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_CalculateAndMintEmission_ZeroAddress()";
            const SELECTOR: [u8; 4] = [183u8, 42u8, 110u8, 155u8];
            #[inline]
            fn new<'a>(
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
                test_RevertWhen_CalculateAndMintEmission_ZeroAddressReturn::_tokenize(
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `test_RevertWhen_Constructor_ZeroAddresses()` and selector `0xb230c827`.
```solidity
function test_RevertWhen_Constructor_ZeroAddresses() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Constructor_ZeroAddressesCall;
    ///Container type for the return parameters of the [`test_RevertWhen_Constructor_ZeroAddresses()`](test_RevertWhen_Constructor_ZeroAddressesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_Constructor_ZeroAddressesReturn {}
    #[allow(
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
            impl ::core::convert::From<test_RevertWhen_Constructor_ZeroAddressesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Constructor_ZeroAddressesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Constructor_ZeroAddressesCall {
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
            impl ::core::convert::From<test_RevertWhen_Constructor_ZeroAddressesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_Constructor_ZeroAddressesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_Constructor_ZeroAddressesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_Constructor_ZeroAddressesReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_Constructor_ZeroAddressesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_RevertWhen_Constructor_ZeroAddressesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_Constructor_ZeroAddressesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_Constructor_ZeroAddresses()";
            const SELECTOR: [u8; 4] = [178u8, 48u8, 200u8, 39u8];
            #[inline]
            fn new<'a>(
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
                test_RevertWhen_Constructor_ZeroAddressesReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_InitializeEmissions_AlreadyInitialized()` and selector `0x124fb3ce`.
```solidity
function test_RevertWhen_InitializeEmissions_AlreadyInitialized() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_InitializeEmissions_AlreadyInitializedCall;
    ///Container type for the return parameters of the [`test_RevertWhen_InitializeEmissions_AlreadyInitialized()`](test_RevertWhen_InitializeEmissions_AlreadyInitializedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_InitializeEmissions_AlreadyInitializedReturn {}
    #[allow(
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
                test_RevertWhen_InitializeEmissions_AlreadyInitializedCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_InitializeEmissions_AlreadyInitializedCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_InitializeEmissions_AlreadyInitializedCall {
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
                test_RevertWhen_InitializeEmissions_AlreadyInitializedReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_InitializeEmissions_AlreadyInitializedReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_InitializeEmissions_AlreadyInitializedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_InitializeEmissions_AlreadyInitializedReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_InitializeEmissions_AlreadyInitializedCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_RevertWhen_InitializeEmissions_AlreadyInitializedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_InitializeEmissions_AlreadyInitializedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_InitializeEmissions_AlreadyInitialized()";
            const SELECTOR: [u8; 4] = [18u8, 79u8, 179u8, 206u8];
            #[inline]
            fn new<'a>(
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
                test_RevertWhen_InitializeEmissions_AlreadyInitializedReturn::_tokenize(
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `test_RevertWhen_InitializeEmissions_InvalidChangeFactor()` and selector `0x66a47d65`.
```solidity
function test_RevertWhen_InitializeEmissions_InvalidChangeFactor() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall;
    ///Container type for the return parameters of the [`test_RevertWhen_InitializeEmissions_InvalidChangeFactor()`](test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_InitializeEmissions_InvalidChangeFactorReturn {}
    #[allow(
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
                test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall {
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
                test_RevertWhen_InitializeEmissions_InvalidChangeFactorReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_InitializeEmissions_InvalidChangeFactorReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_InitializeEmissions_InvalidChangeFactorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_InitializeEmissions_InvalidChangeFactorReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_InitializeEmissions_InvalidChangeFactorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_InitializeEmissions_InvalidChangeFactor()";
            const SELECTOR: [u8; 4] = [102u8, 164u8, 125u8, 101u8];
            #[inline]
            fn new<'a>(
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
                test_RevertWhen_InitializeEmissions_InvalidChangeFactorReturn::_tokenize(
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `test_RevertWhen_InitializeEmissions_NotAdmin()` and selector `0x8f3b08f7`.
```solidity
function test_RevertWhen_InitializeEmissions_NotAdmin() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_InitializeEmissions_NotAdminCall;
    ///Container type for the return parameters of the [`test_RevertWhen_InitializeEmissions_NotAdmin()`](test_RevertWhen_InitializeEmissions_NotAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_InitializeEmissions_NotAdminReturn {}
    #[allow(
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
            impl ::core::convert::From<test_RevertWhen_InitializeEmissions_NotAdminCall>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_InitializeEmissions_NotAdminCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_InitializeEmissions_NotAdminCall {
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
                test_RevertWhen_InitializeEmissions_NotAdminReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_InitializeEmissions_NotAdminReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_InitializeEmissions_NotAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_InitializeEmissions_NotAdminReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_InitializeEmissions_NotAdminCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_RevertWhen_InitializeEmissions_NotAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_InitializeEmissions_NotAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_InitializeEmissions_NotAdmin()";
            const SELECTOR: [u8; 4] = [143u8, 59u8, 8u8, 247u8];
            #[inline]
            fn new<'a>(
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
                test_RevertWhen_InitializeEmissions_NotAdminReturn::_tokenize(ret)
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
    /**Function with signature `test_SetChangeFactor_Success()` and selector `0xd9a19470`.
```solidity
function test_SetChangeFactor_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetChangeFactor_SuccessCall;
    ///Container type for the return parameters of the [`test_SetChangeFactor_Success()`](test_SetChangeFactor_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetChangeFactor_SuccessReturn {}
    #[allow(
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
            impl ::core::convert::From<test_SetChangeFactor_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetChangeFactor_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetChangeFactor_SuccessCall {
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
            impl ::core::convert::From<test_SetChangeFactor_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetChangeFactor_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetChangeFactor_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_SetChangeFactor_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_SetChangeFactor_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_SetChangeFactor_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_SetChangeFactor_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_SetChangeFactor_Success()";
            const SELECTOR: [u8; 4] = [217u8, 161u8, 148u8, 112u8];
            #[inline]
            fn new<'a>(
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
                test_SetChangeFactor_SuccessReturn::_tokenize(ret)
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
    /**Function with signature `test_SetChangeFactor_Zero()` and selector `0x735fb47b`.
```solidity
function test_SetChangeFactor_Zero() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetChangeFactor_ZeroCall;
    ///Container type for the return parameters of the [`test_SetChangeFactor_Zero()`](test_SetChangeFactor_ZeroCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetChangeFactor_ZeroReturn {}
    #[allow(
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
            impl ::core::convert::From<test_SetChangeFactor_ZeroCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetChangeFactor_ZeroCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetChangeFactor_ZeroCall {
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
            impl ::core::convert::From<test_SetChangeFactor_ZeroReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetChangeFactor_ZeroReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetChangeFactor_ZeroReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_SetChangeFactor_ZeroReturn {
            fn _tokenize(
                &self,
            ) -> <test_SetChangeFactor_ZeroCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_SetChangeFactor_ZeroCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_SetChangeFactor_ZeroReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_SetChangeFactor_Zero()";
            const SELECTOR: [u8; 4] = [115u8, 95u8, 180u8, 123u8];
            #[inline]
            fn new<'a>(
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
                test_SetChangeFactor_ZeroReturn::_tokenize(ret)
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
    /**Function with signature `treasury()` and selector `0x61d027b3`.
```solidity
function treasury() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct treasuryCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`treasury()`](treasuryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct treasuryReturn {
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
            impl ::core::convert::From<treasuryCall> for UnderlyingRustTuple<'_> {
                fn from(value: treasuryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for treasuryCall {
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
            impl ::core::convert::From<treasuryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: treasuryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for treasuryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for treasuryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "treasury()";
            const SELECTOR: [u8; 4] = [97u8, 208u8, 39u8, 179u8];
            #[inline]
            fn new<'a>(
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
                        let r: treasuryReturn = r.into();
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
                        let r: treasuryReturn = r.into();
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
    ///Container for all the [`EmissionsCalculatorTest`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum EmissionsCalculatorTestCalls {
        #[allow(missing_docs)]
        EMISSIONS_CAP(EMISSIONS_CAPCall),
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        SCALE(SCALECall),
        #[allow(missing_docs)]
        TOTAL_EPOCHS(TOTAL_EPOCHSCall),
        #[allow(missing_docs)]
        admin(adminCall),
        #[allow(missing_docs)]
        calculator(calculatorCall),
        #[allow(missing_docs)]
        changeFactorManager(changeFactorManagerCall),
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
        testFuzz_ChangeFactor_ValidRange(testFuzz_ChangeFactor_ValidRangeCall),
        #[allow(missing_docs)]
        testFuzz_MultipleEpochs_EmissionSum(testFuzz_MultipleEpochs_EmissionSumCall),
        #[allow(missing_docs)]
        test_CalculateAndMintEmission_FinalEpoch(
            test_CalculateAndMintEmission_FinalEpochCall,
        ),
        #[allow(missing_docs)]
        test_CalculateAndMintEmission_FirstEpoch(
            test_CalculateAndMintEmission_FirstEpochCall,
        ),
        #[allow(missing_docs)]
        test_CalculateCumulativeProduct(test_CalculateCumulativeProductCall),
        #[allow(missing_docs)]
        test_Constructor_InitialSetup(test_Constructor_InitialSetupCall),
        #[allow(missing_docs)]
        test_Constructor_RoleAssignment(test_Constructor_RoleAssignmentCall),
        #[allow(missing_docs)]
        test_GetEmissionsInfo(test_GetEmissionsInfoCall),
        #[allow(missing_docs)]
        test_GetRemainingSupply(test_GetRemainingSupplyCall),
        #[allow(missing_docs)]
        test_InitializeEmissions_Success(test_InitializeEmissions_SuccessCall),
        #[allow(missing_docs)]
        test_Integration_FullEmissionCycle(test_Integration_FullEmissionCycleCall),
        #[allow(missing_docs)]
        test_PreviewCurrentEmission(test_PreviewCurrentEmissionCall),
        #[allow(missing_docs)]
        test_RevertWhen_CalculateAndMintEmission_Completed(
            test_RevertWhen_CalculateAndMintEmission_CompletedCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_CalculateAndMintEmission_NotInitialized(
            test_RevertWhen_CalculateAndMintEmission_NotInitializedCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_CalculateAndMintEmission_ZeroAddress(
            test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_Constructor_ZeroAddresses(
            test_RevertWhen_Constructor_ZeroAddressesCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_InitializeEmissions_AlreadyInitialized(
            test_RevertWhen_InitializeEmissions_AlreadyInitializedCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_InitializeEmissions_InvalidChangeFactor(
            test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_InitializeEmissions_NotAdmin(
            test_RevertWhen_InitializeEmissions_NotAdminCall,
        ),
        #[allow(missing_docs)]
        test_SetChangeFactor_Success(test_SetChangeFactor_SuccessCall),
        #[allow(missing_docs)]
        test_SetChangeFactor_Zero(test_SetChangeFactor_ZeroCall),
        #[allow(missing_docs)]
        token(tokenCall),
        #[allow(missing_docs)]
        treasury(treasuryCall),
        #[allow(missing_docs)]
        user(userCall),
    }
    impl EmissionsCalculatorTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 137u8, 22u8, 4u8],
            [7u8, 230u8, 35u8, 62u8],
            [10u8, 106u8, 131u8, 54u8],
            [10u8, 146u8, 84u8, 228u8],
            [18u8, 79u8, 179u8, 206u8],
            [30u8, 215u8, 131u8, 28u8],
            [42u8, 142u8, 162u8, 226u8],
            [42u8, 222u8, 56u8, 128u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [71u8, 218u8, 82u8, 100u8],
            [79u8, 134u8, 50u8, 186u8],
            [79u8, 221u8, 183u8, 166u8],
            [95u8, 21u8, 195u8, 201u8],
            [95u8, 244u8, 200u8, 153u8],
            [97u8, 208u8, 39u8, 179u8],
            [102u8, 164u8, 125u8, 101u8],
            [102u8, 217u8, 169u8, 160u8],
            [109u8, 109u8, 68u8, 54u8],
            [111u8, 124u8, 113u8, 236u8],
            [115u8, 95u8, 180u8, 123u8],
            [118u8, 2u8, 158u8, 120u8],
            [133u8, 34u8, 108u8, 129u8],
            [143u8, 59u8, 8u8, 247u8],
            [145u8, 106u8, 23u8, 198u8],
            [149u8, 161u8, 144u8, 70u8],
            [176u8, 70u8, 79u8, 220u8],
            [177u8, 152u8, 208u8, 40u8],
            [178u8, 48u8, 200u8, 39u8],
            [181u8, 80u8, 138u8, 169u8],
            [183u8, 42u8, 110u8, 155u8],
            [186u8, 65u8, 79u8, 166u8],
            [190u8, 251u8, 150u8, 121u8],
            [203u8, 231u8, 251u8, 172u8],
            [206u8, 62u8, 57u8, 192u8],
            [217u8, 161u8, 148u8, 112u8],
            [220u8, 204u8, 87u8, 241u8],
            [226u8, 12u8, 159u8, 113u8],
            [236u8, 237u8, 85u8, 38u8],
            [248u8, 81u8, 164u8, 64u8],
            [250u8, 118u8, 38u8, 212u8],
            [252u8, 12u8, 84u8, 106u8],
            [253u8, 197u8, 10u8, 202u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(test_RevertWhen_CalculateAndMintEmission_NotInitialized),
            ::core::stringify!(test_CalculateAndMintEmission_FirstEpoch),
            ::core::stringify!(test_GetRemainingSupply),
            ::core::stringify!(setUp),
            ::core::stringify!(test_RevertWhen_InitializeEmissions_AlreadyInitialized),
            ::core::stringify!(excludeSenders),
            ::core::stringify!(test_CalculateCumulativeProduct),
            ::core::stringify!(targetInterfaces),
            ::core::stringify!(targetSenders),
            ::core::stringify!(targetContracts),
            ::core::stringify!(test_Integration_FullEmissionCycle),
            ::core::stringify!(user),
            ::core::stringify!(test_GetEmissionsInfo),
            ::core::stringify!(TOTAL_EPOCHS),
            ::core::stringify!(test_CalculateAndMintEmission_FinalEpoch),
            ::core::stringify!(treasury),
            ::core::stringify!(test_RevertWhen_InitializeEmissions_InvalidChangeFactor),
            ::core::stringify!(targetArtifactSelectors),
            ::core::stringify!(testFuzz_ChangeFactor_ValidRange),
            ::core::stringify!(test_PreviewCurrentEmission),
            ::core::stringify!(test_SetChangeFactor_Zero),
            ::core::stringify!(test_Constructor_InitialSetup),
            ::core::stringify!(targetArtifacts),
            ::core::stringify!(test_RevertWhen_InitializeEmissions_NotAdmin),
            ::core::stringify!(targetSelectors),
            ::core::stringify!(test_RevertWhen_CalculateAndMintEmission_Completed),
            ::core::stringify!(excludeSelectors),
            ::core::stringify!(EMISSIONS_CAP),
            ::core::stringify!(test_RevertWhen_Constructor_ZeroAddresses),
            ::core::stringify!(excludeArtifacts),
            ::core::stringify!(test_RevertWhen_CalculateAndMintEmission_ZeroAddress),
            ::core::stringify!(failed),
            ::core::stringify!(changeFactorManager),
            ::core::stringify!(test_InitializeEmissions_Success),
            ::core::stringify!(calculator),
            ::core::stringify!(test_SetChangeFactor_Success),
            ::core::stringify!(test_Constructor_RoleAssignment),
            ::core::stringify!(excludeContracts),
            ::core::stringify!(SCALE),
            ::core::stringify!(admin),
            ::core::stringify!(IS_TEST),
            ::core::stringify!(token),
            ::core::stringify!(testFuzz_MultipleEpochs_EmissionSum),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <test_RevertWhen_CalculateAndMintEmission_NotInitializedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_CalculateAndMintEmission_FirstEpochCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_GetRemainingSupplyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setUpCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_InitializeEmissions_AlreadyInitializedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_CalculateCumulativeProductCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetInterfacesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Integration_FullEmissionCycleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <userCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_GetEmissionsInfoCall as alloy_sol_types::SolCall>::SIGNATURE,
            <TOTAL_EPOCHSCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_CalculateAndMintEmission_FinalEpochCall as alloy_sol_types::SolCall>::SIGNATURE,
            <treasuryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testFuzz_ChangeFactor_ValidRangeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_PreviewCurrentEmissionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_SetChangeFactor_ZeroCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Constructor_InitialSetupCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_InitializeEmissions_NotAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_CalculateAndMintEmission_CompletedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <EMISSIONS_CAPCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_Constructor_ZeroAddressesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall as alloy_sol_types::SolCall>::SIGNATURE,
            <failedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <changeFactorManagerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_InitializeEmissions_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <calculatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_SetChangeFactor_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <SCALECall as alloy_sol_types::SolCall>::SIGNATURE,
            <adminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <IS_TESTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <tokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testFuzz_MultipleEpochs_EmissionSumCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for EmissionsCalculatorTestCalls {
        const NAME: &'static str = "EmissionsCalculatorTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 43usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::EMISSIONS_CAP(_) => {
                    <EMISSIONS_CAPCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::SCALE(_) => <SCALECall as alloy_sol_types::SolCall>::SELECTOR,
                Self::TOTAL_EPOCHS(_) => {
                    <TOTAL_EPOCHSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::admin(_) => <adminCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::calculator(_) => {
                    <calculatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::changeFactorManager(_) => {
                    <changeFactorManagerCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::testFuzz_ChangeFactor_ValidRange(_) => {
                    <testFuzz_ChangeFactor_ValidRangeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testFuzz_MultipleEpochs_EmissionSum(_) => {
                    <testFuzz_MultipleEpochs_EmissionSumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_CalculateAndMintEmission_FinalEpoch(_) => {
                    <test_CalculateAndMintEmission_FinalEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_CalculateAndMintEmission_FirstEpoch(_) => {
                    <test_CalculateAndMintEmission_FirstEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_CalculateCumulativeProduct(_) => {
                    <test_CalculateCumulativeProductCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Constructor_InitialSetup(_) => {
                    <test_Constructor_InitialSetupCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Constructor_RoleAssignment(_) => {
                    <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_GetEmissionsInfo(_) => {
                    <test_GetEmissionsInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_GetRemainingSupply(_) => {
                    <test_GetRemainingSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_InitializeEmissions_Success(_) => {
                    <test_InitializeEmissions_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Integration_FullEmissionCycle(_) => {
                    <test_Integration_FullEmissionCycleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_PreviewCurrentEmission(_) => {
                    <test_PreviewCurrentEmissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_CalculateAndMintEmission_Completed(_) => {
                    <test_RevertWhen_CalculateAndMintEmission_CompletedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_CalculateAndMintEmission_NotInitialized(_) => {
                    <test_RevertWhen_CalculateAndMintEmission_NotInitializedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_CalculateAndMintEmission_ZeroAddress(_) => {
                    <test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_Constructor_ZeroAddresses(_) => {
                    <test_RevertWhen_Constructor_ZeroAddressesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_InitializeEmissions_AlreadyInitialized(_) => {
                    <test_RevertWhen_InitializeEmissions_AlreadyInitializedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_InitializeEmissions_InvalidChangeFactor(_) => {
                    <test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_InitializeEmissions_NotAdmin(_) => {
                    <test_RevertWhen_InitializeEmissions_NotAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_SetChangeFactor_Success(_) => {
                    <test_SetChangeFactor_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_SetChangeFactor_Zero(_) => {
                    <test_SetChangeFactor_ZeroCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::token(_) => <tokenCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::treasury(_) => <treasuryCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls>] = &[
                {
                    fn test_RevertWhen_CalculateAndMintEmission_NotInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_CalculateAndMintEmission_NotInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_CalculateAndMintEmission_NotInitialized,
                            )
                    }
                    test_RevertWhen_CalculateAndMintEmission_NotInitialized
                },
                {
                    fn test_CalculateAndMintEmission_FirstEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_CalculateAndMintEmission_FirstEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_CalculateAndMintEmission_FirstEpoch,
                            )
                    }
                    test_CalculateAndMintEmission_FirstEpoch
                },
                {
                    fn test_GetRemainingSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_GetRemainingSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::test_GetRemainingSupply)
                    }
                    test_GetRemainingSupply
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EmissionsCalculatorTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn test_RevertWhen_InitializeEmissions_AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_InitializeEmissions_AlreadyInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_InitializeEmissions_AlreadyInitialized,
                            )
                    }
                    test_RevertWhen_InitializeEmissions_AlreadyInitialized
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn test_CalculateCumulativeProduct(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_CalculateCumulativeProductCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_CalculateCumulativeProduct,
                            )
                    }
                    test_CalculateCumulativeProduct
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn test_Integration_FullEmissionCycle(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_Integration_FullEmissionCycleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_Integration_FullEmissionCycle,
                            )
                    }
                    test_Integration_FullEmissionCycle
                },
                {
                    fn user(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <userCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EmissionsCalculatorTestCalls::user)
                    }
                    user
                },
                {
                    fn test_GetEmissionsInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_GetEmissionsInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::test_GetEmissionsInfo)
                    }
                    test_GetEmissionsInfo
                },
                {
                    fn TOTAL_EPOCHS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <TOTAL_EPOCHSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::TOTAL_EPOCHS)
                    }
                    TOTAL_EPOCHS
                },
                {
                    fn test_CalculateAndMintEmission_FinalEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_CalculateAndMintEmission_FinalEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_CalculateAndMintEmission_FinalEpoch,
                            )
                    }
                    test_CalculateAndMintEmission_FinalEpoch
                },
                {
                    fn treasury(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <treasuryCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EmissionsCalculatorTestCalls::treasury)
                    }
                    treasury
                },
                {
                    fn test_RevertWhen_InitializeEmissions_InvalidChangeFactor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_InitializeEmissions_InvalidChangeFactor,
                            )
                    }
                    test_RevertWhen_InitializeEmissions_InvalidChangeFactor
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn testFuzz_ChangeFactor_ValidRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <testFuzz_ChangeFactor_ValidRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::testFuzz_ChangeFactor_ValidRange,
                            )
                    }
                    testFuzz_ChangeFactor_ValidRange
                },
                {
                    fn test_PreviewCurrentEmission(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_PreviewCurrentEmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_PreviewCurrentEmission,
                            )
                    }
                    test_PreviewCurrentEmission
                },
                {
                    fn test_SetChangeFactor_Zero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_SetChangeFactor_ZeroCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::test_SetChangeFactor_Zero)
                    }
                    test_SetChangeFactor_Zero
                },
                {
                    fn test_Constructor_InitialSetup(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_Constructor_InitialSetupCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_Constructor_InitialSetup,
                            )
                    }
                    test_Constructor_InitialSetup
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn test_RevertWhen_InitializeEmissions_NotAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_InitializeEmissions_NotAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_InitializeEmissions_NotAdmin,
                            )
                    }
                    test_RevertWhen_InitializeEmissions_NotAdmin
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn test_RevertWhen_CalculateAndMintEmission_Completed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_CalculateAndMintEmission_CompletedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_CalculateAndMintEmission_Completed,
                            )
                    }
                    test_RevertWhen_CalculateAndMintEmission_Completed
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn EMISSIONS_CAP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <EMISSIONS_CAPCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::EMISSIONS_CAP)
                    }
                    EMISSIONS_CAP
                },
                {
                    fn test_RevertWhen_Constructor_ZeroAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_Constructor_ZeroAddressesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_Constructor_ZeroAddresses,
                            )
                    }
                    test_RevertWhen_Constructor_ZeroAddresses
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn test_RevertWhen_CalculateAndMintEmission_ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_CalculateAndMintEmission_ZeroAddress,
                            )
                    }
                    test_RevertWhen_CalculateAndMintEmission_ZeroAddress
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EmissionsCalculatorTestCalls::failed)
                    }
                    failed
                },
                {
                    fn changeFactorManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <changeFactorManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::changeFactorManager)
                    }
                    changeFactorManager
                },
                {
                    fn test_InitializeEmissions_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_InitializeEmissions_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_InitializeEmissions_Success,
                            )
                    }
                    test_InitializeEmissions_Success
                },
                {
                    fn calculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <calculatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::calculator)
                    }
                    calculator
                },
                {
                    fn test_SetChangeFactor_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_SetChangeFactor_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_SetChangeFactor_Success,
                            )
                    }
                    test_SetChangeFactor_Success
                },
                {
                    fn test_Constructor_RoleAssignment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_Constructor_RoleAssignment,
                            )
                    }
                    test_Constructor_RoleAssignment
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn SCALE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <SCALECall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EmissionsCalculatorTestCalls::SCALE)
                    }
                    SCALE
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EmissionsCalculatorTestCalls::admin)
                    }
                    admin
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EmissionsCalculatorTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn token(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <tokenCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EmissionsCalculatorTestCalls::token)
                    }
                    token
                },
                {
                    fn testFuzz_MultipleEpochs_EmissionSum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <testFuzz_MultipleEpochs_EmissionSumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::testFuzz_MultipleEpochs_EmissionSum,
                            )
                    }
                    testFuzz_MultipleEpochs_EmissionSum
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
            ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls>] = &[
                {
                    fn test_RevertWhen_CalculateAndMintEmission_NotInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_CalculateAndMintEmission_NotInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_CalculateAndMintEmission_NotInitialized,
                            )
                    }
                    test_RevertWhen_CalculateAndMintEmission_NotInitialized
                },
                {
                    fn test_CalculateAndMintEmission_FirstEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_CalculateAndMintEmission_FirstEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_CalculateAndMintEmission_FirstEpoch,
                            )
                    }
                    test_CalculateAndMintEmission_FirstEpoch
                },
                {
                    fn test_GetRemainingSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_GetRemainingSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::test_GetRemainingSupply)
                    }
                    test_GetRemainingSupply
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn test_RevertWhen_InitializeEmissions_AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_InitializeEmissions_AlreadyInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_InitializeEmissions_AlreadyInitialized,
                            )
                    }
                    test_RevertWhen_InitializeEmissions_AlreadyInitialized
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn test_CalculateCumulativeProduct(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_CalculateCumulativeProductCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_CalculateCumulativeProduct,
                            )
                    }
                    test_CalculateCumulativeProduct
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn test_Integration_FullEmissionCycle(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_Integration_FullEmissionCycleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_Integration_FullEmissionCycle,
                            )
                    }
                    test_Integration_FullEmissionCycle
                },
                {
                    fn user(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <userCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::user)
                    }
                    user
                },
                {
                    fn test_GetEmissionsInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_GetEmissionsInfoCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::test_GetEmissionsInfo)
                    }
                    test_GetEmissionsInfo
                },
                {
                    fn TOTAL_EPOCHS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <TOTAL_EPOCHSCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::TOTAL_EPOCHS)
                    }
                    TOTAL_EPOCHS
                },
                {
                    fn test_CalculateAndMintEmission_FinalEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_CalculateAndMintEmission_FinalEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_CalculateAndMintEmission_FinalEpoch,
                            )
                    }
                    test_CalculateAndMintEmission_FinalEpoch
                },
                {
                    fn treasury(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <treasuryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::treasury)
                    }
                    treasury
                },
                {
                    fn test_RevertWhen_InitializeEmissions_InvalidChangeFactor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_InitializeEmissions_InvalidChangeFactor,
                            )
                    }
                    test_RevertWhen_InitializeEmissions_InvalidChangeFactor
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn testFuzz_ChangeFactor_ValidRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <testFuzz_ChangeFactor_ValidRangeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::testFuzz_ChangeFactor_ValidRange,
                            )
                    }
                    testFuzz_ChangeFactor_ValidRange
                },
                {
                    fn test_PreviewCurrentEmission(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_PreviewCurrentEmissionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_PreviewCurrentEmission,
                            )
                    }
                    test_PreviewCurrentEmission
                },
                {
                    fn test_SetChangeFactor_Zero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_SetChangeFactor_ZeroCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::test_SetChangeFactor_Zero)
                    }
                    test_SetChangeFactor_Zero
                },
                {
                    fn test_Constructor_InitialSetup(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_Constructor_InitialSetupCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_Constructor_InitialSetup,
                            )
                    }
                    test_Constructor_InitialSetup
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn test_RevertWhen_InitializeEmissions_NotAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_InitializeEmissions_NotAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_InitializeEmissions_NotAdmin,
                            )
                    }
                    test_RevertWhen_InitializeEmissions_NotAdmin
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn test_RevertWhen_CalculateAndMintEmission_Completed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_CalculateAndMintEmission_CompletedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_CalculateAndMintEmission_Completed,
                            )
                    }
                    test_RevertWhen_CalculateAndMintEmission_Completed
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn EMISSIONS_CAP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <EMISSIONS_CAPCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::EMISSIONS_CAP)
                    }
                    EMISSIONS_CAP
                },
                {
                    fn test_RevertWhen_Constructor_ZeroAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_Constructor_ZeroAddressesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_Constructor_ZeroAddresses,
                            )
                    }
                    test_RevertWhen_Constructor_ZeroAddresses
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn test_RevertWhen_CalculateAndMintEmission_ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_RevertWhen_CalculateAndMintEmission_ZeroAddress,
                            )
                    }
                    test_RevertWhen_CalculateAndMintEmission_ZeroAddress
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::failed)
                    }
                    failed
                },
                {
                    fn changeFactorManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <changeFactorManagerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::changeFactorManager)
                    }
                    changeFactorManager
                },
                {
                    fn test_InitializeEmissions_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_InitializeEmissions_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_InitializeEmissions_Success,
                            )
                    }
                    test_InitializeEmissions_Success
                },
                {
                    fn calculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <calculatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::calculator)
                    }
                    calculator
                },
                {
                    fn test_SetChangeFactor_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_SetChangeFactor_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_SetChangeFactor_Success,
                            )
                    }
                    test_SetChangeFactor_Success
                },
                {
                    fn test_Constructor_RoleAssignment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::test_Constructor_RoleAssignment,
                            )
                    }
                    test_Constructor_RoleAssignment
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn SCALE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <SCALECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::SCALE)
                    }
                    SCALE
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::admin)
                    }
                    admin
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn token(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <tokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EmissionsCalculatorTestCalls::token)
                    }
                    token
                },
                {
                    fn testFuzz_MultipleEpochs_EmissionSum(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EmissionsCalculatorTestCalls> {
                        <testFuzz_MultipleEpochs_EmissionSumCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                EmissionsCalculatorTestCalls::testFuzz_MultipleEpochs_EmissionSum,
                            )
                    }
                    testFuzz_MultipleEpochs_EmissionSum
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
                Self::EMISSIONS_CAP(inner) => {
                    <EMISSIONS_CAPCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::SCALE(inner) => {
                    <SCALECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::TOTAL_EPOCHS(inner) => {
                    <TOTAL_EPOCHSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::admin(inner) => {
                    <adminCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::calculator(inner) => {
                    <calculatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::changeFactorManager(inner) => {
                    <changeFactorManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::testFuzz_ChangeFactor_ValidRange(inner) => {
                    <testFuzz_ChangeFactor_ValidRangeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testFuzz_MultipleEpochs_EmissionSum(inner) => {
                    <testFuzz_MultipleEpochs_EmissionSumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_CalculateAndMintEmission_FinalEpoch(inner) => {
                    <test_CalculateAndMintEmission_FinalEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_CalculateAndMintEmission_FirstEpoch(inner) => {
                    <test_CalculateAndMintEmission_FirstEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_CalculateCumulativeProduct(inner) => {
                    <test_CalculateCumulativeProductCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Constructor_InitialSetup(inner) => {
                    <test_Constructor_InitialSetupCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Constructor_RoleAssignment(inner) => {
                    <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_GetEmissionsInfo(inner) => {
                    <test_GetEmissionsInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_GetRemainingSupply(inner) => {
                    <test_GetRemainingSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_InitializeEmissions_Success(inner) => {
                    <test_InitializeEmissions_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Integration_FullEmissionCycle(inner) => {
                    <test_Integration_FullEmissionCycleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_PreviewCurrentEmission(inner) => {
                    <test_PreviewCurrentEmissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_CalculateAndMintEmission_Completed(inner) => {
                    <test_RevertWhen_CalculateAndMintEmission_CompletedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_CalculateAndMintEmission_NotInitialized(inner) => {
                    <test_RevertWhen_CalculateAndMintEmission_NotInitializedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_CalculateAndMintEmission_ZeroAddress(inner) => {
                    <test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_Constructor_ZeroAddresses(inner) => {
                    <test_RevertWhen_Constructor_ZeroAddressesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_InitializeEmissions_AlreadyInitialized(inner) => {
                    <test_RevertWhen_InitializeEmissions_AlreadyInitializedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_InitializeEmissions_InvalidChangeFactor(inner) => {
                    <test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_InitializeEmissions_NotAdmin(inner) => {
                    <test_RevertWhen_InitializeEmissions_NotAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_SetChangeFactor_Success(inner) => {
                    <test_SetChangeFactor_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_SetChangeFactor_Zero(inner) => {
                    <test_SetChangeFactor_ZeroCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::token(inner) => {
                    <tokenCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::treasury(inner) => {
                    <treasuryCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::user(inner) => {
                    <userCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::EMISSIONS_CAP(inner) => {
                    <EMISSIONS_CAPCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::SCALE(inner) => {
                    <SCALECall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::TOTAL_EPOCHS(inner) => {
                    <TOTAL_EPOCHSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::admin(inner) => {
                    <adminCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::calculator(inner) => {
                    <calculatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::changeFactorManager(inner) => {
                    <changeFactorManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::testFuzz_ChangeFactor_ValidRange(inner) => {
                    <testFuzz_ChangeFactor_ValidRangeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testFuzz_MultipleEpochs_EmissionSum(inner) => {
                    <testFuzz_MultipleEpochs_EmissionSumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_CalculateAndMintEmission_FinalEpoch(inner) => {
                    <test_CalculateAndMintEmission_FinalEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_CalculateAndMintEmission_FirstEpoch(inner) => {
                    <test_CalculateAndMintEmission_FirstEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_CalculateCumulativeProduct(inner) => {
                    <test_CalculateCumulativeProductCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_Constructor_InitialSetup(inner) => {
                    <test_Constructor_InitialSetupCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::test_GetEmissionsInfo(inner) => {
                    <test_GetEmissionsInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_GetRemainingSupply(inner) => {
                    <test_GetRemainingSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_InitializeEmissions_Success(inner) => {
                    <test_InitializeEmissions_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_Integration_FullEmissionCycle(inner) => {
                    <test_Integration_FullEmissionCycleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_PreviewCurrentEmission(inner) => {
                    <test_PreviewCurrentEmissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_CalculateAndMintEmission_Completed(inner) => {
                    <test_RevertWhen_CalculateAndMintEmission_CompletedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_CalculateAndMintEmission_NotInitialized(inner) => {
                    <test_RevertWhen_CalculateAndMintEmission_NotInitializedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_CalculateAndMintEmission_ZeroAddress(inner) => {
                    <test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_Constructor_ZeroAddresses(inner) => {
                    <test_RevertWhen_Constructor_ZeroAddressesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_InitializeEmissions_AlreadyInitialized(inner) => {
                    <test_RevertWhen_InitializeEmissions_AlreadyInitializedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_InitializeEmissions_InvalidChangeFactor(inner) => {
                    <test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_InitializeEmissions_NotAdmin(inner) => {
                    <test_RevertWhen_InitializeEmissions_NotAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_SetChangeFactor_Success(inner) => {
                    <test_SetChangeFactor_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_SetChangeFactor_Zero(inner) => {
                    <test_SetChangeFactor_ZeroCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::token(inner) => {
                    <tokenCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::treasury(inner) => {
                    <treasuryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::user(inner) => {
                    <userCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`EmissionsCalculatorTest`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum EmissionsCalculatorTestEvents {
        #[allow(missing_docs)]
        ChangeFactorSet(ChangeFactorSet),
        #[allow(missing_docs)]
        EmissionMinted(EmissionMinted),
        #[allow(missing_docs)]
        EmissionsInitialized(EmissionsInitialized),
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
    impl EmissionsCalculatorTestEvents {
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
                22u8, 15u8, 193u8, 149u8, 214u8, 229u8, 54u8, 145u8, 211u8, 13u8, 128u8,
                76u8, 225u8, 144u8, 220u8, 9u8, 71u8, 24u8, 145u8, 103u8, 126u8, 67u8,
                67u8, 59u8, 145u8, 167u8, 166u8, 19u8, 28u8, 18u8, 165u8, 154u8,
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
                184u8, 19u8, 255u8, 190u8, 56u8, 125u8, 108u8, 246u8, 230u8, 166u8,
                246u8, 197u8, 248u8, 144u8, 95u8, 118u8, 106u8, 15u8, 28u8, 108u8, 208u8,
                28u8, 103u8, 49u8, 47u8, 112u8, 147u8, 86u8, 198u8, 37u8, 151u8, 189u8,
            ],
            [
                193u8, 44u8, 96u8, 171u8, 194u8, 22u8, 40u8, 110u8, 242u8, 94u8, 52u8,
                177u8, 128u8, 90u8, 12u8, 61u8, 218u8, 115u8, 228u8, 194u8, 253u8, 108u8,
                243u8, 96u8, 232u8, 7u8, 167u8, 169u8, 231u8, 49u8, 103u8, 57u8,
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
            ::core::stringify!(log_string),
            ::core::stringify!(log_int),
            ::core::stringify!(EmissionMinted),
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
            ::core::stringify!(log_named_array_1),
            ::core::stringify!(log_named_bytes32),
            ::core::stringify!(log_named_uint),
            ::core::stringify!(ChangeFactorSet),
            ::core::stringify!(EmissionsInitialized),
            ::core::stringify!(log_named_bytes),
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
            <EmissionMinted as alloy_sol_types::SolEvent>::SIGNATURE,
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
            <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <ChangeFactorSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <EmissionsInitialized as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for EmissionsCalculatorTestEvents {
        const NAME: &'static str = "EmissionsCalculatorTestEvents";
        const COUNT: usize = 25usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ChangeFactorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ChangeFactorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ChangeFactorSet)
                }
                Some(<EmissionMinted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EmissionMinted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EmissionMinted)
                }
                Some(
                    <EmissionsInitialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EmissionsInitialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EmissionsInitialized)
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
    impl alloy_sol_types::private::IntoLogData for EmissionsCalculatorTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ChangeFactorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EmissionMinted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EmissionsInitialized(inner) => {
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
                Self::ChangeFactorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EmissionMinted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EmissionsInitialized(inner) => {
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
    /**Creates a new wrapper around an on-chain [`EmissionsCalculatorTest`](self) contract instance.

See the [wrapper's documentation](`EmissionsCalculatorTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> EmissionsCalculatorTestInstance<P, N> {
        EmissionsCalculatorTestInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<EmissionsCalculatorTestInstance<P, N>>,
    > {
        EmissionsCalculatorTestInstance::<P, N>::deploy(__provider)
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
        EmissionsCalculatorTestInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`EmissionsCalculatorTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EmissionsCalculatorTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EmissionsCalculatorTestInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for EmissionsCalculatorTestInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EmissionsCalculatorTestInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > EmissionsCalculatorTestInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`EmissionsCalculatorTest`](self) contract instance.

See the [wrapper's documentation](`EmissionsCalculatorTestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<EmissionsCalculatorTestInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> EmissionsCalculatorTestInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EmissionsCalculatorTestInstance<P, N> {
            EmissionsCalculatorTestInstance {
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
    > EmissionsCalculatorTestInstance<P, N> {
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
        ///Creates a new call builder for the [`EMISSIONS_CAP`] function.
        pub fn EMISSIONS_CAP(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, EMISSIONS_CAPCall, N> {
            self.call_builder(&EMISSIONS_CAPCall)
        }
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<&P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall)
        }
        ///Creates a new call builder for the [`SCALE`] function.
        pub fn SCALE(&self) -> alloy_contract::SolCallBuilder<&P, SCALECall, N> {
            self.call_builder(&SCALECall)
        }
        ///Creates a new call builder for the [`TOTAL_EPOCHS`] function.
        pub fn TOTAL_EPOCHS(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, TOTAL_EPOCHSCall, N> {
            self.call_builder(&TOTAL_EPOCHSCall)
        }
        ///Creates a new call builder for the [`admin`] function.
        pub fn admin(&self) -> alloy_contract::SolCallBuilder<&P, adminCall, N> {
            self.call_builder(&adminCall)
        }
        ///Creates a new call builder for the [`calculator`] function.
        pub fn calculator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, calculatorCall, N> {
            self.call_builder(&calculatorCall)
        }
        ///Creates a new call builder for the [`changeFactorManager`] function.
        pub fn changeFactorManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, changeFactorManagerCall, N> {
            self.call_builder(&changeFactorManagerCall)
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
        ///Creates a new call builder for the [`testFuzz_ChangeFactor_ValidRange`] function.
        pub fn testFuzz_ChangeFactor_ValidRange(
            &self,
            changeFactor: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testFuzz_ChangeFactor_ValidRangeCall,
            N,
        > {
            self.call_builder(
                &testFuzz_ChangeFactor_ValidRangeCall {
                    changeFactor,
                },
            )
        }
        ///Creates a new call builder for the [`testFuzz_MultipleEpochs_EmissionSum`] function.
        pub fn testFuzz_MultipleEpochs_EmissionSum(
            &self,
            epochs: u8,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testFuzz_MultipleEpochs_EmissionSumCall,
            N,
        > {
            self.call_builder(
                &testFuzz_MultipleEpochs_EmissionSumCall {
                    epochs,
                },
            )
        }
        ///Creates a new call builder for the [`test_CalculateAndMintEmission_FinalEpoch`] function.
        pub fn test_CalculateAndMintEmission_FinalEpoch(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_CalculateAndMintEmission_FinalEpochCall,
            N,
        > {
            self.call_builder(&test_CalculateAndMintEmission_FinalEpochCall)
        }
        ///Creates a new call builder for the [`test_CalculateAndMintEmission_FirstEpoch`] function.
        pub fn test_CalculateAndMintEmission_FirstEpoch(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_CalculateAndMintEmission_FirstEpochCall,
            N,
        > {
            self.call_builder(&test_CalculateAndMintEmission_FirstEpochCall)
        }
        ///Creates a new call builder for the [`test_CalculateCumulativeProduct`] function.
        pub fn test_CalculateCumulativeProduct(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_CalculateCumulativeProductCall, N> {
            self.call_builder(&test_CalculateCumulativeProductCall)
        }
        ///Creates a new call builder for the [`test_Constructor_InitialSetup`] function.
        pub fn test_Constructor_InitialSetup(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_Constructor_InitialSetupCall, N> {
            self.call_builder(&test_Constructor_InitialSetupCall)
        }
        ///Creates a new call builder for the [`test_Constructor_RoleAssignment`] function.
        pub fn test_Constructor_RoleAssignment(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_Constructor_RoleAssignmentCall, N> {
            self.call_builder(&test_Constructor_RoleAssignmentCall)
        }
        ///Creates a new call builder for the [`test_GetEmissionsInfo`] function.
        pub fn test_GetEmissionsInfo(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_GetEmissionsInfoCall, N> {
            self.call_builder(&test_GetEmissionsInfoCall)
        }
        ///Creates a new call builder for the [`test_GetRemainingSupply`] function.
        pub fn test_GetRemainingSupply(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_GetRemainingSupplyCall, N> {
            self.call_builder(&test_GetRemainingSupplyCall)
        }
        ///Creates a new call builder for the [`test_InitializeEmissions_Success`] function.
        pub fn test_InitializeEmissions_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_InitializeEmissions_SuccessCall,
            N,
        > {
            self.call_builder(&test_InitializeEmissions_SuccessCall)
        }
        ///Creates a new call builder for the [`test_Integration_FullEmissionCycle`] function.
        pub fn test_Integration_FullEmissionCycle(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_Integration_FullEmissionCycleCall,
            N,
        > {
            self.call_builder(&test_Integration_FullEmissionCycleCall)
        }
        ///Creates a new call builder for the [`test_PreviewCurrentEmission`] function.
        pub fn test_PreviewCurrentEmission(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_PreviewCurrentEmissionCall, N> {
            self.call_builder(&test_PreviewCurrentEmissionCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_CalculateAndMintEmission_Completed`] function.
        pub fn test_RevertWhen_CalculateAndMintEmission_Completed(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_CalculateAndMintEmission_CompletedCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_CalculateAndMintEmission_CompletedCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_CalculateAndMintEmission_NotInitialized`] function.
        pub fn test_RevertWhen_CalculateAndMintEmission_NotInitialized(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_CalculateAndMintEmission_NotInitializedCall,
            N,
        > {
            self.call_builder(
                &test_RevertWhen_CalculateAndMintEmission_NotInitializedCall,
            )
        }
        ///Creates a new call builder for the [`test_RevertWhen_CalculateAndMintEmission_ZeroAddress`] function.
        pub fn test_RevertWhen_CalculateAndMintEmission_ZeroAddress(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_CalculateAndMintEmission_ZeroAddressCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_Constructor_ZeroAddresses`] function.
        pub fn test_RevertWhen_Constructor_ZeroAddresses(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_Constructor_ZeroAddressesCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_Constructor_ZeroAddressesCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_InitializeEmissions_AlreadyInitialized`] function.
        pub fn test_RevertWhen_InitializeEmissions_AlreadyInitialized(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_InitializeEmissions_AlreadyInitializedCall,
            N,
        > {
            self.call_builder(
                &test_RevertWhen_InitializeEmissions_AlreadyInitializedCall,
            )
        }
        ///Creates a new call builder for the [`test_RevertWhen_InitializeEmissions_InvalidChangeFactor`] function.
        pub fn test_RevertWhen_InitializeEmissions_InvalidChangeFactor(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall,
            N,
        > {
            self.call_builder(
                &test_RevertWhen_InitializeEmissions_InvalidChangeFactorCall,
            )
        }
        ///Creates a new call builder for the [`test_RevertWhen_InitializeEmissions_NotAdmin`] function.
        pub fn test_RevertWhen_InitializeEmissions_NotAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_InitializeEmissions_NotAdminCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_InitializeEmissions_NotAdminCall)
        }
        ///Creates a new call builder for the [`test_SetChangeFactor_Success`] function.
        pub fn test_SetChangeFactor_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_SetChangeFactor_SuccessCall, N> {
            self.call_builder(&test_SetChangeFactor_SuccessCall)
        }
        ///Creates a new call builder for the [`test_SetChangeFactor_Zero`] function.
        pub fn test_SetChangeFactor_Zero(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_SetChangeFactor_ZeroCall, N> {
            self.call_builder(&test_SetChangeFactor_ZeroCall)
        }
        ///Creates a new call builder for the [`token`] function.
        pub fn token(&self) -> alloy_contract::SolCallBuilder<&P, tokenCall, N> {
            self.call_builder(&tokenCall)
        }
        ///Creates a new call builder for the [`treasury`] function.
        pub fn treasury(&self) -> alloy_contract::SolCallBuilder<&P, treasuryCall, N> {
            self.call_builder(&treasuryCall)
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
    > EmissionsCalculatorTestInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ChangeFactorSet`] event.
        pub fn ChangeFactorSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, ChangeFactorSet, N> {
            self.event_filter::<ChangeFactorSet>()
        }
        ///Creates a new event filter for the [`EmissionMinted`] event.
        pub fn EmissionMinted_filter(
            &self,
        ) -> alloy_contract::Event<&P, EmissionMinted, N> {
            self.event_filter::<EmissionMinted>()
        }
        ///Creates a new event filter for the [`EmissionsInitialized`] event.
        pub fn EmissionsInitialized_filter(
            &self,
        ) -> alloy_contract::Event<&P, EmissionsInitialized, N> {
            self.event_filter::<EmissionsInitialized>()
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
