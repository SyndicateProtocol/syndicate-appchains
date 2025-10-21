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

interface RequireOrModuleTest {
    event PermissionCheckAdded(address indexed check);
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
    function emptyData() external view returns (bytes memory);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function module() external view returns (address);
    function nonAdmin() external view returns (address);
    function setUp() external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testAddDuplicatePermissionCheck() external;
    function testAddPermissionCheckToHead() external;
    function testAddPermissionCheckToTail() external;
    function testAddRemovePermissionCheck() external;
    function testCheckCombinations() external;
    function testEmptyGetAllChecks() external view;
    function testIndependentPermissionAndCalldataLists() external;
    function testIsAllowedFailure() external;
    function testIsAllowedNoChecks() external view;
    function testIsAllowedPassOne() external;
    function testIsAllowedPassOnlyOne() external;
    function testOnlyOwnerModifiersForPermissionChecks() external;
    function testRemoveNonExistentPermissionCheck() external;
    function testRevert_TooManyPermissionChecks() external;
    function testRevertsOnZeroAddressAddPermission() external;
    function testRevertsOnZeroAddressRemovePermission() external;
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
    "name": "emptyData",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
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
    "name": "module",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract RequireOrModule"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nonAdmin",
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
    "name": "testAddDuplicatePermissionCheck",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testAddPermissionCheckToHead",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testAddPermissionCheckToTail",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testAddRemovePermissionCheck",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testCheckCombinations",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testEmptyGetAllChecks",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "testIndependentPermissionAndCalldataLists",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedFailure",
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
    "name": "testIsAllowedPassOne",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIsAllowedPassOnlyOne",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testOnlyOwnerModifiersForPermissionChecks",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testRemoveNonExistentPermissionCheck",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testRevert_TooManyPermissionChecks",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testRevertsOnZeroAddressAddPermission",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testRevertsOnZeroAddressRemovePermission",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
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
pub mod RequireOrModuleTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052346101a257600c8054600160ff199182168117909255601f8054909116909117905560405160208082018281106001600160401b0382111761018e576040525f8252601f19810136828401378151906001600160401b03821161018e57602254600181811c91168015610184575b8282101461017057601f811161012c575b5080601f83116001146100c9575081925f926100be575b50508160011b915f199060031b1c1916176022555b60405161534890816101a78239f35b015190505f8061009a565b90601f1983169360225f52825f20925f905b86821061011457505083600195106100fc575b505050811b016022556100af565b01515f1960f88460031b161c191690555f80806100ee565b806001859682949686015181550195019301906100db565b60225f52815f20601f840160051c810191838510610166575b601f0160051c01905b81811061015b5750610083565b5f815560010161014e565b9091508190610145565b634e487b7160e01b5f52602260045260245ffd5b90607f1690610072565b634e487b7160e01b5f52604160045260245ffd5b5f80fdfe6080806040526004361015610012575f80fd5b5f905f3560e01c90816301f8d4df1461339f5750806308226e93146131815780630a9254e41461309d5780631721daaa14612f9c5780631ed7831c14612f1e5780632ade388014612d2257806334d923db14612cc95780633e5e3c2314612c4b5780633f7286f414612bcd57806341e2ac2d14612b6757806357b50acc1461296757806366d9a9a0146128225780636e435a5b14612484578063756896711461224b57806385226c81146121b95780638a3603a314611fc9578063916a17c614611f1f57806391d01c61146119f95780639bd8bed71461182e5780639d62f5ab14611450578063a0974d8714610fd5578063a5081d7314610d7f578063b0464fdc14610cd5578063b5508aa914610c3c578063b86d529814610c12578063b91dc4991461096b578063ba414fa614610946578063c12e46871461091f578063cdc3e010146105ac578063db5bfb0d14610262578063e20c9f71146101d4578063f851a440146101ae5763fa7626d414610189575f80fd5b346101ab57806003193601126101ab57602060ff601f54166040519015158152f35b80fd5b50346101ab57806003193601126101ab5760206001600160a01b03815416604051908152f35b50346101ab57806003193601126101ab5760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106102435761023f856102338187038261361e565b604051918291826136a2565b0390f35b82546001600160a01b031684526020909301926001928301920161021c565b50346101ab57806003193601126101ab576040516101008082019180831067ffffffffffffffff84111761057f57806140ba9383858339039084f08015610574576001600160a01b031691604051918083019183831067ffffffffffffffff84111761054757908392918339039083f080156104f9576001600160a01b031690826001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561051957604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f957610532575b506001600160a01b03601f5460081c16803b156105195781809160446040518094819363052eefd160e01b8352886004840152600160248401525af180156104f95761051d575b506001600160a01b03601f5460081c16803b156105195781809160446040518094819363052eefd160e01b8352896004840152600160248401525af180156104f957610504575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104f557826040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576104e0575b50916004926001600160a01b03601f5460081c16604051948580927f1b42c7110000000000000000000000000000000000000000000000000000000082525afa9283156104d5576104aa936001600160a01b039261049e9287926104ad575b506104a5906104948351613fc0565b8461049e84613a8e565b5116614037565b613ac8565b80f35b6104a59192506104ce903d808a833e6104c6818361361e565b810190613a03565b9190610485565b6040513d86823e3d90fd5b816104ea9161361e565b6104f557825f610426565b8280fd5b6040513d84823e3d90fd5b8161050e9161361e565b6104f557825f6103d3565b5080fd5b816105279161361e565b6104f557825f61038c565b8161053c9161361e565b6104f557825f610345565b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b6040513d85823e3d90fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b50346101ab57806003193601126101ab57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff57604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f95761090a575b5050600460206001600160a01b03601f5460081c16604051928380927fa26b4a880000000000000000000000000000000000000000000000000000000082525afa9081156104f95782916108d8575b50815b81811061082f575060018101809111610802578190737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff576040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f13d867a2000000000000000000000000000000000000000000000000000000006004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156105745783916107ea575b50506001600160a01b03601f5460081c1690813b156107e6576001600160a01b036044849283604051958694859363052eefd160e01b8552166004840152600160248401525af180156104f9576107d1575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b816107ca9161361e565b6101ab5780f35b816107db9161361e565b6101ab57805f61076b565b5050fd5b816107f49161361e565b6107ff57815f610719565b50fd5b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b600181018082116108ab5783906001600160a01b03601f5460081c1690813b156104f5576001600160a01b036044849283604051958694859363052eefd160e01b8552166004840152600160248401525af180156104f957610896575b5050600101610672565b816108a09161361e565b6104f557825f61088c565b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d602011610902575b816108f36020938361361e565b8101031261051957515f61066f565b3d91506108e6565b816109149161361e565b6101ab57805f610620565b50346101ab57806003193601126101ab5760206001600160a01b0360215416604051908152f35b50346101ab57806003193601126101ab576020610961613d7e565b6040519015158152f35b50346101ab57806003193601126101ab5760405190610100918281019281841067ffffffffffffffff851117610be557829382916140ba8339039082f08015610bd8576001600160a01b03166001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657604051906303223eab60e11b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610574578391610bc3575b50506001600160a01b03601f5460081c16803b156107e65782809160446040518094819363052eefd160e01b8352876004840152600160248401525af1908115610574578391610bae575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff576040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fa2d86a1e000000000000000000000000000000000000000000000000000000006004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610574578391610b99575b50506001600160a01b03601f5460081c1690813b156107e657829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156104f9576107d1575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b81610ba39161361e565b6107ff57815f610afb565b81610bb89161361e565b6107ff57815f610a67565b81610bcd9161361e565b6107ff57815f610a1c565b50604051903d90823e3d90fd5b6024837f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b50346101ab57806003193601126101ab5760206001600160a01b03601f5460081c16604051908152f35b50346101ab57806003193601126101ab57601954610c598161384d565b91610c67604051938461361e565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b838310610ca9576040518061023f8782613739565b600160208192604051610cc781610cc08189613922565b038261361e565b815201920192019190610c94565b50346101ab57806003193601126101ab57601c54610cf28161384d565b91610d00604051938461361e565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310610d42576040518061023f87826137b6565b60026020600192604051610d55816135d5565b6001600160a01b038654168152610d6d858701613ad8565b83820152815201920192019190610d2d565b50346101ab57806003193601126101ab576040516101008082019180831067ffffffffffffffff84111761057f57806140ba9383858339039084f08015610574576001600160a01b031691604051918083019183831067ffffffffffffffff84111761054757908392918339039083f080156104f9576001600160a01b0316826001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561051957604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f957610fc0575b506001600160a01b03601f5460081c16803b156105195781809160446040518094819363052eefd160e01b83528960048401528160248401525af180156104f957610fab575b506001600160a01b03601f5460081c16803b156105195781809160446040518094819363052eefd160e01b83528860048401528160248401525af180156104f957610504575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104f557826040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576104e05750916004926001600160a01b03601f5460081c16604051948580927f1b42c7110000000000000000000000000000000000000000000000000000000082525afa9283156104d5576104aa936001600160a01b039261049e9287926104ad57506104a5906104948351613fc0565b81610fb59161361e565b6104f557825f610ea7565b81610fca9161361e565b6104f557825f610e61565b50346101ab57806003193601126101ab5760405190610100918281019281841067ffffffffffffffff851117610be557829382916140ba8339039082f08015610bd8576001600160a01b03166001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657604051906303223eab60e11b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561057457839161143b575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff576040517f491cc7c20000000000000000000000000000000000000000000000000000000081528281806110f160048201905f6060608084019360018152600160208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610574578391611426575b5050604051817f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e78480a26001600160a01b03601f5460081c16803b156113dd578160448186809463052eefd160e01b8352876004840152600160248401525af1908115610574578391611411575b506004906001600160a01b03601f5460081c16604051928380927f1b42c7110000000000000000000000000000000000000000000000000000000082525afa908115610574576001600160a01b0361049e6111f593859387916113f7575b506111f08151613f49565b613a8e565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff576040517f491cc7c200000000000000000000000000000000000000000000000000000000815282818061125e60048201905f6060608084019360018152600160208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156105745783916113e2575b505060405190807fb5d68ca46372bbe6ec138d3d0423608269b3117496a46268f86080cdbcbea9be8480a26001600160a01b03601f5460081c16803b156113dd5760248385819381957f04f386f400000000000000000000000000000000000000000000000000000000845260048401525af180156104f9576113c8575b506004906001600160a01b03601f5460081c16604051928380927f1b42c7110000000000000000000000000000000000000000000000000000000082525afa80156104f95761135b9183916113ae575b5051613ed3565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b6113c291503d8085833e6104c6818361361e565b5f611354565b816113d29161361e565b6101ab57805f611304565b505050fd5b816113ec9161361e565b6107ff57815f611286565b61140b91503d8089833e6104c6818361361e565b5f6111e5565b8161141b9161361e565b6107ff57815f611187565b816114309161361e565b6107ff57815f611119565b816114459161361e565b6107ff57815f611086565b50346101ab57806003193601126101ab576040519060ff8083019280841067ffffffffffffffff851117610be557806152499483868339039083f09081156104f957604051908082019482861067ffffffffffffffff8711176118015794829185968339039083f09081156104f9576001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156113dd57604051906303223eab60e11b82526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156104d55784916117ec575b50506001600160a01b03601f5460081c1690813b156113dd576001600160a01b036044859283604051958694859363052eefd160e01b8552166004840152600160248401525af19081156105745783916117d7575b50506001600160a01b03601f5460081c1690813b156107e6576001600160a01b036044849283604051958694859363052eefd160e01b85521660048401528160248401525af180156104f9576117c2575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576117ad575b506040517f0200da48000000000000000000000000000000000000000000000000000000006020820152306024820152604060448201526116908161166460648201613865565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0810183528261361e565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff57816116eb91604051809381927ff28dceb300000000000000000000000000000000000000000000000000000000835260206004840152602483019061365f565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f957611798575b50506001600160a01b03601f5460081c16602060405180927f7a3979dc000000000000000000000000000000000000000000000000000000008252818061175a30600483016139da565b03915afa80156104f95761176c575080f35b61178d9060203d602011611791575b611785818361361e565b8101906139c2565b5080f35b503d61177b565b816117a29161361e565b6101ab57805f611710565b816117b79161361e565b6101ab57805f61161d565b816117cc9161361e565b6101ab57805f6115ca565b816117e19161361e565b6107ff57815f611579565b816117f69161361e565b6107e657825f611524565b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b50346101ab57806003193601126101ab57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff57604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576119e4575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fe6c4247b000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576119cf575b506001600160a01b03601f5460081c16803b156107ff5781809160446040518094819363052eefd160e01b8352816004840152600160248401525af180156104f9576107d1575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b816119d99161361e565b6101ab57805f611934565b816119ee9161361e565b6101ab57805f6118a2565b50346101ab57806003193601126101ab576040516101008082019082821067ffffffffffffffff83111761057f579082916140ba8339039082f08015610bd8576001600160a01b0316906040519160ff928381019381851067ffffffffffffffff86111761057f57839482916152498339039083f080156104f9576001600160a01b0316906001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156113dd57604051906303223eab60e11b82526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156104d5578491611f0a575b50506001600160a01b03601f5460081c16803b156113dd5783809160446040518094819363052eefd160e01b8352876004840152600160248401525af19081156104d5578491611ef5575b50506001600160a01b03601f5460081c16803b156113dd5783809160446040518094819363052eefd160e01b83528860048401528160248401525af19081156104d5578491611ee0575b50506001600160a01b03601f5460081c16906040517f7a3979dc00000000000000000000000000000000000000000000000000000000815260208180611bc130600483016139da565b0381865afa8015611ed557611bdd918691611eb6575b50613e57565b813b156113dd5783916024839260405194859384927f04f386f400000000000000000000000000000000000000000000000000000000845260048401525af1908115610574578391611ea1575b50506001600160a01b03601f5460081c16803b156107e6578280916024604051809481937f04f386f40000000000000000000000000000000000000000000000000000000083528760048401525af1908115610574578391611e8c575b50506001600160a01b03601f5460081c1690813b156107e657829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156104f957611e77575b506040517f0200da4800000000000000000000000000000000000000000000000000000000602082015230602482015260406044820152611d188161166460648201613865565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff5781611d7391604051809381927ff28dceb300000000000000000000000000000000000000000000000000000000835260206004840152602483019061365f565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f957611e62575b50506001600160a01b03601f5460081c16602060405180927f7a3979dc0000000000000000000000000000000000000000000000000000000082528180611de230600483016139da565b03915afa80156104f957611e45575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b611e5d9060203d60201161179157611785818361361e565b611df1565b81611e6c9161361e565b6101ab57805f611d98565b81611e819161361e565b6101ab57805f611cd1565b81611e969161361e565b6107ff57815f611c87565b81611eab9161361e565b6107ff57815f611c2a565b611ecf915060203d60201161179157611785818361361e565b5f611bd7565b6040513d87823e3d90fd5b81611eea9161361e565b6107e657825f611b78565b81611eff9161361e565b6107e657825f611b2e565b81611f149161361e565b6107e657825f611ae3565b50346101ab57806003193601126101ab57601d54611f3c8161384d565b91611f4a604051938461361e565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310611f8c576040518061023f87826137b6565b60026020600192604051611f9f816135d5565b6001600160a01b038654168152611fb7858701613ad8565b83820152815201920192019190611f77565b50346101ab57806003193601126101ab5760405190610100918281019281841067ffffffffffffffff851117610be557829382916140ba8339039082f08015610bd8576001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657604051906303223eab60e11b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156105745783916121a4575b50506001600160a01b03601f5460081c1690813b156107e6576001600160a01b036044849283604051958694859363052eefd160e01b8552166004840152600160248401525af180156104f95761218f575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f95761217a575b50506001600160a01b03601f5460081c16602060405180927f7a3979dc000000000000000000000000000000000000000000000000000000008252818061216030600483016139da565b03915afa80156104f9576104aa918391611eb65750613e57565b816121849161361e565b6101ab57805f612116565b816121999161361e565b6101ab57805f6120c3565b816121ae9161361e565b6107ff57815f612071565b50346101ab57806003193601126101ab57601a546121d68161384d565b916121e4604051938461361e565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b838310612226576040518061023f8782613739565b60016020819260405161223d81610cc08189613922565b815201920192019190612211565b50346101ab57806003193601126101ab576040516101008082019082821067ffffffffffffffff83111761057f579082916140ba8339039082f0908115612478576040519160ff928381019381851067ffffffffffffffff86111761057f57839482916152498339039083f080156104f9576001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156113dd57604051906303223eab60e11b82526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156104d5578491612463575b50506001600160a01b03601f5460081c1690813b156113dd576001600160a01b036044859283604051958694859363052eefd160e01b8552166004840152600160248401525af19081156105745783916121a45750506001600160a01b03601f5460081c1690813b156107e6576001600160a01b036044849283604051958694859363052eefd160e01b8552166004840152600160248401525af180156104f95761218f575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f95761217a5750506001600160a01b03601f5460081c16602060405180927f7a3979dc000000000000000000000000000000000000000000000000000000008252818061216030600483016139da565b8161246d9161361e565b6107e657825f612322565b604051903d90823e3d90fd5b50346101ab57806003193601126101ab5760405190610100918281019281841067ffffffffffffffff851117610be557829382916140ba8339039082f08015610bd8576001600160a01b03166001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657604051906303223eab60e11b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561057457839161280d575b50506001600160a01b0360215416604051907f118cdaa700000000000000000000000000000000000000000000000000000000602083015260248201526024815261258160448261361e565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657826125dc91604051809381927ff28dceb300000000000000000000000000000000000000000000000000000000835260206004840152602483019061365f565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156105745783916127f8575b50506001600160a01b03601f5460081c16803b156107e65782809160446040518094819363052eefd160e01b8352876004840152600160248401525af19081156105745783916127e3575b50506001600160a01b0360215416604051907f118cdaa700000000000000000000000000000000000000000000000000000000602083015260248201526024815261269b60448261361e565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657826126f691604051809381927ff28dceb300000000000000000000000000000000000000000000000000000000835260206004840152602483019061365f565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156105745783916127ce575b50506001600160a01b03601f5460081c1690813b156107e65782916024839260405194859384927f04f386f400000000000000000000000000000000000000000000000000000000845260048401525af180156104f9576107d1575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b816127d89161361e565b6107ff57815f61271e565b816127ed9161361e565b6107ff57815f61264f565b816128029161361e565b6107ff57815f612604565b816128179161361e565b6107ff57815f612535565b50346101ab57806003193601126101ab57601b5461283f8161384d565b61284c604051918261361e565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b83831061292457868587604051928392602084019060208552518091526040840160408260051b8601019392905b8282106128b957505050500390f35b91936020612914827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0600195979984950301865288519083612904835160408452604084019061365f565b92015190848184039101526136e4565b96019201920185949391926128aa565b60026020600192604051612937816135d5565b60405161294881610cc0818a613922565b8152612955858701613ad8565b8382015281520192019201919061287c565b50346101ab57806003193601126101ab576040516101008082019082821067ffffffffffffffff83111761057f579082916140ba8339039082f08015610bd8576001600160a01b0316816001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561051957604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f957612b52575b506001600160a01b03601f5460081c16803b156105195781809160446040518094819363052eefd160e01b8352886004840152600160248401525af180156104f957612b3d575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561051957816040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f957612b28575b50906004916001600160a01b03601f5460081c16604051938480927f1b42c7110000000000000000000000000000000000000000000000000000000082525afa80156105745761049e6001600160a01b03916104aa948691612b1457506111f08151613f49565b61140b91503d8088833e6104c6818361361e565b81612b329161361e565b61051957815f612aad565b81612b479161361e565b61051957815f612a5a565b81612b5c9161361e565b61051957815f612a13565b50346101ab57806003193601126101ab576004816001600160a01b03601f5460081c16604051928380927f1b42c7110000000000000000000000000000000000000000000000000000000082525afa80156104f9576104aa9183916113ae575051613ed3565b50346101ab57806003193601126101ab5760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110612c2c5761023f856102338187038261361e565b82546001600160a01b0316845260209093019260019283019201612c15565b50346101ab57806003193601126101ab5760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b818110612caa5761023f856102338187038261361e565b82546001600160a01b0316845260209093019260019283019201612c93565b50346101ab57806003193601126101ab576001600160a01b03601f5460081c16602060405180927f7a3979dc000000000000000000000000000000000000000000000000000000008252818061216030600483016139da565b50346101ab57806003193601126101ab57601e54612d3f8161384d565b612d4c604051918261361e565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b838310612e8d5786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310612db85786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110612e4457505050505060208060019297019301930190928695949293612dab565b9091929394602080612e80837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08760019603018952895161365f565b9701950193929101612e20565b604051612e99816135d5565b6001600160a01b038354168152600183018054612eb58161384d565b91612ec3604051938461361e565b8183528a526020808b20908b9084015b838210612ef9575050505060019282602092836002950152815201920192019190612d7c565b600160208192604051612f1081610cc0818a613922565b815201930191019091612ed3565b50346101ab57806003193601126101ab5760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b818110612f7d5761023f856102338187038261361e565b82546001600160a01b0316845260209093019260019283019201612f66565b50346101ab57806003193601126101ab57604051908060225490612fbf82613584565b80855291600181169081156130585750600114612ffb575b61023f84612fe78186038261361e565b60405191829160208352602083019061365f565b602281527f61035b26e3e9eee00e0d72fd1ee8ddca6894550dca6916ea2ac6baa90d11e510939250905b80821061303e57509091508101602001612fe782612fd7565b919260018160209254838588010152019101909291613025565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208087019190915292151560051b85019092019250612fe79150839050612fd7565b50346101ab57806003193601126101ab57307fffffffffffffffffffffffff000000000000000000000000000000000000000060205416176020556104567fffffffffffffffffffffffff0000000000000000000000000000000000000000602154161760215560405161108f8082019082821067ffffffffffffffff83111761057f5760209183916141ba833930815203019082f08015610bd8577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f5580f35b50346101ab57806003193601126101ab5760405190610100918281019281841067ffffffffffffffff851117610be557829382916140ba8339039082f08015610bd8576001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657604051906303223eab60e11b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561057457839161338a575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff576040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f3d0f293d000000000000000000000000000000000000000000000000000000006004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610574578391613375575b50506001600160a01b03601f5460081c1690813b156107e6576001600160a01b03602484928360405195869485937f04f386f40000000000000000000000000000000000000000000000000000000085521660048401525af180156104f9576107d1575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b8161337f9161361e565b6107ff57815f6132bd565b816133949161361e565b6107ff57815f613229565b905034613580575f600319360112613580576001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15613580576303223eab60e11b825260048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561357557613562575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fe6c4247b000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f95761354d575b506001600160a01b03601f5460081c16803b156107ff578180916024604051809481937f04f386f40000000000000000000000000000000000000000000000000000000083528160048401525af180156104f9576107d1575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b816135579161361e565b6101ab57805f6134a0565b61356e91505f9061361e565b5f5f61340f565b6040513d5f823e3d90fd5b5f80fd5b90600182811c921680156135cb575b602083101461359e57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f1691613593565b6040810190811067ffffffffffffffff8211176135f157604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176135f157604052565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b60206040818301928281528451809452019201905f5b8181106136c55750505090565b82516001600160a01b03168452602093840193909201916001016136b8565b90602080835192838152019201905f5b8181106137015750505090565b82517fffffffff00000000000000000000000000000000000000000000000000000000168452602093840193909201916001016136f4565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061376b57505050505090565b90919293946020806137a7837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08660019603018752895161365f565b9701930193019193929061375c565b602081016020825282518091526040820191602060408360051b8301019401925f915b8383106137e857505050505090565b909192939460208061383e837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b038151168452015191818582015201906136e4565b970193019301919392906137d9565b67ffffffffffffffff81116135f15760051b60200190565b6022545f929161387482613584565b80825291600181169081156138e8575060011461388f575050565b60225f9081529293509091907f61035b26e3e9eee00e0d72fd1ee8ddca6894550dca6916ea2ac6baa90d11e5105b8383106138ce575060209250010190565b6001816020929493945483858701015201910191906138bd565b60209495507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091509291921683830152151560051b010190565b5f929181549161393183613584565b8083529260018116908115613986575060011461394d57505050565b5f9081526020812093945091925b83831061396c575060209250010190565b60018160209294939454838587010152019101919061395b565b905060209495507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091509291921683830152151560051b010190565b90816020910312613580575180151581036135805790565b6060906001600160a01b03613a00931681525f6020820152816040820152016022613922565b90565b6020818303126135805780519067ffffffffffffffff821161358057019080601f8301121561358057815190613a388261384d565b92613a46604051948561361e565b82845260208085019360051b82010191821161358057602001915b818310613a6e5750505090565b82516001600160a01b038116810361358057815260209283019201613a61565b805115613a9b5760200190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b805160011015613a9b5760400190565b90604051918281549182825260208201905f5260205f20925f905b806007830110613cf157613b49945491818110613cbb575b818110613c85575b818110613c4f575b818110613c19575b818110613be3575b818110613bad575b818110613b78575b10613b4b575b50038361361e565b565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f613b41565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b168152019301613b3b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301613b33565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b168152019301613b2b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301613b23565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b168152019301613b1b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b168152019301613b13565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b168152019301613b0b565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e0820152019401920185929391613af3565b60085460ff168015613d8d5790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115613575575f91613e25575b50151590565b90506020813d602011613e4f575b81613e406020938361361e565b8101031261358057515f613e1f565b3d9150613e33565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561358057604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561357557613ec95750565b5f613b499161361e565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561358057604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561357557613ec95750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561358057604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152600160248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561357557613ec95750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561358057604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152600260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561357557613ec95750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15613580576001600160a01b039081604051937f515361f60000000000000000000000000000000000000000000000000000000085521660048401521660248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561357557613ec9575056fe6080806040523460145760e790816100198239f35b5f80fdfe60808060405260043610156011575f80fd5b5f3560e01c637a3979dc146023575f80fd5b34609f5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112609f57605660a3565b50605d60c5565b5060443567ffffffffffffffff8111609f5736602382011215609f57806004013567ffffffffffffffff8111609f5736910160240111609f5780600160209252f35b5f80fd5b6004359073ffffffffffffffffffffffffffffffffffffffff82168203609f57565b6024359073ffffffffffffffffffffffffffffffffffffffff82168203609f575660803460b857601f61108f38819003918201601f19168301916001600160401b0383118484101760bc5780849260209460405283398101031260b857516001600160a01b0381169081900360b857801560a5575f80546001600160a01b031981168317825560405192916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a3610fbe90816100d18239f35b631e4fbdf760e01b5f525f60045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c806304f386f41461063c578063052eefd1146104bb5780631b42c7111461029f578063715018a6146102235780637a3979dc146101905780638da5cb5b1461015e578063a26b4a88146101435763f2fde38b14610071575f80fd5b3461013f57602060031936011261013f5773ffffffffffffffffffffffffffffffffffffffff61009f61075a565b6100a7610a3e565b1680156101135773ffffffffffffffffffffffffffffffffffffffff5f54827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f80fd5b3461013f575f60031936011261013f57602060405160288152f35b3461013f575f60031936011261013f57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b3461013f57606060031936011261013f576101a961075a565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361013f576044359067ffffffffffffffff821161013f573660238301121561013f5781600401359067ffffffffffffffff821161013f57366024838501011161013f576020936024610219940191610841565b6040519015158152f35b3461013f575f60031936011261013f5761023b610a3e565b5f73ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461013f575f60031936011261013f576001546102bb816107eb565b6102c8604051918261077d565b8181526102d4826107eb565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe060208201920136833760015f9081527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff165b8482108061049c575b15610492578251821015610465578073ffffffffffffffffffffffffffffffffffffffff6103a3921660208460051b86010152610d74565b901561040757907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81146103da5760010190610362565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b50509091505b604051918291602083019060208452518091526040830191905f5b818110610436575050500390f35b825173ffffffffffffffffffffffffffffffffffffffff16845285945060209384019390920191600101610428565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b505090915061040d565b5073ffffffffffffffffffffffffffffffffffffffff8116151561036b565b3461013f57604060031936011261013f576104d461075a565b60243590811515820361013f576104e9610a3e565b73ffffffffffffffffffffffffffffffffffffffff81169182156106145761051082610a8a565b6105ec57602860015410156105c457156105b65761052d90610ed5565b15610558577f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e75f80a2005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f41646472657373206e6f742061646465640000000000000000000000000000006044820152fd5b6105bf90610dd5565b61052d565b7f13d867a2000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fa2d86a1e000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe6c4247b000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461013f57602060031936011261013f5761065561075a565b61065d610a3e565b73ffffffffffffffffffffffffffffffffffffffff81169081156106145761068481610a8a565b156107325773ffffffffffffffffffffffffffffffffffffffff6106a88392610c5f565b16036106d4577fb5d68ca46372bbe6ec138d3d0423608269b3117496a46268f86080cdbcbea9be5f80a2005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601360248201527f41646472657373206e6f742072656d6f766564000000000000000000000000006044820152fd5b7f3d0f293d000000000000000000000000000000000000000000000000000000005f5260045ffd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361013f57565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176107be57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b67ffffffffffffffff81116107be5760051b60200190565b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b60015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d549394909373ffffffffffffffffffffffffffffffffffffffff169182156109cb57915b73ffffffffffffffffffffffffffffffffffffffff81168015610a1b57602060405180927f7a3979dc00000000000000000000000000000000000000000000000000000000825273ffffffffffffffffffffffffffffffffffffffff8916600483015273ffffffffffffffffffffffffffffffffffffffff87166024830152606060448301528180610944606482018d8c610803565b03915afa908115610a10575f916109d6575b506109cb5761096490610d74565b906108ae575050506109c79073ffffffffffffffffffffffffffffffffffffffff935b6040519485947f0200da48000000000000000000000000000000000000000000000000000000008652166004850152604060248501526044840191610803565b0390fd5b509350505050600190565b90506020813d8211610a08575b816109f06020938361077d565b8101031261013f5751801515810361013f575f610956565b3d91506109e3565b6040513d5f823e3d90fd5b505050506109c79073ffffffffffffffffffffffffffffffffffffffff93610987565b73ffffffffffffffffffffffffffffffffffffffff5f54163303610a5e57565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff16805f52600260205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f2054161580610b4d575b15610b475760015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff1603610b4357600190565b5f90565b50600190565b50805f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615610ad4565b60010173ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f2054161580610c15575b15610c0e575f805260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2054169116145f14610b4357600190565b5050600190565b5073ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615610bce565b73ffffffffffffffffffffffffffffffffffffffff811680158015610d62575b610d5c575f90815260026020818152604080842084805280835281852080546001808852848820805473ffffffffffffffffffffffffffffffffffffffff908116808b52898952878b208b80528952878b208054929095167fffffffffffffffffffffffff00000000000000000000000000000000000000009283168117909555938a52978752858920828a5287529490972080548716909117905580548516905590915280549091169055547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116103da5760015590565b50505f90565b50610d6e826001610b82565b15610c7f565b610d7f816001610b82565b610d8a57505f905f90565b73ffffffffffffffffffffffffffffffffffffffff165f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f205416908115159190565b610de0816001610b82565b1580610ec4575b610df057505f90565b7f6ee3efecae883df2d7ccda22610b4ca771a299e707cb0d65c4ec97dc4e6668ad805473ffffffffffffffffffffffffffffffffffffffff9283165f818152600260208181526040808420600180865281845282862080547fffffffffffffffffffffffff000000000000000000000000000000000000000090811690915589548116881790995598909616808552928252808420978452968152868320805487169094179093558180529290915292909220805490911690911790555b600154600181018091116103da57600155600190565b50610ed05f6001610b82565b610de7565b610ee0816001610b82565b1580610fad575b610ef057505f90565b7f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d805473ffffffffffffffffffffffffffffffffffffffff9283165f81815260026020818152604080842084805280835281852080547fffffffffffffffffffffffff00000000000000000000000000000000000000009081169091558854811687179098559790951680845291815284832083805281528483208054871690941790935560018252949091522080549091169091179055610eae565b50610fb95f6001610b82565b610ee7566080806040523460145760e690816100198239f35b5f80fdfe60808060405260043610156011575f80fd5b5f3560e01c637a3979dc146023575f80fd5b34609e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112609e57605660a2565b50605d60c4565b5060443567ffffffffffffffff8111609e5736602382011215609e57806004013567ffffffffffffffff8111609e5736910160240111609e57805f60209252f35b5f80fd5b6004359073ffffffffffffffffffffffffffffffffffffffff82168203609e57565b6024359073ffffffffffffffffffffffffffffffffffffffff82168203609e5756
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4a\x01\xA2W`\x0C\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x1F\x80T\x90\x91\x16\x90\x91\x17\x90U`@Q` \x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x01\x8EW`@R_\x82R`\x1F\x19\x81\x016\x82\x84\x017\x81Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\x8EW`\"T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x01\x84W[\x82\x82\x10\x14a\x01pW`\x1F\x81\x11a\x01,W[P\x80`\x1F\x83\x11`\x01\x14a\0\xC9WP\x81\x92_\x92a\0\xBEW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\"U[`@QaSH\x90\x81a\x01\xA7\x829\xF3[\x01Q\x90P_\x80a\0\x9AV[\x90`\x1F\x19\x83\x16\x93`\"_R\x82_ \x92_\x90[\x86\x82\x10a\x01\x14WPP\x83`\x01\x95\x10a\0\xFCW[PPP\x81\x1B\x01`\"Ua\0\xAFV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\0\xEEV[\x80`\x01\x85\x96\x82\x94\x96\x86\x01Q\x81U\x01\x95\x01\x93\x01\x90a\0\xDBV[`\"_R\x81_ `\x1F\x84\x01`\x05\x1C\x81\x01\x91\x83\x85\x10a\x01fW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x01[WPa\0\x83V[_\x81U`\x01\x01a\x01NV[\x90\x91P\x81\x90a\x01EV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x7F\x16\x90a\0rV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x01\xF8\xD4\xDF\x14a3\x9FWP\x80c\x08\"n\x93\x14a1\x81W\x80c\n\x92T\xE4\x14a0\x9DW\x80c\x17!\xDA\xAA\x14a/\x9CW\x80c\x1E\xD7\x83\x1C\x14a/\x1EW\x80c*\xDE8\x80\x14a-\"W\x80c4\xD9#\xDB\x14a,\xC9W\x80c>^<#\x14a,KW\x80c?r\x86\xF4\x14a+\xCDW\x80cA\xE2\xAC-\x14a+gW\x80cW\xB5\n\xCC\x14a)gW\x80cf\xD9\xA9\xA0\x14a(\"W\x80cnCZ[\x14a$\x84W\x80cuh\x96q\x14a\"KW\x80c\x85\"l\x81\x14a!\xB9W\x80c\x8A6\x03\xA3\x14a\x1F\xC9W\x80c\x91j\x17\xC6\x14a\x1F\x1FW\x80c\x91\xD0\x1Ca\x14a\x19\xF9W\x80c\x9B\xD8\xBE\xD7\x14a\x18.W\x80c\x9Db\xF5\xAB\x14a\x14PW\x80c\xA0\x97M\x87\x14a\x0F\xD5W\x80c\xA5\x08\x1Ds\x14a\r\x7FW\x80c\xB0FO\xDC\x14a\x0C\xD5W\x80c\xB5P\x8A\xA9\x14a\x0C<W\x80c\xB8mR\x98\x14a\x0C\x12W\x80c\xB9\x1D\xC4\x99\x14a\tkW\x80c\xBAAO\xA6\x14a\tFW\x80c\xC1.F\x87\x14a\t\x1FW\x80c\xCD\xC3\xE0\x10\x14a\x05\xACW\x80c\xDB[\xFB\r\x14a\x02bW\x80c\xE2\x0C\x9Fq\x14a\x01\xD4W\x80c\xF8Q\xA4@\x14a\x01\xAEWc\xFAv&\xD4\x14a\x01\x89W_\x80\xFD[4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x02CWa\x02?\x85a\x023\x81\x87\x03\x82a6\x1EV[`@Q\x91\x82\x91\x82a6\xA2V[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x02\x1CV[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Qa\x01\0\x80\x82\x01\x91\x80\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x05\x7FW\x80a@\xBA\x93\x83\x85\x839\x03\x90\x84\xF0\x80\x15a\x05tW`\x01`\x01`\xA0\x1B\x03\x16\x91`@Q\x91\x80\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x05GW\x90\x83\x92\x91\x839\x03\x90\x83\xF0\x80\x15a\x04\xF9W`\x01`\x01`\xA0\x1B\x03\x16\x90\x82`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x19W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x052W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x05\x19W\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x88`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x05\x1DW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x05\x19W\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x89`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x05\x04W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xF5W\x82`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x04\xE0W[P\x91`\x04\x92`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x80\x92\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x92\x83\x15a\x04\xD5Wa\x04\xAA\x93`\x01`\x01`\xA0\x1B\x03\x92a\x04\x9E\x92\x87\x92a\x04\xADW[Pa\x04\xA5\x90a\x04\x94\x83Qa?\xC0V[\x84a\x04\x9E\x84a:\x8EV[Q\x16a@7V[a:\xC8V[\x80\xF3[a\x04\xA5\x91\x92Pa\x04\xCE\x90=\x80\x8A\x83>a\x04\xC6\x81\x83a6\x1EV[\x81\x01\x90a:\x03V[\x91\x90a\x04\x85V[`@Q=\x86\x82>=\x90\xFD[\x81a\x04\xEA\x91a6\x1EV[a\x04\xF5W\x82_a\x04&V[\x82\x80\xFD[`@Q=\x84\x82>=\x90\xFD[\x81a\x05\x0E\x91a6\x1EV[a\x04\xF5W\x82_a\x03\xD3V[P\x80\xFD[\x81a\x05'\x91a6\x1EV[a\x04\xF5W\x82_a\x03\x8CV[\x81a\x05<\x91a6\x1EV[a\x04\xF5W\x82_a\x03EV[`$\x86\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[`@Q=\x85\x82>=\x90\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\t\nW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xA2kJ\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04\xF9W\x82\x91a\x08\xD8W[P\x81[\x81\x81\x10a\x08/WP`\x01\x81\x01\x80\x91\x11a\x08\x02W\x81\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x13\xD8g\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a\x07\xEAW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W`\x01`\x01`\xA0\x1B\x03`D\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x07\xD1W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[\x81a\x07\xCA\x91a6\x1EV[a\x01\xABW\x80\xF3[\x81a\x07\xDB\x91a6\x1EV[a\x01\xABW\x80_a\x07kV[PP\xFD[\x81a\x07\xF4\x91a6\x1EV[a\x07\xFFW\x81_a\x07\x19V[P\xFD[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[`\x01\x81\x01\x80\x82\x11a\x08\xABW\x83\x90`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x04\xF5W`\x01`\x01`\xA0\x1B\x03`D\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x08\x96W[PP`\x01\x01a\x06rV[\x81a\x08\xA0\x91a6\x1EV[a\x04\xF5W\x82_a\x08\x8CV[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11a\t\x02W[\x81a\x08\xF3` \x93\x83a6\x1EV[\x81\x01\x03\x12a\x05\x19WQ_a\x06oV[=\x91Pa\x08\xE6V[\x81a\t\x14\x91a6\x1EV[a\x01\xABW\x80_a\x06 V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW` a\taa=~V[`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90a\x01\0\x91\x82\x81\x01\x92\x81\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x0B\xE5W\x82\x93\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a\x0B\xC3W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07\xE6W\x82\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x87`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a\x0B\xAEW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xA2\xD8j\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a\x0B\x99W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x07\xD1WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[\x81a\x0B\xA3\x91a6\x1EV[a\x07\xFFW\x81_a\n\xFBV[\x81a\x0B\xB8\x91a6\x1EV[a\x07\xFFW\x81_a\ngV[\x81a\x0B\xCD\x91a6\x1EV[a\x07\xFFW\x81_a\n\x1CV[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x83\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x19Ta\x0CY\x81a8MV[\x91a\x0Cg`@Q\x93\x84a6\x1EV[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\x0C\xA9W`@Q\x80a\x02?\x87\x82a79V[`\x01` \x81\x92`@Qa\x0C\xC7\x81a\x0C\xC0\x81\x89a9\"V[\x03\x82a6\x1EV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x0C\x94V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x1CTa\x0C\xF2\x81a8MV[\x91a\r\0`@Q\x93\x84a6\x1EV[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\rBW`@Q\x80a\x02?\x87\x82a7\xB6V[`\x02` `\x01\x92`@Qa\rU\x81a5\xD5V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\rm\x85\x87\x01a:\xD8V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\r-V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Qa\x01\0\x80\x82\x01\x91\x80\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x05\x7FW\x80a@\xBA\x93\x83\x85\x839\x03\x90\x84\xF0\x80\x15a\x05tW`\x01`\x01`\xA0\x1B\x03\x16\x91`@Q\x91\x80\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x05GW\x90\x83\x92\x91\x839\x03\x90\x83\xF0\x80\x15a\x04\xF9W`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x19W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x0F\xC0W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x05\x19W\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x89`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x0F\xABW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x05\x19W\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x88`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x05\x04WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xF5W\x82`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x04\xE0WP\x91`\x04\x92`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x80\x92\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x92\x83\x15a\x04\xD5Wa\x04\xAA\x93`\x01`\x01`\xA0\x1B\x03\x92a\x04\x9E\x92\x87\x92a\x04\xADWPa\x04\xA5\x90a\x04\x94\x83Qa?\xC0V[\x81a\x0F\xB5\x91a6\x1EV[a\x04\xF5W\x82_a\x0E\xA7V[\x81a\x0F\xCA\x91a6\x1EV[a\x04\xF5W\x82_a\x0EaV[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90a\x01\0\x91\x82\x81\x01\x92\x81\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x0B\xE5W\x82\x93\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a\x14;W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81\x80a\x10\xF1`\x04\x82\x01\x90_```\x80\x84\x01\x93`\x01\x81R`\x01` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a\x14&W[PP`@Q\x81\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7\x84\x80\xA2`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x13\xDDW\x81`D\x81\x86\x80\x94c\x05.\xEF\xD1`\xE0\x1B\x83R\x87`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a\x14\x11W[P`\x04\x90`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05tW`\x01`\x01`\xA0\x1B\x03a\x04\x9Ea\x11\xF5\x93\x85\x93\x87\x91a\x13\xF7W[Pa\x11\xF0\x81Qa?IV[a:\x8EV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81\x80a\x12^`\x04\x82\x01\x90_```\x80\x84\x01\x93`\x01\x81R`\x01` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a\x13\xE2W[PP`@Q\x90\x80\x7F\xB5\xD6\x8C\xA4cr\xBB\xE6\xEC\x13\x8D=\x04#`\x82i\xB3\x11t\x96\xA4bh\xF8`\x80\xCD\xBC\xBE\xA9\xBE\x84\x80\xA2`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x13\xDDW`$\x83\x85\x81\x93\x81\x95\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x13\xC8W[P`\x04\x90`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x04\xF9Wa\x13[\x91\x83\x91a\x13\xAEW[PQa>\xD3V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[a\x13\xC2\x91P=\x80\x85\x83>a\x04\xC6\x81\x83a6\x1EV[_a\x13TV[\x81a\x13\xD2\x91a6\x1EV[a\x01\xABW\x80_a\x13\x04V[PPP\xFD[\x81a\x13\xEC\x91a6\x1EV[a\x07\xFFW\x81_a\x12\x86V[a\x14\x0B\x91P=\x80\x89\x83>a\x04\xC6\x81\x83a6\x1EV[_a\x11\xE5V[\x81a\x14\x1B\x91a6\x1EV[a\x07\xFFW\x81_a\x11\x87V[\x81a\x140\x91a6\x1EV[a\x07\xFFW\x81_a\x11\x19V[\x81a\x14E\x91a6\x1EV[a\x07\xFFW\x81_a\x10\x86V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90`\xFF\x80\x83\x01\x92\x80\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x0B\xE5W\x80aRI\x94\x83\x86\x839\x03\x90\x83\xF0\x90\x81\x15a\x04\xF9W`@Q\x90\x80\x82\x01\x94\x82\x86\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x11\x17a\x18\x01W\x94\x82\x91\x85\x96\x839\x03\x90\x83\xF0\x90\x81\x15a\x04\xF9W`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13\xDDW`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04\xD5W\x84\x91a\x17\xECW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x13\xDDW`\x01`\x01`\xA0\x1B\x03`D\x85\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a\x17\xD7W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W`\x01`\x01`\xA0\x1B\x03`D\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x17\xC2W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x17\xADW[P`@Q\x7F\x02\0\xDAH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R0`$\x82\x01R`@`D\x82\x01Ra\x16\x90\x81a\x16d`d\x82\x01a8eV[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a6\x1EV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW\x81a\x16\xEB\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6_V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x17\x98W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x80a\x17Z0`\x04\x83\x01a9\xDAV[\x03\x91Z\xFA\x80\x15a\x04\xF9Wa\x17lWP\x80\xF3[a\x17\x8D\x90` =` \x11a\x17\x91W[a\x17\x85\x81\x83a6\x1EV[\x81\x01\x90a9\xC2V[P\x80\xF3[P=a\x17{V[\x81a\x17\xA2\x91a6\x1EV[a\x01\xABW\x80_a\x17\x10V[\x81a\x17\xB7\x91a6\x1EV[a\x01\xABW\x80_a\x16\x1DV[\x81a\x17\xCC\x91a6\x1EV[a\x01\xABW\x80_a\x15\xCAV[\x81a\x17\xE1\x91a6\x1EV[a\x07\xFFW\x81_a\x15yV[\x81a\x17\xF6\x91a6\x1EV[a\x07\xE6W\x82_a\x15$V[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x19\xE4W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x19\xCFW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07\xFFW\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x81`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x07\xD1WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[\x81a\x19\xD9\x91a6\x1EV[a\x01\xABW\x80_a\x194V[\x81a\x19\xEE\x91a6\x1EV[a\x01\xABW\x80_a\x18\xA2V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Qa\x01\0\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x05\x7FW\x90\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03\x16\x90`@Q\x91`\xFF\x92\x83\x81\x01\x93\x81\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x05\x7FW\x83\x94\x82\x91aRI\x839\x03\x90\x83\xF0\x80\x15a\x04\xF9W`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13\xDDW`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04\xD5W\x84\x91a\x1F\nW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x13\xDDW\x83\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x87`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x90\x81\x15a\x04\xD5W\x84\x91a\x1E\xF5W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x13\xDDW\x83\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x88`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x90\x81\x15a\x04\xD5W\x84\x91a\x1E\xE0W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`@Q\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81\x80a\x1B\xC10`\x04\x83\x01a9\xDAV[\x03\x81\x86Z\xFA\x80\x15a\x1E\xD5Wa\x1B\xDD\x91\x86\x91a\x1E\xB6W[Pa>WV[\x81;\x15a\x13\xDDW\x83\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a\x1E\xA1W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07\xE6W\x82\x80\x91`$`@Q\x80\x94\x81\x93\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x87`\x04\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a\x1E\x8CW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x1EwW[P`@Q\x7F\x02\0\xDAH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R0`$\x82\x01R`@`D\x82\x01Ra\x1D\x18\x81a\x16d`d\x82\x01a8eV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW\x81a\x1Ds\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6_V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x1EbW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x80a\x1D\xE20`\x04\x83\x01a9\xDAV[\x03\x91Z\xFA\x80\x15a\x04\xF9Wa\x1EEW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[a\x1E]\x90` =` \x11a\x17\x91Wa\x17\x85\x81\x83a6\x1EV[a\x1D\xF1V[\x81a\x1El\x91a6\x1EV[a\x01\xABW\x80_a\x1D\x98V[\x81a\x1E\x81\x91a6\x1EV[a\x01\xABW\x80_a\x1C\xD1V[\x81a\x1E\x96\x91a6\x1EV[a\x07\xFFW\x81_a\x1C\x87V[\x81a\x1E\xAB\x91a6\x1EV[a\x07\xFFW\x81_a\x1C*V[a\x1E\xCF\x91P` =` \x11a\x17\x91Wa\x17\x85\x81\x83a6\x1EV[_a\x1B\xD7V[`@Q=\x87\x82>=\x90\xFD[\x81a\x1E\xEA\x91a6\x1EV[a\x07\xE6W\x82_a\x1BxV[\x81a\x1E\xFF\x91a6\x1EV[a\x07\xE6W\x82_a\x1B.V[\x81a\x1F\x14\x91a6\x1EV[a\x07\xE6W\x82_a\x1A\xE3V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x1DTa\x1F<\x81a8MV[\x91a\x1FJ`@Q\x93\x84a6\x1EV[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x1F\x8CW`@Q\x80a\x02?\x87\x82a7\xB6V[`\x02` `\x01\x92`@Qa\x1F\x9F\x81a5\xD5V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x1F\xB7\x85\x87\x01a:\xD8V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x1FwV[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90a\x01\0\x91\x82\x81\x01\x92\x81\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x0B\xE5W\x82\x93\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a!\xA4W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W`\x01`\x01`\xA0\x1B\x03`D\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa!\x8FW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa!zW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x80a!`0`\x04\x83\x01a9\xDAV[\x03\x91Z\xFA\x80\x15a\x04\xF9Wa\x04\xAA\x91\x83\x91a\x1E\xB6WPa>WV[\x81a!\x84\x91a6\x1EV[a\x01\xABW\x80_a!\x16V[\x81a!\x99\x91a6\x1EV[a\x01\xABW\x80_a \xC3V[\x81a!\xAE\x91a6\x1EV[a\x07\xFFW\x81_a qV[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x1ATa!\xD6\x81a8MV[\x91a!\xE4`@Q\x93\x84a6\x1EV[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\"&W`@Q\x80a\x02?\x87\x82a79V[`\x01` \x81\x92`@Qa\"=\x81a\x0C\xC0\x81\x89a9\"V[\x81R\x01\x92\x01\x92\x01\x91\x90a\"\x11V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Qa\x01\0\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x05\x7FW\x90\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x90\x81\x15a$xW`@Q\x91`\xFF\x92\x83\x81\x01\x93\x81\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x05\x7FW\x83\x94\x82\x91aRI\x839\x03\x90\x83\xF0\x80\x15a\x04\xF9W`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13\xDDW`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04\xD5W\x84\x91a$cW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x13\xDDW`\x01`\x01`\xA0\x1B\x03`D\x85\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a!\xA4WPP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W`\x01`\x01`\xA0\x1B\x03`D\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa!\x8FWPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa!zWPP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x80a!`0`\x04\x83\x01a9\xDAV[\x81a$m\x91a6\x1EV[a\x07\xE6W\x82_a#\"V[`@Q\x90=\x90\x82>=\x90\xFD[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90a\x01\0\x91\x82\x81\x01\x92\x81\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x0B\xE5W\x82\x93\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a(\rW[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01R`$\x81Ra%\x81`D\x82a6\x1EV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W\x82a%\xDC\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6_V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a'\xF8W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07\xE6W\x82\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x87`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a'\xE3W[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01R`$\x81Ra&\x9B`D\x82a6\x1EV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W\x82a&\xF6\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6_V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a'\xCEW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x07\xD1WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[\x81a'\xD8\x91a6\x1EV[a\x07\xFFW\x81_a'\x1EV[\x81a'\xED\x91a6\x1EV[a\x07\xFFW\x81_a&OV[\x81a(\x02\x91a6\x1EV[a\x07\xFFW\x81_a&\x04V[\x81a(\x17\x91a6\x1EV[a\x07\xFFW\x81_a%5V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x1BTa(?\x81a8MV[a(L`@Q\x91\x82a6\x1EV[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a)$W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a(\xB9WPPPP\x03\x90\xF3[\x91\x93` a)\x14\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a)\x04\x83Q`@\x84R`@\x84\x01\x90a6_V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra6\xE4V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a(\xAAV[`\x02` `\x01\x92`@Qa)7\x81a5\xD5V[`@Qa)H\x81a\x0C\xC0\x81\x8Aa9\"V[\x81Ra)U\x85\x87\x01a:\xD8V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a(|V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Qa\x01\0\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x05\x7FW\x90\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x19W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa+RW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x05\x19W\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x88`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa+=W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x19W\x81`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa+(W[P\x90`\x04\x91`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05tWa\x04\x9E`\x01`\x01`\xA0\x1B\x03\x91a\x04\xAA\x94\x86\x91a+\x14WPa\x11\xF0\x81Qa?IV[a\x14\x0B\x91P=\x80\x88\x83>a\x04\xC6\x81\x83a6\x1EV[\x81a+2\x91a6\x1EV[a\x05\x19W\x81_a*\xADV[\x81a+G\x91a6\x1EV[a\x05\x19W\x81_a*ZV[\x81a+\\\x91a6\x1EV[a\x05\x19W\x81_a*\x13V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x04\x81`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x04\xF9Wa\x04\xAA\x91\x83\x91a\x13\xAEWPQa>\xD3V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a,,Wa\x02?\x85a\x023\x81\x87\x03\x82a6\x1EV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a,\x15V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a,\xAAWa\x02?\x85a\x023\x81\x87\x03\x82a6\x1EV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a,\x93V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x80a!`0`\x04\x83\x01a9\xDAV[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x1ETa-?\x81a8MV[a-L`@Q\x91\x82a6\x1EV[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a.\x8DW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a-\xB8W\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a.DWPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a-\xABV[\x90\x91\x92\x93\x94` \x80a.\x80\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa6_V[\x97\x01\x95\x01\x93\x92\x91\x01a. V[`@Qa.\x99\x81a5\xD5V[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta.\xB5\x81a8MV[\x91a.\xC3`@Q\x93\x84a6\x1EV[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a.\xF9WPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a-|V[`\x01` \x81\x92`@Qa/\x10\x81a\x0C\xC0\x81\x8Aa9\"V[\x81R\x01\x93\x01\x91\x01\x90\x91a.\xD3V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a/}Wa\x02?\x85a\x023\x81\x87\x03\x82a6\x1EV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a/fV[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90\x80`\"T\x90a/\xBF\x82a5\x84V[\x80\x85R\x91`\x01\x81\x16\x90\x81\x15a0XWP`\x01\x14a/\xFBW[a\x02?\x84a/\xE7\x81\x86\x03\x82a6\x1EV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a6_V[`\"\x81R\x7Fa\x03[&\xE3\xE9\xEE\xE0\x0E\rr\xFD\x1E\xE8\xDD\xCAh\x94U\r\xCAi\x16\xEA*\xC6\xBA\xA9\r\x11\xE5\x10\x93\x92P\x90[\x80\x82\x10a0>WP\x90\x91P\x81\x01` \x01a/\xE7\x82a/\xD7V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a0%V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x80\x87\x01\x91\x90\x91R\x92\x15\x15`\x05\x1B\x85\x01\x90\x92\x01\x92Pa/\xE7\x91P\x83\x90Pa/\xD7V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` Ua\x04V\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`@Qa\x10\x8F\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x05\x7FW` \x91\x83\x91aA\xBA\x8390\x81R\x03\x01\x90\x82\xF0\x80\x15a\x0B\xD8W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU\x80\xF3[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90a\x01\0\x91\x82\x81\x01\x92\x81\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x0B\xE5W\x82\x93\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a3\x8AW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a3uW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W`\x01`\x01`\xA0\x1B\x03`$\x84\x92\x83`@Q\x95\x86\x94\x85\x93\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x07\xD1WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[\x81a3\x7F\x91a6\x1EV[a\x07\xFFW\x81_a2\xBDV[\x81a3\x94\x91a6\x1EV[a\x07\xFFW\x81_a2)V[\x90P4a5\x80W_`\x03\x196\x01\x12a5\x80W`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a5\x80Wc\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a5uWa5bW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa5MW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07\xFFW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x07\xD1WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[\x81a5W\x91a6\x1EV[a\x01\xABW\x80_a4\xA0V[a5n\x91P_\x90a6\x1EV[__a4\x0FV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a5\xCBW[` \x83\x10\x14a5\x9EWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a5\x93V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a5\xF1W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a5\xF1W`@RV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a6\xC5WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a6\xB8V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a7\x01WPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a6\xF4V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a7kWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a7\xA7\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa6_V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a7\\V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a7\xE8WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a8>\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a6\xE4V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a7\xD9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a5\xF1W`\x05\x1B` \x01\x90V[`\"T_\x92\x91a8t\x82a5\x84V[\x80\x82R\x91`\x01\x81\x16\x90\x81\x15a8\xE8WP`\x01\x14a8\x8FWPPV[`\"_\x90\x81R\x92\x93P\x90\x91\x90\x7Fa\x03[&\xE3\xE9\xEE\xE0\x0E\rr\xFD\x1E\xE8\xDD\xCAh\x94U\r\xCAi\x16\xEA*\xC6\xBA\xA9\r\x11\xE5\x10[\x83\x83\x10a8\xCEWP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a8\xBDV[` \x94\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x92\x91\x92\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[_\x92\x91\x81T\x91a91\x83a5\x84V[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a9\x86WP`\x01\x14a9MWPPPV[_\x90\x81R` \x81 \x93\x94P\x91\x92[\x83\x83\x10a9lWP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a9[V[\x90P` \x94\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x92\x91\x92\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a5\x80WQ\x80\x15\x15\x81\x03a5\x80W\x90V[``\x90`\x01`\x01`\xA0\x1B\x03a:\0\x93\x16\x81R_` \x82\x01R\x81`@\x82\x01R\x01`\"a9\"V[\x90V[` \x81\x83\x03\x12a5\x80W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a5\x80W\x01\x90\x80`\x1F\x83\x01\x12\x15a5\x80W\x81Q\x90a:8\x82a8MV[\x92a:F`@Q\x94\x85a6\x1EV[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x91\x82\x11a5\x80W` \x01\x91[\x81\x83\x10a:nWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a5\x80W\x81R` \x92\x83\x01\x92\x01a:aV[\x80Q\x15a:\x9BW` \x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80Q`\x01\x10\x15a:\x9BW`@\x01\x90V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10a<\xF1Wa;I\x94T\x91\x81\x81\x10a<\xBBW[\x81\x81\x10a<\x85W[\x81\x81\x10a<OW[\x81\x81\x10a<\x19W[\x81\x81\x10a;\xE3W[\x81\x81\x10a;\xADW[\x81\x81\x10a;xW[\x10a;KW[P\x03\x83a6\x1EV[V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_a;AV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01a;;V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01a;3V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01a;+V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01a;#V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01a;\x1BV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01a;\x13V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01a;\x0BV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91a:\xF3V[`\x08T`\xFF\x16\x80\x15a=\x8DW\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a5uW_\x91a>%W[P\x15\x15\x90V[\x90P` \x81=` \x11a>OW[\x81a>@` \x93\x83a6\x1EV[\x81\x01\x03\x12a5\x80WQ_a>\x1FV[=\x91Pa>3V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a5\x80W`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a5uWa>\xC9WPV[_a;I\x91a6\x1EV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a5\x80W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a5uWa>\xC9WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a5\x80W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x01`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a5uWa>\xC9WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a5\x80W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x02`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a5uWa>\xC9WPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a5\x80W`\x01`\x01`\xA0\x1B\x03\x90\x81`@Q\x93\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R\x16`\x04\x84\x01R\x16`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a5uWa>\xC9WPV\xFE`\x80\x80`@R4`\x14W`\xE7\x90\x81a\0\x19\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15`\x11W_\x80\xFD[_5`\xE0\x1Ccz9y\xDC\x14`#W_\x80\xFD[4`\x9FW``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12`\x9FW`V`\xA3V[P`]`\xC5V[P`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x9FW6`#\x82\x01\x12\x15`\x9FW\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x9FW6\x91\x01`$\x01\x11`\x9FW\x80`\x01` \x92R\xF3[_\x80\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03`\x9FWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03`\x9FWV`\x804`\xB8W`\x1Fa\x10\x8F8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`\xBCW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`\xB8WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03`\xB8W\x80\x15`\xA5W_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x82U`@Q\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3a\x0F\xBE\x90\x81a\0\xD1\x829\xF3[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x04\xF3\x86\xF4\x14a\x06<W\x80c\x05.\xEF\xD1\x14a\x04\xBBW\x80c\x1BB\xC7\x11\x14a\x02\x9FW\x80cqP\x18\xA6\x14a\x02#W\x80cz9y\xDC\x14a\x01\x90W\x80c\x8D\xA5\xCB[\x14a\x01^W\x80c\xA2kJ\x88\x14a\x01CWc\xF2\xFD\xE3\x8B\x14a\0qW_\x80\xFD[4a\x01?W` `\x03\x196\x01\x12a\x01?Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\x9Fa\x07ZV[a\0\xA7a\n>V[\x16\x80\x15a\x01\x13Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x80\xFD[4a\x01?W_`\x03\x196\x01\x12a\x01?W` `@Q`(\x81R\xF3[4a\x01?W_`\x03\x196\x01\x12a\x01?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\x01?W```\x03\x196\x01\x12a\x01?Wa\x01\xA9a\x07ZV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01?W`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01?W6`#\x83\x01\x12\x15a\x01?W\x81`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01?W6`$\x83\x85\x01\x01\x11a\x01?W` \x93`$a\x02\x19\x94\x01\x91a\x08AV[`@Q\x90\x15\x15\x81R\xF3[4a\x01?W_`\x03\x196\x01\x12a\x01?Wa\x02;a\n>V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01?W_`\x03\x196\x01\x12a\x01?W`\x01Ta\x02\xBB\x81a\x07\xEBV[a\x02\xC8`@Q\x91\x82a\x07}V[\x81\x81Ra\x02\xD4\x82a\x07\xEBV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0` \x82\x01\x92\x016\x837`\x01_\x90\x81R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x84\x82\x10\x80a\x04\x9CW[\x15a\x04\x92W\x82Q\x82\x10\x15a\x04eW\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x03\xA3\x92\x16` \x84`\x05\x1B\x86\x01\x01Ra\rtV[\x90\x15a\x04\x07W\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x03\xDAW`\x01\x01\x90a\x03bV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[PP\x90\x91P[`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x046WPPP\x03\x90\xF3[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x04(V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[PP\x90\x91Pa\x04\rV[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15\x15a\x03kV[4a\x01?W`@`\x03\x196\x01\x12a\x01?Wa\x04\xD4a\x07ZV[`$5\x90\x81\x15\x15\x82\x03a\x01?Wa\x04\xE9a\n>V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x15a\x06\x14Wa\x05\x10\x82a\n\x8AV[a\x05\xECW`(`\x01T\x10\x15a\x05\xC4W\x15a\x05\xB6Wa\x05-\x90a\x0E\xD5V[\x15a\x05XW\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7_\x80\xA2\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FAddress not added\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[a\x05\xBF\x90a\r\xD5V[a\x05-V[\x7F\x13\xD8g\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xA2\xD8j\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01?W` `\x03\x196\x01\x12a\x01?Wa\x06Ua\x07ZV[a\x06]a\n>V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a\x06\x14Wa\x06\x84\x81a\n\x8AV[\x15a\x072Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x06\xA8\x83\x92a\x0C_V[\x16\x03a\x06\xD4W\x7F\xB5\xD6\x8C\xA4cr\xBB\xE6\xEC\x13\x8D=\x04#`\x82i\xB3\x11t\x96\xA4bh\xF8`\x80\xCD\xBC\xBE\xA9\xBE_\x80\xA2\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FAddress not removed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01?WV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\xBEW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\xBEW`\x05\x1B` \x01\x90V[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DT\x93\x94\x90\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x82\x15a\t\xCBW\x91[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15a\n\x1BW` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16`\x04\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`$\x83\x01R```D\x83\x01R\x81\x80a\tD`d\x82\x01\x8D\x8Ca\x08\x03V[\x03\x91Z\xFA\x90\x81\x15a\n\x10W_\x91a\t\xD6W[Pa\t\xCBWa\td\x90a\rtV[\x90a\x08\xAEWPPPa\t\xC7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93[`@Q\x94\x85\x94\x7F\x02\0\xDAH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x16`\x04\x85\x01R`@`$\x85\x01R`D\x84\x01\x91a\x08\x03V[\x03\x90\xFD[P\x93PPPP`\x01\x90V[\x90P` \x81=\x82\x11a\n\x08W[\x81a\t\xF0` \x93\x83a\x07}V[\x81\x01\x03\x12a\x01?WQ\x80\x15\x15\x81\x03a\x01?W_a\tVV[=\x91Pa\t\xE3V[`@Q=_\x82>=\x90\xFD[PPPPa\t\xC7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93a\t\x87V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\n^WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80_R`\x02` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x0BMW[\x15a\x0BGW`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0BCW`\x01\x90V[_\x90V[P`\x01\x90V[P\x80_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\n\xD4V[`\x01\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x0C\x15W[\x15a\x0C\x0EW_\x80R` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ T\x16\x91\x16\x14_\x14a\x0BCW`\x01\x90V[PP`\x01\x90V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\x0B\xCEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x80\x15a\rbW[a\r\\W_\x90\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T`\x01\x80\x88R\x84\x88 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x8BR\x89\x89R\x87\x8B \x8B\x80R\x89R\x87\x8B \x80T\x92\x90\x95\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81\x17\x90\x95U\x93\x8AR\x97\x87R\x85\x89 \x82\x8AR\x87R\x94\x90\x97 \x80T\x87\x16\x90\x91\x17\x90U\x80T\x85\x16\x90U\x90\x91R\x80T\x90\x91\x16\x90UT\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a\x03\xDAW`\x01U\x90V[PP_\x90V[Pa\rn\x82`\x01a\x0B\x82V[\x15a\x0C\x7FV[a\r\x7F\x81`\x01a\x0B\x82V[a\r\x8AWP_\x90_\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x90\x81\x15\x15\x91\x90V[a\r\xE0\x81`\x01a\x0B\x82V[\x15\x80a\x0E\xC4W[a\r\xF0WP_\x90V[\x7Fn\xE3\xEF\xEC\xAE\x88=\xF2\xD7\xCC\xDA\"a\x0BL\xA7q\xA2\x99\xE7\x07\xCB\re\xC4\xEC\x97\xDCNfh\xAD\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 `\x01\x80\x86R\x81\x84R\x82\x86 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x89T\x81\x16\x88\x17\x90\x99U\x98\x90\x96\x16\x80\x85R\x92\x82R\x80\x84 \x97\x84R\x96\x81R\x86\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U\x81\x80R\x92\x90\x91R\x92\x90\x92 \x80T\x90\x91\x16\x90\x91\x17\x90U[`\x01T`\x01\x81\x01\x80\x91\x11a\x03\xDAW`\x01U`\x01\x90V[Pa\x0E\xD0_`\x01a\x0B\x82V[a\r\xE7V[a\x0E\xE0\x81`\x01a\x0B\x82V[\x15\x80a\x0F\xADW[a\x0E\xF0WP_\x90V[\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9D\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x88T\x81\x16\x87\x17\x90\x98U\x97\x90\x95\x16\x80\x84R\x91\x81R\x84\x83 \x83\x80R\x81R\x84\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U`\x01\x82R\x94\x90\x91R \x80T\x90\x91\x16\x90\x91\x17\x90Ua\x0E\xAEV[Pa\x0F\xB9_`\x01a\x0B\x82V[a\x0E\xE7V`\x80\x80`@R4`\x14W`\xE6\x90\x81a\0\x19\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15`\x11W_\x80\xFD[_5`\xE0\x1Ccz9y\xDC\x14`#W_\x80\xFD[4`\x9EW``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12`\x9EW`V`\xA2V[P`]`\xC4V[P`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x9EW6`#\x82\x01\x12\x15`\x9EW\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x9EW6\x91\x01`$\x01\x11`\x9EW\x80_` \x92R\xF3[_\x80\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03`\x9EWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03`\x9EWV",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f905f3560e01c90816301f8d4df1461339f5750806308226e93146131815780630a9254e41461309d5780631721daaa14612f9c5780631ed7831c14612f1e5780632ade388014612d2257806334d923db14612cc95780633e5e3c2314612c4b5780633f7286f414612bcd57806341e2ac2d14612b6757806357b50acc1461296757806366d9a9a0146128225780636e435a5b14612484578063756896711461224b57806385226c81146121b95780638a3603a314611fc9578063916a17c614611f1f57806391d01c61146119f95780639bd8bed71461182e5780639d62f5ab14611450578063a0974d8714610fd5578063a5081d7314610d7f578063b0464fdc14610cd5578063b5508aa914610c3c578063b86d529814610c12578063b91dc4991461096b578063ba414fa614610946578063c12e46871461091f578063cdc3e010146105ac578063db5bfb0d14610262578063e20c9f71146101d4578063f851a440146101ae5763fa7626d414610189575f80fd5b346101ab57806003193601126101ab57602060ff601f54166040519015158152f35b80fd5b50346101ab57806003193601126101ab5760206001600160a01b03815416604051908152f35b50346101ab57806003193601126101ab5760405180916020601554928381520191601582527f55f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475915b8181106102435761023f856102338187038261361e565b604051918291826136a2565b0390f35b82546001600160a01b031684526020909301926001928301920161021c565b50346101ab57806003193601126101ab576040516101008082019180831067ffffffffffffffff84111761057f57806140ba9383858339039084f08015610574576001600160a01b031691604051918083019183831067ffffffffffffffff84111761054757908392918339039083f080156104f9576001600160a01b031690826001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561051957604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f957610532575b506001600160a01b03601f5460081c16803b156105195781809160446040518094819363052eefd160e01b8352886004840152600160248401525af180156104f95761051d575b506001600160a01b03601f5460081c16803b156105195781809160446040518094819363052eefd160e01b8352896004840152600160248401525af180156104f957610504575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104f557826040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576104e0575b50916004926001600160a01b03601f5460081c16604051948580927f1b42c7110000000000000000000000000000000000000000000000000000000082525afa9283156104d5576104aa936001600160a01b039261049e9287926104ad575b506104a5906104948351613fc0565b8461049e84613a8e565b5116614037565b613ac8565b80f35b6104a59192506104ce903d808a833e6104c6818361361e565b810190613a03565b9190610485565b6040513d86823e3d90fd5b816104ea9161361e565b6104f557825f610426565b8280fd5b6040513d84823e3d90fd5b8161050e9161361e565b6104f557825f6103d3565b5080fd5b816105279161361e565b6104f557825f61038c565b8161053c9161361e565b6104f557825f610345565b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b6040513d85823e3d90fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b50346101ab57806003193601126101ab57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff57604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f95761090a575b5050600460206001600160a01b03601f5460081c16604051928380927fa26b4a880000000000000000000000000000000000000000000000000000000082525afa9081156104f95782916108d8575b50815b81811061082f575060018101809111610802578190737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff576040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f13d867a2000000000000000000000000000000000000000000000000000000006004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156105745783916107ea575b50506001600160a01b03601f5460081c1690813b156107e6576001600160a01b036044849283604051958694859363052eefd160e01b8552166004840152600160248401525af180156104f9576107d1575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b816107ca9161361e565b6101ab5780f35b816107db9161361e565b6101ab57805f61076b565b5050fd5b816107f49161361e565b6107ff57815f610719565b50fd5b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b600181018082116108ab5783906001600160a01b03601f5460081c1690813b156104f5576001600160a01b036044849283604051958694859363052eefd160e01b8552166004840152600160248401525af180156104f957610896575b5050600101610672565b816108a09161361e565b6104f557825f61088c565b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d602011610902575b816108f36020938361361e565b8101031261051957515f61066f565b3d91506108e6565b816109149161361e565b6101ab57805f610620565b50346101ab57806003193601126101ab5760206001600160a01b0360215416604051908152f35b50346101ab57806003193601126101ab576020610961613d7e565b6040519015158152f35b50346101ab57806003193601126101ab5760405190610100918281019281841067ffffffffffffffff851117610be557829382916140ba8339039082f08015610bd8576001600160a01b03166001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657604051906303223eab60e11b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610574578391610bc3575b50506001600160a01b03601f5460081c16803b156107e65782809160446040518094819363052eefd160e01b8352876004840152600160248401525af1908115610574578391610bae575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff576040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fa2d86a1e000000000000000000000000000000000000000000000000000000006004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610574578391610b99575b50506001600160a01b03601f5460081c1690813b156107e657829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156104f9576107d1575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b81610ba39161361e565b6107ff57815f610afb565b81610bb89161361e565b6107ff57815f610a67565b81610bcd9161361e565b6107ff57815f610a1c565b50604051903d90823e3d90fd5b6024837f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b50346101ab57806003193601126101ab5760206001600160a01b03601f5460081c16604051908152f35b50346101ab57806003193601126101ab57601954610c598161384d565b91610c67604051938461361e565b818352601981527f944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695602084015b838310610ca9576040518061023f8782613739565b600160208192604051610cc781610cc08189613922565b038261361e565b815201920192019190610c94565b50346101ab57806003193601126101ab57601c54610cf28161384d565b91610d00604051938461361e565b818352601c81527f0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211602084015b838310610d42576040518061023f87826137b6565b60026020600192604051610d55816135d5565b6001600160a01b038654168152610d6d858701613ad8565b83820152815201920192019190610d2d565b50346101ab57806003193601126101ab576040516101008082019180831067ffffffffffffffff84111761057f57806140ba9383858339039084f08015610574576001600160a01b031691604051918083019183831067ffffffffffffffff84111761054757908392918339039083f080156104f9576001600160a01b0316826001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561051957604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f957610fc0575b506001600160a01b03601f5460081c16803b156105195781809160446040518094819363052eefd160e01b83528960048401528160248401525af180156104f957610fab575b506001600160a01b03601f5460081c16803b156105195781809160446040518094819363052eefd160e01b83528860048401528160248401525af180156104f957610504575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156104f557826040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576104e05750916004926001600160a01b03601f5460081c16604051948580927f1b42c7110000000000000000000000000000000000000000000000000000000082525afa9283156104d5576104aa936001600160a01b039261049e9287926104ad57506104a5906104948351613fc0565b81610fb59161361e565b6104f557825f610ea7565b81610fca9161361e565b6104f557825f610e61565b50346101ab57806003193601126101ab5760405190610100918281019281841067ffffffffffffffff851117610be557829382916140ba8339039082f08015610bd8576001600160a01b03166001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657604051906303223eab60e11b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561057457839161143b575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff576040517f491cc7c20000000000000000000000000000000000000000000000000000000081528281806110f160048201905f6060608084019360018152600160208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610574578391611426575b5050604051817f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e78480a26001600160a01b03601f5460081c16803b156113dd578160448186809463052eefd160e01b8352876004840152600160248401525af1908115610574578391611411575b506004906001600160a01b03601f5460081c16604051928380927f1b42c7110000000000000000000000000000000000000000000000000000000082525afa908115610574576001600160a01b0361049e6111f593859387916113f7575b506111f08151613f49565b613a8e565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff576040517f491cc7c200000000000000000000000000000000000000000000000000000000815282818061125e60048201905f6060608084019360018152600160208201528260408201520152565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156105745783916113e2575b505060405190807fb5d68ca46372bbe6ec138d3d0423608269b3117496a46268f86080cdbcbea9be8480a26001600160a01b03601f5460081c16803b156113dd5760248385819381957f04f386f400000000000000000000000000000000000000000000000000000000845260048401525af180156104f9576113c8575b506004906001600160a01b03601f5460081c16604051928380927f1b42c7110000000000000000000000000000000000000000000000000000000082525afa80156104f95761135b9183916113ae575b5051613ed3565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b6113c291503d8085833e6104c6818361361e565b5f611354565b816113d29161361e565b6101ab57805f611304565b505050fd5b816113ec9161361e565b6107ff57815f611286565b61140b91503d8089833e6104c6818361361e565b5f6111e5565b8161141b9161361e565b6107ff57815f611187565b816114309161361e565b6107ff57815f611119565b816114459161361e565b6107ff57815f611086565b50346101ab57806003193601126101ab576040519060ff8083019280841067ffffffffffffffff851117610be557806152499483868339039083f09081156104f957604051908082019482861067ffffffffffffffff8711176118015794829185968339039083f09081156104f9576001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156113dd57604051906303223eab60e11b82526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156104d55784916117ec575b50506001600160a01b03601f5460081c1690813b156113dd576001600160a01b036044859283604051958694859363052eefd160e01b8552166004840152600160248401525af19081156105745783916117d7575b50506001600160a01b03601f5460081c1690813b156107e6576001600160a01b036044849283604051958694859363052eefd160e01b85521660048401528160248401525af180156104f9576117c2575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576117ad575b506040517f0200da48000000000000000000000000000000000000000000000000000000006020820152306024820152604060448201526116908161166460648201613865565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0810183528261361e565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff57816116eb91604051809381927ff28dceb300000000000000000000000000000000000000000000000000000000835260206004840152602483019061365f565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f957611798575b50506001600160a01b03601f5460081c16602060405180927f7a3979dc000000000000000000000000000000000000000000000000000000008252818061175a30600483016139da565b03915afa80156104f95761176c575080f35b61178d9060203d602011611791575b611785818361361e565b8101906139c2565b5080f35b503d61177b565b816117a29161361e565b6101ab57805f611710565b816117b79161361e565b6101ab57805f61161d565b816117cc9161361e565b6101ab57805f6115ca565b816117e19161361e565b6107ff57815f611579565b816117f69161361e565b6107e657825f611524565b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b50346101ab57806003193601126101ab57806001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff57604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576119e4575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fe6c4247b000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576119cf575b506001600160a01b03601f5460081c16803b156107ff5781809160446040518094819363052eefd160e01b8352816004840152600160248401525af180156104f9576107d1575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b816119d99161361e565b6101ab57805f611934565b816119ee9161361e565b6101ab57805f6118a2565b50346101ab57806003193601126101ab576040516101008082019082821067ffffffffffffffff83111761057f579082916140ba8339039082f08015610bd8576001600160a01b0316906040519160ff928381019381851067ffffffffffffffff86111761057f57839482916152498339039083f080156104f9576001600160a01b0316906001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156113dd57604051906303223eab60e11b82526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156104d5578491611f0a575b50506001600160a01b03601f5460081c16803b156113dd5783809160446040518094819363052eefd160e01b8352876004840152600160248401525af19081156104d5578491611ef5575b50506001600160a01b03601f5460081c16803b156113dd5783809160446040518094819363052eefd160e01b83528860048401528160248401525af19081156104d5578491611ee0575b50506001600160a01b03601f5460081c16906040517f7a3979dc00000000000000000000000000000000000000000000000000000000815260208180611bc130600483016139da565b0381865afa8015611ed557611bdd918691611eb6575b50613e57565b813b156113dd5783916024839260405194859384927f04f386f400000000000000000000000000000000000000000000000000000000845260048401525af1908115610574578391611ea1575b50506001600160a01b03601f5460081c16803b156107e6578280916024604051809481937f04f386f40000000000000000000000000000000000000000000000000000000083528760048401525af1908115610574578391611e8c575b50506001600160a01b03601f5460081c1690813b156107e657829160448392604051948593849263052eefd160e01b84526004840152600160248401525af180156104f957611e77575b506040517f0200da4800000000000000000000000000000000000000000000000000000000602082015230602482015260406044820152611d188161166460648201613865565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff5781611d7391604051809381927ff28dceb300000000000000000000000000000000000000000000000000000000835260206004840152602483019061365f565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f957611e62575b50506001600160a01b03601f5460081c16602060405180927f7a3979dc0000000000000000000000000000000000000000000000000000000082528180611de230600483016139da565b03915afa80156104f957611e45575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b611e5d9060203d60201161179157611785818361361e565b611df1565b81611e6c9161361e565b6101ab57805f611d98565b81611e819161361e565b6101ab57805f611cd1565b81611e969161361e565b6107ff57815f611c87565b81611eab9161361e565b6107ff57815f611c2a565b611ecf915060203d60201161179157611785818361361e565b5f611bd7565b6040513d87823e3d90fd5b81611eea9161361e565b6107e657825f611b78565b81611eff9161361e565b6107e657825f611b2e565b81611f149161361e565b6107e657825f611ae3565b50346101ab57806003193601126101ab57601d54611f3c8161384d565b91611f4a604051938461361e565b818352601d81527f6d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134f602084015b838310611f8c576040518061023f87826137b6565b60026020600192604051611f9f816135d5565b6001600160a01b038654168152611fb7858701613ad8565b83820152815201920192019190611f77565b50346101ab57806003193601126101ab5760405190610100918281019281841067ffffffffffffffff851117610be557829382916140ba8339039082f08015610bd8576001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657604051906303223eab60e11b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156105745783916121a4575b50506001600160a01b03601f5460081c1690813b156107e6576001600160a01b036044849283604051958694859363052eefd160e01b8552166004840152600160248401525af180156104f95761218f575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f95761217a575b50506001600160a01b03601f5460081c16602060405180927f7a3979dc000000000000000000000000000000000000000000000000000000008252818061216030600483016139da565b03915afa80156104f9576104aa918391611eb65750613e57565b816121849161361e565b6101ab57805f612116565b816121999161361e565b6101ab57805f6120c3565b816121ae9161361e565b6107ff57815f612071565b50346101ab57806003193601126101ab57601a546121d68161384d565b916121e4604051938461361e565b818352601a81527f057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e602084015b838310612226576040518061023f8782613739565b60016020819260405161223d81610cc08189613922565b815201920192019190612211565b50346101ab57806003193601126101ab576040516101008082019082821067ffffffffffffffff83111761057f579082916140ba8339039082f0908115612478576040519160ff928381019381851067ffffffffffffffff86111761057f57839482916152498339039083f080156104f9576001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156113dd57604051906303223eab60e11b82526004820152838160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156104d5578491612463575b50506001600160a01b03601f5460081c1690813b156113dd576001600160a01b036044859283604051958694859363052eefd160e01b8552166004840152600160248401525af19081156105745783916121a45750506001600160a01b03601f5460081c1690813b156107e6576001600160a01b036044849283604051958694859363052eefd160e01b8552166004840152600160248401525af180156104f95761218f575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f95761217a5750506001600160a01b03601f5460081c16602060405180927f7a3979dc000000000000000000000000000000000000000000000000000000008252818061216030600483016139da565b8161246d9161361e565b6107e657825f612322565b604051903d90823e3d90fd5b50346101ab57806003193601126101ab5760405190610100918281019281841067ffffffffffffffff851117610be557829382916140ba8339039082f08015610bd8576001600160a01b03166001600160a01b0360215416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657604051906303223eab60e11b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561057457839161280d575b50506001600160a01b0360215416604051907f118cdaa700000000000000000000000000000000000000000000000000000000602083015260248201526024815261258160448261361e565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657826125dc91604051809381927ff28dceb300000000000000000000000000000000000000000000000000000000835260206004840152602483019061365f565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156105745783916127f8575b50506001600160a01b03601f5460081c16803b156107e65782809160446040518094819363052eefd160e01b8352876004840152600160248401525af19081156105745783916127e3575b50506001600160a01b0360215416604051907f118cdaa700000000000000000000000000000000000000000000000000000000602083015260248201526024815261269b60448261361e565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657826126f691604051809381927ff28dceb300000000000000000000000000000000000000000000000000000000835260206004840152602483019061365f565b038183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af19081156105745783916127ce575b50506001600160a01b03601f5460081c1690813b156107e65782916024839260405194859384927f04f386f400000000000000000000000000000000000000000000000000000000845260048401525af180156104f9576107d1575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b816127d89161361e565b6107ff57815f61271e565b816127ed9161361e565b6107ff57815f61264f565b816128029161361e565b6107ff57815f612604565b816128179161361e565b6107ff57815f612535565b50346101ab57806003193601126101ab57601b5461283f8161384d565b61284c604051918261361e565b818152601b83526020810191837f3ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1845b83831061292457868587604051928392602084019060208552518091526040840160408260051b8601019392905b8282106128b957505050500390f35b91936020612914827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0600195979984950301865288519083612904835160408452604084019061365f565b92015190848184039101526136e4565b96019201920185949391926128aa565b60026020600192604051612937816135d5565b60405161294881610cc0818a613922565b8152612955858701613ad8565b8382015281520192019201919061287c565b50346101ab57806003193601126101ab576040516101008082019082821067ffffffffffffffff83111761057f579082916140ba8339039082f08015610bd8576001600160a01b0316816001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561051957604051906303223eab60e11b82526004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f957612b52575b506001600160a01b03601f5460081c16803b156105195781809160446040518094819363052eefd160e01b8352886004840152600160248401525af180156104f957612b3d575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561051957816040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f957612b28575b50906004916001600160a01b03601f5460081c16604051938480927f1b42c7110000000000000000000000000000000000000000000000000000000082525afa80156105745761049e6001600160a01b03916104aa948691612b1457506111f08151613f49565b61140b91503d8088833e6104c6818361361e565b81612b329161361e565b61051957815f612aad565b81612b479161361e565b61051957815f612a5a565b81612b5c9161361e565b61051957815f612a13565b50346101ab57806003193601126101ab576004816001600160a01b03601f5460081c16604051928380927f1b42c7110000000000000000000000000000000000000000000000000000000082525afa80156104f9576104aa9183916113ae575051613ed3565b50346101ab57806003193601126101ab5760405180916020601754928381520191601782527fc624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15915b818110612c2c5761023f856102338187038261361e565b82546001600160a01b0316845260209093019260019283019201612c15565b50346101ab57806003193601126101ab5760405180916020601854928381520191601882527fb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e915b818110612caa5761023f856102338187038261361e565b82546001600160a01b0316845260209093019260019283019201612c93565b50346101ab57806003193601126101ab576001600160a01b03601f5460081c16602060405180927f7a3979dc000000000000000000000000000000000000000000000000000000008252818061216030600483016139da565b50346101ab57806003193601126101ab57601e54612d3f8161384d565b612d4c604051918261361e565b818152601e83526020810191837f50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb6e350845b838310612e8d5786858760405192839260208401906020855251809152604084019160408260051b8601019392815b838310612db85786860387f35b9193957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0908692949603018352855190602060408201926001600160a01b0381511683520151916040602083015282518091526060820190602060608260051b850101940192855b828110612e4457505050505060208060019297019301930190928695949293612dab565b9091929394602080612e80837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08760019603018952895161365f565b9701950193929101612e20565b604051612e99816135d5565b6001600160a01b038354168152600183018054612eb58161384d565b91612ec3604051938461361e565b8183528a526020808b20908b9084015b838210612ef9575050505060019282602092836002950152815201920192019190612d7c565b600160208192604051612f1081610cc0818a613922565b815201930191019091612ed3565b50346101ab57806003193601126101ab5760405180916020601654928381520191601682527fd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289915b818110612f7d5761023f856102338187038261361e565b82546001600160a01b0316845260209093019260019283019201612f66565b50346101ab57806003193601126101ab57604051908060225490612fbf82613584565b80855291600181169081156130585750600114612ffb575b61023f84612fe78186038261361e565b60405191829160208352602083019061365f565b602281527f61035b26e3e9eee00e0d72fd1ee8ddca6894550dca6916ea2ac6baa90d11e510939250905b80821061303e57509091508101602001612fe782612fd7565b919260018160209254838588010152019101909291613025565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660208087019190915292151560051b85019092019250612fe79150839050612fd7565b50346101ab57806003193601126101ab57307fffffffffffffffffffffffff000000000000000000000000000000000000000060205416176020556104567fffffffffffffffffffffffff0000000000000000000000000000000000000000602154161760215560405161108f8082019082821067ffffffffffffffff83111761057f5760209183916141ba833930815203019082f08015610bd8577fffffffffffffffffffffff0000000000000000000000000000000000000000ff74ffffffffffffffffffffffffffffffffffffffff00601f549260081b16911617601f5580f35b50346101ab57806003193601126101ab5760405190610100918281019281841067ffffffffffffffff851117610be557829382916140ba8339039082f08015610bd8576001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107e657604051906303223eab60e11b82526004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af190811561057457839161338a575b5050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156107ff576040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527f3d0f293d000000000000000000000000000000000000000000000000000000006004820152828160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1908115610574578391613375575b50506001600160a01b03601f5460081c1690813b156107e6576001600160a01b03602484928360405195869485937f04f386f40000000000000000000000000000000000000000000000000000000085521660048401525af180156104f9576107d1575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b8161337f9161361e565b6107ff57815f6132bd565b816133949161361e565b6107ff57815f613229565b905034613580575f600319360112613580576001600160a01b0360205416737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15613580576303223eab60e11b825260048201525f8160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af1801561357557613562575b50737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040517fc31eb0e00000000000000000000000000000000000000000000000000000000081527fe6c4247b000000000000000000000000000000000000000000000000000000006004820152818160248183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f95761354d575b506001600160a01b03601f5460081c16803b156107ff578180916024604051809481937f04f386f40000000000000000000000000000000000000000000000000000000083528160048401525af180156104f9576107d1575050737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101ab57806040516390c5013b60e01b8152818160048183737109709ecfa91a80626ff3989d68f67f5b1dd12d5af180156104f9576107c05750f35b816135579161361e565b6101ab57805f6134a0565b61356e91505f9061361e565b5f5f61340f565b6040513d5f823e3d90fd5b5f80fd5b90600182811c921680156135cb575b602083101461359e57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b91607f1691613593565b6040810190811067ffffffffffffffff8211176135f157604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176135f157604052565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b60206040818301928281528451809452019201905f5b8181106136c55750505090565b82516001600160a01b03168452602093840193909201916001016136b8565b90602080835192838152019201905f5b8181106137015750505090565b82517fffffffff00000000000000000000000000000000000000000000000000000000168452602093840193909201916001016136f4565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061376b57505050505090565b90919293946020806137a7837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08660019603018752895161365f565b9701930193019193929061375c565b602081016020825282518091526040820191602060408360051b8301019401925f915b8383106137e857505050505090565b909192939460208061383e837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc086600196030187526040838b516001600160a01b038151168452015191818582015201906136e4565b970193019301919392906137d9565b67ffffffffffffffff81116135f15760051b60200190565b6022545f929161387482613584565b80825291600181169081156138e8575060011461388f575050565b60225f9081529293509091907f61035b26e3e9eee00e0d72fd1ee8ddca6894550dca6916ea2ac6baa90d11e5105b8383106138ce575060209250010190565b6001816020929493945483858701015201910191906138bd565b60209495507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091509291921683830152151560051b010190565b5f929181549161393183613584565b8083529260018116908115613986575060011461394d57505050565b5f9081526020812093945091925b83831061396c575060209250010190565b60018160209294939454838587010152019101919061395b565b905060209495507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091509291921683830152151560051b010190565b90816020910312613580575180151581036135805790565b6060906001600160a01b03613a00931681525f6020820152816040820152016022613922565b90565b6020818303126135805780519067ffffffffffffffff821161358057019080601f8301121561358057815190613a388261384d565b92613a46604051948561361e565b82845260208085019360051b82010191821161358057602001915b818310613a6e5750505090565b82516001600160a01b038116810361358057815260209283019201613a61565b805115613a9b5760200190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b805160011015613a9b5760400190565b90604051918281549182825260208201905f5260205f20925f905b806007830110613cf157613b49945491818110613cbb575b818110613c85575b818110613c4f575b818110613c19575b818110613be3575b818110613bad575b818110613b78575b10613b4b575b50038361361e565b565b7fffffffff000000000000000000000000000000000000000000000000000000001681526020015f613b41565b9260206001917fffffffff0000000000000000000000000000000000000000000000000000000085831b168152019301613b3b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560401b168152019301613b33565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560601b168152019301613b2b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560801b168152019301613b23565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560a01b168152019301613b1b565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560c01b168152019301613b13565b9260206001917fffffffff000000000000000000000000000000000000000000000000000000008560e01b168152019301613b0b565b9160089193506101006001917fffffffff000000000000000000000000000000000000000000000000000000008754818160e01b168352818160c01b166020840152818160a01b166040840152818160801b166060840152818160601b166080840152818160401b1660a0840152818160201b1660c08401521660e0820152019401920185929391613af3565b60085460ff168015613d8d5790565b506040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d60048201527f6661696c656400000000000000000000000000000000000000000000000000006024820152602081604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa908115613575575f91613e25575b50151590565b90506020813d602011613e4f575b81613e406020938361361e565b8101031261358057515f613e1f565b3d9150613e33565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561358057604051907f0c9fd581000000000000000000000000000000000000000000000000000000008252151560048201525f81602481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561357557613ec95750565b5f613b499161361e565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561358057604051907f98296c5400000000000000000000000000000000000000000000000000000000825260048201525f60248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561357557613ec95750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561358057604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152600160248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561357557613ec95750565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561358057604051907f98296c540000000000000000000000000000000000000000000000000000000082526004820152600260248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561357557613ec95750565b90737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15613580576001600160a01b039081604051937f515361f60000000000000000000000000000000000000000000000000000000085521660048401521660248201525f81604481737109709ecfa91a80626ff3989d68f67f5b1dd12d5afa801561357557613ec9575056fe6080806040523460145760e790816100198239f35b5f80fdfe60808060405260043610156011575f80fd5b5f3560e01c637a3979dc146023575f80fd5b34609f5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112609f57605660a3565b50605d60c5565b5060443567ffffffffffffffff8111609f5736602382011215609f57806004013567ffffffffffffffff8111609f5736910160240111609f5780600160209252f35b5f80fd5b6004359073ffffffffffffffffffffffffffffffffffffffff82168203609f57565b6024359073ffffffffffffffffffffffffffffffffffffffff82168203609f575660803460b857601f61108f38819003918201601f19168301916001600160401b0383118484101760bc5780849260209460405283398101031260b857516001600160a01b0381169081900360b857801560a5575f80546001600160a01b031981168317825560405192916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a3610fbe90816100d18239f35b631e4fbdf760e01b5f525f60045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c806304f386f41461063c578063052eefd1146104bb5780631b42c7111461029f578063715018a6146102235780637a3979dc146101905780638da5cb5b1461015e578063a26b4a88146101435763f2fde38b14610071575f80fd5b3461013f57602060031936011261013f5773ffffffffffffffffffffffffffffffffffffffff61009f61075a565b6100a7610a3e565b1680156101135773ffffffffffffffffffffffffffffffffffffffff5f54827fffffffffffffffffffffffff00000000000000000000000000000000000000008216175f55167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3005b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f80fd5b3461013f575f60031936011261013f57602060405160288152f35b3461013f575f60031936011261013f57602073ffffffffffffffffffffffffffffffffffffffff5f5416604051908152f35b3461013f57606060031936011261013f576101a961075a565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361013f576044359067ffffffffffffffff821161013f573660238301121561013f5781600401359067ffffffffffffffff821161013f57366024838501011161013f576020936024610219940191610841565b6040519015158152f35b3461013f575f60031936011261013f5761023b610a3e565b5f73ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461013f575f60031936011261013f576001546102bb816107eb565b6102c8604051918261077d565b8181526102d4826107eb565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe060208201920136833760015f9081527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff165b8482108061049c575b15610492578251821015610465578073ffffffffffffffffffffffffffffffffffffffff6103a3921660208460051b86010152610d74565b901561040757907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81146103da5760010190610362565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b50509091505b604051918291602083019060208452518091526040830191905f5b818110610436575050500390f35b825173ffffffffffffffffffffffffffffffffffffffff16845285945060209384019390920191600101610428565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b505090915061040d565b5073ffffffffffffffffffffffffffffffffffffffff8116151561036b565b3461013f57604060031936011261013f576104d461075a565b60243590811515820361013f576104e9610a3e565b73ffffffffffffffffffffffffffffffffffffffff81169182156106145761051082610a8a565b6105ec57602860015410156105c457156105b65761052d90610ed5565b15610558577f62101cccc1864d3492290070f4dbf16879de7861acb5dcb8180b55d2ed7cd7e75f80a2005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f41646472657373206e6f742061646465640000000000000000000000000000006044820152fd5b6105bf90610dd5565b61052d565b7f13d867a2000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fa2d86a1e000000000000000000000000000000000000000000000000000000005f5260045ffd5b7fe6c4247b000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461013f57602060031936011261013f5761065561075a565b61065d610a3e565b73ffffffffffffffffffffffffffffffffffffffff81169081156106145761068481610a8a565b156107325773ffffffffffffffffffffffffffffffffffffffff6106a88392610c5f565b16036106d4577fb5d68ca46372bbe6ec138d3d0423608269b3117496a46268f86080cdbcbea9be5f80a2005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601360248201527f41646472657373206e6f742072656d6f766564000000000000000000000000006044820152fd5b7f3d0f293d000000000000000000000000000000000000000000000000000000005f5260045ffd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361013f57565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176107be57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b67ffffffffffffffff81116107be5760051b60200190565b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b60015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d549394909373ffffffffffffffffffffffffffffffffffffffff169182156109cb57915b73ffffffffffffffffffffffffffffffffffffffff81168015610a1b57602060405180927f7a3979dc00000000000000000000000000000000000000000000000000000000825273ffffffffffffffffffffffffffffffffffffffff8916600483015273ffffffffffffffffffffffffffffffffffffffff87166024830152606060448301528180610944606482018d8c610803565b03915afa908115610a10575f916109d6575b506109cb5761096490610d74565b906108ae575050506109c79073ffffffffffffffffffffffffffffffffffffffff935b6040519485947f0200da48000000000000000000000000000000000000000000000000000000008652166004850152604060248501526044840191610803565b0390fd5b509350505050600190565b90506020813d8211610a08575b816109f06020938361077d565b8101031261013f5751801515810361013f575f610956565b3d91506109e3565b6040513d5f823e3d90fd5b505050506109c79073ffffffffffffffffffffffffffffffffffffffff93610987565b73ffffffffffffffffffffffffffffffffffffffff5f54163303610a5e57565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff16805f52600260205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f2054161580610b4d575b15610b475760015f527fac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b6020527f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d5473ffffffffffffffffffffffffffffffffffffffff1603610b4357600190565b5f90565b50600190565b50805f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615610ad4565b60010173ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f205f805260205273ffffffffffffffffffffffffffffffffffffffff60405f2054161580610c15575b15610c0e575f805260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2054169116145f14610b4357600190565b5050600190565b5073ffffffffffffffffffffffffffffffffffffffff82165f528060205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f20541615610bce565b73ffffffffffffffffffffffffffffffffffffffff811680158015610d62575b610d5c575f90815260026020818152604080842084805280835281852080546001808852848820805473ffffffffffffffffffffffffffffffffffffffff908116808b52898952878b208b80528952878b208054929095167fffffffffffffffffffffffff00000000000000000000000000000000000000009283168117909555938a52978752858920828a5287529490972080548716909117905580548516905590915280549091169055547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81019081116103da5760015590565b50505f90565b50610d6e826001610b82565b15610c7f565b610d7f816001610b82565b610d8a57505f905f90565b73ffffffffffffffffffffffffffffffffffffffff165f52600260205260405f2060015f5260205273ffffffffffffffffffffffffffffffffffffffff60405f205416908115159190565b610de0816001610b82565b1580610ec4575b610df057505f90565b7f6ee3efecae883df2d7ccda22610b4ca771a299e707cb0d65c4ec97dc4e6668ad805473ffffffffffffffffffffffffffffffffffffffff9283165f818152600260208181526040808420600180865281845282862080547fffffffffffffffffffffffff000000000000000000000000000000000000000090811690915589548116881790995598909616808552928252808420978452968152868320805487169094179093558180529290915292909220805490911690911790555b600154600181018091116103da57600155600190565b50610ed05f6001610b82565b610de7565b610ee0816001610b82565b1580610fad575b610ef057505f90565b7f79c06e8c99a667adda63c5fa6f05695d29630fc62ad2dd069fa929d5714de89d805473ffffffffffffffffffffffffffffffffffffffff9283165f81815260026020818152604080842084805280835281852080547fffffffffffffffffffffffff00000000000000000000000000000000000000009081169091558854811687179098559790951680845291815284832083805281528483208054871690941790935560018252949091522080549091169091179055610eae565b50610fb95f6001610b82565b610ee7566080806040523460145760e690816100198239f35b5f80fdfe60808060405260043610156011575f80fd5b5f3560e01c637a3979dc146023575f80fd5b34609e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112609e57605660a2565b50605d60c4565b5060443567ffffffffffffffff8111609e5736602382011215609e57806004013567ffffffffffffffff8111609e5736910160240111609e57805f60209252f35b5f80fd5b6004359073ffffffffffffffffffffffffffffffffffffffff82168203609e57565b6024359073ffffffffffffffffffffffffffffffffffffffff82168203609e5756
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x01\xF8\xD4\xDF\x14a3\x9FWP\x80c\x08\"n\x93\x14a1\x81W\x80c\n\x92T\xE4\x14a0\x9DW\x80c\x17!\xDA\xAA\x14a/\x9CW\x80c\x1E\xD7\x83\x1C\x14a/\x1EW\x80c*\xDE8\x80\x14a-\"W\x80c4\xD9#\xDB\x14a,\xC9W\x80c>^<#\x14a,KW\x80c?r\x86\xF4\x14a+\xCDW\x80cA\xE2\xAC-\x14a+gW\x80cW\xB5\n\xCC\x14a)gW\x80cf\xD9\xA9\xA0\x14a(\"W\x80cnCZ[\x14a$\x84W\x80cuh\x96q\x14a\"KW\x80c\x85\"l\x81\x14a!\xB9W\x80c\x8A6\x03\xA3\x14a\x1F\xC9W\x80c\x91j\x17\xC6\x14a\x1F\x1FW\x80c\x91\xD0\x1Ca\x14a\x19\xF9W\x80c\x9B\xD8\xBE\xD7\x14a\x18.W\x80c\x9Db\xF5\xAB\x14a\x14PW\x80c\xA0\x97M\x87\x14a\x0F\xD5W\x80c\xA5\x08\x1Ds\x14a\r\x7FW\x80c\xB0FO\xDC\x14a\x0C\xD5W\x80c\xB5P\x8A\xA9\x14a\x0C<W\x80c\xB8mR\x98\x14a\x0C\x12W\x80c\xB9\x1D\xC4\x99\x14a\tkW\x80c\xBAAO\xA6\x14a\tFW\x80c\xC1.F\x87\x14a\t\x1FW\x80c\xCD\xC3\xE0\x10\x14a\x05\xACW\x80c\xDB[\xFB\r\x14a\x02bW\x80c\xE2\x0C\x9Fq\x14a\x01\xD4W\x80c\xF8Q\xA4@\x14a\x01\xAEWc\xFAv&\xD4\x14a\x01\x89W_\x80\xFD[4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW` `\xFF`\x1FT\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW` `\x01`\x01`\xA0\x1B\x03\x81T\x16`@Q\x90\x81R\xF3[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x80\x91` `\x15T\x92\x83\x81R\x01\x91`\x15\x82R\x7FU\xF4H\xFD\xEA\x98\xC4\xD2\x9E\xB3@u~\xF0\xA6l\xD0=\xBB\x958\x90\x8Aj\x81\xD9`&\xB7\x1E\xC4u\x91[\x81\x81\x10a\x02CWa\x02?\x85a\x023\x81\x87\x03\x82a6\x1EV[`@Q\x91\x82\x91\x82a6\xA2V[\x03\x90\xF3[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x02\x1CV[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Qa\x01\0\x80\x82\x01\x91\x80\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x05\x7FW\x80a@\xBA\x93\x83\x85\x839\x03\x90\x84\xF0\x80\x15a\x05tW`\x01`\x01`\xA0\x1B\x03\x16\x91`@Q\x91\x80\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x05GW\x90\x83\x92\x91\x839\x03\x90\x83\xF0\x80\x15a\x04\xF9W`\x01`\x01`\xA0\x1B\x03\x16\x90\x82`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x19W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x052W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x05\x19W\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x88`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x05\x1DW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x05\x19W\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x89`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x05\x04W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xF5W\x82`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x04\xE0W[P\x91`\x04\x92`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x80\x92\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x92\x83\x15a\x04\xD5Wa\x04\xAA\x93`\x01`\x01`\xA0\x1B\x03\x92a\x04\x9E\x92\x87\x92a\x04\xADW[Pa\x04\xA5\x90a\x04\x94\x83Qa?\xC0V[\x84a\x04\x9E\x84a:\x8EV[Q\x16a@7V[a:\xC8V[\x80\xF3[a\x04\xA5\x91\x92Pa\x04\xCE\x90=\x80\x8A\x83>a\x04\xC6\x81\x83a6\x1EV[\x81\x01\x90a:\x03V[\x91\x90a\x04\x85V[`@Q=\x86\x82>=\x90\xFD[\x81a\x04\xEA\x91a6\x1EV[a\x04\xF5W\x82_a\x04&V[\x82\x80\xFD[`@Q=\x84\x82>=\x90\xFD[\x81a\x05\x0E\x91a6\x1EV[a\x04\xF5W\x82_a\x03\xD3V[P\x80\xFD[\x81a\x05'\x91a6\x1EV[a\x04\xF5W\x82_a\x03\x8CV[\x81a\x05<\x91a6\x1EV[a\x04\xF5W\x82_a\x03EV[`$\x86\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[`@Q=\x85\x82>=\x90\xFD[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\t\nW[PP`\x04` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\xA2kJ\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x04\xF9W\x82\x91a\x08\xD8W[P\x81[\x81\x81\x10a\x08/WP`\x01\x81\x01\x80\x91\x11a\x08\x02W\x81\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\x13\xD8g\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a\x07\xEAW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W`\x01`\x01`\xA0\x1B\x03`D\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x07\xD1W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[\x81a\x07\xCA\x91a6\x1EV[a\x01\xABW\x80\xF3[\x81a\x07\xDB\x91a6\x1EV[a\x01\xABW\x80_a\x07kV[PP\xFD[\x81a\x07\xF4\x91a6\x1EV[a\x07\xFFW\x81_a\x07\x19V[P\xFD[`$\x82\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[`\x01\x81\x01\x80\x82\x11a\x08\xABW\x83\x90`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x04\xF5W`\x01`\x01`\xA0\x1B\x03`D\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x08\x96W[PP`\x01\x01a\x06rV[\x81a\x08\xA0\x91a6\x1EV[a\x04\xF5W\x82_a\x08\x8CV[`$\x84\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x90P` \x81=` \x11a\t\x02W[\x81a\x08\xF3` \x93\x83a6\x1EV[\x81\x01\x03\x12a\x05\x19WQ_a\x06oV[=\x91Pa\x08\xE6V[\x81a\t\x14\x91a6\x1EV[a\x01\xABW\x80_a\x06 V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW` `\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x81R\xF3[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW` a\taa=~V[`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90a\x01\0\x91\x82\x81\x01\x92\x81\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x0B\xE5W\x82\x93\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a\x0B\xC3W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07\xE6W\x82\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x87`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a\x0B\xAEW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xA2\xD8j\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a\x0B\x99W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x07\xD1WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[\x81a\x0B\xA3\x91a6\x1EV[a\x07\xFFW\x81_a\n\xFBV[\x81a\x0B\xB8\x91a6\x1EV[a\x07\xFFW\x81_a\ngV[\x81a\x0B\xCD\x91a6\x1EV[a\x07\xFFW\x81_a\n\x1CV[P`@Q\x90=\x90\x82>=\x90\xFD[`$\x83\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW` `\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x90\x81R\xF3[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x19Ta\x0CY\x81a8MV[\x91a\x0Cg`@Q\x93\x84a6\x1EV[\x81\x83R`\x19\x81R\x7F\x94I\x98'>G{IQD\xFB\x87\x94\xC9\x14\x19\x7F<\xCBF\xBE)\0\xF4i\x8F\xD0\xEFt<\x96\x95` \x84\x01[\x83\x83\x10a\x0C\xA9W`@Q\x80a\x02?\x87\x82a79V[`\x01` \x81\x92`@Qa\x0C\xC7\x81a\x0C\xC0\x81\x89a9\"V[\x03\x82a6\x1EV[\x81R\x01\x92\x01\x92\x01\x91\x90a\x0C\x94V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x1CTa\x0C\xF2\x81a8MV[\x91a\r\0`@Q\x93\x84a6\x1EV[\x81\x83R`\x1C\x81R\x7F\x0EEb\xA1\x03\x81\xDE\xC2\x1B ^\xD7&7\xE6\xB1\xB5#\xBD\xD0\xE4\xD4\xD5\n\xF5\xCD#\xDDE\0\xA2\x11` \x84\x01[\x83\x83\x10a\rBW`@Q\x80a\x02?\x87\x82a7\xB6V[`\x02` `\x01\x92`@Qa\rU\x81a5\xD5V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\rm\x85\x87\x01a:\xD8V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\r-V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Qa\x01\0\x80\x82\x01\x91\x80\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x05\x7FW\x80a@\xBA\x93\x83\x85\x839\x03\x90\x84\xF0\x80\x15a\x05tW`\x01`\x01`\xA0\x1B\x03\x16\x91`@Q\x91\x80\x83\x01\x91\x83\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x05GW\x90\x83\x92\x91\x839\x03\x90\x83\xF0\x80\x15a\x04\xF9W`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x19W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x0F\xC0W[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x05\x19W\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x89`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x0F\xABW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x05\x19W\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x88`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x05\x04WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04\xF5W\x82`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x04\xE0WP\x91`\x04\x92`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x94\x85\x80\x92\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x92\x83\x15a\x04\xD5Wa\x04\xAA\x93`\x01`\x01`\xA0\x1B\x03\x92a\x04\x9E\x92\x87\x92a\x04\xADWPa\x04\xA5\x90a\x04\x94\x83Qa?\xC0V[\x81a\x0F\xB5\x91a6\x1EV[a\x04\xF5W\x82_a\x0E\xA7V[\x81a\x0F\xCA\x91a6\x1EV[a\x04\xF5W\x82_a\x0EaV[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90a\x01\0\x91\x82\x81\x01\x92\x81\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x0B\xE5W\x82\x93\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a\x14;W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81\x80a\x10\xF1`\x04\x82\x01\x90_```\x80\x84\x01\x93`\x01\x81R`\x01` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a\x14&W[PP`@Q\x81\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7\x84\x80\xA2`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x13\xDDW\x81`D\x81\x86\x80\x94c\x05.\xEF\xD1`\xE0\x1B\x83R\x87`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a\x14\x11W[P`\x04\x90`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x05tW`\x01`\x01`\xA0\x1B\x03a\x04\x9Ea\x11\xF5\x93\x85\x93\x87\x91a\x13\xF7W[Pa\x11\xF0\x81Qa?IV[a:\x8EV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x7FI\x1C\xC7\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82\x81\x80a\x12^`\x04\x82\x01\x90_```\x80\x84\x01\x93`\x01\x81R`\x01` \x82\x01R\x82`@\x82\x01R\x01RV[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a\x13\xE2W[PP`@Q\x90\x80\x7F\xB5\xD6\x8C\xA4cr\xBB\xE6\xEC\x13\x8D=\x04#`\x82i\xB3\x11t\x96\xA4bh\xF8`\x80\xCD\xBC\xBE\xA9\xBE\x84\x80\xA2`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x13\xDDW`$\x83\x85\x81\x93\x81\x95\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x13\xC8W[P`\x04\x90`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x04\xF9Wa\x13[\x91\x83\x91a\x13\xAEW[PQa>\xD3V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[a\x13\xC2\x91P=\x80\x85\x83>a\x04\xC6\x81\x83a6\x1EV[_a\x13TV[\x81a\x13\xD2\x91a6\x1EV[a\x01\xABW\x80_a\x13\x04V[PPP\xFD[\x81a\x13\xEC\x91a6\x1EV[a\x07\xFFW\x81_a\x12\x86V[a\x14\x0B\x91P=\x80\x89\x83>a\x04\xC6\x81\x83a6\x1EV[_a\x11\xE5V[\x81a\x14\x1B\x91a6\x1EV[a\x07\xFFW\x81_a\x11\x87V[\x81a\x140\x91a6\x1EV[a\x07\xFFW\x81_a\x11\x19V[\x81a\x14E\x91a6\x1EV[a\x07\xFFW\x81_a\x10\x86V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90`\xFF\x80\x83\x01\x92\x80\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x0B\xE5W\x80aRI\x94\x83\x86\x839\x03\x90\x83\xF0\x90\x81\x15a\x04\xF9W`@Q\x90\x80\x82\x01\x94\x82\x86\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x11\x17a\x18\x01W\x94\x82\x91\x85\x96\x839\x03\x90\x83\xF0\x90\x81\x15a\x04\xF9W`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13\xDDW`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04\xD5W\x84\x91a\x17\xECW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x13\xDDW`\x01`\x01`\xA0\x1B\x03`D\x85\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a\x17\xD7W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W`\x01`\x01`\xA0\x1B\x03`D\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x17\xC2W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x17\xADW[P`@Q\x7F\x02\0\xDAH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R0`$\x82\x01R`@`D\x82\x01Ra\x16\x90\x81a\x16d`d\x82\x01a8eV[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a6\x1EV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW\x81a\x16\xEB\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6_V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x17\x98W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x80a\x17Z0`\x04\x83\x01a9\xDAV[\x03\x91Z\xFA\x80\x15a\x04\xF9Wa\x17lWP\x80\xF3[a\x17\x8D\x90` =` \x11a\x17\x91W[a\x17\x85\x81\x83a6\x1EV[\x81\x01\x90a9\xC2V[P\x80\xF3[P=a\x17{V[\x81a\x17\xA2\x91a6\x1EV[a\x01\xABW\x80_a\x17\x10V[\x81a\x17\xB7\x91a6\x1EV[a\x01\xABW\x80_a\x16\x1DV[\x81a\x17\xCC\x91a6\x1EV[a\x01\xABW\x80_a\x15\xCAV[\x81a\x17\xE1\x91a6\x1EV[a\x07\xFFW\x81_a\x15yV[\x81a\x17\xF6\x91a6\x1EV[a\x07\xE6W\x82_a\x15$V[`$\x85\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`A`\x04R\xFD[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW\x80`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x19\xE4W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x19\xCFW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07\xFFW\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x81`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x07\xD1WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[\x81a\x19\xD9\x91a6\x1EV[a\x01\xABW\x80_a\x194V[\x81a\x19\xEE\x91a6\x1EV[a\x01\xABW\x80_a\x18\xA2V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Qa\x01\0\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x05\x7FW\x90\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03\x16\x90`@Q\x91`\xFF\x92\x83\x81\x01\x93\x81\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x05\x7FW\x83\x94\x82\x91aRI\x839\x03\x90\x83\xF0\x80\x15a\x04\xF9W`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13\xDDW`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04\xD5W\x84\x91a\x1F\nW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x13\xDDW\x83\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x87`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x90\x81\x15a\x04\xD5W\x84\x91a\x1E\xF5W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x13\xDDW\x83\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x88`\x04\x84\x01R\x81`$\x84\x01RZ\xF1\x90\x81\x15a\x04\xD5W\x84\x91a\x1E\xE0W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90`@Q\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81\x80a\x1B\xC10`\x04\x83\x01a9\xDAV[\x03\x81\x86Z\xFA\x80\x15a\x1E\xD5Wa\x1B\xDD\x91\x86\x91a\x1E\xB6W[Pa>WV[\x81;\x15a\x13\xDDW\x83\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a\x1E\xA1W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07\xE6W\x82\x80\x91`$`@Q\x80\x94\x81\x93\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x87`\x04\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a\x1E\x8CW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W\x82\x91`D\x83\x92`@Q\x94\x85\x93\x84\x92c\x05.\xEF\xD1`\xE0\x1B\x84R`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x1EwW[P`@Q\x7F\x02\0\xDAH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R0`$\x82\x01R`@`D\x82\x01Ra\x1D\x18\x81a\x16d`d\x82\x01a8eV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW\x81a\x1Ds\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6_V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x1EbW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x80a\x1D\xE20`\x04\x83\x01a9\xDAV[\x03\x91Z\xFA\x80\x15a\x04\xF9Wa\x1EEW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[a\x1E]\x90` =` \x11a\x17\x91Wa\x17\x85\x81\x83a6\x1EV[a\x1D\xF1V[\x81a\x1El\x91a6\x1EV[a\x01\xABW\x80_a\x1D\x98V[\x81a\x1E\x81\x91a6\x1EV[a\x01\xABW\x80_a\x1C\xD1V[\x81a\x1E\x96\x91a6\x1EV[a\x07\xFFW\x81_a\x1C\x87V[\x81a\x1E\xAB\x91a6\x1EV[a\x07\xFFW\x81_a\x1C*V[a\x1E\xCF\x91P` =` \x11a\x17\x91Wa\x17\x85\x81\x83a6\x1EV[_a\x1B\xD7V[`@Q=\x87\x82>=\x90\xFD[\x81a\x1E\xEA\x91a6\x1EV[a\x07\xE6W\x82_a\x1BxV[\x81a\x1E\xFF\x91a6\x1EV[a\x07\xE6W\x82_a\x1B.V[\x81a\x1F\x14\x91a6\x1EV[a\x07\xE6W\x82_a\x1A\xE3V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x1DTa\x1F<\x81a8MV[\x91a\x1FJ`@Q\x93\x84a6\x1EV[\x81\x83R`\x1D\x81R\x7FmD\x07\xE7\xBE!\xF8\x08\xE6P\x9A\xA9\xFA\x91C6\x95y\xDD}v\x0F\xE2\n,\th\x0F\xC1F\x13O` \x84\x01[\x83\x83\x10a\x1F\x8CW`@Q\x80a\x02?\x87\x82a7\xB6V[`\x02` `\x01\x92`@Qa\x1F\x9F\x81a5\xD5V[`\x01`\x01`\xA0\x1B\x03\x86T\x16\x81Ra\x1F\xB7\x85\x87\x01a:\xD8V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x1FwV[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90a\x01\0\x91\x82\x81\x01\x92\x81\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x0B\xE5W\x82\x93\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a!\xA4W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W`\x01`\x01`\xA0\x1B\x03`D\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa!\x8FW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa!zW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x80a!`0`\x04\x83\x01a9\xDAV[\x03\x91Z\xFA\x80\x15a\x04\xF9Wa\x04\xAA\x91\x83\x91a\x1E\xB6WPa>WV[\x81a!\x84\x91a6\x1EV[a\x01\xABW\x80_a!\x16V[\x81a!\x99\x91a6\x1EV[a\x01\xABW\x80_a \xC3V[\x81a!\xAE\x91a6\x1EV[a\x07\xFFW\x81_a qV[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x1ATa!\xD6\x81a8MV[\x91a!\xE4`@Q\x93\x84a6\x1EV[\x81\x83R`\x1A\x81R\x7F\x05|8J}\x1CT\xF3\xA1\xB2\xE5\xE6{&\x17\xB8\"O\xDF\xD1\xEAr4\xEE\xA5s\xA6\xFFf_\xF6>` \x84\x01[\x83\x83\x10a\"&W`@Q\x80a\x02?\x87\x82a79V[`\x01` \x81\x92`@Qa\"=\x81a\x0C\xC0\x81\x89a9\"V[\x81R\x01\x92\x01\x92\x01\x91\x90a\"\x11V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Qa\x01\0\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x05\x7FW\x90\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x90\x81\x15a$xW`@Q\x91`\xFF\x92\x83\x81\x01\x93\x81\x85\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x17a\x05\x7FW\x83\x94\x82\x91aRI\x839\x03\x90\x83\xF0\x80\x15a\x04\xF9W`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13\xDDW`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x83\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x04\xD5W\x84\x91a$cW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x13\xDDW`\x01`\x01`\xA0\x1B\x03`D\x85\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a!\xA4WPP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W`\x01`\x01`\xA0\x1B\x03`D\x84\x92\x83`@Q\x95\x86\x94\x85\x93c\x05.\xEF\xD1`\xE0\x1B\x85R\x16`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa!\x8FWPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa!zWPP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x80a!`0`\x04\x83\x01a9\xDAV[\x81a$m\x91a6\x1EV[a\x07\xE6W\x82_a#\"V[`@Q\x90=\x90\x82>=\x90\xFD[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90a\x01\0\x91\x82\x81\x01\x92\x81\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x0B\xE5W\x82\x93\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03`!T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a(\rW[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01R`$\x81Ra%\x81`D\x82a6\x1EV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W\x82a%\xDC\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6_V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a'\xF8W[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07\xE6W\x82\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x87`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x90\x81\x15a\x05tW\x83\x91a'\xE3W[PP`\x01`\x01`\xA0\x1B\x03`!T\x16`@Q\x90\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R`$\x82\x01R`$\x81Ra&\x9B`D\x82a6\x1EV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W\x82a&\xF6\x91`@Q\x80\x93\x81\x92\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`$\x83\x01\x90a6_V[\x03\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a'\xCEW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W\x82\x91`$\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x07\xD1WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[\x81a'\xD8\x91a6\x1EV[a\x07\xFFW\x81_a'\x1EV[\x81a'\xED\x91a6\x1EV[a\x07\xFFW\x81_a&OV[\x81a(\x02\x91a6\x1EV[a\x07\xFFW\x81_a&\x04V[\x81a(\x17\x91a6\x1EV[a\x07\xFFW\x81_a%5V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x1BTa(?\x81a8MV[a(L`@Q\x91\x82a6\x1EV[\x81\x81R`\x1B\x83R` \x81\x01\x91\x83\x7F:\xD8\xAAO\x87TC#\xA9\xD1\xE5\xDD\x90/@\xC3VRzyUhq\x13\xDB_\x9A\x85\xADW\x9D\xC1\x84[\x83\x83\x10a)$W\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x90[\x82\x82\x10a(\xB9WPPPP\x03\x90\xF3[\x91\x93` a)\x14\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x01\x95\x97\x99\x84\x95\x03\x01\x86R\x88Q\x90\x83a)\x04\x83Q`@\x84R`@\x84\x01\x90a6_V[\x92\x01Q\x90\x84\x81\x84\x03\x91\x01Ra6\xE4V[\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a(\xAAV[`\x02` `\x01\x92`@Qa)7\x81a5\xD5V[`@Qa)H\x81a\x0C\xC0\x81\x8Aa9\"V[\x81Ra)U\x85\x87\x01a:\xD8V[\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a(|V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Qa\x01\0\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x05\x7FW\x90\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x19W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa+RW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x05\x19W\x81\x80\x91`D`@Q\x80\x94\x81\x93c\x05.\xEF\xD1`\xE0\x1B\x83R\x88`\x04\x84\x01R`\x01`$\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa+=W[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\x19W\x81`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa+(W[P\x90`\x04\x91`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x93\x84\x80\x92\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x05tWa\x04\x9E`\x01`\x01`\xA0\x1B\x03\x91a\x04\xAA\x94\x86\x91a+\x14WPa\x11\xF0\x81Qa?IV[a\x14\x0B\x91P=\x80\x88\x83>a\x04\xC6\x81\x83a6\x1EV[\x81a+2\x91a6\x1EV[a\x05\x19W\x81_a*\xADV[\x81a+G\x91a6\x1EV[a\x05\x19W\x81_a*ZV[\x81a+\\\x91a6\x1EV[a\x05\x19W\x81_a*\x13V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x04\x81`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16`@Q\x92\x83\x80\x92\x7F\x1BB\xC7\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\x04\xF9Wa\x04\xAA\x91\x83\x91a\x13\xAEWPQa>\xD3V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x80\x91` `\x17T\x92\x83\x81R\x01\x91`\x17\x82R\x7F\xC6$\xB6l\xC0\x13\x8B\x8F\xAB\xC2\t$\x7Fr\xD7X\xE1\xCF3CumT;\xAD\xBF$!+\xED\x8C\x15\x91[\x81\x81\x10a,,Wa\x02?\x85a\x023\x81\x87\x03\x82a6\x1EV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a,\x15V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x80\x91` `\x18T\x92\x83\x81R\x01\x91`\x18\x82R\x7F\xB1=-v\xD1\xF4\xB7\xBE\x83H\x82\xE4\x10\xB3\xE3\xA8\xAF\xAFi\xF86\0\xAE$\xDB5C\x91\xD27\x8D.\x91[\x81\x81\x10a,\xAAWa\x02?\x85a\x023\x81\x87\x03\x82a6\x1EV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a,\x93V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x81\x80a!`0`\x04\x83\x01a9\xDAV[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`\x1ETa-?\x81a8MV[a-L`@Q\x91\x82a6\x1EV[\x81\x81R`\x1E\x83R` \x81\x01\x91\x83\x7FP\xBBf\x9A\x95\xC7\xB5\x0B~\x8Ao\tE@4\xB2\xB1L\xF2\xB8\\s\r\xCA\x9AS\x9C\xA8,\xB6\xE3P\x84[\x83\x83\x10a.\x8DW\x86\x85\x87`@Q\x92\x83\x92` \x84\x01\x90` \x85RQ\x80\x91R`@\x84\x01\x91`@\x82`\x05\x1B\x86\x01\x01\x93\x92\x81[\x83\x83\x10a-\xB8W\x86\x86\x03\x87\xF3[\x91\x93\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x90\x86\x92\x94\x96\x03\x01\x83R\x85Q\x90` `@\x82\x01\x92`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x83R\x01Q\x91`@` \x83\x01R\x82Q\x80\x91R``\x82\x01\x90` ``\x82`\x05\x1B\x85\x01\x01\x94\x01\x92\x85[\x82\x81\x10a.DWPPPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x90\x92\x86\x95\x94\x92\x93a-\xABV[\x90\x91\x92\x93\x94` \x80a.\x80\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87`\x01\x96\x03\x01\x89R\x89Qa6_V[\x97\x01\x95\x01\x93\x92\x91\x01a. V[`@Qa.\x99\x81a5\xD5V[`\x01`\x01`\xA0\x1B\x03\x83T\x16\x81R`\x01\x83\x01\x80Ta.\xB5\x81a8MV[\x91a.\xC3`@Q\x93\x84a6\x1EV[\x81\x83R\x8AR` \x80\x8B \x90\x8B\x90\x84\x01[\x83\x82\x10a.\xF9WPPPP`\x01\x92\x82` \x92\x83`\x02\x95\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a-|V[`\x01` \x81\x92`@Qa/\x10\x81a\x0C\xC0\x81\x8Aa9\"V[\x81R\x01\x93\x01\x91\x01\x90\x91a.\xD3V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x80\x91` `\x16T\x92\x83\x81R\x01\x91`\x16\x82R\x7F\xD83\x14}}\xC3U\xBAE\x9F\xC7\x88\xF6i\xE5\x8C\xFA\xF9\xDC%\xDD\xCD\x07\x02\xE8}i\xC7\xB5\x12B\x89\x91[\x81\x81\x10a/}Wa\x02?\x85a\x023\x81\x87\x03\x82a6\x1EV[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a/fV[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90\x80`\"T\x90a/\xBF\x82a5\x84V[\x80\x85R\x91`\x01\x81\x16\x90\x81\x15a0XWP`\x01\x14a/\xFBW[a\x02?\x84a/\xE7\x81\x86\x03\x82a6\x1EV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a6_V[`\"\x81R\x7Fa\x03[&\xE3\xE9\xEE\xE0\x0E\rr\xFD\x1E\xE8\xDD\xCAh\x94U\r\xCAi\x16\xEA*\xC6\xBA\xA9\r\x11\xE5\x10\x93\x92P\x90[\x80\x82\x10a0>WP\x90\x91P\x81\x01` \x01a/\xE7\x82a/\xD7V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a0%V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16` \x80\x87\x01\x91\x90\x91R\x92\x15\x15`\x05\x1B\x85\x01\x90\x92\x01\x92Pa/\xE7\x91P\x83\x90Pa/\xD7V[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` T\x16\x17` Ua\x04V\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`!T\x16\x17`!U`@Qa\x10\x8F\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x05\x7FW` \x91\x83\x91aA\xBA\x8390\x81R\x03\x01\x90\x82\xF0\x80\x15a\x0B\xD8W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFFt\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0`\x1FT\x92`\x08\x1B\x16\x91\x16\x17`\x1FU\x80\xF3[P4a\x01\xABW\x80`\x03\x196\x01\x12a\x01\xABW`@Q\x90a\x01\0\x91\x82\x81\x01\x92\x81\x84\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11\x17a\x0B\xE5W\x82\x93\x82\x91a@\xBA\x839\x03\x90\x82\xF0\x80\x15a\x0B\xD8W`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xE6W`@Q\x90c\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a3\x8AW[PPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\xFFW`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x82\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x90\x81\x15a\x05tW\x83\x91a3uW[PP`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x90\x81;\x15a\x07\xE6W`\x01`\x01`\xA0\x1B\x03`$\x84\x92\x83`@Q\x95\x86\x94\x85\x93\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x07\xD1WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[\x81a3\x7F\x91a6\x1EV[a\x07\xFFW\x81_a2\xBDV[\x81a3\x94\x91a6\x1EV[a\x07\xFFW\x81_a2)V[\x90P4a5\x80W_`\x03\x196\x01\x12a5\x80W`\x01`\x01`\xA0\x1B\x03` T\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a5\x80Wc\x03\">\xAB`\xE1\x1B\x82R`\x04\x82\x01R_\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a5uWa5bW[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Q\x7F\xC3\x1E\xB0\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R\x81\x81`$\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa5MW[P`\x01`\x01`\xA0\x1B\x03`\x1FT`\x08\x1C\x16\x80;\x15a\x07\xFFW\x81\x80\x91`$`@Q\x80\x94\x81\x93\x7F\x04\xF3\x86\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x81`\x04\x84\x01RZ\xF1\x80\x15a\x04\xF9Wa\x07\xD1WPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x01\xABW\x80`@Qc\x90\xC5\x01;`\xE0\x1B\x81R\x81\x81`\x04\x81\x83sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xF1\x80\x15a\x04\xF9Wa\x07\xC0WP\xF3[\x81a5W\x91a6\x1EV[a\x01\xABW\x80_a4\xA0V[a5n\x91P_\x90a6\x1EV[__a4\x0FV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a5\xCBW[` \x83\x10\x14a5\x9EWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a5\x93V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a5\xF1W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a5\xF1W`@RV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a6\xC5WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a6\xB8V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a7\x01WPPP\x90V[\x82Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a6\xF4V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a7kWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a7\xA7\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R\x89Qa6_V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a7\\V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a7\xE8WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a8>\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x86`\x01\x96\x03\x01\x87R`@\x83\x8BQ`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x91\x81\x85\x82\x01R\x01\x90a6\xE4V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a7\xD9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a5\xF1W`\x05\x1B` \x01\x90V[`\"T_\x92\x91a8t\x82a5\x84V[\x80\x82R\x91`\x01\x81\x16\x90\x81\x15a8\xE8WP`\x01\x14a8\x8FWPPV[`\"_\x90\x81R\x92\x93P\x90\x91\x90\x7Fa\x03[&\xE3\xE9\xEE\xE0\x0E\rr\xFD\x1E\xE8\xDD\xCAh\x94U\r\xCAi\x16\xEA*\xC6\xBA\xA9\r\x11\xE5\x10[\x83\x83\x10a8\xCEWP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a8\xBDV[` \x94\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x92\x91\x92\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[_\x92\x91\x81T\x91a91\x83a5\x84V[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a9\x86WP`\x01\x14a9MWPPPV[_\x90\x81R` \x81 \x93\x94P\x91\x92[\x83\x83\x10a9lWP` \x92P\x01\x01\x90V[`\x01\x81` \x92\x94\x93\x94T\x83\x85\x87\x01\x01R\x01\x91\x01\x91\x90a9[V[\x90P` \x94\x95P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x91P\x92\x91\x92\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a5\x80WQ\x80\x15\x15\x81\x03a5\x80W\x90V[``\x90`\x01`\x01`\xA0\x1B\x03a:\0\x93\x16\x81R_` \x82\x01R\x81`@\x82\x01R\x01`\"a9\"V[\x90V[` \x81\x83\x03\x12a5\x80W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a5\x80W\x01\x90\x80`\x1F\x83\x01\x12\x15a5\x80W\x81Q\x90a:8\x82a8MV[\x92a:F`@Q\x94\x85a6\x1EV[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x91\x82\x11a5\x80W` \x01\x91[\x81\x83\x10a:nWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a5\x80W\x81R` \x92\x83\x01\x92\x01a:aV[\x80Q\x15a:\x9BW` \x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80Q`\x01\x10\x15a:\x9BW`@\x01\x90V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x82\x01\x90_R` _ \x92_\x90[\x80`\x07\x83\x01\x10a<\xF1Wa;I\x94T\x91\x81\x81\x10a<\xBBW[\x81\x81\x10a<\x85W[\x81\x81\x10a<OW[\x81\x81\x10a<\x19W[\x81\x81\x10a;\xE3W[\x81\x81\x10a;\xADW[\x81\x81\x10a;xW[\x10a;KW[P\x03\x83a6\x1EV[V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01_a;AV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x81R\x01\x93\x01a;;V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`@\x1B\x16\x81R\x01\x93\x01a;3V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85``\x1B\x16\x81R\x01\x93\x01a;+V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\x80\x1B\x16\x81R\x01\x93\x01a;#V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xA0\x1B\x16\x81R\x01\x93\x01a;\x1BV[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xC0\x1B\x16\x81R\x01\x93\x01a;\x13V[\x92` `\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xE0\x1B\x16\x81R\x01\x93\x01a;\x0BV[\x91`\x08\x91\x93Pa\x01\0`\x01\x91\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87T\x81\x81`\xE0\x1B\x16\x83R\x81\x81`\xC0\x1B\x16` \x84\x01R\x81\x81`\xA0\x1B\x16`@\x84\x01R\x81\x81`\x80\x1B\x16``\x84\x01R\x81\x81``\x1B\x16`\x80\x84\x01R\x81\x81`@\x1B\x16`\xA0\x84\x01R\x81\x81` \x1B\x16`\xC0\x84\x01R\x16`\xE0\x82\x01R\x01\x94\x01\x92\x01\x85\x92\x93\x91a:\xF3V[`\x08T`\xFF\x16\x80\x15a=\x8DW\x90V[P`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x82\x01R` \x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x90\x81\x15a5uW_\x91a>%W[P\x15\x15\x90V[\x90P` \x81=` \x11a>OW[\x81a>@` \x93\x83a6\x1EV[\x81\x01\x03\x12a5\x80WQ_a>\x1FV[=\x91Pa>3V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a5\x80W`@Q\x90\x7F\x0C\x9F\xD5\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x15\x15`\x04\x82\x01R_\x81`$\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a5uWa>\xC9WPV[_a;I\x91a6\x1EV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a5\x80W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R_`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a5uWa>\xC9WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a5\x80W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x01`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a5uWa>\xC9WPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a5\x80W`@Q\x90\x7F\x98)lT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\x02`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a5uWa>\xC9WPV[\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a5\x80W`\x01`\x01`\xA0\x1B\x03\x90\x81`@Q\x93\x7FQSa\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R\x16`\x04\x84\x01R\x16`$\x82\x01R_\x81`D\x81sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Z\xFA\x80\x15a5uWa>\xC9WPV\xFE`\x80\x80`@R4`\x14W`\xE7\x90\x81a\0\x19\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15`\x11W_\x80\xFD[_5`\xE0\x1Ccz9y\xDC\x14`#W_\x80\xFD[4`\x9FW``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12`\x9FW`V`\xA3V[P`]`\xC5V[P`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x9FW6`#\x82\x01\x12\x15`\x9FW\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x9FW6\x91\x01`$\x01\x11`\x9FW\x80`\x01` \x92R\xF3[_\x80\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03`\x9FWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03`\x9FWV`\x804`\xB8W`\x1Fa\x10\x8F8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`\xBCW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`\xB8WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03`\xB8W\x80\x15`\xA5W_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x82U`@Q\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3a\x0F\xBE\x90\x81a\0\xD1\x829\xF3[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x04\xF3\x86\xF4\x14a\x06<W\x80c\x05.\xEF\xD1\x14a\x04\xBBW\x80c\x1BB\xC7\x11\x14a\x02\x9FW\x80cqP\x18\xA6\x14a\x02#W\x80cz9y\xDC\x14a\x01\x90W\x80c\x8D\xA5\xCB[\x14a\x01^W\x80c\xA2kJ\x88\x14a\x01CWc\xF2\xFD\xE3\x8B\x14a\0qW_\x80\xFD[4a\x01?W` `\x03\x196\x01\x12a\x01?Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\0\x9Fa\x07ZV[a\0\xA7a\n>V[\x16\x80\x15a\x01\x13Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17_U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x80\xFD[4a\x01?W_`\x03\x196\x01\x12a\x01?W` `@Q`(\x81R\xF3[4a\x01?W_`\x03\x196\x01\x12a\x01?W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\x01?W```\x03\x196\x01\x12a\x01?Wa\x01\xA9a\x07ZV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01?W`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01?W6`#\x83\x01\x12\x15a\x01?W\x81`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01?W6`$\x83\x85\x01\x01\x11a\x01?W` \x93`$a\x02\x19\x94\x01\x91a\x08AV[`@Q\x90\x15\x15\x81R\xF3[4a\x01?W_`\x03\x196\x01\x12a\x01?Wa\x02;a\n>V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x83U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01?W_`\x03\x196\x01\x12a\x01?W`\x01Ta\x02\xBB\x81a\x07\xEBV[a\x02\xC8`@Q\x91\x82a\x07}V[\x81\x81Ra\x02\xD4\x82a\x07\xEBV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0` \x82\x01\x92\x016\x837`\x01_\x90\x81R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x84\x82\x10\x80a\x04\x9CW[\x15a\x04\x92W\x82Q\x82\x10\x15a\x04eW\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x03\xA3\x92\x16` \x84`\x05\x1B\x86\x01\x01Ra\rtV[\x90\x15a\x04\x07W\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x03\xDAW`\x01\x01\x90a\x03bV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[PP\x90\x91P[`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x046WPPP\x03\x90\xF3[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x04(V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[PP\x90\x91Pa\x04\rV[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15\x15a\x03kV[4a\x01?W`@`\x03\x196\x01\x12a\x01?Wa\x04\xD4a\x07ZV[`$5\x90\x81\x15\x15\x82\x03a\x01?Wa\x04\xE9a\n>V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x15a\x06\x14Wa\x05\x10\x82a\n\x8AV[a\x05\xECW`(`\x01T\x10\x15a\x05\xC4W\x15a\x05\xB6Wa\x05-\x90a\x0E\xD5V[\x15a\x05XW\x7Fb\x10\x1C\xCC\xC1\x86M4\x92)\0p\xF4\xDB\xF1hy\xDExa\xAC\xB5\xDC\xB8\x18\x0BU\xD2\xED|\xD7\xE7_\x80\xA2\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FAddress not added\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[a\x05\xBF\x90a\r\xD5V[a\x05-V[\x7F\x13\xD8g\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xA2\xD8j\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01?W` `\x03\x196\x01\x12a\x01?Wa\x06Ua\x07ZV[a\x06]a\n>V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a\x06\x14Wa\x06\x84\x81a\n\x8AV[\x15a\x072Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x06\xA8\x83\x92a\x0C_V[\x16\x03a\x06\xD4W\x7F\xB5\xD6\x8C\xA4cr\xBB\xE6\xEC\x13\x8D=\x04#`\x82i\xB3\x11t\x96\xA4bh\xF8`\x80\xCD\xBC\xBE\xA9\xBE_\x80\xA2\0[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FAddress not removed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x7F=\x0F)=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01?WV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\xBEW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\xBEW`\x05\x1B` \x01\x90V[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DT\x93\x94\x90\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x82\x15a\t\xCBW\x91[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15a\n\x1BW` `@Q\x80\x92\x7Fz9y\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16`\x04\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`$\x83\x01R```D\x83\x01R\x81\x80a\tD`d\x82\x01\x8D\x8Ca\x08\x03V[\x03\x91Z\xFA\x90\x81\x15a\n\x10W_\x91a\t\xD6W[Pa\t\xCBWa\td\x90a\rtV[\x90a\x08\xAEWPPPa\t\xC7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93[`@Q\x94\x85\x94\x7F\x02\0\xDAH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R\x16`\x04\x85\x01R`@`$\x85\x01R`D\x84\x01\x91a\x08\x03V[\x03\x90\xFD[P\x93PPPP`\x01\x90V[\x90P` \x81=\x82\x11a\n\x08W[\x81a\t\xF0` \x93\x83a\x07}V[\x81\x01\x03\x12a\x01?WQ\x80\x15\x15\x81\x03a\x01?W_a\tVV[=\x91Pa\t\xE3V[`@Q=_\x82>=\x90\xFD[PPPPa\t\xC7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93a\t\x87V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x163\x03a\n^WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80_R`\x02` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x0BMW[\x15a\x0BGW`\x01_R\x7F\xAC3\xFFu\xC1\x9Ep\xFE\x83P}\xB0\xD6\x83\xFD4e\xC9\x96Y\x8D\xC9rh\x8Bz\xCEgl\x89\x07{` R\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9DTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0BCW`\x01\x90V[_\x90V[P`\x01\x90V[P\x80_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\n\xD4V[`\x01\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ _\x80R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15\x80a\x0C\x15W[\x15a\x0C\x0EW_\x80R` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ T\x16\x91\x16\x14_\x14a\x0BCW`\x01\x90V[PP`\x01\x90V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x80` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x15a\x0B\xCEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x80\x15a\rbW[a\r\\W_\x90\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T`\x01\x80\x88R\x84\x88 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x8BR\x89\x89R\x87\x8B \x8B\x80R\x89R\x87\x8B \x80T\x92\x90\x95\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81\x17\x90\x95U\x93\x8AR\x97\x87R\x85\x89 \x82\x8AR\x87R\x94\x90\x97 \x80T\x87\x16\x90\x91\x17\x90U\x80T\x85\x16\x90U\x90\x91R\x80T\x90\x91\x16\x90UT\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a\x03\xDAW`\x01U\x90V[PP_\x90V[Pa\rn\x82`\x01a\x0B\x82V[\x15a\x0C\x7FV[a\r\x7F\x81`\x01a\x0B\x82V[a\r\x8AWP_\x90_\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x02` R`@_ `\x01_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ T\x16\x90\x81\x15\x15\x91\x90V[a\r\xE0\x81`\x01a\x0B\x82V[\x15\x80a\x0E\xC4W[a\r\xF0WP_\x90V[\x7Fn\xE3\xEF\xEC\xAE\x88=\xF2\xD7\xCC\xDA\"a\x0BL\xA7q\xA2\x99\xE7\x07\xCB\re\xC4\xEC\x97\xDCNfh\xAD\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 `\x01\x80\x86R\x81\x84R\x82\x86 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x89T\x81\x16\x88\x17\x90\x99U\x98\x90\x96\x16\x80\x85R\x92\x82R\x80\x84 \x97\x84R\x96\x81R\x86\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U\x81\x80R\x92\x90\x91R\x92\x90\x92 \x80T\x90\x91\x16\x90\x91\x17\x90U[`\x01T`\x01\x81\x01\x80\x91\x11a\x03\xDAW`\x01U`\x01\x90V[Pa\x0E\xD0_`\x01a\x0B\x82V[a\r\xE7V[a\x0E\xE0\x81`\x01a\x0B\x82V[\x15\x80a\x0F\xADW[a\x0E\xF0WP_\x90V[\x7Fy\xC0n\x8C\x99\xA6g\xAD\xDAc\xC5\xFAo\x05i])c\x0F\xC6*\xD2\xDD\x06\x9F\xA9)\xD5qM\xE8\x9D\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16_\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x80R\x80\x83R\x81\x85 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U\x88T\x81\x16\x87\x17\x90\x98U\x97\x90\x95\x16\x80\x84R\x91\x81R\x84\x83 \x83\x80R\x81R\x84\x83 \x80T\x87\x16\x90\x94\x17\x90\x93U`\x01\x82R\x94\x90\x91R \x80T\x90\x91\x16\x90\x91\x17\x90Ua\x0E\xAEV[Pa\x0F\xB9_`\x01a\x0B\x82V[a\x0E\xE7V`\x80\x80`@R4`\x14W`\xE6\x90\x81a\0\x19\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15`\x11W_\x80\xFD[_5`\xE0\x1Ccz9y\xDC\x14`#W_\x80\xFD[4`\x9EW``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12`\x9EW`V`\xA2V[P`]`\xC4V[P`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x9EW6`#\x82\x01\x12\x15`\x9EW\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x9EW6\x91\x01`$\x01\x11`\x9EW\x80_` \x92R\xF3[_\x80\xFD[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03`\x9EWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03`\x9EWV",
    );
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
    /**Function with signature `emptyData()` and selector `0x1721daaa`.
```solidity
function emptyData() external view returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emptyDataCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`emptyData()`](emptyDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emptyDataReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<emptyDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: emptyDataCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emptyDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<emptyDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: emptyDataReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emptyDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emptyDataCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Bytes;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emptyData()";
            const SELECTOR: [u8; 4] = [23u8, 33u8, 218u8, 170u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: emptyDataReturn = r.into();
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
                        let r: emptyDataReturn = r.into();
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
    /**Function with signature `module()` and selector `0xb86d5298`.
```solidity
function module() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct moduleCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`module()`](moduleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct moduleReturn {
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
            impl ::core::convert::From<moduleCall> for UnderlyingRustTuple<'_> {
                fn from(value: moduleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for moduleCall {
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
            impl ::core::convert::From<moduleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: moduleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for moduleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for moduleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "module()";
            const SELECTOR: [u8; 4] = [184u8, 109u8, 82u8, 152u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: moduleReturn = r.into();
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
                        let r: moduleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `nonAdmin()` and selector `0xc12e4687`.
```solidity
function nonAdmin() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nonAdminCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`nonAdmin()`](nonAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nonAdminReturn {
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
            impl ::core::convert::From<nonAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: nonAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nonAdminCall {
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
            impl ::core::convert::From<nonAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nonAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nonAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nonAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nonAdmin()";
            const SELECTOR: [u8; 4] = [193u8, 46u8, 70u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
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
                        let r: nonAdminReturn = r.into();
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
                        let r: nonAdminReturn = r.into();
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
    /**Function with signature `testAddDuplicatePermissionCheck()` and selector `0xb91dc499`.
```solidity
function testAddDuplicatePermissionCheck() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testAddDuplicatePermissionCheckCall;
    ///Container type for the return parameters of the [`testAddDuplicatePermissionCheck()`](testAddDuplicatePermissionCheckCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testAddDuplicatePermissionCheckReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testAddDuplicatePermissionCheckCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testAddDuplicatePermissionCheckCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testAddDuplicatePermissionCheckCall {
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
            impl ::core::convert::From<testAddDuplicatePermissionCheckReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testAddDuplicatePermissionCheckReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testAddDuplicatePermissionCheckReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testAddDuplicatePermissionCheckReturn {
            fn _tokenize(
                &self,
            ) -> <testAddDuplicatePermissionCheckCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testAddDuplicatePermissionCheckCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testAddDuplicatePermissionCheckReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testAddDuplicatePermissionCheck()";
            const SELECTOR: [u8; 4] = [185u8, 29u8, 196u8, 153u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testAddDuplicatePermissionCheckReturn::_tokenize(ret)
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
    /**Function with signature `testAddPermissionCheckToHead()` and selector `0xdb5bfb0d`.
```solidity
function testAddPermissionCheckToHead() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testAddPermissionCheckToHeadCall;
    ///Container type for the return parameters of the [`testAddPermissionCheckToHead()`](testAddPermissionCheckToHeadCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testAddPermissionCheckToHeadReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testAddPermissionCheckToHeadCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testAddPermissionCheckToHeadCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testAddPermissionCheckToHeadCall {
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
            impl ::core::convert::From<testAddPermissionCheckToHeadReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testAddPermissionCheckToHeadReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testAddPermissionCheckToHeadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testAddPermissionCheckToHeadReturn {
            fn _tokenize(
                &self,
            ) -> <testAddPermissionCheckToHeadCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testAddPermissionCheckToHeadCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testAddPermissionCheckToHeadReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testAddPermissionCheckToHead()";
            const SELECTOR: [u8; 4] = [219u8, 91u8, 251u8, 13u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testAddPermissionCheckToHeadReturn::_tokenize(ret)
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
    /**Function with signature `testAddPermissionCheckToTail()` and selector `0xa5081d73`.
```solidity
function testAddPermissionCheckToTail() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testAddPermissionCheckToTailCall;
    ///Container type for the return parameters of the [`testAddPermissionCheckToTail()`](testAddPermissionCheckToTailCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testAddPermissionCheckToTailReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testAddPermissionCheckToTailCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testAddPermissionCheckToTailCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testAddPermissionCheckToTailCall {
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
            impl ::core::convert::From<testAddPermissionCheckToTailReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testAddPermissionCheckToTailReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testAddPermissionCheckToTailReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testAddPermissionCheckToTailReturn {
            fn _tokenize(
                &self,
            ) -> <testAddPermissionCheckToTailCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testAddPermissionCheckToTailCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testAddPermissionCheckToTailReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testAddPermissionCheckToTail()";
            const SELECTOR: [u8; 4] = [165u8, 8u8, 29u8, 115u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testAddPermissionCheckToTailReturn::_tokenize(ret)
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
    /**Function with signature `testAddRemovePermissionCheck()` and selector `0xa0974d87`.
```solidity
function testAddRemovePermissionCheck() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testAddRemovePermissionCheckCall;
    ///Container type for the return parameters of the [`testAddRemovePermissionCheck()`](testAddRemovePermissionCheckCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testAddRemovePermissionCheckReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testAddRemovePermissionCheckCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testAddRemovePermissionCheckCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testAddRemovePermissionCheckCall {
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
            impl ::core::convert::From<testAddRemovePermissionCheckReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testAddRemovePermissionCheckReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testAddRemovePermissionCheckReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testAddRemovePermissionCheckReturn {
            fn _tokenize(
                &self,
            ) -> <testAddRemovePermissionCheckCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testAddRemovePermissionCheckCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testAddRemovePermissionCheckReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testAddRemovePermissionCheck()";
            const SELECTOR: [u8; 4] = [160u8, 151u8, 77u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testAddRemovePermissionCheckReturn::_tokenize(ret)
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
    /**Function with signature `testCheckCombinations()` and selector `0x91d01c61`.
```solidity
function testCheckCombinations() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCheckCombinationsCall;
    ///Container type for the return parameters of the [`testCheckCombinations()`](testCheckCombinationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCheckCombinationsReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testCheckCombinationsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCheckCombinationsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCheckCombinationsCall {
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
            impl ::core::convert::From<testCheckCombinationsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCheckCombinationsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCheckCombinationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testCheckCombinationsReturn {
            fn _tokenize(
                &self,
            ) -> <testCheckCombinationsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testCheckCombinationsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testCheckCombinationsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testCheckCombinations()";
            const SELECTOR: [u8; 4] = [145u8, 208u8, 28u8, 97u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testCheckCombinationsReturn::_tokenize(ret)
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
    /**Function with signature `testEmptyGetAllChecks()` and selector `0x41e2ac2d`.
```solidity
function testEmptyGetAllChecks() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testEmptyGetAllChecksCall;
    ///Container type for the return parameters of the [`testEmptyGetAllChecks()`](testEmptyGetAllChecksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testEmptyGetAllChecksReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testEmptyGetAllChecksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testEmptyGetAllChecksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testEmptyGetAllChecksCall {
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
            impl ::core::convert::From<testEmptyGetAllChecksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testEmptyGetAllChecksReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testEmptyGetAllChecksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testEmptyGetAllChecksReturn {
            fn _tokenize(
                &self,
            ) -> <testEmptyGetAllChecksCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testEmptyGetAllChecksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testEmptyGetAllChecksReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testEmptyGetAllChecks()";
            const SELECTOR: [u8; 4] = [65u8, 226u8, 172u8, 45u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testEmptyGetAllChecksReturn::_tokenize(ret)
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
    /**Function with signature `testIndependentPermissionAndCalldataLists()` and selector `0x57b50acc`.
```solidity
function testIndependentPermissionAndCalldataLists() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIndependentPermissionAndCalldataListsCall;
    ///Container type for the return parameters of the [`testIndependentPermissionAndCalldataLists()`](testIndependentPermissionAndCalldataListsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIndependentPermissionAndCalldataListsReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testIndependentPermissionAndCalldataListsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIndependentPermissionAndCalldataListsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIndependentPermissionAndCalldataListsCall {
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
            impl ::core::convert::From<testIndependentPermissionAndCalldataListsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIndependentPermissionAndCalldataListsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIndependentPermissionAndCalldataListsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIndependentPermissionAndCalldataListsReturn {
            fn _tokenize(
                &self,
            ) -> <testIndependentPermissionAndCalldataListsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIndependentPermissionAndCalldataListsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIndependentPermissionAndCalldataListsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIndependentPermissionAndCalldataLists()";
            const SELECTOR: [u8; 4] = [87u8, 181u8, 10u8, 204u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIndependentPermissionAndCalldataListsReturn::_tokenize(ret)
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
    /**Function with signature `testIsAllowedFailure()` and selector `0x9d62f5ab`.
```solidity
function testIsAllowedFailure() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedFailureCall;
    ///Container type for the return parameters of the [`testIsAllowedFailure()`](testIsAllowedFailureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedFailureReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testIsAllowedFailureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedFailureCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedFailureCall {
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
            impl ::core::convert::From<testIsAllowedFailureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedFailureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedFailureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedFailureReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedFailureCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedFailureCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedFailureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedFailure()";
            const SELECTOR: [u8; 4] = [157u8, 98u8, 245u8, 171u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedFailureReturn::_tokenize(ret)
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
    /**Function with signature `testIsAllowedPassOne()` and selector `0x75689671`.
```solidity
function testIsAllowedPassOne() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedPassOneCall;
    ///Container type for the return parameters of the [`testIsAllowedPassOne()`](testIsAllowedPassOneCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedPassOneReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testIsAllowedPassOneCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedPassOneCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedPassOneCall {
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
            impl ::core::convert::From<testIsAllowedPassOneReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedPassOneReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedPassOneReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedPassOneReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedPassOneCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedPassOneCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedPassOneReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedPassOne()";
            const SELECTOR: [u8; 4] = [117u8, 104u8, 150u8, 113u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedPassOneReturn::_tokenize(ret)
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
    /**Function with signature `testIsAllowedPassOnlyOne()` and selector `0x8a3603a3`.
```solidity
function testIsAllowedPassOnlyOne() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedPassOnlyOneCall;
    ///Container type for the return parameters of the [`testIsAllowedPassOnlyOne()`](testIsAllowedPassOnlyOneCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIsAllowedPassOnlyOneReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testIsAllowedPassOnlyOneCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedPassOnlyOneCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedPassOnlyOneCall {
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
            impl ::core::convert::From<testIsAllowedPassOnlyOneReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIsAllowedPassOnlyOneReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIsAllowedPassOnlyOneReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testIsAllowedPassOnlyOneReturn {
            fn _tokenize(
                &self,
            ) -> <testIsAllowedPassOnlyOneCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIsAllowedPassOnlyOneCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIsAllowedPassOnlyOneReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIsAllowedPassOnlyOne()";
            const SELECTOR: [u8; 4] = [138u8, 54u8, 3u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testIsAllowedPassOnlyOneReturn::_tokenize(ret)
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
    /**Function with signature `testOnlyOwnerModifiersForPermissionChecks()` and selector `0x6e435a5b`.
```solidity
function testOnlyOwnerModifiersForPermissionChecks() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testOnlyOwnerModifiersForPermissionChecksCall;
    ///Container type for the return parameters of the [`testOnlyOwnerModifiersForPermissionChecks()`](testOnlyOwnerModifiersForPermissionChecksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testOnlyOwnerModifiersForPermissionChecksReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testOnlyOwnerModifiersForPermissionChecksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testOnlyOwnerModifiersForPermissionChecksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testOnlyOwnerModifiersForPermissionChecksCall {
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
            impl ::core::convert::From<testOnlyOwnerModifiersForPermissionChecksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testOnlyOwnerModifiersForPermissionChecksReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testOnlyOwnerModifiersForPermissionChecksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testOnlyOwnerModifiersForPermissionChecksReturn {
            fn _tokenize(
                &self,
            ) -> <testOnlyOwnerModifiersForPermissionChecksCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testOnlyOwnerModifiersForPermissionChecksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testOnlyOwnerModifiersForPermissionChecksReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testOnlyOwnerModifiersForPermissionChecks()";
            const SELECTOR: [u8; 4] = [110u8, 67u8, 90u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testOnlyOwnerModifiersForPermissionChecksReturn::_tokenize(ret)
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
    /**Function with signature `testRemoveNonExistentPermissionCheck()` and selector `0x08226e93`.
```solidity
function testRemoveNonExistentPermissionCheck() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRemoveNonExistentPermissionCheckCall;
    ///Container type for the return parameters of the [`testRemoveNonExistentPermissionCheck()`](testRemoveNonExistentPermissionCheckCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRemoveNonExistentPermissionCheckReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testRemoveNonExistentPermissionCheckCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRemoveNonExistentPermissionCheckCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRemoveNonExistentPermissionCheckCall {
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
            impl ::core::convert::From<testRemoveNonExistentPermissionCheckReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRemoveNonExistentPermissionCheckReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRemoveNonExistentPermissionCheckReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testRemoveNonExistentPermissionCheckReturn {
            fn _tokenize(
                &self,
            ) -> <testRemoveNonExistentPermissionCheckCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testRemoveNonExistentPermissionCheckCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testRemoveNonExistentPermissionCheckReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testRemoveNonExistentPermissionCheck()";
            const SELECTOR: [u8; 4] = [8u8, 34u8, 110u8, 147u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testRemoveNonExistentPermissionCheckReturn::_tokenize(ret)
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
    /**Function with signature `testRevert_TooManyPermissionChecks()` and selector `0xcdc3e010`.
```solidity
function testRevert_TooManyPermissionChecks() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRevert_TooManyPermissionChecksCall;
    ///Container type for the return parameters of the [`testRevert_TooManyPermissionChecks()`](testRevert_TooManyPermissionChecksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRevert_TooManyPermissionChecksReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testRevert_TooManyPermissionChecksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRevert_TooManyPermissionChecksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRevert_TooManyPermissionChecksCall {
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
            impl ::core::convert::From<testRevert_TooManyPermissionChecksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRevert_TooManyPermissionChecksReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRevert_TooManyPermissionChecksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testRevert_TooManyPermissionChecksReturn {
            fn _tokenize(
                &self,
            ) -> <testRevert_TooManyPermissionChecksCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testRevert_TooManyPermissionChecksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testRevert_TooManyPermissionChecksReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testRevert_TooManyPermissionChecks()";
            const SELECTOR: [u8; 4] = [205u8, 195u8, 224u8, 16u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testRevert_TooManyPermissionChecksReturn::_tokenize(ret)
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
    /**Function with signature `testRevertsOnZeroAddressAddPermission()` and selector `0x9bd8bed7`.
```solidity
function testRevertsOnZeroAddressAddPermission() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRevertsOnZeroAddressAddPermissionCall;
    ///Container type for the return parameters of the [`testRevertsOnZeroAddressAddPermission()`](testRevertsOnZeroAddressAddPermissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRevertsOnZeroAddressAddPermissionReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testRevertsOnZeroAddressAddPermissionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRevertsOnZeroAddressAddPermissionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRevertsOnZeroAddressAddPermissionCall {
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
            impl ::core::convert::From<testRevertsOnZeroAddressAddPermissionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRevertsOnZeroAddressAddPermissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRevertsOnZeroAddressAddPermissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testRevertsOnZeroAddressAddPermissionReturn {
            fn _tokenize(
                &self,
            ) -> <testRevertsOnZeroAddressAddPermissionCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testRevertsOnZeroAddressAddPermissionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testRevertsOnZeroAddressAddPermissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testRevertsOnZeroAddressAddPermission()";
            const SELECTOR: [u8; 4] = [155u8, 216u8, 190u8, 215u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testRevertsOnZeroAddressAddPermissionReturn::_tokenize(ret)
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
    /**Function with signature `testRevertsOnZeroAddressRemovePermission()` and selector `0x01f8d4df`.
```solidity
function testRevertsOnZeroAddressRemovePermission() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRevertsOnZeroAddressRemovePermissionCall;
    ///Container type for the return parameters of the [`testRevertsOnZeroAddressRemovePermission()`](testRevertsOnZeroAddressRemovePermissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRevertsOnZeroAddressRemovePermissionReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<testRevertsOnZeroAddressRemovePermissionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRevertsOnZeroAddressRemovePermissionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRevertsOnZeroAddressRemovePermissionCall {
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
            impl ::core::convert::From<testRevertsOnZeroAddressRemovePermissionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRevertsOnZeroAddressRemovePermissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRevertsOnZeroAddressRemovePermissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testRevertsOnZeroAddressRemovePermissionReturn {
            fn _tokenize(
                &self,
            ) -> <testRevertsOnZeroAddressRemovePermissionCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testRevertsOnZeroAddressRemovePermissionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testRevertsOnZeroAddressRemovePermissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testRevertsOnZeroAddressRemovePermission()";
            const SELECTOR: [u8; 4] = [1u8, 248u8, 212u8, 223u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testRevertsOnZeroAddressRemovePermissionReturn::_tokenize(ret)
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
    ///Container for all the [`RequireOrModuleTest`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum RequireOrModuleTestCalls {
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        admin(adminCall),
        #[allow(missing_docs)]
        emptyData(emptyDataCall),
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
        module(moduleCall),
        #[allow(missing_docs)]
        nonAdmin(nonAdminCall),
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
        testAddDuplicatePermissionCheck(testAddDuplicatePermissionCheckCall),
        #[allow(missing_docs)]
        testAddPermissionCheckToHead(testAddPermissionCheckToHeadCall),
        #[allow(missing_docs)]
        testAddPermissionCheckToTail(testAddPermissionCheckToTailCall),
        #[allow(missing_docs)]
        testAddRemovePermissionCheck(testAddRemovePermissionCheckCall),
        #[allow(missing_docs)]
        testCheckCombinations(testCheckCombinationsCall),
        #[allow(missing_docs)]
        testEmptyGetAllChecks(testEmptyGetAllChecksCall),
        #[allow(missing_docs)]
        testIndependentPermissionAndCalldataLists(
            testIndependentPermissionAndCalldataListsCall,
        ),
        #[allow(missing_docs)]
        testIsAllowedFailure(testIsAllowedFailureCall),
        #[allow(missing_docs)]
        testIsAllowedNoChecks(testIsAllowedNoChecksCall),
        #[allow(missing_docs)]
        testIsAllowedPassOne(testIsAllowedPassOneCall),
        #[allow(missing_docs)]
        testIsAllowedPassOnlyOne(testIsAllowedPassOnlyOneCall),
        #[allow(missing_docs)]
        testOnlyOwnerModifiersForPermissionChecks(
            testOnlyOwnerModifiersForPermissionChecksCall,
        ),
        #[allow(missing_docs)]
        testRemoveNonExistentPermissionCheck(testRemoveNonExistentPermissionCheckCall),
        #[allow(missing_docs)]
        testRevert_TooManyPermissionChecks(testRevert_TooManyPermissionChecksCall),
        #[allow(missing_docs)]
        testRevertsOnZeroAddressAddPermission(testRevertsOnZeroAddressAddPermissionCall),
        #[allow(missing_docs)]
        testRevertsOnZeroAddressRemovePermission(
            testRevertsOnZeroAddressRemovePermissionCall,
        ),
    }
    impl RequireOrModuleTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 248u8, 212u8, 223u8],
            [8u8, 34u8, 110u8, 147u8],
            [10u8, 146u8, 84u8, 228u8],
            [23u8, 33u8, 218u8, 170u8],
            [30u8, 215u8, 131u8, 28u8],
            [42u8, 222u8, 56u8, 128u8],
            [52u8, 217u8, 35u8, 219u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [65u8, 226u8, 172u8, 45u8],
            [87u8, 181u8, 10u8, 204u8],
            [102u8, 217u8, 169u8, 160u8],
            [110u8, 67u8, 90u8, 91u8],
            [117u8, 104u8, 150u8, 113u8],
            [133u8, 34u8, 108u8, 129u8],
            [138u8, 54u8, 3u8, 163u8],
            [145u8, 106u8, 23u8, 198u8],
            [145u8, 208u8, 28u8, 97u8],
            [155u8, 216u8, 190u8, 215u8],
            [157u8, 98u8, 245u8, 171u8],
            [160u8, 151u8, 77u8, 135u8],
            [165u8, 8u8, 29u8, 115u8],
            [176u8, 70u8, 79u8, 220u8],
            [181u8, 80u8, 138u8, 169u8],
            [184u8, 109u8, 82u8, 152u8],
            [185u8, 29u8, 196u8, 153u8],
            [186u8, 65u8, 79u8, 166u8],
            [193u8, 46u8, 70u8, 135u8],
            [205u8, 195u8, 224u8, 16u8],
            [219u8, 91u8, 251u8, 13u8],
            [226u8, 12u8, 159u8, 113u8],
            [248u8, 81u8, 164u8, 64u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(testRevertsOnZeroAddressRemovePermission),
            ::core::stringify!(testRemoveNonExistentPermissionCheck),
            ::core::stringify!(setUp),
            ::core::stringify!(emptyData),
            ::core::stringify!(excludeSenders),
            ::core::stringify!(targetInterfaces),
            ::core::stringify!(testIsAllowedNoChecks),
            ::core::stringify!(targetSenders),
            ::core::stringify!(targetContracts),
            ::core::stringify!(testEmptyGetAllChecks),
            ::core::stringify!(testIndependentPermissionAndCalldataLists),
            ::core::stringify!(targetArtifactSelectors),
            ::core::stringify!(testOnlyOwnerModifiersForPermissionChecks),
            ::core::stringify!(testIsAllowedPassOne),
            ::core::stringify!(targetArtifacts),
            ::core::stringify!(testIsAllowedPassOnlyOne),
            ::core::stringify!(targetSelectors),
            ::core::stringify!(testCheckCombinations),
            ::core::stringify!(testRevertsOnZeroAddressAddPermission),
            ::core::stringify!(testIsAllowedFailure),
            ::core::stringify!(testAddRemovePermissionCheck),
            ::core::stringify!(testAddPermissionCheckToTail),
            ::core::stringify!(excludeSelectors),
            ::core::stringify!(excludeArtifacts),
            ::core::stringify!(module),
            ::core::stringify!(testAddDuplicatePermissionCheck),
            ::core::stringify!(failed),
            ::core::stringify!(nonAdmin),
            ::core::stringify!(testRevert_TooManyPermissionChecks),
            ::core::stringify!(testAddPermissionCheckToHead),
            ::core::stringify!(excludeContracts),
            ::core::stringify!(admin),
            ::core::stringify!(IS_TEST),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <testRevertsOnZeroAddressRemovePermissionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testRemoveNonExistentPermissionCheckCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setUpCall as alloy_sol_types::SolCall>::SIGNATURE,
            <emptyDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetInterfacesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testIsAllowedNoChecksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testEmptyGetAllChecksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testIndependentPermissionAndCalldataListsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testOnlyOwnerModifiersForPermissionChecksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testIsAllowedPassOneCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testIsAllowedPassOnlyOneCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testCheckCombinationsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testRevertsOnZeroAddressAddPermissionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testIsAllowedFailureCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testAddRemovePermissionCheckCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testAddPermissionCheckToTailCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <moduleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testAddDuplicatePermissionCheckCall as alloy_sol_types::SolCall>::SIGNATURE,
            <failedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <nonAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testRevert_TooManyPermissionChecksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testAddPermissionCheckToHeadCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <adminCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for RequireOrModuleTestCalls {
        const NAME: &'static str = "RequireOrModuleTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 33usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::admin(_) => <adminCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::emptyData(_) => {
                    <emptyDataCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::module(_) => <moduleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nonAdmin(_) => <nonAdminCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::testAddDuplicatePermissionCheck(_) => {
                    <testAddDuplicatePermissionCheckCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testAddPermissionCheckToHead(_) => {
                    <testAddPermissionCheckToHeadCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testAddPermissionCheckToTail(_) => {
                    <testAddPermissionCheckToTailCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testAddRemovePermissionCheck(_) => {
                    <testAddRemovePermissionCheckCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testCheckCombinations(_) => {
                    <testCheckCombinationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testEmptyGetAllChecks(_) => {
                    <testEmptyGetAllChecksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIndependentPermissionAndCalldataLists(_) => {
                    <testIndependentPermissionAndCalldataListsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedFailure(_) => {
                    <testIsAllowedFailureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedNoChecks(_) => {
                    <testIsAllowedNoChecksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedPassOne(_) => {
                    <testIsAllowedPassOneCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIsAllowedPassOnlyOne(_) => {
                    <testIsAllowedPassOnlyOneCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testOnlyOwnerModifiersForPermissionChecks(_) => {
                    <testOnlyOwnerModifiersForPermissionChecksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testRemoveNonExistentPermissionCheck(_) => {
                    <testRemoveNonExistentPermissionCheckCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testRevert_TooManyPermissionChecks(_) => {
                    <testRevert_TooManyPermissionChecksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testRevertsOnZeroAddressAddPermission(_) => {
                    <testRevertsOnZeroAddressAddPermissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testRevertsOnZeroAddressRemovePermission(_) => {
                    <testRevertsOnZeroAddressRemovePermissionCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<RequireOrModuleTestCalls>] = &[
                {
                    fn testRevertsOnZeroAddressRemovePermission(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testRevertsOnZeroAddressRemovePermissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testRevertsOnZeroAddressRemovePermission,
                            )
                    }
                    testRevertsOnZeroAddressRemovePermission
                },
                {
                    fn testRemoveNonExistentPermissionCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testRemoveNonExistentPermissionCheckCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testRemoveNonExistentPermissionCheck,
                            )
                    }
                    testRemoveNonExistentPermissionCheck
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RequireOrModuleTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn emptyData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <emptyDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RequireOrModuleTestCalls::emptyData)
                    }
                    emptyData
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn testIsAllowedNoChecks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testIsAllowedNoChecksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testIsAllowedNoChecks)
                    }
                    testIsAllowedNoChecks
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn testEmptyGetAllChecks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testEmptyGetAllChecksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testEmptyGetAllChecks)
                    }
                    testEmptyGetAllChecks
                },
                {
                    fn testIndependentPermissionAndCalldataLists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testIndependentPermissionAndCalldataListsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testIndependentPermissionAndCalldataLists,
                            )
                    }
                    testIndependentPermissionAndCalldataLists
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn testOnlyOwnerModifiersForPermissionChecks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testOnlyOwnerModifiersForPermissionChecksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testOnlyOwnerModifiersForPermissionChecks,
                            )
                    }
                    testOnlyOwnerModifiersForPermissionChecks
                },
                {
                    fn testIsAllowedPassOne(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testIsAllowedPassOneCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testIsAllowedPassOne)
                    }
                    testIsAllowedPassOne
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn testIsAllowedPassOnlyOne(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testIsAllowedPassOnlyOneCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testIsAllowedPassOnlyOne)
                    }
                    testIsAllowedPassOnlyOne
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn testCheckCombinations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testCheckCombinationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testCheckCombinations)
                    }
                    testCheckCombinations
                },
                {
                    fn testRevertsOnZeroAddressAddPermission(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testRevertsOnZeroAddressAddPermissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testRevertsOnZeroAddressAddPermission,
                            )
                    }
                    testRevertsOnZeroAddressAddPermission
                },
                {
                    fn testIsAllowedFailure(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testIsAllowedFailureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testIsAllowedFailure)
                    }
                    testIsAllowedFailure
                },
                {
                    fn testAddRemovePermissionCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testAddRemovePermissionCheckCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testAddRemovePermissionCheck)
                    }
                    testAddRemovePermissionCheck
                },
                {
                    fn testAddPermissionCheckToTail(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testAddPermissionCheckToTailCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testAddPermissionCheckToTail)
                    }
                    testAddPermissionCheckToTail
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn module(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <moduleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RequireOrModuleTestCalls::module)
                    }
                    module
                },
                {
                    fn testAddDuplicatePermissionCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testAddDuplicatePermissionCheckCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testAddDuplicatePermissionCheck,
                            )
                    }
                    testAddDuplicatePermissionCheck
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RequireOrModuleTestCalls::failed)
                    }
                    failed
                },
                {
                    fn nonAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <nonAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RequireOrModuleTestCalls::nonAdmin)
                    }
                    nonAdmin
                },
                {
                    fn testRevert_TooManyPermissionChecks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testRevert_TooManyPermissionChecksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testRevert_TooManyPermissionChecks,
                            )
                    }
                    testRevert_TooManyPermissionChecks
                },
                {
                    fn testAddPermissionCheckToHead(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testAddPermissionCheckToHeadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testAddPermissionCheckToHead)
                    }
                    testAddPermissionCheckToHead
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RequireOrModuleTestCalls::admin)
                    }
                    admin
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(RequireOrModuleTestCalls::IS_TEST)
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
            ) -> alloy_sol_types::Result<RequireOrModuleTestCalls>] = &[
                {
                    fn testRevertsOnZeroAddressRemovePermission(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testRevertsOnZeroAddressRemovePermissionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testRevertsOnZeroAddressRemovePermission,
                            )
                    }
                    testRevertsOnZeroAddressRemovePermission
                },
                {
                    fn testRemoveNonExistentPermissionCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testRemoveNonExistentPermissionCheckCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testRemoveNonExistentPermissionCheck,
                            )
                    }
                    testRemoveNonExistentPermissionCheck
                },
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn emptyData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <emptyDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::emptyData)
                    }
                    emptyData
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn testIsAllowedNoChecks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testIsAllowedNoChecksCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testIsAllowedNoChecks)
                    }
                    testIsAllowedNoChecks
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn testEmptyGetAllChecks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testEmptyGetAllChecksCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testEmptyGetAllChecks)
                    }
                    testEmptyGetAllChecks
                },
                {
                    fn testIndependentPermissionAndCalldataLists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testIndependentPermissionAndCalldataListsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testIndependentPermissionAndCalldataLists,
                            )
                    }
                    testIndependentPermissionAndCalldataLists
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn testOnlyOwnerModifiersForPermissionChecks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testOnlyOwnerModifiersForPermissionChecksCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testOnlyOwnerModifiersForPermissionChecks,
                            )
                    }
                    testOnlyOwnerModifiersForPermissionChecks
                },
                {
                    fn testIsAllowedPassOne(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testIsAllowedPassOneCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testIsAllowedPassOne)
                    }
                    testIsAllowedPassOne
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn testIsAllowedPassOnlyOne(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testIsAllowedPassOnlyOneCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testIsAllowedPassOnlyOne)
                    }
                    testIsAllowedPassOnlyOne
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn testCheckCombinations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testCheckCombinationsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testCheckCombinations)
                    }
                    testCheckCombinations
                },
                {
                    fn testRevertsOnZeroAddressAddPermission(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testRevertsOnZeroAddressAddPermissionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testRevertsOnZeroAddressAddPermission,
                            )
                    }
                    testRevertsOnZeroAddressAddPermission
                },
                {
                    fn testIsAllowedFailure(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testIsAllowedFailureCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testIsAllowedFailure)
                    }
                    testIsAllowedFailure
                },
                {
                    fn testAddRemovePermissionCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testAddRemovePermissionCheckCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testAddRemovePermissionCheck)
                    }
                    testAddRemovePermissionCheck
                },
                {
                    fn testAddPermissionCheckToTail(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testAddPermissionCheckToTailCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testAddPermissionCheckToTail)
                    }
                    testAddPermissionCheckToTail
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn module(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <moduleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::module)
                    }
                    module
                },
                {
                    fn testAddDuplicatePermissionCheck(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testAddDuplicatePermissionCheckCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testAddDuplicatePermissionCheck,
                            )
                    }
                    testAddDuplicatePermissionCheck
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::failed)
                    }
                    failed
                },
                {
                    fn nonAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <nonAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::nonAdmin)
                    }
                    nonAdmin
                },
                {
                    fn testRevert_TooManyPermissionChecks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testRevert_TooManyPermissionChecksCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                RequireOrModuleTestCalls::testRevert_TooManyPermissionChecks,
                            )
                    }
                    testRevert_TooManyPermissionChecks
                },
                {
                    fn testAddPermissionCheckToHead(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <testAddPermissionCheckToHeadCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::testAddPermissionCheckToHead)
                    }
                    testAddPermissionCheckToHead
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn admin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <adminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::admin)
                    }
                    admin
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<RequireOrModuleTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(RequireOrModuleTestCalls::IS_TEST)
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
                Self::emptyData(inner) => {
                    <emptyDataCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::module(inner) => {
                    <moduleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nonAdmin(inner) => {
                    <nonAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::testAddDuplicatePermissionCheck(inner) => {
                    <testAddDuplicatePermissionCheckCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testAddPermissionCheckToHead(inner) => {
                    <testAddPermissionCheckToHeadCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testAddPermissionCheckToTail(inner) => {
                    <testAddPermissionCheckToTailCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testAddRemovePermissionCheck(inner) => {
                    <testAddRemovePermissionCheckCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testCheckCombinations(inner) => {
                    <testCheckCombinationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testEmptyGetAllChecks(inner) => {
                    <testEmptyGetAllChecksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIndependentPermissionAndCalldataLists(inner) => {
                    <testIndependentPermissionAndCalldataListsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedFailure(inner) => {
                    <testIsAllowedFailureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedNoChecks(inner) => {
                    <testIsAllowedNoChecksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedPassOne(inner) => {
                    <testIsAllowedPassOneCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIsAllowedPassOnlyOne(inner) => {
                    <testIsAllowedPassOnlyOneCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testOnlyOwnerModifiersForPermissionChecks(inner) => {
                    <testOnlyOwnerModifiersForPermissionChecksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testRemoveNonExistentPermissionCheck(inner) => {
                    <testRemoveNonExistentPermissionCheckCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testRevert_TooManyPermissionChecks(inner) => {
                    <testRevert_TooManyPermissionChecksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testRevertsOnZeroAddressAddPermission(inner) => {
                    <testRevertsOnZeroAddressAddPermissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testRevertsOnZeroAddressRemovePermission(inner) => {
                    <testRevertsOnZeroAddressRemovePermissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::admin(inner) => {
                    <adminCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::emptyData(inner) => {
                    <emptyDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::module(inner) => {
                    <moduleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::nonAdmin(inner) => {
                    <nonAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::testAddDuplicatePermissionCheck(inner) => {
                    <testAddDuplicatePermissionCheckCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testAddPermissionCheckToHead(inner) => {
                    <testAddPermissionCheckToHeadCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testAddPermissionCheckToTail(inner) => {
                    <testAddPermissionCheckToTailCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testAddRemovePermissionCheck(inner) => {
                    <testAddRemovePermissionCheckCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testCheckCombinations(inner) => {
                    <testCheckCombinationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testEmptyGetAllChecks(inner) => {
                    <testEmptyGetAllChecksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIndependentPermissionAndCalldataLists(inner) => {
                    <testIndependentPermissionAndCalldataListsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedFailure(inner) => {
                    <testIsAllowedFailureCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::testIsAllowedPassOne(inner) => {
                    <testIsAllowedPassOneCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIsAllowedPassOnlyOne(inner) => {
                    <testIsAllowedPassOnlyOneCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testOnlyOwnerModifiersForPermissionChecks(inner) => {
                    <testOnlyOwnerModifiersForPermissionChecksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testRemoveNonExistentPermissionCheck(inner) => {
                    <testRemoveNonExistentPermissionCheckCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testRevert_TooManyPermissionChecks(inner) => {
                    <testRevert_TooManyPermissionChecksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testRevertsOnZeroAddressAddPermission(inner) => {
                    <testRevertsOnZeroAddressAddPermissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testRevertsOnZeroAddressRemovePermission(inner) => {
                    <testRevertsOnZeroAddressRemovePermissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`RequireOrModuleTest`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum RequireOrModuleTestEvents {
        #[allow(missing_docs)]
        PermissionCheckAdded(PermissionCheckAdded),
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
    impl RequireOrModuleTestEvents {
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
            ::core::stringify!(log_array_2),
            ::core::stringify!(log),
            ::core::stringify!(log_named_decimal_int),
            ::core::stringify!(PermissionCheckAdded),
            ::core::stringify!(log_address),
            ::core::stringify!(log_array_1),
            ::core::stringify!(log_named_address),
            ::core::stringify!(log_named_array_1),
            ::core::stringify!(log_named_bytes32),
            ::core::stringify!(log_named_uint),
            ::core::stringify!(PermissionCheckRemoved),
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
            <log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <PermissionCheckAdded as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_address as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <PermissionCheckRemoved as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for RequireOrModuleTestEvents {
        const NAME: &'static str = "RequireOrModuleTestEvents";
        const COUNT: usize = 24usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
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
    impl alloy_sol_types::private::IntoLogData for RequireOrModuleTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::PermissionCheckAdded(inner) => {
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
                Self::PermissionCheckAdded(inner) => {
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
    /**Creates a new wrapper around an on-chain [`RequireOrModuleTest`](self) contract instance.

See the [wrapper's documentation](`RequireOrModuleTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> RequireOrModuleTestInstance<P, N> {
        RequireOrModuleTestInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<RequireOrModuleTestInstance<P, N>>,
    > {
        RequireOrModuleTestInstance::<P, N>::deploy(__provider)
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
        RequireOrModuleTestInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`RequireOrModuleTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`RequireOrModuleTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct RequireOrModuleTestInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for RequireOrModuleTestInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("RequireOrModuleTestInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > RequireOrModuleTestInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`RequireOrModuleTest`](self) contract instance.

See the [wrapper's documentation](`RequireOrModuleTestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<RequireOrModuleTestInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> RequireOrModuleTestInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> RequireOrModuleTestInstance<P, N> {
            RequireOrModuleTestInstance {
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
    > RequireOrModuleTestInstance<P, N> {
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
        ///Creates a new call builder for the [`emptyData`] function.
        pub fn emptyData(&self) -> alloy_contract::SolCallBuilder<&P, emptyDataCall, N> {
            self.call_builder(&emptyDataCall)
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
        ///Creates a new call builder for the [`module`] function.
        pub fn module(&self) -> alloy_contract::SolCallBuilder<&P, moduleCall, N> {
            self.call_builder(&moduleCall)
        }
        ///Creates a new call builder for the [`nonAdmin`] function.
        pub fn nonAdmin(&self) -> alloy_contract::SolCallBuilder<&P, nonAdminCall, N> {
            self.call_builder(&nonAdminCall)
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
        ///Creates a new call builder for the [`testAddDuplicatePermissionCheck`] function.
        pub fn testAddDuplicatePermissionCheck(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testAddDuplicatePermissionCheckCall, N> {
            self.call_builder(&testAddDuplicatePermissionCheckCall)
        }
        ///Creates a new call builder for the [`testAddPermissionCheckToHead`] function.
        pub fn testAddPermissionCheckToHead(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testAddPermissionCheckToHeadCall, N> {
            self.call_builder(&testAddPermissionCheckToHeadCall)
        }
        ///Creates a new call builder for the [`testAddPermissionCheckToTail`] function.
        pub fn testAddPermissionCheckToTail(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testAddPermissionCheckToTailCall, N> {
            self.call_builder(&testAddPermissionCheckToTailCall)
        }
        ///Creates a new call builder for the [`testAddRemovePermissionCheck`] function.
        pub fn testAddRemovePermissionCheck(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testAddRemovePermissionCheckCall, N> {
            self.call_builder(&testAddRemovePermissionCheckCall)
        }
        ///Creates a new call builder for the [`testCheckCombinations`] function.
        pub fn testCheckCombinations(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testCheckCombinationsCall, N> {
            self.call_builder(&testCheckCombinationsCall)
        }
        ///Creates a new call builder for the [`testEmptyGetAllChecks`] function.
        pub fn testEmptyGetAllChecks(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testEmptyGetAllChecksCall, N> {
            self.call_builder(&testEmptyGetAllChecksCall)
        }
        ///Creates a new call builder for the [`testIndependentPermissionAndCalldataLists`] function.
        pub fn testIndependentPermissionAndCalldataLists(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testIndependentPermissionAndCalldataListsCall,
            N,
        > {
            self.call_builder(&testIndependentPermissionAndCalldataListsCall)
        }
        ///Creates a new call builder for the [`testIsAllowedFailure`] function.
        pub fn testIsAllowedFailure(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedFailureCall, N> {
            self.call_builder(&testIsAllowedFailureCall)
        }
        ///Creates a new call builder for the [`testIsAllowedNoChecks`] function.
        pub fn testIsAllowedNoChecks(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedNoChecksCall, N> {
            self.call_builder(&testIsAllowedNoChecksCall)
        }
        ///Creates a new call builder for the [`testIsAllowedPassOne`] function.
        pub fn testIsAllowedPassOne(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedPassOneCall, N> {
            self.call_builder(&testIsAllowedPassOneCall)
        }
        ///Creates a new call builder for the [`testIsAllowedPassOnlyOne`] function.
        pub fn testIsAllowedPassOnlyOne(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testIsAllowedPassOnlyOneCall, N> {
            self.call_builder(&testIsAllowedPassOnlyOneCall)
        }
        ///Creates a new call builder for the [`testOnlyOwnerModifiersForPermissionChecks`] function.
        pub fn testOnlyOwnerModifiersForPermissionChecks(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testOnlyOwnerModifiersForPermissionChecksCall,
            N,
        > {
            self.call_builder(&testOnlyOwnerModifiersForPermissionChecksCall)
        }
        ///Creates a new call builder for the [`testRemoveNonExistentPermissionCheck`] function.
        pub fn testRemoveNonExistentPermissionCheck(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testRemoveNonExistentPermissionCheckCall,
            N,
        > {
            self.call_builder(&testRemoveNonExistentPermissionCheckCall)
        }
        ///Creates a new call builder for the [`testRevert_TooManyPermissionChecks`] function.
        pub fn testRevert_TooManyPermissionChecks(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testRevert_TooManyPermissionChecksCall,
            N,
        > {
            self.call_builder(&testRevert_TooManyPermissionChecksCall)
        }
        ///Creates a new call builder for the [`testRevertsOnZeroAddressAddPermission`] function.
        pub fn testRevertsOnZeroAddressAddPermission(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testRevertsOnZeroAddressAddPermissionCall,
            N,
        > {
            self.call_builder(&testRevertsOnZeroAddressAddPermissionCall)
        }
        ///Creates a new call builder for the [`testRevertsOnZeroAddressRemovePermission`] function.
        pub fn testRevertsOnZeroAddressRemovePermission(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testRevertsOnZeroAddressRemovePermissionCall,
            N,
        > {
            self.call_builder(&testRevertsOnZeroAddressRemovePermissionCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > RequireOrModuleTestInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`PermissionCheckAdded`] event.
        pub fn PermissionCheckAdded_filter(
            &self,
        ) -> alloy_contract::Event<&P, PermissionCheckAdded, N> {
            self.event_filter::<PermissionCheckAdded>()
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
