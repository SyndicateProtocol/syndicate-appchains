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

interface OptimismBridgeProxyTest {
    event BridgeExecuted(address indexed token, uint256 amount, address indexed target);
    event OptimismConfigUpdated(address l2Token, address recipient, uint32 l2Gas);
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
    function l2Gas() external view returns (uint32);
    function l2Token() external view returns (address);
    function optimismBridge() external view returns (address);
    function recipient() external view returns (address);
    function setUp() external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testFuzz_ExecuteBridge_CustomRecipient(address customRecipient) external;
    function testFuzz_ExecuteBridge_ValidAmounts(uint256 amount) external;
    function testFuzz_SetOptimismConfig_ValidGas(uint256 gasLimit) external;
    function test_Constructor_BridgeInfo() external view;
    function test_Constructor_RoleAssignment() external view;
    function test_Constructor_Success() external view;
    function test_ExecuteBridge_Success_CustomParams() external;
    function test_ExecuteBridge_Success_DefaultParams() external;
    function test_ExecuteBridge_Success_MultipleTransfers() external;
    function test_ExecuteBridge_TokenApprovalHandling() external;
    function test_GetOptimismConfig() external view;
    function test_Integration_DailyLimitReset() external;
    function test_Integration_FullBridgeFlow() external;
    function test_RevertWhen_ExecuteBridge_OptimismBridgeFails() external;
    function test_RevertWhen_SetOptimismConfig_NotAdmin() external;
    function test_SetOptimismConfig_Success() external;
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
        "internalType": "contract OptimismBridgeProxy"
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
    "name": "l2Gas",
    "inputs": [],
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
    "name": "l2Token",
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
    "name": "optimismBridge",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockOptimismBridge"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "recipient",
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
    "name": "testFuzz_ExecuteBridge_CustomRecipient",
    "inputs": [
      {
        "name": "customRecipient",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "testFuzz_SetOptimismConfig_ValidGas",
    "inputs": [
      {
        "name": "gasLimit",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_Constructor_BridgeInfo",
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
    "name": "test_Constructor_Success",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "test_ExecuteBridge_Success_CustomParams",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_ExecuteBridge_Success_DefaultParams",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_ExecuteBridge_Success_MultipleTransfers",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_ExecuteBridge_TokenApprovalHandling",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_GetOptimismConfig",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "test_Integration_DailyLimitReset",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_Integration_FullBridgeFlow",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_ExecuteBridge_OptimismBridgeFails",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_RevertWhen_SetOptimismConfig_NotAdmin",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_SetOptimismConfig_Success",
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
    "name": "OptimismConfigUpdated",
    "inputs": [
      {
        "name": "l2Token",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "recipient",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "l2Gas",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
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
pub mod OptimismBridgeProxyTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60808060405234609857600c805460ff199081166001908117909255601f80549091169091179055602280546001600160a01b031990811661123417909155602380548216615678179055602480548216619abc1790556025805490911661def0179055602680546001600160c01b03191676030d400000000000000000000000000000000000001111179055618ee1908161009d8239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f905f3560e01c9081630a9254e414614bd0575080630d170b02146147c65780630d1ce0dd14614472578063117e3b421461444d5780631ed7831c146143cf578063248ec326146143a95780632ade3880146141b55780632cd38fbf146140aa578063374e0ce614613d515780633cbb697914613d2a5780633e5e3c2314613cac5780633f7286f414613c2e5780634f8632ba14613c0757806352743ec41461375f57806356eff2671461373857806366d003ac1461371157806366d9a9a0146135d4578063702877781461335c5780637b4d4ce31461333657806385226c81146132ac578063916a17c614613202578063a3d4485b146131d8578063b0464fdc1461312e578063b44dc9d614612cfe578063b5508aa914612c74578063ba414fa614612c4f578063ccc0cfac14612688578063cffb048b14612267578063d8b296da14611d45578063dccc57f1146119f4578063e178bc5b1461182a578063e20c9f711461179c578063f2067bd71461140a578063f697e78a14610c73578063f81006b214610782578063f851a4401461075b578063fa7626d414610738578063fc0c546a14610711578063fc9c8d39146106ea5763fe47a3f4146101d6575f80fd5b34610603578060031936011261060357806040516122226020820152620493e060408201526040815261020a606082615058565b6001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610634576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156106295783916106d5575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b30000000000000000000000000000000000000000000000000000000084526004840152693f870857a3e0e380000060248401525af180156106295761069e575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610634576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610629578391610689575b50506001600160a01b03601f5460081c16906001600160a01b036021541690823b15610684576103c1928492836040518096819582947f18b68b8c0000000000000000000000000000000000000000000000000000000084526004840152693f870857a3e0e38000006024840152606060448401526064830190614e90565b03925af180156106065761066f575b506004816001600160a01b0360205416604051928380927f530660690000000000000000000000000000000000000000000000000000000082525afa90811561060657829161064d575b5061043b6001600160a01b038251166001600160a01b0360215416906159c1565b61045e6001600160a01b036020830151166001600160a01b0360255416906159c1565b6001600160a01b03604082015116737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561063457604051907f515361f6000000000000000000000000000000000000000000000000000000008252600482015261222260248201528281604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115610629578391610638575b50506060810151737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561063457604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152693f870857a3e0e380000060248201528281604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115610629578391610614575b50506080015163ffffffff16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561061157604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152620493e060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610606576105f25750f35b816105fc91615058565b6106035780f35b80fd5b6040513d84823e3d90fd5b50fd5b8161061e91615058565b61061157815f61056d565b6040513d85823e3d90fd5b5050fd5b8161064291615058565b61061157815f6104e5565b61066991503d8084833e6106618183615058565b810190615601565b5f61041a565b8161067991615058565b61060357805f6103d0565b505050fd5b8161069391615058565b61061157815f610342565b6020813d6020116106cd575b816106b760209383615058565b81010312610634576106c8906150d3565b6102dc565b3d91506106aa565b816106df91615058565b61061157815f61026f565b503461060357806003193601126106035760206001600160a01b0360235416604051908152f35b503461060357806003193601126106035760206001600160a01b0360215416604051908152f35b5034610603578060031936011261060357602060ff601f54166040519015158152f35b503461060357806003193601126106035760206001600160a01b0360225416604051908152f35b50346106035780600319360112610603578060206040516107a38282615058565b8281526001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610684576040519063ca669fa760e01b82526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610c0c578491610c5e575b50826001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526969e10de76676d080000060248401525af18015610c0c57610c2c575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610634576040517f491cc7c2000000000000000000000000000000000000000000000000000000008152600160048201528360248201526001604482015260016064820152838160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610c0c578491610c17575b50506001600160a01b03602154166001600160a01b03835416907f3dba401dc1abbf017086864b33ba815f53ab3cdbb9366ba6bcd8ec342ddd98e8846040516969e10de76676d08000008152a36001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610684576040519063ca669fa760e01b82526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610c0c578491610bf7575b50506001600160a01b03601f5460081c16906001600160a01b036021541690823b15610bf357610a30928592836040518096819582947f18b68b8c00000000000000000000000000000000000000000000000000000000845260048401526969e10de76676d08000006024840152606060448401526064830190614e90565b03925af1908115610629578391610bde575b506004906001600160a01b03835416604051928380927f530660690000000000000000000000000000000000000000000000000000000082525afa80156106295760a0918491610bc4575b50610aae6001600160a01b038251166001600160a01b0360215416906159c1565b610ad06001600160a01b0384830151166001600160a01b0360255416906159c1565b610b1b6001600160a01b03604083015116610af8602654916001600160a01b038316906159c1565b610b056060840151615755565b63ffffffff8060808501511691851c169061594b565b0151610b2a6040519283615058565b828252737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561063457610b8c8391610b9e60405194859384937f97624631000000000000000000000000000000000000000000000000000000008552604060048601526044850190614e90565b90600319848303016024850152614e90565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610606576105f25750f35b610bd891503d8086833e6106618183615058565b5f610a8d565b81610be891615058565b61061157815f610a42565b8480fd5b81610c0191615058565b61063457825f6109b1565b6040513d86823e3d90fd5b81610c2191615058565b61063457825f6108ff565b8281813d8311610c57575b610c418183615058565b8101031261068457610c52906150d3565b610877565b503d610c37565b81610c6891615058565b61063457825f61080b565b503461060357806003193601126106035760405161555560208201526202bf20604082015260408152610ca7606082615058565b6001600160a01b0360215416826001600160a01b036023541692604051937f70a08231000000000000000000000000000000000000000000000000000000008552806004860152602085602481875afa9485156106295783956113d3575b5060206001600160a01b038154166024604051809781937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa93841561062957839461139c575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561132f576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610629578391611387575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b30000000000000000000000000000000000000000000000000000000084526004840152699ed194db19b238c0000060248401525af1801561062957611350575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561132f576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610629578391611337575b50506001600160a01b03601f5460081c16906001600160a01b036021541690823b1561133357610f00928492836040518096819582947f18b68b8c0000000000000000000000000000000000000000000000000000000084526004840152699ed194db19b238c000006024840152606060448401526064830190614e90565b03925af180156106065761131a575b50506001600160a01b0360215416916001600160a01b0360235416604051907f70a082310000000000000000000000000000000000000000000000000000000082526004820152602081602481875afa9081156112dd5785916112e8575b507fffffffffffffffffffffffffffffffffffffffffffff612e6b24e64dc7400000820191821161127e5790610fa29161594b565b6001600160a01b0360205416906040517f70a08231000000000000000000000000000000000000000000000000000000008152826004820152602081602481875afa9081156112dd5785916112ab575b50699ed194db19b238c00000820180921161127e57600492611017869593869361594b565b604051938480927f530660690000000000000000000000000000000000000000000000000000000082525afa91821561062957839261125e575b50611066906001600160a01b038351166159c1565b6110896001600160a01b036020830151166001600160a01b0360255416906159c1565b6001600160a01b03604082015116737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561063457604051907f515361f6000000000000000000000000000000000000000000000000000000008252600482015261555560248201528281604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115610629578391611249575b5050608081611127606063ffffffff9401516158cb565b015116737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561061157604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526202bf2060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561060657611234575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa80156106065782906111fc575b6111f991506158cb565b80f35b506020813d60201161122c575b8161121660209383615058565b81010312611228576111f990516111ef565b5f80fd5b3d9150611209565b8161123e91615058565b61060357805f6111a1565b8161125391615058565b61061157815f611110565b611066919250611277903d8086833e6106618183615058565b9190611051565b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d6020116112d5575b816112c660209383615058565b8101031261122857515f610ff2565b3d91506112b9565b6040513d87823e3d90fd5b90506020813d602011611312575b8161130360209383615058565b8101031261122857515f610f6d565b3d91506112f6565b8161132491615058565b61132f57825f610f0f565b8280fd5b8380fd5b8161134191615058565b61134c57815f610e81565b5080fd5b6020813d60201161137f575b8161136960209383615058565b8101031261132f5761137a906150d3565b610e1b565b3d915061135c565b8161139191615058565b61134c57815f610dae565b925092506020823d6020116113cb575b816113b960209383615058565b8101031261122857849151925f610d54565b3d91506113ac565b925093506020823d602011611402575b816113f060209383615058565b8101031261122857849151935f610d05565b3d91506113e3565b503461060357602060031936011261060357600435816001600160a01b03821680830361134c57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561134c576040517f4c63e56200000000000000000000000000000000000000000000000000000000815281151560048201528281602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115610629578391611787575b505063ffffffff60265460a01c166040519160208301526040820152604081526114cf606082615058565b6001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561132f576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610629578391611772575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b3000000000000000000000000000000000000000000000000000000008452600484015269152d02c7e14af680000060248401525af180156106295761173b575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561132f576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610629578391611726575b50506001600160a01b03601f5460081c16906001600160a01b036021541690823b1561133357611686928492836040518096819582947f18b68b8c000000000000000000000000000000000000000000000000000000008452600484015269152d02c7e14af68000006024840152606060448401526064830190614e90565b03925af1801561060657611711575b50906004916001600160a01b0360205416604051938480927f530660690000000000000000000000000000000000000000000000000000000082525afa80156106295760406001600160a01b03916111f99486916116f7575b500151166159c1565b61170b91503d8088833e6106618183615058565b5f6116ee565b8161171b91615058565b61134c57815f611695565b8161173091615058565b61134c57815f611607565b6020813d60201161176a575b8161175460209383615058565b8101031261132f57611765906150d3565b6115a1565b3d9150611747565b8161177c91615058565b61134c57815f611534565b8161179191615058565b61134c57815f6114a4565b503461060357806003193601126106035760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b81811061180b57611807856117fb81870382615058565b60405191829182614e4e565b0390f35b82546001600160a01b03168452602090930192600192830192016117e4565b50346106035780600319360112610603576004816001600160a01b03601f5460081c16604051928380927fede7cebd0000000000000000000000000000000000000000000000000000000082525afa801561060657828081938293611985575b50604080519161189a8284615058565b600f83527f4f7074696d69736d2042726964676500000000000000000000000000000000006020840152737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561133357610b8c8491611921845195869384937ff320d9630000000000000000000000000000000000000000000000000000000085528760048601526044850190614e90565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561197c5750611967575b50506119626111f9926001600160a01b0360205416906159c1565b615a42565b8161197191615058565b61132f57825f611947565b513d84823e3d90fd5b93505050503d8083833e6119998183615058565b810160608282031261132f57815167ffffffffffffffff81116113335782019080601f830112156113335781516119d29260200161559d565b90826119ec60406119e5602085016151fb565b93016150d3565b91925f61188a565b50346106035780600319360112610603576001600160a01b03601f5460081c166040517fa217fddf000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610629578391611d13575b506022546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b03166024820181905290602081604481865afa8015610c0c578490611cd8575b611ab49150615a42565b6040517f118c38c7000000000000000000000000000000000000000000000000000000008152602081600481865afa908115610c0c578491611ca4575b506040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101919091526001600160a01b03909116602482015260208180604481015b0381855afa8015610629578390611c69575b611b559150615a42565b6040517f3462fac3000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610629578391611c35575b506023546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529060209082908180604481015b03915afa8015610606578290611bfa575b6111f99150615a42565b506020813d602011611c2d575b81611c1460209383615058565b8101031261134c57611c286111f9916150d3565b611bf0565b3d9150611c07565b90506020813d602011611c61575b81611c5060209383615058565b810103126112285751611bdf611b92565b3d9150611c43565b506020813d602011611c9c575b81611c8360209383615058565b8101031261132f57611c97611b55916150d3565b611b4b565b3d9150611c76565b90506020813d602011611cd0575b81611cbf60209383615058565b810103126112285751611b39611af1565b3d9150611cb2565b506020813d602011611d0b575b81611cf260209383615058565b8101031261133357611d06611ab4916150d3565b611aaa565b3d9150611ce5565b90506020813d602011611d3d575b81611d2e60209383615058565b8101031261122857515f611a51565b3d9150611d21565b5034610603578060031936011261060357737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561060357806040517f491cc7c200000000000000000000000000000000000000000000000000000000815281600482015281602482015281604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657612252575b50507f802b8c7b24709b6c9c56179dceeb977cc7ac6fa4f15f84c99a8627abfd97cc3560405180611e2181906203d09060406060840193613333815261444460208201520152565b0390a1806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156106065761223d575b506001600160a01b03601f5460081c16803b15610611578160405180927f85931b74000000000000000000000000000000000000000000000000000000008252818381611eee60048201906203d09060406060840193613333815261444460208201520152565b03925af1801561060657612228575b506001600160a01b03601f5460081c166040517f56eff267000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156106295783916121ee575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610634576001600160a01b03604051917f515361f600000000000000000000000000000000000000000000000000000000835216600482015261333360248201528281604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa9081156106295783916121d9575b50506040517f66d003ac000000000000000000000000000000000000000000000000000000008152602081600481855afa90811561062957839161219f575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610634576001600160a01b03604051917f515361f600000000000000000000000000000000000000000000000000000000835216600482015261444460248201528281604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561062957839161218a575b50506020600491604051928380927f3cbb69790000000000000000000000000000000000000000000000000000000082525afa908115610606578291612150575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156106115763ffffffff604051917f98296c540000000000000000000000000000000000000000000000000000000083521660048201526203d09060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610606576105f25750f35b90506020813d602011612182575b8161216b60209383615058565b810103126106115761217c9061520f565b5f6120d0565b3d915061215e565b8161219491615058565b61061157815f61208f565b90506020813d6020116121d1575b816121ba60209383615058565b81010312610634576121cb906151fb565b5f61200c565b3d91506121ad565b816121e391615058565b61061157815f611fcd565b90506020813d602011612220575b8161220960209383615058565b810103126106345761221a906151fb565b5f611f4a565b3d91506121fc565b8161223291615058565b61060357805f611efd565b8161224791615058565b61060357805f611e87565b8161225c91615058565b61060357805f611dd9565b50346106035760206003193601126106035761229169d3c21bcecceda10000006001600435615aeb565b604090828083516122a28582615058565b600c81527f426f756e6420726573756c7400000000000000000000000000000000000000006020820152845161233f8161230d60208201947fb60e72cc0000000000000000000000000000000000000000000000000000000086528960248401526064830190614e90565b876044830152037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282615058565b51906a636f6e736f6c652e6c6f675afa50826001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561134c5783519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561261d57612673575b5050602154601f5483517f095ea7b30000000000000000000000000000000000000000000000000000000081526001600160a01b0360089290921c8216600482015260248101849052916020918391168187816044810103925af180156125fb5761263c575b50826001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561134c5783519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561261d57612627575b506001600160a01b03601f5460081c166001600160a01b0360215416813b1561132f57829160848392875194859384927f18b68b8c0000000000000000000000000000000000000000000000000000000084526004840152886024840152606060448401528160648401525af1801561261d57612608575b506004906001600160a01b03602054168451928380927f530660690000000000000000000000000000000000000000000000000000000082525afa9081156125fb578260606004959361254f9388916125e1575b50015161594b565b60206001600160a01b03601f5460081c168251948580927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa9081156125d8575083906125a4575b6111f9925061594b565b506020823d6020116125d0575b816125be60209383615058565b81010312611228576111f9915161259a565b3d91506125b1565b513d85823e3d90fd5b6125f591503d808a833e6106618183615058565b5f612547565b50505051903d90823e3d90fd5b8161261291615058565b61132f57825f6124f3565b84513d84823e3d90fd5b8161263191615058565b61132f57825f61247b565b6020813d60201161266b575b8161265560209383615058565b8101031261133357612666906150d3565b612418565b3d9150612648565b8161267d91615058565b61132f57825f6123b2565b5034610603578060031936011261060357806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657612c3a575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526a01a784379d99db4200000060248401525af1801561060657612c03575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657612bee575b506001600160a01b03601f5460081c166001600160a01b036021541690803b15610634576040517f18b68b8c0000000000000000000000000000000000000000000000000000000081526001600160a01b0392909216600483015269d3c21bcecceda10000006024830152606060448301525f606483015282908290608490829084905af1801561060657612bd9575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa8015610606578290612ba5575b6128b6915061584b565b620151804201804211612b78578190737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561061157604051907fe5d6bf020000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657612b63575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657612b4e575b506001600160a01b03601f5460081c166001600160a01b036021541690803b15610634576040517f18b68b8c0000000000000000000000000000000000000000000000000000000081526001600160a01b0392909216600483015269d3c21bcecceda10000006024830152606060448301525f606483015282908290608490829084905af1801561060657612b39575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa8015610606578290612b05575b612a7f915061584b565b600460206001600160a01b03815416604051928380927f371bed680000000000000000000000000000000000000000000000000000000082525afa8015610606578290612ad1575b6111f991506156d4565b506020813d602011612afd575b81612aeb60209383615058565b81010312611228576111f99051612ac7565b3d9150612ade565b506020813d602011612b31575b81612b1f60209383615058565b8101031261122857612a7f9051612a75565b3d9150612b12565b81612b4391615058565b61060357805f612a27565b81612b5891615058565b61060357805f612997565b81612b6d91615058565b61060357805f612934565b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b506020813d602011612bd1575b81612bbf60209383615058565b81010312611228576128b690516128ac565b3d9150612bb2565b81612be391615058565b61060357805f61285e565b81612bf891615058565b61060357805f6127ce565b6020813d602011612c32575b81612c1c60209383615058565b8101031261134c57612c2d906150d3565b61276a565b3d9150612c0f565b81612c4491615058565b61060357805f6126fc565b50346106035780600319360112610603576020612c6a6154c4565b6040519015158152f35b5034610603578060031936011261060357601954612c91816150e0565b91612c9f6040519384615058565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b838310612ce157604051806118078782614f28565b600160208192612cf0856150f8565b815201920192019190612ccc565b50346106035780600319360112610603576001600160a01b03601f5460081c166040517fc9f5b63e000000000000000000000000000000000000000000000000000000008152602081600481855afa80156106295783906130f3575b612d7191506001600160a01b0360205416906159c1565b6040517f56eff267000000000000000000000000000000000000000000000000000000008152602081600481855afa80156106295783906130b8575b612dc491506001600160a01b0360255416906159c1565b6040517f66d003ac000000000000000000000000000000000000000000000000000000008152602081600481855afa90811561062957839161307e575b50612e19602654916001600160a01b038316906159c1565b604051907f3cbb6979000000000000000000000000000000000000000000000000000000008252602082600481865afa918215610c0c578492613039575b509063ffffffff80612e6e9360a01c16911661594b565b6040517f36b089d8000000000000000000000000000000000000000000000000000000008152602081600481855afa8015610629578390613005575b612eb4915061584b565b816040517f67eeba0c000000000000000000000000000000000000000000000000000000008152602081600481865afa908115610606578291612fd0575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561134c57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526a0422ca8b0a00a42500000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561060657612fbb575b50506020600491604051928380927fead93c8f0000000000000000000000000000000000000000000000000000000082525afa8015610606578290611bfa576111f99150615a42565b81612fc591615058565b61134c57815f612f72565b9150506020813d602011612ffd575b81612fec60209383615058565b81010312611228578290515f612ef2565b3d9150612fdf565b506020813d602011613031575b8161301f60209383615058565b8101031261122857612eb49051612eaa565b3d9150613012565b91506020823d602011613076575b8161305460209383615058565b810103126113335763ffffffff8061306e612e6e9461520f565b935050612e57565b3d9150613047565b90506020813d6020116130b0575b8161309960209383615058565b8101031261132f576130aa906151fb565b5f612e01565b3d915061308c565b506020813d6020116130eb575b816130d260209383615058565b8101031261132f576130e6612dc4916151fb565b612dad565b3d91506130c5565b506020813d602011613126575b8161310d60209383615058565b8101031261132f57613121612d71916151fb565b612d5a565b3d9150613100565b5034610603578060031936011261060357601c5461314b816150e0565b916131596040519384615058565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b83831061319b57604051806118078782614fa5565b600260206001926040516131ae8161503c565b6001600160a01b0386541681526131c6858701615220565b83820152815201920192019190613186565b503461060357806003193601126106035760206001600160a01b03601f5460081c16604051908152f35b5034610603578060031936011261060357601d5461321f816150e0565b9161322d6040519384615058565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b83831061326f57604051806118078782614fa5565b600260206001926040516132828161503c565b6001600160a01b03865416815261329a858701615220565b8382015281520192019201919061325a565b5034610603578060031936011261060357601a546132c9816150e0565b916132d76040519384615058565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b83831061331957604051806118078782614f28565b600160208192613328856150f8565b815201920192019190613304565b503461060357806003193601126106035760206001600160a01b03815416604051908152f35b50346106035760206003193601126106035761338062989680615208600435615aeb565b604090828083516133918582615058565b600c81527f426f756e6420726573756c740000000000000000000000000000000000000000602082015284516133fc8161230d60208201947fb60e72cc0000000000000000000000000000000000000000000000000000000086528960248401526064830190614e90565b51906a636f6e736f6c652e6c6f675afa50826001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561134c5783519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561261d576135bf575b506001600160a01b03601f5460081c166001600160a01b03602554169263ffffffff6001600160a01b0360265416911693823b156113335785517f85931b740000000000000000000000000000000000000000000000000000000081526001600160a01b0391821660048201529116602482015263ffffffff841660448201529082908290606490829084905af1801561261d576135aa575b5050600460206001600160a01b03601f5460081c168451928380927f3cbb69790000000000000000000000000000000000000000000000000000000082525afa9283156125d857508392613567575b5063ffffffff6111f9921661594b565b91506020823d6020116135a2575b8161358260209383615058565b8101031261132f5763ffffffff61359b6111f99361520f565b9250613557565b3d9150613575565b816135b491615058565b61132f57825f613508565b816135c991615058565b61132f57825f61346f565b5034610603578060031936011261060357601b546135f1816150e0565b6135fe6040519182615058565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b8383106136d657868587604051928392602084019060208552518091526040840160408260051b8601019392905b82821061366b57505050500390f35b919360206136c6827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc06001959799849503018652885190836136b68351604084526040840190614e90565b9201519084818403910152614ed3565b960192019201859493919261365c565b600260206001926040516136e98161503c565b6136f2866150f8565b81526136ff858701615220565b8382015281520192019201919061362e565b503461060357806003193601126106035760206001600160a01b0360265416604051908152f35b503461060357806003193601126106035760206001600160a01b0360255416604051908152f35b5034610603578060031936011261060357806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657613bf2575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b3000000000000000000000000000000000000000000000000000000008452600484015269152d02c7e14af680000060248401525af1801561060657613bbb575b50602154601f54602080546040517fdd62ed3e0000000000000000000000000000000000000000000000000000000081526001600160a01b0360089490941c84166004820152908316602482015292909183916044918391165afa8015610606578290613b87575b6138b291506157d5565b806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657613b72575b506001600160a01b03601f5460081c166001600160a01b036021541690803b15610634576040517f18b68b8c0000000000000000000000000000000000000000000000000000000081526001600160a01b0392909216600483015269152d02c7e14af68000006024830152606060448301525f606483015282908290608490829084905af1801561060657613b5d575b50506001600160a01b0360215416816001600160a01b0360205416604051907f70a082310000000000000000000000000000000000000000000000000000000082526004820152602081602481865afa908115610606578291613b28575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561134c57604051907f98296c54000000000000000000000000000000000000000000000000000000008252600482015269152d02c7e14af680000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561060657613b13575b505060206001600160a01b03601f5460081c166024604051809481937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa8015610606578290613adf575b6111f991506157d5565b506020813d602011613b0b575b81613af960209383615058565b81010312611228576111f99051613ad5565b3d9150613aec565b81613b1d91615058565b61134c57815f613a82565b9150506020813d602011613b55575b81613b4460209383615058565b81010312611228578290515f613a03565b3d9150613b37565b81613b6791615058565b61060357805f6139a5565b81613b7c91615058565b61060357805f613915565b506020813d602011613bb3575b81613ba160209383615058565b81010312611228576138b290516138a8565b3d9150613b94565b6020813d602011613bea575b81613bd460209383615058565b8101031261134c57613be5906150d3565b613840565b3d9150613bc7565b81613bfc91615058565b61060357805f6137d3565b503461060357806003193601126106035760206001600160a01b0360245416604051908152f35b503461060357806003193601126106035760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110613c8d57611807856117fb81870382615058565b82546001600160a01b0316845260209093019260019283019201613c76565b503461060357806003193601126106035760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b818110613d0b57611807856117fb81870382615058565b82546001600160a01b0316845260209093019260019283019201613cf4565b5034610603578060031936011261060357602063ffffffff60265460a01c16604051908152f35b5034610603578060031936011261060357806001600160a01b0360205416803b15610611578180916024604051809481937f6813d787000000000000000000000000000000000000000000000000000000008352600160048401525af1801561060657614095575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657614080575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b3000000000000000000000000000000000000000000000000000000008452600484015269152d02c7e14af680000060248401525af1801561060657614049575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657614034575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561060357806040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601660248201527f4f7074696d69736d20627269646765206661696c6564000000000000000000006044820152818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156106065761401f575b506001600160a01b03601f5460081c166001600160a01b036021541690803b15610634576040517f18b68b8c0000000000000000000000000000000000000000000000000000000081526001600160a01b0392909216600483015269152d02c7e14af68000006024830152606060448301525f606483015282908290608490829084905af18015610606576105f25750f35b8161402991615058565b61060357805f613f8d565b8161403e91615058565b61060357805f613eed565b6020813d602011614078575b8161406260209383615058565b8101031261134c57614073906150d3565b613e89565b3d9150614055565b8161408a91615058565b61060357805f613e1c565b8161409f91615058565b61060357805f613db9565b5034610603578060031936011261060357600460606001600160a01b03601f5460081c16604051928380927f32d4f5040000000000000000000000000000000000000000000000000000000082525afa801561060657828392849261414d575b5063ffffffff6111f99361412b82936001600160a01b0360255416906159c1565b614142602654916001600160a01b038316906159c1565b60a01c16911661594b565b925050506060813d6060116141ad575b8161416a60609383615058565b8101031261134c578063ffffffff806141856111f9946151fb565b61412b6141a06040614199602088016151fb565b960161520f565b94955090925061410a9050565b3d915061415d565b5034610603578060031936011261060357601e546141d2816150e0565b6141df6040519182615058565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b8383106143205786858760405192839260208401906020855251809152604084019160408260051b8601019392815b83831061424b5786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b8281106142d75750505050506020806001929701930193019092869594929361423e565b9091929394602080614313837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa087600196030189528951614e90565b97019501939291016142b3565b60405161432c8161503c565b6001600160a01b038354168152600183018054614348816150e0565b916143566040519384615058565b8183528a526020808b20908b9084015b83821061438c57505050506001928260209283600295015281520192019201919061420f565b60016020819261439b866150f8565b815201930191019091614366565b503461060357806003193601126106035760206040516a0422ca8b0a00a4250000008152f35b503461060357806003193601126106035760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b81811061442e57611807856117fb81870382615058565b82546001600160a01b0316845260209093019260019283019201614417565b5034610603578060031936011261060357602060405169d3c21bcecceda10000008152f35b5034610603578060031936011261060357806001600160a01b0360245416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561061157604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610606576147b1575b50600460206001600160a01b03601f5460081c16604051928380927f118c38c70000000000000000000000000000000000000000000000000000000082525afa90811561060657829161477c575b506145d76001600160a01b03602454166145ab6040519384927fe2517d3f00000000000000000000000000000000000000000000000000000000602085015260248401602090939291936001600160a01b0360408201951681520152565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282615058565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611578161463291604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190614e90565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657614767575b506001600160a01b03601f5460081c166001600160a01b0360255416602654823b15610684576040517f85931b740000000000000000000000000000000000000000000000000000000081526001600160a01b039283166004820152918116602483015260a01c63ffffffff16604482015290829082908183816064810103925af1801561060657614752575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561060357806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610606576105f25750f35b8161475c91615058565b61060357805f6146e4565b8161477191615058565b61060357805f614657565b9150506020813d6020116147a9575b8161479860209383615058565b81010312611228578190515f61454d565b3d915061478b565b816147bb91615058565b61060357805f6144ff565b5034610603578060031936011261060357806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561061157604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657614bbb575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526969e10de76676d080000060248401525af1801561060657614b84575b50806001600160a01b03601f5460081c166001600160a01b0360215416813b156106345782916084839260405194859384927f18b68b8c0000000000000000000000000000000000000000000000000000000084526004840152692a5a058fc295ed0000006024840152606060448401528160648401525af1801561060657614b6f575b506001600160a01b03601f5460081c166001600160a01b0360215416813b156106345782916084839260405194859384927f18b68b8c0000000000000000000000000000000000000000000000000000000084526004840152693f870857a3e0e38000006024840152606060448401528160648401525af1801561060657614b5a575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561060357806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657614b45575b5050600460206001600160a01b03815416604051928380927f371bed680000000000000000000000000000000000000000000000000000000082525afa8015610606578290614b11575b614a8791506156d4565b600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa8015610606578290614add575b6111f99150615755565b506020813d602011614b09575b81614af760209383615058565b81010312611228576111f99051614ad3565b3d9150614aea565b506020813d602011614b3d575b81614b2b60209383615058565b8101031261122857614a879051614a7d565b3d9150614b1e565b81614b4f91615058565b61060357805f614a33565b81614b6491615058565b61060357805f6149c7565b81614b7991615058565b61060357805f614944565b6020813d602011614bb3575b81614b9d60209383615058565b8101031261134c57614bae906150d3565b6148c0565b3d9150614b90565b81614bc591615058565b61060357805f614853565b905034611228575f60031936011261122857610c2c80820182811067ffffffffffffffff821117614e21578291615caf833903905ff08015614e16576001600160a01b03167fffffffffffffffffffffffff00000000000000000000000000000000000000006021541617602155604051610ab280820182811067ffffffffffffffff821117614e215782916168db833903905ff08015614e16576001600160a01b0316807fffffffffffffffffffffffff000000000000000000000000000000000000000060205416176020556001600160a01b03602254166001600160a01b0360235416906001600160a01b0360255416926026549360405194611b54918287019387851067ffffffffffffffff861117614e215761010096889663ffffffff9561738d893986526020860152604085015269d3c21bcecceda100000060608501526a0422ca8b0a00a425000000608085015260a08401526001600160a01b03811660c084015260a01c1660e08201520301905ff08015614e16577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f556001600160a01b03602154166001600160a01b0360235416813b15611228575f916044839260405194859384927f40c10f1900000000000000000000000000000000000000000000000000000000845260048401526a084595161401484a00000060248401525af18015614e1657614e08575080f35b614e1491505f90615058565b005b6040513d5f823e3d90fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60206040818301928281528451809452019201905f5b818110614e715750505090565b82516001600160a01b0316845260209384019390920191600101614e64565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b818110614ef05750505090565b82517fffffffff0000000000000000000000000000000000000000000000000000000016845260209384019390920191600101614ee3565b602081016020825282518091526040820191602060408360051b8301019401925f915b838310614f5a57505050505090565b9091929394602080614f96837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187528951614e90565b97019301930191939290614f4b565b602081016020825282518091526040820191602060408360051b8301019401925f915b838310614fd757505050505090565b909192939460208061502d837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b03815116845201519181858201520190614ed3565b97019301930191939290614fc8565b6040810190811067ffffffffffffffff821117614e2157604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117614e2157604052565b919082018092116150a657565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5190811515820361122857565b67ffffffffffffffff8111614e215760051b60200190565b90604051915f8154908160011c92600183169283156151f1575b6020851084146151c45784875286939081156151845750600114615140575b5061513e92500383615058565b565b90505f9291925260205f20905f915b81831061516857505090602061513e928201015f615131565b602091935080600191548385890101520191019091849261514f565b6020935061513e9592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f615131565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f1693615112565b51906001600160a01b038216820361122857565b519063ffffffff8216820361122857565b90604051918281549182825260208201905f5260205f20925f905b8060078301106154375761513e945491818110615401575b8181106153cb575b818110615395575b81811061535f575b818110615329575b8181106152f3575b8181106152be575b10615291575b500383615058565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f615289565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b168152019301615283565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b16815201930161527b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b168152019301615273565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b16815201930161526b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b168152019301615263565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b16815201930161525b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b168152019301615253565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e082015201940192018592939161523b565b60085460ff1680156154d35790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115614e16575f9161556b575b50151590565b90506020813d602011615595575b8161558660209383615058565b8101031261122857515f615565565b3d9150615579565b92919267ffffffffffffffff8211614e2157604051916155e5601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200184615058565b829481845281830111611228578281602093845f96015e010152565b6020818303126112285780519067ffffffffffffffff821161122857019060c082820312611228576040519160c0830183811067ffffffffffffffff821117614e2157604052615650816151fb565b835261565e602082016151fb565b602084015261566f604082016151fb565b60408401526060810151606084015261568a6080820161520f565b608084015260a08101519067ffffffffffffffff821161122857019080601f830112156112285781516156bf9260200161559d565b60a082015290565b919082039182116150a657565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152600260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b5f61513e91615058565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526969e10de76676d080000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051907f98296c54000000000000000000000000000000000000000000000000000000008252600482015269d3c21bcecceda100000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152699ed194db19b238c0000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051917f98296c54000000000000000000000000000000000000000000000000000000008352600483015260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611228576040517f515361f60000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152911660248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b8115615abe570690565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f90838311615c2a5782811091821580615c20575b615c1857615b0e84866156c7565b92600184018094116150a657600383111580615c0f575b615c005760031983101580615bf6575b615be25785831115615b9957505090615b5184615b56936156c7565b615ab4565b908115615b9457615b679250615099565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116150a65790565b505090565b959492919095615baa575b50505050565b83949550615b5190615bbc93946156c7565b908115615b9457615bcd92506156c7565b600181018091116150a657905f808080615ba4565b50509050615bf392915019906156c7565b90565b5082198411615b35565b5050919050615bf39250615099565b50828411615b25565b509250505090565b5084821115615b00565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603e60248201527f5374645574696c7320626f756e642875696e743235362c75696e743235362c7560448201527f696e74323536293a204d6178206973206c657373207468616e206d696e2e00006064820152fdfe60806040523461031357604080519081016001600160401b03811182821017610226576040908152600982526845524332304d6f636b60b81b602083015280519081016001600160401b038111828210176102265760405260048152634532304d60e01b602082015281516001600160401b03811161022657600354600181811c91168015610309575b602082101461020857601f81116102a6575b50602092601f821160011461024557928192935f9261023a575b50508160011b915f199060031b1c1916176003555b80516001600160401b03811161022657600454600181811c9116801561021c575b602082101461020857601f81116101a5575b50602091601f8211600114610145579181925f9261013a575b50508160011b915f199060031b1c1916176004555b60405161091490816103188239f35b015190505f80610116565b601f1982169260045f52805f20915f5b85811061018d57508360019510610175575b505050811b0160045561012b565b01515f1960f88460031b161c191690555f8080610167565b91926020600181928685015181550194019201610155565b60045f527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b601f830160051c810191602084106101fe575b601f0160051c01905b8181106101f357506100fd565b5f81556001016101e6565b90915081906101dd565b634e487b7160e01b5f52602260045260245ffd5b90607f16906100eb565b634e487b7160e01b5f52604160045260245ffd5b015190505f806100b5565b601f1982169360035f52805f20915f5b86811061028e5750836001959610610276575b505050811b016003556100ca565b01515f1960f88460031b161c191690555f8080610268565b91926020600181928685015181550194019201610255565b60035f527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b601f830160051c810191602084106102ff575b601f0160051c01905b8181106102f4575061009b565b5f81556001016102e7565b90915081906102de565b90607f1690610089565b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c90816306fdde031461070357508063095ea7b31461067457806318160ddd1461065757806323b872dd146104e1578063313ce567146104c657806340c10f19146103e557806370a08231146103a157806395d89b41146102265780639dc29fac14610138578063a9059cbb146101075763dd62ed3e14610095575f80fd5b34610103576040600319360112610103576100ae610804565b73ffffffffffffffffffffffffffffffffffffffff6100cb610827565b91165f52600160205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060405f2054604051908152f35b5f80fd5b346101035760406003193601126101035761012d610123610804565b602435903361084a565b602060405160018152f35b3461010357604060031936011261010357610151610804565b73ffffffffffffffffffffffffffffffffffffffff602435911680156101fa57805f525f60205260405f20548281106101c8576020835f947fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef938587528684520360408620558060025403600255604051908152a3005b907fe450d38c000000000000000000000000000000000000000000000000000000005f5260045260245260445260645ffd5b7f96c6fd1e000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b34610103575f600319360112610103576040515f600454908160011c60018316928315610397575b60208210841461036a57818552849390811561032857506001146102cc575b5003601f01601f191681019067ffffffffffffffff82118183101761029f5761029b829182604052826107da565b0390f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60045f90815291507f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b5b81831061030c5750508101602001601f1961026d565b60209193508060019154838588010152019101909183926102f6565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208581019190915291151560051b84019091019150601f19905061026d565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b90607f169061024e565b346101035760206003193601126101035773ffffffffffffffffffffffffffffffffffffffff6103cf610804565b165f525f602052602060405f2054604051908152f35b34610103576040600319360112610103576103fe610804565b73ffffffffffffffffffffffffffffffffffffffff16602435811561049a576002549080820180921161046d5760207fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef915f9360025584845283825260408420818154019055604051908152a3005b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7fec442f05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b34610103575f60031936011261010357602060405160128152f35b34610103576060600319360112610103576104fa610804565b610502610827565b6044359073ffffffffffffffffffffffffffffffffffffffff831692835f52600160205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260405f20547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff811061057e575b5061012d935061084a565b8381106106235784156105f75733156105cb5761012d945f52600160205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f526020528360405f209103905584610573565b7f94280d62000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7fe602df05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b83907ffb8f41b2000000000000000000000000000000000000000000000000000000005f523360045260245260445260645ffd5b34610103575f600319360112610103576020600254604051908152f35b346101035760406003193601126101035761068d610804565b6024359033156105f75773ffffffffffffffffffffffffffffffffffffffff169081156105cb57335f52600160205260405f20825f526020528060405f20556040519081527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560203392a3602060405160018152f35b34610103575f600319360112610103575f600354908160011c600183169283156107d0575b60208210841461036a5781855284939081156103285750600114610774575003601f01601f191681019067ffffffffffffffff82118183101761029f5761029b829182604052826107da565b60035f90815291507fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b5b8183106107b45750508101602001601f1961026d565b602091935080600191548385880101520191019091839261079e565b90607f1690610728565b601f19601f602060409481855280519182918282880152018686015e5f8582860101520116010190565b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361010357565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361010357565b73ffffffffffffffffffffffffffffffffffffffff169081156101fa5773ffffffffffffffffffffffffffffffffffffffff1691821561049a57815f525f60205260405f20548181106108e257817fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef92602092855f525f84520360405f2055845f525f825260405f20818154019055604051908152a3565b827fe450d38c000000000000000000000000000000000000000000000000000000005f5260045260245260445260645ffd60808060405234601557610a98908161001a8239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c908163371bed681461086b5750806353066069146106ad5780636813d7871461065e578063838b25201461014c578063959b825a146100845763d3072d821461005e575f80fd5b34610080575f60031936011261008057602060ff600154166040519015158152f35b5f80fd5b34610080576020600319360112610080576004355f54811015610080576100aa906108c7565b5073ffffffffffffffffffffffffffffffffffffffff81541661014873ffffffffffffffffffffffffffffffffffffffff6001840154169273ffffffffffffffffffffffffffffffffffffffff60028201541690600381015461011a600563ffffffff60048501541693016109da565b926040519687968752602087015260408601526060850152608084015260c060a084015260c0830190610884565b0390f35b346100805760c06003193601126100805760043573ffffffffffffffffffffffffffffffffffffffff8116809103610080576024359073ffffffffffffffffffffffffffffffffffffffff82168092036100805760443573ffffffffffffffffffffffffffffffffffffffff8116809103610080576064356084359063ffffffff82168092036100805760a4359467ffffffffffffffff8611610080573660238701121561008057856004013567ffffffffffffffff81116100805736602482890101116100805760ff60015416610600576040517f23b872dd0000000000000000000000000000000000000000000000000000000081523360048201523060248201528360448201526020816064815f8b5af180156105f5576105ba575b5060205f916040519761027d8961097d565b88528188019384526040880196875260608801948552608088019586528060246040519a6102d2857fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f860116018d610999565b828c5201838b01378801015260a085019586525f54680100000000000000008110156105615780600161030792015f556108c7565b94909461058e5773ffffffffffffffffffffffffffffffffffffffff809281806005995116167fffffffffffffffffffffffff00000000000000000000000000000000000000008854161787555116826001870191167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055511673ffffffffffffffffffffffffffffffffffffffff6002850191167fffffffffffffffffffffffff000000000000000000000000000000000000000082541617905551600383015563ffffffff600483019151167fffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000082541617905501905190815167ffffffffffffffff811161056157610420825461092c565b601f811161051c575b50602092601f821160011461048357928192935f92610478575b50507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8260011b9260031b1c19161790555f80f35b015190508380610443565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0821693835f52805f20915f5b86811061050457508360019596106104cd575b505050811b019055005b01517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60f88460031b161c191690558380806104c3565b919260206001819286850151815501940192016104b0565b825f5260205f20601f830160051c81019160208410610557575b601f0160051c01905b81811061054c5750610429565b5f815560010161053f565b9091508190610536565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6020813d6020116105ed575b816105d360209383610999565b81010312610080575180151581036100805750602061026b565b3d91506105c6565b6040513d5f823e3d90fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601660248201527f4f7074696d69736d20627269646765206661696c6564000000000000000000006044820152fd5b34610080576020600319360112610080576004358015158091036100805760ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00600154169116176001555f80f35b34610080575f60031936011261008057606060a06040516106cd8161097d565b5f81525f60208201525f60408201525f838201525f608082015201525f54801561080d577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116107e05761073973ffffffffffffffffffffffffffffffffffffffff916108c7565b506101486040519161074a8361097d565b83815416835263ffffffff846001830154169160208501928352856002820154166040860190815286600383015491606088019283528161079b6005876004880154169660808c01978852016109da565b9660a08a019788526040519a8b9a60208c52511660208b01525116604089015251166060870152516080860152511660a08401525160c08084015260e0830190610884565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600b60248201527f4e6f206465706f736974730000000000000000000000000000000000000000006044820152fd5b34610080575f600319360112610080576020905f548152f35b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b5f548110156108ff575f8080526006919091027f290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e5630191565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b90600182811c92168015610973575b602083101461094657565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f169161093b565b60c0810190811067ffffffffffffffff82111761056157604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761056157604052565b9060405191825f8254926109ed8461092c565b8084529360018116908115610a585750600114610a14575b50610a1292500383610999565b565b90505f9291925260205f20905f915b818310610a3c575050906020610a12928201015f610a05565b6020919350806001915483858901015201910190918492610a23565b60209350610a129592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f610a055660806040523461032757604051601f611b5438819003918201601f19168301916001600160401b038311848410176102f357808492610100946040528339810103126103275761004e8161032b565b61005a6020830161032b565b6100666040840161032b565b6060840151608085015161007c60a0870161032b565b9360e061008b60c0890161032b565b9701519563ffffffff871687036103275760408051929083016001600160401b038111848210176102f357604052600f83526e4f7074696d69736d2042726964676560881b6020840152600180556001600160a01b038216158015610316575b61030757610105826100ff61010b9461033f565b506103b5565b50610448565b508051906001600160401b0382116102f35760075490600182811c921680156102e9575b60208310146102d55781601f849311610267575b50602090601f8311600114610201575f926101f6575b50508160011b915f199060031b1c1916176007555b600280546003939093556004919091556001600160a81b03199091166001600160a01b0392831617600160a01b179055620151804204600655600880546001600160a01b03191692821692909217909155600980546001600160c01b031916939091169290921760a09190911b63ffffffff60a01b161790556040516115f890816104dc8239f35b015190505f80610159565b60075f9081528281209350601f198516905b81811061024f5750908460019594939210610237575b505050811b0160075561016e565b01515f1960f88460031b161c191690555f8080610229565b92936020600181928786015181550195019301610213565b60075f529091507fa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c688601f840160051c810191602085106102cb575b90601f859493920160051c01905b8181106102bd5750610143565b5f81558493506001016102b0565b90915081906102a2565b634e487b7160e01b5f52602260045260245ffd5b91607f169161012f565b634e487b7160e01b5f52604160045260245ffd5b63d92e233d60e01b5f5260045ffd5b506001600160a01b038116156100eb565b5f80fd5b51906001600160a01b038216820361032757565b6001600160a01b0381165f9081525f516020611b345f395f51905f52602052604090205460ff166103b0576001600160a01b03165f8181525f516020611b345f395f51905f5260205260408120805460ff191660011790553391905f516020611ad45f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f516020611b145f395f51905f52602052604090205460ff166103b0576001600160a01b03165f8181525f516020611b145f395f51905f5260205260408120805460ff191660011790553391907f751b795d24b92e3d92d1d0d8f2885f4e9c9c269da350af36ae6b49069babf4bf905f516020611ad45f395f51905f529080a4600190565b6001600160a01b0381165f9081525f516020611af45f395f51905f52602052604090205460ff166103b0576001600160a01b03165f8181525f516020611af45f395f51905f5260205260408120805460ff191660011790553391907fc074beb46b251f73c0fec16eba75d6bc0042d09ee17a740ebbee308dbf87f82f905f516020611ad45f395f51905f529080a460019056fe6080806040526004361015610012575f80fd5b5f905f3560e01c90816301ffc9a714611001575080631033b4cc14610fe4578063118c38c714610faa5780631259a5c814610f8d57806318b68b8c14610a1e578063248a9ca3146109eb5780632f2ff15d146109ad57806332d4f5041461095b5780633462fac31461092057806336568abe146108b557806336b089d8146108975780633cbb69791461087057806356eff2671461083c5780635ab1d61c1461079e57806361b0a56e14610661578063632214901461063e57806365d7a3c91461060a57806366d003ac146105d657806367eeba0c146105b85780636bcc8c14146104ea57806385931b74146103a257806391d148541461034b578063a217fddf1461032f578063b20d30a9146102d9578063c9f5b63e146102a5578063d547741f1461025e578063ead93c8f14610238578063ede7cebd146101d65763fb8c4b511461015d575f80fd5b346101d357806003193601126101d357600554600454818111156101ca5781810381811161019d5760609350905b60405192835260208301526040820152f35b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b6060929061018b565b80fd5b50346101d357806003193601126101d35761020c6002546101f5611153565b9060ff60405193849360608552606085019061126c565b9173ffffffffffffffffffffffffffffffffffffffff8116602085015260a01c16151560408301520390f35b50346101d357806003193601126101d357602060ff60025460a01c166040519015158152f35b50346101d35760406003193601126101d3576102a160043561027e6110c2565b9061029c610297825f525f602052600160405f20015490565b611371565b6114a9565b5080f35b50346101d357806003193601126101d357602073ffffffffffffffffffffffffffffffffffffffff60025416604051908152f35b50346101d35760206003193601126101d3577f207c4cbdf55ec315a13f0d5e047732ec5d947da056e706593aa509909941cedf60406004356103196112e9565b600454908060045582519182526020820152a180f35b50346101d357806003193601126101d357602090604051908152f35b50346101d35760406003193601126101d35773ffffffffffffffffffffffffffffffffffffffff604061037c6110c2565b926004358152806020522091165f52602052602060ff60405f2054166040519015158152f35b50346101d35760606003193601126101d3576103bc61109f565b6103c46110c2565b6044359163ffffffff831683036104e6576104e07f802b8c7b24709b6c9c56179dceeb977cc7ac6fa4f15f84c99a8627abfd97cc35936104026112e9565b73ffffffffffffffffffffffffffffffffffffffff83167fffffffffffffffffffffffff0000000000000000000000000000000000000000600854161760085560095477ffffffff00000000000000000000000000000000000000008260a01b16907fffffffffffffffff00000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff8716911617176009556040519384938491604091949373ffffffffffffffffffffffffffffffffffffffff63ffffffff9281606087019816865216602085015216910152565b0390a180f35b8380fd5b50346101d35760206003193601126101d35773ffffffffffffffffffffffffffffffffffffffff61051961109f565b6105216112e9565b1680156105905773ffffffffffffffffffffffffffffffffffffffff600254827fffffffffffffffffffffffff0000000000000000000000000000000000000000821617600255167fb07f8b1b85042d74022c867c836edeb0bcd70e135b0042390d2b1fd1082980698380a380f35b6004827fd92e233d000000000000000000000000000000000000000000000000000000008152fd5b50346101d357806003193601126101d3576020600454604051908152f35b50346101d357806003193601126101d357602073ffffffffffffffffffffffffffffffffffffffff60095416604051908152f35b50346101d357806003193601126101d35761063a610626611153565b60405191829160208352602083019061126c565b0390f35b50346101d35760206003193601126101d3576106586112e9565b60043560035580f35b50346101d35760606003193601126101d35761067b61109f565b60443573ffffffffffffffffffffffffffffffffffffffff81168082036104e657838052836020526040842073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f2054161561076e5715610746576040517fa9059cbb00000000000000000000000000000000000000000000000000000000602082015273ffffffffffffffffffffffffffffffffffffffff9182166024808301919091523560448083019190915281526107439290919061073d6064846110e5565b16611571565b80f35b6004837fd92e233d000000000000000000000000000000000000000000000000000000008152fd5b6044847fe2517d3f0000000000000000000000000000000000000000000000000000000081523360045280602452fd5b50346101d35760206003193601126101d3576004358015158091036108385760207fb3418989d06835b5c215eebb4d54ed6be7bbb66eb4807164740a2e082fa782d5916107e96112e9565b6002547fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff74ff00000000000000000000000000000000000000008360a01b16911617600255604051908152a180f35b5080fd5b50346101d357806003193601126101d357602073ffffffffffffffffffffffffffffffffffffffff60085416604051908152f35b50346101d357806003193601126101d357602063ffffffff60095460a01c16604051908152f35b50346101d357806003193601126101d3576020600354604051908152f35b50346101d35760406003193601126101d3576108cf6110c2565b3373ffffffffffffffffffffffffffffffffffffffff8216036108f8576102a1906004356114a9565b6004827f6697b232000000000000000000000000000000000000000000000000000000008152fd5b50346101d357806003193601126101d35760206040517fc074beb46b251f73c0fec16eba75d6bc0042d09ee17a740ebbee308dbf87f82f8152f35b50346101d357806003193601126101d3576008546009546040805173ffffffffffffffffffffffffffffffffffffffff9384168152928216602084015260a09190911c63ffffffff1690820152606090f35b50346101d35760406003193601126101d3576102a16004356109cd6110c2565b906109e6610297825f525f602052600160405f20015490565b6113d7565b50346101d35760206003193601126101d3576020610a166004355f525f602052600160405f20015490565b604051908152f35b5034610db5576060600319360112610db557610a3861109f565b6024359060443567ffffffffffffffff8111610db55736602382011215610db55780600401359167ffffffffffffffff8311610db55782820190366024830111610db557600260015414610f6557600260015560ff60025460a01c1615610f3d57335f9081527ffe0c7bd5092c7a6e463488fa239f32058c87d43e336e2e584a272132cf8dc926602052604090205460ff1615610f155773ffffffffffffffffffffffffffffffffffffffff16928315610eed578415610ec5576003548511610e5e576201518042046006548111610e86575b50610b18856005546112af565b60045410610e5e57610b736040517f23b872dd00000000000000000000000000000000000000000000000000000000602082015233602482015230604482015286606482015260648152610b6d6084826110e5565b85611571565b15610e335760409082900312610db55760248101359073ffffffffffffffffffffffffffffffffffffffff8216809203610db557604401359063ffffffff8216809203610db557905b73ffffffffffffffffffffffffffffffffffffffff6002541660405160205f8183017f095ea7b3000000000000000000000000000000000000000000000000000000008152610c6284610c368b88602484016020909392919373ffffffffffffffffffffffffffffffffffffffff60408201951681520152565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081018652856110e5565b83519082895af15f513d82610e17575b505015610db9575b505073ffffffffffffffffffffffffffffffffffffffff600254169173ffffffffffffffffffffffffffffffffffffffff6008541692803b15610db55773ffffffffffffffffffffffffffffffffffffffff935f60e49263ffffffff829660405198899788967f838b25200000000000000000000000000000000000000000000000000000000088528c600489015260248801521660448601528a606486015216608484015260c060a48401528160c48401525af18015610daa57610d95575b50610d47826005546112af565b6005557f3dba401dc1abbf017086864b33ba815f53ab3cdbb9366ba6bcd8ec342ddd98e8602073ffffffffffffffffffffffffffffffffffffffff6002541693604051908152a36001805580f35b610da29193505f906110e5565b5f915f610d3a565b6040513d5f823e3d90fd5b5f80fd5b610e0a610e1092604051907f095ea7b300000000000000000000000000000000000000000000000000000000602083015260248201525f604482015260448152610e046064826110e5565b86611571565b84611571565b5f80610c7a565b909150610e2b5750843b15155b5f80610c72565b600114610e24565b505060095463ffffffff73ffffffffffffffffffffffffffffffffffffffff82169160a01c16610bbc565b7f70d168bc000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fda4e39dd56d72c2ee3d132e0146bc39e905e78e3bc64c40190421c7b2bcef2ab60406005548151908482526020820152a15f6005556006555f610b0b565b7f1f2a2005000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f5c427cd9000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f7bea20b2000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610db5575f600319360112610db5576020600654604051908152f35b34610db5575f600319360112610db55760206040517f751b795d24b92e3d92d1d0d8f2885f4e9c9c269da350af36ae6b49069babf4bf8152f35b34610db5575f600319360112610db5576020600554604051908152f35b34610db5576020600319360112610db557600435907fffffffff000000000000000000000000000000000000000000000000000000008216809203610db557817f7965db0b0000000000000000000000000000000000000000000000000000000060209314908115611075575b5015158152f35b7f01ffc9a7000000000000000000000000000000000000000000000000000000009150148361106e565b6004359073ffffffffffffffffffffffffffffffffffffffff82168203610db557565b6024359073ffffffffffffffffffffffffffffffffffffffff82168203610db557565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761112657604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051905f6007548060011c9160018216918215611262575b6020841083146112355783865285929081156111f85750600114611199575b611197925003836110e5565b565b5060075f90815290917fa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c6885b8183106111dc5750509060206111979282010161118b565b60209193508060019154838589010152019101909184926111c4565b602092506111979491507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001682840152151560051b82010161118b565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b92607f169261116c565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b919082018092116112bc57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b335f9081527fdfff91bcee88f6ea00b2726f6d062a509c32835793470c55bed3bcc74c0f5d37602052604090205460ff161561132157565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f751b795d24b92e3d92d1d0d8f2885f4e9c9c269da350af36ae6b49069babf4bf60245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f205416156113a85750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f146114a357805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f146114a357805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190565b905f602091828151910182855af115610daa575f513d6115ef575073ffffffffffffffffffffffffffffffffffffffff81163b155b6115ad5750565b73ffffffffffffffffffffffffffffffffffffffff907f5274afe7000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b600114156115a6562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0dfe0c7bd5092c7a6e463488fa239f32058c87d43e336e2e584a272132cf8dc926dfff91bcee88f6ea00b2726f6d062a509c32835793470c55bed3bcc74c0f5d37ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`\x98W`\x0C\x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U`\x1F\x80T\x90\x91\x16\x90\x91\x17\x90U`\"\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16a\x124\x17\x90\x91U`#\x80T\x82\x16aVx\x17\x90U`$\x80T\x82\x16a\x9A\xBC\x17\x90U`%\x80T\x90\x91\x16a\xDE\xF0\x17\x90U`&\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16v\x03\r@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\x17\x90Ua\x8E\xE1\x90\x81a\0\x9D\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\n\x92T\xE4\x14aK\xD0WP\x80c\r\x17\x0B\x02\x14aG\xC6W\x80c\r\x1C\xE0\xDD\x14aDrW\x80c\x11~;B\x14aDMW\x80c\x1E\xD7\x83\x1C\x14aC\xCFW\x80c$\x8E\xC3&\x14aC\xA9W\x80c*\xDE8\x80\x14aA\xB5W\x80c,\xD3\x8F\xBF\x14a@\xAAW\x80c7N\x0C\xE6\x14a=QW\x80c<\xBBiy\x14a=*W\x80c>^<#\x14a<\xACW\x80c?r\x86\xF4\x14a<.W\x80cO\x862\xBA\x14a<\x07W\x80cRt>\xC4\x14a7_W\x80cV\xEF\xF2g\x14a78W\x80cf\xD0\x03\xAC\x14a7\x11W\x80cf\xD9\xA9\xA0\x14a5\xD4W\x80cp(wx\x14a3\\W\x80c{ML\xE3\x14a36W\x80c\x85\"l\x81\x14a2\xACW\x80c\x91j\x17\xC6\x14a2\x02W\x80c\xA3\xD4H[\x14a1\xD8W\x80c\xB0FO\xDC\x14a1.W\x80c\xB4M\xC9\xD6\x14a,\xFEW\x80c\xB5P\x8A\xA9\x14a,tW\x80c\xBAAO\xA6\x14a,OW\x80c\xCC\xC0\xCF\xAC\x14a&\x88W\x80c\xCF\xFB\x04\x8B\x14a\"gW\x80c\xD8\xB2\x96\xDA\x14a\x1DEW\x80c\xDC\xCCW\xF1\x14a\x19\xF4W\x80c\xE1x\xBC[\x14a\x18*W\x80c\xE2\x0C\x9Fq\x14a\x17\x9CW\x80c\xF2\x06{\xD7\x14a\x14\nW\x80c\xF6\x97\xE7\x8A\x14a\x0CsW\x80c\xF8\x10\x06\xB2\x14a\x07\x82W\x80c\xF8Q\xA4@\x14a\x07[W\x80c\xFAv&\xD4\x14a\x078W\x80c\xFC\x0CTj\x14a\x07\x11W\x80c\xFC\x9C\x8D9\x14a\x06\xEAWc\xFEG\xA3\xF4\x14a\x01\xD6W_\x80\xFD[4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80`@Qa\"\"` \x82\x01Rb\x04\x93\xE0`@\x82\x01R`@\x81Ra\x02\n``\x82aPXV[`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x06\xD5W[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri?\x87\x08W\xA3\xE0\xE3\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x06)Wa\x06\x9EW[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x06\x89W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x82;\x15a\x06\x84Wa\x03\xC1\x92\x84\x92\x83`@Q\x80\x96\x81\x95\x82\x94\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri?\x87\x08W\xA3\xE0\xE3\x80\0\0`$\x84\x01R```D\x84\x01R`d\x83\x01\x90aN\x90V[\x03\x92Z\xF1\x80\x15a\x06\x06Wa\x06oW[P`\x04\x81`\x01`\x01`\xA0\x1B\x03` T\x16`@Q\x92\x83\x80\x92\x7FS\x06`i\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x06\x06W\x82\x91a\x06MW[Pa\x04;`\x01`\x01`\xA0\x1B\x03\x82Q\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90aY\xC1V[a\x04^`\x01`\x01`\xA0\x1B\x03` \x83\x01Q\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90aY\xC1V[`\x01`\x01`\xA0\x1B\x03`@\x82\x01Q\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`@Q\x90\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ra\"\"`$\x82\x01R\x82\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x06)W\x83\x91a\x068W[PP``\x81\x01Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ri?\x87\x08W\xA3\xE0\xE3\x80\0\0`$\x82\x01R\x82\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x06)W\x83\x91a\x06\x14W[PP`\x80\x01Qc\xFF\xFF\xFF\xFF\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rb\x04\x93\xE0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x06\x06Wa\x05\xF2WP\xF3[\x81a\x05\xFC\x91aPXV[a\x06\x03W\x80\xF3[\x80\xFD[`@Q=\x84\x82>=\x90\xFD[P\xFD[\x81a\x06\x1E\x91aPXV[a\x06\x11W\x81_a\x05mV[`@Q=\x85\x82>=\x90\xFD[PP\xFD[\x81a\x06B\x91aPXV[a\x06\x11W\x81_a\x04\xE5V[a\x06i\x91P=\x80\x84\x83>a\x06a\x81\x83aPXV[\x81\x01\x90aV\x01V[_a\x04\x1AV[\x81a\x06y\x91aPXV[a\x06\x03W\x80_a\x03\xD0V[PPP\xFD[\x81a\x06\x93\x91aPXV[a\x06\x11W\x81_a\x03BV[` \x81=` \x11a\x06\xCDW[\x81a\x06\xB7` \x93\x83aPXV[\x81\x01\x03\x12a\x064Wa\x06\xC8\x90aP\xD3V[a\x02\xDCV[=\x91Pa\x06\xAAV[\x81a\x06\xDF\x91aPXV[a\x06\x11W\x81_a\x02oV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80` `@Qa\x07\xA3\x82\x82aPXV[\x82\x81R`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x84W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0C\x0CW\x84\x91a\x0C^W[P\x82`\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x0C\x0CWa\x0C,W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R\x83`$\x82\x01R`\x01`D\x82\x01R`\x01`d\x82\x01R\x83\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0C\x0CW\x84\x91a\x0C\x17W[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03\x83T\x16\x90\x7F=\xBA@\x1D\xC1\xAB\xBF\x01p\x86\x86K3\xBA\x81_S\xAB<\xDB\xB96k\xA6\xBC\xD8\xEC4-\xDD\x98\xE8\x84`@Qii\xE1\r\xE7fv\xD0\x80\0\0\x81R\xA3`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x84W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0C\x0CW\x84\x91a\x0B\xF7W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x82;\x15a\x0B\xF3Wa\n0\x92\x85\x92\x83`@Q\x80\x96\x81\x95\x82\x94\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x84\x01R```D\x84\x01R`d\x83\x01\x90aN\x90V[\x03\x92Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x0B\xDEW[P`\x04\x90`\x01`\x01`\xA0\x1B\x03\x83T\x16`@Q\x92\x83\x80\x92\x7FS\x06`i\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06)W`\xA0\x91\x84\x91a\x0B\xC4W[Pa\n\xAE`\x01`\x01`\xA0\x1B\x03\x82Q\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90aY\xC1V[a\n\xD0`\x01`\x01`\xA0\x1B\x03\x84\x83\x01Q\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90aY\xC1V[a\x0B\x1B`\x01`\x01`\xA0\x1B\x03`@\x83\x01Q\x16a\n\xF8`&T\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x90aY\xC1V[a\x0B\x05``\x84\x01QaWUV[c\xFF\xFF\xFF\xFF\x80`\x80\x85\x01Q\x16\x91\x85\x1C\x16\x90aYKV[\x01Qa\x0B*`@Q\x92\x83aPXV[\x82\x82Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064Wa\x0B\x8C\x83\x91a\x0B\x9E`@Q\x94\x85\x93\x84\x93\x7F\x97bF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`@`\x04\x86\x01R`D\x85\x01\x90aN\x90V[\x90`\x03\x19\x84\x83\x03\x01`$\x85\x01RaN\x90V[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x06\x06Wa\x05\xF2WP\xF3[a\x0B\xD8\x91P=\x80\x86\x83>a\x06a\x81\x83aPXV[_a\n\x8DV[\x81a\x0B\xE8\x91aPXV[a\x06\x11W\x81_a\nBV[\x84\x80\xFD[\x81a\x0C\x01\x91aPXV[a\x064W\x82_a\t\xB1V[`@Q=\x86\x82>=\x90\xFD[\x81a\x0C!\x91aPXV[a\x064W\x82_a\x08\xFFV[\x82\x81\x81=\x83\x11a\x0CWW[a\x0CA\x81\x83aPXV[\x81\x01\x03\x12a\x06\x84Wa\x0CR\x90aP\xD3V[a\x08wV[P=a\x0C7V[\x81a\x0Ch\x91aPXV[a\x064W\x82_a\x08\x0BV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`@QaUU` \x82\x01Rb\x02\xBF `@\x82\x01R`@\x81Ra\x0C\xA7``\x82aPXV[`\x01`\x01`\xA0\x1B\x03`!T\x16\x82`\x01`\x01`\xA0\x1B\x03`#T\x16\x92`@Q\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R\x80`\x04\x86\x01R` \x85`$\x81\x87Z\xFA\x94\x85\x15a\x06)W\x83\x95a\x13\xD3W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`$`@Q\x80\x97\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x93\x84\x15a\x06)W\x83\x94a\x13\x9CW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13/W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x13\x87W[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x9E\xD1\x94\xDB\x19\xB28\xC0\0\0`$\x84\x01RZ\xF1\x80\x15a\x06)Wa\x13PW[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13/W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x137W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x82;\x15a\x133Wa\x0F\0\x92\x84\x92\x83`@Q\x80\x96\x81\x95\x82\x94\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x9E\xD1\x94\xDB\x19\xB28\xC0\0\0`$\x84\x01R```D\x84\x01R`d\x83\x01\x90aN\x90V[\x03\x92Z\xF1\x80\x15a\x06\x06Wa\x13\x1AW[PP`\x01`\x01`\xA0\x1B\x03`!T\x16\x91`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81\x87Z\xFA\x90\x81\x15a\x12\xDDW\x85\x91a\x12\xE8W[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa.k$\xE6M\xC7@\0\0\x82\x01\x91\x82\x11a\x12~W\x90a\x0F\xA2\x91aYKV[`\x01`\x01`\xA0\x1B\x03` T\x16\x90`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82`\x04\x82\x01R` \x81`$\x81\x87Z\xFA\x90\x81\x15a\x12\xDDW\x85\x91a\x12\xABW[Pi\x9E\xD1\x94\xDB\x19\xB28\xC0\0\0\x82\x01\x80\x92\x11a\x12~W`\x04\x92a\x10\x17\x86\x95\x93\x86\x93aYKV[`@Q\x93\x84\x80\x92\x7FS\x06`i\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x06)W\x83\x92a\x12^W[Pa\x10f\x90`\x01`\x01`\xA0\x1B\x03\x83Q\x16aY\xC1V[a\x10\x89`\x01`\x01`\xA0\x1B\x03` \x83\x01Q\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90aY\xC1V[`\x01`\x01`\xA0\x1B\x03`@\x82\x01Q\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`@Q\x90\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01RaUU`$\x82\x01R\x82\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x06)W\x83\x91a\x12IW[PP`\x80\x81a\x11'``c\xFF\xFF\xFF\xFF\x94\x01QaX\xCBV[\x01Q\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rb\x02\xBF `$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x06\x06Wa\x124W[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90a\x11\xFCW[a\x11\xF9\x91PaX\xCBV[\x80\xF3[P` \x81=` \x11a\x12,W[\x81a\x12\x16` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa\x11\xF9\x90Qa\x11\xEFV[_\x80\xFD[=\x91Pa\x12\tV[\x81a\x12>\x91aPXV[a\x06\x03W\x80_a\x11\xA1V[\x81a\x12S\x91aPXV[a\x06\x11W\x81_a\x11\x10V[a\x10f\x91\x92Pa\x12w\x90=\x80\x86\x83>a\x06a\x81\x83aPXV[\x91\x90a\x10QV[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11a\x12\xD5W[\x81a\x12\xC6` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WQ_a\x0F\xF2V[=\x91Pa\x12\xB9V[`@Q=\x87\x82>=\x90\xFD[\x90P` \x81=` \x11a\x13\x12W[\x81a\x13\x03` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WQ_a\x0FmV[=\x91Pa\x12\xF6V[\x81a\x13$\x91aPXV[a\x13/W\x82_a\x0F\x0FV[\x82\x80\xFD[\x83\x80\xFD[\x81a\x13A\x91aPXV[a\x13LW\x81_a\x0E\x81V[P\x80\xFD[` \x81=` \x11a\x13\x7FW[\x81a\x13i` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wa\x13z\x90aP\xD3V[a\x0E\x1BV[=\x91Pa\x13\\V[\x81a\x13\x91\x91aPXV[a\x13LW\x81_a\r\xAEV[\x92P\x92P` \x82=` \x11a\x13\xCBW[\x81a\x13\xB9` \x93\x83aPXV[\x81\x01\x03\x12a\x12(W\x84\x91Q\x92_a\rTV[=\x91Pa\x13\xACV[\x92P\x93P` \x82=` \x11a\x14\x02W[\x81a\x13\xF0` \x93\x83aPXV[\x81\x01\x03\x12a\x12(W\x84\x91Q\x93_a\r\x05V[=\x91Pa\x13\xE3V[P4a\x06\x03W` `\x03\x196\x01\x12a\x06\x03W`\x045\x81`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x83\x03a\x13LWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13LW`@Q\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x15\x15`\x04\x82\x01R\x82\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x06)W\x83\x91a\x17\x87W[PPc\xFF\xFF\xFF\xFF`&T`\xA0\x1C\x16`@Q\x91` \x83\x01R`@\x82\x01R`@\x81Ra\x14\xCF``\x82aPXV[`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13/W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x17rW[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x06)Wa\x17;W[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13/W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x17&W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x82;\x15a\x133Wa\x16\x86\x92\x84\x92\x83`@Q\x80\x96\x81\x95\x82\x94\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01R```D\x84\x01R`d\x83\x01\x90aN\x90V[\x03\x92Z\xF1\x80\x15a\x06\x06Wa\x17\x11W[P\x90`\x04\x91`\x01`\x01`\xA0\x1B\x03` T\x16`@Q\x93\x84\x80\x92\x7FS\x06`i\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06)W`@`\x01`\x01`\xA0\x1B\x03\x91a\x11\xF9\x94\x86\x91a\x16\xF7W[P\x01Q\x16aY\xC1V[a\x17\x0B\x91P=\x80\x88\x83>a\x06a\x81\x83aPXV[_a\x16\xEEV[\x81a\x17\x1B\x91aPXV[a\x13LW\x81_a\x16\x95V[\x81a\x170\x91aPXV[a\x13LW\x81_a\x16\x07V[` \x81=` \x11a\x17jW[\x81a\x17T` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wa\x17e\x90aP\xD3V[a\x15\xA1V[=\x91Pa\x17GV[\x81a\x17|\x91aPXV[a\x13LW\x81_a\x154V[\x81a\x17\x91\x91aPXV[a\x13LW\x81_a\x14\xA4V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x18\x0BWa\x18\x07\x85a\x17\xFB\x81\x87\x03\x82aPXV[`@Q\x91\x82\x91\x82aNNV[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x17\xE4V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x04\x81`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xED\xE7\xCE\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x80\x81\x93\x82\x93a\x19\x85W[P`@\x80Q\x91a\x18\x9A\x82\x84aPXV[`\x0F\x83R\x7FOptimism Bridge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x133Wa\x0B\x8C\x84\x91a\x19!\x84Q\x95\x86\x93\x84\x93\x7F\xF3 \xD9c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R\x87`\x04\x86\x01R`D\x85\x01\x90aN\x90V[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x19|WPa\x19gW[PPa\x19ba\x11\xF9\x92`\x01`\x01`\xA0\x1B\x03` T\x16\x90aY\xC1V[aZBV[\x81a\x19q\x91aPXV[a\x13/W\x82_a\x19GV[Q=\x84\x82>=\x90\xFD[\x93PPPP=\x80\x83\x83>a\x19\x99\x81\x83aPXV[\x81\x01``\x82\x82\x03\x12a\x13/W\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x133W\x82\x01\x90\x80`\x1F\x83\x01\x12\x15a\x133W\x81Qa\x19\xD2\x92` \x01aU\x9DV[\x90\x82a\x19\xEC`@a\x19\xE5` \x85\x01aQ\xFBV[\x93\x01aP\xD3V[\x91\x92_a\x18\x8AV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x06)W\x83\x91a\x1D\x13W[P`\"T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01\x81\x90R\x90` \x81`D\x81\x86Z\xFA\x80\x15a\x0C\x0CW\x84\x90a\x1C\xD8W[a\x1A\xB4\x91PaZBV[`@Q\x7F\x11\x8C8\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0C\x0CW\x84\x91a\x1C\xA4W[P`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R` \x81\x80`D\x81\x01[\x03\x81\x85Z\xFA\x80\x15a\x06)W\x83\x90a\x1CiW[a\x1BU\x91PaZBV[`@Q\x7F4b\xFA\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x06)W\x83\x91a\x1C5W[P`#T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90` \x90\x82\x90\x81\x80`D\x81\x01[\x03\x91Z\xFA\x80\x15a\x06\x06W\x82\x90a\x1B\xFAW[a\x11\xF9\x91PaZBV[P` \x81=` \x11a\x1C-W[\x81a\x1C\x14` \x93\x83aPXV[\x81\x01\x03\x12a\x13LWa\x1C(a\x11\xF9\x91aP\xD3V[a\x1B\xF0V[=\x91Pa\x1C\x07V[\x90P` \x81=` \x11a\x1CaW[\x81a\x1CP` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WQa\x1B\xDFa\x1B\x92V[=\x91Pa\x1CCV[P` \x81=` \x11a\x1C\x9CW[\x81a\x1C\x83` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wa\x1C\x97a\x1BU\x91aP\xD3V[a\x1BKV[=\x91Pa\x1CvV[\x90P` \x81=` \x11a\x1C\xD0W[\x81a\x1C\xBF` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WQa\x1B9a\x1A\xF1V[=\x91Pa\x1C\xB2V[P` \x81=` \x11a\x1D\x0BW[\x81a\x1C\xF2` \x93\x83aPXV[\x81\x01\x03\x12a\x133Wa\x1D\x06a\x1A\xB4\x91aP\xD3V[a\x1A\xAAV[=\x91Pa\x1C\xE5V[\x90P` \x81=` \x11a\x1D=W[\x81a\x1D.` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WQ_a\x1AQV[=\x91Pa\x1D!V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x03W\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81`\x04\x82\x01R\x81`$\x82\x01R\x81`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa\"RW[PP\x7F\x80+\x8C{$p\x9Bl\x9CV\x17\x9D\xCE\xEB\x97|\xC7\xACo\xA4\xF1_\x84\xC9\x9A\x86'\xAB\xFD\x97\xCC5`@Q\x80a\x1E!\x81\x90b\x03\xD0\x90`@``\x84\x01\x93a33\x81RaDD` \x82\x01R\x01RV[\x03\x90\xA1\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa\"=W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x06\x11W\x81`@Q\x80\x92\x7F\x85\x93\x1Bt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x83\x81a\x1E\xEE`\x04\x82\x01\x90b\x03\xD0\x90`@``\x84\x01\x93a33\x81RaDD` \x82\x01R\x01RV[\x03\x92Z\xF1\x80\x15a\x06\x06Wa\"(W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7FV\xEF\xF2g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x06)W\x83\x91a!\xEEW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`\x01`\x01`\xA0\x1B\x03`@Q\x91\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01Ra33`$\x82\x01R\x82\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x06)W\x83\x91a!\xD9W[PP`@Q\x7Ff\xD0\x03\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x06)W\x83\x91a!\x9FW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`\x01`\x01`\xA0\x1B\x03`@Q\x91\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01RaDD`$\x82\x01R\x82\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x06)W\x83\x91a!\x8AW[PP` `\x04\x91`@Q\x92\x83\x80\x92\x7F<\xBBiy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x06\x06W\x82\x91a!PW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11Wc\xFF\xFF\xFF\xFF`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01Rb\x03\xD0\x90`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x06\x06Wa\x05\xF2WP\xF3[\x90P` \x81=` \x11a!\x82W[\x81a!k` \x93\x83aPXV[\x81\x01\x03\x12a\x06\x11Wa!|\x90aR\x0FV[_a \xD0V[=\x91Pa!^V[\x81a!\x94\x91aPXV[a\x06\x11W\x81_a \x8FV[\x90P` \x81=` \x11a!\xD1W[\x81a!\xBA` \x93\x83aPXV[\x81\x01\x03\x12a\x064Wa!\xCB\x90aQ\xFBV[_a \x0CV[=\x91Pa!\xADV[\x81a!\xE3\x91aPXV[a\x06\x11W\x81_a\x1F\xCDV[\x90P` \x81=` \x11a\" W[\x81a\"\t` \x93\x83aPXV[\x81\x01\x03\x12a\x064Wa\"\x1A\x90aQ\xFBV[_a\x1FJV[=\x91Pa!\xFCV[\x81a\"2\x91aPXV[a\x06\x03W\x80_a\x1E\xFDV[\x81a\"G\x91aPXV[a\x06\x03W\x80_a\x1E\x87V[\x81a\"\\\x91aPXV[a\x06\x03W\x80_a\x1D\xD9V[P4a\x06\x03W` `\x03\x196\x01\x12a\x06\x03Wa\"\x91i\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`\x01`\x045aZ\xEBV[`@\x90\x82\x80\x83Qa\"\xA2\x85\x82aPXV[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x84Qa#?\x81a#\r` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x89`$\x84\x01R`d\x83\x01\x90aN\x90V[\x87`D\x83\x01R\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82aPXV[Q\x90jconsole.logZ\xFAP\x82`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13LW\x83Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a&\x1DWa&sW[PP`!T`\x1FT\x83Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03`\x08\x92\x90\x92\x1C\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91` \x91\x83\x91\x16\x81\x87\x81`D\x81\x01\x03\x92Z\xF1\x80\x15a%\xFBWa&<W[P\x82`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13LW\x83Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a&\x1DWa&'W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x13/W\x82\x91`\x84\x83\x92\x87Q\x94\x85\x93\x84\x92\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R\x88`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a&\x1DWa&\x08W[P`\x04\x90`\x01`\x01`\xA0\x1B\x03` T\x16\x84Q\x92\x83\x80\x92\x7FS\x06`i\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a%\xFBW\x82```\x04\x95\x93a%O\x93\x88\x91a%\xE1W[P\x01QaYKV[` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x82Q\x94\x85\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a%\xD8WP\x83\x90a%\xA4W[a\x11\xF9\x92PaYKV[P` \x82=` \x11a%\xD0W[\x81a%\xBE` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa\x11\xF9\x91Qa%\x9AV[=\x91Pa%\xB1V[Q=\x85\x82>=\x90\xFD[a%\xF5\x91P=\x80\x8A\x83>a\x06a\x81\x83aPXV[_a%GV[PPPQ\x90=\x90\x82>=\x90\xFD[\x81a&\x12\x91aPXV[a\x13/W\x82_a$\xF3V[\x84Q=\x84\x82>=\x90\xFD[\x81a&1\x91aPXV[a\x13/W\x82_a${V[` \x81=` \x11a&kW[\x81a&U` \x93\x83aPXV[\x81\x01\x03\x12a\x133Wa&f\x90aP\xD3V[a$\x18V[=\x91Pa&HV[\x81a&}\x91aPXV[a\x13/W\x82_a#\xB2V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa,:W[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rj\x01\xA7\x847\x9D\x99\xDBB\0\0\0`$\x84\x01RZ\xF1\x80\x15a\x06\x06Wa,\x03W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa+\xEEW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x80;\x15a\x064W`@Q\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x06\x06Wa+\xD9W[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90a+\xA5W[a(\xB6\x91PaXKV[b\x01Q\x80B\x01\x80B\x11a+xW\x81\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa+cW[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa+NW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x80;\x15a\x064W`@Q\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x06\x06Wa+9W[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90a+\x05W[a*\x7F\x91PaXKV[`\x04` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x92\x83\x80\x92\x7F7\x1B\xEDh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90a*\xD1W[a\x11\xF9\x91PaV\xD4V[P` \x81=` \x11a*\xFDW[\x81a*\xEB` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa\x11\xF9\x90Qa*\xC7V[=\x91Pa*\xDEV[P` \x81=` \x11a+1W[\x81a+\x1F` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa*\x7F\x90Qa*uV[=\x91Pa+\x12V[\x81a+C\x91aPXV[a\x06\x03W\x80_a*'V[\x81a+X\x91aPXV[a\x06\x03W\x80_a)\x97V[\x81a+m\x91aPXV[a\x06\x03W\x80_a)4V[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[P` \x81=` \x11a+\xD1W[\x81a+\xBF` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa(\xB6\x90Qa(\xACV[=\x91Pa+\xB2V[\x81a+\xE3\x91aPXV[a\x06\x03W\x80_a(^V[\x81a+\xF8\x91aPXV[a\x06\x03W\x80_a'\xCEV[` \x81=` \x11a,2W[\x81a,\x1C` \x93\x83aPXV[\x81\x01\x03\x12a\x13LWa,-\x90aP\xD3V[a'jV[=\x91Pa,\x0FV[\x81a,D\x91aPXV[a\x06\x03W\x80_a&\xFCV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` a,jaT\xC4V[`@Q\x90\x15\x15\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x19Ta,\x91\x81aP\xE0V[\x91a,\x9F`@Q\x93\x84aPXV[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a,\xE1W`@Q\x80a\x18\x07\x87\x82aO(V[`\x01` \x81\x92a,\xF0\x85aP\xF8V[\x81R\x01\x92\x01\x92\x01\x91\x90a,\xCCV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xC9\xF5\xB6>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x06)W\x83\x90a0\xF3W[a-q\x91P`\x01`\x01`\xA0\x1B\x03` T\x16\x90aY\xC1V[`@Q\x7FV\xEF\xF2g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x06)W\x83\x90a0\xB8W[a-\xC4\x91P`\x01`\x01`\xA0\x1B\x03`%T\x16\x90aY\xC1V[`@Q\x7Ff\xD0\x03\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x06)W\x83\x91a0~W[Pa.\x19`&T\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x90aY\xC1V[`@Q\x90\x7F<\xBBiy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x86Z\xFA\x91\x82\x15a\x0C\x0CW\x84\x92a09W[P\x90c\xFF\xFF\xFF\xFF\x80a.n\x93`\xA0\x1C\x16\x91\x16aYKV[`@Q\x7F6\xB0\x89\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x06)W\x83\x90a0\x05W[a.\xB4\x91PaXKV[\x81`@Q\x7Fg\xEE\xBA\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x06\x06W\x82\x91a/\xD0W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13LW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x06\x06Wa/\xBBW[PP` `\x04\x91`@Q\x92\x83\x80\x92\x7F\xEA\xD9<\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90a\x1B\xFAWa\x11\xF9\x91PaZBV[\x81a/\xC5\x91aPXV[a\x13LW\x81_a/rV[\x91PP` \x81=` \x11a/\xFDW[\x81a/\xEC` \x93\x83aPXV[\x81\x01\x03\x12a\x12(W\x82\x90Q_a.\xF2V[=\x91Pa/\xDFV[P` \x81=` \x11a01W[\x81a0\x1F` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa.\xB4\x90Qa.\xAAV[=\x91Pa0\x12V[\x91P` \x82=` \x11a0vW[\x81a0T` \x93\x83aPXV[\x81\x01\x03\x12a\x133Wc\xFF\xFF\xFF\xFF\x80a0na.n\x94aR\x0FV[\x93PPa.WV[=\x91Pa0GV[\x90P` \x81=` \x11a0\xB0W[\x81a0\x99` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wa0\xAA\x90aQ\xFBV[_a.\x01V[=\x91Pa0\x8CV[P` \x81=` \x11a0\xEBW[\x81a0\xD2` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wa0\xE6a-\xC4\x91aQ\xFBV[a-\xADV[=\x91Pa0\xC5V[P` \x81=` \x11a1&W[\x81a1\r` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wa1!a-q\x91aQ\xFBV[a-ZV[=\x91Pa1\0V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x1CTa1K\x81aP\xE0V[\x91a1Y`@Q\x93\x84aPXV[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a1\x9BW`@Q\x80a\x18\x07\x87\x82aO\xA5V[`\x02` `\x01\x92`@Qa1\xAE\x81aP<V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra1\xC6\x85\x87\x01aR V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a1\x86V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x1DTa2\x1F\x81aP\xE0V[\x91a2-`@Q\x93\x84aPXV[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a2oW`@Q\x80a\x18\x07\x87\x82aO\xA5V[`\x02` `\x01\x92`@Qa2\x82\x81aP<V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra2\x9A\x85\x87\x01aR V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a2ZV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x1ATa2\xC9\x81aP\xE0V[\x91a2\xD7`@Q\x93\x84aPXV[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a3\x19W`@Q\x80a\x18\x07\x87\x82aO(V[`\x01` \x81\x92a3(\x85aP\xF8V[\x81R\x01\x92\x01\x92\x01\x91\x90a3\x04V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W` `\x03\x196\x01\x12a\x06\x03Wa3\x80b\x98\x96\x80aR\x08`\x045aZ\xEBV[`@\x90\x82\x80\x83Qa3\x91\x85\x82aPXV[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x84Qa3\xFC\x81a#\r` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x89`$\x84\x01R`d\x83\x01\x90aN\x90V[Q\x90jconsole.logZ\xFAP\x82`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13LW\x83Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a&\x1DWa5\xBFW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x92c\xFF\xFF\xFF\xFF`\x01`\x01`\xA0\x1B\x03`&T\x16\x91\x16\x93\x82;\x15a\x133W\x85Q\x7F\x85\x93\x1Bt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`D\x82\x01R\x90\x82\x90\x82\x90`d\x90\x82\x90\x84\x90Z\xF1\x80\x15a&\x1DWa5\xAAW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x84Q\x92\x83\x80\x92\x7F<\xBBiy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x92\x83\x15a%\xD8WP\x83\x92a5gW[Pc\xFF\xFF\xFF\xFFa\x11\xF9\x92\x16aYKV[\x91P` \x82=` \x11a5\xA2W[\x81a5\x82` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wc\xFF\xFF\xFF\xFFa5\x9Ba\x11\xF9\x93aR\x0FV[\x92Pa5WV[=\x91Pa5uV[\x81a5\xB4\x91aPXV[a\x13/W\x82_a5\x08V[\x81a5\xC9\x91aPXV[a\x13/W\x82_a4oV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x1BTa5\xF1\x81aP\xE0V[a5\xFE`@Q\x91\x82aPXV[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a6\xD6W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a6kWPPPP\x03\x90\xF3[\x91\x93` a6\xC6\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a6\xB6\x83Q`@\x84R`@\x84\x01\x90aN\x90V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01RaN\xD3V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a6\\V[`\x02` `\x01\x92`@Qa6\xE9\x81aP<V[a6\xF2\x86aP\xF8V[\x81Ra6\xFF\x85\x87\x01aR V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a6.V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`&T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa;\xF2W[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x06\x06Wa;\xBBW[P`!T`\x1FT` \x80T`@Q\x7F\xDDb\xED>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03`\x08\x94\x90\x94\x1C\x84\x16`\x04\x82\x01R\x90\x83\x16`$\x82\x01R\x92\x90\x91\x83\x91`D\x91\x83\x91\x16Z\xFA\x80\x15a\x06\x06W\x82\x90a;\x87W[a8\xB2\x91PaW\xD5V[\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa;rW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x80;\x15a\x064W`@Q\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x06\x06Wa;]W[PP`\x01`\x01`\xA0\x1B\x03`!T\x16\x81`\x01`\x01`\xA0\x1B\x03` T\x16`@Q\x90\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x90\x81\x15a\x06\x06W\x82\x91a;(W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13LW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x06\x06Wa;\x13W[PP` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`$`@Q\x80\x94\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x80\x15a\x06\x06W\x82\x90a:\xDFW[a\x11\xF9\x91PaW\xD5V[P` \x81=` \x11a;\x0BW[\x81a:\xF9` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa\x11\xF9\x90Qa:\xD5V[=\x91Pa:\xECV[\x81a;\x1D\x91aPXV[a\x13LW\x81_a:\x82V[\x91PP` \x81=` \x11a;UW[\x81a;D` \x93\x83aPXV[\x81\x01\x03\x12a\x12(W\x82\x90Q_a:\x03V[=\x91Pa;7V[\x81a;g\x91aPXV[a\x06\x03W\x80_a9\xA5V[\x81a;|\x91aPXV[a\x06\x03W\x80_a9\x15V[P` \x81=` \x11a;\xB3W[\x81a;\xA1` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa8\xB2\x90Qa8\xA8V[=\x91Pa;\x94V[` \x81=` \x11a;\xEAW[\x81a;\xD4` \x93\x83aPXV[\x81\x01\x03\x12a\x13LWa;\xE5\x90aP\xD3V[a8@V[=\x91Pa;\xC7V[\x81a;\xFC\x91aPXV[a\x06\x03W\x80_a7\xD3V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a<\x8DWa\x18\x07\x85a\x17\xFB\x81\x87\x03\x82aPXV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a<vV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a=\x0BWa\x18\x07\x85a\x17\xFB\x81\x87\x03\x82aPXV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a<\xF4V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` c\xFF\xFF\xFF\xFF`&T`\xA0\x1C\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80`\x01`\x01`\xA0\x1B\x03` T\x16\x80;\x15a\x06\x11W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7Fh\x13\xD7\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x06\x06Wa@\x95W[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa@\x80W[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x06\x06Wa@IW[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa@4W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x03W\x80`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FOptimism bridge failed\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa@\x1FW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x80;\x15a\x064W`@Q\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x06\x06Wa\x05\xF2WP\xF3[\x81a@)\x91aPXV[a\x06\x03W\x80_a?\x8DV[\x81a@>\x91aPXV[a\x06\x03W\x80_a>\xEDV[` \x81=` \x11a@xW[\x81a@b` \x93\x83aPXV[\x81\x01\x03\x12a\x13LWa@s\x90aP\xD3V[a>\x89V[=\x91Pa@UV[\x81a@\x8A\x91aPXV[a\x06\x03W\x80_a>\x1CV[\x81a@\x9F\x91aPXV[a\x06\x03W\x80_a=\xB9V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x04```\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F2\xD4\xF5\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x83\x92\x84\x92aAMW[Pc\xFF\xFF\xFF\xFFa\x11\xF9\x93aA+\x82\x93`\x01`\x01`\xA0\x1B\x03`%T\x16\x90aY\xC1V[aAB`&T\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x90aY\xC1V[`\xA0\x1C\x16\x91\x16aYKV[\x92PPP``\x81=``\x11aA\xADW[\x81aAj``\x93\x83aPXV[\x81\x01\x03\x12a\x13LW\x80c\xFF\xFF\xFF\xFF\x80aA\x85a\x11\xF9\x94aQ\xFBV[aA+aA\xA0`@aA\x99` \x88\x01aQ\xFBV[\x96\x01aR\x0FV[\x94\x95P\x90\x92PaA\n\x90PV[=\x91PaA]V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x1ETaA\xD2\x81aP\xE0V[aA\xDF`@Q\x91\x82aPXV[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10aC W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10aBKW\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10aB\xD7WPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93aB>V[\x90\x91\x92\x93\x94` \x80aC\x13\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89QaN\x90V[\x97\x01\x95\x01\x93\x92\x91\x01aB\xB3V[`@QaC,\x81aP<V[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80TaCH\x81aP\xE0V[\x91aCV`@Q\x93\x84aPXV[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10aC\x8CWPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90aB\x0FV[`\x01` \x81\x92aC\x9B\x86aP\xF8V[\x81R\x01\x93\x01\x91\x01\x90\x91aCfV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `@Qj\x04\"\xCA\x8B\n\0\xA4%\0\0\0\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10aD.Wa\x18\x07\x85a\x17\xFB\x81\x87\x03\x82aPXV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aD\x17V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `@Qi\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80`\x01`\x01`\xA0\x1B\x03`$T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06WaG\xB1W[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x11\x8C8\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x06\x06W\x82\x91aG|W[PaE\xD7`\x01`\x01`\xA0\x1B\x03`$T\x16aE\xAB`@Q\x93\x84\x92\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x85\x01R`$\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82aPXV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W\x81aF2\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90aN\x90V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06WaGgW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16`&T\x82;\x15a\x06\x84W`@Q\x7F\x85\x93\x1Bt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x81\x16`$\x83\x01R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16`D\x82\x01R\x90\x82\x90\x82\x90\x81\x83\x81`d\x81\x01\x03\x92Z\xF1\x80\x15a\x06\x06WaGRW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x03W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa\x05\xF2WP\xF3[\x81aG\\\x91aPXV[a\x06\x03W\x80_aF\xE4V[\x81aGq\x91aPXV[a\x06\x03W\x80_aFWV[\x91PP` \x81=` \x11aG\xA9W[\x81aG\x98` \x93\x83aPXV[\x81\x01\x03\x12a\x12(W\x81\x90Q_aEMV[=\x91PaG\x8BV[\x81aG\xBB\x91aPXV[a\x06\x03W\x80_aD\xFFV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06WaK\xBBW[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x06\x06WaK\x84W[P\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x064W\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri*Z\x05\x8F\xC2\x95\xED\0\0\0`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x06\x06WaKoW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x064W\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri?\x87\x08W\xA3\xE0\xE3\x80\0\0`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x06\x06WaKZW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x03W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06WaKEW[PP`\x04` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x92\x83\x80\x92\x7F7\x1B\xEDh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90aK\x11W[aJ\x87\x91PaV\xD4V[`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90aJ\xDDW[a\x11\xF9\x91PaWUV[P` \x81=` \x11aK\tW[\x81aJ\xF7` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa\x11\xF9\x90QaJ\xD3V[=\x91PaJ\xEAV[P` \x81=` \x11aK=W[\x81aK+` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WaJ\x87\x90QaJ}V[=\x91PaK\x1EV[\x81aKO\x91aPXV[a\x06\x03W\x80_aJ3V[\x81aKd\x91aPXV[a\x06\x03W\x80_aI\xC7V[\x81aKy\x91aPXV[a\x06\x03W\x80_aIDV[` \x81=` \x11aK\xB3W[\x81aK\x9D` \x93\x83aPXV[\x81\x01\x03\x12a\x13LWaK\xAE\x90aP\xD3V[aH\xC0V[=\x91PaK\x90V[\x81aK\xC5\x91aPXV[a\x06\x03W\x80_aHSV[\x90P4a\x12(W_`\x03\x196\x01\x12a\x12(Wa\x0C,\x80\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aN!W\x82\x91a\\\xAF\x839\x03\x90_\xF0\x80\x15aN\x16W`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`@Qa\n\xB2\x80\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aN!W\x82\x91ah\xDB\x839\x03\x90_\xF0\x80\x15aN\x16W`\x01`\x01`\xA0\x1B\x03\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x90`\x01`\x01`\xA0\x1B\x03`%T\x16\x92`&T\x93`@Q\x94a\x1BT\x91\x82\x87\x01\x93\x87\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17aN!Wa\x01\0\x96\x88\x96c\xFF\xFF\xFF\xFF\x95as\x8D\x899\x86R` \x86\x01R`@\x85\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0``\x85\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`\x80\x85\x01R`\xA0\x84\x01R`\x01`\x01`\xA0\x1B\x03\x81\x16`\xC0\x84\x01R`\xA0\x1C\x16`\xE0\x82\x01R\x03\x01\x90_\xF0\x80\x15aN\x16W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x12(W_\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rj\x08E\x95\x16\x14\x01HJ\0\0\0`$\x84\x01RZ\xF1\x80\x15aN\x16WaN\x08WP\x80\xF3[aN\x14\x91P_\x90aPXV[\0[`@Q=_\x82>=\x90\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10aNqWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aNdV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10aN\xF0WPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aN\xE3V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aOZWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aO\x96\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89QaN\x90V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aOKV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aO\xD7WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aP-\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90aN\xD3V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aO\xC8V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aN!W`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aN!W`@RV[\x91\x90\x82\x01\x80\x92\x11aP\xA6WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[Q\x90\x81\x15\x15\x82\x03a\x12(WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11aN!W`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15aQ\xF1W[` \x85\x10\x84\x14aQ\xC4W\x84\x87R\x86\x93\x90\x81\x15aQ\x84WP`\x01\x14aQ@W[PaQ>\x92P\x03\x83aPXV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10aQhWPP\x90` aQ>\x92\x82\x01\x01_aQ1V[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92aQOV[` \x93PaQ>\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_aQ1V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93aQ\x12V[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x12(WV[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x12(WV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10aT7WaQ>\x94T\x91\x81\x81\x10aT\x01W[\x81\x81\x10aS\xCBW[\x81\x81\x10aS\x95W[\x81\x81\x10aS_W[\x81\x81\x10aS)W[\x81\x81\x10aR\xF3W[\x81\x81\x10aR\xBEW[\x10aR\x91W[P\x03\x83aPXV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_aR\x89V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01aR\x83V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01aR{V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01aRsV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01aRkV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01aRcV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01aR[V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01aRSV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91aR;V[`\x08T`\xFF\x16\x80\x15aT\xD3W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15aN\x16W_\x91aUkW[P\x15\x15\x90V[\x90P` \x81=` \x11aU\x95W[\x81aU\x86` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WQ_aUeV[=\x91PaUyV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11aN!W`@Q\x91aU\xE5`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x84aPXV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x12(W\x82\x81` \x93\x84_\x96\x01^\x01\x01RV[` \x81\x83\x03\x12a\x12(W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x12(W\x01\x90`\xC0\x82\x82\x03\x12a\x12(W`@Q\x91`\xC0\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aN!W`@RaVP\x81aQ\xFBV[\x83RaV^` \x82\x01aQ\xFBV[` \x84\x01RaVo`@\x82\x01aQ\xFBV[`@\x84\x01R``\x81\x01Q``\x84\x01RaV\x8A`\x80\x82\x01aR\x0FV[`\x80\x84\x01R`\xA0\x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x12(W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x12(W\x81QaV\xBF\x92` \x01aU\x9DV[`\xA0\x82\x01R\x90V[\x91\x90\x82\x03\x91\x82\x11aP\xA6WV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x02`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[_aQ>\x91aPXV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ri\x9E\xD1\x94\xDB\x19\xB28\xC0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[\x81\x15aZ\xBEW\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x90\x83\x83\x11a\\*W\x82\x81\x10\x91\x82\x15\x80a\\ W[a\\\x18Wa[\x0E\x84\x86aV\xC7V[\x92`\x01\x84\x01\x80\x94\x11aP\xA6W`\x03\x83\x11\x15\x80a\\\x0FW[a\\\0W`\x03\x19\x83\x10\x15\x80a[\xF6W[a[\xE2W\x85\x83\x11\x15a[\x99WPP\x90a[Q\x84a[V\x93aV\xC7V[aZ\xB4V[\x90\x81\x15a[\x94Wa[g\x92PaP\x99V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11aP\xA6W\x90V[PP\x90V[\x95\x94\x92\x91\x90\x95a[\xAAW[PPPPV[\x83\x94\x95Pa[Q\x90a[\xBC\x93\x94aV\xC7V[\x90\x81\x15a[\x94Wa[\xCD\x92PaV\xC7V[`\x01\x81\x01\x80\x91\x11aP\xA6W\x90_\x80\x80\x80a[\xA4V[PP\x90Pa[\xF3\x92\x91P\x19\x90aV\xC7V[\x90V[P\x82\x19\x84\x11a[5V[PP\x91\x90Pa[\xF3\x92PaP\x99V[P\x82\x84\x11a[%V[P\x92PPP\x90V[P\x84\x82\x11\x15a[\0V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R\xFD\xFE`\x80`@R4a\x03\x13W`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17a\x02&W`@\x90\x81R`\t\x82RhERC20Mock`\xB8\x1B` \x83\x01R\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17a\x02&W`@R`\x04\x81RcE20M`\xE0\x1B` \x82\x01R\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x02&W`\x03T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x03\tW[` \x82\x10\x14a\x02\x08W`\x1F\x81\x11a\x02\xA6W[P` \x92`\x1F\x82\x11`\x01\x14a\x02EW\x92\x81\x92\x93_\x92a\x02:W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x03U[\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x02&W`\x04T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x02\x1CW[` \x82\x10\x14a\x02\x08W`\x1F\x81\x11a\x01\xA5W[P` \x91`\x1F\x82\x11`\x01\x14a\x01EW\x91\x81\x92_\x92a\x01:W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x04U[`@Qa\t\x14\x90\x81a\x03\x18\x829\xF3[\x01Q\x90P_\x80a\x01\x16V[`\x1F\x19\x82\x16\x92`\x04_R\x80_ \x91_[\x85\x81\x10a\x01\x8DWP\x83`\x01\x95\x10a\x01uW[PPP\x81\x1B\x01`\x04Ua\x01+V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x01gV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x01UV[`\x04_R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B`\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x01\xFEW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x01\xF3WPa\0\xFDV[_\x81U`\x01\x01a\x01\xE6V[\x90\x91P\x81\x90a\x01\xDDV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x7F\x16\x90a\0\xEBV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x01Q\x90P_\x80a\0\xB5V[`\x1F\x19\x82\x16\x93`\x03_R\x80_ \x91_[\x86\x81\x10a\x02\x8EWP\x83`\x01\x95\x96\x10a\x02vW[PPP\x81\x1B\x01`\x03Ua\0\xCAV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02hV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x02UV[`\x03_R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[`\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x02\xFFW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x02\xF4WPa\0\x9BV[_\x81U`\x01\x01a\x02\xE7V[\x90\x91P\x81\x90a\x02\xDEV[\x90`\x7F\x16\x90a\0\x89V[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x06\xFD\xDE\x03\x14a\x07\x03WP\x80c\t^\xA7\xB3\x14a\x06tW\x80c\x18\x16\r\xDD\x14a\x06WW\x80c#\xB8r\xDD\x14a\x04\xE1W\x80c1<\xE5g\x14a\x04\xC6W\x80c@\xC1\x0F\x19\x14a\x03\xE5W\x80cp\xA0\x821\x14a\x03\xA1W\x80c\x95\xD8\x9BA\x14a\x02&W\x80c\x9D\xC2\x9F\xAC\x14a\x018W\x80c\xA9\x05\x9C\xBB\x14a\x01\x07Wc\xDDb\xED>\x14a\0\x95W_\x80\xFD[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\0\xAEa\x08\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\xCBa\x08'V[\x91\x16_R`\x01` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `@_ T`@Q\x90\x81R\xF3[_\x80\xFD[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x01-a\x01#a\x08\x04V[`$5\x903a\x08JV[` `@Q`\x01\x81R\xF3[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x01Qa\x08\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x91\x16\x80\x15a\x01\xFAW\x80_R_` R`@_ T\x82\x81\x10a\x01\xC8W` \x83_\x94\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93\x85\x87R\x86\x84R\x03`@\x86 U\x80`\x02T\x03`\x02U`@Q\x90\x81R\xA3\0[\x90\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`DR`d_\xFD[\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W`@Q_`\x04T\x90\x81`\x01\x1C`\x01\x83\x16\x92\x83\x15a\x03\x97W[` \x82\x10\x84\x14a\x03jW\x81\x85R\x84\x93\x90\x81\x15a\x03(WP`\x01\x14a\x02\xCCW[P\x03`\x1F\x01`\x1F\x19\x16\x81\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x81\x83\x10\x17a\x02\x9FWa\x02\x9B\x82\x91\x82`@R\x82a\x07\xDAV[\x03\x90\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x04_\x90\x81R\x91P\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B[\x81\x83\x10a\x03\x0CWPP\x81\x01` \x01`\x1F\x19a\x02mV[` \x91\x93P\x80`\x01\x91T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x91\x83\x92a\x02\xF6V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x85\x81\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91P`\x1F\x19\x90Pa\x02mV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x90`\x7F\x16\x90a\x02NV[4a\x01\x03W` `\x03\x196\x01\x12a\x01\x03Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x03\xCFa\x08\x04V[\x16_R_` R` `@_ T`@Q\x90\x81R\xF3[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x03\xFEa\x08\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`$5\x81\x15a\x04\x9AW`\x02T\x90\x80\x82\x01\x80\x92\x11a\x04mW` \x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91_\x93`\x02U\x84\x84R\x83\x82R`@\x84 \x81\x81T\x01\x90U`@Q\x90\x81R\xA3\0[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W` `@Q`\x12\x81R\xF3[4a\x01\x03W```\x03\x196\x01\x12a\x01\x03Wa\x04\xFAa\x08\x04V[a\x05\x02a\x08'V[`D5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x92\x83_R`\x01` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`@_ T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x10a\x05~W[Pa\x01-\x93Pa\x08JV[\x83\x81\x10a\x06#W\x84\x15a\x05\xF7W3\x15a\x05\xCBWa\x01-\x94_R`\x01` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R\x83`@_ \x91\x03\x90U\x84a\x05sV[\x7F\x94(\rb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\xE6\x02\xDF\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x83\x90\x7F\xFB\x8FA\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`DR`d_\xFD[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W` `\x02T`@Q\x90\x81R\xF3[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x06\x8Da\x08\x04V[`$5\x903\x15a\x05\xF7Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x05\xCBW3_R`\x01` R`@_ \x82_R` R\x80`@_ U`@Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` 3\x92\xA3` `@Q`\x01\x81R\xF3[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W_`\x03T\x90\x81`\x01\x1C`\x01\x83\x16\x92\x83\x15a\x07\xD0W[` \x82\x10\x84\x14a\x03jW\x81\x85R\x84\x93\x90\x81\x15a\x03(WP`\x01\x14a\x07tWP\x03`\x1F\x01`\x1F\x19\x16\x81\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x81\x83\x10\x17a\x02\x9FWa\x02\x9B\x82\x91\x82`@R\x82a\x07\xDAV[`\x03_\x90\x81R\x91P\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[[\x81\x83\x10a\x07\xB4WPP\x81\x01` \x01`\x1F\x19a\x02mV[` \x91\x93P\x80`\x01\x91T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x91\x83\x92a\x07\x9EV[\x90`\x7F\x16\x90a\x07(V[`\x1F\x19`\x1F` `@\x94\x81\x85R\x80Q\x91\x82\x91\x82\x82\x88\x01R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\x03WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\x03WV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x01\xFAWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x82\x15a\x04\x9AW\x81_R_` R`@_ T\x81\x81\x10a\x08\xE2W\x81\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92` \x92\x85_R_\x84R\x03`@_ U\x84_R_\x82R`@_ \x81\x81T\x01\x90U`@Q\x90\x81R\xA3V[\x82\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`DR`d_\xFD`\x80\x80`@R4`\x15Wa\n\x98\x90\x81a\0\x1A\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c7\x1B\xEDh\x14a\x08kWP\x80cS\x06`i\x14a\x06\xADW\x80ch\x13\xD7\x87\x14a\x06^W\x80c\x83\x8B% \x14a\x01LW\x80c\x95\x9B\x82Z\x14a\0\x84Wc\xD3\x07-\x82\x14a\0^W_\x80\xFD[4a\0\x80W_`\x03\x196\x01\x12a\0\x80W` `\xFF`\x01T\x16`@Q\x90\x15\x15\x81R\xF3[_\x80\xFD[4a\0\x80W` `\x03\x196\x01\x12a\0\x80W`\x045_T\x81\x10\x15a\0\x80Wa\0\xAA\x90a\x08\xC7V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x16a\x01Hs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01\x84\x01T\x16\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02\x82\x01T\x16\x90`\x03\x81\x01Ta\x01\x1A`\x05c\xFF\xFF\xFF\xFF`\x04\x85\x01T\x16\x93\x01a\t\xDAV[\x92`@Q\x96\x87\x96\x87R` \x87\x01R`@\x86\x01R``\x85\x01R`\x80\x84\x01R`\xC0`\xA0\x84\x01R`\xC0\x83\x01\x90a\x08\x84V[\x03\x90\xF3[4a\0\x80W`\xC0`\x03\x196\x01\x12a\0\x80W`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\0\x80W`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\0\x80W`D5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\0\x80W`d5`\x845\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\0\x80W`\xA45\x94g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11a\0\x80W6`#\x87\x01\x12\x15a\0\x80W\x85`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\x80W6`$\x82\x89\x01\x01\x11a\0\x80W`\xFF`\x01T\x16a\x06\0W`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R0`$\x82\x01R\x83`D\x82\x01R` \x81`d\x81_\x8BZ\xF1\x80\x15a\x05\xF5Wa\x05\xBAW[P` _\x91`@Q\x97a\x02}\x89a\t}V[\x88R\x81\x88\x01\x93\x84R`@\x88\x01\x96\x87R``\x88\x01\x94\x85R`\x80\x88\x01\x95\x86R\x80`$`@Q\x9Aa\x02\xD2\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x86\x01\x16\x01\x8Da\t\x99V[\x82\x8CR\x01\x83\x8B\x017\x88\x01\x01R`\xA0\x85\x01\x95\x86R_Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x05aW\x80`\x01a\x03\x07\x92\x01_Ua\x08\xC7V[\x94\x90\x94a\x05\x8EWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x92\x81\x80`\x05\x99Q\x16\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88T\x16\x17\x87UQ\x16\x82`\x01\x87\x01\x91\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UQ\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02\x85\x01\x91\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UQ`\x03\x83\x01Uc\xFF\xFF\xFF\xFF`\x04\x83\x01\x91Q\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x82T\x16\x17\x90U\x01\x90Q\x90\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05aWa\x04 \x82Ta\t,V[`\x1F\x81\x11a\x05\x1CW[P` \x92`\x1F\x82\x11`\x01\x14a\x04\x83W\x92\x81\x92\x93_\x92a\x04xW[PP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17\x90U_\x80\xF3[\x01Q\x90P\x83\x80a\x04CV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x16\x93\x83_R\x80_ \x91_[\x86\x81\x10a\x05\x04WP\x83`\x01\x95\x96\x10a\x04\xCDW[PPP\x81\x1B\x01\x90U\0[\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x83\x80\x80a\x04\xC3V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x04\xB0V[\x82_R` _ `\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x05WW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x05LWPa\x04)V[_\x81U`\x01\x01a\x05?V[\x90\x91P\x81\x90a\x056V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[` \x81=` \x11a\x05\xEDW[\x81a\x05\xD3` \x93\x83a\t\x99V[\x81\x01\x03\x12a\0\x80WQ\x80\x15\x15\x81\x03a\0\x80WP` a\x02kV[=\x91Pa\x05\xC6V[`@Q=_\x82>=\x90\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FOptimism bridge failed\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[4a\0\x80W` `\x03\x196\x01\x12a\0\x80W`\x045\x80\x15\x15\x80\x91\x03a\0\x80W`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x01T\x16\x91\x16\x17`\x01U_\x80\xF3[4a\0\x80W_`\x03\x196\x01\x12a\0\x80W```\xA0`@Qa\x06\xCD\x81a\t}V[_\x81R_` \x82\x01R_`@\x82\x01R_\x83\x82\x01R_`\x80\x82\x01R\x01R_T\x80\x15a\x08\rW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a\x07\xE0Wa\x079s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x08\xC7V[Pa\x01H`@Q\x91a\x07J\x83a\t}V[\x83\x81T\x16\x83Rc\xFF\xFF\xFF\xFF\x84`\x01\x83\x01T\x16\x91` \x85\x01\x92\x83R\x85`\x02\x82\x01T\x16`@\x86\x01\x90\x81R\x86`\x03\x83\x01T\x91``\x88\x01\x92\x83R\x81a\x07\x9B`\x05\x87`\x04\x88\x01T\x16\x96`\x80\x8C\x01\x97\x88R\x01a\t\xDAV[\x96`\xA0\x8A\x01\x97\x88R`@Q\x9A\x8B\x9A` \x8CRQ\x16` \x8B\x01RQ\x16`@\x89\x01RQ\x16``\x87\x01RQ`\x80\x86\x01RQ\x16`\xA0\x84\x01RQ`\xC0\x80\x84\x01R`\xE0\x83\x01\x90a\x08\x84V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7FNo deposits\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[4a\0\x80W_`\x03\x196\x01\x12a\0\x80W` \x90_T\x81R\xF3[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[_T\x81\x10\x15a\x08\xFFW_\x80\x80R`\x06\x91\x90\x91\x02\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x01\x91V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\tsW[` \x83\x10\x14a\tFWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\t;V[`\xC0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05aW`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05aW`@RV[\x90`@Q\x91\x82_\x82T\x92a\t\xED\x84a\t,V[\x80\x84R\x93`\x01\x81\x16\x90\x81\x15a\nXWP`\x01\x14a\n\x14W[Pa\n\x12\x92P\x03\x83a\t\x99V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a\n<WPP\x90` a\n\x12\x92\x82\x01\x01_a\n\x05V[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\n#V[` \x93Pa\n\x12\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a\n\x05V`\x80`@R4a\x03'W`@Q`\x1Fa\x1BT8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x02\xF3W\x80\x84\x92a\x01\0\x94`@R\x839\x81\x01\x03\x12a\x03'Wa\0N\x81a\x03+V[a\0Z` \x83\x01a\x03+V[a\0f`@\x84\x01a\x03+V[``\x84\x01Q`\x80\x85\x01Qa\0|`\xA0\x87\x01a\x03+V[\x93`\xE0a\0\x8B`\xC0\x89\x01a\x03+V[\x97\x01Q\x95c\xFF\xFF\xFF\xFF\x87\x16\x87\x03a\x03'W`@\x80Q\x92\x90\x83\x01`\x01`\x01`@\x1B\x03\x81\x11\x84\x82\x10\x17a\x02\xF3W`@R`\x0F\x83RnOptimism Bridge`\x88\x1B` \x84\x01R`\x01\x80U`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15a\x03\x16W[a\x03\x07Wa\x01\x05\x82a\0\xFFa\x01\x0B\x94a\x03?V[Pa\x03\xB5V[Pa\x04HV[P\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xF3W`\x07T\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x02\xE9W[` \x83\x10\x14a\x02\xD5W\x81`\x1F\x84\x93\x11a\x02gW[P` \x90`\x1F\x83\x11`\x01\x14a\x02\x01W_\x92a\x01\xF6W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x07U[`\x02\x80T`\x03\x93\x90\x93U`\x04\x91\x90\x91U`\x01`\x01`\xA8\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17`\x01`\xA0\x1B\x17\x90Ub\x01Q\x80B\x04`\x06U`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91U`\t\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16\x93\x90\x91\x16\x92\x90\x92\x17`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90U`@Qa\x15\xF8\x90\x81a\x04\xDC\x829\xF3[\x01Q\x90P_\x80a\x01YV[`\x07_\x90\x81R\x82\x81 \x93P`\x1F\x19\x85\x16\x90[\x81\x81\x10a\x02OWP\x90\x84`\x01\x95\x94\x93\x92\x10a\x027W[PPP\x81\x1B\x01`\x07Ua\x01nV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02)V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x02\x13V[`\x07_R\x90\x91P\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x02\xCBW[\x90`\x1F\x85\x94\x93\x92\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x02\xBDWPa\x01CV[_\x81U\x84\x93P`\x01\x01a\x02\xB0V[\x90\x91P\x81\x90a\x02\xA2V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x01/V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\0\xEBV[_\x80\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03'WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x1B4_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x03\xB0W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x1B4_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a\x1A\xD4_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x1B\x14_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x03\xB0W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x1B\x14_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7Fu\x1By]$\xB9.=\x92\xD1\xD0\xD8\xF2\x88_N\x9C\x9C&\x9D\xA3P\xAF6\xAEkI\x06\x9B\xAB\xF4\xBF\x90_Q` a\x1A\xD4_9_Q\x90_R\x90\x80\xA4`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x1A\xF4_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x03\xB0W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x1A\xF4_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\xC0t\xBE\xB4k%\x1Fs\xC0\xFE\xC1n\xBAu\xD6\xBC\0B\xD0\x9E\xE1zt\x0E\xBB\xEE0\x8D\xBF\x87\xF8/\x90_Q` a\x1A\xD4_9_Q\x90_R\x90\x80\xA4`\x01\x90V\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x01\xFF\xC9\xA7\x14a\x10\x01WP\x80c\x103\xB4\xCC\x14a\x0F\xE4W\x80c\x11\x8C8\xC7\x14a\x0F\xAAW\x80c\x12Y\xA5\xC8\x14a\x0F\x8DW\x80c\x18\xB6\x8B\x8C\x14a\n\x1EW\x80c$\x8A\x9C\xA3\x14a\t\xEBW\x80c//\xF1]\x14a\t\xADW\x80c2\xD4\xF5\x04\x14a\t[W\x80c4b\xFA\xC3\x14a\t W\x80c6V\x8A\xBE\x14a\x08\xB5W\x80c6\xB0\x89\xD8\x14a\x08\x97W\x80c<\xBBiy\x14a\x08pW\x80cV\xEF\xF2g\x14a\x08<W\x80cZ\xB1\xD6\x1C\x14a\x07\x9EW\x80ca\xB0\xA5n\x14a\x06aW\x80cc\"\x14\x90\x14a\x06>W\x80ce\xD7\xA3\xC9\x14a\x06\nW\x80cf\xD0\x03\xAC\x14a\x05\xD6W\x80cg\xEE\xBA\x0C\x14a\x05\xB8W\x80ck\xCC\x8C\x14\x14a\x04\xEAW\x80c\x85\x93\x1Bt\x14a\x03\xA2W\x80c\x91\xD1HT\x14a\x03KW\x80c\xA2\x17\xFD\xDF\x14a\x03/W\x80c\xB2\r0\xA9\x14a\x02\xD9W\x80c\xC9\xF5\xB6>\x14a\x02\xA5W\x80c\xD5Gt\x1F\x14a\x02^W\x80c\xEA\xD9<\x8F\x14a\x028W\x80c\xED\xE7\xCE\xBD\x14a\x01\xD6Wc\xFB\x8CKQ\x14a\x01]W_\x80\xFD[4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W`\x05T`\x04T\x81\x81\x11\x15a\x01\xCAW\x81\x81\x03\x81\x81\x11a\x01\x9DW``\x93P\x90[`@Q\x92\x83R` \x83\x01R`@\x82\x01R\xF3[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[``\x92\x90a\x01\x8BV[\x80\xFD[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3Wa\x02\x0C`\x02Ta\x01\xF5a\x11SV[\x90`\xFF`@Q\x93\x84\x93``\x85R``\x85\x01\x90a\x12lV[\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16` \x85\x01R`\xA0\x1C\x16\x15\x15`@\x83\x01R\x03\x90\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` `\xFF`\x02T`\xA0\x1C\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x02\xA1`\x045a\x02~a\x10\xC2V[\x90a\x02\x9Ca\x02\x97\x82_R_` R`\x01`@_ \x01T\x90V[a\x13qV[a\x14\xA9V[P\x80\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16`@Q\x90\x81R\xF3[P4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W\x7F |L\xBD\xF5^\xC3\x15\xA1?\r^\x04w2\xEC]\x94}\xA0V\xE7\x06Y:\xA5\t\x90\x99A\xCE\xDF`@`\x045a\x03\x19a\x12\xE9V[`\x04T\x90\x80`\x04U\x82Q\x91\x82R` \x82\x01R\xA1\x80\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` \x90`@Q\x90\x81R\xF3[P4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@a\x03|a\x10\xC2V[\x92`\x045\x81R\x80` R \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xD3W```\x03\x196\x01\x12a\x01\xD3Wa\x03\xBCa\x10\x9FV[a\x03\xC4a\x10\xC2V[`D5\x91c\xFF\xFF\xFF\xFF\x83\x16\x83\x03a\x04\xE6Wa\x04\xE0\x7F\x80+\x8C{$p\x9Bl\x9CV\x17\x9D\xCE\xEB\x97|\xC7\xACo\xA4\xF1_\x84\xC9\x9A\x86'\xAB\xFD\x97\xCC5\x93a\x04\x02a\x12\xE9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x08T\x16\x17`\x08U`\tTw\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`\xA0\x1B\x16\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x91\x16\x17\x17`\tU`@Q\x93\x84\x93\x84\x91`@\x91\x94\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFc\xFF\xFF\xFF\xFF\x92\x81``\x87\x01\x98\x16\x86R\x16` \x85\x01R\x16\x91\x01RV[\x03\x90\xA1\x80\xF3[\x83\x80\xFD[P4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\x19a\x10\x9FV[a\x05!a\x12\xE9V[\x16\x80\x15a\x05\x90Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17`\x02U\x16\x7F\xB0\x7F\x8B\x1B\x85\x04-t\x02,\x86|\x83n\xDE\xB0\xBC\xD7\x0E\x13[\0B9\r+\x1F\xD1\x08)\x80i\x83\x80\xA3\x80\xF3[`\x04\x82\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` `\x04T`@Q\x90\x81R\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\tT\x16`@Q\x90\x81R\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3Wa\x06:a\x06&a\x11SV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x12lV[\x03\x90\xF3[P4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3Wa\x06Xa\x12\xE9V[`\x045`\x03U\x80\xF3[P4a\x01\xD3W```\x03\x196\x01\x12a\x01\xD3Wa\x06{a\x10\x9FV[`D5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x82\x03a\x04\xE6W\x83\x80R\x83` R`@\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x07nW\x15a\x07FW`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`$\x80\x83\x01\x91\x90\x91R5`D\x80\x83\x01\x91\x90\x91R\x81Ra\x07C\x92\x90\x91\x90a\x07=`d\x84a\x10\xE5V[\x16a\x15qV[\x80\xF3[`\x04\x83\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`D\x84\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04R\x80`$R\xFD[P4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W`\x045\x80\x15\x15\x80\x91\x03a\x088W` \x7F\xB3A\x89\x89\xD0h5\xB5\xC2\x15\xEE\xBBMT\xEDk\xE7\xBB\xB6n\xB4\x80qdt\n.\x08/\xA7\x82\xD5\x91a\x07\xE9a\x12\xE9V[`\x02T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\xA0\x1B\x16\x91\x16\x17`\x02U`@Q\x90\x81R\xA1\x80\xF3[P\x80\xFD[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x08T\x16`@Q\x90\x81R\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` c\xFF\xFF\xFF\xFF`\tT`\xA0\x1C\x16`@Q\x90\x81R\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` `\x03T`@Q\x90\x81R\xF3[P4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x08\xCFa\x10\xC2V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\x08\xF8Wa\x02\xA1\x90`\x045a\x14\xA9V[`\x04\x82\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` `@Q\x7F\xC0t\xBE\xB4k%\x1Fs\xC0\xFE\xC1n\xBAu\xD6\xBC\0B\xD0\x9E\xE1zt\x0E\xBB\xEE0\x8D\xBF\x87\xF8/\x81R\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W`\x08T`\tT`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x82\x16` \x84\x01R`\xA0\x91\x90\x91\x1Cc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x90\xF3[P4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x02\xA1`\x045a\t\xCDa\x10\xC2V[\x90a\t\xE6a\x02\x97\x82_R_` R`\x01`@_ \x01T\x90V[a\x13\xD7V[P4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W` a\n\x16`\x045_R_` R`\x01`@_ \x01T\x90V[`@Q\x90\x81R\xF3[P4a\r\xB5W```\x03\x196\x01\x12a\r\xB5Wa\n8a\x10\x9FV[`$5\x90`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r\xB5W6`#\x82\x01\x12\x15a\r\xB5W\x80`\x04\x015\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\r\xB5W\x82\x82\x01\x906`$\x83\x01\x11a\r\xB5W`\x02`\x01T\x14a\x0FeW`\x02`\x01U`\xFF`\x02T`\xA0\x1C\x16\x15a\x0F=W3_\x90\x81R\x7F\xFE\x0C{\xD5\t,znF4\x88\xFA#\x9F2\x05\x8C\x87\xD4>3n.XJ'!2\xCF\x8D\xC9&` R`@\x90 T`\xFF\x16\x15a\x0F\x15Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x83\x15a\x0E\xEDW\x84\x15a\x0E\xC5W`\x03T\x85\x11a\x0E^Wb\x01Q\x80B\x04`\x06T\x81\x11a\x0E\x86W[Pa\x0B\x18\x85`\x05Ta\x12\xAFV[`\x04T\x10a\x0E^Wa\x0Bs`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R3`$\x82\x01R0`D\x82\x01R\x86`d\x82\x01R`d\x81Ra\x0Bm`\x84\x82a\x10\xE5V[\x85a\x15qV[\x15a\x0E3W`@\x90\x82\x90\x03\x12a\r\xB5W`$\x81\x015\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\r\xB5W`D\x015\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\r\xB5W\x90[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16`@Q` _\x81\x83\x01\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x0Cb\x84a\x0C6\x8B\x88`$\x84\x01` \x90\x93\x92\x91\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01\x95\x16\x81R\x01RV[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x86R\x85a\x10\xE5V[\x83Q\x90\x82\x89Z\xF1_Q=\x82a\x0E\x17W[PP\x15a\r\xB9W[PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x08T\x16\x92\x80;\x15a\r\xB5Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93_`\xE4\x92c\xFF\xFF\xFF\xFF\x82\x96`@Q\x98\x89\x97\x88\x96\x7F\x83\x8B% \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88R\x8C`\x04\x89\x01R`$\x88\x01R\x16`D\x86\x01R\x8A`d\x86\x01R\x16`\x84\x84\x01R`\xC0`\xA4\x84\x01R\x81`\xC4\x84\x01RZ\xF1\x80\x15a\r\xAAWa\r\x95W[Pa\rG\x82`\x05Ta\x12\xAFV[`\x05U\x7F=\xBA@\x1D\xC1\xAB\xBF\x01p\x86\x86K3\xBA\x81_S\xAB<\xDB\xB96k\xA6\xBC\xD8\xEC4-\xDD\x98\xE8` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16\x93`@Q\x90\x81R\xA3`\x01\x80U\x80\xF3[a\r\xA2\x91\x93P_\x90a\x10\xE5V[_\x91_a\r:V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[a\x0E\na\x0E\x10\x92`@Q\x90\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01R_`D\x82\x01R`D\x81Ra\x0E\x04`d\x82a\x10\xE5V[\x86a\x15qV[\x84a\x15qV[_\x80a\x0CzV[\x90\x91Pa\x0E+WP\x84;\x15\x15[_\x80a\x0CrV[`\x01\x14a\x0E$V[PP`\tTc\xFF\xFF\xFF\xFFs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x91`\xA0\x1C\x16a\x0B\xBCV[\x7Fp\xD1h\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xDAN9\xDDV\xD7,.\xE3\xD12\xE0\x14k\xC3\x9E\x90^x\xE3\xBCd\xC4\x01\x90B\x1C{+\xCE\xF2\xAB`@`\x05T\x81Q\x90\x84\x82R` \x82\x01R\xA1_`\x05U`\x06U_a\x0B\x0BV[\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\\B|\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F{\xEA \xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\r\xB5W_`\x03\x196\x01\x12a\r\xB5W` `\x06T`@Q\x90\x81R\xF3[4a\r\xB5W_`\x03\x196\x01\x12a\r\xB5W` `@Q\x7Fu\x1By]$\xB9.=\x92\xD1\xD0\xD8\xF2\x88_N\x9C\x9C&\x9D\xA3P\xAF6\xAEkI\x06\x9B\xAB\xF4\xBF\x81R\xF3[4a\r\xB5W_`\x03\x196\x01\x12a\r\xB5W` `\x05T`@Q\x90\x81R\xF3[4a\r\xB5W` `\x03\x196\x01\x12a\r\xB5W`\x045\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x80\x92\x03a\r\xB5W\x81\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x93\x14\x90\x81\x15a\x10uW[P\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x83a\x10nV[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\xB5WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\xB5WV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x11&W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q\x90_`\x07T\x80`\x01\x1C\x91`\x01\x82\x16\x91\x82\x15a\x12bW[` \x84\x10\x83\x14a\x125W\x83\x86R\x85\x92\x90\x81\x15a\x11\xF8WP`\x01\x14a\x11\x99W[a\x11\x97\x92P\x03\x83a\x10\xE5V[V[P`\x07_\x90\x81R\x90\x91\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88[\x81\x83\x10a\x11\xDCWPP\x90` a\x11\x97\x92\x82\x01\x01a\x11\x8BV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x11\xC4V[` \x92Pa\x11\x97\x94\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01a\x11\x8BV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x92`\x7F\x16\x92a\x11lV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x91\x90\x82\x01\x80\x92\x11a\x12\xBCWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[3_\x90\x81R\x7F\xDF\xFF\x91\xBC\xEE\x88\xF6\xEA\0\xB2rom\x06*P\x9C2\x83W\x93G\x0CU\xBE\xD3\xBC\xC7L\x0F]7` R`@\x90 T`\xFF\x16\x15a\x13!WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7Fu\x1By]$\xB9.=\x92\xD1\xD0\xD8\xF2\x88_N\x9C\x9C&\x9D\xA3P\xAF6\xAEkI\x06\x9B\xAB\xF4\xBF`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x13\xA8WPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x14\xA3W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x14\xA3W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V[\x90_` \x91\x82\x81Q\x91\x01\x82\x85Z\xF1\x15a\r\xAAW_Q=a\x15\xEFWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;\x15[a\x15\xADWPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[`\x01\x14\x15a\x15\xA6V/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\xFE\x0C{\xD5\t,znF4\x88\xFA#\x9F2\x05\x8C\x87\xD4>3n.XJ'!2\xCF\x8D\xC9&\xDF\xFF\x91\xBC\xEE\x88\xF6\xEA\0\xB2rom\x06*P\x9C2\x83W\x93G\x0CU\xBE\xD3\xBC\xC7L\x0F]7\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f905f3560e01c9081630a9254e414614bd0575080630d170b02146147c65780630d1ce0dd14614472578063117e3b421461444d5780631ed7831c146143cf578063248ec326146143a95780632ade3880146141b55780632cd38fbf146140aa578063374e0ce614613d515780633cbb697914613d2a5780633e5e3c2314613cac5780633f7286f414613c2e5780634f8632ba14613c0757806352743ec41461375f57806356eff2671461373857806366d003ac1461371157806366d9a9a0146135d4578063702877781461335c5780637b4d4ce31461333657806385226c81146132ac578063916a17c614613202578063a3d4485b146131d8578063b0464fdc1461312e578063b44dc9d614612cfe578063b5508aa914612c74578063ba414fa614612c4f578063ccc0cfac14612688578063cffb048b14612267578063d8b296da14611d45578063dccc57f1146119f4578063e178bc5b1461182a578063e20c9f711461179c578063f2067bd71461140a578063f697e78a14610c73578063f81006b214610782578063f851a4401461075b578063fa7626d414610738578063fc0c546a14610711578063fc9c8d39146106ea5763fe47a3f4146101d6575f80fd5b34610603578060031936011261060357806040516122226020820152620493e060408201526040815261020a606082615058565b6001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610634576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156106295783916106d5575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b30000000000000000000000000000000000000000000000000000000084526004840152693f870857a3e0e380000060248401525af180156106295761069e575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610634576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610629578391610689575b50506001600160a01b03601f5460081c16906001600160a01b036021541690823b15610684576103c1928492836040518096819582947f18b68b8c0000000000000000000000000000000000000000000000000000000084526004840152693f870857a3e0e38000006024840152606060448401526064830190614e90565b03925af180156106065761066f575b506004816001600160a01b0360205416604051928380927f530660690000000000000000000000000000000000000000000000000000000082525afa90811561060657829161064d575b5061043b6001600160a01b038251166001600160a01b0360215416906159c1565b61045e6001600160a01b036020830151166001600160a01b0360255416906159c1565b6001600160a01b03604082015116737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561063457604051907f515361f6000000000000000000000000000000000000000000000000000000008252600482015261222260248201528281604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115610629578391610638575b50506060810151737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561063457604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152693f870857a3e0e380000060248201528281604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115610629578391610614575b50506080015163ffffffff16737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561061157604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152620493e060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610606576105f25750f35b816105fc91615058565b6106035780f35b80fd5b6040513d84823e3d90fd5b50fd5b8161061e91615058565b61061157815f61056d565b6040513d85823e3d90fd5b5050fd5b8161064291615058565b61061157815f6104e5565b61066991503d8084833e6106618183615058565b810190615601565b5f61041a565b8161067991615058565b61060357805f6103d0565b505050fd5b8161069391615058565b61061157815f610342565b6020813d6020116106cd575b816106b760209383615058565b81010312610634576106c8906150d3565b6102dc565b3d91506106aa565b816106df91615058565b61061157815f61026f565b503461060357806003193601126106035760206001600160a01b0360235416604051908152f35b503461060357806003193601126106035760206001600160a01b0360215416604051908152f35b5034610603578060031936011261060357602060ff601f54166040519015158152f35b503461060357806003193601126106035760206001600160a01b0360225416604051908152f35b50346106035780600319360112610603578060206040516107a38282615058565b8281526001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610684576040519063ca669fa760e01b82526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610c0c578491610c5e575b50826001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526969e10de76676d080000060248401525af18015610c0c57610c2c575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610634576040517f491cc7c2000000000000000000000000000000000000000000000000000000008152600160048201528360248201526001604482015260016064820152838160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610c0c578491610c17575b50506001600160a01b03602154166001600160a01b03835416907f3dba401dc1abbf017086864b33ba815f53ab3cdbb9366ba6bcd8ec342ddd98e8846040516969e10de76676d08000008152a36001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610684576040519063ca669fa760e01b82526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610c0c578491610bf7575b50506001600160a01b03601f5460081c16906001600160a01b036021541690823b15610bf357610a30928592836040518096819582947f18b68b8c00000000000000000000000000000000000000000000000000000000845260048401526969e10de76676d08000006024840152606060448401526064830190614e90565b03925af1908115610629578391610bde575b506004906001600160a01b03835416604051928380927f530660690000000000000000000000000000000000000000000000000000000082525afa80156106295760a0918491610bc4575b50610aae6001600160a01b038251166001600160a01b0360215416906159c1565b610ad06001600160a01b0384830151166001600160a01b0360255416906159c1565b610b1b6001600160a01b03604083015116610af8602654916001600160a01b038316906159c1565b610b056060840151615755565b63ffffffff8060808501511691851c169061594b565b0151610b2a6040519283615058565b828252737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561063457610b8c8391610b9e60405194859384937f97624631000000000000000000000000000000000000000000000000000000008552604060048601526044850190614e90565b90600319848303016024850152614e90565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610606576105f25750f35b610bd891503d8086833e6106618183615058565b5f610a8d565b81610be891615058565b61061157815f610a42565b8480fd5b81610c0191615058565b61063457825f6109b1565b6040513d86823e3d90fd5b81610c2191615058565b61063457825f6108ff565b8281813d8311610c57575b610c418183615058565b8101031261068457610c52906150d3565b610877565b503d610c37565b81610c6891615058565b61063457825f61080b565b503461060357806003193601126106035760405161555560208201526202bf20604082015260408152610ca7606082615058565b6001600160a01b0360215416826001600160a01b036023541692604051937f70a08231000000000000000000000000000000000000000000000000000000008552806004860152602085602481875afa9485156106295783956113d3575b5060206001600160a01b038154166024604051809781937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa93841561062957839461139c575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561132f576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610629578391611387575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b30000000000000000000000000000000000000000000000000000000084526004840152699ed194db19b238c0000060248401525af1801561062957611350575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561132f576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610629578391611337575b50506001600160a01b03601f5460081c16906001600160a01b036021541690823b1561133357610f00928492836040518096819582947f18b68b8c0000000000000000000000000000000000000000000000000000000084526004840152699ed194db19b238c000006024840152606060448401526064830190614e90565b03925af180156106065761131a575b50506001600160a01b0360215416916001600160a01b0360235416604051907f70a082310000000000000000000000000000000000000000000000000000000082526004820152602081602481875afa9081156112dd5785916112e8575b507fffffffffffffffffffffffffffffffffffffffffffff612e6b24e64dc7400000820191821161127e5790610fa29161594b565b6001600160a01b0360205416906040517f70a08231000000000000000000000000000000000000000000000000000000008152826004820152602081602481875afa9081156112dd5785916112ab575b50699ed194db19b238c00000820180921161127e57600492611017869593869361594b565b604051938480927f530660690000000000000000000000000000000000000000000000000000000082525afa91821561062957839261125e575b50611066906001600160a01b038351166159c1565b6110896001600160a01b036020830151166001600160a01b0360255416906159c1565b6001600160a01b03604082015116737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561063457604051907f515361f6000000000000000000000000000000000000000000000000000000008252600482015261555560248201528281604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115610629578391611249575b5050608081611127606063ffffffff9401516158cb565b015116737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561061157604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526202bf2060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561060657611234575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa80156106065782906111fc575b6111f991506158cb565b80f35b506020813d60201161122c575b8161121660209383615058565b81010312611228576111f990516111ef565b5f80fd5b3d9150611209565b8161123e91615058565b61060357805f6111a1565b8161125391615058565b61061157815f611110565b611066919250611277903d8086833e6106618183615058565b9190611051565b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d6020116112d5575b816112c660209383615058565b8101031261122857515f610ff2565b3d91506112b9565b6040513d87823e3d90fd5b90506020813d602011611312575b8161130360209383615058565b8101031261122857515f610f6d565b3d91506112f6565b8161132491615058565b61132f57825f610f0f565b8280fd5b8380fd5b8161134191615058565b61134c57815f610e81565b5080fd5b6020813d60201161137f575b8161136960209383615058565b8101031261132f5761137a906150d3565b610e1b565b3d915061135c565b8161139191615058565b61134c57815f610dae565b925092506020823d6020116113cb575b816113b960209383615058565b8101031261122857849151925f610d54565b3d91506113ac565b925093506020823d602011611402575b816113f060209383615058565b8101031261122857849151935f610d05565b3d91506113e3565b503461060357602060031936011261060357600435816001600160a01b03821680830361134c57737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561134c576040517f4c63e56200000000000000000000000000000000000000000000000000000000815281151560048201528281602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115610629578391611787575b505063ffffffff60265460a01c166040519160208301526040820152604081526114cf606082615058565b6001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561132f576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610629578391611772575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b3000000000000000000000000000000000000000000000000000000008452600484015269152d02c7e14af680000060248401525af180156106295761173b575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561132f576040519063ca669fa760e01b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610629578391611726575b50506001600160a01b03601f5460081c16906001600160a01b036021541690823b1561133357611686928492836040518096819582947f18b68b8c000000000000000000000000000000000000000000000000000000008452600484015269152d02c7e14af68000006024840152606060448401526064830190614e90565b03925af1801561060657611711575b50906004916001600160a01b0360205416604051938480927f530660690000000000000000000000000000000000000000000000000000000082525afa80156106295760406001600160a01b03916111f99486916116f7575b500151166159c1565b61170b91503d8088833e6106618183615058565b5f6116ee565b8161171b91615058565b61134c57815f611695565b8161173091615058565b61134c57815f611607565b6020813d60201161176a575b8161175460209383615058565b8101031261132f57611765906150d3565b6115a1565b3d9150611747565b8161177c91615058565b61134c57815f611534565b8161179191615058565b61134c57815f6114a4565b503461060357806003193601126106035760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b81811061180b57611807856117fb81870382615058565b60405191829182614e4e565b0390f35b82546001600160a01b03168452602090930192600192830192016117e4565b50346106035780600319360112610603576004816001600160a01b03601f5460081c16604051928380927fede7cebd0000000000000000000000000000000000000000000000000000000082525afa801561060657828081938293611985575b50604080519161189a8284615058565b600f83527f4f7074696d69736d2042726964676500000000000000000000000000000000006020840152737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561133357610b8c8491611921845195869384937ff320d9630000000000000000000000000000000000000000000000000000000085528760048601526044850190614e90565b0381737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561197c5750611967575b50506119626111f9926001600160a01b0360205416906159c1565b615a42565b8161197191615058565b61132f57825f611947565b513d84823e3d90fd5b93505050503d8083833e6119998183615058565b810160608282031261132f57815167ffffffffffffffff81116113335782019080601f830112156113335781516119d29260200161559d565b90826119ec60406119e5602085016151fb565b93016150d3565b91925f61188a565b50346106035780600319360112610603576001600160a01b03601f5460081c166040517fa217fddf000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610629578391611d13575b506022546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b03166024820181905290602081604481865afa8015610c0c578490611cd8575b611ab49150615a42565b6040517f118c38c7000000000000000000000000000000000000000000000000000000008152602081600481865afa908115610c0c578491611ca4575b506040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101919091526001600160a01b03909116602482015260208180604481015b0381855afa8015610629578390611c69575b611b559150615a42565b6040517f3462fac3000000000000000000000000000000000000000000000000000000008152602081600481855afa908115610629578391611c35575b506023546040517f91d1485400000000000000000000000000000000000000000000000000000000815260048101929092526001600160a01b031660248201529060209082908180604481015b03915afa8015610606578290611bfa575b6111f99150615a42565b506020813d602011611c2d575b81611c1460209383615058565b8101031261134c57611c286111f9916150d3565b611bf0565b3d9150611c07565b90506020813d602011611c61575b81611c5060209383615058565b810103126112285751611bdf611b92565b3d9150611c43565b506020813d602011611c9c575b81611c8360209383615058565b8101031261132f57611c97611b55916150d3565b611b4b565b3d9150611c76565b90506020813d602011611cd0575b81611cbf60209383615058565b810103126112285751611b39611af1565b3d9150611cb2565b506020813d602011611d0b575b81611cf260209383615058565b8101031261133357611d06611ab4916150d3565b611aaa565b3d9150611ce5565b90506020813d602011611d3d575b81611d2e60209383615058565b8101031261122857515f611a51565b3d9150611d21565b5034610603578060031936011261060357737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561060357806040517f491cc7c200000000000000000000000000000000000000000000000000000000815281600482015281602482015281604482015260016064820152818160848183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657612252575b50507f802b8c7b24709b6c9c56179dceeb977cc7ac6fa4f15f84c99a8627abfd97cc3560405180611e2181906203d09060406060840193613333815261444460208201520152565b0390a1806001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156106065761223d575b506001600160a01b03601f5460081c16803b15610611578160405180927f85931b74000000000000000000000000000000000000000000000000000000008252818381611eee60048201906203d09060406060840193613333815261444460208201520152565b03925af1801561060657612228575b506001600160a01b03601f5460081c166040517f56eff267000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156106295783916121ee575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610634576001600160a01b03604051917f515361f600000000000000000000000000000000000000000000000000000000835216600482015261333360248201528281604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa9081156106295783916121d9575b50506040517f66d003ac000000000000000000000000000000000000000000000000000000008152602081600481855afa90811561062957839161219f575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610634576001600160a01b03604051917f515361f600000000000000000000000000000000000000000000000000000000835216600482015261444460248201528281604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa90811561062957839161218a575b50506020600491604051928380927f3cbb69790000000000000000000000000000000000000000000000000000000082525afa908115610606578291612150575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156106115763ffffffff604051917f98296c540000000000000000000000000000000000000000000000000000000083521660048201526203d09060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015610606576105f25750f35b90506020813d602011612182575b8161216b60209383615058565b810103126106115761217c9061520f565b5f6120d0565b3d915061215e565b8161219491615058565b61061157815f61208f565b90506020813d6020116121d1575b816121ba60209383615058565b81010312610634576121cb906151fb565b5f61200c565b3d91506121ad565b816121e391615058565b61061157815f611fcd565b90506020813d602011612220575b8161220960209383615058565b810103126106345761221a906151fb565b5f611f4a565b3d91506121fc565b8161223291615058565b61060357805f611efd565b8161224791615058565b61060357805f611e87565b8161225c91615058565b61060357805f611dd9565b50346106035760206003193601126106035761229169d3c21bcecceda10000006001600435615aeb565b604090828083516122a28582615058565b600c81527f426f756e6420726573756c7400000000000000000000000000000000000000006020820152845161233f8161230d60208201947fb60e72cc0000000000000000000000000000000000000000000000000000000086528960248401526064830190614e90565b876044830152037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282615058565b51906a636f6e736f6c652e6c6f675afa50826001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561134c5783519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561261d57612673575b5050602154601f5483517f095ea7b30000000000000000000000000000000000000000000000000000000081526001600160a01b0360089290921c8216600482015260248101849052916020918391168187816044810103925af180156125fb5761263c575b50826001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561134c5783519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561261d57612627575b506001600160a01b03601f5460081c166001600160a01b0360215416813b1561132f57829160848392875194859384927f18b68b8c0000000000000000000000000000000000000000000000000000000084526004840152886024840152606060448401528160648401525af1801561261d57612608575b506004906001600160a01b03602054168451928380927f530660690000000000000000000000000000000000000000000000000000000082525afa9081156125fb578260606004959361254f9388916125e1575b50015161594b565b60206001600160a01b03601f5460081c168251948580927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa9081156125d8575083906125a4575b6111f9925061594b565b506020823d6020116125d0575b816125be60209383615058565b81010312611228576111f9915161259a565b3d91506125b1565b513d85823e3d90fd5b6125f591503d808a833e6106618183615058565b5f612547565b50505051903d90823e3d90fd5b8161261291615058565b61132f57825f6124f3565b84513d84823e3d90fd5b8161263191615058565b61132f57825f61247b565b6020813d60201161266b575b8161265560209383615058565b8101031261133357612666906150d3565b612418565b3d9150612648565b8161267d91615058565b61132f57825f6123b2565b5034610603578060031936011261060357806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657612c3a575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526a01a784379d99db4200000060248401525af1801561060657612c03575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657612bee575b506001600160a01b03601f5460081c166001600160a01b036021541690803b15610634576040517f18b68b8c0000000000000000000000000000000000000000000000000000000081526001600160a01b0392909216600483015269d3c21bcecceda10000006024830152606060448301525f606483015282908290608490829084905af1801561060657612bd9575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa8015610606578290612ba5575b6128b6915061584b565b620151804201804211612b78578190737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561061157604051907fe5d6bf020000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657612b63575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657612b4e575b506001600160a01b03601f5460081c166001600160a01b036021541690803b15610634576040517f18b68b8c0000000000000000000000000000000000000000000000000000000081526001600160a01b0392909216600483015269d3c21bcecceda10000006024830152606060448301525f606483015282908290608490829084905af1801561060657612b39575b5050600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa8015610606578290612b05575b612a7f915061584b565b600460206001600160a01b03815416604051928380927f371bed680000000000000000000000000000000000000000000000000000000082525afa8015610606578290612ad1575b6111f991506156d4565b506020813d602011612afd575b81612aeb60209383615058565b81010312611228576111f99051612ac7565b3d9150612ade565b506020813d602011612b31575b81612b1f60209383615058565b8101031261122857612a7f9051612a75565b3d9150612b12565b81612b4391615058565b61060357805f612a27565b81612b5891615058565b61060357805f612997565b81612b6d91615058565b61060357805f612934565b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b506020813d602011612bd1575b81612bbf60209383615058565b81010312611228576128b690516128ac565b3d9150612bb2565b81612be391615058565b61060357805f61285e565b81612bf891615058565b61060357805f6127ce565b6020813d602011612c32575b81612c1c60209383615058565b8101031261134c57612c2d906150d3565b61276a565b3d9150612c0f565b81612c4491615058565b61060357805f6126fc565b50346106035780600319360112610603576020612c6a6154c4565b6040519015158152f35b5034610603578060031936011261060357601954612c91816150e0565b91612c9f6040519384615058565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b838310612ce157604051806118078782614f28565b600160208192612cf0856150f8565b815201920192019190612ccc565b50346106035780600319360112610603576001600160a01b03601f5460081c166040517fc9f5b63e000000000000000000000000000000000000000000000000000000008152602081600481855afa80156106295783906130f3575b612d7191506001600160a01b0360205416906159c1565b6040517f56eff267000000000000000000000000000000000000000000000000000000008152602081600481855afa80156106295783906130b8575b612dc491506001600160a01b0360255416906159c1565b6040517f66d003ac000000000000000000000000000000000000000000000000000000008152602081600481855afa90811561062957839161307e575b50612e19602654916001600160a01b038316906159c1565b604051907f3cbb6979000000000000000000000000000000000000000000000000000000008252602082600481865afa918215610c0c578492613039575b509063ffffffff80612e6e9360a01c16911661594b565b6040517f36b089d8000000000000000000000000000000000000000000000000000000008152602081600481855afa8015610629578390613005575b612eb4915061584b565b816040517f67eeba0c000000000000000000000000000000000000000000000000000000008152602081600481865afa908115610606578291612fd0575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561134c57604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526a0422ca8b0a00a42500000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561060657612fbb575b50506020600491604051928380927fead93c8f0000000000000000000000000000000000000000000000000000000082525afa8015610606578290611bfa576111f99150615a42565b81612fc591615058565b61134c57815f612f72565b9150506020813d602011612ffd575b81612fec60209383615058565b81010312611228578290515f612ef2565b3d9150612fdf565b506020813d602011613031575b8161301f60209383615058565b8101031261122857612eb49051612eaa565b3d9150613012565b91506020823d602011613076575b8161305460209383615058565b810103126113335763ffffffff8061306e612e6e9461520f565b935050612e57565b3d9150613047565b90506020813d6020116130b0575b8161309960209383615058565b8101031261132f576130aa906151fb565b5f612e01565b3d915061308c565b506020813d6020116130eb575b816130d260209383615058565b8101031261132f576130e6612dc4916151fb565b612dad565b3d91506130c5565b506020813d602011613126575b8161310d60209383615058565b8101031261132f57613121612d71916151fb565b612d5a565b3d9150613100565b5034610603578060031936011261060357601c5461314b816150e0565b916131596040519384615058565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b83831061319b57604051806118078782614fa5565b600260206001926040516131ae8161503c565b6001600160a01b0386541681526131c6858701615220565b83820152815201920192019190613186565b503461060357806003193601126106035760206001600160a01b03601f5460081c16604051908152f35b5034610603578060031936011261060357601d5461321f816150e0565b9161322d6040519384615058565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b83831061326f57604051806118078782614fa5565b600260206001926040516132828161503c565b6001600160a01b03865416815261329a858701615220565b8382015281520192019201919061325a565b5034610603578060031936011261060357601a546132c9816150e0565b916132d76040519384615058565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b83831061331957604051806118078782614f28565b600160208192613328856150f8565b815201920192019190613304565b503461060357806003193601126106035760206001600160a01b03815416604051908152f35b50346106035760206003193601126106035761338062989680615208600435615aeb565b604090828083516133918582615058565b600c81527f426f756e6420726573756c740000000000000000000000000000000000000000602082015284516133fc8161230d60208201947fb60e72cc0000000000000000000000000000000000000000000000000000000086528960248401526064830190614e90565b51906a636f6e736f6c652e6c6f675afa50826001600160a01b0360225416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561134c5783519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561261d576135bf575b506001600160a01b03601f5460081c166001600160a01b03602554169263ffffffff6001600160a01b0360265416911693823b156113335785517f85931b740000000000000000000000000000000000000000000000000000000081526001600160a01b0391821660048201529116602482015263ffffffff841660448201529082908290606490829084905af1801561261d576135aa575b5050600460206001600160a01b03601f5460081c168451928380927f3cbb69790000000000000000000000000000000000000000000000000000000082525afa9283156125d857508392613567575b5063ffffffff6111f9921661594b565b91506020823d6020116135a2575b8161358260209383615058565b8101031261132f5763ffffffff61359b6111f99361520f565b9250613557565b3d9150613575565b816135b491615058565b61132f57825f613508565b816135c991615058565b61132f57825f61346f565b5034610603578060031936011261060357601b546135f1816150e0565b6135fe6040519182615058565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b8383106136d657868587604051928392602084019060208552518091526040840160408260051b8601019392905b82821061366b57505050500390f35b919360206136c6827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc06001959799849503018652885190836136b68351604084526040840190614e90565b9201519084818403910152614ed3565b960192019201859493919261365c565b600260206001926040516136e98161503c565b6136f2866150f8565b81526136ff858701615220565b8382015281520192019201919061362e565b503461060357806003193601126106035760206001600160a01b0360265416604051908152f35b503461060357806003193601126106035760206001600160a01b0360255416604051908152f35b5034610603578060031936011261060357806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657613bf2575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b3000000000000000000000000000000000000000000000000000000008452600484015269152d02c7e14af680000060248401525af1801561060657613bbb575b50602154601f54602080546040517fdd62ed3e0000000000000000000000000000000000000000000000000000000081526001600160a01b0360089490941c84166004820152908316602482015292909183916044918391165afa8015610606578290613b87575b6138b291506157d5565b806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657613b72575b506001600160a01b03601f5460081c166001600160a01b036021541690803b15610634576040517f18b68b8c0000000000000000000000000000000000000000000000000000000081526001600160a01b0392909216600483015269152d02c7e14af68000006024830152606060448301525f606483015282908290608490829084905af1801561060657613b5d575b50506001600160a01b0360215416816001600160a01b0360205416604051907f70a082310000000000000000000000000000000000000000000000000000000082526004820152602081602481865afa908115610606578291613b28575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561134c57604051907f98296c54000000000000000000000000000000000000000000000000000000008252600482015269152d02c7e14af680000060248201528181604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561060657613b13575b505060206001600160a01b03601f5460081c166024604051809481937f70a0823100000000000000000000000000000000000000000000000000000000835260048301525afa8015610606578290613adf575b6111f991506157d5565b506020813d602011613b0b575b81613af960209383615058565b81010312611228576111f99051613ad5565b3d9150613aec565b81613b1d91615058565b61134c57815f613a82565b9150506020813d602011613b55575b81613b4460209383615058565b81010312611228578290515f613a03565b3d9150613b37565b81613b6791615058565b61060357805f6139a5565b81613b7c91615058565b61060357805f613915565b506020813d602011613bb3575b81613ba160209383615058565b81010312611228576138b290516138a8565b3d9150613b94565b6020813d602011613bea575b81613bd460209383615058565b8101031261134c57613be5906150d3565b613840565b3d9150613bc7565b81613bfc91615058565b61060357805f6137d3565b503461060357806003193601126106035760206001600160a01b0360245416604051908152f35b503461060357806003193601126106035760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110613c8d57611807856117fb81870382615058565b82546001600160a01b0316845260209093019260019283019201613c76565b503461060357806003193601126106035760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b818110613d0b57611807856117fb81870382615058565b82546001600160a01b0316845260209093019260019283019201613cf4565b5034610603578060031936011261060357602063ffffffff60265460a01c16604051908152f35b5034610603578060031936011261060357806001600160a01b0360205416803b15610611578180916024604051809481937f6813d787000000000000000000000000000000000000000000000000000000008352600160048401525af1801561060657614095575b506001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657614080575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b3000000000000000000000000000000000000000000000000000000008452600484015269152d02c7e14af680000060248401525af1801561060657614049575b50806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611576040519063ca669fa760e01b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657614034575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561060357806040517ff28dceb300000000000000000000000000000000000000000000000000000000815260206004820152601660248201527f4f7074696d69736d20627269646765206661696c6564000000000000000000006044820152818160648183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156106065761401f575b506001600160a01b03601f5460081c166001600160a01b036021541690803b15610634576040517f18b68b8c0000000000000000000000000000000000000000000000000000000081526001600160a01b0392909216600483015269152d02c7e14af68000006024830152606060448301525f606483015282908290608490829084905af18015610606576105f25750f35b8161402991615058565b61060357805f613f8d565b8161403e91615058565b61060357805f613eed565b6020813d602011614078575b8161406260209383615058565b8101031261134c57614073906150d3565b613e89565b3d9150614055565b8161408a91615058565b61060357805f613e1c565b8161409f91615058565b61060357805f613db9565b5034610603578060031936011261060357600460606001600160a01b03601f5460081c16604051928380927f32d4f5040000000000000000000000000000000000000000000000000000000082525afa801561060657828392849261414d575b5063ffffffff6111f99361412b82936001600160a01b0360255416906159c1565b614142602654916001600160a01b038316906159c1565b60a01c16911661594b565b925050506060813d6060116141ad575b8161416a60609383615058565b8101031261134c578063ffffffff806141856111f9946151fb565b61412b6141a06040614199602088016151fb565b960161520f565b94955090925061410a9050565b3d915061415d565b5034610603578060031936011261060357601e546141d2816150e0565b6141df6040519182615058565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b8383106143205786858760405192839260208401906020855251809152604084019160408260051b8601019392815b83831061424b5786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b8281106142d75750505050506020806001929701930193019092869594929361423e565b9091929394602080614313837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa087600196030189528951614e90565b97019501939291016142b3565b60405161432c8161503c565b6001600160a01b038354168152600183018054614348816150e0565b916143566040519384615058565b8183528a526020808b20908b9084015b83821061438c57505050506001928260209283600295015281520192019201919061420f565b60016020819261439b866150f8565b815201930191019091614366565b503461060357806003193601126106035760206040516a0422ca8b0a00a4250000008152f35b503461060357806003193601126106035760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b81811061442e57611807856117fb81870382615058565b82546001600160a01b0316845260209093019260019283019201614417565b5034610603578060031936011261060357602060405169d3c21bcecceda10000008152f35b5034610603578060031936011261060357806001600160a01b0360245416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561061157604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610606576147b1575b50600460206001600160a01b03601f5460081c16604051928380927f118c38c70000000000000000000000000000000000000000000000000000000082525afa90811561060657829161477c575b506145d76001600160a01b03602454166145ab6040519384927fe2517d3f00000000000000000000000000000000000000000000000000000000602085015260248401602090939291936001600160a01b0360408201951681520152565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282615058565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610611578161463291604051809381927ff28dceb3000000000000000000000000000000000000000000000000000000008352602060048401526024830190614e90565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657614767575b506001600160a01b03601f5460081c166001600160a01b0360255416602654823b15610684576040517f85931b740000000000000000000000000000000000000000000000000000000081526001600160a01b039283166004820152918116602483015260a01c63ffffffff16604482015290829082908183816064810103925af1801561060657614752575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561060357806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af18015610606576105f25750f35b8161475c91615058565b61060357805f6146e4565b8161477191615058565b61060357805f614657565b9150506020813d6020116147a9575b8161479860209383615058565b81010312611228578190515f61454d565b3d915061478b565b816147bb91615058565b61060357805f6144ff565b5034610603578060031936011261060357806001600160a01b0360235416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561061157604051907f06447d560000000000000000000000000000000000000000000000000000000082526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657614bbb575b5060206001600160a01b036021541660446001600160a01b03601f5460081c1660405194859384927f095ea7b300000000000000000000000000000000000000000000000000000000845260048401526969e10de76676d080000060248401525af1801561060657614b84575b50806001600160a01b03601f5460081c166001600160a01b0360215416813b156106345782916084839260405194859384927f18b68b8c0000000000000000000000000000000000000000000000000000000084526004840152692a5a058fc295ed0000006024840152606060448401528160648401525af1801561060657614b6f575b506001600160a01b03601f5460081c166001600160a01b0360215416813b156106345782916084839260405194859384927f18b68b8c0000000000000000000000000000000000000000000000000000000084526004840152693f870857a3e0e38000006024840152606060448401528160648401525af1801561060657614b5a575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561060357806040517f90c5013b000000000000000000000000000000000000000000000000000000008152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561060657614b45575b5050600460206001600160a01b03815416604051928380927f371bed680000000000000000000000000000000000000000000000000000000082525afa8015610606578290614b11575b614a8791506156d4565b600460206001600160a01b03601f5460081c16604051928380927f1033b4cc0000000000000000000000000000000000000000000000000000000082525afa8015610606578290614add575b6111f99150615755565b506020813d602011614b09575b81614af760209383615058565b81010312611228576111f99051614ad3565b3d9150614aea565b506020813d602011614b3d575b81614b2b60209383615058565b8101031261122857614a879051614a7d565b3d9150614b1e565b81614b4f91615058565b61060357805f614a33565b81614b6491615058565b61060357805f6149c7565b81614b7991615058565b61060357805f614944565b6020813d602011614bb3575b81614b9d60209383615058565b8101031261134c57614bae906150d3565b6148c0565b3d9150614b90565b81614bc591615058565b61060357805f614853565b905034611228575f60031936011261122857610c2c80820182811067ffffffffffffffff821117614e21578291615caf833903905ff08015614e16576001600160a01b03167fffffffffffffffffffffffff00000000000000000000000000000000000000006021541617602155604051610ab280820182811067ffffffffffffffff821117614e215782916168db833903905ff08015614e16576001600160a01b0316807fffffffffffffffffffffffff000000000000000000000000000000000000000060205416176020556001600160a01b03602254166001600160a01b0360235416906001600160a01b0360255416926026549360405194611b54918287019387851067ffffffffffffffff861117614e215761010096889663ffffffff9561738d893986526020860152604085015269d3c21bcecceda100000060608501526a0422ca8b0a00a425000000608085015260a08401526001600160a01b03811660c084015260a01c1660e08201520301905ff08015614e16577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f556001600160a01b03602154166001600160a01b0360235416813b15611228575f916044839260405194859384927f40c10f1900000000000000000000000000000000000000000000000000000000845260048401526a084595161401484a00000060248401525af18015614e1657614e08575080f35b614e1491505f90615058565b005b6040513d5f823e3d90fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60206040818301928281528451809452019201905f5b818110614e715750505090565b82516001600160a01b0316845260209384019390920191600101614e64565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b90602080835192838152019201905f5b818110614ef05750505090565b82517fffffffff0000000000000000000000000000000000000000000000000000000016845260209384019390920191600101614ee3565b602081016020825282518091526040820191602060408360051b8301019401925f915b838310614f5a57505050505090565b9091929394602080614f96837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187528951614e90565b97019301930191939290614f4b565b602081016020825282518091526040820191602060408360051b8301019401925f915b838310614fd757505050505090565b909192939460208061502d837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b03815116845201519181858201520190614ed3565b97019301930191939290614fc8565b6040810190811067ffffffffffffffff821117614e2157604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117614e2157604052565b919082018092116150a657565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5190811515820361122857565b67ffffffffffffffff8111614e215760051b60200190565b90604051915f8154908160011c92600183169283156151f1575b6020851084146151c45784875286939081156151845750600114615140575b5061513e92500383615058565b565b90505f9291925260205f20905f915b81831061516857505090602061513e928201015f615131565b602091935080600191548385890101520191019091849261514f565b6020935061513e9592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f615131565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b93607f1693615112565b51906001600160a01b038216820361122857565b519063ffffffff8216820361122857565b90604051918281549182825260208201905f5260205f20925f905b8060078301106154375761513e945491818110615401575b8181106153cb575b818110615395575b81811061535f575b818110615329575b8181106152f3575b8181106152be575b10615291575b500383615058565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f615289565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b168152019301615283565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b16815201930161527b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b168152019301615273565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b16815201930161526b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b168152019301615263565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b16815201930161525b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b168152019301615253565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e082015201940192018592939161523b565b60085460ff1680156154d35790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115614e16575f9161556b575b50151590565b90506020813d602011615595575b8161558660209383615058565b8101031261122857515f615565565b3d9150615579565b92919267ffffffffffffffff8211614e2157604051916155e5601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200184615058565b829481845281830111611228578281602093845f96015e010152565b6020818303126112285780519067ffffffffffffffff821161122857019060c082820312611228576040519160c0830183811067ffffffffffffffff821117614e2157604052615650816151fb565b835261565e602082016151fb565b602084015261566f604082016151fb565b60408401526060810151606084015261568a6080820161520f565b608084015260a08101519067ffffffffffffffff821161122857019080601f830112156112285781516156bf9260200161559d565b60a082015290565b919082039182116150a657565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152600260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b5f61513e91615058565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201526969e10de76676d080000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051907f98296c54000000000000000000000000000000000000000000000000000000008252600482015269d3c21bcecceda100000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152699ed194db19b238c0000060248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051917f98296c54000000000000000000000000000000000000000000000000000000008352600483015260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611228576040517f515361f60000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152911660248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561122857604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa8015614e165761574b5750565b8115615abe570690565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f90838311615c2a5782811091821580615c20575b615c1857615b0e84866156c7565b92600184018094116150a657600383111580615c0f575b615c005760031983101580615bf6575b615be25785831115615b9957505090615b5184615b56936156c7565b615ab4565b908115615b9457615b679250615099565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116150a65790565b505090565b959492919095615baa575b50505050565b83949550615b5190615bbc93946156c7565b908115615b9457615bcd92506156c7565b600181018091116150a657905f808080615ba4565b50509050615bf392915019906156c7565b90565b5082198411615b35565b5050919050615bf39250615099565b50828411615b25565b509250505090565b5084821115615b00565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603e60248201527f5374645574696c7320626f756e642875696e743235362c75696e743235362c7560448201527f696e74323536293a204d6178206973206c657373207468616e206d696e2e00006064820152fdfe60806040523461031357604080519081016001600160401b03811182821017610226576040908152600982526845524332304d6f636b60b81b602083015280519081016001600160401b038111828210176102265760405260048152634532304d60e01b602082015281516001600160401b03811161022657600354600181811c91168015610309575b602082101461020857601f81116102a6575b50602092601f821160011461024557928192935f9261023a575b50508160011b915f199060031b1c1916176003555b80516001600160401b03811161022657600454600181811c9116801561021c575b602082101461020857601f81116101a5575b50602091601f8211600114610145579181925f9261013a575b50508160011b915f199060031b1c1916176004555b60405161091490816103188239f35b015190505f80610116565b601f1982169260045f52805f20915f5b85811061018d57508360019510610175575b505050811b0160045561012b565b01515f1960f88460031b161c191690555f8080610167565b91926020600181928685015181550194019201610155565b60045f527f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b601f830160051c810191602084106101fe575b601f0160051c01905b8181106101f357506100fd565b5f81556001016101e6565b90915081906101dd565b634e487b7160e01b5f52602260045260245ffd5b90607f16906100eb565b634e487b7160e01b5f52604160045260245ffd5b015190505f806100b5565b601f1982169360035f52805f20915f5b86811061028e5750836001959610610276575b505050811b016003556100ca565b01515f1960f88460031b161c191690555f8080610268565b91926020600181928685015181550194019201610255565b60035f527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b601f830160051c810191602084106102ff575b601f0160051c01905b8181106102f4575061009b565b5f81556001016102e7565b90915081906102de565b90607f1690610089565b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c90816306fdde031461070357508063095ea7b31461067457806318160ddd1461065757806323b872dd146104e1578063313ce567146104c657806340c10f19146103e557806370a08231146103a157806395d89b41146102265780639dc29fac14610138578063a9059cbb146101075763dd62ed3e14610095575f80fd5b34610103576040600319360112610103576100ae610804565b73ffffffffffffffffffffffffffffffffffffffff6100cb610827565b91165f52600160205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060405f2054604051908152f35b5f80fd5b346101035760406003193601126101035761012d610123610804565b602435903361084a565b602060405160018152f35b3461010357604060031936011261010357610151610804565b73ffffffffffffffffffffffffffffffffffffffff602435911680156101fa57805f525f60205260405f20548281106101c8576020835f947fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef938587528684520360408620558060025403600255604051908152a3005b907fe450d38c000000000000000000000000000000000000000000000000000000005f5260045260245260445260645ffd5b7f96c6fd1e000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b34610103575f600319360112610103576040515f600454908160011c60018316928315610397575b60208210841461036a57818552849390811561032857506001146102cc575b5003601f01601f191681019067ffffffffffffffff82118183101761029f5761029b829182604052826107da565b0390f35b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60045f90815291507f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b5b81831061030c5750508101602001601f1961026d565b60209193508060019154838588010152019101909183926102f6565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208581019190915291151560051b84019091019150601f19905061026d565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b90607f169061024e565b346101035760206003193601126101035773ffffffffffffffffffffffffffffffffffffffff6103cf610804565b165f525f602052602060405f2054604051908152f35b34610103576040600319360112610103576103fe610804565b73ffffffffffffffffffffffffffffffffffffffff16602435811561049a576002549080820180921161046d5760207fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef915f9360025584845283825260408420818154019055604051908152a3005b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7fec442f05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b34610103575f60031936011261010357602060405160128152f35b34610103576060600319360112610103576104fa610804565b610502610827565b6044359073ffffffffffffffffffffffffffffffffffffffff831692835f52600160205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260405f20547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff811061057e575b5061012d935061084a565b8381106106235784156105f75733156105cb5761012d945f52600160205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f526020528360405f209103905584610573565b7f94280d62000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7fe602df05000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b83907ffb8f41b2000000000000000000000000000000000000000000000000000000005f523360045260245260445260645ffd5b34610103575f600319360112610103576020600254604051908152f35b346101035760406003193601126101035761068d610804565b6024359033156105f75773ffffffffffffffffffffffffffffffffffffffff169081156105cb57335f52600160205260405f20825f526020528060405f20556040519081527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560203392a3602060405160018152f35b34610103575f600319360112610103575f600354908160011c600183169283156107d0575b60208210841461036a5781855284939081156103285750600114610774575003601f01601f191681019067ffffffffffffffff82118183101761029f5761029b829182604052826107da565b60035f90815291507fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b5b8183106107b45750508101602001601f1961026d565b602091935080600191548385880101520191019091839261079e565b90607f1690610728565b601f19601f602060409481855280519182918282880152018686015e5f8582860101520116010190565b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361010357565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361010357565b73ffffffffffffffffffffffffffffffffffffffff169081156101fa5773ffffffffffffffffffffffffffffffffffffffff1691821561049a57815f525f60205260405f20548181106108e257817fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef92602092855f525f84520360405f2055845f525f825260405f20818154019055604051908152a3565b827fe450d38c000000000000000000000000000000000000000000000000000000005f5260045260245260445260645ffd60808060405234601557610a98908161001a8239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c908163371bed681461086b5750806353066069146106ad5780636813d7871461065e578063838b25201461014c578063959b825a146100845763d3072d821461005e575f80fd5b34610080575f60031936011261008057602060ff600154166040519015158152f35b5f80fd5b34610080576020600319360112610080576004355f54811015610080576100aa906108c7565b5073ffffffffffffffffffffffffffffffffffffffff81541661014873ffffffffffffffffffffffffffffffffffffffff6001840154169273ffffffffffffffffffffffffffffffffffffffff60028201541690600381015461011a600563ffffffff60048501541693016109da565b926040519687968752602087015260408601526060850152608084015260c060a084015260c0830190610884565b0390f35b346100805760c06003193601126100805760043573ffffffffffffffffffffffffffffffffffffffff8116809103610080576024359073ffffffffffffffffffffffffffffffffffffffff82168092036100805760443573ffffffffffffffffffffffffffffffffffffffff8116809103610080576064356084359063ffffffff82168092036100805760a4359467ffffffffffffffff8611610080573660238701121561008057856004013567ffffffffffffffff81116100805736602482890101116100805760ff60015416610600576040517f23b872dd0000000000000000000000000000000000000000000000000000000081523360048201523060248201528360448201526020816064815f8b5af180156105f5576105ba575b5060205f916040519761027d8961097d565b88528188019384526040880196875260608801948552608088019586528060246040519a6102d2857fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f860116018d610999565b828c5201838b01378801015260a085019586525f54680100000000000000008110156105615780600161030792015f556108c7565b94909461058e5773ffffffffffffffffffffffffffffffffffffffff809281806005995116167fffffffffffffffffffffffff00000000000000000000000000000000000000008854161787555116826001870191167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055511673ffffffffffffffffffffffffffffffffffffffff6002850191167fffffffffffffffffffffffff000000000000000000000000000000000000000082541617905551600383015563ffffffff600483019151167fffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000082541617905501905190815167ffffffffffffffff811161056157610420825461092c565b601f811161051c575b50602092601f821160011461048357928192935f92610478575b50507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8260011b9260031b1c19161790555f80f35b015190508380610443565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0821693835f52805f20915f5b86811061050457508360019596106104cd575b505050811b019055005b01517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60f88460031b161c191690558380806104c3565b919260206001819286850151815501940192016104b0565b825f5260205f20601f830160051c81019160208410610557575b601f0160051c01905b81811061054c5750610429565b5f815560010161053f565b9091508190610536565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6020813d6020116105ed575b816105d360209383610999565b81010312610080575180151581036100805750602061026b565b3d91506105c6565b6040513d5f823e3d90fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601660248201527f4f7074696d69736d20627269646765206661696c6564000000000000000000006044820152fd5b34610080576020600319360112610080576004358015158091036100805760ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00600154169116176001555f80f35b34610080575f60031936011261008057606060a06040516106cd8161097d565b5f81525f60208201525f60408201525f838201525f608082015201525f54801561080d577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116107e05761073973ffffffffffffffffffffffffffffffffffffffff916108c7565b506101486040519161074a8361097d565b83815416835263ffffffff846001830154169160208501928352856002820154166040860190815286600383015491606088019283528161079b6005876004880154169660808c01978852016109da565b9660a08a019788526040519a8b9a60208c52511660208b01525116604089015251166060870152516080860152511660a08401525160c08084015260e0830190610884565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600b60248201527f4e6f206465706f736974730000000000000000000000000000000000000000006044820152fd5b34610080575f600319360112610080576020905f548152f35b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b5f548110156108ff575f8080526006919091027f290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e5630191565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b90600182811c92168015610973575b602083101461094657565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f169161093b565b60c0810190811067ffffffffffffffff82111761056157604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761056157604052565b9060405191825f8254926109ed8461092c565b8084529360018116908115610a585750600114610a14575b50610a1292500383610999565b565b90505f9291925260205f20905f915b818310610a3c575050906020610a12928201015f610a05565b6020919350806001915483858901015201910190918492610a23565b60209350610a129592507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201015f610a055660806040523461032757604051601f611b5438819003918201601f19168301916001600160401b038311848410176102f357808492610100946040528339810103126103275761004e8161032b565b61005a6020830161032b565b6100666040840161032b565b6060840151608085015161007c60a0870161032b565b9360e061008b60c0890161032b565b9701519563ffffffff871687036103275760408051929083016001600160401b038111848210176102f357604052600f83526e4f7074696d69736d2042726964676560881b6020840152600180556001600160a01b038216158015610316575b61030757610105826100ff61010b9461033f565b506103b5565b50610448565b508051906001600160401b0382116102f35760075490600182811c921680156102e9575b60208310146102d55781601f849311610267575b50602090601f8311600114610201575f926101f6575b50508160011b915f199060031b1c1916176007555b600280546003939093556004919091556001600160a81b03199091166001600160a01b0392831617600160a01b179055620151804204600655600880546001600160a01b03191692821692909217909155600980546001600160c01b031916939091169290921760a09190911b63ffffffff60a01b161790556040516115f890816104dc8239f35b015190505f80610159565b60075f9081528281209350601f198516905b81811061024f5750908460019594939210610237575b505050811b0160075561016e565b01515f1960f88460031b161c191690555f8080610229565b92936020600181928786015181550195019301610213565b60075f529091507fa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c688601f840160051c810191602085106102cb575b90601f859493920160051c01905b8181106102bd5750610143565b5f81558493506001016102b0565b90915081906102a2565b634e487b7160e01b5f52602260045260245ffd5b91607f169161012f565b634e487b7160e01b5f52604160045260245ffd5b63d92e233d60e01b5f5260045ffd5b506001600160a01b038116156100eb565b5f80fd5b51906001600160a01b038216820361032757565b6001600160a01b0381165f9081525f516020611b345f395f51905f52602052604090205460ff166103b0576001600160a01b03165f8181525f516020611b345f395f51905f5260205260408120805460ff191660011790553391905f516020611ad45f395f51905f528180a4600190565b505f90565b6001600160a01b0381165f9081525f516020611b145f395f51905f52602052604090205460ff166103b0576001600160a01b03165f8181525f516020611b145f395f51905f5260205260408120805460ff191660011790553391907f751b795d24b92e3d92d1d0d8f2885f4e9c9c269da350af36ae6b49069babf4bf905f516020611ad45f395f51905f529080a4600190565b6001600160a01b0381165f9081525f516020611af45f395f51905f52602052604090205460ff166103b0576001600160a01b03165f8181525f516020611af45f395f51905f5260205260408120805460ff191660011790553391907fc074beb46b251f73c0fec16eba75d6bc0042d09ee17a740ebbee308dbf87f82f905f516020611ad45f395f51905f529080a460019056fe6080806040526004361015610012575f80fd5b5f905f3560e01c90816301ffc9a714611001575080631033b4cc14610fe4578063118c38c714610faa5780631259a5c814610f8d57806318b68b8c14610a1e578063248a9ca3146109eb5780632f2ff15d146109ad57806332d4f5041461095b5780633462fac31461092057806336568abe146108b557806336b089d8146108975780633cbb69791461087057806356eff2671461083c5780635ab1d61c1461079e57806361b0a56e14610661578063632214901461063e57806365d7a3c91461060a57806366d003ac146105d657806367eeba0c146105b85780636bcc8c14146104ea57806385931b74146103a257806391d148541461034b578063a217fddf1461032f578063b20d30a9146102d9578063c9f5b63e146102a5578063d547741f1461025e578063ead93c8f14610238578063ede7cebd146101d65763fb8c4b511461015d575f80fd5b346101d357806003193601126101d357600554600454818111156101ca5781810381811161019d5760609350905b60405192835260208301526040820152f35b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b6060929061018b565b80fd5b50346101d357806003193601126101d35761020c6002546101f5611153565b9060ff60405193849360608552606085019061126c565b9173ffffffffffffffffffffffffffffffffffffffff8116602085015260a01c16151560408301520390f35b50346101d357806003193601126101d357602060ff60025460a01c166040519015158152f35b50346101d35760406003193601126101d3576102a160043561027e6110c2565b9061029c610297825f525f602052600160405f20015490565b611371565b6114a9565b5080f35b50346101d357806003193601126101d357602073ffffffffffffffffffffffffffffffffffffffff60025416604051908152f35b50346101d35760206003193601126101d3577f207c4cbdf55ec315a13f0d5e047732ec5d947da056e706593aa509909941cedf60406004356103196112e9565b600454908060045582519182526020820152a180f35b50346101d357806003193601126101d357602090604051908152f35b50346101d35760406003193601126101d35773ffffffffffffffffffffffffffffffffffffffff604061037c6110c2565b926004358152806020522091165f52602052602060ff60405f2054166040519015158152f35b50346101d35760606003193601126101d3576103bc61109f565b6103c46110c2565b6044359163ffffffff831683036104e6576104e07f802b8c7b24709b6c9c56179dceeb977cc7ac6fa4f15f84c99a8627abfd97cc35936104026112e9565b73ffffffffffffffffffffffffffffffffffffffff83167fffffffffffffffffffffffff0000000000000000000000000000000000000000600854161760085560095477ffffffff00000000000000000000000000000000000000008260a01b16907fffffffffffffffff00000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff8716911617176009556040519384938491604091949373ffffffffffffffffffffffffffffffffffffffff63ffffffff9281606087019816865216602085015216910152565b0390a180f35b8380fd5b50346101d35760206003193601126101d35773ffffffffffffffffffffffffffffffffffffffff61051961109f565b6105216112e9565b1680156105905773ffffffffffffffffffffffffffffffffffffffff600254827fffffffffffffffffffffffff0000000000000000000000000000000000000000821617600255167fb07f8b1b85042d74022c867c836edeb0bcd70e135b0042390d2b1fd1082980698380a380f35b6004827fd92e233d000000000000000000000000000000000000000000000000000000008152fd5b50346101d357806003193601126101d3576020600454604051908152f35b50346101d357806003193601126101d357602073ffffffffffffffffffffffffffffffffffffffff60095416604051908152f35b50346101d357806003193601126101d35761063a610626611153565b60405191829160208352602083019061126c565b0390f35b50346101d35760206003193601126101d3576106586112e9565b60043560035580f35b50346101d35760606003193601126101d35761067b61109f565b60443573ffffffffffffffffffffffffffffffffffffffff81168082036104e657838052836020526040842073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f2054161561076e5715610746576040517fa9059cbb00000000000000000000000000000000000000000000000000000000602082015273ffffffffffffffffffffffffffffffffffffffff9182166024808301919091523560448083019190915281526107439290919061073d6064846110e5565b16611571565b80f35b6004837fd92e233d000000000000000000000000000000000000000000000000000000008152fd5b6044847fe2517d3f0000000000000000000000000000000000000000000000000000000081523360045280602452fd5b50346101d35760206003193601126101d3576004358015158091036108385760207fb3418989d06835b5c215eebb4d54ed6be7bbb66eb4807164740a2e082fa782d5916107e96112e9565b6002547fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff74ff00000000000000000000000000000000000000008360a01b16911617600255604051908152a180f35b5080fd5b50346101d357806003193601126101d357602073ffffffffffffffffffffffffffffffffffffffff60085416604051908152f35b50346101d357806003193601126101d357602063ffffffff60095460a01c16604051908152f35b50346101d357806003193601126101d3576020600354604051908152f35b50346101d35760406003193601126101d3576108cf6110c2565b3373ffffffffffffffffffffffffffffffffffffffff8216036108f8576102a1906004356114a9565b6004827f6697b232000000000000000000000000000000000000000000000000000000008152fd5b50346101d357806003193601126101d35760206040517fc074beb46b251f73c0fec16eba75d6bc0042d09ee17a740ebbee308dbf87f82f8152f35b50346101d357806003193601126101d3576008546009546040805173ffffffffffffffffffffffffffffffffffffffff9384168152928216602084015260a09190911c63ffffffff1690820152606090f35b50346101d35760406003193601126101d3576102a16004356109cd6110c2565b906109e6610297825f525f602052600160405f20015490565b6113d7565b50346101d35760206003193601126101d3576020610a166004355f525f602052600160405f20015490565b604051908152f35b5034610db5576060600319360112610db557610a3861109f565b6024359060443567ffffffffffffffff8111610db55736602382011215610db55780600401359167ffffffffffffffff8311610db55782820190366024830111610db557600260015414610f6557600260015560ff60025460a01c1615610f3d57335f9081527ffe0c7bd5092c7a6e463488fa239f32058c87d43e336e2e584a272132cf8dc926602052604090205460ff1615610f155773ffffffffffffffffffffffffffffffffffffffff16928315610eed578415610ec5576003548511610e5e576201518042046006548111610e86575b50610b18856005546112af565b60045410610e5e57610b736040517f23b872dd00000000000000000000000000000000000000000000000000000000602082015233602482015230604482015286606482015260648152610b6d6084826110e5565b85611571565b15610e335760409082900312610db55760248101359073ffffffffffffffffffffffffffffffffffffffff8216809203610db557604401359063ffffffff8216809203610db557905b73ffffffffffffffffffffffffffffffffffffffff6002541660405160205f8183017f095ea7b3000000000000000000000000000000000000000000000000000000008152610c6284610c368b88602484016020909392919373ffffffffffffffffffffffffffffffffffffffff60408201951681520152565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081018652856110e5565b83519082895af15f513d82610e17575b505015610db9575b505073ffffffffffffffffffffffffffffffffffffffff600254169173ffffffffffffffffffffffffffffffffffffffff6008541692803b15610db55773ffffffffffffffffffffffffffffffffffffffff935f60e49263ffffffff829660405198899788967f838b25200000000000000000000000000000000000000000000000000000000088528c600489015260248801521660448601528a606486015216608484015260c060a48401528160c48401525af18015610daa57610d95575b50610d47826005546112af565b6005557f3dba401dc1abbf017086864b33ba815f53ab3cdbb9366ba6bcd8ec342ddd98e8602073ffffffffffffffffffffffffffffffffffffffff6002541693604051908152a36001805580f35b610da29193505f906110e5565b5f915f610d3a565b6040513d5f823e3d90fd5b5f80fd5b610e0a610e1092604051907f095ea7b300000000000000000000000000000000000000000000000000000000602083015260248201525f604482015260448152610e046064826110e5565b86611571565b84611571565b5f80610c7a565b909150610e2b5750843b15155b5f80610c72565b600114610e24565b505060095463ffffffff73ffffffffffffffffffffffffffffffffffffffff82169160a01c16610bbc565b7f70d168bc000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fda4e39dd56d72c2ee3d132e0146bc39e905e78e3bc64c40190421c7b2bcef2ab60406005548151908482526020820152a15f6005556006555f610b0b565b7f1f2a2005000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fd92e233d000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f5c427cd9000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f7bea20b2000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610db5575f600319360112610db5576020600654604051908152f35b34610db5575f600319360112610db55760206040517f751b795d24b92e3d92d1d0d8f2885f4e9c9c269da350af36ae6b49069babf4bf8152f35b34610db5575f600319360112610db5576020600554604051908152f35b34610db5576020600319360112610db557600435907fffffffff000000000000000000000000000000000000000000000000000000008216809203610db557817f7965db0b0000000000000000000000000000000000000000000000000000000060209314908115611075575b5015158152f35b7f01ffc9a7000000000000000000000000000000000000000000000000000000009150148361106e565b6004359073ffffffffffffffffffffffffffffffffffffffff82168203610db557565b6024359073ffffffffffffffffffffffffffffffffffffffff82168203610db557565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761112657604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051905f6007548060011c9160018216918215611262575b6020841083146112355783865285929081156111f85750600114611199575b611197925003836110e5565b565b5060075f90815290917fa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c6885b8183106111dc5750509060206111979282010161118b565b60209193508060019154838589010152019101909184926111c4565b602092506111979491507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001682840152151560051b82010161118b565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b92607f169261116c565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b919082018092116112bc57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b335f9081527fdfff91bcee88f6ea00b2726f6d062a509c32835793470c55bed3bcc74c0f5d37602052604090205460ff161561132157565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f52336004527f751b795d24b92e3d92d1d0d8f2885f4e9c9c269da350af36ae6b49069babf4bf60245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff33165f5260205260ff60405f205416156113a85750565b7fe2517d3f000000000000000000000000000000000000000000000000000000005f523360045260245260445ffd5b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f205416155f146114a357805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f2060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905573ffffffffffffffffffffffffffffffffffffffff339216907f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d5f80a4600190565b50505f90565b805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260ff60405f2054165f146114a357805f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff83165f5260205260405f207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905573ffffffffffffffffffffffffffffffffffffffff339216907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b5f80a4600190565b905f602091828151910182855af115610daa575f513d6115ef575073ffffffffffffffffffffffffffffffffffffffff81163b155b6115ad5750565b73ffffffffffffffffffffffffffffffffffffffff907f5274afe7000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b600114156115a6562f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0dfe0c7bd5092c7a6e463488fa239f32058c87d43e336e2e584a272132cf8dc926dfff91bcee88f6ea00b2726f6d062a509c32835793470c55bed3bcc74c0f5d37ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\n\x92T\xE4\x14aK\xD0WP\x80c\r\x17\x0B\x02\x14aG\xC6W\x80c\r\x1C\xE0\xDD\x14aDrW\x80c\x11~;B\x14aDMW\x80c\x1E\xD7\x83\x1C\x14aC\xCFW\x80c$\x8E\xC3&\x14aC\xA9W\x80c*\xDE8\x80\x14aA\xB5W\x80c,\xD3\x8F\xBF\x14a@\xAAW\x80c7N\x0C\xE6\x14a=QW\x80c<\xBBiy\x14a=*W\x80c>^<#\x14a<\xACW\x80c?r\x86\xF4\x14a<.W\x80cO\x862\xBA\x14a<\x07W\x80cRt>\xC4\x14a7_W\x80cV\xEF\xF2g\x14a78W\x80cf\xD0\x03\xAC\x14a7\x11W\x80cf\xD9\xA9\xA0\x14a5\xD4W\x80cp(wx\x14a3\\W\x80c{ML\xE3\x14a36W\x80c\x85\"l\x81\x14a2\xACW\x80c\x91j\x17\xC6\x14a2\x02W\x80c\xA3\xD4H[\x14a1\xD8W\x80c\xB0FO\xDC\x14a1.W\x80c\xB4M\xC9\xD6\x14a,\xFEW\x80c\xB5P\x8A\xA9\x14a,tW\x80c\xBAAO\xA6\x14a,OW\x80c\xCC\xC0\xCF\xAC\x14a&\x88W\x80c\xCF\xFB\x04\x8B\x14a\"gW\x80c\xD8\xB2\x96\xDA\x14a\x1DEW\x80c\xDC\xCCW\xF1\x14a\x19\xF4W\x80c\xE1x\xBC[\x14a\x18*W\x80c\xE2\x0C\x9Fq\x14a\x17\x9CW\x80c\xF2\x06{\xD7\x14a\x14\nW\x80c\xF6\x97\xE7\x8A\x14a\x0CsW\x80c\xF8\x10\x06\xB2\x14a\x07\x82W\x80c\xF8Q\xA4@\x14a\x07[W\x80c\xFAv&\xD4\x14a\x078W\x80c\xFC\x0CTj\x14a\x07\x11W\x80c\xFC\x9C\x8D9\x14a\x06\xEAWc\xFEG\xA3\xF4\x14a\x01\xD6W_\x80\xFD[4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80`@Qa\"\"` \x82\x01Rb\x04\x93\xE0`@\x82\x01R`@\x81Ra\x02\n``\x82aPXV[`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x06\xD5W[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri?\x87\x08W\xA3\xE0\xE3\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x06)Wa\x06\x9EW[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x06\x89W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x82;\x15a\x06\x84Wa\x03\xC1\x92\x84\x92\x83`@Q\x80\x96\x81\x95\x82\x94\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri?\x87\x08W\xA3\xE0\xE3\x80\0\0`$\x84\x01R```D\x84\x01R`d\x83\x01\x90aN\x90V[\x03\x92Z\xF1\x80\x15a\x06\x06Wa\x06oW[P`\x04\x81`\x01`\x01`\xA0\x1B\x03` T\x16`@Q\x92\x83\x80\x92\x7FS\x06`i\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x06\x06W\x82\x91a\x06MW[Pa\x04;`\x01`\x01`\xA0\x1B\x03\x82Q\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90aY\xC1V[a\x04^`\x01`\x01`\xA0\x1B\x03` \x83\x01Q\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90aY\xC1V[`\x01`\x01`\xA0\x1B\x03`@\x82\x01Q\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`@Q\x90\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ra\"\"`$\x82\x01R\x82\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x06)W\x83\x91a\x068W[PP``\x81\x01Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ri?\x87\x08W\xA3\xE0\xE3\x80\0\0`$\x82\x01R\x82\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x06)W\x83\x91a\x06\x14W[PP`\x80\x01Qc\xFF\xFF\xFF\xFF\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rb\x04\x93\xE0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x06\x06Wa\x05\xF2WP\xF3[\x81a\x05\xFC\x91aPXV[a\x06\x03W\x80\xF3[\x80\xFD[`@Q=\x84\x82>=\x90\xFD[P\xFD[\x81a\x06\x1E\x91aPXV[a\x06\x11W\x81_a\x05mV[`@Q=\x85\x82>=\x90\xFD[PP\xFD[\x81a\x06B\x91aPXV[a\x06\x11W\x81_a\x04\xE5V[a\x06i\x91P=\x80\x84\x83>a\x06a\x81\x83aPXV[\x81\x01\x90aV\x01V[_a\x04\x1AV[\x81a\x06y\x91aPXV[a\x06\x03W\x80_a\x03\xD0V[PPP\xFD[\x81a\x06\x93\x91aPXV[a\x06\x11W\x81_a\x03BV[` \x81=` \x11a\x06\xCDW[\x81a\x06\xB7` \x93\x83aPXV[\x81\x01\x03\x12a\x064Wa\x06\xC8\x90aP\xD3V[a\x02\xDCV[=\x91Pa\x06\xAAV[\x81a\x06\xDF\x91aPXV[a\x06\x11W\x81_a\x02oV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`\"T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80` `@Qa\x07\xA3\x82\x82aPXV[\x82\x81R`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x84W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0C\x0CW\x84\x91a\x0C^W[P\x82`\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x0C\x0CWa\x0C,W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x04\x82\x01R\x83`$\x82\x01R`\x01`D\x82\x01R`\x01`d\x82\x01R\x83\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0C\x0CW\x84\x91a\x0C\x17W[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03\x83T\x16\x90\x7F=\xBA@\x1D\xC1\xAB\xBF\x01p\x86\x86K3\xBA\x81_S\xAB<\xDB\xB96k\xA6\xBC\xD8\xEC4-\xDD\x98\xE8\x84`@Qii\xE1\r\xE7fv\xD0\x80\0\0\x81R\xA3`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x84W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x0C\x0CW\x84\x91a\x0B\xF7W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x82;\x15a\x0B\xF3Wa\n0\x92\x85\x92\x83`@Q\x80\x96\x81\x95\x82\x94\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x84\x01R```D\x84\x01R`d\x83\x01\x90aN\x90V[\x03\x92Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x0B\xDEW[P`\x04\x90`\x01`\x01`\xA0\x1B\x03\x83T\x16`@Q\x92\x83\x80\x92\x7FS\x06`i\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06)W`\xA0\x91\x84\x91a\x0B\xC4W[Pa\n\xAE`\x01`\x01`\xA0\x1B\x03\x82Q\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90aY\xC1V[a\n\xD0`\x01`\x01`\xA0\x1B\x03\x84\x83\x01Q\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90aY\xC1V[a\x0B\x1B`\x01`\x01`\xA0\x1B\x03`@\x83\x01Q\x16a\n\xF8`&T\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x90aY\xC1V[a\x0B\x05``\x84\x01QaWUV[c\xFF\xFF\xFF\xFF\x80`\x80\x85\x01Q\x16\x91\x85\x1C\x16\x90aYKV[\x01Qa\x0B*`@Q\x92\x83aPXV[\x82\x82Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064Wa\x0B\x8C\x83\x91a\x0B\x9E`@Q\x94\x85\x93\x84\x93\x7F\x97bF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`@`\x04\x86\x01R`D\x85\x01\x90aN\x90V[\x90`\x03\x19\x84\x83\x03\x01`$\x85\x01RaN\x90V[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x06\x06Wa\x05\xF2WP\xF3[a\x0B\xD8\x91P=\x80\x86\x83>a\x06a\x81\x83aPXV[_a\n\x8DV[\x81a\x0B\xE8\x91aPXV[a\x06\x11W\x81_a\nBV[\x84\x80\xFD[\x81a\x0C\x01\x91aPXV[a\x064W\x82_a\t\xB1V[`@Q=\x86\x82>=\x90\xFD[\x81a\x0C!\x91aPXV[a\x064W\x82_a\x08\xFFV[\x82\x81\x81=\x83\x11a\x0CWW[a\x0CA\x81\x83aPXV[\x81\x01\x03\x12a\x06\x84Wa\x0CR\x90aP\xD3V[a\x08wV[P=a\x0C7V[\x81a\x0Ch\x91aPXV[a\x064W\x82_a\x08\x0BV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`@QaUU` \x82\x01Rb\x02\xBF `@\x82\x01R`@\x81Ra\x0C\xA7``\x82aPXV[`\x01`\x01`\xA0\x1B\x03`!T\x16\x82`\x01`\x01`\xA0\x1B\x03`#T\x16\x92`@Q\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R\x80`\x04\x86\x01R` \x85`$\x81\x87Z\xFA\x94\x85\x15a\x06)W\x83\x95a\x13\xD3W[P` `\x01`\x01`\xA0\x1B\x03\x81T\x16`$`@Q\x80\x97\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x93\x84\x15a\x06)W\x83\x94a\x13\x9CW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13/W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x13\x87W[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x9E\xD1\x94\xDB\x19\xB28\xC0\0\0`$\x84\x01RZ\xF1\x80\x15a\x06)Wa\x13PW[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13/W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x137W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x82;\x15a\x133Wa\x0F\0\x92\x84\x92\x83`@Q\x80\x96\x81\x95\x82\x94\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x9E\xD1\x94\xDB\x19\xB28\xC0\0\0`$\x84\x01R```D\x84\x01R`d\x83\x01\x90aN\x90V[\x03\x92Z\xF1\x80\x15a\x06\x06Wa\x13\x1AW[PP`\x01`\x01`\xA0\x1B\x03`!T\x16\x91`\x01`\x01`\xA0\x1B\x03`#T\x16`@Q\x90\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81\x87Z\xFA\x90\x81\x15a\x12\xDDW\x85\x91a\x12\xE8W[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa.k$\xE6M\xC7@\0\0\x82\x01\x91\x82\x11a\x12~W\x90a\x0F\xA2\x91aYKV[`\x01`\x01`\xA0\x1B\x03` T\x16\x90`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82`\x04\x82\x01R` \x81`$\x81\x87Z\xFA\x90\x81\x15a\x12\xDDW\x85\x91a\x12\xABW[Pi\x9E\xD1\x94\xDB\x19\xB28\xC0\0\0\x82\x01\x80\x92\x11a\x12~W`\x04\x92a\x10\x17\x86\x95\x93\x86\x93aYKV[`@Q\x93\x84\x80\x92\x7FS\x06`i\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x06)W\x83\x92a\x12^W[Pa\x10f\x90`\x01`\x01`\xA0\x1B\x03\x83Q\x16aY\xC1V[a\x10\x89`\x01`\x01`\xA0\x1B\x03` \x83\x01Q\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x90aY\xC1V[`\x01`\x01`\xA0\x1B\x03`@\x82\x01Q\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`@Q\x90\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01RaUU`$\x82\x01R\x82\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x06)W\x83\x91a\x12IW[PP`\x80\x81a\x11'``c\xFF\xFF\xFF\xFF\x94\x01QaX\xCBV[\x01Q\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rb\x02\xBF `$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x06\x06Wa\x124W[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90a\x11\xFCW[a\x11\xF9\x91PaX\xCBV[\x80\xF3[P` \x81=` \x11a\x12,W[\x81a\x12\x16` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa\x11\xF9\x90Qa\x11\xEFV[_\x80\xFD[=\x91Pa\x12\tV[\x81a\x12>\x91aPXV[a\x06\x03W\x80_a\x11\xA1V[\x81a\x12S\x91aPXV[a\x06\x11W\x81_a\x11\x10V[a\x10f\x91\x92Pa\x12w\x90=\x80\x86\x83>a\x06a\x81\x83aPXV[\x91\x90a\x10QV[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11a\x12\xD5W[\x81a\x12\xC6` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WQ_a\x0F\xF2V[=\x91Pa\x12\xB9V[`@Q=\x87\x82>=\x90\xFD[\x90P` \x81=` \x11a\x13\x12W[\x81a\x13\x03` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WQ_a\x0FmV[=\x91Pa\x12\xF6V[\x81a\x13$\x91aPXV[a\x13/W\x82_a\x0F\x0FV[\x82\x80\xFD[\x83\x80\xFD[\x81a\x13A\x91aPXV[a\x13LW\x81_a\x0E\x81V[P\x80\xFD[` \x81=` \x11a\x13\x7FW[\x81a\x13i` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wa\x13z\x90aP\xD3V[a\x0E\x1BV[=\x91Pa\x13\\V[\x81a\x13\x91\x91aPXV[a\x13LW\x81_a\r\xAEV[\x92P\x92P` \x82=` \x11a\x13\xCBW[\x81a\x13\xB9` \x93\x83aPXV[\x81\x01\x03\x12a\x12(W\x84\x91Q\x92_a\rTV[=\x91Pa\x13\xACV[\x92P\x93P` \x82=` \x11a\x14\x02W[\x81a\x13\xF0` \x93\x83aPXV[\x81\x01\x03\x12a\x12(W\x84\x91Q\x93_a\r\x05V[=\x91Pa\x13\xE3V[P4a\x06\x03W` `\x03\x196\x01\x12a\x06\x03W`\x045\x81`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x83\x03a\x13LWsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13LW`@Q\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x15\x15`\x04\x82\x01R\x82\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x06)W\x83\x91a\x17\x87W[PPc\xFF\xFF\xFF\xFF`&T`\xA0\x1C\x16`@Q\x91` \x83\x01R`@\x82\x01R`@\x81Ra\x14\xCF``\x82aPXV[`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13/W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x17rW[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x06)Wa\x17;W[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13/W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x06)W\x83\x91a\x17&W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x82;\x15a\x133Wa\x16\x86\x92\x84\x92\x83`@Q\x80\x96\x81\x95\x82\x94\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01R```D\x84\x01R`d\x83\x01\x90aN\x90V[\x03\x92Z\xF1\x80\x15a\x06\x06Wa\x17\x11W[P\x90`\x04\x91`\x01`\x01`\xA0\x1B\x03` T\x16`@Q\x93\x84\x80\x92\x7FS\x06`i\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06)W`@`\x01`\x01`\xA0\x1B\x03\x91a\x11\xF9\x94\x86\x91a\x16\xF7W[P\x01Q\x16aY\xC1V[a\x17\x0B\x91P=\x80\x88\x83>a\x06a\x81\x83aPXV[_a\x16\xEEV[\x81a\x17\x1B\x91aPXV[a\x13LW\x81_a\x16\x95V[\x81a\x170\x91aPXV[a\x13LW\x81_a\x16\x07V[` \x81=` \x11a\x17jW[\x81a\x17T` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wa\x17e\x90aP\xD3V[a\x15\xA1V[=\x91Pa\x17GV[\x81a\x17|\x91aPXV[a\x13LW\x81_a\x154V[\x81a\x17\x91\x91aPXV[a\x13LW\x81_a\x14\xA4V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x18\x0BWa\x18\x07\x85a\x17\xFB\x81\x87\x03\x82aPXV[`@Q\x91\x82\x91\x82aNNV[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x17\xE4V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x04\x81`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xED\xE7\xCE\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x80\x81\x93\x82\x93a\x19\x85W[P`@\x80Q\x91a\x18\x9A\x82\x84aPXV[`\x0F\x83R\x7FOptimism Bridge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x133Wa\x0B\x8C\x84\x91a\x19!\x84Q\x95\x86\x93\x84\x93\x7F\xF3 \xD9c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R\x87`\x04\x86\x01R`D\x85\x01\x90aN\x90V[\x03\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x19|WPa\x19gW[PPa\x19ba\x11\xF9\x92`\x01`\x01`\xA0\x1B\x03` T\x16\x90aY\xC1V[aZBV[\x81a\x19q\x91aPXV[a\x13/W\x82_a\x19GV[Q=\x84\x82>=\x90\xFD[\x93PPPP=\x80\x83\x83>a\x19\x99\x81\x83aPXV[\x81\x01``\x82\x82\x03\x12a\x13/W\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x133W\x82\x01\x90\x80`\x1F\x83\x01\x12\x15a\x133W\x81Qa\x19\xD2\x92` \x01aU\x9DV[\x90\x82a\x19\xEC`@a\x19\xE5` \x85\x01aQ\xFBV[\x93\x01aP\xD3V[\x91\x92_a\x18\x8AV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xA2\x17\xFD\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x06)W\x83\x91a\x1D\x13W[P`\"T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01\x81\x90R\x90` \x81`D\x81\x86Z\xFA\x80\x15a\x0C\x0CW\x84\x90a\x1C\xD8W[a\x1A\xB4\x91PaZBV[`@Q\x7F\x11\x8C8\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x0C\x0CW\x84\x91a\x1C\xA4W[P`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R` \x81\x80`D\x81\x01[\x03\x81\x85Z\xFA\x80\x15a\x06)W\x83\x90a\x1CiW[a\x1BU\x91PaZBV[`@Q\x7F4b\xFA\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x06)W\x83\x91a\x1C5W[P`#T`@Q\x7F\x91\xD1HT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90` \x90\x82\x90\x81\x80`D\x81\x01[\x03\x91Z\xFA\x80\x15a\x06\x06W\x82\x90a\x1B\xFAW[a\x11\xF9\x91PaZBV[P` \x81=` \x11a\x1C-W[\x81a\x1C\x14` \x93\x83aPXV[\x81\x01\x03\x12a\x13LWa\x1C(a\x11\xF9\x91aP\xD3V[a\x1B\xF0V[=\x91Pa\x1C\x07V[\x90P` \x81=` \x11a\x1CaW[\x81a\x1CP` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WQa\x1B\xDFa\x1B\x92V[=\x91Pa\x1CCV[P` \x81=` \x11a\x1C\x9CW[\x81a\x1C\x83` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wa\x1C\x97a\x1BU\x91aP\xD3V[a\x1BKV[=\x91Pa\x1CvV[\x90P` \x81=` \x11a\x1C\xD0W[\x81a\x1C\xBF` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WQa\x1B9a\x1A\xF1V[=\x91Pa\x1C\xB2V[P` \x81=` \x11a\x1D\x0BW[\x81a\x1C\xF2` \x93\x83aPXV[\x81\x01\x03\x12a\x133Wa\x1D\x06a\x1A\xB4\x91aP\xD3V[a\x1A\xAAV[=\x91Pa\x1C\xE5V[\x90P` \x81=` \x11a\x1D=W[\x81a\x1D.` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WQ_a\x1AQV[=\x91Pa\x1D!V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03Wsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x03W\x80`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81`\x04\x82\x01R\x81`$\x82\x01R\x81`D\x82\x01R`\x01`d\x82\x01R\x81\x81`\x84\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa\"RW[PP\x7F\x80+\x8C{$p\x9Bl\x9CV\x17\x9D\xCE\xEB\x97|\xC7\xACo\xA4\xF1_\x84\xC9\x9A\x86'\xAB\xFD\x97\xCC5`@Q\x80a\x1E!\x81\x90b\x03\xD0\x90`@``\x84\x01\x93a33\x81RaDD` \x82\x01R\x01RV[\x03\x90\xA1\x80`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa\"=W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x06\x11W\x81`@Q\x80\x92\x7F\x85\x93\x1Bt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x83\x81a\x1E\xEE`\x04\x82\x01\x90b\x03\xD0\x90`@``\x84\x01\x93a33\x81RaDD` \x82\x01R\x01RV[\x03\x92Z\xF1\x80\x15a\x06\x06Wa\"(W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7FV\xEF\xF2g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x06)W\x83\x91a!\xEEW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`\x01`\x01`\xA0\x1B\x03`@Q\x91\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01Ra33`$\x82\x01R\x82\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x06)W\x83\x91a!\xD9W[PP`@Q\x7Ff\xD0\x03\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x06)W\x83\x91a!\x9FW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x064W`\x01`\x01`\xA0\x1B\x03`@Q\x91\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01RaDD`$\x82\x01R\x82\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a\x06)W\x83\x91a!\x8AW[PP` `\x04\x91`@Q\x92\x83\x80\x92\x7F<\xBBiy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x06\x06W\x82\x91a!PW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11Wc\xFF\xFF\xFF\xFF`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16`\x04\x82\x01Rb\x03\xD0\x90`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x06\x06Wa\x05\xF2WP\xF3[\x90P` \x81=` \x11a!\x82W[\x81a!k` \x93\x83aPXV[\x81\x01\x03\x12a\x06\x11Wa!|\x90aR\x0FV[_a \xD0V[=\x91Pa!^V[\x81a!\x94\x91aPXV[a\x06\x11W\x81_a \x8FV[\x90P` \x81=` \x11a!\xD1W[\x81a!\xBA` \x93\x83aPXV[\x81\x01\x03\x12a\x064Wa!\xCB\x90aQ\xFBV[_a \x0CV[=\x91Pa!\xADV[\x81a!\xE3\x91aPXV[a\x06\x11W\x81_a\x1F\xCDV[\x90P` \x81=` \x11a\" W[\x81a\"\t` \x93\x83aPXV[\x81\x01\x03\x12a\x064Wa\"\x1A\x90aQ\xFBV[_a\x1FJV[=\x91Pa!\xFCV[\x81a\"2\x91aPXV[a\x06\x03W\x80_a\x1E\xFDV[\x81a\"G\x91aPXV[a\x06\x03W\x80_a\x1E\x87V[\x81a\"\\\x91aPXV[a\x06\x03W\x80_a\x1D\xD9V[P4a\x06\x03W` `\x03\x196\x01\x12a\x06\x03Wa\"\x91i\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`\x01`\x045aZ\xEBV[`@\x90\x82\x80\x83Qa\"\xA2\x85\x82aPXV[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x84Qa#?\x81a#\r` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x89`$\x84\x01R`d\x83\x01\x90aN\x90V[\x87`D\x83\x01R\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82aPXV[Q\x90jconsole.logZ\xFAP\x82`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13LW\x83Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a&\x1DWa&sW[PP`!T`\x1FT\x83Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03`\x08\x92\x90\x92\x1C\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91` \x91\x83\x91\x16\x81\x87\x81`D\x81\x01\x03\x92Z\xF1\x80\x15a%\xFBWa&<W[P\x82`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13LW\x83Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a&\x1DWa&'W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x13/W\x82\x91`\x84\x83\x92\x87Q\x94\x85\x93\x84\x92\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R\x88`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a&\x1DWa&\x08W[P`\x04\x90`\x01`\x01`\xA0\x1B\x03` T\x16\x84Q\x92\x83\x80\x92\x7FS\x06`i\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a%\xFBW\x82```\x04\x95\x93a%O\x93\x88\x91a%\xE1W[P\x01QaYKV[` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x82Q\x94\x85\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a%\xD8WP\x83\x90a%\xA4W[a\x11\xF9\x92PaYKV[P` \x82=` \x11a%\xD0W[\x81a%\xBE` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa\x11\xF9\x91Qa%\x9AV[=\x91Pa%\xB1V[Q=\x85\x82>=\x90\xFD[a%\xF5\x91P=\x80\x8A\x83>a\x06a\x81\x83aPXV[_a%GV[PPPQ\x90=\x90\x82>=\x90\xFD[\x81a&\x12\x91aPXV[a\x13/W\x82_a$\xF3V[\x84Q=\x84\x82>=\x90\xFD[\x81a&1\x91aPXV[a\x13/W\x82_a${V[` \x81=` \x11a&kW[\x81a&U` \x93\x83aPXV[\x81\x01\x03\x12a\x133Wa&f\x90aP\xD3V[a$\x18V[=\x91Pa&HV[\x81a&}\x91aPXV[a\x13/W\x82_a#\xB2V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa,:W[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rj\x01\xA7\x847\x9D\x99\xDBB\0\0\0`$\x84\x01RZ\xF1\x80\x15a\x06\x06Wa,\x03W[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa+\xEEW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x80;\x15a\x064W`@Q\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x06\x06Wa+\xD9W[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90a+\xA5W[a(\xB6\x91PaXKV[b\x01Q\x80B\x01\x80B\x11a+xW\x81\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90\x7F\xE5\xD6\xBF\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa+cW[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa+NW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x80;\x15a\x064W`@Q\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x06\x06Wa+9W[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90a+\x05W[a*\x7F\x91PaXKV[`\x04` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x92\x83\x80\x92\x7F7\x1B\xEDh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90a*\xD1W[a\x11\xF9\x91PaV\xD4V[P` \x81=` \x11a*\xFDW[\x81a*\xEB` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa\x11\xF9\x90Qa*\xC7V[=\x91Pa*\xDEV[P` \x81=` \x11a+1W[\x81a+\x1F` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa*\x7F\x90Qa*uV[=\x91Pa+\x12V[\x81a+C\x91aPXV[a\x06\x03W\x80_a*'V[\x81a+X\x91aPXV[a\x06\x03W\x80_a)\x97V[\x81a+m\x91aPXV[a\x06\x03W\x80_a)4V[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[P` \x81=` \x11a+\xD1W[\x81a+\xBF` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa(\xB6\x90Qa(\xACV[=\x91Pa+\xB2V[\x81a+\xE3\x91aPXV[a\x06\x03W\x80_a(^V[\x81a+\xF8\x91aPXV[a\x06\x03W\x80_a'\xCEV[` \x81=` \x11a,2W[\x81a,\x1C` \x93\x83aPXV[\x81\x01\x03\x12a\x13LWa,-\x90aP\xD3V[a'jV[=\x91Pa,\x0FV[\x81a,D\x91aPXV[a\x06\x03W\x80_a&\xFCV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` a,jaT\xC4V[`@Q\x90\x15\x15\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x19Ta,\x91\x81aP\xE0V[\x91a,\x9F`@Q\x93\x84aPXV[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a,\xE1W`@Q\x80a\x18\x07\x87\x82aO(V[`\x01` \x81\x92a,\xF0\x85aP\xF8V[\x81R\x01\x92\x01\x92\x01\x91\x90a,\xCCV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x7F\xC9\xF5\xB6>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x06)W\x83\x90a0\xF3W[a-q\x91P`\x01`\x01`\xA0\x1B\x03` T\x16\x90aY\xC1V[`@Q\x7FV\xEF\xF2g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x06)W\x83\x90a0\xB8W[a-\xC4\x91P`\x01`\x01`\xA0\x1B\x03`%T\x16\x90aY\xC1V[`@Q\x7Ff\xD0\x03\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x06)W\x83\x91a0~W[Pa.\x19`&T\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x90aY\xC1V[`@Q\x90\x7F<\xBBiy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x86Z\xFA\x91\x82\x15a\x0C\x0CW\x84\x92a09W[P\x90c\xFF\xFF\xFF\xFF\x80a.n\x93`\xA0\x1C\x16\x91\x16aYKV[`@Q\x7F6\xB0\x89\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x06)W\x83\x90a0\x05W[a.\xB4\x91PaXKV[\x81`@Q\x7Fg\xEE\xBA\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x90\x81\x15a\x06\x06W\x82\x91a/\xD0W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13LW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x06\x06Wa/\xBBW[PP` `\x04\x91`@Q\x92\x83\x80\x92\x7F\xEA\xD9<\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90a\x1B\xFAWa\x11\xF9\x91PaZBV[\x81a/\xC5\x91aPXV[a\x13LW\x81_a/rV[\x91PP` \x81=` \x11a/\xFDW[\x81a/\xEC` \x93\x83aPXV[\x81\x01\x03\x12a\x12(W\x82\x90Q_a.\xF2V[=\x91Pa/\xDFV[P` \x81=` \x11a01W[\x81a0\x1F` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa.\xB4\x90Qa.\xAAV[=\x91Pa0\x12V[\x91P` \x82=` \x11a0vW[\x81a0T` \x93\x83aPXV[\x81\x01\x03\x12a\x133Wc\xFF\xFF\xFF\xFF\x80a0na.n\x94aR\x0FV[\x93PPa.WV[=\x91Pa0GV[\x90P` \x81=` \x11a0\xB0W[\x81a0\x99` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wa0\xAA\x90aQ\xFBV[_a.\x01V[=\x91Pa0\x8CV[P` \x81=` \x11a0\xEBW[\x81a0\xD2` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wa0\xE6a-\xC4\x91aQ\xFBV[a-\xADV[=\x91Pa0\xC5V[P` \x81=` \x11a1&W[\x81a1\r` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wa1!a-q\x91aQ\xFBV[a-ZV[=\x91Pa1\0V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x1CTa1K\x81aP\xE0V[\x91a1Y`@Q\x93\x84aPXV[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a1\x9BW`@Q\x80a\x18\x07\x87\x82aO\xA5V[`\x02` `\x01\x92`@Qa1\xAE\x81aP<V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra1\xC6\x85\x87\x01aR V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a1\x86V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x1DTa2\x1F\x81aP\xE0V[\x91a2-`@Q\x93\x84aPXV[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a2oW`@Q\x80a\x18\x07\x87\x82aO\xA5V[`\x02` `\x01\x92`@Qa2\x82\x81aP<V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra2\x9A\x85\x87\x01aR V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a2ZV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x1ATa2\xC9\x81aP\xE0V[\x91a2\xD7`@Q\x93\x84aPXV[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a3\x19W`@Q\x80a\x18\x07\x87\x82aO(V[`\x01` \x81\x92a3(\x85aP\xF8V[\x81R\x01\x92\x01\x92\x01\x91\x90a3\x04V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W` `\x03\x196\x01\x12a\x06\x03Wa3\x80b\x98\x96\x80aR\x08`\x045aZ\xEBV[`@\x90\x82\x80\x83Qa3\x91\x85\x82aPXV[`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x84Qa3\xFC\x81a#\r` \x82\x01\x94\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x89`$\x84\x01R`d\x83\x01\x90aN\x90V[Q\x90jconsole.logZ\xFAP\x82`\x01`\x01`\xA0\x1B\x03`\"T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13LW\x83Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a&\x1DWa5\xBFW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16\x92c\xFF\xFF\xFF\xFF`\x01`\x01`\xA0\x1B\x03`&T\x16\x91\x16\x93\x82;\x15a\x133W\x85Q\x7F\x85\x93\x1Bt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`D\x82\x01R\x90\x82\x90\x82\x90`d\x90\x82\x90\x84\x90Z\xF1\x80\x15a&\x1DWa5\xAAW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x84Q\x92\x83\x80\x92\x7F<\xBBiy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x92\x83\x15a%\xD8WP\x83\x92a5gW[Pc\xFF\xFF\xFF\xFFa\x11\xF9\x92\x16aYKV[\x91P` \x82=` \x11a5\xA2W[\x81a5\x82` \x93\x83aPXV[\x81\x01\x03\x12a\x13/Wc\xFF\xFF\xFF\xFFa5\x9Ba\x11\xF9\x93aR\x0FV[\x92Pa5WV[=\x91Pa5uV[\x81a5\xB4\x91aPXV[a\x13/W\x82_a5\x08V[\x81a5\xC9\x91aPXV[a\x13/W\x82_a4oV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x1BTa5\xF1\x81aP\xE0V[a5\xFE`@Q\x91\x82aPXV[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a6\xD6W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a6kWPPPP\x03\x90\xF3[\x91\x93` a6\xC6\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a6\xB6\x83Q`@\x84R`@\x84\x01\x90aN\x90V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01RaN\xD3V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a6\\V[`\x02` `\x01\x92`@Qa6\xE9\x81aP<V[a6\xF2\x86aP\xF8V[\x81Ra6\xFF\x85\x87\x01aR V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a6.V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`&T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`%T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa;\xF2W[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x06\x06Wa;\xBBW[P`!T`\x1FT` \x80T`@Q\x7F\xDDb\xED>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03`\x08\x94\x90\x94\x1C\x84\x16`\x04\x82\x01R\x90\x83\x16`$\x82\x01R\x92\x90\x91\x83\x91`D\x91\x83\x91\x16Z\xFA\x80\x15a\x06\x06W\x82\x90a;\x87W[a8\xB2\x91PaW\xD5V[\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa;rW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x80;\x15a\x064W`@Q\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x06\x06Wa;]W[PP`\x01`\x01`\xA0\x1B\x03`!T\x16\x81`\x01`\x01`\xA0\x1B\x03` T\x16`@Q\x90\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81\x86Z\xFA\x90\x81\x15a\x06\x06W\x82\x91a;(W[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13LW`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x82\x01R\x81\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a\x06\x06Wa;\x13W[PP` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`$`@Q\x80\x94\x81\x93\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x80\x15a\x06\x06W\x82\x90a:\xDFW[a\x11\xF9\x91PaW\xD5V[P` \x81=` \x11a;\x0BW[\x81a:\xF9` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa\x11\xF9\x90Qa:\xD5V[=\x91Pa:\xECV[\x81a;\x1D\x91aPXV[a\x13LW\x81_a:\x82V[\x91PP` \x81=` \x11a;UW[\x81a;D` \x93\x83aPXV[\x81\x01\x03\x12a\x12(W\x82\x90Q_a:\x03V[=\x91Pa;7V[\x81a;g\x91aPXV[a\x06\x03W\x80_a9\xA5V[\x81a;|\x91aPXV[a\x06\x03W\x80_a9\x15V[P` \x81=` \x11a;\xB3W[\x81a;\xA1` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa8\xB2\x90Qa8\xA8V[=\x91Pa;\x94V[` \x81=` \x11a;\xEAW[\x81a;\xD4` \x93\x83aPXV[\x81\x01\x03\x12a\x13LWa;\xE5\x90aP\xD3V[a8@V[=\x91Pa;\xC7V[\x81a;\xFC\x91aPXV[a\x06\x03W\x80_a7\xD3V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `\x01`\x01`\xA0\x1B\x03`$T\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a<\x8DWa\x18\x07\x85a\x17\xFB\x81\x87\x03\x82aPXV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a<vV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a=\x0BWa\x18\x07\x85a\x17\xFB\x81\x87\x03\x82aPXV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a<\xF4V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` c\xFF\xFF\xFF\xFF`&T`\xA0\x1C\x16`@Q\x90\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80`\x01`\x01`\xA0\x1B\x03` T\x16\x80;\x15a\x06\x11W\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7Fh\x13\xD7\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x01`\x04\x84\x01RZ\xF1\x80\x15a\x06\x06Wa@\x95W[P`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa@\x80W[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x06\x06Wa@IW[P\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90c\xCAf\x9F\xA7`\xE0\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa@4W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x03W\x80`@Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FOptimism bridge failed\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\x81\x81`d\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa@\x1FW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x90\x80;\x15a\x064W`@Q\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01Ri\x15-\x02\xC7\xE1J\xF6\x80\0\0`$\x83\x01R```D\x83\x01R_`d\x83\x01R\x82\x90\x82\x90`\x84\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x06\x06Wa\x05\xF2WP\xF3[\x81a@)\x91aPXV[a\x06\x03W\x80_a?\x8DV[\x81a@>\x91aPXV[a\x06\x03W\x80_a>\xEDV[` \x81=` \x11a@xW[\x81a@b` \x93\x83aPXV[\x81\x01\x03\x12a\x13LWa@s\x90aP\xD3V[a>\x89V[=\x91Pa@UV[\x81a@\x8A\x91aPXV[a\x06\x03W\x80_a>\x1CV[\x81a@\x9F\x91aPXV[a\x06\x03W\x80_a=\xB9V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x04```\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F2\xD4\xF5\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x83\x92\x84\x92aAMW[Pc\xFF\xFF\xFF\xFFa\x11\xF9\x93aA+\x82\x93`\x01`\x01`\xA0\x1B\x03`%T\x16\x90aY\xC1V[aAB`&T\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x90aY\xC1V[`\xA0\x1C\x16\x91\x16aYKV[\x92PPP``\x81=``\x11aA\xADW[\x81aAj``\x93\x83aPXV[\x81\x01\x03\x12a\x13LW\x80c\xFF\xFF\xFF\xFF\x80aA\x85a\x11\xF9\x94aQ\xFBV[aA+aA\xA0`@aA\x99` \x88\x01aQ\xFBV[\x96\x01aR\x0FV[\x94\x95P\x90\x92PaA\n\x90PV[=\x91PaA]V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`\x1ETaA\xD2\x81aP\xE0V[aA\xDF`@Q\x91\x82aPXV[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10aC W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10aBKW\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10aB\xD7WPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93aB>V[\x90\x91\x92\x93\x94` \x80aC\x13\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89QaN\x90V[\x97\x01\x95\x01\x93\x92\x91\x01aB\xB3V[`@QaC,\x81aP<V[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80TaCH\x81aP\xE0V[\x91aCV`@Q\x93\x84aPXV[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10aC\x8CWPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90aB\x0FV[`\x01` \x81\x92aC\x9B\x86aP\xF8V[\x81R\x01\x93\x01\x91\x01\x90\x91aCfV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `@Qj\x04\"\xCA\x8B\n\0\xA4%\0\0\0\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10aD.Wa\x18\x07\x85a\x17\xFB\x81\x87\x03\x82aPXV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01aD\x17V[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W` `@Qi\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0\x81R\xF3[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80`\x01`\x01`\xA0\x1B\x03`$T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06WaG\xB1W[P`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x11\x8C8\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x06\x06W\x82\x91aG|W[PaE\xD7`\x01`\x01`\xA0\x1B\x03`$T\x16aE\xAB`@Q\x93\x84\x92\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x85\x01R`$\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82aPXV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W\x81aF2\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90aN\x90V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06WaGgW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`%T\x16`&T\x82;\x15a\x06\x84W`@Q\x7F\x85\x93\x1Bt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x81\x16`$\x83\x01R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16`D\x82\x01R\x90\x82\x90\x82\x90\x81\x83\x81`d\x81\x01\x03\x92Z\xF1\x80\x15a\x06\x06WaGRW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x03W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06Wa\x05\xF2WP\xF3[\x81aG\\\x91aPXV[a\x06\x03W\x80_aF\xE4V[\x81aGq\x91aPXV[a\x06\x03W\x80_aFWV[\x91PP` \x81=` \x11aG\xA9W[\x81aG\x98` \x93\x83aPXV[\x81\x01\x03\x12a\x12(W\x81\x90Q_aEMV[=\x91PaG\x8BV[\x81aG\xBB\x91aPXV[a\x06\x03W\x80_aD\xFFV[P4a\x06\x03W\x80`\x03\x196\x01\x12a\x06\x03W\x80`\x01`\x01`\xA0\x1B\x03`#T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x11W`@Q\x90\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06WaK\xBBW[P` `\x01`\x01`\xA0\x1B\x03`!T\x16`D`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x93\x84\x92\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x84\x01RZ\xF1\x80\x15a\x06\x06WaK\x84W[P\x80`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x064W\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri*Z\x05\x8F\xC2\x95\xED\0\0\0`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x06\x06WaKoW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`\x01`\x01`\xA0\x1B\x03`!T\x16\x81;\x15a\x064W\x82\x91`\x84\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x18\xB6\x8B\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Ri?\x87\x08W\xA3\xE0\xE3\x80\0\0`$\x84\x01R```D\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15a\x06\x06WaKZW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06\x03W\x80`@Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x06\x06WaKEW[PP`\x04` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x92\x83\x80\x92\x7F7\x1B\xEDh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90aK\x11W[aJ\x87\x91PaV\xD4V[`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x103\xB4\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x06\x06W\x82\x90aJ\xDDW[a\x11\xF9\x91PaWUV[P` \x81=` \x11aK\tW[\x81aJ\xF7` \x93\x83aPXV[\x81\x01\x03\x12a\x12(Wa\x11\xF9\x90QaJ\xD3V[=\x91PaJ\xEAV[P` \x81=` \x11aK=W[\x81aK+` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WaJ\x87\x90QaJ}V[=\x91PaK\x1EV[\x81aKO\x91aPXV[a\x06\x03W\x80_aJ3V[\x81aKd\x91aPXV[a\x06\x03W\x80_aI\xC7V[\x81aKy\x91aPXV[a\x06\x03W\x80_aIDV[` \x81=` \x11aK\xB3W[\x81aK\x9D` \x93\x83aPXV[\x81\x01\x03\x12a\x13LWaK\xAE\x90aP\xD3V[aH\xC0V[=\x91PaK\x90V[\x81aK\xC5\x91aPXV[a\x06\x03W\x80_aHSV[\x90P4a\x12(W_`\x03\x196\x01\x12a\x12(Wa\x0C,\x80\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aN!W\x82\x91a\\\xAF\x839\x03\x90_\xF0\x80\x15aN\x16W`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`@Qa\n\xB2\x80\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aN!W\x82\x91ah\xDB\x839\x03\x90_\xF0\x80\x15aN\x16W`\x01`\x01`\xA0\x1B\x03\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` U`\x01`\x01`\xA0\x1B\x03`\"T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x90`\x01`\x01`\xA0\x1B\x03`%T\x16\x92`&T\x93`@Q\x94a\x1BT\x91\x82\x87\x01\x93\x87\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17aN!Wa\x01\0\x96\x88\x96c\xFF\xFF\xFF\xFF\x95as\x8D\x899\x86R` \x86\x01R`@\x85\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0``\x85\x01Rj\x04\"\xCA\x8B\n\0\xA4%\0\0\0`\x80\x85\x01R`\xA0\x84\x01R`\x01`\x01`\xA0\x1B\x03\x81\x16`\xC0\x84\x01R`\xA0\x1C\x16`\xE0\x82\x01R\x03\x01\x90_\xF0\x80\x15aN\x16W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU`\x01`\x01`\xA0\x1B\x03`!T\x16`\x01`\x01`\xA0\x1B\x03`#T\x16\x81;\x15a\x12(W_\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01Rj\x08E\x95\x16\x14\x01HJ\0\0\0`$\x84\x01RZ\xF1\x80\x15aN\x16WaN\x08WP\x80\xF3[aN\x14\x91P_\x90aPXV[\0[`@Q=_\x82>=\x90\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10aNqWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aNdV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10aN\xF0WPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aN\xE3V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aOZWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aO\x96\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89QaN\x90V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aOKV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10aO\xD7WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80aP-\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90aN\xD3V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90aO\xC8V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aN!W`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aN!W`@RV[\x91\x90\x82\x01\x80\x92\x11aP\xA6WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[Q\x90\x81\x15\x15\x82\x03a\x12(WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11aN!W`\x05\x1B` \x01\x90V[\x90`@Q\x91_\x81T\x90\x81`\x01\x1C\x92`\x01\x83\x16\x92\x83\x15aQ\xF1W[` \x85\x10\x84\x14aQ\xC4W\x84\x87R\x86\x93\x90\x81\x15aQ\x84WP`\x01\x14aQ@W[PaQ>\x92P\x03\x83aPXV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10aQhWPP\x90` aQ>\x92\x82\x01\x01_aQ1V[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92aQOV[` \x93PaQ>\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_aQ1V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x93`\x7F\x16\x93aQ\x12V[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x12(WV[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x12(WV[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10aT7WaQ>\x94T\x91\x81\x81\x10aT\x01W[\x81\x81\x10aS\xCBW[\x81\x81\x10aS\x95W[\x81\x81\x10aS_W[\x81\x81\x10aS)W[\x81\x81\x10aR\xF3W[\x81\x81\x10aR\xBEW[\x10aR\x91W[P\x03\x83aPXV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_aR\x89V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01aR\x83V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01aR{V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01aRsV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01aRkV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01aRcV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01aR[V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01aRSV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91aR;V[`\x08T`\xFF\x16\x80\x15aT\xD3W\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15aN\x16W_\x91aUkW[P\x15\x15\x90V[\x90P` \x81=` \x11aU\x95W[\x81aU\x86` \x93\x83aPXV[\x81\x01\x03\x12a\x12(WQ_aUeV[=\x91PaUyV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11aN!W`@Q\x91aU\xE5`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x84aPXV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x12(W\x82\x81` \x93\x84_\x96\x01^\x01\x01RV[` \x81\x83\x03\x12a\x12(W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x12(W\x01\x90`\xC0\x82\x82\x03\x12a\x12(W`@Q\x91`\xC0\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17aN!W`@RaVP\x81aQ\xFBV[\x83RaV^` \x82\x01aQ\xFBV[` \x84\x01RaVo`@\x82\x01aQ\xFBV[`@\x84\x01R``\x81\x01Q``\x84\x01RaV\x8A`\x80\x82\x01aR\x0FV[`\x80\x84\x01R`\xA0\x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x12(W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x12(W\x81QaV\xBF\x92` \x01aU\x9DV[`\xA0\x82\x01R\x90V[\x91\x90\x82\x03\x91\x82\x11aP\xA6WV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x02`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[_aQ>\x91aPXV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Rii\xE1\r\xE7fv\xD0\x80\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ri\xD3\xC2\x1B\xCE\xCC\xED\xA1\0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01Ri\x9E\xD1\x94\xDB\x19\xB28\xC0\0\0`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x91\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01R`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12(W`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15aN\x16WaWKWPV[\x81\x15aZ\xBEW\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x90\x83\x83\x11a\\*W\x82\x81\x10\x91\x82\x15\x80a\\ W[a\\\x18Wa[\x0E\x84\x86aV\xC7V[\x92`\x01\x84\x01\x80\x94\x11aP\xA6W`\x03\x83\x11\x15\x80a\\\x0FW[a\\\0W`\x03\x19\x83\x10\x15\x80a[\xF6W[a[\xE2W\x85\x83\x11\x15a[\x99WPP\x90a[Q\x84a[V\x93aV\xC7V[aZ\xB4V[\x90\x81\x15a[\x94Wa[g\x92PaP\x99V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11aP\xA6W\x90V[PP\x90V[\x95\x94\x92\x91\x90\x95a[\xAAW[PPPPV[\x83\x94\x95Pa[Q\x90a[\xBC\x93\x94aV\xC7V[\x90\x81\x15a[\x94Wa[\xCD\x92PaV\xC7V[`\x01\x81\x01\x80\x91\x11aP\xA6W\x90_\x80\x80\x80a[\xA4V[PP\x90Pa[\xF3\x92\x91P\x19\x90aV\xC7V[\x90V[P\x82\x19\x84\x11a[5V[PP\x91\x90Pa[\xF3\x92PaP\x99V[P\x82\x84\x11a[%V[P\x92PPP\x90V[P\x84\x82\x11\x15a[\0V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R\xFD\xFE`\x80`@R4a\x03\x13W`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17a\x02&W`@\x90\x81R`\t\x82RhERC20Mock`\xB8\x1B` \x83\x01R\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17a\x02&W`@R`\x04\x81RcE20M`\xE0\x1B` \x82\x01R\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x02&W`\x03T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x03\tW[` \x82\x10\x14a\x02\x08W`\x1F\x81\x11a\x02\xA6W[P` \x92`\x1F\x82\x11`\x01\x14a\x02EW\x92\x81\x92\x93_\x92a\x02:W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x03U[\x80Q`\x01`\x01`@\x1B\x03\x81\x11a\x02&W`\x04T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x02\x1CW[` \x82\x10\x14a\x02\x08W`\x1F\x81\x11a\x01\xA5W[P` \x91`\x1F\x82\x11`\x01\x14a\x01EW\x91\x81\x92_\x92a\x01:W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x04U[`@Qa\t\x14\x90\x81a\x03\x18\x829\xF3[\x01Q\x90P_\x80a\x01\x16V[`\x1F\x19\x82\x16\x92`\x04_R\x80_ \x91_[\x85\x81\x10a\x01\x8DWP\x83`\x01\x95\x10a\x01uW[PPP\x81\x1B\x01`\x04Ua\x01+V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x01gV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x01UV[`\x04_R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B`\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x01\xFEW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x01\xF3WPa\0\xFDV[_\x81U`\x01\x01a\x01\xE6V[\x90\x91P\x81\x90a\x01\xDDV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x7F\x16\x90a\0\xEBV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x01Q\x90P_\x80a\0\xB5V[`\x1F\x19\x82\x16\x93`\x03_R\x80_ \x91_[\x86\x81\x10a\x02\x8EWP\x83`\x01\x95\x96\x10a\x02vW[PPP\x81\x1B\x01`\x03Ua\0\xCAV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02hV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x02UV[`\x03_R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[`\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x02\xFFW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x02\xF4WPa\0\x9BV[_\x81U`\x01\x01a\x02\xE7V[\x90\x91P\x81\x90a\x02\xDEV[\x90`\x7F\x16\x90a\0\x89V[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x06\xFD\xDE\x03\x14a\x07\x03WP\x80c\t^\xA7\xB3\x14a\x06tW\x80c\x18\x16\r\xDD\x14a\x06WW\x80c#\xB8r\xDD\x14a\x04\xE1W\x80c1<\xE5g\x14a\x04\xC6W\x80c@\xC1\x0F\x19\x14a\x03\xE5W\x80cp\xA0\x821\x14a\x03\xA1W\x80c\x95\xD8\x9BA\x14a\x02&W\x80c\x9D\xC2\x9F\xAC\x14a\x018W\x80c\xA9\x05\x9C\xBB\x14a\x01\x07Wc\xDDb\xED>\x14a\0\x95W_\x80\xFD[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\0\xAEa\x08\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\xCBa\x08'V[\x91\x16_R`\x01` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `@_ T`@Q\x90\x81R\xF3[_\x80\xFD[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x01-a\x01#a\x08\x04V[`$5\x903a\x08JV[` `@Q`\x01\x81R\xF3[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x01Qa\x08\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x91\x16\x80\x15a\x01\xFAW\x80_R_` R`@_ T\x82\x81\x10a\x01\xC8W` \x83_\x94\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93\x85\x87R\x86\x84R\x03`@\x86 U\x80`\x02T\x03`\x02U`@Q\x90\x81R\xA3\0[\x90\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`DR`d_\xFD[\x7F\x96\xC6\xFD\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W`@Q_`\x04T\x90\x81`\x01\x1C`\x01\x83\x16\x92\x83\x15a\x03\x97W[` \x82\x10\x84\x14a\x03jW\x81\x85R\x84\x93\x90\x81\x15a\x03(WP`\x01\x14a\x02\xCCW[P\x03`\x1F\x01`\x1F\x19\x16\x81\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x81\x83\x10\x17a\x02\x9FWa\x02\x9B\x82\x91\x82`@R\x82a\x07\xDAV[\x03\x90\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x04_\x90\x81R\x91P\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B[\x81\x83\x10a\x03\x0CWPP\x81\x01` \x01`\x1F\x19a\x02mV[` \x91\x93P\x80`\x01\x91T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x91\x83\x92a\x02\xF6V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x85\x81\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91P`\x1F\x19\x90Pa\x02mV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x90`\x7F\x16\x90a\x02NV[4a\x01\x03W` `\x03\x196\x01\x12a\x01\x03Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x03\xCFa\x08\x04V[\x16_R_` R` `@_ T`@Q\x90\x81R\xF3[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x03\xFEa\x08\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`$5\x81\x15a\x04\x9AW`\x02T\x90\x80\x82\x01\x80\x92\x11a\x04mW` \x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91_\x93`\x02U\x84\x84R\x83\x82R`@\x84 \x81\x81T\x01\x90U`@Q\x90\x81R\xA3\0[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7F\xECD/\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W` `@Q`\x12\x81R\xF3[4a\x01\x03W```\x03\x196\x01\x12a\x01\x03Wa\x04\xFAa\x08\x04V[a\x05\x02a\x08'V[`D5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x92\x83_R`\x01` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`@_ T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x10a\x05~W[Pa\x01-\x93Pa\x08JV[\x83\x81\x10a\x06#W\x84\x15a\x05\xF7W3\x15a\x05\xCBWa\x01-\x94_R`\x01` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R\x83`@_ \x91\x03\x90U\x84a\x05sV[\x7F\x94(\rb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\xE6\x02\xDF\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x83\x90\x7F\xFB\x8FA\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`DR`d_\xFD[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W` `\x02T`@Q\x90\x81R\xF3[4a\x01\x03W`@`\x03\x196\x01\x12a\x01\x03Wa\x06\x8Da\x08\x04V[`$5\x903\x15a\x05\xF7Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x05\xCBW3_R`\x01` R`@_ \x82_R` R\x80`@_ U`@Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` 3\x92\xA3` `@Q`\x01\x81R\xF3[4a\x01\x03W_`\x03\x196\x01\x12a\x01\x03W_`\x03T\x90\x81`\x01\x1C`\x01\x83\x16\x92\x83\x15a\x07\xD0W[` \x82\x10\x84\x14a\x03jW\x81\x85R\x84\x93\x90\x81\x15a\x03(WP`\x01\x14a\x07tWP\x03`\x1F\x01`\x1F\x19\x16\x81\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x81\x83\x10\x17a\x02\x9FWa\x02\x9B\x82\x91\x82`@R\x82a\x07\xDAV[`\x03_\x90\x81R\x91P\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[[\x81\x83\x10a\x07\xB4WPP\x81\x01` \x01`\x1F\x19a\x02mV[` \x91\x93P\x80`\x01\x91T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x91\x83\x92a\x07\x9EV[\x90`\x7F\x16\x90a\x07(V[`\x1F\x19`\x1F` `@\x94\x81\x85R\x80Q\x91\x82\x91\x82\x82\x88\x01R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\x03WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\x03WV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x15a\x01\xFAWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x82\x15a\x04\x9AW\x81_R_` R`@_ T\x81\x81\x10a\x08\xE2W\x81\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92` \x92\x85_R_\x84R\x03`@_ U\x84_R_\x82R`@_ \x81\x81T\x01\x90U`@Q\x90\x81R\xA3V[\x82\x7F\xE4P\xD3\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`DR`d_\xFD`\x80\x80`@R4`\x15Wa\n\x98\x90\x81a\0\x1A\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c7\x1B\xEDh\x14a\x08kWP\x80cS\x06`i\x14a\x06\xADW\x80ch\x13\xD7\x87\x14a\x06^W\x80c\x83\x8B% \x14a\x01LW\x80c\x95\x9B\x82Z\x14a\0\x84Wc\xD3\x07-\x82\x14a\0^W_\x80\xFD[4a\0\x80W_`\x03\x196\x01\x12a\0\x80W` `\xFF`\x01T\x16`@Q\x90\x15\x15\x81R\xF3[_\x80\xFD[4a\0\x80W` `\x03\x196\x01\x12a\0\x80W`\x045_T\x81\x10\x15a\0\x80Wa\0\xAA\x90a\x08\xC7V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x16a\x01Hs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01\x84\x01T\x16\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02\x82\x01T\x16\x90`\x03\x81\x01Ta\x01\x1A`\x05c\xFF\xFF\xFF\xFF`\x04\x85\x01T\x16\x93\x01a\t\xDAV[\x92`@Q\x96\x87\x96\x87R` \x87\x01R`@\x86\x01R``\x85\x01R`\x80\x84\x01R`\xC0`\xA0\x84\x01R`\xC0\x83\x01\x90a\x08\x84V[\x03\x90\xF3[4a\0\x80W`\xC0`\x03\x196\x01\x12a\0\x80W`\x045s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\0\x80W`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\0\x80W`D5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\0\x80W`d5`\x845\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\0\x80W`\xA45\x94g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11a\0\x80W6`#\x87\x01\x12\x15a\0\x80W\x85`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\x80W6`$\x82\x89\x01\x01\x11a\0\x80W`\xFF`\x01T\x16a\x06\0W`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R0`$\x82\x01R\x83`D\x82\x01R` \x81`d\x81_\x8BZ\xF1\x80\x15a\x05\xF5Wa\x05\xBAW[P` _\x91`@Q\x97a\x02}\x89a\t}V[\x88R\x81\x88\x01\x93\x84R`@\x88\x01\x96\x87R``\x88\x01\x94\x85R`\x80\x88\x01\x95\x86R\x80`$`@Q\x9Aa\x02\xD2\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x86\x01\x16\x01\x8Da\t\x99V[\x82\x8CR\x01\x83\x8B\x017\x88\x01\x01R`\xA0\x85\x01\x95\x86R_Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x05aW\x80`\x01a\x03\x07\x92\x01_Ua\x08\xC7V[\x94\x90\x94a\x05\x8EWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x92\x81\x80`\x05\x99Q\x16\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88T\x16\x17\x87UQ\x16\x82`\x01\x87\x01\x91\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UQ\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02\x85\x01\x91\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UQ`\x03\x83\x01Uc\xFF\xFF\xFF\xFF`\x04\x83\x01\x91Q\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x82T\x16\x17\x90U\x01\x90Q\x90\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05aWa\x04 \x82Ta\t,V[`\x1F\x81\x11a\x05\x1CW[P` \x92`\x1F\x82\x11`\x01\x14a\x04\x83W\x92\x81\x92\x93_\x92a\x04xW[PP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82`\x01\x1B\x92`\x03\x1B\x1C\x19\x16\x17\x90U_\x80\xF3[\x01Q\x90P\x83\x80a\x04CV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x16\x93\x83_R\x80_ \x91_[\x86\x81\x10a\x05\x04WP\x83`\x01\x95\x96\x10a\x04\xCDW[PPP\x81\x1B\x01\x90U\0[\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x83\x80\x80a\x04\xC3V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x04\xB0V[\x82_R` _ `\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x05WW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x05LWPa\x04)V[_\x81U`\x01\x01a\x05?V[\x90\x91P\x81\x90a\x056V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[` \x81=` \x11a\x05\xEDW[\x81a\x05\xD3` \x93\x83a\t\x99V[\x81\x01\x03\x12a\0\x80WQ\x80\x15\x15\x81\x03a\0\x80WP` a\x02kV[=\x91Pa\x05\xC6V[`@Q=_\x82>=\x90\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FOptimism bridge failed\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[4a\0\x80W` `\x03\x196\x01\x12a\0\x80W`\x045\x80\x15\x15\x80\x91\x03a\0\x80W`\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x01T\x16\x91\x16\x17`\x01U_\x80\xF3[4a\0\x80W_`\x03\x196\x01\x12a\0\x80W```\xA0`@Qa\x06\xCD\x81a\t}V[_\x81R_` \x82\x01R_`@\x82\x01R_\x83\x82\x01R_`\x80\x82\x01R\x01R_T\x80\x15a\x08\rW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a\x07\xE0Wa\x079s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x08\xC7V[Pa\x01H`@Q\x91a\x07J\x83a\t}V[\x83\x81T\x16\x83Rc\xFF\xFF\xFF\xFF\x84`\x01\x83\x01T\x16\x91` \x85\x01\x92\x83R\x85`\x02\x82\x01T\x16`@\x86\x01\x90\x81R\x86`\x03\x83\x01T\x91``\x88\x01\x92\x83R\x81a\x07\x9B`\x05\x87`\x04\x88\x01T\x16\x96`\x80\x8C\x01\x97\x88R\x01a\t\xDAV[\x96`\xA0\x8A\x01\x97\x88R`@Q\x9A\x8B\x9A` \x8CRQ\x16` \x8B\x01RQ\x16`@\x89\x01RQ\x16``\x87\x01RQ`\x80\x86\x01RQ\x16`\xA0\x84\x01RQ`\xC0\x80\x84\x01R`\xE0\x83\x01\x90a\x08\x84V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7FNo deposits\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[4a\0\x80W_`\x03\x196\x01\x12a\0\x80W` \x90_T\x81R\xF3[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[_T\x81\x10\x15a\x08\xFFW_\x80\x80R`\x06\x91\x90\x91\x02\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x01\x91V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\tsW[` \x83\x10\x14a\tFWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\t;V[`\xC0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05aW`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x05aW`@RV[\x90`@Q\x91\x82_\x82T\x92a\t\xED\x84a\t,V[\x80\x84R\x93`\x01\x81\x16\x90\x81\x15a\nXWP`\x01\x14a\n\x14W[Pa\n\x12\x92P\x03\x83a\t\x99V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a\n<WPP\x90` a\n\x12\x92\x82\x01\x01_a\n\x05V[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\n#V[` \x93Pa\n\x12\x95\x92P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a\n\x05V`\x80`@R4a\x03'W`@Q`\x1Fa\x1BT8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x02\xF3W\x80\x84\x92a\x01\0\x94`@R\x839\x81\x01\x03\x12a\x03'Wa\0N\x81a\x03+V[a\0Z` \x83\x01a\x03+V[a\0f`@\x84\x01a\x03+V[``\x84\x01Q`\x80\x85\x01Qa\0|`\xA0\x87\x01a\x03+V[\x93`\xE0a\0\x8B`\xC0\x89\x01a\x03+V[\x97\x01Q\x95c\xFF\xFF\xFF\xFF\x87\x16\x87\x03a\x03'W`@\x80Q\x92\x90\x83\x01`\x01`\x01`@\x1B\x03\x81\x11\x84\x82\x10\x17a\x02\xF3W`@R`\x0F\x83RnOptimism Bridge`\x88\x1B` \x84\x01R`\x01\x80U`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15a\x03\x16W[a\x03\x07Wa\x01\x05\x82a\0\xFFa\x01\x0B\x94a\x03?V[Pa\x03\xB5V[Pa\x04HV[P\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xF3W`\x07T\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x02\xE9W[` \x83\x10\x14a\x02\xD5W\x81`\x1F\x84\x93\x11a\x02gW[P` \x90`\x1F\x83\x11`\x01\x14a\x02\x01W_\x92a\x01\xF6W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x07U[`\x02\x80T`\x03\x93\x90\x93U`\x04\x91\x90\x91U`\x01`\x01`\xA8\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17`\x01`\xA0\x1B\x17\x90Ub\x01Q\x80B\x04`\x06U`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91U`\t\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16\x93\x90\x91\x16\x92\x90\x92\x17`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90U`@Qa\x15\xF8\x90\x81a\x04\xDC\x829\xF3[\x01Q\x90P_\x80a\x01YV[`\x07_\x90\x81R\x82\x81 \x93P`\x1F\x19\x85\x16\x90[\x81\x81\x10a\x02OWP\x90\x84`\x01\x95\x94\x93\x92\x10a\x027W[PPP\x81\x1B\x01`\x07Ua\x01nV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02)V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x02\x13V[`\x07_R\x90\x91P\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x02\xCBW[\x90`\x1F\x85\x94\x93\x92\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x02\xBDWPa\x01CV[_\x81U\x84\x93P`\x01\x01a\x02\xB0V[\x90\x91P\x81\x90a\x02\xA2V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x01/V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[c\xD9.#=`\xE0\x1B_R`\x04_\xFD[P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\0\xEBV[_\x80\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03'WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x1B4_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x03\xB0W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x1B4_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_Q` a\x1A\xD4_9_Q\x90_R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x1B\x14_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x03\xB0W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x1B\x14_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7Fu\x1By]$\xB9.=\x92\xD1\xD0\xD8\xF2\x88_N\x9C\x9C&\x9D\xA3P\xAF6\xAEkI\x06\x9B\xAB\xF4\xBF\x90_Q` a\x1A\xD4_9_Q\x90_R\x90\x80\xA4`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_Q` a\x1A\xF4_9_Q\x90_R` R`@\x90 T`\xFF\x16a\x03\xB0W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_Q` a\x1A\xF4_9_Q\x90_R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\xC0t\xBE\xB4k%\x1Fs\xC0\xFE\xC1n\xBAu\xD6\xBC\0B\xD0\x9E\xE1zt\x0E\xBB\xEE0\x8D\xBF\x87\xF8/\x90_Q` a\x1A\xD4_9_Q\x90_R\x90\x80\xA4`\x01\x90V\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x01\xFF\xC9\xA7\x14a\x10\x01WP\x80c\x103\xB4\xCC\x14a\x0F\xE4W\x80c\x11\x8C8\xC7\x14a\x0F\xAAW\x80c\x12Y\xA5\xC8\x14a\x0F\x8DW\x80c\x18\xB6\x8B\x8C\x14a\n\x1EW\x80c$\x8A\x9C\xA3\x14a\t\xEBW\x80c//\xF1]\x14a\t\xADW\x80c2\xD4\xF5\x04\x14a\t[W\x80c4b\xFA\xC3\x14a\t W\x80c6V\x8A\xBE\x14a\x08\xB5W\x80c6\xB0\x89\xD8\x14a\x08\x97W\x80c<\xBBiy\x14a\x08pW\x80cV\xEF\xF2g\x14a\x08<W\x80cZ\xB1\xD6\x1C\x14a\x07\x9EW\x80ca\xB0\xA5n\x14a\x06aW\x80cc\"\x14\x90\x14a\x06>W\x80ce\xD7\xA3\xC9\x14a\x06\nW\x80cf\xD0\x03\xAC\x14a\x05\xD6W\x80cg\xEE\xBA\x0C\x14a\x05\xB8W\x80ck\xCC\x8C\x14\x14a\x04\xEAW\x80c\x85\x93\x1Bt\x14a\x03\xA2W\x80c\x91\xD1HT\x14a\x03KW\x80c\xA2\x17\xFD\xDF\x14a\x03/W\x80c\xB2\r0\xA9\x14a\x02\xD9W\x80c\xC9\xF5\xB6>\x14a\x02\xA5W\x80c\xD5Gt\x1F\x14a\x02^W\x80c\xEA\xD9<\x8F\x14a\x028W\x80c\xED\xE7\xCE\xBD\x14a\x01\xD6Wc\xFB\x8CKQ\x14a\x01]W_\x80\xFD[4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W`\x05T`\x04T\x81\x81\x11\x15a\x01\xCAW\x81\x81\x03\x81\x81\x11a\x01\x9DW``\x93P\x90[`@Q\x92\x83R` \x83\x01R`@\x82\x01R\xF3[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[``\x92\x90a\x01\x8BV[\x80\xFD[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3Wa\x02\x0C`\x02Ta\x01\xF5a\x11SV[\x90`\xFF`@Q\x93\x84\x93``\x85R``\x85\x01\x90a\x12lV[\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16` \x85\x01R`\xA0\x1C\x16\x15\x15`@\x83\x01R\x03\x90\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` `\xFF`\x02T`\xA0\x1C\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x02\xA1`\x045a\x02~a\x10\xC2V[\x90a\x02\x9Ca\x02\x97\x82_R_` R`\x01`@_ \x01T\x90V[a\x13qV[a\x14\xA9V[P\x80\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16`@Q\x90\x81R\xF3[P4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W\x7F |L\xBD\xF5^\xC3\x15\xA1?\r^\x04w2\xEC]\x94}\xA0V\xE7\x06Y:\xA5\t\x90\x99A\xCE\xDF`@`\x045a\x03\x19a\x12\xE9V[`\x04T\x90\x80`\x04U\x82Q\x91\x82R` \x82\x01R\xA1\x80\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` \x90`@Q\x90\x81R\xF3[P4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@a\x03|a\x10\xC2V[\x92`\x045\x81R\x80` R \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xD3W```\x03\x196\x01\x12a\x01\xD3Wa\x03\xBCa\x10\x9FV[a\x03\xC4a\x10\xC2V[`D5\x91c\xFF\xFF\xFF\xFF\x83\x16\x83\x03a\x04\xE6Wa\x04\xE0\x7F\x80+\x8C{$p\x9Bl\x9CV\x17\x9D\xCE\xEB\x97|\xC7\xACo\xA4\xF1_\x84\xC9\x9A\x86'\xAB\xFD\x97\xCC5\x93a\x04\x02a\x12\xE9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x08T\x16\x17`\x08U`\tTw\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`\xA0\x1B\x16\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x91\x16\x17\x17`\tU`@Q\x93\x84\x93\x84\x91`@\x91\x94\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFc\xFF\xFF\xFF\xFF\x92\x81``\x87\x01\x98\x16\x86R\x16` \x85\x01R\x16\x91\x01RV[\x03\x90\xA1\x80\xF3[\x83\x80\xFD[P4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\x19a\x10\x9FV[a\x05!a\x12\xE9V[\x16\x80\x15a\x05\x90Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17`\x02U\x16\x7F\xB0\x7F\x8B\x1B\x85\x04-t\x02,\x86|\x83n\xDE\xB0\xBC\xD7\x0E\x13[\0B9\r+\x1F\xD1\x08)\x80i\x83\x80\xA3\x80\xF3[`\x04\x82\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` `\x04T`@Q\x90\x81R\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\tT\x16`@Q\x90\x81R\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3Wa\x06:a\x06&a\x11SV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x12lV[\x03\x90\xF3[P4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3Wa\x06Xa\x12\xE9V[`\x045`\x03U\x80\xF3[P4a\x01\xD3W```\x03\x196\x01\x12a\x01\xD3Wa\x06{a\x10\x9FV[`D5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x82\x03a\x04\xE6W\x83\x80R\x83` R`@\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x07nW\x15a\x07FW`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`$\x80\x83\x01\x91\x90\x91R5`D\x80\x83\x01\x91\x90\x91R\x81Ra\x07C\x92\x90\x91\x90a\x07=`d\x84a\x10\xE5V[\x16a\x15qV[\x80\xF3[`\x04\x83\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`D\x84\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04R\x80`$R\xFD[P4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W`\x045\x80\x15\x15\x80\x91\x03a\x088W` \x7F\xB3A\x89\x89\xD0h5\xB5\xC2\x15\xEE\xBBMT\xEDk\xE7\xBB\xB6n\xB4\x80qdt\n.\x08/\xA7\x82\xD5\x91a\x07\xE9a\x12\xE9V[`\x02T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\xA0\x1B\x16\x91\x16\x17`\x02U`@Q\x90\x81R\xA1\x80\xF3[P\x80\xFD[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x08T\x16`@Q\x90\x81R\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` c\xFF\xFF\xFF\xFF`\tT`\xA0\x1C\x16`@Q\x90\x81R\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` `\x03T`@Q\x90\x81R\xF3[P4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x08\xCFa\x10\xC2V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\x08\xF8Wa\x02\xA1\x90`\x045a\x14\xA9V[`\x04\x82\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W` `@Q\x7F\xC0t\xBE\xB4k%\x1Fs\xC0\xFE\xC1n\xBAu\xD6\xBC\0B\xD0\x9E\xE1zt\x0E\xBB\xEE0\x8D\xBF\x87\xF8/\x81R\xF3[P4a\x01\xD3W\x80`\x03\x196\x01\x12a\x01\xD3W`\x08T`\tT`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x82\x16` \x84\x01R`\xA0\x91\x90\x91\x1Cc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x90\xF3[P4a\x01\xD3W`@`\x03\x196\x01\x12a\x01\xD3Wa\x02\xA1`\x045a\t\xCDa\x10\xC2V[\x90a\t\xE6a\x02\x97\x82_R_` R`\x01`@_ \x01T\x90V[a\x13\xD7V[P4a\x01\xD3W` `\x03\x196\x01\x12a\x01\xD3W` a\n\x16`\x045_R_` R`\x01`@_ \x01T\x90V[`@Q\x90\x81R\xF3[P4a\r\xB5W```\x03\x196\x01\x12a\r\xB5Wa\n8a\x10\x9FV[`$5\x90`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r\xB5W6`#\x82\x01\x12\x15a\r\xB5W\x80`\x04\x015\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\r\xB5W\x82\x82\x01\x906`$\x83\x01\x11a\r\xB5W`\x02`\x01T\x14a\x0FeW`\x02`\x01U`\xFF`\x02T`\xA0\x1C\x16\x15a\x0F=W3_\x90\x81R\x7F\xFE\x0C{\xD5\t,znF4\x88\xFA#\x9F2\x05\x8C\x87\xD4>3n.XJ'!2\xCF\x8D\xC9&` R`@\x90 T`\xFF\x16\x15a\x0F\x15Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x83\x15a\x0E\xEDW\x84\x15a\x0E\xC5W`\x03T\x85\x11a\x0E^Wb\x01Q\x80B\x04`\x06T\x81\x11a\x0E\x86W[Pa\x0B\x18\x85`\x05Ta\x12\xAFV[`\x04T\x10a\x0E^Wa\x0Bs`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R3`$\x82\x01R0`D\x82\x01R\x86`d\x82\x01R`d\x81Ra\x0Bm`\x84\x82a\x10\xE5V[\x85a\x15qV[\x15a\x0E3W`@\x90\x82\x90\x03\x12a\r\xB5W`$\x81\x015\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\r\xB5W`D\x015\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\r\xB5W\x90[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16`@Q` _\x81\x83\x01\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x0Cb\x84a\x0C6\x8B\x88`$\x84\x01` \x90\x93\x92\x91\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01\x95\x16\x81R\x01RV[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x86R\x85a\x10\xE5V[\x83Q\x90\x82\x89Z\xF1_Q=\x82a\x0E\x17W[PP\x15a\r\xB9W[PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x08T\x16\x92\x80;\x15a\r\xB5Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93_`\xE4\x92c\xFF\xFF\xFF\xFF\x82\x96`@Q\x98\x89\x97\x88\x96\x7F\x83\x8B% \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88R\x8C`\x04\x89\x01R`$\x88\x01R\x16`D\x86\x01R\x8A`d\x86\x01R\x16`\x84\x84\x01R`\xC0`\xA4\x84\x01R\x81`\xC4\x84\x01RZ\xF1\x80\x15a\r\xAAWa\r\x95W[Pa\rG\x82`\x05Ta\x12\xAFV[`\x05U\x7F=\xBA@\x1D\xC1\xAB\xBF\x01p\x86\x86K3\xBA\x81_S\xAB<\xDB\xB96k\xA6\xBC\xD8\xEC4-\xDD\x98\xE8` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x02T\x16\x93`@Q\x90\x81R\xA3`\x01\x80U\x80\xF3[a\r\xA2\x91\x93P_\x90a\x10\xE5V[_\x91_a\r:V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[a\x0E\na\x0E\x10\x92`@Q\x90\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01R_`D\x82\x01R`D\x81Ra\x0E\x04`d\x82a\x10\xE5V[\x86a\x15qV[\x84a\x15qV[_\x80a\x0CzV[\x90\x91Pa\x0E+WP\x84;\x15\x15[_\x80a\x0CrV[`\x01\x14a\x0E$V[PP`\tTc\xFF\xFF\xFF\xFFs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x91`\xA0\x1C\x16a\x0B\xBCV[\x7Fp\xD1h\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xDAN9\xDDV\xD7,.\xE3\xD12\xE0\x14k\xC3\x9E\x90^x\xE3\xBCd\xC4\x01\x90B\x1C{+\xCE\xF2\xAB`@`\x05T\x81Q\x90\x84\x82R` \x82\x01R\xA1_`\x05U`\x06U_a\x0B\x0BV[\x7F\x1F* \x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xD9.#=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\\B|\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F{\xEA \xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\r\xB5W_`\x03\x196\x01\x12a\r\xB5W` `\x06T`@Q\x90\x81R\xF3[4a\r\xB5W_`\x03\x196\x01\x12a\r\xB5W` `@Q\x7Fu\x1By]$\xB9.=\x92\xD1\xD0\xD8\xF2\x88_N\x9C\x9C&\x9D\xA3P\xAF6\xAEkI\x06\x9B\xAB\xF4\xBF\x81R\xF3[4a\r\xB5W_`\x03\x196\x01\x12a\r\xB5W` `\x05T`@Q\x90\x81R\xF3[4a\r\xB5W` `\x03\x196\x01\x12a\r\xB5W`\x045\x90\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x80\x92\x03a\r\xB5W\x81\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x93\x14\x90\x81\x15a\x10uW[P\x15\x15\x81R\xF3[\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91P\x14\x83a\x10nV[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\xB5WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\xB5WV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x11&W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q\x90_`\x07T\x80`\x01\x1C\x91`\x01\x82\x16\x91\x82\x15a\x12bW[` \x84\x10\x83\x14a\x125W\x83\x86R\x85\x92\x90\x81\x15a\x11\xF8WP`\x01\x14a\x11\x99W[a\x11\x97\x92P\x03\x83a\x10\xE5V[V[P`\x07_\x90\x81R\x90\x91\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88[\x81\x83\x10a\x11\xDCWPP\x90` a\x11\x97\x92\x82\x01\x01a\x11\x8BV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x11\xC4V[` \x92Pa\x11\x97\x94\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01a\x11\x8BV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x92`\x7F\x16\x92a\x11lV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x91\x90\x82\x01\x80\x92\x11a\x12\xBCWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[3_\x90\x81R\x7F\xDF\xFF\x91\xBC\xEE\x88\xF6\xEA\0\xB2rom\x06*P\x9C2\x83W\x93G\x0CU\xBE\xD3\xBC\xC7L\x0F]7` R`@\x90 T`\xFF\x16\x15a\x13!WV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R\x7Fu\x1By]$\xB9.=\x92\xD1\xD0\xD8\xF2\x88_N\x9C\x9C&\x9D\xA3P\xAF6\xAEkI\x06\x9B\xAB\xF4\xBF`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x16_R` R`\xFF`@_ T\x16\x15a\x13\xA8WPV[\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$R`D_\xFD[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16\x15_\x14a\x14\xA3W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r_\x80\xA4`\x01\x90V[PP_\x90V[\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`\xFF`@_ T\x16_\x14a\x14\xA3W\x80_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81T\x16\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF3\x92\x16\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B_\x80\xA4`\x01\x90V[\x90_` \x91\x82\x81Q\x91\x01\x82\x85Z\xF1\x15a\r\xAAW_Q=a\x15\xEFWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16;\x15[a\x15\xADWPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7FRt\xAF\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[`\x01\x14\x15a\x15\xA6V/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\xFE\x0C{\xD5\t,znF4\x88\xFA#\x9F2\x05\x8C\x87\xD4>3n.XJ'!2\xCF\x8D\xC9&\xDF\xFF\x91\xBC\xEE\x88\xF6\xEA\0\xB2rom\x06*P\x9C2\x83W\x93G\x0CU\xBE\xD3\xBC\xC7L\x0F]7\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5",
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
    /**Event with signature `OptimismConfigUpdated(address,address,uint32)` and selector `0x802b8c7b24709b6c9c56179dceeb977cc7ac6fa4f15f84c99a8627abfd97cc35`.
```solidity
event OptimismConfigUpdated(address l2Token, address recipient, uint32 l2Gas);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OptimismConfigUpdated {
        #[allow(missing_docs)]
        pub l2Token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub l2Gas: u32,
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
        impl alloy_sol_types::SolEvent for OptimismConfigUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OptimismConfigUpdated(address,address,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                128u8, 43u8, 140u8, 123u8, 36u8, 112u8, 155u8, 108u8, 156u8, 86u8, 23u8,
                157u8, 206u8, 235u8, 151u8, 124u8, 199u8, 172u8, 111u8, 164u8, 241u8,
                95u8, 132u8, 201u8, 154u8, 134u8, 39u8, 171u8, 253u8, 151u8, 204u8, 53u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    l2Token: data.0,
                    recipient: data.1,
                    l2Gas: data.2,
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
                        &self.l2Token,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.l2Gas),
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
        impl alloy_sol_types::private::IntoLogData for OptimismConfigUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OptimismConfigUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OptimismConfigUpdated) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `l2Gas()` and selector `0x3cbb6979`.
```solidity
function l2Gas() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2GasCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`l2Gas()`](l2GasCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2GasReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l2GasCall> for UnderlyingRustTuple<'_> {
                fn from(value: l2GasCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2GasCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<l2GasReturn> for UnderlyingRustTuple<'_> {
                fn from(value: l2GasReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2GasReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2GasCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2Gas()";
            const SELECTOR: [u8; 4] = [60u8, 187u8, 105u8, 121u8];
            #[inline]
            fn new<'a>(
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
                        let r: l2GasReturn = r.into();
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
                        let r: l2GasReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `l2Token()` and selector `0x56eff267`.
```solidity
function l2Token() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2TokenCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`l2Token()`](l2TokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2TokenReturn {
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
            impl ::core::convert::From<l2TokenCall> for UnderlyingRustTuple<'_> {
                fn from(value: l2TokenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2TokenCall {
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
            impl ::core::convert::From<l2TokenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: l2TokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2TokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2TokenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2Token()";
            const SELECTOR: [u8; 4] = [86u8, 239u8, 242u8, 103u8];
            #[inline]
            fn new<'a>(
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
                        let r: l2TokenReturn = r.into();
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
                        let r: l2TokenReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `optimismBridge()` and selector `0x7b4d4ce3`.
```solidity
function optimismBridge() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct optimismBridgeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`optimismBridge()`](optimismBridgeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct optimismBridgeReturn {
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
            impl ::core::convert::From<optimismBridgeCall> for UnderlyingRustTuple<'_> {
                fn from(value: optimismBridgeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for optimismBridgeCall {
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
            impl ::core::convert::From<optimismBridgeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: optimismBridgeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for optimismBridgeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for optimismBridgeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "optimismBridge()";
            const SELECTOR: [u8; 4] = [123u8, 77u8, 76u8, 227u8];
            #[inline]
            fn new<'a>(
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
                        let r: optimismBridgeReturn = r.into();
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
                        let r: optimismBridgeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `recipient()` and selector `0x66d003ac`.
```solidity
function recipient() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct recipientCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`recipient()`](recipientCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct recipientReturn {
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
            impl ::core::convert::From<recipientCall> for UnderlyingRustTuple<'_> {
                fn from(value: recipientCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for recipientCall {
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
            impl ::core::convert::From<recipientReturn> for UnderlyingRustTuple<'_> {
                fn from(value: recipientReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for recipientReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for recipientCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "recipient()";
            const SELECTOR: [u8; 4] = [102u8, 208u8, 3u8, 172u8];
            #[inline]
            fn new<'a>(
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
                        let r: recipientReturn = r.into();
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
                        let r: recipientReturn = r.into();
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
    /**Function with signature `testFuzz_ExecuteBridge_CustomRecipient(address)` and selector `0xf2067bd7`.
```solidity
function testFuzz_ExecuteBridge_CustomRecipient(address customRecipient) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_ExecuteBridge_CustomRecipientCall {
        #[allow(missing_docs)]
        pub customRecipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`testFuzz_ExecuteBridge_CustomRecipient(address)`](testFuzz_ExecuteBridge_CustomRecipientCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_ExecuteBridge_CustomRecipientReturn {}
    #[allow(
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
            impl ::core::convert::From<testFuzz_ExecuteBridge_CustomRecipientCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_ExecuteBridge_CustomRecipientCall) -> Self {
                    (value.customRecipient,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_ExecuteBridge_CustomRecipientCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { customRecipient: tuple.0 }
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
            impl ::core::convert::From<testFuzz_ExecuteBridge_CustomRecipientReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_ExecuteBridge_CustomRecipientReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_ExecuteBridge_CustomRecipientReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzz_ExecuteBridge_CustomRecipientReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzz_ExecuteBridge_CustomRecipientCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzz_ExecuteBridge_CustomRecipientCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzz_ExecuteBridge_CustomRecipientReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzz_ExecuteBridge_CustomRecipient(address)";
            const SELECTOR: [u8; 4] = [242u8, 6u8, 123u8, 215u8];
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
                        &self.customRecipient,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzz_ExecuteBridge_CustomRecipientReturn::_tokenize(ret)
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
    /**Function with signature `testFuzz_SetOptimismConfig_ValidGas(uint256)` and selector `0x70287778`.
```solidity
function testFuzz_SetOptimismConfig_ValidGas(uint256 gasLimit) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_SetOptimismConfig_ValidGasCall {
        #[allow(missing_docs)]
        pub gasLimit: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`testFuzz_SetOptimismConfig_ValidGas(uint256)`](testFuzz_SetOptimismConfig_ValidGasCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzz_SetOptimismConfig_ValidGasReturn {}
    #[allow(
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
            impl ::core::convert::From<testFuzz_SetOptimismConfig_ValidGasCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_SetOptimismConfig_ValidGasCall) -> Self {
                    (value.gasLimit,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_SetOptimismConfig_ValidGasCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { gasLimit: tuple.0 }
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
            impl ::core::convert::From<testFuzz_SetOptimismConfig_ValidGasReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzz_SetOptimismConfig_ValidGasReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzz_SetOptimismConfig_ValidGasReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzz_SetOptimismConfig_ValidGasReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzz_SetOptimismConfig_ValidGasCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzz_SetOptimismConfig_ValidGasCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzz_SetOptimismConfig_ValidGasReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzz_SetOptimismConfig_ValidGas(uint256)";
            const SELECTOR: [u8; 4] = [112u8, 40u8, 119u8, 120u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.gasLimit),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzz_SetOptimismConfig_ValidGasReturn::_tokenize(ret)
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
    /**Function with signature `test_Constructor_BridgeInfo()` and selector `0xe178bc5b`.
```solidity
function test_Constructor_BridgeInfo() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Constructor_BridgeInfoCall;
    ///Container type for the return parameters of the [`test_Constructor_BridgeInfo()`](test_Constructor_BridgeInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Constructor_BridgeInfoReturn {}
    #[allow(
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
            impl ::core::convert::From<test_Constructor_BridgeInfoCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Constructor_BridgeInfoCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Constructor_BridgeInfoCall {
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
            impl ::core::convert::From<test_Constructor_BridgeInfoReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Constructor_BridgeInfoReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Constructor_BridgeInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Constructor_BridgeInfoReturn {
            fn _tokenize(
                &self,
            ) -> <test_Constructor_BridgeInfoCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Constructor_BridgeInfoCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Constructor_BridgeInfoReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Constructor_BridgeInfo()";
            const SELECTOR: [u8; 4] = [225u8, 120u8, 188u8, 91u8];
            #[inline]
            fn new<'a>(
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
                test_Constructor_BridgeInfoReturn::_tokenize(ret)
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
    /**Function with signature `test_ExecuteBridge_Success_CustomParams()` and selector `0xfe47a3f4`.
```solidity
function test_ExecuteBridge_Success_CustomParams() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_Success_CustomParamsCall;
    ///Container type for the return parameters of the [`test_ExecuteBridge_Success_CustomParams()`](test_ExecuteBridge_Success_CustomParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_Success_CustomParamsReturn {}
    #[allow(
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
            impl ::core::convert::From<test_ExecuteBridge_Success_CustomParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteBridge_Success_CustomParamsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_Success_CustomParamsCall {
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
            impl ::core::convert::From<test_ExecuteBridge_Success_CustomParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteBridge_Success_CustomParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_Success_CustomParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_ExecuteBridge_Success_CustomParamsReturn {
            fn _tokenize(
                &self,
            ) -> <test_ExecuteBridge_Success_CustomParamsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_ExecuteBridge_Success_CustomParamsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_ExecuteBridge_Success_CustomParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_ExecuteBridge_Success_CustomParams()";
            const SELECTOR: [u8; 4] = [254u8, 71u8, 163u8, 244u8];
            #[inline]
            fn new<'a>(
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
                test_ExecuteBridge_Success_CustomParamsReturn::_tokenize(ret)
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
    /**Function with signature `test_ExecuteBridge_Success_DefaultParams()` and selector `0xf81006b2`.
```solidity
function test_ExecuteBridge_Success_DefaultParams() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_Success_DefaultParamsCall;
    ///Container type for the return parameters of the [`test_ExecuteBridge_Success_DefaultParams()`](test_ExecuteBridge_Success_DefaultParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_Success_DefaultParamsReturn {}
    #[allow(
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
            impl ::core::convert::From<test_ExecuteBridge_Success_DefaultParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteBridge_Success_DefaultParamsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_Success_DefaultParamsCall {
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
            impl ::core::convert::From<test_ExecuteBridge_Success_DefaultParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteBridge_Success_DefaultParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_Success_DefaultParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_ExecuteBridge_Success_DefaultParamsReturn {
            fn _tokenize(
                &self,
            ) -> <test_ExecuteBridge_Success_DefaultParamsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_ExecuteBridge_Success_DefaultParamsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_ExecuteBridge_Success_DefaultParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_ExecuteBridge_Success_DefaultParams()";
            const SELECTOR: [u8; 4] = [248u8, 16u8, 6u8, 178u8];
            #[inline]
            fn new<'a>(
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
                test_ExecuteBridge_Success_DefaultParamsReturn::_tokenize(ret)
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
    /**Function with signature `test_ExecuteBridge_Success_MultipleTransfers()` and selector `0x0d170b02`.
```solidity
function test_ExecuteBridge_Success_MultipleTransfers() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_Success_MultipleTransfersCall;
    ///Container type for the return parameters of the [`test_ExecuteBridge_Success_MultipleTransfers()`](test_ExecuteBridge_Success_MultipleTransfersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_Success_MultipleTransfersReturn {}
    #[allow(
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
            impl ::core::convert::From<test_ExecuteBridge_Success_MultipleTransfersCall>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_ExecuteBridge_Success_MultipleTransfersCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_Success_MultipleTransfersCall {
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
                test_ExecuteBridge_Success_MultipleTransfersReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_ExecuteBridge_Success_MultipleTransfersReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_Success_MultipleTransfersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_ExecuteBridge_Success_MultipleTransfersReturn {
            fn _tokenize(
                &self,
            ) -> <test_ExecuteBridge_Success_MultipleTransfersCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_ExecuteBridge_Success_MultipleTransfersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_ExecuteBridge_Success_MultipleTransfersReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_ExecuteBridge_Success_MultipleTransfers()";
            const SELECTOR: [u8; 4] = [13u8, 23u8, 11u8, 2u8];
            #[inline]
            fn new<'a>(
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
                test_ExecuteBridge_Success_MultipleTransfersReturn::_tokenize(ret)
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
    /**Function with signature `test_ExecuteBridge_TokenApprovalHandling()` and selector `0x52743ec4`.
```solidity
function test_ExecuteBridge_TokenApprovalHandling() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_TokenApprovalHandlingCall;
    ///Container type for the return parameters of the [`test_ExecuteBridge_TokenApprovalHandling()`](test_ExecuteBridge_TokenApprovalHandlingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ExecuteBridge_TokenApprovalHandlingReturn {}
    #[allow(
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
            impl ::core::convert::From<test_ExecuteBridge_TokenApprovalHandlingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteBridge_TokenApprovalHandlingCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_TokenApprovalHandlingCall {
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
            impl ::core::convert::From<test_ExecuteBridge_TokenApprovalHandlingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ExecuteBridge_TokenApprovalHandlingReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ExecuteBridge_TokenApprovalHandlingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_ExecuteBridge_TokenApprovalHandlingReturn {
            fn _tokenize(
                &self,
            ) -> <test_ExecuteBridge_TokenApprovalHandlingCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_ExecuteBridge_TokenApprovalHandlingCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_ExecuteBridge_TokenApprovalHandlingReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_ExecuteBridge_TokenApprovalHandling()";
            const SELECTOR: [u8; 4] = [82u8, 116u8, 62u8, 196u8];
            #[inline]
            fn new<'a>(
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
                test_ExecuteBridge_TokenApprovalHandlingReturn::_tokenize(ret)
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
    /**Function with signature `test_GetOptimismConfig()` and selector `0x2cd38fbf`.
```solidity
function test_GetOptimismConfig() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetOptimismConfigCall;
    ///Container type for the return parameters of the [`test_GetOptimismConfig()`](test_GetOptimismConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_GetOptimismConfigReturn {}
    #[allow(
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
            impl ::core::convert::From<test_GetOptimismConfigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetOptimismConfigCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetOptimismConfigCall {
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
            impl ::core::convert::From<test_GetOptimismConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_GetOptimismConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_GetOptimismConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_GetOptimismConfigReturn {
            fn _tokenize(
                &self,
            ) -> <test_GetOptimismConfigCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_GetOptimismConfigCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_GetOptimismConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_GetOptimismConfig()";
            const SELECTOR: [u8; 4] = [44u8, 211u8, 143u8, 191u8];
            #[inline]
            fn new<'a>(
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
                test_GetOptimismConfigReturn::_tokenize(ret)
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
    /**Function with signature `test_Integration_DailyLimitReset()` and selector `0xccc0cfac`.
```solidity
function test_Integration_DailyLimitReset() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Integration_DailyLimitResetCall;
    ///Container type for the return parameters of the [`test_Integration_DailyLimitReset()`](test_Integration_DailyLimitResetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Integration_DailyLimitResetReturn {}
    #[allow(
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
            impl ::core::convert::From<test_Integration_DailyLimitResetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Integration_DailyLimitResetCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Integration_DailyLimitResetCall {
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
            impl ::core::convert::From<test_Integration_DailyLimitResetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Integration_DailyLimitResetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Integration_DailyLimitResetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Integration_DailyLimitResetReturn {
            fn _tokenize(
                &self,
            ) -> <test_Integration_DailyLimitResetCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Integration_DailyLimitResetCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Integration_DailyLimitResetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Integration_DailyLimitReset()";
            const SELECTOR: [u8; 4] = [204u8, 192u8, 207u8, 172u8];
            #[inline]
            fn new<'a>(
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
                test_Integration_DailyLimitResetReturn::_tokenize(ret)
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
    /**Function with signature `test_Integration_FullBridgeFlow()` and selector `0xf697e78a`.
```solidity
function test_Integration_FullBridgeFlow() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Integration_FullBridgeFlowCall;
    ///Container type for the return parameters of the [`test_Integration_FullBridgeFlow()`](test_Integration_FullBridgeFlowCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_Integration_FullBridgeFlowReturn {}
    #[allow(
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
            impl ::core::convert::From<test_Integration_FullBridgeFlowCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Integration_FullBridgeFlowCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Integration_FullBridgeFlowCall {
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
            impl ::core::convert::From<test_Integration_FullBridgeFlowReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_Integration_FullBridgeFlowReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_Integration_FullBridgeFlowReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_Integration_FullBridgeFlowReturn {
            fn _tokenize(
                &self,
            ) -> <test_Integration_FullBridgeFlowCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_Integration_FullBridgeFlowCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_Integration_FullBridgeFlowReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_Integration_FullBridgeFlow()";
            const SELECTOR: [u8; 4] = [246u8, 151u8, 231u8, 138u8];
            #[inline]
            fn new<'a>(
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
                test_Integration_FullBridgeFlowReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_ExecuteBridge_OptimismBridgeFails()` and selector `0x374e0ce6`.
```solidity
function test_RevertWhen_ExecuteBridge_OptimismBridgeFails() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall;
    ///Container type for the return parameters of the [`test_RevertWhen_ExecuteBridge_OptimismBridgeFails()`](test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_ExecuteBridge_OptimismBridgeFailsReturn {}
    #[allow(
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
                test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall {
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
                test_RevertWhen_ExecuteBridge_OptimismBridgeFailsReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_ExecuteBridge_OptimismBridgeFailsReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_ExecuteBridge_OptimismBridgeFailsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_ExecuteBridge_OptimismBridgeFailsReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_ExecuteBridge_OptimismBridgeFailsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_ExecuteBridge_OptimismBridgeFails()";
            const SELECTOR: [u8; 4] = [55u8, 78u8, 12u8, 230u8];
            #[inline]
            fn new<'a>(
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
                test_RevertWhen_ExecuteBridge_OptimismBridgeFailsReturn::_tokenize(ret)
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
    /**Function with signature `test_RevertWhen_SetOptimismConfig_NotAdmin()` and selector `0x0d1ce0dd`.
```solidity
function test_RevertWhen_SetOptimismConfig_NotAdmin() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_SetOptimismConfig_NotAdminCall;
    ///Container type for the return parameters of the [`test_RevertWhen_SetOptimismConfig_NotAdmin()`](test_RevertWhen_SetOptimismConfig_NotAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_RevertWhen_SetOptimismConfig_NotAdminReturn {}
    #[allow(
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
            impl ::core::convert::From<test_RevertWhen_SetOptimismConfig_NotAdminCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_RevertWhen_SetOptimismConfig_NotAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_SetOptimismConfig_NotAdminCall {
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
            impl ::core::convert::From<test_RevertWhen_SetOptimismConfig_NotAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_RevertWhen_SetOptimismConfig_NotAdminReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_RevertWhen_SetOptimismConfig_NotAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_RevertWhen_SetOptimismConfig_NotAdminReturn {
            fn _tokenize(
                &self,
            ) -> <test_RevertWhen_SetOptimismConfig_NotAdminCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_RevertWhen_SetOptimismConfig_NotAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_RevertWhen_SetOptimismConfig_NotAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_RevertWhen_SetOptimismConfig_NotAdmin()";
            const SELECTOR: [u8; 4] = [13u8, 28u8, 224u8, 221u8];
            #[inline]
            fn new<'a>(
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
                test_RevertWhen_SetOptimismConfig_NotAdminReturn::_tokenize(ret)
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
    /**Function with signature `test_SetOptimismConfig_Success()` and selector `0xd8b296da`.
```solidity
function test_SetOptimismConfig_Success() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetOptimismConfig_SuccessCall;
    ///Container type for the return parameters of the [`test_SetOptimismConfig_Success()`](test_SetOptimismConfig_SuccessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_SetOptimismConfig_SuccessReturn {}
    #[allow(
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
            impl ::core::convert::From<test_SetOptimismConfig_SuccessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetOptimismConfig_SuccessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetOptimismConfig_SuccessCall {
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
            impl ::core::convert::From<test_SetOptimismConfig_SuccessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_SetOptimismConfig_SuccessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_SetOptimismConfig_SuccessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_SetOptimismConfig_SuccessReturn {
            fn _tokenize(
                &self,
            ) -> <test_SetOptimismConfig_SuccessCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_SetOptimismConfig_SuccessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_SetOptimismConfig_SuccessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_SetOptimismConfig_Success()";
            const SELECTOR: [u8; 4] = [216u8, 178u8, 150u8, 218u8];
            #[inline]
            fn new<'a>(
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
                test_SetOptimismConfig_SuccessReturn::_tokenize(ret)
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
    ///Container for all the [`OptimismBridgeProxyTest`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum OptimismBridgeProxyTestCalls {
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
        l2Gas(l2GasCall),
        #[allow(missing_docs)]
        l2Token(l2TokenCall),
        #[allow(missing_docs)]
        optimismBridge(optimismBridgeCall),
        #[allow(missing_docs)]
        recipient(recipientCall),
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
        testFuzz_ExecuteBridge_CustomRecipient(
            testFuzz_ExecuteBridge_CustomRecipientCall,
        ),
        #[allow(missing_docs)]
        testFuzz_ExecuteBridge_ValidAmounts(testFuzz_ExecuteBridge_ValidAmountsCall),
        #[allow(missing_docs)]
        testFuzz_SetOptimismConfig_ValidGas(testFuzz_SetOptimismConfig_ValidGasCall),
        #[allow(missing_docs)]
        test_Constructor_BridgeInfo(test_Constructor_BridgeInfoCall),
        #[allow(missing_docs)]
        test_Constructor_RoleAssignment(test_Constructor_RoleAssignmentCall),
        #[allow(missing_docs)]
        test_Constructor_Success(test_Constructor_SuccessCall),
        #[allow(missing_docs)]
        test_ExecuteBridge_Success_CustomParams(
            test_ExecuteBridge_Success_CustomParamsCall,
        ),
        #[allow(missing_docs)]
        test_ExecuteBridge_Success_DefaultParams(
            test_ExecuteBridge_Success_DefaultParamsCall,
        ),
        #[allow(missing_docs)]
        test_ExecuteBridge_Success_MultipleTransfers(
            test_ExecuteBridge_Success_MultipleTransfersCall,
        ),
        #[allow(missing_docs)]
        test_ExecuteBridge_TokenApprovalHandling(
            test_ExecuteBridge_TokenApprovalHandlingCall,
        ),
        #[allow(missing_docs)]
        test_GetOptimismConfig(test_GetOptimismConfigCall),
        #[allow(missing_docs)]
        test_Integration_DailyLimitReset(test_Integration_DailyLimitResetCall),
        #[allow(missing_docs)]
        test_Integration_FullBridgeFlow(test_Integration_FullBridgeFlowCall),
        #[allow(missing_docs)]
        test_RevertWhen_ExecuteBridge_OptimismBridgeFails(
            test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall,
        ),
        #[allow(missing_docs)]
        test_RevertWhen_SetOptimismConfig_NotAdmin(
            test_RevertWhen_SetOptimismConfig_NotAdminCall,
        ),
        #[allow(missing_docs)]
        test_SetOptimismConfig_Success(test_SetOptimismConfig_SuccessCall),
        #[allow(missing_docs)]
        token(tokenCall),
        #[allow(missing_docs)]
        user(userCall),
    }
    impl OptimismBridgeProxyTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [10u8, 146u8, 84u8, 228u8],
            [13u8, 23u8, 11u8, 2u8],
            [13u8, 28u8, 224u8, 221u8],
            [17u8, 126u8, 59u8, 66u8],
            [30u8, 215u8, 131u8, 28u8],
            [36u8, 142u8, 195u8, 38u8],
            [42u8, 222u8, 56u8, 128u8],
            [44u8, 211u8, 143u8, 191u8],
            [55u8, 78u8, 12u8, 230u8],
            [60u8, 187u8, 105u8, 121u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [79u8, 134u8, 50u8, 186u8],
            [82u8, 116u8, 62u8, 196u8],
            [86u8, 239u8, 242u8, 103u8],
            [102u8, 208u8, 3u8, 172u8],
            [102u8, 217u8, 169u8, 160u8],
            [112u8, 40u8, 119u8, 120u8],
            [123u8, 77u8, 76u8, 227u8],
            [133u8, 34u8, 108u8, 129u8],
            [145u8, 106u8, 23u8, 198u8],
            [163u8, 212u8, 72u8, 91u8],
            [176u8, 70u8, 79u8, 220u8],
            [180u8, 77u8, 201u8, 214u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [204u8, 192u8, 207u8, 172u8],
            [207u8, 251u8, 4u8, 139u8],
            [216u8, 178u8, 150u8, 218u8],
            [220u8, 204u8, 87u8, 241u8],
            [225u8, 120u8, 188u8, 91u8],
            [226u8, 12u8, 159u8, 113u8],
            [242u8, 6u8, 123u8, 215u8],
            [246u8, 151u8, 231u8, 138u8],
            [248u8, 16u8, 6u8, 178u8],
            [248u8, 81u8, 164u8, 64u8],
            [250u8, 118u8, 38u8, 212u8],
            [252u8, 12u8, 84u8, 106u8],
            [252u8, 156u8, 141u8, 57u8],
            [254u8, 71u8, 163u8, 244u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(setUp),
            ::core::stringify!(test_ExecuteBridge_Success_MultipleTransfers),
            ::core::stringify!(test_RevertWhen_SetOptimismConfig_NotAdmin),
            ::core::stringify!(MAX_SINGLE_TRANSFER),
            ::core::stringify!(excludeSenders),
            ::core::stringify!(DAILY_LIMIT),
            ::core::stringify!(targetInterfaces),
            ::core::stringify!(test_GetOptimismConfig),
            ::core::stringify!(test_RevertWhen_ExecuteBridge_OptimismBridgeFails),
            ::core::stringify!(l2Gas),
            ::core::stringify!(targetSenders),
            ::core::stringify!(targetContracts),
            ::core::stringify!(user),
            ::core::stringify!(test_ExecuteBridge_TokenApprovalHandling),
            ::core::stringify!(l2Token),
            ::core::stringify!(recipient),
            ::core::stringify!(targetArtifactSelectors),
            ::core::stringify!(testFuzz_SetOptimismConfig_ValidGas),
            ::core::stringify!(optimismBridge),
            ::core::stringify!(targetArtifacts),
            ::core::stringify!(targetSelectors),
            ::core::stringify!(bridgeProxy),
            ::core::stringify!(excludeSelectors),
            ::core::stringify!(test_Constructor_Success),
            ::core::stringify!(excludeArtifacts),
            ::core::stringify!(failed),
            ::core::stringify!(test_Integration_DailyLimitReset),
            ::core::stringify!(testFuzz_ExecuteBridge_ValidAmounts),
            ::core::stringify!(test_SetOptimismConfig_Success),
            ::core::stringify!(test_Constructor_RoleAssignment),
            ::core::stringify!(test_Constructor_BridgeInfo),
            ::core::stringify!(excludeContracts),
            ::core::stringify!(testFuzz_ExecuteBridge_CustomRecipient),
            ::core::stringify!(test_Integration_FullBridgeFlow),
            ::core::stringify!(test_ExecuteBridge_Success_DefaultParams),
            ::core::stringify!(admin),
            ::core::stringify!(IS_TEST),
            ::core::stringify!(token),
            ::core::stringify!(caller),
            ::core::stringify!(test_ExecuteBridge_Success_CustomParams),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <setUpCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_ExecuteBridge_Success_MultipleTransfersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_SetOptimismConfig_NotAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <MAX_SINGLE_TRANSFERCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DAILY_LIMITCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetInterfacesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_GetOptimismConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2GasCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <userCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_ExecuteBridge_TokenApprovalHandlingCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2TokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <recipientCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testFuzz_SetOptimismConfig_ValidGasCall as alloy_sol_types::SolCall>::SIGNATURE,
            <optimismBridgeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <bridgeProxyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Constructor_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <failedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Integration_DailyLimitResetCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testFuzz_ExecuteBridge_ValidAmountsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_SetOptimismConfig_SuccessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Constructor_BridgeInfoCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testFuzz_ExecuteBridge_CustomRecipientCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_Integration_FullBridgeFlowCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_ExecuteBridge_Success_DefaultParamsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <adminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <IS_TESTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <tokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <callerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_ExecuteBridge_Success_CustomParamsCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for OptimismBridgeProxyTestCalls {
        const NAME: &'static str = "OptimismBridgeProxyTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 40usize;
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
                Self::l2Gas(_) => <l2GasCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::l2Token(_) => <l2TokenCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::optimismBridge(_) => {
                    <optimismBridgeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::recipient(_) => {
                    <recipientCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                Self::testFuzz_ExecuteBridge_CustomRecipient(_) => {
                    <testFuzz_ExecuteBridge_CustomRecipientCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testFuzz_ExecuteBridge_ValidAmounts(_) => {
                    <testFuzz_ExecuteBridge_ValidAmountsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testFuzz_SetOptimismConfig_ValidGas(_) => {
                    <testFuzz_SetOptimismConfig_ValidGasCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Constructor_BridgeInfo(_) => {
                    <test_Constructor_BridgeInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Constructor_RoleAssignment(_) => {
                    <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Constructor_Success(_) => {
                    <test_Constructor_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_ExecuteBridge_Success_CustomParams(_) => {
                    <test_ExecuteBridge_Success_CustomParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_ExecuteBridge_Success_DefaultParams(_) => {
                    <test_ExecuteBridge_Success_DefaultParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_ExecuteBridge_Success_MultipleTransfers(_) => {
                    <test_ExecuteBridge_Success_MultipleTransfersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_ExecuteBridge_TokenApprovalHandling(_) => {
                    <test_ExecuteBridge_TokenApprovalHandlingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_GetOptimismConfig(_) => {
                    <test_GetOptimismConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Integration_DailyLimitReset(_) => {
                    <test_Integration_DailyLimitResetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_Integration_FullBridgeFlow(_) => {
                    <test_Integration_FullBridgeFlowCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_ExecuteBridge_OptimismBridgeFails(_) => {
                    <test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_RevertWhen_SetOptimismConfig_NotAdmin(_) => {
                    <test_RevertWhen_SetOptimismConfig_NotAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_SetOptimismConfig_Success(_) => {
                    <test_SetOptimismConfig_SuccessCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls>] = &[
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismBridgeProxyTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn test_ExecuteBridge_Success_MultipleTransfers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_ExecuteBridge_Success_MultipleTransfersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_ExecuteBridge_Success_MultipleTransfers,
                            )
                    }
                    test_ExecuteBridge_Success_MultipleTransfers
                },
                {
                    fn test_RevertWhen_SetOptimismConfig_NotAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_RevertWhen_SetOptimismConfig_NotAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_RevertWhen_SetOptimismConfig_NotAdmin,
                            )
                    }
                    test_RevertWhen_SetOptimismConfig_NotAdmin
                },
                {
                    fn MAX_SINGLE_TRANSFER(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <MAX_SINGLE_TRANSFERCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::MAX_SINGLE_TRANSFER)
                    }
                    MAX_SINGLE_TRANSFER
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn DAILY_LIMIT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <DAILY_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::DAILY_LIMIT)
                    }
                    DAILY_LIMIT
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn test_GetOptimismConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_GetOptimismConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::test_GetOptimismConfig)
                    }
                    test_GetOptimismConfig
                },
                {
                    fn test_RevertWhen_ExecuteBridge_OptimismBridgeFails(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_OptimismBridgeFails,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_OptimismBridgeFails
                },
                {
                    fn l2Gas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <l2GasCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismBridgeProxyTestCalls::l2Gas)
                    }
                    l2Gas
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn user(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <userCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismBridgeProxyTestCalls::user)
                    }
                    user
                },
                {
                    fn test_ExecuteBridge_TokenApprovalHandling(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_ExecuteBridge_TokenApprovalHandlingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_ExecuteBridge_TokenApprovalHandling,
                            )
                    }
                    test_ExecuteBridge_TokenApprovalHandling
                },
                {
                    fn l2Token(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <l2TokenCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismBridgeProxyTestCalls::l2Token)
                    }
                    l2Token
                },
                {
                    fn recipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <recipientCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismBridgeProxyTestCalls::recipient)
                    }
                    recipient
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn testFuzz_SetOptimismConfig_ValidGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <testFuzz_SetOptimismConfig_ValidGasCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::testFuzz_SetOptimismConfig_ValidGas,
                            )
                    }
                    testFuzz_SetOptimismConfig_ValidGas
                },
                {
                    fn optimismBridge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <optimismBridgeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::optimismBridge)
                    }
                    optimismBridge
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn bridgeProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <bridgeProxyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::bridgeProxy)
                    }
                    bridgeProxy
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn test_Constructor_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_Constructor_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::test_Constructor_Success)
                    }
                    test_Constructor_Success
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismBridgeProxyTestCalls::failed)
                    }
                    failed
                },
                {
                    fn test_Integration_DailyLimitReset(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_Integration_DailyLimitResetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_Integration_DailyLimitReset,
                            )
                    }
                    test_Integration_DailyLimitReset
                },
                {
                    fn testFuzz_ExecuteBridge_ValidAmounts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <testFuzz_ExecuteBridge_ValidAmountsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::testFuzz_ExecuteBridge_ValidAmounts,
                            )
                    }
                    testFuzz_ExecuteBridge_ValidAmounts
                },
                {
                    fn test_SetOptimismConfig_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_SetOptimismConfig_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_SetOptimismConfig_Success,
                            )
                    }
                    test_SetOptimismConfig_Success
                },
                {
                    fn test_Constructor_RoleAssignment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_Constructor_RoleAssignment,
                            )
                    }
                    test_Constructor_RoleAssignment
                },
                {
                    fn test_Constructor_BridgeInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_Constructor_BridgeInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_Constructor_BridgeInfo,
                            )
                    }
                    test_Constructor_BridgeInfo
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn testFuzz_ExecuteBridge_CustomRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <testFuzz_ExecuteBridge_CustomRecipientCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::testFuzz_ExecuteBridge_CustomRecipient,
                            )
                    }
                    testFuzz_ExecuteBridge_CustomRecipient
                },
                {
                    fn test_Integration_FullBridgeFlow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_Integration_FullBridgeFlowCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_Integration_FullBridgeFlow,
                            )
                    }
                    test_Integration_FullBridgeFlow
                },
                {
                    fn test_ExecuteBridge_Success_DefaultParams(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_ExecuteBridge_Success_DefaultParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_ExecuteBridge_Success_DefaultParams,
                            )
                    }
                    test_ExecuteBridge_Success_DefaultParams
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismBridgeProxyTestCalls::admin)
                    }
                    admin
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismBridgeProxyTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn token(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <tokenCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismBridgeProxyTestCalls::token)
                    }
                    token
                },
                {
                    fn caller(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <callerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismBridgeProxyTestCalls::caller)
                    }
                    caller
                },
                {
                    fn test_ExecuteBridge_Success_CustomParams(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_ExecuteBridge_Success_CustomParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_ExecuteBridge_Success_CustomParams,
                            )
                    }
                    test_ExecuteBridge_Success_CustomParams
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
            ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls>] = &[
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn test_ExecuteBridge_Success_MultipleTransfers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_ExecuteBridge_Success_MultipleTransfersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_ExecuteBridge_Success_MultipleTransfers,
                            )
                    }
                    test_ExecuteBridge_Success_MultipleTransfers
                },
                {
                    fn test_RevertWhen_SetOptimismConfig_NotAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_RevertWhen_SetOptimismConfig_NotAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_RevertWhen_SetOptimismConfig_NotAdmin,
                            )
                    }
                    test_RevertWhen_SetOptimismConfig_NotAdmin
                },
                {
                    fn MAX_SINGLE_TRANSFER(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <MAX_SINGLE_TRANSFERCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::MAX_SINGLE_TRANSFER)
                    }
                    MAX_SINGLE_TRANSFER
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn DAILY_LIMIT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <DAILY_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::DAILY_LIMIT)
                    }
                    DAILY_LIMIT
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn test_GetOptimismConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_GetOptimismConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::test_GetOptimismConfig)
                    }
                    test_GetOptimismConfig
                },
                {
                    fn test_RevertWhen_ExecuteBridge_OptimismBridgeFails(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_RevertWhen_ExecuteBridge_OptimismBridgeFails,
                            )
                    }
                    test_RevertWhen_ExecuteBridge_OptimismBridgeFails
                },
                {
                    fn l2Gas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <l2GasCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::l2Gas)
                    }
                    l2Gas
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn user(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <userCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::user)
                    }
                    user
                },
                {
                    fn test_ExecuteBridge_TokenApprovalHandling(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_ExecuteBridge_TokenApprovalHandlingCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_ExecuteBridge_TokenApprovalHandling,
                            )
                    }
                    test_ExecuteBridge_TokenApprovalHandling
                },
                {
                    fn l2Token(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <l2TokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::l2Token)
                    }
                    l2Token
                },
                {
                    fn recipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <recipientCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::recipient)
                    }
                    recipient
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn testFuzz_SetOptimismConfig_ValidGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <testFuzz_SetOptimismConfig_ValidGasCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::testFuzz_SetOptimismConfig_ValidGas,
                            )
                    }
                    testFuzz_SetOptimismConfig_ValidGas
                },
                {
                    fn optimismBridge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <optimismBridgeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::optimismBridge)
                    }
                    optimismBridge
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn bridgeProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <bridgeProxyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::bridgeProxy)
                    }
                    bridgeProxy
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn test_Constructor_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_Constructor_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::test_Constructor_Success)
                    }
                    test_Constructor_Success
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::failed)
                    }
                    failed
                },
                {
                    fn test_Integration_DailyLimitReset(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_Integration_DailyLimitResetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_Integration_DailyLimitReset,
                            )
                    }
                    test_Integration_DailyLimitReset
                },
                {
                    fn testFuzz_ExecuteBridge_ValidAmounts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <testFuzz_ExecuteBridge_ValidAmountsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::testFuzz_ExecuteBridge_ValidAmounts,
                            )
                    }
                    testFuzz_ExecuteBridge_ValidAmounts
                },
                {
                    fn test_SetOptimismConfig_Success(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_SetOptimismConfig_SuccessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_SetOptimismConfig_Success,
                            )
                    }
                    test_SetOptimismConfig_Success
                },
                {
                    fn test_Constructor_RoleAssignment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_Constructor_RoleAssignmentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_Constructor_RoleAssignment,
                            )
                    }
                    test_Constructor_RoleAssignment
                },
                {
                    fn test_Constructor_BridgeInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_Constructor_BridgeInfoCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_Constructor_BridgeInfo,
                            )
                    }
                    test_Constructor_BridgeInfo
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn testFuzz_ExecuteBridge_CustomRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <testFuzz_ExecuteBridge_CustomRecipientCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::testFuzz_ExecuteBridge_CustomRecipient,
                            )
                    }
                    testFuzz_ExecuteBridge_CustomRecipient
                },
                {
                    fn test_Integration_FullBridgeFlow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_Integration_FullBridgeFlowCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_Integration_FullBridgeFlow,
                            )
                    }
                    test_Integration_FullBridgeFlow
                },
                {
                    fn test_ExecuteBridge_Success_DefaultParams(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_ExecuteBridge_Success_DefaultParamsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_ExecuteBridge_Success_DefaultParams,
                            )
                    }
                    test_ExecuteBridge_Success_DefaultParams
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::admin)
                    }
                    admin
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn token(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <tokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::token)
                    }
                    token
                },
                {
                    fn caller(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <callerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismBridgeProxyTestCalls::caller)
                    }
                    caller
                },
                {
                    fn test_ExecuteBridge_Success_CustomParams(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismBridgeProxyTestCalls> {
                        <test_ExecuteBridge_Success_CustomParamsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismBridgeProxyTestCalls::test_ExecuteBridge_Success_CustomParams,
                            )
                    }
                    test_ExecuteBridge_Success_CustomParams
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
                Self::l2Gas(inner) => {
                    <l2GasCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::l2Token(inner) => {
                    <l2TokenCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::optimismBridge(inner) => {
                    <optimismBridgeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::recipient(inner) => {
                    <recipientCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::testFuzz_ExecuteBridge_CustomRecipient(inner) => {
                    <testFuzz_ExecuteBridge_CustomRecipientCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testFuzz_ExecuteBridge_ValidAmounts(inner) => {
                    <testFuzz_ExecuteBridge_ValidAmountsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testFuzz_SetOptimismConfig_ValidGas(inner) => {
                    <testFuzz_SetOptimismConfig_ValidGasCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Constructor_BridgeInfo(inner) => {
                    <test_Constructor_BridgeInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::test_ExecuteBridge_Success_CustomParams(inner) => {
                    <test_ExecuteBridge_Success_CustomParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_ExecuteBridge_Success_DefaultParams(inner) => {
                    <test_ExecuteBridge_Success_DefaultParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_ExecuteBridge_Success_MultipleTransfers(inner) => {
                    <test_ExecuteBridge_Success_MultipleTransfersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_ExecuteBridge_TokenApprovalHandling(inner) => {
                    <test_ExecuteBridge_TokenApprovalHandlingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_GetOptimismConfig(inner) => {
                    <test_GetOptimismConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Integration_DailyLimitReset(inner) => {
                    <test_Integration_DailyLimitResetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_Integration_FullBridgeFlow(inner) => {
                    <test_Integration_FullBridgeFlowCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_OptimismBridgeFails(inner) => {
                    <test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_RevertWhen_SetOptimismConfig_NotAdmin(inner) => {
                    <test_RevertWhen_SetOptimismConfig_NotAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_SetOptimismConfig_Success(inner) => {
                    <test_SetOptimismConfig_SuccessCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::l2Gas(inner) => {
                    <l2GasCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::l2Token(inner) => {
                    <l2TokenCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::optimismBridge(inner) => {
                    <optimismBridgeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::recipient(inner) => {
                    <recipientCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::testFuzz_ExecuteBridge_CustomRecipient(inner) => {
                    <testFuzz_ExecuteBridge_CustomRecipientCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::testFuzz_SetOptimismConfig_ValidGas(inner) => {
                    <testFuzz_SetOptimismConfig_ValidGasCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_Constructor_BridgeInfo(inner) => {
                    <test_Constructor_BridgeInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::test_ExecuteBridge_Success_CustomParams(inner) => {
                    <test_ExecuteBridge_Success_CustomParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_ExecuteBridge_Success_DefaultParams(inner) => {
                    <test_ExecuteBridge_Success_DefaultParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_ExecuteBridge_Success_MultipleTransfers(inner) => {
                    <test_ExecuteBridge_Success_MultipleTransfersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_ExecuteBridge_TokenApprovalHandling(inner) => {
                    <test_ExecuteBridge_TokenApprovalHandlingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_GetOptimismConfig(inner) => {
                    <test_GetOptimismConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_Integration_DailyLimitReset(inner) => {
                    <test_Integration_DailyLimitResetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_Integration_FullBridgeFlow(inner) => {
                    <test_Integration_FullBridgeFlowCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_ExecuteBridge_OptimismBridgeFails(inner) => {
                    <test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_RevertWhen_SetOptimismConfig_NotAdmin(inner) => {
                    <test_RevertWhen_SetOptimismConfig_NotAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_SetOptimismConfig_Success(inner) => {
                    <test_SetOptimismConfig_SuccessCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`OptimismBridgeProxyTest`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum OptimismBridgeProxyTestEvents {
        #[allow(missing_docs)]
        BridgeExecuted(BridgeExecuted),
        #[allow(missing_docs)]
        OptimismConfigUpdated(OptimismConfigUpdated),
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
    impl OptimismBridgeProxyTestEvents {
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
                128u8, 43u8, 140u8, 123u8, 36u8, 112u8, 155u8, 108u8, 156u8, 86u8, 23u8,
                157u8, 206u8, 235u8, 151u8, 124u8, 199u8, 172u8, 111u8, 164u8, 241u8,
                95u8, 132u8, 201u8, 154u8, 134u8, 39u8, 171u8, 253u8, 151u8, 204u8, 53u8,
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
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(log_named_array_0),
            ::core::stringify!(log_string),
            ::core::stringify!(log_int),
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
            ::core::stringify!(OptimismConfigUpdated),
            ::core::stringify!(log_array_1),
            ::core::stringify!(log_named_address),
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
            <log_string as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_int as alloy_sol_types::SolEvent>::SIGNATURE,
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
            <OptimismConfigUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for OptimismBridgeProxyTestEvents {
        const NAME: &'static str = "OptimismBridgeProxyTestEvents";
        const COUNT: usize = 24usize;
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
                    <OptimismConfigUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OptimismConfigUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OptimismConfigUpdated)
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
    impl alloy_sol_types::private::IntoLogData for OptimismBridgeProxyTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::BridgeExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OptimismConfigUpdated(inner) => {
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
                Self::OptimismConfigUpdated(inner) => {
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
    /**Creates a new wrapper around an on-chain [`OptimismBridgeProxyTest`](self) contract instance.

See the [wrapper's documentation](`OptimismBridgeProxyTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> OptimismBridgeProxyTestInstance<P, N> {
        OptimismBridgeProxyTestInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<OptimismBridgeProxyTestInstance<P, N>>,
    > {
        OptimismBridgeProxyTestInstance::<P, N>::deploy(__provider)
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
        OptimismBridgeProxyTestInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`OptimismBridgeProxyTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OptimismBridgeProxyTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OptimismBridgeProxyTestInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for OptimismBridgeProxyTestInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OptimismBridgeProxyTestInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > OptimismBridgeProxyTestInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`OptimismBridgeProxyTest`](self) contract instance.

See the [wrapper's documentation](`OptimismBridgeProxyTestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<OptimismBridgeProxyTestInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> OptimismBridgeProxyTestInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OptimismBridgeProxyTestInstance<P, N> {
            OptimismBridgeProxyTestInstance {
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
    > OptimismBridgeProxyTestInstance<P, N> {
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
        ///Creates a new call builder for the [`l2Gas`] function.
        pub fn l2Gas(&self) -> alloy_contract::SolCallBuilder<&P, l2GasCall, N> {
            self.call_builder(&l2GasCall)
        }
        ///Creates a new call builder for the [`l2Token`] function.
        pub fn l2Token(&self) -> alloy_contract::SolCallBuilder<&P, l2TokenCall, N> {
            self.call_builder(&l2TokenCall)
        }
        ///Creates a new call builder for the [`optimismBridge`] function.
        pub fn optimismBridge(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, optimismBridgeCall, N> {
            self.call_builder(&optimismBridgeCall)
        }
        ///Creates a new call builder for the [`recipient`] function.
        pub fn recipient(&self) -> alloy_contract::SolCallBuilder<&P, recipientCall, N> {
            self.call_builder(&recipientCall)
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
        ///Creates a new call builder for the [`testFuzz_ExecuteBridge_CustomRecipient`] function.
        pub fn testFuzz_ExecuteBridge_CustomRecipient(
            &self,
            customRecipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testFuzz_ExecuteBridge_CustomRecipientCall,
            N,
        > {
            self.call_builder(
                &testFuzz_ExecuteBridge_CustomRecipientCall {
                    customRecipient,
                },
            )
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
        ///Creates a new call builder for the [`testFuzz_SetOptimismConfig_ValidGas`] function.
        pub fn testFuzz_SetOptimismConfig_ValidGas(
            &self,
            gasLimit: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testFuzz_SetOptimismConfig_ValidGasCall,
            N,
        > {
            self.call_builder(
                &testFuzz_SetOptimismConfig_ValidGasCall {
                    gasLimit,
                },
            )
        }
        ///Creates a new call builder for the [`test_Constructor_BridgeInfo`] function.
        pub fn test_Constructor_BridgeInfo(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_Constructor_BridgeInfoCall, N> {
            self.call_builder(&test_Constructor_BridgeInfoCall)
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
        ///Creates a new call builder for the [`test_ExecuteBridge_Success_CustomParams`] function.
        pub fn test_ExecuteBridge_Success_CustomParams(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_ExecuteBridge_Success_CustomParamsCall,
            N,
        > {
            self.call_builder(&test_ExecuteBridge_Success_CustomParamsCall)
        }
        ///Creates a new call builder for the [`test_ExecuteBridge_Success_DefaultParams`] function.
        pub fn test_ExecuteBridge_Success_DefaultParams(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_ExecuteBridge_Success_DefaultParamsCall,
            N,
        > {
            self.call_builder(&test_ExecuteBridge_Success_DefaultParamsCall)
        }
        ///Creates a new call builder for the [`test_ExecuteBridge_Success_MultipleTransfers`] function.
        pub fn test_ExecuteBridge_Success_MultipleTransfers(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_ExecuteBridge_Success_MultipleTransfersCall,
            N,
        > {
            self.call_builder(&test_ExecuteBridge_Success_MultipleTransfersCall)
        }
        ///Creates a new call builder for the [`test_ExecuteBridge_TokenApprovalHandling`] function.
        pub fn test_ExecuteBridge_TokenApprovalHandling(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_ExecuteBridge_TokenApprovalHandlingCall,
            N,
        > {
            self.call_builder(&test_ExecuteBridge_TokenApprovalHandlingCall)
        }
        ///Creates a new call builder for the [`test_GetOptimismConfig`] function.
        pub fn test_GetOptimismConfig(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_GetOptimismConfigCall, N> {
            self.call_builder(&test_GetOptimismConfigCall)
        }
        ///Creates a new call builder for the [`test_Integration_DailyLimitReset`] function.
        pub fn test_Integration_DailyLimitReset(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_Integration_DailyLimitResetCall,
            N,
        > {
            self.call_builder(&test_Integration_DailyLimitResetCall)
        }
        ///Creates a new call builder for the [`test_Integration_FullBridgeFlow`] function.
        pub fn test_Integration_FullBridgeFlow(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_Integration_FullBridgeFlowCall, N> {
            self.call_builder(&test_Integration_FullBridgeFlowCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_ExecuteBridge_OptimismBridgeFails`] function.
        pub fn test_RevertWhen_ExecuteBridge_OptimismBridgeFails(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_ExecuteBridge_OptimismBridgeFailsCall)
        }
        ///Creates a new call builder for the [`test_RevertWhen_SetOptimismConfig_NotAdmin`] function.
        pub fn test_RevertWhen_SetOptimismConfig_NotAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_RevertWhen_SetOptimismConfig_NotAdminCall,
            N,
        > {
            self.call_builder(&test_RevertWhen_SetOptimismConfig_NotAdminCall)
        }
        ///Creates a new call builder for the [`test_SetOptimismConfig_Success`] function.
        pub fn test_SetOptimismConfig_Success(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_SetOptimismConfig_SuccessCall, N> {
            self.call_builder(&test_SetOptimismConfig_SuccessCall)
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
    > OptimismBridgeProxyTestInstance<P, N> {
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
        ///Creates a new event filter for the [`OptimismConfigUpdated`] event.
        pub fn OptimismConfigUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, OptimismConfigUpdated, N> {
            self.event_filter::<OptimismConfigUpdated>()
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
